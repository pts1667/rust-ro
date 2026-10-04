use std::sync::{Arc, Barrier};
use std::thread;

use configuration::configuration::DatabaseConfig;
use database::model::{AccountRecord, CharacterRecord, SeedData};
use database::{read, required, tx_write};
use models::enums::EnumWithMaskValueU64;
use models::enums::bonus::BonusType;
use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonus, TemporaryStatusBonuses};
use sled::transaction::Transactional;
use tokio::runtime::Runtime;

use super::*;
use crate::util::tick::get_tick;

fn catalog_item(id: i32, item_type: &str, slots: Option<i16>) -> ItemModel {
    serde_json::from_str(&format!(
        r#"{{"id":{id},"name_aegis":"Item{id}","name_english":"Item {id}","item_type":"{item_type}","weight":10,
        "price_buy":10,"price_sell":5,"slots":{},"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0}}"#,
        slots.map_or("null".into(), |slots| slots.to_string())
    ))
    .unwrap()
}

fn repository() -> SledRepository {
    let repository = SledRepository::temporary().unwrap();
    repository
        .database
        .seed(
            &SeedData {
                accounts: vec![AccountRecord {
                    account_id: 2_000_000,
                    username: "player".into(),
                    password: "password".into(),
                }],
                characters: vec![CharacterRecord {
                    char_id: 150_000,
                    account_id: 2_000_000,
                    char_num: 0,
                    name: "Player".into(),
                    zeny: 100,
                    inventory_slots: 100,
                    ..CharacterRecord::default()
                }],
                ..SeedData::default()
            },
            false,
        )
        .unwrap();
    (&repository.database.items, &repository.database.item_names)
        .transaction(|(tree, names)| {
            for item in [
                catalog_item(501, "Healing", None),
                catalog_item(2229, "Armor", Some(1)),
                catalog_item(4001, "Card", None),
            ] {
                tx_write(tree, &item.id.to_be_bytes(), &item)?;
                tx_write(names, item.name_aegis.as_bytes(), &item.id)?;
            }
            Ok(())
        })
        .unwrap();
    repository
}

fn addition(item_id: i32, amount: i16) -> InventoryItemUpdate {
    InventoryItemUpdate {
        char_id: 150_000,
        item_id,
        amount,
        identified: true,
        stackable: item_id != 2229,
        unique_id: if item_id == 2229 { 50 } else { 0 },
        ..InventoryItemUpdate::default()
    }
}

#[test]
fn purchase_failure_rolls_back_inventory_zeny_indexes_and_id_allocation() {
    let repository = repository();
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        assert!(repository.character_inventory_update_add(&[addition(501, 11)], true).await.is_err());
        assert_eq!(repository.character_zeny_fetch(150_000).await.unwrap(), 100);
        assert!(repository.character_inventory_fetch(150_000).await.unwrap().is_empty());
        assert!(repository.database.inventory_owners.is_empty());
        assert_eq!(read::<i32>(&repository.database.metadata, b"inventory_id").unwrap(), None);
    });
}

#[test]
fn purchased_items_return_persistent_ids_and_stack_without_losing_amounts() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let first = repository.character_inventory_update_add(&[addition(501, 2)], true).await.unwrap();
        let second = repository.character_inventory_update_add(&[addition(501, 3)], true).await.unwrap();
        assert!(first[0].id > 0);
        assert_eq!(first[0].id, second[0].id);
        assert_eq!(second[0].amount, 3);
        let stored = repository.character_inventory_fetch(150_000).await.unwrap();
        assert_eq!(stored[0].amount, 5);
        assert_eq!(repository.character_zeny_fetch(150_000).await.unwrap(), 50);
        assert_eq!(
            required::<i32>(&repository.database.inventory_owners, &stored[0].id.to_be_bytes()).unwrap(),
            150_000
        );
    });
}

#[test]
fn concurrent_purchases_cannot_spend_the_same_zeny_twice() {
    let repository = Arc::new(repository());
    Runtime::new()
        .unwrap()
        .block_on(repository.character_update_status(150_000, "zeny".into(), 10))
        .unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let repository = repository.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait();
                Runtime::new()
                    .unwrap()
                    .block_on(repository.character_inventory_update_add(&[addition(501, 1)], true))
            })
        })
        .collect();
    assert_eq!(
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap().is_ok())
            .filter(|success| *success)
            .count(),
        1
    );
    Runtime::new().unwrap().block_on(async {
        assert_eq!(repository.character_zeny_fetch(150_000).await.unwrap(), 0);
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap()[0].amount, 1);
    });
}

