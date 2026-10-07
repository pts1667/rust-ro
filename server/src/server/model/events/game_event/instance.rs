use super::*;
use crate::server::Server;
use crate::server::state::server::ServerState;

/// A player asked to destroy their memorial dungeon (`CZ_MEMORIALDUNGEON_COMMAND`).
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterInstanceCommand {
    pub char_id: u32,
}

impl GameEventHandler for CharacterInstanceCommand {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        server.instance_destroy_command(state, self.char_id);
        Ok(())
    }
}
