use models::enums::map::MapActorType;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
use models::status_bonus::{BattleFlag, CombatTrigger};

use super::super::ScriptSkillService;
use super::super::ground::GroundKind;
use super::super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::ground_unit::GroundUnitSnapshot;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::script_combat_service;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub(crate) fn sync_ground_unit_visibility(&self, state: &ServerState, ground: &mut super::super::ground::GroundSkill, tick: u128) {
        if !ground.cast_verified || !ground.displayed || ground.active_from > tick || ground.expires_at <= tick {
            return;
        }
        let view = if ground.kind.trap() && ground.triggered && ground.capture.is_none() {
            140
        } else if ground.portal.as_ref().is_some_and(|portal| portal.ready) {
            128
        } else {
            ground.kind.view_id()
        };
        for cell in ground.cells.iter_mut().filter(|cell| cell.id != 0 && cell.remaining_hits > 0) {
            let current = state
                .characters()
                .values()
                .filter(|character| {
                    character.loaded_from_client_side
                        && !state.pending_character_logouts.contains_key(&character.char_id)
                        && character.current_map_name() == &ground.map
                        && character.current_map_instance() == ground.instance
                        && crate::server::model::path::manhattan_distance(character.x, character.y, cell.x, cell.y)
                            <= crate::server::PLAYER_FOV
                })
                .filter_map(|character| {
                    state
                        .character_logins
                        .get(&character.char_id)
                        .map(|owner| (character.char_id, std::sync::Arc::downgrade(&owner.session)))
                })
                .collect::<std::collections::HashMap<_, _>>();
            for id in cell.observers.keys().filter(|id| !current.contains_key(id)) {
                if state.get_character(*id).is_some_and(|character| character.loaded_from_client_side) {
                    let mut packet = 0x0120_u16.to_le_bytes().to_vec();
                    packet.extend_from_slice(&cell.id.to_le_bytes());
                    self.queue_notification(crate::server::model::events::client_notification::Notification::Char(
                        crate::server::model::events::client_notification::CharNotification::new(*id, packet),
                    ));
                }
            }
            for (id, session) in &current {
                if cell
                    .observers
                    .get(id)
                    .is_none_or(|previous| !std::sync::Weak::ptr_eq(previous, session))
                {
                    let packet = if ground.kind == GroundKind::Graffiti {
                        Self::graffiti_entry_packet(cell.id, ground.source_id, cell.x, cell.y, &ground.message)
                    } else {
                        Self::ground_entry_packet_for(
                            self.configuration.packetver(),
                            cell.id,
                            ground.source_id,
                            cell.x,
                            cell.y,
                            ground.level,
                            view,
                        )
                    };
                    self.queue_notification(crate::server::model::events::client_notification::Notification::Char(
                        crate::server::model::events::client_notification::CharNotification::new(*id, packet),
                    ));
                }
            }
            cell.observers = current;
        }
    }

    pub(crate) fn refresh_ground_unit_snapshot(&self, state: &mut ServerState, map: &MapInstanceKey, id: u32, tick: u128) {
        let Ok(grounds) = self.ground_skills.lock() else {
            return;
        };
        let unit = grounds
            .iter()
            .find(|ground| {
                ground.kind.trap()
                    && ground.cast_verified
                    && ground.active_from <= tick
                    && ground.expires_at > tick
                    && ground.map == *map.map_name()
                    && ground.instance == map.map_instance()
                    && ground.cells.iter().any(|cell| cell.id == id && cell.remaining_hits > 0)
            })
            .map(|ground| {
                let cell = ground.cells.iter().find(|cell| cell.id == id).unwrap();
                GroundUnitSnapshot {
                    id,
                    skill_id: ground.skill_id,
                    map: map.clone(),
                    x: cell.x,
                    y: cell.y,
                    hp: cell.remaining_hits,
                    expires_at: ground.expires_at,
                    used: ground.triggered && ground.capture.is_none(),
                    view_id: if ground.triggered && ground.capture.is_none() {
                        140
                    } else {
                        ground.kind.view_id()
                    },
                }
            });
        if let Some(unit) = unit {
            state.ground_units.insert(id, unit);
        } else {
            state.ground_units.remove(&id);
        }
    }

    pub(crate) fn sync_ground_unit_snapshots(&self, state: &mut ServerState, tick: u128) {
        let Ok(grounds) = self.ground_skills.lock() else {
            return;
        };
        state.ground_units = grounds
            .iter()
            .filter(|ground| ground.kind.trap() && ground.cast_verified && ground.active_from <= tick && ground.expires_at > tick)
            .flat_map(|ground| {
                ground
                    .cells
                    .iter()
                    .filter(|cell| cell.id != 0 && cell.remaining_hits > 0)
                    .map(|cell| GroundUnitSnapshot {
                        id: cell.id,
                        skill_id: ground.skill_id,
                        map: MapInstanceKey::new(ground.map.clone(), ground.instance),
                        x: cell.x,
                        y: cell.y,
                        hp: cell.remaining_hits,
                        expires_at: ground.expires_at,
                        used: ground.triggered && ground.capture.is_none(),
                        view_id: if ground.triggered && ground.capture.is_none() {
                            140
                        } else {
                            ground.kind.view_id()
                        },
                    })
            })
            .map(|unit| (unit.id, unit))
            .collect();
    }

    pub(crate) fn apply_ground_unit_damage(
        &self,
        server: &Server,
        state: &ServerState,
        map: &MapInstanceKey,
        mut damage: Damage,
        tick: u128,
    ) -> bool {
        let Ok(mut grounds) = self.ground_skills.lock() else {
            return false;
        };
        let Some(ground) = grounds.iter_mut().find(|ground| {
            ground.kind.trap()
                && ground.map == *map.map_name()
                && ground.instance == map.map_instance()
                && ground.cells.iter().any(|cell| cell.id == damage.target_id)
        }) else {
            return false;
        };
        if !ground.cast_verified || ground.active_from > tick || ground.expires_at <= tick || !damage.matches_notification_map(map) {
            return true;
        }
        let target = models::status::StatusSnapshot::new_for_skill_unit(true);
        let flags = state.map_flags(map);
        crate::server::service::map_flag_service::apply_map_skill_damage(&flags, &mut damage, &target);
        let allowed = self.configuration.config().game.skill_units.damage_sources & MapActorType::from(damage.source_kind).as_flag() != 0;
        let amount = if allowed && damage.landed {
            crate::server::service::map_flag_service::apply_map_combat_damage(
                &flags,
                &self.configuration.config().game,
                damage.damage,
                damage.skill_id,
                damage.battle_flags,
            )
        } else {
            0
        };
        let cell = ground.cells.iter_mut().find(|cell| cell.id == damage.target_id).unwrap();
        let admitted = amount.min(u32::from(cell.remaining_hits));
        cell.remaining_hits = cell.remaining_hits.saturating_sub(admitted as u16);
        if cell.remaining_hits == 0 {
            ground.recovery_item = None;
            ground.triggered = true;
            ground.expires_at = tick;
            self.release_trap_capture(server, ground);
        }
        damage.notify_admitted(
            &self.client_notification_sender,
            i64::from(admitted),
            self.configuration.packetver(),
        );
        drop(grounds);
        if damage.landed && damage.damage > 0 && damage.skill_id != 0 {
            if let Some(metadata) = SkillMetadata::find(damage.skill_id) {
                let cells = metadata
                    .knockback
                    .as_ref()
                    .and_then(|knockback| knockback.value(damage.skill_level, "Amount"))
                    .unwrap_or(0)
                    .max(0) as u16;
                if cells > 0 && metadata.name != "KN_BOWLINGBASH" {
                    if let Some(source) = state.map_item_snapshot(damage.attacker_id, map.map_name(), map.map_instance()) {
                        self.knockback_ground_unit(map, damage.target_id, source.x(), source.y(), cells, state, tick);
                    }
                }
            }
        }
        if admitted > 0 && damage.proc_depth < 8 {
            if damage.battle_flags & BattleFlag::Weapon.as_flag() != 0 {
                script_combat_service::emit(
                    server,
                    damage.attacker_id,
                    damage.target_id,
                    CombatTrigger::Attack,
                    damage.battle_flags,
                    damage.skill_id,
                    admitted,
                );
            }
        }
        true
    }

    pub(crate) fn knockback_ground_unit(
        &self,
        map: &MapInstanceKey,
        id: u32,
        source_x: u16,
        source_y: u16,
        cells: u16,
        state: &ServerState,
        tick: u128,
    ) {
        let Some(instance) = state.get_map_instance(map.map_name(), map.map_instance()) else {
            return;
        };
        let map_state = instance.state();
        let Ok(mut grounds) = self.ground_skills.lock() else {
            return;
        };
        let Some(ground) = grounds.iter_mut().find(|ground| {
            ground.kind.trap()
                && ground.cast_verified
                && ground.active_from <= tick
                && ground.expires_at > tick
                && ground.map == *map.map_name()
                && ground.instance == map.map_instance()
                && ground.cells.iter().any(|cell| cell.id == id && cell.remaining_hits > 0)
        }) else {
            return;
        };
        if SkillMetadata::find(ground.skill_id)
            .and_then(|metadata| metadata.unit.as_ref()?.get("Flag")?.get("NoKnockback")?.as_bool())
            .unwrap_or(false)
        {
            return;
        }
        let cell = ground.cells.iter().find(|cell| cell.id == id).unwrap();
        let (x, y) = Self::knockback_destination(cell.x, cell.y, source_x, source_y, cells, |x, y| {
            x < instance.x_size()
                && y < instance.y_size()
                && map_state.cells()[y as usize * instance.x_size() as usize + x as usize]
                    & models::enums::cell::CellType::Walkable.as_flag()
                    != 0
        });
        if (x, y) == (cell.x, cell.y) {
            return;
        }
        let mut remove = 0x0120_u16.to_le_bytes().to_vec();
        remove.extend_from_slice(&id.to_le_bytes());
        self.notify_ground_cell(ground, cell, remove);
        let cell = ground.cells.iter_mut().find(|cell| cell.id == id).unwrap();
        cell.x = x;
        cell.y = y;
        let cell = ground.cells.iter().find(|cell| cell.id == id).unwrap();
        self.notify_ground_cell(
            ground,
            cell,
            Self::ground_entry_packet_for(
                self.configuration.packetver(),
                id,
                ground.source_id,
                x,
                y,
                ground.level,
                if ground.triggered && ground.capture.is_none() {
                    140
                } else {
                    ground.kind.view_id()
                },
            ),
        );
    }

    pub(crate) fn spend_ground_unit(&self, map: &MapInstanceKey, id: u32, tick: u128) {
        let Ok(mut grounds) = self.ground_skills.lock() else {
            return;
        };
        if let Some(ground) = grounds.iter_mut().find(|ground| {
            ground.map == *map.map_name()
                && ground.instance == map.map_instance()
                && ground.cast_verified
                && ground.expires_at > tick
                && ground.cells.iter().any(|cell| cell.id == id && cell.remaining_hits > 0)
                && matches!(
                    ground.kind,
                    GroundKind::ClaymoreTrap
                        | GroundKind::LandMine
                        | GroundKind::BlastMine
                        | GroundKind::Shockwave
                        | GroundKind::Sandman
                        | GroundKind::Flasher
                        | GroundKind::FreezingTrap
                )
        }) {
            ground.triggered = true;
            ground.recovery_item = None;
            ground.expires_at = tick + 1500;
            self.change_trap_view(ground, 140);
        }
    }
}
