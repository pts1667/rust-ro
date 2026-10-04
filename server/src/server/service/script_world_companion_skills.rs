use models::status::StatusSnapshot;
use movement::position::Position;
use packets::packets::{Packet, PacketZcNotifySkill2, PacketZcUseSkill};

use super::{
    ScriptWorldService, companion_status_snapshot, homunculus, homunculus_world_id, mercenary_status, mercenary_world_id, protocol,
    recalculate_mercenary, world_data,
};
use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::game_systems::{CharacterGameSystems, CompanionCast, CompanionPosition, ScriptWorldRequest};
use crate::server::script::skill::GroundSkillSource;
use crate::server::script::skill::companion::{CompanionSkillContext, CompanionSkillEffect};
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptWorldService {
    pub(crate) fn heal_companion(
        &self,
        server: &Server,
        character: &mut Character,
        target_id: u32,
        hp: u32,
        sp: u32,
        now: u64,
    ) -> Result<(), String> {
        if !super::companion_snapshots(character)
            .iter()
            .any(|snapshot| snapshot.map_item().id() == target_id)
        {
            return Ok(());
        }
        let mut staged = character.game_systems.clone();
        apply_local_effect(&mut staged, &CompanionSkillEffect::Heal { target_id, hp, sp }, now)?;
        if staged != character.game_systems {
            let saved = self
                .repository
                .save_character_game_systems(character.char_id, &staged)
                .map_err(|error| error.to_string())?;
            super::install_state(character, saved);
            self.send_homunculus(character)?;
            self.send_mercenary(character, now)?;
            self.render_companions(server, character, now)?;
        }
        Ok(())
    }

    pub fn handle_companion_status_change(
        &self,
        server: &Server,
        state: &mut ServerState,
        target_id: u32,
        request: models::status_change::StatusChangeRequest,
        now: u128,
    ) -> Result<bool, String> {
        self.mutate_companion_status(
            server,
            state,
            target_id,
            Some(CompanionSkillEffect::Status { target_id, request }),
            now,
        )
    }

    pub fn handle_companion_status_alternatives(&self, server: &Server, state: &mut ServerState, target_id: u32,
        requests: Vec<models::status_change::StatusChangeRequest>, now: u128) -> Result<bool, String> {
        let owner = state.characters().values().find_map(|character| super::companion_health(character, target_id).map(|_| character.char_id));
        let Some(owner) = owner else { return Ok((1_000_000_000..1_300_000_000).contains(&target_id)); };
        let character = state.characters_mut().get_mut(&owner).unwrap();
        if character.game_systems.pet.as_ref().is_some_and(|pet| super::pet_world_id(pet.id) == target_id) { return Ok(true); }
        for request in requests {
            let mut staged = character.game_systems.clone();
            if !apply_local_effect(&mut staged, &CompanionSkillEffect::Status { target_id, request }, now as u64)? { continue; }
            if staged != character.game_systems {
                let saved = self.repository.save_character_game_systems(character.char_id, &staged).map_err(|error| error.to_string())?;
                super::install_state(character, saved);
                if companion_status_snapshot(character, target_id).is_some_and(|status|
                    status.active_statuses().iter().any(|status| status.kind.blocks_casting())) {
                    self.interrupt_companion_cast(character, target_id, true)?;
                }
                self.send_homunculus(character)?;
                self.send_mercenary(character, now as u64)?;
                self.render_companions(server, character, now as u64)?;
            }
            break;
        }
        Ok(true)
    }

    pub fn handle_companion_end_status(
        &self,
        server: &Server,
        state: &mut ServerState,
        target_id: u32,
        kind: Option<models::status_change::StatusChangeKind>,
        now: u128,
    ) -> Result<bool, String> {
        self.mutate_companion_status(
            server,
            state,
            target_id,
            kind.map(|kind| CompanionSkillEffect::EndStatus { target_id, kind }),
            now,
        )
    }

    fn mutate_companion_status(
        &self,
        server: &Server,
        state: &mut ServerState,
        target_id: u32,
        effect: Option<CompanionSkillEffect>,
        now: u128,
    ) -> Result<bool, String> {
        let owner = state
            .characters()
            .values()
            .find_map(|character| super::companion_health(character, target_id).map(|_| character.char_id));
        let Some(owner) = owner else {
            return Ok((1_000_000_000..1_300_000_000).contains(&target_id));
        };
        let character = state.characters_mut().get_mut(&owner).unwrap();
        if character
            .game_systems
            .pet
            .as_ref()
            .is_some_and(|pet| super::pet_world_id(pet.id) == target_id)
        {
            return Ok(true);
        }
        let mut staged = character.game_systems.clone();
        if let Some(effect) = effect {
            apply_local_effect(&mut staged, &effect, now as u64)?;
        } else if let Some(homunculus) = staged
            .homunculus
            .as_mut()
            .filter(|homunculus| homunculus_world_id(homunculus) == target_id)
        {
            homunculus.statuses.clear();
            homunculus::recalculate_homunculus(homunculus);
        } else if let Some(mercenary) = staged
            .mercenary
            .as_mut()
            .filter(|mercenary| mercenary_world_id(mercenary) == target_id)
        {
            mercenary.statuses.clear();
            recalculate_mercenary(mercenary);
        }
        if staged != character.game_systems {
            let saved = self
                .repository
                .save_character_game_systems(character.char_id, &staged)
                .map_err(|error| error.to_string())?;
            super::install_state(character, saved);
            if companion_status_snapshot(character, target_id)
                .is_some_and(|snapshot| snapshot.active_statuses().iter().any(|status| status.kind.blocks_casting()))
            {
                self.interrupt_companion_cast(character, target_id, true)?;
            }
            self.send_homunculus(character)?;
            self.send_mercenary(character, now as u64)?;
            self.render_companions(server, character, now as u64)?;
        }
        Ok(true)
    }

    pub fn companion_skill_source(&self, character: &Character, skill_id: u32) -> Option<(u32, u8)> {
        let metadata = SkillMetadata::find(skill_id)?;
        if let Some(homunculus) = &character.game_systems.homunculus {
            if homunculus.active && homunculus.hp > 0 {
                if let Some(level) = homunculus.skills.get(&skill_id).copied().filter(|level| *level > 0) {
                    return Some((homunculus_world_id(homunculus), level));
                }
            }
        }
        let mercenary = character.game_systems.mercenary.as_ref().filter(|mercenary| mercenary.hp > 0)?;
        let definition = world_data()
            .mercenaries
            .iter()
            .find(|definition| definition.class_id == mercenary.class_id)?;
        definition
            .skills
            .iter()
            .find(|skill| skill.name == metadata.name)
            .map(|skill| (mercenary_world_id(mercenary), skill.level))
    }

    pub(crate) fn begin_companion_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        target_id: u32,
        now: u64,
    ) -> Result<(), String> {
        self.prepare_companion_cast(server, state, character, skill_id, level, target_id, None, now)
    }

    pub(crate) fn begin_companion_ground_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        now: u64,
    ) -> Result<(), String> {
        self.prepare_companion_cast(server, state, character, skill_id, level, 0, Some((x, y)), now)
    }

    fn prepare_companion_cast(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        target_id: u32,
        ground: Option<(u16, u16)>,
        now: u64,
    ) -> Result<(), String> {
        self.prepare_companion_cast_with_options(server, state, character, skill_id, level, target_id, ground, now, None)
    }

    pub fn start_scripted_companion_skill(&self, server: &Server, state: &mut ServerState,
        request: crate::server::model::events::game_event::ScriptSkillCast, tick: u128) -> Result<(), String> {
        if request.message_id.is_some_and(|message| message > 0) { return Err("Unit skill messages require a monster caster".into()); }
        let owner_id = state.characters().values().find(|owner| companion_status_snapshot(owner, request.source_id).is_some())
            .map(|owner| owner.char_id).ok_or("Companion caster is unavailable")?;
        let mut character = state.characters_mut().remove(&owner_id).ok_or("Companion owner disconnected")?;
        let result = u8::try_from(request.level).map_err(|_| "Invalid companion skill level".to_string()).and_then(|level|
            self.prepare_companion_cast_with_options(server, state, &mut character, request.skill_id, level,
                request.target_id, request.ground, tick as u64, Some(&request)));
        state.insert_character(character);
        result
    }

    fn prepare_companion_cast_with_options(&self, server: &Server, state: &ServerState, character: &mut Character,
        skill_id: u32, level: u8, target_id: u32, ground: Option<(u16, u16)>, now: u64,
        options: Option<&crate::server::model::events::game_event::ScriptSkillCast>) -> Result<(), String> {
        if state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoSkill) {
            return Err("Skills are disabled on this map".into());
        }
        if super::resume_companion_timers(&mut character.game_systems, now) {
            self.persist(character)?;
        }
        let (id, learned) = if let Some(options) = options {
            let status = companion_status_snapshot(character, options.source_id).ok_or("Scripted companion is unavailable")?;
            if status.hp() == 0 { return Err("A dead companion cannot cast".into()); }
            (options.source_id, level)
        } else { self.companion_skill_source(character, skill_id).ok_or("Companion skill is not learned or granted by the contract")? };
        if character
            .game_systems
            .mercenary
            .as_ref()
            .is_some_and(|mercenary| mercenary_world_id(mercenary) == id && !super::mercenary_contract_active(mercenary, now))
        {
            return Err("Mercenary contract has expired".into());
        }
        let metadata = SkillMetadata::find(skill_id).ok_or("Unknown companion skill")?;
        if level == 0 || level > learned || level > metadata.max_level || matches!(metadata.target_type.as_deref(), None | Some("Passive"))
        {
            return Err("Invalid companion skill level or passive skill".into());
        }
        let source = companion_status_snapshot(character, id).ok_or("Companion is unavailable")?;
        if source.active_statuses().iter().any(|status| status.kind.blocks_casting()) {
            return Err("A status change prevents companion casting".into());
        }
        if now < super::companion_cooldown_until(character, id, skill_id)
            || character.game_systems.companion_commands.get(&id).is_some_and(|command| {
                command.cast.is_some() || now < command.can_act_at || command.cooldowns.get(&skill_id).is_some_and(|expiry| now < *expiry)
            })
        {
            return Err("Companion skill is still casting or on cooldown".into());
        }
        let target_id = if ground.is_some() {
            0
        } else if options.is_none() && metadata.target_type.as_deref() == Some("Self") || target_id == 0 {
            id
        } else {
            target_id
        };
        let (target, target_position) = if let Some((x, y)) = ground {
            let ground_source = companion_ground_source(character, id, &source, 0);
            server
                .script_skill_service()
                .validate_actor_ground_with_options(state, &ground_source, skill_id, level, x, y, now as u128,
                    options.is_some_and(|options| options.ignore_range), options.is_some())?;
            (source.clone(), Position { x, y, dir: 0 })
        } else {
            let (target, position, _) = if metadata.name == "MA_REMOVETRAP" {
                trap_target(server, character, &source, target_id, now)?
            } else {
                self.skill_target(server, state, character, target_id)?
            };
            (target, position)
        };
        validate_companion_target_job(metadata, character.status.job)?;
        if metadata.target_type.as_deref() == Some("Attack")
            && !super::companion_can_target(
                &source,
                &target,
                if metadata.flags.get("TargetHidden").copied().unwrap_or(false) {
                    crate::server::service::visibility_service::TargetingMode::DirectHiddenSkill
                } else {
                    crate::server::service::visibility_service::TargetingMode::Direct
                },
            )
        {
            return Err("Companion cannot target this hidden actor".into());
        }
        let position = companion_position(character, id);
        let range = metadata.range(level).unwrap_or(1).unsigned_abs().max(1);
        if !options.is_some_and(|options| options.ignore_range) && u32::from(position.x.abs_diff(target_position.x).max(position.y.abs_diff(target_position.y))) > range {
            return Err("Companion skill target is out of range".into());
        }
        if metadata.target_type.as_deref() == Some("Attack")
            && !server.player_combat_target_allowed(state, character, target_id)
        {
            return Err("Companion target is not an enemy on this map".into());
        }
        if metadata.target_type.as_deref() == Some("Attack")
            && state
                .get_map_instance_from_character(character)
                .is_some_and(|map| map.state().get_mob(target_id).is_some_and(|mob| mob.summon_ai != 0))
        {
            return Err("Companions cannot attack friendly summoned monsters".into());
        }
        validate_cost(&source, metadata, level)?;
        let cast_time = (companion_cast_time(&source, metadata, level).min(i64::MAX as u64) as i64
            + i64::from(options.map_or(0, |options| options.cast_time_adjust_ms))).max(0) as u64;
        character.game_systems.companion_commands.entry(id).or_default().cast = Some(CompanionCast {
            skill_id,
            level,
            target_id,
            ground,
            scripted: options.is_some(),
            ignore_range: options.is_some_and(|options| options.ignore_range),
            cast_cancel: options.and_then(|options| options.cast_cancel),
            completes_at: now.saturating_add(cast_time),
        });
        if cast_time == 0 {
            return self.finish_companion_skill(server, state, character, id, now);
        }
        let mut packet = protocol::header(0x013E);
        packet.extend_from_slice(&id.to_le_bytes());
        packet.extend_from_slice(&target_id.to_le_bytes());
        packet.extend_from_slice(&target_position.x.to_le_bytes());
        packet.extend_from_slice(&target_position.y.to_le_bytes());
        packet.extend_from_slice(&(skill_id as u16).to_le_bytes());
        packet.extend_from_slice(&0u32.to_le_bytes());
        packet.extend_from_slice(&(cast_time.min(u64::from(u32::MAX)) as u32).to_le_bytes());
        self.area(character, packet)
    }

    pub(crate) fn tick_companion_skills(&self, server: &Server, character: &mut Character, now: u64) {
        for (id, command) in &mut character.game_systems.companion_commands {
            if let Some(cast) = command.cast.as_mut() {
                if now >= cast.completes_at {
                    cast.completes_at = u64::MAX;
                    server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                        char_id: character.char_id,
                        request: ScriptWorldRequest::FinishCompanionSkill(*id),
                    }));
                }
            }
        }
    }

    pub(crate) fn interrupt_companion_cast(&self, character: &mut Character, id: u32, forced: bool) -> Result<(), String> {
        let Some(command) = character.game_systems.companion_commands.get_mut(&id) else {
            return Ok(());
        };
        if command
            .cast
            .as_ref()
            .is_none_or(|cast| !forced && !cast.cast_cancel.unwrap_or_else(|| SkillMetadata::find(cast.skill_id).is_none_or(|metadata| metadata.cast_cancel.unwrap_or(true))))
        {
            return Ok(());
        }
        command.cast = None;
        let mut packet = protocol::header(0x01B9);
        packet.extend_from_slice(&id.to_le_bytes());
        self.area(character, packet)
    }

    pub(crate) fn finish_companion_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        id: u32,
        now: u64,
    ) -> Result<(), String> {
        let cast = character
            .game_systems
            .companion_commands
            .get_mut(&id)
            .and_then(|command| command.cast.take())
            .ok_or("No companion cast is pending")?;
        let metadata = SkillMetadata::find(cast.skill_id).ok_or("Unknown companion skill")?;
        if state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoSkill) {
            return Err("Skills became disabled while the companion was casting".into());
        }
        if metadata.target_type.as_deref() == Some("Attack") && !server.player_combat_target_allowed(state, character, cast.target_id) {
            return Err("Companion target is no longer an enemy on this map".into());
        }
        let (source_id, learned) = if cast.scripted { (id, cast.level) }
            else { self.companion_skill_source(character, cast.skill_id).ok_or("Companion is unavailable")? };
        if source_id != id || cast.level > learned {
            return Err("Companion skill source changed while casting".into());
        }
        if character
            .game_systems
            .mercenary
            .as_ref()
            .is_some_and(|mercenary| mercenary_world_id(mercenary) == id && !super::mercenary_contract_active(mercenary, now))
        {
            return Err("Mercenary contract expired while casting".into());
        }
        let source = companion_status_snapshot(character, id).ok_or("Companion is unavailable")?;
        if source.hp() == 0 || source.active_statuses().iter().any(|status| status.kind.blocks_casting()) {
            return Err("Companion cast was interrupted by a status change".into());
        }
        let (target, target_position, target_base_level) = if let Some((x, y)) = cast.ground {
            (
                source.clone(),
                Position { x, y, dir: 0 },
                u32::from(super::companion_health(character, id).unwrap().2),
            )
        } else {
            if metadata.name == "MA_REMOVETRAP" {
                trap_target(server, character, &source, cast.target_id, now)?
            } else {
                self.skill_target(server, state, character, cast.target_id)?
            }
        };
        validate_companion_target_job(metadata, character.status.job)?;
        if metadata.target_type.as_deref() == Some("Attack")
            && !super::companion_can_target(
                &source,
                &target,
                crate::server::service::visibility_service::TargetingMode::SkillCompletion {
                    can_hit_hidden: metadata.flags.get("TargetHidden").copied().unwrap_or(false),
                },
            )
        {
            return Err("Companion skill target became hidden".into());
        }
        let position = companion_position(character, id);
        if !cast.ignore_range && u32::from(position.x.abs_diff(target_position.x).max(position.y.abs_diff(target_position.y)))
            > metadata.range(cast.level).unwrap_or(1).unsigned_abs().max(1)
        {
            return Err("Companion target moved out of range".into());
        }
        let homunculus = character
            .game_systems
            .homunculus
            .as_ref()
            .filter(|homunculus| homunculus_world_id(homunculus) == id);
        let context = CompanionSkillContext {
            target_base_level,
            raw_attack: super::companion_attack_damage(character, id, false, &mut fastrand::Rng::new()).unwrap_or(0),
            base_level: homunculus.map_or_else(
                || u32::from(character.game_systems.mercenary.as_ref().unwrap().level),
                |homunculus| u32::from(homunculus.level),
            ),
            intimacy: homunculus.map_or(0, |homunculus| homunculus.intimacy),
            brain_level: homunculus.map_or(0, |homunculus| homunculus::homunculus_skill_level(homunculus, "HLIF_BRAIN")),
            enemy_target_id: character
                .game_systems
                .companion_commands
                .get(&id)
                .and_then(|command| command.target)
                .or_else(|| character.attack.map(|attack| attack.target)),
            target_is_player: cast.target_id == character.char_id || state.characters().contains_key(&cast.target_id),
        };
        let mut effects = if cast.ground.is_some() {
            vec![CompanionSkillEffect::Ground {
                skill_id: cast.skill_id,
                level: cast.level,
                target_id: 0,
            }]
        } else if metadata.name == "MA_REMOVETRAP" {
            Vec::new()
        } else {
            server.script_skill_service().resolve_companion_skill_with_context(
                server,
                &source,
                &target,
                cast.skill_id,
                cast.level,
                id,
                cast.target_id,
                character.char_id,
                now as u128,
                &context,
            )?
        };
        let map = state
            .get_map_instance_from_character(character)
            .ok_or("Companion map is unavailable")?;
        if metadata.target_type.as_deref() == Some("Attack") && map.state().get_mob(cast.target_id).is_some_and(|mob| mob.summon_ai != 0) {
            return Err("Companions cannot attack friendly summoned monsters".into());
        }
        if metadata.name == "HVAN_EXPLOSION" {
            let radius = metadata.splash(cast.level).unwrap_or(5).unsigned_abs();
            effects.retain(|effect| !matches!(effect, CompanionSkillEffect::Damage(_)));
            for mob in map.state().mobs().values().filter(|mob| {
                mob.hp() > 0
                    && mob.summon_ai == 0
                    && u32::from(position.x.abs_diff(mob.x).max(position.y.abs_diff(mob.y))) <= radius
                    && !server.state().contains_locked_map_item(mob.id)
            }) {
                let resolved = server.script_skill_service().resolve_companion_skill_with_context(
                    server,
                    &source,
                    &mob.status,
                    cast.skill_id,
                    cast.level,
                    id,
                    mob.id,
                    character.char_id,
                    now as u128,
                    &context,
                )?;
                effects.extend(
                    resolved
                        .into_iter()
                        .filter(|effect| matches!(effect, CompanionSkillEffect::Damage(_))),
                );
            }
        }
        let ground_source = companion_ground_source(character, id, &source, context.raw_attack);
        for effect in &effects {
            if let CompanionSkillEffect::Ground {
                skill_id,
                level,
                target_id,
            } = effect
            {
                let (x, y) = if let Some(position) = cast.ground {
                    position
                } else {
                    let (_, position, _) = self.skill_target(server, state, character, *target_id)?;
                    (position.x, position.y)
                };
                server
                    .script_skill_service()
                    .validate_actor_ground_with_options(state, &ground_source, *skill_id, *level, x, y, now as u128, cast.ignore_range, cast.scripted)?;
            }
        }
        let (hp_cost, sp_cost) = validate_cost(&source, metadata, cast.level)?;
        let mut staged = character.game_systems.clone();
        if let Some(homunculus) = staged
            .homunculus
            .as_mut()
            .filter(|homunculus| homunculus_world_id(homunculus) == id)
        {
            homunculus.hp -= hp_cost;
            homunculus.sp -= sp_cost;
        } else if let Some(mercenary) = staged.mercenary.as_mut().filter(|mercenary| mercenary_world_id(mercenary) == id) {
            mercenary.hp -= hp_cost;
            mercenary.sp -= sp_cost;
        }
        for effect in &effects {
            apply_local_effect(&mut staged, effect, now)?;
        }
        if let Some(delay) = metadata
            .cooldown
            .as_ref()
            .and_then(|value| value.value(cast.level, "Time"))
            .filter(|delay| *delay > 0)
        {
            let expiry = now.saturating_add(delay as u64);
            if let Some(homunculus) = staged
                .homunculus
                .as_mut()
                .filter(|homunculus| homunculus_world_id(homunculus) == id)
            {
                homunculus.skill_cooldowns.insert(cast.skill_id, expiry);
            } else if let Some(mercenary) = staged.mercenary.as_mut().filter(|mercenary| mercenary_world_id(mercenary) == id) {
                mercenary.skill_cooldowns.insert(cast.skill_id, expiry);
            }
        }
        let item_costs = self.companion_item_costs(metadata, cast.level)?;
        let saved = if item_costs.is_empty() {
            self.repository.save_character_game_systems(character.char_id, &staged)
        } else {
            self.repository
                .save_game_systems_consuming_items(character.char_id, &staged, &item_costs)
        }
        .map_err(|error| error.to_string())?;
        super::install_state(character, saved);
        if !item_costs.is_empty() {
            server
                .inventory_service()
                .reload_inventory(server.runtime(), character.char_id, character);
        }
        let command = character.game_systems.companion_commands.entry(id).or_default();
        command.can_act_at = now.saturating_add(
            metadata
                .after_cast_act_delay
                .as_ref()
                .and_then(|value| value.value(cast.level, "Time"))
                .unwrap_or(0)
                .max(0) as u64,
        );
        if let Some(cooldown) = metadata.cooldown.as_ref().and_then(|value| value.value(cast.level, "Time")) {
            command.cooldowns.insert(cast.skill_id, now.saturating_add(cooldown.max(0) as u64));
        }
        if let Some(delay) = metadata
            .after_cast_walk_delay
            .as_ref()
            .and_then(|value| value.value(cast.level, "Time"))
        {
            command.next_move_at = command.next_move_at.max(now.saturating_add(delay.max(0) as u64));
        }
        self.send_homunculus(character)?;
        self.send_mercenary(character, now)?;
        if metadata.name == "MA_REMOVETRAP" {
            server.script_skill_service().remove_ground_trap(
                character.current_map_name(),
                character.current_map_instance(),
                cast.target_id,
                now as u128,
            )?;
        }
        let ground_effect = effects.iter().any(|effect| matches!(effect, CompanionSkillEffect::Ground { .. }));
        for effect in effects {
            match effect {
                CompanionSkillEffect::Ground {
                    skill_id,
                    level,
                    target_id,
                } => {
                    let (x, y) = if let Some(position) = cast.ground {
                        position
                    } else {
                        let (_, position, _) = self.skill_target(server, state, character, target_id)?;
                        (position.x, position.y)
                    };
                    server.script_skill_service().place_actor_ground_skill_with_options(
                        server,
                        state,
                        ground_source.clone(),
                        skill_id,
                        level,
                        x,
                        y,
                        now as u128,
                        cast.ignore_range,
                        cast.scripted,
                    )?;
                }
                CompanionSkillEffect::Damage(damage) => {
                    if map.state().get_mob(damage.target_id).is_some() {
                        map.add_to_next_tick(MapEvent::MobDamage(damage));
                    } else {
                        server.add_to_next_tick(GameEvent::CharacterDamage(damage));
                    }
                    let mut packet = PacketZcNotifySkill2::new(server.packetver());
                    packet.set_aid(id);
                    packet.set_target_id(damage.target_id);
                    packet.set_skid(cast.skill_id as u16);
                    packet.set_level(i16::from(cast.level));
                    packet.set_damage(if damage.healing > 0 { -(damage.healing.min(i32::MAX as u32) as i32) }
                        else { damage.damage.min(i32::MAX as u32) as i32 });
                    packet.set_count(1);
                    packet.set_action(6);
                    packet.fill_raw();
                    self.area(character, packet.raw)?;
                }
                CompanionSkillEffect::Status { target_id, request } if !is_local_companion(&character.game_systems, target_id) => {
                    if map.state().get_mob(target_id).is_some() {
                        if request.kind == models::status_change::StatusChangeKind::Provoke {
                            map.add_to_next_tick(MapEvent::MobProvoke(crate::server::model::events::map_event::MobProvoke {
                                mob_id: target_id,
                                source_id: id,
                                request,
                                coma: crate::server::service::combat_trigger_service::ComaBonuses::from_bonuses(source.bonuses()),
                            }));
                        } else {
                            map.add_to_next_tick(MapEvent::MobStatusChange {
                                mob_id: target_id,
                                request,
                            });
                        }
                    } else {
                        server.add_to_next_tick(GameEvent::CharacterStatusChange(
                            crate::server::model::events::game_event::CharacterStatusChange {
                                char_id: target_id,
                                request,
                            },
                        ));
                    }
                }
                CompanionSkillEffect::DelayedStatus {
                    target_id,
                    request,
                    delay_ms,
                } => {
                    if map.state().get_mob(target_id).is_some() {
                        map.add_to_delayed_tick(
                            MapEvent::MobStatusChange {
                                mob_id: target_id,
                                request,
                            },
                            u128::from(delay_ms),
                        );
                    } else {
                        server.add_to_delayed_tick(
                            GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange {
                                char_id: target_id,
                                request,
                            }),
                            u128::from(delay_ms),
                        );
                    }
                }
                CompanionSkillEffect::EndStatus { target_id, kind } if !is_local_companion(&character.game_systems, target_id) => {
                    if map.state().get_mob(target_id).is_some() {
                        map.add_to_next_tick(MapEvent::MobEndStatus {
                            mob_id: target_id,
                            kind: Some(kind),
                        });
                    } else {
                        server.add_to_next_tick(GameEvent::CharacterEndStatus(
                            crate::server::model::events::game_event::CharacterEndStatus {
                                char_id: target_id,
                                kind: Some(kind),
                            },
                        ));
                    }
                }
                CompanionSkillEffect::Heal { target_id, hp, sp } if !is_local_companion(&character.game_systems, target_id) => {
                    if target_id == character.char_id {
                        if character.status.hp > 0
                            && !character
                                .status
                                .has_status_change(models::status_change::StatusChangeKind::NoRecovery)
                        {
                            let target = StatusService::instance().to_snapshot(&character.status);
                            server.character_service().update_hp_sp(
                                character,
                                character.status.hp.saturating_add(hp).min(target.max_hp()),
                                character.status.sp.saturating_add(sp).min(target.max_sp()),
                            );
                        }
                    } else if map.state().get_mob(target_id).is_some() {
                        map.add_to_next_tick(MapEvent::MobHeal { mob_id: target_id, hp, sp });
                    } else if state.characters().contains_key(&target_id) {
                        server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                            char_id: target_id,
                            request: ScriptWorldRequest::HealByCompanion { source_id: id, hp, sp },
                        }));
                    } else if let Some(owner) =
                        state.companion_owner(target_id, character.current_map_name(), character.current_map_instance())
                    {
                        server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                            char_id: owner.char_id,
                            request: ScriptWorldRequest::HealCompanion { target_id, hp, sp },
                        }));
                    }
                }
                CompanionSkillEffect::SwapPositions { source_id, target_id } if target_id == character.char_id => {
                    let source = companion_position(character, source_id);
                    let owner = CompanionPosition {
                        x: character.x,
                        y: character.y,
                        map_instance: character.current_map_instance(),
                    };
                    character.game_systems.rendered_companions.insert(source_id, owner);
                    character.game_systems.companion_commands.entry(source_id).or_default().stay = true;
                    character.x = source.x;
                    character.y = source.y;
                    character.clear_attack();
                    character.movements.clear();
                    server.character_service().defer_position_update(character);
                    for (actor, x, y) in [(source_id, owner.x, owner.y), (character.char_id, source.x, source.y)] {
                        let mut packet = protocol::header(0x0088);
                        packet.extend_from_slice(&actor.to_le_bytes());
                        packet.extend_from_slice(&x.to_le_bytes());
                        packet.extend_from_slice(&y.to_le_bytes());
                        self.area(character, packet)?;
                    }
                }
                CompanionSkillEffect::SelfDestruct { delay_ms } => server.add_to_delayed_tick(
                    GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                        char_id: character.char_id,
                        request: ScriptWorldRequest::CompanionSelfDestruct(id),
                    }),
                    u128::from(delay_ms),
                ),
                _ => {}
            }
        }
        if ground_effect {
            return Ok(());
        }
        let mut packet = PacketZcUseSkill::new(server.packetver());
        packet.set_src_aid(id);
        packet.set_target_aid(cast.target_id);
        packet.set_skid(cast.skill_id as u16);
        packet.set_level(i16::from(cast.level));
        packet.set_result(true);
        packet.fill_raw();
        self.area(character, packet.raw)
    }

    fn companion_item_costs(&self, metadata: &SkillMetadata, level: u8) -> Result<Vec<(i32, u32)>, String> {
        let Some(requirements) = metadata
            .requires
            .as_ref()
            .and_then(|requires| requires.get("ItemCost"))
            .and_then(|items| items.as_array())
        else {
            return Ok(Vec::new());
        };
        let mut costs = Vec::new();
        for requirement in requirements {
            if requirement
                .get("Level")
                .and_then(|value| value.as_u64())
                .is_some_and(|required| required != u64::from(level))
            {
                continue;
            }
            let name = requirement
                .get("Item")
                .and_then(|value| value.as_str())
                .ok_or("Companion skill item requirement has no item name")?;
            let item = self
                .configuration
                .find_item_by_name(name)
                .ok_or("Companion skill item requirement is unknown")?;
            let amount = requirement
                .get("Amount")
                .and_then(|value| value.as_u64())
                .and_then(|value| u32::try_from(value).ok())
                .filter(|value| *value > 0)
                .ok_or("Invalid companion skill item amount")?;
            costs.push((item.id as i32, amount));
        }
        Ok(costs)
    }

    fn skill_target(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        id: u32,
    ) -> Result<(StatusSnapshot, Position, u32), String> {
        if id == character.char_id {
            return Ok((
                StatusService::instance().to_snapshot(&character.status),
                Position {
                    x: character.x,
                    y: character.y,
                    dir: character.dir,
                },
                character.status.base_level,
            ));
        }
        if let Some(snapshot) = companion_status_snapshot(character, id).filter(|_| {
            super::companion_snapshots(character)
                .iter()
                .any(|snapshot| snapshot.map_item().id() == id)
        }) {
            let position = companion_position(character, id);
            return Ok((
                snapshot,
                Position {
                    x: position.x,
                    y: position.y,
                    dir: 0,
                },
                u32::from(super::companion_health(character, id).unwrap().2),
            ));
        }
        if let Some(target) = state
            .characters()
            .get(&id)
            .filter(|target| target.map_instance_key == character.map_instance_key && target.status.hp > 0)
        {
            return Ok((
                StatusService::instance().to_snapshot(&target.status),
                Position {
                    x: target.x,
                    y: target.y,
                    dir: target.dir,
                },
                target.status.base_level,
            ));
        }
        if let Some(owner) = state.companion_owner(id, character.current_map_name(), character.current_map_instance()) {
            let snapshot = companion_status_snapshot(owner, id).ok_or("Companion target has no combat status")?;
            let position = companion_position(owner, id);
            return Ok((
                snapshot,
                Position {
                    x: position.x,
                    y: position.y,
                    dir: 0,
                },
                u32::from(super::companion_health(owner, id).unwrap().2),
            ));
        }
        let map = server
            .state()
            .get_map_instance_from_character(character)
            .ok_or("Companion map is unavailable")?;
        let map_state = map.state();
        let mob = map_state
            .get_mob(id)
            .filter(|mob| mob.hp() > 0 && !server.state().contains_locked_map_item(id))
            .ok_or("Companion skill target is unavailable")?;
        Ok((
            mob.status.clone(),
            Position {
                x: mob.x,
                y: mob.y,
                dir: 0,
            },
            mob.status_effects.base_level,
        ))
    }

    pub(crate) fn destroy_companion(&self, server: &Server, character: &mut Character, id: u32, now: u64) -> Result<(), String> {
        if let Some(homunculus) = character
            .game_systems
            .homunculus
            .as_mut()
            .filter(|homunculus| homunculus_world_id(homunculus) == id)
        {
            homunculus.hp = 0;
            homunculus.active = false;
            homunculus.statuses.clear();
        } else if character
            .game_systems
            .mercenary
            .as_ref()
            .is_some_and(|mercenary| mercenary_world_id(mercenary) == id)
        {
            character.game_systems.mercenary = None;
        } else {
            return Ok(());
        }
        self.persist(character)?;
        self.send_homunculus(character)?;
        self.render_companions(server, character, now)
    }
}

