use models::enums::bonus::BonusType;
use models::status_change::StatusChangeKind;
use serde::{Deserialize, Serialize};
use crate::server::model::map_instance::MapInstanceKey;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PetSupportRuntime {
    pub pet_id: u32,
    pub initialized: bool,
    pub requires_accessory: bool,
    pub base_bonuses: Vec<BonusType>,
    pub bonus: Option<PetTimedBonus>,
    pub recovery: Option<PetRecovery>,
    pub skill: Option<PetSupportSkill>,
    pub attack: Option<PetAttackSkill>,
    pub loot: Option<PetLootRuntime>,
    pub casting: Option<PetSupportCast>,
    pub can_act_at: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetLootRuntime {
    pub capacity: u8,
    pub next_at: u64,
    pub return_requested: bool,
    pub target: Option<u32>,
    pub claim_queued: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PetLootCargo {
    pub pet_id: u32,
    pub items: Vec<database::model::InventoryRecord>,
    pub pending_drop: Option<PetLootDropReservation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PetLootDropReceipt {
    pub claim_id: u64,
    pub map: String,
    pub map_instance: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PetLootDropReservation {
    pub receipt: PetLootDropReceipt,
    pub x: u16,
    pub y: u16,
    pub expires_at: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PendingPetLootClaim {
    pub claim_id: u64,
    pub pet_id: u32,
    pub target_id: u32,
    pub map_key: MapInstanceKey,
    pub expires_at: u64,
    pub committed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetSupportCast {
    pub id: u64,
    pub skill_id: u32,
    pub level: u8,
    pub target_id: u32,
    pub attack: Option<PetAttackSkill>,
    pub map_key: MapInstanceKey,
    pub completes_at: u64,
    pub queued: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetAttackSkill {
    pub skill_id: u32,
    pub level: u8,
    pub fixed_damage: Option<u32>,
    pub hits: i16,
    pub rate: i32,
    pub bonus_rate: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetTimedBonus {
    pub bonuses: Vec<BonusType>,
    pub duration_ms: u64,
    pub delay_ms: u64,
    pub next_at: Option<u64>,
    pub active: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetRecovery {
    pub kind: StatusChangeKind,
    pub delay_ms: u64,
    pub next_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PetSupportSkill {
    pub skill_id: u32,
    pub level: u8,
    pub delay_ms: u64,
    pub hp_threshold: u8,
    pub sp_threshold: u8,
    pub next_at: Option<u64>,
}
