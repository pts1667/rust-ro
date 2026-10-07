//! Commands of bots, run by the game loop like the packets of a real client.
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use movement::path::path_search_client_side_algorithm;
use movement::position::Position;
use serde_json::{Value, json};
use tokio::sync::oneshot;

use super::interaction::{ChatAction, ItemAction, PartyAction, ProgressAction, SkillCast, TradeAction};
use super::observation::{self, is_walkable};
use crate::server::Server;
use crate::server::model::character_lifecycle::{CharacterMapReady, CharacterRespawn, SelectedCharacter};
use crate::server::model::events::game_event::{
    CharacterAttack, CharacterPickUpItem, CharacterRequestMove, GameEvent, GameEventHandler, NpcContact,
};
use crate::server::model::movement::Movable;
use crate::server::model::script_timer::ScriptTimerOwner;
use crate::server::model::session::Session;
use crate::server::script::PlayerInput;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// A real client answers this long after the character was admitted, once its map is loaded.
const NPC_TALK_DISTANCE: u16 = 2;
/// Items are picked up from `2` cells away, walking to the next cell keeps the walk short.
const ITEM_PICKUP_DISTANCE: u16 = 2;
const ITEM_APPROACH_DISTANCE: u16 = 1;
/// Cells from which a novice's melee attack lands; the server accepts adjacent diagonals.
const ATTACK_REACH: u16 = 1;
const APPROACH_WIDENING: [u16; 3] = [1, 3, 6];
const APPROACH_CANDIDATES: usize = 12;

pub type BotReply = Arc<Mutex<Option<oneshot::Sender<Result<Value, String>>>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum DialogInput {
    Next,
    Choose(u8),
    Number(i32),
    Text(String),
    Close,
}

#[derive(Clone)]
pub enum BotRequest {
    Connect(Arc<Mutex<Option<SelectedCharacter>>>),
    Disconnect,
    Observe,
    /// The character alone, to follow a walk without the cost of a full observation.
    Status,
    /// The walkable cells, around `(x, y, radius)` or of the whole map.
    Map(Option<(u16, u16, u16)>),
    Move { x: u16, y: u16 },
    /// Starts walking to where `target_id` can be used.
    Approach { target_id: u32 },
    /// Uses `target_id` from where the bot stands: talks to an NPC, picks an item up or attacks a monster.
    Interact { target_id: u32 },
    /// One step of a fight against a monster, see `bot_attack`.
    Attack { target_id: u32 },
    Dialog(DialogInput),
    Item(ItemAction),
    Skill(SkillCast),
    Chat(ChatAction),
    Party(PartyAction),
    Trade(TradeAction),
    Progress(ProgressAction),
    Stop,
    Respawn,
}

impl BotRequest {
    fn name(&self) -> &'static str {
        match self {
            BotRequest::Connect(_) => "Connect",
            BotRequest::Disconnect => "Disconnect",
            BotRequest::Observe => "Observe",
            BotRequest::Status => "Status",
            BotRequest::Map(_) => "Map",
            BotRequest::Move { .. } => "Move",
            BotRequest::Approach { .. } => "Approach",
            BotRequest::Interact { .. } => "Interact",
            BotRequest::Attack { .. } => "Attack",
            BotRequest::Dialog(_) => "Dialog",
            BotRequest::Item(_) => "Item",
            BotRequest::Skill(_) => "Skill",
            BotRequest::Chat(_) => "Chat",
            BotRequest::Party(_) => "Party",
            BotRequest::Trade(_) => "Trade",
            BotRequest::Progress(_) => "Progress",
            BotRequest::Stop => "Stop",
            BotRequest::Respawn => "Respawn",
        }
    }
}

#[derive(Clone)]
pub struct BotCommand {
    pub char_id: u32,
    pub account_id: u32,
    pub request: BotRequest,
    pub reply: BotReply,
}

impl BotCommand {
    pub fn new(char_id: u32, account_id: u32, request: BotRequest) -> (Self, oneshot::Receiver<Result<Value, String>>) {
        let (sender, receiver) = oneshot::channel();
        (Self { char_id, account_id, request, reply: Arc::new(Mutex::new(Some(sender))) }, receiver)
    }
}

impl Debug for BotCommand {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("BotCommand").field("char_id", &self.char_id).field("request", &self.request.name()).finish()
    }
}

impl PartialEq for BotCommand {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.reply, &other.reply)
    }
}

