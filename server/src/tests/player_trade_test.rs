use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use database::model::{CharacterInventory, CharacterRecord, InventoryRecord, SeedData};
use models::enums::cell::CellType;
use models::enums::item::EquipmentLocation;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU64, EnumWithNumberValue};
use models::status::KnownSkill;
use sled::transaction::Transactional;

use super::ServerServiceTestContext;
use crate::repository::{InventoryRepository, SledRepository};
use crate::server::model::client_socket::ClientSocket;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::{GameEvent, PlayerTradeAction};
use crate::server::model::game_systems::{ItemContainer, PlayerTradePhase, PlayerTradeRequest, ScriptWorldRequest, StoreRequest, ContainerRequest};
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::model::permission_groups::PermissionGroups;
use crate::server::model::session::{AccountSession, Session};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::player_trade_service::{TRADE_IDLE_TIMEOUT_MS, decode_request};

const FIRST: u32 = 150_000;
const SECOND: u32 = 150_001;

fn fixture() -> (ServerServiceTestContext, Arc<SledRepository>) {
    let (context, repository) = super::world_party_tests::fixture();
    let first_items = vec![
        InventoryRecord {
            id: 10,
            unique_id: 987654,
            item_id: 1201,
            amount: 1,
            refine: 7,
            is_damaged: true,
            card0: 255,
            card1: 3,
            card2: 71,
            card3: 72,
            ..Default::default()
        },
        InventoryRecord {
            id: 11,
            item_id: 501,
            amount: 5,
            is_identified: true,
            ..Default::default()
        },
    ];
    let second_items = vec![InventoryRecord {
        id: 20,
        item_id: 501,
        amount: 4,
        is_identified: true,
        ..Default::default()
    }];
    repository
        .database
        .seed(
            &SeedData {
                characters: vec![
                    CharacterRecord {
                        char_id: FIRST as i32,
                        account_id: 2_000_000,
                        name: "Trader A".into(),
                        hp: 40,
                        max_hp: 1000,
                        sp: 1000,
                        max_sp: 1000,
                        zeny: 100,
                        inventory_slots: 100,
                        ..Default::default()
                    },
                    CharacterRecord {
                        char_id: SECOND as i32,
                        account_id: 2_000_001,
                        name: "Trader B".into(),
                        hp: 40,
                        max_hp: 1000,
                        sp: 1000,
                        max_sp: 1000,
                        zeny: 50,
                        inventory_slots: 100,
                        ..Default::default()
                    },
                ],
                inventories: vec![
                    CharacterInventory {
                        char_id: FIRST as i32,
                        items: first_items,
                    },
                    CharacterInventory {
                        char_id: SECOND as i32,
                        items: second_items,
                    },
                ],
                ..Default::default()
            },
            true,
        )
        .unwrap();
    repository
        .database
        .items
        .transaction(|tree| {
            for id in [501_i32, 1201, 1750] {
                database::tx_write(tree, &id.to_be_bytes(), GlobalConfigService::instance().get_item(id))?;
            }
            Ok(())
        })
        .unwrap();
    for id in [FIRST, SECOND] {
        let inventory = context.runtime().block_on(repository.character_inventory_fetch(id as i32)).unwrap();
        let mut character = context.server.state_mut().characters_mut().remove(&id).unwrap();
        character.name = if id == FIRST { "Trader A" } else { "Trader B" }.into();
        character.status.hp = 40;
        character.status.zeny = if id == FIRST { 100 } else { 50 };
        character.loaded_from_client_side = true;
        character.inventory = inventory.into_iter().map(Some).collect();
        character.status.known_skills.retain(|skill| skill.value != SkillEnum::NvBasic);
        character.status.known_skills.push(KnownSkill {
            value: SkillEnum::NvBasic,
            level: 7,
        });
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let socket = Arc::new(RwLock::new(ClientSocket::tcp(TcpStream::connect(listener.local_addr().unwrap()).unwrap())));
        let _peer = listener.accept().unwrap();
        let mut session = Session::create_empty(character.account_id, id as i32, 0, context.server.packetver());
        session.char_id = Some(id);
        session.map_server_socket = Some(socket);
        context.server.state_mut().add_session(character.account_id, Arc::new(session));
        context.server.state_mut().insert_character(character);
    }
    (context, repository)
}

