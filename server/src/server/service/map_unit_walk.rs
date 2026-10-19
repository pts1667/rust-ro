use models::enums::mob::MobMode;
use movement::hierarchy;
use movement::position::Position;
use packets::packets::{Packet, PacketZcChangeDirection, PacketZcNotifyMove, PacketZcStopmove};

use super::MapInstanceService;
use crate::server::model::events::game_event::{GameEvent, ScriptNpcEvent};
use crate::server::model::events::map_event::{ForcedWalk, ScriptNpcCallback};
use crate::server::model::movement::{Movable, Movement};
use crate::server::model::path::path_search_client_side_algorithm;
use crate::server::script::skill::ScriptSkillService;
use crate::server::script::skill::actor::{NpcSkillState, NpcWalk};
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::mob::{Mob, MobAction};
use crate::util::tick::get_tick_client;

/// True when `npcwalkto` can take the NPC to `(x, y)`: it stands there or a path leads there, however long.
pub(crate) fn npc_can_walk_to(state: &MapInstanceState, npc: &NpcSkillState, x: u16, y: u16) -> bool {
    (npc.x, npc.y) == (x, y) || state.navigation().reachable((npc.x, npc.y), (x, y))
}

/// The first leg of an NPC walk from `from` to `to` and the ends of the legs after it, the next one last. A walk the client
/// can path in one go is one leg; a longer one follows the waypoints of the pathfinding hierarchy.
fn npc_walk_legs(state: &MapInstanceState, from: (u16, u16), to: (u16, u16)) -> Option<(Vec<Movement>, Vec<(u16, u16)>)> {
    let direct = walk_path(state, from, to);
    if !direct.is_empty() {
        return Some((direct, vec![]));
    }
    let mut legs = state.navigation().route(state.cells(), from, to)?;
    legs.reverse();
    let (steps, rest) = next_npc_leg(state, from, legs)?;
    Some((steps, rest))
}

/// Paths the next leg; when the client's A* cannot, the leg is cut into shorter ones along a local route.
fn next_npc_leg(state: &MapInstanceState, from: (u16, u16), mut legs: Vec<(u16, u16)>) -> Option<(Vec<Movement>, Vec<(u16, u16)>)> {
    skip_legs_ending_at(&mut legs, from);
    let leg = legs.pop()?;
    let steps = walk_path(state, from, leg);
    if !steps.is_empty() {
        return Some((steps, legs));
    }
    let local = hierarchy::local_route(state.x_size(), state.y_size(), state.cells(), from, leg, hierarchy::CHUNK)?;
    let mut sub_legs = hierarchy::sub_legs(&local).into_iter();
    let first = sub_legs.next()?;
    legs.extend(sub_legs.rev());
    let steps = walk_path(state, from, first);
    (!steps.is_empty()).then_some((steps, legs))
}

/// Waypoints can fall on the cell a leg ended on, which leaves nothing to walk.
fn skip_legs_ending_at(legs: &mut Vec<(u16, u16)>, at: (u16, u16)) {
    while legs.last() == Some(&at) {
        legs.pop();
    }
}

/// The cells from `from` to `to`, the next one last, as the client would walk them; empty when nothing leads there.
pub(crate) fn walk_path(state: &MapInstanceState, from: (u16, u16), (x, y): (u16, u16)) -> Vec<Movement> {
    if x >= state.x_size() || y >= state.y_size() {
        return vec![];
    }
    let path = path_search_client_side_algorithm(state.x_size(), state.y_size(), state.cells(), from.0, from.1, x, y);
    // The search answers a path to the last cell it tried when it runs out of cells.
    if path.last().is_none_or(|end| (end.x, end.y) != (x, y)) {
        return vec![];
    }
    Movement::from_path(path, 0)
}

