//! Alive Towns: about a hundred procedural townsfolk per town. Each repeats a small routine (shopping at a counter, going
//! into a building, strolling between two places, loitering), joins small groups on the plazas now and then, and emotes.
//! Some have short quests for the players who walk up to them (`quests.rs`).
//! Settings and their defaults are in `info.json`; `config/plugins/alive_towns.json` overrides them.

mod director;
mod persona;
mod quests;
mod random;
mod schedule;
mod towns;
mod towns_data;

use script_sdk_2::constants::*;
use script_sdk_2::{Ctx, Function, Hook, PluginNpc, Script};
use serde::Deserialize;

use crate::persona::{Kind, Persona, parse_name, unique_name};
use crate::schedule::{Doing, Plan};
use crate::towns::{TOWNS, find};

script_sdk_2::plugin_module! {
    info { name: "alive_towns", version: "1.2.0", capabilities: [World, Dialogue, Interface, Character, Economy] }
    init => director::start,
    npcs {
        "Alive Towns" => director::controller,
        "townsfolk" => townsfolk,
    }
    events {
        "Alive Towns::OnDoor" => director::went_inside,
        "Alive Towns::OnTimer1000" => director::tick,
    }
    hooks {
        MapEnter => director::entered,
        Login => logged_in,
        MobKilled => killed,
        Die => died,
        ItemObtained => obtained,
    }
    interceptors {}
    windows { "offer" => quests::offer_event }
    bot_actions { "offers" => (quests::bot_offers, "The quests the townsfolk of your town offer you this hour, with where they stand: no arguments") }
    generated_npcs => generated_npc,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub towns: Vec<String>,
    pub npcs_per_town: u32,
    pub seed: u32,
    pub idle_when_empty: bool,
    pub gathering_percent: u32,
    pub player_look_percent: u32,
    pub children_percent: u32,
    pub guards_per_town: u32,
    /// Fastest and slowest, in ms per cell.
    pub walk_speed: [u16; 2],
    pub quests: bool,
    pub quest_offer_percent: u32,
    /// Cells between a player and a townsperson for an offer to show, |dx| + |dy| as for sight.
    pub quest_offer_range: u32,
    /// Shortest and longest time limit, in minutes.
    pub quest_minutes: [u32; 2],
    pub quest_max_active: u32,
    pub quest_reward_percent: u32,
    pub fame_per_level: u32,
    pub fame_quest_done: u32,
    /// For a quest run out of time, and for a death.
    pub fame_lost: u32,
    pub fame_quest_declined: u32,
}

impl Settings {
    /// Indices in [`TOWNS`] of the configured towns, once the settings are checked.
    pub fn towns(&self) -> Result<Vec<usize>, String> {
        if self.npcs_per_town > 999 {
            return Err(format!("npcs_per_town is {}, at most 999", self.npcs_per_town));
        }
        let [fast, slow] = self.walk_speed;
        if !(20..=1000).contains(&fast) || !(20..=1000).contains(&slow) || fast > slow {
            return Err(format!("walk_speed {:?} must be [fastest, slowest] within 20 to 1000", self.walk_speed));
        }
        let [shortest, longest] = self.quest_minutes;
        if !(10..=1440).contains(&shortest) || !(10..=1440).contains(&longest) || shortest > longest {
            return Err(format!("quest_minutes {:?} must be [shortest, longest] within 10 to 1440", self.quest_minutes));
        }
        if !(1..=17).contains(&self.quest_offer_range) || !(1..=5).contains(&self.quest_max_active) || self.quest_offer_percent > 100 {
            return Err("quest_offer_range is 1 to 17, quest_max_active 1 to 5 and quest_offer_percent at most 100".into());
        }
        let changes = [self.fame_quest_done, self.fame_lost, self.fame_quest_declined];
        if !(1..=1000).contains(&self.fame_per_level) || changes.iter().any(|change| *change > 1000) {
            return Err("fame_per_level is 1 to 1000, and fame_quest_done, fame_lost and fame_quest_declined at most 1000".into());
        }
        let mut towns = Vec::new();
        for map in &self.towns {
            let town = find(map).ok_or_else(|| {
                let known: Vec<&str> = TOWNS.iter().map(|town| town.map).collect();
                format!("{map} is not a town of config/wasm/warps.json; the towns are {}", known.join(", "))
            })?;
            if !towns.contains(&town) {
                towns.push(town);
            }
        }
        Ok(towns)
    }
}

/// The controller of each configured town, then their townsfolk, one town after the other.
fn generated_npc(index: usize) -> Result<Option<PluginNpc>, String> {
    let settings: Settings = script_sdk_2::configured()?;
    let towns = settings.towns()?;
    if let Some(&town) = towns.get(index) {
        return Ok(Some(PluginNpc {
            name: director::controller_name(town),
            entry: Some(director::CONTROLLER.into()),
            map_name: Some(TOWNS[town].map.into()),
            sprite: Some("HIDDEN_NPC".into()),
            x: Some(0),
            y: Some(0),
            ..Default::default()
        }));
    }
    let index = index - towns.len();
    let per_town = settings.npcs_per_town as usize;
    let Some(&town) = towns.get(index.checked_div(per_town).unwrap_or(usize::MAX)) else { return Ok(None) };
    let index = (index % per_town) as u32;
    let persona = persona::persona(&settings, town, index);
    Ok(Some(PluginNpc {
        name: unique_name(persona.display, town, index),
        entry: Some("townsfolk".into()),
        map_name: Some(TOWNS[town].map.into()),
        sprite: Some(persona.sprite),
        x: Some(persona.home.0),
        y: Some(persona.home.1),
        dir: Some(persona.dir),
        appearance: persona.appearance,
        speed: Some(persona.speed),
        ..Default::default()
    }))
}

