use super::{MapEventContext, MapEventHandler};

#[derive(Debug, PartialEq, Clone)]
pub struct GroundTrapRecover {
    pub item_id: i32,
    pub amount: u16,
    pub x: u16,
    pub y: u16,
}

impl MapEventHandler for crate::server::script::skill::trap::GroundTrapCapture {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .capture_ground_trap_on_map(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for crate::server::script::skill::trap::GroundTrapRelease {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .release_ground_trap_on_map(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for crate::server::script::skill::trap::GroundTrapEffect {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        ctx.service
            .apply_ground_trap_effect_on_map(ctx.map_instance.state_mut().as_mut(), request, ctx.tick);
    }
}

impl MapEventHandler for GroundTrapRecover {
    fn handle(self, ctx: &MapEventContext) {
        let GroundTrapRecover { item_id, amount, x, y } = self;
        ctx.service
            .recover_ground_trap(ctx.map_instance.state_mut().as_mut(), item_id, amount, x, y);
    }
}

impl MapEventHandler for crate::server::script::skill::actor::MapActorSkillCast {
    fn handle(self, ctx: &MapEventContext) {
        let request = self;
        let (skill_id, level, source, source_type, at, target_id, ground) = (
            request.request.skill_id,
            request.request.level,
            request.source.id,
            request.source.object_type,
            (request.source.x, request.source.y),
            request.request.target_id,
            request.request.ground,
        );
        if let Err(error) = ctx
            .service
            .start_actor_skill(ctx.map_instance.state_mut().as_mut(), request, ctx.tick)
        {
            error!(
                "Actor skill cast failed on {}: {} (skill {} lv {}, {:?} {} at {:?}, target {}, ground {:?})",
                ctx.map_instance.name(),
                error,
                skill_id,
                level,
                source_type,
                source,
                at,
                target_id,
                ground
            );
        }
    }
}

impl MapEventHandler for crate::server::service::map_npc_effect::MapNpcEffect {
    fn handle(self, ctx: &MapEventContext) {
        let effect = self;
        ctx.service
            .apply_npc_effect(ctx.map_instance.state_mut().as_mut(), effect, ctx.tick);
    }
}
