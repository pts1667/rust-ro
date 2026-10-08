use models::enums::EnumWithMaskValueU32;
use models::enums::mob::{MobMode, MobRace};
use models::enums::skill_enums::SkillEnum;

use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use super::ground::GroundSkill;
use crate::server::state::character::Character;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterDamage, CharacterEndStatus, CharacterSpDrain, CharacterStatusChange, GameEvent};
use crate::server::model::events::map_event::{MapEvent, MobDamage, MobHeal, MobKnockback, MobStatusChange};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_instance::MapInstanceKey;
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
const EVIL_LAND_DEFAULT_BLIND_MS: i32 = 30_000;
const FOG_BLIND_MS: i32 = 10_000;
/// Marks the blindness Fog Wall applies, so leaving the fog only removes that blindness.
const FOG_BLIND_MARK: i32 = 954;
const WARM_SP_PER_MOB_HIT: u32 = 2;
const WARM_SP_PER_PLAYER_TICK: u32 = 10;
pub(super) const GANBANTEIN_SUCCESS_PERCENT: u8 = 80;
const GANBANTEIN_RADIUS: u16 = 1;
const SPIDER_WEB_DEFAULT_MS: i32 = 8000;
const SPIDER_WEB_MAX_LAYERS: i32 = 3;
const BODY_RELOCATION_ASURA_BLOCK_MS: u128 = 2000;

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
            (None, Some(source)) => Self::scale_fixed_heal(&StatusService::instance().to_snapshot(&source.status), base),
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
                instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage }));
                ground.waves = ground.waves.saturating_add(1);
            } else if status.hp() < status.max_hp() {
                instance.add_to_next_tick(MapEvent::MobHeal(MobHeal {
                    mob_id,
                    hp: Self::target_heal_amount(&status, heal),
                    sp: 0,
                }));
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
            if Self::sanctuary_repels(&snapshot) {
                let (Some(source), None) = (source_character, &ground.actor_source) else {
                    continue;
                };
                if ground.waves >= budget || !server.player_ground_target_allowed(state, source, char_id, true) {
                    continue;
                }
                let damage = self
                    .offensive_heal_damage(
                        server,
                        &StatusService::instance().to_snapshot(&source.status),
                        &snapshot,
                        &effect_for(char_id, ScriptSkillAction::Cast),
                        tick,
                    )
                    .with_skill_notification(source.current_map_name(), source.current_map_instance(), source.x, source.y, tick, 1, 0);
                server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage }));
                ground.waves = ground.waves.saturating_add(1);
                continue;
            }
            if hp >= snapshot.max_hp() {
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
                instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id, request: request.clone() }));
            }
        }
        let owner = if ground.actor_source.is_none() { state.get_character(ground.source_id) } else { None };
        let poisoned = state
            .characters()
            .values()
            .filter(|target| {
                target.status.hp > 0
                    && target.current_map_name() == &ground.map
                    && target.current_map_instance() == ground.instance
                    && ground.covers(target.x, target.y)
                    && !target.status.has_status_change(StatusChangeKind::Poison)
                    && (ground.actor_source.is_some() || owner.is_some_and(|owner| server.player_ground_target_allowed(state, owner, target.char_id, true)))
            })
            .map(|target| target.char_id)
            .collect::<Vec<_>>();
        for char_id in poisoned {
            server.add_to_next_tick(GameEvent::CharacterStatusChange(
                crate::server::model::events::game_event::CharacterStatusChange { char_id, request: request.clone() },
            ));
        }
    }

    pub(super) fn tick_evil_land(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick || tick < ground.next_hit_at {
            return;
        }
        ground.next_hit_at = ground.next_hit_at.saturating_add(ground.interval);
        let duration = SkillMetadata::find(ground.skill_id)
            .and_then(|metadata| metadata.duration(ground.level, true))
            .unwrap_or(EVIL_LAND_DEFAULT_BLIND_MS);
        let request = StatusChangeRequest::guaranteed(StatusChangeKind::Blind, duration, i32::from(ground.level));
        let blinded = state
            .characters()
            .values()
            .filter(|target| {
                target.status.hp > 0
                    && target.current_map_name() == &ground.map
                    && target.current_map_instance() == ground.instance
                    && ground.covers(target.x, target.y)
                    && !target.status.has_status_change(StatusChangeKind::Blind)
            })
            .map(|target| target.char_id)
            .collect::<Vec<_>>();
        for char_id in blinded {
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
        if self.relocate_skill_actor(server, state, character, x, y, tick) {
            character
                .script_skill_state
                .skill_blocked_until
                .insert(SkillEnum::MoExtremityfist.id(), tick + BODY_RELOCATION_ASURA_BLOCK_MS);
        }
    }

    /// Shadow Leap moves the caster to the chosen cell and ends Hiding whether or not the move succeeds.
    pub(super) fn shadow_leap(&self, server: &Server, state: &ServerState, character: &mut Character, x: u16, y: u16, tick: u128) {
        self.relocate_skill_actor(server, state, character, x, y, tick);
        crate::server::service::status_effect_service::StatusEffectService::end(
            server,
            character,
            Some(StatusChangeKind::Hiding),
            tick,
            &self.client_notification_sender,
        );
    }

    /// Warm moves with its caster, knocks back monsters inside it and drains SP from hostile players.
    pub(super) fn tick_warm(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        let group = ground.cells.first().map(|cell| cell.id).unwrap_or(0) as i32;
        let Some(caster) = state.get_character(ground.source_id) else {
            ground.expires_at = tick;
            return;
        };
        let warm_active = caster.status.status_change(StatusChangeKind::Warm).is_some_and(|change| change.values[3] == group);
        if !warm_active || caster.current_map_name() != &ground.map || caster.current_map_instance() != ground.instance {
            ground.expires_at = tick;
            return;
        }
        ground.source_x = caster.x;
        ground.source_y = caster.y;
        if let Some(cell) = ground.cells.first_mut() {
            cell.x = caster.x;
            cell.y = caster.y;
        }
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        let flags = state.map_flags(&MapInstanceKey::new(ground.map.clone(), ground.instance));
        let hostile_players = flags.enabled(MapFlag::Pvp) || flags.is_gvg();
        let mut caster_sp = caster.status.sp;
        for mob in instance.state().mobs().values().filter(|mob| mob.status.hp() > 0 && ground.covers(mob.x, mob.y)) {
            if mob.mode & MobMode::Boss.as_flag() != 0 && fastrand::u8(0..5) != 0 {
                continue;
            }
            if caster_sp < WARM_SP_PER_MOB_HIT {
                ground.expires_at = tick;
                break;
            }
            caster_sp -= WARM_SP_PER_MOB_HIT;
            server.add_to_next_tick(GameEvent::CharacterSpDrain(CharacterSpDrain {
                char_id: caster.char_id,
                amount: WARM_SP_PER_MOB_HIT,
            }));
            instance.add_to_next_tick(MapEvent::MobKnockback(MobKnockback {
                mob_id: mob.id,
                source_x: caster.x,
                source_y: caster.y,
                cells: 2 + fastrand::u16(0..=3),
            }));
        }
        if hostile_players {
            for target in state.characters().values().filter(|target| {
                target.char_id != caster.char_id
                    && target.status.hp > 0
                    && target.current_map_name() == &ground.map
                    && target.current_map_instance() == ground.instance
                    && ground.covers(target.x, target.y)
                    && (caster.game_systems.party_id == 0 || target.game_systems.party_id != caster.game_systems.party_id)
            }) {
                server.add_to_next_tick(GameEvent::CharacterSpDrain(CharacterSpDrain {
                    char_id: target.char_id,
                    amount: WARM_SP_PER_PLAYER_TICK,
                }));
            }
        }
    }

    /// Fog Wall blinds enemies inside it. Players lose that blindness when they leave the fog; monsters keep it until it ends.
    pub(super) fn tick_fog_wall(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick {
            return;
        }
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        let caster_party = state.get_character(ground.source_id).map_or(0, |caster| caster.game_systems.party_id);
        let flags = state.map_flags(&MapInstanceKey::new(ground.map.clone(), ground.instance));
        let hostile_players = flags.enabled(MapFlag::Pvp) || flags.is_gvg();
        for mob in instance.state().mobs().values().filter(|mob| {
            mob.status.hp() > 0
                && ground.covers(mob.x, mob.y)
                && !mob.status_effects.has_status_change(StatusChangeKind::Blind)
                && !mob.status_effects.has_status_change(StatusChangeKind::Deluge)
        }) {
            instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange {
                mob_id: mob.id,
                request: Self::fog_blind_request(ground.level),
            }));
        }
        for target in state.characters().values().filter(|target| {
            target.char_id != ground.source_id
                && target.status.hp > 0
                && target.current_map_name() == &ground.map
                && target.current_map_instance() == ground.instance
        }) {
            let covered = ground.covers(target.x, target.y);
            let enemy = hostile_players && (caster_party == 0 || target.game_systems.party_id != caster_party);
            if covered && enemy && !target.status.has_status_change(StatusChangeKind::Blind) && !target.status.has_status_change(StatusChangeKind::Deluge) {
                server.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange {
                    char_id: target.char_id,
                    request: Self::fog_blind_request(ground.level),
                }));
            } else if !covered && target.status.status_change(StatusChangeKind::Blind).is_some_and(|change| change.values[3] == FOG_BLIND_MARK) {
                server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus {
                    char_id: target.char_id,
                    kind: Some(StatusChangeKind::Blind),
                }));
            }
        }
    }

    fn fog_blind_request(level: u8) -> StatusChangeRequest {
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Blind, FOG_BLIND_MS, i32::from(level));
        request.values[3] = FOG_BLIND_MARK;
        request
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
        let duration = spider_web_duration(base, layers, instance.state().flags.versus(state.siege_active()));
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::SpiderWeb, duration, layers + 1);
        request.flags = 0;
        if player {
            server.add_to_next_tick(GameEvent::CharacterStatusChange(
                crate::server::model::events::game_event::CharacterStatusChange { char_id: target_id, request },
            ));
        } else {
            instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: target_id, request }));
        }
        ground.triggered = true;
        ground.expires_at = tick.saturating_add(duration.max(0) as u128);
    }
}

/// Duration in PvM is 1st 8s, 2nd 16s, 3rd 8s; versus maps halve the base and grow by layer: 4s, 8s, 12s.
fn spider_web_duration(base: i32, layers: i32, versus: bool) -> i32 {
    let base = if versus { base / 2 } else { base };
    let growing_layers = if versus { 3 } else { 2 };
    if layers > 0 && layers < growing_layers { base * (layers + 1) } else { base }
}

#[cfg(test)]
mod spider_web_tests {
    use super::spider_web_duration;

    #[test]
    fn layers_stack_durations_per_map_type() {
        let monster_map: Vec<i32> = (0..3).map(|layers| spider_web_duration(8000, layers, false)).collect();
        assert_eq!(monster_map, [8000, 16000, 8000]);
        let versus_map: Vec<i32> = (0..3).map(|layers| spider_web_duration(8000, layers, true)).collect();
        assert_eq!(versus_map, [4000, 8000, 12000]);
    }
}
