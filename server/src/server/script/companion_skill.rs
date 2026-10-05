use models::enums::element::Element;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU32, EnumWithNumberValue};
use models::status::StatusSnapshot;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};

use super::ScriptSkillService;
use super::metadata::SkillMetadata;
use crate::server::Server;
use crate::server::model::action::Damage;

#[derive(Clone, Debug, Default)]
pub struct CompanionSkillContext {
    pub base_level: u32,
    pub intimacy: u32,
    pub brain_level: u8,
    pub enemy_target_id: Option<u32>,
    pub target_is_player: bool,
    pub raw_attack: u32,
    pub target_base_level: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompanionSkillEffect {
    Damage(Damage),
    Status {
        target_id: u32,
        request: StatusChangeRequest,
    },
    Heal {
        target_id: u32,
        hp: u32,
        sp: u32,
    },
    EndStatus {
        target_id: u32,
        kind: StatusChangeKind,
    },
    DelayedStatus {
        target_id: u32,
        request: StatusChangeRequest,
        delay_ms: u32,
    },
    Ground {
        skill_id: u32,
        level: u8,
        target_id: u32,
    },
    SwapPositions {
        source_id: u32,
        target_id: u32,
    },
    SetIntimacy(u32),
    SelfDestruct {
        delay_ms: u32,
    },
}

impl ScriptSkillService {
    pub fn resolve_companion_skill(
        &self,
        server: &Server,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        skill_id: u32,
        level: u8,
        source_id: u32,
        target_id: u32,
        owner_id: u32,
        tick: u128,
    ) -> Result<Vec<CompanionSkillEffect>, String> {
        self.resolve_companion_skill_with_context(
            server,
            source,
            target,
            skill_id,
            level,
            source_id,
            target_id,
            owner_id,
            tick,
            &CompanionSkillContext::default(),
        )
    }

