use models::enums::EnumWithMaskValueU32;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::enums::skill_enums::SkillEnum;
use models::status::StatusSnapshot;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

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
                request(StatusChangeKind::Stun, 300, false),
                request(StatusChangeKind::Blind, 300, true),
            ]
        } else {
            vec![request(StatusChangeKind::Stun, 500, false)]
        }
    }

    pub fn secondary_status_requests(
        metadata: &SkillMetadata,
        level: u8,
        source_level: u32,
        fatal_blow: bool,
        assassin_spirit: bool,
        target: &StatusSnapshot,
        gust_hits: u8,
        joint: models::status_change::JointBreak,
    ) -> Vec<StatusChangeRequest> {
        use StatusChangeKind::*;
        let level_i = level as i32;
        let mut effects = vec![];
        let mut status = |kind, chance: i32, secondary: bool| {
            effects.push(StatusChangeRequest {
                kind,
                duration_ms: metadata.duration(level, secondary).unwrap_or(0),
                values: [level_i, 0, 0, 0],
                rate: chance.clamp(0, u16::MAX as i32) as u16,
                flags: 0,
            });
        };
        match metadata.name.as_str() {
            "TF_POISON" => status(Poison, (10 + 4 * level_i) * 100, true),
            "NPC_POISON" => status(Poison, 2000 * level_i, true),
            "NPC_HELLJUDGEMENT" => status(Curse, 10000, true),
            "AS_SPLASHER" => status(Poison, 10000, true),
            "MG_FROSTDIVER" => status(Freeze, (3 * level_i + 35).min(level_i + 60) * 100, true),
            "WZ_FROSTNOVA" => status(Freeze, (5 * level_i + 33) * 100, true),
            "AS_SONICBLOW" => status(Stun, (2 * level_i + 10) * if assassin_spirit { 200 } else { 100 }, true),
            "SM_BASH" if fatal_blow && level > 5 => status(Stun, (level_i - 5) * source_level.min(i32::MAX as u32) as i32 * 10, true),
            "WZ_METEOR" => status(Stun, 300 * level_i, true),
            "WZ_VERMILION" => status(Blind, (4 * level_i).min(40) * 100, true),
            "WZ_STORMGUST" if gust_hits >= 3 => status(Freeze, 15_000, true),
            "WS_CARTTERMINATION" => status(Stun, 500 * level_i, true),
            "MER_CRASH" => status(Stun, 600 * level_i, true),
            "MA_LANDMINE" | "HT_LANDMINE" => status(Stun, 1000, true),
            "MA_FREEZINGTRAP" | "HT_FREEZINGTRAP" => status(Freeze, 10000, true),
            "ML_SPIRALPIERCE" => status(Ankle, 10000, true),
            "SL_STUN" if *target.size() == models::enums::size::Size::Medium => status(Stun, (30 + 10 * level_i) * 100, false),
            "RG_RAID" => {
                status(Stun, (10 + 3 * level_i) * 100, false);
                status(Blind, (10 + 3 * level_i) * 100, true);
            }
            "CR_GRANDCROSS"
                if *target.element() == Element::Undead || *target.race() == MobRace::RUndead || *target.race() == MobRace::Demon =>
            {
                status(Blind, 10_000, true)
            }
            "LK_JOINTBEAT" => {
                let rate = ((50 * (level_i + 1) - 270 * target.str() as i32 / 100) * 10).clamp(0, u16::MAX as i32) as u16;
                if joint == models::status_change::JointBreak::Neck {
                    status(Bleeding, 10_000, true);
                }
                effects.push(StatusChangeRequest {
                    kind: JointBeat,
                    duration_ms: metadata.duration(level, true).unwrap_or(0),
                    values: [level_i, joint.as_flag() as i32, 0, 0],
                    rate,
                    flags: 0,
                });
            }
            _ => {}
        }
        effects
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
        let fatal_blow = !companion
            && snapshot
                .known_skills()
                .iter()
                .any(|skill| skill.value == SkillEnum::SmFatalblow && skill.level > 0);
        let assassin_spirit = snapshot
            .status_change(StatusChangeKind::Spirit)
            .is_some_and(|change| change.values[1] == SkillEnum::SlAssasin.id() as i32);
        let target = state
            .get_character(hit.target_id)
            .map(|character| StatusService::instance().to_snapshot(&character.status))
            .or_else(|| {
                state
                    .map_item(hit.target_id, &map, instance_id)
                    .and_then(|item| state.map_item_mob_status(&item, &map, instance_id))
            });
        if metadata.name == "NPC_VAMPIRE_GIFT" {
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
        let joints = [
            models::status_change::JointBreak::Ankle,
            models::status_change::JointBreak::Wrist,
            models::status_change::JointBreak::Knee,
            models::status_change::JointBreak::Shoulder,
            models::status_change::JointBreak::Waist,
            models::status_change::JointBreak::Neck,
        ];
        let joint = if target.active_statuses().iter().any(|change| {
            change.kind == StatusChangeKind::JointBeat && change.values[1] as u32 & models::status_change::JointBreak::Neck.as_flag() != 0
        }) {
            models::status_change::JointBreak::Neck
        } else {
            joints[fastrand::usize(0..joints.len())]
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
            for request in Self::secondary_status_requests(
                metadata,
                hit.skill_level,
                source_level,
                fatal_blow,
                assassin_spirit,
                &target,
                gust_hits,
                joint,
            ) {
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

#[cfg(test)]
mod tests {
    use models::status_change::JointBreak;

    use super::*;

    #[test]
    fn classic_additional_effects_keep_independent_probabilities_and_gust_threshold() {
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let frost = SkillMetadata::find(SkillEnum::MgFrostdiver.id()).unwrap();
        let effects = ScriptSkillService::secondary_status_requests(frost, 10, 99, false, false, &target, 0, JointBreak::Ankle);
        assert_eq!(effects[0].rate, 6500);
        let gust = SkillMetadata::find(SkillEnum::WzStormgust.id()).unwrap();
        assert!(ScriptSkillService::secondary_status_requests(gust, 10, 99, false, false, &target, 2, JointBreak::Ankle).is_empty());
        assert_eq!(
            ScriptSkillService::secondary_status_requests(gust, 10, 99, false, false, &target, 3, JointBreak::Ankle)[0].rate,
            15000
        );
    }

    #[test]
    fn npc_poison_applies_its_secondary_duration_and_level_chance_after_a_hit() {
        let metadata = SkillMetadata::all().iter().find(|skill| skill.name == "NPC_POISON").unwrap();
        let target = StatusSnapshot::_from(&models::status::Status::default());
        let requests = ScriptSkillService::secondary_status_requests(metadata, 3, 20, false, false, &target, 0, JointBreak::Ankle);
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
            vec![(StatusChangeKind::Stun, 300, 5000), (StatusChangeKind::Blind, 300, 30000)]
        );
        let other = ScriptSkillService::stone_fling_status_requests(false, 1);
        assert_eq!(other.len(), 1);
        assert_eq!(
            (other[0].kind, other[0].rate, other[0].duration_ms),
            (StatusChangeKind::Stun, 500, 5000)
        );
    }
}
