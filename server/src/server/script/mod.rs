use std::mem;

pub use script_runtime::WasmRuntime;
pub use script_sdk::Value;

pub(crate) mod bonus;
pub mod constant;
mod game_api;
pub(crate) mod game_data;
mod host;
mod interaction;
pub mod item_script_handler;
pub(crate) mod item_dialog;
mod shop;
pub(crate) mod utilities;
pub(crate) mod unit_data;
pub(crate) mod pet_auto_bonus;
pub mod skill;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod generated_npc_smoke;
pub use host::{NpcScriptHost, PlayerInput, ScriptRequest};

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct GlobalVariableEntry {
    pub name: String,
    pub value: Value,
    pub scope: GlobalVariableScope,
    pub index: Option<usize>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum GlobalVariableScope {
    CharTemporary,
    #[allow(dead_code)]
    AccountTemporary,
}

#[derive(Default)]
pub struct ScriptGlobalVariableStore {
    pub variables: Vec<GlobalVariableEntry>,
}

impl ScriptGlobalVariableStore {
    pub fn push(&mut self, variable: GlobalVariableEntry) {
        self.remove_global_by_name_and_scope(&variable.name, &variable.scope, &variable.index);
        self.variables.push(variable);
    }

    pub fn find_global_by_name_and_scope(&self, name: &String, scope: &GlobalVariableScope) -> Option<GlobalVariableEntry> {
        self.variables
            .iter()
            .find(|entry| entry.name == *name && entry.scope == *scope && mem::discriminant(&entry.index) == mem::discriminant(&None))
            .cloned()
    }

    pub fn remove_global_by_name_and_scope(&mut self, name: &String, scope: &GlobalVariableScope, index: &Option<usize>) {
        let position = self.variables.iter().position(|entry| {
            entry.name == *name
                && entry.scope == *scope
                && ((index.is_some() && entry.index.is_some() && index.unwrap() == entry.index.unwrap())
                    || index.is_none() && entry.index.is_none())
        });
        if let Some(position) = position {
            self.variables.remove(position);
        }
    }

    pub fn remove_global_by_name_and_scope_and_index(&mut self, name: &String, scope: &GlobalVariableScope, index: usize) {
        let position = self.variables.iter().position(|entry| {
            entry.name == *name && entry.scope == *scope && entry.index.is_some() && *entry.index.as_ref().unwrap() == index
        });
        if let Some(position) = position {
            self.variables.remove(position);
        }
    }

    pub fn find_global_array_entries(&self, name: &String, scope: GlobalVariableScope) -> Vec<GlobalVariableEntry> {
        self.variables
            .iter()
            .filter(|entry| &entry.name == name && entry.scope == scope && entry.index.is_some())
            .cloned()
            .collect::<Vec<GlobalVariableEntry>>()
    }
}