fn companion_position(character: &Character, id: u32) -> CompanionPosition {
    character
        .game_systems
        .rendered_companions
        .get(&id)
        .copied()
        .unwrap_or(CompanionPosition {
            x: character.x,
            y: character.y,
            map_instance: character.current_map_instance(),
        })
}

fn companion_ground_source(character: &Character, id: u32, status: &StatusSnapshot, raw_attack: u32) -> GroundSkillSource {
    let position = companion_position(character, id);
    GroundSkillSource {
        actor_id: id,
        owner_id: character.char_id,
        map: character.current_map_name().clone(),
        instance: character.current_map_instance(),
        x: position.x,
        y: position.y,
        status: status.clone(),
        raw_attack,
        fixed_damage: None,
    }
}

fn trap_target(
    server: &Server,
    character: &Character,
    source: &StatusSnapshot,
    unit_id: u32,
    now: u64,
) -> Result<(StatusSnapshot, Position, u32), String> {
    let (x, y) = server
        .script_skill_service()
        .ground_unit_position(
            character.current_map_name(),
            character.current_map_instance(),
            unit_id,
            now as u128,
        )
        .ok_or("Remove Trap requires a live trap on the companion's map")?;
    Ok((source.clone(), Position { x, y, dir: 0 }, 0))
}