impl GameEventHandler for BotCommand {
    /// The game loop drops the event, and so the reply channel, when the character left the game.
    fn required_character(&self) -> Option<u32> {
        (!matches!(self.request, BotRequest::Connect(_))).then_some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let reply = self.reply.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take();
        let result = server.run_bot_request(state, &self, tick);
        if let Some(reply) = reply {
            let _ = reply.send(result);
        }
        Ok(())
    }
}

/// What a bot can reach by id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetKind {
    Npc,
    Item,
    Mob,
    Player,
    Warp,
}

impl TargetKind {
    fn name(self) -> &'static str {
        match self {
            TargetKind::Npc => "npc",
            TargetKind::Item => "item",
            TargetKind::Mob => "mob",
            TargetKind::Player => "player",
            TargetKind::Warp => "warp",
        }
    }
}

pub(super) struct Target {
    kind: TargetKind,
    x: u16,
    y: u16,
    half_width: u16,
    half_height: u16,
}

impl Target {
    fn at(kind: TargetKind, x: u16, y: u16) -> Self {
        Self { kind, x, y, half_width: 0, half_height: 0 }
    }

    pub(super) fn position(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub(super) fn is_player(&self) -> bool {
        self.kind == TargetKind::Player
    }
}

pub(super) fn distance(from: (u16, u16), to: (u16, u16)) -> u16 {
    from.0.abs_diff(to.0).max(from.1.abs_diff(to.1))
}

pub(super) fn locate(state: &ServerState, character: &Character, id: u32) -> Option<Target> {
    let instance = state.get_map_instance_from_character(character)?;
    let map_state = instance.state();
    if let Some(item) = map_state.get_dropped_item(id) {
        return Some(Target::at(TargetKind::Item, item.x(), item.y()));
    }
    if let Some(mob) = map_state.get_mob(id) {
        return Some(Target::at(TargetKind::Mob, mob.x(), mob.y()));
    }
    if let Some(npc) = map_state.script_skill_state.npcs.get(&id).filter(|npc| !npc.hidden) {
        return Some(Target::at(TargetKind::Npc, npc.x, npc.y));
    }
    if let Some(warp) = instance.get_warp(id) {
        return Some(Target { half_width: warp.x_size, half_height: warp.y_size, ..Target::at(TargetKind::Warp, warp.x, warp.y) });
    }
    state
        .get_character(id)
        .filter(|other| other.map_instance_key == character.map_instance_key)
        .map(|other| Target::at(TargetKind::Player, other.x(), other.y()))
}

pub(super) fn require_active(character: &Character) -> Result<(), String> {
    if character.is_dead() || character.status.hp == 0 {
        return Err("The character is dead, respawn first".into());
    }
    Ok(())
}

impl Server {
    pub(crate) fn run_bot_request(&self, state: &mut ServerState, command: &BotCommand, tick: u128) -> Result<Value, String> {
        let char_id = command.char_id;
        match &command.request {
            BotRequest::Connect(selected) => self.connect_bot(state, command.account_id, selected),
            BotRequest::Disconnect => {
                self.begin_character_logout(state, char_id, false, tick);
                Ok(json!({ "disconnected": true }))
            }
            BotRequest::Observe => to_json(observation::observe(self, state, char_id, tick)?),
            BotRequest::Status => to_json(observation::status(self, state, char_id, tick)?),
            BotRequest::Map(center) => to_json(observation::map_grid(state, char_id, *center)?),
            BotRequest::Move { x, y } => self.bot_move(state, char_id, *x, *y),
            BotRequest::Approach { target_id } => self.bot_approach(state, char_id, *target_id),
            BotRequest::Interact { target_id } => self.bot_interact(state, command.account_id, char_id, *target_id),
            BotRequest::Attack { target_id } => self.bot_attack(state, char_id, *target_id),
            BotRequest::Dialog(input) => bot_dialog(state, command.account_id, char_id, input),
            BotRequest::Item(action) => self.bot_item(state, char_id, command.account_id, action),
            BotRequest::Skill(cast) => self.bot_skill(state, char_id, cast),
            BotRequest::Chat(action) => self.bot_chat(state, char_id, action),
            BotRequest::Party(action) => self.bot_party(state, char_id, action),
            BotRequest::Trade(action) => self.bot_trade(state, char_id, command.account_id, action),
            BotRequest::Progress(action) => self.bot_progress(state, char_id, action),
            BotRequest::Stop => {
                let character = state.characters_mut().get_mut(&char_id).ok_or("Character is not in game")?;
                self.character_service().cancel_movement(character, tick);
                character.clear_attack();
                Ok(json!({ "stopped": true }))
            }
            BotRequest::Respawn => {
                let session = bot_session(state, command.account_id, char_id)?;
                self.add_to_next_tick(GameEvent::CharacterRespawn(CharacterRespawn { session }));
                Ok(json!({ "respawning": true }))
            }
        }
    }

