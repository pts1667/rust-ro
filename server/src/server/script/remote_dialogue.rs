use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use packets::packets::{Packet, PacketZcCloseDialog};
use tokio::sync::mpsc;

use super::PlayerInput;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::session::Session;

const INPUT_QUEUE: usize = 4;

/// The conversation of a script with a player it attached with `attachrid` and that is not the player who started it.
/// The attached player's client input is routed to the script until the conversation is released.
#[derive(Default)]
pub struct RemoteDialogue {
    active: Option<RemoteConversation>,
}

struct RemoteConversation {
    char_id: u32,
    npc_id: u32,
    session: Arc<Session>,
    generation: u64,
    inputs: mpsc::Receiver<PlayerInput>,
    window_open: bool,
    notifications: SyncSender<Notification>,
    packetver: u32,
}

impl RemoteDialogue {
    /// Starts routing the client input of `char_id` to the script, unless that is already done.
    pub fn open(&mut self, sessions: &crate::server::model::session::SessionRegistry, char_id: u32, npc_id: u32, notifications: &SyncSender<Notification>, packetver: u32) -> Result<(), String> {
        if self.active.as_ref().is_some_and(|conversation| conversation.char_id == char_id) {
            return Ok(());
        }
        self.release();
        let session = sessions.find_by_char_id(char_id).ok_or("Attached player is offline")?;
        if session.script_handler_channel_sender.lock().unwrap().is_some() {
            return Err("Attached player is in another conversation".into());
        }
        let (sender, inputs) = mpsc::channel(INPUT_QUEUE);
        let generation = session.set_script_handler_channel_sender(sender);
        self.active = Some(RemoteConversation { char_id, npc_id, session, generation, inputs, window_open: false, notifications: notifications.clone(), packetver });
        Ok(())
    }

    pub fn set_window_open(&mut self, open: bool) {
        if let Some(conversation) = self.active.as_mut() {
            conversation.window_open = open;
        }
    }

    pub async fn receive(&mut self) -> Result<PlayerInput, String> {
        let conversation = self.active.as_mut().ok_or("No conversation with the attached player")?;
        conversation.inputs.recv().await.ok_or_else(|| "Attached player disconnected or started another conversation".into())
    }

    /// Stops routing input and closes the window the script left open on the attached player's client.
    pub fn release(&mut self) {
        let Some(conversation) = self.active.take() else { return };
        conversation.session.finish_script(conversation.generation);
        if conversation.window_open {
            let mut packet = PacketZcCloseDialog::new(conversation.packetver);
            packet.naid = conversation.npc_id;
            packet.fill_raw();
            let _ = conversation.notifications.try_send(Notification::Char(CharNotification::new(conversation.char_id, packet.raw)));
        }
    }
}

impl Drop for RemoteDialogue {
    fn drop(&mut self) {
        self.release();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::mpsc::sync_channel;

    use super::*;
    use crate::server::model::session::SessionRegistry;

    fn registry_with(char_id: u32) -> (SessionRegistry, Arc<Session>) {
        let registry = SessionRegistry::default();
        let session = Arc::new(Session::create_empty(2000000 + char_id, 0, 0, 20120307).recreate_with_character(char_id));
        registry.add(session.account_id, session.clone());
        (registry, session)
    }

    #[test]
    fn input_of_the_attached_player_reaches_the_script_until_released() {
        let (registry, session) = registry_with(150001);
        let (notifications, delivered) = sync_channel(8);
        let mut dialogue = RemoteDialogue::default();
        dialogue.open(&registry, 150001, 600, &notifications, 20120307).unwrap();
        dialogue.open(&registry, 150001, 600, &notifications, 20120307).unwrap();
        dialogue.set_window_open(true);

        let sender = session.script_handler_channel_sender.lock().unwrap().clone().unwrap();
        sender.try_send(PlayerInput::Next).unwrap();
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        assert!(matches!(runtime.block_on(dialogue.receive()), Ok(PlayerInput::Next)));

        dialogue.release();
        assert!(session.script_handler_channel_sender.lock().unwrap().is_none());
        assert!(matches!(delivered.try_recv(), Ok(Notification::Char(notification)) if notification.char_id() == 150001));
        assert!(runtime.block_on(dialogue.receive()).is_err());
    }

    #[test]
    fn a_player_in_another_conversation_or_offline_cannot_be_attached() {
        let (registry, session) = registry_with(150002);
        let (notifications, _delivered) = sync_channel(8);
        let (sender, _inputs) = mpsc::channel(1);
        session.set_script_handler_channel_sender(sender);
        let mut dialogue = RemoteDialogue::default();
        assert!(dialogue.open(&registry, 150002, 600, &notifications, 20120307).is_err());
        assert!(dialogue.open(&registry, 999, 600, &notifications, 20120307).is_err());
    }
}
