use std::collections::{BTreeMap, BTreeSet};

use database::model::InventoryRecord;
use models::enums::item::EquipmentLocation;
use models::enums::skill_enums::SkillEnum;
use models::enums::status::StatusTypes;
use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use packets::packets::{Packet, PacketZcLongparChange};

use crate::repository::game_system_repository::{PlayerTradeCommit, PlayerTradeSide, restrictions_allow};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, PlayerTradeAction};
use crate::server::model::game_systems::{PlayerTrade, PlayerTradeItem, PlayerTradePhase, PlayerTradeRequest};
use crate::server::model::map_flags::MapFlag;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[path = "player_trade_protocol.rs"]
mod protocol;
use protocol::{OfferResult, TradeResponse};
pub(crate) use protocol::{decode_request, frame_length};

pub(crate) const TRADE_IDLE_TIMEOUT_MS: u64 = 120_000;
const TRADE_DISTANCE: u16 = 2;

fn item_record(item: &InventoryItemModel) -> InventoryRecord {
    InventoryRecord {
        id: item.id,
        unique_id: item.unique_id,
        item_id: item.item_id,
        amount: item.amount,
        refine: item.refine,
        is_identified: item.is_identified,
        equip: item.equip,
        is_damaged: item.is_damaged,
        card0: item.card0,
        card1: item.card1,
        card2: item.card2,
        card3: item.card3,
    }
}

fn same_instance(item: &InventoryItemModel, offered: &InventoryRecord) -> bool {
    let mut current = item_record(item);
    current.amount = offered.amount;
    current == *offered
}

fn same_stack(first: &InventoryRecord, second: &InventoryRecord) -> bool {
    first.item_id == second.item_id
        && first.unique_id == 0
        && second.unique_id == 0
        && first.refine == second.refine
        && first.is_identified == second.is_identified
        && first.is_damaged == second.is_damaged
        && [first.card0, first.card1, first.card2, first.card3] == [second.card0, second.card1, second.card2, second.card3]
}

fn owns_conversation(state: &ServerState, character: &Character) -> bool {
    state.find_session(character.account_id).is_some_and(|session| {
        session
            .script_handler_channel_sender
            .lock()
            .map_or(true, |channel| channel.is_some())
    })
}

fn available(state: &ServerState, character: &Character, now: u64) -> bool {
    character.loaded_from_client_side
        && character.status.hp > 0
        && !character.is_dead()
        && !character.status.blocks_movement()
        && !character.is_using_skill()
        && character.script_skill_state.casting_until <= u128::from(now)
        && character.pending_item_skill.is_none()
        && !character.timing.skill_menu_blocked()
        && character.pending_craft.is_none()
        && !character.game_systems.storage_open
        && character.game_systems.guild_storage_open.is_none()
        && character.game_systems.vending_store.is_none()
        && character.game_systems.buying_store.is_none()
        && !owns_conversation(state, character)
        && !state.map_flags(&character.map_instance_key).enabled(MapFlag::NoTrade)
}

fn close_enough(first: &Character, second: &Character) -> bool {
    first.map_instance_key == second.map_instance_key && first.x.abs_diff(second.x).max(first.y.abs_diff(second.y)) <= TRADE_DISTANCE
}

fn pair(state: &ServerState, char_id: u32) -> Option<(&Character, &PlayerTrade, &Character, &PlayerTrade)> {
    let first = state.characters().get(&char_id)?;
    let first_trade = first.game_systems.trade.as_ref()?;
    let second = state.characters().get(&first_trade.partner_id)?;
    let second_trade = second.game_systems.trade.as_ref()?;
    (second_trade.partner_id == first.char_id
        && second_trade.session_id == first_trade.session_id
        && second_trade.requested_by == first_trade.requested_by)
        .then_some((first, first_trade, second, second_trade))
}

fn offers_current(character: &Character, trade: &PlayerTrade) -> bool {
    character.status.zeny >= trade.zeny
        && trade.items.iter().all(|offer| {
            character
                .get_item_from_inventory(offer.inventory_index)
                .is_some_and(|item| item.amount >= offer.amount && same_instance(item, &offer.item))
        })
}

