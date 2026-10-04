use std::collections::BTreeMap;

use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use models::skill_grant::ScriptSkillGrant;
use models::status::{KnownSkill, Status};
use script_sdk::{Reply, Value};

use crate::repository::script_character_repository::{
    ScriptExperiencePlan, ScriptLevelResetPlan, ScriptSkillGrantMutation, ScriptSkillResetPlan,
};
use crate::server::Server;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::game_systems::PlayerOption;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;

const MAX_SCRIPT_SKILL_LEVEL: u8 = 13;

pub fn refresh_rank_status(character: &mut Character, ranked_ids: &[u32]) -> bool {
    let ranked = character.status.job == JobName::Taekwon.value() as u32 && ranked_ids.contains(&character.char_id);
    let changed = character.status.taekwon_ranked != ranked;
    character.status.taekwon_ranked = ranked;
    changed
}

pub fn taekwon_rank_active(status: &Status) -> bool {
    status.taekwon_ranked && status.job == JobName::Taekwon.value() as u32 && status.base_level >= 90
}

pub fn taekwon_rank_skill_grants(status: &Status) -> Vec<KnownSkill> {
    if !taekwon_rank_active(status) || status.skill_point != 0 {
        return vec![];
    }
    let configuration = GlobalConfigService::instance();
    configuration
        .get_job_skilltree(JobName::Taekwon)
        .tree()
        .iter()
        .filter_map(|entry| {
            let value = SkillEnum::from_name(entry.name());
            let metadata = crate::server::script::skill::metadata::SkillMetadata::find(value.id())?;
            if value == SkillEnum::NvBasic
                || value.is_platinium()
                || metadata.flags.get("IsQuest").copied().unwrap_or(false)
                || metadata.flags.get("IsWedding").copied().unwrap_or(false)
            {
                return None;
            }
            Some(KnownSkill {
                value,
                level: metadata.max_level,
            })
        })
        .collect()
}

pub fn plan_raw_experience(character: &Character, base: u32, job: u32) -> Result<ScriptExperiencePlan, String> {
    let config = GlobalConfigService::instance();
    let kind = JobName::try_from_value(character.status.job as usize).map_err(|_| "Unknown classic job")?;
    let mut status = character.status.clone();
    let expected = [
        status.base_level,
        status.job_level,
        status.base_exp,
        status.job_exp,
        status.status_point,
        status.skill_point,
        status.hp,
        status.sp,
    ];
    let requirements = &config.config().game.exp_requirements;
    let base_requirements = if kind.is_rebirth() {
        &requirements.base_next_level_requirement.transcendent
    } else {
        &requirements.base_next_level_requirement.normal
    };
    let jobs = &requirements.job_next_level_requirement;
    let job_requirements = if kind.is_taekwon() {
        &jobs.taekwon_class
    } else if kind.is_gunslinger_ninja() {
        &jobs.gunslinger_class
    } else if kind.is_rebirth() {
        if kind.is_novice() {
            &jobs.transcended_novice
        } else if kind.is_first_class() {
            &jobs.transcended_first_class
        } else {
            &jobs.transcended_second_class
        }
    } else if kind.is_novice() {
        &jobs.novice
    } else if kind.is_first_class() {
        &jobs.first_class
    } else {
        &jobs.second_class
    };
    let max_base = config
        .config()
        .game
        .max_base_level
        .min(99)
        .min(base_requirements.len() as u32 + 1)
        .max(1);
    let max_job = u32::from(config.get_job_config(status.job).job_level().max_job_level())
        .min(job_requirements.len() as u32 + 1)
        .max(1);
    if status.hp > 0 {
        let base_before = status.base_level;
        let job_before = status.job_level;
        let (base_level, base_exp) = grant_level_experience(status.base_level, status.base_exp, base, max_base, base_requirements)?;
        let (job_level, job_exp) = grant_level_experience(status.job_level, status.job_exp, job, max_job, job_requirements)?;
        status.base_level = base_level;
        status.base_exp = base_exp;
        status.job_level = job_level;
        status.job_exp = job_exp;
        for level in base_before..base_level {
            let reward = config
                .config()
                .game
                .status_point_rewards
                .iter()
                .find(|reward| reward.level_min as u32 <= level && level <= reward.level_max as u32)
                .ok_or("Missing configured stat point reward")?
                .reward as u32;
            status.status_point = status.status_point.checked_add(reward).ok_or("Stat point overflow")?;
        }
        status.skill_point = status
            .skill_point
            .checked_add(job_level.saturating_sub(job_before))
            .ok_or("Skill point overflow")?;
        let snapshot = StatusService::instance().to_snapshot(&status);
        status.max_hp = snapshot.max_hp();
        status.max_sp = snapshot.max_sp();
        if base_level > base_before {
            status.hp = status.max_hp;
            status.sp = status.max_sp;
        } else {
            status.hp = status.hp.min(status.max_hp);
            status.sp = status.sp.min(status.max_sp);
        }
    }
    if status.status_point > i16::MAX as u32 || status.skill_point > i16::MAX as u32 {
        return Err("Experience grant exceeds persistent point limits".into());
    }
    Ok(ScriptExperiencePlan {
        expected_job: status.job,
        expected,
        base_level: status.base_level,
        job_level: status.job_level,
        base_exp: status.base_exp,
        job_exp: status.job_exp,
        status_points: status.status_point,
        skill_points: status.skill_point,
        hp: status.hp,
        sp: status.sp,
        max_hp: status.max_hp,
        max_sp: status.max_sp,
    })
}

