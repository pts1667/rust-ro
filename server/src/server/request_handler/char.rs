use std::net::Shutdown::Both;
use std::sync::Arc;

use byteorder::{LittleEndian, WriteBytesExt};
use configuration::account_config::MAX_CHARS;
use database::model::CharLogRecord;
use models::status::KnownSkill;
use movement::position::Position;
use packets::packets::{
    CharacterInfoNeoUnion, Packet, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3, PacketChSelectChar,
    PacketChSendMapInfo, PacketCzEnter2, PacketCzRestart, PacketHcAcceptEnterNeoUnion, PacketHcAcceptEnterNeoUnionHeader,
    PacketHcAcceptMakecharNeoUnion, PacketHcNotifyZonesvr, PacketHcRefuseEnter, PacketMapConnection, PacketZcAcceptEnter2,
    PacketZcInventoryExpansionInfo, PacketZcLoadConfirm, PacketZcOverweightPercent, PacketZcReqDisconnectAck2, PacketZcRestartAck,
    ZserverAddr,
};

use crate::repository::model::char_model::{CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::hotkey::Hotkey;
use crate::server::model::request::Request;
use crate::server::model::session::Session;
use crate::server::model::status::StatusFromDb;
use crate::server::request_handler::char_requests::{require_pin, send_pin_state};
use crate::server::service::char_server_service::{
    self, AccountContext, CREATE_DENIED, CREATE_NAME_TAKEN, CREATE_SLOT_NOT_ELIGIBLE, CreateRequest,
};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::pincode;
use crate::server::state::character::Character;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick_client;

const DEFAULT_WALK_SPEED: u16 = 150;

pub fn handle_char_enter(server: &Server, context: Request) {
    let packet_char_enter = cast!(context.packet(), PacketChEnter);
    let refuse = |context: &Request| {
        let mut res = PacketHcRefuseEnter::new(server.packetver());
        res.set_error_code(0);
        res.fill_raw();
        socket_send!(context, res);
    };
    let Some(session) = server.sessions().find(packet_char_enter.aid) else {
        return refuse(&context);
    };
    let credentials_match = session.auth_code == packet_char_enter.auth_code
        && session.user_level == packet_char_enter.user_level
        && session.account.sex == packet_char_enter.sex;
    if !credentials_match {
        return refuse(&context);
    }
    let config = &server.configuration.char_server;
    let over_capacity = config.max_connect_user == 0
        || config.char_maintenance == 1
        || (config.max_connect_user > 0 && server.directory().len() >= config.max_connect_user as usize);
    if over_capacity && i64::from(session.account.group_id) < i64::from(config.gm_allow_group) {
        return refuse(&context);
    }
    let session = Arc::new(session.recreate_with_char_socket(context.socket()));
    server.sessions().add(packet_char_enter.aid, session.clone());
    // A "account id packet" should be sent just before char info packet
    let mut account_id = vec![];
    account_id
        .write_u32::<LittleEndian>(session.account_id)
        .expect("Unable to write Little endian u32 from session account id");
    socket_send_raw!(context, account_id);
    send_character_list(server, &context, &session);
    send_pin_start(server, &context, &session);
}

fn send_pin_start(server: &Server, context: &Request, session: &Session) {
    let Ok(Some(account)) = server.repository.account_by_id(session.account_id) else {
        return;
    };
    let verified = session.account.char_server().pin_verified;
    let state = pincode::start_state(
        &server.configuration.char_server.pincode,
        &account.pincode,
        account.pincode_change,
        chrono::Utc::now().timestamp(),
        verified,
        server.packetver(),
    );
    send_pin_state(server, context, session, state);
}

/// Options of the character list that depend on the char server configuration.
#[derive(Debug, Clone, Copy)]
pub struct CharacterListOptions {
    pub move_enabled: bool,
    pub moves_unlimited: bool,
}

fn character_info(character: &CharSelectModel, options: CharacterListOptions) -> CharacterInfoNeoUnion {
    let mut info = CharacterInfoNeoUnionWrapped::from(character).data;
    info.set_speed(DEFAULT_WALK_SPEED);
    let can_rename = character.rename > 0;
    info.set_b_is_changed_char_name(u16::from(!can_rename));
    info.set_rename_addon(u32::from(can_rename) as _);
    let slot_moves = if !options.move_enabled {
        0
    } else if options.moves_unlimited {
        1
    } else {
        character.moves
    };
    info.set_slot_addon(slot_moves as _);
    for (field, setter) in [
        (info.head_bottom, CharacterInfoNeoUnion::set_head_bottom as fn(&mut CharacterInfoNeoUnion, u16)),
        (info.head_top, CharacterInfoNeoUnion::set_head_top),
        (info.head_mid, CharacterInfoNeoUnion::set_head_mid),
    ] {
        if field > 0 {
            let view = GlobalConfigService::instance().get_item(field as i32).view.unwrap_or(0) as u16;
            setter(&mut info, view);
        }
    }
    info.fill_raw();
    info
}

pub fn block_character_packet(characters: &[CharSelectModel], now: i64, date_format: &str) -> Vec<u8> {
    use chrono::TimeZone;
    let blocked: Vec<(&CharSelectModel, String)> = characters
        .iter()
        .filter(|character| character.unban_time > now)
        .filter_map(|character| {
            let when = chrono::Local.timestamp_opt(character.unban_time, 0).single()?;
            Some((character, when.format(date_format).to_string()))
        })
        .collect();
    let mut packet = vec![0x0d, 0x02];
    packet.extend(((4 + 24 * blocked.len()) as u16).to_le_bytes());
    for (character, date) in blocked {
        packet.extend((character.char_id as u32).to_le_bytes());
        let mut text = [0u8; 20];
        for (slot, byte) in text.iter_mut().zip(date.bytes().take(19)) {
            *slot = byte;
        }
        packet.extend(text);
    }
    packet
}

pub fn send_character_list(server: &Server, context: &Request, session: &Session) {
    let characters = match server.repository.char_account_characters(session.account_id) {
        Ok(characters) => characters,
        Err(error) => {
            error!("Failed to load the characters of account {}: {error}", session.account_id);
            return;
        }
    };
    let config = &server.configuration.char_server;
    let options = CharacterListOptions { move_enabled: config.char_move_enabled, moves_unlimited: config.char_moves_unlimited };
    let infos: Vec<CharacterInfoNeoUnion> = characters.iter().map(|character| character_info(character, options)).collect();
    let packetver = server.packetver();
    let mut char_list = PacketHcAcceptEnterNeoUnion::new(packetver);
    char_list.set_packet_length((27 + infos.len() * CharacterInfoNeoUnion::base_len(packetver)) as i16);
    char_list.set_char_info(infos);
    char_list.set_premium_start_slot(MAX_CHARS);
    char_list.set_premium_end_slot(MAX_CHARS);
    char_list.set_total_slot_num(MAX_CHARS);
    let list: Box<dyn Packet> = if packetver >= 20130000 {
        let mut header = PacketHcAcceptEnterNeoUnionHeader::new(packetver);
        header.set_char_info(char_list);
        header.set_char_slot(MAX_CHARS as i8);
        header.set_premium_slot_end(MAX_CHARS as i8);
        header.set_premium_slot_start(MAX_CHARS as i8);
        header.set_packet_len(29);
        header.fill_raw_with_packetver(Some(packetver));
        Box::new(header)
    } else {
        char_list.fill_raw_with_packetver(Some(packetver));
        Box::new(char_list)
    };
    let mut bytes = list.raw().to_vec();
    if packetver >= 20060819 {
        bytes.extend(block_character_packet(&characters, chrono::Utc::now().timestamp(), "%Y-%m-%d %H:%M:%S"));
    }
    socket_send_raw!(context, bytes);
}

fn refuse_make_char(context: &Request, code: i32) {
    let error = match code {
        CREATE_NAME_TAKEN => 0x00,
        CREATE_DENIED => 0xFF,
        -3 => 0x01,
        CREATE_SLOT_NOT_ELIGIBLE => 0x03,
        _ => 0xFF,
    };
    socket_send_raw!(context, vec![0x6e, 0x00, error]);
}

pub fn handle_make_char(server: &Server, context: Request) {
    let session = context.session();
    if !require_pin(server, &context, &session) {
        return;
    }
    let name_of = |name: &[char]| name.iter().take_while(|c| **c != '\0').collect::<String>();
    let request = if let Some(packet) = context.packet().as_any().downcast_ref::<PacketChMakeChar3>() {
        CreateRequest {
            name: name_of(&packet.name),
            slot: i32::from(packet.char_num),
            hair_style: i32::from(packet.head),
            hair_color: i32::from(packet.head_pal),
            stats: None,
        }
    } else if let Some(packet) = context.packet().as_any().downcast_ref::<PacketChMakeChar2>() {
        CreateRequest {
            name: name_of(&packet.name),
            slot: i32::from(packet.char_num),
            hair_style: i32::from(packet.head),
            hair_color: i32::from(packet.head_pal),
            stats: None,
        }
    } else if let Some(packet) = context.packet().as_any().downcast_ref::<PacketChMakeChar>() {
        CreateRequest {
            name: name_of(&packet.name),
            slot: i32::from(packet.char_num),
            hair_style: i32::from(packet.head),
            hair_color: i32::from(packet.head_pal),
            stats: Some([packet.str, packet.agi, packet.vit, packet.int, packet.dex, packet.luk].map(i32::from)),
        }
    } else {
        error!("Char creation packet was not recognized");
        return;
    };
    let account = AccountContext {
        account_id: session.account_id,
        sex: if session.account.sex == 0 { "F" } else { "M" },
        char_slots: session.account.char_slots,
    };
    match char_server_service::create_character(
        server.repository.as_ref(),
        &server.configuration.char_server,
        server.configuration.game.max_inventory,
        &account,
        server.packetver(),
        &request,
        chrono::Utc::now().timestamp(),
    ) {
        Ok(created) => {
            let options = CharacterListOptions {
                move_enabled: server.configuration.char_server.char_move_enabled,
                moves_unlimited: server.configuration.char_server.char_moves_unlimited,
            };
            let mut packet = PacketHcAcceptMakecharNeoUnion::new(server.packetver());
            packet.set_charinfo(character_info(&created, options));
            packet.fill_raw_with_packetver(Some(server.packetver()));
            socket_send!(context, packet);
        }
        Err(code) => refuse_make_char(&context, code),
    }
}

pub fn handle_select_char(server: &Server, context: Request) {
    let packet_select_char = cast!(context.packet(), PacketChSelectChar);
    if !require_pin(server, &context, &context.session()) {
        return;
    }
    let reject = |context: &Request| {
        let mut packet = PacketHcRefuseEnter::new(server.packetver());
        packet.set_error_code(0);
        packet.fill_raw();
        socket_send!(context, packet);
    };
    let selected_session = match server.await_character_selection(context.session()) {
        Ok(session) => session,
        Err(error) => {
            warn!("Character selection rejected: {error}");
            return reject(&context);
        }
    };
    let session_id = selected_session.account_id;
    let now = chrono::Utc::now().timestamp();
    let selectable = server
        .repository
        .char_account_characters(session_id)
        .ok()
        .and_then(|characters| characters.into_iter().find(|character| character.char_num == i16::from(packet_select_char.char_num)))
        .filter(|character| character.delete_date == 0 && character.unban_time <= now);
    let Some(char_model) = selectable else {
        warn!("Account {session_id} selected a missing, banned or deleted character");
        return reject(&context);
    };
    let skills: Vec<KnownSkill> = server
        .runtime()
        .block_on(async { server.repository.character_skills(char_model.char_id as u32).await.unwrap() });
    let hotkeys: Vec<Hotkey> = server
        .runtime()
        .block_on(async { server.repository.load_hotkeys(char_model.char_id as u32).await.unwrap() });

    let char_id: u32 = char_model.char_id as u32;
    let config = &server.configuration.char_server;
    let (last_map, last_x, last_y) = if char_model.last_map.is_empty() {
        (config.default_map.clone(), config.default_map_x, config.default_map_y)
    } else {
        (char_model.last_map.clone(), char_model.last_x as u16, char_model.last_y as u16)
    };
    if config.log_char {
        let record = CharLogRecord {
            time: now,
            account_id: session_id,
            char_slot: char_model.char_num,
            name: char_model.name.clone(),
            message: "select char".into(),
        };
        if let Err(error) = server.repository.char_log(record) {
            warn!("Failed to write the character log: {error}");
        }
    }

    let mut character = Character::new(
        char_model.name.clone(),
        char_id,
        session_id,
        StatusFromDb::from_char_model(&char_model, &server.configuration.game, skills),
        last_x,
        last_y,
        0,
        last_map,
        selected_session.account.sex,
        hotkeys,
    );
    character.save_map = char_model.save_map.clone();
    character.save_x = char_model.save_x.max(0) as u16;
    character.save_y = char_model.save_y.max(0) as u16;
    character
        .position_revision
        .store(char_model.position_revision, std::sync::atomic::Ordering::Relaxed);
    character.set_options(char_model.option as u32 as u64);
    character.karma = char_model.karma;
    character.manner = char_model.manner;
    match server.repository.character_game_systems(char_id) {
        Ok(systems) => character.game_systems = systems,
        Err(error) => {
            error!("Failed to load character game systems: {error}");
            return;
        }
    }
    match server.repository.account_game_systems(character.account_id) {
        Ok(systems) => character.account_game_systems = systems,
        Err(error) => {
            error!("Failed to load account game systems: {error}");
            return;
        }
    }
    if character.game_systems.guild_id != 0 {
        match server.repository.guild(character.game_systems.guild_id) {
            Ok(Some(guild)) => character.guild_name = guild.name,
            Ok(None) => {}
            Err(error) => {
                error!("Failed to load guild: {error}");
                return;
            }
        }
    }
    character.refresh_script_context();
    let char_id = character.char_id;
    let map = match server.admit_selected_character(selected_session, character) {
        Ok(map) => map,
        Err(error) => {
            warn!("Character admission rejected: {error}");
            let mut packet = PacketHcRefuseEnter::new(server.packetver());
            packet.set_error_code(0);
            packet.fill_raw();
            socket_send!(context, packet);
            return;
        }
    };
    let mut map_name = [0 as char; 16];
    map.fill_char_array(map_name.as_mut());
    if server.packetver() < 20170329 {
        let mut packet_ch_send_map_info = PacketHcNotifyZonesvr::new(GlobalConfigService::instance().packetver());
        packet_ch_send_map_info.set_gid(char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        let mut zserver_addr = ZserverAddr::new(GlobalConfigService::instance().packetver());
        zserver_addr.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        zserver_addr.set_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_addr(zserver_addr);
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    } else {
        let mut packet_ch_send_map_info = PacketChSendMapInfo::new(GlobalConfigService::instance().packetver());
        packet_ch_send_map_info.set_gid(char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        packet_ch_send_map_info.set_map_server_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_map_server_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    }
}

pub fn handle_enter_game(server: &Server, context: Request) {
    let aid;
    let auth_code;
    info!("handle_enter_game");
    if context.packet().as_any().downcast_ref::<PacketCzEnter2>().is_some() {
        let packet_enter_game = cast!(context.packet(), PacketCzEnter2);
        aid = packet_enter_game.aid;
        auth_code = packet_enter_game.auth_code;
    } else {
        error!("Not recognized PacketCzEnterX");
        return;
    }
    let Some(session) = server.sessions().find(aid) else {
        write_lock!(context.socket())
            .shutdown(Both)
            .expect("Unable to shutdown incoming socket. Shutdown was done because session does not exists");
        return;
    };
    if auth_code != session.auth_code || session.char_id.is_none() {
        write_lock!(context.socket())
            .shutdown(Both)
            .expect("Unable to shutdown incoming socket. Shutdown was done because packet auth_code mismatching session auth_code");
        return;
    }
    let entry = match server.admit_character_map_entry(session, context.socket()) {
        Ok(entry) => entry,
        Err(error) => {
            warn!("Map entry rejected: {error}");
            let _ = write_lock!(context.socket()).shutdown(Both);
            return;
        }
    };
    let mut packet_map_connection = PacketMapConnection::new(GlobalConfigService::instance().packetver());
    packet_map_connection.set_aid(aid);
    packet_map_connection.fill_raw();

    socket_send!(context, packet_map_connection);

    /*
    Client expect multiple packets in response to packet PacketCzEnter2
    */
    let mut packet_inventory_expansion_info = PacketZcInventoryExpansionInfo::new(GlobalConfigService::instance().packetver());
    packet_inventory_expansion_info.fill_raw();
    let mut packet_overweight_percent = PacketZcOverweightPercent::new(GlobalConfigService::instance().packetver());
    packet_overweight_percent.fill_raw();
    let mut packet_accept_enter = PacketZcAcceptEnter2::new(GlobalConfigService::instance().packetver());
    packet_accept_enter.set_start_time(get_tick_client());
    packet_accept_enter.set_x_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_y_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_font(0);
    packet_accept_enter.set_pos_dir(
        Position {
            x: entry.x,
            y: entry.y,
            dir: entry.direction,
        }
        .to_pos(),
    );
    packet_accept_enter.fill_raw();

    socket_send!(context, packet_accept_enter);

    /*
     * Inventory
     */
    server.add_to_next_tick(GameEvent::CharacterMapReady(
        crate::server::model::character_lifecycle::CharacterMapReady { session: entry.session },
    ));
}

pub fn handle_restart(server: &Server, context: Request) {
    let packet_restart = cast!(context.packet(), PacketCzRestart);
    let session = context.session();
    if !session
        .map_server_socket
        .as_ref()
        .is_some_and(|socket| std::sync::Arc::ptr_eq(socket, &context.socket()))
    {
        return;
    }
    if packet_restart.atype == 0 {
        server.add_to_next_tick(GameEvent::CharacterRespawn(
            crate::server::model::character_lifecycle::CharacterRespawn { session },
        ));
        return;
    }
    if packet_restart.atype != 1 {
        return;
    }
    server.add_to_next_tick(GameEvent::CharacterLogout(
        crate::server::model::character_lifecycle::CharacterLogout { session, restart: true },
    ));

    let mut restart_ack = PacketZcRestartAck::new(GlobalConfigService::instance().packetver());
    restart_ack.set_atype(packet_restart.atype);
    restart_ack.fill_raw();
    socket_send!(context, restart_ack);
}

pub fn handle_disconnect(server: &Server, context: Request) {
    let session = context.session();
    if !session
        .map_server_socket
        .as_ref()
        .is_some_and(|socket| std::sync::Arc::ptr_eq(socket, &context.socket()))
    {
        return;
    }
    server.add_to_next_tick(GameEvent::CharacterLogout(
        crate::server::model::character_lifecycle::CharacterLogout { session, restart: false },
    ));

    let mut disconnect_ack = PacketZcReqDisconnectAck2::new(GlobalConfigService::instance().packetver());
    disconnect_ack.fill_raw();
    socket_send!(context, disconnect_ack);
}

/// First client version with `ZC_NOTIFY_ACTORINIT` (`0x0b1b`); earlier clients get no reply.
const LOAD_CONFIRM_PACKETVER: u32 = 20190403;

pub fn handle_blocking_play_cancel(context: Request) {
    let packetver = GlobalConfigService::instance().packetver();
    if packetver < LOAD_CONFIRM_PACKETVER {
        return;
    }
    let mut packet_zc_load_confirm = PacketZcLoadConfirm::new(packetver);
    packet_zc_load_confirm.fill_raw();
    socket_send!(context, packet_zc_load_confirm);
}
