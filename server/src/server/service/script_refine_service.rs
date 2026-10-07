//! Script commands of the refiners and the repairmen: `getequip*` queries, `successrefitem`, `failedrefitem`,
//! `downrefitem`, `getbrokenid`, `repair` and `repairall`.

use models::enums::effect::Effect;
use models::enums::item::ItemType;
use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use packets::packets::{Packet, PacketZcNotifyEffect};
use script_sdk::{Function, Reply, Value};

use crate::repository::model::item_model::ItemModel;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::CharacterEquipItem;
use crate::server::model::refine::{self, MAX_REFINE, RefineCostType};
use crate::server::script::ScriptRequest;
use crate::server::script::item_script_handler::equipment_slot;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

const REFINE_ACK_PACKET: u16 = 0x0188;
const MAKING_ITEM_ACK_PACKET: u16 = 0x018f;
const REFINE_SUCCESS: u16 = 0;
const REFINE_FAILURE: u16 = 1;
const REFINE_DOWNGRADE: u16 = 2;
const REFINE_MATERIAL_ID: i32 = 0;
const REFINE_ZENY_COST: i32 = 1;
/// Refining a weapon you forged to +10 earns fame (`blacksmith_fame_refine_threshold`, `fame_refine_lv1` to `lv3`).
const FORGED_CARD: i16 = 255;
const FAME_PER_WEAPON_LEVEL: [i32; 3] = [1, 25, 1000];

pub(crate) fn handles(function: Function) -> bool {
    matches!(
        function,
        Function::GetEquipName
            | Function::GetEquipWeaponLevel
            | Function::GetEquipArmorLevel
            | Function::GetEquipCardId
            | Function::GetEquipIsEquipped
            | Function::GetEquipIsEnableRefine
            | Function::GetEquipPercentRefinery
            | Function::GetEquipRefineCost
            | Function::SuccessRefineItem
            | Function::FailedRefineItem
            | Function::DownRefineItem
            | Function::GetBrokenId
            | Function::Repair
            | Function::RepairAll
    )
}

fn equipped_index(character: &Character, slot: &Value) -> Result<Option<usize>, String> {
    let location = equipment_slot(&slot.text())?.as_flag();
    Ok(character.inventory.iter().position(|item| item.as_ref().is_some_and(|item| item.equip as u64 & location != 0)))
}

fn broken_indexes(character: &Character) -> Vec<usize> {
    character.inventory.iter().enumerate().filter(|(_, item)| item.as_ref().is_some_and(|item| item.is_damaged)).map(|(index, _)| index).collect()
}

/// Index in `broken_indexes` of a 1-based position given by a script.
fn broken_position(position: i32) -> Option<usize> {
    usize::try_from(position).ok().and_then(|position| position.checked_sub(1))
}

impl Server {
    pub(crate) fn script_refine_call(&self, state: &mut ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let mut character = state.characters_mut().remove(&context.char_id).ok_or("Script target character disconnected")?;
        let result = self.refine_command(&mut character, function, arguments);
        state.insert_character(character);
        result
    }

