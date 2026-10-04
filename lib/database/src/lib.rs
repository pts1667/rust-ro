pub mod error;
pub mod model;

use std::collections::HashSet;
use std::path::Path;

pub use error::DatabaseError;
use model::{AccountRecord, CharacterRecord, InventoryRecord, SeedData};
use serde::Serialize;
use serde::de::DeserializeOwned;
use sled::transaction::{ConflictableTransactionError, ConflictableTransactionResult, Transactional, TransactionalTree};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Clone)]
pub struct Database {
    pub metadata: sled::Tree,
    pub accounts: sled::Tree,
    pub account_names: sled::Tree,
    pub characters: sled::Tree,
    pub character_names: sled::Tree,
    pub character_slots: sled::Tree,
    pub inventories: sled::Tree,
    pub inventory_owners: sled::Tree,
    pub skills: sled::Tree,
    pub bonuses: sled::Tree,
    pub hotkeys: sled::Tree,
    pub numeric_variables: sled::Tree,
    pub string_variables: sled::Tree,
    pub items: sled::Tree,
    pub item_names: sled::Tree,
    pub mobs: sled::Tree,
    _db: sled::Db,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DatabaseError> {
        Self::from_db(sled::open(path)?)
    }

    pub fn temporary() -> Result<Self, DatabaseError> {
        Self::from_db(sled::Config::new().temporary(true).open()?)
    }

    fn from_db(db: sled::Db) -> Result<Self, DatabaseError> {
        let metadata = db.open_tree("metadata")?;
        metadata.transaction(|tree| {
            if let Some(version) = tx_read::<u32>(tree, b"schema_version")? {
                if version != SCHEMA_VERSION {
                    return abort(format!("Unsupported database schema version {version}"));
                }
            } else {
                tx_write(tree, b"schema_version", &SCHEMA_VERSION)?;
            }
            Ok(())
        })?;
        Ok(Self {
            metadata,
            accounts: db.open_tree("accounts")?,
            account_names: db.open_tree("account_names")?,
            characters: db.open_tree("characters")?,
            character_names: db.open_tree("character_names")?,
            character_slots: db.open_tree("character_slots")?,
            inventories: db.open_tree("inventories")?,
            inventory_owners: db.open_tree("inventory_owners")?,
            skills: db.open_tree("skills")?,
            bonuses: db.open_tree("bonuses")?,
            hotkeys: db.open_tree("hotkeys")?,
            numeric_variables: db.open_tree("numeric_variables")?,
            string_variables: db.open_tree("string_variables")?,
            items: db.open_tree("items")?,
            item_names: db.open_tree("item_names")?,
            mobs: db.open_tree("mobs")?,
            _db: db,
        })
    }

    pub fn create_account(&self, username: String, password: String) -> Result<u32, DatabaseError> {
        if username.is_empty() || password.is_empty() {
            return Err(DatabaseError::new("Username and password must be nonempty".into()));
        }
        Ok(
            (&self.accounts, &self.account_names, &self.metadata).transaction(|(accounts, names, metadata)| {
                if names.get(username.as_bytes())?.is_some() {
                    return conflict("Username already exists");
                }
                let id = next_id(metadata, b"account_id", 1_999_999)? as u32;
                let account = AccountRecord {
                    account_id: id,
                    username: username.clone(),
                    password: password.clone(),
                };
                tx_write(accounts, &id.to_be_bytes(), &account)?;
                tx_write(names, username.as_bytes(), &id)?;
                Ok(id)
            })?,
        )
    }

    pub fn insert_character(&self, character: &CharacterRecord) -> Result<i32, DatabaseError> {
        validate_character(character)?;
        Ok((
            &self.characters,
            &self.character_names,
            &self.character_slots,
            &self.accounts,
            &self.metadata,
        )
            .transaction(|(characters, names, slots, accounts, metadata)| {
                tx_required::<AccountRecord>(accounts, &(character.account_id as u32).to_be_bytes())?;
                let slot = character_slot_key(character.account_id, character.char_num);
                if names.get(character.name.as_bytes())?.is_some() || slots.get(&slot)?.is_some() {
                    return conflict("Character name or account slot already exists");
                }
                let mut character = character.clone();
                if character.char_id == 0 {
                    character.char_id = next_id(metadata, b"char_id", 149_999)?;
                } else {
                    advance_id(metadata, b"char_id", character.char_id)?;
                }
                if characters.get(character.char_id.to_be_bytes())?.is_some() {
                    return conflict("Character ID already exists");
                }
                tx_write(characters, &character.char_id.to_be_bytes(), &character)?;
                tx_write(names, character.name.as_bytes(), &character.char_id)?;
                tx_write(slots, &slot, &character.char_id)?;
                Ok(character.char_id)
            })?)
    }

