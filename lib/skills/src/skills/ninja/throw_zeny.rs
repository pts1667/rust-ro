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

// NJ_ZENYNAGE - Throw Zeny
pub struct ThrowZeny {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for ThrowZeny {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        526
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
       50
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }

    fn is_magic(&self) -> bool {
        false
    }

    fn is_physical(&self) -> bool {
        true
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.sp() > 50 { Ok(50) } else {Err(())}
    }

    #[inline(always)]
    fn validate_zeny(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.zeny() >= 500 { return Ok(500) } else {return Err(())}
        }
        if self.level == 2 {
            if status.zeny() >= 1000 { return Ok(1000) } else {return Err(())}
        }
        if self.level == 3 {
            if status.zeny() >= 1500 { return Ok(1500) } else {return Err(())}
        }
        if self.level == 4 {
            if status.zeny() >= 2000 { return Ok(2000) } else {return Err(())}
        }
        if self.level == 5 {
            if status.zeny() >= 2500 { return Ok(2500) } else {return Err(())}
        }
        if self.level == 6 {
            if status.zeny() >= 3000 { return Ok(3000) } else {return Err(())}
        }
        if self.level == 7 {
            if status.zeny() >= 3500 { return Ok(3500) } else {return Err(())}
        }
        if self.level == 8 {
            if status.zeny() >= 4000 { return Ok(4000) } else {return Err(())}
        }
        if self.level == 9 {
            if status.zeny() >= 4500 { return Ok(4500) } else {return Err(())}
        }
        if self.level == 10 {
            if status.zeny() >= 5000 { return Ok(5000) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       5000
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
}