fn grant_level_experience(mut level: u32, exp: u32, gain: u32, max_level: u32, requirements: &[u32]) -> Result<(u32, u32), String> {
    if level == 0 || level > max_level {
        return Err("Character level is outside the classic range".into());
    }
    let mut remaining = u64::from(exp) + u64::from(gain);
    while level < max_level {
        let requirement = u64::from(*requirements.get(level as usize - 1).ok_or("Missing level experience requirement")?);
        if requirement == 0 {
            return Err("Level experience requirement must be positive".into());
        }
        if remaining < requirement {
            break;
        }
        remaining -= requirement;
        level += 1;
    }
    Ok((
        level,
        if level == max_level {
            0
        } else {
            remaining.min(i32::MAX as u64) as u32
        },
    ))
}

pub fn apply_experience(server: &Server, character: &mut Character, plan: &ScriptExperiencePlan) {
    let base_up = plan.base_level > character.status.base_level;
    let job_up = plan.job_level > character.status.job_level;
    character.status.base_level = plan.base_level;
    character.status.job_level = plan.job_level;
    character.status.base_exp = plan.base_exp;
    character.status.job_exp = plan.job_exp;
    character.status.status_point = plan.status_points;
    character.status.skill_point = plan.skill_points;
    character.status.max_hp = plan.max_hp;
    character.status.max_sp = plan.max_sp;
    character.status.hp = plan.hp;
    character.status.sp = plan.sp;
    if base_up || job_up {
        use models::enums::effect::Effect;
        use packets::packets::{Packet, PacketZcNotifyEffect};
        for effect in [(base_up, Effect::BaseLevelUp), (job_up, Effect::JobLevelUp)] {
            if !effect.0 {
                continue;
            }
            let mut packet = PacketZcNotifyEffect::new(GlobalConfigService::instance().packetver());
            packet.set_aid(character.char_id);
            packet.set_effect_id(effect.1.value() as i32);
            packet.fill_raw();
            let notification = crate::server::model::events::client_notification::AreaNotification::new(
                character.current_map_name().clone(),
                character.current_map_instance(),
                crate::server::model::events::client_notification::AreaNotificationRangeType::Fov {
                    x: character.x,
                    y: character.y,
                    exclude_id: None,
                },
                packet.raw,
            );
            let _ = server.server_service().notification_sender().send(Notification::Area(notification));
        }
        server.skill_tree_service().send_skill_tree(character);
    }
    server.character_service().reload_client_side_status(character);
    notify_progression(server, character);
}

pub fn get_fame(server: &Server, character: &Character, rank: bool) -> Reply {
    let value = if rank {
        let Some(category) = crate::repository::fame_repository::FameCategory::for_job(character.status.job) else {
            return Ok(0.into());
        };
        u32::from(
            server
                .repository
                .fame_rank(category, character.char_id)
                .map_err(|error| error.to_string())?,
        )
    } else {
        server
            .repository
            .character_fame(character.char_id)
            .map_err(|error| error.to_string())?
    };
    Ok((value as i32).into())
}

pub fn add_fame(server: &Server, character: &mut Character, arguments: &[Value]) -> Reply {
    let amount = arguments.first().ok_or("Missing fame amount")?.number_value()?;
    let updated = server
        .repository
        .add_fame(character.char_id, character.account_id, amount)
        .map_err(|error| error.to_string())?;
    server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::FameChanged(
        crate::server::model::events::game_event::FameChanged {
            category: updated.category,
            ranked_creators: updated.rankings.iter().map(|entry| entry.char_id).collect(),
        },
    ));
    Ok(Value::default())
}

