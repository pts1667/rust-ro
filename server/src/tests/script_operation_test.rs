use std::sync::Arc;
use std::time::{Duration, Instant};

use database::model::{CharacterRecord, InventoryRecord};
use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::KnownSkill;
use models::status_bonus::{BattleFlag, CombatTrigger};
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use sled::transaction::Transactional;

use crate::repository::{InventoryRepository, SledRepository};
use crate::repository::fame_repository::FameCategory;
use crate::server::model::events::game_event::{CharacterUseItem, CharacterUseSkill, FameChanged, GameEvent, NpcContact};
use crate::server::model::action::Damage;
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::MapItems;
use crate::server::model::script::Script;
use crate::server::model::session::Session;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::PlayerInput;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;
use crate::server::Server;

pub(super) fn identification_fixture(source: bool) -> (super::ServerServiceTestContext, Arc<SledRepository>, Character, usize) {
    let (context, repository, mut character) = super::native_payment_tests::fixture(false, false);
    let mut records = vec![InventoryRecord { id: 11, item_id: 1201, amount: 1, is_identified: false, ..InventoryRecord::default() }];
    if source { records.insert(0, InventoryRecord { id: 10, item_id: 611, amount: 2, is_identified: true, ..InventoryRecord::default() }); }
    (&repository.database.inventories, &repository.database.inventory_owners, &repository.database.items).transaction(|(inventories, owners, items)| {
        database::tx_write(inventories, &character.char_id.to_be_bytes(), &records)?;
        for record in &records {
            database::tx_write(owners, &record.id.to_be_bytes(), &(character.char_id as i32))?;
            database::tx_write(items, &record.item_id.to_be_bytes(), GlobalConfigService::instance().get_item(record.item_id))?;
        }
        Ok(())
    }).unwrap();
    character.add_items(context.runtime().block_on(repository.character_inventory_fetch(character.char_id as i32)).unwrap());
    let selected = usize::from(source);
    if source {
        context.server.item_service().use_item_in_state(&context.server, &mut context.server.state_mut(), context.runtime(), CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 }, &mut character);
    } else {
        character.status.known_skills.push(KnownSkill { value: SkillEnum::McIdentify, level: 1 });
        let id = character.char_id;
        context.server.state_mut().insert_character(character);
        context.server.handle_character_skill(&mut *context.server.state_mut(), CharacterUseSkill { char_id: id, target_id: id, skill_id: SkillEnum::McIdentify.id(), skill_level: 1 }, 100).unwrap();
        character = context.server.state_mut().characters_mut().remove(&id).unwrap();
    }
    assert!(character.pending_item_skill.is_some());
    (context, repository, character, selected)
}

fn records(repository: &SledRepository, character: &Character) -> Vec<InventoryRecord> {
    database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap()
}

#[test]
fn lethal_equipment_proc_costs_commit_before_entering_the_dead_state() {
    let (context,repository,mut character)=super::native_payment_tests::fixture(false,true);
    character.status.hp=20;character.status.zeny=55;
    character.script_skill_state.spirit_spheres=vec![10000,10000];
    let mut stored=database::required::<database::model::CharacterRecord>(&repository.database.characters,&character.char_id.to_be_bytes()).unwrap();
    stored.hp=20;stored.zeny=55;
    repository.database.characters.transaction(|tree|database::tx_write(tree,&character.char_id.to_be_bytes(),&stored)).unwrap();
    let mut cost=crate::server::script::skill::requirements::SkillRequirementPlan {minimum_hp:1,hp:20,sp:7,zeny:15,spirit_spheres:2,..Default::default()};
    assert!(context.server.item_service().pay_requirement_plan(&context.server,&mut character,&cost,None,1000).is_err());
    assert_eq!((character.status.hp,character.status.sp,character.status.zeny),(20,1000,55));
    cost.allow_hp_death=true;
    context.server.item_service().pay_requirement_plan(&context.server,&mut character,&cost,None,1000).unwrap();
    assert!(character.is_dead());assert!(!character.is_using_skill());
    assert_eq!((character.status.hp,character.status.sp,character.status.zeny),(0,993,40));
    assert!(character.script_skill_state.spirit_spheres.is_empty());
    let saved=database::required::<database::model::CharacterRecord>(&repository.database.characters,&character.char_id.to_be_bytes()).unwrap();
    assert_eq!((saved.hp,saved.sp,saved.zeny),(0,993,40));
}

