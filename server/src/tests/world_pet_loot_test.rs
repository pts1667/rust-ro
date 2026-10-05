use std::sync::Arc;

use database::model::{CharacterRecord, InventoryRecord};
use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use models::item::{DroppedItem, ItemInstanceAttributes};
use movement::position::Position;
use script_runtime::Host;
use script_sdk::{Function, Request, Value};
use sled::transaction::Transactional;

use super::{fixture, pet};
use crate::repository::{InventoryRepository, SledRepository};
use crate::repository::game_system_repository::GameSystemRepository;
use crate::server::model::events::game_event::{CharacterUseItem, GameEvent, PetLootClaimResult, PetLootDropResult};
use crate::server::model::events::map_event::{MapEvent, PetLootDropRequest, PetLootClaimRequest};
use crate::server::model::game_systems::{CompanionPosition, PetLootCargo, ScriptWorldRequest, PetRequest};
use crate::server::model::map_flags::MapFlag;
use crate::server::model::map_item::{ToMapItem, ToMapItemSnapshot};
use crate::server::script::item_script_handler::ItemEffect;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::server::service::script_world_service::{companion_snapshots, install_state, pet_world_id};

fn equipment() -> InventoryRecord {
    InventoryRecord { item_id: 1201, unique_id: 8_765_432, amount: 1, refine: 7, is_identified: false,
        is_damaged: true, card0: 255, card1: 123, card2: 234, card3: 345, ..Default::default() }
}

fn loot_fixture(capacity: u8) -> (super::super::super::ServerServiceTestContext, Arc<SledRepository>, MapInstanceService) {
    let (context, repository) = fixture();
    repository.database.items.transaction(|tree| {
        for id in [501_i32, 503, 1201] {
            database::tx_write(tree, &id.to_be_bytes(), GlobalConfigService::instance().get_item(id))?;
        }
        Ok(())
    }).unwrap();
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    character.loaded_from_client_side = true;
    let mut systems = character.game_systems.clone();
    systems.pet = Some(pet(1002, character.char_id));
    let saved = repository.save_character_game_systems(character.char_id, &systems).unwrap();
    install_state(&mut character, saved);
    context.server.script_world_service().call(&context.server, &mut character, Function::PetLoot, &[Value::Number(i32::from(capacity))], 100).unwrap();
    character.game_systems.rendered_companions.insert(pet_world_id(77), CompanionPosition { x: 50, y: 50, map_instance: 0 });
    let map = context.server.state().get_map_instance_from_character(&character).unwrap();
    map.state_mut().insert_item(character.to_map_item());
    map.state_mut().cells_mut().fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    let service = MapInstanceService::new(context.client_notification_sender.clone(), GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()), context.server_task_queue.clone());
    let mut actors = vec![character.to_map_item_snapshot()];
    actors.extend(companion_snapshots(&character));
    service.update_mobs_fov(map.state_mut().as_mut(), actors);
    context.server.state_mut().insert_character(character);
    (context, repository, service)
}

fn inventory(context: &super::super::super::ServerServiceTestContext, repository: &SledRepository, items: Vec<InventoryRecord>, slots: i16) {
    (&repository.database.characters, &repository.database.inventories, &repository.database.inventory_owners, &repository.database.metadata)
        .transaction(|(characters, inventories, owners, metadata)| {
            let mut character: CharacterRecord = database::tx_required(characters, &150_000_i32.to_be_bytes())?;
            character.inventory_slots = slots;
            database::tx_write(characters, &150_000_i32.to_be_bytes(), &character)?;
            database::tx_write(inventories, &150_000_i32.to_be_bytes(), &items)?;
            database::tx_write(metadata, b"inventory_id", &100_i32)?;
            for item in &items { database::tx_write(owners, &item.id.to_be_bytes(), &150_000_i32)?; }
            Ok(())
        }).unwrap();
    let items = context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap();
    let mut state = context.server.state_mut();
    let character = state.characters_mut().get_mut(&150_000).unwrap();
    character.inventory.clear();
    character.add_items(items);
}

fn cargo(context: &super::super::super::ServerServiceTestContext, repository: &SledRepository, items: Vec<InventoryRecord>) {
    let mut state = context.server.state_mut();
    let character = state.characters_mut().get_mut(&150_000).unwrap();
    let mut systems = character.game_systems.clone();
    systems.pet_loot = Some(PetLootCargo { pet_id: 77, items, pending_drop: None });
    let saved = repository.save_character_game_systems(150_000, &systems).unwrap();
    install_state(character, saved);
}