pub fn begin_taekwon_mission(server: &Server, character: &mut Character) -> Result<bool, String> {
    let current = server
        .repository
        .taekwon_mission(character.char_id)
        .map_err(|error| error.to_string())?;
    if let Some(mission) = current.filter(|mission| mission.kills > 0 || fastrand::u32(0..100) != 0) {
        notify_mission(server, character, mission)?;
        return Ok(false);
    }
    let target = mission_target(character.status.base_level, false)?;
    let mission = server
        .repository
        .set_taekwon_mission(character.char_id, character.account_id, current, target)
        .map_err(|error| error.to_string())?;
    notify_mission(server, character, mission)?;
    Ok(true)
}

pub fn record_mission_kill(server: &Server, character: &mut Character, mob_id: u32) -> Result<(), String> {
    let Some(mission) = server
        .repository
        .taekwon_mission(character.char_id)
        .map_err(|error| error.to_string())?
    else {
        return Ok(());
    };
    if mission.mob_id != mob_id {
        return Ok(());
    }
    let next = if mission.kills >= 99 {
        Some(mission_target(character.status.base_level, true)?)
    } else {
        None
    };
    let Some(outcome) = server
        .repository
        .record_taekwon_kill(character.char_id, character.account_id, mob_id, next)
        .map_err(|error| error.to_string())?
    else {
        return Ok(());
    };
    if let Some(fame) = outcome.fame {
        server.add_to_next_tick(crate::server::model::events::game_event::GameEvent::FameChanged(
            crate::server::model::events::game_event::FameChanged {
                category: fame.category,
                ranked_creators: fame.rankings.iter().map(|entry| entry.char_id).collect(),
            },
        ));
    }
    notify_mission(server, character, outcome.mission)
}

fn mission_target(base_level: u32, completion: bool) -> Result<u32, String> {
    let configuration = GlobalConfigService::instance();
    let name = if completion { "BRANCH_OF_DEAD_TREE" } else { "TAEKWON_MISSION" };
    let group = crate::server::script::game_data::data()
        .summons
        .iter()
        .find(|group| group.name == name)
        .ok_or("Classic mission target group is unavailable")?;
    let spawned: std::collections::HashSet<_> = configuration
        .maps()
        .values()
        .flat_map(|map| map.mob_spawns().iter().map(|spawn| spawn.mob_id as u32))
        .collect();
    use models::enums::EnumWithMaskValueU32;
    let targets: Vec<_> = group
        .entries
        .iter()
        .filter_map(|entry| {
            let mob = configuration.get_mob_safe(entry.mob_id as i32)?;
            (!completion
                || mob.level as u32 <= base_level
                    && mob.mode as u32 & models::enums::mob::MobMode::Boss.as_flag() == 0
                    && spawned.contains(&entry.mob_id))
            .then_some(entry.mob_id)
        })
        .collect();
    if !targets.is_empty() {
        return Ok(targets[fastrand::usize(0..targets.len())]);
    }
    configuration
        .get_mob_safe(group.default as i32)
        .map(|_| group.default)
        .ok_or("Classic mission default monster is unavailable".into())
}

fn notify_mission(
    server: &Server,
    character: &Character,
    mission: crate::repository::fame_repository::TaekwonMission,
) -> Result<(), String> {
    let mob = GlobalConfigService::instance()
        .get_mob_safe(mission.mob_id as i32)
        .ok_or("Mission monster is unavailable")?;
    let mut packet = 0x020E_u16.to_le_bytes().to_vec();
    let mut name = [0_u8; 24];
    let bytes = mob.name_english.as_bytes();
    let length = bytes.len().min(23);
    name[..length].copy_from_slice(&bytes[..length]);
    packet.extend_from_slice(&name);
    packet.extend_from_slice(&mission.mob_id.to_le_bytes());
    packet.push(mission.kills);
    packet.push(20);
    server
        .server_service()
        .notification_sender()
        .send(Notification::Char(CharNotification::new(character.char_id, packet)))
        .map_err(|error| error.to_string())
}

pub fn learned_level(status: &Status, id: u32) -> u8 {
    status
        .script_skill_grants
        .get(&id)
        .map(|grant| grant.learned_level)
        .unwrap_or_else(|| {
            status
                .known_skills
                .iter()
                .find(|skill| skill.value.id() == id)
                .map_or(0, |skill| skill.level)
        })
}

pub fn persisted_skills(status: &Status) -> BTreeMap<u32, u8> {
    status
        .known_skills
        .iter()
        .filter_map(|skill| {
            let level = learned_level(status, skill.value.id());
            (level > 0).then_some((skill.value.id(), level))
        })
        .collect()
}

pub fn allocated_skill_points(character: &Character) -> u32 {
    character
        .status
        .known_skills
        .iter()
        .filter(|skill| !skill.value.is_platinium() && !character.game_systems.permanent_skill_grants.contains_key(&skill.value.id()))
        .map(|skill| learned_level(&character.status, skill.value.id()) as u32)
        .sum()
}

