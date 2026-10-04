use std::sync::atomic::Ordering;

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
    CharacterAddItems, CharacterChangeJob, CharacterChangeLevel, CharacterLook, CharacterRemoveItem, CharacterRemoveItems, GameEvent,
};
use crate::server::service::script_service::ScriptService;
use crate::server::state::server::ServerState;

fn variable_name(name: &str) -> (VariableScope, String) {
    crate::server::service::item_effect_service::script_variable_name(name)
}

impl ScriptService {
    pub(crate) fn read_item_variable(&self, server: &Server, character: &crate::server::state::character::Character, name: &str) -> Reply {
        let (scope, name) = crate::server::service::item_effect_service::script_variable_name(name);
        let context = ScriptRequest { char_id: character.char_id, account_id: character.account_id, npc_id: 0, npc_entry: 0,
            map_instance: character.current_map_instance(), generation: 0, background: false,
            request: Request::Read(name.clone()), response: std::sync::Arc::new(std::sync::Mutex::new(None)) };
        self.read_variable(server, &context, scope, name, 0)
    }

    fn temporary_key(context: &ScriptRequest, scope: VariableScope, name: &str, index: u32) -> (u32, u8, u32, String, u32) {
        match scope {
            VariableScope::Npc => (0, 0, context.npc_entry, name.into(), index),
            VariableScope::NpcInstance => (1, context.map_instance, context.npc_id, name.into(), index),
            VariableScope::ServerTemporary => (3, 0, 0, name.into(), index),
            _ => (2, 0, context.char_id, name.into(), index),
        }
    }

    fn read_variable(&self, server: &Server, context: &ScriptRequest, scope: VariableScope, name: String, index: u32) -> Reply {
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
                VariableScope::CharacterTemporary | VariableScope::ServerTemporary | VariableScope::Npc | VariableScope::NpcInstance
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
        let response = context.response.lock().unwrap().take();
        let Some(response) = response else {
            return;
        };
        if response.is_closed() {
            return;
        }
        let valid = state.find_session(context.account_id).is_some_and(|session| {
            (context.background || session.script_generation.load(Ordering::Acquire) == context.generation) && session.char_id == Some(context.char_id)
        });
        let reply = if valid && state.characters().contains_key(&context.char_id) {
            self.process_request(server, state, &context)
        } else {
            Err("Conversation is no longer active".into())
        };
        let _ = response.send(reply);
    }

    fn process_request(&self, server: &Server, state: &mut ServerState, context: &ScriptRequest) -> Reply {
        match context.request.clone() {
            Request::Read(name) => {
                let character = state.characters().get(&context.char_id).ok_or("Character disconnected")?;
                if let Some(value) = status_variable(&character.status, &name) {
                    return Ok(value);
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
                    let amount = u32::try_from(value.number_value()?).map_err(|_| "Invalid zeny")?;
                    server.add_to_next_tick(GameEvent::CharacterUpdateZeny(
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
                let mut additions = vec![];
                for (id, amount, price) in items {
                    if amount <= 0 || price < 0 || offers.get(&id) != Some(&price) {
                        return Err("Invalid purchase".into());
                    }
                    let item = self.configuration_service.find_item(id as i32).ok_or("Unknown shop item")?;
                    let mut model = InventoryItemModel::from_item_model(item, amount, true);
                    model.shop_price = Some(price);
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
                        price,
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
                if crate::server::service::map_flag_service::is_map_flag_call(function) {
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
                        _ => 0,
                    }) as i32));
                }
                let number = |index: usize| arguments.get(index).ok_or_else(|| "Missing argument".to_string())?.number_value();
                let target_argument = match function {
                    Function::ResetLevel | Function::AddFame => Some(1),
                    Function::GetFame | Function::GetFameRank => Some(0),
                    _ => None,
                };
                let target_id = target_argument.and_then(|index| arguments.get(index)).map(Value::number_value).transpose()?
                    .map(|id| u32::try_from(id).map_err(|_| "Invalid script target character")).transpose()?.filter(|id| *id != 0).unwrap_or(context.char_id);
                let mut character = state.characters_mut().remove(&target_id).ok_or("Script target character disconnected")?;
                let result: Reply = (|| { match function {
                    Function::GetLook => Ok(Value::Number(
                        character.get_look(LookType::try_from_value(number(0)? as usize).map_err(|_| "Invalid look type")?) as i32,
                    )),
                    Function::SetLook => {
                        let look_type = LookType::try_from_value(number(0)? as usize).map_err(|_| "Invalid look type")?;
                        let look_value = u16::try_from(number(1)?).map_err(|_| "Invalid look value")?;
                        server.add_to_next_tick(GameEvent::CharacterUpdateLook(CharacterLook {
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
                        server.add_to_next_tick(GameEvent::CharacterChangeJob(CharacterChangeJob {
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
                    Function::StartStatus | Function::StartStatus2 | Function::StartStatus4 => {
                        use models::enums::EnumWithMaskValueU32;
                        let (target, request) = crate::server::service::item_effect_service::status_request(function, &arguments,
                            models::status_change::StatusStartFlag::NoAvoid.as_flag())?;
                        if target.is_some_and(|target| target != character.char_id) {
                            server.add_to_next_tick(GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange { char_id: target.unwrap(), request }));
                        } else {
                            crate::server::service::status_effect_service::StatusEffectService::start(server, &mut character, request, crate::util::tick::get_tick(), &server.server_service().notification_sender())?;
                        }
                        Ok(Value::default())
                    }
                    Function::Print => {
                        debug!(
                            "NPC {}: {}",
                            context.npc_id,
                            arguments.iter().map(|value| value.text()).collect::<Vec<_>>().join(" ")
                        );
                        Ok(Value::default())
                    }
                    function if crate::server::service::script_world_service::ScriptWorldService::handles(function) => {
                        server.script_world_service().call(server, &mut character, function, &arguments, crate::util::tick::get_tick() as u64)
                    }
                    function => {
                        let mut host = server.item_service().prepare_host(server, &character, 0, true);
                        let reply = futures::executor::block_on(host.invoke(Request::Call { function, arguments: arguments.clone() }))?;
                        if !host.bonuses.drain().is_empty() { return Err("Bonus grants require an equipment or automatic-bonus program".into()); }
                        server.item_service().apply_effects(server, server.runtime(), &mut character, host.effects)?;
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
