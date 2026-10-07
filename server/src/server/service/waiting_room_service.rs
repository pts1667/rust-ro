//! `waitingroom` and its companions: a chat room owned by an NPC where players gather until a script event fires.

use script_sdk::{Function, Reply, Value, Variable, VariableScope};

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::CharacterZeny;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::script::Script;
use crate::server::model::waiting_room::{MAX_LEVEL, WaitingRoom};
use crate::server::script::ScriptRequest;
use crate::server::script::skill::actor::ScriptSkillActor;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::npc_event_service::named_npc;
use crate::server::service::social_packets as wire;
use crate::server::state::server::ServerState;
use crate::server::Server;

/// The room title shown above the NPC, as the client counts users: the NPC is one of them.
pub(crate) fn waiting_room_entry_packet(state: &ServerState, npc_id: u32) -> Option<Vec<u8>> {
    state.waiting_rooms.of_npc(npc_id).map(wire::waiting_room_entry)
}

impl Server {
    fn waiting_room_area(&self, room: &WaitingRoom, packet: Vec<u8>) {
        let range = AreaNotificationRangeType::Fov { x: room.x, y: room.y, exclude_id: None };
        let _ = self.server_service().notification_sender().try_send(Notification::Area(AreaNotification::new(room.map.clone(), room.instance, range, packet)));
    }

    fn waiting_room_names(state: &ServerState, room: &WaitingRoom) -> Vec<String> {
        room.members.iter().map(|member| state.get_character(*member).map(|character| character.name.clone()).unwrap_or_default()).collect()
    }

    /// Handles `CZ_REQ_ENTER_ROOM` for a waiting room; false when `chat_id` is not one.
    pub(crate) fn join_waiting_room(&self, state: &mut ServerState, char_id: u32, chat_id: u32) -> bool {
        let Some(room) = state.waiting_rooms.of_chat(chat_id) else { return false };
        let npc_id = room.npc_id;
        let Some(character) = state.get_character(char_id) else { return true };
        let busy = character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some()
            || state.chat_rooms.room_of(char_id).is_some() || state.waiting_rooms.room_of(char_id).is_some();
        let same_map = normalize_map(character.current_map_name()) == room.map && character.current_map_instance() == room.instance;
        let verdict = if busy || !same_map {
            Err(crate::server::model::waiting_room::Refusal::Full)
        } else {
            state.waiting_rooms.check_join(npc_id, u32::from(character.status.base_level), character.status.zeny)
        };
        if let Err(refusal) = verdict {
            self.send_raw(char_id, wire::room_refused(refusal.code()));
            return true;
        }
        let name = character.name.clone();
        let Some(room) = state.waiting_rooms.join(npc_id, char_id).cloned() else { return true };
        self.send_raw(char_id, wire::waiting_room_entered(room.id, &Self::waiting_room_names(state, &room)));
        for member in room.members.iter().filter(|member| **member != char_id) {
            self.send_raw(*member, wire::room_member_joined(room.members.len(), &name));
        }
        self.waiting_room_area(&room, wire::waiting_room_entry(&room));
        self.fire_waiting_room_event(state, &room);
        true
    }

    fn fire_waiting_room_event(&self, state: &ServerState, room: &WaitingRoom) {
        if room.trigger_reached() {
            self.trigger_npc_event(state, &room.event);
        }
    }

    /// Takes the player out of the waiting room they are in; false when they are in none.
    pub(crate) fn depart_waiting_room(&self, state: &mut ServerState, char_id: u32, kicked: bool) -> bool {
        let name = state.get_character(char_id).map(|character| character.name.clone()).unwrap_or_default();
        let Some(room) = state.waiting_rooms.leave(char_id).cloned() else { return false };
        let left = wire::room_member_left(room.members.len(), &name, kicked);
        self.send_raw(char_id, left.clone());
        for member in &room.members {
            self.send_raw(*member, left.clone());
        }
        self.waiting_room_area(&room, wire::waiting_room_entry(&room));
        true
    }

