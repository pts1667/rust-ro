use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicI8;
use std::sync::Arc;

use models::status::StatusSnapshot;
use movement::position::Position;

use crate::server::model::map::MAP_EXT;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_flag_overrides::{MapFlagOverrides, SiegeFlag};
use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItemType, MapItems, ToMapItem, ToMapItemSnapshot};
use crate::server::model::request::Request;
use crate::server::model::script::Script;
use crate::server::model::session::{Session, SessionRegistry};
use crate::server::service::script_world_service::{companion_name, companion_snapshots, companion_status_snapshot};
use crate::server::state::character::Character;
use crate::server::state::character_directory::CharacterDirectory;
use crate::util::hasher::NoopHasherU32;

pub struct ServerState {
    ground_units: crate::server::model::ground_unit::GroundUnitSnapshots,
    map_items: MapItems,
    map_instances: HashMap<String, Vec<Arc<MapInstance>>>,
    map_instances_count: AtomicI8,
    sessions: SessionRegistry,
    directory: CharacterDirectory,
    characters: HashMap<u32, Character, NoopHasherU32>,
    locked_map_item: HashSet<u32, NoopHasherU32>, /* map item that should be removed from map instance, in next tick, avoid to use them
                                                   * meanwhile. */
    map_flag_overrides: MapFlagOverrides,
    siege: SiegeFlag,
    pub(crate) script_timers: crate::server::model::script_timer::ScriptTimers,
    pub(crate) character_logins: HashMap<u32, crate::server::model::script_timer::ScriptTimerOwner>,
    pub(crate) pending_character_logouts: HashMap<u32, crate::server::model::character_lifecycle::PendingCharacterLogout>,
    permission_groups: Arc<crate::server::model::permission_groups::PermissionGroups>,
    pub(crate) chat_rooms: crate::server::model::chat_room::ChatRooms,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::model::game_systems::{CompanionPosition, HomunculusRecord};
    use crate::server::model::map_instance::MapInstanceKey;
    use crate::server::service::script_world_service::{homunculus_world_id, world_data};

    #[test]
    fn companion_lookup_returns_actual_actor_and_rejects_inactive_or_other_map_actors() {
        let mut state = ServerState::new(MapItems::default());
        let mut owner = Character::new(
            "Owner".into(),
            150_000,
            2_000_000,
            models::status::Status::default(),
            50,
            60,
            0,
            "prontera".into(),
            1,
            vec![],
        );
        owner.loaded_from_client_side = true;
        owner.map_instance_key = MapInstanceKey::new("prontera".into(), 2);
        let homunculus = HomunculusRecord {
            id: 3,
            class_id: world_data().homunculi[0].class_id,
            name: "Owned Homunculus".into(),
            active: true,
            hp: 100,
            max_hp: 100,
            max_sp: 20,
            base_max_hp: 100,
            base_max_sp: 20,
            level: 12,
            stats: [10; 6],
            ..HomunculusRecord::default()
        };
        let id = homunculus_world_id(&homunculus);
        owner.game_systems.homunculus = Some(homunculus);
        owner.game_systems.rendered_companions.insert(id, CompanionPosition {
            x: 51,
            y: 62,
            map_instance: 2,
        });
        state.insert_character(owner);
        let map = "prontera".to_string();
        let item = state.map_item(id, &map, 2).unwrap();
        assert_eq!((item.id(), *item.object_type()), (id, MapItemType::Homunculus));
        let snapshot = state.map_item_snapshot(id, &map, 2).unwrap();
        assert_eq!((snapshot.x(), snapshot.y()), (51, 62));
        assert_eq!(state.map_item_x_y(&item, &"prontera.gat".into(), 2).unwrap().x, 51);
        assert_eq!(state.map_item_name(&item, &map, 2).as_deref(), Some("Owned Homunculus"));
        assert_eq!(state.map_item_mob_status(&item, &map, 2).unwrap().hp(), 100);
        assert_eq!(state.companion_owner(id, &map, 2).unwrap().char_id, 150_000);
        assert!(state.map_item(id, &"geffen".into(), 2).is_none());
        assert!(state.map_item_snapshot(id, &map, 0).is_none());
        assert!(state.map_item_mob_status(&item, &map, 0).is_none());
        state
            .characters_mut()
            .get_mut(&150_000)
            .unwrap()
            .game_systems
            .homunculus
            .as_mut()
            .unwrap()
            .active = false;
        assert!(state.map_item(id, &map, 2).is_none());
        assert!(state.map_item_snapshot(id, &map, 2).is_none());
        assert!(state.map_item_mob_status(&item, &map, 2).is_none());
        assert!(
            state
                .map_item_character(&MapItem::new(150_001, 0, MapItemType::Character))
                .is_none()
        );
    }
}

