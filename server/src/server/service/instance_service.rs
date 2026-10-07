//! Memorial dungeons: creation, entry, expiry, the client windows (packets 0x02CB-0x02CE) and the `instance_*` script commands.
//!
//! Maps of an instance are created together, as map instances sharing the instance id. Destroying an instance first
//! warps everybody out to their save point; the maps are dropped once they are empty.

use std::time::{SystemTime, UNIX_EPOCH};

use script_sdk::{Function, Reply, Value};
use tokio::sync::oneshot;

use crate::server::model::instance::{self, InstanceMode, MemorialInstance};
use crate::server::model::map::Map;
use crate::server::script::ScriptRequest;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;
use crate::server::Server;

const CREATE_PACKET: u16 = 0x02cb;
const CHANGE_WAIT_PACKET: u16 = 0x02cc;
const STATUS_PACKET: u16 = 0x02cd;
const NOTIFY_PACKET: u16 = 0x02ce;
const NAME_BYTES: usize = 61;
const NO_MAP_POSITION: u16 = 0xffff;

pub(crate) const IE_OK: i32 = 0;
pub(crate) const IE_NOMEMBER: i32 = 1;
pub(crate) const IE_NOINSTANCE: i32 = 2;
pub(crate) const IE_OTHER: i32 = 3;

const NOTIFY_ACTIVE: u32 = 0;
const NOTIFY_LIVE_TIMEOUT: u32 = 1;
const NOTIFY_ENTER_TIMEOUT: u32 = 2;
const NOTIFY_USER_REQUEST: u32 = 3;

const FORCE_CLOSE_AFTER_SECONDS: u64 = 60;
const NOT_DEAD_FLAG: i32 = 1;

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::InstanceDestroy
            | Function::InstanceEnter
            | Function::InstanceNpcName
            | Function::InstanceMapName
            | Function::InstanceId
            | Function::InstanceWarpAll
            | Function::InstanceAnnounce
            | Function::InstanceCheckParty
            | Function::InstanceCheckGuild
            | Function::InstanceInfo
            | Function::InstanceLiveInfo
            | Function::InstanceList
            | Function::GetInstanceVar
            | Function::SetInstanceVar
            | Function::GetPartyName
    )
}

fn now_seconds() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_secs())
}

fn name_field(name: &str) -> [u8; NAME_BYTES] {
    let mut field = [0u8; NAME_BYTES];
    let length = name.len().min(NAME_BYTES - 1);
    field[..length].copy_from_slice(&name.as_bytes()[..length]);
    field
}

/// `ZC_MEMORIALDUNGEON_SUBSCRIPTION_INFO`: the instance is booked, `position` in the creation queue.
fn create_packet(name: &str, position: u16) -> Vec<u8> {
    let mut packet = CREATE_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&name_field(name));
    packet.extend_from_slice(&position.to_le_bytes());
    packet
}

/// `ZC_MEMORIALDUNGEON_CHANGEWAIT`: new position in the creation queue, `NO_MAP_POSITION` when the maps could not be created.
fn change_wait_packet(position: u16) -> Vec<u8> {
    let mut packet = CHANGE_WAIT_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&position.to_le_bytes());
    packet
}

/// `ZC_MEMORIALDUNGEON_INFO`: life time and idle time limits as unix times, 0 for none.
fn status_packet(name: &str, keep_limit: u32, idle_limit: u32) -> Vec<u8> {
    let mut packet = STATUS_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&name_field(name));
    packet.extend_from_slice(&keep_limit.to_le_bytes());
    packet.extend_from_slice(&idle_limit.to_le_bytes());
    packet
}

/// `ZC_MEMORIALDUNGEON_NOTIFY`: 0 changed idle limit, 1 and 2 expired, 3 destroyed by its owner, 4 creation failed.
fn notify_packet(kind: u32, limit: u32) -> Vec<u8> {
    let mut packet = NOTIFY_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&kind.to_le_bytes());
    packet.extend_from_slice(&limit.to_le_bytes());
    packet
}

