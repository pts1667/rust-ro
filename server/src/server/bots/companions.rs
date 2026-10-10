//! Pets, homunculi and mercenaries: the requests a client sends for them, run by the game loop so that the answer is the outcome.
use serde::Serialize;
use serde_json::{Value, json};

use super::command::{distance, locate, require_active};
use super::interaction::{SkillCast, SkillRef, character_of};
use super::observation::StatsView;
use crate::server::Server;
use crate::server::model::game_systems::{
    CompanionRequest, HomunculusRecord, HomunculusRequest, MercenaryRecord, PetRecord, PetRequest, ScriptWorldRequest,
};
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_world_service::{
    available_homunculus_skills, homunculus_world_id, mercenary_world_id, pet_world_id, world_data,
};
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub const HOMUNCULUS_SKILLS: std::ops::RangeInclusive<u32> = 8001..=8016;
pub const MERCENARY_SKILLS: std::ops::RangeInclusive<u32> = 8201..=8240;

/// The menu numbers of the pet window of the client.
const PET_MENU_FEED: u8 = 1;
const PET_MENU_PERFORM: u8 = 2;
const PET_MENU_RETURN_TO_EGG: u8 = 3;
const PET_MENU_UNEQUIP: u8 = 4;
/// The menu numbers of the homunculus window.
const HOMUNCULUS_MENU_FEED: u8 = 1;
const HOMUNCULUS_MENU_DELETE: u8 = 2;
/// The mercenary window's command that dismisses the contract.
const MERCENARY_DISMISS: u8 = 2;
/// Cells between the character and the monster it tames, as the map thread requires.
const CAPTURE_REACH: u16 = 5;
/// The client sends emotions up to this.
const MAX_PET_EMOTION: i32 = 200_000;

