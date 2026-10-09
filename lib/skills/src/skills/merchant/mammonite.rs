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

// MC_MAMMONITE - Mammonite
pub struct Mammonite {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Mammonite {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        42
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
       -1
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
       5
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
        if status.sp() > 5 { Ok(5) } else {Err(())}
    }

    #[inline(always)]
    fn validate_zeny(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.zeny() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 2 {
            if status.zeny() >= 200 { return Ok(200) } else {return Err(())}
        }
        if self.level == 3 {
            if status.zeny() >= 300 { return Ok(300) } else {return Err(())}
        }
        if self.level == 4 {
            if status.zeny() >= 400 { return Ok(400) } else {return Err(())}
        }
        if self.level == 5 {
            if status.zeny() >= 500 { return Ok(500) } else {return Err(())}
        }
        if self.level == 6 {
            if status.zeny() >= 600 { return Ok(600) } else {return Err(())}
        }
        if self.level == 7 {
            if status.zeny() >= 700 { return Ok(700) } else {return Err(())}
        }
        if self.level == 8 {
            if status.zeny() >= 800 { return Ok(800) } else {return Err(())}
        }
        if self.level == 9 {
            if status.zeny() >= 900 { return Ok(900) } else {return Err(())}
        }
        if self.level == 10 {
            if status.zeny() >= 1000 { return Ok(1000) } else {return Err(())}
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
            return Some(1.500)
        }
        if self.level == 2 {
            return Some(2.000)
        }
        if self.level == 3 {
            return Some(2.500)
        }
        if self.level == 4 {
            return Some(3.000)
        }
        if self.level == 5 {
            return Some(3.500)
        }
        if self.level == 6 {
            return Some(4.000)
        }
        if self.level == 7 {
            return Some(4.500)
        }
        if self.level == 8 {
            return Some(5.000)
        }
        if self.level == 9 {
            return Some(5.500)
        }
        if self.level == 10 {
            return Some(6.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Weapon
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }

    #[inline(always)]
    fn cost_rules(&self) -> CostRules {
        CostRules { unfair_trick_zeny: true, ..CostRules::default() }
    }
}
