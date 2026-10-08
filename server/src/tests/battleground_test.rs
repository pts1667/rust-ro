use models::status_change::StatusChangeKind;
use script_sdk::{Function, Value};

use crate::server::model::battleground_queue::{ADMISSION_WINDOW_MS, BattlegroundQueueAction, BattlegroundQueueCommand, QueueState};
use crate::server::model::map_flags::{MapFlag, MapFlags};
use crate::server::service::battleground_service::battleground_members;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;

fn fixture() -> super::ServerServiceTestContext {
    let (context, _) = super::world_party_tests::fixture();
    let mut flags = MapFlags::default();
    flags.set(MapFlag::Battleground, true, &[1]).unwrap();
    context.server.map_flag_overrides().insert(("empty".into(), 0), flags);
    context
}

fn call(context: &super::ServerServiceTestContext, attached: u32, function: Function, arguments: Vec<Value>) -> Value {
    context.server.battleground_call(&mut *context.server.state_mut(), attached, function, &arguments).unwrap()
}

fn create_team(context: &super::ServerServiceTestContext, x: i32) -> i32 {
    call(context, 0, Function::BgCreate, vec!["empty".into(), x.into(), 20.into()]).number_value().unwrap()
}

fn join(context: &super::ServerServiceTestContext, team: i32, char_id: u32) -> i32 {
    call(context, 0, Function::BgJoin, vec![team.into(), "".into(), 0.into(), 0.into(), (char_id as i32).into()])
        .number_value()
        .unwrap()
}

#[test]
fn teammates_are_protected_and_rival_teams_are_hostile_on_a_battleground_map() {
    let context = fixture();
    let (first, second) = (create_team(&context, 10), create_team(&context, 90));
    assert_eq!((join(&context, first, 150_000), join(&context, second, 150_001)), (1, 1));
    let state = context.server.state();
    let leader = state.get_character(150_000).unwrap();
    assert!(context.server.player_combat_target_allowed(&state, leader, 150_001));
    drop(state);
    assert_eq!(call(&context, 0, Function::BgGetData, vec![first.into(), 0.into()]), Value::Number(1));
    call(&context, 0, Function::BgDestroy, vec![second.into()]);
    assert_eq!(join(&context, first, 150_001), 1);
    let state = context.server.state();
    assert!(!context.server.player_combat_target_allowed(&state, state.get_character(150_000).unwrap(), 150_001));
}

#[test]
fn joining_requires_a_battleground_map_and_one_team_per_player() {
    let context = fixture();
    let (first, second) = (create_team(&context, 10), create_team(&context, 90));
    assert_eq!(join(&context, first, 150_000), 1);
    assert_eq!(join(&context, second, 150_000), 0);
    context.server.map_flag_overrides().insert(("empty".into(), 0), MapFlags::default());
    let rejected = context.server.battleground_call(
        &mut *context.server.state_mut(),
        0,
        Function::BgJoin,
        &[second.into(), "".into(), 0.into(), 0.into(), 150_001.into()],
    );
    assert!(rejected.is_err());
}

#[test]
fn members_respawn_at_their_team_cemetery_and_leave_clears_the_roster() {
    let context = fixture();
    let team = create_team(&context, 10);
    join(&context, team, 150_000);
    call(&context, 0, Function::BgTeamSetXy, vec![team.into(), 33.into(), 44.into()]);
    let cemetery = context.server.battleground_cemetery(&context.server.state(), 150_000).unwrap();
    assert_eq!((cemetery.map.as_str(), cemetery.x, cemetery.y), ("empty", 33, 44));
    call(&context, 150_000, Function::BgLeave, vec![]);
    assert!(context.server.battleground_cemetery(&context.server.state(), 150_000).is_none());
    assert_eq!(call(&context, 0, Function::BgGetData, vec![team.into(), 0.into()]), Value::Number(0));
}

