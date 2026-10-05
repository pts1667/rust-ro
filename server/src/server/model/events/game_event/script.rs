use models::enums::EnumWithMaskValueU32;
use models::enums::script::BroadcastFlag;
use script_sdk::{Variable, VariableScope};

use super::*;
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map::Map;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptWarp {
    pub char_id: u32,
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub destination_instance: Option<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NpcContact {
    pub char_id: u32,
    pub account_id: u32,
    pub npc_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptSpawned {
    pub char_id: u32,
    pub mob_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptEvent {
    pub char_id: u32,
    pub entry_id: u32,
    pub args: Vec<script_sdk::Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptNpcEvent {
    pub npc_id: u32,
    pub scope_instance: u8,
    pub entry_id: u32,
    pub char_id: Option<u32>,
    pub depth: u8,
    pub queued_until: u128,
    pub args: Option<Vec<script_sdk::Value>>,
    pub timer_guard: Option<crate::server::model::script_timer::ScriptTimerGuard>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptMapSpawn {
    pub char_id: u32,
    pub map: String,
    pub request: crate::server::model::events::map_event::ScriptSpawn,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptIdentify {
    pub char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FameChanged {
    pub category: crate::repository::fame_repository::FameCategory,
    pub ranked_creators: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaekwonMissionKill {
    pub char_id: u32,
    pub mob_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemScriptComplete {
    pub action: CharacterUseItem,
    pub source: crate::repository::script_inventory_repository::ScriptItemConsumption,
    pub generation: u64,
    pub effects: Vec<crate::server::script::item_script_handler::ItemEffect>,
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptPartyWarp {
    pub char_id: u32,
    pub party_id: u32,
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub source_map: Option<String>,
    pub range_x: u16,
    pub range_y: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptBroadcast {
    pub char_id: u32,
    pub flags: u32,
    pub packet: Vec<u8>,
}

impl GameEventHandler for crate::server::script::ScriptRequest {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let request = self;
        server.script_service().handle_request(server, state, request);
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::unit_data::ScriptNpcTransfer {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let transfer = self;
        crate::server::script::unit_data::route_transfer(server, state, transfer);
        Ok(())
    }
}

impl GameEventHandler for ScriptNpcEvent {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let event = self;
        server.run_npc_event(state, event, tick)?;
        Ok(())
    }
}

impl GameEventHandler for NpcContact {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let contact = self;
        server.start_npc_conversation(state, contact)?;
        Ok(())
    }
}

impl GameEventHandler for ScriptSpawned {
    fn handle(self, server: &Server, _state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let crate::server::model::events::game_event::ScriptSpawned { char_id, mob_ids } = self;
        let variables: Vec<_> = mob_ids
            .into_iter()
            .enumerate()
            .map(|(index, id)| Variable {
                scope: VariableScope::ServerTemporary,
                name: "mobid".into(),
                index: index as u32,
                value: (id as i32).into(),
            })
            .collect();
        server.script_service().install_temporary_variables(char_id, &variables);
        Ok(())
    }
}

impl GameEventHandler for ScriptEvent {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let event = self;
        server.run_compiled_event(state, event)?;
        Ok(())
    }
}

impl GameEventHandler for ScriptMapSpawn {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let event = self;
        let source = state.characters().get(&event.char_id).ok_or("Monster source disconnected")?;
        let map = GlobalConfigService::instance()
            .find_map(&event.map)
            .ok_or("Monster map is unavailable")?;
        let instance_id = if Map::name_without_ext(source.current_map_name()) == event.map {
            source.current_map_instance()
        } else {
            0
        };
        let instance = state
            .get_map_instance(&event.map, instance_id)
            .unwrap_or_else(|| server.server_service().create_map_instance(state, map, instance_id));
        instance.add_to_next_tick(MapEvent::ScriptSpawn(event.request));
        Ok(())
    }
}

impl GameEventHandler for ScriptWarp {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let warp = self;
        if state.characters().contains_key(&warp.char_id) {
            server.server_service().schedule_warp_to_walkable_cell_in_instance(
                state,
                &warp.map,
                warp.x,
                warp.y,
                warp.char_id,
                warp.destination_instance.unwrap_or(0),
            );
        }
        Ok(())
    }
}

impl GameEventHandler for ScriptPartyWarp {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let event = self;
        for warp in server.plan_party_warp(state, &event)? {
            server.add_to_next_tick(GameEvent::ScriptWarp(warp));
        }
        Ok(())
    }
}

impl GameEventHandler for ScriptBroadcast {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let event = self;
        let source = state.characters().get(&event.char_id).ok_or("Announcement source disconnected")?;
        let scope = event.flags & (BroadcastFlag::Map.as_flag() | BroadcastFlag::Area.as_flag() | BroadcastFlag::ReservedTarget.as_flag());
        let recipients: Vec<_> = state
            .characters()
            .values()
            .filter(|character| match scope {
                0 => true,
                1 => character.map_instance_key == source.map_instance_key,
                2 => {
                    character.map_instance_key == source.map_instance_key
                        && character.x().abs_diff(source.x()) <= crate::server::PLAYER_FOV
                        && character.y().abs_diff(source.y()) <= crate::server::PLAYER_FOV
                }
                3 => character.char_id == source.char_id,
                _ => false,
            })
            .map(|character| character.char_id)
            .collect();
        let sender = server.server_service().notification_sender();
        for char_id in recipients {
            sender
                .send(Notification::Char(CharNotification::new(char_id, event.packet.clone())))
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::service::script_crafting_service::CraftSelection {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let selection = self;
        let mut character = state.characters_mut().remove(&selection.char_id).ok_or("Crafter disconnected")?;
        let result = server.item_service().make_item(server, &mut character, selection, tick);
        state.insert_character(character);
        result?;
        Ok(())
    }
}

impl GameEventHandler for ScriptIdentify {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let selection = self;
        let mut character = state
            .characters_mut()
            .remove(&selection.char_id)
            .ok_or("Identification source disconnected")?;
        let result = server
            .item_service()
            .identify_item_in_state(server, state, &mut character, selection.index, tick);
        state.insert_character(character);
        result?;
        Ok(())
    }
}

impl GameEventHandler for FameChanged {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let update = self;
        for character in state.characters_mut().values_mut() {
            let changed = match update.category {
                crate::repository::fame_repository::FameCategory::Blacksmith => {
                    crate::repository::fame_repository::refresh_forged_rank(&mut character.status, &update.ranked_creators)
                }
                crate::repository::fame_repository::FameCategory::Taekwon => {
                    let changed = match server.update_taekwon_rank(character, &update.ranked_creators) {
                        Ok(changed) => changed,
                        Err(error) => {
                            warn!("Taekwon rank event failed: {error}");
                            continue;
                        }
                    };
                    if changed {
                        server.skill_tree_service().send_skill_tree(character);
                    }
                    changed
                }
                crate::repository::fame_repository::FameCategory::Alchemist => false,
            };
            if changed {
                server.character_service().reload_client_side_status(character);
                character.refresh_script_context();
            }
        }
        Ok(())
    }
}

impl GameEventHandler for TaekwonMissionKill {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let credit = self;
        if let Some(mut character) = state.characters_mut().remove(&credit.char_id) {
            let result = crate::server::service::script_character_service::record_mission_kill(server, &mut character, credit.mob_id);
            state.insert_character(character);
            result?;
        }
        Ok(())
    }
}

impl GameEventHandler for ItemScriptComplete {
    fn required_character(&self) -> Option<u32> {
        Some(self.action.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let completion = self;
        server.item_service().complete_item_dialog(server, state, completion)?;
        Ok(())
    }
}
