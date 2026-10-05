use models::enums::class::JobName;
use models::enums::look::LookType;
use movement::position::Position;

use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::action::Damage;
use crate::server::model::hotkey::Hotkey;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::movement::Movement;

#[derive(Debug, PartialEq, Clone)]
pub enum CastleLifecycle {
    Init,
    AgitStart,
    AgitEnd,
    EmperiumBroken { map: String, guild_id: u32 },
    GuildBroken { guild_id: u32 },
    Refresh { map: String, abandoned: bool },
    AnnounceConquest { map: String, guild_id: u32 },
    RestartArena { map: String },
    SummonGuardian { map: String, slot: u8 },
    DailyTick,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameEvent {
    GroundTrapCapture(crate::server::script::skill::trap::GroundTrapCapture),
    GroundTrapRelease(crate::server::script::skill::trap::GroundTrapRelease),
    GroundTrapEffect(crate::server::script::skill::trap::GroundTrapEffect),
    GroundTrapSpend { map: MapInstanceKey, unit_id: u32 },
    ScriptRequest(crate::server::script::ScriptRequest),
    ScriptNpcTransfer(crate::server::script::unit_data::ScriptNpcTransfer),
    ScriptNpcEvent(ScriptNpcEvent),
    ScriptLogoutAction(crate::server::model::character_lifecycle::ScriptLogoutAction),
    ScriptLogoutCompleted(crate::server::model::character_lifecycle::ScriptLogoutCompleted),
    CharacterSelectionGate(crate::server::model::character_lifecycle::CharacterSelectionGate),
    CharacterAdmission(crate::server::model::character_lifecycle::CharacterAdmission),
    CharacterMapEntry(crate::server::model::character_lifecycle::CharacterMapEntry),
    CharacterMapReady(crate::server::model::character_lifecycle::CharacterMapReady),
    CharacterLogout(crate::server::model::character_lifecycle::CharacterLogout),
    ClientDisconnected(crate::server::model::character_lifecycle::ClientDisconnected),
    ScriptMapDamage(ScriptMapDamage),
    NpcContact(NpcContact),
    CharacterScriptSkill(crate::server::script::skill::ScriptSkillEffect),
    ScriptSkillHit(crate::server::script::skill::ScriptSkillHit),
    ScriptWarp(ScriptWarp),
    ScriptCombat(crate::server::service::script_combat_service::ScriptCombatRequest),
    MobAttack(crate::server::service::map_combat_service::MobAttackRequest),
    ReflectMagic(crate::server::service::map_combat_service::MagicReflectionRequest),
    ScriptSpawned(ScriptSpawned),
    CastleLifecycle(CastleLifecycle),
    ScriptEvent(ScriptEvent),
    ScriptSpawn(ScriptMapSpawn),
    ScriptUnitSkill(ScriptSkillCast),
    ScriptActorSkillComplete(crate::server::script::skill::actor::ScriptActorSkillCompletion),
    ScriptPartyWarp(ScriptPartyWarp),
    ScriptBroadcast(ScriptBroadcast),
    ScriptCraft(crate::server::service::script_crafting_service::CraftSelection),
    ScriptIdentify(ScriptIdentify),
    ScriptTeleportSelection(ScriptTeleportSelection),
    WarpPortalEnter(crate::server::script::skill::WarpPortalEntry),
    FameChanged(FameChanged),
    TaekwonMissionKill(TaekwonMissionKill),
    ItemScriptComplete(ItemScriptComplete),
    ScriptReveal(crate::server::script::skill::ScriptRevealActor),
    CharacterStatusChange(CharacterStatusChange),
    CharacterStatusAlternatives(CharacterStatusAlternatives),
    CharacterEndStatus(CharacterEndStatus),
    CharacterKnockback(CharacterKnockback),
    CharacterUseGroundSkill(CharacterUseGroundSkill),
    CharacterUseGroundSkillText(CharacterUseGroundSkillText),
    ReleaseScriptCapture(u32),
    PetCaptureClaimResult(PetCaptureClaimResult),
    PetLootClaimResult(PetLootClaimResult),
    PetLootDropResult(PetLootDropResult),
    ScriptWorld(ScriptWorld),
    PlayerTrade(PlayerTradeAction),
    CharacterLeaveGame((u32, u8)),
    CharacterLoadedFromClientSide(u32),
    CharacterRemoveFromMap(CharacterRemoveFromMap),
    CharacterClearFov(u32),
    CharacterJoinGame(u32),
    CharacterMove(CharacterMovement),
    CharacterSavePosition(u32),
    CharacterMemo(crate::server::model::character_lifecycle::CharacterMemo),
    CharacterRespawn(crate::server::model::character_lifecycle::CharacterRespawn),
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
    Duel(crate::server::model::duel::DuelCommand),
    BattlegroundQueue(crate::server::model::battleground_queue::BattlegroundQueueCommand),
    CharacterRequestCardCompositionList(CharacterEquipItem),
    CharacterSlotCard(CharacterSlotCard),
}

impl GameEvent {
    pub(crate) fn required_character(&self) -> Option<u32> {
        use GameEvent::*;
        Some(match self {
            CharacterLoadedFromClientSide(id)
            | CharacterClearFov(id)
            | CharacterJoinGame(id)
            | CharacterSavePosition(id)
            | CharacterCancelMove(id)
            | CharacterUpdateWeight(id)
            | CharacterInitInventory(id)
            | CharacterSit(id)
            | CharacterStand(id)
            | CharacterUpdateClientSideStats(id)
            | MapNotifyItemRemoved(id)
            | CharacterResetSkills(id)
            | CharacterResetStats(id)
            | CharacterRestoreAllHpAndSP(id) => *id,
            CharacterHotkeyAdd(id, _) | CharacterHotkeyRemove(id, _) | CharacterUpdateSpeed(id, _) => *id,
            CharacterRemoveFromMap(event) => event.char_id,
            Duel(event) => event.char_id,
            BattlegroundQueue(event) => event.char_id,
            CharacterMove(event) => event.char_id,
            CharacterMemo(event) => event.session.char_id?,
            CharacterRespawn(event) => event.session.char_id?,
            CharacterChangeMap(event) => event.char_id,
            CharacterUpdateLook(event) => event.char_id,
            CharacterUpdateZeny(event) => event.char_id,
            CharacterAddItems(event) => event.char_id,
            CharacterSellItems(event) => event.char_id,
            CharacterUseItem(event) => event.char_id,
            CharacterEquipItem(event) | CharacterRequestCardCompositionList(event) => event.char_id,
            CharacterTakeoffEquipItem(event) => event.char_id,
            CharacterAttack(event) => event.char_id,
            CharacterUseSkill(event) => event.char_id,
            CharacterDamage(event) => event.target_id,
            CharacterChangeLevel(event) => event.char_id,
            CharacterChangeJobLevel(event) => event.char_id,
            CharacterChangeJob(event) => event.char_id,
            CharacterKillMonster(event) => event.char_id,
            CharacterPickUpItem(event) => event.char_id,
            CharacterUpdateStat(event) => event.char_id,
            CharacterSkillUpgrade(event) => event.char_id,
            CharacterDropItem(event) => event.char_id,
            CharacterSlotCard(event) => event.char_id,
            CharacterKnockback(event) => event.char_id,
            CharacterUseGroundSkill(event) => event.char_id,
            CharacterUseGroundSkillText(event) => event.skill.char_id,
            WarpPortalEnter(event) => event.char_id,
            NpcContact(event) => event.char_id,
            PlayerTrade(event) => event.char_id,
            ScriptWorld(event) => event.char_id,
            ScriptWarp(event) => event.char_id,
            ScriptEvent(event) => event.char_id,
            ItemScriptComplete(event) => event.action.char_id,
            CharacterScriptSkill(event) => event.source_char_id,
            _ => return None,
        })
    }

