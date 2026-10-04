use std::collections::BTreeMap;

use models::enums::bonus::BonusType;
use models::enums::item::EquipmentLocation;
use models::enums::weapon::{AmmoType, WeaponType};
use models::enums::{EnumWithMaskValueU64, EnumWithStringValue};
use models::status_change::StatusChangeKind;

use super::ScriptSkillService;
use super::metadata::SkillMetadata;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::game_systems::PlayerOption;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SkillRequirementPlan {
    pub minimum_hp: u32,
    pub maximum_hp_percent: Option<u8>,
    pub allow_hp_death: bool,
    pub hp: u32,
    pub sp: u32,
    pub zeny: u32,
    pub spirit_spheres: u8,
    pub removals: Vec<SkillItemRemoval>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SkillItemRemoval {
    pub index: usize,
    pub item: InventoryItemModel,
    pub amount: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeferredSkillPayment {
    pub skill_id: u32,
    pub level: u8,
    pub requirements: SkillRequirementPlan,
    pub source_index: Option<usize>,
    pub source_item: Option<(i32, i32, i64)>,
}

impl ScriptSkillService {
    pub fn partition_skill_requirements(
        skill_id: u32,
        level: u8,
        mut upfront: SkillRequirementPlan,
    ) -> (SkillRequirementPlan, SkillRequirementPlan) {
        let mut deferred = SkillRequirementPlan::default();
        if skill_id == models::enums::skill_enums::SkillEnum::CgTarotcard.id() {
            deferred.sp = std::mem::take(&mut upfront.sp);
        }
        if skill_id == models::enums::skill_enums::SkillEnum::MgStonecurse.id() && level > 5 {
            deferred.removals = std::mem::take(&mut upfront.removals);
        }
        (upfront, deferred)
    }

    pub fn requirements_plan(&self, character: &Character, skill_id: u32, level: u8, tick: u128) -> Result<SkillRequirementPlan, String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Pre-renewal requirements are unavailable")?;
        let Some(requirements) = metadata.requires.as_ref() else {
            return Ok(SkillRequirementPlan::default());
        };
        if metadata.flags.get("Toggleable").copied().unwrap_or(false)
            && metadata
                .status
                .as_deref()
                .and_then(StatusChangeKind::from_name)
                .is_some_and(|kind| character.status.has_status_change(kind))
        {
            return Ok(SkillRequirementPlan::default());
        }
        let amount = |field: &str| {
            requirements
                .get(field)
                .and_then(|value| SkillMetadata::json_level_value(value, level, "Amount"))
                .unwrap_or(0)
        };
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let hp_rate = amount("HpRateCost");
        let required_hp = (u64::try_from(amount("HpCost").max(0)).unwrap_or(0)
            + if hp_rate > 0 {
                u64::from(character.status.hp) * hp_rate as u64 / 100
            } else {
                u64::from(snapshot.max_hp()) * u64::from(hp_rate.unsigned_abs()) / 100
            })
        .min(u64::from(u32::MAX)) as u32;
        let minimum_hp = required_hp.saturating_add(1);
        let hp = if matches!(metadata.name.as_str(), "SM_MAGNUM" | "MS_MAGNUM") {
            0
        } else {
            required_hp
        };
        let maximum_hp_percent = (amount("MaxHpTrigger") > 0).then(|| amount("MaxHpTrigger").min(100) as u8);
        let sp_rate = amount("SpRateCost");
        let sp = amount("SpCost").max(0) as u32
            + (if sp_rate > 0 {
                character.status.sp as u64 * sp_rate as u64 / 100
            } else {
                snapshot.max_sp() as u64 * sp_rate.unsigned_abs() as u64 / 100
            })
            .min(u32::MAX as u64) as u32;
        let sp_modifier = snapshot
            .bonuses_raw()
            .iter()
            .filter_map(|bonus| {
                if let BonusType::SpConsumption(amount) = bonus {
                    Some(*amount as i32)
                } else {
                    None
                }
            })
            .sum::<i32>();
        let sp = (sp as u64 * (100 + sp_modifier).max(0) as u64 / 100).min(u32::MAX as u64) as u32;
        let spheres = amount("SpiritSphereCost");
        let sphere_count = character
            .script_skill_state
            .spirit_spheres
            .iter()
            .filter(|expiry| **expiry > tick)
            .count();
        let spheres = if spheres < 0 {
            sphere_count.min(u8::MAX as usize) as u8
        } else {
            spheres.min(u8::MAX as i32) as u8
        };
        let mut plan = SkillRequirementPlan {
            minimum_hp,
            maximum_hp_percent,
            allow_hp_death: false,
            hp,
            sp,
            zeny: amount("ZenyCost").max(0) as u32,
            spirit_spheres: spheres,
            removals: vec![],
        };
        if character.status.hp < minimum_hp {
            return Err("Not enough HP for this skill".into());
        }
        if maximum_hp_percent
            .is_some_and(|maximum| u64::from(character.status.hp) * 100 / u64::from(snapshot.max_hp().max(1)) > u64::from(maximum))
        {
            return Err("Current HP exceeds this skill's upper threshold".into());
        }
        if character.status.sp < sp {
            return Err("Not enough SP for this skill".into());
        }
        if character.status.zeny < plan.zeny {
            return Err("Not enough zeny for this skill".into());
        }
        if sphere_count < spheres as usize {
            return Err("Not enough spirit spheres or coins".into());
        }
        if let Some(weapons) = requirements.get("Weapon").and_then(|value| value.as_object()) {
            if !weapons.iter().any(|(name, enabled)| {
                enabled.as_bool() == Some(true)
                    && WeaponType::try_from_string_ignore_case(name).is_ok_and(|weapon| weapon == *snapshot.right_hand_weapon_type())
            }) {
                return Err("This skill requires a different weapon".into());
            }
        }
        if let Some(statuses) = requirements.get("Status").and_then(|value| value.as_object()) {
            for (name, required) in statuses {
                if required.as_bool() == Some(true)
                    && !StatusChangeKind::from_name(name).is_some_and(|kind| character.status.has_status_change(kind))
                {
                    return Err(format!("This skill requires SC_{}", name.to_ascii_uppercase()));
                }
            }
        }
        match requirements.get("State").and_then(|value| value.as_str()) {
            Some("Shield")
                if !character
                    .status
                    .equipments
                    .iter()
                    .any(|equipment| equipment.location & EquipmentLocation::HandLeft.as_flag() != 0) =>
            {
                return Err("This skill requires a shield".into());
            }
            Some("Cart")
                if character.options
                    & (PlayerOption::Cart1.as_flag()
                        | PlayerOption::Cart2.as_flag()
                        | PlayerOption::Cart3.as_flag()
                        | PlayerOption::Cart4.as_flag()
                        | PlayerOption::Cart5.as_flag())
                    == 0 =>
            {
                return Err("This skill requires a cart".into());
            }
            Some("Move_Enable") if character.status.blocks_movement() => return Err("This skill requires movement".into()),
            Some("Recover_Weight_Rate")
                if character.weight()
                    >= (self.configuration.get_job_config(character.status.job).base_weight() + character.status.str as u32 * 300) / 2 =>
            {
                return Err("This skill requires less than half carrying capacity".into());
            }
            _ => {}
        }
        let mut removal_amounts = BTreeMap::<usize, u16>::new();
        if let Some(allowed) = requirements.get("Ammo").and_then(|value| value.as_object()) {
            let ammo = character.status.ammo.ok_or("Required ammunition is missing")?;
            if !allowed.iter().any(|(name, enabled)| {
                enabled.as_bool() == Some(true) && AmmoType::try_from_string_ignore_case(name).is_ok_and(|kind| kind == ammo.ammo_type)
            }) {
                return Err("Wrong ammunition type".into());
            }
            let amount = amount("AmmoAmount").max(1).min(u16::MAX as i32) as u16;
            let item = character
                .get_item_from_inventory(ammo.inventory_index)
                .ok_or("Equipped ammunition is missing")?;
            if (item.amount as i32) < amount as i32 {
                return Err("Not enough ammunition".into());
            }
            removal_amounts.insert(ammo.inventory_index, amount);
        }
        let no_gemstone = snapshot
            .bonuses_raw()
            .iter()
            .any(|bonus| matches!(bonus, BonusType::EnableNoGemstoneRequired));
        if metadata.name != "AM_CALLHOMUN" {
            for requirement in requirements
                .get("ItemCost")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
            {
                if requirement
                    .get("Level")
                    .and_then(|value| value.as_u64())
                    .is_some_and(|required| required != level as u64)
                {
                    continue;
                }
                let name = requirement
                    .get("Item")
                    .and_then(|value| value.as_str())
                    .ok_or("Invalid skill item requirement")?;
                if no_gemstone && matches!(name, "Blue_Gemstone" | "Red_Gemstone" | "Yellow_Gemstone") {
                    continue;
                }
                let item = self
                    .configuration
                    .find_item_by_name(name)
                    .ok_or_else(|| format!("Skill reagent {} is unavailable", name))?;
                let mut needed = requirement
                    .get("Amount")
                    .and_then(|value| value.as_u64())
                    .unwrap_or(1)
                    .min(u16::MAX as u64) as u16;
                for (index, inventory) in character
                    .inventory
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| item.as_ref().map(|item| (index, item)))
                    .filter(|(_, inventory)| inventory.item_id == item.id && inventory.equip == 0)
                {
                    let available = (inventory.amount.max(0) as u16).saturating_sub(removal_amounts.get(&index).copied().unwrap_or(0));
                    let removed = available.min(needed);
                    if removed > 0 {
                        *removal_amounts.entry(index).or_default() += removed;
                        needed -= removed;
                    }
                    if needed == 0 {
                        break;
                    }
                }
                if needed != 0 {
                    return Err(format!("Required reagent {} is missing", name));
                }
            }
        }
        for (index, amount) in removal_amounts {
            plan.removals.push(SkillItemRemoval {
                index,
                item: character.inventory[index].as_ref().unwrap().clone(),
                amount,
            });
        }
        Ok(plan)
    }

    pub fn apply_consumed_spheres(&self, character: &mut Character, amount: u8, tick: u128) {
        character.script_skill_state.expire_spheres(tick);
        let amount = (amount as usize).min(character.script_skill_state.spirit_spheres.len());
        character.script_skill_state.spirit_spheres.drain(0..amount);
        character.status.spirit_sphere_count = character.script_skill_state.spirit_spheres.len().min(u8::MAX as usize) as u8;
        self.notify_spheres(character);
    }
}
