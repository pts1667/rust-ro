//! Chat, whisper, chat room, friend and broadcast packets, decoded from the raw frame.

use super::framing::FrameLength;
use crate::server::Server;
use crate::server::model::events::game_event::{CharacterClientCommand, CharacterSocial, ClientCommand, GameEvent, MailAction, SocialAction};
use crate::server::model::request::Request;
use crate::server::service::social_packets::name_from_field;

const CZ_WHISPER: u16 = 0x0096;
const CZ_BROADCAST: u16 = 0x0099;
const CZ_SETTING_WHISPER_PC: u16 = 0x00CF;
const CZ_SETTING_WHISPER_STATE: u16 = 0x00D0;
const CZ_REQ_WHISPER_LIST: u16 = 0x00D3;
const CZ_CREATE_CHATROOM: u16 = 0x00D5;
const CZ_REQ_ENTER_ROOM: u16 = 0x00D9;
const CZ_CHANGE_CHATROOM: u16 = 0x00DE;
const CZ_REQ_ROLE_CHANGE: u16 = 0x00E0;
const CZ_REQ_EXPEL_MEMBER: u16 = 0x00E2;
const CZ_EXIT_ROOM: u16 = 0x00E3;
const CZ_LOCALBROADCAST: u16 = 0x019C;
const CZ_ADD_FRIENDS: u16 = 0x0202;
const CZ_DELETE_FRIENDS: u16 = 0x0203;
const CZ_ACK_REQ_ADD_FRIENDS: u16 = 0x0208;
const CZ_MAIL_GET_LIST: u16 = 0x023F;
const CZ_MAIL_OPEN: u16 = 0x0241;
const CZ_MAIL_DELETE: u16 = 0x0243;
const CZ_MAIL_GET_ITEM: u16 = 0x0244;
const CZ_MAIL_RESET_ITEM: u16 = 0x0246;
const CZ_MAIL_ADD_ITEM: u16 = 0x0247;
const CZ_MAIL_SEND: u16 = 0x0248;
const CZ_REQ_MAIL_RETURN: u16 = 0x0273;

const NAME_SIZE: usize = 24;
const PASSWORD_SIZE: usize = 8;
const ROOM_HEADER_SIZE: usize = 15;
const MAIL_TITLE_SIZE: usize = 40;
/// Length, recipient, title and body length of `CZ_MAIL_SEND`.
const MAIL_SEND_HEADER_SIZE: usize = 2 + 2 + NAME_SIZE + MAIL_TITLE_SIZE + 1;

/// Frame lengths of the packets of this module.
pub fn frame_length(id: u16) -> Option<FrameLength> {
    Some(match id {
        CZ_WHISPER | CZ_BROADCAST | CZ_LOCALBROADCAST => FrameLength::Variable { minimum: 4 },
        CZ_MAIL_SEND => FrameLength::Variable { minimum: MAIL_SEND_HEADER_SIZE },
        CZ_MAIL_GET_LIST => FrameLength::Fixed(2),
        CZ_MAIL_OPEN | CZ_MAIL_DELETE | CZ_MAIL_GET_ITEM => FrameLength::Fixed(6),
        CZ_MAIL_RESET_ITEM => FrameLength::Fixed(4),
        CZ_MAIL_ADD_ITEM => FrameLength::Fixed(8),
        CZ_REQ_MAIL_RETURN => FrameLength::Fixed(2 + 4 + NAME_SIZE),
        CZ_CREATE_CHATROOM | CZ_CHANGE_CHATROOM => FrameLength::Variable { minimum: ROOM_HEADER_SIZE },
        CZ_SETTING_WHISPER_PC => FrameLength::Fixed(2 + NAME_SIZE + 1),
        CZ_SETTING_WHISPER_STATE => FrameLength::Fixed(3),
        CZ_REQ_WHISPER_LIST | CZ_EXIT_ROOM => FrameLength::Fixed(2),
        CZ_REQ_ENTER_ROOM => FrameLength::Fixed(2 + 4 + PASSWORD_SIZE),
        CZ_REQ_ROLE_CHANGE => FrameLength::Fixed(2 + 4 + NAME_SIZE),
        CZ_REQ_EXPEL_MEMBER | CZ_ADD_FRIENDS => FrameLength::Fixed(2 + NAME_SIZE),
        CZ_DELETE_FRIENDS => FrameLength::Fixed(10),
        CZ_ACK_REQ_ADD_FRIENDS => FrameLength::Fixed(14),
        _ => return None,
    })
}

