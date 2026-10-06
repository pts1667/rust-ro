use std::sync::{Arc, Barrier};
use std::thread;

use model::{AccountRecord, CharacterInventory, CharacterRecord, InventoryRecord, SeedData};

use super::*;

fn character(id: i32, name: &str, slot: i16) -> CharacterRecord {
    CharacterRecord {
        char_id: id,
        account_id: 2_000_000,
        char_num: slot,
        name: name.into(),
        inventory_slots: 100,
        base_level: 1,
        job_level: 1,
        ..CharacterRecord::default()
    }
}

fn seeded_database() -> Database {
    let database = Database::temporary().unwrap();
    database
        .seed(
            &SeedData {
                accounts: vec![AccountRecord {
                    account_id: 2_000_000,
                    username: "player".into(),
                    password: "password".into(), ..Default::default()
                }],
                ..SeedData::default()
            },
            false,
        )
        .unwrap();
    database
}

#[test]
fn conflicting_character_keeps_indexes_and_id_sequence_unchanged() {
    let database = seeded_database();
    let id = database.insert_character(&character(0, "First", 0)).unwrap();
    assert!(matches!(
        database.insert_character(&character(0, "Second", 0)),
        Err(DatabaseError::Conflict(_))
    ));
    assert!(matches!(
        database.insert_character(&character(0, "First", 1)),
        Err(DatabaseError::Conflict(_))
    ));
    assert_eq!(read::<i32>(&database.metadata, b"char_id").unwrap(), Some(id));
    assert!(database.character_names.get(b"Second").unwrap().is_none());
    let next = database.insert_character(&character(0, "Second", 1)).unwrap();
    assert_eq!(next, id + 1);
}

#[test]
fn invalid_inventory_rolls_back_seed_accounts_characters_and_indexes() {
    let database = Database::temporary().unwrap();
    let result = database.seed(
        &SeedData {
            accounts: vec![AccountRecord {
                account_id: 2_000_000,
                username: "player".into(),
                password: "password".into(), ..Default::default()
            }],
            characters: vec![character(150_000, "First", 0)],
            inventories: vec![CharacterInventory {
                char_id: 150_000,
                items: vec![InventoryRecord {
                    item_id: 501,
                    amount: -1,
                    ..InventoryRecord::default()
                }],
            }],
            ..SeedData::default()
        },
        false,
    );
    assert!(result.is_err());
    assert!(database.accounts.is_empty());
    assert!(database.account_names.is_empty());
    assert!(database.characters.is_empty());
    assert!(database.character_names.is_empty());
    assert!(database.character_slots.is_empty());
    assert!(database.inventory_owners.is_empty());
    assert!(read::<i32>(&database.metadata, b"char_id").unwrap().is_none());
}