fn act(context: &ServerServiceTestContext, id: u32, request: PlayerTradeRequest, now: u64) -> Result<(), String> {
    let guard_137 = context.server.state();
    let character = guard_137.characters().get(&id).unwrap();
    let session = guard_137.find_session(character.account_id).unwrap();
    let action = PlayerTradeAction {
        char_id: id,
        account_id: character.account_id,
        auth_code: session.auth_code,
        request,
    };
    drop(guard_137);
    context.server.handle_player_trade(&mut *context.server.state_mut(), action, now)
}

fn open(context: &ServerServiceTestContext) {
    act(context, FIRST, PlayerTradeRequest::Request(2_000_001), 100).unwrap();
    act(context, SECOND, PlayerTradeRequest::Answer(true), 101).unwrap();
}

fn economy(repository: &SledRepository) -> Vec<Vec<(Vec<u8>, Vec<u8>)>> {
    [
        &repository.database.characters,
        &repository.database.inventories,
        &repository.database.inventory_owners,
        &repository.database.game_systems,
    ]
    .into_iter()
    .map(|tree| {
        tree.iter()
            .map(|pair| {
                let (key, value) = pair.unwrap();
                (key.to_vec(), value.to_vec())
            })
            .collect()
    })
    .collect()
}

fn packet(context: &ServerServiceTestContext, actor: u32, header: u16) -> Vec<u8> {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        context.server.script_world_service().drain_notifications().unwrap();
        let received = context.test_context.received_notification();
        if let Some(packet) = received.lock().unwrap().iter().find_map(|notification| match notification {
            Notification::Char(notification)
                if notification.char_id() == actor
                    && notification.serialized_packet().get(..2) == Some(header.to_le_bytes().as_slice()) =>
            {
                Some(notification.serialized_packet().clone())
            }
            _ => None,
        }) {
            return packet;
        }
        assert!(Instant::now() < deadline, "Missing trade packet {header:#06x} for {actor}");
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn assert_closed(context: &ServerServiceTestContext) {
    for id in [FIRST, SECOND] {
        assert!(context.server.state().characters().get(&id).unwrap().game_systems.trade.is_none());
    }
}

#[test]
fn client_trade_framing_preserves_fragmentation_and_separates_every_supported_request() {
    let mut frames = crate::server::request_handler::framing::ClientFrames::new(20120229);
    let mut request = 0xE4_u16.to_le_bytes().to_vec();
    request.extend_from_slice(&2_000_001_u32.to_le_bytes());
    assert!(frames.push(&request[..3]).unwrap().is_empty());
    let mut suffix = request[3..].to_vec();
    suffix.extend_from_slice(&[0xE6, 0, 3, 0xE8, 0, 2, 0, 1, 0, 0, 0, 0xEB, 0, 0xEF, 0, 0xED, 0]);
    let result = frames.push(&suffix).unwrap();
    assert_eq!(result.iter().map(Vec::len).collect::<Vec<_>>(), vec![6, 3, 8, 2, 2, 2]);
    assert_eq!(
        decode_request(&result[0]).unwrap(),
        Some(PlayerTradeRequest::Request(2_000_001))
    );
    assert_eq!(decode_request(&result[1]).unwrap(), Some(PlayerTradeRequest::Answer(true)));
    assert_eq!(
        decode_request(&result[2]).unwrap(),
        Some(PlayerTradeRequest::Offer { index: 2, amount: 1 })
    );
    assert!(decode_request(&[0xE6, 0, 2]).is_err());
    assert!(decode_request(&[0xE8, 0, 0, 0, 1, 0, 0]).is_err());
    assert!(decode_request(&[0xEB, 0, 0]).is_err());
    assert_eq!(decode_request(&[0xA7, 1, 0, 0]).unwrap(), None);
}

#[test]
fn trade_request_uses_account_ids_and_only_the_recipient_can_accept() {
    let (context, repository) = fixture();
    let before = economy(&repository);
    act(&context, FIRST, PlayerTradeRequest::Request(2_000_001), 100).unwrap();
    let request = packet(&context, SECOND, 0x01F4);
    assert_eq!(request.len(), 32);
    assert_eq!(&request[2..10], b"Trader A");
    assert_eq!(u32::from_le_bytes(request[26..30].try_into().unwrap()), 2_000_000);
    act(&context, FIRST, PlayerTradeRequest::Answer(true), 101).unwrap();
    assert_eq!(
        context
            .server
            .state()
            .characters()
            .get(&FIRST)
            .unwrap()
            .game_systems
            .trade
            .as_ref()
            .unwrap()
            .phase,
        PlayerTradePhase::Requested
    );
    act(&context, SECOND, PlayerTradeRequest::Answer(true), 102).unwrap();
    for (id, other) in [(FIRST, 2_000_001), (SECOND, 2_000_000)] {
        assert_eq!(
            context
                .server
                .state()
                .characters()
                .get(&id)
                .unwrap()
                .game_systems
                .trade
                .as_ref()
                .unwrap()
                .phase,
            PlayerTradePhase::Accepted
        );
        let response = packet(&context, id, 0x01F5);
        assert_eq!((response.len(), response[2]), (9, 3));
        assert_eq!(u32::from_le_bytes(response[3..7].try_into().unwrap()), other);
    }
    assert_eq!(economy(&repository), before);
}

#[test]
fn trade_two_phase_commit_moves_exact_instances_stacks_and_wallets_once() {
    let (context, repository) = fixture();
    let before = economy(&repository);
    open(&context);
    for (id, index, amount) in [(FIRST, 2, 1), (FIRST, 3, 2), (SECOND, 2, 1), (FIRST, 0, 3), (SECOND, 0, 7)] {
        act(&context, id, PlayerTradeRequest::Offer { index, amount }, 103).unwrap();
    }
    assert_eq!(economy(&repository), before);
    let offer = packet(&context, SECOND, 0x080F);
    assert_eq!(offer.len(), 20);
    assert_eq!(u16::from_le_bytes(offer[2..4].try_into().unwrap()), 1201);
    assert_eq!((offer[9], offer[10], offer[11]), (0, 1, 7));
    assert_eq!(u16::from_le_bytes(offer[12..14].try_into().unwrap()), 255);
    act(&context, FIRST, PlayerTradeRequest::Confirm, 104).unwrap();
    assert_eq!(economy(&repository), before);
    act(&context, FIRST, PlayerTradeRequest::Lock, 105).unwrap();
    act(&context, FIRST, PlayerTradeRequest::Offer { index: 0, amount: 20 }, 106).unwrap();
    assert_eq!(
        context
            .server
            .state()
            .characters()
            .get(&FIRST)
            .unwrap()
            .game_systems
            .trade
            .as_ref()
            .unwrap()
            .zeny,
        3
    );
    act(&context, SECOND, PlayerTradeRequest::Lock, 107).unwrap();
    act(&context, FIRST, PlayerTradeRequest::Confirm, 108).unwrap();
    assert_eq!(economy(&repository), before);
    act(&context, FIRST, PlayerTradeRequest::Confirm, 109).unwrap();
    assert_eq!(economy(&repository), before);
    act(&context, SECOND, PlayerTradeRequest::Confirm, 110).unwrap();
    assert_closed(&context);
    for (id, expected_zeny, expected_potions) in [(FIRST, 104, 4), (SECOND, 46, 5)] {
        let guard_311 = context.server.state();
        let character = guard_311.characters().get(&id).unwrap();
        let stored: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
        assert_eq!(character.status.zeny, expected_zeny);
        assert_eq!(stored.zeny as u32, expected_zeny);
        assert_eq!(
            character.inventory_iter().find(|(_, item)| item.item_id == 501).unwrap().1.amount,
            expected_potions
        );
        drop(guard_311);
        assert_eq!(packet(&context, id, 0x00F0), vec![0xF0, 0, 0]);
        let wallet = packet(&context, id, 0x00B1);
        assert_eq!(i32::from_le_bytes(wallet[4..8].try_into().unwrap()) as u32, expected_zeny);
    }
    let guard_323 = context.server.state();
    let second = guard_323.characters().get(&SECOND).unwrap();
    let weapon = second.inventory_iter().find(|(_, item)| item.item_id == 1201).unwrap().1;
    assert_eq!(
        (
            weapon.id,
            weapon.unique_id,
            weapon.refine,
            weapon.is_identified,
            weapon.is_damaged,
            [weapon.card0, weapon.card1, weapon.card2, weapon.card3]
        ),
        (10, 987654, 7, false, true, [255, 3, 71, 72])
    );
    drop(guard_323);
    assert_eq!(
        database::required::<i32>(&repository.database.inventory_owners, &10_i32.to_be_bytes()).unwrap(),
        SECOND as i32
    );
    let after = economy(&repository);
    act(&context, FIRST, PlayerTradeRequest::Confirm, 111).unwrap();
    act(&context, SECOND, PlayerTradeRequest::Confirm, 112).unwrap();
    assert_eq!(economy(&repository), after);
}

#[test]
fn requested_trade_decline_and_accepted_trade_cancel_restore_clients_without_economic_writes() {
    for accepted in [false, true] {
        let (context, repository) = fixture();
        let before = economy(&repository);
        act(&context, FIRST, PlayerTradeRequest::Request(2_000_001), 100).unwrap();
        if accepted {
            act(&context, SECOND, PlayerTradeRequest::Answer(true), 101).unwrap();
            act(&context, FIRST, PlayerTradeRequest::Offer { index: 3, amount: 2 }, 102).unwrap();
            act(&context, SECOND, PlayerTradeRequest::Offer { index: 0, amount: 7 }, 103).unwrap();
            act(&context, FIRST, PlayerTradeRequest::Lock, 104).unwrap();
            act(&context, FIRST, PlayerTradeRequest::Confirm, 105).unwrap();
            act(&context, SECOND, PlayerTradeRequest::Cancel, 106).unwrap();
            for id in [FIRST, SECOND] {
                assert_eq!(packet(&context, id, 0x00EE), vec![0xEE, 0]);
            }
            assert_eq!(packet(&context, SECOND, 0x00B1)[4..8], 50_i32.to_le_bytes());
        } else {
            act(&context, SECOND, PlayerTradeRequest::Answer(false), 101).unwrap();
            for id in [FIRST, SECOND] {
                assert_eq!(packet(&context, id, 0x01F5)[2], 4);
            }
        }
        assert_closed(&context);
        assert_eq!(economy(&repository), before);
    }
}

#[test]
fn trade_admission_checks_distance_instance_death_map_rules_conversations_and_existing_storage() {
    for case in 0..7 {
        let (context, repository) = fixture();
        match case {
            0 => context.server.state_mut().characters_mut().get_mut(&SECOND).unwrap().x += 3,
            1 => {
                context
                    .server
                    .state_mut()
                    .characters_mut()
                    .get_mut(&SECOND)
                    .unwrap()
                    .map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), 1)
            }
            2 => context.server.state_mut().characters_mut().get_mut(&SECOND).unwrap().status.hp = 0,
            3 => {
                let mut flags = MapFlags::default();
                flags.set(MapFlag::NoTrade, true, &[]).unwrap();
                context.server.map_flag_overrides().insert(("empty".into(), 0), flags);
            }
            4 => {
                context
                    .server
                    .state_mut()
                    .characters_mut()
                    .get_mut(&SECOND)
                    .unwrap()
                    .game_systems
                    .storage_open = true
            }
            5 => context
                .server
                .state_mut()
                .characters_mut()
                .get_mut(&FIRST)
                .unwrap()
                .status
                .known_skills
                .retain(|skill| skill.value != SkillEnum::NvBasic),
            6 => {
                let session = context.server.state().find_session(2_000_001).unwrap();
                let (sender, _receiver) = tokio::sync::mpsc::channel(1);
                session.set_script_handler_channel_sender(sender);
            }
            _ => unreachable!(),
        }
        let before = economy(&repository);
        act(&context, FIRST, PlayerTradeRequest::Request(2_000_001), 100).unwrap();
        assert_closed(&context);
        assert!(packet(&context, FIRST, 0x01F5)[2] < 3);
        assert_eq!(economy(&repository), before);
    }
}