#[test]
fn a_failed_lethal_proc_transaction_keeps_the_actor_alive_and_resources_unchanged() {
    let (context,repository,mut character,_)=identification_fixture(true);
    character.status.hp=20;character.status.zeny=55;
    let mut stored=database::required::<database::model::CharacterRecord>(&repository.database.characters,&character.char_id.to_be_bytes()).unwrap();
    stored.hp=20;stored.zeny=55;
    repository.database.characters.transaction(|tree|database::tx_write(tree,&character.char_id.to_be_bytes(),&stored)).unwrap();
    let mut inventory=records(&repository,&character);inventory.retain(|item|item.id!=10);
    repository.database.inventories.transaction(|tree|database::tx_write(tree,&character.char_id.to_be_bytes(),&inventory)).unwrap();
    let cost=crate::server::script::skill::requirements::SkillRequirementPlan {minimum_hp:1,allow_hp_death:true,hp:20,sp:7,zeny:15,..Default::default()};
    assert!(context.server.item_service().pay_requirement_plan(&context.server,&mut character,&cost,Some(0),1000).is_err());
    assert!(!character.is_dead());assert_eq!((character.status.hp,character.status.sp,character.status.zeny),(20,1000,55));
    assert_eq!(character.get_item_from_inventory(0).unwrap().amount,2);
    let saved=database::required::<database::model::CharacterRecord>(&repository.database.characters,&character.char_id.to_be_bytes()).unwrap();
    assert_eq!((saved.hp,saved.sp,saved.zeny),(20,1000,55));
}

#[test]
fn identification_selection_atomically_pays_the_learned_skill_cost() {
    let (context, repository, mut character, selected) = identification_fixture(false);
    let stored: CharacterRecord = database::required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!((character.status.sp, stored.sp), (1000, 1000));
    assert!(!records(&repository, &character)[0].is_identified);
    context.server.item_service().identify_item(&context.server, &mut character, selected, 101).unwrap();
    let stored: CharacterRecord = database::required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!((character.status.sp, stored.sp), (990, 990));
    assert!(records(&repository, &character)[0].is_identified);
    assert!(character.pending_item_skill.is_none());
    assert!(context.server.item_service().identify_item(&context.server, &mut character, selected, 102).is_err());
    assert_eq!(character.status.sp, 990);
}

#[test]
fn magnifier_selection_commits_identification_and_source_consumption_together() {
    let (context, repository, mut character, selected) = identification_fixture(true);
    assert_eq!(records(&repository, &character)[0].amount, 2);
    context.server.item_service().identify_item(&context.server, &mut character, selected, 101).unwrap();
    let saved = records(&repository, &character);
    assert_eq!(saved[0].amount, 1);
    assert!(saved[1].is_identified);
    assert_eq!(character.status.sp, 1000);
    assert!(character.pending_item_skill.is_none());
}

#[test]
fn stale_identification_sources_and_insufficient_sp_preserve_the_selected_item() {
    let (context, repository, mut character, selected) = identification_fixture(true);
    repository.database.inventories.transaction(|tree| {
        let mut saved: Vec<InventoryRecord> = database::tx_required(tree, &character.char_id.to_be_bytes())?;
        saved[0].unique_id = 77;
        database::tx_write(tree, &character.char_id.to_be_bytes(), &saved)
    }).unwrap();
    assert!(context.server.item_service().identify_item(&context.server, &mut character, selected, 101).is_err());
    let saved = records(&repository, &character);
    assert_eq!(saved[0].amount, 2);
    assert!(!saved[1].is_identified);
    assert!(!character.get_item_from_inventory(selected).unwrap().is_identified);

    let (context, repository, mut character, selected) = identification_fixture(false);
    character.status.sp = 9;
    assert!(context.server.item_service().identify_item(&context.server, &mut character, selected, 101).is_err());
    assert!(!records(&repository, &character)[0].is_identified);
    assert_eq!(character.status.sp, 9);
}

