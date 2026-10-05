use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::status::StatusSnapshot;

use super::ScriptSkillService;
use super::actor::ScriptSkillActor;
use super::ground::{GroundKind, GroundSkillSource};
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterStatusAlternatives, GameEvent, ScriptSkillCast};
use crate::server::model::events::map_event::{MapEvent, MobStatusAlternatives, ScriptMobCombat};
use crate::server::model::map_item::MapItemType;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub(super) fn actor_area_targets(
        &self,
        server: &Server,
        state: &ServerState,
        source: &ScriptSkillActor,
        skill_id: u32,
        x: u16,
        y: u16,
        radius: u16,
    ) -> Vec<(u32, StatusSnapshot, bool)> {
        let mut targets = vec![];
        let Some(instance) = state.get_map_instance(&source.map, source.instance) else {
            return targets;
        };
        let map = instance.state();
        let allied_mob = map.get_mob(source.id).is_some_and(|mob| mob.summon_ai != 0);
        if source.object_type != MapItemType::Mob || allied_mob {
            targets.extend(
                map.mobs()
                    .values()
                    .filter(|mob| {
                        mob.hp() > 0
                            && mob.id != source.id
                            && (!mob.summoned || mob.summon_ai == 0)
                            && mob.x.abs_diff(x).max(mob.y.abs_diff(y)) <= radius
                    })
                    .map(|mob| (mob.id, mob.status.clone(), false)),
            );
        }
        for target in state.characters().values().filter(|target| {
            target.status.hp > 0
                && target.char_id != source.id
                && target.current_map_name() == &source.map
                && target.current_map_instance() == source.instance
                && target.x.abs_diff(x).max(target.y.abs_diff(y)) <= radius
        }) {
            if let Some(owner) = state.get_character(source.credit_id) {
                if !server.player_combat_target_allowed(state, owner, target.char_id) {
                    continue;
                }
            } else if allied_mob {
                continue;
            }
            targets.push((
                target.char_id,
                crate::server::service::status_service::StatusService::instance().to_snapshot(&target.status),
                true,
            ));
            for companion in crate::server::service::script_world_service::companion_snapshots(target) {
                if companion.x().abs_diff(x).max(companion.y().abs_diff(y)) <= radius {
                    if let Some(status) =
                        crate::server::service::script_world_service::companion_status_snapshot(target, companion.map_item().id())
                    {
                        targets.push((companion.map_item().id(), status, true));
                    }
                }
            }
        }
        if super::metadata::SkillMetadata::find(skill_id).is_some_and(|metadata| metadata.flags.get("TargetTrap").copied().unwrap_or(false))
        {
            targets.extend(
                state
                    .ground_units
                    .values()
                    .filter(|unit| {
                        unit.map == crate::server::model::map_instance::MapInstanceKey::new(source.map.clone(), source.instance)
                            && unit.alive(crate::util::tick::get_tick())
                            && !unit.used
                            && unit.x.abs_diff(x).max(unit.y.abs_diff(y)) <= radius
                    })
                    .map(|unit| (unit.id, unit.status(), false)),
            );
        }
        targets
    }

    pub(super) fn cast_actor_area_status(
        &self,
        server: &Server,
        state: &mut ServerState,
        source: &ScriptSkillActor,
        metadata: &SkillMetadata,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let radius = metadata.splash(level).unwrap_or(15);
        let radius = if radius < 0 { 15 } else { radius.min(u16::MAX as i32) as u16 };
        let instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Area unit skill map is unavailable")?;
        for (id, status, player) in self.actor_area_targets(server, state, source, metadata.id, x, y, radius) {
            if metadata.name == "AL_CRUCIS"
                && *status.element() != Element::Undead
                && !matches!(*status.race(), MobRace::Demon | MobRace::RUndead)
            {
                continue;
            }
            if metadata.name == "NPC_WIDESOULDRAIN" {
                let drain = Self::wide_soul_drain(level, status.sp());
                if let Some(character) = state.characters_mut().get_mut(&id) {
                    server
                        .character_service()
                        .update_hp_sp(character, character.status.hp, character.status.sp.saturating_sub(drain));
                } else if !player {
                    instance.add_to_next_tick(MapEvent::ScriptMobCombat(ScriptMobCombat {
                        source_id: source.id,
                        target_id: id,
                        effect: crate::server::service::script_combat_service::MobCombatEffect::Vanish { hp: 0, sp: drain },
                    }));
                }
                continue;
            }
            if metadata.name == "NPC_DRAGONFEAR" {
                let requests = Self::dragon_fear_requests(metadata.id, level, source.id, fastrand::usize(0..4));
                if player {
                    server.add_to_next_tick(GameEvent::CharacterStatusAlternatives(CharacterStatusAlternatives {
                        char_id: id,
                        requests,
                    }));
                } else {
                    instance.add_to_next_tick(MapEvent::MobStatusAlternatives(MobStatusAlternatives { mob_id: id, requests }));
                }
            } else if let Some((mut request, delay)) = Self::area_status_request(&metadata.name, metadata.id, level, false, 0) {
                if metadata.name == "AL_CRUCIS" {
                    request.rate = Self::signum_crucis_rate(level, source.status.base_level(), status.base_level());
                }
                if metadata.name.starts_with("NPC_") {
                    request.values[1] = source.id as i32;
                }
                self.start_actor_target_status(server, state, source, id, request, delay, tick)?;
            }
        }
        self.notify_actor_support(
            source,
            &ScriptSkillCast {
                source_id: source.id,
                target_id: source.id,
                skill_id: metadata.id,
                level: u16::from(level),
                ..Default::default()
            },
            true,
        );
        Ok(())
    }

    pub(super) fn place_script_actor_ground(
        &self,
        server: &Server,
        state: &ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(request.skill_id).ok_or("Unknown actor ground skill")?;
        let kind = GroundKind::from_name(&metadata.name).ok_or_else(|| format!("Actor ground unit {} is not implemented", metadata.name))?;
        if !kind.actor_placeable() {
            return Err(format!("{} still requires an actor-specific unit lifecycle", metadata.name));
        }
        if kind == GroundKind::Meteor {
            return self.place_actor_meteors(server, state, source, request, metadata, x, y, tick);
        }
        self.place_actor_ground_skill_with_options(
            server,
            state,
            GroundSkillSource {
                actor_id: source.id,
                owner_id: source.credit_id,
                map: source.map.clone(),
                instance: source.instance,
                x: source.x,
                y: source.y,
                status: source.status.clone(),
                raw_attack: source.raw_attack,
                fixed_damage: None,
            },
            request.skill_id,
            request.level as u8,
            x,
            y,
            tick,
            true,
            true,
        )
    }

    fn place_actor_meteors(
        &self,
        server: &Server,
        state: &ServerState,
        source: &ScriptSkillActor,
        request: &ScriptSkillCast,
        metadata: &SkillMetadata,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let level = request.level as u8;
        let duration = metadata.duration(level, false).unwrap_or(0).max(0) as u128;
        let interval = metadata.unit_value("Interval", level, "Time").unwrap_or(-1);
        let interval = if interval < 0 { 40 } else { interval.max(40) as u128 };
        let radius = metadata.splash(level).unwrap_or(3).max(0);
        let mut placed = 0;
        for number in 1..=(duration / interval).max(1) {
            let target_x = (i32::from(x) + fastrand::i32(-radius..=radius)).max(0) as u16;
            let target_y = (i32::from(y) + fastrand::i32(-radius..=radius)).max(0) as u16;
            let result = self.place_actor_ground_skill_with_options(
                server,
                state,
                GroundSkillSource {
                    actor_id: source.id,
                    owner_id: source.credit_id,
                    map: source.map.clone(),
                    instance: source.instance,
                    x: source.x,
                    y: source.y,
                    status: source.status.clone(),
                    raw_attack: source.raw_attack,
                    fixed_damage: None,
                },
                request.skill_id,
                level,
                target_x,
                target_y,
                tick + number * interval,
                true,
                true,
            );
            placed += usize::from(result.is_ok());
        }
        if placed == 0 {
            return Err("Meteor Storm found no usable cells".into());
        }
        Ok(())
    }
}
