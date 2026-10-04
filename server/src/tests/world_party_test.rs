use std::sync::Arc;
use std::time::{Duration, Instant};

use database::model::{AccountRecord, CharacterRecord};
use models::enums::EnumWithMaskValueU32;
use models::enums::element::Element;
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;
use models::status_bonus::BattleFlag;
use models::status_change::{StatusChangeKind, StatusChangeRequest};
use script_sdk::{Function, Value};
use sled::transaction::Transactional;

use super::ServerServiceTestContext;
use crate::repository::SledRepository;
use crate::repository::game_system_repository::GameSystemRepository;
use crate::server::model::action::Damage;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::game_systems::{HomunculusRecord, ScriptWorldRequest};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_combat_service::MagicAttackContext;
use crate::server::service::script_world_service::ScriptWorldService;
use crate::server::service::status_effect_service::StatusEffectService;

#[path = "world_pet_test.rs"]
mod pet_tests;

pub(super) fn fixture() -> (ServerServiceTestContext, Arc<SledRepository>) {
    let (context, repository, mut leader) = super::native_payment_tests::fixture(false, true);
    leader.status.known_skills.push(KnownSkill {
        value: SkillEnum::NvBasic,
        level: 7,
    });
    leader.status.base_level = 50;
    let mut member = crate::tests::common::character_helper::create_character();
    member.char_id = leader.char_id + 1;
    member.account_id = leader.account_id + 1;
    member.name = "Party Member".into();
    member.map_instance_key = leader.map_instance_key.clone();
    member.x = 51;
    member.y = 50;
    member.status.hp = 1000;
    member.status.max_hp = 1000;
    member.status.base_level = 50;
    (&repository.database.accounts, &repository.database.characters)
        .transaction(|(accounts, characters)| {
            database::tx_write(accounts, &member.account_id.to_be_bytes(), &AccountRecord {
                account_id: member.account_id,
                username: "party-member".into(),
                password: "secret".into(),
            })?;
            database::tx_write(characters, &member.char_id.to_be_bytes(), &CharacterRecord {
                char_id: member.char_id as i32,
                account_id: member.account_id as i32,
                name: member.name.clone(),
                hp: 1000,
                max_hp: 1000,
                base_level: 50,
                inventory_slots: 100,
                last_map: "empty".into(),
                ..CharacterRecord::default()
            })
        })
        .unwrap();
    context.server.state_mut().insert_character(leader);
    context.server.state_mut().insert_character(member);
    (context, repository)
}

fn request(context: &ServerServiceTestContext, actor: u32, request: ScriptWorldRequest) -> Result<(), String> {
    context
        .server
        .script_world_service()
        .handle_request(&context.server, context.server.state_mut().as_mut(), actor, request, 100)
}

fn wait_packet(context: &ServerServiceTestContext, actor: u32, id: u16) -> Vec<u8> {
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        context.server.script_world_service().drain_notifications().unwrap();
        let notifications = context.test_context.received_notification();
        if let Some(packet) = notifications.lock().unwrap().iter().find_map(|notification| match notification {
            Notification::Char(notification)
                if notification.char_id() == actor && notification.serialized_packet().get(..2) == Some(&id.to_le_bytes()) =>
            {
                Some(notification.serialized_packet().clone())
            }
            _ => None,
        }) {
            return packet;
        }
        assert!(Instant::now() < deadline, "Missing party packet {id:#06x}");
        std::thread::sleep(Duration::from_millis(5));
    }
}

