use std::collections::HashMap;

use script_sdk::Value;
use serde::Deserialize;

use crate::server::model::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::model::script::Script;
use crate::server::script::constant::load_constant;

pub struct ScriptLoader;

#[derive(Deserialize)]
struct NpcDefinition {
    map_name: String,
    name: String,
    sprite: String,
    x: u16,
    y: u16,
    dir: u16,
    entry_id: u32,
    #[serde(default)]
    constructor_args: Vec<Value>,
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

    pub fn load_scripts(path: &str) -> Result<HashMap<String, Vec<Script>>, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("Cannot load {path}: {e}"))?;
        let definitions: Vec<NpcDefinition> = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        let mut scripts: HashMap<String, Vec<Script>> = HashMap::new();
        let mut names = std::collections::HashSet::new();
        for definition in definitions {
            if definition.name.is_empty() || !names.insert(definition.name.clone()) || definition.entry_id == 0 {
                return Err("NPC manifest contains an invalid or duplicate NPC".into());
            }
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
                map_name: definition.map_name,
                name: definition.name,
                sprite,
                x: definition.x,
                y: definition.y,
                dir: definition.dir,
                x_size: 0,
                y_size: 0,
                entry_id: definition.entry_id,
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
}
