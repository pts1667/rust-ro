//! `/db/statuses/<name>`: a status effect of `config/status_effect.json`, the states, flags and bonuses it applies.
use std::sync::OnceLock;

use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, embedded_json, matches, not_found, wiki_text, with_fields};

static STATUSES: OnceLock<Value> = OnceLock::new();

fn status_list() -> &'static [Value] {
    embedded_json(&STATUSES, include_str!("../../../../../config/status_effect.json")).as_array().map_or(&[], |statuses| statuses.as_slice())
}

fn name_of(status: &Value) -> &str {
    status["Status"].as_str().unwrap_or_default()
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut names: Vec<&str> = status_list().iter().map(name_of).filter(|name| matches(query.q.as_deref(), &[name])).collect();
    names.sort_unstable();
    names.into_iter().map(|name| Link::new("statuses", name, json!(name), name)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let status = status_list().iter().find(|status| name_of(status).eq_ignore_ascii_case(key)).ok_or_else(|| not_found("status", key))?;
    Ok(with_fields(status.clone(), [("wiki", wiki_text("statuses", name_of(status)))]))
}
