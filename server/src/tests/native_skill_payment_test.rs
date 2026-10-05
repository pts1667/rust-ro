use std::sync::Arc;

use database::model::{AccountRecord, CharacterInventory, CharacterRecord, InventoryRecord, SeedData};
use models::enums::EnumWithMaskValueU64;
use models::enums::item::EquipmentLocation;
use models::enums::skill_enums::SkillEnum;

use super::{ServerServiceTestContext, before_each_with_repository};
use crate::repository::{InventoryRepository, SledRepository};
use crate::server::model::events::game_event::CharacterUseSkill;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::tests::common::character_helper::create_character;
use crate::tests::common::map_instance_helper::create_empty_map_instance;
use crate::tests::common::{self};

pub(super) fn fixture(arrow: bool, no_delay: bool) -> (ServerServiceTestContext, Arc<SledRepository>, Character) {
    common::before_all();
    let mut character = create_character();
    character.map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), 0);
    character.x = 50;
    character.y = 50;
    character.status.hp = 1000;
    character.status.sp = 1000;
    character.status.max_hp = 1000;
    character.status.max_sp = 1000;
    let repository = Arc::new(SledRepository::temporary().unwrap());
    let items = if arrow {
        vec![
            InventoryRecord {
                id: 10,
                item_id: 1901,
                amount: 1,
                equip: EquipmentLocation::HandRight.as_flag() as i32,
                is_identified: true,
                ..InventoryRecord::default()
            },
            InventoryRecord {
                id: 11,
                item_id: 1750,
                amount: 10,
                equip: EquipmentLocation::Ammo.as_flag() as i32,
                is_identified: true,
                ..InventoryRecord::default()
            },
        ]
    } else {
        vec![]
    };
    repository
        .database
        .seed(
            &SeedData {
                accounts: vec![AccountRecord {
                    account_id: character.account_id,
                    username: "Caster".into(),
                    password: "secret".into(),
                }],
                characters: vec![CharacterRecord {
                    char_id: character.char_id as i32,
                    account_id: character.account_id as i32,
                    name: character.name.clone(),
                    hp: 1000,
                    max_hp: 1000,
                    sp: 1000,
                    max_sp: 1000,
                    inventory_slots: 100,
                    ..CharacterRecord::default()
                }],
                inventories: vec![CharacterInventory {
                    char_id: character.char_id as i32,
                    items,
                }],
                ..SeedData::default()
            },
            false,
        )
        .unwrap();
    if arrow {
        repository
            .database
            .items
            .transaction(|tree| {
                for id in [1901_i32, 1750] {
                    database::tx_write(tree, &id.to_be_bytes(), GlobalConfigService::instance().get_item(id))?;
                }
                Ok(())
            })
            .unwrap();
    }
    let context = before_each_with_repository(0, repository.clone(), no_delay);
    if arrow {
        character.add_items(
            context
                .runtime()
                .block_on(repository.character_inventory_fetch(character.char_id as i32))
                .unwrap(),
        );
        character.wear_equip_item(
            0,
            EquipmentLocation::HandRight.as_flag(),
            GlobalConfigService::instance().get_item(1901),
        );
        character.wear_equip_item(
            1,
            EquipmentLocation::Ammo.as_flag(),
            GlobalConfigService::instance().get_item(1750),
        );
    }
    let instance = Arc::new(create_empty_map_instance(
        context.client_notification_sender.clone(),
        Arc::new(TasksQueue::new()),
    ));
    context
        .server
        .state_mut()
        .map_instances_mut()
        .insert("empty".into(), vec![instance]);
    let mut flags = crate::server::model::map_flags::MapFlags::default();
    flags.set(crate::server::model::map_flags::MapFlag::Pvp, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    let mut target = create_character();
    target.char_id += 1;
    target.map_instance_key = character.map_instance_key.clone();
    target.x = 51;
    target.y = 50;
    target.status.hp = 1000;
    target.status.sp = 1000;
    context.server.state_mut().insert_character(target);
    (context, repository, character)
}

fn start(context: &ServerServiceTestContext, character: &mut Character, skill: SkillEnum, level: u8, tick: u128) {
    let target_id = character.char_id + 1;
    context.server.server_service().character_start_use_skill(
        &context.server,
        context.server.state(),
        character,
        CharacterUseSkill {
            char_id: character.char_id,
            target_id,
            skill_id: skill.id(),
            skill_level: level,
        },
        tick,
    );
}

fn stored_sp(repository: &SledRepository, character: &Character) -> i32 {
    database::required::<CharacterRecord>(&repository.database.characters, &character.char_id.to_be_bytes())
        .unwrap()
        .sp
}

#[test]
fn water_ball_consumes_wet_cells_and_hits_sequentially_without_repaying_cost() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    let (context, repository, mut character) = fixture(false, true);
    let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
    instance
        .state_mut()
        .cells_mut()
        .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    for x in 49..=51 {
        for y in 49..=51 {
            instance.state_mut().cells_mut()[y * 100 + x] |= CellType::Water.as_flag();
        }
    }
    let cells = context
        .server
        .script_skill_service()
        .water_ball_cells(context.server.state(), &character, 3, 0, false)
        .unwrap();
    assert_eq!(cells.len(), 9);
    let target = character.char_id + 1;
    start(&context, &mut character, SkillEnum::WzWaterball, 3, 0);
    assert_eq!(stored_sp(&repository, &character), 980);
    let char_id = character.char_id;
    context.server.state_mut().insert_character(character);
    for tick in (40..=120).step_by(40) {
        crate::server::Server::game_loop_iteration(&context.server, tick);
    }
    assert_eq!(context.server.state().get_character(target).unwrap().status.hp, 1000);
    crate::server::Server::game_loop_iteration(&context.server, 160);
    assert_eq!(context.server.state().get_character(target).unwrap().status.hp, 1000);
    crate::server::Server::game_loop_iteration(&context.server, 200);
    let first_hp = context.server.state().get_character(target).unwrap().status.hp;
    assert!(first_hp < 1000);
    for tick in (240..=1440).step_by(40) {
        crate::server::Server::game_loop_iteration(&context.server, tick);
    }
    assert!(context.server.state().get_character(target).unwrap().status.hp < first_hp);
    let character = context.server.state().get_character(char_id).unwrap();
    assert_eq!(stored_sp(&repository, character), 980);
}

