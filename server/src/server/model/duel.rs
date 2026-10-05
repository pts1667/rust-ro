use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use std::thread;

use dashmap::DashMap;

use crate::server::model::events::game_event::GameEvent;
use crate::server::model::tasks_queue::TasksQueue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DuelAction {
    Create,
    Invite,
    Accept,
    Reject,
    Leave,
    Killer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DuelCommand {
    pub char_id: u32,
    pub action: DuelAction,
    pub argument: String,
}

#[derive(Debug, Default)]
struct Duel {
    members: Vec<u32>,
    limit: usize,
}

#[derive(Debug, Default)]
pub struct Duels {
    next_id: u32,
    duels: HashMap<u32, Duel>,
    by_char: HashMap<u32, u32>,
    invites: HashMap<u32, u32>,
}

impl Duels {
    pub fn same_duel(&self, first: u32, second: u32) -> bool {
        self.by_char.get(&first).is_some_and(|duel| self.by_char.get(&second) == Some(duel))
    }

    pub fn duel_of(&self, char_id: u32) -> Option<u32> {
        self.by_char.get(&char_id).copied()
    }

    fn members_of(&self, duel: u32) -> Vec<u32> {
        self.duels.get(&duel).map(|entry| entry.members.clone()).unwrap_or_default()
    }

    pub fn prune(&mut self, is_online: impl Fn(u32) -> bool) {
        let offline: Vec<u32> = self.by_char.keys().copied().filter(|id| !is_online(*id)).collect();
        for char_id in offline {
            self.leave(char_id);
        }
        self.invites.retain(|invitee, _| is_online(*invitee));
    }

    pub fn create(&mut self, char_id: u32, limit: usize) -> Result<u32, &'static str> {
        if self.by_char.contains_key(&char_id) {
            return Err("You are already in a duel");
        }
        self.next_id += 1;
        let id = self.next_id;
        self.duels.insert(id, Duel { members: vec![char_id], limit });
        self.by_char.insert(char_id, id);
        Ok(id)
    }

    pub fn invite(&mut self, inviter: u32, target: u32) -> Result<(), &'static str> {
        let duel = *self.by_char.get(&inviter).ok_or("You are not in a duel")?;
        if self.by_char.contains_key(&target) {
            return Err("The target is already in a duel");
        }
        let entry = &self.duels[&duel];
        if entry.limit != 0 && entry.members.len() >= entry.limit {
            return Err("The duel is full");
        }
        self.invites.insert(target, duel);
        Ok(())
    }

    pub fn accept(&mut self, char_id: u32) -> Result<u32, &'static str> {
        if self.by_char.contains_key(&char_id) {
            return Err("You are already in a duel");
        }
        let duel = self.invites.remove(&char_id).ok_or("You have no pending duel invitation")?;
        let entry = self.duels.get_mut(&duel).ok_or("The duel no longer exists")?;
        if entry.limit != 0 && entry.members.len() >= entry.limit {
            return Err("The duel is full");
        }
        entry.members.push(char_id);
        self.by_char.insert(char_id, duel);
        Ok(duel)
    }

    pub fn reject(&mut self, char_id: u32) -> bool {
        self.invites.remove(&char_id).is_some()
    }

    /// Returns the remaining members when the departure dissolved the duel.
    pub fn leave(&mut self, char_id: u32) -> Option<Vec<u32>> {
        let duel = self.by_char.remove(&char_id)?;
        let entry = self.duels.get_mut(&duel)?;
        entry.members.retain(|member| *member != char_id);
        if entry.members.len() > 1 {
            return None;
        }
        let remaining = self.duels.remove(&duel).map(|entry| entry.members).unwrap_or_default();
        for member in &remaining {
            self.by_char.remove(member);
        }
        self.invites.retain(|_, invited_to| *invited_to != duel);
        Some(remaining)
    }
}

/// Read replica of the duel memberships, written only by the duel actor.
/// Readers may observe a membership change one actor message late.
#[derive(Clone, Default)]
pub struct DuelDirectory {
    by_char: Arc<DashMap<u32, u32>>,
}

impl DuelDirectory {
    pub fn duel_of(&self, char_id: u32) -> Option<u32> {
        self.by_char.get(&char_id).map(|duel| *duel)
    }

    pub fn same_duel(&self, first: u32, second: u32) -> bool {
        match (self.duel_of(first), self.duel_of(second)) {
            (Some(first_duel), Some(second_duel)) => first_duel == second_duel,
            _ => false,
        }
    }

