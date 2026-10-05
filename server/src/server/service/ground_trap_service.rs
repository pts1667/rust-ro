use models::enums::bonus::BonusType;
use models::status_change::StatusChangeKind;

use crate::server::Server;

fn trap_destination_walkable(state: &ServerState, request: &GroundTrapCapture) -> bool {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    state
        .get_map_instance(request.map.map_name(), request.map.map_instance())
        .is_some_and(|map| {
            request.x < map.x_size()
                && request.y < map.y_size()
                && map.state().cells()[request.y as usize * map.x_size() as usize + request.x as usize] & CellType::Walkable.as_flag() != 0
        })
}
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::game_systems::CompanionPosition;
use crate::server::script::skill::trap::{GroundTrapCapture, GroundTrapEffect, GroundTrapEffectKind, GroundTrapRelease, linked_ankle};
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

pub(crate) fn trap_fix_position(id: u32, x: u16, y: u16) -> Vec<u8> {
    [
        0x0088_u16.to_le_bytes().to_vec(),
        id.to_le_bytes().to_vec(),
        x.to_le_bytes().to_vec(),
        y.to_le_bytes().to_vec(),
    ]
    .concat()
}

impl Server {
    pub(crate) fn apply_ground_trap_effect(&self, state: &mut ServerState, request: GroundTrapEffect, tick: u128) -> Result<(), String> {
        if state
            .ground_unit(request.target_id, request.map.map_name(), request.map.map_instance())
            .is_some()
        {
            if let GroundTrapEffectKind::Knockback { source_x, source_y, cells } = request.kind {
                self.script_skill_service()
                    .knockback_ground_unit(&request.map, request.target_id, source_x, source_y, cells, state, tick);
                self.script_skill_service()
                    .refresh_ground_unit_snapshot(state, &request.map, request.target_id, tick);
            }
            return Ok(());
        }
        if let Some(mut character) = state.characters_mut().remove(&request.target_id) {
            let result = (|| {
                if character.map_instance_key != request.map || character.status.hp == 0 {
                    return Ok(());
                }
                match request.kind {
                    GroundTrapEffectKind::Status(status) => {
                        StatusEffectService::start(self, &mut character, status, tick, &self.server_service().notification_sender())?;
                    }
                    GroundTrapEffectKind::DrainSp { percent } => {
                        let snapshot = StatusService::instance().to_snapshot(&character.status);
                        let amount = (u64::from(snapshot.max_sp()) * u64::from(percent.min(100)) / 100)
                            .max(1)
                            .min(u64::from(u32::MAX)) as u32;
                        let hp = character.status.hp;
                        let sp = character.status.sp.saturating_sub(amount);
                        self.character_service().update_hp_sp(&mut character, hp, sp);
                    }
                    GroundTrapEffectKind::Knockback { source_x, source_y, cells } => {
                        self.script_skill_service()
                            .apply_knockback(self, state, &mut character, source_x, source_y, cells, tick)?;
                        self.character_service()
                            .defer_position_update_with_flags(&character, &state.map_flags(&request.map));
                    }
                }
                Ok(())
            })();
            state.insert_character(character);
            return result;
        }
        if let Some(owner) = state.companion_owner(request.target_id, request.map.map_name(), request.map.map_instance()) {
            if state.pending_character_logouts.contains_key(&owner.char_id) {
                return Ok(());
            }
            self.script_world_service()
                .handle_companion_trap_effect(self, state, &request, tick)?;
        } else if let Some(map) = state.get_map_instance(request.map.map_name(), request.map.map_instance()) {
            map.add_to_next_tick(MapEvent::GroundTrapEffect(request));
        }
        Ok(())
    }