fn status_packet_of(instance: &MemorialInstance) -> Vec<u8> {
    status_packet(&instance.definition.name, instance.keep_deadline.unwrap_or(0) as u32, instance.idle_deadline.unwrap_or(0) as u32)
}

/// Online characters who are part of the instance's owner group.
fn recipients(state: &ServerState, instance: &MemorialInstance) -> Vec<u32> {
    let owner = instance.owner_id;
    state
        .characters()
        .values()
        .filter(|character| match instance.mode {
            InstanceMode::Char => character.char_id == owner,
            InstanceMode::Party => owner != 0 && character.game_systems.party_id == owner,
            InstanceMode::Guild => owner != 0 && character.game_systems.guild_id == owner,
            InstanceMode::None | InstanceMode::Clan => false,
        })
        .map(|character| character.char_id)
        .collect()
}

fn on_instance_map(character: &crate::server::state::character::Character, instance: &MemorialInstance) -> bool {
    character.map_instance_key.map_instance() == instance.id && instance.maps.iter().any(|map| *map == character.map_instance_key.map_without_ext())
}

fn belongs_to(character: &crate::server::state::character::Character, instance: &MemorialInstance) -> bool {
    match instance.mode {
        InstanceMode::None => true,
        InstanceMode::Char => character.char_id == instance.owner_id,
        InstanceMode::Party => character.game_systems.party_id == instance.owner_id && instance.owner_id != 0,
        InstanceMode::Guild => character.game_systems.guild_id == instance.owner_id && instance.owner_id != 0,
        InstanceMode::Clan => false,
    }
}

/// Instance id a map name stands for: `name#id` as `instance_mapname` writes it, or the caller's own instance.
pub(crate) fn split_instance_map(name: &str) -> (&str, Option<u8>) {
    match name.rsplit_once('#') {
        Some((map, id)) if !map.is_empty() && !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()) => (map, id.parse().ok()),
        _ => (name, None),
    }
}

impl Server {
    /// Map and instance a script names: `map#id` is explicit, a bare memorial map means the copy of the calling NPC's instance.
    /// `None` leaves the choice to the caller's own rule.
    pub(crate) fn resolve_script_map(&self, scope_instance: u8, name: &str) -> (String, Option<u8>) {
        let (map, explicit) = split_instance_map(name);
        let map = Map::name_without_ext(map).to_ascii_lowercase();
        let instance = explicit.or_else(|| (scope_instance != 0 && self.instances().of_map(&map, scope_instance).is_some()).then_some(scope_instance));
        (map, instance)
    }

    fn free_instance_id(&self, state: &ServerState) -> Option<u8> {
        (1..=u8::MAX).find(|id| !self.instances().contains(*id) && !state.map_instances().values().flatten().any(|map| map.id() == *id))
    }

