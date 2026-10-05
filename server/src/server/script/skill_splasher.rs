use models::enums::mob::MobMode;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::{Status, StatusSnapshot};
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub fn splasher_request(effect: &ScriptSkillEffect, hp: u32, max_hp: u32, immune: bool) -> Result<StatusChangeRequest, String> {
        if immune || hp == 0 || u64::from(hp) * 4 > u64::from(max_hp) * 3 {
            return Err("Venom Splasher requires a non-immune target at or below three quarters health".into());
        }
        let duration = SkillMetadata::find(SkillEnum::AsSplasher.id())
            .and_then(|metadata| metadata.duration(effect.level, false))
            .ok_or("Venom Splasher countdown is missing")?;
        Ok(StatusChangeRequest {
            kind: StatusChangeKind::Splasher,
            duration_ms: duration,
            values: [
                i32::from(effect.level),
                effect.skill_id as i32,
                effect.source_char_id as i32,
                duration,
            ],
            rate: 10000,
            flags: u32::from(effect.proc_depth.min(8)) * StatusStartFlag::ScriptProcDepth.as_flag(),
        })
    }

    pub fn splasher_expiration(target_id: u32, status: &Status, tick: u128) -> Option<ScriptSkillEffect> {
        if status.hp == 0 {
            return None;
        }
        let change = status
            .status_change(StatusChangeKind::Splasher)
            .filter(|change| change.expired(tick))?;
        Some(ScriptSkillEffect {
            source_char_id: change.values[2] as u32,
            target_id,
            skill_id: SkillEnum::AsSplasher.id(),
            level: change.values[0].clamp(1, 10) as u8,
            heal_value: 0,
            proc_depth: (change.flags / StatusStartFlag::ScriptProcDepth.as_flag()).min(8) as u8,
            skill_event_emitted: true,
            cast_generation: 0,
            action: ScriptSkillAction::ExplodeSplasher,
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        })
    }

    pub(super) fn validate_splasher_effect(
        &self,
        state: &ServerState,
        source: &Character,
        effect: &ScriptSkillEffect,
    ) -> Result<(), String> {
        if effect.action != ScriptSkillAction::Cast {
            return Ok(());
        }
        if let Some(target) = state.get_character(effect.target_id) {
            let snapshot = StatusService::instance().to_snapshot(&target.status);
            Self::splasher_request(effect, snapshot.hp(), snapshot.max_hp(), false)?;
        } else {
            let instance = state
                .get_map_instance_from_character(source)
                .ok_or("Venom Splasher map is unavailable")?;
            let map_state = instance.state();
            let target = map_state.get_mob(effect.target_id).ok_or("Venom Splasher target is unavailable")?;
            Self::splasher_request(
                effect,
                target.hp(),
                target.status.max_hp(),
                target.mode & MobMode::Boss.as_flag() != 0,
            )?;
        }
        Ok(())
    }

    pub(super) fn explode_splasher(
        &self,
        server: &Server,
        state: &ServerState,
        target: &Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<(), String> {
        let source = state
            .get_character(effect.source_char_id)
            .filter(|source| source.status.hp > 0 && source.map_instance_key == target.map_instance_key)
            .ok_or("Venom Splasher caster left the map")?;
        self.explode_splasher_at(
            server,
            state,
            source,
            effect,
            target.x,
            target.y,
            Some((target.char_id, StatusService::instance().to_snapshot(&target.status))),
            tick,
        )
    }

    pub(super) fn explode_mob_splasher(
        &self,
        server: &Server,
        state: &ServerState,
        source: &Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<(), String> {
        let instance = state
            .get_map_instance_from_character(source)
            .ok_or("Venom Splasher map is unavailable")?;
        let map_state = instance.state();
        let target = map_state
            .get_mob(effect.target_id)
            .filter(|mob| mob.hp() > 0)
            .ok_or("Venom Splasher target died")?;
        self.explode_splasher_at(server, state, source, effect, target.x, target.y, None, tick)
    }

    fn explode_splasher_at(
        &self,
        server: &Server,
        state: &ServerState,
        source: &Character,
        effect: &ScriptSkillEffect,
        x: u16,
        y: u16,
        player_target: Option<(u32, StatusSnapshot)>,
        tick: u128,
    ) -> Result<(), String> {
        let instance = state
            .get_map_instance_from_character(source)
            .ok_or("Venom Splasher map is unavailable")?;
        let snapshot = StatusService::instance().to_snapshot(&source.status);
        let poison_react = snapshot
            .known_skills()
            .iter()
            .find(|skill| skill.value == SkillEnum::AsPoisonreact)
            .map_or(0, |skill| skill.level);
        let ratio = (500 + 50 * u32::from(effect.level) + 20 * u32::from(poison_react)) as f32 / 100.0;
        let element = server.battle_service().attack_element(&snapshot, None);
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
        let map_state = instance.state();
        let split = map_state
            .mobs()
            .values()
            .filter(|mob| {
                mob.hp() > 0
                    && Self::area_skill_target_allowed(&snapshot, &mob.status, effect.skill_id)
                    && mob.x.abs_diff(x).max(mob.y.abs_diff(y)) <= 1
                    && (!mob.summoned || mob.summon_ai == 0)
            })
            .count()
            + usize::from(player_target.as_ref().is_some_and(|(_, status)| status.hp() > 0));
        let split = split.max(1).min(u32::MAX as usize) as u32;
        for mob in map_state.mobs().values().filter(|mob| {
            mob.hp() > 0
                && Self::area_skill_target_allowed(&snapshot, &mob.status, effect.skill_id)
                && mob.x.abs_diff(x).max(mob.y.abs_diff(y)) <= 2
                && (!mob.summoned || mob.summon_ai == 0)
        }) {
            let damage = server.battle_service().player_physical_splash_damage_signed(
                &snapshot,
                &mob.status,
                ratio,
                1,
                false,
                &element,
                effect.skill_id,
                split,
            );
            let mut damage_event = Damage {
                notification: None,
                source_kind: models::enums::actor::CombatActorKind::Player,
                skill_damage_adjusted: false,
                healing: 0,
                right_hand_damage: None,
                target_id: mob.id,
                attacker_id: source.char_id,
                damage: 0,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: flags,
                skill_id: effect.skill_id,
                skill_level: effect.level,
                proc_depth: effect.proc_depth,
                credit_id: source.char_id,
                defenses_applied: true,
                magic_context: None,
                landed: true,
            };
            damage_event.set_signed_damage(damage);
            damage_event = damage_event.with_skill_notification(
                source.current_map_name(),
                source.current_map_instance(),
                source.x,
                source.y,
                tick,
                1,
                0,
            );
            instance.add_to_next_tick(MapEvent::MobDamage(damage_event));
        }
        if let Some((target_id, target)) = player_target {
            let damage = server.battle_service().player_physical_splash_damage_signed(
                &snapshot,
                &target,
                ratio,
                1,
                false,
                &element,
                effect.skill_id,
                split,
            );
            let mut damage_event = Damage {
                notification: None,
                source_kind: models::enums::actor::CombatActorKind::Player,
                skill_damage_adjusted: false,
                healing: 0,
                right_hand_damage: None,
                target_id,
                attacker_id: source.char_id,
                damage: 0,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: flags,
                skill_id: effect.skill_id,
                skill_level: effect.level,
                proc_depth: effect.proc_depth,
                credit_id: source.char_id,
                defenses_applied: true,
                magic_context: None,
                landed: true,
            };
            damage_event.set_signed_damage(damage);
            damage_event = damage_event.with_skill_notification(
                source.current_map_name(),
                source.current_map_instance(),
                source.x,
                source.y,
                tick,
                1,
                0,
            );
            server.add_to_next_tick(GameEvent::CharacterDamage(damage_event));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::service::status_effect_service::StatusEffectService;
    #[test]
    fn splasher_only_explodes_on_natural_expiry_and_keeps_proc_depth() {
        let effect = ScriptSkillEffect {
            source_char_id: 10,
            target_id: 20,
            skill_id: SkillEnum::AsSplasher.id(),
            level: 10,
            heal_value: 0,
            proc_depth: 5,
            skill_event_emitted: false,
            cast_generation: 0,
            action: ScriptSkillAction::Cast,
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        };
        assert!(ScriptSkillService::splasher_request(&effect, 76, 100, false).is_err());
        assert!(ScriptSkillService::splasher_request(&effect, 75, 100, true).is_err());
        let mut status = Status {
            hp: 75,
            max_hp: 100,
            ..Status::default()
        };
        StatusEffectService::apply_status(
            &mut status,
            ScriptSkillService::splasher_request(&effect, 75, 100, false).unwrap(),
            100,
            0,
        )
        .unwrap();
        assert!(ScriptSkillService::splasher_expiration(20, &status, 2099).is_none());
        let explosion = ScriptSkillService::splasher_expiration(20, &status, 2100).unwrap();
        assert_eq!(
            (explosion.source_char_id, explosion.target_id, explosion.proc_depth),
            (10, 20, 5)
        );
        assert_eq!(explosion.action, ScriptSkillAction::ExplodeSplasher);
        assert!(explosion.deferred_requirements.is_none());
        StatusEffectService::end_status(&mut status, Some(StatusChangeKind::Splasher));
        assert!(ScriptSkillService::splasher_expiration(20, &status, 2100).is_none());
    }
}
