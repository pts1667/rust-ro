use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use models::enums::class::{EquipClassFlag, JobName};
use models::enums::item::{ItemFlag, ItemType};
use models::enums::status::StatusTypes;
use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64, EnumWithNumberValue};
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};
use packets::packets::{Packet, PacketZcItemPickupAck3, PacketZcItemThrowAck, PacketZcLongparChange, PacketZcUseItemAck2};
use script_sdk::{Function, Value, Variable, VariableScope};
use tokio::runtime::Runtime;

use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemConsumption, ScriptItemGrant, ScriptCharacterChange};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterUseItem, GameEvent, ScriptMapSpawn, CharacterUpdateWeight};
use crate::server::model::events::map_event::ScriptSpawn;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::model::permission_groups::Permission;
use crate::server::model::movement::Movable;
use crate::server::script::item_script_handler::{ItemEffect, ItemScriptHost};
use crate::server::service::item_service::ItemService;
use crate::server::service::script_world_service::{persistent_world_operation, plan_persistent_effects, ScriptWorldService};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

pub(crate) fn script_variable_name(name: &str) -> (VariableScope, String) {
    let (scope, prefix) = if name.starts_with("##") { (VariableScope::Account, 2) }
        else if name.starts_with('#') { (VariableScope::Account, 1) }
        else if name.starts_with("$@") { (VariableScope::ServerTemporary, 2) }
        else if name.starts_with('$') { (VariableScope::Server, 1) }
        else if name.starts_with('@') { (VariableScope::CharacterTemporary, 1) }
        else if name.starts_with('.') { (VariableScope::Npc, 1) }
        else { (VariableScope::Character, 0) };
    (scope, name[prefix..].to_string())
}

pub(crate) fn status_kind(value: &Value) -> Result<StatusChangeKind, String> {
    match value {
        Value::Number(id) => StatusChangeKind::from_id(*id),
        Value::String(name) => StatusChangeKind::from_name(name),
        _ => None,
    }.ok_or_else(|| format!("Unknown status {}", value.text()))
}

pub(crate) fn status_request(function: Function, arguments: &[Value], default_flags: u32) -> Result<(Option<u32>, StatusChangeRequest), String> {
    let count = match function { Function::StartStatus => 1, Function::StartStatus2 => 2, Function::StartStatus4 => 4, _ => return Err("Invalid status operation".into()) };
    let number = |index: usize| arguments.get(index).ok_or("Missing status argument")?.number_value();
    let mut values = [0; 4];
    for (index, value) in values.iter_mut().take(count).enumerate() { *value = number(index + 2)?; }
    let duration_ms = number(1)?;
    if duration_ms < -1 { return Err("Invalid status duration".into()); }
    let rate = arguments.get(count + 2).map(Value::number_value).transpose()?.unwrap_or(10_000).clamp(0, 10_000) as u16;
    let flags = arguments.get(count + 3).map(Value::number_value).transpose()?.map_or(Ok(default_flags), |flags| u32::try_from(flags).map_err(|_| "Invalid status flags"))?;
    let target = arguments.get(count + 4).map(Value::number_value).transpose()?.map(|id| u32::try_from(id).map_err(|_| "Invalid status target")).transpose()?;
    Ok((target, StatusChangeRequest { kind: status_kind(arguments.first().ok_or("Missing status kind")?)?, duration_ms, values, rate, flags }))
}

pub(crate) fn status_to_end(arguments: &[Value]) -> Result<(Option<u32>, Option<StatusChangeKind>), String> {
    let value = arguments.first().ok_or("Missing status kind")?;
    let kind = if value.text() == "SC_ALL" || value == &Value::Number(-1) { None } else { Some(status_kind(value)?) };
    let target = arguments.get(1).map(Value::number_value).transpose()?.map(|id| u32::try_from(id).map_err(|_| "Invalid status target")).transpose()?;
    Ok((target, kind))
}

/// `item_use_interval` of `battle/items.conf`.
const ITEM_USE_INTERVAL_MS: u128 = 100;

pub(crate) fn validate_item_map_flags(flags: &MapFlags, item: &ItemModel) -> Result<(), String> {
    if flags.enabled(MapFlag::NoItemConsumption) {
        return Err("Items cannot be used on this map".into());
    }
    if item.flags & ItemFlag::DeadBranch.as_flag() != 0 && (flags.enabled(MapFlag::NoBranch) || flags.is_gvg()) {
        return Err("Dead branches cannot be used on this map".into());
    }
    if crate::server::script::game_data::item_in_use_group(item.id, "MF_NOTELEPORT")
        && (flags.enabled(MapFlag::NoTeleport) || flags.is_gvg())
    {
        return Err("Teleport items cannot be used on this map".into());
    }
    if crate::server::script::game_data::item_in_use_group(item.id, "MF_NORETURN") && flags.enabled(MapFlag::NoReturn) {
        return Err("Return items cannot be used on this map".into());
    }
    Ok(())
}

impl ItemService {
    fn validate_item_in_state(&self, server: &Server, state: &ServerState, character: &Character, item: &InventoryItemModel) -> Result<(), String> {
        if character.game_systems.is_trading() || character.timing.skill_menu_blocked() { return Err("Items are unavailable during trading or destination selection".into()); }
        let flags = state.map_flags(&character.map_instance_key);
        validate_item_map_flags(&flags, self.configuration_service.get_item(item.item_id))?;
        if crate::server::script::game_data::item_in_use_group(item.item_id, "GIANT_FLY_WING") {
            if flags.enabled(MapFlag::NoWarp) { return Err("Party teleport is not allowed on this map".into()); }
            let party = server.repository.party(character.game_systems.party_id).map_err(|error| error.to_string())?.ok_or("Giant Fly Wing requires a party")?;
            if party.leader_char_id != character.char_id || !party.members.contains(&character.char_id) { return Err("Giant Fly Wing requires the party leader".into()); }
            if !state.characters().values().any(|member| member.char_id != character.char_id
                && member.game_systems.party_id == party.id && party.members.contains(&member.char_id)
                && member.map_instance_key == character.map_instance_key && !member.is_dead() && member.status.hp > 0)
            { return Err("Giant Fly Wing requires another living party member on this map".into()); }
        }
        Ok(())
    }

