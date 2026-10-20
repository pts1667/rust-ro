//! Items of plugins (`docs/plugins.md`). A plugin's `items.json` lists items with ids in [`PLUGIN_ITEM_IDS`], written with the
//! fields of `config/items.json` plus what a client needs to draw them. At boot they join the game's items, and the
//! database keeps their definition: a player still holding one after its plugin is gone keeps a plain item, and the
//! server never meets an id it does not know.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use models::enums::EnumWithStringValue;
use models::enums::item::ItemType;
use plugin_ui::catalog::{CatalogItem, PLUGIN_ITEM_IDS};
use script_sdk::PluginInfo;
use serde_json::{Map, Value};

use crate::repository::model::item_model::ItemModel;

/// The fields of `config/items.json` a plugin item may set. `script` is not one: plugin items run Rust, see `items { }`.
const ITEM_FIELDS: [&str; 37] = [
    "id", "name_aegis", "name_english", "item_type", "weapon_type", "ammo_type", "price_buy", "price_sell", "weight", "attack",
    "defense", "range", "slots", "job_flags", "class_flags", "location", "element", "gender", "weapon_level", "armor_level",
    "equip_level_min", "equip_level_max", "refineable", "view", "alias_name", "flags", "delay_duration", "delay_status",
    "stack_amount", "stack_inventory", "stack_cart", "stack_storage", "stack_guildstorage", "nouse_override", "nouse_sitting",
    "trade_override", "trade_flags",
];
/// What only plugin items have: how clients draw them and how their scripts run.
const PLUGIN_FIELDS: [&str; 5] = ["description", "look", "icon", "dialogue", "dynamic_bonuses"];
const DEFAULTS: [(&str, u64); 6] = [("weight", 0), ("job_flags", u64::MAX), ("class_flags", 0), ("location", 0), ("flags", 0), ("trade_flags", 0)];

#[derive(Debug, Clone)]
pub struct PluginItem {
    pub model: ItemModel,
    pub description: Vec<String>,
    pub look: Option<u32>,
    /// The icon's file in the plugin's `ui/` folder.
    pub icon: Option<String>,
    /// Its use script talks to the player, so it runs like an NPC conversation.
    pub dialogue: bool,
    /// Its bonus script reads the wearer (level, refine, other gear) and runs again whenever the status is computed,
    /// instead of once at boot.
    pub dynamic_bonuses: bool,
    pub has_use_script: bool,
    pub has_bonus_script: bool,
}

impl PluginItem {
    pub fn id(&self) -> u32 {
        self.model.id as u32
    }

    pub fn catalog_entry(&self, plugin: &str) -> CatalogItem {
        CatalogItem {
            id: self.id(),
            plugin: Some(plugin.to_string()),
            name: self.model.name_aegis.clone(),
            display_name: self.model.name_english.clone(),
            description: self.description.clone(),
            item_type: self.model.item_type.as_str().to_string(),
            slots: self.model.slots.unwrap_or(0).clamp(0, 4) as u8,
            look: self.look,
            icon: self.icon.as_ref().map(|file| plugin_ui::plugin_asset_path(plugin, file)),
        }
    }
}

/// Uses items run a script when used; worn items and cards may describe bonuses.
fn takes_use_script(kind: ItemType) -> bool {
    matches!(kind, ItemType::Healing | ItemType::Usable | ItemType::DelayConsume | ItemType::Cash)
}

fn takes_bonus_script(kind: ItemType) -> bool {
    kind.is_wearable() || kind.is_card()
}

