use std::sync::Arc;

use models::enums::cell::CellType;
use models::enums::mob::MobMode;
use models::enums::skill_enums::SkillEnum;
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
use models::status_change::{StatusChangeKind, StatusChangeRequest, StatusStartFlag};

use super::ServerServiceTestContext;
use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{GameEvent, ScriptSkillCast, CharacterDamage};
use crate::server::model::events::map_event::{MapEvent, MobStatusChange};
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::model::map_item::{MapItems, ToMapItem, ToMapItemSnapshot};
use crate::server::model::script::Script;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::skill::actor::{self, NpcSkillState};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::tests::common;

pub(super) const NPC_ID: u32 = 80_001;
const MOB_ID: u32 = 80_002;

pub(super) fn fixture() -> (ServerServiceTestContext, Arc<MapInstance>, MapInstanceService, u32) {
    let (context, _, owner) = super::native_payment_tests::fixture(false, true);
    let target_id = owner.char_id + 1;
    let script = Script {
        id: NPC_ID,
        scope_instance: 0,
        map_name: "empty".into(),
        name: "Caster".into(),
        sprite: 46,
        x: 50,
        y: 50,
        dir: 0,
        x_size: 0,
        y_size: 0,
        entry_id: 0,
        constructor_args: vec![],
    };
    let npc = NpcSkillState {
        level: 70,
        parameters: [0, 0, 0, 80, 0, 0],
        ..NpcSkillState::new(&script)
    };
    let map = Box::leak(Box::new(Map::new(
        100,
        100,
        10000,
        "empty".into(),
        "empty.gat".into(),
        vec![],
        vec![],
        vec![script],
    )));
    let instance = Arc::new(MapInstance::from_map(
        common::test_script_vm(),
        map,
        0,
        vec![CellType::Walkable.as_flag() | CellType::Shootable.as_flag(); 10000],
        context.client_notification_sender.clone(),
        MapItems::new(1000),
        Arc::new(TasksQueue::new()),
    ));
    context.server.state_mut().insert_character(owner);
    context
        .server
        .state_mut()
        .map_instances_mut()
        .insert("empty".into(), vec![instance.clone()]);
    let players = context
        .server
        .state()
        .characters()
        .values()
        .map(|character| character.to_map_item_snapshot())
        .collect::<Vec<_>>();
    {
        let mut map_state = instance.state_mut();
        for player in &players {
            map_state.insert_item(player.map_item());
        }
        map_state.update_characters(players);
        map_state.script_skill_state.npcs.insert(NPC_ID, npc);
        let mut mob = crate::tests::common::mob_helper::create_mob(MOB_ID, "PORING");
        mob.x = 50;
        mob.y = 50;
        mob.atk1 = 100;
        mob.atk2 = 100;
        mob.atk_motion = 0;
        mob.status.set_hp(1000);
        mob.status_effects.hp = 1000;
        mob.status.set_matk_min(80);
        mob.status.set_matk_max(80);
        mob.status.set_base_dex(0);
        mob.status.set_base_level(70);
        mob.status_effects.base_level = 70;
        mob.mode = MobMode::CanMove.as_flag() | MobMode::CanAttack.as_flag();
        map_state.insert_mob(mob);
    }
    let service = MapInstanceService::new(
        context.client_notification_sender.clone(),
        GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()),
        context.server_task_queue.clone(),
    );
    (context, instance, service, target_id)
}

fn request(source_id: u32, target_id: u32, skill: SkillEnum) -> ScriptSkillCast {
    ScriptSkillCast {
        source_id,
        target_id,
        skill_id: skill.id(),
        level: 1,
        ..Default::default()
    }
}

pub(super) fn start(
    context: &ServerServiceTestContext,
    instance: &MapInstance,
    service: &MapInstanceService,
    request: ScriptSkillCast,
    tick: u128,
) -> Result<(), String> {
    context
        .server
        .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptUnitSkill(request), tick)?;
    while let Some(events) = instance.task_queue().pop() {
        for event in events {
            if let MapEvent::ActorSkillCast(cast) = event {
                return service.start_actor_skill(&mut instance.state_mut(), cast, tick);
            }
        }
    }
    Err("No actor cast was queued".into())
}

