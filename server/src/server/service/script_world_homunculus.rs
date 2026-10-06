use std::collections::BTreeMap;
use crate::server::state::server::ServerState;

use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::enums::size::Size;
use models::enums::{EnumWithMaskValueU16, EnumWithStringValue};
use models::status::{Status, StatusSnapshot};
use models::status_bonus::StatusBonus;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::{ScriptWorldService, homunculus_world_id, install_state, mercenary_world_id, protocol, world_data};
use crate::server::Server;
use crate::server::model::events::game_event::CharacterKillMonster;
use crate::server::model::game_systems::HomunculusRecord;
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::state::character::Character;

#[derive(Debug, Clone, PartialEq)]
pub enum HomunculusRequest {
    CallHomunculus,
    RestHomunculus,
    ResurrectHomunculus {
            skill_level: u8,
        },
    HomunculusMenu(u8),
    HomunculusRename(String),
    CompanionMove {
            id: u32,
            x: u16,
            y: u16,
        },
    CompanionMoveToOwner(u32),
    CompanionAttack {
            id: u32,
            target: u32,
            repeat: bool,
        },
}


impl ScriptWorldService {
    pub fn learn_homunculus_skill(&self, character: &mut Character, skill_id: u32) -> Result<(), String> {
        let homunculus = character.game_systems.homunculus.as_mut().ok_or("No homunculus exists")?;
        if !homunculus.active || homunculus.hp == 0 || homunculus.skill_points == 0 {
            return Err("Homunculus cannot learn a skill in the current state".into());
        }
        let definition = available_homunculus_skills(homunculus)
            .into_iter()
            .find(|(_, metadata, _)| metadata.id == skill_id)
            .ok_or("Homunculus skill prerequisites are not met")?
            .0;
        let current = homunculus.skills.entry(skill_id).or_default();
        if *current >= definition.max_level {
            return Err("Homunculus skill is already at its maximum level".into());
        }
        *current += 1;
        homunculus.skill_points -= 1;
        recalculate_homunculus(homunculus);
        self.persist(character)?;
        self.send_homunculus(character)
    }

    pub(crate) fn send_homunculus_skills(&self, character: &Character) -> Result<(), String> {
        let Some(homunculus) = &character.game_systems.homunculus else {
            return Ok(());
        };
        let mut packet = protocol::header(0x0235);
        packet.extend_from_slice(&0u16.to_le_bytes());
        for (definition, metadata, level) in available_homunculus_skills(homunculus) {
            let inf = match metadata.target_type.as_deref() {
                Some("Attack") => 1u16,
                Some("Ground") => 2,
                Some("Self") => 4,
                Some("Support") => 16,
                _ => 0,
            };
            let sp = metadata
                .requires
                .as_ref()
                .and_then(|requires| requires.get("SpCost"))
                .and_then(|value| SkillMetadata::json_level_value(value, level.max(1), "Amount"))
                .unwrap_or(0)
                .clamp(0, i32::from(u16::MAX)) as u16;
            let range = metadata.range(level.max(1)).unwrap_or(1).unsigned_abs().min(u32::from(u16::MAX)) as u16;
            packet.extend_from_slice(&(metadata.id as u16).to_le_bytes());
            packet.extend_from_slice(&inf.to_le_bytes());
            packet.extend_from_slice(&0u16.to_le_bytes());
            packet.extend_from_slice(&u16::from(level).to_le_bytes());
            packet.extend_from_slice(&sp.to_le_bytes());
            packet.extend_from_slice(&range.to_le_bytes());
            protocol::fixed_string(&mut packet, &metadata.name, 24);
            packet.push(u8::from(level < definition.max_level));
        }
        protocol::set_length(&mut packet);
        self.send(character.char_id, packet)
    }

