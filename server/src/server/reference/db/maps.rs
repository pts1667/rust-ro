//! `/db/maps/<name>`: a map with its flags, warps, monster spawns and the NPCs placed on it.
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, index, matches, mobs, not_found, npcs};
use crate::server::model::map_flags::MapFlag;
use crate::server::service::global_config_service::GlobalConfigService;

pub fn link(name: &str) -> Link {
    Link::new("maps", name, json!(name), name)
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut names: Vec<&String> = GlobalConfigService::instance().maps().keys().filter(|name| matches(query.q.as_deref(), &[name])).collect();
    names.sort();
    names.into_iter().map(|name| link(name)).collect()
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let map = GlobalConfigService::instance().find_map(key).ok_or_else(|| not_found("map", key))?;
    let flags: Vec<&str> = MapFlag::CATALOGUE.iter().filter(|(flag, _)| map.flags().enabled(*flag)).map(|(_, name)| *name).collect();
    let warps: Vec<Value> = map
        .warps()
        .iter()
        .map(|warp| {
            json!({
                "name": warp.name,
                "x": warp.x,
                "y": warp.y,
                "x_size": warp.x_size,
                "y_size": warp.y_size,
                "to": { "map": link(&warp.dest_map_name), "x": warp.to_x, "y": warp.to_y },
            })
        })
        .collect();
    let spawns: Vec<Value> = map
        .mob_spawns()
        .iter()
        .map(|spawn| {
            json!({
                "mob": mobs::link(i32::from(spawn.mob_id)),
                "x": spawn.x,
                "y": spawn.y,
                "x_size": spawn.x_size,
                "y_size": spawn.y_size,
                "amount": spawn.to_spawn_amount,
                "type": index::kind_name(&spawn.mob_type),
            })
        })
        .collect();
    let placed: Vec<Value> = map
        .scripts()
        .iter()
        .map(|script| json!({ "npc": npcs::link(&script.map_name, &script.name), "x": script.x, "y": script.y, "shop": index::shop_offers(script).is_some() }))
        .collect();
    Ok(json!({
        "name": map.name(),
        "x_size": map.x_size(),
        "y_size": map.y_size(),
        "flags": flags,
        "warps": warps,
        "mob_spawns": spawns,
        "npcs": placed,
    }))
}
