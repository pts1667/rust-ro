//! `CZ_MEMORIALDUNGEON_COMMAND`: the memorial dungeon window asks to destroy the dungeon.

use super::framing::FrameLength;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterInstanceCommand, GameEvent};
use crate::server::model::request::Request;

const CZ_MEMORIALDUNGEON_COMMAND: u16 = 0x02CF;

pub fn frame_length(id: u16) -> Option<FrameLength> {
    (id == CZ_MEMORIALDUNGEON_COMMAND).then_some(FrameLength::Fixed(6))
}

/// Routes the packet of this module; returns false for any other packet.
pub fn handle_raw(server: &Server, context: &Request) -> bool {
    let raw = context.packet().raw();
    if raw.len() < 6 || u16::from_le_bytes([raw[0], raw[1]]) != CZ_MEMORIALDUNGEON_COMMAND {
        return false;
    }
    if let Some(char_id) = super::connection_char_id(server, context) {
        server.add_to_next_tick(GameEvent::CharacterInstanceCommand(CharacterInstanceCommand { char_id }));
    }
    true
}
