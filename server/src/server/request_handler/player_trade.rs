use std::sync::Arc;

use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, PlayerTradeAction};
use crate::server::model::request::Request;

pub(crate) fn handle_raw(server: &Server, context: &Request) -> Result<bool, String> {
    let Some(request) = crate::server::service::player_trade_service::decode_request(context.packet().raw())? else {
        return Ok(false);
    };
    let Some(account_id) = server.ensure_session_exists(&context.socket()) else {
        return Ok(true);
    };
    let Some(session) = server.sessions().find(account_id) else {
        return Ok(true);
    };
    let Some(char_id) = session.char_id else {
        return Ok(true);
    };
    let Some(socket) = session.map_server_socket.as_ref() else {
        return Ok(true);
    };
    if !Arc::ptr_eq(socket, &context.socket()) {
        return Ok(true);
    }
    server.add_to_next_tick(GameEvent::PlayerTrade(PlayerTradeAction {
        char_id,
        account_id,
        auth_code: session.auth_code,
        request,
    }));
    Ok(true)
}
