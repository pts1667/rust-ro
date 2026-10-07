use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use models::enums::bonus::BonusType;
use models::status::Status;
use script_runtime::WasmRuntime;
use serde::Deserialize;

use crate::repository::ItemRepository;
use crate::repository::model::item_model::ItemModel;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::script::item_script_handler::ItemScriptHost;
use crate::server::service::global_config_service::GlobalConfigService;

#[allow(dead_code)]
pub struct ItemService {
    pub(crate) client_notification_sender: SyncSender<Notification>,
    pub(crate) persistence_event_sender: SyncSender<PersistenceEvent>,
    pub(crate) repository: Arc<dyn ItemRepository>,
    pub(crate) configuration_service: &'static GlobalConfigService,
    pub(crate) item_script_vm: Arc<WasmRuntime>,
}

#[derive(Deserialize)]
pub(crate) struct ItemScript {
    id: u32,
    pub(crate) dynamic: bool,
    source_hash: String,
    #[serde(default)]
    pub reads: Vec<String>,
    #[serde(default)]
    pub calls: Vec<String>,
    #[serde(default)]
    pub interactive: bool,
}

impl ItemService {
    pub(crate) fn script_metadata(item_id: u32) -> Option<&'static ItemScript> {
        static METADATA: std::sync::OnceLock<std::collections::HashMap<u32, ItemScript>> = std::sync::OnceLock::new();
        METADATA.get_or_init(|| serde_json::from_str::<Vec<ItemScript>>(include_str!("../../../../config/wasm/items.json"))
            .expect("Invalid compiled item manifest").into_iter().map(|entry| (entry.id, entry)).collect()).get(&item_id)
    }

    pub fn new(
        client_notification_sender: SyncSender<Notification>,
        persistence_event_sender: SyncSender<PersistenceEvent>,
        repository: Arc<dyn ItemRepository>,
        item_script_vm: Arc<WasmRuntime>,
        configuration_service: &'static GlobalConfigService,
    ) -> Self {
        Self {
            client_notification_sender,
            persistence_event_sender,
            repository,
            item_script_vm,
            configuration_service,
        }
    }

    pub fn convert_script_into_bonuses(items: &mut Vec<ItemModel>, vm: Arc<WasmRuntime>) -> (i32, i32) {
        Self::load_item_scripts(
            items,
            vm,
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/items.json"),
        )
    }

    pub fn load_item_scripts(items: &mut Vec<ItemModel>, vm: Arc<WasmRuntime>, path: impl AsRef<std::path::Path>) -> (i32, i32) {
        let metadata_bytes: Vec<u8> = std::fs::read(path)
            .expect("Cannot load item script manifest")
            .into_iter()
            .filter(|byte| *byte != b'\r')
            .collect();
        let digest = md5::compute(&metadata_bytes);
        let fingerprint = u64::from_le_bytes(digest.0[..8].try_into().unwrap());
        assert_eq!(
            vm.catalog_hash().expect("Wasm module has no item catalog fingerprint"),
            fingerprint,
            "Wasm module and item manifest do not match; rebuild scripts"
        );
        let metadata: Vec<ItemScript> = serde_json::from_slice(&metadata_bytes).expect("Invalid item script manifest");
        let metadata: std::collections::HashMap<_, _> = metadata.into_iter().map(|entry| (entry.id, entry)).collect();
        let mut cached = 0;
        let mut dynamic = 0;
        for item in items {
            let Some(script) = metadata.get(&(item.id as u32)) else {
                if item.script.as_ref().is_some_and(|source| !source.is_empty()) {
                    panic!("Item {} has no compiled Wasm script", item.id);
                }
                continue;
            };
            assert_eq!(
                item.script
                    .as_ref()
                    .map(|source| format!("{:x}", md5::compute(source.as_bytes())))
                    .as_ref(),
                Some(&script.source_hash),
                "Item {} does not match the compiled script manifest",
                item.id
            );
            item.item_bonuses_are_dynamic = script.dynamic;
            if script.dynamic {
                dynamic += 1;
                continue;
            }
            let (host, result) = futures::executor::block_on(vm.execute(
                ItemScriptHost::bonuses(Status::default(), item.id as u32),
                "run_item",
                item.id as u32,
            ));
            if let Err(error) = result {
                panic!("Cannot load static item bonuses {}: {}", item.id, host.error.unwrap_or(error));
            }
            item.bonuses = host.bonuses.drain();
            for bonus in &item.bonuses {
                if let BonusType::ElementWeapon(element) = bonus {
                    item.element = Some(*element);
                }
            }
            cached += 1;
        }
        (cached, dynamic)
    }
}
