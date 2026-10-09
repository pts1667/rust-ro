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

// GS_DESPERADO - Desperado
pub struct Desperado {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Desperado {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        516
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
            return 32
        }
        if self.level == 2 {
            return 34
        }
        if self.level == 3 {
            return 36
        }
        if self.level == 4 {
            return 38
        }
        if self.level == 5 {
            return 40
        }
        if self.level == 6 {
            return 42
        }
        if self.level == 7 {
            return 44
        }
        if self.level == 8 {
            return 46
        }
        if self.level == 9 {
            return 48
        }
        if self.level == 10 {
            return 50
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
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
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 48 { return Ok(48) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {
        if let Some(ammo_and_amount) = character_ammo {
            if ammo_and_amount.1 >= 10 && (8 & ammo_and_amount.0.as_flag()) > 0 { Ok(10) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 131072 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       1000
    }

    #[inline(always)]
    fn base_after_cast_walk_delay(&self) -> u32 {
       1000
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
            return Some(1.500)
        }
        if self.level == 3 {
            return Some(2.000)
        }
        if self.level == 4 {
            return Some(2.500)
        }
        if self.level == 5 {
            return Some(3.000)
        }
        if self.level == 6 {
            return Some(3.500)
        }
        if self.level == 7 {
            return Some(4.000)
        }
        if self.level == 8 {
            return Some(4.500)
        }
        if self.level == 9 {
            return Some(5.000)
        }
        if self.level == 10 {
            return Some(5.500)
        }
        None
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
