use std::sync::atomic::Ordering;

use tokio::sync::oneshot;

use models::enums::class::JobName;
use models::enums::look::LookType;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value, Variable, VariableScope};

use super::ScriptRequest;
use super::item_script_handler::{ItemScriptHost, status_variable};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::Server;
use crate::server::model::events::game_event::{
    CharacterAddItems, CharacterChangeJob, CharacterLook, CharacterRemoveItem, CharacterRemoveItems, GameEvent,
};
use crate::server::service::script_service::ScriptService;
use crate::server::state::server::ServerState;

fn variable_name(name: &str) -> (VariableScope, String) {
    crate::server::service::item_effect_service::script_variable_name(name)
}

impl ScriptService {
    /// Levels of Discount, Compulsion Discount and Overcharge, in the order of `shop::skill_ids`.
    fn shop_skill_levels(server: &Server, character: &crate::server::state::character::Character) -> [i32; 3] {
        let mut host = server.item_service().prepare_host(server, character, 0, false);
        super::shop::skill_ids().map(|id| {
            futures::executor::block_on(host.invoke(Request::Call { function: Function::GetSkillLv, arguments: vec![Value::Number(id)] }))
                .ok()
                .and_then(|level| level.number_value().ok())
                .unwrap_or(0)
        })
    }

    fn schedule_game_event(server: &Server, context: &ScriptRequest, event: GameEvent) {
        let event = if let Some(token) = context.logout_token {
            GameEvent::ScriptLogoutAction(crate::server::model::character_lifecycle::ScriptLogoutAction {
                char_id: context.char_id, token, action: Box::new(event),
            })
        } else { event };
        server.add_to_next_tick(event);
    }
    fn validate_variable_scope(context: &ScriptRequest, scope: VariableScope) -> Result<(), String> {
        if context.char_id == 0 && matches!(scope, VariableScope::Character | VariableScope::CharacterTemporary | VariableScope::Account) {
            return Err("Script variable requires an attached player".into());
        }
        Ok(())
    }
    pub(crate) fn read_item_variable(&self, server: &Server, character: &crate::server::state::character::Character, name: &str) -> Reply {
        let (scope, name) = crate::server::service::item_effect_service::script_variable_name(name);
        let context = ScriptRequest { char_id: character.char_id, account_id: character.account_id, npc_id: 0, npc_entry: 0, npc_scope_instance: 0,
            map_instance: character.current_map_instance(), generation: 0, background: false, event_depth: 0, timer_context: None, logout_token: None,
            request: Request::Read(name.clone()), response: std::sync::Arc::new(std::sync::Mutex::new(None)) };
        self.read_variable(server, &context, scope, name, 0)
    }

    fn temporary_key(context: &ScriptRequest, scope: VariableScope, name: &str, index: u32) -> (u32, u8, u32, String, u32) {
        match scope {
            VariableScope::Npc => (0, 0, context.npc_entry, name.into(), index),
            VariableScope::NpcInstance => (1, context.npc_scope_instance, context.npc_id, name.into(), index),
            VariableScope::Instance => (4, 0, u32::from(context.npc_scope_instance), name.into(), index),
            VariableScope::ServerTemporary => (3, 0, 0, name.into(), index),
            _ => (2, 0, context.char_id, name.into(), index),
        }
    }

    pub(crate) fn set_server_temporary(&self, name: &str, value: Value) {
        let (_, name) = variable_name(name);
        self.npc_variables.lock().unwrap().insert((3, 0, 0, name, 0), value);
    }

    pub(crate) fn server_temporary(&self, name: &str) -> Option<Value> {
        let (_, name) = variable_name(name);
        self.npc_variables.lock().unwrap().get(&(3, 0, 0, name, 0)).cloned()
    }

