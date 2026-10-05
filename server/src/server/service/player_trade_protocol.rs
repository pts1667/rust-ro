use models::enums::EnumWithNumberValue;

use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::server::model::game_systems::PlayerTradeRequest;
use crate::server::request_handler::framing::FrameLength;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TradeResponse {
    TooFar = 0,
    Unavailable = 1,
    Failed = 2,
    Accepted = 3,
    Rejected = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OfferResult {
    Success = 0,
    Overweight = 1,
    Closed = 2,
    Full = 3,
    StackFull = 4,
}

pub fn frame_length(id: u16) -> Option<FrameLength> {
    Some(FrameLength::Fixed(match id {
        0x00E4 => 6,
        0x00E6 => 3,
        0x00E8 => 8,
        0x00EB | 0x00ED | 0x00EF => 2,
        _ => return None,
    }))
}

pub fn decode_request(bytes: &[u8]) -> Result<Option<PlayerTradeRequest>, String> {
    let Some(header) = bytes.get(..2) else {
        return Ok(None);
    };
    let id = u16::from_le_bytes(header.try_into().unwrap());
    let Some(FrameLength::Fixed(length)) = frame_length(id) else {
        return Ok(None);
    };
    if bytes.len() != length {
        return Err("Invalid player-trade packet length".into());
    }
    Ok(Some(match id {
        0x00E4 => PlayerTradeRequest::Request(u32::from_le_bytes(bytes[2..6].try_into().unwrap())),
        0x00E6 => PlayerTradeRequest::Answer(match bytes[2] {
            3 => true,
            4 => false,
            _ => return Err("Invalid player-trade answer".into()),
        }),
        0x00E8 => PlayerTradeRequest::Offer {
            index: u16::from_le_bytes(bytes[2..4].try_into().unwrap()),
            amount: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        },
        0x00EB => PlayerTradeRequest::Lock,
        0x00ED => PlayerTradeRequest::Cancel,
        0x00EF => PlayerTradeRequest::Confirm,
        _ => unreachable!(),
    }))
}

pub fn request(name: &str, account_id: u32, level: u32, packetver: u32) -> Vec<u8> {
    let mut packet = header(if packetver > 6 { 0x01F4 } else { 0x00E5 });
    let mut fixed = [0; 24];
    let bytes = name.as_bytes();
    let count = bytes.len().min(23);
    fixed[..count].copy_from_slice(&bytes[..count]);
    packet.extend_from_slice(&fixed);
    if packetver > 6 {
        packet.extend_from_slice(&account_id.to_le_bytes());
        packet.extend_from_slice(&(level.min(u32::from(u16::MAX)) as u16).to_le_bytes());
    }
    packet
}

pub fn response(result: TradeResponse, account_id: u32, level: u32, packetver: u32) -> Vec<u8> {
    let mut packet = header(if packetver > 6 { 0x01F5 } else { 0x00E7 });
    packet.push(result as u8);
    if packetver > 6 {
        packet.extend_from_slice(&account_id.to_le_bytes());
        packet.extend_from_slice(&(level.min(u32::from(u16::MAX)) as u16).to_le_bytes());
    }
    packet
}

pub fn offer_ack(index: u16, result: OfferResult, packetver: u32) -> Vec<u8> {
    let mut packet = header(0x00EA);
    packet.extend_from_slice(&index.to_le_bytes());
    packet.push(if packetver < 20110705 && result as u8 > 2 {
        OfferResult::Overweight as u8
    } else {
        result as u8
    });
    packet
}

pub fn offer(item: Option<(&InventoryItemModel, &ItemModel)>, amount: u32, packetver: u32) -> Vec<u8> {
    let id = if packetver >= 20200916 {
        0x0B42
    } else if packetver >= 20161102 {
        0x0A96
    } else if packetver >= 20150226 {
        0x0A09
    } else if packetver >= 20100223 {
        0x080F
    } else {
        0x00E9
    };
    let mut packet = header(id);
    let item_id = item.map_or(0, |(record, _)| record.item_id as u32);
    if packetver >= 20181121 {
        packet.extend_from_slice(&item_id.to_le_bytes());
    } else if packetver >= 20100223 {
        packet.extend_from_slice(&(item_id as u16).to_le_bytes());
    }
    if packetver >= 20100223 {
        packet.push(item.map_or(0, |(_, model)| model.item_type.value() as u8));
    }
    packet.extend_from_slice(&amount.to_le_bytes());
    if packetver < 20100223 {
        packet.extend_from_slice(&(item_id as u16).to_le_bytes());
    }
    packet.push(item.map_or(0, |(record, _)| u8::from(record.is_identified)));
    packet.push(item.map_or(0, |(record, _)| u8::from(record.is_damaged)));
    let refine = item.map_or(0, |(record, _)| record.refine as u8);
    if packetver < 20200916 {
        packet.push(refine);
    }
    let cards = item.map_or([0; 4], |(record, _)| [record.card0, record.card1, record.card2, record.card3]);
    for card in cards {
        packet.extend_from_slice(&(card as u16).to_le_bytes());
    }
    if packetver >= 20150226 {
        packet.extend_from_slice(&[0; 25]);
    }
    if packetver >= 20161102 {
        packet.extend_from_slice(&item.map_or(0, |(_, model)| model.location as u32).to_le_bytes());
        packet.extend_from_slice(&(item.map_or(0, |(_, model)| model.view.unwrap_or(0)) as u16).to_le_bytes());
    }
    if packetver >= 20200916 {
        packet.extend_from_slice(&[refine, 0]);
    }
    packet
}

pub fn lock(other: bool) -> Vec<u8> {
    vec![0xEC, 0, u8::from(other)]
}
pub fn cancelled() -> Vec<u8> {
    header(0x00EE)
}
pub fn completed() -> Vec<u8> {
    vec![0xF0, 0, 0]
}
fn header(id: u16) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}