    pub fn resolve_companion_skill_with_context(
        &self,
        server: &Server,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        skill_id: u32,
        level: u8,
        source_id: u32,
        target_id: u32,
        owner_id: u32,
        tick: u128,
        context: &CompanionSkillContext,
    ) -> Result<Vec<CompanionSkillEffect>, String> {
        use CompanionSkillEffect::*;
        let metadata = SkillMetadata::find(skill_id).ok_or("Companion skill has no pre-renewal definition")?;
        if level == 0 || level > metadata.max_level {
            return Err("Companion skill level is invalid".into());
        }
        let status_request = |kind: StatusChangeKind, recipient: u32| {
            let mut request = StatusChangeRequest::guaranteed(kind, metadata.duration(level, false).unwrap_or(0), level as i32);
            if matches!(kind, StatusChangeKind::HomDefence | StatusChangeKind::HomAvoid) && recipient == source_id {
                request.values[3] = 1;
            }
            if kind == StatusChangeKind::Devotion {
                request.values[0] = source_id as i32;
                request.values[1] = 0;
                request.values[2] = metadata.range(level).unwrap_or(9);
                request.duration_ms = metadata.duration(level, true).unwrap_or(0);
            }
            Status {
                target_id: recipient,
                request,
            }
        };
        match metadata.name.as_str() {
            "MA_SKIDTRAP" | "MA_LANDMINE" | "MA_SANDMAN" | "MA_FREEZINGTRAP" | "MA_SHOWER" => {
                return Ok(vec![Ground {
                    skill_id,
                    level,
                    target_id,
                }]);
            }
            "HLIF_BRAIN" | "HAMI_SKIN" | "HVAN_INSTRUCT" => return Ok(vec![]),
            "HLIF_AVOID" | "HAMI_DEFENCE" => {
                let kind = if metadata.name == "HLIF_AVOID" {
                    StatusChangeKind::HomAvoid
                } else {
                    StatusChangeKind::HomDefence
                };
                return Ok(vec![status_request(kind, source_id), status_request(kind, owner_id)]);
            }
            "HAMI_BLOODLUST" => return Ok(vec![status_request(StatusChangeKind::Bloodlust, source_id)]),
            "HFLI_FLEET" => return Ok(vec![status_request(StatusChangeKind::Fleet, source_id)]),
            "HFLI_SPEED" => return Ok(vec![status_request(StatusChangeKind::HomSpeed, source_id)]),
            "HLIF_CHANGE" => {
                return Ok(vec![
                    Heal {
                        target_id: source_id,
                        hp: source.max_hp(),
                        sp: source.max_sp(),
                    },
                    status_request(StatusChangeKind::HomChange, source_id),
                ]);
            }
            "HAMI_CASTLE" => {
                return Ok(if source_id != owner_id && fastrand::u32(0..100) < 20 * level as u32 {
                    vec![SwapPositions {
                        source_id,
                        target_id: owner_id,
                    }]
                } else {
                    vec![]
                });
            }
            "HLIF_HEAL" => {
                let healing =
                    Self::heal_amount(source, context.base_level, level).saturating_mul(100 + 2 * context.brain_level as u32) / 100;
                return Ok(vec![Heal {
                    target_id: owner_id,
                    hp: healing,
                    sp: 0,
                }]);
            }
            "HVAN_CHAOTIC" => {
                let level_index = level as usize - 1;
                let roll = fastrand::u32(1..=100);
                let recipient = if roll <= [20, 50, 25, 50, 34][level_index] {
                    source_id
                } else if roll <= [50, 60, 75, 54, 67][level_index] {
                    owner_id
                } else {
                    context.enemy_target_id.unwrap_or(source_id)
                };
                return Ok(vec![Heal {
                    target_id: recipient,
                    hp: Self::heal_amount(source, context.base_level, fastrand::u8(1..=level)),
                    sp: 0,
                }]);
            }
            "MER_RECUPERATE" => {
                return Ok(
                    [StatusChangeKind::Poison, StatusChangeKind::DeadlyPoison, StatusChangeKind::Silence]
                        .into_iter()
                        .map(|kind| EndStatus { target_id, kind })
                        .collect(),
                );
            }
            "MER_MENTALCURE" => {
                return Ok(vec![EndStatus {
                    target_id,
                    kind: StatusChangeKind::Confusion,
                }]);
            }
            "MER_REGAIN" => {
                return Ok([StatusChangeKind::Stun, StatusChangeKind::Sleep]
                    .into_iter()
                    .map(|kind| EndStatus {
                        target_id: source_id,
                        kind,
                    })
                    .collect());
            }
            "MER_TENDER" => {
                return Ok([StatusChangeKind::Freeze, StatusChangeKind::Stone]
                    .into_iter()
                    .map(|kind| EndStatus { target_id, kind })
                    .collect());
            }
            "MER_BENEDICTION" => {
                return Ok([StatusChangeKind::Curse, StatusChangeKind::Blind]
                    .into_iter()
                    .map(|kind| EndStatus { target_id, kind })
                    .collect());
            }
            "MER_COMPRESS" => {
                return Ok(vec![EndStatus {
                    target_id: source_id,
                    kind: StatusChangeKind::Bleeding,
                }]);
            }
            "MER_SCAPEGOAT" => {
                return Ok(vec![
                    Heal {
                        target_id: owner_id,
                        hp: source.hp(),
                        sp: 0,
                    },
                    SelfDestruct { delay_ms: 0 },
                ]);
            }
            "MER_LEXDIVINA" => {
                if target.has_status_change(StatusChangeKind::Silence) {
                    return Ok(vec![EndStatus {
                        target_id,
                        kind: StatusChangeKind::Silence,
                    }]);
                }
                let request = StatusChangeRequest {
                    kind: StatusChangeKind::Silence,
                    duration_ms: metadata.duration(level, true).unwrap_or(0),
                    values: [level as i32, 0, 0, 0],
                    rate: 10000,
                    flags: 0,
                };
                return Ok(vec![DelayedStatus {
                    target_id,
                    request,
                    delay_ms: 1000,
                }]);
            }
            "MER_DECAGI" | "MER_PROVOKE" => {
                let kind = if metadata.name == "MER_DECAGI" {
                    StatusChangeKind::DecreaseAgi
                } else {
                    StatusChangeKind::Provoke
                };
                if kind == StatusChangeKind::Provoke && Self::undead_target(target) {
                    return Ok(vec![]);
                }
                let chance = if kind == StatusChangeKind::DecreaseAgi {
                    50 + 3 * level as i32 + (context.base_level as i32 + source.int() as i32) / 5
                } else {
                    70 + 3 * level as i32 + context.base_level as i32 - context.target_base_level as i32
                };
                let mut request = StatusChangeRequest {
                    kind,
                    duration_ms: metadata.duration(level, false).unwrap_or(0),
                    values: [level as i32, 0, 0, 0],
                    rate: (chance.max(0) * 100).min(u16::MAX as i32) as u16,
                    flags: 0,
                };
                if kind == StatusChangeKind::Provoke && level == 10 {
                    request.values[2] = 100;
                }
                return Ok(vec![Status { target_id, request }]);
            }
            "ML_DEVOTION" => {
                if target_id != owner_id
                    || context.base_level.abs_diff(context.target_base_level) > 10
                    || target.has_status_change(StatusChangeKind::HellPower)
                    || target
                        .status_change(StatusChangeKind::Devotion)
                        .is_some_and(|change| change.values[0] as u32 != source_id)
                {
                    return Err("Mercenary Devotion requires an eligible owner within ten levels".into());
                }
                return Ok(vec![status_request(StatusChangeKind::Devotion, target_id)]);
            }
            _ => {}
        }
        if metadata.damage_flags.get("NoDamage").copied().unwrap_or(false) || metadata.damage_type.is_none() {
            if let Some(kind) = metadata.status.as_deref().and_then(StatusChangeKind::from_name) {
                return Ok(vec![status_request(
                    kind,
                    if matches!(metadata.target_type.as_deref(), Some("Support" | "Attack")) {
                        target_id
                    } else {
                        source_id
                    },
                )]);
            }
        }
        let actual_skill = if metadata.name == "HVAN_CAPRICE" {
            [
                SkillEnum::MgColdbolt,
                SkillEnum::MgFirebolt,
                SkillEnum::MgLightningbolt,
                SkillEnum::WzEarthspike,
            ][fastrand::usize(0..4)]
            .id()
        } else {
            skill_id
        };
        let mut effects = vec![];
        let object = skills::skill_enums::to_object(SkillEnum::from_id(actual_skill), level);
        let offensive = if matches!(metadata.name.as_str(), "HFLI_SBR44" | "HVAN_EXPLOSION") {
            None
        } else {
            object.as_ref().and_then(|skill| skill.as_offensive_skill())
        };
        let landed =
            metadata.damage_type.as_deref() != Some("Weapon") || server.battle_service().skill_hits(source, target, actual_skill, level);
        let (damage, magic_context, battle_flags) = if Self::uses_metadata_magic(&metadata.name) {
            let (damage, context) = server.battle_service().metadata_magic_damage(source, target, metadata, level)?;
            if metadata.name == "NPC_MAGICALATTACK" {
                effects.push(Status { target_id: source_id,
                    request: StatusChangeRequest::guaranteed(StatusChangeKind::MagicalAttack, metadata.duration(level, false).unwrap_or(0), i32::from(level)) });
            } else if metadata.name == "SL_SMA" {
                effects.push(EndStatus { target_id: source_id, kind: StatusChangeKind::Sma });
            } else if level >= 7 && !source.status_change(StatusChangeKind::Sma).is_some_and(|ready| !ready.expired(tick)) {
                let duration = SkillMetadata::find(SkillEnum::SlSma.id()).and_then(|metadata| metadata.duration(level, false)).unwrap_or(3000);
                effects.push(Status { target_id: source_id,
                    request: StatusChangeRequest::guaranteed(StatusChangeKind::Sma, duration, i32::from(level)) });
            }
            (damage, Some(context), metadata.battle_flags(true))
        } else if let Some(offensive) = offensive {
            let flags = (if crate::server::service::battle_service::BattleService::is_weapon_skill(offensive) {
                BattleFlag::Weapon
            } else if offensive.is_magic() {
                BattleFlag::Magic
            } else {
                BattleFlag::Misc
            })
            .as_flag()
                | (if offensive.is_ranged() || offensive.is_magic() {
                    BattleFlag::Long
                } else {
                    BattleFlag::Short
                })
                .as_flag()
                | BattleFlag::Skill.as_flag();
            if !landed {
                (0, None, flags)
            } else if crate::server::service::battle_service::BattleService::is_weapon_skill(offensive) {
                (
                    server.battle_service().actor_physical_skill_damage_signed(
                        context.raw_attack,
                        source,
                        target,
                        context.target_is_player,
                        offensive.dmg_atk().unwrap_or(1.0),
                        offensive.hit_count() as i16,
                        &server.battle_service().attack_element(source, Some(offensive)),
                        flags,
                        actual_skill,
                    ),
                    None,
                    flags,
                )
            } else {
                let (damage, magic) = server
                    .battle_service()
                    .calculate_damage_with_context(source, target, Some(offensive));
                (damage, magic, flags)
            }
        } else {
            let flags = metadata.battle_flags(metadata.range(level).unwrap_or(1) > 3 || metadata.damage_type.as_deref() == Some("Magic"));
            match metadata.name.as_str() {
                "HFLI_MOON" => (
                    if landed {
                        server.battle_service().actor_physical_skill_damage_signed(
                            context.raw_attack,
                            source,
                            target,
                            context.target_is_player,
                            (110 + 110 * level as u32) as f32 / 100.0,
                            metadata.hit_count.as_ref().and_then(|hits| hits.value(level, "Count")).unwrap_or(1) as i16,
                            &Element::Neutral,
                            flags,
                            actual_skill,
                        )
                    } else {
                        0
                    },
                    None,
                    flags,
                ),
                "HFLI_SBR44" => {
                    if context.intimacy < 400 {
                        return Err("SBR44 requires more than Hate with Passion intimacy".into());
                    }
                    effects.push(SetIntimacy(100));
                    (
                        server
                            .battle_service()
                            .actor_physical_damage(
                                context.intimacy.saturating_mul(level as u32),
                                source,
                                target,
                                context.target_is_player,
                                false,
                                &Element::Neutral,
                                flags,
                            )
                            .min(i32::MAX as u32) as i32,
                        None,
                        flags,
                    )
                }
                "HVAN_EXPLOSION" => {
                    if context.intimacy < 45000 {
                        return Err("Bio Explosion requires at least 450 intimacy".into());
                    }
                    effects.push(SetIntimacy(100));
                    effects.push(SelfDestruct {
                        delay_ms: metadata.duration(level, false).unwrap_or(1500).max(0) as u32,
                    });
                    (
                        (source.max_hp().saturating_mul(50 + 50 * level as u32) / 100).min(i32::MAX as u32) as i32,
                        None,
                        flags,
                    )
                }
                "MS_BASH" | "MS_MAGNUM" | "MS_BOWLINGBASH" | "MA_DOUBLE" | "MA_SHOWER" | "MA_CHARGEARROW" | "MA_SHARPSHOOTING"
                | "ML_PIERCE" | "ML_BRANDISH" | "ML_SPIRALPIERCE" | "MER_CRASH" => {
                    let (ratio, hits, raw) =
                        Self::mercenary_weapon_formula(metadata.name.as_str(), level, source, target, context.raw_attack)?;
                    let element = if metadata.name == "MS_MAGNUM" {
                        Element::Fire
                    } else {
                        server.battle_service().attack_element(source, None)
                    };
                    (
                        if landed {
                            server.battle_service().actor_physical_skill_damage_signed(
                                raw,
                                source,
                                target,
                                context.target_is_player,
                                ratio,
                                hits,
                                &element,
                                flags,
                                actual_skill,
                            )
                        } else {
                            0
                        },
                        None,
                        flags,
                    )
                }
                _ => return Err(format!("Companion damage skill {} has no resolved formula", metadata.name)),
            }
        };
        let mut damage_event = crate::server::model::action::Damage { notification: None,
            source_kind: *source.combat_actor_kind(),
            skill_damage_adjusted: false,
            healing: 0,
            right_hand_damage: None,
            target_id,
            attacker_id: source_id,
            damage: 0,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags,
            skill_id: actual_skill,
            skill_level: level,
            proc_depth: 0,
            credit_id: owner_id,
            defenses_applied: true,
            magic_context,
            landed,
        };
        damage_event.set_signed_damage(damage);
        effects.insert(0, Damage(damage_event));
        Ok(effects)
    }

