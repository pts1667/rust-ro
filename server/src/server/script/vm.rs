use std::collections::HashMap;
use std::sync::Arc;

use script_runtime::{Host, WasmRuntime};
use script_sdk::Entry;

use super::entries::resolve;
use crate::server::plugins::Plugins;

/// Runs NPC and event scripts in the module their manifest names, plugin scripts included.
pub struct ScriptVm {
    modules: HashMap<String, Arc<WasmRuntime>>,
    plugins: Arc<Plugins>,
}

impl ScriptVm {
    pub fn new(modules: HashMap<String, Arc<WasmRuntime>>) -> Self {
        Self::with_plugins(modules, Plugins::default())
    }

    /// The plugins' modules join the base ones under their `plugin:<name>` keys.
    pub fn with_plugins(mut modules: HashMap<String, Arc<WasmRuntime>>, plugins: Plugins) -> Self {
        modules.extend(plugins.modules());
        Self { modules, plugins: Arc::new(plugins) }
    }

    pub fn plugins(&self) -> &Arc<Plugins> {
        &self.plugins
    }

    /// Runs `entry` of `module` without interning it: hook entries carry their data and are never run twice.
    pub async fn run_entry<H: Host>(&self, host: H, module: &str, entry: &Entry) -> (H, Result<(), String>) {
        let Some(runtime) = self.modules.get(module) else {
            return (host, Err(format!("Script module {module} is not loaded")));
        };
        runtime.execute_named(host, entry).await
    }

    /// Runs the NPC or event `handle`.
    pub async fn run<H: Host>(&self, host: H, handle: u32) -> (H, Result<(), String>) {
        let Some(script) = resolve(handle) else {
            return (host, Err(format!("Unknown named script handle {handle:#x}")));
        };
        let Some(module) = self.modules.get(&script.module) else {
            return (host, Err(format!("Script module {} is not loaded", script.module)));
        };
        module.execute_named(host, &script.entry).await
    }
}

/// Runs item scripts from the items module and pet scripts from the pets module.
pub struct ItemVm {
    items: Arc<WasmRuntime>,
    pets: Arc<WasmRuntime>,
}

impl ItemVm {
    pub fn new(items: Arc<WasmRuntime>, pets: Arc<WasmRuntime>) -> Self {
        Self { items, pets }
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

    /// Runs the passive bonus script of the pet class `id`.
    pub async fn run_pet<H: Host>(&self, host: H, id: u32) -> (H, Result<(), String>) {
        self.pets.execute_named(host, &Entry::Pet(id)).await
    }

    /// Runs the support script of the pet class `id`.
    pub async fn run_pet_support<H: Host>(&self, host: H, id: u32) -> (H, Result<(), String>) {
        self.pets.execute_named(host, &Entry::PetSupport(id)).await
    }

    /// Runs the automatic bonus program `id` of a pet.
    pub async fn run_pet_program<H: Host>(&self, host: H, id: u32) -> (H, Result<(), String>) {
        self.pets.execute_named(host, &Entry::PetProgram(id)).await
    }
}
