#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// SL_SUPERNOVICE
pub struct SlSupernovice {
    pub(crate) level: u8,
}

impl Skill for SlSupernovice {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 5 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        451
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
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        match self.level {
            1 => 460,
            2 => 360,
            3 => 260,
            4 => 160,
            5 => 60,
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
        500
    }

    #[inline(always)]
    fn class_effect(&self) -> Option<ClassEffect> {
        Some(ClassEffect::SoulLink)
    }
}
