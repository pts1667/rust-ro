use configuration::serde_helper::*;
use models::enums::EnumWithNumberValue;
use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::item::ItemType;
use models::enums::weapon::{AmmoType, WeaponType};
use models::item::{NormalInventoryItem, WearAmmo, WearGear, WearWeapon};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemModels {
    pub items: Vec<ItemModel>,
}
impl From<Vec<ItemModel>> for ItemModels {
    fn from(items: Vec<ItemModel>) -> Self {
        ItemModels { items }
    }
}
impl From<ItemModels> for Vec<ItemModel> {
    fn from(item_models: ItemModels) -> Self {
        item_models.items
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemModel {
    pub id: i32,
    pub name_aegis: String,
    pub name_english: String,
    #[serde(serialize_with = "serialize_string_enum", deserialize_with = "deserialize_string_enum")]
    pub item_type: ItemType,
    #[serde(
        serialize_with = "serialize_optional_string_enum",
        deserialize_with = "deserialize_optional_string_enum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub weapon_type: Option<WeaponType>,
    #[serde(
        serialize_with = "serialize_optional_string_enum",
        deserialize_with = "deserialize_optional_string_enum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ammo_type: Option<AmmoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_buy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_sell: Option<i32>,
    pub weight: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defense: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<i16>,
    pub job_flags: u64,
    pub class_flags: u64,
    pub location: u64,
    #[serde(
        serialize_with = "serialize_optional_string_enum",
        deserialize_with = "deserialize_optional_string_enum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub element: Option<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_level: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_level: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equip_level_min: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equip_level_max: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refineable: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    pub flags: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_amount: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_inventory: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_cart: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_storage: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_guildstorage: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nouse_override: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nouse_sitting: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_override: Option<i32>,
    pub trade_flags: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(skip)]
    pub bonuses: Vec<BonusType>,
    #[serde(skip)]
    pub item_bonuses_are_dynamic: bool,
}

impl ItemModel {
    fn attack_element(&self) -> Element {
        self.bonuses
            .iter()
            .rev()
            .find_map(|bonus| match bonus {
                BonusType::ElementWeapon(element) => Some(*element),
                _ => None,
            })
            .or(self.element)
            .unwrap_or(Element::Neutral)
    }

    pub fn to_wear_weapon(&self, inventory_index: usize, location: u64, inventory_model: &InventoryItemModel) -> WearWeapon {
        let element = if inventory_model.card0 == 255 && self.attack_element() == Element::Neutral {
            let value = inventory_model.card1 as u16 % 16;
            if value <= 9 {
                Element::from_value(value as usize)
            } else {
                Element::Neutral
            }
        } else {
            self.attack_element()
        };
        WearWeapon {
            item_id: self.id,
            attack: self.attack.unwrap_or(0) as u32,
            level: self.weapon_level.unwrap_or(0) as u8,
            weapon_type: self.weapon_type.unwrap_or(WeaponType::Fist),
            location,
            element,
            refine: inventory_model.refine as u8,
            card0: inventory_model.card0,
            card1: inventory_model.card1,
            card2: inventory_model.card2,
            card3: inventory_model.card3,
            ranked_forged: false,
            inventory_index,
            range: self.range.unwrap_or(1) as u8,
        }
    }

    pub fn to_wear_gear(&self, inventory_index: usize, location: u64, inventory_model: &InventoryItemModel) -> WearGear {
        WearGear {
            item_id: self.id,
            level: self.armor_level.unwrap_or(0) as u8,
            location,
            refine: inventory_model.refine as u8,
            card0: inventory_model.card0,
            card1: inventory_model.card1,
            card2: inventory_model.card2,
            card3: inventory_model.card3,
            def: self.defense.unwrap_or(0),
            inventory_index,
        }
    }

    pub fn to_wear_ammo(&self, inventory_index: usize) -> WearAmmo {
        WearAmmo {
            item_id: self.id,
            inventory_index,
            element: self.attack_element(),
            attack: self.attack.unwrap_or(0) as u8,
            ammo_type: self.ammo_type.unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ItemBuySellModel {
    pub id: Option<i32>,
    pub item_type: String,
    pub price_buy: Option<i32>,
    pub price_sell: Option<i32>,
    pub stack_amount: Option<i16>,
    pub weight: Option<i32>,
    pub name_english: Option<String>,
}

#[derive(Debug)]
pub struct GetItemModel {
    pub id: i32,
    pub item_type: String,
    pub amount: i16,
    pub weight: i32,
    pub name_english: String,
    pub name_aegis: String,
}

#[allow(dead_code)]
struct DBWeaponType {
    weapon_type: WeaponType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DBItemType {
    item_type: ItemType,
}

impl DBItemType {
    // used by tests
    #[allow(dead_code)]
    pub(crate) fn from_type(item_type: ItemType) -> Self {
        Self { item_type }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InventoryItemModel {
    pub shop_price: Option<i32>,
    // Come from inventory table
    pub id: i32,
    pub unique_id: i64,
    pub item_id: i32,
    pub(crate) item_type: DBItemType,
    pub amount: i16,
    pub refine: i16,
    pub is_identified: bool,
    pub equip: i32,
    pub is_damaged: bool,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
    // Come from itemdb table
    pub name_english: String,
    pub weight: i32,
}

impl InventoryItemModel {
    pub fn from_record(record: &database::model::InventoryRecord, item: &ItemModel) -> Self {
        let mut model = Self::from_item_model(item, record.amount, record.is_identified);
        model.id = record.id;
        model.unique_id = record.unique_id;
        model.refine = record.refine;
        model.equip = record.equip;
        model.is_damaged = record.is_damaged;
        model.card0 = record.card0;
        model.card1 = record.card1;
        model.card2 = record.card2;
        model.card3 = record.card3;
        model
    }

    pub fn from_item_model(item: &ItemModel, amount: i16, is_identified: bool) -> Self {
        Self {
            shop_price: None,
            id: 0,
            unique_id: 0,
            item_id: item.id,
            item_type: DBItemType { item_type: item.item_type },
            amount,
            refine: 0,
            is_identified,
            equip: 0,
            is_damaged: false,
            card0: 0,
            card1: 0,
            card2: 0,
            card3: 0,
            name_english: item.name_english.clone(),
            weight: item.weight,
        }
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type.item_type
    }

    pub fn to_normal_item(&self) -> NormalInventoryItem {
        NormalInventoryItem {
            item_id: self.item_id,
            amount: self.amount,
            name_english: self.name_english.to_string(),
        }
    }

    pub fn has_free_slot(&self, item: &ItemModel) -> bool {
        let slots = item.slots.unwrap_or(0_i16);
        (slots == 1 && self.card0 == 0)
            || (slots == 2 && (self.card0 == 0 || self.card1 == 0))
            || (slots == 3 && (self.card0 == 0 || self.card1 == 0 || self.card2 == 0))
            || (slots == 4 && (self.card0 == 0 || self.card1 == 0 || self.card2 == 0 || self.card3 == 0))
    }

    pub fn set_card_at(&mut self, index: usize, card_id: i16) {
        if index == 0 {
            self.card0 = card_id;
        } else if index == 1 {
            self.card1 = card_id;
        } else if index == 2 {
            self.card2 = card_id;
        } else if index == 3 {
            self.card3 = card_id;
        }
    }
}
