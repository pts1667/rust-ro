use movement::position::Position;
use packets::packets::{Packet, PacketZcAckReqnameall2};

use super::*;
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::request_handler::atcommand::handle_atcommand;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;
use crate::util::string::StringUtil;

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
        let map_item = state
            .map_item(gid, character.current_map_name(), character.current_map_instance())
            .ok_or_else(|| format!("Can't find map item with id: {gid}"))?;
        let mut packet = PacketZcAckReqnameall2::new(GlobalConfigService::instance().packetver());
        packet.set_gid(gid);
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
        packet.set_name(name);
        if let Some((guild_name, castle_name)) =
            server.guardian_label(state, character.current_map_name(), character.current_map_instance(), gid)
        {
            let mut field: [char; 24] = [0 as char; 24];
            guild_name.fill_char_array(field.as_mut());
            packet.set_guild_name(field);
            let mut field: [char; 24] = [0 as char; 24];
            castle_name.fill_char_array(field.as_mut());
            packet.set_position_name(field);
        }
        packet.fill_raw();
        server
            .server_service()
            .notification_sender()
            .send(Notification::Char(CharNotification::new(char_id, std::mem::take(packet.raw_mut()))))
            .map_err(|_| "Failed to send notification packet_zc_ack_reqnameall2 to client".to_string())
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
