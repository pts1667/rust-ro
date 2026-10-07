use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use models::enums::bonus::BonusType;
use models::enums::class::JobName;
use models::enums::item::{EquipmentLocation, ItemGroup};
use models::enums::weapon::WeaponType;
use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use models::status::Status;
use script_runtime::Host;
use script_sdk::{Function, Reply, Request, Value};

use super::bonus::BonusScriptHandler;
use super::constant::load_constant;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemEffect {
    GuildStorageOpen(crate::server::model::game_systems::GuildStorageOpenReceipt),
    PoolDraw(super::game_data::PoolDrawReceipt),
    Heal {
        hp: i32,
        sp: i32,
        percentage: bool,
        item_scaling: bool,
        item_id: u32,
    },
    Call {
        function: Function,
        arguments: Vec<Value>,
    },
    Write {
        name: String,
        value: Value,
    },
    Grant {
        item_id: i32,
        amount: i16,
        identified: bool,
    },
}

#[derive(Debug, Clone, Default)]
pub struct ItemScriptContext {
    pub char_id: u32,
    pub name: String,
    pub map: String,
    pub party_name: String,
    pub guild_name: String,
    pub inventory: HashMap<i32, i32>,
    pub queries: HashMap<String, Value>,
}

pub struct ItemScriptHost {
    pub status: Status,
    pub item_id: u32,
    pub bonuses: BonusScriptHandler,
    pub effects: Vec<ItemEffect>,
    pub error: Option<String>,
    pub variables: HashMap<String, Value>,
    pub context: ItemScriptContext,
    effects_allowed: bool,
    pool_repository: Option<Arc<dyn crate::repository::Repository>>,
    pools: HashMap<(i32, i32), super::game_data::ItemPoolState>,
    guild_storage_context: Option<GuildStorageScriptContext>,
}

struct GuildStorageScriptContext {
    char_id: u32,
    revision: u64,
    original_personal_open: bool,
    original_guild_open: Option<u32>,
    personal_open_in_script: bool,
    guild_opened_in_script: bool,
}

pub fn constant(name: &str) -> Reply {
    super::unit_data::constant(name)
        .or_else(|| crate::server::service::script_presentation_service::presentation_constant(name))
        .or_else(|| super::utilities::constant(name))
        .or_else(|| super::game_data::constant(name))
        .or_else(|| crate::server::service::script_world_service::pet_constant(name))
        .or_else(|| load_constant(&name.to_string()))
        .or_else(|| crate::server::service::script_npc_commands::rathena_constant(name))
        .or_else(|| match name {
            "RC_undead" => load_constant(&"RC_Undead".into()),
            "RC_Demihuman" => load_constant(&"RC_DemiHuman".into()),
            "IG_Food" => Some(Value::Number(ItemGroup::Food.value() as i32)),
            "W_1HSWORD" => Some(Value::Number(WeaponType::Sword1H.value() as i32)),
            "W_2HSWORD" => Some(Value::Number(WeaponType::Sword2H.value() as i32)),
            "W_1HAXE" => Some(Value::Number(WeaponType::Axe1H.value() as i32)),
            "W_2HAXE" => Some(Value::Number(WeaponType::Axe2H.value() as i32)),
            "W_MACE" => Some(Value::Number(WeaponType::Mace.value() as i32)),
            "W_STAFF" => Some(Value::Number(WeaponType::Staff.value() as i32)),
            "W_BOW" => Some(Value::Number(WeaponType::Bow.value() as i32)),
            _ => None,
        })
        .or_else(|| {
            if name.starts_with('b')
                || name.starts_with("SC_")
                || name.starts_with("DT_")
                || name.starts_with("EQI_")
                || name.starts_with("ITEMINFO_")
                || name.starts_with("EF_")
            {
                Some(Value::String(name.to_string()))
            } else {
                None
            }
        })
        .ok_or_else(|| format!("Unknown script constant {name}"))
}

