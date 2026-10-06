use std::collections::HashSet;
use std::cell::RefCell;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, RwLock, Weak, mpsc};
use std::thread;
use std::thread::Scope;
use std::time::Duration;

use configuration::configuration::Config;
use model::events::client_notification::{AreaNotificationRangeType, Notification};
use model::events::game_event::GameEvent;
use model::events::persistence_event::PersistenceEvent;
use packets::packets_parser::parse;
use script::skill::ScriptSkillService;
use script_runtime::WasmRuntime;
use tokio::runtime::Runtime;

use crate::repository::Repository;
use crate::server::game_loop::GAME_TICK_RATE;
use crate::server::model::map_item::MapItems;
use crate::server::model::request::Request;
use crate::server::model::response::Response;
use crate::server::model::session::{SessionRecord, SessionRegistry};
use crate::server::model::battleground::Battlegrounds;
use crate::server::model::character_lifecycle::CharacterSelectionGate;
use crate::server::model::duel::Duels;
use crate::server::model::map_flag_overrides::{MapFlagOverrides, SiegeFlag};
use crate::server::model::notification_backlog::NotificationBacklog;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;
use crate::server::service::script_service::ScriptService;
use crate::server::model::client_socket::{ClientConnection, ClientSocket};
use crate::server::service::script_world_service::ScriptWorldService;
use crate::server::service::server_service::ServerService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character_directory::CharacterDirectory;
use crate::server::state::server::ServerState;
use crate::util::packet::{PacketDirection, PacketsBuffer, debug_packets_from_vec, print_packet};
use crate::util::tick::{delayed_tick, get_tick};

pub mod boot;
mod game_loop;
pub mod map_instance_loop;
pub mod model;
pub mod persistence;
pub mod request_handler;
pub mod script;
pub mod service;
pub mod state;
pub mod websocket;

pub(crate) type StateGuard<'a> = parking_lot::MutexGuard<'a, ServerState>;

const STATE_LOCK_TIMEOUT: Duration = Duration::from_secs(10);

thread_local!(pub static PACKETVER: RefCell<u32> = const { RefCell::new(0) });
// Todo make this configurable
pub const PLAYER_FOV: u16 = 20;
pub const MOB_FOV: u16 = 14;

pub struct Server {
    pub configuration: &'static Config,
    pub repository: Arc<dyn Repository>,
    state: parking_lot::Mutex<ServerState>,
    sessions: SessionRegistry,
    directory: CharacterDirectory,
    duels: Duels,
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