#[test]
fn trade_ticks_cancel_both_sides_on_death_departure_map_restriction_and_idle_timeout() {
    for case in 0..5 {
        let (context, repository) = fixture();
        open(&context);
        act(&context, FIRST, PlayerTradeRequest::Offer { index: 3, amount: 2 }, 102).unwrap();
        let before = economy(&repository);
        let now = match case {
            0 => {
                context.server.state_mut().characters_mut().get_mut(&SECOND).unwrap().status.hp = 0;
                103
            }
            1 => {
                context.server.state_mut().characters_mut().get_mut(&SECOND).unwrap().x += 3;
                103
            }
            2 => {
                context
                    .server
                    .state_mut()
                    .characters_mut()
                    .get_mut(&SECOND)
                    .unwrap()
                    .map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), 1);
                103
            }
            3 => {
                let mut flags = MapFlags::default();
                flags.set(MapFlag::NoTrade, true, &[]).unwrap();
                context.server.map_flag_overrides().insert(("empty".into(), 0), flags);
                103
            }
            4 => 102 + TRADE_IDLE_TIMEOUT_MS,
            _ => unreachable!(),
        };
        context.server.tick_player_trades(&mut *context.server.state_mut(), now);
        assert_closed(&context);
        assert_eq!(economy(&repository), before);
        for id in [FIRST, SECOND] {
            assert_eq!(packet(&context, id, 0x00EE), vec![0xEE, 0]);
        }
    }
}

