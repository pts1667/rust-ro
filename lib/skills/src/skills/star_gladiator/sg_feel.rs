#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::status::StatusSnapshot;
use std::any::Any;
use crate::{*};

// SG_FEEL - Feel
pub struct SgFeel {
    pub(crate) level: u8,
}

impl Skill for SgFeel {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 3 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        427
    }

    #[inline(always)]
    fn range(&self) -> i8 {
        0
    }

    fn is_ranged(&self) -> bool {
        false
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Support
    }

    fn level(&self) -> u8 {
        self.level
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
    fn menu(&self) -> Option<MenuKind> {
        Some(MenuKind::StarPlace)
    }
}
