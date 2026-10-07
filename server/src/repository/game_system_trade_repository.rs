use super::*;
use crate::repository::MAX_ZENY;
use crate::server::model::game_systems::{ItemContainer, PlayerOption, VendingStore};

pub struct StoreMove {
    pub char_id: u32,
    pub account_id: u32,
    pub vending_id: Option<u32>,
    pub buying_id: Option<u32>,
    pub map: String,
    pub map_instance: u8,
    pub x: u16,
    pub y: u16,
    pub close_vending: bool,
    pub close_buying: bool,
}

pub struct StoreMoveResult {
    pub vending: Option<VendingStore>,
    pub buying: Option<BuyingStore>,
}

pub fn move_character_stores(repository: &SledRepository, change: &StoreMove) -> Result<StoreMoveResult, Error> {
    let db = &repository.database;
    Ok((&db.game_systems, &db.characters).transaction(|(systems, characters)| {
        let character: CharacterRecord = tx_required(characters, &(change.char_id as i32).to_be_bytes())?;
        if character.account_id as u32 != change.account_id {
            return abort("Store movement account does not own the character");
        }
        if tx_read::<u32>(systems, &key(b"vender/", change.char_id))? != change.vending_id
            || tx_read::<u32>(systems, &key(b"buyer/", change.char_id))? != change.buying_id
        {
            return abort("Store changed before map movement");
        }
        let mut vending = None;
        if let Some(id) = change.vending_id {
            let mut store: VendingStore = tx_required(systems, &key(b"vending/", id))?;
            if store.char_id != change.char_id || store.account_id != change.account_id {
                return abort("Vending store belongs to another character");
            }
            if change.close_vending {
                systems.remove(key(b"vending/", id))?;
                systems.remove(key(b"vender/", change.char_id))?;
            } else {
                store.map = change.map.clone();
                store.map_instance = change.map_instance;
                store.x = change.x;
                store.y = change.y;
                tx_write(systems, &key(b"vending/", id), &store)?;
                vending = Some(store);
            }
        }
        let mut buying = None;
        if let Some(id) = change.buying_id {
            let mut store: BuyingStore = tx_required(systems, &key(b"store/", id))?;
            if store.char_id != change.char_id || store.account_id != change.account_id {
                return abort("Buying store belongs to another character");
            }
            if change.close_buying {
                systems.remove(key(b"store/", id))?;
                systems.remove(key(b"buyer/", change.char_id))?;
            } else {
                store.map = change.map.clone();
                store.map_instance = change.map_instance;
                store.x = change.x;
                store.y = change.y;
                tx_write(systems, &key(b"store/", id), &store)?;
                buying = Some(store);
            }
        }
        Ok(StoreMoveResult { vending, buying })
    })?)
}

#[derive(Debug)]
pub struct ContainerTransfer {
    pub cart: Vec<InventoryRecord>,
    pub storage: Vec<InventoryRecord>,
    pub guild_storage: Vec<InventoryRecord>,
}

#[derive(Debug)]
pub struct VendingTrade {
    pub seller_id: u32,
    pub seller_zeny: u32,
    pub buyer_zeny: u32,
    pub cart: Vec<InventoryRecord>,
    pub store: Option<VendingStore>,
}

pub fn character_cart(repository: &SledRepository, char_id: u32) -> Result<Vec<InventoryRecord>, Error> {
    Ok(read(&repository.database.game_systems, &key(b"cart/", char_id))?.unwrap_or_default())
}

pub fn set_character_options(repository: &SledRepository, char_id: u32, expected: u64, options: u64) -> Result<(), Error> {
    let options = i32::try_from(options).map_err(|_| Error::new("Invalid character options".into()))?;
    repository.database.characters.transaction(|characters| {
        let mut character: CharacterRecord = tx_required(characters, &(char_id as i32).to_be_bytes())?;
        if character.option as u64 != expected {
            return abort("Character options changed");
        }
        character.option = options;
        tx_write(characters, &(char_id as i32).to_be_bytes(), &character)
    })?;
    Ok(())
}

