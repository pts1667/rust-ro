//! Skills and jobs of plugins: a scaffold (`docs/plugins.md`). A plugin's `skills.json` and `jobs.json` declare them with ids
//! in [`PLUGIN_SKILL_IDS`] and [`PLUGIN_JOB_IDS`]. The server checks them and publishes them in the client catalog, but no
//! character can learn, use or become them yet: characters hold skills as `SkillEnum`, which has no room for them, and
//! clients would not know their ids. What remains is listed in the docs.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use configuration::configuration::SkillConfig;
use models::enums::class::JobName;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use plugin_ui::catalog::{CatalogJob, CatalogSkill, CatalogTreeSkill, PLUGIN_JOB_IDS, PLUGIN_SKILL_IDS, SkillRequirement, SkillTarget};
use serde::Deserialize;

const MAX_SKILL_LEVEL: u8 = 10;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PluginSkill {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub description: Vec<String>,
    pub max_level: u8,
    #[serde(default)]
    pub target: SkillTarget,
    #[serde(default)]
    pub sp_cost: Vec<u16>,
    #[serde(default)]
    pub range: u8,
    #[serde(default)]
    pub look: Option<u32>,
    /// A file of the plugin's `ui/` folder.
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PluginJob {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    /// A job of the game, by name: `Wizard`, `Knight`...
    pub base_job: String,
    #[serde(default)]
    pub skills: Vec<PluginTreeSkill>,
}

/// A skill of a job's tree, by name: a plugin skill or one of the game.
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PluginTreeSkill {
    pub skill: String,
    pub max_level: u8,
    #[serde(default)]
    pub requires: Vec<PluginSkillRequirement>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PluginSkillRequirement {
    pub skill: String,
    pub level: u8,
}

fn read<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Vec<T>, String> {
    match std::fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|error| format!("{} is invalid: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(error) => Err(format!("cannot read {}: {error}", path.display())),
    }
}

/// Reads and checks `<folder>/skills.json`.
pub fn load_skills(folder: &Path) -> Result<Vec<PluginSkill>, String> {
    let skills: Vec<PluginSkill> = read(&folder.join("skills.json"))?;
    let (mut ids, mut names) = (HashSet::new(), HashSet::new());
    for skill in &skills {
        let id = skill.id;
        if !PLUGIN_SKILL_IDS.contains(&id) {
            return Err(format!("skill id {id} is outside the ids of plugin skills, {}..={}", PLUGIN_SKILL_IDS.start(), PLUGIN_SKILL_IDS.end()));
        }
        if !ids.insert(id) || !names.insert(skill.name.clone()) {
            return Err(format!("skill {id} ({}) is defined twice", skill.name));
        }
        if skill.name.is_empty() || !skill.name.bytes().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_') {
            return Err(format!("skill {id}: name {:?} must be upper case letters, digits or _, like SM_BASH", skill.name));
        }
        if !(1..=MAX_SKILL_LEVEL).contains(&skill.max_level) {
            return Err(format!("skill {id}: max_level must be 1 to {MAX_SKILL_LEVEL}"));
        }
        if !skill.sp_cost.is_empty() && skill.sp_cost.len() != usize::from(skill.max_level) {
            return Err(format!("skill {id}: sp_cost has {} levels, max_level is {}", skill.sp_cost.len(), skill.max_level));
        }
        if let Some(icon) = &skill.icon {
            super::items::check_icon(folder, icon).map_err(|error| format!("skill {id}: {error}"))?;
        }
    }
    Ok(skills)
}

/// Reads and checks `<folder>/jobs.json`. Skill names are resolved with the other plugins' skills by [`catalog`].
pub fn load_jobs(folder: &Path) -> Result<Vec<PluginJob>, String> {
    let jobs: Vec<PluginJob> = read(&folder.join("jobs.json"))?;
    let mut ids = HashSet::new();
    for job in &jobs {
        if !PLUGIN_JOB_IDS.contains(&job.id) {
            return Err(format!("job id {} is outside the ids of plugin jobs, {}..={}", job.id, PLUGIN_JOB_IDS.start(), PLUGIN_JOB_IDS.end()));
        }
        if !ids.insert(job.id) {
            return Err(format!("job {} is defined twice", job.id));
        }
        JobName::try_from_string_ignore_case(&job.base_job).map_err(|_| format!("job {}: base_job {} is not a job of the game", job.id, job.base_job))?;
        for tree_skill in &job.skills {
            if !(1..=MAX_SKILL_LEVEL).contains(&tree_skill.max_level) {
                return Err(format!("job {}: skill {} has max_level outside 1 to {MAX_SKILL_LEVEL}", job.id, tree_skill.skill));
            }
        }
    }
    Ok(jobs)
}

/// The catalog entries of the plugins' skills and jobs, with the names of their trees resolved against the game's skills
/// and every plugin's. `plugins` are plugin names with their declarations, in load order.
pub fn catalog(plugins: &[(&str, &[PluginSkill], &[PluginJob])], game_skills: &HashMap<u32, SkillConfig>) -> Result<(Vec<CatalogSkill>, Vec<CatalogJob>), String> {
    let mut skill_ids: HashMap<&str, (u32, u8)> = game_skills.values().map(|skill| (skill.name.as_str(), (skill.id, MAX_SKILL_LEVEL))).collect();
    let mut owners: HashMap<u32, &str> = HashMap::new();
    let mut skills = Vec::new();
    for (plugin, plugin_skills, _) in plugins {
        for skill in plugin_skills.iter() {
            if let Some(other) = owners.insert(skill.id, plugin) {
                return Err(format!("Plugins {other} and {plugin} both define skill {}", skill.id));
            }
            if skill_ids.insert(&skill.name, (skill.id, skill.max_level)).is_some() {
                return Err(format!("Plugin {plugin} is refused: skill name {} is already taken", skill.name));
            }
            if let Some(look) = skill.look.filter(|look| !game_skills.contains_key(look)) {
                return Err(format!("Plugin {plugin} is refused: skill {} looks like skill {look}, which is not a skill of the game", skill.id));
            }
            skills.push(CatalogSkill {
                id: skill.id,
                plugin: plugin.to_string(),
                name: skill.name.clone(),
                display_name: skill.display_name.clone(),
                description: skill.description.clone(),
                max_level: skill.max_level,
                target: skill.target,
                sp_cost: skill.sp_cost.clone(),
                range: skill.range,
                look: skill.look,
                icon: skill.icon.as_ref().map(|file| plugin_ui::plugin_asset_path(plugin, file)),
            });
        }
    }
    let resolve = |plugin: &str, job: u32, name: &str| {
        skill_ids.get(name).copied().ok_or_else(|| format!("Plugin {plugin} is refused: job {job} names skill {name}, which neither the game nor a plugin defines"))
    };
    let mut job_owners: HashMap<u32, &str> = HashMap::new();
    let mut jobs = Vec::new();
    for (plugin, _, plugin_jobs) in plugins {
        for job in plugin_jobs.iter() {
            if let Some(other) = job_owners.insert(job.id, plugin) {
                return Err(format!("Plugins {other} and {plugin} both define job {}", job.id));
            }
            let mut tree = Vec::with_capacity(job.skills.len());
            for tree_skill in &job.skills {
                let (skill, max_level) = resolve(plugin, job.id, &tree_skill.skill)?;
                if tree_skill.max_level > max_level {
                    return Err(format!("Plugin {plugin} is refused: job {} learns {} up to level {}, past its {max_level}", job.id, tree_skill.skill, tree_skill.max_level));
                }
                let requires = tree_skill
                    .requires
                    .iter()
                    .map(|requirement| resolve(plugin, job.id, &requirement.skill).map(|(skill, _)| SkillRequirement { skill, level: requirement.level }))
                    .collect::<Result<_, _>>()?;
                tree.push(CatalogTreeSkill { skill, max_level: tree_skill.max_level, requires });
            }
            jobs.push(CatalogJob {
                id: job.id,
                plugin: plugin.to_string(),
                name: job.name.clone(),
                display_name: job.display_name.clone(),
                base_job: JobName::from_string_ignore_case(&job.base_job).value() as u32,
                skills: tree,
            });
        }
    }
    skills.sort_by_key(|skill| skill.id);
    jobs.sort_by_key(|job| job.id);
    Ok((skills, jobs))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::OnceLock;

    use configuration::configuration::Config;
    use serde_json::json;

    use super::*;

    fn game_skills() -> &'static HashMap<u32, SkillConfig> {
        static SKILLS: OnceLock<HashMap<u32, SkillConfig>> = OnceLock::new();
        SKILLS.get_or_init(|| Config::load_skills_config(concat!(env!("CARGO_MANIFEST_DIR"), "/..")).unwrap())
    }

    /// A plugin folder under the temp directory holding these `skills.json` and `jobs.json`.
    fn folder(test: &str, skills: serde_json::Value, jobs: serde_json::Value) -> PathBuf {
        let folder = std::env::temp_dir().join(format!("rust-ro-plugin-skills-{}-{test}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("skills.json"), skills.to_string()).unwrap();
        std::fs::write(folder.join("jobs.json"), jobs.to_string()).unwrap();
        folder
    }

    fn skill(id: u32, name: &str) -> serde_json::Value {
        json!({ "id": id, "name": name, "display_name": name, "max_level": 5, "target": "enemy", "sp_cost": [10, 12, 14, 16, 18], "look": 5 })
    }

    fn job(id: u32, tree: serde_json::Value) -> serde_json::Value {
        json!({ "id": id, "name": "Rune_Fencer", "display_name": "Rune Fencer", "base_job": "Swordsman", "skills": tree })
    }

    #[test]
    fn a_plugin_without_skills_or_jobs_declares_none() {
        let folder = std::env::temp_dir().join(format!("rust-ro-plugin-skills-{}-empty", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        assert!(load_skills(&folder).unwrap().is_empty() && load_jobs(&folder).unwrap().is_empty());
    }

    #[test]
    fn jobs_name_their_tree_by_the_skills_of_the_game_and_of_every_plugin() {
        let first = folder("first", json!([skill(20000, "RF_RUNE_STRIKE")]), json!([]));
        let tree = json!([
            { "skill": "SM_BASH", "max_level": 10 },
            { "skill": "RF_RUNE_STRIKE", "max_level": 5, "requires": [{ "skill": "SM_BASH", "level": 5 }] },
            { "skill": "RF_WARD", "max_level": 3, "requires": [{ "skill": "RF_RUNE_STRIKE", "level": 1 }] },
        ]);
        let second = folder("second", json!([skill(20001, "RF_WARD")]), json!([job(4500, tree)]));
        let (first_skills, second_skills) = (load_skills(&first).unwrap(), load_skills(&second).unwrap());
        let second_jobs = load_jobs(&second).unwrap();
        let (skills, jobs) = catalog(&[("first", &first_skills, &[]), ("second", &second_skills, &second_jobs)], game_skills()).unwrap();
        assert_eq!(skills.iter().map(|skill| (skill.id, skill.plugin.as_str(), skill.target)).collect::<Vec<_>>(), [
            (20000, "first", SkillTarget::Enemy),
            (20001, "second", SkillTarget::Enemy)
        ]);
        let job = &jobs[0];
        assert_eq!((job.id, job.base_job), (4500, JobName::Swordsman.value() as u32));
        let tree: Vec<_> = job.skills.iter().map(|skill| (skill.skill, skill.max_level, skill.requires.iter().map(|req| (req.skill, req.level)).collect::<Vec<_>>())).collect();
        assert_eq!(tree, [(5, 10, vec![]), (20000, 5, vec![(5, 5)]), (20001, 3, vec![(20000, 1)])]);
    }

    #[test]
    fn declarations_that_do_not_hold_are_refused() {
        for (test, skills, expected) in [
            ("range", json!([skill(10000, "RF_X")]), "outside the ids"),
            ("twice", json!([skill(20000, "RF_X"), skill(20000, "RF_Y")]), "defined twice"),
            ("name", json!([skill(20000, "rf_x")]), "upper case"),
            ("level", json!([{ "id": 20000, "name": "RF_X", "display_name": "x", "max_level": 11 }]), "max_level"),
            ("sp", json!([{ "id": 20000, "name": "RF_X", "display_name": "x", "max_level": 2, "sp_cost": [1] }]), "sp_cost"),
            ("icon", json!([{ "id": 20000, "name": "RF_X", "display_name": "x", "max_level": 1, "icon": "x.png" }]), "not a file"),
            ("field", json!([{ "id": 20000, "name": "RF_X", "display_name": "x", "max_level": 1, "damage": 3 }]), "unknown field"),
        ] {
            let error = load_skills(&folder(test, skills, json!([]))).unwrap_err();
            assert!(error.contains(expected), "{test}: {error}");
        }
        for (test, jobs, expected) in [
            ("job-range", json!([job(4000, json!([]))]), "outside the ids"),
            ("job-twice", json!([job(4500, json!([])), job(4500, json!([]))]), "defined twice"),
            ("job-base", json!([{ "id": 4500, "name": "X", "display_name": "X", "base_job": "Pirate" }]), "not a job of the game"),
            ("job-level", json!([job(4500, json!([{ "skill": "SM_BASH", "max_level": 0 }]))]), "max_level"),
        ] {
            let error = load_jobs(&folder(test, json!([]), jobs)).unwrap_err();
            assert!(error.contains(expected), "{test}: {error}");
        }
    }

    #[test]
    fn the_catalog_refuses_trees_and_skills_it_cannot_resolve() {
        let refused = |test: &str, skills: serde_json::Value, jobs: serde_json::Value| {
            let folder = folder(test, skills, jobs);
            let (skills, jobs) = (load_skills(&folder).unwrap(), load_jobs(&folder).unwrap());
            catalog(&[("p", &skills, &jobs)], game_skills()).unwrap_err()
        };
        assert!(refused("unknown", json!([]), json!([job(4500, json!([{ "skill": "RF_NOPE", "max_level": 1 }]))])).contains("neither the game nor a plugin"));
        assert!(refused("past", json!([skill(20000, "RF_X")]), json!([job(4500, json!([{ "skill": "RF_X", "max_level": 6 }]))])).contains("past its 5"));
        assert!(refused("taken", json!([skill(20000, "SM_BASH")]), json!([])).contains("already taken"));
        let looks_like_nothing = json!([{ "id": 20000, "name": "RF_X", "display_name": "x", "max_level": 1, "look": 999999 }]);
        assert!(refused("look", looks_like_nothing, json!([])).contains("not a skill of the game"));
    }
}
