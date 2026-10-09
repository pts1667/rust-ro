#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// MER_DECAGI
pub struct MerDecagi {
    pub(crate) level: u8,
}

impl Skill for MerDecagi {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 10 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        8234
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Support
    }

    fn level(&self) -> u8 {
        self.level
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
        match self.level {
            1 => 15,
            2 => 17,
            3 => 19,
            4 => 21,
            5 => 23,
            6 => 25,
            7 => 27,
            8 => 29,
            9 => 31,
            10 => 33,
            _ => 0,
        }
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
    }

    fn is_magic(&self) -> bool {
        true
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        let cost = u32::from(self.sp_cost());
        if status.sp() >= cost { Ok(cost) } else { Err(()) }
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        1000
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        1000
    }

    #[inline(always)]
    fn companion_effect(&self) -> Option<CompanionEffect> {
        Some(CompanionEffect::MercenaryDecreaseAgi)
    }
}
