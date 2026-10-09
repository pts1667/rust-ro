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
use models::enums::skill_enums::SkillEnum;

// SM_BASH - Bash
pub struct Bash {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Bash {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        5
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
        if self.level == 1 {
            return 8
        }
        if self.level == 2 {
            return 8
        }
        if self.level == 3 {
            return 8
        }
        if self.level == 4 {
            return 8
        }
        if self.level == 5 {
            return 8
        }
        if self.level == 6 {
            return 15
        }
        if self.level == 7 {
            return 15
        }
        if self.level == 8 {
            return 15
        }
        if self.level == 9 {
            return 15
        }
        if self.level == 10 {
            return 15
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
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 8 { return Ok(8) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
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
            return Some(1.300)
        }
        if self.level == 2 {
            return Some(1.600)
        }
        if self.level == 3 {
            return Some(1.900)
        }
        if self.level == 4 {
            return Some(2.200)
        }
        if self.level == 5 {
            return Some(2.500)
        }
        if self.level == 6 {
            return Some(2.800)
        }
        if self.level == 7 {
            return Some(3.100)
        }
        if self.level == 8 {
            return Some(3.400)
        }
        if self.level == 9 {
            return Some(3.700)
        }
        if self.level == 10 {
            return Some(4.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Weapon
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, hit: &HitContext) -> Vec<StatusInfliction> {
        let fatal_blow = hit.source.known_skills().iter().any(|skill| skill.value == SkillEnum::SmFatalblow && skill.level > 0);
                if !fatal_blow || self.level <= 5 {
                    return vec![];
                }
                let chance = (i32::from(self.level) - 5) * hit.source.base_level().min(i32::MAX as u32) as i32 * 10;
                vec![StatusInfliction::secondary(StatusChangeKind::Stun, chance, self.level)]
    }
}
