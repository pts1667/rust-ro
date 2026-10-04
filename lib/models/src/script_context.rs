use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct ScriptCharacterState {
    pub char_id: u32,
    pub account_id: u32,
    pub party_id: u32,
    pub guild_id: u32,
    pub name: String,
    pub map: String,
    pub party_name: String,
    pub guild_name: String,
    pub partner_id: u32,
    pub vip_expires_at: u64,
    pub options: u64,
    pub mounting: bool,
    pub inventory: HashMap<i32, i32>,
    pub weight: u32,
    pub karma: i32,
    pub manner: i32,
    pub pet: Option<ScriptPetState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptPetState {
    pub id: u32,
    pub class_id: u16,
    pub name: String,
    pub intimacy: i32,
    pub hunger: i32,
    pub renamed: bool,
    pub level: u16,
    pub actor_id: u32,
    pub egg_id: i32,
    pub food_id: i32,
    pub equipped_item: i32,
    pub support_bonuses: Vec<crate::enums::bonus::BonusType>,
}
