use packets::packets::{Packet, PacketCzReqname, PacketCzReqnameall2, PacketZcHatEffect};

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterLoadedFromClientSide, CharacterRequestName, GameEvent};
use crate::server::model::request::Request;
use crate::server::service::global_config_service::GlobalConfigService;

/// First client version with `ZC_EQUIPMENT_EFFECT` (`0x0a3b`).
const EQUIPMENT_EFFECT_PACKETVER: u32 = 20150507;

pub fn handle_map_item_name(server: &Server, context: Request) {
    let gid = if context.packet().as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
        let packet_cz_req_allname2 = cast!(context.packet(), PacketCzReqnameall2);
        packet_cz_req_allname2.gid
    } else if context.packet().as_any().downcast_ref::<PacketCzReqname>().is_some() {
        let packet_cz_req_name = cast!(context.packet(), PacketCzReqname);
        packet_cz_req_name.aid
    } else {
        0
    };
    server.add_to_next_tick(GameEvent::CharacterRequestName(CharacterRequestName {
        char_id: context.session().char_id.unwrap(),
        gid,
    }));
}

pub fn handle_char_loaded_client_side(server: &Server, context: Request) {
    info!("Reload char");
    let session = context.session();
    let session_id = session.account_id;

    let packetver = GlobalConfigService::instance().packetver();
    if packetver >= EQUIPMENT_EFFECT_PACKETVER {
        let mut packet_zc_hat_effect = PacketZcHatEffect::new(packetver);
        packet_zc_hat_effect.set_aid(session_id);
        packet_zc_hat_effect.set_status(1);
        packet_zc_hat_effect.set_len(9); // len is: 9 (packet len) + number of effects
        packet_zc_hat_effect.fill_raw();
        socket_send!(context, packet_zc_hat_effect);
    }
    server.add_to_tick(GameEvent::CharacterLoadedFromClientSide(CharacterLoadedFromClientSide { char_id: session.char_id.unwrap() }), 2);
}
