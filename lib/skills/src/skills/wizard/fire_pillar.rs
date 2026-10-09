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

// WZ_FIREPILLAR - Fire Pillar
pub struct FirePillar {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for FirePillar {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        80
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
        10
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
       75
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
    }

    fn is_magic(&self) -> bool {
        true
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 75 { Ok(75) } else {Err(())}
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 3000
        }
        if self.level == 2 {
            return 2700
        }
        if self.level == 3 {
            return 2400
        }
        if self.level == 4 {
            return 2100
        }
        if self.level == 5 {
            return 1800
        }
        if self.level == 6 {
            return 1500
        }
        if self.level == 7 {
            return 1200
        }
        if self.level == 8 {
            return 900
        }
        if self.level == 9 {
            return 600
        }
        if self.level == 10 {
            return 300
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       1000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        if self.level == 1 {
            return 3
        }
        if self.level == 2 {
            return 4
        }
        if self.level == 3 {
            return 5
        }
        if self.level == 4 {
            return 6
        }
        if self.level == 5 {
            return 7
        }
        if self.level == 6 {
            return 8
        }
        if self.level == 7 {
            return 9
        }
        if self.level == 8 {
            return 10
        }
        if self.level == 9 {
            return 11
        }
        if self.level == 10 {
            return 12
        }
        0
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Fire
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }

    #[inline(always)]
    fn ground_kind(&self) -> Option<GroundKind> {
        Some(GroundKind::FirePillar)
    }
}
