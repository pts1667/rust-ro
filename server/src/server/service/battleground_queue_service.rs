use models::status_change::{StatusChangeKind, StatusChangeRequest};
use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::model::battleground::BgPoint;
use crate::server::model::battleground_queue::{
    ADMISSION_WINDOW_MS, APPLY_DELAY_MS, BattlegroundQueueAction, BattlegroundQueueCommand, BgType, QueueSide, QueueState, REQUEUE_INTERVAL_MS,
    battleground_type, battleground_type_by_name, battleground_types,
};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::battleground_service::fixed_string;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::util::tick::get_tick;

const LIGHT_GREEN: u32 = 0x55FF00;
const SOLO_APPLICATION: u16 = 1;
const PARTY_APPLICATION: u16 = 2;
const GUILD_APPLICATION: u16 = 4;

const MESSAGE_WRONG_MAP: &str = "You can't apply to a battleground queue from this map.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ApplyResult {
    None = 0,
    Accept = 1,
    QueueFinished = 2,
    InvalidName = 3,
    InvalidApplication = 4,
    PlayerCount = 5,
    PlayerLevel = 6,
    Duplicate = 7,
    Reconnect = 8,
    PartyGuildLeader = 9,
    PlayerClass = 10,
}

pub(crate) fn handles_script_call(function: Function) -> bool {
    matches!(function, Function::BgReserve | Function::BgUnbook | Function::BgInfo)
}

fn named_packet(id: u16, result: Option<u8>, names: &[&str], trailer: Option<u32>) -> Vec<u8> {
    let mut packet = id.to_le_bytes().to_vec();
    if let Some(result) = result {
        packet.push(result);
    }
    for name in names {
        fixed_string(&mut packet, name, 24);
    }
    if let Some(number) = trailer {
        packet.extend_from_slice(&number.to_le_bytes());
    }
    packet
}

fn notice_packet(text: &str) -> Vec<u8> {
    let [red, green, blue, _] = LIGHT_GREEN.to_le_bytes();
    let color = u32::from_le_bytes([blue, green, red, 0]);
    let mut packet = 0x02C1_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&((13 + text.len()) as u16).to_le_bytes());
    packet.extend_from_slice(&0u32.to_le_bytes());
    packet.extend_from_slice(&color.to_le_bytes());
    packet.extend_from_slice(text.as_bytes());
    packet.push(0);
    packet
}

impl Server {
    fn bg_queue_send(&self, char_id: u32, packet: Vec<u8>) {
        let _ = self.server_service().notification_sender().send(Notification::Char(CharNotification::new(char_id, packet)));
    }

    fn apply_result(&self, char_id: u32, result: ApplyResult, name: &str) {
        self.bg_queue_send(char_id, named_packet(0x08D8, Some(result as u8), &[name], None));
    }

    fn apply_notify(&self, state: &ServerState, char_id: u32, name: &str) {
        let Some(queue) = state.battlegrounds.queues.queue_of(char_id).map(|index| &state.battlegrounds.queues.queues[index]) else { return };
        self.bg_queue_send(char_id, named_packet(0x08D9, None, &[name], Some(queue.len() as u32)));
    }

    fn lobby_notify(&self, char_id: u32, name: &str) {
        self.bg_queue_send(char_id, named_packet(0x08DF, None, &[name, name], None));
    }

    fn entry_init(&self, char_id: u32) {
        self.bg_queue_send(char_id, 0x090E_u16.to_le_bytes().to_vec());
    }

    fn queue_notice(&self, char_id: u32, text: &str) {
        self.bg_queue_send(char_id, notice_packet(text));
    }

    pub(crate) fn with_character<R>(&self, state: &mut ServerState, char_id: u32, action: impl FnOnce(&mut Character) -> R) -> Option<R> {
        let mut character = state.characters_mut().remove(&char_id)?;
        let result = action(&mut character);
        state.insert_character(character);
        Some(result)
    }

    pub(crate) fn start_battleground_queue_status(&self, state: &mut ServerState, char_id: u32, kind: StatusChangeKind, duration_ms: i32) {
        let sender = self.server_service().notification_sender();
        let request = StatusChangeRequest::guaranteed(kind, duration_ms, 1);
        self.with_character(state, char_id, |character| {
            if let Err(error) = StatusEffectService::start(self, character, request, get_tick(), &sender) {
                warn!("Battleground queue status failed: {error}");
            }
        });
    }

