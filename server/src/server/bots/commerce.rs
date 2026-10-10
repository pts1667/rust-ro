//! Carts, vending stores and buying stores: the packets a client sends for them, run by the game loop so that the answer is the outcome.
use database::model::InventoryRecord;
use models::enums::EnumWithMaskValueU64;
use models::enums::item::{ItemFlag, ItemTradeFlag};
use serde::Serialize;
use serde_json::{Value, json};

use super::command::{distance, require_active};
use super::interaction::check_amount;
use crate::server::Server;
use crate::server::model::game_systems::{BuyingOffer, ItemContainer, StoreRequest, VendingOffer};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_character_service::learned_level;
use crate::server::service::script_world_service::has_cart;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// Cells from a store at which its goods can be bought, as `script_world_vending` requires.
const STORE_REACH: u16 = 5;
const MAX_PRICE: u32 = 1_000_000_000;
/// What a buying store asks per item and pays per item.
const MAX_WANTED_AMOUNT: u16 = 9999;
const MAX_WANTED_PRICE: u32 = 99_990_000;
const MAX_TITLE_BYTES: usize = 79;
/// Merchant skill Vending, whose level sets how many different items a store sells (level + 2).
const VENDING_SKILL: u32 = 41;
const VENDING_EXTRA_SLOTS: u8 = 2;

