use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Weak};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use models::enums::skill_enums::SkillEnum;

use super::actor::ScriptSkillActor;
use skills::GroundKind;

use super::ground::{GroundCell, GroundSkill, GroundSkillSource, NEXT_GROUND_UNIT};
use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptSkillCast, ScriptTeleportSelection};
use crate::server::model::game_systems::MemoPoint;
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::MapItemType;
use crate::server::model::script::Script;
use crate::server::model::session::SessionBinding;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Clone, Debug, PartialEq)]
pub struct WarpPortalMenuCast {
    pub map: MapInstanceKey,
    pub x: u16,
    pub y: u16,
    pub keep_requirements: bool,
    pub issued_skill: bool,
    pub session: Option<SessionBinding>,
}

#[derive(Clone, Debug)]
pub struct PendingWarpPortalMenu {
    pub cast: WarpPortalMenuCast,
    pub level: u8,
    pub destinations: Vec<MemoPoint>,
    pub expires_at: u128,
}

pub(super) struct WarpPortalState {
    pub destination: MapInstanceKey,
    pub x: u16,
    pub y: u16,
    pub opens_at: u128,
    pub ready: bool,
    pub blocked: bool,
    pub caster_session: Option<SessionBinding>,
    pub caster_npc: Option<Weak<Script>>,
    pub in_flight: HashMap<u32, Option<SessionBinding>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WarpPortalEntry {
    pub char_id: u32,
    pub unit_id: u32,
    pub map: MapInstanceKey,
    pub session: Option<SessionBinding>,
}

pub(crate) fn clear_menu(character: &mut Character, tick: u128) {
    if let Some(menu) = character.script_skill_state.pending_warp_portal.take() {
        character.timing.set_skill_menu_blocked(false);
        if character.timing.get_canmove_tick() == menu.expires_at {
            character.timing.set_canmove_tick(tick);
        }
        if character.timing.get_canact_tick() == menu.expires_at {
            character.timing.set_canact_tick(tick);
        }
    }
}

impl ScriptSkillService {
    pub fn cancel_warp_portal_menu(&self, character: &mut Character, tick: u128) {
        clear_menu(character, tick);
    }

    fn portal_session(state: &ServerState, character: &Character) -> Option<SessionBinding> {
        state
            .find_session(character.account_id)
            .map(|session| SessionBinding::new(&session))
    }

    fn portal_session_current(state: &ServerState, character: &Character, binding: &Option<SessionBinding>) -> bool {
        binding.as_ref().is_none_or(|binding| {
            state
                .find_session(character.account_id)
                .is_some_and(|session| session.char_id == Some(character.char_id) && binding.matches(&session))
        })
    }

    pub(super) fn validate_portal_cast(
        &self,
        state: &ServerState,
        character: &Character,
        cast: &WarpPortalMenuCast,
        level: u8,
    ) -> Result<(), String> {
        if !(1..=4).contains(&level) || character.status.hp == 0 || character.status.blocks_casting() {
            return Err("Warp Portal caster or skill level is unavailable".into());
        }
        if character.map_instance_key != cast.map || !Self::portal_session_current(state, character, &cast.session) {
            return Err("Warp Portal cast belongs to another map or login".into());
        }
        Self::validate_stealth_cast(state, character, SkillEnum::AlWarp.id())?;
        Self::validate_skill_map(state, character, SkillEnum::AlWarp.id(), level, cast.issued_skill)?;
        self.validate_portal_terrain(state, &cast.map, cast.x, cast.y)
    }

    fn validate_portal_terrain(&self, state: &ServerState, map: &MapInstanceKey, x: u16, y: u16) -> Result<(), String> {
        let instance = state
            .get_map_instance(map.map_name(), map.map_instance())
            .ok_or("Warp Portal map is unavailable")?;
        if x >= instance.x_size()
            || y >= instance.y_size()
            || instance
                .state()
                .cells()
                .get(usize::from(y) * usize::from(instance.x_size()) + usize::from(x))
                .is_none_or(|cell| cell & CellType::Walkable.as_flag() == 0 || cell & CellType::Shootable.as_flag() == 0)
        {
            return Err("Warp Portal cell is not usable terrain".into());
        }
        Ok(())
    }

