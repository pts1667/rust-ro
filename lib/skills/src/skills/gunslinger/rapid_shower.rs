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

// GS_RAPIDSHOWER - Rapid Shower
pub struct RapidShower {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for RapidShower {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        515
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
            return 22
        }
        if self.level == 2 {
            return 24
        }
        if self.level == 3 {
            return 26
        }
        if self.level == 4 {
            return 28
        }
        if self.level == 5 {
            return 30
        }
        if self.level == 6 {
            return 32
        }
        if self.level == 7 {
            return 34
        }
        if self.level == 8 {
            return 36
        }
        if self.level == 9 {
            return 38
        }
        if self.level == 10 {
            return 40
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
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
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
    fn hit_count(&self) -> i8 {
       -5
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(5.500)
        }
        if self.level == 2 {
            return Some(6.000)
        }
        if self.level == 3 {
            return Some(6.500)
        }
        if self.level == 4 {
            return Some(7.000)
        }
        if self.level == 5 {
            return Some(7.500)
        }
        if self.level == 6 {
            return Some(8.000)
        }
        if self.level == 7 {
            return Some(8.500)
        }
        if self.level == 8 {
            return Some(9.000)
        }
        if self.level == 9 {
            return Some(9.500)
        }
        if self.level == 10 {
            return Some(10.000)
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