fn floor(context: &super::super::super::ServerServiceTestContext, id: u32, owner: Option<u32>) -> DroppedItem {
    let record = equipment();
    let item = DroppedItem { map_item_id: id, item_id: record.item_id, location: Position { x: 50, y: 50, dir: 0 },
        sub_location: Position { x: 3, y: 6, dir: 0 }, owner_id: owner, dropped_at: 99, amount: 1,
        is_identified: record.is_identified, attributes: ItemInstanceAttributes { unique_id: record.unique_id, refine: record.refine,
            damaged: record.is_damaged, cards: [record.card0, record.card1, record.card2, record.card3] }, player_dropped: true };
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    map.state_mut().insert_dropped_item(item);
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().map_view.insert(item.to_map_item());
    item
}

fn map_events(context: &super::super::super::ServerServiceTestContext) -> Vec<MapEvent> {
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let mut events = Vec::new();
    for _ in 0..32 { events.extend(map.pop_task().unwrap_or_default()); }
    events
}

fn server_events(context: &super::super::super::ServerServiceTestContext) -> Vec<GameEvent> {
    let mut events = Vec::new();
    for _ in 0..32 { events.extend(context.server_task_queue.pop().unwrap_or_default()); }
    events
}

fn claim(context: &super::super::super::ServerServiceTestContext, service: &MapInstanceService, target_id: u32) -> PetLootClaimResult {
    context.server.script_world_service().handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::Pet(PetRequest::PetLootTarget(target_id)), 100).unwrap();
    assert!(context.server.state().contains_locked_map_item(target_id));
    assert!(context.server.state().get_character(150_000).unwrap().game_systems.pet_loot.is_none());
    let request: PetLootClaimRequest = map_events(context).into_iter().find_map(|event| match event {
        MapEvent::ClaimPetLoot(request) => Some(request), _ => None,
    }).unwrap();
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    service.claim_pet_loot(map.state_mut().as_mut(), request, 101);
    server_events(context).into_iter().find_map(|event| match event { GameEvent::PetLootClaimResult(result) => Some(result), _ => None }).unwrap()
}

#[test]
fn pet_loot_claim_waits_for_exact_map_custody_blocks_player_pickup_and_commits_once() {
    let (context, repository, map_service) = loot_fixture(30);
    let dropped = floor(&context, 9000, Some(150_000));
    let result = claim(&context, &map_service, dropped.map_item_id);
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    assert!(map.state().get_dropped_item(9000).is_none());
    assert!(map.state().get_map_item(9000).is_some());
    assert_eq!(result.item, Some(dropped));
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    assert!(!context.server.server_service().character_pickup_item(&context.server, context.server.state_mut().as_mut(), &mut character, 9000, &map).unwrap());
    assert!(character.inventory.is_empty());
    context.server.state_mut().insert_character(character);
    context.server.script_world_service().complete_pet_loot(&context.server, context.server.state_mut().as_mut(), result.clone(), 102).unwrap();
    let saved = repository.character_game_systems(150_000).unwrap();
    assert_eq!(saved.pet_loot.as_ref().unwrap().items, vec![equipment()]);
    context.server.script_world_service().complete_pet_loot(&context.server, context.server.state_mut().as_mut(), result, 103).unwrap();
    assert_eq!(repository.character_game_systems(150_000).unwrap().revision, saved.revision);
    let finalizes = map_events(&context).into_iter().filter_map(|event| match event { MapEvent::FinalizePetLoot(finalize) => Some(finalize), _ => None }).collect::<Vec<_>>();
    assert!(finalizes.iter().all(|finalize| finalize.commit));
    for finalize in finalizes { map_service.finalize_pet_loot(map.state_mut().as_mut(), finalize); }
    assert!(map.state().get_map_item(9000).is_none());
    assert!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap().is_empty());
}