pub fn status_variable(status: &Status, name: &str) -> Option<Value> {
    let value = match name.to_ascii_lowercase().as_str() {
        "zeny" => status.zeny,
        "class" => status.job,
        "upper" => match status.job {
            4001..=4022 => 1,
            4023..=4045 => 2,
            _ => 0,
        },
        "baselevel" => status.base_level,
        "joblevel" => status.job_level,
        "skillpoint" => status.skill_point,
        "hp" => status.hp,
        "sp" => status.sp,
        "maxhp" => status.max_hp,
        "maxsp" => status.max_sp,
        "sex" => u32::from(status.is_male),
        "baseclass" => JobName::from_mask(
            JobName::from_value(status.job as usize).mask() & models::enums::class::JOB_BASE_MASK,
            status.is_male,
        )
        .map_or(0, |job| job.value() as u32),
        "basejob" => JobName::from_mask(
            JobName::from_value(status.job as usize).mask() & models::enums::class::JOB_UPPER_MASK,
            status.is_male,
        )
        .map_or(0, |job| job.value() as u32),
        _ => return None,
    };
    Some(Value::Number(value as i32))
}

impl ItemScriptHost {
    pub fn bonuses(status: Status, item_id: u32) -> Self {
        Self::new(status, item_id, false)
    }

    pub fn consumable(status: Status, item_id: u32) -> Self {
        Self::new(status, item_id, true)
    }

    fn new(status: Status, item_id: u32, effects_allowed: bool) -> Self {
        let context = status
            .script_context
            .as_deref()
            .map(|snapshot| {
                use crate::server::model::game_systems::PlayerOption;
                let options = snapshot.options;
                let mut queries = HashMap::new();
                for (function, present) in [
                    (
                        Function::CheckCart,
                        options
                            & (PlayerOption::Cart1.as_flag()
                                | PlayerOption::Cart2.as_flag()
                                | PlayerOption::Cart3.as_flag()
                                | PlayerOption::Cart4.as_flag()
                                | PlayerOption::Cart5.as_flag())
                            != 0,
                    ),
                    (Function::CheckRiding, options & PlayerOption::Riding.as_flag() != 0),
                    (Function::CheckFalcon, options & PlayerOption::Falcon.as_flag() != 0),
                    (Function::CheckMadogear, options & PlayerOption::Madogear.as_flag() != 0),
                    (Function::IsMounting, snapshot.mounting),
                ] {
                    queries.insert(format!("{function:?}"), i32::from(present).into());
                }
                queries.insert("GetPartnerId".into(), (snapshot.partner_id as i32).into());
                let now = chrono::Utc::now().timestamp().max(0) as u64;
                let expires = snapshot.vip_expires_at;
                queries.insert("VipStatus".into(), i32::from(expires > now).into());
                queries.insert("VipStatus:1".into(), i32::from(expires > now).into());
                queries.insert(
                    "VipStatus:2".into(),
                    (if expires > now { expires.min(i32::MAX as u64) } else { 0 } as i32).into(),
                );
                queries.insert(
                    "VipStatus:3".into(),
                    (expires.saturating_sub(now).min(i32::MAX as u64) as i32).into(),
                );
                ItemScriptContext {
                    char_id: snapshot.char_id,
                    name: snapshot.name.clone(),
                    map: snapshot.map.clone(),
                    party_name: snapshot.party_name.clone(),
                    guild_name: snapshot.guild_name.clone(),
                    inventory: snapshot.inventory.clone(),
                    queries,
                }
            })
            .unwrap_or_default();
        let variables = status
            .script_context
            .as_deref()
            .map(|snapshot| {
                HashMap::from([
                    ("Weight".into(), (snapshot.weight as i32).into()),
                    ("Karma".into(), snapshot.karma.into()),
                    ("Manner".into(), snapshot.manner.into()),
                ])
            })
            .unwrap_or_default();
        Self {
            status,
            item_id,
            bonuses: BonusScriptHandler::new(),
            effects: vec![],
            error: None,
            variables,
            context,
            effects_allowed,
            pool_repository: None,
            pools: HashMap::new(),
            guild_storage_context: None,
        }
    }

    pub fn with_pool_repository(mut self, repository: Arc<dyn crate::repository::Repository>) -> Self {
        self.pool_repository = Some(repository);
        self
    }

