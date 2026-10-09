use models::enums::bonus::BonusType;
use models::enums::item::EquipmentLocation;
use models::enums::weapon::WeaponType;
use models::enums::skill::{SkillDamageFlags, SkillTargetType};
use models::enums::{EnumWithMaskValueU32, EnumWithMaskValueU64};
use models::status::{Status, StatusSnapshot};
use models::status_bonus::{ActiveAutoBonus, AutoBonus, BattleFlag, CombatTrigger, PeriodicBonusKind};
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use movement::position::Position;
use script_sdk::Value;

use crate::server::Server;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterZeny, GameEvent, CharacterDamage};
use crate::server::model::events::map_event::{MapEvent, MobDamage, ScriptDropItem, ScriptMobCombat};
use crate::server::model::status::StatusFromDb;
use crate::server::script::item_script_handler::ItemScriptHost;
use crate::server::service::combat_trigger_service::{CombatEffect, CombatEffectTarget, CombatEvent, resolve};
use crate::server::service::global_config_service::GlobalConfigService;
pub use crate::server::service::item_healing_service::scale_item_healing;
use crate::server::service::status_effect_service::StatusEffectService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;
use crate::server::state::server::ServerState;

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptCombatRequest {
    pub source_id: u32,
    pub target_id: u32,
    pub trigger: CombatTrigger,
    pub battle_flags: u32,
    pub skill_id: u32,
    pub damage: u32,
    pub right_hand_damage: Option<u32>,
    pub other_mob_id: Option<u32>,
    pub depth: u8,
    pub drop_position: Option<Position>,
    pub origin_map: Option<crate::server::model::map_instance::MapInstanceKey>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MobCombatEffect {
    Status(StatusChangeRequest),
    Vanish { hp: u32, sp: u32 },
    ClassChange { mob_id: u32 },
}

pub fn emit(server: &Server, source_id: u32, target_id: u32, trigger: CombatTrigger, battle_flags: u32, skill_id: u32, damage: u32) {
    server.add_to_next_tick(GameEvent::ScriptCombat(ScriptCombatRequest {
        source_id,
        target_id,
        trigger,
        battle_flags,
        skill_id,
        damage,
        right_hand_damage: None,
        other_mob_id: None,
        depth: 0,
        drop_position: None,
        origin_map: None,
    }));
}

pub fn emit_kill(server: &Server, source_id: u32, mob_id: u32, battle_flags: u32, skill_id: u32, mob_x: u16, mob_y: u16) {
    server.add_to_next_tick(GameEvent::ScriptCombat(ScriptCombatRequest {
        source_id,
        target_id: 0,
        trigger: CombatTrigger::Kill,
        battle_flags,
        skill_id,
        damage: 0,
        right_hand_damage: None,
        other_mob_id: Some(mob_id),
        depth: 0,
        drop_position: Some(Position {
            x: mob_x,
            y: mob_y,
            dir: 0,
        }),
        origin_map: None,
    }));
}

pub fn handle(server: &Server, state: &mut ServerState, request: ScriptCombatRequest, tick: u128) {
    if request.depth >= 8 {
        return;
    }
    let Some(character) = state.get_character(request.source_id) else {
        return;
    };
    if request.trigger != CombatTrigger::Kill && request.origin_map.as_ref().is_some_and(|key| key != &character.map_instance_key) {
        return;
    }
    let configuration = GlobalConfigService::instance();
    let owner = StatusService::instance().to_snapshot(&character.status);
    let other = if let Some(other) = state.get_character(request.target_id) {
        if other.map_instance_key != character.map_instance_key {
            return;
        }
        Some(StatusService::instance().to_snapshot(&other.status))
    } else {
        state
            .map_item(
                request.target_id,
                character.current_map_name(),
                character.current_map_instance(),
            )
            .and_then(|item| state.map_item_mob_status(&item, character.current_map_name(), character.current_map_instance()))
    }
    .or_else(|| {
        request
            .other_mob_id
            .and_then(|mob_id| configuration.get_mob_safe(mob_id as i32).map(StatusFromDb::from_mob_model))
    });
    let Some(other) = other else {
        return;
    };
    let mob = configuration.get_mob_safe(other.job() as i32);
    let event = CombatEvent {
        trigger: request.trigger,
        battle_flags: request.battle_flags,
        skill_id: request.skill_id,
        damage: request.damage,
        right_hand_damage: request.right_hand_damage,
        uses_ammo: crate::server::service::battle_service::BattleService::attack_uses_ammo(&owner, request.skill_id),
        other: &other,
        monster_class: *other.mob_class(),
        monster_level: mob.map_or(1, |mob| mob.level.max(1) as u32),
    };
    let mut rng = fastrand::Rng::new();
    let mut effects = resolve(owner.bonuses(), &event, &mut rng);
    let soul_drain = owner.known_skill_level(models::enums::skill_enums::SkillEnum::HwSouldrain);
    if request.trigger == CombatTrigger::Kill && soul_drain > 0 && request.battle_flags & models::status_bonus::BattleFlag::Magic.as_flag() != 0 {
        effects.push(CombatEffect::Heal { hp: 0, sp: (event.monster_level * (95 + 15 * u32::from(soul_drain)) / 100) as i32 });
    }
    for effect in effects {
        if let Err(error) = apply_effect(server, state, &request, &other, effect, tick, &mut rng) {
            error!("Unable to apply equipment combat effect for {}: {}", request.source_id, error);
        }
    }
}

fn apply_effect(
    server: &Server,
    state: &mut ServerState,
    request: &ScriptCombatRequest,
    other: &StatusSnapshot,
    effect: CombatEffect,
    tick: u128,
    rng: &mut fastrand::Rng,
) -> Result<(), String> {
    if matches!(effect, CombatEffect::DropItem { .. } | CombatEffect::DropGroup { .. }) {
        let key = request
            .origin_map
            .as_ref()
            .or_else(|| state.get_character(request.source_id).map(|owner| &owner.map_instance_key));
        if key.is_some_and(|key| state.map_flags(key).enabled(crate::server::model::map_flags::MapFlag::NoMobLoot)) {
            return Ok(());
        }
    }
    match effect {
        CombatEffect::Splash { radius } => apply_splash(server, state, request, radius, tick),
        CombatEffect::RunBonus(bonus) => {
            let mut character = state
                .characters_mut()
                .remove(&request.source_id)
                .ok_or("Automatic bonus owner is unavailable")?;
            let result = run_auto_bonus(server, state, &mut character, bonus, tick);
            state.insert_character(character);
            result
        }
        CombatEffect::AutoSpell { skill_id, level } => {
            let cost = {
                let owner = state.characters().get(&request.source_id).ok_or("Auto Spell owner is unavailable")?;
                let level = u8::try_from(level).map_err(|_| "Auto Spell level is out of range")?;
                server.script_skill_service().requirements_plan(owner, skill_id, level, tick).map_or(0, |plan| plan.sp) * 2 / 3
            };
            let owner = state.characters_mut().get_mut(&request.source_id).ok_or("Auto Spell owner is unavailable")?;
            if owner.status.sp < cost {
                return Ok(());
            }
            server.character_service().update_hp_sp(owner, owner.status.hp, owner.status.sp - cost);
            apply_effect(server, state, request, other, CombatEffect::CastSkill { skill_id, level, target: CombatEffectTarget::Other }, tick, rng)
        }
        CombatEffect::CastSkill { skill_id, level, target } => {
            let automatic_self = GlobalConfigService::instance()
                .find_skill_config(&Value::Number(skill_id as i32))
                .is_some_and(|skill| {
                    matches!(
                        skill.target_type(),
                        SkillTargetType::MySelf | SkillTargetType::Party | SkillTargetType::Friend
                    )
                });
            let target_id = if target == CombatEffectTarget::Owner || target == CombatEffectTarget::Automatic && automatic_self {
                request.source_id
            } else {
                request.target_id
            };
            server.script_skill_service().cast_equipment_proc(
                server,
                state,
                request.source_id,
                target_id,
                skill_id,
                level,
                tick,
                request.depth + 1,
                request.trigger,
            )?;
            let flags = skill_flags(skill_id);
            server.add_to_next_tick(GameEvent::ScriptCombat(ScriptCombatRequest {
                source_id: request.source_id,
                target_id,
                trigger: CombatTrigger::Skill,
                battle_flags: flags,
                skill_id,
                damage: 0,
                right_hand_damage: None,
                other_mob_id: None,
                depth: request.depth + 1,
                drop_position: None,
                origin_map: request.origin_map.clone(),
            }));
            Ok(())
        }
        CombatEffect::ApplyStatus {
            effect,
            rate,
            duration,
            target,
        } => {
            let kind = StatusChangeKind::from_ailment(effect).ok_or("Effect cannot be applied as a timed status")?;
            let target_id = if target == CombatEffectTarget::Owner {
                request.source_id
            } else {
                request.target_id
            };
            let duration = if duration == 0 { default_effect_duration(kind) } else { duration };
            apply_status(
                server,
                state,
                request.source_id,
                target_id,
                StatusChangeRequest {
                    kind,
                    duration_ms: duration.min(i32::MAX as u32) as i32,
                    values: [7, 0, 0, 0],
                    rate,
                    flags: 0,
                },
                tick,
            )
        }
        CombatEffect::Heal { hp, sp } => {
            let character = state
                .characters_mut()
                .get_mut(&request.source_id)
                .ok_or("Drain owner is unavailable")?;
            heal(server, character, hp as i64, sp as i64);
            Ok(())
        }
        CombatEffect::Vanish { hp, sp } => {
            if let Some(character) = state.characters_mut().get_mut(&request.target_id) {
                server.character_service().update_hp_sp(
                    character,
                    character.status.hp.saturating_sub(hp),
                    character.status.sp.saturating_sub(sp),
                );
            } else {
                map_effect(state, request.source_id, request.target_id, MobCombatEffect::Vanish { hp, sp })?;
            }
            Ok(())
        }
        CombatEffect::Reflect { damage, magic: _ } => {
            if damage == 0 {
                return Ok(());
            }
            if state.get_character(request.target_id).is_some() {
                server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage: Damage {
                    notification: None,
                    source_kind: models::enums::actor::CombatActorKind::Player,
                    skill_damage_adjusted: false,
                    target_id: request.target_id,
                    attacker_id: request.source_id,
                    damage,
                    healing: 0,
                    right_hand_damage: None,
                    attacked_at: tick,
                    damage_motion: 0,
                    battle_flags: 0,
                    skill_id: 0,
                    skill_level: 0,
                    landed: false,
                    proc_depth: request.depth + 1,
                    credit_id: request.source_id,
                    defenses_applied: true,
                    magic_context: None,
                } }));
            } else {
                let character = state.get_character(request.source_id).ok_or("Reflection owner is unavailable")?;
                state
                    .get_map_instance_from_character(character)
                    .ok_or("Reflection map is unavailable")?
                    .add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: Damage {
                        notification: None,
                        source_kind: models::enums::actor::CombatActorKind::Player,
                        skill_damage_adjusted: false,
                        target_id: request.target_id,
                        attacker_id: request.source_id,
                        damage,
                        healing: 0,
                        right_hand_damage: None,
                        attacked_at: tick,
                        damage_motion: 0,
                        battle_flags: 0,
                        skill_id: 0,
                        skill_level: 0,
                        landed: false,
                        proc_depth: request.depth + 1,
                        credit_id: request.source_id,
                        defenses_applied: true,
                        magic_context: None,
                    } }));
            }
            Ok(())
        }
        CombatEffect::DropItem { item_id } => drop_item(state, request, item_id, 1),
        CombatEffect::DropGroup { group_id } => {
            use crate::server::script::game_data::{group, stage_group_entry};
            let group_id = i32::try_from(group_id).map_err(|_| "Invalid drop group")?;
            let group = group(&Value::Number(group_id)).ok_or("Drop group does not exist")?;
            let pool = server.repository.item_group_pool(group_id, 1).map_err(|error| error.to_string())?;
            let (entry, receipt) = stage_group_entry(group, 1, &pool, rng).ok_or("Drop group has no eligible items")?;
            let item_id = u32::try_from(entry.item_id).map_err(|_| "Invalid drop group item")?;
            let (map, event) = prepare_drop_item(state, request, item_id, entry.amount)?;
            if let Some(receipt) = receipt {
                server
                    .repository
                    .commit_item_pool_draws(&[receipt])
                    .map_err(|error| error.to_string())?;
            }
            map.add_to_next_tick(event);
            Ok(())
        }
        CombatEffect::Zeny(amount) => {
            let character = state
                .characters_mut()
                .get_mut(&request.source_id)
                .ok_or("Zeny owner is unavailable")?;
            let zeny = server
                .repository
                .character_adjust_zeny(
                    character.char_id,
                    character.account_id,
                    character.status.zeny,
                    i64::from(amount),
                )
                .map_err(|error| error.to_string())?;
            character.status.zeny = zeny;
            server.add_to_next_tick(GameEvent::CharacterUpdateZeny(CharacterZeny {
                char_id: request.source_id,
                zeny: None,
            }));
            Ok(())
        }
        CombatEffect::NoRecovery { duration } => apply_status(
            server,
            state,
            request.source_id,
            request.target_id,
            StatusChangeRequest::guaranteed(StatusChangeKind::NoRecovery, duration.min(i32::MAX as u32) as i32, 1),
            tick,
        ),
        CombatEffect::SetDefense { value, duration, magic } => apply_status(
            server,
            state,
            request.source_id,
            request.target_id,
            StatusChangeRequest::guaranteed(
                if magic {
                    StatusChangeKind::MdefSet
                } else {
                    StatusChangeKind::DefSet
                },
                duration.min(i32::MAX as u32) as i32,
                value as i32,
            ),
            tick,
        ),
        CombatEffect::BreakEquipment { weapon } => break_equipment(server, state, request.target_id, BreakSlot::from_weapon_flag(weapon)),
        CombatEffect::ClassChange => {
            if state.get_character(request.target_id).is_some() {
                return Ok(());
            }
            let mob_id =
                crate::server::script::game_data::random_summon("CLASSCHANGE", rng).ok_or("Class-change summon group is unavailable")?;
            if other.hp() == 0 {
                return Ok(());
            }
            map_effect(state, request.source_id, request.target_id, MobCombatEffect::ClassChange {
                mob_id,
            })
        }
    }
}