    pub fn record_companion_kill(
        &self,
        _server: &Server,
        character: &mut Character,
        kill: &CharacterKillMonster,
        flags: &crate::server::model::map_flags::MapFlags,
        now: u64,
    ) -> Result<(), String> {
        let mut changed = false;
        if let Some(homunculus) = character.game_systems.homunculus.as_mut() {
            let (base_exp, _) =
                crate::server::service::script_experience_service::actor_experience_share_with_map(kill, homunculus_world_id(homunculus), flags);
            if homunculus.active && homunculus.hp > 0 && base_exp > 0 {
                let rate = f64::from(self.configuration.config().game.base_exp_rate);
                let gain = (f64::from(base_exp) * rate).ceil().clamp(0.0, u64::MAX as f64) as u64;
                gain_homunculus_experience(homunculus, gain, &mut fastrand::Rng::new())?;
                changed = true;
            }
        }
        if let Some(mercenary) = character.game_systems.mercenary.as_mut() {
            let owner_or_mercenary =
                kill.attacker_id == character.char_id || kill.attacker_id == 0 || kill.attacker_id == mercenary_world_id(mercenary);
            let level = self
                .configuration
                .get_mob_safe(i32::from(kill.mob_id))
                .map(|mob| mob.level.max(0) as u32)
                .unwrap_or(0);
            if kill.char_id == character.char_id && owner_or_mercenary && mercenary.hp > 0 && level >= character.status.base_level / 2 {
                mercenary.kill_count = mercenary.kill_count.saturating_add(1).min(i32::MAX as u32);
                if mercenary.kill_count % 50 == 0 {
                    if mercenary.guild < 3 {
                        let faith = character.game_systems.mercenary_faith.entry(mercenary.guild).or_default();
                        *faith = faith.saturating_add(1).min(i16::MAX as u16);
                    }
                    let kinds = [
                        StatusChangeKind::MercFleeUp,
                        StatusChangeKind::MercAttackUp,
                        StatusChangeKind::MercHpUp,
                        StatusChangeKind::MercSpUp,
                        StatusChangeKind::MercHitUp,
                    ];
                    let kind = kinds[fastrand::usize(..kinds.len())];
                    let mut status = super::mercenary_status(mercenary);
                    StatusEffectService::apply_status(
                        &mut status,
                        StatusChangeRequest::guaranteed(kind, 300_000, fastrand::i32(1..=5)),
                        now as u128,
                        0,
                    )?;
                    mercenary.statuses = status.active_statuses;
                    super::recalculate_mercenary(mercenary);
                    if kind == StatusChangeKind::MercHpUp {
                        mercenary.hp = mercenary.max_hp;
                    }
                    if kind == StatusChangeKind::MercSpUp {
                        mercenary.sp = mercenary.max_sp;
                    }
                }
                changed = true;
            }
        }
        if changed {
            self.persist(character)?;
            self.send_homunculus(character)?;
            self.send_mercenary(character, now)?;
        }
        Ok(())
    }

