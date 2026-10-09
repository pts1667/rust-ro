#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::element::Element::{*};
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// MA_FREEZINGTRAP - Freezing Trap
pub struct MaFreezingTrap {
    pub(crate) level: u8,
}

impl Skill for MaFreezingTrap {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 5 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        8212
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
    }

    fn level(&self) -> u8 {
        self.level
    }

    #[inline(always)]
    fn range(&self) -> i8 {
        3
    }

    fn is_ranged(&self) -> bool {
        false
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        10
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
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
    fn hit_count(&self) -> i8 {
        1
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Water
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![StatusInfliction::secondary(StatusChangeKind::Freeze, 10_000, self.level).delayed(StatusDelay::AfterAttackMotion(100))]
    }
}