    /// Books the instance and returns it, or the reply the script gets at once (an error, or a negative `instance_create` code).
    fn reserve_instance(&self, state: &ServerState, context: &ScriptRequest, arguments: &[Value]) -> Result<MemorialInstance, Reply> {
        let name = arguments.first().ok_or_else(|| Err("Missing argument".to_string()))?.text();
        let mode = match arguments.get(1) {
            Some(mode) => InstanceMode::from_number(mode.number_value().map_err(Err)?).ok_or_else(|| Err("Unknown instance mode".to_string()))?,
            None => InstanceMode::Party,
        };
        let character = state.characters().get(&context.char_id);
        let owner_id = match arguments.get(2) {
            Some(owner) => owner.number_value().map_err(Err)? as u32,
            None => match mode {
                InstanceMode::None => context.npc_id,
                InstanceMode::Char => character.map_or(0, |character| character.char_id),
                InstanceMode::Party => character.map_or(0, |character| character.game_systems.party_id),
                InstanceMode::Guild => character.map_or(0, |character| character.game_systems.guild_id),
                InstanceMode::Clan => 0,
            },
        };
        let refused = |code: i32| Err(Ok(Value::Number(code)));
        let Some(definition) = instance::definition_by_name(&name) else {
            error!("instance_create: unknown instance {name}");
            return refused(-1);
        };
        let owner_known = match mode {
            InstanceMode::None => true,
            InstanceMode::Char => state.characters().contains_key(&owner_id),
            InstanceMode::Party => owner_id != 0 && state.characters().values().any(|character| character.game_systems.party_id == owner_id),
            InstanceMode::Guild => owner_id != 0 && state.characters().values().any(|character| character.game_systems.guild_id == owner_id),
            InstanceMode::Clan => false,
        };
        if !owner_known {
            error!("instance_create: owner {owner_id} of {name} was not found");
            return refused(-2);
        }
        if mode != InstanceMode::None && self.instances().of_owner(mode, owner_id).is_some() {
            return refused(-3);
        }
        let configuration = GlobalConfigService::instance();
        if let Some(missing) = definition.all_maps().find(|map| configuration.find_map(map).is_none()) {
            error!("instance_create: map {missing} of {name} is not loaded");
            return refused(-4);
        }
        let Some(id) = self.free_instance_id(state) else { return refused(-4) };
        Ok(MemorialInstance {
            id,
            definition,
            mode,
            owner_id,
            keep_deadline: None,
            idle_deadline: None,
            closing_since: None,
            maps: Vec::new(),
            item_ranges: Vec::new(),
            building: true,
        })
    }

    /// `instance_create`: books the instance at once and reads its map caches off the game loop; the script is answered when the maps exist.
    pub(crate) fn instance_create_deferred(&self, state: &mut ServerState, context: &ScriptRequest, arguments: &[Value], response: oneshot::Sender<Reply>) {
        let instance = match self.reserve_instance(state, context, arguments) {
            Ok(instance) => instance,
            Err(reply) => {
                let _ = response.send(reply);
                return;
            }
        };
        let (id, definition) = (instance.id, instance.definition);
        let members = recipients(state, &instance);
        let position = self.instances().begin_build(instance, response);
        for char_id in members {
            self.send_raw(char_id, create_packet(&definition.name, position as u16));
        }
        let instances = self.instances().clone();
        let tasks = self.game_tasks();
        let map_names: Vec<String> = definition.all_maps().cloned().collect();
        self.runtime().spawn_blocking(move || {
            let loaded = map_names
                .iter()
                .map(|name| map_cache::read_mcache(std::path::Path::new(unsafe { crate::MAP_DIR }), name).map(|cache| cache.cells).map_err(|error| format!("{name}: {error}")))
                .collect();
            instances.store_cells(id, loaded);
            tasks.add_to_first_index(crate::server::model::events::game_event::GameEvent::InstanceMapsLoaded(crate::server::model::events::game_event::InstanceMapsLoaded { id }));
        });
    }

