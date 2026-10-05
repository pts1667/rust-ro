#![allow(dead_code)]
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use tokio::runtime::Runtime;

use crate::server::Server;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::skill::ScriptSkillService;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;
use crate::server::service::script_service::ScriptService;
use crate::server::service::server_service::ServerService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::mocked_repository::MockedRepository;
use crate::tests::common::sync_helper::CountDownLatch;
use crate::tests::common::{ServerBuilder, TestContext, create_mpsc, test_script_vm};

#[path = "native_skill_payment_test.rs"]
mod native_payment_tests;

#[cfg(feature = "unit_tests")]
#[path = "actor_unit_skill_test.rs"]
mod actor_unit_skill_tests;

#[cfg(feature = "unit_tests")]
#[path = "npc_unit_data_test.rs"]
mod npc_unit_data_tests;

#[cfg(feature = "unit_tests")]
#[path = "npc_effect_test.rs"]
mod npc_effect_tests;

#[path = "item_dialog_test.rs"]
mod item_dialog_tests;

#[path = "script_operation_test.rs"]
mod script_operation_tests;

#[cfg(feature = "unit_tests")]
#[path = "script_map_command_test.rs"]
mod script_map_command_tests;

#[path = "world_party_test.rs"]
mod world_party_tests;

#[path = "party_reward_test.rs"]
mod party_reward_tests;

#[path = "map_flag_test.rs"]
mod map_flag_tests;

#[path = "item_map_flag_test.rs"]
mod item_map_flag_tests;

#[path = "party_warp_test.rs"]
mod party_warp_tests;
#[path = "battleground_test.rs"]
mod battleground_tests;

#[path = "player_trade_test.rs"]
mod player_trade_tests;

struct ServerServiceTestContext {
    test_context: TestContext,
    client_notification_sender: SyncSender<Notification>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    movement_task_queue: Arc<TasksQueue<GameEvent>>,
    status_service: &'static StatusService,
    server: Server,
    runtime: Arc<Runtime>,
}

impl ServerServiceTestContext {
    pub fn runtime(&self) -> &Runtime {
        self.runtime.as_ref()
    }
}

fn before_each() -> ServerServiceTestContext {
    before_each_with_latch(0)
}

#[cfg(not(feature = "integration_tests"))]
fn before_each_pickup() -> (ServerServiceTestContext, crate::server::state::character::Character) {
    use sled::transaction::Transactional;
    let (context, repository, character) = native_payment_tests::fixture(false, true);
    repository.database.items.transaction(|tree| {
        for name in ["Red_Potion", "Clover", "Knife"] {
            let item = GlobalConfigService::instance().get_item_by_name(name);
            database::tx_write(tree, &item.id.to_be_bytes(), item)?;
        }
        Ok(())
    }).unwrap();
    (context, character)
}

fn before_each_with_latch(latch_size: usize) -> ServerServiceTestContext {
    before_each_with_repository(latch_size, Arc::new(MockedRepository), true)
}

