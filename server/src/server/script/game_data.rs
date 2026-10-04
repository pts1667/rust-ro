use std::collections::HashMap;
use std::sync::OnceLock;

use database::{abort, tx_read, tx_write};
use script_sdk::Value;
use serde::{Deserialize, Serialize};
use sled::transaction::{ConflictableTransactionResult, TransactionalTree};

#[derive(Debug, Deserialize)]
pub struct ScriptGameData {
    pub groups: Vec<ItemGroup>,
    pub summons: Vec<SummonGroup>,
    pub recipes: Vec<Recipe>,
    pub item_use_groups: HashMap<String, Vec<i32>>,
    pub item_aliases: HashMap<String, i32>,
}

#[derive(Debug, Deserialize)]
pub struct ItemGroup {
    pub id: i32,
    pub name: String,
    pub subgroups: Vec<ItemSubGroup>,
}

#[derive(Debug, Deserialize)]
pub struct ItemSubGroup {
    pub id: i32,
    pub algorithm: String,
    pub entries: Vec<ItemGroupEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemGroupEntry {
    pub item_id: i32,
    pub rate: u32,
    pub amount: i16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemPoolState {
    pub revision: u64,
    pub definition: Vec<(i32, u32, i16)>,
    pub remaining: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PoolDrawReceipt {
    pub group_id: i32,
    pub subgroup_id: i32,
    pub before: ItemPoolState,
    pub after: ItemPoolState,
    pub selected_index: usize,
}

pub fn item_pool_key(group_id: i32, subgroup_id: i32) -> Vec<u8> {
    [b"item_pool/".as_slice(), &group_id.to_be_bytes(), &subgroup_id.to_be_bytes()].concat()
}

fn normalized_pool(state: &ItemPoolState, definition: &[(i32, u32, i16)]) -> Vec<u32> {
    if state.definition != definition
        || state.remaining.len() != definition.len()
        || state.remaining.iter().all(|remaining| *remaining == 0)
    {
        definition.iter().map(|(_, rate, _)| *rate).collect()
    } else {
        state.remaining.clone()
    }
}

pub fn stage_group_entry(
    group: &ItemGroup,
    subgroup_id: i32,
    state: &ItemPoolState,
    rng: &mut fastrand::Rng,
) -> Option<(ItemGroupEntry, Option<PoolDrawReceipt>)> {
    let subgroup = group.subgroups.iter().find(|subgroup| subgroup.id == subgroup_id)?;
    if subgroup.algorithm != "SharedPool" {
        return random_group_entry(group, subgroup_id, rng).map(|entry| (entry, None));
    }
    let definition = subgroup
        .entries
        .iter()
        .map(|entry| (entry.item_id, entry.rate, entry.amount))
        .collect::<Vec<_>>();
    let mut remaining = normalized_pool(state, &definition);
    let selected_index = weighted_index(remaining.iter().copied(), rng)?;
    remaining[selected_index] -= 1;
    if remaining.iter().all(|remaining| *remaining == 0) {
        remaining = definition.iter().map(|(_, rate, _)| *rate).collect();
    }
    let after = ItemPoolState {
        revision: state.revision.checked_add(1)?,
        definition,
        remaining,
    };
    Some((
        subgroup.entries[selected_index].clone(),
        Some(PoolDrawReceipt {
            group_id: group.id,
            subgroup_id,
            before: state.clone(),
            after,
            selected_index,
        }),
    ))
}

pub fn tx_commit_pool_draws(
    tree: &TransactionalTree,
    receipts: &[PoolDrawReceipt],
) -> ConflictableTransactionResult<(), crate::repository::Error> {
    for receipt in receipts {
        let key = item_pool_key(receipt.group_id, receipt.subgroup_id);
        let current: ItemPoolState = tx_read(tree, &key)?.unwrap_or_default();
        if current != receipt.before {
            return abort("Shared item pool changed while rewards were prepared");
        }
        if current.revision.checked_add(1) != Some(receipt.after.revision) || receipt.after.definition.is_empty() {
            return abort("Invalid shared item pool revision");
        }
        let mut expected = normalized_pool(&current, &receipt.after.definition);
        let Some(remaining) = expected.get_mut(receipt.selected_index).filter(|remaining| **remaining > 0) else {
            return abort("Shared item pool entry is exhausted");
        };
        *remaining -= 1;
        if expected.iter().all(|remaining| *remaining == 0) {
            expected = receipt.after.definition.iter().map(|(_, rate, _)| *rate).collect();
        }
        if expected != receipt.after.remaining {
            return abort("Invalid shared item pool depletion receipt");
        }
        tx_write(tree, &key, &receipt.after)?;
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct SummonGroup {
    pub id: i32,
    pub name: String,
    pub default: u32,
    pub entries: Vec<SummonEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SummonEntry {
    pub mob_id: u32,
    pub rate: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub item_id: i32,
    pub level: u16,
    pub skill_id: u32,
    pub skill_level: u16,
    pub materials: Vec<RecipeMaterial>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecipeMaterial {
    pub item_id: i32,
    pub amount: i16,
}

pub fn data() -> &'static ScriptGameData {
    static DATA: OnceLock<ScriptGameData> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("game_data.json")).expect("Invalid compiled pre-renewal script game data"))
}

pub fn item_in_use_group(item_id: i32, group: &str) -> bool {
    data().item_use_groups.get(group).is_some_and(|items| items.contains(&item_id))
}

pub fn constant(name: &str) -> Option<Value> {
    let guild_storage = match name {
        "GSTORAGE_OPEN" => Some(0),
        "GSTORAGE_STORAGE_ALREADY_OPEN" => Some(1),
        "GSTORAGE_ALREADY_OPEN" => Some(2),
        "GSTORAGE_NO_GUILD" => Some(3),
        "GSTORAGE_NO_STORAGE" => Some(4),
        "GSTORAGE_NO_PERMISSION" => Some(5),
        _ => None,
    };
    if let Some(code) = guild_storage {
        return Some(Value::Number(code));
    }
    if let Some(group) = name.strip_prefix("IG_") {
        let group = if group.eq_ignore_ascii_case("Accessory") {
            "ACCESORY"
        } else {
            group
        };
        return data()
            .groups
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(group))
            .map(|entry| entry.id.into());
    }
    if let Some(group) = name.strip_prefix("MOBG_") {
        return data()
            .summons
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(group))
            .map(|entry| entry.id.into());
    }
    None
}

pub fn group(value: &Value) -> Option<&'static ItemGroup> {
    if let Ok(id) = value.number_value() {
        data().groups.iter().find(|group| group.id == id)
    } else {
        let name = value.text();
        let name = name.strip_prefix("IG_").unwrap_or(&name);
        data().groups.iter().find(|group| group.name.eq_ignore_ascii_case(name))
    }
}

pub fn random_group_entry(group: &ItemGroup, subgroup_id: i32, rng: &mut fastrand::Rng) -> Option<ItemGroupEntry> {
    let subgroup = group.subgroups.iter().find(|subgroup| subgroup.id == subgroup_id)?;
    if subgroup.entries.is_empty() {
        return None;
    }
    match subgroup.algorithm.as_str() {
        "All" => Some(subgroup.entries[rng.usize(0..subgroup.entries.len())].clone()),
        "Drop" => {
            let entry = &subgroup.entries[rng.usize(0..subgroup.entries.len())];
            (rng.u32(0..10_000) < entry.rate).then(|| entry.clone())
        }
        "SharedPool" => None,
        "Random" => weighted_index(subgroup.entries.iter().map(|entry| entry.rate), rng).map(|index| subgroup.entries[index].clone()),
        _ => None,
    }
}

pub fn random_group_item(group_id: u32, rng: &mut fastrand::Rng) -> Option<(i32, i16)> {
    let group = group(&(group_id as i32).into())?;
    let entry = random_group_entry(group, 1, rng)?;
    Some((entry.item_id, entry.amount))
}

pub fn random_summon(group_name: &str, rng: &mut fastrand::Rng) -> Option<u32> {
    let group = data().summons.iter().find(|group| group.name.eq_ignore_ascii_case(group_name))?;
    Some(weighted_index(group.entries.iter().map(|entry| entry.rate), rng).map_or(group.default, |index| group.entries[index].mob_id))
}

pub fn random_summon_id(group_id: i32, rng: &mut fastrand::Rng) -> Option<u32> {
    let group = data().summons.iter().find(|group| group.id == group_id)?;
    random_summon(&group.name, rng)
}

fn weighted_index(rates: impl Iterator<Item = u32> + Clone, rng: &mut fastrand::Rng) -> Option<usize> {
    let total: u64 = rates.clone().map(u64::from).sum();
    if total == 0 {
        return None;
    }
    let mut selected = rng.u64(0..total);
    for (index, rate) in rates.enumerate() {
        if selected < u64::from(rate) {
            return Some(index);
        }
        selected -= u64::from(rate);
    }
    None
}

#[cfg(test)]
mod tests {
    use database::model::{AccountRecord, CharacterInventory, CharacterRecord, InventoryRecord, SeedData};
    use database::{read, required};

