#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// HLIF_CHANGE
pub struct HlifChange {
    pub(crate) level: u8,
}

impl Skill for HlifChange {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 3 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        8004
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Support
    }

    fn level(&self) -> u8 {
        self.level
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
        3
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        100
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::MySelf
    }

    fn is_magic(&self) -> bool {
        false
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
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        match self.level {
            1 => 600000,
            2 => 900000,
            3 => 1200000,
            _ => 0,
        }
    }

    #[inline(always)]
    fn companion_effect(&self) -> Option<CompanionEffect> {
        Some(CompanionEffect::Transform)
    }
}
