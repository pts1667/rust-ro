use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRecord {
    pub account_id: u32,
    #[serde(alias = "userid")]
    pub username: String,
    #[serde(alias = "user_pass")]
    pub password: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CharacterRecord {
    pub char_id: i32,
    pub account_id: i32,
    pub char_num: i16,
    pub name: String,
    pub class: i16,
    pub zeny: i32,
    pub status_point: i16,
    pub skill_point: i16,
    pub str: i16,
    pub agi: i16,
    pub vit: i16,
    pub int: i16,
    pub dex: i16,
    pub luk: i16,
    pub max_hp: i32,
    pub hp: i32,
    pub max_sp: i32,
    pub sp: i32,
    pub hair: i16,
    pub hair_color: i16,
    pub last_map: String,
    pub last_x: i16,
    pub last_y: i16,
    pub save_map: String,
    pub save_x: i16,
    pub save_y: i16,
    pub sex: String,
    pub inventory_slots: i16,
    pub clothes_color: i16,
    pub body: i16,
    pub weapon: i16,
    pub shield: i16,
    pub head_top: i16,
    pub head_mid: i16,
    pub head_bottom: i16,
    pub robe: i32,
    pub base_level: i32,
    pub job_level: i32,
    pub base_exp: i32,
    pub job_exp: i32,
    pub option: i32,
    pub karma: i32,
    pub manner: i32,
    pub rename: i16,
    pub delete_date: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct InventoryRecord {
    pub id: i32,
    pub unique_id: i64,
    #[serde(alias = "nameid")]
    pub item_id: i32,
    pub amount: i16,
    pub refine: i16,
    #[serde(alias = "identified")]
    pub is_identified: bool,
    pub equip: i32,
    #[serde(alias = "damaged")]
    pub is_damaged: bool,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterInventory {
    pub char_id: i32,
    pub items: Vec<InventoryRecord>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterSkills {
    pub char_id: i32,
    pub skills: BTreeMap<u32, u8>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SeedData {
    pub accounts: Vec<AccountRecord>,
    pub characters: Vec<CharacterRecord>,
    pub inventories: Vec<CharacterInventory>,
    pub skills: Vec<CharacterSkills>,
}
