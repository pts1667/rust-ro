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

// NJ_ISSEN - Final Strike
pub struct FinalStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for FinalStrike {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        544
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
       -5
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
            return 55
        }
        if self.level == 2 {
            return 60
        }
        if self.level == 3 {
            return 65
        }
        if self.level == 4 {
            return 70
        }
        if self.level == 5 {
            return 75
        }
        if self.level == 6 {
            return 80
        }
        if self.level == 7 {
            return 85
        }
        if self.level == 8 {
            return 90
        }
        if self.level == 9 {
            return 95
        }
        if self.level == 10 {
            return 100
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
            if status.sp() >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 65 { return Ok(65) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 75 { return Ok(75) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 85 { return Ok(85) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 95 { return Ok(95) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(40.000)
        }
        if self.level == 2 {
            return Some(80.000)
        }
        if self.level == 3 {
            return Some(120.000)
        }
        if self.level == 4 {
            return Some(160.000)
        }
        if self.level == 5 {
            return Some(200.000)
        }
        if self.level == 6 {
            return Some(240.000)
        }
        if self.level == 7 {
            return Some(280.000)
        }
        if self.level == 8 {
            return Some(320.000)
        }
        if self.level == 9 {
            return Some(360.000)
        }
        if self.level == 10 {
            return Some(400.000)
        }
        None
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
    fn player_only_callback(&self) -> bool {
        true
    }
}
