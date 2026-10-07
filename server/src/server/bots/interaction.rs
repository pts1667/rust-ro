//! Items, skills, chat, parties and trades of bots: the events a client sends for them, after the checks a client makes first.
//! The game answers these through the state of the character (inventory, party, trade), which the observation shows.
use models::enums::skill::SkillTargetType;
use serde::Deserialize;
use serde_json::{Value, json};

use super::command::{distance, locate, require_active};
use crate::server::Server;
use crate::server::model::events::game_event::{
    CharacterEquipItem, CharacterSkillUpgrade, CharacterUpdateStat, CharacterRemoveItem, CharacterSocial, CharacterTakeoffEquipItem, CharacterUseGroundSkill, CharacterUseItem,
    CharacterUseSkill, GameEvent, PlayerTradeAction, ScriptWorld, SocialAction,
};
use crate::server::model::game_systems::{PartyRequest, ScriptWorldRequest};
use crate::server::model::game_systems::PlayerTradeRequest;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const MAX_CHAT_LENGTH: usize = 200;
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
    /// `SP_STR` to `SP_LUK` of the client.
    fn status_id(self) -> u16 {
        13 + self as u16
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
    Create { name: String },
    Invite { name: String },
    Answer { accept: bool },
    Leave,
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

fn character_of(state: &ServerState, char_id: u32) -> Result<&Character, String> {
    state.get_character(char_id).ok_or_else(|| "Character is not in game".to_string())
}

fn sent(what: Value) -> Result<Value, String> {
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

fn chat_text(text: &str) -> Result<String, String> {
    let text = text.trim();
    if text.is_empty() || text.chars().count() > MAX_CHAT_LENGTH || text.chars().any(char::is_control) {
        return Err(format!("A message has between 1 and {MAX_CHAT_LENGTH} characters and no control characters"));
    }
    Ok(text.to_string())
}

impl Server {
    pub(super) fn bot_item(&self, state: &ServerState, char_id: u32, account_id: u32, action: &ItemAction) -> Result<Value, String> {
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

    pub(super) fn bot_progress(&self, state: &ServerState, char_id: u32, action: &ProgressAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        match action {
            ProgressAction::RaiseStat { stat, amount } => {
                if character.status.status_point == 0 {
                    return Err("The character has no status point to spend".into());
                }
                if *amount == 0 {
                    return Err("The amount is at least 1".into());
                }
                self.add_to_next_tick(GameEvent::CharacterUpdateStat(CharacterUpdateStat {
                    char_id,
                    stat_id: stat.status_id(),
                    change_amount: *amount,
                }));
                sent(json!({ "stat": format!("{stat:?}").to_lowercase(), "amount": amount }))
            }
            ProgressAction::LearnSkill { skill } => {
                if character.status.skill_point == 0 {
                    return Err("The character has no skill point to spend".into());
                }
                let config = GlobalConfigService::instance();
                let found = match skill {
                    SkillRef::Id(id) => config.find_skill_config(&script_sdk::Value::Number(*id as i32)),
                    SkillRef::Name(name) => config.find_skill_config(&script_sdk::Value::String(name.clone())),
                }
                .ok_or("Unknown skill")?;
                let skill_id = u16::try_from(found.id).map_err(|_| "Unknown skill")?;
                self.add_to_next_tick(GameEvent::CharacterSkillUpgrade(CharacterSkillUpgrade { char_id, skill_id }));
                sent(json!({ "skill": found.name }))
            }
        }
    }

    pub(super) fn bot_skill(&self, state: &ServerState, char_id: u32, cast: &SkillCast) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        require_active(character)?;
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
        let aimed = match target_type {
            SkillTargetType::Passive => return Err(format!("{} is passive", skill_config.name)),
            SkillTargetType::MySelf | SkillTargetType::Party => {
                self.add_to_next_tick(GameEvent::CharacterUseSkill(CharacterUseSkill { char_id, target_id: char_id, skill_id, skill_level: level }));
                json!({ "target": char_id })
            }
            SkillTargetType::Target | SkillTargetType::Friend | SkillTargetType::Trap => {
                let target_id = cast.target.ok_or("This skill needs a target")?;
                if target_id != char_id {
                    let target = locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?;
                    check_range(target.position())?;
                }
                self.add_to_next_tick(GameEvent::CharacterUseSkill(CharacterUseSkill { char_id, target_id, skill_id, skill_level: level }));
                json!({ "target": target_id })
            }
            SkillTargetType::Ground => {
                let (x, y) = match (cast.cell, cast.target) {
                    (Some(cell), _) => cell,
                    (None, Some(target_id)) => locate(state, character, target_id).ok_or_else(|| format!("No target {target_id} on this map"))?.position(),
                    (None, None) => return Err("This skill is cast on a cell: give x and y".into()),
                };
                check_range((x, y))?;
                self.add_to_next_tick(GameEvent::CharacterUseGroundSkill(CharacterUseGroundSkill { char_id, skill_id, skill_level: level, x, y }));
                json!({ "x": x, "y": y })
            }
        };
        sent(json!({ "skill": skill_config.name, "level": level, "aimed": aimed }))
    }

    pub(super) fn bot_chat(&self, state: &ServerState, char_id: u32, action: &ChatAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        match action {
            ChatAction::Say(text) => {
                let text = chat_text(text)?;
                if text.starts_with(['@', '#', '%']) {
                    return Err("Messages starting with @, # or % are commands, bots do not send them".into());
                }
                let spoken = format!("{} : {text}\0", character.name).into_bytes();
                self.add_to_next_tick(GameEvent::CharacterSocial(CharacterSocial { char_id, action: SocialAction::PublicChat(spoken) }));
            }
            ChatAction::Whisper { to, text } => {
                let text = chat_text(text)?;
                if to.starts_with('#') {
                    return Err("Whisper to the name of a character".into());
                }
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

    pub(super) fn bot_party(&self, state: &ServerState, char_id: u32, action: &PartyAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let party = |request| GameEvent::ScriptWorld(ScriptWorld { char_id, request: ScriptWorldRequest::Party(request) });
        let event = match action {
            PartyAction::Create { name } => {
                if character.game_systems.party.is_some() {
                    return Err("The character is already in a party, leave it first".into());
                }
                party(PartyRequest::CreateParty { name: name.trim().to_string(), item_pickup: false, item_share: false })
            }
            PartyAction::Invite { name } => {
                if character.game_systems.party.is_none() {
                    return Err("Create a party before inviting".into());
                }
                party(PartyRequest::InvitePartyByName(name.trim().to_string()))
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
        };
        self.add_to_next_tick(event);
        sent(Value::Null)
    }

    pub(super) fn bot_trade(&self, state: &ServerState, char_id: u32, account_id: u32, action: &TradeAction) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let session = state
            .find_session(account_id)
            .filter(|session| session.char_id == Some(char_id))
            .ok_or("The bot has no session")?;
        let request = match action {
            TradeAction::Request { target } => {
                require_active(character)?;
                if *target == char_id || locate(state, character, *target).is_none_or(|found| !found.is_player()) {
                    return Err(format!("No player {target} on this map"));
                }
                PlayerTradeRequest::Request(*target)
            }
            TradeAction::Answer { accept } => PlayerTradeRequest::Answer(*accept),
            TradeAction::OfferItem { index, amount } => {
                character.get_item_from_inventory(*index).ok_or_else(|| format!("No item at inventory index {index}"))?;
                let wire_index = u16::try_from(*index).ok().and_then(|index| index.checked_add(TRADE_ITEM_INDEX_OFFSET)).ok_or("Invalid index")?;
                PlayerTradeRequest::Offer { index: wire_index, amount: *amount }
            }
            TradeAction::OfferZeny { amount } => PlayerTradeRequest::Offer { index: 0, amount: *amount },
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
