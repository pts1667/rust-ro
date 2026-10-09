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

// AL_BLESSING - Blessing
pub struct Blessing {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Blessing {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        34
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
            return 28
        }
        if self.level == 2 {
            return 32
        }
        if self.level == 3 {
            return 36
        }
        if self.level == 4 {
            return 40
        }
        if self.level == 5 {
            return 44
        }
        if self.level == 6 {
            return 48
        }
        if self.level == 7 {
            return 52
        }
        if self.level == 8 {
            return 56
        }
        if self.level == 9 {
            return 60
        }
        if self.level == 10 {
            return 64
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
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 48 { return Ok(48) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 56 { return Ok(56) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 64 { return Ok(64) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn has_bonuses_to_target(&self) -> bool {
        true
    }

    fn bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(1), 14, tick, 60000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(1), 14, tick, 60000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(1), 14, tick, 60000, 34),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(2), 14, tick, 80000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(2), 14, tick, 80000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(2), 14, tick, 80000, 34),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(3), 14, tick, 100000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(3), 14, tick, 100000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(3), 14, tick, 100000, 34),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(4), 14, tick, 120000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(4), 14, tick, 120000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(4), 14, tick, 120000, 34),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(5), 14, tick, 140000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(5), 14, tick, 140000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(5), 14, tick, 140000, 34),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(6), 14, tick, 160000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(6), 14, tick, 160000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(6), 14, tick, 160000, 34),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(7), 14, tick, 180000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(7), 14, tick, 180000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(7), 14, tick, 180000, 34),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(8), 14, tick, 200000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(8), 14, tick, 200000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(8), 14, tick, 200000, 34),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(9), 14, tick, 220000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(9), 14, tick, 220000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(9), 14, tick, 220000, 34),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Dex(10), 14, tick, 240000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Int(10), 14, tick, 240000, 34),
                TemporaryStatusBonus::with_duration(BonusType::Str(10), 14, tick, 240000, 34),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        16
    }
}
