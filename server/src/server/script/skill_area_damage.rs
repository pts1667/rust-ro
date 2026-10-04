use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};

use models::enums::cell::CellType;
use models::enums::mob::MobMode;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithNumberValue};

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_item::MapItemType;
use crate::server::service::battle_service::BattleService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

static NEXT_WATER_SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(super) struct WaterBallSequence {
    pub id: u64,
    pub target_id: u32,
    pub map: String,
    pub instance: u8,
    pub count: u16,
    pub expires_at: u128,
}

#[derive(Clone, Copy, Debug)]
pub struct BowlingTarget {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub immovable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BowlingCollision {
    pub id: u32,
    pub hits: u8,
    pub moved: u16,
    pub direction: (i32, i32),
    pub origin: (u16, u16),
}

impl ScriptSkillService {
    pub fn complete_damage_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        original: Damage,
        battle: &BattleService,
        tick: u128,
    ) -> Result<Vec<(MapItemType, Damage)>, String> {
        if self.queue_weapon_aftermath(server, state, character, &original, tick) {
            return Ok(vec![]);
        }
        let Some(metadata) = SkillMetadata::find(original.skill_id) else {
            return Ok(vec![(MapItemType::Mob, original)]);
        };
        if metadata.name == "SM_MAGNUM" {
            let mut request = models::status_change::StatusChangeRequest::guaranteed(
                models::status_change::StatusChangeKind::WeaponAttackElement,
                metadata.duration(original.skill_level, true).unwrap_or(10000),
                models::enums::element::Element::Fire.value() as i32,
            );
            request.values[1] = 20;
            server.add_to_next_tick(GameEvent::CharacterStatusChange(
                crate::server::model::events::game_event::CharacterStatusChange {
                    char_id: character.char_id,
                    request,
                },
            ));
        }
        if metadata.name == "WZ_WATERBALL" {
            self.begin_water_ball(server, state, character, original, tick)?;
            return Ok(vec![]);
        }
        let self_area = metadata.target_type.as_deref() == Some("Self") && metadata.damage_flags.get("Splash").copied().unwrap_or(false);
        let target = if original.target_id == character.char_id {
            Some((character.x, character.y, MapItemType::Character))
        } else {
            state
                .map_item_snapshot(
                    original.target_id,
                    character.current_map_name(),
                    character.current_map_instance(),
                )
                .map(|target| (target.position.x, target.position.y, *target.map_item.object_type()))
        };
        let Some((x, y, target_kind)) = target else {
            return Err("Skill target is no longer on the map".into());
        };
        let splash = metadata.splash(original.skill_level).unwrap_or(0).max(0) as u16;
        if !metadata.damage_flags.get("Splash").copied().unwrap_or(false) || splash == 0 {
            return Ok(vec![(target_kind, original)]);
        }
        let (x, y) = if self_area { (character.x, character.y) } else { (x, y) };
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        let map_state = instance.state();
        let source = StatusService::instance().to_snapshot(&character.status);
        let skill = skills::skill_enums::to_object(SkillEnum::from_id(original.skill_id), original.skill_level)
            .ok_or("Splash skill has no damage implementation")?;
        let offensive = skill.as_offensive_skill().ok_or("Splash skill has no offensive implementation")?;
        let mut collisions = None;
        if metadata.name == "KN_BOWLINGBASH" {
            let targets = map_state
                .mobs()
                .values()
                .filter(|mob| mob.status.hp() > 0)
                .map(|mob| BowlingTarget {
                    id: mob.id,
                    x: mob.x,
                    y: mob.y,
                    immovable: mob.mode & MobMode::Boss.as_flag() != 0,
                })
                .collect::<Vec<_>>();
            collisions = Some(Self::bowling_collisions(
                character.x,
                character.y,
                character.dir,
                original.skill_level,
                original.target_id,
                targets,
                |x, y| {
                    x < instance.x_size()
                        && y < instance.y_size()
                        && map_state.cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Walkable.as_flag() != 0
                },
            ));
        }
        let targets = map_state
            .mobs()
            .values()
            .filter(|mob| {
                mob.status.hp() > 0
                    && (!mob.summoned || mob.summon_ai == 0)
                    && Self::area_skill_target_allowed(&source, &mob.status, original.skill_id)
                    && collisions.as_ref().map_or_else(
                        || mob.x.abs_diff(x).max(mob.y.abs_diff(y)) <= splash,
                        |collisions| collisions.iter().any(|hit| hit.id == mob.id),
                    )
            })
            .collect::<Vec<_>>();
        let divisor = if metadata.damage_flags.get("SplashSplit").copied().unwrap_or(false) {
            targets.len().max(1) as u32
        } else {
            1
        };
        let mut damages = vec![];
        for target in targets {
            let use_original = target.id == original.target_id && !self_area;
            let landed = if use_original {
                original.landed
            } else {
                !crate::server::service::battle_service::BattleService::is_weapon_skill(offensive) || battle.skill_hits(&source, &target.status, original.skill_id, original.skill_level)
            };
            let (damage, magic_context) = if use_original {
                (
                    if original.healing > 0 {
                        -(original.healing.min(i32::MAX as u32) as i32)
                    } else {
                        original.damage.min(i32::MAX as u32) as i32
                    },
                    original.magic_context,
                )
            } else if landed {
                battle.calculate_damage_with_context(&source, &target.status, Some(offensive))
            } else {
                (0, None)
            };
            let mut damage_event = original.clone();
            damage_event.target_id = target.id;
            damage_event.set_signed_damage(damage / divisor.max(1).min(i32::MAX as u32) as i32);
            damage_event.magic_context = magic_context;
            damage_event.landed = landed;
            if metadata.name == "SM_MAGNUM" && landed {
                let ratio = 1.0
                    + f32::from(original.skill_level)
                        * if target.x.abs_diff(x).max(target.y.abs_diff(y)) <= 1 {
                            0.2
                        } else {
                            0.1
                        };
                damage_event.set_signed_damage(battle.player_physical_skill_damage_signed(
                    &source,
                    &target.status,
                    ratio,
                    1,
                    false,
                    &models::enums::element::Element::Fire,
                    original.skill_id,
                ));
            }
            if metadata.name == "MG_FIREBALL" && target.x.abs_diff(x).max(target.y.abs_diff(y)) == 2 {
                if let Some(mut context) = damage_event.magic_context {
                    context.modifier *= 0.75;
                    damage_event.set_signed_damage(
                        battle.magic_damage_from_context(&source, &target.status, context) / divisor.max(1).min(i32::MAX as u32) as i32,
                    );
                    damage_event.magic_context = Some(context);
                }
            }
            if let Some(collisions) = &collisions {
                let collision = collisions.iter().find(|collision| collision.id == target.id).unwrap();
                let hits = offensive.hit_count().unsigned_abs().max(1) as u32;
                damage_event.damage /= hits;
                damage_event.healing /= hits;
                for _ in 0..collision.hits {
                    damages.push((MapItemType::Mob, damage_event.clone()));
                }
                if landed && collision.moved > 0 {
                    instance.add_to_next_tick(MapEvent::MobKnockback {
                        mob_id: target.id,
                        source_x: (collision.origin.0 as i32 - collision.direction.0).clamp(0, u16::MAX as i32) as u16,
                        source_y: (collision.origin.1 as i32 - collision.direction.1).clamp(0, u16::MAX as i32) as u16,
                        cells: collision.moved,
                    });
                }
            } else {
                damages.push((MapItemType::Mob, damage_event));
            }
        }
        if !self_area && target_kind == MapItemType::Character {
            damages.push((target_kind, original));
        }
        let _ = tick;
        Ok(damages)
    }

