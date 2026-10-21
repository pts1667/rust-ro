//! Writes `scripts/plugins/alive_towns/src/towns_data.rs`: per town, the doors, shop counters, idle spots and plazas the
//! townsfolk of the `alive_towns` plugin walk between, all within the walkable region of the town's arrival cell, and the
//! monsters of the maps around it that their quests send players after.
//!
//! Run from the repository root: `cargo run --package tools --bin alive-towns-data --release`.

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fmt::Write as _;
use std::path::Path;

use regex_lite::Regex;
use serde::Deserialize;

type Cell = (u16, u16);

const OUTPUT: &str = "scripts/plugins/alive_towns/src/towns_data.rs";
/// Maps at most this many warps away from a town are its hunting grounds.
const HUNT_HOPS: u8 = 3;
const MAX_PREY: usize = 40;
/// Fewer of a monster on a map are not worth a quest there.
const MIN_SPAWNS: u32 = 5;
/// Loot dropped less often than this (out of 10000) is not asked for.
const MIN_LOOT_RATE: u32 = 800;
/// Monsters this slow never leave their cell (Pupa, eggs, plants).
const STILL_SPEED: u16 = 1000;
const SPOT_STEP: u16 = 8;
const PLAZA_STEP: u16 = 16;
const MOVES: [(i32, i32); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
/// `x` and `y` steps of the eight RO directions, 0 being north (+y) and turning counterclockwise.
const DIRECTIONS: [(i32, i32); 8] = [(0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)];

#[derive(Deserialize)]
struct WarpEntry {
    category: String,
    name: String,
    map: String,
    x: u16,
    y: u16,
}

#[derive(Deserialize)]
struct NpcEntry {
    map_name: String,
    x: u16,
    y: u16,
    dir: u16,
    name: String,
    sprite: String,
    #[serde(default)]
    test: bool,
}

#[derive(Deserialize)]
struct MobFile {
    mobs: Vec<MobEntry>,
}

#[derive(Deserialize)]
struct MobEntry {
    id: u32,
    name_english: String,
    level: u16,
    speed: u16,
    exp: u32,
    job_exp: u32,
    #[serde(default)]
    monster_class: String,
    drops: Vec<DropEntry>,
}

#[derive(Deserialize)]
struct DropEntry {
    item_id: u32,
    rate: u32,
    is_card: bool,
}

#[derive(Deserialize)]
struct ItemFile {
    items: Vec<ItemEntry>,
}

#[derive(Deserialize)]
struct ItemEntry {
    id: u32,
    name_english: String,
    item_type: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Spawn {
    map: String,
    mob: u32,
    count: u32,
}

#[derive(Debug, PartialEq)]
struct Prey {
    mob: u32,
    name: String,
    level: u16,
    base_exp: u32,
    job_exp: u32,
    map: String,
    map_name: String,
    hops: u8,
    still: bool,
    /// (item id, name, rate out of 10000).
    loot: Option<(u32, String, u32)>,
}

#[derive(Debug, Clone, PartialEq)]
struct Warp {
    map: String,
    at: Cell,
    span: Cell,
    destination: String,
    arrival: Cell,
}

#[derive(Debug, Clone, PartialEq)]
struct Npc {
    at: Cell,
    dir: u16,
    display: String,
    person: bool,
}

#[derive(Debug, PartialEq)]
struct Door {
    cell: Cell,
    front: Cell,
    inside: String,
}

#[derive(Debug, PartialEq)]
struct Counter {
    npc: String,
    front: Cell,
    face: u16,
}

#[derive(Debug, PartialEq)]
struct TownData {
    doors: Vec<Door>,
    counters: Vec<Counter>,
    spots: Vec<Cell>,
    plazas: Vec<Cell>,
    prey: Vec<Prey>,
}

struct Grid {
    x_size: u16,
    y_size: u16,
    /// Cells reachable from the town's arrival cell.
    region: Vec<bool>,
}

impl Grid {
    fn new(x_size: u16, y_size: u16, walkable: &[bool], spawn: Cell) -> Option<(Self, Cell)> {
        let index = |(x, y): Cell| usize::from(x) + usize::from(y) * usize::from(x_size);
        let spawn = nearest((x_size, y_size), spawn, 6, |cell| walkable[index(cell)])?;
        let mut region = vec![false; walkable.len()];
        region[index(spawn)] = true;
        let mut queue = VecDeque::from([spawn]);
        while let Some(cell) = queue.pop_front() {
            for next in MOVES.iter().filter_map(|step| offset((x_size, y_size), cell, *step)) {
                if walkable[index(next)] && !region[index(next)] {
                    region[index(next)] = true;
                    queue.push_back(next);
                }
            }
        }
        Some((Self { x_size, y_size, region }, spawn))
    }

    fn size(&self) -> Cell {
        (self.x_size, self.y_size)
    }

    fn inside(&self, cell: Cell) -> bool {
        cell.0 < self.x_size && cell.1 < self.y_size && self.region[usize::from(cell.0) + usize::from(cell.1) * usize::from(self.x_size)]
    }

    /// Whether every cell within `radius` of `cell` is in the region.
    fn open_around(&self, (x, y): Cell, radius: u16) -> bool {
        x >= radius
            && y >= radius
            && (x - radius..=x + radius).all(|cx| (y - radius..=y + radius).all(|cy| self.inside((cx, cy))))
    }
}

fn offset((x_size, y_size): Cell, (x, y): Cell, (dx, dy): (i32, i32)) -> Option<Cell> {
    let (nx, ny) = (i32::from(x) + dx, i32::from(y) + dy);
    (nx >= 0 && ny >= 0 && nx < i32::from(x_size) && ny < i32::from(y_size)).then(|| (nx as u16, ny as u16))
}

/// The closest cell to `center` within `radius` (Chebyshev rings, row order inside a ring) that `accept`s.
fn nearest(size: Cell, center: Cell, radius: u16, accept: impl Fn(Cell) -> bool) -> Option<Cell> {
    (0..=i32::from(radius)).find_map(|ring| {
        (-ring..=ring)
            .flat_map(|dy| (-ring..=ring).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| dx.abs() == ring || dy.abs() == ring)
            .filter_map(|step| offset(size, center, step))
            .find(|cell| accept(*cell))
    })
}

fn within(cell: Cell, center: Cell, (span_x, span_y): Cell) -> bool {
    cell.0.abs_diff(center.0) <= span_x && cell.1.abs_diff(center.1) <= span_y
}

fn is_door(destination: &str) -> bool {
    Regex::new(r"_in(\d+)?$|_in_|church|temple|_tower$|_pre$").unwrap().is_match(destination)
}

fn town_data(grid: &Grid, map: &str, warps: &[Warp], npcs: &[Npc]) -> TownData {
    let size = grid.size();
    let near_warp = |cell: Cell, margin: u16| warps.iter().any(|warp| warp.map == map && within(cell, warp.at, (warp.span.0 + margin, warp.span.1 + margin)));
    let near_npc = |cell: Cell, margin: u16| npcs.iter().any(|npc| within(cell, npc.at, (margin, margin)));

    let mut doors = Vec::new();
    for warp in warps.iter().filter(|warp| warp.map == map && is_door(&warp.destination)) {
        let Some(cell) = nearest(size, warp.at, warp.span.0.max(warp.span.1), |cell| grid.inside(cell) && within(cell, warp.at, warp.span)) else { continue };
        let returned = warps
            .iter()
            .filter(|back| back.map == warp.destination && back.destination == map && within(back.arrival, warp.at, (8, 8)))
            .map(|back| back.arrival)
            .find(|arrival| grid.inside(*arrival) && !near_warp(*arrival, 0));
        let front = returned.or_else(|| nearest(size, warp.at, 8, |cell| grid.inside(cell) && !near_warp(cell, 1)));
        if let Some(front) = front {
            doors.push(Door { cell, front, inside: warp.destination.clone() });
        }
    }

    let occupied: HashSet<Cell> = npcs.iter().map(|npc| npc.at).collect();
    let mut counters = Vec::new();
    let mut fronts = HashSet::new();
    for npc in npcs.iter().filter(|npc| npc.person && !npc.display.is_empty()) {
        let (dx, dy) = DIRECTIONS[usize::from(npc.dir % 8)];
        let front = [2, 1, 3]
            .into_iter()
            .filter_map(|distance| offset(size, npc.at, (dx * distance, dy * distance)))
            .find(|cell| grid.inside(*cell) && !occupied.contains(cell) && !near_warp(*cell, 0));
        if let Some(front) = front.filter(|front| fronts.insert(*front)) {
            counters.push(Counter { npc: npc.display.clone(), front, face: (npc.dir + 4) % 8 });
        }
    }

    let sample = |step: u16, accept: &dyn Fn(Cell) -> bool| {
        let mut picked = Vec::new();
        for y in (step / 2..size.1).step_by(usize::from(step)) {
            for x in (step / 2..size.0).step_by(usize::from(step)) {
                if let Some(cell) = nearest(size, (x, y), step / 2, accept) {
                    picked.push(cell);
                }
            }
        }
        picked
    };
    let spots = sample(SPOT_STEP, &|cell| grid.open_around(cell, 1) && !near_warp(cell, 2) && !near_npc(cell, 1));
    let plazas = sample(PLAZA_STEP, &|cell| grid.open_around(cell, 3) && !near_warp(cell, 3) && !near_npc(cell, 3));
    TownData { doors, counters, spots, plazas, prey: Vec::new() }
}

/// Maps within [`HUNT_HOPS`] warps of `town` with their distance, reached through neither another town nor a building.
fn nearby_maps(town: &str, warps: &[Warp], towns: &HashSet<String>) -> BTreeMap<String, u8> {
    let mut distance = BTreeMap::from([(town.to_string(), 0u8)]);
    let mut queue = VecDeque::from([town.to_string()]);
    while let Some(map) = queue.pop_front() {
        let hops = distance[&map];
        if hops >= HUNT_HOPS {
            continue;
        }
        for warp in warps.iter().filter(|warp| warp.map == map) {
            if towns.contains(&warp.destination) || is_door(&warp.destination) || distance.contains_key(&warp.destination) {
                continue;
            }
            distance.insert(warp.destination.clone(), hops + 1);
            queue.push_back(warp.destination.clone());
        }
    }
    distance.remove(town);
    distance
}

/// The common monsters of the maps around a town, each on the closest map that has a good number of them, the ones
/// closest to town first when there are too many, then by level.
fn town_prey(maps: &BTreeMap<String, u8>, spawns: &[Spawn], mobs: &HashMap<u32, &MobEntry>, items: &HashMap<u32, &ItemEntry>, map_names: &HashMap<String, String>) -> Vec<Prey> {
    let mut counts: BTreeMap<(String, u32), u32> = BTreeMap::new();
    for spawn in spawns.iter().filter(|spawn| maps.contains_key(&spawn.map)) {
        *counts.entry((spawn.map.clone(), spawn.mob)).or_default() += spawn.count;
    }
    let mut best: BTreeMap<u32, (u8, u32, String)> = BTreeMap::new();
    for ((map, mob), count) in counts {
        let Some(entry) = mobs.get(&mob) else { continue };
        if count < MIN_SPAWNS || entry.exp == 0 || entry.monster_class != "Normal" {
            continue;
        }
        let hops = maps[&map];
        let better = best.get(&mob).is_none_or(|(best_hops, best_count, _)| (hops, std::cmp::Reverse(count)) < (*best_hops, std::cmp::Reverse(*best_count)));
        if better {
            best.insert(mob, (hops, count, map));
        }
    }
    let mut picked: Vec<(u32, (u8, u32, String))> = best.into_iter().collect();
    picked.sort_by_key(|(mob, (hops, count, _))| (*hops, std::cmp::Reverse(*count), *mob));
    picked.truncate(MAX_PREY);
    let mut prey: Vec<Prey> = picked
        .into_iter()
        .map(|(mob, (hops, _, map))| {
            let entry = mobs[&mob];
            let loot = entry
                .drops
                .iter()
                .filter(|drop| !drop.is_card && drop.rate >= MIN_LOOT_RATE)
                .filter_map(|drop| items.get(&drop.item_id).filter(|item| item.item_type == "Etc").map(|item| (drop.item_id, item.name_english.clone(), drop.rate)))
                .max_by_key(|(id, _, rate)| (*rate, std::cmp::Reverse(*id)));
            Prey {
                mob,
                name: entry.name_english.clone(),
                level: entry.level,
                base_exp: entry.exp,
                job_exp: entry.job_exp,
                map_name: map_names.get(&map).cloned().unwrap_or_else(|| map.clone()),
                map,
                hops,
                still: entry.speed >= STILL_SPEED,
                loot,
            }
        })
        .collect();
    prey.sort_by_key(|prey| (prey.level, prey.mob));
    prey
}

fn parse_spawns(text: &str) -> Vec<Spawn> {
    let line = Regex::new(r"^(\w+),\d+,\d+(?:,\d+,\d+)?\tmonster\t[^\t]+\t(\d+),(\d+)").unwrap();
    text.lines()
        .filter_map(|text| line.captures(text))
        .map(|caps| Spawn { map: caps[1].to_string(), mob: caps[2].parse().unwrap(), count: caps[3].parse().unwrap() })
        .collect()
}

fn files_under(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap_or_else(|error| panic!("{}: {error}", dir.display())) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else if path.extension().is_some_and(|extension| extension == "txt") {
            files.push(path);
        }
    }
    files.sort();
    files
}

fn parse_warps(text: &str) -> Vec<Warp> {
    let line = Regex::new(r"^(\w+),(\d+),(\d+),\d+\s+warp2?\s+\S+\s+(\d+),(\d+),(\w+),(\d+),(\d+)").unwrap();
    text.lines()
        .filter_map(|text| line.captures(text))
        .map(|caps| {
            let number = |index: usize| caps[index].parse::<u16>().unwrap();
            Warp {
                map: caps[1].to_string(),
                at: (number(2), number(3)),
                span: (number(4), number(5)),
                destination: caps[6].to_string(),
                arrival: (number(7), number(8)),
            }
        })
        .collect()
}

fn sprite_names(constants: &str) -> HashMap<i32, String> {
    let entry = Regex::new(r#""(\d\w*)" => Value::new_number\((-?\d+)\)"#).unwrap();
    let mut names = HashMap::new();
    for caps in entry.captures_iter(constants) {
        names.entry(caps[2].parse().unwrap()).or_insert_with(|| caps[1].to_string());
    }
    names
}

fn is_person(sprite: &str) -> bool {
    Regex::new(r"^[1248]W?_[MF](_|$)|^4W_(KID|SAILOR)$|^4_KID01$|^8W_SOLDIER$").unwrap().is_match(sprite)
}

fn display_name(name: &str) -> String {
    name.split('#').next().unwrap_or_default().split("::").next().unwrap_or_default().trim().to_string()
}

fn render(towns: &[(String, String, TownData)]) -> String {
    let mut out = String::from("// Generated by tools/alive-towns-data, do not edit: rerun `cargo run --package tools --bin alive-towns-data --release`.\n");
    out.push_str("use crate::towns::{Counter, Door, Loot, Prey, Town};\n\n");
    out.push_str("pub static TOWNS: &[Town] = &[\n");
    let cells = |cells: &[Cell]| cells.iter().map(|(x, y)| format!("({x}, {y})")).collect::<Vec<_>>().join(", ");
    for (map, name, data) in towns {
        let _ = writeln!(out, "    Town {{
        map: {map:?},
        name: {name:?},");
        out.push_str("        doors: &[\n");
        for door in &data.doors {
            let _ = writeln!(out, "            Door {{ cell: ({}, {}), front: ({}, {}), inside: {:?} }},", door.cell.0, door.cell.1, door.front.0, door.front.1, door.inside);
        }
        out.push_str("        ],\n        counters: &[\n");
        for counter in &data.counters {
            let _ = writeln!(out, "            Counter {{ npc: {:?}, front: ({}, {}), face: {} }},", counter.npc, counter.front.0, counter.front.1, counter.face);
        }
        let _ = writeln!(out, "        ],\n        spots: &[{}],\n        plazas: &[{}],", cells(&data.spots), cells(&data.plazas));
        out.push_str("        prey: &[\n");
        for prey in &data.prey {
            let loot = match &prey.loot {
                Some((item, name, rate)) => format!("Some(Loot {{ item: {item}, name: {name:?}, rate: {rate} }})"),
                None => "None".to_string(),
            };
            let _ = writeln!(
                out,
                "            Prey {{ mob: {}, name: {:?}, level: {}, base_exp: {}, job_exp: {}, map: {:?}, map_name: {:?}, still: {}, loot: {loot} }},",
                prey.mob, prey.name, prey.level, prey.base_exp, prey.job_exp, prey.map, prey.map_name, prey.still
            );
        }
        out.push_str("        ],\n    },\n");
    }
    out.push_str("];\n");
    out
}

fn main() {
    let read = |path: &str| std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{path}: {error}"));
    let entries: Vec<WarpEntry> = serde_json::from_str(&read("config/wasm/warps.json")).unwrap();
    let npc_entries: Vec<NpcEntry> = serde_json::from_str(&read("config/wasm/npcs.json")).unwrap();
    let sprites = sprite_names(&read("server/src/server/script/constant.rs"));
    let mut warps = Vec::new();
    let mut files: Vec<_> = std::fs::read_dir("config/npc/pre-re/warps/cities").unwrap().map(|entry| entry.unwrap().path()).collect();
    files.sort();
    for file in files {
        warps.extend(parse_warps(&std::fs::read_to_string(file).unwrap()));
    }

    let mut all_warps = Vec::new();
    for dir in ["cities", "fields", "dungeons"] {
        for file in files_under(&Path::new("config/npc/pre-re/warps").join(dir)) {
            all_warps.extend(parse_warps(&read(file.to_str().unwrap())));
        }
    }
    let mut spawns = Vec::new();
    for file in files_under(Path::new("config/npc/pre-re/mobs")) {
        spawns.extend(parse_spawns(&read(file.to_str().unwrap())));
    }
    let mob_file: MobFile = serde_json::from_str(&read("config/mobs.json")).unwrap();
    let mobs: HashMap<u32, &MobEntry> = mob_file.mobs.iter().map(|mob| (mob.id, mob)).collect();
    let item_file: ItemFile = serde_json::from_str(&read("config/items.json")).unwrap();
    let items: HashMap<u32, &ItemEntry> = item_file.items.iter().map(|item| (item.id, item)).collect();
    let map_names: HashMap<String, String> = serde_json::from_str(&read("config/map_names.json")).unwrap();

    let mut towns = BTreeMap::new();
    for entry in entries.into_iter().filter(|entry| entry.category == "Towns") {
        towns.entry(entry.map.clone()).or_insert(entry);
    }
    let town_maps: HashSet<String> = towns.keys().cloned().collect();
    let mut generated = Vec::new();
    for (map, town) in towns {
        let cells = match map_cache::read_mcache(Path::new("config/maps/pre-re"), &map) {
            Ok(cells) => cells,
            Err(error) => {
                eprintln!("skipping {map}: {error:?}");
                continue;
            }
        };
        let walkable: Vec<bool> = cells.cells.iter().map(|cell| cell & 1 == 1).collect();
        let Some((grid, _)) = Grid::new(cells.xs, cells.ys, &walkable, (town.x, town.y)) else {
            eprintln!("skipping {map}: no walkable cell near the arrival ({}, {})", town.x, town.y);
            continue;
        };
        let npcs: Vec<Npc> = npc_entries
            .iter()
            .filter(|npc| npc.map_name == map && !npc.test)
            .map(|npc| {
                let sprite = npc.sprite.parse::<i32>().ok().and_then(|id| sprites.get(&id).cloned()).unwrap_or_else(|| npc.sprite.clone());
                Npc { at: (npc.x, npc.y), dir: npc.dir, display: display_name(&npc.name), person: is_person(&sprite) }
            })
            .collect();
        let mut data = town_data(&grid, &map, &warps, &npcs);
        data.prey = town_prey(&nearby_maps(&map, &all_warps, &town_maps), &spawns, &mobs, &items, &map_names);
        println!(
            "{map:<12} {:>3} doors {:>3} counters {:>4} spots {:>3} plazas {:>3} prey",
            data.doors.len(),
            data.counters.len(),
            data.spots.len(),
            data.plazas.len(),
            data.prey.len()
        );
        generated.push((map, town.name, data));
    }
    std::fs::write(OUTPUT, render(&generated)).unwrap();
    println!("wrote {OUTPUT}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 40×30 town: a wall at x = 20 with no gap cuts off the east, a building block (5..10, 5..10) holds a door at its
    /// south face, and a shopkeeper at (14, 20) faces south.
    fn town() -> (Grid, Vec<Warp>, Vec<Npc>) {
        let (x_size, y_size) = (40u16, 30u16);
        let mut walkable = vec![true; usize::from(x_size) * usize::from(y_size)];
        let mut block = |x: u16, y: u16| walkable[usize::from(x) + usize::from(y) * usize::from(x_size)] = false;
        for y in 0..y_size {
            block(20, y);
        }
        for x in 5..10 {
            for y in 5..10 {
                block(x, y);
            }
        }
        let (grid, _) = Grid::new(x_size, y_size, &walkable, (2, 2)).unwrap();
        let warps = vec![
            Warp { map: "town".into(), at: (7, 4), span: (1, 1), destination: "town_in".into(), arrival: (50, 50) },
            Warp { map: "town_in".into(), at: (50, 48), span: (1, 1), destination: "town".into(), arrival: (7, 2) },
            Warp { map: "town".into(), at: (0, 15), span: (1, 3), destination: "town_fild01".into(), arrival: (10, 10) },
        ];
        let npcs = vec![
            Npc { at: (14, 20), dir: 4, display: "Tool Dealer".into(), person: true },
            Npc { at: (16, 20), dir: 4, display: "Bulletin Board".into(), person: false },
        ];
        (grid, warps, npcs)
    }

    #[test]
    fn town_points_stay_in_the_arrival_region_and_away_from_warps() {
        let (grid, warps, npcs) = town();
        let data = town_data(&grid, "town", &warps, &npcs);
        assert_eq!(data.doors, vec![Door { cell: (7, 4), front: (7, 2), inside: "town_in".into() }], "the field warp is no door");
        assert_eq!(data.counters, vec![Counter { npc: "Tool Dealer".into(), front: (14, 18), face: 0 }]);
        assert!(!data.spots.is_empty() && !data.plazas.is_empty());
        for cell in data.spots.iter().chain(&data.plazas) {
            assert!(cell.0 < 20, "{cell:?} is beyond the wall");
            assert!(!within(*cell, (0, 15), (3, 5)), "{cell:?} is on the field warp");
        }
        for plaza in &data.plazas {
            assert!(grid.open_around(*plaza, 2), "{plaza:?} has no free ring");
        }
    }

    #[test]
    fn prey_lives_on_the_nearest_fields_and_drops_common_loot() {
        let warp = |map: &str, destination: &str| Warp { map: map.into(), at: (0, 0), span: (1, 1), destination: destination.into(), arrival: (1, 1) };
        let warps = vec![
            warp("town", "town_fild01"),
            warp("town", "town_in"),
            warp("town_fild01", "town_fild02"),
            warp("town_fild01", "other_town"),
            warp("town_fild02", "town_fild03"),
            warp("town_fild03", "town_fild04"),
            warp("other_town", "other_fild"),
        ];
        let towns = HashSet::from(["town".to_string(), "other_town".to_string()]);
        let maps = nearby_maps("town", &warps, &towns);
        assert_eq!(maps, BTreeMap::from([("town_fild01".to_string(), 1), ("town_fild02".to_string(), 2), ("town_fild03".to_string(), 3)]));

        let spawns = parse_spawns(
            "town_fild01,0,0\tmonster\tPoring\t1002,3\ntown_fild02,0,0\tmonster\tPoring\t1002,40\ntown_fild01,0,0\tmonster\tPoring\t1002,4\n\
             town_fild01,10,10,5,5\tmonster\tGreen Plant\t1080,9\ntown_fild02,0,0\tboss_monster\tEclipse\t1093,1\nother_fild,0,0\tmonster\tLunatic\t1063,50\n",
        );
        assert_eq!(spawns.len(), 5, "boss_monster lines are left out");
        let mob = |id: u32, name: &str, exp: u32, drops: Vec<DropEntry>| MobEntry { id, name_english: name.into(), level: 1, speed: 200, exp, job_exp: 1, monster_class: "Normal".into(), drops };
        let mob_entries = [
            mob(1002, "Poring", 2, vec![DropEntry { item_id: 909, rate: 7000, is_card: false }, DropEntry { item_id: 4001, rate: 9000, is_card: true }, DropEntry { item_id: 501, rate: 9000, is_card: false }]),
            mob(1080, "Green Plant", 0, vec![]),
            mob(1063, "Lunatic", 6, vec![]),
        ];
        let mobs: HashMap<u32, &MobEntry> = mob_entries.iter().map(|mob| (mob.id, mob)).collect();
        let item_entries = [
            ItemEntry { id: 909, name_english: "Jellopy".into(), item_type: "Etc".into() },
            ItemEntry { id: 501, name_english: "Red Potion".into(), item_type: "Healing".into() },
            ItemEntry { id: 4001, name_english: "Poring Card".into(), item_type: "Card".into() },
        ];
        let items: HashMap<u32, &ItemEntry> = item_entries.iter().map(|item| (item.id, item)).collect();
        let names = HashMap::from([("town_fild01".to_string(), "Town Field".to_string())]);
        let prey = town_prey(&maps, &spawns, &mobs, &items, &names);
        assert_eq!(prey.len(), 1, "plants give no experience and the Lunatics are beyond another town");
        assert_eq!((prey[0].map.as_str(), prey[0].map_name.as_str(), prey[0].hops), ("town_fild01", "Town Field", 1), "7 Porings next door beat 40 further away");
        assert_eq!(prey[0].loot, Some((909, "Jellopy".into(), 7000)));
        assert!(!prey[0].still);
    }

    #[test]
    fn rathena_warp_lines_and_names_are_read() {
        let warps = parse_warps("prontera,107,215,0\twarp\tprt01\t2,2,prt_in,240,139\n// comment\n");
        assert_eq!(warps, vec![Warp { map: "prontera".into(), at: (107, 215), span: (2, 2), destination: "prt_in".into(), arrival: (240, 139) }]);
        assert!(is_door("prt_in") && is_door("payon_in01") && is_door("prt_church") && !is_door("prt_fild08") && !is_door("pay_arche"));
        assert!(is_person("4_M_04") && is_person("8_F") && is_person("4W_SAILOR") && !is_person("2_POSTBOX") && !is_person("GUILD_FLAG"));
        assert_eq!(display_name("Tool Dealer#prt"), "Tool Dealer");
    }
}
