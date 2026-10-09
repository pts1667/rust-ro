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

// BA_ASSASSINCROSS - Impressive Riff
pub struct ImpressiveRiff {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for ImpressiveRiff {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        320
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
            return 38
        }
        if self.level == 2 {
            return 41
        }
        if self.level == 3 {
            return 44
        }
        if self.level == 4 {
            return 47
        }
        if self.level == 5 {
            return 50
        }
        if self.level == 6 {
            return 53
        }
        if self.level == 7 {
            return 56
        }
        if self.level == 8 {
            return 59
        }
        if self.level == 9 {
            return 62
        }
        if self.level == 10 {
            return 65
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
            if status.sp() >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 41 { return Ok(41) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 47 { return Ok(47) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 53 { return Ok(53) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 56 { return Ok(56) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 59 { return Ok(59) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 62 { return Ok(62) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 65 { return Ok(65) } else {return Err(())}
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
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(6.0), 0, tick, 120000, 320),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(7.0), 0, tick, 120000, 320),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(8.0), 0, tick, 120000, 320),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(9.0), 0, tick, 120000, 320),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(10.0), 0, tick, 120000, 320),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(11.0), 0, tick, 120000, 320),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(12.0), 0, tick, 120000, 320),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(13.0), 0, tick, 120000, 320),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(14.0), 0, tick, 120000, 320),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(15.0), 0, tick, 120000, 320),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::Performance(PerformanceProfile { status: models::status_change::StatusChangeKind::AssnCros, reach: PerformanceReach::Everyone, ensemble: false, effect: PerformanceEffect::Aura, lesson: models::enums::skill_enums::SkillEnum::BaMusicallesson })
    }

    #[inline(always)]
    fn performance_values(&self, level: i32, stats: &StatusSnapshot, lesson: i32) -> (i32, i32) {
        ((lesson / 2 + 5 + level + i32::from(stats.agi()) / 20) * 10, 0)
    }
}
