use std::collections::HashMap;
use std::sync::Arc;

use script_runtime::{Host, WasmRuntime};
use script_sdk::Entry;

use super::entries::{NAMED_BASE, resolve};

/// Runs NPC and event scripts: numeric entries go to the legacy module, named ones to the module their manifest names.
pub struct ScriptVm {
    legacy: Arc<WasmRuntime>,
    modules: HashMap<String, Arc<WasmRuntime>>,
}

impl ScriptVm {
    pub fn new(legacy: Arc<WasmRuntime>, modules: HashMap<String, Arc<WasmRuntime>>) -> Self {
        Self { legacy, modules }
    }

    /// Runs the numeric export `entry` of the legacy module.
    pub async fn execute<H: Host>(&self, host: H, entry: &str, id: u32) -> (H, Result<(), String>) {
        self.legacy.execute(host, entry, id).await
    }

    /// Runs the NPC or event `handle`. `legacy_export` is only used when the handle is a numeric entry.
    pub async fn run<H: Host>(&self, host: H, legacy_export: &str, handle: u32) -> (H, Result<(), String>) {
        if handle < NAMED_BASE {
            return self.legacy.execute(host, legacy_export, handle).await;
        }
        let Some(script) = resolve(handle) else {
            return (host, Err(format!("Unknown named script handle {handle:#x}")));
        };
        let Some(module) = self.modules.get(&script.module) else {
            return (host, Err(format!("Script module {} is not loaded", script.module)));
        };
        module.execute_named(host, &script.entry).await
    }
}

/// Runs item scripts from the items module. The legacy module still holds the pet scripts.
pub struct ItemVm {
    legacy: Arc<WasmRuntime>,
    items: Arc<WasmRuntime>,
}

impl ItemVm {
    pub fn new(legacy: Arc<WasmRuntime>, items: Arc<WasmRuntime>) -> Self {
        Self { legacy, items }
    }

    /// The fingerprint of the items module, which must match the item manifest.
    pub fn catalog_hash(&self) -> Result<u64, String> {
        self.items.catalog_hash()
    }

    /// Runs the use script of item `id`, or its passive script when the item is worn.
    pub async fn run_item<H: Host>(&self, host: H, id: u32) -> (H, Result<(), String>) {
        self.items.execute_named(host, &Entry::Item(id)).await
    }

    /// Runs the automatic bonus or visual program `id` of an item.
    pub async fn run_program<H: Host>(&self, host: H, id: u32) -> (H, Result<(), String>) {
        self.items.execute_named(host, &Entry::Program(id)).await
    }

    /// Runs a numeric export of the legacy module, such as the pet scripts.
    pub async fn execute<H: Host>(&self, host: H, entry: &str, id: u32) -> (H, Result<(), String>) {
        self.legacy.execute(host, entry, id).await
    }
}
