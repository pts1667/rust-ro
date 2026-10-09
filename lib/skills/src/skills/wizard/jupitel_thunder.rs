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

// WZ_JUPITEL - Jupitel Thunder
pub struct JupitelThunder {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for JupitelThunder {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        84
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
            return 20
        }
        if self.level == 2 {
            return 23
        }
        if self.level == 3 {
            return 26
        }
        if self.level == 4 {
            return 29
        }
        if self.level == 5 {
            return 32
        }
        if self.level == 6 {
            return 35
        }
        if self.level == 7 {
            return 38
        }
        if self.level == 8 {
            return 41
        }
        if self.level == 9 {
            return 44
        }
        if self.level == 10 {
            return 47
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
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
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 47 { return Ok(47) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 2500
        }
        if self.level == 2 {
            return 3000
        }
        if self.level == 3 {
            return 3500
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 4500
        }
        if self.level == 6 {
            return 5000
        }
        if self.level == 7 {
            return 5500
        }
        if self.level == 8 {
            return 6000
        }
        if self.level == 9 {
            return 6500
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
            return Some(3.000)
        }
        if self.level == 2 {
            return Some(4.000)
        }
        if self.level == 3 {
            return Some(5.000)
        }
        if self.level == 4 {
            return Some(6.000)
        }
        if self.level == 5 {
            return Some(7.000)
        }
        if self.level == 6 {
            return Some(8.000)
        }
        if self.level == 7 {
            return Some(9.000)
        }
        if self.level == 8 {
            return Some(10.000)
        }
        if self.level == 9 {
            return Some(11.000)
        }
        if self.level == 10 {
            return Some(12.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Wind
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
}
