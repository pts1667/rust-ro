use std::net::Shutdown;
use std::sync::Arc;

use database::model::AccountRecord;

use super::framing::FrameLength;
use crate::server::Server;
use crate::server::model::request::Request;
use crate::server::model::session::Session;
use crate::server::request_handler::char::send_character_list;
use crate::server::service::char_server_service::{self, AccountContext, DeleteOutcome};
use crate::server::service::login_service::LoginService;
use crate::server::service::pincode;

const CH_DELETE_CHAR3_RESERVED: u16 = 0x0827;
const CH_DELETE_CHAR3: u16 = 0x0829;
const CH_DELETE_CHAR3_CANCEL: u16 = 0x082b;
const CH_DELETE_CHAR_LEGACY: u16 = 0x0068;
const CH_DELETE_CHAR: u16 = 0x01fb;
const CH_REQ_IS_VALID_CHARNAME: u16 = 0x028d;
const CH_REQ_CHANGE_CHARNAME: u16 = 0x08fc;
const CH_SECOND_PASSWD_ACK: u16 = 0x08b8;
const CH_MAKE_SECOND_PASSWD: u16 = 0x08ba;
const CH_EDIT_SECOND_PASSWD: u16 = 0x08be;
const CH_AVAILABLE_SECOND_PASSWD: u16 = 0x08c5;
const CH_REQ_CHANGE_CHARACTER_SLOT: u16 = 0x08d4;

const PIN_SUPPORTED_SINCE: u32 = 20110309;
const NAME_CHANGE_SUPPORTED_SINCE: u32 = 20111101;
const SLOT_CHANGE_SUPPORTED_SINCE: u32 = 20110928;
const LEGACY_DELETE_UNTIL: u32 = 20040419;

pub fn frame_length(id: u16, packetver: u32) -> Option<FrameLength> {
    let fixed = |length| Some(FrameLength::Fixed(length));
    match id {
        CH_DELETE_CHAR3_RESERVED | CH_DELETE_CHAR3_CANCEL => fixed(6),
        CH_DELETE_CHAR3 => fixed(12),
        CH_DELETE_CHAR_LEGACY if packetver < LEGACY_DELETE_UNTIL => fixed(46),
        CH_DELETE_CHAR if packetver >= LEGACY_DELETE_UNTIL => fixed(56),
        CH_REQ_IS_VALID_CHARNAME if packetver >= NAME_CHANGE_SUPPORTED_SINCE => fixed(34),
        CH_REQ_CHANGE_CHARNAME if packetver >= NAME_CHANGE_SUPPORTED_SINCE => fixed(30),
        CH_SECOND_PASSWD_ACK | CH_MAKE_SECOND_PASSWD if packetver >= PIN_SUPPORTED_SINCE => fixed(10),
        CH_EDIT_SECOND_PASSWD if packetver >= PIN_SUPPORTED_SINCE => fixed(14),
        CH_AVAILABLE_SECOND_PASSWD if packetver >= PIN_SUPPORTED_SINCE => fixed(6),
        CH_REQ_CHANGE_CHARACTER_SLOT if packetver >= SLOT_CHANGE_SUPPORTED_SINCE => fixed(8),
        _ => None,
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]])
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
    bytes[..end].iter().map(|byte| *byte as char).collect()
}

pub fn pin_state_packet(seed: u32, account_id: u32, state: u16) -> Vec<u8> {
    let mut packet = vec![0xb9, 0x08];
    packet.extend(seed.to_le_bytes());
    packet.extend(account_id.to_le_bytes());
    packet.extend(state.to_le_bytes());
    packet
}

pub fn delete_reserved_reply(char_id: u32, result: i32, date: u32) -> Vec<u8> {
    let mut packet = vec![0x28, 0x08];
    packet.extend(char_id.to_le_bytes());
    packet.extend(result.to_le_bytes());
    packet.extend(date.to_le_bytes());
    packet
}

