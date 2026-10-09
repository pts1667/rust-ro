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

// HP_BASILICA - Basilica
pub struct Basilica {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Basilica {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        362
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Support
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
       4
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
        if self.level == 1 {
            return 80
        }
        if self.level == 2 {
            return 90
        }
        if self.level == 3 {
            return 100
        }
        if self.level == 4 {
            return 110
        }
        if self.level == 5 {
            return 120
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }

    fn is_magic(&self) -> bool {
        false
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 110 { return Ok(110) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 120 { return Ok(120) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 715, name_english: "Yellow_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 716, name_english: "Red_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 523, name_english: "Holy_Water".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 715 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        if !inventory.iter().any(|item| item.item_id == 716 && item.amount >= 1) {
            return Err(UseSkillFailure::RedGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        if !inventory.iter().any(|item| item.item_id == 523 && item.amount >= 1) {
            return Err(UseSkillFailure::Holywater);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 5000
        }
        if self.level == 2 {
            return 6000
        }
        if self.level == 3 {
            return 7000
        }
        if self.level == 4 {
            return 8000
        }
        if self.level == 5 {
            return 9000
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 4000
        }
        if self.level == 4 {
            return 5000
        }
        if self.level == 5 {
            return 6000
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 4000
        }
        if self.level == 4 {
            return 5000
        }
        if self.level == 5 {
            return 6000
        }
        0
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        4
    }

    #[inline(always)]
    fn ground_kind(&self) -> Option<GroundKind> {
        Some(GroundKind::Basilica)
    }

    #[inline(always)]
    fn ground_placement(&self) -> Option<GroundPlacement> {
        Some(GroundPlacement::Basilica)
    }
}
