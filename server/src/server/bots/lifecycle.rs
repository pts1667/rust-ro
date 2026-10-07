//! Accounts and characters of bots in the database. Every function blocks on the database, call them from a blocking thread.
use database::model::AccountRecord;
use rand::Rng;

use crate::repository::model::char_model::CharSelectModel;
use crate::server::Server;
use crate::server::request_handler::char::load_character;
use crate::server::service::char_server_service::{
    AccountContext, CREATE_DENIED, CREATE_NAME_TAKEN, CreateRequest, create_character,
};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;

const MAX_NAME_LENGTH: usize = 19;
const ACCOUNT_PREFIX: &str = "bot_";
const BOT_SLOT: i32 = 0;
const INVALID_NAME: i32 = -3;
const HAIR_STYLES: std::ops::RangeInclusive<i32> = 0..=23;
const HAIR_COLORS: std::ops::RangeInclusive<i32> = 0..=8;
/// What the client sends in its login answer: 0 for a woman, 1 for a man.
const CLIENT_SEX_MALE: u8 = 1;
const CLIENT_SEX_FEMALE: u8 = 0;

pub struct StoredBot {
    pub account_id: u32,
    pub character: CharSelectModel,
}

/// The look of a new bot, the rest of a character is the same for everyone.
pub struct NewBot {
    pub name: String,
    pub female: bool,
    pub hair_style: i32,
    pub hair_color: i32,
}

pub enum CreateError {
    AlreadyExists,
    Refused(String),
}

impl From<String> for CreateError {
    fn from(message: String) -> Self {
        CreateError::Refused(message)
    }
}

/// Bot accounts are told apart from the ones of players by their name, nobody knows their random password.
fn account_name(bot: &str) -> String {
    format!("{ACCOUNT_PREFIX}{}", bot.trim().to_lowercase().replace(' ', "_"))
}

fn refusal(code: i32) -> String {
    match code {
        CREATE_NAME_TAKEN => "A character with this name already exists".into(),
        INVALID_NAME => "The name is too short or has characters that character names can not have".into(),
        CREATE_DENIED => "The server refused the name: it is too short, has characters that are not allowed, or character creation is disabled".into(),
        other => format!("Character creation was refused (code {other})"),
    }
}

fn database_error(error: impl ToString) -> String {
    error.to_string()
}

fn living_character(server: &Server, account_id: u32) -> Result<Option<CharSelectModel>, String> {
    Ok(server
        .repository
        .as_ref()
        .char_account_characters(account_id)
        .map_err(database_error)?
        .into_iter()
        .find(|character| character.delete_date == 0))
}

/// The bot called `name` that an earlier call created, if there is one.
pub fn find_stored_bot(server: &Server, name: &str) -> Result<Option<StoredBot>, String> {
    let Some(account) = server.repository.as_ref().account_by_name(&account_name(name)).map_err(database_error)? else {
        return Ok(None);
    };
    Ok(living_character(server, account.account_id)?.map(|character| StoredBot { account_id: account.account_id, character }))
}

/// Creates the account and the character of a bot, which starts at `bots.start_map`.
pub fn create_stored_bot(server: &Server, new_bot: &NewBot) -> Result<StoredBot, CreateError> {
    let name = new_bot.name.trim();
    if name.is_empty() || name.chars().count() > MAX_NAME_LENGTH {
        return Err(format!("The name of a bot has between 1 and {MAX_NAME_LENGTH} characters").into());
    }
    if !HAIR_STYLES.contains(&new_bot.hair_style) || !HAIR_COLORS.contains(&new_bot.hair_color) {
        return Err(format!("The hair style goes from {} to {}, the hair color from {} to {}", HAIR_STYLES.start(), HAIR_STYLES.end(), HAIR_COLORS.start(), HAIR_COLORS.end()).into());
    }
    let repository = server.repository.as_ref();
    let username = account_name(name);
    let sex = if new_bot.female { "F" } else { "M" };
    // An account without character is what an earlier refused creation left behind
    let account_id = match repository.account_by_name(&username).map_err(database_error)? {
        Some(account) => account.account_id,
        None => {
            let mut account = AccountRecord::new(0, &username, format!("{:032x}", rand::thread_rng().gen::<u128>()));
            account.sex = sex.into();
            repository.account_create(account).map_err(database_error)?
        }
    };
    if living_character(server, account_id)?.is_some() {
        return Err(CreateError::AlreadyExists);
    }
    let configuration = server.configuration;
    let account = AccountContext { account_id, sex, char_slots: configuration::account_config::MAX_CHARS };
    let request = CreateRequest { name: name.to_string(), slot: BOT_SLOT, hair_style: new_bot.hair_style, hair_color: new_bot.hair_color, stats: None };
    let created = create_character(
        repository,
        &configuration.char_server,
        configuration.game.max_inventory,
        &account,
        server.packetver(),
        &request,
        chrono::Utc::now().timestamp(),
    )
    .map_err(|code| CreateError::Refused(refusal(code)))?;
    let bots = &configuration.bots;
    if GlobalConfigService::instance().find_map(&bots.start_map).is_none() {
        warn!("bots.start_map {} is not a loaded map, bot {name} starts at the character server start point", bots.start_map);
        return Ok(StoredBot { account_id, character: created });
    }
    let character = repository
        .char_update(created.char_id as u32, &|character| {
            character.last_map = bots.start_map.clone();
            character.save_map = bots.start_map.clone();
            character.last_x = bots.start_x as i16;
            character.last_y = bots.start_y as i16;
            character.save_x = bots.start_x as i16;
            character.save_y = bots.start_y as i16;
        })
        .map_err(database_error)?;
    Ok(StoredBot { account_id, character })
}

pub fn load_bot_character(server: &Server, stored: &StoredBot) -> Result<Character, String> {
    let sex = if stored.character.sex == "F" { CLIENT_SEX_FEMALE } else { CLIENT_SEX_MALE };
    load_character(server, stored.account_id, sex, &stored.character)
}