fn validate_cost(source: &StatusSnapshot, metadata: &SkillMetadata, level: u8) -> Result<(u32, u32), String> {
    let cost = |name| {
        metadata
            .requires
            .as_ref()
            .and_then(|requires| requires.get(name))
            .and_then(|value| SkillMetadata::json_level_value(value, level, "Amount"))
            .unwrap_or(0)
    };
    let percentage = |rate: i32, current: u32, maximum: u32| {
        let pool = if rate > 0 { current } else { maximum };
        (u64::from(pool) * u64::from(rate.unsigned_abs()) / 100).min(u64::from(u32::MAX)) as u32
    };
    let maximum_hp_percent = cost("MaxHpTrigger");
    if maximum_hp_percent > 0 && u64::from(source.hp()) * 100 / u64::from(source.max_hp().max(1)) > maximum_hp_percent as u64 {
        return Err("Companion has too much HP for this skill".into());
    }
    let hp = (cost("HpCost").max(0) as u32)
        .saturating_add(percentage(cost("HpRateCost"), source.hp(), source.max_hp()));
    let sp = (cost("SpCost").max(0) as u32).saturating_add(percentage(cost("SpRateCost"), source.sp(), source.max_sp()));
    if source.sp() < sp || (hp > 0 && source.hp() <= hp) {
        Err("Companion has insufficient HP or SP".into())
    } else {
        Ok((if matches!(metadata.name.as_str(), "SM_MAGNUM" | "MS_MAGNUM") { 0 } else { hp }, sp))
    }
}

