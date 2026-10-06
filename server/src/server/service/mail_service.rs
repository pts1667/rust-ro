//! Classic mail (PACKETVER before 2015-05-13): text, up to 30 mails per inbox, one item and zeny per mail.

use models::enums::EnumWithMaskValueU64;
use models::enums::item::{ItemTradeFlag, ItemType};

use crate::repository::mail_repository::{MAIL_BODY_LENGTH, MAIL_MAX_INBOX, MAIL_TITLE_LENGTH, MailItem, MailMessage};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterAddItems, CharacterRemoveItem, CharacterRemoveItems, CharacterZeny, MailAction};
use crate::server::model::game_systems::MailDraft;
use crate::server::model::map_flags::MapFlag;
use crate::server::model::permission_groups::{CommandKind, Permission};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::social_packets as wire;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// `MAX_ZENY` of rathena.
const MAX_ZENY: u64 = 1_000_000_000;
/// Marks an egg whose pet was hatched, the same way the player trade does.
const HATCHED_EGG_CARD: i16 = 256;
const RESET_ALL: u16 = 0;
const RESET_ITEM: u16 = 1;
const RESET_ZENY: u16 = 2;
const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

fn now_seconds() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

fn mail_item_of(item: &InventoryItemModel, amount: i16) -> MailItem {
    MailItem {
        item_id: item.item_id,
        amount,
        refine: item.refine,
        identified: item.is_identified,
        damaged: item.is_damaged,
        unique_id: item.unique_id,
        cards: [item.card0, item.card1, item.card2, item.card3],
    }
}

fn inventory_item_of(item: &MailItem) -> Option<InventoryItemModel> {
    let model = GlobalConfigService::instance().find_item(item.item_id)?;
    let mut inventory_item = InventoryItemModel::from_item_model(model, item.amount, item.identified);
    inventory_item.refine = item.refine;
    inventory_item.is_damaged = item.damaged;
    inventory_item.unique_id = item.unique_id;
    [inventory_item.card0, inventory_item.card1, inventory_item.card2, inventory_item.card3] = item.cards;
    Some(inventory_item)
}

impl Server {
    pub(crate) fn run_mail_action(&self, state: &mut ServerState, char_id: u32, action: MailAction, tick: u128) {
        let Some(character) = state.get_character(char_id) else { return };
        if !self.mail_allowed(state, character) {
            return;
        }
        match action {
            MailAction::ResetDraft(kind) => self.reset_mail_draft(state, char_id, kind),
            MailAction::List => {
                self.reset_mail_draft(state, char_id, RESET_ALL);
                self.send_mail_list(char_id);
            }
            MailAction::Open(mail_id) => self.open_mail(char_id, mail_id),
            MailAction::Delete(mail_id) => {
                let deleted = self.repository.mail_delete(char_id, mail_id).unwrap_or(false);
                self.send_raw(char_id, wire::mail_delete_result(mail_id, !deleted));
            }
            MailAction::TakeAttachment(mail_id) => self.take_mail_attachment(state, char_id, mail_id),
            MailAction::Attach { index, amount } => self.attach_to_mail(state, char_id, index, amount),
            MailAction::Send { recipient, title, body } => self.send_mail(state, char_id, tick, &recipient, title, body),
            MailAction::Return(mail_id) => self.return_mail(char_id, mail_id),
        }
    }

    /// Mail is a town feature unless the group may use `@mail`.
    fn mail_allowed(&self, state: &ServerState, character: &Character) -> bool {
        state.map_flags(&character.map_instance_key).enabled(MapFlag::Town)
            || state
                .permission_groups()
                .can_use_command(state.group_id_of(character.account_id), "mail", CommandKind::At)
    }

    /// `@mail` and `openmail`: opens the mail window.
    pub(crate) fn open_mail_window(&self, state: &ServerState, char_id: u32) {
        let Some(character) = state.get_character(char_id) else { return };
        let systems = &character.game_systems;
        let busy = systems.is_trading() || systems.vending_store.is_some() || systems.buying_store.is_some() || systems.storage_open;
        if !busy {
            self.send_raw(char_id, wire::mail_window(false));
        }
    }

    fn inbox(&self, char_id: u32) -> Vec<MailMessage> {
        let config = &self.configuration.game.mail;
        let mut inbox = self
            .repository
            .mail_inbox(char_id, now_seconds(), config.return_days, config.delete_days)
            .unwrap_or_else(|error| {
                warn!("Mailbox of {char_id} could not be read: {error}");
                Vec::new()
            });
        inbox.truncate(MAIL_MAX_INBOX);
        inbox
    }

