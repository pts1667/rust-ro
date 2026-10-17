use std::collections::HashMap;

use script_sdk::{Entry, Value};
use serde::Deserialize;

use crate::server::model::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::model::script::Script;
use crate::server::script::constant::load_constant;
use crate::server::script::entries::intern;

pub struct ScriptLoader;

#[derive(Deserialize)]
struct NpcDefinition {
    map_name: String,
    name: String,
    sprite: String,
    x: u16,
    y: u16,
    dir: u16,
    module: String,
    entry: String,
    #[serde(default)]
    x_size: u16,
    #[serde(default)]
    y_size: u16,
    #[serde(default)]
    constructor_args: Vec<Value>,
    #[serde(default)]
    test: bool,
}

impl ScriptLoader {
    pub fn load_map_flags(path: &str) -> Result<HashMap<String, crate::server::model::map_flags::MapFlags>, String> {
        use crate::server::model::map_flags::{MapFlag, MapFlagDefinition, MapFlags};
        let bytes = std::fs::read(path).map_err(|error| format!("Cannot load {path}: {error}"))?;
        let definitions: Vec<MapFlagDefinition> = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
        let mut flags: HashMap<String, MapFlags> = HashMap::new();
        for definition in definitions {
            let map = crate::server::model::map::Map::name_without_ext(&definition.map).to_ascii_lowercase();
            if map.is_empty() {
                return Err("Map flag has no map name".into());
            }
            let entry = flags.entry(map).or_default();
            let flag = MapFlag::from_name(&definition.flag)?;
            if definition.enabled && flag == MapFlag::SkillDuration {
                let id = *definition.arguments.first().ok_or("Skill duration requires a skill")?;
                if crate::server::script::skill::metadata::SkillMetadata::find(id as u32).is_none() {
                    return Err("Unknown skill duration skill".into());
                }
            }
            if definition.save.is_some() && flag != MapFlag::NoSave {
                return Err("Save point requires the nosave flag".into());
            }
            entry.set(flag, definition.enabled, &definition.arguments)?;
            if definition.enabled {
                if let Some((map, x, y)) = definition.save {
                    if map.is_empty() {
                        return Err("Map save destination is empty".into());
                    }
                    entry.save = Some((
                        if map.eq_ignore_ascii_case("SavePoint") {
                            "SavePoint".into()
                        } else {
                            crate::server::model::map::Map::name_without_ext(&map).to_ascii_lowercase()
                        },
                        x,
                        y,
                    ));
                }
            }
        }
        Ok(flags)
    }

    /// `test_npcs` places the entries marked `test` too (`scripting.test_npcs`).
    pub fn load_scripts(path: &str, test_npcs: bool) -> Result<HashMap<String, Vec<Script>>, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("Cannot load {path}: {e}"))?;
        let definitions: Vec<NpcDefinition> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        let mut scripts: HashMap<String, Vec<Script>> = HashMap::new();
        let mut names = std::collections::HashSet::new();
        for definition in definitions {
            if definition.name.is_empty() || !names.insert(definition.name.clone()) {
                return Err("NPC manifest contains an invalid or duplicate NPC".into());
            }
            if definition.module.is_empty() || definition.entry.is_empty() {
                return Err(format!("NPC {} needs a module and an entry", definition.name));
            }
            if definition.test && !test_npcs {
                continue;
            }
            let entry_id = intern(&definition.module, Entry::Npc(definition.entry));
            let sprite = definition
                .sprite
                .parse::<u16>()
                .ok()
                .or_else(|| {
                    load_constant(&definition.sprite)
                        .and_then(|v| v.number_value().ok())
                        .and_then(|id| u16::try_from(id).ok())
                })
                .ok_or_else(|| format!("Unknown NPC sprite {}", definition.sprite))?;
            scripts.entry(definition.map_name.clone()).or_default().push(Script {
                id: 0,
                scope_instance: 0,
                map_name: definition.map_name,
                name: definition.name,
                sprite,
                x: definition.x,
                y: definition.y,
                dir: definition.dir,
                x_size: definition.x_size,
                y_size: definition.y_size,
                entry_id,
                constructor_args: definition.constructor_args,
            });
        }
        Ok(scripts)
    }
}

impl ToMapItem for Script {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.sprite as i16, MapItemType::Npc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_or_invalid_map_rule_assets_fail_loading_instead_of_disabling_restrictions() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("rust-ro-map-rules-{}-{nonce}.json", std::process::id()));
        assert!(ScriptLoader::load_map_flags(path.to_str().unwrap()).is_err());
        std::fs::write(
            &path,
            br#"[{"map":"test","flag":"skill_duration","arguments":[2147483647,100]}]"#,
        )
        .unwrap();
        assert!(ScriptLoader::load_map_flags(path.to_str().unwrap()).is_err());
        std::fs::write(&path, br#"[{"map":"test","flag":"noskill","save":["prontera",1,1]}]"#).unwrap();
        assert!(ScriptLoader::load_map_flags(path.to_str().unwrap()).is_err());
        std::fs::write(&path, br#"[{"map":"test","flag":"nosave","save":["PRONTERA.gat",155,181]}]"#).unwrap();
        let flags = ScriptLoader::load_map_flags(path.to_str().unwrap()).unwrap();
        assert_eq!(flags["test"].save, Some(("prontera".into(), 155, 181)));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_npcs_are_placed_only_when_enabled() {
        let path = std::env::temp_dir().join(format!("rust-ro-test-npcs-{}.json", std::process::id()));
        std::fs::write(
            &path,
            br#"[{"map_name":"prontera","name":"Warper#prt","sprite":"111","x":1,"y":1,"dir":0,"module":"systems","entry":"warper","test":true},
                 {"map_name":"prontera","name":"Guard","sprite":"111","x":2,"y":2,"dir":0,"module":"systems","entry":"shop"}]"#,
        )
        .unwrap();
        let names = |test_npcs| {
            ScriptLoader::load_scripts(path.to_str().unwrap(), test_npcs).unwrap()["prontera"].iter().map(|script| script.name.clone()).collect::<Vec<_>>()
        };
        assert_eq!(names(false), vec!["Guard"]);
        assert_eq!(names(true), vec!["Warper#prt", "Guard"]);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn the_shipped_test_npcs_are_the_custom_block() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/wasm/npcs.json");
        let manifest: Vec<serde_json::Value> = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let test: Vec<&serde_json::Value> = manifest.iter().filter(|npc| npc["test"] == true).collect();
        let named = |entry: &str| test.iter().filter(|npc| npc["entry"] == entry).count();
        assert_eq!(named("warper"), manifest.iter().filter(|npc| npc["entry"] == "warper").count());
        for entry in ["job_master", "mount_master", "stylist", "counter", "variables"] {
            assert!(named(entry) > 0, "{entry} is not marked test");
        }
        assert!(test.iter().any(|npc| npc["name"] == "Headgear 05"));
        assert!(test.iter().all(|npc| npc["entry"] != "breeder"));
    }
}
