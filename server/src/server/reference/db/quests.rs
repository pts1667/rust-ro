//! `/db/quests/<id>`: a quest of the quest database with its objectives and drops, the monsters they name as links.
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, items, matches, mobs, not_found, to_json, with_fields};
use crate::server::model::quest::{quest_catalog, quest_definition};

/// What the server's quest data does not hold: the quest's giver, rewards and prerequisites are in the NPC scripts.
const NOTES: &str = "Rewards, prerequisites, the NPC that gives the quest and its description are not in the quest data. They are in the \
                     NPC scripts under scripts/quests. A null mob matches any monster that fits the objective's other filters.";

pub fn link(id: u32) -> Link {
    let title = quest_definition(id).map_or_else(|| id.to_string(), |quest| quest.title.clone());
    Link::new("quests", &id.to_string(), json!(id), &title)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut quests: Vec<_> = quest_catalog().values().filter(|quest| matches(query.q.as_deref(), &[&quest.title])).collect();
    quests.sort_by_key(|quest| quest.id);
    quests.into_iter().map(|quest| link(quest.id)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let id: u32 = key.parse().map_err(|_| DbError::Invalid(format!("quests are keyed by their number, not {key}")))?;
    let definition = quest_definition(id).ok_or_else(|| not_found("quest", key))?;
    let objectives: Vec<Value> = definition
        .objectives
        .iter()
        .map(|objective| {
            let mobs_allowed: Vec<Value> = objective.mobs_allowed.iter().map(|mob| to_json(&mobs::link(i32::from(*mob)))).collect();
            with_fields(to_json(objective), [("mob", mob_value(objective.mob)), ("mobs_allowed", json!(mobs_allowed))])
        })
        .collect();
    let drops: Vec<Value> = definition
        .drops
        .iter()
        .map(|drop| with_fields(to_json(drop), [("mob", mob_value(drop.mob)), ("item", to_json(&items::link(drop.item as i32)))]))
        .collect();
    Ok(with_fields(
        to_json(definition),
        [("objectives", json!(objectives)), ("drops", json!(drops)), ("notes", json!(NOTES))],
    ))
}

fn mob_value(mob: u16) -> Value {
    if mob == 0 { Value::Null } else { to_json(&mobs::link(i32::from(mob))) }
}
