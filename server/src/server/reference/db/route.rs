//! `/db/route?from=<map>&to=<map>&x=<x>&y=<y>`: the fewest hops from one map to another, for a client that walks them in
//! order. A hop is a warp cell or a warper NPC (with the menu entries that lead there). Only ways the data states are
//! taken: no guessed way back of a warp, no random warp, no NPC whose menu is unknown (guards, Kafras).
//! Each hop's `path` lists the cells to walk to on its map, each one leg the client's own A* can walk.
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

use axum::Json;
use axum::extract::Query;
use map_cache::MapCells;
use movement::hierarchy::{self, Cell, NavHierarchy};
use movement::path::path_search_client_side_algorithm;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{DbError, destinations, not_found, transports};
use crate::server::model::map_regions;
use crate::server::model::warp::Warp;
use crate::server::service::global_config_service::GlobalConfigService;

pub const MAX_HOPS: usize = 20;
/// The client talks to an NPC from this many cells away, |dx| and |dy| each.
const TALK_RANGE: u16 = 2;

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
    let start = query.x.zip(query.y);
    let walks = tokio::task::spawn_blocking({
        let hops = hops.clone();
        move || walks(start, &hops)
    })
    .await
    .unwrap_or_default();
    let hops: Vec<Value> = hops
        .iter()
        .zip(walks.into_iter().chain(std::iter::repeat(None)))
        .map(|(hop, walk)| {
            let mut value = hop.to_json();
            if let Some(walk) = walk {
                value["path"] = json!(walk.iter().map(|(x, y)| [x, y]).collect::<Vec<_>>());
            }
            value
        })
        .collect();
    Ok(Json(json!({ "from": from, "to": to, "region": region, "hops": hops })))
}

/// The waypoints of each hop, see [`Ground::walk`]: from `start` for the first, from where the one before lands for the
/// others. `None` where no walk is known.
fn walks(start: Option<Cell>, hops: &[Hop]) -> Vec<Option<Vec<Cell>>> {
    let mut from = start;
    hops.iter()
        .map(|hop| {
            let walk = from.and_then(|from| ground(&hop.map)?.walk(from, hop));
            from = Some((hop.to_x, hop.to_y));
            walk
        })
        .collect()
}

/// The cells of a map from the map cache, and its pathfinding hierarchy.
struct Ground {
    cells: MapCells,
    navigation: NavHierarchy,
}

/// Grounds kept for the next routes; the cache starts over when it is full.
const CACHED_GROUNDS: usize = 64;

fn ground(map: &str) -> Option<Arc<Ground>> {
    static GROUNDS: OnceLock<Mutex<HashMap<String, Arc<Ground>>>> = OnceLock::new();
    let grounds = GROUNDS.get_or_init(Default::default);
    if let Some(ground) = grounds.lock().unwrap().get(map) {
        return Some(ground.clone());
    }
    let cells = map_cache::read_mcache(Path::new(unsafe { crate::MAP_DIR }), map).ok()?;
    let ground = Arc::new(Ground::new(cells)?);
    let mut grounds = grounds.lock().unwrap();
    if grounds.len() >= CACHED_GROUNDS {
        grounds.clear();
    }
    grounds.insert(map.to_string(), ground.clone());
    Some(ground)
}

impl Ground {
    fn new(cells: MapCells) -> Option<Self> {
        (cells.cells.len() == usize::from(cells.xs) * usize::from(cells.ys))
            .then(|| Self { navigation: NavHierarchy::build(cells.xs, cells.ys, &cells.cells), cells })
    }

    /// Where `hop` is taken from: a cell of the warp's area, or one the client talks to the NPC from.
    fn ends(&self, hop: &Hop) -> Vec<Cell> {
        let (reach_x, reach_y) = match hop.way {
            Way::Warp { x_size, y_size } => (x_size, y_size),
            Way::Warper { .. } => (TALK_RANGE, TALK_RANGE),
        };
        let xs = hop.x.saturating_sub(reach_x)..=hop.x.saturating_add(reach_x);
        xs.flat_map(|x| (hop.y.saturating_sub(reach_y)..=hop.y.saturating_add(reach_y)).map(move |y| (x, y)))
            .filter(|(x, y)| self.cells.is_walkable(*x, *y))
            .collect()
    }