#[test]
fn earthquake_uses_attack_and_hits_three_split_waves() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;

    use crate::server::model::events::map_event::MapEvent;
    let (context, _, mut character) = fixture(false, true);
    character.status.str = 30;
    let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
    instance
        .state_mut()
        .cells_mut()
        .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    for id in [111, 112] {
        let mut mob = crate::tests::common::mob_helper::create_mob(id, "PORING");
        mob.x = 50 + (id - 110) as u16;
        mob.y = 50;
        mob.status.set_hp(1000);
        mob.status.set_max_hp(1000);
        mob.status.set_mdef(100);
        instance.state_mut().mobs_mut().insert(id, mob);
    }
    context
        .server
        .script_skill_service()
        .place_ground_skill_depth(
            &context.server,
            context.server.state(),
            &mut character,
            SkillEnum::NpcEarthquake.id(),
            1,
            50,
            50,
            0,
            true,
            0,
        )
        .unwrap();
    context.server.state_mut().insert_character(character);
    let mut hits = vec![];
    for tick in [0, 99, 100, 399, 400, 699, 700, 900] {
        context
            .server
            .script_skill_service()
            .tick_ground_skills(&context.server, context.server.state(), tick);
        let events = instance.task_queue().pop().unwrap_or_default();
        let damage = events
            .into_iter()
            .filter_map(|event| {
                if let MapEvent::MobDamage(damage) = event {
                    Some(damage)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(damage.len(), if [100, 400, 700].contains(&tick) { 2 } else { 0 });
        hits.extend(damage);
    }
    assert_eq!(hits.len(), 6);
    assert!(
        hits.iter()
            .all(|damage| damage.damage > 20 && damage.magic_context.unwrap().matk > 20)
    );
}

#[test]
fn grand_cross_produces_three_live_self_damage_pulses() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    let (context, _, mut character) = fixture(false, true);
    character.status.str = 30;
    character.status.int = 30;
    let char_id = character.char_id;
    let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
    instance
        .state_mut()
        .cells_mut()
        .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    context
        .server
        .script_skill_service()
        .place_ground_skill_depth(
            &context.server,
            context.server.state(),
            &mut character,
            SkillEnum::CrGrandcross.id(),
            1,
            50,
            50,
            0,
            true,
            0,
        )
        .unwrap();
    context.server.state_mut().insert_character(character);
    let mut hp = vec![];
    for tick in (40..=1040).step_by(40) {
        crate::server::Server::game_loop_iteration(&context.server, tick);
        hp.push(context.server.state().get_character(char_id).unwrap().status.hp);
    }
    assert_eq!(hp.windows(2).filter(|pair| pair[0] > pair[1]).count(), 3);
    assert!(hp.last().copied().unwrap() < 1000);
}

#[test]
fn native_skill_payment_occurs_once_at_cast_completion() {
    let (context, repository, mut character) = fixture(false, false);
    start(&context, &mut character, SkillEnum::MgFirebolt, 5, 100);
    assert!(character.is_using_skill());
    let finish = character.skill_in_use().start_skill_tick + character.skill_in_use().skill.cast_time() as u128;
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (1000, 1000));
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), finish - 1, &mut character);
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (1000, 1000));
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), finish, &mut character);
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (980, 980));
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), finish + 1, &mut character);
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (980, 980));
}

#[test]
fn area_and_utility_skills_pay_once_when_the_queued_effect_completes() {
    use models::status::KnownSkill;
    for (skill, cost) in [
        (SkillEnum::BaFrostjoker, 12),
        (SkillEnum::BsGreed, 10),
        (SkillEnum::WzEstimation, 10),
    ] {
        let (context, repository, mut character) = fixture(false, false);
        let id = character.char_id;
        character.status.known_skills.push(KnownSkill { value: skill, level: 1 });
        let target = if skill == SkillEnum::WzEstimation {
            let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
            let mut mob = crate::tests::common::mob_helper::create_mob(111, "PORING");
            mob.x = 51;
            mob.y = 50;
            instance.state_mut().mobs_mut().insert(111, mob);
            111
        } else {
            id
        };
        context.server.state_mut().insert_character(character);
        context
            .server
            .handle_character_skill(
                context.server.state_mut().as_mut(),
                CharacterUseSkill {
                    char_id: id,
                    target_id: target,
                    skill_id: skill.id(),
                    skill_level: 1,
                },
                0,
            )
            .unwrap();
        assert_eq!(
            stored_sp(&repository, context.server.state().get_character(id).unwrap()),
            1000,
            "{skill:?} charged before completion"
        );
        crate::server::Server::game_loop_iteration(&context.server, 40);
        let character = context.server.state().get_character(id).unwrap();
        assert_eq!(
            (character.status.sp, stored_sp(&repository, character)),
            (1000 - cost as u32, 1000 - cost),
            "{skill:?} did not commit its cost"
        );
        crate::server::Server::game_loop_iteration(&context.server, 80);
        assert_eq!(
            stored_sp(&repository, context.server.state().get_character(id).unwrap()),
            1000 - cost,
            "{skill:?} paid more than once"
        );
    }
}

#[test]
fn interrupted_or_dead_target_casts_leave_resources_unchanged() {
    let (context, repository, mut character) = fixture(false, false);
    start(&context, &mut character, SkillEnum::MgFirebolt, 5, 100);
    character.clear_skill_in_use();
    context.server.script_skill_service().cancel_queued_cast(&mut character);
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), 10000, &mut character);
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (1000, 1000));
    start(&context, &mut character, SkillEnum::MgFirebolt, 5, 10000);
    context
        .server
        .state_mut()
        .characters_mut()
        .get_mut(&(character.char_id + 1))
        .unwrap()
        .status
        .hp = 0;
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), 20000, &mut character);
    assert!(!character.is_using_skill());
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (1000, 1000));
}

#[test]
fn palm_strike_defers_the_hit_until_after_one_second_and_attack_motion() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use crate::server::service::status_effect_service::StatusEffectService;
    use crate::server::service::status_service::StatusService;
    let (context, repository, mut character) = fixture(false, true);
    character.status.str = 30;
    StatusEffectService::apply_status(
        &mut character.status,
        StatusChangeRequest::guaranteed(StatusChangeKind::ExplosionSpirits, 60000, 1),
        0,
        0,
    )
    .unwrap();
    let target_id = character.char_id + 1;
    StatusEffectService::apply_status(
        &mut context.server.state_mut().characters_mut().get_mut(&target_id).unwrap().status,
        StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 10000, 1),
        0,
        0,
    )
    .unwrap();
    let delay = 1000 + u128::from(StatusService::instance().attack_motion(&StatusService::instance().to_snapshot(&character.status)));
    start(&context, &mut character, SkillEnum::ChPalmstrike, 1, 0);
    assert_eq!(stored_sp(&repository, &character), 998);
    context.server.state_mut().insert_character(character);
    let execution = delay.div_ceil(40) * 40;
    for tick in (40..=execution).step_by(40) {
        crate::server::Server::game_loop_iteration(&context.server, tick);
        assert_eq!(context.server.state().get_character(target_id).unwrap().status.hp, 1000);
    }
    crate::server::Server::game_loop_iteration(&context.server, execution + 40);
    assert!(context.server.state().get_character(target_id).unwrap().status.hp < 1000);
    assert_eq!(
        stored_sp(&repository, context.server.state().get_character(target_id - 1).unwrap()),
        998
    );
}

#[test]
fn final_strike_leaves_one_hp_removes_nen_and_slides_past_the_target() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use crate::server::service::status_effect_service::StatusEffectService;
    let (context, repository, mut character) = fixture(false, true);
    let id = character.char_id;
    let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    instance
        .state_mut()
        .cells_mut()
        .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    StatusEffectService::apply_status(
        &mut character.status,
        StatusChangeRequest::guaranteed(StatusChangeKind::Nen, 60000, 5),
        0,
        0,
    )
    .unwrap();
    start(&context, &mut character, SkillEnum::NjIssen, 1, 0);
    assert_eq!(stored_sp(&repository, &character), 945);
    context.server.state_mut().insert_character(character);
    crate::server::Server::game_loop_iteration(&context.server, 40);
    let character = context.server.state().get_character(id).unwrap();
    assert_eq!(character.status.hp, 1);
    assert!(!character.status.has_status_change(StatusChangeKind::Nen));
    assert_eq!((character.x, character.y), (53, 50));
}

