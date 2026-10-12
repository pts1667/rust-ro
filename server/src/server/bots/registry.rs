use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex, MutexGuard};

use dashmap::DashMap;
use serde::Serialize;

use super::dialog::{self, Decoded, Dialog};
use super::events::{BotEvent, BotEventKind, BotSnapshot};

const MESSAGE_BACKLOG: usize = 50;
/// Events kept for a bot: a few minutes of a busy fight.
const EVENT_BACKLOG: usize = 500;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BotMessage {
    /// Grows with every message of the bot, lets a client skip the ones it has seen.
    pub seq: u64,
    /// `system`, `chat` (heard around the bot), `whisper` or `party`.
    pub channel: &'static str,
    pub text: String,
}

#[derive(Default)]
struct BotScreen {
    dialog: Option<Dialog>,
    /// Counts every change of the dialogue, so that a waiting call notices an answer that came and went.
    dialog_version: u64,
    messages: VecDeque<BotMessage>,
    next_seq: u64,
    events: VecDeque<BotEvent>,
    next_event_seq: u64,
    /// The character at the previous game loop iteration, see `BotSnapshot`.
    snapshot: Option<BotSnapshot>,
    /// The death of the character was recorded with its killer since the last snapshot.
    death_recorded: bool,
}

impl BotScreen {
    fn push_event(&mut self, kind: BotEventKind) {
        self.next_event_seq += 1;
        self.events.push_back(BotEvent { seq: self.next_event_seq, at: now_millis(), kind });
        if self.events.len() > EVENT_BACKLOG {
            self.events.pop_front();
        }
    }
}

/// A character driven through the API, known by the name of its character.
pub struct BotHandle {
    pub name: String,
    pub account_id: u32,
    pub char_id: u32,
    connected: AtomicBool,
    command_epoch: AtomicU64,
    /// Milliseconds since the epoch of the last request made for the bot.
    last_request: AtomicU64,
    screen: Mutex<BotScreen>,
}

impl BotHandle {
    pub fn new(name: String, account_id: u32, char_id: u32) -> Self {
        Self { name, account_id, char_id, connected: AtomicBool::new(false), command_epoch: AtomicU64::new(0), last_request: AtomicU64::new(now_millis()), screen: Mutex::new(BotScreen::default()) }
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
            self.forget_snapshot();
        }
    }

    pub fn touch(&self) {
        self.last_request.store(now_millis(), Ordering::Release);
    }

    pub fn idle_for(&self) -> Duration {
        Duration::from_millis(now_millis().saturating_sub(self.last_request.load(Ordering::Acquire)))
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

    /// The events numbered after `seq`, oldest first. The oldest are dropped past `EVENT_BACKLOG`.
    pub fn events_after(&self, seq: u64) -> Vec<BotEvent> {
        self.screen().events.iter().filter(|event| event.seq > seq).cloned().collect()
    }

    /// The name and monster class of the actor `id` from the last event that names it.
    fn source_named(&self, id: u32) -> Option<(String, Option<i16>)> {
        self.screen().events.iter().rev().find_map(|event| event.kind.source_named(id))
    }

    /// The number of the last event, 0 before the first one.
    pub fn last_event(&self) -> u64 {
        self.screen().next_event_seq
    }

    fn record(&self, kind: BotEventKind) {
        self.screen().push_event(kind);
    }

    fn record_death(&self, kind: BotEventKind) {
        let mut screen = self.screen();
        screen.death_recorded = true;
        screen.push_event(kind);
    }

    /// `loaded` is false between a map change and the client's answer to it. The first snapshot waits for it, as the
    /// character is not all there before; later ones do not, or a warp taken right after another would go unseen.
    fn sample(&self, now: BotSnapshot, loaded: bool) {
        let mut screen = self.screen();
        if let Some(before) = screen.snapshot.take() {
            for kind in before.changes(&now, screen.death_recorded) {
                screen.push_event(kind);
            }
        } else if !loaded {
            return;
        }
        screen.death_recorded = false;
        screen.snapshot = Some(now);
    }

    /// Forgets the character as it was, for a bot that left the game: its next snapshot starts over.
    fn forget_snapshot(&self) {
        self.screen().snapshot = None;
    }

    fn apply_packet(&self, packet: &[u8]) {
        let mut screen = self.screen();
        match dialog::decode(&mut screen.dialog, packet) {
            Decoded::Unrelated => {}
            Decoded::Dialog => screen.dialog_version += 1,
            Decoded::Message { channel, text } => {
                screen.next_seq += 1;
                let seq = screen.next_seq;
                screen.messages.push_back(BotMessage { seq, channel, text });
                if screen.messages.len() > MESSAGE_BACKLOG {
                    screen.messages.pop_front();
                }
            }
        }
    }
}

fn now_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_millis() as u64)
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

    pub fn remove(&self, bot: &BotHandle) {
        self.0.by_char.remove(&bot.char_id);
        self.0.by_name.remove(&Self::key(&bot.name));
    }

    pub fn is_bot(&self, char_id: u32) -> bool {
        self.0.by_char.contains_key(&char_id)
    }

    /// The characters of the bots that are in the game.
    pub fn connected_char_ids(&self) -> Vec<u32> {
        self.0.by_char.iter().filter(|entry| entry.is_connected()).map(|entry| *entry.key()).collect()
    }

    /// Adds an event to the bot of `char_id`; does nothing for characters that are not bots.
    pub fn record(&self, char_id: u32, kind: BotEventKind) {
        if let Some(bot) = self.0.by_char.get(&char_id) {
            bot.record(kind);
        }
    }

    /// Adds the death of the bot of `char_id`, so that the comparison of snapshots does not tell it again.
    pub fn record_death(&self, char_id: u32, kind: BotEventKind) {
        if let Some(bot) = self.0.by_char.get(&char_id) {
            bot.record_death(kind);
        }
    }

    /// The name and monster class of the actor `id` from the events of the bot of `char_id`.
    pub fn source_named(&self, char_id: u32, id: u32) -> Option<(String, Option<i16>)> {
        self.0.by_char.get(&char_id).and_then(|bot| bot.source_named(id))
    }

    /// Compares the bot of `char_id` with its previous snapshot, see `BotSnapshot::changes`.
    pub fn sample(&self, char_id: u32, snapshot: BotSnapshot, loaded: bool) {
        if let Some(bot) = self.0.by_char.get(&char_id) {
            bot.sample(snapshot, loaded);
        }
    }

    /// Feeds a packet the server sent to a character; does nothing for characters that are not bots.
    pub fn observe_packet(&self, char_id: u32, packet: &[u8]) {
        if let Some(bot) = self.0.by_char.get(&char_id) {
            bot.apply_packet(packet);
        }
    }
}
