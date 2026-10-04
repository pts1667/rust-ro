use accessor::GettersAll;
use movement::position::Position;

use crate::enums::EnumWithMaskValueU64;
use crate::enums::element::Element;
use crate::enums::item::EquipmentLocation;
use crate::enums::weapon::{AmmoType, WeaponType};

pub fn special_card_metadata(card0: i16) -> bool { matches!(card0, 254..=256) }

fn star_damage(card0: i16, card1: i16) -> u16 {
    if card0 != 255 { return 0; }
    let stars = card1 as u16 / 256;
    if stars >= 15 { 40 } else { stars }
}

pub struct EquippedItem {
    pub item_id: i32,
    pub location: u64,
    pub index: usize,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct DroppedItem {
    pub map_item_id: u32,
    pub item_id: i32,
    pub location: Position,
    pub sub_location: Position,
    pub owner_id: Option<u32>,
    pub dropped_at: u128,
    pub amount: u16,
    pub is_identified: bool,
    pub attributes: ItemInstanceAttributes,
    pub player_dropped: bool,
}

#[derive(Clone, Debug, Copy, Default, PartialEq, Eq)]
pub struct ItemInstanceAttributes {
    pub unique_id: i64,
    pub refine: i16,
    pub damaged: bool,
    pub cards: [i16; 4],
}

impl DroppedItem {
    pub fn item_id(&self) -> i32 {
        self.item_id
    }

    pub fn x(&self) -> u16 {
        self.location.x
    }

    pub fn y(&self) -> u16 {
        self.location.y
    }
}

#[derive(Debug, Clone)]
pub struct NormalInventoryItem {
    pub item_id: i32,
    pub amount: i16,
    pub name_english: String,
}

#[derive(Debug, Clone, Copy, GettersAll)]
pub struct WearWeapon {
    pub item_id: i32,
    pub attack: u32,
    pub level: u8,
    pub weapon_type: WeaponType,
    pub location: u64,
    pub refine: u8,
    pub element: Element,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
    pub ranked_forged: bool,
    pub inventory_index: usize,
    pub range: u8,
}

impl WearWeapon {
    pub fn forged_star_damage(&self) -> u16 { star_damage(self.card0, self.card1) + if self.card0 == 255 && self.ranked_forged { 10 } else { 0 } }

    pub fn creator_id(&self) -> Option<u32> {
        matches!(self.card0, 254 | 255).then(|| self.card2 as u16 as u32 | ((self.card3 as u16 as u32) << 16))
    }
    pub fn to_snapshot(&self) -> WearWeaponSnapshot {
        WearWeaponSnapshot {
            item_id: self.item_id,
            attack: self.attack,
            level: self.level,
            weapon_type: self.weapon_type,
            element: self.element,
            range: self.range,
            refine: self.refine,
            card0: self.card0,
            card1: self.card1,
            card2: self.card2,
            card3: self.card3,
            ranked_forged: self.ranked_forged,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll)]
pub struct WearWeaponSnapshot {
    item_id: i32,
    attack: u32,
    level: u8,
    weapon_type: WeaponType,
    element: Element,
    range: u8,
    refine: u8,
    card0: i16,
    card1: i16,
    card2: i16,
    card3: i16,
    ranked_forged: bool,
}

impl WearWeaponSnapshot {
    pub fn forged_star_damage(&self) -> u16 { star_damage(self.card0, self.card1) + if self.card0 == 255 && self.ranked_forged { 10 } else { 0 } }

    pub fn creator_id(&self) -> Option<u32> {
        matches!(self.card0, 254 | 255).then(|| self.card2 as u16 as u32 | ((self.card3 as u16 as u32) << 16))
    }
}

#[derive(Debug, Clone, Copy, GettersAll)]
pub struct WearGear {
    pub item_id: i32,
    pub level: u8,
    pub location: u64,
    pub refine: u8,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
    pub def: i16,
    pub inventory_index: usize,
}

impl WearGear {
    pub fn to_snapshot(&self) -> WearGearSnapshot {
        WearGearSnapshot {
            item_id: self.item_id,
            level: self.level,
            refine: self.refine,
            card0: self.card0,
            card1: self.card1,
            card2: self.card2,
            card3: self.card3,
            def: self.def,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll)]
pub struct WearGearSnapshot {
    item_id: i32,
    level: u8,
    refine: u8,
    card0: i16,
    card1: i16,
    card2: i16,
    card3: i16,
    def: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct WearAmmo {
    pub item_id: i32,
    pub inventory_index: usize,
    pub ammo_type: AmmoType,
    pub element: Element,
    pub attack: u8,
}

impl WearAmmo {
    pub fn to_snapshot(&self) -> WearAmmoSnapshot {
        WearAmmoSnapshot {
            item_id: self.item_id,
            ammo_type: self.ammo_type,
            element: self.element,
            attack: self.attack,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll)]
pub struct WearAmmoSnapshot {
    item_id: i32,
    ammo_type: AmmoType,
    element: Element,
    attack: u8,
}

impl WearAmmoSnapshot {
    pub fn set_element(&mut self,element:Element) {self.element=element;}
}

pub trait Wearable {
    fn location(&self) -> u64;
    fn item_id(&self) -> i32;
}

impl Wearable for WearGear {
    fn location(&self) -> u64 {
        self.location
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}

impl Wearable for WearWeapon {
    fn location(&self) -> u64 {
        self.location
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}

impl Wearable for EquippedItem {
    fn location(&self) -> u64 {
        self.location
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}

impl Wearable for WearAmmo {
    fn location(&self) -> u64 {
        EquipmentLocation::Ammo.as_flag()
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}
