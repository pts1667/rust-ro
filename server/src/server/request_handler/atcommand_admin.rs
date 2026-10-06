use chrono::{Local, TimeZone};

use crate::server::Server;
use crate::server::request_handler::login::NOTIFY_DISCONNECTED;
use crate::server::service::account_admin_service::{self, AdminError};
use crate::server::state::server::ServerState;

const TIME_FORMAT_HELP: &str =
    "Time parameter format is +/-<value> to alter. y/a = Year, m = Month, d/j = Day, h = Hour, n/mn = Minute, s = Second.";

fn banished_notice(target: &str, until: i64) -> String {
    let date = Local.timestamp_opt(until, 0).single().map_or_else(String::new, |date| date.format("%d-%m-%Y %H:%M:%S").to_string());
    format!("Your {target} has been banished until {date} ")
}

fn failure(error: &AdminError, name: &str) -> String {
    match error {
        AdminError::UnknownCharacter => format!("The player '{name}' doesn't exist."),
        AdminError::UnknownAccount => format!("The account of '{name}' doesn't exist."),
        AdminError::NotBanned => format!("'{name}' is not banned."),
        AdminError::InvalidTime => "Invalid time for the command (time=0)".to_string(),
        AdminError::UnbansTarget => "The new date would lift the ban, use the unban command instead.".to_string(),
        AdminError::Storage(message) => format!("The request failed: {message}"),
    }
}

fn tell(server: &Server, char_id: u32, text: &str) {
    use packets::packets::Packet as _;
    let mut packet = crate::util::packet::playerchat_packet(server.packetver(), text);
    server
        .server_service()
        .notification_sender()
        .send(crate::server::model::events::client_notification::Notification::Char(
            crate::server::model::events::client_notification::CharNotification::new(char_id, std::mem::take(packet.raw_mut())),
        ))
        .unwrap_or_else(|_| error!("Failed to send a chat message to client"));
}

/// Sends the notice to every online character of the account and closes its session.
fn disconnect_account(server: &Server, state: &ServerState, account_id: u32, notice: Option<&str>) {
    let Some(session) = state.find_session(account_id) else {
        return;
    };
    if let (Some(char_id), Some(notice)) = (session.char_id, notice) {
        tell(server, char_id, notice);
    }
    server.kick_session(&session, NOTIFY_DISCONNECTED);
}

fn disconnect_character(server: &Server, state: &ServerState, char_id: u32, notice: Option<&str>) {
    let Some(character) = state.get_character(char_id) else {
        return;
    };
    if let Some(notice) = notice {
        tell(server, char_id, notice);
    }
    if let Some(session) = state.find_session(character.account_id).filter(|session| session.char_id == Some(char_id)) {
        server.kick_session(&session, NOTIFY_DISCONNECTED);
    }
}

fn parse_ban(command: &str, arguments: &str) -> Result<(i64, String), Vec<String>> {
    let Some((time, name)) = account_admin_service::parse_ban_arguments(arguments) else {
        return Err(vec![format!("Please enter ban time and a player name (usage: @{command} <time> <char name>).")]);
    };
    let timediff = account_admin_service::solve_time(time, Local::now());
    if timediff == 0 {
        return Err(vec![format!("Invalid time for {command} command (time={timediff})"), TIME_FORMAT_HELP.to_string()]);
    }
    if timediff < 0 {
        return Err(vec!["You are not allowed to alter the time of a ban.".to_string()]);
    }
    Ok((timediff, name.to_string()))
}

fn require_name(command: &str, arguments: &str) -> Result<String, Vec<String>> {
    let name = arguments.trim();
    if name.is_empty() {
        return Err(vec![format!("Please enter a player name (usage: @{command} <char name>).")]);
    }
    Ok(name.to_string())
}