pub fn find_map_instance(
    map_instances: &HashMap<String, Vec<Arc<MapInstance>>>,
    map_name: &String,
    map_instance_id: u8,
) -> Option<Arc<MapInstance>> {
    let map_name = if map_name.ends_with(MAP_EXT) {
        &map_name[..(map_name.len() - 4)]
    } else {
        map_name.as_str()
    };

    map_instances
        .get(map_name)?
        .iter()
        .find(|map_instance| map_instance_id == map_instance.key().map_instance())
        .cloned()
}

impl ServerState {
    pub(crate) fn ground_units(&self) -> &crate::server::model::ground_unit::GroundUnitSnapshots {
        &self.ground_units
    }

    pub(crate) fn ground_unit(&self, id: u32, map: &str, instance: u8) -> Option<crate::server::model::ground_unit::GroundUnitSnapshot> {
        self.ground_units.get(id, map, instance, crate::util::tick::get_tick())
    }

    pub fn new(map_items: MapItems) -> Self {
        Self {
            ground_units: Default::default(),
            map_items,
            map_instances: Default::default(),
            map_instances_count: Default::default(),
            sessions: SessionRegistry::default(),
            directory: CharacterDirectory::default(),
            characters: Default::default(),
            locked_map_item: Default::default(),
            map_flag_overrides: MapFlagOverrides::default(),
            siege: SiegeFlag::default(),
            script_timers: Default::default(),
            character_logins: Default::default(),
            pending_character_logouts: Default::default(),
            permission_groups: Default::default(),
            chat_rooms: Default::default(),
        }
    }

    pub fn set_permission_groups(&mut self, groups: crate::server::model::permission_groups::PermissionGroups) {
        self.permission_groups = Arc::new(groups);
    }

    pub fn permission_groups(&self) -> &crate::server::model::permission_groups::PermissionGroups {
        &self.permission_groups
    }

    /// Group of the account's current session; accounts without one are in the default group.
    pub fn group_id_of(&self, account_id: u32) -> u32 {
        self.sessions.find(account_id).map_or(crate::server::model::permission_groups::DEFAULT_GROUP_ID, |session| session.account.group_id)
    }

    pub fn has_permission(&self, account_id: u32, permission: crate::server::model::permission_groups::Permission) -> bool {
        self.permission_groups.has_permission(self.group_id_of(account_id), permission)
    }

    pub fn remove_session(&self, session_id: u32) {
        self.sessions.remove(session_id);
    }

    pub fn add_session(&self, session_id: u32, session: Arc<Session>) {
        self.sessions.add(session_id, session);
    }

    pub fn get_session(&self, session_id: u32) -> Arc<Session> {
        self.sessions.get(session_id)
    }

    pub fn find_session(&self, session_id: u32) -> Option<Arc<Session>> {
        self.sessions.find(session_id)
    }

    pub fn get_character_unsafe(&self, char_id: u32) -> &Character {
        self.characters.get(&char_id).unwrap()
    }

    pub fn get_character(&self, char_id: u32) -> Option<&Character> {
        self.characters.get(&char_id)
    }

    pub fn get_character_from_context_unsafe(&self, context: &Request) -> &Character {
        let char_id = context.session().char_id.unwrap();
        self.characters.get(&char_id).unwrap()
    }

    pub fn insert_character(&mut self, character: Character) {
        if !self.pending_character_logouts.contains_key(&character.char_id) {
            self.map_items.insert(character.char_id, character.to_map_item());
        }
        self.characters.insert(character.char_id, character);
    }

    /// Runs `operation` with the character taken out of `characters`, so it can use the rest of the state without aliasing it.
    pub fn with_character_taken<R>(&mut self, char_id: u32, operation: impl FnOnce(&mut ServerState, &mut Character) -> R) -> Option<R> {
        let mut character = self.characters.remove(&char_id)?;
        let result = operation(self, &mut character);
        self.insert_character(character);
        Some(result)
    }

    pub(crate) fn retire_character_items(&mut self, char_id: u32, account_id: u32) {
        self.map_items.remove(char_id);
        self.map_items.remove(account_id);
    }

    pub fn insert_map_item(&mut self, id: u32, map_item: MapItem) {
        self.map_items.insert(id, map_item);
    }

    pub fn directory(&self) -> &CharacterDirectory {
        &self.directory
    }

    pub fn map_flag_overrides(&self) -> &MapFlagOverrides {
        &self.map_flag_overrides
    }