#[test]
fn trade_stale_wallet_or_item_identity_aborts_both_halves() {
    for changed_item in [false, true] {
        let (context, repository) = fixture();
        open(&context);
        act(&context, FIRST, PlayerTradeRequest::Offer { index: 2, amount: 1 }, 102).unwrap();
        act(&context, SECOND, PlayerTradeRequest::Offer { index: 0, amount: 7 }, 103).unwrap();
        act(&context, FIRST, PlayerTradeRequest::Lock, 104).unwrap();
        act(&context, SECOND, PlayerTradeRequest::Lock, 105).unwrap();
        if changed_item {
            repository
                .database
                .inventories
                .transaction(|tree| {
                    let mut records: Vec<InventoryRecord> = database::tx_required(tree, &FIRST.to_be_bytes())?;
                    records[0].unique_id += 1;
                    database::tx_write(tree, &FIRST.to_be_bytes(), &records)
                })
                .unwrap();
        } else {
            repository
                .database
                .characters
                .transaction(|tree| {
                    let mut record: CharacterRecord = database::tx_required(tree, &SECOND.to_be_bytes())?;
                    record.zeny = 0;
                    database::tx_write(tree, &SECOND.to_be_bytes(), &record)
                })
                .unwrap();
        }
        let before = economy(&repository);
        act(&context, FIRST, PlayerTradeRequest::Confirm, 106).unwrap();
        assert!(act(&context, SECOND, PlayerTradeRequest::Confirm, 107).is_err());
        assert_closed(&context);
        assert_eq!(economy(&repository), before);
    }
}

