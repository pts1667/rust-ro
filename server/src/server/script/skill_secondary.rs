use models::enums::EnumWithMaskValueU32;
use models::enums::skill_enums::SkillEnum;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use skills::{ActorBehaviour, HitContext, StatusDelay, StatusInfliction};

use super::metadata::SkillMetadata;
use super::{ScriptSkillHit, ScriptSkillService};
use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::{MapEvent, MobKnockback, MobStatusChange, ScriptMobCombat};
use crate::server::service::script_combat_service::MobCombatEffect;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    /// Cicada Skin Shedding: rathena pushes the blocking character away from the attacker.
    pub(crate) fn utsusemi_block_knockback(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut crate::server::state::character::Character,
        attacker_id: u32,
        tick: u128,
    ) -> Result<(), String> {
        let Some(cells) = SkillMetadata::find(SkillEnum::NjUtsusemi.id())
            .and_then(|metadata| metadata.knockback.as_ref())
            .and_then(|value| value.value(1, "Amount"))
        else {
            return Ok(());
        };
        let source = match state.characters().get(&attacker_id) {
            Some(attacker) => Some((attacker.x, attacker.y)),
            None => state
                .get_map_instance_from_character(character)
                .and_then(|instance| instance.state().get_mob(attacker_id).map(|mob| (mob.x, mob.y))),
        };
        let Some((source_x, source_y)) = source else {
            return Ok(());
        };
        self.apply_knockback(server, state, character, source_x, source_y, cells.clamp(0, i32::from(u16::MAX)) as u16, tick)
    }

    /// The implementation of a skill. Monster skills live in `npc`.
    pub(crate) fn skill_object(metadata: &SkillMetadata, level: u8) -> Option<Box<dyn skills::Skill>> {
        let skill_enum = SkillEnum::from_id(metadata.id);
        skills::skill_enums::to_object(skill_enum, level).or_else(|| skills::npc::to_object(skill_enum, level))
    }

    /// What an actor does with the skill beyond damage and statuses. Monster skills without an object are `Default`.
    pub(crate) fn actor_behaviour(metadata: &SkillMetadata, level: u8) -> ActorBehaviour {
        Self::skill_object(metadata, level).map_or(ActorBehaviour::Default, |skill| skill.actor_behaviour())
    }

    /// The statuses a damaging hit inflicts, as the skill object declares them.
    pub(crate) fn hit_statuses(object: Option<&dyn skills::Skill>, hit: &HitContext) -> HitStatuses {
        let Some(skill) = object else {
            return HitStatuses { inflictions: vec![], alternatives: false };
        };
        HitStatuses {
            inflictions: skill.inflict_status_effect_to_target(hit),
            alternatives: skill.status_alternatives(),
        }
    }

    pub(crate) fn status_change_request(metadata: &SkillMetadata, level: u8, infliction: StatusInfliction) -> StatusChangeRequest {
        StatusChangeRequest {
            kind: infliction.kind,
            duration_ms: metadata.duration(level, infliction.secondary_duration).unwrap_or(0),
            values: infliction.values,
            rate: infliction.chance,
            flags: 0,
        }
    }

    /// Milliseconds after the hit. `attack_motion` is only evaluated for `AfterAttackMotion`.
    pub(crate) fn status_delay(delay: StatusDelay, attack_motion: impl FnOnce() -> u32) -> u128 {
        match delay {
            StatusDelay::Immediate => 0,
            StatusDelay::Ms(ms) => u128::from(ms),
            StatusDelay::AfterAttackMotion(extra) => u128::from(attack_motion()) + u128::from(extra),
        }
    }

    pub fn after_skill_damage(&self, server: &Server, state: &mut ServerState, hit: ScriptSkillHit, tick: u128) -> Result<(), String> {
        if state.ground_units().contains(hit.target_id) { return Ok(()); }
        if hit.damage == 0 || hit.depth >= 8 {
            return Ok(());
        }
        let metadata = SkillMetadata::find(hit.skill_id).ok_or("Damaging skill has no callback metadata")?;
        let Some(source) = state.get_character(hit.source_id).or_else(|| {
            state
                .characters()
                .values()
                .find(|owner| crate::server::service::script_world_service::companion_status_snapshot(owner, hit.source_id).is_some())
        }) else {
            return self.after_actor_skill_damage(server, state, hit, tick);
        };
        let map = source.current_map_name().clone();
        let instance_id = source.current_map_instance();
        let companion = source.char_id != hit.source_id;
        let actor_position = if companion {
            crate::server::service::script_world_service::companion_snapshots(source)
                .into_iter()
                .find(|actor| actor.map_item().id() == hit.source_id)
                .map(|actor| (actor.x(), actor.y()))
        } else {
            None
        };
        let (source_x, source_y) = actor_position.unwrap_or((source.x, source.y));
        let source_level = if companion {
            crate::server::service::script_world_service::companion_health(source, hit.source_id)
                .map_or(source.status.base_level, |(_, _, level)| u32::from(level))
        } else {
            source.status.base_level
        };
        let snapshot = if companion {
            crate::server::service::script_world_service::companion_status_snapshot(source, hit.source_id).unwrap()
        } else {
            StatusService::instance().to_snapshot(&source.status)
        };
        let target = state
            .get_character(hit.target_id)
            .map(|character| StatusService::instance().to_snapshot(&character.status))
            .or_else(|| {
                state
                    .map_item(hit.target_id, &map, instance_id)
                    .and_then(|item| state.map_item_mob_status(&item, &map, instance_id))
            });
        if metadata.monster_skill() == Some(super::monster::MonsterSkill::VampireGift) {
            if let Some(source) = state.characters_mut().get_mut(&hit.source_id) {
                if source.status.hp > 0 && !source.status.has_status_change(StatusChangeKind::NoRecovery) {
                    server.character_service().update_hp_sp(
                        source,
                        source.status.hp.saturating_add(hit.damage).min(snapshot.max_hp()),
                        source.status.sp,
                    );
                }
            }
        }
        let Some(target) = target else {
            return Ok(());
        };
        if metadata.name == "RG_INTIMIDATE" && !companion {
            let mob = state.get_map_instance(&map, instance_id).and_then(|instance| {
                instance.state().get_mob(hit.target_id).map(|mob| {
                    (
                        mob.status_effects.base_level,
                        mob.mode & models::enums::mob::MobMode::Boss.as_flag() != 0,
                    )
                })
            });
            let (target_level, immune) = state
                .get_character(hit.target_id)
                .map(|target| (target.status.base_level, false))
                .or(mob)
                .unwrap_or((source_level, false));
            if Self::snatch_succeeds(source_level, target_level, hit.skill_level, immune, fastrand::u8(0..100)) {
                let effect = super::ScriptSkillEffect {
                    source_char_id: hit.source_id,
                    target_id: hit.source_id,
                    skill_id: hit.skill_id,
                    level: hit.skill_level,
                    heal_value: 0,
                    proc_depth: hit.depth,
                    skill_event_emitted: true,
                    cast_generation: 0,
                    action: super::ScriptSkillAction::SnatchWarp {
                        victim_id: hit.target_id,
                        map: map.clone(),
                        instance: instance_id,
                    },
                    deferred_requirements: None,
                    prepared_outcome: None,
                    source_index: None,
                    source_item: None,
                };
                server.add_to_tick(
                    GameEvent::CharacterScriptSkill(effect),
                    800_u128.div_ceil(40).saturating_sub(1) as usize,
                );
            }
        }
        let object = Self::skill_object(metadata, hit.skill_level);
        let key = (map.clone(), instance_id, hit.target_id);
        let gust_hits = if object.as_deref().is_some_and(|skill| skill.counts_hits_on_target()) {
            let mut counters = self.storm_gust_hits.lock().map_err(|_| "Storm Gust counter state is unavailable")?;
            if target.hp() == 0 {
                counters.remove(&key);
                0
            } else {
                let count = counters.entry(key.clone()).or_default();
                if target
                    .active_statuses()
                    .iter()
                    .any(|change| change.kind == StatusChangeKind::Freeze)
                    || *count > 250
                {
                    *count = 0;
                }
                *count = count.saturating_add(1);
                *count
            }
        } else {
            0
        };
        if target.hp() > 0 {
            let HitStatuses { inflictions, alternatives } = Self::hit_statuses(
                object.as_deref(),
                &HitContext {
                    source: &snapshot,
                    target: &target,
                    hits_on_target: gust_hits,
                    caster_is_player: !companion,
                },
            );
            if alternatives {
                let requests = inflictions
                    .iter()
                    .map(|infliction| Self::status_change_request(metadata, hit.skill_level, *infliction))
                    .collect::<Vec<_>>();
                if let Some(target) = state.characters_mut().get_mut(&hit.target_id) {
                    StatusEffectService::start_alternatives(server, target, requests, tick, &self.client_notification_sender)?;
                } else if state.companion_owner(hit.target_id, &map, instance_id).is_some() {
                    server.add_to_next_tick(GameEvent::CharacterStatusAlternatives(
                        crate::server::model::events::game_event::CharacterStatusAlternatives {
                            char_id: hit.target_id,
                            requests,
                        },
                    ));
                } else if let Some(instance) = state.get_map_instance(&map, instance_id) {
                    instance.add_to_next_tick(MapEvent::MobStatusAlternatives(
                        crate::server::model::events::map_event::MobStatusAlternatives {
                            mob_id: hit.target_id,
                            requests,
                        },
                    ));
                }
            }
            for infliction in if alternatives { vec![] } else { inflictions } {
                if infliction.monsters_only && state.get_character(hit.target_id).is_some() {
                    continue;
                }
                let delay = Self::status_delay(infliction.delay, || StatusService::instance().attack_motion(&snapshot));
                let request = Self::status_change_request(metadata, hit.skill_level, infliction);
                if state.get_character(hit.target_id).is_some() || state.companion_owner(hit.target_id, &map, instance_id).is_some() {
                    if delay > 0 {
                        server.add_to_delayed_tick(
                            GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange {
                                char_id: hit.target_id,
                                request,
                            }),
                            delay,
                        );
                    } else if let Some(character) = state.characters_mut().get_mut(&hit.target_id) {
                        StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
                    } else {
                        server.add_to_next_tick(GameEvent::CharacterStatusChange(
                            crate::server::model::events::game_event::CharacterStatusChange {
                                char_id: hit.target_id,
                                request,
                            },
                        ));
                    }
                } else if let Some(instance) = state.get_map_instance(&map, instance_id) {
                    if delay > 0 {
                        instance.add_to_delayed_tick(
                            MapEvent::MobStatusChange(MobStatusChange {
                                mob_id: hit.target_id,
                                request,
                            }),
                            delay,
                        );
                    } else {
                        instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange {
                            mob_id: hit.target_id,
                            request,
                        }));
                    }
                }
            }
        }
        if metadata.name == "PA_PRESSURE" {
            let percent = 15 + 5 * hit.skill_level as u32;
            if let Some(target) = state.characters_mut().get_mut(&hit.target_id) {
                server.character_service().update_hp_sp(
                    target,
                    target.status.hp,
                    target.status.sp.saturating_sub(target.status.sp * percent / 100),
                );
            } else if let Some(instance) = state.get_map_instance(&map, instance_id) {
                instance.add_to_next_tick(MapEvent::ScriptMobCombat(ScriptMobCombat {
                    source_id: hit.source_id,
                    target_id: hit.target_id,
                    effect: MobCombatEffect::Vanish {
                        hp: 0,
                        sp: target.sp().saturating_mul(percent) / 100,
                    },
                }));
            }
        }
        if object.as_deref().map_or(true, |skill| skill.knocks_back_on_hit()) {
            if let Some(cells) = metadata
                .knockback
                .as_ref()
                .and_then(|value| value.value(hit.skill_level, "Amount"))
                .filter(|value| *value > 0)
            {
                if state.get_character(hit.target_id).is_some() {
                    server.add_to_next_tick(GameEvent::CharacterKnockback(
                        crate::server::model::events::game_event::CharacterKnockback {
                            char_id: hit.target_id,
                            source_x,
                            source_y,
                            cells: cells.min(u16::MAX as i32) as u16,
                        },
                    ));
                } else if let Some(instance) = state.get_map_instance(&map, instance_id) {
                    if instance.state().get_mob(hit.target_id).is_some() {
                        instance.add_to_next_tick(MapEvent::MobKnockback(MobKnockback {
                            mob_id: hit.target_id,
                            source_x,
                            source_y,
                            cells: cells.min(u16::MAX as i32) as u16,
                        }));
                    }
                }
            }
        }
        Ok(())
    }
}

