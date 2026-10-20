//! Plugins (`docs/adr/7-plugins.md`, `docs/plugins.md`): script modules with an identity, their own placements and
//! configuration, hooks and interceptors, loaded from `scripting.plugins_dir` in the order of `scripting.plugins`.

pub mod items;
pub mod rates;
pub mod skills;
pub mod ui;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use configuration::configuration::{PluginSetting, ScriptingConfig};
use script_runtime::{PluginPolicy, PluginSandbox, WasmRuntime};
pub use script_sdk::PluginNpc;
use script_sdk::{ABI_VERSION, Capabilities, Capability, Entry, Hook, HookKind, Interception, InterceptorKind, PluginInfo, Verdict};
use serde::Deserialize;

use crate::server::model::events::game_event::{GameEvent, PluginHook};
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::entries::intern;
use crate::server::script::skill::actor::NPC_WALK_SPEEDS;

/// Most NPCs one plugin may generate.
const MAX_GENERATED_NPCS: usize = 5_000;

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
    /// The defaults of the plugin's `info.json` with the overrides of `<plugins_dir>/<name>.json`, as the plugin reads them.
    pub settings: serde_json::Map<String, serde_json::Value>,
    /// The plugin's folder; files of its `ui/` subfolder are served to clients as `plugin/<name>/...`.
    pub folder: PathBuf,
    /// Its `items.json`.
    pub items: Vec<items::PluginItem>,
    /// Its `skills.json` and `jobs.json`, declared only (see `skills`).
    pub skills: Vec<skills::PluginSkill>,
    pub jobs: Vec<skills::PluginJob>,
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

    pub fn handles_window(&self, id: &str) -> bool {
        self.info.windows.iter().any(|window| window == id)
    }

    pub fn has_bot_action(&self, name: &str) -> bool {
        self.info.bot_actions.iter().any(|action| action.name == name)
    }

    pub fn has_command(&self, name: &str) -> bool {
        self.info.commands.iter().any(|command| command.name == name)
    }

    fn intercepts(&self, kind: InterceptorKind) -> bool {
        self.interceptors & kind.bit() != 0 && self.disabled.load(Ordering::Relaxed) & kind.bit() == 0
    }

    fn load(plugins_dir: &Path, setting: &PluginSetting, loaded: &[LoadedPlugin]) -> Result<Self, String> {
        let name = setting.name();
        let refuse = |reason: String| format!("Plugin {name} is refused: {reason}");
        let folder = plugins_dir.join(name);
        let overrides = plugins_dir.join(format!("{name}.json"));
        if folder.join("config.json").exists() {
            return Err(refuse(format!("its settings moved from {} to {}", folder.join("config.json").display(), overrides.display())));
        }
        let module = WasmRuntime::open(folder.join("plugin.wasm")).map_err(refuse)?;
        let info = PluginSandbox::new(&module).info().map_err(|error| refuse(format!("plugin_info failed (is its info.json valid?): {error}")))?;
        if info.name != name {
            return Err(refuse(format!("it calls itself {}, not after its folder", info.name)));
        }
        if info.sdk_abi != ABI_VERSION {
            return Err(refuse(format!("it was built for script ABI {}, the server speaks {ABI_VERSION}", info.sdk_abi)));
        }
        if let Some(missing) = info.requires.iter().find(|required| !loaded.iter().any(|plugin| plugin.name() == required.as_str())) {
            return Err(refuse(format!("it requires {missing}, which must be listed and enabled before it")));
        }
        let settings = match read_optional::<serde_json::Value>(&overrides).map_err(refuse)? {
            Some(overrides_json) => info.manifest.configure(&overrides_json).map_err(|error| refuse(format!("{}: {error}", overrides.display())))?,
            None => info.manifest.defaults(),
        };
        let declared: Capabilities = info.capabilities.iter().copied().collect();
        let capabilities = match setting.capabilities() {
            Some(names) => declared.intersection(names.iter().map(|name| capability(name)).collect::<Result<_, _>>().map_err(refuse)?),
            None => declared,
        };
        let mut npcs: Vec<PluginNpc> = read_optional(&folder.join("npcs.json")).map_err(refuse)?.unwrap_or_default();
        let events = read_optional(&folder.join("events.json")).map_err(refuse)?.unwrap_or_default();
        let items = items::load(&folder, &info).map_err(refuse)?;
        let skills = skills::load_skills(&folder).map_err(refuse)?;
        let jobs = skills::load_jobs(&folder).map_err(refuse)?;
        let config = serde_json::Value::Object(settings.clone()).to_string();
        let runtime = Arc::new(module.with_plugin(PluginPolicy { name: name.into(), capabilities, config }));
        npcs.extend(PluginSandbox::new(&runtime).generated_npcs(MAX_GENERATED_NPCS).map_err(|error| refuse(format!("its generated NPCs failed: {error}")))?);
        if let Some(npc) = npcs.iter().find(|npc| npc.speed.is_some_and(|speed| !NPC_WALK_SPEEDS.contains(&speed))) {
            return Err(refuse(format!("NPC {} walks at {} ms per cell, outside {}..={}", npc.name, npc.speed.unwrap_or_default(), NPC_WALK_SPEEDS.start(), NPC_WALK_SPEEDS.end())));
        }
        let module = format!("plugin:{name}");
        Ok(Self {
            capabilities,
            init_handle: intern(&module, Entry::PluginInit),
            module,
            npcs,
            events,
            settings,
            folder,
            items,
            skills,
            jobs,
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
    catalog: std::sync::OnceLock<plugin_ui::catalog::PluginCatalog>,
}

impl Default for Plugins {
    fn default() -> Self {
        Self { list: Vec::new(), interceptors: 0, deadline: Duration::from_micros(200), max_failures: 10, catalog: Default::default() }
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
            let plugin = LoadedPlugin::load(Path::new(&config.plugins_dir), setting, &list)?;
            info!(
                "Loaded plugin {} {} \"{}\" (capabilities {:?}, hooks {:?}, interceptors {:?})",
                plugin.name(),
                plugin.info.version,
                plugin.info.manifest.title,
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
            catalog: Default::default(),
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

    /// Adds the plugins' items to the game's, see [`items::install`].
    pub fn install_items(&self, items: &mut Vec<crate::repository::model::item_model::ItemModel>) -> Result<items::Installed, String> {
        let plugins: Vec<_> = self.list.iter().map(|plugin| (plugin.name(), plugin.items.as_slice())).collect();
        items::install(&plugins, items)
    }

    /// What clients need to draw the plugins' additions: `items`, from [`Self::install_items`], with the skills and jobs.
    pub fn catalog(
        &self,
        items: Vec<plugin_ui::catalog::CatalogItem>,
        game_skills: &std::collections::HashMap<u32, configuration::configuration::SkillConfig>,
    ) -> Result<plugin_ui::catalog::PluginCatalog, String> {
        let plugins: Vec<_> = self.list.iter().map(|plugin| (plugin.name(), plugin.skills.as_slice(), plugin.jobs.as_slice())).collect();
        let (skills, jobs) = skills::catalog(&plugins, game_skills)?;
        if !skills.is_empty() || !jobs.is_empty() {
            warn!("Plugins declare {} skills and {} jobs: clients see them in the catalog, but characters cannot learn or use them yet", skills.len(), jobs.len());
        }
        Ok(plugin_ui::catalog::PluginCatalog { items, skills, jobs })
    }

    /// Keeps the catalog clients download; set once at boot.
    pub fn publish_catalog(&self, catalog: plugin_ui::catalog::PluginCatalog) {
        if self.catalog.set(catalog).is_err() {
            warn!("The plugin catalog was already published");
        }
    }

    /// The catalog clients download, when plugins add anything to it.
    pub fn published_catalog(&self) -> Option<&plugin_ui::catalog::PluginCatalog> {
        self.catalog.get().filter(|catalog| !catalog.items.is_empty() || !catalog.skills.is_empty() || !catalog.jobs.is_empty())
    }

    pub fn by_name(&self, name: &str) -> Option<&LoadedPlugin> {
        self.list.iter().find(|plugin| plugin.name() == name)
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
