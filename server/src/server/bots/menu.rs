//! Windows a skill opens once it was cast: a destination for Teleport and Warp Portal, the item of Arrow Crafting or a weapon to refine,
//! the recipes of Pharmacy. The observation lists what can be picked, and the answers run in the game loop so a refusal is the error of the call.
use models::enums::skill_enums::SkillEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::command::bot_session;
use crate::server::Server;
use crate::server::model::events::game_event::ScriptTeleportSelection;
use crate::server::model::map::Map;
use crate::server::model::session::SessionBinding;
use crate::server::script::game_data::data;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_crafting_service::CraftSelection;
use crate::server::service::skill_menu_service::{MENU_CANCELLED, SkillMenuChoice};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const CANCEL: &str = "cancel";
/// The destination Teleport level 1 and 2 offer first: a random cell of the map.
const RANDOM_DESTINATION: &str = "Random";

/// What a menu is answered with: a map name for the destinations, the `choice` number of an offer for the others.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum MenuAnswer {
    Number(u32),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillMenuAction {
    Choose(MenuAnswer),
    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CraftAction {
    pub item_id: i32,
    /// Star Crumb or element stone, forging only: up to three.
    pub additives: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct MenuChoiceView {
    pub choice: Value,
    pub name: String,
}

/// A skill waiting for the answer of its caster.
#[derive(Debug, Serialize)]
pub struct SkillMenuView {
    pub skill: String,
    /// `teleport`, `warp_portal`, `making_arrow`, `weapon_refine`, `auto_spell`, `repair_weapon`...
    pub kind: &'static str,
    pub choices: Vec<MenuChoiceView>,
}

#[derive(Debug, Serialize)]
pub struct MaterialView {
    pub item_id: i32,
    pub name: String,
    pub amount: i16,
}

#[derive(Debug, Serialize)]
pub struct RecipeView {
    pub item_id: i32,
    pub name: String,
    pub materials: Vec<MaterialView>,
}

/// The crafting window of Pharmacy, Cooking or a forging skill.
#[derive(Debug, Serialize)]
pub struct CraftingView {
    pub cooking: bool,
    pub recipes: Vec<RecipeView>,
}

fn item_name(item_id: i32) -> String {
    GlobalConfigService::item_name_if_loaded(item_id).unwrap_or_else(|| item_id.to_string())
}

fn text_choice(name: &str) -> MenuChoiceView {
    MenuChoiceView { choice: json!(name), name: name.to_string() }
}

/// The destinations of the pending Teleport or Warp Portal menu, with the id of its skill.
fn destinations(character: &Character) -> Option<(&'static str, u32, Vec<String>)> {
    let state = &character.script_skill_state;
    if let Some(portal) = &state.pending_warp_portal {
        let maps = portal.destinations.iter().map(|point| Map::name_without_ext(&point.map)).collect();
        return Some(("warp_portal", SkillEnum::AlWarp.id(), maps));
    }
    let teleport = state.pending_teleport.as_ref()?;
    let mut maps = vec![RANDOM_DESTINATION.to_string()];
    if teleport.level == 2 {
        maps.push(teleport.save_map.clone());
    }
    Some(("teleport", teleport.skill_id, maps))
}

fn skill_name(skill_id: u32) -> String {
    GlobalConfigService::instance()
        .find_skill_config(&script_sdk::Value::Number(skill_id as i32))
        .map_or_else(|| skill_id.to_string(), |skill| skill.name.clone())
}

pub fn skill_menu_view(server: &Server, state: &ServerState, character: &Character, tick: u128) -> Option<SkillMenuView> {
    if let Some((kind, skill_id, maps)) = destinations(character) {
        return Some(SkillMenuView { skill: skill_name(skill_id), kind, choices: maps.iter().map(|map| text_choice(map)).collect() });
    }
    let menu = character.pending_skill_menu.as_ref().filter(|menu| menu.expires_at > tick)?;
    Some(SkillMenuView {
        skill: skill_name(menu.skill_id),
        kind: menu.kind_name(),
        choices: server
            .skill_menu_offers(state, character, menu)
            .into_iter()
            .map(|offer| MenuChoiceView { choice: json!(offer.choice), name: offer.name })
            .collect(),
    })
}

pub fn crafting_view(character: &Character) -> Option<CraftingView> {
    let session = character.pending_craft.as_ref()?;
    let recipes = data()
        .recipes
        .iter()
        .filter(|recipe| session.recipes.contains(&recipe.id))
        .map(|recipe| RecipeView {
            item_id: recipe.item_id,
            name: item_name(recipe.item_id),
            materials: recipe
                .materials
                .iter()
                .map(|material| MaterialView { item_id: material.item_id, name: item_name(material.item_id), amount: material.amount })
                .collect(),
        })
        .collect();
    Some(CraftingView { cooking: session.cooking, recipes })
}

fn offered_destination<'a>(maps: &'a [String], answer: &MenuAnswer) -> Result<&'a str, String> {
    let MenuAnswer::Text(map) = answer else {
        return Err(format!("Answer with the name of a map: {}", maps.join(", ")));
    };
    maps.iter()
        .find(|offered| offered.eq_ignore_ascii_case(map.trim()))
        .map(String::as_str)
        .ok_or_else(|| format!("{} is not offered, the destinations are {}", map.trim(), maps.join(", ")))
}

