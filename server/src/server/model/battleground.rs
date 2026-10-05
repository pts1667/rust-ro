use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::server::model::battleground_queue::BgQueues;

pub const MAX_BG_MEMBERS: usize = 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BgPoint {
    pub map: String,
    pub x: u16,
    pub y: u16,
}

/// Per-character battleground tracking, used to relay position and health to teammates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BgMember {
    pub last_x: u16,
    pub last_y: u16,
    pub last_hp: u32,
    pub seen_on_battleground_map: bool,
    pub entry_point: Option<BgPoint>,
}

impl Default for BgMember {
    fn default() -> Self {
        Self::joined_at((0, 0), None)
    }
}

impl BgMember {
    pub fn joined_at(position: (u16, u16), entry_point: Option<BgPoint>) -> Self {
        Self {
            last_x: position.0,
            last_y: position.1,
            last_hp: u32::MAX,
            seen_on_battleground_map: false,
            entry_point,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BgTeam {
    pub id: u32,
    pub cemetery: Option<BgPoint>,
    pub quit_event: String,
    pub die_event: String,
    pub active_event: String,
    pub deserter_seconds: u32,
}

#[derive(Debug, Default)]
struct TeamBook {
    teams: HashMap<u32, BgTeam>,
    scores: HashMap<String, (u16, u16)>,
}

/// Battleground team definitions, scores and queues. Membership lives on the characters (`Character::bg_id`).
/// Cheap to clone; every clone shares the same data, locked per call.
#[derive(Debug, Default, Clone)]
pub struct Battlegrounds {
    book: Arc<Mutex<TeamBook>>,
    queues: Arc<Mutex<BgQueues>>,
}

impl Battlegrounds {
    fn book(&self) -> MutexGuard<'_, TeamBook> {
        self.book.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// The closure must not call back into anything that locks the queues.
    pub fn with_queues<R>(&self, action: impl FnOnce(&mut BgQueues) -> R) -> R {
        action(&mut self.queues.lock().unwrap_or_else(|poisoned| poisoned.into_inner()))
    }

    pub fn create(&self, cemetery: Option<BgPoint>, quit_event: String, die_event: String, active_event: String, deserter_seconds: u32) -> u32 {
        let mut book = self.book();
        let id = (1..).find(|id| !book.teams.contains_key(id)).unwrap_or(1);
        book.teams.insert(id, BgTeam { id, cemetery, quit_event, die_event, active_event, deserter_seconds });
        id
    }

    pub fn team(&self, id: u32) -> Option<BgTeam> {
        self.book().teams.get(&id).cloned()
    }

    pub fn remove_team(&self, id: u32) -> bool {
        self.book().teams.remove(&id).is_some()
    }

    pub fn set_cemetery_position(&self, id: u32, x: u16, y: u16) {
        if let Some(cemetery) = self.book().teams.get_mut(&id).and_then(|team| team.cemetery.as_mut()) {
            cemetery.x = x;
            cemetery.y = y;
        }
    }

    pub fn score(&self, map: &str) -> (u16, u16) {
        self.book().scores.get(map).copied().unwrap_or_default()
    }

    pub fn set_score(&self, map: &str, first: u16, second: u16) {
        self.book().scores.insert(map.to_string(), (first, second));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teams_reuse_the_lowest_free_id() {
        let bg = Battlegrounds::default();
        let first = bg.create(None, String::new(), String::new(), String::new(), 0);
        let second = bg.create(None, String::new(), String::new(), String::new(), 0);
        assert_eq!((first, second), (1, 2));
        assert!(bg.remove_team(first));
        assert!(!bg.remove_team(first));
        assert_eq!(bg.create(None, String::new(), String::new(), String::new(), 0), 1);
    }

    #[test]
    fn cemetery_moves_only_for_teams_that_have_one() {
        let bg = Battlegrounds::default();
        let with = bg.create(Some(BgPoint { map: "prontera".into(), x: 1, y: 1 }), String::new(), String::new(), String::new(), 0);
        let without = bg.create(None, String::new(), String::new(), String::new(), 0);
        bg.set_cemetery_position(with, 5, 6);
        bg.set_cemetery_position(without, 5, 6);
        let cemetery = bg.team(with).unwrap().cemetery.unwrap();
        assert_eq!((cemetery.x, cemetery.y), (5, 6));
        assert!(bg.team(without).unwrap().cemetery.is_none());
    }

    #[test]
    fn scores_are_per_map_and_clones_share_them() {
        let bg = Battlegrounds::default();
        let remote = bg.clone();
        remote.set_score("bat_b01", 3, 4);
        assert_eq!(bg.score("bat_b01"), (3, 4));
        assert_eq!(bg.score("bat_b02"), (0, 0));
    }
}
