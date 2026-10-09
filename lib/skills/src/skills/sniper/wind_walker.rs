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

// SN_WINDWALK - Wind Walker
pub struct WindWalker {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for WindWalker {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        383
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
            return 46
        }
        if self.level == 2 {
            return 52
        }
        if self.level == 3 {
            return 58
        }
        if self.level == 4 {
            return 64
        }
        if self.level == 5 {
            return 70
        }
        if self.level == 6 {
            return 76
        }
        if self.level == 7 {
            return 82
        }
        if self.level == 8 {
            return 88
        }
        if self.level == 9 {
            return 94
        }
        if self.level == 10 {
            return 100
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
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 58 { return Ok(58) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 64 { return Ok(64) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 76 { return Ok(76) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 82 { return Ok(82) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 88 { return Ok(88) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 94 { return Ok(94) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 2000
        }
        if self.level == 2 {
            return 2400
        }
        if self.level == 3 {
            return 2800
        }
        if self.level == 4 {
            return 3200
        }
        if self.level == 5 {
            return 3600
        }
        if self.level == 6 {
            return 4000
        }
        if self.level == 7 {
            return 4400
        }
        if self.level == 8 {
            return 4800
        }
        if self.level == 9 {
            return 5200
        }
        if self.level == 10 {
            return 5600
        }
        0
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       2000
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(1), 14, tick, 130000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(2), 14, tick, 130000, 383),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(1), 14, tick, 160000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(4), 14, tick, 160000, 383),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(2), 14, tick, 190000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(6), 14, tick, 190000, 383),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(2), 14, tick, 220000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(8), 14, tick, 220000, 383),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(3), 14, tick, 250000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(10), 14, tick, 250000, 383),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(3), 14, tick, 280000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(12), 14, tick, 280000, 383),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(4), 14, tick, 310000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(14), 14, tick, 310000, 383),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(4), 14, tick, 340000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(16), 14, tick, 340000, 383),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(5), 14, tick, 370000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(18), 14, tick, 370000, 383),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(5), 14, tick, 400000, 383),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(20), 14, tick, 400000, 383),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        4
    }
}
