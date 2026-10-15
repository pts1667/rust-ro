use std::io::Write;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};

use configuration::account_config::{CharServerConfig, LoginConfig};
use configuration::configuration::ServerConfig;
use packets::packets::{
    Packet, PacketAcAcceptLogin, PacketAcAcceptLogin2, PacketAcRefuseLogin, PacketAcRefuseLoginR2, PacketAcRefuseLoginR3, PacketCaLogin,
    PacketScNotifyBan, ServerAddr, ServerAddr2,
};
use rand::Rng;

use super::framing::FrameLength;
use crate::server::Server;
use crate::server::model::client_socket::ClientSocket;
use crate::server::model::request::Request;
use crate::server::model::session::{AccountSession, Session};
use crate::server::service::login_service::{CreateAccountError, LoginOutcome, LoginRequest, LoginService};

/// `SC_NOTIFY_BAN` results sent to a client that is turned away after a successful authentication.
pub const NOTIFY_SERVER_CLOSED: u8 = 1;
pub const NOTIFY_ALREADY_ONLINE: u8 = 8;
pub const NOTIFY_DISCONNECTED: u8 = 15;

/// Not a client packet: user id (24), password (24) and sex (`M` or `F`) after the header.
const CA_CREATE_ACCOUNT: u16 = 0xC3E0;
/// Not a client packet: result code (`0` when created), then the account id (`0` when refused).
const AC_CREATE_ACCOUNT_RESULT: u16 = 0xC3E1;
const CREATE_ACCOUNT_LENGTH: usize = 2 + 24 + 24 + 1;
/// Not a client packet: no body. Answered with `AC_SERVER_INFO`.
const CA_SERVER_INFO: u16 = 0xC3E2;
/// Not a client packet: accounts required (`1`/`0`), account creation on (`1`/`0`), then the packet version.
const AC_SERVER_INFO: u16 = 0xC3E3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedServer {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub name: String,
    pub users: u16,
    pub kind: u16,
    pub new_flag: u16,
}

pub fn client_ip(socket: &Arc<RwLock<ClientSocket>>) -> Ipv4Addr {
    match read_lock!(socket).peer_addr() {
        Ok(SocketAddr::V4(address)) => *address.ip(),
        Ok(SocketAddr::V6(address)) => address.ip().to_ipv4_mapped().unwrap_or(Ipv4Addr::LOCALHOST),
        Err(_) => Ipv4Addr::LOCALHOST,
    }
}

pub fn advertised_char_ip(server: &ServerConfig, login: &LoginConfig, client: Ipv4Addr) -> Ipv4Addr {
    login
        .subnets
        .iter()
        .find(|subnet| subnet.matches(client))
        .map(|subnet| subnet.char_ip)
        .unwrap_or_else(|| default_advertised_ip(server))
}

pub fn advertised_map_ip(server: &ServerConfig, login: &LoginConfig, client: Ipv4Addr) -> Ipv4Addr {
    login
        .subnets
        .iter()
        .find(|subnet| subnet.matches(client))
        .map(|subnet| subnet.map_ip)
        .unwrap_or_else(|| default_advertised_ip(server))
}

fn default_advertised_ip(server: &ServerConfig) -> Ipv4Addr {
    match server.connect_ip() {
        std::net::IpAddr::V4(ip) => ip,
        std::net::IpAddr::V6(_) => Ipv4Addr::LOCALHOST,
    }
}

pub fn listed_server(server: &ServerConfig, login: &LoginConfig, char_server: &CharServerConfig, client: Ipv4Addr, users: usize) -> ListedServer {
    ListedServer {
        ip: advertised_char_ip(server, login, client),
        port: server.port,
        name: char_server.server_name.chars().take(19).collect(),
        users: users.min(u16::MAX as usize) as u16,
        kind: char_server.char_maintenance,
        new_flag: char_server.char_new_display,
    }
}

pub fn ip_to_wire(ip: Ipv4Addr) -> u32 {
    u32::from_le_bytes(ip.octets())
}

