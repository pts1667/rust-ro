//! `/db/jobs/<id or name>`: a job of `config/job.json` with its skill tree from `config/skill_tree.json`.
use std::sync::OnceLock;

use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, embedded_json, matches, not_found, skills, to_json, with_fields};

static JOBS: OnceLock<Value> = OnceLock::new();
static TREES: OnceLock<Value> = OnceLock::new();

fn jobs() -> Vec<(&'static str, &'static Value)> {
    embedded_json(&JOBS, include_str!("../../../../../config/job.json"))["jobs"].as_object().into_iter().flat_map(|jobs| jobs.iter()).map(|(name, job)| (name.as_str(), job)).collect()
}

fn trees() -> impl Iterator<Item = (&'static str, &'static Value)> {
    embedded_json(&TREES, include_str!("../../../../../config/skill_tree.json"))["jobs_tree"].as_object().into_iter().flat_map(|trees| trees.iter()).map(|(name, tree)| (name.as_str(), tree))
}

/// Tree keys use underscores where job names have spaces, and the case differs (`Star_Gladiator`).
fn normalized(name: &str) -> String {
    name.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_ascii_lowercase()
}

fn job_id(name: &str) -> Option<u64> {
    jobs().into_iter().find(|(job, _)| *job == name)?.1["id"].as_u64()
}

pub fn link(name: &str) -> Link {
    match job_id(name) {
        Some(id) => Link::new("jobs", &id.to_string(), json!(id), name),
        None => Link::new("jobs", name, Value::Null, name),
    }
}

/// The jobs whose skill tree has the skill.
pub fn learners(skill: &str) -> Vec<Link> {
    trees()
        .filter(|(_, tree)| tree["tree"].as_array().is_some_and(|entries| entries.iter().any(|entry| entry["name"].as_str() == Some(skill))))
        .map(|(job, _)| link(job))
        .collect()
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut jobs: Vec<(u64, &str)> = jobs()
        .into_iter()
        .filter(|(name, _)| matches(query.q.as_deref(), &[name]))
        .map(|(name, job)| (job["id"].as_u64().unwrap_or(u64::MAX), name))
        .collect();
    jobs.sort_by_key(|(id, _)| *id);
    jobs.into_iter().map(|(_, name)| link(name)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let (name, job) = match key.parse::<u64>() {
        Ok(id) => jobs().into_iter().find(|(_, job)| job["id"].as_u64() == Some(id)),
        Err(_) => jobs().into_iter().find(|(name, _)| name.eq_ignore_ascii_case(key)),
    }
    .ok_or_else(|| not_found("job", key))?;

    let skill_tree = trees().find(|(tree, _)| normalized(tree) == normalized(name)).map(|(_, tree)| {
        let entries: Vec<Value> = tree["tree"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|entry| {
                let mut value = entry.clone();
                let skill = entry["name"].as_str().map(skills::link_by_name);
                value["skill"] = json!(skill.as_ref().map(to_json));
                value
            })
            .collect();
        let mut value = tree.clone();
        value["tree"] = json!(entries);
        value
    });
    Ok(with_fields(job.clone(), [("name", json!(name)), ("skill_tree", json!(skill_tree))]))
}
