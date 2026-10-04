use std::collections::{HashMap, VecDeque};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use models::item::DroppedItem;

use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItems, ToMapItem};
use crate::server::state::mob::Mob;
use crate::util::coordinate;
use crate::util::hasher::NoopHasherU32;

#[derive(SettersAll)]
pub struct MapInstanceState {
    key: MapInstanceKey,
    pub flags: crate::server::model::map_flags::MapFlags,
    x_size: u16,
    y_size: u16,
    // index in this array will give x and y position of the cell.
    cells: Vec<u16>,
    mobs: HashMap<u32, Mob, NoopHasherU32>,
    map_items: MapItems,
    dropped_items: hashbrown::HashMap<u32, DroppedItem, NoopHasherU32>,
    mob_spawns_tracks: HashMap<u32, MobSpawnTrack>,
    /// Character snapshots for mob AI targeting (updated via UpdateMobsFov)
    characters: Vec<MapItemSnapshot>,
    pub actor_visibility: HashMap<u32, crate::server::service::visibility_service::StealthState>,
    pub script_skill_state: crate::server::script::skill::actor::MapSkillState,
    pub pending_pet_captures: HashMap<u64, PendingPetCapture>,
    pub pending_pet_loot: HashMap<u64, PendingPetLoot>,
    pub pending_pet_loot_drops: HashMap<u64, PendingPetLootDrop>,
    pub completed_pet_loot: HashMap<u64, CompletedPetLoot>,
    pub completed_pet_loot_order: VecDeque<u64>,
    pub completed_pet_loot_drops: HashMap<u64, CompletedPetLootDrop>,
    pub completed_pet_loot_drop_order: VecDeque<u64>,

    mob_movement_paused: bool,
}

pub struct PendingPetCapture {
    pub char_id: u32,
    pub mob: Mob,
}

pub struct PendingPetLoot {
    pub char_id: u32,
    pub pet_id: u32,
    pub item: DroppedItem,
}

pub struct PendingPetLootDrop {
    pub char_id: u32,
    pub x: u16,
    pub y: u16,
    pub fingerprint: u64,
    pub items: Vec<DroppedItem>,
}

pub struct CompletedPetLoot {
    pub char_id: u32,
    pub pet_id: u32,
    pub target_id: u32,
    pub item: Option<DroppedItem>,
}

pub struct CompletedPetLootDrop {
    pub char_id: u32,
    pub fingerprint: u64,
    pub accepted: bool,
}

pub struct MobSpawnTrack {
    pub spawn_id: u32,
    pub spawned_amount: i16,
    pub mob_respawn_at: Vec<u128>,
}

impl MobSpawnTrack {
    pub fn default(spawn_id: u32) -> MobSpawnTrack {
        MobSpawnTrack {
            spawn_id,
            spawned_amount: 0,
            mob_respawn_at: Default::default(),
        }
    }

    pub fn increment_spawn(&mut self) {
        self.spawned_amount += 1;
    }

    pub fn decrement_spawn(&mut self) {
        self.spawned_amount -= 1;
    }
}

impl MapInstanceState {
    pub fn new(
        key: MapInstanceKey,
        x_size: u16,
        y_size: u16,
        cells: Vec<u16>,
        map_items: MapItems,
        mob_spawns_tracks: HashMap<u32, MobSpawnTrack>,
    ) -> MapInstanceState {
        Self {
            key,
            flags: Default::default(),
            x_size,
            y_size,
            cells,
            mobs: Default::default(),
            map_items,
            dropped_items: Default::default(),
            mob_spawns_tracks,
            characters: Vec::new(),
            actor_visibility: Default::default(),
            script_skill_state: Default::default(),
            pending_pet_captures: Default::default(),
            pending_pet_loot: Default::default(),
            pending_pet_loot_drops: Default::default(),
            completed_pet_loot: Default::default(),
            completed_pet_loot_order: Default::default(),
            completed_pet_loot_drops: Default::default(),
            completed_pet_loot_drop_order: Default::default(),
            mob_movement_paused: false,
        }
    }

    #[inline]
    pub fn get_cell_index_of(&self, x: u16, y: u16) -> usize {
        coordinate::get_cell_index_of(x, y, self.x_size())
    }

    pub fn is_warp_cell(&self, x: u16, y: u16) -> bool {
        if self.cells().is_empty() {
            warn!("Cannot call is_warp_cell as cells are not initialized, returning false");
            return false;
        }
        let i = self.get_cell_index_of(x, y);
        match self.cells().get(i) {
            Some(value) => (value & CellType::Warp.as_flag()) == CellType::Warp.as_flag(),
            None => false,
        }
    }

