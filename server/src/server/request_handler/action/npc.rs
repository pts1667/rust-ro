use std::sync::Arc;

use packets::packets::{
    PacketCzAckSelectDealtype, PacketCzChooseMenu, PacketCzContactnpc, PacketCzInputEditdlg, PacketCzInputEditdlgstr,
    PacketCzPcPurchaseItemlist, PacketCzPcSellItemlist,
};

use crate::server::Server;
use crate::server::model::request::Request;
use crate::server::script::PlayerInput;
use crate::server::model::events::game_event::{GameEvent, NpcContact};

pub fn handle_contact_npc(server: Arc<Server>, context: Request) {
    let packet = cast!(context.packet(), PacketCzContactnpc);
    let session = context.session();
    server.add_to_next_tick(GameEvent::NpcContact(NpcContact { char_id: session.char_id(), account_id: session.account_id, npc_id: packet.naid }));
}

fn send(context: Request, input: PlayerInput) {
    let session = context.session();
    let sender = session.script_handler_channel_sender.lock().unwrap().clone();
    let Some(sender) = sender else {
        script_debug!("NPC input {input:?} ignored for char {}: no active conversation", session.char_id());
        return;
    };
    script_debug!("NPC input {input:?} from char {}", session.char_id());
    if let Err(error) = sender.try_send(input) {
        script_debug!("NPC input dropped for char {}: {error}", session.char_id());
    }
}

pub fn handle_player_next(context: Request) {
    send(context, PlayerInput::Next);
}
pub fn handle_player_choose_menu(context: Request) {
    let packet = cast!(context.packet(), PacketCzChooseMenu);
    let option = packet.num;
    send(context, PlayerInput::Selection(option));
}
pub fn handle_player_input_number(context: Request) {
    let packet = cast!(context.packet(), PacketCzInputEditdlg);
    let number = packet.value;
    send(context, PlayerInput::Number(number));
}
pub fn handle_player_input_string(context: Request) {
    let packet = cast!(context.packet(), PacketCzInputEditdlgstr);
    if let Ok(text) = String::from_utf8(packet.msg_raw.clone()) {
        send(context, PlayerInput::Text(text.trim_end_matches(char::from(0)).to_string()));
    }
}
pub fn handle_player_select_deal_type(context: Request) {
    let packet = cast!(context.packet(), PacketCzAckSelectDealtype);
    let kind = packet.atype;
    send(context, PlayerInput::DealType(kind));
}
pub fn handle_player_purchase_items(context: Request) {
    let packet = cast!(context.packet(), PacketCzPcPurchaseItemlist);
    let items = packet.item_list.iter().map(|item| (item.itid as u32, item.count)).collect();
    send(context, PlayerInput::Purchases(items));
}
pub fn handle_player_sell_items(context: Request) {
    let packet = cast!(context.packet(), PacketCzPcSellItemlist);
    let items = packet.item_list.iter().map(|item| (item.index as usize, item.count)).collect();
    send(context, PlayerInput::Sales(items));
}
