use models::enums::EnumWithMaskValueU32;
use models::enums::skill_enums::SkillEnum;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChange, StatusChangeKind};
use script_sdk::{Function, Value};

use crate::server::model::action::Damage;
use crate::server::model::events::game_event::{CharacterUseSkill, GameEvent, CharacterDamage};
use crate::server::model::events::map_event::{MapEvent, SetMapFlags};
use crate::server::model::map_flags::{MapFlag, MapFlags};

fn damage(source: u32, target: u32) -> Damage {
    Damage { notification: None,
        source_kind: models::enums::actor::CombatActorKind::Player,
        skill_damage_adjusted: false,
        target_id: target,
        attacker_id: source,
        credit_id: source,
        damage: 100,
        healing: 0,
        right_hand_damage: None,
        attacked_at: 1000,
        damage_motion: 0,
        battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Short.as_flag() | BattleFlag::Normal.as_flag(),
        skill_id: 0,
        skill_level: 0,
        landed: true,
        proc_depth: 0,
        defenses_applied: true,
        magic_context: None,
    }
}

#[test]
fn ordinary_maps_deny_hostile_player_skills_before_cast_or_payment() {
    let (context, repository, mut source) = super::native_payment_tests::fixture(false, true);
    let target = source.char_id + 1;
    context
        .server
        .state_mut()
        .runtime_map_flags
        .insert(("empty".into(), 0), MapFlags::default());
    context.server.server_service().character_start_use_skill(
        &context.server,
        context.server.state(),
        &mut source,
        CharacterUseSkill {
            char_id: 150000,
            target_id: target,
            skill_id: SkillEnum::MgColdbolt.id(),
            skill_level: 1,
        },
        1000,
    );
    assert!(!source.is_using_skill());
    assert_eq!(source.status.sp, 1000);
    assert!(source.script_skill_state.native_requirements.is_none());
    let stored =
        database::required::<database::model::CharacterRecord>(&repository.database.characters, &source.char_id.to_be_bytes()).unwrap();
    assert_eq!(stored.sp, 1000);
    assert!(!context.server.player_combat_target_allowed(context.server.state(), &source, target));
    assert!(
        context
            .server
            .player_skill_target_allowed(context.server.state(), &source, target, SkillEnum::AlBlessing.id(), false)
    );
}

#[test]
fn pvp_protects_party_and_guild_members_until_the_corresponding_flags_are_set() {
    let (context, _, mut source) = super::native_payment_tests::fixture(false, true);
    let target = source.char_id + 1;
    assert!(context.server.player_combat_target_allowed(context.server.state(), &source, target));
    source.game_systems.party_id = 7;
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&target)
        .unwrap()
        .game_systems
        .party_id = 7;
    assert!(!context.server.player_combat_target_allowed(context.server.state(), &source, target));
    context
        .server
        .state_mut()
        .runtime_map_flags
        .get_mut(&("empty".into(), 0))
        .unwrap()
        .set(MapFlag::PvpNoParty, true, &[])
        .unwrap();
    assert!(context.server.player_combat_target_allowed(context.server.state(), &source, target));
    source.game_systems.guild_id = 8;
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&target)
        .unwrap()
        .game_systems
        .guild_id = 8;
    assert!(!context.server.player_combat_target_allowed(context.server.state(), &source, target));
    context
        .server
        .state_mut()
        .runtime_map_flags
        .get_mut(&("empty".into(), 0))
        .unwrap()
        .set(MapFlag::PvpNoGuild, true, &[])
        .unwrap();
    assert!(context.server.player_combat_target_allowed(context.server.state(), &source, target));
    assert!(
        !context
            .server
            .player_combat_target_allowed(context.server.state(), &source, source.char_id)
    );
}

#[test]
fn castle_player_combat_requires_an_active_siege_and_rejects_other_instances() {
    let (context, _, source) = super::native_payment_tests::fixture(false, true);
    let target = source.char_id + 1;
    let mut flags = MapFlags::default();
    flags.set(MapFlag::GvgCastle, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    assert!(!context.server.player_combat_target_allowed(context.server.state(), &source, target));
    context.server.state_mut().siege_active = true;
    assert!(context.server.player_combat_target_allowed(context.server.state(), &source, target));
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&target)
        .unwrap()
        .map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), 7);
    assert!(!context.server.player_combat_target_allowed(context.server.state(), &source, target));
}

