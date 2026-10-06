use std::sync::Arc;

use database::model::{CharacterRecord, InventoryRecord};
use sled::transaction::Transactional;

use crate::repository::{InventoryRepository, SledRepository};
use crate::server::model::events::game_event::{CharacterUseItem, GameEvent, ScriptIdentify};
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_effect_service::validate_item_map_flags;
use crate::server::state::character::Character;

fn item_fixture(item_id: i32) -> (super::ServerServiceTestContext, Arc<SledRepository>, Character) {
    let (context, repository, mut character) = super::native_payment_tests::fixture(false, true);
    character.status.hp = 1;
    let record = InventoryRecord {
        id: 10, item_id, amount: 2, is_identified: true, unique_id: 8181,
        refine: 7, is_damaged: true, card0: 4001, ..InventoryRecord::default()
    };
    (&repository.database.characters, &repository.database.inventories, &repository.database.inventory_owners, &repository.database.items)
        .transaction(|(characters, inventories, owners, items)| {
            let mut stored: CharacterRecord = database::tx_required(characters, &character.char_id.to_be_bytes())?;
            stored.hp = 1;
            database::tx_write(characters, &character.char_id.to_be_bytes(), &stored)?;
            database::tx_write(inventories, &character.char_id.to_be_bytes(), &vec![record.clone()])?;
            database::tx_write(owners, &10_i32.to_be_bytes(), &(character.char_id as i32))?;
            database::tx_write(items, &item_id.to_be_bytes(), GlobalConfigService::instance().get_item(item_id))
        }).unwrap();
    character.add_items(context.runtime().block_on(repository.character_inventory_fetch(character.char_id as i32)).unwrap());
    (context, repository, character)
}

fn set_flag(context: &super::ServerServiceTestContext, flag: MapFlag) {
    let mut flags = MapFlags::default();
    flags.set(flag, true, &[]).unwrap();
    context.server.map_flag_overrides().insert(("empty".into(), 0), flags);
}

#[test]
fn forbidden_consumables_preserve_the_exact_source_and_all_resources() {
    for (item_id, flag) in [
        (501, MapFlag::NoItemConsumption),
        (604, MapFlag::NoBranch),
        (604, MapFlag::Gvg),
        (601, MapFlag::NoTeleport),
        (601, MapFlag::Gvg),
        (602, MapFlag::NoReturn),
    ] {
        let (context, repository, mut character) = item_fixture(item_id);
        set_flag(&context, flag);
        let before: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
        let action = CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 };
        context.server.item_service().use_item_in_state(&context.server, &mut context.server.state(), context.runtime(), action, &mut character);
        let after: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
        assert_eq!(after, before, "{item_id} on {flag:?}");
        assert_eq!((character.status.hp, character.status.sp, character.status.zeny), (1, 1000, 0));
        assert_eq!(character.get_item_from_inventory(0).unwrap().amount, 2);
        assert!(character.pending_item_skill.is_none());
        assert!(context.server.pop_task().is_none());
    }
}

#[test]
fn ordinary_healing_commits_once_after_item_use_is_reenabled() {
    let (context, repository, mut character) = item_fixture(501);
    set_flag(&context, MapFlag::NoItemConsumption);
    let action = CharacterUseItem { char_id: character.char_id, target_char_id: character.char_id, index: 0 };
    context.server.item_service().use_item_in_state(&context.server, &mut context.server.state(), context.runtime(), action.clone(), &mut character);
    context.server.map_flag_overrides().insert(("empty".into(), 0), MapFlags::default());
    context.server.item_service().use_item_in_state(&context.server, &mut context.server.state(), context.runtime(), action, &mut character);
    let stored: CharacterRecord = database::required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
    let inventory: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
    assert!(character.status.hp > 1);
    assert_eq!(stored.hp, character.status.hp as i32);
    assert_eq!(inventory[0].amount, 1);
    assert_eq!(inventory[0].unique_id, 8181);
    assert_eq!(inventory[0].card0, 4001);
}

#[test]
fn item_map_groups_restrict_the_appropriate_kind_of_warp() {
    crate::tests::common::before_all();
    let config = GlobalConfigService::instance();
    let mut flags = MapFlags::default();
    flags.set(MapFlag::NoReturn, true, &[]).unwrap();
    assert!(validate_item_map_flags(&flags, config.get_item(602)).is_err());
    assert!(validate_item_map_flags(&flags, config.get_item(601)).is_ok());
    flags.set(MapFlag::NoReturn, false, &[]).unwrap();
    flags.set(MapFlag::NoTeleport, true, &[]).unwrap();
    assert!(validate_item_map_flags(&flags, config.get_item(601)).is_err());
    assert!(validate_item_map_flags(&flags, config.get_item(602)).is_ok());
    assert!(validate_item_map_flags(&flags, config.get_item(501)).is_ok());
    assert!(crate::server::script::game_data::item_in_use_group(12212, "GIANT_FLY_WING"));
}

