use std::collections::HashMap;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};

use script_runtime::WasmRuntime;
use tokio::runtime::Runtime;

use crate::repository::ItemRepository;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::{CharacterAddItems, GameEvent};
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::Value;
use crate::server::service::global_config_service::GlobalConfigService;

#[allow(dead_code)]
pub struct ScriptService {
    client_notification_sender: SyncSender<Notification>,
    pub(crate) configuration_service: &'static GlobalConfigService,
    repository: Arc<dyn ItemRepository>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    pub vm: Arc<WasmRuntime>,
    pub(crate) npc_variables: Mutex<HashMap<(u32, u8, u32, String, u32), Value>>,
}

impl ScriptService {
    pub fn event_entry(label: &str) -> Option<u32> {
        static EVENTS: std::sync::OnceLock<HashMap<String, u32>> = std::sync::OnceLock::new();
        EVENTS.get_or_init(|| serde_json::from_str(include_str!("../../../../config/wasm/events.json")).expect("Invalid compiled script event registry")).get(label).copied()
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
        vm: Arc<WasmRuntime>,
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