/// Moves between the inventory, the cart and the open storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartMove {
    /// Inventory to cart.
    Put,
    /// Cart to inventory.
    Get,
    /// Cart to the open storage.
    ToStorage,
    /// Open storage to the cart.
    FromStorage,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VendingLine {
    pub cart_index: u16,
    pub amount: u16,
    pub price: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyingLine {
    pub item_id: i32,
    pub amount: u16,
    pub price: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoreAction {
    CartMove { how: CartMove, index: usize, amount: u32 },
    OpenVending { title: String, lines: Vec<VendingLine> },
    CloseVending,
    /// `(index of the offer, amount)` of the goods of a vendor.
    BuyFromVendor { vendor: u32, lines: Vec<(u16, u16)> },
    OpenBuying { title: String, zeny_limit: u32, lines: Vec<BuyingLine> },
    CloseBuying,
    /// `(inventory index, amount)` of the items given to a buyer.
    SellToBuyer { buyer: u32, lines: Vec<(usize, u16)> },
}

#[derive(Debug, Serialize)]
pub struct CartItemView {
    pub index: usize,
    pub item_id: i32,
    pub name: String,
    pub amount: i16,
    pub refine: i16,
}

/// A good a vendor sells, `index` is what `vending_buy` names.
#[derive(Debug, Serialize)]
pub struct SaleView {
    pub index: u16,
    pub item_id: i32,
    pub name: String,
    pub amount: u16,
    pub price: u32,
    pub refine: i16,
}

#[derive(Debug, Serialize)]
pub struct VendingView {
    pub title: String,
    pub items: Vec<SaleView>,
}

#[derive(Debug, Serialize)]
pub struct VendorView {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub x: u16,
    pub y: u16,
    /// Cells from the bot, `vending_buy` needs 5 or less.
    pub distance: u16,
    pub items: Vec<SaleView>,
}

#[derive(Debug, Serialize)]
pub struct WantedView {
    pub item_id: i32,
    pub name: String,
    pub amount: u16,
    pub price: u32,
}

#[derive(Debug, Serialize)]
pub struct BuyingView {
    pub title: String,
    pub zeny_limit: u32,
    pub items: Vec<WantedView>,
}

#[derive(Debug, Serialize)]
pub struct BuyerView {
    pub id: u32,
    pub name: String,
    pub title: String,
    pub x: u16,
    pub y: u16,
    pub distance: u16,
    pub zeny_limit: u32,
    pub items: Vec<WantedView>,
}

/// Everything about carts and stores in the observation.
#[derive(Debug, Serialize)]
pub struct CommerceView {
    /// The items of the cart, `null` without a cart.
    pub cart: Option<Vec<CartItemView>>,
    /// The store of the bot, `null` while it has none.
    pub vending: Option<VendingView>,
    pub buying: Option<BuyingView>,
    /// Slots a Purchase Street Stall License opened for `buying_open`, 0 when none is prepared.
    pub buying_slots: u8,
    pub vendors: Vec<VendorView>,
    pub buyers: Vec<BuyerView>,
}

fn item_name(item_id: i32) -> String {
    GlobalConfigService::item_name_if_loaded(item_id).unwrap_or_else(|| item_id.to_string())
}

fn sales(owner: &Character, offers: &[VendingOffer]) -> Vec<SaleView> {
    offers
        .iter()
        .filter_map(|offer| {
            let record = owner.game_systems.cart_items.iter().find(|record| record.id == offer.inventory_id)?;
            Some(SaleView { index: offer.index, item_id: record.item_id, name: item_name(record.item_id), amount: offer.amount, price: offer.price, refine: record.refine })
        })
        .collect()
}

fn wanted(offers: &[BuyingOffer]) -> Vec<WantedView> {
    offers.iter().map(|offer| WantedView { item_id: offer.item_id, name: item_name(offer.item_id), amount: offer.amount, price: offer.price }).collect()
}

pub fn commerce_view(state: &ServerState, character: &Character) -> CommerceView {
    let systems = &character.game_systems;
    let here = (character.x(), character.y());
    let others = || state.characters().values().filter(|other| other.char_id != character.char_id && other.map_instance_key == character.map_instance_key);
    CommerceView {
        cart: has_cart(character).then(|| cart_items(&systems.cart_items)),
        vending: systems.vending_store.as_ref().map(|store| VendingView { title: store.title.clone(), items: sales(character, &store.offers) }),
        buying: systems.buying_store.as_ref().map(|store| BuyingView { title: store.title.clone(), zeny_limit: store.zeny_limit, items: wanted(&store.offers) }),
        buying_slots: systems.buying_slots,
        vendors: others()
            .filter_map(|owner| {
                let store = owner.game_systems.vending_store.as_ref()?;
                Some(VendorView {
                    id: owner.char_id,
                    name: owner.name.clone(),
                    title: store.title.clone(),
                    x: owner.x(),
                    y: owner.y(),
                    distance: distance(here, (owner.x(), owner.y())),
                    items: sales(owner, &store.offers),
                })
            })
            .collect(),
        buyers: others()
            .filter_map(|owner| {
                let store = owner.game_systems.buying_store.as_ref()?;
                Some(BuyerView {
                    id: owner.char_id,
                    name: owner.name.clone(),
                    title: store.title.clone(),
                    x: owner.x(),
                    y: owner.y(),
                    distance: distance(here, (owner.x(), owner.y())),
                    zeny_limit: store.zeny_limit,
                    items: wanted(&store.offers),
                })
            })
            .collect(),
    }
}

fn cart_items(records: &[InventoryRecord]) -> Vec<CartItemView> {
    records
        .iter()
        .enumerate()
        .map(|(index, record)| CartItemView { index, item_id: record.item_id, name: item_name(record.item_id), amount: record.amount, refine: record.refine })
        .collect()
}

fn store_title(title: &str) -> Result<String, String> {
    let title = title.trim();
    if title.is_empty() || title.len() > MAX_TITLE_BYTES || title.chars().any(char::is_control) {
        return Err(format!("A title has between 1 and {MAX_TITLE_BYTES} characters and no control characters"));
    }
    Ok(title.to_string())
}

fn check_price(price: u32, highest: u32) -> Result<(), String> {
    if price == 0 || price > highest {
        return Err(format!("A price goes from 1 to {highest} zeny"));
    }
    Ok(())
}

fn check_line_amount(amount: u16, available: u16) -> Result<(), String> {
    if amount == 0 || amount > available {
        return Err(format!("The amount goes from 1 to {available}"));
    }
    Ok(())
}

fn require_free_to_trade(character: &Character) -> Result<(), String> {
    require_active(character)?;
    if character.game_systems.is_trading() {
        return Err("Finish the trade first".into());
    }
    Ok(())
}

impl Server {
    pub(crate) fn bot_store(&self, state: &mut ServerState, char_id: u32, action: &StoreAction, tick: u128) -> Result<Value, String> {
        let now = u64::try_from(tick).unwrap_or(u64::MAX);
        state
            .with_character_taken(char_id, |state, character| match action {
                StoreAction::CartMove { how, index, amount } => self.move_cart_items(state, character, *how, *index, *amount),
                StoreAction::OpenVending { title, lines } => self.open_vending_store(state, character, title, lines, now),
                StoreAction::CloseVending => {
                    if character.game_systems.vending_store.is_none() {
                        return Err("No vending store is open".to_string());
                    }
                    self.script_world_service().close_vending(character)?;
                    Ok(json!({ "closed": true }))
                }
                StoreAction::BuyFromVendor { vendor, lines } => self.buy_from_vendor(state, character, *vendor, lines, now),
                StoreAction::OpenBuying { title, zeny_limit, lines } => self.open_buying_store(state, character, title, *zeny_limit, lines, now),
                StoreAction::CloseBuying => {
                    if character.game_systems.buying_store.is_none() {
                        return Err("No buying store is open".to_string());
                    }
                    self.script_world_service().close_buying(character)?;
                    Ok(json!({ "closed": true }))
                }
                StoreAction::SellToBuyer { buyer, lines } => self.sell_to_buyer(state, character, *buyer, lines, now),
            })
            .ok_or("Character is not in game")?
    }

    fn move_cart_items(&self, state: &mut ServerState, character: &mut Character, how: CartMove, index: usize, amount: u32) -> Result<Value, String> {
        require_free_to_trade(character)?;
        if !has_cart(character) {
            return Err("The character has no cart: rent one from a Kafra".into());
        }
        let (source, destination, name) = match how {
            CartMove::Put => {
                let item = character.get_item_from_inventory(index).ok_or_else(|| format!("No item at inventory index {index}"))?;
                if item.equip != 0 {
                    return Err("Take the item off before putting it in the cart".into());
                }
                check_amount(amount, item.amount)?;
                (ItemContainer::Inventory, ItemContainer::Cart, item.name_english.clone())
            }
            CartMove::Get | CartMove::ToStorage => {
                let record = character.game_systems.cart_items.get(index).ok_or_else(|| format!("No item at cart index {index}"))?;
                check_amount(amount, record.amount)?;
                let destination = if how == CartMove::Get { ItemContainer::Inventory } else { ItemContainer::Storage };
                (ItemContainer::Cart, destination, item_name(record.item_id))
            }
            CartMove::FromStorage => {
                let systems = &character.game_systems;
                if !systems.storage_open {
                    return Err("No storage is open: talk to a Kafra".into());
                }
                let record = systems.storage_items.get(index).ok_or_else(|| format!("No item at storage index {index}"))?;
                check_amount(amount, record.amount)?;
                (ItemContainer::Storage, ItemContainer::Cart, item_name(record.item_id))
            }
        };
        if destination == ItemContainer::Storage && !character.game_systems.storage_open {
            return Err("No storage is open: talk to a Kafra".into());
        }
        let wire_index = u16::try_from(index).map_err(|_| "Invalid index")?;
        self.script_world_service().move_container(self, state, character, source, destination, wire_index, amount)?;
        Ok(json!({ "item": name, "amount": amount }))
    }

    fn open_vending_store(&self, state: &mut ServerState, character: &mut Character, title: &str, lines: &[VendingLine], now: u64) -> Result<Value, String> {
        require_free_to_trade(character)?;
        let title = store_title(title)?;
        if !has_cart(character) {
            return Err("Vending needs a cart: rent one from a Kafra".into());
        }
        if character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some() {
            return Err("A store is already open: close it first".into());
        }
        let level = learned_level(&character.status, VENDING_SKILL);
        if level == 0 {
            return Err("Vending (MC_VENDING) is not learned".into());
        }
        let slots = usize::from(level + VENDING_EXTRA_SLOTS);
        if lines.is_empty() || lines.len() > slots {
            return Err(format!("Vending level {level} sells 1 to {slots} different items"));
        }
        for (position, line) in lines.iter().enumerate() {
            let record = character.game_systems.cart_items.get(usize::from(line.cart_index)).ok_or_else(|| format!("No item at cart index {}", line.cart_index))?;
            check_line_amount(line.amount, u16::try_from(record.amount).unwrap_or(0))?;
            check_price(line.price, MAX_PRICE)?;
            if !record.is_identified || record.is_damaged {
                return Err(format!("{} is not identified or is broken, it can not be sold", item_name(record.item_id)));
            }
            if lines[..position].iter().any(|earlier| earlier.cart_index == line.cart_index) {
                return Err(format!("Cart index {} is listed twice", line.cart_index));
            }
        }
        let world = self.script_world_service();
        world.store_request(self, state, character, StoreRequest::PrepareVending { skill_level: level }, now)?;
        let offers = lines.iter().map(|line| (line.cart_index, line.amount, line.price)).collect();
        if let Err(error) = world.store_request(self, state, character, StoreRequest::CreateVendingStore { title: title.clone(), offers }, now) {
            character.game_systems.vending_slots = 0;
            return Err(error);
        }
        Ok(json!({ "title": title, "items": lines.len() }))
    }

    fn buy_from_vendor(&self, state: &mut ServerState, character: &mut Character, vendor: u32, lines: &[(u16, u16)], now: u64) -> Result<Value, String> {
        require_free_to_trade(character)?;
        let owner = state
            .get_character(vendor)
            .filter(|owner| owner.char_id != character.char_id && owner.map_instance_key == character.map_instance_key)
            .ok_or_else(|| format!("No player {vendor} on this map"))?;
        let store = owner.game_systems.vending_store.as_ref().ok_or_else(|| format!("{} has no vending store open", owner.name))?;
        let apart = distance((character.x(), character.y()), (owner.x(), owner.y()));
        if apart > STORE_REACH {
            return Err(format!("The store is {apart} cells away, buying needs {STORE_REACH} or less: move closer"));
        }
        if lines.is_empty() {
            return Err("Give at least one line in items".into());
        }
        let mut cost: u64 = 0;
        for (position, (index, amount)) in lines.iter().enumerate() {
            let offer = store.offers.iter().find(|offer| offer.index == *index).ok_or_else(|| format!("The store does not sell index {index}"))?;
            check_line_amount(*amount, offer.amount)?;
            if lines[..position].iter().any(|(earlier, _)| earlier == index) {
                return Err(format!("Index {index} is listed twice"));
            }
            cost += u64::from(offer.price) * u64::from(*amount);
        }
        if cost > u64::from(character.status.zeny) {
            return Err(format!("The purchase costs {cost} zeny, the character has {}", character.status.zeny));
        }
        let zeny_before = character.status.zeny;
        let world = self.script_world_service();
        world.store_request(self, state, character, StoreRequest::OpenVendingStore(vendor), now)?;
        world.store_request(self, state, character, StoreRequest::PurchaseVendingStore { account_id: vendor, store_id: None, items: lines.to_vec() }, now)?;
        Ok(json!({ "spent": zeny_before.saturating_sub(character.status.zeny), "zeny": character.status.zeny }))
    }

    fn open_buying_store(&self, state: &mut ServerState, character: &mut Character, title: &str, zeny_limit: u32, lines: &[BuyingLine], now: u64) -> Result<Value, String> {
        require_free_to_trade(character)?;
        let title = store_title(title)?;
        let slots = usize::from(character.game_systems.buying_slots);
        if slots == 0 {
            return Err("No buying store is prepared: use a Purchase Street Stall License first".into());
        }
        if lines.is_empty() || lines.len() > slots {
            return Err(format!("The license allows 1 to {slots} different items"));
        }
        if zeny_limit == 0 || zeny_limit > character.status.zeny {
            return Err(format!("The zeny limit goes from 1 to the {} zeny of the character", character.status.zeny));
        }
        let config = GlobalConfigService::instance();
        for (position, line) in lines.iter().enumerate() {
            let model = config.find_item(line.item_id).ok_or_else(|| format!("Unknown item {}", line.item_id))?;
            let allowed = model.item_type.is_stackable()
                && model.flags & ItemFlag::BuyingStore.as_flag() != 0
                && model.trade_flags & ItemTradeFlag::NoTrade.as_flag() == 0;
            if !allowed {
                return Err(format!("{} can not be asked for by a buying store", model.name_english));
            }
            if !character.inventory.iter().flatten().any(|item| item.item_id == line.item_id && item.amount > 0) {
                return Err(format!("A buying store asks for items the character already holds, it has no {}", model.name_english));
            }
            check_line_amount(line.amount, MAX_WANTED_AMOUNT)?;
            check_price(line.price, MAX_WANTED_PRICE)?;
            if lines[..position].iter().any(|earlier| earlier.item_id == line.item_id) {
                return Err(format!("Item {} is listed twice", line.item_id));
            }
        }
        let offers = lines.iter().map(|line| BuyingOffer { item_id: line.item_id, amount: line.amount, price: line.price }).collect();
        self.script_world_service().store_request(self, state, character, StoreRequest::CreateBuyingStore { title: title.clone(), zeny_limit, offers }, now)?;
        Ok(json!({ "title": title, "items": lines.len() }))
    }

    fn sell_to_buyer(&self, state: &mut ServerState, character: &mut Character, buyer: u32, lines: &[(usize, u16)], now: u64) -> Result<Value, String> {
        require_free_to_trade(character)?;
        let owner = state
            .get_character(buyer)
            .filter(|owner| owner.char_id != character.char_id && owner.map_instance_key == character.map_instance_key)
            .ok_or_else(|| format!("No player {buyer} on this map"))?;
        let store = owner.game_systems.buying_store.as_ref().ok_or_else(|| format!("{} has no buying store open", owner.name))?;
        let apart = distance((character.x(), character.y()), (owner.x(), owner.y()));
        if apart > STORE_REACH {
            return Err(format!("The store is {apart} cells away, selling needs {STORE_REACH} or less: move closer"));
        }
        if lines.is_empty() {
            return Err("Give at least one line in items".into());
        }
        let mut items = Vec::new();
        let mut earned: u64 = 0;
        for (position, (index, amount)) in lines.iter().enumerate() {
            let item = character.get_item_from_inventory(*index).ok_or_else(|| format!("No item at inventory index {index}"))?;
            if item.equip != 0 {
                return Err("Take the item off before selling it".into());
            }
            let wanted = store.offers.iter().find(|offer| offer.item_id == item.item_id).ok_or_else(|| format!("{} does not buy {}", owner.name, item.name_english))?;
            check_line_amount(*amount, u16::try_from(item.amount).unwrap_or(0).min(wanted.amount))?;
            if lines[..position].iter().any(|(earlier, _)| earlier == index) {
                return Err(format!("Inventory index {index} is listed twice"));
            }
            earned += u64::from(wanted.price) * u64::from(*amount);
            items.push((u16::try_from(*index).map_err(|_| "Invalid index")?, item.item_id, *amount));
        }
        if earned > u64::from(store.zeny_limit) {
            return Err(format!("{} spends at most {} zeny, this sale is worth {earned}", owner.name, store.zeny_limit));
        }
        let (store_id, zeny_before) = (store.id, character.status.zeny);
        self.script_world_service().store_request(self, state, character, StoreRequest::TradeBuyingStore { account_id: buyer, store_id, items }, now)?;
        Ok(json!({ "earned": character.status.zeny.saturating_sub(zeny_before), "zeny": character.status.zeny }))
    }
}