    fn send_mail_list(&self, char_id: u32) {
        let inbox = self.inbox(char_id);
        self.send_raw(char_id, wire::mail_list(&inbox));
        if inbox.len() >= MAIL_MAX_INBOX {
            self.tell(char_id, &format!("Inbox is full (Max {MAIL_MAX_INBOX}). Delete some mails."));
        }
    }

    /// Tells a character who just entered the map about its inbox, as `mail_show_status` asks.
    pub(crate) fn mail_login(&self, char_id: u32) {
        let show = self.configuration.game.mail.show_status;
        if show == 0 {
            return;
        }
        let unread = self.inbox(char_id).iter().filter(|mail| !mail.read).count();
        if show == 1 || unread > 0 {
            self.tell(char_id, &format!("You have {unread} new emails ({unread} unread)"));
        }
    }

    fn open_mail(&self, char_id: u32, mail_id: i32) {
        let Some(mail) = self.inbox(char_id).into_iter().find(|mail| mail.id == mail_id) else {
            self.send_raw(char_id, wire::mail_return_result(mail_id, true));
            return;
        };
        let item_type = mail
            .item
            .as_ref()
            .and_then(|item| GlobalConfigService::instance().find_item(item.item_id))
            .map_or(0, |model| model.item_type.to_client_type() as u16);
        self.send_raw(char_id, wire::mail_opened(&mail, item_type));
        if !mail.read {
            if let Err(error) = self.repository.mail_mark_read(char_id, mail_id) {
                warn!("Mail {mail_id} could not be marked as read: {error}");
            }
            self.send_mail_list(char_id);
        }
    }

    fn resync_inventory(&self, character: &mut Character) {
        for packet in self.inventory_service().inventory_packets(character) {
            self.send_raw(character.char_id, packet);
        }
    }

    fn reset_mail_draft(&self, state: &mut ServerState, char_id: u32, kind: u16) {
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        let draft = &mut character.game_systems.mail_draft;
        let had_item = draft.item.is_some();
        if matches!(kind, RESET_ALL | RESET_ITEM) {
            draft.item = None;
        }
        if matches!(kind, RESET_ALL | RESET_ZENY) {
            draft.zeny = 0;
        }
        if had_item && draft.item.is_none() {
            self.resync_inventory(character);
        }
    }

    /// `CZ_MAIL_ADD_ITEM`: index 0 attaches zeny, anything else an inventory item.
    fn attach_to_mail(&self, state: &mut ServerState, char_id: u32, index: u16, amount: u32) {
        let allowed = state
            .get_character(char_id)
            .is_some_and(|character| state.has_permission(character.account_id, Permission::Trade));
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        let replaced_item = character.game_systems.mail_draft.item.is_some();
        let accepted = allowed && !character.game_systems.is_trading() && Self::fill_draft(character, index, amount);
        if accepted && index != 0 && replaced_item {
            // The client shows an item again only when the inventory is resent.
            self.resync_inventory(character);
        }
        self.send_raw(char_id, wire::mail_draft_result(index, u8::from(!accepted)));
    }

    fn fill_draft(character: &mut Character, index: u16, amount: u32) -> bool {
        if index == 0 {
            character.game_systems.mail_draft.zeny = amount.min(character.status.zeny);
            return true;
        }
        let Some(item) = character.get_item_from_inventory(usize::from(index)) else { return false };
        let Some(model) = GlobalConfigService::instance().find_item(item.item_id) else { return false };
        let Ok(amount) = i16::try_from(amount) else { return false };
        let unmailable = model.trade_flags & ItemTradeFlag::NoMail.as_flag() != 0
            || (model.item_type == ItemType::PetEgg && item.card0 == HATCHED_EGG_CARD);
        if amount < 1 || amount > item.amount || item.equip != 0 || unmailable || (!model.item_type.is_stackable() && amount != 1) {
            return false;
        }
        let attached = (usize::from(index), amount, item.item_id);
        character.game_systems.mail_draft.item = Some(attached);
        true
    }

    fn send_mail(&self, state: &mut ServerState, char_id: u32, tick: u128, recipient: &str, title: String, body: String) {
        let failed = self.try_send_mail(state, char_id, tick, recipient, title, body).is_err();
        self.reset_mail_draft(state, char_id, RESET_ALL);
        self.send_raw(char_id, wire::mail_send_result(failed));
    }

