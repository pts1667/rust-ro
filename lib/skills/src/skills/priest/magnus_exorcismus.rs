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

// PR_MAGNUS - Magnus Exorcismus
pub struct MagnusExorcismus {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}

impl Skill for MagnusExorcismus {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }

    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn id(&self) -> u32 {
        79
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
            return 40
        }
        if self.level == 2 {
            return 42
        }
        if self.level == 3 {
            return 44
        }
        if self.level == 4 {
            return 46
        }
        if self.level == 5 {
            return 48
        }
        if self.level == 6 {
            return 50
        }
        if self.level == 7 {
            return 52
        }
        if self.level == 8 {
            return 54
        }
        if self.level == 9 {
            return 56
        }
        if self.level == 10 {
            return 58
        }
        0
    }

    fn target_type(&self) -> SkillTargetType {
        SkillTargetType::Ground
    }

    fn is_magic(&self) -> bool {
        true
    }

    fn is_physical(&self) -> bool {
        false
    }

    #[inline(always)]
    fn validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp() >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp() >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp() >= 44 { return Ok(44) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp() >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp() >= 48 { return Ok(48) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp() >= 50 { return Ok(50) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp() >= 52 { return Ok(52) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp() >= 54 { return Ok(54) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp() >= 56 { return Ok(56) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp() >= 58 { return Ok(58) } else {return Err(())}
        }
        Err(())
    }

    #[inline(always)]
    fn validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if !inventory.iter().any(|item| item.item_id == 717 && item.amount >= 1) {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }

    #[inline(always)]
    fn base_cast_time(&self) -> u32 {
       15000
    }

    #[inline(always)]
    fn base_after_cast_act_delay(&self) -> u32 {
       4000
    }

    #[inline(always)]
    fn hit_count(&self) -> i8 {
        if self.level == 1 {
            return 1
        }
        if self.level == 2 {
            return 2
        }
        if self.level == 3 {
            return 3
        }
        if self.level == 4 {
            return 4
        }
        if self.level == 5 {
            return 5
        }
        if self.level == 6 {
            return 6
        }
        if self.level == 7 {
            return 7
        }
        if self.level == 8 {
            return 8
        }
        if self.level == 9 {
            return 9
        }
        if self.level == 10 {
            return 10
        }
        0
    }

    #[inline(always)]
    fn dmg_matk(&self) -> Option<f32> {
        if self.level == 1 {
            return Some(1.000)
        }
        if self.level == 2 {
            return Some(2.000)
        }
        if self.level == 3 {
            return Some(3.000)
        }
        if self.level == 4 {
            return Some(4.000)
        }
        if self.level == 5 {
            return Some(5.000)
        }
        if self.level == 6 {
            return Some(6.000)
        }
        if self.level == 7 {
            return Some(7.000)
        }
        if self.level == 8 {
            return Some(8.000)
        }
        if self.level == 9 {
            return Some(9.000)
        }
        if self.level == 10 {
            return Some(10.000)
        }
        None
    }

    #[inline(always)]
    fn element(&self) -> Element {
        Element::Holy
    }

    #[inline(always)]
    fn inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {
        vec![]
    }
}
