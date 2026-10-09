#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::element::Element::{*};
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// ML_SPIRALPIERCE - Spiral Pierce
pub struct MlSpiralPierce {
    pub(crate) level: u8,
}

impl Skill for MlSpiralPierce {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 5 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        8218
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
    }

    fn level(&self) -> u8 {
        self.level
    }

    #[inline(always)]
    fn range(&self) -> i8 {
        4
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
        15 + 3 * u16::from(self.level)
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
        [300, 500, 700, 900, 1_000][usize::from(self.level) - 1]
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        1_000 + 200 * u32::from(self.level)
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        5
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Weapon
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![StatusInfliction::secondary(StatusChangeKind::Ankle, 10_000, self.level)]
    }

    #[inline(always)]
    fn companion_effect(&self) -> Option<CompanionEffect> {
        Some(CompanionEffect::Weapon(MercenaryWeapon::SpiralPierce))
    }
}
