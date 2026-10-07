use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use models::enums::class::JobName;
use models::enums::element::Element;
use models::enums::EnumWithNumberValue;
use script_sdk::{Function, Value};

use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemGrant};
use crate::repository::fame_repository::CraftingFamePlan;
use crate::server::model::events::game_event::{GameEvent, FameChanged};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::script::game_data::{data, Recipe};
use crate::server::service::item_service::ItemService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::Server;

#[derive(Debug, Clone, PartialEq)]
pub struct CraftSession {
    pub trigger: u16,
    pub cooking: bool,
    pub recipes: Vec<u32>,
    pub expires_at: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftSelection {
    pub char_id: u32,
    pub item_id: i32,
    pub materials: [i32; 3],
    pub cooking: bool,
}

fn count_items(character: &Character) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for (_, item) in character.inventory_iter() { *counts.entry(item.item_id).or_default() += i32::from(item.amount); }
    counts
}

fn recipe_matches(recipe: &Recipe, trigger: u16) -> bool {
    if trigger > 20 { recipe.level == trigger }
    else if trigger > 10 { (11..=20).contains(&recipe.level) }
    else { recipe.level <= trigger }
}

fn can_make(character: &Character, recipe: &Recipe, counts: &HashMap<i32, i32>) -> bool {
    let skill = character.status.known_skills.iter().find(|skill| skill.value.id() == recipe.skill_id).map_or(0, |skill| u16::from(skill.level));
    (recipe.skill_id == 0 || skill >= recipe.skill_level) && recipe.materials.iter().all(|material| counts.get(&material.item_id).copied().unwrap_or_default() >= i32::from(material.amount.max(1)))
}

impl ItemService {
    pub(crate) fn validate_crafting(&self, character: &Character, function: Function, arguments: &[Value]) -> Result<(), String> {
        let trigger = u16::try_from(arguments.first().ok_or("Missing crafting level")?.number_value()?).map_err(|_| "Invalid crafting level")?;
        if function == Function::Cooking && !(11..=20).contains(&trigger) || function == Function::Produce && !matches!(trigger, 1..=3 | 11..=23) { return Err("Invalid classic crafting level".into()); }
        if character.is_dead() || character.game_systems.buying_store.is_some() || character.game_systems.vending_store.is_some() { return Err("Character cannot craft now".into()); }
        Ok(())
    }

    pub(crate) fn open_crafting(&self, character: &mut Character, function: Function, arguments: &[Value]) -> Result<(), String> {
        self.validate_crafting(character, function, arguments)?;
        self.open_crafting_window(character, function == Function::Cooking, arguments[0].number_value()? as u16, None)
    }