fn validate_companion_target_job(metadata: &SkillMetadata, owner_job: u32) -> Result<(), String> {
    use models::enums::EnumWithNumberValue;
    use models::enums::class::JobName;
    if metadata.name == "ML_DEVOTION"
        && matches!(
            JobName::from_value(owner_job as usize),
            JobName::Crusader | JobName::Paladin | JobName::BabyCrusader
        )
    {
        Err("Crusader characters cannot receive Mercenary Sacrifice".into())
    } else {
        Ok(())
    }
}

pub(super) fn companion_cast_time(source: &StatusSnapshot, metadata: &SkillMetadata, level: u8) -> u64 {
    let base = metadata
        .cast_time
        .as_ref()
        .and_then(|value| value.value(level, "Time"))
        .unwrap_or(0)
        .max(0) as u64;
    if metadata.cast_time_flags.get("IgnoreDex").copied().unwrap_or(false) {
        base
    } else {
        base.saturating_mul(150u64.saturating_sub(u64::from(source.dex()))) / 150
    }
}

fn is_local_companion(systems: &CharacterGameSystems, id: u32) -> bool {
    systems
        .homunculus
        .as_ref()
        .is_some_and(|homunculus| homunculus_world_id(homunculus) == id)
        || systems
            .mercenary
            .as_ref()
            .is_some_and(|mercenary| mercenary_world_id(mercenary) == id)
}

