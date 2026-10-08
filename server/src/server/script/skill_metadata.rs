use std::collections::BTreeMap;
use std::sync::OnceLock;

use models::enums::EnumWithMaskValueU32;
use models::status_bonus::BattleFlag;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum LeveledValue {
    Scalar(i32),
    Levels(Vec<LevelEntry>),
}

#[derive(Clone, Debug, Deserialize)]
pub struct LevelEntry {
    #[serde(rename = "Level")]
    level: u8,
    #[serde(flatten)]
    values: BTreeMap<String, i32>,
}

impl LeveledValue {
    pub fn value(&self, level: u8, field: &str) -> Option<i32> {
        match self {
            Self::Scalar(value) => Some(*value),
            Self::Levels(values) => values
                .iter()
                .find(|value| value.level == level)
                .and_then(|value| value.values.get(field))
                .copied(),
        }
    }
}

/// First-level dispatch of a skill: the handler family that runs it when a player or an actor casts it.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum SkillRoute {
    // Script path, one variant per `SkillOperation`.
    Damage,
    Status,
    AreaStatus,
    Ground,
    Recovery,
    Inventory,
    Movement,
    Spirit,
    Dispel,
    Tarot,
    Estimate,
    /// Opens an item selection menu (`skill_menu_service`).
    Menu,
    /// Active guild skill (`guild_skill_service`).
    Guild,
    /// `lib/skills` native offensive skill.
    Native,
    /// Passive: the effect is read from the learned level where it applies.
    Passive,
    /// Monster, homunculus, mercenary and other actor-only skills.
    Actor,
    /// No handler yet; casting does nothing. Listed so the gap stays visible.
    Unrouted,
}

impl SkillRoute {
    pub fn operation(self) -> Option<super::callbacks::SkillOperation> {
        use super::callbacks::SkillOperation as Operation;
        Some(match self {
            Self::Damage => Operation::Damage,
            Self::Status => Operation::Status,
            Self::AreaStatus => Operation::AreaStatus,
            Self::Ground => Operation::Ground,
            Self::Recovery => Operation::Recovery,
            Self::Inventory => Operation::Inventory,
            Self::Movement => Operation::Movement,
            Self::Spirit => Operation::Spirit,
            Self::Dispel => Operation::Dispel,
            Self::Tarot => Operation::Tarot,
            Self::Estimate => Operation::Estimate,
            Self::Menu | Self::Guild | Self::Native | Self::Passive | Self::Actor | Self::Unrouted => return None,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkillMetadata {
    pub route: Option<SkillRoute>,
    pub id: u32,
    pub name: String,
    pub max_level: u8,
    #[serde(rename = "Type")]
    pub damage_type: Option<String>,
    pub target_type: Option<String>,
    pub element: Option<serde_json::Value>,
    pub range: Option<LeveledValue>,
    pub hit_count: Option<LeveledValue>,
    pub splash_area: Option<LeveledValue>,
    pub cast_time: Option<LeveledValue>,
    pub cast_cancel: Option<bool>,
    #[serde(default)]
    pub cast_time_flags: BTreeMap<String, bool>,
    #[serde(default)]
    pub cast_delay_flags: BTreeMap<String, bool>,
    pub after_cast_act_delay: Option<LeveledValue>,
    pub after_cast_walk_delay: Option<LeveledValue>,
    pub duration1: Option<LeveledValue>,
    pub duration2: Option<LeveledValue>,
    pub cooldown: Option<LeveledValue>,
    pub knockback: Option<LeveledValue>,
    pub status: Option<String>,
    pub unit: Option<serde_json::Value>,
    pub requires: Option<serde_json::Value>,
    #[serde(default)]
    pub flags: BTreeMap<String, bool>,
    #[serde(default)]
    pub damage_flags: BTreeMap<String, bool>,
}

impl SkillMetadata {
    pub fn all() -> &'static [SkillMetadata] {
        static METADATA: OnceLock<Vec<SkillMetadata>> = OnceLock::new();
        METADATA.get_or_init(|| {
            serde_json::from_str(include_str!("skill_metadata.json")).expect("Embedded pre-renewal skill metadata is invalid")
        })
    }

    pub fn find(id: u32) -> Option<&'static Self> {
        Self::all().iter().find(|skill| skill.id == id)
    }

    pub fn find_by_name(name: &str) -> Option<&'static Self> {
        static BY_NAME: OnceLock<std::collections::HashMap<&'static str, &'static SkillMetadata>> = OnceLock::new();
        BY_NAME
            .get_or_init(|| Self::all().iter().map(|skill| (skill.name.as_str(), skill)).collect())
            .get(name)
            .copied()
    }

    pub fn flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }

    pub fn damages(&self) -> bool {
        !self.damage_flags.get("NoDamage").copied().unwrap_or(false)
    }

    pub fn duration(&self, level: u8, secondary: bool) -> Option<i32> {
        if secondary { &self.duration2 } else { &self.duration1 }
            .as_ref()
            .and_then(|value| value.value(level, "Time"))
    }

