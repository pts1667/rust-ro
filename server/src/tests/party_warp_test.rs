use std::sync::{Arc, Mutex};

use database::model::InventoryRecord;
use models::enums::cell::CellType;
use models::enums::EnumWithMaskValueU16;
use script_sdk::{Function, Request, Value};
use sled::transaction::Transactional;

use crate::repository::{InventoryRepository, SledRepository};
use crate::repository::game_system_repository::GameSystemRepository;
use crate::server::model::events::game_event::{CharacterUseItem, GameEvent, ScriptPartyWarp};
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::session::Session;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::party_warp_service::party_warp_request;

fn fixture() -> (super::ServerServiceTestContext, Arc<SledRepository>, u32) {
    let (context, repository) = super::world_party_tests::fixture();
    let created = repository.create_party(150_000, "Warp Party".into(), false, false).unwrap();
    let party_id = created.party.unwrap().id;
    let joined = repository.join_party(150_001, party_id, 150_000).unwrap();
    for (id, systems) in joined.member_states {
        context.server.state_mut().characters_mut().get_mut(&id).unwrap().game_systems = systems;
    }
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    map.state_mut().cells_mut().fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    (context, repository, party_id)
}

fn event(party_id: u32, map: &str) -> ScriptPartyWarp {
    ScriptPartyWarp { char_id: 150_000, party_id, map: map.into(), x: 50, y: 50,
        source_map: None, range_x: 0, range_y: 0 }
}

#[test]
fn party_warp_parser_preserves_the_explicit_party_source_map_and_scatter_radius() {
    let (context, _, party_id) = fixture();
    let parsed = party_warp_request(context.server.state().get_character(150_000).unwrap(),
        &["RandomAll".into(), 0.into(), 0.into(), (party_id as i32).into(), "EMPTY.gat".into(), 3.into(), 3.into()]).unwrap();
    assert_eq!(parsed.party_id, party_id);
    assert_eq!(parsed.source_map.as_deref(), Some("empty"));
    assert_eq!((parsed.range_x, parsed.range_y), (3, 3));
    assert!(party_warp_request(context.server.state().get_character(150_000).unwrap(), &["empty".into(), (-1).into(), 0.into()]).is_err());
}

#[test]
fn random_party_warp_uses_one_center_and_scatter_within_the_existing_instance() {
    let (context, _, party_id) = fixture();
    let mut request = event(party_id, "RandomAll");
    request.range_x = 3; request.range_y = 3; request.source_map = Some("empty".into());
    let warps = context.server.plan_party_warp(context.server.state_mut().as_mut(), &request).unwrap();
    assert_eq!(warps.len(), 2);
    let leader = warps.iter().find(|warp| warp.char_id == 150_000).unwrap();
    let member = warps.iter().find(|warp| warp.char_id == 150_001).unwrap();
    assert!(leader.x.abs_diff(member.x) <= 3 && leader.y.abs_diff(member.y) <= 3);
    assert!(warps.iter().all(|warp| warp.map == "empty" && warp.destination_instance == Some(0)));
    request.range_x = 0; request.range_y = 0;
    let warps = context.server.plan_party_warp(context.server.state_mut().as_mut(), &request).unwrap();
    assert_eq!((warps[0].x, warps[0].y), (warps[1].x, warps[1].y));
}

#[test]
fn party_warp_respects_origin_rules_dead_members_and_source_instance_filters() {
    let (context, _, party_id) = fixture();
    let mut flags = MapFlags::default(); flags.set(MapFlag::NoReturn, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    let mut request = event(party_id, "RandomAll");
    assert_eq!(context.server.plan_party_warp(context.server.state_mut().as_mut(), &request).unwrap().iter().map(|warp| warp.char_id).collect::<Vec<_>>(), vec![150_000]);
    assert!(context.server.plan_party_warp(context.server.state_mut().as_mut(), &event(party_id, "empty")).unwrap().is_empty());
    let mut flags = MapFlags::default(); flags.set(MapFlag::NoWarp, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    assert!(context.server.plan_party_warp(context.server.state_mut().as_mut(), &request).is_err());
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), MapFlags::default());
    request.source_map = Some("empty".into());
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().map_instance_key = MapInstanceKey::new("empty".into(), 7);
    assert_eq!(context.server.plan_party_warp(context.server.state_mut().as_mut(), &request).unwrap().len(), 1);
    let mut state = context.server.state_mut();
    let member = state.characters_mut().get_mut(&150_001).unwrap();
    member.map_instance_key = MapInstanceKey::new("empty".into(), 0); member.status.hp = 0;
    assert_eq!(context.server.plan_party_warp(state.as_mut(), &request).unwrap().len(), 1);
}

#[test]
fn party_warp_supports_leader_individual_save_points_and_each_members_random_map() {
    let (context, _, party_id) = fixture();
    for id in [150_000, 150_001] {
        let mut state = context.server.state_mut();
        let member = state.characters_mut().get_mut(&id).unwrap();
        member.save_map = "empty.gat".into(); member.save_x = if id == 150_000 { 15 } else { 25 }; member.save_y = 20;
    }
    let leader = context.server.plan_party_warp(context.server.state_mut().as_mut(), &event(party_id, "Leader")).unwrap();
    assert_eq!(leader.len(), 1); assert_eq!(leader[0].char_id, 150_001); assert_eq!((leader[0].x, leader[0].y), (50, 50));
    let saved = context.server.plan_party_warp(context.server.state_mut().as_mut(), &event(party_id, "SavePoint")).unwrap();
    assert!(saved.iter().all(|warp| (warp.x, warp.y) == (15, 20)));
    let individual = context.server.plan_party_warp(context.server.state_mut().as_mut(), &event(party_id, "SavePointAll")).unwrap();
    assert_eq!(individual.iter().find(|warp| warp.char_id == 150_001).map(|warp| (warp.x, warp.y)), Some((25, 20)));
    assert_eq!(context.server.plan_party_warp(context.server.state_mut().as_mut(), &event(party_id, "Random")).unwrap().len(), 2);
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    map.state_mut().cells_mut().fill(0);
    assert!(context.server.plan_party_warp(context.server.state_mut().as_mut(), &event(party_id, "RandomAll")).is_err());
}

