//! Carts and stores through the bot API: each action answers with what the game did or why it refused.
use models::enums::EnumWithMaskValueU64;
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;

use database::model::InventoryRecord;

use super::ServerServiceTestContext;
use super::player_trade_tests::fixture;
use crate::repository::{InventoryRepository, SledRepository};
use crate::server::bots::{BuyingLine, CartMove, StoreAction, VendingLine};
use crate::server::model::game_systems::PlayerOption;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;

const FIRST: u32 = 150_000;
const SECOND: u32 = 150_001;
const POTION_INDEX: usize = 1;

fn on<R>(context: &ServerServiceTestContext, char_id: u32, change: impl FnOnce(&mut Character) -> R) -> R {
    let mut state = context.server.state_mut();
    change(state.characters_mut().get_mut(&char_id).unwrap())
}

fn store(context: &ServerServiceTestContext, char_id: u32, action: StoreAction) -> Result<serde_json::Value, String> {
    let mut state = context.server.state_mut();
    context.server.bot_store(&mut state, char_id, &action, 1_000)
}

fn error_of<T: std::fmt::Debug>(result: Result<T, String>) -> String {
    result.expect_err("the action should have been refused")
}

fn give_cart(context: &ServerServiceTestContext, char_id: u32) {
    let flag = PlayerOption::Cart1.as_flag();
    context.server.script_world_service().repository.set_character_options(char_id, 0, flag).unwrap();
    on(context, char_id, |character| character.set_options(flag));
}

fn stand(context: &ServerServiceTestContext, char_id: u32, x: u16, y: u16) {
    on(context, char_id, |character| character.update_position(x, y));
}

#[test]
fn a_cart_takes_items_only_from_a_character_that_has_one() {
    let (context, _) = fixture();
    let put = |amount| store(&context, FIRST, StoreAction::CartMove { how: CartMove::Put, index: POTION_INDEX, amount });
    assert!(error_of(put(1)).contains("no cart"));

    give_cart(&context, FIRST);
    assert!(error_of(put(0)).contains("1 to 5"));
    assert!(error_of(put(6)).contains("1 to 5"));
    assert!(error_of(store(&context, FIRST, StoreAction::CartMove { how: CartMove::Put, index: 9, amount: 1 })).contains("No item at inventory index 9"));
    assert!(error_of(store(&context, FIRST, StoreAction::CartMove { how: CartMove::Get, index: 0, amount: 1 })).contains("No item at cart index 0"));
    assert!(error_of(store(&context, FIRST, StoreAction::CartMove { how: CartMove::ToStorage, index: 0, amount: 1 })).contains("No item at cart index 0"));
    assert!(error_of(store(&context, FIRST, StoreAction::CartMove { how: CartMove::FromStorage, index: 0, amount: 1 })).contains("No storage is open"));

    assert_eq!(put(3).unwrap()["amount"], 3);
    let (cart, kept) = on(&context, FIRST, |character| {
        (
            character.game_systems.cart_items.iter().map(|record| (record.item_id, record.amount)).collect::<Vec<_>>(),
            character.get_item_from_inventory(POTION_INDEX).map(|item| item.amount),
        )
    });
    assert_eq!((cart, kept), (vec![(501, 3)], Some(2)));

    store(&context, FIRST, StoreAction::CartMove { how: CartMove::Get, index: 0, amount: 3 }).unwrap();
    assert!(on(&context, FIRST, |character| character.game_systems.cart_items.is_empty()));
}

#[test]
fn a_vendor_sells_to_a_close_buyer_and_every_refusal_says_why() {
    let (context, _) = fixture();
    let open = |lines: Vec<VendingLine>| store(&context, FIRST, StoreAction::OpenVending { title: "Potions".into(), lines });
    let line = |amount, price| VendingLine { cart_index: 0, amount, price };
    assert!(error_of(open(vec![line(1, 10)])).contains("needs a cart"));

    give_cart(&context, FIRST);
    assert!(error_of(open(vec![line(1, 10)])).contains("not learned"));
    on(&context, FIRST, |character| character.status.known_skills.push(KnownSkill { value: SkillEnum::McVending, level: 1 }));
    assert!(error_of(open(vec![line(1, 10)])).contains("No item at cart index 0"));

    store(&context, FIRST, StoreAction::CartMove { how: CartMove::Put, index: POTION_INDEX, amount: 4 }).unwrap();
    assert!(error_of(open(vec![])).contains("1 to 3 different items"));
    assert!(error_of(open(vec![line(5, 10)])).contains("1 to 4"));
    assert!(error_of(open(vec![line(1, 0)])).contains("price goes from 1"));
    assert!(error_of(open(vec![line(1, 10), line(1, 10)])).contains("listed twice"));
    assert!(error_of(store(&context, FIRST, StoreAction::OpenVending { title: " ".into(), lines: vec![line(1, 10)] })).contains("title"));

    assert_eq!(open(vec![line(3, 20)]).unwrap()["items"], 1);
    assert!(error_of(open(vec![line(1, 10)])).contains("already open"));

    stand(&context, FIRST, 100, 100);
    stand(&context, SECOND, 100, 110);
    let buy = |vendor, lines: Vec<(u16, u16)>| store(&context, SECOND, StoreAction::BuyFromVendor { vendor, lines });
    assert!(error_of(buy(FIRST, vec![(0, 1)])).contains("10 cells away"));
    stand(&context, SECOND, 103, 100);
    assert!(error_of(buy(SECOND, vec![(0, 1)])).contains("No player"));
    assert!(error_of(buy(FIRST, vec![])).contains("at least one"));
    assert!(error_of(buy(FIRST, vec![(4, 1)])).contains("does not sell index 4"));
    assert!(error_of(buy(FIRST, vec![(0, 4)])).contains("1 to 3"));
    assert!(error_of(buy(FIRST, vec![(0, 3), (0, 1)])).contains("listed twice"));
    assert!(error_of(buy(FIRST, vec![(0, 3)])).contains("costs 60 zeny, the character has 50"));

    let bought = buy(FIRST, vec![(0, 2)]).unwrap();
    assert_eq!(bought["spent"], 40);
    assert_eq!(bought["zeny"], 10);
    let potions = on(&context, SECOND, |character| character.inventory.iter().flatten().filter(|item| item.item_id == 501).map(|item| item.amount).sum::<i16>());
    assert_eq!(potions, 6);
    assert_eq!(on(&context, FIRST, |character| character.game_systems.vending_store.as_ref().map(|store| store.offers[0].amount)), Some(1));

    store(&context, FIRST, StoreAction::CloseVending).unwrap();
    assert!(error_of(store(&context, FIRST, StoreAction::CloseVending)).contains("No vending store"));
    assert!(error_of(buy(FIRST, vec![(0, 1)])).contains("no vending store"));
}

