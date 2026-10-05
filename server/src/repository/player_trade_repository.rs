use std::collections::BTreeSet;

use database::model::{CharacterRecord, InventoryRecord};
use database::{abort, next_id, tx_read, tx_required, tx_write};
use models::enums::EnumWithMaskValueU64;
use models::enums::item::{EquipmentLocation, ItemTradeFlag, ItemType};
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use super::{character_key, write_state, Error, SledRepository};
use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::server::model::game_systems::{CharacterGameSystems, PlayerTradeItem, PlayerTradeReceipt};

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

pub(super) fn allocate_session(repository: &SledRepository) -> Result<u64, Error> {
    Ok(repository.database.metadata.transaction(|metadata| {
        let previous: u64 = tx_read(metadata, b"player_trade_session_id")?.unwrap_or(0);
        let next = previous.checked_add(1)
            .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Player trade session ID exhausted".into())))?;
        tx_write(metadata, b"player_trade_session_id", &next)?;
        Ok(next)
    })?)
}

fn offer_payload(change: &PlayerTradeCommit) -> Result<Vec<u8>, Error> {
    let mut sides = [&change.first, &change.second];
    sides.sort_by_key(|side| side.char_id);
    let offers: Vec<_> = sides.into_iter().map(|side| {
        let mut items: Vec<_> = side.items.iter().map(|offer| (&offer.item, offer.amount)).collect();
        items.sort_by_key(|(item, _)| item.id);
        (side.char_id, side.account_id, side.zeny, items)
    }).collect();
    serde_json::to_vec(&offers).map_err(|error| Error::new(error.to_string()))
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
            || (record.equip != 0 && (item.item_type != ItemType::Ammo || record.equip as u64 != EquipmentLocation::Ammo.as_flag()))
            || (!item.item_type.is_stackable() && (record.amount != 1 || offer.amount != 1)) {
            return abort("Trade item ownership, quantity or equipment changed");
        }
        let masks = super::trade::trade_masks(items, &item, record)?;
        if !restrictions_allow(&masks, partners) { return abort("An offered item or card cannot be traded"); }
        if item.item_type == ItemType::PetEgg && record.card0 == 256 {
            super::pet_custody::transfer_egg(systems, record, record.id, recipient, side.char_id)?;
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
        && record.unique_id == 0 && incoming.unique_id == 0
        && (record.equip == 0 || (item.item_type == ItemType::Ammo && record.equip as u64 == EquipmentLocation::Ammo.as_flag()))
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
        if item.item_type.is_stackable() && item.stack_inventory.unwrap_or(0) != 0
            && i32::from(record.amount) > item.stack_amount.unwrap_or(i32::from(i16::MAX)).min(i32::from(i16::MAX)) {
            return abort("Trade stack exceeds the configured inventory limit");
        }
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
    if change.first.items.len() > 10 || change.second.items.len() > 10 {
        return Err(Error::new("A player trade can contain at most ten item lines".into()));
    }
    let payload = offer_payload(change)?;
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
            let session_seen = [&first_state, &second_state].into_iter()
                .any(|state| state.last_trade_receipt.as_ref().is_some_and(|receipt| receipt.session_id == change.session_id));
            if session_seen && (!replayed || first_state.last_trade_receipt.as_ref().is_none_or(|receipt| receipt.payload != payload)
                || second_state.last_trade_receipt.as_ref().is_none_or(|receipt| receipt.payload != payload)) {
                return abort("Player trade session was already committed with different offers");
            }
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
                first_state.last_trade_receipt = Some(PlayerTradeReceipt { session_id: change.session_id, partner_id: change.second.char_id, payload: payload.clone() });
                second_state.last_trade_receipt = Some(PlayerTradeReceipt { session_id: change.session_id, partner_id: change.first.char_id, payload: payload.clone() });
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

    fn pet_fixture() -> (SledRepository, crate::server::model::game_systems::PetRecord, InventoryRecord) {
        use crate::server::model::game_systems::PetRecord;
        let (repository, _) = setup();
        repository.database.seed(&SeedData {
            characters: vec![CharacterRecord { char_id: 150002, account_id: 2000000, char_num: 1, name: "Trader C".into(),
                hp: 100, inventory_slots: 100, ..Default::default() }], ..Default::default()
        }, false).unwrap();
        repository.database.items.transaction(|tree| {
            let item: ItemModel = serde_json::from_value(serde_json::json!({"id":9001,"name_aegis":"Poring_Egg","name_english":"Poring Egg",
                "item_type":"PetEgg","weight":10,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0})).unwrap();
            tx_write(tree, &9001_i32.to_be_bytes(), &item)
        }).unwrap();
        let (pet, egg) = repository.create_pet_egg(150000, &PetRecord { id: 0, owner_char_id: 0, class_id: 1002,
            name: "Custody Poring".into(), level: 1, egg_item_id: 9001, egg_inventory_id: 0, intimacy: 550, hunger: 70,
            equipped_item: 0, next_hunger_at: 0, incubating: true, renamed: true }, 10000).unwrap();
        let records: Vec<InventoryRecord> = database::required(&repository.database.inventories, &150000_u32.to_be_bytes()).unwrap();
        let egg = records.into_iter().find(|record| record.id == egg.id).unwrap();
        (repository, pet, egg)
    }

    fn egg_trade(repository: &SledRepository, source: u32, destination: u32, egg: &InventoryRecord) -> PlayerTradeCommit {
        let side = |char_id, items| {
            let character: CharacterRecord = database::required(&repository.database.characters, &u32::to_be_bytes(char_id)).unwrap();
            PlayerTradeSide { char_id, account_id: character.account_id as u32,
                expected_revision: repository.character_game_systems(char_id).unwrap().revision,
                max_weight: 10000, max_slots: 100, items, zeny: 0 }
        };
        PlayerTradeCommit { session_id: repository.allocate_player_trade_session_id().unwrap(),
            first: side(source, vec![PlayerTradeItem { inventory_index: 0, item: egg.clone(), amount: 1 }]),
            second: side(destination, Vec::new()) }
    }

    fn stored_pet(repository: &SledRepository, pet_id: u32) -> crate::server::model::game_systems::PetRecord {
        database::required(&repository.database.game_systems, &super::super::key(b"pet/", pet_id)).unwrap()
    }

    #[test]
    fn player_trade_pet_egg_follows_account_storage_cart_custody_and_hatches_after_another_transfer() {
        use crate::server::model::game_systems::{ItemContainer, PlayerOption};
        let (repository, pet, egg) = pet_fixture();
        repository.storage_transfer(150000, egg.id, 1, true, 10000).unwrap();
        assert_eq!(stored_pet(&repository, pet.id).owner_char_id, 0);
        assert!(repository.database.inventory_owners.get(egg.id.to_be_bytes()).unwrap().is_none());
        repository.set_character_options(150002, 0, PlayerOption::Cart1.as_flag()).unwrap();
        repository.container_transfer(150002, egg.id, 1, ItemContainer::Storage, ItemContainer::Cart, 10000).unwrap();
        assert_eq!(stored_pet(&repository, pet.id).owner_char_id, 150002);
        repository.container_transfer(150002, egg.id, 1, ItemContainer::Cart, ItemContainer::Inventory, 10000).unwrap();
        repository.database.game_systems.transaction(|tree| {
            let mut cached: crate::server::model::game_systems::PetRecord = tx_required(tree, &super::super::key(b"pet/", pet.id))?;
            cached.owner_char_id = 150000;
            cached.egg_inventory_id += 1000;
            tx_write(tree, &super::super::key(b"pet/", pet.id), &cached)
        }).unwrap();
        let change = egg_trade(&repository, 150002, 150001, &egg);
        let result = repository.commit_player_trade(&change).unwrap();
        let transferred = result.second_inventory.iter().find(|record| record.id == egg.id).unwrap();
        assert_eq!((transferred.unique_id, transferred.card0, transferred.card1, transferred.card2, transferred.card3),
            (egg.unique_id, egg.card0, egg.card1, egg.card2, egg.card3));
        assert_eq!((stored_pet(&repository, pet.id).owner_char_id, stored_pet(&repository, pet.id).egg_inventory_id), (150001, egg.id));
        let state = repository.hatch_pet(150001, egg.id, 1000).unwrap();
        let hatched = state.pet.unwrap();
        assert_eq!((hatched.id, hatched.owner_char_id, hatched.name, hatched.intimacy), (pet.id, 150001, pet.name, 550));
        repository.return_pet_to_egg(150001).unwrap();
        let inventory: Vec<InventoryRecord> = database::required(&repository.database.inventories, &150001_u32.to_be_bytes()).unwrap();
        assert_eq!(inventory.iter().find(|record| record.id == egg.id), Some(&egg));
    }

    #[test]
    fn player_trade_pet_egg_follows_guild_storage_custody_and_all_membership_and_capacity_failures_roll_back() {
        use crate::server::model::game_systems::{GuildStorageAccess, ItemContainer};
        let (repository, pet, egg) = pet_fixture();
        (&repository.database.inventories, &repository.database.inventory_owners).transaction(|(inventories, owners)| {
            let mut records: Vec<InventoryRecord> = tx_required(inventories, &150000_u32.to_be_bytes())?;
            records.push(InventoryRecord { id: 100, item_id: 714, amount: 1, is_identified: true, ..Default::default() });
            tx_write(inventories, &150000_u32.to_be_bytes(), &records)?;
            tx_write(owners, &100_i32.to_be_bytes(), &150000_i32)
        }).unwrap();
        let guild = repository.create_guild(150000, "Egg Custody Guild".into()).unwrap();
        repository.join_guild(150001, guild.id).unwrap();
        assert!(matches!(repository.open_guild_storage(150000).unwrap(), GuildStorageAccess::Open { .. }));
        repository.container_transfer(150000, egg.id, 1, ItemContainer::Inventory, ItemContainer::GuildStorage, 10000).unwrap();
        assert_eq!(stored_pet(&repository, pet.id).owner_char_id, 0);
        let before = snapshot(&repository);
        assert!(repository.container_transfer(150002, egg.id, 1, ItemContainer::GuildStorage, ItemContainer::Inventory, 10000).is_err());
        assert_eq!(snapshot(&repository), before);
        repository.close_guild_storage(150000, guild.id).unwrap();
        assert!(matches!(repository.open_guild_storage(150001).unwrap(), GuildStorageAccess::Open { .. }));
        let before = snapshot(&repository);
        assert!(repository.container_transfer(150001, egg.id, 1, ItemContainer::GuildStorage, ItemContainer::Inventory, 0).is_err());
        assert_eq!(snapshot(&repository), before);
        repository.container_transfer(150001, egg.id, 1, ItemContainer::GuildStorage, ItemContainer::Inventory, 10000).unwrap();
        repository.close_guild_storage(150001, guild.id).unwrap();
        assert_eq!(stored_pet(&repository, pet.id).owner_char_id, 150001);
        repository.commit_player_trade(&egg_trade(&repository, 150001, 150000, &egg)).unwrap();
        let hatched = repository.hatch_pet(150000, egg.id, 1000).unwrap().pet.unwrap();
        assert_eq!((hatched.id, hatched.owner_char_id, hatched.intimacy, hatched.name), (pet.id, 150000, 550, pet.name));
    }

    #[test]
    fn player_trade_pet_egg_vending_transfer_preserves_identity_and_rolls_back_pet_custody_with_wallet_and_store() {
        use crate::server::model::game_systems::{ItemContainer, PlayerOption, VendingOffer, VendingStore};
        let (repository, pet, egg) = pet_fixture();
        repository.set_character_options(150000, 0, PlayerOption::Cart1.as_flag()).unwrap();
        repository.container_transfer(150000, egg.id, 1, ItemContainer::Inventory, ItemContainer::Cart, 10000).unwrap();
        let store = repository.create_vending_store(&VendingStore { id: 0, char_id: 150000, account_id: 2000000,
            title: "Poring Egg".into(), map: "prontera".into(), map_instance: 0, x: 100, y: 100,
            offers: vec![VendingOffer { inventory_id: egg.id, index: 0, amount: 1, price: 10 }] }).unwrap();
        let before = snapshot(&repository);
        assert!(repository.vending_store_trade(150001, store.id, &[(0, 1)], 0).is_err());
        assert_eq!(snapshot(&repository), before);
        let result = repository.vending_store_trade(150001, store.id, &[(0, 1)], 10000).unwrap();
        assert_eq!((result.seller_zeny, result.buyer_zeny), (110, 40));
        assert!(result.store.is_none());
        assert!(result.cart.is_empty());
        assert_eq!((stored_pet(&repository, pet.id).owner_char_id, stored_pet(&repository, pet.id).egg_inventory_id), (150001, egg.id));
        let after = snapshot(&repository);
        assert!(repository.vending_store_trade(150001, store.id, &[(0, 1)], 10000).is_err());
        assert_eq!(snapshot(&repository), after);
        repository.commit_player_trade(&egg_trade(&repository, 150001, 150002, &egg)).unwrap();
        let hatched = repository.hatch_pet(150002, egg.id, 1000).unwrap().pet.unwrap();
        assert_eq!((hatched.id, hatched.owner_char_id, hatched.name, hatched.intimacy), (pet.id, 150002, pet.name, 550));
    }

    #[test]
    fn player_trade_rejects_active_or_forged_pet_eggs_without_changing_either_inventory_or_pet() {
        for mutation in 0..6 {
            let (repository, pet, mut egg) = pet_fixture();
            (&repository.database.game_systems, &repository.database.inventories, &repository.database.inventory_owners)
                .transaction(|(systems, inventories, owners)| {
                    let mut cached = pet.clone();
                    match mutation {
                        0 => cached.incubating = false,
                        1 => cached.egg_item_id += 1,
                        2 | 5 => {
                            let mut state = CharacterGameSystems::default();
                            let mut active = pet.clone(); active.incubating = false; state.pet = Some(active);
                            tx_write(systems, &character_key(150000), &state)?;
                            if mutation == 5 { cached.owner_char_id = 0; }
                        }
                        3 => tx_write(owners, &egg.id.to_be_bytes(), &150001_i32)?,
                        4 => {
                            let mut records: Vec<InventoryRecord> = tx_required(inventories, &150000_u32.to_be_bytes())?;
                            let record = records.iter_mut().find(|record| record.id == egg.id).unwrap(); record.unique_id += 100;
                            tx_write(inventories, &150000_u32.to_be_bytes(), &records)?;
                        }
                        _ => unreachable!(),
                    }
                    tx_write(systems, &super::super::key(b"pet/", pet.id), &cached)
                }).unwrap();
            if mutation == 4 { egg.unique_id += 100; }
            let mut change = egg_trade(&repository, 150000, 150001, &egg);
            change.first.zeny = 5;
            let before = snapshot(&repository);
            assert!(repository.commit_player_trade(&change).is_err(), "mutation {mutation}");
            assert_eq!(snapshot(&repository), before, "mutation {mutation}");
            assert!(repository.storage_transfer(150000, egg.id, 1, true, 10000).is_err(), "mutation {mutation}");
            assert_eq!(snapshot(&repository), before, "mutation {mutation}");
            assert!(repository.hatch_pet(150000, egg.id, 1000).is_err(), "mutation {mutation}");
            assert_eq!(snapshot(&repository), before, "mutation {mutation}");
        }
    }

    #[test]
    fn player_trade_replays_only_the_original_payload_and_rejects_changed_receipts_without_writes() {
        let (repository, change) = setup();
        repository.commit_player_trade(&change).unwrap();
        let before = snapshot(&repository);
        let mut cases = Vec::new();
        let mut wallet = change.clone(); wallet.first.zeny += 1; cases.push(wallet);
        let mut amount = change.clone(); amount.second.items[0].amount += 1; cases.push(amount);
        let mut identity = change.clone(); identity.first.items[0].item.unique_id += 1; cases.push(identity);
        let mut missing = change.clone(); missing.first.items.clear(); cases.push(missing);
        for case in cases {
            assert!(repository.commit_player_trade(&case).is_err());
            assert_eq!(snapshot(&repository), before);
        }
        let mut reordered = change.clone();
        std::mem::swap(&mut reordered.first, &mut reordered.second);
        reordered.first.expected_revision = 999;
        reordered.second.max_weight = 0;
        repository.commit_player_trade(&reordered).unwrap();
        assert_eq!(snapshot(&repository), before);
        repository.database.game_systems.transaction(|systems| {
            let mut second: CharacterGameSystems = tx_required(systems, &character_key(150001))?;
            second.last_trade_receipt.as_mut().unwrap().payload.clear();
            tx_write(systems, &character_key(150001), &second)
        }).unwrap();
        let before = snapshot(&repository);
        assert!(repository.commit_player_trade(&change).is_err());
        assert_eq!(snapshot(&repository), before);
    }

    #[test]
    fn player_trade_received_ammo_merges_into_an_equipped_stack_without_using_an_extra_slot() {
        let (repository, mut change) = setup();
        repository.database.items.transaction(|items| {
            let item: ItemModel = serde_json::from_value(serde_json::json!({"id":1750,"name_aegis":"Arrow","name_english":"Arrow",
                "item_type":"Ammo","weight":1,"job_flags":0,"class_flags":0,"location":EquipmentLocation::Ammo.as_flag(),"flags":0,"trade_flags":0})).unwrap();
            tx_write(items, &1750_i32.to_be_bytes(), &item)
        }).unwrap();
        let arrow = InventoryRecord { id: 50, item_id: 1750, amount: 10, is_identified: true,
            equip: EquipmentLocation::Ammo.as_flag() as i32, ..Default::default() };
        let other = InventoryRecord { id: 51, amount: 20, ..arrow.clone() };
        (&repository.database.inventories, &repository.database.inventory_owners, &repository.database.metadata).transaction(|(inventories, owners, metadata)| {
            for (char_id, record) in [(150000_i32, &arrow), (150001, &other)] {
                tx_write(inventories, &char_id.to_be_bytes(), &vec![record.clone()])?;
                tx_write(owners, &record.id.to_be_bytes(), &char_id)?;
            }
            tx_write(metadata, b"inventory_id", &51_i32)
        }).unwrap();
        change.first.items = vec![PlayerTradeItem { inventory_index: 0, item: arrow.clone(), amount: 5 }];
        change.second.items.clear();
        change.first.zeny = 0; change.second.zeny = 0; change.second.max_slots = 1;
        let result = repository.commit_player_trade(&change).unwrap();
        assert_eq!(result.second_inventory.len(), 1);
        let received = &result.second_inventory[0];
        assert_eq!((received.id, received.amount, received.equip), (51, 25, other.equip));
        assert_eq!(result.first_inventory[0].amount, 5);
    }

    #[test]
    fn player_trade_session_ids_remain_monotonic_after_reopen_and_reject_overflow_without_writes() {
        let path = std::env::temp_dir().join(format!("rust-ro-trade-session-{}-{}", std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        {
            let repository = SledRepository { database: database::Database::open(&path).unwrap() };
            assert_eq!(repository.allocate_player_trade_session_id().unwrap(), 1);
            assert_eq!(repository.allocate_player_trade_session_id().unwrap(), 2);
        }
        {
            let repository = SledRepository { database: database::Database::open(&path).unwrap() };
            assert_eq!(repository.allocate_player_trade_session_id().unwrap(), 3);
            repository.database.metadata.transaction(|metadata| tx_write(metadata, b"player_trade_session_id", &u64::MAX)).unwrap();
            let before = snapshot(&repository);
            assert!(repository.allocate_player_trade_session_id().is_err());
            assert_eq!(snapshot(&repository), before);
        }
        std::fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn player_trade_container_and_vending_checks_include_actual_socketed_card_restrictions() {
        use crate::server::model::game_systems::{ItemContainer, PlayerOption, VendingOffer, VendingStore};
        let (repository, change) = setup();
        let weapon_id = change.first.items[0].item.id;
        repository.set_character_options(150000, 0, PlayerOption::Cart1.as_flag()).unwrap();
        repository.database.inventories.transaction(|tree| {
            let mut inventory: Vec<InventoryRecord> = tx_required(tree, &150000_u32.to_be_bytes())?;
            inventory[0].is_identified = true; inventory[0].is_damaged = false;
            tx_write(tree, &150000_u32.to_be_bytes(), &inventory)
        }).unwrap();
        repository.database.items.transaction(|tree| {
            let mut card: ItemModel = tx_required(tree, &4001_i32.to_be_bytes())?;
            card.trade_flags = ItemTradeFlag::NoStorage.as_flag() | ItemTradeFlag::NoCart.as_flag();
            tx_write(tree, &4001_i32.to_be_bytes(), &card)
        }).unwrap();
        let before = snapshot(&repository);
        assert!(repository.storage_transfer(150000, weapon_id, 1, true, 10000).is_err());
        assert!(repository.container_transfer(150000, weapon_id, 1, ItemContainer::Inventory, ItemContainer::Storage, 10000).is_err());
        assert!(repository.container_transfer(150000, weapon_id, 1, ItemContainer::Inventory, ItemContainer::Cart, 10000).is_err());
        assert_eq!(snapshot(&repository), before);
        repository.database.items.transaction(|tree| {
            let mut card: ItemModel = tx_required(tree, &4001_i32.to_be_bytes())?;
            card.trade_flags = ItemTradeFlag::NoTrade.as_flag();
            tx_write(tree, &4001_i32.to_be_bytes(), &card)
        }).unwrap();
        repository.container_transfer(150000, weapon_id, 1, ItemContainer::Inventory, ItemContainer::Cart, 10000).unwrap();
        let store = VendingStore { id: 0, char_id: 150000, account_id: 2000000, title: "Carded Weapon".into(),
            map: "prontera".into(), map_instance: 0, x: 100, y: 100,
            offers: vec![VendingOffer { inventory_id: weapon_id, index: 0, amount: 1, price: 10 }] };
        let before = snapshot(&repository);
        assert!(repository.create_vending_store(&store).is_err());
        assert_eq!(snapshot(&repository), before);
        repository.database.items.transaction(|tree| {
            let mut card: ItemModel = tx_required(tree, &4001_i32.to_be_bytes())?;
            card.trade_flags = 0;
            tx_write(tree, &4001_i32.to_be_bytes(), &card)
        }).unwrap();
        let store = repository.create_vending_store(&store).unwrap();
        repository.database.items.transaction(|tree| {
            let mut card: ItemModel = tx_required(tree, &4001_i32.to_be_bytes())?;
            card.trade_flags = ItemTradeFlag::NoTrade.as_flag();
            tx_write(tree, &4001_i32.to_be_bytes(), &card)
        }).unwrap();
        let before = snapshot(&repository);
        assert!(repository.vending_store_trade(150001, store.id, &[(0, 1)], 10000).is_err());
        assert_eq!(snapshot(&repository), before);
    }

    #[test]
    fn player_trade_non_ammo_items_cannot_use_an_equipped_ammo_offer() {
        let (repository, mut change) = setup();
        repository.database.inventories.transaction(|tree| {
            let mut inventory: Vec<InventoryRecord> = tx_required(tree, &150000_u32.to_be_bytes())?;
            inventory[0].equip = EquipmentLocation::Ammo.as_flag() as i32;
            tx_write(tree, &150000_u32.to_be_bytes(), &inventory)
        }).unwrap();
        change.first.items[0].item.equip = EquipmentLocation::Ammo.as_flag() as i32;
        let before = snapshot(&repository);
        assert!(repository.commit_player_trade(&change).is_err());
        assert_eq!(snapshot(&repository), before);
    }

    #[test]
    fn player_trade_wallets_survive_a_stale_generic_character_snapshot() {
        use crate::repository::CharacterRepository;
        use models::status::{Status, StatusSnapshot};
        let (repository, change) = setup();
        let mut old_first = Status::default(); old_first.zeny = 100;
        let mut old_second = Status::default(); old_second.zeny = 50;
        let result = repository.commit_player_trade(&change).unwrap();
        let snapshots = vec![StatusSnapshot::_from(&old_first), StatusSnapshot::_from(&old_second)];
        tokio::runtime::Runtime::new().unwrap().block_on(repository.characters_update(
            vec![&old_first, &old_second], snapshots, vec![150000, 150001], vec![90, 91], vec![100, 101],
            vec!["prontera".into(), "prontera".into()],
        )).unwrap();
        let first: CharacterRecord = database::required(&repository.database.characters, &150000_u32.to_be_bytes()).unwrap();
        let second: CharacterRecord = database::required(&repository.database.characters, &150001_u32.to_be_bytes()).unwrap();
        assert_eq!((first.zeny, second.zeny), (result.first_zeny as i32, result.second_zeny as i32));
        assert_eq!((first.last_x, second.last_x), (90, 91));
        let before = snapshot(&repository);
        let replay = repository.commit_player_trade(&change).unwrap();
        assert_eq!((replay.first_zeny, replay.second_zeny), (104, 46));
        assert_eq!(snapshot(&repository), before);
    }

    #[test]
    fn player_trade_equipment_writes_and_deletions_cannot_follow_an_instance_to_its_new_owner() {
        use crate::repository::InventoryRepository;
        use crate::server::model::events::persistence_event::DeleteItems;
        let (repository, change) = setup();
        let item: ItemModel = database::required(&repository.database.items, &1201_i32.to_be_bytes()).unwrap();
        let mut old_update = InventoryItemModel::from_record(&change.first.items[0].item, &item);
        old_update.equip = EquipmentLocation::HandRight.as_flag() as i32;
        let result = repository.commit_player_trade(&change).unwrap();
        let before = snapshot(&repository);
        futures::executor::block_on(async {
            assert!(repository.character_inventory_wearable_item_update(vec![old_update.clone()]).await.is_err());
            assert!(repository.character_inventory_commit_equipment(150000, vec![old_update.clone()]).await.is_err());
            assert!(matches!(repository.character_inventory_delete(DeleteItems { char_id: 150000,
                item_inventory_id: old_update.id, unique_id: old_update.unique_id, amount_to_remove: 1 }).await, Err(Error::NotFound)));
            let mut foreign = result.first_inventory[0].clone(); foreign.equip = EquipmentLocation::HandRight.as_flag() as i32;
            assert!(repository.character_inventory_commit_equipment(150001, vec![old_update.clone(), foreign]).await.is_err());
        });
        assert_eq!(snapshot(&repository), before);
        futures::executor::block_on(repository.character_inventory_commit_equipment(150001, vec![old_update.clone()])).unwrap();
        let records: Vec<InventoryRecord> = database::required(&repository.database.inventories, &150001_u32.to_be_bytes()).unwrap();
        let equipped = records.iter().find(|record| record.id == old_update.id).unwrap();
        let mut expected = change.first.items[0].item.clone(); expected.equip = old_update.equip;
        assert_eq!(equipped, &expected);
    }
}
