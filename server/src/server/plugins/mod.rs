//! Plugins (`docs/adr/7-plugins.md`, `docs/plugins.md`): script modules with an identity, their own placements and
//! configuration, hooks and interceptors, loaded from `scripting.plugins_dir` in the order of `scripting.plugins`.

pub mod rates;

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use configuration::configuration::{PluginSetting, ScriptingConfig};
use script_runtime::{PluginPolicy, PluginSandbox, WasmRuntime};
use script_sdk::{ABI_VERSION, Capabilities, Capability, Entry, Hook, HookKind, Interception, InterceptorKind, PluginInfo, Value, Verdict};
use serde::Deserialize;

use crate::server::model::events::game_event::{GameEvent, PluginHook};
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::entries::intern;

/// Hooks some loaded plugin observes. Services test it before queuing a hook, so an unobserved hook costs one load.
static OBSERVED_HOOKS: AtomicU32 = AtomicU32::new(0);

pub fn observed(kind: HookKind) -> bool {
    OBSERVED_HOOKS.load(Ordering::Relaxed) & kind.bit() != 0
}

/// Queues `hook` for the next tick when a plugin observes its kind, for services that only hold the task queue.
pub fn queue_hook(queue: &TasksQueue<GameEvent>, hook: Hook) {
    if observed(hook.kind()) {
        queue.add_to_first_index(GameEvent::PluginHook(PluginHook { hook }));
    }
}

/// An NPC of a plugin's `npcs.json`: the shape of `config/wasm/npcs.json` without `module`. With `replaces`, it takes the place
/// of the NPC of the same name, and the fields it leaves out are that NPC's.
#[derive(Deserialize, Debug, Clone)]
pub struct PluginNpc {
    pub name: String,
    /// The plugin's NPC entry; the NPC's name when left out.
    #[serde(default)]
    pub entry: Option<String>,
    pub map_name: Option<String>,
    pub sprite: Option<String>,
    pub x: Option<u16>,
    pub y: Option<u16>,
    pub dir: Option<u16>,
    #[serde(default)]
    pub x_size: Option<u16>,
    #[serde(default)]
    pub y_size: Option<u16>,
    #[serde(default)]
    pub constructor_args: Option<Vec<Value>>,
    #[serde(default)]
    pub test: bool,
    #[serde(default)]
    pub replaces: bool,
}

/// An event label of a plugin's `events.json`.
#[derive(Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct PluginEvent {
    /// The plugin's event entry; the label when left out.
    pub entry: Option<String>,
    pub replaces: bool,
}

pub struct LoadedPlugin {
    pub info: PluginInfo,
    pub capabilities: Capabilities,
    /// The name the script VM knows the module by, `plugin:<name>`.
    pub module: String,
    pub runtime: Arc<WasmRuntime>,
    /// Handle of the plugin's initialisation; runs without an NPC (initialisation, hooks) carry it as their script.
    pub init_handle: u32,
    pub npcs: Vec<PluginNpc>,
    pub events: BTreeMap<String, PluginEvent>,
    hooks: u32,
    interceptors: u32,
    sandbox: Mutex<PluginSandbox>,
    failures: [AtomicU32; 6],
    disabled: AtomicU32,
}

impl LoadedPlugin {
    pub fn name(&self) -> &str {
        &self.info.name
    }

    pub fn observes(&self, kind: HookKind) -> bool {
        self.hooks & kind.bit() != 0
    }

    fn intercepts(&self, kind: InterceptorKind) -> bool {
        self.interceptors & kind.bit() != 0 && self.disabled.load(Ordering::Relaxed) & kind.bit() == 0
    }

