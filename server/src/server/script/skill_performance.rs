use models::enums::class::{JobFamily, JobName};
use models::enums::skill_enums::SkillEnum;
use models::enums::weapon::WeaponType;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_change::{StatusChange, StatusChangeKind, StatusChangeRequest};

use super::metadata::SkillMetadata;
use super::{ScriptSkillAction, ScriptSkillEffect, ScriptSkillService};
use crate::server::Server;
use crate::server::service::global_config_service::GlobalConfigService;
use super::trap::{GroundTrapEffect, GroundTrapEffectKind};
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterDamage, CharacterEndStatus, CharacterStatusChange, GameEvent};
use crate::server::model::events::map_event::{MapEvent, MobDamage, MobStatusChange};
use crate::server::service::script_character_service::learned_level;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

const RADIATE_INTERVAL_MS: u128 = 1000;
const PULSE_SECONDS: i32 = 6;
const HARASS_PULSE_SECONDS: i32 = 3;
const MARIONETTE_RANGE: u16 = 7;
/// Effects that must stop soon after the target leaves the area or the performance ends.
const SHORT_LINGER_MS: i32 = 3000;
const SONG_RADIUS: u16 = 3;
const ENSEMBLE_RADIUS: u16 = 4;
const ADAPTATION_MIN_ELAPSED_MS: i32 = 5000;
const DANCING_EXTRA_MS: i32 = 1000;

