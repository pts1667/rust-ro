//! Quest catalog (`db/pre-re/quest_db.yml` as imported by `tools/scripts-import/import_quests.py`) and the quest log of a character.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

/// `MAX_QUEST_OBJECTIVES` of rathena.
pub const MAX_QUEST_OBJECTIVES: usize = 3;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct QuestObjective {
    /// 0 targets any monster matching the filters below.
    pub mob: u16,
    pub count: u16,
    pub min_level: u16,
    pub max_level: u16,
    pub race: String,
    pub size: String,
    pub element: String,
    pub location: String,
    /// Text shown by the client instead of a monster name.
    pub map_name: String,
    pub mobs_allowed: Vec<u16>,
}

impl Default for QuestObjective {
    fn default() -> Self {
        Self {
            mob: 0,
            count: 0,
            min_level: 0,
            max_level: 0,
            race: "All".into(),
            size: "All".into(),
            element: "All".into(),
            location: String::new(),
            map_name: String::new(),
            mobs_allowed: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuestDrop {
    /// 0 applies to every monster.
    pub mob: u16,
    pub item: u32,
    pub count: u16,
    /// 10000 is 100%.
    pub rate: u16,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct QuestDefinition {
    pub id: u32,
    pub title: String,
    /// Seconds after the start, or the time of the day (`time_at`) at which the quest expires.
    pub time: i64,
    pub time_at: bool,
    /// Day of the week (0 is Sunday) for `time_at`, -1 for any day.
    pub time_week: i32,
    pub objectives: Vec<QuestObjective>,
    pub drops: Vec<QuestDrop>,
}

impl Default for QuestDefinition {
    fn default() -> Self {
        Self { id: 0, title: String::new(), time: 0, time_at: false, time_week: -1, objectives: Vec::new(), drops: Vec::new() }
    }
}

pub fn quest_catalog() -> &'static HashMap<u32, QuestDefinition> {
    static CATALOG: OnceLock<HashMap<u32, QuestDefinition>> = OnceLock::new();
    CATALOG.get_or_init(|| {
        let quests: Vec<QuestDefinition> = serde_json::from_str(include_str!("../script/quests.json")).expect("Embedded quest catalog is invalid");
        quests.into_iter().map(|quest| (quest.id, quest)).collect()
    })
}

pub fn quest_definition(quest_id: u32) -> Option<&'static QuestDefinition> {
    quest_catalog().get(&quest_id)
}

/// The client names the states: 0 inactive, 1 active, 2 complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestState {
    Inactive,
    Active,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestEntry {
    pub quest_id: u32,
    pub state: QuestState,
    /// Unix time at which the quest times out, 0 when it never does.
    pub expires_at: u32,
    pub counts: [u16; MAX_QUEST_OBJECTIVES],
}

/// `checkquest` second argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestCheck {
    Have,
    PlayTime,
    Hunting,
}

impl QuestCheck {
    pub fn from_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Have),
            1 => Some(Self::PlayTime),
            2 => Some(Self::Hunting),
            _ => None,
        }
    }
}

/// What the character just killed, for the hunting objectives.
#[derive(Debug, Clone, Copy)]
pub struct QuestKill<'a> {
    pub mob_id: u16,
    pub level: u16,
    pub race: &'a str,
    pub size: &'a str,
    pub element: &'a str,
    pub map: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalClock {
    pub now: i64,
    pub seconds_today: u32,
    /// 0 is Sunday.
    pub weekday: i32,
}

impl LocalClock {
    pub fn current() -> Self {
        use chrono::{Datelike, Timelike};
        let local = chrono::Local::now();
        Self {
            now: local.timestamp(),
            seconds_today: local.num_seconds_from_midnight(),
            weekday: local.weekday().num_days_from_sunday() as i32,
        }
    }
}

/// When the quest times out, as `quest_time` of rathena.
pub fn quest_expiry(definition: &QuestDefinition, clock: LocalClock) -> u32 {
    let expiry = if definition.time_at {
        let mut day_shift = i64::from(i64::from(clock.seconds_today) >= definition.time % 86_400);
        if definition.time_week > -1 {
            let weekday = i64::from(clock.weekday);
            let week = i64::from(definition.time_week);
            day_shift = if week < weekday + day_shift { week + 7 - weekday } else { week - weekday };
        }
        clock.now + day_shift * 86_400 + definition.time - i64::from(clock.seconds_today)
    } else if definition.time > 0 {
        clock.now + definition.time
    } else {
        0
    };
    u32::try_from(expiry.max(0)).unwrap_or(u32::MAX)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestError {
    Unknown(u32),
    AlreadyHas(u32),
    Missing(u32),
    Completed(u32),
}

impl std::fmt::Display for QuestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(id) => write!(f, "quest {id} is not in the quest database"),
            Self::AlreadyHas(id) => write!(f, "the character already has quest {id}"),
            Self::Missing(id) => write!(f, "the character does not have quest {id}"),
            Self::Completed(id) => write!(f, "the character has completed quest {id}"),
        }
    }
}

