//! Wire encoders for the chat, whisper, chat room and friend packets (PACKETVER 20091104 to 20130000).

use crate::repository::mail_repository::{MAIL_TITLE_LENGTH, MailMessage};
use crate::server::model::chat_room::ChatRoom;

pub const WHISPER_SUCCESS: u8 = 0;
pub const WHISPER_TARGET_OFFLINE: u8 = 1;
pub const WHISPER_IGNORED: u8 = 2;
pub const WHISPER_ALL_IGNORED: u8 = 3;

pub const NAME_SIZE: usize = 24;

pub fn name_field(name: &str) -> [u8; NAME_SIZE] {
    let mut field = [0u8; NAME_SIZE];
    let bytes = name.as_bytes();
    let length = bytes.len().min(NAME_SIZE - 1);
    field[..length].copy_from_slice(&bytes[..length]);
    field
}

/// Reads a NUL padded name field.
pub fn name_from_field(field: &[u8]) -> String {
    let end = field.iter().position(|byte| *byte == 0).unwrap_or(field.len());
    String::from_utf8_lossy(&field[..end]).into_owned()
}

fn variable(id: u16, body: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(4 + body.len());
    packet.extend_from_slice(&id.to_le_bytes());
    packet.extend_from_slice(&((4 + body.len()) as u16).to_le_bytes());
    packet.extend_from_slice(body);
    packet
}

fn fixed(id: u16, body: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(2 + body.len());
    packet.extend_from_slice(&id.to_le_bytes());
    packet.extend_from_slice(body);
    packet
}

fn terminated(text: &[u8]) -> Vec<u8> {
    let mut bytes = text.to_vec();
    bytes.push(0);
    bytes
}

/// `ZC_WHISPER`
pub fn whisper(sender: &str, is_admin: bool, message: &[u8]) -> Vec<u8> {
    let mut body = name_field(sender).to_vec();
    body.extend_from_slice(&u32::from(is_admin).to_le_bytes());
    body.extend_from_slice(&terminated(message));
    variable(0x0097, &body)
}

/// `ZC_ACK_WHISPER`
pub fn whisper_result(result: u8) -> Vec<u8> {
    fixed(0x0098, &[result])
}

/// `ZC_NOTIFY_CHAT`: public chat heard by others.
pub fn notify_chat(speaker_id: u32, text: &[u8]) -> Vec<u8> {
    let mut body = speaker_id.to_le_bytes().to_vec();
    body.extend_from_slice(&terminated(text));
    variable(0x008D, &body)
}

/// `ZC_NOTIFY_PLAYERCHAT`: public chat echoed back to the speaker.
pub fn notify_player_chat(text: &[u8]) -> Vec<u8> {
    variable(0x008E, &terminated(text))
}

/// `ZC_BROADCAST`
pub fn broadcast(text: &str) -> Vec<u8> {
    variable(0x009A, &terminated(text.as_bytes()))
}

/// `ZC_BROADCAST2`: the font values of `@kamic`.
pub fn broadcast_colored(color: u32, text: &str) -> Vec<u8> {
    let mut body = color.to_le_bytes().to_vec();
    for value in [0x190_i16, 12, 0, 0] {
        body.extend_from_slice(&value.to_le_bytes());
    }
    body.extend_from_slice(&terminated(text.as_bytes()));
    variable(0x01C3, &body)
}

/// `ZC_NPC_CHAT`: a channel line; the client wants BGR.
pub fn channel_message(color_rgb: u32, text: &str) -> Vec<u8> {
    let color = ((color_rgb & 0xff) << 16) | (color_rgb & 0xff00) | (color_rgb >> 16);
    let mut body = 0_u32.to_le_bytes().to_vec();
    body.extend_from_slice(&color.to_le_bytes());
    body.extend_from_slice(&terminated(text.as_bytes()));
    variable(0x02C1, &body)
}

/// `ZC_SETTING_WHISPER_PC`
pub fn whisper_pc_setting(kind: u8, result: u8) -> Vec<u8> {
    fixed(0x00D1, &[kind, result])
}

/// `ZC_SETTING_WHISPER_STATE`
pub fn whisper_state_setting(kind: u8, failure: bool) -> Vec<u8> {
    fixed(0x00D2, &[kind, u8::from(failure)])
}

/// `ZC_WHISPER_LIST`
pub fn whisper_list(names: &[String]) -> Vec<u8> {
    let body: Vec<u8> = names.iter().flat_map(|name| name_field(name)).collect();
    variable(0x00D4, &body)
}

/// `ZC_ACK_CREATE_CHATROOM`
pub fn create_chat_room_result(flag: u8) -> Vec<u8> {
    fixed(0x00D6, &[flag])
}