#[derive(Default)]
pub(super) struct PerformanceClock {
    next_radiate_at: u128,
    last_pulse_second: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Reach {
    /// Everybody on a normal map, only the party where players fight each other.
    Everyone,
    Anyone,
    Party,
    Enemies,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Effect {
    /// Hands `Performance::status` to everybody in reach.
    Aura,
    /// Damage to every enemy in the area on each pulse (Unchained Serenade).
    Damage,
    /// SP drain on enemy players on each pulse (Hip Shaker).
    Drain,
}

#[derive(Clone, Copy)]
struct Performance {
    status: StatusChangeKind,
    reach: Reach,
    ensemble: bool,
    effect: Effect,
}

impl Performance {
    fn radius(self) -> u16 {
        if self.ensemble { ENSEMBLE_RADIUS } else { SONG_RADIUS }
    }

    fn pulse_seconds(self) -> i32 {
        if self.effect == Effect::Aura { PULSE_SECONDS } else { HARASS_PULSE_SECONDS }
    }
}

fn performance(name: &str) -> Option<Performance> {
    use Reach::*;
    use StatusChangeKind::*;
    let song = |status, reach| Performance { status, reach, ensemble: false, effect: Effect::Aura };
    let ensemble = |status, reach| Performance { status, reach, ensemble: true, effect: Effect::Aura };
    let harassing = |effect| Performance { status: Dancing, reach: Enemies, ensemble: false, effect };
    Some(match name {
        "BA_WHISTLE" => song(Whistle, Everyone),
        "BA_ASSASSINCROSS" => song(AssnCros, Everyone),
        "BA_POEMBRAGI" => song(PoemBragi, Everyone),
        "BA_APPLEIDUN" => song(AppleIdun, Everyone),
        "DC_HUMMING" => song(Humming, Everyone),
        "DC_FORTUNEKISS" => song(Fortune, Everyone),
        "DC_SERVICEFORYOU" => song(Service4U, Everyone),
        "DC_DONTFORGETME" => song(DontForgetMe, Enemies),
        "BD_DRUMBATTLEFIELD" => ensemble(DrumBattle, Party),
        "BD_RINGNIBELUNGEN" => ensemble(Nibelungen, Party),
        "BD_SIEGFRIED" => ensemble(Siegfried, Party),
        "BD_ETERNALCHAOS" => ensemble(EternalChaos, Enemies),
        "BD_LULLABY" => ensemble(Sleep, Enemies),
        "BD_ROKISWEIL" => ensemble(RokisWeil, Anyone),
        "BD_INTOABYSS" => ensemble(IntoAbyss, Party),
        "BD_RICHMANKIM" => ensemble(RichMankim, Enemies),
        "BA_DISSONANCE" => harassing(Effect::Damage),
        "DC_UGLYDANCE" => harassing(Effect::Drain),
        _ => return None,
    })
}

fn lesson_for(name: &str) -> SkillEnum {
    if name.starts_with("BA_") || name.starts_with("BD_") { SkillEnum::BaMusicallesson } else { SkillEnum::DcDancinglesson }
}

/// The two numbers the song hands to its status, `values[1]` and `values[2]`.
fn song_values(name: &str, level: i32, stats: &StatusSnapshot, lesson: i32) -> (i32, i32) {
    let (agi, dex, int, vit, luk) = (i32::from(stats.agi()), i32::from(stats.dex()), i32::from(stats.int()), i32::from(stats.vit()), i32::from(stats.luk()));
    match name {
        "BA_WHISTLE" => (level + agi / 10 + lesson / 2, (level + 1) / 2 + luk / 30 + lesson / 5),
        "BA_ASSASSINCROSS" => ((lesson / 2 + 5 + level + agi / 20) * 10, 0),
        "BA_POEMBRAGI" => (3 * level + dex / 10 + lesson, (if level < 10 { 3 * level } else { 50 }) + int / 5 + 2 * lesson),
        "BA_APPLEIDUN" => (5 + 2 * level + vit / 10 + lesson / 2, 0),
        "DC_HUMMING" => (1 + 2 * level + dex / 10 + lesson, 0),
        "DC_FORTUNEKISS" => ((10 + level + luk / 10) * 10 + 5 * lesson, 0),
        "DC_SERVICEFORYOU" => (15 + level + int / 10 + lesson / 2, 20 + 3 * level + int / 10 + lesson / 2),
        "DC_DONTFORGETME" => ((5 + 3 * level + dex / 10 + lesson) * 10, 5 + 3 * level + agi / 10 + lesson),
        "BD_DRUMBATTLEFIELD" => ((level + 1) * 25, (level + 1) * 2),
        "BD_RINGNIBELUNGEN" => ((level + 2) * 25, 0),
        "BD_SIEGFRIED" => (55 + 5 * level, 10 * level),
        "BD_RICHMANKIM" => (25 + 11 * level, 0),
        _ => (0, 0),
    }
}

fn dissonance_damage(level: i32, lesson: i32) -> u32 {
    (30 + 10 * level + level * lesson).max(0) as u32
}

fn ugly_dance_drain(level: i32, lesson: i32) -> u32 {
    (5 + 5 * level + level * lesson).max(0) as u32
}

fn apple_of_idun_heal(level: i32, vit: i32, lesson: i32) -> u32 {
    (30 + 5 * level + vit / 2 + 5 * lesson).max(0) as u32
}

impl ScriptSkillService {
    pub(super) fn is_performance_skill(name: &str) -> bool {
        performance(name).is_some() || matches!(name, "BD_ENCORE" | "BD_ADAPTATION" | "CG_LONGINGFREEDOM")
    }

    fn find_partner(state: &ServerState, source: &Character, skill_id: u32) -> Option<(u32, u8)> {
        state.characters().values().find_map(|target| {
            let eligible = target.char_id != source.char_id
                && target.current_map_name() == source.current_map_name()
                && target.current_map_instance() == source.current_map_instance()
                && target.x.abs_diff(source.x).max(target.y.abs_diff(source.y)) <= 1
                && target.status.hp > 0
                && !target.status.blocks_casting()
                && !target.status.blocks_movement()
                && !target.status.has_status_change(StatusChangeKind::Dancing)
                && target.status.is_male != source.status.is_male
                && source.game_systems.party_id != 0
                && target.game_systems.party_id == source.game_systems.party_id
                && JobName::try_from_value(target.status.job as usize).is_ok_and(|job| matches!(job.job_family(), JobFamily::Bard | JobFamily::Dancer))
                && matches!(StatusService::instance().to_snapshot(&target.status).right_hand_weapon_type(), WeaponType::Musical | WeaponType::Whip);
            let level = learned_level(&target.status, skill_id);
            (eligible && level > 0).then_some((target.char_id, level))
        })
    }

    /// While performing only a few skills are allowed, an ensemble needs its partner next to the caster.
    pub fn validate_performing(&self, state: &ServerState, character: &Character, skill_id: u32) -> Result<(), String> {
        if character.status.has_status_change(StatusChangeKind::RokisWeil) && skill_id != SkillEnum::BdAdaptation.id() {
            return Err("Loki's Veil forbids skills".into());
        }
        let controlling = skill_id == SkillEnum::CgMarionette.id();
        if (character.status.has_status_change(StatusChangeKind::Marionette) && !controlling)
            || (character.status.has_status_change(StatusChangeKind::Marionette2) && controlling)
        {
            return Err("Marionette Control forbids this skill".into());
        }
        let metadata = SkillMetadata::find(skill_id);
        let flag = |name: &str| metadata.is_some_and(|metadata| metadata.flags.get(name).copied().unwrap_or(false));
        if character.status.has_status_change(StatusChangeKind::Dancing) && !flag("AllowWhenPerforming") {
            let repeats_performance = flag("IsSong") || flag("IsEnsemble") || skill_id == SkillEnum::BdEncore.id();
            if !character.status.has_status_change(StatusChangeKind::Longing) || repeats_performance {
                return Err("Cannot use this skill while performing".into());
            }
        }
        if metadata.and_then(|metadata| performance(&metadata.name)).is_some_and(|performance| performance.ensemble)
            && Self::find_partner(state, character, skill_id).is_none()
        {
            return Err("An ensemble needs a partner standing next to the caster".into());
        }
        Ok(())
    }

    pub(super) fn apply_performance_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        effect: &ScriptSkillEffect,
        name: &str,
        tick: u128,
    ) -> Result<(), String> {
        match name {
            "BD_ADAPTATION" => {
                let elapsed_ms = character.status.status_change(StatusChangeKind::Dancing).map(|dance| dance.values[2].saturating_mul(1000)).ok_or("Not performing")?;
                if elapsed_ms < ADAPTATION_MIN_ELAPSED_MS {
                    return Err("The performance has not lasted long enough".into());
                }
                StatusEffectService::end(server, character, Some(StatusChangeKind::Dancing), tick, &self.client_notification_sender);
                Ok(())
            }
            "CG_LONGINGFREEDOM" => {
                let ensemble_dance = character.status.status_change(StatusChangeKind::Dancing).filter(|dance| dance.values[3] != 0 && dance.values[0] & 0xFFFF != SkillEnum::CgMoonlit.id() as i32);
                if ensemble_dance.is_none() || character.status.has_status_change(StatusChangeKind::Longing) {
                    return Err("Longing for Freedom needs an ensemble in progress".into());
                }
                let duration_ms = SkillMetadata::find(effect.skill_id).and_then(|metadata| metadata.duration(effect.level, false)).unwrap_or(0);
                let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Longing, duration_ms, i32::from(effect.level));
                request.flags = 0;
                StatusEffectService::start(server, character, request, tick, &self.client_notification_sender).map(|_| ())
            }
            "BD_ENCORE" => {
                let (skill_id, level) = character.script_skill_state.last_performance.ok_or("There is nothing to repeat")?;
                let level = level.min(learned_level(&character.status, skill_id));
                if level == 0 {
                    return Err("The performance is no longer known".into());
                }
                self.start_performance(server, state, character, skill_id, level, true, tick)
            }
            _ => self.start_performance(server, state, character, effect.skill_id, effect.level, false, tick),
        }
    }

