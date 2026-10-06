use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Timelike};
use database::model::{AccountRecord, CharacterRecord};

use crate::repository::Repository;

/// Account state set by `@block` and cleared by `@unblock`.
pub const ACCOUNT_STATE_BLOCKED: u32 = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdminError {
    UnknownCharacter,
    UnknownAccount,
    NotBanned,
    InvalidTime,
    /// The adjusted date would not be in the future.
    UnbansTarget,
    Storage(String),
}

impl From<crate::repository::Error> for AdminError {
    fn from(error: crate::repository::Error) -> Self {
        Self::Storage(error.to_string())
    }
}

/// `solve_time`: seconds from `now` to the date a modifier such as `+1d2h`, `30mn` or `1y` leads to, `0` for no modifier.
/// Fields are added the way `struct tm` does and normalised afterwards, so `+1m` on the 31st may land in the next month.
pub fn solve_time(modifier: &str, now: chrono::DateTime<Local>) -> i64 {
    let bytes = modifier.as_bytes();
    let (mut seconds, mut minutes, mut hours, mut days, mut months, mut years) = (0i64, 0i64, 0i64, 0i64, 0i64, 0i64);
    let mut index = 0;
    while index < bytes.len() {
        let value = leading_integer(&bytes[index..]);
        if value == 0 {
            index += 1;
            continue;
        }
        if matches!(bytes[index], b'-' | b'+') {
            index += 1;
        }
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }
        let Some(unit) = bytes.get(index).copied() else {
            break;
        };
        index += 1;
        match unit {
            b's' => seconds += value,
            b'n' => minutes += value,
            b'm' if bytes.get(index) == Some(&b'n') => {
                minutes += value;
                index += 1;
            }
            b'h' => hours += value,
            b'd' | b'j' => days += value,
            b'm' => months += value,
            b'y' | b'a' => years += value,
            _ => {}
        }
    }
    let month_index = i64::from(now.month0()) + months + years * 12;
    let year = i64::from(now.year()) + month_index.div_euclid(12);
    let month = month_index.rem_euclid(12) as u32 + 1;
    let Some(first) = NaiveDate::from_ymd_opt(year as i32, month, 1) else {
        return 0;
    };
    let date = first + Duration::days(i64::from(now.day()) - 1 + days);
    let time = Duration::seconds(
        i64::from(now.hour()) * 3600 + i64::from(now.minute()) * 60 + i64::from(now.second()) + hours * 3600 + minutes * 60 + seconds,
    );
    let then = date.and_hms_opt(0, 0, 0).expect("midnight exists") + time;
    Local.from_local_datetime(&then).earliest().map_or(0, |then| then.timestamp() - now.timestamp())
}

fn leading_integer(bytes: &[u8]) -> i64 {
    let mut index = 0;
    let negative = match bytes.first() {
        Some(b'-') => {
            index = 1;
            true
        }
        Some(b'+') => {
            index = 1;
            false
        }
        _ => false,
    };
    let mut value: i64 = 0;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        value = value.saturating_mul(10).saturating_add(i64::from(bytes[index] - b'0'));
        index += 1;
    }
    if negative { -value } else { value }
}

pub fn parse_ban_arguments(arguments: &str) -> Option<(&str, &str)> {
    let arguments = arguments.trim();
    let (time, name) = arguments.split_once(char::is_whitespace)?;
    let name = name.trim();
    (!time.is_empty() && !name.is_empty()).then_some((time, name))
}

fn new_ban_date(current: i64, timediff: i64, now: i64) -> Result<i64, AdminError> {
    if timediff == 0 {
        return Err(AdminError::InvalidTime);
    }
    let base = if current == 0 || current < now { now } else { current };
    let date = base + timediff;
    if date <= now { Err(AdminError::UnbansTarget) } else { Ok(date) }
}

fn character_named(repository: &dyn Repository, name: &str) -> Result<CharacterRecord, AdminError> {
    repository.char_find_by_name(name)?.ok_or(AdminError::UnknownCharacter)
}