fn touch_pair(state: &mut ServerState, char_id: u32, now: u64) {
    let Some(partner_id) = state
        .characters()
        .get(&char_id)
        .and_then(|character| character.game_systems.trade.as_ref())
        .map(|trade| trade.partner_id)
    else {
        return;
    };
    for id in [char_id, partner_id] {
        if let Some(trade) = state
            .characters_mut()
            .get_mut(&id)
            .and_then(|character| character.game_systems.trade.as_mut())
        {
            trade.requested_at = now;
        }
    }
}

fn inventory_capacity(server: &Server, target: &Character, offers: &[PlayerTradeItem]) -> Result<(), OfferResult> {
    let config = GlobalConfigService::instance();
    let mut records: Vec<_> = target.inventory_iter().map(|(_, item)| item_record(item)).collect();
    for offer in offers {
        let item = config.find_item(offer.item.item_id).ok_or(OfferResult::Closed)?;
        if item.item_type.is_stackable() {
            if let Some(existing) = records.iter_mut().find(|record| same_stack(record, &offer.item)) {
                let amount = i32::from(existing.amount) + i32::from(offer.amount);
                let limit = item
                    .stack_amount
                    .filter(|_| item.stack_inventory.unwrap_or(0) != 0)
                    .unwrap_or(i32::from(i16::MAX));
                if amount > limit.min(i32::from(i16::MAX)) {
                    return Err(OfferResult::StackFull);
                }
                existing.amount = amount as i16;
                continue;
            }
        }
        let mut record = offer.item.clone();
        record.amount = offer.amount;
        record.equip = 0;
        records.push(record);
    }
    if records.len() > usize::from(config.config().game.max_inventory) {
        return Err(OfferResult::Full);
    }
    let mut weight = 0u64;
    for record in records {
        let item = config.find_item(record.item_id).ok_or(OfferResult::Closed)?;
        if item.weight < 0 || record.amount <= 0 {
            return Err(OfferResult::Closed);
        }
        weight = weight.saturating_add(item.weight as u64 * record.amount as u64);
    }
    if weight > u64::from(server.character_service().max_weight(target)) {
        return Err(OfferResult::Overweight);
    }
    Ok(())
}

impl Server {
    pub(crate) fn handle_player_trade(&self, state: &mut ServerState, action: PlayerTradeAction, now: u64) -> Result<(), String> {
        let actor = state
            .characters()
            .get(&action.char_id)
            .filter(|character| character.account_id == action.account_id)
            .ok_or("Trade actor is unavailable")?;
        let session = state
            .find_session(action.account_id)
            .filter(|session| session.char_id == Some(actor.char_id) && session.auth_code == action.auth_code)
            .ok_or("Trade session expired")?;
        drop(session);
        match action.request {
            PlayerTradeRequest::Request(account_id) => self.request_player_trade(state, action.char_id, account_id, now),
            PlayerTradeRequest::Cancel => self.cancel_player_trade(state, action.char_id),
            request => {
                let Some((first, first_trade, second, second_trade)) = pair(state, action.char_id) else {
                    self.cancel_player_trade(state, action.char_id)?;
                    return Ok(());
                };
                if !available(state, first, now)
                    || !available(state, second, now)
                    || !close_enough(first, second)
                    || now.saturating_sub(first_trade.requested_at) >= TRADE_IDLE_TIMEOUT_MS
                    || now.saturating_sub(second_trade.requested_at) >= TRADE_IDLE_TIMEOUT_MS
                    || !offers_current(first, first_trade)
                    || !offers_current(second, second_trade)
                {
                    self.cancel_player_trade(state, action.char_id)?;
                    return Err("Player trade conditions changed".into());
                }
                match request {
                    PlayerTradeRequest::Answer(accept) => self.answer_player_trade(state, action.char_id, accept, now),
                    PlayerTradeRequest::Offer { index, amount } => self.offer_player_trade(state, action.char_id, index, amount, now),
                    PlayerTradeRequest::Lock => self.lock_player_trade(state, action.char_id, now),
                    PlayerTradeRequest::Confirm => self.confirm_player_trade(state, action.char_id, now),
                    _ => unreachable!(),
                }
            }
        }
    }

