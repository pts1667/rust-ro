use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Instant;

use crate::server::model::events::map_event::MapEventContext;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::movement::{Movable, Movement};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::util::tick::{get_tick, real_duration, too_little_sleep};

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
                let mut last_mobs_spawn = get_tick();
                let mut last_mobs_action = get_tick();
                loop {
                    if !map_instance.is_alive() {
                        break;
                    }
                    let started = Instant::now();
                    let tick = get_tick();
                    map_instance_service_clone.tick_npc_statuses(map_instance.state_mut().as_mut(), tick);
                    map_instance_service_clone.tick_mob_statuses(
                        map_instance.state_mut().as_mut(),
                        map_instance.task_queue().as_ref(),
                        tick,
                    );
                    if !map_instance.state().mob_movement_paused()
                        && tick.saturating_sub(last_mobs_action)
                            >= (GlobalConfigService::instance().config().game.mob_action_refresh_frequency * 1000.0) as u128
                    {
                        let mut map_instance_guard = map_instance.state_mut();
                        let map_instance_state = map_instance_guard.as_mut();
                        map_instance_service_clone.mobs_action(map_instance_state, map_instance.task_queue(), tick);
                        map_instance_service_clone.mobs_skill_ai(map_instance_state, tick);
                        last_mobs_action = tick;
                    }
                    if tick.saturating_sub(last_mobs_spawn)
                        >= (GlobalConfigService::instance().config().game.mob_spawn_refresh_frequency * 1000.0) as u128
                    {
                        map_instance_service_clone.spawn_mobs(map_instance.map(), map_instance.state_mut().as_mut());
                        last_mobs_spawn = tick;
                    }

                    if let Some(tasks) = map_instance.pop_task() {
                        for task in tasks {
                            if map_instance_service_clone.handle_npc_map_event(map_instance.state_mut().as_mut(), &task, tick) {
                                continue;
                            }
                            task.dispatch(&MapEventContext {
                                service: &map_instance_service_clone,
                                map_instance: &map_instance,
                                tick,
                            });
                        }
                    }
                    map_instance_service_clone.tick_actor_skills(map_instance.state_mut().as_mut(), tick);
                    let time_spent = started.elapsed();
                    let sleep_duration = real_duration(MAP_LOOP_TICK_RATE).saturating_sub(time_spent);
                    if too_little_sleep(sleep_duration, real_duration(MAP_LOOP_TICK_RATE)) {
                        warn!(
                            "Less than 5 milliseconds of sleep, map_instance_{}_loop is too slow - {}ms because game loop took {}ms",
                            map_instance.name(),
                            sleep_duration.as_millis(),
                            time_spent.as_millis()
                        );
                    }
                    sleep(sleep_duration);
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
                    let started = Instant::now();
                    let tick = get_tick();
                    let mut map_instance_state = map_instance.state_mut();

                    map_instance_service.remove_dead_mobs(&mut map_instance_state);
                    let key = map_instance_state.key().clone();
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
                                    map_instance_service.mob_walk_ended(mob, &key);
                                }
                            }
                        }
                    }
                    map_instance_service.step_npc_walks(&mut map_instance_state, tick);
                    drop(map_instance_state);
                    let time_spent = started.elapsed();
                    let sleep_duration = real_duration(MOVEMENT_TICK_RATE).saturating_sub(time_spent);
                    if too_little_sleep(sleep_duration, real_duration(MOVEMENT_TICK_RATE)) {
                        warn!(
                            "Mob Movement loop: less than 5 milliseconds of sleep, movement loop is too slow - {}ms because movement loop \
                             took {}ms",
                            sleep_duration.as_millis(),
                            time_spent.as_millis()
                        );
                    }
                    sleep(sleep_duration);
                }
                info!("Shutdown map_instance_{}_mob_movement_thread", map_instance.name());
            })
            .unwrap();
    }
}
