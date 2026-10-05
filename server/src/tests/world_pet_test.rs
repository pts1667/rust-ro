use std::sync::Arc;

use models::enums::bonus::BonusType;
use models::enums::status::StatusTypes;
use models::enums::EnumWithNumberValue;
use packets::packets::{Packet, PacketZcStatusValues};
use script_runtime::Host;
use script_sdk::{Function, Request, Value};
use sled::transaction::Transactional;

use super::{fixture, request};
use crate::repository::InventoryRepository;
use crate::server::model::events::game_event::{GameEvent, PetCaptureClaimResult};
use crate::server::model::events::map_event::{MapEvent, MobDamage};
use crate::server::model::game_systems::{PetRecord, ScriptWorldRequest};
use crate::server::script::item_script_handler::ItemScriptHost;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::server::service::script_world_service::{advance_pet_hunger, pet_information, pet_script_state, world_data};
use crate::repository::game_system_repository::GameSystemRepository;
use crate::server::service::status_service::StatusService;
use crate::server::state::map_instance::MobSpawnTrack;
use crate::server::model::map_flags::MapFlag;
use crate::server::service::status_effect_service::StatusEffectService;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use crate::server::service::script_world_service::{ScriptWorldService, pet_world_id};
use crate::server::model::game_systems::CompanionPosition;
use models::enums::element::Element;
use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;

#[path = "world_pet_loot_test.rs"]
mod loot_tests;

fn pet(class: u16, owner: u32) -> PetRecord {
    let definition = world_data().pets.iter().find(|pet| pet.class_id == class).unwrap();
    PetRecord { id: 77, owner_char_id: owner, class_id: class, name: definition.name.clone(), level: definition.level,
        egg_item_id: definition.egg_item, egg_inventory_id: 0, intimacy: 910, hunger: 73, equipped_item: 0,
        next_hunger_at: 60000, incubating: false, renamed: true }
}

#[test]
fn active_pet_queries_use_real_owner_context_in_both_npc_and_item_hosts() {
    let (context, _) = fixture();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    source.game_systems.pet = Some(pet(1002, source.char_id));
    source.refresh_script_context();
    let snapshot = pet_script_state(source.game_systems.pet.as_ref().unwrap());
    let mut host = ItemScriptHost::bonuses(source.status.clone(), 0);
    for field in 0..=10 {
        let expected = pet_information(Some(&snapshot), field);
        assert_eq!(context.server.script_world_service().call(&context.server, &mut source, Function::GetPetInfo,
            &[Value::Number(field), Value::Number(150_000)], 100).unwrap(), expected);
        assert_eq!(context.runtime().block_on(host.invoke(Request::Call { function: Function::GetPetInfo,
            arguments: vec![Value::Number(field)] })).unwrap(), expected);
    }
    source.game_systems.pet.as_mut().unwrap().incubating = true;
    source.refresh_script_context();
    assert_eq!(context.server.script_world_service().call(&context.server, &mut source, Function::GetPetInfo,
        &[Value::Number(2)], 100).unwrap(), Value::String("null".into()));
    assert_eq!(context.server.script_world_service().call(&context.server, &mut source, Function::GetPetInfo,
        &[Value::Number(3)], 100).unwrap(), Value::Number(0));
    assert_eq!(context.server.script_world_service().call(&context.server, &mut source, Function::GetPetInfo,
        &[Value::Number(2), Value::Number(999)], 100).unwrap(), Value::String("null".into()));
}