fn room_summary(room: &ChatRoom) -> Vec<u8> {
    let mut body = room.owner().to_le_bytes().to_vec();
    body.extend_from_slice(&room.id.to_le_bytes());
    body.extend_from_slice(&room.limit.to_le_bytes());
    body.extend_from_slice(&(room.members.len() as u16).to_le_bytes());
    body.push(u8::from(room.public));
    body.extend_from_slice(room.title.as_bytes());
    body
}

/// `ZC_ROOM_NEWENTRY`: the title shown above the owner.
pub fn room_entry(room: &ChatRoom) -> Vec<u8> {
    variable(0x00D7, &room_summary(room))
}

/// `ZC_CHANGE_CHATROOM`
pub fn room_changed(room: &ChatRoom) -> Vec<u8> {
    variable(0x00DF, &room_summary(room))
}

/// `ZC_DESTROY_ROOM`
pub fn room_destroyed(room_id: u32) -> Vec<u8> {
    fixed(0x00D8, &room_id.to_le_bytes())
}

/// `ZC_REFUSE_ENTER_ROOM`: 0 full, 1 wrong password, 2 kicked.
pub fn room_refused(result: u8) -> Vec<u8> {
    fixed(0x00DA, &[result])
}

/// `ZC_ENTER_ROOM`: member names in room order, the first one is the owner.
pub fn room_entered(room_id: u32, member_names: &[String]) -> Vec<u8> {
    let mut body = room_id.to_le_bytes().to_vec();
    for (index, name) in member_names.iter().enumerate() {
        body.extend_from_slice(&u32::from(index > 0).to_le_bytes());
        body.extend_from_slice(&name_field(name));
    }
    variable(0x00DB, &body)
}

/// `ZC_MEMBER_NEWENTRY`
pub fn room_member_joined(users: usize, name: &str) -> Vec<u8> {
    let mut body = (users as u16).to_le_bytes().to_vec();
    body.extend_from_slice(&name_field(name));
    fixed(0x00DC, &body)
}

/// `ZC_MEMBER_EXIT`
pub fn room_member_left(users_after: usize, name: &str, kicked: bool) -> Vec<u8> {
    let mut body = (users_after as u16).to_le_bytes().to_vec();
    body.extend_from_slice(&name_field(name));
    body.push(u8::from(kicked));
    fixed(0x00DD, &body)
}

/// `ZC_ROLE_CHANGE`: 0 owner, 1 normal.
pub fn room_role(is_owner: bool, name: &str) -> Vec<u8> {
    let mut body = u32::from(!is_owner).to_le_bytes().to_vec();
    body.extend_from_slice(&name_field(name));
    fixed(0x00E1, &body)
}

/// `ZC_FRIENDS_LIST`
pub fn friends_list(friends: &[(u32, u32, String)]) -> Vec<u8> {
    let mut body = Vec::with_capacity(friends.len() * 32);
    for (account_id, char_id, name) in friends {
        body.extend_from_slice(&account_id.to_le_bytes());
        body.extend_from_slice(&char_id.to_le_bytes());
        body.extend_from_slice(&name_field(name));
    }
    variable(0x0201, &body)
}

/// `ZC_FRIENDS_STATE`
pub fn friend_state(account_id: u32, char_id: u32, online: bool) -> Vec<u8> {
    let mut body = account_id.to_le_bytes().to_vec();
    body.extend_from_slice(&char_id.to_le_bytes());
    body.push(u8::from(!online));
    fixed(0x0206, &body)
}

/// `ZC_REQ_ADD_FRIENDS`
pub fn friend_request(account_id: u32, char_id: u32, name: &str) -> Vec<u8> {
    let mut body = account_id.to_le_bytes().to_vec();
    body.extend_from_slice(&char_id.to_le_bytes());
    body.extend_from_slice(&name_field(name));
    fixed(0x0207, &body)
}

/// `ZC_ADD_FRIENDS_LIST`: 0 added, 1 refused, 2 own list full, 3 other list full.
pub fn friend_request_result(result: u16, account_id: u32, char_id: u32, name: &str) -> Vec<u8> {
    let mut body = result.to_le_bytes().to_vec();
    body.extend_from_slice(&account_id.to_le_bytes());
    body.extend_from_slice(&char_id.to_le_bytes());
    body.extend_from_slice(&name_field(name));
    fixed(0x0209, &body)
}

/// `ZC_DELETE_FRIENDS`
pub fn friend_removed(account_id: u32, char_id: u32) -> Vec<u8> {
    let mut body = account_id.to_le_bytes().to_vec();
    body.extend_from_slice(&char_id.to_le_bytes());
    fixed(0x020A, &body)
}

fn text_field(text: &str, size: usize) -> Vec<u8> {
    let mut field = vec![0u8; size];
    let length = text.len().min(size);
    field[..length].copy_from_slice(&text.as_bytes()[..length]);
    field
}

