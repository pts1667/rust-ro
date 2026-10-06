use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::OnceLock;

use log::{error, info, warn};
use script_sdk::Value;
use serde::Deserialize;

use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CastleLifecycle, GameEvent};
use crate::server::model::events::map_event::{CastleCommand, GuardianSpawn, MapEvent, ScriptMapCommand, ScriptSpawn};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::state::server::ServerState;

pub(crate) const CD_GUILD_ID: u8 = 1;
pub(crate) const CD_CURRENT_ECONOMY: u8 = 2;
pub(crate) const CD_CURRENT_DEFENSE: u8 = 3;
pub(crate) const CD_INVESTED_ECONOMY: u8 = 4;
pub(crate) const CD_INVESTED_DEFENSE: u8 = 5;
pub(crate) const CD_ENABLED_KAFRA: u8 = 9;
pub(crate) const CD_ENABLED_GUARDIAN00: u8 = 10;
const GUARDIAN_SLOTS: u8 = 8;
const GD_GUARDRESEARCH: u32 = 10002;
const GD_GUARDUP: u32 = 10003;
const GD_DEVELOPMENT: u32 = 10014;
const EMPERIUM: i32 = 1288;
const GUARDIAN_CLASSES: [i32; 3] = [1285, 1286, 1287];
const BROADCAST_WOE_BLUE: &str = "0x00CCFF";
const CONQUEST_ANNOUNCE_DELAY_MS: u128 = 7_000;

static LAST_CLOCK_DAY: AtomicI64 = AtomicI64::new(i64::MIN);