#[test]
fn back_stab_rejects_the_front_without_payment_then_turns_a_rear_target() {
    let (context, repository, mut character) = fixture(false, true);
    let id = character.char_id;
    let target_id = id + 1;
    context.server.state_mut().characters_mut().get_mut(&target_id).unwrap().dir = 2;
    start(&context, &mut character, SkillEnum::RgBackstap, 1, 0);
    assert_eq!(stored_sp(&repository, &character), 1000);
    assert!(!character.is_using_skill());
    context.server.state_mut().characters_mut().get_mut(&target_id).unwrap().dir = 6;
    start(&context, &mut character, SkillEnum::RgBackstap, 1, 0);
    assert_eq!(stored_sp(&repository, &character), 984);
    context.server.state_mut().insert_character(character);
    crate::server::Server::game_loop_iteration(&context.server, 40);
    assert_eq!(context.server.state().get_character(target_id).unwrap().dir, 2);
}

#[test]
fn intimidate_warps_after_eight_hundred_milliseconds_and_only_brings_a_nearby_victim() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use crate::server::service::status_effect_service::StatusEffectService;
    for (victim_stays_nearby, instance_id) in [(true, 0), (false, 0), (true, 7)] {
        let (context, repository, mut character) = fixture(false, true);
        character.status.base_level = 99;
        character.status.str = 20;
        let id = character.char_id;
        let target_id = id + 1;
        if instance_id != 0 {
            let flags = context.server.state().map_flags_for("empty", 0);
            context.server.state_mut().runtime_map_flags.insert(("empty".into(), instance_id), flags);
            let instance = crate::server::model::map_instance::MapInstance::from_map(
                common::test_script_vm(),
                crate::tests::common::map_instance_helper::create_empty_map(),
                instance_id,
                vec![CellType::Walkable.as_flag() | CellType::Shootable.as_flag(); 100 * 100],
                context.client_notification_sender.clone(),
                crate::server::model::map_item::MapItems::new(0),
                Arc::new(TasksQueue::new()),
            );
            context
                .server
                .state_mut()
                .map_instances_mut()
                .insert("empty".into(), vec![Arc::new(instance)]);
            character.map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), instance_id);
            context
                .server
                .state_mut()
                .characters_mut()
                .get_mut(&target_id)
                .unwrap()
                .map_instance_key = character.map_instance_key.clone();
        }
        let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
        instance
            .state_mut()
            .cells_mut()
            .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
        StatusEffectService::apply_status(
            &mut context.server.state_mut().characters_mut().get_mut(&target_id).unwrap().status,
            StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 10000, 1),
            0,
            0,
        )
        .unwrap();
        start(&context, &mut character, SkillEnum::RgIntimidate, 5, 0);
        assert_eq!(stored_sp(&repository, &character), 975);
        context.server.state_mut().insert_character(character);
        crate::server::Server::game_loop_iteration(&context.server, 40);
        assert!(context.server.state().get_character(target_id).unwrap().status.hp < 1000);
        crate::server::Server::game_loop_iteration(&context.server, 80);
        if !victim_stays_nearby {
            context.server.state_mut().characters_mut().get_mut(&target_id).unwrap().x = 80;
        }
        instance.state_mut().cells_mut().fill(CellType::Shootable.as_flag());
        instance.state_mut().cells_mut()[10 * 100 + 10] |= CellType::Walkable.as_flag();
        for tick in (120..=880).step_by(40) {
            crate::server::Server::game_loop_iteration(&context.server, tick);
            let caster = context.server.state().get_character(id).unwrap();
            assert_eq!((caster.x, caster.y), (50, 50));
        }
        for tick in (920..=1040).step_by(40) {
            crate::server::Server::game_loop_iteration(&context.server, tick);
        }
        let caster = context.server.state().get_character(id).unwrap();
        let victim = context.server.state().get_character(target_id).unwrap();
        assert_eq!((caster.x, caster.y), (10, 10));
        assert_eq!((victim.x, victim.y), if victim_stays_nearby { (10, 10) } else { (80, 50) });
        assert_eq!(caster.current_map_instance(), instance_id);
        assert_eq!(victim.current_map_instance(), instance_id);
        assert_eq!(stored_sp(&repository, caster), 975);
    }
}

#[test]
fn expired_sphere_rolls_back_native_skill_payment() {
    let (context, repository, mut character) = fixture(false, false);
    character.script_skill_state.spirit_spheres.push(101);
    start(&context, &mut character, SkillEnum::MoInvestigate, 1, 100);
    assert!(character.is_using_skill());
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), 10000, &mut character);
    assert!(!character.is_using_skill());
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (1000, 1000));
}

#[test]
fn wide_soul_drain_applies_current_sp_loss_inside_its_radius_and_spares_friendly_summons() {
    use crate::server::model::events::map_event::MapEvent;
    use crate::server::service::map_instance_service::MapInstanceService;
    use crate::server::service::mob_service::MobService;
    let (context, _, mut character) = fixture(false, true);
    let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
    for (id, x, friendly) in [(111, 63, false), (112, 64, false), (113, 51, true)] {
        let mut mob = crate::tests::common::mob_helper::create_mob(id, "PORING");
        mob.x = x;
        mob.y = 50;
        mob.status.set_sp(23);
        mob.status.set_max_sp(100);
        mob.status_effects.sp = 23;
        mob.status_effects.max_sp = 100;
        mob.summoned = friendly;
        mob.summon_ai = u16::from(friendly);
        instance.state_mut().mobs_mut().insert(id, mob);
    }
    let service = MapInstanceService::new(
        context.client_notification_sender.clone(),
        GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()),
        context.server_task_queue.clone(),
    );
    let caster_id = character.char_id;
    context
        .server
        .script_skill_service()
        .cast_skill(
            &context.server,
            context.server.state(),
            &mut character,
            680,
            6,
            caster_id,
            false,
            0,
            true,
        )
        .unwrap();
    for event in instance.task_queue().pop().unwrap_or_default() {
        if let MapEvent::ScriptMobCombat {
            source_id,
            target_id,
            effect,
        } = event
        {
            service.script_mob_combat(
                instance.state_mut().as_mut(),
                source_id,
                target_id,
                effect,
                instance.task_queue(),
                40,
            );
        }
    }
    let state = instance.state();
    assert_eq!(state.get_mob(111).unwrap().status.sp(), 19);
    assert_eq!(state.get_mob(111).unwrap().status_effects.sp, 19);
    assert_eq!(state.get_mob(112).unwrap().status.sp(), 23);
    assert_eq!(state.get_mob(113).unwrap().status.sp(), 23);
}

