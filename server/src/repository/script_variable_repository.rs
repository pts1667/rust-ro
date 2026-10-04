use std::collections::BTreeMap;

use database::{read, tx_read, tx_write};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::repository::{Error, ScriptVariableRepository, SledRepository};

fn variable_key(scope: u8, owner: u32, name: &str) -> Vec<u8> {
    let mut key = Vec::with_capacity(5 + name.len());
    key.push(scope);
    key.extend_from_slice(&owner.to_be_bytes());
    key.extend_from_slice(name.as_bytes());
    key
}

fn save<T: Serialize + DeserializeOwned + Clone>(
    tree: &sled::Tree,
    scope: u8,
    owner: u32,
    name: &str,
    index: u32,
    value: &T,
) -> Result<(), Error> {
    let key = variable_key(scope, owner, name);
    tree.transaction(|tree| {
        let mut values: BTreeMap<u32, T> = tx_read(tree, &key)?.unwrap_or_default();
        values.insert(index, value.clone());
        tx_write(tree, &key, &values)
    })?;
    Ok(())
}

fn fetch<T: DeserializeOwned>(tree: &sled::Tree, scope: u8, owner: u32, name: &str) -> Result<BTreeMap<u32, T>, Error> {
    Ok(read(tree, &variable_key(scope, owner, name))?.unwrap_or_default())
}

impl ScriptVariableRepository for SledRepository {
    fn script_variable_char_num_save(&self, char_id: u32, key: String, index: u32, value: i32) {
        save(&self.database.numeric_variables, 0, char_id, &key, index, &value).expect("Failed to persist script variable");
    }

    fn script_variable_char_num_fetch_one(&self, char_id: u32, variable_name: String, index: u32) -> i32 {
        fetch::<i32>(&self.database.numeric_variables, 0, char_id, &variable_name)
            .expect("Failed to load script variable")
            .remove(&index)
            .unwrap_or_default()
    }

    fn script_variable_char_num_fetch_all(&self, char_id: u32, variable_name: String) -> Vec<(u32, i32)> {
        fetch::<i32>(&self.database.numeric_variables, 0, char_id, &variable_name)
            .expect("Failed to load script variables")
            .into_iter()
            .collect()
    }

    fn script_variable_char_str_save(&self, char_id: u32, key: String, index: u32, value: String) {
        save(&self.database.string_variables, 0, char_id, &key, index, &value).expect("Failed to persist script variable");
    }

    fn script_variable_char_str_fetch_one(&self, char_id: u32, variable_name: String, index: u32) -> String {
        fetch::<String>(&self.database.string_variables, 0, char_id, &variable_name)
            .expect("Failed to load script variable")
            .remove(&index)
            .unwrap_or_default()
    }

    fn script_variable_char_str_fetch_all(&self, char_id: u32, variable_name: String) -> Vec<(u32, String)> {
        fetch::<String>(&self.database.string_variables, 0, char_id, &variable_name)
            .expect("Failed to load script variables")
            .into_iter()
            .collect()
    }

    fn script_variable_account_num_save(&self, account_id: u32, key: String, index: u32, value: i32) {
        save(&self.database.numeric_variables, 1, account_id, &key, index, &value).expect("Failed to persist script variable");
    }

    fn script_variable_account_num_fetch_one(&self, account_id: u32, variable_name: String, index: u32) -> i32 {
        fetch::<i32>(&self.database.numeric_variables, 1, account_id, &variable_name)
            .expect("Failed to load script variable")
            .remove(&index)
            .unwrap_or_default()
    }

    fn script_variable_account_num_fetch_all(&self, account_id: u32, variable_name: String) -> Vec<(u32, i32)> {
        fetch::<i32>(&self.database.numeric_variables, 1, account_id, &variable_name)
            .expect("Failed to load script variables")
            .into_iter()
            .collect()
    }

    fn script_variable_account_str_save(&self, account_id: u32, key: String, index: u32, value: String) {
        save(&self.database.string_variables, 1, account_id, &key, index, &value).expect("Failed to persist script variable");
    }

    fn script_variable_account_str_fetch_one(&self, account_id: u32, variable_name: String, index: u32) -> String {
        fetch::<String>(&self.database.string_variables, 1, account_id, &variable_name)
            .expect("Failed to load script variable")
            .remove(&index)
            .unwrap_or_default()
    }

    fn script_variable_account_str_fetch_all(&self, account_id: u32, variable_name: String) -> Vec<(u32, String)> {
        fetch::<String>(&self.database.string_variables, 1, account_id, &variable_name)
            .expect("Failed to load script variables")
            .into_iter()
            .collect()
    }

    fn script_variable_server_num_save(&self, varname: String, index: u32, value: i32) {
        save(&self.database.numeric_variables, 2, 0, &varname, index, &value).expect("Failed to persist script variable");
    }

    fn script_variable_server_num_fetch_one(&self, variable_name: String, index: u32) -> i32 {
        fetch::<i32>(&self.database.numeric_variables, 2, 0, &variable_name)
            .expect("Failed to load script variable")
            .remove(&index)
            .unwrap_or_default()
    }

    fn script_variable_server_num_fetch_all(&self, variable_name: String) -> Vec<(u32, i32)> {
        fetch::<i32>(&self.database.numeric_variables, 2, 0, &variable_name)
            .expect("Failed to load script variables")
            .into_iter()
            .collect()
    }

    fn script_variable_server_str_save(&self, varname: String, index: u32, value: String) {
        save(&self.database.string_variables, 2, 0, &varname, index, &value).expect("Failed to persist script variable");
    }

    fn script_variable_server_str_fetch_one(&self, variable_name: String, index: u32) -> String {
        fetch::<String>(&self.database.string_variables, 2, 0, &variable_name)
            .expect("Failed to load script variable")
            .remove(&index)
            .unwrap_or_default()
    }

    fn script_variable_server_str_fetch_all(&self, variable_name: String) -> Vec<(u32, String)> {
        fetch::<String>(&self.database.string_variables, 2, 0, &variable_name)
            .expect("Failed to load script variables")
            .into_iter()
            .collect()
    }
}
