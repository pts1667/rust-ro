use models::enums::class::JobName;
use models::enums::look::LookType;
use movement::position::Position;

use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::action::Damage;
use crate::server::model::hotkey::Hotkey;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::movement::Movement;

#[derive(Debug, PartialEq, Clone)]
pub enum GameEvent {
    ScriptRequest(crate::server::script::ScriptRequest),
    NpcContact(NpcContact),
    CharacterScriptSkill(crate::server::script::skill::ScriptSkillEffect),
    ScriptSkillHit(crate::server::script::skill::ScriptSkillHit),
    ScriptWarp(ScriptWarp),
    ScriptCombat(crate::server::service::script_combat_service::ScriptCombatRequest),
    MobAttack(crate::server::service::map_combat_service::MobAttackRequest),
    ReflectMagic(crate::server::service::map_combat_service::MagicReflectionRequest),
    ScriptSpawned(ScriptSpawned),
    ScriptEvent(ScriptEvent),
    ScriptSpawn(ScriptMapSpawn),
    ScriptUnitSkill(ScriptSkillCast),
    ScriptActorSkillComplete(crate::server::script::skill::actor::ScriptActorSkillCompletion),
    ScriptPartyWarp(ScriptPartyWarp),
    ScriptBroadcast(ScriptBroadcast),
    ScriptCraft(crate::server::service::script_crafting_service::CraftSelection),
    ScriptIdentify(ScriptIdentify),
    ScriptTeleportSelection(ScriptTeleportSelection),
    FameChanged(FameChanged),
    TaekwonMissionKill(TaekwonMissionKill),
    ItemScriptComplete(ItemScriptComplete),
    ScriptReveal(crate::server::script::skill::ScriptRevealActor),
    CharacterStatusChange(CharacterStatusChange),
    CharacterStatusAlternatives(CharacterStatusAlternatives),
    CharacterEndStatus(CharacterEndStatus),
    CharacterKnockback(CharacterKnockback),
    CharacterUseGroundSkill(CharacterUseGroundSkill),
    ReleaseScriptCapture(u32),
    PetCaptureClaimResult(PetCaptureClaimResult),
    PetLootClaimResult(PetLootClaimResult),
    PetLootDropResult(PetLootDropResult),
    ScriptWorld(ScriptWorld),
    CharacterLeaveGame((u32, u8)),
    CharacterLoadedFromClientSide(u32),
    CharacterRemoveFromMap(CharacterRemoveFromMap),
    CharacterClearFov(u32),
    CharacterJoinGame(u32),
    CharacterMove(CharacterMovement),
    CharacterCancelMove(u32),
    CharacterChangeMap(CharacterChangeMap),
    CharacterUpdateLook(CharacterLook),
    CharacterUpdateZeny(CharacterZeny),
    CharacterUpdateWeight(u32),
    CharacterAddItems(CharacterAddItems),
    CharacterSellItems(CharacterRemoveItems),
    CharacterInitInventory(u32),
    CharacterUseItem(CharacterUseItem),
    CharacterEquipItem(CharacterEquipItem),
    CharacterTakeoffEquipItem(CharacterTakeoffEquipItem),
    CharacterAttack(CharacterAttack),
    CharacterSit(u32),
    CharacterStand(u32),
    CharacterUseSkill(CharacterUseSkill),
    CharacterDamage(Damage),
    CharacterUpdateClientSideStats(u32),
    CharacterChangeLevel(CharacterChangeLevel),
    CharacterChangeJobLevel(CharacterChangeJobLevel),
    CharacterChangeJob(CharacterChangeJob),
    CharacterKillMonster(CharacterKillMonster),
    CharacterPickUpItem(CharacterPickUpItem),
    CharacterUpdateStat(CharacterUpdateStat),
    CharacterSkillUpgrade(CharacterSkillUpgrade),
    CharacterHotkeyAdd(u32, Hotkey),
    CharacterHotkeyRemove(u32, usize),
    MapNotifyItemRemoved(u32),
    CharacterDropItem(CharacterRemoveItem),
    CharacterResetSkills(u32),
    CharacterResetStats(u32),
    CharacterUpdateSpeed(u32, u16),
    CharacterRestoreAllHpAndSP(u32),
    CharacterRequestCardCompositionList(CharacterEquipItem),
    CharacterSlotCard(CharacterSlotCard),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptWarp {
    pub char_id: u32,
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub destination_instance: Option<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetCaptureClaimResult {
    pub claim_id: u64,
    pub char_id: u32,
    pub target_id: u32,
    pub map_key: MapInstanceKey,
    pub class_id: Option<u16>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootClaimResult {
    pub claim_id: u64,
    pub char_id: u32,
    pub pet_id: u32,
    pub target_id: u32,
    pub map_key: MapInstanceKey,
    pub item: Option<models::item::DroppedItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootDropResult {
    pub claim_id: u64,
    pub char_id: u32,
    pub map_key: MapInstanceKey,
    pub accepted: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NpcContact {
    pub char_id: u32,
    pub account_id: u32,
    pub npc_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptSpawned {
    pub char_id: u32,
    pub mob_ids: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterStatusChange {
    pub char_id: u32,
    pub request: models::status_change::StatusChangeRequest,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterEndStatus {
    pub char_id: u32,
    pub kind: Option<models::status_change::StatusChangeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterKnockback {
    pub char_id: u32,
    pub source_x: u16,
    pub source_y: u16,
    pub cells: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptWorld {
    pub char_id: u32,
    pub request: crate::server::model::game_systems::ScriptWorldRequest,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptEvent {
    pub char_id: u32,
    pub entry_id: u32,
    pub args: Vec<script_sdk::Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptMapSpawn {
    pub char_id: u32,
    pub map: String,
    pub request: crate::server::model::events::map_event::ScriptSpawn,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ScriptSkillCast {
    pub source_id: u32,
    pub target_id: u32,
    pub skill_id: u32,
    pub level: u16,
    pub ground: Option<(u16, u16)>,
    pub cast_time_adjust_ms: i32,
    pub cast_cancel: Option<bool>,
    pub message_id: Option<u16>,
    pub ignore_range: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptIdentify {
    pub char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptTeleportSelection {
    pub char_id: u32,
    pub skill_id: u32,
    pub map: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterStatusAlternatives {
    pub char_id: u32,
    pub requests: Vec<models::status_change::StatusChangeRequest>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FameChanged {
    pub category: crate::repository::fame_repository::FameCategory,
    pub ranked_creators: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TaekwonMissionKill {
    pub char_id: u32,
    pub mob_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemScriptComplete {
    pub action: CharacterUseItem,
    pub source: crate::repository::script_inventory_repository::ScriptItemConsumption,
    pub generation: u64,
    pub effects: Vec<crate::server::script::item_script_handler::ItemEffect>,
    pub error: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptPartyWarp {
    pub char_id: u32,
    pub party_id: u32,
    pub map: String,
    pub x: u16,
    pub y: u16,
    pub source_map: Option<String>,
    pub range_x: u16,
    pub range_y: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptBroadcast {
    pub char_id: u32,
    pub flags: u32,
    pub packet: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseGroundSkill {
    pub char_id: u32,
    pub skill_id: u32,
    pub skill_level: u8,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeMap {
    pub char_id: u32,
    pub new_map_name: String,
    pub new_instance_id: u8,
    pub new_position: Option<Position>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRemoveFromMap {
    pub char_id: u32,
    pub map_name: String,
    pub instance_id: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterMovement {
    pub char_id: u32,
    pub start_at: u128,
    pub destination: Position,
    pub current_position: Position,
    pub path: Vec<Movement>,
    pub cancel_attack: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterLook {
    pub char_id: u32,
    pub look_type: LookType,
    pub look_value: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterZeny {
    pub char_id: u32,
    pub zeny: Option<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterAddItems {
    pub char_id: u32,
    pub should_perform_check: bool, // indicate if we should perform checks before adding items to user
    pub buy: bool,                  // indicate zeny should be used to buy item (zeny will be updated)
    pub items: Vec<InventoryItemModel>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRemoveItems {
    pub char_id: u32,
    pub sell: bool, // indicate zeny should be given to character after item sell (zeny will be updated)
    pub items: Vec<CharacterRemoveItem>,
    pub notify_client: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseItem {
    pub char_id: u32,
    pub target_char_id: u32,
    pub index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterEquipItem {
    pub char_id: u32,
    pub index: usize,
    pub requested_location: Option<u64>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterTakeoffEquipItem {
    pub char_id: u32,
    pub index: usize,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CharacterRemoveItem {
    pub char_id: u32,
    pub index: usize,
    pub amount: i16,
    pub price: i32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterAttack {
    pub char_id: u32,
    pub target_id: u32,
    pub repeat: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseSkill {
    pub char_id: u32,
    pub target_id: u32,
    pub skill_id: u32,
    pub skill_level: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeJobLevel {
    pub char_id: u32,
    pub set_level: Option<u32>,
    pub add_level: Option<i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChangeJob {
    pub char_id: u32,
    pub job: JobName,
    pub should_reset_skills: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterKillMonster {
    pub attacker_id: u32,
    pub char_id: u32,
    pub mob_id: i16,
    pub mob_x: u16,
    pub mob_y: u16,
    pub map_instance_key: MapInstanceKey,
    pub mob_base_exp: u32,
    pub mob_job_exp: u32,
    pub mob_max_hp: u32,
    pub contributions: Vec<crate::server::model::action::DamageContribution>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterPickUpItem {
    pub char_id: u32,
    pub map_item_id: u32,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUpdateStat {
    pub char_id: u32,
    pub stat_id: u16,
    pub change_amount: u16,
}
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSkillUpgrade {
    pub char_id: u32,
    pub skill_id: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRequestCardCompositionList {
    pub char_id: u32,
    pub card_index: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterSlotCard {
    pub char_id: u32,
    pub card_index: usize,
    pub equip_index: usize,
}
