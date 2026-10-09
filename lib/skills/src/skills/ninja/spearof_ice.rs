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

// NJ_HYOUSENSOU - Spear of Ice
pub struct SpearofIce {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for SpearofIce {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        537
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
        if self.level == 1 {
            return 15
        }
        if self.level == 2 {
            return 18
        }
        if self.level == 3 {
            return 21
        }
        if self.level == 4 {
            return 24
        }
        if self.level == 5 {
            return 27
        }
        if self.level == 6 {
            return 30
        }
        if self.level == 7 {
            return 33
        }
        if self.level == 8 {
            return 36
        }
        if self.level == 9 {
            return 39
        }
        if self.level == 10 {
            return 42
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
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 33 { return Ok(33) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 700
        }
        if self.level == 2 {
            return 1400
        }
        if self.level == 3 {
            return 2100
        }
        if self.level == 4 {
            return 2800
        }
        if self.level == 5 {
            return 3500
        }
        if self.level == 6 {
            return 4200
        }
        if self.level == 7 {
            return 4900
        }
        if self.level == 8 {
            return 5600
        }
        if self.level == 9 {
            return 6300
        }
        if self.level == 10 {
            return 7000
        }
        0
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
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(4.500)
        }
        if self.level == 2 {
            return Some(6.000)
        }
        if self.level == 3 {
            return Some(7.500)
        }
        if self.level == 4 {
            return Some(9.000)
        }
        if self.level == 5 {
            return Some(10.500)
        }
        if self.level == 6 {
            return Some(12.000)
        }
        if self.level == 7 {
            return Some(13.500)
        }
        if self.level == 8 {
            return Some(15.000)
        }
        if self.level == 9 {
            return Some(16.500)
        }
        if self.level == 10 {
            return Some(18.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Water
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