pub(super) fn complete(context: &ServerServiceTestContext, instance: &MapInstance, service: &MapInstanceService, tick: u128) -> Result<usize, String> {
    service.tick_actor_skills(&mut instance.state_mut(), tick);
    let mut count = 0;
    let mut retained = vec![];
    while let Some(events) = context.server_task_queue.pop() {
        for event in events {
            if let GameEvent::ScriptActorSkillComplete(completion) = event {
                context.server.handle_script_event(
                    &mut context.server.state_mut(),
                    GameEvent::ScriptActorSkillComplete(completion),
                    tick,
                )?;
                count += 1;
            } else {
                retained.push(event);
            }
        }
    }
    for event in retained {
        context.server_task_queue.add_to_index(event, 0);
    }
    Ok(count)
}

fn damage_event(context: &ServerServiceTestContext) -> Damage {
    while let Some(events) = context.server_task_queue.pop() {
        for event in events {
            if let GameEvent::CharacterDamage(CharacterDamage { damage }) = event {
                return damage;
            }
            if let GameEvent::ScriptMapDamage(request) = event {
                return request.damage;
            }
        }
    }
    panic!("Actor did not queue damage");
}

fn incoming_damage(target_id: u32, attacker_id: u32, tick: u128) -> Damage {
    Damage { notification: None,
        source_kind: models::enums::actor::CombatActorKind::Player,
        skill_damage_adjusted: false,
        target_id,
        attacker_id,
        damage: 1,
        healing: 0,
        right_hand_damage: None,
        attacked_at: tick,
        damage_motion: 0,
        battle_flags: models::status_bonus::BattleFlag::Weapon.as_flag() | models::status_bonus::BattleFlag::Short.as_flag(),
        skill_id: 0,
        skill_level: 0,
        proc_depth: 0,
        credit_id: attacker_id,
        defenses_applied: true,
        magic_context: None,
        landed: true,
    }
}

#[test]
fn npc_offensive_cast_uses_canonical_stats_and_actual_actor_at_completion() {
    let (context, instance, service, target) = fixture();
    let mut cast = request(NPC_ID, target, SkillEnum::MgFirebolt);
    cast.cast_time_adjust_ms = 500;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(instance.state().script_skill_state.casts[&NPC_ID].finish_at, 2200);
    assert_eq!(complete(&context, &instance, &service, 2199).unwrap(), 0);
    assert_eq!(complete(&context, &instance, &service, 2200).unwrap(), 1);
    let damage = damage_event(&context);
    assert_eq!(
        (damage.attacker_id, damage.credit_id, damage.target_id),
        (NPC_ID, NPC_ID, target)
    );
    assert!(damage.damage > 0 && damage.magic_context.is_some());
    context
        .server
        .admit_character_damage(&mut context.server.state_mut(), damage, 2200)
        .unwrap();
    assert!(context.server.state().get_character(target).unwrap().status.hp < 1000);
    assert_eq!(complete(&context, &instance, &service, 2201).unwrap(), 0);
}

#[test]
fn npc_support_and_heal_apply_to_the_real_player_without_a_player_caster() {
    let (context, instance, service, target) = fixture();
    start(
        &context,
        &instance,
        &service,
        request(NPC_ID, target, SkillEnum::AlBlessing),
        1000,
    )
    .unwrap();
    complete(&context, &instance, &service, 1000).unwrap();
    assert!(
        context
            .server
            .state()
            .get_character(target)
            .unwrap()
            .status
            .has_status_change(StatusChangeKind::Blessing)
    );
    context.server.state_mut().characters_mut().get_mut(&target).unwrap().status.hp = 1;
    start(&context, &instance, &service, request(NPC_ID, target, SkillEnum::AlHeal), 2000).unwrap();
    complete(&context, &instance, &service, 2000).unwrap();
    assert!(context.server.state().get_character(target).unwrap().status.hp > 1);
    assert_eq!(
        (
            instance.state().script_skill_state.npcs[&NPC_ID].hp,
            instance.state().script_skill_state.npcs[&NPC_ID].sp
        ),
        (1, 1)
    );
}

