use std::cell::RefCell;
use std::collections::HashSet;
use std::mem;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;

use models::enums::EnumWithNumberValue;
use models::enums::action::ActionType;
use models::enums::skill::SkillType;
use models::enums::skill_enums::SkillEnum;
use models::status::{Status, StatusSnapshot};
use models::status_bonus::BonusExpiry;
use movement::position::Position;
use packets::packets::{Packet, PacketZcMsgStateChange, PacketZcNotifyAct};
use script_runtime::WasmRuntime;

use crate::MAP_DIR;
use crate::server::boot::map_loader::MapLoader;
use crate::server::map_instance_loop::MapInstanceLoop;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::game_event::{
    CharacterChangeMap, CharacterMovement, CharacterRemoveFromMap, CharacterUseSkill, GameEvent, CharacterClearFov, CharacterDamage, CharacterUpdateClientSideStats};
use crate::server::model::events::map_event::{MapEvent, MobDamage, RemoveDroppedItemFromMap};
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::{CHARACTER_MAX_MAP_ITEM_ID, MAP_INSTANCE_MAX_MAP_ITEM_ID, MapItemSnapshot, MapItemType, MapItems, ToMapItemSnapshot};
use crate::server::model::movement::{Movable, Movement};
use crate::server::model::path::{manhattan_distance, path_search_client_side_algorithm};
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::skill::ScriptSkillService;
use crate::server::service::battle_service::BattleService;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::server::service::script_service::ScriptService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::Server;
use crate::util::tick::get_tick;

#[allow(dead_code)]
pub struct ServerService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    movement_task_queue: Arc<TasksQueue<GameEvent>>,
    vm: Arc<WasmRuntime>,
    character_service: CharacterService,
    inventory_service: InventoryService,
    item_service: ItemService,
    skill_service: SkillService,
    battle_service: BattleService,
    status_service: &'static StatusService,
    script_service: ScriptService,
    skill_tree_service: SkillTreeService,
    script_skill_service: ScriptSkillService,
}

impl ServerService {
    pub(crate) fn notification_sender(&self) -> SyncSender<Notification> {
        self.client_notification_sender.clone()
    }

    pub fn schedule_warp_to_walkable_cell_by_character(&self, map: &str, x: u16, y: u16, char_id: u32) {
        self.server_task_queue.add_to_first_index(GameEvent::ScriptWarp(crate::server::model::events::game_event::ScriptWarp {
            char_id,
            map: map.to_string(),
            x,
            y,
            destination_instance: None,
        }));
    }

    pub fn schedule_warp_to_walkable_cell_by_character_in_instance(&self, map: &str, x: u16, y: u16, char_id: u32, instance_id: u8) {
        self.server_task_queue.add_to_first_index(GameEvent::ScriptWarp(crate::server::model::events::game_event::ScriptWarp {
            char_id, map: map.to_string(), x, y, destination_instance: Some(instance_id),
        }));
    }
    pub(crate) fn new(
        client_notification_sender: SyncSender<Notification>,
        configuration_service: &'static GlobalConfigService,
        server_task_queue: Arc<TasksQueue<GameEvent>>,
        movement_task_queue: Arc<TasksQueue<GameEvent>>,
        vm: Arc<WasmRuntime>,
        inventory_service: InventoryService,
        battle_service: BattleService,
        skill_service: SkillService,
        status_service: &'static StatusService,
        script_service: ScriptService,
        character_service: CharacterService,
        skill_tree_service: SkillTreeService,
        item_service: ItemService,
        script_skill_service: ScriptSkillService,
    ) -> Self {
        ServerService {
            client_notification_sender,
            configuration_service,
            server_task_queue,
            movement_task_queue,
            vm,
            inventory_service,
            battle_service,
            skill_service,
            status_service,
            script_service,
            character_service,
            skill_tree_service,
            item_service,
            script_skill_service,
        }
    }

    #[inline]
    pub fn skill_service(&self) -> &SkillService {
        &self.skill_service
    }

    #[inline]
    pub fn battle_service(&self) -> &BattleService {
        &self.battle_service
    }

    #[inline]
    pub fn script_service(&self) -> &ScriptService {
        &self.script_service
    }

    #[inline]
    pub fn inventory_service(&self) -> &InventoryService {
        &self.inventory_service
    }

    #[inline]
    pub fn character_service(&self) -> &CharacterService {
        &self.character_service
    }

    #[inline]
    pub fn skill_tree_service(&self) -> &SkillTreeService {
        &self.skill_tree_service
    }

    #[inline]
    pub fn item_service(&self) -> &ItemService {
        &self.item_service
    }

