#![allow(dead_code)]
use std::sync::Arc;

use models::enums::EnumWithMaskValueU32;

use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::tests::common;
use crate::tests::common::sync_helper::CountDownLatch;
use crate::tests::common::{TestContext, create_mpsc};

struct MapInstanceServiceTestContext {
    test_context: TestContext,
    map_instance_service: MapInstanceService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

fn before_each() -> MapInstanceServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> MapInstanceServiceTestContext {
    common::before_all();
    crate::server::service::status_service::StatusService::init(GlobalConfigService::instance(), common::test_script_vm());
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let mob_service = MobService::new(client_notification_sender.clone(), GlobalConfigService::instance());
    let server_task_queue = Arc::new(TasksQueue::new());
    let count_down_latch = CountDownLatch::new(latch_size);
    MapInstanceServiceTestContext {
        test_context: TestContext::new(
            client_notification_sender.clone(),
            client_notification_receiver,
            persistence_event_sender,
            persistence_event_receiver,
            count_down_latch,
        ),
        server_task_queue: server_task_queue.clone(),
        map_instance_service: MapInstanceService::new(
            client_notification_sender,
            GlobalConfigService::instance(),
            mob_service,
            server_task_queue,
        ),
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::collections::HashMap;
    use std::mem;
    use std::sync::Arc;
    use std::time::Duration;

    use models::enums::EnumWithMaskValueU32;
    use models::item::DroppedItem;
    use movement::position::Position;
    use packets::packets::PacketZcItemDisappear;

    use crate::server::map_instance_loop::MAP_LOOP_TICK_RATE;
    use crate::server::model::action::Damage;
    use crate::server::model::events::game_event::{CharacterKillMonster, GameEvent, MapNotifyItemRemoved, ReleaseScriptCapture};
    use crate::server::model::events::map_event::{MapEvent, MobDropItems, MobLocation};
    use crate::server::model::map_item::{MapItem, MapItemType};
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::common::assert_helper::{
        NotificationExpectation, SentPacket, has_sent_notification, task_queue_contains_event_at_tick,
    };
    use crate::tests::common::map_instance_helper::create_empty_map_instance_state;
    use crate::tests::common::mob_helper::create_mob;
    use crate::tests::map_instance_service_test::before_each;
    use crate::util::tick::{delayed_tick, get_tick};
    use crate::{assert_eq_with_variance, assert_sent_packet_in_current_packetver, assert_task_queue_contains_event_at_tick};

    #[test]
    fn test_mob_die_should_remove_map_item_from_map_instance_and_mob() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let mut mob = create_mob(mob_item_id, "PORING");
        mob.status.set_hp(0);
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_die(&mut map_instance_state, mob_item_id, 0);
        assert_eq!(map_instance_state.get_mob(mob_item_id).unwrap().is_present(), false);
        context.map_instance_service.remove_dead_mobs(&mut map_instance_state);
        // Then
        assert_eq!(
            mem::discriminant(&map_instance_state.get_mob(mob_item_id)),
            mem::discriminant(&None)
        );
        assert_eq!(
            mem::discriminant(&map_instance_state.get_map_item(mob_item_id)),
            mem::discriminant(&None)
        );
    }

    #[test]
    fn test_mob_die_should_emit_an_event_with_attacker_with_higher_damage_and_mob_id() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let mut mob = create_mob(mob_item_id, "PORING");
        let mob_id = mob.mob_id;
        let x = mob.x;
        let y = mob.y;
        let mob_model = GlobalConfigService::instance().get_mob_by_name("PORING");
        mob.add_attack(150000, 20);
        mob.add_attack(150001, 40);
        mob.add_attack(150000, 30);
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_die(&mut map_instance_state, mob_item_id, 0);
        // Then
        task_queue_contains_event_at_tick(
            context.server_task_queue.clone(),
            GameEvent::CharacterKillMonster(CharacterKillMonster {
                char_id: 150000,
                attacker_id: 0,
                mob_id,
                mob_x: x,
                mob_y: y,
                map_instance_key: map_instance_state.key().clone(),
                mob_base_exp: mob_model.exp as u32,
                mob_job_exp: mob_model.job_exp as u32,
                mob_max_hp: mob_model.hp as u32,
                contributions: vec![],
            }),
            0,
        );
    }

    #[test]
    fn test_mob_drop_item_when_mob_is_a_normal_monster() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let poring = GlobalConfigService::instance().get_mob_by_name("PORING");
        let mob_drop_items = MobDropItems {
            owner_id: 150000,
            mob_id: poring.id as i16,
            mob_x: 10,
            mob_y: 10,
        };
        let mut expected_dropped_item_amount: HashMap<u32, (String, u32)> = Default::default();
        poring.drops.iter().for_each(|drop_config| {
            // Sometime mob like "PORING" drop an item twice (like "apple") with different
            // drop rates
            let item_drop_amount_expectation = expected_dropped_item_amount
                .entry(GlobalConfigService::instance().get_item_id_from_name(drop_config.item_name.as_str()))
                .or_insert((drop_config.item_name.clone(), 0));
            item_drop_amount_expectation.1 += drop_config.rate as u32; // we use drop rate from configuration as we will iterate 10000 times, drop rate == expected amount of item dropped after 10000 mob killed
        });

