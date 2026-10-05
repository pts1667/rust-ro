use std::collections::HashMap;
use std::sync::OnceLock;

use models::status_bonus::BattleFlag;
use models::status_change::StatusChangeKind;
use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64};
use serde::Deserialize;

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::{GameEvent, ScriptSkillCast};
use crate::server::model::events::map_event::ScriptSpawn;
use crate::server::script::skill::ScriptSkillService;
use crate::server::script::skill::actor::{ScriptSkillActor, actor_cast_time};
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::mob::{Mob, MobAction};

const MOB_SKILLS_PATH: &str = "config/mob_skills.json";
const RECENT_ATTACK_WINDOW_MS: u128 = 1000;
const FRIEND_SEARCH_RANGE: u16 = 9;
const NEARBY_MOB_RANGE: u16 = 9;
const AFTER_SKILL_WINDOW_MS: u128 = 2000;

#[derive(Debug, Clone, Deserialize)]
pub struct MobSkillEntry {
    pub mob_id: i32,
    pub state: String,
    pub skill_id: u32,
    pub level: u16,
    pub rate: u32,
    pub cast_time: i64,
    pub delay: u128,
    pub cancelable: bool,
    pub target: String,
    pub condition: String,
    pub condition_value: String,
    pub values: Vec<String>,
}

#[derive(Default)]
pub struct MobSkillDatabase {
    by_mob: HashMap<i32, Vec<MobSkillEntry>>,
    boss: Vec<MobSkillEntry>,
    normal: Vec<MobSkillEntry>,
    all: Vec<MobSkillEntry>,
}

impl MobSkillDatabase {
    pub fn from_entries(entries: Vec<MobSkillEntry>) -> Self {
        let mut database = Self::default();
        for entry in entries {
            match entry.mob_id {
                -1 => database.boss.push(entry),
                -2 => database.normal.push(entry),
                -3 => database.all.push(entry),
                id => database.by_mob.entry(id).or_default().push(entry),
            }
        }
        database
    }

    pub fn instance() -> &'static Self {
        static DATABASE: OnceLock<MobSkillDatabase> = OnceLock::new();
        DATABASE.get_or_init(|| match std::fs::read_to_string(MOB_SKILLS_PATH) {
            Ok(content) => match serde_json::from_str::<Vec<MobSkillEntry>>(&content) {
                Ok(entries) => Self::from_entries(entries),
                Err(error) => {
                    error!("Invalid {MOB_SKILLS_PATH}: {error}");
                    Self::default()
                }
            },
            Err(_) => {
                warn!("{MOB_SKILLS_PATH} is missing; monsters will not use skills");
                Self::default()
            }
        })
    }

    fn entries_for<'a>(&'a self, mob: &'a Mob) -> impl Iterator<Item = &'a MobSkillEntry> {
        let boss = mob.mode & models::enums::mob::MobMode::Boss.as_flag() != 0;
        self.by_mob
            .get(&i32::from(mob.mob_id))
            .into_iter()
            .flatten()
            .chain(if boss { self.boss.iter() } else { self.normal.iter() })
            .chain(self.all.iter())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MobSkillState {
    Idle,
    Walk,
    Chase,
    Attack,
}

fn state_matches(entry_state: &str, state: MobSkillState, angry: bool) -> bool {
    match entry_state {
        "any" => true,
        "idle" => state == MobSkillState::Idle,
        "walk" => state == MobSkillState::Walk,
        "chase" => state == MobSkillState::Chase,
        "attack" => state == MobSkillState::Attack,
        "angry" => state == MobSkillState::Attack && angry,
        "follow" => state == MobSkillState::Chase && angry,
        "anytarget" => matches!(state, MobSkillState::Chase | MobSkillState::Attack),
        _ => false,
    }
}

fn mob_state(mob: &Mob) -> Option<MobSkillState> {
    match mob.action {
        MobAction::Idle => Some(MobSkillState::Idle),
        MobAction::Moving => Some(MobSkillState::Walk),
        MobAction::Chasing { .. } => Some(MobSkillState::Chase),
        MobAction::Attacking { .. } => Some(MobSkillState::Attack),
        MobAction::Flinching { .. } | MobAction::Returning => None,
    }
}

fn status_kinds(name: &str) -> &'static [StatusChangeKind] {
    use StatusChangeKind::*;
    match name {
        "anybad" => &[Stone, Freeze, Stun, Sleep, Poison, Curse, Silence, Confusion, Blind],
        "stone" => &[Stone],
        "freeze" => &[Freeze],
        "stun" => &[Stun],
        "sleep" => &[Sleep],
        "poison" => &[Poison],
        "curse" => &[Curse],
        "silence" => &[Silence],
        "confusion" => &[Confusion],
        "blind" => &[Blind],
        "hiding" => &[Hiding],
        _ => &[],
    }
}

