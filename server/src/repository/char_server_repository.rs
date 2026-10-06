use database::model::{CharLogRecord, CharacterRecord, InventoryRecord};
use database::{CharacterCreation, CreateCharacterError, RenameCharacterError, read, tx_read, tx_required, tx_write};
use sled::transaction::Transactional;

use crate::repository::game_system_repository::character_key;
use crate::repository::{CharServerRepository, Error, SledRepository};
use crate::server::model::game_systems::CharacterGameSystems;

const CHAR_SCOPE: u8 = 0;
const PET_EGG_MARKER: i16 = 256;

fn prefixed_key(prefix: &[u8], id: u32) -> Vec<u8> {
    [prefix, &id.to_be_bytes()].concat()
}

fn egg_pet_id(item: &InventoryRecord) -> Option<u32> {
    (item.card0 == PET_EGG_MARKER).then(|| u32::from(item.card1 as u16) | (u32::from(item.card2 as u16) << 16))
}

impl CharServerRepository for SledRepository {
    fn char_create(&self, creation: &CharacterCreation) -> Result<CharacterRecord, CreateCharacterError> {
        self.database.create_character(creation)
    }

    fn char_account_characters(&self, account_id: u32) -> Result<Vec<CharacterRecord>, Error> {
        self.database
            .character_slots
            .scan_prefix(account_id.to_be_bytes())
            .map(|entry| {
                let id: i32 = serde_json::from_slice(&entry?.1)?;
                database::required(&self.database.characters, &id.to_be_bytes())
            })
            .collect()
    }

    fn char_find(&self, char_id: u32) -> Result<Option<CharacterRecord>, Error> {
        read(&self.database.characters, &char_id.to_be_bytes())
    }

    fn char_find_by_name(&self, name: &str) -> Result<Option<CharacterRecord>, Error> {
        let exact = read::<i32>(&self.database.character_names, name.as_bytes())?;
        let id = match exact {
            Some(id) => Some(id),
            None => {
                let mut found = None;
                for entry in self.database.character_names.iter() {
                    let (candidate, id) = entry?;
                    if candidate.eq_ignore_ascii_case(name.as_bytes()) {
                        found = Some(serde_json::from_slice(&id)?);
                        break;
                    }
                }
                found
            }
        };
        id.map_or(Ok(None), |id| read(&self.database.characters, &id.to_be_bytes()))
    }

    fn char_update(&self, char_id: u32, update: &dyn Fn(&mut CharacterRecord)) -> Result<CharacterRecord, Error> {
        Ok(self.database.characters.transaction(|characters| {
            let mut character: CharacterRecord = tx_required(characters, &char_id.to_be_bytes())?;
            update(&mut character);
            tx_write(characters, &char_id.to_be_bytes(), &character)?;
            Ok(character)
        })?)
    }

    fn char_rename(&self, char_id: u32, new_name: &str, case_sensitive: bool) -> Result<String, RenameCharacterError> {
        let old_name = self.database.rename_character(char_id as i32, new_name, case_sensitive)?;
        self.database
            .game_systems
            .transaction(|systems| crate::repository::fame_repository::rename_character_fame_tx(systems, char_id, new_name))
            .map_err(|error| RenameCharacterError::from(database::DatabaseError::from(error)))?;
        Ok(old_name)
    }

    fn char_move_slot(&self, account_id: u32, from: i16, to: i16, allow_swap: bool) -> Result<bool, Error> {
        self.database.move_character_slot(account_id, from, to, allow_swap)
    }

    fn char_name_taken(&self, name: &str, case_sensitive: bool) -> Result<bool, Error> {
        self.database.character_name_taken(name, case_sensitive)
    }

    fn char_remove_items(&self, char_id: u32, item_ids: &[i32]) -> Result<usize, Error> {
        let id = char_id as i32;
        Ok((&self.database.inventories, &self.database.inventory_owners).transaction(|(inventories, owners)| {
            let items: Vec<InventoryRecord> = tx_read(inventories, &id.to_be_bytes())?.unwrap_or_default();
            let (removed, kept): (Vec<_>, Vec<_>) = items.into_iter().partition(|item| item_ids.contains(&item.item_id));
            for item in &removed {
                owners.remove(item.id.to_be_bytes().to_vec())?;
            }
            if !removed.is_empty() {
                tx_write(inventories, &id.to_be_bytes(), &kept)?;
            }
            Ok(removed.len())
        })?)
    }