#[test]
fn failed_sale_rolls_back_prior_item_removals_and_zeny() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let mut item = repository
            .character_inventory_update_add(&[addition(501, 5)], false)
            .await
            .unwrap()
            .remove(0);
        item.amount = 4;
        let mut missing = item.clone();
        missing.id = 999;
        let removal = CharacterRemoveItem {
            char_id: 150_000,
            index: 0,
            amount: 1,
            price: 5,
        };
        assert!(
            repository
                .character_inventory_update_remove(&vec![(item, removal), (missing, removal)], true)
                .await
                .is_err()
        );
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap()[0].amount, 5);
        assert_eq!(repository.character_zeny_fetch(150_000).await.unwrap(), 100);
    });
}

#[test]
fn sale_updates_remaining_quantity_and_zeny_together() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let mut item = repository
            .character_inventory_update_add(&[addition(501, 5)], false)
            .await
            .unwrap()
            .remove(0);
        item.amount = 3;
        let removal = CharacterRemoveItem {
            char_id: 150_000,
            index: 0,
            amount: 2,
            price: 5,
        };
        repository
            .character_inventory_update_remove(&vec![(item, removal)], true)
            .await
            .unwrap();
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap()[0].amount, 3);
        assert_eq!(repository.character_zeny_fetch(150_000).await.unwrap(), 110);
    });
}

#[test]
fn card_composition_consumes_card_atomically_and_full_slot_leaves_inventory_unchanged() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let added = repository
            .character_inventory_update_add(&[addition(2229, 1), addition(4001, 2)], false)
            .await
            .unwrap();
        assert_eq!(repository.character_slot_card(150_000, &added[1], &added[0]).await.unwrap(), 0);
        let stored = repository.character_inventory_fetch(150_000).await.unwrap();
        assert_eq!(stored[0].card0, 4001);
        assert_eq!(stored[1].amount, 1);
        assert!(repository.character_slot_card(150_000, &stored[1], &stored[0]).await.is_err());
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap(), stored);
    });
}

#[test]
fn equipment_updates_keep_other_inventory_fields() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let mut equipment = repository
            .character_inventory_update_add(&[addition(2229, 1)], false)
            .await
            .unwrap()
            .remove(0);
        equipment.equip = 1;
        repository
            .character_inventory_wearable_item_update(vec![equipment.clone()])
            .await
            .unwrap();
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap(), vec![equipment]);
    });
}

#[test]
fn script_variable_scopes_types_owners_and_indexes_are_isolated() {
    let repository = repository();
    repository.script_variable_char_num_save(1, "value".into(), 0, 10);
    repository.script_variable_char_num_save(1, "value".into(), 256, 20);
    repository.script_variable_char_num_save(2, "value".into(), 0, 30);
    repository.script_variable_account_num_save(1, "value".into(), 0, 40);
    repository.script_variable_server_num_save("value".into(), 0, 50);
    repository.script_variable_char_str_save(1, "value".into(), 0, "hello".into());
    assert_eq!(repository.script_variable_char_num_fetch_all(1, "value".into()), vec![
        (0, 10),
        (256, 20)
    ]);
    assert_eq!(repository.script_variable_char_num_fetch_one(2, "value".into(), 0), 30);
    assert_eq!(repository.script_variable_account_num_fetch_one(1, "value".into(), 0), 40);
    assert_eq!(repository.script_variable_server_num_fetch_one("value".into(), 0), 50);
    assert_eq!(repository.script_variable_char_str_fetch_one(1, "value".into(), 0), "hello");
    assert_eq!(repository.script_variable_char_num_fetch_one(1, "missing".into(), 0), 0);
}

#[test]
fn persisted_bonuses_are_filtered_and_consumed_once() {
    let repository = repository();
    let mut bonuses = TemporaryStatusBonuses::default();
    bonuses.add(TemporaryStatusBonus::with_duration_and_source(
        BonusType::Str(10),
        StatusBonusFlag::Persist.as_flag(),
        get_tick(),
        60_000,
        None,
    ));
    bonuses.add(TemporaryStatusBonus::with_duration_and_source(
        BonusType::Agi(10),
        0,
        get_tick(),
        60_000,
        None,
    ));
    Runtime::new().unwrap().block_on(async {
        repository
            .character_save_temporary_bonus(150_000, 2_000_000, &bonuses)
            .await
            .unwrap();
        assert_eq!(
            repository
                .character_load_temporary_bonus(150_000, 2_000_001)
                .await
                .unwrap()
                .iter()
                .count(),
            0
        );
        assert_eq!(
            repository
                .character_load_temporary_bonus(150_000, 2_000_000)
                .await
                .unwrap()
                .iter()
                .count(),
            1
        );
        assert_eq!(
            repository
                .character_load_temporary_bonus(150_000, 2_000_000)
                .await
                .unwrap()
                .iter()
                .count(),
            0
        );
    });
}

