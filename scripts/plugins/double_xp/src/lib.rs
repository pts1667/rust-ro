//! Example plugin: an experience and drop rate event. Its settings and their defaults are in `info.json`;
//! `config/plugins/double_xp.json` overrides them.

use script_sdk_2::constants::BC_ALL;
use script_sdk_2::{Ctx, Hook, Interception, Rate, Script, Verdict};
use serde::Deserialize;

script_sdk_2::plugin_module! {
    info { name: "double_xp", version: "1.0.0", capabilities: [Rates, Broadcast, Dialogue] }
    init => init,
    npcs { "Rate Clerk" => rate_clerk }
    events { "Rate Clerk::OnMinute00" => announce }
    hooks {
        Login => welcome,
        MobKilled => count_kill,
    }
    interceptors { ExpGain => scale_experience }
}

#[derive(Deserialize)]
struct Settings {
    /// Percent of the experience a monster gives.
    exp_percent: u32,
    /// Percent of the drop chances, applied through the server rates.
    drop_percent: u32,
    /// Kills between two messages to the player.
    milestone: i32,
}

const KILLS: &str = "double_xp_kills";

fn init(ctx: &Ctx) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    ctx.rates().set(Rate::Drop, settings.drop_percent)?;
    ctx.var("$started").set(1)?;
    ctx.announce(&headline(&settings), BC_ALL)
}

fn headline(settings: &Settings) -> String {
    format!("Rate event: {}% experience and {}% drops.", settings.exp_percent, settings.drop_percent)
}

fn rate_clerk(ctx: &Ctx) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    ctx.mes_as("Rate Clerk", &headline(&settings))?;
    let kills = ctx.var(KILLS).get()?.number()?;
    ctx.mes(&format!("You have slain {kills} monsters during the event."))?;
    ctx.mes(&format!("Drops run at {}% of the server rate.", ctx.rates().get(Rate::Drop)?))?;
    ctx.close()
}

fn announce(ctx: &Ctx) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    ctx.announce(&headline(&settings), BC_ALL)
}

fn welcome(ctx: &Ctx, _hook: &Hook) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    ctx.player().message(&headline(&settings))
}

fn count_kill(ctx: &Ctx, _hook: &Hook) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    let kills = ctx.var(KILLS).get()?.number()? + 1;
    ctx.var(KILLS).set(kills)?;
    if settings.milestone > 0 && kills % settings.milestone == 0 {
        ctx.player().message(&format!("{kills} monsters slain during the rate event."))?;
    }
    Ok(())
}

fn scale_experience(interception: &Interception) -> Verdict {
    let Interception::ExpGain { base, job, .. } = interception else { return Verdict::Allow };
    let percent = script_sdk_2::setting::<u32>("exp_percent").unwrap_or(100);
    let scale = |exp: &u32| (u64::from(*exp) * u64::from(percent) / 100).min(u64::from(u32::MAX)) as u32;
    Verdict::Exp { base: scale(base), job: scale(job) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_is_scaled_by_the_configured_percent() {
        script_sdk_2::plugin::configure_for_test(include_str!("../info.json"));
        let gain = Interception::ExpGain { char_id: 1, mob_id: 1002, base: 10, job: 7 };
        assert_eq!(scale_experience(&gain), Verdict::Exp { base: 20, job: 14 }, "200% by default");
        assert_eq!(scale_experience(&Interception::Chat { char_id: 1, map: "prontera".into(), message: "hi".into() }), Verdict::Allow);
    }
}