    fn try_send_mail(&self, state: &mut ServerState, char_id: u32, tick: u128, recipient: &str, title: String, body: String) -> Result<(), ()> {
        let config = &self.configuration.game.mail;
        let character = state.get_character(char_id).ok_or(())?;
        let systems = &character.game_systems;
        if systems.is_trading() || title.is_empty() {
            return Err(());
        }
        if tick < systems.last_mail_tick + u128::from(config.delay_ms) {
            self.tell(char_id, "Cannot send mails too fast!!");
            return Err(());
        }
        let today = now_seconds() / SECONDS_PER_DAY;
        let sent_today = if systems.mails_sent_today.0 == today { systems.mails_sent_today.1 } else { 0 };
        if config.daily_count > 0 && sent_today >= config.daily_count {
            return Err(());
        }
        let destination = self
            .repository
            .char_find_by_name(recipient)
            .ok()
            .flatten()
            .filter(|record| record.account_id as u32 != character.account_id)
            .ok_or(())?;
        let draft = systems.mail_draft.clone();
        let item = match draft.item {
            Some((index, amount, item_id)) => {
                let held = character.get_item_from_inventory(index).filter(|held| held.item_id == item_id && held.amount >= amount);
                Some((index, amount, mail_item_of(held.ok_or(())?, amount)))
            }
            None => None,
        };
        let fee = u64::from(draft.zeny) * u64::from(config.zeny_fee_percent) / 100;
        let price = if item.is_some() { u64::from(config.attachment_price) } else { 0 };
        let total = u64::from(draft.zeny) + fee + price;
        if total > u64::from(character.status.zeny) {
            return Err(());
        }
        let message = MailMessage {
            sender_id: char_id,
            sender_name: character.name.clone(),
            dest_id: destination.char_id as u32,
            dest_name: destination.name.clone(),
            title: title.chars().take(MAIL_TITLE_LENGTH).collect(),
            body: body.chars().take(MAIL_BODY_LENGTH).collect(),
            timestamp: now_seconds(),
            zeny: draft.zeny,
            item: item.as_ref().map(|(_, _, item)| item.clone()),
            ..MailMessage::default()
        };
        let remaining_zeny = character.status.zeny - total as u32;
        let paid = state
            .with_character_taken(char_id, |_, character| {
                self.charge_for_mail(character, remaining_zeny, item.as_ref().map(|(index, amount, _)| (*index, *amount)))
            })
            .unwrap_or(false);
        if !paid {
            return Err(());
        }
        match self.repository.mail_deliver(message) {
            Ok(delivered) => {
                if let Some(character) = state.characters_mut().get_mut(&char_id) {
                    character.game_systems.last_mail_tick = tick;
                    character.game_systems.mails_sent_today = (today, sent_today + 1);
                }
                self.send_raw(delivered.dest_id, wire::mail_received(delivered.id, &delivered.title, &delivered.sender_name));
                Ok(())
            }
            Err(error) => {
                warn!("Mail of {char_id} could not be stored: {error}");
                state.with_character_taken(char_id, |_, character| {
                    self.refund_mail(character, total as u32, item.map(|(_, _, item)| item));
                });
                Err(())
            }
        }
    }

    /// Takes the zeny and the attached item from the sender; false (with nothing taken) when either cannot be taken.
    fn charge_for_mail(&self, character: &mut Character, remaining_zeny: u32, item: Option<(usize, i16)>) -> bool {
        let before = character.status.zeny;
        self.character_service().update_zeny(
            self.runtime.as_ref(),
            CharacterZeny { char_id: character.char_id, zeny: Some(remaining_zeny) },
            character,
        );
        if character.status.zeny != remaining_zeny {
            return false;
        }
        let Some((index, amount)) = item else { return true };
        let removal = CharacterRemoveItems {
            char_id: character.char_id,
            sell: false,
            items: vec![CharacterRemoveItem { char_id: character.char_id, index, amount, price: 0 }],
            notify_client: true,
        };
        if self.inventory_service().remove_item_from_inventory(self.runtime.as_ref(), removal, character).is_err() {
            self.character_service()
                .update_zeny(self.runtime.as_ref(), CharacterZeny { char_id: character.char_id, zeny: Some(before) }, character);
            return false;
        }
        true
    }

