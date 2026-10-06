use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use models::status::StatusSnapshot;
use movement::position::Position;

use super::map_instance::MapInstanceKey;
use super::map_item::{MapItem, MapItemSnapshot, MapItemType};

#[derive(Clone, Debug)]
pub(crate) struct GroundUnitSnapshot {
    pub id: u32,
    pub skill_id: u32,
    pub map: MapInstanceKey,
    pub x: u16,
    pub y: u16,
    pub hp: u16,
    pub expires_at: u128,
    pub view_id: u32,
    pub used: bool,
}

impl GroundUnitSnapshot {
    pub fn alive(&self, tick: u128) -> bool {
        self.hp > 0 && self.expires_at > tick
    }

    pub fn map_item(&self) -> MapItem {
        MapItem::new(self.id, self.view_id as i16, MapItemType::SkillUnit)
    }

    pub fn snapshot(&self) -> MapItemSnapshot {
        MapItemSnapshot::new(self.map_item(), Position {
            x: self.x,
            y: self.y,
            dir: 0,
        })
    }

    pub fn status(&self) -> StatusSnapshot {
        StatusSnapshot::new_for_skill_unit(self.hp > 0)
    }
}

/// Snapshots of trap-like ground units by unit id, rebuilt from the ground skills that own the real data.
/// Cheap to clone; every clone shares one table. Lookups return copies, so no lock is held while callers work.
#[derive(Clone, Debug, Default)]
pub(crate) struct GroundUnitSnapshots {
    units: Arc<RwLock<HashMap<u32, GroundUnitSnapshot>>>,
}

impl GroundUnitSnapshots {
    fn read(&self) -> std::sync::RwLockReadGuard<'_, HashMap<u32, GroundUnitSnapshot>> {
        self.units.read().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, HashMap<u32, GroundUnitSnapshot>> {
        self.units.write().unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn get(&self, id: u32, map: &str, instance: u8, tick: u128) -> Option<GroundUnitSnapshot> {
        let units = self.read();
        let unit = units.get(&id)?;
        (unit.map.map_name() == map && unit.map.map_instance() == instance && unit.alive(tick)).then(|| unit.clone())
    }

    pub fn contains(&self, id: u32) -> bool {
        self.read().contains_key(&id)
    }

    pub fn map_of(&self, id: u32) -> Option<MapInstanceKey> {
        self.read().get(&id).map(|unit| unit.map.clone())
    }

    pub fn matching(&self, keep: impl Fn(&GroundUnitSnapshot) -> bool) -> Vec<GroundUnitSnapshot> {
        self.read().values().filter(|unit| keep(unit)).cloned().collect()
    }

    pub fn set(&self, unit: Option<GroundUnitSnapshot>, id: u32) {
        let mut units = self.write();
        match unit {
            Some(unit) => units.insert(id, unit),
            None => units.remove(&id),
        };
    }

    pub fn replace_all(&self, units: HashMap<u32, GroundUnitSnapshot>) {
        *self.write() = units;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unit(id: u32, hp: u16, expires_at: u128) -> GroundUnitSnapshot {
        GroundUnitSnapshot {
            id,
            skill_id: 1,
            map: MapInstanceKey::new("prontera".into(), 0),
            x: 10,
            y: 10,
            hp,
            expires_at,
            view_id: 1,
            used: false,
        }
    }

    #[test]
    fn lookup_requires_matching_map_and_a_live_unit() {
        let units = GroundUnitSnapshots::default();
        units.replace_all(HashMap::from([(1, unit(1, 5, 100)), (2, unit(2, 0, 100))]));
        assert!(units.get(1, "prontera.gat", 0, 50).is_some());
        assert!(units.get(1, "prontera.gat", 1, 50).is_none());
        assert!(units.get(1, "geffen.gat", 0, 50).is_none());
        assert!(units.get(1, "prontera.gat", 0, 100).is_none());
        assert!(units.get(2, "prontera.gat", 0, 50).is_none());
        assert!(units.contains(2));
        assert_eq!(units.matching(|unit| unit.hp > 0).len(), 1);
    }

    #[test]
    fn clones_share_one_table() {
        let units = GroundUnitSnapshots::default();
        let remote = units.clone();
        remote.set(Some(unit(3, 5, 100)), 3);
        assert!(units.map_of(3).is_some());
        units.set(None, 3);
        assert!(!remote.contains(3));
    }
}
