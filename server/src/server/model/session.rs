use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};

use dashmap::DashMap;

use packets::packets::{Packet, PacketUnknown};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use tokio::sync::mpsc::Sender;

use crate::server::script::PlayerInput;
use crate::server::state::character_directory::CharacterPresence;

#[derive(Clone)]
pub struct SessionBinding(Weak<Session>);

impl SessionBinding {
    pub fn new(session: &Arc<Session>) -> Self { Self(Arc::downgrade(session)) }
    pub fn matches(&self, session: &Arc<Session>) -> bool {
        self.0.upgrade().is_some_and(|original| Arc::ptr_eq(&original, session))
    }
}

impl std::fmt::Debug for SessionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SessionBinding").field(&self.0.as_ptr()).finish()
    }
}

impl PartialEq for SessionBinding {
    fn eq(&self, other: &Self) -> bool { Weak::ptr_eq(&self.0, &other.0) }
}

pub struct Session {
    pub char_server_socket: Option<Arc<RwLock<TcpStream>>>,
    pub map_server_socket: Option<Arc<RwLock<TcpStream>>>,
    pub account_id: u32,
    // random value, known as login_id1 in hercules
    pub auth_code: i32,
    // random value, known as login_id2 in hercules
    pub user_level: u32,
    pub char_id: Option<u32>,
    pub packetver: u32,
    pub is_simulated: bool,
    pub script_generation: AtomicU64,
    pub script_handler_channel_sender: Mutex<Option<Sender<PlayerInput>>>,
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.account_id == other.account_id
    }
}
impl Eq for Session {}
impl Hash for Session {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.account_id.hash(state);
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionRecord {
    pub session_id: u32,
    pub char_id: Option<u32>,
    pub map_name: String,
    pub position: Position,
    packetver: u32,
    pub entries: Mutex<Vec<SessionRecordEntry>>,
    #[serde(skip)]
    pub entries_formatted: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    dir: u16,
}

const PACKET_TO_RECORDS: &'static [&'static str; 2] = &["PacketCzRequestMove", "PacketCzPlayerChat"];
impl SessionRecord {
    pub fn new(char_id: u32, presence: &CharacterPresence, packetver: u32) -> Self {
        Self {
            session_id: presence.account_id,
            char_id: Some(char_id),
            map_name: presence.map_name.clone(),
            position: Position {
                x: presence.x,
                y: presence.y,
                dir: presence.dir,
            },
            entries: Mutex::new(vec![]),
            packetver,
            entries_formatted: String::new(),
        }
    }

    pub fn record(&self, tick: u128, packet_id: String, packet_name: String, packet: &dyn Packet) {
        if PACKET_TO_RECORDS.contains(&packet_name.as_str()) {
            let packet_to_json = packet.to_json(self.packetver);
            debug!("Recording {}", packet_to_json);
            self.entries.lock().unwrap().push(SessionRecordEntry {
                time: tick,
                packet_id,
                packetver: self.packetver,
                packet_name,
                data: RawValue::from_string(packet_to_json).unwrap(),
                packet: None,
            });
        } else {
            debug!("Will not record packet with name {}", packet_name);
        }
    }

    pub fn finish(&self) {
        if self.entries.lock().unwrap().is_empty() {
            return;
        }
        let mut file = File::create(format!("target/session_{}.json", self.session_id)).unwrap();
        file.write_all(&serde_json::to_string_pretty(self).unwrap().as_bytes()).unwrap();
    }

    pub fn format_entries(&mut self) -> &String {
        if !self.entries_formatted.is_empty() {
            return &self.entries_formatted;
        }
        let mut result = Vec::new();
        let mut count_map = HashMap::new();
        let mut last_name = String::new();
        let entries_guard = self.entries.lock().unwrap();

        for entry in entries_guard.iter() {
            let name = &entry.packet_name;
            if name.eq(&last_name) {
                *count_map.entry(name.clone()).or_insert(1) += 1;
            } else {
                if let Some(count) = count_map.remove(&last_name) {
                    if count > 1 {
                        result.push(format!("{}({})", last_name, count));
                    } else if !last_name.is_empty() {
                        result.push(last_name.clone());
                    }
                }
                last_name = name.clone();
                count_map.insert(name.clone(), 1);
            }
        }

        if let Some(count) = count_map.remove(&last_name) {
            if count > 1 {
                result.push(format!("{}({})", last_name, count));
            } else if !last_name.is_empty() {
                result.push(last_name.clone());
            }
        }

        self.entries_formatted = result.join(", ");
        &self.entries_formatted
    }
}