    /// Puts the character in the game without a client: the packets for it are read by the bot registry instead of sent to a socket.
    fn connect_bot(&self, state: &mut ServerState, account_id: u32, selected: &Mutex<Option<SelectedCharacter>>) -> Result<Value, String> {
        let selected = selected.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take().ok_or("The character was already admitted")?;
        let char_id = selected.char_id;
        if state.characters().contains_key(&char_id) || state.pending_character_logouts.contains_key(&char_id) {
            return Err("The bot is already in the game or is still leaving it".into());
        }
        if state.characters().values().any(|character| character.account_id == account_id) {
            return Err("The account of the bot already has a character in the game".into());
        }
        let mut character = selected.into_character();
        crate::server::service::map_position_service::restore_login_position(state, &mut character)?;
        let session = Arc::new(Session::create_for_bot(account_id, char_id, rand::random(), self.packetver()));
        state
            .character_logins
            .insert(char_id, ScriptTimerOwner { char_id, account_id, session: session.clone() });
        state.add_session(account_id, session.clone());
        state.insert_character(character);
        self.prepare_character_map(state, CharacterMapReady { session });
        Ok(json!({ "connecting": true }))
    }

    /// Walks to a cell. The pathfinder gives up on long routes, those are walked in legs: the answer then names the `waypoint`
    /// the bot walks to first, and the caller asks again from there.
    fn bot_move(&self, state: &ServerState, char_id: u32, x: u16, y: u16) -> Result<Value, String> {
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        require_active(character)?;
        if (character.x(), character.y()) == (x, y) {
            return Ok(json!({ "path_length": 0 }));
        }
        let (destination, reply) = match path_length(state, character, x, y) {
            Ok(path_length) => ((x, y), json!({ "path_length": path_length })),
            Err(error) => {
                let (waypoint, route_length) = long_route_waypoint(state, character, x, y).ok_or(error)?;
                (waypoint, json!({ "path_length": route_length, "waypoint": { "x": waypoint.0, "y": waypoint.1 } }))
            }
        };
        self.add_to_next_movement_tick(GameEvent::CharacterRequestMove(CharacterRequestMove {
            char_id,
            destination: Position { x: destination.0, y: destination.1, dir: 0 },
        }));
        Ok(reply)
    }