#[test]
fn all_embedded_pet_bonus_scripts_execute_as_wasm_and_poring_bonus_tracks_loyalty_and_incubation() {
    let (context, _) = fixture();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let vm = crate::tests::common::test_script_vm();
    let mut failures = Vec::new();
    for definition in &world_data().pets {
        source.game_systems.pet = Some(pet(definition.class_id, source.char_id));
        source.refresh_script_context();
        let host = ItemScriptHost::bonuses(source.status.clone(), 0);
        let (host, result) = context.runtime().block_on(vm.execute(host, "run_pet", u32::from(definition.class_id)));
        if result.is_err() || host.error.is_some() {
            failures.push(format!("Pet {}: {:?}, host: {:?}", definition.class_id, result, host.error));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    source.game_systems.pet = Some(pet(1002, source.char_id));
    source.game_systems.pet.as_mut().unwrap().intimacy = 909;
    source.refresh_script_context();
    let neutral = StatusService::instance().to_snapshot(&source.status);
    source.game_systems.pet.as_mut().unwrap().intimacy = 910;
    source.refresh_script_context();
    let loyal = StatusService::instance().to_snapshot(&source.status);
    assert_eq!(loyal.luk(), neutral.luk() + 2);
    assert!(loyal.bonuses().iter().any(|bonus| matches!(bonus.bonus(), BonusType::Crit(value) if *value == 1.0)));
    source.game_systems.pet.as_mut().unwrap().incubating = true;
    source.refresh_script_context();
    assert_eq!(StatusService::instance().to_snapshot(&source.status).luk(), neutral.luk());
}

#[test]
fn pet_hunger_reaching_zero_waits_for_the_next_interval_before_starvation_and_then_ticks_every_twenty_seconds() {
    let definition = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap();
    let mut pet = pet(1002, 150_000);
    pet.hunger = definition.fullness;
    pet.next_hunger_at = 100;
    let before = pet.clone();
    assert!(!advance_pet_hunger(&mut pet, definition, 99));
    assert_eq!(pet, before);
    assert!(advance_pet_hunger(&mut pet, definition, 100));
    assert_eq!((pet.hunger, pet.intimacy, pet.next_hunger_at), (0, 910, 100 + definition.hungry_delay));
    let next = pet.next_hunger_at;
    assert!(advance_pet_hunger(&mut pet, definition, next));
    assert_eq!((pet.hunger, pet.intimacy, pet.next_hunger_at), (0, 910 + definition.intimacy_hungry, next + 20000));
    assert!(!advance_pet_hunger(&mut pet, definition, next + 19999));
    assert!(advance_pet_hunger(&mut pet, definition, next + 20000));
    assert_eq!(pet.intimacy, 910 + 2 * definition.intimacy_hungry);
    pet.incubating = true;
    let before = pet.clone();
    assert!(!advance_pet_hunger(&mut pet, definition, next + 40000));
    assert_eq!(pet, before);
}

#[test]
fn pet_starvation_persists_the_loyalty_loss_and_updates_the_client_stats() {
    let (context, repository) = fixture();
    let mut character = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let mut systems = character.game_systems.clone();
    let mut active = pet(1002, character.char_id);
    active.hunger = 0;
    active.next_hunger_at = 100;
    systems.pet = Some(active);
    let saved = repository.save_character_game_systems(character.char_id, &systems).unwrap();
    crate::server::service::script_world_service::install_state(&mut character, saved);
    assert_eq!(StatusService::instance().to_snapshot(&character.status).bonus_luk(), 2);
    let service = context.server.script_world_service();
    assert!(service.tick_in_state(&context.server, context.server.state(), &mut character, 100).unwrap());
    assert_eq!(character.game_systems.pet.as_ref().unwrap().intimacy, 905);
    assert_eq!(repository.character_game_systems(character.char_id).unwrap().pet.unwrap().intimacy, 905);
    assert_eq!(StatusService::instance().to_snapshot(&character.status).bonus_luk(), 0);
    let mut expected = PacketZcStatusValues::new(context.server.packetver());
    expected.set_status_type(StatusTypes::Luk.value() as u32);
    expected.set_default_status(i32::from(character.status.luk));
    expected.set_plus_status(0);
    expected.fill_raw();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        service.drain_notifications().unwrap();
        if context.test_context.received_notification().lock().unwrap().iter().any(|notification| matches!(notification,
            crate::server::model::events::client_notification::Notification::Char(notification)
            if notification.char_id() == character.char_id && notification.serialized_packet().windows(expected.raw.len()).any(|bytes| bytes == expected.raw))) { break; }
        assert!(std::time::Instant::now() < deadline, "Pet loyalty loss did not update client LUK");
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

fn capture_fixture() -> (super::super::ServerServiceTestContext, Arc<crate::repository::SledRepository>, MapInstanceService) {
    let (context, repository) = fixture();
    let definition = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap();
    repository.database.items.transaction(|items| database::tx_write(items, &definition.egg_item.to_be_bytes(),
        GlobalConfigService::instance().get_item(definition.egg_item))).unwrap();
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().loaded_from_client_side = true;
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let mut mob = crate::tests::common::mob_helper::create_mob_at_position(42, "PORING", 50, 50);
    mob.set_hp(10);
    map.state_mut().insert_mob(mob);
    let mut track = MobSpawnTrack::default(0);
    track.increment_spawn();
    map.state_mut().mob_spawns_tracks_mut().insert(0, track);
    let service = MapInstanceService::new(context.client_notification_sender.clone(), GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()), context.server_task_queue.clone());
    (context, repository, service)
}

fn claim(context: &super::super::ServerServiceTestContext, service: &MapInstanceService) -> PetCaptureClaimResult {
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    context.server.script_world_service().call(&context.server, &mut source, Function::Pet, &[Value::Number(1002)], 100).unwrap();
    context.server.state_mut().insert_character(source);
    request(context, 150_000, ScriptWorldRequest::CapturePet(42)).unwrap();
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let request = map.pop_task().unwrap().into_iter().find_map(|event| match event { MapEvent::ClaimPetCapture(request) => Some(request), _ => None }).unwrap();
    service.claim_pet_capture_with_roll(map.state_mut().as_mut(), request, 101, 0);
    context.server_task_queue.pop().unwrap().into_iter().find_map(|event| match event {
        GameEvent::PetCaptureClaimResult(result) => Some(result), _ => None,
    }).unwrap()
}

#[test]
fn pet_egg_commit_waits_for_the_map_claim_and_duplicate_acknowledgements_are_idempotent() {
    let (context, repository, map_service) = capture_fixture();
    let result = claim(&context, &map_service);
    assert!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap().is_empty());
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    assert!(map.state().get_mob(42).is_none());
    assert!(map.state().pending_pet_captures.contains_key(&result.claim_id));
    let world = context.server.script_world_service();
    world.complete_pet_capture(&context.server, context.server.state_mut().as_mut(), result.clone(), 102).unwrap();
    world.complete_pet_capture(&context.server, context.server.state_mut().as_mut(), result, 103).unwrap();
    let inventory = context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap();
    assert_eq!(inventory.len(), 1);
    let definition = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap();
    assert_eq!((inventory[0].item_id, inventory[0].card0), (definition.egg_item, 256));
    assert_eq!(repository.database.game_systems.scan_prefix(b"pet/").count(), 1);
    for event in map.pop_task().unwrap() {
        if let MapEvent::FinalizePetCapture(finalize) = event {
            assert!(finalize.commit);
            map_service.finalize_pet_capture(map.state_mut().as_mut(), finalize);
        }
    }
    assert!(map.state().pending_pet_captures.is_empty());
    assert_eq!(map.state().mob_spawns_tracks()[&0].spawned_amount, 0);
}

#[test]
fn a_pet_egg_capacity_failure_restores_the_claim_without_creating_any_persistent_pet() {
    let (context, repository, map_service) = capture_fixture();
    repository.database.characters.transaction(|characters| {
        let mut character: database::model::CharacterRecord = database::tx_read(characters, &150_000_i32.to_be_bytes())?.unwrap();
        character.inventory_slots = 0;
        database::tx_write(characters, &150_000_i32.to_be_bytes(), &character)
    }).unwrap();
    let result = claim(&context, &map_service);
    assert!(context.server.script_world_service().complete_pet_capture(&context.server, context.server.state_mut().as_mut(), result, 102).is_err());
    assert!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap().is_empty());
    assert_eq!(repository.database.game_systems.scan_prefix(b"pet/").count(), 0);
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    for event in map.pop_task().unwrap() {
        if let MapEvent::FinalizePetCapture(finalize) = event {
            assert!(!finalize.commit);
            map_service.finalize_pet_capture(map.state_mut().as_mut(), finalize);
        }
    }
    assert_eq!(map.state().get_mob(42).unwrap().hp(), 10);
    assert_eq!(map.state().mob_spawns_tracks()[&0].spawned_amount, 1);
    assert!(context.server.state().get_character(150_000).unwrap().game_systems.pending_pet_capture.is_none());
}

