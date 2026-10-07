mod character;
mod client_command;
mod lifecycle;
mod quest;
mod request;
mod script;
mod skill;
mod social;
mod world;

pub use character::*;
pub use client_command::*;
pub use lifecycle::*;
pub use quest::*;
pub use request::*;
pub use script::*;
pub use skill::*;
pub use social::*;
pub use world::*;

use crate::server::Server;
use crate::server::state::server::ServerState;

/// One game loop event: its payload type implements this trait next to its definition.
pub(crate) trait GameEventHandler {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String>;

    /// Character that must still be in the game for the event to run.
    fn required_character(&self) -> Option<u32> {
        None
    }

    /// Other characters involved, on top of `required_character`; the event is held back while any is logging out.
    fn also_affects(&self, _matches: &mut dyn FnMut(u32) -> bool) -> bool {
        false
    }
}

macro_rules! game_events {
    ($($variant:ident($payload:ty)),+ $(,)?) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum GameEvent {
            $($variant($payload)),+
        }

        impl GameEvent {
            pub(crate) fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant(_) => stringify!($variant)),+
                }
            }

            pub(crate) fn dispatch(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
                match self {
                    $(Self::$variant(event) => GameEventHandler::handle(event, server, state, tick)),+
                }
            }

            pub(crate) fn required_character(&self) -> Option<u32> {
                match self {
                    $(Self::$variant(event) => GameEventHandler::required_character(event)),+
                }
            }

            pub(crate) fn affects_character(&self, mut matches: impl FnMut(u32) -> bool) -> bool {
                if self.required_character().is_some_and(&mut matches) {
                    return true;
                }
                match self {
                    $(Self::$variant(event) => GameEventHandler::also_affects(event, &mut matches)),+
                }
            }
        }
    };
}

game_events! {
    GroundTrapCapture(crate::server::script::skill::trap::GroundTrapCapture),
    GroundTrapRelease(crate::server::script::skill::trap::GroundTrapRelease),
    GroundTrapEffect(crate::server::script::skill::trap::GroundTrapEffect),
    GroundTrapSpend(GroundTrapSpend),
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
    ReleaseScriptCapture(ReleaseScriptCapture),
    PetCaptureClaimResult(PetCaptureClaimResult),
    PetLootClaimResult(PetLootClaimResult),
    PetLootDropResult(PetLootDropResult),
    ScriptWorld(ScriptWorld),
    PlayerTrade(PlayerTradeAction),
    CharacterLeaveGame(CharacterLeaveGame),
    CharacterLoadedFromClientSide(CharacterLoadedFromClientSide),
    CharacterRemoveFromMap(CharacterRemoveFromMap),
    CharacterClearFov(CharacterClearFov),
    CharacterJoinGame(CharacterJoinGame),
    CharacterMove(CharacterMovement),
    CharacterRequestMove(CharacterRequestMove),
    CharacterRequestName(CharacterRequestName),
    CharacterChat(CharacterChat),
    CharacterSavePosition(CharacterSavePosition),
    CharacterMemo(crate::server::model::character_lifecycle::CharacterMemo),
    CharacterRespawn(crate::server::model::character_lifecycle::CharacterRespawn),
    CharacterCancelMove(CharacterCancelMove),
    CharacterChangeMap(CharacterChangeMap),
    CharacterUpdateLook(CharacterLook),
    CharacterUpdateZeny(CharacterZeny),
    CharacterUpdateWeight(CharacterUpdateWeight),
    CharacterAddItems(CharacterAddItems),
    CharacterSellItems(CharacterRemoveItems),
    CharacterInitInventory(CharacterInitInventory),
    CharacterUseItem(CharacterUseItem),
    CharacterEquipItem(CharacterEquipItem),
    CharacterTakeoffEquipItem(CharacterTakeoffEquipItem),
    CharacterAttack(CharacterAttack),
    CharacterSit(CharacterSit),
    CharacterStand(CharacterStand),
    CharacterUseSkill(CharacterUseSkill),
    CharacterDamage(CharacterDamage),
    CharacterUpdateClientSideStats(CharacterUpdateClientSideStats),
    CharacterChangeLevel(CharacterChangeLevel),
    CharacterChangeJobLevel(CharacterChangeJobLevel),
    CharacterChangeJob(CharacterChangeJob),
    CharacterKillMonster(CharacterKillMonster),
    CharacterPickUpItem(CharacterPickUpItem),
    CharacterUpdateStat(CharacterUpdateStat),
    CharacterSkillUpgrade(CharacterSkillUpgrade),
    CharacterHotkeyAdd(CharacterHotkeyAdd),
    CharacterHotkeyRemove(CharacterHotkeyRemove),
    MapNotifyItemRemoved(MapNotifyItemRemoved),
    CharacterDropItem(CharacterRemoveItem),
    CharacterResetSkills(CharacterResetSkills),
    CharacterResetStats(CharacterResetStats),
    CharacterUpdateSpeed(CharacterUpdateSpeed),
    CharacterRestoreAllHpAndSP(CharacterRestoreAllHpAndSP),
    Duel(crate::server::model::duel::DuelCommand),
    BattlegroundQueue(crate::server::model::battleground_queue::BattlegroundQueueCommand),
    CharacterRequestCardCompositionList(CharacterRequestCardCompositionList),
    CharacterSlotCard(CharacterSlotCard),
    CharacterClientCommand(CharacterClientCommand),
    CharacterSocial(CharacterSocial),
    CharacterQuestActivation(CharacterQuestActivation),
    QuestMonsterKill(QuestMonsterKill),
}
