use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;

use crate::server::state::character::Character;
use crate::util::hasher::NoopHasherU32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterPresence {
    pub account_id: u32,
    pub map_name: String,
    pub map_instance_id: u8,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
}

impl CharacterPresence {
    fn matches(&self, character: &Character) -> bool {
        self.account_id == character.account_id
            && self.map_instance_id == character.current_map_instance()
            && self.x == character.x()
            && self.y == character.y()
            && self.dir == character.dir()
            && self.map_name == *character.current_map_name()
    }

    fn of(character: &Character) -> Self {
        Self {
            account_id: character.account_id,
            map_name: character.current_map_name().clone(),
            map_instance_id: character.current_map_instance(),
            x: character.x(),
            y: character.y(),
            dir: character.dir(),
        }
    }
}

/// Read-only view of where characters are, for threads that must not read `ServerState`.
/// The game loop and the movement loop publish it while they hold the state lock, so readers
/// can lag by up to one loop tick.
#[derive(Clone, Default)]
pub struct CharacterDirectory(Arc<DashMap<u32, CharacterPresence>>);

impl CharacterDirectory {
    pub fn publish(&self, characters: &HashMap<u32, Character, NoopHasherU32>) {
        self.0.retain(|char_id, _| characters.contains_key(char_id));
        for (char_id, character) in characters {
            let unchanged = self.0.get(char_id).is_some_and(|presence| presence.matches(character));
            if !unchanged {
                self.0.insert(*char_id, CharacterPresence::of(character));
            }
        }
    }

    pub fn presence(&self, char_id: u32) -> Option<CharacterPresence> {
        self.0.get(&char_id).map(|presence| presence.clone())
    }

    pub fn in_fov(&self, map_name: &str, map_instance_id: u8, x: u16, y: u16, range: u16, exclude_id: Option<u32>) -> Vec<u32> {
        self.0
            .iter()
            .filter(|entry| {
                let presence = entry.value();
                presence.map_name == map_name
                    && presence.map_instance_id == map_instance_id
                    && crate::server::model::path::manhattan_distance(presence.x, presence.y, x, y) <= range
                    && exclude_id != Some(*entry.key())
            })
            .map(|entry| *entry.key())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn character(char_id: u32, map: &str, x: u16, y: u16) -> Character {
        Character::new(
            format!("char{char_id}"),
            char_id,
            2_000_000 + char_id,
            models::status::Status::default(),
            x,
            y,
            0,
            map.into(),
            0,
            vec![],
        )
    }

    fn characters(list: Vec<Character>) -> HashMap<u32, Character, NoopHasherU32> {
        list.into_iter().map(|character| (character.char_id, character)).collect()
    }

    #[test]
    fn in_fov_selects_characters_on_the_same_map_instance_within_range_excluding_the_sender() {
        let directory = CharacterDirectory::default();
        directory.publish(&characters(vec![
            character(1, "prontera.gat", 50, 50),
            character(2, "prontera.gat", 55, 55),
            character(3, "prontera.gat", 90, 90),
            character(4, "geffen.gat", 50, 50),
        ]));

        let mut all = directory.in_fov("prontera.gat", 0, 50, 50, 20, None);
        all.sort();
        let mut without_sender = directory.in_fov("prontera.gat", 0, 50, 50, 20, Some(1));
        without_sender.sort();

        assert_eq!(all, vec![1, 2]);
        assert_eq!(without_sender, vec![2]);
    }

    #[test]
    fn publish_follows_moves_and_forgets_removed_characters() {
        let directory = CharacterDirectory::default();
        let mut state = characters(vec![character(1, "prontera", 50, 50), character(2, "prontera", 51, 51)]);
        directory.publish(&state);

        state.get_mut(&1).unwrap().update_position(10, 12);
        state.remove(&2);
        directory.publish(&state);

        let presence = directory.presence(1).unwrap();
        assert_eq!((presence.x, presence.y, presence.account_id), (10, 12, 2_000_001));
        assert!(directory.presence(2).is_none());
    }
}
