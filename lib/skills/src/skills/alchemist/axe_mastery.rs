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

// AM_AXEMASTERY - Axe Mastery
pub struct AxeMastery {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for AxeMastery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        226
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
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 3), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 3), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 6), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 6), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 9), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 9), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 12), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 12), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 15), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 15), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 18), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 18), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 21), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 21), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 24), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 24), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 27), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 27), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe1H, 30), StatusBonusFlag::Default.as_flag(), 226),
                TemporaryStatusBonus::with_passive_skill(BonusType::MasteryDamageUsingWeaponType(Axe2H, 30), StatusBonusFlag::Default.as_flag(), 226),]);
        }
        TemporaryStatusBonuses::default()
    }
}