fn apply_splash(server: &Server, state: &mut ServerState, request: &ScriptCombatRequest, radius: u16, tick: u128) -> Result<(), String> {
    let source = state.get_character(request.source_id).ok_or("Splash attacker is unavailable")?;
    let source_status = StatusService::instance().to_snapshot(&source.status);
    let instance = state.get_map_instance_from_character(source).ok_or("Splash map is unavailable")?;
    let map = instance.state();
    let center = state
        .get_character(request.target_id)
        .map(|target| Position {
            x: target.x,
            y: target.y,
            dir: target.dir,
        })
        .or_else(|| map.get_mob(request.target_id).map(|target| target.position()))
        .or(request.drop_position)
        .ok_or("Splash center is unavailable")?;
    let mut targets: Vec<_> = map
        .mobs()
        .values()
        .filter(|mob| {
            mob.id != request.target_id
                && mob.is_present()
                && mob.summon_owner != Some(request.source_id)
                && mob.summon_ai == 0
                && mob.x.abs_diff(center.x) <= radius
                && mob.y.abs_diff(center.y) <= radius
        })
        .map(|mob| (mob.id, mob.status.clone(), mob.position(), false))
        .collect();
    if state.get_character(request.target_id).is_some() {
        targets.extend(
            state
                .characters()
                .values()
                .filter(|target| {
                    target.char_id != request.source_id
                        && target.char_id != request.target_id
                        && target.status.hp > 0
                        && target.map_instance_key == source.map_instance_key
                        && !(source.game_systems.party_id != 0 && source.game_systems.party_id == target.game_systems.party_id)
                        && !(source.game_systems.guild_id != 0 && source.game_systems.guild_id == target.game_systems.guild_id)
                        && target.x.abs_diff(center.x) <= radius
                        && target.y.abs_diff(center.y) <= radius
                })
                .map(|target| {
                    (
                        target.char_id,
                        StatusService::instance().to_snapshot(&target.status),
                        Position {
                            x: target.x,
                            y: target.y,
                            dir: target.dir,
                        },
                        true,
                    )
                }),
        );
    }
    drop(map);
    let ranged = request.battle_flags & BattleFlag::Long.as_flag() != 0;
    let flags = BattleFlag::Weapon.as_flag()
        | BattleFlag::Skill.as_flag()
        | if ranged {
            BattleFlag::Long.as_flag()
        } else {
            BattleFlag::Short.as_flag()
        };
    for (target_id, target_status, position, player) in targets {
        let damage = server
            .battle_service()
            .normal_splash_damage_signed(&source_status, &target_status, ranged);
        let event = Damage {
            notification: None,
            source_kind: models::enums::actor::CombatActorKind::Player,
            skill_damage_adjusted: false,
            target_id,
            attacker_id: request.source_id,
            damage: damage.max(0) as u32,
            healing: if damage < 0 { damage.unsigned_abs() } else { 0 },
            right_hand_damage: None,
            attacked_at: tick,
            damage_motion: 0,
            battle_flags: flags,
            skill_id: 0,
            skill_level: 0,
            landed: true,
            proc_depth: request.depth + 1,
            credit_id: request.source_id,
            defenses_applied: true,
            magic_context: None,
        }
        .with_action_notification(
            source.current_map_name(),
            source.current_map_instance(),
            position.x,
            position.y,
            tick,
            1,
            0,
            models::enums::action::ActionType::Splash,
            (damage, 0),
        );
        if player {
            server.add_to_next_tick(GameEvent::CharacterDamage(CharacterDamage { damage: event }));
        } else {
            instance.add_to_next_tick(MapEvent::MobDamage(MobDamage { damage: event }));
        }
    }
    Ok(())
}

