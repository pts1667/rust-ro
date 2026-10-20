//! What plugins add to scripts: their configuration, server rates, hooks and interceptors (`docs/plugins.md`).

use std::cell::UnsafeCell;

use script_sdk::{Function, Interception, InterceptorKind, PLUGIN_BUFFER_BYTES, PluginInfo, PluginManifest, PluginNpc, PluginNpcPage, Rate, Verdict};
use serde::de::DeserializeOwned;

use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

/// An observer of a [`Hook`](script_sdk::Hook): it runs like an event, with the character the hook is about attached.
pub type HookFn = fn(&Ctx<'_>, &script_sdk::Hook) -> Script;

/// Runs one of the plugin's `@` commands, with the player who typed it attached. [`Ctx::reply`] answers in their chat
/// window.
pub type CommandFn = fn(&Ctx<'_>, &script_sdk::PluginCommand) -> Script;

/// An interceptor. It runs in the game loop with no host calls and a deadline of a few hundred microseconds, so it only
/// computes its answer from the decision and [`configured`].
pub type InterceptFn = fn(&Interception) -> Verdict;

impl<'a> Ctx<'a> {
    /// The plugin's settings decoded into `T`: every key of its `info.json`, with the value of `config/plugins/<name>.json`
    /// when the operator set one, else the default.
    pub fn plugin_config<T: DeserializeOwned>(&self) -> Result<T, Stop> {
        let text = self.plugin_config_json()?;
        serde_json::from_str(&text).map_err(|error| Stop::from(format!("Invalid plugin configuration: {error}")))
    }

    /// One setting of the plugin, by its `info.json` key: `ctx.plugin_setting::<u32>("exp_percent")`.
    pub fn plugin_setting<T: DeserializeOwned>(&self, key: &str) -> Result<T, Stop> {
        setting_of(&self.plugin_config_json()?, key).map_err(Stop::from)
    }

    /// The plugin's settings as the JSON object [`plugin_config`](Self::plugin_config) decodes.
    pub fn plugin_config_json(&self) -> Result<String, Stop> {
        Ok(self.call(Function::PluginConfig, vec![])?.text())
    }

    /// Answers the player who typed the plugin command being run, in their chat window; a line break starts another line. Bots that
    /// ran the command receive the lines.
    pub fn reply(&self, text: &str) -> Script {
        self.call(Function::CommandReply, vec![text.into()]).map(|_| ())
    }

    /// The server rates, which a plugin with the `Rates` capability can scale: `ctx.rates().set(Rate::BaseExp, 200)`.
    pub fn rates<'c>(&'c self) -> Rates<'c, 'a> {
        Rates { ctx: self }
    }
}

pub struct Rates<'c, 'a> {
    ctx: &'c Ctx<'a>,
}

impl Rates<'_, '_> {
    /// Sets this plugin's share of `rate`, in percent (100 leaves it unchanged). The shares of all plugins multiply.
    pub fn set(&self, rate: Rate, percent: u32) -> Script {
        let percent = i32::try_from(percent).map_err(|_| Stop::from("Rate is out of range"))?;
        self.ctx.call(Function::SetRate, vec![rate.name().into(), percent.into()]).map(|_| ())
    }

    /// What all plugins together apply to `rate`, in percent.
    pub fn get(&self, rate: Rate) -> Result<u32, Stop> {
        let percent = self.ctx.call(Function::GetRate, vec![rate.name().into()])?.number()?;
        Ok(percent.max(0) as u32)
    }
}

/// The plugin's settings decoded into `T`, for interceptors. They are parsed on every call; keep them small or cache them.
pub fn configured<T: DeserializeOwned>() -> Result<T, String> {
    serde_json::from_str(configuration()).map_err(|error| error.to_string())
}

/// One setting of the plugin, by its `info.json` key, for interceptors.
pub fn setting<T: DeserializeOwned>(key: &str) -> Result<T, String> {
    setting_of(configuration(), key)
}

/// The plugin's `info.json`: its title, description and settings with their defaults.
pub fn plugin_manifest() -> Result<PluginManifest, String> {
    let text = unsafe { *MANIFEST.0.get() };
    PluginManifest::parse(text.ok_or("Only plugins have a manifest")?)
}

/// Gives [`configured`] and [`setting`] the defaults of `info_json`, as the host would, for a plugin's unit tests:
/// `script_sdk_2::plugin::configure_for_test(include_str!("../info.json"))`.
pub fn configure_for_test(info_json: &str) {
    let defaults = PluginManifest::parse(info_json).expect("info.json").defaults();
    unsafe { *CONFIG.0.get() = Some(serde_json::Value::Object(defaults).to_string()) };
}

fn configuration() -> &'static str {
    unsafe { &*CONFIG.0.get() }.as_deref().unwrap_or("{}")
}