fn account_of(repository: &dyn Repository, character: &CharacterRecord) -> Result<AccountRecord, AdminError> {
    repository.account_by_id(character.account_id as u32)?.ok_or(AdminError::UnknownAccount)
}

/// `@ban`: extends the account ban of the character's owner. Returns the owner and the new ban end.
pub fn ban_account(repository: &dyn Repository, name: &str, timediff: i64, now: i64) -> Result<(AccountRecord, i64), AdminError> {
    let account = account_of(repository, &character_named(repository, name)?)?;
    let until = new_ban_date(account.unban_time, timediff, now)?;
    let account = repository.account_update(account.account_id, &|stored| stored.unban_time = until)?;
    Ok((account, until))
}

/// `@unban`: lifts the account ban of the character's owner.
pub fn unban_account(repository: &dyn Repository, name: &str) -> Result<AccountRecord, AdminError> {
    let account = account_of(repository, &character_named(repository, name)?)?;
    if account.unban_time == 0 {
        return Err(AdminError::NotBanned);
    }
    Ok(repository.account_update(account.account_id, &|stored| stored.unban_time = 0)?)
}

/// `@block` / `@unblock`: sets or clears the account state that refuses logins.
pub fn set_account_blocked(repository: &dyn Repository, name: &str, blocked: bool) -> Result<AccountRecord, AdminError> {
    let account = account_of(repository, &character_named(repository, name)?)?;
    let state = if blocked { ACCOUNT_STATE_BLOCKED } else { 0 };
    Ok(repository.account_update(account.account_id, &|stored| stored.state = state)?)
}

/// `@charban`: extends the ban of a single character. Returns the character and the new ban end.
pub fn ban_character(repository: &dyn Repository, name: &str, timediff: i64, now: i64) -> Result<(CharacterRecord, i64), AdminError> {
    let character = character_named(repository, name)?;
    let until = new_ban_date(character.unban_time, timediff, now)?;
    let character = repository.char_update(character.char_id as u32, &|stored| stored.unban_time = until)?;
    Ok((character, until))
}

pub fn unban_character(repository: &dyn Repository, name: &str) -> Result<CharacterRecord, AdminError> {
    let character = character_named(repository, name)?;
    if character.unban_time == 0 {
        return Err(AdminError::NotBanned);
    }
    Ok(repository.char_update(character.char_id as u32, &|stored| stored.unban_time = 0)?)
}

#[cfg(test)]
mod tests {
    use database::model::{AccountRecord, CharacterRecord};
    use database::CharacterCreation;

    use super::*;
    use crate::repository::{CharServerRepository, LoginRepository, SledRepository};

    fn at(year: i32, month: u32, day: u32, hour: u32) -> chrono::DateTime<Local> {
        Local.with_ymd_and_hms(year, month, day, hour, 30, 0).earliest().unwrap()
    }

    #[test]
    fn solve_time_adds_each_unit() {
        let now = at(2026, 3, 10, 12);
        assert_eq!(solve_time("30s", now), 30);
        assert_eq!(solve_time("5mn", now), 300);
        assert_eq!(solve_time("5n", now), 300);
        assert_eq!(solve_time("2h", now), 7200);
        assert_eq!(solve_time("+1d", now), 86_400);
        assert_eq!(solve_time("1j", now), 86_400);
        assert_eq!(solve_time("-1h", now), -3600);
    }

    #[test]
    fn solve_time_combines_units_and_calendar_fields() {
        let now = at(2026, 3, 10, 12);
        assert_eq!(solve_time("1d2h30mn15s", now), 86_400 + 7200 + 1800 + 15);
        let next_month = at(2026, 4, 10, 12).timestamp() - now.timestamp();
        assert_eq!(solve_time("1m", now), next_month);
        let next_year = at(2027, 3, 10, 12).timestamp() - now.timestamp();
        assert_eq!(solve_time("1y", now), next_year);
        assert_eq!(solve_time("1a", now), next_year);
    }

    #[test]
    fn solve_time_normalises_like_struct_tm() {
        let now = at(2026, 1, 31, 12);
        let expected = at(2026, 3, 3, 12).timestamp() - now.timestamp();
        assert_eq!(solve_time("1m", now), expected, "January 31st plus a month is March 3rd");
    }

