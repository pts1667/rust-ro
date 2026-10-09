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

// AL_INCAGI - Increase AGI
pub struct IncreaseAgi {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for IncreaseAgi {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        29
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
            return 18
        }
        if self.level == 2 {
            return 21
        }
        if self.level == 3 {
            return 24
        }
        if self.level == 4 {
            return 27
        }
        if self.level == 5 {
            return 30
        }
        if self.level == 6 {
            return 33
        }
        if self.level == 7 {
            return 36
        }
        if self.level == 8 {
            return 39
        }
        if self.level == 9 {
            return 42
        }
        if self.level == 10 {
            return 45
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
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 33 { return Ok(33) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 45 { return Ok(45) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_hp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if status.hp() > 15 { Ok(15) } else {Err(())}
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
    fn has_bonuses_to_target(&self) -> bool {
        true
    }

    fn bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(3), 14, tick, 60000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 60000, 29),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(4), 14, tick, 80000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 80000, 29),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(5), 14, tick, 100000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 100000, 29),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(6), 14, tick, 120000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 120000, 29),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(7), 14, tick, 140000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 140000, 29),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(8), 14, tick, 160000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 160000, 29),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(9), 14, tick, 180000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 180000, 29),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(10), 14, tick, 200000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 200000, 29),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(11), 14, tick, 220000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 220000, 29),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(12), 14, tick, 240000, 29),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), 14, tick, 240000, 29),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        16
    }

    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::UndeadBuffDamage
    }
}