    fn publish(&self, duels: &Duels) {
        for (char_id, duel) in &duels.by_char {
            self.by_char.insert(*char_id, *duel);
        }
        self.by_char.retain(|char_id, _| duels.by_char.contains_key(char_id));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DuelRequest {
    Create { char_id: u32, limit: usize },
    Invite { inviter: u32, target: u32 },
    Accept { char_id: u32 },
    Reject { char_id: u32, silent: bool },
    Leave { char_id: u32, explicit: bool },
    Prune { online: Vec<u32> },
}

/// Result of a [`DuelRequest`], delivered to the game loop which owns the client notifications.
#[derive(Debug, Clone, PartialEq)]
pub enum DuelOutcome {
    Created { char_id: u32, result: Result<u32, &'static str> },
    Invited { inviter: u32, target: u32, result: Result<(), &'static str> },
    Accepted { char_id: u32, result: Result<Vec<u32>, &'static str> },
    Rejected { char_id: u32, had_invitation: bool, silent: bool },
    Left { char_id: u32, explicit: bool, was_member: bool, remaining: Vec<u32> },
}

impl Duels {
    pub fn apply(&mut self, request: DuelRequest) -> Option<DuelOutcome> {
        match request {
            DuelRequest::Create { char_id, limit } => Some(DuelOutcome::Created { char_id, result: self.create(char_id, limit) }),
            DuelRequest::Invite { inviter, target } => Some(DuelOutcome::Invited { inviter, target, result: self.invite(inviter, target) }),
            DuelRequest::Accept { char_id } => {
                let result = self.accept(char_id).map(|duel| self.members_of(duel));
                Some(DuelOutcome::Accepted { char_id, result })
            }
            DuelRequest::Reject { char_id, silent } => Some(DuelOutcome::Rejected { char_id, had_invitation: self.reject(char_id), silent }),
            DuelRequest::Leave { char_id, explicit } => {
                let was_member = self.by_char.contains_key(&char_id);
                let remaining = self.leave(char_id).unwrap_or_default();
                Some(DuelOutcome::Left { char_id, explicit, was_member, remaining })
            }
            DuelRequest::Prune { online } => {
                self.prune(|id| online.contains(&id));
                None
            }
        }
    }
}

/// Owns [`Duels`] on a dedicated thread. The thread ends once every handle has been dropped.
#[derive(Clone)]
pub struct DuelActor {
    sender: Sender<DuelRequest>,
}

impl DuelActor {
    pub fn spawn(directory: DuelDirectory, outcomes: Arc<TasksQueue<GameEvent>>) -> Self {
        let (sender, receiver) = channel::<DuelRequest>();
        thread::Builder::new()
            .name("duel-actor".to_string())
            .spawn(move || {
                let mut duels = Duels::default();
                while let Ok(request) = receiver.recv() {
                    let outcome = duels.apply(request);
                    directory.publish(&duels);
                    if let Some(outcome) = outcome {
                        outcomes.add_to_first_index(GameEvent::DuelOutcome(outcome));
                    }
                }
            })
            .expect("Failed to spawn duel actor thread");
        Self { sender }
    }

    pub fn send(&self, request: DuelRequest) {
        if self.sender.send(request).is_err() {
            log::error!("Duel actor is not running");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::*;

    #[test]
    fn duel_lifecycle() {
        let mut duels = Duels::default();
        duels.create(1, 0).unwrap();
        duels.invite(1, 2).unwrap();
        duels.accept(2).unwrap();
        assert!(duels.same_duel(1, 2));
        assert_eq!(duels.leave(2), Some(vec![1]));
        assert!(!duels.same_duel(1, 2));
        assert!(duels.duel_of(1).is_none());
    }

    #[test]
    fn apply_reports_remaining_members_and_membership() {
        let mut duels = Duels::default();
        duels.apply(DuelRequest::Create { char_id: 1, limit: 0 });
        duels.apply(DuelRequest::Invite { inviter: 1, target: 2 });
        let accepted = duels.apply(DuelRequest::Accept { char_id: 2 });
        assert_eq!(accepted, Some(DuelOutcome::Accepted { char_id: 2, result: Ok(vec![1, 2]) }));
        let left = duels.apply(DuelRequest::Leave { char_id: 2, explicit: true });
        assert_eq!(left, Some(DuelOutcome::Left { char_id: 2, explicit: true, was_member: true, remaining: vec![1] }));
        let stranger = duels.apply(DuelRequest::Leave { char_id: 9, explicit: true });
        assert_eq!(stranger, Some(DuelOutcome::Left { char_id: 9, explicit: true, was_member: false, remaining: vec![] }));
    }

    #[test]
    fn actor_publishes_directory_before_reporting_outcome() {
        let directory = DuelDirectory::default();
        let outcomes = Arc::new(TasksQueue::new());
        let actor = DuelActor::spawn(directory.clone(), outcomes.clone());
        actor.send(DuelRequest::Create { char_id: 1, limit: 0 });
        actor.send(DuelRequest::Invite { inviter: 1, target: 2 });
        actor.send(DuelRequest::Accept { char_id: 2 });
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut received = vec![];
        while received.len() < 3 && Instant::now() < deadline {
            if let Some(batch) = outcomes.pop() {
                received.extend(batch);
            }
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(received.len(), 3);
        assert!(directory.same_duel(1, 2));
        actor.send(DuelRequest::Leave { char_id: 2, explicit: true });
        let deadline = Instant::now() + Duration::from_secs(5);
        while directory.duel_of(1).is_some() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(5));
        }
        assert!(!directory.same_duel(1, 2));
        assert!(directory.duel_of(1).is_none());
    }
}
