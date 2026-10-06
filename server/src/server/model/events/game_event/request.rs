use movement::position::Position;
use packets::packets::{Packet, PacketZcAckReqnameall, PacketZcAckReqnameall2};

use super::*;
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::request_handler::atcommand::handle_atcommand;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;
use crate::util::string::StringUtil;

/// First client version where `ZC_ACK_REQNAMEALL` became `0x0a30` and gained a title id.
const REQNAMEALL2_PACKETVER: u32 = 20150225;

/// Planned and executed by the movement thread, which owns movement state access.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRequestMove {
    pub char_id: u32,
    pub destination: Position,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterRequestName {
    pub char_id: u32,
    pub gid: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CharacterChat {
    pub char_id: u32,
    pub message: String,
}

impl GameEventHandler for CharacterRequestMove {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, _server: &Server, _state: &mut ServerState, _tick: u128) -> Result<(), String> {
        // handled by dedicated thread
        Ok(())
    }
}

impl GameEventHandler for CharacterRequestName {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterRequestName { char_id, gid } = self;
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        let lookup_id = if gid == character.account_id { char_id } else { gid };
        let map_item = state
            .map_item(lookup_id, character.current_map_name(), character.current_map_instance())
            .ok_or_else(|| format!("Can't find map item with id: {gid}"))?;
        let mut name: [char; 24] = [0 as char; 24];
        #[cfg(feature = "debug_mob_movement")]
        {
            map_item.id().to_string().fill_char_array(name.as_mut());
        }
        #[cfg(not(feature = "debug_mob_movement"))]
        {
            let map_item_name = state
                .map_item_name(&map_item, character.current_map_name(), character.current_map_instance())
                .unwrap_or_else(|| "unknown".to_string());
            map_item_name.fill_char_array(name.as_mut());
        }
        let mut guild_field: [char; 24] = [0 as char; 24];
        let mut position_field: [char; 24] = [0 as char; 24];
        if let Some((guild_name, castle_name)) =
            server.guardian_label(state, character.current_map_name(), character.current_map_instance(), lookup_id)
        {
            guild_name.fill_char_array(guild_field.as_mut());
            castle_name.fill_char_array(position_field.as_mut());
        }
        let packetver = GlobalConfigService::instance().packetver();
        let raw = if packetver >= REQNAMEALL2_PACKETVER {
            let mut packet = PacketZcAckReqnameall2::new(packetver);
            packet.set_gid(gid);
            packet.set_name(name);
            packet.set_guild_name(guild_field);
            packet.set_position_name(position_field);
            packet.fill_raw();
            std::mem::take(packet.raw_mut())
        } else {
            let mut packet = PacketZcAckReqnameall::new(packetver);
            packet.set_aid(gid);
            packet.set_cname(name);
            packet.set_gname(guild_field);
            packet.set_rname(position_field);
            packet.fill_raw();
            std::mem::take(packet.raw_mut())
        };
        server
            .server_service()
            .notification_sender()
            .send(Notification::Char(CharNotification::new(char_id, raw)))
            .map_err(|_| "Failed to send name reply to client".to_string())
    }
}

impl GameEventHandler for CharacterChat {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let CharacterChat { char_id, message } = self;
        let character = state.get_character(char_id).ok_or("Character is not in game")?;
        debug!("Received chat from {}: {:?}", character.name, message);
        if !message.starts_with(format!("{} : @", character.name).as_str()) {
            return Ok(());
        }
        if state.map_flags(&character.map_instance_key).enabled(crate::server::model::map_flags::MapFlag::NoCommand) {
            debug!("Command rejected on a map with nocommand: {}", character.name);
            return Ok(());
        }
        handle_atcommand(server, state, char_id, &message);
        Ok(())
    }
}
