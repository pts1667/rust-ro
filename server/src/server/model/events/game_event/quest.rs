use super::*;
use crate::server::Server;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::state::server::ServerState;

/// A player toggled a quest between active and inactive (`CZ_ACTIVE_QUEST`).
#[derive(Debug, PartialEq, Clone)]
pub struct CharacterQuestActivation {
    pub char_id: u32,
    pub quest_id: u32,
    pub active: bool,
}

impl GameEventHandler for CharacterQuestActivation {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        server.quest_set_active(state, self.char_id, self.quest_id, self.active);
        Ok(())
    }
}

/// A monster died and `char_id` is credited with the kill.
#[derive(Debug, PartialEq, Clone)]
pub struct QuestMonsterKill {
    pub char_id: u32,
    pub mob_id: u16,
    pub map_instance_key: MapInstanceKey,
    pub mob_x: u16,
    pub mob_y: u16,
}

impl GameEventHandler for QuestMonsterKill {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        server.quest_kill(state, self.char_id, self.mob_id, &self.map_instance_key, self.mob_x, self.mob_y);
        Ok(())
    }
}