    fn bot_approach(&self, state: &ServerState, char_id: u32, target_id: u32) -> Result<Value, String> {
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        require_active(character)?;
        let target = locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?;
        let here = (character.x(), character.y());
        let (reach, ring) = match target.kind {
            TargetKind::Npc => (NPC_TALK_DISTANCE, APPROACH_WIDENING.iter().map(|widening| NPC_TALK_DISTANCE * widening).collect()),
            TargetKind::Item => (ITEM_PICKUP_DISTANCE, vec![ITEM_APPROACH_DISTANCE]),
            TargetKind::Warp => (0, vec![target.half_width.max(target.half_height)]),
            TargetKind::Mob => (ATTACK_REACH, vec![ATTACK_REACH]),
            TargetKind::Player => return Err("Players can not be targeted".into()),
        };
        if target.kind != TargetKind::Warp && distance(here, (target.x, target.y)) <= reach {
            return Ok(json!({ "kind": target.kind.name(), "in_range": true }));
        }
        let instance = state.get_map_instance_from_character(character).ok_or("The map of the character is not loaded yet")?;
        let (width, height) = (instance.x_size(), instance.y_size());
        // Collected first: path_length takes the lock of the map state again
        let candidates: Vec<Vec<(u16, u16)>> = {
            let map_state = instance.state();
            ring.into_iter()
                .map(|ring| {
                    let mut cells: Vec<(u16, u16)> = (target.y.saturating_sub(ring)..=target.y.saturating_add(ring).min(height - 1))
                        .flat_map(|y| (target.x.saturating_sub(ring)..=target.x.saturating_add(ring).min(width - 1)).map(move |x| (x, y)))
                        .filter(|&(x, y)| {
                            // The map flags the cells that really trigger the warp, which is narrower than its rectangle
                            (target.kind != TargetKind::Warp || map_state.is_warp_cell(x, y)) && is_walkable(map_state.cells(), width, x, y)
                        })
                        .collect();
                    cells.sort_by_key(|&cell| (distance(here, cell), distance((target.x, target.y), cell)));
                    cells
                })
                .collect()
        };
        for cells in &candidates {
            for &(x, y) in cells.iter().take(APPROACH_CANDIDATES) {
                if (x, y) == here {
                    return Ok(json!({ "kind": target.kind.name(), "in_range": true }));
                }
                if let Ok(path_length) = path_length(state, character, x, y) {
                    self.add_to_next_movement_tick(GameEvent::CharacterRequestMove(CharacterRequestMove {
                        char_id,
                        destination: Position { x, y, dir: 0 },
                    }));
                    return Ok(json!({
                        "kind": target.kind.name(),
                        "in_range": false,
                        "destination": { "x": x, "y": y },
                        "path_length": path_length,
                    }));
                }
            }
        }
        // Too far for the pathfinder: walk a leg of the route, the caller approaches again from there
        for &(x, y) in candidates.iter().filter_map(|cells| cells.first()) {
            if let Some((waypoint, route_length)) = long_route_waypoint(state, character, x, y) {
                self.add_to_next_movement_tick(GameEvent::CharacterRequestMove(CharacterRequestMove {
                    char_id,
                    destination: Position { x: waypoint.0, y: waypoint.1, dir: 0 },
                }));
                return Ok(json!({
                    "kind": target.kind.name(),
                    "in_range": false,
                    "destination": { "x": waypoint.0, "y": waypoint.1 },
                    "path_length": route_length,
                }));
            }
        }
        Err(format!("No reachable cell from where target {target_id} can be used"))
    }

    /// One step of a fight: attacks when the monster is within reach, walks towards it otherwise. A walk cancels the attack, so
    /// the client side loop of a real player (walk, then attack again) is the caller's.
    fn bot_attack(&self, state: &ServerState, char_id: u32, target_id: u32) -> Result<Value, String> {
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        require_active(character)?;
        match locate(state, character, target_id) {
            Some(Target { kind: TargetKind::Mob, x, y, .. }) => {
                if distance((character.x(), character.y()), (x, y)) <= ATTACK_REACH {
                    if !character.is_attacking() {
                        self.add_to_next_tick(GameEvent::CharacterAttack(CharacterAttack { char_id, target_id, repeat: true }));
                    }
                    Ok(json!({ "in_range": true }))
                } else if character.is_moving() {
                    Ok(json!({ "in_range": false }))
                } else {
                    self.bot_approach(state, char_id, target_id)
                }
            }
            Some(_) => Err("Only monsters can be attacked".into()),
            None => Ok(json!({ "gone": true })),
        }
    }