/// `ZC_MAIL_REQ_GET_LIST`
pub fn mail_list(mails: &[MailMessage]) -> Vec<u8> {
    let mut body = (mails.len() as u32).to_le_bytes().to_vec();
    for mail in mails {
        body.extend_from_slice(&(mail.id as u32).to_le_bytes());
        body.extend_from_slice(&text_field(&mail.title, MAIL_TITLE_LENGTH));
        body.push(u8::from(mail.read));
        body.extend_from_slice(&text_field(&mail.sender_name, NAME_SIZE));
        body.extend_from_slice(&(mail.timestamp as u32).to_le_bytes());
    }
    variable(0x0240, &body)
}

/// `ZC_MAIL_REQ_OPEN`: `item_type` is the client item type of the attachment.
pub fn mail_opened(mail: &MailMessage, item_type: u16) -> Vec<u8> {
    let body_text = if mail.body.is_empty() { "(no message)" } else { mail.body.as_str() };
    let mut body = (mail.id as u32).to_le_bytes().to_vec();
    body.extend_from_slice(&text_field(&mail.title, MAIL_TITLE_LENGTH));
    body.extend_from_slice(&text_field(&mail.sender_name, NAME_SIZE));
    body.extend_from_slice(&0u32.to_le_bytes());
    body.extend_from_slice(&mail.zeny.to_le_bytes());
    match &mail.item {
        Some(item) => {
            body.extend_from_slice(&i32::from(item.amount).to_le_bytes());
            body.extend_from_slice(&(item.item_id as u16).to_le_bytes());
            body.extend_from_slice(&item_type.to_le_bytes());
            body.extend_from_slice(&[u8::from(item.identified), u8::from(item.damaged), item.refine as u8]);
            for card in item.cards {
                body.extend_from_slice(&card.to_le_bytes());
            }
        }
        None => body.extend_from_slice(&[0u8; 19]),
    }
    body.push(body_text.len() as u8);
    body.extend_from_slice(&terminated(body_text.as_bytes()));
    variable(0x0242, &body)
}

/// `ZC_MAIL_REQ_GET_ITEM`: 0 success, 1 failure, 2 too many items.
pub fn mail_attachment_result(result: u8) -> Vec<u8> {
    fixed(0x0245, &[result])
}

/// `ZC_ACK_MAIL_ADD_ITEM`: 0 success, 1 failure.
pub fn mail_draft_result(index: u16, result: u8) -> Vec<u8> {
    let mut body = index.to_le_bytes().to_vec();
    body.push(result);
    fixed(0x0255, &body)
}

/// `ZC_MAIL_REQ_SEND`: 0 success, 1 failure.
pub fn mail_send_result(failed: bool) -> Vec<u8> {
    fixed(0x0249, &[u8::from(failed)])
}

/// `ZC_ACK_MAIL_DELETE`
pub fn mail_delete_result(mail_id: i32, failed: bool) -> Vec<u8> {
    let mut body = (mail_id as u32).to_le_bytes().to_vec();
    body.extend_from_slice(&u16::from(failed).to_le_bytes());
    fixed(0x0257, &body)
}

/// `ZC_ACK_MAIL_RETURN`
pub fn mail_return_result(mail_id: i32, failed: bool) -> Vec<u8> {
    let mut body = (mail_id as u32).to_le_bytes().to_vec();
    body.extend_from_slice(&u16::from(failed).to_le_bytes());
    fixed(0x0274, &body)
}

/// `ZC_MAIL_RECEIVE`: new mail icon; the identifier is 0 when only the icon is refreshed.
pub fn mail_received(mail_id: i32, title: &str, sender: &str) -> Vec<u8> {
    let mut body = (mail_id as u32).to_le_bytes().to_vec();
    body.extend_from_slice(&text_field(title, MAIL_TITLE_LENGTH));
    body.extend_from_slice(&text_field(sender, NAME_SIZE));
    fixed(0x024A, &body)
}

/// `ZC_MAIL_WINDOWS`: opens (0) or closes (1) the mail window.
pub fn mail_window(close: bool) -> Vec<u8> {
    fixed(0x0260, &u32::from(close).to_le_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_packets_declare_their_whole_length() {
        let packet = whisper("Sender", false, b"hello");
        assert_eq!(u16::from_le_bytes([packet[2], packet[3]]) as usize, packet.len());
        assert_eq!(packet.len(), 4 + 24 + 4 + 6);
        assert_eq!(room_entered(1, &["a".into(), "b".into()]).len(), 4 + 4 + 2 * 28);
        assert_eq!(friends_list(&[(1, 2, "friend".into())]).len(), 4 + 32);
    }
}