#[derive(Debug, Deserialize)]
pub(crate) struct CastleSpawn {
    pub name: String,
    pub id: i32,
    pub count: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CastleSpawns {
    pub field: Vec<CastleSpawn>,
    pub emperium_room: Vec<CastleSpawn>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct GuardianSlot {
    #[serde(rename = "type")]
    pub kind: u8,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CastleTreasure {
    pub r#box: i32,
    pub cells: Vec<[u16; 2]>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct CastleFlagPlacement {
    pub map: String,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct CastleFlags {
    pub outside: Vec<CastleFlagPlacement>,
    pub inside: Vec<CastleFlagPlacement>,
}

impl CastleFlags {
    fn maps(&self) -> std::collections::BTreeSet<&str> {
        self.outside.iter().chain(&self.inside).map(|flag| flag.map.as_str()).collect()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct Castle {
    #[allow(dead_code)]
    pub id: u32,
    pub map: String,
    pub name: String,
    pub emperium: [u16; 2],
    pub spawns: CastleSpawns,
    pub guardians: Vec<GuardianSlot>,
    #[allow(dead_code)]
    pub master_room: [u16; 2],
    pub treasure: CastleTreasure,
    #[serde(default)]
    pub flags: CastleFlags,
}

pub(crate) fn castles() -> &'static [Castle] {
    static CASTLES: OnceLock<Vec<Castle>> = OnceLock::new();
    CASTLES.get_or_init(|| serde_json::from_str(include_str!("../script/castles.json")).expect("Embedded castle catalog is invalid"))
}

pub(crate) fn castle_by_map(map: &str) -> Option<&'static Castle> {
    let map = normalize_map(map);
    castles().iter().find(|castle| castle.map == map)
}

fn guardian_class(kind: u8) -> i32 {
    match kind {
        1 => 1287,
        2 => 1285,
        _ => 1286,
    }
}

fn spawn_request(mob_id: i32, x: u16, y: u16, name: &str, amount: u16) -> ScriptSpawn {
    ScriptSpawn {
        mob_id,
        x: i32::from(x),
        y: i32::from(y),
        name: name.to_string(),
        amount,
        event: String::new(),
        event_npc: None,
        size: None,
        ai: None,
        owner_id: 0,
        guardian: None,
        bg_id: 0,
        max_hp: None,
        lifetime_ms: None,
    reserved_id: None,
    }
}

impl Server {
    fn castle_value(&self, castle: &Castle, field: u8) -> i32 {
        self.repository.castle_value(&castle.map, field).unwrap_or(0)
    }

    fn castle_owner(&self, castle: &Castle) -> u32 {
        u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0)
    }

    /// Owning guild and castle names shown when a guardian or Emperium is targeted.
    pub(crate) fn guardian_label(&self, state: &ServerState, map_name: &str, instance_id: u8, mob_id: u32) -> Option<(String, String)> {
        let owner = state.get_map_instance(&map_name.to_string(), instance_id)?.state().get_mob(mob_id)?.castle_owner;
        if owner == 0 {
            return None;
        }
        let castle = castle_by_map(map_name)?;
        let guild = self.repository.guild(owner).ok().flatten()?;
        Some((guild.name, castle.name.clone()))
    }

    fn set_castle_value(&self, castle: &Castle, field: u8, value: i32) {
        if let Err(error) = self.repository.set_castle_value(&castle.map, field, value) {
            error!("Castle {} field {field} update failed: {error}", castle.map);
        }
    }

    fn guild_skill_level(&self, guild_id: u32, skill_id: u32) -> u8 {
        self.repository.guild(guild_id).ok().flatten().map_or(0, |guild| guild.skill_level(skill_id))
    }

    fn send_castle_command(&self, state: &mut ServerState, castle: &Castle, command: CastleCommand) {
        let instance = match state.get_map_instance(&castle.map, 0) {
            Some(instance) => instance,
            None => {
                let Some(definition) = GlobalConfigService::instance().find_map(&castle.map) else {
                    warn!("Castle map {} is not loaded", castle.map);
                    return;
                };
                self.server_service().create_map_instance(state, definition, 0)
            }
        };
        instance.add_to_next_tick(MapEvent::CastleCommand(command));
    }

    fn castle_announce(&self, state: &mut ServerState, map: Option<&str>, message: &str) {
        let flags = Value::Number(if map.is_some() { 1 } else { 0 });
        let arguments = [Value::String(message.to_string()), flags, Value::String(BROADCAST_WOE_BLUE.into()), Value::Number(400), Value::Number(12)];
        let packet = match super::script_presentation_service::announcement_packet(&arguments) {
            Ok(packet) => packet,
            Err(error) => {
                warn!("Castle announcement failed: {error}");
                return;
            }
        };
        let recipients: Vec<u32> = state
            .characters()
            .values()
            .filter(|character| map.is_none_or(|map| normalize_map(character.current_map_name()) == map))
            .map(|character| character.char_id)
            .collect();
        self.map_notifications
            .extend(recipients.into_iter().map(|char_id| Notification::Char(CharNotification::new(char_id, packet.clone()))));
        self.drain_map_notifications();
    }

    fn castle_friendly_guilds(&self, owner: u32) -> Vec<u32> {
        let mut guilds = vec![owner];
        if let Ok(Some(guild)) = self.repository.guild(owner) {
            guilds.extend(guild.allies);
        }
        guilds
    }

    fn respawn_castle_characters(&self, state: &mut ServerState, castle: &Castle, spared_guild: Option<u32>) {
        let evicted: Vec<(u32, String, u16, u16)> = state
            .characters()
            .values()
            .filter(|character| normalize_map(character.current_map_name()) == castle.map)
            .filter(|character| spared_guild.is_none_or(|guild| character.game_systems.guild_id != guild || guild == 0))
            .map(|character| (character.char_id, normalize_map(&character.save_map), character.save_x, character.save_y))
            .collect();
        for (char_id, map, x, y) in evicted {
            self.server_service().schedule_warp_to_walkable_cell(state, &map, x, y, char_id);
        }
    }

    fn start_castle_arena(&self, state: &mut ServerState, castle: &Castle) {
        let defense = self.castle_value(castle, CD_CURRENT_DEFENSE);
        let mut request = spawn_request(EMPERIUM, castle.emperium[0], castle.emperium[1], "Emperium", 1);
        request.guardian = Some(GuardianSpawn { defense, guard_upgrade: 0, emperium: true, friendly_guilds: Vec::new(), owner_guild: self.castle_owner(castle) });
        self.send_castle_command(state, castle, CastleCommand::SpawnUnlessPresent(request));
    }

    fn populate_empty_castle(&self, state: &mut ServerState, castle: &Castle) {
        self.send_castle_command(state, castle, CastleCommand::ClearMobs(None));
        for spawn in &castle.spawns.field {
            self.send_castle_command(state, castle, CastleCommand::Spawn(spawn_request(spawn.id, 0, 0, &spawn.name, spawn.count)));
        }
        for spawn in &castle.spawns.emperium_room {
            let request = spawn_request(spawn.id, castle.emperium[0], castle.emperium[1], &spawn.name, spawn.count);
            self.send_castle_command(state, castle, CastleCommand::Spawn(request));
        }
        self.start_castle_arena(state, castle);
    }

    fn spawn_castle_guardians(&self, state: &mut ServerState, castle: &Castle, owner: u32) {
        self.send_castle_command(state, castle, CastleCommand::ClearMobs(Some(GUARDIAN_CLASSES.to_vec())));
        let defense = self.castle_value(castle, CD_CURRENT_DEFENSE);
        let guard_upgrade = self.guild_skill_level(owner, GD_GUARDUP);
        let friendly = self.castle_friendly_guilds(owner);
        for slot in 0..castle.guardians.len().min(usize::from(GUARDIAN_SLOTS)) {
            if self.castle_value(castle, CD_ENABLED_GUARDIAN00 + slot as u8) != 0 {
                self.spawn_guardian_slot(state, castle, slot, defense, guard_upgrade, &friendly);
            }
        }
    }

    fn spawn_guardian_slot(&self, state: &mut ServerState, castle: &Castle, slot: usize, defense: i32, guard_upgrade: u8, friendly: &[u32]) {
        let guardian = &castle.guardians[slot];
        let mut request = spawn_request(guardian_class(guardian.kind), guardian.x, guardian.y, "", 1);
        request.guardian = Some(GuardianSpawn { defense, guard_upgrade, emperium: false, friendly_guilds: friendly.to_vec(), owner_guild: friendly.first().copied().unwrap_or(0) });
        self.send_castle_command(state, castle, CastleCommand::Spawn(request));
    }

    fn refresh_castle_flags(&self, state: &ServerState, castle: &Castle) {
        let guild_id = u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0);
        let version = if guild_id == 0 {
            0
        } else {
            self.repository.guild(guild_id).ok().flatten().map_or(0, |guild| guild.emblem_version as u16)
        };
        for map in castle.flags.maps() {
            if let Some(instance) = state.get_map_instance(&map.to_string(), 0) {
                instance.add_to_next_tick(MapEvent::ScriptMapCommand(ScriptMapCommand::FlagEmblem {
                    castle_map: castle.map.clone(),
                    guild_id,
                    version,
                }));
            }
        }
    }

    fn refresh_castle(&self, state: &mut ServerState, castle: &Castle) {
        self.refresh_castle_flags(state, castle);
        let owner = u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0);
        if owner == 0 {
            self.populate_empty_castle(state, castle);
        } else {
            self.spawn_castle_guardians(state, castle, owner);
        }
    }

    pub(crate) fn handle_castle_lifecycle(&self, state: &mut ServerState, event: CastleLifecycle, tick: u128) {
        match event {
            CastleLifecycle::Init => {
                LAST_CLOCK_DAY.store(current_day(), Ordering::Relaxed);
                for castle in castles() {
                    self.refresh_castle(state, castle);
                }
            }
            CastleLifecycle::AgitStart => {
                for castle in castles() {
                    let owner = u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0);
                    self.respawn_castle_characters(state, castle, Some(owner));
                    let kept: Vec<i32> = GUARDIAN_CLASSES.iter().copied().chain([EMPERIUM]).collect();
                    self.send_castle_command(state, castle, CastleCommand::ClearMobsExcept(kept));
                    self.start_castle_arena(state, castle);
                }
            }
            CastleLifecycle::AgitEnd => {
                for castle in castles() {
                    if self.castle_value(castle, CD_GUILD_ID) != 0 {
                        self.send_castle_command(state, castle, CastleCommand::ClearMobs(Some(vec![EMPERIUM])));
                    }
                }
            }
            CastleLifecycle::EmperiumBroken { map, guild_id } => self.castle_conquered(state, &map, guild_id),
            CastleLifecycle::GuildBroken { guild_id } => {
                for castle in castles() {
                    if self.castle_value(castle, CD_GUILD_ID) == guild_id as i32 {
                        self.send_castle_command(state, castle, CastleCommand::ClearMobs(Some(GUARDIAN_CLASSES.to_vec())));
                        self.set_castle_value(castle, CD_GUILD_ID, 0);
                        self.refresh_castle_flags(state, castle);
                        self.set_castle_value(castle, CD_ENABLED_KAFRA, 0);
                        self.add_to_delayed_tick(
                            GameEvent::CastleLifecycle(CastleLifecycle::Refresh { map: castle.map.clone(), abandoned: true }),
                            CONQUEST_ANNOUNCE_DELAY_MS,
                        );
                    }
                }
            }
            CastleLifecycle::Refresh { map, abandoned } => {
                let Some(castle) = castle_by_map(&map) else { return };
                if abandoned {
                    self.castle_announce(state, None, &format!("Guild Base [{}] has been abandoned.", castle.name));
                }
                self.refresh_castle(state, castle);
            }
            CastleLifecycle::AnnounceConquest { map, guild_id } => {
                let Some(castle) = castle_by_map(&map) else { return };
                let guild_name = self.repository.guild(guild_id).ok().flatten().map(|guild| guild.name).unwrap_or_default();
                self.castle_announce(state, None, &format!("The [{}] castle has been conquered by the [{guild_name}] guild.", castle.name));
            }
            CastleLifecycle::RestartArena { map } => {
                if state.siege_active() {
                    if let Some(castle) = castle_by_map(&map) {
                        self.start_castle_arena(state, castle);
                    }
                }
            }
            CastleLifecycle::SummonGuardian { map, slot } => {
                let Some(castle) = castle_by_map(&map) else { return };
                let owner = u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0);
                if owner == 0 || usize::from(slot) >= castle.guardians.len() {
                    return;
                }
                let defense = self.castle_value(castle, CD_CURRENT_DEFENSE);
                let friendly = self.castle_friendly_guilds(owner);
                self.spawn_guardian_slot(state, castle, usize::from(slot), defense, self.guild_skill_level(owner, GD_GUARDUP), &friendly);
            }
            CastleLifecycle::DailyTick => self.castle_daily_tick(state, tick),
        }
    }

