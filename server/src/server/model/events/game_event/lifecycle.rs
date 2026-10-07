use movement::position::Position;

use super::*;
use crate::server::Server;
use crate::server::model::events::map_event::{MapEvent, InsertCharToMap, RemoveCharFromMap};
use crate::server::model::map::Map;
use crate::server::model::map_item::ToMapItem;
use crate::server::model::movement::Movement;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRemoveFromMap {
    pub char_id: u32,
    pub map_name: String,
    pub instance_id: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterMovement {
    pub char_id: u32,
    pub start_at: u128,
    pub destination: Position,
    pub current_position: Position,
    pub path: Vec<Movement>,
    pub cancel_attack: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterLeaveGame {
    pub char_id: u32,
    pub atype: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterJoinGame {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterLoadedFromClientSide {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterClearFov {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSavePosition {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterCancelMove {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CastleLifecycle {
    Init,
    AgitStart,
    AgitEnd,
    EmperiumBroken { map: String, guild_id: u32 },
    GuildBroken { guild_id: u32 },
    Refresh { map: String, abandoned: bool },
    AnnounceConquest { map: String, guild_id: u32 },
    RestartArena { map: String },
    SummonGuardian { map: String, slot: u8 },
    DailyTick,
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterSelectionGate {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let gate = self;
        server.character_selection_gate(state, gate, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterAdmission {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let admission = self;
        server.install_character_admission(state, admission);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterMapEntry {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let entry = self;
        server.install_character_map_entry(state, entry);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterMapReady {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let ready = self;
        server.prepare_character_map(state, ready);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterLogout {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        server.handle_character_logout(state, request, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::ClientDisconnected {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        server.handle_client_disconnect(state, request, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::ScriptLogoutAction {
    fn handle(self, _server: &Server, _state: &mut ServerState, _tick: u128) -> Result<(), String> {
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::ScriptLogoutCompleted {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let completion = self;
        server.complete_timer_quit_callback(state, completion, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterMemo {
    fn required_character(&self) -> Option<u32> {
        self.session.char_id
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let request = self;
        if let Err(error) = server.memo_location(state, request) {
            warn!("Memo failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::character_lifecycle::CharacterRespawn {
    fn required_character(&self) -> Option<u32> {
        self.session.char_id
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let request = self;
        if let Err(error) = server.respawn_character(state, request) {
            warn!("Respawn failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterLeaveGame {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterLeaveGame { char_id, atype: _ } = self;
        server.disconnect_character_in_state(state, char_id);
        Ok(())
    }
}

impl GameEventHandler for CharacterJoinGame {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterJoinGame { char_id } = self;
        if !server.bind_character_session(state, char_id) {
            return Ok(());
        }
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.skill_tree_service().send_skill_tree(character);
        server
            .character_service()
            .load_temporary_bonuses_from_db(server.runtime.as_ref(), character);
        Ok(())
    }
}

impl GameEventHandler for CharacterLoadedFromClientSide {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterLoadedFromClientSide { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        character.loaded_from_client_side = true;
        character.clear_map_view();
        server.send_character_config(character);
        server.notify_map_property(state, char_id);
        server.notify_weather(state, char_id);
        server.trigger_map_load_events(state, char_id);
        server.enter_pvp_ranking(state, char_id);
        server.friend_login(state, char_id);
        server.mail_login(char_id);
        server.channel_login(state, char_id);
        server.quest_login(state, char_id);
        server.instance_login(state, char_id);
        Ok(())
    }
}

impl GameEventHandler for CharacterClearFov {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, _server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterClearFov { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        character.clear_map_view();
        Ok(())
    }
}

impl GameEventHandler for CharacterChangeMap {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let event = self;
        server.leave_chat_room(state, event.char_id, false);
        if let Err(error) = server.cancel_player_trade(state, event.char_id) {
            warn!("Trade cancellation failed: {error}");
        }
        let memorial_gone = event.new_instance_id != 0
            && crate::server::model::instance::is_memorial_map(&event.new_map_name)
            && state.get_map_instance(&event.new_map_name, event.new_instance_id).is_none();
        if memorial_gone {
            let character = state.characters().get(&event.char_id).ok_or("Character disconnected")?;
            let (map, x, y) = (Map::name_without_ext(&character.save_map), character.save_x, character.save_y);
            server.server_service.schedule_warp_to_walkable_cell(state, &map, x, y, event.char_id);
            return Ok(());
        }
        let map_instance = state
            .get_map_instance(&event.new_map_name, event.new_instance_id)
            .unwrap_or_else(|| {
                server.server_service.create_map_instance(
                    state,
                    GlobalConfigService::instance().get_map(&event.new_map_name),
                    event.new_instance_id,
                )
            });
        let flags = state.map_flags(map_instance.key());
        let character = state.characters_mut().get_mut(&event.char_id).unwrap();
        if let Err(error) =
            server
                .script_world_service()
                .prepare_store_map_move(character, map_instance.key(), event.new_position.unwrap(), &flags)
        {
            warn!("Map movement store update failed: {error}");
            let origin = character.map_instance_key.clone();
            let map_item = character.to_map_item();
            if let Some(instance) = state.get_map_instance(origin.map_name(), origin.map_instance()) {
                instance.add_to_next_tick(MapEvent::InsertCharToMap(InsertCharToMap { map_item }));
            }
            return Ok(());
        }
        server
            .character_service()
            .change_map_with_flags(map_instance.key(), event.new_position.unwrap(), character, &flags);
        let char_map_item = character.to_map_item();
        map_instance.add_to_next_tick(MapEvent::InsertCharToMap(InsertCharToMap { map_item: char_map_item }));
        server.add_to_next_tick(GameEvent::CharacterInitInventory(CharacterInitInventory {
            char_id: character.char_id,
        }));
        let account_id = character.account_id;
        state.insert_map_item(account_id, char_map_item);
        Ok(())
    }
}

impl GameEventHandler for CharacterRemoveFromMap {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let character_remove_from_map = self;
        if let Err(error) = server.cancel_player_trade(state, character_remove_from_map.char_id) {
            warn!("Trade cancellation failed: {error}");
        }
        let character = state.characters_mut().get_mut(&character_remove_from_map.char_id).unwrap();
        character.movements = vec![];
        if let Some(instance) = state.get_map_instance(&character_remove_from_map.map_name, character_remove_from_map.instance_id) {
            instance.add_to_next_tick(MapEvent::RemoveCharFromMap(RemoveCharFromMap { char_id: character_remove_from_map.char_id }));
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterSavePosition {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterSavePosition { char_id } = self;
        if let Some(character) = state.get_character(char_id) {
            let flags = state.map_flags(&character.map_instance_key);
            server.character_service().defer_position_update_with_flags(character, &flags);
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterMovement {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, _server: &Server, _state: &mut ServerState, _tick: u128) -> Result<(), String> {
        // handled by dedicated thread
        Ok(())
    }
}

impl GameEventHandler for CharacterCancelMove {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let CharacterCancelMove { char_id } = self;
        let character = state.characters_mut().get_mut(&char_id).unwrap();
        server.character_service().cancel_movement(character, tick);
        Ok(())
    }
}

impl GameEventHandler for CastleLifecycle {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let event = self;
        server.handle_castle_lifecycle(state, event, tick);
        Ok(())
    }
}
