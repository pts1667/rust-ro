//! `/db/warps/<map>/<x>,<y>`: a warp cell of a loaded map, the tile that takes a character to another map. Listings take `q`.
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, maps, matches, not_found};
use crate::server::model::warp::Warp;
use crate::server::service::global_config_service::GlobalConfigService;

fn link(warp: &Warp) -> Link {
    let key = format!("{}/{},{}", warp.map_name, warp.x, warp.y);
    Link::new("warps", &key, json!(key), &warp.name)
}

pub fn href(warp: &Warp) -> String {
    link(warp).href
}

pub fn links(query: &ListQuery) -> Vec<Link> {
    let mut warps: Vec<&Warp> = GlobalConfigService::instance()
        .maps()
        .values()
        .flat_map(|map| map.warps())
        .filter(|warp| matches(query.q.as_deref(), &[&warp.name]))
        .collect();
    warps.sort_by(|a, b| (&a.map_name, a.x, a.y).cmp(&(&b.map_name, b.x, b.y)));
    warps.into_iter().map(link).collect()
}

/// The map, x and y of a `<map>/<x>,<y>` key.
pub fn parse_cell(key: &str) -> Option<(&str, u16, u16)> {
    let (map, cell) = key.split_once('/')?;
    let (x, y) = cell.split_once(',')?;
    Some((map, x.parse().ok()?, y.parse().ok()?))
}

pub fn entry(key: &str) -> Result<Value, DbError> {
    let (map_name, x, y) = parse_cell(key).ok_or_else(|| DbError::Invalid("a warp is at /db/warps/<map>/<x>,<y>".into()))?;
    let map =GlobalConfigService::instance().find_map(map_name).ok_or_else(|| not_found("map", map_name))?;
    let warp = map.warps().iter().find(|warp| warp.x == x && warp.y == y).ok_or_else(|| not_found("warp", key))?;
    Ok(json!({
        "key": key,
        "name": warp.name,
        "map": maps::link(&warp.map_name),
        "x": warp.x,
        "y": warp.y,
        "x_size": warp.x_size,
        "y_size": warp.y_size,
        "to": { "map": maps::link(&warp.dest_map_name), "x": warp.to_x, "y": warp.to_y },
    }))
}