fn hp_rate(mob: &Mob) -> u32 {
    let max = mob.status.max_hp().max(1);
    (u64::from(mob.hp()) * 100 / u64::from(max)) as u32
}

fn number(value: &str) -> Option<i64> {
    value.parse().ok()
}

enum SpecialMobSkill {
    Emotion(u8),
    SummonSlaves { classes: Vec<i32>, amount: u16 },
    CallSlaves,
}

struct Friend {
    id: u32,
    x: u16,
    y: u16,
}

impl MapInstanceService {
    pub fn mobs_skill_ai(&self, state: &mut MapInstanceState, tick: u128) {
        let database = MobSkillDatabase::instance();
        let map = state.key().map_name().clone();
        let instance = state.key().map_instance();
        let characters: Vec<(u32, u16, u16)> = state
            .characters()
            .iter()
            .map(|character| (character.map_item().id(), character.x(), character.y()))
            .collect();
        let orphans: Vec<u32> = state
            .mobs()
            .values()
            .filter(|slave| slave.summoned && slave.summon_ai == 0 && slave.summon_owner.is_some_and(|owner| {
                !state.mobs().get(&owner).is_some_and(|master| master.is_present() && master.hp() > 0)
            }))
            .map(|slave| slave.id)
            .collect();
        for orphan in orphans {
            if let Some(slave) = state.mobs_mut().get_mut(&orphan) {
                slave.set_to_remove();
            }
        }
        let mob_ids: Vec<u32> = state.mobs().keys().copied().collect();
        let mut casts = Vec::new();
        let mut specials = Vec::new();
        for id in mob_ids {
            if state.script_skill_state.casts.contains_key(&id) {
                continue;
            }
            let Some(mob) = state.mobs().get(&id) else {
                continue;
            };
            if !mob.is_present() || mob.hp() == 0 || mob.script_cast_until > tick {
                continue;
            }
            let Some(mob_state) = mob_state(mob) else {
                continue;
            };
            let slaves = state.mobs().values().filter(|other| other.summon_owner == Some(id)).count() as i64;
            let nearby = state
                .mobs()
                .values()
                .filter(|other| other.id != id && other.is_present() && other.x.abs_diff(mob.x).max(other.y.abs_diff(mob.y)) <= NEARBY_MOB_RANGE)
                .count() as i64;
            let first_pass = !mob.skill_spawn_done;
            let master_attacked = mob
                .summon_owner
                .and_then(|owner| state.mobs().get(&owner))
                .is_some_and(|master| master.last_attacked_at != 0 && tick.saturating_sub(master.last_attacked_at) <= RECENT_ATTACK_WINDOW_MS);
            let mut selected = None;
            for (index, entry) in database.entries_for(mob).enumerate() {
                let key = entry_key(entry, index);
                if mob.skill_ready_at.get(&key).is_some_and(|ready| *ready > tick) {
                    continue;
                }
                let angry = mob.last_attacked_at != 0;
                if !state_matches(&entry.state, mob_state, angry) {
                    continue;
                }
                if entry.rate < 10_000 && fastrand::u32(0..10_000) >= entry.rate {
                    continue;
                }
                let Some(metadata) = SkillMetadata::find(entry.skill_id) else {
                    continue;
                };
                if let Some(special) = Self::special_action(mob, entry, metadata) {
                    if Self::condition_met(mob, entry, first_pass, slaves, nearby, master_attacked, &characters, tick) {
                        selected = Some((key, None, entry.delay));
                        specials.push((id, special));
                        break;
                    }
                    continue;
                }
                let actor = ScriptSkillActor::from_mob(mob, map.clone(), instance);
                if ScriptSkillService::validate_script_actor_operation(metadata, &actor, entry.level.min(u16::from(metadata.max_level)) as u8).is_err()
                    || entry.level == 0
                    || !actor.can_cast(entry.skill_id)
                {
                    continue;
                }
                let Some(friend) = Self::condition_friend(state, mob, entry) else {
                    if matches!(entry.condition.as_str(), "friendhpltmaxrate" | "friendhpinrate" | "friendstatuson" | "friendstatusoff") {
                        continue;
                    }
                    if !Self::condition_met(mob, entry, first_pass, slaves, nearby, master_attacked, &characters, tick) {
                        continue;
                    }
                    if let Some(cast) = Self::build_cast(mob, entry, metadata, &actor, None, &characters) {
                        selected = Some((key, Some(cast), entry.delay));
                        break;
                    }
                    continue;
                };
                if let Some(cast) = Self::build_cast(mob, entry, metadata, &actor, Some(&friend), &characters) {
                    selected = Some((key, Some(cast), entry.delay));
                    break;
                }
            }
            if first_pass || selected.is_some() {
                if let Some(mob) = state.mobs_mut().get_mut(&id) {
                    mob.skill_spawn_done = true;
                    if let Some((key, cast, delay)) = &selected {
                        mob.skill_ready_at.insert(*key, tick + *delay);
                        if let Some(cast) = cast {
                            mob.last_cast_skill = cast.skill_id;
                            mob.last_cast_at = tick;
                        }
                    }
                }
            }
            if let Some((_, Some(mut cast), _)) = selected {
                cast.source_map = Some(map.clone());
                cast.source_instance = Some(instance);
                casts.push(cast);
            }
        }
        for (id, special) in specials {
            self.run_special_mob_skill(state, id, special);
        }
        for cast in casts {
            self.send_mob_skill(cast);
        }
    }

