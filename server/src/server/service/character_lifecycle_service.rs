use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use tokio::sync::{mpsc, oneshot};

use crate::server::Server;
use crate::server::model::character_lifecycle::{
    ActiveTimerQuit, CharacterAdmission, CharacterLogout, CharacterSelectionGate, ClientDisconnected, PendingCharacterLogout,
    ScriptLogoutCompleted, TimerQuitCallback,
};
use crate::server::model::events::game_event::{GameEvent, CharacterInitInventory, CharacterJoinGame};
use crate::server::model::events::map_event::{MapEvent, RemoveCharFromMap};
use crate::server::model::script_timer::ScriptTimerOwner;
use crate::server::model::session::Session;
use crate::server::script::{NpcScriptHost, ScriptRequest};
use crate::server::service::npc_event_service::event_npc;
use crate::server::service::script_service::ScriptService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl Server {
    async fn receive_lifecycle<T>(&self, mut receiver: oneshot::Receiver<Result<T, String>>) -> Result<T, String> {
        let deadline = tokio::time::Instant::now()
            + std::time::Duration::from_secs(self.configuration.scripting.conversation_timeout_secs.max(1).saturating_mul(3));
        loop {
            tokio::select! {
                result = &mut receiver => return result.map_err(|_| "Character lifecycle stopped".to_string())?,
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                    if !self.is_alive() || tokio::time::Instant::now() >= deadline { return Err("Character lifecycle wait ended".into()); }
                },
            }
        }
    }

    pub(crate) fn await_character_selection(&self, session: Arc<Session>) -> Result<Arc<Session>, String> {
        let (sender, receiver) = oneshot::channel();
        self.add_to_next_tick(GameEvent::CharacterSelectionGate(CharacterSelectionGate {
            session,
            response: Arc::new(Mutex::new(Some(sender))),
        }));
        self.runtime().block_on(self.receive_lifecycle(receiver))
    }

    pub(crate) fn admit_selected_character(&self, session: Arc<Session>, character: Character) -> Result<String, String> {
        let (sender, receiver) = oneshot::channel();
        self.add_to_next_tick(GameEvent::CharacterAdmission(CharacterAdmission {
            session,
            character: Arc::new(Mutex::new(Some(
                crate::server::model::character_lifecycle::SelectedCharacter::from_character(character),
            ))),
            response: Arc::new(Mutex::new(Some(sender))),
        }));
        self.runtime().block_on(self.receive_lifecycle(receiver))
    }

    pub(crate) fn admit_character_map_entry(
        &self,
        session: Arc<Session>,
        socket: Arc<std::sync::RwLock<std::net::TcpStream>>,
    ) -> Result<crate::server::model::character_lifecycle::CharacterMapEntryData, String> {
        let (sender, receiver) = oneshot::channel();
        self.add_to_next_tick(GameEvent::CharacterMapEntry(
            crate::server::model::character_lifecycle::CharacterMapEntry {
                session,
                socket,
                response: Arc::new(Mutex::new(Some(sender))),
            },
        ));
        self.runtime().block_on(self.receive_lifecycle(receiver))
    }

    pub(crate) fn install_character_map_entry(
        &self,
        state: &mut ServerState,
        request: crate::server::model::character_lifecycle::CharacterMapEntry,
    ) {
        let Some(response) = request.response.lock().unwrap().take() else {
            return;
        };
        if response.is_closed() {
            return;
        }
        let reply = (|| {
            let current = state
                .find_session(request.session.account_id)
                .filter(|session| Arc::ptr_eq(session, &request.session))
                .ok_or("Map entry session expired")?;
            let char_id = current.char_id.ok_or("Map entry has no selected character")?;
            if state.pending_character_logouts.contains_key(&char_id) {
                return Err("Character is leaving the game".to_string());
            }
            let character = state
                .get_character(char_id)
                .filter(|character| character.account_id == current.account_id)
                .ok_or("Selected character is unavailable")?;
            let (x, y, direction) = (character.x(), character.y(), character.dir());
            let session = if current.is_simulated {
                current
            } else {
                Arc::new(current.recreate_with_map_socket(request.socket))
            };
            state.add_session(session.account_id, session.clone());
            state.character_logins.insert(char_id, ScriptTimerOwner {
                char_id,
                account_id: session.account_id,
                session: session.clone(),
            });
            Ok(crate::server::model::character_lifecycle::CharacterMapEntryData { session, x, y, direction })
        })();
        let _ = response.send(reply);
    }

    pub(crate) fn prepare_character_map(
        &self,
        state: &mut ServerState,
        request: crate::server::model::character_lifecycle::CharacterMapReady,
    ) {
        let Some(current) = state
            .find_session(request.session.account_id)
            .filter(|session| Arc::ptr_eq(session, &request.session))
        else {
            return;
        };
        let Some(char_id) = current
            .char_id
            .filter(|char_id| !state.pending_character_logouts.contains_key(char_id))
        else {
            return;
        };
        let Some(character) = state.get_character(char_id) else {
            return;
        };
        let (map, x, y) = (
            crate::server::model::map::Map::name_without_ext(character.current_map_name()).to_string(),
            character.x(),
            character.y(),
        );
        self.add_to_next_tick(GameEvent::CharacterJoinGame(CharacterJoinGame { char_id }));
        self.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, char_id);
        self.add_to_next_tick(GameEvent::CharacterInitInventory(CharacterInitInventory { char_id }));
    }

    pub(crate) fn character_selection_gate(&self, state: &mut ServerState, gate: CharacterSelectionGate, tick: u128) {
        if gate.response.lock().unwrap().as_ref().is_none_or(|response| response.is_closed()) {
            return;
        }
        let valid = state
            .find_session(gate.session.account_id)
            .is_some_and(|session| session.auth_code == gate.session.auth_code && session.user_level == gate.session.user_level);
        if valid {
            let already_selected = state.find_session(gate.session.account_id).is_some_and(|session| {
                session.map_server_socket.is_none()
                    && session.char_id.is_some_and(|char_id| {
                        state
                            .get_character(char_id)
                            .is_some_and(|character| !character.loaded_from_client_side)
                            && !state.pending_character_logouts.contains_key(&char_id)
                    })
            });
            if already_selected {
                if let Some(response) = gate.response.lock().unwrap().take() {
                    let _ = response.send(Err("Character is already selected".into()));
                }
                return;
            }
            let previous = state
                .characters()
                .values()
                .filter(|character| character.account_id == gate.session.account_id)
                .map(|character| character.char_id)
                .collect::<Vec<_>>();
            for char_id in previous {
                self.begin_character_logout(state, char_id, true, tick);
            }
            if state
                .pending_character_logouts
                .values()
                .any(|pending| pending.owner.account_id == gate.session.account_id)
            {
                state.character_selection_waiters.push(gate);
                return;
            }
        }
        self.reply_character_selection(state, gate);
    }

    fn reply_character_selection(&self, state: &ServerState, gate: CharacterSelectionGate) {
        let reply = state
            .find_session(gate.session.account_id)
            .filter(|session| {
                session.auth_code == gate.session.auth_code && session.user_level == gate.session.user_level && session.char_id.is_none()
            })
            .ok_or_else(|| "Character selection session expired".to_string());
        if let Some(response) = gate.response.lock().unwrap().take() {
            let _ = response.send(reply);
        }
    }

    pub(crate) fn install_character_admission(&self, state: &mut ServerState, admission: CharacterAdmission) {
        let Some(response) = admission.response.lock().unwrap().take() else {
            return;
        };
        if response.is_closed() {
            return;
        }
        let result = (|| {
            let current = state
                .find_session(admission.session.account_id)
                .filter(|session| Arc::ptr_eq(session, &admission.session))
                .ok_or("Character admission session expired")?;
            if current.char_id.is_some()
                || state
                    .characters()
                    .values()
                    .any(|character| character.account_id == current.account_id)
                || state
                    .pending_character_logouts
                    .values()
                    .any(|pending| pending.owner.account_id == current.account_id)
            {
                return Err("Account already has a character in the game".to_string());
            }
            let character = admission
                .character
                .lock()
                .unwrap()
                .take()
                .ok_or("Character admission was already consumed")?;
            if character.account_id != current.account_id || state.characters().contains_key(&character.char_id) {
                return Err("Character admission owner is invalid".to_string());
            }
            let mut character = character.into_character();
            crate::server::service::map_position_service::restore_login_position(state, &mut character)?;
            let map = character.current_map_name().clone();
            let session = Arc::new(current.recreate_with_character(character.char_id));
            state.character_logins.insert(character.char_id, ScriptTimerOwner {
                char_id: character.char_id,
                account_id: character.account_id,
                session: session.clone(),
            });
            state.add_session(current.account_id, session);
            state.insert_character(character);
            Ok(map)
        })();
        let _ = response.send(result);
    }

    pub(crate) fn bind_character_session(&self, state: &mut ServerState, char_id: u32) -> bool {
        let Some(character) = state.get_character(char_id) else {
            return false;
        };
        let Some(session) = state
            .find_session(character.account_id)
            .filter(|session| session.char_id == Some(char_id))
        else {
            return false;
        };
        if state
            .character_logins
            .get(&char_id)
            .is_some_and(|owner| owner.session.auth_code != session.auth_code)
        {
            return false;
        }
        state.character_logins.insert(char_id, ScriptTimerOwner {
            char_id,
            account_id: character.account_id,
            session,
        });
        true
    }

    pub(crate) fn handle_character_logout(&self, state: &mut ServerState, request: CharacterLogout, tick: u128) {
        let Some(char_id) = request.session.char_id else {
            return;
        };
        let matches = state.character_logins.get(&char_id).map_or_else(
            || {
                state
                    .find_session(request.session.account_id)
                    .is_some_and(|session| session.auth_code == request.session.auth_code)
            },
            |owner| owner.account_id == request.session.account_id && owner.session.auth_code == request.session.auth_code,
        );
        if matches {
            self.begin_character_logout(state, char_id, request.restart, tick);
        }
    }

    pub(crate) fn handle_client_disconnect(&self, state: &mut ServerState, request: ClientDisconnected, tick: u128) {
        let session = state
            .sessions()
            .find_by_map_socket(&request.socket)
            .or_else(|| {
                state
                    .character_logins
                    .values()
                    .find(|owner| {
                        owner
                            .session
                            .map_server_socket
                            .as_ref()
                            .is_some_and(|socket| Arc::ptr_eq(socket, &request.socket))
                            && state.find_session(owner.account_id).is_none_or(|session| {
                                session.auth_code != owner.session.auth_code || session.char_id != Some(owner.char_id)
                            })
                    })
                    .map(|owner| owner.session.clone())
            });
        if let Some(session) = session {
            self.handle_character_logout(state, CharacterLogout { session, restart: false }, tick);
        }
    }

    pub(crate) fn begin_character_logout(&self, state: &mut ServerState, char_id: u32, restart: bool, tick: u128) {
        if state.pending_character_logouts.contains_key(&char_id) {
            return;
        }
        let Some(character) = state.get_character(char_id) else {
            return;
        };
        let account_id = character.account_id;
        let owner = state
            .character_logins
            .get(&char_id)
            .cloned()
            .or_else(|| {
                state
                    .script_timers
                    .npc
                    .values()
                    .filter_map(|timer| timer.owner.as_ref())
                    .find(|owner| owner.char_id == char_id)
                    .cloned()
            })
            .or_else(|| {
                state
                    .find_session(account_id)
                    .filter(|session| session.char_id == Some(char_id))
                    .map(|session| ScriptTimerOwner {
                        char_id,
                        account_id,
                        session,
                    })
            });
        let mut callbacks = VecDeque::new();
        if let Some(owner) = &owner {
            for (key, timer) in &state.script_timers.npc {
                if key.char_id != Some(char_id)
                    || timer.started_at.is_none()
                    || timer.next_label >= timer.labels.len()
                    || timer
                        .owner
                        .as_ref()
                        .is_none_or(|timer_owner| timer_owner.session.auth_code != owner.session.auth_code)
                {
                    continue;
                }
                if let Some(entry_id) = ScriptService::event_entry(&format!("{}::OnTimerQuit", timer.npc_name)) {
                    callbacks.push_back(TimerQuitCallback {
                        key: *key,
                        entry_id,
                        timer_ms: timer.labels[timer.next_label].0,
                        label_count: timer.labels.len(),
                    });
                }
            }
            owner.session.cancel_script();
            if let Some(session) = state
                .find_session(account_id)
                .filter(|session| session.auth_code == owner.session.auth_code && session.char_id == Some(char_id))
            {
                session.cancel_script();
                if restart {
                    state.add_session(account_id, Arc::new(session.recreate_without_character()));
                } else {
                    state.remove_session(account_id);
                }
            }
        }
        state.script_timers.disconnect(char_id);
        self.battleground_leave(state, char_id, true, true);
        self.battleground_queue_leave(state, char_id, false);
        if let Err(error) = self.cancel_player_trade(state, char_id) {
            warn!("Trade cancellation failed during logout: {error}");
        }
        if let Some(mut character) = state.characters_mut().remove(&char_id) {
            if let Some(map) = state.get_map_instance_from_character(&character) {
                map.add_to_next_tick(MapEvent::RemoveCharFromMap(RemoveCharFromMap { char_id }));
            }
            character.loaded_from_client_side = false;
            character.clear_attack();
            character.movements.clear();
            character.pending_item_skill = None;
            self.script_skill_service().cancel_queued_cast(&mut character);
            self.script_world_service().cancel_pet_capture_in_state(self, state, &mut character);
            state.insert_character(character);
        }
        if !callbacks.is_empty() {
            state.pending_character_logouts.insert(char_id, PendingCharacterLogout {
                owner: owner.unwrap(),
                callbacks,
                current: None,
                deadline: tick.saturating_add(u128::from(self.configuration.scripting.conversation_timeout_secs.max(1)) * 2000),
            });
            state.retire_character_items(char_id, account_id);
            self.start_timer_quit_callback(state, char_id, tick);
        } else {
            self.finish_character_logout(state, char_id);
        }
    }

    fn start_timer_quit_callback(&self, state: &mut ServerState, char_id: u32, tick: u128) {
        let callback = state
            .pending_character_logouts
            .get(&char_id)
            .and_then(|pending| pending.callbacks.front());
        let Some(callback) = callback else {
            self.finish_character_logout(state, char_id);
            return;
        };
        let Some((actor, script)) = event_npc(state, callback.key.npc_id, callback.key.scope_instance) else {
            return;
        };
        let Some(server) = self.shared() else {
            self.finish_character_logout(state, char_id);
            return;
        };
        let token = match state.script_timers.next_id() {
            Ok(token) => token,
            Err(error) => {
                error!("Timer quit callback could not start: {error}");
                self.finish_character_logout(state, char_id);
                return;
            }
        };
        let pending = state.pending_character_logouts.get_mut(&char_id).unwrap();
        let callback = pending.callbacks.pop_front().unwrap();
        let session = pending.owner.session.clone();
        let generation = session.script_generation.load(std::sync::atomic::Ordering::Acquire);
        let (_, inputs) = mpsc::channel(1);
        pending.current = Some(ActiveTimerQuit {
            token,
            key: callback.key,
            npc_entry: script.entry_id,
            elapsed_ms: callback.timer_ms,
            started_at: Some(tick),
            label_count: callback.label_count,
        });
        let host = NpcScriptHost {
            server: server.clone(),
            session,
            script,
            inputs,
            notifications: self.server_service().notification_sender(),
            generation,
            map_instance: actor.instance,
            background: true,
            event_depth: 0,
            event_arguments: None,
            timer_context: Some(callback.key),
            logout_token: Some(token),
            error: None,
        };
        let vm = self.script_service().vm.clone();
        let timeout = std::time::Duration::from_secs(self.configuration.scripting.conversation_timeout_secs.max(1));
        self.runtime().spawn(async move {
            match tokio::time::timeout(timeout, vm.execute(host, "run_event", callback.entry_id)).await {
                Ok((host, Err(error))) => warn!(
                    "NPC {} timer quit callback failed: {}",
                    callback.key.npc_id,
                    host.error.unwrap_or(error)
                ),
                Err(_) => warn!("NPC {} timer quit callback timed out", callback.key.npc_id),
                _ => {}
            }
            server.add_to_delayed_tick(
                GameEvent::ScriptLogoutCompleted(ScriptLogoutCompleted { char_id, token }),
                crate::server::game_loop::GAME_TICK_RATE * 2,
            );
        });
    }

    pub(crate) fn complete_timer_quit_callback(&self, state: &mut ServerState, completion: ScriptLogoutCompleted, tick: u128) {
        let Some(pending) = state
            .pending_character_logouts
            .get_mut(&completion.char_id)
            .filter(|pending| pending.current.as_ref().is_some_and(|current| current.token == completion.token))
        else {
            return;
        };
        pending.current = None;
        self.start_timer_quit_callback(state, completion.char_id, tick);
    }

    pub(crate) fn valid_logout_request(&self, state: &ServerState, context: &ScriptRequest) -> bool {
        state.pending_character_logouts.get(&context.char_id).is_some_and(|pending| {
            pending.owner.account_id == context.account_id
                && pending.current.as_ref().is_some_and(|current| {
                    context.logout_token == Some(current.token)
                        && context.npc_id == current.key.npc_id
                        && context.npc_scope_instance == current.key.scope_instance
                        && context.npc_entry == current.npc_entry
                })
                && state.characters().contains_key(&context.char_id)
        })
    }

    pub(crate) fn tick_character_logouts(&self, state: &mut ServerState, tick: u128) {
        let disconnected = state
            .character_logins
            .iter()
            .filter(|(char_id, owner)| {
                state.characters().contains_key(char_id)
                    && !state.pending_character_logouts.contains_key(char_id)
                    && state
                        .find_session(owner.account_id)
                        .is_none_or(|session| session.auth_code != owner.session.auth_code || session.char_id != Some(**char_id))
            })
            .map(|(char_id, _)| *char_id)
            .collect::<Vec<_>>();
        for char_id in disconnected {
            self.begin_character_logout(state, char_id, false, tick);
        }
        let pending = state.pending_character_logouts.keys().copied().collect::<Vec<_>>();
        for char_id in pending {
            if state
                .pending_character_logouts
                .get(&char_id)
                .is_some_and(|pending| tick >= pending.deadline)
            {
                warn!("Character {} logout cleanup timed out", char_id);
                self.finish_character_logout(state, char_id);
            } else if state
                .pending_character_logouts
                .get(&char_id)
                .is_some_and(|pending| pending.current.is_none())
            {
                self.start_timer_quit_callback(state, char_id, tick);
            }
        }
        let waiters = std::mem::take(&mut state.character_selection_waiters);
        for gate in waiters {
            if state
                .pending_character_logouts
                .values()
                .any(|pending| pending.owner.account_id == gate.session.account_id)
            {
                state.character_selection_waiters.push(gate);
            } else {
                self.reply_character_selection(state, gate);
            }
        }
    }
}
