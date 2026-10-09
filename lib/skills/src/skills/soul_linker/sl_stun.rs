#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::element::Element::{*};
use models::enums::size::Size;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// SL_STUN - Stun
pub struct SlStun {
    pub(crate) level: u8,
}

impl Skill for SlStun {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 7 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        468
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
        7
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        16 + 2 * u16::from(self.level)
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
        100
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        500
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        1
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Endowed
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, hit: &HitContext) -> Vec<StatusInfliction> {
        if *hit.target.size() != Size::Medium {
            return vec![];
        }
        vec![StatusInfliction::primary(StatusChangeKind::Stun, (30 + 10 * i32::from(self.level)) * 100, self.level)]
    }

    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::Magic(MagicProfile { grants_sma_from_level: Some(7), es_magic: true, ..MagicProfile::default() })
    }

    #[inline(always)]
    fn cost_rules(&self) -> CostRules {
        CostRules { kaina_sp_reduction: true, ..CostRules::default() }
    }

    #[inline(always)]
    fn magic_modifier(&self, _target_small: bool, _source_base_level: u32) -> Option<f32> {
        Some(1.0 + 0.05 * f32::from(self.level))
    }
}
