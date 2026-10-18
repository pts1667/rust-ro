//! `/db/quests/<id>`: a quest of the quest database with its objectives and drops, the monsters they name as links, and the
//! text written for it in `config/quest_text.json`.
use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, items, matches, mobs, not_found, to_json, with_fields};
use crate::server::model::quest::{quest_catalog, quest_definition};

/// What the quest data does not hold. The giver NPCs are listed on the maps where they are placed.
const NOTES: &str = concat!(
    "Rewards are not in the quest data. They are only in the NPC scripts under scripts/quests. ",
    "The NPCs that start a quest are listed on its maps, in their `quests`. ",
    "description, guide, recommended_level, level_basis, prerequisites and level_override come from config/quest_text.json, ",
    "null for a quest not listed there; generated is false for text written by hand. ",
    "A null mob matches any monster that fits the objective's other filters.",
);

/// A quest that must be started or finished first, with the script line that checks it.
#[derive(Deserialize, Serialize)]
pub struct Prerequisite {
    pub quest: u32,
    pub state: String,
    pub quote: String,
}

/// One entry of `config/quest_text.json`, written by `tools/scripts-import/gen_quest_text.py` or by hand.
#[derive(Deserialize)]
struct QuestText {
    description: String,
    guide: Vec<String>,
    recommended_level: Value,
    level_basis: String,
    #[serde(default)]
    sources: Vec<String>,
    #[serde(default)]
    prerequisites: Vec<Prerequisite>,
    #[serde(default)]
    level_override: Option<Value>,
    generated: bool,
}

static TEXTS: OnceLock<BTreeMap<String, QuestText>> = OnceLock::new();

fn texts() -> &'static BTreeMap<String, QuestText> {
    TEXTS.get_or_init(|| serde_json::from_str(include_str!("../../../../../config/quest_text.json")).expect("Embedded quest text is invalid"))
}

pub use crate::server::model::quest::quests_offered_by as offered_by;

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
    let text = texts().get(&id.to_string());
    let body = with_fields(to_json(definition), [("objectives", json!(objectives)), ("drops", json!(drops))]);
    Ok(with_fields(
        body,
        [
            ("description", to_json(&text.map(|text| &text.description))),
            ("guide", to_json(&text.map(|text| &text.guide))),
            ("recommended_level", text.map_or(Value::Null, |text| text.recommended_level.clone())),
            ("level_basis", to_json(&text.map(|text| &text.level_basis))),
            ("sources", to_json(&text.map(|text| &text.sources))),
            ("prerequisites", to_json(&text.map(|text| &text.prerequisites))),
            ("level_override", text.and_then(|text| text.level_override.clone()).unwrap_or(Value::Null)),
            ("generated", to_json(&text.map(|text| text.generated))),
            ("notes", json!(NOTES)),
        ],
    ))
}

fn mob_value(mob: u16) -> Value {
    if mob == 0 { Value::Null } else { to_json(&mobs::link(i32::from(mob))) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::reference::db::tests::get_json;
    use axum::http::StatusCode;

    #[test]
    fn an_npc_names_the_quests_it_starts_and_nothing_for_other_npcs() {
        assert_eq!(offered_by("jobs_pre_re_jobs_s_1_1", "Cleric#aco"), &[1001, 1002, 1003]);
        assert!(offered_by("jobs_pre_re_jobs_s_1_1", "Cleric#aco ").is_empty());
        assert!(offered_by("no_such_module", "Cleric#aco").is_empty());
    }

    #[test]
    fn every_text_is_for_a_catalog_quest_and_complete() {
        for (key, text) in texts() {
            let id: u32 = key.parse().unwrap();
            assert!(quest_definition(id).is_some(), "{key} is not in the quest catalog");
            assert!(!text.description.trim().is_empty(), "{key} has no description");
            assert!((1..=6).contains(&text.guide.len()) && text.guide.iter().all(|step| !step.trim().is_empty()), "{key} guide");
            assert!(text.recommended_level.is_u64() || text.recommended_level == "Unknown", "{key} recommended_level");
            assert!(!text.level_basis.trim().is_empty(), "{key} has no level_basis");
            for prerequisite in &text.prerequisites {
                assert!(quest_definition(prerequisite.quest).is_some(), "{key} needs quest {}", prerequisite.quest);
            }
        }
    }

    #[test]
    fn the_merchant_quests_are_written_by_hand() {
        for id in 1009..=1012 {
            let text = texts().get(&id.to_string()).unwrap_or_else(|| panic!("{id} has no text"));
            assert!(!text.generated, "{id} is written by hand");
        }
    }

    #[tokio::test]
    async fn a_quest_entry_carries_its_text_and_says_where_it_came_from() {
        let (status, generated) = get_json("/db/quests/1000").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(generated["generated"], true);
        assert!(generated["guide"].as_array().is_some_and(|steps| !steps.is_empty()));
        let (_, by_hand) = get_json("/db/quests/3027").await;
        assert_eq!(by_hand["recommended_level"], 1);
        assert_eq!(by_hand["generated"], false);
        let (_, merchant) = get_json("/db/quests/1009").await;
        assert_eq!(merchant["recommended_level"], "Unknown");
    }

    #[tokio::test]
    async fn a_quest_names_its_prerequisites_and_an_empty_list_when_it_has_none() {
        let (status, started) = get_json("/db/quests/8133").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(started["prerequisites"][0]["quest"], 8132);
        assert_eq!(started["prerequisites"][0]["state"], "started");
        let (_, merchant) = get_json("/db/quests/1009").await;
        assert_eq!(merchant["prerequisites"], json!([]));
        let (_, gate) = get_json("/db/quests/2158").await;
        assert_eq!(gate["level_override"]["level"], 70);
    }

    #[tokio::test]
    async fn a_quest_without_text_has_null_text_fields() {
        let id = quest_catalog().keys().filter(|id| !texts().contains_key(&id.to_string())).min().copied().unwrap();
        let (status, quest) = get_json(&format!("/db/quests/{id}")).await;
        assert_eq!(status, StatusCode::OK);
        assert!(quest["description"].is_null());
        assert!(quest["recommended_level"].is_null());
        assert!(quest["prerequisites"].is_null());
    }
}