/// Reads and checks `<folder>/items.json` against what the plugin's module declares.
pub fn load(folder: &Path, info: &PluginInfo) -> Result<Vec<PluginItem>, String> {
    let path = folder.join("items.json");
    let entries: Vec<Value> = match std::fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|error| format!("{} is invalid: {error}", path.display()))?,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => return Err(format!("cannot read {}: {error}", path.display())),
    };
    let mut items = Vec::with_capacity(entries.len());
    let (mut ids, mut names) = (HashSet::new(), HashSet::new());
    for (index, entry) in entries.into_iter().enumerate() {
        let item = parse(entry, folder, info).map_err(|error| format!("items.json entry {index}: {error}"))?;
        if !ids.insert(item.id()) {
            return Err(format!("item {} is defined twice", item.id()));
        }
        if !names.insert(item.model.name_aegis.to_lowercase()) {
            return Err(format!("item name {} is used twice", item.model.name_aegis));
        }
        items.push(item);
    }
    for id in info.item_scripts.iter().chain(&info.item_bonuses) {
        if !ids.contains(id) {
            return Err(format!("the module has a script for item {id}, which its items.json does not define"));
        }
    }
    Ok(items)
}

fn parse(entry: Value, folder: &Path, info: &PluginInfo) -> Result<PluginItem, String> {
    let Value::Object(mut fields) = entry else { return Err("an item is a JSON object".into()) };
    if let Some(unknown) = fields.keys().find(|key| !ITEM_FIELDS.contains(&key.as_str()) && !PLUGIN_FIELDS.contains(&key.as_str())) {
        return Err(if unknown == "script" {
            "plugin items take no rathena script: give them a handler in the module's `items` or `item_bonuses`".to_string()
        } else {
            format!("{unknown} is not an item field")
        });
    }
    let description = description(fields.remove("description"))?;
    let look = optional(&mut fields, "look", Value::as_u64)?.map(|look| u32::try_from(look).map_err(|_| "look is not an item id")).transpose()?;
    let icon = optional(&mut fields, "icon", |value| value.as_str().map(String::from))?;
    let dialogue = optional(&mut fields, "dialogue", Value::as_bool)?.unwrap_or(false);
    let dynamic_bonuses = optional(&mut fields, "dynamic_bonuses", Value::as_bool)?.unwrap_or(false);
    for (key, default) in DEFAULTS {
        fields.entry(key).or_insert(Value::from(default));
    }
    let model: ItemModel = serde_json::from_value(Value::Object(fields)).map_err(|error| error.to_string())?;
    let id = u32::try_from(model.id).ok().filter(|id| PLUGIN_ITEM_IDS.contains(id)).ok_or_else(|| {
        format!("item id {} is outside the ids of plugin items, {}..={}", model.id, PLUGIN_ITEM_IDS.start(), PLUGIN_ITEM_IDS.end())
    })?;
    let name = &model.name_aegis;
    if name.is_empty() || name.len() > 50 || !name.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_') {
        return Err(format!("item {id}: name_aegis {name:?} must be 1 to 50 letters, digits or _"));
    }
    if model.name_english.trim().is_empty() {
        return Err(format!("item {id} has no name_english"));
    }
    if model.item_type.is_wearable() && model.location == 0 {
        return Err(format!("item {id} is worn but has no location"));
    }
    let has_use_script = info.item_scripts.contains(&id);
    let has_bonus_script = info.item_bonuses.contains(&id);
    let kind = model.item_type;
    if takes_use_script(kind) != has_use_script {
        return Err(if has_use_script {
            format!("item {id} is {}, which is not used, yet the module has a use script for it", kind.as_str())
        } else {
            format!("item {id} is {}, so the module needs a use script for it in `items`", kind.as_str())
        });
    }
    if has_bonus_script && !takes_bonus_script(kind) {
        return Err(format!("item {id} is {}, which is neither worn nor a card, yet the module has a bonus script for it", kind.as_str()));
    }
    if dialogue && !has_use_script {
        return Err(format!("item {id} has a dialogue but no use script"));
    }
    if dynamic_bonuses && !has_bonus_script {
        return Err(format!("item {id} has dynamic bonuses but no bonus script"));
    }
    if let Some(icon) = &icon {
        check_icon(folder, icon).map_err(|error| format!("item {id}: {error}"))?;
    }
    Ok(PluginItem { model, description, look, icon, dialogue, dynamic_bonuses, has_use_script, has_bonus_script })
}

