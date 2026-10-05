use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use crate::server::model::map_flags::MapFlags;

/// Map flags changed at runtime, by map name and instance. Cheap to clone; every clone shares one table.
#[derive(Debug, Default, Clone)]
pub struct MapFlagOverrides {
    flags: Arc<RwLock<HashMap<(String, u8), MapFlags>>>,
}

impl MapFlagOverrides {
    pub fn get(&self, name: &str, instance: u8) -> Option<MapFlags> {
        let flags = self.flags.read().unwrap_or_else(|poisoned| poisoned.into_inner());
        flags.get(&(name.to_string(), instance)).cloned()
    }

    pub fn insert(&self, key: (String, u8), flags: MapFlags) {
        let mut table = self.flags.write().unwrap_or_else(|poisoned| poisoned.into_inner());
        table.insert(key, flags);
    }

    pub fn update<R>(&self, key: (String, u8), change: impl FnOnce(&mut MapFlags) -> R) -> R {
        let mut table = self.flags.write().unwrap_or_else(|poisoned| poisoned.into_inner());
        change(table.entry(key).or_default())
    }
}

/// Whether guild castle sieges are running. Cheap to clone; every clone shares one flag.
#[derive(Debug, Default, Clone)]
pub struct SiegeFlag {
    active: Arc<AtomicBool>,
}

impl SiegeFlag {
    pub fn get(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }

    pub fn set(&self, active: bool) {
        self.active.store(active, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clones_share_overrides_and_siege_flag() {
        let overrides = MapFlagOverrides::default();
        let remote = overrides.clone();
        remote.insert(("prontera".into(), 0), MapFlags::default());
        assert!(overrides.get("prontera", 0).is_some());
        assert!(overrides.get("prontera", 1).is_none());
        overrides.update(("geffen".into(), 0), |_| ());
        assert!(remote.get("geffen", 0).is_some());

        let siege = SiegeFlag::default();
        let other = siege.clone();
        other.set(true);
        assert!(siege.get());
    }
}