pub fn skill_flags(skill_id: u32) -> u32 {
    if let Some(metadata) = crate::server::script::skill::metadata::SkillMetadata::find(skill_id) {
        return metadata.battle_flags(metadata.range(1).unwrap_or(1) > 3 || metadata.damage_type.as_deref() == Some("Magic"));
    }
    let configuration = GlobalConfigService::instance();
    let Some(skill) = configuration.find_skill_config(&Value::Number(skill_id as i32)) else {
        return BattleFlag::Skill.as_flag();
    };
    let kind = if skill.damage_flags().unwrap_or(0) & SkillDamageFlags::IsMagical.as_flag() != 0
        || skill.dmg_matk().is_some()
        || skill.dmg_matk_per_level().is_some()
    {
        BattleFlag::Magic
    } else {
        BattleFlag::Weapon
    };
    let range = if skill.range().unwrap_or(1) > 3 {
        BattleFlag::Long
    } else {
        BattleFlag::Short
    };
    kind.as_flag() | range.as_flag() | BattleFlag::Skill.as_flag()
}

fn run_auto_bonus(server: &Server, state: &mut ServerState, character: &mut Character, definition: AutoBonus, tick: u128) -> Result<(), String> {
    if !auto_bonus_source_is_present(character, &definition) {
        return Ok(());
    }
    let pet_bonus = definition.source_pet_id != 0;
    let run_program = |host: ItemScriptHost, program_id: u32| {
        let vm = &server.item_service().item_script_vm;
        if pet_bonus {
            futures::executor::block_on(vm.run_pet_program(host, program_id))
        } else {
            futures::executor::block_on(vm.run_program(host, program_id))
        }
    };
    let host = server
        .item_service()
        .prepare_host(server, character, definition.source_item_id, !pet_bonus);
    let (host, result) = run_program(host, definition.program_id);
    result.map_err(|error| host.error.clone().unwrap_or(error))?;
    let bonuses = host.bonuses.drain();
    let mut effects = host.effects;
    if definition.visual_program_id != 0 {
        let mut visual = server
            .item_service()
            .prepare_host(server, character, definition.source_item_id, true);
        visual.variables.extend(host.variables);
        let (visual, result) = run_program(visual, definition.visual_program_id);
        result.map_err(|error| visual.error.clone().unwrap_or(error))?;
        effects.extend(visual.effects);
    }
    server.item_service().validate_effects(&effects)?;
    server.item_service().apply_effects(server, state, server.runtime(), character, effects)?;
    if !auto_bonus_source_is_present(character, &definition) {
        return Ok(());
    }
    let active = ActiveAutoBonus {
        definition,
        expires_at: tick.saturating_add(definition.duration as u128),
        bonuses,
    };
    if let Some(existing) = character.status.active_auto_bonuses.iter_mut().find(|bonus| {
        bonus.definition.program_id == definition.program_id
            && bonus.definition.trigger == definition.trigger
            && bonus.definition.source_item_id == definition.source_item_id
            && bonus.definition.source_location == definition.source_location
            && bonus.definition.source_pet_id == definition.source_pet_id
    }) {
        *existing = active;
    } else {
        character.status.active_auto_bonuses.push(active);
    }
    reload_auto_bonus_status(server, character);
    Ok(())
}