fn name_chars(name: &str) -> [char; 20] {
    let mut chars = [0 as char; 20];
    name.chars().take(19).enumerate().for_each(|(i, c)| chars[i] = c);
    chars
}

fn random_login_id() -> u32 {
    rand::thread_rng().gen_range(1..=u32::MAX)
}

fn sex_to_client(sex: &str) -> u8 {
    match sex {
        "F" => 0,
        "S" => 2,
        _ => 1,
    }
}

fn text(chars: &[char]) -> String {
    chars.iter().take_while(|c| **c != '\0').collect()
}

pub(crate) fn handle_login(server: Arc<Server>, context: Request) {
    let packet_ca_login = cast!(context.packet(), PacketCaLogin);
    info!("packetver {}", packet_ca_login.version);
    let username = text(&packet_ca_login.id);
    let password = text(&packet_ca_login.passwd);
    let ip = client_ip(&context.socket());
    let login_config = &server.configuration.login;
    let request = LoginRequest {
        username: &username,
        password: &password,
        client_ip: ip,
        now_ms: chrono::Utc::now().timestamp_millis(),
    };
    match server.login_service().authenticate(server.repository.as_ref(), login_config, &request) {
        LoginOutcome::Refused { code, unblock_time } => {
            let packet = refuse_login_packet(server.packetver(), code, unblock_time, &login_config.date_format);
            socket_send_raw!(context, packet);
        }
        LoginOutcome::ServerClosed => {
            socket_send_raw!(context, notify_ban_packet(server.packetver(), NOTIFY_SERVER_CLOSED));
        }
        LoginOutcome::Accepted(account) => {
            if let Some(existing) = server.sessions().find(account.account_id) {
                if existing.char_server_socket.is_some() || existing.map_server_socket.is_some() || existing.char_id.is_some() {
                    LoginService::log_event(
                        server.repository.as_ref(),
                        login_config,
                        &ip.to_string(),
                        &account.username,
                        8,
                        "already online",
                        request.now_ms / 1000,
                    );
                    info!("User '{}' is already online - Rejected.", account.username);
                    server.kick_session(&existing, NOTIFY_ALREADY_ONLINE);
                    socket_send_raw!(context, notify_ban_packet(server.packetver(), NOTIFY_ALREADY_ONLINE));
                    return;
                }
                server.sessions().remove(account.account_id);
            }
            LoginService::log_accepted(server.repository.as_ref(), login_config, &request, &account);
            let char_slots = match account.char_slots {
                0 => login_config.default_char_slots(),
                slots => slots.min(configuration::account_config::MAX_CHARS),
            };
            let login_id1 = random_login_id() as i32;
            let login_id2 = random_login_id();
            let session = Session::create_empty(account.account_id, login_id1, login_id2, packet_ca_login.version)
                .with_account(AccountSession::new(sex_to_client(&account.sex), account.group_id, char_slots));
            server.sessions().add(account.account_id, Arc::new(session));
            let listed = listed_server(
                &server.configuration.server,
                login_config,
                &server.configuration.char_server,
                ip,
                server.directory().len(),
            );
            let response = accept_login_packet(server.packetver(), &account.sex, login_id1, login_id2, account.account_id, &listed);
            socket_send_raw!(context, response);
        }
    }
}

pub fn frame_length(id: u16) -> Option<FrameLength> {
    Some(FrameLength::Fixed(match id {
        CA_CREATE_ACCOUNT => CREATE_ACCOUNT_LENGTH,
        CA_SERVER_INFO => 2,
        _ => return None,
    }))
}

/// Routes the custom login packets; returns false for any other packet.
pub fn handle_raw(server: &Server, context: &Request) -> bool {
    let raw = context.packet().raw();
    if raw.len() < 2 {
        return false;
    }
    match u16::from_le_bytes([raw[0], raw[1]]) {
        CA_CREATE_ACCOUNT => {
            handle_create_account(server, context, raw);
            true
        }
        CA_SERVER_INFO => {
            let packet = server_info_packet(&server.configuration.login, client_ip(&context.socket()), server.packetver());
            socket_send_raw!(context, packet);
            true
        }
        _ => false,
    }
}

