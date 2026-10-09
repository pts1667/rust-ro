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

// MG_THUNDERSTORM - Thunder Storm
pub struct ThunderStorm {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for ThunderStorm {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        21
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
            return 29
        }
        if self.level == 2 {
            return 34
        }
        if self.level == 3 {
            return 39
        }
        if self.level == 4 {
            return 44
        }
        if self.level == 5 {
            return 49
        }
        if self.level == 6 {
            return 54
        }
        if self.level == 7 {
            return 59
        }
        if self.level == 8 {
            return 64
        }
        if self.level == 9 {
            return 69
        }
        if self.level == 10 {
            return 74
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
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
            if status.sp() >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 54 { return Ok(54) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 59 { return Ok(59) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 69 { return Ok(69) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 74 { return Ok(74) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 1000
        }
        if self.level == 2 {
            return 2000
        }
        if self.level == 3 {
            return 3000
        }
        if self.level == 4 {
            return 4000
        }
        if self.level == 5 {
            return 5000
        }
        if self.level == 6 {
            return 6000
        }
        if self.level == 7 {
            return 7000
        }
        if self.level == 8 {
            return 8000
        }
        if self.level == 9 {
            return 9000
        }
        if self.level == 10 {
            return 10000
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       2000
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
            return Some(0.800)
        }
        if self.level == 2 {
            return Some(1.600)
        }
        if self.level == 3 {
            return Some(2.400)
        }
        if self.level == 4 {
            return Some(3.200)
        }
        if self.level == 5 {
            return Some(4.000)
        }
        if self.level == 6 {
            return Some(4.800)
        }
        if self.level == 7 {
            return Some(5.600)
        }
        if self.level == 8 {
            return Some(6.400)
        }
        if self.level == 9 {
            return Some(7.200)
        }
        if self.level == 10 {
            return Some(8.000)
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

    #[inline(always)]
    fn ground_kind(&self) -> Option<GroundKind> {
        Some(GroundKind::Thunderstorm)
    }

    #[inline(always)]
    fn pet_ground_attack(&self) -> bool {
        true
    }
}
