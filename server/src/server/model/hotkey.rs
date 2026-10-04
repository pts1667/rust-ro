#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Hotkey {
    pub index: i16,
    pub is_skill: i16,
    pub itemskill_id: i32,
    pub skill_lvl: i16,
}
