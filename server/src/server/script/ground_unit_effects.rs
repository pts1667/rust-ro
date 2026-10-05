use models::enums::mob::MobRace;

use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use super::ground::GroundSkill;
use crate::server::state::character::Character;
use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;
use models::status::StatusSnapshot;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use super::metadata::SkillMetadata;

const SANCTUARY_BASE_HEAL_PER_LEVEL: u32 = 100;
const SANCTUARY_MAX_LEVEL_HEAL: u32 = 777;
const SANCTUARY_MAX_LEVEL_THRESHOLD: u8 = 6;
const SANCTUARY_EXTRA_TARGETS: u8 = 3;
const VENOM_DUST_DEFAULT_POISON_MS: i32 = 60_000;
pub(super) const GANBANTEIN_SUCCESS_PERCENT: u8 = 80;
const GANBANTEIN_RADIUS: u16 = 1;
const SPIDER_WEB_DEFAULT_MS: i32 = 8000;
const SPIDER_WEB_MAX_LAYERS: i32 = 3;

impl ScriptSkillService {
    fn sanctuary_base_heal(level: u8) -> u32 {
        if level > SANCTUARY_MAX_LEVEL_THRESHOLD {
            SANCTUARY_MAX_LEVEL_HEAL
        } else {
            u32::from(level) * SANCTUARY_BASE_HEAL_PER_LEVEL
        }
    }

    fn sanctuary_repels(target: &StatusSnapshot) -> bool {
        Self::undead_target(target) || *target.race() == MobRace::Demon
    }

    pub(super) fn tick_sanctuary(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick || tick < ground.next_hit_at {
            return;
        }
        ground.next_hit_at = ground.next_hit_at.saturating_add(ground.interval);
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        let source_character = if ground.actor_source.is_none() { state.get_character(ground.source_id) } else { None };
        let base = Self::sanctuary_base_heal(ground.level);
        let heal = match (&ground.actor_source, source_character) {
            (None, Some(source)) => {
                Self::scale_source_heal(source, &StatusService::instance().to_snapshot(&source.status), base)
            }
            _ => base,
        };
        let effect_for = |target_id: u32, action: ScriptSkillAction| ScriptSkillEffect {
            source_char_id: ground.source_id,
            target_id,
            skill_id: ground.skill_id,
            level: ground.level,
            heal_value: heal * 2,
            proc_depth: ground.depth,
            skill_event_emitted: true,
            cast_generation: 0,
            action,
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        };
        let budget = ground.level.saturating_add(SANCTUARY_EXTRA_TARGETS);
        let mobs = instance
            .state()
            .mobs()
            .values()
            .filter(|mob| mob.status.hp() > 0 && ground.covers(mob.x, mob.y))
            .map(|mob| (mob.id, mob.status.clone()))
            .collect::<Vec<_>>();
        for (mob_id, status) in mobs {
            if Self::sanctuary_repels(&status) {
                let (Some(source), None) = (source_character, &ground.actor_source) else {
                    continue;
                };
                if ground.waves >= budget {
                    continue;
                }
                let damage = self
                    .offensive_heal_damage(
                        server,
                        &StatusService::instance().to_snapshot(&source.status),
                        &status,
                        &effect_for(mob_id, ScriptSkillAction::Cast),
                        tick,
                    )
                    .with_skill_notification(source.current_map_name(), source.current_map_instance(), source.x, source.y, tick, 1, 0);
                instance.add_to_next_tick(MapEvent::MobDamage(damage));
                ground.waves = ground.waves.saturating_add(1);
            } else if status.hp() < status.max_hp() {
                instance.add_to_next_tick(MapEvent::MobHeal {
                    mob_id,
                    hp: Self::target_heal_amount(&status, heal),
                    sp: 0,
                });
            }
        }
        let players = state
            .characters()
            .values()
            .filter(|target| {
                target.status.hp > 0
                    && target.current_map_name() == &ground.map
                    && target.current_map_instance() == ground.instance
                    && ground.covers(target.x, target.y)
            })
            .map(|target| (target.char_id, StatusService::instance().to_snapshot(&target.status), target.status.hp))
            .collect::<Vec<_>>();
        for (char_id, snapshot, hp) in players {
            if Self::sanctuary_repels(&snapshot) || hp >= snapshot.max_hp() {
                continue;
            }
            let amount = Self::target_heal_amount(&snapshot, heal);
            server.add_to_next_tick(GameEvent::CharacterScriptSkill(effect_for(
                char_id,
                ScriptSkillAction::Heal { hp: amount, sp: 0 },
            )));
        }
        if ground.waves >= budget {
            ground.expires_at = tick;
        }
    }

