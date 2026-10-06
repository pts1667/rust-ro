use std::net::Shutdown::Both;
use std::sync::Arc;

use byteorder::{LittleEndian, WriteBytesExt};
use models::status::KnownSkill;
use movement::position::Position;
use packets::packets::{
    CharacterInfoNeoUnion, Packet, PacketChDeleteChar4Reserved, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3,
    PacketChSelectChar, PacketChSendMapInfo, PacketCzEnter2, PacketCzRestart, PacketHcAcceptEnterNeoUnion,
    PacketHcAcceptEnterNeoUnionHeader, PacketHcAcceptMakecharNeoUnion, PacketHcBlockCharacter, PacketHcDeleteChar4Reserved,
    PacketHcNotifyZonesvr, PacketHcRefuseEnter, PacketMapConnection, PacketPincodeLoginstate, PacketZcAcceptEnter2,
    PacketZcInventoryExpansionInfo, PacketZcLoadConfirm, PacketZcOverweightPercent, PacketZcReqDisconnectAck2, PacketZcRestartAck,
    ZserverAddr,
};

use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::hotkey::Hotkey;
use crate::server::model::request::Request;
use crate::server::model::status::StatusFromDb;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick_client;

pub fn handle_char_enter(server: &Server, context: Request) {
    let packet_char_enter = cast!(context.packet(), PacketChEnter);
    if let Some(session) = server.sessions().find(packet_char_enter.aid) {
        if session.auth_code == packet_char_enter.auth_code && session.user_level == packet_char_enter.user_level {
            let session = Arc::new(session.recreate_with_char_socket(context.socket()));
            server.sessions().add(packet_char_enter.aid, session.clone());
            let packet_hc_accept_enter_neo_union: Box<dyn Packet> = server.runtime().block_on(async {
                let mut hc_accept_enter_neo_union = load_chars_info(session.account_id, server).await;
                if GlobalConfigService::instance().packetver() >= 20130000 {
                    let mut accept_enter_neo_union_header =
                        PacketHcAcceptEnterNeoUnionHeader::new(GlobalConfigService::instance().packetver());
                    accept_enter_neo_union_header.set_char_info(hc_accept_enter_neo_union);
                    accept_enter_neo_union_header.set_char_slot(12);
                    accept_enter_neo_union_header.set_premium_slot_end(12);
                    accept_enter_neo_union_header.set_premium_slot_start(12);
                    accept_enter_neo_union_header.set_packet_len(29);
                    accept_enter_neo_union_header.fill_raw_with_packetver(Some(server.packetver()));
                    let x: Box<dyn Packet> = Box::new(accept_enter_neo_union_header);
                    x
                } else {
                    hc_accept_enter_neo_union.fill_raw_with_packetver(Some(server.packetver()));
                    let x: Box<dyn Packet> = Box::new(hc_accept_enter_neo_union);
                    x
                }
            });
            let mut pincode_loginstate = PacketPincodeLoginstate::new(GlobalConfigService::instance().packetver());
            pincode_loginstate.set_aid(session.account_id);
            pincode_loginstate.set_pincode_seed(session.auth_code);
            pincode_loginstate.fill_raw();
            let mut packet_hc_block_character = PacketHcBlockCharacter::new(GlobalConfigService::instance().packetver());
            packet_hc_block_character
                .set_packet_length(PacketHcBlockCharacter::base_len(GlobalConfigService::instance().packetver()) as i16);
            packet_hc_block_character.fill_raw();
            // The pincode packet should be appended to PacketHcAcceptEnterNeoUnionHeader
            // packet
            let final_response_packet: Vec<u8> = chain_packets(vec![packet_hc_accept_enter_neo_union.as_ref(), &packet_hc_block_character]);
            let mut wtr = vec![];
            // A "account id packet" should be sent just before char info packet
            wtr.write_u32::<LittleEndian>(session.account_id)
                .expect("Unable to write Little endian u32 from session account id");
            socket_send_raw!(context, wtr);
            socket_send_raw!(context, final_response_packet);
            return;
        }
    }
    let mut res = PacketHcRefuseEnter::new(GlobalConfigService::instance().packetver());
    res.set_error_code(0);
    res.fill_raw();
    socket_send!(context, res);
}