    fn bot_interact(&self, state: &ServerState, account_id: u32, char_id: u32, target_id: u32) -> Result<Value, String> {
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        require_active(character)?;
        let target = locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?;
        let here = (character.x(), character.y());
        match target.kind {
            TargetKind::Npc => {
                self.start_npc_conversation(state, NpcContact { char_id, account_id, npc_id: target_id })?;
                Ok(json!({ "action": "talk" }))
            }
            TargetKind::Item => {
                if distance(here, (target.x, target.y)) > ITEM_PICKUP_DISTANCE {
                    return Err("The item is too far, walk closer first".into());
                }
                self.add_to_next_tick(GameEvent::CharacterPickUpItem(CharacterPickUpItem { char_id, map_item_id: target_id }));
                Ok(json!({ "action": "pick_up" }))
            }
            TargetKind::Mob => {
                self.add_to_next_tick(GameEvent::CharacterAttack(CharacterAttack { char_id, target_id, repeat: true }));
                Ok(json!({ "action": "attack" }))
            }
            TargetKind::Warp => Err("Warps trigger when the character walks onto them".into()),
            TargetKind::Player => Err("Attacking players is not supported".into()),
        }
    }
}

fn to_json(value: impl serde::Serialize) -> Result<Value, String> {
    serde_json::to_value(value).map_err(|error| error.to_string())
}

fn path_length(state: &ServerState, character: &Character, x: u16, y: u16) -> Result<usize, String> {
    let instance = state.get_map_instance_from_character(character).ok_or("The map of the character is not loaded yet")?;
    let (width, height) = (instance.x_size(), instance.y_size());
    if x >= width || y >= height {
        return Err(format!("({x},{y}) is outside the map, which is {width}x{height} cells"));
    }
    let map_state = instance.state();
    if !is_walkable(map_state.cells(), width, x, y) {
        return Err(format!("({x},{y}) is not a walkable cell"));
    }
    let path = path_search_client_side_algorithm(width, height, map_state.cells(), character.x(), character.y(), x, y);
    if path.is_empty() {
        return Err(format!("({x},{y}) can not be reached from ({},{})", character.x(), character.y()));
    }
    Ok(path.len())
}

/// Cells ahead on the route (found on the whole map) that the pathfinder still reaches, longest first.
const WAYPOINT_STEPS: [usize; 3] = [70, 40, 15];

/// The next waypoint and the length of the whole route to `(x, y)`, when there is a route on foot.
fn long_route_waypoint(state: &ServerState, character: &Character, x: u16, y: u16) -> Option<((u16, u16), usize)> {
    let instance = state.get_map_instance_from_character(character)?;
    let (width, height) = (instance.x_size(), instance.y_size());
    let route = {
        let map_state = instance.state();
        if x >= width || y >= height || !is_walkable(map_state.cells(), width, x, y) {
            return None;
        }
        route_on_grid(map_state.cells(), width, height, (character.x(), character.y()), (x, y))?
    };
    WAYPOINT_STEPS.iter().find_map(|step| {
        let waypoint = *route.get((*step).min(route.len() - 1))?;
        path_length(state, character, waypoint.0, waypoint.1).ok().map(|_| (waypoint, route.len()))
    })
}

/// Breadth first search over the walkable cells, diagonals included but not around a corner. Cells after `from`, up to `to`.
fn route_on_grid(cells: &[u16], width: u16, height: u16, from: (u16, u16), to: (u16, u16)) -> Option<Vec<(u16, u16)>> {
    let index = |(x, y): (u16, u16)| usize::from(y) * usize::from(width) + usize::from(x);
    let walkable = |x: i32, y: i32| {
        (0..i32::from(width)).contains(&x) && (0..i32::from(height)).contains(&y) && is_walkable(cells, width, x as u16, y as u16)
    };
    let mut parent: Vec<Option<(u16, u16)>> = vec![None; usize::from(width) * usize::from(height)];
    parent[index(from)] = Some(from);
    let mut queue = std::collections::VecDeque::from([from]);
    while let Some(current) = queue.pop_front() {
        if current == to {
            break;
        }
        let (cx, cy) = (i32::from(current.0), i32::from(current.1));
        for (dx, dy) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
            let (nx, ny) = (cx + dx, cy + dy);
            let diagonal_blocked = dx != 0 && dy != 0 && !(walkable(cx + dx, cy) && walkable(cx, cy + dy));
            if !walkable(nx, ny) || diagonal_blocked {
                continue;
            }
            let next = (nx as u16, ny as u16);
            if parent[index(next)].is_none() {
                parent[index(next)] = Some(current);
                queue.push_back(next);
            }
        }
    }
    parent[index(to)]?;
    let mut route = vec![to];
    while let Some(previous) = parent[index(*route.last()?)].filter(|previous| *previous != from) {
        route.push(previous);
    }
    route.reverse();
    Some(route)
}

fn bot_session(state: &ServerState, account_id: u32, char_id: u32) -> Result<Arc<Session>, String> {
    state
        .find_session(account_id)
        .filter(|session| session.char_id == Some(char_id))
        .ok_or_else(|| "The bot has no session".to_string())
}

fn bot_dialog(state: &ServerState, account_id: u32, char_id: u32, input: &DialogInput) -> Result<Value, String> {
    let session = bot_session(state, account_id, char_id)?;
    let input = match input {
        DialogInput::Close => {
            return if session.close_dialog() {
                Ok(json!({ "closed": true }))
            } else {
                Err("No conversation is open".into())
            };
        }
        DialogInput::Next => PlayerInput::Next,
        DialogInput::Choose(option) => PlayerInput::Selection(*option),
        DialogInput::Number(number) => PlayerInput::Number(*number),
        DialogInput::Text(text) => PlayerInput::Text(text.clone()),
    };
    let sender = session
        .script_handler_channel_sender
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
        .ok_or("No conversation is waiting for an answer")?;
    sender.try_send(input).map_err(|error| format!("The conversation did not take the answer: {error}"))?;
    Ok(json!({ "sent": true }))
}
