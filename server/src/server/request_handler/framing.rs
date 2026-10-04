use std::collections::BTreeMap;

use packets::packets::Packet;

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
                    lengths.insert(id, FrameLength::Fixed($kind::base_len(packetver)));
                }
            )+ };
        }
        macro_rules! variable {
            ($($kind:ident),+ $(,)?) => { $(
                let packet = $kind::new(packetver);
                let id = packet.id(packetver);
                if let Ok(id) = u16::from_str_radix(id.trim_start_matches("0x"), 16) {
                    lengths.insert(id, FrameLength::Variable { minimum: $kind::base_len(packetver).max(4) });
                }
            )+ };
        }
        fixed!(
            PacketCaLogin,
            PacketChDeleteChar4Reserved,
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
            let length = crate::server::service::script_world_service::world_frame_length(id, self.packetver)
                .or_else(|| super::script_operations::frame_length(id, self.packetver))
                .or_else(|| self.lengths.get(&id).copied())
                .or_else(|| crate::server::service::script_world_service::client_frame_length(id, self.packetver))
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
        let mut frames=ClientFrames::new(20120229);
        let mut selection=0x011b_u16.to_le_bytes().to_vec();
        selection.extend_from_slice(&(models::enums::skill_enums::SkillEnum::AlTeleport.id() as u16).to_le_bytes());
        let mut map=[0u8;16];map[..10].copy_from_slice(b"Random.gat");selection.extend_from_slice(&map);
        assert!(frames.push(&selection[..15]).unwrap().is_empty());
        let mut last=selection[15..].to_vec();last.extend_from_slice(&[0xa1,0x01,0]);
        assert_eq!(frames.push(&last).unwrap(),vec![selection,vec![0xa1,0x01,0]]);
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