pub fn handle_make_char(server: &Server, context: Request) {
    let mut char_model: Option<CharInsertModel> = None;
    if context.packet().as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
        let vit = 1;
        let max_hp = 40 * (100 + vit as i32) / 100;
        let int = 1;
        let max_sp = 40 * (100 + int as i32) / 100;
        let packet_make_char = cast!(context.packet(), PacketChMakeChar3);
        let name = packet_make_char.name.iter().filter(|c| **c != '\0').collect();
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id as i32,
            char_num: packet_make_char.char_num as i16,
            name,
            class: 0,
            zeny: 10000, // make this configurable
            status_point: 48,
            str: 1,
            agi: 1,
            vit,
            int,
            dex: 1,
            luk: 1,
            max_hp,
            hp: max_hp,
            max_sp,
            sp: max_sp,
            hair: packet_make_char.head,
            hair_color: packet_make_char.head_pal as i32,
            last_map: "new_1-1".to_string(), // make this configurable
            last_x: 53,
            last_y: 111,
            save_map: "new_1-1".to_string(), // make this configurable
            save_x: 53,
            save_y: 111,
            sex: if packet_make_char.sex == 1 {
                "M".to_string()
            } else {
                "F".to_string()
            },
            inventory_slots: context.configuration().game.max_inventory as i32,
        });
    } else if context.packet().as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
        let packet_make_char = cast!(context.packet(), PacketChMakeChar2);
        let vit = 5_i16;
        let max_hp = 40 * (100 + vit as i32) / 100;
        let int = 5;
        let max_sp = 40 * (100 + int as i32) / 100;
        let name = packet_make_char.name.iter().filter(|c| **c != '\0').collect();
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id as i32,
            char_num: packet_make_char.char_num as i16,
            name,
            class: 0,
            zeny: 10000, // make this configurable
            status_point: 48,
            str: 5_i16,
            agi: 5_i16,
            vit,
            int,
            dex: 5_i16,
            luk: 5_i16,
            max_hp,
            hp: max_hp,
            max_sp,
            sp: max_sp,
            hair: packet_make_char.head,
            hair_color: packet_make_char.head_pal as i32,
            last_map: "new_1-1".to_string(), // make this configurable
            last_x: 53,
            last_y: 111,
            save_map: "new_1-1".to_string(), // make this configurable
            save_x: 53,
            save_y: 111,
            sex: "M".to_string(), // TODO use account sex
            inventory_slots: context.configuration().game.max_inventory as i32,
        });
    } else if context.packet().as_any().downcast_ref::<PacketChMakeChar>().is_some() {
        let packet_make_char = cast!(context.packet(), PacketChMakeChar);
        let vit = packet_make_char.vit as i16;
        let max_hp = 40 * (100 + vit as i32) / 100;
        let int = packet_make_char.int as i16;
        let max_sp = 40 * (100 + int as i32) / 100;
        let name = packet_make_char.name.iter().filter(|c| **c != '\0').collect();
        char_model = Some(CharInsertModel {
            account_id: context.session().account_id as i32,
            char_num: packet_make_char.char_num as i16,
            name,
            class: 0,
            zeny: 10000, // make this configurable
            status_point: 48,
            str: packet_make_char.str as i16,
            agi: packet_make_char.agi as i16,
            vit,
            int,
            dex: packet_make_char.dex as i16,
            luk: packet_make_char.luk as i16,
            max_hp,
            hp: max_hp,
            max_sp,
            sp: max_sp,
            hair: packet_make_char.head,
            hair_color: packet_make_char.head_pal as i32,
            last_map: "new_1-1".to_string(), // make this configurable
            last_x: 53,
            last_y: 111,
            save_map: "new_1-1".to_string(), // make this configurable
            save_x: 53,
            save_y: 111,
            sex: "M".to_string(), // TODO use account sex
            inventory_slots: context.configuration().game.max_inventory as i32,
        });
    }
    if char_model.is_none() {
        error!("Char model is not initialized, probably packet was not recognized");
        return;
    }

    let created_char = server.runtime().block_on(async {
        let char_model = char_model.unwrap();
        let name = char_model.name.as_str();
        server.repository.character_insert(&char_model).await.unwrap();
        // TODO add default stuff
        let created_char: CharacterInfoNeoUnionWrapped = server.repository.character_info(char_model.account_id, name).await.unwrap();
        created_char.data
    });
    let mut packet_hc_accept_makechar_neo_union = PacketHcAcceptMakecharNeoUnion::new(GlobalConfigService::instance().packetver());
    packet_hc_accept_makechar_neo_union.set_charinfo(created_char);
    packet_hc_accept_makechar_neo_union.fill_raw_with_packetver(Some(server.packetver()));
    socket_send!(context, packet_hc_accept_makechar_neo_union);
}

