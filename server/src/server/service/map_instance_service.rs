use std::ops::Deref;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use models::enums::action::ActionType;
use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::mob::MobMode;
use models::enums::size::Size;
use models::enums::skill_enums::SkillEnum;
use models::enums::vanish::VanishType;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithNumberValue};
use models::item::DroppedItem;
use models::status_bonus::{BattleFlag, CombatTrigger};
use models::status_change::StatusChangeRequest;
use movement::position::Position;
use packets::packets::{
    Packet, PacketZcItemDisappear, PacketZcItemFallEntry, PacketZcNotifyAct, PacketZcNotifyMove, PacketZcNotifyStandentry7,
    PacketZcNotifyVanish, PacketZcUseSkill,
};

use crate::server::game_loop::GAME_TICK_RATE;
use crate::server::map_instance_loop::MAP_LOOP_TICK_RATE;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::{CharacterKillMonster, GameEvent, ScriptEvent};
use crate::server::model::events::map_event::{CharacterDropItems, MapEvent, MobAttackCharacter, MobDropItems, MobLocation, ScriptSpawn};
use crate::server::model::map::Map;
use crate::server::model::map_item::{MapItemSnapshot, MapItemType, ToMapItemSnapshot};
use crate::server::model::status::StatusFromDb;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::battle_service::{BattleService, NormalAttackRoll};
use crate::server::service::combat_trigger_service::{magic_reflection, physical_reflection};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_combat_service::{MagicReflectionRequest, MobAttackRequest};
use crate::server::service::mob_service::{MobAIAction, MobService};
use crate::server::service::script_combat_service::{MobCombatEffect, ScriptCombatRequest};
use crate::server::service::script_service::ScriptService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::mob::{Mob, MobMovement};
use crate::util::tick::{delayed_tick, get_tick, get_tick_client};

#[path = "map_pet_capture.rs"]
mod pet_capture;
#[path = "map_pet_loot.rs"]
mod pet_loot;

pub struct MapInstanceService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    mob_service: MobService,
    battle_service: BattleService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

impl MapInstanceService {
    pub fn start_actor_skill(&self, state: &mut MapInstanceState, request: crate::server::script::skill::actor::MapActorSkillCast, tick: u128) -> Result<(), String> {
        crate::server::script::skill::actor::start_map_cast(state, request, tick, &self.client_notification_sender)
    }

    pub fn tick_actor_skills(&self, state: &mut MapInstanceState, tick: u128) {
        for completion in crate::server::script::skill::actor::tick_map_casts(state, tick) {
            self.server_task_queue.add_to_first_index(GameEvent::ScriptActorSkillComplete(completion));
        }
    }
    pub(crate) fn new(
        client_notification_sender: SyncSender<Notification>,
        configuration_service: &'static GlobalConfigService,
        mob_service: MobService,
        server_task_queue: Arc<TasksQueue<GameEvent>>,
    ) -> Self {
        MapInstanceService {
            battle_service: BattleService::new(
                client_notification_sender.clone(),
                StatusService::instance(),
                configuration_service,
                crate::server::service::battle_service::BattleResultMode::Normal,
            ),
            client_notification_sender,
            configuration_service,
            mob_service,
            server_task_queue,
        }
    }

    pub fn spawn_mobs(&self, map: &Map, map_instance_state: &mut MapInstanceState) {
        for mob_spawn in map.mob_spawns().iter() {
            let spawned = {
                let mob_spawn_track = map_instance_state.mob_spawns_tracks().get(&mob_spawn.id).unwrap();
                if mob_spawn_track.spawned_amount >= mob_spawn.to_spawn_amount {
                    continue;
                }
                if mob_spawn.has_delay() {
                    // TODO check when respawn is planned
                }
                mob_spawn.to_spawn_amount - mob_spawn_track.spawned_amount
            };
            let mut cell: (u16, u16);
            for _ in 0..spawned {
                if mob_spawn.is_fixed_position() {
                    cell = (mob_spawn.x, mob_spawn.y);
                } else {
                    // if mob_spawn.is_zone_constraint() {
                    // TODO implement constraint zone
                    cell = Map::find_random_walkable_cell(map_instance_state.cells_mut().deref(), map.x_size());
                }
                let mob_map_item_id = map_instance_state.map_items_mut().generate_id();
                let mut mob = Mob::new(
                    mob_map_item_id,
                    cell.0,
                    cell.1,
                    mob_spawn.mob_id,
                    mob_spawn.id,
                    mob_spawn.info.name.clone(),
                    mob_spawn.info.name_english.clone(),
                    mob_spawn.info.damage_motion as u32,
                    StatusFromDb::from_mob_model(&mob_spawn.info),
                    mob_spawn.info.mode as u32,
                    mob_spawn.info.range1 as u16,
                    mob_spawn.info.range3 as u16,
                    mob_spawn.info.atk_delay as u32,
                    mob_spawn.info.atk_motion as u32,
                    mob_spawn.info.atk1 as u16,
                    mob_spawn.info.atk2 as u16,
                );
                mob.status_effects.base_level = mob_spawn.info.level.max(1) as u32;

                debug!("Spawning mob {}", mob_map_item_id);
                map_instance_state.insert_mob(mob);
                // END
                let mob_spawn_track = map_instance_state.mob_spawns_tracks_mut().get_mut(&mob_spawn.id).unwrap();
                mob_spawn_track.increment_spawn();
            }
        }
    }

    pub fn update_mobs_fov(&self, map_instance_state: &mut MapInstanceState, characters: Vec<MapItemSnapshot>) {
        map_instance_state.update_characters(characters);
    }

    pub fn start_mob_status(&self, state: &mut MapInstanceState, mob_id: u32, request: StatusChangeRequest, tick: u128) {
        let changed = state.mobs_mut().get_mut(&mob_id).filter(|mob| mob.is_present()).and_then(|mob| {
            match mob.start_status(request, tick, fastrand::u16(0..10000)) {
                Ok(outcome) if outcome.started => Some((
                    mob.x,
                    mob.y,
                    StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                )),
                Ok(_) => None,
                Err(error) => {
                    error!("Cannot apply status to mob {}: {}", mob_id, error);
                    None
                }
            }
        });
        if let Some((x, y, packet)) = changed {
            self.notify_area(state, x, y, packet);
        }
    }

    pub fn end_mob_status(&self, state: &mut MapInstanceState, mob_id: u32, kind: Option<models::status_change::StatusChangeKind>) {
        let changed = state.mobs_mut().get_mut(&mob_id).filter(|mob| mob.is_present()).and_then(|mob| {
            (!mob.end_status(kind).is_empty()).then(|| {
                (
                    mob.x,
                    mob.y,
                    StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                )
            })
        });
        if let Some((x, y, packet)) = changed {
            self.notify_area(state, x, y, packet);
        }
    }

    pub fn start_mob_status_alternatives(
        &self,
        state: &mut MapInstanceState,
        request: crate::server::model::events::map_event::MobStatusAlternatives,
        tick: u128,
    ) {
        let changed = state
            .mobs_mut()
            .get_mut(&request.mob_id)
            .filter(|mob| mob.is_present())
            .and_then(|mob| {
                for candidate in request.requests {
                    match mob.start_status(candidate, tick, fastrand::u16(0..10000)) {
                        Ok(outcome) if outcome.started => {
                            return Some((
                                mob.x,
                                mob.y,
                                StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                            ));
                        }
                        Ok(_) => {}
                        Err(error) => error!("Cannot apply alternative status to mob {}: {}", request.mob_id, error),
                    }
                }
                None
            });
        if let Some((x, y, packet)) = changed {
            self.notify_area(state, x, y, packet);
        }
    }

