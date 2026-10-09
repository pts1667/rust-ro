#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// MS_BOWLINGBASH
pub struct MsBowlingbash {
    pub(crate) level: u8,
}

impl Skill for MsBowlingbash {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 10 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        8203
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
    }

    fn level(&self) -> u8 {
        self.level
    }

    #[inline(always)]
    fn range(&self) -> i8 {
        -2
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
        match self.level {
            1 => 13,
            2 => 14,
            3 => 15,
            4 => 16,
            5 => 17,
            6 => 18,
            7 => 19,
            8 => 20,
            9 => 21,
            10 => 22,
            _ => 0,
        }
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
        let cost = u32::from(self.sp_cost());
        if status.sp() >= cost { Ok(cost) } else { Err(()) }
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        700
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        0
    }

    #[inline(always)]
    fn companion_effect(&self) -> Option<CompanionEffect> {
        Some(CompanionEffect::Weapon(MercenaryWeapon::BowlingBash))
    }
}