/// Statuses a hit inflicts. With `alternatives`, only the first one that lands is applied.
pub(crate) struct HitStatuses {
    pub(crate) inflictions: Vec<StatusInfliction>,
    pub(crate) alternatives: bool,
}

#[cfg(test)]
mod tests {
    use models::status::StatusSnapshot;
    use skills::{MagicProfile, SplashProfile};

    use super::*;

    fn hit_statuses_for(metadata: &SkillMetadata, level: u8, hits_on_target: u8, caster_is_player: bool) -> HitStatuses {
        let source = StatusSnapshot::_from(&models::status::Status::default());
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let object = ScriptSkillService::skill_object(metadata, level);
        ScriptSkillService::hit_statuses(
            object.as_deref(),
            &HitContext {
                source: &source,
                target: &target,
                hits_on_target,
                caster_is_player,
            },
        )
    }

    fn hit_requests_for(metadata: &SkillMetadata, level: u8, hits_on_target: u8) -> Vec<StatusChangeRequest> {
        hit_statuses_for(metadata, level, hits_on_target, true)
            .inflictions
            .into_iter()
            .map(|infliction| ScriptSkillService::status_change_request(metadata, level, infliction))
            .collect()
    }

    #[test]
    fn storm_gust_freezes_only_from_the_third_hit() {
        let gust = SkillMetadata::find(SkillEnum::WzStormgust.id()).unwrap();
        assert!(hit_requests_for(gust, 10, 2).is_empty());
        assert_eq!(hit_requests_for(gust, 10, 3)[0].rate, 15_000);
    }