    use super::*;
    use crate::repository::SledRepository;
    use crate::repository::game_system_repository::GameSystemRepository;
    use crate::repository::model::item_model::ItemModel;
    use crate::repository::script_inventory_repository::{
        ScriptInventoryRepository, ScriptInventoryTransaction, ScriptItemConsumption, ScriptItemGrant,
    };

    fn test_pool() -> ItemGroup {
        ItemGroup {
            id: 50_001,
            name: "TEST_POOL".into(),
            subgroups: vec![ItemSubGroup {
                id: 1,
                algorithm: "SharedPool".into(),
                entries: vec![
                    ItemGroupEntry {
                        item_id: 501,
                        rate: 1,
                        amount: 1,
                    },
                    ItemGroupEntry {
                        item_id: 502,
                        rate: 2,
                        amount: 1,
                    },
                ],
            }],
        }
    }

    #[test]
    fn shared_pools_exhaust_each_entry_before_refilling() {
        let group = test_pool();
        let mut rng = fastrand::Rng::with_seed(9);
        let mut state = ItemPoolState::default();
        for _ in 0..2 {
            let mut draws = (0..3)
                .map(|_| {
                    let (entry, receipt) = stage_group_entry(&group, 1, &state, &mut rng).unwrap();
                    state = receipt.unwrap().after;
                    entry.item_id
                })
                .collect::<Vec<_>>();
            draws.sort();
            assert_eq!(draws, [501, 502, 502]);
        }
    }

