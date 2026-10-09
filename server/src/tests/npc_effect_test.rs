use std::sync::Arc;

use models::enums::cell::CellType;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};

use super::ServerServiceTestContext;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{GameEvent, ScriptMapDamage, ScriptSkillCast};
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map::Map;
use crate::server::model::map_instance::{MapInstance, MapInstanceKey};
use crate::server::model::map_item::{MapItem, MapItemType, MapItems};
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::skill::ScriptSkillHit;
use crate::server::script::skill::actor::NpcSkillState;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::map_npc_effect::{MapNpcEffect, NpcEffect};
use crate::tests::common;

const SOURCE: u32 = 80_001;
const TARGET: u32 = 80_003;

fn fixture() -> (ServerServiceTestContext, Arc<MapInstance>, MapInstanceService) {
    let (context, instance, service, _) = super::actor_unit_skill_tests::fixture();
    let mut script = instance.get_script(SOURCE).unwrap().as_ref().clone();
    script.id = TARGET;
    script.name = "Recipient".into();
    script.x = 51;
    let mut target = NpcSkillState::new(&script);
    target.hp = 1000;
    target.max_hp = 1000;
    target.sp = 20;
    target.max_sp = 50;
    target.level = 70;
    target.parameters = [0; 6];
    target.recalculate_misc();
    {
        let mut state = instance.state_mut();
        let source = state.script_skill_state.npcs.get_mut(&SOURCE).unwrap();
        source.hp = 1000;
        source.max_hp = 1000;
        source.recalculate_misc();
        state.script_skill_state.npcs.insert(TARGET, target);
        state.insert_item(MapItem::new(TARGET, script.sprite as i16, MapItemType::Npc));
    }
    (context, instance, service)
}

fn apply(service: &MapInstanceService, instance: &MapInstance, target: u32, effect: NpcEffect, tick: u128) {
    service.apply_npc_effect(&mut instance.state_mut(), MapNpcEffect { actor_id: target, effect }, tick);
}

fn process_map(service: &MapInstanceService, instance: &MapInstance, tick: u128) {
    while let Some(events) = instance.task_queue().pop() {
        for event in events {
            if service.handle_npc_map_event(&mut instance.state_mut(), &event, tick) {
                continue;
            }
            match event {
                MapEvent::NpcEffect(request) => service.apply_npc_effect(&mut instance.state_mut(), request, tick),
                _ => panic!("Unexpected event {event:?}"),
            }
        }
    }
}

fn cast(context: &ServerServiceTestContext, instance: &MapInstance, service: &MapInstanceService, skill: SkillEnum, tick: u128) {
    super::actor_unit_skill_tests::start(
        context,
        instance,
        service,
        ScriptSkillCast {
            source_map: Some("empty.gat".into()),
            source_instance: Some(0),
            source_id: SOURCE,
            target_id: TARGET,
            skill_id: skill.id(),
            level: 1,
            ..Default::default()
        },
        tick,
    )
    .unwrap();
    assert_eq!(
        super::actor_unit_skill_tests::complete(context, instance, service, tick + 1000).unwrap(),
        1
    );
    process_map(service, instance, tick + 1000);
}

fn damage(target: u32, amount: u32, flags: u32) -> Damage {
    Damage { notification: None,
        source_kind: models::enums::actor::CombatActorKind::Player,
        skill_damage_adjusted: false,
        target_id: target,
        attacker_id: 80_002,
        credit_id: 80_002,
        damage: amount,
        healing: 0,
        attacked_at: 1000,
        damage_motion: 0,
        right_hand_damage: None,
        battle_flags: flags,
        skill_id: 0,
        skill_level: 0,
        proc_depth: 0,
        defenses_applied: true,
        magic_context: None,
        landed: true,
    }
}

fn status(kind: StatusChangeKind, duration: i32) -> StatusChangeRequest {
    let mut request = StatusChangeRequest::guaranteed(kind, duration, 1);
    request.flags |= StatusStartFlag::NoRateReduction.as_flag() | StatusStartFlag::NoDurationReduction.as_flag();
    request
}

#[test]
fn scripted_npc_support_heal_and_cure_reach_canonical_npc_recipients() {
    let (context, instance, service) = fixture();
    cast(&context, &instance, &service, SkillEnum::AlBlessing, 1000);
    assert!(
        instance.state().script_skill_state.npcs[&TARGET]
            .status()
            .has_status_change(StatusChangeKind::Blessing)
    );
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].snapshot().str(), 1);
    instance.state_mut().script_skill_state.npcs.get_mut(&TARGET).unwrap().hp = 100;
    cast(&context, &instance, &service, SkillEnum::AlHeal, 3000);
    assert!(instance.state().script_skill_state.npcs[&TARGET].hp > 100);
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Status(status(StatusChangeKind::NoRecovery, 10000)),
        5000,
    );
    let before = instance.state().script_skill_state.npcs[&TARGET].hp;
    cast(&context, &instance, &service, SkillEnum::AlHeal, 6000);
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].hp, before);
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::EndStatus(Some(StatusChangeKind::NoRecovery)),
        8000,
    );
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Status(status(StatusChangeKind::Poison, 10000)),
        8000,
    );
    cast(&context, &instance, &service, SkillEnum::TfDetoxify, 9000);
    assert!(
        !instance.state().script_skill_state.npcs[&TARGET]
            .status()
            .has_status_change(StatusChangeKind::Poison)
    );
}