    #[inline]
    pub fn script_skill_service(&self) -> &ScriptSkillService {
        &self.script_skill_service
    }

    pub fn create_map_instance(&self, server_state: &mut ServerState, map: &'static Map, instance_id: u8) -> Arc<MapInstance> {
        self.create_map_instance_with(server_state, map, instance_id, false, None)
    }

    /// `memorial` gives the NPCs of the map the instance id as name suffix so scripts of other maps can address them (`instance_npcname`).
    pub fn create_map_instance_with(&self, server_state: &mut ServerState, map: &'static Map, instance_id: u8, memorial: bool, loaded_cells: Option<Vec<u16>>) -> Arc<MapInstance> {
        info!(
            "create map instance: {} x_size: {}, y_size {}, length: {}",
            map.name(),
            map.x_size(),
            map.y_size(),
            map.length()
        );
        let item_range = server_state.allocate_item_range();
        let start_sequence = CHARACTER_MAX_MAP_ITEM_ID + item_range * MAP_INSTANCE_MAX_MAP_ITEM_ID;
        let mut map_items = MapItems::new(start_sequence);

        let mut cells = loaded_cells.unwrap_or_else(|| MapLoader::generate_cells(map.name(), map.length() as usize, unsafe { MAP_DIR }));
        map.set_warp_cells(&mut cells, &mut map_items);

        let map_instance = MapInstance::from_map_with(
            self.vm.clone(),
            map,
            instance_id,
            cells,
            self.client_notification_sender.clone(),
            map_items,
            Arc::new(TasksQueue::new()),
            memorial,
            item_range,
        );
        map_instance.state_mut().flags = server_state.map_flags_for(map.name(), instance_id);
        server_state.map_instances_count().fetch_add(1, Relaxed);
        let map_instance_ref = Arc::new(map_instance);
        let entry = server_state.map_instances_mut().entry(map.name().to_string()).or_default();
        entry.push(map_instance_ref.clone());
        let map_instance_service = MapInstanceService::new(
            self.client_notification_sender.clone(),
            GlobalConfigService::instance(),
            MobService::new(self.client_notification_sender.clone(), GlobalConfigService::instance()),
            self.server_task_queue.clone(),
        );
        MapInstanceLoop::start_map_instance_thread(map_instance_ref.clone(), map_instance_service);
        if instance_id > 0 {
            for npc in map_instance_ref.state().script_skill_state.npcs.values() {
                if let Some(entry_id) = ScriptService::event_entry(&format!("{}::OnInstanceInit", npc.script.name)) {
                    self.server_task_queue.add_to_first_index(GameEvent::ScriptNpcEvent(crate::server::model::events::game_event::ScriptNpcEvent {
                        npc_id: npc.id, scope_instance: npc.script.scope_instance, entry_id, char_id: None, depth: 0, queued_until: 0, args: None, timer_guard: None,
                    }));
                }
            }
        }
        map_instance_ref
    }

    pub fn schedule_warp_to_walkable_cell(&self, server_state: &mut ServerState, destination_map: &str, x: u16, y: u16, char_id: u32) {
        self.schedule_warp_to_walkable_cell_in_instance(server_state, destination_map, x, y, char_id, 0);
    }

    pub fn schedule_warp_to_walkable_cell_in_instance(&self, server_state: &mut ServerState, destination_map: &str, x: u16, y: u16, char_id: u32, instance_id: u8) {
        let Some(character) = server_state.characters().get(&char_id) else {
            script_debug!("Warp to {destination_map} skipped: char {char_id} is not online");
            return;
        };
        let origin = character.map_instance_key.clone();
        let map_name = Map::name_without_ext(destination_map);
        let map_instance = if let Some(instance) = server_state.get_map_instance(&map_name, instance_id) { instance }
            else if instance_id != 0 && crate::server::model::instance::is_memorial_map(&map_name) {
                script_debug!("Warp of char {char_id} to {map_name} skipped: instance {instance_id} is gone");
                return;
            }
            else if let Some(map) = self.configuration_service.find_map(&map_name) { self.create_map_instance(server_state, map, instance_id) }
            else {
                script_debug!("Warp of char {char_id} to {map_name} ({x},{y}) skipped: map is not loaded");
                return;
            };
        self.server_task_queue.add_to_first_index(GameEvent::CharacterClearFov(CharacterClearFov { char_id }));
        self.server_task_queue.add_to_index(
            GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap {
                char_id,
                map_name: origin.map_name().clone(),
                instance_id: origin.map_instance(),
            }),
            0,
        );

