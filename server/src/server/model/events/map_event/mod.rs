mod ground;
mod map;
mod mob;
mod pet;
mod script;

pub use ground::*;
pub use map::*;
pub use mob::*;
pub use pet::*;
pub use script::*;

use crate::server::model::map_instance::MapInstance;
use crate::server::service::map_instance_service::MapInstanceService;

pub(crate) struct MapEventContext<'a> {
    pub service: &'a MapInstanceService,
    pub map_instance: &'a MapInstance,
    pub tick: u128,
}

/// One map instance loop event: its payload type implements this trait next to its definition.
pub(crate) trait MapEventHandler {
    fn handle(self, ctx: &MapEventContext);
}

macro_rules! map_events {
    ($($variant:ident($payload:ty)),+ $(,)?) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum MapEvent {
            $($variant($payload)),+
        }

        impl MapEvent {
            pub(crate) fn dispatch(self, ctx: &MapEventContext) {
                match self {
                    $(Self::$variant(event) => MapEventHandler::handle(event, ctx)),+
                }
            }
        }
    };
}

map_events! {
    GroundTrapCapture(crate::server::script::skill::trap::GroundTrapCapture),
    GroundTrapRelease(crate::server::script::skill::trap::GroundTrapRelease),
    GroundTrapEffect(crate::server::script::skill::trap::GroundTrapEffect),
    GroundTrapRecover(GroundTrapRecover),
    SetMapFlags(SetMapFlags),
    UpdateMobsFov(UpdateMobsFov),
    UpdateActorVisibility(UpdateActorVisibility),
    RemoveCharFromMap(RemoveCharFromMap),
    InsertCharToMap(InsertCharToMap),
    RemoveDroppedItemFromMap(RemoveDroppedItemFromMap),
    MobDamage(MobDamage),
    ActorSkillCast(crate::server::script::skill::actor::MapActorSkillCast),
    UnitData(crate::server::script::unit_data::MapUnitDataRequest),
    InstallScriptNpc(crate::server::script::unit_data::ScriptNpcTransfer),
    ReleaseScriptNpc(ReleaseScriptNpc),
    NpcEffect(crate::server::service::map_npc_effect::MapNpcEffect),
    MobStatusChange(MobStatusChange),
    MobStatusAlternatives(MobStatusAlternatives),
    MobProvoke(MobProvoke),
    MobDispel(MobDispel),
    MobEndStatus(MobEndStatus),
    MobHeal(MobHeal),
    MobRandomWarp(MobRandomWarp),
    MobFace(MobFace),
    MobWarpTo(MobWarpTo),
    MobKnockback(MobKnockback),
    MobSlide(MobSlide),
    MobLoseTarget(MobLoseTarget),
    ScriptMobCombat(ScriptMobCombat),
    ScriptDropItem(ScriptDropItem),
    ScriptSpawn(ScriptSpawn),
    CastleCommand(CastleCommand),
    ScriptMobCommand(ScriptMobCommand),
    ScriptMapCommand(ScriptMapCommand),
    CaptureMob(CaptureMob),
    ClaimPetCapture(PetCaptureClaimRequest),
    FinalizePetCapture(PetCaptureFinalize),
    ClaimPetLoot(PetLootClaimRequest),
    FinalizePetLoot(PetLootFinalize),
    PreparePetLootDrop(PetLootDropRequest),
    FinalizePetLootDrop(PetLootDropFinalize),
    MobDeathClientNotification(MobLocation),
    MobDropItems(MobDropItems),
    MobAutoLootDrops(MobAutoLootDrops),
    MobAttackCharacter(MobAttackCharacter),
    AdminKillAllMobs(AdminKillAllMobs),
    AdminTogglePauseMobMovement(AdminTogglePauseMobMovement),
    CharDropItems(CharacterDropItems),
}
