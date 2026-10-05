use movement::position::Position;
use packets::packets::{PacketCzRequestMove, PacketCzRequestMove2};

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterRequestMove, GameEvent};
use crate::server::model::position::PositionPacket;
use crate::server::model::request::Request;

pub fn handle_char_move(server: &Server, context: Request) {
    let destination = if context.packet().as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
        let move_packet = cast!(context.packet(), PacketCzRequestMove2);
        Position::from_move2_packet(move_packet)
    } else {
        let move_packet = cast!(context.packet(), PacketCzRequestMove);
        Position::from_move_packet(move_packet)
    };
    debug!("Request move to {}", destination);
    server.add_to_next_movement_tick(GameEvent::CharacterRequestMove(CharacterRequestMove {
        char_id: context.session().char_id.unwrap(),
        destination,
    }));
}