#[test]
fn party_world_requests_require_pending_invitations_and_keep_the_live_and_saved_roster_in_sync() {
    let (context, repository) = fixture();
    request(&context, 150_000, ScriptWorldRequest::CreateParty {
        name: "Play Party".into(),
        item_pickup: true,
        item_share: true,
    })
    .unwrap();
    let party_id = context.server.state().get_character(150_000).unwrap().game_systems.party_id;
    assert_ne!(party_id, 0);
    assert_eq!(wait_packet(&context, 150_000, 0x00FA), vec![0xFA, 0, 0]);
    assert!(
        request(&context, 150_001, ScriptWorldRequest::AnswerPartyInvite {
            party_id,
            accept: true
        })
        .is_err()
    );
    assert_eq!(repository.character_game_systems(150_001).unwrap().party_id, 0);
    request(&context, 150_000, ScriptWorldRequest::InviteParty(2_000_001)).unwrap();
    let invite = wait_packet(&context, 150_001, 0x02C6);
    assert_eq!(invite.len(), 30);
    assert_eq!(u32::from_le_bytes(invite[2..6].try_into().unwrap()), party_id);
    assert!(
        request(&context, 150_001, ScriptWorldRequest::AnswerPartyInvite {
            party_id: party_id + 1,
            accept: true
        })
        .is_err()
    );
    request(&context, 150_000, ScriptWorldRequest::InvitePartyByName("Party Member".into())).unwrap();
    request(&context, 150_001, ScriptWorldRequest::AnswerPartyInvite {
        party_id,
        accept: true,
    })
    .unwrap();
    for id in [150_000, 150_001] {
        let live = &context.server.state().get_character(id).unwrap().game_systems;
        let saved = repository.character_game_systems(id).unwrap();
        assert_eq!(live.revision, saved.revision);
        assert_eq!(live.party_members, vec![150_000, 150_001]);
        assert_eq!(live.party.as_ref().unwrap().members, live.party_members);
    }
    assert!(
        request(&context, 150_001, ScriptWorldRequest::ExpelParty {
            account_id: 2_000_000,
            name: "Walkiry".into()
        })
        .is_err()
    );
    request(&context, 150_000, ScriptWorldRequest::ChangePartyOptions {
        exp_share: true,
        item_rules: Some((false, true)),
    })
    .unwrap();
    assert!(
        context
            .server
            .state()
            .get_character(150_001)
            .unwrap()
            .game_systems
            .party
            .as_ref()
            .unwrap()
            .exp_share
    );
    request(&context, 150_000, ScriptWorldRequest::ChangePartyLeader(2_000_001)).unwrap();
    assert_eq!(
        context.server.state().get_character(150_000).unwrap().game_systems.party_leader_id,
        150_001
    );
    request(&context, 150_001, ScriptWorldRequest::LeaveParty).unwrap();
    assert!(repository.party(party_id).unwrap().is_none());
    for id in [150_000, 150_001] {
        let live = &context.server.state().get_character(id).unwrap().game_systems;
        assert_eq!(live.party_id, 0);
        assert!(live.party.is_none() && live.party_members.is_empty());
        assert_eq!(live.revision, repository.character_game_systems(id).unwrap().revision);
    }
    assert_eq!(wait_packet(&context, 150_000, 0x0105).len(), 31);
}

#[test]
fn party_creation_checks_basic_skill_and_installs_committed_state_even_if_notification_delivery_fails() {
    let (context, repository) = fixture();
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&150_000)
        .unwrap()
        .status
        .known_skills
        .clear();
    request(&context, 150_000, ScriptWorldRequest::CreateParty {
        name: "Blocked Party".into(),
        item_pickup: false,
        item_share: false,
    })
    .unwrap();
    assert_eq!(repository.character_game_systems(150_000).unwrap().party_id, 0);
    assert_eq!(wait_packet(&context, 150_000, 0x0110).len(), 10);
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&150_000)
        .unwrap()
        .status
        .known_skills
        .push(KnownSkill {
            value: SkillEnum::NvBasic,
            level: 7,
        });
    let (notifications, receiver) = std::sync::mpsc::sync_channel(1);
    drop(receiver);
    let service = ScriptWorldService::new(notifications, repository.clone(), GlobalConfigService::instance());
    assert!(
        service
            .handle_request(
                &context.server,
                context.server.state_mut().as_mut(),
                150_000,
                ScriptWorldRequest::CreateParty {
                    name: "Committed Party".into(),
                    item_pickup: false,
                    item_share: false
                },
                100
            )
            .is_err()
    );
    let live = &context.server.state().get_character(150_000).unwrap().game_systems;
    let saved = repository.character_game_systems(150_000).unwrap();
    assert_ne!(saved.party_id, 0);
    assert_eq!((live.party_id, live.revision), (saved.party_id, saved.revision));
    assert!(live.party.is_some());
}

fn install_homunculus(context: &ServerServiceTestContext, repository: &SledRepository, statuses: &[StatusChangeKind]) -> u32 {
    let mut homunculus = HomunculusRecord {
        id: 8,
        class_id: 6001,
        name: "Lif".into(),
        level: 10,
        hp: 100,
        max_hp: 100,
        max_sp: 100,
        base_max_hp: 100,
        base_max_sp: 100,
        stats: [10; 6],
        active: true,
        ..HomunculusRecord::default()
    };
    let mut status = models::status::Status {
        hp: 100,
        max_hp: 100,
        max_sp: 100,
        ..models::status::Status::default()
    };
    for kind in statuses {
        StatusEffectService::apply_status(&mut status, StatusChangeRequest::guaranteed(*kind, 60000, 5), 0, 0).unwrap();
    }
    homunculus.statuses = status.active_statuses;
    let id = crate::server::service::script_world_service::homunculus_world_id(&homunculus);
    let mut systems = context.server.state().get_character(150_001).unwrap().game_systems.clone();
    systems.homunculus = Some(homunculus);
    let saved = repository.save_character_game_systems(150_001, &systems).unwrap();
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().game_systems = saved;
    id
}