#[test]
fn monster_self_buff_and_summon_skill_keep_actor_and_loot_owner_separate() {
    let (context, instance, service, target) = fixture();
    let owner = target - 1;
    instance.state_mut().mobs_mut().get_mut(&MOB_ID).unwrap().summon_owner = Some(owner);
    start(
        &context,
        &instance,
        &service,
        request(MOB_ID, MOB_ID, SkillEnum::SmEndure),
        1000,
    )
    .unwrap();
    complete(&context, &instance, &service, 1000).unwrap();
    while let Some(events) = instance.task_queue().pop() {
        for event in events {
            if let MapEvent::MobStatusChange(MobStatusChange { mob_id, request }) = event {
                service.start_mob_status(&mut instance.state_mut(), mob_id, request, 1000);
            }
        }
    }
    assert!(
        instance
            .state()
            .get_mob(MOB_ID)
            .unwrap()
            .status
            .has_status_change(StatusChangeKind::Endure)
    );
    let mut cast = request(MOB_ID, target, SkillEnum::MgFirebolt);
    cast.cast_time_adjust_ms = -5000;
    start(&context, &instance, &service, cast, 2000).unwrap();
    complete(&context, &instance, &service, 2000).unwrap();
    let damage = damage_event(&context);
    assert_eq!((damage.attacker_id, damage.credit_id), (MOB_ID, owner));
    assert!(damage.damage > 0);
}

#[test]
fn admitted_damage_cancels_only_cancelable_monster_casts() {
    for cancelable in [true, false] {
        let (context, instance, service, target) = fixture();
        let mut cast = request(MOB_ID, target, SkillEnum::MgFirebolt);
        cast.cast_cancel = Some(cancelable);
        start(&context, &instance, &service, cast, 1000).unwrap();
        service.mob_being_attacked(
            &mut instance.state_mut(),
            incoming_damage(MOB_ID, target, 1100),
            instance.task_queue(),
            1100,
        );
        assert_eq!(instance.state().script_skill_state.casts.contains_key(&MOB_ID), !cancelable);
        assert_eq!(
            actor::tick_map_casts(&mut instance.state_mut(), 2500).len(),
            usize::from(!cancelable)
        );
    }
}

#[test]
fn silence_death_movement_and_target_departure_discard_pending_actor_casts() {
    for cause in 0..4 {
        let (context, instance, service, target) = fixture();
        start(
            &context,
            &instance,
            &service,
            request(MOB_ID, target, SkillEnum::MgFirebolt),
            1000,
        )
        .unwrap();
        match cause {
            0 => {
                let mut status = StatusChangeRequest::guaranteed(StatusChangeKind::Silence, 5000, 1);
                status.flags |= StatusStartFlag::NoAvoid.as_flag();
                service.start_mob_status(&mut instance.state_mut(), MOB_ID, status, 1100);
            }
            1 => instance.state_mut().mobs_mut().get_mut(&MOB_ID).unwrap().status.set_hp(0),
            2 => instance.state_mut().mobs_mut().get_mut(&MOB_ID).unwrap().x += 1,
            _ => {
                let mut map = instance.state_mut();
                map.remove_item_with_id(target);
                map.update_characters(vec![]);
            }
        }
        assert!(actor::tick_map_casts(&mut instance.state_mut(), 2500).is_empty());
        assert!(!instance.state().script_skill_state.casts.contains_key(&MOB_ID));
    }
}

#[test]
fn range_and_obstacles_are_checked_unless_ignored_and_ground_still_requires_usable_terrain() {
    let (context, instance, service, target) = fixture();
    context.server.state_mut().characters_mut().get_mut(&target).unwrap().x = 70;
    instance.state_mut().update_characters(vec![
        context.server.state().get_character(target).unwrap().to_map_item_snapshot(),
    ]);
    let mut cast = request(MOB_ID, target, SkillEnum::MgFirebolt);
    assert!(start(&context, &instance, &service, cast.clone(), 1000).is_err());
    cast.ignore_range = true;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert!(actor::interrupt_map_cast(
        &mut instance.state_mut(),
        MOB_ID,
        1100,
        &context.client_notification_sender
    ));
    context.server.state_mut().characters_mut().get_mut(&target).unwrap().x = 53;
    instance.state_mut().update_characters(vec![
        context.server.state().get_character(target).unwrap().to_map_item_snapshot(),
    ]);
    instance.state_mut().cells_mut()[50 * 100 + 52] = 0;
    assert!(
        start(
            &context,
            &instance,
            &service,
            request(MOB_ID, target, SkillEnum::MgFirebolt),
            2000
        )
        .is_err()
    );
    let mut ground = request(MOB_ID, 0, SkillEnum::WzHeavendrive);
    ground.ground = Some((52, 50));
    ground.ignore_range = true;
    assert!(start(&context, &instance, &service, ground, 2000).is_err());
}