fn reload_auto_bonus_status(server: &Server, character: &mut Character) {
    let snapshot = StatusService::instance().to_snapshot(&character.status);
    let hp = character.status.hp.min(snapshot.max_hp());
    let sp = character.status.sp.min(snapshot.max_sp());
    if hp != character.status.hp || sp != character.status.sp {
        server.character_service().update_hp_sp(character, hp, sp);
    }
    server.character_service().reload_client_side_status(character);
}

fn auto_bonus_source_is_present(character: &Character, definition: &AutoBonus) -> bool {
    if definition.source_pet_id != 0 {
        character
            .game_systems
            .pet
            .as_ref()
            .is_some_and(|pet| pet.id == definition.source_pet_id && !pet.incubating && pet.intimacy > 0)
    } else {
        source_is_equipped_at(&character.status, definition.source_item_id, definition.source_location)
    }
}

fn apply_status(
    server: &Server,
    state: &mut ServerState,
    source_id: u32,
    target_id: u32,
    request: StatusChangeRequest,
    tick: u128,
) -> Result<(), String> {
    if let Some(character) = state.characters_mut().get_mut(&target_id) {
        StatusEffectService::start(server, character, request, tick, &server.server_service().notification_sender())?;
        Ok(())
    } else {
        map_effect(state, source_id, target_id, MobCombatEffect::Status(request))
    }
}

