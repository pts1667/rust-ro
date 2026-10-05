use models::status_change::{StatusChangeKind, StatusChangeRequest};

use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::map_item::{MapItem, MapItemSnapshot};
use crate::server::service::script_combat_service::MobCombatEffect;

#[derive(Debug, PartialEq, Clone)]
pub enum MapEvent {
    GroundTrapCapture(crate::server::script::skill::trap::GroundTrapCapture),
    GroundTrapRelease(crate::server::script::skill::trap::GroundTrapRelease),
    GroundTrapEffect(crate::server::script::skill::trap::GroundTrapEffect),
    GroundTrapRecover {
        item_id: i32,
        amount: u16,
        x: u16,
        y: u16,
    },
    SetMapFlags(crate::server::model::map_flags::MapFlags),
    UpdateMobsFov(Vec<MapItemSnapshot>),
    UpdateActorVisibility(Vec<(u32, crate::server::service::visibility_service::StealthState)>),
    RemoveCharFromMap(u32),
    InsertCharToMap(MapItem),
    RemoveDroppedItemFromMap(u32),
    MobDamage(Damage),
    ActorSkillCast(crate::server::script::skill::actor::MapActorSkillCast),
    UnitData(crate::server::script::unit_data::MapUnitDataRequest),
    InstallScriptNpc(crate::server::script::unit_data::ScriptNpcTransfer),
    ReleaseScriptNpc(u32),
    NpcEffect(crate::server::service::map_npc_effect::MapNpcEffect),
    MobStatusChange {
        mob_id: u32,
        request: StatusChangeRequest,
    },
    MobStatusAlternatives(MobStatusAlternatives),
    MobProvoke(MobProvoke),
    MobDispel(MobDispel),
    MobEndStatus {
        mob_id: u32,
        kind: Option<StatusChangeKind>,
    },
    MobHeal {
        mob_id: u32,
        hp: u32,
        sp: u32,
    },
    MobRandomWarp {
        mob_id: u32,
    },
    MobFace(MobFace),
    MobWarpTo(MobWarpTo),
    MobKnockback {
        mob_id: u32,
        source_x: u16,
        source_y: u16,
        cells: u16,
    },
    MobSlide {
        mob_id: u32,
        source_x: u16,
        source_y: u16,
        cells: u16,
    },
    MobLoseTarget {
        mob_id: u32,
    },
    ScriptMobCombat {
        source_id: u32,
        target_id: u32,
        effect: MobCombatEffect,
    },
    ScriptDropItem {
        owner_id: u32,
        item_id: i32,
        amount: i16,
        x: u16,
        y: u16,
    },
    ScriptSpawn(ScriptSpawn),
    CastleCommand(CastleCommand),
    ScriptMobCommand(ScriptMobCommand),
    ScriptMapCommand(ScriptMapCommand),
    CaptureMob(u32),
    ClaimPetCapture(PetCaptureClaimRequest),
    FinalizePetCapture(PetCaptureFinalize),
    ClaimPetLoot(PetLootClaimRequest),
    FinalizePetLoot(PetLootFinalize),
    PreparePetLootDrop(PetLootDropRequest),
    FinalizePetLootDrop(PetLootDropFinalize),
    MobDeathClientNotification(MobLocation),
    MobDropItems(MobDropItems),
    MobAttackCharacter(MobAttackCharacter),
    AdminKillAllMobs(u32),
    AdminTogglePauseMobMovement,
    CharDropItems(CharacterDropItems),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobFace {
    pub mob_id: u32,
    pub dir: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobStatusAlternatives {
    pub mob_id: u32,
    pub requests: Vec<StatusChangeRequest>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobProvoke {
    pub mob_id: u32,
    pub source_id: u32,
    pub request: StatusChangeRequest,
    pub coma: crate::server::service::combat_trigger_service::ComaBonuses,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobDispel {
    pub mob_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetCaptureClaimRequest {
    pub claim_id: u64,
    pub char_id: u32,
    pub target_id: u32,
    pub x: u16,
    pub y: u16,
    pub lure_item_id: i32,
    pub flag: u8,
    pub expires_at: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetCaptureFinalize {
    pub claim_id: u64,
    pub target_id: u32,
    pub commit: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootClaimRequest {
    pub claim_id: u64,
    pub char_id: u32,
    pub pet_id: u32,
    pub target_id: u32,
    pub x: u16,
    pub y: u16,
    pub expires_at: u64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PetLootFinalize {
    pub claim_id: u64,
    pub target_id: u32,
    pub commit: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootDropRequest {
    pub claim_id: u64,
    pub char_id: u32,
    pub x: u16,
    pub y: u16,
    pub items: Vec<database::model::InventoryRecord>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PetLootDropFinalize {
    pub claim_id: u64,
    pub commit: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobWarpTo {
    pub mob_id: u32,
    pub x: u16,
    pub y: u16,
}

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
    pub bg_id: u32,
    pub max_hp: Option<u32>,
    pub lifetime_ms: Option<u32>,
    pub reserved_id: Option<u32>,
}

/// Script-driven changes to the monsters spawned with a callback event; `None` targets every monster.
#[derive(Debug, PartialEq, Clone)]
pub enum ScriptMobCommand {
    Kill { event_entry: Option<u32> },
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
    NpcVisibility { npc_id: u32, visible: bool },
    SetCell { area: CellArea, cell: models::enums::cell::CellType, enabled: bool },
    FlagEmblem { castle_map: String, guild_id: u32, version: u16 },
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
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ScriptNpcCallback {
    pub npc_id: u32,
    pub scope_instance: u8,
    pub entry_id: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobLocation {
    pub mob_id: u32,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobDropItems {
    pub owner_id: u32,
    pub mob_id: i16,
    pub mob_x: u16,
    pub mob_y: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterDropItems {
    pub owner_id: u32,
    pub char_x: u16,
    pub char_y: u16,
    pub item_removal_info: Vec<(InventoryItemModel, CharacterRemoveItem)>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobAttackCharacter {
    pub mob_id: u32,
    pub target_char_id: u32,
    pub damage: u32,
    pub attack_motion: u32,
    pub mob_x: u16,
    pub mob_y: u16,
}