    pub(crate) fn capture_ground_trap(&self, state: &mut ServerState, request: GroundTrapCapture, tick: u128) -> Result<(), String> {
        if !self.script_skill_service().pending_trap_capture(&request, tick)
            || state.pending_character_logouts.contains_key(&request.target_id)
        {
            return Ok(());
        }
        let destination_walkable = trap_destination_walkable(state, &request);
        if let Some(mut character) = state.characters_mut().remove(&request.target_id) {
            let result = (|| {
                if character.map_instance_key != request.map || character.status.hp == 0 {
                    return Ok(());
                }
                let snapshot = StatusService::instance().to_snapshot(&character.status);
                let move_target = destination_walkable
                    && !snapshot
                        .bonuses_raw()
                        .iter()
                        .any(|bonus| matches!(bonus, BonusType::EnableNoKnockback));
                let sender = self.server_service().notification_sender();
                if StatusEffectService::start(self, &mut character, request.request, tick, &sender)? && move_target {
                    character.movements.clear();
                    character.update_position(request.x, request.y);
                    character.refresh_script_context();
                    self.character_service()
                        .defer_position_update_with_flags(&character, &state.map_flags(&request.map));
                    self.character_service()
                        .send_area_notification_around_characters(&character, trap_fix_position(character.char_id, request.x, request.y));
                }
                Ok(())
            })();
            state.insert_character(character);
            return result;
        }
        if let Some(owner) = state.companion_owner(request.target_id, request.map.map_name(), request.map.map_instance()) {
            let owner_id = owner.char_id;
            if state.pending_character_logouts.contains_key(&owner_id) {
                return Ok(());
            }
            let move_target = destination_walkable
                && crate::server::service::script_world_service::companion_status_snapshot(owner, request.target_id).is_some_and(
                    |snapshot| {
                        !snapshot
                            .bonuses_raw()
                            .iter()
                            .any(|bonus| matches!(bonus, BonusType::EnableNoKnockback))
                    },
                );
            self.script_world_service()
                .handle_companion_status_change(self, state, request.target_id, request.request, tick)?;
            if move_target && linked_ankle(state, &request.map, request.target_id, request.trap_id).is_some() {
                if let Some(owner) = state.characters_mut().get_mut(&owner_id) {
                    owner.game_systems.rendered_companions.insert(request.target_id, CompanionPosition {
                        x: request.x,
                        y: request.y,
                        map_instance: request.map.map_instance(),
                    });
                    let command = owner.game_systems.companion_commands.entry(request.target_id).or_default();
                    command.destination = None;
                    let sender = self.server_service().notification_sender();
                    let _ = sender.send(Notification::Area(AreaNotification::new(
                        request.map.map_name().clone(),
                        request.map.map_instance(),
                        AreaNotificationRangeType::Fov {
                            x: request.x,
                            y: request.y,
                            exclude_id: None,
                        },
                        trap_fix_position(request.target_id, request.x, request.y),
                    )));
                }
            }
            return Ok(());
        }
        if let Some(map) = state.get_map_instance(request.map.map_name(), request.map.map_instance()) {
            map.add_to_next_tick(MapEvent::GroundTrapCapture(request));
        }
        Ok(())
    }

    pub(crate) fn release_ground_trap(&self, state: &mut ServerState, request: GroundTrapRelease, tick: u128) -> Result<(), String> {
        if linked_ankle(state, &request.map, request.target_id, request.trap_id).is_none() {
            return Ok(());
        }
        if let Some(mut character) = state.characters_mut().remove(&request.target_id) {
            StatusEffectService::end(
                self,
                &mut character,
                Some(StatusChangeKind::Ankle),
                tick,
                &self.server_service().notification_sender(),
            );
            state.insert_character(character);
        } else if state
            .companion_owner(request.target_id, request.map.map_name(), request.map.map_instance())
            .is_some()
        {
            self.script_world_service()
                .handle_companion_end_status(self, state, request.target_id, Some(StatusChangeKind::Ankle), tick)?;
        } else if let Some(map) = state.get_map_instance(request.map.map_name(), request.map.map_instance()) {
            map.add_to_next_tick(MapEvent::GroundTrapRelease(request));
        }
        Ok(())
    }
}
