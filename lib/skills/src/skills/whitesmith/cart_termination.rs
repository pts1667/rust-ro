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

// WS_CARTTERMINATION - Cart Termination
pub struct CartTermination {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for CartTermination {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        485
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
       -2
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
       15
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
        if status.sp() > 15 { Ok(15) } else {Err(())}
    }

    #[inline(always)]
    fn validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // CartBoost
            if status.state() & 16777216 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn validate_zeny(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.zeny() >= 600 { return Ok(600) } else {return Err(())}
        }
        if self.level == 2 {
            if status.zeny() >= 700 { return Ok(700) } else {return Err(())}
        }
        if self.level == 3 {
            if status.zeny() >= 800 { return Ok(800) } else {return Err(())}
        }
        if self.level == 4 {
            if status.zeny() >= 900 { return Ok(900) } else {return Err(())}
        }
        if self.level == 5 {
            if status.zeny() >= 1000 { return Ok(1000) } else {return Err(())}
        }
        if self.level == 6 {
            if status.zeny() >= 1100 { return Ok(1100) } else {return Err(())}
        }
        if self.level == 7 {
            if status.zeny() >= 1200 { return Ok(1200) } else {return Err(())}
        }
        if self.level == 8 {
            if status.zeny() >= 1300 { return Ok(1300) } else {return Err(())}
        }
        if self.level == 9 {
            if status.zeny() >= 1400 { return Ok(1400) } else {return Err(())}
        }
        if self.level == 10 {
            if status.zeny() >= 1500 { return Ok(1500) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 8386559 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
        } else {
            // Allow to use Fist
            Ok(())
        }
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn dmg_atk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(5.330)
        }
        if self.level == 2 {
            return Some(5.710)
        }
        if self.level == 3 {
            return Some(6.150)
        }
        if self.level == 4 {
            return Some(6.660)
        }
        if self.level == 5 {
            return Some(7.270)
        }
        if self.level == 6 {
            return Some(8.000)
        }
        if self.level == 7 {
            return Some(8.880)
        }
        if self.level == 8 {
            return Some(10.000)
        }
        if self.level == 9 {
            return Some(11.420)
        }
        if self.level == 10 {
            return Some(13.330)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Weapon
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        let mut effects = vec![];
        if let Some(chance) = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50].get(usize::from(self.level).saturating_sub(1)) {
            effects.push(StatusInfliction::secondary(StatusChangeKind::Stun, *chance * 100, self.level));
        }
        effects
    }

    #[inline(always)]
    fn bypasses_reflect_shield(&self) -> bool {
        true
    }
}
