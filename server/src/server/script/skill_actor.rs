use std::collections::HashMap;
use std::sync::mpsc::SyncSender;

use models::enums::cell::CellType;
use models::enums::element::Element;
use models::enums::mob::{MobMode, MobRace};
use models::enums::size::Size;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::{Status, StatusSnapshot};
use models::status_change::StatusChangeKind;

use super::metadata::SkillMetadata;
use super::requirements::DeferredSkillPayment;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::ScriptSkillCast;
use crate::server::model::map_item::{MapItemSnapshot, MapItemType};
use crate::server::model::script::Script;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::visibility_service::{StealthState, TargetingMode, VisibilityObserver, can_target};
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::mob::Mob;

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptSkillActor {
    pub id: u32,
    pub credit_id: u32,
    pub object_type: MapItemType,
    pub map: String,
    pub instance: u8,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
    pub status: StatusSnapshot,
    pub raw_attack: u32,
    pub mode: u32,
    pub attack_motion: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapActorSkillCast {
    pub request: ScriptSkillCast,
    pub source: ScriptSkillActor,
    pub target: Option<MapItemSnapshot>,
    pub npc: Option<NpcSkillState>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptActorSkillCompletion {
    pub request: ScriptSkillCast,
    pub source: ScriptSkillActor,
    pub generation: u64,
    pub payment: Option<DeferredSkillPayment>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NpcSkillState {
    pub id: u32,
    pub sprite: u16,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
    pub level: u32,
    pub stat_point: u16,
    pub parameters: [u16; 6],
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub size: Size,
    pub speed: u16,
    pub attack_min: u16,
    pub attack_max: u16,
}

impl NpcSkillState {
    pub fn new(script: &Script) -> Self {
        Self {
            id: script.id,
            sprite: script.sprite,
            x: script.x,
            y: script.y,
            dir: script.dir,
            level: 0,
            stat_point: 0,
            parameters: [0; 6],
            hp: 1,
            sp: 1,
            max_hp: 1,
            max_sp: 1,
            size: Size::Small,
            speed: 0,
            attack_min: 0,
            attack_max: 0,
        }
    }

    pub fn snapshot(&self) -> StatusSnapshot {
        let [str, agi, vit, int, dex, luk] = self.parameters.map(|value| value.saturating_add(self.stat_point));
        let mut snapshot = StatusSnapshot::new_for_mob(
            u32::from(self.sprite),
            self.hp,
            self.sp,
            self.max_hp,
            self.max_sp,
            str,
            agi,
            vit,
            int,
            dex,
            luk,
            0,
            0,
            (u32::from(int) + u32::from(int / 7).pow(2)).min(u32::from(u16::MAX)) as u16,
            (u32::from(int) + u32::from(int / 5).pow(2)).min(u32::from(u16::MAX)) as u16,
            self.speed,
            0,
            0,
            self.size,
            Element::Neutral,
            MobRace::DemiHuman,
            1,
        );
        snapshot.set_base_level(self.level);
        snapshot.set_hit((self.level + u32::from(dex)).clamp(1, i16::MAX as u32) as i16);
        snapshot.set_flee((self.level + u32::from(agi)).clamp(1, i16::MAX as u32) as i16);
        snapshot
    }

    pub fn actor(&self, map: String, instance: u8) -> ScriptSkillActor {
        ScriptSkillActor {
            id: self.id,
            credit_id: self.id,
            object_type: MapItemType::Npc,
            map,
            instance,
            x: self.x,
            y: self.y,
            dir: self.dir,
            status: self.snapshot(),
            raw_attack: u32::from(fastrand::u16(
                self.attack_min.min(self.attack_max)..=self.attack_min.max(self.attack_max),
            )),
            mode: MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag(),
            attack_motion: 0,
        }
    }
}

impl ScriptSkillActor {
    pub fn from_mob(mob: &Mob, map: String, instance: u8) -> Self {
        let raw_attack = if mob.status.has_status_change(StatusChangeKind::MaximizePower) {
            mob.atk1.max(mob.atk2)
        } else {
            fastrand::u16(mob.atk1.min(mob.atk2)..=mob.atk1.max(mob.atk2))
        };
        Self {
            id: mob.id,
            credit_id: mob.summon_owner.unwrap_or(mob.id),
            object_type: MapItemType::Mob,
            map,
            instance,
            x: mob.x,
            y: mob.y,
            dir: mob.dir,
            status: mob.status.clone(),
            raw_attack: u32::from(raw_attack),
            mode: mob.mode,
            attack_motion: mob.atk_motion,
        }
    }

    pub fn can_cast(&self, skill_id: u32) -> bool {
        let status = Status {
            hp: self.status.hp(),
            active_statuses: self.status.active_statuses().clone(),
            ..Status::default()
        };
        status.hp > 0 && !status.blocks_casting() && super::ScriptSkillService::player_skill_source_allowed(&status, skill_id)
    }

    pub fn observer(&self) -> VisibilityObserver {
        if self.object_type == MapItemType::Mob {
            VisibilityObserver::for_mob(self.mode, *self.status.race())
        } else {
            VisibilityObserver::player(&self.status)
        }
    }
}

#[derive(Default)]
pub struct MapSkillState {
    pub npcs: HashMap<u32, NpcSkillState>,
    pub casts: HashMap<u32, ActiveActorCast>,
    pub generations: HashMap<u32, u64>,
    next_generation: u64,
}

#[derive(Clone, Debug)]
pub struct ActiveActorCast {
    pub cast: MapActorSkillCast,
    pub generation: u64,
    pub finish_at: u128,
    pub cancelable: bool,
}

pub fn map_actor(state: &MapInstanceState, actor_id: u32) -> Option<ScriptSkillActor> {
    let map = state.key().map_name().clone();
    let instance = state.key().map_instance();
    state
        .get_mob(actor_id)
        .filter(|mob| mob.is_present())
        .map(|mob| ScriptSkillActor::from_mob(mob, map.clone(), instance))
        .or_else(|| {
            state
                .script_skill_state
                .npcs
                .get(&actor_id)
                .filter(|npc| {
                    state
                        .get_map_item(actor_id)
                        .is_some_and(|item| *item.object_type() == MapItemType::Npc)
                })
                .map(|npc| npc.actor(map, instance))
        })
}

pub fn actor_cast_time(source: &ScriptSkillActor, request: &ScriptSkillCast) -> Result<u128, String> {
    let metadata = SkillMetadata::find(request.skill_id).ok_or("Unit skill has no pre-renewal definition")?;
    let modifier = if metadata.cast_time_flags.get("IgnoreDex").copied().unwrap_or(false) {
        1.0
    } else {
        (1.0 - f32::from(source.status.dex()) / 150.0).max(0.0)
    };
    let duration = metadata.cast_duration(request.level as u8, modifier);
    Ok((duration.min(i64::MAX as u128) as i64 + i64::from(request.cast_time_adjust_ms)).max(0) as u128)
}

pub fn actor_skill_range(source: &ScriptSkillActor, metadata: &SkillMetadata, level: u8, ground: bool) -> u16 {
    if source.object_type == MapItemType::Npc && ground {
        return 15;
    }
    let mut range = metadata.range(level).unwrap_or(0).unsigned_abs().min(14) as u16;
    if range == 0 { 9 } else { range }
}

pub fn usable_terrain(state: &MapInstanceState, from: (u16, u16), to: (u16, u16)) -> bool {
    let shootable = |x: i32, y: i32| {
        x >= 0
            && y >= 0
            && x < i32::from(state.x_size())
            && y < i32::from(state.y_size())
            && state
                .cells()
                .get(y as usize * state.x_size() as usize + x as usize)
                .is_some_and(|cell| cell & CellType::Shootable.as_flag() != 0)
    };
    let (mut x, mut y) = (i32::from(from.0), i32::from(from.1));
    let (tx, ty) = (i32::from(to.0), i32::from(to.1));
    let (dx, dy) = ((tx - x).abs(), -(ty - y).abs());
    let (sx, sy) = ((tx - x).signum(), (ty - y).signum());
    let mut error = dx + dy;
    loop {
        if !shootable(x, y) {
            return false;
        }
        if x == tx && y == ty {
            return true;
        }
        let twice = 2 * error;
        if twice >= dy {
            error += dy;
            x += sx;
        }
        if twice <= dx {
            error += dx;
            y += sy;
        }
    }
}

fn target_position(state: &MapInstanceState, cast: &MapActorSkillCast, source: &ScriptSkillActor) -> Option<(u16, u16)> {
    if let Some(ground) = cast.request.ground {
        return Some(ground);
    }
    if cast.request.target_id == source.id {
        return Some((source.x, source.y));
    }
    if let Some(mob) = state.get_mob(cast.request.target_id).filter(|mob| mob.is_present()) {
        return Some((mob.x, mob.y));
    }
    if let Some(npc) = state.script_skill_state.npcs.get(&cast.request.target_id) {
        return Some((npc.x, npc.y));
    }
    state
        .characters()
        .iter()
        .find(|target| target.map_item().id() == cast.request.target_id)
        .map(|target| (target.x(), target.y()))
        .or_else(|| {
            cast.target
                .filter(|target| state.get_map_item(target.map_item().id()).is_some())
                .map(|target| (target.x(), target.y()))
        })
}

fn validate_map_cast(state: &MapInstanceState, cast: &MapActorSkillCast, source: &ScriptSkillActor, completed: bool) -> Result<(), String> {
    let metadata = SkillMetadata::find(cast.request.skill_id).ok_or("Unit skill has no pre-renewal definition")?;
    if cast.request.level == 0 || cast.request.level > u16::from(metadata.max_level) {
        return Err("Unit skill level is invalid".into());
    }
    if !source.can_cast(cast.request.skill_id) {
        return Err("Actor status prevents casting".into());
    }
    if metadata.flags.get("NoTargetSelf").copied().unwrap_or(false) && source.id == cast.request.target_id {
        return Err("This skill cannot target its caster".into());
    }
    if cast.request.message_id.is_some_and(|message| message > 0) && source.object_type != MapItemType::Mob {
        return Err("Unit skill messages require a monster caster".into());
    }
    let position = target_position(state, cast, source).ok_or("Unit skill target left the map")?;
    if position.0 >= state.x_size() || position.1 >= state.y_size() {
        return Err("Unit skill target is outside the map".into());
    }
    if cast.request.ground.is_some() && !usable_terrain(state, position, position) {
        return Err("Unit skill ground is unusable".into());
    }
    let checks_range = !cast.request.ignore_range && (source.object_type != MapItemType::Npc || cast.request.ground.is_some());
    if checks_range {
        let range = actor_skill_range(source, metadata, cast.request.level as u8, cast.request.ground.is_some());
        if source.x.abs_diff(position.0).max(source.y.abs_diff(position.1)) > range
            || !usable_terrain(state, (source.x, source.y), position)
        {
            return Err("Unit skill target is out of range or behind an obstacle".into());
        }
    }
    if let Some(target) = state.actor_visibility.get(&cast.request.target_id) {
        let hidden = metadata.flags.get("TargetHidden").copied().unwrap_or(false);
        let mode = if completed {
            TargetingMode::SkillCompletion { can_hit_hidden: hidden }
        } else if hidden {
            TargetingMode::DirectHiddenSkill
        } else {
            TargetingMode::Direct
        };
        if !can_target(source.observer(), *target, mode) {
            return Err("Unit skill cannot target this hidden actor".into());
        }
    }
    if let Some(target) = state.get_mob(cast.request.target_id) {
        if !can_target(
            source.observer(),
            StealthState::from_snapshot(&target.status),
            if completed {
                TargetingMode::SkillCompletion {
                    can_hit_hidden: metadata.flags.get("TargetHidden").copied().unwrap_or(false),
                }
            } else {
                TargetingMode::Direct
            },
        ) {
            return Err("Unit skill cannot target this hidden monster".into());
        }
    }
    Ok(())
}

pub fn notify_actor(sender: &SyncSender<Notification>, source: &ScriptSkillActor, packet: Vec<u8>) {
    let _ = sender.try_send(Notification::Area(AreaNotification::new(
        source.map.clone(),
        source.instance,
        AreaNotificationRangeType::Fov {
            x: source.x,
            y: source.y,
            exclude_id: None,
        },
        packet,
    )));
}

pub fn casting_packet(source: &ScriptSkillActor, request: &ScriptSkillCast, duration: u128) -> Vec<u8> {
    let packetver = GlobalConfigService::instance().packetver();
    let mut packet = (if packetver >= 20181212 {
        0x0B1A_u16
    } else if packetver >= 20091124 {
        0x07FB_u16
    } else {
        0x013E_u16
    })
    .to_le_bytes()
    .to_vec();
    packet.extend_from_slice(&source.id.to_le_bytes());
    packet.extend_from_slice(&if request.ground.is_some() { 0 } else { request.target_id }.to_le_bytes());
    let (x, y) = request.ground.unwrap_or((0, 0));
    packet.extend_from_slice(&x.to_le_bytes());
    packet.extend_from_slice(&y.to_le_bytes());
    packet.extend_from_slice(&(request.skill_id as u16).to_le_bytes());
    let element = SkillMetadata::find(request.skill_id)
        .and_then(|metadata| metadata.element(request.level as u8))
        .and_then(|element| <Element as models::enums::EnumWithStringValue>::try_from_string(element).ok())
        .unwrap_or(Element::Neutral);
    packet.extend_from_slice(&(element.value() as u32).to_le_bytes());
    packet.extend_from_slice(&(duration.min(i32::MAX as u128) as i32).to_le_bytes());
    if packetver >= 20091124 {
        packet.push(0);
    }
    if packetver >= 20181212 {
        packet.extend_from_slice(&0_u32.to_le_bytes());
    }
    packet
}

pub fn start_map_cast(
    state: &mut MapInstanceState,
    mut cast: MapActorSkillCast,
    tick: u128,
    sender: &SyncSender<Notification>,
) -> Result<(), String> {
    if let Some(npc) = cast.npc.take() {
        state.script_skill_state.npcs.entry(npc.id).or_insert(npc);
    }
    let source = map_actor(state, cast.request.source_id).ok_or("Unit skill source is not on this map")?;
    if state.script_skill_state.casts.contains_key(&source.id) {
        return Err("Actor is already casting".into());
    }
    validate_map_cast(state, &cast, &source, false)?;
    let duration = actor_cast_time(&source, &cast.request)?;
    state.script_skill_state.next_generation = state.script_skill_state.next_generation.wrapping_add(1).max(1);
    let generation = state.script_skill_state.next_generation;
    state.script_skill_state.generations.insert(source.id, generation);
    if let Some(mob) = state.mobs_mut().get_mut(&source.id) {
        mob.movements.clear();
        mob.script_cast_until = tick + duration.max(1);
        mob.timing.set_canattack_tick(tick + duration);
    }
    let cancelable = duration > 0
        && cast
            .request
            .cast_cancel
            .unwrap_or_else(|| SkillMetadata::find(cast.request.skill_id).unwrap().cast_cancel.unwrap_or(true));
    cast.source = source.clone();
    notify_actor(sender, &source, casting_packet(&source, &cast.request, duration));
    if let Some(message_id) = cast.request.message_id.filter(|id| *id > 0) {
        if let Some(packet) = mob_message_packet(state, source.id, message_id) {
            notify_actor(sender, &source, packet);
        }
    }
    state.script_skill_state.casts.insert(source.id, ActiveActorCast {
        cast,
        generation,
        finish_at: tick + duration,
        cancelable,
    });
    Ok(())
}

pub fn tick_map_casts(state: &mut MapInstanceState, tick: u128) -> Vec<ScriptActorSkillCompletion> {
    let ids = state.script_skill_state.casts.keys().copied().collect::<Vec<_>>();
    let mut completed = vec![];
    for id in ids {
        let Some(active) = state.script_skill_state.casts.get(&id) else {
            continue;
        };
        let source = map_actor(state, id);
        let invalid = source.as_ref().is_none_or(|source| {
            !source.can_cast(active.cast.request.skill_id) || (source.x, source.y) != (active.cast.source.x, active.cast.source.y)
        });
        if !invalid && active.finish_at > tick {
            continue;
        }
        let active = state.script_skill_state.casts.remove(&id).unwrap();
        if let Some(mob) = state.mobs_mut().get_mut(&id) {
            mob.script_cast_until = 0;
        }
        let Some(source) = source.filter(|_| !invalid) else {
            state.script_skill_state.generations.remove(&id);
            continue;
        };
        if validate_map_cast(state, &active.cast, &source, true).is_err() {
            state.script_skill_state.generations.remove(&id);
            continue;
        }
        completed.push(ScriptActorSkillCompletion {
            request: active.cast.request,
            source,
            generation: active.generation,
            payment: None,
        });
    }
    completed
}

pub fn interrupt_map_cast(state: &mut MapInstanceState, actor_id: u32, tick: u128, sender: &SyncSender<Notification>) -> bool {
    let Some(active) = state.script_skill_state.casts.get(&actor_id) else {
        return false;
    };
    if !active.cancelable || active.finish_at <= tick {
        return false;
    }
    let Some(source) = map_actor(state, actor_id) else {
        return false;
    };
    if super::ScriptSkillService::cast_interruption_protected(&source.status, &state.flags) {
        return false;
    }
    state.script_skill_state.casts.remove(&actor_id);
    state.script_skill_state.generations.remove(&actor_id);
    if let Some(mob) = state.mobs_mut().get_mut(&actor_id) {
        mob.script_cast_until = 0;
    }
    let mut packet = 0x01B9_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&actor_id.to_le_bytes());
    notify_actor(sender, &source, packet);
    true
}

fn mob_message_packet(state: &MapInstanceState, actor_id: u32, message_id: u16) -> Option<Vec<u8>> {
    #[derive(serde::Deserialize)]
    struct MobMessage {
        id: u16,
        text: String,
        color: u32,
    }
    static MESSAGES: std::sync::OnceLock<Vec<MobMessage>> = std::sync::OnceLock::new();
    let message = MESSAGES
        .get_or_init(|| serde_json::from_str(include_str!("actor_mob_messages.json")).expect("Invalid monster skill messages"))
        .iter()
        .find(|message| message.id == message_id)?;
    let name = state.get_mob(actor_id)?.name.split('#').next().unwrap_or_default();
    let text = format!("{name} : {}", message.text);
    let bytes = text.as_bytes();
    let length = bytes.len().min(499);
    let mut packet = 0x02C1_u16.to_le_bytes().to_vec();
    packet.extend_from_slice(&(13_u16 + length as u16).to_le_bytes());
    packet.extend_from_slice(&actor_id.to_le_bytes());
    let [red, green, blue, _] = message.color.to_le_bytes();
    packet.extend_from_slice(&u32::from_le_bytes([blue, green, red, 0]).to_le_bytes());
    packet.extend_from_slice(&bytes[..length]);
    packet.push(0);
    Some(packet)
}
