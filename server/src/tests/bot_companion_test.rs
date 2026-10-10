//! Pets, homunculi, mercenaries and `@` commands through the bot API: each action answers with what the game did or why it refused.
use std::sync::Arc;

use super::ServerServiceTestContext;
use super::player_trade_tests::fixture;
use crate::repository::{GameSystemRepository, InventoryRepository, SledRepository};
use crate::server::bots::{CompanionAction, HomunculusAction, MercenaryAction, Order, PetAction, ProgressAction, SkillCast, SkillRef, observe};
use models::enums::item::ItemType;

use crate::server::model::game_systems::{HomunculusRecord, MercenaryRecord, PendingPetCapture, PetRecord};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::permission_groups::PermissionGroups;
use crate::server::model::session::{AccountSession, Session};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_world_service::{homunculus_world_id, mercenary_world_id, world_data};

const FIRST: u32 = 150_000;
const SECOND: u32 = 150_001;
const TICK: u128 = 1_000;
const SUPER_PLAYER: u32 = 1;

fn error_of<T: std::fmt::Debug>(result: Result<T, String>) -> String {
    result.expect_err("the action should have been refused")
}

fn companion(context: &ServerServiceTestContext, char_id: u32, action: impl Into<CompanionAction>) -> Result<serde_json::Value, String> {
    let mut state = context.server.state_mut();
    context.server.bot_companion(&mut state, char_id, &action.into(), TICK)
}

fn seen(context: &ServerServiceTestContext, char_id: u32) -> serde_json::Value {
    let state = context.server.state();
    serde_json::to_value(observe(&context.server, &state, char_id, TICK).unwrap()).unwrap()["companions"].clone()
}

fn cast(context: &ServerServiceTestContext, char_id: u32, skill: &str, target: Option<u32>) -> Result<serde_json::Value, String> {
    let mut state = context.server.state_mut();
    let cast = SkillCast { skill: SkillRef::Name(skill.into()), level: None, target, cell: None };
    context.server.bot_skill(&mut state, char_id, &cast, TICK)
}

fn on<R>(context: &ServerServiceTestContext, char_id: u32, change: impl FnOnce(&mut crate::server::state::character::Character) -> R) -> R {
    let mut state = context.server.state_mut();
    change(state.characters_mut().get_mut(&char_id).unwrap())
}

/// The notification channel of the test server has no buffer, so a render that finds nobody receiving is not a refusal of the action.
fn settle(result: Result<serde_json::Value, String>) {
    if let Err(error) = result {
        assert!(error.contains("notification queue"), "{error}");
    }
}

fn install_pet(context: &ServerServiceTestContext, repository: &SledRepository, renamed: bool) {
    let definition = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap();
    let pet = PetRecord {
        id: 77,
        owner_char_id: FIRST,
        class_id: definition.class_id,
        name: definition.name.clone(),
        level: definition.level,
        egg_item_id: definition.egg_item,
        egg_inventory_id: 0,
        intimacy: 910,
        hunger: 73,
        equipped_item: 0,
        next_hunger_at: u64::MAX,
        incubating: false,
        renamed,
    };
    let mut systems = context.server.state().get_character(FIRST).unwrap().game_systems.clone();
    systems.pet = Some(pet);
    let saved = repository.save_character_game_systems(FIRST, &systems).unwrap();
    on(context, FIRST, |character| character.game_systems = saved);
}

fn install_homunculus(context: &ServerServiceTestContext, repository: &SledRepository, change: impl FnOnce(&mut HomunculusRecord)) -> u32 {
    let mut homunculus = HomunculusRecord {
        id: 8,
        class_id: 6001,
        name: "Lif".into(),
        level: 10,
        hp: 100,
        max_hp: 100,
        sp: 100,
        max_sp: 100,
        base_max_hp: 100,
        base_max_sp: 100,
        stats: [10; 6],
        active: true,
        ..HomunculusRecord::default()
    };
    change(&mut homunculus);
    let id = homunculus_world_id(&homunculus);
    let mut systems = context.server.state().get_character(SECOND).unwrap().game_systems.clone();
    systems.homunculus = Some(homunculus);
    let saved = repository.save_character_game_systems(SECOND, &systems).unwrap();
    on(context, SECOND, |character| character.game_systems = saved);
    id
}

