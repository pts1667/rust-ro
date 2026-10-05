use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use configuration::configuration::SkillConfig;
use models::enums::skill::SkillType;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use packets::packets::{Packet, PacketZcUseSkill};
use script_sdk::Value;

use crate::repository::Repository;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterAddItems, CharacterUseSkill, GameEvent, CharacterDamage};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::map_item::{MapItemType, ToMapItemSnapshot};
use crate::server::service::battle_service::BattleService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[path = "skill_actor.rs"]
pub mod actor;
#[path = "skill_actor_area.rs"]
mod actor_area;
#[path = "skill_actor_dispatch.rs"]
mod actor_dispatch;
#[path = "skill_actor_effects.rs"]
mod actor_effects;
#[path = "actor_ground_skill.rs"]
mod actor_ground;
#[path = "skill_area_damage.rs"]
mod area_damage;
#[path = "skill_autocast.rs"]
mod autocast;
#[path = "skill_callbacks.rs"]
pub mod callbacks;
#[path = "companion_skill.rs"]
pub mod companion;
#[path = "skill_delayed.rs"]
mod delayed;
#[path = "skill_devotion.rs"]
mod devotion;
#[path = "ground_skill.rs"]
mod ground;
#[path = "ground_unit_effects.rs"]
mod ground_unit_effects;
#[path = "skill_magic.rs"]
mod magic;
#[path = "skill_metadata.rs"]
pub mod metadata;
#[path = "skill_party_support.rs"]
mod party_support;
#[path = "skill_requirements.rs"]
pub mod requirements;
#[path = "skill_reveal.rs"]
mod reveal;
#[path = "skill_secondary.rs"]
mod secondary;
#[path = "skill_splasher.rs"]
mod splasher;
#[path = "skill_stealth.rs"]
mod stealth;
#[path = "skill_summon.rs"]
mod summon;
#[path = "skill_support.rs"]
mod support;
#[path = "skill_targeted.rs"]
mod targeted;
#[path = "skill_teleport.rs"]
mod teleport;
#[path = "ground_text_skill.rs"]
mod text_ground;
#[path = "ground_trap.rs"]
pub(crate) mod trap;
#[path = "skill_utility.rs"]
mod utility;
#[path = "skill_warp_portal.rs"]
mod warp_portal;
pub use ground::{FixedGroundSkillDamage, GroundSkillSource};
pub use reveal::ScriptRevealActor;
pub use targeted::{PreparedSkillOutcome, ScriptSkillCompletionPlan};
pub use teleport::PendingTeleportMenu;
pub(crate) use warp_portal::clear_menu as clear_warp_portal_menu;
pub use warp_portal::{PendingWarpPortalMenu, WarpPortalEntry, WarpPortalMenuCast};

