//! Sitting, emotes, guilds, friends, chat rooms and the party booking board of bots: the events a client sends for
//! them, after the checks a client makes first. What the game answers shows in the observation and the messages.
use models::enums::class::JobName;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::command::require_active;
use super::interaction::{character_of, chat_text, player_named};
use crate::server::Server;
use crate::server::model::events::game_event::{
    CharacterClientCommand, CharacterSit, CharacterSocial, CharacterStand, ClientCommand, GameEvent, ScriptWorld, SocialAction,
};
use crate::server::model::game_systems::{BookingRequest, GuildRequest, ScriptWorldRequest};
use crate::server::model::party_booking::BOOKING_JOBS;
use crate::server::state::server::ServerState;

/// The emotes by the name of their client command (`/thx` is `thx`), in the order of their numbers.
pub const EMOTES: [&str; 34] = [
    "!", "?", "ho", "lv", "swt", "ic", "an", "ag", "$", "...", "scissors", "rock", "paper", "flag", "lv2", "thx", "wah", "sry", "heh", "swt2",
    "hmm", "no1", "no", "omg", "oh", "x", "hlp", "go", "sob", "gg", "kis", "kis2", "pif", "ok",
];
/// Emote numbers the server takes, the one of a chat ban aside.
const EMOTE_COUNT: u8 = 88;
const EMOTE_CHAT_PROHIBIT: u8 = 34;
/// An entry of the job list of a booking that asks for nobody.
const NO_JOB: i16 = 0xFF;
const GUILD_NAME_SIZE: usize = 24;
const ROOM_TITLE_SIZE: usize = 60;
const ROOM_PASSWORD_SIZE: usize = 8;

/// The name of emote `number`, its number when it has no command.
pub fn emote_name(number: u8) -> String {
    EMOTES.get(usize::from(number)).map_or_else(|| number.to_string(), |name| (*name).to_string())
}

/// An emote by its name (`thx`, `/thx`) or its number.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum EmoteRef {
    Number(u8),
    Name(String),
}

