#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use models::enums::{*};
use models::enums::skill::*;
use models::enums::weapon::AmmoType;
use models::enums::element::Element::{*};
use models::item::WearWeapon;
use models::status::StatusSnapshot;
use models::item::NormalInventoryItem;
use models::enums::weapon::WeaponType::{*};
use models::enums::bonus::{BonusType};
use models::enums::status::StatusEffect::{*};
use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonus};
use models::enums::mob::MobRace::{*};
use std::any::Any;
use crate::{*};

// CR_AUTOGUARD - Guard
pub struct Guard {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Guard {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        249
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Support
    }

    fn level(&self) -> u8 {
        self.level
    }

    #[inline(always)]
    fn cast_time(&self) -> u32 {
        self.cast_time
    }

    #[inline(always)]
    fn after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }

    #[inline(always)]
    fn after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }

    #[inline(always)]
    fn update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }

    #[inline(always)]
    fn update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }

    #[inline(always)]
    fn update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
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
        10
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 12
        }
        if self.level == 2 {
            return 14
        }
        if self.level == 3 {
            return 16
        }
        if self.level == 4 {
            return 18
        }
        if self.level == 5 {
            return 20
        }
        if self.level == 6 {
            return 22
        }
        if self.level == 7 {
            return 24
        }
        if self.level == 8 {
            return 26
        }
        if self.level == 9 {
            return 28
        }
        if self.level == 10 {
            return 30
        }
        0
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
        if self.level == 1 {
            if status.sp() >= 12 { return Ok(12) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 16 { return Ok(16) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // Shield
            if status.state() & 32 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 5.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(5), 14, tick, 300000, 249),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 10.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(10), 14, tick, 300000, 249),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 15.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(14), 14, tick, 300000, 249),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 20.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(18), 14, tick, 300000, 249),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 25.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(21), 14, tick, 300000, 249),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 30.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(24), 14, tick, 300000, 249),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 35.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(26), 14, tick, 300000, 249),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 40.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(28), 14, tick, 300000, 249),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 45.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(29), 14, tick, 300000, 249),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AutospellSkillIdChancePercentage(249, 50.0), 14, tick, 300000, 249),
                TemporaryStatusBonus::with_duration(BonusType::PhysicalAttackBlockChancePercentage(30), 14, tick, 300000, 249),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        4
    }
}