fn install_mercenary(context: &ServerServiceTestContext, repository: &SledRepository) -> u32 {
    let definition = world_data().mercenaries.iter().find(|mercenary| mercenary.class_id == 1191).unwrap();
    let mercenary = MercenaryRecord {
        id: 4,
        class_id: definition.class_id,
        name: definition.name.clone(),
        level: definition.level,
        hp: definition.hp,
        max_hp: definition.hp,
        sp: definition.sp,
        max_sp: definition.sp,
        expires_at: u64::MAX / 2,
        contract_remaining: None,
        skill_cooldowns: Default::default(),
        cooldown_pause_at: None,
        guild: 0,
        kill_count: 0,
        statuses: Vec::new(),
        last_attack_at: 0,
        last_regen_hp_at: 0,
        last_regen_sp_at: 0,
    };
    let id = mercenary_world_id(&mercenary);
    let mut systems = context.server.state().get_character(FIRST).unwrap().game_systems.clone();
    systems.mercenary = Some(mercenary);
    let saved = repository.save_character_game_systems(FIRST, &systems).unwrap();
    on(context, FIRST, |character| character.game_systems = saved);
    id
}

#[test]
fn a_pet_is_listed_and_every_request_without_one_says_so() {
    let (context, repository) = fixture();
    let nothing = seen(&context, FIRST);
    assert!(nothing["pet"].is_null() && nothing["pet_eggs"].is_null() && nothing["pet_capture"].is_null());
    assert!(error_of(companion(&context, FIRST, PetAction::Hatch { index: 0 })).contains("No hatching window"));
    assert!(error_of(companion(&context, FIRST, PetAction::Feed)).contains("No pet is active"));
    assert!(error_of(companion(&context, FIRST, PetAction::Rename("Biscuit".into()))).contains("No pet is active"));
    assert!(error_of(companion(&context, FIRST, PetAction::Capture { target: 5 })).contains("No monster 5"));
    assert!(error_of(companion(&context, FIRST, PetAction::Emote(200_001))).contains("0 to 200000"));

    install_pet(&context, &repository, false);
    let pet = seen(&context, FIRST)["pet"].clone();
    assert_eq!((pet["name"].as_str(), pet["intimacy_stage"].as_str(), pet["hunger_stage"].as_str()), (Some("Poring"), Some("loyal"), Some("neutral")));
    assert!(pet["food"]["name"].is_string() && pet["accessory"].is_null());

    let renamed = companion(&context, FIRST, PetAction::Rename("  Biscuit ".into())).unwrap();
    assert_eq!(renamed["pet"]["name"], "Biscuit");
    assert!(error_of(companion(&context, FIRST, PetAction::Rename("Again".into()))).contains("already been renamed"));
    assert!(error_of(companion(&context, FIRST, PetAction::Equip { index: 0 })).contains("cannot be equipped"));
    assert_eq!(on(&context, FIRST, |character| character.game_systems.pet.as_ref().map(|pet| pet.name.clone())).as_deref(), Some("Biscuit"));
}