    pub fn duels(&self) -> &Duels {
        &self.duels
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
        npc_script_vm: Arc<WasmRuntime>,
        item_script_vm: Arc<WasmRuntime>,
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
        let state = ServerState::new(map_items);
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

        let (response_sender, single_response_receiver) = std::sync::mpsc::sync_channel::<Response>(0);
        let client_notification_sender_clone = client_notification_sender.clone();
        thread::scope(|server_thread_scope: &Scope| {
            let listener = TcpListener::bind(server_ref.configuration.server.bind_address(port)).unwrap();
            // listener.set_nonblocking(true);
            let server_shared_ref = server_ref.clone();
            if enable_client_interfaces {
                info!("Server listen on {}", server_ref.configuration.server.bind_address(port));
                let websocket_enabled = server_ref.configuration.server.enable_websocket;
                if websocket_enabled {
                    info!("WebSocket clients are accepted on the same port");
                }
                thread::Builder::new()
                    .name("client_connection_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        for tcp_stream in listener.incoming() {
                            if !server_shared_ref.is_alive() {
                                break;
                            }
                            // Receive new connection, starting new thread
                            let server_shared_ref = server_shared_ref.clone();
                            debug!("Received new connection");
                            let response_sender_clone = response_sender.clone();
                            let client_notification_sender_clone = client_notification_sender_clone.clone();
                            let tcp_stream = tcp_stream.unwrap();
                            thread::Builder::new()
                                .name(format!("client_{}_thread", tcp_stream.peer_addr().unwrap()))
                                .spawn_scoped(server_thread_scope, move || {
                                    PACKETVER.with(|ver| *ver.borrow_mut() = server_shared_ref.packetver());

                                    let connection = match ClientConnection::accept(tcp_stream, websocket_enabled) {
                                        Ok(connection) => connection,
                                        Err(error) => {
                                            warn!("Client connection rejected: {}", error);
                                            return;
                                        }
                                    };
                                    let mut reader = connection.reader;
                                    let tcp_stream_arc = connection.socket;
                                    let mut buffer = [0; 2048];
                                    let mut frames = request_handler::framing::ClientFrames::new(server_shared_ref.packetver());
                                    'connection: loop {
                                        if !server_shared_ref.is_alive() {
                                            let _ = tcp_stream_arc.read().unwrap().shutdown(Shutdown::Both);
                                            break;
                                        }
                                        match reader.read(&mut buffer) {
                                            Ok(bytes_read) => {
                                                if bytes_read == 0 {
                                                    info!("shutdown thread client");
                                                    tcp_stream_arc.read().unwrap().shutdown(Shutdown::Both).expect(
                                                        "Unable to shutdown incoming socket. Shutdown was done because remote socket \
                                                         seems closed.",
                                                    );
                                                    break;
                                                }
                                                let incoming = match frames.push(&buffer[..bytes_read]) {
                                                    Ok(incoming) => incoming,
                                                    Err(error) => {
                                                        warn!("Invalid client packet stream: {}", error);
                                                        let _ = tcp_stream_arc.read().unwrap().shutdown(Shutdown::Both);
                                                        break 'connection;
                                                    }
                                                };
                                                for frame in incoming {
                                                    let id = u16::from_le_bytes([frame[0], frame[1]]);
                                                    let packet: Box<dyn packets::packets::Packet> =
                                                        if service::script_world_service::world_frame_length(
                                                            id,
                                                            server_shared_ref.packetver(),
                                                        )
                                                        .is_some()
                                                            || request_handler::script_operations::frame_length(
                                                                id,
                                                                server_shared_ref.packetver(),
                                                            )
                                                            .is_some()
                                                        {
                                                            Box::new(packets::packets::PacketUnknown {
                                                                raw: frame,
                                                                packet_id: format!("0x{id:04x}"),
                                                            })
                                                        } else {
                                                            parse(&frame, server_shared_ref.packetver())
                                                        };
                                                    if GlobalConfigService::instance().config().server.trace_packet {
                                                        print_packet(&None, None, PacketDirection::Forward, &packet);
                                                    }
                                                    let context = Request::new(
                                                        server_shared_ref.configuration,
                                                        None,
                                                        server_shared_ref.packetver(),
                                                        tcp_stream_arc.clone(),
                                                        packet.as_ref(),
                                                        response_sender_clone.clone(),
                                                        client_notification_sender_clone.clone(),
                                                    );
                                                    request_handler::handle(server_shared_ref.clone(), context);
                                                }
                                            }
                                            Err(err) => {
                                                error!("{}", err);
                                                let _ = tcp_stream_arc.read().unwrap().shutdown(Shutdown::Both);
                                                break;
                                            }
                                        }
                                    }
                                    server_shared_ref.add_to_next_tick(GameEvent::ClientDisconnected(
                                        crate::server::model::character_lifecycle::ClientDisconnected { socket: tcp_stream_arc },
                                    ));
                                })
                                .unwrap();
                        }
                        info!("Shutdown client_connection_thread");
                    })
                    .unwrap();
                // Start a thread sending response packet to client request

                let _server_ref_clone = server_ref.clone();
                thread::Builder::new()
                    .name("client_response_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        while let Ok(response) = single_response_receiver.recv() {
                            let tcp_stream = &response.socket();
                            let data = response.serialized_packet();
                            let mut tcp_stream_guard = tcp_stream.write().unwrap();
                            debug!("Respond to {:?} with: {:02X?}", tcp_stream_guard.peer_addr(), data);
                            if GlobalConfigService::instance().config().server.trace_packet {
                                debug_packets_from_vec(
                                    Some(tcp_stream_guard.peer_addr().as_ref().unwrap()),
                                    PacketDirection::Backward,
                                    GlobalConfigService::instance().packetver(),
                                    data,
                                    &Option::None,
                                );
                            }
                            tcp_stream_guard.write_all(data).unwrap();
                            tcp_stream_guard.flush().unwrap();
                        }
                        info!("Shutdown client_response_thread");
                    })
                    .unwrap();
                // Start a thread sending packet to notify client from game update
                let server_ref_clone = server_ref.clone();
                thread::Builder::new()
                    .name("client_notification_thread".to_string())
                    .spawn_scoped(server_thread_scope, move || {
                        let server_ref = server_ref_clone;
                        let mut packets_by_session: Vec<PacketsBuffer> = Vec::new();
                        loop {
                            if !server_ref.is_alive() {
                                thread::sleep(Duration::from_millis(500));
                            }
                            packets_by_session.retain(|buffer| {
                                if buffer.should_flush() {
                                    if let Some(tcp_stream) = server_ref
                                        .sessions()
                                        .find_by_char_id(buffer.session_id())
                                        .and_then(|session| session.map_server_socket.clone())
                                    {
                                        let mut tcp_stream_guard = tcp_stream.write().unwrap();
                                        if tcp_stream_guard.peer_addr().is_ok() {
                                            debug!(
                                                "Respond to {:?} with {} bytes with: {:02X?}",
                                                tcp_stream_guard.peer_addr(),
                                                buffer.data().len(),
                                                buffer.data()
                                            );
                                            if buffer.data().is_empty() {
                                                debug!("{} - {:?}", buffer.session_id(), buffer.data());
                                            }
                                            if GlobalConfigService::instance().config().server.trace_packet {
                                                debug_packets_from_vec(
                                                    Some(tcp_stream_guard.peer_addr().as_ref().unwrap()),
                                                    PacketDirection::Backward,
                                                    GlobalConfigService::instance().packetver(),
                                                    buffer.data(),
                                                    &Option::None,
                                                );
                                                info!(
                                                    "Flushing {} {}bytes - {:02X?}",
                                                    buffer.session_id(),
                                                    buffer.data().len(),
                                                    buffer.data()
                                                );
                                            }
                                            if tcp_stream_guard.write_all(buffer.data()).is_ok() {
                                                tcp_stream_guard.flush().unwrap();
                                            }
                                        } else {
                                            error!("{:?} socket has been closed", tcp_stream_guard.peer_addr().err());
                                        }
                                    }
                                    return false;
                                }
                                true
                            });
                            match single_client_notification_receiver.recv_timeout(Duration::from_millis(16)) {
                                Ok(response) => match response {
                                    Notification::Char(char_notification) => {
                                        Self::buffer_packets(
                                            &mut packets_by_session,
                                            char_notification.char_id(),
                                            char_notification.serialized_packet().as_slice(),
                                        );
                                    }
                                    Notification::Area(area_notification) => match area_notification.range_type {
                                        AreaNotificationRangeType::Map => {}
                                        AreaNotificationRangeType::Fov { x, y, exclude_id } => {
                                            server_ref
                                                .directory()
                                                .in_fov(&area_notification.map_name, area_notification.map_instance_id, x, y, PLAYER_FOV, exclude_id)
                                                .into_iter()
                                                .for_each(|char_id| {
                                                    if GlobalConfigService::instance().config().server.trace_packet {
                                                        debug_packets_from_vec(
                                                            None,
                                                            PacketDirection::Backward,
                                                            GlobalConfigService::instance().packetver(),
                                                            area_notification.serialized_packet(),
                                                            &Some(String::from("Enqueu in buffer")),
                                                        );
                                                    }
                                                    Self::buffer_packets(
                                                        &mut packets_by_session,
                                                        char_id,
                                                        area_notification.serialized_packet().as_slice(),
                                                    );
                                                });
                                        }
                                    },
                                },
                                Err(mpsc::RecvTimeoutError::Timeout) => {}
                                _ => {}
                            }
                            if !server_ref.is_alive() {
                                break;
                            }
                        }
                        info!("Shutdown client_notification_thread");
                    })
                    .unwrap();
            } else {
                info!("Server does not listen client requests");
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

    fn buffer_packets(packets_by_session: &mut Vec<PacketsBuffer>, char_id: u32, data: &[u8]) {
        let maybe_buffer = packets_by_session.iter_mut().find(|buffer| buffer.session_id() == char_id);
        if maybe_buffer.is_none() {
            let mut buffer = PacketsBuffer::new(char_id, 2048);
            buffer.push(data);
            packets_by_session.push(buffer);
        } else {
            let buffer = maybe_buffer.unwrap();
            if buffer.can_contain(data.len()) {
                buffer.push(data);
            } else {
                let mut buffer = PacketsBuffer::new(char_id, 2048);
                buffer.push(data);
                packets_by_session.push(buffer);
            }
        }
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<RwLock<ClientSocket>>) -> Option<u32> {
        let stream_guard = read_lock!(tcp_stream);
        let session_option = self.sessions.find_by_stream(&stream_guard);
        if session_option.is_none() {
            debug!("Session does not exist! for socket {:?}", stream_guard);
        }
        session_option
    }
}