#[test]
fn direct_area_and_ground_spells_preserve_elemental_absorption_through_mob_admission() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    use models::enums::element::Element;
    use models::enums::mob::MobRace;
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use crate::server::model::events::map_event::MapEvent;
    use crate::server::service::map_instance_service::MapInstanceService;
    use crate::server::service::mob_service::MobService;
    for (skill, armor_element, ground) in [
        (SkillEnum::MgColdbolt, Element::Water, false),
        (SkillEnum::MgFireball, Element::Fire, false),
        (SkillEnum::WzStormgust, Element::Water, true),
    ] {
        let (context, _, mut character) = fixture(false, true);
        character.status.int = 50;
        let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
        instance
            .state_mut()
            .cells_mut()
            .fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
        let mut mob = crate::tests::common::mob_helper::create_mob(111, "PORING");
        mob.x = 51;
        mob.y = 50;
        mob.status.set_race(MobRace::Formless);
        mob.base_status.set_race(MobRace::Formless);
        mob.status.set_element(armor_element);
        mob.base_status.set_element(armor_element);
        mob.status.set_element_level(4);
        mob.base_status.set_element_level(4);
        mob.status.set_max_hp(1000);
        mob.base_status.set_max_hp(1000);
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::NoRecovery, 60000, 1), 0, 0)
            .unwrap();
        mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Berserk, 60000, 1), 0, 0)
            .unwrap();
        mob.set_hp(200);
        instance.state_mut().mobs_mut().insert(111, mob);
        if ground {
            context
                .server
                .script_skill_service()
                .place_ground_skill_depth(
                    &context.server,
                    context.server.state(),
                    &mut character,
                    skill.id(),
                    1,
                    51,
                    50,
                    0,
                    true,
                    0,
                )
                .unwrap();
            context.server.state_mut().insert_character(character);
            context
                .server
                .script_skill_service()
                .tick_ground_skills(&context.server, context.server.state(), 100);
        } else {
            context
                .server
                .script_skill_service()
                .cast_skill(
                    &context.server,
                    context.server.state(),
                    &mut character,
                    skill.id(),
                    1,
                    111,
                    false,
                    0,
                    true,
                )
                .unwrap();
        }
        let damage = instance
            .task_queue()
            .pop()
            .unwrap_or_default()
            .into_iter()
            .find_map(|event| {
                if let MapEvent::MobDamage(damage) = event {
                    Some(damage)
                } else {
                    None
                }
            })
            .expect("Accepted spell did not queue its elemental outcome");
        assert_eq!(damage.damage, 0, "{skill:?} converted absorption into damage");
        assert!(damage.healing > 0, "{skill:?} discarded negative elemental damage");
        let maximum = instance.state().get_mob(111).unwrap().status.max_hp();
        let service = MapInstanceService::new(
            context.client_notification_sender.clone(),
            GlobalConfigService::instance(),
            MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()),
            context.server_task_queue.clone(),
        );
        service.mob_being_attacked(instance.state_mut().as_mut(), damage, instance.task_queue(), 100);
        let state = instance.state();
        let target = state.get_mob(111).unwrap();
        assert_eq!(target.hp(), 200_u32.saturating_add(damage.healing).min(maximum));
        assert_eq!(target.status_effects.hp, target.hp());
        assert!(target.status.has_status_change(StatusChangeKind::NoRecovery));
        assert!(target.status.has_status_change(StatusChangeKind::Berserk));
        assert!(target.damages.is_empty());
        assert!(context.server_task_queue.is_empty());
    }
}

#[test]
fn dragon_fear_tries_the_next_ailment_after_an_existing_status_rejects_the_first() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};

    use crate::server::model::events::map_event::MobStatusAlternatives;
    use crate::server::service::map_instance_service::MapInstanceService;
    use crate::server::service::mob_service::MobService;
    let (context, _, character) = fixture(false, true);
    let instance = context.server.state().get_map_instance_from_character(&character).unwrap();
    let mut mob = crate::tests::common::mob_helper::create_mob(111, "PORING");
    mob.base_status.set_base_vit(0);
    mob.status.set_base_vit(0);
    mob.base_status.set_base_luk(0);
    mob.status.set_base_luk(0);
    mob.start_status(StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 10000, 1), 0, 0)
        .unwrap();
    instance.state_mut().mobs_mut().insert(111, mob);
    let skill_id = crate::server::script::skill::metadata::SkillMetadata::all()
        .iter()
        .find(|skill| skill.name == "NPC_DRAGONFEAR")
        .unwrap()
        .id;
    let requests = crate::server::script::skill::ScriptSkillService::dragon_fear_requests(skill_id, 1, character.char_id, 0);
    let service = MapInstanceService::new(
        context.client_notification_sender.clone(),
        GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()),
        context.server_task_queue.clone(),
    );
    service.start_mob_status_alternatives(
        instance.state_mut().as_mut(),
        MobStatusAlternatives { mob_id: 111, requests },
        40,
    );
    let state = instance.state();
    let target = state.get_mob(111).unwrap();
    assert!(target.status.has_status_change(StatusChangeKind::Stun));
    let silence = target.status.status_change(StatusChangeKind::Silence).unwrap();
    assert_eq!(silence.values[1], character.char_id as i32);
    assert!(!target.status.has_status_change(StatusChangeKind::Confusion));
    assert!(!target.status.has_status_change(StatusChangeKind::Bleeding));
}

#[test]
fn missing_ammunition_rolls_back_native_skill_payment() {
    let (context, repository, mut character) = fixture(true, false);
    start(&context, &mut character, SkillEnum::CgArrowvulcan, 1, 100);
    assert!(character.is_using_skill());
    assert_eq!(character.status.sp, 1000);
    character.inventory[1] = None;
    repository
        .database
        .inventories
        .transaction(|tree| {
            let mut items: Vec<InventoryRecord> = database::tx_required(tree, &character.char_id.to_be_bytes())?;
            items.retain(|item| item.id != 11);
            database::tx_write(tree, &character.char_id.to_be_bytes(), &items)?;
            Ok(())
        })
        .unwrap();
    context
        .server
        .server_service()
        .character_use_skill(&context.server, context.server.state(), 10000, &mut character);
    assert!(!character.is_using_skill());
    assert_eq!((character.status.sp, stored_sp(&repository, &character)), (1000, 1000));
    let records: Vec<InventoryRecord> = database::required(&repository.database.inventories, &character.char_id.to_be_bytes()).unwrap();
    assert_eq!(records.len(), 1);
}

