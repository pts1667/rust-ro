use super::{MapEventContext, MapEventHandler};

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptSpawn {
    pub mob_id: i32,
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub amount: u16,
    pub event: String,
    pub event_npc: Option<ScriptNpcCallback>,
    pub size: Option<u8>,
    pub ai: Option<u16>,
    pub owner_id: u32,
    pub guardian: Option<GuardianSpawn>,
    /// Spawned by `guardian`: `maprespawnguildid` leaves it standing.
    pub is_guardian: bool,
    pub bg_id: u32,
    pub max_hp: Option<u32>,
    pub lifetime_ms: Option<u32>,
    pub reserved_id: Option<u32>,
    /// Opposite corner of the rectangle `(x, y)` starts (`areamonster`).
    pub area_end: Option<(i32, i32)>,
}

/// Script-driven changes to the monsters spawned with a callback event; `None` targets every monster.
#[derive(Debug, PartialEq, Clone)]
pub enum ScriptMobCommand {
    Kill { event_entry: Option<u32> },
    /// Removes script monsters the way `maprespawnguildid` does, sparing guardians, Emperium and clones unless asked.
    RemoveRespawnable { remove_clones: bool },
    SetDamageImmunity { event_entry: u32, immune: bool },
    SetTeam { mob_id: u32, bg_id: u32 },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CellArea {
    pub x1: u16,
    pub y1: u16,
    pub x2: u16,
    pub y2: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScriptMapCommand {
    NpcVisibility {
        npc_id: u32,
        visible: bool,
    },
    SetCell {
        area: CellArea,
        cell: models::enums::cell::CellType,
        enabled: bool,
    },
    FlagEmblem {
        castle_map: String,
        guild_id: u32,
        version: u16,
    },
    NpcEmblem {
        npc_id: u32,
        guild_id: u32,
        version: u16,
    },
    NpcRemove {
        npc_id: u32,
    },
    NpcMove {
        npc_id: u32,
        x: u16,
        y: u16,
        dir: Option<u16>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum CastleCommand {
    /// Silently removes the listed monster classes, or every monster when `None`.
    ClearMobs(Option<Vec<i32>>),
    /// Silently removes every monster whose class is not listed.
    ClearMobsExcept(Vec<i32>),
    Spawn(ScriptSpawn),
    /// Spawns unless a living monster of the same class already stands on that exact cell.
    SpawnAtEmptyCell(ScriptSpawn),
    /// Spawns unless a living monster of the same class already stands on the map.
    SpawnUnlessPresent(ScriptSpawn),
}

#[derive(Debug, PartialEq, Clone)]
pub struct GuardianSpawn {
    pub defense: i32,
    pub guard_upgrade: u8,
    pub emperium: bool,
    pub friendly_guilds: Vec<u32>,
    pub owner_guild: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ScriptNpcCallback {
    pub npc_id: u32,
    pub scope_instance: u8,
    pub entry_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReleaseScriptNpc {
    pub id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptDropItem {
    pub owner_id: u32,
    pub item_id: i32,
    pub amount: i16,
    pub x: u16,
    pub y: u16,
}

impl MapEventHandler for crate::server::script::unit_data::MapUnitDataRequest {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.unit_data(ctx.map_instance.state_mut().as_mut(), request);
    }
}

impl MapEventHandler for crate::server::script::unit_data::ScriptNpcTransfer {
    fn handle(self, ctx: &MapEventContext) {
        let transfer = self;
        ctx.service.install_script_npc(ctx.map_instance.state_mut().as_mut(), transfer);
    }
}

impl MapEventHandler for ReleaseScriptNpc {
    fn handle(self, ctx: &MapEventContext) {
        let ReleaseScriptNpc { id } = self;
        ctx.map_instance.state_mut().script_skill_state.transferring_npcs.remove(&id);
    }
}

impl MapEventHandler for ScriptDropItem {
    fn handle(self, ctx: &MapEventContext) {
        let ScriptDropItem {
            owner_id,
            item_id,
            amount,
            x,
            y,
        } = self;
        ctx.service
            .script_drop_item(ctx.map_instance.state_mut().as_mut(), owner_id, item_id, amount, x, y);
    }
}

impl MapEventHandler for ScriptSpawn {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        if let Err(error) = ctx.service.script_spawn(ctx.map_instance.state_mut().as_mut(), request) {
            error!("Script monster spawn failed on {}: {}", ctx.map_instance.name(), error);
        }
    }
}

impl MapEventHandler for CastleCommand {
    fn handle(self, ctx: &MapEventContext) {
        let command = self;
        ctx.service.castle_command(ctx.map_instance.state_mut().as_mut(), command);
    }
}

impl MapEventHandler for ScriptMobCommand {
    fn handle(self, ctx: &MapEventContext) {
        let command = self;
        ctx.service.script_mob_command(ctx.map_instance.state_mut().as_mut(), command);
    }
}

impl MapEventHandler for ScriptMapCommand {
    fn handle(self, ctx: &MapEventContext) {
        let command = self;
        ctx.service.script_map_command(ctx.map_instance.state_mut().as_mut(), command);
    }
}
