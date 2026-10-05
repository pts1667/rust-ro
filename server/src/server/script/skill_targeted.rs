use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::item::{EquipmentLocation, ItemType};
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64};
use models::status::{Status, StatusSnapshot};
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};
use script_sdk::Value;

use super::metadata::SkillMetadata;
use super::ground_unit_effects::GANBANTEIN_SUCCESS_PERCENT;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::service::script_combat_service::MobCombatEffect;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;


#[derive(Clone, Debug, PartialEq)]
enum TargetEffect {
    Status(StatusChangeRequest),
    EndStatus(StatusChangeKind),
    Action(ScriptSkillAction),
    Damage(u32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedSkillOutcome {
    pub succeeded: bool,
    effects: Vec<TargetEffect>,
}

#[derive(Clone, Debug)]
pub struct ScriptSkillCompletionPlan {
    pub effect: ScriptSkillEffect,
    pub cost: super::requirements::SkillRequirementPlan,
    pub succeeded: bool,
}

impl ScriptSkillService {
    pub fn conditional_mob_immunity(mob: &crate::server::state::mob::Mob, effect: &ScriptSkillEffect) -> bool {
        if effect.skill_id == SkillEnum::MgStonecurse.id() {
            return mob.resists_status(&Self::stone_curse_request(effect.level, effect.source_char_id));
        }
        effect.skill_id == SkillEnum::CgTarotcard.id()
            && (mob.mob_id == 1288
                || mob.status.mob_groups().contains(&models::enums::mob::MobGroup::Battlefield)
                || mob.status.has_status_change(StatusChangeKind::Basilica))
    }

    pub fn conditional_player_immunity(character: &Character, effect: &ScriptSkillEffect) -> bool {
        if effect.skill_id == SkillEnum::CgTarotcard.id() {
            return character.status.has_status_change(StatusChangeKind::Basilica);
        }
        effect.skill_id == SkillEnum::MgStonecurse.id()
            && *StatusService::instance().to_snapshot(&character.status).element() == Element::Undead
    }

    pub fn prepare_conditional_completion(
        &self,
        effect: &ScriptSkillEffect,
        target: &Status,
        snapshot: &StatusSnapshot,
        immune: bool,
        tick: u128,
    ) -> Result<ScriptSkillCompletionPlan, String> {
        if let Some(prepared) = &effect.prepared_outcome {
            let succeeded = prepared.succeeded;
            let mut effect = effect.clone();
            let cost = if succeeded {
                effect.deferred_requirements.take().unwrap_or_default()
            } else {
                effect.deferred_requirements = None;
                Default::default()
            };
            return Ok(ScriptSkillCompletionPlan { effect, cost, succeeded });
        }
        let mut planned = effect.clone();
        let mut succeeded = true;
        let mut effects = vec![];
        if effect.skill_id == SkillEnum::CgTarotcard.id() {
            succeeded = target.hp > 0 && !immune && fastrand::u8(0..100) <= effect.level.saturating_mul(8);
            if succeeded {
                effects = Self::draw_tarot_effects(effect.level, effect.source_char_id, 0);
            }
        } else if effect.skill_id == SkillEnum::HwGanbantein.id() {
            succeeded = fastrand::u8(0..100) < GANBANTEIN_SUCCESS_PERCENT;
        } else if effect.skill_id == SkillEnum::MgStonecurse.id() {
            succeeded = false;
            if target.hp > 0 && !immune {
                let request = StatusEffectService::request_with_resistance(
                    target,
                    snapshot,
                    Self::stone_curse_request(effect.level, effect.source_char_id),
                );
                let mut candidate = target.clone();
                candidate.str = snapshot.str();
                candidate.agi = snapshot.agi();
                candidate.vit = snapshot.vit();
                candidate.int = snapshot.int();
                candidate.dex = snapshot.dex();
                candidate.luk = snapshot.luk();
                let outcome = StatusEffectService::apply_status(&mut candidate, request, tick, fastrand::u16(0..10000))?;
                succeeded = outcome.started;
                effects.extend(outcome.removed.into_iter().map(TargetEffect::EndStatus));
                if let Some(change) = candidate.status_change(StatusChangeKind::StoneWait).filter(|_| succeeded) {
                    effects.push(TargetEffect::Status(StatusChangeRequest {
                        kind: change.kind,
                        duration_ms: change.remaining_ms(tick).min(i32::MAX as u32) as i32,
                        values: change.values,
                        rate: 10000,
                        flags: StatusStartFlag::Loaded.as_flag()
                            | StatusStartFlag::NoAvoid.as_flag()
                            | StatusStartFlag::NoDurationReduction.as_flag(),
                    }));
                }
            }
        } else {
            return Ok(ScriptSkillCompletionPlan {
                effect: planned,
                cost: super::requirements::SkillRequirementPlan::default(),
                succeeded,
            });
        }
        planned.prepared_outcome = Some(PreparedSkillOutcome { succeeded, effects });
        let cost = if succeeded {
            effect.deferred_requirements.clone().unwrap_or_default()
        } else {
            super::requirements::SkillRequirementPlan::default()
        };
        planned.deferred_requirements = None;
        Ok(ScriptSkillCompletionPlan {
            effect: planned,
            cost,
            succeeded,
        })
    }

    pub(super) fn source_heal_amount(&self, character: &Character, level: u8) -> u32 {
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let hp = Self::heal_amount(&snapshot, character.status.base_level, level);
        Self::scale_source_heal(character, &snapshot, hp)
    }

    pub(super) fn scale_source_heal(character: &Character, snapshot: &StatusSnapshot, hp: u32) -> u32 {
        let meditation = character
            .status
            .known_skills
            .iter()
            .find(|skill| skill.value == SkillEnum::HpMeditatio)
            .map_or(0, |skill| skill.level as u32);
        Self::scale_heal_power(snapshot, hp, meditation)
    }

    /// Sanctuary has a fixed base in rathena, so Meditatio does not scale it.
    pub(super) fn scale_fixed_heal(snapshot: &StatusSnapshot, hp: u32) -> u32 {
        Self::scale_heal_power(snapshot, hp, 0)
    }

    fn scale_heal_power(snapshot: &StatusSnapshot, hp: u32, meditation: u32) -> u32 {
        let power = snapshot
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| {
                if let BonusType::HealSkillPercentage(value) = bonus {
                    Some(*value as i32)
                } else {
                    None
                }
            })
            .sum::<i32>();
        (hp as u64 * (100 + 2 * meditation) as u64 / 100 * (100 + power).max(0) as u64 / 100).min(u32::MAX as u64) as u32
    }

    pub(super) fn target_heal_amount(target: &StatusSnapshot, amount: u32) -> u32 {
        let mut amount = amount as u64;
        let power = target
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| {
                if let BonusType::HpRegenFromSkillPercentage(value) = bonus {
                    Some(*value as i32)
                } else {
                    None
                }
            })
            .sum::<i32>();
        amount = amount * (100 + power).max(0) as u64 / 100;
        if let Some(change) = target.status_change(StatusChangeKind::IncHealRate) {
            amount = amount * (100 + change.values[0]).max(0) as u64 / 100;
        }
        if let Some(change) = target.status_change(StatusChangeKind::CriticalWound) {
            amount = amount * (100 - change.values[1]).clamp(0, 100) as u64 / 100;
        }
        amount.min(u32::MAX as u64) as u32
    }

