use std::collections::HashMap;
use std::sync::Arc;

use crate::server::model::events::game_event::ScriptNpcEvent;
use crate::server::model::session::Session;
use crate::server::state::server::ServerState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NpcTimerKey {
    pub npc_id: u32,
    pub scope_instance: u8,
    pub char_id: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptTimerGuard {
    Npc { key: NpcTimerKey, revision: u64 },
    Player { id: u64, revision: u64 },
}

#[derive(Clone)]
pub struct ScriptTimerOwner {
    pub char_id: u32,
    pub account_id: u32,
    pub session: Arc<Session>,
}

impl ScriptTimerOwner {
    pub fn is_current(&self, state: &ServerState) -> bool {
        state.characters().contains_key(&self.char_id)
            && state
                .find_session(self.account_id)
                .is_some_and(|session| session.char_id == Some(self.char_id) && session.auth_code == self.session.auth_code
                    && session.user_level == self.session.user_level)
    }
}

pub struct NpcTimer {
    pub npc_name: String,
    pub elapsed_ms: u64,
    pub started_at: Option<u128>,
    pub next_label: usize,
    pub labels: Vec<(u64, u32)>,
    pub revision: u64,
    pub owner: Option<ScriptTimerOwner>,
}

impl NpcTimer {
    pub fn elapsed(&self, now: u128) -> u64 {
        self.elapsed_ms.saturating_add(
            self.started_at
                .map_or(0, |started| now.saturating_sub(started).min(u128::from(u64::MAX)) as u64),
        )
    }

    pub fn set_elapsed(&mut self, elapsed_ms: u64, now: u128) {
        self.elapsed_ms = elapsed_ms;
        self.next_label = self.labels.partition_point(|(time, _)| *time <= elapsed_ms);
        if self.started_at.is_some() {
            self.started_at = Some(now);
        }
    }
}

pub struct PlayerScriptTimer {
    pub owner: ScriptTimerOwner,
    pub label: String,
    pub event: ScriptNpcEvent,
    pub due_at: u128,
    pub queued: bool,
    pub revision: u64,
}

#[derive(Default)]
pub struct ScriptTimers {
    pub npc: HashMap<NpcTimerKey, NpcTimer>,
    pub attachments: HashMap<(u32, u8), ScriptTimerOwner>,
    pub player: HashMap<u64, PlayerScriptTimer>,
    next_id: u64,
}

impl ScriptTimers {
    pub fn next_id(&mut self) -> Result<u64, String> {
        self.next_id = self.next_id.checked_add(1).ok_or("Script timer identifiers exhausted")?;
        Ok(self.next_id)
    }

    pub fn is_valid(&self, state: &ServerState, guard: ScriptTimerGuard) -> bool {
        match guard {
            ScriptTimerGuard::Npc { key, revision } => self
                .npc
                .get(&key)
                .is_some_and(|timer| timer.revision == revision && timer.owner.as_ref().is_none_or(|owner| owner.is_current(state))),
            ScriptTimerGuard::Player { id, revision } => self
                .player
                .get(&id)
                .is_some_and(|timer| timer.revision == revision && timer.queued && timer.owner.is_current(state)),
        }
    }

    pub fn consume(&mut self, guard: ScriptTimerGuard) {
        if let ScriptTimerGuard::Player { id, revision } = guard {
            if self.player.get(&id).is_some_and(|timer| timer.revision == revision) {
                self.player.remove(&id);
            }
        }
    }

    pub fn disconnect(&mut self, char_id: u32) {
        self.player.retain(|_, timer| timer.owner.char_id != char_id);
        self.npc.retain(|key, _| key.char_id != Some(char_id));
        self.attachments.retain(|_, owner| owner.char_id != char_id);
    }
}
