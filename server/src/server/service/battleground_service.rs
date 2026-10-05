use std::sync::atomic::{AtomicU64, Ordering};

use models::status_change::StatusChangeKind;
use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::battleground::BgPoint;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptWarp};
use crate::server::model::map_flags::MapFlag;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const TICK_INTERVAL_MS: u64 = 500;
const HP_PACKET_NAME_LENGTH_VERSION: u32 = 20140613;
const SERVER_NAME: &str = "Server";
const DEFAULT_DESERTER_SECONDS: u32 = 600;

static NEXT_TICK: AtomicU64 = AtomicU64::new(0);

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::BgCreate
            | Function::BgJoin
            | Function::BgLeave
            | Function::BgDesert
            | Function::BgDestroy
            | Function::BgWarp
            | Function::BgTeamSetXy
            | Function::BgGetData
            | Function::BgGetAreaUsers
            | Function::BgUpdateScore
    )
}

pub(crate) fn fixed_string(packet: &mut Vec<u8>, value: &str, length: usize) {
    let start = packet.len();
    packet.resize(start + length, 0);
    let copied = value.len().min(length - 1);
    packet[start..start + copied].copy_from_slice(&value.as_bytes()[..copied]);
}

fn position_packet(account_id: u32, name: &str, job: u32, x: u16, y: u16) -> Vec<u8> {
    let mut packet = 0x02DF_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&account_id.to_le_bytes());
    fixed_string(&mut packet, name, 24);
    packet.extend_from_slice(&(job as u16).to_le_bytes());
    packet.extend_from_slice(&x.to_le_bytes());
    packet.extend_from_slice(&y.to_le_bytes());
    packet
}

fn position_removal_packet(account_id: u32) -> Vec<u8> {
    position_packet(account_id, "", 0, u16::MAX, u16::MAX)
}

fn health_packet(packetver: u32, account_id: u32, name: &str, hp: u32, max_hp: u32) -> Vec<u8> {
    if packetver < HP_PACKET_NAME_LENGTH_VERSION {
        let (hp, max_hp) = if max_hp > u32::from(i16::MAX as u16) { (hp / (max_hp / 100).max(1), 100) } else { (hp, max_hp) };
        let mut packet = 0x02E0_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&account_id.to_le_bytes());
        fixed_string(&mut packet, name, 24);
        packet.extend_from_slice(&(hp as u16).to_le_bytes());
        packet.extend_from_slice(&(max_hp as u16).to_le_bytes());
        packet
    } else {
        let mut packet = 0x0A0E_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&account_id.to_le_bytes());
        packet.extend_from_slice(&hp.to_le_bytes());
        packet.extend_from_slice(&max_hp.to_le_bytes());
        packet
    }
}

fn chat_packet(account_id: u32, name: &str, message: &str) -> Vec<u8> {
    let mut packet = 0x02DC_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet.extend_from_slice(&account_id.to_le_bytes());
    fixed_string(&mut packet, name, 24);
    packet.extend_from_slice(message.as_bytes());
    packet.push(0);
    let length = packet.len() as u16;
    packet[2..4].copy_from_slice(&length.to_le_bytes());
    packet
}

fn score_packet(first: u16, second: u16) -> Vec<u8> {
    let mut packet = 0x02DE_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&first.to_le_bytes());
    packet.extend_from_slice(&second.to_le_bytes());
    packet
}

impl Server {
    fn battleground_send(&self, char_id: u32, packet: Vec<u8>) {
        let _ = self.server_service().notification_sender().send(Notification::Char(CharNotification::new(char_id, packet)));
    }