    fn queue_status_blocks(&self, state: &ServerState, char_id: u32, name: &str) -> bool {
        let Some(character) = state.characters().get(&char_id) else { return true };
        let tick = get_tick();
        let remaining = |kind: StatusChangeKind| {
            character
                .status
                .status_change(kind)
                .and_then(|change| change.expires_at)
                .map(|expires_at| (expires_at.saturating_sub(tick) / 1000) as u32)
        };
        if let Some(seconds) = remaining(StatusChangeKind::EntryQueueApplyDelay) {
            self.apply_result(char_id, ApplyResult::None, name);
            self.queue_notice(char_id, &format!("You can't apply to a battleground queue for {seconds} seconds due to recently leaving one."));
            return true;
        }
        if let Some(seconds) = remaining(StatusChangeKind::EntryQueueNotifyAdmissionTimeOut) {
            self.apply_result(char_id, ApplyResult::None, name);
            self.queue_notice(
                char_id,
                &format!(
                    "You can't apply to a battleground queue due to recently deserting a battleground. Time remaining: {} minutes and {} seconds.",
                    seconds / 60,
                    seconds % 60
                ),
            );
            return true;
        }
        false
    }

    fn queue_joinable(&self, state: &ServerState, bg: &BgType, char_id: u32) -> bool {
        let Some(character) = state.characters().get(&char_id) else { return false };
        let name = bg.name.as_str();
        if bg.job_restrictions.contains(&character.status.job) {
            self.apply_result(char_id, ApplyResult::PlayerClass, name);
            return false;
        }
        let level = u32::from(character.status.base_level);
        if (bg.min_level > 0 && level < bg.min_level) || (bg.max_level > 0 && level > bg.max_level) {
            self.apply_result(char_id, ApplyResult::PlayerLevel, name);
            return false;
        }
        if self.queue_status_blocks(state, char_id, name) {
            return false;
        }
        let current_map = normalize_map(character.current_map_name());
        if battleground_types().iter().flat_map(|bg| &bg.maps).any(|map| map.map == current_map) {
            self.apply_result(char_id, ApplyResult::None, name);
            self.queue_notice(char_id, MESSAGE_WRONG_MAP);
            return false;
        }
        true
    }

    pub(crate) fn handle_battleground_queue_command(&self, state: &mut ServerState, command: BattlegroundQueueCommand) {
        state.battlegrounds.queues.ensure_created();
        let char_id = command.char_id;
        match command.action {
            BattlegroundQueueAction::Apply { kind, name } => self.queue_apply(state, char_id, kind, &name),
            BattlegroundQueueAction::Cancel(name) => {
                let Some(index) = state.battlegrounds.queues.queue_of(char_id) else {
                    return self.bg_queue_send(char_id, named_packet(0x08DB, Some(0), &[&name], None));
                };
                if state.battlegrounds.queues.queues[index].state == QueueState::SetupDelay {
                    return;
                }
                let success = self.battleground_queue_leave(state, char_id, true);
                self.bg_queue_send(char_id, named_packet(0x08DB, Some(u8::from(success)), &[&name], None));
            }
            BattlegroundQueueAction::Reply { accept } => {
                if state.battlegrounds.queues.queue_of(char_id).is_none() {
                    return;
                }
                if accept {
                    self.queue_accept(state, char_id);
                } else {
                    self.battleground_queue_leave(state, char_id, true);
                    self.entry_init(char_id);
                }
            }
            BattlegroundQueueAction::Number(name) => self.apply_notify(state, char_id, &name),
        }
    }

    fn queue_apply(&self, state: &mut ServerState, char_id: u32, kind: u16, name: &str) {
        if state.battlegrounds.queues.queue_of(char_id).is_some() {
            return self.apply_result(char_id, ApplyResult::Duplicate, name);
        }
        let Some(bg) = battleground_type_by_name(name) else {
            return self.apply_result(char_id, ApplyResult::InvalidName, name);
        };
        let Some(character) = state.characters().get(&char_id) else { return };
        let mut group = vec![char_id];
        match kind {
            SOLO_APPLICATION if bg.solo => {}
            PARTY_APPLICATION if bg.party => {
                let Some(party) = character.game_systems.party.as_ref().filter(|party| party.id == character.game_systems.party_id) else {
                    return self.apply_result(char_id, ApplyResult::InvalidApplication, name);
                };
                if party.leader_char_id != char_id {
                    return self.apply_result(char_id, ApplyResult::PartyGuildLeader, name);
                }
                group = party.members.iter().copied().filter(|id| state.characters().contains_key(id)).collect();
            }
            GUILD_APPLICATION if bg.guild => {
                let guild_id = character.game_systems.guild_id;
                let is_master = guild_id != 0
                    && self.repository.guild(guild_id).ok().flatten().is_some_and(|guild| guild.master_char_id == char_id);
                if !is_master {
                    let result = if guild_id == 0 { ApplyResult::InvalidApplication } else { ApplyResult::PartyGuildLeader };
                    return self.apply_result(char_id, result, name);
                }
                group = state.characters().values().filter(|member| member.game_systems.guild_id == guild_id).map(|member| member.char_id).collect();
            }
            _ => return self.apply_result(char_id, ApplyResult::InvalidApplication, name),
        }
        if kind != SOLO_APPLICATION && group.len() > bg.max_players {
            return self.apply_result(char_id, ApplyResult::PlayerCount, name);
        }
        group.truncate(bg.max_players);
        self.queue_join_group(state, bg, char_id, group);
    }