    pub fn dispel_mob(&self, state: &mut MapInstanceState, mob_id: u32) {
        let changed = state
            .mobs_mut()
            .get_mut(&mob_id)
            .filter(|mob| mob.is_present() && mob.hp() > 0)
            .and_then(|mob| {
                (!mob.dispel_statuses().is_empty()).then(|| {
                    (
                        mob.x,
                        mob.y,
                        StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                    )
                })
            });
        if let Some((x, y, packet)) = changed {
            self.notify_area(state, x, y, packet);
        }
    }

    pub fn provoke_mob(&self, state: &mut MapInstanceState, request: crate::server::model::events::map_event::MobProvoke, tick: u128) {
        let source_live = state.get_map_item(request.source_id).is_some_and(|item| match item.object_type() {
            MapItemType::Character | MapItemType::Homunculus | MapItemType::Mercenary => true,
            MapItemType::Mob => state.get_mob(request.source_id).is_some_and(|mob| mob.is_present() && mob.hp() > 0),
            _ => false,
        });
        if !source_live || request.request.kind != models::status_change::StatusChangeKind::Provoke {
            return;
        }
        let changed = state
            .mobs_mut()
            .get_mut(&request.mob_id)
            .filter(|mob| {
                mob.is_present()
                    && mob.hp() > 0
                    && mob.mode & MobMode::Boss.as_flag() == 0
                    && !matches!(mob.status.mob_class(), models::enums::mob::MobClass::Boss | models::enums::mob::MobClass::Guardian | models::enums::mob::MobClass::Battlefield)
                    && *mob.status.race() != models::enums::mob::MobRace::RUndead
                    && *mob.status.element() != Element::Undead
            })
            .and_then(|mob| match mob.start_status(request.request, tick, fastrand::u16(0..10000)) {
                Ok(outcome) if outcome.started => {
                    mob.target_id = Some(request.source_id);
                    mob.movements.clear();
                    if !mob.is_flinching() {
                        mob.action = crate::server::state::mob::MobAction::Chasing {
                            target_id: request.source_id,
                        };
                    }
                    let class = *mob.status.mob_class();
                    let rate = request.coma.chance_against(&mob.status, class, false, BattleFlag::Misc.as_flag());
                    if rate > 0 && fastrand::u16(0..10000) < rate {
                        let mut coma = StatusChangeRequest::guaranteed(models::status_change::StatusChangeKind::Coma, 0, 1);
                        coma.values[2] = request.source_id as i32;
                        if let Err(error) = mob.start_status(coma, tick, 0) {
                            error!("Cannot apply Provoke Coma to mob {}: {}", mob.id, error);
                        }
                    }
                    Some((
                        mob.x,
                        mob.y,
                        StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                    ))
                }
                Ok(_) => None,
                Err(error) => {
                    error!("Cannot provoke mob {}: {}", request.mob_id, error);
                    None
                }
            });
        if let Some((x, y, packet)) = changed {
            self.notify_area(state, x, y, packet);
        }
    }

    pub fn heal_mob(&self, state: &mut MapInstanceState, mob_id: u32, hp: u32, sp: u32) {
        let Some(mob) = state.mobs_mut().get_mut(&mob_id).filter(|mob| {
            mob.is_present()
                && mob.hp() > 0
                && !mob
                    .status_effects
                    .has_status_change(models::status_change::StatusChangeKind::NoRecovery)
        }) else {
            return;
        };
        mob.set_hp(mob.hp().saturating_add(hp).min(mob.status.max_hp()));
        mob.status.set_sp(mob.status.sp().saturating_add(sp).min(mob.status.max_sp()));
        mob.status_effects.sp = mob.status.sp();
        self.notify_mob_stand(state, mob_id);
    }

    pub fn tick_mob_statuses(&self, state: &mut MapInstanceState, tasks: &TasksQueue<MapEvent>, tick: u128) {
        let mut notifications = Vec::new();
        let mut deaths = Vec::new();
        for mob in state.mobs_mut().values_mut().filter(|mob| mob.is_present()) {
            if let Some(effect) = crate::server::script::skill::ScriptSkillService::splasher_expiration(mob.id, &mob.status_effects, tick) {
                self.server_task_queue.add_to_first_index(GameEvent::CharacterScriptSkill(effect));
            }
            if !mob.tick_statuses(tick).is_empty() {
                notifications.push((
                    mob.x,
                    mob.y,
                    StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                ));
            }
            if mob.should_die() {
                mob.last_attack_flags = BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag();
                mob.last_attack_skill = 0;
                deaths.push(MobLocation {
                    mob_id: mob.id,
                    x: mob.x,
                    y: mob.y,
                });
            }
        }
        for (x, y, packet) in notifications {
            self.notify_area(state, x, y, packet);
        }
        for death in deaths {
            tasks.add_to_first_index(MapEvent::MobDeathClientNotification(death));
            self.mob_die(state, death.mob_id, 0);
        }
        let spheres = state
            .mobs()
            .values()
            .filter(|mob| mob.is_present() && mob.summon_ai == 2 && mob.next_summon_action != 0 && mob.next_summon_action <= tick)
            .map(|mob| (mob.id, mob.position(), mob.hp(), mob.summon_owner.unwrap_or(mob.id)))
            .collect::<Vec<_>>();
        for (sphere_id, position, hp, owner) in spheres {
            let targets = state
                .mobs()
                .values()
                .filter(|mob| {
                    mob.is_present()
                        && mob.summon_ai == 0
                        && mob.id != sphere_id
                        && mob.x.abs_diff(position.x).max(mob.y.abs_diff(position.y)) <= 5
                })
                .map(|mob| (mob.id, mob.status.clone(), mob.damage_motion))
                .collect::<Vec<_>>();
            for (id, target, motion) in targets {
                let damage = (hp as f32 * BattleService::element_modifier(&Element::Fire, &target))
                    .max(0.0)
                    .floor() as u32;
                let mut packet = PacketZcNotifyAct::new(self.configuration_service.packetver());
                packet.set_gid(sphere_id);
                packet.set_target_gid(id);
                packet.set_action(ActionType::Attack.value() as u8);
                packet.set_damage(damage.min(i16::MAX as u32) as i16);
                packet.set_count(1);
                packet.fill_raw();
                self.notify_area(state, position.x, position.y, packet.raw);
                tasks.add_to_first_index(MapEvent::MobDamage(Damage {
                    target_id: id,
                    attacker_id: sphere_id,
                    damage,
                    healing: 0,
                    right_hand_damage: None,
                    attacked_at: tick,
                    damage_motion: motion,
                    battle_flags: BattleFlag::Misc.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Skill.as_flag(),
                    skill_id: models::enums::skill_enums::SkillEnum::NpcSelfdestruction.id(),
                    skill_level: 1,
                    landed: true,
                    proc_depth: 0,
                    credit_id: owner,
                    defenses_applied: true,
                    magic_context: None,
                }));
            }
            if let Some(mob) = state.mobs_mut().get_mut(&sphere_id) {
                mob.set_hp(0);
                mob.last_credit_id = 0;
                mob.last_attack_flags = 0;
            }
            tasks.add_to_first_index(MapEvent::MobDeathClientNotification(MobLocation {
                mob_id: sphere_id,
                x: position.x,
                y: position.y,
            }));
            self.mob_die(state, sphere_id, 0);
        }
    }

