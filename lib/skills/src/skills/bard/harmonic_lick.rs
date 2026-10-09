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

// BD_RINGNIBELUNGEN - Harmonic Lick
pub struct HarmonicLick {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for HarmonicLick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        310
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
        5
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
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(AllWeaponType, 75), 0, tick, 60000, 310),]);
        }
        if self.level == 2 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(AllWeaponType, 100), 0, tick, 60000, 310),]);
        }
        if self.level == 3 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(AllWeaponType, 125), 0, tick, 60000, 310),]);
        }
        if self.level == 4 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(AllWeaponType, -106), 0, tick, 60000, 310),]);
        }
        if self.level == 5 {
            return TemporaryStatusBonuses::new(vec![
                TemporaryStatusBonus::with_duration(BonusType::MasteryDamageUsingWeaponType(AllWeaponType, -81), 0, tick, 60000, 310),]);
        }
        TemporaryStatusBonuses::default()
    }

    #[inline(always)]
    fn actor_behaviour(&self) -> ActorBehaviour {
        ActorBehaviour::Performance(PerformanceProfile { status: models::status_change::StatusChangeKind::Nibelungen, reach: PerformanceReach::Party, ensemble: true, effect: PerformanceEffect::Aura, lesson: models::enums::skill_enums::SkillEnum::BaMusicallesson })
    }

    #[inline(always)]
    fn performance_values(&self, level: i32, stats: &StatusSnapshot, lesson: i32) -> (i32, i32) {
        ((level + 2) * 25, 0)
    }
}
