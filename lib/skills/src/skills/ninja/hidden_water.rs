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

// NJ_SUITON - Hidden Water
pub struct HiddenWater {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for HiddenWater {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        538
    }

    fn skill_type(&self) -> SkillType {
        SkillType::Offensive
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
            return 15
        }
        if self.level == 2 {
            return 18
        }
        if self.level == 3 {
            return 21
        }
        if self.level == 4 {
            return 24
        }
        if self.level == 5 {
            return 27
        }
        if self.level == 6 {
            return 30
        }
        if self.level == 7 {
            return 33
        }
        if self.level == 8 {
            return 36
        }
        if self.level == 9 {
            return 39
        }
        if self.level == 10 {
            return 42
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
            if status.sp() >= 15 { return Ok(15) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 21 { return Ok(21) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 24 { return Ok(24) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 27 { return Ok(27) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 33 { return Ok(33) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 39 { return Ok(39) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 7522, name_english: "Ice_Stone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 7522 && item.amount >= 1) {
            return Err(UseSkillFailure::NeedItem);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       3000
    }

    #[inline(always)]
    fn has_bonuses_to_self(&self) -> bool {
        true
    }

    #[inline(always)]
    fn bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {
        if self.level == 1 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(0), 0, tick, 15000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 15000, 538),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-3), 0, tick, 20000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 20000, 538),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-3), 0, tick, 25000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 25000, 538),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-3), 0, tick, 30000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 30000, 538),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-5), 0, tick, 35000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 35000, 538),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-5), 0, tick, 40000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 40000, 538),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-5), 0, tick, 45000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 45000, 538),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-8), 0, tick, 50000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 50000, 538),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-8), 0, tick, 55000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 55000, 538),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::Agi(-8), 0, tick, 60000, 538),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-50), 0, tick, 60000, 538),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Water
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
}
