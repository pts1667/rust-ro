//! What the bot API answers when an action can not be done: the character is left alone and the call says why.
use super::player_trade_tests::fixture;
use super::ServerServiceTestContext;
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;

use crate::server::bots::{
    ChatAction, CraftAction, DuelRequest, MenuAnswer, PartyAction, ProgressAction, SkillMenuAction, SkillRef, Stat, StorageAction, TradeAction, observe,
};
use crate::server::model::map_flags::MapFlags;
use crate::server::script::game_data::data;
use crate::server::script::skill::PendingTeleportMenu;
use crate::server::service::script_crafting_service::CraftSession;
use crate::server::service::skill_menu_service::{SkillMenu, SkillMenuKind};
use crate::server::state::character::Character;

const FIRST: u32 = 150_000;
const SECOND: u32 = 150_001;
const FIRST_ACCOUNT: u32 = 2_000_000;
const POTION: i32 = 501;

fn first<R>(context: &ServerServiceTestContext, change: impl FnOnce(&mut Character) -> R) -> R {
    let mut state = context.server.state_mut();
    change(state.characters_mut().get_mut(&FIRST).unwrap())
}

fn error_of<T: std::fmt::Debug>(result: Result<T, String>) -> String {
    result.expect_err("the action should have been refused")
}

#[test]
fn stat_points_are_spent_at_once_or_refused_with_the_reason() {
    let (context, _) = fixture();
    let raise = |amount| {
        let mut state = context.server.state_mut();
        context.server.bot_progress(&mut state, FIRST, &ProgressAction::RaiseStat { stat: Stat::Str, amount })
    };
    first(&context, |character| character.status.status_point = 10);
    let before = first(&context, |character| character.status.str);

    let spent = raise(2).unwrap();
    assert_eq!(spent["value"], before + 2);
    assert_eq!(first(&context, |character| (character.status.str, character.status.status_point)), (before + 2, spent["status_points"].as_u64().unwrap() as u32));
    assert!(spent["status_points"].as_u64().unwrap() < 10);

    assert!(error_of(raise(0)).contains("at least 1"));
    assert!(error_of(raise(200)).contains("can not go above"));
    let held = first(&context, |character| character.status.str);
    first(&context, |character| character.status.status_point = 0);
    assert!(error_of(raise(1)).contains("no status point"));
    assert_eq!(first(&context, |character| character.status.str), held);
}

#[test]
fn skill_points_are_refused_with_the_reason_a_player_would_need() {
    let (context, _) = fixture();
    let learn = |skill: SkillRef| {
        let mut state = context.server.state_mut();
        context.server.bot_progress(&mut state, FIRST, &ProgressAction::LearnSkill { skill })
    };
    assert!(error_of(learn(SkillRef::Name("NV_BASIC".into()))).contains("no skill point"));

    first(&context, |character| character.status.skill_point = 3);
    assert!(error_of(learn(SkillRef::Name("XX_FAKE".into()))).contains("Unknown skill"));
    assert!(error_of(learn(SkillRef::Name("GD_APPROVAL".into()))).contains("guild skill"));
    let first_aid = error_of(learn(SkillRef::Name("NV_FIRSTAID".into())));
    assert!(first_aid.contains("can not be learned yet") && first_aid.contains("NV_BASIC"), "{first_aid}");
    assert_eq!(first(&context, |character| character.status.skill_point), 3);
}

#[test]
fn a_trade_request_needs_a_close_player_and_offers_need_what_they_name() {
    let (context, _) = fixture();
    let trade = |action: TradeAction| {
        let state = context.server.state();
        context.server.bot_trade(&state, FIRST, FIRST_ACCOUNT, &action)
    };
    {
        let mut state = context.server.state_mut();
        state.characters_mut().get_mut(&FIRST).unwrap().update_position(100, 100);
        state.characters_mut().get_mut(&SECOND).unwrap().update_position(100, 110);
    }
    let far = error_of(trade(TradeAction::Request { target: SECOND }));
    assert!(far.contains("10 cells away"), "{far}");
    assert!(error_of(trade(TradeAction::Request { target: FIRST })).contains("No player"));

    context.server.state_mut().characters_mut().get_mut(&SECOND).unwrap().update_position(101, 100);
    assert!(trade(TradeAction::Request { target: SECOND }).is_ok());

    // An offer needs a trade, but the checks on the item come first
    assert!(error_of(trade(TradeAction::OfferItem { index: 9, amount: 1 })).contains("No item"));
    assert!(error_of(trade(TradeAction::OfferItem { index: 1, amount: 6 })).contains("1 to 5"));
    assert!(error_of(trade(TradeAction::OfferItem { index: 1, amount: 0 })).contains("1 to 5"));
    let zeny = error_of(trade(TradeAction::OfferZeny { amount: 101 }));
    assert!(zeny.contains("100 zeny"), "{zeny}");

    first(&context, |character| character.inventory[0].as_mut().unwrap().equip = 2);
    assert!(error_of(trade(TradeAction::OfferItem { index: 0, amount: 1 })).contains("Take the item off"));
}