    fn refine_command(&self, character: &mut Character, function: Function, arguments: &[Value]) -> Reply {
        let configuration = GlobalConfigService::instance();
        let argument = |index: usize| arguments.get(index).ok_or_else(|| "Missing argument".to_string());
        let number = |index: usize| argument(index)?.number_value();
        match function {
            Function::GetBrokenId => {
                let broken = broken_indexes(character);
                let item = broken_position(number(0)?).and_then(|position| broken.get(position)).and_then(|index| character.inventory[*index].as_ref());
                return Ok(item.map_or(0, |item| item.item_id).into());
            }
            Function::Repair => {
                if let Some(index) = broken_position(number(0)?).and_then(|position| broken_indexes(character).get(position).copied()) {
                    self.repair_items(character, &[index])?;
                }
                return Ok(Value::default());
            }
            Function::RepairAll => {
                let broken = broken_indexes(character);
                self.repair_items(character, &broken)?;
                return Ok(Value::default());
            }
            _ => {}
        }
        let index = equipped_index(character, argument(0)?)?;
        let item = index.and_then(|index| character.inventory[index].as_ref());
        let model: Option<&ItemModel> = item.map(|item| configuration.get_item(item.item_id));
        match function {
            Function::GetEquipIsEquipped => Ok(i32::from(item.is_some()).into()),
            Function::GetEquipName => Ok(model.map_or_else(String::new, |model| model.name_english.clone()).into()),
            Function::GetEquipWeaponLevel => {
                Ok(model.filter(|model| model.item_type == ItemType::Weapon).map_or(0, |model| i32::from(model.weapon_level.unwrap_or(0))).into())
            }
            Function::GetEquipArmorLevel => {
                Ok(model.filter(|model| model.item_type == ItemType::Armor).map_or(0, |model| i32::from(model.armor_level.unwrap_or(0))).into())
            }
            Function::GetEquipCardId => {
                let slot = number(1)?;
                Ok(item.filter(|_| (0..4).contains(&slot)).map_or(-1, |item| i32::from([item.card0, item.card1, item.card2, item.card3][slot as usize] as u16)).into())
            }
            Function::GetEquipIsEnableRefine => Ok(i32::from(model.is_some_and(refine::is_refineable)).into()),
            Function::GetEquipPercentRefinery => {
                let enriched = arguments.get(1).map(Value::number_value).transpose()?.unwrap_or(0) != 0;
                let kind = if enriched { RefineCostType::Enriched } else { RefineCostType::Normal };
                let cost = item.zip(model).and_then(|(item, model)| refine::refine_cost(model, item.refine, kind));
                Ok(cost.map_or(0, |cost| i32::from(cost.rate) / 100).into())
            }
            Function::GetEquipRefineCost => {
                let kind = RefineCostType::from_number(number(1)?);
                let cost = kind.zip(item).zip(model).and_then(|((kind, item), model)| refine::refine_cost(model, item.refine, kind));
                Ok(match (cost, number(2)?) {
                    (Some(cost), REFINE_MATERIAL_ID) => cost.material,
                    (Some(cost), REFINE_ZENY_COST) => i32::try_from(cost.price).unwrap_or(i32::MAX),
                    _ => -1,
                }
                .into())
            }
            Function::SuccessRefineItem | Function::DownRefineItem => {
                let index = index.ok_or("No item equipped at this slot")?;
                let steps = arguments.get(1).map(Value::number_value).transpose()?.unwrap_or(1);
                let success = function == Function::SuccessRefineItem;
                let refined = self.change_refine(character, index, if success { steps } else { -steps }, if success { REFINE_SUCCESS } else { REFINE_DOWNGRADE })?;
                if success {
                    self.reward_forging_fame(character, index);
                }
                Ok(i32::from(refined).into())
            }
            Function::FailedRefineItem => {
                let index = index.ok_or("No item equipped at this slot")?;
                self.break_equipment(character, index)?;
                Ok(1.into())
            }
            _ => Err("Not a refine command".into()),
        }
    }

    /// Unequips, changes the refine level, shows the item again and puts it back on, like rathena.
    pub(crate) fn change_refine(&self, character: &mut Character, index: usize, delta: i32, ack: u16) -> Result<i16, String> {
        let item = character.inventory[index].clone().ok_or("Item disappeared")?;
        if delta > 0 && item.refine >= MAX_REFINE {
            return Ok(MAX_REFINE);
        }
        let refine = (i32::from(item.refine) + delta).clamp(0, i32::from(MAX_REFINE)) as i16;
        let location = item.equip as u64;
        if location != 0 {
            self.inventory_service().takeoff_equip_item(character, index).ok_or("The item cannot be taken off now")?;
        }
        self.runtime()
            .block_on(self.repository.character_set_item_condition(character.char_id, item.clone(), refine, item.is_damaged))
            .map_err(|error| error.to_string())?;
        let refined = character.inventory[index].as_mut().ok_or("Item disappeared")?;
        refined.refine = refine;
        let refined = refined.clone();
        self.send_refine_ack(character, index, ack, refine);
        self.item_service().notify_removed(character.char_id, index, 1);
        self.item_service().notify_grant(character.char_id, index, &refined, 1);
        if location != 0 {
            self.inventory_service().equip_item(character, CharacterEquipItem { char_id: character.char_id, index, requested_location: Some(location) });
        }
        self.notify_refine_effect(character, if ack == REFINE_SUCCESS { Effect::RefineSuccess } else { Effect::RefineFailure });
        Ok(refine)
    }