impl Server {
    pub(crate) fn bot_skill_menu(&self, state: &mut ServerState, account_id: u32, char_id: u32, action: &SkillMenuAction, tick: u128) -> Result<Value, String> {
        let session = bot_session(state, account_id, char_id)?;
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        if let Some((kind, skill_id, maps)) = destinations(character) {
            let map = match action {
                SkillMenuAction::Cancel => CANCEL,
                SkillMenuAction::Choose(answer) => offered_destination(&maps, answer)?,
            }
            .to_string();
            let selection = ScriptTeleportSelection { char_id, skill_id, map: map.clone(), session: Some(SessionBinding::new(&session)) };
            let skill_service = self.script_skill_service();
            state
                .with_character_taken(char_id, |state, character| {
                    if kind == "warp_portal" {
                        skill_service.finish_warp_portal_menu(self, state, character, &selection, tick)
                    } else {
                        skill_service.finish_teleport_menu(self, state, character, &selection, tick)
                    }
                })
                .ok_or("Character is not in game")??;
            return Ok(if map == CANCEL { json!({ "cancelled": true }) } else { json!({ "chosen": map }) });
        }
        let menu = character.pending_skill_menu.clone().filter(|menu| menu.expires_at > tick).ok_or("No skill is waiting for a choice")?;
        let value = match action {
            SkillMenuAction::Cancel => {
                self.choose_in_skill_menu(state, char_id, SkillMenuChoice::Arrow(MENU_CANCELLED), tick)?;
                return Ok(json!({ "cancelled": true }));
            }
            SkillMenuAction::Choose(MenuAnswer::Text(_)) => return Err("Answer with the number of a choice".into()),
            SkillMenuAction::Choose(MenuAnswer::Number(value)) => *value,
        };
        let offers = self.skill_menu_offers(state, character, &menu);
        if !offers.iter().any(|offer| offer.choice == value) {
            let listed: Vec<String> = offers.iter().map(|offer| format!("{} ({})", offer.choice, offer.name)).collect();
            return Err(format!("{value} is not offered, the choices are {}", listed.join(", ")));
        }
        self.choose_in_skill_menu(state, char_id, menu.choice(value)?, tick)?;
        Ok(json!({ "chosen": value }))
    }

    /// Makes an item of the open crafting window; the answer says whether it succeeded, which the recipe leaves to chance.
    pub(crate) fn bot_craft(&self, state: &mut ServerState, char_id: u32, action: &CraftAction, tick: u128) -> Result<Value, String> {
        state
            .with_character_taken(char_id, |_, character| {
                let session = character.pending_craft.as_ref().ok_or("No crafting window is open: cast the crafting skill first")?;
                let offered = data().recipes.iter().any(|recipe| session.recipes.contains(&recipe.id) && recipe.item_id == action.item_id);
                if !offered {
                    return Err(format!("{} is not in the window, see crafting in the observation", action.item_id));
                }
                if action.additives.len() > 3 {
                    return Err("At most 3 additives".to_string());
                }
                let mut materials = [0; 3];
                materials[..action.additives.len()].copy_from_slice(&action.additives);
                let selection = CraftSelection { char_id, item_id: action.item_id, materials, cooking: session.cooking };
                let success = self.item_service().make_item(self, character, selection, tick)?;
                Ok(json!({ "item": item_name(action.item_id), "success": success }))
            })
            .ok_or("Character is not in game")?
    }
}
