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

// PF_HPCONVERSION - Indulge
pub struct Indulge {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Indulge {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        373
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Interactive
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
        0
    }

    fn is_ranged(&self) -> bool {
        false
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 1
        }
        if self.level == 2 {
            return 2
        }
        if self.level == 3 {
            return 3
        }
        if self.level == 4 {
            return 4
        }
        if self.level == 5 {
            return 5
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
            if status.sp() >= 1 { return Ok(1) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 2 { return Ok(2) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 3 { return Ok(3) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 4 { return Ok(4) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 5 { return Ok(5) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1400
        }
        if self.level == 4 {
            return 1600
        }
        if self.level == 5 {
            return 1800
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 1200
        }
        if self.level == 3 {
            return 1400
        }
        if self.level == 4 {
            return 1600
        }
        if self.level == 5 {
            return 1800
        }
        0
    }

    #[inline(always)]
    fn class_effect(&self) -> Option<ClassEffect> {
        Some(ClassEffect::HpConversion)
    }
}