/// Runs `@ban`, `@unban`, `@charban`, `@charunban`, `@block`, `@unblock` or `@kick`; returns the lines to tell the caller.
pub fn handle(server: &Server, state: &ServerState, char_id: u32, command: &str, arguments: &str) -> Vec<String> {
    let repository = server.repository.as_ref();
    let now = chrono::Utc::now().timestamp();
    let outcome: Result<Vec<String>, Vec<String>> = (|| match command {
        "ban" => {
            let (timediff, name) = parse_ban(command, arguments)?;
            let (account, until) = account_admin_service::ban_account(repository, &name, timediff, now).map_err(|error| vec![failure(&error, &name)])?;
            disconnect_account(server, state, account.account_id, Some(&banished_notice("account", until)));
            Ok(vec![format!("Login-server has been asked to ban the player '{name}'.")])
        }
        "char_ban" => {
            let (timediff, name) = parse_ban(command, arguments)?;
            let (character, until) = account_admin_service::ban_character(repository, &name, timediff, now).map_err(|error| vec![failure(&error, &name)])?;
            disconnect_character(server, state, character.char_id as u32, Some(&banished_notice("char", until)));
            Ok(vec![format!("Character server has been asked to ban the player '{name}'.")])
        }
        "unban" => {
            let name = require_name(command, arguments)?;
            account_admin_service::unban_account(repository, &name).map_err(|error| vec![failure(&error, &name)])?;
            Ok(vec![format!("Login-server has been asked to unban the player '{name}'.")])
        }
        "char_unban" => {
            let name = require_name(command, arguments)?;
            account_admin_service::unban_character(repository, &name).map_err(|error| vec![failure(&error, &name)])?;
            Ok(vec![format!("Character server has been asked to unban the player '{name}'.")])
        }
        "char_block" | "char_unblock" => {
            let name = require_name(command, arguments)?;
            let blocking = command == "char_block";
            let account = account_admin_service::set_account_blocked(repository, &name, blocking).map_err(|error| vec![failure(&error, &name)])?;
            if blocking {
                disconnect_account(server, state, account.account_id, Some("Your account is not authorised any more."));
            }
            let action = if blocking { "block" } else { "unblock" };
            Ok(vec![format!("Login-server has been asked to {action} the player '{name}'.")])
        }
        "kick" => {
            let name = require_name(command, arguments)?;
            let target = state.characters().values().find(|character| character.name.eq_ignore_ascii_case(&name));
            let Some(target) = target else {
                return Err(vec!["Character not found.".to_string()]);
            };
            let groups = state.permission_groups();
            let caller_level = state.get_character(char_id).map_or(0, |caller| groups.level(state.group_id_of(caller.account_id)));
            if groups.level(state.group_id_of(target.account_id)) > caller_level {
                return Err(vec!["Your GM level don't authorize you to do this action on this player.".to_string()]);
            }
            disconnect_character(server, state, target.char_id, None);
            Ok(vec![])
        }
        _ => Err(vec![format!("@{command} is an Unknown Command.")]),
    })();
    outcome.unwrap_or_else(|replies| replies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ban_arguments_report_usage_and_time_problems() {
        assert!(parse_ban("ban", "").unwrap_err()[0].contains("usage: @ban <time> <char name>"));
        assert!(parse_ban("ban", "1d").is_err());
        let invalid = parse_ban("ban", "xyz Hero").unwrap_err();
        assert_eq!(invalid[0], "Invalid time for ban command (time=0)");
        assert_eq!(invalid[1], TIME_FORMAT_HELP);
        assert_eq!(parse_ban("ban", "-1d Hero").unwrap_err(), vec!["You are not allowed to alter the time of a ban."]);
        let (timediff, name) = parse_ban("ban", "2h Sir Hero").unwrap();
        assert_eq!((timediff, name.as_str()), (7200, "Sir Hero"));
    }

    #[test]
    fn name_only_commands_need_a_name() {
        assert!(require_name("unban", "  ").unwrap_err()[0].contains("usage: @unban <char name>"));
        assert_eq!(require_name("unban", " Hero ").unwrap(), "Hero");
    }

    #[test]
    fn banished_notice_names_the_target_and_the_end_of_the_ban() {
        let notice = banished_notice("account", 86_400 * 365);
        assert!(notice.starts_with("Your account has been banished until "), "{notice}");
        assert!(notice.contains("1971"));
        assert!(banished_notice("char", 0).starts_with("Your char has been"));
    }

    #[test]
    fn failures_are_phrased_for_the_game_master() {
        assert_eq!(failure(&AdminError::UnknownCharacter, "Hero"), "The player 'Hero' doesn't exist.");
        assert_eq!(failure(&AdminError::NotBanned, "Hero"), "'Hero' is not banned.");
    }
}
