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
use models::status_change::StatusChangeKind;

// WZ_VERMILION - Lord of Vermilion
pub struct LordofVermilion {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for LordofVermilion {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        85
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
            return 60
        }
        if self.level == 2 {
            return 64
        }
        if self.level == 3 {
            return 68
        }
        if self.level == 4 {
            return 72
        }
        if self.level == 5 {
            return 76
        }
        if self.level == 6 {
            return 80
        }
        if self.level == 7 {
            return 84
        }
        if self.level == 8 {
            return 88
        }
        if self.level == 9 {
            return 92
        }
        if self.level == 10 {
            return 96
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
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 68 { return Ok(68) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 72 { return Ok(72) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 76 { return Ok(76) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 84 { return Ok(84) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 88 { return Ok(88) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 92 { return Ok(92) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 96 { return Ok(96) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 15000
        }
        if self.level == 2 {
            return 14500
        }
        if self.level == 3 {
            return 14000
        }
        if self.level == 4 {
            return 13500
        }
        if self.level == 5 {
            return 13000
        }
        if self.level == 6 {
            return 12500
        }
        if self.level == 7 {
            return 12000
        }
        if self.level == 8 {
            return 11500
        }
        if self.level == 9 {
            return 11000
        }
        if self.level == 10 {
            return 10500
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       5000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       -10
    }

    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.000)
        }
        if self.level == 2 {
            return Some(1.200)
        }
        if self.level == 3 {
            return Some(1.400)
        }
        if self.level == 4 {
            return Some(1.600)
        }
        if self.level == 5 {
            return Some(1.800)
        }
        if self.level == 6 {
            return Some(2.000)
        }
        if self.level == 7 {
            return Some(2.200)
        }
        if self.level == 8 {
            return Some(2.400)
        }
        if self.level == 9 {
            return Some(2.600)
        }
        if self.level == 10 {
            return Some(3.800)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Wind
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        let mut effects = vec![];
        if let Some(chance) = [4, 8, 12, 16, 20, 24, 28, 32, 36, 40].get(usize::from(self.level).saturating_sub(1)) {
            effects.push(StatusInfliction::secondary(StatusChangeKind::Blind, *chance * 100, self.level));
        }
        effects
    }

    #[inline(always)]
    fn ground_kind(&self) -> Option<GroundKind> {
        Some(GroundKind::Vermilion)
    }
}
