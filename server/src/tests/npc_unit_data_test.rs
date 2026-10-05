use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use script_sdk::{Function, Reply, Request, Value};
use tokio::sync::oneshot;

use super::ServerServiceTestContext;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::{MapEvent, ReleaseScriptNpc};
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::MapItems;
use crate::server::model::session::Session;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::ScriptRequest;
use crate::server::script::skill::actor::NpcSkillState;
use crate::server::script::unit_data::NpcUnitDataField;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::tests::common;

const NPC_ID: u32 = 80_001;

fn fixture() -> (ServerServiceTestContext, Arc<MapInstance>, MapInstanceService, Arc<Session>) {
    let (context, instance, service, target) = super::actor_unit_skill_tests::fixture();
    let character_id = target - 1;
    let account_id = context.server.state().get_character(character_id).unwrap().account_id;
    let session = Arc::new(
        Session::create_empty(account_id, 42, 0, GlobalConfigService::instance().packetver()).recreate_with_character(character_id),
    );
    context
        .server
        .state()
        .sessions()
        .write()
        .unwrap()
        .insert(account_id, session.clone());
    let script = instance.get_script(NPC_ID).unwrap();
    instance
        .state_mut()
        .script_skill_state
        .npcs
        .insert(NPC_ID, NpcSkillState::uninitialized(&script));
    (context, instance, service, session)
}

fn begin(context: &ServerServiceTestContext, session: &Session, function: Function, arguments: Vec<Value>) -> oneshot::Receiver<Reply> {
    let (sender, receiver) = oneshot::channel();
    let request = ScriptRequest {
        char_id: session.char_id.unwrap(),
        account_id: session.account_id,
        npc_id: NPC_ID,
        npc_entry: 0,
        npc_scope_instance: 0,
        map_instance: 0,
        generation: session.script_generation.load(Ordering::Acquire),
        background: false,
        event_depth: 0,
        timer_context: None,
        logout_token: None,
        request: Request::Call { function, arguments },
        response: Arc::new(Mutex::new(Some(sender))),
    };
    context
        .server
        .script_service()
        .handle_request(&context.server, &mut context.server.state_mut(), request);
    receiver
}

fn process_map(instance: &MapInstance, service: &MapInstanceService) {
    while let Some(events) = instance.task_queue().pop() {
        for event in events {
            match event {
                MapEvent::UnitData(request) => service.unit_data(&mut instance.state_mut(), request),
                MapEvent::InstallScriptNpc(transfer) => service.install_script_npc(&mut instance.state_mut(), transfer),
                MapEvent::ReleaseScriptNpc(ReleaseScriptNpc { id }) => {
                    instance.state_mut().script_skill_state.transferring_npcs.remove(&id);
                }
                _ => panic!("Unexpected map event {event:?}"),
            }
        }
    }
}

fn process_transfers(context: &ServerServiceTestContext) {
    while let Some(events) = context.server_task_queue.pop() {
        for event in events {
            assert!(
                matches!(event, GameEvent::ScriptNpcTransfer(_)),
                "Unexpected game event {event:?}"
            );
            context
                .server
                .handle_script_event(&mut context.server.state_mut(), event, 1000)
                .unwrap();
        }
    }
}

fn write(
    context: &ServerServiceTestContext,
    session: &Session,
    instance: &MapInstance,
    service: &MapInstanceService,
    field: NpcUnitDataField,
    value: Value,
) -> Reply {
    let mut receiver = begin(context, session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        (field as i32).into(),
        value,
    ]);
    assert!(matches!(receiver.try_recv(), Err(oneshot::error::TryRecvError::Empty)));
    process_map(instance, service);
    receiver.try_recv().unwrap()
}

fn destination(
    context: &ServerServiceTestContext,
    origin: &MapInstance,
    name: &str,
    with_duplicate: bool,
    instance_id: u8,
) -> Arc<MapInstance> {
    let scripts = if with_duplicate {
        vec![origin.get_script(NPC_ID).unwrap().as_ref().clone()]
    } else {
        vec![]
    };
    let map = Box::leak(Box::new(Map::new(
        100,
        100,
        10000,
        name.into(),
        format!("{name}.gat"),
        vec![],
        vec![],
        scripts,
    )));
    let instance = Arc::new(MapInstance::from_map(
        common::test_script_vm(),
        map,
        instance_id,
        vec![CellType::Walkable.as_flag() | CellType::Shootable.as_flag(); 10000],
        context.client_notification_sender.clone(),
        MapItems::new(1000),
        Arc::new(TasksQueue::new()),
    ));
    context
        .server
        .state_mut()
        .map_instances_mut()
        .entry(name.into())
        .or_default()
        .push(instance.clone());
    instance
}