    pub fn with_guild_storage_context(mut self, char_id: u32, systems: &crate::server::model::game_systems::CharacterGameSystems) -> Self {
        self.guild_storage_context = Some(GuildStorageScriptContext {
            char_id,
            revision: systems.revision,
            original_personal_open: systems.storage_open,
            original_guild_open: systems.guild_storage_open,
            personal_open_in_script: false,
            guild_opened_in_script: false,
        });
        self
    }

    fn draw_group_entry(
        &mut self,
        group: &super::game_data::ItemGroup,
        subgroup_id: i32,
        rng: &mut fastrand::Rng,
    ) -> Result<Option<super::game_data::ItemGroupEntry>, String> {
        let Some(subgroup) = group.subgroups.iter().find(|subgroup| subgroup.id == subgroup_id) else {
            return Ok(None);
        };
        let key = (group.id, subgroup_id);
        let state = if subgroup.algorithm == "SharedPool" {
            if let Some(state) = self.pools.get(&key) {
                state.clone()
            } else {
                self.pool_repository
                    .as_ref()
                    .ok_or("Shared item pool requires a transactional repository")?
                    .item_group_pool(group.id, subgroup_id)
                    .map_err(|error| error.to_string())?
            }
        } else {
            super::game_data::ItemPoolState::default()
        };
        let Some((entry, receipt)) = super::game_data::stage_group_entry(group, subgroup_id, &state, rng) else {
            return Ok(None);
        };
        if let Some(receipt) = receipt {
            self.pools.insert(key, receipt.after.clone());
            self.effects.push(ItemEffect::PoolDraw(receipt));
        }
        Ok(Some(entry))
    }

