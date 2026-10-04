use std::collections::BTreeMap;

use database::{read, tx_read, tx_write};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::repository::{Error, ScriptVariableRepository, SledRepository};

pub(crate) fn variable_key(scope: u8, owner: u32, name: &str) -> Vec<u8> {
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
    fn script_variables_increment_batch(&self, char_id: u32, account_id: u32, variables: &[script_sdk::Variable]) -> Result<Vec<i32>, Error> {
        use script_sdk::VariableScope;
        Ok(self.database.numeric_variables.transaction(|numbers| {
            let mut result = Vec::with_capacity(variables.len());
            for variable in variables {
                let (scope, owner) = match variable.scope {
                    VariableScope::Character => (0, char_id),
                    VariableScope::Account => (1, account_id),
                    VariableScope::Server => (2, 0),
                    _ => return database::abort("Counter scope is not persistent"),
                };
                if variable.name.is_empty() || variable.name.len() > 128 || variable.name.ends_with('$') { return database::abort("Counter name is invalid"); }
                let amount = variable.value.number_value().map_err(|error| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput(error)))?;
                let key = variable_key(scope, owner, &variable.name);
                let mut values: BTreeMap<u32, i32> = tx_read(numbers, &key)?.unwrap_or_default();
                let next = values.get(&variable.index).copied().unwrap_or_default().checked_add(amount)
                    .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Counter overflow".into())))?;
                values.insert(variable.index, next);
                tx_write(numbers, &key, &values)?;
                result.push(next);
            }
            Ok(result)
        })?)
    }

    fn script_variables_save_batch(&self, char_id: u32, account_id: u32, variables: &[script_sdk::Variable]) -> Result<(), Error> {
        use script_sdk::{Value, VariableScope};
        use sled::transaction::Transactional;
        (&self.database.numeric_variables, &self.database.string_variables).transaction(|(numbers, strings)| {
            for variable in variables {
                let (scope, owner) = match variable.scope {
                    VariableScope::Character => (0, char_id),
                    VariableScope::Account => (1, account_id),
                    VariableScope::Server => (2, 0),
                    _ => continue,
                };
                if variable.name.is_empty() || variable.name.len() > 128 {
                    return database::abort("Invalid script variable name");
                }
                let key = variable_key(scope, owner, &variable.name);
                match &variable.value {
                    Value::Number(value) => {
                        let mut values: BTreeMap<u32, i32> = tx_read(numbers, &key)?.unwrap_or_default();
                        values.insert(variable.index, *value);
                        tx_write(numbers, &key, &values)?;
                    }
                    Value::String(value) if value.len() <= script_sdk::MAX_MESSAGE_BYTES => {
                        let mut values: BTreeMap<u32, String> = tx_read(strings, &key)?.unwrap_or_default();
                        values.insert(variable.index, value.clone());
                        tx_write(strings, &key, &values)?;
                    }
                    _ => return database::abort("Script variables must contain numbers or strings"),
                }
            }
            Ok(())
        })?;
        Ok(())
    }

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

#[cfg(test)]
mod counter_tests {
    use super::*;
    use script_sdk::{Variable, VariableScope};
    use std::sync::Arc;

    #[test]
    fn concurrent_script_counters_preserve_every_increment_across_scopes() {
        let repository = Arc::new(SledRepository::temporary().unwrap());
        let workers: Vec<_> = (0..8).map(|_| {
            let repository = repository.clone();
            std::thread::spawn(move || {
                for _ in 0..100 {
                    repository.script_variables_increment_batch(150_000, 2_000_000, &[
                        Variable { scope: VariableScope::Character, name: "Kills".into(), index: 0, value: 1.into() },
                        Variable { scope: VariableScope::Account, name: "Kills".into(), index: 0, value: 2.into() },
                        Variable { scope: VariableScope::Server, name: "Kills".into(), index: 1, value: 3.into() },
                    ]).unwrap();
                }
            })
        }).collect();
        for worker in workers { worker.join().unwrap(); }
        assert_eq!(repository.script_variable_char_num_fetch_one(150_000, "Kills".into(), 0), 800);
        assert_eq!(repository.script_variable_account_num_fetch_one(2_000_000, "Kills".into(), 0), 1600);
        assert_eq!(repository.script_variable_server_num_fetch_one("Kills".into(), 1), 2400);
    }

    #[test]
    fn overflowing_or_invalid_counter_rolls_back_the_entire_batch() {
        let repository = SledRepository::temporary().unwrap();
        repository.script_variable_char_num_save(150_000, "Full".into(), 0, i32::MAX);
        let variables = vec![Variable { scope: VariableScope::Account, name: "First".into(), index: 0, value: 1.into() },
            Variable { scope: VariableScope::Character, name: "Full".into(), index: 0, value: 1.into() }];
        assert!(repository.script_variables_increment_batch(150_000, 2_000_000, &variables).is_err());
        assert_eq!(repository.script_variable_account_num_fetch_one(2_000_000, "First".into(), 0), 0);
        assert_eq!(repository.script_variable_char_num_fetch_one(150_000, "Full".into(), 0), i32::MAX);
        let mut invalid = variables; invalid[1].name = "Text$".into();
        assert!(repository.script_variables_increment_batch(150_000, 2_000_000, &invalid).is_err());
        assert_eq!(repository.script_variable_account_num_fetch_one(2_000_000, "First".into(), 0), 0);
    }
}