    /// A failed refine destroys the equipment, cards included.
    pub(crate) fn break_equipment(&self, character: &mut Character, index: usize) -> Result<(), String> {
        if character.inventory[index].as_ref().is_some_and(|item| item.equip != 0) {
            self.inventory_service().takeoff_equip_item(character, index).ok_or("The item cannot be taken off now")?;
        }
        self.send_refine_ack(character, index, REFINE_FAILURE, 0);
        self.inventory_service().remove_single_item_from_inventory(self.runtime(), index, character, true)?;
        self.notify_refine_effect(character, Effect::RefineFailure);
        Ok(())
    }

    pub(crate) fn clear_damage(&self, character: &mut Character, index: usize) -> Result<i32, String> {
        let item = character.inventory[index].clone().ok_or("Item disappeared")?;
        self.runtime()
            .block_on(self.repository.character_set_item_condition(character.char_id, item.clone(), item.refine, false))
            .map_err(|error| error.to_string())?;
        character.inventory[index].as_mut().ok_or("Item disappeared")?.is_damaged = false;
        Ok(item.item_id)
    }

    fn repair_items(&self, character: &mut Character, indexes: &[usize]) -> Result<(), String> {
        for &index in indexes {
            let item_id = self.clear_damage(character, index)?;
            let mut packet = MAKING_ITEM_ACK_PACKET.to_le_bytes().to_vec();
            packet.extend_from_slice(&0_u16.to_le_bytes());
            packet.extend_from_slice(&(item_id as u16).to_le_bytes());
            self.send_to_character(character.char_id, packet);
        }
        if !indexes.is_empty() {
            self.notify_refine_effect(character, Effect::RefineSuccess);
            for packet in self.inventory_service().inventory_packets(character) {
                self.send_to_character(character.char_id, packet);
            }
        }
        Ok(())
    }

    pub(crate) fn reward_forging_fame(&self, character: &mut Character, index: usize) {
        let Some(item) = character.inventory[index].as_ref() else { return };
        let model = GlobalConfigService::instance().get_item(item.item_id);
        let creator = u32::from(item.card2 as u16) | u32::from(item.card3 as u16) << 16;
        if model.item_type != ItemType::Weapon || item.refine != MAX_REFINE || item.card0 != FORGED_CARD || creator != character.char_id {
            return;
        }
        let fame = usize::try_from(model.weapon_level.unwrap_or(0) - 1).ok().and_then(|level| FAME_PER_WEAPON_LEVEL.get(level)).copied();
        if let Some(fame) = fame {
            let _ = crate::server::service::script_character_service::add_fame(self, character, &[Value::Number(fame)]);
        }
    }

    fn send_refine_ack(&self, character: &Character, index: usize, result: u16, refine: i16) {
        let mut packet = REFINE_ACK_PACKET.to_le_bytes().to_vec();
        packet.extend_from_slice(&result.to_le_bytes());
        packet.extend_from_slice(&u16::try_from(index + 2).unwrap_or_default().to_le_bytes());
        packet.extend_from_slice(&(refine as u16).to_le_bytes());
        self.send_to_character(character.char_id, packet);
    }

    pub(crate) fn notify_refine_effect(&self, character: &Character, effect: Effect) {
        let mut packet = PacketZcNotifyEffect::new(GlobalConfigService::instance().packetver());
        packet.set_aid(character.char_id);
        packet.set_effect_id(effect.value() as i32);
        packet.fill_raw();
        self.character_service().send_area_notification_around_characters(character, packet.raw);
    }

    pub(crate) fn send_to_character(&self, char_id: u32, packet: Vec<u8>) {
        let _ = self.server_service().notification_sender().send(Notification::Char(CharNotification::new(char_id, packet)));
    }
}
