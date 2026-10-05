use packets::packets::{Packet, PacketZcNotifyPlayerchat};

use crate::server::Server;
use crate::server::model::duel::{DuelAction, DuelCommand, DuelOutcome, DuelRequest};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

impl Server {
    pub(crate) fn handle_duel_command(&self, state: &mut ServerState, command: DuelCommand) {
        let actor = self.duel_actor();
        actor.send(DuelRequest::Prune { online: state.characters().keys().copied().collect() });
        let char_id = command.char_id;
        let Some(name) = state.characters().get(&char_id).map(|character| character.name.clone()) else {
            return;
        };
        match command.action {
            DuelAction::Create => {
                let limit = command.argument.parse::<usize>().unwrap_or(0);
                actor.send(DuelRequest::Create { char_id, limit });
            }
            DuelAction::Invite => {
                let target = state
                    .characters()
                    .values()
                    .find(|other| other.name == command.argument && other.char_id != char_id && other.map_instance_key == state.characters()[&char_id].map_instance_key)
                    .map(|other| other.char_id);
                let Some(target) = target else {
                    return self.duel_message(char_id, "Player not found on this map");
                };
                actor.send(DuelRequest::Invite { inviter: char_id, target });
            }
            DuelAction::Accept => actor.send(DuelRequest::Accept { char_id }),
            DuelAction::Reject => actor.send(DuelRequest::Reject { char_id, silent: false }),
            DuelAction::Killer => {
                let Some(character) = state.characters_mut().get_mut(&char_id) else {
                    return;
                };
                character.killer = !character.killer;
                let text = if character.killer { "You are now a killer: you can attack any player" } else { "You are no longer a killer" };
                self.duel_message(char_id, text);
            }
            DuelAction::Leave => actor.send(DuelRequest::Leave { char_id, explicit: true }),
        }
    }

    pub(crate) fn apply_duel_outcome(&self, state: &mut ServerState, outcome: DuelOutcome) {
        match outcome {
            DuelOutcome::Created { char_id, result } => match result {
                Ok(_) => {
                    self.duel_message(char_id, "You have created a duel. Use @invite <name> to invite players.");
                    self.notify_map_property(state, char_id);
                }
                Err(error) => self.duel_message(char_id, error),
            },
            DuelOutcome::Invited { inviter, target, result } => match result {
                Ok(()) => {
                    self.duel_message(inviter, "Duel invitation sent");
                    let inviter_name = state.characters().get(&inviter).map(|character| character.name.clone());
                    if let Some(inviter_name) = inviter_name {
                        self.duel_message(target, &format!("{inviter_name} invites you to a duel. Use @accept or @reject."));
                    }
                }
                Err(error) => self.duel_message(inviter, error),
            },
            DuelOutcome::Accepted { char_id, result } => match result {
                Ok(members) => {
                    self.notify_map_property(state, char_id);
                    let name = state.characters().get(&char_id).map(|character| character.name.clone()).unwrap_or_default();
                    for member in members {
                        self.duel_message(member, &format!("{name} joined the duel"));
                    }
                }
                Err(error) => self.duel_message(char_id, error),
            },
            DuelOutcome::Rejected { char_id, had_invitation, silent } => {
                if silent {
                    return;
                }
                let text = if had_invitation { "Duel invitation rejected" } else { "You have no pending duel invitation" };
                self.duel_message(char_id, text);
            }
            DuelOutcome::Left { char_id, explicit, was_member, remaining } => {
                if !was_member {
                    if explicit {
                        self.duel_message(char_id, "You are not in a duel");
                    }
                    return;
                }
                if explicit {
                    self.duel_message(char_id, "You left the duel");
                }
                self.notify_map_property(state, char_id);
                for member in remaining {
                    self.duel_message(member, "The duel has ended");
                    self.notify_map_property(state, member);
                }
            }
        }
    }

    /// Removes a character from its duel and discards any pending invitation, as for a death.
    pub(crate) fn leave_duel(&self, char_id: u32) {
        let actor = self.duel_actor();
        actor.send(DuelRequest::Leave { char_id, explicit: false });
        actor.send(DuelRequest::Reject { char_id, silent: true });
    }

    fn duel_message(&self, char_id: u32, text: &str) {
        let mut packet = PacketZcNotifyPlayerchat::new(GlobalConfigService::instance().packetver());
        packet.set_msg(text.to_string());
        packet.set_packet_length((4 + packet.msg.len()) as i16);
        packet.fill_raw();
        let _ = self
            .server_service()
            .notification_sender()
            .send(Notification::Char(CharNotification::new(char_id, packet.raw)));
    }
}