    fn refund_mail(&self, character: &mut Character, zeny: u32, item: Option<MailItem>) {
        let wallet = character.status.zeny.saturating_add(zeny);
        self.character_service()
            .update_zeny(self.runtime.as_ref(), CharacterZeny { char_id: character.char_id, zeny: Some(wallet) }, character);
        if let Some(inventory_item) = item.as_ref().and_then(inventory_item_of) {
            let restore = CharacterAddItems { char_id: character.char_id, should_perform_check: false, buy: false, items: vec![inventory_item] };
            if let Err(error) = self.inventory_service().try_add_items_in_inventory(self.runtime.as_ref(), restore, character) {
                error!("Item of a failed mail could not be given back to {}: {error}", character.char_id);
            }
        }
    }

    /// `CZ_MAIL_GET_ITEM`: moves the zeny and the item of a mail to the character.
    fn take_mail_attachment(&self, state: &mut ServerState, char_id: u32, mail_id: i32) {
        let Some(mail) = self.inbox(char_id).into_iter().find(|mail| mail.id == mail_id && mail.has_attachment()) else {
            return;
        };
        let result = state
            .with_character_taken(char_id, |_, character| self.collect_attachment(character, &mail))
            .unwrap_or(1);
        self.send_raw(char_id, wire::mail_attachment_result(result));
    }

    /// 0 on success, 1 when the zeny would overflow, 2 when the item does not fit.
    fn collect_attachment(&self, character: &mut Character, mail: &MailMessage) -> u8 {
        if mail.zeny > 0 && u64::from(mail.zeny) + u64::from(character.status.zeny) > MAX_ZENY {
            return 1;
        }
        let inventory_item = mail.item.as_ref().and_then(inventory_item_of);
        if let Some(item) = &inventory_item {
            if !self.fits_in_inventory(character, item) {
                return 2;
            }
        }
        let (zeny, taken_item) = match self.repository.mail_take_attachment(character.char_id, mail.id) {
            Ok(taken) => taken,
            Err(error) => {
                warn!("Attachment of mail {} could not be taken: {error}", mail.id);
                return 1;
            }
        };
        if let Some(inventory_item) = taken_item.as_ref().and_then(inventory_item_of) {
            let add = CharacterAddItems { char_id: character.char_id, should_perform_check: false, buy: false, items: vec![inventory_item] };
            if self.inventory_service().try_add_items_in_inventory(self.runtime.as_ref(), add, character).is_err() {
                if let Err(error) = self.repository.mail_restore_attachment(mail.id, zeny, taken_item) {
                    error!("Attachment of mail {} was lost: {error}", mail.id);
                }
                return 2;
            }
        }
        if zeny > 0 {
            let wallet = character.status.zeny + zeny;
            self.character_service()
                .update_zeny(self.runtime.as_ref(), CharacterZeny { char_id: character.char_id, zeny: Some(wallet) }, character);
        }
        0
    }

    fn fits_in_inventory(&self, character: &Character, item: &InventoryItemModel) -> bool {
        let weight = u64::from(character.weight()) + item.weight.max(0) as u64 * item.amount.max(0) as u64;
        if weight > u64::from(self.character_service().max_weight(character)) {
            return false;
        }
        let stacks_onto_existing = item.item_type().is_stackable()
            && character.inventory.iter().flatten().any(|held| {
                held.item_id == item.item_id && held.unique_id == item.unique_id && held.refine == item.refine
                    && held.amount.checked_add(item.amount).is_some()
            });
        let used = character.inventory.iter().flatten().count();
        stacks_onto_existing || used < usize::from(self.configuration.game.max_inventory)
    }

    /// `CZ_REQ_MAIL_RETURN`
    fn return_mail(&self, char_id: u32, mail_id: i32) {
        let returnable = self.inbox(char_id).into_iter().any(|mail| mail.id == mail_id && mail.sender_id != 0);
        let returned = returnable
            .then(|| self.repository.mail_return(char_id, mail_id, now_seconds()))
            .and_then(|result| {
                result.unwrap_or_else(|error| {
                    warn!("Mail {mail_id} could not be returned: {error}");
                    None
                })
            });
        if let Some(returned) = &returned {
            self.send_raw(returned.dest_id, wire::mail_received(returned.id, &returned.title, &returned.sender_name));
        }
        self.send_raw(char_id, wire::mail_return_result(mail_id, returned.is_none()));
    }
}

impl MailDraft {
    pub fn is_empty(&self) -> bool {
        self.item.is_none() && self.zeny == 0
    }
}
