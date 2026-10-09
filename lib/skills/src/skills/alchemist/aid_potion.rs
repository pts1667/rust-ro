#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::weapon::AmmoType;
use models::enums::element::Element::{*};
use models::item::WearWeapon;
use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;
use models::enums::weapon::WeaponType::{*};
use models::enums::bonus::{BonusType};
use models::enums::status::StatusEffect::{*};
use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonus};
use models::enums::mob::MobRace::{*};
use std::any::Any;
use crate::{*};

// AM_POTIONPITCHER - Aid Potion
pub struct AidPotion {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for AidPotion {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        231
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
    }

    fn level(&self) -> u8 {
        self.level
    }

    #[inline(always)]
    fn cast_time(&self) -> u32 {
        self.cast_time
    }

    #[inline(always)]
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }

    #[inline(always)]
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }

    #[inline(always)]
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }

    #[inline(always)]
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }

    #[inline(always)]
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }

    #[inline(always)]
    fn range(&self) -> i8 {
       9
    }

    fn is_ranged(&self) -> bool {
        true
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
       1
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }

    fn is_magic(&self) -> bool {
        true
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 1 { Ok(1) } else {Err(())}
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 501, name_english: "Red_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 502, name_english: "Orange_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 503, name_english: "Yellow_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 504, name_english: "White_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 505, name_english: "Blue_Potion".to_string(), amount: 1}),(NormalInventoryItem {item_id: 522, name_english: "Fruit_Of_Mastela".to_string(), amount: 1}),(NormalInventoryItem {item_id: 526, name_english: "Royal_Jelly".to_string(), amount: 1}),(NormalInventoryItem {item_id: 608, name_english: "Seed_Of_Yggdrasil".to_string(), amount: 1}),(NormalInventoryItem {item_id: 607, name_english: "Yggdrasilberry".to_string(), amount: 1}),(NormalInventoryItem {item_id: 657, name_english: "Berserk_Potion".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 501 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 502 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 503 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 504 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 505 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 522 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 526 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 608 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 607 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 657 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       500
    }

    #[inline(always)]
    fn has_bonuses_to_target(&self) -> bool {
        true
    }

    fn bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Neutral
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }

    #[inline(always)]
    fn class_effect(&self) -> Option<ClassEffect> {
        Some(ClassEffect::AidPotion)
    }

    #[inline(always)]
    fn cost_rules(&self) -> CostRules {
        CostRules { item_per_level: true, ..CostRules::default() }
    }
}