    fn special_action(mob: &Mob, entry: &MobSkillEntry, metadata: &SkillMetadata) -> Option<SpecialMobSkill> {
        match metadata.name.as_str() {
            "NPC_EMOTION" => Some(SpecialMobSkill::Emotion(entry.values.first()?.parse().ok()?)),
            "NPC_SUMMONSLAVE" => {
                let classes: Vec<i32> = entry
                    .values
                    .iter()
                    .filter_map(|value| value.parse().ok())
                    .filter(|class| *class > 0)
                    .collect();
                (!classes.is_empty() && mob.summon_owner.is_none()).then_some(SpecialMobSkill::SummonSlaves {
                    classes,
                    amount: entry.level.max(1),
                })
            }
            "NPC_CALLSLAVE" => Some(SpecialMobSkill::CallSlaves),
            _ => None,
        }
    }

    fn run_special_mob_skill(&self, state: &mut MapInstanceState, id: u32, special: SpecialMobSkill) {
        let Some((x, y)) = state.mobs().get(&id).map(|mob| (mob.x, mob.y)) else {
            return;
        };
        match special {
            SpecialMobSkill::Emotion(emotion) => {
                let mut packet = 0x00C0_u16.to_le_bytes().to_vec();
                packet.extend_from_slice(&id.to_le_bytes());
                packet.push(emotion);
                let notification = AreaNotification::new(
                    state.key().map_name().clone(),
                    state.key().map_instance(),
                    AreaNotificationRangeType::Fov { x, y, exclude_id: None },
                    packet,
                );
                let _ = self.client_notification_sender.send(Notification::Area(notification));
            }
            SpecialMobSkill::CallSlaves => {
                let slaves: Vec<u32> = state
                    .mobs()
                    .values()
                    .filter(|slave| slave.summon_owner == Some(id) && slave.is_present())
                    .map(|slave| slave.id)
                    .collect();
                for slave in slaves {
                    self.warp_mob_to(state, slave, x, y);
                }
            }
            SpecialMobSkill::SummonSlaves { classes, amount } => {
                for _ in 0..amount {
                    let class = classes[fastrand::usize(..classes.len())];
                    let request = ScriptSpawn {
                        mob_id: class,
                        x: i32::from(x),
                        y: i32::from(y),
                        name: String::new(),
                        amount: 1,
                        event: String::new(),
                        event_npc: None,
                        size: None,
                        ai: None,
                        owner_id: 0,
                    };
                    match self.script_spawn(state, request) {
                        Ok(ids) => {
                            for slave in ids {
                                if let Some(slave) = state.mobs_mut().get_mut(&slave) {
                                    slave.summon_owner = Some(id);
                                }
                            }
                        }
                        Err(error) => debug!("Monster slave summon failed: {error}"),
                    }
                }
            }
        }
    }

