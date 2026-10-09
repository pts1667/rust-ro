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

// WZ_SIGHTRASHER - Sightrasher
pub struct Sightrasher {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Sightrasher {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        81
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
        0
    }

    fn is_ranged(&self) -> bool {
        false
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        10
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 35
        }
        if self.level == 2 {
            return 37
        }
        if self.level == 3 {
            return 39
        }
        if self.level == 4 {
            return 41
        }
        if self.level == 5 {
            return 43
        }
        if self.level == 6 {
            return 45
        }
        if self.level == 7 {
            return 47
        }
        if self.level == 8 {
            return 49
        }
        if self.level == 9 {
            return 51
        }
        if self.level == 10 {
            return 53
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }

    fn is_magic(&self) -> bool {
        true
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 43 { return Ok(43) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 47 { return Ok(47) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 51 { return Ok(51) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 53 { return Ok(53) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // Sight
            if status.state() & 1048576 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       500
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       2000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.200)
        }
        if self.level == 2 {
            return Some(1.400)
        }
        if self.level == 3 {
            return Some(1.600)
        }
        if self.level == 4 {
            return Some(1.800)
        }
        if self.level == 5 {
            return Some(2.000)
        }
        if self.level == 6 {
            return Some(2.200)
        }
        if self.level == 7 {
            return Some(2.400)
        }
        if self.level == 8 {
            return Some(2.600)
        }
        if self.level == 9 {
            return Some(2.800)
        }
        if self.level == 10 {
            return Some(3.000)
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