#[test]
fn live_rank_events_preserve_resources_on_promotion_and_atomically_cap_demotion() {
    let (context, repository, mut character) = super::native_payment_tests::fixture(false, false);
    character.status.job = JobName::Taekwon.value() as u32;
    character.status.base_level = 90;
    let normal = StatusService::instance().to_snapshot(&character.status);
    let normal_pools = (normal.max_hp(), normal.max_sp());
    character.status.hp = normal_pools.0;
    character.status.sp = normal_pools.1;
    let id = character.char_id;
    context.server.state_mut().insert_character(character);
    context.server.handle_script_event(&mut *context.server.state_mut(), GameEvent::FameChanged(FameChanged { category: FameCategory::Taekwon, ranked_creators: vec![id] }), 100).unwrap();
    {
        let mut state = context.server.state_mut();
        let character = state.characters_mut().get_mut(&id).unwrap();
        assert!(character.status.taekwon_ranked);
        assert_eq!((character.status.hp, character.status.sp), normal_pools);
        assert!(character.status.max_hp > normal_pools.0);
        character.status.hp = character.status.max_hp;
        character.status.sp = character.status.max_sp;
    }
    context.server.handle_script_event(&mut *context.server.state_mut(), GameEvent::FameChanged(FameChanged { category: FameCategory::Taekwon, ranked_creators: vec![] }), 101).unwrap();
    let guard_168 = context.server.state();
    let character = guard_168.characters().get(&id).unwrap();
    assert!(!character.status.taekwon_ranked);
    assert_eq!((character.status.hp, character.status.sp), normal_pools);
    assert_eq!((character.status.max_hp, character.status.max_sp), normal_pools);
    drop(guard_168);
    let saved: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
    assert_eq!((saved.hp as u32, saved.sp as u32, saved.max_hp as u32, saved.max_sp as u32), (normal_pools.0, normal_pools.1, normal_pools.0, normal_pools.1));
}

fn npc_fixture() -> (Arc<Server>, Arc<Session>, NpcContact) {
    let (context, _repository, character) = super::native_payment_tests::fixture(false, false);
    let npc_id = 600;
    let script = Script { id: npc_id, scope_instance: 0, entry_id: crate::server::script::entries::system_npc("counter"), map_name: "empty".into(), name: "Counter".into(), sprite: 0, x: 51, y: 50, dir: 0, x_size: 0, y_size: 0, constructor_args: vec![] };
    let map = Box::leak(Box::new(Map::new(100, 100, 10_000, "empty".into(), "empty.gat".into(), vec![], vec![], vec![script])));
    let cells = vec![models::enums::cell::CellType::Walkable.as_flag(); 10_001];
    let instance = MapInstance::from_map(crate::tests::common::test_npc_vm(), map, 0, cells, context.client_notification_sender.clone(), MapItems::new(0), Arc::new(TasksQueue::new()));
    context.server.state_mut().map_instances_mut().insert("empty".into(), vec![Arc::new(instance)]);
    let mut session = Session::create_empty(character.account_id, 0, 0, context.server.packetver());
    session.char_id = Some(character.char_id);
    let session = Arc::new(session);
    context.server.state().add_session(character.account_id, session.clone());
    let contact = NpcContact { char_id: character.char_id, account_id: character.account_id, npc_id };
    context.server.state_mut().insert_character(character);
    let server = Arc::new(context.server);
    server.bind_shared();
    (server, session, contact)
}

#[test]
fn npc_contact_starts_a_compiled_dialog_and_returns_mutations_to_the_main_loop() {
    let (server, session, contact) = npc_fixture();
    server.handle_script_event(&mut *server.state_mut(), GameEvent::NpcContact(contact), 100).unwrap();
    let input = session.script_handler_channel_sender.lock().unwrap().clone().unwrap();
    server.runtime().block_on(input.send(PlayerInput::Next)).unwrap();
    let deadline = Instant::now() + Duration::from_secs(3);
    let mut mutation_requests = 0;
    while Instant::now() < deadline {
        if let Some(events) = server.pop_task() {
            for event in events {
                if let GameEvent::ScriptRequest(request) = event {
                    mutation_requests += 1;
                    server.script_service().handle_request(&server, &mut *server.state_mut(), request);
                } else { panic!("Unexpected NPC mutation: {event:?}"); }
            }
        }
        if session.script_handler_channel_sender.lock().unwrap().is_none() { break; }
        std::thread::sleep(Duration::from_millis(5));
    }
    assert_eq!(mutation_requests, 1);
    assert!(session.script_handler_channel_sender.lock().unwrap().is_none());
}

