use std::collections::HashSet;
use std::cell::RefCell;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, Weak, mpsc};
use std::thread;
use std::thread::Scope;
use std::time::Duration;

use configuration::configuration::Config;
use model::events::client_notification::{AreaNotificationRangeType, Notification};
use model::events::game_event::GameEvent;
use model::events::persistence_event::PersistenceEvent;
use script::ScriptVm;
use script::skill::ScriptSkillService;
use crate::server::script::ItemVm;
use tokio::runtime::Runtime;

use crate::repository::Repository;
use crate::server::bots::BotRegistry;
use crate::server::game_loop::GAME_TICK_RATE;
use crate::server::model::map_item::MapItems;
use crate::server::model::session::{SessionRecord, SessionRegistry};
use crate::server::model::battleground::Battlegrounds;
use crate::server::model::character_lifecycle::CharacterSelectionGate;
use crate::server::model::duel::Duels;
use crate::server::model::map_flag_overrides::{MapFlagOverrides, SiegeFlag};
use crate::server::model::motd::Motd;
use crate::server::model::notification_backlog::NotificationBacklog;
use crate::server::model::permission_groups::PermissionGroups;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;
use crate::server::service::login_service::LoginService;
use crate::server::service::script_service::ScriptService;
use crate::server::model::client_handle::ClientHandle;
use crate::server::service::script_world_service::ScriptWorldService;
use crate::server::service::server_service::ServerService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character_directory::CharacterDirectory;
use crate::server::state::server::ServerState;
use crate::util::packet::{PacketDirection, debug_packets_from_vec};
use crate::util::tick::{delayed_tick, get_tick};

pub mod boot;
pub mod bots;
pub mod client_data;
pub mod forum;
pub mod reference;
mod game_loop;
pub mod map_instance_loop;
pub mod model;
pub mod persistence;
pub mod plugins;
pub mod request_handler;
pub mod script;
pub mod service;
pub mod state;
pub mod websocket;
mod client_io;

pub(crate) type StateGuard<'a> = parking_lot::MutexGuard<'a, ServerState>;

const STATE_LOCK_TIMEOUT: Duration = Duration::from_secs(10);

thread_local!(pub static PACKETVER: RefCell<u32> = const { RefCell::new(0) });
// Todo make this configurable
pub const PLAYER_FOV: u16 = 20;
pub const MOB_FOV: u16 = 14;

fn load_permission_groups(path: &str) -> PermissionGroups {
    if !std::path::Path::new(path).exists() {
        warn!("Group file {path} does not exist, only the default Player group is defined");
        return PermissionGroups::builtin();
    }
    PermissionGroups::load(path).unwrap_or_else(|error| panic!("Failed to load the group file {path}: {error}"))
}

pub struct Server {
    pub configuration: &'static Config,
    pub repository: Arc<dyn Repository>,
    state: parking_lot::Mutex<ServerState>,
    sessions: SessionRegistry,
    directory: CharacterDirectory,
    duels: Duels,
    instances: crate::server::model::instance::Instances,
    battlegrounds: Battlegrounds,
    map_flag_overrides: MapFlagOverrides,
    siege: SiegeFlag,
    cell_basilica: Mutex<HashSet<u32>>,
    character_selection_waiters: Mutex<Vec<CharacterSelectionGate>>,
    map_notifications: NotificationBacklog,
    tasks_queue: Arc<TasksQueue<GameEvent>>,
    movement_tasks_queue: Arc<TasksQueue<GameEvent>>,
    server_service: ServerService,
    shutdown: AtomicBool,
    runtime: Arc<Runtime>,
    recording_sessions: Mutex<Vec<Arc<SessionRecord>>>,
    shared: OnceLock<Weak<Server>>,
    script_world_service: ScriptWorldService,
    login_service: LoginService,
    motd: Motd,
    day_night: crate::server::model::day_night::DayNight,
    bots: BotRegistry,
    plugin_ui: crate::server::plugins::ui::PluginUiStore,
}

