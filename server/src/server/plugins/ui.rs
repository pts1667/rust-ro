//! Plugin windows (`docs/plugin-ui.md`): what each character has open, what bots observe of each plugin, and the queue that
//! runs a plugin's window handlers and bot actions one at a time per character, so two quick clicks cannot race on the
//! same zeny.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard};

use plugin_ui::wire::FEATURE_PLUGIN_UI;
use plugin_ui::{UiEvent, Window};
use script_sdk::BotAction;
use serde_json::Value as Json;

use crate::server::bots::BotReply;

pub const MAX_WINDOWS: usize = 8;
pub const MAX_QUEUED: usize = 4;
pub const MAX_OBSERVATIONS: usize = 32;
pub const MAX_OBSERVATION_BYTES: usize = 16 * 1024;

/// Something a plugin handler must run for a character. `reply` answers the bot that asked for it.
pub enum PluginJob {
    Event { event: UiEvent, reply: Option<BotReply> },
    BotAction { action: BotAction, reply: BotReply },
}

impl PluginJob {
    pub fn reply(&self) -> Option<&BotReply> {
        match self {
            PluginJob::Event { reply, .. } => reply.as_ref(),
            PluginJob::BotAction { reply, .. } => Some(reply),
        }
    }
}

/// Sends `result` to the bot waiting on `reply`, once.
pub fn answer(reply: &BotReply, result: Result<Json, String>) {
    let sender = reply.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).take();
    if let Some(sender) = sender {
        let _ = sender.send(result);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenWindow {
    pub plugin: String,
    pub window: Window,
}

#[derive(Default)]
struct CharacterUi {
    /// The client announced it draws plugin UI.
    client: bool,
    windows: Vec<OpenWindow>,
    observations: BTreeMap<String, BTreeMap<String, Json>>,
    queues: HashMap<String, Queue>,
    replies: HashMap<String, Json>,
}

#[derive(Default)]
struct Queue {
    running: bool,
    pending: VecDeque<PluginJob>,
}

#[derive(Clone, Default)]
pub struct PluginUiStore {
    characters: Arc<Mutex<HashMap<u32, CharacterUi>>>,
}

impl PluginUiStore {
    fn lock(&self) -> MutexGuard<'_, HashMap<u32, CharacterUi>> {
        self.characters.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn set_client_features(&self, char_id: u32, flags: u32) {
        self.lock().entry(char_id).or_default().client = flags & FEATURE_PLUGIN_UI != 0;
    }

    /// Whether plugin UI packets may be sent to the character's client: the stock client does not know them.
    pub fn has_client(&self, char_id: u32) -> bool {
        self.lock().get(&char_id).is_some_and(|ui| ui.client)
    }

    /// Opens `window`, or replaces the open window of the same plugin and id.
    pub fn open(&self, char_id: u32, plugin: &str, window: Window) -> Result<(), String> {
        let mut characters = self.lock();
        let ui = characters.entry(char_id).or_default();
        if let Some(open) = ui.windows.iter_mut().find(|open| open.plugin == plugin && open.window.id == window.id) {
            open.window = window;
            return Ok(());
        }
        if ui.windows.iter().filter(|open| open.plugin == plugin).count() >= MAX_WINDOWS {
            return Err(format!("Plugin {plugin} already has {MAX_WINDOWS} windows open for this player"));
        }
        ui.windows.push(OpenWindow { plugin: plugin.into(), window });
        Ok(())
    }

    pub fn close(&self, char_id: u32, plugin: &str, id: &str) -> bool {
        let mut characters = self.lock();
        let Some(ui) = characters.get_mut(&char_id) else { return false };
        let before = ui.windows.len();
        ui.windows.retain(|open| open.plugin != plugin || open.window.id != id);
        ui.windows.len() != before
    }

    pub fn window(&self, char_id: u32, plugin: &str, id: &str) -> Option<Window> {
        let characters = self.lock();
        characters.get(&char_id)?.windows.iter().find(|open| open.plugin == plugin && open.window.id == id).map(|open| open.window.clone())
    }

    pub fn windows(&self, char_id: u32) -> Vec<OpenWindow> {
        self.lock().get(&char_id).map(|ui| ui.windows.clone()).unwrap_or_default()
    }

    /// Sets, or with `None` removes, what bots observe of `plugin` under `key`.
    pub fn observe(&self, char_id: u32, plugin: &str, key: &str, value: Option<Json>) -> Result<(), String> {
        let mut characters = self.lock();
        let ui = characters.entry(char_id).or_default();
        let entries = ui.observations.entry(plugin.into()).or_default();
        match value {
            None => {
                entries.remove(key);
            }
            Some(value) => {
                let bytes = value.to_string().len();
                if bytes > MAX_OBSERVATION_BYTES {
                    return Err(format!("Observation {key} is {bytes} bytes, over {MAX_OBSERVATION_BYTES}"));
                }
                if !entries.contains_key(key) && entries.len() >= MAX_OBSERVATIONS {
                    return Err(format!("Plugin {plugin} already has {MAX_OBSERVATIONS} observations for this player"));
                }
                entries.insert(key.into(), value);
            }
        }
        if entries.is_empty() {
            ui.observations.remove(plugin);
        }
        Ok(())
    }

    pub fn observations(&self, char_id: u32) -> BTreeMap<String, BTreeMap<String, Json>> {
        self.lock().get(&char_id).map(|ui| ui.observations.clone()).unwrap_or_default()
    }

    pub fn set_reply(&self, char_id: u32, plugin: &str, value: Json) {
        self.lock().entry(char_id).or_default().replies.insert(plugin.into(), value);
    }

    pub fn take_reply(&self, char_id: u32, plugin: &str) -> Option<Json> {
        self.lock().get_mut(&char_id)?.replies.remove(plugin)
    }

    /// Queues `job`. It comes back in `Ok` when nothing of `plugin` runs for the character, to be started at once, and in
    /// `Err` when the queue is full.
    pub fn enqueue(&self, char_id: u32, plugin: &str, job: PluginJob) -> Result<Option<PluginJob>, PluginJob> {
        let mut characters = self.lock();
        let queue = characters.entry(char_id).or_default().queues.entry(plugin.into()).or_default();
        if !queue.running {
            queue.running = true;
            return Ok(Some(job));
        }
        if queue.pending.len() >= MAX_QUEUED {
            return Err(job);
        }
        queue.pending.push_back(job);
        Ok(None)
    }

    /// The run of `plugin` for the character ended: the next job to start, if any.
    pub fn finish(&self, char_id: u32, plugin: &str) -> Option<PluginJob> {
        let mut characters = self.lock();
        let queue = characters.get_mut(&char_id)?.queues.get_mut(plugin)?;
        let next = queue.pending.pop_front();
        queue.running = next.is_some();
        next
    }

    /// The character left the game: everything of theirs goes, and the jobs that will not run are handed back so the
    /// bots waiting on them can be answered.
    pub fn forget(&self, char_id: u32) -> Vec<PluginJob> {
        self.lock().remove(&char_id).map(|ui| ui.queues.into_values().flat_map(|queue| queue.pending).collect()).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use plugin_ui::Node;

    use super::*;

    fn window(id: &str) -> Window {
        Window::new(id, "W", 100, Node::text("x"))
    }

    fn event() -> PluginJob {
        PluginJob::Event { event: UiEvent::closed("w"), reply: None }
    }

    #[test]
    fn windows_are_replaced_by_id_and_limited_per_plugin() {
        let store = PluginUiStore::default();
        store.open(1, "p", window("a")).unwrap();
        store.open(1, "p", Window::new("a", "Again", 100, Node::text("y"))).unwrap();
        assert_eq!(store.windows(1).len(), 1);
        assert_eq!(store.window(1, "p", "a").unwrap().title, "Again");
        for index in 1..MAX_WINDOWS {
            store.open(1, "p", window(&index.to_string())).unwrap();
        }
        assert!(store.open(1, "p", window("over")).is_err());
        assert!(store.open(1, "q", window("over")).is_ok(), "other plugins have their own limit");
        assert!(store.close(1, "p", "a"));
        assert!(!store.close(1, "p", "a"));
    }

    #[test]
    fn jobs_run_one_at_a_time_per_character_and_plugin() {
        let store = PluginUiStore::default();
        assert!(matches!(store.enqueue(1, "p", event()), Ok(Some(_))), "an idle plugin starts at once");
        assert!(matches!(store.enqueue(1, "q", event()), Ok(Some(_))), "another plugin is not held back");
        assert!(matches!(store.enqueue(2, "p", event()), Ok(Some(_))), "nor another character");
        for _ in 0..MAX_QUEUED {
            assert!(matches!(store.enqueue(1, "p", event()), Ok(None)));
        }
        assert!(store.enqueue(1, "p", event()).is_err(), "the queue is full");
        for _ in 0..MAX_QUEUED {
            assert!(store.finish(1, "p").is_some());
        }
        assert!(store.finish(1, "p").is_none());
        assert!(matches!(store.enqueue(1, "p", event()), Ok(Some(_))), "idle again");
    }

    #[test]
    fn observations_are_bounded_and_forgotten_at_logout() {
        let store = PluginUiStore::default();
        store.observe(1, "p", "stock", Some(serde_json::json!([501]))).unwrap();
        assert!(store.observe(1, "p", "big", Some(Json::String("x".repeat(MAX_OBSERVATION_BYTES)))).is_err());
        assert_eq!(store.observations(1)["p"]["stock"], serde_json::json!([501]));
        store.observe(1, "p", "stock", None).unwrap();
        assert!(store.observations(1).is_empty());
        store.set_client_features(1, FEATURE_PLUGIN_UI);
        store.enqueue(1, "p", event()).ok();
        store.enqueue(1, "p", event()).ok();
        assert!(store.has_client(1));
        assert_eq!(store.forget(1).len(), 1, "the queued job is handed back");
        assert!(!store.has_client(1));
    }
}