/// Where a companion is sent or what it is told to attack.
#[derive(Debug, Clone, PartialEq)]
pub enum Order {
    Move { x: u16, y: u16 },
    /// Walks back to the owner and follows again.
    Follow,
    Attack { target: u32, repeat: bool },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PetAction {
    /// Hatches the egg at an inventory index, in the window a pet egg opened.
    Hatch { index: usize },
    Feed,
    Perform,
    ReturnToEgg,
    Unequip,
    Rename(String),
    Emote(i32),
    Equip { index: usize },
    /// Throws the taming item that was just used at a monster.
    Capture { target: u32 },
    /// The outcome of a capture, by the claim `Capture` answered with.
    CaptureOutcome { claim: u64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum HomunculusAction {
    Feed,
    Rename(String),
    Delete,
    Order(Order),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MercenaryAction {
    Dismiss,
    Order(Order),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompanionAction {
    Pet(PetAction),
    Homunculus(HomunculusAction),
    Mercenary(MercenaryAction),
}

impl From<PetAction> for CompanionAction {
    fn from(action: PetAction) -> Self {
        CompanionAction::Pet(action)
    }
}

impl From<HomunculusAction> for CompanionAction {
    fn from(action: HomunculusAction) -> Self {
        CompanionAction::Homunculus(action)
    }
}

impl From<MercenaryAction> for CompanionAction {
    fn from(action: MercenaryAction) -> Self {
        CompanionAction::Mercenary(action)
    }
}

#[derive(Debug, Serialize)]
pub struct ItemRef {
    pub item_id: i32,
    pub name: String,
}

fn item_ref(item_id: i32) -> Option<ItemRef> {
    (item_id > 0).then(|| ItemRef { item_id, name: GlobalConfigService::item_name_if_loaded(item_id).unwrap_or_else(|| item_id.to_string()) })
}

#[derive(Debug, Serialize)]
pub struct PetView {
    pub name: String,
    pub class_id: u16,
    pub level: u16,
    /// 0 to 1000, below 100 the pet is shy and refuses to help.
    pub intimacy: i32,
    pub intimacy_stage: &'static str,
    /// 0 to 100; feeding above 75 earns less intimacy and above 90 costs some.
    pub hunger: i32,
    pub hunger_stage: &'static str,
    pub renamed: bool,
    /// What `pet_feed` consumes.
    pub food: Option<ItemRef>,
    pub accessory: Option<ItemRef>,
    /// The accessory item this pet wears, for `pet_equip`.
    pub accessory_wanted: Option<ItemRef>,
    pub x: Option<u16>,
    pub y: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct EggView {
    /// The inventory index `pet_hatch` takes.
    pub index: usize,
    pub item_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CaptureView {
    /// The taming item in use, `pet_capture` throws it.
    pub item: Option<ItemRef>,
    pub expires_in_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct CompanionSkillView {
    pub id: u32,
    pub name: String,
    pub level: u8,
    pub max_level: u8,
    /// `target`, `ground`, `self` or `passive`: what the skill is aimed at.
    pub aim: &'static str,
    pub range: Option<i32>,
    /// A skill point of the homunculus raises it with `learn_skill`.
    pub learnable: bool,
}

#[derive(Debug, Serialize)]
pub struct HomunculusView {
    pub name: String,
    pub class_id: u16,
    pub level: u16,
    /// A resting homunculus is called with the skill Call Homunculus (`AM_CALLHOMUN`).
    pub active: bool,
    pub evolved: bool,
    pub renamed: bool,
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u32,
    pub max_sp: u32,
    /// 0 to 100000; the homunculus runs away at 0.
    pub intimacy: u32,
    /// 0 to 100; it needs feeding before it starves.
    pub hunger: u16,
    pub experience: u64,
    pub skill_points: u16,
    pub stats: StatsView,
    pub food: Option<ItemRef>,
    pub skills: Vec<CompanionSkillView>,
    pub x: Option<u16>,
    pub y: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct MercenaryView {
    pub name: String,
    pub class_id: u16,
    pub level: u16,
    /// 0 archer, 1 lancer, 2 swordsman.
    pub guild: u8,
    pub hp: u32,
    pub max_hp: u32,
    pub sp: u32,
    pub max_sp: u32,
    pub kill_count: u32,
    pub contract_ms_left: u64,
    pub skills: Vec<CompanionSkillView>,
    pub x: Option<u16>,
    pub y: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct CompanionsView {
    pub pet: Option<PetView>,
    /// The pet eggs of the hatching window a pet egg opened: answer with `pet_hatch`.
    pub pet_eggs: Option<Vec<EggView>>,
    /// A taming item was used: `pet_capture` throws it at a monster.
    pub pet_capture: Option<CaptureView>,
    pub homunculus: Option<HomunculusView>,
    pub mercenary: Option<MercenaryView>,
}

fn intimacy_stage(intimacy: i32) -> &'static str {
    match intimacy {
        ..=99 => "awkward",
        100..=249 => "shy",
        250..=749 => "neutral",
        750..=909 => "cordial",
        _ => "loyal",
    }
}

fn hunger_stage(hunger: i32) -> &'static str {
    match hunger {
        ..=10 => "very_hungry",
        11..=25 => "hungry",
        26..=75 => "neutral",
        76..=90 => "satisfied",
        _ => "stuffed",
    }
}

fn aim_of(metadata: &SkillMetadata) -> &'static str {
    match metadata.target_type.as_deref() {
        Some("Attack" | "Support" | "Friend") => "target",
        Some("Ground") => "ground",
        Some("Self") => "self",
        _ => "passive",
    }
}

fn companion_skill(metadata: &SkillMetadata, level: u8, max_level: u8, learnable: bool) -> CompanionSkillView {
    CompanionSkillView {
        id: metadata.id,
        name: metadata.name.clone(),
        level,
        max_level,
        aim: aim_of(metadata),
        range: metadata.range(level.max(1)).filter(|range| *range > 0),
        learnable,
    }
}

fn position_of(character: &Character, companion_id: u32) -> (Option<u16>, Option<u16>) {
    let position = character.game_systems.rendered_companions.get(&companion_id);
    (position.map(|position| position.x), position.map(|position| position.y))
}

fn active_pet(character: &Character) -> Option<&PetRecord> {
    character.game_systems.pet.as_ref().filter(|pet| !pet.incubating)
}

fn pet_view(character: &Character) -> Option<PetView> {
    let pet = active_pet(character)?;
    let definition = world_data().pets.iter().find(|definition| definition.class_id == pet.class_id);
    let (x, y) = position_of(character, pet_world_id(pet.id));
    Some(PetView {
        name: pet.name.clone(),
        class_id: pet.class_id,
        level: pet.level,
        intimacy: pet.intimacy,
        intimacy_stage: intimacy_stage(pet.intimacy),
        hunger: pet.hunger,
        hunger_stage: hunger_stage(pet.hunger),
        renamed: pet.renamed,
        food: definition.and_then(|definition| item_ref(definition.food_item)),
        accessory: item_ref(pet.equipped_item),
        accessory_wanted: definition.and_then(|definition| item_ref(definition.equip_item)),
        x,
        y,
    })
}

fn hatching_eggs(character: &Character) -> Option<Vec<EggView>> {
    character.game_systems.pet_hatching.then(|| {
        character
            .inventory_iter()
            .filter(|(_, item)| item.card0 == 256 && item.amount > 0 && world_data().pets.iter().any(|pet| pet.egg_item == item.item_id))
            .map(|(index, item)| EggView { index, item_id: item.item_id, name: item.name_english.clone() })
            .collect()
    })
}

fn homunculus_view(character: &Character, homunculus: &HomunculusRecord) -> HomunculusView {
    let data = world_data().homunculi.iter().find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id);
    let can_learn = homunculus.active && homunculus.hp > 0 && homunculus.skill_points > 0;
    let [str, agi, vit, int, dex, luk] = homunculus.stats;
    let (x, y) = position_of(character, homunculus_world_id(homunculus));
    HomunculusView {
        name: homunculus.name.clone(),
        class_id: homunculus.class_id,
        level: homunculus.level,
        active: homunculus.active,
        evolved: homunculus.evolved,
        renamed: homunculus.renamed,
        hp: homunculus.hp,
        max_hp: homunculus.max_hp,
        sp: homunculus.sp,
        max_sp: homunculus.max_sp,
        intimacy: homunculus.intimacy,
        hunger: homunculus.hunger,
        experience: homunculus.experience,
        skill_points: homunculus.skill_points,
        stats: StatsView { str, agi, vit, int, dex, luk },
        food: data.and_then(|data| item_ref(data.food_item)),
        skills: available_homunculus_skills(homunculus)
            .into_iter()
            .map(|(definition, metadata, level)| companion_skill(metadata, level, definition.max_level, can_learn && level < definition.max_level))
            .collect(),
        x,
        y,
    }
}

fn mercenary_view(character: &Character, mercenary: &MercenaryRecord, tick: u128) -> MercenaryView {
    let skills = world_data()
        .mercenaries
        .iter()
        .find(|definition| definition.class_id == mercenary.class_id)
        .map(|definition| {
            definition
                .skills
                .iter()
                .filter_map(|skill| SkillMetadata::find_by_name(&skill.name).map(|metadata| companion_skill(metadata, skill.level, skill.level, false)))
                .collect()
        })
        .unwrap_or_default();
    let left = mercenary.contract_remaining.unwrap_or_else(|| mercenary.expires_at.saturating_sub(u64::try_from(tick).unwrap_or(u64::MAX)));
    let (x, y) = position_of(character, mercenary_world_id(mercenary));
    MercenaryView {
        name: mercenary.name.clone(),
        class_id: mercenary.class_id,
        level: mercenary.level,
        guild: mercenary.guild,
        hp: mercenary.hp,
        max_hp: mercenary.max_hp,
        sp: mercenary.sp,
        max_sp: mercenary.max_sp,
        kill_count: mercenary.kill_count,
        contract_ms_left: left,
        skills,
        x,
        y,
    }
}

pub fn companions_view(character: &Character, tick: u128) -> CompanionsView {
    let now = u64::try_from(tick).unwrap_or(u64::MAX);
    let systems = &character.game_systems;
    CompanionsView {
        pet: pet_view(character),
        pet_eggs: hatching_eggs(character),
        pet_capture: systems
            .pet_capture
            .as_ref()
            .filter(|capture| capture.expires_at > now)
            .map(|capture| CaptureView { item: item_ref(capture.item_id), expires_in_ms: capture.expires_at - now }),
        homunculus: systems.homunculus.as_ref().map(|homunculus| homunculus_view(character, homunculus)),
        mercenary: systems.mercenary.as_ref().map(|mercenary| mercenary_view(character, mercenary, tick)),
    }
}

/// The id of a homunculus or mercenary skill, by id or by name.
pub fn companion_skill_id(skill: &SkillRef) -> Option<u32> {
    let id = match skill {
        SkillRef::Id(id) => *id,
        SkillRef::Name(name) => SkillMetadata::find_by_name(&name.to_ascii_uppercase())?.id,
    };
    (HOMUNCULUS_SKILLS.contains(&id) || MERCENARY_SKILLS.contains(&id)).then_some(id)
}

fn companion_id(character: &Character, mercenary: bool) -> Result<u32, String> {
    let systems = &character.game_systems;
    if mercenary {
        let mercenary = systems.mercenary.as_ref().filter(|mercenary| mercenary.hp > 0).ok_or("The character has no mercenary")?;
        Ok(mercenary_world_id(mercenary))
    } else {
        let homunculus = systems.homunculus.as_ref().ok_or("The character has no homunculus")?;
        if !homunculus.active || homunculus.hp == 0 {
            return Err("The homunculus is resting or dead: call it with Call Homunculus (AM_CALLHOMUN)".into());
        }
        Ok(homunculus_world_id(homunculus))
    }
}

impl Server {
    pub(crate) fn bot_companion(&self, state: &mut ServerState, char_id: u32, action: &CompanionAction, tick: u128) -> Result<Value, String> {
        let now = u64::try_from(tick).unwrap_or(u64::MAX);
        require_active(character_of(state, char_id)?)?;
        match action {
            CompanionAction::Pet(action) => self.bot_pet(state, char_id, action, now),
            CompanionAction::Homunculus(action) => self.bot_homunculus(state, char_id, action, now),
            CompanionAction::Mercenary(action) => self.bot_mercenary(state, char_id, action, now),
        }
    }

    fn run_world_request(&self, state: &mut ServerState, char_id: u32, request: ScriptWorldRequest, now: u64) -> Result<(), String> {
        self.script_world_service().handle_request(self, state, char_id, request, now)
    }

    fn bot_pet(&self, state: &mut ServerState, char_id: u32, action: &PetAction, now: u64) -> Result<Value, String> {
        let request = match action {
            PetAction::CaptureOutcome { claim } => return capture_outcome(character_of(state, char_id)?, *claim),
            PetAction::Hatch { index } => {
                let eggs = hatching_eggs(character_of(state, char_id)?).ok_or("No hatching window is open: use a Pet Incubator first")?;
                if !eggs.iter().any(|egg| egg.index == *index) {
                    let indexes: Vec<String> = eggs.iter().map(|egg| egg.index.to_string()).collect();
                    return Err(format!("Inventory index {index} is not an egg of the window, the eggs are at {}", indexes.join(", ")));
                }
                PetRequest::HatchPet(u16::try_from(*index).map_err(|_| "Invalid index")?)
            }
            PetAction::Feed => PetRequest::PetMenu(PET_MENU_FEED),
            PetAction::Perform => PetRequest::PetMenu(PET_MENU_PERFORM),
            PetAction::ReturnToEgg => PetRequest::PetMenu(PET_MENU_RETURN_TO_EGG),
            PetAction::Unequip => PetRequest::PetMenu(PET_MENU_UNEQUIP),
            PetAction::Rename(name) => PetRequest::PetRename(name.trim().to_string()),
            PetAction::Emote(emotion) => {
                if !(0..=MAX_PET_EMOTION).contains(emotion) {
                    return Err(format!("An emotion goes from 0 to {MAX_PET_EMOTION}"));
                }
                PetRequest::PetEmotion(*emotion)
            }
            PetAction::Equip { index } => PetRequest::EquipPetAccessory(u16::try_from(*index).map_err(|_| "Invalid index")?),
            PetAction::Capture { target } => {
                let character = character_of(state, char_id)?;
                let monster = state.get_map_instance_from_character(character).and_then(|map| map.state().get_mob(*target).map(|mob| (mob.x, mob.y)));
                let (x, y) = monster.ok_or_else(|| format!("No monster {target} on this map"))?;
                let apart = distance((character.x(), character.y()), (x, y));
                if apart > CAPTURE_REACH {
                    return Err(format!("The monster is {apart} cells away, taming needs {CAPTURE_REACH} or less: move closer"));
                }
                PetRequest::CapturePet(*target)
            }
        };
        let capturing = matches!(action, PetAction::Capture { .. });
        self.run_world_request(state, char_id, ScriptWorldRequest::Pet(request), now)?;
        let character = character_of(state, char_id)?;
        if capturing {
            let pending = character.game_systems.pending_pet_capture.as_ref().ok_or("The taming item no longer works: it expired, or this map forbids taming")?;
            return Ok(json!({ "capturing": true, "claim": pending.claim_id }));
        }
        Ok(json!({ "pet": pet_view(character) }))
    }

    fn bot_homunculus(&self, state: &mut ServerState, char_id: u32, action: &HomunculusAction, now: u64) -> Result<Value, String> {
        let request = match action {
            HomunculusAction::Feed => ScriptWorldRequest::Homunculus(HomunculusRequest::HomunculusMenu(HOMUNCULUS_MENU_FEED)),
            HomunculusAction::Delete => ScriptWorldRequest::Homunculus(HomunculusRequest::HomunculusMenu(HOMUNCULUS_MENU_DELETE)),
            HomunculusAction::Rename(name) => ScriptWorldRequest::Homunculus(HomunculusRequest::HomunculusRename(name.trim().to_string())),
            HomunculusAction::Order(order) => {
                let id = companion_id(character_of(state, char_id)?, false)?;
                order_request(id, order)
            }
        };
        self.run_world_request(state, char_id, request, now)?;
        let character = character_of(state, char_id)?;
        match action {
            HomunculusAction::Delete => Ok(json!({ "deleted": character.game_systems.homunculus.is_none() })),
            HomunculusAction::Order(order) => Ok(json!({ "homunculus": order_name(order) })),
            _ => Ok(json!({ "homunculus": character.game_systems.homunculus.as_ref().map(|homunculus| homunculus_view(character, homunculus)) })),
        }
    }

    fn bot_mercenary(&self, state: &mut ServerState, char_id: u32, action: &MercenaryAction, now: u64) -> Result<Value, String> {
        let id = companion_id(character_of(state, char_id)?, true)?;
        let request = match action {
            MercenaryAction::Dismiss => ScriptWorldRequest::Companion(CompanionRequest::DismissMercenary(MERCENARY_DISMISS)),
            MercenaryAction::Order(order) => order_request(id, order),
        };
        self.run_world_request(state, char_id, request, now)?;
        match action {
            MercenaryAction::Dismiss => Ok(json!({ "dismissed": character_of(state, char_id)?.game_systems.mercenary.is_none() })),
            MercenaryAction::Order(order) => Ok(json!({ "mercenary": order_name(order) })),
        }
    }

    /// Casts a skill of the homunculus or the mercenary now, in the game loop: a refusal (SP, range, cooldown) is the error of the call.
    pub(crate) fn bot_companion_skill(&self, state: &mut ServerState, char_id: u32, cast: &SkillCast, skill_id: u32, now: u64) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        require_active(character)?;
        let metadata = SkillMetadata::find(skill_id).ok_or("Unknown skill")?;
        let (companion, learned) = self
            .script_world_service()
            .companion_skill_source(character, skill_id)
            .ok_or_else(|| format!("{} is not a skill of the active homunculus or of the mercenary", metadata.name))?;
        let level = cast.level.unwrap_or(learned);
        if level == 0 || level > learned {
            return Err(format!("{} is known up to level {learned}", metadata.name));
        }
        let request = match aim_of(metadata) {
            "ground" => {
                let (x, y) = match (cast.cell, cast.target) {
                    (Some(cell), _) => cell,
                    (None, Some(target)) => locate(state, character, target).ok_or_else(|| format!("No target {target} on this map"))?.position(),
                    (None, None) => return Err("This skill is cast on a cell: give x and y".into()),
                };
                CompanionRequest::UseCompanionGroundSkill { skill_id, skill_level: level, x, y }
            }
            "target" => {
                let target_id = cast.target.ok_or("This skill needs a target")?;
                CompanionRequest::UseCompanionSkill { skill_id, skill_level: level, target_id }
            }
            "self" => CompanionRequest::UseCompanionSkill { skill_id, skill_level: level, target_id: 0 },
            _ => return Err(format!("{} is passive", metadata.name)),
        };
        let caster = if mercenary_world_id_of(character) == Some(companion) { "mercenary" } else { "homunculus" };
        self.run_world_request(state, char_id, ScriptWorldRequest::Companion(request), now)?;
        let casting = character_of(state, char_id)?.game_systems.companion_commands.get(&companion).is_some_and(|command| command.cast.is_some());
        Ok(json!({ "skill": metadata.name, "level": level, "caster": caster, "casting": casting, "started": true }))
    }

    /// Spends a skill point of the homunculus on its skill.
    pub(crate) fn bot_learn_homunculus_skill(&self, character: &mut Character, skill_id: u32) -> Result<Value, String> {
        if !HOMUNCULUS_SKILLS.contains(&skill_id) {
            return Err("Mercenaries have the skills of their contract, there is nothing to learn".into());
        }
        self.script_world_service().learn_homunculus_skill(character, skill_id)?;
        let homunculus = character.game_systems.homunculus.as_ref().ok_or("No homunculus exists")?;
        Ok(json!({
            "skill": SkillMetadata::find(skill_id).map(|metadata| metadata.name.as_str()),
            "level": homunculus.skills.get(&skill_id).copied().unwrap_or(0),
            "homunculus_skill_points": homunculus.skill_points,
        }))
    }
}

fn mercenary_world_id_of(character: &Character) -> Option<u32> {
    character.game_systems.mercenary.as_ref().map(mercenary_world_id)
}

fn order_name(order: &Order) -> &'static str {
    match order {
        Order::Move { .. } => "moving",
        Order::Follow => "following",
        Order::Attack { .. } => "attacking",
    }
}

fn order_request(id: u32, order: &Order) -> ScriptWorldRequest {
    ScriptWorldRequest::Homunculus(match order {
        Order::Move { x, y } => HomunculusRequest::CompanionMove { id, x: *x, y: *y },
        Order::Follow => HomunculusRequest::CompanionMoveToOwner(id),
        Order::Attack { target, repeat } => HomunculusRequest::CompanionAttack { id, target: *target, repeat: *repeat },
    })
}

fn capture_outcome(character: &Character, claim: u64) -> Result<Value, String> {
    let systems = &character.game_systems;
    let outcome = if systems.pending_pet_capture.as_ref().is_some_and(|pending| pending.claim_id == claim) {
        "pending"
    } else if systems.completed_pet_capture.as_ref().is_some_and(|completed| completed.claim_id == claim && completed.committed) {
        "captured"
    } else {
        "failed"
    };
    Ok(json!({ "capture": outcome }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_stages_follow_the_thresholds_of_the_pet_scripts() {
        assert_eq!((intimacy_stage(0), intimacy_stage(100), intimacy_stage(250), intimacy_stage(750), intimacy_stage(910)), ("awkward", "shy", "neutral", "cordial", "loyal"));
        assert_eq!((hunger_stage(10), hunger_stage(25), hunger_stage(75), hunger_stage(90), hunger_stage(100)), ("very_hungry", "hungry", "neutral", "satisfied", "stuffed"));
    }

    #[test]
    fn companion_skills_are_named_by_id_or_by_name() {
        assert_eq!(companion_skill_id(&SkillRef::Id(8001)), Some(8001));
        assert_eq!(companion_skill_id(&SkillRef::Id(8201)), Some(8201));
        assert_eq!(companion_skill_id(&SkillRef::Id(28)), None);
        assert_eq!(companion_skill_id(&SkillRef::Name("hlif_heal".into())), Some(8001));
        assert_eq!(companion_skill_id(&SkillRef::Name("AL_HEAL".into())), None);
    }
}