#[test]
fn unavailable_or_remote_npc_contacts_do_not_start_a_conversation() {
    let (server, session, contact) = npc_fixture();
    server.state_mut().characters_mut().get_mut(&contact.char_id).unwrap().x = 1;
    assert!(server.handle_script_event(&mut *server.state_mut(), GameEvent::NpcContact(contact.clone()), 100).is_err());
    server.state_mut().characters_mut().get_mut(&contact.char_id).unwrap().x = 50;
    let mut invalid = contact.clone(); invalid.account_id += 1;
    assert!(server.handle_script_event(&mut *server.state_mut(), GameEvent::NpcContact(invalid), 100).is_err());
    server.state_mut().characters_mut().get_mut(&contact.char_id).unwrap().status.hp = 0;
    assert!(server.handle_script_event(&mut *server.state_mut(), GameEvent::NpcContact(contact), 100).is_err());
    assert!(session.script_handler_channel_sender.lock().unwrap().is_none());
    assert!(server.pop_task().is_none());
}

fn incoming_damage(target_id: u32, attacker_id: u32, flags: u32, skill: SkillEnum) -> Damage {
    Damage { notification: None, source_kind: models::enums::actor::CombatActorKind::Player, skill_damage_adjusted: false, target_id, attacker_id, credit_id: attacker_id, damage: 100, healing: 0, attacked_at: 100,
        damage_motion: 500, battle_flags: flags, skill_id: skill.id(), skill_level: 1,
        proc_depth: 0, defenses_applied: true, landed: true, magic_context: None, right_hand_damage: None }
}

#[test]
fn elemental_absorption_heals_atomically_without_consuming_shields_or_interrupting_movement() {
    let (context, repository, mut character) = super::native_payment_tests::fixture(false, false);
    let id = character.char_id;
    for kind in [StatusChangeKind::Berserk, StatusChangeKind::MagicRod, StatusChangeKind::Kyrie] {
        let mut request = StatusChangeRequest::guaranteed(kind, 1000, 1);
        if kind == StatusChangeKind::Kyrie { request.values[1] = 1000; request.values[2] = 10; }
        StatusEffectService::apply_status(&mut character.status, request, 100, 0).unwrap();
    }
    character.status.hp = 1; character.status.sp = 17;
    let cap = StatusService::instance().to_snapshot(&character.status).max_hp();
    let statuses = character.status.active_statuses.clone();
    let canmove = character.timing.get_canmove_tick();
    context.server.state_mut().insert_character(character);
    let mut damage = incoming_damage(id, id + 1, BattleFlag::Magic.as_flag() | BattleFlag::Skill.as_flag(), SkillEnum::MgFirebolt);
    damage.set_signed_damage(-i32::MAX);
    context.server.admit_character_damage(&mut *context.server.state_mut(), damage, 101).unwrap();
    let state = context.server.state();
    let live = state.get_character(id).unwrap();
    assert_eq!((live.status.hp, live.status.sp), (cap, 17));
    assert_eq!(live.status.active_statuses, statuses);
    assert_eq!(live.timing.get_canmove_tick(), canmove);
    let saved: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
    assert_eq!(saved.hp as u32, cap);
    assert!(context.server.pop_task().is_none());
}

