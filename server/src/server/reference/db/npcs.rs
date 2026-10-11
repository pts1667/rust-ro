//! `/db/npcs/<map>/<name>`: an NPC placed on a map, with the script it runs and the items a shop sells. Listings take `q` and `map`.
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, index, items, maps, matches, not_found, to_json};
use crate::server::script::entries;
use crate::server::service::global_config_service::GlobalConfigService;

/// Keyed by the map and the name; when two NPCs of a map share a name the first one answers.
pub fn link(map: &str, name: &str) -> Link {
    let key = format!("{map}/{name}");
    Link::new("npcs", &key, json!(key), name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut scripts: Vec<_> = GlobalConfigService::instance()
        .maps()
        .values()
        .flat_map(|map| map.scripts())
        .filter(|script| query.map.as_deref().is_none_or(|map| map == script.map_name))
        .filter(|script| matches(query.q.as_deref(), &[&script.name]))
        .collect();
    scripts.sort_by(|a, b| (&a.map_name, &a.name, a.x, a.y).cmp(&(&b.map_name, &b.name, b.x, b.y)));
    scripts.into_iter().map(|script| link(&script.map_name, &script.name)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let (map, name) = key.split_once('/').ok_or_else(|| DbError::Invalid("an NPC is at /db/npcs/<map>/<name>".into()))?;
    let script = GlobalConfigService::instance()
        .find_map(map)
        .and_then(|map| map.scripts().iter().find(|script| script.name == name))
        .ok_or_else(|| not_found("NPC", key))?;
    let named = entries::resolve(script.entry_id);
    let shop: Option<Vec<Value>> = index::shop_offers(script).map(|offers| {
        offers.into_iter().map(|(item, price)| json!({ "item": items::link(item), "price": price })).collect()
    });
    Ok(json!({
        "map": maps::link(&script.map_name),
        "name": script.name,
        "sprite": script.sprite,
        "x": script.x,
        "y": script.y,
        "dir": script.dir,
        "module": named.as_ref().map(|named| named.module.as_str()),
        "entry": named.as_ref().map(|named| to_json(&named.entry)),
        "constructor_args": to_json(&script.constructor_args),
        "shop": shop,
    }))
}