#[test]
fn fresh_repository_seeds_catalogs_item_sources_and_accounts_once() {
    let repository = SledRepository::temporary().unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let configuration = DatabaseConfig {
        items_path: root.join("config/items.json").to_string_lossy().into_owned(),
        mobs_path: root.join("config/mobs.json").to_string_lossy().into_owned(),
        seed_path: Some(root.join("db/seed.json").to_string_lossy().into_owned()),
        ..DatabaseConfig::default()
    };
    repository.seed_assets(&configuration).unwrap();
    Runtime::new().unwrap().block_on(async {
        assert!(repository.get_all_items().await.unwrap().len() > 4_000);
        assert!(repository.get_all_mobs().await.unwrap().len() > 1_000);
        assert_eq!(repository.get_item_script(501).await.unwrap(), "itemheal rand(45,65),0;");
        assert_eq!(repository.login("admin".into(), "qwertz".into()).await.unwrap(), 2_000_000);
        assert!(repository.login("admin".into(), "wrong".into()).await.is_err());
        repository.character_update_status(150_000, "zeny".into(), 12).await.unwrap();
        repository.seed_assets(&configuration).unwrap();
        assert_eq!(repository.character_zeny_fetch(150_000).await.unwrap(), 12);
        assert!(!repository.character_inventory_fetch(150_000).await.unwrap().is_empty());
    });
}

#[test]
fn compilation_updates_roll_back_when_any_source_hash_is_wrong() {
    let repository = repository();
    let mut first = catalog_item(1001, "Etc", None);
    first.script = Some("bonus bStr,1;".into());
    let mut second = catalog_item(1002, "Etc", None);
    second.script = Some("bonus bStr,2;".into());
    repository
        .database
        .items
        .transaction(|tree| {
            tx_write(tree, &first.id.to_be_bytes(), &first)?;
            tx_write(tree, &second.id.to_be_bytes(), &second)
        })
        .unwrap();
    Runtime::new().unwrap().block_on(async {
        let updates = vec![
            (1001, vec![1], fastmurmur3::hash(first.script.as_ref().unwrap().as_bytes())),
            (1002, vec![2], 0),
        ];
        assert!(repository.update_script_compilation(updates).await.is_err());
        let stored: ItemModel = required(&repository.database.items, &1001_i32.to_be_bytes()).unwrap();
        assert!(stored.script_compilation.is_none());
    });
}

#[test]
fn hotkey_replacement_is_atomic_and_empty_save_removes_old_keys() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let hotkeys = vec![Hotkey {
            index: 1,
            is_skill: 1,
            itemskill_id: 10,
            skill_lvl: 2,
        }];
        repository.save_hotkeys(150_000, &hotkeys).await.unwrap();
        assert_eq!(repository.load_hotkeys(150_000).await.unwrap(), hotkeys);
        repository.save_hotkeys(150_000, &Vec::new()).await.unwrap();
        assert!(repository.load_hotkeys(150_000).await.unwrap().is_empty());
    });
}

#[test]
fn queued_item_consumption_preserves_intervening_inventory_additions() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let item = repository
            .character_inventory_update_add(&[addition(501, 5)], false)
            .await
            .unwrap()
            .remove(0);
        repository.character_inventory_update_add(&[addition(501, 3)], false).await.unwrap();
        repository
            .character_inventory_delete(DeleteItems {
                char_id: 150_000,
                item_inventory_id: item.id,
                unique_id: 0,
                amount_to_remove: 1,
            })
            .await
            .unwrap();
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap()[0].amount, 7);
        assert!(
            repository
                .character_inventory_delete(DeleteItems {
                    char_id: 150_000,
                    item_inventory_id: item.id,
                    unique_id: 0,
                    amount_to_remove: 8,
                })
                .await
                .is_err()
        );
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap()[0].amount, 7);
    });
}

#[test]
fn item_lookup_supports_ids_case_insensitive_names_and_wildcards_without_duplicates() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let items = repository
            .get_items(vec![
                Value::Number(501),
                Value::String("item50_".into()),
                Value::String("ITEM22%".into()),
            ])
            .await
            .unwrap();
        assert_eq!(items.iter().map(|item| item.id).collect::<Vec<_>>(), vec![501, 2229]);
        assert_eq!(repository.get_items(vec![Value::Number(4001)]).await.unwrap()[0].id, 4001);
    });
}

#[test]
fn adding_equipment_preserves_refinement_damage_and_cards() {
    let repository = repository();
    Runtime::new().unwrap().block_on(async {
        let mut update = addition(2229, 1);
        update.refine = 7;
        update.damaged = true;
        update.cards[0] = 4001;
        let added = repository.character_inventory_update_add(&[update], false).await.unwrap();
        assert_eq!(added[0].refine, 7);
        assert!(added[0].is_damaged);
        assert_eq!(added[0].card0, 4001);
        assert_eq!(repository.character_inventory_fetch(150_000).await.unwrap(), added);
    });
}
