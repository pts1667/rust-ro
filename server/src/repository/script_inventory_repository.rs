use std::collections::BTreeMap;

use database::model::{CharacterRecord, InventoryRecord};
use database::{abort, next_id, tx_read, tx_required, tx_write};
use script_sdk::{Value, Variable, VariableScope};
use sled::transaction::{ConflictableTransactionResult, Transactional, TransactionalTree};

use super::model::item_model::{InventoryItemModel, ItemModel};
use super::{Error, SledRepository};
use super::game_system_repository::{CommittedWorldEffects, tx_commit_world_effects};
use crate::server::service::script_world_service::WorldEffectPlan;
use crate::repository::script_character_repository::{apply_skill_reset_tx, apply_experience_tx, ScriptSkillResetPlan, ScriptExperiencePlan};
use super::fame_repository::{commit_crafting_fame_tx, CraftingFamePlan, CraftingFameOutcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptItemConsumption {
    pub inventory_id: i32,
    pub item_id: i32,
    pub unique_id: i64,
    pub amount: i16,
}

#[derive(Debug, Clone)]
pub struct ScriptItemGrant {
    pub item_id: i32,
    pub amount: i16,
    pub identified: bool,
    pub refine: i16,
    pub cards: [i16; 4],
    pub unique_id: Option<i64>,
    pub damaged: bool,
}

#[derive(Debug, Clone)]
pub enum ScriptCharacterChange {
    Resources { hp: u32, sp: u32 },
    ResourcePools { hp: u32, sp: u32, max_hp: u32, max_sp: u32 },
    Experience(ScriptExperiencePlan),
    ResetSkills(ScriptSkillResetPlan),
}

#[derive(Debug, Clone)]
pub struct ScriptInventoryTransaction {
    pub char_id: u32,
    pub account_id: u32,
    pub consumption: Option<ScriptItemConsumption>,
    pub exact_removals: Vec<ScriptItemConsumption>,
    pub removals: Vec<(i32, i16)>,
    pub identifications: Vec<i32>,
    pub grants: Vec<ScriptItemGrant>,
    pub variables: Vec<Variable>,
    pub zeny: Option<u32>,
    pub hp: Option<u32>,
    pub sp: Option<u32>,
    pub max_weight: u32,
    pub max_slots: usize,
    pub world: Option<WorldEffectPlan>,
    pub reset_skills: Option<ScriptSkillResetPlan>,
    pub fame: Option<CraftingFamePlan>,
    pub character_changes: Vec<ScriptCharacterChange>,
    pub pool_draws: Vec<crate::server::script::game_data::PoolDrawReceipt>,
}

#[derive(Debug)]
pub struct ScriptInventoryResult {
    pub inventory: Vec<InventoryItemModel>,
    pub zeny: u32,
    pub world: Option<CommittedWorldEffects>,
    pub fame: Option<CraftingFameOutcome>,
}

pub trait ScriptInventoryRepository: Send + Sync {
    fn script_inventory_transaction(&self, _change: &ScriptInventoryTransaction) -> Result<ScriptInventoryResult, Error> {
        Err(Error::InvalidInput("Script inventory transactions are unavailable".into()))
    }
}

impl ScriptInventoryRepository for SledRepository {
    fn script_inventory_transaction(&self, change: &ScriptInventoryTransaction) -> Result<ScriptInventoryResult, Error> {
        Ok((
            &self.database.characters,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.items,
            &self.database.metadata,
            &self.database.numeric_variables,
            &self.database.string_variables,
            &self.database.game_systems,
            &self.database.skills,
        ).transaction(|(characters, inventories, owners, items, metadata, numbers, strings, game_systems, skills)| {
            let key = change.char_id.to_be_bytes();
            let mut character: CharacterRecord = tx_required(characters, &key)?;
            if character.account_id as u32 != change.account_id {
                return abort("Script character does not belong to this account");
            }
            let mut records: Vec<InventoryRecord> = tx_read(inventories, &key)?.unwrap_or_default();
            let mut previous_weight = 0_u64;
            for record in &records {
                let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
                if item.weight < 0 || record.amount <= 0 { return abort("Invalid inventory weight or amount"); }
                previous_weight += item.weight as u64 * record.amount as u64;
            }
            let previous_slots = records.len();
            if let Some(consumption) = &change.consumption {
                if consumption.amount <= 0 { return abort("Invalid consumption amount"); }
                let position = records.iter().position(|record| record.id == consumption.inventory_id
                    && record.item_id == consumption.item_id && record.unique_id == consumption.unique_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                if records[position].amount < consumption.amount || records[position].equip != 0 {
                    return abort("Consumable changed before the script completed");
                }
                records[position].amount -= consumption.amount;
                if records[position].amount == 0 {
                    owners.remove(records[position].id.to_be_bytes().to_vec())?;
                    records.remove(position);
                }
            }
            for removal in &change.exact_removals {
                if removal.amount <= 0 { return abort("Invalid exact removal amount"); }
                let index = records.iter().position(|record| record.id == removal.inventory_id && record.item_id == removal.item_id && record.unique_id == removal.unique_id)
                    .ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                if records[index].amount < removal.amount { return abort("Required skill item changed before casting"); }
                records[index].amount -= removal.amount;
                if records[index].amount == 0 { owners.remove(records[index].id.to_be_bytes().to_vec())?; records.remove(index); }
            }
            for &(item_id, amount) in &change.removals {
                if amount <= 0 { return abort("Script removal amount must be positive"); }
                let available: i32 = records.iter().filter(|record| record.item_id == item_id).map(|record| i32::from(record.amount)).sum();
                if available < i32::from(amount) { return abort("Not enough items for the script operation"); }
                let mut remaining = amount;
                for equipped in [false, true] {
                    let mut index = 0;
                    while index < records.len() && remaining > 0 {
                        let record = &mut records[index];
                        if record.item_id != item_id || (record.equip != 0) != equipped { index += 1; continue; }
                        let removed = record.amount.min(remaining);
                        record.amount -= removed;
                        remaining -= removed;
                        if record.amount == 0 {
                            owners.remove(record.id.to_be_bytes().to_vec())?;
                            records.remove(index);
                        } else { index += 1; }
                    }
                }
            }
            for id in &change.identifications {
                let record = records.iter_mut().find(|record| record.id == *id).ok_or(sled::transaction::ConflictableTransactionError::Abort(Error::NotFound))?;
                if record.is_identified { return abort("Selected item is already identified"); }
                record.is_identified = true;
            }
            let max_slots = change.max_slots.min(character.inventory_slots.max(0) as usize);
            for grant in &change.grants {
                if grant.amount <= 0 { return abort("Script grant amount must be positive"); }
                let item: ItemModel = tx_required(items, &grant.item_id.to_be_bytes())?;
                let stackable = item.item_type.is_stackable();
                if stackable && grant.unique_id.is_some_and(|id| id != 0) {
                    return abort("Stackable items cannot carry an equipment unique ID");
                }
                if !stackable && grant.unique_id.is_some_and(|id| id != 0) && grant.amount != 1 {
                    return abort("An existing equipment instance cannot be duplicated");
                }
                if !stackable && grant.unique_id.is_some_and(|id| id != 0 && records.iter().any(|record| record.unique_id == id)) {
                    return abort("Equipment unique ID already exists");
                }
                let matching = records.iter().position(|record| stackable && record.item_id == grant.item_id && record.unique_id == 0
                    && record.is_identified == grant.identified && record.refine == grant.refine && record.equip == 0
                    && record.is_damaged == grant.damaged
                    && [record.card0, record.card1, record.card2, record.card3] == grant.cards);
                if let Some(position) = matching {
                    records[position].amount = records[position].amount.checked_add(grant.amount).ok_or_else(|| {
                        sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Item stack would overflow".into()))
                    })?;
                } else {
                    let count = if stackable { 1 } else { grant.amount as usize };
                    if records.len().saturating_add(count) > max_slots.max(previous_slots) { return abort("Not enough inventory space for the script reward"); }
                    for _ in 0..count {
                        let id = next_id(metadata, b"inventory_id", 0)?;
                        let unique_id = if stackable { 0 } else {
                            grant.unique_id.filter(|id| *id != 0).unwrap_or_else(|| (i64::from(change.char_id) << 32) | i64::from(id))
                        };
                        let record = InventoryRecord { id, item_id: grant.item_id, unique_id,
                            amount: if stackable { grant.amount } else { 1 }, is_identified: grant.identified, refine: grant.refine,
                            is_damaged: grant.damaged,
                            card0: grant.cards[0], card1: grant.cards[1], card2: grant.cards[2], card3: grant.cards[3], ..InventoryRecord::default() };
                        tx_write(owners, &id.to_be_bytes(), &(change.char_id as i32))?;
                        records.push(record);
                    }
                }
            }
            if records.len() > max_slots && records.len() > previous_slots { return abort("Character inventory is full"); }
            let mut weight = 0_u64;
            let mut inventory = Vec::with_capacity(records.len());
            for record in &records {
                let item: ItemModel = tx_required(items, &record.item_id.to_be_bytes())?;
                if record.amount <= 0 || item.weight < 0 { return abort("Invalid inventory weight or amount"); }
                weight = weight.checked_add(item.weight as u64 * record.amount as u64).ok_or_else(|| {
                    sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Inventory weight overflow".into()))
                })?;
                inventory.push(InventoryItemModel::from_record(record, &item));
            }
            if weight > u64::from(change.max_weight) && weight > previous_weight { return abort("Script reward exceeds maximum weight"); }
            for update in &change.character_changes {
                match update {
                    ScriptCharacterChange::Resources { hp, sp } => {
                        character.hp = i32::try_from(*hp).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("HP is out of bounds".into())))?;
                        character.sp = i32::try_from(*sp).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("SP is out of bounds".into())))?;
                    }
                    ScriptCharacterChange::Experience(plan) => apply_experience_tx(&mut character, plan)?,
                    ScriptCharacterChange::ResourcePools { hp, sp, max_hp, max_sp } => {
                        if *hp > *max_hp || *sp > *max_sp || *max_hp == 0 { return abort("Invalid character resource pools"); }
                        let convert = |value: u32| i32::try_from(value).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Resource pool is out of bounds".into())));
                        character.hp = convert(*hp)?;
                        character.sp = convert(*sp)?;
                        character.max_hp = convert(*max_hp)?;
                        character.max_sp = convert(*max_sp)?;
                    }
                    ScriptCharacterChange::ResetSkills(plan) => apply_skill_reset_tx(&mut character, skills, game_systems, change.char_id, plan)?,
                }
            }
            if let Some(zeny) = change.zeny {
                character.zeny = i32::try_from(zeny).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("Zeny is out of bounds".into())))?;
            }
            if let Some(hp) = change.hp { character.hp = i32::try_from(hp).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("HP is out of bounds".into())))?; }
            if let Some(sp) = change.sp { character.sp = i32::try_from(sp).map_err(|_| sled::transaction::ConflictableTransactionError::Abort(Error::InvalidInput("SP is out of bounds".into())))?; }
            if let Some(reset) = &change.reset_skills { apply_skill_reset_tx(&mut character, skills, game_systems, change.char_id, reset)?; }
            if change.zeny.is_some() || change.hp.is_some() || change.sp.is_some() || change.reset_skills.is_some() || !change.character_changes.is_empty() { tx_write(characters, &key, &character)?; }
            for variable in &change.variables { write_variable(numbers, strings, change, variable)?; }
            let world = change.world.as_ref().map(|plan| tx_commit_world_effects(game_systems, metadata, characters, change.char_id, plan)).transpose()?;
            let fame = change.fame.as_ref().map(|plan| commit_crafting_fame_tx(&character, game_systems, items, plan)).transpose()?;
            crate::server::script::game_data::tx_commit_pool_draws(game_systems, &change.pool_draws)?;
            tx_write(inventories, &key, &records)?;
            Ok(ScriptInventoryResult { inventory, zeny: character.zeny as u32, world, fame })
        })?)
    }
}