    pub(crate) fn battleground_call(&self, state: &mut ServerState, attached: u32, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let text = |index: usize| -> Result<String, String> { Ok(arguments.get(index).ok_or("Missing argument")?.string_value()?.clone()) };
        let optional_text = |index: usize| arguments.get(index).and_then(|value| value.string_value().ok()).cloned().unwrap_or_default();
        let team = |index: usize| -> Result<u32, String> { u32::try_from(number(index)?).map_err(|_| "Invalid battleground".to_string()) };
        let target = |index: usize| -> u32 {
            arguments.get(index).and_then(|value| value.number_value().ok()).and_then(|id| u32::try_from(id).ok()).filter(|id| *id != 0).unwrap_or(attached)
        };
        match function {
            Function::BgCreate => {
                let map = text(0)?;
                let cemetery = if map == "-" {
                    None
                } else if map_exists(state, &map) {
                    Some(BgPoint { map: normalize_map(&map), x: coordinate(number(1)?)?, y: coordinate(number(2)?)? })
                } else {
                    return Ok(Value::Number(0));
                };
                let id = state.battlegrounds.create(cemetery, optional_text(3), optional_text(4), optional_text(5), DEFAULT_DESERTER_SECONDS);
                Ok(Value::Number(id as i32))
            }
            Function::BgJoin => {
                let bg_id = team(0)?;
                let char_id = target(4);
                let destination = match optional_text(1).as_str() {
                    "" => state.battlegrounds.team(bg_id).and_then(|team| team.cemetery.clone()),
                    map => Some(BgPoint { map: normalize_map(map), x: coordinate(number(2)?)?, y: coordinate(number(3)?)? }),
                };
                let Some(destination) = destination else { return Ok(Value::Number(0)) };
                if !state.map_flags_for(&destination.map, 0).enabled(MapFlag::Battleground) {
                    return Err(format!("Map {} requires the battleground map flag", destination.map));
                }
                Ok(Value::Number(i32::from(
                    self.battleground_join(state, bg_id, char_id, None) && self.battleground_warp(state, char_id, &destination),
                )))
            }
            Function::BgLeave | Function::BgDesert => {
                let char_id = target(0);
                if state.battlegrounds.team_of(char_id) != 0 {
                    self.battleground_leave(state, char_id, false, function == Function::BgDesert);
                }
                Ok(Value::default())
            }
            Function::BgDestroy => {
                self.battleground_destroy(state, team(0)?);
                Ok(Value::default())
            }
            Function::BgWarp => {
                let bg_id = team(0)?;
                let map = normalize_map(&text(1)?);
                if !map_exists(state, &map) {
                    return Ok(Value::default());
                }
                let destination = BgPoint { map, x: coordinate(number(2)?)?, y: coordinate(number(3)?)? };
                for char_id in state.battlegrounds.member_ids(bg_id) {
                    self.battleground_warp(state, char_id, &destination);
                }
                Ok(Value::default())
            }
            Function::BgTeamSetXy => {
                let (x, y) = (coordinate(number(1)?)?, coordinate(number(2)?)?);
                if let Some(cemetery) = state.battlegrounds.team_mut(team(0)?).and_then(|team| team.cemetery.as_mut()) {
                    cemetery.x = x;
                    cemetery.y = y;
                }
                Ok(Value::default())
            }
            Function::BgGetData => {
                let members = state.battlegrounds.member_ids(team(0)?);
                Ok(match number(1)? {
                    0 => Value::Number(members.len() as i32),
                    1 => Value::Array(members.into_iter().map(|id| Value::Number(id as i32)).collect()),
                    other => return Err(format!("Unknown battleground data identifier {other}")),
                })
            }
            Function::BgGetAreaUsers => {
                let map = normalize_map(&text(1)?);
                let (x0, y0, x1, y1) = (number(2)?, number(3)?, number(4)?, number(5)?);
                let count = state
                    .battlegrounds
                    .member_ids(team(0)?)
                    .into_iter()
                    .filter_map(|id| state.characters().get(&id))
                    .filter(|member| {
                        normalize_map(member.current_map_name()) == map
                            && (x0..=x1).contains(&i32::from(member.x()))
                            && (y0..=y1).contains(&i32::from(member.y()))
                    })
                    .count();
                Ok(Value::Number(count as i32))
            }
            Function::BgUpdateScore => {
                let map = normalize_map(&text(0)?);
                if !map_exists(state, &map) {
                    return Ok(Value::default());
                }
                let (first, second) = (score(number(1)?), score(number(2)?));
                state.battlegrounds.set_score(&map, first, second);
                for character in state.characters().values().filter(|character| normalize_map(character.current_map_name()) == map) {
                    self.battleground_send(character.char_id, score_packet(first, second));
                }
                Ok(Value::default())
            }
            _ => Err("Unsupported battleground call".into()),
        }
    }

    fn battleground_warp(&self, state: &ServerState, char_id: u32, destination: &BgPoint) -> bool {
        if !state.characters().contains_key(&char_id) {
            return false;
        }
        self.add_to_next_tick(GameEvent::ScriptWarp(ScriptWarp {
            char_id,
            map: destination.map.clone(),
            x: destination.x,
            y: destination.y,
            destination_instance: None,
        }));
        true
    }