fn cart_mask() -> u64 {
    PlayerOption::Cart1.as_flag()
        | PlayerOption::Cart2.as_flag()
        | PlayerOption::Cart3.as_flag()
        | PlayerOption::Cart4.as_flag()
        | PlayerOption::Cart5.as_flag()
}

fn matching_stack(a: &InventoryRecord, b: &InventoryRecord) -> bool {
    a.item_id == b.item_id
        && a.unique_id == 0
        && b.unique_id == 0
        && a.is_identified == b.is_identified
        && a.is_damaged == b.is_damaged
        && a.refine == b.refine
        && [a.card0, a.card1, a.card2, a.card3] == [b.card0, b.card1, b.card2, b.card3]
}

pub(super) fn trade_masks(
    items: &TransactionalTree,
    item: &ItemModel,
    record: &InventoryRecord,
) -> ConflictableTransactionResult<Vec<u64>, Error> {
    let mut masks = vec![item.trade_flags];
    if !models::item::special_card_metadata(record.card0) {
        for id in [record.card0, record.card1, record.card2, record.card3]
            .into_iter()
            .take(item.slots.unwrap_or(0).clamp(0, 4) as usize)
            .filter(|id| *id != 0)
        {
            let card: ItemModel = tx_required(items, &(id as u16 as i32).to_be_bytes())?;
            masks.push(card.trade_flags);
        }
    }
    Ok(masks)
}

fn transfer_record(
    source: &mut Vec<InventoryRecord>,
    destination: &mut Vec<InventoryRecord>,
    record_id: i32,
    amount: u32,
    owner: Option<u32>,
    destination_holder: u32,
    source_holder: u32,
    systems: &TransactionalTree,
    owners: &TransactionalTree,
    items: &TransactionalTree,
    metadata: &TransactionalTree,
) -> ConflictableTransactionResult<(), Error> {
    let index = source
        .iter()
        .position(|record| record.id == record_id)
        .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
    let record = source[index].clone();
    let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
    if amount == 0
        || amount > i16::MAX as u32
        || record.equip != 0
        || record.amount < amount as i16
        || (!item.item_type.is_stackable() && (record.amount != 1 || amount != 1))
    {
        return abort("Invalid container item quantity");
    }
    let whole = record.amount == amount as i16;
    let existing = if item.item_type.is_stackable() {
        destination.iter().position(|target| matching_stack(&record, target))
    } else {
        None
    };
    let moved_id = if let Some(index) = existing {
        destination[index].amount = destination[index]
            .amount
            .checked_add(amount as i16)
            .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Item stack overflow".into())))?;
        destination[index].id
    } else {
        if destination
            .iter()
            .any(|target| target.id == record.id || (record.unique_id != 0 && target.unique_id == record.unique_id))
        {
            return abort("Container item identity already exists at the destination");
        }
        let mut moved = record.clone();
        moved.amount = amount as i16;
        if !whole {
            moved.id = next_id(metadata, b"inventory_id", 0)?;
        }
        let id = moved.id;
        destination.push(moved);
        id
    };
    if item.item_type == ItemType::PetEgg && record.card0 == 256 {
        super::pet_custody::transfer_egg(systems, &record, moved_id, destination_holder, source_holder)?;
    }
    if whole {
        source.remove(index);
        owners.remove(record.id.to_be_bytes().to_vec())?;
    } else {
        source[index].amount -= amount as i16;
    }
    if let Some(owner) = owner {
        tx_write(owners, &moved_id.to_be_bytes(), &(owner as i32))?;
    }
    Ok(())
}

