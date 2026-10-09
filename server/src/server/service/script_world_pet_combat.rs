use configuration::configuration::PetSupportConfig;
use models::enums::element::Element;
use models::enums::mob::MobRace;
use models::status::StatusSnapshot;
use models::status_change::StatusChangeKind;
use skills::{ActorBehaviour, SupportChance, SupportProfile};
use script_sdk::{Function, Value};

use super::{ScriptWorldService, companion_status_snapshot, pet_world_id, protocol, world_data};
use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterStatusChange, GameEvent};
use crate::server::model::events::map_event::{MapEvent, MobProvoke, MobDamage, MobEndStatus, MobStatusChange};
use crate::server::model::game_systems::{PetAttackSkill, PetRecord, PetSupportCast};
use crate::server::model::map_flags::MapFlag;
use crate::server::script::skill::companion::{CompanionSkillContext, CompanionSkillEffect};
use crate::server::script::skill::ScriptSkillService;
use crate::server::script::skill::metadata::SkillMetadata;
use crate::server::script::skill::{FixedGroundSkillDamage, GroundSkillSource};
use crate::server::service::battle_service::BattleService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

pub(super) fn configure_pet_attack(function: Function, args: &[Value]) -> Result<PetAttackSkill, String> {
    let config = GlobalConfigService::instance()
        .find_skill_config(args.first().ok_or("Missing pet attack skill")?)
        .ok_or("Unknown pet attack skill")?;
    let metadata = SkillMetadata::find(config.id).ok_or("Pet attack skill has no classic definition")?;
    if !matches!(metadata.target_type.as_deref(), Some("Attack" | "Ground" | "Self")) {
        return Err("Pet attack requires an offensive skill".into());
    }
    if metadata.target_type.as_deref() == Some("Ground") && !ScriptSkillService::pet_ground_attack(metadata) {
        return Err(format!("Pet ground attack {} has no implemented actor unit", metadata.name));
    }
    if metadata.target_type.as_deref() == Some("Self")
        && (metadata.damage_flags.get("NoDamage").copied().unwrap_or(false) || metadata.damage_type.is_none())
    {
        return Err("Pet attack requires a damaging self skill".into());
    }
    let number = |index: usize| args.get(index).ok_or("Missing pet attack argument")?.number_value();
    if function == Function::PetSkillAttack2 {
        let damage = u32::try_from(number(1)?).map_err(|_| "Pet fixed damage must be nonnegative")?;
        let hits = i16::try_from(number(2)?)
            .ok()
            .filter(|hits| *hits > 0)
            .ok_or("Pet fixed attack requires a positive hit count")?;
        Ok(PetAttackSkill {
            skill_id: config.id,
            level: metadata.max_level,
            fixed_damage: Some(damage),
            hits,
            rate: number(3)?.clamp(0, 100),
            bonus_rate: number(4)?.clamp(0, 100),
        })
    } else {
        let level = u8::try_from(number(1)?)
            .ok()
            .filter(|level| *level > 0)
            .ok_or("Pet attack level must be positive")?
            .min(metadata.max_level);
        let skill = skills::skill_enums::to_object(models::enums::skill_enums::SkillEnum::from_id(config.id), level);
        if skill.as_ref().and_then(|skill| skill.as_offensive_skill()).is_none()
            && !(metadata.damage_flags.get("NoDamage").copied().unwrap_or(false) && metadata.status.is_some())
            && metadata.unit.is_none()
            && ScriptSkillService::weapon_ratio(metadata, level).is_none()
        {
            return Err(format!("Pet attack {} has no implemented effect", metadata.name));
        }
        Ok(PetAttackSkill {
            skill_id: config.id,
            level,
            fixed_damage: None,
            hits: 0,
            rate: number(2)?.clamp(0, 100),
            bonus_rate: number(3)?.clamp(0, 100),
        })
    }
}

