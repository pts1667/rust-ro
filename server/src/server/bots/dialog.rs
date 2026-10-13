//! What an NPC conversation shows, rebuilt from the packets the server addresses to a bot.
use serde::Serialize;

use crate::server::service::global_config_service::GlobalConfigService;

const SAY_DIALOG: u16 = 0x00b4;
const WAIT_DIALOG: u16 = 0x00b5;
const CLOSE_DIALOG: u16 = 0x00b6;
const MENU_LIST: u16 = 0x00b7;
const SELECT_DEALTYPE: u16 = 0x00c4;
const PURCHASE_ITEMLIST: u16 = 0x00c6;
const SELL_ITEMLIST: u16 = 0x00c7;
const PURCHASE_RESULT: u16 = 0x00ca;
const OPEN_EDITDLG: u16 = 0x0142;
const OPEN_EDITDLG_STRING: u16 = 0x01d4;
const NOTIFY_PLAYER_CHAT: u16 = 0x008e;
const NOTIFY_CHAT: u16 = 0x008d;
const WHISPER: u16 = 0x0097;
const NOTIFY_CHAT_PARTY: u16 = 0x0109;
const PARTY_CREATED: u16 = 0x00FA;
const GUILD_CHAT: u16 = 0x017F;
const EMOTION: u16 = 0x00C0;
const GUILD_CREATED: u16 = 0x0167;
const GUILD_INVITE_RESULT: u16 = 0x0169;
const GUILD_INVITED: u16 = 0x016A;
const FRIEND_REQUEST: u16 = 0x0207;
const FRIEND_RESULT: u16 = 0x0209;
const ROOM_CREATED: u16 = 0x00D6;
const ROOM_REFUSED: u16 = 0x00DA;
const BOOKING_REGISTERED: u16 = 0x0803;
const BOOKING_DELETED: u16 = 0x0807;
const PARTY_INVITE_RESULT: u16 = 0x02C5;
/// Header, then the name of the invited character.
const INVITED_NAME: std::ops::Range<usize> = 2..26;
const INVITE_RESULT_CODE: std::ops::Range<usize> = 26..30;