fn set_known_skill(status: &mut Status, value: SkillEnum, level: u8) {
    status.known_skills.retain(|skill| skill.value != value);
    if level > 0 {
        status.known_skills.push(KnownSkill { value, level });
    }
}

pub fn grant_temporary_skill(status: &mut Status, value: SkillEnum, level: u8, additive: bool) {
    let id = value.id();
    let previous = status
        .known_skills
        .iter()
        .find(|skill| skill.value == value)
        .map_or(0, |skill| skill.level);
    let current_level = if additive {
        previous.saturating_add(level).min(MAX_SCRIPT_SKILL_LEVEL)
    } else {
        previous.max(level)
    };
    let learned = learned_level(status, id);
    status.script_skill_grants.insert(id, ScriptSkillGrant {
        learned_level: learned,
        current_level,
    });
    set_known_skill(status, value, current_level);
}

pub fn plan_reset_skills(character: &Character, refund_points: bool) -> Result<ScriptSkillResetPlan, String> {
    let job = JobName::try_from_value(character.status.job as usize).map_err(|_| "Unknown character job")?;
    let expected_skills = persisted_skills(&character.status);
    let mut remaining = BTreeMap::new();
    let mut known_skills = Vec::new();
    let mut temporary_grants = BTreeMap::new();
    let mut refund = 0_u32;
    for skill in &character.status.known_skills {
        let id = skill.value.id();
        let granted = character.game_systems.permanent_skill_grants.contains_key(&id);
        let keep_basic = skill.value == SkillEnum::NvBasic && !job.is_novice();
        let lost_trick_dead = skill.value == SkillEnum::NvTrickdead && !job.is_novice() && !granted;
        let keep = !lost_trick_dead && (granted || keep_basic || skill.value.is_platinium());
        let learned = learned_level(&character.status, id);
        if keep {
            known_skills.push(*skill);
            if learned > 0 {
                remaining.insert(id, learned);
            }
            if let Some(grant) = character.status.script_skill_grants.get(&id) {
                temporary_grants.insert(id, *grant);
            }
        } else if !skill.value.is_platinium() && !granted {
            refund = refund.checked_add(u32::from(learned)).ok_or("Skill point refund overflow")?;
        }
    }
    let skill_points = character
        .status
        .skill_point
        .checked_add(if refund_points { refund } else { 0 })
        .ok_or("Skill point refund overflow")?;
    if skill_points > i16::MAX as u32 {
        return Err("Skill point refund exceeds the persistent limit".into());
    }
    let has = |value: SkillEnum| known_skills.iter().any(|skill| skill.value == value && skill.level > 0);
    let mut options = character.options;
    if !has(SkillEnum::KnRiding) {
        options &= !PlayerOption::Riding.as_flag();
    }
    if !has(SkillEnum::HtFalcon) {
        options &= !PlayerOption::Falcon.as_flag();
    }
    if !has(SkillEnum::McPushcart) {
        options &= !(PlayerOption::Cart1.as_flag()
            | PlayerOption::Cart2.as_flag()
            | PlayerOption::Cart3.as_flag()
            | PlayerOption::Cart4.as_flag()
            | PlayerOption::Cart5.as_flag());
    }
    Ok(ScriptSkillResetPlan {
        expected_skills,
        persisted_skills: remaining,
        permanent_grants: character.game_systems.permanent_skill_grants.clone(),
        known_skills,
        temporary_grants,
        refund,
        expected_skill_points: character.status.skill_point,
        skill_points,
        options,
    })
}

pub fn apply_reset_skills(server: &Server, character: &mut Character, plan: &ScriptSkillResetPlan) {
    character.status.known_skills = plan.known_skills.clone();
    character.status.script_skill_grants = plan.temporary_grants.clone();
    character.status.skill_point = plan.skill_points;
    let changed_options = character.options != plan.options;
    character.options = plan.options;
    if changed_options {
        notify_options(server, character);
    }
    server.skill_tree_service().send_skill_tree(character);
    server.character_service().reload_client_side_status(character);
}