    pub(crate) fn battleground_join(&self, state: &mut ServerState, bg_id: u32, char_id: u32, entry_point: Option<BgPoint>) -> bool {
        let Some(character) = state.characters().get(&char_id) else { return false };
        let position = (character.x(), character.y());
        if !state.battlegrounds.join(bg_id, char_id, position, entry_point) {
            return false;
        }
        Self::mirror_battleground_team(state, char_id, bg_id);
        self.battleground_sync_member(state, char_id);
        true
    }

    fn mirror_battleground_team(state: &mut ServerState, char_id: u32, bg_id: u32) {
        if let Some(character) = state.characters_mut().get_mut(&char_id) {
            character.bg_id = bg_id;
        }
    }

    /// Sends the new member the roster state and announces the member to the rest of the team.
    fn battleground_sync_member(&self, state: &ServerState, char_id: u32) {
        let Some(character) = state.characters().get(&char_id) else { return };
        let bg_id = state.battlegrounds.team_of(char_id);
        let packetver = self.packetver();
        for other in state.battlegrounds.member_ids(bg_id).into_iter().filter(|id| *id != char_id) {
            let Some(other) = state.characters().get(&other) else { continue };
            if other.map_instance_key != character.map_instance_key {
                continue;
            }
            let max_hp = StatusService::instance().to_snapshot(&other.status).max_hp();
            self.battleground_send(char_id, position_packet(other.account_id, &other.name, other.status.job, other.x(), other.y()));
            self.battleground_send(char_id, health_packet(packetver, other.account_id, &other.name, other.status.hp, max_hp));
        }
        let (first, second) = state.battlegrounds.score(&normalize_map(character.current_map_name()));
        if (first, second) != (0, 0) {
            self.battleground_send(char_id, score_packet(first, second));
        }
    }

    /// Removes the member from the team; returns the remaining member count.
    pub(crate) fn battleground_leave(&self, state: &mut ServerState, char_id: u32, quit: bool, deserter: bool) -> Option<usize> {
        let quit_event = quit
            .then(|| state.battlegrounds.team(state.battlegrounds.team_of(char_id)).map(|team| team.quit_event.clone()))
            .flatten()
            .filter(|event| !event.is_empty());
        let (bg_id, entry_point, deserter_seconds) = state.battlegrounds.leave(char_id)?;
        Self::mirror_battleground_team(state, char_id, 0);
        if let Some(event) = quit_event {
            self.trigger_npc_event(state, &event);
        }
        let Some(character) = state.characters().get(&char_id) else {
            return Some(state.battlegrounds.member_ids(bg_id).len());
        };
        let (account_id, name) = (character.account_id, character.name.clone());
        let save_point = BgPoint { map: normalize_map(&character.save_map), x: character.save_x, y: character.save_y };
        let removal = position_removal_packet(account_id);
        for other in state.battlegrounds.member_ids(bg_id) {
            self.battleground_send(other, removal.clone());
        }
        if !quit {
            let destination = entry_point
                .filter(|point| !state.map_flags_for(&point.map, 0).enabled(MapFlag::NoSave))
                .unwrap_or(save_point);
            self.battleground_warp(state, char_id, &destination);
        }
        let message = if quit {
            format!("{SERVER_NAME}: {name} has quit the game...")
        } else {
            format!("{SERVER_NAME}: {name} is leaving the battlefield...")
        };
        self.battleground_message(state, bg_id, 0, SERVER_NAME, &message);
        if deserter && deserter_seconds > 0 {
            let duration = i32::try_from(deserter_seconds.saturating_mul(1000)).unwrap_or(i32::MAX);
            self.start_battleground_queue_status(state, char_id, StatusChangeKind::EntryQueueNotifyAdmissionTimeOut, duration);
        }
        Some(state.battlegrounds.member_ids(bg_id).len())
    }

    pub(crate) fn battleground_destroy(&self, state: &mut ServerState, bg_id: u32) {
        let removals: Vec<(u32, Vec<u8>)> = state
            .battlegrounds
            .member_ids(bg_id)
            .into_iter()
            .filter_map(|id| state.characters().get(&id).map(|character| (id, position_removal_packet(character.account_id))))
            .collect();
        let members = state.battlegrounds.delete(bg_id);
        for member in &members {
            Self::mirror_battleground_team(state, *member, 0);
        }
        for (_, removal) in &removals {
            for member in &members {
                self.battleground_send(*member, removal.clone());
            }
        }
    }