    fn call(&mut self, function: Function, arguments: Vec<Value>) -> Reply {
        let number = |index: usize| arguments.get(index).ok_or_else(|| "Missing argument".to_string())?.number_value();
        match function {
            Function::GuildOpenStorage if self.effects_allowed => {
                let context = self
                    .guild_storage_context
                    .as_mut()
                    .ok_or("Guild storage requires a live character context")?;
                if context.guild_opened_in_script {
                    return Ok(Value::Number(2));
                }
                let receipt = self
                    .pool_repository
                    .as_ref()
                    .ok_or("Guild storage requires a transactional repository")?
                    .prepare_guild_storage_open(
                        context.char_id,
                        context.revision,
                        context.original_personal_open,
                        context.original_guild_open,
                        context.personal_open_in_script,
                    )
                    .map_err(|error| error.to_string())?;
                let code = receipt.result_code;
                context.guild_opened_in_script = code == 0;
                self.effects.push(ItemEffect::GuildStorageOpen(receipt));
                Ok(Value::Number(i32::from(code)))
            }
            Function::OpenStorage if self.effects_allowed => {
                if let Some(context) = self.guild_storage_context.as_mut() {
                    if context.original_guild_open.is_some() || context.guild_opened_in_script {
                        return Ok(Value::Number(1));
                    }
                    context.personal_open_in_script = true;
                }
                self.effects.push(ItemEffect::Call { function, arguments });
                Ok(Value::default())
            }
            Function::GetEquipRefineryCnt | Function::GetEquipId => {
                let slot = arguments.first().ok_or("Missing equipment slot")?.text();
                let location = match slot.as_str() {
                    "EQI_HAND_R" => EquipmentLocation::HandRight,
                    "EQI_HAND_L" => EquipmentLocation::HandLeft,
                    "EQI_GARMENT" => EquipmentLocation::Garment,
                    "EQI_ARMOR" => EquipmentLocation::Armor,
                    "EQI_SHOES" => EquipmentLocation::Shoes,
                    "EQI_HEAD_TOP" => EquipmentLocation::HeadTop,
                    "EQI_HEAD_MID" => EquipmentLocation::HeadMid,
                    "EQI_HEAD_LOW" => EquipmentLocation::HeadLow,
                    "EQI_ACC_L" => EquipmentLocation::AccessoryLeft,
                    "EQI_ACC_R" => EquipmentLocation::AccessoryRight,
                    _ => return Err(format!("Unknown equipment slot {slot}")),
                }
                .as_flag();
                let weapon = self
                    .status
                    .weapons
                    .iter()
                    .find(|item| item.location & location != 0)
                    .map(|item| (item.item_id, item.refine));
                let gear = self
                    .status
                    .equipments
                    .iter()
                    .find(|item| item.location & location != 0)
                    .map(|item| (item.item_id, item.refine));
                let (id, refine) = weapon.or(gear).unwrap_or((-1, 0));
                Ok(Value::Number(if function == Function::GetEquipId {
                    id
                } else {
                    i32::from(refine)
                }))
            }
            Function::GetItemInfo => {
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                super::utilities::item_information(
                    super::utilities::find_item(configuration, arguments.first().ok_or("Missing item")?),
                    arguments.get(1).ok_or("Missing item field")?,
                    configuration,
                )
            }
            Function::GetItemName => {
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                let item = super::utilities::find_item(configuration, arguments.first().ok_or("Missing item")?);
                Ok(item.map_or_else(|| "null".to_string(), |item| item.name_english.clone()).into())
            }
            Function::GetPetInfo => {
                let pet = self.status.script_context.as_ref().and_then(|context| {
                    let target = arguments.get(1).and_then(|value| value.number_value().ok()).unwrap_or(context.char_id as i32);
                    (target == context.char_id as i32).then_some(context.pet.as_ref()).flatten()
                });
                Ok(crate::server::service::script_world_service::pet_information(pet, number(0)?))
            }
            Function::Bonus | Function::Bonus2 | Function::Bonus3 | Function::Bonus4 | Function::Bonus5 | Function::Skill => {
                self.bonuses.apply(function, arguments)?;
                Ok(Value::default())
            }
            Function::AutoBonus | Function::AutoBonus2 | Function::AutoBonus3 => {
                self.bonuses.register_auto_bonus(function, &arguments, self.item_id)?;
                Ok(Value::default())
            }
            Function::PetAutoBonus | Function::PetAutoBonus2 | Function::PetAutoBonus3 => {
                let pet = self.status.script_context.as_ref().and_then(|context| context.pet.as_ref())
                    .filter(|pet| pet.intimacy > 0).ok_or("Pet automatic bonuses require an active pet")?;
                if self.effects_allowed {
                    self.effects.push(ItemEffect::Call { function, arguments });
                } else {
                    self.bonuses.register_pet_auto_bonus(function, &arguments, pet.id, pet.class_id)?;
                }
                Ok(Value::default())
            }
            Function::GetRefine => Ok(Value::Number(
                self.status
                    .weapons
                    .iter()
                    .find(|item| {
                        item.item_id == self.item_id as i32
                            || !models::item::special_card_metadata(item.card0)
                                && [item.card0, item.card1, item.card2, item.card3]
                                    .iter()
                                    .any(|card| i32::from(*card) == self.item_id as i32)
                    })
                    .map(|item| item.refine as i32)
                    .or_else(|| {
                        self.status
                            .equipments
                            .iter()
                            .find(|item| {
                                item.item_id == self.item_id as i32
                                    || !models::item::special_card_metadata(item.card0)
                                        && [item.card0, item.card1, item.card2, item.card3]
                                            .iter()
                                            .any(|card| i32::from(*card) == self.item_id as i32)
                            })
                            .map(|item| item.refine as i32)
                    })
                    .unwrap_or_default(),
            )),
            Function::ReadParam => {
                let name = arguments.first().ok_or("Missing stat")?.text().to_ascii_lowercase();
                let value = match name.as_str() {
                    "bstr" => self.status.str,
                    "bagi" => self.status.agi,
                    "bvit" => self.status.vit,
                    "bint" => self.status.int,
                    "bdex" => self.status.dex,
                    "bluk" => self.status.luk,
                    _ => return Err(format!("Unknown stat {name}")),
                };
                Ok(Value::Number(value as i32))
            }
            Function::GetSkillLv => {
                let wanted = arguments.first().ok_or("Missing skill")?;
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                let id = match wanted {
                    Value::Number(id) if *id > 0 => *id as u32,
                    _ => configuration.find_skill_config(wanted).map_or(0, |skill| skill.id),
                };
                Ok(Value::Number(i32::from(self.effective_skill_level(id, configuration))))
            }
            Function::IsEquipped => {
                let equipped = self.status.all_equipped_items();
                Ok(Value::Number(i32::from(arguments.iter().all(|value| {
                    value
                        .number_value()
                        .is_ok_and(|id| equipped.iter().any(|item| item.item_id() == id))
                }))))
            }
            Function::Rand => {
                let (minimum, maximum) = if arguments.len() == 1 {
                    (0, number(0)?.checked_sub(1).ok_or("Invalid random range")?)
                } else {
                    (number(0)?, number(1)?)
                };
                if minimum > maximum {
                    return Err("Invalid random range".into());
                }
                Ok(Value::Number(fastrand::i32(minimum..=maximum)))
            }
            Function::Min | Function::Max => {
                let values: Vec<i32> = arguments.iter().map(Value::number_value).collect::<Result<_, _>>()?;
                Ok(Value::Number(
                    if function == Function::Min {
                        values.into_iter().min()
                    } else {
                        values.into_iter().max()
                    }
                    .ok_or("Missing arguments")?,
                ))
            }
            Function::Pow => {
                let exponent = u32::try_from(number(1)?).map_err(|_| "Negative exponent")?;
                Ok(Value::Number(number(0)?.checked_pow(exponent).ok_or("Exponent overflow")?))
            }
            Function::GetTime => Ok(super::utilities::get_time(arguments.first().ok_or("Missing time field")?)),
            Function::GetCharacterId => {
                let kind = number(0)?;
                if let Some(name) = arguments.get(1) {
                    let name = name.string_value()?;
                    if *name != self.context.name {
                        return Ok(self.context.queries.get(&format!("GetCharacterId:{name}:{kind}")).cloned().unwrap_or_default());
                    }
                }
                let Some(character) = self.status.script_context.as_deref() else { return Ok(0.into()); };
                Ok(Value::Number(match kind {
                    0 => character.char_id,
                    1 => character.party_id,
                    2 => character.guild_id,
                    3 => character.account_id,
                    _ => 0,
                } as i32))
            }
            Function::StrCharInfo => Ok(match number(0)? {
                0 => self.context.name.clone().into(),
                1 => self.context.party_name.clone().into(),
                2 => self.context.guild_name.clone().into(),
                3 => self.context.map.clone().into(),
                _ => Value::String(String::new()),
            }),
            Function::VipStatus
            | Function::GetPartnerId
            | Function::CheckMadogear
            | Function::CheckCart
            | Function::CheckRiding
            | Function::CheckFalcon
            | Function::IsMounting => {
                let key = format!("{function:?}:{}", arguments.first().map(Value::text).unwrap_or_default());
                self.context
                    .queries
                    .get(&key)
                    .cloned()
                    .or_else(|| self.context.queries.get(&format!("{function:?}")).cloned())
                    .ok_or_else(|| format!("No character state is available for {function:?}"))
            }
            Function::CountItem => {
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                let Some(item) = super::utilities::find_item(configuration, arguments.first().ok_or("Missing item")?) else {
                    return Ok(0.into());
                };
                Ok(self.context.inventory.get(&item.id).copied().unwrap_or_default().into())
            }
            Function::GetItem if self.effects_allowed => {
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                let item = super::utilities::find_item(configuration, arguments.first().ok_or("Missing item")?).ok_or("Unknown item")?;
                self.grant(item.id, number(1)?, true)
            }
            Function::GetNamedItem | Function::GetItem2 if self.effects_allowed => {
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                let item = super::utilities::find_item(configuration, arguments.first().ok_or("Missing item")?).ok_or("Unknown item")?;
                let amount = if function == Function::GetItem2 { number(1)? } else { 1 };
                let amount_i16 = i16::try_from(amount).ok().filter(|amount| *amount > 0).ok_or("Item grant must be positive")?;
                let count = self.context.inventory.entry(item.id).or_default();
                *count = count.checked_add(i32::from(amount_i16)).ok_or("Inventory count is out of bounds")?;
                self.effects.push(ItemEffect::Call { function, arguments });
                Ok(Value::default())
            }
            Function::DelItem if self.effects_allowed => {
                let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
                let item = super::utilities::find_item(configuration, arguments.first().ok_or("Missing item")?).ok_or("Unknown item")?;
                let amount = number(1)?;
                let count = self.context.inventory.entry(item.id).or_default();
                if amount <= 0 || *count < amount {
                    return Err("Not enough items to delete".into());
                }
                *count -= amount;
                self.effects.push(ItemEffect::Call {
                    function,
                    arguments: vec![item.id.into(), amount.into()],
                });
                Ok(Value::default())
            }
            Function::RandomGroupItem | Function::GetGroupItem if self.effects_allowed => {
                let group = super::game_data::group(arguments.first().ok_or("Missing item group")?).ok_or("Unknown item group")?;
                let mut rng = fastrand::Rng::new();
                if function == Function::RandomGroupItem {
                    let subgroup = arguments.get(2).map(Value::number_value).transpose()?.unwrap_or(1);
                    let entry = self
                        .draw_group_entry(group, subgroup, &mut rng)?
                        .ok_or("Item group produced no reward")?;
                    let requested = arguments.get(1).map(Value::number_value).transpose()?.unwrap_or(0);
                    let amount = if requested == 0 { i32::from(entry.amount) } else { requested };
                    let forced_identified = arguments.get(3).map(Value::number_value).transpose()?.unwrap_or(0) != 0;
                    let item = crate::server::service::global_config_service::GlobalConfigService::instance()
                        .find_item(entry.item_id)
                        .ok_or("Unknown reward item")?;
                    self.grant(entry.item_id, amount, forced_identified || !item.item_type.is_equipment())?;
                } else {
                    let identified = arguments.get(1).map(Value::number_value).transpose()?.unwrap_or(0) != 0;
                    for subgroup in &group.subgroups {
                        let entries = if subgroup.algorithm == "All" {
                            subgroup.entries.clone()
                        } else {
                            self.draw_group_entry(group, subgroup.id, &mut rng)?.into_iter().collect()
                        };
                        for entry in entries {
                            let item = crate::server::service::global_config_service::GlobalConfigService::instance()
                                .find_item(entry.item_id)
                                .ok_or("Unknown reward item")?;
                            self.grant(
                                entry.item_id,
                                i32::from(entry.amount),
                                identified || !item.item_type.is_equipment(),
                            )?;
                        }
                    }
                }
                Ok(Value::default())
            }
            Function::ItemHeal | Function::Heal | Function::PercentHeal if self.effects_allowed => {
                let hp = number(0)?;
                let sp = number(1)?;
                self.effects.push(ItemEffect::Heal {
                    hp,
                    sp,
                    percentage: function == Function::PercentHeal,
                    item_scaling: function == Function::ItemHeal,
                    item_id: self.item_id,
                });
                Ok(Value::default())
            }
            Function::Pet if self.effects_allowed => {
                let arguments = if arguments.is_empty() {
                    if self.item_id == 0 { return Err("Pet capture requires a lure outside an item script".into()); }
                    vec![Value::Number(self.item_id as i32)]
                } else { arguments };
                self.effects.push(ItemEffect::Call { function, arguments });
                Ok(Value::default())
            }
            Function::PetSkillBonus | Function::PetRecovery | Function::PetSkillSupport | Function::PetSkillAttack | Function::PetSkillAttack2 | Function::PetLoot if self.effects_allowed => {
                self.effects.push(ItemEffect::Call { function, arguments });
                Ok(Value::default())
            }
            Function::ItemSkill
            | Function::Warp
            | Function::SpecialEffect
            | Function::SkillEffect
            | Function::StartStatus
            | Function::StartStatus2
            | Function::StartStatus4
            | Function::EndStatus
            | Function::Monster
            | Function::Produce
            | Function::Cooking
            | Function::BirthPet
            | Function::GuildExperience
            | Function::MercenaryCreate
            | Function::MercenaryStartStatus
            | Function::MercenaryHeal
            | Function::SetFont
            | Function::SearchStores
            | Function::Homevolution
            | Function::Announce
            | Function::BuyingStore
            | Function::GetExperience
            | Function::ResetSkills
            | Function::PartyWarp
            | Function::UnitSkill
            | Function::UnitSkillToId
            | Function::UnitSkillToPosition
                if self.effects_allowed =>
            {
                self.effects.push(ItemEffect::Call { function, arguments });
                Ok(Value::default())
            }
            _ => Err(format!(
                "Game operation {function:?} is not implemented for this script context"
            )),
        }
    }