    #[test]
    fn hit_statuses_come_from_the_skill_object_with_its_chance() {
        let frost = SkillMetadata::find(SkillEnum::MgFrostdiver.id()).unwrap();
        let requests = hit_requests_for(frost, 10, 0);
        assert_eq!(
            requests.iter().map(|request| (request.kind, request.rate)).collect::<Vec<_>>(),
            vec![(StatusChangeKind::Freeze, 6500)]
        );
        assert_eq!(requests[0].duration_ms, frost.duration(10, true).unwrap());
        let holy_cross = SkillMetadata::find(SkillEnum::CrHolycross.id()).unwrap();
        let blind = hit_requests_for(holy_cross, 10, 0);
        assert_eq!(blind.iter().map(|request| (request.kind, request.rate)).collect::<Vec<_>>(), vec![(StatusChangeKind::Blind, 3000)]);
    }

    #[test]
    fn rogue_intimidate_stuns_for_the_primary_duration_and_blinds_for_the_secondary() {
        let raid = SkillMetadata::find(SkillEnum::RgRaid.id()).unwrap();
        let requests = hit_requests_for(raid, 5, 0);
        assert_eq!(
            requests
                .iter()
                .map(|request| (request.kind, request.rate, request.duration_ms))
                .collect::<Vec<_>>(),
            vec![
                (StatusChangeKind::Stun, 2500, raid.duration(5, false).unwrap()),
                (StatusChangeKind::Blind, 2500, raid.duration(5, true).unwrap()),
            ]
        );
    }

