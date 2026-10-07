use std::collections::BTreeSet;

use models::enums::cell::CellType;
use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};

use super::{ScriptWorldService, protocol};
use crate::server::Server;
use crate::server::model::game_systems::{BuyingOffer, BuyingSale, BuyingStore, ItemContainer, PlayerOption, StoreSearchResult, VendingOffer, VendingStore};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::permission_groups::Permission;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub fn has_cart(character: &Character) -> bool {
    character.options & cart_mask() != 0
}

fn cart_mask() -> u64 {
    PlayerOption::Cart1.as_flag()
        | PlayerOption::Cart2.as_flag()
        | PlayerOption::Cart3.as_flag()
        | PlayerOption::Cart4.as_flag()
        | PlayerOption::Cart5.as_flag()
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoreRequest {
    CreateBuyingStore {
            title: String,
            zeny_limit: u32,
            offers: Vec<BuyingOffer>,
        },
    CloseBuyingStore,
    OpenBuyingStore(u32),
    TradeBuyingStore {
            account_id: u32,
            store_id: u32,
            items: Vec<(u16, i32, u16)>,
        },
    SearchStores {
            kind: u8,
            min_price: u32,
            max_price: u32,
            items: Vec<i32>,
            cards: Vec<u16>,
        },
    NextSearchPage,
    CloseStoreSearch,
    LocateStore {
            account_id: u32,
            store_id: u32,
            item_id: i32,
        },
    PrepareVending {
            skill_level: u8,
        },
    CreateVendingStore {
            title: String,
            offers: Vec<(u16, u16, u32)>,
        },
    CloseVendingStore,
    OpenVendingStore(u32),
    PurchaseVendingStore {
            account_id: u32,
            store_id: Option<u32>,
            items: Vec<(u16, u16)>,
        },
}


impl ScriptWorldService {
    pub(crate) fn prepare_store_map_move(
        &self,
        character: &mut Character,
        destination: &crate::server::model::map_instance::MapInstanceKey,
        position: movement::position::Position,
        flags: &crate::server::model::map_flags::MapFlags,
    ) -> Result<(), String> {
        self.close_storage(character)?;
        let vending_id = character.game_systems.vending_store.as_ref().map(|store| store.id);
        let buying_id = character.game_systems.buying_store.as_ref().map(|store| store.id);
        if vending_id.is_some() || buying_id.is_some() {
            let result = self
                .repository
                .move_character_stores(&crate::repository::game_system_repository::StoreMove {
                    char_id: character.char_id,
                    account_id: character.account_id,
                    vending_id,
                    buying_id,
                    map: destination.map_without_ext(),
                    map_instance: destination.map_instance(),
                    x: position.x,
                    y: position.y,
                    close_vending: flags.enabled(MapFlag::NoVending),
                    close_buying: flags.enabled(MapFlag::NoBuyingStore),
                })
                .map_err(|error| error.to_string())?;
            character.game_systems.vending_store = result.vending;
            character.game_systems.buying_store = result.buying;
            let mut disappearance = Vec::new();
            if vending_id.is_some() {
                disappearance.extend(protocol::vending_disappear(character.char_id));
            }
            if buying_id.is_some() {
                disappearance.extend(protocol::store_disappear(character.char_id));
            }
            // A committed store relocation must finish even if its notification cannot be
            // queued.
            if let Err(error) = self.area(character, disappearance) {
                log::warn!("Store movement notification failed for {}: {error}", character.char_id);
            }
        }
        character.game_systems.vending_slots = 0;
        character.game_systems.buying_slots = 0;
        character.game_systems.opened_vending_store = None;
        character.game_systems.remote_vending_store = None;
        character.game_systems.remote_store = None;
        Ok(())
    }

    pub(crate) fn close_buying(&self, character: &mut Character) -> Result<(), String> {
        if let Some(store) = &character.game_systems.buying_store {
            self.repository
                .close_buying_store(character.char_id, store.id)
                .map_err(|error| error.to_string())?;
            character.game_systems.buying_store = None;
            self.area(character, protocol::store_disappear(character.char_id))?;
        }
        character.game_systems.buying_slots = 0;
        Ok(())
    }

    pub fn initialize_cart(&self, character: &mut Character) -> Result<(), String> {
        if character.game_systems.vending_store.is_none() && character.game_systems.buying_store.is_none() {
            self.repository
                .clear_character_stores(character.char_id)
                .map_err(|error| error.to_string())?;
        }
        character.game_systems.cart_items = self
            .repository
            .character_cart(character.char_id)
            .map_err(|error| error.to_string())?;
        if has_cart(character) {
            self.send_cart(character)?;
        }
        Ok(())
    }

    pub(crate) fn set_cart(&self, character: &mut Character, style: u8, changing: bool) -> Result<(), String> {
        if character.game_systems.vending_store.is_some() {
            return Err("Cannot change a cart while vending".into());
        }
        if style > 5 {
            return Err("Unknown classic cart style".into());
        }
        let required_skill = if changing { 154 } else { 39 };
        if style > 0
            && !character
                .status
                .known_skills
                .iter()
                .any(|skill| skill.value.id() == required_skill && skill.level > 0)
        {
            return Err("Required cart skill is not learned".into());
        }
        if changing && !has_cart(character) {
            return Err("Character has no cart".into());
        }
        let minimum = [0, 0, 40, 65, 80, 90][usize::from(style)];
        if style > 0 && character.status.base_level <= minimum {
            return Err("Base level is too low for this cart style".into());
        }
        let flag = match style {
            0 => 0,
            1 => PlayerOption::Cart1.as_flag(),
            2 => PlayerOption::Cart2.as_flag(),
            3 => PlayerOption::Cart3.as_flag(),
            4 => PlayerOption::Cart4.as_flag(),
            _ => PlayerOption::Cart5.as_flag(),
        };
        let options = character.options & !cart_mask() | flag;
        self.repository
            .set_character_options(character.char_id, character.options, options)
            .map_err(|error| error.to_string())?;
        character.set_options(options);
        if style == 0 {
            self.send(character.char_id, protocol::header(0x012B))?;
        } else {
            self.initialize_cart(character)?;
        }
        self.send_options(character)
    }

    pub(crate) fn set_mount_option(&self, character: &mut Character, mount: PlayerOption, enable: bool) -> Result<(), String> {
        let (skill_id, flag) = match mount {
            PlayerOption::Falcon => (127, PlayerOption::Falcon.as_flag()),
            _ => (63, PlayerOption::Riding.as_flag()),
        };
        if enable
            && (character.options & PlayerOption::Madogear.as_flag() != 0
                || !character
                    .status
                    .known_skills
                    .iter()
                    .any(|skill| skill.value.id() == skill_id && skill.level > 0))
        {
            return Ok(());
        }
        let options = if enable { character.options | flag } else { character.options & !flag };
        if options == character.options {
            return Ok(());
        }
        self.repository
            .set_character_options(character.char_id, character.options, options)
            .map_err(|error| error.to_string())?;
        character.set_options(options);
        self.send_options(character)
    }

    pub(crate) fn remove_option(&self, character: &mut Character) -> Result<(), String> {
        let riding = PlayerOption::Riding.as_flag() | PlayerOption::Falcon.as_flag() | PlayerOption::Madogear.as_flag();
        if character.options & riding == 0 {
            return self.set_cart(character, 0, false);
        }
        let options = character.options & !riding;
        self.repository
            .set_character_options(character.char_id, character.options, options)
            .map_err(|error| error.to_string())?;
        character.set_options(options);
        self.send_options(character)
    }

    fn send_options(&self, character: &Character) -> Result<(), String> {
        let mut packet = protocol::header(0x0229);
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&[0; 4]);
        packet.extend_from_slice(&(character.options as u32).to_le_bytes());
        packet.push(0);
        self.area(character, packet)
    }

    pub(crate) fn move_container(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        source: ItemContainer,
        destination: ItemContainer,
        index: u16,
        amount: u32,
    ) -> Result<(), String> {
        let source = if source == ItemContainer::Storage && character.game_systems.guild_storage_open.is_some() {
            ItemContainer::GuildStorage
        } else {
            source
        };
        let destination = if destination == ItemContainer::Storage && character.game_systems.guild_storage_open.is_some() {
            ItemContainer::GuildStorage
        } else {
            destination
        };
        if (source == ItemContainer::Cart || destination == ItemContainer::Cart)
            && state.map_flags(&character.map_instance_key).enabled(MapFlag::NoUseCart)
        {
            return Err("Cart transfers are disabled on this map".into());
        }
        if (source == ItemContainer::Storage || destination == ItemContainer::Storage) && !character.game_systems.storage_open {
            return Err("Storage window is not open".into());
        }
        if (source == ItemContainer::GuildStorage || destination == ItemContainer::GuildStorage)
            && character.game_systems.guild_storage_open.is_none()
        {
            return Err("Guild storage is not open".into());
        }
        if (source == ItemContainer::Cart || destination == ItemContainer::Cart) && !has_cart(character) {
            return Err("Character has no cart".into());
        }
        if character.game_systems.vending_store.is_some() || character.game_systems.buying_store.is_some() {
            return Err("Cannot move container items while a store is open".into());
        }
        let record_id = match source {
            ItemContainer::Inventory => character.get_item_from_inventory(index as usize).map(|record| record.id),
            ItemContainer::Cart => character.game_systems.cart_items.get(index as usize).map(|record| record.id),
            ItemContainer::Storage => character.game_systems.storage_items.get(index as usize).map(|record| record.id),
            ItemContainer::GuildStorage => character
                .game_systems
                .guild_storage_items
                .get(index as usize)
                .map(|record| record.id),
        }
        .ok_or("Unknown container item index")?;
        let transfer = self
            .repository
            .container_transfer(
                character.char_id,
                record_id,
                amount,
                source,
                destination,
                server.character_service().max_weight(character) * 9 / 10,
            )
            .map_err(|error| error.to_string())?;
        character.game_systems.cart_items = transfer.cart;
        if character.game_systems.storage_open {
            character.game_systems.storage_items = transfer.storage;
        }
        if character.game_systems.guild_storage_open.is_some() {
            character.game_systems.guild_storage_items = transfer.guild_storage;
        }
        if source == ItemContainer::Inventory || destination == ItemContainer::Inventory {
            server
                .inventory_service()
                .reload_inventory(server.runtime(), character.char_id, character);
        }
        if source == ItemContainer::Cart || destination == ItemContainer::Cart {
            self.send_cart(character)?;
        }
        if matches!(source, ItemContainer::Storage | ItemContainer::GuildStorage)
            || matches!(destination, ItemContainer::Storage | ItemContainer::GuildStorage)
        {
            self.send_storage(character)?;
        }
        Ok(())
    }

    pub(crate) fn prepare_vending(&self, state: &ServerState, character: &mut Character, skill_level: u8) -> Result<(), String> {
        if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoVending)
            || state.cell_has(&character.map_instance_key, character.x(), character.y(), CellType::NoVending)
        {
            return Err("Vending is disabled on this map".into());
        }
        let level = character
            .status
            .known_skills
            .iter()
            .find(|skill| skill.value.id() == 41)
            .map(|skill| skill.level)
            .unwrap_or(0);
        if !has_cart(character)
            || skill_level == 0
            || skill_level > 10
            || u32::from(skill_level) > level as u32
            || character.status.hp == 0
            || character.game_systems.vending_store.is_some()
            || character.game_systems.buying_store.is_some()
            || character.game_systems.storage_open
            || character.game_systems.guild_storage_open.is_some()
        {
            return Err("Vending cannot be prepared in the current state".into());
        }
        character.game_systems.vending_slots = skill_level + 2;
        let mut packet = protocol::header(0x012D);
        packet.extend_from_slice(&u16::from(skill_level + 2).to_le_bytes());
        self.send(character.char_id, packet)
    }

    pub(crate) fn create_vending(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        title: String,
        offers: Vec<(u16, u16, u32)>,
    ) -> Result<(), String> {
        if !state.has_permission(character.account_id, Permission::Trade) {
            return Err("Your group is not allowed to sell items".into());
        }
        if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoVending)
            || state.cell_has(&character.map_instance_key, character.x(), character.y(), CellType::NoVending)
        {
            return Err("Vending is disabled on this map".into());
        }
        if character.game_systems.vending_slots == 0
            || offers.is_empty()
            || offers.len() > usize::from(character.game_systems.vending_slots)
            || character.status.hp == 0
            || character.game_systems.buying_store.is_some()
            || character.game_systems.vending_store.is_some()
            || character.game_systems.storage_open
            || character.game_systems.guild_storage_open.is_some()
        {
            return Err("Vending preparation window is not open".into());
        }
        let game = &self.configuration.config().game;
        let offers = offers
            .into_iter()
            .map(|(index, amount, price)| {
                let record = character
                    .game_systems
                    .cart_items
                    .get(index as usize)
                    .ok_or("Unknown vending cart index")?;
                Ok(VendingOffer {
                    index,
                    inventory_id: record.id,
                    amount,
                    price: price.min(game.vending_max_value),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let total: u64 = offers.iter().map(|offer| u64::from(offer.price) * u64::from(offer.amount)).sum();
        if !game.vending_over_max && u64::from(character.status.zeny) + total > crate::repository::MAX_ZENY {
            return Err("The total price of the store would take its owner over the zeny limit".into());
        }
        let store = VendingStore {
            id: 0,
            char_id: character.char_id,
            account_id: character.account_id,
            title,
            map: character.map_instance_key.map_without_ext(),
            map_instance: character.current_map_instance(),
            x: character.x,
            y: character.y,
            offers,
        };
        let store = self.repository.create_vending_store(&store).map_err(|error| error.to_string())?;
        character.game_systems.vending_slots = 0;
        character.game_systems.vending_store = Some(store.clone());
        self.stop_for_store(server, character)?;
        self.send(
            character.char_id,
            protocol::vending_items(
                &store,
                &character.game_systems.cart_items,
                true,
                self.configuration,
                server.packetver(),
            ),
        )?;
        self.area(character, protocol::vending_sign(&store))
    }

    pub(crate) fn close_vending(&self, character: &mut Character) -> Result<(), String> {
        if let Some(store) = &character.game_systems.vending_store {
            self.repository
                .close_vending_store(character.char_id, store.id)
                .map_err(|error| error.to_string())?;
            character.game_systems.vending_store = None;
            self.area(character, protocol::vending_disappear(character.char_id))?;
        }
        character.game_systems.vending_slots = 0;
        Ok(())
    }

    pub(crate) fn open_vending(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        account_id: u32,
    ) -> Result<(), String> {
        let owner = state
            .characters()
            .values()
            .find(|owner| owner.char_id == account_id)
            .ok_or("Vending owner is offline")?;
        let store = owner.game_systems.vending_store.as_ref().ok_or("Vending store is closed")?;
        if !nearby(character, owner) {
            return Err("Vending store is out of reach".into());
        }
        character.game_systems.opened_vending_store = Some(store.id);
        self.send(
            character.char_id,
            protocol::vending_items(
                store,
                &owner.game_systems.cart_items,
                false,
                self.configuration,
                server.packetver(),
            ),
        )
    }

    pub(crate) fn purchase_vending(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        account_id: u32,
        store_id: Option<u32>,
        purchases: Vec<(u16, u16)>,
    ) -> Result<(), String> {
        let id = state
            .characters()
            .values()
            .find(|owner| owner.char_id == account_id)
            .map(|owner| owner.char_id)
            .ok_or("Vending owner is offline")?;
        let owner = state.characters_mut().get_mut(&id).unwrap();
        let store = owner.game_systems.vending_store.as_ref().ok_or("Vending store is closed")?;
        let requested = store_id
            .or(character.game_systems.opened_vending_store)
            .ok_or("Vending purchase window is not open")?;
        if store.id != requested
            || (character.game_systems.opened_vending_store != Some(requested)
                && character.game_systems.remote_vending_store != Some(requested))
        {
            return Err("Vending store changed since it was opened".into());
        }
        if !nearby(character, owner) && character.game_systems.remote_vending_store != Some(requested) {
            return Err("Vending store is out of reach".into());
        }
        let trade = self
            .repository
            .vending_store_trade(
                character.char_id,
                requested,
                &purchases,
                server.character_service().max_weight(character) * 9 / 10,
                (self.configuration.config().game.vending_tax, self.configuration.config().game.vending_tax_min),
            )
            .map_err(|error| error.to_string())?;
        character.game_systems.opened_vending_store = None;
        owner.status.zeny = trade.seller_zeny;
        character.status.zeny = trade.buyer_zeny;
        owner.game_systems.cart_items = trade.cart;
        owner.game_systems.vending_store = trade.store;
        server
            .inventory_service()
            .reload_inventory(server.runtime(), character.char_id, character);
        self.send_cart(owner)?;
        if self.configuration.config().game.buyer_name {
            server.tell(owner.char_id, &format!("Player '{}' bought from your shop.", character.name));
        }
        for (index, amount) in purchases {
            let mut packet = protocol::header(0x0137);
            packet.extend_from_slice(&(index + 2).to_le_bytes());
            packet.extend_from_slice(&amount.to_le_bytes());
            self.send(owner.char_id, packet)?;
        }
        if owner.game_systems.vending_store.is_none() {
            self.area(owner, protocol::vending_disappear(owner.char_id))?;
        }
        self.send_zeny(owner)?;
        self.send_zeny(character)
    }

    pub(crate) fn send_cart(&self, character: &Character) -> Result<(), String> {
        for packet in protocol::cart_packets(
            &character.game_systems.cart_items,
            self.configuration,
            self.configuration.packetver(),
        ) {
            self.send(character.char_id, packet)?;
        }
        Ok(())
    }

    pub(crate) fn send_zeny(&self, character: &Character) -> Result<(), String> {
        let mut packet = protocol::header(0x00B0);
        packet.extend_from_slice(&(models::enums::status::StatusTypes::Zeny.value() as u16).to_le_bytes());
        packet.extend_from_slice(&character.status.zeny.to_le_bytes());
        self.send(character.char_id, packet)
    }
}

fn nearby(a: &Character, b: &Character) -> bool {
    a.current_map_name() == b.current_map_name()
        && a.current_map_instance() == b.current_map_instance()
        && a.x.abs_diff(b.x).max(a.y.abs_diff(b.y)) <= 5
}

pub fn vending_store_sign_packet(store: &VendingStore) -> Vec<u8> {
    protocol::vending_sign(store)
}

impl ScriptWorldService {
    pub(crate) fn store_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: StoreRequest,
        now: u64,
    ) -> Result<(), String> {
        match request {
            StoreRequest::PrepareVending { skill_level } => self.prepare_vending(state, character, skill_level),
            StoreRequest::CreateVendingStore { title, offers } => self.create_vending(server, state, character, title, offers),
            StoreRequest::CloseVendingStore => self.close_vending(character),
            StoreRequest::OpenVendingStore(account_id) => self.open_vending(server, state, character, account_id),
            StoreRequest::PurchaseVendingStore {
                account_id,
                store_id,
                items,
            } => self.purchase_vending(server, state, character, account_id, store_id, items),
            StoreRequest::CreateBuyingStore { title, zeny_limit, offers } => {
                if !state.has_permission(character.account_id, Permission::Trade) {
                    return Err("Your group is not allowed to buy items from players".into());
                }
                if state.map_flags(&character.map_instance_key).enabled(MapFlag::NoBuyingStore)
                    || state.cell_has(&character.map_instance_key, character.x(), character.y(), CellType::NoVending)
                {
                    return Err("Buying stores are disabled on this map".into());
                }
                if character.game_systems.buying_slots == 0
                    || offers.len() > usize::from(character.game_systems.buying_slots)
                    || character.game_systems.buying_store.is_some()
                    || character.game_systems.vending_store.is_some()
                    || character.game_systems.storage_open
                    || character.game_systems.guild_storage_open.is_some()
                    || character.status.hp == 0
                {
                    return Err("Buying store preparation window is not open".into());
                }
                let mut wanted_weight = u64::from(character.weight());
                for offer in &offers {
                    let item = self.configuration.find_item(offer.item_id).ok_or("Unknown buying store item")?;
                    let owned = character
                        .inventory
                        .iter()
                        .filter_map(Option::as_ref)
                        .filter(|item| item.item_id == offer.item_id)
                        .map(|item| i32::from(item.amount))
                        .sum::<i32>();
                    if owned + i32::from(offer.amount) > i32::from(i16::MAX) {
                        return Err("Requested buying store amount exceeds the item stack limit".into());
                    }
                    wanted_weight = wanted_weight.saturating_add(item.weight.max(0) as u64 * u64::from(offer.amount));
                }
                if wanted_weight >= u64::from(server.character_service().max_weight(character)) * 9 / 10 {
                    return Err("Cannot carry the full buying store order without becoming overweight".into());
                }
                let store = BuyingStore {
                    id: 0,
                    char_id: character.char_id,
                    account_id: character.account_id,
                    title,
                    map: character.map_instance_key.map_without_ext(),
                    map_instance: character.current_map_instance(),
                    x: character.x,
                    y: character.y,
                    zeny_limit,
                    offers,
                };
                let store = self.repository.create_buying_store(&store).map_err(|error| error.to_string())?;
                character.game_systems.buying_store = Some(store.clone());
                character.game_systems.buying_slots = 0;
                self.stop_for_store(server, character)?;
                self.send(
                    character.char_id,
                    protocol::store_items(&store, true, server.packetver(), |item_id| {
                        self.configuration.get_item(item_id).item_type.value() as u8
                    }),
                )?;
                self.area(character, protocol::store_sign(&store))
            }
            StoreRequest::CloseBuyingStore => self.close_buying(character),
            StoreRequest::OpenBuyingStore(account_id) => {
                let owner = state
                    .characters()
                    .values()
                    .find(|owner| owner.char_id == account_id)
                    .ok_or("Buying store owner is offline")?;
                let store = owner.game_systems.buying_store.as_ref().ok_or("Buying store is closed")?;
                if owner.current_map_instance() != character.current_map_instance()
                    || owner.current_map_name() != character.current_map_name()
                    || owner.x.abs_diff(character.x).max(owner.y.abs_diff(character.y)) > 5
                {
                    return Err("Buying store is out of reach".into());
                }
                self.send(
                    character.char_id,
                    protocol::store_items(store, false, server.packetver(), |item_id| {
                        self.configuration.get_item(item_id).item_type.value() as u8
                    }),
                )
            }
            StoreRequest::TradeBuyingStore {
                account_id,
                store_id,
                items,
            } => {
                let buyer_id = state
                    .characters()
                    .values()
                    .find(|owner| owner.char_id == account_id)
                    .ok_or("Buying store owner is offline")?
                    .char_id;
                let buyer = state.characters_mut().get_mut(&buyer_id).unwrap();
                let store = buyer.game_systems.buying_store.as_ref().ok_or("Buying store is closed")?.clone();
                if store.id != store_id {
                    return Err("Buying store changed since it was opened".into());
                }
                if character.current_map_name() != buyer.current_map_name()
                    || character.current_map_instance() != buyer.current_map_instance()
                    || character.x.abs_diff(buyer.x).max(character.y.abs_diff(buyer.y)) > 5
                {
                    if character.game_systems.remote_store != Some(store_id) {
                        return Err("Buying store is out of reach or changed".into());
                    }
                }
                let mut sales = Vec::new();
                for (index, item_id, amount) in &items {
                    let item = character
                        .get_item_from_inventory(*index as usize)
                        .ok_or("Unknown inventory sale index")?;
                    if item.item_id != *item_id {
                        return Err("Inventory sale item does not match requested identifier".into());
                    }
                    sales.push(BuyingSale {
                        inventory_id: item.id,
                        item_id: *item_id,
                        amount: *amount,
                    });
                }
                let max_weight = server.character_service().max_weight(buyer) * 9 / 10;
                let trade = self
                    .repository
                    .buying_store_trade(character.char_id, store_id, &sales, max_weight)
                    .map_err(|error| error.to_string())?;
                for (index, item_id, amount) in &items {
                    let price = store.offers.iter().find(|offer| offer.item_id == *item_id).unwrap().price;
                    let mut packet = protocol::header(0x081C);
                    packet.extend_from_slice(&(index + 2).to_le_bytes());
                    packet.extend_from_slice(&amount.to_le_bytes());
                    packet.extend_from_slice(&price.to_le_bytes());
                    self.send(character.char_id, packet)?;
                }
                character.status.zeny = trade.seller_zeny;
                buyer.status.zeny = trade.buyer_zeny;
                buyer.game_systems.buying_store = trade.store;
                server
                    .inventory_service()
                    .reload_inventory(server.runtime(), character.char_id, character);
                server.inventory_service().reload_inventory(server.runtime(), buyer.char_id, buyer);
                if let Some(store) = &buyer.game_systems.buying_store {
                    self.send(
                        buyer.char_id,
                        protocol::store_items(store, true, server.packetver(), |item_id| {
                            self.configuration.get_item(item_id).item_type.value() as u8
                        }),
                    )?;
                } else {
                    self.area(buyer, protocol::store_disappear(buyer.char_id))?;
                    let mut packet = protocol::header(0x081A);
                    packet.extend_from_slice(&4u16.to_le_bytes());
                    self.send(buyer.char_id, packet)?;
                }
                Ok(())
            }
            StoreRequest::SearchStores {
                kind,
                min_price,
                max_price,
                items,
                cards,
            } => {
                if kind > 1 {
                    return Err("Invalid store search filter".into());
                }
                let (min_price, max_price) = if max_price > 0 && min_price > max_price {
                    (max_price, min_price)
                } else {
                    (min_price, max_price)
                };
                let search = character
                    .game_systems
                    .store_search
                    .as_mut()
                    .ok_or("Store search window is not open")?;
                if now < search.next_query_at {
                    return self.send(character.char_id, protocol::search_failure(3));
                }
                if search.remaining_uses == 0 {
                    return self.send(character.char_id, protocol::search_failure(2));
                }
                search.remaining_uses -= 1;
                search.next_query_at = now.saturating_add(10_000);
                search.results.clear();
                search.next_page = 0;
                character.game_systems.remote_store = None;
                character.game_systems.remote_vending_store = None;
                if items.iter().any(|item| self.configuration.find_item(*item).is_none())
                    || cards.iter().any(|card| self.configuration.find_item(i32::from(*card)).is_none())
                {
                    self.send(character.char_id, protocol::search_failure(0))?;
                    return self.search_page(character, server.packetver());
                }
                let items: BTreeSet<i32> = items.into_iter().collect();
                let mut results = Vec::new();
                let mut overflow = false;
                if kind == 1 && cards.is_empty() {
                    'buying: for owner in state.characters().values().filter(|owner| owner.char_id != character.char_id) {
                        if let Some(store) = &owner.game_systems.buying_store {
                            if search.map.as_ref().is_some_and(|map| *map != store.map) {
                                continue;
                            }
                            for offer in &store.offers {
                                if offer.amount > 0
                                    && offer.price >= min_price
                                    && (max_price == 0 || offer.price <= max_price)
                                    && (items.is_empty() || items.contains(&offer.item_id))
                                {
                                    if results.len() == 30 {
                                        overflow = true;
                                        break 'buying;
                                    }
                                    results.push(StoreSearchResult {
                                        store: store.into(),
                                        kind: 1,
                                        item_id: offer.item_id,
                                        amount: offer.amount,
                                        price: offer.price,
                                        item_type: self.configuration.get_item(offer.item_id).item_type.value() as u8,
                                        refine: 0,
                                        cards: [0; 4],
                                    });
                                }
                            }
                        }
                    }
                }
                if kind == 0 {
                    'vending: for owner in state.characters().values().filter(|owner| owner.char_id != character.char_id) {
                        let Some(store) = &owner.game_systems.vending_store else {
                            continue;
                        };
                        if search.map.as_ref().is_some_and(|map| *map != store.map) {
                            continue;
                        }
                        for offer in &store.offers {
                            let Some(record) = owner.game_systems.cart_items.iter().find(|record| record.id == offer.inventory_id) else {
                                continue;
                            };
                            let slots = [record.card0 as u16, record.card1 as u16, record.card2 as u16, record.card3 as u16];
                            if offer.amount == 0
                                || offer.price < min_price
                                || (max_price > 0 && offer.price > max_price)
                                || (!items.is_empty() && !items.contains(&record.item_id))
                                || (!cards.is_empty() && !cards.iter().any(|card| slots.contains(card)))
                            {
                                continue;
                            }
                            if results.len() == 30 {
                                overflow = true;
                                break 'vending;
                            }
                            results.push(StoreSearchResult {
                                store: store.into(),
                                kind: 0,
                                item_id: record.item_id,
                                amount: offer.amount,
                                price: offer.price,
                                item_type: self.configuration.get_item(record.item_id).item_type.value() as u8,
                                refine: record.refine as u8,
                                cards: slots,
                            });
                        }
                    }
                }
                results.sort_by_key(|result| (result.price, result.store.id, result.item_id));
                search.results = results;
                search.next_page = 0;
                if overflow {
                    self.send(character.char_id, protocol::search_failure(1))?;
                } else if search.results.is_empty() {
                    self.send(character.char_id, protocol::search_failure(0))?;
                }
                self.search_page(character, server.packetver())
            }
            StoreRequest::NextSearchPage => self.search_page(character, server.packetver()),
            StoreRequest::CloseStoreSearch => {
                character.game_systems.store_search = None;
                character.game_systems.remote_store = None;
                character.game_systems.remote_vending_store = None;
                Ok(())
            }
            StoreRequest::LocateStore {
                account_id,
                store_id,
                item_id,
            } => {
                let search = character
                    .game_systems
                    .store_search
                    .as_ref()
                    .ok_or("Store search window is not open")?;
                let result = search
                    .results
                    .iter()
                    .find(|result| result.store.char_id == account_id && result.store.id == store_id && result.item_id == item_id)
                    .ok_or("Store result was not returned by this search")?;
                let owner = state.characters().get(&result.store.char_id).ok_or("Store owner went offline")?;
                if result.kind == 0 {
                    let store = owner
                        .game_systems
                        .vending_store
                        .as_ref()
                        .filter(|store| store.id == store_id)
                        .ok_or("Store is closed")?;
                    if search.remote {
                        character.game_systems.remote_vending_store = Some(store_id);
                        character.game_systems.opened_vending_store = Some(store_id);
                        return self.send(
                            character.char_id,
                            protocol::vending_items(
                                store,
                                &owner.game_systems.cart_items,
                                false,
                                self.configuration,
                                server.packetver(),
                            ),
                        );
                    }
                    if owner.current_map_name() != character.current_map_name()
                        || owner.current_map_instance() != character.current_map_instance()
                    {
                        return Err("Store is on another map".into());
                    }
                    let mut packet = protocol::header(0x083D);
                    packet.extend_from_slice(&store.x.to_le_bytes());
                    packet.extend_from_slice(&store.y.to_le_bytes());
                    return self.send(character.char_id, packet);
                }
                let store = owner
                    .game_systems
                    .buying_store
                    .as_ref()
                    .filter(|store| store.id == store_id)
                    .ok_or("Store is closed")?;
                if search.remote {
                    character.game_systems.remote_store = Some(store_id);
                    self.send(
                        character.char_id,
                        protocol::store_items(store, false, server.packetver(), |item_id| {
                            self.configuration.get_item(item_id).item_type.value() as u8
                        }),
                    )
                } else if owner.current_map_name() == character.current_map_name()
                    && owner.current_map_instance() == character.current_map_instance()
                {
                    let mut packet = protocol::header(0x083D);
                    packet.extend_from_slice(&store.x.to_le_bytes());
                    packet.extend_from_slice(&store.y.to_le_bytes());
                    self.send(character.char_id, packet)
                } else {
                    Err("Store is on another map".into())
                }
            }
        }
    }

    fn search_page(&self, character: &mut Character, packetver: u32) -> Result<(), String> {
        let search = character
            .game_systems
            .store_search
            .as_mut()
            .ok_or("Store search window is not open")?;
        let start = search.next_page;
        let end = (start + 10).min(search.results.len());
        let packet = protocol::search_results(
            &search.results[start..end],
            start == 0,
            end < search.results.len(),
            search.remaining_uses,
            packetver,
        );
        search.next_page = end;
        self.send(character.char_id, packet)
    }
}
