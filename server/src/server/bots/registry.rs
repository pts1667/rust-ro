use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use dashmap::DashMap;
use serde::Serialize;

use super::dialog::{self, Decoded, Dialog};

const MESSAGE_BACKLOG: usize = 50;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BotMessage {
    /// Grows with every message of the bot, lets a client skip the ones it has seen.
    pub seq: u64,
    pub text: String,
}

#[derive(Default)]
struct BotScreen {
    dialog: Option<Dialog>,
    /// Counts every change of the dialogue, so that a waiting call notices an answer that came and went.
    dialog_version: u64,
    messages: VecDeque<BotMessage>,
    next_seq: u64,
}

/// A character driven through the API, known by the name of its character.
pub struct BotHandle {
    pub name: String,
    pub account_id: u32,
    pub char_id: u32,
    connected: AtomicBool,
    command_epoch: AtomicU64,
    screen: Mutex<BotScreen>,
}

impl BotHandle {
    pub fn new(name: String, account_id: u32, char_id: u32) -> Self {
        Self { name, account_id, char_id, connected: AtomicBool::new(false), command_epoch: AtomicU64::new(0), screen: Mutex::new(BotScreen::default()) }
    }

    fn screen(&self) -> MutexGuard<'_, BotScreen> {
        self.screen.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    pub fn set_connected(&self, connected: bool) {
        self.connected.store(connected, Ordering::Release);
        if !connected {
            self.clear_dialog();
        }
    }

    /// Numbers the commands that take over the bot, so that a fight running in the background can tell it was replaced.
    pub fn next_command(&self) -> u64 {
        self.command_epoch.fetch_add(1, Ordering::AcqRel) + 1
    }

    pub fn is_current_command(&self, epoch: u64) -> bool {
        self.command_epoch.load(Ordering::Acquire) == epoch
    }

    pub fn dialog(&self) -> Option<Dialog> {
        self.screen().dialog.clone()
    }

    pub fn dialog_version(&self) -> u64 {
        self.screen().dialog_version
    }

    pub fn clear_dialog(&self) {
        let mut screen = self.screen();
        screen.dialog = None;
        screen.dialog_version += 1;
    }

    pub fn recent_messages(&self, count: usize) -> Vec<BotMessage> {
        let screen = self.screen();
        screen.messages.iter().skip(screen.messages.len().saturating_sub(count)).cloned().collect()
    }

    pub fn messages_after(&self, seq: u64) -> Vec<BotMessage> {
        self.screen().messages.iter().filter(|message| message.seq > seq).cloned().collect()
    }

    fn apply_packet(&self, packet: &[u8]) {
        let mut screen = self.screen();
        match dialog::decode(&mut screen.dialog, packet) {
            Decoded::Unrelated => {}
            Decoded::Dialog => screen.dialog_version += 1,
            Decoded::Message(text) => {
                screen.next_seq += 1;
                let seq = screen.next_seq;
                screen.messages.push_back(BotMessage { seq, text });
                if screen.messages.len() > MESSAGE_BACKLOG {
                    screen.messages.pop_front();
                }
            }
        }
    }
}

#[derive(Default)]
struct Bots {
    by_name: DashMap<String, Arc<BotHandle>>,
    by_char: DashMap<u32, Arc<BotHandle>>,
}

/// Bots of this server run, cloneable handle shared by the API threads and the notification thread.
#[derive(Clone, Default)]
pub struct BotRegistry(Arc<Bots>);

impl BotRegistry {
    fn key(name: &str) -> String {
        name.to_lowercase()
    }

    pub fn register(&self, bot: BotHandle) -> Arc<BotHandle> {
        let bot = Arc::new(bot);
        self.0.by_char.insert(bot.char_id, bot.clone());
        self.0.by_name.insert(Self::key(&bot.name), bot.clone());
        bot
    }

    pub fn find(&self, name: &str) -> Option<Arc<BotHandle>> {
        self.0.by_name.get(&Self::key(name)).map(|bot| bot.clone())
    }

    pub fn list(&self) -> Vec<Arc<BotHandle>> {
        let mut bots: Vec<_> = self.0.by_name.iter().map(|entry| entry.value().clone()).collect();
        bots.sort_by(|left, right| left.name.cmp(&right.name));
        bots
    }

    pub fn connected_count(&self) -> usize {
        self.0.by_name.iter().filter(|entry| entry.is_connected()).count()
    }

    pub fn is_bot(&self, char_id: u32) -> bool {
        self.0.by_char.contains_key(&char_id)
    }

    /// Feeds a packet the server sent to a character; does nothing for characters that are not bots.
    pub fn observe_packet(&self, char_id: u32, packet: &[u8]) {
        if let Some(bot) = self.0.by_char.get(&char_id) {
            bot.apply_packet(packet);
        }
    }
}