    #[test]
    fn land_mine_stuns_one_second_after_the_hit() {
        let landmine = SkillMetadata::find(SkillEnum::MaLandmine.id()).unwrap();
        let inflictions = hit_statuses_for(landmine, 1, 0, false).inflictions;
        assert_eq!(inflictions.len(), 1);
        assert!(matches!(inflictions[0].delay, StatusDelay::Ms(1_000)));
        let requests = hit_requests_for(landmine, 1, 0);
        assert_eq!(
            requests.iter().map(|request| (request.kind, request.rate, request.duration_ms)).collect::<Vec<_>>(),
            vec![(StatusChangeKind::Stun, 1000, landmine.duration(1, true).unwrap())]
        );
    }

    #[test]
    fn freezing_trap_freezes_after_the_attack_motion_plus_its_delay() {
        let trap = SkillMetadata::find(SkillEnum::MaFreezingtrap.id()).unwrap();
        let inflictions = hit_statuses_for(trap, 1, 0, false).inflictions;
        assert_eq!(inflictions.len(), 1);
        assert!(matches!(inflictions[0].delay, StatusDelay::AfterAttackMotion(100)));
        assert_eq!(ScriptSkillService::status_delay(StatusDelay::AfterAttackMotion(100), || 500), 600);
        assert_eq!(ScriptSkillService::status_delay(StatusDelay::Ms(1_000), || 0), 1_000);
    }