#[test]
fn stale_pet_loot_commit_restores_the_exact_floor_item_without_awarding_cargo() {
    let (context, repository, map_service) = loot_fixture(30);
    let dropped = floor(&context, 9000, None);
    let result = claim(&context, &map_service, 9000);
    let mut current = repository.character_game_systems(150_000).unwrap();
    current.font = 3;
    repository.save_character_game_systems(150_000, &current).unwrap();
    assert!(context.server.script_world_service().complete_pet_loot(&context.server, context.server.state_mut().as_mut(), result, 102).is_err());
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    for event in map_events(&context) { if let MapEvent::FinalizePetLoot(finalize) = event {
        assert!(!finalize.commit);
        map_service.finalize_pet_loot(map.state_mut().as_mut(), finalize);
    } }
    assert_eq!(map.state().get_dropped_item(9000), Some(&dropped));
    assert!(repository.character_game_systems(150_000).unwrap().pet_loot.is_none());
}

#[test]
fn pet_loot_configuration_and_consumption_rollback_together_then_return_exact_equipment() {
    let (context, repository, _) = loot_fixture(30);
    let source = InventoryRecord { id: 10, item_id: 501, amount: 2, is_identified: true, ..Default::default() };
    inventory(&context, &repository, vec![source.clone()], 2);
    cargo(&context, &repository, vec![equipment()]);
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let source_item = character.get_item_from_inventory(0).unwrap().clone();
    let action = CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 };
    let mut host = context.server.item_service().prepare_host(&context.server, &character, 501, true);
    context.runtime().block_on(host.invoke(Request::Call { function: Function::PetLoot, arguments: vec![Value::Number(99)] })).unwrap();
    let effects = host.effects.clone();
    assert!(matches!(effects.as_slice(), [ItemEffect::Call { function: Function::PetLoot, .. }]));
    let mut newer = repository.character_game_systems(150_000).unwrap();
    newer.font = 3;
    let committed = repository.save_character_game_systems(150_000, &newer).unwrap();
    assert!(context.server.item_service().finish_item_effects(&context.server, context.runtime(), &mut character, &action, &source_item, effects.clone()).is_err());
    assert_eq!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap()[0].amount, 2);
    assert_eq!(repository.character_game_systems(150_000).unwrap().pet_loot.as_ref().unwrap().items, vec![equipment()]);
    install_state(&mut character, committed);
    repository.database.characters.transaction(|tree| {
        let mut stored: CharacterRecord = database::tx_required(tree, &150_000_i32.to_be_bytes())?;
        stored.inventory_slots = 0;
        database::tx_write(tree, &150_000_i32.to_be_bytes(), &stored)
    }).unwrap();
    assert!(context.server.item_service().finish_item_effects(&context.server, context.runtime(), &mut character, &action, &source_item, effects.clone()).is_err());
    assert_eq!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap()[0].amount, 2);
    assert_eq!(repository.character_game_systems(150_000).unwrap().pet_loot.as_ref().unwrap().items, vec![equipment()]);
    repository.database.characters.transaction(|tree| {
        let mut stored: CharacterRecord = database::tx_required(tree, &150_000_i32.to_be_bytes())?;
        stored.inventory_slots = 2;
        database::tx_write(tree, &150_000_i32.to_be_bytes(), &stored)
    }).unwrap();
    context.server.item_service().finish_item_effects(&context.server, context.runtime(), &mut character, &action, &source_item, effects).unwrap();
    let items = context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap();
    assert_eq!(items.iter().find(|item| item.item_id == 501).unwrap().amount, 1);
    let returned = items.iter().find(|item| item.item_id == 1201).unwrap();
    assert_eq!((returned.unique_id, returned.refine, returned.is_damaged, returned.is_identified,
        [returned.card0, returned.card1, returned.card2, returned.card3]), (8_765_432, 7, true, false, [255, 123, 234, 345]));
    assert!(repository.character_game_systems(150_000).unwrap().pet_loot.is_none());
    assert!(character.game_systems.pet_loot.is_none());
    assert_eq!(character.game_systems.pet_support.as_ref().unwrap().loot.as_ref().unwrap().capacity, 30);
}