#[derive(Clone, Debug, Default)]
pub struct ScriptSkillState {
    pub spirit_spheres: Vec<u128>,
    pub coins: u8,
    pub running: bool,
    pub run_started_at: u128,
    pub run_level: u8,
    pub casting_until: u128,
    pub cast_generation: u64,
    pub casting_skill_id: u32,
    pub casting_skill_level: u8,
    pub cast_cancel_override: Option<bool>,
    pub deferred_requirements: Option<requirements::DeferredSkillPayment>,
    pub native_requirements: Option<requirements::DeferredSkillPayment>,
    pub pending_teleport: Option<PendingTeleportMenu>,
    pub pending_warp_portal: Option<PendingWarpPortalMenu>,
    pub ground_skill_text: Vec<u8>,
    pub skill_blocked_until: std::collections::BTreeMap<u32, u128>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PendingItemSkill {
    pub skill_id: u32,
    pub level: u8,
    pub keep_requirements: bool,
    pub item_index: Option<usize>,
    pub source_item: Option<(i32, i32, i64)>,
    pub expires_at: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptSkillEffect {
    pub source_char_id: u32,
    pub target_id: u32,
    pub skill_id: u32,
    pub level: u8,
    pub heal_value: u32,
    pub proc_depth: u8,
    pub skill_event_emitted: bool,
    pub cast_generation: u64,
    pub action: ScriptSkillAction,
    pub deferred_requirements: Option<requirements::SkillRequirementPlan>,
    pub prepared_outcome: Option<PreparedSkillOutcome>,
    pub source_index: Option<usize>,
    pub source_item: Option<(i32, i32, i64)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScriptSkillAction {
    Cast,
    OpenWarpPortalMenu(WarpPortalMenuCast),
    MagicAttack {
        target_id: u32,
        map: crate::server::model::map_instance::MapInstanceKey,
        issued_skill: bool,
    },
    OpenTeleportMenu,
    CleanGraffiti {
        map: crate::server::model::map_instance::MapInstanceKey,
        x: u16,
        y: u16,
    },
    Heal {
        hp: u32,
        sp: u32,
    },
    SetResources {
        hp: Option<u32>,
        sp: Option<u32>,
    },
    ClearBuffs,
    RandomWarp,
    BreakEquipment {
        location: u64,
    },
    ActivateGround {
        skill_id: u32,
        cast_generation: u64,
    },
    TrapControl {
        trap_id: u32,
        map: crate::server::model::map_instance::MapInstanceKey,
        spring: bool,
        ignore_range: bool,
    },
    ExplodeSplasher,
    WaterBall {
        sequence: u64,
        cell: u16,
    },
    AreaStatus {
        x: u16,
        y: u16,
    },
    Summon {
        x: u16,
        y: u16,
    },
    Face {
        direction: u16,
    },
    FinalStrike {
        x: u16,
        y: u16,
        map: String,
        instance: u8,
    },
    DelayedWeaponHit {
        target_id: u32,
        map: String,
        instance: u8,
    },
    SnatchWarp {
        victim_id: u32,
        map: String,
        instance: u8,
    },
    DelayedStatus {
        request: StatusChangeRequest,
        map: String,
        instance: u8,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptSkillHit {
    pub source_map: Option<String>,
    pub source_instance: Option<u8>,
    pub source_id: u32,
    pub target_id: u32,
    pub skill_id: u32,
    pub skill_level: u8,
    pub damage: u32,
    pub depth: u8,
}

pub struct ScriptSkillService {
    client_notification_sender: SyncSender<Notification>,
    repository: Arc<dyn Repository>,
    configuration: &'static GlobalConfigService,
    ground_skills: Mutex<Vec<ground::GroundSkill>>,
    storm_gust_hits: Mutex<std::collections::HashMap<(String, u8, u32), u8>>,
    deferred_notifications: Mutex<std::collections::VecDeque<Notification>>,
    water_ball_sequences: Mutex<std::collections::HashMap<u32, area_damage::WaterBallSequence>>,
}

impl ScriptSkillService {
    pub fn new(
        client_notification_sender: SyncSender<Notification>,
        _persistence_event_sender: SyncSender<PersistenceEvent>,
        repository: Arc<dyn Repository>,
        configuration: &'static GlobalConfigService,
    ) -> Self {
        Self {
            client_notification_sender,
            repository,
            configuration,
            ground_skills: Mutex::new(vec![]),
            storm_gust_hits: Mutex::new(std::collections::HashMap::new()),
            deferred_notifications: Mutex::new(std::collections::VecDeque::new()),
            water_ball_sequences: Mutex::new(std::collections::HashMap::new()),
        }
    }

    pub fn validate_skill(&self, skill: &SkillConfig, level: u32) -> Result<(), String> {
        if level == 0 || level > u8::MAX as u32 {
            return Err("Item skill level must be between 1 and 255".into());
        }
        if skill.name() != "AL_TELEPORT" && level > skill.max_level() {
            return Err(format!("Item skill level exceeds {}", skill.name()));
        }
        if SkillEnum::try_from_value(skill.id).is_err() {
            return Err(format!("Unknown skill {}", skill.name()));
        }
        if Self::operation(skill.name()).is_some() {
            return Ok(());
        }
        if skills::skill_enums::to_object(SkillEnum::from_id(skill.id), level as u8).is_none() {
            return Err(format!("Skill {} has no executable implementation", skill.name()));
        }
        Ok(())
    }

    pub fn cast_cancelable(&self, skill_id: u32) -> bool {
        metadata::SkillMetadata::find(skill_id)
            .map(|skill| skill.cast_cancel.unwrap_or(true))
            .or_else(|| {
                self.configuration
                    .find_skill_config(&Value::Number(skill_id as i32))
                    .map(|skill| *skill.cast_cancel())
            })
            .unwrap_or(true)
    }

    pub fn active_cast_cancelable(&self, character: &Character) -> bool {
        character.script_skill_state.cast_cancel_override.unwrap_or_else(|| {
            let skill_id = character
                .skill_in_use
                .as_ref()
                .map(|cast| cast.skill.id())
                .unwrap_or(character.script_skill_state.casting_skill_id);
            self.cast_cancelable(skill_id)
        })
    }

    pub fn cancel_queued_cast(&self, character: &mut Character) {
        character.script_skill_state.cast_generation = character.script_skill_state.cast_generation.wrapping_add(1);
        character.script_skill_state.casting_until = 0;
        character.script_skill_state.casting_skill_id = 0;
        character.script_skill_state.casting_skill_level = 0;
        character.script_skill_state.cast_cancel_override = None;
        character.script_skill_state.deferred_requirements = None;
        character.script_skill_state.native_requirements = None;
        character.script_skill_state.pending_teleport = None;
        self.cancel_warp_portal_menu(character, crate::util::tick::get_tick());
    }

    pub fn validate_effect_cast(&self, source: &Character, effect: &ScriptSkillEffect) -> Result<(), String> {
        if source.char_id != effect.source_char_id
            || source.status.hp == 0
            || (effect.cast_generation != 0 && source.script_skill_state.cast_generation != effect.cast_generation)
        {
            return Err("Skill cast was interrupted".into());
        }
        Ok(())
    }

    pub fn handle_skill(
        &self,
        server: &Server,
        character: &mut Character,
        skill: &SkillConfig,
        level: u32,
        check_requirements: bool,
    ) -> Result<(), String> {
        self.validate_skill(skill, level)?;
        if character.status.hp == 0 {
            return Err("Dead characters cannot start item skills".into());
        }
        let tick = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_millis();
        if character.is_using_skill() || character.script_skill_state.casting_until > tick {
            return Err("Cannot activate an item skill while casting another skill".into());
        }
        if character.status.blocks_casting() {
            return Err("A status change prevents item skills".into());
        }
        if skill.name() == "AL_TELEPORT" {
            let (map, x, y) = Self::teleport_destination(character, level)?;
            server
                .server_service
                .schedule_warp_to_walkable_cell_by_character(&map, x, y, character.char_id);
            return Ok(());
        }
        character.pending_item_skill = Some(PendingItemSkill {
            skill_id: skill.id,
            level: level as u8,
            keep_requirements: check_requirements,
            item_index: None,
            source_item: None,
            expires_at: tick + 120_000,
        });
        if skill.name() == "MC_IDENTIFY" {
            return self.send_identification_list(character);
        }
        let source = StatusService::instance().to_snapshot(&character.status);
        let packet = Self::autorun_skill_packet_with_range(skill, level as u16, self.player_skill_range(&source, skill.id, level as u8));
        self.queue_notification(Notification::Char(CharNotification::new(character.char_id, packet)));
        Ok(())
    }

    pub fn teleport_destination(character: &Character, level: u32) -> Result<(String, u16, u16), String> {
        match level {
            1 => Ok((
                Map::name_without_ext(character.current_map_name()).to_string(),
                RANDOM_CELL.0,
                RANDOM_CELL.1,
            )),
            2 | 3 => Ok((
                Map::name_without_ext(&character.save_map).to_string(),
                character.save_x,
                character.save_y,
            )),
            _ => Err("Teleport supports levels 1, 2 and the item-only return level 3".into()),
        }
    }

    pub fn autorun_skill_packet(skill: &SkillConfig, level: u16) -> Vec<u8> {
        Self::autorun_skill_packet_with_range(skill, level, Self::range(skill, level as u8).unsigned_abs().min(14) as u16)
    }

    fn autorun_skill_packet_with_range(skill: &SkillConfig, level: u16, range: u16) -> Vec<u8> {
        let mut packet = Vec::with_capacity(39);
        packet.extend_from_slice(&0x0147_u16.to_le_bytes());
        packet.extend_from_slice(&(skill.id as u16).to_le_bytes());
        packet.extend_from_slice(&(skill.target_type().value() as u32).to_le_bytes());
        packet.extend_from_slice(&level.to_le_bytes());
        packet.extend_from_slice(&0_u16.to_le_bytes());
        packet.extend_from_slice(&range.to_le_bytes());
        let mut name = [0_u8; 24];
        let length = skill.name().len().min(23);
        name[..length].copy_from_slice(&skill.name().as_bytes()[..length]);
        packet.extend_from_slice(&name);
        packet.push(0);
        packet
    }

    pub fn cast_pending(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        event: &CharacterUseSkill,
        tick: u128,
    ) -> Result<Option<usize>, String> {
        self.validate_pending_cast(state, character, event, tick)?;
        let pending = character
            .pending_item_skill
            .as_ref()
            .ok_or("No item skill is awaiting a target")?
            .clone();
        if pending.expires_at <= tick {
            character.pending_item_skill = None;
            return Err("Item skill targeting expired".into());
        }
        if event.skill_id != pending.skill_id || event.skill_level != pending.level {
            return Err("Targeting reply does not match the pending item skill".into());
        }
        self.cast_skill(
            server,
            state,
            character,
            event.skill_id,
            event.skill_level,
            event.target_id,
            pending.keep_requirements,
            tick,
            false,
        )?;
        character.pending_item_skill = None;
        Ok(pending.item_index)
    }

    pub fn cast_proc(
        &self,
        server: &Server,
        state: &mut ServerState,
        source_id: u32,
        target_id: u32,
        skill_id: u32,
        level: u16,
        tick: u128,
    ) -> Result<(), String> {
        self.cast_proc_with_depth(server, state, source_id, target_id, skill_id, level, tick, 0)
    }

    pub fn cast_proc_with_depth(
        &self,
        server: &Server,
        state: &mut ServerState,
        source_id: u32,
        target_id: u32,
        skill_id: u32,
        level: u16,
        tick: u128,
        depth: u8,
    ) -> Result<(), String> {
        if depth >= 8 {
            return Err("Automatic skill recursion limit reached".into());
        }
        let mut character = state
            .characters_mut()
            .remove(&source_id)
            .ok_or("Autocast source is not in the game")?;
        let result = u8::try_from(level)
            .map_err(|_| "Autocast level is out of range".to_string())
            .and_then(|level| {
                self.cast_skill_depth(
                    server,
                    state,
                    &mut character,
                    skill_id,
                    level,
                    target_id,
                    false,
                    tick,
                    true,
                    depth,
                )
            });
        state.insert_character(character);
        result
    }

    pub fn cast_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        target_id: u32,
        keep_requirements: bool,
        tick: u128,
        instant: bool,
    ) -> Result<(), String> {
        self.cast_skill_depth(
            server,
            state,
            character,
            skill_id,
            level,
            target_id,
            keep_requirements,
            tick,
            instant,
            0,
        )
    }

    fn cast_skill_depth(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        target_id: u32,
        keep_requirements: bool,
        tick: u128,
        instant: bool,
        depth: u8,
    ) -> Result<(), String> {
        let skill = self
            .configuration
            .find_skill_config(&Value::Number(skill_id as i32))
            .ok_or("Unknown item skill")?;
        self.validate_skill(skill, level as u32)?;
        if character.status.hp == 0 {
            return Err("Dead characters cannot cast skills".into());
        }
        if !instant && character.timing.get_canact_tick() > tick {
            return Err("Skill delay has not ended".into());
        }
        if character.status.blocks_casting() {
            return Err("A status change prevents skill use".into());
        }
        Self::validate_stealth_cast(state, character, skill_id)?;
        Self::validate_skill_map(state, character, skill_id, level, true)?;
        if !instant && keep_requirements && character.script_skill_state.deferred_requirements.is_none() {
            let requirements = self.requirements_plan(character, skill_id, level, tick)?;
            character.script_skill_state.deferred_requirements = Some(requirements::DeferredSkillPayment {
                skill_id,
                level,
                keep_requirements: true,
                requirements,
                source_index: None,
                source_item: None,
            });
        }
        self.end_cloaking_on_skill(server, character, skill_id, tick);
        if matches!(skill.name().as_str(), "HT_REMOVETRAP" | "HT_SPRINGTRAP") {
            self.validate_player_trap_control(state, character, target_id, skill_id, level, tick, instant)?;
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: instant,
                cast_generation: 0,
                action: ScriptSkillAction::TrapControl {
                    trap_id: target_id,
                    map: character.map_instance_key.clone(),
                    spring: skill.name() == "HT_SPRINGTRAP",
                    ignore_range: instant,
                },
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            if instant {
                return self.apply_target_effect(server, state, character, &effect, tick);
            }
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        if (matches!(
            Self::operation(skill.name()),
            Some(callbacks::SkillOperation::Spirit | callbacks::SkillOperation::Movement)
        ) && skill.name() != "RG_INTIMIDATE")
            || matches!(
                skill.name().as_str(),
                "MC_VENDING" | "MC_PUSHCART" | "AM_CALLHOMUN" | "AM_REST" | "AM_RESURRECTHOMUN" | "WE_CALLPARTNER" | "WE_CALLBABY" | "WE_CALLPARENT"
            )
        {
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: instant,
                cast_generation: 0,
                action: ScriptSkillAction::Cast,
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            if instant {
                return self.apply_target_effect(server, state, character, &effect, tick);
            }
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        if matches!(Self::operation(skill.name()), Some(callbacks::SkillOperation::AreaStatus)) {
            if instant {
                return self.cast_area_status(server, state, character, skill_id, level, character.x, character.y, tick);
            }
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: false,
                cast_generation: 0,
                action: ScriptSkillAction::AreaStatus {
                    x: character.x,
                    y: character.y,
                },
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        if skill.name() == "BS_GREED" {
            if instant {
                return self.collect_nearby_items(server, state, character);
            }
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: false,
                cast_generation: 0,
                action: ScriptSkillAction::Cast,
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        let target = if target_id == character.char_id {
            Some(character.to_map_item_snapshot())
        } else {
            state.map_item_snapshot(target_id, character.current_map_name(), character.current_map_instance())
        }
        .ok_or("Item skill target is not on this map")?;
        self.validate_damage_target(state, character, skill_id, target_id)?;
        self.validate_support_target(state, character, skill_id, target_id)?;
        if !server.player_skill_target_allowed(state, character, target_id, skill_id, instant) {
            return Err("Hidden actors cannot be targeted by this skill".into());
        }
        if !instant
            && character.x.abs_diff(target.position.x).max(character.y.abs_diff(target.position.y))
                > self
                    .player_skill_range(&StatusService::instance().to_snapshot(&character.status), skill.id, level)
                    .max(1)
        {
            return Err("Item skill target is out of range".into());
        }
        if Self::uses_metadata_magic(skill.name()) {
            let issued_skill = instant
                || character
                    .pending_item_skill
                    .as_ref()
                    .is_some_and(|pending| pending.item_index.is_some() || pending.source_item.is_some());
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: instant,
                cast_generation: 0,
                action: ScriptSkillAction::MagicAttack {
                    target_id,
                    map: character.map_instance_key.clone(),
                    issued_skill,
                },
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            if instant {
                return self.apply_target_effect(server, state, character, &effect, tick);
            }
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        if matches!(Self::operation(skill.name()), Some(callbacks::SkillOperation::Ground)) {
            return self.place_ground_skill_depth(
                server,
                state,
                character,
                skill_id,
                level,
                target.position.x,
                target.position.y,
                tick,
                instant,
                depth,
            );
        }
        if skill.name() == "WZ_ESTIMATION" {
            if instant {
                return self.show_monster_estimation(server, state, character, target_id, level);
            }
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: false,
                cast_generation: 0,
                action: ScriptSkillAction::Cast,
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        if Self::status_for_skill(skill.name()).is_some() || Self::is_special_skill(skill.name()) {
            if !matches!(target.map_item.object_type(), MapItemType::Character) {
                if *target.map_item.object_type() != MapItemType::Mob {
                    return Err("Skill target does not support this effect".into());
                }
                let effect = ScriptSkillEffect {
                    source_char_id: character.char_id,
                    target_id,
                    skill_id,
                    level,
                    heal_value: self.source_heal_amount(character, level),
                    proc_depth: depth,
                    skill_event_emitted: instant,
                    cast_generation: 0,
                    action: ScriptSkillAction::Cast,
                    deferred_requirements: None,
                    prepared_outcome: None,
                    source_index: None,
                    source_item: None,
                };
                if instant {
                    self.apply_mob_target_effect(server, state, character, &effect, tick)?;
                } else {
                    self.queue_target_effect(server, character, skill, effect, tick);
                }
                return Ok(());
            }
            let _snapshot = StatusService::instance().to_snapshot(&character.status);
            let healing = self.source_heal_amount(character, level);
            let target_hp = if target_id == character.char_id {
                character.status.hp
            } else {
                state.get_character(target_id).ok_or("Player target disappeared")?.status.hp
            };
            if skill.name() == "AL_HEAL" && target_hp == 0 {
                return Err("Heal cannot resurrect a dead target".into());
            }
            if skill.name() == "ALL_RESURRECTION" && target_hp > 0 {
                return Err("Resurrection requires a dead target".into());
            }
            let effect = ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id,
                skill_id,
                level,
                heal_value: healing,
                proc_depth: depth,
                skill_event_emitted: instant,
                cast_generation: 0,
                action: ScriptSkillAction::Cast,
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            if instant && target_id == character.char_id {
                self.apply_target_effect(server, state, character, &effect, tick)?;
            } else {
                self.queue_target_effect(server, character, skill, effect, tick);
            }
            return Ok(());
        }
        let source_status = StatusService::instance().to_snapshot(&character.status);
        let target_status = match target.map_item.object_type() {
            MapItemType::Mob | MapItemType::SkillUnit => {
                state.map_item_mob_status(&target.map_item, character.current_map_name(), character.current_map_instance())
            }
            MapItemType::Character if target_id == character.char_id => Some(source_status.clone()),
            MapItemType::Character => state
                .get_character(target_id)
                .map(|target| StatusService::instance().to_snapshot(&target.status)),
            _ => None,
        }
        .ok_or("Item skill target has no battle status")?;
        let object = skills::skill_enums::to_object(SkillEnum::from_id(skill_id), level).ok_or("Skill has no executable implementation")?;
        if matches!(
            object.skill_type(),
            SkillType::Passive | SkillType::Interactive | SkillType::Performance
        ) {
            return Err(format!("Skill {} needs an interactive handler", skill.name()));
        }
        if instant {
            if let Some(offensive) = object.as_offensive_skill() {
                let landed = skill_id == SkillEnum::ChPalmstrike.id()
                    || !BattleService::is_weapon_skill(offensive)
                    || server.battle_service().skill_hits(&source_status, &target_status, skill_id, level);
                let (damage, magic_context) =
                    if landed && skill_id != SkillEnum::WzWaterball.id() && skill_id != SkillEnum::ChPalmstrike.id() {
                        server
                            .battle_service()
                            .calculate_damage_with_context(&source_status, &target_status, Some(offensive))
                    } else {
                        (0, None)
                    };
                let battle_flags = (if BattleService::is_weapon_skill(offensive) {
                    BattleFlag::Weapon
                } else if offensive.is_magic() {
                    BattleFlag::Magic
                } else {
                    BattleFlag::Misc
                })
                .as_flag()
                    | if skill_id == SkillEnum::TfThrowstone.id() {
                        BattleFlag::Weapon.as_flag()
                    } else {
                        0
                    }
                    | (if offensive.is_ranged() || offensive.is_magic() {
                        BattleFlag::Long
                    } else {
                        BattleFlag::Short
                    })
                    .as_flag()
                    | BattleFlag::Skill.as_flag();
                let mut damage_event = Damage {
                    notification: None,
                    source_kind: models::enums::actor::CombatActorKind::Player,
                    skill_damage_adjusted: false,
                    healing: 0,
                    right_hand_damage: None,
                    target_id,
                    attacker_id: character.char_id,
                    damage: 0,
                    attacked_at: tick,
                    damage_motion: 0,
                    battle_flags,
                    skill_id,
                    skill_level: level,
                    proc_depth: depth,
                    credit_id: 0,
                    defenses_applied: true,
                    magic_context,
                    landed,
                };
                damage_event.set_signed_damage(damage);
                if skill_id != SkillEnum::WzWaterball.id() && skill_id != SkillEnum::ChPalmstrike.id() {
                    damage_event = damage_event.with_skill_notification(
                        character.current_map_name(),
                        character.current_map_instance(),
                        character.x,
                        character.y,
                        tick,
                        1,
                        0,
                    );
                }
                let instance = state
                    .get_map_instance_from_character(character)
                    .ok_or("Map instance is unavailable")?;
                for (kind, damage) in self.complete_damage_skill(server, state, character, damage_event, server.battle_service(), tick)? {
                    if kind == MapItemType::Mob {
                        instance.add_to_next_tick(MapEvent::MobDamage(damage));
                    } else {
                        server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage }));
                    }
                }
                return Ok(());
            }
        }
        let receipt = character
            .script_skill_state
            .deferred_requirements
            .take()
            .filter(|payment| payment.skill_id == skill_id && payment.level == level);
        character.script_skill_state.native_requirements = if receipt.is_some() {
            receipt
        } else if keep_requirements {
            Some(requirements::DeferredSkillPayment {
                skill_id,
                level,
                keep_requirements: true,
                requirements: self.requirements_plan(character, skill_id, level, tick)?,
                source_index: None,
                source_item: None,
            })
        } else {
            None
        };
        let result = server.skill_service().start_item_skill(
            character,
            Some(target),
            &source_status,
            Some(&target_status),
            skill_id,
            level,
            tick,
            false,
        );
        if !result.is_valid() {
            return Err("Item skill requirements are not satisfied".into());
        }
        if result.has_no_delay() {
            server.server_service.character_use_skill(server, state, tick, character);
        }
        Ok(())
    }

    pub fn apply_target_effect(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        tick: u128,
    ) -> Result<(), String> {
        if effect.cast_generation != 0 {
            let source = if effect.source_char_id == character.char_id {
                Some(&*character)
            } else {
                state.get_character(effect.source_char_id)
            }
            .ok_or("Caster left before the skill completed")?;
            if source.status.hp == 0 || source.script_skill_state.cast_generation != effect.cast_generation {
                return Err("Skill cast was interrupted".into());
            }
        }
        let skill = self
            .configuration
            .find_skill_config(&Value::Number(effect.skill_id as i32))
            .ok_or("Unknown item skill")?;
        if effect.prepared_outcome.is_none()
            && matches!(effect.action, ScriptSkillAction::Cast)
            && (effect.skill_id == SkillEnum::MgStonecurse.id() || effect.skill_id == SkillEnum::CgTarotcard.id())
        {
            let snapshot = StatusService::instance().to_snapshot(&character.status);
            let plan = self.prepare_conditional_completion(
                effect,
                &character.status,
                &snapshot,
                Self::conditional_player_immunity(character, effect),
                tick,
            )?;
            if plan.cost != requirements::SkillRequirementPlan::default() {
                return Err("Conditional skill completion requires its payment transaction".into());
            }
            return self.apply_target_effect(server, state, character, &plan.effect, tick);
        }
        if effect.source_char_id == character.char_id && !effect.skill_event_emitted && matches!(effect.action, ScriptSkillAction::Cast) {
            character.script_skill_state.casting_until = 0;
            character.script_skill_state.casting_skill_id = 0;
        }
        if self.apply_script_action(server, state, character, effect, tick)? {
            return Ok(());
        }
        if self.apply_player_support_skill(server, state, character, effect, tick)? {
            return Ok(());
        }
        if self.apply_targeted_player_skill(server, state, character, effect, tick)? {
            self.notify_support_skill(character, effect);
            return Ok(());
        }
        if self.apply_utility_skill(server, state, character, effect, tick)? {
            return Ok(());
        }
        if let Some(kind) = Self::status_for_skill(skill.name()) {
            let toggled = metadata::SkillMetadata::find(effect.skill_id)
                .is_some_and(|metadata| metadata.flags.get("Toggleable").copied().unwrap_or(false))
                && character.status.has_status_change(kind);
            if toggled {
                StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
            } else {
                let mut request = Self::skill_status_request(skill, kind, effect.level);
                if kind == StatusChangeKind::Cloaking && Self::adjacent_cloaking_wall(state, character) {
                    request.values[3] |= models::status_change::CloakingFlag::AdjacentWall.as_flag() as i32;
                }
                StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
            }
            if kind == StatusChangeKind::Concentrate {
                self.reveal_from_actor(server, state, &self.reveal_actor(character, kind), tick)?;
            }
        } else {
            match skill.name().as_str() {
                "AL_HEAL" => {
                    if character.status.hp == 0 {
                        return Err("Heal cannot resurrect a dead target".into());
                    }
                    if character.status.has_status_change(StatusChangeKind::NoRecovery)
                        || character.status.has_status_change(StatusChangeKind::Berserk)
                    {
                        return Err("Recovery is disabled by a status change".into());
                    }
                    let snapshot = StatusService::instance().to_snapshot(&character.status);
                    let healing = Self::target_heal_amount(&snapshot, effect.heal_value);
                    server.character_service().update_hp_sp(
                        character,
                        character.status.hp.saturating_add(healing).min(snapshot.max_hp()),
                        character.status.sp,
                    );
                }
                "ALL_RESURRECTION" => {
                    if character.status.hp != 0 {
                        return Err("Resurrection requires a dead target".into());
                    }
                    if character.status.has_status_change(StatusChangeKind::HellPower) {
                        return Err("Hell Power prevents resurrection".into());
                    }
                    let snapshot = StatusService::instance().to_snapshot(&character.status);
                    let (hp, sp) = Self::resurrection_resources(&snapshot, effect.level)?;
                    server.character_service().update_hp_sp(character, hp, sp);
                    character.action = crate::server::state::character::CharacterAction::Idle;
                    let mut packet = 0x0148_u16.to_le_bytes().to_vec();
                    packet.extend_from_slice(&character.char_id.to_le_bytes());
                    packet.extend_from_slice(&0_u16.to_le_bytes());
                    self.notify_area(character, packet);
                }
                "TF_DETOXIFY" => {
                    StatusEffectService::end(
                        server,
                        character,
                        Some(StatusChangeKind::Poison),
                        tick,
                        &self.client_notification_sender,
                    );
                    StatusEffectService::end(
                        server,
                        character,
                        Some(StatusChangeKind::DeadlyPoison),
                        tick,
                        &self.client_notification_sender,
                    );
                }
                "AL_CURE" => {
                    for kind in [StatusChangeKind::Silence, StatusChangeKind::Blind, StatusChangeKind::Confusion] {
                        StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
                    }
                }
                "PR_STRECOVERY" => {
                    for kind in [
                        StatusChangeKind::Stone,
                        StatusChangeKind::StoneWait,
                        StatusChangeKind::Freeze,
                        StatusChangeKind::Stun,
                        StatusChangeKind::Sleep,
                        StatusChangeKind::NoRecovery,
                    ] {
                        StatusEffectService::end(server, character, Some(kind), tick, &self.client_notification_sender);
                    }
                }
                "SA_DISPELL" => {
                    if fastrand::u32(0..100) < 50 + 10 * effect.level as u32 {
                        let removed = StatusEffectService::dispel_statuses(&mut character.status, false);
                        for kind in removed {
                            StatusEffectService::send_icon(character, kind, false, tick, &self.client_notification_sender);
                        }
                        server.character_service().reload_client_side_status(character);
                        StatusEffectService::send_visual_status(character, &self.client_notification_sender);
                    }
                }
                "NV_FIRSTAID" => {
                    let snapshot = StatusService::instance().to_snapshot(&character.status);
                    server.character_service().update_hp_sp(
                        character,
                        character.status.hp.saturating_add(5).min(snapshot.max_hp()),
                        character.status.sp,
                    );
                }
                "AL_TELEPORT" => {
                    Self::validate_skill_map(state, character, effect.skill_id, effect.level, true)?;
                    let (map, x, y) = Self::teleport_destination(character, effect.level as u32)?;
                    server
                        .server_service
                        .schedule_warp_to_walkable_cell_by_character(&map, x, y, character.char_id);
                }
                "ALL_REVERSEORCISH" | "SA_REVERSEORCISH" => {
                    StatusEffectService::start(
                        server,
                        character,
                        StatusChangeRequest::guaranteed(StatusChangeKind::Orcish, 20000, 1),
                        tick,
                        &self.client_notification_sender,
                    )?;
                }
                "ITEM_ENCHANTARMS" => {
                    let request = StatusChangeRequest::guaranteed(
                        StatusChangeKind::EnchantArms,
                        Self::duration(skill, effect.level).max(90000) as i32,
                        effect.level.saturating_sub(1) as i32,
                    );
                    StatusEffectService::start(server, character, request, tick, &self.client_notification_sender)?;
                }
                "MC_IDENTIFY" => self.send_identification_list(character)?,
                "TF_PICKSTONE" => {
                    let item = self.configuration.find_item(7049).ok_or("Stone item asset is unavailable")?;
                    server.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems {
                        char_id: character.char_id,
                        should_perform_check: true,
                        buy: false,
                        items: vec![crate::repository::model::item_model::InventoryItemModel::from_item_model(
                            item, 1, true,
                        )],
                    }));
                }
                _ => return Err(format!("Item skill {} has no target handler", skill.name())),
            }
        }
        self.notify_support_skill(character, effect);
        Ok(())
    }

    fn send_identification_list(&self, character: &Character) -> Result<(), String> {
        let indices = character
            .inventory
            .iter()
            .enumerate()
            .filter_map(|(index, item)| item.as_ref().filter(|item| !item.is_identified).map(|_| index))
            .collect::<Vec<_>>();
        if indices.is_empty() {
            return Err("There are no unidentified items".into());
        }
        let length = u16::try_from(4 + indices.len() * 2).map_err(|_| "Identification list is too large")?;
        let mut packet = 0x0177_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&length.to_le_bytes());
        for index in indices {
            packet.extend_from_slice(
                &u16::try_from(index + 2)
                    .map_err(|_| "Identification index is out of range")?
                    .to_le_bytes(),
            );
        }
        self.queue_notification(Notification::Char(CharNotification::new(character.char_id, packet)));
        Ok(())
    }

    pub fn identify_selected(&self, server: &Server, character: &mut Character, index: usize, tick: u128) -> Result<Option<usize>, String> {
        let item = self.validate_identification(character, index, tick)?;
        server
            .runtime()
            .block_on(self.repository.character_identify_item(character.char_id, item))
            .map_err(|error| error.to_string())?;
        self.apply_identification_result(character, index)
    }

    pub fn validate_identification(
        &self,
        character: &Character,
        index: usize,
        tick: u128,
    ) -> Result<crate::repository::model::item_model::InventoryItemModel, String> {
        let pending = character
            .pending_item_skill
            .as_ref()
            .ok_or("Identification was not requested")?
            .clone();
        if pending.skill_id != SkillEnum::McIdentify.id() || pending.expires_at <= tick {
            return Err("Identification request is not active".into());
        }
        character
            .get_item_from_inventory(index)
            .filter(|item| !item.is_identified)
            .cloned()
            .ok_or_else(|| "Selected item is already identified or missing".into())
    }

    pub fn apply_identification_result(&self, character: &mut Character, index: usize) -> Result<Option<usize>, String> {
        let pending = character
            .pending_item_skill
            .as_ref()
            .ok_or("Identification was not requested")?
            .clone();
        character.get_item_from_inventory(index).ok_or("Selected item disappeared")?;
        let client_index = u16::try_from(index + 2).map_err(|_| "Identification index is out of range")?;
        character.inventory[index].as_mut().unwrap().is_identified = true;
        character.pending_item_skill = None;
        let mut packet = 0x0179_u16.to_le_bytes().to_vec();
        packet.extend_from_slice(&client_index.to_le_bytes());
        packet.push(0);
        self.queue_notification(Notification::Char(CharNotification::new(character.char_id, packet)));
        Ok(pending.item_index)
    }

    fn status_for_skill(name: &str) -> Option<StatusChangeKind> {
        use StatusChangeKind::*;
        match name {
            "SM_ENDURE" => Some(Endure),
            "SM_PROVOKE" | "SM_SELFPROVOKE" => Some(Provoke),
            "AL_ANGELUS" => Some(Angelus),
            "AL_BLESSING" => Some(Blessing),
            "AL_INCAGI" => Some(IncreaseAgi),
            "AL_DECAGI" => Some(DecreaseAgi),
            "AC_CONCENTRATION" => Some(Concentrate),
            "MC_LOUD" => Some(Loud),
            "PR_MAGNIFICAT" => Some(Magnificat),
            "PR_GLORIA" => Some(Gloria),
            "PR_IMPOSITIO" => Some(Impositio),
            "PR_ASPERSIO" => Some(Aspersio),
            "PR_KYRIE" => Some(Kyrie),
            "PR_SUFFRAGIUM" => Some(Suffragium),
            "PR_LEXAETERNA" => Some(LexAeterna),
            "HP_ASSUMPTIO" => Some(Assumptio),
            "BS_ADRENALINE" => Some(Adrenaline),
            "BS_WEAPONPERFECT" => Some(WeaponPerfection),
            "BS_OVERTHRUST" => Some(Overthrust),
            "KN_TWOHANDQUICKEN" => Some(TwoHandQuicken),
            "SN_WINDWALK" => Some(WindWalk),
            "MG_SIGHT" => Some(Sight),
            "AL_RUWACH" => Some(Ruwach),
            "ALL_PARTYFLEE" => Some(PartyFlee),
            "ALL_ANGEL_PROTECT" => Some(IncAllStatus),
            "PR_LEXDIVINA" => Some(Silence),
            "AL_PNEUMA" => Some(Pneuma),
            "AL_CRUCIS" => Some(SignumCrucis),
            "SA_FLAMELAUNCHER" => Some(FireWeapon),
            "SA_FROSTWEAPON" => Some(WaterWeapon),
            "SA_LIGHTNINGLOADER" => Some(WindWeapon),
            "SA_SEISMICWEAPON" => Some(EarthWeapon),
            "CR_AUTOGUARD" => Some(AutoGuard),
            "CR_REFLECTSHIELD" => Some(ReflectShield),
            "MO_EXPLOSIONSPIRITS" => Some(ExplosionSpirits),
            "LK_AURABLADE" => Some(AuraBlade),
            "LK_CONCENTRATION" => Some(Concentration),
            "NPC_MAGICMIRROR" => Some(MagicMirror),
            "NPC_POWERUP" => Some(IncAttackRate),
            "NPC_DEFENDER" => Some(Armor),
            "NPC_WEAPONBRAKER" => Some(WeaponBreaker),
            "KN_AUTOCOUNTER" => Some(AutoCounter),
            "NPC_STONESKIN" | "NPC_ANTIMAGIC" => Some(ArmorChange),
            "NPC_SLOWCAST" => Some(SlowCast),
            "NPC_CRITICALWOUND" => Some(CriticalWound),
            "NPC_HELLPOWER" => Some(HellPower),
            "WZ_QUAGMIRE" => Some(Quagmire),
            "MG_ENERGYCOAT" => Some(EnergyCoat),
            "TF_HIDING" => Some(Hiding),
            "AS_CLOAKING" => Some(Cloaking),
            "ST_CHASEWALK" => Some(ChaseWalk),
            "BS_MAXIMIZE" => Some(MaximizePower),
            "SA_MAGICROD" => Some(MagicRod),
            "NJ_NEN" => Some(Nen),
            _ => None,
        }
    }

    fn is_special_skill(name: &str) -> bool {
        matches!(
            name,
            "AL_TELEPORT"
                | "AL_HEAL"
                | "ALL_RESURRECTION"
                | "MC_IDENTIFY"
                | "TF_DETOXIFY"
                | "AL_CURE"
                | "PR_STRECOVERY"
                | "SA_DISPELL"
                | "SA_SPELLBREAKER"
                | "CG_TAROTCARD"
                | "MG_STONECURSE"
                | "DC_WINKCHARM"
                | "RG_STRIPARMOR"
                | "RG_STRIPWEAPON"
                | "RG_STRIPSHIELD"
                | "RG_STRIPHELM"
                | "ST_FULLSTRIP"
                | "AS_SPLASHER"
                | "CR_DEVOTION"
                | "TK_MISSION"
                | "NV_FIRSTAID"
                | "ALL_REVERSEORCISH"
                | "SA_REVERSEORCISH"
                | "ITEM_ENCHANTARMS"
                | "TF_PICKSTONE"
        )
    }

    fn skill_status_request(skill: &SkillConfig, kind: StatusChangeKind, level: u8) -> StatusChangeRequest {
        let mut request = StatusChangeRequest::guaranteed(kind, Self::duration(skill, level).min(i32::MAX as u32) as i32, level as i32);
        if skill.name() == "NPC_ANTIMAGIC" {
            request.values[1] = skill.id as i32;
        }
        if matches!(kind, StatusChangeKind::Sight | StatusChangeKind::Ruwach) {
            request.values[1] = skill.id as i32;
        }
        request.flags = 0;
        request
    }

    fn range(skill: &SkillConfig, level: u8) -> i32 {
        metadata::SkillMetadata::find(skill.id)
            .and_then(|skill| skill.range(level))
            .or_else(|| {
                skill
                    .range_per_level()
                    .as_ref()
                    .and_then(|values| values.get(level.saturating_sub(1) as usize))
                    .copied()
                    .or(*skill.range())
            })
            .unwrap_or(1)
    }

    pub fn player_skill_range(&self, source: &StatusSnapshot, skill_id: u32, level: u8) -> u16 {
        if let Some(metadata) = metadata::SkillMetadata::find(skill_id) {
            return metadata.player_range(source, level);
        }
        self.configuration
            .find_skill_config(&Value::Number(skill_id as i32))
            .map_or(1, |skill| Self::range(skill, level).unsigned_abs().min(14) as u16)
    }

    fn duration(skill: &SkillConfig, level: u8) -> u32 {
        metadata::SkillMetadata::find(skill.id)
            .and_then(|skill| skill.duration(level, false))
            .map(|value| value.max(0) as u32)
            .or_else(|| {
                skill
                    .duration1_per_level()
                    .as_ref()
                    .and_then(|values| values.get(level.saturating_sub(1) as usize))
                    .copied()
                    .or(*skill.duration1())
            })
            .unwrap_or(0)
    }

    pub fn heal_amount(status: &StatusSnapshot, base_level: u32, level: u8) -> u32 {
        (base_level + status.int() as u32) / 8 * (4 + 8 * level as u32)
    }

    pub fn resurrection_hp_percent(level: u8) -> Result<u8, String> {
        match level {
            1 => Ok(10),
            2 => Ok(30),
            3 => Ok(50),
            4 => Ok(80),
            _ => Err("Resurrection level must be between 1 and 4".into()),
        }
    }

    pub fn resurrection_resources(status: &StatusSnapshot, level: u8) -> Result<(u32, u32), String> {
        let percent = Self::resurrection_hp_percent(level)?;
        if status
            .bonuses_raw()
            .iter()
            .any(|bonus| matches!(bonus, models::enums::bonus::BonusType::EnableFullHpSpRecoverOnResurrect))
        {
            return Ok((status.max_hp(), status.max_sp()));
        }
        Ok((
            (u64::from(status.max_hp()) * u64::from(percent) / 100).max(1) as u32,
            status.sp(),
        ))
    }

    fn notify_area(&self, character: &Character, packet: Vec<u8>) {
        if let Err(error) = self.client_notification_sender.try_send(Notification::Area(AreaNotification::new(
            character.current_map_name().clone(),
            character.current_map_instance(),
            AreaNotificationRangeType::Fov {
                x: character.x,
                y: character.y,
                exclude_id: None,
            },
            packet,
        ))) {
            warn!("Unable to notify item skill: {}", error);
        }
    }

    fn queue_notification(&self, notification: Notification) {
        use std::sync::mpsc::TrySendError;
        let Ok(mut pending) = self.deferred_notifications.lock() else {
            error!("Skill notification queue is unavailable");
            return;
        };
        if !pending.is_empty() {
            pending.push_back(notification);
            return;
        }
        match self.client_notification_sender.try_send(notification) {
            Ok(()) => {}
            Err(TrySendError::Full(notification)) => pending.push_back(notification),
            Err(TrySendError::Disconnected(_)) => warn!("Client notification receiver disconnected"),
        }
    }

    pub(crate) fn retry_pending_notifications(&self) {
        use std::sync::mpsc::TrySendError;
        let Ok(mut pending) = self.deferred_notifications.lock() else {
            return;
        };
        while let Some(notification) = pending.pop_front() {
            match self.client_notification_sender.try_send(notification) {
                Ok(()) => {}
                Err(TrySendError::Full(notification)) => {
                    pending.push_front(notification);
                    break;
                }
                Err(TrySendError::Disconnected(_)) => {
                    pending.clear();
                    break;
                }
            }
        }
    }

    fn notify_support_skill(&self, character: &Character, effect: &ScriptSkillEffect) {
        self.notify_support_skill_result(character, effect, true);
    }

    fn notify_support_skill_result(&self, character: &Character, effect: &ScriptSkillEffect, succeeded: bool) {
        let mut packet = PacketZcUseSkill::new(self.configuration.packetver());
        packet.set_src_aid(effect.source_char_id);
        packet.set_target_aid(effect.target_id);
        packet.set_skid(effect.skill_id as u16);
        packet.set_level(effect.level as i16);
        packet.set_result(succeeded);
        packet.fill_raw();
        self.notify_area(character, packet.raw);
    }

    pub fn notify_magic_rod_absorption(&self, character: &Character, incoming_level: u8) {
        let mut packet = PacketZcUseSkill::new(self.configuration.packetver());
        packet.set_src_aid(character.char_id);
        packet.set_target_aid(character.char_id);
        packet.set_skid(SkillEnum::SaMagicrod.id() as u16);
        packet.set_level(i16::from(incoming_level));
        packet.set_result(true);
        packet.fill_raw();
        self.notify_area(character, packet.raw);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fly_and_butterfly_wings_use_distinct_destinations() {
        let mut character = crate::tests::common::character_helper::create_character();
        character.save_map = "payon".into();
        character.save_x = 100;
        character.save_y = 77;
        assert_eq!(
            ScriptSkillService::teleport_destination(&character, 1).unwrap(),
            ("Prontera".into(), RANDOM_CELL.0, RANDOM_CELL.1)
        );
        assert_eq!(
            ScriptSkillService::teleport_destination(&character, 3).unwrap(),
            ("payon".into(), 100, 77)
        );
    }

    #[test]
    fn resurrection_recovers_classic_hp_percentages() {
        assert_eq!(
            (1..=4)
                .map(|level| ScriptSkillService::resurrection_hp_percent(level).unwrap())
                .collect::<Vec<_>>(),
            vec![10, 30, 50, 80]
        );
        assert!(ScriptSkillService::resurrection_hp_percent(5).is_err());
    }

    #[test]
    fn full_recovery_equipment_restores_both_resurrection_pools() {
        let mut snapshot = StatusSnapshot::_from(&models::status::Status {
            max_hp: 1000,
            max_sp: 400,
            sp: 17,
            ..models::status::Status::default()
        });
        snapshot.set_max_hp(1000);
        snapshot.set_max_sp(400);
        assert_eq!(ScriptSkillService::resurrection_resources(&snapshot, 1).unwrap(), (100, 17));
        snapshot.set_bonuses(vec![models::status_bonus::StatusBonus::new(
            models::enums::bonus::BonusType::EnableFullHpSpRecoverOnResurrect,
        )]);
        assert_eq!(ScriptSkillService::resurrection_resources(&snapshot, 1).unwrap(), (1000, 400));
    }

    #[test]
    fn item_skill_packet_preserves_skill_id_level_and_target_mode() {
        let skill: SkillConfig = serde_json::from_str(
            r#"{"id":19,"name":"MG_FIREBOLT","description":"Fire Bolt","maxLevel":10,"targetType":"Target","range":9}"#,
        )
        .unwrap();
        let packet = ScriptSkillService::autorun_skill_packet(&skill, 5);
        assert_eq!(packet.len(), 39);
        assert_eq!(u16::from_le_bytes(packet[2..4].try_into().unwrap()), skill.id as u16);
        assert_eq!(u16::from_le_bytes(packet[8..10].try_into().unwrap()), 5);
        assert_eq!(&packet[14..26], b"MG_FIREBOLT\0");
    }
}
