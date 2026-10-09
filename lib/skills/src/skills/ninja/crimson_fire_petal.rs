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

// NJ_KOUENKA - Crimson Fire Petal
pub struct CrimsonFirePetal {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for CrimsonFirePetal {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        534
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
            return 18
        }
        if self.level == 2 {
            return 20
        }
        if self.level == 3 {
            return 22
        }
        if self.level == 4 {
            return 24
        }
        if self.level == 5 {
            return 26
        }
        if self.level == 6 {
            return 28
        }
        if self.level == 7 {
            return 30
        }
        if self.level == 8 {
            return 32
        }
        if self.level == 9 {
            return 34
        }
        if self.level == 10 {
            return 36
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
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
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
        if self.level == 6 {
            return 6
        }
        if self.level == 7 {
            return 7
        }
        if self.level == 8 {
            return 8
        }
        if self.level == 9 {
            return 9
        }
        if self.level == 10 {
            return 10
        }
        0
    }

    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(0.900)
        }
        if self.level == 2 {
            return Some(1.800)
        }
        if self.level == 3 {
            return Some(2.700)
        }
        if self.level == 4 {
            return Some(3.600)
        }
        if self.level == 5 {
            return Some(4.500)
        }
        if self.level == 6 {
            return Some(5.400)
        }
        if self.level == 7 {
            return Some(6.300)
        }
        if self.level == 8 {
            return Some(7.200)
        }
        if self.level == 9 {
            return Some(8.100)
        }
        if self.level == 10 {
            return Some(9.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Fire
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