impl Clone for SessionRecord {
    fn clone(&self) -> Self {
        Self {
            session_id: self.session_id,
            char_id: self.char_id.clone(),
            map_name: self.map_name.clone(),
            position: self.position.clone(),
            packetver: self.packetver,
            entries: Mutex::new(self.entries.lock().unwrap().clone()),
            entries_formatted: self.entries_formatted.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionRecordEntry {
    pub time: u128,
    pub packet_id: String,
    pub packetver: u32,
    pub packet_name: String,
    pub data: Box<RawValue>,
    #[serde(skip)]
    packet: Option<Box<dyn Packet>>,
}

unsafe impl Send for SessionRecordEntry {}
unsafe impl Sync for SessionRecordEntry {}

impl Clone for SessionRecordEntry {
    fn clone(&self) -> Self {
        Self {
            time: self.time,
            packet_id: self.packet_id.clone(),
            packetver: self.packetver,
            packet_name: self.packet_name.to_string(),
            data: self.data.clone(),
            packet: None,
        }
    }
}

impl SessionRecordEntry {
    pub fn packet(&mut self) -> &Box<dyn Packet> {
        if let Some(ref packet) = self.packet {
            return packet;
        }
        let result = packets::packets_parser::parse_json(self.data.get(), self.packetver);
        if let Ok(mut packet) = result {
            packet.fill_raw();
            self.packet = Some(packet);
        } else {
            error!("can't parse packet from session record due to {}", result.err().unwrap());
            self.packet = Some(Box::new(PacketUnknown {
                raw: vec![],
                packet_id: "".to_string(),
            }));
        }
        self.packet.as_ref().unwrap()
    }
}

/// Concurrent session registry shared by the network threads and the game loop.
/// Never hold a returned reference across another registry call: lookups clone the `Arc`.
#[derive(Clone, Default)]
pub struct SessionRegistry(Arc<DashMap<u32, Arc<Session>>>);

impl SessionRegistry {
    pub fn add(&self, session_id: u32, session: Arc<Session>) {
        self.0.insert(session_id, session);
    }

    pub fn remove(&self, session_id: u32) {
        if let Some((_, session)) = self.0.remove(&session_id) {
            session.cancel_script();
        }
    }

    pub fn find(&self, session_id: u32) -> Option<Arc<Session>> {
        self.0.get(&session_id).map(|session| session.clone())
    }

    pub fn get(&self, session_id: u32) -> Arc<Session> {
        self.find(session_id).unwrap()
    }

    pub fn find_by_char_id(&self, char_id: u32) -> Option<Arc<Session>> {
        self.0.iter().find(|entry| entry.char_id == Some(char_id)).map(|entry| entry.value().clone())
    }

    pub fn find_by_map_socket(&self, socket: &Arc<RwLock<TcpStream>>) -> Option<Arc<Session>> {
        self.0
            .iter()
            .find(|entry| entry.map_server_socket.as_ref().is_some_and(|map_socket| Arc::ptr_eq(map_socket, socket)))
            .map(|entry| entry.value().clone())
    }

    pub fn for_each(&self, mut action: impl FnMut(&Arc<Session>)) {
        let sessions: Vec<Arc<Session>> = self.0.iter().map(|entry| entry.value().clone()).collect();
        sessions.iter().for_each(&mut action);
    }

    pub fn find_by_stream(&self, tcp_stream: &TcpStream) -> Option<u32> {
        let found = self.0.iter().find(|entry| {
            let session = entry.value();
            if session.map_server_socket.is_some() {
                let map_server_socket = read_lock!(session.map_server_socket.as_ref().unwrap());
                let is_map_stream = if session.is_simulated {
                    map_server_socket.peer_addr().is_ok() && map_server_socket.local_addr().unwrap() == tcp_stream.peer_addr().unwrap()
                } else {
                    map_server_socket.peer_addr().is_ok() && map_server_socket.peer_addr().unwrap() == tcp_stream.peer_addr().unwrap()
                };
                if is_map_stream {
                    return true;
                }
            }
            if session.char_server_socket.is_none() {
                return false;
            }
            let char_server_socket = read_lock!(session.char_server_socket.as_ref().unwrap());
            if char_server_socket.peer_addr().is_err() {
                return false;
            }
            char_server_socket.peer_addr().unwrap() == tcp_stream.peer_addr().unwrap()
        });
        found.map(|entry| *entry.key())
    }
}

impl Session {
    pub fn create_empty(account_id: u32, auth_code: i32, user_level: u32, packetver: u32) -> Session {
        Session {
            char_server_socket: None,
            map_server_socket: None,
            account_id,
            auth_code,
            user_level,
            char_id: None,
            packetver,
            is_simulated: false,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn recreate_with_char_socket(&self, char_socket: Arc<RwLock<TcpStream>>) -> Session {
        Session {
            char_server_socket: Some(char_socket),
            map_server_socket: self.map_server_socket.clone(),
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: self.char_id,
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn recreate_with_map_socket(&self, map_socket: Arc<RwLock<TcpStream>>) -> Session {
        Session {
            char_server_socket: self.char_server_socket.clone(),
            map_server_socket: Some(map_socket),
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: self.char_id,
            packetver: self.packetver,
            is_simulated: false,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn create_with_map_socket_and_char_id(
        account_id: u32,
        char_id: u32,
        packetver: u32,
        map_socket: Arc<RwLock<TcpStream>>,
    ) -> Session {
        Session {
            char_server_socket: None,
            map_server_socket: Some(map_socket),
            account_id,
            auth_code: 0,
            user_level: 99,
            char_id: Some(char_id),
            packetver,
            is_simulated: true,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn recreate_with_character(&self, char_id: u32) -> Session {
        Session {
            char_server_socket: self.char_server_socket.clone(),
            map_server_socket: self.map_server_socket.clone(),
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: Some(char_id),
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn recreate_without_character(&self) -> Session {
        Session {
            char_server_socket: self.char_server_socket.clone(),
            map_server_socket: None,
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: None,
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn snapshot(&self) -> Session {
        Session {
            char_server_socket: None,
            map_server_socket: None,
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: self.char_id,
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_generation: AtomicU64::new(0),
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn set_script_handler_channel_sender(&self, sender: Sender<PlayerInput>) -> u64 {
        let mut channel = self.script_handler_channel_sender.lock().unwrap();
        let generation = self.script_generation.fetch_add(1, Ordering::AcqRel) + 1;
        *channel = Some(sender);
        generation
    }

    pub fn finish_script(&self, generation: u64) {
        let mut channel = self.script_handler_channel_sender.lock().unwrap();
        if self.script_generation.load(Ordering::Acquire) == generation {
            channel.take();
        }
    }

    pub fn cancel_script(&self) {
        let mut channel = self.script_handler_channel_sender.lock().unwrap();
        self.script_generation.fetch_add(1, Ordering::AcqRel);
        channel.take();
    }

    pub fn char_id(&self) -> u32 {
        self.char_id.expect("Expect char_id to be set in the session, but was not")
    }

    pub fn disconnect(&self) {
        self.cancel_script();
        if let Some(socket) = self.char_server_socket.as_ref() {
            let write_guard = socket.write().unwrap();
            if let Ok(_) = write_guard.shutdown(Shutdown::Both) {
                debug!("Disconnected from char server");
            }
        }
        if let Some(socket) = self.map_server_socket.as_ref() {
            let write_guard = socket.write().unwrap();
            if let Ok(_) = write_guard.shutdown(Shutdown::Both) {
                debug!("Disconnected from map server");
            }
        }
    }
}
