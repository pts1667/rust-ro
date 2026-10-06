use configuration::account_config::{
    CHAR_DEL_BIRTHDATE, CHAR_DEL_EMAIL, CHAR_DEL_RESTRICT_GUILD, CHAR_DEL_RESTRICT_PARTY, CharServerConfig, MAX_CHARS, StartItem,
};
use database::model::{AccountRecord, CharLogRecord, CharacterRecord, DEFAULT_ACCOUNT_EMAIL, InventoryRecord};
use database::{CharacterCreation, CreateCharacterError, CreationRefusal, RenameCharacterError};
use models::enums::EnumWithMaskValueU64;
use models::enums::item::EquipmentLocation;
use rand::Rng;

use crate::repository::Repository;

/// Result codes of `char_make_new_char`.
pub const CREATE_NAME_TAKEN: i32 = -1;
pub const CREATE_DENIED: i32 = -2;
pub const CREATE_SLOT_NOT_ELIGIBLE: i32 = -4;

const WEDDING_RING_M: i32 = 2634;
const WEDDING_RING_F: i32 = 2635;
const CALL_BABY_SKILL: u32 = 410;
const TRIMMED_NAME_CHARS: [char; 5] = [' ', '\t', '\n', '\r', '\u{1a}'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountContext {
    pub account_id: u32,
    pub sex: &'static str,
    pub char_slots: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateRequest {
    pub name: String,
    pub slot: i32,
    pub hair_style: i32,
    pub hair_color: i32,
    /// Strength, agility, vitality, intelligence, dexterity, luck; sent by clients older than 20120307.
    pub stats: Option<[i32; 6]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteOutcome {
    Deleted,
    NotFound,
    Database,
    BaseLevel,
    Guild,
    Party,
    TooEarly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeletionReservation {
    pub result: i32,
    pub date: u32,
}

pub fn normalize_name(name: &str) -> String {
    name.trim_matches(|c| TRIMMED_NAME_CHARS.contains(&c)).to_string()
}

pub fn max_hp(vit: i32) -> i32 {
    40 * (100 + vit) / 100
}

pub fn max_sp(int: i32) -> i32 {
    11 * (100 + int) / 100
}

/// `char_check_char_name`: `0` when the name can be used, `-1` when it is taken or reserved, `-2` when it is invalid.
pub fn check_char_name(repository: &dyn Repository, config: &CharServerConfig, name: &str) -> i32 {
    if name.is_empty() || name.len() < config.char_name_min_length || name.chars().any(|c| c.is_control()) {
        return CREATE_DENIED;
    }
    if name.eq_ignore_ascii_case(&config.wisp_server_name) {
        return CREATE_NAME_TAKEN;
    }
    if name.starts_with('#') {
        return CREATE_DENIED;
    }
    match config.char_name_option {
        1 if !name.chars().all(|c| config.char_name_letters.contains(c)) => return CREATE_DENIED,
        2 if name.chars().any(|c| config.char_name_letters.contains(c)) => return CREATE_DENIED,
        _ => {}
    }
    match repository.char_name_taken(name, config.name_ignoring_case) {
        Ok(false) => 0,
        Ok(true) => CREATE_NAME_TAKEN,
        Err(error) => {
            warn!("Character name lookup failed: {error}");
            CREATE_DENIED
        }
    }
}

fn start_item_record(item: &StartItem) -> InventoryRecord {
    InventoryRecord {
        item_id: item.item_id as i32,
        amount: item.amount.min(i16::MAX as u16) as i16,
        equip: item.equip as i32,
        is_identified: true,
        ..InventoryRecord::default()
    }
}

fn equipped_at(items: &[StartItem], location: EquipmentLocation) -> i16 {
    items
        .iter()
        .find(|item| u64::from(item.equip) & location.as_flag() != 0)
        .map_or(0, |item| item.item_id as i16)
}

fn log_char(repository: &dyn Repository, config: &CharServerConfig, account_id: u32, slot: i16, name: &str, message: &str, now: i64) {
    if !config.log_char {
        return;
    }
    let record = CharLogRecord { time: now, account_id, char_slot: slot, name: name.to_string(), message: message.to_string() };
    if let Err(error) = repository.char_log(record) {
        warn!("Failed to write the character log: {error}");
    }
}

/// `char_make_new_char`; `Err` carries the rathena result code.
pub fn create_character(
    repository: &dyn Repository,
    config: &CharServerConfig,
    max_inventory: u16,
    account: &AccountContext,
    packetver: u32,
    request: &CreateRequest,
    now: i64,
) -> Result<CharacterRecord, i32> {
    if !config.char_new {
        return Err(CREATE_DENIED);
    }
    let name = normalize_name(&request.name);
    let verdict = check_char_name(repository, config, &name);
    if verdict < 0 {
        return Err(verdict);
    }
    if request.slot < 0 || request.slot >= i32::from(account.char_slots) {
        return Err(CREATE_SLOT_NOT_ELIGIBLE);
    }
    let (stats, status_point) = if packetver >= 20120307 {
        ([1; 6], config.start_status_points as i16)
    } else {
        let stats = request.stats.ok_or(CREATE_DENIED)?;
        let [str, agi, vit, int, dex, luk] = stats;
        let in_range = stats.iter().all(|stat| (1..=9).contains(stat));
        if !in_range || stats.iter().sum::<i32>() != 30 || str + int != 10 || agi + luk != 10 || vit + dex != 10 {
            return Err(CREATE_DENIED);
        }
        (stats, 0)
    };
    let [str, agi, vit, int, dex, luk] = stats;
    let start = &config.start_point[rand::thread_rng().gen_range(0..config.start_point.len())];
    let hair_color = i16::try_from(request.hair_color).map_err(|_| CREATE_DENIED)?;
    let hair = i16::try_from(request.hair_style).map_err(|_| CREATE_DENIED)?;
    let character = CharacterRecord {
        account_id: account.account_id as i32,
        char_num: request.slot as i16,
        name: name.clone(),
        class: 0,
        zeny: i32::try_from(config.start_zeny).unwrap_or(i32::MAX),
        status_point,
        str: str as i16,
        agi: agi as i16,
        vit: vit as i16,
        int: int as i16,
        dex: dex as i16,
        luk: luk as i16,
        max_hp: max_hp(vit),
        hp: max_hp(vit),
        max_sp: max_sp(int),
        sp: max_sp(int),
        hair,
        hair_color,
        last_map: start.map.clone(),
        last_x: start.point.x as i16,
        last_y: start.point.y as i16,
        save_map: start.map.clone(),
        save_x: start.point.x as i16,
        save_y: start.point.y as i16,
        sex: account.sex.to_string(),
        inventory_slots: max_inventory as i16,
        base_level: 1,
        job_level: 1,
        weapon: equipped_at(&config.start_items, EquipmentLocation::HandRight),
        shield: equipped_at(&config.start_items, EquipmentLocation::HandLeft),
        head_top: equipped_at(&config.start_items, EquipmentLocation::HeadTop),
        head_mid: equipped_at(&config.start_items, EquipmentLocation::HeadMid),
        head_bottom: equipped_at(&config.start_items, EquipmentLocation::HeadLow),
        body: equipped_at(&config.start_items, EquipmentLocation::Armor),
        ..CharacterRecord::default()
    };
    let creation = CharacterCreation {
        character,
        items: config.start_items.iter().map(start_item_record).collect(),
        slot_limit: account.char_slots.min(MAX_CHARS),
        case_sensitive_names: config.name_ignoring_case,
    };
    match repository.char_create(&creation) {
        Ok(created) => {
            log_char(repository, config, account.account_id, created.char_num, &created.name, "make new char", now);
            info!("Created char: account: {}, char: {}, slot: {}, name: {}", account.account_id, created.char_id, created.char_num, created.name);
            Ok(created)
        }
        Err(CreateCharacterError::Refused(CreationRefusal::NameTaken)) => Err(CREATE_NAME_TAKEN),
        Err(CreateCharacterError::Refused(CreationRefusal::SlotNotAllowed)) => Err(CREATE_SLOT_NOT_ELIGIBLE),
        Err(CreateCharacterError::Refused(CreationRefusal::AccountFull | CreationRefusal::SlotInUse)) => Err(CREATE_DENIED),
        Err(CreateCharacterError::Database(error)) => {
            warn!("Character creation failed: {error}");
            Err(CREATE_DENIED)
        }
    }
}

fn owned_character(repository: &dyn Repository, account_id: u32, char_id: u32) -> Option<CharacterRecord> {
    repository
        .char_find(char_id)
        .ok()
        .flatten()
        .filter(|character| character.account_id == account_id as i32)
}

/// `0x0827`: queues the character for deletion. `relative_date` is true for the client versions whose reply carries seconds left.
pub fn reserve_deletion(
    repository: &dyn Repository,
    config: &CharServerConfig,
    account_id: u32,
    char_id: u32,
    now: i64,
    relative_date: bool,
) -> DeletionReservation {
    const DATABASE_ERROR: i32 = 3;
    let refuse = |result| DeletionReservation { result, date: 0 };
    let Some(character) = owned_character(repository, account_id, char_id) else {
        return refuse(DATABASE_ERROR);
    };
    if character.delete_date != 0 {
        return refuse(0);
    }
    let Ok(systems) = repository.character_game_systems(char_id) else {
        return refuse(DATABASE_ERROR);
    };
    if config.char_del_restriction & CHAR_DEL_RESTRICT_GUILD != 0 && systems.guild_id != 0 {
        return refuse(4);
    }
    if config.char_del_restriction & CHAR_DEL_RESTRICT_PARTY != 0 && systems.party_id != 0 {
        return refuse(5);
    }
    let date = u32::try_from(now + i64::from(config.char_del_delay)).unwrap_or(u32::MAX);
    if repository.char_update(char_id, &|stored| stored.delete_date = date).is_err() {
        return refuse(DATABASE_ERROR);
    }
    DeletionReservation { result: 1, date: if relative_date { config.char_del_delay } else { date } }
}

/// `0x082b`: `1` on success, `2` when the character is not on the account or storage failed.
pub fn cancel_deletion(repository: &dyn Repository, account_id: u32, char_id: u32) -> i32 {
    if owned_character(repository, account_id, char_id).is_none() {
        return 2;
    }
    match repository.char_update(char_id, &|stored| stored.delete_date = 0) {
        Ok(_) => 1,
        Err(_) => 2,
    }
}

fn account_birthdate(account: &AccountRecord) -> &str {
    if account.birthdate.is_empty() { "0000-00-00" } else { &account.birthdate }
}

/// `chclif_delchar_check`: whether `code` proves ownership under the enabled `flags`.
pub fn deletion_code_matches(account: &AccountRecord, code: &str, flags: u8) -> bool {
    if flags & CHAR_DEL_EMAIL != 0
        && (code.eq_ignore_ascii_case(&account.email) || (account.email.eq_ignore_ascii_case(DEFAULT_ACCOUNT_EMAIL) && code.is_empty()))
    {
        return true;
    }
    flags & CHAR_DEL_BIRTHDATE != 0 && (account_birthdate(account).get(2..) == Some(code) || (account.birthdate.is_empty() && code.is_empty()))
}

/// `0x0829`: confirms the deletion with the birth date; returns the `HC_DELETE_CHAR3` result code.
pub fn accept_deletion(
    repository: &dyn Repository,
    config: &CharServerConfig,
    account: &AccountRecord,
    char_id: u32,
    birthdate: [u8; 6],
    now: i64,
) -> i32 {
    let digits: String = birthdate.iter().map(|byte| *byte as char).collect();
    let formatted = format!("{}-{}-{}", &digits[0..2], &digits[2..4], &digits[4..6]);
    if !deletion_code_matches(account, &formatted, CHAR_DEL_BIRTHDATE) {
        return 5;
    }
    match delete_character(repository, config, account.account_id, char_id, now) {
        DeleteOutcome::Deleted => 1,
        DeleteOutcome::Database | DeleteOutcome::NotFound => 3,
        DeleteOutcome::Party | DeleteOutcome::Guild | DeleteOutcome::BaseLevel => 2,
        DeleteOutcome::TooEarly => 4,
    }
}

/// `char_delete`: checks the deletion rules, detaches the character from its party, guild and family, then erases it.
pub fn delete_character(repository: &dyn Repository, config: &CharServerConfig, account_id: u32, char_id: u32, now: i64) -> DeleteOutcome {
    let Some(character) = owned_character(repository, account_id, char_id) else {
        return DeleteOutcome::NotFound;
    };
    let level = character.base_level;
    if (config.char_del_level > 0 && level >= config.char_del_level) || (config.char_del_level < 0 && level <= -config.char_del_level) {
        return DeleteOutcome::BaseLevel;
    }
    let Ok(systems) = repository.character_game_systems(char_id) else {
        return DeleteOutcome::Database;
    };
    if config.char_del_restriction & CHAR_DEL_RESTRICT_GUILD != 0 && systems.guild_id != 0 {
        return DeleteOutcome::Guild;
    }
    if config.char_del_restriction & CHAR_DEL_RESTRICT_PARTY != 0 && systems.party_id != 0 {
        return DeleteOutcome::Party;
    }
    if config.char_del_delay > 0 && (character.delete_date == 0 || i64::from(character.delete_date) > now) {
        return DeleteOutcome::TooEarly;
    }
    match detach_and_purge(repository, account_id, char_id, &systems) {
        Ok(()) => {
            log_char(repository, config, account_id, 0, &character.name, &format!("Deleted char (CID {char_id})"), now);
            info!("Deleted char: account: {account_id}, char: {char_id}, name: {}", character.name);
            DeleteOutcome::Deleted
        }
        Err(error) => {
            warn!("Char deletion aborted: {}, {error}", character.name);
            DeleteOutcome::Database
        }
    }
}

fn detach_and_purge(
    repository: &dyn Repository,
    account_id: u32,
    char_id: u32,
    systems: &crate::server::model::game_systems::CharacterGameSystems,
) -> Result<(), crate::repository::Error> {
    if systems.partner_id != 0 {
        repository.divorce_character(char_id)?;
        for id in [char_id, systems.partner_id] {
            repository.char_remove_items(id, &[WEDDING_RING_M, WEDDING_RING_F])?;
        }
    }
    for parent in [systems.father_id, systems.mother_id].into_iter().filter(|parent| *parent != 0) {
        let mut state = repository.character_game_systems(parent)?;
        state.child_id = 0;
        state.permanent_skill_grants.remove(&CALL_BABY_SKILL);
        repository.save_character_game_systems(parent, &state)?;
    }
    if systems.party_id != 0 {
        repository.leave_party(char_id, None)?;
    }
    if systems.guild_id != 0 {
        match repository.guild(systems.guild_id)? {
            Some(guild) if guild.master_char_id == char_id => {
                repository.break_guild(guild.id)?;
            }
            Some(guild) => {
                repository.leave_guild(char_id, guild.id, None)?;
            }
            None => {}
        }
    }
    repository.clear_character_stores(char_id)?;
    repository.char_purge(account_id, char_id)
}

/// `0x08fc`: `HC_ACK_CHANGE_CHARNAME` result codes; `None` when the character is not on the account (no reply).
pub fn rename_character(
    repository: &dyn Repository,
    config: &CharServerConfig,
    account_id: u32,
    char_id: u32,
    requested_name: &str,
    now: i64,
) -> Option<i16> {
    let character = owned_character(repository, account_id, char_id)?;
    let name = normalize_name(requested_name);
    if name.is_empty() {
        return Some(2);
    }
    if name == character.name {
        return Some(0);
    }
    if character.rename == 0 {
        return Some(1);
    }
    let systems = repository.character_game_systems(char_id).ok()?;
    if !config.char_rename_party && systems.party_id != 0 {
        return Some(6);
    }
    if !config.char_rename_guild && systems.guild_id != 0 {
        return Some(5);
    }
    match check_char_name(repository, config, &name) {
        0 => {}
        CREATE_NAME_TAKEN => return Some(4),
        _ => return Some(8),
    }
    match repository.char_rename(char_id, &name, config.name_ignoring_case) {
        Ok(_) => {
            log_char(repository, config, account_id, character.char_num, &name, "change char name", now);
            Some(0)
        }
        Err(RenameCharacterError::NameTaken) => Some(4),
        Err(RenameCharacterError::Database(error)) => {
            warn!("Character rename failed: {error}");
            Some(3)
        }
    }
}

/// `0x08d4`: moves or exchanges character slots; returns the moves left of the moved character on success.
pub fn move_character_slot(
    repository: &dyn Repository,
    config: &CharServerConfig,
    account: &AccountContext,
    from: u16,
    to: u16,
) -> Option<u32> {
    if from >= u16::from(MAX_CHARS) || to >= u16::from(MAX_CHARS) || to >= u16::from(account.char_slots) || !config.char_move_enabled {
        return None;
    }
    let characters = repository.char_account_characters(account.account_id).ok()?;
    let moving = characters.iter().find(|character| character.char_num == from as i16)?;
    if !config.char_moves_unlimited && moving.moves == 0 {
        return None;
    }
    if !repository.char_move_slot(account.account_id, from as i16, to as i16, config.char_movetoused).ok()? {
        return None;
    }
    let moves = if config.char_moves_unlimited {
        moving.moves
    } else {
        repository
            .char_update(moving.char_id as u32, &|stored| stored.moves = stored.moves.saturating_sub(1))
            .map_or(moving.moves.saturating_sub(1), |stored| stored.moves)
    };
    Some(moves)
}

#[cfg(test)]
mod tests {
    use database::model::AccountRecord;

    use super::*;
    use crate::repository::{CharServerRepository, GameSystemRepository, LoginRepository, SledRepository};

    fn account(repository: &SledRepository) -> AccountContext {
        let id = repository.account_create(AccountRecord::new(0, "player", "secret")).unwrap();
        AccountContext { account_id: id, sex: "F", char_slots: 3 }
    }

    fn request(name: &str, slot: i32) -> CreateRequest {
        CreateRequest { name: name.into(), slot, hair_style: 4, hair_color: 2, stats: None }
    }

    fn create(repository: &SledRepository, account: &AccountContext, name: &str, slot: i32) -> CharacterRecord {
        create_character(repository, &CharServerConfig::default(), 100, account, 20120307, &request(name, slot), 1000).unwrap()
    }

    #[test]
    fn creation_uses_start_values_account_sex_and_the_pre_renewal_hp_sp_formulas() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let created = create(&repository, &account, "  Hero  ", 0);
        assert_eq!(created.name, "Hero", "surrounding blanks are trimmed");
        assert_eq!((created.sex.as_str(), created.status_point, created.zeny), ("F", 48, 0));
        assert_eq!((created.str, created.vit, created.int), (1, 1, 1));
        assert_eq!((created.max_hp, created.max_sp), (40 * 101 / 100, 11 * 101 / 100));
        assert_eq!((created.last_map.as_str(), created.last_x, created.last_y), ("new_1-1", 53, 111));
        assert_eq!((created.weapon, created.body), (1201, 2301));
        let items: Vec<InventoryRecord> = database::required(&repository.database.inventories, &created.char_id.to_be_bytes()).unwrap();
        assert_eq!(items.iter().map(|item| (item.item_id, item.equip)).collect::<Vec<_>>(), vec![(1201, 2), (2301, 16)]);
        assert!(repository.database.char_log.len() == 1);
    }

    #[test]
    fn older_clients_must_send_valid_stat_distributions() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let config = CharServerConfig::default();
        let create_old = |name: &str, slot, stats| {
            let request = CreateRequest { stats: Some(stats), ..request(name, slot) };
            create_character(&repository, &config, 100, &account, 20120229, &request, 1)
        };
        let created = create_old("Hero", 0, [5, 5, 5, 5, 5, 5]).unwrap();
        assert_eq!((created.status_point, created.str, created.vit), (0, 5, 5));
        assert_eq!(create_old("Second", 1, [8, 2, 5, 3, 5, 7]).unwrap_err(), CREATE_DENIED, "strength + intelligence must be ten");
        assert_eq!(create_old("Second", 1, [10, 5, 5, 0, 5, 5]).unwrap_err(), CREATE_DENIED, "no stat outside 1..=9");
        assert_eq!(create_old("Second", 1, [5, 5, 5, 5, 5, 6]).unwrap_err(), CREATE_DENIED, "the total must be thirty");
        assert!(create_old("Second", 1, [9, 1, 5, 1, 5, 9]).is_ok());
        let missing = create_character(&repository, &config, 100, &account, 20120229, &request("Third", 2), 1);
        assert_eq!(missing.unwrap_err(), CREATE_DENIED);
    }

    #[test]
    fn creation_enforces_name_rules() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let config = CharServerConfig::default();
        let attempt = |name: &str| create_character(&repository, &config, 100, &account, 20120307, &request(name, 0), 1);
        assert_eq!(attempt("abc").unwrap_err(), CREATE_DENIED, "shorter than char_name_min_length");
        assert_eq!(attempt("#Chan").unwrap_err(), CREATE_DENIED);
        assert_eq!(attempt("Bad!Name").unwrap_err(), CREATE_DENIED, "outside char_name_letters");
        assert_eq!(attempt("Bad\u{7}Name").unwrap_err(), CREATE_DENIED);
        assert_eq!(attempt("SERVER").unwrap_err(), CREATE_NAME_TAKEN, "the wisp name is reserved");
        assert!(attempt("Hero").is_ok());
        assert_eq!(attempt("HERO").unwrap_err(), CREATE_NAME_TAKEN, "names are unique ignoring case by default");
        let forbidden = CharServerConfig { char_name_option: 2, char_name_letters: "xyz".into(), ..CharServerConfig::default() };
        assert_eq!(check_char_name(&repository, &forbidden, "Maxim"), CREATE_DENIED);
        assert_eq!(check_char_name(&repository, &forbidden, "Hello"), 0);
        let any = CharServerConfig { char_name_option: 0, ..CharServerConfig::default() };
        assert_eq!(check_char_name(&repository, &any, "Bad!Name"), 0);
    }

    #[test]
    fn creation_enforces_slots_capacity_and_the_new_character_switch() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let config = CharServerConfig::default();
        let attempt = |name: &str, slot| create_character(&repository, &config, 100, &account, 20120307, &request(name, slot), 1);
        assert_eq!(attempt("Hero", 3).unwrap_err(), CREATE_SLOT_NOT_ELIGIBLE, "account only has three slots");
        assert_eq!(attempt("Hero", -1).unwrap_err(), CREATE_SLOT_NOT_ELIGIBLE);
        assert!(attempt("Hero", 0).is_ok());
        assert_eq!(attempt("Other", 0).unwrap_err(), CREATE_DENIED, "slot in use");
        let closed = CharServerConfig { char_new: false, ..CharServerConfig::default() };
        let refused = create_character(&repository, &closed, 100, &account, 20120307, &request("Fresh", 1), 1);
        assert_eq!(refused.unwrap_err(), CREATE_DENIED);
    }

    #[test]
    fn deletion_is_queued_then_confirmed_with_the_birth_date() {
        let repository = SledRepository::temporary().unwrap();
        let account_ctx = account(&repository);
        let hero = create(&repository, &account_ctx, "Hero", 0);
        let char_id = hero.char_id as u32;
        let config = CharServerConfig::default();
        let reservation = reserve_deletion(&repository, &config, account_ctx.account_id, char_id, 1_000, false);
        assert_eq!(reservation, DeletionReservation { result: 1, date: 1_000 + 86_400 });
        assert_eq!(reserve_deletion(&repository, &config, account_ctx.account_id, char_id, 1_001, false).result, 0, "already queued");
        let account = repository.account_by_id(account_ctx.account_id).unwrap().unwrap();
        assert_eq!(accept_deletion(&repository, &config, &account, char_id, *b"000000", 1_100), 4, "delay not over");
        assert_eq!(accept_deletion(&repository, &config, &account, char_id, *b"991231", 100_000), 5, "wrong birth date");
        assert_eq!(accept_deletion(&repository, &config, &account, char_id, *b"000000", 100_000), 1);
        assert!(repository.char_find(char_id).unwrap().is_none());
        assert_eq!(accept_deletion(&repository, &config, &account, char_id, *b"000000", 100_000), 3, "gone");
        assert!(repository.database.char_log.len() == 2);
    }

    #[test]
    fn relative_clients_receive_the_delay_instead_of_the_date() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        let config = CharServerConfig { char_del_delay: 600, ..CharServerConfig::default() };
        assert_eq!(reserve_deletion(&repository, &config, account.account_id, hero.char_id as u32, 5_000, true).date, 600);
    }

    #[test]
    fn deletion_can_be_cancelled_and_only_for_own_characters() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        let id = hero.char_id as u32;
        let config = CharServerConfig::default();
        assert_eq!(reserve_deletion(&repository, &config, account.account_id + 1, id, 1, false).result, 3);
        reserve_deletion(&repository, &config, account.account_id, id, 1, false);
        assert_eq!(cancel_deletion(&repository, account.account_id + 1, id), 2);
        assert_eq!(cancel_deletion(&repository, account.account_id, id), 1);
        assert_eq!(repository.char_find(id).unwrap().unwrap().delete_date, 0);
    }

    #[test]
    fn party_and_guild_membership_block_deletion_until_the_restriction_is_lifted() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        let id = hero.char_id as u32;
        let mut systems = repository.character_game_systems(id).unwrap();
        systems.guild_id = 9;
        repository.save_character_game_systems(id, &systems).unwrap();
        let config = CharServerConfig::default();
        assert_eq!(reserve_deletion(&repository, &config, account.account_id, id, 1, false).result, 4);
        assert_eq!(delete_character(&repository, &config, account.account_id, id, 1_000_000), DeleteOutcome::Guild);
        let party_only = CharServerConfig { char_del_restriction: CHAR_DEL_RESTRICT_PARTY, char_del_delay: 0, ..CharServerConfig::default() };
        assert_eq!(delete_character(&repository, &party_only, account.account_id, id, 1), DeleteOutcome::Deleted, "a dangling guild id does not block deletion");
    }

    #[test]
    fn level_restrictions_follow_char_del_level() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        let id = hero.char_id as u32;
        repository.char_update(id, &|stored| stored.base_level = 50).unwrap();
        let instant = |level| CharServerConfig { char_del_level: level, char_del_delay: 0, ..CharServerConfig::default() };
        assert_eq!(delete_character(&repository, &instant(40), account.account_id, id, 1), DeleteOutcome::BaseLevel, "level 50 >= 40");
        assert_eq!(delete_character(&repository, &instant(-60), account.account_id, id, 1), DeleteOutcome::BaseLevel, "level 50 <= 60");
        assert_eq!(delete_character(&repository, &instant(-40), account.account_id, id, 1), DeleteOutcome::Deleted);
    }

    #[test]
    fn deletion_without_delay_works_immediately_and_forgets_the_inventory() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        let config = CharServerConfig { char_del_delay: 0, ..CharServerConfig::default() };
        assert_eq!(delete_character(&repository, &config, account.account_id, hero.char_id as u32, 1), DeleteOutcome::Deleted);
        assert!(repository.database.inventory_owners.is_empty());
        assert_eq!(delete_character(&repository, &config, account.account_id, hero.char_id as u32, 1), DeleteOutcome::NotFound);
    }

    #[test]
    fn marriage_and_adoption_are_undone_when_a_member_is_deleted() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let husband = create(&repository, &account, "Husband", 0);
        let wife = create(&repository, &account, "Wife", 1);
        let baby = create(&repository, &account, "Babyone", 2);
        let (h, w, b) = (husband.char_id as u32, wife.char_id as u32, baby.char_id as u32);
        repository.marry_characters(h, w).unwrap();
        repository.adopt_character(h, w, b).unwrap();
        let ring = InventoryRecord { id: 777, item_id: WEDDING_RING_M, amount: 1, ..Default::default() };
        let mut items: Vec<InventoryRecord> = database::required(&repository.database.inventories, &husband.char_id.to_be_bytes()).unwrap();
        items.push(ring);
        repository.database.inventories.insert(husband.char_id.to_be_bytes(), serde_json::to_vec(&items).unwrap()).unwrap();
        let config = CharServerConfig { char_del_delay: 0, ..CharServerConfig::default() };

        assert_eq!(delete_character(&repository, &config, account.account_id, b, 1), DeleteOutcome::Deleted);
        assert_eq!(repository.character_game_systems(h).unwrap().child_id, 0);
        assert_eq!(repository.character_game_systems(w).unwrap().child_id, 0);
        assert_eq!(repository.character_game_systems(h).unwrap().partner_id, w, "still married");

        assert_eq!(delete_character(&repository, &config, account.account_id, h, 1), DeleteOutcome::Deleted);
        assert_eq!(repository.character_game_systems(w).unwrap().partner_id, 0);
    }

    #[test]
    fn legacy_deletion_codes_check_email_and_birth_date_per_char_del_option() {
        let mut account = AccountRecord::new(5, "player", "secret");
        assert!(deletion_code_matches(&account, "", CHAR_DEL_EMAIL), "default e-mail accepts an empty code");
        assert!(!deletion_code_matches(&account, "x@y.z", CHAR_DEL_EMAIL));
        account.email = "me@example.com".into();
        assert!(deletion_code_matches(&account, "ME@example.com", CHAR_DEL_EMAIL), "case-insensitive");
        assert!(!deletion_code_matches(&account, "", CHAR_DEL_EMAIL));
        account.birthdate = "1990-05-17".into();
        assert!(deletion_code_matches(&account, "90-05-17", CHAR_DEL_BIRTHDATE));
        assert!(!deletion_code_matches(&account, "me@example.com", CHAR_DEL_BIRTHDATE));
        assert!(deletion_code_matches(&account, "me@example.com", CHAR_DEL_EMAIL | CHAR_DEL_BIRTHDATE));
        let blank = AccountRecord::new(6, "other", "secret");
        assert!(deletion_code_matches(&blank, "", CHAR_DEL_BIRTHDATE));
    }

    #[test]
    fn rename_follows_the_rathena_result_codes() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        create(&repository, &account, "Taken", 1);
        let id = hero.char_id as u32;
        let config = CharServerConfig::default();
        let rename = |name: &str| rename_character(&repository, &config, account.account_id, id, name, 9);
        assert_eq!(rename("Hero"), Some(0), "unchanged name");
        assert_eq!(rename("Legend"), Some(1), "no rename available");
        repository.char_update(id, &|stored| stored.rename = 1).unwrap();
        assert_eq!(rename(""), Some(2));
        assert_eq!(rename("Taken"), Some(4));
        assert_eq!(rename("Bad!Name"), Some(8));
        assert_eq!(rename("Legend"), Some(0));
        assert_eq!(repository.char_find(id).unwrap().unwrap().name, "Legend");
        assert_eq!(rename_character(&repository, &config, account.account_id + 1, id, "Other", 9), None);
        assert_eq!(rename("Another"), Some(1), "the rename was spent");
    }

    #[test]
    fn rename_is_blocked_in_a_party_or_guild_unless_allowed() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let hero = create(&repository, &account, "Hero", 0);
        let id = hero.char_id as u32;
        repository.char_update(id, &|stored| stored.rename = 1).unwrap();
        let mut systems = repository.character_game_systems(id).unwrap();
        systems.party_id = 3;
        repository.save_character_game_systems(id, &systems).unwrap();
        let config = CharServerConfig::default();
        assert_eq!(rename_character(&repository, &config, account.account_id, id, "Legend", 1), Some(6));
        systems = repository.character_game_systems(id).unwrap();
        systems.party_id = 0;
        systems.guild_id = 4;
        repository.save_character_game_systems(id, &systems).unwrap();
        assert_eq!(rename_character(&repository, &config, account.account_id, id, "Legend", 1), Some(5));
        let allowed = CharServerConfig { char_rename_guild: true, ..CharServerConfig::default() };
        assert_eq!(rename_character(&repository, &allowed, account.account_id, id, "Legend", 1), Some(0));
    }

    #[test]
    fn slot_moves_need_moves_left_unless_unlimited_and_swap_only_when_allowed() {
        let repository = SledRepository::temporary().unwrap();
        let account = account(&repository);
        let first = create(&repository, &account, "Aaaa", 0);
        let second = create(&repository, &account, "Bbbb", 1);
        let config = CharServerConfig::default();
        assert_eq!(move_character_slot(&repository, &config, &account, 0, 2), None, "no moves granted");
        repository.char_update(first.char_id as u32, &|stored| stored.moves = 2).unwrap();
        assert_eq!(move_character_slot(&repository, &config, &account, 0, 2), Some(1));
        assert_eq!(repository.char_find(first.char_id as u32).unwrap().unwrap().char_num, 2);
        assert_eq!(move_character_slot(&repository, &config, &account, 2, 3), None, "slot 3 is beyond the account slots");
        let no_swap = CharServerConfig { char_movetoused: false, ..CharServerConfig::default() };
        assert_eq!(move_character_slot(&repository, &no_swap, &account, 2, 1), None);
        assert_eq!(move_character_slot(&repository, &config, &account, 2, 1), Some(0), "swaps with the character in slot 1");
        assert_eq!(repository.char_find(second.char_id as u32).unwrap().unwrap().char_num, 2);
        assert_eq!(move_character_slot(&repository, &config, &account, 1, 0), None, "out of moves");
        let unlimited = CharServerConfig { char_moves_unlimited: true, ..CharServerConfig::default() };
        assert_eq!(move_character_slot(&repository, &unlimited, &account, 1, 0), Some(0));
        let disabled = CharServerConfig { char_move_enabled: false, char_moves_unlimited: true, ..CharServerConfig::default() };
        assert_eq!(move_character_slot(&repository, &disabled, &account, 0, 1), None);
        assert_eq!(move_character_slot(&repository, &unlimited, &account, 12, 0), None);
    }
}
