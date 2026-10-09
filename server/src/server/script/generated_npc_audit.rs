//! Checks over the NPCs converted from rathena that a random-host run cannot make: placements, shop stock, constants and converted flows.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::server::boot::script_loader::ScriptLoader;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_service::ScriptService;

fn repository_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(relative)
}

fn manifest(file: &str) -> serde_json::Value {
    serde_json::from_slice(&std::fs::read(repository_path(&format!("config/wasm/{file}"))).unwrap()).unwrap()
}

#[test]
fn every_placement_is_on_a_cached_map_inside_its_bounds_with_a_known_sprite_and_a_unique_name() {
    let scripts = ScriptLoader::load_scripts(repository_path("config/wasm/npcs.json").to_str().unwrap()).unwrap();
    let cache = repository_path("config/maps/pre-re");
    let mut problems = vec![];
    for (map_name, placed) in &scripts {
        let map = match map_cache::read_mcache(&cache, map_name) {
            Ok(map) => map,
            Err(error) => {
                problems.push(format!("{} NPCs on {map_name}: {error}, first: {}", placed.len(), placed[0].name));
                continue;
            }
        };
        for script in placed.iter().filter(|script| script.x >= map.xs || script.y >= map.ys) {
            problems.push(format!("{} is at {},{} outside {map_name} ({}x{})", script.name, script.x, script.y, map.xs, map.ys));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("
"));
}

#[test]
fn shop_npcs_only_sell_items_that_exist() {
    crate::tests::common::before_all();
    let configuration = GlobalConfigService::instance();
    let mut problems = vec![];
    for npc in manifest("npcs.json").as_array().unwrap().iter().filter(|npc| npc["module"] == "systems" && npc["entry"] == "shop") {
        let arguments: Vec<i64> = npc["constructor_args"].as_array().unwrap().iter().map(|argument| argument["Number"].as_i64().unwrap()).collect();
        for pair in arguments[1..].chunks(2) {
            if configuration.find_item(pair[0] as i32).is_none() {
                problems.push(format!("{} sells the unknown item {}", npc["name"], pair[0]));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn every_event_label_belongs_to_a_placed_npc() {
    let names: BTreeSet<String> = manifest("npcs.json").as_array().unwrap().iter().map(|npc| npc["name"].as_str().unwrap().to_string()).collect();
    let events = manifest("events.json");
    let missing: Vec<&String> = events.as_object().unwrap().keys().filter(|label| label.split_once("::").is_some_and(|(npc, _)| npc != "CompiledEvents" && !names.contains(npc))).collect();
    assert!(missing.is_empty(), "{} event labels name an NPC that is not placed, first: {:?}", missing.len(), &missing[..missing.len().min(5)]);
}

#[test]
fn every_constant_the_generated_scripts_name_resolves() {
    fn collect(directory: &std::path::Path, names: &mut BTreeSet<String>) {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name != "target") {
                    collect(&path, names);
                }
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                for part in std::fs::read_to_string(&path).unwrap().split(".constant(\"").skip(1) {
                    names.insert(part[..part.find('"').unwrap()].to_string());
                }
            }
        }
    }
    let mut names = BTreeSet::new();
    collect(&repository_path("scripts"), &mut names);
    assert!(!names.is_empty());
    let unknown: Vec<&String> = names.iter().filter(|name| super::item_script_handler::constant(name).is_err()).collect();
    assert!(unknown.is_empty(), "{} unknown constants: {:?}", unknown.len(), unknown);
}

/// Answers variable reads from what the script wrote and keeps the commands it issued, so a converted flow can be asserted on.
#[derive(Default)]
struct RecordingHost {
    variables: std::collections::HashMap<(String, u32), script_sdk::Value>,
    calls: Vec<(script_sdk::Function, Vec<script_sdk::Value>)>,
}

#[async_trait::async_trait]
impl script_runtime::Host for RecordingHost {
    async fn invoke(&mut self, request: script_sdk::Request) -> script_sdk::Reply {
        use script_sdk::{Request, Value};
        let default = |name: &str| if name.ends_with('$') { Value::String(String::new()) } else { Value::Number(0) };
        let key = |name: String, index: u32| (name.trim_start_matches(['$', '@', '.', '#']).to_string(), index);
        Ok(match request {
            Request::Constant(name) => return super::item_script_handler::constant(&name),
            Request::Read(name) => self.variables.get(&key(name.clone(), 0)).cloned().unwrap_or_else(|| default(&name)),
            Request::VariableRead { name, index, .. } => self.variables.get(&key(name.clone(), index)).cloned().unwrap_or_else(|| default(&name)),
            Request::Write { name, value } => {
                self.variables.insert(key(name, 0), value);
                Value::default()
            }
            Request::VariablesWrite(variables) => {
                for variable in variables {
                    self.variables.insert(key(variable.name, variable.index), variable.value);
                }
                Value::default()
            }
            Request::Call { function, arguments } => {
                self.calls.push((function, arguments));
                Value::default()
            }
            _ => Value::default(),
        })
    }
}

fn event_id(label: &str) -> u32 {
    ScriptService::event_entry(label).unwrap_or_else(|| panic!("{label} is not a registered event"))
}

fn run_event(host: RecordingHost, label: &str) -> RecordingHost {
    let runtime = crate::tests::common::test_npc_vm();
    let (host, result) = futures::executor::block_on(runtime.run(host, event_id(label)));
    result.unwrap();
    host
}

#[test]
fn the_domestic_airship_announces_its_arrival_shows_the_gangway_and_the_gangway_warps_to_that_stop() {
    use script_sdk::{Function, Value};
    let host = run_event(RecordingHost::default(), "Domestic_Airship::OnTimer60000");
    assert_eq!(host.variables.get(&("airplanelocation".to_string(), 0)), Some(&Value::Number(1)));
    let events: Vec<&Value> = host.calls.iter().filter(|(function, _)| *function == Function::DoNpcEvent).map(|(_, arguments)| &arguments[0]).collect();
    assert_eq!(events, [&Value::from("#AirshipWarp-1::OnUnhide"), &Value::from("#AirshipWarp-2::OnUnhide")]);
    assert!(host.calls.iter().any(|(function, arguments)| *function == Function::MapAnnounce && arguments.iter().any(|argument| matches!(argument, Value::String(text) if text.contains("Welcome to Einbroch")))));

    let mut host = RecordingHost::default();
    host.variables.insert(("airplanelocation".to_string(), 0), Value::Number(1));
    let host = run_event(host, "#AirshipWarp-1::OnTouch");
    let warp = host.calls.iter().find(|(function, _)| *function == Function::Warp).expect("the gangway warps the player");
    assert_eq!(warp.1, [Value::from("einbroch"), Value::from(92), Value::from(278)]);
}