    pub fn siege(&self) -> &SiegeFlag {
        &self.siege
    }

    pub fn siege_active(&self) -> bool {
        self.siege.get()
    }

    pub fn publish_directory(&self) {
        self.directory.publish(&self.characters);
    }

    pub fn sessions(&self) -> &SessionRegistry {
        &self.sessions
    }

    pub fn characters(&self) -> &HashMap<u32, Character, NoopHasherU32> {
        &self.characters
    }

    pub fn characters_mut(&mut self) -> &mut HashMap<u32, Character, NoopHasherU32> {
        &mut self.characters
    }

    pub fn map_instances(&self) -> &HashMap<String, Vec<Arc<MapInstance>>> {
        &self.map_instances
    }

    pub fn map_instances_mut(&mut self) -> &mut HashMap<String, Vec<Arc<MapInstance>>> {
        &mut self.map_instances
    }

    pub fn map_instances_count(&self) -> &AtomicI8 {
        &self.map_instances_count
    }

    pub fn insert_locked_map_item(&mut self, id: u32) {
        self.locked_map_item.insert(id);
    }

    pub fn contains_locked_map_item(&self, id: u32) -> bool {
        self.locked_map_item.contains(&id)
    }

    pub fn remove_locked_map_item(&mut self, id: u32) {
        self.locked_map_item.remove(&id);
    }

    #[inline]
    pub fn get_map_instance_from_character(&self, character: &Character) -> Option<Arc<MapInstance>> {
        self.get_map_instance(character.current_map_name(), character.current_map_instance())
    }

    #[inline]
    pub fn get_map_instance(&self, map_name: &String, map_instance_id: u8) -> Option<Arc<MapInstance>> {
        find_map_instance(self.map_instances(), map_name, map_instance_id)
    }

    /// Disjoint borrows for loops that mutate characters while looking up the map they stand on.
    pub fn characters_mut_with_map_instances(
        &mut self,
    ) -> (&mut HashMap<u32, Character, NoopHasherU32>, &HashMap<String, Vec<Arc<MapInstance>>>) {
        (&mut self.characters, &self.map_instances)
    }

    pub fn companion_owner(&self, actor_id: u32, map_name: &String, map_instance_id: u8) -> Option<&Character> {
        self.characters.values().find(|owner| {
            owner.loaded_from_client_side
                && owner.current_map_instance() == map_instance_id
                && owner.current_map_name().trim_end_matches(MAP_EXT) == map_name.trim_end_matches(MAP_EXT)
                && companion_snapshots(owner)
                    .iter()
                    .any(|snapshot| snapshot.map_item().id() == actor_id)
        })
    }

    fn companion_snapshot(&self, actor_id: u32, map_name: &String, map_instance_id: u8) -> Option<MapItemSnapshot> {
        let owner = self.companion_owner(actor_id, map_name, map_instance_id)?;
        companion_snapshots(owner)
            .into_iter()
            .find(|snapshot| snapshot.map_item().id() == actor_id)
    }