#[test]
fn accepted_trade_blocks_storage_store_creation_item_use_and_equipment_changes() {
    let (context, repository) = fixture();
    context.server.state_mut().characters_mut().get_mut(&FIRST).unwrap().inventory[0]
        .as_mut()
        .unwrap()
        .is_damaged = false;
    repository
        .database
        .inventories
        .transaction(|tree| {
            let mut records: Vec<InventoryRecord> = database::tx_required(tree, &FIRST.to_be_bytes())?;
            records[0].is_damaged = false;
            database::tx_write(tree, &FIRST.to_be_bytes(), &records)
        })
        .unwrap();
    open(&context);
    let before = economy(&repository);
    for request in [
        ScriptWorldRequest::Container(ContainerRequest::StorageDeposit { index: 2, amount: 1 }),
        ScriptWorldRequest::Container(ContainerRequest::ContainerTransfer {
            source: ItemContainer::Inventory,
            destination: ItemContainer::Cart,
            index: 2,
            amount: 1,
        }),
        ScriptWorldRequest::Store(StoreRequest::PrepareVending { skill_level: 1 }),
        ScriptWorldRequest::Store(StoreRequest::OpenVendingStore(2_000_001)),
    ] {
        assert!(
            context
                .server
                .script_world_service()
                .handle_request(&context.server, &mut *context.server.state_mut(), FIRST, request, 102)
                .is_err()
        );
    }
    let mut first = context.server.state_mut().characters_mut().remove(&FIRST).unwrap();
    assert!(
        context
            .server
            .inventory_service()
            .equip_item(&mut first, crate::server::model::events::game_event::CharacterEquipItem {
                char_id: FIRST,
                index: 0,
                requested_location: Some(EquipmentLocation::HandRight.as_flag())
            })
            .is_none()
    );
    context.server.item_service().use_item_in_state(
        &context.server,
        &mut context.server.state(),
        context.runtime(),
        crate::server::model::events::game_event::CharacterUseItem {
            char_id: FIRST,
            target_char_id: FIRST,
            index: 1,
        },
        &mut first,
    );
    context.server.state_mut().insert_character(first);
    assert_eq!(economy(&repository), before);
}