/// What stops a monster from taking a `unitwalk`.
pub(crate) fn mob_can_walk(mob: &Mob) -> bool {
    mob.is_present() && mob.status.hp() > 0 && MobMode::can_move(mob.mode) && !mob.blocks_movement()
        && !(mob.forced_walk.is_some() && mob.is_moving())
}

/// The walk of the NPC from the cell it stands on, for clients that see it start or come into view while it walks.
pub(crate) fn npc_walk_packet(npc: &NpcSkillState, packetver: u32) -> Option<Vec<u8>> {
    npc.walk.as_ref().map(|walk| move_packet(npc.id, (npc.x, npc.y), walk.destination, packetver))
}

fn move_packet(id: u32, (from_x, from_y): (u16, u16), (x, y): (u16, u16), packetver: u32) -> Vec<u8> {
    let mut packet = PacketZcNotifyMove::new(packetver);
    packet.set_gid(id);
    packet.move_data = Position { x: from_x, y: from_y, dir: 0 }.to_move_data(&Position { x, y, dir: 0 });
    packet.set_move_start_time(get_tick_client());
    packet.fill_raw();
    packet.raw
}

impl MapInstanceService {
    /// Replaces any walk in progress. A hidden NPC, or a destination no path leads to, leaves the NPC as it is.
    pub(super) fn start_npc_walk(&self, state: &mut MapInstanceState, npc_id: u32, (x, y): (u16, u16), on_arrival: Option<ScriptNpcCallback>, forced: bool, tick: u128) {
        let Some(mut npc) = state.script_skill_state.npcs.get(&npc_id).cloned().filter(|npc| !npc.hidden && !npc.is_dead()) else { return };
        if (npc.x, npc.y) == (x, y) {
            if npc.walk.take().is_some() {
                state.script_skill_state.npcs.insert(npc_id, npc.clone());
                self.notify_area(state, npc.x, npc.y, stop_packet(npc.id, x, y, self.configuration_service.packetver()));
            }
            self.queue_walk_arrival(on_arrival, None, Some(npc_id));
            return;
        }
        let Some((mut steps, legs)) = npc_walk_legs(state, (npc.x, npc.y), (x, y)) else { return };
        let Some(first) = steps.last_mut() else { return };
        first.set_move_at(tick + Movement::delay(npc.snapshot().speed(), first.is_diagonal()));
        let destination = steps.first().map_or((x, y), |end| (end.position().x, end.position().y));
        npc.walk = Some(NpcWalk { steps, destination, legs, on_arrival, forced });
        self.cancel_npc_cast(state, npc_id);
        state.script_skill_state.npcs.insert(npc_id, npc.clone());
        if let Some(packet) = npc_walk_packet(&npc, self.configuration_service.packetver()) {
            self.notify_area(state, npc.x, npc.y, packet);
        }
    }

    /// `movenpc` onto the cell the NPC stands on: it turns without being drawn again.
    pub(super) fn turn_npc(&self, state: &mut MapInstanceState, npc_id: u32, dir: u16) {
        let Some(npc) = state.script_skill_state.npcs.get_mut(&npc_id) else { return };
        npc.dir = dir;
        let (hidden, x, y) = (npc.hidden, npc.x, npc.y);
        if hidden {
            return;
        }
        let mut packet = PacketZcChangeDirection::new(self.configuration_service.packetver());
        packet.set_aid(npc_id);
        packet.set_dir(dir as u8);
        packet.fill_raw();
        self.notify_area(state, x, y, packet.raw);
    }

    /// rathena's default `npcstop`: the NPC finishes the cell it is entering, so clients that drew it half-way agree.
    pub(super) fn stop_npc_walk(&self, state: &mut MapInstanceState, npc_id: u32) {
        let Some(npc) = state.script_skill_state.npcs.get_mut(&npc_id) else { return };
        let Some(walk) = npc.walk.as_mut() else { return };
        let Some(next) = walk.steps.pop() else {
            npc.walk = None;
            return;
        };
        let (x, y) = (next.position().x, next.position().y);
        *walk = NpcWalk { steps: vec![next], destination: (x, y), legs: vec![], on_arrival: None, forced: walk.forced };
        let (id, from_x, from_y) = (npc.id, npc.x, npc.y);
        self.notify_area(state, from_x, from_y, stop_packet(id, x, y, self.configuration_service.packetver()));
    }

