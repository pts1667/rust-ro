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
                    password: "password".into(),
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
                password: "password".into(),
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