#[test]
fn disabling_pet_capture_between_map_claim_and_egg_commit_restores_the_exact_live_monster() {
    let (context, repository, map_service) = capture_fixture();
    let result = claim(&context, &map_service);
    context.server.map_flag_call(context.server.state_mut().as_mut(), 150_000, Function::SetMapFlag,
        &[Value::String("empty".into()), Value::Number(MapFlag::NoPetCapture as i32)]).unwrap();
    context.server.script_world_service().complete_pet_capture(&context.server, context.server.state_mut().as_mut(), result, 102).unwrap();
    assert!(context.runtime().block_on(repository.character_inventory_fetch(150_000)).unwrap().is_empty());
    assert_eq!(repository.database.game_systems.scan_prefix(b"pet/").count(), 0);
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    for event in map.pop_task().unwrap() {
        if let MapEvent::FinalizePetCapture(finalize) = event {
            assert!(!finalize.commit);
            map_service.finalize_pet_capture(map.state_mut().as_mut(), finalize);
        }
    }
    assert_eq!(map.state().get_mob(42).unwrap().hp(), 10);
    assert_eq!(map.state().mob_spawns_tracks()[&0].spawned_amount, 1);
    assert!(context.server.state().get_character(150_000).unwrap().game_systems.pending_pet_capture.is_none());
}

