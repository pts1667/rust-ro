#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::element::Element::{*};
use models::status::StatusSnapshot;
use std::any::Any;
use crate::{*};

// SL_SMA - Sma
pub struct SlSma {
    pub(crate) level: u8,
}

impl Skill for SlSma {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 10 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        469
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
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
        8 * u16::from(self.level)
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
        2000
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        500
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        self.level as i8
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Endowed
    }

    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::Magic(MagicProfile { consumes_sma: true, ..MagicProfile::default() })
    }
}
