use std::collections::BTreeSet;

use models::enums::EnumWithNumberValue;

use super::{ScriptWorldService, install_state, pet_world_id, protocol, world_data};
use crate::server::Server;
use crate::server::model::battleground_queue::BattlegroundQueueCommand;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::game_systems::{BuyingSale, BuyingStore, ScriptWorldRequest, StoreSearchResult};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptWorldService {
    pub fn handle_request(
        &self,
        server: &Server,
        state: &mut ServerState,
        char_id: u32,
        request: ScriptWorldRequest,
        now: u64,
    ) -> Result<(), String> {
        if state.characters().get(&char_id).is_some_and(|character| character.game_systems.is_trading() || character.timing.skill_menu_blocked())
            && matches!(&request, ScriptWorldRequest::ContainerTransfer { .. }
                | ScriptWorldRequest::StorageDeposit { .. } | ScriptWorldRequest::StorageWithdraw { .. }
                | ScriptWorldRequest::PrepareVending { .. } | ScriptWorldRequest::CreateVendingStore { .. }
                | ScriptWorldRequest::OpenVendingStore(_) | ScriptWorldRequest::PurchaseVendingStore { .. }
                | ScriptWorldRequest::CreateBuyingStore { .. } | ScriptWorldRequest::OpenBuyingStore(_)
                | ScriptWorldRequest::TradeBuyingStore { .. } | ScriptWorldRequest::HatchPet(_)
                | ScriptWorldRequest::EquipPetAccessory(_) | ScriptWorldRequest::PetMenu(1)
                | ScriptWorldRequest::CapturePet(_)) {
            return Err("Inventory operations are unavailable during trading".into());
        }
        let mut character = state.characters_mut().remove(&char_id).ok_or("Character is not online")?;
        let previous_pet_bonus = super::pet_bonus_state(&character);
        let skill = match &request {
            ScriptWorldRequest::UseCompanionSkill { skill_id, .. } | ScriptWorldRequest::UseCompanionGroundSkill { skill_id, .. } => {
                Some((*skill_id, None))
            }
            ScriptWorldRequest::FinishCompanionSkill(id) => character
                .game_systems
                .companion_commands
                .get(id)
                .and_then(|command| command.cast.as_ref())
                .map(|cast| (cast.skill_id, Some(*id))),
            _ => None,
        };
        let result = self.request(server, state, &mut character, request, now);
        self.refresh_pet_bonuses(server, &mut character, previous_pet_bonus);
        if result.is_err() {
            if let Some((skill_id, actor_id)) = skill {
                if let Err(error) = self.send(
                    character.char_id,
                    protocol::skill_failed(skill_id, models::enums::skill::UseSkillFailure::Fail),
                ) {
                    warn!("Failed to send companion skill failure: {}", error);
                }
                if let Some(actor_id) = actor_id {
                    let mut packet = protocol::header(0x01B9);
                    packet.extend_from_slice(&actor_id.to_le_bytes());
                    if let Err(error) = self.area(&character, packet) {
                        warn!("Failed to cancel companion cast display: {}", error);
                    }
                }
            }
        }
        state.characters_mut().insert(char_id, character);
        result
    }

    fn request(
        &self,
        server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        request: ScriptWorldRequest,
        now: u64,
    ) -> Result<(), String> {
        match request {
            request @ (ScriptWorldRequest::CreateParty { .. }
            | ScriptWorldRequest::InviteParty(_)
            | ScriptWorldRequest::InvitePartyByName(_)
            | ScriptWorldRequest::AnswerPartyInvite { .. }
            | ScriptWorldRequest::LeaveParty
            | ScriptWorldRequest::ExpelParty { .. }
            | ScriptWorldRequest::ChangePartyLeader(_)
            | ScriptWorldRequest::ChangePartyOptions { .. }
            | ScriptWorldRequest::DisablePartyInvites(_)
            | ScriptWorldRequest::PartyMessage(_)
            | ScriptWorldRequest::RefreshParty) => self.party_request(server, state, character, request),
            ScriptWorldRequest::BattlegroundQueue(action) => {
                server.add_to_next_tick(GameEvent::BattlegroundQueue(BattlegroundQueueCommand { char_id: character.char_id, action }));
                Ok(())
            }
            ScriptWorldRequest::BattlegroundMessage(message) => {
                if !message.starts_with(&format!("{} : ", character.name)) {
                    return Err("Battleground chat sender does not match the character".into());
                }
                server.battleground_chat(state, character, &message);
                Ok(())
            }
            request @ (ScriptWorldRequest::CreateGuild(_)
            | ScriptWorldRequest::InviteGuild(_)
            | ScriptWorldRequest::AnswerGuildInvite { .. }
            | ScriptWorldRequest::GuildMenu
            | ScriptWorldRequest::GuildInformation(_)
            | ScriptWorldRequest::LeaveGuild { .. }
            | ScriptWorldRequest::ExpelGuild { .. }
            | ScriptWorldRequest::DisbandGuild(_)
            | ScriptWorldRequest::GuildNotice { .. }
            | ScriptWorldRequest::GuildPositions(_)
            | ScriptWorldRequest::GuildMemberPositions(_)
            | ScriptWorldRequest::GuildEmblem(_)
            | ScriptWorldRequest::GuildEmblemRequest(_)
            | ScriptWorldRequest::GuildMessage(_)
            | ScriptWorldRequest::GuildSkillUp(_)
            | ScriptWorldRequest::GuildAllianceRequest(_)
            | ScriptWorldRequest::GuildAllianceReply { .. }
            | ScriptWorldRequest::GuildOpposition(_)
            | ScriptWorldRequest::GuildRelationBreak { .. }) => self.guild_request(server, state, character, request),
            ScriptWorldRequest::CompanionAttackLanded { id, damage } => {
                let Some(homunculus) = character
                    .game_systems
                    .homunculus
                    .as_mut()
                    .filter(|homunculus| super::homunculus_world_id(homunculus) == id && homunculus.active && homunculus.hp > 0)
                else {
                    return Ok(());
                };
                let Some(bloodlust) = homunculus
                    .statuses
                    .iter()
                    .find(|status| status.kind == models::status_change::StatusChangeKind::Bloodlust)
                    .filter(|status| status.expires_at.is_none_or(|expiry| u128::from(now) < expiry))
                else {
                    return Ok(());
                };
                if homunculus
                    .statuses
                    .iter()
                    .any(|status| status.kind == models::status_change::StatusChangeKind::NoRecovery)
                {
                    return Ok(());
                }
                if fastrand::u32(0..100) >= bloodlust.values[2].clamp(0, 100) as u32 {
                    return Ok(());
                }
                let heal = u64::from(damage) * bloodlust.values[3].max(0) as u64 / 100;
                let before = homunculus.hp;
                homunculus.hp = homunculus
                    .hp
                    .saturating_add(heal.min(u64::from(u32::MAX)) as u32)
                    .min(homunculus.max_hp);
                if homunculus.hp != before {
                    self.persist(character)?;
                    self.send_homunculus(character)?;
                }
                Ok(())
            }
            ScriptWorldRequest::HealByCompanion { source_id: _, hp, sp } => {
                if character.status.hp > 0
                    && !character
                        .status
                        .has_status_change(models::status_change::StatusChangeKind::NoRecovery)
                {
                    let snapshot = crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status);
                    server.character_service().update_hp_sp(
                        character,
                        character.status.hp.saturating_add(hp).min(snapshot.max_hp()),
                        character.status.sp.saturating_add(sp).min(snapshot.max_sp()),
                    );
                }
                Ok(())
            }
            ScriptWorldRequest::HealCompanion { target_id, hp, sp } => self.heal_companion(server, character, target_id, hp, sp, now),
            ScriptWorldRequest::UseCompanionSkill {
                skill_id,
                skill_level,
                target_id,
            } => self.begin_companion_skill(server, state, character, skill_id, skill_level, target_id, now),
            ScriptWorldRequest::UseCompanionGroundSkill {
                skill_id,
                skill_level,
                x,
                y,
            } => self.begin_companion_ground_skill(server, state, character, skill_id, skill_level, x, y, now),
            ScriptWorldRequest::FinishCompanionSkill(id) => self.finish_companion_skill(server, state, character, id, now),
            ScriptWorldRequest::FinishPetSupport(id) => self.finish_pet_support(server, state, character, id, now),
            ScriptWorldRequest::PetCombatTarget { target_id, retaliation } => self.pet_combat_target(server, state, character, target_id, retaliation, now),
            ScriptWorldRequest::PetLootTarget(target_id) => self.pet_loot_target(state, character, target_id, now),
            ScriptWorldRequest::CompanionSelfDestruct(id) => self.destroy_companion(server, character, id, now),
            request @ (ScriptWorldRequest::CallHomunculus
            | ScriptWorldRequest::RestHomunculus
            | ScriptWorldRequest::ResurrectHomunculus { .. }
            | ScriptWorldRequest::HomunculusMenu(_)
            | ScriptWorldRequest::HomunculusRename(_)
            | ScriptWorldRequest::CompanionMove { .. }
            | ScriptWorldRequest::CompanionMoveToOwner(_)
            | ScriptWorldRequest::CompanionAttack { .. }) => self.homunculus_request(server, character, request, now),
            ScriptWorldRequest::FameList(kind) => self.fame_list(character, kind),
            ScriptWorldRequest::CallPartner => self.call_partner(server, state, character),
            request @ (ScriptWorldRequest::BookingRegister { .. }
            | ScriptWorldRequest::BookingSearch { .. }
            | ScriptWorldRequest::BookingDelete
            | ScriptWorldRequest::BookingUpdate(_)) => self.booking_request(state, character, request),
            ScriptWorldRequest::CallBaby => self.call_family(server, state, character, false),
            ScriptWorldRequest::CallParents => self.call_family(server, state, character, true),
            ScriptWorldRequest::AdoptRequest(account) => self.request_adoption(state, character, account),
            ScriptWorldRequest::AdoptAnswer { father_account, mother_account, accept } => {
                self.answer_adoption(server, state, character, father_account, mother_account, accept)
            }
            ScriptWorldRequest::SetCart(style) => self.set_cart(character, style, false),
            ScriptWorldRequest::ChangeCart(style) => self.set_cart(character, style, true),
            ScriptWorldRequest::RemoveOption => {
                self.remove_option(character)?;
                server.character_service().reload_client_side_status(character);
                Ok(())
            }
            ScriptWorldRequest::ContainerTransfer {
                source,
                destination,
                index,
                amount,
            } => self.move_container(server, state, character, source, destination, index, amount),
            ScriptWorldRequest::PrepareVending { skill_level } => self.prepare_vending(state, character, skill_level),
            ScriptWorldRequest::CreateVendingStore { title, offers } => self.create_vending(server, state, character, title, offers),
            ScriptWorldRequest::CloseVendingStore => self.close_vending(character),
            ScriptWorldRequest::OpenVendingStore(account_id) => self.open_vending(server, state, character, account_id),
            ScriptWorldRequest::PurchaseVendingStore {
                account_id,
                store_id,
                items,
            } => self.purchase_vending(server, state, character, account_id, store_id, items),
            ScriptWorldRequest::CapturePet(target) => self.capture_pet(server, state, character, target, now),
            ScriptWorldRequest::HatchPet(index) => {
                if !std::mem::take(&mut character.game_systems.pet_hatching) {
                    return Err("Pet hatching window is not open".into());
                }
                let item = character
                    .get_item_from_inventory(index as usize)
                    .ok_or("Unknown pet egg inventory index")?;
                let saved = self
                    .repository
                    .hatch_pet(character.char_id, item.id, now)
                    .map_err(|error| error.to_string())?;
                install_state(character, saved);
                server
                    .inventory_service()
                    .reload_inventory(server.runtime(), character.char_id, character);
                self.send_pet(character)?;
                self.render_companions(server, character, now)
            }
            ScriptWorldRequest::PetMenu(menu) => match menu {
                0 => self.send_pet(character),
                1 => self.feed_pet(server, character, now),
                2 => {
                    self.drop_pet_loot_in_state(server, state, character, now)?;
                    let pet = character.game_systems.pet.as_ref().ok_or("No pet is active")?;
                    let command = character.game_systems.companion_commands.entry(pet_world_id(pet.id)).or_default();
                    command.destination = None;
                    command.target = None;
                    command.next_move_at = now.saturating_add(2000);
                    let mut packet = protocol::header(0x01A4);
                    packet.push(4);
                    packet.extend_from_slice(&pet_world_id(pet.id).to_le_bytes());
                    packet.extend_from_slice(&fastrand::u32(1..=if pet.intimacy > 900 { 4 } else { 3 }).to_le_bytes());
                    self.area(character, packet)
                }
                3 => {
                    self.return_pet_loot_in_state(server, state, character, now)?;
                    let saved = self
                        .repository
                        .return_pet_to_egg(character.char_id)
                        .map_err(|error| error.to_string())?;
                    install_state(character, saved);
                    server
                        .inventory_service()
                        .reload_inventory(server.runtime(), character.char_id, character);
                    self.render_companions(server, character, now)
                }
                4 => {
                    let saved = self
                        .repository
                        .pet_accessory(character.char_id, None, 0)
                        .map_err(|error| error.to_string())?;
                    install_state(character, saved);
                    server
                        .inventory_service()
                        .reload_inventory(server.runtime(), character.char_id, character);
                    self.send_pet(character)?;
                    if let Some(packet) = super::pet_accessory_packet(character, self.configuration) {
                        self.area(character, packet)?;
                    }
                    Ok(())
                }
                _ => Err("Unknown pet menu operation".into()),
            },
            ScriptWorldRequest::PetRename(name) => {
                if name.trim().is_empty() || name.len() > 23 || name.chars().any(char::is_control) {
                    return Err("Invalid pet name".into());
                }
                let pet = character.game_systems.pet.as_mut().ok_or("No pet is active")?;
                if pet.renamed {
                    return Err("The pet has already been renamed".into());
                }
                pet.name = name;
                pet.renamed = true;
                self.persist(character)?;
                self.send_pet(character)
            }
            ScriptWorldRequest::PetEmotion(value) => {
                if !(0..=200_000).contains(&value) {
                    return Err("Invalid pet emotion".into());
                }
                let pet = character.game_systems.pet.as_ref().ok_or("No pet is active")?;
                let mut packet = protocol::header(0x01AA);
                packet.extend_from_slice(&pet_world_id(pet.id).to_le_bytes());
                packet.extend_from_slice(&value.to_le_bytes());
                self.area(character, packet)
            }
            ScriptWorldRequest::EquipPetAccessory(index) => {
                let pet = character.game_systems.pet.as_ref().ok_or("No pet is active")?;
                let definition = world_data()
                    .pets
                    .iter()
                    .find(|definition| definition.class_id == pet.class_id)
                    .ok_or("Unknown pet class")?;
                let item = character
                    .get_item_from_inventory(index as usize)
                    .ok_or("Unknown pet accessory inventory index")?;
                if definition.equip_item == 0 || item.item_id != definition.equip_item {
                    return Err("This accessory cannot be equipped by this pet".into());
                }
                let saved = self
                    .repository
                    .pet_accessory(character.char_id, Some(item.id), definition.equip_item)
                    .map_err(|error| error.to_string())?;
                install_state(character, saved);
                server
                    .inventory_service()
                    .reload_inventory(server.runtime(), character.char_id, character);
                self.send_pet(character)?;
                if let Some(packet) = super::pet_accessory_packet(character, self.configuration) {
                    self.area(character, packet)?;
                }
                Ok(())
            }
            ScriptWorldRequest::CreateBuyingStore { title, zeny_limit, offers } => {
                if state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoBuyingStore) {
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
            ScriptWorldRequest::CloseBuyingStore => self.close_buying(character),
            ScriptWorldRequest::OpenBuyingStore(account_id) => {
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
            ScriptWorldRequest::TradeBuyingStore {
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
            ScriptWorldRequest::SearchStores {
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
            ScriptWorldRequest::NextSearchPage => self.search_page(character, server.packetver()),
            ScriptWorldRequest::CloseStoreSearch => {
                character.game_systems.store_search = None;
                character.game_systems.remote_store = None;
                character.game_systems.remote_vending_store = None;
                Ok(())
            }
            ScriptWorldRequest::LocateStore {
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
            ScriptWorldRequest::DismissMercenary(command) => {
                if command == 2 {
                    character.game_systems.mercenary = None;
                    self.persist(character)?;
                    self.render_companions(server, character, now)
                } else {
                    self.send_mercenary(character, now)
                }
            }
            ScriptWorldRequest::StorageDeposit { index, amount } => self.move_container(
                server,
                state,
                character,
                crate::server::model::game_systems::ItemContainer::Inventory,
                crate::server::model::game_systems::ItemContainer::Storage,
                index,
                amount,
            ),
            ScriptWorldRequest::StorageWithdraw { index, amount } => self.move_container(
                server,
                state,
                character,
                crate::server::model::game_systems::ItemContainer::Storage,
                crate::server::model::game_systems::ItemContainer::Inventory,
                index,
                amount,
            ),
            ScriptWorldRequest::CloseStorage => self.close_storage(character),
        }
    }

    fn capture_pet(
        &self,
        _server: &Server,
        state: &mut ServerState,
        character: &mut Character,
        target: u32,
        now: u64,
    ) -> Result<(), String> {
        if character.game_systems.pending_pet_capture.is_some() {
            return Err("A pet capture is awaiting confirmation".into());
        }
        let capture = character.game_systems.pet_capture.take().ok_or("No pet capture is in progress")?;
        if now >= capture.expires_at || character.status.hp == 0 || !character.loaded_from_client_side
            || state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoPetCapture) {
            return self.send(character.char_id, vec![0xA0, 0x01, 0]);
        }
        if state.contains_locked_map_item(target) {
            return Err("Monster is already being captured or removed".into());
        }
        let map = state.get_map_instance_from_character(character).ok_or("Character map is unavailable")?;
        let claim_id = self.next_pet_capture_id.fetch_update(std::sync::atomic::Ordering::Relaxed, std::sync::atomic::Ordering::Relaxed, |value| value.checked_add(1))
            .map_err(|_| "Pet capture identifiers exhausted")?;
        let expires_at = capture.expires_at.min(now.saturating_add(5000));
        character.game_systems.pending_pet_capture = Some(crate::server::model::game_systems::PendingPetCapture {
            claim_id, target_id: target, map_key: character.map_instance_key.clone(), expires_at, committed: false,
        });
        state.insert_locked_map_item(target);
        map.add_to_next_tick(MapEvent::ClaimPetCapture(crate::server::model::events::map_event::PetCaptureClaimRequest {
            claim_id, char_id: character.char_id, target_id: target, x: character.x, y: character.y,
            lure_item_id: capture.item_id, flag: capture.flag, expires_at,
        }));
        Ok(())
    }
    fn feed_pet(&self, server: &Server, character: &mut Character, now: u64) -> Result<(), String> {
        let pet = character.game_systems.pet.as_ref().ok_or("No pet is active")?;
        let data = world_data()
            .pets
            .iter()
            .find(|data| data.class_id == pet.class_id)
            .ok_or("Unknown pet class")?;
        let intimacy = if pet.hunger > 90 {
            data.intimacy_overfed
        } else if pet.hunger > 75 {
            (data.intimacy_fed / 2).max(1)
        } else {
            data.intimacy_fed
        };
        let state = self
            .repository
            .feed_pet(
                character.char_id,
                data.food_item,
                data.hunger_increase,
                intimacy,
                now + data.hungry_delay,
            )
            .map_err(|error| error.to_string())?;
        install_state(character, state);
        server
            .inventory_service()
            .reload_inventory(server.runtime(), character.char_id, character);
        self.send_pet(character)?;
        let mut packet = protocol::header(0x01A3);
        packet.push(1);
        packet.extend_from_slice(&(data.food_item as u16).to_le_bytes());
        self.send(character.char_id, packet)
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
