use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use script_sdk::Entry;
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::repository::ItemRepository;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::entries::intern;
use crate::server::script::{ScriptVm, Value};
use crate::server::service::global_config_service::GlobalConfigService;

#[derive(Deserialize)]
#[serde(untagged)]
enum EventManifestEntry {
    Legacy(u32),
    Named { module: String, entry: String },
}

#[allow(dead_code)]
pub struct ScriptService {
    client_notification_sender: SyncSender<Notification>,
    pub(crate) configuration_service: &'static GlobalConfigService,
    repository: Arc<dyn ItemRepository>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    pub vm: Arc<ScriptVm>,
    pub(crate) npc_variables: Mutex<HashMap<(u32, u8, u32, String, u32), Value>>,
}

impl ScriptService {
    fn compiled_events() -> &'static HashMap<String, u32> {
        static EVENTS: std::sync::OnceLock<HashMap<String, u32>> = std::sync::OnceLock::new();
        EVENTS.get_or_init(|| {
            let manifest: HashMap<String, EventManifestEntry> =
                serde_json::from_str(include_str!("../../../../config/wasm/events.json")).expect("Invalid compiled script event registry");
            manifest
                .into_iter()
                .map(|(label, entry)| {
                    let handle = match entry {
                        EventManifestEntry::Legacy(entry_id) => entry_id,
                        EventManifestEntry::Named { module, entry } => intern(&module, Entry::Event(entry)),
                    };
                    (label, handle)
                })
                .collect()
        })
    }

    pub fn event_entry(label: &str) -> Option<u32> {
        let events = Self::compiled_events();
        events.get(label).copied().or_else(|| {
            let (name, event) = label.split_once("::")?;
            events.get(&format!("{}::{event}", base_npc_name(name)?)).copied()
        })
    }

    pub(crate) fn npc_timer_entries(name: &str) -> Vec<(u64, u32)> {
        let name = if Self::compiled_events().keys().any(|label| label.starts_with(&format!("{name}::"))) { name } else { base_npc_name(name).unwrap_or(name) };
        let prefix = format!("{name}::OnTimer");
        let mut entries = Self::compiled_events().iter().filter_map(|(label, entry_id)| {
            let suffix = label.strip_prefix(&prefix)?;
            if suffix.is_empty() || !suffix.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
            let time = suffix.parse::<u64>().ok().filter(|time| *time > 0 && *time <= i32::MAX as u64)?;
            Some((time, *entry_id))
        }).collect::<Vec<_>>();
        entries.sort_unstable();
        entries.dedup_by_key(|entry| entry.0);
        entries
    }

    pub(crate) fn install_temporary_variables(&self, char_id: u32, variables: &[script_sdk::Variable]) {
        let mut stored = self.npc_variables.lock().unwrap();
        for variable in variables {
            if variable.scope == script_sdk::VariableScope::CharacterTemporary {
                stored.insert((2, 0, char_id, variable.name.clone(), variable.index), variable.value.clone());
            } else if variable.scope == script_sdk::VariableScope::ServerTemporary {
                stored.insert((3, 0, 0, variable.name.clone(), variable.index), variable.value.clone());
            }
        }
    }

    pub(crate) fn new(
        client_notification_sender: SyncSender<Notification>,
        configuration_service: &'static GlobalConfigService,
        repository: Arc<dyn ItemRepository>,
        server_task_queue: Arc<TasksQueue<GameEvent>>,
        vm: Arc<ScriptVm>,
    ) -> Self {
        ScriptService {
            client_notification_sender,
            configuration_service,
            repository,
            server_task_queue,
            vm,
            npc_variables: Mutex::new(HashMap::new()),
        }
    }

    pub fn schedule_get_items(&self, char_id: u32, runtime: &Runtime, item_ids_amounts: Vec<(Value, i16)>, buy: bool) {
        let mut items = runtime
            .block_on(async {
                self.repository
                    .get_items(item_ids_amounts.iter().map(|(v, _)| v.clone()).collect())
                    .await
            })
            .unwrap();
        items.iter_mut().for_each(|item| {
            item.amount = item_ids_amounts
                .iter()
                .find(|(id, _amount)| match id {
                    Value::Number(v) => *v == item.id,
                    Value::String(v) => v.to_lowercase() == item.name_aegis.to_lowercase(),
                    _ => false,
                })
                .unwrap()
                .1
        });
        self.server_task_queue
            .add_to_first_index(GameEvent::CharacterAddItems(CharacterAddItems {
                char_id,
                should_perform_check: true,
                buy,
                items: items
                    .iter()
                    .map(|item| InventoryItemModel::from_item_model(self.configuration_service.get_item(item.id), item.amount, true))
                    .collect(),
            }));
    }
}

/// Name of the NPC a memorial dungeon copy (`Name_12`) was duplicated from.
pub(crate) fn base_npc_name(name: &str) -> Option<&str> {
    let (base, suffix) = name.rsplit_once('_')?;
    (!suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())).then_some(base)
}
