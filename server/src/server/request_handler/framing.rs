use std::collections::BTreeMap;

use packets::packets::{
    Packet, PacketCzAckStorePassword, PacketCzCancelLockon, PacketCzChangeDirection, PacketCzChopokgi, PacketCzClientVersion, PacketCzCloseDialog, PacketCzDoridori,
    PacketCzCloseStore, PacketCzConfig, PacketCzEquipwinMicroscope, PacketCzLesseffect, PacketCzMovetoMap, PacketCzProgress,
    PacketCzReqEmotion, PacketCzReqPvppoint, PacketCzReqUserCount, PacketCzReqnameBygid, PacketCzReset, PacketCzStandingResurrection,
};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameLength {
    Fixed(usize),
    Variable { minimum: usize },
}

pub struct ClientFrames {
    packetver: u32,
    pending: Vec<u8>,
    lengths: BTreeMap<u16, FrameLength>,
}

impl ClientFrames {
    pub fn new(packetver: u32) -> Self {
        let mut lengths = BTreeMap::new();
        macro_rules! fixed {
            ($($kind:ident),+ $(,)?) => { $(
                let packet = $kind::new(packetver);
                let id = packet.id(packetver);
                if let Ok(id) = u16::from_str_radix(id.trim_start_matches("0x"), 16) {
                    lengths.insert(id.swap_bytes(), FrameLength::Fixed($kind::base_len(packetver)));
                }
            )+ };
        }
        macro_rules! variable {
            ($($kind:ident),+ $(,)?) => { $(
                let packet = $kind::new(packetver);
                let id = packet.id(packetver);
                if let Ok(id) = u16::from_str_radix(id.trim_start_matches("0x"), 16) {
                    lengths.insert(id.swap_bytes(), FrameLength::Variable { minimum: $kind::base_len(packetver).max(4) });
                }
            )+ };
        }
        fixed!(
            PacketCaLogin,
            PacketChEnter,
            PacketChMakeChar,
            PacketChMakeChar2,
            PacketChMakeChar3,
            PacketChSelectChar,
            PacketCzAckSelectDealtype,
            PacketCzBlockingPlayCancel,
            PacketCzChooseMenu,
            PacketCzContactnpc,
            PacketCzEnter2,
            PacketCzInputEditdlg,
            PacketCzItemPickup,
            PacketCzItemThrow,
            PacketCzNotifyActorinit,
            PacketCzReqDisconnect2,
            PacketCzReqItemcomposition,
            PacketCzReqItemcompositionList,
            PacketCzReqNextScript,
            PacketCzReqTakeoffEquip,
            PacketCzReqWearEquip,
            PacketCzReqname,
            PacketCzReqnameall2,
            PacketCzCloseDialog,
            PacketCzCancelLockon,
            PacketCzChangeDirection,
            PacketCzReqEmotion,
            PacketCzReqnameBygid,
            PacketCzReqPvppoint,
            PacketCzEquipwinMicroscope,
            PacketCzConfig,
            PacketCzLesseffect,
            PacketCzReqUserCount,
            PacketCzClientVersion,
            PacketCzProgress,
            PacketCzStandingResurrection,
            PacketCzChopokgi,
            PacketCzCloseStore,
            PacketCzMovetoMap,
            PacketCzReset,
            PacketCzDoridori,
            PacketCzAckStorePassword,
            PacketCzReqDisconnect,
            PacketCzRequestAct,
            PacketCzRequestMove,
            PacketCzRequestMove2,
            PacketCzRequestTime,
            PacketCzRestart,
            PacketCzShortcutKeyChange,
            PacketCzStatusChange,
            PacketCzUpgradeSkilllevel,
            PacketCzUseItem,
            PacketCzUseSkill
        );
        variable!(
            PacketCzInputEditdlgstr,
            PacketCzPcPurchaseItemlist,
            PacketCzPcSellItemlist,
            PacketCzPlayerChat
        );
        Self {
            packetver,
            pending: Vec::with_capacity(2048),
            lengths,
        }
    }