    fn grant(&mut self, item_id: i32, amount: i32, identified: bool) -> Reply {
        let amount = i16::try_from(amount).map_err(|_| "Item grant amount is out of bounds")?;
        if amount <= 0 {
            return Err("Item grant must be positive".into());
        }
        let count = self.context.inventory.entry(item_id).or_default();
        *count = count.checked_add(i32::from(amount)).ok_or("Inventory count is out of bounds")?;
        self.effects.push(ItemEffect::Grant {
            item_id,
            amount,
            identified,
        });
        Ok(Value::default())
    }

    fn effective_skill_level(
        &self,
        skill_id: u32,
        configuration: &crate::server::service::global_config_service::GlobalConfigService,
    ) -> u8 {
        if skill_id == 0 {
            return 0;
        }
        let mut level = self
            .status
            .known_skills
            .iter()
            .find(|skill| skill.value.id() == skill_id)
            .map_or(0, |skill| skill.level);
        if let Some(grant) = self.status.script_skill_grants.get(&skill_id) {
            level = level.max(grant.current_level);
        }
        level = level.max(
            crate::server::service::script_character_service::taekwon_rank_skill_grants(&self.status)
                .iter()
                .filter(|grant| grant.value.id() == skill_id)
                .map(|grant| grant.level)
                .max()
                .unwrap_or(0),
        );
        let grant_level = |bonus: &BonusType| match bonus {
            BonusType::EnableSkillId(id, level) if *id == skill_id => Some(*level),
            _ => None,
        };
        let cached = self.status.equipment_bonuses.iter().filter_map(|bonus| grant_level(bonus.bonus()));
        let timed = self.status.temporary_bonuses.iter().filter_map(|bonus| grant_level(bonus.bonus()));
        let auto = self
            .status
            .active_auto_bonuses
            .iter()
            .flat_map(|bonus| bonus.bonuses.iter())
            .filter_map(grant_level);
        let current = self.bonuses.bonuses.read().unwrap();
        level = level.max(
            cached
                .chain(timed)
                .chain(auto)
                .chain(current.iter().filter_map(grant_level))
                .max()
                .unwrap_or(0),
        );
        let mut items = Vec::new();
        for weapon in &self.status.weapons {
            items.push(weapon.item_id);
            if !models::item::special_card_metadata(weapon.card0) {
                items.extend([weapon.card0, weapon.card1, weapon.card2, weapon.card3].map(i32::from));
            }
        }
        for gear in &self.status.equipments {
            items.push(gear.item_id);
            if !models::item::special_card_metadata(gear.card0) {
                items.extend([gear.card0, gear.card1, gear.card2, gear.card3].map(i32::from));
            }
        }
        if let Some(ammo) = &self.status.ammo {
            items.push(ammo.item_id);
        }
        let static_level = items
            .into_iter()
            .filter_map(|id| configuration.find_item(id))
            .filter(|item| !item.item_bonuses_are_dynamic)
            .flat_map(|item| item.bonuses.iter())
            .filter_map(grant_level)
            .max()
            .unwrap_or(0);
        level.max(static_level)
    }
}