pub(super) fn pet_combat_rate(pet: &PetRecord, definition_rate: u16, config: &PetSupportConfig) -> u32 {
    let minimum = u32::from(config.minimum_intimacy.min(999));
    let intimacy = pet.intimacy.clamp(0, 1000) as u32;
    if intimacy < minimum {
        return 0;
    }
    let modifier = u64::from(1000 * (intimacy - minimum) / (1000 - minimum) + 500) * u64::from(config.support_rate) / 100;
    let rate = u64::from(definition_rate) * modifier / 1000;
    if definition_rate > 0 { rate.max(1).min(10000) as u32 } else { 0 }
}

pub(super) fn pet_snapshot(character: &Character) -> Option<StatusSnapshot> {
    let pet = character.game_systems.pet.as_ref().filter(|pet| !pet.incubating)?;
    let data = GlobalConfigService::instance().get_mob_safe(i32::from(pet.class_id))?;
    let mut status = crate::server::model::status::StatusFromDb::from_mob_model(data);
    status.set_combat_actor_kind(models::enums::actor::CombatActorKind::Pet);
    status.set_base_level(u32::from(pet.level));
    status.set_hit((i32::from(pet.level) + i32::from(status.dex())).clamp(0, i32::from(i16::MAX)) as i16);
    status.set_flee((i32::from(pet.level) + i32::from(status.agi())).clamp(0, i32::from(i16::MAX)) as i16);
    status.set_crit(1.0 + (u32::from(status.luk()) * 10 / 3) as f32 / 10.0);
    status.set_aspd(200.0 - data.atk_delay.max(100) as f32 / 10.0);
    status.set_speed(character.status.speed);
    Some(status)
}

impl ScriptWorldService {
    pub(crate) fn pet_combat_target(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        target_id: u32,
        retaliation: bool,
        now: u64,
    ) -> Result<(), String> {
        self.initialize_pet_support(server, character, now)?;
        let config = self.pet_configuration();
        if !(if retaliation {
            config.damage_support
        } else {
            config.attack_support
        }) {
            return Ok(());
        }
        let Some(pet) = character
            .game_systems
            .pet
            .as_ref()
            .filter(|pet| !pet.incubating && pet.hunger > 0 && pet.intimacy >= i32::from(config.minimum_intimacy))
        else {
            return Ok(());
        };
        if character.status.hp == 0 || state.contains_locked_map_item(target_id) {
            return Ok(());
        }
        let map = state
            .get_map_instance_from_character(character)
            .ok_or("Pet combat map is unavailable")?;
        let Some(target) = map
            .state()
            .get_mob(target_id)
            .cloned()
            .filter(|target| target.hp() > 0 && target.summon_ai == 0 && u16::try_from(target.mob_id) != Ok(pet.class_id))
        else {
            return Ok(());
        };
        let source = pet_snapshot(character).ok_or("Pet monster data is unavailable")?;
        if !super::companion_can_target(
            &source,
            &target.status,
            crate::server::service::visibility_service::TargetingMode::Direct,
        ) {
            return Ok(());
        }
        let definition = world_data()
            .pets
            .iter()
            .find(|definition| definition.class_id == pet.class_id)
            .ok_or("Pet definition is unavailable")?;
        let data = self
            .configuration
            .get_mob_safe(i32::from(pet.class_id))
            .ok_or("Pet monster data is unavailable")?;
        let actor_id = pet_world_id(pet.id);
        let position = character.game_systems.rendered_companions.get(&actor_id).copied().unwrap_or(
            crate::server::model::game_systems::CompanionPosition {
                x: character.x,
                y: character.y,
                map_instance: character.current_map_instance(),
            },
        );
        if position.x.abs_diff(target.x).max(position.y.abs_diff(target.y)) > data.range2.max(1) as u16 {
            return Ok(());
        }
        let rate = pet_combat_rate(
            pet,
            if retaliation {
                definition.retaliation_rate
            } else {
                definition.attack_rate
            },
            config,
        );
        if fastrand::u32(0..10000) >= rate {
            return Ok(());
        }
        let command = character.game_systems.companion_commands.entry(actor_id).or_default();
        if command.target.is_none() || fastrand::u32(0..10000) < u32::from(definition.change_target_rate) {
            command.target = Some(target_id);
            command.repeat = true;
            command.stay = false;
        }
        Ok(())
    }