pub fn container_transfer(
    repository: &SledRepository,
    char_id: u32,
    record_id: i32,
    amount: u32,
    source: ItemContainer,
    destination: ItemContainer,
    max_weight: u32,
) -> Result<ContainerTransfer, Error> {
    if source == destination || amount == 0 || amount > i16::MAX as u32 {
        return Err(Error::new("Invalid item container transfer".into()));
    }
    let db = &repository.database;
    Ok((
        &db.game_systems,
        &db.characters,
        &db.inventories,
        &db.inventory_owners,
        &db.items,
        &db.metadata,
    )
        .transaction(|(systems, characters, inventories, owners, items, metadata)| {
            let character: CharacterRecord = tx_required(characters, &(char_id as i32).to_be_bytes())?;
            if (source == ItemContainer::Cart || destination == ItemContainer::Cart) && character.option as u64 & cart_mask() == 0 {
                return abort("Character has no cart");
            }
            if tx_read::<u32>(systems, &key(b"vender/", char_id))?.is_some() {
                return abort("Cannot move items while vending");
            }
            let storage_key = key(b"storage/", character.account_id as u32);
            let cart_key = key(b"cart/", char_id);
            let guild_storage_key = if source == ItemContainer::GuildStorage || destination == ItemContainer::GuildStorage {
                Some(key(b"guild_storage/", super::guild_storage::require_access(systems, char_id)?))
            } else {
                None
            };
            let mut guild_storage: Vec<InventoryRecord> = if let Some(key) = &guild_storage_key {
                tx_read(systems, key)?.unwrap_or_default()
            } else {
                Vec::new()
            };
            let mut storage: Vec<InventoryRecord> = tx_read(systems, &storage_key)?.unwrap_or_default();
            let mut cart: Vec<InventoryRecord> = tx_read(systems, &cart_key)?.unwrap_or_default();
            let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &(char_id as i32).to_be_bytes())?.unwrap_or_default();
            let (from, to) = match (source, destination) {
                (ItemContainer::Inventory, ItemContainer::Cart) => (&mut inventory, &mut cart),
                (ItemContainer::Inventory, ItemContainer::Storage) => (&mut inventory, &mut storage),
                (ItemContainer::Cart, ItemContainer::Inventory) => (&mut cart, &mut inventory),
                (ItemContainer::Cart, ItemContainer::Storage) => (&mut cart, &mut storage),
                (ItemContainer::Storage, ItemContainer::Inventory) => (&mut storage, &mut inventory),
                (ItemContainer::Storage, ItemContainer::Cart) => (&mut storage, &mut cart),
                (ItemContainer::Inventory, ItemContainer::GuildStorage) => (&mut inventory, &mut guild_storage),
                (ItemContainer::Cart, ItemContainer::GuildStorage) => (&mut cart, &mut guild_storage),
                (ItemContainer::GuildStorage, ItemContainer::Inventory) => (&mut guild_storage, &mut inventory),
                (ItemContainer::GuildStorage, ItemContainer::Cart) => (&mut guild_storage, &mut cart),
                _ => return abort("Invalid container transfer"),
            };
            let record = from
                .iter()
                .find(|item| item.id == record_id)
                .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
            let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
            if source == ItemContainer::Inventory && tx_required::<i32>(owners, &record.id.to_be_bytes())? != char_id as i32 {
                return abort("Container item ownership changed");
            }
            let masks = trade_masks(items, &item, record)?;
            if (destination == ItemContainer::Cart && masks.iter().any(|mask| mask & ItemTradeFlag::NoCart.as_flag() != 0))
                || (destination == ItemContainer::Storage && masks.iter().any(|mask| mask & ItemTradeFlag::NoStorage.as_flag() != 0))
                || (destination == ItemContainer::GuildStorage
                    && masks.iter().any(|mask| mask & ItemTradeFlag::NoGuildStorage.as_flag() != 0))
            {
                return abort("Item cannot be stored in this container");
            }
            transfer_record(
                from,
                to,
                record_id,
                amount,
                (destination == ItemContainer::Inventory).then_some(char_id),
                if matches!(destination, ItemContainer::Inventory | ItemContainer::Cart) {
                    char_id
                } else {
                    0
                },
                char_id,
                systems,
                owners,
                items,
                metadata,
            )?;
            if inventory.len() > character.inventory_slots as usize
                || storage.len() > 600
                || cart.len() > 100
                || guild_storage.len() > 600
                || (destination == ItemContainer::Inventory && inventory_weight(&inventory, items)? > u64::from(max_weight))
                || (destination == ItemContainer::Cart && inventory_weight(&cart, items)? > 80_000)
            {
                return abort("Container transfer exceeds slots or weight");
            }
            tx_write(inventories, &(char_id as i32).to_be_bytes(), &inventory)?;
            tx_write(systems, &cart_key, &cart)?;
            tx_write(systems, &storage_key, &storage)?;
            if let Some(key) = &guild_storage_key {
                tx_write(systems, key, &guild_storage)?;
            }
            Ok(ContainerTransfer {
                cart,
                storage,
                guild_storage,
            })
        })?)
}

