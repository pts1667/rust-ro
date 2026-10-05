use std::sync::{Arc, Mutex};

use models::enums::cell::CellType;
use models::status_change::StatusChangeKind;
use models::enums::EnumWithMaskValueU16;
use script_sdk::{Function, Request, Value};
use tokio::sync::oneshot;

use super::ServerServiceTestContext;
use crate::server::model::events::map_event::MapEvent;
use crate::server::model::map_instance::MapInstance;
use crate::server::script::ScriptRequest;
use crate::server::service::map_instance_service::MapInstanceService;

const NPC_ID: u32 = super::actor_unit_skill_tests::NPC_ID;

fn call(context: &ServerServiceTestContext, function: Function, arguments: Vec<Value>) -> Result<Value, String> {
    call_as(context, 0, function, arguments)
}

fn call_as(context: &ServerServiceTestContext, char_id: u32, function: Function, arguments: Vec<Value>) -> Result<Value, String> {
    let (sender, mut receiver) = oneshot::channel();
    let request = ScriptRequest {
        char_id,
        account_id: 0,
        npc_id: NPC_ID,
        npc_entry: 0,
        npc_scope_instance: 0,
        map_instance: 0,
        generation: 0,
        background: true,
        event_depth: 0,
        timer_context: None,
        logout_token: None,
        request: Request::Call { function, arguments: arguments.clone() },
        response: Arc::new(Mutex::new(Some(sender))),
    };
    let reply = if function == Function::SpecialEffect {
        context.server.script_npc_effect(&context.server.state(), &request, &arguments)
    } else {
        context.server.script_map_call(&mut context.server.state_mut(), &request, function, &arguments)
    };
    drop(receiver.try_recv());
    reply
}

fn process_map(instance: &MapInstance, service: &MapInstanceService) {
    while let Some(events) = instance.task_queue().pop() {
        for event in events {
            match event {
                MapEvent::ScriptMapCommand(command) => service.script_map_command(&mut instance.state_mut(), command),
                MapEvent::ScriptSpawn(request) => {
                    service.script_spawn(&mut instance.state_mut(), request).unwrap();
                }
                MapEvent::ScriptMobCommand(command) => service.script_mob_command(&mut instance.state_mut(), command),
                _ => panic!("Unexpected map event {event:?}"),
            }
        }
    }
}

#[test]
fn disabled_npcs_vanish_from_the_map_and_return_when_enabled() {
    let (context, instance, service, _) = super::actor_unit_skill_tests::fixture();
    call(&context, Function::DisableNpc, vec!["Caster".into()]).unwrap();
    process_map(&instance, &service);
    assert!(instance.state().script_skill_state.npcs[&NPC_ID].hidden);
    assert!(instance.state().get_map_item(NPC_ID).is_none());
    call(&context, Function::EnableNpc, vec!["Caster".into()]).unwrap();
    process_map(&instance, &service);
    assert!(!instance.state().script_skill_state.npcs[&NPC_ID].hidden);
    assert!(instance.state().get_map_item(NPC_ID).is_some());
    assert!(call(&context, Function::DisableNpc, vec!["Missing".into()]).is_err());
}

#[test]
fn set_cell_rewrites_cell_flags_and_basilica_cells_grant_the_status_to_those_standing_on_them() {
    let (context, instance, service, target) = super::actor_unit_skill_tests::fixture();
    let char_id = target - 1;
    let (x, y) = {
        let state = context.server.state();
        let character = state.get_character(char_id).unwrap();
        (i32::from(character.x()), i32::from(character.y()))
    };
    let area = |cell: i32, enabled: i32| vec!["empty".into(), (x - 1).into(), (y - 1).into(), (x + 1).into(), (y + 1).into(), cell.into(), enabled.into()];
    call(&context, Function::SetCell, area(0, 0)).unwrap();
    process_map(&instance, &service);
    let index = instance.state().get_cell_index_of(x as u16, y as u16);
    assert_eq!(instance.state().cells()[index] & CellType::Walkable.as_flag(), 0);
    assert_ne!(instance.state().cells()[index] & CellType::Shootable.as_flag(), 0);
    call(&context, Function::SetCell, area(0, 1)).unwrap();
    call(&context, Function::SetCell, area(4, 1)).unwrap();
    process_map(&instance, &service);
    assert!(call(&context, Function::SetCell, area(3, 1)).is_err());

    let tick = crate::util::tick::get_tick();
    context.server.tick_cell_statuses(&mut context.server.state_mut(), tick);
    assert!(context.server.state().get_character(char_id).unwrap().status.has_status_change(StatusChangeKind::Basilica));

    call(&context, Function::SetCell, area(4, 0)).unwrap();
    process_map(&instance, &service);
    context.server.tick_cell_statuses(&mut context.server.state_mut(), tick + 1000);
    assert!(!context.server.state().get_character(char_id).unwrap().status.has_status_change(StatusChangeKind::Basilica));
}

