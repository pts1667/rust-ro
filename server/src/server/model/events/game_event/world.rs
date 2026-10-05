use super::*;
use crate::server::Server;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::state::server::ServerState;

#[derive(Debug, PartialEq, Clone)]
pub struct PetCaptureClaimResult {
    pub claim_id: u64,
    pub char_id: u32,
    pub target_id: u32,
    pub map_key: MapInstanceKey,
    pub class_id: Option<u16>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootClaimResult {
    pub claim_id: u64,
    pub char_id: u32,
    pub pet_id: u32,
    pub target_id: u32,
    pub map_key: MapInstanceKey,
    pub item: Option<models::item::DroppedItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PetLootDropResult {
    pub claim_id: u64,
    pub char_id: u32,
    pub map_key: MapInstanceKey,
    pub accepted: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerTradeAction {
    pub char_id: u32,
    pub account_id: u32,
    pub auth_code: i32,
    pub request: crate::server::model::game_systems::PlayerTradeRequest,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ScriptWorld {
    pub char_id: u32,
    pub request: crate::server::model::game_systems::ScriptWorldRequest,
}

impl GameEventHandler for PetCaptureClaimResult {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let result = self;
        if let Err(error) = server
            .script_world_service()
            .complete_pet_capture(server, state, result, tick as u64)
        {
            warn!("Pet capture completion failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for PetLootClaimResult {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let result = self;
        if let Err(error) = server.script_world_service().complete_pet_loot(server, state, result, tick as u64) {
            warn!("Pet loot completion failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for PetLootDropResult {
    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let result = self;
        if let Err(error) = server
            .script_world_service()
            .complete_pet_loot_drop(server, state, result, tick as u64)
        {
            warn!("Pet loot delivery failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for ScriptWorld {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let crate::server::model::events::game_event::ScriptWorld { char_id, request } = self;
        if let Err(error) = server
            .script_world_service()
            .handle_request(server, state, char_id, request, tick as u64)
        {
            warn!("Script world request failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for PlayerTradeAction {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, tick: u128) -> Result<(), String> {
        let action = self;
        if let Err(error) = server.handle_player_trade(state, action, tick as u64) {
            warn!("Player trade request failed: {error}");
        }
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::duel::DuelCommand {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let command = self;
        server.handle_duel_command(state, command);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::duel::DuelOutcome {
    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        server.apply_duel_outcome(state, self);
        Ok(())
    }
}

impl GameEventHandler for crate::server::model::battleground_queue::BattlegroundQueueCommand {
    fn required_character(&self) -> Option<u32> {
        Some(self.char_id)
    }

    fn handle(self, server: &Server, state: &mut ServerState, _tick: u128) -> Result<(), String> {
        let command = self;
        server.handle_battleground_queue_command(state, command);
        Ok(())
    }
}
