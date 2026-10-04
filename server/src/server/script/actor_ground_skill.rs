use std::collections::HashSet;
use std::sync::atomic::Ordering;

use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithStringValue};
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{Packet, PacketZcNotifySkill2};

use super::ground::{GroundCell, GroundKind, GroundSkill, NEXT_GROUND_UNIT};
use super::metadata::SkillMetadata;
use super::{GroundSkillSource, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::events::game_event::GameEvent;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

impl ScriptSkillService {
    pub fn validate_actor_ground(
        &self,
        state: &ServerState,
        source: &GroundSkillSource,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        self.validate_actor_ground_with_options(state, source, skill_id, level, x, y, tick, false, false)
    }

    pub fn validate_actor_ground_with_options(&self, state: &ServerState, source: &GroundSkillSource,
        skill_id: u32, level: u8, x: u16, y: u16, tick: u128, ignore_range: bool, scripted: bool) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Ground skill metadata is missing")?;
        if !scripted && state.map_flags_for(&source.map, source.instance).enabled(crate::server::model::map_flags::MapFlag::NoSkill) {
            return Err("Companion skills cannot be used on this map".into());
        }
        if level == 0 || level > metadata.max_level || source.status.hp() == 0 {
            return Err("Ground skill source or level is invalid".into());
        }
        let kind = GroundKind::from_name(&metadata.name)
            .filter(|kind| scripted || kind.trap() || matches!(kind, GroundKind::ArrowShower | GroundKind::HeavenDrive | GroundKind::Thunderstorm))
            .ok_or("Actor ground skill has no implemented classic unit effect")?;
        if !ignore_range && source.x.abs_diff(x).max(source.y.abs_diff(y)) > metadata.range(level).unwrap_or(1).unsigned_abs().max(1) as u16 {
            return Err("Companion ground target is out of range".into());
        }
        let instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Ground map is unavailable")?;
        let map_state = instance.state();
        if x >= instance.x_size()
            || y >= instance.y_size()
            || map_state
                .cells()
                .get(y as usize * instance.x_size() as usize + x as usize)
                .is_none_or(|cell| cell & CellType::Shootable.as_flag() == 0)
        {
            return Err("Ground target is outside usable terrain".into());
        }
        if kind.trap() {
            let grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
            if grounds.iter().any(|ground| {
                ground.kind.trap()
                    && ground.map == source.map
                    && ground.instance == source.instance
                    && ground.expires_at > tick
                    && ground
                        .cells
                        .iter()
                        .any(|cell| cell.remaining_hits > 0 && cell.x == x && cell.y == y)
            }) {
                return Err("Another trap already occupies that cell".into());
            }
            if source.actor_id == source.owner_id {
                let range = metadata.unit_value("Range", level, "Size").unwrap_or(0).max(0) as u16;
                if source.x.abs_diff(x).max(source.y.abs_diff(y)) <= range
                    || map_state
                        .mobs()
                        .values()
                        .any(|mob| mob.hp() > 0 && mob.x.abs_diff(x).max(mob.y.abs_diff(y)) <= range)
                    || state.characters().values().any(|player| {
                        player.status.hp > 0
                            && player.current_map_name() == &source.map
                            && player.current_map_instance() == source.instance
                            && player.x.abs_diff(x).max(player.y.abs_diff(y)) <= range
                    })
                {
                    return Err("A living actor occupies the trap trigger area".into());
                }
            }
        }
        Ok(())
    }

    pub fn place_actor_ground_skill(
        &self,
        _server: &Server,
        state: &ServerState,
        source: GroundSkillSource,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        self.place_actor_ground_skill_with_options(_server, state, source, skill_id, level, x, y, tick, false, false)
    }

    pub fn place_actor_ground_skill_with_options(&self, _server: &Server, state: &ServerState, source: GroundSkillSource,
        skill_id: u32, level: u8, x: u16, y: u16, tick: u128, ignore_range: bool, scripted: bool) -> Result<(), String> {
        self.validate_actor_ground_with_options(state, &source, skill_id, level, x, y, tick, ignore_range, scripted)?;
        let metadata = SkillMetadata::find(skill_id).unwrap();
        let kind = GroundKind::from_name(&metadata.name).unwrap();
        let duration = metadata.duration(level, false).unwrap_or(100).max(1) as u128;
        let range = if kind == GroundKind::Pneuma { 1 } else if kind == GroundKind::ArrowShower {
            metadata.splash(level).unwrap_or(2)
        } else {
            metadata.unit_value("Range", level, "Size").unwrap_or(0)
        }
        .max(0) as u16;
        let layout = if !kind.trap() && kind != GroundKind::ArrowShower {
            metadata.unit_value("Layout", level, "Size").unwrap_or(0).max(0) as u16
        } else { 0 };
        let instance = state.get_map_instance(&source.map, source.instance).ok_or("Ground map is unavailable")?;
        let locations = match kind {
            GroundKind::Firewall => Self::firewall_cells(source.x, source.y, x, y),
            GroundKind::Pneuma => vec![(x, y)],
            GroundKind::GrandCross => Self::grand_cross_cells(x, y),
            _ => Self::square_cells(x, y, layout),
        };
        let cells = locations.into_iter().filter(|(x, y)| *x < instance.x_size() && *y < instance.y_size()
            && instance.state().cells()[*y as usize * instance.x_size() as usize + *x as usize] & CellType::Shootable.as_flag() != 0)
            .map(|(x, y)| GroundCell { id: NEXT_GROUND_UNIT.fetch_add(1, Ordering::Relaxed), x, y,
                remaining_hits: if kind == GroundKind::Firewall { 4 + u16::from(level) } else if kind == GroundKind::GrandCross { 3 } else { u16::MAX } })
            .collect::<Vec<_>>();
        if cells.is_empty() { return Err("Ground skill has no usable cells".into()); }
        let mut ground = GroundSkill {
            kind,
            source_id: source.actor_id,
            source_x: source.x,
            source_y: source.y,
            map: source.map.clone(),
            instance: source.instance,
            skill_id,
            level,
            depth: 0,
            active_from: tick,
            expires_at: tick + duration,
            next_hit_at: tick + if matches!(kind, GroundKind::GrandCross | GroundKind::Earthquake | GroundKind::StormGust) { 100 } else { 0 },
            interval: metadata.unit_value("Interval", level, "Time").unwrap_or(40).max(40) as u128,
            effect_range: range,
            displayed: false,
            skill_event_emitted: true,
            cast_generation: 0,
            cast_finish_at: tick,
            cast_verified: true,
            cells,
            affected: HashSet::new(),
            actor_source: Some(source),
            triggered: false,
            waves: 0,
        };
        let mut pose = 0x0117_u16.to_le_bytes().to_vec();
        pose.extend_from_slice(&(skill_id as u16).to_le_bytes());
        pose.extend_from_slice(&ground.source_id.to_le_bytes());
        pose.extend_from_slice(&u16::from(level).to_le_bytes());
        pose.extend_from_slice(&x.to_le_bytes());
        pose.extend_from_slice(&y.to_le_bytes());
        pose.extend_from_slice(&(tick as u32).to_le_bytes());
        self.notify_ground_cell(&ground, &ground.cells[0], pose);
        ground.displayed = true;
        for cell in &ground.cells {
            self.notify_ground_cell(&ground, cell, Self::ground_entry_packet_for(
                self.configuration.packetver(),
                cell.id,
                ground.source_id,
                cell.x,
                cell.y,
                level,
                kind.view_id(),
            ));
        }
        self.ground_skills
            .lock()
            .map_err(|_| "Ground skill state is unavailable")?
            .push(ground);
        Ok(())
    }

    pub fn activate_ground_cast(&self, character: &mut Character, skill_id: u32, generation: u64, tick: u128) -> Result<(), String> {
        self.validate_ground_activation(character, skill_id, generation, tick)?;
        let mut grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        for ground in grounds.iter_mut().filter(|ground| {
            ground.source_id == character.char_id
                && ground.skill_id == skill_id
                && ground.cast_generation == generation
                && !ground.cast_verified
                && ground.expires_at > tick
        }) {
            let elapsed = tick.saturating_sub(ground.cast_finish_at);
            ground.active_from += elapsed;
            ground.expires_at += elapsed;
            ground.next_hit_at += elapsed;
            ground.cast_finish_at = tick;
            ground.cast_verified = true;
        }
        character.script_skill_state.casting_until = 0;
        character.script_skill_state.casting_skill_id = 0;
        Ok(())
    }

    pub fn validate_ground_activation(&self, character: &Character, skill_id: u32, generation: u64, tick: u128) -> Result<(), String> {
        if character.status.hp == 0 || (generation != 0 && character.script_skill_state.cast_generation != generation) {
            return Err("Ground cast was interrupted".into());
        }
        let grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        if grounds.iter().any(|ground| {
            ground.source_id == character.char_id
                && ground.skill_id == skill_id
                && ground.cast_generation == generation
                && !ground.cast_verified
                && ground.cast_finish_at <= tick
                && ground.expires_at > tick
                && ground.map == *character.current_map_name()
                && ground.instance == character.current_map_instance()
        }) {
            Ok(())
        } else {
            Err("Ground cast no longer has an active placement".into())
        }
    }

    pub fn ground_unit_position(&self, map: &str, instance: u8, unit_id: u32, tick: u128) -> Option<(u16, u16)> {
        let grounds = self.ground_skills.lock().ok()?;
        grounds
            .iter()
            .filter(|ground| {
                ground.kind.trap() && ground.map == map && ground.instance == instance && ground.cast_verified && ground.expires_at > tick
            })
            .flat_map(|ground| &ground.cells)
            .find(|cell| cell.id == unit_id && cell.remaining_hits > 0)
            .map(|cell| (cell.x, cell.y))
    }

    pub fn remove_ground_trap(&self, map: &str, instance: u8, unit_id: u32, tick: u128) -> Result<(), String> {
        let mut grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        let ground = grounds
            .iter_mut()
            .find(|ground| {
                ground.kind.trap()
                    && ground.map == map
                    && ground.instance == instance
                    && ground.expires_at > tick
                    && ground.cells.iter().any(|cell| cell.id == unit_id)
            })
            .ok_or("Remove Trap requires a live trap")?;
        ground.expires_at = tick;
        Ok(())
    }

    pub(super) fn tick_actor_ground_skill(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick || ground.triggered || tick < ground.next_hit_at {
            return;
        }
        ground.next_hit_at = tick + 40;
        let Some(source) = ground.actor_source.clone() else {
            return;
        };
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        if !ground.kind.trap() && ground.kind != GroundKind::ArrowShower {
            self.tick_actor_magic_ground(server, state, ground, &source, tick);
            return;
        }
        let map_state = instance.state();
        let targets = map_state
            .mobs()
            .values()
            .filter(|mob| mob.hp() > 0 && (!mob.summoned || mob.summon_ai == 0))
            .collect::<Vec<_>>();
        let Some(trigger) = targets.iter().find(|mob| ground.covers(mob.x, mob.y)) else {
            return;
        };
        let trigger_id = trigger.id;
        let (center_x, center_y) = if matches!(ground.kind, GroundKind::Sandman | GroundKind::FreezingTrap) {
            (trigger.x, trigger.y)
        } else {
            (ground.cells[0].x, ground.cells[0].y)
        };
        let metadata = SkillMetadata::find(ground.skill_id).unwrap();
        let radius = metadata.splash(ground.level).unwrap_or(0).max(0) as u16;
        let flags = metadata.battle_flags(metadata.range(ground.level).unwrap_or(1) > 3);
        for mob in targets.into_iter().filter(|mob| {
            if radius == 0 {
                mob.id == trigger_id
            } else {
                mob.x.abs_diff(center_x).max(mob.y.abs_diff(center_y)) <= radius
            }
        }) {
            match ground.kind {
                GroundKind::SkidTrap => {
                    instance.add_to_next_tick(MapEvent::MobKnockback {
                        mob_id: mob.id,
                        source_x: ground.source_x,
                        source_y: ground.source_y,
                        cells: metadata
                            .knockback
                            .as_ref()
                            .and_then(|value| value.value(ground.level, "Amount"))
                            .unwrap_or(2)
                            .max(0) as u16,
                    });
                    instance.add_to_next_tick(MapEvent::MobLoseTarget { mob_id: mob.id });
                }
                GroundKind::Sandman => {
                    let request = StatusChangeRequest {
                        kind: StatusChangeKind::Sleep,
                        duration_ms: metadata.duration(ground.level, true).unwrap_or(30000),
                        values: [ground.level as i32, 0, 0, 0],
                        rate: (40 + 10 * u16::from(ground.level)) * 100,
                        flags: 0,
                    };
                    instance.add_to_delayed_tick(MapEvent::MobStatusChange { mob_id: mob.id, request }, 1000);
                }
                GroundKind::LandMine | GroundKind::FreezingTrap | GroundKind::ArrowShower => {
                    let landed = ground.kind == GroundKind::LandMine
                        || server
                            .battle_service()
                            .skill_hits(&source.status, &mob.status, ground.skill_id, ground.level);
                    let damage = if !landed {
                        0
                    } else if ground.kind == GroundKind::LandMine {
                        let raw = Self::classic_land_mine_damage(ground.level, source.status.dex(), source.status.int());
                        server
                            .battle_service()
                            .actor_misc_skill_damage(raw, &source.status, &mob.status, &Element::Earth, flags, ground.skill_id)
                            .min(i32::MAX as u32) as i32
                    } else {
                        let ratio = if ground.kind == GroundKind::ArrowShower {
                            (75 + 5 * ground.level as u32) as f32 / 100.0
                        } else {
                            1.0
                        };
                        let element = if ground.kind == GroundKind::FreezingTrap {
                            Element::Water
                        } else {
                            server.battle_service().attack_element(&source.status, None)
                        };
                        server.battle_service().actor_physical_skill_damage_signed(
                            source.raw_attack,
                            &source.status,
                            &mob.status,
                            false,
                            ratio,
                            1,
                            &element,
                            flags,
                            ground.skill_id,
                        )
                    };
                    let mut damage_event = Damage {
                        healing: 0,
                        right_hand_damage: None,
                        target_id: mob.id,
                        attacker_id: source.actor_id,
                        damage: 0,
                        attacked_at: tick,
                        damage_motion: 0,
                        battle_flags: flags,
                        skill_id: ground.skill_id,
                        skill_level: ground.level,
                        proc_depth: 0,
                        credit_id: source.owner_id,
                        defenses_applied: true,
                        magic_context: None,
                        landed,
                    };
                    damage_event.set_signed_damage(damage);
                    instance.add_to_next_tick(MapEvent::MobDamage(damage_event));
                    self.notify_actor_ground_damage(ground, mob.id, damage);
                }
                _ => {}
            }
        }
        ground.triggered = true;
        ground.expires_at = tick + if ground.kind.trap() { 1500 } else { 40 };
        if ground.kind.trap() {
            for cell in &ground.cells {
                let mut packet = (if self.configuration.packetver() >= 20181121 {
                    0x0A43_u16
                } else {
                    0x01D7_u16
                })
                .to_le_bytes()
                .to_vec();
                packet.extend_from_slice(&cell.id.to_le_bytes());
                packet.push(0);
                if self.configuration.packetver() >= 20181121 {
                    packet.extend_from_slice(&140_u32.to_le_bytes());
                    packet.extend_from_slice(&0_u32.to_le_bytes());
                } else {
                    packet.extend_from_slice(&140_u16.to_le_bytes());
                    packet.extend_from_slice(&0_u16.to_le_bytes());
                }
                self.notify_ground_cell(ground, cell, packet);
            }
        }
    }

    pub fn classic_land_mine_damage(level: u8, dex: u16, int: u16) -> u32 {
        (u64::from(level) * (u64::from(dex) + 75) * (100 + u64::from(int)) / 100).min(u64::from(u32::MAX)) as u32
    }

    fn notify_actor_ground_damage(&self, ground: &GroundSkill, target_id: u32, damage: i32) {
        let mut packet = PacketZcNotifySkill2::new(self.configuration.packetver());
        packet.set_aid(ground.source_id);
        packet.set_target_id(target_id);
        packet.set_skid(ground.skill_id as u16);
        packet.set_level(ground.level as i16);
        packet.set_damage(damage);
        packet.set_count(ground.actor_source.as_ref().and_then(|source| source.fixed_damage).map_or(1, |fixed| fixed.hits.max(1)));
        packet.set_action(6);
        packet.fill_raw();
        self.notify_ground_cell(ground, &ground.cells[0], packet.raw);
    }

    fn tick_actor_magic_ground(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, source: &GroundSkillSource, tick: u128) {
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else { return; };
        let Some(metadata) = SkillMetadata::find(ground.skill_id) else { return; };
        ground.next_hit_at = tick.saturating_add(ground.interval);
        if matches!(ground.kind, GroundKind::Pneuma | GroundKind::Quagmire | GroundKind::Deluge | GroundKind::LandProtector) { return; }
        let mut targets = instance.state().mobs().values().filter(|mob| mob.hp() > 0 && (!mob.summoned || mob.summon_ai == 0)
            && ground.covers(mob.x, mob.y) && Self::area_skill_target_allowed(&source.status, &mob.status, ground.skill_id))
            .map(|mob| (mob.id, mob.status.clone(), false)).collect::<Vec<_>>();
        if let Some(owner) = state.get_character(source.owner_id) {
            targets.extend(state.characters().values().filter(|player| player.map_instance_key == owner.map_instance_key
                && ground.covers(player.x, player.y) && server.player_combat_target_allowed(state, owner, player.char_id))
                .map(|player| (player.char_id, crate::server::service::status_service::StatusService::instance().to_snapshot(&player.status), true)));
        } else if let Some(actor) = self.find_script_skill_actor(state, source.actor_id) {
            targets = self.actor_area_targets(server, state, &actor, source.x, source.y, u16::MAX).into_iter()
                .filter(|(id, _, _)| state.map_item_snapshot(*id, &ground.map, ground.instance).is_some_and(|target| ground.covers(target.x(), target.y())))
                .collect();
        }
        let object = skills::skill_enums::to_object(models::enums::skill_enums::SkillEnum::from_id(ground.skill_id), ground.level);
        let offensive = object.as_ref().and_then(|skill| skill.as_offensive_skill());
        for (target_id, target, player) in targets {
            let flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
            let (amount, context) = if let Some(fixed) = source.fixed_damage {
                let element = match metadata.element(ground.level) {
                    Some("Weapon" | "Endowed") => server.battle_service().attack_element(&source.status, None),
                    Some("Random") => [Element::Neutral, Element::Water, Element::Earth, Element::Fire, Element::Wind, Element::Poison,
                        Element::Holy, Element::Dark, Element::Ghost, Element::Undead][fastrand::usize(0..10)],
                    Some(name) => Element::try_from_string(name).unwrap_or(Element::Neutral),
                    None => Element::Neutral,
                };
                let amount = if !fixed.ignore_infinite_defense && server.battle_service().is_infinite_defense(&target, flags) {
                    i32::from(fixed.hits.max(1))
                } else { (fixed.amount as f64 * f64::from(crate::server::service::battle_service::BattleService::element_modifier(&element, &target)))
                    .floor().clamp(i32::MIN as f64, i32::MAX as f64) as i32 };
                (amount, None)
            } else if let Some(offensive) = offensive {
                server.battle_service().calculate_damage_with_context(&source.status, &target, Some(offensive))
            } else { continue; };
            let mut damage = Damage { target_id, attacker_id: source.actor_id, credit_id: source.owner_id, damage: 0, healing: 0,
                right_hand_damage: None, attacked_at: tick, damage_motion: 0, battle_flags: flags, skill_id: ground.skill_id,
                skill_level: ground.level, proc_depth: ground.depth, defenses_applied: true, magic_context: context, landed: true };
            damage.set_signed_damage(amount);
            if player { server.add_to_next_tick(GameEvent::CharacterDamage(damage)); } else { instance.add_to_next_tick(MapEvent::MobDamage(damage)); }
            self.notify_actor_ground_damage(ground, target_id, amount);
        }
        ground.waves = ground.waves.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classic_land_mine_uses_dex_and_int_without_renewal_level_scaling() {
        assert_eq!(ScriptSkillService::classic_land_mine_damage(5, 25, 20), 600);
        assert_eq!(ScriptSkillService::classic_land_mine_damage(1, 0, 0), 75);
    }
}