fn setting_of<T: DeserializeOwned>(settings: &str, key: &str) -> Result<T, String> {
    let settings: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(settings).map_err(|error| format!("Invalid plugin configuration: {error}"))?;
    let value = settings.get(key).ok_or_else(|| format!("Plugin setting {key} is not declared in info.json"))?;
    T::deserialize(value).map_err(|error| format!("Plugin setting {key}: {error}"))
}

/// The messages the host exchanges with a plugin outside of script runs. A module runs on one thread.
struct Shared<T>(UnsafeCell<T>);
unsafe impl<T> Sync for Shared<T> {}

static BUFFER: Shared<[u8; PLUGIN_BUFFER_BYTES]> = Shared(UnsafeCell::new([0; PLUGIN_BUFFER_BYTES]));
static CONFIG: Shared<Option<String>> = Shared(UnsafeCell::new(None));
static MANIFEST: Shared<Option<&'static str>> = Shared(UnsafeCell::new(None));

#[doc(hidden)]
pub fn set_manifest(info_json: &'static str) {
    unsafe { *MANIFEST.0.get() = Some(info_json) };
}

#[doc(hidden)]
pub fn buffer_address() -> u32 {
    BUFFER.0.get() as usize as u32
}

fn read_buffer(length: u32) -> Option<&'static [u8]> {
    let buffer = unsafe { &*BUFFER.0.get() };
    buffer.get(..length as usize)
}

fn write_buffer(bytes: &[u8]) -> i32 {
    let buffer = unsafe { &mut *BUFFER.0.get() };
    match buffer.get_mut(..bytes.len()) {
        Some(target) => {
            target.copy_from_slice(bytes);
            bytes.len() as i32
        }
        None => -1,
    }
}

/// Writes the plugin's [`PluginInfo`]; an `info.json` that does not decode answers -1, which the host refuses.
#[doc(hidden)]
pub fn export_info(info_json: &'static str, info: impl FnOnce(PluginManifest) -> PluginInfo) -> i32 {
    set_manifest(info_json);
    let Ok(manifest) = PluginManifest::parse(info_json) else { return -1 };
    serde_json::to_vec(&info(manifest)).map_or(-1, |bytes| write_buffer(&bytes))
}

#[doc(hidden)]
pub fn export_configure(length: u32) -> i32 {
    let Some(text) = read_buffer(length).and_then(|bytes| std::str::from_utf8(bytes).ok()) else { return -1 };
    unsafe { *CONFIG.0.get() = Some(text.to_string()) };
    0
}

/// Writes the page of `generate`'s NPCs that starts at the requested index and fits in the buffer. `generate` answers the NPC
/// at an index (`None` past the last), so a page holds no more than it sends: a guest has 1 MiB of memory.
#[doc(hidden)]
pub fn export_npcs(generate: fn(usize) -> Result<Option<PluginNpc>, String>, length: u32) -> i32 {
    #[derive(serde::Deserialize)]
    struct PageRequest {
        from: usize,
    }
    let Some(request) = read_buffer(length).and_then(|bytes| serde_json::from_slice::<PageRequest>(bytes).ok()) else { return -1 };
    serde_json::to_vec(&page_of(generate, request.from, PLUGIN_BUFFER_BYTES)).map_or(-1, |bytes| write_buffer(&bytes))
}

/// The NPCs from `from` whose page, encoded as the host reads it, stays within `budget` bytes.
fn page_of(generate: impl Fn(usize) -> Result<Option<PluginNpc>, String>, from: usize, budget: usize) -> Result<PluginNpcPage, String> {
    const ENVELOPE: usize = 64;
    let mut size = ENVELOPE;
    let mut page = Vec::new();
    for index in from.. {
        let Some(npc) = generate(index)? else { return Ok(PluginNpcPage { npcs: page, next: None }) };
        let length = serde_json::to_vec(&npc).map_or(usize::MAX, |bytes| bytes.len()) + 1;
        if size.saturating_add(length) > budget && !page.is_empty() {
            return Ok(PluginNpcPage { npcs: page, next: Some(index) });
        }
        size = size.saturating_add(length);
        page.push(npc);
    }
    unreachable!("an index past usize::MAX")
}