    pub(crate) fn try_pet_attack_skill(
        &self,
        server: &Server,
        state: &ServerState,
        character: &mut Character,
        target_id: u32,
        now: u64,
    ) -> Result<bool, String> {
        let config = self.pet_configuration();
        let Some(pet) = character
            .game_systems
            .pet
            .as_ref()
            .filter(|pet| {
                !pet.incubating
                    && pet.hunger > 0
                    && pet.intimacy >= i32::from(config.minimum_intimacy)
                    && (!config.require_accessory || pet.equipped_item > 0)
            })
            .cloned()
        else {
            return Ok(false);
        };
        if !config.status_support || state.map_flags(&character.map_instance_key).enabled(MapFlag::NoSkill) {
            return Ok(false);
        }
        let Some(support) = character
            .game_systems
            .pet_support
            .as_ref()
            .filter(|support| support.pet_id == pet.id)
        else {
            return Ok(false);
        };
        if support.casting.is_some() || now < support.can_act_at {
            return Ok(true);
        }
        let Some(attack) = support.attack.clone() else {
            return Ok(false);
        };
        let chance = (attack.rate + pet.intimacy.clamp(0, 1000) * attack.bonus_rate / 1000).clamp(0, 100) as u32;
        if fastrand::u32(0..100) >= chance {
            return Ok(false);
        }
        let map = state
            .get_map_instance_from_character(character)
            .ok_or("Pet combat map is unavailable")?;
        let Some(target) = map
            .state()
            .get_mob(target_id)
            .cloned()
            .filter(|mob| mob.hp() > 0 && mob.summon_ai == 0)
        else {
            return Ok(false);
        };
        if state.contains_locked_map_item(target_id) {
            return Ok(false);
        }
        let source = pet_snapshot(character).ok_or("Pet monster data is unavailable")?;
        let metadata = SkillMetadata::find(attack.skill_id).ok_or("Pet skill definition is unavailable")?;
        let actor_id = pet_world_id(pet.id);
        let position = character.game_systems.rendered_companions.get(&actor_id).copied().unwrap_or(
            crate::server::model::game_systems::CompanionPosition {
                x: character.x,
                y: character.y,
                map_instance: character.current_map_instance(),
            },
        );
        let range = metadata.range(attack.level).unwrap_or(1).unsigned_abs().max(1);
        if u32::from(position.x.abs_diff(target.x).max(position.y.abs_diff(target.y))) > range {
            return Ok(false);
        }
        let cast_time = super::companion_skills::companion_cast_time(&source, metadata, attack.level);
        let cast_id = self
            .next_pet_cast_id
            .try_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |value| value.checked_add(1),
            )
            .map_err(|_| "Pet cast identifiers exhausted")?;
        character.game_systems.pet_support.as_mut().unwrap().casting = Some(PetSupportCast {
            id: cast_id,
            skill_id: attack.skill_id,
            level: attack.level,
            target_id,
            attack: Some(attack.clone()),
            map_key: character.map_instance_key.clone(),
            completes_at: now.saturating_add(cast_time),
            queued: false,
        });
        let command = character.game_systems.companion_commands.entry(actor_id).or_default();
        command.can_act_at = now.saturating_add(cast_time);
        command.next_move_at = command.can_act_at;
        command.destination = None;
        if cast_time == 0 {
            self.finish_pet_support(server, state, character, cast_id, now)?;
            return Ok(true);
        }
        let mut packet = protocol::header(0x013E);
        packet.extend_from_slice(&actor_id.to_le_bytes());
        packet.extend_from_slice(&target_id.to_le_bytes());
        packet.extend_from_slice(&target.x.to_le_bytes());
        packet.extend_from_slice(&target.y.to_le_bytes());
        packet.extend_from_slice(&(attack.skill_id as u16).to_le_bytes());
        packet.extend_from_slice(&0u32.to_le_bytes());
        packet.extend_from_slice(&(cast_time.min(u64::from(u32::MAX)) as u32).to_le_bytes());
        self.area(character, packet)?;
        Ok(true)
    }

    pub(super) fn finish_pet_attack(
        &self,
        server: &Server,
        state: &ServerState,
        character: &Character,
        pet: &PetRecord,
        attack: &PetAttackSkill,
        target_id: u32,
        now: u64,
    ) -> Result<(), String> {
        let config = self.pet_configuration();
        if !config.status_support
            || !(config.attack_support || config.damage_support)
            || pet.hunger == 0
            || pet.intimacy < i32::from(config.minimum_intimacy)
            || state.contains_locked_map_item(target_id)
        {
            return Ok(());
        }
        let map = state
            .get_map_instance_from_character(character)
            .ok_or("Pet combat map is unavailable")?;
        let Some(target) = map
            .state()
            .get_mob(target_id)
            .cloned()
            .filter(|target| target.hp() > 0 && target.summon_ai == 0)
        else {
            return Ok(());
        };
        let actor_id = pet_world_id(pet.id);
        let source = companion_status_snapshot(character, actor_id).ok_or("Pet monster data is unavailable")?;
        let metadata = SkillMetadata::find(attack.skill_id).ok_or("Pet skill definition is unavailable")?;
        if !super::companion_can_target(
            &source,
            &target.status,
            crate::server::service::visibility_service::TargetingMode::SkillCompletion {
                can_hit_hidden: metadata.flags.get("TargetHidden").copied().unwrap_or(false),
            },
        ) {
            return Ok(());
        }
        let position = character.game_systems.rendered_companions.get(&actor_id).copied().unwrap_or(
            crate::server::model::game_systems::CompanionPosition {
                x: character.x,
                y: character.y,
                map_instance: character.current_map_instance(),
            },
        );
        let ground_source = GroundSkillSource {
            actor_id,
            owner_id: character.char_id,
            map: character.current_map_name().clone(),
            instance: character.current_map_instance(),
            x: position.x,
            y: position.y,
            status: source.clone(),
            raw_attack: super::companion_attack_damage(character, actor_id, false, &mut fastrand::Rng::new()).unwrap_or(0),
            fixed_damage: attack.fixed_damage.map(|amount| FixedGroundSkillDamage {
                amount,
                hits: attack.hits,
                ignore_infinite_defense: config.ignore_infinite_defense,
            }),
        };
        if u32::from(position.x.abs_diff(target.x).max(position.y.abs_diff(target.y)))
            > metadata.range(attack.level).unwrap_or(1).unsigned_abs().max(1)
        {
            return Ok(());
        }
        if let (ActorBehaviour::AreaStatus { at_target_point: true }, Some((request, delay))) = (
            ScriptSkillService::skill_behaviour(attack.skill_id, attack.level),
            ScriptSkillService::area_status_request(attack.skill_id, attack.level, false, 0),
        ) {
            let radius = metadata.splash(attack.level).unwrap_or(2).unsigned_abs().min(u32::from(u16::MAX)) as u16;
            for mob in map
                .state()
                .mobs()
                .values()
                .filter(|mob| mob.hp() > 0 && mob.summon_ai == 0 && mob.x.abs_diff(target.x).max(mob.y.abs_diff(target.y)) <= radius)
            {
                map.add_to_delayed_tick(
                    MapEvent::MobStatusChange(MobStatusChange {
                        mob_id: mob.id,
                        request: request.clone(),
                    }),
                    delay,
                );
            }
            for player in state.characters().values().filter(|player| {
                player.map_instance_key == character.map_instance_key
                    && player.x.abs_diff(target.x).max(player.y.abs_diff(target.y)) <= radius
                    && server.player_combat_target_allowed(state, character, player.char_id)
            }) {
                server.add_to_delayed_tick(
                    GameEvent::CharacterStatusChange(CharacterStatusChange {
                        char_id: player.char_id,
                        request: request.clone(),
                    }),
                    delay,
                );
            }
            let mut packet = protocol::header(0x0117);
            packet.extend_from_slice(&(attack.skill_id as u16).to_le_bytes());
            packet.extend_from_slice(&actor_id.to_le_bytes());
            packet.extend_from_slice(&i16::from(attack.level).to_le_bytes());
            packet.extend_from_slice(&target.x.to_le_bytes());
            packet.extend_from_slice(&target.y.to_le_bytes());
            packet.extend_from_slice(&(now as u32).to_le_bytes());
            return self.area(character, packet);
        }
        if metadata.target_type.as_deref() == Some("Ground") {
            server.script_skill_service().validate_actor_ground(
                state,
                &ground_source,
                attack.skill_id,
                attack.level,
                target.x,
                target.y,
                now as u128,
            )?;
            return server.script_skill_service().place_actor_ground_skill(
                server,
                state,
                ground_source,
                attack.skill_id,
                attack.level,
                target.x,
                target.y,
                now as u128,
            );
        }
        let mut effects = if let Some(fixed) = attack.fixed_damage {
            let element = pet_skill_element(server, &source, metadata, attack.level);
            let flags =
                metadata.battle_flags(metadata.range(attack.level).unwrap_or(1) > 3 || metadata.damage_type.as_deref() == Some("Magic"));
            let signed = if !config.ignore_infinite_defense && server.battle_service().is_infinite_defense(&target.status, flags) {
                i32::from(attack.hits)
            } else {
                (fixed as f64 * f64::from(BattleService::element_modifier(&element, &target.status)))
                    .floor()
                    .clamp(i32::MIN as f64, i32::MAX as f64) as i32
            };
            let mut damage = Damage {
                notification: None,
                source_kind: models::enums::actor::CombatActorKind::Pet,
                skill_damage_adjusted: false,
                target_id,
                attacker_id: actor_id,
                credit_id: character.char_id,
                damage: 0,
                healing: 0,
                right_hand_damage: None,
                attacked_at: now as u128,
                damage_motion: target.damage_motion,
                battle_flags: flags,
                skill_id: attack.skill_id,
                skill_level: attack.level,
                landed: true,
                proc_depth: 0,
                defenses_applied: true,
                magic_context: None,
            };
            damage.set_signed_damage(signed);
            vec![CompanionSkillEffect::Damage(damage)]
        } else if let Some(ratio) = ScriptSkillService::weapon_ratio(metadata, attack.level) {
            let chance = ((80 + i32::from(source.hit()) - i32::from(target.status.flee())) * 120 / 100).clamp(5, 95);
            let hit = fastrand::i32(0..100) < chance;
            let flags = metadata.battle_flags(metadata.range(attack.level).unwrap_or(1).unsigned_abs() > 3);
            let signed = if hit {
                server.battle_service().actor_physical_skill_damage_signed(
                    ground_source.raw_attack,
                    &source,
                    &target.status,
                    false,
                    ratio,
                    1,
                    &pet_skill_element(server, &source, metadata, attack.level),
                    flags,
                    attack.skill_id,
                )
            } else {
                0
            };
            let mut damage = Damage {
                notification: None,
                source_kind: models::enums::actor::CombatActorKind::Pet,
                skill_damage_adjusted: false,
                target_id,
                attacker_id: actor_id,
                credit_id: character.char_id,
                damage: 0,
                healing: 0,
                right_hand_damage: None,
                attacked_at: now as u128,
                damage_motion: target.damage_motion,
                battle_flags: flags,
                skill_id: attack.skill_id,
                skill_level: attack.level,
                landed: hit,
                proc_depth: 0,
                defenses_applied: true,
                magic_context: None,
            };
            damage.set_signed_damage(signed);
            vec![CompanionSkillEffect::Damage(damage)]
        } else {
            server.script_skill_service().resolve_companion_skill_with_context(
                server,
                &source,
                &target.status,
                attack.skill_id,
                attack.level,
                actor_id,
                target_id,
                character.char_id,
                now as u128,
                &CompanionSkillContext {
                    base_level: u32::from(pet.level),
                    intimacy: pet.intimacy.max(0) as u32,
                    raw_attack: ground_source.raw_attack,
                    target_base_level: target.status.base_level(),
                    ..CompanionSkillContext::default()
                },
            )?
        };
        if matches!(
            ScriptSkillService::skill_behaviour(attack.skill_id, attack.level),
            ActorBehaviour::Support(SupportProfile { chance: SupportChance::Provoke, .. })
        ) {
            if *target.status.race() == MobRace::RUndead || *target.status.element() == Element::Undead {
                return Ok(());
            }
            let chance = (70 + 3 * i32::from(attack.level) + i32::from(pet.level) - target.status.base_level() as i32).clamp(0, 100);
            for effect in &mut effects {
                if let CompanionSkillEffect::Status { request, .. } = effect {
                    request.rate = (chance * 100) as u16;
                    request.flags = 0;
                }
            }
        }
        for effect in effects {
            match effect {
                CompanionSkillEffect::Damage(damage) => {
                    let damage = damage.with_skill_notification(
                        character.current_map_name(),
                        character.current_map_instance(),
                        ground_source.x,
                        ground_source.y,
                        now as u128,
                        attack.hits.max(1),
                        0,
                    );
                    map.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage }));
                }
                CompanionSkillEffect::Status { target_id, request } => {
                    if request.kind == StatusChangeKind::Provoke {
                        map.add_to_next_tick(MapEvent::MobProvoke(MobProvoke {
                            mob_id: target_id,
                            source_id: actor_id,
                            request,
                            coma: Default::default(),
                        }));
                    } else {
                        map.add_to_next_tick(MapEvent::MobStatusChange(MobStatusChange {
                            mob_id: target_id,
                            request,
                        }));
                    }
                }
                CompanionSkillEffect::DelayedStatus {
                    target_id,
                    request,
                    delay_ms,
                } => map.add_to_delayed_tick(
                    MapEvent::MobStatusChange(MobStatusChange {
                        mob_id: target_id,
                        request,
                    }),
                    u128::from(delay_ms),
                ),
                CompanionSkillEffect::EndStatus { target_id, kind } => map.add_to_next_tick(MapEvent::MobEndStatus(MobEndStatus {
                    mob_id: target_id,
                    kind: Some(kind),
                })),
                CompanionSkillEffect::Ground { skill_id, level, .. } => {
                    server.script_skill_service().validate_actor_ground(
                        state,
                        &ground_source,
                        skill_id,
                        level,
                        target.x,
                        target.y,
                        now as u128,
                    )?;
                    server.script_skill_service().place_actor_ground_skill(
                        server,
                        state,
                        ground_source.clone(),
                        skill_id,
                        level,
                        target.x,
                        target.y,
                        now as u128,
                    )?;
                }
                _ => return Err("Pet attack resolved a companion-specific effect".into()),
            }
        }
        Ok(())
    }
}

fn pet_skill_element(server: &Server, source: &StatusSnapshot, metadata: &SkillMetadata, level: u8) -> Element {
    server.battle_service().skill_attack_element(source, metadata, level)
}