    fn validate_portal_placement(
        &self,
        state: &ServerState,
        source: &GroundSkillSource,
        x: u16,
        y: u16,
        tick: u128,
        player: bool,
    ) -> Result<(), String> {
        let map = MapInstanceKey::new(source.map.clone(), source.instance);
        self.validate_portal_terrain(state, &map, x, y)?;
        let metadata = SkillMetadata::find(SkillEnum::AlWarp.id()).ok_or("Warp Portal unit metadata is missing")?;
        self.validate_classic_unit_placement(state, source, metadata, 1, x, y, tick)?;
        let grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        if player
            && grounds
                .iter()
                .filter(|ground| {
                    ground.source_id == source.actor_id
                        && ground.kind == GroundKind::WarpPortal
                        && ground.expires_at > tick
                        && ground.cells.iter().any(|cell| cell.remaining_hits > 0)
                })
                .count()
                >= 3
        {
            return Err("At most three Warp Portals may be active".into());
        }
        if grounds.iter().any(|ground| {
            ground.map == source.map
                && ground.instance == source.instance
                && ground.expires_at > tick
                && matches!(ground.kind, GroundKind::WarpPortal | GroundKind::LandProtector)
                && ground.covers(x, y)
        }) {
            return Err("Another portal or Land Protector covers this cell".into());
        }
        Ok(())
    }

    pub(super) fn start_warp_portal_menu(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
        instant: bool,
        depth: u8,
        cast_time: Option<u128>,
    ) -> Result<(), String> {
        if character.script_skill_state.pending_warp_portal.is_some() {
            return Err("A Warp Portal destination is already awaiting selection".into());
        }
        if character
            .script_skill_state
            .deferred_requirements
            .as_ref()
            .is_some_and(|payment| payment.skill_id != skill_id || payment.level != level)
        {
            return Err("Another skill payment is awaiting completion".into());
        }
        let mut payment = character
            .script_skill_state
            .deferred_requirements
            .take()
            .filter(|payment| payment.skill_id == skill_id && payment.level == level);
        let keep_requirements = payment.as_ref().is_some_and(|payment| payment.keep_requirements);
        if let Some(payment) = &mut payment {
            payment.requirements.removals.clear();
        }
        character.script_skill_state.deferred_requirements = payment;
        let cast = WarpPortalMenuCast {
            map: character.map_instance_key.clone(),
            x,
            y,
            keep_requirements,
            issued_skill: instant
                || character
                    .pending_item_skill
                    .as_ref()
                    .is_some_and(|pending| pending.source_item.is_some()),
            session: Self::portal_session(state, character),
        };
        self.validate_portal_cast(state, character, &cast, level)?;
        let effect = ScriptSkillEffect {
            source_char_id: character.char_id,
            target_id: character.char_id,
            skill_id,
            level,
            heal_value: 0,
            proc_depth: depth,
            skill_event_emitted: instant,
            cast_generation: 0,
            action: ScriptSkillAction::OpenWarpPortalMenu(cast),
            deferred_requirements: None,
            prepared_outcome: None,
            source_index: None,
            source_item: None,
        };
        if instant {
            return self.apply_target_effect(server, state, character, &effect, tick);
        }
        let skill = self
            .configuration
            .find_skill_config(&(skill_id as i32).into())
            .ok_or("Warp Portal skill is unavailable")?;
        character.clear_attack();
        character.movements.clear();
        self.queue_target_effect_with_cast_time(server, character, skill, effect, tick, cast_time);
        Ok(())
    }

    pub(super) fn notify_portal_cast(&self, character: &Character, effect: &ScriptSkillEffect, cast: &WarpPortalMenuCast, duration: u128) {
        let source = ScriptSkillActor {
            id: character.char_id,
            credit_id: character.char_id,
            object_type: MapItemType::Character,
            map: character.current_map_name().clone(),
            instance: character.current_map_instance(),
            x: character.x,
            y: character.y,
            dir: character.dir,
            status: StatusService::instance().to_snapshot(&character.status),
            raw_attack: 0,
            mode: 0,
            attack_motion: 0,
        };
        let request = ScriptSkillCast {
            source_id: character.char_id,
            source_map: Some(source.map.clone()),
            source_instance: Some(source.instance),
            target_id: character.char_id,
            skill_id: effect.skill_id,
            level: u16::from(effect.level),
            ground: Some((cast.x, cast.y)),
            cast_time_adjust_ms: 0,
            cast_cancel: None,
            message_id: None,
            ignore_range: false,
        };
        self.notify_area(character, super::actor::casting_packet(&source, &request, duration));
    }

