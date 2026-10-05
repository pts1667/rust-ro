use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use packets::packets::{Packet, PacketZcNotifyMove, PacketZcNotifyPlayermove};

use crate::server::Server;
use crate::server::model::events::client_notification::{AreaNotification, CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterSavePosition, GameEvent};
use crate::server::model::events::map_event::{MapEvent, UpdateActorVisibility, UpdateMobsFov};
use crate::server::model::map_item::ToMapItemSnapshot;
use crate::server::model::movement::{Movable, Movement};
use crate::server::service::global_config_service::GlobalConfigService;

const MOVEMENT_TICK_RATE: u128 = 20;
pub const GAME_TICK_RATE: u128 = 40;

impl Server {
    pub(crate) fn game_loop(server_ref: Arc<Server>) {
        server_ref.bind_shared();
        server_ref.schedule_npc_initialization(&server_ref.state());
        loop {
            if !server_ref.is_alive() {
                break;
            }
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            Self::game_loop_iteration(server_ref.clone().as_ref(), tick);
            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (GAME_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!(
                    "Less than 5 milliseconds of sleep, game loop is too slow - {}ms because game loop took {}ms",
                    sleep_duration, time_spent
                );
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }

    pub(crate) fn game_loop_iteration(server_ref: &Server, tick: u128) {
        let _state_loops_guard = server_ref.lock_state_loops();
        let mut server_state_mut = server_ref.state_mut();
        server_ref.drain_map_notifications(server_state_mut.as_mut());
        server_ref.tick_player_trades(server_state_mut.as_mut(), tick as u64);
        server_ref.tick_character_logouts(server_state_mut.as_mut(), tick);
        server_ref.tick_script_timers(server_state_mut.as_mut(), tick);

        let actor_ids: Vec<_> = server_state_mut
            .characters()
            .values()
            .filter(|character| character.loaded_from_client_side)
            .map(|character| character.char_id)
            .collect();
        let sender = server_ref.server_service().notification_sender();
        for char_id in &actor_ids {
            if let Some(mut character) = server_state_mut.characters_mut().remove(char_id) {
                character.refresh_script_context();
                server_ref.server_service.character_remove_expired_bonuses(&mut character, tick);
                crate::server::service::status_effect_service::StatusEffectService::tick(server_ref, &mut character, tick, &sender);
                server_ref.script_skill_service().tick_character_state(&mut character, tick);
                server_ref
                    .script_skill_service()
                    .tick_revealing_statuses(server_ref, server_state_mut.as_mut(), &mut character, tick);
                server_ref
                    .script_skill_service()
                    .tick_devotion_links(server_ref, server_state_mut.as_mut(), &mut character, tick);
                server_ref
                    .script_skill_service()
                    .tick_stealth(server_ref, server_state_mut.as_mut(), &mut character, tick);
                crate::server::service::script_combat_service::tick_character(server_ref, &mut character, tick);
                if let Err(error) =
                    server_ref
                        .script_world_service()
                        .tick_in_state(server_ref, server_state_mut.as_mut(), &mut character, tick as u64)
                {
                    warn!("World tick failed: {error}");
                }
                server_state_mut.insert_character(character);
            }
        }
        server_ref
            .script_skill_service()
            .sync_ground_unit_snapshots(server_state_mut.as_mut(), tick);
        server_ref
            .script_skill_service()
            .tick_ground_skills(server_ref, server_state_mut.as_mut(), tick);
        server_ref
            .script_skill_service()
            .sync_ground_unit_snapshots(server_state_mut.as_mut(), tick);
        if let Some(tasks) = server_ref.pop_task() {
            for task in tasks {
                let (task, logout_owner) = if let GameEvent::ScriptLogoutAction(action) = task {
                    let valid = server_state_mut
                        .pending_character_logouts
                        .get(&action.char_id)
                        .is_some_and(|pending| pending.current.as_ref().is_some_and(|current| current.token == action.token));
                    if !valid {
                        continue;
                    }
                    (*action.action, Some(action.char_id))
                } else {
                    (task, None)
                };
                if task
                    .required_character()
                    .is_some_and(|char_id| !server_state_mut.characters().contains_key(&char_id))
                    || task.affects_character(|char_id| {
                        server_state_mut.pending_character_logouts.contains_key(&char_id) && logout_owner != Some(char_id)
                    })
                {
                    continue;
                }
                let event_name = task.name();
                if let Err(error) = task.dispatch(server_ref, server_state_mut.as_mut(), tick) {
                    warn!("{event_name} failed: {error}");
                }
            }
        }
        let actor_ids: Vec<_> = server_state_mut
            .characters()
            .values()
            .filter(|character| character.loaded_from_client_side)
            .map(|character| character.char_id)
            .collect();
        server_ref
            .script_skill_service()
            .sync_ground_unit_snapshots(server_state_mut.as_mut(), tick);
        server_ref.apply_guild_auras(server_state_mut.as_mut(), tick);
        server_ref.castle_clock();
        server_ref.tick_battlegrounds(server_state_mut.as_mut(), tick);
        server_ref.tick_cell_statuses(server_state_mut.as_mut(), tick);
        server_ref.tick_battleground_queues(server_state_mut.as_mut(), tick);
        for char_id in actor_ids {
            if let Some(mut character) = server_state_mut.characters_mut().remove(&char_id) {
                let map_instance = server_state_mut.get_map_instance_from_character(&character);
                if let Some(map_instance) = map_instance {
                    server_ref.character_service().load_units_in_fov(
                        server_state_mut.as_mut(),
                        &mut character,
                        map_instance.state().borrow().as_ref(),
                    );
                }
                let target_visible = !character.is_attacking()
                    || server_ref.player_target_allowed(
                        server_state_mut.as_mut(),
                        &character,
                        character.attack().target,
                        crate::server::service::visibility_service::TargetingMode::Direct,
                    );
                if target_visible
                    && !character.status.blocks_attack()
                    && character.game_systems.buying_store.is_none()
                    && character.game_systems.vending_store.is_none()
                {
                    server_ref
                        .server_service
                        .character_attack(server_ref, server_state_mut.as_mut(), tick, &mut character);
                } else {
                    character.clear_attack();
                }
                server_ref
                    .server_service
                    .character_pending_skill(server_ref, server_state_mut.as_mut(), tick, &mut character);
                server_ref
                    .server_service
                    .character_use_skill(server_ref, server_state_mut.as_mut(), tick, &mut character);
                server_ref.character_service().regen_hp(&mut character, tick);
                server_ref.character_service().regen_sp(&mut character, tick);
                character.refresh_script_context();
                server_state_mut.insert_character(character);
            }
        }
        for (_, map) in server_state_mut.map_instances().iter() {
            for instance in map.iter() {
                let map_name = instance.key().map_name();
                let instance_id = instance.key().map_instance();
                instance.add_to_next_tick(MapEvent::UpdateActorVisibility(UpdateActorVisibility { actors: server_state_mut
                        .characters()
                        .values()
                        .filter(|character| character.current_map_name() == map_name && character.current_map_instance() == instance_id)
                        .flat_map(|character| {
                            use crate::server::service::visibility_service::StealthState;
                            let mut visibility = vec![(
                                character.char_id,
                                StealthState::from_status_options(&character.status, character.options),
                            )];
                            for companion in crate::server::service::script_world_service::companion_snapshots(character) {
                                let id = companion.map_item().id();
                                if let Some(status) = crate::server::service::script_world_service::companion_status_snapshot(character, id)
                                {
                                    visibility.push((id, StealthState::from_snapshot(&status)));
                                }
                            }
                            visibility
                        })
                        .collect(), }));
                instance.add_to_next_tick(MapEvent::UpdateMobsFov(UpdateMobsFov { characters: server_state_mut
                        .characters()
                        .iter()
                        .filter(|(_, character)| {
                            character.status.hp > 0
                                && character.current_map_name() == map_name
                                && character.current_map_instance() == instance_id
                        })
                        .flat_map(|(_, character)| {
                            let mut actors = vec![character.to_map_item_snapshot()];
                            actors.extend(crate::server::service::script_world_service::companion_snapshots(character));
                            actors
                        })
                        .collect(), }));
            }
        }
    }

    pub(crate) fn character_movement_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>) {
        loop {
            if !server_ref.is_alive() {
                break;
            }
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let state_loops_guard = server_ref.lock_state_loops();
            let mut server_state_mut = server_ref.state_mut();
            if let Some(tasks) = server_ref.pop_movement_task() {
                for task in tasks {
                    if let GameEvent::CharacterMove(character_movement) = task {
                        if server_state_mut.pending_character_logouts.contains_key(&character_movement.char_id) {
                            continue;
                        }
                        let Some(character) = server_state_mut.characters_mut().get_mut(&character_movement.char_id) else {
                            continue;
                        };
                        if character.status.blocks_movement()
                            || character.game_systems.is_trading()
                            || character.game_systems.buying_store.is_some()
                            || character.game_systems.vending_store.is_some()
                        {
                            continue;
                        }
                        if character_movement.cancel_attack {
                            character.clear_attack();
                            character.clear_pending_skill();
                        }
                        let status_snapshot = server_ref.server_service.get_status_snapshot(&character.status, tick);
                        let speed = status_snapshot.speed();
                        if character_movement.path.last().is_none() {
                            continue;
                        }
                        let new_movement = character_movement.path.last().unwrap();
                        let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new(GlobalConfigService::instance().packetver());

                        let move_at = tick + Movement::delay(speed, new_movement.is_diagonal());
                        if let Some(previous_movement) = character.peek_movement() {
                            #[cfg(feature = "debug_movement")]
                            {
                                if tick >= previous_movement.move_at() {
                                    info!(
                                        "change path! was {} will {}, move at {}",
                                        previous_movement.position(),
                                        new_movement.position(),
                                        move_at
                                    );
                                    crate::util::debug::debug_in_game_chat(
                                        client_notification_sender_clone.clone(),
                                        character,
                                        format!(
                                            "change path! was {} will {}, move at {}",
                                            previous_movement.position(),
                                            new_movement.position(),
                                            move_at
                                        ),
                                    );
                                } else {
                                    server_ref.add_to_next_movement_tick(GameEvent::CharacterMove(character_movement));
                                    continue;
                                }
                            }
                            #[cfg(not(feature = "debug_movement"))]
                            {
                                if tick < previous_movement.move_at() {
                                    server_ref.add_to_next_movement_tick(GameEvent::CharacterMove(character_movement));
                                    continue;
                                }
                            }
                        }
                        character.set_movement(character_movement.path);
                        character.transition_to_moving();
                        let movement = character.peek_mut_movement().unwrap();
                        movement.set_move_at(move_at);
                        let moved_at = movement.move_at() as u32;
                        packet_zc_notify_playermove.set_move_start_time(moved_at); // todo: time conversion check on client side ???
                        let movement = character_movement.current_position.to_move_data(&character_movement.destination);
                        packet_zc_notify_playermove.set_move_data(movement.clone());
                        packet_zc_notify_playermove.fill_raw();
                        client_notification_sender_clone
                            .send(Notification::Char(CharNotification::new(
                                character.char_id,
                                std::mem::take(packet_zc_notify_playermove.raw_mut()),
                            )))
                            .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                        let mut packet_zc_notify_move = PacketZcNotifyMove::new(GlobalConfigService::instance().packetver());
                        packet_zc_notify_move.set_move_data(movement);
                        packet_zc_notify_move.set_gid(character.char_id);
                        packet_zc_notify_move.set_move_start_time(moved_at as u32);
                        packet_zc_notify_move.fill_raw();
                        client_notification_sender_clone
                            .send(Notification::Area(AreaNotification::from_character_exclude_self(
                                character,
                                std::mem::take(packet_zc_notify_move.raw_mut()),
                            )))
                            .expect("Failed to send notification event with PacketZcNotifyPlayermove");
                    }
                }
            }

            // If movement not smooth:
            // teleport in front -> server movement faster than client movement
            // teleport back -> server movement slower than client movement
            let mut character_finished_to_move = vec![];
            for (_, character) in server_state_mut
                .characters_mut()
                .iter_mut()
                .filter(|(_, character)| character.is_moving())
            {
                let status_snapshot = server_ref.server_service.get_status_snapshot(&character.status, tick);
                let speed = status_snapshot.speed();
                if let Some(movement) = character.peek_movement() {
                    if tick >= movement.move_at() {
                        let movement = character.pop_movement().unwrap();
                        #[cfg(feature = "debug_movement")]
                        {
                            let last_move_at = character.last_moved_at;
                            info!(
                                "move {} at {} after {}ms since last move",
                                movement.position(),
                                tick,
                                tick - last_move_at
                            );
                            crate::util::debug::debug_in_game_chat(
                                client_notification_sender_clone.clone(),
                                character,
                                format!(
                                    "move {} at {} after {}ms since last move",
                                    movement.position(),
                                    tick,
                                    tick - last_move_at
                                ),
                            );
                        }
                        character.set_last_moved_at(tick);
                        character.update_position(movement.position().x, movement.position().y);
                        let map_ref = server_ref.state().get_map_instance_from_character(character);
                        if let Some(map_ref) = map_ref {
                            if map_ref.state().is_warp_cell(movement.position().x, movement.position().y) {
                                let warp = map_ref.get_warp_at(movement.position().x, movement.position().y).unwrap();
                                server_ref.server_service.schedule_warp_to_walkable_cell(
                                    server_ref.state_mut().as_mut(),
                                    warp.dest_map_name.as_str(),
                                    warp.to_x,
                                    warp.to_y,
                                    character.char_id,
                                );
                                character.clear_movement();
                                continue;
                            }
                        }
                        #[cfg(feature = "debug_movement")]
                        {
                            if let Some(next_movement) = character.peek_movement() {
                                let next_move_at = tick + Movement::delay(speed, next_movement.is_diagonal());
                                #[cfg(feature = "debug_movement")]
                                {
                                    let last_move_at = character.last_moved_at;
                                    info!(
                                        "move {} at {} after {}ms since last move, next move will be {} at {}",
                                        movement.position(),
                                        tick,
                                        tick - last_move_at,
                                        next_movement.position(),
                                        next_move_at
                                    );
                                    crate::util::debug::debug_in_game_chat(
                                        client_notification_sender_clone.clone(),
                                        character,
                                        format!(
                                            "move {} at {} after {}ms since last move, next move will be {} at {}",
                                            movement.position(),
                                            tick,
                                            tick - last_move_at,
                                            next_movement.position(),
                                            next_move_at
                                        ),
                                    );
                                }
                                let next_movement = character.peek_mut_movement().unwrap();
                                next_movement.set_move_at(next_move_at);
                            } else {
                                character_finished_to_move.push(character);
                            }
                        }
                        #[cfg(not(feature = "debug_movement"))]
                        {
                            if let Some(next_movement) = character.peek_mut_movement() {
                                let next_move_at = tick + Movement::delay(speed, next_movement.is_diagonal());
                                next_movement.set_move_at(next_move_at);
                            } else {
                                character_finished_to_move.push(character);
                            }
                        }
                    }
                }
            }
            for character in character_finished_to_move {
                character.transition_to_idle();
                server_ref.add_to_next_tick(GameEvent::CharacterSavePosition(CharacterSavePosition { char_id: character.char_id }));
            }
            drop(server_state_mut);
            drop(state_loops_guard);

            let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
            let sleep_duration = (MOVEMENT_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
            if sleep_duration < 5 {
                warn!(
                    "Movement loop: less than 5 milliseconds of sleep, movement loop is too slow - {}ms because movement loop took {}ms",
                    sleep_duration, time_spent
                );
            }
            sleep(Duration::from_millis(sleep_duration));
        }
    }
}
