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
use models::status_change::StatusChangeKind;
use crate::{*};

// AL_DECAGI - Decrease AGI
pub struct DecreaseAgi {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for DecreaseAgi {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        30
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
        if self.level == 1 {
            return 15
        }
        if self.level == 2 {
            return 17
        }
        if self.level == 3 {
            return 19
        }
        if self.level == 4 {
            return 21
        }
        if self.level == 5 {
            return 23
        }
        if self.level == 6 {
            return 25
        }
        if self.level == 7 {
            return 27
        }
        if self.level == 8 {
            return 29
        }
        if self.level == 9 {
            return 31
        }
        if self.level == 10 {
            return 33
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Target
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
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 17 { return Ok(17) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 19 { return Ok(19) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 31 { return Ok(31) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 33 { return Ok(33) } else {return Err(())}
        }
        Err(())
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
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-3), 14, tick, 40000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 40000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 53.0), 14, tick, 40000, 30),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-4), 14, tick, 50000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 50000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 56.0), 14, tick, 50000, 30),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-5), 14, tick, 60000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 60000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 59.0), 14, tick, 60000, 30),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-6), 14, tick, 70000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 70000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 62.0), 14, tick, 70000, 30),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-7), 14, tick, 80000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 80000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 65.0), 14, tick, 80000, 30),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-8), 14, tick, 90000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 90000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 68.0), 14, tick, 90000, 30),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-9), 14, tick, 100000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 100000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 71.0), 14, tick, 100000, 30),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-10), 14, tick, 110000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 110000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 74.0), 14, tick, 110000, 30),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-11), 14, tick, 120000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 120000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 77.0), 14, tick, 120000, 30),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-12), 14, tick, 130000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-25), 14, tick, 130000, 30),
                TemporaryStatusBonus::with_duration(BonusType::SkillIdSuccessPercentage(30, 80.0), 14, tick, 130000, 30),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        16
    }

    #[inline(always)]
    fn status_kind(&self) -> Option<StatusChangeKind> {
        Some(StatusChangeKind::DecreaseAgi)
    }
}
