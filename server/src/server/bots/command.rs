//! Commands of bots, run by the game loop like the packets of a real client.
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use movement::path::path_search_client_side_algorithm;
use movement::position::Position;
use serde_json::{Value, json};
use tokio::sync::oneshot;

use super::combat::DuelRequest;
use super::commerce::StoreAction;
use super::companions::CompanionAction;
use super::interaction::{ChatAction, ItemAction, PartyAction, ProgressAction, SkillCast, StorageAction, TradeAction};
use super::menu::{CraftAction, SkillMenuAction};
use super::social::SocialRequest;
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
const PLAYERS_AT_PEACE: &str =
    "Players can only fight in a duel, on a PvP, GvG or battleground map, or at a castle while a siege is on";
const APPROACH_WIDENING: [u16; 3] = [1, 3, 6];
const APPROACH_CANDIDATES: usize = 12;

pub type BotReply = Arc<Mutex<Option<oneshot::Sender<Result<Value, String>>>>>;

#[derive(Debug, Clone, PartialEq)]
pub enum DialogInput {
    Next,
    Choose(u8),
    Number(i32),
    Text(String),
    /// Buy or sell at a shop.
    Deal(Deal),
    /// Item ids and amounts to buy.
    Buy(Vec<(u32, i16)>),
    /// Inventory indexes and amounts to sell.
    Sell(Vec<(usize, i16)>),
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Deal {
    Buy,
    Sell,
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
    /// How far a cell, or the nearest cell from where a target can be used, is on foot.
    Path(PathQuery),
    /// Uses `target_id` from where the bot stands: talks to an NPC, picks an item up or attacks a monster.
    Interact { target_id: u32 },
    /// One step of a fight against a monster, see `bot_attack`.
    Attack { target_id: u32 },
    Dialog(DialogInput),
    Item(ItemAction),
    Skill(SkillCast),
    Chat(ChatAction),
    Party(PartyAction),
    /// Sitting, emotes, guilds, friends, chat rooms and party bookings.
    Social(SocialRequest),
    Trade(TradeAction),
    Progress(ProgressAction),
    Storage(StorageAction),
    /// Carts, vending stores and buying stores.
    Store(StoreAction),
    /// The answer to a window a skill opened.
    SkillMenu(SkillMenuAction),
    Craft(CraftAction),
    Duel(DuelRequest),
    /// Pets, homunculi and mercenaries.
    Companion(CompanionAction),
    /// An `@` command, with its text.
    Command(String),
    Stop,
    Respawn,
    /// A click in a plugin window or an action a plugin offers; the plugin's run answers it.
    Plugin(PluginRequest),
}

#[derive(Debug, Clone)]
pub enum PluginRequest {
    Ui { plugin: String, event: plugin_ui::UiEvent },
    Action { plugin: String, name: String, args: String },
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
            BotRequest::Path(_) => "Path",
            BotRequest::Interact { .. } => "Interact",
            BotRequest::Attack { .. } => "Attack",
            BotRequest::Dialog(_) => "Dialog",
            BotRequest::Item(_) => "Item",
            BotRequest::Skill(_) => "Skill",
            BotRequest::Chat(_) => "Chat",
            BotRequest::Party(_) => "Party",
            BotRequest::Social(_) => "Social",
            BotRequest::Trade(_) => "Trade",
            BotRequest::Progress(_) => "Progress",
            BotRequest::Storage(_) => "Storage",
            BotRequest::Store(_) => "Store",
            BotRequest::SkillMenu(_) => "SkillMenu",
            BotRequest::Craft(_) => "Craft",
            BotRequest::Duel(_) => "Duel",
            BotRequest::Companion(_) => "Companion",
            BotRequest::Command(_) => "Command",
            BotRequest::Stop => "Stop",
            BotRequest::Respawn => "Respawn",
            BotRequest::Plugin(_) => "Plugin",
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
        if let BotRequest::Command(text) = &self.request {
            if let Ok(command) = server.bot_command_line(state, self.char_id, text) {
                if state.permission_groups().is_plugin_command(&command.name) {
                    server.plugin_command(state, self.char_id, &command.name, command.args, Some(self.reply));
                    return Ok(());
                }
            }
        }
        if let BotRequest::Plugin(request) = self.request {
            match request {
                PluginRequest::Ui { plugin, event } => server.plugin_ui_event(state, self.char_id, &plugin, event, Some(self.reply)),
                PluginRequest::Action { plugin, name, args } => server.plugin_bot_action(state, self.char_id, &plugin, name, args, self.reply),
            }
            return Ok(());
        }
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

/// What `path` measures the walk to: a cell, or a target (the cells it can be used from, as `use` and `attack` walk to).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathQuery {
    Cell { x: u16, y: u16 },
    Target(u32),
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
            BotRequest::Path(query) => self.bot_path(state, char_id, query),
            BotRequest::Interact { target_id } => self.bot_interact(state, command.account_id, char_id, *target_id),
            BotRequest::Attack { target_id } => self.bot_attack(state, char_id, *target_id),
            BotRequest::Dialog(input) => bot_dialog(state, command.account_id, char_id, input),
            BotRequest::Item(action) => self.bot_item(state, char_id, command.account_id, action),
            BotRequest::Skill(cast) => self.bot_skill(state, char_id, cast, tick),
            BotRequest::Chat(action) => self.bot_chat(state, char_id, action),
            BotRequest::Party(action) => self.bot_party(state, char_id, action),
            BotRequest::Social(request) => self.bot_social(state, char_id, request),
            BotRequest::Trade(action) => self.bot_trade(state, char_id, command.account_id, action),
            BotRequest::Progress(action) => self.bot_progress(state, char_id, action),
            BotRequest::Storage(action) => self.bot_storage(state, char_id, action),
            BotRequest::Store(action) => self.bot_store(state, char_id, action, tick),
            BotRequest::SkillMenu(action) => self.bot_skill_menu(state, command.account_id, char_id, action, tick),
            BotRequest::Craft(action) => self.bot_craft(state, char_id, action, tick),
            BotRequest::Duel(request) => self.bot_duel(state, char_id, request),
            BotRequest::Companion(action) => self.bot_companion(state, char_id, action, tick),
            BotRequest::Command(text) => self.bot_command(state, char_id, text),
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
            BotRequest::Plugin(_) => Err("Plugin requests are answered by the plugin".into()),
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
        let session = Arc::new(Session::create_for_bot(account_id, char_id, rand::random(), self.packetver(), self.configuration.bots.command_group));
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
        if let Some((waypoint, route_length)) = around_warps(state, character, (x, y), &|cell| cell == (x, y)) {
            self.add_to_next_movement_tick(GameEvent::CharacterRequestMove(CharacterRequestMove {
                char_id,
                destination: Position { x: waypoint.0, y: waypoint.1, dir: 0 },
            }));
            return Ok(json!({ "path_length": route_length, "waypoint": { "x": waypoint.0, "y": waypoint.1 } }));
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
            TargetKind::Player => (ATTACK_REACH, vec![ATTACK_REACH]),
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
        // Only the cells of the warp walked to may be stepped on
        let allowed = |(cx, cy): (u16, u16)| {
            target.kind == TargetKind::Warp && cx.abs_diff(target.x) <= target.half_width && cy.abs_diff(target.y) <= target.half_height
        };
        for cells in &candidates {
            for &(x, y) in cells.iter().take(APPROACH_CANDIDATES) {
                if (x, y) == here {
                    return Ok(json!({ "kind": target.kind.name(), "in_range": true }));
                }
                if let Some((waypoint, route_length)) = around_warps(state, character, (x, y), &allowed) {
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

    /// The length of the walk to a cell or a target, found over the whole map like a long walk, so the answer holds for walks
    /// the pathfinder takes in legs. A cell in another region, or behind a closed door, is `reachable: false`.
    fn bot_path(&self, state: &ServerState, char_id: u32, query: &PathQuery) -> Result<Value, String> {
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        let instance = state.get_map_instance_from_character(character).ok_or("The map of the character is not loaded yet")?;
        let (width, height) = (instance.x_size(), instance.y_size());
        let here = (character.x(), character.y());
        let regions = crate::server::model::map_regions::of_map(character.current_map_name());
        let region = |(x, y): (u16, u16)| regions.as_ref().and_then(|regions| regions.landing(x, y));
        let (kind, center, reach, warp) = match *query {
            PathQuery::Cell { x, y } => {
                if x >= width || y >= height {
                    return Err(format!("({x},{y}) is outside the map, which is {width}x{height} cells"));
                }
                ("cell", (x, y), 0, false)
            }
            PathQuery::Target(target_id) => {
                let target = locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?;
                let reach = match target.kind {
                    TargetKind::Npc => NPC_TALK_DISTANCE,
                    TargetKind::Item => ITEM_PICKUP_DISTANCE,
                    TargetKind::Mob | TargetKind::Player => ATTACK_REACH,
                    TargetKind::Warp => target.half_width.max(target.half_height),
                };
                (target.kind.name(), target.position(), reach, target.kind == TargetKind::Warp)
            }
        };
        // Read after `locate`, which takes the lock of the map state too
        let map_state = instance.state();
        // The map flags the cells that really trigger a warp, which is narrower than its rectangle
        let goal = |cell: (u16, u16)| distance(cell, center) <= reach && (!warp || map_state.is_warp_cell(cell.0, cell.1));
        let found = nearest_on_grid(map_state.cells(), width, height, here, &goal);
        let mut answer = path_answer(here, found, kind, &region, center);
        if found.is_none() && kind == "cell" && !is_walkable(map_state.cells(), width, center.0, center.1) {
            answer["reason"] = json!("not a walkable cell");
        }
        Ok(answer)
    }

    /// One step of a fight: attacks when the monster or player is within reach, walks towards it otherwise. A walk cancels the
    /// attack, so the client side loop of a real player (walk, then attack again) is the caller's.
    pub(crate) fn bot_attack(&self, state: &ServerState, char_id: u32, target_id: u32) -> Result<Value, String> {
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        require_active(character)?;
        match locate(state, character, target_id) {
            Some(Target { kind: kind @ (TargetKind::Mob | TargetKind::Player), x, y, .. }) => {
                if kind == TargetKind::Player {
                    if state.get_character(target_id).is_some_and(|player| player.is_dead() || player.status.hp == 0) {
                        return Ok(json!({ "gone": true }));
                    }
                    if !self.player_combat_target_allowed(state, character, target_id) {
                        return Err(PLAYERS_AT_PEACE.into());
                    }
                }
                if distance((character.x(), character.y()), (x, y)) <= ATTACK_REACH {
                    if !character.is_attacking() || character.attack().target != target_id {
                        self.add_to_next_tick(GameEvent::CharacterAttack(CharacterAttack { char_id, target_id, repeat: true }));
                    }
                    Ok(json!({ "in_range": true }))
                } else if character.is_moving() {
                    Ok(json!({ "in_range": false }))
                } else {
                    self.bot_approach(state, char_id, target_id)
                }
            }
            Some(_) => Err("Only monsters and players can be attacked".into()),
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
            TargetKind::Player => Err("Players are not used: attack them, trade with them or talk to them".into()),
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

/// The answer of `path`: where the bot is, the cell it would walk to and the length of the walk, with the regions of both.
fn path_answer(
    here: (u16, u16),
    found: Option<((u16, u16), usize)>,
    kind: &str,
    region: &dyn Fn((u16, u16)) -> Option<u32>,
    target: (u16, u16),
) -> Value {
    let to = found.map_or(target, |(cell, _)| cell);
    json!({
        "kind": kind,
        "from": { "x": here.0, "y": here.1, "region": region(here) },
        "to": { "x": to.0, "y": to.1, "region": region(to) },
        "reachable": found.is_some(),
        "path_length": found.map(|(_, length)| length),
    })
}

/// The nearest cell on foot for which `goal` holds, and the steps to it, by the moves of `route_on_grid`.
fn nearest_on_grid(cells: &[u16], width: u16, height: u16, from: (u16, u16), goal: &dyn Fn((u16, u16)) -> bool) -> Option<((u16, u16), usize)> {
    let index = |(x, y): (u16, u16)| usize::from(y) * usize::from(width) + usize::from(x);
    let walkable = |x: i32, y: i32| {
        (0..i32::from(width)).contains(&x) && (0..i32::from(height)).contains(&y) && is_walkable(cells, width, x as u16, y as u16)
    };
    let mut steps: Vec<Option<usize>> = vec![None; usize::from(width) * usize::from(height)];
    steps[index(from)] = Some(0);
    let mut queue = std::collections::VecDeque::from([from]);
    while let Some(current) = queue.pop_front() {
        let taken = steps[index(current)]?;
        if goal(current) {
            return Some((current, taken));
        }
        let (cx, cy) = (i32::from(current.0), i32::from(current.1));
        for (dx, dy) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
            let (nx, ny) = (cx + dx, cy + dy);
            let diagonal_blocked = dx != 0 && dy != 0 && !(walkable(cx + dx, cy) && walkable(cx, cy + dy));
            if !walkable(nx, ny) || diagonal_blocked {
                continue;
            }
            let next = (nx as u16, ny as u16);
            if steps[index(next)].is_none() {
                steps[index(next)] = Some(taken + 1);
                queue.push_back(next);
            }
        }
    }
    None
}

/// Steps of the way around warps tried as the next cell to walk to, longest first.
const AROUND_WARP_STEPS: [usize; 7] = [70, 40, 15, 8, 4, 2, 1];

/// A cell to walk to first, on a way to `to` that steps on no warp cell outside `allowed`, when the walk straight there
/// would step on one: the game loop sends a character through any warp cell it walks onto, and the pathfinder does not
/// avoid them (a walk out of a house door to the town gate went back in through the next door). The cell and the length
/// of the whole way. None when the walk straight there steps on no other warp, or when no way around exists.
fn around_warps(state: &ServerState, character: &Character, to: (u16, u16), allowed: &dyn Fn((u16, u16)) -> bool) -> Option<((u16, u16), usize)> {
    let instance = state.get_map_instance_from_character(character)?;
    let (width, height) = (instance.x_size(), instance.y_size());
    if to.0 >= width || to.1 >= height {
        return None;
    }
    let map_state = instance.state();
    way_around_warps(map_state.cells(), width, height, (character.x(), character.y()), to, allowed)
}

/// `around_warps` on the cells of a map.
fn way_around_warps(
    cells: &[u16],
    width: u16,
    height: u16,
    here: (u16, u16),
    to: (u16, u16),
    allowed: &dyn Fn((u16, u16)) -> bool,
) -> Option<((u16, u16), usize)> {
    let warp = CellType::Warp.as_flag();
    let index = |(x, y): (u16, u16)| usize::from(y) * usize::from(width) + usize::from(x);
    let blocked = |cell: (u16, u16)| cell != here && cells.get(index(cell)).is_some_and(|c| c & warp == warp) && !allowed(cell);
    let walk_steps_on_warp = |to: (u16, u16)| {
        let path = path_search_client_side_algorithm(width, height, cells, here.0, here.1, to.0, to.1);
        path.is_empty() || path.iter().any(|node| blocked((node.x, node.y)))
    };
    // A walk the pathfinder does not find (it gives up after about 100 cells) goes around the warps on the grid too:
    // it used to be walked in legs over the whole map (`long_route_waypoint`), through the door cells
    let direct = path_search_client_side_algorithm(width, height, cells, here.0, here.1, to.0, to.1);
    if !direct.is_empty() && !direct.iter().any(|node| blocked((node.x, node.y))) {
        return None;
    }
    let walkable = CellType::Walkable.as_flag();
    let masked: Vec<u16> = cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            let at = ((i % usize::from(width)) as u16, (i / usize::from(width)) as u16);
            if blocked(at) { cell & !walkable } else { *cell }
        })
        .collect();
    let route = route_on_grid(&masked, width, height, here, to)?;
    AROUND_WARP_STEPS.iter().find_map(|step| {
        let waypoint = *route.get((*step).min(route.len() - 1))?;
        (!walk_steps_on_warp(waypoint)).then_some((waypoint, route.len()))
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

pub(super) fn bot_session(state: &ServerState, account_id: u32, char_id: u32) -> Result<Arc<Session>, String> {
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
        DialogInput::Deal(Deal::Buy) => PlayerInput::DealType(0),
        DialogInput::Deal(Deal::Sell) => PlayerInput::DealType(1),
        DialogInput::Buy(lines) => PlayerInput::Purchases(lines.clone()),
        DialogInput::Sell(lines) => PlayerInput::Sales(lines.clone()),
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

#[cfg(test)]
mod walk_tests {
    use super::*;

    /// A 20x9 room, all walkable, with a door warp (cells 8..=10, 3..=5) in the middle of the straight line from
    /// (2,4) to (17,4).
    fn room() -> (Vec<u16>, u16, u16) {
        let (width, height) = (20u16, 9u16);
        let walkable = CellType::Walkable.as_flag();
        let warp = CellType::Warp.as_flag();
        let cells = (0..usize::from(width) * usize::from(height))
            .map(|i| {
                let (x, y) = (i % 20, i / 20);
                if (8..=10).contains(&x) && (3..=5).contains(&y) { walkable | warp } else { walkable }
            })
            .collect();
        (cells, width, height)
    }

    #[test]
    fn a_walk_goes_around_a_warp_it_is_not_walking_to() {
        let (cells, width, height) = room();
        let (waypoint, length) = way_around_warps(&cells, width, height, (2, 4), (17, 4), &|_| false).expect("the walk crosses the door");
        let path = path_search_client_side_algorithm(width, height, &cells, 2, 4, waypoint.0, waypoint.1);
        let warp = CellType::Warp.as_flag();
        assert!(!path.is_empty());
        assert!(path.iter().all(|node| cells[usize::from(node.y) * 20 + usize::from(node.x)] & warp == 0), "the first leg steps on no warp");
        assert!(length >= 15);
        // Walking to the door itself, or along a line that misses it, is left alone
        assert_eq!(way_around_warps(&cells, width, height, (2, 4), (9, 4), &|(x, y)| (8..=10).contains(&x) && (3..=5).contains(&y)), None);
        assert_eq!(way_around_warps(&cells, width, height, (2, 0), (17, 0), &|_| false), None);
    }

    #[test]
    fn a_long_walk_goes_around_a_warp_it_is_not_walking_to() {
        // 200 cells long, 3 high: the door takes the two top rows at x = 100, so the walk goes by the bottom row
        let (width, height) = (200u16, 3u16);
        let walkable = CellType::Walkable.as_flag();
        let warp = CellType::Warp.as_flag();
        let cells: Vec<u16> = (0..usize::from(width) * usize::from(height))
            .map(|i| {
                let (x, y) = (i % 200, i / 200);
                if x == 100 && y < 2 { walkable | warp } else { walkable }
            })
            .collect();
        let (waypoint, length) = way_around_warps(&cells, width, height, (2, 1), (197, 1), &|_| false).expect("the long walk goes around the door");
        assert!(waypoint.0 < 100, "the waypoint is before the door: {waypoint:?}");
        assert_eq!(length, 195, "the whole way round is walked");
        let path = path_search_client_side_algorithm(width, height, &cells, 2, 1, waypoint.0, waypoint.1);
        assert!(!path.is_empty() && path.iter().all(|node| cells[usize::from(node.y) * 200 + usize::from(node.x)] & warp == 0));
    }
}