#[doc(hidden)]
pub fn export_intercept(table: &[(InterceptorKind, InterceptFn)], length: u32) -> i32 {
    let Some(interception) = read_buffer(length).and_then(|bytes| serde_json::from_slice::<Interception>(bytes).ok()) else { return -1 };
    let verdict = match table.iter().find(|(kind, _)| *kind == interception.kind()) {
        Some((_, interceptor)) => interceptor(&interception),
        None => Verdict::Allow,
    };
    serde_json::to_vec(&verdict).map_or(-1, |bytes| write_buffer(&bytes))
}

#[cfg(test)]
mod tests {
    use script_sdk::{Hook, Request, Value};

    use super::*;
    use crate::MockTransport;

    fn double(interception: &Interception) -> Verdict {
        match interception {
            Interception::ExpGain { base, job, .. } => Verdict::Exp { base: base * 2, job: job * 2 },
            _ => Verdict::Allow,
        }
    }

    #[test]
    fn interceptions_are_answered_through_the_buffer() {
        let gain = serde_json::to_vec(&Interception::ExpGain { char_id: 1, mob_id: 1002, base: 10, job: 4 }).unwrap();
        let length = write_buffer(&gain) as u32;
        let written = export_intercept(&[(InterceptorKind::ExpGain, double)], length);
        let verdict: Verdict = serde_json::from_slice(read_buffer(written as u32).unwrap()).unwrap();
        assert_eq!(verdict, Verdict::Exp { base: 20, job: 8 });

        let chat = serde_json::to_vec(&Interception::Chat { char_id: 1, map: "prontera".into(), message: "hi".into() }).unwrap();
        let length = write_buffer(&chat) as u32;
        let written = export_intercept(&[(InterceptorKind::ExpGain, double)], length);
        assert_eq!(serde_json::from_slice::<Verdict>(read_buffer(written as u32).unwrap()).unwrap(), Verdict::Allow, "undeclared kinds pass");
        assert_eq!(export_intercept(&[], 3), -1, "garbage is refused");
    }

    #[test]
    fn config_and_rates_go_through_the_host() {
        #[derive(serde::Deserialize)]
        struct Settings {
            multiplier: u32,
        }
        let transport = MockTransport::new(|request| match request {
            Request::Call { function: Function::PluginConfig, .. } => Ok(Value::String(r#"{"multiplier":3}"#.into())),
            Request::Call { function: Function::GetRate, .. } => Ok(Value::Number(300)),
            _ => Ok(Value::default()),
        });
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.plugin_config::<Settings>().unwrap().multiplier, 3);
        assert_eq!(ctx.plugin_setting::<u32>("multiplier").unwrap(), 3);
        assert!(ctx.plugin_setting::<u32>("divider").is_err(), "a key info.json does not declare");
        assert!(ctx.plugin_setting::<String>("multiplier").is_err(), "a value of another type does not decode");
        ctx.rates().set(Rate::BaseExp, 300).unwrap();
        assert_eq!(ctx.rates().get(Rate::BaseExp), Ok(300));
        assert!(transport.requests().contains(&Request::Call { function: Function::SetRate, arguments: vec!["BaseExp".into(), 300.into()] }));
        let _ = Hook::Logout { char_id: 1, account_id: 1 };
    }

    #[test]
    fn generated_npcs_are_paged_to_fit_the_buffer() {
        let generate = |index: usize| Ok((index < 300).then(|| PluginNpc { name: format!("Townsfolk#{index:03}"), map_name: Some("prontera".into()), ..Default::default() }));
        let mut from = 0;
        let (mut pages, mut total) = (0, 0);
        loop {
            let page = page_of(generate, from, 2_048);
            assert!(serde_json::to_vec(&page).unwrap().len() <= 2_048);
            let page = page.unwrap();
            assert_eq!(page.npcs[0].name, format!("Townsfolk#{from:03}"));
            pages += 1;
            total += page.npcs.len();
            match page.next {
                Some(next) => from = next,
                None => break,
            }
        }
        assert_eq!(total, 300);
        assert!(pages > 1, "{pages} pages");
    }
}