#[test]
fn map_change_after_magnifier_targeting_keeps_source_and_target_unchanged() {
    let (context, repository, character, selected) = super::script_operation_tests::identification_fixture(true);
    let char_id = character.char_id;
    let before: Vec<InventoryRecord> = database::required(&repository.database.inventories, &char_id.to_be_bytes()).unwrap();
    set_flag(&context, MapFlag::NoItemConsumption);
    context.server.state_mut().insert_character(character);
    assert!(context.server.handle_script_event(&mut *context.server.state_mut(), GameEvent::ScriptIdentify(ScriptIdentify { char_id, index: selected }), 101).is_err());
    let after: Vec<InventoryRecord> = database::required(&repository.database.inventories, &char_id.to_be_bytes()).unwrap();
    assert_eq!(after, before);
    assert!(!after[selected].is_identified);
    assert_eq!(context.server.state().characters().get(&char_id).unwrap().status.sp, 1000);
}

#[test]
fn map_change_before_delayed_item_payment_preserves_costs_and_source() {
    let (context, repository, mut character, _) = super::script_operation_tests::identification_fixture(true);
    let before: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
    set_flag(&context, MapFlag::NoItemConsumption);
    let cost = crate::server::script::skill::requirements::SkillRequirementPlan { minimum_hp: 1, hp: 1, sp: 7, ..Default::default() };
    assert!(context.server.item_service().pay_requirement_plan_in_state(&context.server, &context.server.state(), &mut character, &cost, Some(0), 101).is_err());
    let after: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!(after, before);
    assert_eq!((character.status.hp, character.status.sp), (1000, 1000));
    let stored: CharacterRecord = database::required(&repository.database.characters, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!((stored.hp, stored.sp), (1000, 1000));
}

#[test]
fn accepted_trade_guards_preserve_items_and_resources_and_requested_trade_allows_use() {
    use crate::server::model::events::game_event::{CharacterRemoveItem, NpcContact};
    use crate::server::model::game_systems::{PlayerTrade, PlayerTradePhase};
    let (context, repository, mut character) = item_fixture(501);
    let char_id = character.char_id;
    let account_id = character.account_id;
    character.game_systems.trade = Some(PlayerTrade { session_id: 10, partner_id: char_id + 1, requested_by: char_id,
        phase: PlayerTradePhase::Accepted, items: vec![], zeny: 0, requested_at: 0 });
    let action = CharacterUseItem { char_id, target_char_id: char_id, index: 0 };
    context.server.item_service().use_item_in_state(&context.server, &mut context.server.state(), context.runtime(), action.clone(), &mut character);
    let cost = crate::server::script::skill::requirements::SkillRequirementPlan { minimum_hp: 1, sp: 7, ..Default::default() };
    assert!(context.server.item_service().pay_requirement_plan(&context.server, &mut character, &cost, None, 101).is_err());
    assert_eq!((character.status.hp, character.status.sp), (1, 1000));
    context.server.state_mut().insert_character(character);
    assert!(!context.server.server_service().character_drop_item(&context.server, &mut *context.server.state_mut(),
        CharacterRemoveItem { char_id, index: 0, amount: 1, price: 0 }).unwrap());
    let error = context.server.start_npc_conversation(&context.server.state(), NpcContact { char_id, account_id, npc_id: 0 }).unwrap_err();
    assert_eq!(error, "NPC visitor is unavailable");
    let saved: Vec<InventoryRecord> = database::required(&repository.database.inventories, &char_id.to_be_bytes()).unwrap();
    assert_eq!((saved[0].amount, saved[0].unique_id), (2, 8181));
    assert!(context.server.pop_task().is_none());
    let mut character = context.server.state_mut().characters_mut().remove(&char_id).unwrap();
    character.game_systems.trade.as_mut().unwrap().phase = PlayerTradePhase::Requested;
    context.server.item_service().use_item_in_state(&context.server, &mut context.server.state(), context.runtime(), action, &mut character);
    let saved: Vec<InventoryRecord> = database::required(&repository.database.inventories, &char_id.to_be_bytes()).unwrap();
    assert_eq!(saved[0].amount, 1);
    assert!(character.status.hp > 1);
}
