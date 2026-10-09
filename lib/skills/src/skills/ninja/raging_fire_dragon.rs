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

// NJ_BAKUENRYU - Raging Fire Dragon
pub struct RagingFireDragon {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for RagingFireDragon {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        536
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
        if self.level == 1 {
            return 20
        }
        if self.level == 2 {
            return 25
        }
        if self.level == 3 {
            return 30
        }
        if self.level == 4 {
            return 35
        }
        if self.level == 5 {
            return 40
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
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
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 7521, name_english: "Flame_Stone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 7521 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       3000
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       2000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       -3
    }

    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(3.000)
        }
        if self.level == 2 {
            return Some(4.500)
        }
        if self.level == 3 {
            return Some(6.000)
        }
        if self.level == 4 {
            return Some(7.500)
        }
        if self.level == 5 {
            return Some(9.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Fire
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
}