/// Active and inactive quests come first, completed ones after, like `quest_log` of rathena.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QuestLog {
    entries: Vec<QuestEntry>,
}

impl QuestLog {
    pub fn from_entries(entries: Vec<QuestEntry>) -> Self {
        let (mut open, closed): (Vec<_>, Vec<_>) = entries.into_iter().filter(|entry| quest_definition(entry.quest_id).is_some()).partition(|entry| entry.state != QuestState::Complete);
        open.extend(closed);
        Self { entries: open }
    }

    pub fn entries(&self) -> &[QuestEntry] {
        &self.entries
    }

    /// Quests that are not completed.
    pub fn open_entries(&self) -> &[QuestEntry] {
        &self.entries[..self.open_count()]
    }

    fn open_count(&self) -> usize {
        self.entries.iter().take_while(|entry| entry.state != QuestState::Complete).count()
    }

    pub fn find(&self, quest_id: u32) -> Option<&QuestEntry> {
        self.entries.iter().find(|entry| entry.quest_id == quest_id)
    }

    pub fn add(&mut self, quest_id: u32, clock: LocalClock) -> Result<QuestEntry, QuestError> {
        let definition = quest_definition(quest_id).ok_or(QuestError::Unknown(quest_id))?;
        if self.find(quest_id).is_some() {
            return Err(QuestError::AlreadyHas(quest_id));
        }
        let entry = QuestEntry { quest_id, state: QuestState::Active, expires_at: quest_expiry(definition, clock), counts: [0; MAX_QUEST_OBJECTIVES] };
        let position = self.open_count();
        self.entries.insert(position, entry);
        Ok(entry)
    }

    /// Replaces an open quest by a new one in place.
    pub fn change(&mut self, old_id: u32, new_id: u32, clock: LocalClock) -> Result<QuestEntry, QuestError> {
        let definition = quest_definition(new_id).ok_or(QuestError::Unknown(new_id))?;
        if self.find(new_id).is_some() {
            return Err(QuestError::AlreadyHas(new_id));
        }
        let position = self.find_open(old_id)?;
        let entry = QuestEntry { quest_id: new_id, state: QuestState::Active, expires_at: quest_expiry(definition, clock), counts: [0; MAX_QUEST_OBJECTIVES] };
        self.entries[position] = entry;
        Ok(entry)
    }

    pub fn delete(&mut self, quest_id: u32) -> Result<QuestEntry, QuestError> {
        let position = self.entries.iter().position(|entry| entry.quest_id == quest_id).ok_or(QuestError::Missing(quest_id))?;
        Ok(self.entries.remove(position))
    }

    fn find_open(&self, quest_id: u32) -> Result<usize, QuestError> {
        match self.entries.iter().position(|entry| entry.quest_id == quest_id) {
            None => Err(QuestError::Missing(quest_id)),
            Some(position) if self.entries[position].state == QuestState::Complete => Err(QuestError::Completed(quest_id)),
            Some(position) => Ok(position),
        }
    }

    /// Completed quests are moved behind the open ones.
    pub fn set_state(&mut self, quest_id: u32, state: QuestState) -> Result<(), QuestError> {
        let position = self.find_open(quest_id)?;
        let last_open = self.open_count() - 1;
        self.entries[position].state = state;
        if state == QuestState::Complete {
            self.entries.swap(position, last_open);
        }
        Ok(())
    }

    /// `checkquest`: -1 when the quest is not in the log.
    pub fn check(&self, quest_id: u32, check: QuestCheck, now: i64) -> i32 {
        let Some(entry) = self.find(quest_id) else { return -1 };
        match check {
            QuestCheck::Have => match entry.state {
                QuestState::Inactive => 1,
                QuestState::Active => 1,
                QuestState::Complete => 2,
            },
            QuestCheck::PlayTime => {
                if i64::from(entry.expires_at) < now {
                    2
                } else {
                    i32::from(entry.state == QuestState::Complete)
                }
            }
            QuestCheck::Hunting => {
                if entry.state == QuestState::Complete {
                    return 0;
                }
                let Some(definition) = quest_definition(quest_id) else { return -1 };
                let done = definition.objectives.iter().zip(entry.counts).all(|(objective, count)| count >= objective.count);
                if done {
                    2
                } else {
                    i32::from(i64::from(entry.expires_at) < now)
                }
            }
        }
    }