#[test]
fn destroying_a_team_releases_every_member_and_leaving_resets_tracking() {
    let context = fixture();
    let (first, second) = (create_team(&context, 10), create_team(&context, 90));
    join(&context, first, 150_000);
    join(&context, second, 150_001);
    assert_eq!(context.server.state().get_character(150_000).unwrap().bg_id, first as u32);
    assert!(context.server.state().get_character(150_000).unwrap().bg_tracking.last_hp == u32::MAX);
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().bg_tracking.last_hp = 10;
    call(&context, 150_000, Function::BgLeave, vec![]);
    let guard_88 = context.server.state();
    let leader = guard_88.get_character(150_000).unwrap();
    assert_eq!((leader.bg_id, leader.bg_tracking.last_hp), (0, u32::MAX));
    drop(guard_88);
    call(&context, 0, Function::BgDestroy, vec![second.into()]);
    assert_eq!(context.server.state().get_character(150_001).unwrap().bg_id, 0);
    assert!(context.server.battlegrounds().team(second as u32).is_none());
    assert_eq!(join(&context, second, 150_001), 0);
}

fn add_players(context: &super::ServerServiceTestContext, count: u32) -> Vec<u32> {
    let mut ids = vec![150_000, 150_001];
    for index in 0..count.saturating_sub(2) {
        let mut player = crate::tests::common::character_helper::create_character();
        player.char_id = 160_000 + index;
        player.account_id = 160_000 + index;
        player.name = format!("Queued {index}");
        player.map_instance_key = context.server.state().get_character(150_000).unwrap().map_instance_key.clone();
        ids.push(player.char_id);
        context.server.state_mut().insert_character(player);
    }
    for id in &ids {
        let mut state = context.server.state_mut();
        let player = state.characters_mut().get_mut(id).unwrap();
        player.status.base_level = 90;
        player.status.job = 1;
    }
    ids
}

fn queue_command(context: &super::ServerServiceTestContext, char_id: u32, action: BattlegroundQueueAction) {
    context.server.handle_battleground_queue_command(&mut *context.server.state_mut(), BattlegroundQueueCommand { char_id, action });
}

#[test]
fn a_full_queue_asks_for_acceptance_then_builds_two_teams_and_publishes_their_ids() {
    let context = fixture();
    let ids = add_players(&context, 12);
    for id in &ids {
        queue_command(&context, *id, BattlegroundQueueAction::Apply { kind: 1, name: "Flavius".into() });
    }
    {
        let (sides, reserved) = context.server.battlegrounds().with_queues(|queues| {
            let queue = queues.queues.iter().find(|queue| queue.state == QueueState::SetupDelay).unwrap();
            ((queue.team_a.len(), queue.team_b.len()), queues.reserved_maps.contains("bat_b01"))
        });
        assert_eq!(sides, (6, 6));
        assert!(reserved);
    }
    for id in &ids {
        queue_command(&context, *id, BattlegroundQueueAction::Reply { accept: true });
    }
    context.server.tick_battleground_queues(&mut *context.server.state_mut(), crate::util::tick::get_tick() + 1);
    let state = context.server.state();
    let first = context.server.script_service().server_temporary("$@FlaviusBG1_id1").unwrap().number_value().unwrap() as u32;
    let second = context.server.script_service().server_temporary("$@FlaviusBG1_id2").unwrap().number_value().unwrap() as u32;
    assert_ne!(first, second);
    assert_eq!((battleground_members(&state, first).len(), battleground_members(&state, second).len()), (6, 6));
    let cemetery = context.server.battlegrounds().team(first).unwrap().cemetery.unwrap();
    assert_eq!((cemetery.map.as_str(), cemetery.x, cemetery.y), ("bat_b01", 10, 290));
    assert!(context.server.battlegrounds().with_queues(|queues| queues.queues.iter().any(|queue| queue.state == QueueState::Active)));
}

#[test]
fn an_unanswered_admission_window_dissolves_the_queue_and_frees_the_arena() {
    let context = fixture();
    let ids = add_players(&context, 12);
    for id in &ids {
        queue_command(&context, *id, BattlegroundQueueAction::Apply { kind: 1, name: "Flavius".into() });
    }
    context.server.tick_battleground_queues(&mut *context.server.state_mut(), crate::util::tick::get_tick() + ADMISSION_WINDOW_MS as u128 + 1);
    assert!(context.server.battlegrounds().with_queues(|queues| queues.reserved_maps.is_empty()));
    assert!(ids.iter().all(|id| context.server.battlegrounds().with_queues(|queues| queues.queue_of(*id)).is_none()));
}