fn handle_create_account(server: &Server, context: &Request, raw: &[u8]) {
    if raw.len() != CREATE_ACCOUNT_LENGTH {
        warn!("Rejected a malformed account creation packet of {} bytes", raw.len());
        return;
    }
    let username = c_string(&raw[2..26]);
    let password = c_string(&raw[26..50]);
    let sex = char::from(raw[50]).to_ascii_uppercase();
    let request = LoginRequest {
        username: &username,
        password: &password,
        client_ip: client_ip(&context.socket()),
        now_ms: chrono::Utc::now().timestamp_millis(),
    };
    let result = server.login_service().create_account(server.repository.as_ref(), &server.configuration.login, &request, sex);
    socket_send_raw!(context, create_account_result_packet(result));
}

/// What a client on `client` must do to log in, and the packet version the server runs.
pub fn server_info_packet(login: &LoginConfig, client: Ipv4Addr, packetver: u32) -> Vec<u8> {
    let mut packet = AC_SERVER_INFO.to_le_bytes().to_vec();
    packet.push(u8::from(!login.skips_account_system(client)));
    packet.push(u8::from(login.new_account));
    packet.extend_from_slice(&packetver.to_le_bytes());
    packet
}

fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

pub fn create_account_result_packet(result: Result<u32, CreateAccountError>) -> Vec<u8> {
    let (code, account_id) = match result {
        Ok(account_id) => (0, account_id),
        Err(error) => (error as u8, 0),
    };
    let mut packet = AC_CREATE_ACCOUNT_RESULT.to_le_bytes().to_vec();
    packet.push(code);
    packet.extend_from_slice(&account_id.to_le_bytes());
    packet
}

pub fn notify_ban_packet(packetver: u32, result: u8) -> Vec<u8> {
    let mut packet = PacketScNotifyBan::new(packetver);
    packet.set_error_code(result);
    packet.fill_raw();
    packet.raw().to_vec()
}

pub fn accept_login_packet(packetver: u32, sex: &str, login_id1: i32, login_id2: u32, account_id: u32, listed: &ListedServer) -> Vec<u8> {
    if packetver < 20170315 {
        let mut packet = PacketAcAcceptLogin::new(packetver);
        packet.set_packet_length((PacketAcAcceptLogin::base_len(packetver) + ServerAddr::base_len(packetver)) as i16);
        packet.set_aid(account_id);
        packet.set_auth_code(login_id1);
        packet.set_user_level(login_id2);
        packet.set_sex(sex_to_client(sex));
        let mut entry = ServerAddr::new(packetver);
        entry.set_ip(ip_to_wire(listed.ip));
        entry.set_port(listed.port as i16);
        entry.set_name(name_chars(&listed.name));
        entry.set_user_count(listed.users);
        entry.set_state(listed.kind);
        entry.set_property(listed.new_flag);
        packet.set_server_list(vec![entry]);
        packet.fill_raw();
        packet.raw().to_vec()
    } else {
        let mut packet = PacketAcAcceptLogin2::new(packetver);
        packet.set_packet_length((PacketAcAcceptLogin2::base_len(packetver) + ServerAddr2::base_len(packetver)) as i16);
        packet.set_aid(account_id);
        packet.set_auth_code(login_id1);
        packet.set_user_level(login_id2);
        packet.set_sex(sex_to_client(sex));
        let mut entry = ServerAddr2::new(packetver);
        entry.set_ip(ip_to_wire(listed.ip));
        entry.set_port(listed.port as i16);
        entry.set_name(name_chars(&listed.name));
        entry.set_user_count(listed.users);
        entry.set_state(listed.kind);
        entry.set_property(listed.new_flag);
        packet.set_server_list(vec![entry]);
        packet.fill_raw();
        packet.raw().to_vec()
    }
}