    pub(crate) fn homunculus_request(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        request: HomunculusRequest,
        now: u64,
    ) -> Result<(), String> {
        match request {
            HomunculusRequest::CallHomunculus => {
                if let Some(homunculus) = character.game_systems.homunculus.as_mut() {
                    if homunculus.active || homunculus.hp == 0 {
                        return Err("Homunculus cannot be called in the current state".into());
                    }
                    homunculus.active = true;
                    homunculus.next_hunger_at = now + 60_000;
                    self.persist(character)?;
                } else {
                    let data = &world_data().homunculi[fastrand::usize(..world_data().homunculi.len())];
                    let mut staged = character.game_systems.clone();
                    staged.homunculus = Some(HomunculusRecord {
                        id: 0,
                        class_id: data.class_id,
                        name: data.name.clone(),
                        level: 1,
                        intimacy: 2100,
                        hunger: 32,
                        hp: 10,
                        sp: 0,
                        max_hp: data.base[0],
                        max_sp: data.base[1],
                        base_max_hp: data.base[0],
                        base_max_sp: data.base[1],
                        statuses: Vec::new(),
                        stats: data.base[2..]
                            .iter()
                            .map(|stat| *stat as u16)
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                        active: true,
                        evolved: false,
                        skill_points: 0,
                        skills: BTreeMap::new(),
                        experience: 0,
                        renamed: false,
                        next_hunger_at: now + data.hungry_delay,
                        skill_cooldowns: BTreeMap::new(),
                        cooldown_pause_at: None,
                    });
                    let saved = self
                        .repository
                        .save_game_systems_consuming_item(character.char_id, &staged, 7142)
                        .map_err(|error| error.to_string())?;
                    install_state(character, saved);
                    server
                        .inventory_service()
                        .reload_inventory(server.runtime(), character.char_id, character);
                }
            }
            HomunculusRequest::RestHomunculus => {
                let homunculus = character.game_systems.homunculus.as_mut().ok_or("No homunculus exists")?;
                if !homunculus.active || homunculus.hp == 0 || u64::from(homunculus.hp) * 100 < u64::from(homunculus.max_hp) * 80 {
                    return Err("An active homunculus needs at least 80% HP to rest".into());
                }
                homunculus.active = false;
                homunculus.statuses.clear();
                character.game_systems.companion_commands.remove(&homunculus_world_id(homunculus));
                recalculate_homunculus(homunculus);
                self.persist(character)?;
            }
            HomunculusRequest::ResurrectHomunculus { skill_level } => {
                if !(1..=5).contains(&skill_level) {
                    return Err("Invalid homunculus resurrection level".into());
                }
                let homunculus = character.game_systems.homunculus.as_mut().ok_or("No homunculus exists")?;
                if homunculus.hp > 0 {
                    return Err("The homunculus is still alive".into());
                }
                homunculus.active = true;
                homunculus.hp = ((u64::from(homunculus.max_hp) * u64::from(skill_level) * 20 / 100) as u32).max(1);
                homunculus.next_hunger_at = now + 60_000;
                self.persist(character)?;
            }
            HomunculusRequest::HomunculusMenu(menu) => {
                if !character
                    .game_systems
                    .homunculus
                    .as_ref()
                    .is_some_and(|homunculus| homunculus.active && homunculus.hp > 0)
                {
                    return Err("No homunculus is active".into());
                }
                match menu {
                    0 => return self.send_homunculus(character),
                    1 => self.feed_homunculus(server, character)?,
                    2 => {
                        character.game_systems.homunculus = None;
                        self.persist(character)?;
                    }
                    _ => return Err("Unknown homunculus command".into()),
                }
            }
            HomunculusRequest::HomunculusRename(name) => {
                if name.trim().is_empty() || name.len() > 23 || name.chars().any(char::is_control) {
                    return Err("Invalid homunculus name".into());
                }
                let homunculus = character.game_systems.homunculus.as_mut().ok_or("No homunculus exists")?;
                if homunculus.renamed {
                    return Err("The homunculus has already been renamed".into());
                }
                homunculus.name = name;
                homunculus.renamed = true;
                self.persist(character)?;
            }
            HomunculusRequest::CompanionMove { id, x, y } => {
                validate_companion(character, id)?;
                let map = state
                    .get_map_instance_from_character(character)
                    .ok_or("Companion map is unavailable")?;
                let map_state = map.state();
                if x >= map_state.x_size()
                    || y >= map_state.y_size()
                    || map_state.cells()[usize::from(x) + usize::from(y) * usize::from(map_state.x_size())]
                        & models::enums::cell::CellType::Walkable.as_flag()
                        == 0
                    || character.x.abs_diff(x).max(character.y.abs_diff(y)) > 15
                {
                    return Err("Companion movement destination is out of reach".into());
                }
                let command = character.game_systems.companion_commands.entry(id).or_default();
                command.target = None;
                command.destination = Some((x, y));
                command.stay = true;
                return Ok(());
            }
            HomunculusRequest::CompanionMoveToOwner(id) => {
                validate_companion(character, id)?;
                character.game_systems.companion_commands.remove(&id);
                return Ok(());
            }
            HomunculusRequest::CompanionAttack { id, target, repeat } => {
                validate_companion(character, id)?;
                let map = state
                    .get_map_instance_from_character(character)
                    .ok_or("Companion map is unavailable")?;
                let map_state = map.state();
                let mob = map_state.get_mob(target).ok_or("Companion attack target is unavailable")?;
                if mob.hp() == 0
                    || mob.summon_ai != 0
                    || character.x.abs_diff(mob.x).max(character.y.abs_diff(mob.y)) > 15
                    || state.contains_locked_map_item(target)
                    || !super::companion_can_target(
                        &super::companion_status_snapshot(character, id).ok_or("Companion is unavailable")?,
                        &mob.status,
                        crate::server::service::visibility_service::TargetingMode::Direct,
                    )
                {
                    return Err("Companion attack target is out of reach".into());
                }
                let command = character.game_systems.companion_commands.entry(id).or_default();
                command.target = Some(target);
                command.repeat = repeat;
                command.destination = None;
                command.stay = false;
                return Ok(());
            }
        }
        if character.game_systems.homunculus.is_some() {
            self.send_homunculus(character)?;
        }
        self.render_companions(server, state, character, now)
    }