    /// Clients walk an NPC at the speed of its last entry packet, so a walking NPC is drawn again and sent on from where it is.
    pub(super) fn set_npc_speed(&self, state: &mut MapInstanceState, npc_id: u32, speed: u16) {
        let Some(npc) = state.script_skill_state.npcs.get_mut(&npc_id) else { return };
        npc.speed = speed;
        npc.base_status.set_base_speed(speed);
        npc.base_status.set_speed(speed);
        let npc = npc.clone();
        if npc.hidden {
            return;
        }
        self.refresh_npc(state, &npc);
        if let Some(packet) = npc_walk_packet(&npc, self.configuration_service.packetver()) {
            self.notify_area(state, npc.x, npc.y, packet);
        }
    }

    /// Moves walking NPCs along their path, one cell per step delay, starts the next leg of those that end one, and runs the
    /// arrival label of those that arrive.
    pub fn step_npc_walks(&self, state: &mut MapInstanceState, tick: u128) {
        let mut arrivals = Vec::new();
        let mut leg_ends = Vec::new();
        for npc in state.script_skill_state.npcs.values_mut() {
            let due = |walk: &NpcWalk| walk.steps.last().is_some_and(|step| tick >= step.move_at());
            if !npc.walk.as_ref().is_some_and(due) {
                continue;
            }
            let speed = npc.snapshot().speed();
            let walk = npc.walk.as_mut().unwrap();
            while let Some(step) = walk.steps.last().copied().filter(|step| tick >= step.move_at()) {
                walk.steps.pop();
                let (x, y) = (step.position().x, step.position().y);
                npc.dir = ScriptSkillService::direction_to(npc.x, npc.y, x, y, npc.dir);
                (npc.x, npc.y) = (x, y);
                if let Some(next) = walk.steps.last_mut() {
                    next.set_move_at(step.move_at() + Movement::delay(speed, next.is_diagonal()));
                }
            }
            if walk.steps.is_empty() {
                skip_legs_ending_at(&mut walk.legs, (npc.x, npc.y));
            }
            if walk.steps.is_empty() && !walk.legs.is_empty() {
                leg_ends.push(npc.id);
            } else if walk.steps.is_empty() {
                arrivals.extend(walk.on_arrival.map(|arrival| (arrival, npc.id)));
                npc.walk = None;
            }
        }
        for npc_id in leg_ends {
            self.start_next_npc_leg(state, npc_id, tick);
        }
        for (arrival, npc_id) in arrivals {
            self.queue_walk_arrival(Some(arrival), None, Some(npc_id));
        }
    }

    /// The NPC ended a leg: it walks the next one, or stands when the cells changed and nothing leads on.
    fn start_next_npc_leg(&self, state: &mut MapInstanceState, npc_id: u32, tick: u128) {
        let Some(mut npc) = state.script_skill_state.npcs.get(&npc_id).cloned() else { return };
        let Some(mut walk) = npc.walk.take() else { return };
        let Some((mut steps, legs)) = next_npc_leg(state, (npc.x, npc.y), std::mem::take(&mut walk.legs)) else {
            state.script_skill_state.npcs.insert(npc_id, npc.clone());
            self.notify_area(state, npc.x, npc.y, stop_packet(npc.id, npc.x, npc.y, self.configuration_service.packetver()));
            return;
        };
        if let Some(first) = steps.last_mut() {
            first.set_move_at(tick + Movement::delay(npc.snapshot().speed(), first.is_diagonal()));
        }
        walk.destination = steps.first().map_or(walk.destination, |end| (end.position().x, end.position().y));
        walk.steps = steps;
        walk.legs = legs;
        npc.walk = Some(walk);
        state.script_skill_state.npcs.insert(npc_id, npc.clone());
        if let Some(packet) = npc_walk_packet(&npc, self.configuration_service.packetver()) {
            self.notify_area(state, npc.x, npc.y, packet);
        }
    }

