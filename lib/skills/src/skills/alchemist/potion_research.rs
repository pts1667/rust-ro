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

// AM_LEARNINGPOTION - Potion Research
pub struct PotionResearch {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for PotionResearch {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        227
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
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(5), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(5), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 1.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(10), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(10), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 2.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(15), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(15), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 3.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(20), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(20), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 4.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(25), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(25), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 5.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(30), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(30), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 6.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(35), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(35), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 7.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(40), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(40), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 8.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(45), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(45), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 9.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::HpRegenFromItemPercentage(50), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SpRegenFromItemPercentage(50), StatusBonusFlag::Default.as_flag(), 227),
                TemporaryStatusBonus::with_passive_skill(BonusType::SkillIdSuccessPercentage(227, 10.0), StatusBonusFlag::Default.as_flag(), 227),]);
        }
        TemporaryStatusBonuses::default()
    }
}