#[test]
fn storage_moves_items_only_while_open_and_says_what_went_wrong() {
    let (context, _) = fixture();
    let storage = |action: StorageAction| {
        let mut state = context.server.state_mut();
        context.server.bot_storage(&mut state, FIRST, &action)
    };
    assert!(error_of(storage(StorageAction::Deposit { index: 1, amount: 1 })).contains("talk to a Kafra"));
    assert!(error_of(storage(StorageAction::Close)).contains("talk to a Kafra"));

    first(&context, |character| character.game_systems.storage_open = true);
    assert!(error_of(storage(StorageAction::Deposit { index: 9, amount: 1 })).contains("No item at inventory index 9"));
    assert!(error_of(storage(StorageAction::Deposit { index: 1, amount: 6 })).contains("1 to 5"));
    assert!(error_of(storage(StorageAction::Withdraw { index: 0, amount: 1 })).contains("No item at storage index 0"));

    let deposited = storage(StorageAction::Deposit { index: 1, amount: 2 }).unwrap();
    assert_eq!(deposited["amount"], 2);
    let (stored, kept) = first(&context, |character| {
        (character.game_systems.storage_items.iter().map(|record| (record.item_id, record.amount)).collect::<Vec<_>>(), character.get_item_from_inventory(1).map(|item| item.amount))
    });
    assert_eq!((stored, kept), (vec![(POTION, 2)], Some(3)));

    storage(StorageAction::Withdraw { index: 0, amount: 2 }).unwrap();
    assert!(first(&context, |character| character.game_systems.storage_items.is_empty()));
    assert_eq!(first(&context, |character| character.get_item_from_inventory(1).map(|item| item.amount)), Some(5));

    storage(StorageAction::Close).unwrap();
    assert!(!first(&context, |character| character.game_systems.storage_open));
}

#[test]
fn chat_and_parties_name_players_who_are_in_the_game() {
    let (context, _) = fixture();
    let state = context.server.state();
    let whisper = |to: &str| context.server.bot_chat(&state, FIRST, &ChatAction::Whisper { to: to.into(), text: "hi".into() });
    assert!(whisper("Trader B").is_ok());
    assert!(whisper("trader b").is_ok());
    assert!(error_of(whisper("Nobody")).contains("Nobody is not in the game"));
    assert!(error_of(context.server.bot_chat(&state, FIRST, &ChatAction::Say("@warp prontera".into()))).contains("commands"));
    assert!(error_of(context.server.bot_party(&state, FIRST, &PartyAction::Invite { name: "Trader B".into() })).contains("Create a party"));
}

const TICK: u128 = 1_000;

fn menu_answer(context: &ServerServiceTestContext, action: SkillMenuAction) -> Result<serde_json::Value, String> {
    let mut state = context.server.state_mut();
    context.server.bot_skill_menu(&mut state, FIRST_ACCOUNT, FIRST, &action, TICK)
}

fn seen_by_first(context: &ServerServiceTestContext) -> serde_json::Value {
    let state = context.server.state();
    serde_json::to_value(observe(&context.server, &state, FIRST, TICK).unwrap()).unwrap()
}

#[test]
fn a_teleport_menu_is_answered_with_a_map_it_offered() {
    let (context, _) = fixture();
    assert!(error_of(menu_answer(&context, SkillMenuAction::Cancel)).contains("No skill is waiting"));
    assert!(seen_by_first(&context)["skill_menu"].is_null());

    first(&context, |character| {
        character.script_skill_state.pending_teleport = Some(PendingTeleportMenu {
            skill_id: SkillEnum::AlTeleport.id(),
            level: 2,
            origin_map: character.map_instance_key.clone(),
            save_map: "prontera".into(),
        });
    });
    let menu = seen_by_first(&context)["skill_menu"].clone();
    assert_eq!(menu["kind"], "teleport");
    assert_eq!(menu["choices"], serde_json::json!([{ "choice": "Random", "name": "Random" }, { "choice": "prontera", "name": "prontera" }]));

    assert!(error_of(menu_answer(&context, SkillMenuAction::Choose(MenuAnswer::Number(1)))).contains("name of a map"));
    let refused = error_of(menu_answer(&context, SkillMenuAction::Choose(MenuAnswer::Text("geffen".into()))));
    assert!(refused.contains("geffen is not offered") && refused.contains("Random, prontera"), "{refused}");
    assert!(first(&context, |character| character.script_skill_state.pending_teleport.is_some()));

    assert_eq!(menu_answer(&context, SkillMenuAction::Cancel).unwrap()["cancelled"], true);
    assert!(first(&context, |character| character.script_skill_state.pending_teleport.is_none()));
}