fn optional<T>(fields: &mut Map<String, Value>, key: &str, read: impl Fn(&Value) -> Option<T>) -> Result<Option<T>, String> {
    match fields.remove(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => read(&value).map(Some).ok_or_else(|| format!("{key} has the wrong type: {value}")),
    }
}

/// A text with line breaks, or its lines.
fn description(value: Option<Value>) -> Result<Vec<String>, String> {
    match value {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::String(text)) => Ok(text.lines().map(String::from).collect()),
        Some(Value::Array(lines)) => lines.into_iter().map(|line| line.as_str().map(String::from).ok_or_else(|| "description lines are texts".to_string())).collect(),
        Some(_) => Err("description is a text or a list of lines".into()),
    }
}

/// An icon is a lower case PNG of the plugin's `ui/` folder, which the client data port serves.
pub(super) fn check_icon(folder: &Path, icon: &str) -> Result<(), String> {
    let valid = icon.ends_with(".png")
        && !icon.contains("..")
        && !icon.starts_with(['/', '\\'])
        && icon.chars().all(|character| !character.is_ascii_uppercase());
    if !valid {
        return Err(format!("icon {icon} must be a lower case .png path inside the plugin's ui/ folder"));
    }
    if !folder.join("ui").join(icon).is_file() {
        return Err(format!("icon {icon} is not a file of {}", folder.join("ui").display()));
    }
    Ok(())
}

/// What [`install`] did to the game's items.
#[derive(Debug, Default)]
pub struct Installed {
    /// The catalog entries of the plugin items, those no plugin defines anymore included.
    pub catalog: Vec<CatalogItem>,
    /// The definitions the database must keep, so the items outlive their plugin.
    pub to_save: Vec<ItemModel>,
}