    pub fn seed(&self, seed: &SeedData, replace: bool) -> Result<(), DatabaseError> {
        for character in &seed.characters {
            validate_character(character)?;
            if character.char_id <= 0 {
                return Err(DatabaseError::new("Seed characters must have a positive ID".into()));
            }
        }
        let mut account_ids = HashSet::new();
        let mut character_ids = HashSet::new();
        for account in &seed.accounts {
            if account.account_id == 0
                || account.account_id > i32::MAX as u32
                || account.username.is_empty()
                || account.password.is_empty()
                || !account_ids.insert(account.account_id)
            {
                return Err(DatabaseError::new("Invalid or duplicate seed account".into()));
            }
        }
        for character in &seed.characters {
            if !character_ids.insert(character.char_id) {
                return Err(DatabaseError::new("Duplicate seed character ID".into()));
            }
        }
        let mut inventory_characters = HashSet::new();
        let mut skill_characters = HashSet::new();
        for inventory in &seed.inventories {
            if !inventory_characters.insert(inventory.char_id) {
                return Err(DatabaseError::new("Duplicate seed inventory".into()));
            }
        }
        for skills in &seed.skills {
            if !skill_characters.insert(skills.char_id) {
                return Err(DatabaseError::new("Duplicate seed skills".into()));
            }
        }
        (
            &self.accounts,
            &self.account_names,
            &self.characters,
            &self.character_names,
            &self.character_slots,
            &self.inventories,
            &self.inventory_owners,
            &self.skills,
            &self.metadata,
        )
            .transaction(
                |(accounts, account_names, characters, names, slots, inventories, owners, skills, metadata)| {
                    for account in &seed.accounts {
                        let key = account.account_id.to_be_bytes();
                        if let Some(existing) = tx_read::<AccountRecord>(accounts, &key)? {
                            if !replace {
                                continue;
                            }
                            account_names.remove(existing.username.as_bytes())?;
                        }
                        if let Some(owner) = tx_read::<u32>(account_names, account.username.as_bytes())? {
                            if owner != account.account_id {
                                return conflict("Seed username belongs to another account");
                            }
                        }
                        tx_write(accounts, &key, account)?;
                        tx_write(account_names, account.username.as_bytes(), &account.account_id)?;
                        advance_id(metadata, b"account_id", account.account_id as i32)?;
                    }
                    let mut skipped = HashSet::new();
                    for character in &seed.characters {
                        let key = character.char_id.to_be_bytes();
                        if let Some(existing) = tx_read::<CharacterRecord>(characters, &key)? {
                            if !replace {
                                skipped.insert(character.char_id);
                                continue;
                            }
                            names.remove(existing.name.as_bytes())?;
                            slots.remove(character_slot_key(existing.account_id, existing.char_num).to_vec())?;
                        }
                        tx_required::<AccountRecord>(accounts, &(character.account_id as u32).to_be_bytes())?;
                        let slot = character_slot_key(character.account_id, character.char_num);
                        if names.get(character.name.as_bytes())?.is_some() || slots.get(&slot)?.is_some() {
                            return conflict("Seed character name or account slot already exists");
                        }
                        tx_write(characters, &key, character)?;
                        tx_write(names, character.name.as_bytes(), &character.char_id)?;
                        tx_write(slots, &slot, &character.char_id)?;
                        advance_id(metadata, b"char_id", character.char_id)?;
                    }
                    for inventory in &seed.inventories {
                        let key = inventory.char_id.to_be_bytes();
                        if skipped.contains(&inventory.char_id) || (!replace && inventories.get(&key)?.is_some()) {
                            continue;
                        }
                        tx_required::<CharacterRecord>(characters, &key)?;
                        for old in tx_read::<Vec<InventoryRecord>>(inventories, &key)?.unwrap_or_default() {
                            owners.remove(old.id.to_be_bytes().to_vec())?;
                        }
                        let mut records = inventory.items.clone();
                        let mut identities = HashSet::new();
                        for item in &mut records {
                            if item.item_id <= 0 || item.amount <= 0 || !identities.insert((item.item_id, item.unique_id)) {
                                return abort("Invalid or duplicate seed inventory item");
                            }
                            if item.id == 0 {
                                item.id = next_id(metadata, b"inventory_id", 0)?;
                            } else if item.id > 0 {
                                advance_id(metadata, b"inventory_id", item.id)?;
                            } else {
                                return abort("Inventory IDs must be positive");
                            }
                            if owners.get(item.id.to_be_bytes())?.is_some() {
                                return conflict("Inventory ID already exists");
                            }
                            tx_write(owners, &item.id.to_be_bytes(), &inventory.char_id)?;
                        }
                        tx_write(inventories, &key, &records)?;
                    }
                    for character_skills in &seed.skills {
                        let key = character_skills.char_id.to_be_bytes();
                        if skipped.contains(&character_skills.char_id) || (!replace && skills.get(&key)?.is_some()) {
                            continue;
                        }
                        tx_required::<CharacterRecord>(characters, &key)?;
                        tx_write(skills, &key, &character_skills.skills)?;
                    }
                    Ok(())
                },
            )?;
        Ok(())
    }
}

