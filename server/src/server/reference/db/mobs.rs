//! `/db/mobs/<id or name>`: a monster with its drops, the skills it can use, where it spawns, the quests that hunt it and the summons that call it.
use serde_json::{Value, json};

use super::index;
use super::{DbError, Link, ListQuery, items, maps, matches, not_found, quests, skills, to_json, wiki_text, with_fields};
use crate::server::script::game_data;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::mob_skill_ai::MobSkillDatabase;
use crate::repository::model::mob_model::{Drop, MobModel};

pub fn link(id: i32) -> Link {
    let name = GlobalConfigService::instance().get_mob_safe(id).map_or_else(|| id.to_string(), |mob| mob.name_english.clone());
    Link::new("mobs", &id.to_string(), json!(id), &name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut mobs: Vec<&MobModel> = GlobalConfigService::instance()
        .mobs()
        .filter(|mob| matches(query.q.as_deref(), &[&mob.name_english, &mob.name]))
        .collect();
    mobs.sort_by_key(|mob| mob.id);
    mobs.into_iter().map(|mob| link(mob.id)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let mob = find(key).ok_or_else(|| not_found("mob", key))?;
    let index = index::get();
    let id = mob.id;

    let skill_uses: Vec<Value> = MobSkillDatabase::instance()
        .entries_of(id, index::is_boss(mob.mode))
        .map(|entry| {
            let mut value = to_json(entry);
            value["skill"] = to_json(&skills::link(entry.skill_id));
            value
        })
        .collect();
    let spawns: Vec<Value> = index
        .mob_spawns
        .get(&id)
        .into_iter()
        .flatten()
        .map(|spawn| {
            json!({
                "map": maps::link(&spawn.map),
                "x": spawn.x,
                "y": spawn.y,
                "x_size": spawn.x_size,
                "y_size": spawn.y_size,
                "amount": spawn.amount,
                "fixed_delay_ms": spawn.fixed_delay_ms,
                "random_delay_ms": spawn.random_delay_ms,
                "type": spawn.kind,
            })
        })
        .collect();
    let quest_links: Vec<Value> = index.mob_quests.get(&id).into_iter().flatten().map(|quest_id| to_json(&quests::link(*quest_id))).collect();
    let summoned_by: Vec<Value> = index
        .mob_summons
        .get(&id)
        .into_iter()
        .flatten()
        .filter_map(|group_id| game_data::data().summons.iter().find(|group| group.id == *group_id))
        .map(|group| json!({ "group": group.id, "name": group.name }))
        .collect();

    Ok(with_fields(
        to_json(mob),
        [
            ("mvp", json!(mob.is_mvp())),
            ("drops", drops(&mob.drops)),
            ("mvp_drops", drops(&mob.mvp_drops)),
            ("skills", json!(skill_uses)),
            ("spawns", json!(spawns)),
            ("quests", json!(quest_links)),
            ("summoned_by", json!(summoned_by)),
            ("wiki", wiki_text("mobs", &id.to_string())),
        ],
    ))
}

/// A name several monsters share answers with the lowest id, so the same key always gives the same monster.
fn find(key: &str) -> Option<&'static MobModel> {
    let config = GlobalConfigService::instance();
    match key.parse::<i32>() {
        Ok(id) => config.get_mob_safe(id),
        Err(_) => config.mobs().filter(|mob| mob.name.eq_ignore_ascii_case(key) || mob.name_english.eq_ignore_ascii_case(key)).min_by_key(|mob| mob.id),
    }
}

fn drops(list: &[Drop]) -> Value {
    let drops: Vec<Value> = list.iter().map(|drop| json!({ "item": items::link(drop.item_id), "rate": drop.rate, "card": drop.is_card })).collect();
    json!(drops)
}
