//! Runs what players and bots do in plugin windows, and the actions plugins offer to bots, through the plugin's handlers
//! (`docs/plugin-ui.md`). Jobs of one plugin for one character run one after the other, see `plugins::ui`.

use plugin_ui::{EventKind, UiEvent};
use script_sdk::{BotAction, Entry};
use serde_json::{Value as Json, json};

use crate::server::Server;
use crate::server::bots::BotReply;
use crate::server::model::events::game_event::{GameEvent, PluginJobDone};
use crate::server::plugins::ui::{PluginJob, answer};
use crate::server::state::server::ServerState;

impl Server {
    /// A player or a bot acted in a plugin window. The event is checked against the window they were shown.
    pub(crate) fn plugin_ui_event(&self, state: &ServerState, char_id: u32, plugin: &str, event: UiEvent, reply: Option<BotReply>) {
        match self.accept_plugin_ui_event(char_id, plugin, event) {
            Ok(Some(event)) => self.queue_plugin_job(state, char_id, plugin, PluginJob::Event { event, reply }),
            Ok(None) => {
                if let Some(reply) = reply {
                    answer(&reply, Ok(json!({ "window": Json::Null })));
                }
            }
            Err(error) => {
                script_debug!("Plugin UI event of character {char_id} refused: {error}");
                if let Some(reply) = reply {
                    answer(&reply, Err(error));
                }
            }
        }
    }

    /// `None` when no handler needs to run: a closed window whose plugin does not handle its events.
    fn accept_plugin_ui_event(&self, char_id: u32, plugin_name: &str, event: UiEvent) -> Result<Option<UiEvent>, String> {
        let plugin = self.plugins().by_name(plugin_name).ok_or_else(|| format!("There is no plugin {plugin_name}"))?;
        let window = self
            .plugin_ui()
            .window(char_id, plugin_name, &event.window)
            .ok_or_else(|| format!("Window {} of plugin {plugin_name} is not open", event.window))?;
        let event = event.accepted_by(&window)?;
        if event.kind == EventKind::Closed {
            self.plugin_ui().close(char_id, plugin_name, &event.window);
        }
        Ok(plugin.handles_window(&event.window).then_some(event))
    }

    /// A bot asked for an action `plugin_name` offers; `args` is JSON.
    pub(crate) fn plugin_bot_action(&self, state: &ServerState, char_id: u32, plugin_name: &str, name: String, args: String, reply: BotReply) {
        let Some(plugin) = self.plugins().by_name(plugin_name) else {
            return answer(&reply, Err(format!("There is no plugin {plugin_name}")));
        };
        if !plugin.has_bot_action(&name) {
            return answer(&reply, Err(format!("Plugin {plugin_name} has no bot action {name}")));
        }
        self.queue_plugin_job(state, char_id, plugin_name, PluginJob::BotAction { action: BotAction { name, args }, reply });
    }

    fn queue_plugin_job(&self, state: &ServerState, char_id: u32, plugin: &str, job: PluginJob) {
        match self.plugin_ui().enqueue(char_id, plugin, job) {
            Ok(Some(job)) => self.start_plugin_job(state, char_id, plugin, job),
            Ok(None) => {}
            Err(job) => match job.reply() {
                Some(reply) => answer(reply, Err(format!("Too many requests are waiting for plugin {plugin}"))),
                None => script_debug!("Plugin UI event of character {char_id} dropped, plugin {plugin} is busy"),
            },
        }
    }

    /// Runs `job` with the character attached. When the run ends, `PluginJobDone` starts the next job of the queue.
    pub(crate) fn start_plugin_job(&self, state: &ServerState, char_id: u32, plugin_name: &str, job: PluginJob) {
        let done = GameEvent::PluginJobDone(PluginJobDone { char_id, plugin: plugin_name.to_string() });
        let session = state
            .get_character(char_id)
            .and_then(|character| state.find_session(character.account_id))
            .filter(|session| session.char_id == Some(char_id));
        let (Some(server), Some(plugin), Some(session)) = (self.shared(), self.plugins().by_name(plugin_name), session) else {
            if let Some(reply) = job.reply() {
                answer(reply, Err("The character is not in the game".into()));
            }
            return self.add_to_next_tick(done);
        };
        let host = self.plugin_host(server.clone(), plugin, session);
        let (entry, reply) = match job {
            PluginJob::Event { event, reply } => (Entry::UiEvent(event), reply),
            PluginJob::BotAction { action, reply } => (Entry::BotAction(action), Some(reply)),
        };
        let timeout = std::time::Duration::from_secs(self.configuration.scripting.conversation_timeout_secs.max(1));
        let (vm, module, store, name) = (self.script_service().vm.clone(), plugin.module.clone(), self.plugin_ui().clone(), plugin_name.to_string());
        self.runtime().spawn(async move {
            let outcome = match tokio::time::timeout(timeout, vm.run_entry(host, &module, &entry)).await {
                Ok((_, Ok(()))) => Ok(()),
                Ok((host, Err(error))) => Err(host.error.unwrap_or(error)),
                Err(_) => Err("the run timed out".to_string()),
            };
            let bot_reply = store.take_reply(char_id, &name);
            match (reply, outcome) {
                (Some(reply), Err(error)) => answer(&reply, Err(format!("Plugin {name} failed: {error}"))),
                (Some(reply), Ok(())) => answer(
                    &reply,
                    Ok(match &entry {
                        Entry::UiEvent(event) => json!({ "window": store.window(char_id, &name, &event.window) }),
                        _ => bot_reply.unwrap_or(Json::Null),
                    }),
                ),
                (None, Err(error)) => script_debug!("Plugin {name} window handler failed: {error}"),
                (None, Ok(())) => {}
            }
            server.add_to_next_tick(done);
        });
    }

    /// The character left the game: their windows, observations and waiting jobs go.
    pub(crate) fn forget_plugin_ui(&self, char_id: u32) {
        for job in self.plugin_ui().forget(char_id) {
            if let Some(reply) = job.reply() {
                answer(reply, Err("The character left the game".into()));
            }
        }
    }
}
