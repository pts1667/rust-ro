//! `/db/skills/<id or name>`: a skill of `config/skill.json` with its metadata, the jobs that learn it and the monsters that cast it.
use std::sync::OnceLock;

use serde_json::{Value, json};

use super::index;
use super::{DbError, Link, ListQuery, embedded_json, jobs, matches, mobs, not_found, to_json, with_fields};

static SKILLS: OnceLock<Value> = OnceLock::new();
static METADATA: OnceLock<Vec<Value>> = OnceLock::new();

fn skill_list() -> &'static [Value] {
    embedded_json(&SKILLS, include_str!("../../../../../config/skill.json"))["skills"].as_array().map_or(&[], |skills| skills.as_slice())
}

fn metadata_list() -> &'static [Value] {
    METADATA.get_or_init(|| serde_json::from_str(include_str!("../../script/skill_metadata.json")).unwrap_or_default())
}

fn name_of(id: u32) -> Option<&'static str> {
    skill_list().iter().find(|skill| skill["id"].as_u64() == Some(u64::from(id)))?["name"].as_str()
}

pub fn link(id: u32) -> Link {
    let name = name_of(id).map_or_else(|| id.to_string(), str::to_string);
    Link::new("skills", &id.to_string(), json!(id), &name)
}

/// A skill named by the skill tree, which keeps the name when the skill is unknown.
pub fn link_by_name(name: &str) -> Link {
    match skill_list().iter().find(|skill| skill["name"].as_str() == Some(name)).and_then(|skill| skill["id"].as_u64()) {
        Some(id) => link(id as u32),
        None => Link::new("skills", name, Value::Null, name),
    }
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut skills: Vec<(u64, &str)> = skill_list()
        .iter()
        .filter_map(|skill| Some((skill["id"].as_u64()?, skill["name"].as_str()?)))
        .filter(|(_, name)| matches(query.q.as_deref(), &[name]))
        .collect();
    skills.sort_by_key(|(id, _)| *id);
    skills.into_iter().map(|(id, _)| link(id as u32)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let skill = match key.parse::<u64>() {
        Ok(id) => skill_list().iter().find(|skill| skill["id"].as_u64() == Some(id)),
        Err(_) => skill_list().iter().find(|skill| skill["name"].as_str().is_some_and(|name| name.eq_ignore_ascii_case(key))),
    }
    .ok_or_else(|| not_found("skill", key))?;
    let id = skill["id"].as_u64().unwrap_or_default() as u32;
    let name = skill["name"].as_str().unwrap_or_default();

    let metadata = metadata_list().iter().find(|entry| entry["Id"].as_u64() == Some(u64::from(id))).cloned();
    let cast_by: Vec<Value> = index::get().skill_mobs.get(&id).into_iter().flatten().map(|mob| to_json(&mobs::link(*mob))).collect();
    Ok(with_fields(
        skill.clone(),
        [
            ("metadata", json!(metadata)),
            ("cast_by", json!(cast_by)),
            ("learned_by", to_json(&jobs::learners(name))),
        ],
    ))
}