impl EmoteRef {
    fn number(&self) -> Result<u8, String> {
        let number = match self {
            EmoteRef::Number(number) => *number,
            EmoteRef::Name(name) => {
                let name = name.trim().trim_start_matches('/');
                EMOTES
                    .iter()
                    .position(|known| known.eq_ignore_ascii_case(name))
                    .ok_or_else(|| format!("Unknown emote {name}, the emotes are {}", EMOTES.join(" ")))? as u8
            }
        };
        if number >= EMOTE_COUNT || number == EMOTE_CHAT_PROHIBIT {
            return Err(format!("Emote numbers go from 0 to {}, {EMOTE_CHAT_PROHIBIT} aside", EMOTE_COUNT - 1));
        }
        Ok(number)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SocialRequest {
    Sit,
    Stand,
    Emote(EmoteRef),
    GuildCreate { name: String },
    GuildInvite { name: String },
    GuildAnswer { accept: bool },
    GuildLeave,
    GuildChat(String),
    FriendAdd { name: String },
    /// `from` is the request the bot received, as its window shows it: account and character id.
    FriendAnswer { accept: bool, from: Option<(u32, u32)> },
    FriendRemove { name: String },
    RoomCreate { title: String, limit: u16, public: bool, password: String },
    RoomJoin { owner: String, npc: Option<u32>, password: String },
    RoomLeave,
    BookingRegister { level: Option<u32>, map_id: i16, jobs: Vec<String> },
    BookingSearch { level: Option<u32>, map_id: i16, job: Option<String> },
    BookingDelete,
}

/// The job number of a booking entry.
fn booking_job(name: &str) -> Result<i16, String> {
    let name = name.trim();
    let job = JobName::try_from_string(name)
        .or_else(|_| JobName::try_from_string(&name.replace("Swordman", "Swordsman")))
        .map_err(|_| format!("Unknown job {name}"))?;
    Ok(job.value() as i16)
}

fn booking_level(level: Option<u32>, character_level: u32) -> i16 {
    level.unwrap_or(character_level).min(i16::MAX as u32) as i16
}

/// The name of booking job number `job`, `null` for an empty entry.
pub fn booking_job_name(job: i16) -> Option<String> {
    (job >= 0 && job != NO_JOB).then(|| JobName::try_from_value(job as usize).map_or_else(|_| job.to_string(), |job| format!("{job:?}")))
}

/// A party booking ad, as the booking window shows it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BookingAdView {
    pub index: u32,
    /// The character that put it up: whisper it to ask to join.
    pub name: String,
    pub level: i16,
    pub map_id: i16,
    /// The jobs it looks for, empty for anyone.
    pub jobs: Vec<String>,
    /// Seconds since the epoch.
    pub started: u32,
}

impl BookingAdView {
    pub fn of(ad: &crate::server::model::party_booking::BookingAd) -> Self {
        Self {
            index: ad.index,
            name: ad.name.clone(),
            level: ad.level,
            map_id: ad.map_id,
            jobs: ad.jobs.iter().filter_map(|job| booking_job_name(*job)).collect(),
            started: ad.started,
        }
    }
}

impl Server {
    pub(crate) fn bot_social(&self, state: &ServerState, char_id: u32, request: &SocialRequest) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let script = |request| GameEvent::ScriptWorld(ScriptWorld { char_id, request });
        let social = |action| GameEvent::CharacterSocial(CharacterSocial { char_id, action });
        let event = match request {
            SocialRequest::Sit | SocialRequest::Stand => {
                require_active(character)?;
                if matches!(request, SocialRequest::Sit) {
                    GameEvent::CharacterSit(CharacterSit { char_id })
                } else {
                    GameEvent::CharacterStand(CharacterStand { char_id })
                }
            }
            SocialRequest::Emote(emote) => {
                let number = emote.number()?;
                GameEvent::CharacterClientCommand(CharacterClientCommand { char_id, command: ClientCommand::Emotion(number) })
            }
            SocialRequest::GuildCreate { name } => {
                let name = name.trim();
                if name.is_empty() || name.len() >= GUILD_NAME_SIZE {
                    return Err(format!("A guild name has 1 to {} characters", GUILD_NAME_SIZE - 1));
                }
                if character.game_systems.guild_id != 0 {
                    return Err("The character is already in a guild, leave it first".into());
                }
                script(ScriptWorldRequest::Guild(GuildRequest::CreateGuild(name.to_string())))
            }
            SocialRequest::GuildInvite { name } => {
                if character.game_systems.guild_id == 0 {
                    return Err("The character has no guild".into());
                }
                let invited = player_named(state, name)?;
                if invited.char_id == char_id {
                    return Err("A character can not invite itself".into());
                }
                if invited.game_systems.guild_id != 0 {
                    return Err(format!("{} is already in a guild", invited.name));
                }
                script(ScriptWorldRequest::Guild(GuildRequest::InviteGuild(invited.char_id)))
            }
            SocialRequest::GuildAnswer { accept } => {
                let invitation = character.game_systems.guild_invitation.as_ref().ok_or("No guild invitation is pending")?;
                script(ScriptWorldRequest::Guild(GuildRequest::AnswerGuildInvite { guild_id: invitation.guild_id, accept: *accept }))
            }
            SocialRequest::GuildLeave => {
                if character.game_systems.guild_id == 0 {
                    return Err("The character has no guild".into());
                }
                script(ScriptWorldRequest::Guild(GuildRequest::LeaveGuild { guild_id: character.game_systems.guild_id, reason: String::new() }))
            }
            SocialRequest::GuildChat(text) => {
                if character.game_systems.guild_id == 0 {
                    return Err("The character has no guild".into());
                }
                script(ScriptWorldRequest::Guild(GuildRequest::GuildMessage(format!("{} : {}", character.name, chat_text(text)?))))
            }
            SocialRequest::FriendAdd { name } => {
                let other = player_named(state, name)?;
                if other.char_id == char_id {
                    return Err("A character can not befriend itself".into());
                }
                if character.game_systems.friends.iter().any(|friend| friend.char_id == other.char_id) {
                    return Err(format!("{} is already a friend", other.name));
                }
                social(SocialAction::FriendRequest { name: other.name.clone() })
            }
            SocialRequest::FriendAnswer { accept, from } => {
                let (account_id, from_char) = from.ok_or("No friend request is pending")?;
                if character.game_systems.friend_request != from_char {
                    return Err("The friend request is no longer pending".into());
                }
                social(SocialAction::FriendReply { inviter_account_id: account_id, accepted: *accept })
            }
            SocialRequest::FriendRemove { name } => {
                let friend = character
                    .game_systems
                    .friends
                    .iter()
                    .find(|friend| state.get_character(friend.char_id).is_some_and(|other| other.name.eq_ignore_ascii_case(name.trim())))
                    .ok_or_else(|| format!("{} is not an online friend", name.trim()))?;
                social(SocialAction::FriendRemove { account_id: friend.account_id, char_id: friend.char_id })
            }
            SocialRequest::RoomCreate { title, limit, public, password } => {
                let title = chat_text(title)?;
                if title.len() > ROOM_TITLE_SIZE || password.len() > ROOM_PASSWORD_SIZE {
                    return Err(format!("A room title has at most {ROOM_TITLE_SIZE} characters, a password {ROOM_PASSWORD_SIZE}"));
                }
                if state.chat_rooms.room_of(char_id).is_some() {
                    return Err("The character is already in a chat room, leave it first".into());
                }
                social(SocialAction::CreateRoom { title, password: password.clone(), limit: (*limit).clamp(2, 20), public: *public })
            }
            SocialRequest::RoomJoin { npc: Some(npc), password, .. } => {
                let room = state.waiting_rooms.of_npc(*npc).ok_or_else(|| format!("NPC {npc} has no waiting room"))?;
                if room.map != *character.current_map_name() {
                    return Err(format!("The waiting room of NPC {npc} is on {}", room.map));
                }
                if state.chat_rooms.room_of(char_id).is_some() || state.waiting_rooms.room_of(char_id).is_some() {
                    return Err("The character is already in a chat room, leave it first".into());
                }
                social(SocialAction::EnterRoom { room_id: room.id, password: password.clone() })
            }
            SocialRequest::RoomJoin { owner, password, .. } => {
                let owner = player_named(state, owner)?;
                let room = state.chat_rooms.owned_by(owner.char_id).ok_or_else(|| format!("{} has no chat room", owner.name))?;
                if owner.map_instance_key != character.map_instance_key {
                    return Err(format!("{} is on another map", owner.name));
                }
                if state.chat_rooms.room_of(char_id).is_some() {
                    return Err("The character is already in a chat room, leave it first".into());
                }
                social(SocialAction::EnterRoom { room_id: room.id, password: password.clone() })
            }
            SocialRequest::RoomLeave => {
                if state.chat_rooms.room_of(char_id).is_none() && state.waiting_rooms.room_of(char_id).is_none() {
                    return Err("The character is in no chat room".into());
                }
                social(SocialAction::LeaveRoom)
            }
            SocialRequest::BookingRegister { level, map_id, jobs } => {
                if jobs.len() > BOOKING_JOBS {
                    return Err(format!("A booking lists {BOOKING_JOBS} jobs at most"));
                }
                let mut wanted = [NO_JOB; BOOKING_JOBS];
                for (slot, name) in wanted.iter_mut().zip(jobs) {
                    *slot = booking_job(name)?;
                }
                if self.script_world_service().party_bookings.ad_of(char_id).is_some() {
                    return Err("The character already has a booking, delete it first".into());
                }
                let level = booking_level(*level, character.status.base_level);
                script(ScriptWorldRequest::Booking(BookingRequest::BookingRegister { level, map_id: *map_id, jobs: wanted }))
            }
            SocialRequest::BookingSearch { level, map_id, job } => {
                let job = job.as_deref().map(booking_job).transpose()?.unwrap_or(-1);
                if *map_id != 0 && job != -1 {
                    return Err("A booking search filters by map or by job, not both".into());
                }
                let level = level.map_or(0, |level| booking_level(Some(level), 0));
                script(ScriptWorldRequest::Booking(BookingRequest::BookingSearch { level, map_id: *map_id, job, last_index: 0 }))
            }
            SocialRequest::BookingDelete => {
                if self.script_world_service().party_bookings.ad_of(char_id).is_none() {
                    return Err("The character has no booking".into());
                }
                script(ScriptWorldRequest::Booking(BookingRequest::BookingDelete))
            }
        };
        self.add_to_next_tick(event);
        Ok(json!({ "sent": true }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emotes_go_by_name_or_number() {
        assert_eq!(EmoteRef::Name("/thx".into()).number(), Ok(15));
        assert_eq!(EmoteRef::Name("OK".into()).number(), Ok(33));
        assert_eq!(EmoteRef::Number(2).number(), Ok(2));
        assert!(EmoteRef::Number(34).number().is_err());
        assert!(EmoteRef::Name("dance".into()).number().is_err());
        assert_eq!(emote_name(15), "thx");
        assert_eq!(emote_name(60), "60");
    }

    #[test]
    fn booking_jobs_read_both_spellings() {
        assert_eq!(booking_job("Swordman"), booking_job("Swordsman"));
        assert_eq!(booking_job_name(booking_job("Acolyte").unwrap()), Some("Acolyte".to_string()));
        assert_eq!(booking_job_name(NO_JOB), None);
        assert_eq!(booking_job_name(-1), None);
    }
}
