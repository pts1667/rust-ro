use models::enums::mob::MobCapability;
use models::status_change::{StatusChange, StatusChangeKind};

use super::MapInstanceService;
use super::npc_effect::{MapNpcEffect, NpcEffect};
use super::unit_data::walkable;
use crate::server::script::skill::trap::{GroundTrapCapture, GroundTrapEffect, GroundTrapEffectKind, GroundTrapRelease};
use crate::server::service::ground_trap_service::trap_fix_position;
use crate::server::state::map_instance::MapInstanceState;

fn holds_trap(statuses: &[StatusChange], trap_id: u32) -> bool {
    statuses
        .iter()
        .any(|change| change.kind == StatusChangeKind::Ankle && change.values[1] == trap_id as i32)
}

impl MapInstanceService {
    pub(crate) fn recover_ground_trap(&self, state: &mut MapInstanceState, item_id: i32, amount: u16, x: u16, y: u16) {
        if amount == 0 || self.configuration_service.find_item(item_id).is_none() {
            return;
        }
        if let Some(item) = self.drop_items(
            state,
            &mut fastrand::Rng::new(),
            x,
            y,
            item_id,
            true,
            amount,
            None,
            Default::default(),
            false,
        ) {
            self.notify_drop_items(state, x, y, vec![item]);
        }
    }

    pub(crate) fn apply_ground_trap_effect_on_map(&self, state: &mut MapInstanceState, request: GroundTrapEffect, tick: u128) {
        if state.key() != &request.map {
            return;
        }
        if state.get_mob(request.target_id).is_some_and(|mob| mob.is_present() && mob.hp() > 0) {
            match request.kind {
                GroundTrapEffectKind::Status(status) => self.start_mob_status(state, request.target_id, status, tick),
                GroundTrapEffectKind::DrainSp { percent } => {
                    let mob = state.mobs_mut().get_mut(&request.target_id).unwrap();
                    let amount = (u64::from(mob.status.max_sp()) * u64::from(percent.min(100)) / 100)
                        .max(1)
                        .min(u64::from(u32::MAX)) as u32;
                    mob.status_effects.sp = mob.status_effects.sp.saturating_sub(amount);
                    mob.status.set_sp(mob.status_effects.sp);
                }
                GroundTrapEffectKind::Knockback { source_x, source_y, cells } => {
                    self.mob_knockback(state, request.target_id, source_x, source_y, cells);
                    if let Some(mob) = state.mobs_mut().get_mut(&request.target_id) {
                        mob.lose_target();
                    }
                }
                GroundTrapEffectKind::WalkDelay { milliseconds } => {
                    let mob = state.mobs_mut().get_mut(&request.target_id).unwrap();
                    mob.movements.clear();
                    mob.timing
                        .set_canmove_tick(mob.timing.get_canmove_tick().max(tick + u128::from(milliseconds)));
                }
                GroundTrapEffectKind::BreakWeapon => {}
            }
        } else if let Some(npc) = state.script_skill_state.npcs.get(&request.target_id).filter(|npc| npc.hp > 0) {
            let effect = match request.kind {
                GroundTrapEffectKind::Status(status) => NpcEffect::Status(status),
                GroundTrapEffectKind::DrainSp { percent } => NpcEffect::Vanish {
                    source_id: 0,
                    hp: 0,
                    sp: (u64::from(npc.max_sp) * u64::from(percent.min(100)) / 100)
                        .max(1)
                        .min(u64::from(u32::MAX)) as u32,
                },
                GroundTrapEffectKind::Knockback { source_x, source_y, cells } => NpcEffect::Knockback { source_x, source_y, cells },
                GroundTrapEffectKind::WalkDelay { .. } | GroundTrapEffectKind::BreakWeapon => return,
            };
            self.apply_npc_effect(
                state,
                MapNpcEffect {
                    actor_id: request.target_id,
                    effect,
                },
                tick,
            );
        }
    }

    pub(crate) fn capture_ground_trap_on_map(&self, state: &mut MapInstanceState, request: GroundTrapCapture, tick: u128) {
        if !request.lease.active() || tick > request.deadline || state.key() != &request.map {
            return;
        }
        let can_move = walkable(state, request.x, request.y);
        if state.get_mob(request.target_id).is_some_and(|mob| mob.is_present() && mob.hp() > 0) {
            self.start_mob_status(state, request.target_id, request.request, tick);
            let moved = state
                .mobs_mut()
                .get_mut(&request.target_id)
                .filter(|mob| {
                    can_move
                        && holds_trap(&mob.status_effects.active_statuses, request.trap_id)
                        && !mob.status.has_mob_capability(MobCapability::KnockbackImmune)
                })
                .map(|mob| {
                    mob.movements.clear();
                    mob.update_position(request.x, request.y);
                })
                .is_some();
            if moved {
                self.notify_area(
                    state,
                    request.x,
                    request.y,
                    trap_fix_position(request.target_id, request.x, request.y),
                );
            }
        } else if state.script_skill_state.npcs.get(&request.target_id).is_some_and(|npc| npc.hp > 0) {
            self.apply_npc_effect(
                state,
                MapNpcEffect {
                    actor_id: request.target_id,
                    effect: NpcEffect::Status(request.request),
                },
                tick,
            );
            if can_move
                && state.script_skill_state.npcs.get(&request.target_id).is_some_and(|npc| {
                    holds_trap(&npc.active_statuses, request.trap_id) && !npc.snapshot().has_mob_capability(MobCapability::KnockbackImmune)
                })
            {
                self.apply_npc_effect(
                    state,
                    MapNpcEffect {
                        actor_id: request.target_id,
                        effect: NpcEffect::Move {
                            x: request.x,
                            y: request.y,
                        },
                    },
                    tick,
                );
            }
        }
    }

    pub(crate) fn release_ground_trap_on_map(&self, state: &mut MapInstanceState, request: GroundTrapRelease, tick: u128) {
        if state.key() != &request.map {
            return;
        }
        if state
            .get_mob(request.target_id)
            .is_some_and(|mob| holds_trap(&mob.status_effects.active_statuses, request.trap_id))
        {
            self.end_mob_status(state, request.target_id, Some(StatusChangeKind::Ankle));
        } else if state
            .script_skill_state
            .npcs
            .get(&request.target_id)
            .is_some_and(|npc| holds_trap(&npc.active_statuses, request.trap_id))
        {
            self.apply_npc_effect(
                state,
                MapNpcEffect {
                    actor_id: request.target_id,
                    effect: NpcEffect::EndStatus(Some(StatusChangeKind::Ankle)),
                },
                tick,
            );
        }
    }
}