    fn char_log(&self, record: CharLogRecord) -> Result<(), Error> {
        self.database.append_char_log(&record)
    }

    fn char_purge(&self, account_id: u32, char_id: u32) -> Result<(), Error> {
        let id = char_id as i32;
        let variable_prefix = [&[CHAR_SCOPE][..], &char_id.to_be_bytes()].concat();
        let mut variable_keys = Vec::new();
        for tree in [&self.database.numeric_variables, &self.database.string_variables] {
            for entry in tree.scan_prefix(&variable_prefix) {
                variable_keys.push((tree.clone(), entry?.0));
            }
        }
        (
            &self.database.characters,
            &self.database.character_names,
            &self.database.character_slots,
            &self.database.inventories,
            &self.database.inventory_owners,
            &self.database.skills,
            &self.database.bonuses,
            &self.database.hotkeys,
            &self.database.game_systems,
        )
            .transaction(|(characters, names, slots, inventories, owners, skills, bonuses, hotkeys, systems)| {
                let character: CharacterRecord = tx_required(characters, &id.to_be_bytes())?;
                if character.account_id != account_id as i32 {
                    return database::abort("Character belongs to another account");
                }
                let items: Vec<InventoryRecord> = tx_read(inventories, &id.to_be_bytes())?.unwrap_or_default();
                let cart: Vec<InventoryRecord> = tx_read(systems, &prefixed_key(b"cart/", char_id))?.unwrap_or_default();
                for item in &items {
                    owners.remove(item.id.to_be_bytes().to_vec())?;
                }
                let state: CharacterGameSystems = tx_read(systems, &character_key(char_id))?.unwrap_or_default();
                let pets = items
                    .iter()
                    .chain(cart.iter())
                    .filter_map(egg_pet_id)
                    .chain(state.pet.iter().map(|pet| pet.id));
                for pet_id in pets {
                    systems.remove(prefixed_key(b"pet/", pet_id))?;
                }
                for key in [
                    character_key(char_id),
                    prefixed_key(b"cart/", char_id),
                    prefixed_key(b"character_fame/", char_id),
                    prefixed_key(b"taekwon_mission/", char_id),
                ] {
                    systems.remove(key)?;
                }
                crate::repository::fame_repository::forget_character_fame_tx(systems, char_id)?;
                inventories.remove(id.to_be_bytes().to_vec())?;
                skills.remove(id.to_be_bytes().to_vec())?;
                hotkeys.remove(id.to_be_bytes().to_vec())?;
                let mut bonus_key = [0u8; 8];
                bonus_key[..4].copy_from_slice(&char_id.to_be_bytes());
                bonus_key[4..].copy_from_slice(&account_id.to_be_bytes());
                bonuses.remove(bonus_key.to_vec())?;
                names.remove(character.name.as_bytes())?;
                slots.remove(database::character_slot_key(character.account_id, character.char_num).to_vec())?;
                characters.remove(id.to_be_bytes().to_vec())?;
                Ok(())
            })?;
        for (tree, key) in variable_keys {
            tree.remove(key)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use database::model::{AccountRecord, CharacterRecord, InventoryRecord};

    use super::*;
    use crate::repository::{GameSystemRepository, ScriptVariableRepository};
    use crate::server::model::game_systems::PetRecord;

    fn repository_with_character() -> (SledRepository, CharacterRecord) {
        let repository = SledRepository::temporary().unwrap();
        let account = repository.account_create_for_test("player");
        let character = repository
            .char_create(&CharacterCreation {
                character: CharacterRecord { account_id: account as i32, name: "Hero".into(), inventory_slots: 100, ..Default::default() },
                items: vec![InventoryRecord { item_id: 1201, amount: 1, equip: 2, is_identified: true, ..Default::default() }],
                slot_limit: 12,
                case_sensitive_names: false,
            })
            .unwrap();
        (repository, character)
    }

    impl SledRepository {
        fn account_create_for_test(&self, name: &str) -> u32 {
            use crate::repository::LoginRepository;
            self.account_create(AccountRecord::new(0, name, "secret")).unwrap()
        }
    }

    #[test]
    fn lists_account_characters_in_slot_order_and_finds_them_by_id() {
        let (repository, hero) = repository_with_character();
        let second = repository
            .char_create(&CharacterCreation {
                character: CharacterRecord { account_id: hero.account_id, name: "Alt".into(), char_num: 1, inventory_slots: 100, ..Default::default() },
                items: vec![],
                slot_limit: 12,
                case_sensitive_names: false,
            })
            .unwrap();
        let listed = repository.char_account_characters(hero.account_id as u32).unwrap();
        assert_eq!(listed.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(), vec!["Hero", "Alt"]);
        assert_eq!(repository.char_find(second.char_id as u32).unwrap().unwrap().name, "Alt");
        assert!(repository.char_find(1).unwrap().is_none());
    }

    #[test]
    fn purging_removes_the_character_and_everything_keyed_by_it() {
        let (repository, hero) = repository_with_character();
        let char_id = hero.char_id as u32;
        repository.script_variable_char_num_save(char_id, "QUEST".into(), 0, 5);
        repository.script_variable_char_str_save(char_id, "NAME$".into(), 0, "x".into());
        repository.script_variable_char_num_save(char_id + 1, "OTHER".into(), 0, 1);
        repository.database.hotkeys.insert(char_id.to_be_bytes(), b"[]".to_vec()).unwrap();
        repository.database.skills.insert(char_id.to_be_bytes(), b"{}".to_vec()).unwrap();
        let pet = PetRecord { id: 7, owner_char_id: char_id, class_id: 1002, name: "Poring".into(), level: 1, egg_item_id: 9001, egg_inventory_id: 4040, intimacy: 0, hunger: 0, equipped_item: 0, next_hunger_at: 0, incubating: true, renamed: false };
        repository.database.game_systems.insert(prefixed_key(b"pet/", 7), serde_json::to_vec(&pet).unwrap()).unwrap();
        let egg = InventoryRecord { id: 4040, item_id: 9001, amount: 1, card0: 256, card1: 7, card2: 0, ..Default::default() };
        let mut stored: Vec<InventoryRecord> = database::required(&repository.database.inventories, &hero.char_id.to_be_bytes()).unwrap();
        stored.push(egg);
        repository.database.inventories.insert(hero.char_id.to_be_bytes(), serde_json::to_vec(&stored).unwrap()).unwrap();
        repository.database.inventory_owners.insert(4040i32.to_be_bytes(), serde_json::to_vec(&hero.char_id).unwrap()).unwrap();

        repository.char_purge(hero.account_id as u32, char_id).unwrap();

        assert!(repository.char_find(char_id).unwrap().is_none());
        assert!(repository.database.character_names.get(b"Hero").unwrap().is_none());
        assert!(repository.database.character_slots.is_empty());
        assert!(repository.database.inventories.get(hero.char_id.to_be_bytes()).unwrap().is_none());
        assert!(repository.database.inventory_owners.is_empty());
        assert!(repository.database.hotkeys.is_empty() && repository.database.skills.is_empty());
        assert!(repository.database.game_systems.get(prefixed_key(b"pet/", 7)).unwrap().is_none(), "the egg's pet record goes too");
        assert_eq!(repository.script_variable_char_num_fetch_one(char_id, "QUEST".into(), 0), 0);
        assert_eq!(repository.script_variable_char_str_fetch_one(char_id, "NAME$".into(), 0), "");
        assert_eq!(repository.script_variable_char_num_fetch_one(char_id + 1, "OTHER".into(), 0), 1, "other characters keep their variables");
    }

    #[test]
    fn purging_refuses_a_character_of_another_account() {
        let (repository, hero) = repository_with_character();
        assert!(repository.char_purge(hero.account_id as u32 + 1, hero.char_id as u32).is_err());
        assert!(repository.char_find(hero.char_id as u32).unwrap().is_some());
    }

    #[test]
    fn characters_are_found_by_name_ignoring_case_as_a_fallback() {
        let (repository, hero) = repository_with_character();
        assert_eq!(repository.char_find_by_name("Hero").unwrap().unwrap().char_id, hero.char_id);
        assert_eq!(repository.char_find_by_name("hERO").unwrap().unwrap().char_id, hero.char_id);
        assert!(repository.char_find_by_name("Nobody").unwrap().is_none());
    }

    #[test]
    fn rename_updates_the_character_name_index() {
        let (repository, hero) = repository_with_character();
        let record = repository.char_update(hero.char_id as u32, &|character| character.rename = 1).unwrap();
        assert_eq!(record.rename, 1);
        assert_eq!(repository.char_rename(hero.char_id as u32, "Legend", false).unwrap(), "Hero");
        assert_eq!(repository.char_find(hero.char_id as u32).unwrap().unwrap().name, "Legend");
        assert!(repository.character_game_systems(hero.char_id as u32).is_ok());
    }
}