#[test]
fn support_lex_divina_pays_once_then_delays_silence_and_cures_existing_silence() {
    use models::status_change::StatusChangeKind;
    let (context, repository, mut character) = fixture(false, true);
    let source_id = character.char_id;
    let target_id = source_id + 1;
    {
        let mut state = context.server.state_mut();
        let target = state.characters_mut().get_mut(&target_id).unwrap();
        target.status.vit = 0;
        target.status.luk = 0;
    }
    context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut character, SkillEnum::PrLexdivina.id(), 1, target_id, true, 0, false,
    ).unwrap();
    let completed_at = character.script_skill_state.casting_until;
    assert_eq!(stored_sp(&repository, &character), 1000);
    context.server.state_mut().insert_character(character);
    for tick in (40..completed_at).step_by(40) {
        crate::server::Server::game_loop_iteration(&context.server, tick);
    }
    assert_eq!(stored_sp(&repository, context.server.state().get_character(source_id).unwrap()), 1000);
    crate::server::Server::game_loop_iteration(&context.server, completed_at);
    let paid_sp = stored_sp(&repository, context.server.state().get_character(source_id).unwrap());
    assert!(paid_sp < 1000);
    for tick in ((completed_at + 40)..=(completed_at + 960)).step_by(40) {
        crate::server::Server::game_loop_iteration(&context.server, tick);
        assert!(!context.server.state().get_character(target_id).unwrap().status.has_status_change(StatusChangeKind::Silence));
    }
    crate::server::Server::game_loop_iteration(&context.server, completed_at + 1000);
    assert!(context.server.state().get_character(target_id).unwrap().status.has_status_change(StatusChangeKind::Silence));
    assert_eq!(stored_sp(&repository, context.server.state().get_character(source_id).unwrap()), paid_sp);
    let mut source = context.server.state_mut().characters_mut().remove(&source_id).unwrap();
    context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut source, SkillEnum::PrLexdivina.id(), 1, target_id, false, completed_at + 1000, true,
    ).unwrap();
    context.server.state_mut().insert_character(source);
    crate::server::Server::game_loop_iteration(&context.server, completed_at + 1040);
    assert!(!context.server.state().get_character(target_id).unwrap().status.has_status_change(StatusChangeKind::Silence));
}

#[test]
fn support_endows_reject_unarmed_targets_and_failed_replacement_unequips_without_breaking() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};
    use crate::server::service::status_effect_service::StatusEffectService;
    let (context, _, mut character) = fixture(true, true);
    let target_id = character.char_id + 1;
    assert!(context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut character, SkillEnum::SaFlamelauncher.id(), 1, target_id, false, 0, true,
    ).is_err());
    StatusEffectService::start(&context.server, &mut character,
        StatusChangeRequest::guaranteed(StatusChangeKind::FireWeapon, 60000, 5), 0, &context.client_notification_sender,
    ).unwrap();
    let source_id = character.char_id;
    context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut character, SkillEnum::SaFlamelauncher.id(), 1, source_id, false, 0, true,
    ).unwrap();
    assert!(character.status.right_hand_weapon().is_none());
    let weapon = character.get_item_from_inventory(0).unwrap();
    assert_eq!(weapon.equip, 0);
    assert!(!weapon.is_damaged);
    assert_eq!(character.status.status_change(StatusChangeKind::FireWeapon).unwrap().values[0], 5);
}

#[test]
fn support_provoke_interrupts_cancelable_casts_and_respects_cast_protection() {
    use models::enums::bonus::BonusType;
    use models::status_bonus::TemporaryStatusBonus;
    use models::status_change::StatusChangeKind;
    for protected in [false, true] {
        let (context, _, mut source) = fixture(false, true);
        source.status.base_level = 99;
        let source_id = source.char_id;
        context.server.state_mut().insert_character(source);
        let mut target = context.server.state_mut().characters_mut().remove(&(source_id + 1)).unwrap();
        target.script_skill_state.casting_until = 10000;
        target.script_skill_state.casting_skill_id = SkillEnum::MgFirebolt.id();
        target.script_skill_state.casting_skill_level = 1;
        if protected { target.status.temporary_bonuses.add(TemporaryStatusBonus::with_passive_skill(BonusType::EnableNoCancelCast, 0, 0)); }
        let effect = crate::server::script::skill::ScriptSkillEffect {
            source_char_id: source_id, target_id: target.char_id, skill_id: SkillEnum::SmProvoke.id(), level: 10,
            heal_value: 0, proc_depth: 0, skill_event_emitted: true, cast_generation: 0,
            action: crate::server::script::skill::ScriptSkillAction::Cast,
            deferred_requirements: None, prepared_outcome: None, source_index: None, source_item: None,
        };
        context.server.script_skill_service().apply_target_effect(&context.server, context.server.state(), &mut target, &effect, 100).unwrap();
        assert!(target.status.has_status_change(StatusChangeKind::Provoke));
        assert_eq!(target.script_skill_state.casting_until, if protected { 10000 } else { 0 });
    }
}

#[test]
fn support_magnum_requires_surviving_hp_but_only_deducts_sp_at_completion() {
    let (context, repository, mut character) = fixture(false, true);
    character.status.hp = 20;
    assert!(context.server.script_skill_service().requirements_plan(&character, SkillEnum::SmMagnum.id(), 1, 0).is_err());
    character.status.hp = 21;
    let plan = context.server.script_skill_service().requirements_plan(&character, SkillEnum::SmMagnum.id(), 1, 0).unwrap();
    assert_eq!((plan.minimum_hp, plan.hp), (21, 0));
    assert_eq!(plan.sp, 30);
    character.status.hp = 20;
    assert!(context.server.item_service().pay_requirement_plan(&context.server, &mut character, &plan, None, 0).is_err());
    assert_eq!(stored_sp(&repository, &character), 1000);
    character.status.hp = 21;
    character.status.sp = plan.sp;
    context.server.item_service().pay_requirement_plan(&context.server, &mut character, &plan, None, 0).unwrap();
    assert_eq!((character.status.hp, character.status.sp), (21, 0));
    assert_eq!(stored_sp(&repository, &character), 0);
}

#[test]
fn support_party_buffs_reach_only_living_same_map_members_in_range_and_pay_once() {
    use models::status_change::StatusChangeKind;
    let (context, repository, mut source) = fixture(false, true);
    let source_id = source.char_id;
    source.game_systems.party_id = 77;
    {
        let mut state = context.server.state_mut();
        state.characters_mut().get_mut(&(source_id + 1)).unwrap().game_systems.party_id = 77;
        for (offset, party, x, dead, instance) in [(2, 77, 65, false, 0), (3, 88, 51, false, 0), (4, 77, 51, true, 0), (5, 77, 51, false, 1)] {
            let mut target = create_character();
            target.char_id = source_id + offset;
            target.map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), instance);
            target.x = x;
            target.y = 50;
            target.status.hp = if dead { 0 } else { 1000 };
            target.game_systems.party_id = party;
            state.insert_character(target);
        }
    }
    context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut source, SkillEnum::AlAngelus.id(), 10, source_id, true, 0, false,
    ).unwrap();
    let completed_at = source.script_skill_state.casting_until;
    context.server.state_mut().insert_character(source);
    for tick in (40..=completed_at + 80).step_by(40) { crate::server::Server::game_loop_iteration(&context.server, tick); }
    for id in [source_id, source_id + 1] {
        assert!(context.server.state().get_character(id).unwrap().status.has_status_change(StatusChangeKind::Angelus));
    }
    for offset in [2, 3, 4, 5] {
        assert!(!context.server.state().get_character(source_id + offset).unwrap().status.has_status_change(StatusChangeKind::Angelus));
    }
    let source = context.server.state().get_character(source_id).unwrap();
    assert_eq!(stored_sp(&repository, source), 950);
}

