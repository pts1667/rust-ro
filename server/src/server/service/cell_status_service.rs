use std::sync::atomic::{AtomicU64, Ordering};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use crate::server::Server;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::server::ServerState;
use crate::util::tick::get_tick;

const TICK_INTERVAL_MS: u64 = 500;
const INFINITE_DURATION: i32 = -1;

static NEXT_TICK: AtomicU64 = AtomicU64::new(0);

impl ServerState {
    pub fn cell_has(&self, key: &crate::server::model::map_instance::MapInstanceKey, x: u16, y: u16, cell: CellType) -> bool {
        self.get_map_instance(key.map_name(), key.map_instance()).is_some_and(|instance| {
            let map = instance.state();
            map.cells()
                .get(map.get_cell_index_of(x, y))
                .is_some_and(|flags| flags & cell.as_flag() != 0)
        })
    }
}

impl Server {
    /// Mirrors rathena's `pc_cell_basilica`: standing on a basilica cell grants the Basilica status, leaving it removes it.
    pub(crate) fn tick_cell_statuses(&self, state: &mut ServerState, tick: u128) {
        if (tick as u64) < NEXT_TICK.load(Ordering::Relaxed) {
            return;
        }
        NEXT_TICK.store(tick as u64 + TICK_INTERVAL_MS, Ordering::Relaxed);
        let basilica = CellType::Basilica.as_flag();
        let changes: Vec<(u32, bool)> = state
            .characters()
            .values()
            .filter_map(|character| {
                let instance = state.get_map_instance(character.current_map_name(), character.current_map_instance())?;
                let map = instance.state();
                let cell = map.cells().get(map.get_cell_index_of(character.x(), character.y())).copied()?;
                let on_cell = cell & basilica != 0;
                let granted_by_cell = self.cell_basilica().contains(&character.char_id);
                let has_status = character.status.has_status_change(StatusChangeKind::Basilica);
                match (on_cell, granted_by_cell, has_status) {
                    (true, false, false) | (false, true, _) => Some((character.char_id, on_cell)),
                    _ => None,
                }
            })
            .collect();
        let sender = self.server_service().notification_sender();
        for (char_id, entered) in changes {
            if entered {
                self.cell_basilica().insert(char_id);
            } else {
                self.cell_basilica().remove(&char_id);
            }
            self.with_character(state, char_id, |character| {
                if entered {
                    let request = StatusChangeRequest::guaranteed(StatusChangeKind::Basilica, INFINITE_DURATION, 0);
                    if let Err(error) = StatusEffectService::start(self, character, request, get_tick(), &sender) {
                        warn!("Basilica cell status failed: {error}");
                    }
                } else {
                    StatusEffectService::end(self, character, Some(StatusChangeKind::Basilica), get_tick(), &sender);
                }
            });
        }
    }
}