enum Decoded {
    Social(SocialAction),
    AtCommand(String),
    Malformed,
}

fn text(bytes: &[u8]) -> String {
    name_from_field(bytes)
}

fn u32_at(raw: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([raw[offset], raw[offset + 1], raw[offset + 2], raw[offset + 3]])
}

fn room_settings(raw: &[u8]) -> Option<(String, String, u16, bool)> {
    if raw.len() < ROOM_HEADER_SIZE {
        return None;
    }
    let limit = u16::from_le_bytes([raw[4], raw[5]]);
    let public = raw[6] != 0;
    let password = text(&raw[7..7 + PASSWORD_SIZE]);
    Some((text(&raw[ROOM_HEADER_SIZE..]), password, limit, public))
}

fn decode(id: u16, raw: &[u8]) -> Option<Decoded> {
    let social = |action| Some(Decoded::Social(action));
    let mail = |action| Some(Decoded::Social(SocialAction::Mail(action)));
    match id {
        CZ_WHISPER if raw.len() > 4 + NAME_SIZE => social(SocialAction::Whisper {
            target: text(&raw[4..4 + NAME_SIZE]),
            message: raw[4 + NAME_SIZE..].to_vec(),
        }),
        CZ_BROADCAST => Some(Decoded::AtCommand(format!("@kami {}", text(&raw[4..])))),
        CZ_LOCALBROADCAST => Some(Decoded::AtCommand(format!("@lkami {}", text(&raw[4..])))),
        CZ_SETTING_WHISPER_PC => social(SocialAction::IgnoreName {
            name: text(&raw[2..2 + NAME_SIZE]),
            block: raw[2 + NAME_SIZE] == 0,
        }),
        CZ_SETTING_WHISPER_STATE => social(SocialAction::IgnoreAll { block: raw[2] == 0 }),
        CZ_REQ_WHISPER_LIST => social(SocialAction::IgnoreList),
        CZ_CREATE_CHATROOM => Some(match room_settings(raw) {
            Some((title, password, limit, public)) => Decoded::Social(SocialAction::CreateRoom { title, password, limit, public }),
            None => Decoded::Malformed,
        }),
        CZ_CHANGE_CHATROOM => Some(match room_settings(raw) {
            Some((title, password, limit, public)) => Decoded::Social(SocialAction::ChangeRoom { title, password, limit, public }),
            None => Decoded::Malformed,
        }),
        CZ_REQ_ENTER_ROOM => social(SocialAction::EnterRoom {
            room_id: u32_at(raw, 2),
            password: text(&raw[6..6 + PASSWORD_SIZE]),
        }),
        CZ_REQ_ROLE_CHANGE => social(SocialAction::RoomOwner { name: text(&raw[6..6 + NAME_SIZE]) }),
        CZ_REQ_EXPEL_MEMBER => social(SocialAction::KickFromRoom { name: text(&raw[2..2 + NAME_SIZE]) }),
        CZ_EXIT_ROOM => social(SocialAction::LeaveRoom),
        CZ_ADD_FRIENDS => social(SocialAction::FriendRequest { name: text(&raw[2..2 + NAME_SIZE]) }),
        CZ_DELETE_FRIENDS => social(SocialAction::FriendRemove {
            account_id: u32_at(raw, 2),
            char_id: u32_at(raw, 6),
        }),
        CZ_ACK_REQ_ADD_FRIENDS => social(SocialAction::FriendReply {
            inviter_account_id: u32_at(raw, 2),
            accepted: u32_at(raw, 10) != 0,
        }),
        CZ_MAIL_GET_LIST => mail(MailAction::List),
        CZ_MAIL_OPEN => mail(MailAction::Open(u32_at(raw, 2) as i32)),
        CZ_MAIL_DELETE => mail(MailAction::Delete(u32_at(raw, 2) as i32)),
        CZ_MAIL_GET_ITEM => mail(MailAction::TakeAttachment(u32_at(raw, 2) as i32)),
        CZ_MAIL_RESET_ITEM => mail(MailAction::ResetDraft(u16::from_le_bytes([raw[2], raw[3]]))),
        CZ_MAIL_ADD_ITEM => mail(MailAction::Attach {
            index: u16::from_le_bytes([raw[2], raw[3]]),
            amount: u32_at(raw, 4),
        }),
        CZ_REQ_MAIL_RETURN => mail(MailAction::Return(u32_at(raw, 2) as i32)),
        CZ_MAIL_SEND if raw.len() >= MAIL_SEND_HEADER_SIZE => {
            let body_start = MAIL_SEND_HEADER_SIZE;
            let declared = usize::from(raw[body_start - 1]);
            let body = &raw[body_start..raw.len().min(body_start + declared)];
            mail(MailAction::Send {
                recipient: text(&raw[4..4 + NAME_SIZE]),
                title: text(&raw[4 + NAME_SIZE..4 + NAME_SIZE + MAIL_TITLE_SIZE]),
                body: text(body),
            })
        }
        _ => None,
    }
}