#[async_trait]
impl Host for ItemScriptHost {
    async fn invoke(&mut self, request: Request) -> Reply {
        match request {
            Request::Constant(name) => constant(&name),
            Request::Read(name) => Ok(self
                .variables
                .get(&name)
                .cloned()
                .or_else(|| status_variable(&self.status, &name))
                .unwrap_or_else(|| {
                    if name.ends_with('$') {
                        Value::String(String::new())
                    } else {
                        Value::default()
                    }
                })),
            Request::Write { name, value } if self.effects_allowed => {
                self.variables.insert(name.clone(), value.clone());
                self.effects.push(ItemEffect::Write { name, value });
                Ok(Value::default())
            }
            Request::Call { function, arguments } => self.call(function, arguments),
            Request::ReportError(error) => {
                self.error = Some(error);
                Ok(Value::default())
            }
            _ => Err("Operation is unavailable in an item script".into()),
        }
    }
}

#[cfg(test)]
mod equipment_query_tests {
    use models::enums::skill_enums::SkillEnum;
    use models::status::KnownSkill;
    use models::status_bonus::{StatusBonus, StatusBonuses};

    use super::*;

    #[test]
    fn refine_queries_follow_each_real_card_slot_but_never_creator_metadata() {
        let mut status = Status::default();
        status.equipments.push(models::item::WearGear {
            item_id: 2229,
            level: 1,
            location: EquipmentLocation::Armor.as_flag(),
            refine: 7,
            card0: 4001,
            card1: 4002,
            card2: 4003,
            card3: 4004,
            def: 1,
            inventory_index: 0,
        });
        for id in [2229, 4001, 4002, 4003, 4004] {
            let mut host = ItemScriptHost::bonuses(status.clone(), id);
            assert_eq!(host.call(Function::GetRefine, vec![]).unwrap(), Value::Number(7));
        }
        status.equipments[0].card0 = 254;
        for id in [254, 4002, 4003, 4004] {
            let mut host = ItemScriptHost::bonuses(status.clone(), id);
            assert_eq!(host.call(Function::GetRefine, vec![]).unwrap(), Value::Number(0));
        }
        status.weapons.push(models::item::WearWeapon {
            item_id: 1201,
            attack: 30,
            level: 1,
            weapon_type: WeaponType::Dagger,
            location: EquipmentLocation::HandRight.as_flag(),
            refine: 9,
            element: models::enums::element::Element::Fire,
            card0: 255,
            card1: 4001,
            card2: 4002,
            card3: 4003,
            ranked_forged: false,
            inventory_index: 1,
            range: 1,
        });
        let mut host = ItemScriptHost::bonuses(status.clone(), 1201);
        assert_eq!(host.call(Function::GetRefine, vec![]).unwrap(), Value::Number(9));
        let mut host = ItemScriptHost::bonuses(status, 4001);
        assert_eq!(host.call(Function::GetRefine, vec![]).unwrap(), Value::Number(0));
    }