    fn queue_join_group(&self, state: &mut ServerState, bg: &'static BgType, requester: u32, mut group: Vec<u32>) {
        if !self.queue_joinable(state, bg, requester) {
            return;
        }
        let queues = &state.battlegrounds.queues;
        let Some(index) = queues
            .queues
            .iter()
            .position(|queue| queue.bg_id == bg.id && !matches!(queue.state, QueueState::SetupDelay | QueueState::Ended))
        else {
            return self.apply_result(requester, ApplyResult::Reconnect, &bg.name);
        };
        let Some(side) = queues.queues[index].choose_side(group.len(), bg, fastrand::bool()) else {
            return self.apply_result(requester, ApplyResult::Reconnect, &bg.name);
        };
        let queue_id = queues.queues[index].queue_id;
        while let Some(member) = group.pop() {
            if state.battlegrounds.queues.queues[index].side_mut(side).len() >= bg.max_players {
                break;
            }
            if state.battlegrounds.queues.queue_of(member).is_some() || !self.queue_joinable(state, bg, member) {
                continue;
            }
            state.battlegrounds.queues.queues[index].side_mut(side).push(member);
            self.apply_result(member, ApplyResult::Accept, &bg.name);
            self.apply_notify(state, member, &bg.name);
        }
        let queue = &state.battlegrounds.queues.queues[index];
        match queue.state {
            QueueState::Active => {
                let map = queue.map_index.map(|map| normalize_map(&bg.maps[map].map));
                let waiting: Vec<u32> = match side {
                    QueueSide::A => queue.team_a.clone(),
                    QueueSide::B => queue.team_b.clone(),
                }
                .into_iter()
                .filter(|id| state.characters().get(id).is_some_and(|member| Some(normalize_map(member.current_map_name())) != map))
                .collect();
                for member in waiting {
                    self.lobby_notify(member, &bg.name);
                }
            }
            QueueState::Setup if queue.team_a.len() >= bg.required_players && queue.team_b.len() >= bg.required_players => {
                self.queue_on_ready(state, queue_id);
            }
            _ => {}
        }
    }

    /// Reserves a free arena and asks every queued player to accept.
    fn queue_on_ready(&self, state: &mut ServerState, queue_id: u32) {
        let Some(index) = state.battlegrounds.queues.find(queue_id) else { return };
        let bg_id = state.battlegrounds.queues.queues[index].bg_id;
        let Some(bg) = battleground_type(bg_id) else { return };
        let queue = &state.battlegrounds.queues.queues[index];
        if queue.team_a.len() < bg.required_players || queue.team_b.len() < bg.required_players {
            return;
        }
        let Some(map_index) = state.battlegrounds.queues.reserve_free_map(bg) else {
            state.battlegrounds.queues.queues[index].requeue_at = Some(get_tick() as u64 + REQUEUE_INTERVAL_MS);
            return;
        };
        let now = get_tick() as u64;
        let queue = &mut state.battlegrounds.queues.queues[index];
        queue.map_index = Some(map_index);
        queue.state = QueueState::SetupDelay;
        queue.expire_at = Some(now + ADMISSION_WINDOW_MS);
        let members: Vec<u32> = queue.members().collect();
        for member in members {
            self.lobby_notify(member, &bg.name);
        }
    }

