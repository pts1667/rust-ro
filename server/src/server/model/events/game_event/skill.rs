use models::enums::EnumWithMaskValueU32;
use models::status_bonus::{BattleFlag, CombatTrigger};

use super::*;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::map_event::{MapEvent, MobEndStatus, MobStatusChange};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::script::skill::{ScriptSkillAction, ScriptSkillService};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::service::{map_combat_service, script_combat_service};
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterStatusChange {
    pub char_id: u32,
    pub request: models::status_change::StatusChangeRequest,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterEndStatus {
    pub char_id: u32,
    pub kind: Option<models::status_change::StatusChangeKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterKnockback {
    pub char_id: u32,
    pub source_x: u16,
    pub source_y: u16,
    pub cells: u16,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ScriptSkillCast {
    pub source_map: Option<String>,
    pub source_instance: Option<u8>,
    pub source_id: u32,
    pub target_id: u32,
    pub skill_id: u32,
    pub level: u16,
    pub ground: Option<(u16, u16)>,
    pub cast_time_adjust_ms: i32,
    pub cast_cancel: Option<bool>,
    pub message_id: Option<u16>,
    pub ignore_range: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptMapDamage {
    pub map: MapInstanceKey,
    pub damage: Damage,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptTeleportSelection {
    pub char_id: u32,
    pub skill_id: u32,
    pub map: String,
    pub session: Option<crate::server::model::session::SessionBinding>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterStatusAlternatives {
    pub char_id: u32,
    pub requests: Vec<models::status_change::StatusChangeRequest>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseGroundSkill {
    pub char_id: u32,
    pub skill_id: u32,
    pub skill_level: u8,
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseGroundSkillText {
    pub skill: CharacterUseGroundSkill,
    pub message: Vec<u8>,
    pub session: crate::server::model::session::SessionBinding,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterUseSkill {
    pub char_id: u32,
    pub target_id: u32,
    pub skill_id: u32,
    pub skill_level: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroundTrapSpend {
    pub map: MapInstanceKey,
    pub unit_id: u32,
}

impl GameEventHandler for crate::server::script::skill::trap::GroundTrapCapture {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        if let Err(error) = server.capture_ground_trap(state, request, tick) {
            warn!("Trap capture failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::trap::GroundTrapRelease {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        if let Err(error) = server.release_ground_trap(state, request, tick) {
            warn!("Trap release failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::trap::GroundTrapEffect {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        if let Err(error) = server.apply_ground_trap_effect(state, request, tick) {
            warn!("Trap effect failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for GroundTrapSpend {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let GroundTrapSpend { map, unit_id } = self;
        server.script_skill_service().spend_ground_unit(&map, unit_id, tick);
        server
            .script_skill_service()
            .refresh_ground_unit_snapshot(state, &map, unit_id, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::ScriptSkillEffect {
    fn required_character(&self) -> Option<u32> {
        Some(self.source_char_id)
    }

    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.source_char_id) || matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let mut effect = self;
        let source = state.characters().get(&effect.source_char_id).ok_or("Skill caster disconnected")?;
        server.script_skill_service().validate_effect_cast(source, &effect)?;
        server.script_skill_service().validate_effect_target(state, source, &effect, tick)?;
        let internal_action = matches!(
            effect.action,
            ScriptSkillAction::OpenTeleportMenu
                | ScriptSkillAction::OpenWarpPortalMenu(_)
                | ScriptSkillAction::TrapControl { .. }
                | ScriptSkillAction::SetResources { .. }
                | ScriptSkillAction::ActivateGround { .. }
                | ScriptSkillAction::CleanGraffiti { .. }
                | ScriptSkillAction::ExplodeSplasher
                | ScriptSkillAction::WaterBall { .. }
                | ScriptSkillAction::AreaStatus { .. }
                | ScriptSkillAction::Summon { .. }
                | ScriptSkillAction::Face { .. }
                | ScriptSkillAction::FinalStrike { .. }
                | ScriptSkillAction::DelayedWeaponHit { .. }
                | ScriptSkillAction::SnatchWarp { .. }
        );
        let target_id = match effect.action {
            ScriptSkillAction::MagicAttack { target_id, .. } => target_id,
            _ => effect.target_id,
        };
        if !internal_action && !server.player_skill_target_allowed(state, source, target_id, effect.skill_id, true) {
            return Err("Skill target is hidden or unavailable".into());
        }
        let (target_status, snapshot, immune) = if let Some(target) = state.characters().get(&effect.target_id) {
            (
                target.status.clone(),
                StatusService::instance().to_snapshot(&target.status),
                ScriptSkillService::conditional_player_immunity(target, &effect),
            )
        } else {
            let instance = state.get_map_instance_from_character(source).ok_or("Skill map is unavailable")?;
            let instance_state = instance.state();
            let target = instance_state.get_mob(effect.target_id).ok_or("Skill target is unavailable")?;
            (
                target.status_effects.clone(),
                target.status.clone(),
                ScriptSkillService::conditional_mob_immunity(target, &effect),
            )
        };
        let (mut cost, conditional) = ScriptSkillService::partition_skill_requirements(
            effect.skill_id,
            effect.level,
            effect.deferred_requirements.take().unwrap_or_default(),
        );
        effect.deferred_requirements = Some(conditional);
        let completion = server
            .script_skill_service()
            .prepare_conditional_completion(&effect, &target_status, &snapshot, immune, tick)?;
        cost.hp = cost.hp.checked_add(completion.cost.hp).ok_or("Skill HP cost overflow")?;
        cost.sp = cost.sp.checked_add(completion.cost.sp).ok_or("Skill SP cost overflow")?;
        cost.zeny = cost.zeny.checked_add(completion.cost.zeny).ok_or("Skill zeny cost overflow")?;
        cost.spirit_spheres = cost
            .spirit_spheres
            .checked_add(completion.cost.spirit_spheres)
            .ok_or("Skill sphere cost overflow")?;
        cost.removals.extend(completion.cost.removals);
        effect = completion.effect;
        let mut caster = state
            .characters_mut()
            .remove(&effect.source_char_id)
            .ok_or("Skill caster disconnected")?;
        let paid = (|| {
            if let Some(identity) = effect.source_item {
                let item = effect
                    .source_index
                    .and_then(|index| caster.get_item_from_inventory(index))
                    .ok_or("Delayed source is unavailable")?;
                if (item.id, item.item_id, item.unique_id) != identity {
                    return Err("Delayed consumable identity changed".into());
                }
            }
            server
                .item_service()
                .pay_requirement_plan_in_state(server, state, &mut caster, &cost, effect.source_index, tick)
        })();
        if !effect.skill_event_emitted {
            caster.script_skill_state.casting_until = 0;
            caster.script_skill_state.casting_skill_id = 0;
        }
        state.insert_character(caster);
        paid?;
        if !completion.succeeded {
            if effect.skill_id == models::enums::skill_enums::SkillEnum::HwGanbantein.id() {
                let mut packet = 0x0110_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&(effect.skill_id as u16).to_le_bytes());
                packet.extend_from_slice(&0_u32.to_le_bytes());
                packet.extend_from_slice(&[
                    0,
                    <models::enums::skill::UseSkillFailure as models::enums::EnumWithNumberValue>::value(
                        &models::enums::skill::UseSkillFailure::Fail,
                    ) as u8,
                ]);
                let _ = server
                    .server_service()
                    .notification_sender()
                    .send(Notification::Char(CharNotification::new(effect.source_char_id, packet)));
            }
            return Ok(());
        }
        if let Some(mut character) = state.characters_mut().remove(&effect.target_id) {
            let result = server
                .script_skill_service()
                .apply_target_effect(server, state, &mut character, &effect, tick);
            state.insert_character(character);
            result?;
        } else {
            let mut caster = state
                .characters_mut()
                .remove(&effect.source_char_id)
                .ok_or("Skill caster disconnected")?;
            let result = server
                .script_skill_service()
                .apply_mob_target_effect(server, state, &mut caster, &effect, tick);
            state.insert_character(caster);
            result?;
        }
        if !effect.skill_event_emitted {
            let battle_flags = crate::server::script::skill::metadata::SkillMetadata::find(effect.skill_id).map_or(
                BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
                |metadata| {
                    metadata.battle_flags(
                        metadata.damage_type.as_deref() == Some("Magic") || metadata.range(effect.level).is_some_and(|range| range > 3),
                    )
                },
            );
            server.add_to_next_tick(GameEvent::ScriptCombat(script_combat_service::ScriptCombatRequest {
                source_id: effect.source_char_id,
                target_id,
                trigger: CombatTrigger::Skill,
                battle_flags,
                skill_id: effect.skill_id,
                damage: 0,
                other_mob_id: None,
                depth: effect.proc_depth,
                drop_position: None,
                right_hand_damage: None,
                origin_map: state
                    .characters()
                    .get(&effect.source_char_id)
                    .map(|source| source.map_instance_key.clone()),
            }));
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::ScriptSkillHit {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.source_id) || matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let hit = self;
        server.script_skill_service().after_skill_damage(server, state, hit, tick)?;
        Ok(())
    }
}

impl GameEventHandler for crate::server::service::script_combat_service::ScriptCombatRequest {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.source_id) || matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        script_combat_service::handle(server, state, request, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::service::map_combat_service::MobAttackRequest {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.attack.target_char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        map_combat_service::handle(server, state, request, tick);
        Ok(())
    }
}

impl GameEventHandler for crate::server::service::map_combat_service::MagicReflectionRequest {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.damage.target_id) || matches(self.damage.attacker_id) || matches(self.reflector_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        map_combat_service::reflect_magic(server, state, request, tick)?;
        Ok(())
    }
}

impl GameEventHandler for ScriptSkillCast {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.source_id) || matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let mut request = self;
        crate::server::service::script_unit_skill_service::normalize_unit_skill_actor_ids(state, &mut request);
        if request.skill_id == crate::server::service::guild_skill_service::GD_ITEMEMERGENCYCALL {
            server.use_item_emergency_call(state, request.source_id, request.level)?;
        } else {
            server.script_skill_service().cast_script_unit_skill(server, state, request, tick)?;
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::actor::ScriptActorSkillCompletion {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let completion = self;
        server
            .script_skill_service()
            .finish_script_actor_skill(server, state, completion, tick)?;
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::WarpPortalEntry {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let entry = self;
        server.script_skill_service().enter_warp_portal(server, state, entry, tick)?;
        Ok(())
    }
}

impl GameEventHandler for crate::server::script::skill::ScriptRevealActor {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let actor = self;
        server.script_skill_service().reveal_from_actor(server, state, &actor, tick)?;
        Ok(())
    }
}

impl GameEventHandler for CharacterStatusChange {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let crate::server::model::events::game_event::CharacterStatusChange { char_id, request } = self;
        if server
            .script_world_service()
            .handle_companion_status_change(server, state, char_id, request.clone(), tick)?
        {
            return Ok(());
        }
        if let Some(mut character) = state.characters_mut().remove(&char_id) {
            let result = StatusEffectService::start(
                server,
                &mut character,
                request,
                tick,
                &server.server_service().notification_sender(),
            );
            state.insert_character(character);
            result?;
        } else {
            let instance = state
                .map_instances()
                .values()
                .flatten()
                .find(|instance| instance.state().get_mob(char_id).is_some())
                .ok_or("Status target is no longer on a map")?;
            instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: char_id, request }));
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterStatusAlternatives {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let crate::server::model::events::game_event::CharacterStatusAlternatives { char_id, requests } = self;
        if server
            .script_world_service()
            .handle_companion_status_alternatives(server, state, char_id, requests.clone(), tick)?
        {
            return Ok(());
        }
        if let Some(mut character) = state.characters_mut().remove(&char_id) {
            let result = StatusEffectService::start_alternatives(
                server,
                &mut character,
                requests,
                tick,
                &server.server_service().notification_sender(),
            );
            state.insert_character(character);
            result?;
        } else {
            let instance = state
                .map_instances()
                .values()
                .flatten()
                .find(|instance| instance.state().get_mob(char_id).is_some())
                .ok_or("Status target is no longer on a map")?;
            instance.add_to_next_tick(MapEvent::MobStatusAlternatives(
                crate::server::model::events::map_event::MobStatusAlternatives { mob_id: char_id, requests },
            ));
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterEndStatus {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let crate::server::model::events::game_event::CharacterEndStatus { char_id, kind } = self;
        if server
            .script_world_service()
            .handle_companion_end_status(server, state, char_id, kind, tick)?
        {
            return Ok(());
        }
        if let Some(mut character) = state.characters_mut().remove(&char_id) {
            StatusEffectService::end(
                server,
                &mut character,
                kind,
                tick,
                &server.server_service().notification_sender(),
            );
            state.insert_character(character);
        } else {
            let instance = state
                .map_instances()
                .values()
                .flatten()
                .find(|instance| instance.state().get_mob(char_id).is_some())
                .ok_or("Status target is no longer on a map")?;
            instance.add_to_next_tick(MapEvent::MobEndStatus(MobEndStatus { mob_id: char_id, kind }));
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterKnockback {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let knockback = self;
        if let Some(mut character) = state.characters_mut().remove(&knockback.char_id) {
            let result = server.script_skill_service().apply_knockback(
                server,
                state,
                &mut character,
                knockback.source_x,
                knockback.source_y,
                knockback.cells,
                tick,
            );
            state.insert_character(character);
            result?;
        }
        Ok(())
    }
}

impl GameEventHandler for CharacterUseGroundSkill {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        use_ground_skill(server, state, self, None, tick)
    }
}

impl GameEventHandler for CharacterUseGroundSkillText {
    fn required_character(&self) -> Option<u32> {
        Some(self.skill.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let event = self;
        let character = state
            .get_character(event.skill.char_id)
            .ok_or("Text ground skill source disconnected")?;
        if !state
            .find_session(character.account_id)
            .is_some_and(|session| session.char_id == Some(character.char_id) && event.session.matches(&session))
        {
            return Err("Text ground skill input belongs to an expired login".into());
        }
        if !ScriptSkillService::is_text_ground_skill(event.skill.skill_id) || event.message.len() > 79 {
            return Err("Invalid text ground skill input".into());
        }

        use_ground_skill(server, state, event.skill, Some(event.message), tick)
    }
}

fn use_ground_skill(
    server: &Server,
    state: &mut ServerState,
    event: CharacterUseGroundSkill,
    message: Option<Vec<u8>>,
    tick: u128,
) -> Result<(), String> {
    if state
        .get_character(event.char_id)
        .is_some_and(|character| character.game_systems.is_trading() || character.timing.skill_menu_blocked())
    {
        return Err("Skills cannot be used during a trade or destination menu".into());
    }
    if (8001..=8016).contains(&event.skill_id) || (8201..=8240).contains(&event.skill_id) {
        server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
            char_id: event.char_id,
            request: crate::server::model::game_systems::ScriptWorldRequest::Companion(crate::server::model::game_systems::CompanionRequest::UseCompanionGroundSkill {
                skill_id: event.skill_id,
                skill_level: event.skill_level,
                x: event.x,
                y: event.y,
            }),
        }));
        return Ok(());
    }
    let mut character = state
        .characters_mut()
        .remove(&event.char_id)
        .ok_or("Ground skill source disconnected")?;
    let result: Result<(), String> = (|| {
        if let Some(message) = message {
            character.script_skill_state.ground_skill_text = message;
        }
        let pending = character.pending_item_skill.clone();
        if let Some(pending) = pending {
            server.script_skill_service().validate_pending_ground(
                state,
                &character,
                event.skill_id,
                event.skill_level,
                event.x,
                event.y,
                tick,
            )?;
            server.item_service().defer_skill_requirements_in_state(
                server,
                state,
                &mut character,
                event.skill_id,
                event.skill_level,
                tick,
                pending.keep_requirements,
                pending.item_index,
            )?;
            if let Some(pending) = character.pending_item_skill.as_mut() {
                pending.keep_requirements = false;
                pending.item_index = None;
            }
            server.script_skill_service().cast_pending_ground(
                server,
                state,
                &mut character,
                event.skill_id,
                event.skill_level,
                event.x,
                event.y,
                tick,
            )?;
        } else {
            let learned = StatusService::instance()
                .to_snapshot(&character.status)
                .known_skills()
                .iter()
                .find(|skill| skill.value.id() == event.skill_id)
                .map_or(0, |skill| skill.level);
            if event.skill_level == 0 || event.skill_level > learned {
                return Err("Ground skill level is not learned".into());
            }
            server.script_skill_service().validate_ground_target(
                state,
                &character,
                event.skill_id,
                event.skill_level,
                event.x,
                event.y,
                tick,
            )?;
            server.item_service().defer_skill_requirements_in_state(
                server,
                state,
                &mut character,
                event.skill_id,
                event.skill_level,
                tick,
                true,
                None,
            )?;
            server.script_skill_service().place_ground_skill(
                server,
                state,
                &mut character,
                event.skill_id,
                event.skill_level,
                event.x,
                event.y,
                tick,
            )?;
        }
        Ok(())
    })();
    state.insert_character(character);
    result?;

    Ok(())
}

impl GameEventHandler for CharacterUseSkill {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.target_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let character_use_skill = self;
        if let Err(error) = server.handle_character_skill(state, character_use_skill, tick) {
            warn!("Skill use failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for ScriptMapDamage {
    fn also_affects(&self, matches: &mut dyn FnMut(u32) -> bool) -> bool {
        matches(self.damage.target_id) || matches(self.damage.attacker_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let request = self;
        server.admit_script_map_damage(state, request, tick)?;
        Ok(())
    }
}

impl GameEventHandler for ScriptTeleportSelection {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let selection = self;
        let Some(mut character) = state.characters_mut().remove(&selection.char_id) else {
            return Ok(());
        };
        let result = if selection.session.as_ref().is_some_and(|binding| {
            state
                .find_session(character.account_id)
                .is_none_or(|session| !binding.matches(&session))
        }) {
            Err("Warp destination choice belongs to another login".into())
        } else if selection.skill_id == models::enums::skill_enums::SkillEnum::AlWarp.id() {
            server
                .script_skill_service()
                .finish_warp_portal_menu(server, state, &mut character, &selection, tick)
        } else {
            server
                .script_skill_service()
                .finish_teleport_menu(server, state, &mut character, &selection, tick)
        };
        state.insert_character(character);
        result?;
        Ok(())
    }
}
