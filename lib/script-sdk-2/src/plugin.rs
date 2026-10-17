//! What plugins add to scripts: their configuration, server rates, hooks and interceptors (`docs/plugins.md`).

use std::cell::UnsafeCell;

use script_sdk::{Function, Interception, InterceptorKind, PLUGIN_BUFFER_BYTES, PluginInfo, Rate, Verdict};
use serde::de::DeserializeOwned;

use crate::ctx::Ctx;
use crate::flow::{Script, Stop};

/// An observer of a [`Hook`](script_sdk::Hook): it runs like an event, with the character the hook is about attached.
pub type HookFn = fn(&Ctx<'_>, &script_sdk::Hook) -> Script;

/// An interceptor. It runs in the game loop with no host calls and a deadline of a few hundred microseconds, so it only
/// computes its answer from the decision and [`configured`].
pub type InterceptFn = fn(&Interception) -> Verdict;

impl<'a> Ctx<'a> {
    /// The plugin's `config.json`, decoded into `T`.
    pub fn plugin_config<T: DeserializeOwned>(&self) -> Result<T, Stop> {
        let text = self.plugin_config_json()?;
        serde_json::from_str(&text).map_err(|error| Stop::from(format!("Invalid plugin configuration: {error}")))
    }

    /// The plugin's `config.json` as it is written, `{}` when the plugin has none.
    pub fn plugin_config_json(&self) -> Result<String, Stop> {
        Ok(self.call(Function::PluginConfig, vec![])?.text())
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

/// The plugin's `config.json` decoded into `T`, for interceptors. It is parsed on every call; keep it small or cache it.
pub fn configured<T: DeserializeOwned>() -> Result<T, String> {
    let text = unsafe { &*CONFIG.0.get() };
    serde_json::from_str(text.as_deref().unwrap_or("{}")).map_err(|error| error.to_string())
}

/// The messages the host exchanges with a plugin outside of script runs. A module runs on one thread.
struct Shared<T>(UnsafeCell<T>);
unsafe impl<T> Sync for Shared<T> {}

static BUFFER: Shared<[u8; PLUGIN_BUFFER_BYTES]> = Shared(UnsafeCell::new([0; PLUGIN_BUFFER_BYTES]));
static CONFIG: Shared<Option<String>> = Shared(UnsafeCell::new(None));

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

#[doc(hidden)]
pub fn export_info(info: &PluginInfo) -> i32 {
    serde_json::to_vec(info).map_or(-1, |bytes| write_buffer(&bytes))
}

#[doc(hidden)]
pub fn export_configure(length: u32) -> i32 {
    let Some(text) = read_buffer(length).and_then(|bytes| std::str::from_utf8(bytes).ok()) else { return -1 };
    unsafe { *CONFIG.0.get() = Some(text.to_string()) };
    0
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
        ctx.rates().set(Rate::BaseExp, 300).unwrap();
        assert_eq!(ctx.rates().get(Rate::BaseExp), Ok(300));
        assert!(transport.requests().contains(&Request::Call { function: Function::SetRate, arguments: vec!["BaseExp".into(), 300.into()] }));
        let _ = Hook::Logout { char_id: 1, account_id: 1 };
    }
}