    fn feed_homunculus(&self, server: &Server, character: &mut Character) -> Result<(), String> {
        let mut staged = character.game_systems.clone();
        let homunculus = staged.homunculus.as_mut().ok_or("No homunculus exists")?;
        let data = world_data()
            .homunculi
            .iter()
            .find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id)
            .ok_or("Unknown homunculus class")?;
        feed_homunculus_record(homunculus);
        if homunculus.intimacy == 0 {
            staged.homunculus = None;
        }
        let saved = self
            .repository
            .save_game_systems_consuming_item(character.char_id, &staged, data.food_item)
            .map_err(|error| error.to_string())?;
        install_state(character, saved);
        server
            .inventory_service()
            .reload_inventory(server.runtime(), character.char_id, character);
        let mut packet = protocol::header(0x022F);
        packet.push(1);
        if self.configuration.packetver() >= 20181121 {
            packet.extend_from_slice(&(data.food_item as u32).to_le_bytes());
        } else {
            packet.extend_from_slice(&(data.food_item as u16).to_le_bytes());
        }
        self.send(character.char_id, packet)
    }
}

pub(crate) fn available_homunculus_skills(
    homunculus: &HomunculusRecord,
) -> Vec<(&'static super::HomunculusSkillDefinition, &'static SkillMetadata, u8)> {
    let Some(data) = world_data()
        .homunculi
        .iter()
        .find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id)
    else {
        return Vec::new();
    };
    data.skills
        .iter()
        .filter_map(|definition| {
            let metadata = SkillMetadata::all().iter().find(|metadata| metadata.name == definition.name)?;
            let level = homunculus.skills.get(&metadata.id).copied().unwrap_or(0);
            if level == 0
                && (definition.required_level > homunculus.level
                    || definition.required_intimacy > homunculus.intimacy
                    || (definition.evolution && !homunculus.evolved)
                    || definition.required.iter().any(|required| {
                        let id = SkillMetadata::all()
                            .iter()
                            .find(|metadata| metadata.name == required.name)
                            .map(|metadata| metadata.id);
                        id.and_then(|id| homunculus.skills.get(&id)).copied().unwrap_or(0) < required.level
                    }))
            {
                return None;
            }
            Some((definition, metadata, level))
        })
        .collect()
}