fn support_guest(requests: Vec<Request>) -> script_runtime::WasmRuntime {
    let mut data = String::new();
    let mut calls = String::new();
    for (index, request) in requests.into_iter().enumerate() {
        let bytes = serde_json::to_vec(&request).unwrap();
        let offset = index * 1024;
        assert!(bytes.len() < 1024);
        let escaped = bytes.iter().map(|byte| format!("\\{byte:02x}")).collect::<String>();
        data.push_str(&format!("(data (i32.const {offset}) \"{escaped}\")"));
        calls.push_str(&format!("i32.const {offset} i32.const {} i32.const 32768 i32.const 8192 call $invoke drop ", bytes.len()));
    }
    let wasm = format!("(module (import \"rust_ro\" \"invoke\" (func $invoke (param i32 i32 i32 i32) (result i32)))
        (memory (export \"memory\") 1) {data}
        (func (export \"script_abi\") (result i32) i32.const {})
        (func (export \"run_pet_support\") (param i32) (result i32) {calls} i32.const 0))", script_sdk::ABI_VERSION);
    script_runtime::WasmRuntime::from_bytes(wasm.as_bytes(), script_runtime::Limits::default()).unwrap()
}

#[test]
fn compiled_pet_support_calls_install_real_bonuses_recovery_and_skills_and_reject_ignored_host_errors() {
    let (context, _) = fixture();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let mut active = pet(1002, source.char_id);
    active.equipped_item = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap().equip_item;
    source.game_systems.pet = Some(active.clone());
    source.refresh_script_context();
    let vm = support_guest(vec![
        Request::Call { function: Function::Bonus, arguments: vec![Value::from("bStr"), Value::Number(2)] },
        Request::Call { function: Function::PetSkillBonus, arguments: vec![Value::from("bMaxHPrate"), Value::Number(100), Value::Number(1), Value::Number(1)] },
        Request::Call { function: Function::PetRecovery, arguments: vec![Value::from("SC_POISON"), Value::Number(2)] },
        Request::Call { function: Function::PetSkillSupport, arguments: vec![Value::from("AL_HEAL"), Value::Number(1), Value::Number(3), Value::Number(40), Value::Number(100)] },
    ]);
    let host = crate::server::service::script_world_service::PetSupportHost::new(source.status.clone(), active.clone(), Default::default(), 100);
    let (host, result) = context.runtime().block_on(vm.execute(host, "run_pet_support", 1002));
    result.unwrap();
    let support = host.into_support().unwrap();
    assert_eq!(support.base_bonuses, vec![BonusType::Str(2)]);
    assert_eq!(support.bonus.as_ref().unwrap().next_at, Some(1100));
    assert_eq!(support.recovery.as_ref().unwrap().kind, StatusChangeKind::Poison);
    assert_eq!((support.skill.as_ref().unwrap().skill_id, support.skill.as_ref().unwrap().next_at), (28, Some(3100)));
    source.game_systems.pet_support = Some(support);
    source.refresh_script_context();
    assert_eq!(StatusService::instance().to_snapshot(&source.status).str(), source.status.str + 2);
    let invalid = support_guest(vec![Request::Call { function: Function::PetSkillSupport,
        arguments: vec![Value::from("UNKNOWN_PET_SKILL"), Value::Number(1), Value::Number(1), Value::Number(100), Value::Number(100)] }]);
    let host = crate::server::service::script_world_service::PetSupportHost::new(source.status.clone(), active, Default::default(), 100);
    let (host, result) = context.runtime().block_on(invalid.execute(host, "run_pet_support", 1002));
    result.unwrap();
    assert!(host.into_support().is_err());
}

#[test]
fn timed_pet_bonus_recalculates_thresholds_before_the_live_heal_and_is_removed_with_the_accessory() {
    let (context, _) = fixture();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let mut active = pet(1170, source.char_id);
    active.equipped_item = world_data().pets.iter().find(|pet| pet.class_id == 1170).unwrap().equip_item;
    source.game_systems.pet = Some(active);
    source.refresh_script_context();
    let baseline = StatusService::instance().to_snapshot(&source.status);
    source.status.hp = baseline.max_hp() / 2;
    source.status.sp = baseline.max_sp();
    let world = context.server.script_world_service();
    world.call(&context.server, &mut source, Function::PetSkillBonus,
        &[Value::from("bMaxHPrate"), Value::Number(100), Value::Number(1), Value::Number(1)], 100).unwrap();
    world.call(&context.server, &mut source, Function::PetSkillSupport,
        &[Value::from("AL_HEAL"), Value::Number(1), Value::Number(1), Value::Number(25), Value::Number(100)], 100).unwrap();
    world.tick_in_state(&context.server, context.server.state(), &mut source, 1040).unwrap();
    assert_eq!(StatusService::instance().to_snapshot(&source.status).max_hp(), baseline.max_hp());
    world.tick_in_state(&context.server, context.server.state(), &mut source, 1100).unwrap();
    assert_eq!(StatusService::instance().to_snapshot(&source.status).max_hp(), baseline.max_hp() * 2);
    let heal = context.server_task_queue.pop().unwrap().into_iter().find_map(|event| match event {
        GameEvent::ScriptWorld(world) if matches!(world.request, ScriptWorldRequest::HealByCompanion { .. }) => Some(world.request),
        _ => None,
    }).expect("Timed HP bonus did not enable the pet heal threshold");
    let previous_hp = source.status.hp;
    context.server.state_mut().insert_character(source);
    request(&context, 150_000, heal).unwrap();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    assert!(source.status.hp > previous_hp);
    source.game_systems.pet.as_mut().unwrap().equipped_item = 0;
    source.refresh_script_context();
    assert_eq!(StatusService::instance().to_snapshot(&source.status).max_hp(), baseline.max_hp());
    world.tick_in_state(&context.server, context.server.state(), &mut source, 1140).unwrap();
    assert_eq!(source.game_systems.pet_support.as_ref().unwrap().bonus.as_ref().unwrap().next_at, None);
    assert!(!source.game_systems.pet_support.as_ref().unwrap().bonus.as_ref().unwrap().active);
}

#[test]
fn pet_recovery_is_scheduled_from_a_new_status_and_cast_completion_rechecks_map_policy() {
    let (context, _) = fixture();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let mut active = pet(1002, source.char_id);
    active.equipped_item = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap().equip_item;
    source.game_systems.pet = Some(active);
    source.refresh_script_context();
    let baseline = StatusService::instance().to_snapshot(&source.status);
    source.status.hp = baseline.max_hp();
    source.status.sp = baseline.max_sp();
    let world = context.server.script_world_service();
    world.call(&context.server, &mut source, Function::PetRecovery, &[Value::from("SC_POISON"), Value::Number(1)], 100).unwrap();
    assert!(StatusEffectService::start(&context.server, &mut source,
        StatusChangeRequest::guaranteed(StatusChangeKind::Poison, 60000, 1), 200, &context.client_notification_sender).unwrap());
    assert_eq!(source.game_systems.pet_support.as_ref().unwrap().recovery.as_ref().unwrap().next_at, Some(1200));
    world.tick_in_state(&context.server, context.server.state(), &mut source, 1160).unwrap();
    assert_eq!(source.game_systems.pet_support.as_ref().unwrap().recovery.as_ref().unwrap().next_at, Some(1200));
    world.tick_in_state(&context.server, context.server.state(), &mut source, 1200).unwrap();
    let cured = context.server_task_queue.pop().unwrap().into_iter().find_map(|event| match event {
        GameEvent::CharacterEndStatus(status) => Some(status), _ => None,
    }).expect("Recovery timer did not request the actual status cure");
    StatusEffectService::end(&context.server, &mut source, cured.kind, 1200, &context.client_notification_sender);
    assert!(!source.status.has_status_change(StatusChangeKind::Poison));
    world.call(&context.server, &mut source, Function::PetSkillSupport,
        &[Value::from("PR_MAGNIFICAT"), Value::Number(1), Value::Number(1), Value::Number(100), Value::Number(100)], 1200).unwrap();
    world.tick_in_state(&context.server, context.server.state(), &mut source, 2200).unwrap();
    let casting = source.game_systems.pet_support.as_ref().unwrap().casting.as_ref().unwrap().clone();
    assert!(casting.completes_at > 2200);
    context.server.state_mut().insert_character(source);
    context.server.map_flag_call(context.server.state_mut().as_mut(), 150_000, Function::SetMapFlag,
        &[Value::from("empty"), Value::Number(MapFlag::NoSkill as i32)]).unwrap();
    world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::FinishPetSupport(casting.id), casting.completes_at).unwrap();
    let source = context.server.state().get_character(150_000).unwrap();
    assert!(source.game_systems.pet_support.as_ref().unwrap().casting.is_none());
    assert!(!source.status.has_status_change(StatusChangeKind::Magnificat));
    assert!(!context.server_task_queue.pop().unwrap_or_default().into_iter().any(|event| matches!(event,
        GameEvent::CharacterStatusChange(status) if status.request.kind == StatusChangeKind::Magnificat)));
}

fn pet_combat_fixture() -> (super::super::ServerServiceTestContext, Arc<crate::repository::SledRepository>, MapInstanceService, ScriptWorldService) {
    let (context, repository, map_service) = capture_fixture();
    let config = configuration::configuration::PetSupportConfig { status_support: true, attack_support: true,
        require_accessory: false, support_rate: 1000, ..Default::default() };
    let world = ScriptWorldService::new(context.client_notification_sender.clone(), repository.clone(), GlobalConfigService::instance())
        .with_pet_configuration(config);
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let active = pet(1170, source.char_id);
    source.game_systems.rendered_companions.insert(pet_world_id(active.id), CompanionPosition { x: 49, y: 50, map_instance: 0 });
    source.game_systems.pet = Some(active);
    source.game_systems.last_companion_map = source.current_map_name().clone();
    source.refresh_script_context();
    context.server.state_mut().insert_character(source);
    let instance = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    instance.state_mut().cells_mut().fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    (context, repository, map_service, world)
}

fn finish_pet_cast(context: &super::super::ServerServiceTestContext, world: &ScriptWorldService) -> u64 {
    let casting = context.server.state().get_character(150_000).unwrap().game_systems.pet_support.as_ref()
        .and_then(|support| support.casting.as_ref()).cloned();
    if let Some(casting) = casting {
        world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
            ScriptWorldRequest::FinishPetSupport(casting.id), casting.completes_at).unwrap();
        casting.completes_at
    } else { 3000 }
}

#[test]
fn pet_target_hooks_obey_the_config_and_loyalty_and_normal_damage_uses_the_real_pet_actor() {
    let (context, _, map_service, world) = pet_combat_fixture();
    world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::PetCombatTarget { target_id: 42, retaliation: true }, 100).unwrap();
    let actor = pet_world_id(77);
    assert!(context.server.state().get_character(150_000).unwrap().game_systems.companion_commands.get(&actor).is_none());
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().game_systems.pet.as_mut().unwrap().intimacy = 899;
    world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::PetCombatTarget { target_id: 42, retaliation: false }, 100).unwrap();
    assert!(context.server.state().get_character(150_000).unwrap().game_systems.companion_commands.get(&actor).is_none());
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().game_systems.pet.as_mut().unwrap().intimacy = 910;
    world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::PetCombatTarget { target_id: 42, retaliation: false }, 100).unwrap();
    assert_eq!(context.server.state().get_character(150_000).unwrap().game_systems.companion_commands[&actor].target, Some(42));
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let mut landed = None;
    for tick in (3000..60000).step_by(3000) {
        let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
        world.tick_in_state(&context.server, context.server.state(), &mut source, tick).unwrap();
        context.server.state_mut().insert_character(source);
        if let Some(damage) = map.pop_task().unwrap_or_default().into_iter().find_map(|event| match event {
            MapEvent::MobDamage(MobDamage { damage }) if damage.landed && damage.damage > 0 => Some(damage), _ => None,
        }) { landed = Some(damage); break; }
    }
    let damage = landed.expect("The real pet actor never dealt a normal attack");
    assert_eq!((damage.attacker_id, damage.credit_id, damage.target_id, damage.skill_id), (actor, 150_000, 42, 0));
    map_service.mob_being_attacked(map.state_mut().as_mut(), damage, map.task_queue(), damage.attacked_at);
    assert_eq!(map.state().get_mob(42).unwrap().hp(), 0);
    let kill = context.server_task_queue.pop().unwrap().into_iter().find_map(|event| match event {
        GameEvent::CharacterKillMonster(kill) => Some(kill), _ => None,
    }).expect("Pet damage did not enter the real kill/reward path");
    assert_eq!((kill.attacker_id, kill.char_id), (actor, 150_000));
    assert!(kill.contributions.iter().any(|contribution| contribution.actor_id == actor && contribution.owner_id == 150_000));
}

#[test]
fn pet_fixed_skill_preserves_elements_and_applies_real_capped_absorption_without_sp_cost() {
    let (context, repository, map_service, world) = pet_combat_fixture();
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    map.state_mut().mobs_mut().get_mut(&42).unwrap().status.set_element(Element::Wind);
    map.state_mut().mobs_mut().get_mut(&42).unwrap().status.set_element_level(4);
    let modifier = crate::server::service::battle_service::BattleService::element_modifier(&Element::Wind, &map.state().get_mob(42).unwrap().status);
    assert!(modifier < 0.0);
    let stored: database::model::CharacterRecord = database::required(&repository.database.characters, &150_000_i32.to_be_bytes()).unwrap();
    let before_sp = context.server.state().get_character(150_000).unwrap().status.sp;
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    world.call(&context.server, &mut source, Function::PetSkillAttack2,
        &[Value::from("NPC_WINDATTACK"), Value::Number(200), Value::Number(2), Value::Number(100), Value::Number(0)], 100).unwrap();
    context.server.state_mut().insert_character(source);
    world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::PetCombatTarget { target_id: 42, retaliation: false }, 100).unwrap();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    world.tick_in_state(&context.server, context.server.state(), &mut source, 3000).unwrap();
    context.server.state_mut().insert_character(source);
    let completion = finish_pet_cast(&context, &world);
    let damage = map.pop_task().unwrap().into_iter().find_map(|event| match event { MapEvent::MobDamage(MobDamage { damage }) => Some(damage), _ => None }).unwrap();
    assert_eq!((damage.attacker_id, damage.credit_id, damage.damage), (pet_world_id(77), 150_000, 0));
    assert_eq!(damage.healing, (200.0 * -modifier).floor() as u32);
    map_service.mob_being_attacked(map.state_mut().as_mut(), damage, map.task_queue(), completion as u128);
    let mob = map.state().get_mob(42).unwrap().clone();
    assert_eq!(mob.hp(), 10u32.saturating_add(damage.healing).min(mob.status.max_hp()));
    assert!(mob.actor_damages.is_empty());
    assert!(context.server_task_queue.is_empty());
    assert_eq!(context.server.state().get_character(150_000).unwrap().status.sp, before_sp);
    let after: database::model::CharacterRecord = database::required(&repository.database.characters, &150_000_i32.to_be_bytes()).unwrap();
    assert_eq!(after.sp, stored.sp);
}

