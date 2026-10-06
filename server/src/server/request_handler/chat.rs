use packets::packets::PacketCzPlayerChat;

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterChat, CharacterSocial, GameEvent, SocialAction};
use crate::server::model::request::Request;

const HEADER_SIZE: usize = 4;
const COMMAND_SEPARATOR: &[u8] = b" : ";

/// Chat that starts with `@` after the `Name : ` prefix is a command, anything else is spoken.
fn is_command(text: &[u8]) -> bool {
    text.windows(COMMAND_SEPARATOR.len())
        .position(|window| window == COMMAND_SEPARATOR)
        .is_some_and(|index| text.get(index + COMMAND_SEPARATOR.len()) == Some(&b'@'))
}

pub fn handle_chat(server: &Server, context: Request) {
    let packet_player_chat = cast!(context.packet(), PacketCzPlayerChat);
    let char_id = context.session().char_id();
    let text = context.packet().raw().get(HEADER_SIZE..).unwrap_or_default();
    if is_command(text) {
        server.add_to_next_tick(GameEvent::CharacterChat(CharacterChat {
            char_id,
            message: packet_player_chat.msg.clone(),
        }));
    } else {
        server.add_to_next_tick(GameEvent::CharacterSocial(CharacterSocial {
            char_id,
            action: SocialAction::PublicChat(text.to_vec()),
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_text_after_the_speaker_prefix_can_be_a_command() {
        assert!(is_command(b"Alice : @warp prontera\0"));
        assert!(!is_command(b"Alice : hello @warp\0"));
        assert!(!is_command(b"@Alice : hello\0"));
    }
}
