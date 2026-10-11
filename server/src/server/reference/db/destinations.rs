//! `/db/destinations/<map>/<x>,<y>`: a place on the warper's menu, the list of `config/wasm/warps.json` that the warper NPCs offer.
//! Listings take `q`.
use std::sync::OnceLock;

use serde::Deserialize;
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, maps, matches, not_found, warps};

#[derive(Deserialize)]
struct Destination {
    category: String,
    group: String,
    name: String,
    map: String,
    x: u16,
    y: u16,
}

static DESTINATIONS: OnceLock<Vec<Destination>> = OnceLock::new();

fn destinations() -> &'static [Destination] {
    DESTINATIONS.get_or_init(|| serde_json::from_str(include_str!("../../../../../config/wasm/warps.json")).expect("Embedded configuration file is invalid"))
}

/// Destination names repeat (seven of them are shared), so the key is the place itself.
fn link(destination: &Destination) -> Link {
    let key = format!("{}/{},{}", destination.map, destination.x, destination.y);
    Link::new("destinations", &key, json!(key), &destination.name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    destinations().iter().filter(|destination| matches(query.q.as_deref(), &[&destination.name])).map(link).collect()
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
