//! Mini-quests. A player walking up to a townsperson who has a quest for them sees it in a bubble above that townsperson,
//! with Accept and Decline. Offers are drawn per player from (seed, town, townsperson, character, hour): who offers, what
//! (hunting one of the monsters around the town, or bringing some of their loot), and how much, from the player's level.
//! Nothing is stored for an offer; an accepted quest goes to the character's journal, and its reward is granted as soon
//! as it is done and the player is in one of the towns. The accepted quests show in a tracker under the minimap.
//!
//! Hooks of one character can run at the same time (an area spell kills several monsters at once), so kills are counted
//! with atomic increments, one counter per journal place, and a reward is claimed through another counter that only one
//! run can take from 0 to 1. The journal itself is only written when the player accepts, declines, enters a map or
//! claims, which do not race in practice.

use script_sdk_2::constants::*;
use script_sdk_2::ui::{EventKind, HudSlot, Node, Overhead, UiEvent, Window, format_countdown};
use script_sdk_2::{Above, BotAction, Ctx, Function, MapPlayer, Request, Script, Stop, Val, Value, VariableScope};
use serde::{Deserialize, Serialize};

use crate::persona::{self, Kind, Persona};
use crate::random::{QUEST, Rng};
use crate::schedule::Plan;
use crate::towns::{Cell, Prey, TOWNS, direction, find};
use crate::Settings;

pub const OFFER_WINDOW: &str = "offer";
pub const TRACKER_WINDOW: &str = "quests";
const JOURNAL: &str = "alive_towns_quests$";
/// Offers are drawn anew every hour.
const OFFER_SLOT_SECS: i64 = 3600;
/// An offer stays up until the player is this many cells beyond the offer range.
const OFFER_LEEWAY: u16 = 3;
/// Monsters this many levels below the player, or above, are what they are sent after.
const LEVELS_BELOW: u32 = 10;
const LEVELS_ABOVE: u32 = 3;
/// A loot quest asks for no more than this many kills' worth of drops.
const MAX_KILLS: u32 = 40;
const DONE_COLOR: &str = "7cfc7c";
const DIM_COLOR: &str = "c8c8c8";
const GOLD_COLOR: &str = "ffd34d";
/// Reward and time limit in the bubble, whose body is near white.
const REWARD_COLOR: &str = "8a6200";
const LIMIT_COLOR: &str = "606060";
const TITLE_COLOR: &str = "ffffff";

fn kills_var(index: u8) -> String {
    format!("alive_towns_kills_{index}")
}

