use super::*;
use crate::server::Server;
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub enum ClientCommand {
    ChangeDirection { head_dir: u16, dir: u8 },
    Emotion(u8),
    CharacterName(u32),
    PvpInfo,
    ViewEquipment { target_id: u32 },
    Config { kind: i32, enabled: bool },
    LessEffect(bool),
    UserCount,
    StopAttack,
    CloseStorage,
    AutoRevive,
    ExplosionSpirits,
    AtCommand(String),
}

/// Small client requests that only read or toggle state of the sending character.
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterClientCommand {
    pub char_id: u32,
    pub command: ClientCommand,
}

impl GameEventHandler for CharacterClientCommand {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        server.run_client_command(state, self.char_id, self.command, tick)
    }
}
