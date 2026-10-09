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

// DC_DONTFORGETME - Slow Grace
pub struct SlowGrace {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for SlowGrace {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        328
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
            return 28
        }
        if self.level == 2 {
            return 31
        }
        if self.level == 3 {
            return 34
        }
        if self.level == 4 {
            return 37
        }
        if self.level == 5 {
            return 40
        }
        if self.level == 6 {
            return 43
        }
        if self.level == 7 {
            return 46
        }
        if self.level == 8 {
            return 49
        }
        if self.level == 9 {
            return 52
        }
        if self.level == 10 {
            return 55
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
            if status.sp() >= 28 { return Ok(28) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 31 { return Ok(31) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 37 { return Ok(37) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 43 { return Ok(43) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 49 { return Ok(49) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 55 { return Ok(55) } else {return Err(())}
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
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-8.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-8), 0, tick, 180000, 328),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-11.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-11), 0, tick, 180000, 328),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-14.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-14), 0, tick, 180000, 328),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-17.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-17), 0, tick, 180000, 328),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-20.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-20), 0, tick, 180000, 328),]);
        }
        if self.level == 6 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-23.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-23), 0, tick, 180000, 328),]);
        }
        if self.level == 7 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-26.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-26), 0, tick, 180000, 328),]);
        }
        if self.level == 8 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-29.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-29), 0, tick, 180000, 328),]);
        }
        if self.level == 9 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-32.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-32), 0, tick, 180000, 328),]);
        }
        if self.level == 10 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::AspdPercentage(-35.0), 0, tick, 180000, 328),
                TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(-35), 0, tick, 180000, 328),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::Performance(PerformanceProfile { status: models::status_change::StatusChangeKind::DontForgetMe, reach: PerformanceReach::Enemies, ensemble: false, effect: PerformanceEffect::Aura, lesson: models::enums::skill_enums::SkillEnum::DcDancinglesson })
    }

    #[inline(always)]
    fn performance_values(&self, level: i32, stats: &StatusSnapshot, lesson: i32) -> (i32, i32) {
        ((5 + 3 * level + i32::from(stats.dex()) / 10 + lesson) * 10, 5 + 3 * level + i32::from(stats.agi()) / 10 + lesson)
    }
}
