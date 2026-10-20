//! `/db/destinations/<map>/<x>,<y>`: a place on the warper's menu, the list of `config/wasm/warps.json` that the warper NPCs offer.
//! Listings take `q`.
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, maps, matches, not_found, warps};

#[derive(Deserialize)]
pub struct Destination {
    category: String,
    group: String,
    pub name: String,
    pub map: String,
    pub x: u16,
    pub y: u16,
}

static DESTINATIONS: OnceLock<Vec<Destination>> = OnceLock::new();

pub fn destinations() -> &'static [Destination] {
    DESTINATIONS.get_or_init(|| serde_json::from_str(include_str!("../../../../../config/wasm/warps.json")).expect("Embedded configuration file is invalid"))
}

/// The entries a player picks on the warper to go there: category, group, then the name unless the group holds one place.
pub fn menu_path(destination: &Destination) -> Vec<&str> {
    let mut path = vec![destination.category.as_str(), destination.group.as_str()];
    let in_group = destinations().iter().filter(|other| other.category == destination.category && other.group == destination.group).count();
    if in_group > 1 {
        path.push(&destination.name);
    }
    path
}

/// Destination names repeat (seven of them are shared), so the key is the place itself.
fn link(destination: &Destination) -> Link {
    let key = format!("{}/{},{}", destination.map, destination.x, destination.y);
    Link::new("destinations", &key, json!(key), &destination.name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    destinations()
        .iter()
        .filter(|destination| query.category.as_deref().is_none_or(|category| category.eq_ignore_ascii_case(&destination.category)))
        .filter(|destination| matches(query.q.as_deref(), &[&destination.name]))
        .map(link)
        .collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let (map_name, x, y) = warps::parse_cell(key).ok_or_else(|| DbError::Invalid("a destination is at /db/destinations/<map>/<x>,<y>".into()))?;
    let destination = destinations()
        .iter()
        .find(|destination| destination.map == map_name && destination.x == x && destination.y == y)
        .ok_or_else(|| not_found("destination", key))?;
    Ok(json!({
        "key": key,
        "name": destination.name,
        "category": destination.category,
        "group": destination.group,
        "map": maps::link(&destination.map),
        "x": destination.x,
        "y": destination.y,
    }))
}