        let iterations = 1100; // we need at least 1k iteration to have good statistics
        // When
        let mut drops_per_item: HashMap<u32, u32> = HashMap::new();
        for _ in 0..=iterations {
            for _ in 0..10000 {
                // killing 10k poring
                let drops = context.map_instance_service.mob_drop_items(&mut map_instance_state, mob_drop_items);
                for drop in drops {
                    let item_drop_count = drops_per_item.entry(drop.item_id as u32).or_insert(0);
                    *item_drop_count += 1;
                }
            }
        }
        // Then
        for (item_id, (item_name, expected_drop_amount)) in expected_dropped_item_amount {
            let total_amount_dropped = drops_per_item.get(&item_id).unwrap();
            let average = (*total_amount_dropped as f32 / iterations as f32).round() as u32;
            assert_eq_with_variance!(
                2,
                average,
                expected_drop_amount,
                "Expected item {} to be dropped {} times but was dropped {} times",
                item_name,
                expected_drop_amount,
                average
            );
        }
    }

    #[test]
    fn test_mob_drop_items_should_add_item_to_map_items() {
        // Given
        let context = before_each();
        let poring = GlobalConfigService::instance().get_mob_by_name("PORING");
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_drop_items = MobDropItems {
            owner_id: 150000,
            mob_id: poring.id as i16,
            mob_x: 10,
            mob_y: 10,
        };
        // When
        let drops = context.map_instance_service.mob_drop_items(&mut map_instance_state, mob_drop_items);
        // Then
        for drop in drops {
            assert!(map_instance_state.get_map_item(drop.map_item_id).is_some());
            assert!(matches!(
                map_instance_state.get_map_item(drop.map_item_id).unwrap().object_type(),
                MapItemType::DroppedItem
            ));
            assert!(map_instance_state.get_dropped_item(drop.map_item_id).is_some());
        }
    }

    #[test]
    fn test_mob_being_attacked_should_damage_mob() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let map_instance_tasks_queue = Arc::new(<TasksQueue<MapEvent>>::new());
        let mob = create_mob(mob_item_id, "PORING");
        let max_hp = mob.status.max_hp();
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_being_attacked(
            &mut map_instance_state,
            Damage { notification: None,
                source_kind: models::enums::actor::CombatActorKind::Player,
                skill_damage_adjusted: false,
                target_id: mob_item_id,
                attacker_id: 150000,
                damage: 10,
                healing: 0,
                right_hand_damage: None,
                attacked_at: get_tick(),
                damage_motion: 480,
                battle_flags: models::status_bonus::BattleFlag::Weapon.as_flag()
                    | models::status_bonus::BattleFlag::Short.as_flag()
                    | models::status_bonus::BattleFlag::Normal.as_flag(),
                skill_id: 0,
                skill_level: 0,
                landed: true,
                proc_depth: 0,
                credit_id: 150000,
                defenses_applied: true,
                magic_context: None,
            },
            map_instance_tasks_queue,
            get_tick(),
        );
        // Then
        assert_eq!(map_instance_state.get_mob(mob_item_id).unwrap().hp(), max_hp - 10);
    }

    #[test]
    fn test_mob_being_attacked_should_trigger_mob_die_when_mob_has_get_more_damage_than_its_hp() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let map_instance_tasks_queue = Arc::new(<TasksQueue<MapEvent>>::new());
        let mob = create_mob(mob_item_id, "PORING");
        let max_hp = mob.status.max_hp();
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_being_attacked(
            &mut map_instance_state,
            Damage { notification: None,
                source_kind: models::enums::actor::CombatActorKind::Player,
                skill_damage_adjusted: false,
                target_id: mob_item_id,
                attacker_id: 150000,
                damage: max_hp + 10,
                healing: 0,
                right_hand_damage: None,
                attacked_at: get_tick(),
                damage_motion: 480,
                battle_flags: models::status_bonus::BattleFlag::Weapon.as_flag()
                    | models::status_bonus::BattleFlag::Short.as_flag()
                    | models::status_bonus::BattleFlag::Normal.as_flag(),
                skill_id: 0,
                skill_level: 0,
                landed: true,
                proc_depth: 0,
                credit_id: 150000,
                defenses_applied: true,
                magic_context: None,
            },
            map_instance_tasks_queue,
            get_tick(),
        );
        context.map_instance_service.remove_dead_mobs(&mut map_instance_state);
        // Then
        assert!(map_instance_state.get_mob(mob_item_id).is_none());
    }

    #[test]
    fn test_mob_being_attacked_should_delay_mob_vanish_client_side_notification() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let map_instance_tasks_queue = Arc::new(<TasksQueue<MapEvent>>::new());
        let mob = create_mob(mob_item_id, "PORING");
        let original_mob = mob.clone();
        let max_hp = mob.status.max_hp();
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_being_attacked(
            &mut map_instance_state,
            Damage { notification: None,
                source_kind: models::enums::actor::CombatActorKind::Player,
                skill_damage_adjusted: false,
                target_id: mob_item_id,
                attacker_id: 150000,
                damage: max_hp + 10,
                healing: 0,
                right_hand_damage: None,
                attacked_at: get_tick() + 10,
                damage_motion: 480,
                battle_flags: models::status_bonus::BattleFlag::Weapon.as_flag()
                    | models::status_bonus::BattleFlag::Short.as_flag()
                    | models::status_bonus::BattleFlag::Normal.as_flag(),
                skill_id: 0,
                skill_level: 0,
                landed: true,
                proc_depth: 0,
                credit_id: 150000,
                defenses_applied: true,
                magic_context: None,
            },
            map_instance_tasks_queue.clone(),
            get_tick(),
        );
        // Then
        assert_task_queue_contains_event_at_tick!(
            map_instance_tasks_queue,
            MapEvent::MobDeathClientNotification(MobLocation {
                mob_id: original_mob.id,
                x: original_mob.x,
                y: original_mob.y
            }),
            delayed_tick(10, MAP_LOOP_TICK_RATE)
        );
    }

    #[test]
    fn test_remove_dropped_item_from_map_should_trigger_server_map_item_remove() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let clover = DroppedItem {
            attributes: Default::default(),
            player_dropped: false,
            map_item_id: 1000,
            item_id: GlobalConfigService::instance().get_item_id_from_name("Clover") as i32,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        };
        map_instance_state.insert_dropped_item(clover);
        // When
        context
            .map_instance_service
            .remove_dropped_item_from_map(&mut map_instance_state, clover.map_item_id);
        // Then
        context
            .test_context
            .increment_latch()
            .wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(
            context,
            NotificationExpectation::of_fov(50, 50, vec![SentPacket::with_id(PacketZcItemDisappear::packet_id(
                GlobalConfigService::instance().packetver()
            ))])
        );
        assert_task_queue_contains_event_at_tick!(
            context.server_task_queue.clone(),
            GameEvent::MapNotifyItemRemoved(MapNotifyItemRemoved { map_item_id: clover.map_item_id }),
            0
        );
    }

    fn admitted_damage(target_id: u32, damage: u32, landed: bool) -> Damage {
        Damage { notification: None,
            source_kind: models::enums::actor::CombatActorKind::Player,
            skill_damage_adjusted: false,
            target_id,
            attacker_id: 150000,
            damage,
            healing: 0,
            right_hand_damage: None,
            attacked_at: 0,
            damage_motion: 0,
            battle_flags: models::status_bonus::BattleFlag::Weapon.as_flag()
                | models::status_bonus::BattleFlag::Short.as_flag()
                | models::status_bonus::BattleFlag::Normal.as_flag(),
            skill_id: 0,
            skill_level: 0,
            landed,
            proc_depth: 0,
            credit_id: 150000,
            defenses_applied: true,
            magic_context: None,
        }
    }

    fn pet_loot_floor_item() -> DroppedItem {
        DroppedItem {
            map_item_id: 7000,
            item_id: 1101,
            location: Position { x: 10, y: 10, dir: 0 },
            sub_location: Position { x: 3, y: 6, dir: 0 },
            owner_id: Some(150000),
            dropped_at: 12,
            amount: 1,
            is_identified: false,
            player_dropped: true,
            attributes: models::item::ItemInstanceAttributes {
                unique_id: 999,
                refine: 7,
                damaged: true,
                cards: [4001, 4002, 4003, 4004],
            },
        }
    }

    fn pet_loot_state() -> crate::server::state::map_instance::MapInstanceState {
        let mut state = create_empty_map_instance_state();
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        state.update_characters(vec![crate::server::model::map_item::MapItemSnapshot::new(
            MapItem::new(
                crate::server::service::script_world_service::pet_world_id(9),
                1113,
                MapItemType::Pet,
            ),
            Position { x: 0, y: 0, dir: 0 },
        )]);
        state.insert_dropped_item(pet_loot_floor_item());
        state
    }

    fn pet_loot_claim(claim_id: u64) -> crate::server::model::events::map_event::PetLootClaimRequest {
        crate::server::model::events::map_event::PetLootClaimRequest {
            claim_id,
            char_id: 150000,
            pet_id: 9,
            target_id: 7000,
            x: 10,
            y: 10,
            expires_at: 100,
        }
    }

    #[test]
    fn pet_loot_claim_blocks_queued_removal_and_restores_exact_metadata_on_rollback() {
        use crate::server::model::events::map_event::PetLootFinalize;
        let context = before_each();
        let mut state = pet_loot_state();
        assert!(
            state
                .get_map_item(crate::server::service::script_world_service::pet_world_id(9))
                .is_none()
        );
        context.map_instance_service.claim_pet_loot(&mut state, pet_loot_claim(10), 1);
        context.map_instance_service.claim_pet_loot(&mut state, pet_loot_claim(10), 2);
        assert_eq!(state.pending_pet_loot.len(), 1);
        assert!(state.get_dropped_item(7000).is_none());
        assert!(state.get_map_item(7000).is_some());
        let claims = context.server_task_queue.pop().unwrap();
        assert_eq!(claims.len(), 2);
        assert!(
            claims
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootClaimResult(result) if result.item == Some(pet_loot_floor_item())))
        );
        context.map_instance_service.remove_dropped_item_from_map(&mut state, 7000);
        assert!(context.server_task_queue.pop().unwrap_or_default().is_empty());
        assert!(state.get_map_item(7000).is_some());
        context.map_instance_service.finalize_pet_loot(&mut state, PetLootFinalize {
            claim_id: 11,
            target_id: 7000,
            commit: false,
        });
        assert!(context.server_task_queue.pop().unwrap_or_default().is_empty());
        assert_eq!(state.pending_pet_loot.len(), 1);
        context.map_instance_service.finalize_pet_loot(&mut state, PetLootFinalize {
            claim_id: 10,
            target_id: 7000,
            commit: false,
        });
        assert_eq!(state.get_dropped_item(7000), Some(&pet_loot_floor_item()));
        assert!(state.get_map_item(7000).is_some());
        assert!(state.pending_pet_loot.is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .contains(&GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: 7000 }))
        );
        context.map_instance_service.finalize_pet_loot(&mut state, PetLootFinalize {
            claim_id: 10,
            target_id: 7000,
            commit: true,
        });
        assert_eq!(state.get_dropped_item(7000), Some(&pet_loot_floor_item()));
    }

    #[test]
    fn pet_loot_commit_removes_one_floor_item_and_duplicate_acknowledgements_do_not_reopen_it() {
        use crate::server::model::events::map_event::PetLootFinalize;
        let context = before_each();
        let mut state = pet_loot_state();
        context.map_instance_service.claim_pet_loot(&mut state, pet_loot_claim(20), 1);
        context.server_task_queue.pop();
        context.map_instance_service.finalize_pet_loot(&mut state, PetLootFinalize {
            claim_id: 20,
            target_id: 7000,
            commit: true,
        });
        context.map_instance_service.finalize_pet_loot(&mut state, PetLootFinalize {
            claim_id: 20,
            target_id: 7000,
            commit: false,
        });
        context.map_instance_service.claim_pet_loot(&mut state, pet_loot_claim(20), 3);
        assert!(state.get_dropped_item(7000).is_none());
        assert!(state.get_map_item(7000).is_none());
        assert!(state.pending_pet_loot.is_empty());
        let events = context.server_task_queue.pop().unwrap();
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::MapNotifyItemRemoved(MapNotifyItemRemoved { map_item_id: 7000 })))
                .count(),
            1
        );
        assert!(
            events
                .into_iter()
                .any(|event| matches!(event, GameEvent::PetLootClaimResult(result) if result.item == Some(pet_loot_floor_item())))
        );
        context
            .test_context
            .increment_latch()
            .wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_eq!(context.test_context.received_notification().lock().unwrap().len(), 1);
    }

    #[test]
    fn pet_loot_rejects_expired_distant_foreign_or_missing_floor_items_and_releases_failed_claim_locks() {
        use crate::server::model::events::map_event::PetLootFinalize;
        for case in 0..5 {
            let context = before_each();
            let mut state = pet_loot_state();
            let mut request = pet_loot_claim(30);
            match case {
                0 => request.expires_at = 0,
                1 => request.x = 13,
                2 => {
                    let mut item = state.remove_dropped_item(7000).unwrap();
                    item.owner_id = Some(150001);
                    state.insert_dropped_item(item);
                }
                3 => state.update_characters(vec![]),
                _ => {
                    state.remove_dropped_item(7000);
                }
            }
            context.map_instance_service.claim_pet_loot(&mut state, request, 1);
            assert!(state.pending_pet_loot.is_empty());
            assert!(
                context
                    .server_task_queue
                    .pop()
                    .unwrap()
                    .into_iter()
                    .all(|event| matches!(event, GameEvent::PetLootClaimResult(result) if result.item.is_none()))
            );
            context.map_instance_service.finalize_pet_loot(&mut state, PetLootFinalize {
                claim_id: 30,
                target_id: 7000,
                commit: false,
            });
            assert!(
                context
                    .server_task_queue
                    .pop()
                    .unwrap()
                    .contains(&GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: 7000 }))
            );
        }
    }

    fn pet_loot_drop(claim_id: u64) -> crate::server::model::events::map_event::PetLootDropRequest {
        crate::server::model::events::map_event::PetLootDropRequest {
            claim_id,
            char_id: 150000,
            x: 10,
            y: 10,
            items: vec![
                database::model::InventoryRecord {
                    item_id: 1101,
                    amount: 1,
                    unique_id: 999,
                    refine: 7,
                    is_damaged: true,
                    card0: 4001,
                    card1: 4002,
                    card2: 4003,
                    card3: 4004,
                    ..Default::default()
                },
                database::model::InventoryRecord {
                    item_id: 501,
                    amount: 2,
                    is_identified: true,
                    ..Default::default()
                },
            ],
        }
    }

    #[test]
    fn pet_loot_overflow_is_invisible_until_commit_and_duplicate_receipts_preserve_item_identity() {
        use crate::server::model::events::map_event::PetLootDropFinalize;
        use crate::server::model::map_flags::MapFlag;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let request = pet_loot_drop(40);
        context.map_instance_service.prepare_pet_loot_drop(&mut state, request.clone());
        context.map_instance_service.prepare_pet_loot_drop(&mut state, request.clone());
        let staged = state.pending_pet_loot_drops[&40].items.clone();
        assert_eq!(state.pending_pet_loot_drops.len(), 1);
        assert!(state.map_items().is_empty());
        assert!(staged.iter().all(|item| state.get_dropped_item(item.map_item_id).is_none()));
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootDropResult(result) if result.accepted))
        );
        state.flags.set(MapFlag::NoDrop, true, &[]).unwrap();
        context
            .map_instance_service
            .finalize_pet_loot_drop(&mut state, PetLootDropFinalize {
                claim_id: 40,
                commit: true,
            });
        assert_eq!(state.map_items().len(), 2);
        assert_eq!(state.get_dropped_item(staged[0].map_item_id), Some(&staged[0]));
        assert_eq!(staged[0].attributes, pet_loot_floor_item().attributes);
        assert_eq!((staged[1].item_id, staged[1].amount, staged[1].is_identified), (501, 2, true));
        assert!(staged.iter().all(|item| item.owner_id.is_none() && !item.player_dropped));
        context
            .map_instance_service
            .finalize_pet_loot_drop(&mut state, PetLootDropFinalize {
                claim_id: 40,
                commit: false,
            });
        context.map_instance_service.prepare_pet_loot_drop(&mut state, request);
        assert_eq!(state.map_items().len(), 2);
        assert!(state.pending_pet_loot_drops.is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootDropResult(result) if result.accepted))
        );
        context
            .test_context
            .increment_latch()
            .wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_eq!(context.test_context.received_notification().lock().unwrap().len(), 1);
    }

    #[test]
    fn pet_loot_overflow_rejects_no_drop_and_invalid_items_and_rollback_never_publishes_cargo() {
        use crate::server::model::events::map_event::PetLootDropFinalize;
        use crate::server::model::map_flags::MapFlag;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        state.flags.set(MapFlag::NoDrop, true, &[]).unwrap();
        context.map_instance_service.prepare_pet_loot_drop(&mut state, pet_loot_drop(50));
        assert!(state.pending_pet_loot_drops.is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootDropResult(result) if !result.accepted))
        );
        state.flags.set(MapFlag::NoDrop, false, &[]).unwrap();
        let mut invalid = pet_loot_drop(51);
        invalid.items[1].item_id = 0;
        context.map_instance_service.prepare_pet_loot_drop(&mut state, invalid);
        assert!(state.pending_pet_loot_drops.is_empty());
        assert!(state.map_items().is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootDropResult(result) if !result.accepted))
        );
        context.map_instance_service.prepare_pet_loot_drop(&mut state, pet_loot_drop(52));
        let ids = state.pending_pet_loot_drops[&52]
            .items
            .iter()
            .map(|item| item.map_item_id)
            .collect::<Vec<_>>();
        context.server_task_queue.pop();
        for id in &ids {
            context.map_instance_service.remove_dropped_item_from_map(&mut state, *id);
        }
        assert!(context.server_task_queue.pop().unwrap_or_default().is_empty());
        context
            .map_instance_service
            .finalize_pet_loot_drop(&mut state, PetLootDropFinalize {
                claim_id: 52,
                commit: false,
            });
        context
            .map_instance_service
            .finalize_pet_loot_drop(&mut state, PetLootDropFinalize {
                claim_id: 52,
                commit: true,
            });
        context.map_instance_service.prepare_pet_loot_drop(&mut state, pet_loot_drop(52));
        assert!(state.pending_pet_loot_drops.is_empty());
        assert!(state.map_items().is_empty());
        assert!(ids.into_iter().all(|id| state.get_dropped_item(id).is_none()));
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootDropResult(result) if !result.accepted))
        );
    }

    #[test]
    fn pet_loot_overflow_rejects_a_map_without_any_walkable_placement() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        state.set_cells(vec![0; 100 * 100 + 1]);
        context.map_instance_service.prepare_pet_loot_drop(&mut state, pet_loot_drop(60));
        assert!(state.pending_pet_loot_drops.is_empty());
        assert!(state.map_items().is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetLootDropResult(result) if !result.accepted))
        );
    }

    #[test]
    fn pet_capture_claim_preserves_the_actor_until_commit_and_restores_it_on_rollback() {
        use crate::server::model::events::map_event::{PetCaptureClaimRequest, PetCaptureFinalize};
        use crate::server::state::map_instance::MobSpawnTrack;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        mob.set_hp(20);
        mob.dir = 7;
        mob.timing.set_canmove_tick(123);
        mob.add_attack(150000, 10);
        let pet = crate::server::service::script_world_service::world_data()
            .pets
            .iter()
            .find(|pet| pet.class_id == mob.mob_id as u16)
            .unwrap();
        let request = PetCaptureClaimRequest {
            claim_id: 1,
            char_id: 150000,
            target_id: 42,
            x: mob.x,
            y: mob.y,
            lure_item_id: pet.tame_item,
            flag: 0,
            expires_at: 1000,
        };
        let original_status = mob.status.clone();
        state.insert_mob(mob);
        let mut track = MobSpawnTrack::default(0);
        track.increment_spawn();
        state.mob_spawns_tracks_mut().insert(0, track);
        context
            .map_instance_service
            .claim_pet_capture_with_roll(&mut state, request.clone(), 1, 0);
        assert!(state.get_mob(42).is_none());
        assert!(state.get_map_item(42).is_some());
        assert_eq!(state.mob_spawns_tracks()[&0].spawned_amount, 1);
        assert_eq!(state.pending_pet_captures[&1].mob.status, original_status);
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .any(|event| matches!(event, GameEvent::PetCaptureClaimResult(result) if result.class_id == Some(pet.class_id)))
        );
        context
            .map_instance_service
            .mob_being_attacked(&mut state, admitted_damage(42, 10000, true), Arc::new(TasksQueue::new()), 2);
        assert_eq!(state.pending_pet_captures[&1].mob.hp(), 10);
        assert!(context.server_task_queue.pop().unwrap_or_default().is_empty());
        context.map_instance_service.finalize_pet_capture(&mut state, PetCaptureFinalize {
            claim_id: 99,
            target_id: 42,
            commit: true,
        });
        assert!(state.pending_pet_captures.contains_key(&1));
        context.map_instance_service.finalize_pet_capture(&mut state, PetCaptureFinalize {
            claim_id: 1,
            target_id: 42,
            commit: false,
        });
        let restored = state.get_mob(42).unwrap();
        assert_eq!(restored.status, original_status);
        assert_eq!(restored.dir, 7);
        assert_eq!(restored.timing.get_canmove_tick(), 123);
        assert_eq!(restored.attacker_with_higher_damage(), 150000);
        assert_eq!(state.mob_spawns_tracks()[&0].spawned_amount, 1);
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: 42 })))
        );
        let committed = PetCaptureClaimRequest { claim_id: 2, ..request };
        context
            .map_instance_service
            .claim_pet_capture_with_roll(&mut state, committed, 3, 0);
        context.server_task_queue.pop();
        let finalize = PetCaptureFinalize {
            claim_id: 2,
            target_id: 42,
            commit: true,
        };
        context.map_instance_service.finalize_pet_capture(&mut state, finalize.clone());
        context.map_instance_service.finalize_pet_capture(&mut state, finalize);
        assert!(state.get_mob(42).is_none());
        assert!(state.get_map_item(42).is_none());
        assert!(state.pending_pet_captures.is_empty());
        assert_eq!(state.mob_spawns_tracks()[&0].spawned_amount, 0);
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: 42 })))
        );
    }

    #[test]
    fn pet_capture_rejects_a_monster_killed_before_the_claim() {
        use crate::server::model::events::map_event::PetCaptureClaimRequest;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mob = create_mob(42, "PORING");
        let pet = crate::server::service::script_world_service::world_data()
            .pets
            .iter()
            .find(|pet| pet.class_id == mob.mob_id as u16)
            .unwrap();
        let request = PetCaptureClaimRequest {
            claim_id: 1,
            char_id: 150000,
            target_id: 42,
            x: mob.x,
            y: mob.y,
            lure_item_id: pet.tame_item,
            flag: 0,
            expires_at: 1000,
        };
        state.insert_mob(mob);
        context
            .map_instance_service
            .mob_being_attacked(&mut state, admitted_damage(42, 10000, true), Arc::new(TasksQueue::new()), 1);
        context.server_task_queue.pop();
        context.map_instance_service.claim_pet_capture_with_roll(&mut state, request, 2, 0);
        assert!(state.pending_pet_captures.is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetCaptureClaimResult(result) if result.class_id.is_none()))
        );
        assert_eq!(state.get_mob(42).unwrap().hp(), 0);
        context
            .map_instance_service
            .finalize_pet_capture(&mut state, crate::server::model::events::map_event::PetCaptureFinalize {
                claim_id: 1,
                target_id: 42,
                commit: false,
            });
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::ReleaseScriptCapture(ReleaseScriptCapture { id: 42 })))
        );
    }

    #[test]
    fn monster_dispel_preserves_assumptio_and_does_not_trigger_berserk_hp_loss() {
        use models::status_change::{StatusChangeKind, StatusChangeRequest};
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mob = create_mob(42, "PORING");
        let base_hp = mob.hp();
        state.insert_mob(mob);
        for kind in [StatusChangeKind::Berserk, StatusChangeKind::Assumptio, StatusChangeKind::Blessing] {
            context
                .map_instance_service
                .start_mob_status(&mut state, 42, StatusChangeRequest::guaranteed(kind, 5000, 1), 0);
        }
        state.mobs_mut().get_mut(&42).unwrap().target_id = Some(150000);
        context.map_instance_service.dispel_mob(&mut state, 42);
        let mob = state.get_mob(42).unwrap();
        assert!(!mob.status_effects.has_status_change(StatusChangeKind::Berserk));
        assert!(!mob.status_effects.has_status_change(StatusChangeKind::Blessing));
        assert!(mob.status_effects.has_status_change(StatusChangeKind::Assumptio));
        assert_eq!(mob.hp(), base_hp);
        assert_eq!(mob.get_target_id(), None);
        state.mobs_mut().get_mut(&42).unwrap().target_id = Some(150000);
        context.map_instance_service.dispel_mob(&mut state, 42);
        assert_eq!(state.get_mob(42).unwrap().get_target_id(), None);
    }

    #[test]
    fn provoke_retargets_only_after_a_live_valid_target_accepts_the_status_and_preserves_hit_stun() {
        use models::status_change::{StatusChangeKind, StatusChangeRequest};

        use crate::server::model::events::map_event::MobProvoke;
        use crate::server::state::mob::MobAction;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        mob.target_id = Some(999);
        mob.action = MobAction::Chasing { target_id: 999 };
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        let request = StatusChangeRequest::guaranteed(StatusChangeKind::Provoke, 5000, 1);
        let mut failed = request.clone();
        failed.rate = 0;
        failed.flags = 0;
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                coma: Default::default(),
                request: failed,
            },
            1,
        );
        assert_eq!(state.get_mob(42).unwrap().get_target_id(), Some(999));
        assert!(
            !state
                .get_mob(42)
                .unwrap()
                .status_effects
                .has_status_change(StatusChangeKind::Provoke)
        );
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                coma: Default::default(),
                request: request.clone(),
            },
            2,
        );
        assert_eq!(state.get_mob(42).unwrap().get_target_id(), Some(150000));
        assert!(matches!(state.get_mob(42).unwrap().action, MobAction::Chasing {
            target_id: 150000
        }));
        let mob = state.mobs_mut().get_mut(&42).unwrap();
        mob.action = MobAction::Flinching { until: 100 };
        mob.timing.set_canmove_tick(100);
        mob.target_id = Some(999);
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                coma: Default::default(),
                request: request.clone(),
            },
            3,
        );
        assert!(matches!(state.get_mob(42).unwrap().action, MobAction::Flinching { until: 100 }));
        assert_eq!(state.get_mob(42).unwrap().timing.get_canmove_tick(), 100);
        assert_eq!(state.get_mob(42).unwrap().target_id, Some(150000));
        state.remove_item_with_id(150000);
        state.mobs_mut().get_mut(&42).unwrap().target_id = Some(999);
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                coma: Default::default(),
                request: request.clone(),
            },
            4,
        );
        assert_eq!(state.get_mob(42).unwrap().target_id, Some(999));
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        state
            .mobs_mut()
            .get_mut(&42)
            .unwrap()
            .status
            .set_race(models::enums::mob::MobRace::RUndead);
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                coma: Default::default(),
                request: request.clone(),
            },
            5,
        );
        assert_eq!(state.get_mob(42).unwrap().target_id, Some(999));
        let mob = state.mobs_mut().get_mut(&42).unwrap();
        mob.status.set_race(models::enums::mob::MobRace::Plant);
        mob.mode |= models::enums::mob::MobMode::Boss.as_flag();
        mob.status.set_mob_class(models::enums::mob::MobClass::Boss);
        mob.status.set_mob_capabilities(models::enums::mob::MobCapability::StatusImmune.as_flag());
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                coma: Default::default(),
                request,
            },
            6,
        );
        assert_eq!(state.get_mob(42).unwrap().target_id, Some(999));
    }

    #[test]
    fn mob_status_alternatives_stop_at_the_first_successful_status() {
        use models::status_change::{StatusChangeKind, StatusChangeRequest};
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        state.insert_mob(create_mob(42, "PORING"));
        let mut failed = StatusChangeRequest::guaranteed(StatusChangeKind::Freeze, 5000, 1);
        failed.rate = 0;
        failed.flags = 0;
        context.map_instance_service.start_mob_status_alternatives(
            &mut state,
            crate::server::model::events::map_event::MobStatusAlternatives {
                mob_id: 42,
                requests: vec![
                    failed,
                    StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 5000, 1),
                    StatusChangeRequest::guaranteed(StatusChangeKind::Poison, 5000, 1),
                ],
            },
            1,
        );
        let status = &state.get_mob(42).unwrap().status_effects;
        assert!(!status.has_status_change(StatusChangeKind::Freeze));
        assert!(status.has_status_change(StatusChangeKind::Stun));
        assert!(!status.has_status_change(StatusChangeKind::Poison));
    }

    #[test]
    fn absorbed_elemental_damage_heals_without_consuming_shields_or_emitting_combat_events() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        use models::status_change::{StatusChangeKind, StatusChangeRequest};
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        let max_hp = mob.status.max_hp();
        mob.set_hp(max_hp - 5);
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        for kind in [
            StatusChangeKind::Kyrie,
            StatusChangeKind::MagicRod,
            StatusChangeKind::MagicMirror,
            StatusChangeKind::NoRecovery,
        ] {
            context
                .map_instance_service
                .start_mob_status(&mut state, 42, StatusChangeRequest::guaranteed(kind, 5000, 5), 0);
        }
        let before = state.get_mob(42).unwrap().status_effects.active_statuses.clone();
        let mut hit = admitted_damage(42, 100, true);
        hit.skill_id = SkillEnum::MgFirebolt.id();
        hit.skill_level = 1;
        hit.battle_flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        hit.set_signed_damage(-100);
        assert_eq!((hit.damage, hit.healing), (0, 100));
        context
            .map_instance_service
            .mob_being_attacked(&mut state, hit, Arc::new(TasksQueue::new()), 1);
        let mob = state.get_mob(42).unwrap();
        assert_eq!(mob.hp(), max_hp);
        assert_eq!(mob.status_effects.active_statuses, before);
        assert!(mob.actor_damages.is_empty());
        assert_eq!(mob.last_attacker_id, 0);
        assert!(context.server_task_queue.pop().unwrap_or_default().is_empty());
        state.mobs_mut().get_mut(&42).unwrap().set_hp(0);
        context
            .map_instance_service
            .mob_being_attacked(&mut state, hit, Arc::new(TasksQueue::new()), 2);
        assert_eq!(state.get_mob(42).unwrap().hp(), 0);
    }

    #[test]
    fn equipment_shields_are_consumed_once_and_only_admitted_damage_is_credited() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mob = create_mob(42, "PORING");
        let hp = mob.hp();
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        context.map_instance_service.start_mob_status(
            &mut state,
            42,
            models::status_change::StatusChangeRequest::guaranteed(models::status_change::StatusChangeKind::Kyrie, 2000, 10),
            0,
        );
        let tasks = Arc::new(TasksQueue::new());
        let mut attack = admitted_damage(42, 10, true);
        attack.right_hand_damage = Some(5);
        context
            .map_instance_service
            .mob_being_attacked(&mut state, attack, tasks.clone(), 1);
        assert_eq!(state.get_mob(42).unwrap().hp(), hp);
        context.map_instance_service.mob_being_attacked(&mut state, attack, tasks, 2);
        let expected = 20 - hp * 30 / 100;
        assert_eq!(state.get_mob(42).unwrap().hp(), hp - expected);
        assert_eq!(state.get_mob(42).unwrap().actor_damages[&150000].damage, expected);
        let proc = context
            .server_task_queue
            .pop()
            .unwrap()
            .into_iter()
            .find_map(|event| {
                if let GameEvent::ScriptCombat(request) = event {
                    Some(request)
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(proc.right_hand_damage, Some(expected / 2));
        assert!(
            !state
                .get_mob(42)
                .unwrap()
                .status_effects
                .has_status_change(models::status_change::StatusChangeKind::Kyrie)
        );
    }

    #[test]
    fn magic_rod_absorbs_direct_spells_before_shields_and_suppresses_hit_callbacks() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        use models::status_change::{StatusChangeKind, StatusChangeRequest};
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        let hp = mob.hp();
        mob.base_status.set_max_sp(50);
        mob.status.set_max_sp(50);
        mob.status.set_sp(1);
        mob.status_effects.max_sp = 50;
        mob.status_effects.sp = 1;
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        for kind in [StatusChangeKind::MagicRod, StatusChangeKind::Kyrie] {
            context
                .map_instance_service
                .start_mob_status(&mut state, 42, StatusChangeRequest::guaranteed(kind, 5000, 5), 0);
        }
        let shield = state
            .get_mob(42)
            .unwrap()
            .status_effects
            .status_change(StatusChangeKind::Kyrie)
            .unwrap()
            .clone();
        let mut hit = admitted_damage(42, 100, true);
        hit.skill_id = SkillEnum::MgFirebolt.id();
        hit.skill_level = 1;
        hit.battle_flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        context
            .map_instance_service
            .mob_being_attacked(&mut state, hit, Arc::new(TasksQueue::new()), 1);
        let mob = state.get_mob(42).unwrap();
        assert_eq!(mob.hp(), hp);
        assert_eq!(mob.status.sp(), 13);
        assert_eq!(mob.status_effects.status_change(StatusChangeKind::Kyrie), Some(&shield));
        assert!(mob.actor_damages.is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap_or_default()
                .iter()
                .all(|event| !matches!(event, GameEvent::ScriptCombat(_) | GameEvent::ScriptSkillHit(_)))
        );
        context.map_instance_service.start_mob_status(
            &mut state,
            42,
            StatusChangeRequest::guaranteed(StatusChangeKind::MagicMirror, 5000, 5),
            1,
        );
        hit.landed = false;
        context
            .map_instance_service
            .mob_being_attacked(&mut state, hit, Arc::new(TasksQueue::new()), 2);
        assert_eq!(state.get_mob(42).unwrap().hp(), hp);
        assert_eq!(state.get_mob(42).unwrap().status.sp(), 25);
        assert!(
            context.server_task_queue.pop().unwrap_or_default().iter().all(|event| !matches!(
                event,
                GameEvent::ReflectMagic(_) | GameEvent::ScriptCombat(_) | GameEvent::ScriptSkillHit(_)
            ))
        );
        state
            .mobs_mut()
            .get_mut(&42)
            .unwrap()
            .end_status(Some(StatusChangeKind::MagicMirror));
        hit.skill_id = SkillEnum::WzStormgust.id();
        hit.damage = 5;
        hit.landed = true;
        context
            .map_instance_service
            .mob_being_attacked(&mut state, hit, Arc::new(TasksQueue::new()), 3);
        assert_eq!(state.get_mob(42).unwrap().hp(), hp - 5);
        assert_eq!(state.get_mob(42).unwrap().status.sp(), 25);
    }

    #[test]
    fn equipment_star_only_misses_damage_without_emitting_attack_procs() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mob = create_mob(42, "PORING");
        let hp = mob.hp();
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        context
            .map_instance_service
            .mob_being_attacked(&mut state, admitted_damage(42, 5, false), Arc::new(TasksQueue::new()), 1);
        assert_eq!(state.get_mob(42).unwrap().hp(), hp - 5);
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap_or_default()
                .iter()
                .all(|event| !matches!(event, GameEvent::ScriptCombat(_)))
        );
    }

    #[test]
    fn equipment_actual_kill_emits_one_attack_one_kill_and_actor_contributions() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mob = create_mob(42, "PORING");
        let hp = mob.hp();
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        context
            .map_instance_service
            .mob_being_attacked(&mut state, admitted_damage(42, hp + 100, true), Arc::new(TasksQueue::new()), 1);
        let events = context.server_task_queue.pop().unwrap();
        let procs = events
            .iter()
            .filter_map(|event| {
                if let GameEvent::ScriptCombat(proc) = event {
                    Some(proc.trigger)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(procs, vec![
            models::status_bonus::CombatTrigger::Attack,
            models::status_bonus::CombatTrigger::Kill
        ]);
        let kill = events
            .iter()
            .find_map(|event| {
                if let GameEvent::CharacterKillMonster(kill) = event {
                    Some(kill)
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(kill.attacker_id, 150000);
        assert_eq!(kill.contributions, vec![crate::server::model::action::DamageContribution {
            actor_id: 150000,
            owner_id: 150000,
            damage: hp
        }]);
        assert!(!state.get_mob(42).unwrap().is_present());
    }

    #[test]
    fn equipment_script_spawn_is_live_and_never_decrements_static_respawn_tracking() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut track = crate::server::state::map_instance::MobSpawnTrack::default(0);
        track.spawned_amount = 3;
        state.mob_spawns_tracks_mut().insert(0, track);
        let request = crate::server::model::events::map_event::ScriptSpawn {
            event_npc: None,
            mob_id: 1002,
            x: 1,
            y: 1,
            name: "Summoned Poring".into(),
            amount: 2,
            event: String::new(),
            size: Some(2),
            ai: Some(0),
            owner_id: 150000,
            guardian: None,
            bg_id: 0,
            max_hp: None,
            lifetime_ms: None,
        reserved_id: None,
        };
        let ids = context.map_instance_service.script_spawn(&mut state, request.clone()).unwrap();
        assert_eq!(ids.len(), 2);
        assert_eq!(state.get_mob(ids[0]).unwrap().status.size(), &models::enums::size::Size::Large);
        assert_eq!(state.get_mob(ids[0]).unwrap().name, "Summoned Poring");
        context.map_instance_service.mob_die(&mut state, ids[0], 0);
        assert_eq!(state.mob_spawns_tracks()[&0].spawned_amount, 3);
        let mut invalid = request;
        invalid.event = "Missing::OnKill".into();
        assert!(context.map_instance_service.script_spawn(&mut state, invalid).is_err());
        assert_eq!(state.mobs().len(), 2);
    }

    #[test]
    fn naturally_expired_mob_splasher_emits_one_explosion_before_removing_the_countdown() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        mob.status_effects.active_statuses.push(models::status_change::StatusChange {
            kind: models::status_change::StatusChangeKind::Splasher,
            values: [5, models::enums::skill_enums::SkillEnum::AsSplasher.id() as i32, 150000, 1000],
            started_at: 0,
            expires_at: Some(1000),
            next_periodic_at: 1000,
            flags: 0,
            inherited_from: None,
        });
        state.insert_mob(mob);
        let tasks = TasksQueue::new();
        context.map_instance_service.tick_mob_statuses(&mut state, &tasks, 999);
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap_or_default()
                .iter()
                .all(|event| !matches!(event, GameEvent::CharacterScriptSkill(_)))
        );
        context.map_instance_service.tick_mob_statuses(&mut state, &tasks, 1000);
        let events = context.server_task_queue.pop().unwrap();
        let explosions = events
            .iter()
            .filter_map(|event| {
                if let GameEvent::CharacterScriptSkill(effect) = event {
                    Some(effect)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(explosions.len(), 1);
        assert_eq!(
            (explosions[0].source_char_id, explosions[0].target_id, explosions[0].level),
            (150000, 42, 5)
        );
        assert!(explosions[0].skill_event_emitted);
        assert!(
            !state
                .get_mob(42)
                .unwrap()
                .status_effects
                .has_status_change(models::status_change::StatusChangeKind::Splasher)
        );
        context.map_instance_service.tick_mob_statuses(&mut state, &tasks, 1040);
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap_or_default()
                .iter()
                .all(|event| !matches!(event, GameEvent::CharacterScriptSkill(_)))
        );
    }

    #[test]
    fn provoke_coma_is_admitted_only_after_the_status_succeeds_without_ordinary_attack_callbacks() {
        use models::enums::bonus::BonusType;
        use models::enums::mob::MobClass;
        use models::status_bonus::StatusBonus;
        use models::status_change::{StatusChangeKind, StatusChangeRequest};

        use crate::server::model::events::map_event::MobProvoke;
        use crate::server::service::combat_trigger_service::ComaBonuses;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut actor = create_mob(42, "PORING");
        actor.base_status.set_max_sp(10);
        actor.status.set_max_sp(10);
        actor.status.set_sp(5);
        actor.status_effects.max_sp = 10;
        actor.status_effects.sp = 5;
        state.insert_mob(actor);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        let hp = state.get_mob(42).unwrap().hp();
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Provoke, 5000, 1);
        request.rate = 0;
        request.flags = 0;
        let coma = ComaBonuses::from_bonuses(
            &[StatusBonus::new(BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(
                MobClass::All,
                100.0,
            ))],
        );
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                request,
                coma: coma.clone(),
            },
            0,
        );
        assert_eq!(state.get_mob(42).unwrap().hp(), hp);
        assert_eq!(state.get_mob(42).unwrap().status.sp(), 5);
        let weapon_only = ComaBonuses::from_bonuses(&[StatusBonus::new(BonusType::WeaponComaAgainstClass(MobClass::All, 10000))]);
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                request: StatusChangeRequest::guaranteed(StatusChangeKind::Provoke, 5000, 1),
                coma: weapon_only,
            },
            1,
        );
        assert_eq!(state.get_mob(42).unwrap().hp(), hp);
        context.map_instance_service.provoke_mob(
            &mut state,
            MobProvoke {
                mob_id: 42,
                source_id: 150000,
                request: StatusChangeRequest::guaranteed(StatusChangeKind::Provoke, 5000, 1),
                coma,
            },
            2,
        );
        let mob = state.get_mob(42).unwrap();
        assert_eq!(mob.hp(), 1);
        assert_eq!(mob.status.sp(), 1);
        assert_eq!(mob.target_id, Some(150000));
        assert!(mob.status_effects.has_status_change(StatusChangeKind::Provoke));
        assert!(context.server_task_queue.pop().unwrap_or_default().is_empty());
    }

    #[test]
    fn gvg_mob_damage_consumes_shields_before_rates_and_caps_hp_after_the_reduction() {
        use models::status_change::{StatusChangeKind, StatusChangeRequest};

        use crate::server::model::map_flags::MapFlag;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        mob.base_status.set_max_hp(100);
        mob.status.set_max_hp(100);
        mob.status_effects.max_hp = 100;
        mob.set_hp(100);
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        state.flags.set(MapFlag::Gvg, true, &[]).unwrap();
        context.map_instance_service.start_mob_status(
            &mut state,
            42,
            StatusChangeRequest::guaranteed(StatusChangeKind::Kyrie, 2000, 10),
            0,
        );
        let tasks = Arc::new(TasksQueue::new());
        context
            .map_instance_service
            .mob_being_attacked(&mut state, admitted_damage(42, 50, true), tasks.clone(), 1);
        let mob = state.get_mob(42).unwrap();
        assert_eq!(mob.hp(), 84);
        assert_eq!(mob.actor_damages[&150000].damage, 16);
        assert!(!mob.status_effects.has_status_change(StatusChangeKind::Kyrie));
        let mut skill = admitted_damage(42, 1000, true);
        skill.battle_flags = models::status_bonus::BattleFlag::Weapon.as_flag()
            | models::status_bonus::BattleFlag::Short.as_flag()
            | models::status_bonus::BattleFlag::Skill.as_flag();
        skill.skill_id = models::enums::skill_enums::SkillEnum::SmBash.id();
        skill.skill_level = 1;
        context.map_instance_service.mob_being_attacked(&mut state, skill, tasks, 2);
        assert_eq!(state.get_mob(42).unwrap().hp(), 0);
        assert_eq!(state.get_mob(42).unwrap().actor_damages[&150000].damage, 100);
    }

    #[test]
    fn pressure_bypasses_mob_shields_and_gvg_rates_while_stone_fling_is_blocked_by_pneuma() {
        use models::enums::skill_enums::SkillEnum;
        use models::status_bonus::BattleFlag;
        use models::status_change::{StatusChangeKind, StatusChangeRequest};

        use crate::server::model::map_flags::MapFlag;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mut mob = create_mob(42, "PORING");
        mob.base_status.set_max_hp(5000);
        mob.status.set_max_hp(5000);
        mob.status_effects.max_hp = 5000;
        mob.set_hp(5000);
        state.insert_mob(mob);
        state.insert_item(MapItem::new(150000, 0, MapItemType::Character));
        state.flags.set(MapFlag::Gvg, true, &[]).unwrap();
        for kind in [
            StatusChangeKind::Kyrie,
            StatusChangeKind::LexAeterna,
            StatusChangeKind::Assumptio,
            StatusChangeKind::Pneuma,
        ] {
            context
                .map_instance_service
                .start_mob_status(&mut state, 42, StatusChangeRequest::guaranteed(kind, 10000, 10), 0);
        }
        let shields = state.get_mob(42).unwrap().status_effects.active_statuses.clone();
        let mut pressure = admitted_damage(42, 2000, true);
        pressure.skill_id = SkillEnum::PaPressure.id();
        pressure.skill_level = 5;
        pressure.battle_flags = BattleFlag::Misc.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        let tasks = Arc::new(TasksQueue::new());
        context
            .map_instance_service
            .mob_being_attacked(&mut state, pressure, tasks.clone(), 1);
        assert_eq!(state.get_mob(42).unwrap().hp(), 3000);
        assert_eq!(state.get_mob(42).unwrap().actor_damages[&150000].damage, 2000);
        assert_eq!(state.get_mob(42).unwrap().status_effects.active_statuses, shields);
        let mut stone = admitted_damage(42, 50, true);
        stone.skill_id = SkillEnum::TfThrowstone.id();
        stone.skill_level = 1;
        stone.battle_flags = pressure.battle_flags | BattleFlag::Weapon.as_flag();
        context.map_instance_service.mob_being_attacked(&mut state, stone, tasks, 2);
        assert_eq!(state.get_mob(42).unwrap().hp(), 3000);
        assert_eq!(state.get_mob(42).unwrap().actor_damages[&150000].damage, 2000);
        assert_eq!(state.get_mob(42).unwrap().status_effects.active_statuses, shields);
    }

    #[test]
    fn live_map_flags_block_capture_wild_monster_warp_and_regular_loot() {
        use crate::server::model::events::map_event::PetCaptureClaimRequest;
        use crate::server::model::map_flags::MapFlag;
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        let mob = create_mob(42, "PORING");
        let (x, y) = (mob.x, mob.y);
        let pet = crate::server::service::script_world_service::world_data()
            .pets
            .iter()
            .find(|pet| pet.class_id == mob.mob_id as u16)
            .unwrap();
        let request = PetCaptureClaimRequest {
            claim_id: 1,
            char_id: 150000,
            target_id: 42,
            x,
            y,
            lure_item_id: pet.tame_item,
            flag: 0,
            expires_at: 1000,
        };
        state.insert_mob(mob);
        state.flags.set(MapFlag::NoPetCapture, true, &[]).unwrap();
        context
            .map_instance_service
            .claim_pet_capture_with_roll(&mut state, request.clone(), 1, 0);
        assert!(state.pending_pet_captures.is_empty());
        assert!(
            context
                .server_task_queue
                .pop()
                .unwrap()
                .into_iter()
                .all(|event| matches!(event, GameEvent::PetCaptureClaimResult(result) if result.class_id.is_none()))
        );
        state.flags.set(MapFlag::MonsterNoTeleport, true, &[]).unwrap();
        context.map_instance_service.warp_mob_to(&mut state, 42, 1, 2);
        context.map_instance_service.random_warp_mob(&mut state, 42);
        assert_eq!((state.get_mob(42).unwrap().x, state.get_mob(42).unwrap().y), (x, y));
        state.mobs_mut().get_mut(&42).unwrap().summon_owner = Some(150000);
        context.map_instance_service.warp_mob_to(&mut state, 42, 1, 2);
        assert_eq!((state.get_mob(42).unwrap().x, state.get_mob(42).unwrap().y), (1, 2));
        state.flags.set(MapFlag::NoMobLoot, true, &[]).unwrap();
        assert!(
            context
                .map_instance_service
                .mob_drop_items(&mut state, MobDropItems {
                    owner_id: 150000,
                    mob_id: 1002,
                    mob_x: 1,
                    mob_y: 2
                })
                .is_empty()
        );
        context.map_instance_service.script_drop_item(&mut state, 150000, 501, 1, 1, 2);
        assert_eq!(state.map_items().len(), 1);
        state.flags.set(MapFlag::NoPetCapture, false, &[]).unwrap();
        context
            .map_instance_service
            .claim_pet_capture_with_roll(&mut state, PetCaptureClaimRequest { x: 1, y: 2, ..request }, 2, 0);
        assert_eq!(state.pending_pet_captures.len(), 1);
    }

    #[test]
    fn directed_mob_warp_preserves_facing_and_rejects_positions_outside_the_map() {
        let context = before_each();
        let mut state = create_empty_map_instance_state();
        state.insert_mob(create_mob(42, "PORING"));
        context.map_instance_service.face_mob(&mut state, 42, 7);
        assert_eq!(state.get_mob(42).unwrap().dir, 7);
        context.map_instance_service.warp_mob_to(&mut state, 42, 1, 2);
        let mob = state.get_mob(42).unwrap();
        assert_eq!((mob.x, mob.y, mob.dir), (1, 2, 7));
        let outside = state.x_size();
        context.map_instance_service.warp_mob_to(&mut state, 42, outside, 2);
        context.map_instance_service.face_mob(&mut state, 42, 8);
        let mob = state.get_mob(42).unwrap();
        assert_eq!((mob.x, mob.y, mob.dir), (1, 2, 7));
    }
}
