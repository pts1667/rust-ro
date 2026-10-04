use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use script_sdk::{ABI_VERSION, MAX_MESSAGE_BYTES, Reply, Request};
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
}

impl WasmRuntime {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Arc<Self>, String> {
        let bytes = std::fs::read(path.as_ref()).map_err(|e| {
            format!(
                "Cannot load {}: {e}. Build scripts with cargo run --package tools --bin scripts-build",
                path.as_ref().display()
            )
        })?;
        Self::from_bytes(&bytes, Limits::default()).map(Arc::new)
    }

    pub fn from_bytes(bytes: &[u8], limits: Limits) -> Result<Self, String> {
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config).map_err(|e| e.to_string())?;
        let module = Module::new(&engine, bytes).map_err(|e| e.to_string())?;
        Ok(Self { engine, module, limits })
    }

    pub async fn execute<H: Host>(&self, host: H, entry: &str, id: u32) -> (H, Result<(), String>) {
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
        });
        store.limiter(|execution| &mut execution.limits);
        let result = self.run(&mut store, entry, id).await;
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

    async fn run<H: Host>(&self, store: &mut Store<Execution<H>>, entry: &str, id: u32) -> wasmtime::Result<()> {
        store.set_fuel(self.limits.fuel)?;
        store.fuel_async_yield_interval(Some(100_000))?;
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
        let instance = linker.instantiate_async(&mut *store, &self.module).await?;
        let abi = instance
            .get_typed_func::<(), u32>(&mut *store, "script_abi")?
            .call_async(&mut *store, ())
            .await?;
        wasmtime::ensure!(abi == ABI_VERSION, "Unsupported script ABI {abi}");
        store.data_mut().abi_verified = true;
        let exit = instance
            .get_typed_func::<u32, i32>(&mut *store, entry)?
            .call_async(&mut *store, id)
            .await?;
        wasmtime::ensure!(exit == 0, "Script returned error code {exit}");
        Ok(())
    }
}
