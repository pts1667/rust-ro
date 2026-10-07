//! Whole-array access to stored script variables (`@temp[]`, `$global[]`, `#account[]`, `.npc[]`), for `getarraysize`, `deletearray`, `copyarray` and `implode`.

use std::collections::BTreeMap;

use script_sdk::{Function, Reply, Value, Variable, VariableScope};

use super::ScriptRequest;
use crate::server::Server;
use crate::server::service::script_service::ScriptService;

impl ScriptService {
    pub(super) fn array_call(&self, server: &Server, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let name = arguments.first().ok_or("Missing array name")?.text();
        let (scope, stripped) = crate::server::service::item_effect_service::script_variable_name(&name);
        Self::validate_variable_scope(context, scope)?;
        let stored = self.array_entries(server, context, scope, &stripped);
        let string = stripped.ends_with('$');
        let default = || if string { Value::String(String::new()) } else { Value::default() };
        let is_set = |value: &Value| if string { !value.text().is_empty() } else { value.truthy() };
        let length = stored.iter().filter(|(_, value)| is_set(value)).map(|(index, _)| *index as usize + 1).max().unwrap_or(0);
        if function == Function::ArrayGet {
            return Ok(Value::Array((0..length as u32).map(|index| stored.get(&index).cloned().unwrap_or_else(default)).collect()));
        }
        let Some(Value::Array(values)) = arguments.get(1) else { return Err("Array contents are required".into()) };
        if values.len() > MAX_ARRAY_LENGTH {
            return Err("Script array is too long".into());
        }
        let variables = (0..length.max(values.len()) as u32)
            .map(|index| Variable { scope, name: stripped.clone(), index, value: values.get(index as usize).cloned().unwrap_or_else(default) })
            .collect();
        self.write_variables(server, context, variables)
    }

    fn array_entries(&self, server: &Server, context: &ScriptRequest, scope: VariableScope, name: &str) -> BTreeMap<u32, Value> {
        let repository = &server.repository;
        let string = name.ends_with('$');
        let name_owned = name.to_string();
        match (scope, string) {
            (VariableScope::Character, false) => numbers(repository.script_variable_char_num_fetch_all(context.char_id, name_owned)),
            (VariableScope::Character, true) => strings(repository.script_variable_char_str_fetch_all(context.char_id, name_owned)),
            (VariableScope::Account, false) => numbers(repository.script_variable_account_num_fetch_all(context.account_id, name_owned)),
            (VariableScope::Account, true) => strings(repository.script_variable_account_str_fetch_all(context.account_id, name_owned)),
            (VariableScope::Server, false) => numbers(repository.script_variable_server_num_fetch_all(name_owned)),
            (VariableScope::Server, true) => strings(repository.script_variable_server_str_fetch_all(name_owned)),
            _ => {
                let wanted = Self::temporary_key(context, scope, name, 0);
                self.npc_variables
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|(key, _)| key.0 == wanted.0 && key.1 == wanted.1 && key.2 == wanted.2 && key.3 == wanted.3)
                    .map(|(key, value)| (key.4, value.clone()))
                    .collect()
            }
        }
    }
}

/// `MAX_ARRAYSIZE` of rathena is 2^31; scripts here keep arrays small enough to round-trip through one request.
const MAX_ARRAY_LENGTH: usize = 4096;

fn numbers(entries: Vec<(u32, i32)>) -> BTreeMap<u32, Value> {
    entries.into_iter().map(|(index, value)| (index, Value::Number(value))).collect()
}

fn strings(entries: Vec<(u32, String)>) -> BTreeMap<u32, Value> {
    entries.into_iter().map(|(index, value)| (index, Value::String(value))).collect()
}
