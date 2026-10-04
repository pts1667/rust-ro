use std::collections::BTreeSet;

use database::model::{CharacterRecord, InventoryRecord};
use database::{abort, next_id, tx_read, tx_required, tx_write};
use models::enums::EnumWithMaskValueU64;
use models::enums::item::{EquipmentLocation, ItemTradeFlag, ItemType};
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use super::{character_key, write_state, Error, SledRepository};
use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::server::model::game_systems::{CharacterGameSystems, PetRecord, PlayerTradeItem, PlayerTradeReceipt};

#[derive(Debug, Clone)]
pub struct PlayerTradeSide {
    pub char_id: u32,
    pub account_id: u32,
    pub expected_revision: u64,
    pub max_weight: u32,
    pub max_slots: usize,
    pub items: Vec<PlayerTradeItem>,
    pub zeny: u32,
}

#[derive(Debug, Clone)]
pub struct PlayerTradeCommit {
    pub session_id: u64,
    pub first: PlayerTradeSide,
    pub second: PlayerTradeSide,
}

#[derive(Debug)]
pub struct PlayerTradeResult {
    pub first_inventory: Vec<InventoryItemModel>,
    pub second_inventory: Vec<InventoryItemModel>,
    pub first_zeny: u32,
    pub second_zeny: u32,
    pub first_systems: CharacterGameSystems,
    pub second_systems: CharacterGameSystems,
}

pub fn restrictions_allow(masks: &[u64], partners: bool) -> bool {
    masks.iter().all(|mask| mask & ItemTradeFlag::NoTrade.as_flag() == 0)
        || (partners && masks.iter().all(|mask| mask & ItemTradeFlag::TradePartner.as_flag() != 0))
}

fn same_instance(record: &InventoryRecord, offered: &InventoryRecord) -> bool {
    record.id == offered.id && record.unique_id == offered.unique_id && record.item_id == offered.item_id
        && record.refine == offered.refine && record.is_identified == offered.is_identified && record.is_damaged == offered.is_damaged
        && [record.card0, record.card1, record.card2, record.card3] == [offered.card0, offered.card1, offered.card2, offered.card3]
}

fn take_offers(records: &mut Vec<InventoryRecord>, side: &PlayerTradeSide, partners: bool, items: &TransactionalTree,
    owners: &TransactionalTree, systems: &TransactionalTree, recipient: u32) -> ConflictableTransactionResult<Vec<InventoryRecord>, Error> {
    if side.items.len() > 10 { return abort("A player trade can contain at most ten item lines"); }
    let mut seen = BTreeSet::new();
    let mut outgoing = Vec::with_capacity(side.items.len());
    for offer in &side.items {
        if offer.amount <= 0 || !seen.insert(offer.item.id) { return abort("Invalid or duplicate trade item offer"); }
        let index = records.iter().position(|record| same_instance(record, &offer.item))
            .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
        let record = &records[index];
        let owner: i32 = tx_required(owners, &record.id.to_be_bytes())?;
        let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
        if owner != side.char_id as i32 || record.amount < offer.amount || record.equip != offer.item.equip
            || (record.equip != 0 && record.equip as u64 != EquipmentLocation::Ammo.as_flag())
            || (!item.item_type.is_stackable() && (record.amount != 1 || offer.amount != 1)) {
            return abort("Trade item ownership, quantity or equipment changed");
        }
        let mut masks = vec![item.trade_flags];
        if !models::item::special_card_metadata(record.card0) {
            for card in [record.card0, record.card1, record.card2, record.card3].into_iter().take(item.slots.unwrap_or(0).clamp(0, 4) as usize).filter(|id| *id != 0) {
                let card: ItemModel = tx_required(items, &(card as u16 as i32).to_be_bytes())?;
                masks.push(card.trade_flags);
            }
        }
        if !restrictions_allow(&masks, partners) { return abort("An offered item or card cannot be traded"); }
        if item.item_type == ItemType::PetEgg && record.card0 == 256 {
            let pet_id = u32::from(record.card1 as u16) | (u32::from(record.card2 as u16) << 16);
            let pet_key = [b"pet/".as_slice(), &pet_id.to_be_bytes()].concat();
            let mut pet: PetRecord = tx_required(systems, &pet_key)?;
            if !pet.incubating || pet.owner_char_id != side.char_id || pet.egg_item_id != record.item_id || pet.egg_inventory_id != record.id {
                return abort("An active or changed pet egg cannot be traded");
            }
            pet.owner_char_id = recipient;
            tx_write(systems, &pet_key, &pet)?;
        }
        let mut moved = record.clone();
        moved.amount = offer.amount;
        moved.equip = 0;
        records[index].amount -= offer.amount;
        if records[index].amount == 0 { records.remove(index); owners.remove(moved.id.to_be_bytes().to_vec())?; }
        else { moved.id = 0; }
        outgoing.push(moved);
    }
    Ok(outgoing)
}

