use std::time::{Duration, Instant};

use script_sdk::{Interception, PLUGIN_BUFFER_BYTES, PluginInfo, Verdict};
use wasmtime::{Engine, Instance, Linker, Memory, Module, Store, StoreLimits, StoreLimitsBuilder, TypedFunc};

use crate::{MAX_TABLE_ELEMENTS, WasmRuntime};

/// A warm instance of a plugin for the calls the game loop waits on: `plugin_info` at boot and interceptors. It is linked
/// without host functions, so a guest that calls the host traps, and every call has an epoch deadline. Interceptors cannot
/// ask for the plugin's configuration, so each new instance receives it through `plugin_configure` when it exports one.
pub struct PluginSandbox {
    engine: Engine,
    module: Module,
    memory_bytes: usize,
    config: String,
    live: Option<Live>,
}

struct Live {
    store: Store<StoreLimits>,
    memory: Memory,
    buffer: usize,
    instance: Instance,
}

impl PluginSandbox {
    pub fn new(runtime: &WasmRuntime) -> Self {
        Self {
            engine: runtime.engine.clone(),
            module: runtime.module.clone(),
            memory_bytes: runtime.limits.memory_bytes,
            config: runtime.plugin().map(|plugin| plugin.config.clone()).unwrap_or_default(),
            live: None,
        }
    }

    fn live(&mut self) -> Result<&mut Live, String> {
        if self.live.is_none() {
            let limits = StoreLimitsBuilder::new()
                .memory_size(self.memory_bytes)
                .memories(1)
                .instances(1)
                .tables(1)
                .table_elements(MAX_TABLE_ELEMENTS)
                .build();
            let mut store = Store::new(&self.engine, limits);
            store.limiter(|limits| limits);
            store.set_epoch_deadline(u64::from(u16::MAX));
            let mut linker = Linker::new(&self.engine);
            linker.define_unknown_imports_as_traps(&self.module).map_err(|e| e.to_string())?;
            let instance = linker.instantiate(&mut store, &self.module).map_err(|e| e.to_string())?;
            let memory = instance.get_memory(&mut store, "memory").ok_or("Plugin does not export memory")?;
            let buffer = instance
                .get_typed_func::<(), u32>(&mut store, "plugin_buffer")
                .and_then(|function| function.call(&mut store, ()))
                .map_err(|e| format!("Plugin has no plugin_buffer: {e}"))? as usize;
            if buffer.checked_add(PLUGIN_BUFFER_BYTES).is_none_or(|end| end > memory.data_size(&store)) {
                return Err("Plugin buffer lies outside its memory".into());
            }
            if let Ok(configure) = instance.get_typed_func::<u32, i32>(&mut store, "plugin_configure") {
                if self.config.len() > PLUGIN_BUFFER_BYTES {
                    return Err(format!("Plugin configuration exceeds {PLUGIN_BUFFER_BYTES} bytes"));
                }
                memory.write(&mut store, buffer, self.config.as_bytes()).map_err(|e| e.to_string())?;
                let status = configure.call(&mut store, self.config.len() as u32).map_err(|e| format!("{e:#}"))?;
                if status != 0 {
                    return Err("Plugin refused its configuration".into());
                }
            }
            self.live = Some(Live { store, memory, buffer, instance });
        }
        Ok(self.live.as_mut().unwrap())
    }

    /// Calls `export` with `input` in the buffer, under a deadline of `ticks` epochs, and decodes what it wrote back.
    fn call<I: serde::Serialize, O: serde::de::DeserializeOwned>(&mut self, export: &str, input: Option<&I>, ticks: u64) -> Result<O, String> {
        let result = (|| {
            let live = self.live()?;
            let length = match input {
                Some(input) => {
                    let bytes = serde_json::to_vec(input).map_err(|e| e.to_string())?;
                    if bytes.len() > PLUGIN_BUFFER_BYTES {
                        return Err("Interception exceeds the plugin buffer".to_string());
                    }
                    live.memory.write(&mut live.store, live.buffer, &bytes).map_err(|e| e.to_string())?;
                    bytes.len() as u32
                }
                None => 0,
            };
            let function: TypedFunc<u32, i32> = live.instance.get_typed_func(&mut live.store, export).map_err(|e| e.to_string())?;
            live.store.set_epoch_deadline(ticks);
            let written = function.call(&mut live.store, length).map_err(|e| format!("{e:#}"))?;
            let written = usize::try_from(written).ok().filter(|written| *written <= PLUGIN_BUFFER_BYTES).ok_or("Plugin answered an invalid length")?;
            let mut bytes = vec![0; written];
            live.memory.read(&live.store, live.buffer, &mut bytes).map_err(|e| e.to_string())?;
            serde_json::from_slice(&bytes).map_err(|e| format!("Plugin answer does not decode: {e}"))
        })();
        if result.is_err() {
            // A trap can leave the guest's allocator or globals half updated.
            self.live = None;
        }
        result
    }

    pub fn info(&mut self) -> Result<PluginInfo, String> {
        self.call::<(), _>("plugin_info", None, u64::from(u16::MAX))
    }

    /// Asks the plugin about `interception`. The epoch traps a guest that runs past the deadline by a tick or two; an answer
    /// that arrives after `deadline` is refused all the same, since the game loop waited too long for it.
    pub fn intercept(&mut self, interception: &Interception, deadline: Duration) -> Result<Verdict, String> {
        self.live()?;
        let ticks = deadline.as_millis() as u64 + 2;
        let started = Instant::now();
        let verdict = self.call("plugin_intercept", Some(interception), ticks)?;
        let elapsed = started.elapsed();
        if elapsed > deadline {
            return Err(format!("Interceptor took {} µs, over its {} µs deadline", elapsed.as_micros(), deadline.as_micros()));
        }
        Ok(verdict)
    }
}