pub fn create_vending_store(repository: &SledRepository, store: &VendingStore) -> Result<VendingStore, Error> {
    if store.title.trim().is_empty() || store.title.len() > 79 || store.offers.is_empty() || store.offers.len() > 12 {
        return Err(Error::new("Invalid vending store".into()));
    }
    let db = &repository.database;
    Ok(
        (&db.game_systems, &db.characters, &db.items, &db.metadata).transaction(|(systems, characters, items, metadata)| {
            let character: CharacterRecord = tx_required(characters, &(store.char_id as i32).to_be_bytes())?;
            if character.account_id as u32 != store.account_id || character.option as u64 & cart_mask() == 0 {
                return abort("Vending owner has no cart");
            }
            let cart: Vec<InventoryRecord> = tx_read(systems, &key(b"cart/", store.char_id))?.unwrap_or_default();
            let mut used = BTreeSet::new();
            let mut total = 0u64;
            for offer in &store.offers {
                let record = cart
                    .iter()
                    .find(|record| record.id == offer.inventory_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
                let masks = trade_masks(items, &item, record)?;
                if offer.amount == 0
                    || offer.amount > i16::MAX as u16
                    || offer.amount as i16 > record.amount
                    || !used.insert(offer.inventory_id)
                    || offer.price > 1_000_000_000
                    || !record.is_identified
                    || record.is_damaged
                    || record.equip != 0
                    || masks.iter().any(|mask| mask & ItemTradeFlag::NoTrade.as_flag() != 0)
                    || (!item.item_type.is_stackable() && offer.amount != 1)
                {
                    return abort("Invalid vending offer");
                }
                if item.item_type == ItemType::PetEgg && record.card0 == 256 {
                    super::pet_custody::incubating_pet(systems, record)?;
                }
                total = total
                    .checked_add(u64::from(offer.price) * u64::from(offer.amount))
                    .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Vending price overflow".into())))?;
            }
            if total + character.zeny.max(0) as u64 > i32::MAX as u64 {
                return abort("Vending proceeds would exceed maximum zeny");
            }
            if let Some(old) = tx_read::<u32>(systems, &key(b"vender/", store.char_id))? {
                systems.remove(key(b"vending/", old))?;
            }
            let mut store = store.clone();
            store.id = next_id(metadata, b"vending_store_id", 0)? as u32;
            tx_write(systems, &key(b"vending/", store.id), &store)?;
            tx_write(systems, &key(b"vender/", store.char_id), &store.id)?;
            Ok(store)
        })?,
    )
}

pub fn close_vending_store(repository: &SledRepository, char_id: u32, store_id: u32) -> Result<(), Error> {
    repository.database.game_systems.transaction(|systems| {
        if let Some(store) = tx_read::<VendingStore>(systems, &key(b"vending/", store_id))? {
            if store.char_id != char_id {
                return abort("Vending store belongs to another character");
            }
            systems.remove(key(b"vending/", store_id))?;
        }
        if tx_read::<u32>(systems, &key(b"vender/", char_id))? == Some(store_id) {
            systems.remove(key(b"vender/", char_id))?;
        }
        Ok(())
    })?;
    Ok(())
}

/// `tax` is the `vending_tax` rate and the `vending_tax_min` it applies from.
pub fn vending_store_trade(
    repository: &SledRepository,
    buyer_id: u32,
    store_id: u32,
    purchases: &[(u16, u16)],
    max_weight: u32,
    tax: (u32, u32),
) -> Result<VendingTrade, Error> {
    if purchases.is_empty() || purchases.len() > 12 {
        return Err(Error::new("Invalid vending purchase list".into()));
    }
    let db = &repository.database;
    Ok((
        &db.game_systems,
        &db.characters,
        &db.inventories,
        &db.inventory_owners,
        &db.items,
        &db.metadata,
    )
        .transaction(|(systems, characters, inventories, owners, items, metadata)| {
            let mut store: VendingStore = tx_required(systems, &key(b"vending/", store_id))?;
            if store.char_id == buyer_id {
                return abort("Cannot purchase from your own vending store");
            }
            let mut seller: CharacterRecord = tx_required(characters, &(store.char_id as i32).to_be_bytes())?;
            let mut buyer: CharacterRecord = tx_required(characters, &(buyer_id as i32).to_be_bytes())?;
            let mut cart: Vec<InventoryRecord> = tx_read(systems, &key(b"cart/", store.char_id))?.unwrap_or_default();
            let mut inventory: Vec<InventoryRecord> = tx_read(inventories, &(buyer_id as i32).to_be_bytes())?.unwrap_or_default();
            let mut used = BTreeSet::new();
            let mut total = 0u64;
            for (index, amount) in purchases {
                if !used.insert(*index) || *amount == 0 || *amount > i16::MAX as u16 {
                    return abort("Invalid or duplicate vending purchase");
                }
                let offer = store
                    .offers
                    .iter_mut()
                    .find(|offer| offer.index == *index)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                if offer.amount < *amount {
                    return abort("Vending offer is out of stock");
                }
                let source = cart
                    .iter()
                    .find(|record| record.id == offer.inventory_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                let item: ItemModel = tx_required(items, &source.item_id.to_be_bytes())?;
                let masks = trade_masks(items, &item, source)?;
                if masks.iter().any(|mask| mask & ItemTradeFlag::NoTrade.as_flag() != 0) || !source.is_identified || source.is_damaged {
                    return abort("Vending item can no longer be traded");
                }
                total = total
                    .checked_add(u64::from(offer.price) * u64::from(*amount))
                    .ok_or_else(|| sled::transaction::ConflictableTransactionError::Abort(Error::new("Vending total overflow".into())))?;
                transfer_record(
                    &mut cart,
                    &mut inventory,
                    offer.inventory_id,
                    u32::from(*amount),
                    Some(buyer_id),
                    buyer_id,
                    store.char_id,
                    systems,
                    owners,
                    items,
                    metadata,
                )?;
                offer.amount -= amount;
            }
            if buyer.zeny < 0
                || total > buyer.zeny as u64
                || total > MAX_ZENY
                || total + seller.zeny.max(0) as u64 > MAX_ZENY
                || inventory.len() > buyer.inventory_slots as usize
                || inventory_weight(&inventory, items)? >= u64::from(max_weight)
            {
                return abort("Vending purchase exceeds zeny, slots or weight");
            }
            buyer.zeny -= total as i32;
            let (rate, minimum) = tax;
            let kept = if rate > 0 && total >= u64::from(minimum) { total * u64::from(rate) / 10_000 } else { 0 };
            seller.zeny += (total - kept) as i32;
            tx_write(characters, &(buyer_id as i32).to_be_bytes(), &buyer)?;
            tx_write(characters, &(store.char_id as i32).to_be_bytes(), &seller)?;
            tx_write(inventories, &(buyer_id as i32).to_be_bytes(), &inventory)?;
            tx_write(systems, &key(b"cart/", store.char_id), &cart)?;
            store.offers.retain(|offer| offer.amount > 0);
            let seller_id = store.char_id;
            let store = if store.offers.is_empty() {
                systems.remove(key(b"vending/", store.id))?;
                systems.remove(key(b"vender/", store.char_id))?;
                None
            } else {
                tx_write(systems, &key(b"vending/", store.id), &store)?;
                Some(store)
            };
            Ok(VendingTrade {
                seller_id,
                seller_zeny: seller.zeny as u32,
                buyer_zeny: buyer.zeny as u32,
                cart,
                store,
            })
        })?)
}
