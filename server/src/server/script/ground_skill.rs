use std::collections::HashSet;
use std::sync::atomic::{AtomicU32, Ordering};

use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
use models::status::StatusSnapshot;
use models::status_bonus::{BattleFlag, CombatTrigger};
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use script_sdk::Value;

use super::ScriptSkillService;
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::MapEvent;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

/// A spent Safety Wall removes its status from the target, which is how the wall learns it is used up.
const SAFETY_WALL_APPLY_GRACE_MS: u128 = 400;

pub(super) static NEXT_GROUND_UNIT: AtomicU32 = AtomicU32::new(2_000_000);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroundKind {
    WarpPortal,
    Firewall,
    Pneuma,
    Quagmire,
    Deluge,
    LandProtector,
    Thunderstorm,
    HeavenDrive,
    Meteor,
    StormGust,
    Vermilion,
    GrandCross,
    GrandDarkness,
    SkidTrap,
    AnkleSnare,
    LandMine,
    BlastMine,
    ClaymoreTrap,
    Shockwave,
    Flasher,
    Sandman,
    FreezingTrap,
    TalkieBox,
    Graffiti,
    ArrowShower,
    Earthquake,
    SafetyWall,
    Sanctuary,
    VenomDust,
    SpiderWeb,
}

impl GroundKind {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "AL_WARP" => Self::WarpPortal,
            "MG_FIREWALL" => Self::Firewall,
            "AL_PNEUMA" => Self::Pneuma,
            "MG_SAFETYWALL" => Self::SafetyWall,
            "PR_SANCTUARY" => Self::Sanctuary,
            "AS_VENOMDUST" => Self::VenomDust,
            "PF_SPIDERWEB" => Self::SpiderWeb,
            "WZ_QUAGMIRE" => Self::Quagmire,
            "SA_DELUGE" => Self::Deluge,
            "SA_LANDPROTECTOR" => Self::LandProtector,
            "MG_THUNDERSTORM" => Self::Thunderstorm,
            "WZ_HEAVENDRIVE" => Self::HeavenDrive,
            "WZ_METEOR" => Self::Meteor,
            "WZ_STORMGUST" => Self::StormGust,
            "WZ_VERMILION" => Self::Vermilion,
            "CR_GRANDCROSS" => Self::GrandCross,
            "NPC_GRANDDARKNESS" => Self::GrandDarkness,
            "MA_SKIDTRAP" | "HT_SKIDTRAP" => Self::SkidTrap,
            "HT_ANKLESNARE" => Self::AnkleSnare,
            "MA_LANDMINE" | "HT_LANDMINE" => Self::LandMine,
            "HT_BLASTMINE" => Self::BlastMine,
            "HT_CLAYMORETRAP" => Self::ClaymoreTrap,
            "HT_SHOCKWAVE" => Self::Shockwave,
            "HT_FLASHER" => Self::Flasher,
            "MA_SANDMAN" | "HT_SANDMAN" => Self::Sandman,
            "MA_FREEZINGTRAP" | "HT_FREEZINGTRAP" => Self::FreezingTrap,
            "HT_TALKIEBOX" => Self::TalkieBox,
            "RG_GRAFFITI" => Self::Graffiti,
            "MA_SHOWER" => Self::ArrowShower,
            "NPC_EARTHQUAKE" => Self::Earthquake,
            _ => return None,
        })
    }

    pub(super) fn view_id(self) -> u32 {
        match self {
            Self::WarpPortal => 129,
            Self::Firewall => 127,
            Self::Pneuma => 133,
            Self::SafetyWall => 126,
            Self::Sanctuary => 131,
            Self::VenomDust => 146,
            Self::SpiderWeb => 183,
            Self::Quagmire => 142,
            Self::Deluge => 155,
            Self::LandProtector => 157,
            Self::SkidTrap => 144,
            Self::AnkleSnare => 145,
            Self::LandMine => 147,
            Self::BlastMine => 143,
            Self::ClaymoreTrap => 152,
            Self::Shockwave => 148,
            Self::Flasher => 150,
            Self::Sandman => 149,
            Self::FreezingTrap => 151,
            Self::TalkieBox => 153,
            Self::Graffiti => 176,
            Self::Earthquake => 198,
            _ => 134,
        }
    }

    fn status(self) -> Option<StatusChangeKind> {
        match self {
            Self::Pneuma => Some(StatusChangeKind::Pneuma),
            Self::SafetyWall => Some(StatusChangeKind::SafetyWall),
            Self::Quagmire => Some(StatusChangeKind::Quagmire),
            Self::Deluge => Some(StatusChangeKind::Deluge),
            _ => None,
        }
    }

    /// Kinds that the actor (monster and NPC) cast pipeline can place and run to completion.
    pub(super) fn actor_placeable(self) -> bool {
        matches!(
            self,
            Self::HeavenDrive
                | Self::Thunderstorm
                | Self::Pneuma
                | Self::SafetyWall
                | Self::Sanctuary
                | Self::VenomDust
                | Self::SpiderWeb
                | Self::Quagmire
                | Self::Deluge
                | Self::LandProtector
                | Self::SkidTrap
                | Self::LandMine
                | Self::Sandman
                | Self::FreezingTrap
                | Self::ArrowShower
                | Self::Firewall
                | Self::Meteor
                | Self::StormGust
                | Self::Vermilion
                | Self::Earthquake
                | Self::GrandCross
                | Self::GrandDarkness
        )
    }

    fn damaging(self) -> bool {
        !matches!(
            self,
            Self::WarpPortal
                | Self::TalkieBox
                | Self::Graffiti
                | Self::Pneuma
                | Self::SafetyWall
                | Self::Sanctuary
                | Self::VenomDust
                | Self::SpiderWeb
                | Self::GrandDarkness
                | Self::Quagmire
                | Self::Deluge
                | Self::LandProtector
        )
    }

    pub(super) fn effect_range(self, configured: u16) -> u16 {
        match self {
            Self::Pneuma => 1,
            Self::Sanctuary | Self::VenomDust | Self::SpiderWeb => 0,
            _ => configured,
        }
    }

    pub(super) fn trap(self) -> bool {
        matches!(
            self,
            Self::SkidTrap
                | Self::AnkleSnare
                | Self::LandMine
                | Self::BlastMine
                | Self::ClaymoreTrap
                | Self::Shockwave
                | Self::Flasher
                | Self::Sandman
                | Self::FreezingTrap
                | Self::TalkieBox
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GroundSkillSource {
    pub actor_id: u32,
    pub owner_id: u32,
    pub map: String,
    pub instance: u8,
    pub x: u16,
    pub y: u16,
    pub status: StatusSnapshot,
    pub raw_attack: u32,
    pub fixed_damage: Option<FixedGroundSkillDamage>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedGroundSkillDamage {
    pub amount: u32,
    pub hits: i16,
    pub ignore_infinite_defense: bool,
}

pub struct GroundSkill {
    pub(super) portal: Option<super::warp_portal::WarpPortalState>,
    pub message: Vec<u8>,
    pub capture: Option<super::trap::TrapCaptureState>,
    pub recovery_item: Option<i32>,
    pub kind: GroundKind,
    pub source_id: u32,
    pub source_x: u16,
    pub source_y: u16,
    pub map: String,
    pub instance: u8,
    pub skill_id: u32,
    pub level: u8,
    pub depth: u8,
    pub active_from: u128,
    pub expires_at: u128,
    pub next_hit_at: u128,
    pub interval: u128,
    pub effect_range: u16,
    pub displayed: bool,
    pub skill_event_emitted: bool,
    pub cast_generation: u64,
    pub cast_finish_at: u128,
    pub cast_verified: bool,
    pub cells: Vec<GroundCell>,
    pub affected: HashSet<u32>,
    pub actor_source: Option<GroundSkillSource>,
    pub triggered: bool,
    pub waves: u8,
}

pub struct GroundCell {
    pub observers: std::collections::HashMap<u32, std::sync::Weak<crate::server::model::session::Session>>,
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub remaining_hits: u16,
}

impl GroundSkill {
    pub(super) fn covers(&self, x: u16, y: u16) -> bool {
        self.cells
            .iter()
            .any(|cell| cell.remaining_hits > 0 && cell.x.abs_diff(x).max(cell.y.abs_diff(y)) <= self.effect_range)
    }
}

impl ScriptSkillService {
    pub fn validate_pending_ground(
        &self,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        let pending = character
            .pending_item_skill
            .as_ref()
            .ok_or("No item skill is awaiting a ground target")?;
        if pending.expires_at <= tick {
            return Err("Item skill targeting expired".into());
        }
        if skill_id != pending.skill_id || level != pending.level {
            return Err("Ground targeting reply does not match the pending item skill".into());
        }
        self.validate_ground_target(state, character, skill_id, level, x, y, tick)?;
        if pending.keep_requirements {
            self.requirements_plan(character, skill_id, level, tick)?;
        }
        Ok(())
    }

    pub fn cast_pending_ground(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<Option<usize>, String> {
        self.validate_pending_ground(state, character, skill_id, level, x, y, tick)?;
        let pending = character.pending_item_skill.as_ref().unwrap().clone();
        self.place_ground_skill(server, state, character, skill_id, level, x, y, tick)?;
        character.pending_item_skill = None;
        Ok(pending.item_index)
    }

    pub fn validate_ground_target(
        &self,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        self.validate_ground_target_with_mode(state, character, skill_id, level, x, y, tick, false)
    }

    fn validate_ground_target_with_mode(
        &self,
        state: &ServerState,
        character: &Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
        instant: bool,
    ) -> Result<(), String> {
        let skill = self
            .configuration
            .find_skill_config(&Value::Number(skill_id as i32))
            .ok_or("Unknown ground skill")?;
        self.validate_skill(skill, level as u32)?;
        Self::validate_stealth_cast(state, character, skill_id)?;
        let issued = instant
            || character
                .pending_item_skill
                .as_ref()
                .is_some_and(|pending| pending.item_index.is_some() || pending.source_item.is_some());
        Self::validate_skill_map(state, character, skill_id, level, issued)?;
        let metadata = SkillMetadata::find(skill_id).ok_or("Pre-renewal ground definition is unavailable")?;
        if GroundKind::from_name(&metadata.name).is_none()
            && !matches!(metadata.name.as_str(), "BS_HAMMERFALL" | "RG_CLEANER" | "HW_GANBANTEIN" | "MO_BODYRELOCATION" | "AM_SPHEREMINE" | "AM_CANNIBALIZE")
        {
            return Err("Skill does not accept a ground target".into());
        }
        if character.status.hp == 0
            || character.status.blocks_casting()
            || (!instant && (character.script_skill_state.casting_until > tick || character.timing.get_canact_tick() > tick))
        {
            return Err("Character cannot start a ground skill now".into());
        }
        if !instant
            && character.x.abs_diff(x).max(character.y.abs_diff(y))
                > self
                    .player_skill_range(&StatusService::instance().to_snapshot(&character.status), skill.id, level)
                    .max(1)
        {
            return Err("Ground skill target is out of range".into());
        }
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        if x >= instance.x_size()
            || y >= instance.y_size()
            || instance
                .state()
                .cells()
                .get(y as usize * instance.x_size() as usize + x as usize)
                .is_none_or(|cell| cell & CellType::Shootable.as_flag() == 0)
        {
            return Err("Ground skill target is outside usable terrain".into());
        }
        if GroundKind::from_name(&metadata.name).is_some_and(|kind| kind.trap() || kind == GroundKind::Graffiti) {
            self.validate_actor_ground_with_options(
                state,
                &GroundSkillSource {
                    actor_id: character.char_id,
                    owner_id: character.char_id,
                    map: character.current_map_name().clone(),
                    instance: character.current_map_instance(),
                    x: character.x,
                    y: character.y,
                    status: StatusService::instance().to_snapshot(&character.status),
                    raw_attack: 0,
                    fixed_damage: None,
                },
                skill_id,
                level,
                x,
                y,
                tick,
                true,
                true,
            )?;
        }
        if Self::is_alchemist_summon(skill_id) {
            self.validate_summon_limit(state, character, skill_id, level)?;
        }
        let active = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        if metadata.name == "MG_FIREWALL"
            && active
                .iter()
                .filter(|ground| ground.source_id == character.char_id && ground.kind == GroundKind::Firewall && ground.expires_at > tick)
                .count()
                >= 3
        {
            return Err("At most three Fire Walls may be active".into());
        }
        if metadata.name == "AL_PNEUMA"
            && active.iter().any(|ground| {
                ground.kind == GroundKind::Pneuma
                    && ground.map == *character.current_map_name()
                    && ground.instance == character.current_map_instance()
                    && ground.expires_at > tick
                    && ground.covers(x, y)
            })
        {
            return Err("A Pneuma already covers this cell".into());
        }
        Ok(())
    }

    pub fn place_ground_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        x: u16,
        y: u16,
        tick: u128,
    ) -> Result<(), String> {
        self.place_ground_skill_depth(server, state, character, skill_id, level, x, y, tick, false, 0)
    }

    pub fn place_ground_skill_depth(
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
    ) -> Result<(), String> {
        self.validate_ground_target_with_mode(state, character, skill_id, level, x, y, tick, instant)?;
        self.end_cloaking_on_skill(server, character, skill_id, tick);
        let metadata = SkillMetadata::find(skill_id).unwrap();
        if metadata.name == "AL_WARP" {
            return self.start_warp_portal_menu(server, state, character, skill_id, level, x, y, tick, instant, depth, None);
        }
        if metadata.name == "RG_CLEANER" {
            let skill = self.configuration.find_skill_config(&Value::Number(skill_id as i32)).unwrap();
            let effect = super::ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: instant,
                cast_generation: 0,
                action: super::ScriptSkillAction::CleanGraffiti {
                    map: character.map_instance_key.clone(),
                    x,
                    y,
                },
                deferred_requirements: None,
                prepared_outcome: None,
                source_index: None,
                source_item: None,
            };
            self.queue_target_effect(server, character, skill, effect, tick);
            return Ok(());
        }
        if Self::is_alchemist_summon(skill_id) {
            let skill = self.configuration.find_skill_config(&Value::Number(skill_id as i32)).unwrap();
            let effect = super::ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: false,
                cast_generation: 0,
                action: super::ScriptSkillAction::Summon { x, y },
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
        if matches!(metadata.name.as_str(), "BS_HAMMERFALL" | "HW_GANBANTEIN" | "MO_BODYRELOCATION") {
            if instant && metadata.name == "BS_HAMMERFALL" {
                return self.cast_area_status(server, state, character, skill_id, level, x, y, tick);
            }
            let skill = self.configuration.find_skill_config(&Value::Number(skill_id as i32)).unwrap();
            let effect = super::ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: false,
                cast_generation: 0,
                action: super::ScriptSkillAction::AreaStatus { x, y },
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
        let kind = GroundKind::from_name(&metadata.name).unwrap();
        let instance = state
            .get_map_instance_from_character(character)
            .ok_or("Map instance is unavailable")?;
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let cast_time = if instant {
            0
        } else {
            metadata.cast_duration(level, StatusService::skill_cast_modifier(&snapshot, skill_id))
        };
        let active_from = tick + cast_time;
        let cast_generation = if !instant {
            character.script_skill_state.cast_generation = character.script_skill_state.cast_generation.wrapping_add(1);
            character.script_skill_state.casting_skill_id = skill_id;
            character.script_skill_state.casting_skill_level = level;
            character.script_skill_state.cast_generation
        } else {
            0
        };
        if !instant {
            character.script_skill_state.casting_until = tick + cast_time.div_ceil(40).max(1) * 40;
            character.timing.set_canact_tick(
                active_from
                    + u128::from(StatusService::skill_after_cast_delay(
                        &snapshot,
                        skill_id,
                        metadata
                            .after_cast_act_delay
                            .as_ref()
                            .and_then(|value| value.value(level, "Time"))
                            .unwrap_or(0)
                            .max(0) as u32,
                    )),
            );
            character.timing.set_canmove_tick(
                character.timing.get_canmove_tick().max(
                    active_from
                        + metadata
                            .after_cast_walk_delay
                            .as_ref()
                            .and_then(|value| value.value(level, "Time"))
                            .unwrap_or(0)
                            .max(0) as u128,
                ),
            );
            if character.status.has_status_change(StatusChangeKind::Suffragium) {
                crate::server::service::status_effect_service::StatusEffectService::end_status_at(
                    &mut character.status,
                    Some(StatusChangeKind::Suffragium),
                    tick,
                );
                crate::server::service::status_effect_service::StatusEffectService::send_icon(
                    character,
                    StatusChangeKind::Suffragium,
                    false,
                    tick,
                    &self.client_notification_sender,
                );
            }
        }
        let base_duration = metadata.duration(level, false).unwrap_or(100);
        let duration = if kind == GroundKind::Meteor {
            base_duration.max(0) as u128
        } else {
            crate::server::service::map_flag_service::ground_skill_duration(
                &state.map_flags(&character.map_instance_key),
                skill_id,
                base_duration,
            )
        };
        let layout = metadata.unit_value("Layout", level, "Size").unwrap_or(0);
        let range = kind.effect_range(metadata.unit_value("Range", level, "Size").unwrap_or(0).max(0) as u16);
        let interval = metadata.unit_value("Interval", level, "Time").unwrap_or(-1);
        let interval = if interval < 0 { 40 } else { interval.max(40) as u128 };
        let centers = if kind == GroundKind::Meteor {
            let radius = metadata.splash(level).unwrap_or(3).max(0);
            (1..=duration / interval)
                .map(|number| {
                    (
                        (x as i32 + fastrand::i32(-radius..=radius)).clamp(0, instance.x_size() as i32 - 1) as u16,
                        (y as i32 + fastrand::i32(-radius..=radius)).clamp(0, instance.y_size() as i32 - 1) as u16,
                        number * interval,
                    )
                })
                .collect::<Vec<_>>()
        } else {
            vec![(x, y, 0)]
        };
        let mut active = self.ground_skills.lock().map_err(|_| "Ground skill state is unavailable")?;
        if matches!(kind, GroundKind::Deluge | GroundKind::LandProtector) {
            for ground in active.iter_mut().filter(|ground| {
                ground.source_id == character.char_id && matches!(ground.kind, GroundKind::Deluge | GroundKind::LandProtector)
            }) {
                ground.expires_at = active_from;
            }
        }
        for (number, (x, y, offset)) in centers.into_iter().enumerate() {
            let locations = match kind {
                GroundKind::Firewall => Self::firewall_cells(character.x, character.y, x, y),
                GroundKind::Pneuma => vec![(x, y)],
                GroundKind::Sanctuary => Self::sanctuary_cells(x, y),
                GroundKind::VenomDust => Self::venom_dust_cells(x, y),
                GroundKind::GrandCross | GroundKind::GrandDarkness => Self::grand_cross_cells(x, y),
                _ => Self::square_cells(x, y, layout.max(0) as u16),
            };
            let cells = locations
                .into_iter()
                .filter(|(x, y)| {
                    *x < instance.x_size()
                        && *y < instance.y_size()
                        && instance.state().cells()[*y as usize * instance.x_size() as usize + *x as usize] & CellType::Shootable.as_flag()
                            != 0
                })
                .map(|(x, y)| GroundCell {
                    observers: Default::default(),
                    id: NEXT_GROUND_UNIT.fetch_add(1, Ordering::Relaxed),
                    x,
                    y,
                    remaining_hits: if kind.trap() {
                        3500
                    } else if kind == GroundKind::Firewall {
                        4 + level as u16
                    } else if matches!(kind, GroundKind::GrandCross | GroundKind::GrandDarkness) {
                        3
                    } else {
                        u16::MAX
                    },
                })
                .collect::<Vec<_>>();
            if cells.is_empty() {
                continue;
            }
            active.push(GroundSkill {
                portal: None,
                message: if matches!(kind, GroundKind::TalkieBox | GroundKind::Graffiti) {
                    character.script_skill_state.ground_skill_text.clone()
                } else {
                    vec![]
                },
                capture: None,
                recovery_item: kind
                    .trap()
                    .then(|| self.configuration.find_item_by_name("Booby_Trap").map(|item| item.id))
                    .flatten(),
                kind,
                source_id: character.char_id,
                source_x: character.x,
                source_y: character.y,
                map: character.current_map_name().clone(),
                instance: character.current_map_instance(),
                skill_id,
                level,
                depth,
                active_from: active_from + offset,
                expires_at: active_from + offset + if kind == GroundKind::Meteor { 100 } else { duration },
                next_hit_at: active_from
                    + offset
                    + if matches!(kind, GroundKind::GrandCross | GroundKind::GrandDarkness | GroundKind::Earthquake | GroundKind::StormGust) {
                        100
                    } else {
                        0
                    },
                interval,
                effect_range: range,
                displayed: false,
                skill_event_emitted: instant || number > 0,
                cast_generation,
                cast_finish_at: active_from,
                cast_verified: instant,
                cells,
                affected: HashSet::new(),
                actor_source: (kind.trap() || kind == GroundKind::Graffiti).then(|| GroundSkillSource {
                    actor_id: character.char_id,
                    owner_id: character.char_id,
                    map: character.current_map_name().clone(),
                    instance: character.current_map_instance(),
                    x: character.x,
                    y: character.y,
                    status: snapshot.clone(),
                    raw_attack: 0,
                    fixed_damage: None,
                }),
                triggered: false,
                waves: 0,
            });
        }
        drop(active);
        if !instant {
            let payment = character
                .script_skill_state
                .deferred_requirements
                .take()
                .filter(|payment| payment.skill_id == skill_id && payment.level == level);
            let effect = super::ScriptSkillEffect {
                source_char_id: character.char_id,
                target_id: character.char_id,
                skill_id,
                level,
                heal_value: 0,
                proc_depth: depth,
                skill_event_emitted: true,
                cast_generation,
                action: super::ScriptSkillAction::ActivateGround { skill_id, cast_generation },
                deferred_requirements: payment.as_ref().map(|payment| payment.requirements.clone()),
                prepared_outcome: None,
                source_index: payment.as_ref().and_then(|payment| payment.source_index),
                source_item: payment.and_then(|payment| payment.source_item),
            };
            server.add_to_tick(
                GameEvent::CharacterScriptSkill(effect),
                cast_time.div_ceil(40).max(1).saturating_sub(1).min(usize::MAX as u128) as usize,
            );
        }
        Ok(())
    }

    pub fn tick_ground_skills(&self, server: &Server, state: &ServerState, tick: u128) {
        self.retry_pending_notifications();
        if let Ok(mut sequences) = self.water_ball_sequences.lock() {
            sequences.retain(|source_id, sequence| {
                sequence.expires_at > tick
                    && state.get_character(*source_id).is_some_and(|source| {
                        source.status.hp > 0
                            && source.current_map_name() == &sequence.map
                            && source.current_map_instance() == sequence.instance
                    })
            });
        }
        let Ok(mut grounds) = self.ground_skills.lock() else {
            return;
        };
        for ground in grounds.iter_mut() {
            if ground.kind == GroundKind::WarpPortal {
                if !self.warp_portal_owner_current(state, ground) {
                    ground.expires_at = 0;
                }
                continue;
            }
            if !ground.cast_verified
                && !state.get_character(ground.source_id).is_some_and(|source| {
                    source.status.hp > 0
                        && (ground.cast_generation == 0 || source.script_skill_state.cast_generation == ground.cast_generation)
                })
            {
                ground.expires_at = 0;
            }
            if let Some(source) = &ground.actor_source {
                let map_actor = state.get_map_instance(&source.map, source.instance).is_some_and(|instance| {
                    let map = instance.state();
                    map.get_mob(source.actor_id)
                        .is_some_and(|actor| actor.hp() > 0 || ground.kind == GroundKind::AnkleSnare)
                        || map
                            .script_skill_state
                            .npcs
                            .get(&source.actor_id)
                            .is_some_and(|actor| actor.hp > 0 || ground.kind == GroundKind::AnkleSnare)
                });
                if !map_actor
                    && !state.get_character(source.owner_id).is_some_and(|owner| {
                        owner.current_map_name() == &ground.map
                            && owner.current_map_instance() == ground.instance
                            && (owner.char_id == source.actor_id && (owner.status.hp > 0 || ground.kind == GroundKind::AnkleSnare)
                                || crate::server::service::script_world_service::companion_snapshots(owner)
                                    .iter()
                                    .any(|actor| actor.map_item().id() == ground.source_id))
                    })
                {
                    ground.expires_at = 0;
                }
            }
            if ground.kind == GroundKind::BlastMine
                && ground.cast_verified
                && !ground.triggered
                && ground.expires_at != 0
                && ground.expires_at <= tick
            {
                ground.triggered = true;
                ground.recovery_item = None;
                ground.expires_at = tick + 1500;
                self.change_trap_view(ground, 140);
            }
            if ground.capture.is_some() {
                self.tick_trap_capture(server, state, ground, tick);
            }
            if ground.actor_source.is_none()
                && !state.get_character(ground.source_id).is_some_and(|source| {
                    source.current_map_name() == &ground.map
                        && source.current_map_instance() == ground.instance
                        && (ground.cast_verified
                            || source.status.hp > 0
                                && (ground.cast_generation == 0 || source.script_skill_state.cast_generation == ground.cast_generation))
                })
            {
                ground.expires_at = 0;
            }
        }
        let protected = grounds
            .iter()
            .filter(|ground| {
                ground.cast_verified && ground.kind == GroundKind::LandProtector && ground.active_from <= tick && ground.expires_at > tick
            })
            .flat_map(|ground| {
                ground
                    .cells
                    .iter()
                    .filter(|cell| cell.remaining_hits > 0)
                    .map(move |cell| (ground.map.clone(), ground.instance, cell.x, cell.y))
            })
            .collect::<HashSet<_>>();
        for ground in grounds
            .iter_mut()
            .filter(|ground| ground.kind != GroundKind::LandProtector && ground.active_from <= tick && ground.expires_at > tick)
        {
            if SkillMetadata::find(ground.skill_id)
                .is_some_and(|metadata| metadata.flags.get("IgnoreLandProtector").copied().unwrap_or(false))
            {
                continue;
            }
            for cell in &mut ground.cells {
                if protected.contains(&(ground.map.clone(), ground.instance, cell.x, cell.y)) {
                    cell.remaining_hits = 0;
                }
            }
        }
        let coverage = grounds
            .iter()
            .filter(|ground| {
                ground.cast_verified && ground.active_from <= tick && ground.expires_at > tick && ground.kind.status().is_some()
            })
            .map(|ground| {
                (
                    ground.map.clone(),
                    ground.instance,
                    ground.kind,
                    ground.effect_range,
                    ground
                        .cells
                        .iter()
                        .filter(|cell| cell.remaining_hits > 0)
                        .map(|cell| (cell.x, cell.y))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        for ground in grounds.iter_mut() {
            let Some(instance) = state.get_map_instance(&ground.map, ground.instance) else {
                ground.expires_at = 0;
                continue;
            };
            if tick < ground.active_from || !ground.cast_verified {
                continue;
            }
            let source = state.get_character(ground.source_id);
            if ground.expires_at > tick && !ground.displayed {
                ground.displayed = true;
                if !ground.skill_event_emitted {
                    crate::server::service::script_combat_service::emit(
                        server,
                        ground.source_id,
                        ground.source_id,
                        CombatTrigger::Skill,
                        BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                        ground.skill_id,
                        0,
                    );
                    ground.skill_event_emitted = true;
                }
            }
            if ground.kind == GroundKind::WarpPortal {
                self.tick_warp_portal(server, state, ground, tick);
                continue;
            }
            if ground.kind == GroundKind::TalkieBox {
                self.tick_talkie_box(state, ground, tick);
                continue;
            }
            if ground.kind == GroundKind::Graffiti {
                continue;
            }
            if let Some(kind) = ground.kind.status() {
                let map_state = instance.state();
                let mobs = map_state
                    .mobs()
                    .values()
                    .filter(|mob| mob.status.hp() > 0)
                    .map(|mob| (mob.id, mob.x, mob.y))
                    .collect::<Vec<_>>();
                let players = state
                    .characters()
                    .values()
                    .filter(|target| {
                        target.current_map_name() == &ground.map && target.current_map_instance() == ground.instance && target.status.hp > 0
                    })
                    .map(|target| (target.char_id, target.x, target.y));
                let mut affected = HashSet::new();
                for (target_id, x, y) in mobs.into_iter().chain(players) {
                    if tick >= ground.expires_at || !ground.covers(x, y) {
                        continue;
                    }
                    if kind == StatusChangeKind::Quagmire && ground.actor_source.is_none() && state.get_character(target_id).is_some() {
                        continue;
                    }
                    if kind == StatusChangeKind::SafetyWall && ground.affected.contains(&target_id) && tick > ground.active_from + SAFETY_WALL_APPLY_GRACE_MS {
                        let still_protected = state
                            .get_character(target_id)
                            .map(|target| target.status.has_status_change(kind))
                            .or_else(|| map_state.get_mob(target_id).map(|mob| mob.status_effects.has_status_change(kind)))
                            .unwrap_or(true);
                        if !still_protected {
                            ground.expires_at = tick;
                            continue;
                        }
                    }
                    affected.insert(target_id);
                    if !ground.affected.contains(&target_id) {
                        let mut request = StatusChangeRequest::guaranteed(
                            kind,
                            ground.expires_at.saturating_sub(tick).saturating_add(200).min(i32::MAX as u128) as i32,
                            ground.level as i32,
                        );
                        request.values[3] = ground.cells.first().map(|cell| cell.id).unwrap_or(0) as i32;
                        if state.get_character(target_id).is_some() {
                            server.add_to_next_tick(GameEvent::CharacterStatusChange(
                                crate::server::model::events::game_event::CharacterStatusChange {
                                    char_id: target_id,
                                    request,
                                },
                            ));
                        } else {
                            instance.add_to_next_tick(MapEvent::MobStatusChange {
                                mob_id: target_id,
                                request,
                            });
                        }
                    }
                }
                for target_id in ground.affected.difference(&affected) {
                    let location = state
                        .get_character(*target_id)
                        .map(|target| (target.x, target.y))
                        .or_else(|| map_state.get_mob(*target_id).map(|mob| (mob.x, mob.y)));
                    let covered_elsewhere = location.is_some_and(|(x, y)| {
                        coverage.iter().any(|(map, instance, other_kind, range, cells)| {
                            map == &ground.map
                                && *instance == ground.instance
                                && *other_kind == ground.kind
                                && cells.iter().any(|(cx, cy)| cx.abs_diff(x).max(cy.abs_diff(y)) <= *range)
                        })
                    });
                    if covered_elsewhere {
                        continue;
                    }
                    if state.get_character(*target_id).is_some() {
                        server.add_to_next_tick(GameEvent::CharacterEndStatus(
                            crate::server::model::events::game_event::CharacterEndStatus {
                                char_id: *target_id,
                                kind: Some(kind),
                            },
                        ));
                    } else if kind != StatusChangeKind::Quagmire {
                        instance.add_to_next_tick(MapEvent::MobEndStatus {
                            mob_id: *target_id,
                            kind: Some(kind),
                        });
                    }
                }
                ground.affected = affected;
            }
            if ground.kind == GroundKind::Sanctuary {
                self.tick_sanctuary(server, state, ground, tick);
                continue;
            }
            if ground.kind == GroundKind::VenomDust {
                self.tick_venom_dust(server, state, ground, tick);
                continue;
            }
            if ground.kind == GroundKind::SpiderWeb {
                self.tick_spider_web(server, state, ground, tick);
                continue;
            }
            if ground.actor_source.is_some() {
                self.tick_actor_ground_skill(server, state, ground, tick);
                continue;
            }
            if ground.expires_at <= tick || !ground.kind.damaging() || tick < ground.next_hit_at {
                continue;
            }
            let limit = match ground.kind {
                GroundKind::Earthquake | GroundKind::GrandCross => 3,
                GroundKind::StormGust => 10,
                _ => u8::MAX,
            };
            if ground.waves >= limit {
                continue;
            }
            let Some(source) = source else {
                continue;
            };
            let skill = skills::skill_enums::to_object(SkillEnum::from_id(ground.skill_id), ground.level);
            let offensive = skill.as_ref().and_then(|skill| skill.as_offensive_skill());
            if offensive.is_none() && !matches!(ground.kind, GroundKind::Earthquake | GroundKind::GrandCross) {
                continue;
            }
            let snapshot = StatusService::instance().to_snapshot(&source.status);
            let map_state = instance.state();
            let targets = map_state
                .mobs()
                .values()
                .filter(|mob| {
                    mob.status.hp() > 0
                        && (!mob.summoned || mob.summon_ai == 0)
                        && Self::area_skill_target_allowed(&snapshot, &mob.status, ground.skill_id)
                        && ground.covers(mob.x, mob.y)
                })
                .map(|mob| (mob.id, mob.x, mob.y, mob.status.clone()))
                .collect::<Vec<_>>();
            let split = targets.len().max(1) as u32;
            for (target_id, x, y, target_status) in targets {
                if ground.kind == GroundKind::StormGust && target_status.has_status_change(StatusChangeKind::Freeze) {
                    continue;
                }
                let Some(cell) = ground
                    .cells
                    .iter_mut()
                    .find(|cell| cell.remaining_hits > 0 && cell.x.abs_diff(x).max(cell.y.abs_diff(y)) <= ground.effect_range)
                else {
                    continue;
                };
                let (damage, magic_context) = if ground.kind == GroundKind::Earthquake {
                    let (raw, ratio) = Self::earthquake_attack(&snapshot, ground.level, split);
                    let context = crate::server::service::map_combat_service::MagicAttackContext::new(
                        raw,
                        ratio,
                        Element::Neutral,
                        1,
                        ground.skill_id,
                    );
                    (
                        server
                            .battle_service()
                            .magic_damage_from_context(&snapshot, &target_status, context),
                        Some(context),
                    )
                } else if ground.kind == GroundKind::GrandCross {
                    let (amount, context) =
                        server
                            .battle_service()
                            .grand_cross_damage_signed_with_context(&snapshot, &target_status, ground.level, false);
                    (amount, Some(context))
                } else {
                    server
                        .battle_service()
                        .calculate_damage_with_context(&snapshot, &target_status, offensive)
                };
                let mut damage_event = Damage {
                    notification: None,
                    source_kind: models::enums::actor::CombatActorKind::Player,
                    skill_damage_adjusted: false,
                    healing: 0,
                    right_hand_damage: None,
                    target_id,
                    attacker_id: ground.source_id,
                    damage: 0,
                    attacked_at: tick,
                    damage_motion: 0,
                    battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                    skill_id: ground.skill_id,
                    skill_level: ground.level,
                    proc_depth: ground.depth,
                    credit_id: ground.source_id,
                    defenses_applied: true,
                    magic_context,
                    landed: true,
                };
                damage_event.set_signed_damage(damage);
                damage_event = damage_event.with_skill_notification(
                    source.current_map_name(),
                    source.current_map_instance(),
                    source.x,
                    source.y,
                    tick,
                    1,
                    0,
                );
                instance.add_to_next_tick(MapEvent::MobDamage(damage_event));
                if ground.kind == GroundKind::Firewall {
                    cell.remaining_hits = cell.remaining_hits.saturating_sub(1);
                    if *target_status.element() != Element::Fire
                        && *target_status.element() != Element::Undead
                        && *target_status.race() != MobRace::RUndead
                    {
                        instance.add_to_next_tick(MapEvent::MobKnockback {
                            mob_id: target_id,
                            source_x: ground.source_x,
                            source_y: ground.source_y,
                            cells: 2,
                        });
                    }
                } else if ground.kind == GroundKind::GrandCross {
                    cell.remaining_hits = cell.remaining_hits.saturating_sub(1);
                } else if ground.kind == GroundKind::StormGust {
                    instance.add_to_next_tick(MapEvent::MobKnockback {
                        mob_id: target_id,
                        source_x: ground.source_x,
                        source_y: ground.source_y,
                        cells: 2,
                    });
                }
            }
            if ground.kind == GroundKind::GrandCross && source.status.hp > 0 {
                if let Some(cell) = ground
                    .cells
                    .iter_mut()
                    .find(|cell| cell.remaining_hits > 0 && cell.x == source.x && cell.y == source.y)
                {
                    let (damage, context) =
                        server
                            .battle_service()
                            .grand_cross_damage_signed_with_context(&snapshot, &snapshot, ground.level, true);
                    let mut damage_event = Damage {
                        notification: None,
                        source_kind: models::enums::actor::CombatActorKind::Player,
                        skill_damage_adjusted: false,
                        healing: 0,
                        right_hand_damage: None,
                        target_id: source.char_id,
                        attacker_id: source.char_id,
                        damage: 0,
                        attacked_at: tick,
                        damage_motion: 0,
                        battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag(),
                        skill_id: ground.skill_id,
                        skill_level: ground.level,
                        proc_depth: ground.depth,
                        credit_id: source.char_id,
                        defenses_applied: true,
                        magic_context: Some(context),
                        landed: true,
                    };
                    damage_event.set_signed_damage(damage);
                    damage_event = damage_event.with_skill_notification(
                        source.current_map_name(),
                        source.current_map_instance(),
                        source.x,
                        source.y,
                        tick,
                        1,
                        0,
                    );
                    server.add_to_next_tick(GameEvent::CharacterDamage(damage_event));
                    cell.remaining_hits = cell.remaining_hits.saturating_sub(1);
                }
            }
            ground.next_hit_at = ground.next_hit_at.saturating_add(ground.interval);
            ground.waves = ground.waves.saturating_add(1);
        }
        grounds.retain_mut(|ground| {
            let expired = tick >= ground.expires_at;
            if !expired {
                self.sync_ground_unit_visibility(state, ground, tick);
            }
            if expired
                && ground.expires_at != 0
                && ground.cast_verified
                && !ground.triggered
                && ground.cells.iter().any(|cell| cell.remaining_hits > 0)
            {
                if let Some(item_id) = ground.recovery_item.take() {
                    if let Some(map) = state.get_map_instance(&ground.map, ground.instance) {
                        map.add_to_next_tick(MapEvent::GroundTrapRecover {
                            item_id,
                            amount: 1,
                            x: ground.cells[0].x,
                            y: ground.cells[0].y,
                        });
                    }
                }
            }
            if expired || ground.cells.iter().all(|cell| cell.remaining_hits == 0) {
                self.release_trap_capture(server, ground);
            }
            for cell in &ground.cells {
                if cell.id != 0 && ground.displayed && (expired || cell.remaining_hits == 0) {
                    let mut packet = 0x0120_u16.to_le_bytes().to_vec();
                    packet.extend_from_slice(&cell.id.to_le_bytes());
                    self.notify_ground_cell(ground, cell, packet);
                }
            }
            for cell in &mut ground.cells {
                if expired || cell.remaining_hits == 0 {
                    cell.id = 0;
                }
            }
            !expired && ground.cells.iter().any(|cell| cell.remaining_hits > 0)
        });
    }

    pub fn ground_field_contains(&self, character: &Character, kind: GroundKind, x: u16, y: u16, tick: u128) -> bool {
        self.ground_skills.lock().is_ok_and(|grounds| {
            grounds.iter().any(|ground| {
                ground.kind == kind
                    && ground.map == *character.current_map_name()
                    && ground.instance == character.current_map_instance()
                    && ground.active_from <= tick
                    && ground.expires_at > tick
                    && ground.covers(x, y)
            })
        })
    }

    pub fn earthquake_attack(source: &StatusSnapshot, level: u8, targets: u32) -> (u16, f32) {
        let raw = (i64::from(source.fist_atk()) + i64::from(source.weapon_atk()) + i64::from(source.bonus_atk()))
            .clamp(0, i64::from(u32::MAX)) as u32;
        let level = u32::from(level);
        let ratio = 200 + 100 * level + 100 * (level / 2) + if level > 4 { 100 } else { 0 };
        (
            raw.min(u32::from(u16::MAX)) as u16,
            ratio as f32 / 100.0 / targets.max(1) as f32,
        )
    }

    pub(super) fn notify_ground_cell(&self, ground: &GroundSkill, cell: &GroundCell, packet: Vec<u8>) {
        if let Err(error) = self.client_notification_sender.try_send(Notification::Area(AreaNotification::new(
            ground.map.clone(),
            ground.instance,
            AreaNotificationRangeType::Fov {
                x: cell.x,
                y: cell.y,
                exclude_id: None,
            },
            packet,
        ))) {
            warn!("Unable to notify ground skill: {}", error);
        }
    }

    pub fn square_cells(x: u16, y: u16, radius: u16) -> Vec<(u16, u16)> {
        let radius = radius.min(32) as i32;
        (-radius..=radius)
            .flat_map(|dy| {
                (-radius..=radius).filter_map(move |dx| Some((u16::try_from(x as i32 + dx).ok()?, u16::try_from(y as i32 + dy).ok()?)))
            })
            .collect()
    }

    pub fn venom_dust_cells(x: u16, y: u16) -> Vec<(u16, u16)> {
        Self::square_cells(x, y, 1)
            .into_iter()
            .filter(|(cx, cy)| cx.abs_diff(x) + cy.abs_diff(y) <= 1)
            .collect()
    }

    pub fn sanctuary_cells(x: u16, y: u16) -> Vec<(u16, u16)> {
        Self::square_cells(x, y, 2)
            .into_iter()
            .filter(|(cx, cy)| !(cx.abs_diff(x) == 2 && cy.abs_diff(y) == 2))
            .collect()
    }

    pub fn grand_cross_cells(x: u16, y: u16) -> Vec<(u16, u16)> {
        let dx = [
            0, 0, -1, 0, 1, -2, -1, 0, 1, 2, -4, -3, -2, -1, 0, 1, 2, 3, 4, -2, -1, 0, 1, 2, -1, 0, 1, 0, 0,
        ];
        let dy = [
            -4, -3, -2, -2, -2, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 2, 3, 4,
        ];
        dx.into_iter()
            .zip(dy)
            .filter_map(|(dx, dy)| Some((u16::try_from(x as i32 + dx).ok()?, u16::try_from(y as i32 + dy).ok()?)))
            .collect()
    }

    pub fn firewall_cells(source_x: u16, source_y: u16, x: u16, y: u16) -> Vec<(u16, u16)> {
        let dx = x as i32 - source_x as i32;
        let dy = y as i32 - source_y as i32;
        let offsets: &[(i32, i32)] = if dx.abs() > dy.abs() * 2 {
            &[(0, -1), (0, 0), (0, 1)]
        } else if dy.abs() > dx.abs() * 2 {
            &[(-1, 0), (0, 0), (1, 0)]
        } else if dx.signum() == dy.signum() {
            &[(-1, 1), (-1, 0), (0, 0), (0, -1), (1, -1)]
        } else {
            &[(1, 1), (1, 0), (0, 0), (0, -1), (-1, -1)]
        };
        offsets
            .iter()
            .filter_map(|(dx, dy)| Some((u16::try_from(x as i32 + dx).ok()?, u16::try_from(y as i32 + dy).ok()?)))
            .collect()
    }

    pub fn ground_entry_packet(packetver: u32, id: u32, source_id: u32, x: u16, y: u16, level: u8) -> Vec<u8> {
        Self::ground_entry_packet_for(packetver, id, source_id, x, y, level, 127)
    }

    pub fn ground_entry_packet_for(packetver: u32, id: u32, source_id: u32, x: u16, y: u16, level: u8, unit_view: u32) -> Vec<u8> {
        let modern = packetver >= 20130731;
        let version_three = !modern && packetver > 20120702;
        let mut packet = (if modern {
            0x09CA_u16
        } else if version_three {
            0x099F_u16
        } else {
            0x011F_u16
        })
        .to_le_bytes()
        .to_vec();
        if modern || version_three {
            packet.extend_from_slice(&23_u16.to_le_bytes());
        }
        packet.extend_from_slice(&id.to_le_bytes());
        packet.extend_from_slice(&source_id.to_le_bytes());
        packet.extend_from_slice(&x.to_le_bytes());
        packet.extend_from_slice(&y.to_le_bytes());
        if modern || version_three {
            packet.extend_from_slice(&unit_view.to_le_bytes());
        } else {
            packet.push(unit_view as u8);
        }
        if modern {
            packet.extend_from_slice(&[0, 1, level]);
        } else if version_three {
            packet.extend_from_slice(&0_u16.to_le_bytes());
            packet.push(1);
        } else {
            packet.push(1);
        }
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn firewall_layout_rotates_and_clips_map_edges() {
        assert_eq!(ScriptSkillService::firewall_cells(5, 5, 10, 5), vec![(10, 4), (10, 5), (10, 6)]);
        assert_eq!(ScriptSkillService::firewall_cells(5, 5, 5, 10), vec![(4, 10), (5, 10), (6, 10)]);
        assert_eq!(ScriptSkillService::firewall_cells(5, 5, 10, 10).len(), 5);
    }
    #[test]
    fn grand_cross_uses_the_classic_twenty_nine_cells() {
        let cells = ScriptSkillService::grand_cross_cells(10, 10);
        assert_eq!(cells.len(), 29);
        assert!(cells.contains(&(14, 10)) && cells.contains(&(10, 6)) && cells.contains(&(12, 11)));
        assert!(!cells.contains(&(12, 12)));
    }
    #[test]
    fn sanctuary_covers_a_five_by_five_square_without_corners() {
        let cells = ScriptSkillService::sanctuary_cells(10, 10);
        assert_eq!(cells.len(), 21);
        assert!(cells.contains(&(12, 11)) && cells.contains(&(11, 8)));
        assert!(!cells.contains(&(12, 12)) && !cells.contains(&(8, 8)));
        assert_eq!(GroundKind::Sanctuary.effect_range(1), 0);
        let venom = ScriptSkillService::venom_dust_cells(10, 10);
        assert_eq!(venom.len(), 5);
        assert!(venom.contains(&(9, 10)) && !venom.contains(&(9, 9)));
    }
    #[test]
    fn ground_packets_follow_the_selected_client_version() {
        assert_eq!(ScriptSkillService::ground_entry_packet(20100101, 55, 88, 10, 12, 5).len(), 16);
        let modern = ScriptSkillService::ground_entry_packet_for(20130807, 55, 88, 10, 12, 5, 157);
        assert_eq!(modern.len(), 23);
        assert_eq!(u32::from_le_bytes(modern[16..20].try_into().unwrap()), 157);
        assert_eq!(modern[22], 5);
    }
}
