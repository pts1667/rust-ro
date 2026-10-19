use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use script_sdk::{ABI_VERSION, Capabilities, Entry, Function, MAX_BATCH_REQUESTS, MAX_MESSAGE_BYTES, Reply, Request, Value};
use wasmtime::{
    Caller, Config, Engine, InstanceAllocationStrategy, Linker, Module, PoolingAllocationConfig, Store, StoreLimits,
    StoreLimitsBuilder, UpdateDeadline,
};

mod sandbox;
pub use sandbox::PluginSandbox;

pub const MAX_CONCURRENT_RUNS: u32 = 4000;
pub const MEMORY_LIMIT_BYTES: usize = 1024 * 1024;
const MAX_TABLE_ELEMENTS: usize = 20_000;
const DEFAULT_CPU_BUDGET_MS: u64 = 2000;

/// Every module compiles against this engine so that they draw runs from one pool.
static ENGINE: OnceLock<Engine> = OnceLock::new();
/// Milliseconds since the epoch ticker started; the engine epoch advances with it.
static EPOCH_MS: AtomicU64 = AtomicU64::new(0);
static CPU_BUDGET_MS: AtomicU64 = AtomicU64::new(DEFAULT_CPU_BUDGET_MS);

/// The CPU time a run may spend in its own code, host calls excluded, before it traps (`scripting.cpu_budget_ms`). Runs that
/// start afterwards use it.
pub fn set_cpu_budget_ms(milliseconds: u64) {
    CPU_BUDGET_MS.store(milliseconds.max(1), Ordering::Relaxed);
}

fn epoch_ms() -> u64 {
    EPOCH_MS.load(Ordering::Relaxed)
}

#[async_trait]
pub trait Host: Send + 'static {
    async fn invoke(&mut self, request: Request) -> Reply;
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub memory_bytes: usize,
    pub host_calls: usize,
    pub cpu_ms: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            memory_bytes: MEMORY_LIMIT_BYTES,
            host_calls: 10_000,
            cpu_ms: CPU_BUDGET_MS.load(Ordering::Relaxed),
        }
    }
}

/// Functions whose last argument the runtime sets to the calling plugin's name, after this many arguments of the script,
/// so a plugin cannot act in another's name.
fn owned_arity(function: Function) -> Option<usize> {
    match function {
        Function::UiClose | Function::UiOpen | Function::BotReply => Some(1),
        Function::SetRate | Function::UiOverhead | Function::BotObserve => Some(2),
        _ => None,
    }
}

/// What a plugin module may do: the server checks every request of its runs against it.
#[derive(Debug, Clone)]
pub struct PluginPolicy {
    pub name: String,
    pub capabilities: Capabilities,
    /// The plugin's settings (`info.json` defaults with the operator's overrides) as a JSON object, answered to
    /// `PluginConfig` without a round trip to the game loop.
    pub config: String,
}

impl PluginPolicy {
    /// Refuses what the plugin did not declare and moves its shared variables under its name.
    pub fn admit(&self, request: Request) -> Result<Request, String> {
        if let Some(capability) = request.capability() {
            if !self.capabilities.contains(capability) {
                return Err(format!("Plugin {} has not declared the {capability:?} capability", self.name));
            }
        }
        Ok(match request.for_plugin(&self.name) {
            Request::Call { function, mut arguments } if owned_arity(function).is_some() => {
                arguments.truncate(owned_arity(function).unwrap_or_default());
                arguments.push(Value::String(self.name.clone()));
                Request::Call { function, arguments }
            }
            request => request,
        })
    }
}

pub struct WasmRuntime {
    engine: Engine,
    module: Module,
    limits: Limits,
    plugin: Option<Arc<PluginPolicy>>,
}

struct Execution<H> {
    host: H,
    limits: StoreLimits,
    calls_remaining: usize,
    abi_verified: bool,
    entry: Vec<u8>,
    cpu: CpuMeter,
}

/// Counts the time a run spends executing guest code. Each epoch tick that finds the guest running adds the time since the
/// guest last resumed (after a host call or a yield), so time spent waiting on the game loop is not counted.
struct CpuMeter {
    used_ms: u64,
    budget_ms: u64,
    resumed_at: Arc<AtomicU64>,
}

impl CpuMeter {
    fn new(budget_ms: u64) -> Self {
        Self { used_ms: 0, budget_ms, resumed_at: Arc::new(AtomicU64::new(epoch_ms())) }
    }

    fn resume(&self) {
        self.resumed_at.store(epoch_ms(), Ordering::Relaxed);
    }

    fn tick(&mut self) -> wasmtime::Result<UpdateDeadline> {
        let now = epoch_ms();
        self.used_ms += now.saturating_sub(self.resumed_at.swap(now, Ordering::Relaxed));
        wasmtime::ensure!(self.used_ms <= self.budget_ms, "Script exceeded its CPU budget of {} ms", self.budget_ms);
        Ok(UpdateDeadline::YieldCustom(1, Box::pin(YieldOnce { yielded: false, resumed_at: self.resumed_at.clone() })))
    }
}