    pub fn push(&mut self, bytes: &[u8]) -> Result<Vec<Vec<u8>>, String> {
        if self.pending.len().saturating_add(bytes.len()) > 131_072 {
            return Err("Client packet buffer exceeds its limit".into());
        }
        self.pending.extend_from_slice(bytes);
        let mut consumed = 0;
        let mut frames = Vec::new();
        while self.pending.len().saturating_sub(consumed) >= 2 {
            let remaining = &self.pending[consumed..];
            let id = u16::from_le_bytes([remaining[0], remaining[1]]);
            let length = super::talkie_box::layout(id, self.packetver)
                .map(|layout| FrameLength::Fixed(layout.length))
                .or_else(|| crate::server::service::script_world_service::world_frame_length(id, self.packetver))
                .or_else(|| crate::server::service::player_trade_service::frame_length(id))
                .or_else(|| super::char_requests::frame_length(id, self.packetver))
                .or_else(|| super::script_operations::frame_length(id, self.packetver))
                .or_else(|| crate::server::service::script_world_service::client_frame_length(id, self.packetver))
                .or_else(|| self.lengths.get(&id).copied())
                .ok_or_else(|| format!("Unknown client packet header {id:#06x}"))?;
            let required = match length {
                FrameLength::Fixed(length) => length,
                FrameLength::Variable { minimum } => {
                    if remaining.len() < 4 {
                        break;
                    }
                    let length = usize::from(u16::from_le_bytes([remaining[2], remaining[3]]));
                    if length < minimum {
                        return Err(format!("Client packet {id:#06x} is shorter than its header"));
                    }
                    length
                }
            };
            if required < 2 {
                return Err("Invalid client packet frame definition".into());
            }
            if remaining.len() < required {
                break;
            }
            frames.push(remaining[..required].to_vec());
            consumed += required;
        }
        self.pending.drain(..consumed);
        Ok(frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUPPORTED_VERSIONS: [u32; 2] = [20120229, 20120307];

    fn wire_id(id: &str) -> u16 {
        u16::from_str_radix(id.trim_start_matches("0x"), 16).unwrap().swap_bytes()
    }

    #[test]
    fn frames_login_char_and_map_entry_packets_at_every_supported_version() {
        for version in SUPPORTED_VERSIONS {
            let packets = [
                (wire_id(PacketCaLogin::new(version).id(version)), PacketCaLogin::base_len(version)),
                (wire_id(PacketChEnter::new(version).id(version)), PacketChEnter::base_len(version)),
                (wire_id(PacketChSelectChar::new(version).id(version)), PacketChSelectChar::base_len(version)),
                (wire_id(PacketCzEnter2::new(version).id(version)), PacketCzEnter2::base_len(version)),
                (wire_id(PacketCzRequestMove::new(version).id(version)), PacketCzRequestMove::base_len(version)),
                (wire_id(PacketCzRequestTime::new(version).id(version)), PacketCzRequestTime::base_len(version)),
            ];
            let mut frames = ClientFrames::new(version);
            let mut stream = Vec::new();
            for (id, length) in packets {
                let mut packet = id.to_le_bytes().to_vec();
                packet.resize(length, 0);
                stream.extend(packet);
            }
            let framed = frames.push(&stream).unwrap_or_else(|error| panic!("packetver {version}: {error}"));
            assert_eq!(
                framed.iter().map(|frame| (u16::from_le_bytes([frame[0], frame[1]]), frame.len())).collect::<Vec<_>>(),
                packets,
                "packetver {version}"
            );
        }
        assert_eq!(wire_id(PacketCaLogin::new(20120307).id(20120307)), 0x0064);
        assert_eq!(wire_id(PacketCzEnter2::new(20120307).id(20120307)), 0x086A);
    }

    #[test]
    fn keeps_alive_and_other_table_only_packets_are_framed_at_every_supported_version() {
        for version in SUPPORTED_VERSIONS {
            let mut frames = ClientFrames::new(version);
            let ping = [0x87, 0x01, 1, 2, 3, 4];
            assert_eq!(frames.push(&ping).unwrap(), vec![ping.to_vec()], "packetver {version}");
        }
    }

    #[test]
    fn npc_interaction_packets_are_framed_at_every_supported_version() {
        for version in SUPPORTED_VERSIONS {
            let mut frames = ClientFrames::new(version);
            let click = [0x90, 0x00, 1, 2, 3, 4, 1];
            let next = [0xB9, 0x00, 1, 2, 3, 4];
            let stream = [click.as_slice(), next.as_slice()].concat();
            assert_eq!(frames.push(&stream).unwrap(), vec![click.to_vec(), next.to_vec()], "packetver {version}");
        }
    }

    #[test]
    fn struct_lengths_agree_with_the_packet_table_except_the_legacy_walk_header() {
        const LEGACY_WALK: u16 = 0x035F;
        for version in SUPPORTED_VERSIONS {
            let frames = ClientFrames::new(version);
            for (id, length) in frames.lengths.iter().filter(|(id, _)| **id != LEGACY_WALK) {
                if let Some(table) = crate::server::service::script_world_service::client_frame_length(*id, version) {
                    assert_eq!(*length, table, "packetver {version} header {id:#06x}");
                }
            }
        }
    }

    #[test]
    fn requests_missing_from_the_packet_table_are_framed_from_their_struct() {
        let mut frames = ClientFrames::new(20120307);
        let equip = [0xA9, 0x00, 1, 0, 2, 0];
        let amount = [0x43, 0x01, 1, 2, 3, 4, 5, 6, 7, 8];
        let stream = [equip.as_slice(), amount.as_slice()].concat();
        assert_eq!(frames.push(&stream).unwrap(), vec![equip.to_vec(), amount.to_vec()]);
    }

    #[test]
    fn keeps_fragmented_world_packets_and_separates_coalesced_packets() {
        let mut frames = ClientFrames::new(20120229);
        assert!(frames.push(&[0x9F]).unwrap().is_empty());
        assert!(frames.push(&[0x01, 1, 2]).unwrap().is_empty());
        let complete = frames.push(&[3, 4, 0xA1, 0x01, 0]).unwrap();
        assert_eq!(complete, vec![vec![0x9F, 0x01, 1, 2, 3, 4], vec![0xA1, 0x01, 0]]);
    }

    #[test]
    fn configured_world_and_script_headers_do_not_shadow_existing_client_requests() {
        let frames = ClientFrames::new(20120229);
        for (id, _) in &frames.lengths {
            assert!(
                crate::server::service::script_world_service::world_frame_length(*id, frames.packetver).is_none(),
                "world header {id:#06x} also identifies an existing parsed request"
            );
            assert!(
                super::super::script_operations::frame_length(*id, frames.packetver).is_none(),
                "script header {id:#06x} also identifies an existing parsed request"
            );
        }
    }

    #[test]
    fn separates_crafting_ground_and_vending_commands_in_one_socket_read() {
        let mut frames = ClientFrames::new(20120229);
        let mut bytes = vec![0x8E, 0x01, 0, 0, 0, 0, 0, 0, 0, 0];
        bytes.extend_from_slice(&[0x69, 0x03, 1, 0, 11, 0, 150, 0, 150, 0]);
        bytes.extend_from_slice(&[0x30, 0x01, 1, 2, 3, 4]);
        let result = frames.push(&bytes).unwrap();
        assert_eq!(result.iter().map(Vec::len).collect::<Vec<_>>(), vec![10, 10, 6]);
    }

    #[test]
    fn ground_skill_header_follows_the_packet_version() {
        let ground_skill = |id: [u8; 2], packetver| {
            let mut frames = ClientFrames::new(packetver);
            let mut bytes = vec![id[0], id[1], 1, 0, 11, 0, 150, 0, 150, 0];
            bytes.extend_from_slice(&[0x30, 0x01, 1, 2, 3, 4]);
            frames.push(&bytes).unwrap().iter().map(Vec::len).collect::<Vec<_>>()
        };
        assert_eq!(ground_skill([0x69, 0x03], 20120229), vec![10, 6]);
        assert_eq!(ground_skill([0x38, 0x04], 20120307), vec![10, 6]);
        assert!(super::super::script_operations::frame_length(0x0438, 20120229).is_none());
    }

    #[test]
    fn preserves_variable_length_frames_until_the_declared_body_is_complete() {
        let mut frames = ClientFrames::new(20120229);
        let packet = vec![0xAB, 0x08, 15, 0, 1, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(frames.push(&packet[..3]).unwrap().is_empty());
        assert!(frames.push(&packet[3..14]).unwrap().is_empty());
        assert_eq!(frames.push(&packet[14..]).unwrap(), vec![packet]);
        assert!(frames.push(&[0xAB, 0x08, 3, 0]).is_err());
    }

    #[test]
    fn teleport_selection_preserves_the_fixed_map_name_across_fragmented_reads() {
        let mut frames = ClientFrames::new(20120229);
        let mut selection = 0x011B_u16.to_le_bytes().to_vec();
        selection.extend_from_slice(&(models::enums::skill_enums::SkillEnum::AlTeleport.id() as u16).to_le_bytes());
        let mut map = [0u8; 16];
        map[..10].copy_from_slice(b"Random.gat");
        selection.extend_from_slice(&map);
        assert!(frames.push(&selection[..15]).unwrap().is_empty());
        let mut last = selection[15..].to_vec();
        last.extend_from_slice(&[0xA1, 0x01, 0]);
        assert_eq!(frames.push(&last).unwrap(), vec![selection, vec![0xA1, 0x01, 0]]);
    }

    #[test]
    fn frames_known_unimplemented_client_commands_without_losing_the_next_command() {
        let mut frames = ClientFrames::new(20120229);
        let packet = vec![0x66, 0x03, 1, 2, 3, 0xA1, 0x01, 0];
        assert_eq!(frames.push(&packet).unwrap(), vec![vec![0x66, 0x03, 1, 2, 3], vec![
            0xA1, 0x01, 0
        ]]);
        assert!(frames.push(&[0xFF, 0xFF]).is_err());
    }
}