    fn read_variable(&self, server: &Server, context: &ScriptRequest, scope: VariableScope, name: String, index: u32) -> Reply {
        Self::validate_variable_scope(context, scope)?;
        let string = name.ends_with('$');
        let repository = &server.repository;
        Ok(match (scope, string) {
            (VariableScope::Character, false) => repository.script_variable_char_num_fetch_one(context.char_id, name, index).into(),
            (VariableScope::Character, true) => repository.script_variable_char_str_fetch_one(context.char_id, name, index).into(),
            (VariableScope::Account, false) => repository
                .script_variable_account_num_fetch_one(context.account_id, name, index)
                .into(),
            (VariableScope::Account, true) => repository
                .script_variable_account_str_fetch_one(context.account_id, name, index)
                .into(),
            (VariableScope::Server, false) => repository.script_variable_server_num_fetch_one(name, index).into(),
            (VariableScope::Server, true) => repository.script_variable_server_str_fetch_one(name, index).into(),
            _ => self
                .npc_variables
                .lock()
                .unwrap()
                .get(&Self::temporary_key(context, scope, &name, index))
                .cloned()
                .unwrap_or_else(|| if string { Value::String(String::new()) } else { Value::default() }),
        })
    }

    fn write_variables(&self, server: &Server, context: &ScriptRequest, variables: Vec<Variable>) -> Reply {
        for variable in &variables {
            Self::validate_variable_scope(context, variable.scope)?;
            if variable.name.is_empty()
                || variable.name.len() > 128
                || !matches!(variable.value, Value::Number(_) | Value::String(_))
                || variable.name.ends_with('$') != variable.value.is_string()
            {
                return Err("Invalid script variable".into());
            }
        }
        server
            .repository
            .script_variables_save_batch(context.char_id, context.account_id, &variables)
            .map_err(|e| e.to_string())?;
        let mut temporary = self.npc_variables.lock().unwrap();
        for variable in variables {
            if matches!(
                variable.scope,
                VariableScope::CharacterTemporary | VariableScope::ServerTemporary | VariableScope::Npc | VariableScope::NpcInstance | VariableScope::Instance
            ) {
                temporary.insert(
                    Self::temporary_key(context, variable.scope, &variable.name, variable.index),
                    variable.value,
                );
            }
        }
        Ok(Value::default())
    }

    pub fn handle_request(&self, server: &Server, state: &mut ServerState, context: ScriptRequest) {
        if super::unit_data::handle_request(server, state, &context) {
            return;
        }
        let response = context.response.lock().unwrap().take();
        let Some(response) = response else {
            return;
        };
        if response.is_closed() {
            script_debug!("Script request dropped, the script already ended: npc={} char={} {}", context.npc_id, context.char_id, request_summary(&context.request));
            return;
        }
        let valid = if context.logout_token.is_some() {
            server.valid_logout_request(state, &context)
        } else if state.pending_character_logouts.contains_key(&context.char_id) {
            false
        } else if context.background && context.char_id == 0 && context.account_id == 0 {
            super::unit_data::script_actor(state, &context).is_ok_and(|npc| npc.is_some())
        } else { state.find_session(context.account_id).is_some_and(|session| {
            (context.background || session.script_generation.load(Ordering::Acquire) == context.generation) && session.char_id == Some(context.char_id)
        }) && state.characters().contains_key(&context.char_id) };
        if valid {
            if let Request::Call { function: function @ (Function::Sleep | Function::ProgressBar), arguments } = &context.request {
                self.delay_script(server, &context, *function, arguments, response);
                return;
            }
            if let Request::Call { function: Function::InstanceCreate, arguments } = &context.request {
                server.instance_create_deferred(state, &context, arguments, response);
                return;
            }
        }
        let reply = if valid {
            self.process_request(server, state, &context)
        } else {
            Err("Conversation is no longer active".into())
        };
        if let Err(error) = &reply {
            script_debug!("Script request failed: npc={} char={} {}: {error}", context.npc_id, context.char_id, request_summary(&context.request));
        }
        let _ = response.send(reply);
    }