    #[test]
    fn stone_fling_stuns_players_and_falls_back_to_blind_while_other_actors_only_stun() {
        let stone = SkillMetadata::find(SkillEnum::TfThrowstone.id()).unwrap();
        let player = hit_statuses_for(stone, 1, 0, true);
        assert!(player.alternatives);
        assert_eq!(
            player.inflictions.iter().map(|infliction| (infliction.kind, infliction.chance)).collect::<Vec<_>>(),
            vec![(StatusChangeKind::Stun, 500), (StatusChangeKind::Blind, 500)]
        );
        assert_eq!(
            hit_requests_for(stone, 1, 0)
                .iter()
                .map(|request| (request.kind, request.rate, request.duration_ms))
                .collect::<Vec<_>>(),
            vec![(StatusChangeKind::Stun, 500, 5000), (StatusChangeKind::Blind, 500, 30000)]
        );
        let other = hit_statuses_for(stone, 1, 0, false);
        assert!(other.alternatives);
        assert_eq!(
            other.inflictions.iter().map(|infliction| (infliction.kind, infliction.chance)).collect::<Vec<_>>(),
            vec![(StatusChangeKind::Stun, 500)]
        );
    }

    #[test]
    fn npc_poison_applies_its_secondary_duration_and_level_chance_after_a_hit() {
        let metadata = SkillMetadata::all().iter().find(|skill| skill.name == "NPC_POISON").unwrap();
        let requests = hit_requests_for(metadata, 3, 0);
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].kind, StatusChangeKind::Poison);
        assert_eq!(requests[0].rate, 6000);
        assert_eq!(requests[0].duration_ms, metadata.duration(3, true).unwrap());
    }

    #[test]
    fn actor_behaviour_comes_from_the_skill_object() {
        let behaviour = |skill: SkillEnum, level: u8| ScriptSkillService::actor_behaviour(SkillMetadata::find(skill.id()).unwrap(), level);
        assert_eq!(behaviour(SkillEnum::AlHeal, 1), ActorBehaviour::Heal);
        assert_eq!(behaviour(SkillEnum::AlTeleport, 1), ActorBehaviour::Teleport);
        assert_eq!(behaviour(SkillEnum::TfThrowstone, 1), ActorBehaviour::FixedWeapon { amount: 30 });
        assert_eq!(behaviour(SkillEnum::SaDispell, 5), ActorBehaviour::Dispel { chance: 100 });
        assert_eq!(behaviour(SkillEnum::SlSma, 1), ActorBehaviour::Magic(MagicProfile { consumes_sma: true, grants_sma_from_level: None }));
        assert_eq!(behaviour(SkillEnum::SlStun, 7), ActorBehaviour::Magic(MagicProfile { consumes_sma: false, grants_sma_from_level: Some(7) }));
        assert!(matches!(
            behaviour(SkillEnum::SmMagnum, 1),
            ActorBehaviour::Splash(profile) if profile.single_hit && profile.distance_ratio && profile.knockback_cells == 2
        ));
        assert_eq!(behaviour(SkillEnum::WzFrostnova, 1), ActorBehaviour::Splash(SplashProfile::default()));
    }

    #[test]
    fn strip_profiles_name_their_slots_and_full_strip_rate() {
        let (kinds, full) = ScriptSkillService::strip_profile(SkillEnum::RgStripweapon.id(), 1).unwrap();
        assert_eq!((kinds, full), (&[StatusChangeKind::StripWeapon][..], false));
        let (kinds, full) = ScriptSkillService::strip_profile(SkillEnum::StFullstrip.id(), 1).unwrap();
        assert_eq!(kinds.len(), 4);
        assert!(full);
        assert_eq!(ScriptSkillService::strip_profile(SkillEnum::AlHeal.id(), 1), None);
    }

    #[test]
    fn status_kinds_come_from_objects_or_metadata() {
        assert_eq!(ScriptSkillService::status_for_skill("SM_PROVOKE"), Some(StatusChangeKind::Provoke));
        assert_eq!(ScriptSkillService::status_for_skill("PR_LEXDIVINA"), Some(StatusChangeKind::Silence));
        assert_eq!(ScriptSkillService::status_for_skill("SM_SELFPROVOKE"), Some(StatusChangeKind::Provoke));
        assert_eq!(ScriptSkillService::status_for_skill("AL_HEAL"), None);
    }
}