    fn queue_accept(&self, state: &mut ServerState, char_id: u32) {
        let Some(index) = state.battlegrounds.queues.queue_of(char_id) else { return };
        let queue = &mut state.battlegrounds.queues.queues[index];
        let Some(bg) = battleground_type(queue.bg_id) else { return };
        let Some(map_index) = queue.map_index else { return };
        let map_name = bg.maps[map_index].map.as_str();
        queue.accepted += 1;
        self.bg_queue_send(char_id, named_packet(0x08E1, Some(1), &[map_name, map_name], None));
        match queue.state {
            QueueState::Active => self.join_active_battleground(state, char_id, index),
            QueueState::SetupDelay if queue.accepted == bg.required_players * 2 => {
                queue.expire_at = None;
                queue.start_at = Some(get_tick() as u64 + u64::from(bg.start_delay_seconds) * 1000);
            }
            _ => {}
        }
    }

    pub(crate) fn battleground_queue_leave(&self, state: &mut ServerState, char_id: u32, apply_delay: bool) -> bool {
        let Some(index) = state.battlegrounds.queues.queue_of(char_id) else { return false };
        let queue = &mut state.battlegrounds.queues.queues[index];
        queue.team_a.retain(|id| *id != char_id);
        queue.team_b.retain(|id| *id != char_id);
        let discard = matches!(queue.state, QueueState::Setup | QueueState::SetupDelay) && queue.len() == 0;
        if discard {
            self.clear_queue(state, index, true);
        }
        if apply_delay {
            self.start_battleground_queue_status(state, char_id, StatusChangeKind::EntryQueueApplyDelay, APPLY_DELAY_MS);
        }
        true
    }

    fn clear_queue(&self, state: &mut ServerState, index: usize, ended: bool) {
        let queues = &mut state.battlegrounds.queues;
        if ended {
            if let Some(map) = queues.queues[index].map_index.and_then(|map| battleground_type(queues.queues[index].bg_id).map(|bg| bg.maps[map].map.clone())) {
                queues.reserved_maps.remove(&map);
            }
        }
        queues.queues[index].clear(ended);
    }

    fn join_active_battleground(&self, state: &mut ServerState, char_id: u32, index: usize) {
        let queue = &state.battlegrounds.queues.queues[index];
        let Some(bg) = battleground_type(queue.bg_id) else { return };
        let Some(map) = queue.map_index.map(|map| &bg.maps[map]) else { return };
        let Some(side) = queue.side_of(char_id) else { return };
        let definition = if side == QueueSide::A { &map.team_a } else { &map.team_b };
        let team_id = self.script_service().server_temporary(&definition.variable).and_then(|value| value.number_value().ok()).unwrap_or(0);
        let team_id = u32::try_from(team_id).unwrap_or(0);
        if team_id == 0 || state.battlegrounds.team(team_id).is_none() {
            self.battleground_queue_leave(state, char_id, true);
            self.apply_result(char_id, ApplyResult::Reconnect, &bg.name);
            self.entry_init(char_id);
            return;
        }
        let entry_point = state.characters().get(&char_id).map(|character| BgPoint {
            map: normalize_map(character.current_map_name()),
            x: character.x(),
            y: character.y(),
        });
        self.entry_init(char_id);
        if self.battleground_join(state, team_id, char_id, entry_point) {
            if let Some(team) = state.battlegrounds.team(team_id) {
                if !team.active_event.is_empty() {
                    let label = team.active_event.clone();
                    self.trigger_player_npc_event(state, char_id, &label);
                }
            }
        }
    }

    fn start_battleground(&self, state: &mut ServerState, index: usize) {
        let queue = &state.battlegrounds.queues.queues[index];
        let Some(bg) = battleground_type(queue.bg_id) else {
            return self.clear_queue(state, index, true);
        };
        let Some(map) = queue.map_index.map(|map| &bg.maps[map]) else {
            return self.clear_queue(state, index, true);
        };
        let (side_a, side_b) = (queue.team_a.clone(), queue.team_b.clone());
        let map_name = normalize_map(&map.map);
        let mut team_ids = [0u32; 2];
        for (slot, definition) in [&map.team_a, &map.team_b].into_iter().enumerate() {
            let cemetery = BgPoint { map: map_name.clone(), x: definition.x, y: definition.y };
            team_ids[slot] = state.battlegrounds.create(
                Some(cemetery),
                definition.quit_event.clone(),
                definition.death_event.clone(),
                definition.active_event.clone(),
                bg.deserter_seconds,
            );
        }
        for (members, team_id) in [(side_a, team_ids[0]), (side_b, team_ids[1])] {
            for member in members {
                self.entry_init(member);
                let entry_point = state.characters().get(&member).map(|character| BgPoint {
                    map: normalize_map(character.current_map_name()),
                    x: character.x(),
                    y: character.y(),
                });
                self.battleground_join(state, team_id, member, entry_point);
            }
        }
        self.script_service().set_server_temporary(&map.team_a.variable, Value::Number(team_ids[0] as i32));
        self.script_service().set_server_temporary(&map.team_b.variable, Value::Number(team_ids[1] as i32));
        self.trigger_npc_event(state, &map.start_event);
        state.battlegrounds.queues.queues[index].state = QueueState::Active;
        self.clear_queue(state, index, false);
    }

