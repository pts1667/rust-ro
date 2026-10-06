use models::enums::actor::CombatActorKind;
use models::enums::EnumWithMaskValueU32;
use models::enums::element::Element;
use models::status_bonus::BattleFlag;

use super::ScriptSkillService;
use super::ground::{GroundSkill, GroundSkillSource};
use super::metadata::SkillMetadata;
use super::trap::{GroundTrapEffect, GroundTrapEffectKind, GroundTrapTarget};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{GameEvent, ScriptMapDamage};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::state::server::ServerState;

const EMPERIUM_MOB_ID: i16 = 1288;
const FIRE_PILLAR_ACTIVE_VIEW: u32 = 136;
const FIRE_PILLAR_LINGER_MS: u128 = 1500;

impl ScriptSkillService {
    /// The waiting pillar erupts as soon as an enemy stands in its range, then hits everything in the splash area.
    pub(super) fn tick_fire_pillar(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &mut GroundSkill,
        source: &GroundSkillSource,
        tick: u128,
    ) {
        let targets = self.trap_targets(server, state, ground, source);
        if !targets.iter().any(|target| ground.covers(target.x, target.y)) {
            return;
        }
        let Some(metadata) = SkillMetadata::find(ground.skill_id) else {
            return;
        };
        let level = ground.level;
        let hits = metadata
            .hit_count
            .as_ref()
            .and_then(|count| count.value(level, "Count"))
            .unwrap_or(1)
            .clamp(1, i32::from(i16::MAX)) as i16;
        let splash = metadata.splash(level).unwrap_or(0).max(0) as u16;
        let walk_delay = metadata.duration(level, true).unwrap_or(0).max(0) as u32;
        let (center_x, center_y) = (ground.cells[0].x, ground.cells[0].y);
        let player_source = *source.status.combat_actor_kind() == CombatActorKind::Player;
        let flags = BattleFlag::Magic.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        let map = MapInstanceKey::new(ground.map.clone(), ground.instance);
        for target in targets
            .iter()
            .filter(|target| target.x.abs_diff(center_x).max(target.y.abs_diff(center_y)) <= splash)
        {
            let (amount, context) = server.battle_service().fire_pillar_damage_signed(
                &source.status,
                &target.status,
                ground.skill_id,
                level,
                hits as u16,
                player_source,
            );
            let mut damage = Self::ground_unit_damage(ground, source, target.id, tick, flags, true, Some(context));
            damage.set_signed_damage(amount);
            damage = damage.with_skill_notification(&ground.map, ground.instance, center_x, center_y, tick, hits, 0);
            server.add_to_next_tick(GameEvent::ScriptMapDamage(ScriptMapDamage { map: map.clone(), damage }));
            if walk_delay > 0 {
                server.add_to_next_tick(GameEvent::GroundTrapEffect(GroundTrapEffect {
                    map: map.clone(),
                    target_id: target.id,
                    kind: GroundTrapEffectKind::WalkDelay { milliseconds: walk_delay },
                }));
            }
        }
        ground.triggered = true;
        ground.expires_at = tick + FIRE_PILLAR_LINGER_MS;
        self.change_trap_view(ground, FIRE_PILLAR_ACTIVE_VIEW);
    }

    /// Every interval the bomb hits each enemy around it with a Fire weapon attack and may break their weapon.
    pub(super) fn tick_demonstration(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &mut GroundSkill,
        source: &GroundSkillSource,
        tick: u128,
    ) {
        ground.next_hit_at = tick + ground.interval;
        let targets = self
            .trap_targets(server, state, ground, source)
            .into_iter()
            .filter(|target| ground.covers(target.x, target.y) && !Self::is_emperium(state, ground, target))
            .collect::<Vec<_>>();
        if targets.is_empty() {
            return;
        }
        let Some(metadata) = SkillMetadata::find(ground.skill_id) else {
            return;
        };
        let level = ground.level;
        let player_source = *source.status.combat_actor_kind() == CombatActorKind::Player;
        let flags = metadata.battle_flags(player_source);
        let ratio = (100 + 20 * u32::from(level)) as f32 / 100.0;
        let map = MapInstanceKey::new(ground.map.clone(), ground.instance);
        for target in targets {
            let landed = server
                .battle_service()
                .skill_hits(&source.status, &target.status, ground.skill_id, level);
            let amount = if !landed {
                0
            } else if player_source {
                server.battle_service().player_physical_skill_damage_signed(
                    &source.status,
                    &target.status,
                    ratio,
                    1,
                    player_source,
                    &Element::Fire,
                    ground.skill_id,
                )
            } else {
                server.battle_service().actor_physical_skill_damage_signed(
                    source.raw_attack,
                    &source.status,
                    &target.status,
                    target.player,
                    ratio,
                    1,
                    &Element::Fire,
                    flags,
                    ground.skill_id,
                )
            };
            let mut damage = Self::ground_unit_damage(ground, source, target.id, tick, flags, landed, None);
            damage.set_signed_damage(amount);
            damage = damage.with_skill_notification(&ground.map, ground.instance, ground.cells[0].x, ground.cells[0].y, tick, 1, 0);
            server.add_to_next_tick(GameEvent::ScriptMapDamage(ScriptMapDamage { map: map.clone(), damage }));
            if landed && target.player && fastrand::i32(0..10_000) < 100 * i32::from(level) {
                server.add_to_next_tick(GameEvent::GroundTrapEffect(GroundTrapEffect {
                    map: map.clone(),
                    target_id: target.id,
                    kind: GroundTrapEffectKind::BreakWeapon,
                }));
            }
        }
    }

    fn is_emperium(state: &ServerState, ground: &GroundSkill, target: &GroundTrapTarget) -> bool {
        !target.player
            && state
                .get_map_instance(&ground.map, ground.instance)
                .is_some_and(|map| map.state().get_mob(target.id).is_some_and(|mob| mob.mob_id == EMPERIUM_MOB_ID))
    }

    fn ground_unit_damage(
        ground: &GroundSkill,
        source: &GroundSkillSource,
        target_id: u32,
        tick: u128,
        flags: u32,
        landed: bool,
        magic_context: Option<crate::server::service::map_combat_service::MagicAttackContext>,
    ) -> Damage {
        Damage {
            notification: None,
            source_kind: *source.status.combat_actor_kind(),
            skill_damage_adjusted: false,
            healing: 0,
            right_hand_damage: None,
            target_id,
            attacker_id: source.actor_id,
            damage: 0,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: flags,
            skill_id: ground.skill_id,
            skill_level: ground.level,
            proc_depth: ground.depth,
            credit_id: source.owner_id,
            defenses_applied: true,
            magic_context,
            landed,
        }
    }
}