fn map_effect(state: &ServerState, source_id: u32, target_id: u32, effect: MobCombatEffect) -> Result<(), String> {
    let source = state.get_character(source_id).ok_or("Combat source is unavailable")?;
    state
        .get_map_instance_from_character(source)
        .ok_or("Combat map is unavailable")?
        .add_to_next_tick(MapEvent::ScriptMobCombat(ScriptMobCombat {
            source_id,
            target_id,
            effect,
        }));
    Ok(())
}

fn drop_item(state: &ServerState, request: &ScriptCombatRequest, item_id: u32, amount: i16) -> Result<(), String> {
    let (map, event) = prepare_drop_item(state, request, item_id, amount)?;
    map.add_to_next_tick(event);
    Ok(())
}

fn prepare_drop_item(
    state: &ServerState,
    request: &ScriptCombatRequest,
    item_id: u32,
    amount: i16,
) -> Result<(std::sync::Arc<crate::server::model::map_instance::MapInstance>, MapEvent), String> {
    GlobalConfigService::instance()
        .find_item(item_id as i32)
        .ok_or("Extra drop item does not exist")?;
    if amount <= 0 {
        return Err("Extra drop amount must be positive".into());
    }
    let character = state.get_character(request.source_id).ok_or("Drop owner is unavailable")?;
    let position = request.drop_position.unwrap_or(Position {
        x: character.x,
        y: character.y,
        dir: 0,
    });
    let map = if let Some(key) = &request.origin_map {
        state.get_map_instance(key.map_name(), key.map_instance())
    } else {
        state.get_map_instance_from_character(character)
    }
    .ok_or("Drop map is unavailable")?;
    let location = map.state();
    if state
        .map_flags(location.key())
        .enabled(crate::server::model::map_flags::MapFlag::NoMobLoot)
    {
        return Err("Extra monster drops are disabled on this map".into());
    }
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    if position.x >= location.x_size()
        || position.y >= location.y_size()
        || !location.cells().iter().any(|cell| cell & CellType::Walkable.as_flag() != 0)
    {
        return Err("Extra drop has no valid map location".into());
    }
    drop(location);
    Ok((map, MapEvent::ScriptDropItem(ScriptDropItem {
        owner_id: request.source_id,
        item_id: item_id as i32,
        amount,
        x: position.x,
        y: position.y,
    })))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BreakSlot {
    Weapon,
    Armor,
    Shield,
    Helm,
}

impl BreakSlot {
    pub(crate) fn from_weapon_flag(weapon: bool) -> Self {
        if weapon {
            Self::Weapon
        } else {
            Self::Armor
        }
    }

    fn location(self) -> EquipmentLocation {
        match self {
            Self::Weapon => EquipmentLocation::HandRight,
            Self::Armor => EquipmentLocation::Armor,
            Self::Shield => EquipmentLocation::HandLeft,
            Self::Helm => EquipmentLocation::HeadTop,
        }
    }

    fn unbreakable(self) -> BonusType {
        match self {
            Self::Weapon => BonusType::UnbreakableWeapon,
            Self::Armor => BonusType::UnbreakableArmor,
            Self::Shield => BonusType::UnbreakableShield,
            Self::Helm => BonusType::UnbreakableHelm,
        }
    }

    fn protection(self) -> StatusChangeKind {
        match self {
            Self::Weapon => StatusChangeKind::ProtectWeapon,
            Self::Armor => StatusChangeKind::ProtectArmor,
            Self::Shield => StatusChangeKind::ProtectShield,
            Self::Helm => StatusChangeKind::ProtectHelm,
        }
    }
}

fn weapon_type_breakable(weapon: &WeaponType) -> bool {
    !matches!(
        weapon,
        WeaponType::Fist
            | WeaponType::Axe1H
            | WeaponType::Axe2H
            | WeaponType::Mace
            | WeaponType::Mace2H
            | WeaponType::Staff
            | WeaponType::Staff2H
            | WeaponType::Book
            | WeaponType::Huuma
            | WeaponType::DoubleAa
            | WeaponType::DoubleDa
            | WeaponType::DoubleSa
    )
}

pub(crate) fn break_equipment(server: &Server, state: &mut ServerState, target_id: u32, slot: BreakSlot) -> Result<(), String> {
    let Some(character) = state.characters_mut().get_mut(&target_id) else {
        return Ok(());
    };
    if character.status.has_status_change(slot.protection()) {
        return Ok(());
    }
    let snapshot = StatusService::instance().to_snapshot(&character.status);
    let unbreakable = slot.unbreakable();
    if snapshot.bonuses().iter().any(|bonus| *bonus.bonus() == unbreakable) {
        return Ok(());
    }
    if slot == BreakSlot::Weapon && !weapon_type_breakable(snapshot.right_hand_weapon_type()) {
        return Ok(());
    }
    let location = slot.location().as_flag();
    let Some(index) = character.inventory.iter().enumerate().find_map(|(index, item)| {
        item.as_ref()
            .filter(|item| item.equip as u64 & location != 0 && !item.is_damaged)
            .map(|_| index)
    }) else {
        return Ok(());
    };
    let mut item = character.inventory[index].as_ref().unwrap().clone();
    item.is_damaged = true;
    server
        .runtime()
        .block_on(server.repository.character_set_item_damaged(target_id, item.clone()))
        .map_err(|error| error.to_string())?;
    character.inventory[index] = Some(item);
    server.inventory_service().takeoff_equip_item(character, index);
    Ok(())
}

fn heal(server: &Server, character: &mut Character, hp: i64, sp: i64) {
    if character.status.hp == 0 {
        return;
    }
    let hp = if character.status.has_status_change(StatusChangeKind::NoRecovery) {
        hp.min(0)
    } else {
        hp
    };
    let sp = if character.status.has_status_change(StatusChangeKind::NoRecovery) {
        sp.min(0)
    } else {
        sp
    };
    let snapshot = StatusService::instance().to_snapshot(&character.status);
    let new_hp = (character.status.hp as i64 + hp).clamp(0, snapshot.max_hp() as i64) as u32;
    let new_sp = (character.status.sp as i64 + sp).clamp(0, snapshot.max_sp() as i64) as u32;
    server.character_service().update_hp_sp(character, new_hp, new_sp);
}

pub fn source_is_equipped(status: &Status, item_id: u32) -> bool {
    source_is_equipped_at(status, item_id, 0)
}

pub fn source_is_equipped_at(status: &Status, item_id: u32, location: u64) -> bool {
    if let Some(set) = crate::server::model::item_combos::set_of_script(item_id) {
        let worn = crate::server::model::item_combos::worn_items(status);
        let configuration = crate::server::service::global_config_service::GlobalConfigService::instance();
        return set.active_count(&worn, &|id| configuration.find_item(id).is_some_and(|item| item.item_type == models::enums::item::ItemType::Card)) > 0;
    }
    let id = item_id as i32;
    status.weapons.iter().any(|item| {
        (location == 0 || item.location == location)
            && (item.item_id == id
                || !models::item::special_card_metadata(item.card0)
                    && [item.card0, item.card1, item.card2, item.card3]
                        .into_iter()
                        .any(|card| card as i32 == id))
    }) || status.equipments.iter().any(|item| {
        (location == 0 || item.location == location)
            && (item.item_id == id
                || !models::item::special_card_metadata(item.card0)
                    && [item.card0, item.card1, item.card2, item.card3]
                        .into_iter()
                        .any(|card| card as i32 == id))
    }) || status
        .ammo
        .as_ref()
        .is_some_and(|item| (location == 0 || location == EquipmentLocation::Ammo.as_flag()) && item.item_id == id)
}

pub fn tick_character(server: &Server, character: &mut Character, tick: u128) {
    let before = character.status.active_auto_bonuses.len();
    let equipped_ids = character
        .status
        .active_auto_bonuses
        .iter()
        .filter(|bonus| auto_bonus_source_is_present(character, &bonus.definition))
        .map(|bonus| {
            (
                bonus.definition.source_item_id,
                bonus.definition.source_location,
                bonus.definition.source_pet_id,
            )
        })
        .collect::<std::collections::HashSet<_>>();
    character.status.active_auto_bonuses.retain(|bonus| {
        bonus.expires_at > tick
            && equipped_ids.contains(&(
                bonus.definition.source_item_id,
                bonus.definition.source_location,
                bonus.definition.source_pet_id,
            ))
    });
    if before != character.status.active_auto_bonuses.len() {
        reload_auto_bonus_status(server, character);
    }
    let snapshot = StatusService::instance().to_snapshot(&character.status);
    let mut present = std::collections::HashSet::new();
    let mut hp = 0_i64;
    let mut sp = 0_i64;
    for bonus in snapshot.bonuses() {
        let (kind, amount, period) = match *bonus.bonus() {
            BonusType::HpLossEveryMs(amount, period) => (PeriodicBonusKind::HpLoss, amount, period),
            BonusType::HpRegenEveryMs(amount, period) => (PeriodicBonusKind::HpRegen, amount, period),
            BonusType::SpLossEveryMs(amount, period) => (PeriodicBonusKind::SpLoss, amount, period),
            BonusType::SpRegenEveryMs(amount, period) => (PeriodicBonusKind::SpRegen, amount, period),
            _ => continue,
        };
        if period == 0 {
            continue;
        }
        let key = (kind, amount, period);
        present.insert(key);
        let previous = character.status.bonus_periodic_ticks.entry(key).or_insert(tick);
        let count = tick.saturating_sub(*previous) / period as u128;
        if count == 0 {
            continue;
        }
        *previous += count * period as u128;
        let delta = (amount as i64).saturating_mul(count.min(i64::MAX as u128) as i64);
        match kind {
            PeriodicBonusKind::HpLoss => hp -= delta,
            PeriodicBonusKind::HpRegen => hp += delta,
            PeriodicBonusKind::SpLoss => sp -= delta,
            PeriodicBonusKind::SpRegen => sp += delta,
        }
    }
    character.status.bonus_periodic_ticks.retain(|key, _| present.contains(key));
    if hp != 0 || sp != 0 {
        heal(server, character, hp.max(1_i64 - character.status.hp as i64), sp);
    }
}

fn default_effect_duration(kind: StatusChangeKind) -> u32 {
    match kind {
        StatusChangeKind::Stun | StatusChangeKind::Freeze | StatusChangeKind::Sleep => 3000,
        StatusChangeKind::Poison | StatusChangeKind::Bleeding => 60000,
        StatusChangeKind::Coma => 0,
        _ => 30000,
    }
}

#[cfg(test)]
mod tests {
    use models::item::WearGear;

    use super::*;

    fn gear(location: u64, card: i16) -> WearGear {
        WearGear {
            item_id: 2301,
            level: 1,
            location,
            refine: 0,
            card0: 0,
            card1: card,
            card2: 0,
            card3: 0,
            def: 1,
            inventory_index: 0,
        }
    }

    #[test]
    fn automatic_bonus_source_tracks_the_card_owner_slot() {
        let mut status = Status::default();
        status.equipments = vec![
            gear(EquipmentLocation::Armor.as_flag(), 4412),
            gear(EquipmentLocation::Garment.as_flag(), 4412),
        ];
        assert!(source_is_equipped_at(&status, 4412, EquipmentLocation::Armor.as_flag()));
        assert!(source_is_equipped_at(&status, 4412, EquipmentLocation::Garment.as_flag()));
        status.equipments.remove(0);
        assert!(!source_is_equipped_at(&status, 4412, EquipmentLocation::Armor.as_flag()));
        assert!(source_is_equipped_at(&status, 4412, EquipmentLocation::Garment.as_flag()));
    }

    #[test]
    fn creator_metadata_is_not_an_equipped_card_source() {
        let mut status = Status::default();
        let mut forged = gear(EquipmentLocation::Armor.as_flag(), 4412);
        forged.card0 = 255;
        status.equipments.push(forged);
        assert!(!source_is_equipped(&status, 4412));
        assert!(source_is_equipped(&status, 2301));
    }

    #[test]
    fn queued_kill_drops_keep_the_original_map_after_the_owner_warps() {
        crate::tests::common::before_all();
        use models::enums::EnumWithMaskValueU16;
        use models::enums::cell::CellType;

        use crate::server::model::map_instance::{MapInstance, MapInstanceKey};
        use crate::server::model::map_item::MapItems;
        use crate::server::model::tasks_queue::TasksQueue;
        let (sender, _) = std::sync::mpsc::sync_channel(64);
        let old_tasks = std::sync::Arc::new(TasksQueue::new());
        let new_tasks = std::sync::Arc::new(TasksQueue::new());
        let old = std::sync::Arc::new(crate::tests::common::map_instance_helper::create_empty_map_instance(
            sender.clone(),
            old_tasks.clone(),
        ));
        let new = std::sync::Arc::new(MapInstance::from_map(
            crate::tests::common::test_npc_vm(),
            crate::tests::common::map_instance_helper::create_empty_map(),
            1,
            vec![CellType::Walkable.as_flag(); 10000],
            sender,
            MapItems::default(),
            new_tasks.clone(),
        ));
        let mut state = ServerState::new(MapItems::default());
        state.map_instances_mut().insert("empty".into(), vec![old, new]);
        let mut owner = crate::tests::common::character_helper::create_character();
        owner.map_instance_key = MapInstanceKey::new("empty".into(), 1);
        owner.x = 30;
        owner.y = 40;
        state.characters_mut().insert(owner.char_id, owner);
        let request = ScriptCombatRequest {
            source_id: 150000,
            target_id: 0,
            trigger: CombatTrigger::Kill,
            battle_flags: 0,
            skill_id: 0,
            damage: 0,
            right_hand_damage: None,
            other_mob_id: Some(1002),
            depth: 0,
            drop_position: Some(Position { x: 2, y: 3, dir: 0 }),
            origin_map: Some(MapInstanceKey::new("empty".into(), 0)),
        };
        drop_item(&state, &request, 501, 1).unwrap();
        let events = old_tasks.pop().unwrap();
        assert!(matches!(events.as_slice(), [MapEvent::ScriptDropItem(ScriptDropItem {
            owner_id: 150000,
            item_id: 501,
            amount: 1,
            x: 2,
            y: 3
        })]));
        assert!(new_tasks.pop().unwrap_or_default().is_empty());
        assert!(prepare_drop_item(&state, &request, 501, 0).is_err());
        let mut flags = crate::server::model::map_flags::MapFlags::default();
        flags.set(crate::server::model::map_flags::MapFlag::NoMobLoot, true, &[]).unwrap();
        state.map_flag_overrides().insert(("empty".into(), 0), flags);
        assert!(prepare_drop_item(&state, &request, 501, 1).is_err());
        assert!(old_tasks.pop().unwrap_or_default().is_empty());
    }
}