    fn load(folder: &Path, setting: &PluginSetting, loaded: &[LoadedPlugin]) -> Result<Self, String> {
        let name = setting.name();
        let refuse = |reason: String| format!("Plugin {name} is refused: {reason}");
        let config = match std::fs::read_to_string(folder.join("config.json")) {
            Ok(text) => {
                serde_json::from_str::<serde_json::Value>(&text).map_err(|error| refuse(format!("config.json is invalid: {error}")))?;
                text
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => "{}".into(),
            Err(error) => return Err(refuse(format!("cannot read config.json: {error}"))),
        };
        let module = WasmRuntime::open(folder.join("plugin.wasm")).map_err(refuse)?;
        let info = PluginSandbox::new(&module).info().map_err(|error| refuse(format!("plugin_info failed: {error}")))?;
        if info.name != name {
            return Err(refuse(format!("it calls itself {}, not after its folder", info.name)));
        }
        if info.sdk_abi != ABI_VERSION {
            return Err(refuse(format!("it was built for script ABI {}, the server speaks {ABI_VERSION}", info.sdk_abi)));
        }
        if let Some(missing) = info.requires.iter().find(|required| !loaded.iter().any(|plugin| plugin.name() == required.as_str())) {
            return Err(refuse(format!("it requires {missing}, which must be listed and enabled before it")));
        }
        let declared: Capabilities = info.capabilities.iter().copied().collect();
        let capabilities = match setting.capabilities() {
            Some(names) => declared.intersection(names.iter().map(|name| capability(name)).collect::<Result<_, _>>().map_err(refuse)?),
            None => declared,
        };
        let npcs = read_optional(&folder.join("npcs.json")).map_err(refuse)?.unwrap_or_default();
        let events = read_optional(&folder.join("events.json")).map_err(refuse)?.unwrap_or_default();
        let runtime = Arc::new(module.with_plugin(PluginPolicy { name: name.into(), capabilities, config }));
        let module = format!("plugin:{name}");
        Ok(Self {
            capabilities,
            init_handle: intern(&module, Entry::PluginInit),
            module,
            npcs,
            events,
            hooks: info.hooks.iter().fold(0, |bits, kind| bits | kind.bit()),
            interceptors: info.interceptors.iter().fold(0, |bits, kind| bits | kind.bit()),
            sandbox: Mutex::new(PluginSandbox::new(&runtime)),
            runtime,
            failures: Default::default(),
            disabled: AtomicU32::new(0),
            info,
        })
    }
}

fn capability(name: &str) -> Result<Capability, String> {
    serde_json::from_value(serde_json::Value::String(name.into())).map_err(|_| format!("unknown capability {name}"))
}

fn read_optional<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map(Some).map_err(|error| format!("{} is invalid: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("cannot read {}: {error}", path.display())),
    }
}

/// The loaded plugins, in load order.
pub struct Plugins {
    list: Vec<LoadedPlugin>,
    interceptors: u32,
    deadline: Duration,
    max_failures: u32,
}

impl Default for Plugins {
    fn default() -> Self {
        Self { list: Vec::new(), interceptors: 0, deadline: Duration::from_micros(200), max_failures: 10 }
    }
}

impl Plugins {
    /// Loads the enabled plugins of `scripting.plugins`. Any refusal stops the boot: a server must not run without a plugin its
    /// operator listed.
    pub fn load(config: &ScriptingConfig) -> Result<Self, String> {
        let mut list: Vec<LoadedPlugin> = Vec::new();
        for setting in config.plugins.iter().filter(|setting| setting.enabled()) {
            if list.iter().any(|plugin| plugin.name() == setting.name()) {
                return Err(format!("Plugin {} is listed twice", setting.name()));
            }
            let plugin = LoadedPlugin::load(&Path::new(&config.plugins_dir).join(setting.name()), setting, &list)?;
            info!(
                "Loaded plugin {} {} (capabilities {:?}, hooks {:?}, interceptors {:?})",
                plugin.name(),
                plugin.info.version,
                plugin.capabilities.iter().collect::<Vec<_>>(),
                plugin.info.hooks,
                plugin.info.interceptors
            );
            list.push(plugin);
        }
        Ok(Self {
            interceptors: list.iter().fold(0, |bits, plugin| bits | plugin.interceptors),
            list,
            deadline: Duration::from_micros(config.interceptor_deadline_us.max(1)),
            max_failures: config.interceptor_failures.max(1),
        })
    }

    /// Lets the services queue the hooks these plugins observe. The server does it once at boot; tests that load plugins
    /// leave it alone, so other tests in the process see no extra events.
    pub fn publish_hooks(&self) {
        OBSERVED_HOOKS.fetch_or(self.list.iter().fold(0, |bits, plugin| bits | plugin.hooks), Ordering::Relaxed);
    }

    pub fn iter(&self) -> impl Iterator<Item = &LoadedPlugin> {
        self.list.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// The modules the script VM runs plugin scripts from, by their `plugin:<name>` key.
    pub fn modules(&self) -> impl Iterator<Item = (String, Arc<WasmRuntime>)> + '_ {
        self.list.iter().map(|plugin| (plugin.module.clone(), plugin.runtime.clone()))
    }

    /// The plugin whose initialisation or hook a request without an NPC comes from.
    pub fn by_init_handle(&self, handle: u32) -> Option<&LoadedPlugin> {
        self.list.iter().find(|plugin| plugin.init_handle == handle)
    }

    pub fn observers(&self, kind: HookKind) -> impl Iterator<Item = &LoadedPlugin> {
        self.list.iter().filter(move |plugin| plugin.observes(kind))
    }

    pub fn intercepted(&self, kind: InterceptorKind) -> bool {
        self.interceptors & kind.bit() != 0
    }

    /// Asks the plugins that intercept the kind of `decision`, in load order, each seeing the previous answer. `Err` carries
    /// the message of the first refusal. A plugin that fails is skipped, as if it had allowed the decision.
    pub fn intercept(&self, decision: Interception) -> Result<Interception, String> {
        let kind = decision.kind();
        if !self.intercepted(kind) {
            return Ok(decision);
        }
        let mut decision = decision;
        for plugin in self.list.iter().filter(|plugin| plugin.intercepts(kind)) {
            let answer = plugin.sandbox.lock().unwrap().intercept(&decision, self.deadline);
            match answer.and_then(|verdict| decision.clone().apply(&verdict).map(|next| (verdict, next))) {
                Ok((_, Some(next))) => decision = next,
                Ok((verdict, None)) => {
                    let Verdict::Refuse(message) = verdict else { unreachable!("only a refusal ends the decision") };
                    return Err(message);
                }
                Err(error) => self.record_failure(plugin, kind, &error),
            }
        }
        Ok(decision)
    }

    fn record_failure(&self, plugin: &LoadedPlugin, kind: InterceptorKind, error: &str) {
        let failures = plugin.failures[kind as usize].fetch_add(1, Ordering::Relaxed) + 1;
        warn!("Interceptor {kind:?} of plugin {} failed ({failures}/{}): {error}", plugin.name(), self.max_failures);
        if failures >= self.max_failures {
            plugin.disabled.fetch_or(kind.bit(), Ordering::Relaxed);
            error!("Interceptor {kind:?} of plugin {} is turned off until restart", plugin.name());
        }
    }
}

#[cfg(test)]
mod tests;