    pub(super) fn tick_venom_dust(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick || tick < ground.next_hit_at {
            return;
        }
        ground.next_hit_at = ground.next_hit_at.saturating_add(ground.interval);
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        let duration = SkillMetadata::find(ground.skill_id)
            .and_then(|metadata| metadata.duration(ground.level, true))
            .unwrap_or(VENOM_DUST_DEFAULT_POISON_MS);
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Poison, duration, i32::from(ground.level));
        request.flags = 0;
        if ground.actor_source.is_none() {
            let poisoned = instance
                .state()
                .mobs()
                .values()
                .filter(|mob| {
                    mob.status.hp() > 0 && ground.covers(mob.x, mob.y) && !mob.status_effects.has_status_change(StatusChangeKind::Poison)
                })
                .map(|mob| mob.id)
                .collect::<Vec<_>>();
            for mob_id in poisoned {
                instance.add_to_next_tick(MapEvent::MobStatusChange { mob_id, request: request.clone() });
            }
            return;
        }
        let poisoned = state
            .characters()
            .values()
            .filter(|target| {
                target.status.hp > 0
                    && target.current_map_name() == &ground.map
                    && target.current_map_instance() == ground.instance
                    && ground.covers(target.x, target.y)
                    && !target.status.has_status_change(StatusChangeKind::Poison)
            })
            .map(|target| target.char_id)
            .collect::<Vec<_>>();
        for char_id in poisoned {
            server.add_to_next_tick(GameEvent::CharacterStatusChange(
                crate::server::model::events::game_event::CharacterStatusChange { char_id, request: request.clone() },
            ));
        }
    }

    pub(super) fn clear_ground_units(&self, character: &Character, x: u16, y: u16) {
        self.clear_ground_units_at(character.current_map_name(), character.current_map_instance(), x, y);
    }

    pub(super) fn clear_ground_units_at(&self, map: &str, instance: u8, x: u16, y: u16) {
        let Ok(mut grounds) = self.ground_skills.lock() else {
            return;
        };
        for ground in grounds.iter_mut().filter(|ground| ground.map == map && ground.instance == instance) {
            for cell in ground
                .cells
                .iter_mut()
                .filter(|cell| cell.x.abs_diff(x) <= GANBANTEIN_RADIUS && cell.y.abs_diff(y) <= GANBANTEIN_RADIUS)
            {
                cell.remaining_hits = 0;
            }
        }
    }

    pub(super) fn body_relocation(&self, server: &Server, state: &ServerState, character: &mut Character, x: u16, y: u16, tick: u128) {
        self.relocate_skill_actor(server, state, character, x, y, tick);
    }

    pub(super) fn tick_spider_web(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.triggered || ground.expires_at <= tick {
            return;
        }
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        let base = SkillMetadata::find(ground.skill_id)
            .and_then(|metadata| metadata.duration(ground.level, true))
            .unwrap_or(SPIDER_WEB_DEFAULT_MS);
        let trapped = if ground.actor_source.is_none() {
            instance
                .state()
                .mobs()
                .values()
                .find(|mob| mob.status.hp() > 0 && ground.covers(mob.x, mob.y))
                .map(|mob| (mob.id, false, mob.status_effects.status_change(StatusChangeKind::SpiderWeb).map(|change| change.values[0])))
        } else {
            state
                .characters()
                .values()
                .find(|target| {
                    target.status.hp > 0
                        && target.current_map_name() == &ground.map
                        && target.current_map_instance() == ground.instance
                        && ground.covers(target.x, target.y)
                })
                .map(|target| (target.char_id, true, target.status.status_change(StatusChangeKind::SpiderWeb).map(|change| change.values[0])))
        };
        let Some((target_id, player, layers)) = trapped else {
            return;
        };
        let layers = layers.unwrap_or(0);
        if layers >= SPIDER_WEB_MAX_LAYERS {
            ground.expires_at = tick;
            return;
        }
        let duration = if layers == 1 { base * 2 } else { base };
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::SpiderWeb, duration, layers + 1);
        request.flags = 0;
        if player {
            server.add_to_next_tick(GameEvent::CharacterStatusChange(
                crate::server::model::events::game_event::CharacterStatusChange { char_id: target_id, request },
            ));
        } else {
            instance.add_to_next_tick(MapEvent::MobStatusChange { mob_id: target_id, request });
        }
        ground.triggered = true;
        ground.expires_at = tick.saturating_add(duration.max(0) as u128);
    }
}