pub fn delete_result_reply(id: [u8; 2], char_id: u32, result: i32) -> Vec<u8> {
    let mut packet = id.to_vec();
    packet.extend(char_id.to_le_bytes());
    packet.extend(result.to_le_bytes());
    packet
}

pub fn slot_move_reply(reason: u16, moves: u32) -> Vec<u8> {
    let mut packet = vec![0xd5, 0x08, 8, 0];
    packet.extend(reason.to_le_bytes());
    packet.extend((moves.min(u32::from(u16::MAX)) as u16).to_le_bytes());
    packet
}

fn char_session(server: &Server, context: &Request) -> Result<Arc<Session>, String> {
    let session_id = server.ensure_session_exists(&context.socket()).ok_or("No authenticated session")?;
    let session = server.sessions().find(session_id).ok_or("Session expired")?;
    let on_char_socket = session
        .char_server_socket
        .as_ref()
        .is_some_and(|socket| Arc::ptr_eq(socket, &context.socket()));
    if !on_char_socket {
        return Err("Character server request arrived on a different connection".into());
    }
    Ok(session)
}

fn now_seconds() -> i64 {
    chrono::Utc::now().timestamp()
}

pub fn pin_gate_open(server: &Server, session: &Session) -> bool {
    let config = &server.configuration.char_server.pincode;
    if !config.enabled {
        return true;
    }
    let has_pin = server
        .repository
        .account_by_id(session.account_id)
        .ok()
        .flatten()
        .is_some_and(|account| !account.pincode.is_empty());
    !has_pin || session.account.char_server().pin_verified
}

/// A client that sends anything but PIN traffic before clearing the PIN is dropped, as rathena does.
pub fn require_pin(server: &Server, context: &Request, session: &Session) -> bool {
    if pin_gate_open(server, session) {
        return true;
    }
    warn!("Account {} skipped the PIN check and is disconnected", session.account_id);
    let _ = read_lock!(context.socket()).shutdown(Shutdown::Both);
    false
}

fn account_context(session: &Session) -> AccountContext {
    AccountContext {
        account_id: session.account_id,
        sex: if session.account.sex == 0 { "F" } else { "M" },
        char_slots: session.account.char_slots,
    }
}

fn load_account(server: &Server, session: &Session) -> Result<AccountRecord, String> {
    server
        .repository
        .account_by_id(session.account_id)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "Account no longer exists".to_string())
}

pub fn send_pin_state(server: &Server, context: &Request, session: &Session, state: u16) {
    let seed = rand::random::<u32>() % 0xFFFF;
    session.account.char_server().pin_seed = seed;
    socket_send_raw!(context, pin_state_packet(seed, session.account_id, state));
}

fn handle_pin_window_request(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    if read_u32(bytes, 2) != session.account_id {
        return Err("PIN window requested for another account".into());
    }
    let config = &server.configuration.char_server.pincode;
    if !config.enabled {
        return Ok(());
    }
    let account = load_account(server, session)?;
    let state = if account.pincode.is_empty() { pincode::STATE_NEW } else { pincode::STATE_ASK };
    send_pin_state(server, context, session, state);
    Ok(())
}

fn fail_pin(server: &Server, context: &Request, session: &Session) {
    let config = &server.configuration.char_server.pincode;
    send_pin_state(server, context, session, pincode::STATE_WRONG);
    let tries = {
        let mut state = session.account.char_server();
        state.pin_tries += 1;
        state.pin_tries
    };
    if config.maxtry != 0 && tries >= config.maxtry {
        LoginService::log_event(
            server.repository.as_ref(),
            &server.configuration.login,
            &crate::server::request_handler::login::client_ip(&context.socket()).to_string(),
            &server.repository.account_by_id(session.account_id).ok().flatten().map(|account| account.username).unwrap_or_default(),
            100,
            "PIN Code check failed",
            now_seconds(),
        );
        let _ = read_lock!(context.socket()).shutdown(Shutdown::Both);
    }
}