#[test]
fn normal_player_attacks_use_the_player_damage_queue_and_stop_when_pvp_is_disabled() {
    let (context, _, mut source) = super::native_payment_tests::fixture(false, true);
    let target = source.char_id + 1;
    source.set_attack(target, true, 0);
    context
        .server
        .server_service()
        .character_attack(&context.server, context.server.state(), 1000, &mut source);
    let tasks = context.server_task_queue.pop().unwrap_or_default();
    assert!(
        tasks
            .iter()
            .any(|task| matches!(task,GameEvent::CharacterDamage(CharacterDamage { damage }) if damage.target_id==target&&damage.attacker_id==source.char_id))
    );
    context
        .server
        .state_mut()
        .runtime_map_flags
        .insert(("empty".into(), 0), MapFlags::default());
    context
        .server
        .server_service()
        .character_attack(&context.server, context.server.state(), 2000, &mut source);
    assert!(!source.is_attacking());
    assert!(
        context
            .server_task_queue
            .pop()
            .unwrap_or_default()
            .iter()
            .all(|task| !matches!(task, GameEvent::CharacterDamage(CharacterDamage { damage: _ })))
    );
}

#[test]
fn a_map_rule_change_before_landed_damage_preserves_hp_and_shield_charges() {
    let (context, _, source) = super::native_payment_tests::fixture(false, true);
    let source_id = source.char_id;
    let target_id = source_id + 1;
    let shield = StatusChange {
        kind: StatusChangeKind::Kyrie,
        values: [1, 200, 3, 0],
        started_at: 0,
        expires_at: Some(10000),
        next_periodic_at: 0,
        flags: 0,
        inherited_from: None,
    };
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&target_id)
        .unwrap()
        .status
        .active_statuses
        .push(shield);
    context.server.state_mut().insert_character(source);
    context
        .server
        .state_mut()
        .runtime_map_flags
        .insert(("empty".into(), 0), MapFlags::default());
    context
        .server
        .admit_character_damage(context.server.state_mut().as_mut(), damage(source_id, target_id), 1000)
        .unwrap();
    let target = context.server.state().characters().get(&target_id).unwrap();
    assert_eq!(target.status.hp, 1000);
    assert_eq!(target.status.status_change(StatusChangeKind::Kyrie).unwrap().values, [
        1, 200, 3, 0
    ]);
}

#[test]
fn live_map_flag_calls_update_the_main_state_and_enqueue_the_same_flags_for_the_map_thread() {
    let (context, _, source) = super::native_payment_tests::fixture(false, true);
    let key = source.map_instance_key.clone();
    let source_id = source.char_id;
    let instance = context.server.state().get_map_instance_from_character(&source).unwrap();
    context.server.state_mut().insert_character(source);
    context
        .server
        .map_flag_call(context.server.state_mut().as_mut(), source_id, Function::SetMapFlag, &[
            Value::from("empty.gat"),
            Value::from(MapFlag::NoTeleport as i32),
        ])
        .unwrap();
    assert!(context.server.state().map_flags(&key).enabled(MapFlag::NoTeleport));
    assert_eq!(
        context
            .server
            .map_flag_call(context.server.state_mut().as_mut(), source_id, Function::GetMapFlag, &[
                Value::from("empty"),
                Value::from(MapFlag::NoTeleport as i32)
            ])
            .unwrap(),
        Value::Number(1)
    );
    assert!(
        instance
            .task_queue()
            .pop()
            .unwrap_or_default()
            .iter()
            .any(|event| matches!(event,MapEvent::SetMapFlags(SetMapFlags { flags }) if flags.enabled(MapFlag::NoTeleport)))
    );
    context
        .server
        .map_flag_call(context.server.state_mut().as_mut(), source_id, Function::RemoveMapFlag, &[
            Value::from("empty"),
            Value::from(MapFlag::NoTeleport as i32),
        ])
        .unwrap();
    assert!(!context.server.state().map_flags(&key).enabled(MapFlag::NoTeleport));
}

