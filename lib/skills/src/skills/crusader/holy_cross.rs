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

// CR_HOLYCROSS - Holy Cross
pub struct HolyCross {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for HolyCross {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        253
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
       -2
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
            return 11
        }
        if self.level == 2 {
            return 12
        }
        if self.level == 3 {
            return 13
        }
        if self.level == 4 {
            return 14
        }
        if self.level == 5 {
            return 15
        }
        if self.level == 6 {
            return 16
        }
        if self.level == 7 {
            return 17
        }
        if self.level == 8 {
            return 18
        }
        if self.level == 9 {
            return 19
        }
        if self.level == 10 {
            return 20
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
        true
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 11 { return Ok(11) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 13 { return Ok(13) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       -2
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.350)
        }
        if self.level == 2 {
            return Some(1.700)
        }
        if self.level == 3 {
            return Some(2.050)
        }
        if self.level == 4 {
            return Some(2.400)
        }
        if self.level == 5 {
            return Some(2.750)
        }
        if self.level == 6 {
            return Some(3.100)
        }
        if self.level == 7 {
            return Some(3.450)
        }
        if self.level == 8 {
            return Some(3.800)
        }
        if self.level == 9 {
            return Some(4.150)
        }
        if self.level == 10 {
            return Some(4.500)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Holy
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![StatusInfliction::secondary(StatusChangeKind::Blind, 300 * i32::from(self.level), self.level)]
    }
}
