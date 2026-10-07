//! Skills that open a client menu and act on the choice: Arrow Crafting, Weapon Refine and Repair Weapon.

use models::enums::cell::CellType;
use models::enums::item::ItemType;
use models::enums::skill_enums::SkillEnum;
use models::enums::EnumWithMaskValueU16;

use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemGrant};
use crate::server::model::refine::{self, MAX_REFINE, RefineCostType};
use crate::server::script::game_data::data;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;

const ARROW_LIST_PACKET: u16 = 0x01ad;
const REPAIR_LIST_PACKET: u16 = 0x01fc;
const REPAIR_ACK_PACKET: u16 = 0x01fe;
const WEAPON_LIST_PACKET: u16 = 0x0221;
const AUTO_SPELL_LIST_PACKET: u16 = 0x01cd;
const WEAPON_REFINE_MESSAGE_PACKET: u16 = 0x0223;
const MENU_LIFETIME_MS: u128 = 120_000;
const MENU_CANCELLED: u16 = 0xffff;
/// Weapon refine entries and messages use the inventory index shifted past the two reserved slots.
const CLIENT_INDEX_OFFSET: u32 = 2;

const MESSAGE_UPGRADED: u32 = 0;
const MESSAGE_DESTROYED: u32 = 1;
const MESSAGE_SKILL_TOO_LOW: u32 = 2;
const MESSAGE_MISSING_ORE: u32 = 3;
const REPAIR_SUCCESS: u8 = 0;

const IRON_ORE: i32 = 1002;
const IRON: i32 = 998;
const STEEL: i32 = 999;
const ORIDECON_STONE: i32 = 756;

#[derive(Debug, Clone, PartialEq)]
pub enum SkillMenuKind {
    MakingArrow,
    ElementalConverter,
    WeaponRefine,
    AutoSpell,
    RepairWeapon { target: u32 },
}

/// What the character may pick, until it answers or the menu expires.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillMenu {
    pub kind: SkillMenuKind,
    pub skill_id: u32,
    pub level: u8,
    pub expires_at: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillMenuChoice {
    Arrow(u16),
    WeaponRefine(u32),
    Repair(u16),
    AutoSpell(u32),
}

pub fn is_menu_skill(name: &str) -> bool {
    matches!(name, "AC_MAKINGARROW" | "WS_WEAPONREFINE" | "BS_REPAIRWEAPON" | "SA_AUTOSPELL") || is_crafting_skill(name)
}

/// Skills that make an item from the recipes of `produce_db.txt`.
pub fn is_crafting_skill(name: &str) -> bool {
    matches!(name, "AM_PHARMACY" | "SA_CREATECON" | "AL_HOLYWATER" | "ASC_CDP")
}

const ELEMENTAL_CONVERTER_LEVEL: u16 = 23;
const POTION_LEVEL: u16 = 22;
const HOLY_WATER: i32 = 523;
const POISON_BOTTLE: i32 = 678;

fn has_item(character: &Character, item_id: i32) -> bool {
    character.inventory.iter().flatten().any(|item| item.item_id == item_id && item.amount > 0)
}

fn push_entry(packet: &mut Vec<u8>, index: u16, item: &InventoryItemModel) {
    packet.extend_from_slice(&index.to_le_bytes());
    packet.extend_from_slice(&(item.item_id as u16).to_le_bytes());
    packet.push(item.refine as u8);
    for card in [item.card0, item.card1, item.card2, item.card3] {
        packet.extend_from_slice(&(card as u16).to_le_bytes());
    }
}

fn list_packet(id: u16, entries: &[u8]) -> Result<Vec<u8>, String> {
    let length = u16::try_from(4 + entries.len()).map_err(|_| "Menu is too large")?;
    let mut packet = id.to_le_bytes().to_vec();
    packet.extend_from_slice(&length.to_le_bytes());
    packet.extend_from_slice(entries);
    Ok(packet)
}

fn arrow_sources(character: &Character) -> Vec<i32> {
    let mut sources: Vec<i32> = vec![];
    for arrow in &data().arrows {
        let owned = character.inventory.iter().flatten().any(|item| item.item_id == arrow.source && item.amount > 0 && item.equip == 0 && item.is_identified);
        if owned && !sources.contains(&arrow.source) {
            sources.push(arrow.source);
        }
    }
    sources
}

