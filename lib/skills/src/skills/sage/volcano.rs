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

// SA_VOLCANO - Volcano
pub struct Volcano {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for Volcano {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        285
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
       2
    }

    fn is_ranged(&self) -> bool {
        true
    }

    #[inline(always)]
    fn max_level(&self) -> u8 {
        5
    }

    #[inline(always)]
    fn sp_cost(&self) -> u16 {
        if self.level == 1 {
            return 48
        }
        if self.level == 2 {
            return 46
        }
        if self.level == 3 {
            return 44
        }
        if self.level == 4 {
            return 42
        }
        if self.level == 5 {
            return 40
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
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
            if status.sp() >= 48 { return Ok(48) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 715, name_english: "Yellow_Gemstone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 715 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       5000
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(10), 0, tick, 60000, 285),
                TemporaryStatusBonus::with_duration(BonusType::DamageUsingElementPercentage(Fire, 10), 0, tick, 60000, 285),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(20), 0, tick, 120000, 285),
                TemporaryStatusBonus::with_duration(BonusType::DamageUsingElementPercentage(Fire, 14), 0, tick, 120000, 285),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(30), 0, tick, 180000, 285),
                TemporaryStatusBonus::with_duration(BonusType::DamageUsingElementPercentage(Fire, 17), 0, tick, 180000, 285),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(40), 0, tick, 240000, 285),
                TemporaryStatusBonus::with_duration(BonusType::DamageUsingElementPercentage(Fire, 19), 0, tick, 240000, 285),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Atk(50), 0, tick, 300000, 285),
                TemporaryStatusBonus::with_duration(BonusType::DamageUsingElementPercentage(Fire, 20), 0, tick, 300000, 285),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn client_type(&self) -> usize {
        2
    }

    #[inline(always)]
    fn ground_kind(&self) -> Option<GroundKind> {
        Some(GroundKind::Volcano)
    }
}