pub fn refuse_login_packet(packetver: u32, code: u32, unblock_time: Option<i64>, date_format: &str) -> Vec<u8> {
    if packetver >= 20180627 {
        let mut packet = PacketAcRefuseLoginR3::new(packetver);
        packet.set_error_code(code);
        packet.fill_raw();
        packet.raw().to_vec()
    } else if packetver > 20101123 {
        let mut packet = PacketAcRefuseLoginR2::new(packetver);
        packet.set_error_code(code);
        packet.set_block_date(block_date(unblock_time, date_format));
        packet.fill_raw();
        packet.raw().to_vec()
    } else {
        let mut packet = PacketAcRefuseLogin::new(packetver);
        packet.set_error_code(code.min(u8::MAX as u32) as u8);
        packet.fill_raw();
        packet.raw().to_vec()
    }
}

fn block_date(unblock_time: Option<i64>, date_format: &str) -> [char; 20] {
    use chrono::TimeZone;
    let mut chars = [0 as char; 20];
    let Some(time) = unblock_time.and_then(|seconds| chrono::Local.timestamp_opt(seconds, 0).single()) else {
        return chars;
    };
    use std::fmt::Write as _;
    let mut formatted = String::new();
    if write!(formatted, "{}", time.format(date_format)).is_err() {
        return chars;
    }
    formatted.chars().take(19).enumerate().for_each(|(i, c)| chars[i] = c);
    chars
}

pub fn write_to_socket(socket: &Arc<RwLock<ClientSocket>>, bytes: &[u8]) {
    let mut guard = socket.write().unwrap_or_else(|poisoned| poisoned.into_inner());
    if guard.write_all(bytes).and_then(|_| guard.flush()).is_err() {
        debug!("Could not write to a closed client socket");
    }
}

#[cfg(test)]
mod tests {
    use configuration::account_config::SubnetConfig;

    use super::*;

