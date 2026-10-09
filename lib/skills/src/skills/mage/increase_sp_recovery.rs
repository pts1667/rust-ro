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

// MG_SRECOVERY - Increase SP Recovery
pub struct IncreaseSpRecovery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for IncreaseSpRecovery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        9
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Passive
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
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Passive
    }

    fn is_magic(&self) -> bool {
        false
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(3, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(0.2), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(10), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(6, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(0.4), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(20), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(9, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(0.6), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(30), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(12, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(0.8), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(40), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(15, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(1.0), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(50), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(18, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(1.2), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(60), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(21, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(1.4), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(70), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(24, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(1.6), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(80), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(27, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(1.8), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(90), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenEveryMs(30, 10), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRecoveryMaxSpPercentage(2.0), StatusBonusFlag::Default.as_flag(), 9),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(100), StatusBonusFlag::Default.as_flag(), 9),]);
        }
        TemporaryStatusBonuses::default()
    }
}