impl Server {
    /// Only the game loop and the movement loop take this lock for the length of an iteration, everything else receives
    /// `&mut ServerState` as a parameter. Panics instead of hanging when the lock can't be taken, a timeout means a
    /// function called `lock_state` while its caller already held the state.
    pub(crate) fn lock_state(&self) -> StateGuard<'_> {
        self.state
            .try_lock_for(STATE_LOCK_TIMEOUT)
            .unwrap_or_else(|| panic!("Timed out locking ServerState"))
    }

    #[cfg(any(test, feature = "visual_debugger"))]
    pub fn state(&self) -> StateGuard<'_> {
        self.lock_state()
    }

    #[cfg(test)]
    pub fn state_mut(&self) -> StateGuard<'_> {
        self.lock_state()
    }

    /// Session registry for threads that must not read `ServerState`.
    pub fn sessions(&self) -> &SessionRegistry {
        &self.sessions
    }

    /// Character positions for threads that must not read `ServerState`.
    pub fn directory(&self) -> &CharacterDirectory {
        &self.directory
    }

    pub fn day_night(&self) -> &crate::server::model::day_night::DayNight {
        &self.day_night
    }

    pub fn bots(&self) -> &BotRegistry {
        &self.bots
    }

    /// Plugin windows and bot observations by character, see `plugins::ui`.
    pub fn plugin_ui(&self) -> &crate::server::plugins::ui::PluginUiStore {
        &self.plugin_ui
    }

    pub fn motd(&self) -> &Motd {
        &self.motd
    }

    pub fn login_service(&self) -> &LoginService {
        &self.login_service
    }

    pub fn duels(&self) -> &Duels {
        &self.duels
    }

    pub fn instances(&self) -> &crate::server::model::instance::Instances {
        &self.instances
    }

    pub fn battlegrounds(&self) -> &Battlegrounds {
        &self.battlegrounds
    }

    pub fn map_flag_overrides(&self) -> &MapFlagOverrides {
        &self.map_flag_overrides
    }

    pub fn siege(&self) -> &SiegeFlag {
        &self.siege
    }

    pub(crate) fn cell_basilica(&self) -> MutexGuard<'_, HashSet<u32>> {
        self.cell_basilica.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub(crate) fn pop_task(&self) -> Option<Vec<GameEvent>> {
        self.tasks_queue.pop()
    }

    pub(crate) fn pop_movement_task(&self) -> Option<Vec<GameEvent>> {
        self.movement_tasks_queue.pop()
    }

    pub fn packetver(&self) -> u32 {
        self.configuration.server.packetver
    }

    pub fn bind_shared(self: &Arc<Self>) {
        let _ = self.shared.set(Arc::downgrade(self));
    }

    pub(crate) fn shared(&self) -> Option<Arc<Self>> {
        self.shared.get().and_then(Weak::upgrade)
    }

    pub(crate) fn script_world_service(&self) -> &ScriptWorldService {
        &self.script_world_service
    }

    pub fn new(
        configuration: &'static Config,
        repository: Arc<dyn Repository>,
        map_items: MapItems,
        npc_script_vm: Arc<ScriptVm>,
        item_script_vm: Arc<ItemVm>,
        client_notification_sender: SyncSender<Notification>,
        persistence_event_sender: SyncSender<PersistenceEvent>,
        runtime: Arc<Runtime>,
    ) -> Server {
        let tasks_queue = Arc::new(TasksQueue::new());
        let script_world_service = ScriptWorldService::new(
            client_notification_sender.clone(),
            repository.clone(),
            GlobalConfigService::instance(),
        );
        let movement_tasks_queue = Arc::new(TasksQueue::new());
        StatusService::init(GlobalConfigService::instance(), item_script_vm.clone());

        let server_service = ServerService::new(
            client_notification_sender.clone(),
            GlobalConfigService::instance(),
            tasks_queue.clone(),
            movement_tasks_queue.clone(),
            npc_script_vm.clone(),
            InventoryService::new(
                client_notification_sender.clone(),
                persistence_event_sender.clone(),
                repository.clone(),
                GlobalConfigService::instance(),
                tasks_queue.clone(),
            ),
            BattleService::new(
                client_notification_sender.clone(),
                StatusService::instance(),
                GlobalConfigService::instance(),
                BattleResultMode::Normal,
            ),
            SkillService::new(
                client_notification_sender.clone(),
                persistence_event_sender.clone(),
                BattleService::new(
                    client_notification_sender.clone(),
                    StatusService::instance(),
                    GlobalConfigService::instance(),
                    BattleResultMode::Normal,
                ),
                StatusService::instance(),
                GlobalConfigService::instance(),
            ),
            StatusService::instance(),
            ScriptService::new(
                client_notification_sender.clone(),
                GlobalConfigService::instance(),
                repository.clone(),
                tasks_queue.clone(),
                npc_script_vm.clone(),
            ),
            CharacterService::new(
                client_notification_sender.clone(),
                persistence_event_sender.clone(),
                repository.clone(),
                GlobalConfigService::instance(),
                SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
                StatusService::instance(),
                tasks_queue.clone(),
            ),
            SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
            ItemService::new(
                client_notification_sender.clone(),
                persistence_event_sender.clone(),
                repository.clone(),
                item_script_vm,
                GlobalConfigService::instance(),
            ),
            ScriptSkillService::new(
                client_notification_sender.clone(),
                persistence_event_sender.clone(),
                repository.clone(),
                GlobalConfigService::instance(),
            ),
        );
        let mut state = ServerState::new(map_items);
        let mut groups = load_permission_groups(&configuration.server.groups_path);
        crate::server::service::plugin_command_service::register_plugin_commands(npc_script_vm.plugins(), &mut groups);
        state.set_permission_groups(groups);
        let sessions = state.sessions().clone();
        let directory = state.directory().clone();
        let map_flag_overrides = state.map_flag_overrides().clone();
        let siege = state.siege().clone();
        Server {
            configuration,
            repository,
            tasks_queue,
            state: parking_lot::Mutex::new(state),
            sessions,
            directory,
            duels: Duels::default(),
            instances: Default::default(),
            battlegrounds: Battlegrounds::default(),
            map_flag_overrides,
            siege,
            cell_basilica: Mutex::new(HashSet::new()),
            character_selection_waiters: Mutex::new(Vec::new()),
            map_notifications: NotificationBacklog::default(),
            movement_tasks_queue,
            server_service,
            shutdown: AtomicBool::new(false),
            recording_sessions: Mutex::new(vec![]),
            shared: OnceLock::new(),
            script_world_service,
            login_service: LoginService::new(),
            motd: Motd::load(&configuration.server.motd_path),
            day_night: Default::default(),
            bots: Default::default(),
            plugin_ui: Default::default(),
            runtime,
        }
    }

    pub fn new_without_service_init(
        configuration: &'static Config,
        repository: Arc<dyn Repository>,
        map_items: MapItems,
        tasks_queue: Arc<TasksQueue<GameEvent>>,
        server_service: ServerService,
        runtime: Arc<Runtime>,
    ) -> Server {
        let script_world_service = ScriptWorldService::new(
            server_service.notification_sender(),
            repository.clone(),
            GlobalConfigService::instance(),
        );
        let state = ServerState::new(map_items);
        let sessions = state.sessions().clone();
        let directory = state.directory().clone();
        let map_flag_overrides = state.map_flag_overrides().clone();
        let siege = state.siege().clone();
        Server {
            configuration,
            repository,
            state: parking_lot::Mutex::new(state),
            sessions,
            directory,
            duels: Duels::default(),
            instances: Default::default(),
            battlegrounds: Battlegrounds::default(),
            map_flag_overrides,
            siege,
            cell_basilica: Mutex::new(HashSet::new()),
            character_selection_waiters: Mutex::new(Vec::new()),
            map_notifications: NotificationBacklog::default(),
            tasks_queue,
            movement_tasks_queue: Arc::new(Default::default()),
            server_service,
            shutdown: AtomicBool::new(false),
            recording_sessions: Mutex::new(vec![]),
            shared: OnceLock::new(),
            script_world_service,
            login_service: LoginService::new(),
            motd: Motd::default(),
            day_night: Default::default(),
            bots: Default::default(),
            plugin_ui: Default::default(),
            runtime,
        }
    }

    pub async fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        let state = self.lock_state();
        state
            .map_instances()
            .iter()
            .for_each(|(_, instances)| instances.iter().for_each(|instance| instance.shutdown()));
        let characters: Vec<_> = state.characters().values().collect();
        let positions = characters
            .iter()
            .map(|character| {
                crate::server::service::map_position_service::position_update(character, &state.map_flags(&character.map_instance_key))
            })
            .collect();
        self.character_service()
            .save_characters_state_with_positions(characters, positions, get_tick())
            .await;
        self.sessions.for_each(|session| session.disconnect());
    }

    pub fn is_alive(&self) -> bool {
        !self.shutdown.load(Ordering::Relaxed)
    }

    pub fn runtime(&self) -> &Runtime {
        self.runtime.as_ref()
    }

    #[inline]
    pub fn skill_service(&self) -> &SkillService {
        self.server_service.skill_service()
    }

    #[inline]
    pub fn server_service(&self) -> &ServerService {
        &self.server_service
    }

    #[inline]
    pub fn item_service(&self) -> &ItemService {
        self.server_service.item_service()
    }

    #[inline]
    pub fn battle_service(&self) -> &BattleService {
        self.server_service.battle_service()
    }

    #[inline]
    pub fn script_service(&self) -> &ScriptService {
        self.server_service.script_service()
    }

    #[inline]
    pub fn inventory_service(&self) -> &InventoryService {
        self.server_service.inventory_service()
    }

    #[inline]
    pub fn character_service(&self) -> &CharacterService {
        self.server_service.character_service()
    }

    #[inline]
    pub fn skill_tree_service(&self) -> &SkillTreeService {
        self.server_service.skill_tree_service()
    }

    // TODO: This service should be removed and replace by skill implementation
    #[inline]
    pub fn script_skill_service(&self) -> &ScriptSkillService {
        self.server_service.script_skill_service()
    }

    /// Handle for threads that report back to the game loop.
    pub(crate) fn game_tasks(&self) -> Arc<TasksQueue<GameEvent>> {
        self.tasks_queue.clone()
    }

    pub fn add_to_next_tick(&self, event: GameEvent) {
        self.tasks_queue.add_to_first_index(event)
    }

    pub fn add_to_tick(&self, event: GameEvent, index: usize) {
        self.tasks_queue.add_to_index(event, index)
    }

    pub fn add_to_delayed_tick(&self, event: GameEvent, delay: u128) {
        self.add_to_tick(event, delayed_tick(delay, GAME_TICK_RATE));
    }

    pub fn add_to_next_movement_tick(&self, event: GameEvent) {
        self.movement_tasks_queue.add_to_first_index(event)
    }

    pub fn start_recording_session(&self, char_id: u32) {
        let Some(presence) = self.directory.presence(char_id) else {
            return;
        };
        if !self.is_recording_session(char_id) {
            self.recording_sessions()
                .push(Arc::new(SessionRecord::new(char_id, &presence, self.packetver())));
        }
    }

    pub fn stop_recording_session(&self, char_id: u32) {
        let Some(session_id) = self.directory.presence(char_id).map(|presence| presence.account_id) else {
            return;
        };
        if let Some(recording) = self.get_recording_session(session_id) {
            recording.finish();
        }
        self.recording_sessions()
            .retain(|recording_session_id| recording_session_id.session_id != session_id);
    }

    pub fn is_recording_session(&self, char_id: u32) -> bool {
        let Some(session_id) = self.directory.presence(char_id).map(|presence| presence.account_id) else {
            return false;
        };
        self.recording_sessions()
            .iter()
            .any(|recording_session_id| recording_session_id.session_id == session_id)
    }

    pub fn get_recording_session(&self, session_id: u32) -> Option<Arc<SessionRecord>> {
        self.recording_sessions()
            .iter()
            .find(|recording_session_id| recording_session_id.session_id == session_id)
            .cloned()
    }

    fn recording_sessions(&self) -> MutexGuard<'_, Vec<Arc<SessionRecord>>> {
        self.recording_sessions.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub(crate) fn disconnect_character_in_state(&self, state: &mut ServerState, char_id: u32) {
        self.begin_character_logout(state, char_id, false, get_tick());
    }

    pub(crate) fn finish_character_logout(&self, state: &mut ServerState, char_id: u32) {
        state.pending_character_logouts.remove(&char_id);
        state.character_logins.remove(&char_id);
        state.script_timers.disconnect(char_id);
        if let Err(error) = self.cancel_player_trade(state, char_id) {
            warn!("Trade cancellation failed: {error}");
        }
        self.leave_chat_room(state, char_id, false);
        self.notify_friends(state, char_id, false);
        let Some(mut character) = state.characters_mut().remove(&char_id) else {
            return;
        };
        state.retire_character_items(char_id, character.account_id);
        if let Some(session) = state
            .find_session(character.account_id)
            .filter(|session| session.char_id == Some(char_id))
        {
            session.cancel_script();
        }
        self.script_world_service().cancel_pet_capture_in_state(self, state, &mut character);
        if let Err(error) = self
            .script_world_service()
            .return_pet_loot_in_state(self, state, &mut character, get_tick() as u64)
        {
            warn!("Pet loot return deferred on disconnect: {error}");
        }
        if let Err(error) = self.script_world_service().disconnect_party(state, &mut character) {
            warn!("Party disconnect failed: {error}");
        }
        if let Err(error) = self.script_world_service().disconnect(&mut character) {
            warn!("World disconnect failed: {error}");
        }
        self.script_service()
            .npc_variables
            .lock()
            .unwrap()
            .retain(|(scope, _, owner, ..), _| *scope != 2 || *owner != char_id);
        let position =
            crate::server::service::map_position_service::position_update(&character, &state.map_flags(&character.map_instance_key));
        self.runtime.block_on(async {
            self.character_service()
                .save_characters_state_with_positions(vec![&character], vec![position], get_tick())
                .await;
            if let Err(error) = self.repository.save_hotkeys(char_id, &character.hotkeys).await {
                warn!("Hotkey save failed: {error}");
            }
        });
    }

    #[allow(unused_lifetimes)]
    pub fn start<'server>(
        server_ref: Arc<Server>,
        client_notification_sender: SyncSender<Notification>,
        single_client_notification_receiver: Receiver<Notification>,
        persistence_event_receiver: Receiver<PersistenceEvent>,
        persistence_event_sender: SyncSender<PersistenceEvent>,
        enable_client_interfaces: bool,
    ) {
        let port = server_ref.configuration.server.port;
        server_ref.bind_shared();

        let client_notification_sender_clone = client_notification_sender.clone();
        thread::scope(|server_thread_scope: &Scope| {
            if enable_client_interfaces {
                let listener = TcpListener::bind(server_ref.configuration.server.bind_address(port)).unwrap();
                info!("Server listen on {}", server_ref.configuration.server.bind_address(port));
                if server_ref.configuration.server.enable_websocket {
                    info!("WebSocket clients are accepted on the same port");
                }
                let server_shared_ref = server_ref.clone();
                thread::Builder::new()
                    .name("client_connection_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        client_io::serve(server_shared_ref, listener, client_notification_sender_clone);
                        info!("Shutdown client_connection_thread");
                    })
                    .unwrap();
                let server_ref_clone = server_ref.clone();
                thread::Builder::new()
                    .name("client_notification_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        Self::route_notifications(&server_ref_clone, single_client_notification_receiver);
                        info!("Shutdown client_notification_thread");
                    })
                    .unwrap();
            } else {
                info!("Server does not listen client requests");
            }
            if enable_client_interfaces && server_ref.configuration.bots.enabled {
                let server_ref_clone = server_ref.clone();
                thread::Builder::new()
                    .name("bots_api_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        bots::serve(server_ref_clone);
                        info!("Shutdown bots_api_thread");
                    })
                    .unwrap();
            }
            if enable_client_interfaces && server_ref.configuration.client_data.enabled {
                let server_ref_clone = server_ref.clone();
                thread::Builder::new()
                    .name("client_data_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        client_data::serve(server_ref_clone);
                        info!("Shutdown client_data_thread");
                    })
                    .unwrap();
            }
            if enable_client_interfaces && server_ref.configuration.reference.enabled {
                let server_ref_clone = server_ref.clone();
                thread::Builder::new()
                    .name("reference_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        reference::serve(server_ref_clone);
                        info!("Shutdown reference_thread");
                    })
                    .unwrap();
            }
            let server_ref_clone = server_ref.clone();
            thread::Builder::new()
                .name("game_loop_thread".to_string())
                .spawn_scoped(server_thread_scope, move || {
                    Self::game_loop(server_ref_clone);
                    info!("Shutdown game_loop_thread");
                })
                .unwrap();
            let server_ref_clone = server_ref.clone();
            let client_notification_sender_clone = client_notification_sender.clone();
            thread::Builder::new()
                .name("movement_loop_thread".to_string())
                .spawn_scoped(server_thread_scope, move || {
                    Self::character_movement_loop(server_ref_clone, client_notification_sender_clone);
                    info!("Shutdown movement_loop_thread");
                })
                .unwrap();
            let server_ref_clone = server_ref.clone();
            thread::Builder::new()
                .name("persistence_thread".to_string())
                .spawn_scoped(server_thread_scope, move || {
                    Self::persistence_thread(
                        server_ref_clone.clone(),
                        persistence_event_receiver,
                        server_ref_clone.repository.clone(),
                    );
                    info!("Shutdown persistence_thread");
                })
                .unwrap();
            let server_ref_clone = server_ref.clone();
            thread::Builder::new()
                .name("shutdown_thread".to_string())
                .spawn_scoped(server_thread_scope, move || {
                    server_ref_clone.runtime.block_on(async {
                        tokio::signal::ctrl_c().await.unwrap();
                        persistence_event_sender.send(PersistenceEvent::Shutdown).unwrap();
                        server_ref_clone.shutdown().await;
                        info!("Hello ctrl+c");
                        TcpStream::connect(server_ref_clone.configuration.server.connect_address(port))
                            .map(|mut stream| stream.flush())
                            .ok();
                    });
                    info!("Shutdown shutdown_thread");
                })
                .unwrap();
        });
        info!("Shutdown server");
    }

    /// Queues every notification on the connection of its characters. Nothing here waits on a client: a full queue drops
    /// that client, so the game loop and the map loops sending notifications are never held back by one.
    fn route_notifications(server: &Server, receiver: Receiver<Notification>) {
        while server.is_alive() {
            let notification = match receiver.recv_timeout(Duration::from_millis(500)) {
                Ok(notification) => notification,
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            };
            match notification {
                Notification::Char(char_notification) => {
                    let char_id = char_notification.char_id();
                    // Bots have no socket, what is sent to them is read here instead
                    if server.bots().is_bot(char_id) {
                        bots::receive_packet(server, char_id, char_notification.serialized_packet());
                    } else {
                        Self::queue_for_character(server, char_id, char_notification.into_packet());
                    }
                }
                Notification::Area(area_notification) => match area_notification.range_type {
                    AreaNotificationRangeType::Map => {}
                    AreaNotificationRangeType::Fov { x, y, exclude_id } => {
                        server
                            .directory()
                            .in_fov(&area_notification.map_name, area_notification.map_instance_id, x, y, PLAYER_FOV, exclude_id)
                            .into_iter()
                            .for_each(|char_id| {
                                if server.bots().is_bot(char_id) {
                                    bots::receive_packet(server, char_id, area_notification.serialized_packet());
                                } else {
                                    Self::queue_for_character(server, char_id, area_notification.serialized_packet().clone());
                                }
                            });
                    }
                },
            }
        }
    }

    fn queue_for_character(server: &Server, char_id: u32, packet: Vec<u8>) {
        let Some(socket) = server.sessions().find_by_char_id(char_id).and_then(|session| session.map_server_socket.clone()) else {
            return;
        };
        if GlobalConfigService::instance().config().server.trace_packet {
            debug_packets_from_vec(
                Some(&socket.peer_addr()),
                PacketDirection::Backward,
                GlobalConfigService::instance().packetver(),
                &packet,
                &Option::None,
            );
        }
        socket.send(packet);
    }

    pub fn ensure_session_exists(&self, socket: &ClientHandle) -> Option<u32> {
        let session_option = self.sessions.find_by_connection(socket);
        if session_option.is_none() {
            debug!("Session does not exist! for socket {:?}", socket);
        }
        session_option
    }
}
