use models::enums::script::{BroadcastFlag, ScriptSendTarget};
use models::enums::EnumWithMaskValueU32;
use script_sdk::{Function, Value};

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptBroadcast};
use crate::server::service::item_service::ItemService;
use crate::server::state::character::Character;
use crate::server::Server;

pub(crate) fn presentation_constant(name: &str) -> Option<Value> {
    Some(match name {
        "bc_all" | "bc_pc" | "bc_yellow" => 0.into(),
        "bc_map" => (BroadcastFlag::Map.as_flag() as i32).into(),
        "bc_area" => (BroadcastFlag::Area.as_flag() as i32).into(),
        "bc_self" => ((BroadcastFlag::Map.as_flag() | BroadcastFlag::Area.as_flag()) as i32).into(),
        "bc_npc" => (BroadcastFlag::NpcSource.as_flag() as i32).into(),
        "bc_blue" => (BroadcastFlag::Blue.as_flag() as i32).into(),
        "bc_woe" => (BroadcastFlag::WarOfEmperium.as_flag() as i32).into(),
        "ALL_CLIENT" => (ScriptSendTarget::All as i32).into(),
        "ALL_SAMEMAP" => (ScriptSendTarget::Map as i32).into(),
        "AREA" => (ScriptSendTarget::Area as i32).into(),
        "AREA_WOS" => (ScriptSendTarget::AreaWithoutSelf as i32).into(),
        "SELF" => (ScriptSendTarget::SelfOnly as i32).into(),
        _ => return None,
    })
}

pub(crate) fn announcement_packet(arguments: &[Value]) -> Result<Vec<u8>, String> {
    let message = arguments.first().ok_or("Missing announcement")?.string_value()?;
    if message.len() > 16_000 || message.as_bytes().contains(&0) { return Err("Announcement is too long or contains a NUL".into()); }
    let flags = arguments.get(1).ok_or("Missing announcement flags")?.number_value()? as u32;
    let mut bytes = if let Some(color) = arguments.get(2) {
        let color = match color {
            Value::Number(color) => *color as u32,
            Value::String(color) => u32::from_str_radix(color.trim_start_matches("0x").trim_start_matches("0X"), 16).map_err(|_| "Invalid announcement color")?,
            _ => return Err("Invalid announcement color".into()),
        };
        let mut bytes = 0x01c3_u16.to_le_bytes().to_vec();
        bytes.extend_from_slice(&((16 + message.len() + 1) as u16).to_le_bytes());
        bytes.extend_from_slice(&color.to_le_bytes());
        for (index, default) in [(3, 400), (4, 12), (5, 0), (6, 0)] {
            let value = arguments.get(index).map(Value::number_value).transpose()?.unwrap_or(default);
            bytes.extend_from_slice(&i16::try_from(value).map_err(|_| "Invalid announcement font")?.to_le_bytes());
        }
        bytes
    } else {
        let prefix = if flags & BroadcastFlag::Blue.as_flag() != 0 { b"blue".as_slice() }
            else if flags & BroadcastFlag::WarOfEmperium.as_flag() != 0 { b"ssss".as_slice() } else { &[] };
        let mut bytes = 0x009a_u16.to_le_bytes().to_vec();
        bytes.extend_from_slice(&((4 + prefix.len() + message.len() + 1) as u16).to_le_bytes());
        bytes.extend_from_slice(prefix);
        bytes
    };
    bytes.extend_from_slice(message.as_bytes());
    bytes.push(0);
    Ok(bytes)
}

impl ItemService {
    pub(crate) fn present_effect(&self, server: &Server, character: &Character, function: Function, arguments: &[Value]) -> Result<(), String> {
        let packet = match function {
            Function::SpecialEffect => {
                let effect = arguments.first().ok_or("Missing visual effect")?.number_value()?;
                let mut packet = 0x01f3_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes()); packet.extend_from_slice(&effect.to_le_bytes());
                let target = arguments.get(1).map(Value::number_value).transpose()?.unwrap_or(ScriptSendTarget::Area as i32);
                if target == ScriptSendTarget::SelfOnly as i32 { return self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet))).map_err(|error| error.to_string()); }
                if target == ScriptSendTarget::All as i32 || target == ScriptSendTarget::Map as i32 {
                    let flags = if target == ScriptSendTarget::Map as i32 { BroadcastFlag::Map.as_flag() } else { 0 };
                    server.add_to_next_tick(GameEvent::ScriptBroadcast(ScriptBroadcast { char_id: character.char_id, flags, packet }));
                    return Ok(());
                }
                return self.client_notification_sender.send(Notification::Area(AreaNotification { map_name: character.current_map_name().clone(), map_instance_id: character.current_map_instance(),
                    range_type: AreaNotificationRangeType::Fov { x: character.x(), y: character.y(), exclude_id: (target == ScriptSendTarget::AreaWithoutSelf as i32).then_some(character.char_id) }, packet })).map_err(|error| error.to_string());
            }
            Function::SkillEffect => {
                let skill = u16::try_from(arguments.first().ok_or("Missing effect skill")?.number_value()?).map_err(|_| "Invalid effect skill")?;
                let level = u16::try_from(arguments.get(1).ok_or("Missing effect level")?.number_value()?).map_err(|_| "Invalid effect level")?;
                let mut packet = 0x011a_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&skill.to_le_bytes()); packet.extend_from_slice(&level.to_le_bytes());
                packet.extend_from_slice(&character.char_id.to_le_bytes()); packet.extend_from_slice(&character.char_id.to_le_bytes()); packet.push(1);
                packet
            }
            Function::SetFont => {
                let mut packet = 0x02ef_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&character.char_id.to_le_bytes()); packet.extend_from_slice(&(arguments[0].number_value()? as u16).to_le_bytes());
                packet
            }
            Function::Announce => {
                let packet = announcement_packet(arguments)?;
                server.add_to_next_tick(GameEvent::ScriptBroadcast(ScriptBroadcast { char_id: character.char_id, flags: arguments[1].number_value()? as u32, packet }));
                return Ok(());
            }
            _ => return Err("Invalid visual operation".into()),
        };
        self.client_notification_sender.send(Notification::Area(AreaNotification { map_name: character.current_map_name().clone(), map_instance_id: character.current_map_instance(),
            range_type: AreaNotificationRangeType::Fov { x: character.x(), y: character.y(), exclude_id: None }, packet })).map_err(|error| error.to_string())
    }

    pub(crate) fn warp_party(&self, server: &Server, character: &Character, arguments: &[Value]) -> Result<(), String> {
        let event = super::party_warp_service::party_warp_request(character, arguments)?;
        server.add_to_next_tick(GameEvent::ScriptPartyWarp(event));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_colored_and_blue_announcements_with_correct_lengths() {
        let packet = announcement_packet(&["Hello".into(), 0.into(), "FF0000".into()]).unwrap();
        assert_eq!(&packet[..2], &0x01c3_u16.to_le_bytes());
        assert_eq!(usize::from(u16::from_le_bytes([packet[2], packet[3]])), packet.len());
        assert_eq!(&packet[4..8], &16_711_680_u32.to_le_bytes());
        assert_eq!(&packet[16..], b"Hello\0");
        let blue = announcement_packet(&["Hello".into(), (BroadcastFlag::Blue.as_flag() as i32).into()]).unwrap();
        assert_eq!(&blue[4..], b"blueHello\0");
    }
}
