use models::enums::EnumWithMaskValueU32;
use models::enums::skill_enums::SkillEnum;
use models::status_change::StatusChangeKind;

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterEndStatus, CharacterKnockback, GameEvent};
use crate::server::model::events::map_event::{MapEvent, MobFace, MobWarpTo};
use crate::server::model::map::Map;
use crate::server::model::map_item::MapItemType;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub fn backstab_position_allowed(source_x: u16, source_y: u16, target_x: u16, target_y: u16, target_direction: u16) -> bool {
        if (source_x, source_y) == (target_x, target_y) {
            return false;
        }
        let direction = Self::direction_to(source_x, source_y, target_x, target_y, 0);
        let difference = direction.abs_diff(target_direction % 8);
        difference <= 1 || difference >= 7
    }

    pub fn validate_damage_target(&self, state: &ServerState, character: &Character, skill_id: u32, target_id: u32) -> Result<(), String> {
        if skill_id != SkillEnum::RgBackstap.id() {
            return Ok(());
        }
        let target = state
            .map_item_snapshot(target_id, character.current_map_name(), character.current_map_instance())
            .ok_or("Back Stab target left the map")?;
        if !Self::backstab_position_allowed(
            character.x,
            character.y,
            target.position.x,
            target.position.y,
            target.position.dir,
        ) {
            return Err("Back Stab requires a position behind the target".into());
        }
        Ok(())
    }

    pub fn snatch_succeeds(source_level: u32, target_level: u32, skill_level: u8, immune: bool, roll: u8) -> bool {
        !immune
            && i64::from(roll % 100) < (50 + 5 * i64::from(skill_level) + i64::from(source_level) - i64::from(target_level)).clamp(0, 100)
    }

    pub(super) fn queue_weapon_aftermath(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        damage: &Damage,
        tick: u128,
    ) -> bool {
        if damage.skill_id == SkillEnum::RgBackstap.id() {
            server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus {
                char_id: character.char_id,
                kind: Some(StatusChangeKind::Hiding),
            }));
            if let Some(target) = state.map_item_snapshot(damage.target_id, character.current_map_name(), character.current_map_instance())
            {
                let direction = (Self::direction_to(character.x, character.y, target.position.x, target.position.y, character.dir) + 4) % 8;
                if *target.map_item.object_type() == MapItemType::Mob {
                    if let Some(instance) = state.get_map_instance_from_character(character) {
                        instance.add_to_next_tick(MapEvent::MobFace(MobFace {
                            mob_id: damage.target_id,
                            dir: direction,
                        }));
                    }
                } else {
                    server.add_to_next_tick(GameEvent::CharacterScriptSkill(Self::aftermath_effect(
                        character,
                        damage,
                        damage.target_id,
                        ScriptSkillAction::Face { direction },
                    )));
                }
            }
        }
        if damage.skill_id == SkillEnum::NjIssen.id() {
            let target = state.map_item_snapshot(damage.target_id, character.current_map_name(), character.current_map_instance());
            let (x, y) = target.map_or((character.x, character.y), |target| {
                let (dx, dy) = Self::facing_vector(Self::direction_to(
                    character.x,
                    character.y,
                    target.position.x,
                    target.position.y,
                    character.dir,
                ));
                (
                    (i32::from(target.position.x) + 2 * dx).clamp(0, i32::from(u16::MAX)) as u16,
                    (i32::from(target.position.y) + 2 * dy).clamp(0, i32::from(u16::MAX)) as u16,
                )
            });
            server.add_to_next_tick(GameEvent::CharacterScriptSkill(Self::aftermath_effect(
                character,
                damage,
                character.char_id,
                ScriptSkillAction::FinalStrike {
                    x,
                    y,
                    map: character.current_map_name().clone(),
                    instance: character.current_map_instance(),
                },
            )));
        }
        if damage.skill_id == SkillEnum::ChPalmstrike.id() {
            let delay =
                1000 + u128::from(StatusService::instance().attack_motion(&StatusService::instance().to_snapshot(&character.status)));
            let action = ScriptSkillAction::DelayedWeaponHit {
                target_id: damage.target_id,
                map: character.current_map_name().clone(),
                instance: character.current_map_instance(),
            };
            server.add_to_tick(
                GameEvent::CharacterScriptSkill(Self::aftermath_effect(character, damage, character.char_id, action)),
                delay.div_ceil(40).saturating_sub(1) as usize,
            );
            use models::enums::EnumWithNumberValue;
            use models::enums::action::ActionType;
            use packets::packets::{Packet, PacketZcNotifyAct};
            let mut packet = PacketZcNotifyAct::new(self.configuration.packetver());
            packet.set_gid(character.char_id);
            packet.set_target_gid(damage.target_id);
            packet.set_attack_mt((delay - 1000).min(i32::MAX as u128) as i32);
            packet.set_damage(-1);
            packet.set_count(1);
            packet.set_action(ActionType::AttackNomotion.value() as u8);
            packet.fill_raw();
            self.notify_area(character, packet.raw);
            return true;
        }
        let _ = tick;
        false
    }

    fn aftermath_effect(source: &Character, damage: &Damage, target_id: u32, action: ScriptSkillAction) -> ScriptSkillEffect {
        ScriptSkillEffect {
            source_char_id: source.char_id,
            target_id,
            skill_id: damage.skill_id,
            level: damage.skill_level,
            heal_value: 0,
            proc_depth: damage.proc_depth,
            skill_event_emitted: true,
            cast_generation: 0,
            action,
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        }
    }

    pub(super) fn apply_delayed_weapon_hit(
        &self,
        server: &Server,
        state: &ServerState,
        source: &Character,
        effect: &ScriptSkillEffect,
        target_id: u32,
        map: &String,
        instance_id: u8,
        tick: u128,
    ) -> Result<(), String> {
        if source.current_map_name() != map || source.current_map_instance() != instance_id {
            return Ok(());
        }
        let Some(target) = state.map_item_snapshot(target_id, map, instance_id) else {
            return Ok(());
        };
        let snapshot = if let Some(character) = state.get_character(target_id) {
            StatusService::instance().to_snapshot(&character.status)
        } else {
            state
                .map_item_mob_status(&target.map_item, map, instance_id)
                .ok_or("Delayed skill target has no status")?
        };
        if snapshot.hp() == 0 {
            return Ok(());
        }
        if source.status.has_status_change(StatusChangeKind::Hiding) || snapshot.has_status_change(StatusChangeKind::Hiding) {
            let cells = SkillMetadata::find(effect.skill_id)
                .and_then(|metadata| metadata.knockback.as_ref())
                .and_then(|value| value.value(effect.level, "Amount"))
                .unwrap_or(0)
                .max(0) as u16;
            if *target.map_item.object_type() == MapItemType::Mob {
                if let Some(instance) = state.get_map_instance(map, instance_id) {
                    instance.add_to_next_tick(MapEvent::MobKnockback {
                        mob_id: target_id,
                        source_x: source.x,
                        source_y: source.y,
                        cells,
                    });
                }
            } else {
                server.add_to_next_tick(GameEvent::CharacterKnockback(CharacterKnockback {
                    char_id: target_id,
                    source_x: source.x,
                    source_y: source.y,
                    cells,
                }));
            }
            return Ok(());
        }
        let source_status = StatusService::instance().to_snapshot(&source.status);
        if !server.player_skill_target_allowed(state, source, target_id, effect.skill_id, true) { return Ok(()); }
        let ratio = (200 + 100 * u32::from(effect.level)) as f32 / 100.0;
        let element = server.battle_service().attack_element(&source_status, None);
        let landed = server
            .battle_service()
            .skill_hits(&source_status, &snapshot, effect.skill_id, effect.level);
        let amount = if landed {
            server.battle_service().player_physical_skill_damage_signed(
                &source_status,
                &snapshot,
                ratio,
                1,
                false,
                &element,
                effect.skill_id,
            )
        } else {
            0
        };
        let flags = SkillMetadata::find(effect.skill_id)
            .ok_or("Delayed skill metadata is missing")?
            .battle_flags(false);
        let mut damage = Damage {
            healing: 0,
            right_hand_damage: None,
            target_id,
            attacker_id: source.char_id,
            damage: 0,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: flags,
            skill_id: effect.skill_id,
            skill_level: effect.level,
            landed,
            proc_depth: effect.proc_depth,
            credit_id: source.char_id,
            defenses_applied: true,
            magic_context: None,
        };
        damage.set_signed_damage(amount);
        if *target.map_item.object_type() == MapItemType::Mob {
            state
                .get_map_instance(map, instance_id)
                .ok_or("Delayed skill map is unavailable")?
                .add_to_next_tick(MapEvent::MobDamage(damage));
        } else {
            server.add_to_next_tick(GameEvent::CharacterDamage(damage));
        }
        self.notify_attack_skill(source, target_id, effect.skill_id, effect.level, amount);
        Ok(())
    }

    pub(super) fn relocate_skill_actor(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        x: u16,
        y: u16,
        tick: u128,
    ) -> bool {
        let Some(instance) = state.get_map_instance_from_character(character) else {
            return false;
        };
        let map_state = instance.state();
        use models::enums::EnumWithMaskValueU16;
        use models::enums::cell::CellType;
        if x >= instance.x_size()
            || y >= instance.y_size()
            || map_state.cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() == 0
        {
            return false;
        }
        server.character_service().cancel_movement(character, tick);
        character.clear_pending_skill();
        character.clear_attack();
        character.update_position(x, y);
        character.last_moved_at = tick;
        let mut packet = 0x0088_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&character.char_id.to_le_bytes());
        packet.extend_from_slice(&x.to_le_bytes());
        packet.extend_from_slice(&y.to_le_bytes());
        self.notify_area(character, packet);
        true
    }

    pub(super) fn apply_snatch_warp(
        &self,
        server: &Server,
        state: &ServerState,
        source: &Character,
        victim_id: u32,
        map: &String,
        instance_id: u8,
    ) -> Result<(), String> {
        if source.current_map_name() != map || source.current_map_instance() != instance_id {
            return Ok(());
        }
        if !Self::snatch_map_allowed(state, source) { return Ok(()); }
        let instance = state.get_map_instance(map, instance_id).ok_or("Snatch map is unavailable")?;
        let map_state = instance.state();
        use models::enums::EnumWithMaskValueU16;
        use models::enums::cell::CellType;
        let walkable = |x: u16, y: u16| {
            x < instance.x_size()
                && y < instance.y_size()
                && map_state.cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() != 0
        };
        let count = usize::from(instance.x_size()) * usize::from(instance.y_size());
        let start = fastrand::usize(0..count.max(1));
        let destination = (0..count)
            .map(|offset| (start + offset) % count)
            .map(|cell| {
                (
                    (cell % usize::from(instance.x_size())) as u16,
                    (cell / usize::from(instance.x_size())) as u16,
                )
            })
            .find(|(x, y)| walkable(*x, *y))
            .ok_or("Snatch map has no walkable destination")?;
        server.server_service.schedule_warp_to_walkable_cell_by_character_in_instance(
            &Map::name_without_ext(map),
            destination.0,
            destination.1,
            source.char_id,
            instance_id,
        );
        if let Some(victim) = state
            .map_item_snapshot(victim_id, map, instance_id)
            .filter(|victim| victim.position.x.abs_diff(source.x).max(victim.position.y.abs_diff(source.y)) <= 14)
        {
            let nearby = Self::square_cells(destination.0, destination.1, 1)
                .into_iter()
                .find(|(x, y)| walkable(*x, *y))
                .unwrap_or(destination);
            if *victim.map_item.object_type() == MapItemType::Mob {
                if map_state.get_mob(victim_id).is_some_and(|mob| mob.hp() > 0) {
                    instance.add_to_next_tick(MapEvent::MobWarpTo(MobWarpTo {
                        mob_id: victim_id,
                        x: nearby.0,
                        y: nearby.1,
                    }));
                }
            } else if state.get_character(victim_id).is_some_and(|victim| victim.status.hp > 0) {
                server.server_service.schedule_warp_to_walkable_cell_by_character_in_instance(
                    &Map::name_without_ext(map),
                    nearby.0,
                    nearby.1,
                    victim_id,
                    instance_id,
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn back_stab_uses_the_targets_rear_arc_and_refuses_the_same_cell() {
        assert!(ScriptSkillService::backstab_position_allowed(4, 5, 5, 5, 6));
        assert!(ScriptSkillService::backstab_position_allowed(4, 5, 5, 5, 7));
        assert!(!ScriptSkillService::backstab_position_allowed(4, 5, 5, 5, 2));
        assert!(!ScriptSkillService::backstab_position_allowed(5, 5, 5, 5, 6));
    }
    #[test]
    fn snatch_uses_skill_level_and_actual_actor_levels_with_immunity() {
        assert!(ScriptSkillService::snatch_succeeds(80, 50, 1, false, 84));
        assert!(!ScriptSkillService::snatch_succeeds(80, 50, 1, false, 85));
        assert!(!ScriptSkillService::snatch_succeeds(80, 50, 10, true, 0));
        assert!(!ScriptSkillService::snatch_succeeds(1, 99, 1, false, 0));
    }
}