    pub fn insert_item(&mut self, map_item: MapItem) {
        self.map_items.insert(map_item.id(), map_item);
    }

    pub fn remove_item(&mut self, map_item: MapItem) {
        self.remove_item_with_id(map_item.id());
    }

    pub fn remove_item_with_id(&mut self, id: u32) {
        self.map_items.remove(id);
    }

    pub fn get_mob(&self, mob_id: u32) -> Option<&Mob> {
        self.mobs().get(&mob_id)
    }

    pub fn mobs(&self) -> &HashMap<u32, Mob, NoopHasherU32> {
        &self.mobs
    }

    pub fn mobs_mut(&mut self) -> &mut HashMap<u32, Mob, NoopHasherU32> {
        &mut self.mobs
    }

    pub fn insert_mob(&mut self, mob: Mob) {
        self.insert_item(mob.to_map_item());
        self.mobs_mut().insert(mob.id, mob);
    }

    pub fn remove_mob(&mut self, id: u32) -> Option<Mob> {
        if let Some(mob) = self.mobs_mut().remove(&id) {
            self.remove_item(mob.to_map_item());
            Some(mob)
        } else {
            None
        }
    }

    pub fn get_dropped_item(&self, dropped_item_id: u32) -> Option<&DroppedItem> {
        self.dropped_items().get(&dropped_item_id)
    }

    fn dropped_items(&self) -> &hashbrown::HashMap<u32, DroppedItem, NoopHasherU32> {
        &self.dropped_items
    }

    fn dropped_items_mut(&mut self) -> &mut hashbrown::HashMap<u32, DroppedItem, NoopHasherU32> {
        &mut self.dropped_items
    }

    pub fn insert_dropped_item(&mut self, dropped_item: DroppedItem) {
        self.insert_item(dropped_item.to_map_item());
        self.dropped_items_mut()
            .insert_unique_unchecked(dropped_item.map_item_id, dropped_item);
    }

    pub fn remove_dropped_item(&mut self, id: u32) -> Option<DroppedItem> {
        if self.is_pet_loot_reserved(id) {
            return None;
        }
        if let Some(dropped_item) = self.dropped_items_mut().remove(&id) {
            self.remove_item(dropped_item.to_map_item());
            Some(dropped_item)
        } else {
            None
        }
    }

    pub fn is_pet_loot_reserved(&self, id: u32) -> bool {
        self.pending_pet_loot.values().any(|claim| claim.item.map_item_id == id)
            || self
                .pending_pet_loot_drops
                .values()
                .any(|claim| claim.items.iter().any(|item| item.map_item_id == id))
    }

    pub fn reserve_pet_loot(&mut self, claim_id: u64, char_id: u32, pet_id: u32, target_id: u32) -> Option<DroppedItem> {
        let item = self.dropped_items_mut().remove(&target_id)?;
        self.pending_pet_loot.insert(claim_id, PendingPetLoot { char_id, pet_id, item });
        Some(item)
    }

    pub fn restore_pet_loot(&mut self, item: DroppedItem) {
        self.dropped_items_mut().insert_unique_unchecked(item.map_item_id, item);
    }

    pub fn map_items(&self) -> &hashbrown::HashMap<u32, MapItem, NoopHasherU32> {
        self.map_items.get()
    }

    pub fn map_items_mut(&mut self) -> &mut MapItems {
        &mut self.map_items
    }

    pub fn get_map_item(&self, item_id: u32) -> Option<&MapItem> {
        self.map_items().get(&item_id)
    }

    pub fn mob_spawns_tracks(&self) -> &HashMap<u32, MobSpawnTrack> {
        &self.mob_spawns_tracks
    }

    pub fn mob_spawns_tracks_mut(&mut self) -> &mut HashMap<u32, MobSpawnTrack> {
        &mut self.mob_spawns_tracks
    }

    pub fn key(&self) -> &MapInstanceKey {
        &self.key
    }

    pub fn cells(&self) -> &Vec<u16> {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut Vec<u16> {
        &mut self.cells
    }

    pub fn x_size(&self) -> u16 {
        self.x_size
    }

    pub fn y_size(&self) -> u16 {
        self.y_size
    }

    pub fn mob_movement_paused(&self) -> bool {
        self.mob_movement_paused
    }

    pub fn characters(&self) -> &[MapItemSnapshot] {
        &self.characters
    }

    pub fn update_characters(&mut self, characters: Vec<MapItemSnapshot>) {
        self.characters = characters;
    }
}