    pub(super) fn apply_script_action(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<bool, String> {
        match effect.action {
            ScriptSkillAction::Cast => return Ok(false),
            ScriptSkillAction::OpenWarpPortalMenu(ref cast) => self.open_warp_portal_menu(state, character, effect, cast, tick)?,
            ScriptSkillAction::MagicAttack {
                target_id,
                ref map,
                issued_skill,
            } => {
                self.execute_player_magic(server, state, character, effect, target_id, map, issued_skill, tick)?;
            }
            ScriptSkillAction::OpenTeleportMenu => self.open_teleport_menu(character, effect, tick)?,
            ScriptSkillAction::CleanGraffiti { ref map, x, y } => {
                self.validate_graffiti_cleanup(state, character, map, x, y, effect.skill_id, effect.level)?;
                let radius = SkillMetadata::find(effect.skill_id)
                    .and_then(|skill| skill.splash(effect.level))
                    .unwrap_or(5)
                    .max(0) as u16;
                self.erase_graffiti(map.map_name(), map.map_instance(), x, y, radius, tick)?;
            }
            ScriptSkillAction::ActivateGround { skill_id, cast_generation } => {
                self.activate_ground_cast(state, character, skill_id, cast_generation, tick)?;
            }
            ScriptSkillAction::TrapControl {
                trap_id,
                ref map,
                spring,
                ignore_range,
            } => {
                self.apply_player_trap_control(server, state, character, effect, trap_id, map, spring, ignore_range, tick)?;
            }
            ScriptSkillAction::ExplodeSplasher => {
                self.explode_splasher(server, state, character, effect, tick)?;
            }
            ScriptSkillAction::AreaStatus { x, y } => match SkillMetadata::find(effect.skill_id).map(|metadata| metadata.name.as_str()) {
                Some("HW_GANBANTEIN") => self.clear_ground_units(character, x, y),
                Some("MO_BODYRELOCATION") => self.body_relocation(server, state, character, x, y, tick),
                _ => self.cast_area_status(server, state, character, effect.skill_id, effect.level, x, y, tick)?,
            },
            ScriptSkillAction::Summon { x, y } => self.summon_alchemist_creature(state, character, effect, x, y)?,
            ScriptSkillAction::Face { direction } => {
                character.dir = direction % 8;
                let mut packet = 0x009C_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes());
                packet.extend_from_slice(&0_u16.to_le_bytes());
                packet.push(character.dir as u8);
                self.notify_area(character, packet);
            }
            ScriptSkillAction::FinalStrike { x, y, ref map, instance } => {
                if character.current_map_name() == map && character.current_map_instance() == instance && character.status.hp > 0 {
                    server.character_service().update_hp_sp(character, 1, character.status.sp);
                    for kind in [StatusChangeKind::Nen, StatusChangeKind::Hiding] {
                        StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
                    }
                    if Self::final_strike_slide_allowed(state, character) {
                        self.relocate_skill_actor(server, state, character, x, y, tick);
                    }
                }
            }
            ScriptSkillAction::DelayedWeaponHit {
                target_id,
                ref map,
                instance,
            } => {
                self.apply_delayed_weapon_hit(server, state, character, effect, target_id, map, instance, tick)?;
            }
            ScriptSkillAction::SnatchWarp {
                victim_id,
                ref map,
                instance,
            } => {
                self.apply_snatch_warp(server, state, character, victim_id, map, instance)?;
            }
            ScriptSkillAction::DelayedStatus {
                ref request,
                ref map,
                instance,
            } => {
                let source = if effect.source_char_id == character.char_id {
                    Some(&*character)
                } else {
                    state.get_character(effect.source_char_id)
                };
                if character.status.hp > 0
                    && character.current_map_name() == map
                    && character.current_map_instance() == instance
                    && source.is_some_and(|source| {
                        source.status.hp > 0 && source.current_map_name() == map && source.current_map_instance() == instance
                    })
                {
                    StatusEffectService::start(server, character, request.clone(), tick, &self.client_notification_sender)?;
                }
            }
            ScriptSkillAction::WaterBall { sequence, cell } => {
                let source = state.get_character(effect.source_char_id).ok_or("Water Ball source disconnected")?;
                self.water_ball_shot(server, state, source, effect, sequence, cell, Some(character), tick)?;
            }
            ScriptSkillAction::Heal { hp, sp } => {
                if character.status.hp == 0
                    || character.status.has_status_change(StatusChangeKind::NoRecovery)
                    || character.status.has_status_change(StatusChangeKind::Berserk)
                {
                    return Ok(true);
                }
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                server.character_service().update_hp_sp(
                    character,
                    character.status.hp.saturating_add(hp).min(snapshot.max_hp()),
                    character.status.sp.saturating_add(sp).min(snapshot.max_sp()),
                );
            }
            ScriptSkillAction::SetResources { hp, sp } => {
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                server.character_service().update_hp_sp(
                    character,
                    hp.unwrap_or(character.status.hp).min(snapshot.max_hp()),
                    sp.unwrap_or(character.status.sp).min(snapshot.max_sp()),
                );
            }
            ScriptSkillAction::ClearBuffs => {
                let removed = StatusEffectService::clear_buffs(&mut character.status);
                for kind in removed {
                    StatusEffectService::send_icon(character, kind, false, tick, &self.client_notification_sender);
                }
                server.character_service().reload_client_side_status(character);
                StatusEffectService::send_visual_status(character, &self.client_notification_sender);
            }
            ScriptSkillAction::RandomWarp => server.server_service.schedule_warp_to_walkable_cell_by_character(
                character.current_map_name(),
                crate::server::model::map::RANDOM_CELL.0,
                crate::server::model::map::RANDOM_CELL.1,
                character.char_id,
            ),
            ScriptSkillAction::BreakEquipment { location } => {
                if character.status.hp == 0 {
                    return Ok(true);
                }
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                let protected = if location & EquipmentLocation::Armor.as_flag() != 0 {
                    (BonusType::UnbreakableArmor, StatusChangeKind::ProtectArmor)
                } else if location & EquipmentLocation::HandLeft.as_flag() != 0 {
                    (BonusType::UnbreakableShield, StatusChangeKind::ProtectShield)
                } else {
                    (BonusType::UnbreakableHelm, StatusChangeKind::ProtectHelm)
                };
                if snapshot.bonuses_raw().iter().any(|bonus| **bonus == protected.0) || character.status.has_status_change(protected.1) {
                    return Ok(true);
                }
                if let Some(index) = character.inventory.iter().enumerate().find_map(|(index, item)| {
                    item.as_ref()
                        .filter(|item| !item.is_damaged && item.equip as u64 & location != 0)
                        .map(|_| index)
                }) {
                    let mut item = character.inventory[index].as_ref().unwrap().clone();
                    item.is_damaged = true;
                    server
                        .runtime()
                        .block_on(self.repository.character_set_item_damaged(character.char_id, item.clone()))
                        .map_err(|error| error.to_string())?;
                    character.inventory[index] = Some(item);
                    server.inventory_service().takeoff_equip_item(character, index);
                }
            }
        }
        Ok(true)
    }