    pub(crate) fn battleground_message(&self, state: &ServerState, bg_id: u32, account_id: u32, name: &str, message: &str) {
        let packet = chat_packet(account_id, name, message);
        for member in state.battlegrounds.member_ids(bg_id) {
            self.battleground_send(member, packet.clone());
        }
    }

    /// Relays a `name : text` line to the sender's battleground team; false when the sender has none.
    pub(crate) fn battleground_chat(&self, state: &ServerState, sender: &Character, message: &str) -> bool {
        let bg_id = state.battlegrounds.team_of(sender.char_id);
        if bg_id == 0 {
            return false;
        }
        self.battleground_message(state, bg_id, sender.account_id, &sender.name, message);
        true
    }

    pub(crate) fn battleground_cemetery(&self, state: &ServerState, char_id: u32) -> Option<BgPoint> {
        state.battlegrounds.team(state.battlegrounds.team_of(char_id))?.cemetery.clone()
    }

    pub(crate) fn battleground_member_died(&self, state: &ServerState, char_id: u32) {
        let Some(team) = state.battlegrounds.team(state.battlegrounds.team_of(char_id)) else { return };
        if !team.die_event.is_empty() {
            self.trigger_player_npc_event(state, char_id, &team.die_event);
        }
    }

    pub(crate) fn tick_battlegrounds(&self, state: &mut ServerState, tick: u128) {
        if (tick as u64) < NEXT_TICK.load(Ordering::Relaxed) {
            return;
        }
        NEXT_TICK.store(tick as u64 + TICK_INTERVAL_MS, Ordering::Relaxed);
        let online: Vec<u32> = state.characters().keys().copied().collect();
        state.battlegrounds.prune(|id| online.contains(&id));
        let members: Vec<(u32, u32, u16, u16, u32, bool)> = state
            .battlegrounds
            .members_mut()
            .map(|(bg_id, member)| (bg_id, member.char_id, member.last_x, member.last_y, member.last_hp, member.seen_on_battleground_map))
            .collect();
        let packetver = self.packetver();
        for (bg_id, char_id, last_x, last_y, last_hp, seen) in members {
            let Some(character) = state.characters().get(&char_id) else { continue };
            let on_battleground = state.map_flags(&character.map_instance_key).enabled(MapFlag::Battleground);
            if seen && !on_battleground {
                self.battleground_leave(state, char_id, false, true);
                continue;
            }
            let (x, y, hp) = (character.x(), character.y(), character.status.hp);
            let peers: Vec<u32> = state
                .battlegrounds
                .member_ids(bg_id)
                .into_iter()
                .filter(|id| *id != char_id)
                .filter(|id| state.characters().get(id).is_some_and(|peer| peer.map_instance_key == character.map_instance_key))
                .collect();
            let arrived = on_battleground && !seen;
            if on_battleground && ((x, y) != (last_x, last_y) || arrived) {
                let packet = position_packet(character.account_id, &character.name, character.status.job, x, y);
                for peer in &peers {
                    self.battleground_send(*peer, packet.clone());
                }
            }
            if on_battleground && (hp != last_hp || arrived) {
                let max_hp = StatusService::instance().to_snapshot(&character.status).max_hp();
                let packet = health_packet(packetver, character.account_id, &character.name, hp, max_hp);
                for peer in &peers {
                    self.battleground_send(*peer, packet.clone());
                }
            }
            if arrived {
                self.battleground_sync_member(state, char_id);
            }
            if let Some((_, member)) = state.battlegrounds.members_mut().find(|(_, member)| member.char_id == char_id) {
                member.last_x = x;
                member.last_y = y;
                member.last_hp = hp;
                member.seen_on_battleground_map |= on_battleground;
            }
        }
    }
}

fn map_exists(state: &ServerState, map: &str) -> bool {
    let map = normalize_map(map);
    GlobalConfigService::instance().find_map(&map).is_some() || state.get_map_instance(&map, 0).is_some()
}

fn coordinate(value: i32) -> Result<u16, String> {
    u16::try_from(value).map_err(|_| "Invalid battleground coordinate".into())
}

fn score(value: i32) -> u16 {
    value.clamp(0, i32::from(u16::MAX)) as u16
}