    pub(super) fn open_warp_portal_menu(
        &self,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        cast: &WarpPortalMenuCast,
        tick: u128,
    ) -> Result<(), String> {
        self.validate_portal_cast(state, character, cast, effect.level)?;
        let mut destinations = vec![MemoPoint {
            map: Map::name_without_ext(&character.save_map),
            x: character.save_x,
            y: character.save_y,
        }];
        destinations.extend(
            character
                .game_systems
                .memo_points
                .iter()
                .take(usize::from(effect.level.saturating_sub(1)))
                .flatten()
                .cloned(),
        );
        let maps = destinations.iter().map(|point| point.map.clone()).collect::<Vec<_>>();
        let expires_at = tick.saturating_add(120_000);
        character.script_skill_state.pending_teleport = None;
        character.script_skill_state.cast_cancel_override = None;
        character.script_skill_state.pending_warp_portal = Some(PendingWarpPortalMenu {
            cast: cast.clone(),
            level: effect.level,
            destinations,
            expires_at,
        });
        character.timing.set_canmove_tick(expires_at);
        character.timing.set_canact_tick(expires_at);
        character.timing.set_skill_menu_blocked(true);
        self.notify_support_skill(character, effect);
        self.queue_notification(Notification::Char(CharNotification::new(
            character.char_id,
            Self::teleport_menu_packet(self.configuration.packetver(), effect.skill_id, &maps),
        )));
        Ok(())
    }

    pub fn finish_warp_portal_menu(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        selection: &ScriptTeleportSelection,
        tick: u128,
    ) -> Result<(), String> {
        let menu = character
            .script_skill_state
            .pending_warp_portal
            .as_ref()
            .ok_or("No Warp Portal menu is awaiting a choice")?
            .clone();
        if selection.skill_id != SkillEnum::AlWarp.id()
            || selection.char_id != character.char_id
            || selection.session != menu.cast.session
            || !Self::portal_session_current(state, character, &menu.cast.session)
        {
            return Err("Warp Portal choice belongs to another cast or login".into());
        }
        clear_menu(character, tick);
        if Map::name_without_ext(&selection.map) == "cancel" {
            return Ok(());
        }
        if tick >= menu.expires_at {
            return Err("Warp Portal destination selection expired".into());
        }
        self.validate_portal_cast(state, character, &menu.cast, menu.level)?;
        let destination = menu
            .destinations
            .iter()
            .find(|point| Map::name_without_ext(&point.map) == Map::name_without_ext(&selection.map))
            .ok_or("Warp Portal destination was not offered")?;
        let destination_map = self
            .configuration
            .find_map(&Map::name_without_ext(&destination.map))
            .ok_or("Warp Portal destination map is unavailable")?;
        if destination.x >= destination_map.x_size() || destination.y >= destination_map.y_size() {
            return Err("Warp Portal destination is outside the map".into());
        }
        let source = GroundSkillSource {
            actor_id: character.char_id,
            owner_id: character.char_id,
            map: menu.cast.map.map_name().clone(),
            instance: menu.cast.map.map_instance(),
            x: character.x,
            y: character.y,
            status: StatusService::instance().to_snapshot(&character.status),
            raw_attack: 0,
            fixed_damage: None,
        };
        self.validate_portal_placement(state, &source, menu.cast.x, menu.cast.y, tick, true)?;
        let ground = Self::new_warp_portal(
            source,
            menu.cast.x,
            menu.cast.y,
            destination,
            0,
            menu.level,
            menu.cast.session.clone(),
            tick,
        );
        if menu.cast.keep_requirements {
            let requirements = self.item_requirements_plan(character, SkillEnum::AlWarp.id(), menu.level, tick)?;
            server
                .item_service()
                .pay_requirement_plan_in_state(server, state, character, &requirements, None, tick)?;
        }
        self.ground_skills
            .lock()
            .map_err(|_| "Ground skill state is unavailable")?
            .push(ground);
        Ok(())
    }