pub(crate) fn gain_homunculus_experience(homunculus: &mut HomunculusRecord, gain: u64, rng: &mut fastrand::Rng) -> Result<u16, String> {
    if homunculus.level == 0 {
        return Err("Invalid homunculus level".into());
    }
    let data = world_data()
        .homunculi
        .iter()
        .find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id)
        .ok_or("Unknown homunculus class")?;
    if homunculus.level >= 99 {
        homunculus.experience = 0;
        return Ok(0);
    }
    homunculus.experience = homunculus.experience.checked_add(gain).ok_or("Homunculus experience overflow")?;
    let mut gained = 0;
    while homunculus.level < 99 {
        let required = world_data()
            .homunculus_experience
            .get(usize::from(homunculus.level - 1))
            .copied()
            .unwrap_or(0);
        if required == 0 || homunculus.experience < required {
            break;
        }
        homunculus.experience -= required;
        homunculus.level += 1;
        gained += 1;
        if homunculus.level % 3 == 0 {
            homunculus.skill_points = homunculus.skill_points.saturating_add(1);
        }
        if homunculus.base_max_hp == 0 {
            homunculus.base_max_hp = homunculus.max_hp;
        }
        if homunculus.base_max_sp == 0 {
            homunculus.base_max_sp = homunculus.max_sp;
        }
        homunculus.base_max_hp = homunculus
            .base_max_hp
            .saturating_add(rng.u32(data.growth_min[0]..=data.growth_max[0]));
        homunculus.base_max_sp = homunculus
            .base_max_sp
            .saturating_add(rng.u32(data.growth_min[1]..=data.growth_max[1]));
        for (index, stat) in homunculus.stats.iter_mut().enumerate() {
            let growth = rng.u32(data.growth_min[index + 2]..=data.growth_max[index + 2]);
            *stat = stat.saturating_add((growth / 10).min(u32::from(u16::MAX)) as u16);
        }
    }
    recalculate_homunculus(homunculus);
    if homunculus.level == 99 {
        homunculus.experience = 0;
    }
    if gained > 0 {
        homunculus.hp = homunculus.max_hp;
        homunculus.sp = homunculus.max_sp;
    }
    Ok(gained)
}

pub(crate) fn homunculus_skill_level(homunculus: &HomunculusRecord, name: &str) -> u8 {
    let id = match name {
        "HLIF_BRAIN" => 8003,
        "HAMI_SKIN" => 8007,
        "HVAN_INSTRUCT" => 8015,
        _ => return 0,
    };
    homunculus.skills.get(&id).copied().unwrap_or(0)
}

pub(crate) fn homunculus_stats(homunculus: &HomunculusRecord) -> [u16; 6] {
    let mut stats = homunculus.stats;
    let instruction = usize::from(homunculus_skill_level(homunculus, "HVAN_INSTRUCT").min(5));
    stats[0] = stats[0].saturating_add([0, 1, 1, 3, 4, 4][instruction]);
    stats[3] = stats[3].saturating_add([0, 1, 2, 2, 4, 5][instruction]);
    stats
}

pub(crate) fn recalculate_homunculus(homunculus: &mut HomunculusRecord) {
    if homunculus.base_max_hp == 0 {
        homunculus.base_max_hp = homunculus.max_hp;
    }
    if homunculus.base_max_sp == 0 {
        homunculus.base_max_sp = homunculus.max_sp;
    }
    if let Some(snapshot) = homunculus_snapshot(homunculus, 150) {
        homunculus.max_hp = snapshot.max_hp();
        homunculus.max_sp = snapshot.max_sp();
    }
    homunculus.hp = homunculus.hp.min(homunculus.max_hp);
    homunculus.sp = homunculus.sp.min(homunculus.max_sp);
}