    fn followup_action(&self, server: &Server, effect: &ScriptSkillEffect, target_id: u32, action: ScriptSkillAction) {
        let mut followup = effect.clone();
        followup.target_id = target_id;
        followup.action = action;
        followup.cast_generation = 0;
        followup.skill_event_emitted = true;
        server.add_to_next_tick(GameEvent::CharacterScriptSkill(followup));
    }

    pub(super) fn apply_targeted_player_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<bool, String> {
        let skill = self
            .configuration
            .find_skill_config(&Value::Number(effect.skill_id as i32))
            .ok_or("Unknown skill")?;
        let source = if effect.source_char_id == character.char_id {
            &*character
        } else {
            state.get_character(effect.source_char_id).ok_or("Caster disconnected")?
        };
        let source_status = StatusService::instance().to_snapshot(&source.status);
        let target_status = StatusService::instance().to_snapshot(&character.status);
        match skill.name().as_str() {
            "CR_DEVOTION" => {
                self.start_devotion(server, state, character, effect, tick)?;
            }
            "AS_SPLASHER" => {
                let request = Self::splasher_request(effect, target_status.hp(), target_status.max_hp(), false)?;
                StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
            }
            "MG_STONECURSE" => {
                if let Some(prepared) = &effect.prepared_outcome {
                    if !prepared.succeeded {
                        return Err("Stone Curse failed".into());
                    }
                    for target in &prepared.effects {
                        match target {
                            TargetEffect::Status(request) => {
                                StatusEffectService::start(server, character, request.clone(), tick, &self.client_notification_sender)?;
                            }
                            TargetEffect::EndStatus(kind) => {
                                StatusEffectService::end(server, character, Some(*kind), tick, &self.client_notification_sender)
                            }
                            _ => {}
                        }
                    }
                    return Ok(true);
                }
                let request = Self::stone_curse_request(effect.level, effect.source_char_id);
                StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
            }
            "DC_WINKCHARM" => {
                let duration = SkillMetadata::find(effect.skill_id)
                    .and_then(|skill| skill.duration(effect.level, false))
                    .unwrap_or(0);
                let request = StatusChangeRequest {
                    kind: StatusChangeKind::Confusion,
                    duration_ms: duration,
                    values: [effect.level as i32, 0, 0, 0],
                    rate: 1000,
                    flags: 0,
                };
                if StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)? {
                    let duration = SkillMetadata::find(effect.skill_id)
                        .and_then(|skill| skill.duration(effect.level, true))
                        .unwrap_or(0);
                    StatusEffectService::start(
                        server,
                        character,
                        StatusChangeRequest::guaranteed(StatusChangeKind::WinkCharm, duration, effect.level as i32),
                        tick,
                        &self.client_notification_sender,
                    )?;
                }
            }
            "RG_STRIPWEAPON" | "RG_STRIPSHIELD" | "RG_STRIPARMOR" | "RG_STRIPHELM" | "ST_FULLSTRIP" => {
                let requests = Self::strip_requests(
                    skill.name(),
                    effect.skill_id,
                    effect.level,
                    source_status.dex(),
                    target_status.dex(),
                    true,
                    fastrand::u16(0..1000),
                );
                for request in requests {
                    StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
                }
            }
            "CG_TAROTCARD" => {
                if let Some(prepared) = &effect.prepared_outcome {
                    if !prepared.succeeded {
                        return Err("Tarot Card failed".into());
                    }
                    for target_effect in prepared.effects.clone() {
                        self.apply_player_tarot_effect(server, character, effect, target_effect, tick)?;
                    }
                    return Ok(true);
                }
                if fastrand::u8(0..100) > effect.level.saturating_mul(8) {
                    return Ok(true);
                }
                let effects = Self::draw_tarot_effects(effect.level, effect.source_char_id, 0);
                for target_effect in effects {
                    self.apply_player_tarot_effect(server, character, effect, target_effect, tick)?;
                }
            }
            "SA_SPELLBREAKER" => {
                if character.status.has_status_change(StatusChangeKind::MagicRod) {
                    let amount = source_status.max_sp() / 5;
                    self.followup_action(server, effect, effect.source_char_id, ScriptSkillAction::SetResources {
                        hp: None,
                        sp: Some(source.status.sp.saturating_sub(amount)),
                    });
                    self.apply_script_action(
                        server,
                        state,
                        character,
                        &ScriptSkillEffect {
                            action: ScriptSkillAction::Heal {
                                hp: 0,
                                sp: amount.min(source.status.sp),
                            },
                            ..effect.clone()
                        },
                        tick,
                    )?;
                } else {
                    let cast = character
                        .skill_in_use
                        .as_ref()
                        .filter(|cast| cast.used_at_tick.is_none())
                        .map(|cast| (cast.skill.id(), cast.skill.level()))
                        .or_else(|| {
                            (character.script_skill_state.casting_until > tick).then_some((
                                character.script_skill_state.casting_skill_id,
                                character.script_skill_state.casting_skill_level,
                            ))
                        });
                    if let Some((id, level)) = cast {
                        let cost = SkillMetadata::find(id)
                            .and_then(|skill| skill.requires.as_ref())
                            .and_then(|requires| requires.get("SpCost"))
                            .and_then(|cost| SkillMetadata::json_level_value(cost, level, "Amount"))
                            .unwrap_or(0)
                            .max(0) as u32;
                        character.clear_skill_in_use();
                        character.clear_pending_skill();
                        self.cancel_queued_cast(character);
                        let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
                        packet.extend_from_slice(&character.char_id.to_le_bytes());
                        self.notify_area(character, packet);
                        server
                            .character_service()
                            .update_hp_sp(character, character.status.hp, character.status.sp.saturating_sub(cost));
                        self.followup_action(server, effect, effect.source_char_id, ScriptSkillAction::Heal {
                            hp: 0,
                            sp: cost * 25 * effect.level.saturating_sub(1) as u32 / 100,
                        });
                    }
                }
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    pub fn apply_mob_target_effect(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<(), String> {
        if character.status.hp == 0
            || (effect.cast_generation != 0 && character.script_skill_state.cast_generation != effect.cast_generation)
        {
            return Err("Skill cast was interrupted".into());
        }
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        let instance_state = instance.state();
        let target = instance_state
            .get_mob(effect.target_id)
            .filter(|mob| mob.status.hp() > 0)
            .ok_or("Monster target is unavailable")?;
        if effect.action == ScriptSkillAction::ExplodeSplasher {
            return self.explode_mob_splasher(server, state, character, effect, tick);
        }
        if let ScriptSkillAction::WaterBall { sequence, cell } = effect.action {
            return self.water_ball_shot(server, state, character, effect, sequence, cell, None, tick);
        }
        if let ScriptSkillAction::DelayedStatus {
            ref request,
            ref map,
            instance: origin_instance,
        } = effect.action
        {
            if character.current_map_name() == map && character.current_map_instance() == origin_instance {
                instance.add_to_next_tick(MapEvent::MobStatusChange {
                    mob_id: target.id,
                    request: request.clone(),
                });
            }
            return Ok(());
        }
        if effect.prepared_outcome.is_none()
            && matches!(effect.skill_id, id if id == SkillEnum::MgStonecurse.id() || id == SkillEnum::CgTarotcard.id())
        {
            let plan = self.prepare_conditional_completion(
                effect,
                &target.status_effects,
                &target.status,
                Self::conditional_mob_immunity(target, effect),
                tick,
            )?;
            if plan.cost != super::requirements::SkillRequirementPlan::default() {
                return Err("Conditional skill completion requires its payment transaction".into());
            }
            return self.apply_mob_target_effect(server, state, character, &plan.effect, tick);
        }
        let skill = self
            .configuration
            .find_skill_config(&Value::Number(effect.skill_id as i32))
            .ok_or("Unknown skill")?;
        if !effect.skill_event_emitted {
            character.script_skill_state.casting_until = 0;
            character.script_skill_state.casting_skill_id = 0;
        }
        if skill.name() == "WZ_ESTIMATION" {
            return self.show_monster_estimation(server, state, character, effect.target_id, effect.level);
        }
        if skill.name() == "PR_LEXDIVINA" {
            if target.status.has_status_change(StatusChangeKind::Silence) {
                instance.add_to_next_tick(MapEvent::MobEndStatus {
                    mob_id: target.id,
                    kind: Some(StatusChangeKind::Silence),
                });
            } else {
                self.queue_delayed_status(
                    server,
                    character,
                    effect,
                    Self::delayed_support_request(effect, StatusChangeKind::Silence),
                );
            }
            self.notify_support_skill(character, effect);
            return Ok(());
        }
        if let Some(kind) = Self::status_for_skill(skill.name()) {
            let source = StatusService::instance().to_snapshot(&character.status);
            let request = Self::support_status_request(
                skill,
                kind,
                effect.level,
                character.status.base_level,
                source.int(),
                target.status_effects.base_level,
            );
            if kind == StatusChangeKind::Provoke {
                instance.add_to_next_tick(MapEvent::MobProvoke(crate::server::model::events::map_event::MobProvoke {
                    mob_id: target.id,
                    source_id: effect.source_char_id,
                    request,
                    coma: crate::server::service::combat_trigger_service::ComaBonuses::from_bonuses(source.bonuses()),
                }));
            } else {
                instance.add_to_next_tick(MapEvent::MobStatusChange {
                    mob_id: target.id,
                    request,
                });
            }
        } else {
            match skill.name().as_str() {
                "AS_SPLASHER" => {
                    let immune = target.mode & models::enums::mob::MobMode::Boss.as_flag() != 0;
                    let request = Self::splasher_request(effect, target.status.hp(), target.status.max_hp(), immune)?;
                    instance.add_to_next_tick(MapEvent::MobStatusChange {
                        mob_id: target.id,
                        request,
                    });
                }
                "MG_STONECURSE" => {
                    if let Some(prepared) = &effect.prepared_outcome {
                        if !prepared.succeeded {
                            return Err("Stone Curse failed".into());
                        }
                        for target_effect in prepared.effects.clone() {
                            self.apply_mob_tarot_effect(&instance, effect, &target.status, target_effect, tick);
                        }
                    } else {
                        instance.add_to_next_tick(MapEvent::MobStatusChange {
                            mob_id: target.id,
                            request: Self::stone_curse_request(effect.level, effect.source_char_id),
                        });
                    }
                }
                "DC_WINKCHARM" => {
                    let mut request = StatusChangeRequest::guaranteed(
                        StatusChangeKind::WinkCharm,
                        SkillMetadata::find(effect.skill_id)
                            .and_then(|skill| skill.duration(effect.level, true))
                            .unwrap_or(0),
                        effect.level as i32,
                    );
                    request.values[1] = effect.source_char_id as i32;
                    request.flags = 0;
                    request.rate = ((40 + character.status.base_level as i32 - target.status_effects.base_level as i32).max(0) * 100)
                        .min(u16::MAX as i32) as u16;
                    instance.add_to_next_tick(MapEvent::MobStatusChange {
                        mob_id: target.id,
                        request,
                    });
                }
                "RG_STRIPWEAPON" | "RG_STRIPSHIELD" | "RG_STRIPARMOR" | "RG_STRIPHELM" | "ST_FULLSTRIP" => {
                    let source = StatusService::instance().to_snapshot(&character.status);
                    for request in Self::strip_requests(
                        skill.name(),
                        effect.skill_id,
                        effect.level,
                        source.dex(),
                        target.status.dex(),
                        false,
                        fastrand::u16(0..1000),
                    ) {
                        instance.add_to_next_tick(MapEvent::MobStatusChange {
                            mob_id: target.id,
                            request,
                        });
                    }
                }
                "CG_TAROTCARD" => {
                    if let Some(prepared) = &effect.prepared_outcome {
                        if !prepared.succeeded {
                            return Err("Tarot Card failed".into());
                        }
                        for target_effect in prepared.effects.clone() {
                            self.apply_mob_tarot_effect(&instance, effect, &target.status, target_effect, tick);
                        }
                        self.notify_support_skill(character, effect);
                        return Ok(());
                    }
                    if fastrand::u8(0..100) <= effect.level.saturating_mul(8) {
                        for target_effect in Self::draw_tarot_effects(effect.level, effect.source_char_id, 0) {
                            self.apply_mob_tarot_effect(&instance, effect, &target.status, target_effect, tick);
                        }
                    }
                }
                "AL_HEAL" if Self::undead_target(&target.status) => {
                    let source = StatusService::instance().to_snapshot(&character.status);
                    let damage = self
                        .offensive_heal_damage(server, &source, &target.status, effect, tick)
                        .with_skill_notification(
                            character.current_map_name(),
                            character.current_map_instance(),
                            character.x,
                            character.y,
                            tick,
                            1,
                            0,
                        );
                    instance.add_to_next_tick(MapEvent::MobDamage(damage));
                }
                "AL_HEAL" => instance.add_to_next_tick(MapEvent::MobHeal {
                    mob_id: target.id,
                    hp: Self::target_heal_amount(&target.status, effect.heal_value),
                    sp: 0,
                }),
                "TF_DETOXIFY" => {
                    for kind in [StatusChangeKind::Poison, StatusChangeKind::DeadlyPoison] {
                        instance.add_to_next_tick(MapEvent::MobEndStatus {
                            mob_id: target.id,
                            kind: Some(kind),
                        });
                    }
                }
                "AL_CURE" => {
                    for kind in [StatusChangeKind::Silence, StatusChangeKind::Blind, StatusChangeKind::Confusion] {
                        instance.add_to_next_tick(MapEvent::MobEndStatus {
                            mob_id: target.id,
                            kind: Some(kind),
                        });
                    }
                }
                "PR_STRECOVERY" => {
                    if target.mode & models::enums::mob::MobMode::Boss.as_flag() != 0 {
                        self.notify_support_skill_result(character, effect, false);
                        return Ok(());
                    }
                    instance.add_to_next_tick(MapEvent::MobEndStatus {
                        mob_id: target.id,
                        kind: Some(StatusChangeKind::NoRecovery),
                    });
                    if Self::undead_target(&target.status) {
                        self.queue_delayed_status(
                            server,
                            character,
                            effect,
                            Self::delayed_support_request(effect, StatusChangeKind::Blind),
                        );
                    } else {
                        for kind in [
                            StatusChangeKind::Stone,
                            StatusChangeKind::StoneWait,
                            StatusChangeKind::Freeze,
                            StatusChangeKind::Stun,
                            StatusChangeKind::Sleep,
                        ] {
                            instance.add_to_next_tick(MapEvent::MobEndStatus {
                                mob_id: target.id,
                                kind: Some(kind),
                            });
                        }
                        instance.add_to_next_tick(MapEvent::MobLoseTarget { mob_id: target.id });
                    }
                }
                "SA_DISPELL" => {
                    if target.mode & models::enums::mob::MobMode::Boss.as_flag() == 0 && fastrand::u8(0..100) < 50 + 10 * effect.level {
                        instance.add_to_next_tick(MapEvent::MobDispel(crate::server::model::events::map_event::MobDispel {
                            mob_id: target.id,
                        }));
                    }
                }
                "SA_SPELLBREAKER" => {}
                _ => return Err(format!("Skill {} has no monster target effect", skill.name())),
            }
        }
        self.notify_support_skill(character, effect);
        Ok(())
    }

    pub(super) fn stone_curse_request(level: u8, source_id: u32) -> StatusChangeRequest {
        let metadata = SkillMetadata::find(SkillEnum::MgStonecurse.id()).unwrap();
        StatusChangeRequest {
            kind: StatusChangeKind::StoneWait,
            duration_ms: metadata.duration(level, false).unwrap_or(0),
            values: [level as i32, source_id as i32, metadata.duration(level, true).unwrap_or(0), 0],
            rate: 2000 + 400 * level as u16,
            flags: 0,
        }
    }

    pub(super) fn strip_requests(
        name: &str,
        skill_id: u32,
        level: u8,
        source_dex: u16,
        target_dex: u16,
        player: bool,
        roll: u16,
    ) -> Vec<StatusChangeRequest> {
        use StatusChangeKind::*;
        let difference = source_dex as i32 - target_dex as i32;
        let rate = if name == "ST_FULLSTRIP" {
            let minimum = 50 + 20 * level as i32;
            (minimum + 2 * difference).max(minimum)
        } else {
            50 * (level as i32 + 1) + 2 * difference
        };
        if roll as i32 >= rate {
            return vec![];
        }
        let duration = SkillMetadata::find(skill_id)
            .and_then(|skill| skill.duration(level, false))
            .unwrap_or(0)
            .saturating_add(if player { 0 } else { 15000 })
            .saturating_add((level as i32 + 500 * difference).max(1));
        let kinds = match name {
            "RG_STRIPWEAPON" => vec![StripWeapon],
            "RG_STRIPSHIELD" => vec![StripShield],
            "RG_STRIPARMOR" => vec![StripArmor],
            "RG_STRIPHELM" => vec![StripHelm],
            _ => vec![StripWeapon, StripShield, StripArmor, StripHelm],
        };
        kinds
            .into_iter()
            .map(|kind| StatusChangeRequest::guaranteed(kind, duration, level as i32))
            .collect()
    }

    fn ailment_duration(kind: StatusChangeKind) -> i32 {
        kind.metadata()
            .duration_lookup
            .as_deref()
            .and_then(|name| SkillMetadata::all().iter().find(|skill| skill.name == name))
            .and_then(|skill| skill.duration(1, true))
            .unwrap_or(0)
    }

    fn fixed_damage(effect: &ScriptSkillEffect, damage: u32, tick: u128) -> Damage {
        Damage {
            notification: None,
            source_kind: models::enums::actor::CombatActorKind::Player,
            skill_damage_adjusted: false,
            healing: 0,
            right_hand_damage: None,
            target_id: effect.target_id,
            attacker_id: effect.source_char_id,
            damage,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: 0,
            skill_id: effect.skill_id,
            skill_level: effect.level,
            landed: false,
            proc_depth: effect.proc_depth,
            credit_id: effect.source_char_id,
            defenses_applied: true,
            magic_context: None,
        }
    }

    fn tarot_card(roll: u8) -> u8 {
        [10, 20, 30, 37, 47, 62, 63, 69, 74, 82, 83, 85, 90, 100]
            .iter()
            .position(|threshold| roll < *threshold)
            .map_or(14, |index| index as u8 + 1)
    }

    fn draw_tarot_effects(level: u8, source_id: u32, depth: u8) -> Vec<TargetEffect> {
        let mut card = Self::tarot_card(fastrand::u8(0..100));
        if card == 7 && depth < 16 {
            let mut effects = Self::draw_tarot_effects(level, source_id, depth + 1);
            effects.extend(Self::draw_tarot_effects(level, source_id, depth + 1));
            return effects;
        }
        while card == 7 {
            card = Self::tarot_card(fastrand::u8(0..100));
        }
        Self::tarot_effects(card, level, source_id, fastrand::usize(0..3))
    }

    fn tarot_effects(card: u8, level: u8, source_id: u32, choice: usize) -> Vec<TargetEffect> {
        use StatusChangeKind::*;
        let duration = SkillMetadata::find(SkillEnum::CgTarotcard.id())
            .and_then(|skill| skill.duration(level, true))
            .unwrap_or(0);
        let status = |kind, value, time| {
            let mut request = StatusChangeRequest::guaranteed(kind, time, value);
            request.flags = 0;
            TargetEffect::Status(request)
        };
        match card {
            1 => vec![TargetEffect::Action(ScriptSkillAction::SetResources { hp: None, sp: Some(0) })],
            2 => vec![status(IncMagicAttackRate, -50, duration)],
            3 => vec![TargetEffect::Action(ScriptSkillAction::ClearBuffs)],
            4 => vec![
                TargetEffect::Damage(1000),
                TargetEffect::Action(ScriptSkillAction::BreakEquipment {
                    location: [EquipmentLocation::Armor, EquipmentLocation::HandLeft, EquipmentLocation::HeadTop][choice % 3].as_flag(),
                }),
            ],
            5 => vec![status(IncAttackRate, -50, duration)],
            6 => vec![
                TargetEffect::Action(ScriptSkillAction::Heal { hp: 2000, sp: 0 }),
                TargetEffect::Action(ScriptSkillAction::RandomWarp),
            ],
            8 => vec![if choice % 3 == 2 {
                let mut request = Self::stone_curse_request(1, source_id);
                request.values[0] = level as i32;
                request.rate = 10000;
                TargetEffect::Status(request)
            } else {
                let kind = [Ankle, Freeze][choice % 3];
                status(
                    kind,
                    level as i32,
                    if kind == Ankle { duration } else { Self::ailment_duration(kind) },
                )
            }],
            9 => vec![
                status(Coma, level as i32, 0),
                status(Curse, level as i32, Self::ailment_duration(Curse)),
                status(Poison, level as i32, Self::ailment_duration(Poison)),
            ],
            10 => vec![status(Confusion, level as i32, duration)],
            11 => {
                let mut curse = StatusChangeRequest::guaranteed(Curse, Self::ailment_duration(Curse), 100);
                curse.rate = level as u16 * 100;
                curse.flags = 0;
                vec![
                    TargetEffect::Damage(6666),
                    status(IncAttackRate, -50, duration),
                    status(IncMagicAttackRate, -50, duration),
                    TargetEffect::Status(curse),
                ]
            }
            12 => vec![TargetEffect::Damage(4444)],
            13 => vec![status(Stun, level as i32, Self::ailment_duration(Stun))],
            _ => [IncAttackRate, IncMagicAttackRate, IncHitRate, IncFleeRate, IncDefRate]
                .into_iter()
                .map(|kind| status(kind, -20, duration))
                .collect(),
        }
    }

    fn apply_player_tarot_effect(
        &self,
        server: &Server,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        target: TargetEffect,
        tick: u128,
    ) -> Result<(), String> {
        match target {
            TargetEffect::Status(request) => server.add_to_next_tick(GameEvent::CharacterStatusChange(
                crate::server::model::events::game_event::CharacterStatusChange {
                    char_id: character.char_id,
                    request,
                },
            )),
            TargetEffect::EndStatus(kind) => server.add_to_next_tick(GameEvent::CharacterEndStatus(
                crate::server::model::events::game_event::CharacterEndStatus {
                    char_id: character.char_id,
                    kind: Some(kind),
                },
            )),
            TargetEffect::Action(action) => self.followup_action(server, effect, character.char_id, action),
            TargetEffect::Damage(amount) => server.add_to_next_tick(GameEvent::CharacterDamage(Self::fixed_damage(effect, amount, tick))),
        }
        Ok(())
    }

    fn apply_mob_tarot_effect(
        &self,
        instance: &crate::server::model::map_instance::MapInstance,
        effect: &ScriptSkillEffect,
        target_status: &StatusSnapshot,
        target: TargetEffect,
        tick: u128,
    ) {
        match target {
            TargetEffect::Status(request) => instance.add_to_next_tick(MapEvent::MobStatusChange {
                mob_id: effect.target_id,
                request,
            }),
            TargetEffect::EndStatus(kind) => instance.add_to_next_tick(MapEvent::MobEndStatus {
                mob_id: effect.target_id,
                kind: Some(kind),
            }),
            TargetEffect::Damage(amount) => instance.add_to_next_tick(MapEvent::MobDamage(Self::fixed_damage(effect, amount, tick))),
            TargetEffect::Action(ScriptSkillAction::SetResources { sp: Some(0), .. }) => {
                instance.add_to_next_tick(MapEvent::ScriptMobCombat {
                    source_id: effect.source_char_id,
                    target_id: effect.target_id,
                    effect: MobCombatEffect::Vanish {
                        hp: 0,
                        sp: target_status.sp(),
                    },
                })
            }
            TargetEffect::Action(ScriptSkillAction::Heal { hp, sp }) => instance.add_to_next_tick(MapEvent::MobHeal {
                mob_id: effect.target_id,
                hp,
                sp,
            }),
            TargetEffect::Action(ScriptSkillAction::RandomWarp) => {
                instance.add_to_next_tick(MapEvent::MobRandomWarp { mob_id: effect.target_id })
            }
            TargetEffect::Action(ScriptSkillAction::ClearBuffs) => {
                for change in target_status.active_statuses() {
                    if !change.kind.metadata().flags.get("NoClearBuff").copied().unwrap_or(false)
                        && !change.kind.metadata().flags.get("Debuff").copied().unwrap_or(false)
                    {
                        instance.add_to_next_tick(MapEvent::MobEndStatus {
                            mob_id: effect.target_id,
                            kind: Some(change.kind),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tarot_uses_official_weighted_card_distribution() {
        let mut counts = [0; 14];
        for roll in 0..100 {
            counts[ScriptSkillService::tarot_card(roll) as usize - 1] += 1;
        }
        assert_eq!(counts, [10, 10, 10, 7, 10, 15, 1, 6, 5, 8, 1, 2, 5, 10]);
        assert_eq!(ScriptSkillService::tarot_effects(1, 5, 10, 0), vec![TargetEffect::Action(
            ScriptSkillAction::SetResources { hp: None, sp: Some(0) }
        )]);
    }
    #[test]
    fn full_strip_has_one_shared_roll_and_dex_scaled_duration() {
        let metadata = SkillMetadata::all().iter().find(|skill| skill.name == "ST_FULLSTRIP").unwrap();
        let requests = ScriptSkillService::strip_requests(&metadata.name, metadata.id, 5, 100, 80, false, 189);
        assert_eq!(requests.len(), 4);
        assert_eq!(requests[0].duration_ms, metadata.duration(5, false).unwrap() + 15000 + 10005);
        assert!(ScriptSkillService::strip_requests(&metadata.name, metadata.id, 5, 100, 80, false, 190).is_empty());
        assert_eq!(
            ScriptSkillService::strip_requests(&metadata.name, metadata.id, 5, 1, 200, true, 149).len(),
            4
        );
    }
    #[test]
    fn stone_curse_has_petrification_wait_before_stone_duration() {
        let request = ScriptSkillService::stone_curse_request(10, 99);
        assert_eq!(request.kind, StatusChangeKind::StoneWait);
        assert_eq!(request.rate, 6000);
        assert_eq!(request.values[1], 99);
        assert!(request.duration_ms > 0 && request.values[2] > request.duration_ms);
    }
}
