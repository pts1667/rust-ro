use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub const DEFAULT_ACCOUNT_EMAIL: &str = "a@a.com";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AccountRecord {
    pub account_id: u32,
    #[serde(alias = "userid")]
    pub username: String,
    #[serde(alias = "user_pass")]
    pub password: String,
    /// `M`, `F` or `S` (server account).
    pub sex: String,
    pub group_id: u32,
    pub state: u32,
    pub unban_time: i64,
    pub expiration_time: i64,
    pub email: String,
    /// `YYYY-MM-DD`, empty when unknown.
    pub birthdate: String,
    pub last_ip: String,
    pub last_login: i64,
    pub login_count: u32,
    /// Character slots granted to the account; `0` means the configured default.
    pub char_slots: u8,
    pub pincode: String,
    pub pincode_change: i64,
}

impl Default for AccountRecord {
    fn default() -> Self {
        Self {
            account_id: 0,
            username: String::new(),
            password: String::new(),
            sex: "M".into(),
            group_id: 0,
            state: 0,
            unban_time: 0,
            expiration_time: 0,
            email: DEFAULT_ACCOUNT_EMAIL.into(),
            birthdate: String::new(),
            last_ip: String::new(),
            last_login: 0,
            login_count: 0,
            char_slots: 0,
            pincode: String::new(),
            pincode_change: 0,
        }
    }
}

impl AccountRecord {
    pub fn new(account_id: u32, username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            account_id,
            username: username.into(),
            password: password.into(),
            ..Self::default()
        }
    }
}

/// An active or historical IP ban. `list` is an IPv4 pattern such as `1.2.3.4`, `1.2.3.*`, `1.2.*.*` or `1.*.*.*`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IpBanRecord {
    pub list: String,
    pub begin: i64,
    pub release: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharLogRecord {
    pub time: i64,
    pub account_id: u32,
    pub char_slot: i16,
    pub name: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginLogRecord {
    pub time: i64,
    pub ip: String,
    pub username: String,
    pub result_code: i32,
    pub message: String,
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
    pub position_revision: u64,
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
    /// Unix time until which the character cannot be selected, `0` when not banned.
    pub unban_time: i64,
    /// Remaining slot moves granted to the character.
    pub moves: u32,
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