#[test]
fn pet_cargo_return_keeps_capacity_overflow_until_floor_reservation_and_offline_ack_commit() {
    let (context, repository, map_service) = loot_fixture(30);
    inventory(&context, &repository, vec![], 0);
    cargo(&context, &repository, vec![equipment()]);
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    {
        let mut state = context.server.state_mut();
        state.runtime_map_flags.entry((character.map_instance_key.map_without_ext(), 0)).or_default().set(MapFlag::NoDrop, true, &[]).unwrap();
    }
    assert!(context.server.script_world_service().return_pet_loot_in_state(&context.server, context.server.state(), &mut character, 100).is_err());
    assert_eq!(repository.character_game_systems(150_000).unwrap().pet_loot.unwrap().items, vec![equipment()]);
    assert!(!map_events(&context).iter().any(|event| matches!(event, MapEvent::PreparePetLootDrop(_))));
    context.server.state_mut().runtime_map_flags.get_mut(&(character.map_instance_key.map_without_ext(), 0)).unwrap().set(MapFlag::NoDrop, false, &[]).unwrap();
    assert!(!context.server.script_world_service().return_pet_loot_in_state(&context.server, context.server.state(), &mut character, 101).unwrap());
    let saved = repository.character_game_systems(150_000).unwrap();
    assert_eq!(saved.pet_loot.as_ref().unwrap().items, vec![equipment()]);
    assert!(saved.pet_loot.as_ref().unwrap().pending_drop.is_some());
    let request: PetLootDropRequest = map_events(&context).into_iter().find_map(|event| match event { MapEvent::PreparePetLootDrop(request) => Some(request), _ => None }).unwrap();
    map_service.prepare_pet_loot_drop(map.state_mut().as_mut(), request);
    let result: PetLootDropResult = server_events(&context).into_iter().find_map(|event| match event { GameEvent::PetLootDropResult(result) => Some(result), _ => None }).unwrap();
    assert!(result.accepted);
    assert!(map.state().map_items().values().all(|item| *item.object_type() != crate::server::model::map_item::MapItemType::DroppedItem));
    context.server.script_world_service().disconnect(&mut character).unwrap();
    context.server.script_world_service().complete_pet_loot_drop(&context.server, context.server.state_mut().as_mut(), result.clone(), 602).unwrap();
    let revision = repository.character_game_systems(150_000).unwrap().revision;
    assert!(repository.character_game_systems(150_000).unwrap().pet_loot.is_none());
    context.server.script_world_service().complete_pet_loot_drop(&context.server, context.server.state_mut().as_mut(), result, 603).unwrap();
    assert_eq!(repository.character_game_systems(150_000).unwrap().revision, revision);
    for event in map_events(&context) { if let MapEvent::FinalizePetLootDrop(finalize) = event {
        assert!(finalize.commit);
        map_service.finalize_pet_loot_drop(map.state_mut().as_mut(), finalize);
    } }
    let drops = map.state().map_items().values().filter_map(|item| map.state().get_dropped_item(item.id()).copied()).collect::<Vec<_>>();
    assert_eq!(drops.len(), 1);
    assert_eq!(drops[0].attributes, ItemInstanceAttributes { unique_id: 8_765_432, refine: 7, damaged: true, cards: [255, 123, 234, 345] });
    assert_eq!(drops[0].owner_id, None);
}

#[test]
fn pet_performance_returns_cargo_to_the_floor_even_when_inventory_has_room() {
    let (context, repository, map_service) = loot_fixture(30);
    cargo(&context, &repository, vec![equipment()]);
    context.server.script_world_service().handle_request(&context.server, context.server.state_mut().as_mut(), 150_000, ScriptWorldRequest::Pet(PetRequest::PetMenu(2)), 100).unwrap();
    assert!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap().is_empty());
    let request = map_events(&context).into_iter().find_map(|event| match event { MapEvent::PreparePetLootDrop(request) => Some(request), _ => None }).unwrap();
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    map_service.prepare_pet_loot_drop(map.state_mut().as_mut(), request);
    let result = server_events(&context).into_iter().find_map(|event| match event { GameEvent::PetLootDropResult(result) => Some(result), _ => None }).unwrap();
    context.server.script_world_service().complete_pet_loot_drop(&context.server, context.server.state_mut().as_mut(), result, 602).unwrap();
    for event in map_events(&context) { if let MapEvent::FinalizePetLootDrop(finalize) = event { map_service.finalize_pet_loot_drop(map.state_mut().as_mut(), finalize); } }
    assert!(repository.character_game_systems(150_000).unwrap().pet_loot.is_none());
    assert!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap().is_empty());
    assert_eq!(map.state().map_items().values().filter(|item| *item.object_type() == crate::server::model::map_item::MapItemType::DroppedItem).count(), 1);
}

