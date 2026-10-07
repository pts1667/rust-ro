//! `CZ_ACTIVE_QUEST`: the quest log window toggles a quest between active and inactive.

use super::framing::FrameLength;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterQuestActivation, GameEvent};
use crate::server::model::request::Request;

const CZ_ACTIVE_QUEST: u16 = 0x02B6;

pub fn frame_length(id: u16) -> Option<FrameLength> {
    (id == CZ_ACTIVE_QUEST).then_some(FrameLength::Fixed(7))
}

/// Routes the packet of this module; returns false for any other packet.
pub fn handle_raw(server: &Server, context: &Request) -> bool {
    let raw = context.packet().raw();
    if raw.len() < 7 || u16::from_le_bytes([raw[0], raw[1]]) != CZ_ACTIVE_QUEST {
        return false;
    }
    if let Some(char_id) = super::connection_char_id(server, context) {
        server.add_to_next_tick(GameEvent::CharacterQuestActivation(CharacterQuestActivation {
            char_id,
            quest_id: u32::from_le_bytes([raw[2], raw[3], raw[4], raw[5]]),
            active: raw[6] != 0,
        }));
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::server::request_handler::framing::ClientFrames;

    #[test]
    fn quest_activation_is_framed() {
        let packet = vec![0xB6, 0x02, 0x58, 0x1B, 0, 0, 1];
        assert_eq!(ClientFrames::new(20120307).push(&packet).unwrap(), vec![packet]);
    }
}