    fn trade_response(&self, state: &ServerState, char_id: u32, result: TradeResponse, partner_id: Option<u32>) -> Result<(), String> {
        let partner = partner_id.and_then(|id| state.characters().get(&id));
        self.script_world_service().send(
            char_id,
            protocol::response(
                result,
                partner.map_or(0, |character| character.account_id),
                partner.map_or(0, |character| character.status.base_level),
                self.packetver(),
            ),
        )
    }

    fn request_player_trade(&self, state: &mut ServerState, char_id: u32, account_id: u32, now: u64) -> Result<(), String> {
        let actor = state.characters().get(&char_id).ok_or("Trade actor is unavailable")?;
        if actor.game_systems.is_trading()
            || !available(state, actor, now)
            || !super::status_service::StatusService::instance()
                .to_snapshot(&actor.status)
                .known_skills()
                .iter()
                .any(|skill| skill.value == SkillEnum::NvBasic && skill.level >= 1)
        {
            return self.trade_response(state, char_id, TradeResponse::Failed, None);
        }
        if actor.game_systems.trade.is_some() {
            self.cancel_player_trade(state, char_id)?;
        }
        let targets: BTreeSet<_> = state
            .characters()
            .values()
            .filter(|character| (character.account_id == account_id || character.char_id == account_id) && character.char_id != char_id)
            .map(|character| character.char_id)
            .collect();
        if targets.len() != 1 {
            return self.trade_response(state, char_id, TradeResponse::Unavailable, None);
        }
        let target_id = *targets.first().unwrap();
        let actor = state.characters().get(&char_id).unwrap();
        let target = state.characters().get(&target_id).unwrap();
        if target.game_systems.trade.is_some()
            || !available(state, target, now)
            || target.game_systems.guild_invitation.is_some()
            || target.game_systems.party_invitation.is_some()
        {
            return self.trade_response(state, char_id, TradeResponse::Failed, Some(target_id));
        }
        if !close_enough(actor, target) {
            return self.trade_response(state, char_id, TradeResponse::TooFar, Some(target_id));
        }
        let id = self
            .repository
            .allocate_player_trade_session_id()
            .map_err(|error| error.to_string())?;
        let packet = protocol::request(&actor.name, actor.account_id, actor.status.base_level, self.packetver());
        for (id_actor, partner_id) in [(char_id, target_id), (target_id, char_id)] {
            state.characters_mut().get_mut(&id_actor).unwrap().game_systems.trade = Some(PlayerTrade {
                session_id: id,
                partner_id,
                requested_by: char_id,
                phase: PlayerTradePhase::Requested,
                items: vec![],
                zeny: 0,
                requested_at: now,
            });
        }
        if let Err(error) = self.script_world_service().send(target_id, packet) {
            self.cancel_player_trade(state, char_id)?;
            return Err(error);
        }
        Ok(())
    }

    fn answer_player_trade(&self, state: &mut ServerState, char_id: u32, accept: bool, now: u64) -> Result<(), String> {
        let (actor, trade, partner, partner_trade) = pair(state, char_id).ok_or("Trade request expired")?;
        if trade.requested_by == actor.char_id
            || trade.phase != PlayerTradePhase::Requested
            || partner_trade.phase != PlayerTradePhase::Requested
        {
            return Ok(());
        }
        let partner_id = partner.char_id;
        if !accept {
            self.trade_response(state, char_id, TradeResponse::Rejected, Some(partner_id))?;
            self.trade_response(state, partner_id, TradeResponse::Rejected, Some(char_id))?;
            for id in [char_id, partner_id] {
                state.characters_mut().get_mut(&id).unwrap().game_systems.trade = None;
            }
            return Ok(());
        }
        for id in [char_id, partner_id] {
            let mut character = state.characters_mut().remove(&id).unwrap();
            character.game_systems.trade.as_mut().unwrap().phase = PlayerTradePhase::Accepted;
            character.clear_pending_skill();
            let result = self.script_world_service().stop_for_store(self, &mut character);
            state.insert_character(character);
            if let Err(error) = result {
                self.cancel_player_trade(state, char_id)?;
                return Err(error);
            }
        }
        touch_pair(state, char_id, now);
        self.trade_response(state, char_id, TradeResponse::Accepted, Some(partner_id))?;
        self.trade_response(state, partner_id, TradeResponse::Accepted, Some(char_id))
    }