#[test]
fn scripted_npc_damage_changes_real_npc_hp_and_emits_a_scoped_callback() {
    let (context, instance, service) = fixture();
    cast(&context, &instance, &service, SkillEnum::MgFirebolt, 1000);
    let hp = instance.state().script_skill_state.npcs[&TARGET].hp;
    assert!(hp > 0 && hp < 1000);
    let mut hits = vec![];
    while let Some(events) = context.server_task_queue.pop() {
        for event in events {
            if let GameEvent::ScriptSkillHit(hit) = event {
                hits.push(hit);
            }
        }
    }
    assert_eq!(hits.len(), 1);
    assert_eq!(
        (hits[0].source_id, hits[0].target_id, hits[0].source_instance),
        (SOURCE, TARGET, Some(0))
    );
    assert_eq!(hits[0].source_map.as_deref(), Some("empty.gat"));
    context
        .server
        .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptSkillHit(hits.remove(0)), 2000)
        .unwrap();
}

#[test]
fn npc_damage_immunity_and_shields_suppress_hp_changes_and_callbacks() {
    let (context, instance, service) = fixture();
    instance.state_mut().script_skill_state.npcs.get_mut(&TARGET).unwrap().damage_immune = true;
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Damage(damage(TARGET, 100, BattleFlag::Weapon.as_flag())),
        1000,
    );
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].hp, 1000);
    assert!(context.server_task_queue.pop().is_none());
    instance.state_mut().script_skill_state.npcs.get_mut(&TARGET).unwrap().damage_immune = false;
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Status(status(StatusChangeKind::Kyrie, 10000)),
        1000,
    );
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Damage(damage(TARGET, 100, BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag())),
        1100,
    );
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].hp, 1000);
    assert!(context.server_task_queue.pop().is_none());
}

#[test]
fn npc_damage_cancels_cancelable_casts_and_disabling_status_cancels_protected_casts() {
    let (context, instance, service) = fixture();
    let request = ScriptSkillCast {
        source_id: SOURCE,
        target_id: TARGET,
        skill_id: SkillEnum::MgFirebolt.id(),
        level: 1,
        cast_time_adjust_ms: 1000,
        ..Default::default()
    };
    super::actor_unit_skill_tests::start(&context, &instance, &service, request.clone(), 1000).unwrap();
    apply(
        &service,
        &instance,
        SOURCE,
        NpcEffect::Damage(damage(SOURCE, 1, BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag())),
        1100,
    );
    assert!(!instance.state().script_skill_state.casts.contains_key(&SOURCE));
    assert!(!instance.state().script_skill_state.generations.contains_key(&SOURCE));
    let mut protected = request;
    protected.cast_cancel = Some(false);
    super::actor_unit_skill_tests::start(&context, &instance, &service, protected, 1200).unwrap();
    apply(
        &service,
        &instance,
        SOURCE,
        NpcEffect::Damage(damage(SOURCE, 1, BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag())),
        1300,
    );
    assert!(instance.state().script_skill_state.casts.contains_key(&SOURCE));
    apply(
        &service,
        &instance,
        SOURCE,
        NpcEffect::Status(status(StatusChangeKind::Stun, 5000)),
        1400,
    );
    assert!(!instance.state().script_skill_state.casts.contains_key(&SOURCE));
}

#[test]
fn npc_status_periodic_damage_and_expiry_change_the_same_canonical_actor() {
    let (_, instance, service) = fixture();
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Status(status(StatusChangeKind::Poison, 3000)),
        1000,
    );
    service.tick_npc_statuses(&mut instance.state_mut(), 2000);
    assert!(instance.state().script_skill_state.npcs[&TARGET].hp < 1000);
    assert!(
        instance.state().script_skill_state.npcs[&TARGET]
            .status()
            .has_status_change(StatusChangeKind::Poison)
    );
    service.tick_npc_statuses(&mut instance.state_mut(), 4000);
    assert!(
        !instance.state().script_skill_state.npcs[&TARGET]
            .status()
            .has_status_change(StatusChangeKind::Poison)
    );
}

#[test]
fn npc_death_cancels_casts_and_does_not_allocate_monster_rewards_or_implicitly_revive() {
    let (context, instance, service) = fixture();
    let request = ScriptSkillCast {
        source_id: SOURCE,
        target_id: TARGET,
        skill_id: SkillEnum::MgFirebolt.id(),
        level: 1,
        cast_time_adjust_ms: 1000,
        ..Default::default()
    };
    super::actor_unit_skill_tests::start(&context, &instance, &service, request.clone(), 1000).unwrap();
    apply(&service, &instance, SOURCE, NpcEffect::Damage(damage(SOURCE, 5000, 0)), 1100);
    assert_eq!(instance.state().script_skill_state.npcs[&SOURCE].hp, 0);
    assert!(!instance.state().script_skill_state.casts.contains_key(&SOURCE));
    assert!(context.server_task_queue.pop().is_none());
    assert!(
        context
            .server
            .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptUnitSkill(request), 1200)
            .is_err()
    );
    assert_eq!(instance.state().script_skill_state.npcs[&SOURCE].max_hp, 1000);
    assert_eq!(instance.state().script_skill_state.npcs[&SOURCE].hp, 0);
}