#[test]
fn unit_data_reads_reply_after_the_map_loop_and_preserve_uninitialized_npcs() {
    let (context, instance, service, session) = fixture();
    let before = instance.state().script_skill_state.npcs[&NPC_ID].clone();
    let mut receiver = begin(&context, &session, Function::GetUnitData, vec![(NPC_ID as i32).into()]);
    assert!(matches!(receiver.try_recv(), Err(oneshot::error::TryRecvError::Empty)));
    process_map(&instance, &service);
    let Value::Array(data) = receiver.try_recv().unwrap().unwrap() else {
        panic!("Expected unit data");
    };
    assert_eq!(data.len(), 46);
    assert_eq!(data[NpcUnitDataField::Hp as usize], 0.into());
    assert_eq!(data[NpcUnitDataField::MaxHp as usize], 0.into());
    assert_eq!(data[NpcUnitDataField::Hit as usize], 0.into());
    assert_eq!(data[NpcUnitDataField::X as usize], 50.into());
    assert_eq!(data[NpcUnitDataField::Class as usize], 46.into());
    assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID], before);
    for (name, field) in [
        ("UNPC_MAXHP", NpcUnitDataField::MaxHp),
        ("UNPC_PLUSALLSTAT", NpcUnitDataField::AllStats),
        ("UNPC_GROUP_ID", NpcUnitDataField::GroupId),
    ] {
        assert_eq!(
            crate::server::script::item_script_handler::constant(name).unwrap(),
            (field as i32).into()
        );
    }
}

#[test]
fn unit_data_writes_recalculate_npc_stats_and_preserve_explicit_combat_and_appearance_fields() {
    let (context, instance, service, session) = fixture();
    for (field, value) in [
        (NpcUnitDataField::Level, 70),
        (NpcUnitDataField::Int, 80),
        (NpcUnitDataField::Dex, 20),
        (NpcUnitDataField::AllStats, 5),
        (NpcUnitDataField::MaxHp, 500),
        (NpcUnitDataField::Hp, 250),
        (NpcUnitDataField::MatkMin, 123),
        (NpcUnitDataField::MatkMax, 456),
        (NpcUnitDataField::Mdef, 20),
        (NpcUnitDataField::PerfectDodge, 31),
        (NpcUnitDataField::Critical, 47),
        (NpcUnitDataField::Hair, 8),
        (NpcUnitDataField::HeadTop, 99),
        (NpcUnitDataField::Direction, 4),
    ] {
        assert_eq!(
            write(&context, &session, &instance, &service, field, value.into()).unwrap(),
            Value::default()
        );
    }
    let mut receiver = begin(&context, &session, Function::GetUnitData, vec![(NPC_ID as i32).into()]);
    process_map(&instance, &service);
    let Value::Array(data) = receiver.try_recv().unwrap().unwrap() else {
        panic!("Expected unit data");
    };
    for (field, value) in [
        (NpcUnitDataField::Int, 85),
        (NpcUnitDataField::Dex, 25),
        (NpcUnitDataField::Hit, 95),
        (NpcUnitDataField::Hp, 250),
        (NpcUnitDataField::MaxHp, 500),
        (NpcUnitDataField::MatkMin, 123),
        (NpcUnitDataField::MatkMax, 456),
        (NpcUnitDataField::PerfectDodge, 31),
        (NpcUnitDataField::Critical, 47),
        (NpcUnitDataField::Hair, 8),
        (NpcUnitDataField::HeadTop, 99),
        (NpcUnitDataField::Direction, 4),
    ] {
        assert_eq!(data[field as usize], value.into(), "{field:?}");
    }
    assert_eq!(instance.get_script(NPC_ID).unwrap().dir, 4);
    assert_eq!(
        *instance.state().script_skill_state.npcs[&NPC_ID].snapshot().combat_actor_kind(),
        models::enums::actor::CombatActorKind::Npc
    );
}

#[test]
fn rejected_unit_data_writes_leave_canonical_npc_state_unchanged() {
    let (context, instance, service, session) = fixture();
    let before = instance.state().script_skill_state.npcs[&NPC_ID].clone();
    for (field, value) in [
        (NpcUnitDataField::MaxHp, 0),
        (NpcUnitDataField::Int, -1),
        (NpcUnitDataField::Dex, 65536),
        (NpcUnitDataField::Direction, 8),
        (NpcUnitDataField::Element, 10),
        (NpcUnitDataField::ElementLevel, 0),
        (NpcUnitDataField::X, 100),
    ] {
        assert!(write(&context, &session, &instance, &service, field, value.into()).is_err());
        assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID], before);
    }
    let mut missing = begin(&context, &session, Function::GetUnitData, vec![(-1).into()]);
    assert_eq!(missing.try_recv().unwrap().unwrap(), (-1).into());
    let mut invalid_field = begin(&context, &session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        46.into(),
        1.into(),
    ]);
    assert!(invalid_field.try_recv().unwrap().is_err());
    assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID], before);
}

