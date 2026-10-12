//! `/db/maps/<name>`: a map with its flags, walkable regions, warps, the NPCs that send characters elsewhere, monster spawns,
//! the NPCs placed on it and the quests they start.
use std::collections::BTreeSet;

use script_sdk::Entry;
use serde_json::{Value, json};

use super::{DbError, Link, ListQuery, index, matches, mobs, not_found, npcs, quests, transports, warps};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_regions;
use crate::server::model::warp::Warp;
use crate::server::script::entries;
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
    let regions = map_regions::of_map(map.name());
    let warps: Vec<Value> = map
        .warps()
        .iter()
        .map(|warp| {
            json!({
                "name": warp.name,
                "href": warps::href(warp),
                "x": warp.x,
                "y": warp.y,
                "x_size": warp.x_size,
                "y_size": warp.y_size,
                "region": regions.as_ref().and_then(|regions| regions.of_area(warp.x, warp.y, warp.x_size, warp.y_size)),
                "to": {
                    "map": link(&warp.dest_map_name),
                    "x": warp.to_x,
                    "y": warp.to_y,
                    "region": map_regions::of_map(&warp.dest_map_name).and_then(|regions| regions.landing(warp.to_x, warp.to_y)),
                },
            })
        })
        .collect();
    let mut arriving: Vec<&Warp> = index::get().map_arrivals.get(map.name()).into_iter().flatten().copied().collect();
    arriving.sort_by(|a, b| (&a.map_name, a.x, a.y).cmp(&(&b.map_name, b.x, b.y)));
    let connected: BTreeSet<&str> = map
        .warps()
        .iter()
        .map(|warp| warp.dest_map_name.as_str())
        .chain(arriving.iter().map(|warp| warp.map_name.as_str()))
        .filter(|name| *name != map.name())
        .collect();
    let arrivals: Vec<Value> = arriving
        .iter()
        .map(|warp| {
            json!({
                "name": warp.name,
                "href": warps::href(warp),
                "from": {
                    "map": link(&warp.map_name),
                    "x": warp.x,
                    "y": warp.y,
                    "region": map_regions::of_map(&warp.map_name).and_then(|regions| regions.of_area(warp.x, warp.y, warp.x_size, warp.y_size)),
                },
                "land": { "x": warp.to_x, "y": warp.to_y, "region": regions.as_ref().and_then(|regions| regions.landing(warp.to_x, warp.to_y)) },
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
        .map(|script| {
            json!({
                "npc": npcs::link(&script.map_name, &script.name),
                "x": script.x,
                "y": script.y,
                // NPCs stand on walls too: the region is the one of the cells around, where a character talks from
                "region": regions.as_ref().and_then(|regions| regions.landing(script.x, script.y)),
                "shop": index::shop_offers(script).is_some(),
            })
        })
        .collect();
    let offered: BTreeSet<u32> = map
        .scripts()
        .iter()
        .filter_map(|script| entries::resolve(script.entry_id))
        .filter_map(|named| match named.entry {
            Entry::Npc(name) => Some(quests::offered_by(&named.module, &name)),
            _ => None,
        })
        .flatten()
        .copied()
        .collect();
    Ok(json!({
        "name": map.name(),
        "x_size": map.x_size(),
        "y_size": map.y_size(),
        "flags": flags,
        "regions": regions.as_ref().map(|regions| regions.sizes().iter().enumerate().map(|(index, cells)| json!({ "region": index + 1, "cells": cells })).collect::<Vec<Value>>()),
        "warps": warps,
        "arrivals": arrivals,
        "transports": map.scripts().iter().filter_map(transports::transport).collect::<Vec<Value>>(),
        "transport_arrivals": transports::arrivals(map.name()),
        "connected_maps": connected.into_iter().map(link).collect::<Vec<Link>>(),
        "mob_spawns": spawns,
        "npcs": placed,
        "quests": offered.into_iter().map(quests::link).collect::<Vec<Link>>(),
    }))
}
