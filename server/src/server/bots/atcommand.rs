//! `@` commands of bots: run by the game loop as the permission group `bots.command_group`, so the answer holds what the command said.
use serde_json::{Value, json};

use super::interaction::{character_of, chat_text};
use crate::server::Server;
use crate::server::model::permission_groups::CommandKind;
use crate::server::model::reply_capture::capture_replies;
use crate::server::request_handler::atcommand::handle_atcommand;
use crate::server::state::server::ServerState;

impl Server {
    pub(crate) fn bot_command(&self, state: &mut ServerState, char_id: u32, text: &str) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let (account_id, name) = (character.account_id, character.name.clone());
        let text = chat_text(text)?;
        let typed = text
            .strip_prefix('@')
            .and_then(|command| command.split_whitespace().next())
            .ok_or("A command starts with @ and a command name, like @rates")?;
        let groups = state.permission_groups();
        let group_id = state.group_id_of(account_id);
        let canonical = groups.canonical_command(typed);
        if !groups.can_use_command(group_id, typed, CommandKind::At) {
            let allowed = groups.usable_commands(group_id, CommandKind::At);
            return Err(format!(
                "@{typed} is not a command of the group {group_id} that bots run commands as (bots.command_group), it allows: {}",
                allowed.join(" ")
            ));
        }
        let message = format!("{name} : {text}");
        let ((), replies) = capture_replies(char_id, || handle_atcommand(self, state, char_id, &message));
        Ok(json!({ "command": canonical, "replies": replies }))
    }
}
