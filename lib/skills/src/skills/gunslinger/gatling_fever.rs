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

// GS_GATLINGFEVER - Gatling Fever
pub struct GatlingFever {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for GatlingFever {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        517
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
            return 30
        }
        if self.level == 2 {
            return 32
        }
        if self.level == 3 {
            return 34
        }
        if self.level == 4 {
            return 36
        }
        if self.level == 5 {
            return 38
        }
        if self.level == 6 {
            return 40
        }
        if self.level == 7 {
            return 42
        }
        if self.level == 8 {
            return 44
        }
        if self.level == 9 {
            return 46
        }
        if self.level == 10 {
            return 48
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
        true
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 32 { return Ok(32) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 36 { return Ok(36) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 48 { return Ok(48) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {
        if let Some(character_weapon) = status.right_hand_weapon() {
            if 524288 & character_weapon.weapon_type().as_flag() > 0 { Ok(()) } else { Err(()) }
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
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(2.0), 0, tick, 30000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(30), 0, tick, 30000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-5), 0, tick, 30000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 30000, 517),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(4.0), 0, tick, 45000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(40), 0, tick, 45000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-10), 0, tick, 45000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 45000, 517),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(6.0), 0, tick, 60000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(50), 0, tick, 60000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-15), 0, tick, 60000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 60000, 517),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(8.0), 0, tick, 75000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(60), 0, tick, 75000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-20), 0, tick, 75000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 75000, 517),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(10.0), 0, tick, 90000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(70), 0, tick, 90000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-25), 0, tick, 90000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 90000, 517),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(12.0), 0, tick, 105000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(80), 0, tick, 105000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-30), 0, tick, 105000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 105000, 517),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(14.0), 0, tick, 120000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(90), 0, tick, 120000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-35), 0, tick, 120000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 120000, 517),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(16.0), 0, tick, 135000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(100), 0, tick, 135000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-40), 0, tick, 135000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 135000, 517),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(18.0), 0, tick, 150000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(110), 0, tick, 150000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-45), 0, tick, 150000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 150000, 517),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(20.0), 0, tick, 165000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Atk(120), 0, tick, 165000, 517),
                TemporaryStatusBonus::with_duration(BonusType::Flee(-50), 0, tick, 165000, 517),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-100), 0, tick, 165000, 517),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
       1
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Ammo
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _hit: &HitContext) -> Vec<StatusInfliction> {
        vec![]
    }
}