    fn send_mob_skill(&self, cast: ScriptSkillCast) {
        self.server_task_queue.add_to_first_index(GameEvent::ScriptUnitSkill(cast));
    }

    fn condition_friend(state: &MapInstanceState, mob: &Mob, entry: &MobSkillEntry) -> Option<Friend> {
        let value = number(&entry.condition_value).unwrap_or(0);
        let upper = entry.values.first().and_then(|value| number(value)).unwrap_or(100);
        state
            .mobs()
            .values()
            .filter(|other| {
                other.id != mob.id
                    && other.is_present()
                    && other.hp() > 0
                    && other.x.abs_diff(mob.x).max(other.y.abs_diff(mob.y)) <= FRIEND_SEARCH_RANGE
            })
            .find(|other| match entry.condition.as_str() {
                "friendhpltmaxrate" => i64::from(hp_rate(other)) < value,
                "friendhpinrate" => (value..=upper).contains(&i64::from(hp_rate(other))),
                "friendstatuson" => status_kinds(&entry.condition_value)
                    .iter()
                    .any(|kind| other.status.has_status_change(*kind)),
                "friendstatusoff" => !status_kinds(&entry.condition_value)
                    .iter()
                    .any(|kind| other.status.has_status_change(*kind)),
                _ => false,
            })
            .map(|other| Friend {
                id: other.id,
                x: other.x,
                y: other.y,
            })
    }

    fn condition_met(
        mob: &Mob,
        entry: &MobSkillEntry,
        first_pass: bool,
        slaves: i64,
        nearby: i64,
        master_attacked: bool,
        characters: &[(u32, u16, u16)],
        tick: u128,
    ) -> bool {
        let value = number(&entry.condition_value).unwrap_or(0);
        let recent = mob.last_attacked_at != 0 && tick.saturating_sub(mob.last_attacked_at) <= RECENT_ATTACK_WINDOW_MS;
        match entry.condition.as_str() {
            "always" => true,
            "onspawn" => first_pass,
            "myhpltmaxrate" => i64::from(hp_rate(mob)) < value,
            "myhpinrate" => {
                let upper = entry.values.first().and_then(|value| number(value)).unwrap_or(100);
                (value..=upper).contains(&i64::from(hp_rate(mob)))
            }
            "mystatuson" => status_kinds(&entry.condition_value)
                .iter()
                .any(|kind| mob.status.has_status_change(*kind)),
            "mystatusoff" => !status_kinds(&entry.condition_value)
                .iter()
                .any(|kind| mob.status.has_status_change(*kind)),
            "slavelt" => slaves < value,
            "slavele" => slaves <= value,
            "attackpcgt" => (mob.damages.len() as i64) > value,
            "attackpcge" => (mob.damages.len() as i64) >= value,
            "mobnearbygt" => nearby > value,
            "casttargeted" => mob.get_target_id().is_some(),
            "rudeattacked" => {
                recent
                    && characters
                        .iter()
                        .find(|(id, ..)| *id == mob.last_attacker_id)
                        .is_some_and(|(_, x, y)| x.abs_diff(mob.x).max(y.abs_diff(mob.y)) > mob.attack_range.max(1))
            }
            "closedattacked" => recent && mob.last_attack_flags & BattleFlag::Short.as_flag() != 0,
            "longrangeattacked" => recent && mob.last_attack_flags & BattleFlag::Long.as_flag() != 0,
            "damagedgt" => recent && i64::from(mob.last_damage) > value,
            "afterskill" => {
                mob.last_cast_at != 0
                    && tick.saturating_sub(mob.last_cast_at) <= AFTER_SKILL_WINDOW_MS
                    && (value == 0 || i64::from(mob.last_cast_skill) == value)
            }
            "masterattacked" => master_attacked,
            "skillused" => recent && i64::from(mob.last_attack_skill) == value,
            _ => false,
        }
    }

