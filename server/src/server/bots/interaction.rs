//! Items, skills, chat, parties and trades of bots: the events a client sends for them, after the checks a client makes first.
//! The game answers these through the state of the character (inventory, party, trade), which the observation shows.
use std::ops::RangeInclusive;

use models::enums::skill::SkillTargetType;
use models::enums::skill_enums::SkillEnum;
use models::enums::status::StatusTypes;
use serde::Deserialize;
use serde_json::{Value, json};

use super::command::{distance, locate, require_active};
use super::companions::companion_skill_id;
use crate::server::Server;
use crate::server::model::events::game_event::{
    CharacterEquipItem, CharacterRemoveItem, CharacterSocial, CharacterTakeoffEquipItem, CharacterUseGroundSkill, CharacterUseItem,
    CharacterUseSkill, GameEvent, GameEventHandler, PlayerTradeAction, ScriptWorld, SocialAction,
};
use crate::server::model::game_systems::{PartyRequest, ScriptWorldRequest};
use crate::server::model::game_systems::{ItemContainer, PlayerTradeRequest};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const MAX_CHAT_LENGTH: usize = 200;
/// Cells between two traders, as `player_trade_service` requires.
const TRADE_REACH: u16 = 2;
/// Basic Skill level a character needs to create a party (`PartyRequest::CreateParty`).
const PARTY_CREATE_BASIC_SKILL: u8 = 7;
const GUILD_SKILLS: RangeInclusive<u32> = 10000..=10015;
/// Trade packets count the inventory from here, index 0 being the zeny.
const TRADE_ITEM_INDEX_OFFSET: u16 = 2;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemAction {
    Use { index: usize },
    Equip { index: usize },
    Unequip { index: usize },
    Drop { index: usize, amount: i16 },
}

