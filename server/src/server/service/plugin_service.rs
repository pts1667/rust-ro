use std::sync::Arc;

use script_sdk::{Entry, Hook, Interception};
use tokio::sync::mpsc;

use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, PluginHook};
use crate::server::model::script::Script;
use crate::server::model::session::Session;
use crate::server::plugins::{LoadedPlugin, Plugins};
use crate::server::script::NpcScriptHost;
use crate::server::state::server::ServerState;

impl Server {
    pub(crate) fn plugins(&self) -> &Arc<Plugins> {
        self.script_service().vm.plugins()
    }

    /// Queues `hook` for the plugins that observe it; free when none does.
    pub(crate) fn queue_hook(&self, hook: Hook) {
        if crate::server::plugins::observed(hook.kind()) {
            self.add_to_next_tick(GameEvent::PluginHook(PluginHook { hook }));
        }
    }

    /// Asks the plugins about `decision`: the decision to carry out, possibly changed, or the refusal message.
    pub(crate) fn intercept(&self, decision: Interception) -> Result<Interception, String> {
        self.plugins().intercept(decision)
    }

    /// [`intercept`](Self::intercept) for a yes or no decision, telling `char_id` why when a plugin refuses with a message.
    pub(crate) fn intercept_allowed(&self, char_id: u32, decision: Interception) -> bool {
        match self.intercept(decision) {
            Ok(_) => true,
            Err(message) => {
                if !message.is_empty() {
                    self.tell(char_id, &message);
                }
                false
            }
        }
    }

    /// A plugin run that belongs to no NPC: its initialisation or a hook. The script carries the plugin's init handle, which is
    /// how the game loop recognises the requests of such runs.
    pub(crate) fn plugin_host(&self, server: Arc<Server>, plugin: &LoadedPlugin, session: Arc<Session>) -> NpcScriptHost {
        let script = Script {
            id: 0,
            scope_instance: 0,
            map_name: String::new(),
            name: plugin.name().into(),
            sprite: 0,
            x: 0,
            y: 0,
            dir: 0,
            x_size: 0,
            y_size: 0,
            entry_id: plugin.init_handle,
            constructor_args: Vec::new(),
        };
        let (_, inputs) = mpsc::channel(1);
        NpcScriptHost {
            server,
            session,
            script: Arc::new(script),
            inputs,
            notifications: self.server_service().notification_sender(),
            generation: 0,
            map_instance: 0,
            background: true,
            event_depth: 0,
            event_arguments: None,
            timer_context: None,
            logout_token: None,
            dialog_open: false,
            attached: None,
            remote_dialogue: Default::default(),
            error: None,
        }
    }

    fn detached_session(&self) -> Arc<Session> {
        Arc::new(Session::create_empty(0, 0, 0, self.packetver()).recreate_with_character(0))
    }

    /// Runs the initialisation of every plugin, one after the other in load order. Queued after the base `OnInit` events, so
    /// it starts once they have all been started.
    pub(crate) fn run_plugin_initialization(&self) -> Result<(), String> {
        if self.plugins().is_empty() {
            return Ok(());
        }
        let server = self.shared().ok_or("Plugin runtime is not bound")?;
        let hosts: Vec<_> = self
            .plugins()
            .iter()
            .map(|plugin| (plugin.name().to_string(), plugin.module.clone(), self.plugin_host(server.clone(), plugin, self.detached_session())))
            .collect();
        let vm = self.script_service().vm.clone();
        self.runtime().spawn(async move {
            for (name, module, host) in hosts {
                let (host, result) = vm.run_entry(host, &module, &Entry::PluginInit).await;
                match result {
                    Ok(()) => info!("Plugin {name} initialised"),
                    Err(error) => error!("Plugin {name} failed to initialise: {}", host.error.unwrap_or(error)),
                }
            }
        });
        Ok(())
    }

    /// Runs `hook` in every plugin that observes it, with the character attached while they are online.
    pub(crate) fn run_hook(&self, state: &ServerState, hook: Hook) -> Result<(), String> {
        let kind = hook.kind();
        if self.plugins().observers(kind).next().is_none() {
            return Ok(());
        }
        let server = self.shared().ok_or("Plugin runtime is not bound")?;
        let session = state
            .get_character(hook.char_id())
            .and_then(|character| state.find_session(character.account_id))
            .filter(|session| session.char_id == Some(hook.char_id()))
            .unwrap_or_else(|| self.detached_session());
        let timeout = std::time::Duration::from_secs(self.configuration.scripting.conversation_timeout_secs.max(1));
        let entry = Entry::Hook(hook);
        for plugin in self.plugins().observers(kind) {
            let host = self.plugin_host(server.clone(), plugin, session.clone());
            let (vm, module, name, entry) = (self.script_service().vm.clone(), plugin.module.clone(), plugin.name().to_string(), entry.clone());
            self.runtime().spawn(async move {
                match tokio::time::timeout(timeout, vm.run_entry(host, &module, &entry)).await {
                    Ok((_, Ok(()))) => {}
                    Ok((host, Err(error))) => script_debug!("Plugin {name} hook {kind:?} failed: {}", host.error.unwrap_or(error)),
                    Err(_) => script_debug!("Plugin {name} hook {kind:?} timed out"),
                }
            });
        }
        Ok(())
    }
}