#[test]
fn companion_magic_reflection_precedes_absorption_and_reflected_magic_cannot_bounce_again() {
    let (context, repository) = fixture();
    let owner_sp = context.server.state().get_character(150_001).unwrap().status.sp;
    let id = install_homunculus(&context, &repository, &[
        StatusChangeKind::MagicMirror,
        StatusChangeKind::MagicRod,
    ]);
    let damage = Damage {
        target_id: id,
        attacker_id: 150_000,
        credit_id: 150_000,
        damage: 40,
        attacked_at: 100,
        damage_motion: 0,
        battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Skill.as_flag() | BattleFlag::Long.as_flag(),
        skill_id: SkillEnum::MgFirebolt.id(),
        skill_level: 1,
        landed: true,
        proc_depth: 0,
        defenses_applied: true,
        magic_context: Some(MagicAttackContext::new(100, 1.0, Element::Fire, 1, SkillEnum::MgFirebolt.id())),
        right_hand_damage: None,
        healing: 0,
    };
    assert!(
        context
            .server
            .script_world_service()
            .handle_companion_damage(&context.server, context.server.state_mut().as_mut(), damage, 100)
            .unwrap()
    );
    let homunculus = context
        .server
        .state()
        .get_character(150_001)
        .unwrap()
        .game_systems
        .homunculus
        .as_ref()
        .unwrap();
    assert_eq!((homunculus.hp, homunculus.sp), (100, 0));
    let reflected = context
        .server_task_queue
        .pop()
        .unwrap()
        .into_iter()
        .find_map(|event| match event {
            GameEvent::CharacterDamage(damage) => Some(damage),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        (
            reflected.target_id,
            reflected.attacker_id,
            reflected.credit_id,
            reflected.damage
        ),
        (150_000, id, 150_001, 40)
    );
    assert!(!reflected.landed);
    assert_eq!(reflected.battle_flags, damage.battle_flags);
    assert!(
        context
            .server
            .script_world_service()
            .handle_companion_damage(
                &context.server,
                context.server.state_mut().as_mut(),
                Damage { landed: false, ..damage },
                101
            )
            .unwrap()
    );
    let live = context.server.state().get_character(150_001).unwrap();
    let homunculus = live.game_systems.homunculus.as_ref().unwrap();
    assert_eq!((homunculus.hp, homunculus.sp), (100, 12));
    assert_eq!(live.status.sp, owner_sp);
    assert_eq!(repository.character_game_systems(150_001).unwrap().homunculus.unwrap().sp, 12);
    assert!(context.server_task_queue.is_empty());
}

#[test]
fn companion_damage_rejected_by_persistence_does_not_mutate_live_pools_or_shields() {
    let (context, repository) = fixture();
    let id = install_homunculus(&context, &repository, &[]);
    let before = context.server.state().get_character(150_001).unwrap().game_systems.clone();
    let mut replacement = before.clone();
    replacement.font = 5;
    let replacement = repository.save_character_game_systems(150_001, &replacement).unwrap();
    let damage = Damage {
        target_id: id,
        attacker_id: 150_000,
        credit_id: 150_000,
        damage: 40,
        attacked_at: 100,
        damage_motion: 0,
        battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Normal.as_flag() | BattleFlag::Short.as_flag(),
        skill_id: 0,
        skill_level: 0,
        landed: true,
        proc_depth: 0,
        defenses_applied: true,
        magic_context: None,
        right_hand_damage: None,
        healing: 0,
    };
    assert!(
        context
            .server
            .script_world_service()
            .handle_companion_damage(&context.server, context.server.state_mut().as_mut(), damage, 100)
            .is_err()
    );
    assert_eq!(context.server.state().get_character(150_001).unwrap().game_systems, before);
    assert_eq!(repository.character_game_systems(150_001).unwrap(), replacement);
    assert!(context.server_task_queue.is_empty());
}

#[test]
fn companion_gvg_reduction_follows_shields_and_reflects_only_the_admitted_loss() {
    let (context, repository) = fixture();
    let id = install_homunculus(&context, &repository, &[StatusChangeKind::Kyrie, StatusChangeKind::ReflectShield]);
    context.server.map_flag_call(context.server.state_mut().as_mut(), 150_001, Function::SetMapFlag,
        &[Value::from("empty"), Value::Number(crate::server::model::map_flags::MapFlag::Gvg as i32)]).unwrap();
    let damage = Damage {
        target_id: id, attacker_id: 150_000, credit_id: 150_000, damage: 40, healing: 0,
        attacked_at: 100, damage_motion: 0,
        battle_flags: BattleFlag::Weapon.as_flag() | BattleFlag::Normal.as_flag() | BattleFlag::Short.as_flag(),
        skill_id: 0, skill_level: 0, landed: true, proc_depth: 0, defenses_applied: true,
        magic_context: None, right_hand_damage: None,
    };
    context.server.script_world_service().handle_companion_damage(&context.server, context.server.state_mut().as_mut(), damage, 100).unwrap();
    let live = context.server.state().get_character(150_001).unwrap();
    let homunculus = live.game_systems.homunculus.as_ref().unwrap();
    assert_eq!(homunculus.hp, 84);
    assert!(!homunculus.statuses.iter().any(|status| status.kind == StatusChangeKind::Kyrie));
    let saved = repository.character_game_systems(150_001).unwrap();
    assert_eq!((saved.revision, saved.homunculus.as_ref()), (live.game_systems.revision, Some(homunculus)));
    let reflected = context.server_task_queue.pop().unwrap().into_iter().find_map(|event| match event {
        GameEvent::CharacterDamage(damage) => Some(damage), _ => None,
    }).expect("Companion Reflect Shield did not use the actual admitted damage");
    assert_eq!((reflected.attacker_id, reflected.credit_id, reflected.target_id, reflected.damage), (id, 150_001, 150_000, 4));
    assert_eq!(reflected.battle_flags, 0);
    assert!(!reflected.landed);
}

#[test]
fn companion_status_alternatives_commit_only_the_first_actual_success_and_roll_back_stale_state() {
    let (context, repository) = fixture();
    let id = install_homunculus(&context, &repository, &[]);
    let world = context.server.script_world_service();
    let mut rejected = StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 1000, 1);
    rejected.flags = 0;
    rejected.rate = 0;
    let before = repository.character_game_systems(150_001).unwrap();
    assert!(world.handle_companion_status_alternatives(&context.server, context.server.state_mut().as_mut(), id,
        vec![rejected.clone(), rejected.clone()], 100).unwrap());
    assert_eq!(repository.character_game_systems(150_001).unwrap(), before);
    assert!(world.handle_companion_status_alternatives(&context.server, context.server.state_mut().as_mut(), id,
        vec![rejected, StatusChangeRequest::guaranteed(StatusChangeKind::Blind, 1000, 1),
            StatusChangeRequest::guaranteed(StatusChangeKind::Sleep, 1000, 1)], 100).unwrap());
    let saved = repository.character_game_systems(150_001).unwrap();
    assert_eq!(saved.revision, before.revision + 1);
    let statuses = &saved.homunculus.as_ref().unwrap().statuses;
    assert_eq!(statuses.len(), 1);
    assert_eq!(statuses[0].kind, StatusChangeKind::Blind);
    let live = context.server.state().get_character(150_001).unwrap().game_systems.clone();
    let mut competing = saved;
    competing.font = 2;
    let competing = repository.save_character_game_systems(150_001, &competing).unwrap();
    assert!(world.handle_companion_status_alternatives(&context.server, context.server.state_mut().as_mut(), id,
        vec![StatusChangeRequest::guaranteed(StatusChangeKind::Sleep, 1000, 1)], 101).is_err());
    assert_eq!(context.server.state().get_character(150_001).unwrap().game_systems, live);
    assert_eq!(repository.character_game_systems(150_001).unwrap(), competing);
}

