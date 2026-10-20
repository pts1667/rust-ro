//! `/db/route?from=<map>&to=<map>&x=<x>&y=<y>`: the fewest hops from one map to another, for a client that walks them in
//! order. A hop is a warp cell or a warper NPC (with the menu entries that lead there). Only ways the data states are
//! taken: no guessed way back of a warp, no random warp, no NPC whose menu is unknown (guards, Kafras).
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;

use axum::Json;
use axum::extract::Query;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{DbError, destinations, not_found, transports};
use crate::server::model::map_regions;
use crate::server::model::warp::Warp;
use crate::server::service::global_config_service::GlobalConfigService;

pub const MAX_HOPS: usize = 20;

#[derive(Deserialize)]
pub struct RouteQuery {
    from: Option<String>,
    to: Option<String>,
    /// The player's cell on `from`, which picks the walkable region the route starts in.
    x: Option<u16>,
    y: Option<u16>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Way {
    Warp { x_size: u16, y_size: u16 },
    /// The warper: talking to it and picking `menu` in order.
    Warper { npc: String, destination: String, menu: Vec<String> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hop {
    pub map: String,
    pub x: u16,
    pub y: u16,
    /// The walkable region of `map` it is used from, `None` when unknown (taken to connect to every region).
    pub region: Option<u32>,
    pub to_map: String,
    pub to_x: u16,
    pub to_y: u16,
    pub to_region: Option<u32>,
    pub way: Way,
}

impl Hop {
    fn usable_from(&self, region: Option<u32>) -> bool {
        region.is_none() || self.region.is_none() || self.region == region
    }

    fn to_json(&self) -> Value {
        let mut value = json!({
            "map": self.map,
            "x": self.x,
            "y": self.y,
            "region": self.region,
            "to_map": self.to_map,
            "to_x": self.to_x,
            "to_y": self.to_y,
            "to_region": self.to_region,
            "checks": [],
        });
        match &self.way {
            Way::Warp { x_size, y_size } => {
                value["kind"] = json!("warp");
                value["x_size"] = json!(x_size);
                value["y_size"] = json!(y_size);
            }
            Way::Warper { npc, destination, menu } => {
                value["kind"] = json!("npc");
                value["npc"] = json!(npc);
                value["destination"] = json!(destination);
                value["menu"] = json!(menu);
            }
        }
        value
    }
}

pub async fn answer(Query(query): Query<RouteQuery>) -> Result<Json<Value>, DbError> {
    let (Some(from), Some(to)) = (query.from.as_deref(), query.to.as_deref()) else {
        return Err(DbError::Invalid("a route is at /db/route?from=<map>&to=<map>, with the optional x and y of the player on from".into()));
    };
    let config = GlobalConfigService::instance();
    let from_map = config.find_map(from).ok_or_else(|| not_found("map", from))?;
    config.find_map(to).ok_or_else(|| not_found("map", to))?;
    let region = match (query.x, query.y) {
        (Some(x), Some(y)) => map_regions::of_map(from_map.name()).and_then(|regions| regions.landing(x, y)),
        (None, None) => None,
        _ => return Err(DbError::Invalid("give both x and y, or neither".into())),
    };
    let edges = graph();
    let hops = plan(from, region, to, |map| edges.get(map).map_or(&[][..], Vec::as_slice))
        .ok_or_else(|| DbError::NotFound(format!("no route from {from} to {to} within {MAX_HOPS} hops")))?;
    Ok(Json(json!({ "from": from, "to": to, "region": region, "hops": hops.iter().map(Hop::to_json).collect::<Vec<Value>>() })))
}

/// Fewest hops from the part `region` of `from` (`None` for any) to any part of `to`.
pub fn plan<'a>(from: &str, region: Option<u32>, to: &str, edges: impl Fn(&str) -> &'a [Hop]) -> Option<Vec<Hop>> {
    if from == to {
        return Some(Vec::new());
    }
    type Node = (String, Option<u32>);
    let start: Node = (from.to_string(), region);
    let mut previous: HashMap<Node, (Node, &Hop)> = HashMap::new();
    let mut depth: HashMap<Node, usize> = HashMap::from([(start.clone(), 0)]);
    let mut queue = VecDeque::from([start.clone()]);
    while let Some(node) = queue.pop_front() {
        let hops = depth[&node];
        if hops >= MAX_HOPS {
            continue;
        }
        for hop in edges(&node.0) {
            let next: Node = (hop.to_map.clone(), hop.to_region);
            // A warp inside a map only helps when it leads to another known part of it (the floors of a tower)
            let inner = hop.to_map == hop.map;
            if !hop.usable_from(node.1) || depth.contains_key(&next) || (inner && (hop.to_region.is_none() || hop.to_region == node.1)) {
                continue;
            }
            depth.insert(next.clone(), hops + 1);
            previous.insert(next.clone(), (node.clone(), hop));
            if hop.to_map == to {
                let mut path = Vec::new();
                let mut current = next;
                while current != start {
                    let (back, hop) = previous.remove(&current)?;
                    path.push(hop.clone());
                    current = back;
                }
                path.reverse();
                return Some(path);
            }
            queue.push_back(next);
        }
    }
    None
}

fn graph() -> &'static HashMap<String, Vec<Hop>> {
    static GRAPH: OnceLock<HashMap<String, Vec<Hop>>> = OnceLock::new();
    GRAPH.get_or_init(|| GlobalConfigService::instance().maps().keys().map(|name| (name.clone(), hops_out_of(name))).collect())
}

fn hops_out_of(map_name: &str) -> Vec<Hop> {
    let config = GlobalConfigService::instance();
    let Some(map) = config.find_map(map_name) else { return Vec::new() };
    let regions = map_regions::of_map(map_name);
    let mut hops: Vec<Hop> = live_warps(map.warps())
        .into_iter()
        .filter_map(|warp| {
            let destination = config.find_map(&warp.dest_map_name)?;
            // A warp that lands on a warp of the map it leads to goes on at once: prt_castle's way to prt_gld lands in
            // prt_gld's way back to prontera
            let landing = live_warps(destination.warps()).into_iter().find(|next| covers(next, warp.to_x, warp.to_y) && config.find_map(&next.dest_map_name).is_some());
            let (to_map, to_x, to_y) = landing.map_or((&warp.dest_map_name, warp.to_x, warp.to_y), |next| (&next.dest_map_name, next.to_x, next.to_y));
            Some(Hop {
                map: map_name.to_string(),
                x: warp.x,
                y: warp.y,
                region: regions.as_ref().and_then(|regions| regions.of_area(warp.x, warp.y, warp.x_size, warp.y_size)),
                to_map: to_map.clone(),
                to_x,
                to_y,
                to_region: map_regions::of_map(to_map).and_then(|regions| regions.landing(to_x, to_y)),
                way: Way::Warp { x_size: warp.x_size, y_size: warp.y_size },
            })
        })
        .collect();
    for warper in map.scripts().iter().filter(|script| transports::is_warper(script)) {
        // NPCs stand on walls too: the region is the one of the cells around, where a character talks from
        let region = regions.as_ref().and_then(|regions| regions.landing(warper.x, warper.y));
        for destination in destinations::destinations().iter().filter(|destination| config.find_map(&destination.map).is_some()) {
            hops.push(Hop {
                map: map_name.to_string(),
                x: warper.x,
                y: warper.y,
                region,
                to_map: destination.map.clone(),
                to_x: destination.x,
                to_y: destination.y,
                to_region: map_regions::of_map(&destination.map).and_then(|regions| regions.landing(destination.x, destination.y)),
                way: Way::Warper {
                    npc: warper.name.clone(),
                    destination: destination.name.clone(),
                    menu: destinations::menu_path(destination).into_iter().map(str::to_string).collect(),
                },
            });
        }
    }
    hops
}

/// Whether the cell is inside the warp's area (`x_size` and `y_size` are half its width and height).
fn covers(warp: &Warp, x: u16, y: u16) -> bool {
    x.abs_diff(warp.x) <= warp.x_size && y.abs_diff(warp.y) <= warp.y_size
}

/// The warps a character can take: not one whose whole area lies inside a bigger warp, which takes the step first
/// (prontera's way to prt_gld lies inside its way to prt_castle).
fn live_warps(warps: &[Warp]) -> Vec<&Warp> {
    let inside = |a: &Warp, b: &Warp| {
        a.x.saturating_sub(a.x_size) >= b.x.saturating_sub(b.x_size)
            && a.x + a.x_size <= b.x + b.x_size
            && a.y.saturating_sub(a.y_size) >= b.y.saturating_sub(b.y_size)
            && a.y + a.y_size <= b.y + b.y_size
            && (a.x_size, a.y_size) != (b.x_size, b.y_size)
    };
    let mut seen = HashSet::new();
    warps.iter().filter(|a| !warps.iter().any(|b| inside(a, b)) && seen.insert((a.x, a.y, &a.dest_map_name))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn warp(map: &str, region: Option<u32>, to_map: &str, to_region: Option<u32>) -> Hop {
        Hop {
            map: map.into(),
            x: 10,
            y: 10,
            region,
            to_map: to_map.into(),
            to_x: 20,
            to_y: 20,
            to_region,
            way: Way::Warp { x_size: 1, y_size: 1 },
        }
    }

    fn route(graph: &HashMap<&str, Vec<Hop>>, from: &str, region: Option<u32>, to: &str) -> Option<Vec<String>> {
        let found = plan(from, region, to, |map| graph.get(map).map_or(&[][..], Vec::as_slice))?;
        Some(found.iter().map(|hop| format!("{}>{}", hop.map, hop.to_map)).collect())
    }

    #[test]
    fn the_route_takes_the_fewest_hops_and_none_to_stay() {
        let graph = HashMap::from([
            ("town", vec![warp("town", None, "field", None), warp("town", None, "cave", None)]),
            ("field", vec![warp("field", None, "dungeon", None)]),
            ("cave", vec![warp("cave", None, "deep", None)]),
            ("deep", vec![warp("deep", None, "dungeon", None)]),
        ]);
        assert_eq!(route(&graph, "town", None, "dungeon").unwrap(), ["town>field", "field>dungeon"]);
        assert_eq!(route(&graph, "town", None, "town").unwrap(), Vec::<String>::new());
        assert!(route(&graph, "dungeon", None, "town").is_none());
    }

    #[test]
    fn a_warp_of_another_region_is_reached_through_the_warps_inside_the_map() {
        let graph = HashMap::from([(
            "tower",
            vec![warp("tower", Some(1), "tower", Some(2)), warp("tower", Some(2), "roof", None), warp("tower", Some(1), "tower", None)],
        )]);
        assert_eq!(route(&graph, "tower", Some(1), "roof").unwrap(), ["tower>tower", "tower>roof"]);
        assert_eq!(route(&graph, "tower", Some(2), "roof").unwrap(), ["tower>roof"]);
        assert!(route(&graph, "tower", Some(3), "roof").is_none());
    }

    #[test]
    fn a_route_longer_than_the_cap_is_not_found() {
        let names: Vec<String> = (0..=MAX_HOPS + 1).map(|index| format!("map{index}")).collect();
        let graph: HashMap<&str, Vec<Hop>> = names.windows(2).map(|pair| (pair[0].as_str(), vec![warp(&pair[0], None, &pair[1], None)])).collect();
        assert_eq!(route(&graph, "map0", None, &names[MAX_HOPS]).unwrap().len(), MAX_HOPS);
        assert!(route(&graph, "map0", None, &names[MAX_HOPS + 1]).is_none());
    }

    #[test]
    fn a_warper_hop_names_its_menu_entries() {
        let hop = Hop {
            way: Way::Warper { npc: "Warper#prt".into(), destination: "Geffen".into(), menu: vec!["Towns".into(), "Geffen".into()] },
            ..warp("prontera", Some(1), "geffen", Some(1))
        };
        let value = hop.to_json();
        assert_eq!(value["kind"], "npc");
        assert_eq!(value["menu"], json!(["Towns", "Geffen"]));
        assert_eq!(value["checks"], json!([]));
        assert_eq!(warp("prontera", None, "prt_fild08", None).to_json()["kind"], "warp");
    }

    #[test]
    fn a_group_of_one_place_is_picked_without_a_third_menu() {
        let prontera = destinations::destinations().iter().find(|destination| destination.map == "prontera").unwrap();
        assert_eq!(destinations::menu_path(prontera), ["Towns", "Prontera"]);
        let field = destinations::destinations().iter().find(|destination| destination.map == "prt_fild08").unwrap();
        assert_eq!(destinations::menu_path(field), ["Fields", "Prontera Fields", "Prontera Field 8"]);
    }
}