    fn offer_player_trade(&self, state: &mut ServerState, char_id: u32, index: u16, amount: u32, now: u64) -> Result<(), String> {
        let (actor, trade, partner, _) = pair(state, char_id).ok_or("Trade expired")?;
        if trade.phase != PlayerTradePhase::Accepted {
            return Ok(());
        }
        let partner_id = partner.char_id;
        if index == 0 {
            if amount > actor.status.zeny || u64::from(partner.status.zeny) + u64::from(amount) > i32::MAX as u64 {
                self.cancel_player_trade(state, char_id)?;
                return Err("Invalid player trade zeny offer".into());
            }
            state
                .characters_mut()
                .get_mut(&char_id)
                .unwrap()
                .game_systems
                .trade
                .as_mut()
                .unwrap()
                .zeny = amount;
            touch_pair(state, char_id, now);
            return self
                .script_world_service()
                .send(partner_id, protocol::offer(None, amount, self.packetver()));
        }
        if amount == 0 {
            return self
                .script_world_service()
                .send(char_id, protocol::offer_ack(0, OfferResult::Success, self.packetver()));
        }
        let result = self.plan_player_trade_offer(actor, trade, partner, index, amount);
        let (offers, offered_item, added) = match result {
            Ok(plan) => plan,
            Err(result) => {
                return self
                    .script_world_service()
                    .send(char_id, protocol::offer_ack(index, result, self.packetver()));
            }
        };
        let model = GlobalConfigService::instance().get_item(offered_item.item_id);
        let packet = protocol::offer(Some((&offered_item, model)), added as u32, self.packetver());
        state
            .characters_mut()
            .get_mut(&char_id)
            .unwrap()
            .game_systems
            .trade
            .as_mut()
            .unwrap()
            .items = offers;
        touch_pair(state, char_id, now);
        self.script_world_service()
            .send(char_id, protocol::offer_ack(index, OfferResult::Success, self.packetver()))?;
        self.script_world_service().send(partner_id, packet)
    }

    fn plan_player_trade_offer(
        &self,
        actor: &Character,
        trade: &PlayerTrade,
        partner: &Character,
        index: u16,
        amount: u32,
    ) -> Result<(Vec<PlayerTradeItem>, InventoryItemModel, i16), OfferResult> {
        let index = usize::from(index.checked_sub(2).ok_or(OfferResult::Closed)?);
        let record = actor.get_item_from_inventory(index).ok_or(OfferResult::Closed)?;
        let requested = i16::try_from(amount).map_err(|_| OfferResult::Closed)?;
        let config = GlobalConfigService::instance();
        let item = config.find_item(record.item_id).ok_or(OfferResult::Closed)?;
        if requested <= 0
            || requested > record.amount
            || record.id <= 0
            || (record.equip != 0
                && (record.equip as u64 != EquipmentLocation::Ammo.as_flag()
                    || !matches!(record.item_type(), models::enums::item::ItemType::Ammo)))
        {
            return Err(OfferResult::Closed);
        }
        let mut masks = vec![item.trade_flags];
        if !models::item::special_card_metadata(record.card0) {
            for card in [record.card0, record.card1, record.card2, record.card3]
                .into_iter()
                .take(item.slots.unwrap_or(0).clamp(0, 4) as usize)
                .filter(|id| *id != 0)
            {
                masks.push(config.find_item(card as u16 as i32).ok_or(OfferResult::Closed)?.trade_flags);
            }
        }
        let partners = actor.game_systems.partner_id == partner.char_id && partner.game_systems.partner_id == actor.char_id;
        if !restrictions_allow(&masks, partners) {
            return Err(OfferResult::Closed);
        }
        if matches!(record.item_type(), models::enums::item::ItemType::PetEgg) && record.card0 == 256 && record.card3 != 0 {
            return Err(OfferResult::Closed);
        }
        let mut offers = trade.items.clone();
        let added = if let Some(existing) = offers.iter_mut().find(|offer| offer.inventory_index == index) {
            if !same_instance(record, &existing.item) {
                return Err(OfferResult::Closed);
            }
            let added = requested.min(record.amount.saturating_sub(existing.amount));
            if added <= 0 {
                return Err(OfferResult::Closed);
            }
            existing.amount += added;
            added
        } else {
            if offers.len() >= 10 {
                return Err(OfferResult::Full);
            }
            offers.push(PlayerTradeItem {
                inventory_index: index,
                item: item_record(record),
                amount: requested,
            });
            requested
        };
        inventory_capacity(self, partner, &offers)?;
        Ok((offers, record.clone(), added))
    }