    /// `unitwalk` on a monster: it leaves its target and walks, and the AI waits until the walk ends
    /// ([`mob_walk_ended`](Self::mob_walk_ended)).
    pub(super) fn start_mob_walk(&self, state: &mut MapInstanceState, mob_id: u32, (x, y): (u16, u16), on_arrival: Option<ScriptNpcCallback>, tick: u128) {
        let Some(from) = state.get_mob(mob_id).filter(|mob| mob_can_walk(mob)).map(|mob| (mob.x, mob.y)) else { return };
        if from == (x, y) {
            self.queue_walk_arrival(on_arrival, None, Some(mob_id));
            return;
        }
        let mut steps = walk_path(state, from, (x, y));
        let map = state.key().clone();
        let Some(mob) = state.mobs_mut().get_mut(&mob_id) else { return };
        let Some(first) = steps.last_mut() else { return };
        first.set_move_at(tick + Movement::delay(mob.status.speed(), first.is_diagonal()));
        mob.set_movement(steps);
        mob.action = MobAction::Moving;
        mob.forced_walk = Some(ForcedWalk { map, destination: (x, y), on_arrival });
        self.notify_area(state, from.0, from.1, move_packet(mob_id, from, (x, y), self.configuration_service.packetver()));
    }

    pub(super) fn stop_mob_walk(&self, state: &mut MapInstanceState, mob_id: u32) {
        let Some(mob) = state.mobs_mut().get_mut(&mob_id) else { return };
        let walked = mob.forced_walk.take().is_some() || mob.is_moving();
        mob.clear_movement();
        mob.update_movement_complete();
        let (x, y) = (mob.x, mob.y);
        if walked {
            self.notify_area(state, x, y, stop_packet(mob_id, x, y, self.configuration_service.packetver()));
        }
    }

    /// Called by the movement thread when a monster's path runs out: the `unitwalk` label runs if it stopped on the destination.
    pub fn mob_walk_ended(&self, mob: &mut Mob, map: &crate::server::model::map_instance::MapInstanceKey) {
        if let Some(walk) = mob.forced_walk.take().filter(|walk| walk.arrived(map, (mob.x, mob.y))) {
            self.queue_walk_arrival(walk.on_arrival, None, Some(mob.id));
        }
    }

    /// Queues the arrival label of a walk, attached to `char_id` when a player walked; a walking monster or NPC is the label's
    /// argument 0.
    pub(crate) fn queue_walk_arrival(&self, on_arrival: Option<ScriptNpcCallback>, char_id: Option<u32>, walker: Option<u32>) {
        let Some(callback) = on_arrival else { return };
        self.server_task_queue.add_to_first_index(walk_arrival_event(callback, char_id, walker));
    }
}

fn walk_arrival_event(ScriptNpcCallback { npc_id, scope_instance, entry_id }: ScriptNpcCallback, char_id: Option<u32>, walker: Option<u32>) -> GameEvent {
    GameEvent::ScriptNpcEvent(ScriptNpcEvent {
        npc_id,
        scope_instance,
        entry_id,
        char_id,
        depth: 0,
        queued_until: 0,
        args: walker.map(|id| vec![script_sdk::Value::Number(id as i32)]),
        timer_guard: None,
    })
}

pub(crate) fn stop_packet(id: u32, x: u16, y: u16, packetver: u32) -> Vec<u8> {
    let mut packet = PacketZcStopmove::new(packetver);
    packet.set_aid(id);
    packet.set_x_pos(x as i16);
    packet.set_y_pos(y as i16);
    packet.fill_raw();
    packet.raw
}