    /// The window of a crafting skill only lists the recipes of that skill.
    pub(crate) fn open_crafting_window(&self, character: &mut Character, cooking: bool, trigger: u16, only_skill: Option<u32>) -> Result<(), String> {
        let counts = count_items(character);
        let recipes: Vec<_> = data().recipes.iter().filter(|recipe| recipe_matches(recipe, trigger) && can_make(character, recipe, &counts)
            && only_skill.is_none_or(|skill| recipe.skill_id == skill) && self.configuration_service.find_item(recipe.item_id).is_some()).collect();
        if only_skill.is_some() && recipes.is_empty() {
            return Err("Nothing can be made".into());
        }
        let mut packet = if cooking { 0x025a_u16.to_le_bytes().to_vec() } else { 0x018d_u16.to_le_bytes().to_vec() };
        packet.extend_from_slice(&((if cooking { 6 } else { 4 } + recipes.len() * if cooking { 2 } else { 8 }) as u16).to_le_bytes());
        if cooking { packet.extend_from_slice(&1_u16.to_le_bytes()); }
        for recipe in &recipes {
            packet.extend_from_slice(&(recipe.item_id as u16).to_le_bytes());
            if !cooking { packet.extend_from_slice(&[0; 6]); }
        }
        let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|error| error.to_string())?.as_millis();
        character.pending_craft = (!recipes.is_empty()).then(|| CraftSession { trigger, cooking, recipes: recipes.iter().map(|recipe| recipe.id).collect(), expires_at: now + 120_000 });
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet))).map_err(|error| error.to_string())
    }

    /// Items the character can make with the recipes of a trigger level, in the order of the database.
    pub(crate) fn makeable_items(&self, character: &Character, trigger: u16) -> Vec<i32> {
        let counts = count_items(character);
        data().recipes.iter().filter(|recipe| recipe_matches(recipe, trigger) && can_make(character, recipe, &counts)
            && self.configuration_service.find_item(recipe.item_id).is_some()).map(|recipe| recipe.item_id).collect()
    }

    /// Makes `item_id` as the skill that owns its recipe, without a crafting window.
    pub(crate) fn make_with_skill(&self, server: &Server, character: &mut Character, item_id: i32, trigger: u16, tick: u128) -> Result<bool, String> {
        let recipe = data().recipes.iter().find(|recipe| recipe.item_id == item_id && recipe.level == trigger).ok_or("No recipe makes this item")?;
        character.pending_craft = Some(CraftSession { trigger, cooking: false, recipes: vec![recipe.id], expires_at: tick + 1000 });
        let result = self.make_item(server, character, CraftSelection { char_id: character.char_id, item_id, materials: [0; 3], cooking: false }, tick);
        character.pending_craft = None;
        result
    }

    pub(crate) fn make_item(&self, server: &Server, character: &mut Character, selection: CraftSelection, tick: u128) -> Result<bool, String> {
        let session = character.pending_craft.as_ref().ok_or("No crafting window is active")?;
        if session.expires_at < tick || selection.cooking != session.cooking { return Err("Crafting window expired or response type changed".into()); }
        let counts = count_items(character);
        let recipe = data().recipes.iter().find(|recipe| recipe.item_id == selection.item_id && session.recipes.contains(&recipe.id) && can_make(character, recipe, &counts)).ok_or("Recipe or materials are unavailable")?;
        let mut removals: HashMap<i32, i16> = recipe.materials.iter().filter(|material| material.amount > 0).map(|material| (material.item_id, material.amount)).collect();
        let weapon = self.configuration_service.get_item(recipe.item_id).item_type == models::enums::item::ItemType::Weapon;
        let mut stars = 0;
        let mut element = 0;
        for &material in &selection.materials {
            if material == 0 { continue; }
            if !weapon { return Err("Crafting additives can only be used for weapons".into()); }
            if material == 1000 { stars += 1; }
            else if (994..=997).contains(&material) && element == 0 {
                element = [Element::Fire, Element::Water, Element::Wind, Element::Earth][(material - 994) as usize].value() as i16;
            } else { return Err("Invalid forging additive".into()); }
            let amount = removals.entry(material).or_default();
            *amount = amount.checked_add(1).ok_or("Forging material count overflow")?;
        }
        if removals.iter().any(|(id, amount)| counts.get(id).copied().unwrap_or_default() < i32::from(*amount)) { return Err("Forging additives are unavailable".into()); }
        let mut rng = fastrand::Rng::new();
        let chance = self.crafting_chance(character, recipe, session.trigger, stars, element != 0, &counts, &mut rng);
        let success = rng.i32(0..10_000) < chance;
        let alchemy = self.configuration_service.find_skill_config(&(recipe.skill_id as i32).into()).is_some_and(|skill| skill.name() == "AM_PHARMACY");
        let creator = [character.char_id as u16 as i16, (character.char_id >> 16) as u16 as i16];
        let cards = if weapon { [255, (stars * 5 * 256 + i32::from(element)) as i16, creator[0], creator[1]] }
            else if alchemy { [254, 0, creator[0], creator[1]] } else { [0; 4] };
        let change = ScriptInventoryTransaction { char_id: character.char_id, account_id: character.account_id, consumption: None, exact_removals: vec![], hp: None, sp: None,
            removals: removals.into_iter().collect(), grants: if success { vec![ScriptItemGrant { item_id: recipe.item_id, amount: 1, identified: true, refine: 0, cards, unique_id: None, damaged: false }] } else { vec![] },
            identifications: vec![], variables: vec![], zeny: None, max_weight: server.character_service().max_weight(character), max_slots: 100, world: None, reset_skills: None,
            character_changes: vec![], pool_draws: vec![], fame: Some(CraftingFamePlan { item_id: recipe.item_id, skill_id: recipe.skill_id, success,
                additive_slots: stars as u8 + u8::from(element != 0), previous_potion_streak: character.game_systems.potion_success_counter }) };
        let result = server.repository.script_inventory_transaction(&change).map_err(|error| error.to_string())?;
        self.install_inventory(server, character, result.inventory, None);
        if let Some(fame) = result.fame {
            character.game_systems.potion_success_counter = fame.potion_streak;
            if let Some(category) = fame.category.filter(|_| fame.gained > 0) {
                server.add_to_next_tick(GameEvent::FameChanged(FameChanged { category, ranked_creators: fame.rankings.iter().map(|entry| entry.char_id).collect() }));
            }
        }
        character.pending_craft = None;
        let code: u16 = if alchemy { if success { 2 } else { 3 } } else if selection.cooking { if success { 4 } else { 5 } } else if success { 0 } else { 1 };
        let mut packet = 0x018f_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&code.to_le_bytes()); packet.extend_from_slice(&(recipe.item_id as u16).to_le_bytes());
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet))).map_err(|error| error.to_string())?;
        character.refresh_script_context();
        Ok(success)
    }

    fn crafting_chance(&self, character: &Character, recipe: &Recipe, trigger: u16, stars: i32, elemental: bool, counts: &HashMap<i32, i32>, rng: &mut fastrand::Rng) -> i32 {
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let skill = |name: &str| character.status.known_skills.iter().find(|skill| skill.value.to_name() == name).map_or(0, |skill| i32::from(skill.level));
        let level = character.status.known_skills.iter().find(|skill| skill.value.id() == recipe.skill_id).map_or(0, |skill| i32::from(skill.level));
        let name = self.configuration_service.find_skill_config(&(recipe.skill_id as i32).into()).map(|skill| skill.name().as_str()).unwrap_or("");
        let item = self.configuration_service.get_item(recipe.item_id);
        let mut chance = if item.item_type == models::enums::item::ItemType::Weapon {
            let weapon_level = i32::from(item.weapon_level.unwrap_or(1)).clamp(1, 3);
            character.status.job_level as i32 * 20 + i32::from(snapshot.dex()) * 10 + i32::from(snapshot.luk()) * 10 + rng.i32(1..=100) * 10
                + 4 / weapon_level * 1000 + level * 500 + skill("BS_WEAPONRESEARCH") * 100 - if elemental { 2500 } else { 0 } - stars * 1500
                + if counts.contains_key(&989) { 1000 } else if counts.contains_key(&988) { 500 } else if counts.contains_key(&987) { 250 } else { 0 }
        } else { match name {
            "BS_IRON" | "BS_STEEL" | "BS_ENCHANTEDSTONE" => {
                let base = character.status.job_level as i32 * 20 + i32::from(snapshot.dex()) * 10 + i32::from(snapshot.luk()) * 10 + rng.i32(1..=100) * 10;
                match recipe.item_id { 998 => base + 4000 + level * 500, 999 => base + 3000 + level * 500, 1000 => 100_000, _ => base + 1000 + level * 500 }
            }
            "ASC_CDP" => 2000 + 40 * i32::from(snapshot.dex()) + 20 * i32::from(snapshot.luk()),
            "AL_HOLYWATER" | "SA_CREATECON" => 100_000,
            "AM_PHARMACY" => {
                let base = skill("AM_LEARNINGPOTION") * 50 + skill("AM_PHARMACY") * 300 + character.status.job_level as i32 * 20
                    + i32::from(snapshot.int()) / 2 * 10 + i32::from(snapshot.dex()) * 10 + i32::from(snapshot.luk()) * 10;
                base + match recipe.item_id { 501 | 503 | 504 => rng.i32(1..=100) * 10 + 2000, 970 => rng.i32(1..=100) * 10 + 1000,
                    7135..=7138 => rng.i32(1..=100) * 10, 546 => -rng.i32(1..=50) * 10, 547 | 7139 => -rng.i32(1..=100) * 10, _ => 0 }
            }
            _ if (11..=20).contains(&recipe.level) => {
                if trigger >= 15 { 10_000 } else { 1200 * (i32::from(trigger) - 10) + 20 * (character.status.base_level as i32 + 1) + 20 * (i32::from(snapshot.dex()) + 1)
                    + 100 * rng.i32(6..30) - 400 * (i32::from(recipe.level) - 10) - 10 * (101 - i32::from(snapshot.luk())) - 500 * (recipe.materials.len() as i32 - 1) - 100 * rng.i32(1..=4) }
            }
            _ => 5000,
        }};
        let baby = JobName::try_from_value(character.status.job as usize).is_ok_and(|job| job.mask() & models::enums::class::JOB_BABY_MASK != 0);
        if baby && (item.item_type == models::enums::item::ItemType::Weapon || name == "AM_PHARMACY") { chance = chance * 70 / 100; }
        chance.clamp(1, 100_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_recipe_windows_filter_weapon_metals_and_all_dishes() {
        let recipe = |level| Recipe { id: 1, item_id: 501, level, skill_id: 0, skill_level: 0, materials: vec![] };
        assert!(recipe_matches(&recipe(1), 3));
        assert!(!recipe_matches(&recipe(21), 3));
        assert!(recipe_matches(&recipe(21), 21));
        assert!(!recipe_matches(&recipe(22), 21));
        assert!(recipe_matches(&recipe(20), 11));
        assert!(!recipe_matches(&recipe(21), 11));
    }
}
