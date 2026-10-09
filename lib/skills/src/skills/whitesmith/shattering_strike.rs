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

// WS_MELTDOWN - Shattering Strike
pub struct ShatteringStrike {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for ShatteringStrike {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        384
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
            return 50
        }
        if self.level == 2 {
            return 50
        }
        if self.level == 3 {
            return 60
        }
        if self.level == 4 {
            return 60
        }
        if self.level == 5 {
            return 70
        }
        if self.level == 6 {
            return 70
        }
        if self.level == 7 {
            return 80
        }
        if self.level == 8 {
            return 80
        }
        if self.level == 9 {
            return 90
        }
        if self.level == 10 {
            return 90
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
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 500
        }
        if self.level == 2 {
            return 500
        }
        if self.level == 3 {
            return 600
        }
        if self.level == 4 {
            return 600
        }
        if self.level == 5 {
            return 700
        }
        if self.level == 6 {
            return 700
        }
        if self.level == 7 {
            return 800
        }
        if self.level == 8 {
            return 800
        }
        if self.level == 9 {
            return 900
        }
        if self.level == 10 {
            return 1000
        }
        0
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(0.7), 14, tick, 15000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(1), 14, tick, 15000, 384),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(1.4), 14, tick, 20000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(2), 14, tick, 20000, 384),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(2.1), 14, tick, 25000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(3), 14, tick, 25000, 384),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(2.8), 14, tick, 30000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(4), 14, tick, 30000, 384),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(3.5), 14, tick, 35000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(5), 14, tick, 35000, 384),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(4.2), 14, tick, 40000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(6), 14, tick, 40000, 384),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(4.9), 14, tick, 45000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(7), 14, tick, 45000, 384),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(5.6), 14, tick, 50000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(8), 14, tick, 50000, 384),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(6.3), 14, tick, 55000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(9), 14, tick, 55000, 384),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::BreakArmorPercentage(7.0), 14, tick, 60000, 384),
                TemporaryStatusBonus::with_duration(BonusType::BreakWeaponPercentage(10), 14, tick, 60000, 384),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        4
    }
}