#[test]
fn npc_movement_and_direction_are_visible_through_canonical_lookups() {
    let (context, instance, service) = fixture();
    apply(&service, &instance, TARGET, NpcEffect::Move { x: 60, y: 61 }, 1000);
    apply(&service, &instance, TARGET, NpcEffect::Direction(4), 1000);
    let snapshot = context.server.state().map_item_snapshot(TARGET, &"empty".into(), 0).unwrap();
    assert_eq!((snapshot.x(), snapshot.y(), snapshot.position().dir), (60, 61, 4));
    let script = instance.get_script(TARGET).unwrap();
    assert_eq!((script.x, script.y, script.dir), (60, 61, 4));
    apply(
        &service,
        &instance,
        TARGET,
        NpcEffect::Knockback {
            source_x: 59,
            source_y: 61,
            cells: 2,
        },
        1100,
    );
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].x, 62);
    apply(&service, &instance, TARGET, NpcEffect::Move { x: 100, y: 61 }, 1200);
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].x, 62);
}

#[test]
fn npc_magic_reflection_returns_damage_to_the_actual_npc_caster() {
    let (context, instance, service) = fixture();
    let mut mirror = status(StatusChangeKind::MagicMirror, 10000);
    mirror.values = [5, 100, 0, 0];
    apply(&service, &instance, TARGET, NpcEffect::Status(mirror), 1000);
    cast(&context, &instance, &service, SkillEnum::MgFirebolt, 2000);
    assert_eq!(instance.state().script_skill_state.npcs[&TARGET].hp, 1000);
    let mut reflections = 0;
    while let Some(events) = context.server_task_queue.pop() {
        for event in events {
            if let GameEvent::ReflectMagic(request) = event {
                context
                    .server
                    .handle_script_event(&mut context.server.state_mut(), GameEvent::ReflectMagic(request), 3000)
                    .unwrap();
                reflections += 1;
            }
        }
    }
    assert_eq!(reflections, 1);
    process_map(&service, &instance, 3000);
    assert!(instance.state().script_skill_state.npcs[&SOURCE].hp < 1000);
}

#[test]
fn callbacks_and_delayed_damage_cannot_cross_map_instances() {
    let (context, instance, service) = fixture();
    let scripts = instance
        .state()
        .script_skill_state
        .npcs
        .values()
        .map(|npc| npc.current_script("empty").as_ref().clone())
        .collect();
    let map = Box::leak(Box::new(Map::new(
        100,
        100,
        10000,
        "empty".into(),
        "empty.gat".into(),
        vec![],
        vec![],
        scripts,
    )));
    let clone = Arc::new(MapInstance::from_map(
        common::test_npc_vm(),
        map,
        1,
        vec![CellType::Walkable.as_flag() | CellType::Shootable.as_flag(); 10000],
        context.client_notification_sender.clone(),
        MapItems::new(1000),
        Arc::new(TasksQueue::new()),
    ));
    {
        let mut state = clone.state_mut();
        for npc in state.script_skill_state.npcs.values_mut() {
            npc.initialize_for_cast();
            npc.hp = 1000;
            npc.max_hp = 1000;
        }
    }
    context
        .server
        .state_mut()
        .map_instances_mut()
        .get_mut("empty")
        .unwrap()
        .push(clone.clone());
    let frozen = || {
        instance.state().script_skill_state.npcs[&TARGET]
            .status()
            .has_status_change(StatusChangeKind::Freeze)
    };
    for _ in 0..50 {
        let hit = ScriptSkillHit {
            source_map: Some("empty.gat".into()),
            source_instance: Some(0),
            source_id: SOURCE,
            target_id: TARGET,
            skill_id: SkillEnum::MgFrostdiver.id(),
            skill_level: 10,
            damage: 100,
            depth: 0,
        };
        context
            .server
            .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptSkillHit(hit), 1000)
            .unwrap();
        process_map(&service, &instance, 1000);
        if frozen() {
            break;
        }
    }
    assert!(frozen());
    assert!(clone.state().script_skill_state.npcs[&TARGET].active_statuses.is_empty());
    let character_id = 150_001;
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&character_id)
        .unwrap()
        .map_instance_key = MapInstanceKey::new("empty".into(), 1);
    let delayed = ScriptMapDamage {
        map: instance.key().clone(),
        damage: Damage {
            target_id: character_id,
            attacker_id: SOURCE,
            damage: 100,
            ..damage(character_id, 100, 0)
        },
    };
    context
        .server
        .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptMapDamage(delayed), 2000)
        .unwrap();
    assert_eq!(context.server.state().get_character(character_id).unwrap().status.hp, 1000);
}