    #[test]
    fn solve_time_ignores_unusable_input() {
        let now = at(2026, 3, 10, 12);
        assert_eq!(solve_time("", now), 0);
        assert_eq!(solve_time("abc", now), 0);
        assert_eq!(solve_time("0d", now), 0);
        assert_eq!(solve_time("5", now), 0, "a number without a unit");
        assert_eq!(solve_time("2x1h", now), 3600, "unknown units are skipped");
    }

    #[test]
    fn ban_arguments_need_a_time_and_a_name_with_blanks() {
        assert_eq!(parse_ban_arguments("1d Hero"), Some(("1d", "Hero")));
        assert_eq!(parse_ban_arguments("  2h   Sir Hero  "), Some(("2h", "Sir Hero")));
        assert_eq!(parse_ban_arguments("1d"), None);
        assert_eq!(parse_ban_arguments(""), None);
    }

    struct World {
        repository: SledRepository,
        account_id: u32,
    }

    fn world() -> World {
        let repository = SledRepository::temporary().unwrap();
        let account_id = repository.account_create(AccountRecord::new(0, "player", "secret")).unwrap();
        repository
            .char_create(&CharacterCreation {
                character: CharacterRecord { account_id: account_id as i32, name: "Hero".into(), inventory_slots: 100, ..Default::default() },
                items: vec![],
                slot_limit: 12,
                case_sensitive_names: false,
            })
            .unwrap();
        World { repository, account_id }
    }

    #[test]
    fn account_bans_start_now_extend_active_bans_and_refuse_unbanning_dates() {
        let world = world();
        let (account, until) = ban_account(&world.repository, "Hero", 3600, 1_000).unwrap();
        assert_eq!((account.account_id, until), (world.account_id, 4_600));
        let (_, extended) = ban_account(&world.repository, "hero", 600, 2_000).unwrap();
        assert_eq!(extended, 5_200, "added to the running ban");
        let (_, fresh) = ban_account(&world.repository, "Hero", 100, 9_000).unwrap();
        assert_eq!(fresh, 9_100, "an elapsed ban starts over");
        assert_eq!(ban_account(&world.repository, "Hero", -50_000, 9_050).unwrap_err(), AdminError::UnbansTarget);
        assert_eq!(ban_account(&world.repository, "Hero", 0, 9_050).unwrap_err(), AdminError::InvalidTime);
        assert_eq!(ban_account(&world.repository, "Nobody", 10, 1).unwrap_err(), AdminError::UnknownCharacter);
        assert_eq!(world.repository.account_by_id(world.account_id).unwrap().unwrap().unban_time, 9_100);
    }

    #[test]
    fn unbanning_clears_the_ban_and_complains_when_there_is_none() {
        let world = world();
        assert_eq!(unban_account(&world.repository, "Hero").unwrap_err(), AdminError::NotBanned);
        ban_account(&world.repository, "Hero", 60, 1).unwrap();
        assert_eq!(unban_account(&world.repository, "Hero").unwrap().unban_time, 0);
    }

    #[test]
    fn blocking_sets_the_account_state_that_refuses_logins() {
        let world = world();
        assert_eq!(set_account_blocked(&world.repository, "Hero", true).unwrap().state, ACCOUNT_STATE_BLOCKED);
        assert_eq!(set_account_blocked(&world.repository, "Hero", false).unwrap().state, 0);
    }

    #[test]
    fn character_bans_only_touch_the_character() {
        let world = world();
        let (character, until) = ban_character(&world.repository, "Hero", 120, 500).unwrap();
        assert_eq!((character.unban_time, until), (620, 620));
        assert_eq!(world.repository.account_by_id(world.account_id).unwrap().unwrap().unban_time, 0);
        assert_eq!(unban_character(&world.repository, "Hero").unwrap().unban_time, 0);
        assert_eq!(unban_character(&world.repository, "Hero").unwrap_err(), AdminError::NotBanned);
        assert!(world.repository.char_find_by_name("Hero").unwrap().is_some());
    }
}