fn weapon_ore(character: &Character, item: &InventoryItemModel, model: &ItemModel, skill_level: u8) -> Option<i32> {
    let usable = model.item_type == ItemType::Weapon && item.is_identified && item.equip == 0 && item.refine < i16::from(skill_level);
    let cost = usable.then(|| refine::refine_cost(model, item.refine, RefineCostType::Normal)).flatten()?;
    has_item(character, cost.material).then_some(cost.material)
}

fn refinable_weapons(character: &Character, skill_level: u8) -> Vec<usize> {
    let configuration = GlobalConfigService::instance();
    character
        .inventory
        .iter()
        .enumerate()
        .filter_map(|(index, item)| {
            let item = item.as_ref()?;
            weapon_ore(character, item, configuration.find_item(item.item_id)?, skill_level).map(|_| index)
        })
        .collect()
}

fn repair_material(model: &ItemModel) -> Option<i32> {
    match model.item_type {
        ItemType::Weapon => match model.weapon_level? {
            1 => Some(IRON_ORE),
            2 => Some(IRON),
            3 => Some(STEEL),
            4 => Some(ORIDECON_STONE),
            _ => None,
        },
        ItemType::Armor => (model.armor_level? == 1).then_some(STEEL),
        _ => None,
    }
}

/// Skills the Auto Spell menu offers, in the order of the client list, with the Auto Spell level above which each is available.
const AUTO_SPELLS: [(SkillEnum, u8); 7] = [
    (SkillEnum::MgNapalmbeat, 0),
    (SkillEnum::MgColdbolt, 1),
    (SkillEnum::MgFirebolt, 1),
    (SkillEnum::MgLightningbolt, 1),
    (SkillEnum::MgSoulstrike, 4),
    (SkillEnum::MgFireball, 7),
    (SkillEnum::MgFrostdiver, 9),
];

fn learned_level(character: &Character, skill: SkillEnum) -> u8 {
    StatusService::instance().to_snapshot(&character.status).known_skills().iter().find(|known| known.value == skill).map_or(0, |known| known.level)
}

fn auto_spell_choices(character: &Character, auto_spell_level: u8) -> Vec<SkillEnum> {
    AUTO_SPELLS.iter().filter(|(skill, required)| auto_spell_level > *required && learned_level(character, *skill) > 0).map(|(skill, _)| *skill).collect()
}

fn broken_items(character: &Character) -> Vec<usize> {
    character.inventory.iter().enumerate().filter(|(_, item)| item.as_ref().is_some_and(|item| item.is_damaged)).map(|(index, _)| index).collect()
}

impl Server {
    pub(crate) fn open_skill_menu(
        &self,
        state: &ServerState,
        character: &mut Character,
        skill_name: &str,
        skill_id: u32,
        level: u8,
        target_id: u32,
        tick: u128,
    ) -> Result<(), String> {
        self.script_skill_service().requirements_plan(character, skill_id, level, tick)?;
        if matches!(skill_name, "AM_PHARMACY" | "AL_HOLYWATER" | "ASC_CDP") {
            return self.start_crafting_skill(state, character, skill_name, skill_id, level, tick);
        }
        let (kind, packet) = match skill_name {
            "SA_CREATECON" => {
                let entries: Vec<u8> = self.item_service().makeable_items(character, ELEMENTAL_CONVERTER_LEVEL).iter().flat_map(|id| (*id as u16).to_le_bytes()).collect();
                if entries.is_empty() {
                    return Err("No elemental converter can be made".into());
                }
                (SkillMenuKind::ElementalConverter, list_packet(ARROW_LIST_PACKET, &entries)?)
            }
            "AC_MAKINGARROW" => {
                let entries: Vec<u8> = arrow_sources(character).iter().flat_map(|id| (*id as u16).to_le_bytes()).collect();
                if entries.is_empty() {
                    return Err("Nothing to make arrows from".into());
                }
                (SkillMenuKind::MakingArrow, list_packet(ARROW_LIST_PACKET, &entries)?)
            }
            "WS_WEAPONREFINE" => {
                let mut entries = vec![];
                for index in refinable_weapons(character, level) {
                    let client_index = index as u32 + CLIENT_INDEX_OFFSET;
                    push_entry(&mut entries, u16::try_from(client_index).map_err(|_| "Inventory index is out of range")?, character.inventory[index].as_ref().ok_or("Item disappeared")?);
                }
                if entries.is_empty() {
                    return Err("No weapon can be refined".into());
                }
                (SkillMenuKind::WeaponRefine, list_packet(WEAPON_LIST_PACKET, &entries)?)
            }
            "SA_AUTOSPELL" => {
                let spells = auto_spell_choices(character, level);
                if spells.is_empty() {
                    return Err("No skill can be turned into an Auto Spell".into());
                }
                let mut packet = AUTO_SPELL_LIST_PACKET.to_le_bytes().to_vec();
                for slot in AUTO_SPELLS.iter().map(|(skill, _)| *skill) {
                    packet.extend_from_slice(&if spells.contains(&slot) { slot.id() } else { 0 }.to_le_bytes());
                }
                (SkillMenuKind::AutoSpell, packet)
            }
            "BS_REPAIRWEAPON" => {
                let target = self.repair_target(state, character, skill_id, level, target_id)?;
                let mut entries = vec![];
                for index in broken_items(target) {
                    push_entry(&mut entries, u16::try_from(index).map_err(|_| "Inventory index is out of range")?, target.inventory[index].as_ref().ok_or("Item disappeared")?);
                }
                if entries.is_empty() {
                    return Err("Nothing to repair".into());
                }
                (SkillMenuKind::RepairWeapon { target: target_id }, list_packet(REPAIR_LIST_PACKET, &entries)?)
            }
            _ => return Err(format!("{skill_name} does not open a menu")),
        };
        character.pending_skill_menu = Some(SkillMenu { kind, skill_id, level, expires_at: tick + MENU_LIFETIME_MS });
        self.send_to_character(character.char_id, packet);
        Ok(())
    }