    fn expire_queue(&self, state: &mut ServerState, index: usize) {
        let queue = &state.battlegrounds.queues.queues[index];
        let Some(bg) = battleground_type(queue.bg_id) else { return };
        let members: Vec<u32> = queue.members().collect();
        for member in members {
            self.apply_result(member, ApplyResult::QueueFinished, &bg.name);
            self.entry_init(member);
        }
        self.clear_queue(state, index, true);
    }

    pub(crate) fn tick_battleground_queues(&self, state: &mut ServerState, tick: u128) {
        let now = tick as u64;
        for index in 0..state.battlegrounds.queues.queues.len() {
            let queue = &state.battlegrounds.queues.queues[index];
            let (expire, start, requeue, queue_id) = (queue.expire_at, queue.start_at, queue.requeue_at, queue.queue_id);
            if expire.is_some_and(|at| now >= at) {
                self.expire_queue(state, index);
            } else if start.is_some_and(|at| now >= at) {
                state.battlegrounds.queues.queues[index].start_at = None;
                self.start_battleground(state, index);
            } else if requeue.is_some_and(|at| now >= at) {
                state.battlegrounds.queues.queues[index].requeue_at = None;
                self.queue_on_ready(state, queue_id);
            }
        }
        let offline: Vec<u32> = state
            .battlegrounds
            .queues
            .queues
            .iter()
            .flat_map(|queue| queue.members())
            .filter(|id| !state.characters().contains_key(id))
            .collect();
        for char_id in offline {
            self.battleground_queue_leave(state, char_id, false);
        }
    }

    pub(crate) fn battleground_queue_script_call(&self, state: &mut ServerState, function: Function, arguments: &[Value]) -> Reply {
        let name = arguments.first().ok_or("Missing argument")?.string_value()?.clone();
        match function {
            Function::BgReserve | Function::BgUnbook => {
                state.battlegrounds.queues.ensure_created();
                let reserve = function == Function::BgReserve;
                let ended = arguments.get(1).and_then(|value| value.number_value().ok()).unwrap_or(0) != 0;
                let map = normalize_map(&name);
                let Some(bg) = battleground_types().iter().find(|bg| bg.maps.iter().any(|candidate| candidate.map == map)) else {
                    return Ok(Value::Number(0));
                };
                let map_index = bg.maps.iter().position(|candidate| candidate.map == map).unwrap_or(0);
                if reserve {
                    state.battlegrounds.queues.reserved_maps.insert(map.clone());
                } else {
                    state.battlegrounds.queues.reserved_maps.remove(&map);
                }
                let affected: Vec<usize> = state
                    .battlegrounds
                    .queues
                    .queues
                    .iter()
                    .enumerate()
                    .filter(|(_, queue)| queue.bg_id == bg.id && queue.map_index == Some(map_index))
                    .map(|(index, _)| index)
                    .collect();
                for index in affected {
                    if ended {
                        state.battlegrounds.queues.queues[index].state = QueueState::Ended;
                    }
                    if !reserve {
                        self.clear_queue(state, index, true);
                    }
                }
                Ok(Value::Number(1))
            }
            Function::BgInfo => {
                let bg = battleground_type_by_name(&name).ok_or_else(|| format!("Invalid battleground name {name}"))?;
                let kind = arguments.get(1).ok_or("Missing argument")?.number_value()?;
                Ok(match kind {
                    0 => Value::Number(bg.id as i32),
                    1 => Value::Number(bg.required_players as i32),
                    2 => Value::Number(bg.max_players as i32),
                    3 => Value::Number(bg.min_level as i32),
                    4 => Value::Number(bg.max_level as i32),
                    5 => Value::Array(bg.maps.iter().map(|map| Value::String(map.map.clone())).collect()),
                    6 => Value::Number(bg.deserter_seconds as i32),
                    other => return Err(format!("Unknown battleground info type {other}")),
                })
            }
            _ => Err("Unsupported battleground queue call".into()),
        }
    }
}
