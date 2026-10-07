use std::sync::Arc;

use packets::packets::{
    PacketCaLogin, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3, PacketChSelectChar,
    PacketCzAckSelectDealtype, PacketCzBlockingPlayCancel, PacketCzChooseMenu, PacketCzContactnpc, PacketCzEnter2, PacketCzInputEditdlg,
    PacketCzInputEditdlgstr, PacketCzItemPickup, PacketCzItemThrow, PacketCzNotifyActorinit, PacketCzPcPurchaseItemlist,
    PacketCzPcSellItemlist, PacketCzPlayerChat, PacketCzReqDisconnect, PacketCzReqDisconnect2, PacketCzReqItemcomposition, PacketCzReqItemcompositionList,
    PacketCzReqNextScript, PacketCzReqTakeoffEquip, PacketCzReqWearEquip, PacketCzReqname, PacketCzReqnameall2, PacketCzRequestAct,
    PacketCzRequestMove, PacketCzRequestMove2, PacketCzRequestTime, PacketCzRestart, PacketCzShortcutKeyChange, PacketCzStatusChange,
    PacketCzUpgradeSkilllevel, PacketCzUseItem, PacketCzUseSkill, PacketUnknown, PacketZcNotifyTime,
};

use crate::packets::packets::Packet;
use crate::server::Server;
use crate::server::model::request::Request;
use crate::server::request_handler::action::action::{handle_action, handle_pickup_item};
use crate::server::request_handler::action::character::{handle_player_skill_allocation, handle_player_status_change};
use crate::server::request_handler::action::hotkey::handle_shortcut_change;
use crate::server::request_handler::action::item::{
    handle_player_card_composition_list, handle_player_drop_item, handle_player_equip_item, handle_player_slot_card,
    handle_player_takeoff_equip_item, handle_player_use_item,
};
use crate::server::request_handler::action::npc::{
    handle_contact_npc, handle_player_choose_menu, handle_player_input_number, handle_player_input_string, handle_player_next,
    handle_player_purchase_items, handle_player_select_deal_type, handle_player_sell_items,
};
use crate::server::request_handler::action::skill::handle_use_skill;
use crate::server::request_handler::char::{
    handle_blocking_play_cancel, handle_char_enter, handle_disconnect, handle_enter_game, handle_make_char, handle_restart,
    handle_select_char,
};
use crate::server::request_handler::chat::handle_chat;
use crate::server::request_handler::login::handle_login;
use crate::server::request_handler::map::{handle_char_loaded_client_side, handle_map_item_name};
use crate::server::request_handler::movement::handle_char_move;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::tick::{get_tick, get_tick_client};

/// The character behind the connection that sent the packet; the request has no session bound until the final dispatch below.
pub(crate) fn connection_char_id(server: &Server, context: &Request) -> Option<u32> {
    let session_id = server.ensure_session_exists(&context.socket())?;
    server.sessions().find(session_id)?.char_id
}

pub mod action;
pub mod atcommand;
pub mod atcommand_admin;
pub mod atcommand_extra;
/**
 * This module implement client requests handler.
 */
pub mod char;
pub mod char_requests;
pub mod client_command;
pub mod chat;
pub mod framing;
pub mod login;
pub mod map;
pub mod movement;
pub(crate) mod player_trade;
pub mod instance;
pub mod quest;
pub mod script_operations;
pub mod social;
mod talkie_box;