    fn kick_all_from_waiting_room(&self, state: &mut ServerState, npc_id: u32) {
        let members = state.waiting_rooms.of_npc(npc_id).map(|room| room.members.clone()).unwrap_or_default();
        for member in members {
            self.depart_waiting_room(state, member, true);
        }
    }

    fn waiting_room_target(&self, state: &ServerState, context: &ScriptRequest, name: Option<&Value>) -> Result<(ScriptSkillActor, Option<std::sync::Arc<Script>>), String> {
        match name.map(Value::text).filter(|name| !name.is_empty()) {
            Some(name) => named_npc(state, context, &name)?.map(|(actor, script)| (actor, Some(script))).ok_or_else(|| "Waiting room NPC is not found".to_string()),
            None => Ok((crate::server::script::unit_data::script_actor(state, context)?.ok_or("This waiting room command needs an NPC")?, None)),
        }
    }

    pub(crate) fn script_waiting_room_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let optional_number = |index: usize, default: i32| -> Result<i32, String> { arguments.get(index).map(Value::number_value).transpose().map(|value| value.unwrap_or(default)) };
        match function {
            Function::WaitingRoom => {
                let (actor, script) = self.waiting_room_target(state, context, None)?;
                let script_name = state.get_map_instance(&actor.map, actor.instance).and_then(|instance| instance.get_script(actor.id)).map(|script| script.name.clone()).or(script.map(|script| script.name.clone())).unwrap_or_default();
                let event = arguments.get(2).map(Value::text).unwrap_or_default();
                let event = if event.is_empty() || event.contains("::") { event } else { format!("{script_name}::{event}") };
                let limit = u16::try_from(number(1)?).map_err(|_| "Invalid waiting room limit")?;
                let room = WaitingRoom {
                    id: 0, npc_id: actor.id, map: normalize_map(&actor.map), instance: actor.instance, x: actor.x, y: actor.y,
                    title: arguments.first().ok_or("Missing waiting room title")?.text(), limit,
                    trigger: u16::try_from(optional_number(3, i32::from(limit))?.clamp(0, 0x7f)).unwrap_or(0), event, event_disabled: false,
                    zeny: u32::try_from(optional_number(4, 0)?).map_err(|_| "Invalid waiting room fee")?,
                    min_level: u32::try_from(optional_number(5, 1)?).map_err(|_| "Invalid waiting room level")?,
                    max_level: u32::try_from(optional_number(6, MAX_LEVEL as i32)?).map_err(|_| "Invalid waiting room level")?,
                    members: vec![],
                };
                let created = state.waiting_rooms.create(room).cloned().ok_or("This NPC already has a waiting room")?;
                self.waiting_room_area(&created, wire::waiting_room_entry(&created));
                Ok(Value::default())
            }
            Function::DelWaitingRoom => {
                let (actor, _) = self.waiting_room_target(state, context, arguments.first())?;
                self.kick_all_from_waiting_room(state, actor.id);
                if let Some(room) = state.waiting_rooms.delete(actor.id) {
                    self.waiting_room_area(&room, wire::room_destroyed(room.id));
                }
                Ok(Value::default())
            }
            Function::WaitingRoomKickAll => {
                let (actor, _) = self.waiting_room_target(state, context, arguments.first())?;
                self.kick_all_from_waiting_room(state, actor.id);
                Ok(Value::default())
            }
            Function::WaitingRoomKick => {
                let (actor, _) = self.waiting_room_target(state, context, arguments.first())?;
                let name = arguments.get(1).ok_or("Missing character name")?.text();
                let member = state.waiting_rooms.of_npc(actor.id).and_then(|room| room.members.iter().copied().find(|member| state.get_character(*member).is_some_and(|character| character.name == name)));
                if let Some(member) = member {
                    self.depart_waiting_room(state, member, true);
                }
                Ok(Value::default())
            }
            Function::EnableWaitingRoomEvent | Function::DisableWaitingRoomEvent => {
                let (actor, _) = self.waiting_room_target(state, context, arguments.first())?;
                if let Some(room) = state.waiting_rooms.of_npc_mut(actor.id) {
                    room.event_disabled = function == Function::DisableWaitingRoomEvent;
                }
                if let Some(room) = state.waiting_rooms.of_npc(actor.id).cloned() {
                    self.fire_waiting_room_event(state, &room);
                }
                Ok(Value::default())
            }
            Function::GetWaitingRoomState => {
                let (actor, _) = self.waiting_room_target(state, context, arguments.get(1))?;
                let Some(room) = state.waiting_rooms.of_npc(actor.id) else { return Ok(Value::Number(-1)) };
                Ok(match number(0)? {
                    0 => Value::Number(room.members.len() as i32),
                    1 => Value::Number(i32::from(room.limit)),
                    2 => Value::Number(i32::from(room.trigger)),
                    3 => Value::Number(i32::from(room.event_disabled)),
                    4 => Value::String(room.title.clone()),
                    5 => Value::String(String::new()),
                    16 => Value::String(room.event.clone()),
                    32 => Value::Number(i32::from(room.members.len() >= usize::from(room.limit))),
                    33 => Value::Number(i32::from(room.members.len() >= usize::from(room.trigger))),
                    _ => Value::Number(-1),
                })
            }
            Function::WarpWaitingPc => self.warp_waiting_players(state, context, arguments),
            _ => Err(format!("{function:?} is not a waiting room command")),
        }
    }

    /// Warps the players who waited longest, charging the room's fee, and lists their account ids in `$@warpwaitingpc[]`.
    fn warp_waiting_players(&self, state: &mut ServerState, context: &ScriptRequest, arguments: &[Value]) -> Reply {
        let (actor, _) = self.waiting_room_target(state, context, None)?;
        let Some(room) = state.waiting_rooms.of_npc(actor.id).cloned() else { return Ok(Value::default()) };
        let destination = arguments.first().ok_or("Missing destination map")?.text();
        let count = arguments.get(3).map(Value::number_value).transpose()?.map_or(usize::from(room.trigger), |count| usize::try_from(count).unwrap_or(0));
        let mut warped = Vec::new();
        for member in room.members.iter().copied().take(count) {
            let Some(character) = state.get_character(member) else { break };
            if destination == "SavePoint" && state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoTeleport) {
                break;
            }
            if room.zeny > 0 && character.status.zeny < room.zeny {
                break;
            }
            let (account_id, zeny, save) = (character.account_id, character.status.zeny, (character.save_map.clone(), character.save_x, character.save_y));
            self.depart_waiting_room(state, member, false);
            if room.zeny > 0 {
                if let Some(mut character) = state.characters_mut().remove(&member) {
                    self.character_service().update_zeny(self.runtime(), CharacterZeny { char_id: member, zeny: Some(zeny - room.zeny) }, &mut character);
                    state.insert_character(character);
                }
            }
            let (map, x, y, instance) = match destination.as_str() {
                "Random" => (room.map.clone(), RANDOM_CELL.0, RANDOM_CELL.1, room.instance),
                "SavePoint" => (Map::name_without_ext(&save.0), save.1, save.2, 0),
                other => {
                    let (map, instance) = super::instance_service::split_instance_map(other);
                    (Map::name_without_ext(map), u16::try_from(arguments.get(1).ok_or("Missing x")?.number_value()?).map_err(|_| "Invalid warp coordinate")?, u16::try_from(arguments.get(2).ok_or("Missing y")?.number_value()?).map_err(|_| "Invalid warp coordinate")?, instance.unwrap_or(0))
                }
            };
            self.server_service().schedule_warp_to_walkable_cell_by_character_in_instance(&map, x, y, member, instance);
            warped.push(account_id);
        }
        let mut variables: Vec<Variable> = warped.iter().enumerate()
            .map(|(index, account_id)| Variable { scope: VariableScope::ServerTemporary, name: "warpwaitingpc".into(), index: index as u32, value: Value::Number(*account_id as i32) })
            .collect();
        variables.push(Variable { scope: VariableScope::ServerTemporary, name: "warpwaitingpcnum".into(), index: 0, value: Value::Number(warped.len() as i32) });
        self.script_service().install_temporary_variables(0, &variables);
        Ok(Value::default())
    }
}