    fn start_performance(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        skill_id: u32,
        level: u8,
        half_cost: bool,
        tick: u128,
    ) -> Result<(), String> {
        let metadata = SkillMetadata::find(skill_id).ok_or("Unknown performance")?;
        let performance = performance(&metadata.name).ok_or("This skill is not a performance")?;
        let (partner, level) = if performance.ensemble {
            let (partner, partner_level) = Self::find_partner(state, character, skill_id).ok_or("An ensemble needs a partner standing next to the caster")?;
            (Some(partner), ((u32::from(level) + u32::from(partner_level)) / 2) as u8)
        } else {
            (None, level)
        };
        if half_cost {
            let cost = metadata
                .requires
                .as_ref()
                .and_then(|requires| requires.get("SpCost"))
                .and_then(|cost| SkillMetadata::json_level_value(cost, level, "Amount"))
                .unwrap_or(0)
                .max(0) as u32
                / 2;
            if character.status.sp < cost {
                return Err("Not enough SP to repeat the performance".into());
            }
            server.character_service().update_hp_sp(character, character.status.hp, character.status.sp - cost);
        }
        let duration_ms = metadata.duration(level, false).unwrap_or(0).saturating_add(DANCING_EXTRA_MS);
        let dancing = |initiator: bool, partner_id: u32| {
            let mut request = StatusChangeRequest::guaranteed(StatusChangeKind::Dancing, duration_ms, 0);
            request.values = [skill_id as i32 | i32::from(level) << 16, i32::from(initiator), 0, partner_id as i32];
            request.flags = 0;
            request
        };
        StatusEffectService::start(server, character, dancing(true, partner.unwrap_or(0)), tick, &self.client_notification_sender)?;
        character.script_skill_state.last_performance = Some((skill_id, level));
        if let Some(partner) = partner {
            server.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange { char_id: partner, request: dancing(false, character.char_id) }));
        }
        Ok(())
    }

    /// Marionette Control links two party members: the puppeteer gives half of each base stat to the puppet, for as long as both stay within range.
    pub(super) fn start_marionette(&self, server: &Server, state: &ServerState, target: &mut Character, effect: &ScriptSkillEffect, tick: u128) -> Result<(), String> {
        let source = state.get_character(effect.source_char_id).ok_or("Caster disconnected")?;
        if let Some(active) = source.status.status_change(StatusChangeKind::Marionette) {
            let puppet = active.values[1] as u32;
            server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: source.char_id, kind: Some(StatusChangeKind::Marionette) }));
            if puppet == target.char_id {
                StatusEffectService::end(server, target, Some(StatusChangeKind::Marionette2), tick, &self.client_notification_sender);
            } else {
                server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: puppet, kind: Some(StatusChangeKind::Marionette2) }));
            }
            return Ok(());
        }
        let performer = |character: &Character| JobName::try_from_value(character.status.job as usize).is_ok_and(|job| matches!(job.job_family(), JobFamily::Bard | JobFamily::Dancer));
        if source.char_id == target.char_id
            || source.game_systems.party_id == 0
            || source.game_systems.party_id != target.game_systems.party_id
            || (performer(target) && source.status.is_male == target.status.is_male)
            || target.status.has_status_change(StatusChangeKind::Curse)
            || target.status.has_status_change(StatusChangeKind::Quagmire)
            || target.status.has_status_change(StatusChangeKind::Marionette2)
        {
            return Err("Marionette Control cannot target this player".into());
        }
        let maximum = u16::try_from(GlobalConfigService::battle_option("max_parameter").max(1)).unwrap_or(u16::MAX);
        let halves = [source.status.str, source.status.agi, source.status.vit, source.status.int, source.status.dex, source.status.luk].map(|stat| stat / 2);
        let room = [target.status.str, target.status.agi, target.status.vit, target.status.int, target.status.dex, target.status.luk];
        let received = std::array::from_fn::<u16, 6, _>(|index| halves[index].min(maximum.saturating_sub(room[index])));
        let pack = |stats: [u16; 3]| i32::from(stats[0] & 0xFF) | i32::from(stats[1] & 0xFF) << 8 | i32::from(stats[2] & 0xFF) << 16;
        let request = |kind, partner: u32, stats: [u16; 6]| {
            let mut request = StatusChangeRequest::guaranteed(kind, -1, i32::from(effect.level));
            request.values = [i32::from(effect.level), partner as i32, pack([stats[0], stats[1], stats[2]]), pack([stats[3], stats[4], stats[5]])];
            request.flags = 0;
            request
        };
        StatusEffectService::start(server, target, request(StatusChangeKind::Marionette2, source.char_id, received), tick, &self.client_notification_sender)?;
        server.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange {
            char_id: source.char_id,
            request: request(StatusChangeKind::Marionette, target.char_id, halves),
        }));
        Ok(())
    }

    fn tick_marionettes(&self, server: &Server, state: &ServerState) {
        for puppeteer in state.characters().values() {
            let Some(link) = puppeteer.status.status_change(StatusChangeKind::Marionette) else { continue };
            let puppet = state.get_character(link.values[1] as u32);
            let linked = puppet.is_some_and(|puppet| {
                puppet.status.status_change(StatusChangeKind::Marionette2).is_some_and(|back| back.values[1] == puppeteer.char_id as i32)
                    && puppet.map_instance_key == puppeteer.map_instance_key
                    && puppet.x.abs_diff(puppeteer.x).max(puppet.y.abs_diff(puppeteer.y)) <= MARIONETTE_RANGE
            });
            if !linked {
                server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: puppeteer.char_id, kind: Some(StatusChangeKind::Marionette) }));
                if puppet.is_some() {
                    server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: link.values[1] as u32, kind: Some(StatusChangeKind::Marionette2) }));
                }
            }
        }
        for puppet in state.characters().values() {
            let Some(link) = puppet.status.status_change(StatusChangeKind::Marionette2) else { continue };
            let held = state.get_character(link.values[1] as u32).is_some_and(|puppeteer| puppeteer.status.has_status_change(StatusChangeKind::Marionette));
            if !held {
                server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: puppet.char_id, kind: Some(StatusChangeKind::Marionette2) }));
            }
        }
    }

    /// Runs once per loop iteration: keeps ensemble partners paired and lets every performer radiate its effect once a second.
    pub fn tick_performances(&self, server: &Server, state: &ServerState, tick: u128) {
        self.tick_marionettes(server, state);
        let Ok(mut clocks) = self.performance_clocks.lock() else {
            return;
        };
        clocks.retain(|id, _| state.get_character(*id).is_some_and(|character| character.status.has_status_change(StatusChangeKind::Dancing)));
        for dancer in state.characters().values() {
            let Some(dance) = dancer.status.status_change(StatusChangeKind::Dancing) else {
                continue;
            };
            let partner = dance.values[3] as u32;
            let paired = partner == 0
                || state
                    .get_character(partner)
                    .and_then(|partner| partner.status.status_change(StatusChangeKind::Dancing))
                    .is_some_and(|partner_dance| partner_dance.values[3] as u32 == dancer.char_id);
            if !paired {
                server.add_to_next_tick(GameEvent::CharacterEndStatus(CharacterEndStatus { char_id: dancer.char_id, kind: Some(StatusChangeKind::Dancing) }));
                continue;
            }
            if dance.values[1] == 0 {
                continue;
            }
            let clock = clocks.entry(dancer.char_id).or_default();
            if tick < clock.next_radiate_at {
                continue;
            }
            clock.next_radiate_at = tick + RADIATE_INTERVAL_MS;
            let second = dance.values[2];
            let pulse_every = SkillMetadata::find((dance.values[0] & 0xFFFF) as u32)
                .and_then(|metadata| performance(&metadata.name))
                .map_or(PULSE_SECONDS, Performance::pulse_seconds);
            let pulse = second != clock.last_pulse_second && second > 0 && second % pulse_every == 0;
            if pulse {
                clock.last_pulse_second = second;
            }
            self.radiate(server, state, dancer, dance, pulse, tick);
        }
    }

    fn harass(
        &self,
        server: &Server,
        state: &ServerState,
        source: &Character,
        stats: &StatusSnapshot,
        (skill_id, level, lesson): (u32, i32, i32),
        effect: Effect,
        radius: u16,
        tick: u128,
    ) {
        use models::enums::element::Element;
        use models::status_bonus::BattleFlag;
        let in_range = |x: u16, y: u16| source.x.abs_diff(x).max(source.y.abs_diff(y)) <= radius;
        let flags = BattleFlag::Misc.as_flag() | BattleFlag::Long.as_flag() | BattleFlag::Skill.as_flag();
        let strike = |target_id: u32, target: &StatusSnapshot| {
            Damage {
                notification: None,
                source_kind: *stats.combat_actor_kind(),
                skill_damage_adjusted: false,
                target_id,
                attacker_id: source.char_id,
                damage: server.battle_service().actor_misc_skill_damage(dissonance_damage(level, lesson), stats, target, &Element::Neutral, flags, skill_id),
                healing: 0,
                right_hand_damage: None,
                attacked_at: tick,
                damage_motion: 0,
                battle_flags: flags,
                skill_id,
                skill_level: level as u8,
                landed: true,
                proc_depth: 0,
                credit_id: source.char_id,
                defenses_applied: true,
                magic_context: None,
            }
            .with_skill_notification(source.current_map_name(), source.current_map_instance(), source.x, source.y, tick, 1, 0)
        };
        if effect == Effect::Damage {
            if let Some(instance) = state.get_map_instance_from_character(source) {
                let mobs = instance
                    .state()
                    .mobs()
                    .values()
                    .filter(|mob| mob.status.hp() > 0 && in_range(mob.x, mob.y))
                    .map(|mob| (mob.id, mob.status.clone()))
                    .collect::<Vec<_>>();
                for (mob_id, status) in mobs {
                    instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: strike(mob_id, &status) }));
                }
            }
        }
        for target in state.characters().values() {
            if target.char_id == source.char_id
                || target.status.hp == 0
                || target.map_instance_key != source.map_instance_key
                || !in_range(target.x, target.y)
                || (source.game_systems.party_id != 0 && target.game_systems.party_id == source.game_systems.party_id)
                || !server.player_ground_target_allowed(state, source, target.char_id, true)
            {
                continue;
            }
            let snapshot = StatusService::instance().to_snapshot(&target.status);
            match effect {
                Effect::Damage => server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage: strike(target.char_id, &snapshot) })),
                Effect::Drain => {
                    let percent = (u64::from(ugly_dance_drain(level, lesson)) * 100 / u64::from(snapshot.max_sp().max(1))).clamp(1, 100) as u16;
                    server.add_to_next_tick(GameEvent::GroundTrapEffect(GroundTrapEffect {
                        map: target.map_instance_key.clone(),
                        target_id: target.char_id,
                        kind: GroundTrapEffectKind::DrainSp { percent },
                    }));
                }
                Effect::Aura => {}
            }
        }
    }

    fn radiate(&self, server: &Server, state: &ServerState, source: &Character, dance: &StatusChange, pulse: bool, tick: u128) {
        let skill_id = (dance.values[0] & 0xFFFF) as u32;
        let level = dance.values[0] >> 16;
        let Some(metadata) = SkillMetadata::find(skill_id) else { return };
        let Some(performance) = performance(&metadata.name) else { return };
        let stats = StatusService::instance().to_snapshot(&source.status);
        let lesson = i32::from(learned_level(&source.status, lesson_for(&metadata.name).id()));
        let (first, second) = song_values(&metadata.name, level, &stats, lesson);
        let radius = performance.radius();
        let in_range = |x: u16, y: u16| source.x.abs_diff(x).max(source.y.abs_diff(y)) <= radius;
        let lingering = !performance.ensemble && performance.reach != Reach::Enemies;
        let linger_ms = if lingering { metadata.duration(level as u8, true).unwrap_or(20_000) } else { SHORT_LINGER_MS };
        let versus = state.map_flags(&source.map_instance_key).versus(state.siege_active());
        if performance.effect != Effect::Aura {
            if pulse {
                self.harass(server, state, source, &stats, (skill_id, level, lesson), performance.effect, radius, tick);
            }
            return;
        }
        let lullaby = performance.status == StatusChangeKind::Sleep;
        let mut request = StatusChangeRequest::guaranteed(performance.status, linger_ms, level);
        request.values = [level, first, second, 0];
        request.flags = 0;
        if lullaby {
            request.duration_ms = metadata.duration(level as u8, true).unwrap_or(30_000);
        }

        if performance.reach != Reach::Enemies {
            for target in state.characters().values() {
                let party = source.game_systems.party_id != 0 && target.game_systems.party_id == source.game_systems.party_id;
                let allowed = match performance.reach {
                    Reach::Party => party,
                    Reach::Anyone => true,
                    _ => party || !versus,
                };
                if !allowed
                    || target.char_id == source.char_id
                    || target.status.hp == 0
                    || target.map_instance_key != source.map_instance_key
                    || !in_range(target.x, target.y)
                {
                    continue;
                }
                let stale = target
                    .status
                    .status_change(performance.status)
                    .is_none_or(|current| current.remaining_ms(tick) < linger_ms.max(0) as u32 / 2 || current.values != request.values);
                if stale {
                    server.add_to_next_tick(GameEvent::CharacterStatusChange(CharacterStatusChange { char_id: target.char_id, request: request.clone() }));
                }
                if pulse && metadata.name == "BA_APPLEIDUN" {
                    let heal = apple_of_idun_heal(level, i32::from(stats.vit()), lesson);
                    let heal_effect = ScriptSkillEffect {
                        source_char_id: source.char_id,
                        target_id: target.char_id,
                        skill_id,
                        level: level as u8,
                        heal_value: 0,
                        proc_depth: 0,
                        skill_event_emitted: true,
                        cast_generation: 0,
                        action: ScriptSkillAction::Heal { hp: heal, sp: 0 },
                        deferred_requirements: None,
                        prepared_outcome: None,
                        source_index: None,
                        source_item: None,
                    };
                    server.add_to_next_tick(GameEvent::CharacterScriptSkill(heal_effect));
                }
            }
            return;
        }
        if lullaby && !pulse {
            return;
        }
        let Some(instance) = state.get_map_instance_from_character(source) else { return };
        let partner_int = state.get_character(dance.values[3] as u32).map_or(0, |partner| i32::from(StatusService::instance().to_snapshot(&partner.status).int()));
        let map_state = instance.state();
        for mob in map_state.mobs().values().filter(|mob| mob.status.hp() > 0 && in_range(mob.x, mob.y)) {
            let mut request = request.clone();
            if lullaby {
                request.rate = ((i32::from(stats.int()) + partner_int + fastrand::i32(100..=300)) * 10).clamp(0, 10_000) as u16;
            } else if mob.status_effects.status_change(performance.status).is_some_and(|current| current.remaining_ms(tick) > linger_ms.max(0) as u32 / 2) {
                continue;
            }
            instance.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange { mob_id: mob.id, request }));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harassing_songs_pulse_every_three_seconds_and_scale_with_the_lesson() {
        assert_eq!(performance("BA_DISSONANCE").unwrap().pulse_seconds(), 3);
        assert_eq!(performance("DC_UGLYDANCE").unwrap().pulse_seconds(), 3);
        assert_eq!(performance("BA_WHISTLE").unwrap().pulse_seconds(), PULSE_SECONDS);
        assert_eq!(dissonance_damage(5, 10), 30 + 50 + 50);
        assert_eq!(ugly_dance_drain(5, 10), 5 + 25 + 50);
    }
}
