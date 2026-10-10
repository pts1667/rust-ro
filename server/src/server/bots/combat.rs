//! Fights between players. Outside maps where players fight, a duel is how two of them become enemies.
use serde_json::{Value, json};

use super::interaction::{character_of, player_named};
use crate::server::Server;
use crate::server::model::duel::{DuelAction, DuelCommand};
use crate::server::state::server::ServerState;

#[derive(Debug, Clone, PartialEq)]
pub enum DuelRequest {
    /// Opens a duel with `limit` places, 0 for no limit.
    Create { limit: usize },
    Invite { name: String },
    Accept,
    Decline,
    Leave,
}

impl Server {
    /// Runs the duel command now: the checks come first, as the command itself only tells the player in a message.
    pub(crate) fn bot_duel(&self, state: &mut ServerState, char_id: u32, request: &DuelRequest) -> Result<Value, String> {
        let character = character_of(state, char_id)?;
        let duels = self.duels();
        let (action, argument) = match request {
            DuelRequest::Create { limit } => {
                if duels.duel_of(char_id).is_some() {
                    return Err("The character is already in a duel".into());
                }
                (DuelAction::Create, limit.to_string())
            }
            DuelRequest::Invite { name } => {
                let duel = duels.duel_of(char_id).ok_or("Create a duel before inviting")?;
                let invited = player_named(state, name)?;
                if invited.char_id == char_id || invited.map_instance_key != character.map_instance_key {
                    return Err(format!("{} is not another player of this map", name.trim()));
                }
                if duels.duel_of(invited.char_id).is_some() {
                    return Err(format!("{} is already in a duel", invited.name));
                }
                let (invited_id, invited_name) = (invited.char_id, invited.name.clone());
                self.handle_duel_command(state, DuelCommand { char_id, action: DuelAction::Invite, argument: invited_name.clone() });
                if duels.invitation_of(invited_id) != Some(duel) {
                    return Err(format!("{invited_name} could not be invited, the duel is full"));
                }
                return Ok(json!({ "invited": invited_name }));
            }
            DuelRequest::Accept => {
                if duels.duel_of(char_id).is_some() {
                    return Err("The character is already in a duel".into());
                }
                if duels.invitation_of(char_id).is_none() {
                    return Err("No duel invitation is pending".into());
                }
                (DuelAction::Accept, String::new())
            }
            DuelRequest::Decline => {
                if duels.invitation_of(char_id).is_none() {
                    return Err("No duel invitation is pending".into());
                }
                (DuelAction::Reject, String::new())
            }
            DuelRequest::Leave => {
                if duels.duel_of(char_id).is_none() {
                    return Err("The character is not in a duel".into());
                }
                (DuelAction::Leave, String::new())
            }
        };
        self.handle_duel_command(state, DuelCommand { char_id, action, argument });
        Ok(json!({ "duel": duels.duel_of(char_id) }))
    }
}