#[test]
fn support_map_rules_block_learned_skills_and_fly_wings_but_allow_butterfly_wings() {
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    let (context, repository, mut source) = fixture(false, true);
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Pvp, true, &[]).unwrap();
    flags.set(MapFlag::NoSkill, true, &[]).unwrap();
    flags.set(MapFlag::NoTeleport, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    start(&context, &mut source, SkillEnum::SmBash, 1, 0);
    assert!(!source.is_using_skill());
    assert_eq!(stored_sp(&repository, &source), 1000);
    let source_id = source.char_id;
    assert!(context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut source, SkillEnum::AlTeleport.id(), 1, source_id, false, 0, true,
    ).is_err());
    context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut source, SkillEnum::AlTeleport.id(), 3, source_id, false, 0, true,
    ).unwrap();
    assert!(context.server_task_queue.pop().unwrap_or_default().into_iter().any(|event| matches!(event, crate::server::model::events::game_event::GameEvent::ScriptWarp(_))));
}

#[test]
fn autocast_charges_hp_without_sp_or_reagents_and_delays_only_attack_and_hit_triggers() {
    use models::enums::bonus::BonusType;
    use models::status_bonus::{CombatTrigger, TemporaryStatusBonus};
    let (context, repository, mut source) = fixture(false, true);
    let source_id = source.char_id;
    source.status.temporary_bonuses.add(TemporaryStatusBonus::with_passive_skill(BonusType::SkillDelayIncDecPercentage(-50), 0, 0));
    context.server.state_mut().insert_character(source);
    for (trigger, expected_hp, expected_delay) in [(CombatTrigger::Attack, 985, 500), (CombatTrigger::Hit, 970, 500), (CombatTrigger::Skill, 955, 0)] {
        context.server.state_mut().characters_mut().get_mut(&source_id).unwrap().timing.set_canact_tick(0);
        context.server.script_skill_service().cast_equipment_proc(
            &context.server, context.server.state_mut().as_mut(), source_id, source_id, SkillEnum::AlIncagi.id(), 1, 0, 1, trigger,
        ).unwrap();
        let source = context.server.state().get_character(source_id).unwrap();
        assert_eq!((source.status.hp, source.status.sp), (expected_hp, 1000));
        assert_eq!(source.timing.get_canact_tick(), expected_delay);
        let stored: CharacterRecord = database::required(&repository.database.characters, &source_id.to_be_bytes()).unwrap();
        assert_eq!((stored.hp, stored.sp), (expected_hp as i32, 1000));
    }
}

#[test]
fn autocast_hp_cost_can_kill_before_a_support_effect_and_still_commits_atomically() {
    use models::status_bonus::CombatTrigger;
    use models::status_change::StatusChangeKind;
    let (context, repository, mut source) = fixture(false, true);
    source.status.hp = 10;
    let source_id = source.char_id;
    context.server.state_mut().insert_character(source);
    assert!(context.server.script_skill_service().cast_equipment_proc(
        &context.server, context.server.state_mut().as_mut(), source_id, source_id, SkillEnum::AlIncagi.id(), 1, 0, 1, CombatTrigger::Attack,
    ).is_err());
    let source = context.server.state().get_character(source_id).unwrap();
    assert_eq!(source.status.hp, 0);
    assert!(source.is_dead());
    assert!(!source.status.has_status_change(StatusChangeKind::IncreaseAgi));
    let stored: CharacterRecord = database::required(&repository.database.characters, &source_id.to_be_bytes()).unwrap();
    assert_eq!((stored.hp, stored.sp), (0, 1000));
}

#[test]
fn final_strike_keeps_damage_and_hp_penalty_in_gvg_without_sliding_the_caster() {
    use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU32};
    use models::enums::cell::CellType;
    use models::status_bonus::BattleFlag;
    use crate::server::model::events::map_event::MapEvent;
    use crate::server::model::map_flags::{MapFlag, MapFlags};
    let (context, _, mut source) = fixture(false, true);
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Gvg, true, &[]).unwrap();
    context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
    let instance = context.server.state().get_map_instance_from_character(&source).unwrap();
    instance.state_mut().cells_mut().fill(CellType::Walkable.as_flag() | CellType::Shootable.as_flag());
    let mut target = crate::tests::common::mob_helper::create_mob(111, "PORING");
    target.x = 51;
    target.y = 50;
    instance.state_mut().mobs_mut().insert(111, target);
    context.server.script_skill_service().cast_skill(
        &context.server, context.server.state(), &mut source, SkillEnum::NjIssen.id(), 1, 111, false, 0, true,
    ).unwrap();
    let damage = instance.task_queue().pop().unwrap_or_default().into_iter().find_map(|event| {
        if let MapEvent::MobDamage(damage) = event { Some(damage) } else { None }
    }).expect("Final Strike did not dispatch its damage");
    assert_ne!(damage.battle_flags & BattleFlag::Weapon.as_flag(), 0);
    let source_id = source.char_id;
    context.server.state_mut().insert_character(source);
    crate::server::Server::game_loop_iteration(&context.server, 40);
    let source = context.server.state().get_character(source_id).unwrap();
    assert_eq!((source.x, source.y, source.status.hp), (50, 50, 1));
}

#[test]
fn autocast_tarot_charges_raw_sp_only_for_the_prepared_successful_draw() {
    use models::enums::bonus::BonusType;
    use models::status_bonus::{CombatTrigger, TemporaryStatusBonus};
    for succeeded in [false, true] {
        let (context, repository, mut source) = fixture(false, true);
        let source_id = source.char_id;
        source.status.temporary_bonuses.add(TemporaryStatusBonus::with_passive_skill(BonusType::SpConsumption(-90), 0, 0));
        let instance = context.server.state().get_map_instance_from_character(&source).unwrap();
        let mut target = crate::tests::common::mob_helper::create_mob(111, "PORING");
        target.x = 51;
        target.y = 50;
        instance.state_mut().mobs_mut().insert(111, target);
        context.server.state_mut().insert_character(source);
        let seed = (0..1000).find(|seed| {
            fastrand::seed(*seed);
            (fastrand::u8(0..100) <= 40) == succeeded
        }).unwrap();
        fastrand::seed(seed);
        let result = context.server.script_skill_service().cast_equipment_proc(
            &context.server, context.server.state_mut().as_mut(), source_id, 111, SkillEnum::CgTarotcard.id(), 5, 0, 1, CombatTrigger::Skill,
        );
        assert_eq!(result.is_ok(), succeeded);
        let source = context.server.state().get_character(source_id).unwrap();
        assert_eq!(stored_sp(&repository, source), if succeeded { 960 } else { 1000 });
        assert_eq!(source.timing.get_canact_tick(), 0);
    }
}