pub(crate) fn homunculus_status(homunculus: &HomunculusRecord) -> Status {
    let [strength, agility, vitality, intelligence, dexterity, luck] = homunculus_stats(homunculus);
    Status {
        job: u32::from(homunculus.class_id),
        base_level: u32::from(homunculus.level),
        hp: homunculus.hp,
        sp: homunculus.sp,
        max_hp: homunculus.max_hp,
        max_sp: homunculus.max_sp,
        str: strength,
        agi: agility,
        vit: vitality,
        int: intelligence,
        dex: dexterity,
        luk: luck,
        active_statuses: homunculus.statuses.clone(),
        ..Status::default()
    }
}

pub(crate) fn homunculus_snapshot(homunculus: &HomunculusRecord, speed: u16) -> Option<StatusSnapshot> {
    let data = world_data()
        .homunculi
        .iter()
        .find(|data| data.class_id == homunculus.class_id || data.evolution_class == homunculus.class_id)?;
    let [strength, agility, vitality, intelligence, dexterity, luck] = homunculus_stats(homunculus);
    let skin = homunculus_skill_level(homunculus, "HAMI_SKIN");
    let brain = homunculus_skill_level(homunculus, "HLIF_BRAIN");
    let max_hp = (u64::from(if homunculus.base_max_hp == 0 {
        homunculus.max_hp
    } else {
        homunculus.base_max_hp
    }) * (100 + 2 * u64::from(skin))
        / 100)
        .min(u64::from(u32::MAX)) as u32;
    let max_sp = (u64::from(if homunculus.base_max_sp == 0 {
        homunculus.max_sp
    } else {
        homunculus.base_max_sp
    }) * (100 + u64::from(brain))
        / 100)
        .min(u64::from(u32::MAX)) as u32;
    let weapon_max = strength.saturating_add(homunculus.level);
    let matk = intelligence.saturating_add((intelligence / 5).pow(2));
    let mut snapshot = StatusSnapshot::new_for_mob(
        u32::from(homunculus.class_id),
        homunculus.hp,
        homunculus.sp,
        max_hp,
        max_sp,
        strength,
        agility,
        vitality,
        intelligence,
        dexterity,
        luck,
        dexterity,
        weapon_max,
        matk,
        matk,
        speed,
        (homunculus.level / 10 + vitality / 5).min(99).saturating_add(4 * u16::from(skin)),
        (homunculus.level / 10 + intelligence / 5).min(99),
        Size::try_from_string(if homunculus.evolved { &data.evolution_size } else { &data.size }).unwrap_or(Size::Small),
        Element::try_from_string(&data.element).unwrap_or(Element::Neutral),
        MobRace::try_from_string(&data.race).unwrap_or(MobRace::DemiHuman),
        1,
    );
    snapshot.set_base_level(u32::from(homunculus.level));
    snapshot.set_combat_actor_kind(models::enums::actor::CombatActorKind::Homunculus);
    let bonuses = homunculus.statuses.iter().flat_map(|status| status.bonuses()).collect::<Vec<_>>();
    let (max_hp, max_sp) = super::companion_maximum_pools(
        max_hp,
        max_sp,
        &bonuses,
        homunculus.statuses.iter().any(|status| status.kind == StatusChangeKind::Berserk),
    );
    for bonus in &bonuses {
        bonus.add_bonus_to_status(&mut snapshot);
    }
    StatusEffectService::adjust_status_attributes(&homunculus_status(homunculus), &mut snapshot);
    let intelligence = snapshot.int();
    let matk = intelligence.saturating_add((intelligence / 5).pow(2));
    snapshot.set_matk_min(matk);
    snapshot.set_matk_max(matk);
    let strength = snapshot.str();
    snapshot.set_base_atk(
        if homunculus.statuses.iter().any(|status| status.kind == StatusChangeKind::HomChange) {
            matk
        } else {
            strength.saturating_add((strength / 10).pow(2))
        },
    );
    snapshot.set_hit(
        (i32::from(snapshot.hit()) + i32::from(homunculus.level) + i32::from(snapshot.dex()))
            .clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16,
    );
    snapshot.set_flee(
        (i32::from(snapshot.flee()) + i32::from(homunculus.level) + i32::from(snapshot.agi()))
            .clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16,
    );
    snapshot.set_crit(0.0);
    let delay = ((1000 - i32::from(snapshot.agi()) * 4 - i32::from(snapshot.dex())).max(0) as u64 * u64::from(data.attack_delay) / 1000)
        .clamp(100, 2000);
    snapshot.set_aspd(200.0 - delay as f32 / 10.0);
    for bonus in &bonuses {
        if !matches!(bonus, models::enums::bonus::BonusType::AspdPercentage(_)) {
            bonus.add_percentage_bonus_to_status(&mut snapshot);
        }
    }
    snapshot.bonuses_mut().extend(bonuses.into_iter().map(StatusBonus::new));
    snapshot.set_active_statuses(homunculus.statuses.clone());
    let rate = super::companion_attack_rate(&snapshot);
    snapshot.set_atk_left_side((f32::from(snapshot.base_atk()) * rate).floor() as i32 + i32::from(snapshot.bonus_atk()));
    snapshot.set_atk_right_side(if snapshot.has_status_change(StatusChangeKind::HomChange) {
        0
    } else {
        (f32::from(snapshot.str().saturating_add(homunculus.level)) * rate).floor() as i32
    });
    StatusEffectService::adjust_snapshot_for_target(&homunculus_status(homunculus), &mut snapshot, false);
    snapshot.set_max_hp(max_hp);
    snapshot.set_max_sp(max_sp);
    Some(snapshot)
}