    pub fn create_npc_warp_portal(
        &self,
        state: &ServerState,
        source: &ScriptSkillActor,
        arguments: &[script_sdk::Value],
        tick: u128,
    ) -> Result<script_sdk::Value, String> {
        if source.object_type != MapItemType::Npc || arguments.len() != 5 {
            return Err("Warp Portal command requires an NPC and five arguments".into());
        }
        let coordinate = |index: usize| {
            u16::try_from(arguments[index].number_value()?).map_err(|_| "Warp Portal coordinate is out of bounds".to_string())
        };
        let (x, y, destination_x, destination_y) = (coordinate(0)?, coordinate(1)?, coordinate(3)?, coordinate(4)?);
        let destination_name = Map::name_without_ext(arguments[2].string_value()?);
        let destination = self
            .configuration
            .find_map(&destination_name)
            .ok_or("Warp Portal destination map is unavailable")?;
        if destination_x >= destination.x_size() || destination_y >= destination.y_size() {
            return Err("Warp Portal destination is outside the map".into());
        }
        let source_instance = state
            .get_map_instance(&source.map, source.instance)
            .ok_or("Warp Portal source map is unavailable")?;
        let script = source_instance.get_script(source.id).ok_or("Warp Portal NPC was removed")?;
        let ground_source = GroundSkillSource {
            actor_id: source.id,
            owner_id: source.id,
            map: source.map.clone(),
            instance: source.instance,
            x: source.x,
            y: source.y,
            status: source.status.clone(),
            raw_attack: 0,
            fixed_damage: None,
        };
        self.validate_portal_placement(state, &ground_source, x, y, tick, false)?;
        let destination_instance = if destination_name == Map::name_without_ext(&source.map) {
            source.instance
        } else {
            0
        };
        let mut ground = Self::new_warp_portal(
            ground_source,
            x,
            y,
            &MemoPoint {
                map: destination_name,
                x: destination_x,
                y: destination_y,
            },
            destination_instance,
            4,
            None,
            tick,
        );
        ground.portal.as_mut().unwrap().caster_npc = Some(Arc::downgrade(&script));
        self.ground_skills
            .lock()
            .map_err(|_| "Ground skill state is unavailable")?
            .push(ground);
        Ok(script_sdk::Value::default())
    }

    fn new_warp_portal(
        source: GroundSkillSource,
        x: u16,
        y: u16,
        destination: &MemoPoint,
        destination_instance: u8,
        level: u8,
        caster_session: Option<SessionBinding>,
        tick: u128,
    ) -> GroundSkill {
        let actor_source = (*source.status.combat_actor_kind() == models::enums::actor::CombatActorKind::Npc).then(|| source.clone());
        let duration = SkillMetadata::find(SkillEnum::AlWarp.id())
            .and_then(|metadata| metadata.duration(level, false))
            .unwrap_or(5000)
            .max(2000) as u128;
        GroundSkill {
            portal: Some(WarpPortalState {
                destination: MapInstanceKey::new(destination.map.clone(), destination_instance),
                x: destination.x,
                y: destination.y,
                opens_at: tick + 2000,
                ready: false,
                blocked: false,
                caster_session,
                caster_npc: None,
                in_flight: HashMap::new(),
            }),
            capture: None,
            recovery_item: None,
            kind: GroundKind::WarpPortal,
            message: vec![],
            source_id: source.actor_id,
            source_x: source.x,
            source_y: source.y,
            map: source.map.clone(),
            instance: source.instance,
            skill_id: SkillEnum::AlWarp.id(),
            level,
            depth: 0,
            active_from: tick,
            expires_at: tick + duration,
            next_hit_at: tick,
            interval: 40,
            effect_range: 0,
            displayed: false,
            skill_event_emitted: true,
            cast_generation: 0,
            cast_finish_at: tick,
            cast_verified: true,
            cells: vec![GroundCell {
                observers: HashMap::new(),
                id: NEXT_GROUND_UNIT.fetch_add(1, Ordering::Relaxed),
                x,
                y,
                remaining_hits: u16::from(level) + 6,
            }],
            affected: HashSet::new(),
            actor_source,
            triggered: false,
            waves: 0,
        }
    }

