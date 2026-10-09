use std::sync::atomic::Ordering;

use packets::packets::{Packet, PacketZcCloseDialog};
use tokio::sync::mpsc;

use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::NpcContact;
use crate::server::script::NpcScriptHost;
use crate::server::state::server::ServerState;
use crate::server::Server;

impl Server {
    pub(crate) fn start_npc_conversation(&self, state: &ServerState, contact: NpcContact) -> Result<(), String> {
        let character = state.characters().get(&contact.char_id).filter(|character| character.account_id == contact.account_id && !character.is_dead() && character.status.hp > 0 && !character.game_systems.is_trading() && !character.timing.skill_menu_blocked())
            .ok_or("NPC visitor is unavailable")?;
        let session = state.find_session(contact.account_id).filter(|session| session.char_id == Some(contact.char_id)).ok_or("NPC session expired")?;
        let map_name = character.current_map_name();
        let map_instance = character.current_map_instance();
        let map_item = state.map_item(contact.npc_id, map_name, map_instance).ok_or("NPC is not on this map")?;
        let script = state.map_item_script(&map_item, map_name, map_instance).ok_or("NPC has no compiled program")?;
        if state.get_map_instance(map_name, map_instance).is_some_and(|instance| instance.state().script_skill_state.npcs.get(&contact.npc_id).is_some_and(|npc| npc.hidden)) {
            return Err("NPC is disabled".into());
        }
        if character.x.abs_diff(script.x).max(character.y.abs_diff(script.y)) > crate::server::PLAYER_FOV { return Err("NPC is out of range".into()); }
        let server = self.shared().ok_or("NPC runtime is not bound")?;
        let (sender, receiver) = mpsc::channel(4);
        let generation = session.set_script_handler_channel_sender(sender);
        let notifications = self.server_service().notification_sender();
        let host = NpcScriptHost { server, session: session.clone(), script: script.clone(), inputs: receiver,
            notifications: notifications.clone(), generation, background: false, event_depth: 0, event_arguments: None, timer_context: None, logout_token: None, map_instance, dialog_open: false, attached: None, error: None };
        let vm = self.script_service().vm.clone();
        let entry = script.entry_id;
        let npc_id = script.id;
        let packetver = self.packetver();
        let timeout = std::time::Duration::from_secs(self.configuration.scripting.conversation_timeout_secs.max(1));
        let npc_name = script.name.clone();
        let char_id = session.char_id();
        let started = std::time::Instant::now();
        script_debug!("NPC conversation started: npc={npc_id} ({npc_name}) entry={entry} char={char_id}");
        self.runtime().spawn(async move {
            let (error, dialog_open) = match tokio::time::timeout(timeout, vm.run(host, entry)).await {
                Ok((host, result)) => (result.err().map(|error| host.error.unwrap_or(error)), host.dialog_open),
                Err(_) => (Some("NPC conversation timed out".into()), true),
            };
            let elapsed_ms = started.elapsed().as_millis();
            match &error {
                Some(error) => script_debug!("NPC conversation failed: npc={npc_id} ({npc_name}) entry={entry} char={char_id} after {elapsed_ms}ms: {error}"),
                None => script_debug!("NPC conversation finished: npc={npc_id} ({npc_name}) entry={entry} char={char_id} after {elapsed_ms}ms"),
            }
            // rAthena's `end` closes the dialogue too; a script ending after `next` (Hoffman's warp) would leave the window open.
            if (error.is_some() || dialog_open) && session.script_generation.load(Ordering::Acquire) == generation {
                let mut packet = PacketZcCloseDialog::new(packetver); packet.naid = npc_id; packet.fill_raw();
                let _ = notifications.try_send(Notification::Char(CharNotification::new(session.char_id(), packet.raw)));
            }
            session.finish_script(generation);
        });
        Ok(())
    }
}
