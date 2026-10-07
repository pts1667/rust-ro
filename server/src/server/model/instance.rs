//! Memorial dungeons: the `instance_db` definitions and the instances that are alive.
//!
//! An instance owns one copy of each of its maps; they are map instances of the same instance id, which is the id
//! scripts see (`instance_id()`), and lets the NPCs of every map find each other.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

use script_sdk::Reply;
use serde::Deserialize;
use tokio::sync::oneshot;

#[derive(Debug, Clone, Deserialize)]
pub struct InstanceDefinition {
    pub id: u32,
    pub name: String,
    /// Lifetime in seconds, 0 for none.
    pub time_limit: u64,
    /// Seconds an empty instance survives, 0 for ever.
    pub idle_timeout: u64,
    pub no_npc: bool,
    pub no_map_flag: bool,
    pub destroyable: bool,
    pub enter_map: String,
    pub enter_x: u16,
    pub enter_y: u16,
    pub maps: Vec<String>,
}

impl InstanceDefinition {
    /// Entry map first, then the additional maps.
    pub fn all_maps(&self) -> impl Iterator<Item = &String> {
        std::iter::once(&self.enter_map).chain(self.maps.iter())
    }
}

pub fn definitions() -> &'static [InstanceDefinition] {
    static DEFINITIONS: OnceLock<Vec<InstanceDefinition>> = OnceLock::new();
    DEFINITIONS.get_or_init(|| serde_json::from_str(include_str!("instances.json")).expect("Invalid instance database"))
}

pub fn definition_by_name(name: &str) -> Option<&'static InstanceDefinition> {
    definitions().iter().find(|definition| definition.name == name)
}

/// Memorial maps only exist as copies the instance service creates; nothing else may spawn one.
pub fn is_memorial_map(map: &str) -> bool {
    definitions().iter().any(|definition| definition.all_maps().any(|name| name == map))
}

pub fn definition_by_id(id: u32) -> Option<&'static InstanceDefinition> {
    definitions().iter().find(|definition| definition.id == id)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceMode {
    None,
    Char,
    Party,
    Guild,
    Clan,
}

impl InstanceMode {
    pub fn from_number(number: i32) -> Option<Self> {
        Some(match number {
            0 => Self::None,
            1 => Self::Char,
            2 => Self::Party,
            3 => Self::Guild,
            4 => Self::Clan,
            _ => return None,
        })
    }

    pub fn number(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone)]
pub struct MemorialInstance {
    /// Instance id of every map copy.
    pub id: u8,
    pub definition: &'static InstanceDefinition,
    pub mode: InstanceMode,
    pub owner_id: u32,
    /// Unix seconds.
    pub keep_deadline: Option<u64>,
    pub idle_deadline: Option<u64>,
    /// Set once the instance is being destroyed: nobody can enter, the maps go when they are empty.
    pub closing_since: Option<u64>,
    /// Source map names, entry map first.
    pub maps: Vec<String>,
    pub item_ranges: Vec<u32>,
    /// The map caches are still being read; the instance has no maps and nobody can enter.
    pub building: bool,
}

/// What an instance waiting for its maps holds: the script that called `instance_create` and the map cells read off the game loop.
pub struct PendingBuild {
    pub response: Option<oneshot::Sender<Reply>>,
    pub cells: Option<Result<Vec<Vec<u16>>, String>>,
}

#[derive(Default)]
struct InstanceBook {
    instances: BTreeMap<u8, MemorialInstance>,
    last_check: u64,
    /// Instances being built, oldest first; the position in this queue is what the client shows.
    build_queue: Vec<u8>,
    builds: std::collections::HashMap<u8, PendingBuild>,
}

/// Cloneable handle: instances are read by the game loop and by script requests.
#[derive(Clone, Default)]
pub struct Instances(Arc<Mutex<InstanceBook>>);

impl Instances {
    fn book(&self) -> MutexGuard<'_, InstanceBook> {
        self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// True once per second of `now`; timers are checked at that pace.
    pub fn due(&self, now: u64) -> bool {
        let mut book = self.book();
        let due = book.last_check != now;
        book.last_check = now;
        due
    }

    /// Registers an instance that is being built and returns its 1-based position in the queue.
    pub fn begin_build(&self, instance: MemorialInstance, response: oneshot::Sender<Reply>) -> usize {
        let mut book = self.book();
        let id = instance.id;
        book.instances.insert(id, instance);
        book.builds.insert(id, PendingBuild { response: Some(response), cells: None });
        book.build_queue.push(id);
        book.build_queue.len()
    }

    pub fn store_cells(&self, id: u8, cells: Result<Vec<Vec<u16>>, String>) {
        if let Some(build) = self.book().builds.get_mut(&id) {
            build.cells = Some(cells);
        }
    }

    pub fn take_build(&self, id: u8) -> Option<PendingBuild> {
        let mut book = self.book();
        book.build_queue.retain(|queued| *queued != id);
        book.builds.remove(&id)
    }

    pub fn build_queue(&self) -> Vec<u8> {
        self.book().build_queue.clone()
    }

    pub fn insert(&self, instance: MemorialInstance) {
        self.book().instances.insert(instance.id, instance);
    }

    pub fn get(&self, id: u8) -> Option<MemorialInstance> {
        self.book().instances.get(&id).cloned()
    }

    pub fn update<R>(&self, id: u8, change: impl FnOnce(&mut MemorialInstance) -> R) -> Option<R> {
        self.book().instances.get_mut(&id).map(change)
    }

    pub fn remove(&self, id: u8) -> Option<MemorialInstance> {
        self.book().instances.remove(&id)
    }

    pub fn contains(&self, id: u8) -> bool {
        self.book().instances.contains_key(&id)
    }

    /// Instance an owner booked, closing ones do not count.
    pub fn of_owner(&self, mode: InstanceMode, owner_id: u32) -> Option<u8> {
        self.book().instances.values().find(|instance| instance.mode == mode && instance.owner_id == owner_id && instance.closing_since.is_none()).map(|instance| instance.id)
    }

    pub fn snapshot(&self) -> Vec<MemorialInstance> {
        self.book().instances.values().cloned().collect()
    }

    pub fn of_map(&self, map: &str, instance_id: u8) -> Option<MemorialInstance> {
        self.book().instances.get(&instance_id).filter(|instance| instance.maps.iter().any(|name| name == map)).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_lists_the_pre_renewal_dungeons() {
        let names: Vec<_> = definitions().iter().map(|definition| definition.name.as_str()).collect();
        assert_eq!(names, ["Endless Tower", "Sealed Catacomb", "Orc's Memory", "Nidhoggur's Nest"]);
        let tower = definition_by_name("Endless Tower").unwrap();
        assert_eq!(tower.all_maps().count(), 6);
    }

    #[test]
    fn owners_find_only_open_instances() {
        let instances = Instances::default();
        let definition = definition_by_id(3).unwrap();
        instances.insert(MemorialInstance { id: 5, definition, mode: InstanceMode::Party, owner_id: 9, keep_deadline: None, idle_deadline: None, closing_since: None, maps: vec!["1@orcs".into()], item_ranges: vec![], building: false });
        assert_eq!(instances.of_owner(InstanceMode::Party, 9), Some(5));
        assert_eq!(instances.of_owner(InstanceMode::Guild, 9), None);
        instances.update(5, |instance| instance.closing_since = Some(1));
        assert_eq!(instances.of_owner(InstanceMode::Party, 9), None);
        assert!(instances.of_map("1@orcs", 5).is_some() && instances.of_map("2@orcs", 5).is_none());
    }
}