    fn lock_player_trade(&self, state: &mut ServerState, char_id: u32, now: u64) -> Result<(), String> {
        let (_, trade, partner, _) = pair(state, char_id).ok_or("Trade expired")?;
        if trade.phase != PlayerTradePhase::Accepted {
            return Ok(());
        }
        let partner_id = partner.char_id;
        state
            .characters_mut()
            .get_mut(&char_id)
            .unwrap()
            .game_systems
            .trade
            .as_mut()
            .unwrap()
            .phase = PlayerTradePhase::Locked;
        touch_pair(state, char_id, now);
        self.script_world_service()
            .send(char_id, protocol::offer_ack(0, OfferResult::Success, self.packetver()))?;
        self.script_world_service().send(char_id, protocol::lock(false))?;
        self.script_world_service().send(partner_id, protocol::lock(true))
    }

    fn confirm_player_trade(&self, state: &mut ServerState, char_id: u32, now: u64) -> Result<(), String> {
        let (_, trade, partner, partner_trade) = pair(state, char_id).ok_or("Trade expired")?;
        if trade.phase != PlayerTradePhase::Locked {
            return Ok(());
        }
        let partner_id = partner.char_id;
        let ready = partner_trade.phase == PlayerTradePhase::Confirmed;
        state
            .characters_mut()
            .get_mut(&char_id)
            .unwrap()
            .game_systems
            .trade
            .as_mut()
            .unwrap()
            .phase = PlayerTradePhase::Confirmed;
        touch_pair(state, char_id, now);
        if !ready {
            return Ok(());
        }
        let (first, first_trade, second, second_trade) = pair(state, char_id).unwrap();
        let side = |character: &Character, trade: &PlayerTrade| PlayerTradeSide {
            char_id: character.char_id,
            account_id: character.account_id,
            expected_revision: character.game_systems.revision,
            max_weight: self.character_service().max_weight(character),
            max_slots: usize::from(GlobalConfigService::instance().config().game.max_inventory),
            items: trade.items.clone(),
            zeny: trade.zeny,
        };
        let result = self.repository.commit_player_trade(&PlayerTradeCommit {
            session_id: first_trade.session_id,
            first: side(first, first_trade),
            second: side(second, second_trade),
        });
        let committed = match result {
            Ok(result) => result,
            Err(error) => {
                self.cancel_player_trade(state, char_id)?;
                return Err(error.to_string());
            }
        };
        for (id, inventory, zeny, systems) in [
            (
                char_id,
                committed.first_inventory,
                committed.first_zeny,
                committed.first_systems,
            ),
            (
                partner_id,
                committed.second_inventory,
                committed.second_zeny,
                committed.second_systems,
            ),
        ] {
            let mut character = state.characters_mut().remove(&id).unwrap();
            let mut remaining: BTreeMap<_, _> = inventory.into_iter().map(|item| (item.id, item)).collect();
            for slot in &mut character.inventory {
                *slot = slot.as_ref().and_then(|item| remaining.remove(&item.id));
            }
            for item in remaining.into_values() {
                character.add_in_inventory(item);
            }
            character.status.takeoff_all_equipment();
            character.game_systems.trade = None;
            super::script_world_service::install_state(&mut character, systems);
            character.status.zeny = zeny;
            character.refresh_script_context();
            let result = self
                .script_world_service()
                .send(id, protocol::completed())
                .and_then(|_| self.sync_trade_inventory(&mut character));
            state.insert_character(character);
            if let Err(error) = result {
                warn!("Trade committed but client refresh failed: {error}");
            }
        }
        Ok(())
    }

