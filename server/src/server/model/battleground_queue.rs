use std::collections::HashSet;
use std::sync::OnceLock;

use serde::Deserialize;

pub const ADMISSION_WINDOW_MS: u64 = 20_000;
pub const REQUEUE_INTERVAL_MS: u64 = 10_000;
pub const APPLY_DELAY_MS: i32 = 60_000;

#[derive(Debug, Clone, Deserialize)]
pub struct BgTeamDefinition {
    pub x: u16,
    pub y: u16,
    pub death_event: String,
    pub quit_event: String,
    pub active_event: String,
    pub variable: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BgMapDefinition {
    pub map: String,
    pub start_event: String,
    pub team_a: BgTeamDefinition,
    pub team_b: BgTeamDefinition,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BgType {
    pub id: u32,
    pub name: String,
    pub required_players: usize,
    pub max_players: usize,
    pub min_level: u32,
    pub max_level: u32,
    pub deserter_seconds: u32,
    pub start_delay_seconds: u32,
    pub solo: bool,
    pub party: bool,
    pub guild: bool,
    pub job_restrictions: Vec<u32>,
    pub maps: Vec<BgMapDefinition>,
}

pub fn battleground_types() -> &'static [BgType] {
    static TYPES: OnceLock<Vec<BgType>> = OnceLock::new();
    TYPES.get_or_init(|| serde_json::from_str(include_str!("../script/battlegrounds.json")).expect("Embedded battleground catalog is invalid"))
}

pub fn battleground_type_by_name(name: &str) -> Option<&'static BgType> {
    battleground_types().iter().find(|bg| bg.name == name)
}