        debug!("Char enter on map {}", map_name);
        script_debug!("Warp scheduled: char {char_id} from {} to {map_name} ({x},{y}) instance {}", origin.map_name(), map_instance.id());
        let (x, y) = if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
            let walkable_cell = Map::find_random_walkable_cell(map_instance.state().cells(), map_instance.x_size());
            (walkable_cell.0, walkable_cell.1)
        } else {
            (x, y)
        };

        self.server_task_queue.add_to_index(
            GameEvent::CharacterChangeMap(CharacterChangeMap {
                char_id,
                new_map_name: map_name,
                new_instance_id: map_instance.id(),
                new_position: Some(Position { x, y, dir: 3 }),
            }),
            2,
        );
    }

    pub fn character_attack(&self, server: &Server, server_state: &ServerState, tick: u128, character: &mut Character) {
        if !character.is_attacking() {
            return;
        }
        if !server.player_combat_target_allowed(server_state, character, character.attack().target)
            || !server.player_target_allowed(server_state, character, character.attack().target, super::visibility_service::TargetingMode::Direct) {
            character.clear_attack();
            return;
        }

        let map_item = server_state.map_item(
            character.attack().target,
            character.current_map_name(),
            character.current_map_instance(),
        );
        if let Some(map_item) = map_item {
            let range = self.get_status_snapshot(&character.status, tick).attack_range();
            let target_position = server_state
                .map_item_x_y(&map_item, character.current_map_name(), character.current_map_instance())
                .unwrap();
            let is_in_range = range as i16 >= manhattan_distance(character.x, character.y, target_position.x, target_position.y) as i16 - 1;
            let maybe_map_instance = server_state.get_map_instance(character.current_map_name(), character.current_map_instance());
            let map_instance = maybe_map_instance.as_ref().unwrap();
            if !is_in_range && !character.is_moving() {
                let path = path_search_client_side_algorithm(
                    map_instance.x_size(),
                    map_instance.y_size(),
                    map_instance.state().cells(),
                    character.x,
                    character.y,
                    target_position.x,
                    target_position.y,
                );
                let path = Movement::from_path(path, tick);
                let current_position = Position {
                    x: character.x,
                    y: character.y,
                    dir: 0,
                };
                debug!(
                    "Too far from target, moving from {} toward it: {}",
                    current_position, target_position
                );
                self.movement_task_queue
                    .add_to_first_index(GameEvent::CharacterMove(CharacterMovement {
                        char_id: character.char_id,
                        start_at: tick,
                        destination: target_position,
                        current_position,
                        path,
                        cancel_attack: false,
                    }));
            } else if is_in_range {
                if character.is_moving() {
                    self.character_service.cancel_movement(character, tick);
                }
                let snapshot = server_state
                    .map_item_snapshot(map_item.id(), character.current_map_name(), character.current_map_instance())
                    .unwrap();
                let mut maybe_damage = None;
                if matches!(*map_item.object_type(), MapItemType::Mob | MapItemType::Character | MapItemType::Homunculus | MapItemType::Mercenary | MapItemType::SkillUnit) {
                    let source = self.get_status_snapshot(&character.status, tick);
                    let attack_motion = self.status_service.attack_motion(&source) as u128;
                    if tick < character.attack().last_attack_tick.saturating_add(attack_motion) || tick < character.timing.get_canact_tick() { return; }
                    if ScriptSkillService::attack_blocked_by_combo(character, tick) { return; }
                    if !self.script_skill_service.admit_normal_attack(server, character, tick) { character.clear_attack(); return; }
                    if self.script_skill_service.try_triple_attack(server, server_state, character, map_item.id(), tick) {
                        character.update_last_attack_tick(tick);
                        return;
                    }
                    let Some(target_status) = self.get_target_status(server_state, character, Some(map_item.id()), tick) else { character.clear_attack(); return; };
                    maybe_damage = self.battle_service.basic_attack(
                        character,
                        snapshot,
                        &self.get_status_snapshot(&character.status, tick),
                        &target_status,
                        tick,
                    );
                }
                if let Some(damage) = maybe_damage {
                    if damage.landed {
                        self.script_skill_service.try_stance_combo(character, map_item.id(), tick);
                    }
                    if self.configuration_service.config().game.pet_support.attack_support && character.game_systems.pet.as_ref().is_some_and(|pet|!pet.incubating&&pet.intimacy>0) {
                        server.add_to_next_tick(GameEvent::ScriptWorld(crate::server::model::events::game_event::ScriptWorld {
                            char_id:character.char_id,request:crate::server::model::game_systems::ScriptWorldRequest::Pet(crate::server::model::game_systems::PetRequest::PetCombatTarget {target_id:damage.target_id,retaliation:false}),
                        }));
                    }
                    if damage.skill_id == models::enums::skill_enums::SkillEnum::NpcMagicalattack.id() {
                        let delay = damage.attacked_at.saturating_sub(tick);
                        if *map_item.object_type() == MapItemType::Mob {
                            map_instance.add_to_delayed_tick(MapEvent::MobDamage(MobDamage { damage }), delay);
                        } else {
                            server.add_to_delayed_tick(GameEvent::ScriptMapDamage(crate::server::model::events::game_event::ScriptMapDamage {
                                map: character.map_instance_key.clone(), damage,
                            }), delay);
                        }
                    } else {
                        self.apply_damage(*map_item.object_type(), map_instance, damage);
                    }
                }
            }
        } else {
            character.clear_attack();
        }
    }

    pub(crate) fn character_drop_item(&self,server:&Server,state:&mut ServerState,drop:crate::server::model::events::game_event::CharacterRemoveItem)->Result<bool,String> {
        use models::enums::item::ItemTradeFlag;
        use models::enums::EnumWithMaskValueU64;
        let Some(mut character)=state.characters_mut().remove(&drop.char_id) else {return Ok(false);};
        let result=(|| {
            let flags=state.map_flags(&character.map_instance_key);
            let Some(item)=character.get_item_from_inventory(drop.index) else {return Err("Drop item is unavailable".into());};
            if !state.has_permission(character.account_id,crate::server::model::permission_groups::Permission::Trade)
                ||character.status.hp==0||character.is_dead()||drop.amount<=0||item.equip!=0||item.amount<drop.amount
                ||flags.enabled(crate::server::model::map_flags::MapFlag::NoDrop)
                ||character.game_systems.is_trading()||character.timing.skill_menu_blocked()||character.game_systems.buying_store.is_some()||character.game_systems.vending_store.is_some()
                ||self.configuration_service.get_item(item.item_id).trade_flags as u64&ItemTradeFlag::NoDrop.as_flag()!=0 {
                return Ok(false);
            }
            let instance=state.get_map_instance_from_character(&character).ok_or("Drop map is unavailable")?;
            server.inventory_service().character_drop_items(server.runtime(),&mut character,
                crate::server::model::events::game_event::CharacterRemoveItems {char_id:drop.char_id,sell:false,items:vec![drop.clone()],notify_client:true},&instance)?;
            Ok(true)
        })();
        if !result.as_ref().is_ok_and(|success|*success) {server.item_service().notify_removed(character.char_id,drop.index,0);}
        state.insert_character(character);
        result
    }

    pub fn character_remove_expired_bonuses(&self, character: &mut Character, tick: u128) {
        let should_reload_client_side_status = RefCell::new(false);
        if !character.status.temporary_bonuses.is_empty() {
            let icons = RefCell::new(HashSet::new());
            character
                .status
                .temporary_bonuses
                .retain(|temporary_bonus| match temporary_bonus.expirency() {
                    BonusExpiry::Never => true,
                    BonusExpiry::Time(until) => {
                        let expired = tick >= *until;
                        if expired {
                            *should_reload_client_side_status.borrow_mut() = true;
                            if temporary_bonus.has_icon() {
                                let icon = temporary_bonus.icon().unwrap();
                                if icons.borrow().contains(&icon) {
                                    return false;
                                }
                                icons.borrow_mut().insert(icon);
                                let mut packet_zc_msg_state_change = PacketZcMsgStateChange::new(self.configuration_service.packetver());
                                packet_zc_msg_state_change.set_state(false);
                                packet_zc_msg_state_change.set_aid(character.char_id);
                                packet_zc_msg_state_change.set_index(temporary_bonus.icon().unwrap() as i16);
                                packet_zc_msg_state_change.fill_raw();
                                self.client_notification_sender
                                    .send(Notification::Char(CharNotification::new(
                                        character.char_id,
                                        mem::take(packet_zc_msg_state_change.raw_mut()),
                                    )))
                                    .unwrap_or_else(|_| error!("Failed to send notification packet_zc_msg_state_change to client"));
                            }
                        }
                        !expired
                    }
                    // TODO later
                    BonusExpiry::Counter(_) => true,
                })
        }
        if *should_reload_client_side_status.borrow() {
            self.server_task_queue
                .add_to_first_index(GameEvent::CharacterUpdateClientSideStats(CharacterUpdateClientSideStats { char_id: character.char_id }))
        }
    }

    fn apply_damage(&self, map_item_type: MapItemType, map_instance: &Arc<MapInstance>, damage: Damage) {
        if map_item_type == MapItemType::SkillUnit {
            self.server_task_queue.add_to_first_index(GameEvent::ScriptMapDamage(crate::server::model::events::game_event::ScriptMapDamage { map: map_instance.key().clone(), damage }));
        } else if matches!(map_item_type, MapItemType::Mob) {
            map_instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage }));
        } else if matches!(map_item_type, MapItemType::Character | MapItemType::Homunculus | MapItemType::Mercenary) {
            self.server_task_queue.add_to_first_index(GameEvent::CharacterDamage(CharacterDamage { damage }));
        }
    }

    pub fn get_status_snapshot(&self, status: &Status, _tick: u128) -> StatusSnapshot {
        self.status_service.to_snapshot(status)
    }

    pub fn character_start_use_skill(
        &self,
        server: &Server,
        server_state: &ServerState,
        character: &mut Character,
        character_use_skill: CharacterUseSkill,
        tick: u128,
    ) {
        let combo_ready = self.script_skill_service.combo_ready(character, character_use_skill.skill_id, tick);
        if combo_ready && character.is_using_skill() && character.skill_has_been_used() {
            character.clear_skill_in_use();
        }
        if character.status.hp == 0 || character.is_using_skill() || character.status.blocks_casting() || (character.timing.get_canact_tick() > tick && !combo_ready) {
            return;
        }
        if !server.player_skill_target_allowed(server_state, character, character_use_skill.target_id, character_use_skill.skill_id, false) { return; }
        let target = Self::get_target(server_state, character, Some(character_use_skill.target_id));
        if target.is_none() {
            return;
        }
        let target_snapshot = target.as_ref().unwrap();

        let skill_enum = SkillEnum::from_id(character_use_skill.skill_id);
        let skill = skills::skill_enums::to_object(skill_enum, character_use_skill.skill_level);
        if skill.is_none() {
            return;
        }
        let _skill = skill.unwrap();
        let effective_range = self.script_skill_service.player_skill_range(&self.get_status_snapshot(&character.status, tick), character_use_skill.skill_id, character_use_skill.skill_level);

        let is_in_range = character.x.abs_diff(target_snapshot.position.x).max(character.y.abs_diff(target_snapshot.position.y)) <= effective_range.max(1);

        if !is_in_range {
            if character.is_moving() {
                self.character_service.cancel_movement(character, tick);
            }
            character.set_pending_skill(
                character_use_skill.target_id,
                character_use_skill.skill_id,
                character_use_skill.skill_level,
            );
            let maybe_map_instance = server_state.get_map_instance(character.current_map_name(), character.current_map_instance());
            let map_instance = maybe_map_instance.as_ref().unwrap();
            let path = path_search_client_side_algorithm(
                map_instance.x_size(),
                map_instance.y_size(),
                map_instance.state().cells(),
                character.x,
                character.y,
                target_snapshot.position.x,
                target_snapshot.position.y,
            );
            let path = Movement::from_path(path, tick);
            let current_position = Position {
                x: character.x,
                y: character.y,
                dir: 0,
            };
            self.movement_task_queue
                .add_to_first_index(GameEvent::CharacterMove(CharacterMovement {
                    char_id: character.char_id,
                    start_at: tick,
                    destination: target_snapshot.position,
                    current_position,
                    path,
                    cancel_attack: false,
                }));
            return;
        }

        if character.is_moving() {
            self.character_service.cancel_movement(character, tick);
        }
        if let Err(error) = self.script_skill_service.validate_native_environment(server_state, character, character_use_skill.skill_id, character_use_skill.skill_level, tick) { warn!("Skill environment failed: {}", error); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
        if self.script_skill_service.validate_damage_target(server_state, character, character_use_skill.skill_id, character_use_skill.target_id).is_err() { self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
        let requirements = match self.script_skill_service.requirements_plan(character, character_use_skill.skill_id, character_use_skill.skill_level, tick) {
            Ok(plan) => plan,
            Err(error) => { warn!("Skill requirements failed: {}", error); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
        };
        character.script_skill_state.native_requirements = Some(crate::server::script::skill::requirements::DeferredSkillPayment { skill_id: character_use_skill.skill_id, level: character_use_skill.skill_level, keep_requirements: true, requirements, source_index: None, source_item: None });
        self.script_skill_service.end_cloaking_on_skill(server, character, character_use_skill.skill_id, tick);
        let skill_use_response = self.skill_service.start_item_skill(
            character,
            target,
            &self.get_status_snapshot(&character.status, tick),
            self.get_target_status(server_state, character, Some(character_use_skill.target_id), tick)
                .as_ref(),
            character_use_skill.skill_id,
            character_use_skill.skill_level,
            tick,
            false,
        );
        if !skill_use_response.is_valid() { character.script_skill_state.native_requirements = None; }
        if skill_use_response.is_valid() && skill_use_response.has_no_delay() {
            self.character_use_skill(server, server_state, tick, character);
        }
    }

    pub fn character_pending_skill(&self, server: &Server, server_state: &ServerState, tick: u128, character: &mut Character) {
        if character.status.hp == 0 { character.clear_pending_skill(); return; }
        if !character.has_pending_skill() {
            return;
        }
        let pending = *character.pending_skill();
        if !server.player_skill_target_allowed(server_state, character, pending.target_id, pending.skill_id, false) { character.clear_pending_skill(); return; }
        let target = Self::get_target(server_state, character, Some(pending.target_id));
        if target.is_none() {
            character.clear_pending_skill();
            return;
        }
        let target_snapshot = target.as_ref().unwrap();

        let skill_enum = SkillEnum::from_id(pending.skill_id);
        let skill = skills::skill_enums::to_object(skill_enum, pending.skill_level);
        if skill.is_none() {
            character.clear_pending_skill();
            return;
        }
        let _skill = skill.unwrap();
        let effective_range = self.script_skill_service.player_skill_range(&self.get_status_snapshot(&character.status, tick), pending.skill_id, pending.skill_level);

        let is_in_range = character.x.abs_diff(target_snapshot.position.x).max(character.y.abs_diff(target_snapshot.position.y)) <= effective_range.max(1);

        if is_in_range {
            character.clear_pending_skill();
            if character.is_moving() {
                self.character_service.cancel_movement(character, tick);
            }
            if let Err(error) = self.script_skill_service.validate_native_environment(server_state, character, pending.skill_id, pending.skill_level, tick) { warn!("Pending skill environment failed: {}", error); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
            if self.script_skill_service.validate_damage_target(server_state, character, pending.skill_id, pending.target_id).is_err() { self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
            let requirements = match self.script_skill_service.requirements_plan(character, pending.skill_id, pending.skill_level, tick) {
                Ok(plan) => plan,
                Err(error) => { warn!("Pending skill requirements failed: {}", error); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
            };
            character.script_skill_state.native_requirements = Some(crate::server::script::skill::requirements::DeferredSkillPayment { skill_id: pending.skill_id, level: pending.skill_level, keep_requirements: true, requirements, source_index: None, source_item: None });
            self.script_skill_service.end_cloaking_on_skill(server, character, pending.skill_id, tick);
            let skill_use_response = self.skill_service.start_item_skill(
                character,
                target,
                &self.get_status_snapshot(&character.status, tick),
                self.get_target_status(server_state, character, Some(pending.target_id), tick)
                    .as_ref(),
                pending.skill_id,
                pending.skill_level,
                tick,
                false,
            );
            if !skill_use_response.is_valid() { character.script_skill_state.native_requirements = None; }
            if skill_use_response.is_valid() && skill_use_response.has_no_delay() {
                self.character_use_skill(server, server_state, tick, character);
            }
        }
    }

    pub fn character_use_skill(&self, server: &Server, server_state: &ServerState, tick: u128, character: &mut Character) {
        if !character.is_using_skill() {
            return;
        }
        if character.skill_has_been_used() {
            self.skill_service.after_skill_used(character, tick);
        } else {
            if !self.skill_service.cast_is_ready(character, tick) { return; }
            if let Some(target_id) = character.skill_in_use().target { if !server.player_skill_target_allowed(server_state, character, target_id, character.skill_in_use().skill.id(), true) { character.clear_skill_in_use(); self.script_skill_service.cancel_queued_cast(character); return; } }
            let target = Self::get_target(server_state, character, character.skill_in_use().target);
            let target_status = self.get_target_status(server_state, character, character.skill_in_use().target, tick);
            if target.is_none() || target_status.as_ref().is_none_or(|target| target.hp() == 0) || character.status.hp == 0 {
                character.clear_skill_in_use(); self.script_skill_service.cancel_queued_cast(character); return;
            }
            if character.skill_in_use().target.is_some_and(|target| self.script_skill_service.validate_damage_target(server_state, character, character.skill_in_use().skill.id(), target).is_err()) { character.clear_skill_in_use(); self.script_skill_service.cancel_queued_cast(character); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
            if let Some(payment) = character.script_skill_state.native_requirements.take() {
                if payment.requirements.sp > 0 && self.script_skill_service.validate_native_environment(server_state, character, payment.skill_id, payment.level, tick).is_err() { character.clear_skill_in_use(); self.script_skill_service.cancel_queued_cast(character); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
                let identity_valid = payment.source_index.zip(payment.source_item).is_none_or(|(index, identity)| character.get_item_from_inventory(index).is_some_and(|item| (item.id, item.item_id, item.unique_id) == identity));
                let result = if identity_valid { self.item_service.pay_requirement_plan_in_state(server, server_state, character, &payment.requirements, payment.source_index, tick) } else { Err("Delayed item source changed during casting".into()) };
                if let Err(error) = result { warn!("Skill completion payment failed: {}", error); character.clear_skill_in_use(); self.script_skill_service.cancel_queued_cast(character); self.skill_service.send_skill_fail_packet(character, models::enums::skill::UseSkillFailure::Fail); return; }
            }
            let skill_use_response = self.skill_service.do_use_skill(
                character,
                target,
                &self.get_status_snapshot(&character.status, tick),
                target_status.as_ref(),
                tick,
            );
            if let Some(skill_use_response) = skill_use_response {
                let after_cast_delay = character.skill_in_use().skill.after_cast_act_delay();
                self.script_skill_service.advance_combo(character, skill_use_response.skill_id, skill_use_response.target_id, after_cast_delay, tick);
                if skill_use_response.skill_type == SkillType::Offensive {
                    let maybe_map_instance = server_state.get_map_instance(character.current_map_name(), character.current_map_instance());
                    let map_instance = maybe_map_instance.as_ref().unwrap();
                    match self.script_skill_service.complete_damage_skill(server, server_state, character, skill_use_response.to_damage(), &self.battle_service, tick) {
                        Ok(damages) => for (kind, damage) in ScriptSkillService::double_cast(character, skill_use_response.skill_id, damages, fastrand::u8(0..100)) {
                            if kind == MapItemType::SkillUnit { self.apply_damage(kind, map_instance, damage); }
                            else if kind == MapItemType::Mob { map_instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage })); }
                            else { self.server_task_queue.add_to_first_index(GameEvent::CharacterDamage(CharacterDamage { damage })); }
                        },
                        Err(error) => warn!("Unable to complete area damage for skill {}: {}", skill_use_response.skill_id, error),
                    }
                }
                self.server_task_queue.add_to_first_index(GameEvent::ScriptCombat(crate::server::service::script_combat_service::ScriptCombatRequest {
                    source_id: character.char_id, target_id: skill_use_response.target_id, trigger: models::status_bonus::CombatTrigger::Skill,
                    battle_flags: skill_use_response.battle_flags, skill_id: skill_use_response.skill_id, damage: skill_use_response.damage_to_target.max(0) as u32,
                    other_mob_id: None, depth: skill_use_response.proc_depth, drop_position: None, origin_map: Some(character.map_instance_key.clone()), right_hand_damage: None,
                }));
                if !skill_use_response.bonuses.is_empty() {
                    character.status.temporary_bonuses.merge(skill_use_response.bonuses);
                    self.server_task_queue
                        .add_to_first_index(GameEvent::CharacterUpdateClientSideStats(CharacterUpdateClientSideStats { char_id: character.char_id }));
                }
            }
        }
    }

    // TODO cache per tick
    fn get_target(server_state: &ServerState, character: &Character, target_id: Option<u32>) -> Option<MapItemSnapshot> {
        if let Some(target_id) = target_id {
            if target_id == character.char_id { return Some(character.to_map_item_snapshot()); }
            server_state.map_item_snapshot(target_id, character.current_map_name(), character.current_map_instance())
        } else {
            None
        }
    }

    // TODO cache per tick
    pub(crate) fn get_target_status(
        &self,
        server_state: &ServerState,
        character: &Character,
        target_id: Option<u32>,
        tick: u128,
    ) -> Option<StatusSnapshot> {
        if let Some(target_id) = target_id {
            if target_id == character.char_id { return Some(self.get_status_snapshot(&character.status, tick)); }
            let map_item = server_state.map_item(target_id, character.current_map_name(), character.current_map_instance());
            if let Some(map_item) = map_item {
                match map_item.object_type() {
                    MapItemType::Character => {
                        return Some(self.get_status_snapshot(&server_state.get_character_unsafe(target_id).status, tick));
                    }
                    MapItemType::Mob | MapItemType::Homunculus | MapItemType::Mercenary | MapItemType::SkillUnit => {
                        return Some(
                            server_state
                                .map_item_mob_status(&map_item, character.current_map_name(), character.current_map_instance())
                                .unwrap(),
                        );
                    }
                    _ => {
                        return None;
                    }
                }
            }
            None
        } else {
            None
        }
    }

    pub fn character_pickup_item(
        &self,
        server: &Server,
        server_state: &mut ServerState,
        character: &mut Character,
        map_item_id: u32,
        map_instance: &MapInstance,
    ) -> Result<bool, String> {
        self.pickup_item(server, server_state, character, map_item_id, map_instance, false)
    }

    /// Autoloot picks up from anywhere on the map, the other rules of picking up still apply.
    pub fn character_autoloot_item(
        &self,
        server: &Server,
        server_state: &mut ServerState,
        character: &mut Character,
        map_item_id: u32,
        map_instance: &MapInstance,
    ) -> Result<bool, String> {
        self.pickup_item(server, server_state, character, map_item_id, map_instance, true)
    }

    fn pickup_item(
        &self,
        server: &Server,
        server_state: &mut ServerState,
        character: &mut Character,
        map_item_id: u32,
        map_instance: &MapInstance,
        remote: bool,
    ) -> Result<bool, String> {
        use crate::repository::script_inventory_repository::{ScriptInventoryTransaction, ScriptItemGrant};
        use super::script_world_service::{party_can_pick_up, party_loot_candidates};
        if server_state.contains_locked_map_item(map_item_id) || character.is_dead() || character.status.hp == 0 || character.timing.skill_menu_blocked()
            || character.map_instance_key != *map_instance.key() || !remote && !character.is_map_item_in_fov(map_item_id) {
            return Ok(false);
        }
        let Some(dropped_item) = map_instance.state().get_dropped_item(map_item_id).copied() else { return Ok(false); };
        if !remote && character.x.abs_diff(dropped_item.x()).max(character.y.abs_diff(dropped_item.y())) > 2 {
            return Ok(false);
        }
        let config = &self.configuration_service.config().game;
        let lock_seconds = if dropped_item.player_dropped { config.player_dropped_item_locked_to_owner_duration_in_secs }
            else { config.mob_dropped_item_locked_to_owner_duration_in_secs };
        if dropped_item.owner_id.is_some_and(|owner| !party_can_pick_up(character, owner)
            && get_tick().saturating_sub(dropped_item.dropped_at) < u128::from(lock_seconds) * 1000) {
            return Ok(false);
        }
        let amount = i16::try_from(dropped_item.amount).map_err(|_| "Floor item amount is out of bounds")?;
        let transfer = |recipient: &mut Character| -> Result<(), String> {
            let result = server.repository.script_inventory_transaction(&ScriptInventoryTransaction {
                char_id: recipient.char_id, account_id: recipient.account_id,
                consumption: None, exact_removals: vec![], removals: vec![], identifications: vec![],
                grants: vec![ScriptItemGrant {
                    item_id: dropped_item.item_id, amount, identified: dropped_item.is_identified,
                    refine: dropped_item.attributes.refine, cards: dropped_item.attributes.cards,
                    unique_id: Some(dropped_item.attributes.unique_id), damaged: dropped_item.attributes.damaged,
                }],
                variables: vec![], zeny: None, hp: None, sp: None,
                max_weight: server.character_service().max_weight(recipient), max_slots: usize::from(config.max_inventory),
                world: None, reset_skills: None, fame: None, character_changes: vec![], pool_draws: vec![],
            }).map_err(|error| error.to_string())?;
            server.item_service().install_inventory(server, recipient, result.inventory, None);
            Ok(())
        };
        let mut candidates = party_loot_candidates(server_state, character);
        fastrand::shuffle(&mut candidates);
        let mut committed = false;
        let mut last_error = None;
        for id in candidates {
            let result = if id == character.char_id {
                transfer(character)
            } else if let Some(mut recipient) = server_state.characters_mut().remove(&id) {
                let result = transfer(&mut recipient);
                server_state.insert_character(recipient);
                result
            } else { continue; };
            match result {
                Ok(()) => { committed = true; break; }
                Err(error) => { last_error = Some(error); }
            }
        }
        if !committed { return Err(last_error.unwrap_or_else(|| "No party member could receive the floor item".into())); }
        server_state.insert_locked_map_item(map_item_id);
        let mut packet = PacketZcNotifyAct::new(self.configuration_service.packetver());
        packet.set_gid(character.char_id);
        packet.set_action(ActionType::Itempickup.value() as u8);
        packet.fill_raw();
        self.client_notification_sender.send(Notification::Area(AreaNotification::new(
            map_instance.key().map_name().clone(), map_instance.key().map_instance(),
            AreaNotificationRangeType::Fov { x: dropped_item.x(), y: dropped_item.y(), exclude_id: None }, packet.raw,
        ))).unwrap_or_else(|_| error!("Failed to send pickup animation"));
        map_instance.add_to_next_tick(MapEvent::RemoveDroppedItemFromMap(RemoveDroppedItemFromMap { dropped_item_id: map_item_id }));
        Ok(true)
    }
}