#[test]
fn npc_special_effects_are_broadcast_from_the_npc() {
    let (context, _, _, _) = super::actor_unit_skill_tests::fixture();
    assert!(call_as(&context, 0, Function::SpecialEffect, vec![83.into()]).is_ok());
}

#[test]
fn bg_monster_returns_the_spawned_id_and_set_team_reassigns_it() {
    let (context, instance, service, _) = super::actor_unit_skill_tests::fixture();
    let spawned = call(&context, Function::BgMonster, vec![7.into(), "this".into(), 50.into(), 50.into(), "Guard".into(), 1002.into(), "".into()]).unwrap();
    let id = u32::try_from(spawned.number_value().unwrap()).unwrap();
    assert_ne!(id, 0);
    process_map(&instance, &service);
    assert_eq!(instance.state().get_mob(id).map(|mob| mob.bg_id), Some(7));
    call(&context, Function::BgMonsterSetTeam, vec![(id as i32).into(), 9.into()]).unwrap();
    process_map(&instance, &service);
    assert_eq!(instance.state().get_mob(id).map(|mob| mob.bg_id), Some(9));
}

mod alchemist_summons {
    use super::*;
    use models::enums::skill_enums::SkillEnum;

    fn cast(context: &ServerServiceTestContext, char_id: u32, skill: SkillEnum, level: u8) -> Result<(), String> {
        let mut character = context.server.state_mut().characters_mut().remove(&char_id).unwrap();
        let (x, y) = (character.x() + 1, character.y());
        let result = context.server.script_skill_service().place_ground_skill_depth(
            &context.server,
            &context.server.state(),
            &mut character,
            skill.id(),
            level,
            x,
            y,
            crate::util::tick::get_tick(),
            true,
            0,
        );
        context.server.state_mut().insert_character(character);
        result
    }

    fn spawn_pending(instance: &MapInstance, service: &MapInstanceService) {
        while let Some(events) = instance.task_queue().pop() {
            for event in events {
                if let MapEvent::ScriptSpawn(request) = event {
                    service.script_spawn(&mut instance.state_mut(), request).unwrap();
                }
            }
        }
    }

    #[test]
    fn marine_spheres_and_flora_are_owned_timed_and_limited_by_skill_level() {
        let (context, instance, service, target) = super::super::actor_unit_skill_tests::fixture();
        let char_id = target - 1;

        cast(&context, char_id, SkillEnum::AmSpheremine, 3).unwrap();
        spawn_pending(&instance, &service);
        let sphere = instance.state().mobs().values().find(|mob| mob.mob_id == 1142).map(|mob| (mob.summon_owner, mob.summon_ai, mob.status.max_hp(), mob.expires_at)).unwrap();
        assert_eq!((sphere.0, sphere.1, sphere.2), (Some(char_id), 2, 3200));
        assert!(sphere.3.is_some());

        cast(&context, char_id, SkillEnum::AmCannibalize, 5).unwrap();
        spawn_pending(&instance, &service);
        assert_eq!(instance.state().mobs().values().filter(|mob| mob.mob_id == 1590).count(), 1);
        assert!(cast(&context, char_id, SkillEnum::AmCannibalize, 5).is_err());

        let tick = crate::util::tick::get_tick() + 600_001;
        service.tick_mob_statuses(&mut instance.state_mut(), instance.task_queue().as_ref(), tick);
        assert!(instance.state().mobs().values().all(|mob| mob.summon_owner != Some(char_id) || !mob.is_present()));
    }
}