#[test]
fn teleport_pays_at_menu_opening_and_cancel_does_not_refund_or_repay() {
    use models::status::KnownSkill;
    use crate::server::model::events::game_event::ScriptTeleportSelection;
    for (level, remaining_sp) in [(1, 990), (2, 991)] {
        let (context, repository, mut character) = fixture(false, false);
        let char_id = character.char_id;
        character.status.known_skills.push(KnownSkill { value: SkillEnum::AlTeleport, level });
        character.save_map = "empty.gat".into();
        context.server.state_mut().insert_character(character);
        context.server.handle_character_skill(context.server.state_mut().as_mut(), CharacterUseSkill {
            char_id, target_id: char_id, skill_id: SkillEnum::AlTeleport.id(), skill_level: level,
        }, 0).unwrap();
        let source = context.server.state().get_character(char_id).unwrap();
        assert_eq!(stored_sp(&repository, source), 1000);
        assert!(source.script_skill_state.pending_teleport.is_none());
        crate::server::Server::game_loop_iteration(&context.server, 40);
        let source = context.server.state().get_character(char_id).unwrap();
        assert_eq!(stored_sp(&repository, source), remaining_sp);
        assert_eq!(source.script_skill_state.pending_teleport.as_ref().unwrap().level, level);
        let mut source = context.server.state_mut().characters_mut().remove(&char_id).unwrap();
        let selection = ScriptTeleportSelection { char_id, skill_id: SkillEnum::AlTeleport.id(), map: "cancel".into(), session: None };
        let selection_tick = if level == 1 { 80 } else { 120040 };
        context.server.script_skill_service().finish_teleport_menu(&context.server, context.server.state(), &mut source, &selection, selection_tick).unwrap();
        assert!(source.script_skill_state.pending_teleport.is_none());
        assert_eq!((source.x, source.y), (50, 50));
        assert_eq!(stored_sp(&repository, &source), remaining_sp);
        assert!(context.server.script_skill_service().finish_teleport_menu(&context.server, context.server.state(), &mut source, &selection, 120).is_err());
        assert_eq!(stored_sp(&repository, &source), remaining_sp);
    }
}

#[test]
fn teleport_selection_warps_once_without_repayment_and_preserves_random_instance() {
    use models::enums::EnumWithMaskValueU16;
    use models::enums::cell::CellType;
    use crate::server::model::events::game_event::ScriptTeleportSelection;
    use crate::server::model::map_instance::MapInstanceKey;
    for (level, instance_id, destination, expected_sp) in [(1, 7, "Random.gat", 990), (2, 0, "empty.gat", 991)] {
        let (context, repository, mut character) = fixture(false, false);
        if instance_id != 0 {
            let instance = crate::server::model::map_instance::MapInstance::from_map(
                common::test_script_vm(), crate::tests::common::map_instance_helper::create_empty_map(), instance_id,
                vec![CellType::Walkable.as_flag() | CellType::Shootable.as_flag(); 100 * 100],
                context.client_notification_sender.clone(), crate::server::model::map_item::MapItems::new(0), Arc::new(TasksQueue::new()),
            );
            context.server.state_mut().map_instances_mut().insert("empty".into(), vec![Arc::new(instance)]);
            character.map_instance_key = MapInstanceKey::new("empty".into(), instance_id);
        }
        character.save_map = "empty.gat".into();
        character.save_x = 10;
        character.save_y = 10;
        let char_id = character.char_id;
        context.server.script_skill_service().start_native_teleport_menu(&context.server, context.server.state(), &mut character, level, 0).unwrap();
        context.server.state_mut().insert_character(character);
        crate::server::Server::game_loop_iteration(&context.server, 40);
        let mut source = context.server.state_mut().characters_mut().remove(&char_id).unwrap();
        let instance = context.server.state().get_map_instance_from_character(&source).unwrap();
        instance.state_mut().cells_mut().fill(CellType::Shootable.as_flag());
        instance.state_mut().cells_mut()[10 * 100 + 10] |= CellType::Walkable.as_flag();
        let selection = ScriptTeleportSelection { char_id, skill_id: SkillEnum::AlTeleport.id(), map: destination.into(), session: None };
        context.server.script_skill_service().finish_teleport_menu(&context.server, context.server.state(), &mut source, &selection, 80).unwrap();
        assert!(source.script_skill_state.pending_teleport.is_none());
        assert!(context.server.script_skill_service().finish_teleport_menu(&context.server, context.server.state(), &mut source, &selection, 80).is_err());
        context.server.state_mut().insert_character(source);
        for tick in (80..=240).step_by(40) { crate::server::Server::game_loop_iteration(&context.server, tick); }
        let source = context.server.state().get_character(char_id).unwrap();
        assert_eq!((source.x, source.y, source.current_map_instance()), (10, 10, instance_id));
        assert_eq!(stored_sp(&repository, source), expected_sp);
    }
}

#[test]
fn teleport_rejects_cleared_dead_changed_map_and_unoffered_selections_without_repayment() {
    use crate::server::model::events::game_event::ScriptTeleportSelection;
    use crate::server::model::map_flags::MapFlag;
    for rejection in ["cleared", "dead", "changed-map", "unoffered", "no-teleport"] {
        let (context, repository, mut character) = fixture(false, false);
        let char_id = character.char_id;
        context.server.script_skill_service().start_native_teleport_menu(&context.server, context.server.state(), &mut character, 1, 0).unwrap();
        context.server.state_mut().insert_character(character);
        crate::server::Server::game_loop_iteration(&context.server, 40);
        let mut source = context.server.state_mut().characters_mut().remove(&char_id).unwrap();
        let tick = 80;
        if rejection == "cleared" { source.script_skill_state.pending_teleport = None; }
        if rejection == "dead" { source.status.hp = 0; }
        if rejection == "changed-map" { source.map_instance_key = crate::server::model::map_instance::MapInstanceKey::new("empty".into(), 7); }
        if rejection == "no-teleport" {
            let mut flags = context.server.state().map_flags_for("empty", 0);
            flags.set(MapFlag::NoTeleport, true, &[]).unwrap();
            context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
        }
        let selection = ScriptTeleportSelection { char_id, skill_id: SkillEnum::AlTeleport.id(), map: if rejection == "unoffered" { "prontera" } else { "Random" }.into(), session: None };
        assert!(context.server.script_skill_service().finish_teleport_menu(&context.server, context.server.state(), &mut source, &selection, tick).is_err(), "{rejection}");
        assert!(source.script_skill_state.pending_teleport.is_none());
        assert_eq!(stored_sp(&repository, &source), 990);
        assert_eq!((source.x, source.y), (50, 50));
        assert!(!context.server_task_queue.content_as_str().contains("ScriptWarp"));
    }
}

#[test]
fn teleport_interrupted_or_blocked_before_menu_opening_does_not_charge() {
    use crate::server::model::map_flags::MapFlag;
    for rejection in ["interrupt", "dead", "no-teleport", "no-skill"] {
        let (context, repository, mut character) = fixture(false, false);
        let char_id = character.char_id;
        context.server.script_skill_service().start_native_teleport_menu(&context.server, context.server.state(), &mut character, 1, 0).unwrap();
        if rejection == "interrupt" { context.server.script_skill_service().cancel_queued_cast(&mut character); }
        if rejection == "dead" { character.status.hp = 0; }
        if matches!(rejection, "no-teleport" | "no-skill") {
            let mut flags = context.server.state().map_flags_for("empty", 0);
            flags.set(if rejection == "no-teleport" { MapFlag::NoTeleport } else { MapFlag::NoSkill }, true, &[]).unwrap();
            context.server.state_mut().runtime_map_flags.insert(("empty".into(), 0), flags);
        }
        context.server.state_mut().insert_character(character);
        crate::server::Server::game_loop_iteration(&context.server, 40);
        let source = context.server.state().get_character(char_id).unwrap();
        assert!(source.script_skill_state.pending_teleport.is_none());
        assert_eq!(stored_sp(&repository, source), 1000);
    }
}

