use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};

use super::{ScriptWorldService, protocol};
use crate::server::Server;
use crate::server::model::game_systems::{ItemContainer, PlayerOption, VendingOffer, VendingStore};
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

impl ScriptWorldService {
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
        character.options = options;
        if style == 0 {
            self.send(character.char_id, protocol::header(0x012B))?;
        } else {
            self.initialize_cart(character)?;
        }
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
        character.options = options;
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
        character: &mut Character,
        source: ItemContainer,
        destination: ItemContainer,
        index: u16,
        amount: u32,
    ) -> Result<(), String> {
        let source = if source == ItemContainer::Storage && character.game_systems.guild_storage_open.is_some() { ItemContainer::GuildStorage } else { source };
        let destination = if destination == ItemContainer::Storage && character.game_systems.guild_storage_open.is_some() { ItemContainer::GuildStorage } else { destination };
        if (source == ItemContainer::Storage || destination == ItemContainer::Storage) && !character.game_systems.storage_open {
            return Err("Storage window is not open".into());
        }
        if (source == ItemContainer::GuildStorage || destination == ItemContainer::GuildStorage) && character.game_systems.guild_storage_open.is_none() { return Err("Guild storage is not open".into()); }
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
            ItemContainer::GuildStorage => character.game_systems.guild_storage_items.get(index as usize).map(|record| record.id),
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
        if character.game_systems.guild_storage_open.is_some() { character.game_systems.guild_storage_items = transfer.guild_storage; }
        if source == ItemContainer::Inventory || destination == ItemContainer::Inventory {
            server
                .inventory_service()
                .reload_inventory(server.runtime(), character.char_id, character);
        }
        if source == ItemContainer::Cart || destination == ItemContainer::Cart {
            self.send_cart(character)?;
        }
        if matches!(source, ItemContainer::Storage | ItemContainer::GuildStorage) || matches!(destination, ItemContainer::Storage | ItemContainer::GuildStorage) {
            self.send_storage(character)?;
        }
        Ok(())
    }

    pub(crate) fn prepare_vending(&self, character: &mut Character, skill_level: u8) -> Result<(), String> {
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
        character: &mut Character,
        title: String,
        offers: Vec<(u16, u16, u32)>,
    ) -> Result<(), String> {
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
                    price,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
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