/// Merges the plugin items into `items`, the game's items as the database holds them, whose plugin ids are the items
/// of an earlier boot. A definition replaces the one of the same id; an item no plugin defines anymore stays, without
/// scripts. `plugins` are the plugin names with their items, in load order.
pub fn install(plugins: &[(&str, &[PluginItem])], items: &mut Vec<ItemModel>) -> Result<Installed, String> {
    let defined: HashMap<u32, (&str, &PluginItem)> = plugins.iter().flat_map(|(plugin, items)| items.iter().map(|item| (item.id(), (*plugin, item)))).collect();
    let mut owners: HashMap<u32, &str> = HashMap::new();
    for (plugin, plugin_items) in plugins {
        for item in plugin_items.iter() {
            if let Some(other) = owners.insert(item.id(), plugin) {
                return Err(format!("Plugins {other} and {plugin} both define item {}", item.id()));
            }
        }
    }
    let is_plugin_id = |id: i32| u32::try_from(id).is_ok_and(|id| PLUGIN_ITEM_IDS.contains(&id));
    let mut names: HashMap<String, (i32, Option<&str>)> =
        items.iter().filter(|item| !is_plugin_id(item.id)).map(|item| (item.name_aegis.to_lowercase(), (item.id, None))).collect();
    for (plugin, item) in plugins.iter().flat_map(|(plugin, items)| items.iter().map(move |item| (*plugin, item))) {
        if let Some((other_id, other_plugin)) = names.insert(item.model.name_aegis.to_lowercase(), (item.model.id, Some(plugin))) {
            let owner = other_plugin.map_or("the game".to_string(), |other| format!("plugin {other}"));
            return Err(format!("Plugin {plugin} is refused: item name {} is already item {other_id} of {owner}", item.model.name_aegis));
        }
    }
    let mut installed = Installed::default();
    let mut retired = Vec::new();
    items.retain_mut(|item| {
        if !is_plugin_id(item.id) {
            return true;
        }
        if defined.contains_key(&(item.id as u32)) {
            return false;
        }
        if names.contains_key(&item.name_aegis.to_lowercase()) {
            item.name_aegis = format!("{}_{}", item.name_aegis, item.id);
            installed.to_save.push(item.clone());
        }
        retired.push(item.id);
        installed.catalog.push(CatalogItem {
            id: item.id as u32,
            plugin: None,
            name: item.name_aegis.clone(),
            display_name: item.name_english.clone(),
            description: vec!["No loaded plugin defines this item anymore.".into()],
            item_type: item.item_type.as_str().to_string(),
            slots: item.slots.unwrap_or(0).clamp(0, 4) as u8,
            look: None,
            icon: None,
        });
        true
    });
    if !retired.is_empty() {
        warn!("Items {retired:?} were defined by plugins that are not loaded: players keep them as plain items until those plugins are back");
    }
    let mut defined: Vec<_> = defined.into_values().collect();
    defined.sort_by_key(|(_, item)| item.id());
    for (plugin, item) in defined {
        if let Some(look) = item.look {
            if !items.iter().any(|known| known.id as u32 == look && !is_plugin_id(known.id)) {
                return Err(format!("Plugin {plugin} is refused: item {} looks like item {look}, which is not an item of the game", item.id()));
            }
        }
        items.push(item.model.clone());
        installed.to_save.push(item.model.clone());
        installed.catalog.push(item.catalog_entry(plugin));
    }
    installed.catalog.sort_by_key(|entry| entry.id);
    Ok(installed)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn lucky_charms() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../config/plugins/lucky_charms")
    }

    fn info(item_scripts: &[u32], item_bonuses: &[u32]) -> PluginInfo {
        serde_json::from_value(json!({
            "name": "test", "version": "1.0.0", "sdk_abi": 1, "manifest": { "title": "Test", "description": "" },
            "item_scripts": item_scripts, "item_bonuses": item_bonuses,
        }))
        .unwrap()
    }

    fn parsed(entry: Value, item_scripts: &[u32], item_bonuses: &[u32]) -> Result<PluginItem, String> {
        parse(entry, &lucky_charms(), &info(item_scripts, item_bonuses))
    }

    fn game_item(id: i32, name: &str) -> ItemModel {
        serde_json::from_value(json!({
            "id": id, "name_aegis": name, "name_english": name, "item_type": "Etc", "weight": 1, "job_flags": 0, "class_flags": 0,
            "location": 0, "flags": 0, "trade_flags": 0,
        }))
        .unwrap()
    }

    fn plugin_item(id: u32, name: &str, look: Option<u32>) -> PluginItem {
        let mut item = parsed(json!({ "id": id, "name_aegis": name, "name_english": name, "item_type": "Etc" }), &[], &[]).unwrap();
        item.look = look;
        item
    }

    #[test]
    fn the_example_items_load_with_defaults_for_what_they_leave_out() {
        let info = info(&[40001, 40003], &[40002]);
        let items = load(&lucky_charms(), &info).unwrap();
        assert_eq!(items.iter().map(PluginItem::id).collect::<Vec<_>>(), [40001, 40002, 40003]);
        let ticket = &items[0];
        assert_eq!((ticket.model.job_flags, ticket.model.location), (u64::MAX, 0));
        assert!(ticket.has_use_script && !ticket.has_bonus_script);
        let entry = ticket.catalog_entry("lucky_charms");
        assert_eq!(entry.icon.as_deref(), Some(plugin_ui::plugin_asset_path("lucky_charms", "ticket.png").as_str()));
        assert_eq!((entry.look, entry.description.len()), (Some(6153), 2));
        assert!(items[2].dialogue);
        assert_eq!(items[1].catalog_entry("lucky_charms").slots, 1);
    }

    #[test]
    fn a_script_the_items_file_does_not_back_is_refused() {
        let error = load(&lucky_charms(), &info(&[40001, 40003, 40009], &[40002])).unwrap_err();
        assert!(error.contains("item 40009"), "{error}");
    }

    #[test]
    fn items_that_do_not_fit_their_scripts_are_refused() {
        let base = |extra: Value| {
            let mut entry = json!({ "id": 40100, "name_aegis": "Test_Item", "name_english": "Test Item", "item_type": "Usable" });
            entry.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
            entry
        };
        assert!(parsed(base(json!({})), &[40100], &[]).is_ok());
        for (entry, scripts, bonuses, expected) in [
            (base(json!({ "script": "itemheal 1,0;" })), &[40100][..], &[][..], "no rathena script"),
            (base(json!({ "id": 501 })), &[][..], &[][..], "outside the ids"),
            (base(json!({ "name_aegis": "Bad Name" })), &[40100][..], &[][..], "name_aegis"),
            (base(json!({})), &[][..], &[][..], "needs a use script"),
            (base(json!({ "item_type": "Etc" })), &[40100][..], &[][..], "not used"),
            (base(json!({ "item_type": "Armor" })), &[][..], &[][..], "no location"),
            (base(json!({})), &[40100][..], &[40100][..], "neither worn nor a card"),
            (base(json!({ "item_type": "Etc", "dialogue": true })), &[][..], &[][..], "dialogue but no use script"),
            (base(json!({ "item_type": "Card", "dynamic_bonuses": true })), &[][..], &[][..], "dynamic bonuses"),
            (base(json!({ "icon": "missing.png" })), &[40100][..], &[][..], "not a file"),
            (base(json!({ "icon": "../plugin.wasm" })), &[40100][..], &[][..], "inside the plugin's ui/"),
            (base(json!({ "colour": 1 })), &[40100][..], &[][..], "not an item field"),
        ] {
            let error = parsed(entry.clone(), scripts, bonuses).unwrap_err();
            assert!(error.contains(expected), "{entry}: {error}");
        }
    }

    #[test]
    fn plugin_items_replace_their_earlier_definition_and_retired_ones_stay() {
        let mut items = vec![game_item(501, "Red_Potion"), game_item(40001, "Old_Ticket"), game_item(40050, "Gone"), game_item(40051, "Shiny")];
        let ticket = plugin_item(40001, "Ticket", Some(501));
        let shiny = plugin_item(40060, "Shiny", None);
        let installed = install(&[("a", &[ticket]), ("b", &[shiny])], &mut items).unwrap();
        let mut ids: Vec<_> = items.iter().map(|item| (item.id, item.name_aegis.as_str())).collect();
        ids.sort();
        assert_eq!(ids, [(501, "Red_Potion"), (40001, "Ticket"), (40050, "Gone"), (40051, "Shiny_40051"), (40060, "Shiny")]);
        let catalog: Vec<_> = installed.catalog.iter().map(|entry| (entry.id, entry.plugin.as_deref())).collect();
        assert_eq!(catalog, [(40001, Some("a")), (40050, None), (40051, None), (40060, Some("b"))]);
        let saved: Vec<_> = installed.to_save.iter().map(|item| item.id).collect();
        assert_eq!(saved, [40051, 40001, 40060], "the renamed retired item and the definitions");
    }

    #[test]
    fn clashes_between_plugins_and_with_the_game_are_refused() {
        let items = || vec![game_item(501, "Red_Potion")];
        let refused = |plugins: &[(&str, &[PluginItem])]| install(plugins, &mut items()).unwrap_err();
        let one = [plugin_item(40001, "One", None)];
        assert!(refused(&[("a", &one), ("b", &one)]).contains("both define item 40001"));
        assert!(refused(&[("a", &[plugin_item(40001, "red_potion", None)])]).contains("item 501 of the game"));
        assert!(refused(&[("a", &one), ("b", &[plugin_item(40002, "ONE", None)])]).contains("is already item 40001 of plugin a"));
        assert!(refused(&[("a", &[plugin_item(40001, "One", Some(40002))])]).contains("not an item of the game"));
    }
}