#[test]
fn companion_pressure_bypasses_lex_assumptio_and_kyrie_without_consuming_the_shields() {
    for shield in [StatusChangeKind::Kyrie, StatusChangeKind::Assumptio] {
        let (context, repository) = fixture();
        let id = install_homunculus(&context, &repository, &[StatusChangeKind::LexAeterna, shield]);
        let before = context.server.state().get_character(150_001).unwrap().game_systems.homunculus.as_ref().unwrap().statuses.clone();
        let pressure = Damage { target_id: id, attacker_id: 150_000, credit_id: 150_000, damage: 40, healing: 0,
            right_hand_damage: None, attacked_at: 100, damage_motion: 0,
            battle_flags: BattleFlag::Misc.as_flag() | BattleFlag::Skill.as_flag() | BattleFlag::Long.as_flag(),
            skill_id: SkillEnum::PaPressure.id(), skill_level: 1, landed: true, proc_depth: 0, defenses_applied: true, magic_context: None };
        context.server.script_world_service().handle_companion_damage(&context.server, context.server.state_mut().as_mut(), pressure, 100).unwrap();
        let saved = repository.character_game_systems(150_001).unwrap();
        let homunculus = saved.homunculus.unwrap();
        assert_eq!(homunculus.hp, 60);
        assert_eq!(homunculus.statuses, before);
    }
}