    /// Pharmacy opens its window, Aqua Benedicta and Create Deadly Poison make their one item at once.
    fn start_crafting_skill(&self, state: &ServerState, character: &mut Character, skill_name: &str, skill_id: u32, level: u8, tick: u128) -> Result<(), String> {
        let item_service = self.item_service();
        match skill_name {
            "AM_PHARMACY" => {
                item_service.open_crafting_window(character, false, POTION_LEVEL, Some(skill_id))?;
                if let Err(error) = item_service.pay_skill_requirements(self, character, skill_id, level, tick, true, None) {
                    character.pending_craft = None;
                    return Err(error);
                }
                Ok(())
            }
            "AL_HOLYWATER" => {
                let water = state
                    .get_map_instance_from_character(character)
                    .is_some_and(|instance| instance.state().cells().get(character.y as usize * instance.x_size() as usize + character.x as usize).is_some_and(|cell| cell & CellType::Water.as_flag() != 0));
                if !water {
                    return Err("Aqua Benedicta requires standing in water".into());
                }
                item_service.pay_skill_requirements(self, character, skill_id, level, tick, true, None)?;
                item_service.make_with_skill(self, character, HOLY_WATER, POTION_LEVEL, tick).map(|_| ())
            }
            _ => {
                item_service.pay_skill_requirements(self, character, skill_id, level, tick, true, None)?;
                if !item_service.make_with_skill(self, character, POISON_BOTTLE, POTION_LEVEL, tick)? {
                    let max_hp = StatusService::instance().to_snapshot(&character.status).max_hp();
                    let hp = character.status.hp.saturating_sub(max_hp / 4).max(1);
                    self.character_service().update_hp_sp(character, hp, character.status.sp);
                }
                Ok(())
            }
        }
    }

    fn repair_target<'a>(&self, state: &'a ServerState, character: &'a Character, skill_id: u32, level: u8, target_id: u32) -> Result<&'a Character, String> {
        if target_id == character.char_id {
            return Ok(character);
        }
        let target = state.get_character(target_id).ok_or("Repair target is not available")?;
        if target.map_instance_key != character.map_instance_key {
            return Err("Repair target is on another map".into());
        }
        let range = self.script_skill_service().player_skill_range(&StatusService::instance().to_snapshot(&character.status), skill_id, level).max(1);
        if character.x.abs_diff(target.x).max(character.y.abs_diff(target.y)) > range {
            return Err("Repair target is out of range".into());
        }
        Ok(target)
    }

    pub(crate) fn choose_in_skill_menu(&self, state: &mut ServerState, char_id: u32, choice: SkillMenuChoice, tick: u128) -> Result<(), String> {
        let mut character = state.characters_mut().remove(&char_id).ok_or("Menu owner disconnected")?;
        let result = self.apply_skill_menu_choice(state, &mut character, choice, tick);
        character.pending_skill_menu = None;
        state.insert_character(character);
        result
    }