    pub fn range(&self, level: u8) -> Option<i32> {
        self.range.as_ref().and_then(|value| value.value(level, "Size"))
    }

    pub fn player_range(&self, source: &models::status::StatusSnapshot, level: u8) -> u16 {
        use models::enums::skill_enums::SkillEnum;
        let mut range = self.range(level).unwrap_or(1).unsigned_abs().min(14) as u16;
        if self.flags.get("AlterRangeVulture").copied().unwrap_or(false) {
            range = range.saturating_add(u16::from(source.known_skill_level(SkillEnum::AcVulture)));
        }
        if self.flags.get("AlterRangeSnakeEye").copied().unwrap_or(false) {
            range = range.saturating_add(u16::from(source.known_skill_level(SkillEnum::GsSnakeeye)));
        }
        range
    }

    pub fn element(&self, level: u8) -> Option<&str> {
        let element = self.element.as_ref()?;
        element.as_str().or_else(|| {
            element
                .as_array()?
                .iter()
                .find(|entry| entry.get("Level").and_then(|level| level.as_u64()) == Some(u64::from(level)))?
                .get("Element")?
                .as_str()
        })
    }

    pub fn cast_duration(&self, level: u8, cast_modifier: f32) -> u128 {
        let duration = self
            .cast_time
            .as_ref()
            .and_then(|value| value.value(level, "Time"))
            .unwrap_or(0)
            .max(0) as f32;
        (duration * cast_modifier).ceil() as u128
    }

    pub fn splash(&self, level: u8) -> Option<i32> {
        self.splash_area.as_ref().and_then(|value| value.value(level, "Area"))
    }

    pub fn unit_value(&self, field: &str, level: u8, level_field: &str) -> Option<i32> {
        Self::json_level_value(self.unit.as_ref()?.get(field)?, level, level_field)
    }

    pub fn unit_flag(&self, name: &str) -> bool {
        self.unit
            .as_ref()
            .and_then(|unit| unit.get("Flag")?.get(name)?.as_bool())
            .unwrap_or(false)
    }

    pub fn json_level_value(value: &serde_json::Value, level: u8, field: &str) -> Option<i32> {
        value
            .as_i64()
            .map(|value| value.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
            .or_else(|| {
                value
                    .as_array()?
                    .iter()
                    .find(|value| value.get("Level").and_then(|value| value.as_u64()) == Some(level as u64))?
                    .get(field)?
                    .as_i64()
                    .and_then(|value| i32::try_from(value).ok())
            })
    }

    pub fn battle_flags(&self, ranged: bool) -> u32 {
        (match self.damage_type.as_deref() {
            Some("Weapon") => BattleFlag::Weapon,
            Some("Magic") => BattleFlag::Magic,
            _ => BattleFlag::Misc,
        })
        .as_flag()
            | if self.id == models::enums::skill_enums::SkillEnum::TfThrowstone.id() {
                BattleFlag::Weapon.as_flag()
            } else {
                0
            }
            | (if ranged { BattleFlag::Long } else { BattleFlag::Short }).as_flag()
            | BattleFlag::Skill.as_flag()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pre_renewal_metadata_retains_skill_levels_and_delayed_effect_durations() {
        let frost = SkillMetadata::find(15).unwrap();
        assert_eq!(frost.name, "MG_FROSTDIVER");
        assert_eq!(frost.duration(5, true), Some(15000));
        let joker = SkillMetadata::all().iter().find(|skill| skill.name == "BA_FROSTJOKER").unwrap();
        assert_eq!(joker.duration(1, false), Some(15000));
        assert_eq!(joker.duration(1, true), Some(12000));
    }

    #[test]
    fn player_skill_range_uses_absolute_base_then_the_matching_effective_eye_passive() {
        use models::enums::skill_enums::SkillEnum;
        use models::status::{KnownSkill, Status, StatusSnapshot};
        let mut status = StatusSnapshot::_from(&Status::default());
        status.set_known_skills(vec![
            KnownSkill {
                value: SkillEnum::AcVulture,
                level: 10,
            },
            KnownSkill {
                value: SkillEnum::GsSnakeeye,
                level: 7,
            },
        ]);
        let double = SkillMetadata::find(SkillEnum::AcDouble.id()).unwrap();
        assert_eq!(double.range(1), Some(-9));
        assert_eq!(double.player_range(&status, 1), 19);
        assert_eq!(
            SkillMetadata::find(SkillEnum::GsRapidshower.id()).unwrap().player_range(&status, 1),
            16
        );
        assert_eq!(
            SkillMetadata::find(SkillEnum::MgFirebolt.id()).unwrap().player_range(&status, 1),
            9
        );
        let mut long_base = double.clone();
        long_base.range = Some(LeveledValue::Scalar(-30));
        assert_eq!(long_base.player_range(&status, 1), 24);
    }
}