/// A skill by its id or by its name (`NV_BASIC`, `AL_HEAL`...).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum SkillRef {
    Id(u32),
    Name(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkillCast {
    pub skill: SkillRef,
    pub level: Option<u8>,
    pub target: Option<u32>,
    pub cell: Option<(u16, u16)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stat {
    Str,
    Agi,
    Vit,
    Int,
    Dex,
    Luk,
}

impl Stat {
    fn status_type(self) -> StatusTypes {
        match self {
            Stat::Str => StatusTypes::Str,
            Stat::Agi => StatusTypes::Agi,
            Stat::Vit => StatusTypes::Vit,
            Stat::Int => StatusTypes::Int,
            Stat::Dex => StatusTypes::Dex,
            Stat::Luk => StatusTypes::Luk,
        }
    }
}

/// Spending the points a level up gives, like the + buttons of the client.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressAction {
    RaiseStat { stat: Stat, amount: u16 },
    LearnSkill { skill: SkillRef },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatAction {
    Say(String),
    Whisper { to: String, text: String },
    Party(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PartyAction {
    Create { name: String, item_pickup: bool, item_share: bool },
    Invite { name: String },
    Answer { accept: bool },
    Leave,
    Options { exp_share: Option<bool>, item_pickup: Option<bool>, item_share: Option<bool> },
    Leader { name: String },
    Kick { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TradeAction {
    Request { target: u32 },
    Answer { accept: bool },
    OfferItem { index: usize, amount: u32 },
    OfferZeny { amount: u32 },
    Lock,
    Confirm,
    Cancel,
}

/// Moving items in the storage a Kafra opened, see `storage` in the observation.
#[derive(Debug, Clone, PartialEq)]
pub enum StorageAction {
    Deposit { index: usize, amount: u32 },
    Withdraw { index: usize, amount: u32 },
    Close,
}

/// The event a cast sends, by what the skill is aimed at.
enum Aim {
    Unit(CharacterUseSkill),
    Ground(CharacterUseGroundSkill),
}

pub(super) fn character_of(state: &ServerState, char_id: u32) -> Result<&Character, String> {
    state.get_character(char_id).ok_or_else(|| "Character is not in game".to_string())
}

pub(super) fn player_named<'a>(state: &'a ServerState, name: &str) -> Result<&'a Character, String> {
    state
        .characters()
        .values()
        .find(|other| other.name.eq_ignore_ascii_case(name.trim()))
        .ok_or_else(|| format!("{} is not in the game", name.trim()))
}

pub(super) fn sent(what: Value) -> Result<Value, String> {
    if what.is_null() { Ok(json!({ "sent": true })) } else { Ok(json!({ "sent": true, "request": what })) }
}

/// How a skill is aimed: at a `target`, a `ground` cell, or only at oneself.
pub fn target_kind(target: SkillTargetType) -> &'static str {
    match target {
        SkillTargetType::Passive => "passive",
        SkillTargetType::Target | SkillTargetType::Friend | SkillTargetType::Trap => "target",
        SkillTargetType::Ground => "ground",
        SkillTargetType::MySelf | SkillTargetType::Party => "self",
    }
}

pub(super) fn check_amount(amount: u32, held: i16) -> Result<(), String> {
    if amount == 0 || amount > u32::try_from(held).unwrap_or(0) {
        return Err(format!("The amount goes from 1 to {held}"));
    }
    Ok(())
}

fn skill_refusal(server: &Server, character: &Character, skill: SkillEnum, name: &str) -> String {
    let available = server.skill_tree_service().skill_tree(character);
    if !available.iter().any(|known| known.value == skill) {
        let names: Vec<&str> = available.iter().map(|known| known.value.to_name()).collect();
        return format!("{name} can not be learned yet, the skills available now are {}", names.join(", "));
    }
    format!("{name} can not be raised further (level {})", learned_level(&character.status, skill.id()))
}

pub(super) fn chat_text(text: &str) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() || text.chars().count() > MAX_CHAT_LENGTH || text.chars().any(char::is_control) {
        return Err(format!("A message has between 1 and {MAX_CHAT_LENGTH} characters and no control characters"));
    }
    Ok(text.to_string())
}

impl Server {
    pub(crate) fn bot_item(&self, state: &ServerState, char_id: u32, account_id: u32, action: &ItemAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        require_active(character)?;
        let (index, item) = match action {
            ItemAction::Use { index } | ItemAction::Equip { index } | ItemAction::Unequip { index } | ItemAction::Drop { index, .. } => {
                (*index, character.get_item_from_inventory(*index).ok_or_else(|| format!("No item at inventory index {index}"))?)
            }
        };
        match action {
            ItemAction::Use { .. } => {
                self.add_to_next_tick(GameEvent::CharacterUseItem(CharacterUseItem { char_id, target_char_id: account_id, index }));
            }
            ItemAction::Equip { .. } => {
                if item.equip != 0 {
                    return Err("The item is already equipped".into());
                }
                self.add_to_next_tick(GameEvent::CharacterEquipItem(CharacterEquipItem { char_id, index, requested_location: None }));
            }
            ItemAction::Unequip { .. } => {
                if item.equip == 0 {
                    return Err("The item is not equipped".into());
                }
                self.add_to_next_tick(GameEvent::CharacterTakeoffEquipItem(CharacterTakeoffEquipItem { char_id, index }));
            }
            ItemAction::Drop { amount, .. } => {
                if *amount < 1 || *amount > item.amount {
                    return Err(format!("The amount goes from 1 to {}", item.amount));
                }
                self.add_to_next_tick(GameEvent::CharacterDropItem(CharacterRemoveItem { char_id, index, amount: *amount, price: 0 }));
            }
        }
        sent(json!({ "index": index, "item": item.name_english }))
    }

    /// Spends the points now, in the game loop, so the answer says what the server did instead of what was asked.
    pub(crate) fn bot_progress(&self, state: &mut ServerState, char_id: u32, action: &ProgressAction) -> Result<Value, String> {
        let character = state.characters_mut().get_mut(&char_id).ok_or("Character is not in game")?;
        match action {
            ProgressAction::RaiseStat { stat, amount } => {
                let points = character.status.status_point;
                if points == 0 {
                    return Err("The character has no status point to spend".into());
                }
                if *amount == 0 {
                    return Err("The amount is at least 1".into());
                }
                let service = self.character_service();
                let held = service.stat_value(&character.status, &stat.status_type());
                let highest = GlobalConfigService::instance().config().game.max_stat_level;
                if held.saturating_add(*amount) > highest {
                    return Err(format!("{stat:?} is {held}, it can not go above {highest}"));
                }
                if !service.allocate_status_point(character, stat.status_type(), *amount) {
                    return Err(format!("Raising {stat:?} by {amount} costs more than the {points} status points left"));
                }
                Ok(json!({
                    "stat": format!("{stat:?}").to_lowercase(),
                    "value": service.stat_value(&character.status, &stat.status_type()),
                    "status_points": character.status.status_point,
                }))
            }
            ProgressAction::LearnSkill { skill } => {
                if let Some(skill_id) = companion_skill_id(skill) {
                    return self.bot_learn_homunculus_skill(character, skill_id);
                }
                if character.status.skill_point == 0 {
                    return Err("The character has no skill point to spend".into());
                }
                let config = GlobalConfigService::instance();
                let found = match skill {
                    SkillRef::Id(id) => config.find_skill_config(&script_sdk::Value::Number(*id as i32)),
                    SkillRef::Name(name) => config.find_skill_config(&script_sdk::Value::String(name.clone())),
                }
                .ok_or("Unknown skill")?;
                let skill_id = u32::try_from(found.id).map_err(|_| "Unknown skill")?;
                if GUILD_SKILLS.contains(&skill_id) {
                    return Err(format!("{} is a guild skill, bots do not learn those", found.name));
                }
                let skill = SkillEnum::from_id(skill_id);
                if !self.character_service().allocate_skill_point(character, skill) {
                    return Err(skill_refusal(self, character, skill, &found.name));
                }
                Ok(json!({
                    "skill": found.name,
                    "level": learned_level(&character.status, skill_id),
                    "skill_points": character.status.skill_point,
                }))
            }
        }
    }

    /// Moves the items now, in the game loop, so a refusal (too heavy, storage full) comes back as the error of the call.
    pub(crate) fn bot_storage(&self, state: &mut ServerState, char_id: u32, action: &StorageAction) -> Result<Value, String> {
        state
            .with_character_taken(char_id, |state, character| {
                let systems = &character.game_systems;
                if !systems.storage_open && systems.guild_storage_open.is_none() {
                    return Err("No storage is open: talk to a Kafra".to_string());
                }
                let (source, destination, index, amount, name) = match action {
                    StorageAction::Close => {
                        self.script_world_service().close_storage(character)?;
                        return Ok(json!({ "closed": true }));
                    }
                    StorageAction::Deposit { index, amount } => {
                        let item = character.get_item_from_inventory(*index).ok_or_else(|| format!("No item at inventory index {index}"))?;
                        if item.equip != 0 {
                            return Err("Take the item off before storing it".into());
                        }
                        check_amount(*amount, item.amount)?;
                        (ItemContainer::Inventory, ItemContainer::Storage, *index, *amount, item.name_english.clone())
                    }
                    StorageAction::Withdraw { index, amount } => {
                        let records = if systems.guild_storage_open.is_some() { &systems.guild_storage_items } else { &systems.storage_items };
                        let record = records.get(*index).ok_or_else(|| format!("No item at storage index {index}"))?;
                        check_amount(*amount, record.amount)?;
                        let name = GlobalConfigService::item_name_if_loaded(record.item_id).unwrap_or_default();
                        (ItemContainer::Storage, ItemContainer::Inventory, *index, *amount, name)
                    }
                };
                let index = u16::try_from(index).map_err(|_| "Invalid index")?;
                self.script_world_service().move_container(self, state, character, source, destination, index, amount)?;
                Ok(json!({ "item": name, "amount": amount }))
            })
            .ok_or("Character is not in game")?
    }

    /// Starts the cast now, in the game loop: a refusal (not enough SP, a cell the skill can not use) is the error of the call.
    pub(crate) fn bot_skill(&self, state: &mut ServerState, char_id: u32, cast: &SkillCast, tick: u128) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        require_active(character)?;
        if let Some(skill_id) = companion_skill_id(&cast.skill) {
            return self.bot_companion_skill(state, char_id, cast, skill_id, u64::try_from(tick).unwrap_or(u64::MAX));
        }
        let config = GlobalConfigService::instance();
        let skill_config = match &cast.skill {
            SkillRef::Id(id) => config.find_skill_config(&script_sdk::Value::Number(*id as i32)),
            SkillRef::Name(name) => config.find_skill_config(&script_sdk::Value::String(name.clone())),
        }
        .ok_or("Unknown skill")?;
        let skill_id = skill_config.id;
        let learned = StatusService::instance()
            .to_snapshot(&character.status)
            .known_skills()
            .iter()
            .find(|skill| skill.value.id() == skill_id)
            .map_or(0, |skill| skill.level);
        if learned == 0 {
            return Err(format!("{} is not a skill of this character", skill_config.name));
        }
        let level = cast.level.unwrap_or(learned);
        if level == 0 || level > learned {
            return Err(format!("{} is known up to level {learned}", skill_config.name));
        }
        let here = (character.x(), character.y());
        let target_type = *skill_config.target_type();
        let range = skill_config
            .range_per_level()
            .as_ref()
            .and_then(|ranges| ranges.get(usize::from(level) - 1).copied())
            .or(*skill_config.range())
            .filter(|range| *range > 0);
        let check_range = |at: (u16, u16)| match range {
            Some(range) if i32::from(distance(here, at)) > range + 1 => {
                Err(format!("The target is {} cells away, the range of the skill is {range}: move closer", distance(here, at)))
            }
            _ => Ok(()),
        };
        let (event, aimed) = match target_type {
            SkillTargetType::Passive => return Err(format!("{} is passive", skill_config.name)),
            SkillTargetType::MySelf | SkillTargetType::Party => {
                (Aim::Unit(CharacterUseSkill { char_id, target_id: char_id, skill_id, skill_level: level }), json!({ "target": char_id }))
            }
            SkillTargetType::Target | SkillTargetType::Friend | SkillTargetType::Trap => {
                let target_id = cast.target.ok_or("This skill needs a target")?;
                if target_id != char_id {
                    let target = locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?;
                    check_range(target.position())?;
                }
                (Aim::Unit(CharacterUseSkill { char_id, target_id, skill_id, skill_level: level }), json!({ "target": target_id }))
            }
            SkillTargetType::Ground => {
                let (x, y) = match (cast.cell, cast.target) {
                    (Some(cell), _) => cell,
                    (None, Some(target_id)) => locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?.position(),
                    (None, None) => return Err("This skill is cast on a cell: give x and y".into()),
                };
                check_range((x, y))?;
                (Aim::Ground(CharacterUseGroundSkill { char_id, skill_id, skill_level: level, x, y }), json!({ "x": x, "y": y }))
            }
        };
        match event {
            Aim::Unit(event) => self.handle_character_skill(state, event, tick)?,
            Aim::Ground(event) => event.handle(self, state, tick)?,
        }
        Ok(json!({ "skill": skill_config.name, "level": level, "aimed": aimed, "started": true }))
    }

    pub(crate) fn bot_chat(&self, state: &ServerState, char_id: u32, action: &ChatAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        match action {
            ChatAction::Say(text) => {
                let text = chat_text(text)?;
                if text.starts_with(['@', '#', '%']) {
                    return Err("Messages starting with @, # or % are commands: use the command action for an @ command".into());
                }
                let spoken = format!("{} : {text}\0", character.name).into_bytes();
                self.add_to_next_tick(GameEvent::CharacterSocial(CharacterSocial { char_id, action: SocialAction::PublicChat(spoken) }));
            }
            ChatAction::Whisper { to, text } => {
                let text = chat_text(text)?;
                if to.starts_with('#') {
                    return Err("Whisper to the name of a character".into());
                }
                player_named(state, to)?;
                let message = [text.as_bytes(), b"\0"].concat();
                self.add_to_next_tick(GameEvent::CharacterSocial(CharacterSocial {
                    char_id,
                    action: SocialAction::Whisper { target: to.clone(), message },
                }));
            }
            ChatAction::Party(text) => {
                if character.game_systems.party.is_none() {
                    return Err("The character has no party".into());
                }
                let text = format!("{} : {}", character.name, chat_text(text)?);
                self.add_to_next_tick(GameEvent::ScriptWorld(ScriptWorld { char_id, request: ScriptWorldRequest::Party(PartyRequest::PartyMessage(text)) }));
            }
        }
        sent(Value::Null)
    }

    pub(crate) fn bot_party(&self, state: &ServerState, char_id: u32, action: &PartyAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let party = |request| GameEvent::ScriptWorld(ScriptWorld { char_id, request: ScriptWorldRequest::Party(request) });
        let event = match action {
            PartyAction::Create { name, item_pickup, item_share } => {
                if character.game_systems.party.is_some() {
                    return Err("The character is already in a party, leave it first".into());
                }
                if learned_level(&character.status, SkillEnum::NvBasic.id()) < PARTY_CREATE_BASIC_SKILL {
                    return Err(format!("Creating a party needs Basic Skill level {PARTY_CREATE_BASIC_SKILL}; joining one needs none"));
                }
                party(PartyRequest::CreateParty { name: name.trim().to_string(), item_pickup: *item_pickup, item_share: *item_share })
            }
            PartyAction::Invite { name } => {
                if character.game_systems.party.is_none() {
                    return Err("Create a party before inviting".into());
                }
                let name = name.trim();
                let invited = player_named(state, name)?;
                if invited.char_id == char_id {
                    return Err("A character can not invite itself".into());
                }
                if invited.game_systems.party.is_some() {
                    return Err(format!("{name} is already in a party"));
                }
                if invited.game_systems.party_invitation.is_some() {
                    return Err(format!("{name} already has an invitation to answer"));
                }
                party(PartyRequest::InvitePartyByName(name.to_string()))
            }
            PartyAction::Answer { accept } => {
                let invitation = character.game_systems.party_invitation.as_ref().ok_or("No party invitation is pending")?;
                party(PartyRequest::AnswerPartyInvite { party_id: invitation.party_id, accept: *accept })
            }
            PartyAction::Leave => {
                if character.game_systems.party.is_none() {
                    return Err("The character has no party".into());
                }
                party(PartyRequest::LeaveParty)
            }
            PartyAction::Options { exp_share, item_pickup, item_share } => {
                let current = led_party(character)?;
                party(PartyRequest::ChangePartyOptions {
                    exp_share: exp_share.unwrap_or(current.exp_share),
                    item_rules: Some((item_pickup.unwrap_or(current.item_pickup), item_share.unwrap_or(current.item_share))),
                })
            }
            PartyAction::Leader { name } => {
                let current = led_party(character)?;
                let name = name.trim();
                let member = player_named(state, name)?;
                if member.char_id == char_id || !current.members.contains(&member.char_id) {
                    return Err(format!("{name} is not another member of the party"));
                }
                if member.map_instance_key != character.map_instance_key {
                    return Err(format!("{name} must be on the same map to lead the party"));
                }
                party(PartyRequest::ChangePartyLeader(member.account_id))
            }
            PartyAction::Kick { name } => {
                let current = led_party(character)?;
                let name = name.trim();
                let records = self.script_world_service().repository.party_member_records(current.id).map_err(|error| error.to_string())?;
                let member = records
                    .iter()
                    .find(|record| record.name.eq_ignore_ascii_case(name) && record.char_id as u32 != char_id)
                    .ok_or_else(|| format!("{name} is not another member of the party"))?;
                party(PartyRequest::ExpelParty { account_id: member.account_id as u32, name: member.name.clone() })
            }
        };
        self.add_to_next_tick(event);
        sent(Value::Null)
    }

    pub(crate) fn bot_trade(&self, state: &ServerState, char_id: u32, account_id: u32, action: &TradeAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let session = state
            .find_session(account_id)
            .filter(|session| session.char_id == Some(char_id))
            .ok_or("The bot has no session")?;
        let request = match action {
            TradeAction::Request { target } => {
                require_active(character)?;
                let partner = locate(state, character, *target)
                    .filter(|found| *target != char_id && found.is_player())
                    .ok_or_else(|| format!("No player {target} on this map"))?;
                let apart = distance((character.x(), character.y()), partner.position());
                if apart > TRADE_REACH {
                    return Err(format!("The player is {apart} cells away, trading needs {TRADE_REACH} or less: move closer"));
                }
                if learned_level(&character.status, SkillEnum::NvBasic.id()) == 0 {
                    return Err("Trading needs Basic Skill level 1".into());
                }
                PlayerTradeRequest::Request(*target)
            }
            TradeAction::Answer { accept } => PlayerTradeRequest::Answer(*accept),
            TradeAction::OfferItem { index, amount } => {
                let item = character.get_item_from_inventory(*index).ok_or_else(|| format!("No item at inventory index {index}"))?;
                if item.equip != 0 {
                    return Err("Take the item off before offering it".into());
                }
                if *amount == 0 || *amount > u32::try_from(item.amount).unwrap_or(0) {
                    return Err(format!("The amount goes from 1 to {}", item.amount));
                }
                let wire_index = u16::try_from(*index).ok().and_then(|index| index.checked_add(TRADE_ITEM_INDEX_OFFSET)).ok_or("Invalid index")?;
                PlayerTradeRequest::Offer { index: wire_index, amount: *amount }
            }
            TradeAction::OfferZeny { amount } => {
                if *amount > character.status.zeny {
                    return Err(format!("The character has {} zeny, not {amount}", character.status.zeny));
                }
                PlayerTradeRequest::Offer { index: 0, amount: *amount }
            }
            TradeAction::Lock => PlayerTradeRequest::Lock,
            TradeAction::Confirm => PlayerTradeRequest::Confirm,
            TradeAction::Cancel => PlayerTradeRequest::Cancel,
        };
        if !matches!(action, TradeAction::Request { .. } | TradeAction::Cancel) && character.game_systems.trade.is_none() {
            return Err("No trade is open".into());
        }
        self.add_to_next_tick(GameEvent::PlayerTrade(PlayerTradeAction { char_id, account_id, auth_code: session.auth_code, request }));
        sent(Value::Null)
    }
}

/// The party of `character`, when it leads it.
fn led_party(character: &Character) -> Result<&crate::server::model::game_systems::PartyRecord, String> {
    let party = character.game_systems.party.as_ref().ok_or("The character has no party")?;
    if party.leader_char_id != character.char_id {
        return Err("Only the party leader can do this".into());
    }
    Ok(party)
}
