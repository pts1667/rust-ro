use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::movement::{Movable, Movement};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::util::tick::get_tick;

pub struct MapInstanceLoop;

pub const MOVEMENT_TICK_RATE: u128 = 20;
pub const MAP_LOOP_TICK_RATE: u128 = 40;

impl MapInstanceLoop {
    pub fn start_map_instance_thread(map_instance: Arc<MapInstance>, map_instance_service: MapInstanceService) {
        let map_instance_clone_for_thread = map_instance.clone();
        let map_instance_service = Arc::new(map_instance_service);
        info!("Start thread for {}", map_instance.name());
        let map_instance_service_clone = map_instance_service.clone();
        thread::Builder::new()
            .name(format!("map_instance_{}_loop_thread", map_instance.name()))
            .spawn(move || {
                let map_instance = map_instance_clone_for_thread;
                let mut last_mobs_spawn = Instant::now();
                let mut last_mobs_action = Instant::now();
                loop {
                    if !map_instance.is_alive() {
                        break;
                    }
                    let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
                    let now = Instant::now();
                    map_instance_service_clone.tick_mob_statuses(
                        map_instance.state_mut().as_mut(),
                        map_instance.task_queue().as_ref(),
                        tick,
                    );
                    if !map_instance.state().mob_movement_paused()
                        && last_mobs_action.elapsed().as_millis()
                            >= (GlobalConfigService::instance().config().game.mob_action_refresh_frequency * 1000.0) as u128
                    {
                        let map_instance_state = map_instance.state_mut().as_mut();
                        map_instance_service_clone.mobs_action(map_instance_state, map_instance.task_queue(), tick);
                        last_mobs_action = now;
                    }
                    if last_mobs_spawn.elapsed().as_millis()
                        >= (GlobalConfigService::instance().config().game.mob_spawn_refresh_frequency * 1000.0) as u128
                    {
                        map_instance_service_clone.spawn_mobs(map_instance.map(), map_instance.state_mut().as_mut());
                        last_mobs_spawn = now;
                    }

                    if let Some(tasks) = map_instance.pop_task() {
                        for task in tasks {
                            match task {
                                MapEvent::ActorSkillCast(request) => {
                                    if let Err(error) = map_instance_service_clone.start_actor_skill(map_instance.state_mut().as_mut(), request, tick) {
                                        error!("Actor skill cast failed on {}: {}", map_instance.name(), error);
                                    }
                                }
                                MapEvent::SetMapFlags(flags) => {
                                    map_instance.state_mut().flags = flags;
                                }
                                MapEvent::CaptureMob(id) => {
                                    map_instance_service_clone.capture_mob(map_instance.state_mut().as_mut(), id);
                                }
                                MapEvent::ClaimPetCapture(request) => {
                                    map_instance_service_clone.claim_pet_capture(map_instance.state_mut().as_mut(), request, tick);
                                }
                                MapEvent::MobStatusAlternatives(request) => {
                                    map_instance_service_clone.start_mob_status_alternatives(
                                        map_instance.state_mut().as_mut(),
                                        request,
                                        tick,
                                    );
                                }
                                MapEvent::MobProvoke(request) => {
                                    map_instance_service_clone.provoke_mob(map_instance.state_mut().as_mut(), request, tick);
                                }
                                MapEvent::MobDispel(request) => {
                                    map_instance_service_clone.dispel_mob(map_instance.state_mut().as_mut(), request.mob_id);
                                }
                                MapEvent::FinalizePetCapture(request) => {
                                    map_instance_service_clone.finalize_pet_capture(map_instance.state_mut().as_mut(), request);
                                }
                                MapEvent::ClaimPetLoot(request) => {
                                    map_instance_service_clone.claim_pet_loot(map_instance.state_mut().as_mut(), request, tick);
                                }
                                MapEvent::FinalizePetLoot(request) => {
                                    map_instance_service_clone.finalize_pet_loot(map_instance.state_mut().as_mut(), request);
                                }
                                MapEvent::PreparePetLootDrop(request) => {
                                    map_instance_service_clone.prepare_pet_loot_drop(map_instance.state_mut().as_mut(), request);
                                }
                                MapEvent::FinalizePetLootDrop(request) => {
                                    map_instance_service_clone.finalize_pet_loot_drop(map_instance.state_mut().as_mut(), request);
                                }
                                MapEvent::UpdateMobsFov(characters) => {
                                    let map_instance_state = map_instance.state_mut().as_mut();
                                    map_instance_service_clone.update_mobs_fov(map_instance_state, characters);
                                }
                                MapEvent::UpdateActorVisibility(actors) => {
                                    map_instance.state_mut().actor_visibility = actors.into_iter().collect();
                                }
                                MapEvent::MobDamage(damage) => {
                                    let mut map_instance_state = map_instance.state_mut();
                                    map_instance_service_clone.mob_being_attacked(
                                        map_instance_state.as_mut(),
                                        damage,
                                        map_instance.task_queue(),
                                        tick,
                                    );
                                }
                                MapEvent::MobStatusChange { mob_id, request } => {
                                    map_instance_service_clone.start_mob_status(map_instance.state_mut().as_mut(), mob_id, request, tick);
                                }
                                MapEvent::MobEndStatus { mob_id, kind } => {
                                    map_instance_service_clone.end_mob_status(map_instance.state_mut().as_mut(), mob_id, kind);
                                }
                                MapEvent::MobHeal { mob_id, hp, sp } => {
                                    map_instance_service_clone.heal_mob(map_instance.state_mut().as_mut(), mob_id, hp, sp);
                                }
                                MapEvent::MobRandomWarp { mob_id } => {
                                    map_instance_service_clone.random_warp_mob(map_instance.state_mut().as_mut(), mob_id);
                                }
                                MapEvent::MobFace(request) => {
                                    map_instance_service_clone.face_mob(map_instance.state_mut().as_mut(), request.mob_id, request.dir);
                                }
                                MapEvent::MobWarpTo(request) => {
                                    map_instance_service_clone.warp_mob_to(
                                        map_instance.state_mut().as_mut(),
                                        request.mob_id,
                                        request.x,
                                        request.y,
                                    );
                                }
                                MapEvent::MobKnockback {
                                    mob_id,
                                    source_x,
                                    source_y,
                                    cells,
                                } => {
                                    map_instance_service_clone.mob_knockback(
                                        map_instance.state_mut().as_mut(),
                                        mob_id,
                                        source_x,
                                        source_y,
                                        cells,
                                    );
                                }
                                MapEvent::MobLoseTarget { mob_id } => {
                                    if let Some(mob) = map_instance.state_mut().mobs_mut().get_mut(&mob_id) {
                                        mob.lose_target();
                                    }
                                }
                                MapEvent::ScriptMobCombat {
                                    source_id,
                                    target_id,
                                    effect,
                                } => {
                                    map_instance_service_clone.script_mob_combat(
                                        map_instance.state_mut().as_mut(),
                                        source_id,
                                        target_id,
                                        effect,
                                        map_instance.task_queue(),
                                        tick,
                                    );
                                }
                                MapEvent::ScriptDropItem {
                                    owner_id,
                                    item_id,
                                    amount,
                                    x,
                                    y,
                                } => {
                                    map_instance_service_clone.script_drop_item(
                                        map_instance.state_mut().as_mut(),
                                        owner_id,
                                        item_id,
                                        amount,
                                        x,
                                        y,
                                    );
                                }
                                MapEvent::ScriptSpawn(request) => {
                                    if let Err(error) = map_instance_service_clone.script_spawn(map_instance.state_mut().as_mut(), request)
                                    {
                                        error!("Script monster spawn failed on {}: {}", map_instance.name(), error);
                                    }
                                }
                                MapEvent::MobDeathClientNotification(mob_location) => {
                                    let map_instance_state = map_instance.state();
                                    map_instance_service_clone.mob_die_client_notification(map_instance_state.as_ref(), mob_location);
                                }
                                MapEvent::RemoveCharFromMap(char_id) => {
                                    map_instance.state_mut().remove_item_with_id(char_id);
                                }
                                MapEvent::InsertCharToMap(map_item) => {
                                    map_instance.state_mut().insert_item(map_item);
                                }
                                MapEvent::MobDropItems(mob_drop_items) => {
                                    map_instance_service_clone
                                        .mob_drop_items_and_send_packet(map_instance.state_mut().as_mut(), mob_drop_items);
                                }
                                MapEvent::RemoveDroppedItemFromMap(dropped_item_id) => {
                                    map_instance_service_clone
                                        .remove_dropped_item_from_map(map_instance.state_mut().as_mut(), dropped_item_id);
                                }
                                MapEvent::CharDropItems(character_drop_items) => {
                                    map_instance_service_clone
                                        .character_drop_items_and_send_packet(map_instance.state_mut().as_mut(), character_drop_items);
                                }
                                MapEvent::AdminKillAllMobs(char_id) => {
                                    map_instance_service_clone.kill_all_mobs(
                                        map_instance.state_mut().as_mut(),
                                        map_instance.task_queue(),
                                        char_id,
                                    );
                                }
                                MapEvent::AdminTogglePauseMobMovement => {
                                    let map_instance_state = map_instance.state_mut().as_mut();
                                    map_instance_state.set_mob_movement_paused(!map_instance_state.mob_movement_paused());
                                }
                                MapEvent::MobAttackCharacter(attack) => {
                                    let map_instance_state = map_instance.state();
                                    map_instance_service_clone.mob_attack_character(
                                        map_instance_state.as_ref(),
                                        attack,
                                        map_instance.task_queue().as_ref(),
                                        tick,
                                    );
                                }
                            }
                        }
                    }
                    map_instance_service_clone.tick_actor_skills(map_instance.state_mut().as_mut(), tick);
                    let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
                    let sleep_duration = (MAP_LOOP_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
                    if sleep_duration < 5 {
                        warn!(
                            "Less than 5 milliseconds of sleep, map_instance_{}_loop is too slow - {}ms because game loop took {}ms",
                            map_instance.name(),
                            sleep_duration,
                            time_spent
                        );
                    }
                    sleep(Duration::from_millis(sleep_duration));
                }
                info!("Shutdown map_instance_{}_loop_thread", map_instance.name());
            })
            .unwrap();
        thread::Builder::new()
            .name(format!("map_instance_{}_mob_movement_thread", map_instance.name()))
            .spawn(move || {
                loop {
                    if !map_instance.is_alive() {
                        break;
                    }
                    let tick = get_tick();
                    let mut map_instance_state = map_instance.state_mut();

                    map_instance_service.remove_dead_mobs(&mut map_instance_state);
                    let mobs = map_instance_state.mobs_mut();
                    for mob in mobs.values_mut() {
                        // Update flinch state - transition to Idle when done
                        mob.update_flinch(tick);

                        // Skip movement if not moving
                        if !mob.is_moving() {
                            continue;
                        }

                        let speed = mob.status.speed();
                        if let Some(movement) = mob.peek_movement() {
                            if tick >= movement.move_at() {
                                // Check state machine - skip if flinching
                                if !mob.can_move(tick) || mob.is_flinching() {
                                    #[cfg(feature = "debug_mob_movement")]
                                    {
                                        info!("Mob delayed movement because he is flinching");
                                    }
                                    continue;
                                }
                                let movement = mob.pop_movement().unwrap();
                                #[cfg(feature = "debug_mob_movement")]
                                {
                                    info!("mob {} move {} at {}", mob.id, movement.position(), movement.move_at());
                                }
                                mob.set_last_moved_at(tick);
                                mob.update_position(movement.position().x, movement.position().y);

                                if let Some(next_movement) = mob.peek_mut_movement() {
                                    next_movement.set_move_at(tick + Movement::delay(speed, next_movement.is_diagonal()))
                                } else {
                                    // Movement complete - transition to Idle
                                    mob.update_movement_complete();
                                }
                            }
                        }
                    }
                    let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
                    let sleep_duration = (MOVEMENT_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
                    if sleep_duration < 5 {
                        warn!(
                            "Mob Movement loop: less than 5 milliseconds of sleep, movement loop is too slow - {}ms because movement loop \
                             took {}ms",
                            sleep_duration, time_spent
                        );
                    }
                    sleep(Duration::from_millis(sleep_duration));
                }
                info!("Shutdown map_instance_{}_mob_movement_thread", map_instance.name());
            })
            .unwrap();
    }
}