    #[test]
    fn shared_pools_commit_chained_draws_and_reject_duplicate_receipts_atomically() {
        let repository = SledRepository::temporary().unwrap();
        let group = test_pool();
        let mut rng = fastrand::Rng::with_seed(11);
        let (_, first) = stage_group_entry(&group, 1, &ItemPoolState::default(), &mut rng).unwrap();
        let first = first.unwrap();
        let (_, second) = stage_group_entry(&group, 1, &first.after, &mut rng).unwrap();
        let second = second.unwrap();
        assert!(repository.commit_item_pool_draws(&[first.clone(), first.clone()]).is_err());
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), ItemPoolState::default());
        repository.commit_item_pool_draws(&[first.clone(), second.clone()]).unwrap();
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), second.after);
        let mut invalid = second.clone();
        invalid.before = second.after.clone();
        invalid.after.revision += 1;
        invalid.after.remaining = vec![100, 100];
        assert!(repository.commit_item_pool_draws(&[invalid]).is_err());
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), second.after);
    }

    #[test]
    fn shared_pools_refill_after_definition_changes_and_server_restart() {
        let repository = SledRepository::temporary().unwrap();
        let mut group = test_pool();
        let mut rng = fastrand::Rng::with_seed(12);
        let (_, receipt) = stage_group_entry(&group, 1, &ItemPoolState::default(), &mut rng).unwrap();
        repository.commit_item_pool_draws(&[receipt.unwrap()]).unwrap();
        let old = repository.item_group_pool(group.id, 1).unwrap();
        group.subgroups[0].entries = vec![ItemGroupEntry {
            item_id: 503,
            rate: 4,
            amount: 2,
        }];
        let (entry, receipt) = stage_group_entry(&group, 1, &old, &mut rng).unwrap();
        let receipt = receipt.unwrap();
        assert_eq!((entry.item_id, entry.amount), (503, 2));
        assert_eq!(receipt.after.remaining, [3]);
        repository.commit_item_pool_draws(&[receipt]).unwrap();
        repository.database.game_systems.insert(b"unrelated-world-state", &[1]).unwrap();
        repository.reset_item_group_pools().unwrap();
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), ItemPoolState::default());
        assert_eq!(
            repository
                .database
                .game_systems
                .get(b"unrelated-world-state")
                .unwrap()
                .unwrap()
                .as_ref(),
            &[1]
        );
    }

    #[test]
    fn shared_pools_rejected_inventory_transactions_preserve_consumables_and_quota() {
        let repository = SledRepository::temporary().unwrap();
        repository
            .database
            .seed(
                &SeedData {
                    accounts: vec![AccountRecord {
                        account_id: 2_000_000,
                        username: "PoolPlayer".into(),
                        password: "secret".into(),
                    }],
                    characters: vec![CharacterRecord {
                        char_id: 150_000,
                        account_id: 2_000_000,
                        name: "PoolPlayer".into(),
                        inventory_slots: 100,
                        zeny: 100,
                        ..CharacterRecord::default()
                    }],
                    inventories: vec![CharacterInventory {
                        char_id: 150_000,
                        items: vec![InventoryRecord {
                            id: 10,
                            item_id: 501,
                            amount: 2,
                            is_identified: true,
                            ..InventoryRecord::default()
                        }],
                    }],
                    ..SeedData::default()
                },
                false,
            )
            .unwrap();
        repository.database.items.transaction(|items| {
            for item_id in [501i32, 503] {
                let item: ItemModel = serde_json::from_value(serde_json::json!({"id":item_id,"name_aegis":format!("Item{item_id}"),"name_english":format!("Item {item_id}"),"item_type":"Healing","weight":10,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0})).unwrap();
                tx_write(items, &item_id.to_be_bytes(), &item)?;
            }
            Ok(())
        }).unwrap();
        let group = ItemGroup {
            id: 50_002,
            name: "SINGLE_POOL".into(),
            subgroups: vec![ItemSubGroup {
                id: 1,
                algorithm: "SharedPool".into(),
                entries: vec![ItemGroupEntry {
                    item_id: 503,
                    rate: 2,
                    amount: 1,
                }],
            }],
        };
        let (_, receipt) = stage_group_entry(&group, 1, &ItemPoolState::default(), &mut fastrand::Rng::with_seed(3)).unwrap();
        let receipt = receipt.unwrap();
        let mut change = ScriptInventoryTransaction {
            char_id: 150_000,
            account_id: 2_000_000,
            consumption: Some(ScriptItemConsumption {
                inventory_id: 10,
                item_id: 501,
                unique_id: 0,
                amount: 1,
            }),
            exact_removals: vec![],
            removals: vec![],
            identifications: vec![],
            grants: vec![ScriptItemGrant {
                item_id: 503,
                amount: 1,
                identified: true,
                refine: 0,
                cards: [0; 4],
                unique_id: None,
                damaged: false,
            }],
            variables: vec![script_sdk::Variable {
                scope: script_sdk::VariableScope::Character,
                name: "PoolDrawCount".into(),
                index: 0,
                value: 1.into(),
            }],
            zeny: Some(25),
            hp: None,
            sp: None,
            max_weight: 1000,
            max_slots: 1,
            world: None,
            reset_skills: None,
            fame: None,
            character_changes: vec![],
            pool_draws: vec![receipt.clone()],
        };
        let before_id = read::<i32>(&repository.database.metadata, b"inventory_id").unwrap();
        assert!(repository.script_inventory_transaction(&change).is_err());
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), ItemPoolState::default());
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!((inventory.len(), inventory[0].amount), (1, 2));
        assert_eq!(read::<i32>(&repository.database.metadata, b"inventory_id").unwrap(), before_id);
        assert!(repository.database.numeric_variables.is_empty());
        change.max_slots = 100;
        repository.script_inventory_transaction(&change).unwrap();
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), receipt.after);
        let committed_id = read::<i32>(&repository.database.metadata, b"inventory_id").unwrap();
        let committed: Vec<InventoryRecord> = required(&repository.database.inventories, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!((committed.len(), committed[0].amount), (2, 1));
        assert!(repository.script_inventory_transaction(&change).is_err());
        let unchanged: Vec<InventoryRecord> = required(&repository.database.inventories, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!((unchanged.len(), unchanged[0].amount, unchanged[1].amount), (2, 1, 1));
        assert_eq!(repository.item_group_pool(group.id, 1).unwrap(), receipt.after);
        assert_eq!(
            read::<i32>(&repository.database.metadata, b"inventory_id").unwrap(),
            committed_id
        );
        assert_eq!(
            required::<CharacterRecord>(&repository.database.characters, &change.char_id.to_be_bytes())
                .unwrap()
                .zeny,
            25
        );
    }

    #[test]
    fn classic_reward_constants_and_recipes_resolve() {
        assert_eq!(constant("IG_BlueBox"), Some(0.into()));
        assert!(constant("IG_Accesory").is_some());
        assert_eq!(constant("MOBG_BRANCH_OF_DEAD_TREE"), Some(0.into()));
        assert!(data().recipes.iter().all(|recipe| recipe.skill_id < 1000 && recipe.level <= 23));
        let mut rng = fastrand::Rng::with_seed(7);
        assert!(random_group_item(0, &mut rng).is_some());
        assert!(random_summon("BLOODY_DEAD_BRANCH", &mut rng).is_some());
    }
}
