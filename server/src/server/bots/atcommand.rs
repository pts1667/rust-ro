//! `@` commands of bots: run by the game loop as the permission group `bots.command_group`, so the answer holds what the command said.
use serde_json::{Value, json};

use super::interaction::{character_of, chat_text};
use crate::server::Server;
use crate::server::model::permission_groups::CommandKind;
use crate::server::model::reply_capture::capture_replies;
use crate::server::request_handler::atcommand::handle_atcommand;
use crate::server::state::server::ServerState;

/// A command a bot may run: its canonical name and the words after it.
pub(crate) struct BotCommandLine {
    pub name: String,
    pub args: Vec<String>,
    text: String,
}

impl Server {
    pub(crate) fn bot_command(&self, state: &mut ServerState, char_id: u32, text: &str) -> Result<Value, String> {
        let command = self.bot_command_line(state, char_id, text)?;
        let name = character_of(state, char_id)?.name.clone();
        let message = format!("{name} : {}", command.text);
        let ((), replies) = capture_replies(char_id, || handle_atcommand(self, state, char_id, &message));
        Ok(json!({ "command": command.name, "replies": replies }))
    }

    /// Reads `@name args` and checks that the group bots run commands as may use it.
    pub(crate) fn bot_command_line(&self, state: &ServerState, char_id: u32, text: &str) -> Result<BotCommandLine, String> {
        let account_id = character_of(state, char_id)?.account_id;
        let text = chat_text(text)?;
        let mut words = text.strip_prefix('@').map(str::split_whitespace).ok_or("A command starts with @ and a command name, like @rates")?;
        let typed = words.next().ok_or("A command starts with @ and a command name, like @rates")?;
        let groups = state.permission_groups();
        let group_id = state.group_id_of(account_id);
        if !groups.can_use_command(group_id, typed, CommandKind::At) {
            let allowed = groups.usable_commands(group_id, CommandKind::At);
            return Err(format!(
                "@{typed} is not a command of the group {group_id} that bots run commands as (bots.command_group), it allows: {}",
                allowed.join(" ")
            ));
        }
        Ok(BotCommandLine { name: groups.canonical_command(typed), args: words.map(String::from).collect(), text })
    }
}
