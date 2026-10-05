use models::enums::EnumWithMaskValueU32;
use models::enums::mob::MobCapability;
use models::status::StatusSnapshot;
use models::status_change::{StatusChange, StatusChangeKind, StatusChangeRequest, StatusStartFlag};

use super::ScriptSkillService;
use super::ground::{GroundKind, GroundSkill, GroundSkillSource};
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::events::game_event::{GameEvent, GroundTrapSpend};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::service::script_world_service::companion_status_snapshot;
use crate::server::service::status_service::StatusService;
use crate::server::state::server::ServerState;

#[path = "ground_trap_control.rs"]
mod control;
#[path = "ground_unit.rs"]
mod unit;

#[derive(Clone, Debug)]
pub struct TrapLease(std::sync::Arc<std::sync::atomic::AtomicBool>);

impl PartialEq for TrapLease {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}

impl TrapLease {
    fn new() -> Self {
        Self(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true)))
    }

    pub(crate) fn active(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }

    pub(super) fn cancel(&self) {
        self.0.store(false, std::sync::atomic::Ordering::Release);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroundTrapCapture {
    pub lease: TrapLease,
    pub deadline: u128,
    pub map: MapInstanceKey,
    pub target_id: u32,
    pub trap_id: u32,
    pub x: u16,
    pub y: u16,
    pub request: StatusChangeRequest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroundTrapRelease {
    pub map: MapInstanceKey,
    pub target_id: u32,
    pub trap_id: u32,
}

pub struct TrapCaptureState {
    pub lease: TrapLease,
    pub target_id: u32,
    pub trap_id: u32,
    pub confirmed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroundTrapEffect {
    pub map: MapInstanceKey,
    pub target_id: u32,
    pub kind: GroundTrapEffectKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GroundTrapEffectKind {
    Status(StatusChangeRequest),
    DrainSp { percent: u16 },
    Knockback { source_x: u16, source_y: u16, cells: u16 },
}

pub(super) struct GroundTrapTarget {
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub status: StatusSnapshot,
    pub player: bool,
}

pub(crate) fn linked_ankle(state: &ServerState, map: &MapInstanceKey, target_id: u32, trap_id: u32) -> Option<StatusChange> {
    let matches = |change: &&StatusChange| change.kind == StatusChangeKind::Ankle && change.values[1] == trap_id as i32;
    if let Some(character) = state
        .get_character(target_id)
        .filter(|character| character.map_instance_key == *map)
    {
        return character.status.active_statuses.iter().find(matches).cloned();
    }
    if let Some(owner) = state.companion_owner(target_id, map.map_name(), map.map_instance()) {
        if let Some(status) = companion_status_snapshot(owner, target_id) {
            return status.active_statuses().iter().find(matches).cloned();
        }
    }
    let instance = state.get_map_instance(map.map_name(), map.map_instance())?;
    let state = instance.state();
    if let Some(mob) = state.get_mob(target_id) {
        return mob.status_effects.active_statuses.iter().find(matches).cloned();
    }
    state
        .script_skill_state
        .npcs
        .get(&target_id)?
        .active_statuses
        .iter()
        .find(matches)
        .cloned()
}

impl ScriptSkillService {
    pub(super) fn tick_classic_trap(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &mut GroundSkill,
        source: &GroundSkillSource,
        tick: u128,
    ) {
        use models::enums::mob::{MobClass, MobRace};
        let targets = self.trap_targets(server, state, ground, source);
        let maximum_triggers = if self.configuration.config().game.skill_units.multi_trigger_traps && ground.kind != GroundKind::SkidTrap {
            usize::MAX
        } else {
            1
        };
        let triggers = targets
            .iter()
            .filter(|target| {
                ground.covers(target.x, target.y)
                    && (ground.kind != GroundKind::Flasher
                        || *target.status.mob_class() != MobClass::Boss
                        || *target.status.race() != MobRace::Plant)
            })
            .take(maximum_triggers)
            .map(|target| (target.id, target.x, target.y))
            .collect::<Vec<_>>();
        if triggers.is_empty() {
            return;
        }
        for trigger in triggers {
            self.trigger_classic_trap(server, state, ground, source, &targets, trigger, tick);
        }
        ground.triggered = true;
        ground.recovery_item = None;
        ground.expires_at = tick + 1500;
        self.change_trap_view(ground, 140);
    }

    fn trigger_classic_trap(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &GroundSkill,
        source: &GroundSkillSource,
        targets: &[GroundTrapTarget],
        trigger: (u32, u16, u16),
        tick: u128,
    ) {
        use models::enums::actor::CombatActorKind;
        use models::enums::element::Element;

        use crate::server::model::action::Damage;
        use crate::server::model::events::game_event::ScriptMapDamage;
        let center = if matches!(ground.kind, GroundKind::Sandman | GroundKind::FreezingTrap) {
            (trigger.1, trigger.2)
        } else {
            (ground.cells[0].x, ground.cells[0].y)
        };
        let trigger_id = trigger.0;
        let metadata = SkillMetadata::find(ground.skill_id).unwrap();
        let radius = metadata.splash(ground.level).unwrap_or(0).max(0) as u16;
        let flags = metadata.battle_flags(metadata.range(ground.level).unwrap_or(1) > 3);
        let map = MapInstanceKey::new(ground.map.clone(), ground.instance);
        for target in targets.iter().filter(|target| {
            if radius == 0 {
                target.id == trigger_id
            } else {
                target.x.abs_diff(center.0).max(target.y.abs_diff(center.1)) <= radius
            }
        }) {
            let (effect, delay) = match ground.kind {
                GroundKind::SkidTrap => (
                    Some(GroundTrapEffectKind::Knockback {
                        source_x: ground.source_x,
                        source_y: ground.source_y,
                        cells: metadata
                            .knockback
                            .as_ref()
                            .and_then(|value| value.value(ground.level, "Amount"))
                            .unwrap_or(2)
                            .max(0) as u16,
                    }),
                    0,
                ),
                GroundKind::Sandman | GroundKind::Flasher => (
                    Some(GroundTrapEffectKind::Status(StatusChangeRequest {
                        kind: if ground.kind == GroundKind::Sandman {
                            StatusChangeKind::Sleep
                        } else {
                            StatusChangeKind::Blind
                        },
                        duration_ms: metadata.duration(ground.level, true).unwrap_or(30000),
                        values: [i32::from(ground.level), 0, 0, 0],
                        rate: if ground.kind == GroundKind::Sandman {
                            (40 + 10 * u16::from(ground.level)) * 100
                        } else {
                            10000
                        },
                        flags: 0,
                    })),
                    1000,
                ),
                GroundKind::Shockwave => (
                    Some(GroundTrapEffectKind::DrainSp {
                        percent: 15 * u16::from(ground.level) + 5,
                    }),
                    0,
                ),
                _ => (None, 0),
            };
            if let Some(kind) = effect {
                let event = GameEvent::GroundTrapEffect(GroundTrapEffect {
                    map: map.clone(),
                    target_id: target.id,
                    kind,
                });
                if delay > 0 {
                    server.add_to_delayed_tick(event, delay);
                } else {
                    server.add_to_next_tick(event);
                }
                continue;
            }
            let landed = ground.kind != GroundKind::FreezingTrap
                || server
                    .battle_service()
                    .skill_hits(&source.status, &target.status, ground.skill_id, ground.level);
            let amount = if !landed {
                0
            } else if ground.kind == GroundKind::FreezingTrap {
                if *source.status.combat_actor_kind() == CombatActorKind::Player {
                    server.battle_service().player_physical_skill_damage_signed(
                        &source.status,
                        &target.status,
                        1.0,
                        1,
                        false,
                        &Element::Water,
                        ground.skill_id,
                    )
                } else {
                    server.battle_service().actor_physical_skill_damage_signed(
                        source.raw_attack,
                        &source.status,
                        &target.status,
                        target.player,
                        1.0,
                        1,
                        &Element::Water,
                        flags,
                        ground.skill_id,
                    )
                }
            } else {
                let (raw, element) = match ground.kind {
                    GroundKind::LandMine => (
                        Self::classic_land_mine_damage(ground.level, source.status.dex(), source.status.int()),
                        Element::Earth,
                    ),
                    GroundKind::BlastMine => (
                        Self::classic_splash_trap_damage(ground.level, source.status.dex(), source.status.int(), 50),
                        Element::Wind,
                    ),
                    GroundKind::ClaymoreTrap => (
                        Self::classic_splash_trap_damage(ground.level, source.status.dex(), source.status.int(), 75),
                        Element::Fire,
                    ),
                    _ => continue,
                };
                server.battle_service().actor_misc_skill_damage_signed(
                    raw,
                    &source.status,
                    &target.status,
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
            damage = damage.with_skill_notification(&ground.map, ground.instance, center.0, center.1, tick, 1, 0);
            server.add_to_next_tick(GameEvent::ScriptMapDamage(ScriptMapDamage { map: map.clone(), damage }));
        }
        if ground.kind == GroundKind::ClaymoreTrap {
            let raw = Self::classic_splash_trap_damage(ground.level, source.status.dex(), source.status.int(), 75);
            for unit in state.ground_units.values().filter(|unit| {
                unit.map == map
                    && unit.alive(tick)
                    && !unit.used
                    && !ground.cells.iter().any(|cell| cell.id == unit.id)
                    && unit.x.abs_diff(center.0).max(unit.y.abs_diff(center.1)) <= radius
            }) {
                let amount = server.battle_service().actor_misc_skill_damage_signed(
                    raw,
                    &source.status,
                    &unit.status(),
                    &Element::Fire,
                    flags,
                    ground.skill_id,
                );
                let mut damage = Damage {
                    notification: None,
                    source_kind: *source.status.combat_actor_kind(),
                    skill_damage_adjusted: false,
                    healing: 0,
                    right_hand_damage: None,
                    target_id: unit.id,
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
                    landed: true,
                };
                damage.set_signed_damage(amount);
                damage = damage.with_skill_notification(&ground.map, ground.instance, center.0, center.1, tick, 1, 0);
                server.add_to_next_tick(GameEvent::GroundTrapSpend(GroundTrapSpend {
                    map: map.clone(),
                    unit_id: unit.id,
                }));
                server.add_to_next_tick(GameEvent::ScriptMapDamage(ScriptMapDamage { map: map.clone(), damage }));
            }
        }
    }

    pub(super) fn classic_splash_trap_damage(level: u8, dex: u16, int: u16, base: u16) -> u32 {
        (u64::from(level) * (u64::from(dex) + 2 * u64::from(base)) * (100 + u64::from(int)) / 200).min(u64::from(u32::MAX)) as u32
    }

    pub(super) fn trap_targets(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &GroundSkill,
        source: &GroundSkillSource,
    ) -> Vec<GroundTrapTarget> {
        use crate::server::model::map_item::MapItemType;
        use crate::server::service::script_world_service::companion_snapshots;
        let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
            return vec![];
        };
        let metadata = SkillMetadata::find(ground.skill_id).unwrap();
        let radius = ground
            .effect_range
            .saturating_add(metadata.splash(ground.level).unwrap_or(0).max(0) as u16);
        let center = &ground.cells[0];
        let near = |x: u16, y: u16| x.abs_diff(center.x).max(y.abs_diff(center.y)) <= radius;
        let owner = state.get_character(source.owner_id);
        let hit_all = ground.kind.trap()
            && *source.status.combat_actor_kind() == models::enums::actor::CombatActorKind::Player
            && self.configuration.config().game.skill_units.traps_target_all_on_versus
            && state.map_flags_for(&ground.map, ground.instance).versus(state.siege_active);
        let allowed = |id| {
            if hit_all {
                return true;
            }
            id != source.actor_id
                && owner.is_none_or(|owner| server.player_ground_target_allowed(state, owner, id, ground.kind == GroundKind::AnkleSnare))
        };
        let map = instance.state();
        let allied_mob = map.get_mob(source.actor_id).is_some_and(|mob| mob.summon_ai != 0);
        let monster = *source.status.combat_actor_kind() == models::enums::actor::CombatActorKind::Monster;
        let mut targets = vec![];
        if !monster || allied_mob {
            targets.extend(
                map.mobs()
                    .values()
                    .filter(|mob| {
                        mob.is_present()
                            && mob.hp() > 0
                            && (hit_all || !mob.summoned || mob.summon_ai == 0)
                            && near(mob.x, mob.y)
                            && allowed(mob.id)
                    })
                    .map(|mob| GroundTrapTarget {
                        id: mob.id,
                        x: mob.x,
                        y: mob.y,
                        status: mob.status.clone(),
                        player: false,
                    }),
            );
        }
        for player in state.characters().values().filter(|player| {
            player.current_map_name() == &ground.map && player.current_map_instance() == ground.instance && player.status.hp > 0
        }) {
            if near(player.x, player.y) && allowed(player.char_id) && !allied_mob {
                targets.push(GroundTrapTarget {
                    id: player.char_id,
                    x: player.x,
                    y: player.y,
                    status: StatusService::instance().to_snapshot(&player.status),
                    player: true,
                });
            }
            for actor in companion_snapshots(player).into_iter().filter(|actor| {
                *actor.map_item().object_type() != MapItemType::Pet
                    && near(actor.x(), actor.y())
                    && allowed(actor.map_item().id())
                    && !allied_mob
            }) {
                if let Some(status) = companion_status_snapshot(player, actor.map_item().id()).filter(|status| status.hp() > 0) {
                    targets.push(GroundTrapTarget {
                        id: actor.map_item().id(),
                        x: actor.x(),
                        y: actor.y(),
                        status,
                        player: true,
                    });
                }
            }
        }
        targets
    }

    pub(super) fn trap_source(&self, state: &ServerState, source: &GroundSkillSource) -> GroundSkillSource {
        let mut source = source.clone();
        source.status = state
            .get_character(source.actor_id)
            .map(|character| StatusService::instance().to_snapshot(&character.status))
            .or_else(|| {
                state
                    .get_character(source.owner_id)
                    .and_then(|owner| companion_status_snapshot(owner, source.actor_id))
            })
            .or_else(|| {
                state.get_map_instance(&source.map, source.instance).and_then(|map| {
                    let map = map.state();
                    map.get_mob(source.actor_id)
                        .map(|mob| mob.status.clone())
                        .or_else(|| map.script_skill_state.npcs.get(&source.actor_id).map(|npc| npc.snapshot()))
                })
            })
            .unwrap_or(source.status);
        source
    }

    pub(super) fn change_trap_view(&self, ground: &GroundSkill, view: u32) {
        for cell in &ground.cells {
            let modern = self.configuration.packetver() >= 20181121;
            let mut packet = (if modern { 0x0A43_u16 } else { 0x01D7_u16 }).to_le_bytes().to_vec();
            packet.extend_from_slice(&cell.id.to_le_bytes());
            packet.push(0);
            if modern {
                packet.extend_from_slice(&view.to_le_bytes());
                packet.extend_from_slice(&0_u32.to_le_bytes());
            } else {
                packet.extend_from_slice(&(view as u16).to_le_bytes());
                packet.extend_from_slice(&0_u16.to_le_bytes());
            }
            self.notify_ground_cell(ground, cell, packet);
        }
    }

    pub(crate) fn pending_trap_capture(&self, request: &GroundTrapCapture, tick: u128) -> bool {
        request.lease.active()
            && tick <= request.deadline
            && self.ground_skills.lock().is_ok_and(|grounds| {
                grounds.iter().any(|ground| {
                    ground.expires_at > tick
                        && crate::server::service::map_flag_service::normalize_map(&ground.map) == request.map.map_without_ext()
                        && ground.instance == request.map.map_instance()
                        && ground
                            .capture
                            .as_ref()
                            .is_some_and(|capture| capture.target_id == request.target_id && capture.trap_id == request.trap_id)
                })
            })
    }

    pub(super) fn ankle_capture_duration(source_level: u32, target_agi: u16, immune: bool, duration: i32) -> i32 {
        let duration = i64::from(duration.max(0)) / if immune { 5 } else { 1 };
        (duration - duration * i64::from(target_agi) / 200)
            .max(30 * (i64::from(source_level) + 100))
            .min(i64::from(i32::MAX)) as i32
    }

    pub(super) fn begin_ankle_capture(
        &self,
        server: &Server,
        state: &ServerState,
        ground: &mut GroundSkill,
        source: &GroundSkillSource,
        tick: u128,
    ) {
        let map = MapInstanceKey::new(ground.map.clone(), ground.instance);
        let Some(target) = self
            .trap_targets(server, state, ground, source)
            .into_iter()
            .find(|target| ground.covers(target.x, target.y))
        else {
            return;
        };
        let target_id = target.id;
        let metadata = SkillMetadata::find(ground.skill_id).unwrap();
        let duration = Self::ankle_capture_duration(
            source.status.base_level(),
            target.status.agi(),
            target.status.has_mob_capability(MobCapability::StatusImmune),
            metadata.duration(ground.level, true).unwrap_or(4000),
        );
        let trap_id = ground.cells[0].id;
        let request = StatusChangeRequest {
            kind: StatusChangeKind::Ankle,
            duration_ms: duration,
            values: [i32::from(ground.level), trap_id as i32, 0, 0],
            rate: 10000,
            flags: StatusStartFlag::NoRateReduction.as_flag() | StatusStartFlag::NoDurationReduction.as_flag(),
        };
        let lease = TrapLease::new();
        ground.capture = Some(TrapCaptureState {
            lease: lease.clone(),
            target_id,
            trap_id,
            confirmed: false,
        });
        ground.triggered = true;
        ground.expires_at = tick + 3000;
        self.change_trap_view(ground, GroundKind::AnkleSnare.view_id());
        server.add_to_next_tick(GameEvent::GroundTrapCapture(GroundTrapCapture {
            lease,
            deadline: tick + 3000,
            map,
            target_id,
            trap_id,
            x: ground.cells[0].x,
            y: ground.cells[0].y,
            request,
        }));
    }

    pub(super) fn tick_trap_capture(&self, _server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        if ground.expires_at <= tick {
            return;
        }
        let Some(capture) = &mut ground.capture else {
            return;
        };
        let map = MapInstanceKey::new(ground.map.clone(), ground.instance);
        if let Some(change) = linked_ankle(state, &map, capture.target_id, capture.trap_id).filter(|change| !change.expired(tick)) {
            capture.confirmed = true;
            ground.expires_at = change.expires_at.unwrap_or(tick + 3000);
        } else if capture.confirmed {
            ground.expires_at = tick;
        }
    }

    pub(super) fn release_trap_capture(&self, server: &Server, ground: &mut GroundSkill) {
        if let Some(capture) = ground.capture.take() {
            capture.lease.cancel();
            server.add_to_next_tick(GameEvent::GroundTrapRelease(GroundTrapRelease {
                map: MapInstanceKey::new(ground.map.clone(), ground.instance),
                target_id: capture.target_id,
                trap_id: capture.trap_id,
            }));
        }
    }
}