const JELLOPY: i32 = 909;

/// Both traders hold Jellopy, which buying stores accept; potions are not allowed in them.
fn give_jellopy(context: &ServerServiceTestContext, repository: &SledRepository) {
    repository
        .database
        .items
        .transaction(|tree| database::tx_write(tree, &JELLOPY.to_be_bytes(), GlobalConfigService::instance().get_item(JELLOPY)))
        .unwrap();
    repository
        .database
        .inventories
        .transaction(|tree| {
            for char_id in [FIRST, SECOND] {
                let mut records: Vec<InventoryRecord> = database::tx_required(tree, &char_id.to_be_bytes())?;
                records.push(InventoryRecord { id: 30 + char_id as i32, item_id: JELLOPY, amount: 6, is_identified: true, ..Default::default() });
                database::tx_write(tree, &char_id.to_be_bytes(), &records)?;
            }
            Ok(())
        })
        .unwrap();
    for char_id in [FIRST, SECOND] {
        let inventory = context.runtime().block_on(repository.character_inventory_fetch(char_id as i32)).unwrap();
        on(context, char_id, |character| character.inventory = inventory.into_iter().map(Some).collect());
    }
}

#[test]
fn a_buyer_pays_for_the_items_it_asked_for_and_nothing_else() {
    let (context, repository) = fixture();
    give_jellopy(&context, &repository);
    let open = |lines: Vec<BuyingLine>, zeny_limit| store(&context, SECOND, StoreAction::OpenBuying { title: "Wanted".into(), zeny_limit, lines });
    let wanted = |amount, price| BuyingLine { item_id: JELLOPY, amount, price };
    assert!(error_of(open(vec![wanted(2, 5)], 40)).contains("Purchase Street Stall License"));

    on(&context, SECOND, |character| character.game_systems.buying_slots = 2);
    assert!(error_of(open(vec![], 40)).contains("1 to 2 different items"));
    assert!(error_of(open(vec![wanted(2, 5)], 0)).contains("zeny limit"));
    assert!(error_of(open(vec![wanted(2, 5)], 51)).contains("zeny limit"));
    assert!(error_of(open(vec![BuyingLine { item_id: 999_999, amount: 1, price: 5 }], 40)).contains("Unknown item"));
    assert!(error_of(open(vec![BuyingLine { item_id: 501, amount: 1, price: 5 }], 40)).contains("can not be asked for"));
    assert!(error_of(open(vec![wanted(10_000, 5)], 40)).contains("1 to 9999"));
    assert!(error_of(open(vec![wanted(2, 0)], 40)).contains("price goes from 1"));
    assert!(error_of(open(vec![wanted(2, 5), wanted(1, 5)], 40)).contains("listed twice"));
    assert_eq!(open(vec![wanted(2, 5)], 40).unwrap()["items"], 1);

    stand(&context, FIRST, 100, 100);
    stand(&context, SECOND, 100, 110);
    let jellopy_index = on(&context, FIRST, |character| character.inventory.iter().position(|item| item.as_ref().is_some_and(|item| item.item_id == JELLOPY)).unwrap());
    let sell = |lines: Vec<(usize, u16)>| store(&context, FIRST, StoreAction::SellToBuyer { buyer: SECOND, lines });
    assert!(error_of(sell(vec![(jellopy_index, 1)])).contains("10 cells away"));
    stand(&context, FIRST, 100, 108);
    assert!(error_of(sell(vec![])).contains("at least one"));
    assert!(error_of(sell(vec![(99, 1)])).contains("No item at inventory index 99"));
    assert!(error_of(sell(vec![(POTION_INDEX, 1)])).contains("does not buy"));
    assert!(error_of(sell(vec![(jellopy_index, 3)])).contains("1 to 2"));
    assert!(error_of(sell(vec![(jellopy_index, 1), (jellopy_index, 1)])).contains("listed twice"));

    let sold = sell(vec![(jellopy_index, 2)]).unwrap();
    assert_eq!(sold["earned"], 10);
    assert_eq!(on(&context, FIRST, |character| character.get_item_from_inventory(jellopy_index).map(|item| item.amount)), Some(4));
    assert!(on(&context, SECOND, |character| character.game_systems.buying_store.is_none()));
}
