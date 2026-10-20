//! What plugins add to the game that a client must know to draw it: items, and the skills and jobs they declare. The
//! server publishes it as [`CATALOG_PATH`] next to the client data, listed in `/data/check` like any other file. A client
//! that reads it draws plugin items with their names, descriptions and icons; the stock client knows none of it. See
//! `docs/plugins.md`.

use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

/// Path of the catalog under `/data/`.
pub const CATALOG_PATH: &str = "plugin-catalog.json";

/// Item ids reserved for plugins, clear of the game's own and within the client's 16-bit item ids.
pub const PLUGIN_ITEM_IDS: RangeInclusive<u32> = 40_000..=59_999;
/// Skill ids reserved for plugins.
pub const PLUGIN_SKILL_IDS: RangeInclusive<u32> = 20_000..=20_999;
/// Job ids reserved for plugins.
pub const PLUGIN_JOB_IDS: RangeInclusive<u32> = 4_500..=4_599;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct PluginCatalog {
    pub items: Vec<CatalogItem>,
    pub skills: Vec<CatalogSkill>,
    pub jobs: Vec<CatalogJob>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogItem {
    pub id: u32,
    /// The plugin that defines it. `None` for an item no loaded plugin defines anymore: players keep it, with no script.
    pub plugin: Option<String>,
    /// The item's unique name, as scripts and `@item` spell it.
    pub name: String,
    pub display_name: String,
    /// Lines of its description. `^RRGGBB` starts a colour, `^000000` goes back to black.
    pub description: Vec<String>,
    /// Its type as `config/items.json` spells it: `Healing`, `Usable`, `Etc`, `Armor`, `Weapon`, `Card`...
    pub item_type: String,
    pub slots: u8,
    /// An item of the client's own data whose icon and sprites draw this one.
    pub look: Option<u32>,
    /// A PNG under `/data/` (`plugin/<name>/<file>`), drawn as the inventory icon instead of the look's.
    pub icon: Option<String>,
}

/// How a skill is aimed.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillTarget {
    #[default]
    Passive,
    #[serde(rename = "self")]
    Myself,
    Enemy,
    Ally,
    Ground,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogSkill {
    pub id: u32,
    pub plugin: String,
    /// The skill's unique name, in upper case like `SM_BASH`.
    pub name: String,
    pub display_name: String,
    pub description: Vec<String>,
    pub max_level: u8,
    pub target: SkillTarget,
    /// SP cost per level, from level 1.
    pub sp_cost: Vec<u16>,
    /// Cells from the caster it reaches; 0 for passive and self skills.
    pub range: u8,
    /// A skill of the client's own data whose icon this one shows.
    pub look: Option<u32>,
    /// A PNG under `/data/` (`plugin/<name>/<file>`), drawn as the icon instead of the look's.
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogJob {
    pub id: u32,
    pub plugin: String,
    pub name: String,
    pub display_name: String,
    /// The game job it builds on: its sprites, stats and equipment. Its id as the client knows it.
    pub base_job: u32,
    /// The skill tree, on top of the base job's.
    pub skills: Vec<CatalogTreeSkill>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct CatalogTreeSkill {
    pub skill: u32,
    pub max_level: u8,
    /// Skills to learn first, with their level.
    pub requires: Vec<SkillRequirement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SkillRequirement {
    pub skill: u32,
    pub level: u8,
}
