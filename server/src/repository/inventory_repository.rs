use std::collections::BTreeMap;

use async_trait::async_trait;
use database::model::InventoryRecord;
use database::{abort, next_id, tx_read, tx_required, tx_write};
use models::enums::item::ItemType;
use sled::transaction::Transactional;

use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::repository::{Error, InventoryRepository, SledRepository};
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};

#[cfg(test)]
mod damaged_equipment_tests {
    use super::*;
    use database::model::{AccountRecord,CharacterInventory,CharacterRecord,SeedData};

    #[test]
    fn equipment_breaking_validates_owner_and_identity_then_commits_damage_and_takeoff_together() {
        let repository=SledRepository::temporary().unwrap();
        let record=InventoryRecord {id:10,item_id:1201,amount:1,equip:2,unique_id:400,card0:255,card1:15*256+3,card2:1,card3:2,..Default::default()};
        repository.database.seed(&SeedData {accounts:vec![AccountRecord {account_id:2000000,username:"Player".into(),password:"password".into(), ..Default::default() }],
            characters:vec![CharacterRecord {char_id:150000,account_id:2000000,name:"Player".into(),inventory_slots:100,..Default::default()}],
            inventories:vec![CharacterInventory {char_id:150000,items:vec![record.clone()]}],..Default::default()},false).unwrap();
        let item:ItemModel=serde_json::from_value(serde_json::json!({"id":1201,"name_aegis":"Knife","name_english":"Knife","weight":10,"item_type":"Weapon","job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0})).unwrap();
        let model=InventoryItemModel::from_record(&record,&item);
        let before=repository.database.inventories.get(150000_i32.to_be_bytes()).unwrap().unwrap();
        assert!(futures::executor::block_on(repository.character_set_item_damaged(150001,model.clone())).is_err());
        let mut stale=model.clone(); stale.unique_id+=1;
        assert!(futures::executor::block_on(repository.character_set_item_damaged(150000,stale)).is_err());
        assert_eq!(repository.database.inventories.get(150000_i32.to_be_bytes()).unwrap().unwrap(),before);
        futures::executor::block_on(repository.character_set_item_damaged(150000,model)).unwrap();
        let saved:Vec<InventoryRecord>=database::required(&repository.database.inventories,&150000_i32.to_be_bytes()).unwrap();
        assert_eq!(saved.len(),1); assert!(saved[0].is_damaged); assert_eq!(saved[0].equip,0);
        assert_eq!((saved[0].unique_id,saved[0].card0,saved[0].card1,saved[0].card2,saved[0].card3),(400,255,15*256+3,1,2));
        assert_eq!(database::required::<i32>(&repository.database.inventory_owners,&10_i32.to_be_bytes()).unwrap(),150000);
    }
}

#[async_trait]
impl InventoryRepository for SledRepository {
    async fn character_set_item_damaged(&self, char_id: u32, item: InventoryItemModel) -> Result<(), Error> {
        (&self.database.inventories, &self.database.inventory_owners).transaction(|(inventories, owners)| {
            let owner: i32 = tx_required(owners, &item.id.to_be_bytes())?;
            if owner != char_id as i32 { return abort("Equipment belongs to another character"); }
            let mut records: Vec<InventoryRecord> = tx_required(inventories, &owner.to_be_bytes())?;
            let record = records.iter_mut().find(|record| record.id == item.id && record.item_id == item.item_id && record.unique_id == item.unique_id)
                .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
            record.is_damaged = true;
            record.equip = 0;
            tx_write(inventories, &owner.to_be_bytes(), &records)?;
            Ok(())
        })?;
        Ok(())
    }
    async fn character_identify_item(&self, char_id: u32, item: InventoryItemModel) -> Result<(), Error> {
        (&self.database.inventories, &self.database.inventory_owners).transaction(|(inventories, owners)| {
            let owner: i32 = tx_required(owners, &item.id.to_be_bytes())?;
            if owner != char_id as i32 { return abort("Item identification belongs to another character"); }
            let mut records: Vec<InventoryRecord> = tx_required(inventories, &owner.to_be_bytes())?;
            let record = records.iter_mut().find(|record| record.id == item.id && record.item_id == item.item_id && record.unique_id == item.unique_id)
                .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
            if record.is_identified { return abort("Item is already identified"); }
            record.is_identified = true;
            tx_write(inventories, &owner.to_be_bytes(), &records)?;
            Ok(())
        })?;
        Ok(())
    }
    async fn character_inventory_update_add(&self, updates: &[InventoryItemUpdate], buy: bool) -> Result<Vec<InventoryItemModel>, Error> {
        Ok((
            &self.database.inventories,
            &self.database.characters,
            &self.database.items,
            &self.database.metadata,
            &self.database.inventory_owners,
        )
            .transaction(|(inventories, characters, items, metadata, owners)| {
                let mut groups = BTreeMap::<i32, Vec<InventoryRecord>>::new();
                let mut chars = BTreeMap::<i32, CharSelectModel>::new();
                let mut added = Vec::new();
                for update in updates {
                    if update.amount <= 0 {
                        return abort("Added inventory amount must be positive");
                    }
                    let key = update.char_id.to_be_bytes();
                    if !groups.contains_key(&update.char_id) {
                        groups.insert(update.char_id, tx_read(inventories, &key)?.unwrap_or_default());
                        chars.insert(update.char_id, tx_required(characters, &key)?);
                    }
                    let item: ItemModel = tx_required(items, &update.item_id.to_be_bytes())?;
                    if update.stackable != item.item_type.is_stackable() {
                        return abort("Inventory stacking mode does not match item type");
                    }
                    let records = groups.get_mut(&update.char_id).unwrap();
                    let unique_id = if update.stackable { 0 } else { update.unique_id };
                    let position = records
                        .iter()
                        .position(|record| record.item_id == update.item_id && record.unique_id == unique_id);
                    let index = if let Some(position) = position {
                        if !update.stackable {
                            return abort("Equipment unique ID already exists");
                        }
                        records[position].amount = records[position].amount.checked_add(update.amount).ok_or_else(|| {
                            sled::transaction::ConflictableTransactionError::Abort(Error::new("Inventory amount is out of bounds".into()))
                        })?;
                        position
                    } else {
                        let id = next_id(metadata, b"inventory_id", 0)?;
                        tx_write(owners, &id.to_be_bytes(), &update.char_id)?;
                        records.push(InventoryRecord {
                            id,
                            item_id: update.item_id,
                            unique_id,
                            amount: update.amount,
                            is_identified: update.identified,
                            refine: update.refine,
                            is_damaged: update.damaged,
                            card0: update.cards[0],
                            card1: update.cards[1],
                            card2: update.cards[2],
                            card3: update.cards[3],
                            ..InventoryRecord::default()
                        });
                        records.len() - 1
                    };
                    let character = chars.get_mut(&update.char_id).unwrap();
                    if records.len() > character.inventory_slots as usize {
                        return abort("Character inventory is full");
                    }
                    if buy {
                        let cost = i64::from(update.price.or(item.price_buy).unwrap_or(0)) * i64::from(update.amount);
                        if cost < 0 || cost > i64::from(character.zeny) {
                            return abort("Not enough zeny");
                        }
                        character.zeny -= cost as i32;
                    }
                    let mut added_item = InventoryItemModel::from_record(&records[index], &item);
                    added_item.amount = update.amount;
                    added.push(added_item);
                }
                for (id, records) in &groups {
                    tx_write(inventories, &id.to_be_bytes(), records)?;
                }
                if buy {
                    for (id, character) in &chars {
                        tx_write(characters, &id.to_be_bytes(), character)?;
                    }
                }
                Ok(added)
            })?)
    }

    async fn character_inventory_update_remove(
        &self,
        updates: &Vec<(InventoryItemModel, CharacterRemoveItem)>,
        sell: bool,
    ) -> Result<(), Error> {
        (
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.characters,
        )
            .transaction(|(inventories, owners, characters)| {
                let mut groups = BTreeMap::<i32, Vec<InventoryRecord>>::new();
                let mut chars = BTreeMap::<i32, CharSelectModel>::new();
                for (expected, removal) in updates {
                    if removal.amount <= 0 || (sell && removal.price < 0) {
                        return abort("Invalid inventory removal amount or price");
                    }
                    let id = removal.char_id as i32;
                    if !groups.contains_key(&id) {
                        groups.insert(id, tx_read(inventories, &id.to_be_bytes())?.unwrap_or_default());
                        chars.insert(id, tx_required(characters, &id.to_be_bytes())?);
                    }
                    let records = groups.get_mut(&id).unwrap();
                    let index = records
                        .iter()
                        .position(|item| item.id == expected.id && item.item_id == expected.item_id && item.unique_id == expected.unique_id)
                        .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                    let record = &mut records[index];
                    if record.equip != 0 || record.amount < removal.amount {
                        return abort("Cannot remove equipped items or more items than owned");
                    }
                    let remaining = record.amount - removal.amount;
                    if remaining != expected.amount {
                        return abort("Inventory changed before removal");
                    }
                    if remaining == 0 {
                        owners.remove(record.id.to_be_bytes().to_vec())?;
                        records.remove(index);
                    } else {
                        record.amount = remaining;
                    }
                    if sell {
                        let character = chars.get_mut(&id).unwrap();
                        let zeny = i64::from(character.zeny) + i64::from(removal.amount) * i64::from(removal.price);
                        character.zeny = i32::try_from(zeny).map_err(|_| {
                            sled::transaction::ConflictableTransactionError::Abort(Error::new("Zeny is out of bounds".into()))
                        })?;
                    }
                }
                for (id, records) in &groups {
                    tx_write(inventories, &id.to_be_bytes(), records)?;
                }
                if sell {
                    for (id, character) in &chars {
                        tx_write(characters, &id.to_be_bytes(), character)?;
                    }
                }
                Ok(())
            })?;
        Ok(())
    }

    async fn character_inventory_delete(&self, deletion: DeleteItems) -> Result<(), Error> {
        if deletion.amount_to_remove <= 0 {
            return Err(Error::new("Item consumption amount must be positive".into()));
        }
        (&self.database.inventories, &self.database.inventory_owners).transaction(|(inventories, owners)| {
            let key = deletion.char_id.to_be_bytes();
            let mut records: Vec<InventoryRecord> = tx_read(inventories, &key)?.unwrap_or_default();
            let index = records
                .iter()
                .position(|item| item.id == deletion.item_inventory_id && item.unique_id == deletion.unique_id)
                .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
            if deletion.amount_to_remove > records[index].amount {
                return abort("Cannot consume more items than owned");
            }
            records[index].amount -= deletion.amount_to_remove;
            if records[index].amount == 0 {
                owners.remove(records[index].id.to_be_bytes().to_vec())?;
                records.remove(index);
            }
            tx_write(inventories, &key, &records)
        })?;
        Ok(())
    }

    async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        let records: Vec<InventoryRecord> = database::read(&self.database.inventories, &char_id.to_be_bytes())?.unwrap_or_default();
        records
            .into_iter()
            .map(|record| {
                let item: ItemModel = database::required(&self.database.items, &record.item_id.to_be_bytes())?;
                Ok(InventoryItemModel::from_record(&record, &item))
            })
            .collect()
    }

    async fn character_inventory_wearable_item_update(&self, updates: Vec<InventoryItemModel>) -> Result<(), Error> {
        if updates.is_empty() { return Ok(()); }
        Err(Error::InvalidInput("Equipment changes require the original character owner".into()))
    }

    async fn character_inventory_commit_equipment(&self, char_id: u32, updates: Vec<InventoryItemModel>) -> Result<(), Error> {
        if char_id == 0 { return Err(Error::InvalidInput("Invalid equipment owner".into())); }
        (&self.database.inventories, &self.database.inventory_owners).transaction(|(inventories, owners)| {
            let mut records: Vec<InventoryRecord> = tx_required(inventories, &char_id.to_be_bytes())?;
            let mut seen = std::collections::BTreeSet::new();
            for update in &updates {
                let id: i32 = tx_required(owners, &update.id.to_be_bytes())?;
                if id != char_id as i32 || !seen.insert(update.id) {
                    return abort("Equipment owner changed or an instance was specified twice");
                }
                let record = records
                    .iter_mut()
                    .find(|record| record.id == update.id && record.item_id == update.item_id && record.unique_id == update.unique_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                if record.amount != update.amount || record.refine != update.refine || record.is_identified != update.is_identified
                    || record.is_damaged != update.is_damaged || [record.card0, record.card1, record.card2, record.card3]
                        != [update.card0, update.card1, update.card2, update.card3] {
                    return abort("Equipment instance changed before the update");
                }
                record.equip = update.equip;
            }
            tx_write(inventories, &char_id.to_be_bytes(), &records)
        })?;
        Ok(())
    }

    async fn character_slot_card(&self, char_id: i32, card: &InventoryItemModel, equipment: &InventoryItemModel) -> Result<i32, Error> {
        Ok((
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.items,
        )
            .transaction(|(inventories, owners, items)| {
                let key = char_id.to_be_bytes();
                let mut records: Vec<InventoryRecord> = tx_read(inventories, &key)?.unwrap_or_default();
                let equipment_index = records
                    .iter()
                    .position(|record| record.id == equipment.id && record.item_id == equipment.item_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let card_index = records
                    .iter()
                    .position(|record| record.id == card.id && record.item_id == card.item_id && record.amount > 0)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let card_model: ItemModel = tx_required(items, &card.item_id.to_be_bytes())?;
                let equipment_model: ItemModel = tx_required(items, &equipment.item_id.to_be_bytes())?;
                if equipment_index == card_index || card_model.item_type != ItemType::Card || records[card_index].equip != 0 {
                    return abort("Invalid card composition");
                }
                let record = &mut records[equipment_index];
                let cards = [record.card0, record.card1, record.card2, record.card3];
                let slots = equipment_model.slots.unwrap_or(0).clamp(0, 4) as usize;
                let slot = cards[..slots]
                    .iter()
                    .position(|id| *id == 0)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let card_id = i16::try_from(card.item_id)
                    .map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::new("Card ID is out of bounds".into())))?;
                match slot {
                    0 => record.card0 = card_id,
                    1 => record.card1 = card_id,
                    2 => record.card2 = card_id,
                    _ => record.card3 = card_id,
                }
                records[card_index].amount -= 1;
                if records[card_index].amount == 0 {
                    owners.remove(records[card_index].id.to_be_bytes().to_vec())?;
                    records.remove(card_index);
                }
                tx_write(inventories, &key, &records)?;
                Ok(slot as i32)
            })?)
    }
}