pub fn grant_skill(server: &Server, character: &mut Character, arguments: &[Value]) -> Reply {
    let config = GlobalConfigService::instance()
        .find_skill_config(arguments.first().ok_or("Missing skill")?)
        .ok_or("Unknown pre-renewal skill")?;
    let value = SkillEnum::try_from_value(config.id).map_err(|_| "Unknown skill ID")?;
    let level = u8::try_from(arguments.get(1).ok_or("Missing skill level")?.number_value()?).map_err(|_| "Invalid skill level")?;
    if level > MAX_SCRIPT_SKILL_LEVEL {
        return Err("Script skill level exceeds the classic engine limit".into());
    }
    let flag = arguments.get(2).map(Value::number_value).transpose()?.unwrap_or(1);
    if !(0..=3).contains(&flag) {
        return Err("Invalid skill grant flag".into());
    }
    let id = value.id();
    match flag {
        0 | 3 => {
            let mutation = ScriptSkillGrantMutation {
                skill_id: id,
                level,
                expected_level: learned_level(&character.status, id),
                permanent: flag == 3,
                expected_grants: character.game_systems.permanent_skill_grants.clone(),
            };
            let systems = server
                .repository
                .character_script_grant_skill(character.char_id, character.account_id, &mutation)
                .map_err(|error| error.to_string())?;
            character.game_systems.permanent_skill_grants = systems.permanent_skill_grants;
            character.game_systems.revision = systems.revision;
            character.status.script_skill_grants.remove(&id);
            set_known_skill(&mut character.status, value, level);
        }
        1 | 2 => {
            grant_temporary_skill(&mut character.status, value, level, flag == 2);
        }
        _ => unreachable!(),
    }
    server.skill_tree_service().send_skill_tree(character);
    server.character_service().reload_client_side_status(character);
    Ok(Value::default())
}

pub fn plan_level_reset(character: &Character, action: u8) -> Result<(Status, ScriptLevelResetPlan), String> {
    if !(1..=4).contains(&action) {
        return Err("Level reset action must be 1, 2, 3 or 4".into());
    }
    let mut staged = character.status.clone();
    staged.takeoff_all_equipment();
    staged.active_auto_bonuses.clear();
    staged.bonus_periodic_ticks.clear();
    let mut additional_skills = BTreeMap::new();
    let mut reset_skills = None;
    if action == 1 {
        let mut reset = plan_reset_skills(character, false)?;
        reset.skill_points = 0;
        reset.options = 0;
        staged.known_skills = reset.known_skills.clone();
        staged.script_skill_grants = reset.temporary_grants.clone();
        staged.str = 1;
        staged.agi = 1;
        staged.vit = 1;
        staged.int = 1;
        staged.dex = 1;
        staged.luk = 1;
        staged.status_point = if JobName::from_value(staged.job as usize).is_rebirth() {
            100
        } else {
            48
        };
        staged.state = 0;
        use models::status_change::StatusChangeKind::*;
        staged.active_statuses.retain(|change| {
            !matches!(
                change.kind,
                Sight | Hiding | Cloaking | Orcish | Wedding | Ruwach | Christmas | Summer
            )
        });
        if staged.job == JobName::NoviceHigh.value() as u32 {
            for skill in [SkillEnum::NvFirstaid, SkillEnum::NvTrickdead] {
                if !character.game_systems.permanent_skill_grants.contains_key(&skill.id()) {
                    set_known_skill(&mut staged, skill, 1);
                    additional_skills.insert(skill.id(), 1);
                }
            }
        }
        reset_skills = Some(reset);
    }
    if action <= 3 {
        staged.base_level = 1;
        staged.base_exp = 0;
    }
    if action != 3 {
        staged.job_level = 1;
        staged.job_exp = 0;
    }
    if action <= 2 {
        staged.skill_point = 0;
    }
    let snapshot = StatusService::instance().to_snapshot(&staged);
    staged.max_hp = snapshot.max_hp();
    staged.max_sp = snapshot.max_sp();
    staged.hp = staged.hp.min(staged.max_hp);
    staged.sp = staged.sp.min(staged.max_sp);
    let options = if action == 1 { 0 } else { character.options };
    let plan = ScriptLevelResetPlan {
        action,
        expected_class: staged.job,
        base_level: staged.base_level,
        job_level: staged.job_level,
        base_exp: staged.base_exp,
        job_exp: staged.job_exp,
        skill_points: staged.skill_point,
        status_points: staged.status_point,
        attributes: [staged.str, staged.agi, staged.vit, staged.int, staged.dex, staged.luk],
        options,
        hp: staged.hp,
        sp: staged.sp,
        max_hp: staged.max_hp,
        max_sp: staged.max_sp,
        reset_skills,
        additional_skills,
    };
    Ok((staged, plan))
}

