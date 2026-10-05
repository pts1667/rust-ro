use std::borrow::Borrow;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use models::enums::EnumWithNumberValue;
use models::enums::skill_enums::SkillEnum;
use packets::packets::{Packet, PacketZcNotifyMove, PacketZcNotifyPlayermove};

use crate::server::Server;
use crate::server::model::events::client_notification::{AreaNotification, CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterRemoveItems, GameEvent};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_item::{ToMapItem, ToMapItemSnapshot};
use crate::server::model::movement::{Movable, Movement};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;

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
                match task {
                    GameEvent::GroundTrapSpend { map, unit_id } => {
                        server_ref.script_skill_service().spend_ground_unit(&map, unit_id, tick);
                        server_ref
                            .script_skill_service()
                            .refresh_ground_unit_snapshot(server_state_mut.as_mut(), &map, unit_id, tick);
                    }
                    GameEvent::GroundTrapEffect(request) => {
                        if let Err(error) = server_ref.apply_ground_trap_effect(server_state_mut.as_mut(), request, tick) {
                            warn!("Trap effect failed: {error}");
                        }
                    }
                    GameEvent::GroundTrapCapture(request) => {
                        if let Err(error) = server_ref.capture_ground_trap(server_state_mut.as_mut(), request, tick) {
                            warn!("Trap capture failed: {error}");
                        }
                    }
                    GameEvent::GroundTrapRelease(request) => {
                        if let Err(error) = server_ref.release_ground_trap(server_state_mut.as_mut(), request, tick) {
                            warn!("Trap release failed: {error}");
                        }
                    }
                    GameEvent::CharacterSelectionGate(gate) => server_ref.character_selection_gate(server_state_mut.as_mut(), gate, tick),
                    GameEvent::CharacterAdmission(admission) => {
                        server_ref.install_character_admission(server_state_mut.as_mut(), admission)
                    }
                    GameEvent::CharacterMapEntry(entry) => server_ref.install_character_map_entry(server_state_mut.as_mut(), entry),
                    GameEvent::CharacterMapReady(ready) => server_ref.prepare_character_map(server_state_mut.as_mut(), ready),
                    GameEvent::CharacterLogout(request) => server_ref.handle_character_logout(server_state_mut.as_mut(), request, tick),
                    GameEvent::ClientDisconnected(request) => server_ref.handle_client_disconnect(server_state_mut.as_mut(), request, tick),
                    GameEvent::ScriptLogoutCompleted(completion) => {
                        server_ref.complete_timer_quit_callback(server_state_mut.as_mut(), completion, tick)
                    }
                    GameEvent::ScriptLogoutAction(_) => {}
                    GameEvent::PlayerTrade(action) => {
                        if let Err(error) = server_ref.handle_player_trade(server_state_mut.as_mut(), action, tick as u64) {
                            warn!("Player trade request failed: {error}");
                        }
                    }
                    GameEvent::ReleaseScriptCapture(id) => {
                        server_state_mut.remove_locked_map_item(id);
                    }
                    GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld { char_id, request }) => {
                        if let Err(error) = server_ref.script_world_service().handle_request(
                            server_ref,
                            server_state_mut.as_mut(),
                            char_id,
                            request,
                            tick as u64,
                        ) {
                            warn!("Script world request failed: {error}");
                        }
                    }
                    GameEvent::ScriptWarp(warp) => {
                        if server_state_mut.characters().contains_key(&warp.char_id) {
                            server_ref.server_service().schedule_warp_to_walkable_cell_in_instance(
                                server_state_mut.as_mut(),
                                &warp.map,
                                warp.x,
                                warp.y,
                                warp.char_id,
                                warp.destination_instance.unwrap_or(0),
                            );
                        }
                    }
                    event @ (GameEvent::CharacterScriptSkill(_)
                    | GameEvent::ScriptCombat(_)
                    | GameEvent::MobAttack(_)
                    | GameEvent::ScriptNpcTransfer(_)
                    | GameEvent::ScriptMapDamage(_)
                    | GameEvent::ScriptNpcEvent(_)
                    | GameEvent::NpcContact(_)
                    | GameEvent::ReflectMagic(_)
                    | GameEvent::ScriptSpawned(crate::server::model::events::game_event::ScriptSpawned { .. })
                    | GameEvent::ScriptEvent(_)
                    | GameEvent::ScriptSpawn(_)
                    | GameEvent::ScriptUnitSkill(_)
                    | GameEvent::ScriptActorSkillComplete(_)
                    | GameEvent::ScriptPartyWarp(_)
                    | GameEvent::ScriptBroadcast(_)
                    | GameEvent::ScriptCraft(_)
                    | GameEvent::ScriptIdentify(_)
                    | GameEvent::ScriptTeleportSelection(_)
                    | GameEvent::WarpPortalEnter(_)
                    | GameEvent::CharacterStatusAlternatives(_)
                    | GameEvent::CharacterStatusChange(crate::server::model::events::game_event::CharacterStatusChange {
                        ..
                    })
                    | GameEvent::CharacterEndStatus(crate::server::model::events::game_event::CharacterEndStatus { .. })
                    | GameEvent::CharacterUseGroundSkill(_)
                    | GameEvent::CharacterUseGroundSkillText(_)
                    | GameEvent::ScriptSkillHit(_)
                    | GameEvent::FameChanged(_)
                    | GameEvent::TaekwonMissionKill(_)
                    | GameEvent::ItemScriptComplete(_)
                    | GameEvent::ScriptReveal(_)
                    | GameEvent::CharacterKnockback(_)) => {
                        if let Err(error) = server_ref.handle_script_event(server_state_mut.as_mut(), event, tick) {
                            warn!("Script game event failed: {error}");
                        }
                    }
                    GameEvent::ScriptRequest(request) => {
                        server_ref
                            .script_service()
                            .handle_request(server_ref, server_state_mut.as_mut(), request);
                    }
                    GameEvent::PetCaptureClaimResult(result) => {
                        if let Err(error) = server_ref.script_world_service().complete_pet_capture(
                            server_ref,
                            server_state_mut.as_mut(),
                            result,
                            tick as u64,
                        ) {
                            warn!("Pet capture completion failed: {error}");
                        }
                    }
                    GameEvent::PetLootClaimResult(result) => {
                        if let Err(error) =
                            server_ref
                                .script_world_service()
                                .complete_pet_loot(server_ref, server_state_mut.as_mut(), result, tick as u64)
                        {
                            warn!("Pet loot completion failed: {error}");
                        }
                    }
                    GameEvent::PetLootDropResult(result) => {
                        if let Err(error) = server_ref.script_world_service().complete_pet_loot_drop(
                            server_ref,
                            server_state_mut.as_mut(),
                            result,
                            tick as u64,
                        ) {
                            warn!("Pet loot delivery failed: {error}");
                        }
                    }
                    GameEvent::CharacterLeaveGame((char_id, _atype)) => {
                        server_ref.disconnect_character_in_state(server_state_mut.as_mut(), char_id);
                    }
                    GameEvent::CharacterJoinGame(char_id) => {
                        if !server_ref.bind_character_session(server_state_mut.as_mut(), char_id) {
                            continue;
                        }
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.skill_tree_service().send_skill_tree(character);
                        server_ref
                            .character_service()
                            .load_temporary_bonuses_from_db(server_ref.runtime.as_ref(), character);
                    }
                    GameEvent::CharacterChangeMap(event) => {
                        if let Err(error) = server_ref.cancel_player_trade(server_state_mut.as_mut(), event.char_id) {
                            warn!("Trade cancellation failed: {error}");
                        }
                        let map_instance = server_ref
                            .state()
                            .get_map_instance(&event.new_map_name, event.new_instance_id)
                            .unwrap_or_else(|| {
                                server_ref.server_service.create_map_instance(
                                    server_state_mut.as_mut(),
                                    GlobalConfigService::instance().get_map(&event.new_map_name),
                                    event.new_instance_id,
                                )
                            });
                        let flags = server_state_mut.map_flags(map_instance.key());
                        let character = server_state_mut.characters_mut().get_mut(&event.char_id).unwrap();
                        if let Err(error) = server_ref.script_world_service().prepare_store_map_move(
                            character,
                            map_instance.key(),
                            event.new_position.unwrap(),
                            &flags,
                        ) {
                            warn!("Map movement store update failed: {error}");
                            let origin = character.map_instance_key.clone();
                            let map_item = character.to_map_item();
                            if let Some(instance) = server_state_mut.get_map_instance(origin.map_name(), origin.map_instance()) {
                                instance.add_to_next_tick(MapEvent::InsertCharToMap(map_item));
                            }
                            continue;
                        }
                        server_ref.character_service().change_map_with_flags(
                            map_instance.key(),
                            event.new_position.unwrap(),
                            character,
                            &flags,
                        );
                        let char_map_item = character.to_map_item();
                        map_instance.add_to_next_tick(MapEvent::InsertCharToMap(char_map_item));
                        server_ref.add_to_next_tick(GameEvent::CharacterInitInventory(character.char_id));
                        let account_id = character.account_id;
                        server_state_mut.insert_map_item(account_id, char_map_item);
                    }
                    GameEvent::CharacterRemoveFromMap(character_remove_from_map) => {
                        if let Err(error) = server_ref.cancel_player_trade(server_state_mut.as_mut(), character_remove_from_map.char_id) {
                            warn!("Trade cancellation failed: {error}");
                        }
                        let character = server_state_mut
                            .characters_mut()
                            .get_mut(&character_remove_from_map.char_id)
                            .unwrap();
                        character.movements = vec![];
                        if let Some(instance) = server_ref
                            .state()
                            .get_map_instance(&character_remove_from_map.map_name, character_remove_from_map.instance_id)
                        {
                            instance.add_to_next_tick(MapEvent::RemoveCharFromMap(character_remove_from_map.char_id));
                        }
                    }
                    GameEvent::CharacterClearFov(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        character.clear_map_view();
                    }
                    GameEvent::CharacterLoadedFromClientSide(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        character.loaded_from_client_side = true;
                        character.clear_map_view();
                        server_ref.notify_map_property(server_state_mut.as_mut(), char_id);
                    }
                    GameEvent::CharacterMove(_) => {
                        // handled by dedicated thread
                    }
                    GameEvent::CharacterSavePosition(char_id) => {
                        if let Some(character) = server_state_mut.get_character(char_id) {
                            let flags = server_state_mut.map_flags(&character.map_instance_key);
                            server_ref.character_service().defer_position_update_with_flags(character, &flags);
                        }
                    }
                    GameEvent::CharacterMemo(request) => {
                        if let Err(error) = server_ref.memo_location(server_state_mut.as_mut(), request) {
                            warn!("Memo failed: {error}");
                        }
                    }
                    GameEvent::CharacterRespawn(request) => {
                        if let Err(error) = server_ref.respawn_character(server_state_mut.as_mut(), request) {
                            warn!("Respawn failed: {error}");
                        }
                    }
                    GameEvent::CharacterUpdateLook(character_look) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_look.char_id).unwrap();
                        server_ref.character_service().change_look(character_look, character)
                    }
                    GameEvent::CharacterUpdateZeny(zeny_update) => {
                        let character = server_state_mut.characters_mut().get_mut(&zeny_update.char_id).unwrap();
                        server_ref
                            .character_service()
                            .update_zeny(server_ref.runtime.as_ref(), zeny_update, character);
                    }
                    GameEvent::CharacterAddItems(add_items) => {
                        let character = server_state_mut.characters_mut().get_mut(&add_items.char_id).unwrap();
                        server_ref
                            .inventory_service()
                            .add_items_in_inventory(server_ref.runtime.as_ref(), add_items, character);
                    }
                    GameEvent::CharacterInitInventory(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref
                            .inventory_service()
                            .reload_inventory(server_ref.runtime.as_ref(), char_id, character);
                        server_ref.inventory_service().reload_equipped_item_sprites(character);
                        server_ref.refresh_forged_rank(character);
                        server_ref.refresh_taekwon_rank(character);
                        server_ref.character_service().reload_client_side_status(character);
                        server_ref.character_service().reload_client_side_hotkeys(character);
                        if let Err(error) = server_ref.script_world_service().initialize_cart(character) {
                            warn!("Cart initialization failed: {error}");
                        }
                        if let Err(error) = server_ref.script_world_service().initialize_guild(server_ref, character) {
                            warn!("Guild initialization failed: {error}");
                        }
                        if let Err(error) = server_ref.script_world_service().initialize_party(server_ref, character) {
                            warn!("Party initialization failed: {error}");
                        }
                        character.refresh_script_context();
                    }
                    GameEvent::CharacterUpdateWeight(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().notify_weight(character);
                    }
                    GameEvent::CharacterUseItem(character_use_item) => {
                        if let Some(mut character) = server_state_mut.characters_mut().remove(&character_use_item.char_id) {
                            server_ref.item_service().use_item_in_state(
                                server_ref,
                                &server_state_mut,
                                server_ref.runtime.as_ref(),
                                character_use_item,
                                &mut character,
                            );
                            server_state_mut.insert_character(character);
                        }
                    }
                    GameEvent::CharacterAttack(character_attack) => {
                        let Some(source) = server_state_mut.characters().get(&character_attack.char_id) else {
                            continue;
                        };
                        if !server_ref.player_target_allowed(
                            &server_state_mut,
                            source,
                            character_attack.target_id,
                            crate::server::service::visibility_service::TargetingMode::Direct,
                        ) {
                            continue;
                        }
                        let character = server_state_mut.characters_mut().get_mut(&character_attack.char_id).unwrap();
                        if character.is_dead()
                            || character.status.blocks_attack()
                            || character.game_systems.is_trading()
                            || character.game_systems.buying_store.is_some()
                            || character.game_systems.vending_store.is_some()
                        {
                            continue;
                        }
                        if !character.is_attacking() {
                            // last_attack_tick = 0 allows first attack to happen immediately
                            // canmove_tick is set in basic_attack() when attack animation starts
                            character.set_attack(character_attack.target_id, character_attack.repeat, 0);
                        }
                    }
                    GameEvent::CharacterEquipItem(character_equip_item) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_equip_item.char_id).unwrap();
                        if character
                            .get_item_from_inventory(character_equip_item.index)
                            .is_some_and(|item| item.item_type() == models::enums::item::ItemType::PetArmor)
                        {
                            server_ref.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                                char_id: character.char_id,
                                request: crate::server::model::game_systems::ScriptWorldRequest::EquipPetAccessory(
                                    character_equip_item.index as u16,
                                ),
                            }));
                            continue;
                        }
                        let equipped_item = server_ref.inventory_service().equip_item(character, character_equip_item);
                        equipped_item.map(|item| {
                            server_ref
                                .inventory_service()
                                .sprite_change_packet_for_item(character, &item, false)
                                .map(|packet| {
                                    server_ref
                                        .character_service()
                                        .send_area_notification_around_characters(character, packet)
                                })
                        });
                    }
                    GameEvent::CharacterTakeoffEquipItem(character_takeoff_equip_item) => {
                        let character = server_state_mut
                            .characters_mut()
                            .get_mut(&character_takeoff_equip_item.char_id)
                            .unwrap();
                        let index = character_takeoff_equip_item.index;
                        server_ref.inventory_service().takeoff_equip_item(character, index).map(|item| {
                            server_ref
                                .inventory_service()
                                .sprite_change_packet_for_item(character, &item, true)
                                .map(|packet| {
                                    server_ref
                                        .character_service()
                                        .send_area_notification_around_characters(character, packet)
                                })
                        });
                    }
                    GameEvent::CharacterUpdateClientSideStats(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.refresh_forged_rank(character);
                        server_ref.refresh_taekwon_rank(character);
                        server_ref.character_service().reload_client_side_status(character);
                    }
                    GameEvent::CharacterChangeLevel(character_change_level) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_change_level.char_id).unwrap();
                        let delta = server_ref.character_service().update_base_level(
                            character,
                            character_change_level.set_level,
                            character_change_level.add_level,
                        );
                        if delta < 0 {
                            // TODO ensure equip required min level
                        }
                    }
                    GameEvent::CharacterChangeJobLevel(character_change_level) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_change_level.char_id).unwrap();
                        let delta = server_ref.character_service().update_job_level(
                            character,
                            character_change_level.set_level,
                            character_change_level.add_level,
                        );
                        if delta < 0 {
                            // TODO ensure equip required min level
                        }
                    }
                    GameEvent::CharacterChangeJob(character_change_job) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_change_job.char_id).unwrap();
                        match server_ref.repository.character_change_fame_class(
                            character.char_id,
                            character.account_id,
                            character_change_job.job.value() as u32,
                        ) {
                            Ok(updates) => {
                                for update in updates {
                                    server_ref.add_to_next_tick(GameEvent::FameChanged(
                                        crate::server::model::events::game_event::FameChanged {
                                            category: update.category,
                                            ranked_creators: update.rankings.iter().map(|entry| entry.char_id).collect(),
                                        },
                                    ));
                                }
                            }
                            Err(error) => {
                                warn!("Job transaction failed: {error}");
                                continue;
                            }
                        }
                        server_ref.character_service().change_job(
                            character,
                            character_change_job.job,
                            character_change_job.should_reset_skills,
                        );
                        server_ref.refresh_taekwon_rank(character);
                        server_ref.character_service().reload_client_side_status(character);
                        // TODO ensure equip required class
                    }
                    GameEvent::CharacterKillMonster(kill) => {
                        if let Err(error) = server_ref.reward_monster_kill(server_state_mut.as_mut(), kill, tick) {
                            warn!("Monster reward failed: {error}");
                        }
                    }
                    GameEvent::CharacterPickUpItem(character_pickup_item) => {
                        if let Some(mut character) = server_state_mut.characters_mut().remove(&character_pickup_item.char_id) {
                            if let Some(map_instance) = server_state_mut.get_map_instance_from_character(&character) {
                                if let Err(error) = server_ref.server_service.character_pickup_item(
                                    server_ref,
                                    server_state_mut.as_mut(),
                                    &mut character,
                                    character_pickup_item.map_item_id,
                                    map_instance.as_ref(),
                                ) {
                                    warn!("Floor item transfer failed: {error}");
                                }
                            }
                            server_state_mut.insert_character(character);
                        }
                    }
                    GameEvent::MapNotifyItemRemoved(map_item_id) => {
                        server_state_mut.remove_locked_map_item(map_item_id);
                    }
                    GameEvent::CharacterUpdateStat(character_update_stat) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_update_stat.char_id).unwrap();
                        server_ref
                            .character_service()
                            .character_increase_stat(character, character_update_stat);
                    }
                    GameEvent::CharacterSkillUpgrade(character_skill_upgrade) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_skill_upgrade.char_id).unwrap();
                        if (8001..=8016).contains(&(character_skill_upgrade.skill_id as u32)) {
                            if let Err(error) = server_ref
                                .script_world_service()
                                .learn_homunculus_skill(character, character_skill_upgrade.skill_id as u32)
                            {
                                warn!("Homunculus skill learning failed: {error}");
                            }
                            continue;
                        }
                        if (10000..=10015).contains(&u32::from(character_skill_upgrade.skill_id)) {
                            server_ref.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                                char_id: character.char_id,
                                request: crate::server::model::game_systems::ScriptWorldRequest::GuildSkillUp(u32::from(character_skill_upgrade.skill_id)),
                            }));
                            continue;
                        }
                        server_ref
                            .character_service()
                            .allocate_skill_point(character, SkillEnum::from_id(character_skill_upgrade.skill_id as u32));
                    }
                    GameEvent::CharacterDropItem(character_drop_item) => {
                        if let Err(error) =
                            server_ref
                                .server_service()
                                .character_drop_item(server_ref, server_state_mut.as_mut(), character_drop_item)
                        {
                            warn!("Item drop failed: {error}");
                        }
                    }
                    GameEvent::CharacterSellItems(character_remove_items) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_remove_items.char_id).unwrap();
                        server_ref
                            .inventory_service()
                            .character_sell_items(server_ref.runtime.as_ref(), character, character_remove_items);
                    }
                    GameEvent::CharacterResetSkills(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().reset_skills(character, true);
                    }
                    GameEvent::CharacterResetStats(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().reset_stats(character);
                    }
                    GameEvent::CharacterUseSkill(character_use_skill) => {
                        if let Err(error) = server_ref.handle_character_skill(server_state_mut.as_mut(), character_use_skill, tick) {
                            warn!("Skill use failed: {error}");
                        }
                    }
                    GameEvent::CharacterDamage(damage) => {
                        if let Err(error) = server_ref.admit_character_damage(server_state_mut.as_mut(), damage, tick) {
                            warn!("Character damage failed: {error}");
                        }
                    }
                    GameEvent::CharacterUpdateSpeed(char_id, speed) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        character.status.set_speed(speed);
                        server_ref.character_service().reload_client_side_status(character);
                    }
                    GameEvent::CharacterHotkeyAdd(char_id, hotkey) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().hotkey_add(character, hotkey);
                    }
                    GameEvent::CharacterHotkeyRemove(char_id, hotkey_index) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().hotkey_remove(character, hotkey_index);
                    }
                    GameEvent::Duel(command) => {
                        server_ref.handle_duel_command(server_state_mut.as_mut(), command);
                    }
                    GameEvent::CharacterRestoreAllHpAndSP(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        let status = StatusService::instance().to_snapshot(&character.status);
                        server_ref
                            .character_service()
                            .update_hp_sp(character, status.max_hp(), status.max_sp());
                    }
                    GameEvent::CharacterSit(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().sit(character);
                    }
                    GameEvent::CharacterStand(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().stand(character);
                    }
                    GameEvent::CharacterCancelMove(char_id) => {
                        let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
                        server_ref.character_service().cancel_movement(character, tick);
                    }
                    GameEvent::CharacterRequestCardCompositionList(character_request_card_composition_list) => {
                        let character = server_state_mut
                            .characters_mut()
                            .get_mut(&character_request_card_composition_list.char_id)
                            .unwrap();
                        server_ref
                            .inventory_service()
                            .send_card_composition_list(character, character_request_card_composition_list);
                    }
                    GameEvent::CharacterSlotCard(character_slot_card) => {
                        let character = server_state_mut.characters_mut().get_mut(&character_slot_card.char_id).unwrap();
                        server_ref
                            .inventory_service()
                            .slot_card(server_ref.runtime(), character, character_slot_card);
                    }
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
                instance.add_to_next_tick(MapEvent::UpdateActorVisibility(
                    server_state_mut
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
                        .collect(),
                ));
                instance.add_to_next_tick(MapEvent::UpdateMobsFov(
                    server_state_mut
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
                        .collect(),
                ));
            }
        }
    }

    pub(crate) fn character_movement_loop(server_ref: Arc<Server>, client_notification_sender_clone: SyncSender<Notification>) {
        loop {
            if !server_ref.is_alive() {
                break;
            }
            let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
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
                server_ref.add_to_next_tick(GameEvent::CharacterSavePosition(character.char_id));
            }

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
