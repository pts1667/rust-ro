use std::sync::Arc;
use std::sync::atomic::Ordering;

use packets::packets::{Packet, PacketZcCloseDialog, PacketZcShowImage2};
use tokio::sync::mpsc;

use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterUseItem, GameEvent, ItemScriptComplete};
use crate::server::model::script::Script;
use crate::server::script::item_dialog::ItemDialogHost;
use crate::server::script::item_script_handler::ItemScriptHost;
use crate::server::script::NpcScriptHost;
use crate::server::service::item_service::ItemService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

const ITEM_DIALOG_NPC: u32 = 4_000_000_000;

impl ItemService {
    pub(crate) fn start_item_dialog(&self, server: &Server, state: &ServerState, character: &Character, action: CharacterUseItem, item: &InventoryItemModel, host: ItemScriptHost) -> Result<(), String> {
        let session = state.find_session(character.account_id).ok_or("Item session is unavailable")?;
        if session.script_handler_channel_sender.lock().unwrap().is_some() { return Err("Another conversation is already active".into()); }
        let server = server.shared().ok_or("Item runtime is not bound")?;
        let (sender, receiver) = mpsc::channel(4);
        let generation = session.set_script_handler_channel_sender(sender);
        let source = Self::consumption(item, 1);
        let entry = item.item_id as u32;
        let dialog = NpcScriptHost { server: server.clone(), session, script: Arc::new(Script {
            id: ITEM_DIALOG_NPC, scope_instance: character.current_map_instance(), entry_id: entry, name: format!("Item{entry}"), map_name: character.current_map_name().clone(),
            sprite: 0, x: character.x, y: character.y, dir: 0, x_size: 0, y_size: 0, constructor_args: vec![],
        }), inputs: receiver, notifications: self.client_notification_sender.clone(), generation,
            map_instance: character.current_map_instance(), background: false, event_depth: 0, event_arguments: None, timer_context: None, logout_token: None, dialog_open: false, error: None };
        let host = ItemDialogHost { item: host, dialog };
        let vm = self.item_script_vm.clone();
        let timeout = std::time::Duration::from_secs(server.configuration.scripting.conversation_timeout_secs.max(1));
        let task_server = server.clone();
        server.runtime().spawn(async move {
            let (effects, error) = match tokio::time::timeout(timeout, vm.execute(host, "run_item", entry)).await {
                Ok((host, result)) => (host.item.effects, result.err().map(|error| host.item.error.unwrap_or(error))),
                Err(_) => (vec![], Some("Item conversation timed out".into())),
            };
            task_server.add_to_next_tick(GameEvent::ItemScriptComplete(ItemScriptComplete { action, source, generation, effects, error }));
        });
        Ok(())
    }

    pub(crate) fn complete_item_dialog(&self, server: &Server, state: &mut ServerState, completion: ItemScriptComplete) -> Result<(), String> {
        let Some(character) = state.characters().get(&completion.action.char_id) else { return Ok(()); };
        let Some(session) = state.find_session(character.account_id) else { return Ok(()); };
        if session.char_id != Some(character.char_id) || session.script_generation.load(Ordering::Acquire) != completion.generation { return Ok(()); }
        let mut character = state.characters_mut().remove(&completion.action.char_id).unwrap();
        let result = (|| {
            if let Some(error) = completion.error { return Err(error); }
            let item = character.get_item_from_inventory(completion.action.index).cloned().ok_or("Item was removed during the conversation")?;
            if !item_identity_matches(&item, &completion.source) { return Err("Item changed during the conversation".into()); }
            self.finish_item_effects_in_state(server, state, server.runtime(), &mut character, &completion.action, &item, completion.effects)
        })();
        let count = character.get_item_from_inventory(completion.action.index).filter(|item| item.id == completion.source.inventory_id).map_or(0, |item| item.amount);
        self.notify_use(&character, &completion.action, count, result.is_ok());
        let mut close = PacketZcCloseDialog::new(server.packetver()); close.naid = ITEM_DIALOG_NPC; close.fill_raw();
        let _ = self.client_notification_sender.try_send(Notification::Char(CharNotification::new(character.char_id, close.raw)));
        let mut cutin = PacketZcShowImage2::new(server.packetver()); cutin.set_image_name([char::from(0); 64]); cutin.set_atype(255); cutin.fill_raw();
        let _ = self.client_notification_sender.try_send(Notification::Char(CharNotification::new(character.char_id, cutin.raw)));
        session.finish_script(completion.generation);
        state.insert_character(character);
        result
    }
}

fn item_identity_matches(item: &InventoryItemModel, source: &crate::repository::script_inventory_repository::ScriptItemConsumption) -> bool {
    item.id == source.inventory_id && item.item_id == source.item_id && item.unique_id == source.unique_id && item.amount >= source.amount && item.equip == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogue_source_validation_rejects_replaced_instances_and_exhausted_stacks() {
        let model = serde_json::from_value(serde_json::json!({ "id":501, "name_aegis":"Red_Potion", "name_english":"Red Potion",
            "item_type":"Healing", "weight":0, "job_flags":0, "class_flags":0, "location":0, "flags":0, "trade_flags":0 })).unwrap();
        let mut item = InventoryItemModel::from_item_model(&model, 1, true);
        item.id = 10; item.unique_id = 7;
        let source = ItemService::consumption(&item, 1);
        assert!(item_identity_matches(&item, &source));
        item.unique_id = 8; assert!(!item_identity_matches(&item, &source));
        item.unique_id = 7; item.amount = 0; assert!(!item_identity_matches(&item, &source));
        item.amount = 1; item.item_id = 502; assert!(!item_identity_matches(&item, &source));
    }
}