pub fn reset_level(server: &Server, character: &mut Character, arguments: &[Value]) -> Reply {
    let action =
        u8::try_from(arguments.first().ok_or("Missing level reset action")?.number_value()?).map_err(|_| "Invalid level reset action")?;
    let (staged, plan) = plan_level_reset(character, action)?;
    let systems = server
        .repository
        .character_script_reset_level(character.char_id, character.account_id, &plan)
        .map_err(|error| error.to_string())?;
    let equipped: Vec<_> = character
        .inventory
        .iter()
        .enumerate()
        .filter_map(|(index, item)| item.as_ref().filter(|item| item.equip != 0).map(|_| index))
        .collect();
    for index in equipped {
        if let Some(item) = character.takeoff_equip_item(index) {
            let mut packet = 0x00AC_u16.to_le_bytes().to_vec();
            packet.extend_from_slice(&(index as u16).to_le_bytes());
            packet.extend_from_slice(&(item.location as u16).to_le_bytes());
            packet.push(0);
            let _ = server
                .server_service()
                .notification_sender()
                .send(Notification::Char(CharNotification::new(character.char_id, packet)));
        }
    }
    let removed_statuses: Vec<_> = character
        .status
        .active_statuses
        .iter()
        .filter(|change| !staged.active_statuses.iter().any(|remaining| remaining.kind == change.kind))
        .map(|change| change.kind)
        .collect();
    character.status = staged;
    character.options = plan.options;
    character.game_systems.mounting = systems.mounting;
    character.game_systems.revision = systems.revision;
    character.attack = None;
    character.movements.clear();
    character.skill_in_use = None;
    character.pending_skill = None;
    character.pending_item_skill = None;
    character.pending_craft = None;
    server.script_skill_service().cancel_queued_cast(character);
    character.action = if character.status.hp == 0 {
        crate::server::state::character::CharacterAction::Dead
    } else {
        crate::server::state::character::CharacterAction::Idle
    };
    server.inventory_service().reload_equipped_item_sprites(character);
    let sender = server.server_service().notification_sender();
    for kind in removed_statuses {
        crate::server::service::status_effect_service::StatusEffectService::send_icon(
            character,
            kind,
            false,
            crate::util::tick::get_tick(),
            &sender,
        );
    }
    notify_options(server, character);
    server.skill_tree_service().send_skill_tree(character);
    server.character_service().reload_client_side_status(character);
    notify_progression(server, character);
    Ok(Value::default())
}

fn notify_progression(server: &Server, character: &Character) {
    use models::enums::status::StatusTypes;
    use packets::packets::{Packet, PacketZcParChange};
    let mut raw = Vec::new();
    for (kind, value) in [
        (StatusTypes::Baselevel, character.status.base_level),
        (StatusTypes::Joblevel, character.status.job_level),
        (StatusTypes::Baseexp, character.status.base_exp),
        (StatusTypes::Jobexp, character.status.job_exp),
    ] {
        let mut packet = PacketZcParChange::new(GlobalConfigService::instance().packetver());
        packet.set_var_id(kind.value() as u16);
        packet.set_count(value as i32);
        packet.fill_raw();
        raw.extend(packet.raw);
    }
    let _ = server
        .server_service()
        .notification_sender()
        .send(Notification::Char(CharNotification::new(character.char_id, raw)));
}

