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

// TK_RUN - Running
pub struct Running {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Running {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        411
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Interactive
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
            return 100
        }
        if self.level == 2 {
            return 90
        }
        if self.level == 3 {
            return 80
        }
        if self.level == 4 {
            return 70
        }
        if self.level == 5 {
            return 60
        }
        if self.level == 6 {
            return 50
        }
        if self.level == 7 {
            return 40
        }
        if self.level == 8 {
            return 30
        }
        if self.level == 9 {
            return 20
        }
        if self.level == 10 {
            return 10
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
            if status.sp() >= 100 { return Ok(100) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 90 { return Ok(90) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 70 { return Ok(70) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 10 { return Ok(10) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if status.state() > 0 {
            // MoveEnable
            if status.state() & 128 > 0 { Ok(()) } else { Err(()) }
        } else {
            Err(())
        }
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
        if self.level == 1 {
            return 6000
        }
        if self.level == 2 {
            return 5000
        }
        if self.level == 3 {
            return 4000
        }
        if self.level == 4 {
            return 3000
        }
        if self.level == 5 {
            return 2000
        }
        if self.level == 6 {
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
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 10), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 20), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 30), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 40), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 50), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 60), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 70), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 80), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 90), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(Fist, 100), 0, tick, 1000, 411),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-56), 0, tick, 1000, 411),]);
        }
        TemporaryStatusBonuses::default()
    }
}