#[test]
fn no_vending_cell_rejects_vending_only_on_that_cell() {
    let (context, _) = fixture();
    let key = context.server.state().get_character(FIRST).unwrap().map_instance_key.clone();
    let (x, y) = {
        let state = context.server.state();
        let character = state.get_character(FIRST).unwrap();
        (character.x(), character.y())
    };
    let prepare = || {
        context.server.script_world_service().handle_request(
            &context.server,
            &mut *context.server.state_mut(),
            FIRST,
            ScriptWorldRequest::Store(StoreRequest::PrepareVending { skill_level: 1 }),
            102,
        )
    };
    let blocked = "Vending is disabled on this map";
    assert_ne!(prepare().unwrap_err(), blocked);
    let set_cell = |enabled: bool| {
        let state = context.server.state();
        let instance = state.get_map_instance(key.map_name(), key.map_instance()).unwrap();
        let mut map = instance.state_mut();
        let index = map.get_cell_index_of(x, y);
        let mask = CellType::NoVending.as_flag();
        let value = &mut map.cells_mut()[index];
        *value = if enabled { *value | mask } else { *value & !mask };
    };
    set_cell(true);
    assert!(context.server.state().cell_has(&key, x, y, CellType::NoVending));
    assert_eq!(prepare().unwrap_err(), blocked);
    set_cell(false);
    assert_ne!(prepare().unwrap_err(), blocked);
}

#[test]
fn trade_network_router_requires_current_map_socket_and_game_loop_handles_the_action() {
    let (context, _) = fixture();
    let session = context.server.state().find_session(2_000_000).unwrap();
    let socket = session.map_server_socket.as_ref().unwrap().clone();
    let mut bytes = 0xE4_u16.to_le_bytes().to_vec();
    bytes.extend_from_slice(&2_000_001_u32.to_le_bytes());
    let parsed = packets::packets_parser::parse(&bytes, context.server.packetver());
    let (responses, _receiver) = std::sync::mpsc::sync_channel(1);
    let request = crate::server::model::request::Request::new(
        context.server.configuration,
        Some(2_000_000),
        context.server.packetver(),
        socket,
        parsed.as_ref(),
        responses,
        context.client_notification_sender.clone(),
    );
    assert!(crate::server::request_handler::player_trade::handle_raw(&context.server, &request).unwrap());
    let tasks = context.server.pop_task().unwrap();
    assert_eq!(tasks.len(), 1);
    let action = match tasks.into_iter().next().unwrap() {
        GameEvent::PlayerTrade(action) => action,
        _ => panic!("Trade was not routed"),
    };
    context.server.add_to_next_tick(GameEvent::PlayerTrade(action));
    crate::server::Server::game_loop_iteration(&context.server, 100);
    assert_eq!(
        context
            .server
            .state()
            .characters()
            .get(&SECOND)
            .unwrap()
            .game_systems
            .trade
            .as_ref()
            .unwrap()
            .phase,
        PlayerTradePhase::Requested
    );
    let stale = PlayerTradeAction {
        char_id: FIRST,
        account_id: 2_000_000,
        auth_code: session.auth_code + 1,
        request: PlayerTradeRequest::Cancel,
    };
    assert!(
        context
            .server
            .handle_player_trade(&mut *context.server.state_mut(), stale, 101)
            .is_err()
    );
    assert!(
        context
            .server
            .state()
            .characters()
            .get(&FIRST)
            .unwrap()
            .game_systems
            .trade
            .is_some()
    );
}