    /// Game loop side of the creation: builds the maps from the cells that were read and answers the waiting script.
    pub(crate) fn instance_finish_build(&self, state: &mut ServerState, id: u8) {
        let Some(build) = self.instances().take_build(id) else { return };
        let respond = |reply: Reply| {
            if let Some(response) = build.response {
                let _ = response.send(reply);
            }
        };
        let Some(instance) = self.instances().get(id) else {
            respond(Ok(Value::Number(-4)));
            return;
        };
        let definition = instance.definition;
        let loaded = match build.cells {
            Some(Ok(cells)) => cells,
            Some(Err(error)) => {
                error!("[Instance] {}: maps could not be read: {error}", definition.name);
                Vec::new()
            }
            None => Vec::new(),
        };
        let configuration = GlobalConfigService::instance();
        let maps: Vec<_> = definition.all_maps().filter_map(|name| configuration.find_map(name)).collect();
        if loaded.len() != maps.len() || maps.len() != definition.all_maps().count() {
            for char_id in recipients(state, &instance) {
                self.send_raw(char_id, change_wait_packet(NO_MAP_POSITION));
            }
            self.instances().remove(id);
            self.send_queue_positions(state);
            respond(Ok(Value::Number(-4)));
            return;
        }
        let mut item_ranges = Vec::new();
        for (map, cells) in maps.into_iter().zip(loaded) {
            item_ranges.push(self.server_service().create_map_instance_with(state, map, id, true, Some(cells)).item_range());
        }
        let now = now_seconds();
        self.instances().update(id, |instance| {
            instance.maps = definition.all_maps().cloned().collect();
            instance.item_ranges = item_ranges;
            instance.keep_deadline = (definition.time_limit > 0).then(|| now + definition.time_limit);
            instance.idle_deadline = (definition.idle_timeout > 0).then(|| now + definition.idle_timeout);
            instance.building = false;
        });
        if let Some(ready) = self.instances().get(id) {
            for char_id in recipients(state, &ready) {
                self.send_raw(char_id, status_packet_of(&ready));
            }
        }
        self.send_queue_positions(state);
        info!("[Instance] Created: {} ({id})", definition.name);
        respond(Ok(Value::Number(i32::from(id))));
    }

    fn send_queue_positions(&self, state: &ServerState) {
        for (index, id) in self.instances().build_queue().into_iter().enumerate() {
            let Some(instance) = self.instances().get(id) else { continue };
            for char_id in recipients(state, &instance) {
                self.send_raw(char_id, change_wait_packet(index as u16 + 1));
            }
        }
    }

    pub(crate) fn instance_enter(&self, state: &mut ServerState, char_id: u32, name: &str, position: Option<(u16, u16)>, instance_id: Option<u8>) -> i32 {
        let Some(definition) = instance::definition_by_name(name) else {
            error!("instance_enter: unknown instance {name}");
            return IE_OTHER;
        };
        let Some(character) = state.characters().get(&char_id) else { return IE_OTHER };
        let (party_id, guild_id) = (character.game_systems.party_id, character.game_systems.guild_id);
        let instance = match instance_id {
            Some(id) => match self.instances().get(id) {
                Some(instance) => instance,
                None => return IE_NOINSTANCE,
            },
            None => {
                if party_id == 0 {
                    return IE_NOMEMBER;
                }
                match self.instances().of_owner(InstanceMode::Party, party_id).and_then(|id| self.instances().get(id)) {
                    Some(instance) => instance,
                    None => return IE_NOINSTANCE,
                }
            }
        };
        match instance.mode {
            InstanceMode::Char if instance.owner_id != char_id => return IE_OTHER,
            InstanceMode::Party if party_id == 0 => return IE_NOMEMBER,
            InstanceMode::Party if party_id != instance.owner_id => return IE_OTHER,
            InstanceMode::Guild if guild_id == 0 => return IE_NOMEMBER,
            InstanceMode::Guild if guild_id != instance.owner_id => return IE_OTHER,
            InstanceMode::Clan => return IE_NOMEMBER,
            _ => {}
        }
        if instance.definition.id != definition.id || instance.closing_since.is_some() || instance.building {
            return IE_OTHER;
        }
        let (x, y) = position.unwrap_or((definition.enter_x, definition.enter_y));
        self.server_service().schedule_warp_to_walkable_cell_in_instance(state, &definition.enter_map, x, y, char_id, instance.id);
        IE_OK
    }

