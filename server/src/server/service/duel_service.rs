use crate::server::Server;
use crate::server::model::duel::{DuelAction, DuelCommand};
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;
use crate::util::packet::playerchat_packet;

impl Server {
    pub(crate) fn handle_duel_command(&self, state: &mut ServerState, command: DuelCommand) {
        let online: Vec<u32> = state.characters().keys().copied().collect();
        state.duels.prune(|id| online.contains(&id));
        let char_id = command.char_id;
        let Some(name) = state.characters().get(&char_id).map(|character| character.name.clone()) else {
            return;
        };
        match command.action {
            DuelAction::Create => {
                let limit = command.argument.parse::<usize>().unwrap_or(0);
                match state.duels.create(char_id, limit) {
                    Ok(_) => {
                        self.duel_message(char_id, "You have created a duel. Use @invite <name> to invite players.");
                        self.notify_map_property(state, char_id);
                    }
                    Err(error) => self.duel_message(char_id, error),
                }
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
                match state.duels.invite(char_id, target) {
                    Ok(()) => {
                        self.duel_message(char_id, "Duel invitation sent");
                        self.duel_message(target, &format!("{name} invites you to a duel. Use @accept or @reject."));
                    }
                    Err(error) => self.duel_message(char_id, error),
                }
            }
            DuelAction::Accept => match state.duels.accept(char_id) {
                Ok(duel) => {
                    self.notify_map_property(state, char_id);
                    for member in state.characters().keys().copied().filter(|id| state.duels.duel_of(*id) == Some(duel)).collect::<Vec<_>>() {
                        self.duel_message(member, &format!("{name} joined the duel"));
                    }
                }
                Err(error) => self.duel_message(char_id, error),
            },
            DuelAction::Reject => {
                if state.duels.reject(char_id) {
                    self.duel_message(char_id, "Duel invitation rejected");
                } else {
                    self.duel_message(char_id, "You have no pending duel invitation");
                }
            }
            DuelAction::Killer => {
                let Some(character) = state.characters_mut().get_mut(&char_id) else {
                    return;
                };
                character.killer = !character.killer;
                let text = if character.killer { "You are now a killer: you can attack any player" } else { "You are no longer a killer" };
                self.duel_message(char_id, text);
            }
            DuelAction::Leave => {
                if state.duels.duel_of(char_id).is_none() {
                    return self.duel_message(char_id, "You are not in a duel");
                }
                self.duel_message(char_id, "You left the duel");
                self.leave_duel(state, char_id);
            }
        }
    }

    /// Removes a character from its duel, refreshing the client cursor for everyone whose duel ended.
    pub(crate) fn leave_duel(&self, state: &mut ServerState, char_id: u32) {
        let remaining = state.duels.leave(char_id);
        self.notify_map_property(state, char_id);
        for member in remaining.unwrap_or_default() {
            self.duel_message(member, "The duel has ended");
            self.notify_map_property(state, member);
        }
    }

    fn duel_message(&self, char_id: u32, text: &str) {
        let packet = playerchat_packet(GlobalConfigService::instance().packetver(), text);
        let _ = self
            .server_service()
            .notification_sender()
            .send(Notification::Char(CharNotification::new(char_id, packet.raw)));
    }
}