pub fn read<T: DeserializeOwned>(tree: &sled::Tree, key: &[u8]) -> Result<Option<T>, DatabaseError> {
    tree.get(key)?
        .map(|bytes| serde_json::from_slice(&bytes).map_err(DatabaseError::from))
        .transpose()
}

pub fn required<T: DeserializeOwned>(tree: &sled::Tree, key: &[u8]) -> Result<T, DatabaseError> {
    read(tree, key)?.ok_or(DatabaseError::NotFound)
}

pub fn tx_read<T: DeserializeOwned>(tree: &TransactionalTree, key: &[u8]) -> ConflictableTransactionResult<Option<T>, DatabaseError> {
    tree.get(key)?
        .map(|bytes| serde_json::from_slice(&bytes).map_err(|error| ConflictableTransactionError::Abort(error.into())))
        .transpose()
}

pub fn tx_required<T: DeserializeOwned>(tree: &TransactionalTree, key: &[u8]) -> ConflictableTransactionResult<T, DatabaseError> {
    tx_read(tree, key)?.ok_or(ConflictableTransactionError::Abort(DatabaseError::NotFound))
}

pub fn tx_write<T: Serialize>(tree: &TransactionalTree, key: &[u8], value: &T) -> ConflictableTransactionResult<(), DatabaseError> {
    let bytes = serde_json::to_vec(value).map_err(|error| ConflictableTransactionError::Abort(error.into()))?;
    tree.insert(key, bytes)?;
    Ok(())
}

pub fn abort<T>(message: impl Into<String>) -> ConflictableTransactionResult<T, DatabaseError> {
    Err(ConflictableTransactionError::Abort(DatabaseError::new(message.into())))
}

pub fn conflict<T>(message: impl Into<String>) -> ConflictableTransactionResult<T, DatabaseError> {
    Err(ConflictableTransactionError::Abort(DatabaseError::Conflict(message.into())))
}

pub fn next_id(tree: &TransactionalTree, key: &[u8], initial: i32) -> ConflictableTransactionResult<i32, DatabaseError> {
    let current = tx_read::<i32>(tree, key)?.unwrap_or(initial).max(initial);
    let next = current
        .checked_add(1)
        .ok_or_else(|| ConflictableTransactionError::Abort(DatabaseError::new("Database ID space exhausted".into())))?;
    tx_write(tree, key, &next)?;
    Ok(next)
}

pub fn advance_id(tree: &TransactionalTree, key: &[u8], value: i32) -> ConflictableTransactionResult<(), DatabaseError> {
    let current = tx_read::<i32>(tree, key)?.unwrap_or(0);
    tx_write(tree, key, &current.max(value))
}

pub fn character_slot_key(account_id: i32, slot: i16) -> [u8; 6] {
    let mut key = [0; 6];
    key[..4].copy_from_slice(&account_id.to_be_bytes());
    key[4..].copy_from_slice(&slot.to_be_bytes());
    key
}

fn validate_character(character: &CharacterRecord) -> Result<(), DatabaseError> {
    if character.char_id < 0
        || character.account_id <= 0
        || character.char_num < 0
        || character.char_num > u8::MAX as i16
        || character.name.is_empty()
        || character.name.chars().count() > 24
        || character.zeny < 0
        || character.inventory_slots <= 0
    {
        return Err(DatabaseError::new("Invalid character record".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