fn claim_var(index: u8) -> String {
    format!("alive_towns_claim_{index}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfferKey {
    pub town: u8,
    pub npc: u16,
    pub slot: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Task {
    Hunt { mob: u32 },
    Gather { item: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Quest {
    pub key: OfferKey,
    /// The journal place, which names its kill and claim counters.
    pub index: u8,
    pub giver: String,
    pub task: Task,
    /// The monster hunted, or the item brought.
    pub target: String,
    pub map: String,
    pub map_name: String,
    pub count: u32,
    /// Unix seconds.
    pub expires: i64,
    pub base_exp: u32,
    pub job_exp: u32,
    pub zeny: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Journal {
    pub quests: Vec<Quest>,
    /// Offers taken or declined, kept for the current and the previous hour.
    pub closed: Vec<OfferKey>,
}

/// Where a quest stands, read from the counters and the inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub have: u32,
    pub claimed: bool,
}

pub struct Offer {
    pub quest: Quest,
    pub minutes: u32,
    pub text: String,
}

impl Offer {
    /// The offer with its exp at the server's rates; [`offer`] draws it at 1x.
    pub fn rated(mut self, rates: ExpRates) -> Self {
        let rate = |exp: u32, percent: u32| (u64::from(exp) * u64::from(percent) / 100).min(i32::MAX as u64) as u32;
        self.quest.base_exp = rate(self.quest.base_exp, rates.base);
        self.quest.job_exp = rate(self.quest.job_exp, rates.job);
        self
    }
}

/// The server's exp rates in percent (100 is 1x), those kill exp gets. Quest exp follows them, zeny does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpRates {
    pub base: u32,
    pub job: u32,
}

/// Needs an attached player.
pub fn exp_rates(ctx: &Ctx) -> Result<ExpRates, Stop> {
    let mut reads = ctx.batch();
    reads.call(Function::GetBattleFlag, vec!["base_exp_rate".into()]);
    reads.call(Function::GetBattleFlag, vec!["job_exp_rate".into()]);
    let replies = reads.send()?;
    Ok(ExpRates { base: replies[0].number()?.max(0) as u32, job: replies[1].number()?.max(0) as u32 })
}

impl Quest {
    pub fn title(&self) -> String {
        match self.task {
            Task::Hunt { .. } => format!("Hunt {}", plural(&self.target)),
            Task::Gather { .. } => format!("Bring {}", self.target),
        }
    }

    fn done(&self, status: Status) -> bool {
        status.have >= self.count
    }
}

impl Journal {
    fn taken(&self, key: OfferKey) -> bool {
        self.closed.contains(&key) || self.quests.iter().any(|quest| quest.key == key)
    }

    fn free_index(&self, max: u32) -> Option<u8> {
        (0..max as u8).find(|index| self.quests.iter().all(|quest| quest.index != *index))
    }
}

/// `level` is the player's base level when the offer is shown; accepting it draws the same quest again from it.
pub fn offer(settings: &Settings, town: usize, npc: u32, persona: &Persona, char_id: u32, level: u32, slot: i64) -> Option<Offer> {
    let all = TOWNS[town].prey;
    if all.is_empty() {
        return None;
    }
    let mut rng = Rng::new(&[u64::from(settings.seed), QUEST, town as u64, u64::from(npc), u64::from(char_id), slot as u64]);
    if !rng.chance(settings.quest_offer_percent) {
        return None;
    }
    let fitting = fitting_prey(all, level);
    let prey = *rng.pick(&fitting);
    let gather = match persona.kind {
        Kind::Guard => false,
        Kind::Child => true,
        Kind::Folk => rng.chance(40),
    };
    let (task, target, count, kills) = match prey.loot.as_ref().filter(|_| gather) {
        Some(loot) => {
            let rate = u32::from(loot.rate);
            let count = (rng.range(3, 10) as u32).min(MAX_KILLS * rate / 10_000).max(1);
            (Task::Gather { item: loot.item }, loot.name, count, (count * 10_000).div_ceil(rate))
        }
        None => {
            let count = if u32::from(prey.level) > level { rng.range(5, 12) } else { rng.range(10, 25) } as u32;
            (Task::Hunt { mob: prey.mob }, prey.name, count, count)
        }
    };
    let percent = u64::from(settings.quest_reward_percent);
    let scaled = |value: u64| (value * percent / 100).min(i32::MAX as u64) as u32;
    let [shortest, longest] = settings.quest_minutes;
    let minutes = rng.range(u64::from(shortest), u64::from(longest)) as u32;
    let text = line(&mut rng, persona.kind, task, target, count, prey);
    let quest = Quest {
        key: OfferKey { town: town as u8, npc: npc as u16, slot: slot as u32 },
        index: 0,
        giver: persona.display.to_string(),
        task,
        target: target.to_string(),
        map: prey.map.to_string(),
        map_name: prey.map_name.to_string(),
        count,
        expires: 0,
        base_exp: scaled(u64::from(prey.base_exp) * u64::from(kills) * 3 / 2),
        job_exp: scaled(u64::from(prey.job_exp) * u64::from(kills) * 3 / 2),
        zeny: scaled((u64::from(prey.level) * 10 + 20) * u64::from(kills)),
    };
    Some(Offer { quest, minutes, text })
}

/// The monsters around the town close to the player's level, or the three closest when there are fewer.
fn fitting_prey(prey: &'static [Prey], level: u32) -> Vec<&'static Prey> {
    let near: Vec<&Prey> = prey.iter().filter(|prey| (level.saturating_sub(LEVELS_BELOW)..=level + LEVELS_ABOVE).contains(&u32::from(prey.level))).collect();
    if near.len() >= 3 {
        return near;
    }
    let mut closest: Vec<&Prey> = prey.iter().collect();
    closest.sort_by_key(|prey| (u32::from(prey.level).abs_diff(level), prey.mob));
    closest.truncate(3);
    closest
}

fn line(rng: &mut Rng, kind: Kind, task: Task, target: &str, count: u32, prey: &Prey) -> String {
    let place = format!("{} ({})", prey.map_name, prey.map);
    let mobs = plural(prey.name);
    let lines = match (kind, task) {
        (_, Task::Hunt { .. }) if prey.still => vec![
            format!("The {mobs} around {place} keep spreading. Could you clear {count} of them?"),
            format!("Someone has to clear the {mobs} near {place}. {count} would be a fine start."),
        ],
        (Kind::Guard, _) => vec![
            format!("Orders from the captain: cull {count} {mobs} near {place}. We are short-handed."),
            format!("Travellers keep getting hurt by {mobs} around {place}. Bring down {count} of them, will you?"),
        ],
        (Kind::Child, Task::Gather { .. }) => vec![
            format!("Can you bring me {count} {target}? The {mobs} at {place} have lots!"),
            format!("I am making a collection! I still need {count} {target}. {mobs} at {place} drop them."),
        ],
        (_, Task::Gather { .. }) => vec![
            format!("I need {count} {target} for my work. The {mobs} near {place} drop them."),
            format!("Could you gather {count} {target}? You can get them from {mobs} at {place}."),
        ],
        (_, Task::Hunt { .. }) => vec![
            format!("{mobs} have grown bold near {place}. Could you thin them out? {count} should do."),
            format!("My cart was ambushed by {mobs} on the road through {place}. Teach {count} of them a lesson?"),
            format!("I would pay well for someone to hunt {count} {mobs} around {place}."),
        ],
    };
    rng.pick(&lines).clone()
}

fn plural(name: &str) -> String {
    if name.ends_with('s') || name.ends_with('x') {
        name.to_string()
    } else if let Some(stem) = name.strip_suffix("lf") {
        format!("{stem}lves")
    } else if name.ends_with("sh") || name.ends_with("ch") {
        format!("{name}es")
    } else if name.ends_with('y') && !name.ends_with("ey") && !name.ends_with("ay") && !name.ends_with("oy") {
        format!("{}ies", &name[..name.len() - 1])
    } else {
        format!("{name}s")
    }
}

/// `12345` as `12,345`.
fn grouped(value: u32) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

fn button_id(verb: &str, key: OfferKey, level: u32) -> String {
    format!("{verb}:{}:{}:{}:{level}", key.town, key.npc, key.slot)
}

fn parse_button(widget: &str) -> Option<(&str, OfferKey, u32)> {
    let mut parts = widget.split(':');
    let verb = parts.next()?;
    let mut number = || parts.next()?.parse::<u32>().ok();
    let key = OfferKey { town: u8::try_from(number()?).ok()?, npc: u16::try_from(number()?).ok()?, slot: number()? };
    let level = number()?;
    (usize::from(key.town) < TOWNS.len()).then_some((verb, key, level))
}

/// The bubble above townsperson `npc_id`.
pub fn bubble(offer: &Offer, level: u32, npc_id: u32) -> Window {
    let quest = &offer.quest;
    Window::new(
        OFFER_WINDOW,
        &quest.giver,
        230,
        Node::column([
            Node::wrapped(offer.text.as_str()),
            Node::wrapped(format!("Reward: {} base exp, {} job exp, {} zeny", grouped(quest.base_exp), grouped(quest.job_exp), grouped(quest.zeny))).color(REWARD_COLOR),
            Node::text(format!("Time limit: {}", format_countdown(offer.minutes * 60))).color(LIMIT_COLOR),
            Node::row([Node::button(button_id("accept", quest.key, level), "Accept"), Node::button(button_id("decline", quest.key, level), "Decline")]),
        ]),
    )
    .anchored(npc_id, 4)
}

/// The tracker under the minimap, or `None` without quests.
pub fn tracker(journal: &Journal, statuses: &[Status], now: i64, max: u32) -> Option<Window> {
    if journal.quests.is_empty() {
        return None;
    }
    let mut lines = Vec::new();
    for (quest, status) in journal.quests.iter().zip(statuses) {
        if quest.done(*status) {
            lines.push(Node::text(quest.title()).outlined().color(DONE_COLOR));
            lines.push(Node::text("Done! Enter a town for the reward.").outlined().color(DONE_COLOR));
        } else {
            let left = (quest.expires - now).clamp(0, i64::from(u32::MAX)) as u32;
            lines.push(Node::row([Node::text(quest.title()).outlined().color(TITLE_COLOR), Node::countdown(left).outlined().color(DIM_COLOR)]));
            lines.push(Node::text(format!("{}/{} - {}", status.have.min(quest.count), quest.count, quest.map_name)).outlined().color(DIM_COLOR));
        }
    }
    let title = format!("Quests {}/{max}", journal.quests.len());
    Some(Window::new(TRACKER_WINDOW, &title, 200, Node::column(lines).gap(2)).hud(HudSlot::BelowMinimap).frameless().collapsible())
}

/// What bots see of a quest.
#[derive(Serialize)]
struct Observed<'q> {
    title: String,
    task: Task,
    map: &'q str,
    have: u32,
    count: u32,
    done: bool,
    seconds_left: i64,
}

/// The journal and the time, in one round trip.
fn load(ctx: &Ctx) -> Result<(Journal, i64), Stop> {
    let mut reads = ctx.batch();
    reads.request(Request::Read(JOURNAL.into()));
    reads.call(Function::GetTimeTick, vec![2.into()]);
    let replies = reads.send()?;
    let text = replies[0].text();
    let journal = if text.is_empty() { Journal::default() } else { serde_json::from_str(&text).unwrap_or_default() };
    Ok((journal, i64::from(replies[1].number()?)))
}

fn save(ctx: &Ctx, journal: &Journal) -> Script {
    let text = serde_json::to_string(journal).map_err(|error| Stop::from(error.to_string()))?;
    ctx.request(Request::Write { name: JOURNAL.into(), value: Value::String(text) }).map(|_| ())
}

fn statuses(ctx: &Ctx, journal: &Journal) -> Result<Vec<Status>, Stop> {
    if journal.quests.is_empty() {
        return Ok(Vec::new());
    }
    let mut reads = ctx.batch();
    for quest in &journal.quests {
        match quest.task {
            Task::Hunt { .. } => reads.request(Request::Read(kills_var(quest.index))),
            Task::Gather { item } => reads.call(Function::CountItem, vec![(item as i32).into()]),
        };
        reads.request(Request::Read(claim_var(quest.index)));
    }
    let replies = reads.send()?;
    replies.chunks(2).map(|pair| Ok(Status { have: pair[0].number()?.max(0) as u32, claimed: pair[1].number()? != 0 })).collect()
}

/// Drops claimed quests, expired unfinished ones (telling the player) and old closed offers; whether anything went.
fn settle(ctx: &Ctx, journal: &mut Journal, statuses: &mut Vec<Status>, now: i64) -> Result<bool, Stop> {
    let before = (journal.quests.len(), journal.closed.len());
    let mut kept = Vec::new();
    let mut kept_statuses = Vec::new();
    for (quest, status) in journal.quests.drain(..).zip(statuses.drain(..)) {
        if status.claimed {
            continue;
        }
        if quest.expires <= now && !quest.done(status) {
            ctx.player().message(&format!("Quest expired: {}.", quest.title()))?;
            continue;
        }
        kept.push(quest);
        kept_statuses.push(status);
    }
    journal.quests = kept;
    *statuses = kept_statuses;
    let slot = now.div_euclid(OFFER_SLOT_SECS) as u32;
    journal.closed.retain(|key| key.slot + 1 >= slot);
    Ok(before != (journal.quests.len(), journal.closed.len()))
}

/// Grants the reward of every finished quest whose claim this run takes; whether any was granted.
fn claim(ctx: &Ctx, journal: &mut Journal, statuses: &mut Vec<Status>) -> Result<bool, Stop> {
    let mut granted = false;
    for (quest, status) in journal.quests.iter().zip(statuses.iter_mut()) {
        if status.claimed || !quest.done(*status) {
            continue;
        }
        let taken = ctx.increment(&[(VariableScope::Character, &claim_var(quest.index), 1)])?[0] == 1;
        status.claimed = true;
        if !taken {
            continue;
        }
        if let Task::Gather { item } = quest.task {
            ctx.items().take(item as i32, quest.count as i32)?;
        }
        ctx.player().give_experience(quest.base_exp as i32, quest.job_exp as i32)?;
        let zeny = ctx.player().zeny()?;
        ctx.player().set_zeny(zeny.saturating_add(quest.zeny as i32))?;
        ctx.player().message(&format!("{} thanks you: {} done, {} zeny received.", quest.giver, quest.title(), grouped(quest.zeny)))?;
        let overhead = Overhead::text("Quest complete!", Some(GOLD_COLOR), 2_400).pop(300).rise(36.0, 1_800).fade_out(600);
        ctx.ui().overhead(Above::Player, &overhead)?;
        granted = true;
    }
    let (quests, kept): (Vec<Quest>, Vec<Status>) =
        journal.quests.drain(..).zip(statuses.drain(..)).filter(|(_, status)| !status.claimed).unzip();
    journal.quests = quests;
    *statuses = kept;
    Ok(granted)
}

fn show(ctx: &Ctx, settings: &Settings, journal: &Journal, statuses: &[Status], now: i64) -> Script {
    match tracker(journal, statuses, now, settings.quest_max_active) {
        Some(window) => ctx.ui().open(&window)?,
        None => ctx.ui().close(TRACKER_WINDOW)?,
    }
    if journal.quests.is_empty() {
        return ctx.bot().forget("quests");
    }
    let observed: Vec<Observed> = journal
        .quests
        .iter()
        .zip(statuses)
        .map(|(quest, status)| Observed {
            title: quest.title(),
            task: quest.task,
            map: &quest.map,
            have: status.have.min(quest.count),
            count: quest.count,
            done: quest.done(*status),
            seconds_left: (quest.expires - now).max(0),
        })
        .collect();
    ctx.bot().observe("quests", &observed)
}

fn in_town(settings: &Settings, map: &str) -> bool {
    settings.towns.iter().any(|town| town == map)
}

/// The attached player's map, named like the hooks name it (the server answers `prontera.gat`).
fn player_map(ctx: &Ctx) -> Result<String, Stop> {
    let map = ctx.player().map_name()?;
    Ok(map.strip_suffix(".gat").map(str::to_string).unwrap_or(map))
}

/// A map was entered or the character logged in: expired quests go, finished ones are rewarded in town, and the tracker
/// is drawn again.
pub fn arrived(ctx: &Ctx, settings: &Settings, map: &str) -> Script {
    let (mut journal, now) = load(ctx)?;
    if journal == Journal::default() {
        return Ok(());
    }
    let mut statuses = statuses(ctx, &journal)?;
    let mut changed = settle(ctx, &mut journal, &mut statuses, now)?;
    if in_town(settings, map) {
        changed |= claim(ctx, &mut journal, &mut statuses)?;
    }
    if changed {
        save(ctx, &journal)?;
    }
    show(ctx, settings, &journal, &statuses, now)
}

/// `MobKilled`: counts the kill for every hunt of that monster.
pub fn killed(ctx: &Ctx, settings: &Settings, mob: u32, map: &str) -> Script {
    let (journal, now) = load(ctx)?;
    let hunts: Vec<&Quest> = journal.quests.iter().filter(|quest| quest.task == Task::Hunt { mob } && quest.expires > now).collect();
    if hunts.is_empty() {
        return Ok(());
    }
    let names: Vec<String> = hunts.iter().map(|quest| kills_var(quest.index)).collect();
    let counters: Vec<(VariableScope, &str, i32)> = names.iter().map(|name| (VariableScope::Character, name.as_str(), 1)).collect();
    let kills = ctx.increment(&counters)?;
    if in_town(settings, map) {
        return arrived(ctx, settings, map);
    }
    for (quest, kills) in hunts.iter().zip(kills) {
        if kills == quest.count as i32 {
            ctx.player().message(&format!("{}: done! Enter a town for your reward.", quest.title()))?;
        }
    }
    let statuses = statuses(ctx, &journal)?;
    show(ctx, settings, &journal, &statuses, now)
}

/// `ItemObtained`: the loot of a gathering quest; finishing one in town rewards it at once.
pub fn obtained(ctx: &Ctx, settings: &Settings, item: u32, amount: u32) -> Script {
    let (journal, now) = load(ctx)?;
    if !journal.quests.iter().any(|quest| quest.task == Task::Gather { item } && quest.expires > now) {
        return Ok(());
    }
    let map = player_map(ctx)?;
    if in_town(settings, &map) {
        return arrived(ctx, settings, &map);
    }
    let statuses = statuses(ctx, &journal)?;
    for (quest, status) in journal.quests.iter().zip(&statuses) {
        if quest.task == (Task::Gather { item }) && quest.done(*status) && status.have.saturating_sub(amount) < quest.count {
            ctx.player().message(&format!("{}: done! Enter a town for your reward.", quest.title()))?;
        }
    }
    show(ctx, settings, &journal, &statuses, now)
}

/// Accept or Decline in a bubble. Either way the townsperson goes back to their routine (see [`offer_round`]).
pub fn offer_event(ctx: &Ctx, event: &UiEvent) -> Script {
    if event.kind != EventKind::Click {
        return Ok(());
    }
    let Some((verb, key, level)) = parse_button(&event.widget) else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    let char_id = ctx.player().char_id()? as u32;
    ctx.ui().close(OFFER_WINDOW)?;
    ctx.request(Request::Write { name: answered_var(char_id), value: Value::Number(1) })?;
    let (mut journal, now) = load(ctx)?;
    let mut statuses = statuses(ctx, &journal)?;
    let changed = settle(ctx, &mut journal, &mut statuses, now)?;
    let current = now.div_euclid(OFFER_SLOT_SECS) as u32;
    if journal.taken(key) || key.slot + 1 < current || u32::from(key.npc) >= settings.npcs_per_town {
        return if changed { save(ctx, &journal) } else { Ok(()) };
    }
    if verb != "accept" {
        journal.closed.push(key);
        return save(ctx, &journal);
    }
    let Some(index) = journal.free_index(settings.quest_max_active) else {
        ctx.player().message(&format!("You already hold {} quests.", settings.quest_max_active))?;
        return if changed { save(ctx, &journal) } else { Ok(()) };
    };
    let persona = persona::persona(&settings, usize::from(key.town), u32::from(key.npc));
    let Some(offer) = offer(&settings, usize::from(key.town), u32::from(key.npc), &persona, char_id, level, i64::from(key.slot)) else { return Ok(()) };
    let offer = offer.rated(exp_rates(ctx)?);
    let mut quest = offer.quest;
    quest.index = index;
    quest.expires = now + i64::from(offer.minutes) * 60;
    let mut reset = ctx.batch();
    reset.request(Request::Write { name: kills_var(index), value: Value::Number(0) });
    reset.request(Request::Write { name: claim_var(index), value: Value::Number(0) });
    reset.send()?;
    ctx.player().message(&format!("Quest accepted: {}. {} left.", quest.title(), format_countdown(offer.minutes * 60)))?;
    journal.quests.push(quest);
    journal.closed.push(key);
    save(ctx, &journal)?;
    let statuses = self::statuses(ctx, &journal)?;
    show(ctx, &settings, &journal, &statuses, now)
}

/// An offer of the `offers` bot action.
#[derive(Serialize)]
struct Listed {
    /// The townsperson's unit, above which the bubble shows.
    npc_id: i32,
    giver: String,
    /// Where they stand now; absent while they are inside a building.
    x: Option<u16>,
    y: Option<u16>,
    title: String,
    task: Task,
    target: String,
    count: u32,
    map: String,
    map_name: String,
    minutes: u32,
    base_exp: u32,
    job_exp: u32,
    zeny: u32,
}

/// Bot action `offers`: what the townsfolk of the bot's town offer its character this hour, at its current base level,
/// leaving out the offers it took or declined. The bot accepts one in the bubble, by walking up to the townsperson.
pub fn bot_offers(ctx: &Ctx, _action: &BotAction) -> Script {
    let settings: Settings = ctx.plugin_config()?;
    let map = player_map(ctx)?;
    let Some(town) = find(&map).filter(|_| settings.quests && in_town(&settings, &map)) else {
        return ctx.bot().reply(&serde_json::json!({ "offers": [], "error": "Quests are offered in the towns of alive_towns only" }));
    };
    let char_id = ctx.player().char_id()? as u32;
    let level = ctx.player().base_level()?.max(1) as u32;
    let (journal, now) = load(ctx)?;
    let rates = exp_rates(ctx)?;
    let slot = now.div_euclid(OFFER_SLOT_SECS);
    let offers: Vec<(u32, Offer)> = (0..settings.npcs_per_town)
        .filter_map(|npc| offer(&settings, town, npc, &persona::persona(&settings, town, npc), char_id, level, slot).map(|offer| (npc, offer.rated(rates))))
        .filter(|(_, offer)| !journal.taken(offer.quest.key))
        .collect();
    let (ids, cells) = if offers.is_empty() {
        (Vec::new(), Vec::new())
    } else {
        let names = offers.iter().map(|(npc, offer)| Value::String(persona::unique_name(&offer.quest.giver, town, *npc))).collect();
        let found = ctx.call(Function::GetNpcId, vec![0.into(), Val::from(Value::Array(names))])?;
        let ids: Vec<i32> = found.into_array().ok_or_else(|| Stop::from("GetNpcId did not answer an array"))?.iter().map(|id| id.number().unwrap_or(0)).collect();
        let cells = ctx.npc_cells(TOWNS[town].map, &ids)?;
        (ids, cells)
    };
    let listed: Vec<Listed> = offers
        .into_iter()
        .zip(ids.into_iter().zip(cells))
        .filter(|(_, (id, _))| *id != 0)
        .map(|((_, offer), (npc_id, cell))| Listed {
            npc_id,
            title: offer.quest.title(),
            giver: offer.quest.giver,
            x: cell.map(|cell| cell.0),
            y: cell.map(|cell| cell.1),
            task: offer.quest.task,
            target: offer.quest.target,
            count: offer.quest.count,
            map: offer.quest.map,
            map_name: offer.quest.map_name,
            minutes: offer.minutes,
            base_exp: offer.quest.base_exp,
            job_exp: offer.quest.job_exp,
            zeny: offer.quest.zeny,
        })
        .collect();
    ctx.bot().reply(&serde_json::json!({
        "town": TOWNS[town].map,
        "offers": listed,
        "range": settings.quest_offer_range,
        "free_slots": settings.quest_max_active.saturating_sub(journal.quests.len() as u32),
        "renews_in_secs": OFFER_SLOT_SECS - now.rem_euclid(OFFER_SLOT_SECS),
    }))
}

/// Set by a click in the bubble, so the controller lets the townsperson go.
fn answered_var(char_id: u32) -> String {
    format!("$@answered_{char_id}")
}

/// Set when the player enters a map, logins included: the server took their bubble down, so the controller forgets what it
/// showed them and lets the townsperson go.
fn entered_var(char_id: u32) -> String {
    format!("$@entered_{char_id}")
}

/// `MapEnter` of the attached player.
pub fn entered_map(ctx: &Ctx) -> Script {
    let char_id = ctx.player().char_id()? as u32;
    ctx.request(Request::Write { name: entered_var(char_id), value: Value::Number(1) }).map(|_| ())
}

/// A player standing by a townsperson with a quest for them. While the bubble is `open`, the townsperson waits at `cell`
/// facing the player; once it is answered, the player gets no other offer until they walk away from `cell`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Near {
    pub char_id: u32,
    pub npc: usize,
    pub open: bool,
    pub cell: Cell,
}

/// `char:npc:open:x:y` entries, `;`-separated, as the controller keeps them between runs.
pub fn parse_near(text: &str) -> Vec<Near> {
    text.split(';')
        .filter_map(|entry| {
            let mut parts = entry.split(':');
            let (char_id, npc) = (parts.next()?.parse().ok()?, parts.next()?.parse().ok()?);
            let open = parts.next()? == "1";
            Some(Near { char_id, npc, open, cell: (parts.next()?.parse().ok()?, parts.next()?.parse().ok()?) })
        })
        .collect()
}

pub fn format_near(near: &[Near]) -> String {
    near.iter().map(|near| format!("{}:{}:{}:{}:{}", near.char_id, near.npc, u8::from(near.open), near.cell.0, near.cell.1)).collect::<Vec<_>>().join(";")
}

/// The townsfolk waiting for a player's answer, whose routine steps the controller holds back.
pub fn held(near: &[Near]) -> Vec<usize> {
    near.iter().filter(|near| near.open).map(|near| near.npc).collect()
}

/// Measured like the server's sight (`PLAYER_FOV`, 20 cells), so an offer's townsperson is in view.
fn distance(a: Cell, b: Cell) -> u16 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

/// Runs `with` with the player of `account_id` attached, if they are still online; detaches afterwards.
fn with_player<T>(ctx: &Ctx, account_id: u32, with: impl FnOnce(&Ctx) -> Result<T, Stop>) -> Result<Option<T>, Stop> {
    if ctx.call(Function::AttachRid, vec![(account_id as i32).into()])?.number()? == 0 {
        return Ok(None);
    }
    let result = with(ctx);
    ctx.call(Function::DetachRid, Vec::new())?;
    result.map(Some)
}

/// One second of the town's offers, from the controller. A player who comes near a townsperson with a quest for them
/// gets a bubble above that townsperson, who stops and faces them; walking away takes the bubble down, and an answer or
/// the bubble going lets the townsperson walk back into their routine. `previous` is what the previous run returned.
#[allow(clippy::too_many_arguments)]
pub fn offer_round(ctx: &Ctx, settings: &Settings, plan: &Plan, town: usize, ids: &[i32], tick: i64, now: i64, players: &[MapPlayer], previous: &[Near]) -> Result<Vec<Near>, Stop> {
    let was_held = held(previous);
    let mut previous = previous.to_vec();
    if !previous.is_empty() {
        let mut reads = ctx.batch();
        for near in &previous {
            reads.request(Request::Read(answered_var(near.char_id)));
            reads.request(Request::Read(entered_var(near.char_id)));
        }
        let replies = reads.send()?;
        let mut clear = ctx.batch();
        let mut gone = Vec::new();
        for (near, reply) in previous.iter_mut().zip(replies.chunks(2)) {
            if reply[1].number()? != 0 {
                gone.push(near.char_id);
                clear.request(Request::Write { name: entered_var(near.char_id), value: Value::Number(0) });
            } else if reply[0].number()? != 0 {
                near.open = false;
                clear.request(Request::Write { name: answered_var(near.char_id), value: Value::Number(0) });
            }
        }
        previous.retain(|near| !gone.contains(&near.char_id));
        if !clear.is_empty() {
            clear.send()?;
        }
    }
    let positions: Vec<Option<Cell>> = if players.is_empty() {
        Vec::new()
    } else {
        ctx.npc_cells(TOWNS[town].map, ids)?
    };
    let slot = now.div_euclid(OFFER_SLOT_SECS);
    let range = settings.quest_offer_range as u16;
    let mut next = Vec::new();
    for player in players {
        let at = (player.x, player.y);
        if let Some(near) = previous.iter().find(|near| near.char_id == player.char_id) {
            if distance(near.cell, at) <= range + OFFER_LEEWAY {
                next.push(*near);
                continue;
            }
            if near.open {
                with_player(ctx, player.account_id, |ctx| ctx.ui().close(OFFER_WINDOW))?;
            }
        }
        let mut close: Vec<(u16, usize, Cell)> =
            positions.iter().enumerate().filter_map(|(npc, cell)| cell.map(|cell| (distance(cell, at), npc, cell))).filter(|(gap, ..)| *gap <= range).collect();
        close.sort();
        let found = close.into_iter().find_map(|(_, npc, cell)| offer(settings, town, npc as u32, &plan.personas[npc], player.char_id, player.base_level, slot).map(|offer| (npc, cell, offer)));
        let Some((npc, cell, offer)) = found else { continue };
        let shown = with_player(ctx, player.account_id, |ctx| {
            ctx.request(Request::Write { name: entered_var(player.char_id), value: Value::Number(0) })?;
            let (journal, _) = load(ctx)?;
            if journal.taken(offer.quest.key) || journal.quests.len() >= settings.quest_max_active as usize {
                return Ok(false);
            }
            ctx.request(Request::Write { name: answered_var(player.char_id), value: Value::Number(0) })?;
            ctx.ui().open(&bubble(&offer.rated(exp_rates(ctx)?), player.base_level, ids[npc] as u32))?;
            Ok(true)
        })?;
        let Some(open) = shown else { continue };
        if open {
            let mut greet = ctx.batch();
            greet.call(Function::NpcStop, vec![ids[npc].into()]);
            greet.call(Function::MoveNpc, vec![ids[npc].into(), (-1).into(), (-1).into(), i32::from(direction(cell, at)).into()]);
            greet.call(Function::Emotion, vec![ET_SURPRISE.into(), ids[npc].into()]);
            greet.send()?;
        }
        next.push(Near { char_id: player.char_id, npc, open, cell });
    }
    let still_held = held(&next);
    let mut resume = ctx.batch();
    for npc in was_held.into_iter().filter(|npc| !still_held.contains(npc)) {
        let state = plan.state_at(npc as u32, tick);
        if state.hidden {
            resume.call(Function::DisableNpc, vec![ids[npc].into()]);
            resume.call(Function::MoveNpc, vec![ids[npc].into(), i32::from(state.cell.0).into(), i32::from(state.cell.1).into(), i32::from(state.dir).into()]);
        } else {
            resume.call(Function::NpcWalkTo, vec![i32::from(state.cell.0).into(), i32::from(state.cell.1).into(), ids[npc].into()]);
        }
    }
    if !resume.is_empty() {
        resume.send()?;
    }
    Ok(next)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use script_sdk_2::MockTransport;
    use script_sdk_2::ui::{Placement, validate_window};

    use super::*;
    use crate::tests::{config_json, settings};
    use crate::towns::find;

    const CHAR_ID: u32 = 150_001;
    const NOW: i64 = 1_800_000_000;
    const BASE_RATE: u32 = 500;
    const JOB_RATE: u32 = 300;
    const RATES: ExpRates = ExpRates { base: BASE_RATE, job: JOB_RATE };

    /// The game id of a townsperson, from their unique name.
    fn npc_id(name: &str) -> i32 {
        let (_, index) = persona::parse_name(name).unwrap();
        110_000_000 + index as i32
    }

    fn prontera() -> usize {
        find("prontera").unwrap()
    }

    /// The first offer townsperson `from..` of Prontera has for a level-`level` player in the current hour.
    fn first_offer(settings: &Settings, level: u32) -> (u32, Offer) {
        let town = prontera();
        (settings.guards_per_town..settings.npcs_per_town)
            .find_map(|npc| offer(settings, town, npc, &persona::persona(settings, town, npc), CHAR_ID, level, NOW / OFFER_SLOT_SECS).map(|offer| (npc, offer)))
            .unwrap()
    }

    /// A character with variables, an inventory, zeny and a clock.
    struct Player {
        variables: Rc<RefCell<HashMap<String, Value>>>,
        items: Rc<RefCell<HashMap<i32, i32>>>,
        now: Rc<RefCell<i64>>,
        map: Rc<RefCell<String>>,
        /// Where the townsfolk stand, by NPC id; the others are not in view.
        npcs: Rc<RefCell<HashMap<i32, Cell>>>,
    }

    fn player() -> (Player, MockTransport) {
        let player = Player {
            variables: Rc::new(RefCell::new(HashMap::new())),
            items: Rc::new(RefCell::new(HashMap::new())),
            now: Rc::new(RefCell::new(NOW)),
            map: Rc::new(RefCell::new("prt_fild08".into())),
            npcs: Rc::default(),
        };
        let (variables, items, now, map, npcs) = (player.variables.clone(), player.items.clone(), player.now.clone(), player.map.clone(), player.npcs.clone());
        let config = config_json();
        let transport = MockTransport::new(move |request| match request {
            Request::Read(name) => Ok(variables.borrow().get(name).cloned().unwrap_or_else(|| if name.ends_with('$') { Value::String(String::new()) } else { Value::Number(0) })),
            Request::Write { name, value } => {
                variables.borrow_mut().insert(name.clone(), value.clone());
                Ok(Value::default())
            }
            Request::VariablesIncrement(counters) => {
                let mut variables = variables.borrow_mut();
                Ok(Value::Array(
                    counters
                        .iter()
                        .map(|counter| {
                            let value = variables.get(&counter.name).and_then(|value| value.number_value().ok()).unwrap_or(0) + counter.value.number_value().unwrap();
                            variables.insert(counter.name.clone(), Value::Number(value));
                            Value::Number(value)
                        })
                        .collect(),
                ))
            }
            Request::Call { function: Function::PluginConfig, .. } => Ok(Value::String(config.clone())),
            Request::Call { function: Function::GetTimeTick, .. } => Ok(Value::Number(*now.borrow() as i32)),
            Request::Call { function: Function::GetCharacterId, .. } => Ok(Value::Number(CHAR_ID as i32)),
            Request::Call { function: Function::StrCharInfo, .. } => Ok(Value::String(map.borrow().clone())),
            Request::Call { function: Function::CountItem, arguments } => Ok(Value::Number(*items.borrow().get(&arguments[0].number_value().unwrap()).unwrap_or(&0))),
            Request::Call { function: Function::DelItem, arguments } => {
                *items.borrow_mut().entry(arguments[0].number_value().unwrap()).or_default() -= arguments[1].number_value().unwrap();
                Ok(Value::default())
            }
            Request::Call { function: Function::AttachRid, .. } => Ok(Value::Number(1)),
            Request::Call { function: Function::GetBattleFlag, arguments } => Ok(Value::Number(if arguments[0].text() == "base_exp_rate" { BASE_RATE } else { JOB_RATE } as i32)),
            Request::Call { function: Function::GetNpcId, arguments } => {
                let Value::Array(names) = &arguments[1] else { return Err("names".into()) };
                Ok(Value::Array(names.iter().map(|name| Value::Number(npc_id(&name.text()))).collect()))
            }
            Request::Call { function: Function::GetNpcXy, arguments } => {
                let Value::Array(ids) = &arguments[1] else { return Err("ids".into()) };
                let cell = |id: &Value| npcs.borrow().get(&id.number_value().unwrap()).map_or([-1, -1], |cell| [i32::from(cell.0), i32::from(cell.1)]);
                Ok(Value::Array(ids.iter().map(|id| Value::Array(cell(id).map(Value::Number).to_vec())).collect()))
            }
            _ => Ok(Value::default()),
        });
        (player, transport)
    }

    fn journal(player: &Player) -> Journal {
        match player.variables.borrow().get(JOURNAL) {
            Some(Value::String(text)) => serde_json::from_str(text).unwrap(),
            _ => Journal::default(),
        }
    }

    fn click(widget: &str) -> UiEvent {
        UiEvent::click(OFFER_WINDOW, widget)
    }

    fn accept(ctx: &Ctx, npc: u32, level: u32) {
        let key = OfferKey { town: prontera() as u8, npc: npc as u16, slot: (NOW / OFFER_SLOT_SECS) as u32 };
        offer_event(ctx, &click(&button_id("accept", key, level))).unwrap();
    }

    #[test]
    fn offers_are_per_player_tailored_to_the_level_and_fit_a_bubble() {
        let settings = settings();
        let town = prontera();
        let slot = NOW / OFFER_SLOT_SECS;
        let mut offered = 0;
        for npc in 0..settings.npcs_per_town {
            let persona = persona::persona(&settings, town, npc);
            let Some(offer) = offer(&settings, town, npc, &persona, CHAR_ID, 30, slot) else { continue };
            offered += 1;
            let again = super::offer(&settings, town, npc, &persona, CHAR_ID, 30, slot).unwrap();
            assert_eq!(again.quest, offer.quest, "an offer is drawn again the same at accept time");
            let prey = TOWNS[town].prey.iter().find(|prey| prey.map == offer.quest.map && (prey.name == offer.quest.target || prey.loot.as_ref().is_some_and(|loot| loot.name == offer.quest.target))).unwrap();
            assert!(fitting_prey(TOWNS[town].prey, 30).iter().any(|fit| fit.mob == prey.mob), "{} is no fit for level 30", prey.name);
            if persona.kind == Kind::Guard {
                assert!(matches!(offer.quest.task, Task::Hunt { .. }));
            }
            assert!(offer.quest.count > 0 && offer.quest.base_exp > 0 && offer.quest.zeny > 0);
            assert!((settings.quest_minutes[0]..=settings.quest_minutes[1]).contains(&offer.minutes));
            let mut window = bubble(&offer, 30, 110_000_000 + npc);
            validate_window(&mut window, "alive_towns").unwrap();
            assert!(matches!(window.placement, Placement::Anchored { unit_id, .. } if unit_id == 110_000_000 + npc) && !window.closable);
        }
        let expected = settings.npcs_per_town * settings.quest_offer_percent / 100;
        assert!(offered > expected / 2 && offered < expected * 2, "{offered} offers, about {expected} expected");
        let someone_else = (0..settings.npcs_per_town).filter(|npc| offer(&settings, town, *npc, &persona::persona(&settings, town, *npc), CHAR_ID + 1, 30, slot).is_some()).count();
        let mine: Vec<u32> = (0..settings.npcs_per_town).filter(|npc| offer(&settings, town, *npc, &persona::persona(&settings, town, *npc), CHAR_ID, 30, slot).is_some()).collect();
        let theirs: Vec<u32> = (0..settings.npcs_per_town).filter(|npc| offer(&settings, town, *npc, &persona::persona(&settings, town, *npc), CHAR_ID + 1, 30, slot).is_some()).collect();
        assert!(someone_else > 0 && mine != theirs, "another player meets other offers");
    }

    #[test]
    fn low_and_high_levels_are_sent_after_what_suits_them() {
        let prey = TOWNS[prontera()].prey;
        assert!(fitting_prey(prey, 1).iter().all(|prey| prey.level <= 4));
        let top = prey.iter().map(|prey| prey.level).max().unwrap();
        assert!(fitting_prey(prey, 99).iter().all(|fit| top - fit.level <= 20), "the strongest around are left for level 99");
        assert_eq!(plural("Poring"), "Porings");
        assert_eq!(plural("Wolf"), "Wolves");
        assert_eq!(plural("Hornet"), "Hornets");
        assert_eq!(plural("Thief Bug"), "Thief Bugs");
        assert_eq!(grouped(1_234_567), "1,234,567");
    }

    #[test]
    fn accepting_starts_a_quest_and_the_tracker_and_holds_at_most_the_limit() {
        let settings = settings();
        let (player, transport) = player();
        let ctx = Ctx::new(&transport);
        let (npc, first) = first_offer(&settings, 30);
        accept(&ctx, npc, 30);
        let journal = journal(&player);
        assert_eq!(journal.quests.len(), 1);
        let quest = &journal.quests[0];
        assert_eq!((quest.target.as_str(), quest.count, quest.index), (first.quest.target.as_str(), first.quest.count, 0));
        assert_eq!(quest.expires, NOW + i64::from(first.minutes) * 60);
        assert_eq!((quest.base_exp, quest.job_exp, quest.zeny), (first.quest.base_exp * 5, first.quest.job_exp * 3, first.quest.zeny), "exp at the server's rates");
        assert_eq!(transport.calls(Function::UiClose)[0], [Value::String(OFFER_WINDOW.into())]);
        let opened = transport.calls(Function::UiOpen);
        let tracker: Window = serde_json::from_str(&opened.last().unwrap()[0].text()).unwrap();
        assert_eq!(tracker.id, TRACKER_WINDOW);
        assert!(matches!(tracker.placement, Placement::Hud { slot: HudSlot::BelowMinimap }) && tracker.collapsible);

        accept(&ctx, npc, 30);
        assert_eq!(self::journal(&player).quests.len(), 1, "an offer is taken once");
        let decline = OfferKey { town: prontera() as u8, npc: 99, slot: (NOW / OFFER_SLOT_SECS) as u32 };
        offer_event(&ctx, &click(&button_id("decline", decline, 30))).unwrap();
        assert!(self::journal(&player).closed.contains(&decline));

        let town = prontera();
        let more: Vec<u32> = (npc + 1..settings.npcs_per_town)
            .filter(|other| offer(&settings, town, *other, &persona::persona(&settings, town, *other), CHAR_ID, 30, NOW / OFFER_SLOT_SECS).is_some())
            .take(settings.quest_max_active as usize)
            .collect();
        for other in &more {
            accept(&ctx, *other, 30);
        }
        let journal = self::journal(&player);
        assert_eq!(journal.quests.len(), settings.quest_max_active as usize);
        let indices: std::collections::HashSet<u8> = journal.quests.iter().map(|quest| quest.index).collect();
        assert_eq!(indices.len(), journal.quests.len(), "each quest has its own counters");
        assert!(transport.calls(Function::Message).iter().any(|line| line[0].text().contains("already hold")));
    }

    #[test]
    fn kills_count_and_the_reward_comes_once_on_entering_a_town() {
        let mut settings = settings();
        settings.quest_offer_percent = 100;
        let (player, transport) = player();
        let ctx = Ctx::new(&transport);
        let town = prontera();
        let npc = settings.guards_per_town - 1;
        let level = 30;
        accept_with(&ctx, &settings, town, npc, level);
        let quest = journal(&player).quests[0].clone();
        let Task::Hunt { mob } = quest.task else { panic!("guards ask for hunts") };

        for _ in 0..quest.count - 1 {
            killed(&ctx, &settings, mob, &quest.map).unwrap();
        }
        killed(&ctx, &settings, mob + 1, &quest.map).unwrap();
        assert!(!transport.calls(Function::Message).iter().any(|line| line[0].text().contains("done!")));
        killed(&ctx, &settings, mob, &quest.map).unwrap();
        assert!(transport.calls(Function::Message).iter().any(|line| line[0].text().contains("done!")));
        assert!(transport.calls(Function::GetExperience).is_empty(), "no reward away from town");

        arrived(&ctx, &settings, "prt_fild01").unwrap();
        assert!(transport.calls(Function::GetExperience).is_empty());
        arrived(&ctx, &settings, "prontera").unwrap();
        assert_eq!(transport.calls(Function::GetExperience), [vec![Value::Number(quest.base_exp as i32), Value::Number(quest.job_exp as i32)]]);
        assert_eq!(player.variables.borrow()["Zeny"], Value::Number(quest.zeny as i32));
        assert!(journal(&player).quests.is_empty());
        assert_eq!(transport.calls(Function::UiClose).last().unwrap()[0], Value::String(TRACKER_WINDOW.into()), "no quest, no tracker");
        arrived(&ctx, &settings, "prontera").unwrap();
        assert_eq!(transport.calls(Function::GetExperience).len(), 1);
    }

    /// Accepts the offer of townsperson `npc` with `settings` in effect instead of the defaults.
    fn accept_with(ctx: &Ctx, settings: &Settings, town: usize, npc: u32, level: u32) {
        let offer = offer(settings, town, npc, &persona::persona(settings, town, npc), CHAR_ID, level, NOW / OFFER_SLOT_SECS).unwrap();
        let mut quest = offer.quest;
        quest.expires = NOW + i64::from(offer.minutes) * 60;
        let journal = Journal { quests: vec![quest], closed: Vec::new() };
        save(ctx, &journal).unwrap();
        for name in [kills_var(0), claim_var(0)] {
            ctx.request(Request::Write { name, value: Value::Number(0) }).unwrap();
        }
    }

    #[test]
    fn a_claim_already_taken_by_another_run_is_not_paid_twice() {
        let mut settings = settings();
        settings.quest_offer_percent = 100;
        let (player, transport) = player();
        let ctx = Ctx::new(&transport);
        accept_with(&ctx, &settings, prontera(), 0, 30);
        let quest = journal(&player).quests[0].clone();
        player.variables.borrow_mut().insert(kills_var(0), Value::Number(quest.count as i32));
        player.variables.borrow_mut().insert(claim_var(0), Value::Number(1));
        arrived(&ctx, &settings, "prontera").unwrap();
        assert!(transport.calls(Function::GetExperience).is_empty());
        assert!(journal(&player).quests.is_empty(), "the claimed quest leaves the journal");
    }

    #[test]
    fn loot_is_handed_over_in_town_and_unfinished_quests_expire() {
        let mut settings = settings();
        settings.quest_offer_percent = 100;
        let (player, transport) = player();
        let ctx = Ctx::new(&transport);
        let town = prontera();
        let child = (settings.guards_per_town..settings.npcs_per_town)
            .find(|npc| {
                let persona = persona::persona(&settings, town, *npc);
                persona.kind == Kind::Child && matches!(offer(&settings, town, *npc, &persona, CHAR_ID, 30, NOW / OFFER_SLOT_SECS).unwrap().quest.task, Task::Gather { .. })
            })
            .unwrap();
        accept_with(&ctx, &settings, town, child, 30);
        let quest = journal(&player).quests[0].clone();
        let Task::Gather { item } = quest.task else { unreachable!() };
        player.items.borrow_mut().insert(item as i32, quest.count as i32 + 2);
        obtained(&ctx, &settings, item, 3).unwrap();
        assert!(transport.calls(Function::Message).iter().any(|line| line[0].text().contains("done!")));
        assert!(transport.calls(Function::GetExperience).is_empty());
        *player.map.borrow_mut() = "prontera".into();
        obtained(&ctx, &settings, item, 1).unwrap();
        assert_eq!(transport.calls(Function::GetExperience).len(), 1, "finished in town, rewarded at once");
        assert_eq!(player.items.borrow()[&(item as i32)], 2);

        accept_with(&ctx, &settings, town, 0, 30);
        *player.now.borrow_mut() = journal(&player).quests[0].expires + 1;
        arrived(&ctx, &settings, "prontera").unwrap();
        assert!(journal(&player).quests.is_empty());
        assert!(transport.calls(Function::Message).iter().any(|line| line[0].text().starts_with("Quest expired")));
        assert_eq!(transport.calls(Function::GetExperience).len(), 1);
    }

    #[test]
    fn the_tracker_counts_down_and_marks_finished_quests() {
        let settings = settings();
        let (_, offer) = first_offer(&settings, 30);
        let mut quest = offer.quest;
        quest.expires = NOW + 5_400;
        let mut done = quest.clone();
        done.index = 1;
        let journal = Journal { quests: vec![quest.clone(), done], closed: Vec::new() };
        let statuses = [Status { have: 1, claimed: false }, Status { have: quest.count, claimed: false }];
        let mut window = tracker(&journal, &statuses, NOW, 3).unwrap();
        validate_window(&mut window, "alive_towns").unwrap();
        assert_eq!(window.title, "Quests 2/3");
        let nodes = window.nodes();
        assert!(nodes.iter().any(|node| matches!(node, Node::Countdown { seconds: 5_400, outline: true, .. })));
        assert!(nodes.iter().any(|node| matches!(node, Node::Text { text, .. } if text.starts_with(&format!("1/{}", quest.count)))));
        assert!(nodes.iter().any(|node| matches!(node, Node::Text { text, color: Some(color), .. } if text.starts_with("Done!") && color == DONE_COLOR)));
        assert!(tracker(&Journal::default(), &[], NOW, 3).is_none());
    }

    #[test]
    fn walking_up_to_a_townsperson_stops_them_with_their_offer_until_an_answer_or_walking_away() {
        let settings = settings();
        let town = prontera();
        let plan = Plan::new(&settings, town);
        let tick = 9_000_000;
        let ids: Vec<i32> = (0..settings.npcs_per_town as i32).map(|index| 50_000 + index).collect();
        let (npc, cell) = (0..settings.npcs_per_town)
            .filter_map(|npc| plan.position_at(npc, tick).map(|cell| (npc, cell)))
            .find(|(npc, _)| offer(&settings, town, *npc, &plan.personas[*npc as usize], CHAR_ID, 30, NOW / OFFER_SLOT_SECS).is_some())
            .unwrap();
        let (player, transport) = player();
        player.npcs.borrow_mut().insert(50_000 + npc as i32, cell);
        let ctx = Ctx::new(&transport);
        let standing = |x: u16, y: u16| [MapPlayer { account_id: 2_000_000, char_id: CHAR_ID, x, y, base_level: 30 }];
        let id = Value::Number(50_000 + npc as i32);
        let walks_back = |transport: &MockTransport| transport.calls(Function::NpcWalkTo).iter().filter(|walk| walk[2] == id).count();

        assert_eq!(settings.quest_offer_range, 14);
        assert!(offer_round(&ctx, &settings, &plan, town, &ids, tick, NOW, &standing(cell.0 + 10, cell.1 + 10), &[]).unwrap().is_empty(), "20 cells away as sight counts them");
        assert!(transport.calls(Function::UiOpen).is_empty());
        let near = offer_round(&ctx, &settings, &plan, town, &ids, tick, NOW, &standing(cell.0, cell.1 + 1), &[]).unwrap();
        let opened = transport.calls(Function::UiOpen);
        assert_eq!(opened.len(), 1);
        let window: Window = serde_json::from_str(&opened[0][0].text()).unwrap();
        assert_eq!(window.placement, Placement::Anchored { unit_id: 50_000 + npc, offset_y: 4 });
        assert_eq!(near, [Near { char_id: CHAR_ID, npc: npc as usize, open: true, cell }]);
        assert_eq!(held(&near), [npc as usize]);
        assert_eq!(transport.calls(Function::NpcStop), [vec![id.clone()]]);
        assert_eq!(transport.calls(Function::MoveNpc), [vec![id.clone(), Value::Number(-1), Value::Number(-1), Value::Number(0)]], "turned north to the player");
        assert_eq!(transport.calls(Function::DetachRid).len(), 1);
        assert_eq!(parse_near(&format_near(&near)), near);

        let later = tick + 20_000;
        let still = offer_round(&ctx, &settings, &plan, town, &ids, later, NOW, &standing(cell.0, cell.1 + 1), &near).unwrap();
        assert_eq!(still, near, "the townsperson waits where they stopped, whatever their routine says");
        assert_eq!(transport.calls(Function::UiOpen).len(), 1, "shown once while the player stays");

        player.variables.borrow_mut().insert(entered_var(CHAR_ID), Value::Number(1));
        let reentered = offer_round(&ctx, &settings, &plan, town, &ids, later, NOW, &standing(cell.0, cell.1 + 1), &still).unwrap();
        assert_eq!(reentered, still);
        assert_eq!(transport.calls(Function::UiOpen).len(), 2, "a map change, a login included, took the bubble down: it is sent again");
        assert_eq!(player.variables.borrow()[&entered_var(CHAR_ID)], Value::Number(0));
        assert_eq!(walks_back(&transport), 0, "the townsperson keeps waiting");

        player.variables.borrow_mut().insert(answered_var(CHAR_ID), Value::Number(1));
        let answered = offer_round(&ctx, &settings, &plan, town, &ids, later, NOW, &standing(cell.0, cell.1 + 1), &still).unwrap();
        assert!(answered.len() == 1 && !answered[0].open && held(&answered).is_empty(), "no new offer from them while the player stays");
        assert_eq!(walks_back(&transport), 1, "an answer sends the townsperson back to their routine");
        assert_eq!(player.variables.borrow()[&answered_var(CHAR_ID)], Value::Number(0));

        let reopened = offer_round(&ctx, &settings, &plan, town, &ids, tick, NOW, &standing(cell.0, cell.1 + 1), &[]).unwrap();
        let away = offer_round(&ctx, &settings, &plan, town, &ids, tick, NOW, &standing(cell.0.saturating_sub(40), cell.1), &reopened).unwrap();
        assert_eq!(transport.calls(Function::UiClose), [vec![Value::String(OFFER_WINDOW.into())]]);
        assert!(away.iter().all(|near| near.npc != npc as usize));
        assert_eq!(walks_back(&transport), 2);
        let reopened = offer_round(&ctx, &settings, &plan, town, &ids, tick, NOW, &standing(cell.0, cell.1 + 1), &[]).unwrap();
        assert!(offer_round(&ctx, &settings, &plan, town, &ids, tick, NOW, &[], &reopened).unwrap().is_empty());
        assert_eq!(walks_back(&transport), 3, "a player leaving the town lets the townsperson go");
    }

    #[test]
    fn bots_list_the_offers_of_their_town_at_the_server_rates() {
        let settings = settings();
        let (player, transport) = player();
        let ctx = Ctx::new(&transport);
        let action = BotAction { name: "offers".into(), args: "null".into() };
        let reply = |transport: &MockTransport| -> serde_json::Value { serde_json::from_str(&transport.calls(Function::BotReply).last().unwrap()[0].text()).unwrap() };
        bot_offers(&ctx, &action).unwrap();
        assert!(reply(&transport)["error"].is_string(), "no offers on a field");

        *player.map.borrow_mut() = "prontera.gat".into();
        player.variables.borrow_mut().insert("BaseLevel".into(), Value::Number(30));
        let town = prontera();
        let (taken, _) = first_offer(&settings, 30);
        accept(&ctx, taken, 30);
        let mine: Vec<(u32, Offer)> = (0..settings.npcs_per_town)
            .filter(|npc| *npc != taken)
            .filter_map(|npc| offer(&settings, town, npc, &persona::persona(&settings, town, npc), CHAR_ID, 30, NOW / OFFER_SLOT_SECS).map(|offer| (npc, offer)))
            .collect();
        let (seen, _) = mine[0];
        player.npcs.borrow_mut().insert(110_000_000 + seen as i32, (150, 180));
        bot_offers(&ctx, &action).unwrap();
        let reply = reply(&transport);
        assert_eq!(reply["town"], "prontera");
        assert_eq!(reply["free_slots"], settings.quest_max_active - 1);
        let listed = reply["offers"].as_array().unwrap();
        assert_eq!(listed.len(), mine.len(), "every offer but the one taken");
        for (entry, (npc, offer)) in listed.iter().zip(&mine) {
            assert_eq!(entry["npc_id"], 110_000_000 + npc);
            assert_eq!(entry["title"], offer.quest.title());
            assert_eq!((entry["base_exp"].as_u64(), entry["job_exp"].as_u64()), (Some(u64::from(offer.quest.base_exp * 5)), Some(u64::from(offer.quest.job_exp * 3))));
        }
        assert_eq!((listed[0]["x"].as_u64(), listed[0]["y"].as_u64()), (Some(150), Some(180)));
        assert!(listed[1]["x"].is_null(), "indoors");
    }

    #[test]
    fn rates_scale_exp_only() {
        let (_, offer) = first_offer(&settings(), 30);
        let (base, job, zeny) = (offer.quest.base_exp, offer.quest.job_exp, offer.quest.zeny);
        let rated = offer.rated(RATES);
        assert_eq!((rated.quest.base_exp, rated.quest.job_exp, rated.quest.zeny), (base * 5, job * 3, zeny));
    }

    #[test]
    fn buttons_name_their_offer() {
        let key = OfferKey { town: 21, npc: 7, slot: 500_000 };
        assert_eq!(parse_button(&button_id("accept", key, 42)), Some(("accept", key, 42)));
        assert_eq!(parse_button("accept:99:7:1:1"), None);
        assert_eq!(parse_button("accept:1:x:1:1"), None);
        assert!(button_id("decline", OfferKey { town: 255, npc: 999, slot: u32::MAX }, 99).len() <= 64);
    }
}
