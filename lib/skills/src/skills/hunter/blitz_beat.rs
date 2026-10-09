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

// HT_BLITZBEAT - Blitz Beat
pub struct BlitzBeat {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for BlitzBeat {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        129
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
       5
    }

    fn is_ranged(&self) -> bool {
        true
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 10
        }
        if self.level == 2 {
            return 13
        }
        if self.level == 3 {
            return 16
        }
        if self.level == 4 {
            return 19
        }
        if self.level == 5 {
            return 22
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
            if status.sp() >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 13 { return Ok(13) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // Falcon
            if status.state() & 8 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       1500
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       1000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        self.level as i8
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Neutral
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
}
