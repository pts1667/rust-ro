//! The plugin UI packets a client sends (`plugin_ui::wire`, `docs/plugin-ui.md`): `CZ_CLIENT_FEATURES`, which tells the server
//! the client draws plugin UI, and `CZ_PLUGIN_UI_EVENT`, what the player did in a plugin window.

use plugin_ui::wire::{self, ClientMessage};

use super::framing::FrameLength;
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, PluginUiEvent};
use crate::server::model::request::Request;

pub fn frame_length(id: u16) -> Option<FrameLength> {
    wire::client_frame_length(id).map(|length| match length {
        wire::FrameLength::Fixed(bytes) => FrameLength::Fixed(bytes),
        wire::FrameLength::Variable => FrameLength::Variable { minimum: 4 },
    })
}

/// Routes the packets of this module; returns false for any other packet.
pub fn handle_raw(server: &Server, context: &Request) -> bool {
    let raw = context.packet().raw();
    if raw.len() < 2 || wire::client_frame_length(u16::from_le_bytes([raw[0], raw[1]])).is_none() {
        return false;
    }
    let Some(char_id) = super::connection_char_id(server, context) else {
        warn!("Plugin UI packet from a connection without a character");
        return true;
    };
    match wire::decode_client(raw) {
        Ok(ClientMessage::Features(flags)) => {
            script_debug!("Character {char_id} announced client features {flags:#x}");
            for open in server.plugin_ui().set_client_features(char_id, flags) {
                server.send_plugin_ui(char_id, wire::encode_open(&wire::WindowOpen { plugin: open.plugin, window: open.window }));
            }
        }
        Ok(ClientMessage::Event(message)) => {
            server.add_to_next_tick(GameEvent::PluginUiEvent(PluginUiEvent { char_id, plugin: message.plugin, event: message.event }))
        }
        Err(error) => warn!("Rejected a malformed plugin UI packet from character {char_id}: {error}"),
    }
    true
}

#[cfg(test)]
mod tests {
    use plugin_ui::UiEvent;
    use plugin_ui::wire::{EventMessage, FEATURE_PLUGIN_UI, encode_event, encode_features};

    use crate::server::request_handler::framing::ClientFrames;

    #[test]
    fn plugin_ui_packets_are_framed() {
        let features = encode_features(FEATURE_PLUGIN_UI);
        let event = encode_event(&EventMessage { plugin: "ui_shop".into(), event: UiEvent::click("shop", "buy:501") }).unwrap();
        let mut stream = features.clone();
        stream.extend_from_slice(&event);
        assert_eq!(ClientFrames::new(20120307).push(&stream).unwrap(), vec![features, event]);
    }
}