#[test]
fn nonmonster_messages_fail_before_casting() {
    let (context, instance, service, target) = fixture();
    let mut cast = request(NPC_ID, target, SkillEnum::MgFirebolt);
    cast.message_id = Some(1);
    assert!(start(&context, &instance, &service, cast, 1000).is_err());
    assert!(instance.state().script_skill_state.casts.is_empty());
}

#[test]
fn npc_provoke_uses_nonplayer_values_and_interrupts_the_real_target_cast() {
    let (context, instance, service, target) = fixture();
    {
        let mut state = context.server.state_mut();
        let character = state.characters_mut().get_mut(&target).unwrap();
        character.script_skill_state.casting_skill_id = SkillEnum::MgFirebolt.id();
        character.script_skill_state.casting_until = 9000;
        character.script_skill_state.cast_generation = 1;
    }
    let mut cast = request(NPC_ID, target, SkillEnum::SmProvoke);
    cast.level = 10;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
    let state = context.server.state();
    let character = state.get_character(target).unwrap();
    let provoke = character.status.status_change(StatusChangeKind::Provoke).unwrap();
    assert_eq!(provoke.values[1..3], [0, 100]);
    assert_eq!(character.script_skill_state.casting_until, 0);
    assert_eq!(character.script_skill_state.cast_generation, 2);
}

#[test]
fn npc_ground_cast_keeps_a_live_source_and_damages_player_targets() {
    let (context, instance, service, target) = fixture();
    let mut cast = request(NPC_ID, 0, SkillEnum::WzHeavendrive);
    cast.ground = Some((51, 50));
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
    context
        .server
        .script_skill_service()
        .tick_ground_skills(&context.server, &context.server.state(), 1000);
    let mut damages = vec![];
    while let Some(events) = context.server_task_queue.pop() {
        for event in events {
            if let GameEvent::CharacterDamage(CharacterDamage { damage }) = event {
                damages.push(damage);
            }
        }
    }
    let damage = damages
        .iter()
        .find(|damage| damage.target_id == target)
        .expect("Ground spell did not reach the selected player");
    assert_eq!(
        (damage.attacker_id, damage.credit_id, damage.target_id),
        (NPC_ID, NPC_ID, target)
    );
    assert!(damage.damage > 0 && damage.magic_context.is_some());
}

#[test]
fn npc_hunter_traps_trigger_on_the_players_standing_on_them() {
    for skill in [
        SkillEnum::HtClaymoretrap,
        SkillEnum::HtBlastmine,
        SkillEnum::HtAnklesnare,
        SkillEnum::HtShockwave,
        SkillEnum::HtFlasher,
    ] {
        let (context, instance, service, target) = fixture();
        instance.state_mut().mobs_mut().clear();
        let (x, y) = {
            let state = context.server.state();
            let character = state.get_character(target).expect("trap target is in game");
            (character.x(), character.y())
        };
        let mut cast = request(NPC_ID, 0, skill);
        cast.ground = Some((x, y));
        cast.cast_time_adjust_ms = -10000;
        start(&context, &instance, &service, cast, 1000).unwrap_or_else(|error| panic!("{skill:?}: {error}"));
        assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1, "{skill:?} did not finish casting");
        let ticks = [1000, 1040, 1080, 1120];
        for tick in ticks {
            context
                .server
                .script_skill_service()
                .tick_ground_skills(&context.server, &context.server.state(), tick);
        }
        let mut reached_target = false;
        while let Some(events) = context.server_task_queue.pop() {
            for event in events {
                let hit = match event {
                    GameEvent::ScriptMapDamage(request) => request.damage.target_id,
                    GameEvent::GroundTrapEffect(effect) => effect.target_id,
                    GameEvent::GroundTrapCapture(capture) => capture.target_id,
                    _ => continue,
                };
                reached_target |= context.server.state().get_character(hit).is_some();
            }
        }
        assert!(reached_target, "{skill:?} placed by an NPC never reached the player standing on it");
    }
}