#[test]
fn cancelled_unit_data_calls_cannot_change_npc_state() {
    let (context, instance, service, session) = fixture();
    let before = instance.state().script_skill_state.npcs[&NPC_ID].clone();
    let mut receiver = begin(&context, &session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        (NpcUnitDataField::Int as i32).into(),
        50.into(),
    ]);
    session.script_generation.fetch_add(1, Ordering::AcqRel);
    process_map(&instance, &service);
    assert!(receiver.try_recv().unwrap().is_err());
    assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID], before);
    drop(begin(&context, &session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        (NpcUnitDataField::Int as i32).into(),
        50.into(),
    ]));
    process_map(&instance, &service);
    assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID], before);
}

#[test]
fn unit_data_uses_the_callers_instance_when_npc_ids_are_cloned() {
    let (context, instance, service, session) = fixture();
    let clone = destination(&context, &instance, "empty", true, 1);
    write(&context, &session, &instance, &service, NpcUnitDataField::Int, 50.into()).unwrap();
    assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID].parameters[3], 50);
    assert_eq!(clone.state().script_skill_state.npcs[&NPC_ID].parameters[3], 0);
    assert!(clone.task_queue().pop().is_none());
}

#[test]
fn npc_map_transfer_preserves_state_and_script_metadata_and_replies_after_installation() {
    let (context, origin, service, session) = fixture();
    let target = destination(&context, &origin, "other", false, 0);
    write(&context, &session, &origin, &service, NpcUnitDataField::Int, 50.into()).unwrap();
    write(&context, &session, &origin, &service, NpcUnitDataField::MaxHp, 200.into()).unwrap();
    write(&context, &session, &origin, &service, NpcUnitDataField::Hp, 100.into()).unwrap();
    let original_script = origin.get_script(NPC_ID).unwrap();
    let mut receiver = begin(&context, &session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        (NpcUnitDataField::Map as i32).into(),
        Value::String("other".into()),
    ]);
    process_map(&origin, &service);
    assert!(origin.get_script(NPC_ID).is_none());
    assert!(origin.state().script_skill_state.transferring_npcs.contains(&NPC_ID));
    assert!(matches!(receiver.try_recv(), Err(oneshot::error::TryRecvError::Empty)));
    process_transfers(&context);
    assert!(matches!(receiver.try_recv(), Err(oneshot::error::TryRecvError::Empty)));
    process_map(&target, &service);
    assert_eq!(receiver.try_recv().unwrap().unwrap(), Value::default());
    let moved = target.get_script(NPC_ID).unwrap();
    assert_eq!(
        (
            moved.map_name.as_str(),
            moved.name.as_str(),
            moved.entry_id,
            &moved.constructor_args
        ),
        (
            "other",
            original_script.name.as_str(),
            original_script.entry_id,
            &original_script.constructor_args
        )
    );
    assert!(moved.x < target.x_size() && moved.y < target.y_size());
    assert_eq!(target.state().script_skill_state.npcs[&NPC_ID].hp, 100);
    assert_eq!(target.state().script_skill_state.npcs[&NPC_ID].parameters[3], 50);
    assert!(origin.state().get_map_item(NPC_ID).is_none());
    process_transfers(&context);
    process_map(&origin, &service);
    assert!(!origin.state().script_skill_state.transferring_npcs.contains(&NPC_ID));
    let mut read = begin(&context, &session, Function::GetUnitData, vec![(NPC_ID as i32).into()]);
    process_map(&target, &service);
    assert!(matches!(read.try_recv().unwrap().unwrap(), Value::Array(_)));
}

#[test]
fn conflicting_destination_rolls_back_npc_transfer_before_reply() {
    let (context, origin, service, session) = fixture();
    let target = destination(&context, &origin, "other", true, 0);
    let before = origin.state().script_skill_state.npcs[&NPC_ID].clone();
    let mut receiver = begin(&context, &session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        (NpcUnitDataField::Map as i32).into(),
        Value::String("other".into()),
    ]);
    process_map(&origin, &service);
    process_transfers(&context);
    process_map(&target, &service);
    assert!(receiver.try_recv().unwrap().is_err());
    assert_eq!(origin.state().script_skill_state.npcs[&NPC_ID], before);
    assert_eq!(target.state().script_skill_state.npcs[&NPC_ID].hp, 0);
    assert!(!origin.state().script_skill_state.transferring_npcs.contains(&NPC_ID));
}

