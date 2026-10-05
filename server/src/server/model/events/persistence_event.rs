use std::fmt::Debug;

use models::enums::skill_enums::SkillEnum;

use crate::repository::model::item_model::InventoryItemModel;

#[derive(Debug, PartialEq)]
pub enum PersistenceEvent {
    SaveCharacterPosition(SavePositionUpdate),
    UpdateCharacterStatusU32(StatusUpdate<u32>),
    DeleteItemsFromInventory(DeleteItems),
    UpdateEquippedItems(Vec<InventoryItemModel>),
    ResetSkills(ResetSkills),
    IncreaseSkillLevel(IncreaseSkillLevel),
    Shutdown,
}
#[derive(Debug, PartialEq)]
pub struct SavePositionUpdate {
    pub char_id: u32,
    pub account_id: u32,
    pub map_name: String,
    pub x: u16,
    pub y: u16,
    pub revision: u64,
}
#[derive(Debug, PartialEq)]
pub struct StatusUpdate<T: Debug + Sized + PartialEq> {
    pub char_id: u32,
    pub(crate) field: String,
    pub(crate) value: T,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct InventoryItemUpdate {
    pub item_id: i32,
    pub char_id: i32,
    pub unique_id: i64,
    pub amount: i16,
    pub identified: bool,
    pub stackable: bool,
    pub refine: i16,
    pub damaged: bool,
    pub cards: [i16; 4],
    pub price: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct DeleteItems {
    pub char_id: i32,
    pub item_inventory_id: i32,
    pub unique_id: i64,
    pub amount_to_remove: i16,
}

#[derive(Debug, PartialEq)]
pub struct ResetSkills {
    pub char_id: i32,
    pub skills: Vec<i32>,
}
#[derive(Debug, PartialEq)]
pub struct IncreaseSkillLevel {
    pub char_id: i32,
    pub skill: SkillEnum,
    pub increment: u8,
}
