use database::model::{CharacterRecord, InventoryRecord};
use database::{abort, next_id, tx_read, tx_required, tx_write};
use sled::transaction::Transactional;

use super::{character_key, write_state, Error, InventoryItemModel, ItemModel, SledRepository};
use crate::server::model::game_systems::CharacterGameSystems;

#[derive(Debug)]
pub struct PetLootReturn {
    pub systems: CharacterGameSystems,
    pub inventory: Vec<InventoryItemModel>,
    pub returned: usize,
}

pub fn return_cargo(repository: &SledRepository, char_id: u32, expected_revision: u64, max_weight: u32, max_slots: usize) -> Result<PetLootReturn, Error> {
    Ok((
        &repository.database.game_systems,
        &repository.database.characters,
        &repository.database.inventories,
        &repository.database.inventory_owners,
        &repository.database.items,
        &repository.database.metadata,
    ).transaction(|(systems, characters, inventories, owners, items, metadata)| {
        let key = char_id.to_be_bytes();
        let mut state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
        if state.revision != expected_revision { return abort("Pet cargo changed before it could be returned"); }
        let character: CharacterRecord = tx_required(characters, &key)?;
        let mut records: Vec<InventoryRecord> = tx_read(inventories, &key)?.unwrap_or_default();
        let max_slots = max_slots.min(character.inventory_slots.max(0) as usize);
        let mut weight = 0u64;
        for record in &records {
            let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
            if record.amount <= 0 || item.weight < 0 { return abort("Invalid inventory item amount or weight"); }
            weight += item.weight as u64 * record.amount as u64;
        }
        let mut returned = 0usize;
        if let Some(cargo) = &mut state.pet_loot {
            if cargo.pending_drop.is_some() { return abort("Pet cargo is reserved for a floor return"); }
            if cargo.items.len() > 30 { return abort("Invalid pet cargo capacity"); }
            let mut remaining = Vec::new();
            for record in &cargo.items {
                let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
                let stackable = item.item_type.is_stackable();
                if record.amount <= 0 || record.equip != 0 || item.weight < 0
                    || (!stackable && record.unique_id != 0 && record.amount != 1) || (stackable && record.unique_id != 0) {
                    return abort("Invalid item in pet cargo");
                }
                if !stackable && record.unique_id != 0 && records.iter().any(|item| item.unique_id == record.unique_id) {
                    return abort("Pet cargo equipment already exists in inventory");
                }
                let matching = records.iter().position(|item| stackable && item.item_id == record.item_id
                    && item.unique_id == 0 && item.equip == 0 && item.refine == record.refine
                    && item.is_identified == record.is_identified && item.is_damaged == record.is_damaged
                    && [item.card0, item.card1, item.card2, item.card3] == [record.card0, record.card1, record.card2, record.card3]);
                let additional_weight = item.weight as u64 * record.amount as u64;
                let additional_slots = if stackable { 1 } else { record.amount as usize };
                let fits_stack = matching.is_none_or(|index| records[index].amount.checked_add(record.amount).is_some());
                if weight.saturating_add(additional_weight) > u64::from(max_weight)
                    || (matching.is_none() && records.len().saturating_add(additional_slots) > max_slots) || !fits_stack {
                    remaining.push(record.clone());
                    continue;
                }
                if let Some(index) = matching {
                    records[index].amount += record.amount;
                } else {
                    for _ in 0..additional_slots {
                        let mut granted = record.clone();
                        granted.id = next_id(metadata, b"inventory_id", 0)?;
                        if !stackable {
                            granted.amount = 1;
                            if granted.unique_id == 0 { granted.unique_id = (i64::from(char_id) << 32) | i64::from(granted.id); }
                        }
                        tx_write(owners, &granted.id.to_be_bytes(), &(char_id as i32))?;
                        records.push(granted);
                    }
                }
                weight += additional_weight;
                returned += 1;
            }
            cargo.items = remaining;
        }
        if returned > 0 {
            if state.pet_loot.as_ref().is_some_and(|cargo| cargo.items.is_empty()) { state.pet_loot = None; }
            write_state(systems, char_id, &mut state)?;
            tx_write(inventories, &key, &records)?;
        }
        let mut inventory = Vec::with_capacity(records.len());
        for record in &records {
            let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
            inventory.push(InventoryItemModel::from_record(record, &item));
        }
        Ok(PetLootReturn { systems: state, inventory, returned })
    })?)
}
