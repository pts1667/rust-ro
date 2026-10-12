//! What happened to a bot, as a numbered list of events: what hit it, what killed it, the monsters it killed and the
//! experience they gave, levels, map changes and the items it gained or lost. A program reads them after a sequence number
//! (`GET /bots/{name}/events?after=`, or pushed on the WebSocket) instead of comparing two observations.
//!
//! Damage, deaths and kills are recorded where the game applies them. The rest comes from comparing the character with
//! what it was at the previous game loop iteration, which catches every way a level or an item can change.
use std::collections::BTreeMap;

use models::enums::actor::CombatActorKind;
use serde::Serialize;

use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::CharacterKillMonster;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum BotEventKind {
    /// Something hit the bot. `source_kind` is `mob`, `player` or `other` (a homunculus, a trap...).
    Damaged {
        source_id: u32,
        source: Option<String>,
        source_kind: &'static str,
        mob_id: Option<i16>,
        damage: u32,
        skill: Option<String>,
        hp: u32,
        max_hp: u32,
    },
    /// An attack on the bot that did no damage (a miss, a dodge, a block).
    Missed { source_id: u32, source: Option<String>, source_kind: &'static str, mob_id: Option<i16> },
    /// The bot died. `killer` is what dealt the last blow, when the game knows it.
    Died { killer_id: Option<u32>, killer: Option<String>, killer_kind: Option<&'static str>, mob_id: Option<i16> },
    /// A monster died and the bot had a share of its experience (the experience is after the server's rates).
    Killed { mob_id: i16, name: String, x: u16, y: u16, base_exp: u32, job_exp: u32 },
    LevelUp { kind: &'static str, level: u32 },
    JobChanged { from: String, to: String },
    MapChanged { from: String, to: String, x: u16, y: u16 },
    /// The bot came back from the dead (a respawn, a resurrection).
    Revived { map: String, x: u16, y: u16 },
    ItemGained { item_id: i32, name: String, amount: i32 },
    ItemLost { item_id: i32, name: String, amount: i32 },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BotEvent {
    pub seq: u64,
    /// Milliseconds since the Unix epoch.
    pub at: u64,
    #[serde(flatten)]
    pub kind: BotEventKind,
}

/// What the comparison of two game loop iterations looks at.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BotSnapshot {
    map: String,
    x: u16,
    y: u16,
    base_level: u32,
    job_level: u32,
    job: String,
    dead: bool,
    /// Amount held of each item id, with its name.
    items: BTreeMap<i32, (i32, String)>,
}

impl BotSnapshot {
    pub fn of(character: &Character) -> Self {
        let mut items: BTreeMap<i32, (i32, String)> = BTreeMap::new();
        for (_, item) in character.inventory_iter() {
            let entry = items.entry(item.item_id).or_insert_with(|| (0, item.name_english.clone()));
            entry.0 += i32::from(item.amount);
        }
        Self {
            map: crate::server::model::map::Map::name_without_ext(character.current_map_name()),
            x: character.x(),
            y: character.y(),
            base_level: character.status.base_level,
            job_level: character.status.job_level,
            job: job_name(character.status.job),
            dead: character.is_dead() || character.status.hp == 0,
            items,
        }
    }

    /// The events that lead from `self` to `now`. A death is left to the damage that caused it when `death_recorded`.
    pub fn changes(&self, now: &Self, death_recorded: bool) -> Vec<BotEventKind> {
        let mut events = Vec::new();
        if now.map != self.map {
            events.push(BotEventKind::MapChanged { from: self.map.clone(), to: now.map.clone(), x: now.x, y: now.y });
        }
        if now.dead && !self.dead && !death_recorded {
            events.push(BotEventKind::Died { killer_id: None, killer: None, killer_kind: None, mob_id: None });
        }
        if !now.dead && self.dead {
            events.push(BotEventKind::Revived { map: now.map.clone(), x: now.x, y: now.y });
        }
        if now.job != self.job {
            events.push(BotEventKind::JobChanged { from: self.job.clone(), to: now.job.clone() });
        }
        if now.base_level > self.base_level {
            events.push(BotEventKind::LevelUp { kind: "base", level: now.base_level });
        }
        // A job change starts the job level over
        if now.job_level > self.job_level && now.job == self.job {
            events.push(BotEventKind::LevelUp { kind: "job", level: now.job_level });
        }
        for (item_id, (amount, name)) in &now.items {
            let before = self.items.get(item_id).map_or(0, |(amount, _)| *amount);
            if *amount > before {
                events.push(BotEventKind::ItemGained { item_id: *item_id, name: name.clone(), amount: amount - before });
            }
        }
        for (item_id, (amount, name)) in &self.items {
            let after = now.items.get(item_id).map_or(0, |(amount, _)| *amount);
            if after < *amount {
                events.push(BotEventKind::ItemLost { item_id: *item_id, name: name.clone(), amount: amount - after });
            }
        }
        events
    }
}

fn job_name(job: u32) -> String {
    use models::enums::EnumWithNumberValue;
    models::enums::class::JobName::try_from_value(job as usize).map_or_else(|_| job.to_string(), |job| format!("{job:?}"))
}

/// Who a damage came from: its id, name, kind and monster class.
struct Source {
    id: u32,
    name: Option<String>,
    kind: &'static str,
    mob_id: Option<i16>,
}

impl BotEventKind {
    /// The name and monster class this event gives the actor `id` that hit or missed the bot.
    pub fn source_named(&self, id: u32) -> Option<(String, Option<i16>)> {
        match self {
            BotEventKind::Damaged { source_id, source: Some(name), mob_id, .. } | BotEventKind::Missed { source_id, source: Some(name), mob_id, .. }
                if *source_id == id =>
            {
                Some((name.clone(), *mob_id))
            }
            _ => None,
        }
    }
}

fn source_of(server: &Server, state: &ServerState, target: &Character, damage: &Damage) -> Source {
    let attacker_id = damage.attacker_id;
    if let Some(player) = state.characters().get(&attacker_id) {
        return Source { id: attacker_id, name: Some(player.name.clone()), kind: "player", mob_id: None };
    }
    let mob = state
        .get_map_instance_from_character(target)
        .and_then(|instance| instance.state().get_mob(attacker_id).map(|mob| (mob.name_english.clone(), mob.mob_id)));
    if let Some((name, mob_id)) = mob {
        return Source { id: attacker_id, name: Some(name), kind: "mob", mob_id: Some(mob_id) };
    }
    // A monster that died in the same tick as its hit is gone already: the hits it landed before named it
    let kind = if damage.source_kind == CombatActorKind::Monster { "mob" } else { "other" };
    match server.bots().source_named(target.char_id, attacker_id) {
        Some((name, mob_id)) => Source { id: attacker_id, name: Some(name), kind, mob_id },
        None => Source { id: attacker_id, name: None, kind, mob_id: None },
    }
}

fn skill_name(skill_id: u32) -> Option<String> {
    (skill_id != 0).then(|| {
        GlobalConfigService::instance()
            .find_skill_config(&script_sdk::Value::Number(skill_id as i32))
            .map_or_else(|| skill_id.to_string(), |skill| skill.name.clone())
    })
}

impl Server {
    /// Records a damage the game applied to a bot, and its death when the damage killed it. `amount` is what the HP went down by.
    pub(crate) fn record_bot_damage(&self, state: &ServerState, damage: &Damage, amount: u32) {
        if !self.bots().is_bot(damage.target_id) {
            return;
        }
        let Some(target) = state.characters().get(&damage.target_id) else { return };
        let source = source_of(self, state, target, damage);
        let max_hp = crate::server::service::status_service::StatusService::instance().to_snapshot(&target.status).max_hp();
        let kind = if amount == 0 {
            BotEventKind::Missed { source_id: source.id, source: source.name.clone(), source_kind: source.kind, mob_id: source.mob_id }
        } else {
            BotEventKind::Damaged {
                source_id: source.id,
                source: source.name.clone(),
                source_kind: source.kind,
                mob_id: source.mob_id,
                damage: amount,
                skill: skill_name(damage.skill_id),
                hp: target.status.hp,
                max_hp,
            }
        };
        self.bots().record(damage.target_id, kind);
        if amount > 0 && (target.status.hp == 0 || target.is_dead()) {
            let killer = (source.kind != "other").then_some(source);
            self.bots().record_death(
                damage.target_id,
                BotEventKind::Died {
                    killer_id: Some(damage.attacker_id),
                    killer: killer.as_ref().and_then(|source| source.name.clone()),
                    killer_kind: killer.as_ref().map(|source| source.kind),
                    mob_id: killer.and_then(|source| source.mob_id),
                },
            );
        }
    }

    /// Records the experience a monster gave to the bots among `awards` (character id, base and job experience).
    pub(crate) fn record_bot_kill(&self, kill: &CharacterKillMonster, awards: &[(u32, u32, u32)]) {
        let name = GlobalConfigService::instance().get_mob(i32::from(kill.mob_id)).name_english.clone();
        let mut credited = false;
        for (char_id, base_exp, job_exp) in awards {
            credited |= *char_id == kill.char_id;
            if self.bots().is_bot(*char_id) {
                let kind = BotEventKind::Killed { mob_id: kill.mob_id, name: name.clone(), x: kill.mob_x, y: kill.mob_y, base_exp: *base_exp, job_exp: *job_exp };
                self.bots().record(*char_id, kind);
            }
        }
        // The killer of a monster that gave no experience (a PvP map, an event monster)
        if !credited && self.bots().is_bot(kill.char_id) {
            self.bots().record(kill.char_id, BotEventKind::Killed { mob_id: kill.mob_id, name, x: kill.mob_x, y: kill.mob_y, base_exp: 0, job_exp: 0 });
        }
    }

    /// Compares every bot in the game with the previous iteration, see `BotSnapshot::changes`.
    pub(crate) fn sample_bots(&self, state: &ServerState) {
        for char_id in self.bots().connected_char_ids() {
            if let Some(character) = state.characters().get(&char_id) {
                self.bots().sample(char_id, BotSnapshot::of(character), character.loaded_from_client_side);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> BotSnapshot {
        BotSnapshot {
            map: "prontera".into(),
            x: 150,
            y: 150,
            base_level: 10,
            job_level: 5,
            job: "Novice".into(),
            dead: false,
            items: BTreeMap::from([(501, (3, "Red Potion".to_string())), (909, (1, "Jellopy".to_string()))]),
        }
    }

    #[test]
    fn an_attacker_is_named_by_the_hits_it_landed_before() {
        let hit = BotEventKind::Damaged {
            source_id: 7,
            source: Some("Lunatic".into()),
            source_kind: "mob",
            mob_id: Some(1063),
            damage: 9,
            skill: None,
            hp: 20,
            max_hp: 40,
        };
        assert_eq!(hit.source_named(7), Some(("Lunatic".to_string(), Some(1063))));
        assert_eq!(hit.source_named(8), None);
        let unnamed = BotEventKind::Missed { source_id: 7, source: None, source_kind: "other", mob_id: None };
        assert_eq!(unnamed.source_named(7), None);
    }

    #[test]
    fn items_gained_and_lost_are_told_apart_by_their_amounts() {
        let before = snapshot();
        let mut after = snapshot();
        after.items.insert(501, (1, "Red Potion".into()));
        after.items.remove(&909);
        after.items.insert(1201, (1, "Knife".into()));
        assert_eq!(
            before.changes(&after, false),
            vec![
                BotEventKind::ItemGained { item_id: 1201, name: "Knife".into(), amount: 1 },
                BotEventKind::ItemLost { item_id: 501, name: "Red Potion".into(), amount: 2 },
                BotEventKind::ItemLost { item_id: 909, name: "Jellopy".into(), amount: 1 },
            ]
        );
    }

    #[test]
    fn a_death_recorded_by_its_damage_is_not_told_twice() {
        let before = snapshot();
        let mut after = snapshot();
        after.dead = true;
        assert_eq!(before.changes(&after, true), vec![]);
        assert_eq!(before.changes(&after, false), vec![BotEventKind::Died { killer_id: None, killer: None, killer_kind: None, mob_id: None }]);
        assert_eq!(after.changes(&before, false), vec![BotEventKind::Revived { map: "prontera".into(), x: 150, y: 150 }]);
    }

    #[test]
    fn a_job_change_is_not_a_job_level_up() {
        let before = snapshot();
        let mut after = snapshot();
        after.job = "Swordman".into();
        after.job_level = 1;
        after.base_level = 11;
        after.map = "izlude_in".into();
        assert_eq!(
            before.changes(&after, false),
            vec![
                BotEventKind::MapChanged { from: "prontera".into(), to: "izlude_in".into(), x: 150, y: 150 },
                BotEventKind::JobChanged { from: "Novice".into(), to: "Swordman".into() },
                BotEventKind::LevelUp { kind: "base", level: 11 },
            ]
        );
    }
}