    /// `sleep` and `progressbar`: the script resumes after the delay, the game loop is not blocked.
    fn delay_script(&self, server: &Server, context: &ScriptRequest, function: Function, arguments: &[Value], response: oneshot::Sender<Reply>) {
        const LONGEST_DELAY_MS: u64 = 5 * 60 * 1000;
        let milliseconds = match function {
            Function::Sleep => arguments.first().and_then(|value| value.number_value().ok()).unwrap_or(0).max(0) as u64,
            _ => arguments.get(1).and_then(|value| value.number_value().ok()).unwrap_or(0).max(0) as u64 * 1000,
        }
        .min(LONGEST_DELAY_MS);
        if function == Function::ProgressBar && context.char_id != 0 {
            let color = arguments.first().map(|value| value.text()).unwrap_or_default();
            let color = u32::from_str_radix(color.trim_start_matches("0x").trim_start_matches("0X"), 16).unwrap_or(0xFFFFFF);
            let mut packet = 0x02f0_u16.to_le_bytes().to_vec();
            packet.extend_from_slice(&color.to_le_bytes());
            packet.extend_from_slice(&((milliseconds / 1000) as u32).to_le_bytes());
            server.send_raw(context.char_id, packet);
        }
        server.runtime().spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(milliseconds)).await;
            let _ = response.send(Ok(Value::default()));
        });
    }

    fn npc_variable_call(&self, state: &ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let name = arguments.first().ok_or("Missing variable name")?.text();
        let name = name.strip_prefix('.').ok_or("getvariableofnpc needs a .variable")?.to_string();
        let target = arguments.get(1).ok_or("Missing NPC name")?.text();
        let index = u32::try_from(arguments.get(2).ok_or("Missing array index")?.number_value()?).map_err(|_| "Negative array index")?;
        let Some((actor, script)) = crate::server::service::npc_event_service::named_npc(state, context, &target)? else {
            return Err(format!("NPC {target} does not exist"));
        };
        let key = (1, script.scope_instance, actor.id, name.clone(), index);
        if function == Function::GetVariableOfNpc {
            let default = if name.ends_with('$') { Value::String(String::new()) } else { Value::default() };
            return Ok(self.npc_variables.lock().unwrap().get(&key).cloned().unwrap_or(default));
        }
        let value = arguments.get(3).ok_or("Missing value")?.clone();
        if name.ends_with('$') != value.is_string() || !matches!(value, Value::Number(_) | Value::String(_)) {
            return Err("Invalid script variable".into());
        }
        self.npc_variables.lock().unwrap().insert(key, value);
        Ok(Value::default())
    }

    fn process_request(&self, server: &Server, state: &mut ServerState, context: &ScriptRequest) -> Reply {
        match context.request.clone() {
            Request::Read(name) => {
                if let Some(character) = state.characters().get(&context.char_id) {
                    if let Some(value) = status_variable(&character.status, &name) { return Ok(value); }
                    match name.as_str() {
                        "Weight" => return Ok(Value::Number(character.weight() as i32)),
                        "MaxWeight" => return Ok(Value::Number(server.character_service().max_weight(character) as i32)),
                        _ => {}
                    }
                }
                let (scope, name) = variable_name(&name);
                self.read_variable(server, context, scope, name, 0)
            }
            Request::VariableRead { scope, name, index } => self.read_variable(server, context, scope, name, index),
            Request::VariablesWrite(variables) => self.write_variables(server, context, variables),
            Request::VariablesIncrement(variables) => {
                let mut stored = self.npc_variables.lock().unwrap();
                let mut staged = std::collections::HashMap::new();
                let mut result = vec![Value::default(); variables.len()];
                let mut persistent = vec![];
                let mut positions = vec![];
                for (position, variable) in variables.into_iter().enumerate() {
                    Self::validate_variable_scope(context, variable.scope)?;
                    if variable.name.is_empty() || variable.name.len() > 128 || variable.name.ends_with('$') || !matches!(variable.value, Value::Number(_)) {
                        return Err("Counter scope or name is invalid".into());
                    }
                    if matches!(variable.scope, VariableScope::Character | VariableScope::Account | VariableScope::Server) {
                        persistent.push(variable); positions.push(position); continue;
                    }
                    let key = Self::temporary_key(context, variable.scope, &variable.name, variable.index);
                    let current: Value = staged.get(&key).or_else(|| stored.get(&key)).cloned().unwrap_or_default();
                    let value = Value::Number(
                        current
                            .number_value()?
                            .checked_add(variable.value.number_value()?)
                            .ok_or("Counter overflow")?,
                    );
                    staged.insert(key, value.clone());
                    result[position] = value;
                }
                if !persistent.is_empty() {
                    let committed = server.repository.script_variables_increment_batch(context.char_id, context.account_id, &persistent).map_err(|error| error.to_string())?;
                    for (position, value) in positions.into_iter().zip(committed) { result[position] = value.into(); }
                }
                stored.extend(staged);
                Ok(Value::Array(result))
            }
            Request::Write { name, value } => {
                if name == "Zeny" {
                    if context.char_id == 0 { return Err("Zeny requires an attached player".into()); }
                    let amount = u32::try_from(value.number_value()?).map_err(|_| "Invalid zeny")?;
                    Self::schedule_game_event(server, context, GameEvent::CharacterUpdateZeny(
                        crate::server::model::events::game_event::CharacterZeny {
                            char_id: context.char_id,
                            zeny: Some(amount),
                        },
                    ));
                    return Ok(Value::default());
                }
                let (scope, name) = variable_name(&name);
                self.write_variables(server, context, vec![Variable {
                    scope,
                    name,
                    index: 0,
                    value,
                }])
            }
            Request::Inventory => {
                let character = state.characters().get(&context.char_id).ok_or("Character disconnected")?;
                Ok(Value::Array(
                    character
                        .inventory
                        .iter()
                        .enumerate()
                        .filter_map(|(index, item)| {
                            item.as_ref().filter(|item| item.equip == 0).map(|item| {
                                let price = self
                                    .configuration_service
                                    .find_item(item.item_id)
                                    .and_then(|item| item.price_sell.or(item.price_buy))
                                    .unwrap_or(0);
                                Value::Array(vec![
                                    Value::Number(index as i32),
                                    item.item_id.into(),
                                    i32::from(item.amount).into(),
                                    price.into(),
                                ])
                            })
                        })
                        .collect(),
                ))
            }
            Request::Purchase(items) => {
                let character = state.characters().get(&context.char_id).ok_or("Character disconnected")?;
                let instance = state
                    .get_map_instance(character.current_map_name(), context.map_instance)
                    .ok_or("Shop map is unavailable")?;
                let script = instance.get_script(context.npc_id).ok_or("Shop is unavailable")?;
                if script.entry_id != 6 || script.constructor_args.len() < 3 {
                    return Err("NPC is not a shop".into());
                }
                let mut offers = std::collections::HashMap::new();
                for pair in script.constructor_args[1..].chunks_exact(2) {
                    let id = pair[0].number_value()?;
                    let override_price = pair[1].number_value()?;
                    let item = self.configuration_service.find_item(id).ok_or("Unknown shop item")?;
                    offers.insert(
                        id as u32,
                        if override_price == -1 {
                            item.price_buy.unwrap_or(0)
                        } else {
                            override_price
                        },
                    );
                }
                let levels = Self::shop_skill_levels(server, character);
                let mut additions = vec![];
                for (id, amount, price) in items {
                    if amount <= 0 || price < 0 || offers.get(&id) != Some(&price) {
                        return Err("Invalid purchase".into());
                    }
                    let item = self.configuration_service.find_item(id as i32).ok_or("Unknown shop item")?;
                    let mut model = InventoryItemModel::from_item_model(item, amount, true);
                    model.shop_price = Some(super::shop::discounted_price(price, levels[0], levels[1]));
                    additions.push(model);
                }
                let character = state.characters_mut().get_mut(&context.char_id).ok_or("Character disconnected")?;
                let total_weight: i64 = additions.iter().map(|item| i64::from(item.weight) * i64::from(item.amount)).sum();
                if total_weight < 0
                    || total_weight > i64::from(u32::MAX)
                    || !server.character_service().can_carry_weight(character, total_weight as u32)
                {
                    return Err("You cannot hold this amount of items".into());
                }
                server.inventory_service().try_add_items_in_inventory(
                    server.runtime(),
                    CharacterAddItems {
                        char_id: context.char_id,
                        should_perform_check: true,
                        buy: true,
                        items: additions,
                    },
                    character,
                )?;
                Ok(Value::default())
            }
            Request::Sale(items) => {
                let character = state.characters_mut().get_mut(&context.char_id).ok_or("Character disconnected")?;
                let levels = Self::shop_skill_levels(server, character);
                let mut removals = vec![];
                for (index, amount, price) in items {
                    let item = character.get_item_from_inventory(index).ok_or("Invalid sale item")?;
                    let expected = self
                        .configuration_service
                        .find_item(item.item_id)
                        .and_then(|item| item.price_sell.or(item.price_buy))
                        .unwrap_or(0);
                    if amount <= 0 || amount > item.amount || item.equip != 0 || price != expected || price < 0 {
                        return Err("Invalid sale".into());
                    }
                    removals.push(CharacterRemoveItem {
                        char_id: context.char_id,
                        index,
                        amount,
                        price: super::shop::overcharged_price(price, levels[2]),
                    });
                }
                server
                    .inventory_service()
                    .remove_item_from_inventory(
                        server.runtime(),
                        CharacterRemoveItems {
                            char_id: context.char_id,
                            sell: true,
                            items: removals,
                            notify_client: true,
                        },
                        character,
                    )
                    .map_err(|error| error.to_string())?;
                Ok(Value::default())
            }
            Request::Call { function, arguments } => {
                if function == Function::WarpPortal {
                    let source = super::unit_data::script_actor(state, context)?.ok_or("Warp Portal command requires an NPC")?;
                    return server.script_skill_service().create_npc_warp_portal(state, &source, &arguments, crate::util::tick::get_tick());
                }
                if crate::server::service::npc_timer_service::handles(function) {
                    return server.npc_timer_call(state, context, function, &arguments, crate::util::tick::get_tick());
                }
                if matches!(function, Function::GetNpcId | Function::DoEvent | Function::DoNpcEvent) {
                    return server.npc_event_call(state, context, function, &arguments);
                }
                if matches!(function, Function::GetVariableOfNpc | Function::SetVariableOfNpc) {
                    return self.npc_variable_call(state, context, function, &arguments);
                }
                if function == Function::Print {
                    debug!("NPC {}: {}", context.npc_id, arguments.iter().map(|value| value.text()).collect::<Vec<_>>().join(" "));
                    return Ok(Value::default());
                }
                if (context.char_id == 0 && function == Function::Announce)
                    || (matches!(function, Function::Monster | Function::AreaMonster) && super::unit_data::script_actor(state, context)?.is_some()) {
                    return server.npc_background_call(state, context, function, &arguments);
                }
                if matches!(function, Function::Rand | Function::Min | Function::Max | Function::Pow | Function::GetTime | Function::GetItemInfo | Function::GetItemName) {
                    let mut host = ItemScriptHost::bonuses(models::status::Status::default(), 0);
                    return futures::executor::block_on(host.invoke(Request::Call { function, arguments }));
                }
                if matches!(function, Function::UnitSkill | Function::UnitSkillToId | Function::UnitSkillToPosition) {
                    let default_source = if context.char_id == 0 { context.npc_id } else { context.char_id };
                    let mut request = crate::server::service::script_unit_skill_service::unit_skill_request(self.configuration_service, default_source, function, &arguments)?;
                    crate::server::service::script_unit_skill_service::normalize_unit_skill_actor_ids(state, &mut request);
                    let actor = super::unit_data::request_actor(server, state, context, request.source_id)?.ok_or("Unit skill caster is unavailable")?;
                    request.source_map = Some(actor.map);
                    request.source_instance = Some(actor.instance);
                    server.script_skill_service().cast_script_unit_skill(server, state, request, crate::util::tick::get_tick())?;
                    return Ok(Value::default());
                }
                if matches!(function, Function::StartStatus | Function::StartStatus2 | Function::StartStatus4) {
                    use models::enums::EnumWithMaskValueU32;
                    use crate::server::model::map_item::MapItemType;
                    use crate::server::model::events::map_event::MapEvent;
                    let (target, request) = crate::server::service::item_effect_service::status_request(function, &arguments, models::status_change::StatusStartFlag::NoAvoid.as_flag())?;
                    let target = super::unit_data::request_actor(server, state, context, target.unwrap_or(context.char_id))?.ok_or("Status target is unavailable")?;
                    if target.object_type == MapItemType::Character {
                        let mut character = state.characters_mut().remove(&target.id).ok_or("Status target disconnected")?;
                        let result = crate::server::service::status_effect_service::StatusEffectService::start(server, &mut character, request, crate::util::tick::get_tick(), &server.server_service().notification_sender());
                        state.insert_character(character);
                        result?;
                    } else if target.object_type == MapItemType::Npc {
                        let map = state.get_map_instance(&target.map, target.instance).ok_or("Status target map is unavailable")?;
                        map.add_to_next_tick(MapEvent::NpcEffect(crate::server::service::map_npc_effect::MapNpcEffect {
                            actor_id: target.id, effect: crate::server::service::map_npc_effect::NpcEffect::Status(request),
                        }));
                    } else {
                        Self::schedule_game_event(server, context, GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange { char_id: target.id, request }));
                    }
                    return Ok(Value::default());
                }
                if function == Function::EndStatus {
                    use crate::server::model::map_item::MapItemType;
                    use crate::server::model::events::map_event::MapEvent;
                    let (target, kind) = crate::server::service::item_effect_service::status_to_end(&arguments)?;
                    let target = super::unit_data::request_actor(server, state, context, target.unwrap_or(context.char_id))?.ok_or("Status target is unavailable")?;
                    if target.object_type == MapItemType::Character {
                        let mut character = state.characters_mut().remove(&target.id).ok_or("Status target disconnected")?;
                        crate::server::service::status_effect_service::StatusEffectService::end(server, &mut character, kind, crate::util::tick::get_tick(), &server.server_service().notification_sender());
                        state.insert_character(character);
                    } else if target.object_type == MapItemType::Npc {
                        let map = state.get_map_instance(&target.map, target.instance).ok_or("Status target map is unavailable")?;
                        map.add_to_next_tick(MapEvent::NpcEffect(crate::server::service::map_npc_effect::MapNpcEffect {
                            actor_id: target.id, effect: crate::server::service::map_npc_effect::NpcEffect::EndStatus(kind),
                        }));
                    } else {
                        Self::schedule_game_event(server, context, GameEvent::CharacterEndStatus(crate::server::model::events::game_event::CharacterEndStatus { char_id: target.id, kind }));
                    }
                    return Ok(Value::default());
                }
                if matches!(function, Function::GetCastleData | Function::SetCastleData) {
                    let map = arguments.first().ok_or("Castle map is required")?.string_value()?.to_string();
                    if !state.map_flags_for(&map, 0).enabled(crate::server::model::map_flags::MapFlag::GvgCastle) {
                        return Ok(Value::Number(0));
                    }
                    let field = u8::try_from(arguments.get(1).ok_or("Castle data selector is required")?.number_value()?)
                        .map_err(|_| "Unknown castle data selector")?;
                    if function == Function::GetCastleData {
                        return Ok(Value::Number(server.repository.castle_value(&map, field).map_err(|error| error.to_string())?));
                    }
                    let value = arguments.get(2).ok_or("Castle data value is required")?.number_value()?;
                    server.repository.set_castle_value(&map, field, value).map_err(|error| error.to_string())?;
                    return Ok(Value::Number(0));
                }
                if crate::server::service::battleground_service::handles(function) {
                    return server.battleground_call(state, context.char_id, function, &arguments);
                }
                if crate::server::service::quest_service::handles(function) {
                    return server.quest_script_call(state, context, function, &arguments);
                }
                if function == Function::SpecialEffect && context.char_id == 0 {
                    return server.script_npc_effect(state, context, &arguments);
                }
                if crate::server::service::script_npc_commands::handles(function) {
                    return server.script_npc_call(state, context, function, &arguments);
                }
                if crate::server::service::script_refine_service::handles(function) {
                    return server.script_refine_call(state, context, function, &arguments);
                }
                if crate::server::service::instance_service::handles(function) {
                    return server.script_instance_call(state, context, function, &arguments);
                }
                if crate::server::service::script_map_commands::handles(function) {
                    return server.script_map_call(state, context, function, &arguments);
                }
                if crate::server::service::battleground_queue_service::handles_script_call(function) {
                    return server.battleground_queue_script_call(state, function, &arguments);
                }
                if matches!(function, Function::GetGuildInfo | Function::GetGuildSkillLevel | Function::GuardianSummon) {
                    return server.castle_script_call(context.char_id, function, &arguments);
                }
                if matches!(function, Function::AgitStart | Function::AgitEnd | Function::AgitCheck) {
                    return Ok(Value::Number(match function {
                        Function::AgitCheck => i32::from(state.siege_active()),
                        _ => i32::from(server.set_siege_active(state, function == Function::AgitStart)),
                    }));
                }
                if crate::server::service::map_flag_service::is_map_flag_call(function) {
                    if let Some(npc) = super::unit_data::script_actor(state, context)? {
                        let key = crate::server::model::map_instance::MapInstanceKey::new(npc.map, npc.instance);
                        return server.map_flag_call_from(state, Some(&key), function, &arguments);
                    }
                    return server.map_flag_call(state, context.char_id, function, &arguments);
                }
                if function == Function::GetCharacterId {
                    let kind = arguments.first().ok_or("Missing character identifier type")?.number_value()?;
                    let character = if let Some(name) = arguments.get(1) {
                        let name = name.string_value()?;
                        state.characters().values().find(|character| character.name == *name)
                    } else { state.get_character(context.char_id) };
                    return Ok(Value::Number(character.map_or(0, |character| match kind {
                        0 => character.char_id,
                        1 => character.game_systems.party_id,
                        2 => character.game_systems.guild_id,
                        3 => character.account_id,
                        4 => character.bg_id,
                        _ => 0,
                    }) as i32));
                }
                let number = |index: usize| arguments.get(index).ok_or_else(|| "Missing argument".to_string())?.number_value();
                let target_argument = match function {
                    Function::ResetLevel | Function::AddFame => Some(1),
                    Function::GetFame | Function::GetFameRank => Some(0),
                    Function::SavePoint => Some(if arguments.len() > 4 { 5 } else { 3 }),
                    Function::GetSavePoint => Some(1),
                    _ => None,
                };
                let target_id = target_argument.and_then(|index| arguments.get(index)).map(Value::number_value).transpose()?
                    .map(|id| u32::try_from(id).map_err(|_| "Invalid script target character")).transpose()?.filter(|id| *id != 0).unwrap_or(context.char_id);
                let mut character = state.characters_mut().remove(&target_id).ok_or("Script target character disconnected")?;
                let result: Reply = (|| { match function {
                    Function::SavePoint => crate::server::service::map_position_service::set_save_point(server, state, &mut character, &arguments),
                    Function::GetSavePoint => {
                        if !(1..=2).contains(&arguments.len()) { return Err("GetSavePoint requires a type and optional character ID".into()); }
                        Ok(match number(0)? {
                            0 => crate::server::service::map_flag_service::normalize_map(&character.save_map).into(),
                            1 => i32::from(character.save_x).into(),
                            2 => i32::from(character.save_y).into(),
                            _ => 0.into(),
                        })
                    }
                    Function::GetLook => Ok(Value::Number(
                        character.get_look(LookType::try_from_value(number(0)? as usize).map_err(|_| "Invalid look type")?) as i32,
                    )),
                    Function::SetLook => {
                        let look_type = LookType::try_from_value(number(0)? as usize).map_err(|_| "Invalid look type")?;
                        let look_value = u16::try_from(number(1)?).map_err(|_| "Invalid look value")?;
                        Self::schedule_game_event(server, context, GameEvent::CharacterUpdateLook(CharacterLook {
                            char_id: context.char_id,
                            look_type,
                            look_value,
                        }));
                        Ok(Value::default())
                    }
                    Function::GetBattleFlag => {
                        let flag = arguments.first().ok_or("Missing flag")?.text();
                        if !matches!(
                            flag.as_str(),
                            "min_hair_style"
                                | "max_hair_style"
                                | "min_hair_color"
                                | "max_hair_color"
                                | "min_cloth_color"
                                | "max_cloth_color"
                                | "feature.refineui"
                        ) {
                            return Err("Unknown battle flag".into());
                        }
                        Ok(super::constant::get_battle_flag(&flag))
                    }
                    Function::StrCharInfo => Ok(match number(0)? {
                        0 => character.name.clone().into(),
                        1 => character.game_systems.party_name.clone().into(),
                        2 => character.guild_name.clone().into(),
                        3 => character.current_map_name().clone().into(),
                        _ => Value::String(String::new()),
                    }),
                    Function::JobName => Ok(JobName::try_from_value(number(0)? as usize)
                        .map_err(|_| "Invalid job")?
                        .as_str()
                        .into()),
                    Function::EaClass => Ok(Value::Number(
                        JobName::try_from_value(if arguments.is_empty() {
                            character.status.job as usize
                        } else {
                            number(0)? as usize
                        })
                        .map_err(|_| "Invalid job")?
                        .mask() as i32,
                    )),
                    Function::RoClass => Ok(Value::Number(
                        JobName::from_mask(number(0)? as u64, character.status.is_male).map_or(-1, |job| job.value() as i32),
                    )),
                    Function::JobChange => {
                        let job = JobName::try_from_value(number(0)? as usize).map_err(|_| "Invalid job")?;
                        Self::schedule_game_event(server, context, GameEvent::CharacterChangeJob(CharacterChangeJob {
                            char_id: context.char_id,
                            job,
                            should_reset_skills: false,
                        }));
                        Ok(Value::default())
                    }
                    Function::ResetLevel => {
                        crate::server::service::script_character_service::reset_level(server, &mut character, &arguments)
                    }
                    Function::Skill => crate::server::service::script_character_service::grant_skill(server, &mut character, &arguments),
                    Function::GetFame => crate::server::service::script_character_service::get_fame(server, &character, false),
                    Function::GetFameRank => crate::server::service::script_character_service::get_fame(server, &character, true),
                    Function::AddFame => crate::server::service::script_character_service::add_fame(server, &mut character, &arguments),
                    Function::Print => {
                        debug!(
                            "NPC {}: {}",
                            context.npc_id,
                            arguments.iter().map(|value| value.text()).collect::<Vec<_>>().join(" ")
                        );
                        Ok(Value::default())
                    }
                    function if crate::server::service::script_world_service::ScriptWorldService::handles(function) => {
                        server.script_world_service().call(server, state, &mut character, function, &arguments, crate::util::tick::get_tick() as u64)
                    }
                    function => {
                        let mut host = server.item_service().prepare_host(server, &character, 0, true);
                        let reply = futures::executor::block_on(host.invoke(Request::Call { function, arguments: arguments.clone() }))?;
                        if !host.bonuses.drain().is_empty() { return Err("Bonus grants require an equipment or automatic-bonus program".into()); }
                        server.item_service().apply_effects(server, state, server.runtime(), &mut character, host.effects)?;
                        Ok(reply)
                    }
                } })();
                state.insert_character(character);
                result
            }
            _ => Err("Unsupported game request".into()),
        }
    }
}

fn request_summary(request: &Request) -> String {
    let text = format!("{request:?}");
    if text.chars().count() > 160 {
        let mut shortened: String = text.chars().take(160).collect();
        shortened.push('~');
        return shortened;
    }
    text
}