#[test]
fn trade_commit_rechecks_configured_stack_limits_after_an_intervening_inventory_change() {
    let (context, repository) = fixture();
    repository
        .database
        .items
        .transaction(|tree| {
            let mut item: crate::repository::model::item_model::ItemModel = database::tx_required(tree, &501_i32.to_be_bytes())?;
            item.stack_amount = Some(8);
            item.stack_inventory = Some(1);
            database::tx_write(tree, &501_i32.to_be_bytes(), &item)
        })
        .unwrap();
    open(&context);
    act(&context, FIRST, PlayerTradeRequest::Offer { index: 3, amount: 2 }, 102).unwrap();
    act(&context, FIRST, PlayerTradeRequest::Lock, 103).unwrap();
    act(&context, SECOND, PlayerTradeRequest::Lock, 104).unwrap();
    repository
        .database
        .inventories
        .transaction(|tree| {
            let mut records: Vec<InventoryRecord> = database::tx_required(tree, &SECOND.to_be_bytes())?;
            records[0].amount = 7;
            database::tx_write(tree, &SECOND.to_be_bytes(), &records)
        })
        .unwrap();
    let before = economy(&repository);
    act(&context, FIRST, PlayerTradeRequest::Confirm, 105).unwrap();
    assert!(act(&context, SECOND, PlayerTradeRequest::Confirm, 106).is_err());
    assert_closed(&context);
    assert_eq!(economy(&repository), before);
}

#[test]
fn equipment_service_commits_before_live_change_and_rolls_back_when_the_item_owner_changed() {
    for foreign_owner in [false, true] {
        let (context, repository) = fixture();
        let mut character = context.server.state_mut().characters_mut().remove(&FIRST).unwrap();
        character.inventory[0].as_mut().unwrap().is_damaged = false;
        repository
            .database
            .inventories
            .transaction(|tree| {
                let mut records: Vec<InventoryRecord> = database::tx_required(tree, &FIRST.to_be_bytes())?;
                records[0].is_damaged = false;
                database::tx_write(tree, &FIRST.to_be_bytes(), &records)
            })
            .unwrap();
        if foreign_owner {
            repository
                .database
                .inventory_owners
                .transaction(|tree| database::tx_write(tree, &10_i32.to_be_bytes(), &(SECOND as i32)))
                .unwrap();
        }
        let before = economy(&repository);
        let equipped =
            context
                .server
                .inventory_service()
                .equip_item(&mut character, crate::server::model::events::game_event::CharacterEquipItem {
                    char_id: FIRST,
                    index: 0,
                    requested_location: Some(EquipmentLocation::HandRight.as_flag()),
                });
        let records: Vec<InventoryRecord> = database::required(&repository.database.inventories, &FIRST.to_be_bytes()).unwrap();
        if foreign_owner {
            assert!(equipped.is_none());
            assert!(character.status.right_hand_weapon().is_none());
            assert_eq!(character.inventory[0].as_ref().unwrap().equip, 0);
            assert_eq!(economy(&repository), before);
        } else {
            assert!(equipped.is_some());
            assert_eq!(records[0].equip as u64, EquipmentLocation::HandRight.as_flag());
            assert_eq!(character.inventory[0].as_ref().unwrap().equip, records[0].equip);
            assert_eq!(character.status.right_hand_weapon().unwrap().item_id, 1201);
            assert!(context.server.inventory_service().takeoff_equip_item(&mut character, 0).is_some());
            let records: Vec<InventoryRecord> = database::required(&repository.database.inventories, &FIRST.to_be_bytes()).unwrap();
            assert_eq!(records[0].equip, 0);
            assert!(character.status.right_hand_weapon().is_none());
        }
        assert!(context.test_context.received_persistence_events().lock().unwrap().is_empty());
    }
}