#[test]
fn npc_evil_land_blinds_the_players_inside_it() {
    let (context, instance, service, target) = fixture();
    let (x, y) = {
        let state = context.server.state();
        let character = state.get_character(target).expect("evil land target is in game");
        (character.x(), character.y())
    };
    let mut cast = request(NPC_ID, 0, SkillEnum::NpcEvilland);
    cast.ground = Some((x, y));
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
    let mut blinded = vec![];
    for tick in [1000, 1040, 2100] {
        context
            .server
            .script_skill_service()
            .tick_ground_skills(&context.server, &context.server.state(), tick);
        while let Some(events) = context.server_task_queue.pop() {
            for event in events {
                if let GameEvent::CharacterStatusChange(change) = event {
                    if change.request.kind == StatusChangeKind::Blind {
                        blinded.push(change.char_id);
                    }
                }
            }
        }
    }
    assert!(blinded.contains(&target), "the player inside Evil Land was not blinded: {blinded:?}");
}

fn drain_events(context: &ServerServiceTestContext) -> Vec<GameEvent> {
    let mut drained = vec![];
    while let Some(events) = context.server_task_queue.pop() {
        drained.extend(events);
    }
    drained
}

#[test]
fn only_rule_rejections_are_treated_as_handled_outcomes() {
    for message in [actor::TELEPORT_DISABLED, actor::PLACEMENT_OCCUPIED, actor::PLACEMENT_OVERLAPS_UNIT] {
        assert!(actor::is_expected_rejection(message), "{message}");
    }
    assert!(!actor::is_expected_rejection("Unit skill caster left the map"));
}

#[test]
fn teleport_on_a_no_teleport_map_completes_as_a_handled_rejection() {
    let (context, instance, service, _) = fixture();
    let mut flags = MapFlags::default();
    flags.set(MapFlag::NoTeleport, true, &[]).unwrap();
    context.server.map_flag_overrides().insert(("empty".into(), 0), flags);
    let mut cast = request(MOB_ID, MOB_ID, SkillEnum::AlTeleport);
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
    assert!(instance.task_queue().pop().is_none(), "the mob must not be warped");
}

#[test]
fn trap_placement_blocked_by_a_living_mob_completes_as_a_handled_rejection() {
    let (context, instance, service, _) = fixture();
    let mut cast = request(MOB_ID, 0, SkillEnum::HtAnklesnare);
    cast.ground = Some((51, 50));
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1000).unwrap();
    {
        let mut blocker = crate::tests::common::mob_helper::create_mob(MOB_ID + 1, "PORING");
        blocker.x = 51;
        blocker.y = 50;
        instance.state_mut().insert_mob(blocker);
    }
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
}

#[test]
fn npc_fire_pillar_waits_for_an_enemy_then_erupts_once_and_stops_it() {
    let (context, instance, service, target) = fixture();
    instance.state_mut().mobs_mut().clear();
    let (x, y) = {
        let state = context.server.state();
        let character = state.get_character(target).expect("fire pillar target is in game");
        (character.x(), character.y())
    };
    let mut cast = request(NPC_ID, 0, SkillEnum::WzFirepillar);
    cast.level = 5;
    cast.ground = Some((x.saturating_add(8), y));
    cast.ignore_range = true;
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
    context
        .server
        .script_skill_service()
        .tick_ground_skills(&context.server, &context.server.state(), 1040);
    assert!(
        drain_events(&context)
            .iter()
            .all(|event| !matches!(event, GameEvent::ScriptMapDamage(_))),
        "the pillar erupted with nobody near it"
    );
    let mut cast = request(NPC_ID, 0, SkillEnum::WzFirepillar);
    cast.level = 5;
    cast.ground = Some((x, y));
    cast.ignore_range = true;
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1100).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1100).unwrap(), 1);
    let mut damages = vec![];
    let mut stops = vec![];
    for tick in [1100, 1140, 1180] {
        context
            .server
            .script_skill_service()
            .tick_ground_skills(&context.server, &context.server.state(), tick);
        for event in drain_events(&context) {
            match event {
                GameEvent::ScriptMapDamage(request) => damages.push(request.damage),
                GameEvent::GroundTrapEffect(effect) => stops.push(effect),
                _ => {}
            }
        }
    }
    let own: Vec<_> = damages.iter().filter(|damage| damage.target_id == target).collect();
    assert_eq!(own.len(), 1, "the pillar must erupt exactly once on the player: {}", own.len());
    let damage = own[0];
    assert_eq!((damage.target_id, damage.attacker_id), (target, NPC_ID));
    assert!(damage.damage > 0 && damage.magic_context.is_some_and(|context| context.hits == 7));
    assert!(stops.iter().any(|effect| {
        effect.target_id == target
            && effect.kind == crate::server::script::skill::trap::GroundTrapEffectKind::WalkDelay { milliseconds: 1400 }
    }));
}

