use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

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
struct DuelBook {
    next_id: u32,
    duels: HashMap<u32, Duel>,
    by_char: HashMap<u32, u32>,
    invites: HashMap<u32, u32>,
}

impl DuelBook {
    fn same_duel(&self, first: u32, second: u32) -> bool {
        self.by_char.get(&first).is_some_and(|duel| self.by_char.get(&second) == Some(duel))
    }

    fn duel_of(&self, char_id: u32) -> Option<u32> {
        self.by_char.get(&char_id).copied()
    }

    fn prune(&mut self, is_online: impl Fn(u32) -> bool) {
        let offline: Vec<u32> = self.by_char.keys().copied().filter(|id| !is_online(*id)).collect();
        for char_id in offline {
            self.leave(char_id);
        }
        self.invites.retain(|invitee, _| is_online(*invitee));
    }

    fn create(&mut self, char_id: u32, limit: usize) -> Result<u32, &'static str> {
        if self.by_char.contains_key(&char_id) {
            return Err("You are already in a duel");
        }
        self.next_id += 1;
        let id = self.next_id;
        self.duels.insert(id, Duel { members: vec![char_id], limit });
        self.by_char.insert(char_id, id);
        Ok(id)
    }

    fn invite(&mut self, inviter: u32, target: u32) -> Result<(), &'static str> {
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

    fn accept(&mut self, char_id: u32) -> Result<u32, &'static str> {
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

    fn reject(&mut self, char_id: u32) -> bool {
        self.invites.remove(&char_id).is_some()
    }

    /// Returns the remaining members when the departure dissolved the duel.
    fn leave(&mut self, char_id: u32) -> Option<Vec<u32>> {
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

/// Duel memberships and invitations. Cheap to clone; every clone shares the same book,
/// and each method takes the lock only for its own duration.
#[derive(Debug, Default, Clone)]
pub struct Duels {
    book: Arc<Mutex<DuelBook>>,
}

impl Duels {
    fn book(&self) -> MutexGuard<'_, DuelBook> {
        self.book.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn same_duel(&self, first: u32, second: u32) -> bool {
        self.book().same_duel(first, second)
    }

    pub fn duel_of(&self, char_id: u32) -> Option<u32> {
        self.book().duel_of(char_id)
    }

    pub fn prune(&self, is_online: impl Fn(u32) -> bool) {
        self.book().prune(is_online)
    }

    pub fn create(&self, char_id: u32, limit: usize) -> Result<u32, &'static str> {
        self.book().create(char_id, limit)
    }

    pub fn invite(&self, inviter: u32, target: u32) -> Result<(), &'static str> {
        self.book().invite(inviter, target)
    }

    pub fn accept(&self, char_id: u32) -> Result<u32, &'static str> {
        self.book().accept(char_id)
    }

    pub fn reject(&self, char_id: u32) -> bool {
        self.book().reject(char_id)
    }

    /// Returns the remaining members when the departure dissolved the duel.
    pub fn leave(&self, char_id: u32) -> Option<Vec<u32>> {
        self.book().leave(char_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duel_lifecycle() {
        let duels = Duels::default();
        duels.create(1, 0).unwrap();
        duels.invite(1, 2).unwrap();
        duels.accept(2).unwrap();
        assert!(duels.same_duel(1, 2));
        assert_eq!(duels.leave(2), Some(vec![1]));
        assert!(!duels.same_duel(1, 2));
        assert!(duels.duel_of(1).is_none());
    }

    #[test]
    fn clones_share_one_book_across_threads() {
        let duels = Duels::default();
        let remote = duels.clone();
        std::thread::spawn(move || {
            remote.create(1, 0).unwrap();
            remote.invite(1, 2).unwrap();
            remote.accept(2).unwrap();
        })
        .join()
        .unwrap();
        assert!(duels.same_duel(1, 2));
    }
}