#[test]
fn stale_absorption_owner_preserves_live_and_persistent_hp_and_dead_targets_stay_dead() {
    let (context, repository, mut character) = super::native_payment_tests::fixture(false, false);
    let id = character.char_id;
    character.status.hp = 1;
    context.server.state_mut().insert_character(character);
    repository.database.characters.transaction(|tree| {
        let mut saved: CharacterRecord = database::tx_required(tree, &id.to_be_bytes())?;
        saved.account_id += 1;
        database::tx_write(tree, &id.to_be_bytes(), &saved)
    }).unwrap();
    let mut damage = incoming_damage(id, id + 1, BattleFlag::Weapon.as_flag(), SkillEnum::SmBash);
    damage.set_signed_damage(-100);
    assert!(context.server.admit_character_damage(&mut *context.server.state_mut(), damage, 101).is_err());
    assert_eq!(context.server.state().get_character(id).unwrap().status.hp, 1);
    let saved: CharacterRecord = database::required(&repository.database.characters, &id.to_be_bytes()).unwrap();
    assert_eq!(saved.hp, 1000);
    context.server.state_mut().characters_mut().get_mut(&id).unwrap().status.hp = 0;
    context.server.admit_character_damage(&mut *context.server.state_mut(), damage, 102).unwrap();
    assert_eq!(context.server.state().get_character(id).unwrap().status.hp, 0);
}

#[test]
fn devotion_redirects_hp_without_interrupting_either_actor_or_reapplying_defenses() {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    let protected = character.char_id;
    let protector = protected + 1;
    let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Devotion, 1000, protector as i32);
    request.values[2] = 7;
    StatusEffectService::apply_status(&mut character.status, request, 100, 0).unwrap();
    character.script_skill_state.casting_until = 10_000;
    character.script_skill_state.casting_skill_id = SkillEnum::MgFirebolt.id();
    context.server.state_mut().insert_character(character);
    {
        let mut state = context.server.state_mut();
        let protector = state.characters_mut().get_mut(&protector).unwrap();
        protector.status.vit = 100;
        protector.script_skill_state.casting_until = 10_000;
        protector.script_skill_state.casting_skill_id = SkillEnum::MgFirebolt.id();
    }
    let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
    context.server.admit_character_damage(&mut *context.server.state_mut(), incoming_damage(protected, 9999, flags, SkillEnum::NvBasic), 101).unwrap();
    let state = context.server.state();
    let target = state.characters().get(&protected).unwrap();
    let source = state.characters().get(&protector).unwrap();
    assert_eq!((target.status.hp, source.status.hp), (1000, 900));
    assert_eq!((target.script_skill_state.casting_until, source.script_skill_state.casting_until), (10_000, 10_000));
    assert_eq!((target.timing.get_canmove_tick(), source.timing.get_canmove_tick()), (0, 0));
    assert!(context.server.pop_task().is_none());
}

#[test]
fn expired_remote_or_zero_hp_protectors_cannot_receive_devotion_damage() {
    for failure in 0..3 {
        let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
        let protected = character.char_id;
        let protector = protected + 1;
        let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Devotion, 1000, protector as i32);
        request.values[2] = 7;
        StatusEffectService::apply_status(&mut character.status, request, 100, 0).unwrap();
        context.server.state_mut().insert_character(character);
        if failure == 1 { context.server.state_mut().characters_mut().get_mut(&protector).unwrap().x = 90; }
        if failure == 2 { context.server.state_mut().characters_mut().get_mut(&protector).unwrap().status.hp = 0; }
        let tick = if failure == 0 { 1100 } else { 101 };
        let flags = BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag();
        context.server.admit_character_damage(&mut *context.server.state_mut(), incoming_damage(protected, 9999, flags, SkillEnum::NvBasic), tick).unwrap();
        let state = context.server.state();
        let target = state.characters().get(&protected).unwrap();
        assert_eq!(target.status.hp, 900);
        assert!(!target.status.has_status_change(StatusChangeKind::Devotion));
        assert_eq!(state.characters().get(&protector).unwrap().status.hp, if failure == 2 { 0 } else { 1000 });
    }
}

#[test]
fn grand_cross_self_damage_emits_only_the_physical_hit_counter_callback() {
    let (context, _repository, character) = super::native_payment_tests::fixture(false, false);
    let id = character.char_id;
    context.server.state_mut().insert_character(character);
    let flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
    context.server.admit_character_damage(&mut *context.server.state_mut(), incoming_damage(id, id, flags, SkillEnum::CrGrandcross), 100).unwrap();
    assert_eq!(context.server.state().characters().get(&id).unwrap().status.hp, 900);
    let requests: Vec<_> = context.server.pop_task().unwrap().into_iter().filter_map(|event| {
        if let GameEvent::ScriptCombat(request) = event { Some(request) } else { None }
    }).collect();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].trigger, CombatTrigger::Hit);
    assert_eq!(requests[0].battle_flags, BattleFlag::Weapon.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag());
}