fn apply_local_effect(systems: &mut CharacterGameSystems, effect: &CompanionSkillEffect, now: u64) -> Result<bool, String> {
    let target_id = match effect {
        CompanionSkillEffect::Status { target_id, .. }
        | CompanionSkillEffect::Heal { target_id, .. }
        | CompanionSkillEffect::EndStatus { target_id, .. } => *target_id,
        _ => 0,
    };
    if let CompanionSkillEffect::SetIntimacy(value) = effect {
        if let Some(homunculus) = systems.homunculus.as_mut() {
            let changed = homunculus.intimacy != *value;
            homunculus.intimacy = *value;
            return Ok(changed);
        }
        return Ok(false);
    }
    let mut accepted = false;
    if let Some(homunculus) = systems
        .homunculus
        .as_mut()
        .filter(|homunculus| homunculus_world_id(homunculus) == target_id)
    {
        let mut status = homunculus::homunculus_status(homunculus);
        let mut berserk_entry = false;
        let mut berserk_refill = false;
        match effect {
            CompanionSkillEffect::Status { request, .. } => {
                if homunculus.hp == 0 { return Ok(false); }
                berserk_refill = request.values[1] == 0;
                let snapshot = homunculus::homunculus_snapshot(homunculus, 150).ok_or("Homunculus status data is unavailable")?;
                let normalized = StatusEffectService::normalize_request_for_target(request.clone(), &snapshot, false);
                let outcome = StatusEffectService::apply_status_for_target(&mut status, normalized, now as u128, fastrand::u16(0..10_000), false)?;
                accepted = outcome.started;
                berserk_entry = outcome.started
                    && request.kind == models::status_change::StatusChangeKind::Berserk
                    && !request.has_flag(models::status_change::StatusStartFlag::Loaded);
            }
            CompanionSkillEffect::EndStatus { kind, .. } => {
                accepted = !StatusEffectService::end_status_at(&mut status, Some(*kind), now as u128).is_empty();
            }
            CompanionSkillEffect::Heal { hp, sp, .. } => {
                if homunculus.hp > 0 && !status.has_status_change(models::status_change::StatusChangeKind::NoRecovery) {
                    status.hp = status.hp.saturating_add(*hp).min(homunculus.max_hp);
                    status.sp = status.sp.saturating_add(*sp).min(homunculus.max_sp);
                    accepted = status.hp != homunculus.hp || status.sp != homunculus.sp;
                }
            }
            _ => {}
        }
        if !accepted { return Ok(false); }
        homunculus.hp = status.hp;
        homunculus.sp = status.sp;
        homunculus.statuses = status.active_statuses;
        homunculus::recalculate_homunculus(homunculus);
        if berserk_entry {
            let mut derived = homunculus::homunculus_status(homunculus);
            StatusEffectService::finalize_berserk_entry_with_refill(&mut derived, homunculus.max_hp, berserk_refill);
            homunculus.hp = derived.hp;
            homunculus.sp = derived.sp;
            homunculus.statuses = derived.active_statuses;
        }
    } else if let Some(mercenary) = systems
        .mercenary
        .as_mut()
        .filter(|mercenary| mercenary_world_id(mercenary) == target_id)
    {
        let mut status = mercenary_status(mercenary);
        let mut berserk_entry = false;
        let mut berserk_refill = false;
        match effect {
            CompanionSkillEffect::Status { request, .. } => {
                if mercenary.hp == 0 { return Ok(false); }
                berserk_refill = request.values[1] == 0;
                let normalized = StatusEffectService::normalize_request_for_target(request.clone(), &super::mercenary_snapshot(mercenary), false);
                let outcome = StatusEffectService::apply_status_for_target(&mut status, normalized, now as u128, fastrand::u16(0..10_000), false)?;
                accepted = outcome.started;
                berserk_entry = outcome.started
                    && request.kind == models::status_change::StatusChangeKind::Berserk
                    && !request.has_flag(models::status_change::StatusStartFlag::Loaded);
            }
            CompanionSkillEffect::EndStatus { kind, .. } => {
                accepted = !StatusEffectService::end_status_at(&mut status, Some(*kind), now as u128).is_empty();
            }
            CompanionSkillEffect::Heal { hp, sp, .. } => {
                if mercenary.hp > 0 && !status.has_status_change(models::status_change::StatusChangeKind::NoRecovery) {
                    status.hp = status.hp.saturating_add(*hp).min(mercenary.max_hp);
                    status.sp = status.sp.saturating_add(*sp).min(mercenary.max_sp);
                    accepted = status.hp != mercenary.hp || status.sp != mercenary.sp;
                }
            }
            _ => {}
        }
        if !accepted { return Ok(false); }
        mercenary.hp = status.hp;
        mercenary.sp = status.sp;
        mercenary.statuses = status.active_statuses;
        recalculate_mercenary(mercenary);
        if berserk_entry {
            let mut derived = mercenary_status(mercenary);
            StatusEffectService::finalize_berserk_entry_with_refill(&mut derived, mercenary.max_hp, berserk_refill);
            mercenary.hp = derived.hp;
            mercenary.sp = derived.sp;
            mercenary.statuses = derived.active_statuses;
        }
    }
    Ok(accepted)
}

