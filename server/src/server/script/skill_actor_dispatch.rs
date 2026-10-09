use models::enums::skill_enums::SkillEnum;
use models::status::StatusSnapshot;
use script_sdk::Value;

use super::ScriptSkillService;
use super::monster::MonsterSkill;
use super::actor::{self, MapActorSkillCast, NpcSkillState, ScriptActorSkillCompletion, ScriptSkillActor};
use super::metadata::SkillMetadata;
use super::requirements::DeferredSkillPayment;
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, ScriptSkillCast};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_item::{MapItemSnapshot, MapItemType, ToMapItemSnapshot};
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub fn find_script_skill_actor(&self, state: &ServerState, actor_id: u32) -> Option<ScriptSkillActor> {
        self.find_script_skill_actor_in(state, actor_id, None, None).ok().flatten()
    }

    pub fn find_script_skill_actor_in(
        &self,
        state: &ServerState,
        actor_id: u32,
        map: Option<&str>,
        instance_id: Option<u8>,
    ) -> Result<Option<ScriptSkillActor>, String> {
        if let Some(character) = state.get_character(actor_id) {
            return Ok(Some(ScriptSkillActor {
                id: character.char_id,
                credit_id: character.char_id,
                object_type: MapItemType::Character,
                map: character.current_map_name().clone(),
                instance: character.current_map_instance(),
                x: character.x,
                y: character.y,
                dir: character.dir,
                status: StatusService::instance().to_snapshot(&character.status),
                raw_attack: 0,
                mode: 0,
                attack_motion: 0,
            }));
        }
        let mut npcs = Vec::new();
        for instances in state.map_instances().values() {
            for instance in instances {
                if let Some(source) = actor::map_actor(&instance.state(), actor_id) {
                    if source.object_type != MapItemType::Npc {
                        return Ok(Some(source));
                    }
                    npcs.push(source);
                } else if let Some(npc) = instance.get_script(actor_id) {
                    npcs.push(NpcSkillState::new(&npc).actor(instance.key().map_name().clone(), instance.id()));
                }
            }
        }
        if !npcs.is_empty() {
            if let Some(map) = map {
                npcs.retain(|npc| npc.map.trim_end_matches(".gat") == map.trim_end_matches(".gat"));
            }
            if let Some(instance_id) = instance_id {
                npcs.retain(|npc| npc.instance == instance_id);
            }
            if npcs.len() > 1 {
                return Err("NPC actor ID is ambiguous without its map instance".into());
            }
            return Ok(npcs.pop());
        }
        for owner in state.characters().values() {
            if let Some(snapshot) = crate::server::service::script_world_service::companion_status_snapshot(owner, actor_id) {
                let item = crate::server::service::script_world_service::companion_snapshots(owner)
                    .into_iter()
                    .find(|actor| actor.map_item().id() == actor_id)
                    .ok_or("Companion actor has no position")?;
                return Ok(Some(ScriptSkillActor {
                    id: actor_id,
                    credit_id: owner.char_id,
                    object_type: *item.map_item().object_type(),
                    map: owner.current_map_name().clone(),
                    instance: owner.current_map_instance(),
                    x: item.x(),
                    y: item.y(),
                    dir: item.position().dir,
                    status: snapshot,
                    raw_attack: 0,
                    mode: 0,
                    attack_motion: 0,
                }));
            }
        }
        Ok(None)
    }

    pub(super) fn actor_target_status(
        &self,
        state: &ServerState,
        source: &ScriptSkillActor,
        target_id: u32,
    ) -> Result<(MapItemSnapshot, StatusSnapshot), String> {
        let target = state
            .map_item_snapshot(target_id, &source.map, source.instance)
            .ok_or("Unit skill target is not on this map")?;
        let status = if let Some(character) = state.get_character(target_id) {
            if character.current_map_name() != &source.map || character.current_map_instance() != source.instance {
                return Err("Unit skill target is on another map instance".into());
            }
            Some(StatusService::instance().to_snapshot(&character.status))
        } else {
            state.map_item_mob_status(&target.map_item(), &source.map, source.instance)
        };
        Ok((target, status.ok_or("Unit skill target has no battle status")?))
    }

    pub fn cast_script_unit_skill(
        &self,
        server: &Server,
        state: &mut ServerState,
        request: ScriptSkillCast,
        tick: u128,
    ) -> Result<(), String> {
        let mut source = self
            .find_script_skill_actor_in(state, request.source_id, request.source_map.as_deref(), request.source_instance)?
            .ok_or("Unit skill caster is not in the game")?;
        if source.object_type == MapItemType::Npc {
            let map = state
                .get_map_instance(&source.map, source.instance)
                .ok_or("NPC cast map is unavailable")?;
            let npc = map.state().script_skill_state.npcs.get(&source.id).cloned();
            if let Some(mut npc) = npc {
                npc.initialize_for_cast();
                source = npc.actor(source.map.clone(), source.instance);
            }
        }
        let metadata = SkillMetadata::find(request.skill_id).ok_or("Unknown pre-renewal unit skill")?;
        let level = u8::try_from(request.level).map_err(|_| "Invalid unit skill level")?;
        if level == 0 || level > metadata.max_level {
            return Err("Unit skill level exceeds its classic definition".into());
        }
        if state.ground_unit(request.target_id, &source.map, source.instance).is_some()
            && metadata.target_type.as_deref() != Some("Trap")
            && !metadata.flags.get("TargetTrap").copied().unwrap_or(false)
        {
            return Err("Unit skill cannot target trap units".into());
        }
        if request.message_id.is_some_and(|message| message > 0) && source.object_type != MapItemType::Mob {
            return Err("Unit skill messages require a monster caster".into());
        }
        if !source.can_cast(request.skill_id) {
            return Err("Unit skill source cannot cast now".into());
        }
        match source.object_type {
            MapItemType::Character => self.start_player_unit_cast(server, state, request, source, tick),
            MapItemType::Homunculus | MapItemType::Mercenary => server
                .script_world_service()
                .start_scripted_companion_skill(server, state, request, tick),
            MapItemType::Mob | MapItemType::Npc => {
                Self::validate_script_actor_operation(metadata, &source, level)?;
                let instance = state
                    .get_map_instance(&source.map, source.instance)
                    .ok_or("Unit skill map is unavailable")?;
                let target = if request.ground.is_some() {
                    None
                } else {
                    Some(self.actor_target_status(state, &source, request.target_id)?.0)
                };
                let npc = (source.object_type == MapItemType::Npc)
                    .then(|| instance.get_script(source.id).map(|script| NpcSkillState::new(&script)))
                    .flatten();
                let cast = MapActorSkillCast {
                    request,
                    source,
                    target,
                    npc,
                };
                instance.add_to_next_tick(MapEvent::ActorSkillCast(cast));
                Ok(())
            }
            _ => Err("This actor cannot cast skills".into()),
        }
    }

    fn start_player_unit_cast(
        &self,
        server: &Server,
        state: &mut ServerState,
        request: ScriptSkillCast,
        source: ScriptSkillActor,
        tick: u128,
    ) -> Result<(), String> {
        let mut character = state.characters_mut().remove(&source.id).ok_or("Player caster disconnected")?;
        let result = (|| {
            let metadata = SkillMetadata::find(request.skill_id).unwrap();
            let level = request.level as u8;
            let skill = self
                .configuration
                .find_skill_config(&Value::Number(request.skill_id as i32))
                .ok_or("Unknown unit skill")?;
            self.validate_skill(skill, u32::from(level))?;
            if character.game_systems.is_trading()
                || character.is_using_skill()
                || character.script_skill_state.casting_until > tick
                || character.timing.get_canact_tick() > tick
            {
                return Err("Player cannot begin another cast yet".into());
            }
            Self::validate_skill_map(state, &character, request.skill_id, level, false)?;
            Self::validate_stealth_cast(state, &character, request.skill_id)?;
            if let Some((x, y)) = request.ground {
                if !request.ignore_range {
                    self.validate_ground_target(state, &character, request.skill_id, level, x, y, tick)?;
                }
                let instance = state
                    .get_map_instance_from_character(&character)
                    .ok_or("Unit skill map is unavailable")?;
                if !actor::usable_terrain(&instance.state(), (x, y), (x, y)) {
                    return Err("Unit skill ground is unusable".into());
                }
            } else {
                let target = if request.target_id == character.char_id {
                    character.to_map_item_snapshot()
                } else {
                    state
                        .map_item_snapshot(request.target_id, &source.map, source.instance)
                        .ok_or("Unit skill target is not on this map")?
                };
                self.validate_damage_target(state, &character, request.skill_id, request.target_id)?;
                self.validate_support_target(state, &character, request.skill_id, level, request.target_id)?;
                if !server.player_skill_target_allowed(state, &character, request.target_id, request.skill_id, false) {
                    return Err("Unit skill cannot target this actor".into());
                }
                if !request.ignore_range
                    && (character.x.abs_diff(target.x()).max(character.y.abs_diff(target.y()))
                        > self.player_skill_range(&source.status, request.skill_id, level)
                        || !actor::usable_terrain(
                            &state
                                .get_map_instance_from_character(&character)
                                .ok_or("Unit skill map is unavailable")?
                                .state(),
                            (character.x, character.y),
                            (target.x(), target.y()),
                        ))
                {
                    return Err("Unit skill target is out of range or obstructed".into());
                }
            }
            let requirements = self.requirements_plan(&character, request.skill_id, level, tick)?;
            self.validate_native_environment(state, &character, request.skill_id, level, tick)?;
            let base = metadata.cast_duration(level, StatusService::skill_cast_modifier(&source.status, request.skill_id));
            let duration = (base.min(i64::MAX as u128) as i64 + i64::from(request.cast_time_adjust_ms)).max(0) as u128;
            if super::ScriptSkillService::is_warp_portal(request.skill_id) {
                let (x, y) = request.ground.ok_or("Warp Portal requires a ground position")?;
                character.script_skill_state.deferred_requirements = Some(DeferredSkillPayment {
                    skill_id: request.skill_id,
                    level,
                    keep_requirements: true,
                    requirements,
                    source_index: None,
                    source_item: None,
                });
                character.script_skill_state.cast_cancel_override =
                    Some(duration > 0 && request.cast_cancel.unwrap_or(metadata.cast_cancel.unwrap_or(true)));
                return self.start_warp_portal_menu(
                    server,
                    state,
                    &mut character,
                    request.skill_id,
                    level,
                    x,
                    y,
                    tick,
                    false,
                    0,
                    Some(duration),
                );
            }
            self.end_cloaking_on_skill(server, &mut character, request.skill_id, tick);
            character.movements.clear();
            character.clear_attack();
            character.script_skill_state.cast_generation = character.script_skill_state.cast_generation.wrapping_add(1).max(1);
            character.script_skill_state.casting_until = tick + duration.max(1);
            character.script_skill_state.casting_skill_id = request.skill_id;
            character.script_skill_state.casting_skill_level = level;
            character.script_skill_state.cast_cancel_override =
                Some(duration > 0 && request.cast_cancel.unwrap_or(metadata.cast_cancel.unwrap_or(true)));
            actor::notify_actor(
                &self.client_notification_sender,
                &source,
                actor::casting_packet(&source, &request, duration),
            );
            server.add_to_tick(
                GameEvent::ScriptActorSkillComplete(ScriptActorSkillCompletion {
                    generation: character.script_skill_state.cast_generation,
                    payment: Some(DeferredSkillPayment {
                        skill_id: request.skill_id,
                        level,
                        keep_requirements: true,
                        requirements,
                        source_index: None,
                        source_item: None,
                    }),
                    request,
                    source,
                }),
                duration.div_ceil(40).max(1).saturating_sub(1) as usize,
            );
            Ok(())
        })();
        state.insert_character(character);
        result
    }

    pub fn finish_script_actor_skill(
        &self,
        server: &Server,
        state: &mut ServerState,
        completion: ScriptActorSkillCompletion,
        tick: u128,
    ) -> Result<(), String> {
        let request = &completion.request;
        let source = self
            .find_script_skill_actor_in(
                state,
                request.source_id,
                Some(&completion.source.map),
                Some(completion.source.instance),
            )?
            .ok_or("Unit skill caster left the map")?;
        if source.map != completion.source.map || source.instance != completion.source.instance || !source.can_cast(request.skill_id) {
            return Err("Unit skill caster changed maps or was incapacitated".into());
        }
        if source.object_type == MapItemType::Character {
            let mut character = state.characters_mut().remove(&source.id).ok_or("Player caster disconnected")?;
            let result = (|| {
                if character.script_skill_state.cast_generation != completion.generation {
                    return Err("Unit skill was interrupted".into());
                }
                Self::validate_skill_map(state, &character, request.skill_id, request.level as u8, false)?;
                if request.ground.is_none()
                    && !server.player_skill_target_allowed(state, &character, request.target_id, request.skill_id, true)
                {
                    return Err("Unit skill target is no longer eligible".into());
                }
                let payment = completion.payment.as_ref().ok_or("Unit skill has no resource receipt")?;
                if let Some((x, y)) = request.ground {
                    let instance = state
                        .get_map_instance_from_character(&character)
                        .ok_or("Unit skill map is unavailable")?;
                    if !actor::usable_terrain(&instance.state(), (x, y), (x, y)) {
                        return Err("Unit skill ground became unusable".into());
                    }
                } else if request.target_id != source.id {
                    let (position, target) = self.actor_target_status(state, &source, request.target_id)?;
                    if target.hp() == 0 && !matches!(super::ScriptSkillService::skill_behaviour(request.skill_id, request.level as u8), skills::ActorBehaviour::Resurrect) {
                        return Err("Unit skill target died".into());
                    }
                    if !request.ignore_range
                        && (source.x.abs_diff(position.x()).max(source.y.abs_diff(position.y()))
                            > self.player_skill_range(&source.status, request.skill_id, request.level as u8)
                            || !actor::usable_terrain(
                                &state
                                    .get_map_instance(&source.map, source.instance)
                                    .ok_or("Unit skill map is unavailable")?
                                    .state(),
                                (source.x, source.y),
                                (position.x(), position.y()),
                            ))
                    {
                        return Err("Unit skill target moved out of range or behind an obstacle".into());
                    }
                }
                if super::ScriptSkillService::skill_object_by_id(request.skill_id).is_some_and(|skill| skill.conditional_completion()) {
                    let metadata = SkillMetadata::find(request.skill_id).unwrap();
                    let delay = metadata
                        .after_cast_act_delay
                        .as_ref()
                        .and_then(|delay| delay.value(request.level as u8, "Time"))
                        .unwrap_or(0)
                        .max(0) as u32;
                    character
                        .timing
                        .set_canact_tick(tick + u128::from(StatusService::skill_after_cast_delay(&source.status, request.skill_id, delay)));
                    server.add_to_next_tick(GameEvent::CharacterScriptSkill(super::ScriptSkillEffect {
                        source_char_id: source.id,
                        target_id: request.target_id,
                        skill_id: request.skill_id,
                        level: request.level as u8,
                        heal_value: 0,
                        proc_depth: 0,
                        skill_event_emitted: false,
                        cast_generation: completion.generation,
                        action: super::ScriptSkillAction::Cast,
                        deferred_requirements: Some(payment.requirements.clone()),
                        prepared_outcome: None,
                        source_index: None,
                        source_item: None,
                    }));
                    return Ok(());
                }
                server
                    .item_service()
                    .pay_requirement_plan(server, &mut character, &payment.requirements, None, tick)?;
                character.script_skill_state.casting_until = 0;
                character.script_skill_state.cast_cancel_override = None;
                character.script_skill_state.cast_generation = character.script_skill_state.cast_generation.wrapping_add(1);
                if let Some((x, y)) = request.ground {
                    self.place_ground_skill_depth(
                        server,
                        state,
                        &mut character,
                        request.skill_id,
                        request.level as u8,
                        x,
                        y,
                        tick,
                        true,
                        0,
                    )?;
                } else {
                    self.cast_skill_depth(
                        server,
                        state,
                        &mut character,
                        request.skill_id,
                        request.level as u8,
                        request.target_id,
                        false,
                        tick,
                        true,
                        0,
                    )?;
                }
                let metadata = SkillMetadata::find(request.skill_id).unwrap();
                let delay = metadata
                    .after_cast_act_delay
                    .as_ref()
                    .and_then(|delay| delay.value(request.level as u8, "Time"))
                    .unwrap_or(0)
                    .max(0) as u32;
                character
                    .timing
                    .set_canact_tick(tick + u128::from(StatusService::skill_after_cast_delay(&source.status, request.skill_id, delay)));
                crate::server::service::script_combat_service::emit(
                    server,
                    source.id,
                    request.target_id,
                    models::status_bonus::CombatTrigger::Skill,
                    metadata.battle_flags(metadata.range(request.level as u8).unwrap_or(1) > 3),
                    request.skill_id,
                    0,
                );
                Ok(())
            })();
            state.insert_character(character);
            return result;
        }
        let instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Unit skill map is unavailable")?;
        if instance.state().script_skill_state.generations.get(&source.id).copied() != Some(completion.generation) {
            return Err("Unit skill completion no longer belongs to this caster".into());
        }
        match self.execute_actor_skill(server, state, &source, request, tick) {
            Err(error) if actor::is_expected_rejection(&error) => {
                script_debug!("Unit skill {} from {} was not performed: {}", request.skill_id, source.id, error);
                self.notify_actor_support(&source, request, false);
                Ok(())
            }
            result => result,
        }
    }

    pub(crate) fn validate_script_actor_operation(metadata: &SkillMetadata, source: &ScriptSkillActor, level: u8) -> Result<(), String> {
        use skills::ActorBehaviour;
        let direct_support = match Self::actor_behaviour(metadata, level) {
            ActorBehaviour::Default
            | ActorBehaviour::Splash(_)
            | ActorBehaviour::Delayed { .. }
            | ActorBehaviour::FixedWeapon { .. }
            | ActorBehaviour::Magic(_)
            | ActorBehaviour::Devotion
            | ActorBehaviour::VenomSplasher
            | ActorBehaviour::WinkCharm
            | ActorBehaviour::Tarot
            | ActorBehaviour::SpellBreaker
            | ActorBehaviour::Estimation
            | ActorBehaviour::Identify
            | ActorBehaviour::HocusPocus
            | ActorBehaviour::FindStone
            | ActorBehaviour::EnchantArms
            | ActorBehaviour::ReverseOrcish
            | ActorBehaviour::ShadowLeap
            | ActorBehaviour::CondensedPotion
            | ActorBehaviour::Cultivate
            | ActorBehaviour::CollectItems
            | ActorBehaviour::BondBaby
            | ActorBehaviour::Spheres(_)
            | ActorBehaviour::HighJump
            | ActorBehaviour::Mission
            | ActorBehaviour::Run
            | ActorBehaviour::ServiceCall(_)
            | ActorBehaviour::TrapControl { .. }
            | ActorBehaviour::Martyr
            | ActorBehaviour::Intimidate
            | ActorBehaviour::Pressure
            | ActorBehaviour::Benedictio
            | ActorBehaviour::PoisonReact
            | ActorBehaviour::Bowling
            | ActorBehaviour::WaterBall
            | ActorBehaviour::Support(_)
            | ActorBehaviour::UndeadBuffDamage
            | ActorBehaviour::PartyBuff
            | ActorBehaviour::Performance(_)
            | ActorBehaviour::Adaptation
            | ActorBehaviour::LongingFreedom
            | ActorBehaviour::Encore
            | ActorBehaviour::CastCancel => false,
            ActorBehaviour::Teleport => matches!(source.object_type, MapItemType::Mob | MapItemType::Npc),
            _ => true,
        } || metadata.monster_skill().is_some_and(MonsterSkill::is_direct_support);
        if direct_support
            || Self::status_for_metadata(metadata).is_some()
            || Self::metadata_magic(metadata)
            || Self::actor_npc_magic(metadata)
            || Self::actor_npc_weapon(metadata)
            || Self::actor_metadata_status(metadata)
        {
            return Ok(());
        }
        if let Some(kind) = Self::ground_kind(metadata, level) {
            return if kind.actor_placeable() {
                Ok(())
            } else {
                Err(format!("{} requires an actor-specific ground lifecycle", metadata.name))
            };
        }
        if metadata.target_type.as_deref() == Some("Ground") {
            return Err(format!("{} requires an actor-specific ground lifecycle", metadata.name));
        }
        if Self::player_only_callback(metadata, level) {
            return Err(format!("{} requires an additional actor-specific callback", metadata.name));
        }
        let skill = SkillEnum::try_from_value(metadata.id)
            .ok()
            .and_then(|skill| skills::skill_enums::to_object(skill, level));
        if skill
            .as_ref()
            .and_then(|skill| skill.as_offensive_skill())
            .is_some_and(|offensive| {
                crate::server::service::battle_service::BattleService::is_weapon_skill(offensive)
                    || offensive.is_magic()
                    || matches!(Self::actor_behaviour(metadata, level), skills::ActorBehaviour::FixedWeapon { .. })
                    || matches!(Self::actor_behaviour(metadata, level), skills::ActorBehaviour::Pressure)
            })
        {
            return Ok(());
        }
        Err(format!("{} has no completed actor execution handler", metadata.name))
    }
}