    pub fn validate_native_environment(
        &self,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        tick: u128,
    ) -> Result<(), String> {
        Self::validate_stealth_cast(state, character, skill_id)?;
        let issued = character.pending_item_skill.as_ref().is_some_and(|pending| pending.item_index.is_some() || pending.source_item.is_some());
        Self::validate_skill_map(state, character, skill_id, level, issued)?;
        if skill_id == SkillEnum::WzWaterball.id() {
            let instance = state
                .get_map_instance_from_character(character)
                .ok_or("Water Ball map is unavailable")?;
            let map_state = instance.state();
            let water = map_state
                .cells()
                .get(character.y as usize * instance.x_size() as usize + character.x as usize)
                .is_some_and(|cell| cell & CellType::Water.as_flag() != 0);
            if !water && !self.ground_field_contains(character, super::ground::GroundKind::Deluge, character.x, character.y, tick) {
                return Err("Water Ball requires standing in water or Deluge".into());
            }
            if level == 0 || level > 5 {
                return Err("Water Ball level is invalid".into());
            }
        }
        Ok(())
    }

    pub fn water_ball_cells(
        &self,
        state: &ServerState,
        character: &Character,
        level: u8,
        tick: u128,
        consume: bool,
    ) -> Result<Vec<(u16, u16)>, String> {
        let metadata = SkillMetadata::find(SkillEnum::WzWaterball.id()).ok_or("Water Ball metadata is missing")?;
        let radius = metadata.unit_value("Layout", level, "Size").unwrap_or(0).max(0) as u16;
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Water Ball map is unavailable")?;
        let map_state = instance.state();
        let mut grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        let mut cells = vec![];
        for (x, y) in Self::square_cells(character.x, character.y, radius)
            .into_iter()
            .filter(|(x, y)| *x < instance.x_size() && *y < instance.y_size())
        {
            if !Self::shootable_line(
                instance.x_size(),
                instance.y_size(),
                map_state.cells(),
                character.x,
                character.y,
                x,
                y,
            ) {
                continue;
            }
            let natural = map_state.cells()[y as usize * instance.x_size() as usize + x as usize] & CellType::Water.as_flag() != 0;
            let mut deluge = false;
            for ground in grounds.iter_mut().filter(|ground| {
                ground.kind == super::ground::GroundKind::Deluge
                    && ground.map == *character.current_map_name()
                    && ground.instance == character.current_map_instance()
                    && ground.cast_verified
                    && ground.active_from <= tick
                    && ground.expires_at > tick
            }) {
                for cell in ground
                    .cells
                    .iter_mut()
                    .filter(|cell| cell.remaining_hits > 0 && cell.x == x && cell.y == y)
                {
                    deluge = true;
                    if consume {
                        cell.remaining_hits = 0;
                    }
                }
            }
            if natural || deluge {
                cells.push((x, y));
            }
        }
        Ok(cells)
    }

