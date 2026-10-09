use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use script_sdk::{ABI_VERSION, Entry, MAX_MESSAGE_BYTES, Reply, Request};
use wasmtime::{Caller, Config, Engine, Linker, Module, Store, StoreLimits, StoreLimitsBuilder};

#[async_trait]
pub trait Host: Send + 'static {
    async fn invoke(&mut self, request: Request) -> Reply;
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub fuel: u64,
    pub memory_bytes: usize,
    pub host_calls: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            fuel: 20_000_000,
            memory_bytes: 32 * 1024 * 1024,
            host_calls: 10_000,
        }
    }
}

pub struct WasmRuntime {
    engine: Engine,
    module: Module,
    limits: Limits,
}

struct Execution<H> {
    host: H,
    limits: StoreLimits,
    calls_remaining: usize,
    abi_verified: bool,
    entry: Vec<u8>,
}

impl WasmRuntime {
    /// Compiling the generated NPC module takes seconds, so the machine code is cached next to it and reused while it is newer than the module.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Arc<Self>, String> {
        let path = path.as_ref();
        let cache = path.with_extension("cwasm");
        let fresh = |file: &Path| std::fs::metadata(file).and_then(|meta| meta.modified()).ok();
        if let (Some(module_time), Some(cache_time)) = (fresh(path), fresh(&cache)) {
            if cache_time >= module_time {
                if let Some(runtime) = Self::from_precompiled(&cache) {
                    return Ok(Arc::new(runtime));
                }
            }
        }
        let bytes = std::fs::read(path).map_err(|e| {
            format!(
                "Cannot load {}: {e}. Build scripts with cargo run --package tools --bin scripts-build",
                path.display()
            )
        })?;
        let runtime = Self::from_bytes(&bytes, Limits::default())?;
        if let Ok(compiled) = runtime.module.serialize() {
            let partial = cache.with_extension("cwasm.tmp");
            if std::fs::write(&partial, compiled).is_ok() {
                let _ = std::fs::rename(&partial, &cache);
            }
        }
        Ok(Arc::new(runtime))
    }

    fn from_precompiled(cache: &Path) -> Option<Self> {
        let engine = Self::engine().ok()?;
        // SAFETY: the file is produced by `module.serialize()` of this same build and rejected by wasmtime when it comes from another version or configuration.
        let module = unsafe { Module::deserialize_file(&engine, cache) }.ok()?;
        Some(Self { engine, module, limits: Limits::default() })
    }

    fn engine() -> Result<Engine, String> {
        let mut config = Config::new();
        config.consume_fuel(true);
        Engine::new(&config).map_err(|e| e.to_string())
    }

    pub fn from_bytes(bytes: &[u8], limits: Limits) -> Result<Self, String> {
        let engine = Self::engine()?;
        let module = Module::new(&engine, bytes).map_err(|e| e.to_string())?;
        Ok(Self { engine, module, limits })
    }

    /// Runs the script `entry` of a module.
    pub async fn execute_named<H: Host>(&self, host: H, entry: &Entry) -> (H, Result<(), String>) {
        let limits = StoreLimitsBuilder::new()
            .memory_size(self.limits.memory_bytes)
            .memories(1)
            .instances(1)
            .tables(1)
            .table_elements(20_000)
            .build();
        let mut store = Store::new(&self.engine, Execution {
            host,
            limits,
            calls_remaining: self.limits.host_calls,
            abi_verified: false,
            entry: Vec::new(),
        });
        store.limiter(|execution| &mut execution.limits);
        let result = self.run(&mut store, entry).await;
        (store.into_data().host, result.map_err(|error| format!("{error:#}")))
    }

    pub fn catalog_hash(&self) -> Result<u64, String> {
        let mut store = Store::new(
            &self.engine,
            StoreLimitsBuilder::new()
                .memory_size(self.limits.memory_bytes)
                .memories(1)
                .instances(1)
                .build(),
        );
        store.limiter(|limits| limits);
        store.set_fuel(100_000).map_err(|e| e.to_string())?;
        let mut linker = Linker::new(&self.engine);
        linker.define_unknown_imports_as_traps(&self.module).map_err(|e| e.to_string())?;
        let instance = linker.instantiate(&mut store, &self.module).map_err(|e| e.to_string())?;
        instance
            .get_typed_func::<(), u64>(&mut store, "script_catalog_hash")
            .and_then(|function| function.call(&mut store, ()))
            .map_err(|e| e.to_string())
    }

    async fn run<H: Host>(&self, store: &mut Store<Execution<H>>, entry: &Entry) -> wasmtime::Result<()> {
        store.set_fuel(self.limits.fuel)?;
        store.fuel_async_yield_interval(Some(100_000))?;
        store.data_mut().entry = serde_json::to_vec(entry)?;
        let mut linker = Linker::new(&self.engine);
        linker.func_wrap_async(
            "rust_ro",
            "invoke",
            |mut caller: Caller<'_, Execution<H>>,
             (request_pointer, request_length, response_pointer, response_capacity): (u32, u32, u32, u32)| {
                Box::new(async move {
                    wasmtime::ensure!(caller.data().abi_verified, "Host calls are unavailable before ABI verification");
                    wasmtime::ensure!(
                        request_length as usize <= MAX_MESSAGE_BYTES && response_capacity as usize <= MAX_MESSAGE_BYTES,
                        "Script message exceeds limit"
                    );
                    wasmtime::ensure!(caller.data().calls_remaining > 0, "Script host call budget exhausted");
                    caller.data_mut().calls_remaining -= 1;
                    let memory = caller
                        .get_export("memory")
                        .and_then(|export| export.into_memory())
                        .ok_or_else(|| wasmtime::format_err!("Guest does not export memory"))?;
                    let response_end = (response_pointer as usize)
                        .checked_add(response_capacity as usize)
                        .ok_or_else(|| wasmtime::format_err!("Invalid response buffer"))?;
                    wasmtime::ensure!(response_end <= memory.data_size(&caller), "Invalid response buffer");
                    let mut bytes = vec![0u8; request_length as usize];
                    memory.read(&caller, request_pointer as usize, &mut bytes)?;
                    let request: Request = serde_json::from_slice(&bytes)?;
                    let reply = caller.data_mut().host.invoke(request).await;
                    let bytes = serde_json::to_vec(&reply)?;
                    wasmtime::ensure!(bytes.len() <= response_capacity as usize, "Host response exceeds guest buffer");
                    memory.write(&mut caller, response_pointer as usize, &bytes)?;
                    Ok(bytes.len() as i32)
                })
            },
        )?;
        linker.func_wrap_async(
            "rust_ro",
            "entry",
            |mut caller: Caller<'_, Execution<H>>, (buffer_pointer, buffer_capacity): (u32, u32)| {
                Box::new(async move {
                    wasmtime::ensure!(caller.data().abi_verified, "Entry is unavailable before ABI verification");
                    let entry = caller.data().entry.clone();
                    wasmtime::ensure!(entry.len() <= buffer_capacity as usize, "Entry exceeds guest buffer");
                    let memory = caller
                        .get_export("memory")
                        .and_then(|export| export.into_memory())
                        .ok_or_else(|| wasmtime::format_err!("Guest does not export memory"))?;
                    let buffer_end = (buffer_pointer as usize)
                        .checked_add(buffer_capacity as usize)
                        .ok_or_else(|| wasmtime::format_err!("Invalid entry buffer"))?;
                    wasmtime::ensure!(buffer_end <= memory.data_size(&caller), "Invalid entry buffer");
                    memory.write(&mut caller, buffer_pointer as usize, &entry)?;
                    Ok(entry.len() as i32)
                })
            },
        )?;
        let instance = linker.instantiate_async(&mut *store, &self.module).await?;
        let abi = instance
            .get_typed_func::<(), u32>(&mut *store, "script_abi")?
            .call_async(&mut *store, ())
            .await?;
        wasmtime::ensure!(abi == ABI_VERSION, "Unsupported script ABI {abi}");
        store.data_mut().abi_verified = true;
        let exit = instance.get_typed_func::<(), i32>(&mut *store, "script_run")?.call_async(&mut *store, ()).await?;
        wasmtime::ensure!(exit == 0, "Script returned error code {exit}");
        Ok(())
    }
}