#[test]
fn pet_cargo_keeps_a_nonstackable_parcel_whole_until_all_copies_fit_and_allocates_distinct_instances() {
    let (context, repository, _) = loot_fixture(30);
    let mut parcel = equipment();
    parcel.unique_id = 0;
    parcel.amount = 3;
    cargo(&context, &repository, vec![parcel.clone()]);
    inventory(&context, &repository, vec![], 2);
    let before = repository.character_game_systems(150_000).unwrap();
    let returned = repository.return_pet_loot(150_000, before.revision, 100000, 100).unwrap();
    assert_eq!(returned.returned, 0);
    assert_eq!(returned.systems.revision, before.revision);
    assert_eq!(returned.systems.pet_loot.unwrap().items, vec![parcel]);
    assert!(returned.inventory.is_empty());
    inventory(&context, &repository, vec![], 3);
    let returned = repository.return_pet_loot(150_000, before.revision, 100000, 100).unwrap();
    assert_eq!(returned.returned, 1);
    assert!(returned.systems.pet_loot.is_none());
    assert_eq!(returned.inventory.len(), 3);
    let identities = returned.inventory.iter().map(|item| item.unique_id).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(identities.len(), 3);
    assert!(!identities.contains(&0));
    assert!(returned.inventory.iter().all(|item| item.amount == 1 && item.refine == 7 && item.is_damaged
        && [item.card0, item.card1, item.card2, item.card3] == [255, 123, 234, 345]));
}

#[test]
fn pet_loot_ai_walks_to_owned_floor_items_and_returns_full_cargo_without_combat_loyalty_or_accessory_requirements() {
    let (context, repository, map_service) = loot_fixture(1);
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let mut other = floor(&context, 9000, Some(150_001));
    map.state_mut().remove_dropped_item(9000);
    other.attributes.unique_id = 9_999_999;
    map.state_mut().insert_dropped_item(other);
    let mut owned = floor(&context, 9001, Some(150_000));
    map.state_mut().remove_dropped_item(9001);
    owned.location.x = 53;
    map.state_mut().insert_dropped_item(owned);
    {
        let mut state = context.server.state_mut();
        let character = state.characters_mut().get_mut(&150_000).unwrap();
        let mut staged = character.game_systems.clone();
        let pet = staged.pet.as_mut().unwrap();
        pet.intimacy = 1;
        pet.hunger = 0;
        pet.equipped_item = 0;
        let saved = repository.save_character_game_systems(150_000, &staged).unwrap();
        install_state(character, saved);
    }
    let mut acquired = false;
    let mut moved = false;
    for tick in (100..=1000).step_by(40) {
        let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
        context.server.script_world_service().tick_in_state(&context.server, context.server.state(), &mut character, tick).unwrap();
        moved |= character.game_systems.rendered_companions[&pet_world_id(77)].x > 50;
        context.server.state_mut().insert_character(character);
        for event in server_events(&context) { if let GameEvent::ScriptWorld(request) = event {
            if let ScriptWorldRequest::Pet(PetRequest::PetLootTarget(id)) = request.request {
                assert_eq!(id, 9001);
                context.server.script_world_service().handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
                    ScriptWorldRequest::Pet(PetRequest::PetLootTarget(id)), tick).unwrap();
            }
        } }
        for event in map_events(&context) { if let MapEvent::ClaimPetLoot(request) = event {
            assert_eq!(request.target_id, 9001);
            map_service.claim_pet_loot(map.state_mut().as_mut(), request, u128::from(tick + 1));
        } }
        for event in server_events(&context) { if let GameEvent::PetLootClaimResult(result) = event {
            assert!(result.item.is_some());
            context.server.script_world_service().complete_pet_loot(&context.server, context.server.state_mut().as_mut(), result, tick + 2).unwrap();
            for event in map_events(&context) { if let MapEvent::FinalizePetLoot(finalize) = event {
                assert!(finalize.commit);
                map_service.finalize_pet_loot(map.state_mut().as_mut(), finalize);
            } }
            acquired = true;
        } }
        if acquired { break; }
    }
    assert!(moved && acquired);
    let inventory = context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap();
    assert_eq!(inventory.len(), 1);
    assert_eq!(inventory[0].unique_id, 8_765_432);
    assert!(repository.character_game_systems(150_000).unwrap().pet_loot.is_none());
    assert_eq!(map.state().get_dropped_item(9000), Some(&other));
    assert!(map.state().get_dropped_item(9001).is_none());
}
