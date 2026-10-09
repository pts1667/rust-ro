use models::enums::bonus::BonusType;
use models::enums::skill_enums::SkillEnum;
use models::status_bonus::CombatTrigger;
use models::status_change::StatusChangeKind;

use super::metadata::SkillMetadata;
use super::requirements::SkillRequirementPlan;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::model::events::game_event::GameEvent;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

impl ScriptSkillService {
    pub fn autocast_requirements_plan(&self, character: &Character, skill_id: u32, level: u8, tick: u128) -> Result<SkillRequirementPlan, String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Autocast skill requirements are unavailable")?;
        let Some(requirements) = metadata.requires.as_ref() else {
            return Ok(SkillRequirementPlan { minimum_hp: 1, allow_hp_death: true, ..Default::default() });
        };
        let amount = |name: &str| requirements.get(name).and_then(|value| SkillMetadata::json_level_value(value, level, "Amount")).unwrap_or(0);
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let pool_cost = |fixed: i32, rate: i32, current: u32, maximum: u32| {
            (u64::from(fixed.max(0) as u32) + if rate > 0 { u64::from(current) * rate as u64 / 100 }
                else { u64::from(maximum) * u64::from(rate.unsigned_abs()) / 100 }).min(u64::from(current)) as u32
        };
        let rules = Self::cost_rules(metadata, level);
        let hp = if rules.no_hp { 0 }
            else { pool_cost(amount("HpCost"), amount("HpRateCost"), character.status.hp, snapshot.max_hp()) };
        let sp = if rules.autocast_sp {
            let mut sp = pool_cost(amount("SpCost"), amount("SpRateCost"), character.status.sp, snapshot.max_sp());
            let modifier = snapshot.bonuses_raw().iter().filter_map(|bonus| if let BonusType::SpConsumption(value) = bonus { Some(i32::from(*value)) } else { None }).sum::<i32>();
            sp = (u64::from(sp) * (100 + modifier).max(0) as u64 / 100).min(u64::from(u32::MAX)) as u32;
            if character.status.status_change(StatusChangeKind::Spirit).is_some_and(|change| change.values[1] == SkillEnum::SlPriest.id() as i32) { sp = sp.saturating_mul(5); }
            sp.min(character.status.sp)
        } else { 0 };
        let spheres = character.script_skill_state.spirit_spheres.iter().filter(|expiry| **expiry > tick).count().min(u8::MAX as usize) as u8;
        Ok(SkillRequirementPlan {
            minimum_hp: 1, maximum_hp_percent: None, allow_hp_death: true,
            hp, sp, zeny: if rules.no_zeny { 0 } else { (amount("ZenyCost").max(0) as u32).min(character.status.zeny) },
            spirit_spheres: if amount("SpiritSphereCost") < 0 { spheres } else { (amount("SpiritSphereCost").max(0).min(i32::from(spheres))) as u8 },
            removals: vec![],
        })
    }

    pub fn cast_equipment_proc(&self, server: &Server, state: &mut ServerState, source_id: u32, target_id: u32, skill_id: u32, level: u16, tick: u128, depth: u8, trigger: CombatTrigger) -> Result<(), String> {
        if depth >= 8 { return Err("Automatic skill recursion limit reached".into()); }
        let mut character = state.characters_mut().remove(&source_id).ok_or("Autocast source is unavailable")?;
        let result = (|| {
            let level = u8::try_from(level).map_err(|_| "Autocast level is out of range")?;
            let metadata = SkillMetadata::find(skill_id).ok_or("Autocast skill metadata is unavailable")?;
            let skill = self.configuration.find_skill_config(&script_sdk::Value::Number(skill_id as i32)).ok_or("Autocast skill is unavailable")?;
            self.validate_skill(skill, u32::from(level))?;
            if character.status.hp == 0 || character.status.blocks_casting() { return Err("Autocast source cannot cast now".into()); }
            Self::validate_stealth_cast(state, &character, skill_id)?;
            self.validate_support_target(state, &character, skill_id, level, target_id)?;
            if !server.player_skill_target_allowed(state, &character, target_id, skill_id, true) { return Err("Autocast target is unavailable".into()); }
            let plan = self.autocast_requirements_plan(&character, skill_id, level, tick)?;
            server.item_service().pay_requirement_plan(server, &mut character, &plan, None, tick)?;
            if Self::actor_behaviour(metadata, level) == skills::ActorBehaviour::PoisonReact && trigger == CombatTrigger::Hit {
                Self::spend_poison_react(&mut character, tick, &self.client_notification_sender);
            }
            if Self::actor_behaviour(metadata, level) == skills::ActorBehaviour::Martyr {
                Self::pay_martyrs_reckoning(server, &mut character, tick, &self.client_notification_sender);
            }
            let result = if Self::actor_behaviour(metadata, level) == skills::ActorBehaviour::Tarot {
                self.cast_equipment_tarot(server, state, &mut character, target_id, level, tick, depth)
            } else {
                self.cast_skill_depth(server, state, &mut character, skill_id, level, target_id, false, tick, true, depth)
            };
            if matches!(trigger, CombatTrigger::Attack | CombatTrigger::Hit) {
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                let base = metadata.after_cast_act_delay.as_ref().and_then(|delay| delay.value(level, "Time")).unwrap_or(0).max(0) as u32;
                let delay = StatusService::skill_after_cast_delay(&snapshot, skill_id, base);
                character.timing.set_canact_tick(character.timing.get_canact_tick().max(tick + u128::from(delay)));
            }
            result
        })();
        state.insert_character(character);
        result
    }

    fn cast_equipment_tarot(&self, server: &Server, state: &ServerState, source: &mut Character, target_id: u32, level: u8, tick: u128, depth: u8) -> Result<(), String> {
        if source.status.hp == 0 { return Err("Autocast source died before completion".into()); }
        let id = SkillEnum::CgTarotcard.id();
        let sp = SkillMetadata::find(id).and_then(|metadata| metadata.requires.as_ref()).and_then(|requires| requires.get("SpCost"))
            .and_then(|value| SkillMetadata::json_level_value(value, level, "Amount")).unwrap_or(0).max(0) as u32;
        let effect = ScriptSkillEffect {
            source_char_id: source.char_id, target_id, skill_id: id, level, heal_value: 0, proc_depth: depth,
            skill_event_emitted: true, cast_generation: 0, action: ScriptSkillAction::Cast,
            deferred_requirements: Some(SkillRequirementPlan { sp: sp.min(source.status.sp), ..Default::default() }),
            prepared_outcome: None, source_index: None, source_item: None,
        };
        let completion = if let Some(target) = if target_id == source.char_id { Some(&*source) } else { state.get_character(target_id) } {
            self.prepare_conditional_completion(&effect, &target.status, &StatusService::instance().to_snapshot(&target.status), Self::conditional_player_immunity(target, &effect), tick)?
        } else {
            let instance = state.get_map_instance_from_character(source).ok_or("Autocast target map is unavailable")?;
            let instance_state = instance.state();
            let target = instance_state.get_mob(target_id).ok_or("Autocast monster target is unavailable")?;
            self.prepare_conditional_completion(&effect, &target.status_effects, &target.status, Self::conditional_mob_immunity(target, &effect), tick)?
        };
        if !completion.succeeded { return Err("Tarot Card failed".into()); }
        server.item_service().pay_requirement_plan(server, source, &completion.cost, None, tick)?;
        if target_id == source.char_id { self.apply_target_effect(server, state, source, &completion.effect, tick) }
        else if state.get_character(target_id).is_some() { server.add_to_next_tick(GameEvent::CharacterScriptSkill(completion.effect)); Ok(()) }
        else { self.apply_mob_target_effect(server, state, source, &completion.effect, tick) }
    }
}