#[test]
fn cancellation_after_source_removal_restores_npc_without_installing_at_destination() {
    let (context, origin, service, session) = fixture();
    let target = destination(&context, &origin, "other", false, 0);
    let before = origin.state().script_skill_state.npcs[&NPC_ID].clone();
    let mut receiver = begin(&context, &session, Function::SetUnitData, vec![
        (NPC_ID as i32).into(),
        (NpcUnitDataField::Map as i32).into(),
        Value::String("other".into()),
    ]);
    process_map(&origin, &service);
    session.script_generation.fetch_add(1, Ordering::AcqRel);
    process_transfers(&context);
    process_map(&origin, &service);
    assert!(receiver.try_recv().unwrap().is_err());
    assert_eq!(origin.state().script_skill_state.npcs[&NPC_ID], before);
    assert!(target.get_script(NPC_ID).is_none());
    assert!(target.task_queue().pop().is_none());
}

#[test]
fn compiled_wasm_npc_reads_and_writes_unit_data_through_the_real_host_and_map_loop() {
    struct RecordingHost {
        npc: crate::server::script::NpcScriptHost,
        replies: Vec<Reply>,
    }
    #[async_trait::async_trait]
    impl script_runtime::Host for RecordingHost {
        async fn invoke(&mut self, request: Request) -> Reply {
            let reply = script_runtime::Host::invoke(&mut self.npc, request).await;
            self.replies.push(reply.clone());
            reply
        }
    }
    let (context, instance, service, session) = fixture();
    let script = instance.get_script(NPC_ID).unwrap();
    let runtime = context.runtime.clone();
    let server = Arc::new(context.server);
    let write = serde_json::to_vec(&Request::Call {
        function: Function::SetUnitData,
        arguments: vec![(NPC_ID as i32).into(), (NpcUnitDataField::Int as i32).into(), 50.into()],
    })
    .unwrap();
    let read = serde_json::to_vec(&Request::Call {
        function: Function::GetUnitData,
        arguments: vec![(NPC_ID as i32).into()],
    })
    .unwrap();
    let escape = |bytes: &[u8]| bytes.iter().map(|byte| format!("\\{byte:02x}")).collect::<String>();
    let module = format!(
        r#"(module
        (import "rust_ro" "invoke" (func $invoke (param i32 i32 i32 i32) (result i32)))
        (memory (export "memory") 1)
        (data (i32.const 0) "{}")
        (data (i32.const 256) "{}")
        (func (export "script_abi") (result i32) i32.const 1)
        (func (export "run_npc") (param i32) (result i32)
            i32.const 0 i32.const {} i32.const 4096 i32.const 4096 call $invoke drop
            i32.const 256 i32.const {} i32.const 4096 i32.const 4096 call $invoke drop i32.const 0))"#,
        escape(&write),
        escape(&read),
        write.len(),
        read.len()
    );
    let vm = script_runtime::WasmRuntime::from_bytes(module.as_bytes(), script_runtime::Limits::default()).unwrap();
    let (_, inputs) = tokio::sync::mpsc::channel(1);
    let host = RecordingHost {
        npc: crate::server::script::NpcScriptHost {
            server: server.clone(),
            session,
            script,
            inputs,
            notifications: context.client_notification_sender,
            generation: 0,
            map_instance: 0,
            background: false,
            event_depth: 0,
            event_arguments: None,
            timer_context: None,
            logout_token: None,
            error: None,
        },
        replies: vec![],
    };
    let (host, result) = runtime.block_on(async {
        tokio::time::timeout(std::time::Duration::from_secs(10), async {
            let execution = vm.execute(host, "run_npc", 0);
            tokio::pin!(execution);
            loop {
                tokio::select! {
                    result = &mut execution => break result,
                    _ = tokio::task::yield_now() => {}
                }
                while let Some(events) = context.server_task_queue.pop() {
                    for event in events {
                        let GameEvent::ScriptRequest(request) = event else {
                            panic!("Unexpected event {event:?}");
                        };
                        server.script_service().handle_request(&server, &mut server.state_mut(), request);
                    }
                }
                process_map(&instance, &service);
            }
        })
        .await
        .expect("NPC host request did not complete")
    });
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(host.replies.len(), 2);
    assert_eq!(host.replies[0], Ok(Value::default()));
    let Value::Array(data) = host.replies[1].as_ref().unwrap() else {
        panic!("Expected unit data");
    };
    assert_eq!(data[NpcUnitDataField::Int as usize], 50.into());
    assert_eq!(instance.state().script_skill_state.npcs[&NPC_ID].parameters[3], 50);
}
