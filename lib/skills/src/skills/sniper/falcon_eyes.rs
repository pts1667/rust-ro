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

// SN_SIGHT - Falcon Eyes
pub struct FalconEyes {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for FalconEyes {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        380
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
            return 20
        }
        if self.level == 2 {
            return 20
        }
        if self.level == 3 {
            return 25
        }
        if self.level == 4 {
            return 25
        }
        if self.level == 5 {
            return 30
        }
        if self.level == 6 {
            return 30
        }
        if self.level == 7 {
            return 35
        }
        if self.level == 8 {
            return 35
        }
        if self.level == 9 {
            return 40
        }
        if self.level == 10 {
            return 40
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
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(1.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(2), 14, tick, 30000, 380),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(2.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(4), 14, tick, 30000, 380),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(3.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(6), 14, tick, 30000, 380),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(4.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(8), 14, tick, 30000, 380),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(5.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(10), 14, tick, 30000, 380),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(6.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(12), 14, tick, 30000, 380),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(7.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(14), 14, tick, 30000, 380),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(8.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(16), 14, tick, 30000, 380),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(9.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(18), 14, tick, 30000, 380),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AllStats(5), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::Crit(10.0), 14, tick, 30000, 380),
                TemporaryStatusBonus::with_duration(BonusType::AtkPercentage(20), 14, tick, 30000, 380),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        4
    }
}
