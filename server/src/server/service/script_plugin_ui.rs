//! Script calls of plugin UI (`docs/plugin-ui.md`): windows, overhead notifications, bot observations and bot replies. The
//! runtime appends the calling plugin's name to each call, and refuses them to base modules.

use plugin_ui::wire::{OverheadMessage, WindowClose, WindowOpen};
use plugin_ui::{Audience, Overhead, Placement, Window, validate_overhead, validate_window};
use script_sdk::{Function, Reply, Value};

use crate::server::Server;
use crate::server::bots::BotEventKind;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::map_item::MapItemType;
use crate::server::plugins::LoadedPlugin;
use crate::server::script::ScriptRequest;
use crate::server::state::server::ServerState;

/// Cells around a unit within which players see an overhead meant for the area.
const OVERHEAD_RANGE: u16 = 14;

pub fn handles(function: Function) -> bool {
    matches!(function, Function::UiOpen | Function::UiClose | Function::UiOverhead | Function::BotObserve | Function::BotReply)
}

fn text(arguments: &[Value], index: usize, what: &str) -> Result<String, String> {
    arguments.get(index).map(Value::text).ok_or_else(|| format!("Missing {what}"))
}

impl Server {
    pub(crate) fn plugin_ui_call(&self, state: &ServerState, context: &ScriptRequest, function: Function, arguments: &[Value]) -> Reply {
        let owner = arguments.last().map(Value::text).unwrap_or_default();
        let plugin = self.plugins().by_name(&owner).ok_or("Only plugins can use plugin UI")?;
        let character = state.get_character(context.char_id).ok_or_else(|| format!("{function:?} requires an attached player"))?;
        let char_id = character.char_id;
        match function {
            Function::UiOpen => {
                let mut window: Window = serde_json::from_str(&text(arguments, 0, "window")?).map_err(|error| format!("Invalid window: {error}"))?;
                validate_window(&mut window, plugin.name())?;
                if let Placement::Anchored { unit_id, .. } = &mut window.placement {
                    *unit_id = self.client_unit_id(state, context, *unit_id)?;
                }
                self.plugin_ui().open(char_id, plugin.name(), window.clone())?;
                self.send_plugin_ui(char_id, plugin_ui::wire::encode_open(&WindowOpen { plugin: plugin.name().into(), window }));
            }
            Function::UiClose => {
                let id = text(arguments, 0, "window id")?;
                if self.plugin_ui().close(char_id, plugin.name(), &id) {
                    self.send_plugin_ui(char_id, plugin_ui::wire::encode_close(&WindowClose { plugin: plugin.name().into(), id }));
                }
            }
            Function::UiOverhead => {
                let unit = arguments.first().ok_or("Missing unit")?.number_value()?;
                let mut overhead: Overhead = serde_json::from_str(&text(arguments, 1, "overhead")?).map_err(|error| format!("Invalid overhead: {error}"))?;
                validate_overhead(&mut overhead, plugin.name())?;
                self.show_overhead(state, context, plugin, unit, overhead)?;
            }
            Function::BotObserve => {
                let key = text(arguments, 0, "observation key")?;
                if key.is_empty() || key.len() > 64 {
                    return Err("Observation keys are 1 to 64 bytes".into());
                }
                let value: serde_json::Value = serde_json::from_str(&text(arguments, 1, "observation")?).map_err(|error| format!("Invalid observation: {error}"))?;
                self.plugin_ui().observe(char_id, plugin.name(), &key, (!value.is_null()).then_some(value))?;
            }
            Function::BotReply => {
                let value: serde_json::Value = serde_json::from_str(&text(arguments, 0, "reply")?).map_err(|error| format!("Invalid reply: {error}"))?;
                self.plugin_ui().set_reply(char_id, plugin.name(), value);
            }
            _ => return Err(format!("{function:?} is not a plugin UI call")),
        }
        Ok(Value::default())
    }

    /// Sends a plugin UI packet to a character whose client draws plugin UI; others never see the custom ids.
    pub(crate) fn send_plugin_ui(&self, char_id: u32, packet: Result<Vec<u8>, String>) {
        if !self.plugin_ui().has_client(char_id) {
            return;
        }
        match packet {
            Ok(raw) => self
                .server_service()
                .notification_sender()
                .send(Notification::Char(CharNotification::new(char_id, raw)))
                .unwrap_or_else(|_| error!("Failed to send a plugin UI packet")),
            Err(error) => warn!("Plugin UI packet not sent: {error}"),
        }
    }

    /// Closes the windows the character had above units of the map they leave.
    pub(crate) fn close_anchored_plugin_windows(&self, char_id: u32) {
        for open in self.plugin_ui().close_anchored(char_id) {
            self.send_plugin_ui(char_id, plugin_ui::wire::encode_close(&WindowClose { plugin: open.plugin, id: open.window.id }));
        }
    }

    /// The id the client knows a unit in the game by: the account id for players.
    fn client_unit_id(&self, state: &ServerState, context: &ScriptRequest, unit_id: u32) -> Result<u32, String> {
        let target = crate::server::script::unit_data::request_actor(self, state, context, unit_id)?.ok_or_else(|| format!("Unit {unit_id} is not in the game"))?;
        Ok(match target.object_type {
            MapItemType::Character => state.get_character(target.id).map_or(target.id, |character| character.account_id),
            _ => target.id,
        })
    }

    /// `unit` 0 is the attached player. The client knows players by account id, so that is the id the packet carries.
    fn show_overhead(&self, state: &ServerState, context: &ScriptRequest, plugin: &LoadedPlugin, unit: i32, overhead: Overhead) -> Result<(), String> {
        let target_id = if unit == 0 { context.char_id } else { u32::try_from(unit).map_err(|_| "Invalid unit id")? };
        let target = crate::server::script::unit_data::request_actor(self, state, context, target_id)?.ok_or_else(|| format!("Unit {target_id} is not in the game"))?;
        let unit_id = match target.object_type {
            MapItemType::Character => state.get_character(target.id).map_or(target.id, |character| character.account_id),
            _ => target.id,
        };
        let recipients: Vec<u32> = match overhead.audience {
            Audience::SelfOnly => vec![context.char_id],
            Audience::Area => state
                .characters()
                .values()
                .filter(|character| {
                    character.current_map_name() == &target.map
                        && character.current_map_instance() == target.instance
                        && character.x().abs_diff(target.x) <= OVERHEAD_RANGE
                        && character.y().abs_diff(target.y) <= OVERHEAD_RANGE
                })
                .map(|character| character.char_id)
                .collect(),
        };
        let message = OverheadMessage { plugin: plugin.name().into(), overhead };
        let packet = plugin_ui::wire::encode_overhead(unit_id, &message)?;
        for char_id in recipients {
            self.send_plugin_ui(char_id, Ok(packet.clone()));
            if self.bots().is_bot(char_id) {
                self.bots().record(char_id, BotEventKind::PluginOverhead {
                    plugin: message.plugin.clone(),
                    unit_id: target.id,
                    text: message.overhead.text.as_ref().map(|text| text.text.clone()),
                    image: message.overhead.image.as_ref().and_then(|image| image.frames.first().cloned()),
                });
            }
        }
        Ok(())
    }
}
