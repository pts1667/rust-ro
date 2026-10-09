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

// GS_SPREADATTACK - Spread Attack
pub struct SpreadAttack {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for SpreadAttack {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        520
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
            return 15
        }
        if self.level == 2 {
            return 20
        }
        if self.level == 3 {
            return 25
        }
        if self.level == 4 {
            return 30
        }
        if self.level == 5 {
            return 35
        }
        if self.level == 6 {
            return 40
        }
        if self.level == 7 {
            return 45
        }
        if self.level == 8 {
            return 50
        }
        if self.level == 9 {
            return 55
        }
        if self.level == 10 {
            return 60
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
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 45 { return Ok(45) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        if let Some(ammo_and_amount) = character_ammo {
            if ammo_and_amount.1 >= 5 && (8 & ammo_and_amount.0.as_flag()) > 0 { Ok(5) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 1048576 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
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
            return Some(2.800)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Ammo
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