pub fn handle_delete_reserved_char(server: &Server, context: Request) {
    let packet_delete_reserved_char = cast!(context.packet(), PacketChDeleteChar4Reserved);
    server.runtime().block_on(async {
        server
            .repository
            .character_delete_reserved(context.session().account_id, packet_delete_reserved_char.gid)
            .await
            .unwrap();
    });
    let mut packet_hc_delete_char4reserved = PacketHcDeleteChar4Reserved::new(GlobalConfigService::instance().packetver());
    packet_hc_delete_char4reserved.set_gid(packet_delete_reserved_char.gid);
    packet_hc_delete_char4reserved.set_delete_reserved_date(24 * 60 * 60);
    packet_hc_delete_char4reserved.set_result(1);
    packet_hc_delete_char4reserved.fill_raw();
    socket_send!(context, packet_hc_delete_char4reserved);
}

pub fn handle_select_char(server: &Server, context: Request) {
    let packet_select_char = cast!(context.packet(), PacketChSelectChar);
    let selected_session = match server.await_character_selection(context.session()) {
        Ok(session) => session,
        Err(error) => {
            warn!("Character selection rejected: {error}");
            let mut packet = PacketHcRefuseEnter::new(server.packetver());
            packet.set_error_code(0);
            packet.fill_raw();
            socket_send!(context, packet);
            return;
        }
    };
    let session_id = selected_session.account_id;
    let char_model: CharSelectModel = server.runtime().block_on(async {
        if let Some(char_id) = selected_session.char_id {
            server.repository.character_with_id_fetch(char_id).await.unwrap()
        } else {
            server
                .repository
                .character_fetch(session_id, packet_select_char.char_num)
                .await
                .unwrap()
        }
    });
    let skills: Vec<KnownSkill> = server
        .runtime()
        .block_on(async { server.repository.character_skills(char_model.char_id as u32).await.unwrap() });
    let hotkeys: Vec<Hotkey> = server
        .runtime()
        .block_on(async { server.repository.load_hotkeys(char_model.char_id as u32).await.unwrap() });

    let char_id: u32 = char_model.char_id as u32;
    let last_x: u16 = char_model.last_x as u16;
    let last_y: u16 = char_model.last_y as u16;
    let mut last_map: String = char_model.last_map.clone();
    if last_map.is_empty() {
        last_map = "prontera".to_string();
    }

    let mut character = Character::new(
        char_model.name.clone(),
        char_id,
        session_id,
        StatusFromDb::from_char_model(&char_model, &server.configuration.game, skills),
        last_x,
        last_y,
        0,
        last_map,
        if char_model.sex == "M" { 1 } else { 0 },
        hotkeys,
    );
    character.save_map = char_model.save_map.clone();
    character.save_x = char_model.save_x.max(0) as u16;
    character.save_y = char_model.save_y.max(0) as u16;
    character
        .position_revision
        .store(char_model.position_revision, std::sync::atomic::Ordering::Relaxed);
    character.set_options(char_model.option as u32 as u64);
    character.karma = char_model.karma;
    character.manner = char_model.manner;
    match server.repository.character_game_systems(char_id) {
        Ok(systems) => character.game_systems = systems,
        Err(error) => {
            error!("Failed to load character game systems: {error}");
            return;
        }
    }
    match server.repository.account_game_systems(character.account_id) {
        Ok(systems) => character.account_game_systems = systems,
        Err(error) => {
            error!("Failed to load account game systems: {error}");
            return;
        }
    }
    if character.game_systems.guild_id != 0 {
        match server.repository.guild(character.game_systems.guild_id) {
            Ok(Some(guild)) => character.guild_name = guild.name,
            Ok(None) => {}
            Err(error) => {
                error!("Failed to load guild: {error}");
                return;
            }
        }
    }
    character.refresh_script_context();
    let char_id = character.char_id;
    let map = match server.admit_selected_character(selected_session, character) {
        Ok(map) => map,
        Err(error) => {
            warn!("Character admission rejected: {error}");
            let mut packet = PacketHcRefuseEnter::new(server.packetver());
            packet.set_error_code(0);
            packet.fill_raw();
            socket_send!(context, packet);
            return;
        }
    };
    let mut map_name = [0 as char; 16];
    map.fill_char_array(map_name.as_mut());
    if server.packetver() < 20170329 {
        let mut packet_ch_send_map_info = PacketHcNotifyZonesvr::new(GlobalConfigService::instance().packetver());
        packet_ch_send_map_info.set_gid(char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        let mut zserver_addr = ZserverAddr::new(GlobalConfigService::instance().packetver());
        zserver_addr.set_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        zserver_addr.set_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_addr(zserver_addr);
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    } else {
        let mut packet_ch_send_map_info = PacketChSendMapInfo::new(GlobalConfigService::instance().packetver());
        packet_ch_send_map_info.set_gid(char_id);
        packet_ch_send_map_info.set_map_name(map_name);
        packet_ch_send_map_info.set_map_server_port(server.configuration.server.port as i16);
        packet_ch_send_map_info.set_map_server_ip(16777343); // 7F 00 00 01 -> to little endian -> 01 00 00 7F
        packet_ch_send_map_info.fill_raw();
        socket_send!(context, packet_ch_send_map_info);
    }
}

pub fn handle_enter_game(server: &Server, context: Request) {
    let aid;
    let auth_code;
    info!("handle_enter_game");
    if context.packet().as_any().downcast_ref::<PacketCzEnter2>().is_some() {
        let packet_enter_game = cast!(context.packet(), PacketCzEnter2);
        aid = packet_enter_game.aid;
        auth_code = packet_enter_game.auth_code;
    } else {
        error!("Not recognized PacketCzEnterX");
        return;
    }
    let Some(session) = server.sessions().find(aid) else {
        write_lock!(context.socket())
            .shutdown(Both)
            .expect("Unable to shutdown incoming socket. Shutdown was done because session does not exists");
        return;
    };
    if auth_code != session.auth_code || session.char_id.is_none() {
        write_lock!(context.socket())
            .shutdown(Both)
            .expect("Unable to shutdown incoming socket. Shutdown was done because packet auth_code mismatching session auth_code");
        return;
    }
    let entry = match server.admit_character_map_entry(session, context.socket()) {
        Ok(entry) => entry,
        Err(error) => {
            warn!("Map entry rejected: {error}");
            let _ = write_lock!(context.socket()).shutdown(Both);
            return;
        }
    };
    let mut packet_map_connection = PacketMapConnection::new(GlobalConfigService::instance().packetver());
    packet_map_connection.set_aid(aid);
    packet_map_connection.fill_raw();

    socket_send!(context, packet_map_connection);

    /*
    Client expect multiple packets in response to packet PacketCzEnter2
    */
    let mut packet_inventory_expansion_info = PacketZcInventoryExpansionInfo::new(GlobalConfigService::instance().packetver());
    packet_inventory_expansion_info.fill_raw();
    let mut packet_overweight_percent = PacketZcOverweightPercent::new(GlobalConfigService::instance().packetver());
    packet_overweight_percent.fill_raw();
    let mut packet_accept_enter = PacketZcAcceptEnter2::new(GlobalConfigService::instance().packetver());
    packet_accept_enter.set_start_time(get_tick_client());
    packet_accept_enter.set_x_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_y_size(5); // Commented as not used, set at 5 in Hercules
    packet_accept_enter.set_font(0);
    packet_accept_enter.set_pos_dir(
        Position {
            x: entry.x,
            y: entry.y,
            dir: entry.direction,
        }
        .to_pos(),
    );
    packet_accept_enter.fill_raw();

    socket_send!(context, packet_accept_enter);

    /*
     * Inventory
     */
    server.add_to_next_tick(GameEvent::CharacterMapReady(
        crate::server::model::character_lifecycle::CharacterMapReady { session: entry.session },
    ));
}

pub fn handle_restart(server: &Server, context: Request) {
    let packet_restart = cast!(context.packet(), PacketCzRestart);
    let session = context.session();
    if !session
        .map_server_socket
        .as_ref()
        .is_some_and(|socket| std::sync::Arc::ptr_eq(socket, &context.socket()))
    {
        return;
    }
    if packet_restart.atype == 0 {
        server.add_to_next_tick(GameEvent::CharacterRespawn(
            crate::server::model::character_lifecycle::CharacterRespawn { session },
        ));
        return;
    }
    if packet_restart.atype != 1 {
        return;
    }
    server.add_to_next_tick(GameEvent::CharacterLogout(
        crate::server::model::character_lifecycle::CharacterLogout { session, restart: true },
    ));

    let mut restart_ack = PacketZcRestartAck::new(GlobalConfigService::instance().packetver());
    restart_ack.set_atype(packet_restart.atype);
    restart_ack.fill_raw();
    socket_send!(context, restart_ack);
}

pub fn handle_disconnect(server: &Server, context: Request) {
    let session = context.session();
    if !session
        .map_server_socket
        .as_ref()
        .is_some_and(|socket| std::sync::Arc::ptr_eq(socket, &context.socket()))
    {
        return;
    }
    server.add_to_next_tick(GameEvent::CharacterLogout(
        crate::server::model::character_lifecycle::CharacterLogout { session, restart: false },
    ));

    let mut disconnect_ack = PacketZcReqDisconnectAck2::new(GlobalConfigService::instance().packetver());
    disconnect_ack.fill_raw();
    socket_send!(context, disconnect_ack);
}

/// First client version with `ZC_NOTIFY_ACTORINIT` (`0x0b1b`); earlier clients get no reply.
const LOAD_CONFIRM_PACKETVER: u32 = 20190403;

pub fn handle_blocking_play_cancel(context: Request) {
    let packetver = GlobalConfigService::instance().packetver();
    if packetver < LOAD_CONFIRM_PACKETVER {
        return;
    }
    let mut packet_zc_load_confirm = PacketZcLoadConfirm::new(packetver);
    packet_zc_load_confirm.fill_raw();
    socket_send!(context, packet_zc_load_confirm);
}

async fn load_chars_info(account_id: u32, server: &Server) -> PacketHcAcceptEnterNeoUnion {
    let row_results = server.repository.characters_info(account_id).await;
    let mut accept_enter_neo_union = PacketHcAcceptEnterNeoUnion::new(GlobalConfigService::instance().packetver());
    accept_enter_neo_union.set_packet_length((27 + row_results.len() * CharacterInfoNeoUnion::base_len(server.packetver())) as i16);
    accept_enter_neo_union.set_char_info(
        row_results
            .iter()
            .map(|wrapped| {
                let mut character_info = wrapped.data.clone();
                if character_info.head_bottom > 0 {
                    character_info.head_bottom = GlobalConfigService::instance()
                        .get_item(character_info.head_bottom as i32)
                        .view
                        .unwrap_or(0) as u16;
                }
                if character_info.head_top > 0 {
                    character_info.head_top = GlobalConfigService::instance()
                        .get_item(character_info.head_top as i32)
                        .view
                        .unwrap_or(0) as u16;
                }
                if character_info.head_mid > 0 {
                    character_info.head_mid = GlobalConfigService::instance()
                        .get_item(character_info.head_mid as i32)
                        .view
                        .unwrap_or(0) as u16;
                }
                character_info
            })
            .collect::<Vec<CharacterInfoNeoUnion>>(),
    );
    accept_enter_neo_union.set_premium_start_slot(12);
    accept_enter_neo_union.set_premium_end_slot(12);
    accept_enter_neo_union.set_total_slot_num(12);
    accept_enter_neo_union
}
