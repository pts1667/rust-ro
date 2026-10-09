use std::collections::HashMap;
use std::sync::Arc;

use script_runtime::{Host, WasmRuntime};

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