#[test]
fn pet_fixed_heaven_drive_places_real_ground_cells_and_hits_each_covered_enemy_once() {
    let (context, _, _, world) = pet_combat_fixture();
    let map = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let mob = crate::tests::common::mob_helper::create_mob_at_position(43, "PORING", 51, 50);
    map.state_mut().insert_mob(mob);
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    world.call(&context.server, &mut source, Function::PetSkillAttack2,
        &[Value::from("WZ_HEAVENDRIVE"), Value::Number(200), Value::Number(1), Value::Number(100), Value::Number(0)], 100).unwrap();
    context.server.state_mut().insert_character(source);
    world.handle_request(&context.server, context.server.state_mut().as_mut(), 150_000,
        ScriptWorldRequest::PetCombatTarget { target_id: 42, retaliation: false }, 100).unwrap();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    world.tick_in_state(&context.server, context.server.state(), &mut source, 3000).unwrap();
    context.server.state_mut().insert_character(source);
    let completion = finish_pet_cast(&context, &world);
    assert!(!map.pop_task().unwrap_or_default().into_iter().any(|event| matches!(event, MapEvent::MobDamage(MobDamage { damage: _ }))));
    context.server.script_skill_service().tick_ground_skills(&context.server, context.server.state(), completion as u128 + 40);
    let damage = map.pop_task().unwrap_or_default().into_iter().filter_map(|event| match event {
        MapEvent::MobDamage(MobDamage { damage }) => Some(damage), _ => None,
    }).collect::<Vec<_>>();
    assert_eq!(damage.len(), 2);
    assert!(damage.iter().all(|damage| damage.attacker_id == pet_world_id(77) && damage.credit_id == 150_000
        && damage.skill_id == 91 && damage.damage > 0));
    context.server.script_skill_service().tick_ground_skills(&context.server, context.server.state(), completion as u128 + 80);
    assert!(!map.pop_task().unwrap_or_default().into_iter().any(|event| matches!(event, MapEvent::MobDamage(MobDamage { damage: _ }))));
}
