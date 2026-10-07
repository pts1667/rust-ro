use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Deserializer};

use crate::battle_options::BATTLE_OPTIONS;

/// Options of rathena's `conf/battle/*.conf`, keyed by their rathena name.
/// Every option of `battle_data` is known with its pre-renewal default and range, so a name that is
/// not in the table is a typo and is reported when loading.
#[derive(Debug, Clone)]
pub struct BattleConfig {
    values: HashMap<&'static str, i64>,
}

impl Default for BattleConfig {
    fn default() -> Self {
        Self {
            values: BATTLE_OPTIONS.iter().map(|(name, default, _, _)| (*name, *default)).collect(),
        }
    }
}

fn parse_value(raw: &str) -> Option<i64> {
    let raw = raw.trim();
    match raw.to_ascii_lowercase().as_str() {
        "yes" | "on" | "true" => Some(1),
        "no" | "off" | "false" => Some(0),
        other => other
            .strip_prefix("0x")
            .map_or_else(|| other.parse().ok(), |hex| i64::from_str_radix(hex, 16).ok()),
    }
}

impl BattleConfig {
    /// Value of the option, `name` must be a rathena battle option.
    pub fn get(&self, name: &str) -> i64 {
        self.values.get(name).copied().unwrap_or_else(|| {
            debug_assert!(false, "unknown battle option {name}");
            0
        })
    }

    /// Default of an option, for code that can run before the configuration is loaded (unit tests).
    pub fn default_value(name: &str) -> i64 {
        BATTLE_OPTIONS.iter().find(|(known, _, _, _)| *known == name).map_or(0, |(_, default, _, _)| *default)
    }

    pub fn flag(&self, name: &str) -> bool {
        self.get(name) != 0
    }

    pub fn is_known(name: &str) -> bool {
        BATTLE_OPTIONS.iter().any(|(known, _, _, _)| *known == name)
    }

    /// Sets an option, a value out of its range is clamped as rathena does.
    pub fn set(&mut self, name: &str, value: i64) -> Result<(), String> {
        let (known, _, min, max) = BATTLE_OPTIONS
            .iter()
            .find(|(known, _, _, _)| *known == name)
            .ok_or_else(|| format!("unknown battle option {name}"))?;
        self.values.insert(known, value.clamp(*min, *max));
        Ok(())
    }

    /// Reads the text of a `.conf` file (`name: value`, `//` comments) and returns the problems found.
    pub fn load_text(&mut self, text: &str) -> Vec<String> {
        let mut problems = vec![];
        for line in text.lines() {
            let line = line.split("//").next().unwrap_or("").trim();
            let Some((name, value)) = line.split_once(':') else { continue };
            let name = name.trim();
            if name == "import" {
                continue;
            }
            match parse_value(value) {
                Some(value) => {
                    if let Err(error) = self.set(name, value) {
                        problems.push(error);
                    }
                }
                None => problems.push(format!("battle option {name} has invalid value {}", value.trim())),
            }
        }
        problems
    }

    /// Loads every `.conf` file of a directory laid out like rathena's `conf/battle`.
    pub fn load_dir(&mut self, directory: &Path) -> Result<Vec<String>, String> {
        let mut files: Vec<_> = fs::read_dir(directory)
            .map_err(|error| format!("{}: {error}", directory.display()))?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|extension| extension == "conf"))
            .collect();
        files.sort();
        let mut problems = vec![];
        for file in files {
            let text = fs::read_to_string(&file).map_err(|error| format!("{}: {error}", file.display()))?;
            problems.extend(self.load_text(&text).into_iter().map(|problem| format!("{}: {problem}", file.display())));
        }
        Ok(problems)
    }

    /// Takes over every option of `other` that differs from its default.
    pub fn merge_changed_from(&mut self, other: &BattleConfig) {
        let defaults = BattleConfig::default();
        for (name, value) in &other.values {
            if defaults.values.get(name) != Some(value) {
                self.values.insert(name, *value);
            }
        }
    }

    /// Replaces the options named in a JSON object (numbers or booleans).
    pub fn apply_json(&mut self, object: &HashMap<String, serde_json::Value>) -> Vec<String> {
        let mut problems = vec![];
        for (name, value) in object {
            let value = match value {
                serde_json::Value::Bool(value) => Some(i64::from(*value)),
                serde_json::Value::Number(number) => number.as_i64(),
                serde_json::Value::String(text) => parse_value(text),
                _ => None,
            };
            match value {
                Some(value) => {
                    if let Err(error) = self.set(name, value) {
                        problems.push(error);
                    }
                }
                None => problems.push(format!("battle option {name} has no numeric value")),
            }
        }
        problems
    }
}

impl<'de> Deserialize<'de> for BattleConfig {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let object = HashMap::<String, serde_json::Value>::deserialize(deserializer)?;
        let mut config = Self::default();
        for problem in config.apply_json(&object) {
            eprintln!("config.json: {problem}");
        }
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_the_pre_renewal_values_and_conf_text_overrides_them() {
        let mut config = BattleConfig::default();
        assert_eq!(config.get("natural_heal_weight_rate"), 50);
        assert_eq!(config.get("natural_healhp_interval"), 6000);
        let problems = config.load_text("natural_healhp_interval: 3000 // faster\nnatural_heal_weight_rate: 500\nno_such_option: 1\nbasic_skill_check: no\n");
        assert_eq!(config.get("natural_healhp_interval"), 3000);
        assert_eq!(config.get("natural_heal_weight_rate"), 100, "clamped to the range");
        assert!(!config.flag("basic_skill_check"));
        assert_eq!(problems, vec!["unknown battle option no_such_option".to_string()]);
    }
}