#[test]
fn an_egg_hatches_in_the_window_that_listed_it_and_goes_back_to_an_egg() {
    let (context, repository) = fixture();
    let definition = world_data().pets.iter().find(|pet| pet.class_id == 1002).unwrap();
    repository
        .database
        .items
        .transaction(|tree| database::tx_write(tree, &definition.egg_item.to_be_bytes(), GlobalConfigService::instance().get_item(definition.egg_item)))
        .unwrap();
    let egg = PetRecord {
        id: 0,
        owner_char_id: FIRST,
        class_id: definition.class_id,
        name: definition.name.clone(),
        level: definition.level,
        egg_item_id: definition.egg_item,
        egg_inventory_id: 0,
        intimacy: definition.intimacy_start,
        hunger: 100,
        equipped_item: 0,
        next_hunger_at: u64::MAX,
        incubating: true,
        renamed: false,
    };
    repository.create_pet_egg(FIRST, &egg, 100_000).unwrap();
    let inventory = context.runtime().block_on(repository.character_inventory_fetch(FIRST as i32)).unwrap();
    on(&context, FIRST, |character| character.inventory = inventory.into_iter().map(Some).collect());

    assert!(seen(&context, FIRST)["pet_eggs"].is_null());
    on(&context, FIRST, |character| character.game_systems.pet_hatching = true);
    let eggs = seen(&context, FIRST)["pet_eggs"].clone();
    assert_eq!(eggs.as_array().unwrap().len(), 1, "{eggs}");
    assert_eq!(eggs[0]["item_id"], definition.egg_item);
    let index = eggs[0]["index"].as_u64().unwrap() as usize;

    let wrong = error_of(companion(&context, FIRST, PetAction::Hatch { index: 40 }));
    assert!(wrong.contains("not an egg of the window") && wrong.contains(&index.to_string()), "{wrong}");
    assert!(on(&context, FIRST, |character| character.game_systems.pet_hatching), "a refused index keeps the window open");
    settle(companion(&context, FIRST, PetAction::Hatch { index }));
    assert_eq!(seen(&context, FIRST)["pet"]["class_id"], definition.class_id);
    assert!(seen(&context, FIRST)["pet_eggs"].is_null());

    settle(companion(&context, FIRST, PetAction::ReturnToEgg));
    assert!(seen(&context, FIRST)["pet"].is_null());
    assert!(on(&context, FIRST, |character| character.inventory_iter().any(|(_, item)| item.item_id == definition.egg_item)));
}

#[test]
fn every_pet_egg_of_the_catalog_is_typed_as_one() {
    crate::tests::common::before_all();
    let config = GlobalConfigService::instance();
    let wrong: Vec<_> = world_data()
        .pets
        .iter()
        .filter_map(|pet| config.find_item(pet.egg_item))
        .filter(|egg| egg.name_aegis.ends_with("_Egg") && egg.item_type != ItemType::PetEgg)
        .map(|egg| egg.id)
        .collect();
    assert!(wrong.is_empty(), "{wrong:?}");
}

#[test]
fn a_capture_is_pending_then_captured_or_failed() {
    let (context, _) = fixture();
    let outcome = |claim| companion(&context, FIRST, PetAction::CaptureOutcome { claim }).unwrap()["capture"].clone();
    assert_eq!(outcome(7), "failed");
    let claim = |claim_id, committed| PendingPetCapture { claim_id, target_id: 5, map_key: MapInstanceKey::new("empty".to_string(), 0), expires_at: u64::MAX, committed };
    on(&context, FIRST, |character| character.game_systems.pending_pet_capture = Some(claim(7, false)));
    assert_eq!(outcome(7), "pending");
    assert_eq!(outcome(8), "failed");
    on(&context, FIRST, |character| {
        character.game_systems.pending_pet_capture = None;
        character.game_systems.completed_pet_capture = Some(claim(7, true));
    });
    assert_eq!((outcome(7), outcome(8)), ("captured".into(), "failed".into()));
}