fn feed_homunculus_record(homunculus: &mut HomunculusRecord) {
    let adjustment = match homunculus.hunger {
        91.. => -50,
        76.. => -5,
        26.. => 75,
        11.. => 100,
        _ => 50,
    };
    homunculus.intimacy = (i64::from(homunculus.intimacy) + i64::from(adjustment)).clamp(0, 100_000) as u32;
    homunculus.hunger = homunculus.hunger.saturating_add(10).min(100);
}

impl ScriptWorldService {
}

#[cfg(test)]
mod tests {
    use super::*;

    fn homunculus() -> HomunculusRecord {
        let data = &world_data().homunculi[0];
        HomunculusRecord {
            id: 1,
            class_id: data.class_id,
            level: 1,
            name: data.name.clone(),
            hp: 10,
            sp: 0,
            max_hp: data.base[0],
            max_sp: data.base[1],
            stats: data.base[2..]
                .iter()
                .map(|stat| *stat as u16)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            intimacy: 2100,
            hunger: 32,
            active: true,
            ..HomunculusRecord::default()
        }
    }

    #[test]
    fn classic_homunculus_experience_levels_grow_stats_and_award_skill_points() {
        let mut homunculus = homunculus();
        let mut rng = fastrand::Rng::with_seed(42);
        assert_eq!(gain_homunculus_experience(&mut homunculus, 49, &mut rng).unwrap(), 0);
        assert_eq!(homunculus.level, 1);
        assert_eq!(gain_homunculus_experience(&mut homunculus, 1, &mut rng).unwrap(), 1);
        assert_eq!((homunculus.level, homunculus.experience), (2, 0));
        assert!((210..=250).contains(&homunculus.max_hp));
        assert!((17..=18).contains(&homunculus.stats[0]));
        assert_eq!((homunculus.hp, homunculus.sp), (homunculus.max_hp, homunculus.max_sp));
        assert_eq!(gain_homunculus_experience(&mut homunculus, 130, &mut rng).unwrap(), 1);
        assert_eq!((homunculus.level, homunculus.skill_points, homunculus.experience), (3, 1, 20));
        gain_homunculus_experience(&mut homunculus, 1_000_000_000_000, &mut rng).unwrap();
        assert_eq!((homunculus.level, homunculus.experience, homunculus.skill_points), (99, 0, 33));
        assert_eq!(gain_homunculus_experience(&mut homunculus, 1000, &mut rng).unwrap(), 0);
    }