#[test]
fn applications_are_rejected_for_restricted_jobs_low_levels_and_duplicates() {
    let context = fixture();
    {
        let mut state = context.server.state_mut();
        state.characters_mut().get_mut(&150_000).unwrap().status.job = 0;
        let member = state.characters_mut().get_mut(&150_001).unwrap();
        member.status.job = 1;
        member.status.base_level = 40;
    }
    queue_command(&context, 150_000, BattlegroundQueueAction::Apply { kind: 1, name: "Flavius".into() });
    queue_command(&context, 150_001, BattlegroundQueueAction::Apply { kind: 1, name: "Flavius".into() });
    assert!(context.server.battlegrounds().with_queues(|queues| queues.queue_of(150_000)).is_none());
    assert!(context.server.battlegrounds().with_queues(|queues| queues.queue_of(150_001)).is_none());
    context.server.state_mut().characters_mut().get_mut(&150_001).unwrap().status.base_level = 90;
    queue_command(&context, 150_001, BattlegroundQueueAction::Apply { kind: 1, name: "Flavius".into() });
    assert!(context.server.battlegrounds().with_queues(|queues| queues.queue_of(150_001)).is_some());
    queue_command(&context, 150_001, BattlegroundQueueAction::Cancel("Flavius".into()));
    assert!(context.server.battlegrounds().with_queues(|queues| queues.queue_of(150_001)).is_none());
}

#[test]
fn bg_info_reads_the_catalog_and_leaving_a_team_as_a_deserter_blocks_new_applications() {
    let context = fixture();
    let info = |kind: i32| context.server.battleground_queue_script_call(&mut *context.server.state_mut(), Function::BgInfo, &["Flavius".into(), kind.into()]).unwrap();
    assert_eq!((info(0), info(1), info(2), info(3)), (Value::Number(2), Value::Number(6), Value::Number(15), Value::Number(80)));
    let team = create_team(&context, 10);
    join(&context, team, 150_000);
    call(&context, 150_000, Function::BgDesert, vec![]);
    assert!(context.server.state().get_character(150_000).unwrap().status.status_change(StatusChangeKind::EntryQueueNotifyAdmissionTimeOut).is_some());
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().status.base_level = 90;
    context.server.state_mut().characters_mut().get_mut(&150_000).unwrap().status.job = 1;
    queue_command(&context, 150_000, BattlegroundQueueAction::Apply { kind: 1, name: "Flavius".into() });
    assert!(context.server.battlegrounds().with_queues(|queues| queues.queue_of(150_000)).is_none());
}

#[test]
fn battleground_monsters_are_protected_from_their_own_team_and_honor_damage_immunity() {
    use crate::server::model::events::map_event::{ScriptMobCommand, ScriptSpawn};
    let context = fixture();
    let team = create_team(&context, 10);
    join(&context, team, 150_000);
    let instance = context.server.state().get_map_instance(&"empty".into(), 0).unwrap();
    let request = ScriptSpawn {
        mob_id: 1002,
        x: 50,
        y: 50,
        name: "Crystal".into(),
        amount: 1,
        event: String::new(),
        event_npc: None,
        size: None,
        ai: None,
        owner_id: 0,
        guardian: None,
        bg_id: team as u32,
        max_hp: None,
        lifetime_ms: None,
    reserved_id: None,
    area_end: None,
    };
    let service = map_instance_service(&context);
    let ids = service.script_spawn(&mut *instance.state_mut(), request).unwrap();
    let state = context.server.state();
    assert!(!context.server.player_combat_target_allowed(&state, state.get_character(150_000).unwrap(), ids[0]));
    assert!(context.server.player_combat_target_allowed(&state, state.get_character(150_001).unwrap(), ids[0]));
    assert_eq!(instance.state().get_mob(ids[0]).unwrap().bg_id, team as u32);
    drop(state);
    service.script_mob_command(&mut *instance.state_mut(), ScriptMobCommand::Kill { event_entry: None });
    assert!(instance.state().get_mob(ids[0]).is_none());
}

fn map_instance_service(context: &super::ServerServiceTestContext) -> MapInstanceService {
    MapInstanceService::new(
        context.client_notification_sender.clone(),
        GlobalConfigService::instance(),
        MobService::new(context.client_notification_sender.clone(), GlobalConfigService::instance()),
        context.server_task_queue.clone(),
    )
}