    fn castle_conquered(&self, state: &mut ServerState, map: &str, guild_id: u32) {
        let Some(castle) = castle_by_map(map) else { return };
        if guild_id == 0 {
            warn!("A guildless character broke the Emperium in {}; the Emperium respawns", castle.map);
            self.start_castle_arena(state, castle);
            return;
        }
        for field in [CD_CURRENT_ECONOMY, CD_CURRENT_DEFENSE] {
            let reduced = (self.castle_value(castle, field) - 5).max(0);
            self.set_castle_value(castle, field, reduced);
        }
        self.set_castle_value(castle, CD_GUILD_ID, guild_id as i32);
        self.refresh_castle_flags(state, castle);
        self.castle_announce(state, Some(&castle.map), "The emperium has been destroyed.");
        self.respawn_castle_characters(state, castle, Some(guild_id));
        for field in CD_INVESTED_ECONOMY..=CD_ENABLED_KAFRA {
            self.set_castle_value(castle, field, 0);
        }
        if self.guild_skill_level(guild_id, GD_GUARDRESEARCH) == 0 {
            for slot in 0..GUARDIAN_SLOTS {
                self.set_castle_value(castle, CD_ENABLED_GUARDIAN00 + slot, 0);
            }
        }
        self.spawn_castle_guardians(state, castle, guild_id);
        self.add_to_delayed_tick(GameEvent::CastleLifecycle(CastleLifecycle::RestartArena { map: castle.map.clone() }), 500);
        self.add_to_delayed_tick(
            GameEvent::CastleLifecycle(CastleLifecycle::AnnounceConquest { map: castle.map.clone(), guild_id }),
            CONQUEST_ANNOUNCE_DELAY_MS,
        );
        info!("Guild {guild_id} now owns {}", castle.map);
    }