#[cfg(test)]
mod tests {
    use models::enums::EnumWithMaskValueU32;
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use super::*;
    use crate::server::model::game_systems::HomunculusRecord;

    fn systems() -> CharacterGameSystems {
        CharacterGameSystems {
            homunculus: Some(HomunculusRecord {
                id: 1,
                class_id: 6001,
                level: 1,
                active: true,
                hp: 100,
                sp: 100,
                max_hp: 1000,
                max_sp: 200,
                base_max_hp: 1000,
                base_max_sp: 200,
                stats: [10, 10, 10, 10, 50, 10],
                ..HomunculusRecord::default()
            }),
            ..CharacterGameSystems::default()
        }
    }

    #[test]
    fn berserk_fills_derived_companion_pools_once_with_existing_hp_bonuses() {
        let mut systems = systems();
        let id = homunculus_world_id(systems.homunculus.as_ref().unwrap());
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::Status {
                target_id: id,
                request: StatusChangeRequest::guaranteed(StatusChangeKind::MercHpUp, 60000, 10),
            },
            0,
        )
        .unwrap();
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::Status {
                target_id: id,
                request: StatusChangeRequest::guaranteed(StatusChangeKind::Berserk, 60000, 1),
            },
            100,
        )
        .unwrap();
        let homunculus = systems.homunculus.as_mut().unwrap();
        assert_eq!((homunculus.max_hp, homunculus.hp, homunculus.sp), (3500, 3500, 0));
        let change = homunculus
            .statuses
            .iter()
            .find(|change| change.kind == StatusChangeKind::Berserk)
            .unwrap();
        assert_eq!(change.values[1], 175);
        homunculus.hp -= 100;
        homunculus::recalculate_homunculus(homunculus);
        assert_eq!(homunculus.hp, 3400);
    }

    #[test]
    fn companion_costs_separate_hp_trigger_from_payment_and_allow_sp_to_reach_zero() {
        let mut metadata: SkillMetadata = serde_json::from_value(serde_json::json!({
            "Id": 8001, "Name": "HLIF_HEAL", "MaxLevel": 5, "CastTime": 1500,
            "Requires": {"HpCost": 15, "HpRateCost": 10, "MaxHpTrigger": 5, "SpCost": 10, "SpRateCost": -25}
        }))
        .unwrap();
        let systems = systems();
        let mut source = homunculus::homunculus_snapshot(systems.homunculus.as_ref().unwrap(), 150).unwrap();
        source.set_hp(50);
        assert_eq!(validate_cost(&source, &metadata, 1).unwrap(), (20, 60));
        source.set_sp(60);
        assert_eq!(validate_cost(&source, &metadata, 1).unwrap(), (20, 60));
        source.set_sp(59);
        assert!(validate_cost(&source, &metadata, 1).is_err());
        source.set_sp(100);
        source.set_hp(59);
        assert!(validate_cost(&source, &metadata, 1).is_ok());
        source.set_hp(60);
        assert!(validate_cost(&source, &metadata, 1).is_err());
        source.set_hp(15);
        assert!(validate_cost(&source, &metadata, 1).is_err());
        assert_eq!(companion_cast_time(&source, &metadata, 1), 1000);
        metadata.cast_time_flags.insert("IgnoreDex".into(), true);
        assert_eq!(companion_cast_time(&source, &metadata, 1), 1500);
        metadata.cast_time_flags.clear();
        source.set_base_dex(150);
        assert_eq!(companion_cast_time(&source, &metadata, 1), 0);
    }

    #[test]
    fn mercenary_magnum_requires_its_primary_hp_amount_without_consuming_it() {
        let metadata = SkillMetadata::all().iter().find(|metadata| metadata.name == "MS_MAGNUM").unwrap();
        let hp = metadata.requires.as_ref().unwrap().get("HpCost").and_then(|value| SkillMetadata::json_level_value(value, 1, "Amount")).unwrap() as u32;
        assert!(hp > 0);
        let systems = systems();
        let mut source = homunculus::homunculus_snapshot(systems.homunculus.as_ref().unwrap(), 150).unwrap();
        source.set_hp(hp);
        assert!(validate_cost(&source, metadata, 1).is_err());
        source.set_hp(hp + 1);
        assert_eq!(validate_cost(&source, metadata, 1).unwrap().0, 0);
    }

    #[test]
    fn companion_quagmire_uses_non_player_penalties_and_blessing_does_not_apply_player_cures() {
        let mut systems = systems();
        systems.homunculus.as_mut().unwrap().stats[1] = 40;
        let id = homunculus_world_id(systems.homunculus.as_ref().unwrap());
        apply_local_effect(&mut systems, &CompanionSkillEffect::Status {
            target_id: id, request: StatusChangeRequest::guaranteed(StatusChangeKind::Quagmire, 1000, 3),
        }, 0).unwrap();
        let homunculus = systems.homunculus.as_ref().unwrap();
        let change = homunculus.statuses.iter().find(|change| change.kind == StatusChangeKind::Quagmire).unwrap();
        assert_eq!((change.values[1], change.values[2]), (30, 0));
        let snapshot = homunculus::homunculus_snapshot(homunculus, 150).unwrap();
        assert_eq!((snapshot.agi(), snapshot.dex()), (10, 20));
        apply_local_effect(&mut systems, &CompanionSkillEffect::Status {
            target_id: id, request: StatusChangeRequest::guaranteed(StatusChangeKind::Curse, 1000, 1),
        }, 0).unwrap();
        apply_local_effect(&mut systems, &CompanionSkillEffect::Status {
            target_id: id, request: StatusChangeRequest::guaranteed(StatusChangeKind::Blessing, 1000, 5),
        }, 0).unwrap();
        let homunculus = systems.homunculus.as_ref().unwrap();
        assert!(homunculus.statuses.iter().any(|change| change.kind == StatusChangeKind::Curse));
        assert!(homunculus.statuses.iter().any(|change| change.kind == StatusChangeKind::Blessing));
    }

    #[test]
    fn loaded_blessing_debuff_changes_companion_attributes_before_matk_and_hit() {
        let mut systems = systems();
        let homunculus = systems.homunculus.as_mut().unwrap();
        homunculus.stats = [10, 10, 10, 50, 30, 10];
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Blessing, 1000, 5);
        request.flags |= models::status_change::StatusStartFlag::Loaded.as_flag();
        let id = homunculus_world_id(homunculus);
        apply_local_effect(&mut systems, &CompanionSkillEffect::Status { target_id: id, request }, 0).unwrap();
        let snapshot = homunculus::homunculus_snapshot(systems.homunculus.as_ref().unwrap(), 150).unwrap();
        assert_eq!((snapshot.int(), snapshot.dex()), (25, 15));
        assert_eq!((snapshot.matk_min(), snapshot.matk_max(), snapshot.hit()), (50, 50, 16));
    }

    #[test]
    fn companion_berserk_and_overthrust_leave_raw_skill_attack_unscaled() {
        let mut character = crate::tests::common::character_helper::create_character();
        character.game_systems = systems();
        let id = homunculus_world_id(character.game_systems.homunculus.as_ref().unwrap());
        let before = super::super::companion_attack_damage(&character, id, true, &mut fastrand::Rng::with_seed(42)).unwrap();
        apply_local_effect(&mut character.game_systems, &CompanionSkillEffect::Status {
            target_id: id, request: StatusChangeRequest::guaranteed(StatusChangeKind::Berserk, 60000, 1),
        }, 0).unwrap();
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Overthrust, 60000, 5);
        request.flags |= models::status_change::StatusStartFlag::Loaded.as_flag();
        request.values[2] = 25;
        apply_local_effect(&mut character.game_systems, &CompanionSkillEffect::Status { target_id: id, request }, 0).unwrap();
        assert_eq!(super::super::companion_attack_damage(&character, id, true, &mut fastrand::Rng::with_seed(42)).unwrap(), before);
        let snapshot = companion_status_snapshot(&character, id).unwrap();
        assert_eq!(crate::server::service::battle_service::BattleService::weapon_skill_ratio(&snapshot, 1.0, 0), 2.25);
        assert_eq!(crate::server::service::battle_service::BattleService::weapon_skill_ratio(&snapshot, 3.0,
            models::enums::skill_enums::SkillEnum::MsBash.id()), 4.25);
    }

    #[test]
    fn homunculus_and_mercenary_snapshots_use_strongest_quicken_and_add_potion_and_berserk_haste() {
        let mut character = crate::tests::common::character_helper::create_character();
        let mercenary = super::super::plan_persistent_effects(&character,
            &[(script_sdk::Function::MercenaryCreate, vec![script_sdk::Value::Number(6017), script_sdk::Value::Number(60000)])], 0)
            .unwrap().systems.unwrap().mercenary.unwrap();
        character.game_systems = systems();
        character.game_systems.mercenary = Some(mercenary);
        character.game_systems.mercenary.as_mut().unwrap().id = 2;
        let hom_id = homunculus_world_id(character.game_systems.homunculus.as_ref().unwrap());
        let merc_id = mercenary_world_id(character.game_systems.mercenary.as_ref().unwrap());
        for id in [hom_id, merc_id] {
            let base = companion_status_snapshot(&character, id).unwrap().aspd();
            for (kind, level) in [(StatusChangeKind::Fleet, 5), (StatusChangeKind::MercQuicken, 1), (StatusChangeKind::AspdPotion0, 1)] {
                apply_local_effect(&mut character.game_systems, &CompanionSkillEffect::Status {
                    target_id: id, request: StatusChangeRequest::guaranteed(kind, 60000, level),
                }, 0).unwrap();
            }
            let snapshot = companion_status_snapshot(&character, id).unwrap();
            let expected = 200.0 - (200.0 - base) * 0.6;
            assert!((snapshot.aspd() - expected).abs() < 0.001, "{} != {}", snapshot.aspd(), expected);
            apply_local_effect(&mut character.game_systems, &CompanionSkillEffect::Status {
                target_id: id, request: StatusChangeRequest::guaranteed(StatusChangeKind::Berserk, 60000, 1),
            }, 0).unwrap();
            let expected = 200.0 - (200.0 - base) * 0.3;
            let snapshot = companion_status_snapshot(&character, id).unwrap();
            assert!((snapshot.aspd() - expected).abs() < 0.001, "{} != {}", snapshot.aspd(), expected);
        }
    }

    #[test]
    fn companion_healing_respects_no_recovery_and_status_entry_hp_changes() {
        let mut systems = systems();
        let id = homunculus_world_id(systems.homunculus.as_ref().unwrap());
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::Status {
                target_id: id,
                request: StatusChangeRequest::guaranteed(StatusChangeKind::NoRecovery, 1000, 1),
            },
            0,
        )
        .unwrap();
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::Heal {
                target_id: id,
                hp: 1000,
                sp: 1000,
            },
            0,
        )
        .unwrap();
        assert_eq!(
            (systems.homunculus.as_ref().unwrap().hp, systems.homunculus.as_ref().unwrap().sp),
            (100, 100)
        );
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::EndStatus {
                target_id: id,
                kind: StatusChangeKind::NoRecovery,
            },
            0,
        )
        .unwrap();
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::Heal {
                target_id: id,
                hp: 1000,
                sp: 1000,
            },
            0,
        )
        .unwrap();
        assert_eq!(
            (systems.homunculus.as_ref().unwrap().hp, systems.homunculus.as_ref().unwrap().sp),
            (1000, 200)
        );
        apply_local_effect(
            &mut systems,
            &CompanionSkillEffect::Status {
                target_id: id,
                request: StatusChangeRequest::guaranteed(StatusChangeKind::DeadlyPoison, 1000, 1),
            },
            0,
        )
        .unwrap();
        assert_eq!(systems.homunculus.as_ref().unwrap().hp, 900);
    }
}