    /// Counts the kill on every open quest; returns the quests whose counters changed and the extra drops they grant.
    pub fn record_kill(&mut self, kill: &QuestKill) -> (Vec<u32>, Vec<&'static QuestDrop>) {
        let mut updated = Vec::new();
        let mut drops = Vec::new();
        for entry in self.entries.iter_mut().filter(|entry| entry.state != QuestState::Complete) {
            let Some(definition) = quest_definition(entry.quest_id) else { continue };
            let mut changed = false;
            for (objective, count) in definition.objectives.iter().zip(entry.counts.iter_mut()) {
                if objective_matches(objective, kill) && *count < objective.count {
                    *count += 1;
                    changed = true;
                }
            }
            if changed {
                updated.push(entry.quest_id);
            }
            drops.extend(definition.drops.iter().filter(|drop| drop.mob == 0 || drop.mob == kill.mob_id));
        }
        (updated, drops)
    }
}

fn objective_matches(objective: &QuestObjective, kill: &QuestKill) -> bool {
    if objective.mob != 0 {
        return objective.mob == kill.mob_id;
    }
    (objective.min_level == 0 || objective.min_level <= kill.level)
        && (objective.max_level == 0 || objective.max_level >= kill.level)
        && (objective.race == "All" || objective.race == kill.race)
        && (objective.size == "All" || objective.size == kill.size)
        && (objective.element == "All" || objective.element == kill.element)
        && (objective.location.is_empty() || objective.location == kill.map)
        && (objective.mobs_allowed.is_empty() || objective.mobs_allowed.contains(&kill.mob_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLOCK: LocalClock = LocalClock { now: 1_000_000, seconds_today: 3_600, weekday: 3 };

    fn hunting_quest() -> &'static QuestDefinition {
        quest_catalog().values().find(|quest| quest.objectives.len() == 1 && quest.objectives[0].mob != 0 && quest.time == 0).expect("a hunting quest")
    }

    #[test]
    fn completed_quests_stay_behind_the_open_ones() {
        let mut ids = quest_catalog().keys().copied().collect::<Vec<_>>();
        ids.sort();
        let mut log = QuestLog::default();
        for id in &ids[..3] {
            log.add(*id, CLOCK).unwrap();
        }
        assert_eq!(log.add(ids[0], CLOCK), Err(QuestError::AlreadyHas(ids[0])));
        log.set_state(ids[0], QuestState::Complete).unwrap();
        log.add(ids[3], CLOCK).unwrap();
        let order: Vec<u32> = log.entries().iter().map(|entry| entry.quest_id).collect();
        assert_eq!(order.last(), Some(&ids[0]));
        assert_eq!(log.open_entries().len(), 3);
        assert_eq!(log.set_state(ids[0], QuestState::Active), Err(QuestError::Completed(ids[0])));
        assert_eq!(log.check(ids[0], QuestCheck::Have, CLOCK.now), 2);
        assert_eq!(log.check(999_999_999, QuestCheck::Have, CLOCK.now), -1);
    }

    #[test]
    fn kills_advance_matching_objectives_up_to_their_count() {
        let quest = hunting_quest();
        let objective = &quest.objectives[0];
        let mut log = QuestLog::default();
        log.add(quest.id, CLOCK).unwrap();
        let kill = |mob_id| QuestKill { mob_id, level: 1, race: "Formless", size: "Small", element: "Neutral", map: "prontera" };
        assert!(log.record_kill(&kill(objective.mob + 1)).0.is_empty());
        for _ in 0..objective.count + 2 {
            log.record_kill(&kill(objective.mob));
        }
        assert_eq!(log.find(quest.id).unwrap().counts[0], objective.count);
        assert_eq!(log.check(quest.id, QuestCheck::Hunting, CLOCK.now), 2);
    }

    #[test]
    fn expiry_follows_relative_and_fixed_limits() {
        let relative = QuestDefinition { time: 3_600, ..Default::default() };
        assert_eq!(quest_expiry(&relative, CLOCK), 1_003_600);
        let daily = QuestDefinition { time: 5 * 3_600, time_at: true, ..Default::default() };
        assert_eq!(quest_expiry(&daily, CLOCK), 1_000_000 + 18_000 - 3_600);
        let past = QuestDefinition { time: 3_600, time_at: true, ..Default::default() };
        assert_eq!(quest_expiry(&past, CLOCK), 1_000_000 + 86_400);
        assert_eq!(quest_expiry(&QuestDefinition::default(), CLOCK), 0);
    }
}