// Fixed packets are `id, npc id`; variable ones are `id, length, npc id, text`
const FIXED_NPC_ID: std::ops::Range<usize> = 2..6;
const VARIABLE_NPC_ID: std::ops::Range<usize> = 4..8;
const VARIABLE_TEXT_START: usize = 8;
const CHAT_TEXT_START: usize = 4;
/// Header and length, then one entry per item.
const LIST_START: usize = 4;
/// Price, price after discount, item type, item id.
const BUY_ENTRY_LEN: usize = 11;
/// Inventory index, price, price after overcharge.
const SELL_ENTRY_LEN: usize = 10;
const PURCHASE_RESULT_CODE: usize = 2;
/// Header, speaker id.
const HEARD_CHAT_TEXT_START: usize = 8;
const WHISPER_SENDER: std::ops::Range<usize> = 4..28;
/// Header, sender name, administrator flag.
const WHISPER_TEXT_START: usize = 32;

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
    /// Buy or sell choice of a shop.
    Shop,
    /// What the shop sells.
    Buy { items: Vec<BuyOffer> },
    /// What the shop takes from the inventory.
    Sell { items: Vec<SellOffer> },
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct BuyOffer {
    pub item_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What the character pays, the discount of Discount or Compulsion Discount included.
    pub price: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SellOffer {
    /// The `index` of the item in the inventory of the observation.
    pub index: usize,
    /// What the character gets, the bonus of Overcharge included.
    pub price: i32,
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
    Message { channel: &'static str, text: String },
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

fn le_u32(packet: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(packet.get(at..at + 4)?.try_into().ok()?))
}

fn le_u16(packet: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(packet.get(at..at + 2)?.try_into().ok()?))
}

fn buy_offers(packet: &[u8]) -> Vec<BuyOffer> {
    packet
        .get(LIST_START..)
        .unwrap_or_default()
        .chunks_exact(BUY_ENTRY_LEN)
        .filter_map(|entry| {
            let item_id = u32::from(le_u16(entry, 9)?);
            Some(BuyOffer {
                item_id,
                name: GlobalConfigService::item_name_if_loaded(item_id as i32),
                price: le_u32(entry, 4)? as i32,
            })
        })
        .collect()
}

fn sell_offers(packet: &[u8]) -> Vec<SellOffer> {
    packet
        .get(LIST_START..)
        .unwrap_or_default()
        .chunks_exact(SELL_ENTRY_LEN)
        .filter_map(|entry| Some(SellOffer { index: usize::from(le_u16(entry, 0)?), price: le_u32(entry, 6)? as i32 }))
        .collect()
}

fn purchase_result(code: u8) -> &'static str {
    match code {
        0 => "The purchase is done",
        1 => "The purchase failed: not enough zeny",
        2 => "The purchase failed: too heavy to carry",
        3 => "The purchase failed: no room in the inventory",
        _ => "The purchase failed",
    }
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
        PURCHASE_ITEMLIST | SELL_ITEMLIST => {
            let Some(current) = dialog.as_mut() else { return Decoded::Unrelated };
            current.prompt = if id == PURCHASE_ITEMLIST {
                Prompt::Buy { items: buy_offers(packet) }
            } else {
                Prompt::Sell { items: sell_offers(packet) }
            };
            Decoded::Dialog
        }
        PURCHASE_RESULT => {
            let code = packet.get(PURCHASE_RESULT_CODE).copied().unwrap_or(u8::MAX);
            Decoded::Message { channel: "system", text: purchase_result(code).to_string() }
        }
        CLOSE_DIALOG => {
            *dialog = None;
            Decoded::Dialog
        }
        NOTIFY_PLAYER_CHAT => Decoded::Message { channel: "system", text: text_from(packet, CHAT_TEXT_START) },
        NOTIFY_CHAT => Decoded::Message { channel: "chat", text: text_from(packet, HEARD_CHAT_TEXT_START) },
        NOTIFY_CHAT_PARTY => Decoded::Message { channel: "party", text: text_from(packet, HEARD_CHAT_TEXT_START) },
        PARTY_CREATED => {
            let text = match packet.get(2) {
                Some(0) => "Party created.",
                Some(1) => "Party creation failed: the name is taken.",
                Some(2) => "Party creation failed: already in a party.",
                _ => "Party creation failed.",
            };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        PARTY_INVITE_RESULT => {
            let name = packet.get(INVITED_NAME).map(|field| text_from(field, 0)).unwrap_or_default();
            let code = packet.get(INVITE_RESULT_CODE).and_then(|field| field.try_into().ok()).map(u32::from_le_bytes);
            let text = match code {
                Some(0) => format!("{name} is already in a party or has an invitation to answer."),
                Some(1) => format!("{name} declined the party invitation."),
                Some(2) => format!("{name} joined the party."),
                Some(3) => "The party is full.".to_string(),
                Some(4) => format!("{name} is already a member of the party with another character."),
                Some(5) => format!("{name} does not take party invitations."),
                _ => "Nobody of that name is in the game.".to_string(),
            };
            Decoded::Message { channel: "system", text }
        }
        GUILD_CHAT => Decoded::Message { channel: "guild", text: text_from(packet, CHAT_TEXT_START) },
        EMOTION => {
            let (Some(actor), Some(emote)) = (le_u32(packet, 2), packet.get(6)) else { return Decoded::Unrelated };
            Decoded::Message { channel: "emote", text: format!("{actor} : /{}", super::social::emote_name(*emote)) }
        }
        GUILD_CREATED => {
            let text = match packet.get(2) {
                Some(0) => "Guild created.",
                Some(1) => "Guild creation failed: already in a guild.",
                Some(2) => "Guild creation failed: the name is taken.",
                Some(3) => "Guild creation failed: it takes an Emperium.",
                _ => "Guild creation failed.",
            };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        GUILD_INVITE_RESULT => {
            let text = match packet.get(2) {
                Some(0) => "The character is already in a guild or has an invitation to answer.",
                Some(1) => "The guild invitation was declined.",
                Some(2) => "The guild invitation was accepted.",
                Some(3) => "The guild is full.",
                _ => "The guild invitation failed.",
            };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        GUILD_INVITED => {
            let guild = packet.get(6..30).map(|field| text_from(field, 0)).unwrap_or_default();
            Decoded::Message { channel: "system", text: format!("Invited to the guild {guild}: answer with guild_accept or guild_decline.") }
        }
        FRIEND_REQUEST => {
            let name = packet.get(10..34).map(|field| text_from(field, 0)).unwrap_or_default();
            Decoded::Message { channel: "system", text: format!("{name} asks to be friends: answer with friend_accept or friend_decline.") }
        }
        FRIEND_RESULT => {
            let name = packet.get(12..36).map(|field| text_from(field, 0)).unwrap_or_default();
            let text = match le_u16(packet, 2) {
                Some(0) => format!("{name} is now a friend."),
                Some(1) => format!("{name} declined the friend request."),
                Some(2) => "The friend list is full.".to_string(),
                Some(3) => format!("The friend list of {name} is full."),
                _ => "The friend request failed.".to_string(),
            };
            Decoded::Message { channel: "system", text }
        }
        ROOM_CREATED => {
            let text = if packet.get(2) == Some(&0) { "Chat room created." } else { "The chat room could not be created." };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        ROOM_REFUSED => {
            let text = match packet.get(2) {
                Some(0) => "The chat room is full, or out of reach.",
                Some(1) => "Wrong chat room password.",
                Some(2) => "Kicked from that chat room before.",
                _ => "The chat room refused the bot.",
            };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        BOOKING_REGISTERED => {
            let text = if le_u16(packet, 2) == Some(0) { "Party booking registered." } else { "A party booking is already registered." };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        BOOKING_DELETED => {
            let text = if le_u16(packet, 2) == Some(0) { "Party booking deleted." } else { "No party booking to delete." };
            Decoded::Message { channel: "system", text: text.to_string() }
        }
        WHISPER => {
            let sender = packet.get(WHISPER_SENDER).map(|field| text_from(field, 0)).unwrap_or_default();
            Decoded::Message { channel: "whisper", text: format!("{sender} : {}", text_from(packet, WHISPER_TEXT_START)) }
        }
        _ => Decoded::Unrelated,
    }
}

#[cfg(test)]
mod tests {
    use packets::packets::{
        Packet, PacketZcCloseDialog, PacketZcMenuList, PacketZcPcPurchaseItemlist, PacketZcPcPurchaseResult, PacketZcPcSellItemlist, PacketZcSayDialog,
        PacketZcWaitDialog, PurchaseItem, SellItem,
    };

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

    fn conversation_with(npc: u32) -> Option<Dialog> {
        let mut dialog = None;
        decode(&mut dialog, &say(npc, "Welcome"));
        dialog
    }

    #[test]
    fn what_a_shop_sells_is_read_from_its_purchase_list() {
        let mut entries = vec![];
        for (item_id, price, discounted) in [(501u16, 50, 44), (502, 200, 176)] {
            let mut entry = PurchaseItem::new(PACKETVER);
            entry.set_itid(item_id);
            entry.set_price(price);
            entry.set_discountprice(discounted);
            entries.push(entry);
        }
        let mut packet = PacketZcPcPurchaseItemlist::new(PACKETVER);
        packet.set_packet_length((PacketZcPcPurchaseItemlist::base_len(PACKETVER) + entries.len() * PurchaseItem::base_len(PACKETVER)) as i16);
        packet.set_item_list(entries);
        packet.fill_raw();

        let mut dialog = conversation_with(9);
        assert_eq!(decode(&mut dialog, &packet.raw), Decoded::Dialog);
        let Some(Dialog { prompt: Prompt::Buy { items }, .. }) = dialog else { panic!("no purchase list: {dialog:?}") };
        assert_eq!(items.iter().map(|offer| (offer.item_id, offer.price)).collect::<Vec<_>>(), [(501, 44), (502, 176)]);
    }

    #[test]
    fn what_a_shop_takes_is_read_from_its_sell_list() {
        let mut entries = vec![];
        for (index, price, overcharged) in [(3i16, 10, 12), (7, 40, 48)] {
            let mut entry = SellItem::new(PACKETVER);
            entry.set_index(index);
            entry.set_price(price);
            entry.set_overchargeprice(overcharged);
            entries.push(entry);
        }
        let mut packet = PacketZcPcSellItemlist::new(PACKETVER);
        packet.set_packet_length((PacketZcPcSellItemlist::base_len(PACKETVER) + entries.len() * SellItem::base_len(PACKETVER)) as i16);
        packet.set_item_list(entries);
        packet.fill_raw();

        let mut dialog = conversation_with(9);
        assert_eq!(decode(&mut dialog, &packet.raw), Decoded::Dialog);
        let Some(Dialog { prompt: Prompt::Sell { items }, .. }) = dialog else { panic!("no sell list: {dialog:?}") };
        assert_eq!(items.iter().map(|offer| (offer.index, offer.price)).collect::<Vec<_>>(), [(3, 12), (7, 48)]);
    }

    #[test]
    fn a_shop_list_without_a_conversation_is_ignored_and_a_refused_purchase_is_reported() {
        let mut packet = PacketZcPcSellItemlist::new(PACKETVER);
        packet.set_packet_length(PacketZcPcSellItemlist::base_len(PACKETVER) as i16);
        packet.fill_raw();
        assert_eq!(decode(&mut None, &packet.raw), Decoded::Unrelated);

        let mut result = PacketZcPcPurchaseResult::new(PACKETVER);
        result.set_result(1);
        result.fill_raw();
        assert_eq!(
            decode(&mut None, &result.raw),
            Decoded::Message { channel: "system", text: "The purchase failed: not enough zeny".into() }
        );
    }
}
