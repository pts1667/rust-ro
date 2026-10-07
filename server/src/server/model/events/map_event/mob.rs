use super::{MapEventContext, MapEventHandler};
use crate::server::model::action::Damage;
use crate::server::model::map_item::MapItemSnapshot;
use crate::server::service::script_combat_service::MobCombatEffect;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobWarpTo {
    pub mob_id: u32,
    pub x: u16,
    pub y: u16,
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

impl MobDropItems {
    /// The drops of a kill whose killer may loot automatically.
    pub fn into_event(self, autoloot: crate::server::model::autoloot::AutoLoot) -> super::MapEvent {
        if autoloot.is_off() {
            super::MapEvent::MobDropItems(self)
        } else {
            super::MapEvent::MobAutoLootDrops(MobAutoLootDrops { drops: self, autoloot })
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MobAutoLootDrops {
    pub drops: MobDropItems,
    pub autoloot: crate::server::model::autoloot::AutoLoot,
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

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateMobsFov {
    pub characters: Vec<MapItemSnapshot>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobDamage {
    pub damage: Damage,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobStatusChange {
    pub mob_id: u32,
    pub request: StatusChangeRequest,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobEndStatus {
    pub mob_id: u32,
    pub kind: Option<StatusChangeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobHeal {
    pub mob_id: u32,
    pub hp: u32,
    pub sp: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobRandomWarp {
    pub mob_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobKnockback {
    pub mob_id: u32,
    pub source_x: u16,
    pub source_y: u16,
    pub cells: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobSlide {
    pub mob_id: u32,
    pub source_x: u16,
    pub source_y: u16,
    pub cells: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MobLoseTarget {
    pub mob_id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptMobCombat {
    pub source_id: u32,
    pub target_id: u32,
    pub effect: MobCombatEffect,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CaptureMob {
    pub id: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AdminKillAllMobs {
    pub char_id: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AdminTogglePauseMobMovement;

impl MapEventHandler for UpdateMobsFov {
    fn handle(self, ctx: &MapEventContext) {
        let UpdateMobsFov { characters } = self;
        ctx.service.update_mobs_fov(ctx.map_instance.state_mut().as_mut(), characters);
    }
}

impl MapEventHandler for MobDamage {
    fn handle(self, ctx: &MapEventContext) {
        let MobDamage { damage } = self;
        let mut map_instance_state = ctx.map_instance.state_mut();
        ctx.service
            .mob_being_attacked(map_instance_state.as_mut(), damage, ctx.map_instance.task_queue(), ctx.tick);
    }
}

impl MapEventHandler for MobStatusChange {
    fn handle(self, ctx: &MapEventContext) {
        let MobStatusChange { mob_id, request } = self;
        ctx.service
            .start_mob_status(ctx.map_instance.state_mut().as_mut(), mob_id, request, ctx.tick);
    }
}

impl MapEventHandler for MobStatusAlternatives {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .start_mob_status_alternatives(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for MobProvoke {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.provoke_mob(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for MobDispel {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service.dispel_mob(ctx.map_instance.state_mut().as_mut(), request.mob_id);
    }
}

impl MapEventHandler for MobEndStatus {
    fn handle(self, ctx: &MapEventContext) {
        let MobEndStatus { mob_id, kind } = self;
        ctx.service.end_mob_status(ctx.map_instance.state_mut().as_mut(), mob_id, kind);
    }
}

impl MapEventHandler for MobHeal {
    fn handle(self, ctx: &MapEventContext) {
        let MobHeal { mob_id, hp, sp } = self;
        ctx.service.heal_mob(ctx.map_instance.state_mut().as_mut(), mob_id, hp, sp);
    }
}

impl MapEventHandler for MobRandomWarp {
    fn handle(self, ctx: &MapEventContext) {
        let MobRandomWarp { mob_id } = self;
        ctx.service.random_warp_mob(ctx.map_instance.state_mut().as_mut(), mob_id);
    }
}

impl MapEventHandler for MobFace {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .face_mob(ctx.map_instance.state_mut().as_mut(), request.mob_id, request.dir);
    }
}

impl MapEventHandler for MobWarpTo {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .warp_mob_to(ctx.map_instance.state_mut().as_mut(), request.mob_id, request.x, request.y);
    }
}

impl MapEventHandler for MobKnockback {
    fn handle(self, ctx: &MapEventContext) {
        let MobKnockback {
            mob_id,
            source_x,
            source_y,
            cells,
        } = self;
        ctx.service
            .mob_knockback(ctx.map_instance.state_mut().as_mut(), mob_id, source_x, source_y, cells);
    }
}

impl MapEventHandler for MobSlide {
    fn handle(self, ctx: &MapEventContext) {
        let MobSlide {
            mob_id,
            source_x,
            source_y,
            cells,
        } = self;
        ctx.service
            .slide_mob(ctx.map_instance.state_mut().as_mut(), mob_id, source_x, source_y, cells);
    }
}

impl MapEventHandler for MobLoseTarget {
    fn handle(self, ctx: &MapEventContext) {
        let MobLoseTarget { mob_id } = self;
        if let Some(mob) = ctx.map_instance.state_mut().mobs_mut().get_mut(&mob_id) {
            mob.lose_target();
        }
    }
}

impl MapEventHandler for ScriptMobCombat {
    fn handle(self, ctx: &MapEventContext) {
        let ScriptMobCombat {
            source_id,
            target_id,
            effect,
        } = self;
        ctx.service.script_mob_combat(
            ctx.map_instance.state_mut().as_mut(),
            source_id,
            target_id,
            effect,
            ctx.map_instance.task_queue(),
            ctx.tick,
        );
    }
}

impl MapEventHandler for CaptureMob {
    fn handle(self, ctx: &MapEventContext) {
        let CaptureMob { id } = self;
        ctx.service.capture_mob(ctx.map_instance.state_mut().as_mut(), id);
    }
}

impl MapEventHandler for MobLocation {
    fn handle(self, ctx: &MapEventContext) {
        let mob_location = self;
        let map_instance_state = ctx.map_instance.state();
        ctx.service.mob_die_client_notification(map_instance_state.as_ref(), mob_location);
    }
}

impl MapEventHandler for MobDropItems {
    fn handle(self, ctx: &MapEventContext) {
        let mob_drop_items = self;
        ctx.service
            .mob_drop_items_and_send_packet(ctx.map_instance.state_mut().as_mut(), mob_drop_items);
    }
}

impl MapEventHandler for MobAutoLootDrops {
    fn handle(self, ctx: &MapEventContext) {
        ctx.service
            .mob_drop_items_for_autoloot(ctx.map_instance.state_mut().as_mut(), self.drops, self.autoloot);
    }
}

impl MapEventHandler for MobAttackCharacter {
    fn handle(self, ctx: &MapEventContext) {
        let attack = self;
        let map_instance_state = ctx.map_instance.state();
        ctx.service.mob_attack_character(
            map_instance_state.as_ref(),
            attack,
            ctx.map_instance.task_queue().as_ref(),
            ctx.tick,
        );
    }
}

impl MapEventHandler for AdminKillAllMobs {
    fn handle(self, ctx: &MapEventContext) {
        let AdminKillAllMobs { char_id } = self;
        ctx.service
            .kill_all_mobs(ctx.map_instance.state_mut().as_mut(), ctx.map_instance.task_queue(), char_id);
    }
}

impl MapEventHandler for AdminTogglePauseMobMovement {
    fn handle(self, ctx: &MapEventContext) {
        let mut map_instance_state = ctx.map_instance.state_mut();
        let paused = map_instance_state.mob_movement_paused();
        map_instance_state.set_mob_movement_paused(!paused);
    }
}
