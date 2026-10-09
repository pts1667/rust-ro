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

// GS_GROUNDDRIFT - Ground Drift
pub struct GroundDrift {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for GroundDrift {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        521
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
       -9
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
            return 4
        }
        if self.level == 2 {
            return 8
        }
        if self.level == 3 {
            return 12
        }
        if self.level == 4 {
            return 16
        }
        if self.level == 5 {
            return 20
        }
        if self.level == 6 {
            return 24
        }
        if self.level == 7 {
            return 28
        }
        if self.level == 8 {
            return 32
        }
        if self.level == 9 {
            return 36
        }
        if self.level == 10 {
            return 40
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
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
            if status.sp() >= 4 { return Ok(4) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        if let Some(ammo_and_amount) = character_ammo {
            if ammo_and_amount.1 >= 1 && (32 & ammo_and_amount.0.as_flag()) > 0 { Ok(1) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 2097152 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       2000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
       Some(1.000)
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Ammo
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
}
