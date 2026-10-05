use models::enums::{EnumWithMaskValueU8, EnumWithNumberValue, EnumWithStringValue};

use crate::server::model::game_systems::{
    BuyingOffer, BuyingStore, HomunculusInfoFlag, HomunculusRecord, ItemContainer, ScriptWorldRequest, StorageItemFlag, StoreSearchResult,
    VendingStore,
};
use crate::server::request_handler::framing::FrameLength;
use crate::server::service::global_config_service::GlobalConfigService;

fn world_packet_id(id: u16, packetver: u32) -> Option<u16> {
    if matches!(
        id,
        0x019F
            | 0x01A7
            | 0x01A1
            | 0x01A5
            | 0x01A9
            | 0x029F
            | 0x00F7
            | 0x083B
            | 0x0126
            | 0x0127
            | 0x0128
            | 0x0129
            | 0x012A
            | 0x012E
            | 0x012F
            | 0x0130
            | 0x0134
            | 0x01AF
            | 0x01B2
            | 0x0801
            | 0x0231
            | 0x0232
            | 0x0233
            | 0x0234
            | 0x0165
            | 0x0168
            | 0x016B
            | 0x014D
            | 0x014F
            | 0x0159
            | 0x015B
            | 0x015D
            | 0x0161
            | 0x017E
            | 0x0155
            | 0x0153
            | 0x0217
            | 0x0218
            | 0x0225
            | 0x097C
            | 0x0170
            | 0x0172
            | 0x0180
            | 0x0183
            | 0x0151
            | 0x016E
            | 0x00F9
            | 0x01E8
            | 0x00FC
            | 0x00FF
            | 0x02C7
            | 0x0100
            | 0x0102
            | 0x0103
            | 0x0108
            | 0x07D7
            | 0x07DA
            | 0x02C8
    ) {
        return Some(id);
    }
    if (20111102..20120307).contains(&packetver) {
        return Some(match id {
            0x0835 => 0x0811,
            0x089B => 0x0815,
            0x08A1 => 0x0817,
            0x089E => 0x0819,
            0x08AB => 0x0835,
            0x088B => 0x0838,
            0x08A2 => 0x083C,
            0x0893 => 0x00F3,
            0x0897 => 0x00F5,
            0x0898 => 0x022D,
            0x088D => 0x02C4,
            _ => return None,
        });
    }
    if (20120307..20120410).contains(&packetver) {
        return Some(match id {
            0x0815 => 0x0811,
            0x0817 => 0x0815,
            0x0360 => 0x0817,
            0x0811 => 0x0819,
            0x0884 => 0x0835,
            0x0835 => 0x0838,
            0x0838 => 0x083C,
            0x093B => 0x00F3,
            0x0963 => 0x00F5,
            0x0863 => 0x022D,
            0x0929 => 0x02C4,
            _ => return None,
        });
    }
    if (20120410..20120702).contains(&packetver) {
        if id == 0x091C {
            return Some(0x02C4);
        }
        return Some(match id {
            0x0815 => 0x0811,
            0x0817 => 0x0815,
            0x0360 => 0x0817,
            0x0811 => 0x0819,
            0x0819 => 0x0835,
            0x0835 => 0x0838,
            0x0838 => 0x083C,
            0x086C => 0x00F3,
            0x08A6 => 0x00F5,
            0x0885 => 0x022D,
            _ => return None,
        });
    }
    if packetver >= 20111005 && packetver < 20111102 {
        if id == 0x0893 {
            return Some(0x00F3);
        }
        if id == 0x0897 {
            return Some(0x00F5);
        }
    } else if packetver >= 20101124 && packetver < 20111005 {
        if id == 0x0364 {
            return Some(0x00F3);
        }
        if id == 0x0365 {
            return Some(0x00F5);
        }
    } else if matches!(id, 0x00F3 | 0x00F5) {
        return Some(id);
    }
    if id == 0x02C4 && packetver < 20111102 {
        return Some(id);
    }
    if matches!(id, 0x0811 | 0x0815 | 0x0817 | 0x0819 | 0x0835 | 0x0838 | 0x083C | 0x022D) {
        Some(id)
    } else {
        None
    }
}

pub fn world_frame_length(id: u16, packetver: u32) -> Option<FrameLength> {
    Some(match world_packet_id(id, packetver)? {
        0x00F9 | 0x02C4 => FrameLength::Fixed(26),
        0x01E8 => FrameLength::Fixed(28),
        0x00FC | 0x0102 | 0x07DA => FrameLength::Fixed(6),
        0x00FF => FrameLength::Fixed(10),
        0x02C7 => FrameLength::Fixed(7),
        0x0100 => FrameLength::Fixed(2),
        0x0103 => FrameLength::Fixed(30),
        0x07D7 => FrameLength::Fixed(8),
        0x02C8 => FrameLength::Fixed(3),
        0x0108 => FrameLength::Variable { minimum: 5 },
        0x0165 => FrameLength::Fixed(30),
        0x0168 => FrameLength::Fixed(14),
        0x0170 => FrameLength::Fixed(14),
        0x0217 | 0x0218 | 0x0225 => FrameLength::Fixed(2),
        0x097C => FrameLength::Fixed(4),
        0x0172 | 0x0183 => FrameLength::Fixed(10),
        0x0180 => FrameLength::Fixed(6),
        0x016B => FrameLength::Fixed(10),
        0x014D => FrameLength::Fixed(2),
        0x014F => FrameLength::Fixed(6),
        0x0159 | 0x015B => FrameLength::Fixed(54),
        0x015D => FrameLength::Fixed(42),
        0x0151 => FrameLength::Fixed(6),
        0x017E => FrameLength::Variable { minimum: 5 },
        0x016E => FrameLength::Fixed(186),
        0x0161 | 0x0155 | 0x0153 => FrameLength::Variable { minimum: 4 },
        0x019F | 0x01A9 | 0x0817 | 0x0130 | 0x0234 => FrameLength::Fixed(6),
        0x01A7 | 0x01AF => FrameLength::Fixed(4),
        0x01A1 | 0x029F => FrameLength::Fixed(3),
        0x01A5 | 0x0231 => FrameLength::Fixed(26),
        0x022D => FrameLength::Fixed(5),
        0x0232 => FrameLength::Fixed(9),
        0x0233 => FrameLength::Fixed(11),
        0x00F3 | 0x00F5 | 0x0126 | 0x0127 | 0x0128 | 0x0129 => FrameLength::Fixed(8),
        0x00F7 | 0x0815 | 0x0838 | 0x083B | 0x012A | 0x012E => FrameLength::Fixed(2),
        0x012F => FrameLength::Variable { minimum: 84 },
        0x01B2 => FrameLength::Variable { minimum: 85 },
        0x0134 => FrameLength::Variable { minimum: 8 },
        0x0801 => FrameLength::Variable { minimum: 12 },
        0x0811 => FrameLength::Variable { minimum: 89 },
        0x0819 => FrameLength::Variable { minimum: 12 },
        0x0835 => FrameLength::Variable { minimum: 15 },
        0x083C => FrameLength::Fixed(if packetver >= 20181121 { 14 } else { 12 }),
        _ => return None,
    })
}