#[test]
fn status_alternatives_stop_after_started_stun_and_fall_back_when_stun_cannot_start() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};
    use crate::server::service::status_effect_service::StatusEffectService;
    for stun_exists in [false, true] {
        let (context, _, mut target) = fixture(false, true);
        if stun_exists { StatusEffectService::start(&context.server, &mut target, StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 5000, 1), 0, &context.client_notification_sender).unwrap(); }
        let requests = vec![StatusChangeRequest::guaranteed(StatusChangeKind::Stun, 5000, 1), StatusChangeRequest::guaranteed(StatusChangeKind::Blind, 30000, 1)];
        assert!(StatusEffectService::start_alternatives(&context.server, &mut target, requests, 1, &context.client_notification_sender).unwrap());
        assert!(target.status.has_status_change(StatusChangeKind::Stun));
        assert_eq!(target.status.has_status_change(StatusChangeKind::Blind), stun_exists);
    }
}

#[test]
fn stone_fling_pays_one_stone_and_sp_once_and_kyrie_suppresses_damage_and_secondary_effects() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};
    use crate::server::service::status_effect_service::StatusEffectService;
    let (context, repository, mut source) = fixture(false, true);
    let stone = GlobalConfigService::instance().get_item_by_name("Stone");
    repository.database.items.transaction(|tree| database::tx_write(tree, &stone.id.to_be_bytes(), stone)).unwrap();
    repository.database.inventories.transaction(|tree| database::tx_write(tree, &source.char_id.to_be_bytes(), &vec![InventoryRecord {
        id: 21, item_id: stone.id, amount: 2, is_identified: true, ..Default::default()
    }])).unwrap();
    source.add_items(context.runtime().block_on(repository.character_inventory_fetch(source.char_id as i32)).unwrap());
    let source_id = source.char_id;
    let target_id = source_id + 1;
    {
        let mut state = context.server.state_mut();
        let target = state.characters_mut().get_mut(&target_id).unwrap();
        target.status.max_hp = 1000;
        StatusEffectService::apply_status(&mut target.status, StatusChangeRequest::guaranteed(StatusChangeKind::Kyrie, 60000, 10), 0, 0).unwrap();
    }
    start(&context, &mut source, SkillEnum::TfThrowstone, 1, 0);
    assert_eq!(stored_sp(&repository, &source), 998);
    let items: Vec<InventoryRecord> = database::required(&repository.database.inventories, &source_id.to_be_bytes()).unwrap();
    assert_eq!(items[0].amount, 1);
    context.server.state_mut().insert_character(source);
    for tick in (40..=120).step_by(40) { crate::server::Server::game_loop_iteration(&context.server, tick); }
    let target = context.server.state().get_character(target_id).unwrap();
    assert_eq!(target.status.hp, 1000);
    assert_eq!(target.status.status_change(StatusChangeKind::Kyrie).unwrap().values[1..3], [250, 9]);
    assert!(!target.status.has_status_change(StatusChangeKind::Stun));
    assert!(!target.status.has_status_change(StatusChangeKind::Blind));
    assert_eq!(stored_sp(&repository, context.server.state().get_character(source_id).unwrap()), 998);
}

#[test]
fn pressure_native_cast_deals_fixed_damage_and_drains_sp_once_without_consuming_lex() {
    use models::status_change::{StatusChangeKind, StatusChangeRequest};
    use crate::server::service::status_effect_service::StatusEffectService;
    let (context, repository, mut source) = fixture(false, true);
    let source_id = source.char_id;
    let target_id = source_id + 1;
    {
        let mut state = context.server.state_mut();
        let target = state.characters_mut().get_mut(&target_id).unwrap();
        StatusEffectService::apply_status(&mut target.status, StatusChangeRequest::guaranteed(StatusChangeKind::LexAeterna, 60000, 1), 0, 0).unwrap();
    }
    let plan = context.server.script_skill_service().requirements_plan(&source, SkillEnum::PaPressure.id(), 1, 0).unwrap();
    start(&context, &mut source, SkillEnum::PaPressure, 1, 0);
    assert_eq!(stored_sp(&repository, &source), (1000 - plan.sp) as i32);
    context.server.state_mut().insert_character(source);
    for tick in (40..=120).step_by(40) { crate::server::Server::game_loop_iteration(&context.server, tick); }
    let target = context.server.state().get_character(target_id).unwrap();
    assert_eq!((target.status.hp, target.status.sp), (200, 800));
    assert!(target.status.has_status_change(StatusChangeKind::LexAeterna));
    assert_eq!(stored_sp(&repository, context.server.state().get_character(source_id).unwrap()), (1000 - plan.sp) as i32);
}

#[test]
fn vulture_grants_extend_native_skill_range_before_atomic_payment() {
    use models::enums::bonus::BonusType;
    use models::status_bonus::TemporaryStatusBonus;
    for (distance, vulture, accepted) in [(9, 0, true), (10, 0, false), (19, 10, true), (20, 10, false)] {
        let (context, repository, mut source) = fixture(true, true);
        let bow = GlobalConfigService::instance().get_item_by_name("Bow");
        repository.database.items.transaction(|tree| database::tx_write(tree, &bow.id.to_be_bytes(), bow)).unwrap();
        repository.database.inventories.transaction(|tree| {
            let mut inventory: Vec<InventoryRecord> = database::tx_required(tree, &source.char_id.to_be_bytes())?;
            inventory.iter_mut().find(|item| item.id == 10).unwrap().item_id = bow.id;
            database::tx_write(tree, &source.char_id.to_be_bytes(), &inventory)
        }).unwrap();
        source.inventory.clear();
        source.add_items(context.runtime().block_on(repository.character_inventory_fetch(source.char_id as i32)).unwrap());
        source.status.weapons.clear();
        source.wear_equip_item(0, EquipmentLocation::HandRight.as_flag(), bow);
        let target_id = source.char_id + 1;
        context.server.state_mut().characters_mut().get_mut(&target_id).unwrap().x = source.x + distance;
        if vulture > 0 { source.status.temporary_bonuses.add(TemporaryStatusBonus::with_passive_skill(BonusType::EnableSkillId(SkillEnum::AcVulture.id(), vulture), 0, 0)); }
        let plan = context.server.script_skill_service().requirements_plan(&source, SkillEnum::AcDouble.id(), 1, 0).unwrap();
        start(&context, &mut source, SkillEnum::AcDouble, 1, 0);
        assert_eq!(stored_sp(&repository, &source), if accepted { (1000 - plan.sp) as i32 } else { 1000 }, "distance {distance}, Vulture {vulture}");
        let inventory: Vec<InventoryRecord> = database::required(&repository.database.inventories, &source.char_id.to_be_bytes()).unwrap();
        assert_eq!(inventory.iter().find(|item| item.id == 11).unwrap().amount, if accepted { 9 } else { 10 });
        assert_eq!(source.has_pending_skill(), !accepted);
    }
}