    #[test]
    fn homunculus_feeding_uses_classic_loyalty_scale_and_overfeeding_penalties() {
        for (hunger, delta) in [
            (0, 50i32),
            (10, 50),
            (11, 100),
            (25, 100),
            (26, 75),
            (75, 75),
            (76, -5),
            (90, -5),
            (91, -50),
            (100, -50),
        ] {
            let mut homunculus = homunculus();
            homunculus.hunger = hunger;
            feed_homunculus_record(&mut homunculus);
            assert_eq!(homunculus.intimacy, (2100 + delta) as u32);
            assert_eq!(homunculus.hunger, (hunger + 10).min(100));
        }
    }

    #[test]
    fn learned_passives_change_derived_stats_without_compounding_persistent_base_pools() {
        let mut homunculus = homunculus();
        homunculus.class_id = 6002;
        homunculus.base_max_hp = 1000;
        homunculus.max_hp = 1000;
        homunculus.base_max_sp = 100;
        homunculus.max_sp = 100;
        homunculus.skills.insert(8007, 5);
        for _ in 0..3 {
            recalculate_homunculus(&mut homunculus);
            assert_eq!((homunculus.max_hp, homunculus.base_max_hp), (1100, 1000));
        }
        homunculus.skills.remove(&8007);
        recalculate_homunculus(&mut homunculus);
        assert_eq!(homunculus.max_hp, 1000);
        homunculus.class_id = 6001;
        homunculus.skills.insert(8003, 5);
        for _ in 0..3 {
            recalculate_homunculus(&mut homunculus);
            assert_eq!((homunculus.max_sp, homunculus.base_max_sp), (105, 100));
        }
        homunculus.class_id = 6004;
        homunculus.skills.clear();
        homunculus.skills.insert(8015, 5);
        let stats = homunculus_stats(&homunculus);
        assert_eq!(stats[0], homunculus.stats[0] + 4);
        assert_eq!(stats[3], homunculus.stats[3] + 5);
    }

    #[test]
    fn mental_change_uses_buffed_maximum_matk_and_normal_attacks_keep_classic_weapon_bounds() {
        let mut character = crate::tests::common::character_helper::create_character();
        let mut homunculus = homunculus();
        homunculus.stats = [10, 10, 10, 50, 30, 10];
        homunculus.level = 10;
        character.game_systems.homunculus = Some(homunculus);
        let id = homunculus_world_id(character.game_systems.homunculus.as_ref().unwrap());
        assert_eq!(super::super::companion_attack_bounds(&character, id), Some((20, 20, 11)));
        let homunculus = character.game_systems.homunculus.as_mut().unwrap();
        let mut status = homunculus_status(homunculus);
        StatusEffectService::apply_status(
            &mut status,
            StatusChangeRequest::guaranteed(StatusChangeKind::HomChange, 1000, 1),
            0,
            0,
        )
        .unwrap();
        homunculus.statuses = status.active_statuses;
        recalculate_homunculus(homunculus);
        assert_eq!(super::super::companion_attack_bounds(&character, id), Some((0, 0, 266)));
        let mut rng = fastrand::Rng::with_seed(42);
        assert_eq!(
            super::super::companion_attack_damage(&character, id, false, &mut rng),
            Some(266)
        );
    }
}

fn validate_companion(character: &Character, id: u32) -> Result<(), String> {
    let homunculus = character
        .game_systems
        .homunculus
        .as_ref()
        .is_some_and(|homunculus| homunculus.active && homunculus.hp > 0 && homunculus_world_id(homunculus) == id);
    let mercenary = character
        .game_systems
        .mercenary
        .as_ref()
        .is_some_and(|mercenary| mercenary.hp > 0 && mercenary_world_id(mercenary) == id);
    if homunculus || mercenary {
        Ok(())
    } else {
        Err("Companion does not belong to this character".into())
    }
}