fn notify_options(server: &Server, character: &Character) {
    let mut visual_status = character.status.clone();
    visual_status.state = character.options;
    let packet = crate::server::service::status_effect_service::StatusEffectService::visual_state_packet(character.char_id, &visual_status);
    let sender = server.server_service().notification_sender();
    let notification = crate::server::model::events::client_notification::AreaNotification::new(
        character.current_map_name().to_string(),
        character.current_map_instance(),
        crate::server::model::events::client_notification::AreaNotificationRangeType::Fov {
            x: character.x,
            y: character.y,
            exclude_id: None,
        },
        packet,
    );
    let _ = sender.send(Notification::Area(notification));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::common;

    fn character() -> Character {
        common::before_all();
        StatusService::init(GlobalConfigService::instance(), common::test_script_vm());
        let mut character = common::character_helper::create_character();
        character.status.base_level = 40;
        character.status.job_level = 20;
        character.status.base_exp = 123;
        character.status.job_exp = 456;
        character.status.skill_point = 7;
        character.status.status_point = 11;
        character.status.str = 20;
        character.status.int = 15;
        character.status.known_skills = vec![
            KnownSkill {
                value: SkillEnum::NvBasic,
                level: 5,
            },
            KnownSkill {
                value: SkillEnum::MgFirebolt,
                level: 3,
            },
        ];
        character
    }

    #[test]
    fn taekwon_rank_effects_begin_at_ninety_and_demote_without_changing_learned_skills() {
        let mut character = character();
        character.status.job = JobName::Taekwon.value() as u32;
        character.status.base_level = 89;
        character.status.skill_point = 0;
        character.status.known_skills = vec![
            KnownSkill {
                value: SkillEnum::NvBasic,
                level: 9,
            },
            KnownSkill {
                value: SkillEnum::TkRun,
                level: 3,
            },
        ];
        let learned = character.status.known_skills.clone();
        let ordinary = StatusService::instance().to_snapshot(&character.status);
        let char_id = character.char_id;
        assert!(refresh_rank_status(&mut character, &[char_id]));
        assert!(character.status.taekwon_ranked);
        assert!(!taekwon_rank_active(&character.status));
        let below = StatusService::instance().to_snapshot(&character.status);
        assert_eq!((below.max_hp(), below.max_sp()), (ordinary.max_hp(), ordinary.max_sp()));
        assert!(taekwon_rank_skill_grants(&character.status).is_empty());
        character.status.base_level = 90;
        character.status.taekwon_ranked = false;
        let ordinary = StatusService::instance().to_snapshot(&character.status);
        refresh_rank_status(&mut character, &[char_id]);
        let ranked = StatusService::instance().to_snapshot(&character.status);
        let job = GlobalConfigService::instance().get_job_config(character.status.job);
        let expected_sp = (job.base_sp()[89] as f32 * (1.0 + character.status.int as f32 / 100.0) * 3.0).floor() as u32;
        assert_eq!((ranked.max_hp(), ranked.max_sp()), (ordinary.max_hp() * 3, expected_sp));
        assert_eq!(
            ranked
                .known_skills()
                .iter()
                .find(|skill| skill.value == SkillEnum::TkRun)
                .unwrap()
                .level,
            10
        );
        let (sender, _) = std::sync::mpsc::sync_channel(64);
        let tree = crate::server::service::character::skill_tree_service::SkillTreeService::new(sender, GlobalConfigService::instance());
        assert_eq!(
            tree.skill_tree(&character)
                .iter()
                .find(|skill| skill.value == SkillEnum::TkRun)
                .unwrap()
                .level,
            10
        );
        assert_eq!(allocated_skill_points(&character), 12);
        assert_eq!(plan_reset_skills(&character, true).unwrap().refund, 3);
        assert!(refresh_rank_status(&mut character, &[]));
        let demoted = StatusService::instance().to_snapshot(&character.status);
        assert_eq!((demoted.max_hp(), demoted.max_sp()), (ordinary.max_hp(), ordinary.max_sp()));
        assert_eq!(
            demoted
                .known_skills()
                .iter()
                .find(|skill| skill.value == SkillEnum::TkRun)
                .unwrap()
                .level,
            3
        );
        assert_eq!(character.status.known_skills, learned);
    }

    #[test]
    fn taekwon_maximum_skill_grants_wait_for_spent_points_and_rank_effects_end_on_job_change() {
        let mut character = character();
        character.status.job = JobName::Taekwon.value() as u32;
        character.status.base_level = 90;
        character.status.skill_point = 1;
        let id = character.char_id;
        refresh_rank_status(&mut character, &[id]);
        assert!(taekwon_rank_active(&character.status));
        assert!(taekwon_rank_skill_grants(&character.status).is_empty());
        character.status.skill_point = 0;
        assert!(
            taekwon_rank_skill_grants(&character.status)
                .iter()
                .all(|skill| !skill.value.is_platinium() && !skill.value.to_name().starts_with("NV_"))
        );
        assert!(!taekwon_rank_skill_grants(&character.status).is_empty());
        character.status.job = JobName::StarGladiator.value() as u32;
        assert!(!taekwon_rank_active(&character.status));
        assert!(taekwon_rank_skill_grants(&character.status).is_empty());
        assert!(refresh_rank_status(&mut character, &[id]));
        assert!(!character.status.taekwon_ranked);
    }

    #[test]
    fn temporary_max_and_additive_grants_keep_original_learned_level() {
        let mut character = character();
        grant_temporary_skill(&mut character.status, SkillEnum::MgFirebolt, 2, false);
        assert_eq!(
            character
                .status
                .known_skills
                .iter()
                .find(|skill| skill.value == SkillEnum::MgFirebolt)
                .unwrap()
                .level,
            3
        );
        grant_temporary_skill(&mut character.status, SkillEnum::MgFirebolt, 5, false);
        grant_temporary_skill(&mut character.status, SkillEnum::MgFirebolt, 10, true);
        assert_eq!(
            character.status.script_skill_grants[&SkillEnum::MgFirebolt.id()],
            ScriptSkillGrant {
                learned_level: 3,
                current_level: 13
            }
        );
        assert_eq!(persisted_skills(&character.status)[&SkillEnum::MgFirebolt.id()], 3);
        assert_eq!(allocated_skill_points(&character), 8);
    }

    #[test]
    fn reset_refunds_original_levels_preserves_quest_and_permanent_grants_and_free_points() {
        let mut character = character();
        character.game_systems.permanent_skill_grants.insert(SkillEnum::MgFirebolt.id(), 3);
        character.status.known_skills.push(KnownSkill {
            value: SkillEnum::NvFirstaid,
            level: 1,
        });
        grant_temporary_skill(&mut character.status, SkillEnum::NvBasic, 8, true);
        grant_temporary_skill(&mut character.status, SkillEnum::SmBash, 10, false);
        let plan = plan_reset_skills(&character, true).unwrap();
        assert_eq!((plan.refund, plan.skill_points), (5, 12));
        assert_eq!(
            plan.persisted_skills,
            BTreeMap::from([(SkillEnum::MgFirebolt.id(), 3), (SkillEnum::NvFirstaid.id(), 1)])
        );
        assert!(plan.temporary_grants.is_empty());
    }

    #[test]
    fn documented_level_reset_actions_preserve_the_unaffected_fields() {
        let character = character();
        let (two, plan) = plan_level_reset(&character, 2).unwrap();
        assert_eq!(
            (two.base_level, two.job_level, two.base_exp, two.job_exp, two.skill_point),
            (1, 1, 0, 0, 0)
        );
        assert_eq!((two.str, two.int, two.status_point), (20, 15, 11));
        assert_eq!(two.known_skills, character.status.known_skills);
        assert!(plan.reset_skills.is_none());
        let (three, _) = plan_level_reset(&character, 3).unwrap();
        assert_eq!(
            (
                three.base_level,
                three.base_exp,
                three.job_level,
                three.job_exp,
                three.skill_point
            ),
            (1, 0, 20, 456, 7)
        );
        let (four, _) = plan_level_reset(&character, 4).unwrap();
        assert_eq!(
            (four.base_level, four.base_exp, four.job_level, four.job_exp, four.skill_point),
            (40, 123, 1, 0, 7)
        );
    }

    #[test]
    fn novice_high_rebirth_reset_grants_quest_skills_and_one_hundred_stat_points() {
        let mut character = character();
        character.status.job = JobName::NoviceHigh.value() as u32;
        character.game_systems.permanent_skill_grants.insert(SkillEnum::MgFirebolt.id(), 3);
        let (one, plan) = plan_level_reset(&character, 1).unwrap();
        assert_eq!(
            (
                one.base_level,
                one.job_level,
                one.skill_point,
                one.status_point,
                one.str,
                one.int
            ),
            (1, 1, 0, 100, 1, 1)
        );
        assert_eq!(plan.options, 0);
        assert!(one.known_skills.contains(&KnownSkill {
            value: SkillEnum::MgFirebolt,
            level: 3
        }));
        assert!(one.known_skills.contains(&KnownSkill {
            value: SkillEnum::NvFirstaid,
            level: 1
        }));
        assert!(one.known_skills.contains(&KnownSkill {
            value: SkillEnum::NvTrickdead,
            level: 1
        }));
    }

    #[test]
    fn raw_experience_grants_multiple_levels_points_and_base_level_resources_without_rates() {
        let mut character = character();
        character.status.base_level = 1;
        character.status.job_level = 1;
        character.status.base_exp = 0;
        character.status.job_exp = 0;
        character.status.hp = 50;
        character.status.sp = 0;
        character.status.str = 1;
        character.status.int = 1;
        let requirements = &GlobalConfigService::instance().config().game.exp_requirements;
        let base = requirements.base_next_level_requirement.normal[0] + requirements.base_next_level_requirement.normal[1] + 1;
        let job = requirements.job_next_level_requirement.novice[0] + requirements.job_next_level_requirement.novice[1] + 1;
        let plan = plan_raw_experience(&character, base, job).unwrap();
        assert_eq!((plan.base_level, plan.job_level, plan.base_exp, plan.job_exp), (3, 3, 1, 1));
        assert_eq!(plan.skill_points, 9);
        assert_eq!((plan.hp, plan.sp), (plan.max_hp, plan.max_sp));
        assert!(plan.status_points > character.status.status_point);
    }

    #[test]
    fn raw_experience_respects_classic_caps_and_dead_actor_immunity() {
        let mut character = character();
        character.status.hp = 50;
        character.status.base_level = 99;
        character.status.job_level = 10;
        let plan = plan_raw_experience(&character, u32::MAX, u32::MAX).unwrap();
        assert_eq!((plan.base_level, plan.job_level, plan.base_exp, plan.job_exp), (99, 10, 0, 0));
        character.status.hp = 0;
        let dead = plan_raw_experience(&character, u32::MAX, u32::MAX).unwrap();
        assert_eq!(
            (dead.base_level, dead.job_level, dead.base_exp, dead.job_exp),
            (99, 10, 123, 456)
        );
    }
}
