use packets::packets::{
    PacketCzAckStorePassword, PacketCzDoridori,
    PacketCzCancelLockon, PacketCzChangeDirection, PacketCzChopokgi, PacketCzClientVersion, PacketCzCloseDialog, PacketCzCloseStore,
    PacketCzConfig, PacketCzEquipwinMicroscope, PacketCzLesseffect, PacketCzMovetoMap, PacketCzProgress, PacketCzReqEmotion,
    PacketCzReqPvppoint, PacketCzReqUserCount, PacketCzReqnameBygid, PacketCzReset, PacketCzStandingResurrection,
};

use crate::server::Server;
use crate::server::model::events::game_event::{CharacterClientCommand, ClientCommand, GameEvent};
use crate::server::model::request::Request;

const RESET_STATS: i16 = 0;
const RESET_SKILLS: i16 = 1;

fn name_until_nul(name: &[char]) -> String {
    name.iter().take_while(|character| **character != '\0').collect()
}

/// Routes the small gameplay packets that only act on the sending character; returns false for any other packet.
pub fn handle(server: &Server, context: &Request) -> bool {
    let packet = context.packet().as_any();
    let session = context.session();
    if packet.downcast_ref::<PacketCzCloseDialog>().is_some() {
        session.close_dialog();
        return true;
    }
    // Accepted without effect: no progress bar command and storage passwords are a rathena TODO.
    if packet.downcast_ref::<PacketCzClientVersion>().is_some()
        || packet.downcast_ref::<PacketCzProgress>().is_some()
        || packet.downcast_ref::<PacketCzAckStorePassword>().is_some()
    {
        return true;
    }
    let command = if packet.downcast_ref::<PacketCzChangeDirection>().is_some() {
        let raw = context.packet().raw();
        ClientCommand::ChangeDirection {
            head_dir: u16::from_le_bytes([raw[2], raw[3]]),
            dir: raw[4],
        }
    } else if let Some(packet) = packet.downcast_ref::<PacketCzReqEmotion>() {
        ClientCommand::Emotion(packet.atype)
    } else if packet.downcast_ref::<PacketCzReqnameBygid>().is_some() {
        let raw = context.packet().raw();
        ClientCommand::CharacterName(u32::from_le_bytes([raw[2], raw[3], raw[4], raw[5]]))
    } else if packet.downcast_ref::<PacketCzReqPvppoint>().is_some() {
        ClientCommand::PvpInfo
    } else if let Some(packet) = packet.downcast_ref::<PacketCzEquipwinMicroscope>() {
        ClientCommand::ViewEquipment { target_id: packet.aid }
    } else if let Some(packet) = packet.downcast_ref::<PacketCzConfig>() {
        ClientCommand::Config {
            kind: packet.config,
            enabled: packet.value != 0,
        }
    } else if let Some(packet) = packet.downcast_ref::<PacketCzLesseffect>() {
        ClientCommand::LessEffect(packet.is_less != 0)
    } else if packet.downcast_ref::<PacketCzReqUserCount>().is_some() {
        ClientCommand::UserCount
    } else if packet.downcast_ref::<PacketCzCancelLockon>().is_some() {
        ClientCommand::StopAttack
    } else if packet.downcast_ref::<PacketCzCloseStore>().is_some() {
        ClientCommand::CloseStorage
    } else if packet.downcast_ref::<PacketCzStandingResurrection>().is_some() {
        ClientCommand::AutoRevive
    } else if packet.downcast_ref::<PacketCzChopokgi>().is_some() {
        ClientCommand::ExplosionSpirits
    } else if packet.downcast_ref::<PacketCzDoridori>().is_some() {
        ClientCommand::Doridori
    } else if let Some(packet) = packet.downcast_ref::<PacketCzMovetoMap>() {
        ClientCommand::AtCommand(format!(
            "@mapmove {} {} {}",
            name_until_nul(&packet.map_name),
            packet.x_pos,
            packet.y_pos
        ))
    } else if let Some(packet) = packet.downcast_ref::<PacketCzReset>() {
        match packet.atype {
            RESET_STATS => ClientCommand::AtCommand("@resetstat".into()),
            RESET_SKILLS => ClientCommand::AtCommand("@resetskill".into()),
            _ => return true,
        }
    } else {
        return false;
    };
    if let Some(char_id) = session.char_id {
        server.add_to_next_tick(GameEvent::CharacterClientCommand(CharacterClientCommand { char_id, command }));
    }
    true
}

#[cfg(test)]
mod tests {
    use packets::packets::Packet;

    use super::super::framing::ClientFrames;
    use super::*;

    macro_rules! assert_framed_and_parsed {
        ($version:expr, $($kind:ident),+ $(,)?) => {$({
            let mut packet = $kind::new($version);
            packet.fill_raw_with_packetver(Some($version));
            let raw = packet.raw.clone();
            let frames = ClientFrames::new($version)
                .push(&raw)
                .unwrap_or_else(|error| panic!("{} at {}: {error}", stringify!($kind), $version));
            assert_eq!(frames, vec![raw.clone()], "{} at {}", stringify!($kind), $version);
            let parsed = packets::packets_parser::parse(&raw, $version);
            assert!(parsed.as_any().downcast_ref::<$kind>().is_some(), "{} at {}", stringify!($kind), $version);
        })+};
    }

    #[test]
    fn gameplay_packets_reach_their_handler_at_every_supported_version() {
        for version in [20120229, 20120307] {
            assert_framed_and_parsed!(
                version,
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
            );
        }
    }
}