    /// What `pc_isUseitem` and `pc_useitem` check before running an item, then starts the use interval and the item `Delay`.
    fn start_item_use(&self, state: &ServerState, character: &mut Character, item: &InventoryItemModel) -> Result<(), String> {
        if state.has_permission(character.account_id, Permission::ItemUnconditional) {
            return Ok(());
        }
        let model = self.configuration_service.get_item(item.item_id);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|error| error.to_string())?.as_millis();
        if character.game_systems.item_next_use_at > now {
            return Err("Items cannot be used that fast".into());
        }
        if model.nouse_sitting == Some(1) && character.is_sitting() {
            return Err("This item cannot be used while sitting".into());
        }
        if character.status.has_status_change(StatusChangeKind::Stone) || character.status.has_status_change(StatusChangeKind::Freeze)
            || character.status.has_status_change(StatusChangeKind::Stun) || character.status.has_status_change(StatusChangeKind::Sleep) {
            return Err("A status change prevents using items".into());
        }
        let level = character.status.base_level;
        let usable_by_gender = match model.gender.as_deref() { Some("Female" | "F") => character.sex == 0, Some("Male" | "M") => character.sex == 1, _ => true };
        let usable_by_job = model.job_flags & EquipClassFlag::flag_from_job_name(JobName::from_value(character.status.job as usize)) != 0;
        if !usable_by_gender || !usable_by_job || level < model.equip_level_min.unwrap_or(0) as u32 || model.equip_level_max.is_some_and(|max| max > 0 && level > max as u32) {
            return Err("The character does not meet the requirements of this item".into());
        }
        let delay = model.delay_duration.filter(|duration| *duration > 0).map(|duration| (model.delay_status.clone().unwrap_or_else(|| model.id.to_string()), duration));
        if let Some((key, duration)) = delay {
            if character.game_systems.item_delays.get(&key).is_some_and(|until| *until > now) {
                return Err("This item cannot be used again yet".into());
            }
            character.game_systems.item_delays.insert(key, now + duration as u128);
        }
        character.game_systems.item_next_use_at = now + ITEM_USE_INTERVAL_MS;
        Ok(())
    }

    fn validate_pending_item_in_state(&self, server: &Server, state: &ServerState, character: &Character, source_index: Option<usize>) -> Result<(), String> {
        if let Some(item) = Self::validated_skill_source(character, source_index)? {
            self.validate_item_in_state(server, state, character, &item)?;
        }
        Ok(())
    }

    /// `consumeitem`: runs the item's use script on the character without taking the item from the inventory.
    pub(crate) fn run_item_script(&self, server: &Server, state: &mut ServerState, character: &mut Character, item_id: u32) -> Result<(), String> {
        if Self::script_metadata(item_id).is_some_and(|metadata| metadata.interactive) {
            return Err("consumeitem cannot run an interactive item".into());
        }
        let host = self.prepare_host(server, character, item_id, true);
        let (host, result) = futures::executor::block_on(self.item_script_vm.execute(host, "run_item", item_id));
        result.map_err(|error| host.error.clone().unwrap_or(error))?;
        self.apply_effects(server, state, server.runtime(), character, host.effects)
    }

    pub(crate) fn prepare_host(&self, server: &Server, character: &Character, item_id: u32, effects_allowed: bool) -> ItemScriptHost {
        let mut status = character.status.clone();
        status.script_context = Some(std::sync::Arc::new(character.script_character_state()));
        let mut host = if effects_allowed {
            ItemScriptHost::consumable(status, item_id).with_pool_repository(server.repository.clone()).with_guild_storage_context(character.char_id, &character.game_systems)
        } else { ItemScriptHost::bonuses(status, item_id) };
        host.variables.insert("MDiceCone".into(), server.repository.script_variable_char_num_fetch_one(character.char_id, "MDiceCone".into(), 0).into());
        if let Some(metadata) = Self::script_metadata(item_id) {
            for name in &metadata.reads {
                if !host.variables.contains_key(name) && crate::server::script::item_script_handler::status_variable(&host.status, name).is_none() {
                    if let Ok(value) = server.script_service().read_item_variable(server, character, name) { host.variables.insert(name.clone(), value); }
                }
            }
        }
        host
    }

    #[metrics::elapsed]
    pub(crate) fn use_item_in_state(&self, server: &Server, state: &mut ServerState, runtime: &Runtime, action: CharacterUseItem, character: &mut Character) {
        let Some(item) = character.get_item_from_inventory(action.index).cloned().filter(|item| item.item_type().is_consumable() && item.amount > 0) else { return; };
        if character.is_dead() || self.validate_item_in_state(server, state, character, &item).is_err() || self.start_item_use(state, character, &item).is_err() { self.notify_use(character, &action, item.amount, false); return; }
        let mut host = self.prepare_host(server, character, item.item_id as u32, true);
        if Self::script_metadata(item.item_id as u32).is_some_and(|metadata| metadata.calls.iter().any(|call| call == "getcharid")) {
            for character in state.characters().values().chain(std::iter::once(&*character)) {
                for (kind, id) in [character.char_id, character.game_systems.party_id, character.game_systems.guild_id, character.account_id, 0, 0].into_iter().enumerate() {
                    host.context.queries.insert(format!("GetCharacterId:{}:{kind}", character.name), (id as i32).into());
                }
            }
        }
        if Self::script_metadata(item.item_id as u32).is_some_and(|metadata| metadata.interactive) {
            if let Err(error) = self.start_item_dialog(server, state, character, action.clone(), &item, host) {
                warn!("Item conversation could not start: {error}");
                self.notify_use(character, &action, item.amount, false);
            }
            return;
        }
        let (host, result) = futures::executor::block_on(self.item_script_vm.execute(host, "run_item", item.item_id as u32));
        let result = result.and_then(|_| self.finish_item_effects_in_state(server, state, runtime, character, &action, &item, host.effects));
        if let Err(error) = &result { error!("Wasm item {} failed: {}", item.item_id, host.error.as_deref().unwrap_or(error)); }
        let count = character.get_item_from_inventory(action.index).filter(|current| current.id == item.id).map_or(0, |current| current.amount);
        self.notify_use(character, &action, count, result.is_ok());
    }

    pub(crate) fn finish_item_effects_in_state(&self, server: &Server, state: &mut ServerState, runtime: &Runtime, character: &mut Character, action: &CharacterUseItem, item: &InventoryItemModel, effects: Vec<ItemEffect>) -> Result<(), String> {
        if character.is_dead() { return Err("Character died before the item script completed".into()); }
        self.validate_item_in_state(server, state, character, item)?;
            let no_consume = self.configuration_service.get_item(item.item_id).flags & ItemFlag::NoConsume.as_flag() != 0;
            let delayed = item.item_type() == ItemType::DelayConsume && effects.iter().any(|effect| matches!(effect, ItemEffect::Call { function: Function::ItemSkill, arguments } if self.configuration_service.find_skill_config(&arguments[0]).is_some_and(|skill| skill.name() != "AL_TELEPORT")));
            let consumption = (!no_consume && !delayed).then(|| Self::consumption(&item, 1));
            self.commit_effects(server, state, runtime, character, effects, consumption.as_ref())?;
            if delayed && !no_consume {
                if let Some(pending) = character.pending_item_skill.as_mut() { pending.item_index = Some(action.index); pending.source_item = Some((item.id, item.item_id, item.unique_id)); }
            }
            Ok(())
    }

    pub(crate) fn notify_use(&self, character: &Character, action: &CharacterUseItem, count: i16, success: bool) {
        let mut packet = PacketZcUseItemAck2::new(self.configuration_service.packetver());
        packet.set_aid(character.char_id);
        packet.set_index(action.index as u16);
        packet.set_count(count);
        packet.set_result(success);
        packet.fill_raw();
        self.send(character.char_id, packet.raw);
    }

    pub(crate) fn consumption(item: &InventoryItemModel, amount: i16) -> ScriptItemConsumption {
        ScriptItemConsumption { inventory_id: item.id, item_id: item.item_id, unique_id: item.unique_id, amount }
    }

    pub(crate) fn pay_skill_requirements(&self, server: &Server, character: &mut Character, skill_id: u32, level: u8, tick: u128, check_requirements: bool, source_index: Option<usize>) -> Result<(), String> {
        let plan = if check_requirements { server.script_skill_service().requirements_plan(character, skill_id, level, tick)? } else { Default::default() };
        self.pay_requirement_plan(server, character, &plan, source_index, tick)
    }

    pub(crate) fn defer_skill_requirements(&self, server: &Server, character: &mut Character, skill_id: u32, level: u8, tick: u128, check_requirements: bool, source_index: Option<usize>) -> Result<(), String> {
        let plan = if check_requirements { server.script_skill_service().requirements_plan(character, skill_id, level, tick)? } else { Default::default() };
        let source = Self::validated_skill_source(character, source_index)?;
        character.script_skill_state.deferred_requirements = Some(crate::server::script::skill::requirements::DeferredSkillPayment {
            skill_id, level, keep_requirements: check_requirements, requirements: plan, source_index, source_item: source.map(|item| (item.id, item.item_id, item.unique_id)) });
        Ok(())
    }

    pub(crate) fn defer_skill_requirements_in_state(&self, server: &Server, state: &ServerState, character: &mut Character, skill_id: u32, level: u8, tick: u128, check_requirements: bool, source_index: Option<usize>) -> Result<(), String> {
        self.validate_pending_item_in_state(server, state, character, source_index)?;
        self.defer_skill_requirements(server, character, skill_id, level, tick, check_requirements, source_index)
    }

    fn validated_skill_source(character: &Character, source_index: Option<usize>) -> Result<Option<InventoryItemModel>, String> {
        let source = source_index.map(|index| character.get_item_from_inventory(index).cloned().ok_or("Delayed consumable changed before casting")).transpose()?;
        if let Some(item) = &source {
            if !item.item_type().is_consumable() || item.amount <= 0 || item.equip != 0 { return Err("Delayed source is no longer consumable".into()); }
            if character.pending_item_skill.as_ref().and_then(|pending| pending.source_item).is_some_and(|identity| identity != (item.id, item.item_id, item.unique_id)) {
                return Err("Delayed consumable identity changed before casting".into());
            }
        }
        Ok(source)
    }

    pub(crate) fn pay_requirement_plan(&self, server: &Server, character: &mut Character, plan: &crate::server::script::skill::requirements::SkillRequirementPlan, source_index: Option<usize>, tick: u128) -> Result<(), String> {
        if character.game_systems.is_trading() { return Err("Skills cannot be used while trading".into()); }
        if character.status.hp < plan.minimum_hp { return Err("Not enough HP to use the skill".into()); }
        if let Some(limit) = plan.maximum_hp_percent {
            let max_hp = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status).max_hp().max(1);
            if u64::from(character.status.hp) * 100 / u64::from(max_hp) > u64::from(limit) {
                return Err("Current HP exceeds the skill's HP threshold".into());
            }
        }
        if plan.hp > 0 && (character.status.hp < plan.hp || character.status.hp == plan.hp && !plan.allow_hp_death) { return Err("Not enough HP for the skill".into()); }
        if plan.spirit_spheres > 0 && character.script_skill_state.spirit_spheres.iter().filter(|expiry| **expiry > tick).count() < usize::from(plan.spirit_spheres) { return Err("Required spirit spheres expired before casting".into()); }
        let source = Self::validated_skill_source(character, source_index)?;
        let consumption = source.as_ref().map(|item| Self::consumption(item, 1));
        let exact_removals = plan.removals.iter().map(|removal| Ok(Self::consumption(&removal.item, i16::try_from(removal.amount).map_err(|_| "Skill item requirement is out of bounds")?))).collect::<Result<Vec<_>, String>>()?;
        if consumption.is_some() || !exact_removals.is_empty() || plan.hp > 0 || plan.sp > 0 || plan.zeny > 0 {
            let hp = character.status.hp.checked_sub(plan.hp).ok_or("Not enough HP for the skill")?;
            let sp = character.status.sp.checked_sub(plan.sp).ok_or("Not enough SP for the skill")?;
            let zeny = character.status.zeny.checked_sub(plan.zeny).ok_or("Not enough zeny for the skill")?;
            let change = ScriptInventoryTransaction { char_id: character.char_id, account_id: character.account_id,
                consumption: consumption.clone(), exact_removals, removals: vec![], identifications: vec![], grants: vec![], variables: vec![],
                hp: (plan.hp > 0).then_some(hp), sp: (plan.sp > 0).then_some(sp), zeny: (plan.zeny > 0).then_some(zeny),
                max_weight: server.character_service().max_weight(character), max_slots: 100, world: None, reset_skills: None, fame: None, character_changes: vec![], pool_draws: vec![] };
            let result = server.repository.script_inventory_transaction(&change).map_err(|error| error.to_string())?;
            self.install_inventory(server, character, result.inventory, consumption.as_ref());
            if plan.hp > 0 || plan.sp > 0 { server.character_service().update_hp_sp(character, hp, sp); }
            if plan.hp > 0 && hp == 0 {
                character.pending_item_skill = None;
                character.pending_craft = None;
                character.clear_attack();
                character.clear_skill_in_use();
                character.clear_movement();
                server.script_skill_service().cancel_queued_cast(character);
                for kind in StatusEffectService::remove_on_death(&mut character.status) { StatusEffectService::send_icon(character, kind, false, tick, &self.client_notification_sender); }
                character.transition_to_dead();
                StatusEffectService::send_visual_status(character, &self.client_notification_sender);
            }
            if plan.zeny > 0 {
                character.status.zeny = zeny;
                let mut packet = PacketZcLongparChange::new(server.packetver()); packet.set_amount(zeny as i32); packet.set_var_id(StatusTypes::Zeny.value() as u16); packet.fill_raw();
                self.send(character.char_id, packet.raw);
            }
        }
        server.script_skill_service().apply_consumed_spheres(character, plan.spirit_spheres, tick);
        if let (Some(index), Some(item)) = (source_index, source) {
            let count = character.get_item_from_inventory(index).filter(|current| current.id == item.id).map_or(0, |current| current.amount);
            self.notify_use(character, &CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index }, count, true);
        }
        Ok(())
    }

    pub(crate) fn pay_requirement_plan_in_state(&self, server: &Server, state: &ServerState, character: &mut Character, plan: &crate::server::script::skill::requirements::SkillRequirementPlan, source_index: Option<usize>, tick: u128) -> Result<(), String> {
        self.validate_pending_item_in_state(server, state, character, source_index)?;
        self.pay_requirement_plan(server, character, plan, source_index, tick)
    }

    pub(crate) fn identify_item_in_state(&self, server: &Server, state: &ServerState, character: &mut Character, index: usize, tick: u128) -> Result<(), String> {
        self.validate_pending_item_in_state(server, state, character, character.pending_item_skill.as_ref().and_then(|pending| pending.item_index))?;
        self.identify_item(server, character, index, tick)
    }

    pub(crate) fn identify_item(&self, server: &Server, character: &mut Character, index: usize, tick: u128) -> Result<(), String> {
        let item = server.script_skill_service().validate_identification(character, index, tick)?;
        let source_index = character.pending_item_skill.as_ref().and_then(|pending| pending.item_index);
        let source = Self::validated_skill_source(character, source_index)?;
        let pending = character.pending_item_skill.as_ref().ok_or("Identification is no longer active")?;
        let receipt = character.script_skill_state.deferred_requirements.as_ref().filter(|payment| payment.skill_id == pending.skill_id && payment.level == pending.level);
        let cost = if let Some(receipt) = receipt { receipt.requirements.clone() }
            else if pending.keep_requirements { server.script_skill_service().requirements_plan(character, pending.skill_id, pending.level, tick)? } else { Default::default() };
        if cost.hp > 0 && character.status.hp <= cost.hp { return Err("Not enough HP to identify the item".into()); }
        let hp = character.status.hp.checked_sub(cost.hp).ok_or("Not enough HP")?;
        let sp = character.status.sp.checked_sub(cost.sp).ok_or("Not enough SP")?;
        let zeny = character.status.zeny.checked_sub(cost.zeny).ok_or("Not enough zeny")?;
        let consumption = source.as_ref().map(|item| Self::consumption(item, 1));
        let exact_removals = cost.removals.iter().map(|removal| Ok(Self::consumption(&removal.item, i16::try_from(removal.amount).map_err(|_| "Identification requirement is out of bounds")?))).collect::<Result<Vec<_>, String>>()?;
        let change = ScriptInventoryTransaction { char_id: character.char_id, account_id: character.account_id, consumption: consumption.clone(), exact_removals,
            removals: vec![], identifications: vec![item.id], grants: vec![], variables: vec![], zeny: (cost.zeny > 0).then_some(zeny), hp: (cost.hp > 0).then_some(hp), sp: (cost.sp > 0).then_some(sp),
            max_weight: server.character_service().max_weight(character), max_slots: 100, world: None, reset_skills: None, fame: None, character_changes: vec![], pool_draws: vec![] };
        let result = server.repository.script_inventory_transaction(&change).map_err(|error| error.to_string())?;
        self.install_inventory(server, character, result.inventory, consumption.as_ref());
        if cost.hp > 0 || cost.sp > 0 { server.character_service().update_hp_sp(character, hp, sp); }
        character.status.zeny = result.zeny;
        character.script_skill_state.deferred_requirements = None;
        server.script_skill_service().apply_identification_result(character, index)?;
        if let (Some(source_index), Some(source)) = (source_index, source) {
            let count = character.get_item_from_inventory(source_index).filter(|item| item.id == source.id).map_or(0, |item| item.amount);
            self.notify_use(character, &CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: source_index }, count, true);
        }
        Ok(())
    }

    pub(crate) fn validate_effects(&self, effects: &[ItemEffect]) -> Result<(), String> {
        for effect in effects {
            match effect {
                ItemEffect::Heal { .. } => {},
                ItemEffect::PoolDraw(_) => {},
                ItemEffect::GuildStorageOpen(_) => {},
                ItemEffect::Grant { item_id, amount, .. } => {
                    if *amount <= 0 || self.configuration_service.find_item(*item_id).is_none() { return Err("Invalid item grant".into()); }
                }
                ItemEffect::Write { name, value } => {
                    if name.is_empty() || name.len() > 128 || !matches!(value, Value::Number(_) | Value::String(_))
                        || name != "Zeny" && name.ends_with('$') != value.is_string() { return Err("Invalid item variable".into()); }
                    if name == "Zeny" && value.number_value()? < 0 { return Err("Invalid zeny".into()); }
                }
                ItemEffect::Call { function, arguments } => {
                    let number = |index: usize| arguments.get(index).ok_or("Missing operation argument")?.number_value();
                    match function {
                        Function::ItemSkill | Function::UnitSkill => {
                            let skill = self.configuration_service.find_skill_config(arguments.first().ok_or("Missing skill")?).ok_or("Unknown skill")?;
                            if number(1)? <= 0 || number(1)? > 255 { return Err("Invalid skill level".into()); }
                            if skill.name() != "AL_TELEPORT" && number(1)? > skill.max_level() as i32 { return Err("Skill level exceeds its maximum".into()); }
                        }
                        Function::UnitSkillToId | Function::UnitSkillToPosition => {
                            super::script_unit_skill_service::unit_skill_request(self.configuration_service, 0, *function, arguments)?;
                        }
                        Function::StartStatus | Function::StartStatus2 | Function::StartStatus4 => { status_request(*function, arguments, StatusStartFlag::NoDurationReduction.as_flag())?; }
                        Function::EndStatus => { status_to_end(arguments)?; }
                        Function::GetItem | Function::DelItem | Function::GetNamedItem | Function::GetItem2 => {
                            let item = super::super::script::utilities::find_item(self.configuration_service, arguments.first().ok_or("Missing item")?).ok_or("Unknown item")?;
                            if number(1)? <= 0 || number(1)? > i32::from(i16::MAX) || item.id <= 0 { return Err("Invalid item amount".into()); }
                        }
                        Function::Warp | Function::PartyWarp => { arguments.first().ok_or("Missing map")?.string_value()?; number(1)?; number(2)?; }
                        Function::Monster => { self.spawn_request(arguments, 0)?; }
                        Function::SpecialEffect => { number(0)?; }
                        Function::SkillEffect => { number(0)?; number(1)?; }
                        Function::Produce | Function::Cooking => { if number(0)? < 0 { return Err("Invalid crafting level".into()); } }
                        Function::SetFont => { if !(0..=9).contains(&number(0)?) { return Err("Invalid font".into()); } }
                        Function::Announce => { arguments.first().ok_or("Missing announcement")?.string_value()?; number(1)?; }
                        Function::GetExperience => { if number(0)? < 0 || number(1)? < 0 { return Err("Invalid experience amount".into()); } }
                        Function::ResetSkills | Function::OpenStorage => {},
                        function if ScriptWorldService::handles(*function) => {},
                        function => return Err(format!("Invalid staged operation {function:?}")),
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn apply_effects(&self, server: &Server, state: &mut ServerState, runtime: &Runtime, character: &mut Character, effects: Vec<ItemEffect>) -> Result<(), String> {
        self.commit_effects(server, state, runtime, character, effects, None)
    }

    fn commit_effects(&self, server: &Server, state: &mut ServerState, _runtime: &Runtime, character: &mut Character, effects: Vec<ItemEffect>, consumption: Option<&ScriptItemConsumption>) -> Result<(), String> {
        self.validate_effects(&effects)?;
        let tick = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|error| error.to_string())?.as_millis();
        let mut grants = vec![];
        let mut removals = vec![];
        let mut variables = vec![];
        let mut zeny = None;
        let mut world_calls = vec![];
        let mut pool_draws = vec![];
        let mut guild_storage_receipts = vec![];
        let ranked_potion = if effects.iter().any(|effect| matches!(effect, ItemEffect::Heal { item_scaling: true, .. })) {
            if let Some(item) = consumption.and_then(|source| character.inventory_iter().map(|(_, item)| item).find(|item| item.id == source.inventory_id && item.unique_id == source.unique_id)).filter(|item| item.card0 == 254) {
                let creator = u32::from(item.card2 as u16) | (u32::from(item.card3 as u16) << 16);
                server.repository.fame_rank(crate::repository::fame_repository::FameCategory::Alchemist, creator).map_err(|error| error.to_string())? > 0
            } else { false }
        } else { false };
        let mut character_changes = vec![];
        let mut projected = Character::new(character.name.clone(), character.char_id, character.account_id, character.status.clone(), character.x, character.y,
            character.dir, character.current_map_name().clone(), character.sex, vec![]);
        projected.options = character.options;
        projected.map_instance_key = character.map_instance_key.clone();
        projected.game_systems = character.game_systems.clone();
        for effect in &effects {
            match effect {
                ItemEffect::PoolDraw(receipt) => pool_draws.push(receipt.clone()),
                ItemEffect::GuildStorageOpen(receipt) => guild_storage_receipts.push(receipt.clone()),
                ItemEffect::Heal { hp, sp, percentage, item_scaling, item_id } => {
                    let (hp, sp) = self.healed_resources(&projected, projected.status.hp, projected.status.sp, *hp, *sp, *percentage, *item_scaling, *item_id, ranked_potion);
                    projected.status.hp = hp; projected.status.sp = sp;
                    character_changes.push(ScriptCharacterChange::Resources { hp, sp });
                }
                ItemEffect::Grant { item_id, amount, identified } => grants.push(ScriptItemGrant { item_id: *item_id, amount: *amount, identified: *identified, refine: 0, cards: [0; 4], unique_id: None, damaged: false }),
                ItemEffect::Write { name, value } if name == "Zeny" => zeny = Some(value.number_value()? as u32),
                ItemEffect::Write { name, value } => {
                    let (scope, name) = script_variable_name(name);
                    variables.push(Variable { scope, name, index: 0, value: value.clone() });
                }
                ItemEffect::Call { function, arguments } => {
                    if ScriptWorldService::handles(*function) { server.script_world_service().validate_call(state, &projected, *function, arguments, tick as u64)?; }
                    match function {
                        Function::GetItem => {
                            let item = super::super::script::utilities::find_item(self.configuration_service, &arguments[0]).ok_or("Unknown item")?;
                            grants.push(ScriptItemGrant { item_id: item.id, amount: arguments[1].number_value()? as i16, identified: true, refine: 0, cards: [0; 4], unique_id: None, damaged: false });
                        }
                        Function::GetNamedItem => {
                            let item = super::super::script::utilities::find_item(self.configuration_service, &arguments[0]).ok_or("Unknown item")?;
                            let owner = match &arguments[1] {
                                Value::Number(id) => *id as u32,
                                name if name.text() == character.name => character.char_id,
                                _ => return Err("A named item can only carry the name of the attached player".into()),
                            };
                            let cards = [255, 0, (owner & 0xffff) as u16 as i16, (owner >> 16) as u16 as i16];
                            grants.push(ScriptItemGrant { item_id: item.id, amount: 1, identified: true, refine: 0, cards, unique_id: None, damaged: false });
                        }
                        Function::GetItem2 => {
                            let item = super::super::script::utilities::find_item(self.configuration_service, &arguments[0]).ok_or("Unknown item")?;
                            let number = |index: usize| arguments.get(index).map(Value::number_value).transpose().map(|value| value.unwrap_or(0));
                            let cards = [number(5)? as i16, number(6)? as i16, number(7)? as i16, number(8)? as i16];
                            grants.push(ScriptItemGrant { item_id: item.id, amount: number(1)? as i16, identified: number(2)? != 0, refine: number(3)? as i16, cards, unique_id: None, damaged: number(4)? != 0 });
                        }
                        Function::DelItem => {
                            let item = super::super::script::utilities::find_item(self.configuration_service, &arguments[0]).ok_or("Unknown item")?;
                            removals.push((item.id, arguments[1].number_value()? as i16));
                        }
                        Function::Warp => { self.warp_destination(character, arguments)?; }
                        Function::PartyWarp => { super::party_warp_service::party_warp_request(character, arguments)?; }
                        Function::ItemSkill => {
                            let skill = self.configuration_service.find_skill_config(&arguments[0]).ok_or("Unknown skill")?;
                            server.script_skill_service().validate_skill(skill, arguments[1].number_value()? as u32)?;
                            if character.status.blocks_casting() { return Err("A status change prevents using this skill".into()); }
                            if skill.name() == "AL_TELEPORT" {
                                let (map, _, _) = crate::server::script::skill::ScriptSkillService::teleport_destination(character, arguments[1].number_value()? as u32)?;
                                if self.configuration_service.find_map(&map).is_none() { return Err("Return map is unavailable".into()); }
                            }
                        }
                        Function::Produce | Function::Cooking => { self.validate_crafting(character, *function, arguments)?; }
                        Function::GetExperience => {
                            let plan = super::script_character_service::plan_raw_experience(&projected, arguments[0].number_value()? as u32, arguments[1].number_value()? as u32)?;
                            projected.status.base_level = plan.base_level; projected.status.job_level = plan.job_level;
                            projected.status.base_exp = plan.base_exp; projected.status.job_exp = plan.job_exp;
                            projected.status.status_point = plan.status_points; projected.status.skill_point = plan.skill_points;
                            projected.status.hp = plan.hp; projected.status.sp = plan.sp; projected.status.max_hp = plan.max_hp; projected.status.max_sp = plan.max_sp;
                            character_changes.push(ScriptCharacterChange::Experience(plan));
                        }
                        Function::ResetSkills => {
                            let plan = super::script_character_service::plan_reset_skills(&projected, true)?;
                            projected.status.known_skills = plan.known_skills.clone(); projected.status.script_skill_grants = plan.temporary_grants.clone();
                            projected.status.skill_point = plan.skill_points; projected.options = plan.options;
                            character_changes.push(ScriptCharacterChange::ResetSkills(plan));
                        }
                        _ => {},
                    }
                    if persistent_world_operation(*function) { world_calls.push((*function, arguments.clone())); }
                }
            }
        }
        let mut world = (!world_calls.is_empty() || !guild_storage_receipts.is_empty()).then(|| plan_persistent_effects(character, &world_calls, tick as u64)).transpose()?;
        if let Some(plan) = world.as_mut() {
            super::script_world_service::attach_guild_storage_receipts(character, plan, &guild_storage_receipts)?;
            grants.extend(plan.additional_grants.clone());
        }
        let persistent = consumption.is_some() || !grants.is_empty() || !removals.is_empty() || !variables.is_empty() || zeny.is_some() || world.is_some() || !character_changes.is_empty() || !pool_draws.is_empty();
        if persistent {
            let transaction = ScriptInventoryTransaction { char_id: character.char_id, account_id: character.account_id, consumption: consumption.cloned(), exact_removals: vec![], hp: None, sp: None,
                removals, identifications: vec![], grants, variables: variables.clone(), zeny, max_weight: server.character_service().max_weight(character), max_slots: 100, world: world.clone(), reset_skills: None, fame: None, character_changes: character_changes.clone(), pool_draws };
            let result = server.repository.script_inventory_transaction(&transaction).map_err(|error| error.to_string())?;
            self.install_inventory(server, character, result.inventory, consumption);
            for change in &character_changes {
                match change {
                    ScriptCharacterChange::Resources { hp, sp } => server.character_service().update_hp_sp(character, *hp, *sp),
                    ScriptCharacterChange::ResourcePools { hp, sp, max_hp, max_sp } => {
                        character.status.max_hp = *max_hp;
                        character.status.max_sp = *max_sp;
                        server.character_service().update_hp_sp(character, *hp, *sp);
                    }
                    ScriptCharacterChange::Experience(plan) => super::script_character_service::apply_experience(server, character, plan),
                    ScriptCharacterChange::ResetSkills(plan) => super::script_character_service::apply_reset_skills(server, character, plan),
                }
            }
            if character.status.zeny != result.zeny {
                character.status.zeny = result.zeny;
                let mut packet = PacketZcLongparChange::new(server.packetver());
                packet.set_amount(result.zeny as i32); packet.set_var_id(StatusTypes::Zeny.value() as u16); packet.fill_raw();
                self.send(character.char_id, packet.raw);
            }
            if let (Some(plan), Some(saved)) = (&world, result.world) {
                server.script_world_service().apply_committed_effects(server, state, character, plan, saved, tick as u64)?;
            }
            server.script_service().install_temporary_variables(character.char_id, &variables);
        }
        for effect in effects {
            match effect {
                ItemEffect::Heal { .. } => {},
                ItemEffect::GuildStorageOpen(_) => {},
                ItemEffect::Call { function, arguments } if function != Function::GetItem && function != Function::GetNamedItem && function != Function::GetItem2 && function != Function::DelItem && function != Function::ResetSkills && function != Function::GetExperience && !persistent_world_operation(function) => {
                    self.apply_game_call(server, state, character, function, arguments, tick)?;
                }
                ItemEffect::Write { .. } | ItemEffect::Grant { .. } | ItemEffect::Call { .. } | ItemEffect::PoolDraw(_) => {},
            }
        }
        character.refresh_script_context();
        Ok(())
    }

    pub(crate) fn install_inventory(&self, server: &Server, character: &mut Character, inventory: Vec<InventoryItemModel>, consumed: Option<&ScriptItemConsumption>) {
        let mut remaining: HashMap<_, _> = inventory.into_iter().map(|item| (item.id, item)).collect();
        let old = character.inventory.clone();
        for (index, previous) in old.iter().enumerate().filter_map(|(index, item)| item.as_ref().map(|item| (index, item))) {
            let current = remaining.remove(&previous.id);
            let new_amount = current.as_ref().map_or(0, |item| item.amount);
            if current.is_none() && previous.equip != 0 { self.notify_takeoff(character, index); }
            let removed = previous.amount.saturating_sub(new_amount);
            let used = consumed.filter(|used| used.inventory_id == previous.id).map_or(0, |used| used.amount);
            if removed > used { self.notify_removed(character.char_id, index, removed - used); }
            if new_amount > previous.amount { self.notify_grant(character.char_id, index, current.as_ref().unwrap(), new_amount - previous.amount); }
            character.inventory[index] = current;
        }
        let mut additions: Vec<_> = remaining.into_values().collect();
        additions.sort_by_key(|item| item.id);
        for item in additions {
            let index = character.add_in_inventory(item.clone());
            self.notify_grant(character.char_id, index, &item, item.amount);
        }
        server.add_to_next_tick(GameEvent::CharacterUpdateWeight(CharacterUpdateWeight { char_id: character.char_id }));
    }

    fn notify_takeoff(&self, character: &mut Character, index: usize) {
        use packets::packets::PacketZcReqTakeoffEquipAck2;
        if let Some(item) = character.takeoff_equip_item(index) {
            let mut packet = PacketZcReqTakeoffEquipAck2::new(self.configuration_service.packetver());
            packet.set_index(index as u16); packet.set_wear_location(item.location as u16); packet.set_result(0); packet.fill_raw();
            self.send(character.char_id, packet.raw);
        }
    }

    pub(crate) fn notify_removed(&self, char_id: u32, index: usize, amount: i16) {
        let mut packet = PacketZcItemThrowAck::new(self.configuration_service.packetver());
        packet.set_index(index as u16); packet.set_count(amount); packet.fill_raw(); self.send(char_id, packet.raw);
    }

    pub(crate) fn notify_grant(&self, char_id: u32, index: usize, item: &InventoryItemModel, amount: i16) {
        let mut packet = PacketZcItemPickupAck3::new(self.configuration_service.packetver());
        packet.set_itid(item.item_id as u16); packet.set_count(amount as u16); packet.set_index(index as u16);
        packet.set_is_identified(item.is_identified); packet.set_atype(item.item_type().to_client_type() as u8);
        packet.set_is_damaged(item.is_damaged);
        packet.set_refining_level(item.refine as u8);
        let mut slots = packets::packets::EQUIPSLOTINFO::new(self.configuration_service.packetver());
        slots.set_card1(item.card0 as u16); slots.set_card2(item.card1 as u16);
        slots.set_card3(item.card2 as u16); slots.set_card4(item.card3 as u16);
        packet.set_slot(slots);
        packet.set_location(self.configuration_service.get_item(item.item_id).location as u16); packet.fill_raw(); self.send(char_id, packet.raw);
    }

    fn send(&self, char_id: u32, packet: Vec<u8>) {
        self.client_notification_sender.send(Notification::Char(CharNotification::new(char_id, packet))).unwrap_or_else(|_| error!("Failed to notify item effect"));
    }

    fn warp_destination(&self, character: &Character, arguments: &[Value]) -> Result<(String, u16, u16, Option<u8>), String> {
        let name = arguments[0].text();
        let (name, x, y) = match name.as_str() {
            "SavePoint" => (Map::name_without_ext(&character.save_map).to_string(), character.save_x, character.save_y),
            "Random" => (Map::name_without_ext(character.current_map_name()).to_string(), RANDOM_CELL.0, RANDOM_CELL.1),
            _ => (Map::name_without_ext(super::instance_service::split_instance_map(&name).0).to_string(), u16::try_from(arguments[1].number_value()?).map_err(|_| "Invalid warp coordinate")?, u16::try_from(arguments[2].number_value()?).map_err(|_| "Invalid warp coordinate")?),
        };
        let map = self.configuration_service.find_map(&name).ok_or("Destination map is unavailable")?;
        if (x, y) != RANDOM_CELL && (x >= map.x_size() || y >= map.y_size()) { return Err("Warp coordinate is outside the destination map".into()); }
        let instance = super::instance_service::split_instance_map(&arguments[0].text()).1;
        Ok((name, x, y, instance))
    }

    pub(crate) fn spawn_request(&self, arguments: &[Value], owner_id: u32) -> Result<ScriptSpawn, String> {
        let number = |index: usize| arguments.get(index).ok_or("Missing monster argument")?.number_value();
        let map = arguments.first().ok_or("Missing monster map")?.string_value()?;
        if map != "this" && self.configuration_service.find_map(&Map::name_without_ext(super::instance_service::split_instance_map(map).0)).is_none() { return Err("Monster map is unavailable".into()); }
        let mob_id = number(4)?;
        if mob_id >= 0 && self.configuration_service.get_mob_safe(mob_id).is_none() || mob_id < 0 && !super::super::script::game_data::data().summons.iter().any(|group| group.id == -1 - mob_id) { return Err("Unknown summoned monster".into()); }
        let amount = u16::try_from(number(5)?).map_err(|_| "Invalid monster count")?;
        if amount == 0 || amount > 1000 { return Err("Monster count is out of bounds".into()); }
        let event = arguments.get(6).map(Value::text).unwrap_or_default();
        if !event.is_empty() && super::script_service::ScriptService::event_entry(&event).is_none() { return Err("Monster event is not a compiled event".into()); }
        Ok(ScriptSpawn { mob_id, x: number(1)?, y: number(2)?, name: arguments.get(3).ok_or("Missing monster name")?.text(), amount, event, event_npc: None,
            size: arguments.get(7).map(Value::number_value).transpose()?.map(|value| u8::try_from(value).map_err(|_| "Invalid monster size")).transpose()?,
            ai: arguments.get(8).map(Value::number_value).transpose()?.map(|value| u16::try_from(value).map_err(|_| "Invalid monster AI")).transpose()?, owner_id, guardian: None, bg_id: 0, max_hp: None, lifetime_ms: None, reserved_id: None, area_end: None })
    }

    fn apply_game_call(&self, server: &Server, state: &mut ServerState, character: &mut Character, function: Function, arguments: Vec<Value>, tick: u128) -> Result<(), String> {
        match function {
            Function::ItemSkill => {
                let skill = self.configuration_service.find_skill_config(&arguments[0]).ok_or("Unknown skill")?;
                server.script_skill_service().handle_skill(server, character, skill, arguments[1].number_value()? as u32, arguments.get(2).map(Value::truthy).unwrap_or(false))?;
            }
            Function::UnitSkill | Function::UnitSkillToId | Function::UnitSkillToPosition => {
                let mut request = super::script_unit_skill_service::unit_skill_request(self.configuration_service, character.char_id, function, &arguments)?;
                request.source_map = Some(character.current_map_name().clone());
                request.source_instance = Some(character.current_map_instance());
                server.add_to_next_tick(GameEvent::ScriptUnitSkill(request));
            }
            Function::StartStatus | Function::StartStatus2 | Function::StartStatus4 => {
                let (target, request) = status_request(function, &arguments, StatusStartFlag::NoDurationReduction.as_flag())?;
                if target.unwrap_or(character.char_id) == character.char_id { StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?; }
                else { server.add_to_next_tick(GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange { char_id: target.unwrap(), request })); }
            }
            Function::EndStatus => {
                let (target, kind) = status_to_end(&arguments)?;
                if target.unwrap_or(character.char_id) == character.char_id { StatusEffectService::end(server, character, kind, tick, &self.client_notification_sender); }
                else { server.add_to_next_tick(GameEvent::CharacterEndStatus(crate::server::model::events::game_event::CharacterEndStatus { char_id: target.unwrap(), kind })); }
            }
            Function::Warp => {
                let (map, x, y, instance) = self.warp_destination(character, &arguments)?;
                server.server_service().schedule_warp_to_walkable_cell_by_character_in_instance(&map, x, y, character.char_id, instance.unwrap_or(0));
            }
            Function::PartyWarp => { self.warp_party(server, character, &arguments)?; }
            Function::Monster => {
                let request = self.spawn_request(&arguments, character.char_id)?;
                let name = arguments[0].text();
                let map = if name == "this" { Map::name_without_ext(character.current_map_name()).to_string() } else { Map::name_without_ext(&name).to_string() };
                server.add_to_next_tick(GameEvent::ScriptSpawn(ScriptMapSpawn { char_id: character.char_id, map, request }));
            }
            Function::Produce | Function::Cooking => { self.open_crafting(character, function, &arguments)?; }
            Function::SpecialEffect | Function::SkillEffect | Function::SetFont | Function::Announce => { self.present_effect(server, character, function, &arguments)?; }
            Function::GetExperience => {
                server.character_service().gain_base_exp_unrated(character, arguments[0].number_value()? as u32);
                server.character_service().gain_job_exp_unrated(character, arguments[1].number_value()? as u32);
            }
            Function::ResetSkills => server.character_service().reset_skills(character, true),
            Function::OpenStorage => { server.script_world_service().call(server, state, character, function, &arguments, tick as u64)?; }
            function if ScriptWorldService::handles(function) => { server.script_world_service().call(server, state, character, function, &arguments, tick as u64)?; }
            function => return Err(format!("Invalid game effect {function:?}")),
        }
        Ok(())
    }

    fn healed_resources(&self, character: &Character, current_hp: u32, current_sp: u32, hp: i32, sp: i32, percentage: bool, item_scaling: bool, item_id: u32, ranked_potion: bool) -> (u32, u32) {
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let mut hp_delta = if percentage { i64::from(snapshot.max_hp()) * i64::from(hp) / 100 } else { i64::from(hp) };
        let mut sp_delta = if percentage { i64::from(snapshot.max_sp()) * i64::from(sp) / 100 } else { i64::from(sp) };
        if item_scaling {
            (hp_delta, sp_delta) = super::item_healing_service::scale_item_healing(&snapshot, &character.status.known_skills, item_id, hp_delta, sp_delta, ranked_potion,
                StatusChangeKind::from_name("SPIRIT").and_then(|kind| character.status.status_change(kind)).is_some_and(|change| self.configuration_service.find_skill_config(&"SL_ROGUE".into()).is_some_and(|skill| change.values[1] == skill.id as i32)));
        }
        if character.status.has_status_change(StatusChangeKind::NoRecovery) { hp_delta = hp_delta.min(0); sp_delta = sp_delta.min(0); }
        let hp = (i64::from(current_hp) + hp_delta).clamp(0, i64::from(snapshot.max_hp())) as u32;
        let sp = (i64::from(current_sp) + sp_delta).clamp(0, i64::from(snapshot.max_sp())) as u32;
        (hp, sp)
    }
}
