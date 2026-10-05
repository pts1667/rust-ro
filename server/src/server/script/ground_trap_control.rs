use models::enums::EnumWithMaskValueU64;

use super::super::ground::GroundSkill;
use super::super::metadata::SkillMetadata;
use super::super::{ScriptSkillEffect, ScriptSkillService};
use crate::repository::Error;
use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemGrant};
use crate::server::Server;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::game_systems::PlayerOption;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    fn trap_recovery_materials(&self, ground: &GroundSkill) -> Result<Vec<(i32, i16)>, String> {
        if !self.configuration.config().game.skill_units.return_all_trap_materials {
            return Ok(ground
                .recovery_item
                .or_else(|| self.configuration.find_item_by_name("Booby_Trap").map(|item| item.id))
                .map(|item_id| vec![(item_id, 1)])
                .unwrap_or_default());
        }
        let metadata = SkillMetadata::find(ground.skill_id).ok_or("Trap material metadata is missing")?;
        let Some(costs) = metadata.requires.as_ref().and_then(|requires| requires.get("ItemCost")?.as_array()) else {
            return Ok(vec![]);
        };
        let mut materials = Vec::new();
        for cost in costs.iter().filter(|cost| {
            cost.get("Level")
                .and_then(|level| level.as_u64())
                .is_none_or(|level| level == u64::from(ground.level))
        }) {
            let name = cost
                .get("Item")
                .and_then(|name| name.as_str())
                .ok_or("Trap material name is missing")?;
            let item = self
                .configuration
                .find_item_by_name(name)
                .ok_or("Trap recovery material is unavailable")?;
            let amount = cost
                .get("Amount")
                .and_then(|amount| amount.as_i64())
                .and_then(|amount| i16::try_from(amount).ok())
                .filter(|amount| *amount > 0)
                .ok_or("Trap material amount is invalid")?;
            materials.push((item.id, amount));
        }
        Ok(materials)
    }

    fn recover_trap_inventory_items(
        &self,
        server: &Server,
        character: &mut Character,
        materials: &[(i32, i16)],
    ) -> Result<Option<u8>, String> {
        let change = ScriptInventoryTransaction {
            char_id: character.char_id,
            account_id: character.account_id,
            consumption: None,
            exact_removals: vec![],
            removals: vec![],
            identifications: vec![],
            grants: materials
                .iter()
                .map(|(item_id, amount)| ScriptItemGrant {
                    item_id: *item_id,
                    amount: *amount,
                    identified: true,
                    refine: 0,
                    cards: [0; 4],
                    unique_id: None,
                    damaged: false,
                })
                .collect(),
            variables: vec![],
            zeny: None,
            hp: None,
            sp: None,
            max_weight: server.character_service().max_weight(character),
            max_slots: usize::from(self.configuration.config().game.max_inventory),
            world: None,
            reset_skills: None,
            fame: None,
            character_changes: vec![],
            pool_draws: vec![],
        };
        match self.repository.script_inventory_transaction(&change) {
            Ok(result) => {
                server.item_service().install_inventory(server, character, result.inventory, None);
                Ok(None)
            }
            Err(Error::InvalidInput(reason)) => match reason.as_str() {
                "Script reward exceeds maximum weight" => Ok(Some(2)),
                "Character inventory is full" => Ok(Some(4)),
                "Item stack would overflow" => Ok(Some(5)),
                _ => Err(reason),
            },
            Err(error) => Err(error.to_string()),
        }
    }

    pub(crate) fn validate_player_trap_control(
        &self,
        state: &ServerState,
        character: &Character,
        trap_id: u32,
        skill_id: u32,
        level: u8,
        tick: u128,
        ignore_range: bool,
    ) -> Result<(u16, u16), String> {
        let spring = super::super::metadata::SkillMetadata::find(skill_id).is_some_and(|metadata| metadata.name == "HT_SPRINGTRAP");
        if spring && character.options & PlayerOption::Falcon.as_flag() == 0 {
            return Err("Spring Trap requires a falcon".into());
        }
        let grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        let ground = grounds
            .iter()
            .find(|ground| {
                ground.kind.trap()
                    && ground.cast_verified
                    && ground.expires_at > tick
                    && ground.map == *character.current_map_name()
                    && ground.instance == character.current_map_instance()
                    && ground.cells.iter().any(|cell| cell.id == trap_id && cell.remaining_hits > 0)
            })
            .ok_or("Skill requires a live trap on this map")?;
        if !spring && ground.source_id != character.char_id && !state.map_flags(&character.map_instance_key).versus(state.siege_active) {
            return Err("Remove Trap can only retrieve your own traps on this map".into());
        }
        let cell = ground.cells.iter().find(|cell| cell.id == trap_id).unwrap();
        if !ignore_range
            && character.x.abs_diff(cell.x).max(character.y.abs_diff(cell.y))
                > self
                    .player_skill_range(&StatusService::instance().to_snapshot(&character.status), skill_id, level)
                    .max(1)
        {
            return Err("Trap is out of skill range".into());
        }
        Ok((cell.x, cell.y))
    }

    pub(crate) fn apply_player_trap_control(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        trap_id: u32,
        map: &MapInstanceKey,
        spring: bool,
        ignore_range: bool,
        tick: u128,
    ) -> Result<(), String> {
        if character.map_instance_key != *map {
            return Err("Trap control source changed maps".into());
        }
        self.validate_player_trap_control(state, character, trap_id, effect.skill_id, effect.level, tick, ignore_range)?;
        let mut grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        let ground = grounds
            .iter_mut()
            .find(|ground| {
                ground.kind.trap()
                    && ground.cast_verified
                    && ground.expires_at > tick
                    && ground.map == *map.map_name()
                    && ground.instance == map.map_instance()
                    && ground.cells.iter().any(|cell| cell.id == trap_id && cell.remaining_hits > 0)
            })
            .ok_or("Trap expired before removal")?;
        if spring {
            if ground.capture.is_none() {
                ground.triggered = true;
                ground.recovery_item = None;
                ground.expires_at = tick + 1500;
                self.change_trap_view(ground, 140);
            }
        } else {
            if !ground.triggered {
                let materials = self.trap_recovery_materials(ground)?;
                if !materials.is_empty() {
                    if let Some(failure) = self.recover_trap_inventory_items(server, character, &materials)? {
                        let mut packet = packets::packets::PacketZcItemPickupAck3::new(self.configuration.packetver());
                        packet.set_result(failure);
                        packets::packets::Packet::fill_raw(&mut packet);
                        self.queue_notification(crate::server::model::events::client_notification::Notification::Char(
                            crate::server::model::events::client_notification::CharNotification::new(character.char_id, packet.raw),
                        ));
                        if self.configuration.config().game.skill_units.drop_recovery_items_when_full {
                            if let Some(map) = state.get_map_instance_from_character(character) {
                                for (item_id, amount) in materials {
                                    map.add_to_next_tick(MapEvent::GroundTrapRecover {
                                        item_id,
                                        amount: amount as u16,
                                        x: character.x,
                                        y: character.y,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            ground.recovery_item = None;
            ground.expires_at = tick;
            if let Some(capture) = &ground.capture {
                capture.lease.cancel();
            }
        }
        let mut visual = effect.clone();
        visual.target_id = trap_id;
        self.notify_support_skill(character, &visual);
        Ok(())
    }
}