    #[inline]
    pub fn map_item_x_y(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Position> {
        match map_item.object_type() {
            MapItemType::SkillUnit => self
                .ground_unit(map_item.id(), map_name, map_instance_id)
                .map(|unit| unit.snapshot().position()),
            MapItemType::Pet | MapItemType::Homunculus | MapItemType::Mercenary => self
                .companion_snapshot(map_item.id(), map_name, map_instance_id)
                .map(|snapshot| snapshot.position()),
            MapItemType::Character => {
                let characters = self.characters();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(Position {
                        x: character.x(),
                        y: character.y(),
                        dir: 3,
                    }); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(Position {
                            x: mob.x(),
                            y: mob.y(),
                            dir: 3,
                        }); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(Position {
                            x: warp.x(),
                            y: warp.y(),
                            dir: 0,
                        });
                    }
                }
                None
            }
            MapItemType::Unknown => None,
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(npc) = map_instance.state().script_skill_state.npcs.get(&map_item.id()) {
                        return Some(Position {
                            x: npc.x,
                            y: npc.y,
                            dir: npc.dir,
                        });
                    }
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(Position {
                            x: script.x(),
                            y: script.y(),
                            dir: script.dir(),
                        });
                    }
                }
                None
            }
            MapItemType::DroppedItem => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(dropped_item) = map_instance.state().get_dropped_item(map_item.id()) {
                        return Some(Position {
                            x: dropped_item.x(),
                            y: dropped_item.y(),
                            dir: 0,
                        });
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item_name(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<String> {
        match map_item.object_type() {
            MapItemType::SkillUnit => self.ground_unit(map_item.id(), map_name, map_instance_id).and_then(|unit| {
                crate::server::script::skill::metadata::SkillMetadata::find(unit.skill_id).map(|metadata| metadata.name.clone())
            }),
            MapItemType::Pet | MapItemType::Homunculus | MapItemType::Mercenary => self
                .companion_owner(map_item.id(), map_name, map_instance_id)
                .and_then(|owner| companion_name(owner, map_item.id())),
            MapItemType::Character => {
                let characters = self.characters();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(character.name.clone()); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(mob.name_english.clone()); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(warp.name);
                    }
                }
                None
            }
            MapItemType::Unknown => None,
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script.name().clone());
                    }
                }
                None
            }
            MapItemType::DroppedItem => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(dropped_item) = map_instance.state().get_dropped_item(map_item.id()) {
                        return Some(dropped_item.item_id().to_string());
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item(&self, map_item_id: u32, map_name: &String, map_instance_id: u8) -> Option<MapItem> {
        if let Some(unit) = self.ground_unit(map_item_id, map_name, map_instance_id) {
            return Some(unit.map_item());
        }
        let characters = self.characters();
        if let Some(character) = characters.get(&map_item_id) {
            return Some(character.to_map_item());
        }
        if let Some(companion) = self.companion_snapshot(map_item_id, map_name, map_instance_id) {
            return Some(companion.map_item());
        }
        if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
            if let Some(mob) = map_instance.state().get_mob(map_item_id) {
                return Some(mob.to_map_item());
            }
            if let Some(warp) = map_instance.get_warp(map_item_id) {
                return Some(warp.to_map_item());
            }
            if let Some(script) = map_instance.get_script(map_item_id) {
                return Some(script.to_map_item());
            }
        }
        None
    }

    pub fn map_item_script(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Arc<Script>> {
        match map_item.object_type() {
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script);
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn map_item_character(&self, map_item: &MapItem) -> Option<&Character> {
        match map_item.object_type() {
            MapItemType::Character => self.get_character(map_item.id()),
            _ => None,
        }
    }

    pub fn map_item_mob_status(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<StatusSnapshot> {
        match map_item.object_type() {
            MapItemType::SkillUnit => self.ground_unit(map_item.id(), map_name, map_instance_id).map(|unit| unit.status()),
            MapItemType::Homunculus | MapItemType::Mercenary => self
                .companion_owner(map_item.id(), map_name, map_instance_id)
                .and_then(|owner| companion_status_snapshot(owner, map_item.id())),
            MapItemType::Npc => {
                let instance = self.get_map_instance(map_name, map_instance_id)?;
                let state = instance.state();
                state
                    .script_skill_state
                    .npcs
                    .get(&map_item.id())
                    .map(crate::server::script::skill::actor::NpcSkillState::snapshot)
                    .or_else(|| {
                        instance
                            .get_script(map_item.id())
                            .map(|script| crate::server::script::skill::actor::NpcSkillState::uninitialized(script.as_ref()).snapshot())
                    })
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
                    if let Some(mob) = map_instance.state().get_mob(map_item.id()) {
                        return Some(mob.status.clone());
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn map_item_snapshot(&self, map_item_id: u32, map_name: &String, map_instance_id: u8) -> Option<MapItemSnapshot> {
        if let Some(unit) = self.ground_unit(map_item_id, map_name, map_instance_id) {
            return Some(unit.snapshot());
        }
        let characters = self.characters();
        if let Some(character) = characters.get(&map_item_id) {
            return Some(character.to_map_item_snapshot());
        }
        if let Some(companion) = self.companion_snapshot(map_item_id, map_name, map_instance_id) {
            return Some(companion);
        }
        if let Some(map_instance) = self.get_map_instance(map_name, map_instance_id) {
            if let Some(mob) = map_instance.state().get_mob(map_item_id) {
                return Some(mob.to_map_item_snapshot());
            }
            if let Some(_warp) = map_instance.get_warp(map_item_id) {
                return None;
            }
            if let Some(npc) = map_instance.state().script_skill_state.npcs.get(&map_item_id) {
                return Some(MapItemSnapshot::new(
                    MapItem::new(npc.id, npc.sprite as i16, MapItemType::Npc),
                    movement::position::Position {
                        x: npc.x,
                        y: npc.y,
                        dir: npc.dir,
                    },
                ));
            }
            if let Some(script) = map_instance.get_script(map_item_id) {
                return Some(MapItemSnapshot::new(
                    MapItem::new(script.id, script.sprite as i16, MapItemType::Npc),
                    movement::position::Position {
                        x: script.x,
                        y: script.y,
                        dir: script.dir,
                    },
                ));
            }
        }
        None
    }
}