/// Gives the thread back to the executor once, then marks the run as resumed.
struct YieldOnce {
    yielded: bool,
    resumed_at: Arc<AtomicU64>,
}

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            self.resumed_at.store(epoch_ms(), Ordering::Relaxed);
            return Poll::Ready(());
        }
        self.yielded = true;
        context.waker().wake_by_ref();
        Poll::Pending
    }
}

impl WasmRuntime {
    /// Compiling the generated NPC module takes seconds, so the machine code is cached next to it and reused while it is newer than the module.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Arc<Self>, String> {
        Self::load(path.as_ref()).map(Arc::new)
    }

    /// Loads a module without sharing it yet, so a plugin can be asked for its info before [`with_plugin`](Self::with_plugin).
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        Self::load(path.as_ref())
    }

    /// Holds the runs of this module to `policy`.
    pub fn with_plugin(mut self, policy: PluginPolicy) -> Self {
        self.plugin = Some(Arc::new(policy));
        self
    }

    fn load(path: &Path) -> Result<Self, String> {
        let cache = path.with_extension("cwasm");
        let fresh = |file: &Path| std::fs::metadata(file).and_then(|meta| meta.modified()).ok();
        if let (Some(module_time), Some(cache_time)) = (fresh(path), fresh(&cache)) {
            if cache_time >= module_time {
                if let Some(runtime) = Self::from_precompiled(&cache) {
                    return Ok(runtime);
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
        Ok(runtime)
    }

    /// Loads every `<name>.wasm` of `directory` as the module `name`; `replacements` name other files for some of them, or add modules.
    /// The modules are compiled, or restored from their cache, on one thread each.
    pub fn from_directory(
        directory: impl AsRef<Path>,
        replacements: impl IntoIterator<Item = (String, std::path::PathBuf)>,
    ) -> Result<HashMap<String, Arc<Self>>, String> {
        let directory = directory.as_ref();
        let mut paths: BTreeMap<String, std::path::PathBuf> = BTreeMap::new();
        let listing = std::fs::read_dir(directory).map_err(|e| {
            format!("Cannot list {}: {e}. Build scripts with cargo run --package tools --bin scripts-build", directory.display())
        })?;
        for entry in listing {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|extension| extension == "wasm") {
                if let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) {
                    paths.insert(name.to_string(), path);
                }
            }
        }
        paths.extend(replacements);
        let mut pending = paths.into_iter();
        let mut modules = HashMap::new();
        // The first module creates the shared engine; the others would race to build their own.
        let Some((first_name, first_path)) = pending.next() else { return Ok(modules) };
        modules.insert(first_name, Self::from_file(first_path)?);
        let pending: Vec<_> = pending.collect();
        let loaded = std::thread::scope(|scope| {
            let threads: Vec<_> = pending
                .iter()
                .map(|(name, path)| (name, scope.spawn(move || Self::from_file(path))))
                .collect();
            threads
                .into_iter()
                .map(|(name, thread)| {
                    let runtime = thread.join().map_err(|_| format!("Loading script module {name} panicked"))?;
                    runtime.map(|runtime| (name.clone(), runtime)).map_err(|e| format!("Failed to load script module {name}: {e}"))
                })
                .collect::<Result<Vec<_>, String>>()
        })?;
        modules.extend(loaded);
        Ok(modules)
    }

    fn from_precompiled(cache: &Path) -> Option<Self> {
        let engine = Self::engine().ok()?;
        // SAFETY: the file is produced by `module.serialize()` of this same build and rejected by wasmtime when it comes from another version or configuration.
        let module = unsafe { Module::deserialize_file(&engine, cache) }.ok()?;
        Some(Self { engine, module, limits: Limits::default(), plugin: None })
    }

    fn engine() -> Result<Engine, String> {
        if let Some(engine) = ENGINE.get() {
            return Ok(engine.clone());
        }
        let mut pool = PoolingAllocationConfig::default();
        pool.total_core_instances(MAX_CONCURRENT_RUNS)
            .total_memories(MAX_CONCURRENT_RUNS)
            .total_tables(MAX_CONCURRENT_RUNS)
            .total_stacks(MAX_CONCURRENT_RUNS)
            .max_memories_per_module(1)
            .max_tables_per_module(1)
            .max_memory_size(MEMORY_LIMIT_BYTES)
            .table_elements(MAX_TABLE_ELEMENTS);
        let mut config = Config::new();
        config.allocation_strategy(InstanceAllocationStrategy::Pooling(pool));
        // Slots reserve exactly the memory cap with no guard pages, so the pool spans MAX_CONCURRENT_RUNS * MEMORY_LIMIT_BYTES.
        config.memory_reservation(MEMORY_LIMIT_BYTES as u64).memory_guard_size(0);
        config.epoch_interruption(true);
        let engine = Engine::new(&config).map_err(|e| e.to_string())?;
        let mut started = false;
        let engine = ENGINE
            .get_or_init(|| {
                started = true;
                engine
            })
            .clone();
        if started {
            Self::start_epoch_ticker(engine.clone());
        }
        Ok(engine)
    }

    /// Advances the epoch about every millisecond. `EPOCH_MS` follows the clock, not the number of wakeups, so a coarse sleep
    /// granularity makes budgets checked less often, not longer.
    fn start_epoch_ticker(engine: Engine) {
        let started = Instant::now();
        std::thread::Builder::new()
            .name("wasm-epoch".into())
            .spawn(move || loop {
                std::thread::sleep(Duration::from_millis(1));
                EPOCH_MS.store(started.elapsed().as_millis() as u64, Ordering::Relaxed);
                engine.increment_epoch();
            })
            .expect("Cannot start the Wasm epoch ticker");
    }

    pub fn from_bytes(bytes: &[u8], limits: Limits) -> Result<Self, String> {
        let engine = Self::engine()?;
        let module = Module::new(&engine, bytes).map_err(|e| e.to_string())?;
        Ok(Self { engine, module, limits, plugin: None })
    }

    pub fn plugin(&self) -> Option<&PluginPolicy> {
        self.plugin.as_deref()
    }

    /// Runs the script `entry` of a module.
    pub async fn execute_named<H: Host>(&self, host: H, entry: &Entry) -> (H, Result<(), String>) {
        let limits = StoreLimitsBuilder::new()
            .memory_size(self.limits.memory_bytes)
            .memories(1)
            .instances(1)
            .tables(1)
            .table_elements(MAX_TABLE_ELEMENTS)
            .build();
        let mut store = Store::new(&self.engine, Execution {
            host,
            limits,
            calls_remaining: self.limits.host_calls,
            abi_verified: false,
            entry: Vec::new(),
            cpu: CpuMeter::new(self.limits.cpu_ms),
        });
        store.limiter(|execution| &mut execution.limits);
        store.epoch_deadline_callback(|mut context| context.data_mut().cpu.tick());
        store.set_epoch_deadline(1);
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
        store.set_epoch_deadline(self.limits.cpu_ms);
        let mut linker = Linker::new(&self.engine);
        linker.define_unknown_imports_as_traps(&self.module).map_err(|e| e.to_string())?;
        let instance = linker.instantiate(&mut store, &self.module).map_err(|e| e.to_string())?;
        instance
            .get_typed_func::<(), u64>(&mut store, "script_catalog_hash")
            .and_then(|function| function.call(&mut store, ()))
            .map_err(|e| e.to_string())
    }

    /// The request a run may send on, or the reply it gets without reaching the host.
    fn admit(plugin: Option<&PluginPolicy>, request: Request) -> Result<Request, Reply> {
        match (plugin, request) {
            (_, Request::Batch(requests)) if requests.len() > MAX_BATCH_REQUESTS => Err(Err(format!("A batch holds at most {MAX_BATCH_REQUESTS} requests"))),
            (plugin, Request::Batch(requests)) => requests
                .into_iter()
                .enumerate()
                .map(|(index, request)| match request {
                    Request::Batch(_) => Err(Err(format!("batch request {index}: batches cannot be nested"))),
                    Request::Call { function: Function::PluginConfig, .. } => Err(Err(format!("batch request {index}: PluginConfig cannot be batched"))),
                    request => Self::admit(plugin, request).map_err(|reply| Err(format!("batch request {index}: {}", reply.err().unwrap_or_default()))),
                })
                .collect::<Result<Vec<_>, _>>()
                .map(Request::Batch),
            (Some(plugin), Request::Call { function: Function::PluginConfig, .. }) => Err(Ok(Value::String(plugin.config.clone()))),
            (None, Request::Call { function: Function::PluginConfig, .. }) => Err(Err("Only plugins have a configuration".into())),
            (None, Request::Call { function, .. }) if owned_arity(function).is_some() && function != Function::SetRate => {
                Err(Err(format!("Only plugins can call {function:?}")))
            }
            (Some(plugin), request) => plugin.admit(request).map_err(Err),
            (None, request) => Ok(request),
        }
    }

    async fn run<H: Host>(&self, store: &mut Store<Execution<H>>, entry: &Entry) -> wasmtime::Result<()> {
        store.data_mut().entry = serde_json::to_vec(entry)?;
        let mut linker = Linker::new(&self.engine);
        let plugin = self.plugin.clone();
        linker.func_wrap_async(
            "rust_ro",
            "invoke",
            move |mut caller: Caller<'_, Execution<H>>,
                  (request_pointer, request_length, response_pointer, response_capacity): (u32, u32, u32, u32)| {
                let plugin = plugin.clone();
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
                    if let Request::Batch(requests) = &request {
                        let more = requests.len().saturating_sub(1);
                        wasmtime::ensure!(caller.data().calls_remaining >= more, "Script host call budget exhausted");
                        caller.data_mut().calls_remaining -= more;
                    }
                    let reply = match Self::admit(plugin.as_deref(), request) {
                        Ok(request) => caller.data_mut().host.invoke(request).await,
                        Err(reply) => reply,
                    };
                    caller.data().cpu.resume();
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

#[cfg(test)]
mod tests;