    pub fn mercenary_weapon_formula(
        name: &str,
        level: u8,
        source: &StatusSnapshot,
        target: &StatusSnapshot,
        raw: u32,
    ) -> Result<(f32, i16, u32), String> {
        let level = u32::from(level);
        let (percent, hits, raw) = match name {
            "MS_BASH" => (100 + 30 * level, 1, raw),
            "MS_MAGNUM" => (100 + 10 * level, 1, raw),
            "MS_BOWLINGBASH" => (100 + 40 * level, 1, raw),
            "MA_DOUBLE" => ((90 + 10 * level) * 2, 2, raw),
            "MA_SHOWER" => (75 + 5 * level, 1, raw),
            "MA_CHARGEARROW" => (150, 1, raw),
            "MA_SHARPSHOOTING" => (200 + 50 * level, 1, raw),
            "ML_PIERCE" => {
                let hits = target.size().value() as i16 + 1;
                ((100 + 10 * level) * hits as u32, hits, raw)
            }
            "ML_BRANDISH" => {
                let base = 100 + 20 * level;
                (
                    base + if level > 3 { base / 2 } else { 0 }
                        + if level > 6 { base / 4 } else { 0 }
                        + if level > 9 { base / 8 } else { 0 },
                    1,
                    raw,
                )
            }
            "ML_SPIRALPIERCE" => {
                let raw = raw.saturating_add(u32::from(source.str() / 10).pow(2));
                let raw = raw.saturating_mul(match target.size() {
                    models::enums::size::Size::Small => 125,
                    models::enums::size::Size::Large => 75,
                    _ => 100,
                }) / 100;
                (500, 5, raw)
            }
            "MER_CRASH" => (100 + 10 * level, 1, raw),
            _ => return Err("Not a pre-renewal mercenary weapon skill".into()),
        };
        Ok((percent as f32 / 100.0, hits, raw))
    }
}