    fn apply_skill_menu_choice(&self, state: &mut ServerState, character: &mut Character, choice: SkillMenuChoice, tick: u128) -> Result<(), String> {
        let menu = character.pending_skill_menu.clone().ok_or("No skill menu is open")?;
        if menu.expires_at <= tick || character.is_dead() {
            return Err("The skill menu is no longer active".into());
        }
        if character.game_systems.is_trading() {
            return Err("Skill menus cannot be used while trading".into());
        }
        match (&menu.kind, choice) {
            (SkillMenuKind::MakingArrow, SkillMenuChoice::Arrow(item_id)) if item_id != MENU_CANCELLED => self.make_arrows(character, &menu, i32::from(item_id), tick),
            (SkillMenuKind::ElementalConverter, SkillMenuChoice::Arrow(item_id)) if item_id != MENU_CANCELLED => {
                if !self.item_service().makeable_items(character, ELEMENTAL_CONVERTER_LEVEL).contains(&i32::from(item_id)) {
                    return Err("This elemental converter cannot be made now".into());
                }
                self.pay_menu_skill(character, &menu, tick)?;
                self.item_service().make_with_skill(self, character, i32::from(item_id), ELEMENTAL_CONVERTER_LEVEL, tick).map(|_| ())
            }
            (SkillMenuKind::AutoSpell, SkillMenuChoice::AutoSpell(spell)) if spell != 0 => {
                let spell = auto_spell_choices(character, menu.level).into_iter().find(|skill| skill.id() == spell).ok_or("This skill cannot be an Auto Spell")?;
                self.pay_menu_skill(character, &menu, tick)?;
                let learned = learned_level(character, spell);
                self.script_skill_service().start_auto_spell(self, character, menu.level, spell, learned, tick)
            }
            (SkillMenuKind::WeaponRefine, SkillMenuChoice::WeaponRefine(index)) => {
                let index = index.checked_sub(CLIENT_INDEX_OFFSET).and_then(|index| usize::try_from(index).ok());
                index.map_or(Ok(()), |index| self.refine_weapon(character, &menu, index, tick))
            }
            (SkillMenuKind::RepairWeapon { target }, SkillMenuChoice::Repair(index)) if index != MENU_CANCELLED => {
                self.repair_weapon(state, character, &menu, *target, usize::from(index), tick)
            }
            (_, SkillMenuChoice::Arrow(_) | SkillMenuChoice::WeaponRefine(_) | SkillMenuChoice::Repair(_) | SkillMenuChoice::AutoSpell(_)) => Ok(()),
        }
    }

    fn pay_menu_skill(&self, character: &mut Character, menu: &SkillMenu, tick: u128) -> Result<(), String> {
        let plan = self.script_skill_service().requirements_plan(character, menu.skill_id, menu.level, tick)?;
        self.item_service().pay_requirement_plan(self, character, &plan, None, tick)
    }

    fn consume_items(&self, character: &mut Character, removals: Vec<(i32, i16)>, grants: Vec<ScriptItemGrant>) -> Result<(), String> {
        let change = ScriptInventoryTransaction {
            char_id: character.char_id,
            account_id: character.account_id,
            consumption: None,
            exact_removals: vec![],
            removals,
            identifications: vec![],
            grants,
            variables: vec![],
            zeny: None,
            hp: None,
            sp: None,
            max_weight: self.character_service().max_weight(character),
            max_slots: 100,
            world: None,
            reset_skills: None,
            fame: None,
            character_changes: vec![],
            pool_draws: vec![],
        };
        let result = self.repository.script_inventory_transaction(&change).map_err(|error| error.to_string())?;
        self.item_service().install_inventory(self, character, result.inventory, None);
        Ok(())
    }

    fn make_arrows(&self, character: &mut Character, menu: &SkillMenu, source: i32, tick: u128) -> Result<(), String> {
        let recipe = data().arrows.iter().find(|arrow| arrow.source == source).ok_or("This item cannot be turned into arrows")?;
        if !arrow_sources(character).contains(&source) {
            return Err("The item to make arrows from is unavailable".into());
        }
        self.pay_menu_skill(character, menu, tick)?;
        let grants = recipe
            .make
            .iter()
            .map(|product| ScriptItemGrant { item_id: product.item_id, amount: product.amount, identified: true, refine: 0, cards: [0; 4], unique_id: None, damaged: false })
            .collect();
        self.consume_items(character, vec![(source, 1)], grants)
    }