#[test]
fn inventory_loading_preserves_distinct_persistent_rows_and_crafted_potion_metadata() {
    let (context, repository) = fixture();
    let records = vec![
        InventoryRecord {
            id: 31,
            item_id: 501,
            amount: 2,
            is_identified: true,
            card0: 255,
            card1: 11,
            card2: 91,
            ..Default::default()
        },
        InventoryRecord {
            id: 32,
            item_id: 501,
            amount: 3,
            is_identified: true,
            card0: 255,
            card1: 11,
            card2: 91,
            ..Default::default()
        },
        InventoryRecord {
            id: 33,
            item_id: 501,
            amount: 4,
            is_identified: true,
            ..Default::default()
        },
    ];
    repository
        .database
        .seed(
            &SeedData {
                inventories: vec![CharacterInventory {
                    char_id: FIRST as i32,
                    items: records.clone(),
                }],
                ..Default::default()
            },
            true,
        )
        .unwrap();
    let mut character = context.server.state_mut().characters_mut().remove(&FIRST).unwrap();
    context
        .server
        .inventory_service()
        .reload_inventory(context.runtime(), FIRST, &mut character);
    let loaded: Vec<_> = character
        .inventory_iter()
        .map(|(_, item)| (item.id, item.amount, item.card0, item.card2))
        .collect();
    assert_eq!(loaded, vec![(31, 2, 255, 91), (32, 3, 255, 91), (33, 4, 0, 0)]);
    let mut increment = character.inventory[0].as_ref().unwrap().clone();
    increment.amount = 1;
    character.add_items(vec![increment]);
    assert_eq!(character.inventory_iter().count(), 3);
    assert_eq!(character.inventory[0].as_ref().unwrap().amount, 3);
}


fn move_to_group(context: &ServerServiceTestContext, id: u32, group: u32) {
    let mut state = context.server.state_mut();
    let account_id = state.characters().get(&id).unwrap().account_id;
    let old = state.find_session(account_id).unwrap();
    let mut session = Session::create_empty(account_id, old.auth_code, old.user_level, context.server.packetver())
        .with_account(AccountSession::new(1, group, 12));
    session.char_id = old.char_id;
    session.map_server_socket = old.map_server_socket.clone();
    state.add_session(account_id, Arc::new(session));
}

fn groups_without_trade_for_group_seven() -> PermissionGroups {
    PermissionGroups::from_json(
        r#"{"groups": [{"id": 0, "name": "Player", "permissions": ["can_trade"]}, {"id": 7, "name": "Muted", "permissions": []}]}"#,
    )
    .unwrap()
}

#[test]
fn a_group_without_can_trade_cannot_open_trades() {
    let (context, _repository) = fixture();
    context.server.state_mut().set_permission_groups(groups_without_trade_for_group_seven());
    move_to_group(&context, FIRST, 7);
    act(&context, FIRST, PlayerTradeRequest::Request(2_000_001), 100).unwrap();
    assert_closed(&context);
}

#[test]
fn trades_cannot_be_opened_with_a_player_whose_group_lacks_can_trade() {
    let (context, _repository) = fixture();
    context.server.state_mut().set_permission_groups(groups_without_trade_for_group_seven());
    move_to_group(&context, SECOND, 7);
    act(&context, FIRST, PlayerTradeRequest::Request(2_000_001), 100).unwrap();
    assert_closed(&context);
}

#[test]
fn groups_with_can_trade_still_trade_normally() {
    let (context, _repository) = fixture();
    context.server.state_mut().set_permission_groups(groups_without_trade_for_group_seven());
    open(&context);
    for id in [FIRST, SECOND] {
        assert!(context.server.state().characters().get(&id).unwrap().game_systems.trade.is_some());
    }
}