    pub fn mob_knockback(&self, state: &mut MapInstanceState, mob_id: u32, source_x: u16, source_y: u16, cells: u16) {
        let Some(mob) = state.get_mob(mob_id).filter(|mob| mob.is_present()) else {
            return;
        };
        if mob.mode & (MobMode::NoKnockback.as_flag() | MobMode::Boss.as_flag()) != 0 {
            return;
        }
        let from = mob.position();
        let to = knockback_position(state.cells(), state.x_size(), state.y_size(), from, source_x, source_y, cells);
        if from == to {
            return;
        }
        let mob = state.mobs_mut().get_mut(&mob_id).unwrap();
        mob.movements.clear();
        let dir = mob.dir;
        mob.update_position(to.x, to.y);
        mob.dir = dir;
        mob.update_movement_complete();
        let mut packet = 0x0088_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&mob_id.to_le_bytes());
        packet.extend_from_slice(&to.x.to_le_bytes());
        packet.extend_from_slice(&to.y.to_le_bytes());
        self.notify_area(state, from.x, from.y, packet);
    }

    pub fn random_warp_mob(&self, state: &mut MapInstanceState, mob_id: u32) {
        let Some(mob) = state.get_mob(mob_id).filter(|mob| mob.is_present()) else {
            return;
        };
        if mob.summon_owner.is_none() && state.flags.enabled(crate::server::model::map_flags::MapFlag::MonsterNoTeleport) {
            return;
        }
        let from = MobLocation {
            mob_id,
            x: mob.x,
            y: mob.y,
        };
        let locations = spawn_locations(state.cells(), state.x_size(), state.y_size(), 0, 0);
        if locations.is_empty() {
            return;
        }
        let to = locations[fastrand::usize(0..locations.len())];
        if let Some(mob) = state.mobs_mut().get_mut(&mob_id) {
            mob.movements.clear();
            let dir = mob.dir;
            mob.update_position(to.0, to.1);
            mob.dir = dir;
            mob.update_movement_complete();
        }
        let mut vanish = PacketZcNotifyVanish::new(self.configuration_service.packetver());
        vanish.set_gid(mob_id);
        vanish.set_atype(VanishType::Teleport.value() as u8);
        vanish.fill_raw();
        self.notify_area(state, from.x, from.y, vanish.raw);
        self.notify_mob_stand(state, mob_id);
    }

    pub fn face_mob(&self, state: &mut MapInstanceState, mob_id: u32, dir: u16) {
        if dir >= 8 {
            return;
        }
        let Some(mob) = state.mobs_mut().get_mut(&mob_id).filter(|mob| mob.is_present()) else {
            return;
        };
        mob.dir = dir;
        let (x, y) = (mob.x, mob.y);
        let mut packet = 0x009C_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&mob_id.to_le_bytes());
        packet.extend_from_slice(&0_u16.to_le_bytes());
        packet.push(dir as u8);
        self.notify_area(state, x, y, packet);
    }

    pub fn warp_mob_to(&self, state: &mut MapInstanceState, mob_id: u32, x: u16, y: u16) {
        if state.flags.enabled(crate::server::model::map_flags::MapFlag::MonsterNoTeleport)
            && state.get_mob(mob_id).is_some_and(|mob| mob.summon_owner.is_none())
        {
            return;
        }
        if x >= state.x_size()
            || y >= state.y_size()
            || state.cells()[y as usize * state.x_size() as usize + x as usize] & CellType::Walkable.as_flag() == 0
        {
            return;
        }
        let Some(mob) = state.mobs_mut().get_mut(&mob_id).filter(|mob| mob.is_present()) else {
            return;
        };
        let (from_x, from_y) = (mob.x, mob.y);
        mob.movements.clear();
        mob.lose_target();
        let dir = mob.dir;
        mob.update_position(x, y);
        mob.dir = dir;
        mob.update_movement_complete();
        let mut packet = PacketZcNotifyVanish::new(self.configuration_service.packetver());
        packet.set_gid(mob_id);
        packet.set_atype(VanishType::Teleport.value() as u8);
        packet.fill_raw();
        self.notify_area(state, from_x, from_y, packet.raw);
        self.notify_mob_stand(state, mob_id);
    }

    pub fn script_mob_combat(
        &self,
        state: &mut MapInstanceState,
        source_id: u32,
        target_id: u32,
        effect: MobCombatEffect,
        tasks: Arc<TasksQueue<MapEvent>>,
        tick: u128,
    ) {
        match effect {
            MobCombatEffect::Status(request) => self.start_mob_status(state, target_id, request, tick),
            MobCombatEffect::Vanish { hp, sp } => {
                if let Some(mob) = state.mobs_mut().get_mut(&target_id).filter(|mob| mob.is_present()) {
                    mob.status.set_sp(mob.status.sp().saturating_sub(sp));
                    mob.status_effects.sp = mob.status.sp();
                }
                if hp > 0 {
                    self.mob_being_attacked(
                        state,
                        Damage {
                            target_id,
                            attacker_id: source_id,
                            damage: hp,
                            healing: 0,
                            right_hand_damage: None,
                            attacked_at: tick,
                            damage_motion: 0,
                            battle_flags: 0,
                            skill_id: 0,
                            skill_level: 0,
                            landed: false,
                            proc_depth: 8,
                            credit_id: source_id,
                            defenses_applied: true,
                            magic_context: None,
                        },
                        tasks,
                        tick,
                    );
                }
            }
            MobCombatEffect::ClassChange { mob_id } => {
                let Some(model) = self.configuration_service.get_mob_safe(mob_id as i32) else {
                    return;
                };
                let Some(old) = state
                    .get_mob(target_id)
                    .filter(|mob| mob.is_present() && mob.summon_ai <= 1 && mob.mode & MobMode::Boss.as_flag() == 0)
                else {
                    return;
                };
                if self
                    .configuration_service
                    .get_mob(old.mob_id as i32)
                    .race_groups
                    .iter()
                    .any(|group| group.eq_ignore_ascii_case("Guardian") || group.eq_ignore_ascii_case("Treasure"))
                {
                    return;
                }
                let mut replacement = self.mob_from_model(target_id, old.x, old.y, model, old.spawn_id);
                replacement.dir = old.dir;
                replacement.summoned = old.summoned;
                replacement.summon_owner = old.summon_owner;
                replacement.summon_ai = old.summon_ai;
                replacement.event_entry = old.event_entry;
                replacement.set_hp(
                    (replacement.hp() as u64 * old.hp() as u64 / old.status.max_hp().max(1) as u64)
                        .max(1)
                        .min(u32::MAX as u64) as u32,
                );
                replacement.damages = old.damages.clone();
                replacement.actor_damages = old.actor_damages.clone();
                let location = MobLocation {
                    mob_id: target_id,
                    x: old.x,
                    y: old.y,
                };
                state.remove_mob(target_id);
                self.mob_die_client_notification(state, location);
                state.insert_mob(replacement);
                self.notify_mob_stand(state, target_id);
            }
        }
    }

    pub fn script_drop_item(&self, state: &mut MapInstanceState, owner_id: u32, item_id: i32, amount: i16, x: u16, y: u16) {
        if state.flags.enabled(crate::server::model::map_flags::MapFlag::NoMobLoot) {
            return;
        }
        let Some(item) = self.configuration_service.find_item(item_id).filter(|_| amount > 0) else {
            return;
        };
        let mut rng = fastrand::Rng::new();
        let dropped = self.drop_items(
            state,
            &mut rng,
            x,
            y,
            item_id,
            !item.item_type.should_be_identified_when_dropped(),
            amount as u16,
            Some(owner_id),
            Default::default(),
            false,
        );
        if let Some(dropped) = dropped {
            self.notify_drop_items(state, x, y, vec![dropped]);
        }
    }

    pub fn script_spawn(&self, state: &mut MapInstanceState, request: ScriptSpawn) -> Result<Vec<u32>, String> {
        let model = self
            .configuration_service
            .get_mob_safe(request.mob_id)
            .ok_or("Script monster class does not exist")?;
        if request.amount == 0 {
            return Err("Script monster count must be positive".into());
        }
        let ai = request.ai.unwrap_or(0);
        if ai > 3 {
            return Err("Monster AI is unavailable in pre-renewal".into());
        }
        let size = request
            .size
            .map(|size| Size::try_from_value(size as usize))
            .transpose()
            .map_err(|_| "Monster size must be small, medium or large")?;
        if size == Some(Size::All) {
            return Err("Monster size must be small, medium or large".into());
        }
        let event_entry = if request.event.is_empty() {
            None
        } else {
            Some(ScriptService::event_entry(&request.event).ok_or("Monster callback has no compiled event entry")?)
        };
        let locations = spawn_locations(state.cells(), state.x_size(), state.y_size(), request.x, request.y);
        if locations.is_empty() {
            return Err("Monster spawn has no walkable cells".into());
        }
        let mut rng = fastrand::Rng::new();
        let mut ids = Vec::with_capacity(request.amount as usize);
        for _ in 0..request.amount {
            let (x, y) = locations[rng.usize(0..locations.len())];
            let id = state.map_items_mut().generate_id();
            let mut mob = self.mob_from_model(id, x, y, model, 0);
            mob.summoned = true;
            mob.summon_owner = (ai != 0).then_some(request.owner_id);
            mob.summon_ai = ai;
            mob.event_entry = event_entry;
            if !request.name.is_empty() && request.name != "--ja--" && request.name != "--en--" {
                mob.name = request.name.clone();
                mob.name_english = request.name.clone();
            }
            if let Some(size) = size {
                mob.status.set_size(size);
                mob.base_status.set_size(size);
            }
            if ai == 1 || ai == 3 {
                mob.mode |= MobMode::Aggressive.as_flag() | MobMode::CanAttack.as_flag();
            }
            if ai == 3 {
                mob.mode &= !MobMode::CanMove.as_flag();
            }
            if ai == 2 {
                mob.mode &= !MobMode::CanAttack.as_flag();
            }
            state.insert_mob(mob);
            ids.push(id);
            self.notify_mob_stand(state, id);
        }
        self.server_task_queue.add_to_first_index(GameEvent::ScriptSpawned(
            crate::server::model::events::game_event::ScriptSpawned {
                char_id: request.owner_id,
                mob_ids: ids.clone(),
            },
        ));
        Ok(ids)
    }

    fn mob_from_model(&self, id: u32, x: u16, y: u16, model: &crate::repository::model::mob_model::MobModel, spawn_id: u32) -> Mob {
        let mut mob = Mob::new(
            id,
            x,
            y,
            model.id as i16,
            spawn_id,
            model.name.clone(),
            model.name_english.clone(),
            model.damage_motion as u32,
            StatusFromDb::from_mob_model(model),
            model.mode as u32,
            model.range1 as u16,
            model.range3 as u16,
            model.atk_delay as u32,
            model.atk_motion as u32,
            model.atk1 as u16,
            model.atk2 as u16,
        );
        mob.status_effects.base_level = model.level.max(1) as u32;
        mob
    }

    fn notify_mob_stand(&self, state: &MapInstanceState, id: u32) {
        let Some(mob) = state.get_mob(id) else {
            return;
        };
        let mut packet = PacketZcNotifyStandentry7::new(self.configuration_service.packetver());
        packet.set_packet_length(PacketZcNotifyStandentry7::base_len(self.configuration_service.packetver()) as i16);
        packet.set_job(mob.mob_id);
        packet.set_aid(id);
        packet.set_gid(id);
        packet.set_objecttype(MapItemType::Mob.value() as u8);
        packet.set_pos_dir(mob.position().to_pos());
        packet.set_speed(mob.status.speed() as i16);
        packet.set_clevel(mob.status_effects.base_level.min(i16::MAX as u32) as i16);
        packet.set_hp(mob.hp());
        packet.set_max_hp(mob.status.max_hp());
        packet.fill_raw_with_packetver(Some(self.configuration_service.packetver()));
        self.notify_area(state, mob.x, mob.y, packet.raw);
    }

    fn notify_area(&self, state: &MapInstanceState, x: u16, y: u16, raw: Vec<u8>) {
        self.notify_area_key(state.key(), x, y, raw);
    }

    fn notify_area_key(&self, key: &crate::server::model::map_instance::MapInstanceKey, x: u16, y: u16, raw: Vec<u8>) {
        if let Err(error) = self.client_notification_sender.send(Notification::Area(AreaNotification::new(
            key.map_name().clone(),
            key.map_instance(),
            AreaNotificationRangeType::Fov { x, y, exclude_id: None },
            raw,
        ))) {
            error!("Unable to notify map combat: {}", error);
        }
    }

    pub fn mobs_action(&self, map_instance_state: &mut MapInstanceState, map_instance_task_queue: Arc<TasksQueue<MapEvent>>, tick: u128) {
        let start_time = get_tick_client();
        let mut mob_movements: Vec<MobMovement> = Vec::with_capacity(map_instance_state.mobs().len() / 2);
        let mut mob_attacks: Vec<MobAttackCharacter> = Vec::new();
        let cells = map_instance_state.cells().clone();
        let mut characters: Vec<MapItemSnapshot> = map_instance_state.characters().to_vec();
        characters.extend(
            map_instance_state
                .mobs()
                .values()
                .filter(|mob| mob.is_present() && mob.summon_ai != 0)
                .map(ToMapItemSnapshot::to_map_item_snapshot),
        );
        let x_size = map_instance_state.x_size();
        let y_size = map_instance_state.y_size();

        if !characters.is_empty() {
            debug!(
                "mobs_action: {} characters on map {}, {} mobs",
                characters.len(),
                map_instance_state.key().map_name(),
                map_instance_state.mobs().len()
            );
        }

        let hostile_mobs = map_instance_state
            .mobs()
            .values()
            .filter(|mob| mob.is_present() && mob.summon_ai == 0)
            .map(ToMapItemSnapshot::to_map_item_snapshot)
            .collect::<Vec<_>>();
        use crate::server::service::visibility_service::{StealthState, TargetingMode, VisibilityObserver, can_target};
        let mut visibility = map_instance_state.actor_visibility.clone();
        for mob in map_instance_state.mobs().values() {
            visibility.insert(mob.id, StealthState::from_status(&mob.status_effects));
        }
        for mob in map_instance_state.mobs_mut().values_mut() {
            if mob.summon_ai == 2 {
                continue;
            }
            let targets = if mob.summon_ai != 0 {
                hostile_mobs.as_slice()
            } else {
                characters.as_slice()
            };
            let observer = VisibilityObserver::for_mob(mob.mode, *mob.status.race());
            let targets: Vec<_> = targets
                .iter()
                .filter(|target| {
                    can_target(
                        observer,
                        visibility.get(&target.map_item.id()).copied().unwrap_or_default(),
                        TargetingMode::Direct,
                    )
                })
                .copied()
                .collect();
            if let Some(action) = self.mob_service.action_ai(mob, &targets, cells.as_ref(), x_size, y_size, tick) {
                match action {
                    MobAIAction::Move(movement) => mob_movements.push(movement),
                    MobAIAction::Attack(attack) => mob_attacks.push(attack),
                }
            }
        }

        for mob_movement in mob_movements {
            let mut packet_zc_notify_move = PacketZcNotifyMove::new(self.configuration_service.packetver());
            packet_zc_notify_move.set_gid(mob_movement.id);
            packet_zc_notify_move.move_data = mob_movement.from.to_move_data(&mob_movement.to);
            packet_zc_notify_move.set_move_start_time(start_time);
            packet_zc_notify_move.fill_raw();
            #[cfg(feature = "debug_mob_movement")]
            {
                info!(
                    "Mob {} moving from {} to {}. Notify area around {},{}",
                    mob_movement.id, mob_movement.from, mob_movement.to, mob_movement.from.x, mob_movement.from.y
                );
            }
            self.client_notification_sender
                .send(Notification::Area(AreaNotification::new(
                    map_instance_state.key().map_name().clone(),
                    map_instance_state.key().map_instance(),
                    AreaNotificationRangeType::Fov {
                        x: mob_movement.from.x,
                        y: mob_movement.from.y,
                        exclude_id: None,
                    },
                    packet_zc_notify_move.raw,
                )))
                .unwrap_or_else(|_| error!("Failed to send notification packet_zc_notify_move to client"));
        }

        for attack in mob_attacks {
            map_instance_task_queue.add_to_first_index(MapEvent::MobAttackCharacter(attack));
        }
    }

    pub fn mob_attack_character(&self, state: &MapInstanceState, attack: MobAttackCharacter, tasks: &TasksQueue<MapEvent>, tick: u128) {
        let Some(source) = state.get_mob(attack.mob_id).filter(|mob| mob.is_present() && !mob.blocks_attack()) else {
            return;
        };
        use crate::server::service::visibility_service::{StealthState, TargetingMode, VisibilityObserver, can_target};
        let stealth = state
            .get_mob(attack.target_char_id)
            .map(|target| StealthState::from_status(&target.status_effects))
            .or_else(|| state.actor_visibility.get(&attack.target_char_id).copied())
            .unwrap_or_default();
        if !can_target(
            VisibilityObserver::for_mob(source.mode, *source.status.race()),
            stealth,
            TargetingMode::Direct,
        ) {
            return;
        }
        let flags = BattleFlag::Weapon.as_flag()
            | BattleFlag::Normal.as_flag()
            | if source.attack_range > 3 {
                BattleFlag::Long.as_flag()
            } else {
                BattleFlag::Short.as_flag()
            };
        if let Some(target) = state.get_mob(attack.target_char_id).filter(|mob| mob.is_present()) {
            let mut rng = fastrand::Rng::new();
            let roll = BattleService::normal_attack_roll(&source.status, &target.status, source.attack_range > 3, &mut rng);
            let damage = if matches!(roll, NormalAttackRoll::Miss | NormalAttackRoll::LuckyDodge) {
                0
            } else {
                let raw = if roll == NormalAttackRoll::Critical
                    || source
                        .status
                        .has_status_change(models::status_change::StatusChangeKind::MaximizePower)
                {
                    source.atk2.max(source.atk1) as u32
                } else {
                    rng.u32(source.atk1 as u32..=source.atk2.max(source.atk1) as u32)
                };
                let raw = (raw as f64 * f64::from(BattleService::weapon_skill_ratio(&source.status, 1.0, 0)))
                    .clamp(0.0, u32::MAX as f64)
                    .floor() as u32;
                self.battle_service.actor_physical_damage_signed(
                    raw,
                    &source.status,
                    &target.status,
                    false,
                    roll == NormalAttackRoll::Critical,
                    &self.battle_service.attack_element(&source.status, None),
                    flags,
                )
            };
            let mut packet = PacketZcNotifyAct::new(self.configuration_service.packetver());
            packet.set_gid(source.id);
            packet.set_target_gid(target.id);
            packet.set_action(
                match roll {
                    NormalAttackRoll::Critical => ActionType::AttackCritical,
                    NormalAttackRoll::LuckyDodge => ActionType::AttackLucky,
                    _ => ActionType::Attack,
                }
                .value() as u8,
            );
            packet.set_attack_mt((source.atk_motion / 2) as i32);
            packet.set_attacked_mt(target.damage_motion as i32);
            packet.set_damage(damage.clamp(i16::MIN as i32, i16::MAX as i32) as i16);
            packet.set_count(1);
            packet.fill_raw();
            self.notify_area(state, source.x, source.y, packet.raw);
            let delay = source.atk_motion as u128 / 2;
            Self::add_to_delayed_tick(
                tasks,
                MapEvent::MobDamage(Damage {
                    target_id: target.id,
                    attacker_id: source.id,
                    damage: damage.max(0) as u32,
                    healing: if damage < 0 { damage.unsigned_abs() } else { 0 },
                    right_hand_damage: None,
                    attacked_at: tick.saturating_add(delay),
                    damage_motion: target.damage_motion,
                    battle_flags: flags,
                    skill_id: 0,
                    proc_depth: 0,
                    skill_level: 0,
                    landed: !matches!(roll, NormalAttackRoll::Miss | NormalAttackRoll::LuckyDodge),
                    credit_id: source.summon_owner.unwrap_or(source.id),
                    defenses_applied: true,
                    magic_context: None,
                }),
                delay,
            );
        } else {
            self.server_task_queue.add_to_first_index(GameEvent::MobAttack(MobAttackRequest {
                attack,
                source_status: source.status.clone(),
                source_key: state.key().clone(),
                min_atk: source.atk1,
                max_atk: source.atk2,
                battle_flags: flags,
            }));
        }
    }

    pub fn mob_being_attacked(
        &self,
        map_instance_state: &mut MapInstanceState,
        damage: Damage,
        map_instance_tasks_queue: Arc<TasksQueue<MapEvent>>,
        tick: u128,
    ) {
        let origin_map = map_instance_state.key().clone();
        if damage.healing > 0 {
            if let Some(mob) = map_instance_state
                .mobs_mut()
                .get_mut(&damage.target_id)
                .filter(|mob| mob.is_present() && mob.hp() > 0)
            {
                mob.set_hp(mob.hp().saturating_add(damage.healing).min(mob.status.max_hp()));
                self.notify_mob_stand(map_instance_state, damage.target_id);
            }
            return;
        }
        let source = map_instance_state.get_mob(damage.attacker_id).map(|mob| mob.status.clone());
        if damage.landed && damage.damage > 0 && damage.proc_depth < 8 {
            if let Some(target) = map_instance_state.get_mob(damage.target_id).filter(|mob| mob.is_present()) {
                if let Some(kind) = magic_reflection(&target.status, damage.battle_flags, damage.skill_id, &mut fastrand::Rng::new()) {
                    self.server_task_queue
                        .add_to_first_index(GameEvent::ReflectMagic(MagicReflectionRequest {
                            damage,
                            reflector_id: target.id,
                            reflector_credit_id: target.summon_owner.unwrap_or(target.id),
                            kind,
                            map_key: map_instance_state.key().clone(),
                        }));
                    return;
                }
            }
        }
        let is_player_attack = map_instance_state
            .get_map_item(damage.attacker_id)
            .is_some_and(|item| *item.object_type() == MapItemType::Character);
        let credited_id = if damage.credit_id == 0 {
            damage.attacker_id
        } else {
            damage.credit_id
        };
        let key = map_instance_state.key().clone();
        let map_flags = map_instance_state.flags.clone();
        let mut changed = None;
        let mut dead = None;
        let mut admitted_hit = false;
        if let Some(mob) = map_instance_state
            .mobs_mut()
            .get_mut(&damage.target_id)
            .filter(|mob| mob.is_present())
        {
            if mob
                .absorb_magic_rod(damage.battle_flags, damage.skill_id, damage.skill_level)
                .is_some()
            {
                let mut packet = PacketZcUseSkill::new(self.configuration_service.packetver());
                packet.set_src_aid(mob.id);
                packet.set_target_aid(mob.id);
                packet.set_skid(SkillEnum::SaMagicrod.id() as u16);
                packet.set_level(i16::from(damage.skill_level));
                packet.set_result(true);
                packet.fill_raw();
                self.notify_area_key(&key, mob.x, mob.y, packet.raw);
                return;
            }
            let before = mob.status_effects.active_statuses.clone();
            let equipment_damage = if damage.defenses_applied {
                damage.damage
            } else if let Some(source) = source.as_ref() {
                self.battle_service
                    .apply_damage_reduction(
                        damage.damage as f32,
                        source,
                        &mob.status,
                        &Element::Neutral,
                        damage.battle_flags,
                    )
                    .max(0.0)
                    .floor() as u32
            } else {
                damage.damage
            };
            let after_shields = mob.apply_incoming_skill_damage_flags(equipment_damage, damage.battle_flags, damage.skill_id);
            let applied = crate::server::service::map_flag_service::apply_map_combat_damage(
                &map_flags,
                &self.configuration_service.config().game,
                after_shields,
                damage.skill_id,
                damage.battle_flags,
            )
            .min(mob.hp());
            if mob.status_effects.active_statuses != before {
                changed = Some((
                    mob.x,
                    mob.y,
                    StatusEffectService::visual_state_packet(mob.id, &mob.status_effects),
                ));
            }
            if applied == 0 {
                if let Some((x, y, raw)) = changed.take() {
                    self.notify_area_key(&key, x, y, raw);
                }
                return;
            }
            admitted_hit = damage.landed;
            let reflected = physical_reflection(&mob.status, damage.battle_flags, damage.skill_id, applied);
            if reflected > 0 && damage.proc_depth < 8 {
                let returned = Damage {
                    target_id: damage.attacker_id,
                    attacker_id: mob.id,
                    damage: reflected,
                    healing: 0,
                    right_hand_damage: None,
                    attacked_at: tick,
                    damage_motion: 0,
                    battle_flags: 0,
                    skill_id: 0,
                    skill_level: 0,
                    landed: false,
                    proc_depth: damage.proc_depth + 1,
                    credit_id: mob.summon_owner.unwrap_or(mob.id),
                    defenses_applied: true,
                    magic_context: None,
                };
                if source.is_some() {
                    map_instance_tasks_queue.add_to_first_index(MapEvent::MobDamage(returned));
                } else {
                    self.server_task_queue.add_to_first_index(GameEvent::CharacterDamage(returned));
                }
            }
            mob.add_attack(credited_id, applied);
            mob.actor_damages
                .entry(damage.attacker_id)
                .and_modify(|contribution| {
                    contribution.damage = contribution.damage.saturating_add(applied);
                })
                .or_insert(crate::server::model::action::DamageContribution {
                    actor_id: damage.attacker_id,
                    owner_id: credited_id,
                    damage: applied,
                });
            mob.last_attacker_id = damage.attacker_id;
            mob.last_credit_id = credited_id;
            mob.last_attack_flags = damage.battle_flags;
            mob.last_attack_skill = damage.skill_id;
            mob.last_proc_depth = damage.proc_depth;
            mob.last_attacked_at = tick;
            if mob.summon_ai == 2 && mob.next_summon_action == 0 && mob.hp() > 0 {
                mob.next_summon_action = tick.saturating_add(2000);
            }
            mob.transition_to_flinching_with_attacker(damage.attacker_id, tick);
            if damage.landed
                && source.is_none()
                && !is_player_attack
                && credited_id != 0
                && damage.attacker_id != credited_id
                && damage.battle_flags & BattleFlag::Weapon.as_flag() != 0
            {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                        char_id: credited_id,
                        request: crate::server::model::game_systems::ScriptWorldRequest::CompanionAttackLanded {
                            id: damage.attacker_id,
                            damage: applied,
                        },
                    }));
            }
            if damage.landed && is_player_attack && damage.battle_flags != 0 {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ScriptCombat(ScriptCombatRequest {
                        source_id: damage.attacker_id,
                        target_id: mob.id,
                        trigger: CombatTrigger::Attack,
                        battle_flags: damage.battle_flags,
                        skill_id: damage.skill_id,
                        damage: applied,
                        right_hand_damage: damage.admitted_right_hand_damage(applied),
                        other_mob_id: Some(mob.mob_id as u32),
                        depth: damage.proc_depth,
                        drop_position: Some(mob.position()),
                        origin_map: Some(origin_map.clone()),
                    }));
            }
            if damage.landed && damage.skill_id != 0 && damage.proc_depth < 8 {
                self.server_task_queue
                    .add_to_first_index(GameEvent::ScriptSkillHit(crate::server::script::skill::ScriptSkillHit {
                        source_id: damage.attacker_id,
                        target_id: mob.id,
                        skill_id: damage.skill_id,
                        skill_level: damage.skill_level,
                        damage: applied,
                        depth: damage.proc_depth,
                    }));
            }
            if mob.should_die() {
                let delay = damage.attacked_at.saturating_sub(tick);
                Self::add_to_delayed_tick(
                    map_instance_tasks_queue.as_ref(),
                    MapEvent::MobDeathClientNotification(MobLocation {
                        mob_id: mob.id,
                        x: mob.x,
                        y: mob.y,
                    }),
                    delay,
                );
                dead = Some((mob.id, delay / 2));
            }
        }
        if admitted_hit {
            crate::server::script::skill::actor::interrupt_map_cast(map_instance_state, damage.target_id, tick, &self.client_notification_sender);
        }
        if let Some((x, y, raw)) = changed {
            self.notify_area_key(&key, x, y, raw);
        }
        if let Some((id, delay)) = dead {
            self.mob_die(map_instance_state, id, delay);
        }
    }

    pub fn kill_all_mobs(
        &self,
        map_instance_state: &mut MapInstanceState,
        map_instance_tasks_queue: Arc<TasksQueue<MapEvent>>,
        char_id: u32,
    ) {
        let mut ids: Vec<u32> = Vec::with_capacity(map_instance_state.mobs().len());
        for (id, mob) in map_instance_state.mobs_mut().iter_mut() {
            if !mob.is_present() {
                continue;
            }
            debug!("Killing mob {}", id);
            mob.status.set_hp(0);
            mob.add_attack(char_id, 9999);
            mob.last_attacker_id = char_id;
            mob.last_credit_id = char_id;
            Self::add_to_delayed_tick(
                map_instance_tasks_queue.as_ref(),
                MapEvent::MobDeathClientNotification(MobLocation {
                    mob_id: mob.id,
                    x: mob.x,
                    y: mob.y,
                }),
                0,
            );
            ids.push(*id);
        }
        for id in ids {
            self.mob_die(map_instance_state, id, 0);
        }
    }

    pub fn mob_die(&self, map_instance_state: &mut MapInstanceState, id: u32, delay: u128) {
        let (spawn_id, summoned) = {
            let instance_key = map_instance_state.key().clone();
            let player_killer = map_instance_state
                .get_map_item(map_instance_state.get_mob(id).map_or(0, |mob| mob.last_attacker_id))
                .is_some_and(|item| *item.object_type() == MapItemType::Character);
            let mob = map_instance_state
                .mobs_mut()
                .get_mut(&id)
                .unwrap_or_else(|| panic!("can't find mob with id {}", id));
            let mob_model = self.configuration_service.get_mob(mob.mob_id as i32);
            let owner = mob.attacker_with_higher_damage();
            let mut contributions = mob.actor_damages.values().copied().collect::<Vec<_>>();
            contributions.sort_by_key(|contribution| contribution.actor_id);
            if owner != 0 && !(mob.summon_ai != 0 && mob.summon_owner.is_some()) {
                self.server_task_queue.add_to_index(
                    GameEvent::CharacterKillMonster(CharacterKillMonster {
                        char_id: owner,
                        attacker_id: mob.last_attacker_id,
                        mob_id: mob.mob_id,
                        mob_x: mob.x,
                        mob_y: mob.y,
                        map_instance_key: instance_key.clone(),
                        mob_base_exp: mob_model.exp as u32,
                        mob_job_exp: mob_model.job_exp as u32,
                        mob_max_hp: mob.status.max_hp(),
                        contributions,
                    }),
                    delayed_tick(delay, GAME_TICK_RATE),
                );
            }
            if player_killer {
                self.server_task_queue.add_to_index(
                    GameEvent::ScriptCombat(ScriptCombatRequest {
                        source_id: mob.last_attacker_id,
                        target_id: id,
                        trigger: CombatTrigger::Kill,
                        battle_flags: mob.last_attack_flags,
                        skill_id: mob.last_attack_skill,
                        damage: 0,
                        right_hand_damage: None,
                        other_mob_id: Some(mob.mob_id as u32),
                        depth: mob.last_proc_depth,
                        drop_position: Some(mob.position()),
                        origin_map: Some(instance_key.clone()),
                    }),
                    delayed_tick(delay, GAME_TICK_RATE),
                );
            }
            if mob.last_credit_id != 0 {
                self.server_task_queue.add_to_index(
                    GameEvent::TaekwonMissionKill(crate::server::model::events::game_event::TaekwonMissionKill {
                        char_id: mob.last_credit_id,
                        mob_id: mob.mob_id as u32,
                    }),
                    delayed_tick(delay, GAME_TICK_RATE),
                );
            }
            if let Some(entry_id) = mob.event_entry.filter(|_| mob.last_credit_id != 0) {
                self.server_task_queue.add_to_index(
                    GameEvent::ScriptEvent(ScriptEvent {
                        char_id: mob.last_credit_id,
                        entry_id,
                        args: vec![script_sdk::Value::Number(mob.mob_id as i32), script_sdk::Value::Number(id as i32)],
                    }),
                    delayed_tick(delay, GAME_TICK_RATE),
                );
            }
            mob.set_to_remove();
            (mob.spawn_id, mob.summoned)
        };
        if let Some(spawn) = map_instance_state.mob_spawns_tracks_mut().get_mut(&spawn_id).filter(|_| !summoned) {
            spawn.decrement_spawn();
        }
    }

    pub fn remove_dead_mobs(&self, map_instance_state: &mut MapInstanceState) {
        let mobs_to_remove = {
            let mobs = map_instance_state.mobs_mut();
            mobs.iter()
                .filter(|(_k, mob)| !mob.is_present())
                .map(|(k, _)| *k)
                .collect::<Vec<u32>>()
        };
        mobs_to_remove.iter().for_each(|mob| {
            debug!("Remove dead mob {}", mob);
            map_instance_state.remove_mob(*mob);
        });
    }

    pub fn capture_mob(&self, state: &mut MapInstanceState, id: u32) {
        if let Some(mob) = state.remove_mob(id) {
            if let Some(track) = state.mob_spawns_tracks_mut().get_mut(&mob.spawn_id).filter(|_| !mob.summoned) {
                track.decrement_spawn();
            }
            self.mob_die_client_notification(state, MobLocation {
                mob_id: id,
                x: mob.x,
                y: mob.y,
            });
        }
        self.server_task_queue.add_to_first_index(GameEvent::ReleaseScriptCapture(id));
    }

    pub fn mob_die_client_notification(&self, map_instance_state: &MapInstanceState, mob_location: MobLocation) {
        if map_instance_state
            .pending_pet_captures
            .values()
            .any(|claim| claim.mob.id == mob_location.mob_id)
        {
            return;
        }
        let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new(self.configuration_service.packetver());
        packet_zc_notify_vanish.set_gid(mob_location.mob_id);
        packet_zc_notify_vanish.set_atype(VanishType::Die.value() as u8);
        packet_zc_notify_vanish.fill_raw();
        self.client_notification_sender
            .send(Notification::Area(AreaNotification::new(
                map_instance_state.key().map_name().clone(),
                map_instance_state.key().map_instance(),
                AreaNotificationRangeType::Fov {
                    x: mob_location.x,
                    y: mob_location.y,
                    exclude_id: None,
                },
                packet_zc_notify_vanish.raw,
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_notify_vanish to client"));
    }

    pub fn mob_drop_items_and_send_packet(&self, map_instance_state: &mut MapInstanceState, mob_drop_items: MobDropItems) {
        let item_to_drop = self.mob_drop_items(map_instance_state, mob_drop_items);
        self.notify_drop_items(map_instance_state, mob_drop_items.mob_x, mob_drop_items.mob_y, item_to_drop);
    }

    pub fn character_drop_items_and_send_packet(&self, map_instance_state: &mut MapInstanceState, char_drop_items: CharacterDropItems) {
        let mut rng = fastrand::Rng::new();
        let mut item_to_drop: Vec<DroppedItem> = vec![];
        for (item, removal_information) in char_drop_items.item_removal_info {
            if let Some(dropped) = self.drop_items(
                map_instance_state,
                &mut rng,
                char_drop_items.char_x,
                char_drop_items.char_y,
                item.item_id,
                item.is_identified,
                removal_information.amount as u16,
                Some(char_drop_items.owner_id),
                models::item::ItemInstanceAttributes {
                    unique_id: item.unique_id,
                    refine: item.refine,
                    damaged: item.is_damaged,
                    cards: [item.card0, item.card1, item.card2, item.card3],
                },
                true,
            ) {
                item_to_drop.push(dropped);
            }
        }
        self.notify_drop_items(map_instance_state, char_drop_items.char_x, char_drop_items.char_y, item_to_drop);
    }

    pub fn notify_drop_items(&self, map_instance_state: &mut MapInstanceState, x: u16, y: u16, item_to_drop: Vec<DroppedItem>) {
        let mut packets = vec![];
        for item in item_to_drop.iter() {
            let mut packet_zc_item_fall_entry = PacketZcItemFallEntry::new(self.configuration_service.packetver());
            packet_zc_item_fall_entry.set_itid(item.item_id as u16);
            packet_zc_item_fall_entry.set_itaid(item.map_item_id);
            packet_zc_item_fall_entry.set_x_pos(item.location.x as i16);
            packet_zc_item_fall_entry.set_y_pos(item.location.y as i16);
            packet_zc_item_fall_entry.set_sub_x(item.sub_location.x as u8);
            packet_zc_item_fall_entry.set_sub_y(item.sub_location.y as u8);
            packet_zc_item_fall_entry.set_is_identified(item.is_identified);
            packet_zc_item_fall_entry.set_count(item.amount as i16);
            packet_zc_item_fall_entry.fill_raw();
            packets.extend(packet_zc_item_fall_entry.raw);
        }
        self.client_notification_sender
            .send(Notification::Area(AreaNotification::new(
                map_instance_state.key().map_name().clone(),
                map_instance_state.key().map_instance(),
                AreaNotificationRangeType::Fov { x, y, exclude_id: None },
                packets,
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_item_fall_entry to client"));
    }

    pub fn mob_drop_items(&self, map_instance_state: &mut MapInstanceState, mob_drop_items: MobDropItems) -> Vec<DroppedItem> {
        if map_instance_state
            .flags
            .enabled(crate::server::model::map_flags::MapFlag::NoMobLoot)
        {
            return vec![];
        }
        let mut rng = fastrand::Rng::new();
        let mob = self.configuration_service.get_mob(mob_drop_items.mob_id as i32);
        let mut item_to_drop: Vec<DroppedItem> = vec![];
        for drop in mob.drops.iter() {
            let drop_rate = if drop.is_card {
                (drop.rate as f32 * self.configuration_service.config().game.drop_rate_card).round() as u16
            } else {
                (drop.rate as f32 * self.configuration_service.config().game.drop_rate).round() as u16
            };
            if drop_rate >= 10000 || rng.u16(1..=10000) > 10000 - drop_rate {
                let item = self.configuration_service.get_item(drop.item_id);
                if let Some(dropped) = self.drop_items(
                    map_instance_state,
                    &mut rng,
                    mob_drop_items.mob_x,
                    mob_drop_items.mob_y,
                    drop.item_id,
                    !item.item_type.should_be_identified_when_dropped(),
                    1,
                    Some(mob_drop_items.owner_id),
                    Default::default(),
                    false,
                ) {
                    item_to_drop.push(dropped);
                }
            }
        }
        item_to_drop
    }

    fn drop_items(
        &self,
        map_instance_state: &mut MapInstanceState,
        rng: &mut fastrand::Rng,
        x: u16,
        y: u16,
        item_id: i32,
        is_identified: bool,
        amount: u16,
        owner_id: Option<u32>,
        attributes: models::item::ItemInstanceAttributes,
        player_dropped: bool,
    ) -> Option<DroppedItem> {
        let item = self.prepare_dropped_item(
            map_instance_state,
            rng,
            x,
            y,
            item_id,
            is_identified,
            amount,
            owner_id,
            attributes,
            player_dropped,
        )?;
        map_instance_state.insert_dropped_item(item);
        Some(item)
    }

    fn prepare_dropped_item(
        &self,
        map_instance_state: &mut MapInstanceState,
        rng: &mut fastrand::Rng,
        x: u16,
        y: u16,
        item_id: i32,
        is_identified: bool,
        amount: u16,
        owner_id: Option<u32>,
        attributes: models::item::ItemInstanceAttributes,
        player_dropped: bool,
    ) -> Option<DroppedItem> {
        let mut locations = nearby_walkable_locations(
            map_instance_state.cells(),
            map_instance_state.x_size(),
            map_instance_state.y_size(),
            x as i32,
            y as i32,
        );
        if locations.is_empty() {
            locations = spawn_locations(
                map_instance_state.cells(),
                map_instance_state.x_size(),
                map_instance_state.y_size(),
                0,
                0,
            );
            let nearest = locations
                .iter()
                .map(|(px, py)| px.abs_diff(x) as u32 + py.abs_diff(y) as u32)
                .min()?;
            locations.retain(|(px, py)| px.abs_diff(x) as u32 + py.abs_diff(y) as u32 == nearest);
        }
        let (random_x, random_y) = locations[rng.usize(0..locations.len())];
        let map_item_id = map_instance_state.map_items_mut().generate_id();
        let dropped_item = DroppedItem {
            map_item_id,
            item_id,
            location: Position {
                x: random_x,
                y: random_y,
                dir: 0,
            },
            sub_location: Position {
                x: rng.u16(0..=3) * 3 + 3,
                y: rng.u16(0..=3) * 3 + 3,
                dir: 0,
            },
            owner_id,
            dropped_at: get_tick(),
            amount,
            is_identified,
            attributes,
            player_dropped,
        };
        Some(dropped_item)
    }

    pub fn remove_dropped_item_from_map(&self, map_instance_state: &mut MapInstanceState, dropped_item_id: u32) {
        if map_instance_state.is_pet_loot_reserved(dropped_item_id) {
            return;
        }
        if let Some(dropped_item) = map_instance_state.remove_dropped_item(dropped_item_id) {
            let mut packet_zc_item_disappear = PacketZcItemDisappear::new(self.configuration_service.packetver());
            packet_zc_item_disappear.set_itaid(dropped_item_id);
            packet_zc_item_disappear.fill_raw();
            self.client_notification_sender
                .send(Notification::Area(AreaNotification::new(
                    map_instance_state.key().map_name().clone(),
                    map_instance_state.key().map_instance(),
                    AreaNotificationRangeType::Fov {
                        x: dropped_item.x(),
                        y: dropped_item.y(),
                        exclude_id: None,
                    },
                    packet_zc_item_disappear.raw,
                )))
                .unwrap_or_else(|_| error!("Failed to send notification packet_zc_item_disappear to client"));
        }
        self.server_task_queue
            .add_to_first_index(GameEvent::MapNotifyItemRemoved(dropped_item_id));
    }

    fn add_to_delayed_tick(map_instance_tasks_queue: &TasksQueue<MapEvent>, event: MapEvent, delay: u128) {
        map_instance_tasks_queue.add_to_index(event, delayed_tick(delay, MAP_LOOP_TICK_RATE));
    }
}

fn walkable(cells: &[u16], width: u16, height: u16, x: i32, y: i32) -> bool {
    x >= 0
        && y >= 0
        && x < width as i32
        && y < height as i32
        && cells
            .get(y as usize * width as usize + x as usize)
            .is_some_and(|cell| cell & CellType::Walkable.as_flag() != 0)
}

fn spawn_locations(cells: &[u16], width: u16, height: u16, x: i32, y: i32) -> Vec<(u16, u16)> {
    if width == 0 || height == 0 {
        return Vec::new();
    }
    let random = x == 0 && y == 0;
    let (min_x, max_x, min_y, max_y) = if random {
        (0, width as i32 - 1, 0, height as i32 - 1)
    } else {
        (
            (x - 1).max(0),
            (x + 1).min(width as i32 - 1),
            (y - 1).max(0),
            (y + 1).min(height as i32 - 1),
        )
    };
    let mut locations = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if walkable(cells, width, height, x, y) {
                locations.push((x as u16, y as u16));
            }
        }
    }
    locations
}

fn nearby_walkable_locations(cells: &[u16], width: u16, height: u16, x: i32, y: i32) -> Vec<(u16, u16)> {
    let mut locations = Vec::new();
    for py in y.saturating_sub(1)..=y.saturating_add(1) {
        for px in x.saturating_sub(1)..=x.saturating_add(1) {
            if walkable(cells, width, height, px, py) {
                locations.push((px as u16, py as u16));
            }
        }
    }
    locations
}

fn knockback_position(cells: &[u16], width: u16, height: u16, from: Position, source_x: u16, source_y: u16, distance: u16) -> Position {
    let dx = (from.x as i32 - source_x as i32).signum();
    let dy = (from.y as i32 - source_y as i32).signum();
    let mut to = from;
    for _ in 0..distance {
        let x = to.x as i32 + dx;
        let y = to.y as i32 + dy;
        if !walkable(cells, width, height, x, y) {
            break;
        }
        to.x = x as u16;
        to.y = y as u16;
    }
    to
}