    fn server_config(host: &str) -> ServerConfig {
        serde_json::from_str(&format!(
            r#"{{"trace_packet": false, "log_level_module_override": [], "port": 6901,
                "enable_visual_debugger": false, "packetver": 20120307, "host": "{host}"}}"#
        ))
        .unwrap()
    }

    #[test]
    fn advertises_loopback_for_a_wildcard_host_and_the_host_otherwise() {
        let login = LoginConfig::default();
        let client: Ipv4Addr = "10.1.1.1".parse().unwrap();
        assert_eq!(advertised_char_ip(&server_config("0.0.0.0"), &login, client), Ipv4Addr::LOCALHOST);
        assert_eq!(advertised_char_ip(&server_config("192.168.1.20"), &login, client), "192.168.1.20".parse::<Ipv4Addr>().unwrap());
    }

    #[test]
    fn subnet_mapping_overrides_the_advertised_addresses_for_matching_clients() {
        let login = LoginConfig {
            subnets: vec![SubnetConfig {
                mask: "255.255.255.0".parse().unwrap(),
                char_ip: "192.168.0.5".parse().unwrap(),
                map_ip: "192.168.0.6".parse().unwrap(),
            }],
            ..LoginConfig::default()
        };
        let config = server_config("203.0.113.9");
        let lan: Ipv4Addr = "192.168.0.77".parse().unwrap();
        let wan: Ipv4Addr = "198.51.100.1".parse().unwrap();
        assert_eq!(advertised_char_ip(&config, &login, lan).to_string(), "192.168.0.5");
        assert_eq!(advertised_map_ip(&config, &login, lan).to_string(), "192.168.0.6");
        assert_eq!(advertised_char_ip(&config, &login, wan).to_string(), "203.0.113.9");
        assert_eq!(advertised_map_ip(&config, &login, wan).to_string(), "203.0.113.9");
    }

    #[test]
    fn server_list_entry_reflects_the_char_server_settings() {
        let char_server = CharServerConfig { server_name: "A very long server name indeed".into(), char_maintenance: 2, char_new_display: 1, ..Default::default() };
        let listed = listed_server(&server_config("0.0.0.0"), &LoginConfig::default(), &char_server, Ipv4Addr::LOCALHOST, 70_000);
        assert_eq!(listed.name.chars().count(), 19);
        assert_eq!((listed.users, listed.kind, listed.new_flag, listed.port), (u16::MAX, 2, 1, 6901));
    }

    #[test]
    fn accept_login_packet_carries_the_server_list_fields_and_account_sex() {
        let listed = ListedServer { ip: "1.2.3.4".parse().unwrap(), port: 6901, name: "Rust".into(), users: 7, kind: 3, new_flag: 1 };
        let bytes = accept_login_packet(20120307, "F", 11, 22, 2_000_001, &listed);
        assert_eq!(&bytes[..2], &[0x69, 0x00]);
        let packet = PacketAcAcceptLogin::from(&bytes, 20120307);
        assert_eq!((packet.aid, packet.auth_code, packet.user_level, packet.sex), (2_000_001, 11, 22, 0));
        let entry = &packet.server_list[0];
        assert_eq!((entry.ip, entry.port, entry.user_count, entry.state, entry.property), (u32::from_le_bytes([1, 2, 3, 4]), 6901, 7, 3, 1));
        assert_eq!(text(&entry.name), "Rust");
    }

    #[test]
    fn refusal_packet_names_the_unblock_time_for_bans() {
        let bytes = refuse_login_packet(20120307, 6, Some(86_400 * 365), "%Y");
        assert_eq!(&bytes[..2], &[0x3e, 0x08]);
        assert_eq!(&bytes[2..6], &6u32.to_le_bytes());
        let date: String = bytes[6..].iter().take_while(|byte| **byte != 0).map(|byte| *byte as char).collect();
        assert_eq!(date, "1971");
        let plain = refuse_login_packet(20120307, 1, None, "%Y");
        assert!(plain[6..].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn notify_ban_packet_is_the_three_byte_auth_result() {
        assert_eq!(notify_ban_packet(20120307, 8), vec![0x81, 0x00, 8]);
    }

    #[test]
    fn create_account_request_is_framed_at_its_fixed_length() {
        let mut packet = CA_CREATE_ACCOUNT.to_le_bytes().to_vec();
        packet.resize(CREATE_ACCOUNT_LENGTH, 0);
        assert_eq!(crate::server::request_handler::framing::ClientFrames::new(20120307).push(&packet).unwrap(), vec![packet]);
    }

    #[test]
    fn create_account_result_is_a_code_and_the_new_account_id() {
        assert_eq!(create_account_result_packet(Ok(2_000_001)), vec![0xE1, 0xC3, 0, 0x81, 0x84, 0x1E, 0x00]);
        assert_eq!(create_account_result_packet(Err(CreateAccountError::NameTaken)), vec![0xE1, 0xC3, 5, 0, 0, 0, 0]);
    }

    #[test]
    fn server_info_request_is_framed_and_answers_with_the_account_mode_of_the_caller() {
        let request = CA_SERVER_INFO.to_le_bytes().to_vec();
        assert_eq!(crate::server::request_handler::framing::ClientFrames::new(20120307).push(&request).unwrap(), vec![request]);
        let open = LoginConfig { new_account: true, ..LoginConfig::default() };
        let packet = server_info_packet(&open, Ipv4Addr::LOCALHOST, 20120307);
        assert_eq!(&packet[..4], &[0xE3, 0xC3, 1, 1]);
        assert_eq!(&packet[4..], &20120307u32.to_le_bytes());
        let skipping = LoginConfig { skip_account_ips: vec!["127.0.0.1".parse().unwrap()], ..LoginConfig::default() };
        assert_eq!(&server_info_packet(&skipping, Ipv4Addr::LOCALHOST, 20120307)[2..4], &[0, 0]);
        assert_eq!(&server_info_packet(&skipping, "10.0.0.1".parse().unwrap(), 20120307)[2..4], &[1, 0]);
    }
}