    pub(crate) fn affects_character(&self, mut matches: impl FnMut(u32) -> bool) -> bool {
        if self.required_character().is_some_and(&mut matches) {
            return true;
        }
        match self {
            Self::GroundTrapCapture(event) => matches(event.target_id),
            Self::GroundTrapEffect(event) => matches(event.target_id),
            Self::CharacterStatusChange(event) => matches(event.char_id),
            Self::CharacterStatusAlternatives(event) => matches(event.char_id),
            Self::CharacterEndStatus(event) => matches(event.char_id),
            Self::ScriptCombat(event) => matches(event.source_id) || matches(event.target_id),
            Self::ScriptSkillHit(event) => matches(event.source_id) || matches(event.target_id),
            Self::ScriptMapDamage(event) => matches(event.damage.target_id) || matches(event.damage.attacker_id),
            Self::ScriptUnitSkill(event) => matches(event.source_id) || matches(event.target_id),
            Self::CharacterScriptSkill(event) => matches(event.source_char_id) || matches(event.target_id),
            Self::CharacterAttack(event) => matches(event.target_id),
            Self::CharacterUseSkill(event) => matches(event.target_id),
            Self::MobAttack(event) => matches(event.attack.target_char_id),
            Self::ReflectMagic(event) => {
                matches(event.damage.target_id) || matches(event.damage.attacker_id) || matches(event.reflector_id)
            }
            _ => false,
        }
    }
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
pub struct PlayerTradeAction {
    pub char_id: u32,
    pub account_id: u32,
    pub auth_code: i32,
    pub request: crate::server::model::game_systems::PlayerTradeRequest,
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
pub struct ScriptNpcEvent {
    pub npc_id: u32,
    pub scope_instance: u8,
    pub entry_id: u32,
    pub char_id: Option<u32>,
    pub depth: u8,
    pub queued_until: u128,
    pub args: Option<Vec<script_sdk::Value>>,
    pub timer_guard: Option<crate::server::model::script_timer::ScriptTimerGuard>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptMapSpawn {
    pub char_id: u32,
    pub map: String,
    pub request: crate::server::model::events::map_event::ScriptSpawn,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ScriptSkillCast {
    pub source_map: Option<String>,
    pub source_instance: Option<u8>,
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
pub struct ScriptMapDamage {
    pub map: MapInstanceKey,
    pub damage: Damage,
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
    pub session: Option<crate::server::model::session::SessionBinding>,
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
pub struct CharacterUseGroundSkillText {
    pub skill: CharacterUseGroundSkill,
    pub message: Vec<u8>,
    pub session: crate::server::model::session::SessionBinding,
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