    fn begin_water_ball(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        original: Damage,
        tick: u128,
    ) -> Result<(), String> {
        let cells = self.water_ball_cells(state, character, original.skill_level, tick, true)?;
        let id = NEXT_WATER_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let count = cells.len().min(u16::MAX as usize) as u16;
        self.water_ball_sequences
            .lock()
            .map_err(|_| "Water Ball sequence state is unavailable")?
            .insert(character.char_id, WaterBallSequence {
                id,
                target_id: original.target_id,
                map: character.current_map_name().clone(),
                instance: character.current_map_instance(),
                count,
                expires_at: tick + u128::from(count) * 150 + 1000,
            });
        for cell in 0..count {
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: original.target_id,
                skill_id: original.skill_id,
                level: original.skill_level,
                heal_value: 0,
                proc_depth: original.proc_depth,
                skill_event_emitted: true,
                cast_generation: 0,
                action: ScriptSkillAction::WaterBall { sequence: id, cell },
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            let delay = u128::from(cell + 1) * 150;
            server.add_to_tick(
                GameEvent::CharacterScriptSkill(effect),
                delay.div_ceil(40).saturating_sub(1) as usize,
            );
        }
        if count > 0 {
            let end = tick + u128::from(count) * 150;
            character.timing.set_canact_tick(
                character.timing.get_canact_tick().max(
                    end + u128::from(StatusService::instance().attack_motion(&StatusService::instance().to_snapshot(&character.status))),
                ),
            );
            character.timing.set_canmove_tick(character.timing.get_canmove_tick().max(end));
        }
        Ok(())
    }

    pub(super) fn validate_water_ball_shot(
        &self,
        character: &Character,
        effect: &ScriptSkillEffect,
        sequence: u64,
        cell: u16,
        tick: u128,
    ) -> Result<(), String> {
        let sequences = self
            .water_ball_sequences
            .lock()
            .map_err(|_| "Water Ball sequence state is unavailable")?;
        if sequences.get(&character.char_id).is_some_and(|active| {
            active.id == sequence
                && active.target_id == effect.target_id
                && active.count > cell
                && active.expires_at > tick
                && active.map == *character.current_map_name()
                && active.instance == character.current_map_instance()
        }) {
            Ok(())
        } else {
            Err("Water Ball sequence is no longer active".into())
        }
    }

