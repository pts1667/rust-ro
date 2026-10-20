//! `@` commands of plugins (`docs/plugins.md`). The permission groups know them (`PermissionGroups::add_plugin_command`), so
//! the caller has checked the player's group already. A command runs as a job of its plugin for the character: it waits
//! behind that plugin's window events and bot actions, and its replies reach the bot that ran it.

use script_sdk::{Function, PluginCommand, Reply, Value};

use crate::server::Server;
use crate::server::bots::BotReply;
use crate::server::model::permission_groups::PermissionGroups;
use crate::server::plugins::Plugins;
use crate::server::plugins::ui::{PluginJob, answer};
use crate::server::script::ScriptRequest;
use crate::server::state::server::ServerState;

/// Most lines one reply may hold, so a looping command cannot flood the chat window.
const MAX_REPLY_LINES: usize = 50;

pub fn handles(function: Function) -> bool {
    function == Function::CommandReply
}

/// Adds the commands of the loaded plugins to the permission groups. A clash refuses the boot, like any plugin that cannot
/// load.
pub(crate) fn register_plugin_commands(plugins: &Plugins, groups: &mut PermissionGroups) {
    for plugin in plugins.iter() {
        for command in &plugin.info.commands {
            groups
                .add_plugin_command(&command.name, command.level)
                .unwrap_or_else(|error| panic!("Plugin {} is refused: {error}", plugin.name()));
        }
    }
}

impl Server {
    /// Runs `@name args` of the plugin that declared it. `reply` answers the bot that typed it.
    pub(crate) fn plugin_command(&self, state: &ServerState, char_id: u32, name: &str, args: Vec<String>, reply: Option<BotReply>) {
        let Some(plugin) = self.plugins().iter().find(|plugin| plugin.has_command(name)) else {
            if let Some(reply) = reply {
                answer(&reply, Err(format!("No plugin has the command @{name}")));
            }
            return;
        };
        let command = PluginCommand { name: name.to_string(), args };
        self.queue_plugin_job(state, char_id, plugin.name(), PluginJob::Command { command, reply });
    }

    /// `CommandReply(text, plugin)`: a chat line for the player running the command, kept for the bot that ran it.
    pub(crate) fn plugin_command_reply(&self, context: &ScriptRequest, arguments: &[Value]) -> Reply {
        let owner = arguments.last().map(Value::text).unwrap_or_default();
        let plugin = self.plugins().by_name(&owner).ok_or("Only plugins can answer commands")?;
        if context.char_id == 0 {
            return Err("CommandReply requires an attached player".into());
        }
        let text = arguments.first().map(Value::text).ok_or("Missing reply text")?;
        let store = self.plugin_ui();
        for line in text.lines().take(MAX_REPLY_LINES) {
            store.add_command_line(context.char_id, plugin.name(), line);
            self.tell(context.char_id, line);
        }
        Ok(Value::default())
    }
}
