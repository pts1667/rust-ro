use std::sync::Arc;

use crate::server::Server;
use crate::server::model::events::game_event::{
    CharacterUseGroundSkill, CharacterUseGroundSkillText, GameEvent, ScriptIdentify, ScriptTeleportSelection, SkillMenuSelection,
};
use crate::server::model::request::Request;
use crate::server::request_handler::framing::FrameLength;
use crate::server::service::script_crafting_service::CraftSelection;
use crate::server::service::skill_menu_service::SkillMenuChoice;

pub fn frame_length(id: u16, packetver: u32) -> Option<FrameLength> {
    if let Some(layout) = super::talkie_box::layout(id, packetver) {
        return Some(FrameLength::Fixed(layout.length));
    }
    match id {
        0x011B => Some(FrameLength::Fixed(20)),
        0x011D => Some(FrameLength::Fixed(2)),
        0x018E => Some(FrameLength::Fixed(10)),
        0x025B => Some(FrameLength::Fixed(6)),
        0x0178 => Some(FrameLength::Fixed(4)),
        0x01AE => Some(FrameLength::Fixed(4)),
        0x01CE => Some(FrameLength::Fixed(6)),
        0x01FD => Some(FrameLength::Fixed(15)),
        0x0222 => Some(FrameLength::Fixed(6)),
        0x0254 => Some(FrameLength::Fixed(3)),
        0x0369 if (20111102..20120307).contains(&packetver) => Some(FrameLength::Fixed(10)),
        0x0438 if packetver >= 20120307 => Some(FrameLength::Fixed(10)),
        0x0116 if packetver < 20040705 => Some(FrameLength::Fixed(10)),
        _ => None,
    }
}

pub fn handle_raw(server: &Server, context: &Request) -> Result<bool, String> {
    let bytes = context.packet().raw();
    if bytes.len() < 2 {
        return Ok(false);
    }
    let id = u16::from_le_bytes([bytes[0], bytes[1]]);
    let Some(FrameLength::Fixed(length)) = frame_length(id, server.packetver()) else {
        return Ok(false);
    };
    if bytes.len() != length {
        return Err("Script operation packet has the wrong length".into());
    }
    let session_id = server.ensure_session_exists(&context.socket()).ok_or("No authenticated session")?;
    let session = server.sessions().find(session_id).ok_or("Session expired")?;
    let char_id = session.char_id.ok_or("No character selected")?;
    let socket = session.map_server_socket.as_ref().ok_or("No map connection")?;
    if !Arc::ptr_eq(socket, &context.socket()) {
        return Err("Script operation arrived on a different connection".into());
    }
    let read = |offset| u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
    if let Some(layout) = super::talkie_box::layout(id, server.packetver()) {
        let [level, skill, x, y, text] = layout.offsets;
        let skill_id = u32::from(read(skill));
        let skill_level = u8::try_from(read(level)).map_err(|_| "Invalid text ground skill level")?;
        if !crate::server::script::skill::ScriptSkillService::is_text_ground_skill(skill_id, skill_level) {
            return Err("Text ground skill packet requires Talkie Box or Graffiti".into());
        }
        let contents = &bytes[text..];
        let end = contents.iter().position(|byte| *byte == 0).unwrap_or(contents.len()).min(79);
        server.add_to_next_tick(GameEvent::CharacterUseGroundSkillText(CharacterUseGroundSkillText {
            skill: CharacterUseGroundSkill {
                char_id,
                skill_id,
                skill_level,
                x: read(x),
                y: read(y),
            },
            message: contents[..end].to_vec(),
            session: crate::server::model::session::SessionBinding::new(&session),
        }));
        return Ok(true);
    }
    let event = match id {
        0x011D => GameEvent::CharacterMemo(crate::server::model::character_lifecycle::CharacterMemo { session }),
        0x011B => {
            let name = &bytes[4..20];
            let end = name.iter().position(|byte| *byte == 0).unwrap_or(name.len());
            let map = std::str::from_utf8(&name[..end])
                .map_err(|_| "Invalid Teleport map name")?
                .to_owned();
            GameEvent::ScriptTeleportSelection(ScriptTeleportSelection {
                char_id,
                skill_id: u32::from(read(2)),
                map,
                session: Some(crate::server::model::session::SessionBinding::new(&session)),
            })
        }
        0x018E => GameEvent::ScriptCraft(CraftSelection {
            char_id,
            item_id: i32::from(read(2)),
            materials: [i32::from(read(4)), i32::from(read(6)), i32::from(read(8))],
            cooking: false,
        }),
        0x025B => {
            if read(2) != 1 {
                return Err("Invalid cooking window type".into());
            }
            GameEvent::ScriptCraft(CraftSelection {
                char_id,
                item_id: i32::from(read(4)),
                materials: [0; 3],
                cooking: true,
            })
        }
        0x0178 => GameEvent::ScriptIdentify(ScriptIdentify {
            char_id,
            index: usize::from(read(2).checked_sub(2).ok_or("Invalid identification inventory index")?),
        }),
        0x01AE => GameEvent::SkillMenuSelection(SkillMenuSelection { char_id, choice: SkillMenuChoice::Arrow(read(2)) }),
        0x01FD => GameEvent::SkillMenuSelection(SkillMenuSelection { char_id, choice: SkillMenuChoice::Repair(read(2)) }),
        0x0254 => GameEvent::SkillMenuSelection(SkillMenuSelection { char_id, choice: SkillMenuChoice::StarPlace(bytes[2]) }),
        0x01CE => GameEvent::SkillMenuSelection(SkillMenuSelection {
            char_id,
            choice: SkillMenuChoice::AutoSpell(u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]])),
        }),
        0x0222 => GameEvent::SkillMenuSelection(SkillMenuSelection {
            char_id,
            choice: SkillMenuChoice::WeaponRefine(u32::from_le_bytes([bytes[2], bytes[3], bytes[4], bytes[5]])),
        }),
        _ => GameEvent::CharacterUseGroundSkill(CharacterUseGroundSkill {
            char_id,
            skill_id: u32::from(read(4)),
            skill_level: u8::try_from(read(2)).map_err(|_| "Invalid ground skill level")?,
            x: read(6),
            y: read(8),
        }),
    };
    server.add_to_next_tick(event);
    Ok(true)
}