fn before_each_with_repository(latch_size: usize, repository: Arc<dyn crate::repository::Repository>, force_no_delay: bool) -> ServerServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let server_task_queue = Arc::new(TasksQueue::new());
    let movement_task_queue = Arc::new(TasksQueue::new());
    let count_down_latch = CountDownLatch::new(latch_size);
    StatusService::init(GlobalConfigService::instance(), test_script_vm());
    let skill_service = SkillService::new(
        client_notification_sender.clone(), persistence_event_sender.clone(),
        BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal),
        StatusService::instance(), GlobalConfigService::instance());
    let skill_service = if force_no_delay { skill_service.force_no_delay() } else { skill_service };
    let server_service = ServerService::new(
        client_notification_sender.clone(),
        GlobalConfigService::instance(),
        server_task_queue.clone(),
        movement_task_queue.clone(),
        test_script_vm(),
        InventoryService::new(
            client_notification_sender.clone(),
            persistence_event_sender.clone(),
            repository.clone(),
            GlobalConfigService::instance(),
            server_task_queue.clone(),
        ),
        BattleService::new(
            client_notification_sender.clone(),
            StatusService::instance(),
            GlobalConfigService::instance(),
            BattleResultMode::Normal,
        ),
        skill_service,
        StatusService::instance(),
        ScriptService::new(
            client_notification_sender.clone(),
            GlobalConfigService::instance(),
            repository.clone(),
            server_task_queue.clone(),
            test_script_vm(),
        ),
        CharacterService::new(
            client_notification_sender.clone(),
            persistence_event_sender.clone(),
            repository.clone(),
            GlobalConfigService::instance(),
            SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
            StatusService::instance(),
            server_task_queue.clone(),
        ),
        SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
        ItemService::new(
            client_notification_sender.clone(),
            persistence_event_sender.clone(),
            repository.clone(),
            test_script_vm(),
            GlobalConfigService::instance(),
        ),
        ScriptSkillService::new(
            client_notification_sender.clone(),
            persistence_event_sender.clone(),
            repository.clone(),
            GlobalConfigService::instance(),
        ),
    );

    let runtime = Arc::new(Runtime::new().unwrap());
    let server = Server::new_without_service_init(GlobalConfigService::instance().config(), repository, Default::default(), server_task_queue.clone(), server_service, runtime.clone());
    ServerServiceTestContext {
        client_notification_sender: client_notification_sender.clone(),
        test_context: TestContext::new(
            client_notification_sender.clone(),
            client_notification_receiver,
            persistence_event_sender.clone(),
            persistence_event_receiver,
            count_down_latch,
        ),
        server_task_queue: server_task_queue.clone(),
        movement_task_queue: movement_task_queue.clone(),
        status_service: StatusService::instance(),
        runtime,
        server,
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::mem;
    use std::sync::Arc;
    use std::time::Duration;

    use models::enums::bonus::BonusType;
    use models::enums::skill_enums::SkillEnum;
    use models::item::DroppedItem;
    use movement::position::Position;
    use models::status::KnownSkill;
    use models::status_bonus::{StatusBonus, StatusBonuses};
    use packets::packets::{PacketZcMsgStateChange, PacketZcMsgStateChange2};

    use crate::server::Server;
    use crate::server::model::events::game_event::CharacterUseSkill;
    use crate::server::model::events::map_event::{MapEvent, RemoveDroppedItemFromMap};
    use crate::server::model::map_item::ToMapItem;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::common::assert_helper::{
        NotificationExpectation, SentPacket, assert_vecs_equal, has_sent_notification, task_queue_contains_event_at_tick,
    };
    use crate::tests::common::character_helper::create_character;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::common::server_helper::create_empty_server_state;
    use crate::tests::server_service_test::before_each;
    use crate::util::tick::get_tick;
    use crate::{assert_sent_packet_in_current_packetver, assert_vec_equals, status_snapshot};

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_when_item_in_fov() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id,
            item_id: 501,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, 501);
        assert_eq!(item_from_inventory.amount, 2);
    }

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_and_keep_is_identified_status_from_item_drop() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let red_potion_map_item_id = 1000;
        let clover_map_item_id = 1001;
        let knife_map_item_id = 1002;
        let red_potion = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id: red_potion_map_item_id,
            item_id: GlobalConfigService::instance().get_item_id_from_name("Red_Potion") as i32,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        };
        let clover = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id: clover_map_item_id,
            item_id: GlobalConfigService::instance().get_item_id_from_name("Clover") as i32,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        };
        let knife = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id: knife_map_item_id,
            item_id: GlobalConfigService::instance().get_item_id_from_name("Knife") as i32,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 1,
            is_identified: false,
        };
        // Add dropped item in character fov
        character_state.map_view.insert(red_potion.to_map_item());
        map_instance.state_mut().insert_dropped_item(red_potion);
        character_state.map_view.insert(clover.to_map_item());
        map_instance.state_mut().insert_dropped_item(clover);
        character_state.map_view.insert(knife.to_map_item());
        map_instance.state_mut().insert_dropped_item(knife);
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            red_potion_map_item_id,
            &map_instance,
        ).unwrap();
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            clover_map_item_id,
            &map_instance,
        ).unwrap();
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            knife_map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(
            item_from_inventory.item_id,
            GlobalConfigService::instance().get_item_id_from_name("Red_Potion") as i32
        );
        assert!(item_from_inventory.is_identified);
        let item_from_inventory = character_state.get_item_from_inventory(1).unwrap();
        assert_eq!(
            item_from_inventory.item_id,
            GlobalConfigService::instance().get_item_id_from_name("Clover") as i32
        );
        assert!(item_from_inventory.is_identified);
        let item_from_inventory = character_state.get_item_from_inventory(2).unwrap();
        assert_eq!(
            item_from_inventory.item_id,
            GlobalConfigService::instance().get_item_id_from_name("Knife") as i32
        );
        assert!(!item_from_inventory.is_identified);
    }

    #[test]
    fn character_pickup_item_should_prevent_pickup_when_item_not_in_fov() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        map_instance.state_mut().insert_dropped_item(DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id,
            item_id: 501,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        });
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
    }

    #[test]
    fn character_pickup_item_should_prevent_pickup_when_item_is_still_locked_by_another_player() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id,
            item_id: 501,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: Some(15001),
            dropped_at: get_tick() - 10,
            amount: 2,
            is_identified: true,
        };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
    }

    #[test]
    fn character_pickup_item_should_pickup_when_item_is_no_longer_locked_by_another_player() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id,
            item_id: 501,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: Some(15001),
            dropped_at: get_tick()
                - 10
                - (GlobalConfigService::instance()
                    .config()
                    .game
                    .mob_dropped_item_locked_to_owner_duration_in_secs as u128
                    * 1000),
            amount: 2,
            is_identified: true,
        };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_some());
    }

    #[test]
    fn character_pickup_item_should_remove_map_item_from_map_instance() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let task_queue = Arc::new(TasksQueue::new());
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), task_queue.clone());
        let map_item_id = 1000;
        let item = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id,
            item_id: 501,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        assert!(map_instance.state().get_map_item(map_item_id).is_some());
        assert!(map_instance.state().get_dropped_item(map_item_id).is_some());
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_some());
        task_queue_contains_event_at_tick::<MapEvent>(task_queue, MapEvent::RemoveDroppedItemFromMap(RemoveDroppedItemFromMap { dropped_item_id: map_item_id }), 0);
    }

    #[test]
    fn character_pickup_item_should_be_at_most_called_once() {
        // Given
        let (context, mut character_state) = super::before_each_pickup();
        let mut server_state = create_empty_server_state();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { attributes: Default::default(), player_dropped: false,
            map_item_id,
            item_id: 501,
            location: Position { x: 50, y: 50, dir: 0 },
            sub_location: Position { x: 3, y: 3, dir: 0 },
            owner_id: None,
            dropped_at: 0,
            amount: 2,
            is_identified: true,
        };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        context.server.server_service().character_pickup_item(
            &context.server,
            &mut server_state,
            &mut character_state,
            map_item_id,
            &map_instance,
        ).unwrap();
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, 501);
        assert_eq!(item_from_inventory.amount, 2);
    }

    #[test]
    fn character_use_support_skill_should_apply_bonuses_and_send_add_bonuses_packet() {
        let (context, repository, mut character) = super::native_payment_tests::fixture(false, false);
        let char_id = character.char_id;
        character.status.known_skills = vec![KnownSkill { value: SkillEnum::AlBlessing, level: 10 }, KnownSkill { value: SkillEnum::AlIncagi, level: 10 }];
        context.server.state_mut().insert_character(character);
        context.server.handle_character_skill(context.server.state_mut().as_mut(), CharacterUseSkill { char_id, target_id: char_id, skill_id: SkillEnum::AlBlessing.id(), skill_level: 10 }, 0).unwrap();
        for tick in (40..=1000).step_by(40) { Server::game_loop_iteration(&context.server, tick); }
        context.server.handle_character_skill(context.server.state_mut().as_mut(), CharacterUseSkill { char_id, target_id: char_id, skill_id: SkillEnum::AlIncagi.id(), skill_level: 10 }, 1000).unwrap();
        for tick in (1040..=2200).step_by(40) { Server::game_loop_iteration(&context.server, tick); }
        let character = context.server.state().get_character(char_id).unwrap();
        assert!(character.status.has_status_change(models::status_change::StatusChangeKind::Blessing));
        assert!(character.status.has_status_change(models::status_change::StatusChangeKind::IncreaseAgi));
        let snapshot = context.status_service.to_snapshot(&character.status);
        assert_eq!((snapshot.str(), snapshot.int(), snapshot.dex(), snapshot.agi(), snapshot.speed()), (11, 11, 11, 13, 112));
        let stored: database::model::CharacterRecord = database::required(&repository.database.characters, &char_id.to_be_bytes()).unwrap();
        assert_eq!((stored.hp, stored.sp), (985, 891));
        context.test_context.clear_sent_packet();
        Server::game_loop_iteration(&context.server, 243000);
        let character = context.server.state().get_character(char_id).unwrap();
        assert!(!character.status.has_status_change(models::status_change::StatusChangeKind::Blessing));
        assert!(!character.status.has_status_change(models::status_change::StatusChangeKind::IncreaseAgi));
        let snapshot = context.status_service.to_snapshot(&character.status);
        assert_eq!((snapshot.str(), snapshot.int(), snapshot.dex(), snapshot.agi(), snapshot.speed()), (1, 1, 1, 1, 150));
    }
}