#[test]
fn fire_pillar_damage_is_spread_for_players_and_multiplied_for_other_sources() {
    let (context, _instance, _service, target) = fixture();
    let status = {
        let state = context.server.state();
        crate::server::service::status_service::StatusService::instance().to_snapshot(&state.get_character(target).unwrap().status)
    };
    let battle = context.server.battle_service();
    let (single, single_context) = battle.fire_pillar_damage_signed(&status, &status, SkillEnum::WzFirepillar.id(), 5, 7, true);
    let (multiple, multiple_context) = battle.fire_pillar_damage_signed(&status, &status, SkillEnum::WzFirepillar.id(), 5, 7, false);
    assert_eq!((single_context.hits, multiple_context.hits), (1, 7));
    assert!(single > 0 && multiple >= single * 6, "single {single}, multiple {multiple}");
}

#[test]
fn npc_demonstration_hits_every_second_and_never_the_emperium() {
    let (context, instance, service, target) = fixture();
    instance.state_mut().mobs_mut().clear();
    let (x, y) = {
        let state = context.server.state();
        let character = state.get_character(target).expect("demonstration target is in game");
        (character.x(), character.y())
    };
    let mut cast = request(NPC_ID, 0, SkillEnum::AmDemonstration);
    cast.level = 3;
    cast.ground = Some((x, y));
    cast.ignore_range = true;
    cast.cast_time_adjust_ms = -10000;
    start(&context, &instance, &service, cast, 1000).unwrap();
    assert_eq!(complete(&context, &instance, &service, 1000).unwrap(), 1);
    let mut hit_ticks = vec![];
    for tick in [1000, 1040, 1500, 2000, 2040, 3040] {
        context
            .server
            .script_skill_service()
            .tick_ground_skills(&context.server, &context.server.state(), tick);
        if drain_events(&context)
            .iter()
            .any(|event| matches!(event, GameEvent::ScriptMapDamage(request) if request.damage.target_id == target))
        {
            hit_ticks.push(tick);
        }
    }
    assert_eq!(hit_ticks, vec![1000, 2000, 3040], "one hit per interval, then the bomb keeps ticking");
}

#[test]
fn scripted_player_costs_are_atomic_at_completion_and_interrupted_casts_do_not_pay() {
    let (context, instance, service, target) = fixture();
    let source = target - 1;
    let mut cast = request(source, target, SkillEnum::MgFirebolt);
    cast.cast_time_adjust_ms = 5000;
    context
        .server
        .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptUnitSkill(cast.clone()), 1000)
        .unwrap();
    assert_eq!(context.server.state().get_character(source).unwrap().status.sp, 1000);
    context
        .server
        .script_skill_service()
        .cancel_queued_cast(context.server.state_mut().characters_mut().get_mut(&source).unwrap());
    assert!(complete(&context, &instance, &service, 10000).is_err());
    assert_eq!(context.server.state().get_character(source).unwrap().status.sp, 1000);
    context
        .server
        .handle_script_event(&mut context.server.state_mut(), GameEvent::ScriptUnitSkill(cast), 10001)
        .unwrap();
    assert_eq!(complete(&context, &instance, &service, 20000).unwrap(), 1);
    assert_eq!(context.server.state().get_character(source).unwrap().status.sp, 988);
}
