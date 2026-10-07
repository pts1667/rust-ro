use database::{read, tx_write};
use sled::transaction::Transactional;

use crate::repository::{Error, SledRepository};
use crate::server::model::quest::QuestEntry;

fn quests_key(char_id: u32) -> Vec<u8> {
    [b"quests/".as_slice(), &char_id.to_be_bytes()].concat()
}

pub trait QuestRepository {
    fn quest_entries(&self, _char_id: u32) -> Result<Vec<QuestEntry>, Error> {
        Ok(Vec::new())
    }
    /// Replaces the whole quest log of the character, as rathena saves it.
    fn quest_entries_save(&self, _char_id: u32, _entries: &[QuestEntry]) -> Result<(), Error> {
        Err(Error::InvalidInput("Quest storage is unavailable".into()))
    }
}

impl QuestRepository for SledRepository {
    fn quest_entries(&self, char_id: u32) -> Result<Vec<QuestEntry>, Error> {
        Ok(read(&self.database.game_systems, &quests_key(char_id))?.unwrap_or_default())
    }

    fn quest_entries_save(&self, char_id: u32, entries: &[QuestEntry]) -> Result<(), Error> {
        let entries = entries.to_vec();
        Ok((&self.database.game_systems,).transaction(|(systems,)| tx_write(systems, &quests_key(char_id), &entries))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::model::quest::QuestState;

    #[test]
    fn quest_log_round_trips_per_character() {
        let repository = SledRepository::temporary().unwrap();
        let entry = QuestEntry { quest_id: 1000, state: QuestState::Active, expires_at: 99, counts: [1, 2, 3] };
        repository.quest_entries_save(7, &[entry]).unwrap();
        assert_eq!(repository.quest_entries(7).unwrap(), vec![entry]);
        assert!(repository.quest_entries(8).unwrap().is_empty());
    }
}