fn check_current_pin(server: &Server, context: &Request, session: &Session, account: &AccountRecord, attempt: &str) -> bool {
    if !account.pincode.is_empty() && attempt == account.pincode {
        session.account.char_server().pin_tries = 0;
        return true;
    }
    fail_pin(server, context, session);
    false
}

fn store_pin(server: &Server, session: &Session, pin: &str) -> Result<(), String> {
    let now = now_seconds();
    server
        .repository
        .account_update(session.account_id, &|account| {
            account.pincode = pin.to_string();
            account.pincode_change = now;
        })
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn pin_passed(server: &Server, context: &Request, session: &Session) {
    session.account.char_server().pin_verified = true;
    send_pin_state(server, context, session, pincode::passed_state(server.packetver()));
}

fn handle_pin_check(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    let config = &server.configuration.char_server.pincode;
    if !config.enabled || read_u32(bytes, 2) != session.account_id {
        return Err("Unexpected PIN check".into());
    }
    let seed = session.account.char_server().pin_seed;
    let attempt = pincode::decrypt(seed, &bytes[6..10]).ok_or("Malformed PIN")?;
    let account = load_account(server, session)?;
    if check_current_pin(server, context, session, &account, &attempt) {
        pin_passed(server, context, session);
    }
    Ok(())
}

fn handle_pin_change(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    let config = &server.configuration.char_server.pincode;
    if !config.enabled || read_u32(bytes, 2) != session.account_id {
        return Err("Unexpected PIN change".into());
    }
    let seed = session.account.char_server().pin_seed;
    let old = pincode::decrypt(seed, &bytes[6..10]).ok_or("Malformed PIN")?;
    let new = pincode::decrypt(seed, &bytes[10..14]).ok_or("Malformed PIN")?;
    let account = load_account(server, session)?;
    if !check_current_pin(server, context, session, &account, &old) {
        return Ok(());
    }
    if !pincode::allowed(config, &new) {
        send_pin_state(server, context, session, pincode::STATE_ILLEGAL);
        return Ok(());
    }
    store_pin(server, session, &new)?;
    info!("Pincode changed for AID: {}", session.account_id);
    pin_passed(server, context, session);
    Ok(())
}

fn handle_pin_creation(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    let config = &server.configuration.char_server.pincode;
    if !config.enabled || read_u32(bytes, 2) != session.account_id {
        return Err("Unexpected PIN creation".into());
    }
    let seed = session.account.char_server().pin_seed;
    let pin = pincode::decrypt(seed, &bytes[6..10]).ok_or("Malformed PIN")?;
    if !pincode::allowed(config, &pin) {
        send_pin_state(server, context, session, pincode::STATE_ILLEGAL);
        return Ok(());
    }
    store_pin(server, session, &pin)?;
    info!("Pincode added for AID: {}", session.account_id);
    pin_passed(server, context, session);
    Ok(())
}

fn handle_delete_reservation(server: &Server, context: &Request, session: &Session, bytes: &[u8]) {
    let char_id = read_u32(bytes, 2);
    let relative = (server.packetver() > 20130000 && server.packetver() <= 20141022) || server.packetver() >= 20150513;
    let reservation = char_server_service::reserve_deletion(
        server.repository.as_ref(),
        &server.configuration.char_server,
        session.account_id,
        char_id,
        now_seconds(),
        relative,
    );
    socket_send_raw!(context, delete_reserved_reply(char_id, reservation.result, reservation.date));
}

fn handle_delete_confirmation(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    let char_id = read_u32(bytes, 2);
    let account = load_account(server, session)?;
    let birthdate: [u8; 6] = bytes[6..12].try_into().map_err(|_| "Malformed birth date")?;
    if !birthdate.iter().all(u8::is_ascii_digit) {
        socket_send_raw!(context, delete_result_reply([0x2a, 0x08], char_id, 5));
        return Ok(());
    }
    let result = char_server_service::accept_deletion(
        server.repository.as_ref(),
        &server.configuration.char_server,
        &account,
        char_id,
        birthdate,
        now_seconds(),
    );
    socket_send_raw!(context, delete_result_reply([0x2a, 0x08], char_id, result));
    Ok(())
}

fn handle_delete_cancel(server: &Server, context: &Request, session: &Session, bytes: &[u8]) {
    let char_id = read_u32(bytes, 2);
    let result = char_server_service::cancel_deletion(server.repository.as_ref(), session.account_id, char_id);
    socket_send_raw!(context, delete_result_reply([0x2c, 0x08], char_id, result));
}

fn handle_legacy_delete(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    let char_id = read_u32(bytes, 2);
    let account = load_account(server, session)?;
    let config = &server.configuration.char_server;
    let refuse = |error: u8| vec![0x70, 0x00, error];
    if !char_server_service::deletion_code_matches(&account, &c_string(&bytes[6..]), config.char_del_option) {
        socket_send_raw!(context, refuse(0));
        return Ok(());
    }
    let outcome = char_server_service::delete_character(server.repository.as_ref(), config, session.account_id, char_id, now_seconds());
    match outcome {
        DeleteOutcome::Deleted => {
            socket_send_raw!(context, vec![0x6f, 0x00]);
        }
        DeleteOutcome::Database | DeleteOutcome::BaseLevel | DeleteOutcome::TooEarly => {
            socket_send_raw!(context, refuse(0));
        }
        DeleteOutcome::NotFound => {
            socket_send_raw!(context, refuse(1));
        }
        DeleteOutcome::Guild | DeleteOutcome::Party => {
            socket_send_raw!(context, refuse(2));
        }
    }
    Ok(())
}

fn handle_name_check(server: &Server, context: &Request, session: &Session, bytes: &[u8]) -> Result<(), String> {
    if read_u32(bytes, 2) != session.account_id {
        return Err("Name check requested for another account".into());
    }
    let char_id = read_u32(bytes, 6);
    let owned = server
        .repository
        .char_find(char_id)
        .map_err(|error| error.to_string())?
        .is_some_and(|character| character.account_id == session.account_id as i32);
    if !owned {
        return Ok(());
    }
    let name = char_server_service::normalize_name(&c_string(&bytes[10..34]));
    let valid = char_server_service::check_char_name(server.repository.as_ref(), &server.configuration.char_server, &name) == 0;
    let mut reply = vec![0x8e, 0x02];
    reply.extend(u16::from(valid).to_le_bytes());
    socket_send_raw!(context, reply);
    Ok(())
}

fn handle_rename(server: &Server, context: &Request, session: &Session, bytes: &[u8]) {
    let char_id = read_u32(bytes, 2);
    let Some(result) = char_server_service::rename_character(
        server.repository.as_ref(),
        &server.configuration.char_server,
        session.account_id,
        char_id,
        &c_string(&bytes[6..30]),
        now_seconds(),
    ) else {
        return;
    };
    let mut reply = vec![0xfd, 0x08];
    reply.extend(i32::from(result).to_le_bytes());
    socket_send_raw!(context, reply);
    if result == 0 {
        send_character_list(server, context, session);
    }
}

fn handle_slot_move(server: &Server, context: &Request, session: &Session, bytes: &[u8]) {
    let (from, to) = (read_u16(bytes, 2), read_u16(bytes, 4));
    match char_server_service::move_character_slot(
        server.repository.as_ref(),
        &server.configuration.char_server,
        &account_context(session),
        from,
        to,
    ) {
        Some(moves) => {
            socket_send_raw!(context, slot_move_reply(0, moves));
            send_character_list(server, context, session);
        }
        None => {
            socket_send_raw!(context, slot_move_reply(1, 0));
        }
    }
}

pub fn handle_raw(server: &Server, context: &Request) -> Result<bool, String> {
    let bytes = context.packet().raw();
    if bytes.len() < 2 {
        return Ok(false);
    }
    let id = read_u16(bytes, 0);
    let Some(FrameLength::Fixed(length)) = frame_length(id, server.packetver()) else {
        return Ok(false);
    };
    if bytes.len() != length {
        return Err("Character server packet has the wrong length".into());
    }
    let session = char_session(server, context)?;
    let pin_packet = matches!(
        id,
        CH_SECOND_PASSWD_ACK | CH_MAKE_SECOND_PASSWD | CH_EDIT_SECOND_PASSWD | CH_AVAILABLE_SECOND_PASSWD
    );
    if !pin_packet && !require_pin(server, context, &session) {
        return Ok(true);
    }
    match id {
        CH_AVAILABLE_SECOND_PASSWD => handle_pin_window_request(server, context, &session, bytes)?,
        CH_SECOND_PASSWD_ACK => handle_pin_check(server, context, &session, bytes)?,
        CH_EDIT_SECOND_PASSWD => handle_pin_change(server, context, &session, bytes)?,
        CH_MAKE_SECOND_PASSWD => handle_pin_creation(server, context, &session, bytes)?,
        CH_DELETE_CHAR3_RESERVED => handle_delete_reservation(server, context, &session, bytes),
        CH_DELETE_CHAR3 => handle_delete_confirmation(server, context, &session, bytes)?,
        CH_DELETE_CHAR3_CANCEL => handle_delete_cancel(server, context, &session, bytes),
        CH_DELETE_CHAR_LEGACY | CH_DELETE_CHAR => handle_legacy_delete(server, context, &session, bytes)?,
        CH_REQ_IS_VALID_CHARNAME => handle_name_check(server, context, &session, bytes)?,
        CH_REQ_CHANGE_CHARNAME => handle_rename(server, context, &session, bytes),
        CH_REQ_CHANGE_CHARACTER_SLOT => handle_slot_move(server, context, &session, bytes),
        _ => return Ok(false),
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_match_the_rathena_struct_sizes_at_20120307() {
        let length = |id| frame_length(id, 20120307);
        for (id, bytes) in [
            (0x0827, 6),
            (0x0829, 12),
            (0x082b, 6),
            (0x01fb, 56),
            (0x028d, 34),
            (0x08fc, 30),
            (0x08b8, 10),
            (0x08ba, 10),
            (0x08be, 14),
            (0x08c5, 6),
            (0x08d4, 8),
        ] {
            assert_eq!(length(id), Some(FrameLength::Fixed(bytes)), "{id:#06x}");
        }
        assert_eq!(length(0x0068), None, "the 46 byte delete is only for clients before 20040419");
        assert_eq!(frame_length(0x0068, 20030101), Some(FrameLength::Fixed(46)));
        assert_eq!(frame_length(0x08b8, 20110101), None, "no PIN before 20110309");
        assert_eq!(frame_length(0x08fc, 20111031), None, "no rename before 20111101");
    }

    #[test]
    fn reply_packets_have_the_documented_layouts() {
        assert_eq!(pin_state_packet(0x1234, 7, 4), vec![0xb9, 0x08, 0x34, 0x12, 0, 0, 7, 0, 0, 0, 4, 0]);
        assert_eq!(delete_reserved_reply(9, 1, 100), vec![0x28, 0x08, 9, 0, 0, 0, 1, 0, 0, 0, 100, 0, 0, 0]);
        assert_eq!(delete_result_reply([0x2a, 0x08], 9, 5), vec![0x2a, 0x08, 9, 0, 0, 0, 5, 0, 0, 0]);
        assert_eq!(slot_move_reply(0, 3), vec![0xd5, 0x08, 8, 0, 0, 0, 3, 0]);
        assert_eq!(slot_move_reply(1, 0).len(), 8);
    }

    #[test]
    fn c_string_stops_at_the_first_nul() {
        assert_eq!(c_string(b"abc\0def"), "abc");
        assert_eq!(c_string(b"full"), "full");
    }
}