pub fn battleground_type(id: u32) -> Option<&'static BgType> {
    battleground_types().iter().find(|bg| bg.id == id)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BattlegroundQueueAction {
    Apply { kind: u16, name: String },
    Cancel(String),
    Reply { accept: bool },
    Number(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BattlegroundQueueCommand {
    pub char_id: u32,
    pub action: BattlegroundQueueAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueState {
    Setup,
    SetupDelay,
    Active,
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueSide {
    A,
    B,
}

#[derive(Debug, Clone)]
pub struct BgQueue {
    pub queue_id: u32,
    pub bg_id: u32,
    pub team_a: Vec<u32>,
    pub team_b: Vec<u32>,
    pub accepted: usize,
    pub state: QueueState,
    pub expire_at: Option<u64>,
    pub start_at: Option<u64>,
    pub requeue_at: Option<u64>,
    pub map_index: Option<usize>,
}

impl BgQueue {
    pub fn new(queue_id: u32, bg_id: u32) -> Self {
        Self {
            queue_id,
            bg_id,
            team_a: vec![],
            team_b: vec![],
            accepted: 0,
            state: QueueState::Setup,
            expire_at: None,
            start_at: None,
            requeue_at: None,
            map_index: None,
        }
    }

    pub fn side_mut(&mut self, side: QueueSide) -> &mut Vec<u32> {
        match side {
            QueueSide::A => &mut self.team_a,
            QueueSide::B => &mut self.team_b,
        }
    }

    pub fn side_of(&self, char_id: u32) -> Option<QueueSide> {
        if self.team_a.contains(&char_id) {
            Some(QueueSide::A)
        } else if self.team_b.contains(&char_id) {
            Some(QueueSide::B)
        } else {
            None
        }
    }

    pub fn members(&self) -> impl Iterator<Item = u32> + '_ {
        self.team_a.iter().chain(self.team_b.iter()).copied()
    }

    pub fn len(&self) -> usize {
        self.team_a.len() + self.team_b.len()
    }

    /// Picks the side a group of `size` players joins, or None when neither side has room.
    pub fn choose_side(&self, size: usize, bg: &BgType, coin_toss_b: bool) -> Option<QueueSide> {
        if self.team_a.len() + size > bg.max_players && self.team_b.len() + size > bg.max_players {
            return None;
        }
        let mut side = if coin_toss_b { QueueSide::B } else { QueueSide::A };
        let (own, other) = match side {
            QueueSide::A => (self.team_a.len(), self.team_b.len()),
            QueueSide::B => (self.team_b.len(), self.team_a.len()),
        };
        let flip = |side| if side == QueueSide::A { QueueSide::B } else { QueueSide::A };
        if self.state == QueueState::Active {
            if own > other {
                side = flip(side);
            }
        } else if own + size > bg.required_players {
            side = flip(side);
        }
        Some(side)
    }

    /// Resets the queue for the next group; the state is only reset when `ended`.
    pub fn clear(&mut self, ended: bool) {
        self.expire_at = None;
        self.start_at = None;
        self.requeue_at = None;
        if ended {
            self.map_index = None;
            self.team_a.clear();
            self.team_b.clear();
            self.accepted = 0;
            self.state = QueueState::Setup;
        }
    }
}

#[derive(Debug, Default)]
pub struct BgQueues {
    pub queues: Vec<BgQueue>,
    pub reserved_maps: HashSet<String>,
}

impl BgQueues {
    pub fn ensure_created(&mut self) {
        if self.queues.is_empty() {
            self.queues = battleground_types().iter().zip(1..).map(|(bg, id)| BgQueue::new(id, bg.id)).collect();
        }
    }

    pub fn queue_of(&self, char_id: u32) -> Option<usize> {
        self.queues.iter().position(|queue| queue.side_of(char_id).is_some())
    }

    pub fn find(&self, queue_id: u32) -> Option<usize> {
        self.queues.iter().position(|queue| queue.queue_id == queue_id)
    }

    pub fn reserve_free_map(&mut self, bg: &BgType) -> Option<usize> {
        let index = bg.maps.iter().position(|map| !self.reserved_maps.contains(&map.map))?;
        self.reserved_maps.insert(bg.maps[index].map.clone());
        Some(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_catalog_lists_the_stock_battlegrounds() {
        assert_eq!(battleground_types().len(), 3);
        let flavius = battleground_type_by_name("Flavius").unwrap();
        assert_eq!((flavius.required_players, flavius.max_players, flavius.min_level), (6, 15, 80));
        assert_eq!(flavius.maps[0].team_a.variable, "$@FlaviusBG1_id1");
        assert!(flavius.job_restrictions.contains(&23));
    }

    #[test]
    fn a_group_fills_the_other_side_once_its_coin_toss_side_reaches_the_required_count() {
        let bg = battleground_type_by_name("Flavius").unwrap();
        let mut queue = BgQueue::new(1, bg.id);
        queue.team_a = (0..5).collect();
        assert_eq!(queue.choose_side(2, bg, false), Some(QueueSide::B));
        assert_eq!(queue.choose_side(1, bg, false), Some(QueueSide::A));
        queue.team_a = (0..15).collect();
        queue.team_b = (0..15).collect();
        assert_eq!(queue.choose_side(1, bg, true), None);
    }

    #[test]
    fn an_active_queue_balances_towards_the_smaller_side() {
        let bg = battleground_type_by_name("Flavius").unwrap();
        let mut queue = BgQueue::new(1, bg.id);
        queue.state = QueueState::Active;
        queue.team_a = (0..8).collect();
        queue.team_b = (0..5).collect();
        assert_eq!(queue.choose_side(1, bg, false), Some(QueueSide::B));
        assert_eq!(queue.choose_side(1, bg, true), Some(QueueSide::B));
    }

    #[test]
    fn reserving_maps_hands_out_each_arena_once() {
        let bg = battleground_type_by_name("Flavius").unwrap();
        let mut queues = BgQueues::default();
        assert_eq!(queues.reserve_free_map(bg), Some(0));
        assert_eq!(queues.reserve_free_map(bg), None);
    }
}
