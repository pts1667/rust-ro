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

// BA_WHISTLE - Perfect Tablature
pub struct PerfectTablature {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for PerfectTablature {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        319
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Performance
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
            return 24
        }
        if self.level == 2 {
            return 28
        }
        if self.level == 3 {
            return 32
        }
        if self.level == 4 {
            return 36
        }
        if self.level == 5 {
            return 40
        }
        if self.level == 6 {
            return 44
        }
        if self.level == 7 {
            return 48
        }
        if self.level == 8 {
            return 52
        }
        if self.level == 9 {
            return 56
        }
        if self.level == 10 {
            return 60
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
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 48 { return Ok(48) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 56 { return Ok(56) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 60 { return Ok(60) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 24576 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
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
                TemporaryStatusBonus::with_duration(BonusType::Flee(1), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(10), 0, tick, 60000, 319),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(2), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(20), 0, tick, 60000, 319),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(3), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(30), 0, tick, 60000, 319),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(4), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(40), 0, tick, 60000, 319),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(5), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(50), 0, tick, 60000, 319),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(6), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(60), 0, tick, 60000, 319),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(7), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(70), 0, tick, 60000, 319),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(8), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(80), 0, tick, 60000, 319),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(9), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(90), 0, tick, 60000, 319),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Flee(10), 0, tick, 60000, 319),
                TemporaryStatusBonus::with_duration(BonusType::Luk(100), 0, tick, 60000, 319),]);
        }
        TemporaryStatusBonuses::default()
    }
}