#[test]
fn a_homunculus_is_learned_from_and_cast_through_the_bot_api() {
    let (context, repository) = fixture();
    assert!(error_of(companion(&context, SECOND, HomunculusAction::Order(Order::Follow))).contains("no homunculus"));
    assert!(error_of(cast(&context, SECOND, "HLIF_AVOID", None)).contains("not a skill of the active homunculus"));

    install_homunculus(&context, &repository, |homunculus| homunculus.skill_points = 1);
    let view = seen(&context, SECOND)["homunculus"].clone();
    assert_eq!((view["name"].as_str(), view["skill_points"].as_u64(), view["active"].as_bool()), (Some("Lif"), Some(1), Some(true)));
    let heal = view["skills"].as_array().unwrap().iter().find(|skill| skill["name"] == "HLIF_HEAL").expect("a skill the homunculus can learn");
    assert_eq!((heal["level"].as_u64(), heal["learnable"].as_bool(), heal["aim"].as_str()), (Some(0), Some(true), Some("self")));

    let learn = |skill: &str| {
        let mut state = context.server.state_mut();
        context.server.bot_progress(&mut state, SECOND, &ProgressAction::LearnSkill { skill: SkillRef::Name(skill.into()) })
    };
    assert!(error_of(learn("HLIF_BRAIN")).contains("prerequisites"));
    let learned = learn("HLIF_HEAL").unwrap();
    assert_eq!((learned["skill"].as_str(), learned["level"].as_u64(), learned["homunculus_skill_points"].as_u64()), (Some("HLIF_HEAL"), Some(1), Some(0)));
    assert!(error_of(learn("HLIF_HEAL")).contains("cannot learn"));
    assert!(error_of(learn("MS_BASH")).contains("nothing to learn"));
    assert_eq!(on(&context, SECOND, |character| character.status.skill_point), 0);

    assert!(error_of(cast(&context, SECOND, "HLIF_AVOID", None)).contains("not a skill of the active homunculus"));
    on(&context, SECOND, |character| character.game_systems.homunculus.as_mut().unwrap().skills.insert(8002, 1));
    on(&context, SECOND, |character| character.game_systems.homunculus.as_mut().unwrap().sp = 5);
    let short = error_of(cast(&context, SECOND, "HLIF_AVOID", None));
    assert!(!short.contains("not a skill"), "{short}");
    on(&context, SECOND, |character| character.game_systems.homunculus.as_mut().unwrap().sp = 100);
    let started = cast(&context, SECOND, "hlif_avoid", None).unwrap();
    assert_eq!((started["skill"].as_str(), started["caster"].as_str(), started["started"].as_bool()), (Some("HLIF_AVOID"), Some("homunculus"), Some(true)));
    assert!(error_of(cast(&context, SECOND, "HLIF_BRAIN", None)).contains("not a skill"));
}

#[test]
fn a_homunculus_obeys_orders_it_can_follow_and_can_be_renamed_once() {
    let (context, repository) = fixture();
    install_homunculus(&context, &repository, |_| {});
    let (x, y) = on(&context, SECOND, |character| (character.x(), character.y()));
    assert!(error_of(companion(&context, SECOND, HomunculusAction::Order(Order::Move { x: x + 40, y }))).contains("out of reach"));
    assert!(error_of(companion(&context, SECOND, HomunculusAction::Order(Order::Attack { target: 77, repeat: true }))).contains("unavailable"));
    companion(&context, SECOND, HomunculusAction::Order(Order::Move { x: x + 2, y })).unwrap();
    assert_eq!(on(&context, SECOND, |character| character.game_systems.companion_commands.values().next().and_then(|command| command.destination)), Some((x + 2, y)));
    companion(&context, SECOND, HomunculusAction::Order(Order::Follow)).unwrap();
    assert!(on(&context, SECOND, |character| character.game_systems.companion_commands.is_empty()));

    assert_eq!(companion(&context, SECOND, HomunculusAction::Rename("Fido".into())).unwrap()["homunculus"]["name"], "Fido");
    assert!(error_of(companion(&context, SECOND, HomunculusAction::Rename("Rex".into()))).contains("already been renamed"));

    on(&context, SECOND, |character| character.game_systems.homunculus.as_mut().unwrap().active = false);
    assert!(error_of(companion(&context, SECOND, HomunculusAction::Order(Order::Follow))).contains("resting"));
    on(&context, SECOND, |character| character.game_systems.homunculus.as_mut().unwrap().active = true);
    assert_eq!(companion(&context, SECOND, HomunculusAction::Delete).unwrap()["deleted"], true);
    assert!(seen(&context, SECOND)["homunculus"].is_null());
}

