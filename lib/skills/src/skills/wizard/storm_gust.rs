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

// WZ_STORMGUST - Storm Gust
pub struct StormGust {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for StormGust {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        89
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
       78
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
        if status.sp() > 78 { Ok(78) } else {Err(())}
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 6000
        }
        if self.level == 2 {
            return 7000
        }
        if self.level == 3 {
            return 8000
        }
        if self.level == 4 {
            return 9000
        }
        if self.level == 5 {
            return 10000
        }
        if self.level == 6 {
            return 11000
        }
        if self.level == 7 {
            return 12000
        }
        if self.level == 8 {
            return 13000
        }
        if self.level == 9 {
            return 14000
        }
        if self.level == 10 {
            return 15000
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       5000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.400)
        }
        if self.level == 2 {
            return Some(1.800)
        }
        if self.level == 3 {
            return Some(2.200)
        }
        if self.level == 4 {
            return Some(2.600)
        }
        if self.level == 5 {
            return Some(3.000)
        }
        if self.level == 6 {
            return Some(3.400)
        }
        if self.level == 7 {
            return Some(3.800)
        }
        if self.level == 8 {
            return Some(4.200)
        }
        if self.level == 9 {
            return Some(4.600)
        }
        if self.level == 10 {
            return Some(5.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Water
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, hit: &HitContext) -> Vec<StatusInfliction> {
        if hit.hits_on_target < 3 {
            return vec![];
        }
        // Above 10_000 so MDEF reduction still leaves a likely freeze.
        vec![StatusInfliction::secondary(StatusChangeKind::Freeze, 15_000, self.level)]
    }
}
