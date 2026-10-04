use std::collections::HashSet;
use std::fs;

use async_trait::async_trait;
use base64::Engine;
use base64::engine::general_purpose;
use configuration::configuration::DatabaseConfig;
use database::model::SeedData;
use database::{abort, required, tx_read, tx_required, tx_write};
use models::enums::EnumWithStringValue;
use sled::transaction::Transactional;

use crate::repository::model::item_model::{GetItemModel, ItemBuySellModel, ItemModel, ItemModels};
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::repository::{Error, ItemRepository, SledRepository};
use crate::server::script::Value;

impl SledRepository {
    pub fn seed_assets(&self, configuration: &DatabaseConfig) -> Result<(), Error> {
        if !database::read::<bool>(&self.database.metadata, b"catalog_seeded")?.unwrap_or(false) {
            let items: Vec<ItemModel> = serde_json::from_slice::<ItemModels>(&fs::read(&configuration.items_path)?)?.into();
            let mut mobs: Vec<MobModel> = serde_json::from_slice::<MobModels>(&fs::read(&configuration.mobs_path)?)?.into();
            if items.is_empty() || mobs.is_empty() {
                return Err(Error::new("Item and monster seed catalogs must be nonempty".into()));
            }
            let mut ids = HashSet::new();
            let mut names = std::collections::HashMap::new();
            for item in &items {
                if item.id <= 0 || !ids.insert(item.id) || names.insert(item.name_aegis.clone(), item.id).is_some() {
                    return Err(Error::new("Duplicate or invalid item in seed catalog".into()));
                }
            }
            ids.clear();
            for mob in &mut mobs {
                if mob.id <= 0 || !ids.insert(mob.id) {
                    return Err(Error::new("Duplicate or invalid monster in seed catalog".into()));
                }
                for drop in mob.drops.iter_mut().chain(mob.mvp_drops.iter_mut()) {
                    drop.item_id = *names
                        .get(&drop.item_name)
                        .ok_or_else(|| Error::new(format!("Unknown monster drop {}", drop.item_name)))?;
                }
            }
            let item_records = items
                .iter()
                .map(|item| Ok((item.id.to_be_bytes(), serde_json::to_vec(item)?)))
                .collect::<Result<Vec<_>, Error>>()?;
            let mob_records = mobs
                .iter()
                .map(|mob| Ok((mob.id.to_be_bytes(), serde_json::to_vec(mob)?)))
                .collect::<Result<Vec<_>, Error>>()?;
            (
                &self.database.items,
                &self.database.mobs,
                &self.database.item_names,
                &self.database.metadata,
            )
                .transaction(|(items, mobs, item_names, metadata)| {
                    if tx_read::<bool>(metadata, b"catalog_seeded")?.unwrap_or(false) {
                        return Ok(());
                    }
                    for (key, value) in &item_records {
                        items.insert(&key[..], value.clone())?;
                    }
                    for (key, value) in &mob_records {
                        mobs.insert(&key[..], value.clone())?;
                    }
                    for (name, id) in &names {
                        tx_write(item_names, name.as_bytes(), id)?;
                    }
                    tx_write(metadata, b"catalog_seeded", &true)
                })?;
        }
        if !database::read::<bool>(&self.database.metadata, b"accounts_seeded")?.unwrap_or(false) {
            if let Some(path) = &configuration.seed_path {
                let seed: SeedData = serde_json::from_slice(&fs::read(path)?)?;
                self.database.seed(&seed, false)?;
            }
            self.database
                .metadata
                .transaction(|tree| tx_write(tree, b"accounts_seeded", &true))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ItemRepository for SledRepository {
    async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> {
        let mut ids = ids;
        ids.sort_unstable();
        ids.dedup();
        let mut result = Vec::new();
        for id in ids {
            if let Some(item) = database::read::<ItemModel>(&self.database.items, &id.to_be_bytes())? {
                result.push(ItemBuySellModel {
                    id: Some(item.id),
                    item_type: item.item_type.as_str().into(),
                    price_buy: item.price_buy,
                    price_sell: item.price_sell,
                    stack_amount: item.stack_amount.and_then(|amount| i16::try_from(amount).ok()),
                    weight: Some(item.weight),
                    name_english: Some(item.name_english),
                });
            }
        }
        Ok(result)
    }

    async fn get_items(&self, ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error> {
        let mut ids = HashSet::new();
        let mut patterns = Vec::new();
        for value in ids_or_names {
            match value {
                Value::Number(id) => {
                    ids.insert(id);
                }
                Value::String(name) => {
                    let mut pattern = String::from("(?i)^");
                    let mut escaped = false;
                    for c in name.chars() {
                        if escaped {
                            pattern.push_str(&regex_lite::escape(&c.to_string()));
                            escaped = false;
                        } else {
                            match c {
                                '\\' => escaped = true,
                                '%' => pattern.push_str(".*"),
                                '_' => pattern.push('.'),
                                _ => pattern.push_str(&regex_lite::escape(&c.to_string())),
                            }
                        }
                    }
                    if escaped {
                        return Err(Error::new("Item name pattern ends with an escape".into()));
                    }
                    pattern.push('$');
                    patterns.push(regex_lite::Regex::new(&pattern).map_err(|error| Error::new(error.to_string()))?);
                }
            }
        }
        if !patterns.is_empty() {
            for entry in self.database.item_names.iter() {
                let (key, value) = entry?;
                let name = std::str::from_utf8(&key).map_err(|error| Error::new(error.to_string()))?;
                if patterns.iter().any(|pattern| pattern.is_match(name)) {
                    ids.insert(serde_json::from_slice::<i32>(&value)?);
                }
            }
        }
        let mut ids = ids.into_iter().collect::<Vec<_>>();
        ids.sort_unstable();
        let mut result = Vec::new();
        for id in ids {
            if let Some(item) = database::read::<ItemModel>(&self.database.items, &id.to_be_bytes())? {
                result.push(GetItemModel {
                    id: item.id,
                    item_type: item.item_type.as_str().into(),
                    amount: 0,
                    weight: item.weight,
                    name_english: item.name_english,
                    name_aegis: item.name_aegis,
                });
            }
        }
        Ok(result)
    }

    async fn get_item_script(&self, id: i32) -> Result<String, Error> {
        required::<ItemModel>(&self.database.items, &id.to_be_bytes())?
            .script
            .ok_or(Error::NotFound)
    }

    async fn get_weight(&self, ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        for id in ids {
            if seen.insert(id) {
                if let Some(item) = database::read::<ItemModel>(&self.database.items, &id.to_be_bytes())? {
                    result.push((id, item.weight));
                }
            }
        }
        Ok(result)
    }

    async fn get_all_items(&self) -> Result<Vec<ItemModel>, Error> {
        let mut result = Vec::new();
        for entry in self.database.items.iter() {
            let (_, bytes) = entry?;
            let item: ItemModel = serde_json::from_slice(&bytes)?;
            result.push(item);
        }
        Ok(result)
    }

    async fn update_script_compilation(&self, to_update: Vec<(i32, Vec<u8>, u128)>) -> Result<(), Error> {
        self.database.items.transaction(|tree| {
            for (id, bytes, hash) in &to_update {
                let mut item: ItemModel = tx_required(tree, &id.to_be_bytes())?;
                if item.script.as_ref().map(|script| fastmurmur3::hash(script.as_bytes())) != Some(*hash) {
                    return abort("Compiled item script does not match its source");
                }
                item.script_compilation = Some(general_purpose::STANDARD.encode(bytes));
                item.script_compilation_hash = Some(*hash);
                tx_write(tree, &id.to_be_bytes(), &item)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}