    fn build_cast(
        mob: &Mob,
        entry: &MobSkillEntry,
        metadata: &SkillMetadata,
        actor: &ScriptSkillActor,
        friend: Option<&Friend>,
        characters: &[(u32, u16, u16)],
    ) -> Option<ScriptSkillCast> {
        let ground_skill = metadata.target_type.as_deref() == Some("Ground");
        let range = u16::try_from(metadata.range(entry.level as u8).unwrap_or(0).unsigned_abs())
            .unwrap_or(14)
            .max(mob.attack_range.max(1));
        let distance = |x: u16, y: u16| x.abs_diff(mob.x).max(y.abs_diff(mob.y));
        let current_target = mob
            .get_target_id()
            .and_then(|id| characters.iter().find(|(character, ..)| *character == id));
        let (target_id, position) = match entry.target.as_str() {
            "self" => (mob.id, (mob.x, mob.y)),
            "target" => {
                let (id, x, y) = *current_target?;
                if distance(x, y) > range {
                    return None;
                }
                (id, (x, y))
            }
            "friend" => {
                let friend = friend?;
                (friend.id, (friend.x, friend.y))
            }
            "master" => {
                let owner = mob.summon_owner?;
                (owner, (mob.x, mob.y))
            }
            "randomtarget" => {
                let candidates: Vec<_> = characters.iter().filter(|(_, x, y)| distance(*x, *y) <= range).collect();
                let (id, x, y) = **candidates.get(fastrand::usize(..candidates.len().max(1)))?;
                (id, (x, y))
            }
            name if name.starts_with("around") => {
                let around: u8 = name.trim_start_matches("around").parse().unwrap_or(4);
                let (center, radius) = match around {
                    1..=4 => ((mob.x, mob.y), i32::from(around)),
                    5..=8 => {
                        let (_, x, y) = *current_target?;
                        ((x, y), i32::from(around - 4))
                    }
                    _ => ((mob.x, mob.y), 4),
                };
                let x = (i32::from(center.0) + fastrand::i32(-radius..=radius)).clamp(0, i32::from(u16::MAX)) as u16;
                let y = (i32::from(center.1) + fastrand::i32(-radius..=radius)).clamp(0, i32::from(u16::MAX)) as u16;
                (mob.id, (x, y))
            }
            _ => return None,
        };
        let mut cast = ScriptSkillCast {
            source_id: mob.id,
            target_id,
            skill_id: entry.skill_id,
            level: entry.level.min(u16::from(metadata.max_level)),
            cast_cancel: Some(entry.cancelable),
            ..Default::default()
        };
        if ground_skill || entry.target.starts_with("around") {
            cast.ground = Some(position);
        }
        let default_cast = actor_cast_time(actor, &cast).ok()? as i64;
        cast.cast_time_adjust_ms = (entry.cast_time - default_cast).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        Some(cast)
    }
}

fn entry_key(entry: &MobSkillEntry, index: usize) -> usize {
    index * 1_000_000 + entry.skill_id as usize
}