    /// Waypoints from `from` to the closest end of `hop`, `from` left out and that end last. The client's own A* walks
    /// each leg, as it gives up on longer ones: a leg it cannot walk is cut along a local route.
    fn walk(&self, from: Cell, hop: &Hop) -> Option<Vec<Cell>> {
        let to = self.ends(hop).into_iter().filter(|end| self.navigation.reachable(from, *end)).min_by_key(|end| hierarchy::octile(from, *end))?;
        if to == from {
            return Some(Vec::new());
        }
        if self.client_walks(from, to) {
            return Some(vec![to]);
        }
        let mut legs = Vec::new();
        let mut at = from;
        for waypoint in self.navigation.route(&self.cells.cells, from, to)? {
            if waypoint == at {
                continue;
            }
            if self.client_walks(at, waypoint) {
                legs.push(waypoint);
                at = waypoint;
                continue;
            }
            let local = hierarchy::local_route(self.cells.xs, self.cells.ys, &self.cells.cells, at, waypoint, hierarchy::CHUNK)?;
            for leg in hierarchy::sub_legs(&local) {
                if !self.client_walks(at, leg) {
                    return None;
                }
                legs.push(leg);
                at = leg;
            }
        }
        Some(legs)
    }

    fn client_walks(&self, (from_x, from_y): Cell, to: Cell) -> bool {
        let path = path_search_client_side_algorithm(self.cells.xs, self.cells.ys, &self.cells.cells, from_x, from_y, to.0, to.1);
        // The search answers a path to the last cell it tried when it gives up
        path.last().is_some_and(|end| (end.x, end.y) == to)
    }
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
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;

    use super::*;

    /// An open 120×120 map split by a wall at x = 60, open only for y ≥ 100.
    fn walled() -> Ground {
        let size = 120u16;
        let cells = (0..size)
            .flat_map(|y| (0..size).map(move |x| (x, y)))
            .map(|(x, y)| if x == 60 && y < 100 { 0 } else { CellType::Walkable.as_flag() })
            .collect();
        Ground::new(MapCells { name: "walled".into(), xs: size, ys: size, cells }).unwrap()
    }

    fn hop_at(x: u16, y: u16, way: Way) -> Hop {
        Hop { map: "walled".into(), x, y, region: None, to_map: "elsewhere".into(), to_x: 0, to_y: 0, to_region: None, way }
    }

    #[test]
    fn a_long_walk_comes_as_legs_the_client_walks() {
        let ground = walled();
        let from = (10, 10);
        let hop = hop_at(110, 10, Way::Warp { x_size: 1, y_size: 1 });
        assert!(!ground.client_walks(from, (110, 10)), "too far for the client in one go");
        let walk = ground.walk(from, &hop).unwrap();
        let end = *walk.last().unwrap();
        assert!(end.0.abs_diff(110) <= 1 && end.1.abs_diff(10) <= 1, "ends in the warp: {end:?}");
        let mut at = from;
        for leg in walk {
            assert!(ground.client_walks(at, leg), "{at:?} to {leg:?}");
            at = leg;
        }
    }

    #[test]
    fn a_warper_is_walked_up_to_and_a_short_walk_is_one_leg() {
        let ground = walled();
        let hop = hop_at(30, 30, Way::Warper { npc: "Warper".into(), destination: "Prontera".into(), menu: vec![] });
        assert_eq!(ground.walk((20, 30), &hop), Some(vec![(28, 30)]), "to the closest cell the NPC is talked to from");
        assert_eq!(ground.walk((29, 31), &hop), Some(vec![]), "already in talking range");
    }

    #[test]
    fn an_end_out_of_reach_has_no_walk() {
        let size = 40u16;
        let cells = (0..size).flat_map(|y| (0..size).map(move |x| (x, y))).map(|(x, _)| if x == 20 { 0 } else { CellType::Walkable.as_flag() }).collect();
        let ground = Ground::new(MapCells { name: "split".into(), xs: size, ys: size, cells }).unwrap();
        assert_eq!(ground.walk((5, 5), &hop_at(35, 5, Way::Warp { x_size: 0, y_size: 0 })), None);
    }

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