    #[test]
    fn skill_queries_use_session_and_equipment_grants_without_reentering_status_calculation() {
        crate::tests::common::before_all();
        let mut status = Status::default();
        status.known_skills.push(KnownSkill {
            value: SkillEnum::MgFirebolt,
            level: 2,
        });
        status
            .script_skill_grants
            .insert(SkillEnum::MgFirebolt.id(), models::skill_grant::ScriptSkillGrant {
                learned_level: 2,
                current_level: 5,
            });
        let mut host = ItemScriptHost::bonuses(status, 1201);
        assert_eq!(
            host.call(Function::GetSkillLv, vec![(SkillEnum::MgFirebolt.id() as i32).into()])
                .unwrap(),
            Value::Number(5)
        );
        host.status.equipment_bonuses = StatusBonuses::new(vec![StatusBonus::new(BonusType::EnableSkillId(SkillEnum::MgFirebolt.id(), 7))]);
        assert_eq!(
            host.call(Function::GetSkillLv, vec![(SkillEnum::MgFirebolt.id() as i32).into()])
                .unwrap(),
            Value::Number(7)
        );
        host.call(Function::Skill, vec![(SkillEnum::MgFirebolt.id() as i32).into(), 9.into()])
            .unwrap();
        assert_eq!(
            host.call(Function::GetSkillLv, vec![Value::String("MG_FIREBOLT".into())]).unwrap(),
            Value::Number(9)
        );
        assert_eq!(host.bonuses.drain(), vec![BonusType::EnableSkillId(
            SkillEnum::MgFirebolt.id(),
            9
        )]);
    }
}