#[test]
fn magic_rod_absorbs_a_direct_spell_before_damage_interruptions_and_combat_callbacks() {
    let (context, _repository, mut character) = super::native_payment_tests::fixture(false, false);
    let id = character.char_id;
    character.status.sp = 0;
    character.script_skill_state.casting_until = 10_000;
    character.script_skill_state.casting_skill_id = SkillEnum::MgFirebolt.id();
    StatusEffectService::apply_status(&mut character.status, StatusChangeRequest::guaranteed(StatusChangeKind::MagicRod, 1000, 5), 100, 0).unwrap();
    context.server.state_mut().insert_character(character);
    let flags = BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
    context.server.admit_character_damage(&mut *context.server.state_mut(), incoming_damage(id, id + 1, flags, SkillEnum::MgFirebolt), 101).unwrap();
    let guard_367 = context.server.state();
    let target = guard_367.characters().get(&id).unwrap();
    assert_eq!((target.status.hp, target.status.sp), (1000, 12));
    assert_eq!(target.script_skill_state.casting_until, 10_000);
    assert_eq!(target.timing.get_canmove_tick(), 0);
    drop(guard_367);
    assert!(context.server.pop_task().is_none());
}

#[test]
fn utsusemi_blocks_a_physical_hit_and_pushes_the_blocking_character_away_from_the_attacker() {
    let (context, _repository, mut target) = super::native_payment_tests::fixture(false, false);
    let id = target.char_id;
    StatusEffectService::apply_status(&mut target.status, StatusChangeRequest::guaranteed(StatusChangeKind::Utsusemi, 60_000, 5), 100, 0).unwrap();
    let mut attacker = crate::tests::common::character_helper::create_character();
    attacker.char_id = id + 1;
    attacker.status.hp = 100;
    attacker.map_instance_key = target.map_instance_key.clone();
    attacker.x = 51;
    attacker.y = 50;
    context.server.state_mut().insert_character(target);
    context.server.state_mut().insert_character(attacker);

    let damage = incoming_damage(id, id + 1, BattleFlag::Weapon.as_flag(), SkillEnum::SmBash);
    context.server.admit_character_damage(&mut *context.server.state_mut(), damage, 101).unwrap();

    let state = context.server.state();
    let target = state.get_character(id).unwrap();
    assert_eq!(target.status.hp, 1000);
    assert_eq!(target.status.status_change(StatusChangeKind::Utsusemi).unwrap().values[1], 2);
    assert_eq!((target.x, target.y), (43, 50));
}

#[test]
fn utsusemi_does_not_block_or_push_magic_hits() {
    let (context, _repository, mut target) = super::native_payment_tests::fixture(false, false);
    let id = target.char_id;
    StatusEffectService::apply_status(&mut target.status, StatusChangeRequest::guaranteed(StatusChangeKind::Utsusemi, 60_000, 5), 100, 0).unwrap();
    let mut attacker = crate::tests::common::character_helper::create_character();
    attacker.char_id = id + 1;
    attacker.status.hp = 100;
    attacker.map_instance_key = target.map_instance_key.clone();
    attacker.x = 51;
    attacker.y = 50;
    context.server.state_mut().insert_character(target);
    context.server.state_mut().insert_character(attacker);

    let damage = incoming_damage(id, id + 1, BattleFlag::Magic.as_flag(), SkillEnum::MgFirebolt);
    context.server.admit_character_damage(&mut *context.server.state_mut(), damage, 101).unwrap();

    let state = context.server.state();
    let target = state.get_character(id).unwrap();
    assert!(target.status.hp < 1000);
    assert_eq!(target.status.status_change(StatusChangeKind::Utsusemi).unwrap().values[1], 3);
    assert_eq!((target.x, target.y), (50, 50));
}