    pub(super) fn water_ball_shot(
        &self,
        server: &Server,
        state: &ServerState,
        source: &Character,
        effect: &ScriptSkillEffect,
        sequence: u64,
        cell: u16,
        player_target: Option<&Character>,
        tick: u128,
    ) -> Result<(), String> {
        self.validate_water_ball_shot(source, effect, sequence, cell, tick)?;
        let instance = state
            .get_map_instance_from_character(source)
            .ok_or("Water Ball map is unavailable")?;
        let map_state = instance.state();
        let target = player_target
            .map(|target| {
                (
                    target.x,
                    target.y,
                    StatusService::instance().to_snapshot(&target.status),
                    MapItemType::Character,
                )
            })
            .or_else(|| {
                state.get_character(effect.target_id).map(|target| {
                    (
                        target.x,
                        target.y,
                        StatusService::instance().to_snapshot(&target.status),
                        MapItemType::Character,
                    )
                })
            })
            .or_else(|| {
                map_state
                    .get_mob(effect.target_id)
                    .map(|target| (target.x, target.y, target.status.clone(), MapItemType::Mob))
            })
            .ok_or("Water Ball target disappeared")?;
        if target.2.hp() == 0 {
            return Ok(());
        }
        if Self::shootable_line(
            instance.x_size(),
            instance.y_size(),
            map_state.cells(),
            source.x,
            source.y,
            target.0,
            target.1,
        ) {
            let snapshot = StatusService::instance().to_snapshot(&source.status);
            let skill =
                skills::skill_enums::to_object(SkillEnum::WzWaterball, effect.level).ok_or("Water Ball implementation is unavailable")?;
            let offensive = skill
                .as_offensive_skill()
                .ok_or("Water Ball damage implementation is unavailable")?;
            let (amount, magic_context) = server
                .battle_service()
                .calculate_damage_with_context(&snapshot, &target.2, Some(offensive));
            let mut damage = Damage {
                healing: 0,
                right_hand_damage: None,
                target_id: effect.target_id,
                attacker_id: source.char_id,
                damage: 0,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: models::status_bonus::BattleFlag::Magic.as_flag()
                    | models::status_bonus::BattleFlag::Long.as_flag()
                    | models::status_bonus::BattleFlag::Skill.as_flag(),
                skill_id: effect.skill_id,
                skill_level: effect.level,
                proc_depth: effect.proc_depth,
                credit_id: source.char_id,
                defenses_applied: true,
                magic_context,
                landed: true,
            };
            damage.set_signed_damage(amount);
            if target.3 == MapItemType::Mob {
                instance.add_to_next_tick(MapEvent::MobDamage(damage));
            } else {
                server.add_to_next_tick(GameEvent::CharacterDamage(damage));
            }
            self.notify_attack_skill(source, effect.target_id, effect.skill_id, effect.level, amount);
        }
        if let Ok(mut sequences) = self.water_ball_sequences.lock() {
            if sequences.get(&source.char_id).is_some_and(|active| cell + 1 == active.count) {
                sequences.remove(&source.char_id);
            }
        }
        Ok(())
    }

    pub fn shootable_line(width: u16, height: u16, cells: &[u16], start_x: u16, start_y: u16, target_x: u16, target_y: u16) -> bool {
        let (mut x, mut y) = (i32::from(start_x), i32::from(start_y));
        let dx = (i32::from(target_x) - x).abs();
        let dy = -(i32::from(target_y) - y).abs();
        let (sx, sy) = ((i32::from(target_x) - x).signum(), (i32::from(target_y) - y).signum());
        let mut error = dx + dy;
        loop {
            if x < 0
                || y < 0
                || x >= i32::from(width)
                || y >= i32::from(height)
                || cells
                    .get(y as usize * width as usize + x as usize)
                    .is_none_or(|cell| cell & CellType::Shootable.as_flag() == 0)
            {
                return false;
            }
            if x == i32::from(target_x) && y == i32::from(target_y) {
                return true;
            }
            let twice = 2 * error;
            if twice >= dy {
                error += dy;
                x += sx;
            }
            if twice <= dx {
                error += dx;
                y += sy;
            }
        }
    }