fn grant(records: &mut Vec<InventoryRecord>, mut incoming: InventoryRecord, owner: u32, items: &TransactionalTree,
    owners: &TransactionalTree, metadata: &TransactionalTree) -> ConflictableTransactionResult<(), Error> {
    let item: ItemModel = tx_required(items, &incoming.item_id.to_be_bytes())?;
    let matching = records.iter().position(|record| item.item_type.is_stackable() && record.item_id == incoming.item_id
        && record.unique_id == 0 && incoming.unique_id == 0 && record.equip == 0
        && record.refine == incoming.refine && record.is_identified == incoming.is_identified && record.is_damaged == incoming.is_damaged
        && [record.card0, record.card1, record.card2, record.card3] == [incoming.card0, incoming.card1, incoming.card2, incoming.card3]);
    if let Some(index) = matching {
        records[index].amount = records[index].amount.checked_add(incoming.amount)
            .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Trade stack would overflow".into())))?;
    } else {
        if incoming.unique_id != 0 && records.iter().any(|record| record.unique_id == incoming.unique_id) { return abort("Trade equipment identity already exists"); }
        if incoming.id == 0 { incoming.id = next_id(metadata, b"inventory_id", 0)?; }
        tx_write(owners, &incoming.id.to_be_bytes(), &(owner as i32))?;
        records.push(incoming);
    }
    Ok(())
}

fn inventory_models(records: &[InventoryRecord], items: &TransactionalTree, side: &PlayerTradeSide,
    character: &CharacterRecord, enforce_capacity: bool) -> ConflictableTransactionResult<Vec<InventoryItemModel>, Error> {
    if enforce_capacity && records.len() > side.max_slots.min(character.inventory_slots.max(0) as usize) { return abort("Trade recipient inventory is full"); }
    let mut weight = 0u64;
    let mut models = Vec::with_capacity(records.len());
    for record in records {
        let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
        if record.amount <= 0 || item.weight < 0 { return abort("Invalid trade inventory amount or weight"); }
        weight += item.weight as u64 * record.amount as u64;
        models.push(InventoryItemModel::from_record(record, &item));
    }
    if enforce_capacity && weight > u64::from(side.max_weight) { return abort("Trade recipient would be overweight"); }
    Ok(models)
}

