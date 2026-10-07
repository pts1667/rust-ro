//! What an NPC conversation shows, rebuilt from the packets the server addresses to a bot.
use serde::Serialize;

const SAY_DIALOG: u16 = 0x00b4;
const WAIT_DIALOG: u16 = 0x00b5;
const CLOSE_DIALOG: u16 = 0x00b6;
const MENU_LIST: u16 = 0x00b7;
const SELECT_DEALTYPE: u16 = 0x00c4;
const OPEN_EDITDLG: u16 = 0x0142;
const OPEN_EDITDLG_STRING: u16 = 0x01d4;
const NOTIFY_PLAYER_CHAT: u16 = 0x008e;

// Fixed packets are `id, npc id`; variable ones are `id, length, npc id, text`
const FIXED_NPC_ID: std::ops::Range<usize> = 2..6;
const VARIABLE_NPC_ID: std::ops::Range<usize> = 4..8;
const VARIABLE_TEXT_START: usize = 8;
const CHAT_TEXT_START: usize = 4;

/// What the conversation waits for.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Prompt {
    /// The script is still talking or thinking, no answer is expected yet.
    Running,
    Next,
    Menu { options: Vec<String> },
    Number,
    Text,
    /// Buy or sell choice of a shop; shops are not drivable through the API, close the dialogue instead.
    Shop,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Dialog {
    pub npc_id: u32,
    /// Text of the page being shown, like the dialogue window of the client.
    pub lines: Vec<String>,
    pub prompt: Prompt,
}

#[derive(Debug, PartialEq)]
pub enum Decoded {
    Unrelated,
    /// The dialogue changed, it is `None` once the conversation closed.
    Dialog,
    Message(String),
}

fn text_from(packet: &[u8], start: usize) -> String {
    let text = packet.get(start..).unwrap_or_default();
    String::from_utf8_lossy(text.split(|byte| *byte == 0).next().unwrap_or_default()).into_owned()
}

fn npc_at(packet: &[u8], range: std::ops::Range<usize>) -> Option<u32> {
    Some(u32::from_le_bytes(packet.get(range)?.try_into().ok()?))
}

fn conversation<'a>(dialog: &'a mut Option<Dialog>, npc_id: u32) -> &'a mut Dialog {
    if dialog.as_ref().is_none_or(|current| current.npc_id != npc_id) {
        *dialog = Some(Dialog { npc_id, lines: Vec::new(), prompt: Prompt::Running });
    }
    dialog.as_mut().expect("conversation was just created")
}

/// Applies one server packet to the dialogue of a bot.
pub fn decode(dialog: &mut Option<Dialog>, packet: &[u8]) -> Decoded {
    let Some(id) = packet.get(..2).map(|id| u16::from_le_bytes([id[0], id[1]])) else {
        return Decoded::Unrelated;
    };
    match id {
        SAY_DIALOG => {
            let Some(npc_id) = npc_at(packet, VARIABLE_NPC_ID) else { return Decoded::Unrelated };
            let current = conversation(dialog, npc_id);
            // The client clears the window once the player answered the previous page
            if current.prompt != Prompt::Running {
                current.lines.clear();
                current.prompt = Prompt::Running;
            }
            current.lines.push(text_from(packet, VARIABLE_TEXT_START));
            Decoded::Dialog
        }
        WAIT_DIALOG | OPEN_EDITDLG | OPEN_EDITDLG_STRING | SELECT_DEALTYPE => {
            let Some(npc_id) = npc_at(packet, FIXED_NPC_ID) else { return Decoded::Unrelated };
            conversation(dialog, npc_id).prompt = match id {
                WAIT_DIALOG => Prompt::Next,
                OPEN_EDITDLG => Prompt::Number,
                OPEN_EDITDLG_STRING => Prompt::Text,
                _ => Prompt::Shop,
            };
            Decoded::Dialog
        }
        MENU_LIST => {
            let Some(npc_id) = npc_at(packet, VARIABLE_NPC_ID) else { return Decoded::Unrelated };
            let options = text_from(packet, VARIABLE_TEXT_START)
                .split(':')
                .filter(|option| !option.is_empty())
                .map(str::to_string)
                .collect();
            conversation(dialog, npc_id).prompt = Prompt::Menu { options };
            Decoded::Dialog
        }
        CLOSE_DIALOG => {
            *dialog = None;
            Decoded::Dialog
        }
        NOTIFY_PLAYER_CHAT => Decoded::Message(text_from(packet, CHAT_TEXT_START)),
        _ => Decoded::Unrelated,
    }
}

#[cfg(test)]
mod tests {
    use packets::packets::{Packet, PacketZcCloseDialog, PacketZcMenuList, PacketZcSayDialog, PacketZcWaitDialog};

    use super::*;

    const PACKETVER: u32 = 20120307;

    fn say(npc: u32, text: &str) -> Vec<u8> {
        let mut packet = PacketZcSayDialog::new(PACKETVER);
        packet.msg = format!("{text}\0");
        packet.naid = npc;
        packet.packet_length = (PacketZcSayDialog::base_len(PACKETVER) + packet.msg.len()) as i16;
        packet.fill_raw();
        packet.raw
    }

    #[test]
    fn a_conversation_is_rebuilt_from_the_packets_the_scripts_send() {
        let mut menu = PacketZcMenuList::new(PACKETVER);
        menu.naid = 7;
        menu.msg = "Yes:No:\0".to_string();
        menu.packet_length = (PacketZcMenuList::base_len(PACKETVER) + menu.msg.len()) as i16;
        menu.fill_raw();
        let mut next = PacketZcWaitDialog::new(PACKETVER);
        next.naid = 7;
        next.fill_raw();
        let mut close = PacketZcCloseDialog::new(PACKETVER);
        close.naid = 7;
        close.fill_raw();

        let mut dialog = None;
        assert_eq!(decode(&mut dialog, &say(7, "Hello")), Decoded::Dialog);
        assert_eq!(decode(&mut dialog, &next.raw), Decoded::Dialog);
        assert_eq!(dialog, Some(Dialog { npc_id: 7, lines: vec!["Hello".into()], prompt: Prompt::Next }));

        decode(&mut dialog, &say(7, "Choose"));
        decode(&mut dialog, &menu.raw);
        assert_eq!(
            dialog,
            Some(Dialog { npc_id: 7, lines: vec!["Choose".into()], prompt: Prompt::Menu { options: vec!["Yes".into(), "No".into()] } })
        );

        assert_eq!(decode(&mut dialog, &close.raw), Decoded::Dialog);
        assert_eq!(dialog, None);
    }
}
