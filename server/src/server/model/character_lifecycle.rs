use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};

use tokio::sync::oneshot;

use crate::server::model::script_timer::{NpcTimerKey, ScriptTimerOwner};
use crate::server::model::session::Session;
use crate::server::state::character::Character;

pub type LifecycleReply<T> = Arc<Mutex<Option<oneshot::Sender<Result<T, String>>>>>;

#[derive(Clone)]
pub struct CharacterMemo {
    pub session: Arc<Session>,
}

impl Debug for CharacterMemo {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterMemo")
            .field("char_id", &self.session.char_id)
            .finish()
    }
}

impl PartialEq for CharacterMemo {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session)
    }
}

#[derive(Clone)]
pub struct CharacterRespawn {
    pub session: Arc<Session>,
}

impl Debug for CharacterRespawn {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterRespawn")
            .field("char_id", &self.session.char_id)
            .finish()
    }
}

impl PartialEq for CharacterRespawn {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session)
    }
}

pub struct CharacterMapEntryData {
    pub session: Arc<Session>,
    pub x: u16,
    pub y: u16,
    pub direction: u16,
}

#[derive(Clone)]
pub struct CharacterMapEntry {
    pub session: Arc<Session>,
    pub socket: Arc<RwLock<TcpStream>>,
    pub response: LifecycleReply<CharacterMapEntryData>,
}

impl Debug for CharacterMapEntry {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterMapEntry")
            .field("char_id", &self.session.char_id)
            .finish()
    }
}

impl PartialEq for CharacterMapEntry {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session) && Arc::ptr_eq(&self.socket, &other.socket)
    }
}

#[derive(Clone)]
pub struct CharacterMapReady {
    pub session: Arc<Session>,
}

impl Debug for CharacterMapReady {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterMapReady")
            .field("char_id", &self.session.char_id)
            .finish()
    }
}

impl PartialEq for CharacterMapReady {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session)
    }
}

#[derive(Clone)]
pub struct CharacterSelectionGate {
    pub session: Arc<Session>,
    pub response: LifecycleReply<Arc<Session>>,
}

impl Debug for CharacterSelectionGate {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterSelectionGate")
            .field("account_id", &self.session.account_id)
            .finish()
    }
}

impl PartialEq for CharacterSelectionGate {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session) && Arc::ptr_eq(&self.response, &other.response)
    }
}

#[derive(Clone)]
pub struct CharacterAdmission {
    pub session: Arc<Session>,
    pub character: Arc<Mutex<Option<SelectedCharacter>>>,
    pub response: LifecycleReply<String>,
}

pub struct SelectedCharacter {
    pub char_id: u32,
    pub account_id: u32,
    pub name: String,
    pub status: models::status::Status,
    pub x: u16,
    pub y: u16,
    pub map: String,
    pub sex: u8,
    pub hotkeys: Vec<crate::server::model::hotkey::Hotkey>,
    pub save_map: String,
    pub save_x: u16,
    pub save_y: u16,
    pub position_revision: u64,
    pub options: u64,
    pub karma: i32,
    pub manner: i32,
    pub game_systems: crate::server::model::game_systems::CharacterGameSystems,
    pub account_game_systems: crate::server::model::game_systems::AccountGameSystems,
    pub guild_name: String,
}

impl SelectedCharacter {
    pub fn from_character(character: Character) -> Self {
        let map = character.current_map_name().clone();
        let position_revision = character.position_revision.load(std::sync::atomic::Ordering::Relaxed);
        Self {
            char_id: character.char_id,
            account_id: character.account_id,
            name: character.name,
            status: character.status,
            x: character.x,
            y: character.y,
            map,
            sex: character.sex,
            hotkeys: character.hotkeys,
            save_map: character.save_map,
            save_x: character.save_x,
            save_y: character.save_y,
            position_revision,
            options: character.options,
            karma: character.karma,
            manner: character.manner,
            game_systems: character.game_systems,
            account_game_systems: character.account_game_systems,
            guild_name: character.guild_name,
        }
    }

    pub fn into_character(self) -> Character {
        let mut character = Character::new(
            self.name,
            self.char_id,
            self.account_id,
            self.status,
            self.x,
            self.y,
            0,
            self.map,
            self.sex,
            self.hotkeys,
        );
        character.save_map = self.save_map;
        character.save_x = self.save_x;
        character.save_y = self.save_y;
        character
            .position_revision
            .store(self.position_revision, std::sync::atomic::Ordering::Relaxed);
        character.set_options(self.options);
        character.karma = self.karma;
        character.manner = self.manner;
        character.game_systems = self.game_systems;
        character.account_game_systems = self.account_game_systems;
        character.guild_name = self.guild_name;
        character.refresh_script_context();
        character
    }
}

impl Debug for CharacterAdmission {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterAdmission")
            .field("account_id", &self.session.account_id)
            .finish()
    }
}

impl PartialEq for CharacterAdmission {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session) && Arc::ptr_eq(&self.character, &other.character)
    }
}

#[derive(Clone)]
pub struct CharacterLogout {
    pub session: Arc<Session>,
    pub restart: bool,
}

impl Debug for CharacterLogout {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CharacterLogout")
            .field("char_id", &self.session.char_id)
            .field("restart", &self.restart)
            .finish()
    }
}

impl PartialEq for CharacterLogout {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.session, &other.session) && self.restart == other.restart
    }
}

#[derive(Clone)]
pub struct ClientDisconnected {
    pub socket: Arc<RwLock<TcpStream>>,
}

impl Debug for ClientDisconnected {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ClientDisconnected").finish()
    }
}

impl PartialEq for ClientDisconnected {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.socket, &other.socket)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptLogoutCompleted {
    pub char_id: u32,
    pub token: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptLogoutAction {
    pub char_id: u32,
    pub token: u64,
    pub action: Box<crate::server::model::events::game_event::GameEvent>,
}

pub struct TimerQuitCallback {
    pub key: NpcTimerKey,
    pub entry_id: u32,
    pub timer_ms: u64,
    pub label_count: usize,
}

pub struct ActiveTimerQuit {
    pub token: u64,
    pub key: NpcTimerKey,
    pub npc_entry: u32,
    pub elapsed_ms: u64,
    pub started_at: Option<u128>,
    pub label_count: usize,
}

impl ActiveTimerQuit {
    pub fn elapsed(&self, tick: u128) -> u64 {
        self.elapsed_ms.saturating_add(
            self.started_at
                .map_or(0, |started| tick.saturating_sub(started).min(u128::from(u64::MAX)) as u64),
        )
    }
}

pub struct PendingCharacterLogout {
    pub owner: ScriptTimerOwner,
    pub callbacks: VecDeque<TimerQuitCallback>,
    pub current: Option<ActiveTimerQuit>,
    pub deadline: u128,
}
