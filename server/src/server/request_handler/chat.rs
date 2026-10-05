use packets::packets::PacketCzPlayerChat;

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterChat, GameEvent};
use crate::server::model::request::Request;

pub fn handle_chat(server: &Server, context: Request) {
    let packet_player_chat = cast!(context.packet(), PacketCzPlayerChat);
    server.add_to_next_tick(GameEvent::CharacterChat(CharacterChat {
        char_id: context.session().char_id(),
        message: packet_player_chat.msg.clone(),
    }));
}