    fn sync_trade_inventory(&self, character: &mut Character) -> Result<(), String> {
        for packet in self.inventory_service().inventory_packets(character) {
            self.script_world_service().send(character.char_id, packet)?;
        }
        self.send_trade_wallet(character)?;
        self.add_to_next_tick(GameEvent::CharacterUpdateWeight(character.char_id));
        self.add_to_next_tick(GameEvent::CharacterUpdateClientSideStats(character.char_id));
        Ok(())
    }

    fn send_trade_wallet(&self, character: &Character) -> Result<(), String> {
        let mut packet = PacketZcLongparChange::new(self.packetver());
        packet.set_var_id(StatusTypes::Zeny.value() as u16);
        packet.set_amount(character.status.zeny as i32);
        packet.fill_raw();
        self.script_world_service().send(character.char_id, packet.raw)
    }

    pub(crate) fn cancel_player_trade(&self, state: &mut ServerState, char_id: u32) -> Result<(), String> {
        let Some(trade) = state
            .characters_mut()
            .get_mut(&char_id)
            .and_then(|character| character.game_systems.trade.take())
        else {
            return Ok(());
        };
        let partner_id = trade.partner_id;
        let counterpart = state.characters_mut().get_mut(&partner_id).and_then(|character| {
            if character
                .game_systems
                .trade
                .as_ref()
                .is_some_and(|current| current.partner_id == char_id && current.session_id == trade.session_id)
            {
                character.game_systems.trade.take()
            } else {
                None
            }
        });
        let mut failure = None;
        for (id, old_trade) in [(char_id, Some(trade)), (partner_id, counterpart)] {
            let Some(old_trade) = old_trade else {
                continue;
            };
            if let Err(error) = self.script_world_service().send(id, protocol::cancelled()) {
                failure = Some(error);
            }
            if !old_trade.items.is_empty() || old_trade.zeny > 0 {
                if let Some(mut character) = state.characters_mut().remove(&id) {
                    if let Err(error) = self.sync_trade_inventory(&mut character) {
                        failure = Some(error);
                    }
                    state.insert_character(character);
                }
            }
        }
        failure.map_or(Ok(()), Err)
    }

    pub(crate) fn tick_player_trades(&self, state: &mut ServerState, now: u64) {
        if let Err(error) = self.script_world_service().drain_notifications() {
            warn!("Trade notification drain failed: {error}");
        }
        let actors: Vec<_> = state
            .characters()
            .values()
            .filter(|character| character.game_systems.trade.is_some())
            .map(|character| character.char_id)
            .collect();
        let mut visited = BTreeSet::new();
        for id in actors {
            let Some(trade) = state
                .characters()
                .get(&id)
                .and_then(|character| character.game_systems.trade.as_ref())
            else {
                continue;
            };
            if !visited.insert(trade.session_id) {
                continue;
            }
            let invalid = pair(state, id).is_none_or(|(first, first_trade, second, second_trade)| {
                !available(state, first, now)
                    || !available(state, second, now)
                    || !close_enough(first, second)
                    || now.saturating_sub(first_trade.requested_at) >= TRADE_IDLE_TIMEOUT_MS
                    || now.saturating_sub(second_trade.requested_at) >= TRADE_IDLE_TIMEOUT_MS
                    || !offers_current(first, first_trade)
                    || !offers_current(second, second_trade)
            });
            if invalid {
                if let Err(error) = self.cancel_player_trade(state, id) {
                    warn!("Trade cancellation failed: {error}");
                }
            }
        }
    }
}
