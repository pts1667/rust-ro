#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use std::any::Any;
use crate::{*};

// MA_SHARPSHOOTING
pub struct MaSharpshooting {
    pub(crate) level: u8,
}

impl Skill for MaSharpshooting {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level == 0 || level > 5 { return None }
        Some(Self { level })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        8215
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
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        match self.level {
            1 => 18,
            2 => 21,
            3 => 24,
            4 => 27,
            5 => 30,
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
        2000
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
        1500
    }

    #[inline(always)]
    fn companion_effect(&self) -> Option<CompanionEffect> {
        Some(CompanionEffect::Weapon(MercenaryWeapon::SharpShooting))
    }
}