pub fn handle(server: Arc<Server>, mut context: Request) {
    match player_trade::handle_raw(server.as_ref(), &context) {
        Ok(true) => return,
        Ok(false) => {}
        Err(error) => {
            warn!("Rejected malformed player trade packet: {}", error);
            return;
        }
    }
    match char_requests::handle_raw(server.as_ref(), &context) {
        Ok(true) => return,
        Ok(false) => {}
        Err(error) => {
            warn!("Rejected malformed character server packet: {}", error);
            return;
        }
    }
    if social::handle_raw(server.as_ref(), &context) || quest::handle_raw(server.as_ref(), &context) || instance::handle_raw(server.as_ref(), &context) {
        return;
    }
    match script_operations::handle_raw(server.as_ref(), &context) {
        Ok(true) => return,
        Ok(false) => {}
        Err(error) => {
            warn!("Rejected malformed script operation packet: {}", error);
            return;
        }
    }
    match crate::server::service::script_world_service::decode_request(context.packet().raw(), server.packetver()) {
        Ok(Some(request)) => {
            let Some(session_id) = server.ensure_session_exists(&context.socket()) else {
                return;
            };
            let Some(session) = server.sessions().find(session_id) else {
                return;
            };
            let Some(char_id) = session.char_id else {
                return;
            };
            let Some(socket) = session.map_server_socket.as_ref() else {
                return;
            };
            if !Arc::ptr_eq(socket, &context.socket()) {
                return;
            }
            server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::ScriptWorld(
                crate::server::model::events::game_event::ScriptWorld { char_id, request },
            ));
            return;
        }
        Err(error) => {
            warn!("Rejected malformed world packet: {}", error);
            return;
        }
        Ok(None) => {}
    }
    if context.packet().as_any().downcast_ref::<PacketUnknown>().is_some() {
        error!(
            "Unknown packet {} of length {}: {:02X?}",
            context.packet().id(GlobalConfigService::instance().packetver()),
            context.packet().raw().len(),
            context.packet().raw()
        );
        return;
    }
    // Login
    if context.packet().as_any().downcast_ref::<PacketCaLogin>().is_some() {
        debug!("PacketCaLogin");
        return handle_login(server, context);
    }
    // Char selection
    if context.packet().as_any().downcast_ref::<PacketChEnter>().is_some() {
        debug!("PacketChEnter");
        return handle_char_enter(server.as_ref(), context);
    }

    // Enter game
    if context.packet().as_any().downcast_ref::<PacketCzEnter2>().is_some() {
        debug!("PacketCzEnter2");
        // A char session exist, but not yet map session
        return handle_enter_game(server.as_ref(), context);
    }
    /*
     *  Having a session is required for any packets below
     */
    let session_id = server.ensure_session_exists(&context.socket());
    if session_id.is_none() {
        return;
    }
    let session = server.sessions().get(session_id.unwrap());
    if let Some(session_record) = server.get_recording_session(session_id.unwrap()) {
        session_record.record(
            get_tick(),
            context.packet().id(server.configuration.packetver()).to_owned(),
            context.packet().name().to_owned(),
            context.packet(),
        );
    }
    context.set_session(session);
    // Char creation
    if context.packet().as_any().downcast_ref::<PacketChMakeChar>().is_some() {
        debug!("PacketChMakeChar");
        return handle_make_char(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
        debug!("PacketChMakeChar2");
        return handle_make_char(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
        debug!("PacketChMakeChar3");
        return handle_make_char(server.as_ref(), context);
    }
    // Select char
    if context.packet().as_any().downcast_ref::<PacketChSelectChar>().is_some() {
        debug!("PacketChSelectChar");
        return handle_select_char(server.as_ref(), context);
    }
    // Game menu "Character select"
    if context.packet().as_any().downcast_ref::<PacketCzRestart>().is_some() {
        debug!("PacketCzRestart");
        return handle_restart(server.as_ref(), context);
    }
    // Game menu "Exit to windows"
    if context.packet().as_any().downcast_ref::<PacketCzReqDisconnect2>().is_some()
        || context.packet().as_any().downcast_ref::<PacketCzReqDisconnect>().is_some()
    {
        debug!("PacketCzReqDisconnect");
        return handle_disconnect(server.as_ref(), context);
    }
    // Player click on map cell
    if context.packet().as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
        debug!("PacketCzRequestMove2");
        return handle_char_move(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzRequestMove>().is_some() {
        debug!("PacketCzRequestMove");
        return handle_char_move(server.as_ref(), context);
    }
    // Client notify player has been loaded
    if context.packet().as_any().downcast_ref::<PacketCzNotifyActorinit>().is_some() {
        debug!("PacketCzNotifyActorinit");
        return handle_char_loaded_client_side(server.as_ref(), context);
    }
    // Client send PACKET_CZ_BLOCKING_PLAY_CANCEL after char has loaded
    if context.packet().as_any().downcast_ref::<PacketCzBlockingPlayCancel>().is_some() {
        debug!("PacketCzBlockingPlayCancel");
        return handle_blocking_play_cancel(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzRequestAct>().is_some() {
        debug!("PacketCzRequestAct");
        return handle_action(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzItemPickup>().is_some() {
        debug!("PacketCzItemPickup");
        return handle_pickup_item(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
        debug!("PacketCzReqnameall2");
        return handle_map_item_name(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzReqname>().is_some() {
        debug!("PacketCzReqname");
        return handle_map_item_name(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzPlayerChat>().is_some() {
        debug!("PacketCzPlayerChat");
        return handle_chat(server.as_ref(), context);
    }

    // NPC interactions
    if context.packet().as_any().downcast_ref::<PacketCzContactnpc>().is_some() {
        debug!("PacketCzContactnpc");
        return handle_contact_npc(server, context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzReqNextScript>().is_some() {
        debug!("PacketCzReqNextScript");
        return handle_player_next(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzChooseMenu>().is_some() {
        debug!("PacketCzChooseMenu");
        return handle_player_choose_menu(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzInputEditdlg>().is_some() {
        debug!("PacketCzInputEditdlg");
        return handle_player_input_number(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzInputEditdlgstr>().is_some() {
        debug!("PacketCzInputEditdlgstr");
        return handle_player_input_string(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzAckSelectDealtype>().is_some() {
        debug!("PacketCzAckSelectDealtype");
        return handle_player_select_deal_type(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzPcPurchaseItemlist>().is_some() {
        debug!("PacketCzPcPurchaseItemlist");
        return handle_player_purchase_items(context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzPcSellItemlist>().is_some() {
        debug!("PacketCzPcSellItemlist");
        return handle_player_sell_items(context);
    }
    // End NPC interaction

    // Item interaction
    if context.packet().as_any().downcast_ref::<PacketCzUseItem>().is_some() {
        debug!("PacketCzUseItem");
        return handle_player_use_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzReqWearEquip>().is_some() {
        debug!("PacketCzReqWearEquip");
        return handle_player_equip_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzReqTakeoffEquip>().is_some() {
        debug!("PacketCzReqTakeoffEquip");
        return handle_player_takeoff_equip_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzItemThrow>().is_some() {
        debug!("PacketCzItemThrow");
        return handle_player_drop_item(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzReqItemcompositionList>().is_some() {
        debug!("PacketCzReqItemcompositionList");
        return handle_player_card_composition_list(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzReqItemcomposition>().is_some() {
        debug!("PacketCzReqItemcomposition");
        return handle_player_slot_card(server.as_ref(), context);
    }
    // End Item interaction

    // Stats
    if context.packet().as_any().downcast_ref::<PacketCzStatusChange>().is_some() {
        debug!("PacketCzStatusChange");
        return handle_player_status_change(server.as_ref(), context);
    }
    // End stats
    // Skills
    if context.packet().as_any().downcast_ref::<PacketCzUpgradeSkilllevel>().is_some() {
        debug!("PacketCzUpgradeSkilllevel");
        return handle_player_skill_allocation(server.as_ref(), context);
    }
    if context.packet().as_any().downcast_ref::<PacketCzUseSkill>().is_some() {
        debug!("PacketCzUseSkill");
        return handle_use_skill(server.as_ref(), context);
    }
    // End Skills

    // Shortcuts change
    if context.packet().as_any().downcast_ref::<PacketCzShortcutKeyChange>().is_some() {
        debug!("PacketCzShortcutKeyChange");
        return handle_shortcut_change(server.as_ref(), context);
    }

    if context.packet().as_any().downcast_ref::<PacketCzRequestTime>().is_some() {
        let mut packet_zc_notify_time = PacketZcNotifyTime::new(GlobalConfigService::instance().packetver());
        packet_zc_notify_time.set_time(get_tick_client());
        packet_zc_notify_time.fill_raw();
        socket_send!(context, packet_zc_notify_time);
        return;
    }

    if client_command::handle(server.as_ref(), &context) {
        return;
    }

    if context.packet().id(GlobalConfigService::instance().packetver()) == "0x6003" // PacketCzRequestTime2
        || context.packet().id(GlobalConfigService::instance().packetver()) == "0x187"
    // PacketPing
    {
        // TODO handle those packets
        return;
    }
    context.packet().display();
    context.packet().pretty_debug();
}