#[test]
fn companion_elemental_absorption_heals_atomically_without_reflection_or_damage_callbacks() {
    let (context, repository) = fixture();
    let id = install_homunculus(&context, &repository, &[StatusChangeKind::MagicMirror, StatusChangeKind::MagicRod]);
    let mut systems = context.server.state().get_character(150_001).unwrap().game_systems.clone();
    systems.homunculus.as_mut().unwrap().hp = 40;
    let saved = repository.save_character_game_systems(150_001, &systems).unwrap();
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().game_systems = saved;
    let statuses = systems.homunculus.as_ref().unwrap().statuses.clone();
    let damage = Damage {
        target_id: id,
        attacker_id: 150_000,
        credit_id: 150_000,
        damage: 0,
        healing: u32::MAX,
        attacked_at: 100,
        damage_motion: 0,
        battle_flags: BattleFlag::Magic.as_flag() | BattleFlag::Skill.as_flag() | BattleFlag::Long.as_flag(),
        skill_id: SkillEnum::MgFirebolt.id(),
        skill_level: 1,
        landed: true,
        proc_depth: 0,
        defenses_applied: true,
        magic_context: Some(MagicAttackContext::new(100, 1.0, Element::Fire, 1, SkillEnum::MgFirebolt.id())),
        right_hand_damage: None,
    };
    let service = context.server.script_world_service();
    assert!(service.handle_companion_damage(&context.server, context.server.state_mut().as_mut(), damage, 100).unwrap());
    let live = context.server.state().get_character(150_001).unwrap();
    let homunculus = live.game_systems.homunculus.as_ref().unwrap();
    assert_eq!((homunculus.hp, homunculus.sp), (100, 0));
    assert_eq!(homunculus.statuses, statuses);
    assert_eq!(repository.character_game_systems(150_001).unwrap(), live.game_systems);
    assert!(context.server_task_queue.is_empty());

    let mut dead = live.game_systems.clone();
    dead.homunculus.as_mut().unwrap().hp = 0;
    let dead = repository.save_character_game_systems(150_001, &dead).unwrap();
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().game_systems = dead.clone();
    assert!(service.handle_companion_damage(&context.server, context.server.state_mut().as_mut(), damage, 101).unwrap());
    assert_eq!(repository.character_game_systems(150_001).unwrap(), dead);
    assert_eq!(context.server.state().get_character(150_001).unwrap().game_systems, dead);

    let mut injured = dead;
    injured.homunculus.as_mut().unwrap().hp = 40;
    let before = repository.save_character_game_systems(150_001, &injured).unwrap();
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().game_systems = before.clone();
    let mut competing = before.clone();
    competing.font = 3;
    let competing = repository.save_character_game_systems(150_001, &competing).unwrap();
    assert!(service.handle_companion_damage(&context.server, context.server.state_mut().as_mut(), damage, 102).is_err());
    assert_eq!(context.server.state().get_character(150_001).unwrap().game_systems, before);
    assert_eq!(repository.character_game_systems(150_001).unwrap(), competing);
}

#[test]
fn vip_queries_return_active_boolean_expiry_and_remaining_time_for_the_removed_source_actor() {
    let (context, repository) = fixture();
    repository.set_vip_expiration(2_000_000, 100).unwrap();
    let mut source = context.server.state_mut().characters_mut().remove(&150_000).unwrap();
    let service = context.server.script_world_service();
    for (mode, expected) in [(1, 1), (2, 100), (3, 40)] {
        assert_eq!(
            service
                .call(
                    &context.server,
                    &mut source,
                    Function::VipStatus,
                    &[Value::Number(mode), Value::String("Walkiry".into())],
                    60_000
                )
                .unwrap(),
            Value::Number(expected)
        );
        assert_eq!(
            service
                .call(
                    &context.server,
                    &mut source,
                    Function::VipStatus,
                    &[Value::Number(mode)],
                    100_000
                )
                .unwrap(),
            Value::Number(0)
        );
    }
    assert!(
        service
            .call(&context.server, &mut source, Function::VipStatus, &[Value::Number(4)], 60_000)
            .is_err()
    );
}