pub fn commit(repository: &SledRepository, change: &PlayerTradeCommit) -> Result<PlayerTradeResult, Error> {
    if change.session_id == 0 || change.first.char_id == change.second.char_id || change.first.char_id == 0 || change.second.char_id == 0 {
        return Err(Error::new("Invalid player trade participants or session".into()));
    }
    Ok((&repository.database.characters, &repository.database.inventories, &repository.database.inventory_owners,
        &repository.database.items, &repository.database.metadata, &repository.database.game_systems)
        .transaction(|(characters, inventories, owners, items, metadata, systems)| {
            let mut first: CharacterRecord = tx_required(characters, &change.first.char_id.to_be_bytes())?;
            let mut second: CharacterRecord = tx_required(characters, &change.second.char_id.to_be_bytes())?;
            if first.account_id as u32 != change.first.account_id || second.account_id as u32 != change.second.account_id { return abort("Trade account ownership changed"); }
            let mut first_state: CharacterGameSystems = tx_read(systems, &character_key(change.first.char_id))?.unwrap_or_default();
            let mut second_state: CharacterGameSystems = tx_read(systems, &character_key(change.second.char_id))?.unwrap_or_default();
            let mut first_records: Vec<InventoryRecord> = tx_read(inventories, &change.first.char_id.to_be_bytes())?.unwrap_or_default();
            let mut second_records: Vec<InventoryRecord> = tx_read(inventories, &change.second.char_id.to_be_bytes())?.unwrap_or_default();
            let replayed = first_state.last_trade_receipt.as_ref().is_some_and(|receipt| receipt.session_id == change.session_id && receipt.partner_id == change.second.char_id)
                && second_state.last_trade_receipt.as_ref().is_some_and(|receipt| receipt.session_id == change.session_id && receipt.partner_id == change.first.char_id);
            if !replayed {
                if first_state.revision != change.first.expected_revision || second_state.revision != change.second.expected_revision { return abort("Player state changed before trade completion"); }
                if first.zeny < 0 || second.zeny < 0 || change.first.zeny > first.zeny as u32 || change.second.zeny > second.zeny as u32 { return abort("Trade zeny is no longer available"); }
                let partners = first_state.partner_id == change.second.char_id && second_state.partner_id == change.first.char_id;
                let outgoing_first = take_offers(&mut first_records, &change.first, partners, items, owners, systems, change.second.char_id)?;
                let outgoing_second = take_offers(&mut second_records, &change.second, partners, items, owners, systems, change.first.char_id)?;
                for record in outgoing_first { grant(&mut second_records, record, change.second.char_id, items, owners, metadata)?; }
                for record in outgoing_second { grant(&mut first_records, record, change.first.char_id, items, owners, metadata)?; }
                let first_zeny = i64::from(first.zeny) - i64::from(change.first.zeny) + i64::from(change.second.zeny);
                let second_zeny = i64::from(second.zeny) - i64::from(change.second.zeny) + i64::from(change.first.zeny);
                first.zeny = i32::try_from(first_zeny).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::new("Trade zeny would overflow".into())))?;
                second.zeny = i32::try_from(second_zeny).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::new("Trade zeny would overflow".into())))?;
            }
            let first_inventory = inventory_models(&first_records, items, &change.first, &first, !replayed)?;
            let second_inventory = inventory_models(&second_records, items, &change.second, &second, !replayed)?;
            if !replayed {
                first_state.last_trade_receipt = Some(PlayerTradeReceipt { session_id: change.session_id, partner_id: change.second.char_id });
                second_state.last_trade_receipt = Some(PlayerTradeReceipt { session_id: change.session_id, partner_id: change.first.char_id });
                write_state(systems, change.first.char_id, &mut first_state)?;
                write_state(systems, change.second.char_id, &mut second_state)?;
                tx_write(characters, &change.first.char_id.to_be_bytes(), &first)?;
                tx_write(characters, &change.second.char_id.to_be_bytes(), &second)?;
                tx_write(inventories, &change.first.char_id.to_be_bytes(), &first_records)?;
                tx_write(inventories, &change.second.char_id.to_be_bytes(), &second_records)?;
            }
            Ok(PlayerTradeResult { first_inventory, second_inventory, first_zeny: first.zeny as u32, second_zeny: second.zeny as u32,
                first_systems: first_state, second_systems: second_state })
        })?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use database::model::{AccountRecord, CharacterInventory, SeedData};
    use crate::repository::game_system_repository::GameSystemRepository;

    fn setup() -> (SledRepository, PlayerTradeCommit) {
        let repository = SledRepository::temporary().unwrap();
        let weapon = InventoryRecord { id: 10, unique_id: 987654, item_id: 1201, amount: 1, refine: 7,
            is_damaged: true, card0: 4001, card1: 4002, ..Default::default() };
        let potion = InventoryRecord { id: 20, item_id: 501, amount: 3, is_identified: true, ..Default::default() };
        repository.database.seed(&SeedData {
            accounts: vec![AccountRecord { account_id: 2000000, username: "Trader A".into(), password: "password".into() },
                AccountRecord { account_id: 2000001, username: "Trader B".into(), password: "password".into() }],
            characters: vec![CharacterRecord { char_id: 150000, account_id: 2000000, name: "Trader A".into(), zeny: 100, hp: 100, inventory_slots: 100, ..Default::default() },
                CharacterRecord { char_id: 150001, account_id: 2000001, name: "Trader B".into(), zeny: 50, hp: 100, inventory_slots: 100, ..Default::default() }],
            inventories: vec![CharacterInventory { char_id: 150000, items: vec![weapon.clone()] },
                CharacterInventory { char_id: 150001, items: vec![potion.clone()] }], ..Default::default()
        }, false).unwrap();
        repository.database.items.transaction(|tree| {
            for (id, kind, weight, slots) in [(1201_i32, "Weapon", 200, 2), (501, "Healing", 10, 0), (4001, "Card", 10, 0), (4002, "Card", 10, 0)] {
                let item: ItemModel = serde_json::from_value(serde_json::json!({"id":id,"name_aegis":format!("Item{id}"),"name_english":format!("Item {id}"),
                    "item_type":kind,"weight":weight,"slots":slots,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0})).unwrap();
                tx_write(tree, &id.to_be_bytes(), &item)?;
            }
            Ok(())
        }).unwrap();
        let change = PlayerTradeCommit { session_id: 77,
            first: PlayerTradeSide { char_id: 150000, account_id: 2000000, expected_revision: 0, max_weight: 10000, max_slots: 100,
                items: vec![PlayerTradeItem { inventory_index: 0, item: weapon, amount: 1 }], zeny: 3 },
            second: PlayerTradeSide { char_id: 150001, account_id: 2000001, expected_revision: 0, max_weight: 10000, max_slots: 100,
                items: vec![PlayerTradeItem { inventory_index: 0, item: potion, amount: 2 }], zeny: 7 } };
        (repository, change)
    }

    fn snapshot(repository: &SledRepository) -> Vec<Vec<(Vec<u8>, Vec<u8>)>> {
        [&repository.database.characters, &repository.database.inventories, &repository.database.inventory_owners,
            &repository.database.items, &repository.database.metadata, &repository.database.game_systems]
            .into_iter().map(|tree| tree.iter().map(|pair| { let (key, value) = pair.unwrap(); (key.to_vec(), value.to_vec()) }).collect()).collect()
    }

    #[test]
    fn player_trade_swaps_exact_instances_partial_stacks_and_zeny_once_across_both_accounts() {
        let (repository, change) = setup();
        let result = repository.commit_player_trade(&change).unwrap();
        assert_eq!((result.first_zeny, result.second_zeny), (104, 46));
        assert_eq!(result.first_inventory.len(), 1);
        assert_eq!((result.first_inventory[0].item_id, result.first_inventory[0].amount), (501, 2));
        let weapon = result.second_inventory.iter().find(|item| item.item_id == 1201).unwrap();
        assert_eq!((weapon.id, weapon.unique_id, weapon.refine, weapon.is_identified, weapon.is_damaged,
            [weapon.card0, weapon.card1, weapon.card2, weapon.card3]), (10, 987654, 7, false, true, [4001, 4002, 0, 0]));
        assert_eq!(result.second_inventory.iter().find(|item| item.item_id == 501).unwrap().amount, 1);
        assert_eq!(database::required::<i32>(&repository.database.inventory_owners, &10_i32.to_be_bytes()).unwrap(), 150001);
        let after = snapshot(&repository);
        let replay = repository.commit_player_trade(&change).unwrap();
        assert_eq!((replay.first_zeny, replay.second_zeny), (104, 46));
        assert_eq!(snapshot(&repository), after);
    }

    #[test]
    fn player_trade_rejects_stale_identity_quantity_wallet_owner_capacity_and_duplicate_offers_without_partial_writes() {
        let (repository, change) = setup();
        let before = snapshot(&repository);
        let mut cases = Vec::new();
        let mut stale = change.clone(); stale.first.items[0].item.refine += 1; cases.push(stale);
        let mut missing = change.clone(); missing.first.items[0].item.unique_id += 1; cases.push(missing);
        let mut quantity = change.clone(); quantity.second.items[0].amount = 4; cases.push(quantity);
        let mut wallet = change.clone(); wallet.first.zeny = 101; cases.push(wallet);
        let mut owner = change.clone(); owner.first.account_id += 1; cases.push(owner);
        let mut revision = change.clone(); revision.second.expected_revision += 1; cases.push(revision);
        let mut capacity = change.clone(); capacity.second.max_slots = 1; cases.push(capacity);
        let mut overweight = change.clone(); overweight.second.max_weight = 1; cases.push(overweight);
        let mut duplicate = change.clone(); duplicate.first.items.push(duplicate.first.items[0].clone()); cases.push(duplicate);
        for case in cases {
            assert!(repository.commit_player_trade(&case).is_err());
            assert_eq!(snapshot(&repository), before);
        }
    }

    #[test]
    fn player_trade_checks_card_restrictions_and_allows_only_the_complete_partner_exception() {
        let (repository, mut change) = setup();
        repository.database.items.transaction(|tree| {
            let mut card: ItemModel = tx_required(tree, &4001_i32.to_be_bytes())?;
            card.trade_flags = ItemTradeFlag::NoTrade.as_flag() | ItemTradeFlag::TradePartner.as_flag();
            tx_write(tree, &4001_i32.to_be_bytes(), &card)
        }).unwrap();
        let before = snapshot(&repository);
        assert!(repository.commit_player_trade(&change).is_err());
        assert_eq!(snapshot(&repository), before);
        for (char_id, partner_id) in [(150000, 150001), (150001, 150000)] {
            let mut state = repository.character_game_systems(char_id).unwrap();
            state.partner_id = partner_id;
            let state = repository.save_character_game_systems(char_id, &state).unwrap();
            if char_id == change.first.char_id { change.first.expected_revision = state.revision; }
            else { change.second.expected_revision = state.revision; }
        }
        assert!(repository.commit_player_trade(&change).is_err());
        repository.database.items.transaction(|tree| {
            for id in [1201_i32, 4002] {
                let mut item: ItemModel = tx_required(tree, &id.to_be_bytes())?;
                item.trade_flags |= ItemTradeFlag::TradePartner.as_flag();
                tx_write(tree, &id.to_be_bytes(), &item)?;
            }
            Ok(())
        }).unwrap();
        repository.commit_player_trade(&change).unwrap();
    }

    #[test]
    fn player_trade_zeny_overflow_rolls_back_items_and_owner_indices() {
        let (repository, change) = setup();
        repository.database.characters.transaction(|tree| {
            let mut character: CharacterRecord = tx_required(tree, &change.first.char_id.to_be_bytes())?;
            character.zeny = i32::MAX;
            tx_write(tree, &change.first.char_id.to_be_bytes(), &character)
        }).unwrap();
        let before = snapshot(&repository);
        assert!(repository.commit_player_trade(&change).is_err());
        assert_eq!(snapshot(&repository), before);
    }
}