#[test]
fn a_mercenary_is_listed_ordered_and_dismissed() {
    let (context, repository) = fixture();
    assert!(error_of(companion(&context, FIRST, MercenaryAction::Dismiss)).contains("no mercenary"));
    install_mercenary(&context, &repository);
    let view = seen(&context, FIRST)["mercenary"].clone();
    assert_eq!(view["name"], "Mimic");
    let skills: Vec<_> = view["skills"].as_array().unwrap().iter().map(|skill| skill["name"].as_str().unwrap()).collect();
    assert_eq!(skills, ["MER_AUTOBERSERK", "ML_AUTOGUARD", "MS_BASH"]);
    assert!(view["contract_ms_left"].as_u64().unwrap() > 0);

    assert!(error_of(cast(&context, FIRST, "MS_BASH", None)).contains("needs a target"));
    assert!(error_of(cast(&context, FIRST, "MS_MAGNUM", Some(5))).contains("not a skill"));
    let (x, y) = on(&context, FIRST, |character| (character.x(), character.y()));
    assert!(error_of(companion(&context, FIRST, MercenaryAction::Order(Order::Move { x: x + 40, y }))).contains("out of reach"));
    companion(&context, FIRST, MercenaryAction::Order(Order::Follow)).unwrap();
    assert_eq!(companion(&context, FIRST, MercenaryAction::Dismiss).unwrap()["dismissed"], true);
    assert!(seen(&context, FIRST)["mercenary"].is_null());
}

fn join_group(context: &ServerServiceTestContext, char_id: u32, group: u32) {
    let state = context.server.state_mut();
    let account_id = state.characters().get(&char_id).unwrap().account_id;
    let old = state.find_session(account_id).unwrap();
    let mut session = Session::create_empty(account_id, old.auth_code, old.user_level, context.server.packetver()).with_account(AccountSession::new(1, group, 12));
    session.char_id = old.char_id;
    session.map_server_socket = old.map_server_socket.clone();
    state.add_session(account_id, Arc::new(session));
}

#[test]
fn a_command_runs_when_the_group_of_the_bots_allows_it_and_answers_with_what_it_said() {
    let (context, _) = fixture();
    context.server.state_mut().set_permission_groups(
        PermissionGroups::from_json(
            r#"{"groups": [
                {"id": 0, "name": "Player", "commands": ["changedress"]},
                {"id": 1, "name": "Super Player", "commands": ["rates", "noask", "go"]}
            ], "atcommands": {"rates": ["rate"], "noask": [], "go": [], "item": [], "monster": [], "changedress": []}}"#,
        )
        .unwrap(),
    );
    let command = |text: &str| {
        let mut state = context.server.state_mut();
        context.server.bot_command(&mut state, FIRST, text)
    };
    assert!(error_of(command("rates")).contains("starts with @"));
    assert!(error_of(command("@")).contains("starts with @"));
    let refused = error_of(command("@rates"));
    assert!(refused.contains("allows:") && refused.contains("changedress"), "{refused}");

    join_group(&context, FIRST, SUPER_PLAYER);
    let answer = command("@rates").unwrap();
    assert_eq!(answer["command"], "rates");
    assert!(answer["replies"].as_array().unwrap().iter().any(|line| line.as_str().unwrap().contains("Base")), "{answer}");
    let refused = error_of(command("@item 501 1"));
    assert!(refused.contains("not a command") && refused.contains("rates"), "{refused}");
    assert!(error_of(command("@monster 1002")).contains("not a command"));

    let unknown = command("@zzzz").unwrap_err();
    assert!(unknown.contains("not a command"));
    let toggled = command("@noask").unwrap();
    assert_eq!(toggled["replies"][0], "Autorejecting is activated.");
    assert_eq!(command("@noask").unwrap()["replies"][0], "Autorejecting is deactivated.");
}