    fn castle_daily_tick(&self, state: &mut ServerState, _tick: u128) {
        for castle in castles() {
            let owner = u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0);
            if owner == 0 {
                continue;
            }
            let invested_economy = self.castle_value(castle, CD_INVESTED_ECONOMY);
            if invested_economy != 0 {
                let development = i32::from(fastrand::bool() && self.guild_skill_level(owner, GD_DEVELOPMENT) > 0);
                let economy = (self.castle_value(castle, CD_CURRENT_ECONOMY) + invested_economy + development).min(100);
                self.set_castle_value(castle, CD_CURRENT_ECONOMY, economy);
            }
            let invested_defense = self.castle_value(castle, CD_INVESTED_DEFENSE);
            if invested_defense != 0 {
                let defense = (self.castle_value(castle, CD_CURRENT_DEFENSE) + invested_defense).min(100);
                self.set_castle_value(castle, CD_CURRENT_DEFENSE, defense);
            }
            self.set_castle_value(castle, CD_INVESTED_ECONOMY, 0);
            self.set_castle_value(castle, CD_INVESTED_DEFENSE, 0);
            let chests = (self.castle_value(castle, CD_CURRENT_ECONOMY) / 5 + 4) as usize;
            for (index, cell) in castle.treasure.cells.iter().enumerate().take(chests) {
                let class = castle.treasure.r#box + (index as i32 % 2);
                let request = spawn_request(class, cell[0], cell[1], "Treasure Chest", 1);
                self.send_castle_command(state, castle, CastleCommand::SpawnAtEmptyCell(request));
            }
        }
    }

    pub(crate) fn castle_script_call(&self, char_id: u32, function: script_sdk::Function, arguments: &[Value]) -> Result<Value, String> {
        use script_sdk::Function;
        let number = |index: usize| -> Result<i32, String> { arguments.get(index).ok_or("Missing argument")?.number_value() };
        match function {
            Function::GetGuildInfo => {
                let guild = u32::try_from(number(0)?).map_err(|_| "Invalid guild")?;
                let Some(record) = self.repository.guild(guild).map_err(|error| error.to_string())? else {
                    return Ok(Value::String(String::new()));
                };
                Ok(match number(1)? {
                    0 => Value::String(record.name),
                    1 => Value::String(
                        self.repository
                            .guild_member_records(guild)
                            .map_err(|error| error.to_string())?
                            .into_iter()
                            .find(|member| member.char_id as u32 == record.master_char_id)
                            .map(|member| member.name)
                            .unwrap_or_default(),
                    ),
                    _ => Value::Number(i32::from(record.master_char_id == char_id)),
                })
            }
            Function::GetGuildSkillLevel => {
                let guild = u32::try_from(number(0)?).map_err(|_| "Invalid guild")?;
                let skill = u32::try_from(number(1)?).map_err(|_| "Invalid guild skill")?;
                Ok(Value::Number(i32::from(self.guild_skill_level(guild, skill))))
            }
            Function::GuardianSummon => {
                let map = arguments.first().ok_or("Missing castle map")?.string_value()?.to_string();
                let slot = u8::try_from(number(1)?).map_err(|_| "Invalid guardian slot")?;
                let castle = castle_by_map(&map).ok_or("Unknown castle")?;
                let owner = u32::try_from(self.castle_value(castle, CD_GUILD_ID)).unwrap_or(0);
                let is_owner_master = owner != 0
                    && self.repository.guild(owner).map_err(|error| error.to_string())?.is_some_and(|guild| guild.master_char_id == char_id);
                if !is_owner_master || usize::from(slot) >= castle.guardians.len() || self.guild_skill_level(owner, GD_GUARDRESEARCH) == 0 {
                    return Err("Only the owning guild master with Guardian Research can summon a guardian".into());
                }
                self.add_to_next_tick(GameEvent::CastleLifecycle(CastleLifecycle::SummonGuardian { map, slot }));
                Ok(Value::default())
            }
            _ => Err("Unsupported castle call".into()),
        }
    }

    /// Fires the daily castle economy and treasure update when the local date changes.
    pub(crate) fn castle_clock(&self) {
        let day = current_day();
        let previous = LAST_CLOCK_DAY.swap(day, Ordering::Relaxed);
        if previous != i64::MIN && previous != day {
            self.add_to_next_tick(GameEvent::CastleLifecycle(CastleLifecycle::DailyTick));
        }
    }
}

fn current_day() -> i64 {
    use chrono::Datelike;
    i64::from(chrono::Local::now().num_days_from_ce())
}
