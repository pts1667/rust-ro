//! Server-defined chat channels: `@join`, `@leave`, `@channel list`, and speaking by whispering to `#name`.
//! Private, ally and map channels of rathena are not implemented.

use configuration::configuration::ChannelConfig;

use crate::server::Server;
use crate::server::model::permission_groups::Permission;
use crate::server::service::social_packets as wire;
use crate::server::state::server::ServerState;

const CHAT_SIZE_MAX: usize = 255;

fn key(name: &str) -> String {
    name.to_lowercase()
}

impl Server {
    fn channel_named(&self, name: &str) -> Option<&ChannelConfig> {
        self.configuration.game.channels.iter().find(|channel| channel.name.eq_ignore_ascii_case(name))
    }

    fn channel_member_count(state: &ServerState, channel: &ChannelConfig) -> usize {
        let name = key(&channel.name);
        state.characters().values().filter(|character| character.game_systems.channels.contains_key(&name)).count()
    }

    /// Puts a character that just entered the game into the channels flagged `autojoin`.
    pub(crate) fn channel_login(&self, state: &mut ServerState, char_id: u32) {
        let group_id = state.get_character(char_id).map(|character| state.group_id_of(character.account_id));
        let Some(group_id) = group_id else { return };
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return };
        for channel in self.configuration.game.channels.iter().filter(|channel| channel.autojoin) {
            if channel.group_ids.is_empty() || channel.group_ids.contains(&group_id) {
                character.game_systems.channels.entry(key(&channel.name)).or_insert(0);
            }
        }
    }

    /// Answers `@channel`, `@join` and `@leave`.
    pub(crate) fn channel_command(&self, state: &mut ServerState, char_id: u32, command: &str, args: &[&str]) -> String {
        let (action, name) = match command {
            "join" => (command, args.first().copied()),
            _ => (args.first().copied().unwrap_or("list"), args.get(1).copied()),
        };
        match action {
            "list" => self.channel_list(state, char_id),
            "join" => self.join_channel(state, char_id, name),
            "leave" => self.leave_channel(state, char_id, name),
            _ => "Usage: @channel <list|join|leave> [#channel_name]".to_string(),
        }
    }

    fn channel_list(&self, state: &ServerState, char_id: u32) -> String {
        let Some(character) = state.get_character(char_id) else { return String::new() };
        let mut lines = vec!["Available channels:".to_string()];
        for channel in &self.configuration.game.channels {
            let joined = if character.game_systems.channels.contains_key(&key(&channel.name)) { " (joined)" } else { "" };
            lines.push(format!("- {} ({} users){joined}", channel.name, Self::channel_member_count(state, channel)));
        }
        lines.join("\n")
    }

    fn join_channel(&self, state: &mut ServerState, char_id: u32, name: Option<&str>) -> String {
        let Some(name) = name else { return "Unknown channel (usage: @join <#channel_name>).".to_string() };
        let Some(channel) = self.channel_named(name) else { return format!("Unknown channel '{name}' (usage: @join <#channel_name>).") };
        let Some(character) = state.get_character(char_id) else { return String::new() };
        let group_id = state.group_id_of(character.account_id);
        let allowed = channel.group_ids.is_empty()
            || channel.group_ids.contains(&group_id)
            || state.has_permission(character.account_id, Permission::ChannelAdmin);
        if !allowed {
            return format!("Unknown channel '{name}' (usage: @join <#channel_name>).");
        }
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return String::new() };
        if character.game_systems.channels.contains_key(&key(&channel.name)) {
            return format!("You're already in the '{}' channel.", channel.name);
        }
        character.game_systems.channels.insert(key(&channel.name), 0);
        format!("You've joined the '{}' channel.", channel.name)
    }

    fn leave_channel(&self, state: &mut ServerState, char_id: u32, name: Option<&str>) -> String {
        let Some(name) = name else { return "Unknown channel (usage: @leave <#channel_name>).".to_string() };
        let Some(channel) = self.channel_named(name) else { return format!("Unknown channel '{name}' (usage: @leave <#channel_name>).") };
        let Some(character) = state.characters_mut().get_mut(&char_id) else { return String::new() };
        if !character.game_systems.channels.contains_key(&key(&channel.name)) {
            return format!("You're not in that channel (use '@join {}').", channel.name);
        }
        if !channel.leave {
            return format!("You cannot leave the '{}' channel.", channel.name);
        }
        character.game_systems.channels.remove(&key(&channel.name));
        format!("You've left the '{}' channel.", channel.name)
    }

    /// A whisper to `#name`: a line for every member of the channel. False when no such channel exists.
    pub(crate) fn channel_chat(&self, state: &mut ServerState, char_id: u32, target: &str, message: &[u8], tick: u128) -> bool {
        let Some(channel) = self.channel_named(target) else { return false };
        let Some(speaker) = state.get_character(char_id) else { return true };
        let name = key(&channel.name);
        let text = String::from_utf8_lossy(message).into_owned();
        if !speaker.game_systems.channels.contains_key(&name) {
            self.tell(char_id, &format!("You're not in that channel (use '@join {}').", channel.name));
            return true;
        }
        if !channel.chat || text.len() + 1 > CHAT_SIZE_MAX || speaker.manner < 0 {
            return true;
        }
        let last_tick = speaker.game_systems.channels[&name];
        let unrestricted = state.has_permission(speaker.account_id, Permission::ChannelAdmin);
        if !unrestricted && last_tick != 0 && tick < last_tick + u128::from(channel.delay_ms) {
            self.tell(char_id, "You're talking too fast!");
            return true;
        }
        let alias = if channel.alias.is_empty() { &channel.name } else { &channel.alias };
        let line = wire::channel_message(channel.color, &format!("{alias} {} : {text}", speaker.name));
        for member in state.characters().values().filter(|character| character.game_systems.channels.contains_key(&name)) {
            self.send_raw(member.char_id, line.clone());
        }
        if let Some(speaker) = state.characters_mut().get_mut(&char_id) {
            speaker.game_systems.channels.insert(name, tick.max(1));
        }
        true
    }
}
