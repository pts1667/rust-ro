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

// AL_ANGELUS - Angelus
pub struct Angelus {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Angelus {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        33
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
            return 23
        }
        if self.level == 2 {
            return 26
        }
        if self.level == 3 {
            return 29
        }
        if self.level == 4 {
            return 32
        }
        if self.level == 5 {
            return 35
        }
        if self.level == 6 {
            return 38
        }
        if self.level == 7 {
            return 41
        }
        if self.level == 8 {
            return 44
        }
        if self.level == 9 {
            return 47
        }
        if self.level == 10 {
            return 50
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Party
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
            if status.sp() >= 23 { return Ok(23) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 29 { return Ok(29) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 35 { return Ok(35) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 47 { return Ok(47) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       500
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       3500
    }

    #[inline(always)]
    fn has_bonuses_to_party(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(5), 0, tick, 30000, 33),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(10), 0, tick, 60000, 33),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(15), 0, tick, 90000, 33),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(20), 0, tick, 120000, 33),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(25), 0, tick, 150000, 33),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(30), 0, tick, 180000, 33),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(35), 0, tick, 210000, 33),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(40), 0, tick, 240000, 33),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(45), 0, tick, 270000, 33),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::VitDefPercentage(50), 0, tick, 300000, 33),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        4
    }
}