fn write_variable(numbers: &TransactionalTree, strings: &TransactionalTree, change: &ScriptInventoryTransaction, variable: &Variable) -> ConflictableTransactionResult<(), Error> {
    let (scope, owner) = match variable.scope {
        VariableScope::Character => (0_u8, change.char_id),
        VariableScope::Account => (1_u8, change.account_id),
        VariableScope::Server => (2_u8, 0_u32),
        VariableScope::CharacterTemporary | VariableScope::ServerTemporary | VariableScope::Npc | VariableScope::NpcInstance | VariableScope::Instance => return Ok(()),
    };
    if variable.name.is_empty() || variable.name.len() > 128 || variable.name.ends_with('$') != variable.value.is_string() {
        return abort("Invalid script variable name or value type");
    }
    let key = [vec![scope], owner.to_be_bytes().to_vec(), variable.name.as_bytes().to_vec()].concat();
    match &variable.value {
        Value::Number(value) => {
            let mut values: BTreeMap<u32, i32> = tx_read(numbers, &key)?.unwrap_or_default();
            values.insert(variable.index, *value);
            tx_write(numbers, &key, &values)
        }
        Value::String(value) if value.len() <= script_sdk::MAX_MESSAGE_BYTES => {
            let mut values: BTreeMap<u32, String> = tx_read(strings, &key)?.unwrap_or_default();
            values.insert(variable.index, value.clone());
            tx_write(strings, &key, &values)
        }
        _ => abort("Script variable is too large or has an invalid type"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use database::model::{AccountRecord, CharacterInventory, SeedData};
    use database::{read, required};
    use models::status::Status;
    use script_sdk::Function;
    use crate::server::state::character::Character;
    use crate::server::service::script_world_service::plan_persistent_effects;
    use crate::repository::game_system_repository::GameSystemRepository;

    fn setup() -> (SledRepository, ScriptInventoryTransaction) {
        let repository = SledRepository::temporary().unwrap();
        repository.database.seed(&SeedData {
            accounts: vec![AccountRecord { account_id: 2_000_000, username: "Player".into(), password: "secret".into(), ..Default::default() }],
            characters: vec![CharacterRecord { char_id: 150_000, account_id: 2_000_000, name: "Player".into(), hp: 100, sp: 50, zeny: 100, inventory_slots: 100, ..CharacterRecord::default() }],
            inventories: vec![CharacterInventory { char_id: 150_000, items: vec![
                InventoryRecord { id: 10, item_id: 501, amount: 2, is_identified: true, ..InventoryRecord::default() },
                InventoryRecord { id: 11, item_id: 2229, unique_id: 12, amount: 1, is_identified: false, ..InventoryRecord::default() },
            ] }], ..SeedData::default()
        }, false).unwrap();
        repository.database.items.transaction(|items| {
            for (id, kind) in [(501, "Healing"), (503, "Healing"), (2229, "Armor")] {
                let item: ItemModel = serde_json::from_value(serde_json::json!({"id":id,"name_aegis":format!("Item{id}"),"name_english":format!("Item {id}"),
                    "item_type":kind,"weight":10,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0})).unwrap();
                tx_write(items, &(id as i32).to_be_bytes(), &item)?;
            }
            Ok(())
        }).unwrap();
        let change = ScriptInventoryTransaction { char_id: 150_000, account_id: 2_000_000,
            consumption: Some(ScriptItemConsumption { inventory_id: 10, item_id: 501, unique_id: 0, amount: 1 }), exact_removals: vec![], removals: vec![], identifications: vec![],
            grants: vec![ScriptItemGrant { item_id: 503, amount: 3, identified: true, refine: 0, cards: [0; 4], unique_id: None, damaged: false }],
            variables: vec![Variable { scope: VariableScope::Character, name: "MDiceCone".into(), index: 0, value: 7.into() },
                Variable { scope: VariableScope::Account, name: "Note$".into(), index: 2, value: "Committed".into() }],
            zeny: Some(25), hp: Some(80), sp: Some(40), max_weight: 1000, max_slots: 100, world: None, reset_skills: None, fame: None, character_changes: vec![], pool_draws: vec![] };
        (repository, change)
    }

    #[test]
    fn commit_consumes_grants_identifies_updates_resources_and_world_in_one_transaction() {
        let (repository, mut change) = setup();
        let character = Character::new("Player".into(), 150_000, 2_000_000, Status::default(), 1, 1, 0, "prontera".into(), 1, vec![]);
        change.identifications = vec![11];
        change.world = Some(plan_persistent_effects(&character, &[(Function::SetFont, vec![7.into()])], 1000).unwrap());
        let result = repository.script_inventory_transaction(&change).unwrap();
        assert_eq!(result.inventory.iter().find(|item| item.id == 10).unwrap().amount, 1);
        assert_eq!(result.inventory.iter().find(|item| item.item_id == 503).unwrap().amount, 3);
        assert!(result.inventory.iter().find(|item| item.id == 11).unwrap().is_identified);
        let stored: CharacterRecord = required(&repository.database.characters, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!((stored.hp, stored.sp, stored.zeny), (80, 40, 25));
        assert_eq!(repository.character_game_systems(150_000).unwrap().font, 7);
        assert_eq!(result.world.unwrap().systems.unwrap().revision, 1);
        let numeric: BTreeMap<u32, i32> = required(&repository.database.numeric_variables, &[vec![0], 150_000_u32.to_be_bytes().to_vec(), b"MDiceCone".to_vec()].concat()).unwrap();
        assert_eq!(numeric.get(&0), Some(&7));
        let strings: BTreeMap<u32, String> = required(&repository.database.string_variables, &[vec![1], 2_000_000_u32.to_be_bytes().to_vec(), b"Note$".to_vec()].concat()).unwrap();
        assert_eq!(strings.get(&2).map(String::as_str), Some("Committed"));
    }

    #[test]
    fn stale_world_revision_rolls_back_consumption_rewards_ids_variables_and_resources() {
        let (repository, mut change) = setup();
        let character = Character::new("Player".into(), 150_000, 2_000_000, Status::default(), 1, 1, 0, "prontera".into(), 1, vec![]);
        let mut plan = plan_persistent_effects(&character, &[(Function::SetFont, vec![7.into()])], 1000).unwrap();
        plan.expected_revision = 1;
        change.world = Some(plan);
        let before_id = read::<i32>(&repository.database.metadata, b"inventory_id").unwrap();
        assert!(repository.script_inventory_transaction(&change).is_err());
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!(inventory.len(), 2);
        assert_eq!(inventory[0].amount, 2);
        assert_eq!(repository.database.inventory_owners.len(), 2);
        assert_eq!(read::<i32>(&repository.database.metadata, b"inventory_id").unwrap(), before_id);
        assert!(repository.database.numeric_variables.is_empty());
        assert!(repository.database.string_variables.is_empty());
        assert_eq!(repository.character_game_systems(150_000).unwrap().font, 0);
        let stored: CharacterRecord = required(&repository.database.characters, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!((stored.hp, stored.sp, stored.zeny), (100, 50, 100));
    }

    #[test]
    fn stale_consumable_identity_and_reward_capacity_preserve_the_source_item() {
        let (repository, mut change) = setup();
        change.consumption.as_mut().unwrap().unique_id = 99;
        assert!(repository.script_inventory_transaction(&change).is_err());
        change.consumption.as_mut().unwrap().unique_id = 0;
        change.max_weight = 35;
        assert!(repository.script_inventory_transaction(&change).is_err());
        change.max_weight = 1000;
        change.max_slots = 2;
        assert!(repository.script_inventory_transaction(&change).is_err());
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!((inventory.len(), inventory[0].amount), (2, 2));
    }

    #[test]
    fn forged_item_and_fame_share_the_inventory_transaction_and_identity_guard() {
        use super::super::fame_repository::{FameCategory, FameRepository};
        use models::enums::class::JobName;
        use models::enums::skill_enums::SkillEnum;
        use models::enums::EnumWithNumberValue;
        let (repository, mut change) = setup();
        (&repository.database.characters, &repository.database.items).transaction(|(characters, items)| {
            let mut character: CharacterRecord = tx_required(characters, &150_000_u32.to_be_bytes())?;
            character.class = JobName::Blacksmith.value() as i16;
            tx_write(characters, &150_000_u32.to_be_bytes(), &character)?;
            let item: ItemModel = serde_json::from_value(serde_json::json!({"id":1201,"name_aegis":"Knife","name_english":"Knife",
                "item_type":"Weapon","weight":10,"weapon_level":3,"job_flags":0,"class_flags":0,"location":0,"flags":0,"trade_flags":0})).unwrap();
            tx_write(items, &1201_i32.to_be_bytes(), &item)
        }).unwrap();
        change.grants = vec![ScriptItemGrant { item_id: 1201, amount: 1, identified: true, refine: 0, cards: [255, 15 * 256, 150_000_u32 as u16 as i16, (150_000_u32 >> 16) as i16], unique_id: None, damaged: false }];
        change.fame = Some(CraftingFamePlan { item_id: 1201, skill_id: SkillEnum::BsDagger.id(), success: true, additive_slots: 3, previous_potion_streak: 0 });
        change.consumption.as_mut().unwrap().unique_id = 1;
        assert!(repository.script_inventory_transaction(&change).is_err());
        assert_eq!(repository.character_fame(150_000).unwrap(), 0);
        assert!(repository.fame_rankings(FameCategory::Blacksmith).unwrap().is_empty());
        let before: Vec<InventoryRecord> = required(&repository.database.inventories, &150_000_u32.to_be_bytes()).unwrap();
        assert_eq!(before[0].amount, 2);
        change.consumption.as_mut().unwrap().unique_id = 0;
        let result = repository.script_inventory_transaction(&change).unwrap();
        assert_eq!(result.fame.unwrap().gained, 10);
        assert_eq!(result.inventory.iter().find(|item| item.item_id == 1201).unwrap().card0, 255);
        assert_eq!(repository.character_fame(150_000).unwrap(), 10);
        assert_eq!(repository.fame_rank(FameCategory::Blacksmith, 150_000).unwrap(), 1);
    }

    #[test]
    fn ordered_healing_and_experience_commit_together_and_reject_stale_progression() {
        let (repository, mut change) = setup();
        let original: CharacterRecord = required(&repository.database.characters, &change.char_id.to_be_bytes()).unwrap();
        let mut plan = ScriptExperiencePlan { expected_job: original.class as u32,
            expected: [original.base_level as u32, original.job_level as u32, original.base_exp as u32, original.job_exp as u32,
                original.status_point as u32, original.skill_point as u32, 80, 40],
            base_level: original.base_level as u32 + 1, job_level: original.job_level as u32 + 1,
            base_exp: 2, job_exp: 3, status_points: original.status_point as u32 + 3, skill_points: original.skill_point as u32 + 1,
            hp: 150, sp: 75, max_hp: 150, max_sp: 75 };
        change.hp = None; change.sp = None;
        plan.expected[2] += 1;
        change.character_changes = vec![ScriptCharacterChange::Resources { hp: 80, sp: 40 }, ScriptCharacterChange::Experience(plan.clone()), ScriptCharacterChange::Resources { hp: 120, sp: 65 }];
        assert!(repository.script_inventory_transaction(&change).is_err());
        let unchanged: CharacterRecord = required(&repository.database.characters, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!((unchanged.hp, unchanged.sp, unchanged.base_level), (original.hp, original.sp, original.base_level));
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!(inventory[0].amount, 2);
        assert!(repository.database.numeric_variables.is_empty());
        plan.expected[2] -= 1;
        change.character_changes[1] = ScriptCharacterChange::Experience(plan.clone());
        repository.script_inventory_transaction(&change).unwrap();
        let committed: CharacterRecord = required(&repository.database.characters, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!((committed.hp, committed.sp, committed.max_hp, committed.max_sp), (120, 65, 150, 75));
        assert_eq!((committed.base_level as u32, committed.job_level as u32, committed.base_exp, committed.job_exp), (plan.base_level, plan.job_level, 2, 3));
        let inventory: Vec<InventoryRecord> = required(&repository.database.inventories, &change.char_id.to_be_bytes()).unwrap();
        assert_eq!(inventory[0].amount, 1);
    }
}