    fn send_weapon_refine_message(&self, char_id: u32, result: u32, item_id: i32) {
        let mut packet = WEAPON_REFINE_MESSAGE_PACKET.to_le_bytes().to_vec();
        packet.extend_from_slice(&result.to_le_bytes());
        packet.extend_from_slice(&(item_id as u16).to_le_bytes());
        self.send_to_character(char_id, packet);
    }

    fn refine_weapon(&self, character: &mut Character, menu: &SkillMenu, index: usize, tick: u128) -> Result<(), String> {
        let item = character.get_item_from_inventory(index).cloned().ok_or("Selected weapon is unavailable")?;
        let configuration = GlobalConfigService::instance();
        let model = configuration.find_item(item.item_id).ok_or("Unknown item")?;
        if model.item_type != ItemType::Weapon || !item.is_identified || item.equip != 0 || !refine::is_refineable(model) {
            return Err("The selected item cannot be refined".into());
        }
        if item.refine >= i16::from(menu.level) || item.refine >= MAX_REFINE {
            self.send_weapon_refine_message(character.char_id, MESSAGE_SKILL_TOO_LOW, item.item_id);
            return Ok(());
        }
        let cost = refine::refine_cost(model, item.refine, RefineCostType::Normal).ok_or("The selected weapon has no refine cost")?;
        if !has_item(character, cost.material) {
            self.send_weapon_refine_message(character.char_id, MESSAGE_MISSING_ORE, cost.material);
            return Ok(());
        }
        self.pay_menu_skill(character, menu, tick)?;
        let chance = i32::from(cost.rate) / 100 + (character.status.job_level as i32 - 50) / 2;
        self.consume_items(character, vec![(cost.material, 1)], vec![])?;
        if character.inventory[index].as_ref().map(|current| current.id) != Some(item.id) {
            return Err("The selected weapon changed".into());
        }
        if fastrand::i32(0..100) < chance {
            self.change_refine(character, index, 1, 0)?;
            self.send_weapon_refine_message(character.char_id, MESSAGE_UPGRADED, item.item_id);
            self.reward_forging_fame(character, index);
        } else {
            self.break_equipment(character, index)?;
            self.send_weapon_refine_message(character.char_id, MESSAGE_DESTROYED, item.item_id);
        }
        Ok(())
    }

    fn send_repair_ack(&self, char_id: u32, index: usize) {
        let mut packet = REPAIR_ACK_PACKET.to_le_bytes().to_vec();
        packet.extend_from_slice(&(index as u16).to_le_bytes());
        packet.push(REPAIR_SUCCESS);
        self.send_to_character(char_id, packet);
    }

    fn repair_weapon(&self, state: &mut ServerState, character: &mut Character, menu: &SkillMenu, target_id: u32, index: usize, tick: u128) -> Result<(), String> {
        let mut target = (target_id != character.char_id).then(|| state.characters_mut().remove(&target_id).ok_or("Repair target is not available")).transpose()?;
        let result = self.repair_item(character, target.as_mut(), menu, index, tick);
        if let Some(target) = target {
            state.insert_character(target);
        }
        result
    }

    /// The caster pays and the owner of the item, the caster or the target, gets it back in working order.
    fn repair_item(&self, character: &mut Character, mut target: Option<&mut Character>, menu: &SkillMenu, index: usize, tick: u128) -> Result<(), String> {
        let owner = target.as_deref().unwrap_or(character);
        if owner.map_instance_key != character.map_instance_key {
            return Err("Repair target is on another map".into());
        }
        let item = owner.get_item_from_inventory(index).filter(|item| item.is_damaged).ok_or("The selected item is not broken")?;
        let material = GlobalConfigService::instance().find_item(item.item_id).and_then(repair_material).ok_or("This item cannot be repaired")?;
        if !has_item(character, material) {
            return Err("The repair material is missing".into());
        }
        self.pay_menu_skill(character, menu, tick)?;
        self.consume_items(character, vec![(material, 1)], vec![])?;
        self.send_repair_ack(character.char_id, index);
        let Some(target) = target.as_deref_mut() else {
            self.clear_damage(character, index)?;
            return Ok(());
        };
        self.clear_damage(target, index)?;
        self.send_repair_ack(target.char_id, index);
        for packet in self.inventory_service().inventory_packets(target) {
            self.send_to_character(target.char_id, packet);
        }
        Ok(())
    }
}