    pub(super) fn tick_warp_portal(&self, server: &Server, state: &ServerState, ground: &mut GroundSkill, tick: u128) {
        let Some(portal) = ground.portal.as_mut() else {
            ground.expires_at = 0;
            return;
        };
        if ground.expires_at <= tick {
            return;
        }
        let changed = !portal.ready && tick >= portal.opens_at;
        if changed {
            portal.ready = true;
        }
        let cell = &ground.cells[0];
        if portal.ready && !portal.blocked && cell.remaining_hits > 0 {
            portal.in_flight.retain(|id, binding| {
                state.get_character(*id).is_some_and(|character| {
                    character.loaded_from_client_side
                        && character.current_map_name() == &ground.map
                        && character.current_map_instance() == ground.instance
                        && character.x == cell.x
                        && character.y == cell.y
                        && Self::portal_session_current(state, character, binding)
                })
            });
            for character in state.characters().values().filter(|character| {
                character.status.hp > 0
                    && character.loaded_from_client_side
                    && character.current_map_name() == &ground.map
                    && character.current_map_instance() == ground.instance
                    && character.x == cell.x
                    && character.y == cell.y
                    && character.movements.is_empty()
                    && !state.pending_character_logouts.contains_key(&character.char_id)
            }) {
                if portal.in_flight.contains_key(&character.char_id) {
                    continue;
                }
                let session = Self::portal_session(state, character);
                portal.in_flight.insert(character.char_id, session.clone());
                server.add_to_next_tick(GameEvent::WarpPortalEnter(WarpPortalEntry {
                    char_id: character.char_id,
                    unit_id: cell.id,
                    map: MapInstanceKey::new(ground.map.clone(), ground.instance),
                    session,
                }));
            }
        }
        if changed {
            self.change_trap_view(ground, 128);
        }
    }

    pub(super) fn warp_portal_owner_current(&self, state: &ServerState, ground: &GroundSkill) -> bool {
        let Some(portal) = ground.portal.as_ref() else {
            return false;
        };
        if let Some(source) = &ground.actor_source {
            state
                .get_map_instance(&source.map, source.instance)
                .and_then(|instance| instance.get_script(source.actor_id))
                .is_some_and(|script| {
                    portal
                        .caster_npc
                        .as_ref()
                        .and_then(Weak::upgrade)
                        .is_some_and(|original| Arc::ptr_eq(&original, &script))
                })
        } else {
            state.get_character(ground.source_id).is_some_and(|source| {
                !state.pending_character_logouts.contains_key(&ground.source_id)
                    && Self::portal_session_current(state, source, &portal.caster_session)
            })
        }
    }

    pub fn enter_warp_portal(&self, server: &Server, state: &mut ServerState, entry: WarpPortalEntry, tick: u128) -> Result<(), String> {
        let mut grounds = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        let Some(ground) = grounds.iter_mut().find(|ground| {
            ground.kind == GroundKind::WarpPortal
                && ground.expires_at > tick
                && ground.map == *entry.map.map_name()
                && ground.instance == entry.map.map_instance()
                && ground.cells[0].id == entry.unit_id
        }) else {
            return Ok(());
        };
        if !self.warp_portal_owner_current(state, ground) {
            ground.expires_at = 0;
            return Ok(());
        }
        let portal = ground.portal.as_mut().ok_or("Warp Portal destination is missing")?;
        let Some(character) = state.get_character(entry.char_id) else {
            portal.in_flight.remove(&entry.char_id);
            return Ok(());
        };
        let cell = &mut ground.cells[0];
        if character.map_instance_key != entry.map
            || !character.loaded_from_client_side
            || character.status.hp == 0
            || character.x != cell.x
            || character.y != cell.y
            || !character.movements.is_empty()
            || state.pending_character_logouts.contains_key(&entry.char_id)
            || !Self::portal_session_current(state, character, &entry.session)
            || portal.in_flight.get(&entry.char_id) != Some(&entry.session)
            || !portal.ready
            || portal.blocked
            || cell.remaining_hits == 0
        {
            portal.in_flight.remove(&entry.char_id);
            return Ok(());
        }
        let destination = portal.destination.clone();
        let (x, y) = if portal.x == 0 && portal.y == 0 {
            crate::server::model::map::RANDOM_CELL
        } else {
            (portal.x, portal.y)
        };
        if self.configuration.find_map(&destination.map_without_ext()).is_none() {
            portal.in_flight.remove(&entry.char_id);
            return Err("Warp Portal destination was removed".into());
        }
        cell.remaining_hits -= 1;
        if destination == entry.map && x == cell.x && y == cell.y {
            portal.blocked = true;
        }
        drop(grounds);
        server.server_service().schedule_warp_to_walkable_cell_in_instance(
            state,
            destination.map_name(),
            x,
            y,
            entry.char_id,
            destination.map_instance(),
        );
        Ok(())
    }
}