fn logged_in(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::Login { map, .. } = hook else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    if settings.quests { quests::arrived(ctx, &settings, map) } else { Ok(()) }
}

fn killed(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::MobKilled { mob_id, map, .. } = hook else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    if settings.quests { quests::killed(ctx, &settings, *mob_id, map) } else { Ok(()) }
}

fn died(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::Die { .. } = hook else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    if settings.quests { quests::died(ctx, &settings) } else { Ok(()) }
}

fn obtained(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::ItemObtained { item_id, amount, .. } = hook else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    if settings.quests { quests::obtained(ctx, &settings, *item_id, *amount) } else { Ok(()) }
}

/// A townsperson says what it is busy with.
fn townsfolk(ctx: &Ctx) -> Script {
    let Some((town, index)) = parse_name(&ctx.npc().name()?) else { return ctx.close() };
    let settings: Settings = ctx.plugin_config()?;
    if index >= settings.npcs_per_town {
        return ctx.close();
    }
    let now = i64::from(ctx.call(Function::GetTimeTick, vec![0.into()])?.number()? as u32);
    let plan = Plan::new(&settings, town);
    let persona = &plan.personas[index as usize];
    let (line, emote) = line(persona, plan.state_at(index, now).doing, TOWNS[town].name);
    ctx.mes_as(persona.display, &line)?;
    ctx.npc().emotion(emote)?;
    ctx.close()
}

fn line(persona: &Persona, doing: Doing, town: &str) -> (String, i32) {
    match (persona.kind, doing) {
        (Kind::Guard, _) => (format!("Move along, citizen. All quiet in {town}."), ET_OK),
        (_, Doing::Gathering) => ("Just catching up with friends.".into(), ET_KIK),
        (Kind::Child, Doing::Shopping(npc)) => (format!("Mom sent me to the {npc}!"), ET_DELIGHT),
        (Kind::Child, _) => ("Wanna play tag? You're it!".into(), ET_MERONG),
        (_, Doing::Shopping(npc)) => (format!("I'm off to see the {npc}."), ET_MONEY),
        (_, Doing::Entering(_)) => ("I have some business inside.".into(), ET_GO),
        (_, Doing::Strolling) => (format!("Just stretching my legs. {town} is lovely this time of day."), ET_SMILE),
        (_, Doing::Loitering) => ("Hm? Oh, I'm only waiting for a friend.".into(), ET_HUM),
        (_, Doing::Patrolling) => ("Keeping an eye on things.".into(), ET_OK),
        (_, Doing::GoingHome) => ("Time to head home.".into(), ET_SLEEPY),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// The settings as `info.json` declares them, the way the server hands them over.
    pub fn config_json() -> String {
        let manifest: serde_json::Value = serde_json::from_str(include_str!("../info.json")).unwrap();
        let defaults: serde_json::Map<String, serde_json::Value> =
            manifest["config"].as_object().unwrap().iter().map(|(key, entry)| (key.clone(), entry["default"].clone())).collect();
        serde_json::Value::Object(defaults).to_string()
    }

    pub fn settings() -> Settings {
        serde_json::from_str(&config_json()).unwrap()
    }

    #[test]
    fn every_default_town_is_known_and_unknown_ones_are_refused() {
        let mut settings = settings();
        assert_eq!(settings.towns().unwrap().len(), settings.towns.len());
        settings.towns.push("prt_fild08".into());
        assert!(settings.towns().unwrap_err().contains("prt_fild08 is not a town"));
        let mut settings = super::tests::settings();
        settings.walk_speed = [300, 100];
        assert!(settings.towns().is_err());
        let mut settings = super::tests::settings();
        settings.quest_minutes = [180, 120];
        assert!(settings.towns().is_err());
        let mut settings = super::tests::settings();
        settings.quest_max_active = 6;
        assert!(settings.towns().is_err());
        let mut settings = super::tests::settings();
        settings.fame_per_level = 0;
        assert!(settings.towns().is_err());
    }

    #[test]
    fn placements_carry_the_look_and_stay_small() {
        script_sdk_2::plugin::configure_for_test(include_str!("../info.json"));
        let npcs: Vec<PluginNpc> = (0..).map_while(|index| generated_npc(index).unwrap()).collect();
        let settings = settings();
        assert_eq!(npcs.len(), settings.towns.len() * (1 + settings.npcs_per_town as usize));
        assert!(npcs[..settings.towns.len()].iter().all(|npc| npc.name.starts_with("Alive Towns_") && npc.entry.as_deref() == Some("Alive Towns")));
        let names: std::collections::HashSet<&str> = npcs.iter().map(|npc| npc.name.as_str()).collect();
        assert_eq!(names.len(), npcs.len());
        assert!(npcs.iter().filter(|npc| npc.appearance.is_some()).count() > npcs.len() / 2);
        let largest = npcs.iter().map(|npc| serde_json::to_vec(npc).unwrap().len()).max().unwrap();
        assert!(largest < 400, "{largest} bytes");
    }
}