    pub fn bowling_collisions(
        source_x: u16,
        source_y: u16,
        direction: u16,
        level: u8,
        first_id: u32,
        targets: Vec<BowlingTarget>,
        walkable: impl Fn(u16, u16) -> bool,
    ) -> Vec<BowlingCollision> {
        let mut targets = targets.into_iter().map(|target| (target.id, target)).collect::<HashMap<_, _>>();
        let mut seen = HashSet::new();
        let mut result = vec![];
        fn hit(
            id: u32,
            depth: u8,
            source_x: u16,
            source_y: u16,
            direction: u16,
            level: u8,
            targets: &mut HashMap<u32, BowlingTarget>,
            seen: &mut HashSet<u32>,
            result: &mut Vec<BowlingCollision>,
            walkable: &impl Fn(u16, u16) -> bool,
        ) {
            let Some(mut target) = targets.get(&id).copied() else {
                return;
            };
            let distance = (level.saturating_sub(depth) as i32 + 1) / 2;
            let min_x = ((source_x as i32 - distance) / 40 * 40).max(0);
            let min_y = ((source_y as i32 - distance) / 40 * 40).max(0);
            let inside = |x: i32, y: i32| (min_x..=min_x + 39).contains(&x) && (min_y..=min_y + 39).contains(&y);
            if depth > 0 && (!inside(target.x as i32, target.y as i32) || seen.contains(&id)) {
                return;
            }
            seen.insert(id);
            let vector = ScriptSkillService::facing_vector(if depth == 0 { direction } else { fastrand::u16(0..8) });
            let origin = (target.x, target.y);
            let mut collision = BowlingCollision {
                id,
                hits: 1,
                moved: 0,
                direction: vector,
                origin,
            };
            let (mut x, mut y) = (target.x as i32, target.y as i32);
            for _ in 0..distance {
                x += vector.0;
                y += vector.1;
                let (Ok(nx), Ok(ny)) = (u16::try_from(x), u16::try_from(y)) else {
                    break;
                };
                if !walkable(nx, ny) {
                    break;
                }
                if !target.immovable {
                    target.x = nx;
                    target.y = ny;
                    collision.moved += 1;
                    targets.insert(id, target);
                }
                let affected = targets
                    .values()
                    .filter(|other| inside(other.x as i32, other.y as i32) && other.x.abs_diff(nx).max(other.y.abs_diff(ny)) <= 1)
                    .map(|other| other.id)
                    .collect::<Vec<_>>();
                if !affected.is_empty() {
                    for other in affected {
                        hit(
                            other,
                            depth.saturating_add(1),
                            source_x,
                            source_y,
                            direction,
                            level,
                            targets,
                            seen,
                            result,
                            walkable,
                        );
                    }
                    if inside(target.x as i32, target.y as i32) {
                        collision.hits += 1;
                    }
                    break;
                }
            }
            result.push(collision);
        }
        hit(
            first_id,
            0,
            source_x,
            source_y,
            direction,
            level,
            &mut targets,
            &mut seen,
            &mut result,
            &walkable,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classic_bowling_bash_doubles_inside_a_gutter_region_and_chains_once() {
        let targets = vec![
            BowlingTarget {
                id: 1,
                x: 13,
                y: 10,
                immovable: false,
            },
            BowlingTarget {
                id: 2,
                x: 15,
                y: 10,
                immovable: false,
            },
        ];
        let hits = ScriptSkillService::bowling_collisions(10, 10, 6, 10, 1, targets, |_, _| true);
        assert_eq!(hits.len(), 2);
        assert!(hits.iter().all(|hit| hit.hits == 2));
    }
    #[test]
    fn gutter_edges_and_walls_prevent_self_collision_second_hit() {
        let targets = vec![BowlingTarget {
            id: 1,
            x: 41,
            y: 10,
            immovable: false,
        }];
        let hits = ScriptSkillService::bowling_collisions(39, 10, 6, 10, 1, targets, |_, _| true);
        assert_eq!(hits[0].hits, 1);
        let targets = vec![BowlingTarget {
            id: 1,
            x: 13,
            y: 10,
            immovable: false,
        }];
        let hits = ScriptSkillService::bowling_collisions(10, 10, 6, 10, 1, targets, |_, _| false);
        assert_eq!(hits[0].hits, 1);
        assert_eq!(hits[0].moved, 0);
    }
}