#[test]
fn compiled_giant_fly_wing_commits_source_then_routes_filtered_party_warps() {
    let (context, repository, party_id) = fixture();
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let record = InventoryRecord { id: 10, item_id: 12212, amount: 2, is_identified: true, unique_id: 9001, ..InventoryRecord::default() };
    (&repository.database.inventories, &repository.database.inventory_owners, &repository.database.items).transaction(|(inventories, owners, items)| {
        database::tx_write(inventories, &character.char_id.to_be_bytes(), &vec![record.clone()])?;
        database::tx_write(owners, &10_i32.to_be_bytes(), &(character.char_id as i32))?;
        database::tx_write(items, &12212_i32.to_be_bytes(), GlobalConfigService::instance().get_item(12212))
    }).unwrap();
    character.add_items(context.runtime().block_on(repository.character_inventory_fetch(character.char_id as i32)).unwrap());
    let host = context.server.item_service().prepare_host(&context.server, &character, 12212, true);
    let (host, outcome) = context.runtime().block_on(crate::tests::common::test_script_vm().execute(host, "run_item", 12212));
    outcome.unwrap();
    let action = CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 };
    let source = character.get_item_from_inventory(0).unwrap().clone();
    context.server.item_service().finish_item_effects_in_state(&context.server, context.server.state(), context.runtime(), &mut character, &action, &source, host.effects).unwrap();
    let saved: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!((saved[0].amount, saved[0].unique_id), (1, 9001));
    context.server.state_mut().insert_character(character);
    let requests = context.server.pop_task().unwrap();
    let request = requests.into_iter().find_map(|event| if let GameEvent::ScriptPartyWarp(request) = event { Some(request) } else { None }).unwrap();
    assert_eq!(request.party_id, party_id); assert_eq!(request.source_map.as_deref(), Some("empty"));
    assert_eq!((request.range_x, request.range_y), (3, 3));
    context.server.handle_script_event(context.server.state_mut().as_mut(), GameEvent::ScriptPartyWarp(request), 100).unwrap();
    let warps = context.server.pop_task().unwrap();
    assert_eq!(warps.iter().filter(|event| matches!(event, GameEvent::ScriptWarp(_))).count(), 2);
}

#[test]
fn giant_fly_wing_rejects_missing_party_or_missing_local_living_peer_before_consumption() {
    let (context, repository, _) = fixture();
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let record = InventoryRecord { id: 10, item_id: 12212, amount: 2, is_identified: true, ..InventoryRecord::default() };
    (&repository.database.inventories, &repository.database.inventory_owners, &repository.database.items).transaction(|(inventories, owners, items)| {
        database::tx_write(inventories, &character.char_id.to_be_bytes(), &vec![record.clone()])?;
        database::tx_write(owners, &10_i32.to_be_bytes(), &(character.char_id as i32))?;
        database::tx_write(items, &12212_i32.to_be_bytes(), GlobalConfigService::instance().get_item(12212))
    }).unwrap();
    character.add_items(context.runtime().block_on(repository.character_inventory_fetch(character.char_id as i32)).unwrap());
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().status.hp = 0;
    let action = CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 };
    context.server.item_service().use_item_in_state(&context.server, context.server.state(), context.runtime(), action.clone(), &mut character);
    assert!(context.server.pop_task().is_none());
    assert_eq!(character.get_item_from_inventory(0).unwrap().amount, 2);
    character.game_systems.party_id = 0;
    context.server.item_service().use_item_in_state(&context.server, context.server.state(), context.runtime(), action, &mut character);
    let saved: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!(saved[0].amount, 2); assert!(context.server.pop_task().is_none());
}

#[test]
fn npc_character_identifier_queries_use_named_live_characters_and_zero_for_unknown_names() {
    let (context, _, party_id) = fixture();
    let character = context.server.state().get_character(150_000).unwrap();
    let account = character.account_id;
    let mut session = Session::create_empty(account, 0, 0, context.server.packetver()); session.char_id = Some(150_000);
    context.server.state().add_session(account, Arc::new(session));
    for (kind, name, expected) in [(0, None, 150_000), (1, Some("Party Member"), party_id), (3, Some("Party Member"), 2_000_001), (0, Some("Absent"), 0)] {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let mut arguments = vec![Value::Number(kind)]; arguments.extend(name.map(Value::from));
        context.server.script_service().handle_request(&context.server, context.server.state_mut().as_mut(), crate::server::script::ScriptRequest {
            char_id: 150_000, account_id: account, npc_id: 77, npc_entry: 77, map_instance: 0, generation: 0, background: false,
            request: Request::Call { function: Function::GetCharacterId, arguments }, response: Arc::new(Mutex::new(Some(sender))),
        });
        assert_eq!(context.runtime().block_on(receiver).unwrap().unwrap().number_value().unwrap(), expected as i32);
    }
}