pub fn client_frame_length(id: u16, packetver: u32) -> Option<FrameLength> {
    if packetver != 20120229 {
        return None;
    }
    let length = *super::world_data().client_frames.get(&id)?;
    if length == -1 {
        Some(FrameLength::Variable { minimum: 4 })
    } else if length >= 2 {
        Some(FrameLength::Fixed(length as usize))
    } else {
        None
    }
}

pub fn decode_request(bytes: &[u8], packetver: u32) -> Result<Option<ScriptWorldRequest>, String> {
    if bytes.len() < 2 {
        return Ok(None);
    }
    let Some(id) = world_packet_id(u16_at(bytes, 0)?, packetver) else {
        return Ok(None);
    };
    let item_width = if packetver >= 20181121 { 4 } else { 2 };
    let request = match id {
        0x00F9 | 0x01E8 => {
            exact_length(bytes, if id == 0x00F9 { 26 } else { 28 })?;
            let (item_pickup, item_share) = if id == 0x01E8 {
                (boolean(u32::from(bytes[26]))?, boolean(u32::from(bytes[27]))?)
            } else {
                (false, false)
            };
            ScriptWorldRequest::CreateParty {
                name: text(&bytes[2..26])?,
                item_pickup,
                item_share,
            }
        }
        0x00FC => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::InviteParty(u32_at(bytes, 2)?)
        }
        0x02C4 => {
            exact_length(bytes, 26)?;
            ScriptWorldRequest::InvitePartyByName(text(&bytes[2..26])?)
        }
        0x00FF | 0x02C7 => {
            exact_length(bytes, if id == 0x00FF { 10 } else { 7 })?;
            ScriptWorldRequest::AnswerPartyInvite {
                party_id: u32_at(bytes, 2)?,
                accept: boolean(if id == 0x00FF { u32_at(bytes, 6)? } else { u32::from(bytes[6]) })?,
            }
        }
        0x0100 => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::LeaveParty
        }
        0x0103 => {
            exact_length(bytes, 30)?;
            ScriptWorldRequest::ExpelParty {
                account_id: u32_at(bytes, 2)?,
                name: text(&bytes[6..30])?,
            }
        }
        0x0102 | 0x07D7 => {
            exact_length(bytes, if id == 0x0102 { 6 } else { 8 })?;
            ScriptWorldRequest::ChangePartyOptions {
                exp_share: boolean(u32_at(bytes, 2)?)?,
                item_rules: if id == 0x0102 {
                    None
                } else {
                    Some((boolean(u32::from(bytes[6]))?, boolean(u32::from(bytes[7]))?))
                },
            }
        }
        0x07DA => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::ChangePartyLeader(u32_at(bytes, 2)?)
        }
        0x02C8 => {
            exact_length(bytes, 3)?;
            ScriptWorldRequest::DisablePartyInvites(boolean(u32::from(bytes[2]))?)
        }
        0x0108 => {
            variable_length(bytes, 5, 1)?;
            if bytes.len() > 259 || bytes.last() != Some(&0) {
                return Err("Invalid party chat length or terminator".into());
            }
            ScriptWorldRequest::PartyMessage(text(&bytes[4..])?)
        }
        0x0165 => {
            exact_length(bytes, 30)?;
            ScriptWorldRequest::CreateGuild(text(&bytes[6..30])?)
        }
        0x0168 => {
            exact_length(bytes, 14)?;
            ScriptWorldRequest::InviteGuild(u32_at(bytes, 2)?)
        }
        0x016B => {
            exact_length(bytes, 10)?;
            let answer = u32_at(bytes, 6)?;
            if answer > 1 {
                return Err("Invalid guild invitation response".into());
            }
            ScriptWorldRequest::AnswerGuildInvite {
                guild_id: u32_at(bytes, 2)?,
                accept: answer == 1,
            }
        }
        0x0217 => ScriptWorldRequest::FameList(0),
        0x0218 => ScriptWorldRequest::FameList(1),
        0x0225 => ScriptWorldRequest::FameList(2),
        0x097C => {
            exact_length(bytes, 4)?;
            let kind = u16_at(bytes, 2)?;
            ScriptWorldRequest::FameList(u8::try_from(kind).map_err(|_| "Invalid ranking type")?)
        }
        0x0170 => {
            exact_length(bytes, 14)?;
            ScriptWorldRequest::GuildAllianceRequest(u32_at(bytes, 2)?)
        }
        0x0172 => {
            exact_length(bytes, 10)?;
            ScriptWorldRequest::GuildAllianceReply {
                inviter: u32_at(bytes, 2)?,
                accept: u32_at(bytes, 6)? == 1,
            }
        }
        0x0180 => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::GuildOpposition(u32_at(bytes, 2)?)
        }
        0x0183 => {
            exact_length(bytes, 10)?;
            ScriptWorldRequest::GuildRelationBreak {
                guild_id: u32_at(bytes, 2)?,
                hostile: u32_at(bytes, 6)? == 1,
            }
        }
        0x014D => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::GuildMenu
        }
        0x014F => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::GuildInformation(u32_at(bytes, 2)?)
        }
        0x0159 => {
            exact_length(bytes, 54)?;
            ScriptWorldRequest::LeaveGuild {
                guild_id: u32_at(bytes, 2)?,
                reason: text(&bytes[14..54])?,
            }
        }
        0x015B => {
            exact_length(bytes, 54)?;
            ScriptWorldRequest::ExpelGuild {
                guild_id: u32_at(bytes, 2)?,
                member_id: u32_at(bytes, 10)?,
                reason: text(&bytes[14..54])?,
            }
        }
        0x015D => {
            exact_length(bytes, 42)?;
            ScriptWorldRequest::DisbandGuild(text(&bytes[2..42])?)
        }
        0x016E => {
            exact_length(bytes, 186)?;
            ScriptWorldRequest::GuildNotice {
                subject: text(&bytes[6..66])?,
                body: text(&bytes[66..186])?,
            }
        }
        0x0161 => {
            variable_length(bytes, 4, 40)?;
            let mut positions = Vec::new();
            for record in bytes[4..].chunks_exact(40) {
                let mode = u32_at(record, 4)?;
                let tax = u32_at(record, 12)?;
                positions.push((
                    u32_at(record, 0)?,
                    crate::server::model::game_systems::GuildPosition {
                        name: text(&record[16..40])?,
                        invite: mode & 0x01 != 0,
                        punish: mode & 0x10 != 0,
                        exp_tax: u8::try_from(tax).map_err(|_| "Invalid guild experience tax")?,
                    },
                ));
            }
            ScriptWorldRequest::GuildPositions(positions)
        }
        0x0155 => {
            variable_length(bytes, 4, 12)?;
            let mut members = Vec::new();
            for record in bytes[4..].chunks_exact(12) {
                members.push((u32_at(record, 4)?, u32_at(record, 8)?));
            }
            ScriptWorldRequest::GuildMemberPositions(members)
        }
        0x0153 => {
            if bytes.len() < 4 || usize::from(u16_at(bytes, 2)?) != bytes.len() {
                return Err("Malformed guild emblem packet".into());
            }
            ScriptWorldRequest::GuildEmblem(bytes[4..].to_vec())
        }
        0x017E => {
            variable_length(bytes, 5, 1)?;
            if bytes.len() > 259 || bytes.last() != Some(&0) {
                return Err("Invalid guild chat length or terminator".into());
            }
            ScriptWorldRequest::GuildMessage(text(&bytes[4..])?)
        }
        0x0151 => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::GuildEmblemRequest(u32_at(bytes, 2)?)
        }
        0x022D => {
            exact_length(bytes, 5)?;
            ScriptWorldRequest::HomunculusMenu(bytes[4])
        }
        0x0231 => {
            exact_length(bytes, 26)?;
            ScriptWorldRequest::HomunculusRename(text(&bytes[2..])?)
        }
        0x0234 => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::CompanionMoveToOwner(u32_at(bytes, 2)?)
        }
        0x0232 => {
            exact_length(bytes, 9)?;
            let x = u16::from(bytes[6]) << 2 | u16::from(bytes[7] >> 6);
            let y = u16::from(bytes[7] & 63) << 4 | u16::from(bytes[8] >> 4);
            ScriptWorldRequest::CompanionMove {
                id: u32_at(bytes, 2)?,
                x,
                y,
            }
        }
        0x0233 => {
            exact_length(bytes, 11)?;
            ScriptWorldRequest::CompanionAttack {
                id: u32_at(bytes, 2)?,
                target: u32_at(bytes, 6)?,
                repeat: bytes[10] != 0,
            }
        }
        0x0126 | 0x0127 | 0x0128 | 0x0129 => {
            exact_length(bytes, 8)?;
            let (source, destination) = match id {
                0x0126 => (ItemContainer::Inventory, ItemContainer::Cart),
                0x0127 => (ItemContainer::Cart, ItemContainer::Inventory),
                0x0128 => (ItemContainer::Storage, ItemContainer::Cart),
                _ => (ItemContainer::Cart, ItemContainer::Storage),
            };
            ScriptWorldRequest::ContainerTransfer {
                source,
                destination,
                index: u16_at(bytes, 2)?
                    .checked_sub(if source == ItemContainer::Storage { 1 } else { 2 })
                    .ok_or("Invalid container index")?,
                amount: u32_at(bytes, 4)?,
            }
        }
        0x012A => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::RemoveOption
        }
        0x01AF => {
            exact_length(bytes, 4)?;
            ScriptWorldRequest::ChangeCart(u8::try_from(u16_at(bytes, 2)?).map_err(|_| "Invalid cart style")?)
        }
        0x012E => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::CloseVendingStore
        }
        0x0130 => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::OpenVendingStore(u32_at(bytes, 2)?)
        }
        0x012F | 0x01B2 => {
            let base = if id == 0x012F { 84 } else { 85 };
            variable_length(bytes, base, 8)?;
            if id == 0x01B2 && bytes[84] == 0 {
                ScriptWorldRequest::CloseVendingStore
            } else {
                let offers = bytes[base..]
                    .chunks_exact(8)
                    .map(|record| {
                        Ok((
                            u16_at(record, 0)?.checked_sub(2).ok_or("Invalid cart sale index")?,
                            u16_at(record, 2)?,
                            u32_at(record, 4)?,
                        ))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                if offers.len() > 12 {
                    return Err("Too many vending offers".into());
                }
                ScriptWorldRequest::CreateVendingStore {
                    title: text(&bytes[4..84])?,
                    offers,
                }
            }
        }
        0x0134 | 0x0801 => {
            let base = if id == 0x0134 { 8 } else { 12 };
            variable_length(bytes, base, 4)?;
            let items = bytes[base..]
                .chunks_exact(4)
                .map(|record| {
                    Ok((
                        u16_at(record, 2)?.checked_sub(2).ok_or("Invalid vending purchase index")?,
                        u16_at(record, 0)?,
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?;
            if items.len() > 12 {
                return Err("Too many vending purchases".into());
            }
            ScriptWorldRequest::PurchaseVendingStore {
                account_id: u32_at(bytes, 4)?,
                store_id: if id == 0x0801 { Some(u32_at(bytes, 8)?) } else { None },
                items,
            }
        }
        0x019F => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::CapturePet(u32_at(bytes, 2)?)
        }
        0x01A7 => {
            exact_length(bytes, 4)?;
            ScriptWorldRequest::HatchPet(u16_at(bytes, 2)?.checked_sub(2).ok_or("Invalid egg inventory index")?)
        }
        0x01A1 => {
            exact_length(bytes, 3)?;
            ScriptWorldRequest::PetMenu(bytes[2])
        }
        0x01A5 => {
            exact_length(bytes, 26)?;
            ScriptWorldRequest::PetRename(text(&bytes[2..26])?)
        }
        0x01A9 => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::PetEmotion(u32_at(bytes, 2)? as i32)
        }
        0x029F => {
            exact_length(bytes, 3)?;
            ScriptWorldRequest::DismissMercenary(bytes[2])
        }
        0x00F3 => {
            exact_length(bytes, 8)?;
            ScriptWorldRequest::StorageDeposit {
                index: u16_at(bytes, 2)?.checked_sub(2).ok_or("Invalid deposit inventory index")?,
                amount: u32_at(bytes, 4)?,
            }
        }
        0x00F5 => {
            exact_length(bytes, 8)?;
            ScriptWorldRequest::StorageWithdraw {
                index: u16_at(bytes, 2)?.checked_sub(1).ok_or("Invalid storage inventory index")?,
                amount: u32_at(bytes, 4)?,
            }
        }
        0x00F7 => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::CloseStorage
        }
        0x0811 => {
            variable_length(bytes, 89, item_width + 6)?;
            if bytes[8] == 0 {
                ScriptWorldRequest::CloseBuyingStore
            } else {
                let offers = bytes[89..]
                    .chunks_exact(item_width + 6)
                    .map(|record| {
                        Ok(BuyingOffer {
                            item_id: item_at(record, 0, item_width)?,
                            amount: u16_at(record, item_width)?,
                            price: u32_at(record, item_width + 2)?,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                if offers.len() > 5 {
                    return Err("Too many buying store offers".into());
                }
                ScriptWorldRequest::CreateBuyingStore {
                    title: text(&bytes[9..89])?,
                    zeny_limit: u32_at(bytes, 4)?,
                    offers,
                }
            }
        }
        0x0815 => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::CloseBuyingStore
        }
        0x0817 => {
            exact_length(bytes, 6)?;
            ScriptWorldRequest::OpenBuyingStore(u32_at(bytes, 2)?)
        }
        0x0819 => {
            variable_length(bytes, 12, item_width + 4)?;
            let items = bytes[12..]
                .chunks_exact(item_width + 4)
                .map(|record| {
                    Ok((
                        u16_at(record, 0)?.checked_sub(2).ok_or("Invalid sale inventory index")?,
                        item_at(record, 2, item_width)?,
                        u16_at(record, item_width + 2)?,
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?;
            if items.len() > 100 {
                return Err("Too many buying store sale items".into());
            }
            ScriptWorldRequest::TradeBuyingStore {
                account_id: u32_at(bytes, 4)?,
                store_id: u32_at(bytes, 8)?,
                items,
            }
        }
        0x0835 => {
            if bytes.len() < 15 || usize::from(u16_at(bytes, 2)?) != bytes.len() {
                return Err("Malformed store search packet".into());
            }
            let items_count = usize::from(bytes[13]);
            let cards_count = usize::from(bytes[14]);
            if items_count > 10 || cards_count > 10 || bytes.len() != 15 + (items_count + cards_count) * item_width {
                return Err("Invalid store search list length".into());
            }
            let items = (0..items_count)
                .map(|index| item_at(bytes, 15 + index * item_width, item_width))
                .collect::<Result<Vec<_>, _>>()?;
            let cards = (0..cards_count)
                .map(|index| item_at(bytes, 15 + (index + items_count) * item_width, item_width).map(|card| card as u16))
                .collect::<Result<Vec<_>, _>>()?;
            ScriptWorldRequest::SearchStores {
                kind: bytes[4],
                max_price: u32_at(bytes, 5)?,
                min_price: u32_at(bytes, 9)?,
                items,
                cards,
            }
        }
        0x0838 => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::NextSearchPage
        }
        0x083B => {
            exact_length(bytes, 2)?;
            ScriptWorldRequest::CloseStoreSearch
        }
        0x083C => {
            exact_length(bytes, 10 + item_width)?;
            ScriptWorldRequest::LocateStore {
                account_id: u32_at(bytes, 2)?,
                store_id: u32_at(bytes, 6)?,
                item_id: item_at(bytes, 10, item_width)?,
            }
        }
        _ => return Ok(None),
    };
    Ok(Some(request))
}

fn exact_length(bytes: &[u8], expected: usize) -> Result<(), String> {
    if bytes.len() == expected {
        Ok(())
    } else {
        Err("Malformed game system packet length".into())
    }
}

fn boolean(value: u32) -> Result<bool, String> {
    match value {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err("Invalid boolean packet field".into()),
    }
}
fn variable_length(bytes: &[u8], base: usize, record_size: usize) -> Result<(), String> {
    if bytes.len() >= base && usize::from(u16_at(bytes, 2)?) == bytes.len() && (bytes.len() - base) % record_size == 0 {
        Ok(())
    } else {
        Err("Malformed game system item list".into())
    }
}
fn u16_at(bytes: &[u8], offset: usize) -> Result<u16, String> {
    Ok(u16::from_le_bytes(
        bytes
            .get(offset..offset + 2)
            .ok_or("Truncated game system packet")?
            .try_into()
            .unwrap(),
    ))
}
fn u32_at(bytes: &[u8], offset: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(
        bytes
            .get(offset..offset + 4)
            .ok_or("Truncated game system packet")?
            .try_into()
            .unwrap(),
    ))
}
fn item_at(bytes: &[u8], offset: usize, width: usize) -> Result<i32, String> {
    if width == 2 {
        Ok(i32::from(u16_at(bytes, offset)?))
    } else {
        i32::try_from(u32_at(bytes, offset)?).map_err(|_| "Invalid item identifier".into())
    }
}
fn text(bytes: &[u8]) -> Result<String, String> {
    let end = bytes.iter().position(|byte| *byte == 0).unwrap_or(bytes.len());
    String::from_utf8(bytes[..end].to_vec()).map_err(|_| "Game system text must be UTF-8".into())
}

pub fn header(id: u16) -> Vec<u8> {
    id.to_le_bytes().to_vec()
}

pub fn skill_failed(skill_id: u32, cause: models::enums::skill::UseSkillFailure) -> Vec<u8> {
    let mut packet = header(0x0110);
    packet.extend_from_slice(&(skill_id as u16).to_le_bytes());
    packet.extend_from_slice(&0u32.to_le_bytes());
    packet.extend_from_slice(&[0, cause.value() as u8]);
    packet
}
pub fn set_length(packet: &mut [u8]) {
    let length = packet.len() as u16;
    packet[2..4].copy_from_slice(&length.to_le_bytes());
}
pub fn fixed_string(packet: &mut Vec<u8>, value: &str, length: usize) {
    let start = packet.len();
    packet.resize(start + length, 0);
    let copied = value.len().min(length.saturating_sub(1));
    packet[start..start + copied].copy_from_slice(&value.as_bytes()[..copied]);
}
pub fn emotion(id: u32, emotion: u8) -> Vec<u8> {
    let mut packet = header(0x00C0);
    packet.extend_from_slice(&id.to_le_bytes());
    packet.push(emotion);
    packet
}
pub fn store_disappear(account: u32) -> Vec<u8> {
    let mut packet = header(0x0816);
    packet.extend_from_slice(&account.to_le_bytes());
    packet
}
pub fn vending_disappear(account: u32) -> Vec<u8> {
    let mut packet = header(0x0132);
    packet.extend_from_slice(&account.to_le_bytes());
    packet
}
pub fn vending_sign(store: &VendingStore) -> Vec<u8> {
    let mut packet = header(0x0131);
    packet.extend_from_slice(&store.char_id.to_le_bytes());
    fixed_string(&mut packet, &store.title, 80);
    packet
}
pub fn vending_items(
    store: &VendingStore,
    cart: &[database::model::InventoryRecord],
    own: bool,
    configuration: &GlobalConfigService,
    packetver: u32,
) -> Vec<u8> {
    let mut packet = header(if own {
        0x0136
    } else if packetver >= 20100105 {
        0x0800
    } else {
        0x0133
    });
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet.extend_from_slice(&store.char_id.to_le_bytes());
    if !own && packetver >= 20100105 {
        packet.extend_from_slice(&store.id.to_le_bytes());
    }
    for offer in &store.offers {
        let Some(record) = cart.iter().find(|record| record.id == offer.inventory_id) else {
            continue;
        };
        let item = configuration.get_item(record.item_id);
        packet.extend_from_slice(&offer.price.to_le_bytes());
        if own {
            packet.extend_from_slice(&(offer.index + 2).to_le_bytes());
            packet.extend_from_slice(&offer.amount.to_le_bytes());
        } else {
            packet.extend_from_slice(&offer.amount.to_le_bytes());
            packet.extend_from_slice(&(offer.index + 2).to_le_bytes());
        }
        packet.push(item.item_type.to_client_type() as u8);
        if packetver >= 20181121 {
            packet.extend_from_slice(&(record.item_id as u32).to_le_bytes());
        } else {
            packet.extend_from_slice(&(record.item_id as u16).to_le_bytes());
        }
        packet.extend_from_slice(&[u8::from(record.is_identified), u8::from(record.is_damaged), record.refine as u8]);
        for card in [record.card0, record.card1, record.card2, record.card3] {
            packet.extend_from_slice(&card.to_le_bytes());
        }
        if packetver >= 20150226 {
            packet.extend_from_slice(&[0; 25]);
        }
        if !own && packetver >= 20160921 {
            packet.extend_from_slice(&(item.location as u32).to_le_bytes());
            packet.extend_from_slice(&(item.view.unwrap_or(0) as u16).to_le_bytes());
        }
        if packetver >= 20200916 {
            packet.push(0);
        }
    }
    set_length(&mut packet);
    packet
}
pub fn store_sign(store: &BuyingStore) -> Vec<u8> {
    let mut packet = header(0x0814);
    packet.extend_from_slice(&store.char_id.to_le_bytes());
    fixed_string(&mut packet, &store.title, 80);
    packet
}
pub fn store_items(store: &BuyingStore, own: bool, packetver: u32, item_type: impl Fn(i32) -> u8) -> Vec<u8> {
    let mut packet = header(if own { 0x0813 } else { 0x0818 });
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet.extend_from_slice(&store.char_id.to_le_bytes());
    if !own {
        packet.extend_from_slice(&store.id.to_le_bytes());
    }
    packet.extend_from_slice(&store.zeny_limit.to_le_bytes());
    for offer in &store.offers {
        packet.extend_from_slice(&offer.price.to_le_bytes());
        packet.extend_from_slice(&offer.amount.to_le_bytes());
        packet.push(item_type(offer.item_id));
        if packetver >= 20181121 {
            packet.extend_from_slice(&(offer.item_id as u32).to_le_bytes());
        } else {
            packet.extend_from_slice(&(offer.item_id as u16).to_le_bytes());
        }
    }
    set_length(&mut packet);
    packet
}
pub fn search_failure(reason: u8) -> Vec<u8> {
    let mut packet = header(0x0837);
    packet.push(reason);
    packet
}

pub fn search_results(results: &[StoreSearchResult], first_page: bool, more: bool, uses: u8, packetver: u32) -> Vec<u8> {
    let mut packet = header(if packetver >= 20200916 { 0x0B64 } else { 0x0836 });
    packet.extend_from_slice(&0u16.to_le_bytes());
    packet.extend_from_slice(&[u8::from(first_page), u8::from(more), uses]);
    for result in results {
        packet.extend_from_slice(&result.store.id.to_le_bytes());
        packet.extend_from_slice(&result.store.char_id.to_le_bytes());
        fixed_string(&mut packet, &result.store.title, 80);
        if packetver >= 20181121 {
            packet.extend_from_slice(&(result.item_id as u32).to_le_bytes());
        } else {
            packet.extend_from_slice(&(result.item_id as u16).to_le_bytes());
        }
        packet.push(result.item_type);
        packet.extend_from_slice(&result.price.to_le_bytes());
        packet.extend_from_slice(&result.amount.to_le_bytes());
        if packetver < 20200916 {
            packet.push(result.refine);
        }
        for card in result.cards {
            packet.extend_from_slice(&card.to_le_bytes());
        }
        if packetver >= 20150226 {
            packet.extend_from_slice(&[0; 25]);
        }
        if packetver >= 20200916 {
            packet.extend_from_slice(&[result.refine, 0]);
        }
    }
    set_length(&mut packet);
    packet
}
pub fn guild_basic(name: &str, id: u32, level: u16, experience: u64, points: u16, next_experience: u64) -> Vec<u8> {
    let mut packet = header(0x01B6);
    for value in [
        id,
        u32::from(level),
        0,
        16,
        0,
        experience.min(u32::MAX as u64) as u32,
        next_experience.min(u32::MAX as u64) as u32,
        u32::from(points),
        0,
        0,
        0,
    ] {
        packet.extend_from_slice(&value.to_le_bytes());
    }
    fixed_string(&mut packet, name, 24);
    fixed_string(&mut packet, "", 24);
    fixed_string(&mut packet, "", 16);
    packet.extend_from_slice(&0u32.to_le_bytes());
    packet
}
pub fn homunculus_info(homunculus: &HomunculusRecord) -> Vec<u8> {
    let mut packet = header(0x022E);
    fixed_string(&mut packet, &homunculus.name, 24);
    let flags = if homunculus.renamed {
        HomunculusInfoFlag::Renamed.as_flag()
    } else {
        0
    } | if homunculus.active {
        0
    } else {
        HomunculusInfoFlag::Resting.as_flag()
    } | if homunculus.hp > 0 {
        HomunculusInfoFlag::Alive.as_flag()
    } else {
        0
    };
    packet.push(flags);
    let data = super::world_data()
        .homunculi
        .iter()
        .find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id)
        .unwrap();
    let snapshot = super::homunculus::homunculus_snapshot(homunculus, 150).unwrap();
    let delay = ((200.0 - snapshot.aspd()) * 10.0).clamp(100.0, 2000.0) as u16;
    let hp = if homunculus.max_hp > 32767 {
        ((u64::from(homunculus.hp) * 100 / u64::from(homunculus.max_hp)) as u16, 100)
    } else {
        (homunculus.hp as u16, homunculus.max_hp as u16)
    };
    let sp = if homunculus.max_sp > 32767 {
        ((u64::from(homunculus.sp) * 100 / u64::from(homunculus.max_sp)) as u16, 100)
    } else {
        (homunculus.sp as u16, homunculus.max_sp as u16)
    };
    for value in [
        homunculus.level,
        homunculus.hunger,
        (homunculus.intimacy / 100) as u16,
        0,
        (snapshot.atk_left_side() + snapshot.atk_right_side()).clamp(0, i32::from(i16::MAX)) as u16,
        snapshot.matk_max().min(i16::MAX as u16),
        snapshot.hit().max(0) as u16,
        snapshot.luk() / 3 + 1,
        snapshot.def().saturating_add(snapshot.vit().min(i16::MAX as u16) as i16).max(0) as u16,
        snapshot.mdef().max(0) as u16,
        snapshot.flee().max(0) as u16,
        delay,
        hp.0,
        hp.1,
        sp.0,
        sp.1,
    ] {
        packet.extend_from_slice(&value.to_le_bytes());
    }
    packet.extend_from_slice(&(homunculus.experience.min(u32::MAX as u64) as u32).to_le_bytes());
    let next = if homunculus.level >= 99 {
        0
    } else {
        super::world_data()
            .homunculus_experience
            .get(usize::from(homunculus.level - 1))
            .copied()
            .unwrap_or(0)
    };
    packet.extend_from_slice(&(next.min(u32::MAX as u64) as u32).to_le_bytes());
    packet.extend_from_slice(&homunculus.skill_points.to_le_bytes());
    let size =
        models::enums::size::Size::try_from_string(if homunculus.evolved { &data.evolution_size } else { &data.size }).unwrap_or_default();
    packet.extend_from_slice(&(1u16 + size.value() as u16).to_le_bytes());
    packet
}

pub fn storage_packets(
    records: &[database::model::InventoryRecord],
    configuration: &'static GlobalConfigService,
    packetver: u32,
) -> Vec<Vec<u8>> {
    container_packets(records, configuration, packetver, false)
}

pub fn cart_packets(
    records: &[database::model::InventoryRecord],
    configuration: &'static GlobalConfigService,
    packetver: u32,
) -> Vec<Vec<u8>> {
    container_packets(records, configuration, packetver, true)
}

fn container_packets(
    records: &[database::model::InventoryRecord],
    configuration: &'static GlobalConfigService,
    packetver: u32,
    cart: bool,
) -> Vec<Vec<u8>> {
    let modern = packetver >= 20181002;
    let normal_id = if modern {
        0x0B09
    } else if packetver >= 20120925 {
        if cart { 0x0993 } else { 0x0995 }
    } else if packetver >= 20080102 {
        if cart { 0x02E9 } else { 0x02EA }
    } else {
        if cart { 0x01EF } else { 0x0295 }
    };
    let equip_id = if packetver >= 20200916 {
        0x0B39
    } else if modern {
        0x0B0A
    } else if packetver >= 20150226 {
        if cart { 0x0A0F } else { 0x0A10 }
    } else if packetver >= 20120925 {
        if cart { 0x0994 } else { 0x0996 }
    } else {
        if cart { 0x02D2 } else { 0x02D1 }
    };
    let mut normal = header(normal_id);
    let mut equip = header(equip_id);
    normal.extend_from_slice(&0u16.to_le_bytes());
    equip.extend_from_slice(&0u16.to_le_bytes());
    if modern {
        normal.push(if cart { 1 } else { 2 });
        equip.push(if cart { 1 } else { 2 });
    } else if packetver >= 20120925 {
        fixed_string(&mut normal, if cart { "" } else { "Storage" }, 24);
        fixed_string(&mut equip, if cart { "" } else { "Storage" }, 24);
    }
    for (index, record) in records.iter().enumerate() {
        let item = configuration.get_item(record.item_id);
        let stack = item.item_type.is_stackable();
        let packet = if stack { &mut normal } else { &mut equip };
        packet.extend_from_slice(&(index as u16 + if cart { 2 } else { 1 }).to_le_bytes());
        if packetver >= 20181121 {
            packet.extend_from_slice(&(record.item_id as u32).to_le_bytes());
        } else {
            packet.extend_from_slice(&(record.item_id as u16).to_le_bytes());
        }
        packet.push(if item.item_type == models::enums::item::ItemType::PetEgg {
            4
        } else {
            item.item_type.value() as u8
        });
        if packetver < 20120925 {
            packet.push(u8::from(record.is_identified));
        }
        if stack {
            packet.extend_from_slice(&record.amount.to_le_bytes());
        } else if packetver >= 20120925 {
            packet.extend_from_slice(&(item.location as u32).to_le_bytes());
        } else {
            packet.extend_from_slice(&(item.location as u16).to_le_bytes());
        }
        if packetver >= 20120925 {
            packet.extend_from_slice(&0u32.to_le_bytes());
        } else {
            packet.extend_from_slice(&0u16.to_le_bytes());
        }
        if !stack {
            if packetver < 20120925 {
                packet.push(u8::from(record.is_damaged));
            }
            if packetver < 20200916 {
                packet.push(record.refine as u8);
            }
        }
        for card in [record.card0, record.card1, record.card2, record.card3] {
            packet.extend_from_slice(&card.to_le_bytes());
        }
        if packetver >= 20080102 || (!stack && packetver >= 20071002) {
            packet.extend_from_slice(&0u32.to_le_bytes());
        }
        if !stack {
            packet.extend_from_slice(&0u16.to_le_bytes());
            if packetver >= 20100629 {
                packet.extend_from_slice(&(item.view.unwrap_or(0) as u16).to_le_bytes());
            }
            if packetver >= 20150226 {
                packet.push(0);
                packet.extend_from_slice(&[0; 25]);
            }
            if packetver >= 20200916 {
                packet.extend_from_slice(&[record.refine as u8, 0]);
            }
        }
        if packetver >= 20120925 {
            let flag = if record.is_identified {
                StorageItemFlag::Identified.as_flag()
            } else {
                0
            } | if !stack && record.is_damaged {
                StorageItemFlag::Damaged.as_flag()
            } else {
                0
            };
            packet.push(flag);
        }
    }
    set_length(&mut normal);
    set_length(&mut equip);
    let mut packets = Vec::new();
    if modern {
        let mut start = header(0x0B08);
        start.extend_from_slice(&(if cart { 6u16 } else { 13u16 }).to_le_bytes());
        start.push(if cart { 1 } else { 2 });
        if cart {
            start.push(0);
        } else {
            start.extend_from_slice(b"Storage\0");
        }
        packets.push(start);
    }
    packets.extend([normal, equip]);
    if modern {
        packets.push(vec![0x0B, 0x0B, if cart { 1 } else { 2 }, 0]);
    }
    let mut count = header(if cart { 0x0121 } else { 0x00F2 });
    count.extend_from_slice(&(records.len() as u16).to_le_bytes());
    count.extend_from_slice(&(if cart { 100u16 } else { 600u16 }).to_le_bytes());
    if cart {
        let weight = records
            .iter()
            .map(|record| u32::from(record.amount.max(0) as u16) * configuration.get_item(record.item_id).weight.max(0) as u32)
            .sum::<u32>();
        count.extend_from_slice(&weight.to_le_bytes());
        count.extend_from_slice(&80_000u32.to_le_bytes());
    }
    packets.push(count);
    packets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn party_packets_preserve_primary_layouts_and_the_configured_name_invite_shuffle() {
        let mut create = header(0x01E8);
        fixed_string(&mut create, "Classic Party", 24);
        create.extend_from_slice(&[1, 0]);
        assert_eq!(world_frame_length(0x01E8, 20120229), Some(FrameLength::Fixed(28)));
        assert_eq!(
            decode_request(&create, 20120229).unwrap(),
            Some(ScriptWorldRequest::CreateParty {
                name: "Classic Party".into(),
                item_pickup: true,
                item_share: false
            })
        );
        assert!(decode_request(&create[..27], 20120229).is_err());
        let mut name = header(0x088D);
        fixed_string(&mut name, "Member", 24);
        assert_eq!(
            decode_request(&name, 20120229).unwrap(),
            Some(ScriptWorldRequest::InvitePartyByName("Member".into()))
        );
        assert_eq!(world_frame_length(0x02C4, 20120229), None);
        let mut skill = header(0x02C4);
        skill.extend_from_slice(&[0; 8]);
        assert_eq!(decode_request(&skill, 20120229).unwrap(), None);
        let mut answer = header(0x02C7);
        answer.extend_from_slice(&123u32.to_le_bytes());
        answer.push(1);
        assert_eq!(
            decode_request(&answer, 20120229).unwrap(),
            Some(ScriptWorldRequest::AnswerPartyInvite {
                party_id: 123,
                accept: true
            })
        );
        answer[6] = 2;
        assert!(decode_request(&answer, 20120229).is_err());
        let mut options = header(0x07D7);
        options.extend_from_slice(&1u32.to_le_bytes());
        options.extend_from_slice(&[1, 1]);
        assert_eq!(
            decode_request(&options, 20120229).unwrap(),
            Some(ScriptWorldRequest::ChangePartyOptions {
                exp_share: true,
                item_rules: Some((true, true))
            })
        );
        let message = b"Member : hello\0";
        let mut chat = header(0x0108);
        chat.extend_from_slice(&(4u16 + message.len() as u16).to_le_bytes());
        chat.extend_from_slice(message);
        assert_eq!(
            decode_request(&chat, 20120229).unwrap(),
            Some(ScriptWorldRequest::PartyMessage("Member : hello".into()))
        );
        chat.pop();
        assert!(decode_request(&chat, 20120229).is_err());
    }

    #[test]
    fn recognizes_the_configured_clients_shuffled_store_packet_ids() {
        let mut packet = header(0x0835);
        packet.extend_from_slice(&97u16.to_le_bytes());
        packet.extend_from_slice(&100u32.to_le_bytes());
        packet.push(1);
        fixed_string(&mut packet, "Buy herbs", 80);
        packet.extend_from_slice(&507u16.to_le_bytes());
        packet.extend_from_slice(&2u16.to_le_bytes());
        packet.extend_from_slice(&50u32.to_le_bytes());
        let decoded = decode_request(&packet, 20120229).unwrap().unwrap();
        assert!(matches!(decoded, ScriptWorldRequest::CreateBuyingStore { zeny_limit: 100, .. }));
        assert_eq!(
            decode_request(&header(0x089B), 20120229).unwrap(),
            Some(ScriptWorldRequest::CloseBuyingStore)
        );
    }

    #[test]
    fn rejects_truncated_capture_and_forged_variable_packet_lengths() {
        assert!(decode_request(&[0x9F, 0x01, 1], 20120229).is_err());
        let mut packet = header(0x08AB);
        packet.extend_from_slice(&15u16.to_le_bytes());
        packet.push(1);
        packet.extend_from_slice(&100u32.to_le_bytes());
        packet.extend_from_slice(&0u32.to_le_bytes());
        packet.extend_from_slice(&[255, 0]);
        assert!(decode_request(&packet, 20120229).is_err());
    }

    #[test]
    fn guild_commands_frame_full_primary_payloads_and_ignore_forged_actor_fields() {
        let mut create = header(0x0165);
        create.extend_from_slice(&999u32.to_le_bytes());
        fixed_string(&mut create, "Classic Guild", 24);
        assert_eq!(
            decode_request(&create, 20120229).unwrap(),
            Some(ScriptWorldRequest::CreateGuild("Classic Guild".into()))
        );
        let mut invite = header(0x0168);
        for actor in [150_001u32, 999, 999] {
            invite.extend_from_slice(&actor.to_le_bytes());
        }
        assert_eq!(
            decode_request(&invite, 20120229).unwrap(),
            Some(ScriptWorldRequest::InviteGuild(150_001))
        );
        assert!(decode_request(&invite[..6], 20120229).is_err());
        let mut answer = header(0x016B);
        answer.extend_from_slice(&1u32.to_le_bytes());
        answer.extend_from_slice(&2u32.to_le_bytes());
        assert!(decode_request(&answer, 20120229).is_err());
    }
}

pub fn fame_list(kind: u8, rankings: &[crate::repository::fame_repository::FameEntry], own_points: u32, packetver: u32) -> Vec<u8> {
    let modern = packetver >= 20130605;
    let mut packet = if modern {
        let mut packet = header(0x097D);
        packet.extend_from_slice(&u16::from(kind).to_le_bytes());
        packet
    } else {
        header(match kind {
            0 => 0x0219,
            1 => 0x021A,
            2 => 0x0226,
            _ => 0x0238,
        })
    };
    for index in 0..10 {
        fixed_string(&mut packet, rankings.get(index).map_or("", |entry| entry.name.as_str()), 24);
    }
    for index in 0..10 {
        packet.extend_from_slice(&rankings.get(index).map_or(0, |entry| entry.points).to_le_bytes());
    }
    if modern {
        packet.extend_from_slice(&own_points.to_le_bytes());
    }
    packet
}