/// Routes the packets of this module; returns false for any other packet.
pub fn handle_raw(server: &Server, context: &Request) -> bool {
    let raw = context.packet().raw();
    if raw.len() < 2 {
        return false;
    }
    let Some(decoded) = decode(u16::from_le_bytes([raw[0], raw[1]]), raw) else {
        return false;
    };
    let Some(char_id) = super::connection_char_id(server, context) else {
        return true;
    };
    match decoded {
        Decoded::Social(action) => server.add_to_next_tick(GameEvent::CharacterSocial(CharacterSocial { char_id, action })),
        Decoded::AtCommand(command) => server.add_to_next_tick(GameEvent::CharacterClientCommand(CharacterClientCommand {
            char_id,
            command: ClientCommand::AtCommand(command),
        })),
        Decoded::Malformed => warn!("Rejected a malformed chat room packet from character {char_id}"),
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::request_handler::framing::ClientFrames;

    #[test]
    fn social_packets_are_framed_and_decoded() {
        let mut whisper = vec![0x96, 0x00, 0, 0];
        whisper.extend_from_slice(&crate::server::service::social_packets::name_field("Friend"));
        whisper.extend_from_slice(b"hello\0");
        let length = whisper.len() as u16;
        whisper[2..4].copy_from_slice(&length.to_le_bytes());
        let mut frames = ClientFrames::new(20120307);
        assert_eq!(frames.push(&whisper).unwrap(), vec![whisper.clone()]);
        match decode(CZ_WHISPER, &whisper) {
            Some(Decoded::Social(SocialAction::Whisper { target, message })) => {
                assert_eq!((target.as_str(), message.as_slice()), ("Friend", b"hello\0".as_slice()));
            }
            _ => panic!("whisper was not decoded"),
        }
        for id in [CZ_EXIT_ROOM, CZ_REQ_WHISPER_LIST, CZ_ADD_FRIENDS, CZ_DELETE_FRIENDS, CZ_ACK_REQ_ADD_FRIENDS, CZ_REQ_ENTER_ROOM] {
            let FrameLength::Fixed(length) = frame_length(id).unwrap() else { panic!() };
            let mut packet = id.to_le_bytes().to_vec();
            packet.resize(length, 0);
            assert_eq!(ClientFrames::new(20120307).push(&packet).unwrap(), vec![packet.clone()], "{id:#06x}");
            assert!(decode(id, &packet).is_some());
        }
    }
}
