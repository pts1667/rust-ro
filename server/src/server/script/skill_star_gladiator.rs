//! Star Gladiator: the Sun, Moon and Star places and hated monsters, the comfort buffs, anger damage and blessing experience.

use chrono::Datelike;
use models::enums::bonus::BonusType;
use models::enums::class::JobName;
use models::enums::EnumWithNumberValue;
use models::enums::size::Size;
use models::enums::skill_enums::SkillEnum;
use models::status::Status;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::{ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::model::map_flags::MapFlag;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_flag_service::normalize_map;
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;
use crate::server::state::mob::Mob;

const SLOTS: usize = 3;
const STAR_SLOT: usize = 2;
const STARSKILL_PACKET: u16 = 0x020E;
const FEEL_REQUEST_PACKET: u16 = 0x0253;
const NAME_FIELD_LENGTH: usize = 24;
const REGISTER_PLACE: u8 = 1;
const REGISTER_HATE: u8 = 10;
const SHOW_HATE: u8 = 11;
const MEDIUM_HATE_MIN_HP: u32 = 6_000;
const LARGE_HATE_MIN_HP: u32 = 20_000;

const ANGER: [SkillEnum; SLOTS] = [SkillEnum::SgSunAnger, SkillEnum::SgMoonAnger, SkillEnum::SgStarAnger];
const BLESSING: [SkillEnum; SLOTS] = [SkillEnum::SgSunBless, SkillEnum::SgMoonBless, SkillEnum::SgStarBless];

/// The Sun shines on even days of the year, the Moon on odd ones and the Stars every fifth day.
pub(crate) fn is_day(slot: usize, day_of_year: u32) -> bool {
    let day = day_of_year + 1;
    match slot {
        0 => day % 2 == 0,
        1 => day % 2 == 1,
        _ => day % 5 == 0,
    }
}

fn is_today(slot: usize) -> bool {
    is_day(slot, chrono::Local::now().ordinal0())
}

/// Extra damage in percent against the hated monster, `(base level + DEX + LUK)` divided by `12 - 3 * level` for the first three levels.
pub(crate) fn anger_percent(slot: usize, level: u8, base_level: u32, str: u16, dex: u16, luk: u16) -> i32 {
    let mut ratio = (base_level + u32::from(dex) + u32::from(luk) + if slot == STAR_SLOT { u32::from(str) } else { 0 }) as i32;
    if level < 4 {
        ratio /= 12 - 3 * i32::from(level);
    }
    ratio
}

pub(crate) fn anger_bonuses(status: &Status) -> Vec<BonusType> {
    ANGER
        .iter()
        .enumerate()
        .filter_map(|(slot, skill)| {
            let (monster, level) = (status.star_hates[slot], learned_level(status, skill.id()));
            let percent = anger_percent(slot, level, status.base_level, status.str, status.dex, status.luk);
            (monster != 0 && level > 0).then_some(BonusType::PhysicalDamageAgainstMobIdPercentage(monster, percent.clamp(0, i8::MAX as i32) as i8))
        })
        .collect()
}

/// Devil gives 30% attack speed per level, but only once the job is at its maximum level.
pub(crate) fn devil_percent(level: u8, job_level: u32, max_job_level: u32) -> i32 {
    if level > 0 && job_level >= max_job_level { 30 * i32::from(level) } else { 0 }
}

pub(crate) fn devil_bonuses(status: &Status) -> Vec<BonusType> {
    let max_job_level = u32::from(GlobalConfigService::instance().get_job_config(status.job).job_level().max_job_level());
    match devil_percent(learned_level(status, SkillEnum::SgDevil.id()), status.job_level as u32, max_job_level) {
        0 => Vec::new(),
        percent => vec![BonusType::AspdPercentage(percent as f32)],
    }
}

pub(crate) const MIRACLE_ODDS: u32 = 20_000;
pub(crate) const MIRACLE_DURATION_MS: i32 = 3_600_000;
const MIRACLE_AGI_LIMIT: u32 = 46;

/// One attack in `MIRACLE_ODDS` starts Miracle, and above 46 AGI the chance falls to `46 / AGI`.
pub(crate) fn miracle_starts(agi: u32, odds_roll: u32, agi_roll: u32) -> bool {
    odds_roll == 0 && (agi <= MIRACLE_AGI_LIMIT || agi_roll < MIRACLE_AGI_LIMIT)
}

/// Extra experience in percent from a hated monster killed on the day of its slot, the Star slot standing in for all monsters during a Miracle.
pub(crate) fn bless_percent(status: &Status, monster: u32) -> u32 {
    bless_percent_on(status, monster, is_today)
}

fn bless_percent_on(status: &Status, monster: u32, is_day_of: impl Fn(usize) -> bool) -> u32 {
    let slot = if status.has_status_change(StatusChangeKind::Miracle) {
        Some(STAR_SLOT)
    } else {
        (0..SLOTS).find(|slot| status.star_hates[*slot] == monster && monster != 0 && is_day_of(*slot))
    };
    slot.map_or(0, |slot| (if slot == STAR_SLOT { 20 } else { 10 }) * u32::from(learned_level(status, BLESSING[slot].id())))
}

/// Knowledge of the Sun, Moon and Stars raises the weight limit by 10% per level on a remembered map.
pub(crate) fn knowledge_weight_percent(character: &Character) -> u32 {
    let level = u32::from(learned_level(&character.status, SkillEnum::SgKnowledge.id()));
    let here = normalize_map(character.current_map_name());
    if character.game_systems.star_places.contains(&here) { 10 * level } else { 0 }
}

/// A Small monster fills the Sun slot, a Medium one the Moon slot (6000 HP or more) and a Large one the Star slot (20000 HP or more).
pub(crate) fn hate_slot_accepts(slot: usize, size: Size, max_hp: u32) -> bool {
    let (expected, min_hp) = match slot {
        0 => (Size::Small, 0),
        1 => (Size::Medium, MEDIUM_HATE_MIN_HP),
        _ => (Size::Large, LARGE_HATE_MIN_HP),
    };
    size == expected && max_hp >= min_hp
}

/// `ZC_STARSKILL`: a place or a hated monster shown in the client window.
pub(crate) fn starskill_packet(name: &str, monster_id: i32, slot: u8, result: u8) -> Vec<u8> {
    let mut field = [0_u8; NAME_FIELD_LENGTH];
    let length = name.len().min(NAME_FIELD_LENGTH - 1);
    field[..length].copy_from_slice(&name.as_bytes()[..length]);
    let mut packet = STARSKILL_PACKET.to_le_bytes().to_vec();
    packet.extend_from_slice(&field);
    packet.extend_from_slice(&monster_id.to_le_bytes());
    packet.push(slot);
    packet.push(result);
    packet
}

pub(crate) fn place_packet(map: &str, slot: usize) -> Vec<u8> {
    starskill_packet(&format!("{map}.gat"), 0, slot as u8, REGISTER_PLACE)
}

/// `ZC_STARPLACE`: asks the client whether the current map should be remembered.
pub(crate) fn place_request_packet(slot: usize) -> Vec<u8> {
    let mut packet = FEEL_REQUEST_PACKET.to_le_bytes().to_vec();
    packet.push(slot as u8);
    packet
}

pub(crate) fn level_slot(level: u8) -> Result<usize, String> {
    usize::from(level).checked_sub(1).filter(|slot| *slot < SLOTS).ok_or_else(|| format!("Invalid skill level {level}"))
}

impl ScriptSkillService {
    /// The comfort skills only work on the remembered map on the day of their slot, or during a Miracle.
    pub(super) fn star_comfort(&self, server: &Server, character: &mut Character, name: &str, effect: &ScriptSkillEffect, tick: u128) -> Result<(), String> {
        let (slot, kind) = match name {
            "SG_SUN_COMFORT" => (0, StatusChangeKind::SunComfort),
            "SG_MOON_COMFORT" => (1, StatusChangeKind::MoonComfort),
            _ => (STAR_SLOT, StatusChangeKind::StarComfort),
        };
        if effect.source_char_id != character.char_id {
            return Err("Comfort can only be cast on oneself".into());
        }
        let here = normalize_map(character.current_map_name());
        if !character.status.has_status_change(StatusChangeKind::Miracle) && (character.game_systems.star_places[slot] != here || !is_today(slot)) {
            return Err("The comfort needs the remembered map on its day".into());
        }
        let snapshot = StatusService::instance().to_snapshot(&character.status);
        let total = character.status.base_level as i32 + i32::from(snapshot.dex()) + i32::from(snapshot.luk());
        let mut request = Self::timed_request(effect, kind);
        request.values[1] = match slot {
            0 => total / 2,
            1 => total / 10,
            _ => total,
        };
        StatusEffectService::start(server, character, request, tick, &self.client_notification_sender).map(|_| ())
    }

    /// Miracle can start on any attack of a Star Gladiator, except on maps that block it.
    pub(crate) fn try_miracle(&self, server: &Server, state: &ServerState, character: &mut Character, tick: u128) -> Result<(), String> {
        if character.status.job != JobName::StarGladiator.value() as u32 || state.map_flags(&character.map_instance_key).enabled(MapFlag::NoSunMoonStarMiracle) {
            return Ok(());
        }
        let agi = StatusService::instance().to_snapshot(&character.status).agi() as u32;
        if !miracle_starts(agi, fastrand::u32(0..MIRACLE_ODDS), fastrand::u32(0..agi.max(1))) {
            return Ok(());
        }
        let request = StatusChangeRequest::guaranteed(StatusChangeKind::Miracle, MIRACLE_DURATION_MS, 0);
        StatusEffectService::start(server, character, request, tick, &self.client_notification_sender).map(|_| ())
    }

    /// Fusion drains the caster's HP on each landed basic attack. rathena can kill the caster here; this stops at 1 HP.
    pub(crate) fn fusion_hit_cost(status: &Status, max_hp: u32, against_player: bool) -> u32 {
        if !status.has_status_change(StatusChangeKind::Fusion) {
            return 0;
        }
        let hp = status.hp;
        let cost = match (against_player, u64::from(hp) * 100 <= u64::from(max_hp) * 20) {
            (false, _) => 2 * max_hp / 100,
            (true, true) => hp,
            (true, false) => max_hp / 13,
        };
        cost.min(hp.saturating_sub(1))
    }

    /// A slot keeps its monster for good: a second cast only shows it again.
    pub(super) fn star_hate_mob(&self, server: &Server, caster: &mut Character, effect: &ScriptSkillEffect, mob: &Mob) -> Result<bool, String> {
        let slot = level_slot(effect.level)?;
        let name = |class: u32| GlobalConfigService::instance().get_mob_safe(class as i32).map(|model| model.name_english.clone()).unwrap_or_default();
        let known = caster.game_systems.star_hates[slot];
        if known != 0 {
            server.send_to_character(caster.char_id, starskill_packet(&name(known), known as i32, slot as u8, SHOW_HATE));
            return Ok(false);
        }
        if !hate_slot_accepts(slot, *mob.status.size(), mob.status.max_hp()) {
            return Ok(false);
        }
        let class = mob.mob_id as u32;
        caster.game_systems.star_hates[slot] = class;
        server.script_world_service().persist(caster)?;
        caster.refresh_script_context();
        server.send_to_character(caster.char_id, starskill_packet(&name(class), class as i32, slot as u8, REGISTER_HATE));
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fusion_costs_hp_per_landed_basic_attack_and_stops_at_one_hp() {
        let mut status = Status::default();
        status.hp = 1000;
        assert_eq!(ScriptSkillService::fusion_hit_cost(&status, 1000, false), 0);
        StatusEffectService::apply_status(&mut status, StatusChangeRequest::guaranteed(StatusChangeKind::Fusion, 60_000, 0), 0, 0).unwrap();
        assert_eq!(ScriptSkillService::fusion_hit_cost(&status, 1000, false), 20);
        assert_eq!(ScriptSkillService::fusion_hit_cost(&status, 1000, true), 76);
        status.hp = 200;
        assert_eq!(ScriptSkillService::fusion_hit_cost(&status, 1000, true), 199);
        status.hp = 10;
        assert_eq!(ScriptSkillService::fusion_hit_cost(&status, 1000, false), 9);
    }

    #[test]
    fn the_sun_moon_and_stars_take_turns_over_the_year() {
        assert!(is_day(0, 1) && !is_day(0, 0));
        assert!(is_day(1, 0) && !is_day(1, 1));
        assert!(is_day(2, 4) && is_day(2, 9) && !is_day(2, 5));
    }

    #[test]
    fn anger_scales_with_levels_dexterity_and_luck() {
        assert_eq!(anger_percent(0, 1, 99, 0, 60, 30), 189 / 9);
        assert_eq!(anger_percent(0, 3, 99, 0, 60, 30), 189 / 3);
        assert_eq!(anger_percent(0, 4, 99, 0, 60, 30), 189);
        assert_eq!(anger_percent(2, 3, 99, 30, 60, 30), 219 / 3);
    }

    #[test]
    fn anger_only_boosts_damage_against_the_hated_monster() {
        let mut status = Status::default();
        status.base_level = 99;
        status.dex = 60;
        status.luk = 30;
        status.star_hates = [1002, 0, 0];
        status.script_skill_grants.insert(
            SkillEnum::SgSunAnger.id(),
            models::skill_grant::ScriptSkillGrant { learned_level: 3, current_level: 3 },
        );
        assert_eq!(anger_bonuses(&status), vec![BonusType::PhysicalDamageAgainstMobIdPercentage(1002, 63)]);
        status.star_hates = [0; 3];
        assert!(anger_bonuses(&status).is_empty());
    }

    #[test]
    fn hated_monsters_must_fit_the_size_and_health_of_their_slot() {
        assert!(hate_slot_accepts(0, Size::Small, 50));
        assert!(!hate_slot_accepts(0, Size::Medium, 50));
        assert!(!hate_slot_accepts(1, Size::Medium, 5_999));
        assert!(hate_slot_accepts(1, Size::Medium, 6_000));
        assert!(!hate_slot_accepts(2, Size::Large, 19_999));
        assert!(hate_slot_accepts(2, Size::Large, 20_000));
    }

    #[test]
    fn blessing_gives_experience_from_the_hated_monster_on_its_day_only() {
        let mut status = Status::default();
        status.star_hates = [0, 0, 1038];
        status.script_skill_grants.insert(SkillEnum::SgStarBless.id(), models::skill_grant::ScriptSkillGrant { learned_level: 3, current_level: 3 });
        assert_eq!(bless_percent_on(&status, 1038, |_| true), 60);
        assert_eq!(bless_percent_on(&status, 1038, |_| false), 0);
        assert_eq!(bless_percent_on(&status, 1002, |_| true), 0);
    }

    #[test]
    fn knowledge_raises_the_weight_limit_on_a_remembered_map_only() {
        let mut character = crate::tests::common::character_helper::create_character();
        character.status.script_skill_grants.insert(SkillEnum::SgKnowledge.id(), models::skill_grant::ScriptSkillGrant { learned_level: 4, current_level: 4 });
        assert_eq!(knowledge_weight_percent(&character), 0);
        character.game_systems.star_places[1] = normalize_map(character.current_map_name());
        assert_eq!(knowledge_weight_percent(&character), 40);
    }

    #[test]
    fn comfort_statuses_raise_defence_flee_and_attack_speed() {
        let bonuses = |kind, values| {
            let mut request = models::status_change::StatusChangeRequest::guaranteed(kind, -1, 0);
            request.values = values;
            let mut status = Status::default();
            StatusEffectService::apply_status(&mut status, request, 0, 0).unwrap();
            status.status_change(kind).unwrap().bonuses()
        };
        assert_eq!(bonuses(StatusChangeKind::SunComfort, [2, 90, 0, 0]), vec![BonusType::Def(90)]);
        assert_eq!(bonuses(StatusChangeKind::MoonComfort, [2, 18, 0, 0]), vec![BonusType::Flee(18)]);
        assert_eq!(bonuses(StatusChangeKind::StarComfort, [2, 180, 0, 0]), vec![BonusType::AspdPercentage(6.0)]);
    }

    #[test]
    fn devil_speeds_attacks_only_at_the_maximum_job_level() {
        assert_eq!(devil_percent(3, 49, 50), 0);
        assert_eq!(devil_percent(3, 50, 50), 90);
        assert_eq!(devil_percent(0, 50, 50), 0);
    }

    #[test]
    fn miracle_needs_the_odds_roll_and_low_agility_or_the_agility_roll() {
        assert!(miracle_starts(40, 0, 45));
        assert!(!miracle_starts(40, 1, 0));
        assert!(miracle_starts(100, 0, 45));
        assert!(!miracle_starts(100, 0, 46));
    }

    #[test]
    fn star_packets_have_the_client_layout() {
        let packet = starskill_packet("prontera.gat", 0, 1, REGISTER_PLACE);
        assert_eq!(packet.len(), 32);
        assert_eq!(&packet[..2], &[0x0E, 0x02]);
        assert_eq!(&packet[2..14], b"prontera.gat");
        assert_eq!(&packet[30..], &[1, 1]);
        assert_eq!(place_request_packet(2), vec![0x53, 0x02, 2]);
    }
}
