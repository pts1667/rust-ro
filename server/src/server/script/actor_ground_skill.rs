use std::collections::HashSet;
use std::sync::atomic::Ordering;

use models::enums::actor::CombatActorKind;
use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::map::MapActorType;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithStringValue};
use models::status_bonus::BattleFlag;

use super::ground::{GroundCell, GroundKind, GroundSkill, NEXT_GROUND_UNIT};
use super::metadata::SkillMetadata;
use super::{GroundSkillSource, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
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

    pub fn validate_actor_ground_with_options(
        &self,
        state: &ServerState,
        source: &GroundSkillSource,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
        ignore_range: bool,
        scripted: bool,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Ground skill metadata is missing")?;
        if !scripted
            && state
                .map_flags_for(&source.map, source.instance)
                .enabled(crate::server::model::map_flags::MapFlag::NoSkill)
        {
            return Err("Companion skills cannot be used on this map".into());
        }
        if level == 0 || level > metadata.max_level || source.status.hp() == 0 {
            return Err("Ground skill source or level is invalid".into());
        }
        let kind = GroundKind::from_name(&metadata.name)
            .filter(|kind| {
                scripted
                    || kind.trap()
                    || matches!(
                        kind,
                        GroundKind::ArrowShower | GroundKind::HeavenDrive | GroundKind::Thunderstorm
                    )
            })
            .ok_or("Actor ground skill has no implemented classic unit effect")?;
        if kind == GroundKind::WarpPortal {
            return Err("Warp Portal requires a player destination menu or an NPC portal command".into());
        }
        if !ignore_range && source.x.abs_diff(x).max(source.y.abs_diff(y)) > metadata.range(level).unwrap_or(1).unsigned_abs().max(1) as u16
        {
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
        if kind.trap() || kind == GroundKind::Graffiti {
            self.validate_classic_unit_placement(state, source, metadata, level, x, y, tick)?;
        }
        if kind == GroundKind::Graffiti {
            self.validate_graffiti_limit(source, tick)?;
        }
        Ok(())
    }

    pub(super) fn validate_classic_unit_placement(
        &self,
        state: &ServerState,
        source: &GroundSkillSource,
        metadata: &SkillMetadata,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let actor_kind = *source.status.combat_actor_kind();
        let actor_mask = MapActorType::from(actor_kind).as_flag();
        let configuration = &self.configuration.config().game.skill_units;
        let unit_range = metadata.unit_value("Range", level, "Size").unwrap_or(0).max(0) as u16;
        let layout_range = metadata.unit_value("Layout", level, "Size").unwrap_or(0).max(0) as u16;
        let overlaps_traps = metadata.flags.get("IsTrap").copied().unwrap_or(false) || metadata.name == "AL_WARP";
        if metadata.unit_flag("NoReiteration") && configuration.reiteration_sources & actor_mask == 0 {
            let range = layout_range.saturating_add(if actor_kind == CombatActorKind::Player { unit_range } else { 0 });
            let grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
            if grounds.iter().any(|ground| {
                ground.cast_verified
                    && (overlaps_traps && ground.kind.trap() || ground.skill_id == metadata.id)
                    && ground.map == source.map
                    && ground.instance == source.instance
                    && ground.expires_at > tick
                    && ground
                        .cells
                        .iter()
                        .any(|cell| cell.remaining_hits > 0 && cell.x.abs_diff(x).max(cell.y.abs_diff(y)) <= range)
            }) {
                return Err("An existing trap or skill unit overlaps the placement area".into());
            }
        }
        if !metadata.unit_flag("NoFootSet") {
            return Ok(());
        }
        let check_characters = configuration.nofootset_sources & actor_mask != 0;
        if !check_characters && actor_kind != CombatActorKind::Monster {
            return Ok(());
        }
        let range = unit_range.saturating_add(layout_range);
        let nearby = |actor_x: u16, actor_y: u16| actor_x.abs_diff(x).max(actor_y.abs_diff(y)) <= range;
        let map = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Ground map is unavailable")?;
        if (!matches!(actor_kind, CombatActorKind::Npc | CombatActorKind::SkillUnit) && nearby(source.x, source.y))
            || map.state().mobs().values().any(|mob| mob.hp() > 0 && nearby(mob.x, mob.y))
            || check_characters
                && (state.characters().values().any(|player| {
                    player.status.hp > 0
                        && player.current_map_name() == &source.map
                        && player.current_map_instance() == source.instance
                        && nearby(player.x, player.y)
                }) || state
                    .characters()
                    .values()
                    .filter(|owner| owner.current_map_name() == &source.map && owner.current_map_instance() == source.instance)
                    .flat_map(crate::server::service::script_world_service::companion_snapshots)
                    .any(|actor| nearby(actor.x(), actor.y())))
        {
            return Err("A living actor occupies the placement area".into());
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

    pub fn place_actor_ground_skill_with_options(
        &self,
        _server: &Server,
        state: &ServerState,
        source: GroundSkillSource,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
        ignore_range: bool,
        scripted: bool,
    ) -> Result<(), String> {
        self.validate_actor_ground_with_options(state, &source, skill_id, level, x, y, tick, ignore_range, scripted)?;
        let metadata = SkillMetadata::find(skill_id).unwrap();
        let kind = GroundKind::from_name(&metadata.name).unwrap();
        let base_duration = metadata.duration(level, false).unwrap_or(100);
        let duration = if kind == GroundKind::Meteor {
            100
        } else {
            crate::server::service::map_flag_service::ground_skill_duration(
                &state.map_flags_for(&source.map, source.instance),
                skill_id,
                base_duration,
            )
        };
        let range = if kind == GroundKind::Pneuma {
            1
        } else if kind == GroundKind::ArrowShower {
            metadata.splash(level).unwrap_or(2)
        } else {
            metadata.unit_value("Range", level, "Size").unwrap_or(0)
        }
        .max(0) as u16;
        let layout = if kind == GroundKind::Earthquake {
            metadata.splash(level).unwrap_or(5).max(0) as u16
        } else if !kind.trap() && kind != GroundKind::ArrowShower {
            metadata.unit_value("Layout", level, "Size").unwrap_or(0).max(0) as u16
        } else {
            0
        };
        let instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Ground map is unavailable")?;
        let locations = match kind {
            GroundKind::Firewall => Self::firewall_cells(source.x, source.y, x, y),
            GroundKind::Pneuma => vec![(x, y)],
            GroundKind::GrandCross => Self::grand_cross_cells(x, y),
            _ => Self::square_cells(x, y, layout),
        };
        let cells = locations
            .into_iter()
            .filter(|(x, y)| {
                *x < instance.x_size()
                    && *y < instance.y_size()
                    && instance.state().cells()[*y as usize * instance.x_size() as usize + *x as usize] & CellType::Shootable.as_flag() != 0
            })
            .map(|(x, y)| GroundCell {
                observers: Default::default(),
                id: NEXT_GROUND_UNIT.fetch_add(1, Ordering::Relaxed),
                x,
                y,
                remaining_hits: if kind.trap() {
                    3500
                } else if kind == GroundKind::Firewall {
                    4 + u16::from(level)
                } else if kind == GroundKind::GrandCross {
                    3
                } else {
                    u16::MAX
                },
            })
            .collect::<Vec<_>>();
        if cells.is_empty() {
            return Err("Ground skill has no usable cells".into());
        }
        let mut ground = GroundSkill {
            portal: None,
            message: if matches!(kind, GroundKind::TalkieBox | GroundKind::Graffiti) {
                state.get_character(source.actor_id).map_or_else(
                    || b"Boo!".to_vec(),
                    |player| player.script_skill_state.ground_skill_text.clone(),
                )
            } else {
                vec![]
            },
            capture: None,
            recovery_item: None,
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
            next_hit_at: tick
                + if matches!(kind, GroundKind::GrandCross | GroundKind::Earthquake | GroundKind::StormGust) {
                    100
                } else {
                    0
                },
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
        self.sync_ground_unit_visibility(state, &mut ground, tick);
        self.ground_skills
            .lock()
            .map_err(|_| "Ground skill state is unavailable")?
            .push(ground);
        Ok(())
    }

    pub fn activate_ground_cast(
        &self,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        generation: u64,
        tick: u128,
    ) -> Result<(), String> {
        self.validate_ground_activation(state, character, skill_id, generation, tick)?;
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

    pub fn validate_ground_activation(
        &self,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        generation: u64,
        tick: u128,
    ) -> Result<(), String> {
        if character.status.hp == 0
            || character.status.blocks_casting()
            || (generation != 0 && character.script_skill_state.cast_generation != generation)
        {
            return Err("Ground cast was interrupted".into());
        }
        Self::validate_stealth_cast(state, character, skill_id)?;
        let placement = {
            let grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
            grounds
                .iter()
                .find(|ground| {
                    ground.source_id == character.char_id
                        && ground.skill_id == skill_id
                        && ground.cast_generation == generation
                        && !ground.cast_verified
                        && ground.cast_finish_at <= tick
                        && ground.expires_at > tick
                        && ground.map == *character.current_map_name()
                        && ground.instance == character.current_map_instance()
                })
                .map(|ground| (ground.kind, ground.level, ground.cells[0].x, ground.cells[0].y))
        }
        .ok_or("Ground cast no longer has an active placement")?;
        Self::validate_skill_map(state, character, skill_id, placement.1, true)?;
        if placement.0.trap() || placement.0 == GroundKind::Graffiti {
            self.validate_actor_ground_with_options(
                state,
                &GroundSkillSource {
                    actor_id: character.char_id,
                    owner_id: character.char_id,
                    map: character.current_map_name().clone(),
                    instance: character.current_map_instance(),
                    x: character.x,
                    y: character.y,
                    status: crate::server::service::status_service::StatusService::instance().to_snapshot(&character.status),
                    raw_attack: 0,
                    fixed_damage: None,
                },
                skill_id,
                placement.1,
                placement.2,
                placement.3,
                tick,
                true,
                true,
            )?;
        }
        Ok(())
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
        ground.recovery_item = None;
        if let Some(capture) = &ground.capture {
            capture.lease.cancel();
        }
        Ok(())
    }

    pub(super) fn tick_actor_ground_skill(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick || ground.triggered || tick < ground.next_hit_at {
            return;
        }
        ground.next_hit_at = tick + 40;
        let Some(source) = ground.actor_source.as_ref().map(|source| self.trap_source(state, source)) else {
            return;
        };
        if !ground.kind.trap() && ground.kind != GroundKind::ArrowShower {
            self.tick_actor_magic_ground(server, state, ground, &source, tick);
            return;
        }
        if ground.kind == GroundKind::AnkleSnare {
            self.begin_ankle_capture(server, state, ground, &source, tick);
            return;
        }
        if ground.kind.trap() {
            self.tick_classic_trap(server, state, ground, &source, tick);
            return;
        }
        let mut targets = self
            .trap_targets(server, state, ground, &source)
            .into_iter()
            .filter(|target| ground.covers(target.x, target.y))
            .collect::<Vec<_>>();
        targets.extend(
            state
                .ground_units
                .values()
                .filter(|unit| {
                    unit.map == crate::server::model::map_instance::MapInstanceKey::new(ground.map.clone(), ground.instance)
                        && unit.alive(tick)
                        && !unit.used
                        && ground.covers(unit.x, unit.y)
                })
                .map(|unit| super::trap::GroundTrapTarget {
                    id: unit.id,
                    x: unit.x,
                    y: unit.y,
                    status: unit.status(),
                    player: false,
                }),
        );
        if targets.is_empty() {
            return;
        }
        let metadata = SkillMetadata::find(ground.skill_id).unwrap();
        let flags = metadata.battle_flags(metadata.range(ground.level).unwrap_or(1) > 3);
        let ratio = (75 + 5 * u32::from(ground.level)) as f32 / 100.0;
        let element = server.battle_service().attack_element(&source.status, None);
        for target in targets {
            let landed = server
                .battle_service()
                .skill_hits(&source.status, &target.status, ground.skill_id, ground.level);
            let amount = if !landed {
                0
            } else if *source.status.combat_actor_kind() == models::enums::actor::CombatActorKind::Player {
                server.battle_service().player_physical_skill_damage_signed(
                    &source.status,
                    &target.status,
                    ratio,
                    1,
                    true,
                    &element,
                    ground.skill_id,
                )
            } else {
                server.battle_service().actor_physical_skill_damage_signed(
                    source.raw_attack,
                    &source.status,
                    &target.status,
                    target.player,
                    ratio,
                    1,
                    &element,
                    flags,
                    ground.skill_id,
                )
            };
            let mut damage = Damage {
                notification: None,
                source_kind: *source.status.combat_actor_kind(),
                skill_damage_adjusted: false,
                healing: 0,
                right_hand_damage: None,
                target_id: target.id,
                attacker_id: source.actor_id,
                damage: 0,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: flags,
                skill_id: ground.skill_id,
                skill_level: ground.level,
                proc_depth: ground.depth,
                credit_id: source.owner_id,
                defenses_applied: true,
                magic_context: None,
                landed,
            };
            damage.set_signed_damage(amount);
            damage = damage.with_skill_notification(&ground.map, ground.instance, ground.cells[0].x, ground.cells[0].y, tick, 1, 0);
            server.add_to_next_tick(GameEvent::ScriptMapDamage(
                crate::server::model::events::game_event::ScriptMapDamage {
                    map: crate::server::model::map_instance::MapInstanceKey::new(ground.map.clone(), ground.instance),
                    damage,
                },
            ));
        }
        ground.triggered = true;
        ground.expires_at = tick + 40;
    }

    pub fn classic_land_mine_damage(level: u8, dex: u16, int: u16) -> u32 {
        (u64::from(level) * (u64::from(dex) + 75) * (100 + u64::from(int)) / 100).min(u64::from(u32::MAX)) as u32
    }

    fn tick_actor_magic_ground(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &mut GroundSkill,
        source: &GroundSkillSource,
        tick: u128,
    ) {
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return;
        };
        let Some(metadata) = SkillMetadata::find(ground.skill_id) else {
            return;
        };
        ground.next_hit_at = tick.saturating_add(ground.interval);
        if matches!(ground.kind, GroundKind::Earthquake | GroundKind::GrandCross) && ground.waves >= 3 {
            return;
        }
        if matches!(
            ground.kind,
            GroundKind::Pneuma | GroundKind::Quagmire | GroundKind::Deluge | GroundKind::LandProtector
        ) {
            return;
        }
        let mut targets = instance
            .state()
            .mobs()
            .values()
            .filter(|mob| {
                mob.hp() > 0
                    && (!mob.summoned || mob.summon_ai == 0)
                    && ground.covers(mob.x, mob.y)
                    && Self::area_skill_target_allowed(&source.status, &mob.status, ground.skill_id)
            })
            .map(|mob| (mob.id, mob.status.clone(), false))
            .collect::<Vec<_>>();
        if let Some(owner) = state.get_character(source.owner_id) {
            targets.extend(
                state
                    .characters()
                    .values()
                    .filter(|player| {
                        player.map_instance_key == owner.map_instance_key
                            && ground.covers(player.x, player.y)
                            && server.player_combat_target_allowed(state, owner, player.char_id)
                    })
                    .map(|player| {
                        (
                            player.char_id,
                            crate::server::service::status_service::StatusService::instance().to_snapshot(&player.status),
                            true,
                        )
                    }),
            );
        } else if let Some(actor) = self.find_script_skill_actor(state, source.actor_id) {
            targets = self
                .actor_area_targets(server, state, &actor, ground.skill_id, source.x, source.y, u16::MAX)
                .into_iter()
                .filter(|(id, ..)| {
                    state
                        .map_item_snapshot(*id, &ground.map, ground.instance)
                        .is_some_and(|target| ground.covers(target.x(), target.y()))
                })
                .collect();
        }
        let object = skills::skill_enums::to_object(models::enums::skill_enums::SkillEnum::from_id(ground.skill_id), ground.level);
        let offensive = object.as_ref().and_then(|skill| skill.as_offensive_skill());
        let split = targets.len() as u32;
        for (target_id, target, player) in targets {
            let flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
            let (amount, context) = if let Some(fixed) = source.fixed_damage {
                let element = match metadata.element(ground.level) {
                    Some("Weapon" | "Endowed") => server.battle_service().attack_element(&source.status, None),
                    Some("Random") => [
                        Element::Neutral,
                        Element::Water,
                        Element::Earth,
                        Element::Fire,
                        Element::Wind,
                        Element::Poison,
                        Element::Holy,
                        Element::Dark,
                        Element::Ghost,
                        Element::Undead,
                    ][fastrand::usize(0..10)],
                    Some(name) => Element::try_from_string(name).unwrap_or(Element::Neutral),
                    None => Element::Neutral,
                };
                let amount = if !fixed.ignore_infinite_defense && server.battle_service().is_infinite_defense(&target, flags) {
                    i32::from(fixed.hits.max(1))
                } else {
                    (fixed.amount as f64
                        * f64::from(crate::server::service::battle_service::BattleService::element_modifier(
                            &element, &target,
                        )))
                    .floor()
                    .clamp(i32::MIN as f64, i32::MAX as f64) as i32
                };
                (amount, None)
            } else if ground.kind == GroundKind::Earthquake {
                let (raw, ratio) = Self::earthquake_attack(&source.status, ground.level, split);
                let context = crate::server::service::map_combat_service::MagicAttackContext::new(
                    raw,
                    ratio,
                    Element::Neutral,
                    1,
                    ground.skill_id,
                );
                (server.battle_service().magic_damage_from_context(&source.status, &target, context), Some(context))
            } else if ground.kind == GroundKind::GrandCross {
                let (amount, context) =
                    server
                        .battle_service()
                        .grand_cross_damage_signed_with_context(&source.status, &target, ground.level, false);
                (amount, Some(context))
            } else if let Some(offensive) = offensive {
                server
                    .battle_service()
                    .calculate_damage_with_context(&source.status, &target, Some(offensive))
            } else {
                continue;
            };
            let mut damage = Damage {
                notification: None,
                source_kind: *source.status.combat_actor_kind(),
                skill_damage_adjusted: false,
                target_id,
                attacker_id: source.actor_id,
                credit_id: source.owner_id,
                damage: 0,
                healing: 0,
                right_hand_damage: None,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: flags,
                skill_id: ground.skill_id,
                skill_level: ground.level,
                proc_depth: ground.depth,
                defenses_applied: true,
                magic_context: context,
                landed: true,
            };
            damage.set_signed_damage(amount);
            damage = damage.with_skill_notification(
                &ground.map,
                ground.instance,
                ground.cells[0].x,
                ground.cells[0].y,
                tick,
                source.fixed_damage.map_or(1, |fixed| fixed.hits.max(1)),
                0,
            );
            if player {
                server.add_to_next_tick(GameEvent::CharacterDamage(damage));
            } else {
                instance.add_to_next_tick(MapEvent::MobDamage(damage));
            }
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