#[test]
fn concurrent_character_creation_enforces_slot_uniqueness() {
    let database = seeded_database();
    let barrier = Arc::new(Barrier::new(2));
    let handles: Vec<_> = ["First", "Second"]
        .into_iter()
        .map(|name| {
            let database = database.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait();
                database.insert_character(&character(0, name, 0))
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
    assert_eq!(database.characters.len(), 1);
    assert_eq!(database.character_names.len(), 1);
    assert_eq!(database.character_slots.len(), 1);
}

#[test]
fn inventory_seed_preserves_crafted_and_split_stacks_and_rejects_duplicate_identity_atomically() {
    let database = seeded_database();
    let rows = vec![
        InventoryRecord { id: 31, item_id: 501, amount: 2, card0: 255, card2: 91, ..Default::default() },
        InventoryRecord { id: 32, item_id: 501, amount: 3, card0: 255, card2: 92, ..Default::default() },
        InventoryRecord { id: 33, item_id: 501, amount: 4, ..Default::default() },
    ];
    database.seed(&SeedData { characters: vec![character(150_000, "First", 0)],
        inventories: vec![CharacterInventory { char_id: 150_000, items: rows.clone() }], ..Default::default()
    }, false).unwrap();
    let stored: Vec<InventoryRecord> = required(&database.inventories, &150_000_i32.to_be_bytes()).unwrap();
    assert_eq!(stored, rows);
    for row in &stored { assert_eq!(required::<i32>(&database.inventory_owners, &row.id.to_be_bytes()).unwrap(), 150_000); }
    let snapshot = || [&database.inventories, &database.inventory_owners, &database.metadata].into_iter()
        .map(|tree| tree.iter().map(|pair| { let (key, value) = pair.unwrap(); (key.to_vec(), value.to_vec()) }).collect::<Vec<_>>()).collect::<Vec<_>>();
    let before = snapshot();
    for duplicate_unique_id in [false, true] {
        let mut invalid = rows.clone();
        if duplicate_unique_id { invalid[0].unique_id = 77; invalid[1].unique_id = 77; }
        else { invalid[1].id = invalid[0].id; }
        assert!(database.seed(&SeedData { inventories: vec![CharacterInventory { char_id: 150_000, items: invalid }], ..Default::default() }, true).is_err());
        assert_eq!(snapshot(), before);
    }
}

#[test]
fn replacing_seed_updates_name_slot_and_inventory_owner_indexes() {
    let database = seeded_database();
    let mut seed = SeedData {
        characters: vec![character(150_000, "First", 0)],
        inventories: vec![CharacterInventory {
            char_id: 150_000,
            items: vec![InventoryRecord {
                id: 1,
                item_id: 501,
                amount: 5,
                ..InventoryRecord::default()
            }],
        }],
        ..SeedData::default()
    };
    database.seed(&seed, false).unwrap();
    seed.characters[0].name = "Renamed".into();
    seed.characters[0].char_num = 1;
    seed.inventories[0].items[0].id = 2;
    database.seed(&seed, true).unwrap();
    assert!(database.character_names.get(b"First").unwrap().is_none());
    assert!(database.character_slots.get(character_slot_key(2_000_000, 0)).unwrap().is_none());
    assert!(database.inventory_owners.get(1_i32.to_be_bytes()).unwrap().is_none());
    assert_eq!(
        required::<i32>(&database.inventory_owners, &2_i32.to_be_bytes()).unwrap(),
        150_000
    );
    assert_eq!(required::<i32>(&database.character_names, b"Renamed").unwrap(), 150_000);
}

#[test]
fn reopening_database_restores_records() {
    let path = std::env::temp_dir().join(format!(
        "rust_ro_sled_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    {
        let database = Database::open(&path).unwrap();
        database.create_account("player".into(), "password".into()).unwrap();
    }
    {
        let database = Database::open(&path).unwrap();
        let id = required::<u32>(&database.account_names, b"player").unwrap();
        assert_eq!(
            required::<AccountRecord>(&database.accounts, &id.to_be_bytes()).unwrap().username,
            "player"
        );
    }
    assert_eq!(path.parent(), Some(std::env::temp_dir().as_path()));
    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn unknown_schema_version_is_rejected() {
    let database = Database::temporary().unwrap();
    database
        .metadata
        .transaction(|tree| tx_write(tree, b"schema_version", &(SCHEMA_VERSION + 1)))
        .unwrap();
    assert!(Database::from_db(database._db.clone()).is_err());
}

fn creation(name: &str, slot: i16) -> CharacterCreation {
    CharacterCreation {
        character: character(0, name, slot),
        items: vec![InventoryRecord { item_id: 1201, amount: 1, equip: 2, is_identified: true, ..Default::default() }],
        slot_limit: 3,
        case_sensitive_names: false,
    }
}

#[test]
fn created_character_gets_ids_indexes_and_starting_items_together() {
    let database = seeded_database();
    let created = database.create_character(&creation("Hero", 0)).unwrap();
    assert!(created.char_id > 149_999);
    assert_eq!(read::<i32>(&database.character_names, b"Hero").unwrap(), Some(created.char_id));
    let items: Vec<InventoryRecord> = required(&database.inventories, &created.char_id.to_be_bytes()).unwrap();
    assert_eq!((items.len(), items[0].item_id, items[0].equip), (1, 1201, 2));
    assert_eq!(read::<i32>(&database.inventory_owners, &items[0].id.to_be_bytes()).unwrap(), Some(created.char_id));
}

#[test]
fn creation_checks_name_slot_and_capacity_in_rathena_order() {
    let database = seeded_database();
    database.create_character(&creation("Hero", 0)).unwrap();
    let refusal = |result: Result<CharacterRecord, CreateCharacterError>| match result {
        Err(CreateCharacterError::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    };
    assert_eq!(refusal(database.create_character(&creation("Hero", 1))), CreationRefusal::NameTaken);
    assert_eq!(refusal(database.create_character(&creation("hero", 1))), CreationRefusal::NameTaken, "case-insensitive by default");
    let sensitive = CharacterCreation { case_sensitive_names: true, ..creation("hero", 1) };
    database.create_character(&sensitive).unwrap();
    assert_eq!(refusal(database.create_character(&creation("Other", 3))), CreationRefusal::SlotNotAllowed);
    assert_eq!(refusal(database.create_character(&creation("Other", 0))), CreationRefusal::SlotInUse, "slot 0 is taken");
}

#[test]
fn account_capacity_counts_every_character_of_the_account() {
    let database = seeded_database();
    for (index, name) in ["Aaaa", "Bbbb", "Cccc"].into_iter().enumerate() {
        database.create_character(&creation(name, index as i16)).unwrap();
    }
    assert!(matches!(
        database.create_character(&creation("Dddd", 2)),
        Err(CreateCharacterError::Refused(CreationRefusal::AccountFull))
    ));
}

#[test]
fn renaming_swaps_the_name_index_and_spends_a_rename() {
    let database = seeded_database();
    let hero = database.create_character(&CharacterCreation { character: CharacterRecord { rename: 1, ..character(0, "Hero", 0) }, ..creation("x", 0) }).unwrap();
    database.create_character(&creation("Taken", 1)).unwrap();
    assert!(matches!(database.rename_character(hero.char_id, "taken", false), Err(RenameCharacterError::NameTaken)));
    assert_eq!(database.rename_character(hero.char_id, "Legend", false).unwrap(), "Hero");
    assert!(database.character_names.get(b"Hero").unwrap().is_none());
    assert_eq!(read::<i32>(&database.character_names, b"Legend").unwrap(), Some(hero.char_id));
    let stored: CharacterRecord = required(&database.characters, &hero.char_id.to_be_bytes()).unwrap();
    assert_eq!((stored.name.as_str(), stored.rename), ("Legend", 0));
}

#[test]
fn slot_moves_relocate_or_exchange_characters() {
    let database = seeded_database();
    let first = database.create_character(&creation("Aaaa", 0)).unwrap();
    let second = database.create_character(&creation("Bbbb", 1)).unwrap();
    let slot_of = |id: i32| required::<CharacterRecord>(&database.characters, &id.to_be_bytes()).unwrap().char_num;
    assert!(database.move_character_slot(2_000_000, 0, 2, false).unwrap());
    assert_eq!(slot_of(first.char_id), 2);
    assert!(!database.move_character_slot(2_000_000, 2, 1, false).unwrap(), "occupied target without swapping");
    assert!(database.move_character_slot(2_000_000, 2, 1, true).unwrap());
    assert_eq!((slot_of(first.char_id), slot_of(second.char_id)), (1, 2));
    assert_eq!(read::<i32>(&database.character_slots, &character_slot_key(2_000_000, 1)).unwrap(), Some(first.char_id));
    assert!(!database.move_character_slot(2_000_000, 0, 1, true).unwrap(), "nothing in the source slot");
}

#[test]
fn char_log_entries_are_appended_in_time_order() {
    let database = seeded_database();
    for time in [20, 10] {
        database
            .append_char_log(&CharLogRecord { time, account_id: 2_000_000, char_slot: 0, name: "Hero".into(), message: "make new char".into() })
            .unwrap();
    }
    let times: Vec<i64> = database.char_log.iter().map(|entry| serde_json::from_slice::<CharLogRecord>(&entry.unwrap().1).unwrap().time).collect();
    assert_eq!(times, vec![10, 20]);
}
