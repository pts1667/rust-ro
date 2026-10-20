//! NPCs that send characters elsewhere, the ways between maps that are not warp cells: guards, sewer entrances, ship captains.
//! Their destinations are `config/npc_warps.json`, a static scan of the scripts (`tools/scripts-import/extract_npc_warps.py`)
//! that only finds literal destinations; the warper NPCs offer every place of `/db/destinations`.
use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

use script_sdk::Entry;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{maps, npcs};
use crate::server::model::map_regions;
use crate::server::model::script::Script;
use crate::server::script::entries;
use crate::server::service::global_config_service::GlobalConfigService;

/// The module and entry of the hand-written warper NPC.
const WARPER: (&str, &str) = ("systems", "warper");

#[derive(Deserialize)]
pub struct NpcWarp {
    pub map: String,
    /// 0 (or below) lets the server pick a random cell of the map.
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize)]
pub struct NpcWarps {
    pub warps: Vec<NpcWarp>,
    /// The warp happens on walking into the NPC's area (`OnTouch`), not by talking.
    pub touch: bool,
    /// What the script reads on the way to the warp: `zeny`, `item`, `quest`, `level`, `job` or `variable`.
    pub checks: Vec<String>,
}

static WARPS: OnceLock<BTreeMap<String, BTreeMap<String, NpcWarps>>> = OnceLock::new();
static ARRIVALS: OnceLock<HashMap<String, Vec<(&'static Script, &'static NpcWarp)>>> = OnceLock::new();

fn module_and_name(script: &Script) -> Option<(String, String)> {
    let named = entries::resolve(script.entry_id)?;
    match named.entry {
        Entry::Npc(name) => Some((named.module, name)),
        _ => None,
    }
}

fn warps_of(script: &Script) -> Option<&'static NpcWarps> {
    let (module, name) = module_and_name(script)?;
    WARPS
        .get_or_init(|| serde_json::from_str(include_str!("../../../../../config/npc_warps.json")).expect("Embedded NPC warps are invalid"))
        .get(&module)?
        .get(&name)
}

pub fn is_warper(script: &Script) -> bool {
    module_and_name(script).is_some_and(|(module, name)| module == WARPER.0 && name == WARPER.1)
}

fn known_map(name: &str) -> bool {
    GlobalConfigService::instance().find_map(name).is_some()
}

fn landing(map: &str, x: i32, y: i32) -> Option<u32> {
    let (x, y) = (u16::try_from(x).ok().filter(|x| *x > 0)?, u16::try_from(y).ok().filter(|y| *y > 0)?);
    map_regions::of_map(map)?.landing(x, y)
}

/// Where a character stands to use the NPC: next to it, or inside its area for a touch NPC.
fn region_of(script: &Script) -> Option<u32> {
    let regions = map_regions::of_map(&script.map_name)?;
    regions.of_area(script.x, script.y, script.x_size, script.y_size)
}

/// The NPC as a way out of its map, `None` when its script sends nobody anywhere that the scan found.
pub fn transport(script: &'static Script) -> Option<Value> {
    let base = json!({
        "npc": npcs::link(&script.map_name, &script.name),
        "x": script.x,
        "y": script.y,
        "region": region_of(script),
    });
    if is_warper(script) {
        let mut value = base;
        value["warper"] = json!(true);
        value["to"] = json!("/db/destinations");
        return Some(value);
    }
    let found = warps_of(script)?;
    let to: Vec<Value> = found
        .warps
        .iter()
        .filter(|warp| known_map(&warp.map))
        .map(|warp| json!({ "map": maps::link(&warp.map), "x": warp.x, "y": warp.y, "region": landing(&warp.map, warp.x, warp.y) }))
        .collect();
    if to.is_empty() {
        return None;
    }
    let mut value = base;
    // A touch NPC without an area is triggered by standing on its own cell
    value["touch"] = json!(found.touch);
    value["area"] = json!({ "x_size": script.x_size, "y_size": script.y_size });
    value["checks"] = json!(found.checks);
    value["to"] = json!(to);
    Some(value)
}

/// The NPCs of other maps that send characters to `map`.
pub fn arrivals(map: &str) -> Vec<Value> {
    let index = ARRIVALS.get_or_init(|| {
        let mut index: HashMap<String, Vec<(&'static Script, &'static NpcWarp)>> = HashMap::new();
        for source in GlobalConfigService::instance().maps().values() {
            for script in source.scripts() {
                for warp in warps_of(script).map_or(&[][..], |found| found.warps.as_slice()) {
                    if warp.map != script.map_name && known_map(&warp.map) {
                        index.entry(warp.map.clone()).or_default().push((script, warp));
                    }
                }
            }
        }
        for arriving in index.values_mut() {
            arriving.sort_by(|a, b| (&a.0.map_name, &a.0.name, a.1.x, a.1.y).cmp(&(&b.0.map_name, &b.0.name, b.1.x, b.1.y)));
        }
        index
    });
    index
        .get(map)
        .into_iter()
        .flatten()
        .map(|(script, warp)| {
            json!({
                "npc": npcs::link(&script.map_name, &script.name),
                "from": { "map": maps::link(&script.map_name), "x": script.x, "y": script.y, "region": region_of(script) },
                "land": { "x": warp.x, "y": warp.y, "region": landing(&warp.map, warp.x, warp.y) },
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn warps(module: &str, npc: &str) -> &'static NpcWarps {
        WARPS
            .get_or_init(|| serde_json::from_str(include_str!("../../../../../config/npc_warps.json")).expect("Embedded NPC warps are invalid"))
            .get(module)
            .and_then(|npcs| npcs.get(npc))
            .unwrap_or_else(|| panic!("{module} {npc} sends nobody anywhere"))
    }

    #[test]
    fn guards_and_entrances_that_send_characters_away_are_listed() {
        let guard = warps("quests_morocc", "Continental Guard#01");
        assert!(guard.touch);
        assert!(guard.warps.iter().any(|warp| warp.map == "morocc"));
        let sewer = warps("quests_prontera", "Culvert Guardian");
        assert!(!sewer.touch);
        assert!(sewer.warps.iter().any(|warp| warp.map == "prt_sewb1"));
        assert!(sewer.checks.contains(&"variable".to_string()));
    }
}
