use models::enums::EnumWithMaskValueU32;
use models::enums::skill_enums::SkillEnum;
use models::status::StatusSnapshot;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use skills::{HitContext, StatusInfliction};

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

    pub fn stone_fling_status_requests(player_source: bool, level: u8) -> Vec<StatusChangeRequest> {
        let Some(metadata) = SkillMetadata::find(SkillEnum::TfThrowstone.id()) else {
            return vec![];
        };
        let request = |kind, rate, secondary| StatusChangeRequest {
            kind,
            rate,
            values: [i32::from(level), 0, 0, 0],
            flags: 0,
            duration_ms: metadata.duration(level, secondary).unwrap_or(0),
        };
        if player_source {
            vec![
                request(StatusChangeKind::Stun, 500, false),
                request(StatusChangeKind::Blind, 500, true),
            ]
        } else {
            vec![request(StatusChangeKind::Stun, 500, false)]
        }
    }

    /// Statuses the skill object declares for a damaging hit. The server rolls each chance against the target.
    pub(crate) fn trait_status_requests(
        metadata: &SkillMetadata,
        level: u8,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        hits_on_target: u8,
    ) -> Vec<StatusChangeRequest> {
        let skill_enum = SkillEnum::from_id(metadata.id);
        let Some(skill) = skills::skill_enums::to_object(skill_enum, level).or_else(|| skills::npc::to_object(skill_enum, level)) else {
            return vec![];
        };
        let hit = HitContext { source, target, hits_on_target };
        skill
            .inflict_status_effect_to_target(&hit)
            .into_iter()
            .map(|infliction| status_change_request(metadata, level, infliction))
            .collect()
    }

    /// Statuses of skills that have no `lib/skills` object yet. Move each arm into its skill's hook once the object exists.
    pub(crate) fn objectless_status_requests(metadata: &SkillMetadata, level: u8, target: &StatusSnapshot) -> Vec<StatusChangeRequest> {
        let infliction = match metadata.name.as_str() {
            "MER_CRASH" => StatusInfliction::secondary(StatusChangeKind::Stun, 600 * i32::from(level), level),
            "MA_LANDMINE" => StatusInfliction::secondary(StatusChangeKind::Stun, 1_000, level),
            "MA_FREEZINGTRAP" => StatusInfliction::secondary(StatusChangeKind::Freeze, 10_000, level),
            "ML_SPIRALPIERCE" => StatusInfliction::secondary(StatusChangeKind::Ankle, 10_000, level),
            "SL_STUN" if *target.size() == models::enums::size::Size::Medium => {
                StatusInfliction::primary(StatusChangeKind::Stun, (30 + 10 * i32::from(level)) * 100, level)
            }
            _ => return vec![],
        };
        vec![status_change_request(metadata, level, infliction)]
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
        let key = (map.clone(), instance_id, hit.target_id);
        let gust_hits = if metadata.name == "WZ_STORMGUST" {
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
            if metadata.name == "TF_THROWSTONE" {
                let requests = Self::stone_fling_status_requests(!companion, hit.skill_level);
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
            let requests = Self::trait_status_requests(metadata, hit.skill_level, &snapshot, &target, gust_hits)
                .into_iter()
                .chain(Self::objectless_status_requests(metadata, hit.skill_level, &target));
            for request in requests {
                if metadata.name == "CR_GRANDCROSS" && state.get_character(hit.target_id).is_some() {
                    continue;
                }
                let delay = match metadata.name.as_str() {
                    "MA_LANDMINE" | "HT_LANDMINE" => 1000,
                    "MA_FREEZINGTRAP" | "HT_FREEZINGTRAP" => StatusService::instance().attack_motion(&snapshot) as u128 + 100,
                    _ => 0,
                };
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
        if metadata.name != "WZ_STORMGUST" && metadata.name != "MG_FIREWALL" {
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

fn status_change_request(metadata: &SkillMetadata, level: u8, infliction: StatusInfliction) -> StatusChangeRequest {
    StatusChangeRequest {
        kind: infliction.kind,
        duration_ms: metadata.duration(level, infliction.secondary_duration).unwrap_or(0),
        values: infliction.values,
        rate: infliction.chance,
        flags: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_additional_effects_keep_independent_probabilities_and_gust_threshold() {
        let source = StatusSnapshot::_from(&models::status::Status::default());
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let gust = SkillMetadata::find(SkillEnum::WzStormgust.id()).unwrap();
        assert!(ScriptSkillService::trait_status_requests(gust, 10, &source, &target, 2).is_empty());
        assert_eq!(ScriptSkillService::trait_status_requests(gust, 10, &source, &target, 3)[0].rate, 15_000);
    }

    #[test]
    fn hit_statuses_come_from_the_skill_object_with_its_chance() {
        let source = StatusSnapshot::_from(&models::status::Status::default());
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let frost = SkillMetadata::find(SkillEnum::MgFrostdiver.id()).unwrap();
        let requests = ScriptSkillService::trait_status_requests(frost, 10, &source, &target, 0);
        assert_eq!(
            requests.iter().map(|request| (request.kind, request.rate)).collect::<Vec<_>>(),
            vec![(StatusChangeKind::Freeze, 6500)]
        );
        assert_eq!(requests[0].duration_ms, frost.duration(10, true).unwrap());
        let holy_cross = SkillMetadata::find(SkillEnum::CrHolycross.id()).unwrap();
        let blind = ScriptSkillService::trait_status_requests(holy_cross, 10, &source, &target, 0);
        assert_eq!(blind.iter().map(|request| (request.kind, request.rate)).collect::<Vec<_>>(), vec![(StatusChangeKind::Blind, 3000)]);
    }

    #[test]
    fn rogue_intimidate_stuns_for_the_primary_duration_and_blinds_for_the_secondary() {
        let source = StatusSnapshot::_from(&models::status::Status::default());
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let raid = SkillMetadata::find(SkillEnum::RgRaid.id()).unwrap();
        let requests = ScriptSkillService::trait_status_requests(raid, 5, &source, &target, 0);
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
    fn objectless_arms_apply_their_secondary_statuses() {
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let landmine = SkillMetadata::find(SkillEnum::MaLandmine.id()).unwrap();
        let requests = ScriptSkillService::objectless_status_requests(landmine, 1, &target);
        assert_eq!(
            requests.iter().map(|request| (request.kind, request.rate, request.duration_ms)).collect::<Vec<_>>(),
            vec![(StatusChangeKind::Stun, 1000, landmine.duration(1, true).unwrap())]
        );
    }

    #[test]
    fn npc_poison_applies_its_secondary_duration_and_level_chance_after_a_hit() {
        let metadata = SkillMetadata::all().iter().find(|skill| skill.name == "NPC_POISON").unwrap();
        let source = StatusSnapshot::_from(&models::status::Status::default());
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let requests = ScriptSkillService::trait_status_requests(metadata, 3, &source, &target, 0);
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].kind, StatusChangeKind::Poison);
        assert_eq!(requests[0].rate, 6000);
        assert_eq!(requests[0].duration_ms, metadata.duration(3, true).unwrap());
    }

    #[test]
    fn stone_fling_falls_back_from_player_stun_to_blind_but_other_actors_only_stun() {
        let player = ScriptSkillService::stone_fling_status_requests(true, 1);
        assert_eq!(
            player
                .iter()
                .map(|request| (request.kind, request.rate, request.duration_ms))
                .collect::<Vec<_>>(),
            vec![(StatusChangeKind::Stun, 500, 5000), (StatusChangeKind::Blind, 500, 30000)]
        );
        let other = ScriptSkillService::stone_fling_status_requests(false, 1);
        assert_eq!(other.len(), 1);
        assert_eq!(
            (other[0].kind, other[0].rate, other[0].duration_ms),
            (StatusChangeKind::Stun, 500, 5000)
        );
    }
}
