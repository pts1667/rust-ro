use std::sync::Arc;
use std::time::{Duration, Instant};

use database::model::InventoryRecord;
use sled::transaction::Transactional;

use crate::repository::{InventoryRepository, SledRepository};
use crate::server::model::events::game_event::{CharacterUseItem, GameEvent};
use crate::server::model::session::Session;
use crate::server::script::PlayerInput;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::Server;

fn fixture() -> (Arc<Server>, Arc<SledRepository>, Arc<Session>, CharacterUseItem) {
    let (context, repository, mut character) = super::native_payment_tests::fixture(false, false);
    let record = InventoryRecord { id: 10, item_id: 12221, amount: 2, is_identified: true, ..InventoryRecord::default() };
    (&repository.database.inventories, &repository.database.inventory_owners, &repository.database.items).transaction(|(inventories, owners, items)| {
        database::tx_write(inventories, &character.char_id.to_be_bytes(), &vec![record.clone()])?;
        database::tx_write(owners, &10_i32.to_be_bytes(), &(character.char_id as i32))?;
        database::tx_write(items, &12221_i32.to_be_bytes(), GlobalConfigService::instance().get_item(12221))
    }).unwrap();
    character.add_items(context.runtime().block_on(repository.character_inventory_fetch(character.char_id as i32)).unwrap());
    let mut session = Session::create_empty(character.account_id, 0, 0, context.server.packetver());
    session.char_id = Some(character.char_id);
    let session = Arc::new(session);
    context.server.state().add_session(character.account_id, session.clone());
    let action = CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 };
    let server = Arc::new(context.server);
    server.bind_shared();
    server.item_service().use_item(&server, server.runtime(), action.clone(), &mut character);
    server.state_mut().insert_character(character);
    (server, repository, session, action)
}

fn take_completion(server: &Server) -> crate::server::model::events::game_event::ItemScriptComplete {
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        if let Some(tasks) = server.pop_task() {
            for event in tasks {
                if let GameEvent::ItemScriptComplete(completion) = event { return completion; }
                panic!("Unexpected game mutation before the item conversation completed: {event:?}");
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("Item conversation did not complete");
}

#[test]
fn megaphone_input_stages_effects_and_consumes_only_on_main_loop_completion() {
    let (server, repository, session, action) = fixture();
    assert_eq!(server.runtime().block_on(repository.character_inventory_fetch(action.char_id as i32)).unwrap()[0].amount, 2);
    let sender = session.script_handler_channel_sender.lock().unwrap().clone().unwrap();
    server.runtime().block_on(sender.send(PlayerInput::Text("Hello from a compiled item".into()))).unwrap();
    let completion = take_completion(&server);
    assert!(completion.error.is_none(), "{:?}", completion.error);
    assert_eq!(server.runtime().block_on(repository.character_inventory_fetch(action.char_id as i32)).unwrap()[0].amount, 2);
    server.handle_script_event(server.state_mut().as_mut(), GameEvent::ItemScriptComplete(completion), 100).unwrap();
    assert_eq!(server.runtime().block_on(repository.character_inventory_fetch(action.char_id as i32)).unwrap()[0].amount, 1);
    assert!(session.script_handler_channel_sender.lock().unwrap().is_none());
}

#[test]
fn cancelled_item_conversation_keeps_the_original_consumable() {
    let (server, repository, session, action) = fixture();
    session.cancel_script();
    let completion = take_completion(&server);
    assert!(completion.error.is_some());
    server.handle_script_event(server.state_mut().as_mut(), GameEvent::ItemScriptComplete(completion), 100).unwrap();
    assert_eq!(server.runtime().block_on(repository.character_inventory_fetch(action.char_id as i32)).unwrap()[0].amount, 2);
}

#[test]
fn changing_map_rules_during_item_input_preserves_the_source_and_discards_the_announcement() {
    use crate::server::model::map_flags::MapFlag;
    let (server, repository, session, action) = fixture();
    let mut flags = server.state().map_flags_for("empty", 0);
    flags.set(MapFlag::NoItemConsumption, true, &[]).unwrap();
    server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    let sender = session.script_handler_channel_sender.lock().unwrap().clone().unwrap();
    server.runtime().block_on(sender.send(PlayerInput::Text("Blocked announcement".into()))).unwrap();
    let completion = take_completion(&server);
    assert!(completion.error.is_none());
    assert!(server.handle_script_event(server.state_mut().as_mut(), GameEvent::ItemScriptComplete(completion), 100).is_err());
    assert_eq!(server.runtime().block_on(repository.character_inventory_fetch(action.char_id as i32)).unwrap()[0].amount, 2);
    assert!(server.pop_task().is_none());
    assert!(session.script_handler_channel_sender.lock().unwrap().is_none());
}

#[test]
fn replacing_the_item_slot_during_input_prevents_consumption_and_announcement() {
    let (server, repository, session, action) = fixture();
    server.state_mut().characters_mut().get_mut(&action.char_id).unwrap().inventory[0].as_mut().unwrap().unique_id = 5;
    let sender = session.script_handler_channel_sender.lock().unwrap().clone().unwrap();
    server.runtime().block_on(sender.send(PlayerInput::Text("Stale message".into()))).unwrap();
    let completion = take_completion(&server);
    assert!(server.handle_script_event(server.state_mut().as_mut(), GameEvent::ItemScriptComplete(completion), 100).is_err());
    assert_eq!(server.runtime().block_on(repository.character_inventory_fetch(action.char_id as i32)).unwrap()[0].amount, 2);
    assert!(server.pop_task().is_none());
}