#[test]
fn gvg_map_damage_rates_distinguish_skill_damage_and_normal_attack_range() {
    let (context, ..) = super::native_payment_tests::fixture(false, true);
    let config = &crate::server::service::global_config_service::GlobalConfigService::instance()
        .config()
        .game;
    let apply = crate::server::service::map_flag_service::apply_map_combat_damage;
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Gvg, true, &[]).unwrap();
    for kind in [BattleFlag::Weapon, BattleFlag::Magic, BattleFlag::Misc] {
        assert_eq!(apply(&flags, config, 100, 0, kind.as_flag() | BattleFlag::Skill.as_flag()), 60);
    }
    for range in [BattleFlag::Short, BattleFlag::Long] {
        assert_eq!(
            apply(
                &flags,
                config,
                100,
                0,
                BattleFlag::Weapon.as_flag() | range.as_flag() | BattleFlag::Normal.as_flag()
            ),
            80
        );
    }
    assert_eq!(
        apply(
            &flags,
            config,
            100,
            0,
            BattleFlag::Skill.as_flag() | BattleFlag::Weapon.as_flag() | BattleFlag::Magic.as_flag()
        ),
        36
    );
    assert_eq!(
        apply(&flags, config, 1, 0, BattleFlag::Skill.as_flag() | BattleFlag::Weapon.as_flag()),
        1
    );
    assert_eq!(
        apply(&flags, config, 0, 0, BattleFlag::Skill.as_flag() | BattleFlag::Weapon.as_flag()),
        0
    );
    let exempt = crate::server::script::skill::metadata::SkillMetadata::all()
        .iter()
        .find(|skill| skill.flags.get("IgnoreGvgReduction") == Some(&true))
        .unwrap();
    assert_eq!(
        apply(
            &flags,
            config,
            100,
            exempt.id,
            BattleFlag::Skill.as_flag() | BattleFlag::Misc.as_flag()
        ),
        100
    );
    flags.set(MapFlag::Battleground, true, &[1]).unwrap();
    assert_eq!(
        apply(
            &flags,
            config,
            100,
            0,
            BattleFlag::Skill.as_flag() | BattleFlag::Magic.as_flag()
        ),
        60
    );
    assert_eq!(
        apply(
            &MapFlags::default(),
            config,
            100,
            0,
            BattleFlag::Skill.as_flag() | BattleFlag::Magic.as_flag()
        ),
        100
    );
}

#[test]
fn player_gvg_damage_consumes_a_shield_before_applying_the_map_reduction() {
    let (context, _, source) = super::native_payment_tests::fixture(false, true);
    let source_id = source.char_id;
    let target_id = source_id + 1;
    let shield = StatusChange {
        kind: StatusChangeKind::Kyrie,
        values: [1, 50, 3, 0],
        started_at: 0,
        expires_at: Some(10000),
        next_periodic_at: 0,
        flags: 0,
        inherited_from: None,
    };
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&target_id)
        .unwrap()
        .status
        .active_statuses
        .push(shield);
    context.server.state_mut().insert_character(source);
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Gvg, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    context
        .server
        .admit_character_damage(context.server.state_mut().as_mut(), damage(source_id, target_id), 1000)
        .unwrap();
    let target = context.server.state().get_character(target_id).unwrap();
    assert_eq!(target.status.hp, 960);
    assert!(!target.status.has_status_change(StatusChangeKind::Kyrie));
}

#[test]
fn status_alternatives_on_the_player_event_stop_after_the_first_accepted_status() {
    use models::status_change::StatusChangeRequest;

    use crate::server::model::events::game_event::CharacterStatusAlternatives;
    for first_succeeds in [true, false] {
        let (context, _, source) = super::native_payment_tests::fixture(false, true);
        let target = source.char_id + 1;
        let mut first = StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 10000, 1);
        if !first_succeeds {
            first.rate = 0;
            first.flags = 0;
        }
        context
            .server
            .handle_script_event(
                context.server.state_mut().as_mut(),
                GameEvent::CharacterStatusAlternatives(CharacterStatusAlternatives {
                    char_id: target,
                    requests: vec![first, StatusChangeRequest::guaranteed(StatusChangeKind::Blind, 10000, 1)],
                }),
                1000,
            )
            .unwrap();
        let status = &context.server.state().get_character(target).unwrap().status;
        assert_eq!(status.has_status_change(StatusChangeKind::Stun), first_succeeds);
        assert_eq!(status.has_status_change(StatusChangeKind::Blind), !first_succeeds);
    }
}

#[test]
fn dispel_can_target_its_own_party_on_an_ordinary_map() {
    let (context, _, mut source) = super::native_payment_tests::fixture(false, true);
    let target = source.char_id + 1;
    context
        .server
        .state_mut()
        .runtime_map_flags
        .insert(("empty".into(), 0), MapFlags::default());
    assert!(
        !context
            .server
            .player_skill_target_allowed(context.server.state(), &source, target, SkillEnum::SaDispell.id(), false)
    );
    source.game_systems.party_id = 7;
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&target)
        .unwrap()
        .game_systems
        .party_id = 7;
    assert!(
        context
            .server
            .player_skill_target_allowed(context.server.state(), &source, target, SkillEnum::SaDispell.id(), false)
    );
    assert!(context.server.player_skill_target_allowed(
        context.server.state(),
        &source,
        source.char_id,
        SkillEnum::SaDispell.id(),
        false
    ));
}