#[test]
fn an_item_menu_lists_its_choices_and_refuses_what_it_did_not_offer() {
    let (context, _) = fixture();
    first(&context, |character| {
        character.status.known_skills.push(KnownSkill { value: SkillEnum::MgFirebolt, level: 3 });
        character.pending_skill_menu = Some(SkillMenu { kind: SkillMenuKind::AutoSpell, skill_id: SkillEnum::SaAutospell.id(), level: 5, expires_at: TICK + 1 });
    });
    let menu = seen_by_first(&context)["skill_menu"].clone();
    assert_eq!(menu["kind"], "auto_spell");
    assert_eq!(menu["choices"][0]["choice"], SkillEnum::MgFirebolt.id());

    assert!(error_of(menu_answer(&context, SkillMenuAction::Choose(MenuAnswer::Text("MG_FIREBOLT".into())))).contains("number of a choice"));
    let refused = error_of(menu_answer(&context, SkillMenuAction::Choose(MenuAnswer::Number(20))));
    assert!(refused.contains("20 is not offered") && refused.contains(&SkillEnum::MgFirebolt.id().to_string()), "{refused}");
    assert!(first(&context, |character| character.pending_skill_menu.is_some()));

    first(&context, |character| character.pending_skill_menu.as_mut().unwrap().expires_at = TICK);
    assert!(seen_by_first(&context)["skill_menu"].is_null());

    first(&context, |character| character.pending_skill_menu.as_mut().unwrap().expires_at = TICK + 1);
    assert_eq!(menu_answer(&context, SkillMenuAction::Cancel).unwrap()["cancelled"], true);
    assert!(first(&context, |character| character.pending_skill_menu.is_none()));
}

#[test]
fn a_crafting_window_only_makes_what_it_lists_and_what_the_materials_allow() {
    let (context, _) = fixture();
    let craft = |item_id, additives: Vec<i32>| {
        let mut state = context.server.state_mut();
        context.server.bot_craft(&mut state, FIRST, &CraftAction { item_id, additives }, TICK)
    };
    assert!(error_of(craft(POTION, vec![])).contains("No crafting window"));

    let recipe = data().recipes.iter().find(|recipe| recipe.skill_id == 0 && !recipe.materials.is_empty()).expect("a recipe without a skill").clone();
    first(&context, |character| {
        character.pending_craft = Some(CraftSession { trigger: recipe.level, cooking: false, recipes: vec![recipe.id], expires_at: u128::MAX });
    });
    let window = seen_by_first(&context)["crafting"].clone();
    assert_eq!(window["recipes"][0]["item_id"], recipe.item_id);
    assert_eq!(window["recipes"][0]["materials"][0]["item_id"], recipe.materials[0].item_id);

    assert!(error_of(craft(recipe.item_id + 1, vec![])).contains("not in the window"));
    let without_materials = error_of(craft(recipe.item_id, vec![]));
    assert!(without_materials.contains("unavailable"), "{without_materials}");
    assert!(error_of(craft(recipe.item_id, vec![1; 4])).contains("At most 3"));
}

#[test]
fn players_fight_in_a_duel_and_nowhere_else_on_a_peaceful_map() {
    let (context, _) = fixture();
    context.server.map_flag_overrides().insert(("empty".into(), 0), MapFlags::default());
    {
        let mut state = context.server.state_mut();
        state.characters_mut().get_mut(&FIRST).unwrap().update_position(100, 100);
        state.characters_mut().get_mut(&SECOND).unwrap().update_position(101, 100);
    }
    let attack = || context.server.bot_attack(&context.server.state(), FIRST, SECOND);
    let duel = |char_id: u32, request: DuelRequest| {
        let mut state = context.server.state_mut();
        context.server.bot_duel(&mut state, char_id, &request)
    };
    let attackable = || seen_by_first(&context)["players"][0]["attackable"].clone();

    let at_peace = error_of(attack());
    assert!(at_peace.contains("Players can only fight"), "{at_peace}");
    assert_eq!(attackable(), false);

    assert!(error_of(duel(FIRST, DuelRequest::Invite { name: "Trader B".into() })).contains("Create a duel"));
    assert!(error_of(duel(SECOND, DuelRequest::Accept)).contains("No duel invitation"));
    assert!(error_of(duel(FIRST, DuelRequest::Leave)).contains("not in a duel"));
    duel(FIRST, DuelRequest::Create { limit: 0 }).unwrap();
    assert!(error_of(duel(FIRST, DuelRequest::Create { limit: 0 })).contains("already in a duel"));
    assert!(error_of(duel(FIRST, DuelRequest::Invite { name: "Nobody".into() })).contains("not in the game"));
    assert!(error_of(duel(FIRST, DuelRequest::Invite { name: "Trader A".into() })).contains("not another player"));
    assert_eq!(duel(FIRST, DuelRequest::Invite { name: "trader b".into() }).unwrap()["invited"], "Trader B");

    let accepted = duel(SECOND, DuelRequest::Accept).unwrap();
    assert!(accepted["duel"].is_number());
    assert_eq!(attackable(), true);
    assert_eq!(attack().unwrap()["in_range"], true);

    duel(SECOND, DuelRequest::Leave).unwrap();
    assert!(error_of(attack()).contains("Players can only fight"));
}