    /// Starts the destruction of an instance: nobody can enter any more and everybody inside goes to their save point.
    pub(crate) fn instance_destroy(&self, state: &mut ServerState, id: u8) -> bool {
        let Some(instance) = self.instances().get(id) else { return false };
        if instance.building {
            return false;
        }
        if instance.closing_since.is_some() {
            return true;
        }
        let now = now_seconds();
        let kind = if instance.keep_deadline.is_some_and(|deadline| deadline <= now) {
            NOTIFY_LIVE_TIMEOUT
        } else if instance.idle_deadline.is_some_and(|deadline| deadline <= now) {
            NOTIFY_ENTER_TIMEOUT
        } else {
            NOTIFY_USER_REQUEST
        };
        self.instances().update(id, |instance| {
            instance.closing_since = Some(now);
            instance.keep_deadline = None;
            instance.idle_deadline = None;
        });
        for char_id in recipients(state, &instance) {
            self.send_raw(char_id, notify_packet(kind, 0));
        }
        let occupants: Vec<(u32, String, u16, u16)> = state
            .characters()
            .values()
            .filter(|character| on_instance_map(character, &instance))
            .map(|character| (character.char_id, Map::name_without_ext(&character.save_map), character.save_x, character.save_y))
            .collect();
        for (char_id, map, x, y) in occupants {
            self.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, char_id);
        }
        true
    }

    fn remove_instance_maps(&self, state: &mut ServerState, instance: &MemorialInstance) {
        for map_name in &instance.maps {
            let removed = state.map_instances_mut().get_mut(map_name).and_then(|copies| {
                let position = copies.iter().position(|copy| copy.id() == instance.id)?;
                Some(copies.remove(position))
            });
            if let Some(map_instance) = removed {
                map_instance.shutdown();
                state.release_item_range(map_instance.item_range());
                state.map_instances_count().fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
        state.script_timers.npc.retain(|key, _| key.scope_instance != instance.id);
        state.script_timers.attachments.retain(|key, _| key.1 != instance.id);
        self.script_service().npc_variables.lock().unwrap().retain(|key, _| !(key.0 == 1 && key.1 == instance.id || key.0 == 4 && key.2 == u32::from(instance.id)));
    }

    fn finish_instance_destroy(&self, state: &mut ServerState, instance: &MemorialInstance) {
        self.remove_instance_maps(state, instance);
        self.instances().remove(instance.id);
        info!("[Instance] Destroyed: {} ({})", instance.definition.name, instance.id);
    }

    /// Expires instances, notices when they empty or fill up and finishes the ones being destroyed.
    pub(crate) fn tick_instances(&self, state: &mut ServerState) {
        let now = now_seconds();
        if !self.instances().due(now) {
            return;
        }
        for instance in self.instances().snapshot() {
            if instance.building {
                continue;
            }
            if let Some(since) = instance.closing_since {
                let occupied = state.characters().values().any(|character| on_instance_map(character, &instance));
                if !occupied || now.saturating_sub(since) >= FORCE_CLOSE_AFTER_SECONDS {
                    self.finish_instance_destroy(state, &instance);
                }
                continue;
            }
            if instance.keep_deadline.is_some_and(|deadline| deadline <= now) || instance.idle_deadline.is_some_and(|deadline| deadline <= now) {
                self.instance_destroy(state, instance.id);
                continue;
            }
            let occupied = state.characters().values().any(|character| on_instance_map(character, &instance));
            if occupied && instance.idle_deadline.is_some() {
                self.instances().update(instance.id, |instance| instance.idle_deadline = None);
                for char_id in recipients(state, &instance) {
                    self.send_raw(char_id, notify_packet(NOTIFY_ACTIVE, 0));
                }
            } else if !occupied && instance.idle_deadline.is_none() && instance.definition.idle_timeout > 0 {
                let deadline = now + instance.definition.idle_timeout;
                self.instances().update(instance.id, |instance| instance.idle_deadline = Some(deadline));
                if let Some(updated) = self.instances().get(instance.id) {
                    for char_id in recipients(state, &updated) {
                        self.send_raw(char_id, status_packet_of(&updated));
                    }
                }
            }
        }
    }

    /// The window of the memorial dungeons the character belongs to, on every map load.
    pub(crate) fn instance_login(&self, state: &ServerState, char_id: u32) {
        let Some(character) = state.characters().get(&char_id) else { return };
        for instance in self.instances().snapshot() {
            if instance.closing_since.is_none() && !instance.building && instance.mode != InstanceMode::None && belongs_to(character, &instance) {
                self.send_raw(char_id, status_packet_of(&instance));
            }
        }
    }

    /// The "destroy" button of the window (`CZ_MEMORIALDUNGEON_COMMAND`): the owner of an instance closes it.
    pub(crate) fn instance_destroy_command(&self, state: &mut ServerState, char_id: u32) {
        let Some(character) = state.characters().get(&char_id) else { return };
        if character.is_dead() || character.map_instance_key.map_instance() != 0 {
            return;
        }
        let candidate = self.instances().snapshot().into_iter().find(|instance| {
            instance.closing_since.is_none()
                && match instance.mode {
                    InstanceMode::Char => instance.owner_id == char_id,
                    InstanceMode::Party => {
                        character.game_systems.party_id == instance.owner_id
                            && character.game_systems.party.as_ref().is_some_and(|party| party.id == instance.owner_id && party.leader_char_id == char_id)
                    }
                    InstanceMode::Guild => {
                        character.game_systems.guild_id == instance.owner_id
                            && self.repository.guild(instance.owner_id).ok().flatten().is_some_and(|guild| guild.master_char_id == char_id)
                    }
                    InstanceMode::None | InstanceMode::Clan => false,
                }
        });
        if let Some(instance) = candidate.filter(|instance| instance.definition.destroyable) {
            self.instance_destroy(state, instance.id);
        }
    }

    /// Instance of the NPC that runs the script, 0 when it is not part of one.
    fn script_instance_id(&self, context: &ScriptRequest) -> u8 {
        if self.instances().contains(context.npc_scope_instance) { context.npc_scope_instance } else { 0 }
    }

    fn instance_of_mode(&self, state: &ServerState, char_id: u32, mode: InstanceMode) -> u8 {
        let Some(character) = state.characters().get(&char_id) else { return 0 };
        let owner = match mode {
            InstanceMode::Char => character.char_id,
            InstanceMode::Party => character.game_systems.party_id,
            InstanceMode::Guild => character.game_systems.guild_id,
            InstanceMode::None | InstanceMode::Clan => return 0,
        };
        if owner == 0 { 0 } else { self.instances().of_owner(mode, owner).unwrap_or(0) }
    }

    pub(crate) fn script_instance_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        let optional = |index: usize| -> Result<Option<i32>, String> { arguments.get(index).map(Value::number_value).transpose() };
        let text = |index: usize| -> Result<String, String> { Ok(arguments.get(index).ok_or("Missing argument")?.text()) };
        let attached = context.char_id;
        let own_instance = self.script_instance_id(context);
        let resolve_id = |explicit: Option<i32>, default: u8| -> u8 { explicit.and_then(|id| u8::try_from(id).ok()).filter(|id| *id != 0).unwrap_or(default) };
        match function {
            Function::InstanceDestroy => {
                let id = resolve_id(optional(0)?, own_instance);
                if id == 0 || !self.instance_destroy(state, id) {
                    return Err("instance_destroy: no instance to destroy".into());
                }
                Ok(Value::default())
            }
            Function::InstanceEnter => {
                let name = text(0)?;
                let position = match (optional(1)?, optional(2)?) {
                    (Some(x), Some(y)) if x >= 0 && y >= 0 => Some((u16::try_from(x).map_err(|_| "Invalid x")?, u16::try_from(y).map_err(|_| "Invalid y")?)),
                    _ => None,
                };
                let char_id = optional(3)?.filter(|id| *id > 0).map_or(attached, |id| id as u32);
                let explicit = optional(4)?.filter(|id| *id > 0).map(|id| u8::try_from(id).map_err(|_| "Invalid instance")).transpose()?;
                Ok(Value::Number(self.instance_enter(state, char_id, &name, position, explicit)))
            }
            Function::InstanceNpcName => {
                let id = resolve_id(optional(1)?, own_instance);
                if id == 0 || !self.instances().contains(id) {
                    return Err(format!("instance_npcname: invalid instance for NPC {}", text(0)?));
                }
                Ok(Value::String(format!("{}_{id}", text(0)?)))
            }
            Function::InstanceMapName => {
                let id = resolve_id(optional(1)?, own_instance);
                let map = Map::name_without_ext(&text(0)?);
                Ok(Value::String(if self.instances().of_map(&map, id).is_some() { format!("{map}#{id}") } else { String::new() }))
            }
            Function::InstanceId => Ok(Value::Number(match optional(0)? {
                None => i32::from(own_instance),
                Some(mode) => i32::from(self.instance_of_mode(state, attached, InstanceMode::from_number(mode).ok_or("Unknown instance mode")?)),
            })),
            Function::InstanceWarpAll => {
                let id = resolve_id(optional(3)?, self.instance_of_mode(state, attached, InstanceMode::Party));
                let instance = self.instances().get(id).ok_or("instance_warpall: instance is not found")?;
                let target = text(0)?;
                let (map, explicit) = split_instance_map(&target);
                let map = Map::name_without_ext(map);
                let target_instance = explicit.unwrap_or(id);
                if self.instances().of_map(&map, target_instance).is_none() {
                    return Err("instance_warpall: instance map for the instance is not found".into());
                }
                let (x, y) = (u16::try_from(number(1)?).map_err(|_| "Invalid x")?, u16::try_from(number(2)?).map_err(|_| "Invalid y")?);
                let flags = optional(4)?.unwrap_or(0);
                let members: Vec<u32> = state
                    .characters()
                    .values()
                    .filter(|character| on_instance_map(character, &instance) && belongs_to(character, &instance) && !(flags & NOT_DEAD_FLAG != 0 && character.is_dead()))
                    .map(|character| character.char_id)
                    .collect();
                for char_id in members {
                    self.server_service().schedule_warp_to_walkable_cell_in_instance(state, &map, x, y, char_id, target_instance);
                }
                Ok(Value::default())
            }
            Function::InstanceAnnounce => {
                let id = resolve_id(optional(0)?, own_instance);
                let instance = self.instances().get(id).ok_or("instance_announce: instance not found")?;
                let packet = super::script_presentation_service::announcement_packet(&arguments[1..])?;
                let members: Vec<u32> = state.characters().values().filter(|character| on_instance_map(character, &instance)).map(|character| character.char_id).collect();
                for char_id in members {
                    self.send_raw(char_id, packet.clone());
                }
                Ok(Value::default())
            }
            Function::InstanceCheckParty | Function::InstanceCheckGuild => {
                let owner = number(0)? as u32;
                let max_level = GlobalConfigService::instance().config().game.max_base_level as i32;
                let (amount, minimum, maximum) = (optional(1)?.unwrap_or(1), optional(2)?.unwrap_or(1), optional(3)?.unwrap_or(max_level));
                if amount < 1 || !(1..=max_level).contains(&minimum) || !(1..=max_level).contains(&maximum) {
                    return Err("instance_check: invalid amount or level".into());
                }
                let party = function == Function::InstanceCheckParty;
                let members: Vec<i32> = state
                    .characters()
                    .values()
                    .filter(|character| owner != 0 && if party { character.game_systems.party_id } else { character.game_systems.guild_id } == owner)
                    .map(|character| character.status.base_level as i32)
                    .collect();
                if members.is_empty() && party {
                    return Ok(Value::Number(0));
                }
                let within = members.iter().all(|level| (minimum..=maximum).contains(level));
                Ok(Value::Number(i32::from(within && members.len() as i32 >= amount)))
            }
            Function::InstanceInfo => {
                let Some(definition) = instance::definition_by_name(&text(0)?) else { return Ok(Value::Number(-1)) };
                Ok(match number(1)? {
                    0 => Value::Number(definition.id as i32),
                    1 => Value::Number(definition.time_limit as i32),
                    2 => Value::Number(definition.idle_timeout as i32),
                    3 => Value::String(definition.enter_map.clone()),
                    4 => Value::Number(i32::from(definition.enter_x)),
                    5 => Value::Number(i32::from(definition.enter_y)),
                    6 => Value::Number(definition.all_maps().count() as i32),
                    7 => Value::String(definition.all_maps().nth(optional(2)?.unwrap_or(0).max(0) as usize).cloned().unwrap_or_default()),
                    _ => Value::Number(-1),
                })
            }
            Function::InstanceLiveInfo => {
                let id = resolve_id(optional(1)?, own_instance);
                let Some(instance) = self.instances().get(id) else {
                    return Ok(if number(0)? == 0 { Value::String(String::new()) } else { Value::Number(-1) });
                };
                Ok(match number(0)? {
                    0 => Value::String(instance.definition.name.clone()),
                    1 => Value::Number(instance.mode.number()),
                    2 => Value::Number(instance.owner_id as i32),
                    _ => Value::Number(-1),
                })
            }
            Function::InstanceList => {
                let map = Map::name_without_ext(&text(0)?);
                let mode = optional(1)?.map(|mode| InstanceMode::from_number(mode).ok_or("Unknown instance mode")).transpose()?;
                Ok(Value::Array(
                    self.instances
                        .snapshot()
                        .into_iter()
                        .filter(|instance| instance.closing_since.is_none() && instance.maps.iter().any(|name| *name == map) && mode.is_none_or(|mode| mode == instance.mode))
                        .map(|instance| Value::Number(i32::from(instance.id)))
                        .collect(),
                ))
            }
            Function::GetInstanceVar | Function::SetInstanceVar => {
                let name = text(0)?;
                let name = name.strip_prefix('\'').unwrap_or(&name).to_string();
                let (id, index) = if function == Function::GetInstanceVar { (optional(1)?, optional(2)?) } else { (optional(2)?, optional(3)?) };
                let id = resolve_id(id, own_instance);
                if !self.instances().contains(id) {
                    return Err("Instance variable needs an instance".into());
                }
                let key = (4, 0, u32::from(id), name.clone(), index.unwrap_or(0) as u32);
                let mut variables = self.script_service().npc_variables.lock().unwrap();
                if function == Function::GetInstanceVar {
                    return Ok(variables.get(&key).cloned().unwrap_or_else(|| if name.ends_with('$') { Value::String(String::new()) } else { Value::Number(0) }));
                }
                let value = arguments.get(1).cloned().ok_or("Missing value")?;
                if name.ends_with('$') != value.is_string() {
                    return Err("Invalid instance variable".into());
                }
                variables.insert(key, value);
                Ok(Value::default())
            }
            Function::GetPartyName => {
                let party = number(0)? as u32;
                Ok(Value::String(
                    state
                        .characters()
                        .values()
                        .find(|character| party != 0 && character.game_systems.party_id == party)
                        .map(|character| character.game_systems.party_name.clone())
                        .unwrap_or_default(),
                ))
            }
            _ => Err(format!("{function:?} is not an instance command")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packets_have_the_client_layout() {
        assert_eq!(create_packet("Orc's Memory", 1).len(), 65);
        assert_eq!(&create_packet("A", 3)[63..], &[3, 0]);
        let status = status_packet("A", 7, 9);
        assert_eq!((status.len(), &status[..2], &status[63..71]), (71, &[0xcd, 0x02][..], &[7, 0, 0, 0, 9, 0, 0, 0][..]));
        assert_eq!(change_wait_packet(0xffff), vec![0xcc, 0x02, 0xff, 0xff]);
        assert_eq!(notify_packet(3, 0), vec![0xce, 0x02, 3, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn instance_maps_are_named_with_their_instance() {
        assert_eq!(split_instance_map("1@tower#12"), ("1@tower", Some(12)));
        assert_eq!(split_instance_map("prontera"), ("prontera", None));
        assert_eq!(split_instance_map("#odd"), ("#odd", None));
    }
}
