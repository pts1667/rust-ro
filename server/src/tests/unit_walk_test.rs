use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use models::enums::EnumWithMaskValueU16;
use models::enums::cell::CellType;
use script_sdk::{Entry, Function, Reply, Request, Value};
use tokio::sync::oneshot;

use crate::server::Server;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::map_event::{MapEvent, ScriptMapCommand};
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::MapItems;
use crate::server::model::script::Script;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::ScriptRequest;
use crate::server::script::entries::intern;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::{MapInstanceService, npc_walk_packet};
use crate::server::service::mob_service::MobService;
use crate::server::model::movement::Movable;
use crate::server::service::script_service::ScriptService;
use crate::server::state::mob::{Mob, MobAction};

const BAKER: u32 = 620;
const THERAPIST: u32 = 621;
const PORING: u32 = 90_001;
/// A label of `events.json`, so the arrival event resolves to a compiled entry.
const ARRIVAL: &str = "Battle Therapist#b01_a::OnTimer25000";
const BLOCKED: (u16, u16) = (80, 50);
const NOTIFY_MOVE: u16 = 0x0086;
const STOP_MOVE: u16 = 0x0088;

struct Walk {
    server: Server,
    tasks: Arc<TasksQueue<GameEvent>>,
    instance: Arc<MapInstance>,
    service: MapInstanceService,
    packets: Receiver<Notification>,
    char_id: u32,
    account_id: u32,
}

fn npc(id: u32, name: &str, x: u16, y: u16) -> Script {
    Script {
        appearance: None, speed: None,
        id,
        scope_instance: 0,
        entry_id: intern("npc_walk_test", Entry::Npc(name.into())),
        map_name: "empty".into(),
        name: name.into(),
        sprite: 46,
        x,
        y,
        dir: 0,
        x_size: 0,
        y_size: 0,
        constructor_args: vec![],
    }
}

/// The Baker at 40,50, the NPC owning [`ARRIVAL`], a player at 50,50 and a Poring at 30,30 on an open map with one
/// blocked cell.
fn fixture() -> Walk {
    let (context, _repository, character) = super::native_payment_tests::fixture(false, false);
    let (char_id, account_id) = (character.char_id, character.account_id);
    context.server.state_mut().insert_character(character);
    let scripts = vec![npc(BAKER, "Baker", 40, 50), npc(THERAPIST, "Battle Therapist#b01_a", 60, 60)];
    let map = Box::leak(Box::new(Map::new(100, 100, 10_000, "empty".into(), "empty.gat".into(), vec![], vec![], scripts)));
    let mut cells = vec![CellType::Walkable.as_flag(); 10_000];
    cells[usize::from(BLOCKED.1) * 100 + usize::from(BLOCKED.0)] = 0;
    let instance = Arc::new(MapInstance::from_map(crate::tests::common::test_npc_vm(), map, 0, cells, context.client_notification_sender.clone(), MapItems::new(0), Arc::new(TasksQueue::new())));
    instance.state_mut().insert_mob(crate::tests::common::mob_helper::create_mob_at_position(PORING, "PORING", 30, 30));
    context.server.state_mut().map_instances_mut().insert("empty".into(), vec![instance.clone()]);
    let (sender, packets) = std::sync::mpsc::sync_channel::<Notification>(64);
    let service = MapInstanceService::new(sender.clone(), GlobalConfigService::instance(), MobService::new(sender, GlobalConfigService::instance()), context.server_task_queue.clone());
    Walk { server: context.server, tasks: context.server_task_queue, instance, service, packets, char_id, account_id }
}

impl Walk {
    /// A call made by the Baker's own background script, then the map commands it queued.
    fn call(&self, function: Function, arguments: Vec<Value>) -> Reply {
        self.send(Request::Call { function, arguments })
    }

    fn send(&self, request: Request) -> Reply {
        let (sender, mut receiver) = oneshot::channel();
        let request = ScriptRequest {
            char_id: 0,
            account_id: 0,
            npc_id: BAKER,
            npc_entry: intern("npc_walk_test", Entry::Npc("Baker".into())),
            npc_scope_instance: 0,
            map_instance: 0,
            generation: 0,
            background: true,
            event_depth: 0,
            timer_context: None,
            logout_token: None,
            request,
            response: Arc::new(Mutex::new(Some(sender))),
        };
        self.server.script_service().handle_request(&self.server, &mut self.server.state_mut(), request);
        while let Some(events) = self.instance.task_queue().pop() {
            for event in events {
                let MapEvent::ScriptMapCommand(command) = event else { panic!("Unexpected map event {event:?}") };
                self.service.script_map_command(&mut self.instance.state_mut(), command);
            }
        }
        receiver.try_recv().unwrap()
    }

    fn unit_walk(&self, unit: u32, x: i32, y: i32, label: Option<&str>) -> i32 {
        let mut values = vec![Value::Number(unit as i32), Value::Number(x), Value::Number(y)];
        values.extend(label.map(|label| Value::String(label.into())));
        self.call(Function::UnitWalk, values).unwrap().number_value().unwrap()
    }

    fn requested_moves(&self) -> Vec<(u32, u16, u16)> {
        let mut moves = Vec::new();
        while let Some(events) = self.server.pop_movement_task() {
            moves.extend(events.into_iter().filter_map(|event| match event {
                GameEvent::CharacterRequestMove(request) => Some((request.char_id, request.destination.x, request.destination.y)),
                _ => None,
            }));
        }
        moves
    }

    /// Puts the player on `(x, y)` and lets the movement loop see their path end there.
    fn player_stops_at(&self, x: u16, y: u16) {
        let mut state = self.server.state_mut();
        let mut character = state.characters_mut().remove(&self.char_id).unwrap();
        character.update_position(x, y);
        self.server.character_stopped_walking(&mut character);
        state.insert_character(character);
    }

    fn mob<T>(&self, read: impl FnOnce(&Mob) -> T) -> T {
        read(self.instance.state().get_mob(PORING).unwrap())
    }

    fn walk_to(&self, x: i32, y: i32, arguments: &[&str]) -> i32 {
        let mut values = vec![Value::Number(x), Value::Number(y)];
        values.extend(arguments.iter().map(|text| Value::String((*text).into())));
        self.call(Function::NpcWalkTo, values).unwrap().number_value().unwrap()
    }

    fn step(&self, tick: u128) -> (u16, u16) {
        self.service.step_npc_walks(&mut self.instance.state_mut(), tick);
        let state = self.instance.state();
        let baker = &state.script_skill_state.npcs[&BAKER];
        (baker.x, baker.y)
    }

    fn walking(&self) -> bool {
        self.instance.state().script_skill_state.npcs[&BAKER].walk.is_some()
    }

    fn first_step_at(&self) -> u128 {
        self.instance.state().script_skill_state.npcs[&BAKER].walk.as_ref().unwrap().steps.last().unwrap().move_at()
    }

    fn sent(&self) -> Vec<u16> {
        self.packets.try_iter().filter_map(|notification| match notification {
            Notification::Area(area) => Some(u16::from_le_bytes([area.packet[0], area.packet[1]])),
            _ => None,
        }).collect()
    }

    fn arrivals(&self) -> Vec<(u32, u32, Option<u32>)> {
        let mut arrivals = Vec::new();
        while let Some(events) = self.tasks.pop() {
            arrivals.extend(events.into_iter().filter_map(|event| match event {
                GameEvent::ScriptNpcEvent(event) => Some((event.npc_id, event.entry_id, event.char_id)),
                _ => None,
            }));
        }
        arrivals
    }
}

#[test]
fn a_walking_npc_enters_one_cell_per_step_delay_and_faces_its_way() {
    let walk = fixture();
    assert_eq!(walk.walk_to(44, 50, &[]), 1);
    assert_eq!(walk.sent(), [NOTIFY_MOVE]);
    let start = walk.first_step_at() - 200;

    assert_eq!(walk.step(start + 199), (40, 50), "the default speed is 200 ms per cell");
    assert_eq!(walk.step(start + 200), (41, 50));
    assert_eq!(walk.step(start + 799), (43, 50));
    assert_eq!(walk.step(start + 800), (44, 50));
    let state = walk.instance.state();
    let baker = &state.script_skill_state.npcs[&BAKER];
    assert_eq!((baker.walk.as_ref(), baker.dir), (None, 6), "east");
}

#[test]
fn a_late_step_catches_up_on_the_cells_it_missed() {
    let walk = fixture();
    walk.walk_to(44, 50, &[]);
    let start = walk.first_step_at() - 200;
    assert_eq!(walk.step(start + 650), (43, 50));
    assert!(walk.walking());
}

#[test]
fn the_arrival_label_runs_without_a_player_once_the_npc_arrives() {
    let walk = fixture();
    let entry = ScriptService::event_entry(ARRIVAL).unwrap();
    walk.walk_to(42, 50, &["", ARRIVAL]);
    walk.arrivals();
    let start = walk.first_step_at() - 200;
    walk.step(start + 200);
    assert_eq!(walk.arrivals(), []);
    walk.step(start + 400);
    assert_eq!(walk.arrivals(), [(THERAPIST, entry, None)]);
}

#[test]
fn a_walk_to_the_cell_the_npc_stands_on_arrives_at_once() {
    let walk = fixture();
    walk.arrivals();
    assert_eq!(walk.walk_to(40, 50, &["", ARRIVAL]), 1);
    assert_eq!(walk.arrivals().len(), 1);
    assert!(!walk.walking());
}

#[test]
fn stopping_finishes_the_cell_being_entered_and_drops_the_arrival() {
    let walk = fixture();
    walk.walk_to(46, 50, &["", ARRIVAL]);
    let start = walk.first_step_at() - 200;
    walk.step(start + 200);
    walk.sent();
    walk.call(Function::NpcStop, vec![]).unwrap();
    assert_eq!(walk.sent(), [STOP_MOVE]);
    assert_eq!(walk.step(start + 10_000), (42, 50));
    assert!(!walk.walking());
    walk.arrivals();
    assert_eq!(walk.arrivals(), []);
}

#[test]
fn speed_sets_the_step_delay_and_redraws_a_walking_npc_from_where_it_is() {
    let walk = fixture();
    walk.walk_to(44, 50, &[]);
    walk.sent();
    walk.call(Function::NpcSpeed, vec![Value::Number(400)]).unwrap();
    assert_eq!(walk.sent()[1..], [NOTIFY_MOVE], "the entry carries the new speed, then the walk goes on");
    let first = walk.first_step_at();
    walk.step(first);
    assert_eq!(walk.step(first + 399), (41, 50));
    assert_eq!(walk.step(first + 400), (42, 50));

    assert!(walk.call(Function::NpcSpeed, vec![Value::Number(19)]).is_err());
    assert!(walk.call(Function::NpcSpeed, vec![Value::Number(1001)]).is_err());
}

#[test]
fn walk_to_answers_0_without_a_path_or_while_the_npc_is_hidden() {
    let walk = fixture();
    assert_eq!(walk.walk_to(i32::from(BLOCKED.0), i32::from(BLOCKED.1), &[]), 0);
    assert_eq!(walk.walk_to(100, 50, &[]), 0, "outside the map");
    assert!(walk.call(Function::NpcWalkTo, vec![Value::Number(-1), Value::Number(50)]).is_err());
    walk.instance.state_mut().script_skill_state.npcs.get_mut(&BAKER).unwrap().hidden = true;
    assert_eq!(walk.walk_to(44, 50, &[]), 0);
    assert!(!walk.walking());
    assert!(walk.sent().is_empty());
}

#[test]
fn an_unknown_arrival_label_is_an_error() {
    let walk = fixture();
    assert!(walk.call(Function::NpcWalkTo, vec![Value::Number(44), Value::Number(50), Value::String(String::new()), Value::String("Baker::OnNothing".into())]).is_err());
    assert!(!walk.walking());
}

#[test]
fn a_named_npc_walks_instead_of_the_caller() {
    let walk = fixture();
    assert_eq!(walk.walk_to(62, 60, &["Battle Therapist#b01_a"]), 1);
    let state = walk.instance.state();
    assert!(state.script_skill_state.npcs[&THERAPIST].walk.is_some());
    assert!(state.script_skill_state.npcs[&BAKER].walk.is_none());
}

#[test]
fn a_jump_or_hiding_ends_the_walk() {
    let walk = fixture();
    walk.walk_to(44, 50, &["", ARRIVAL]);
    walk.service.script_map_command(&mut walk.instance.state_mut(), ScriptMapCommand::NpcMove { npc_id: BAKER, x: 10, y: 10, dir: None });
    assert!(!walk.walking());

    walk.walk_to(14, 10, &[]);
    walk.service.script_map_command(&mut walk.instance.state_mut(), ScriptMapCommand::NpcVisibility { npc_id: BAKER, visible: false });
    assert!(!walk.walking());
    walk.arrivals();
    walk.step(u128::MAX);
    assert_eq!(walk.arrivals(), []);
}

#[test]
fn players_who_see_a_walking_npc_get_its_walk_after_its_entry() {
    let walk = fixture();
    let packetver = GlobalConfigService::instance().packetver();
    assert_eq!(npc_walk_packet(&walk.instance.state().script_skill_state.npcs[&BAKER], packetver), None);
    walk.walk_to(44, 50, &[]);
    let packet = npc_walk_packet(&walk.instance.state().script_skill_state.npcs[&BAKER], packetver).unwrap();
    assert_eq!(u16::from_le_bytes([packet[0], packet[1]]), NOTIFY_MOVE);
    assert_eq!(u32::from_le_bytes(packet[2..6].try_into().unwrap()), BAKER);
}

#[test]
fn unitwalk_moves_a_player_like_a_click_and_runs_the_label_attached_to_them_on_arrival() {
    let walk = fixture();
    let entry = ScriptService::event_entry(ARRIVAL).unwrap();
    assert_eq!(walk.unit_walk(walk.account_id, 54, 50, Some(ARRIVAL)), 1, "the account id names the player, as getcharid(3) does");
    assert_eq!(walk.requested_moves(), [(walk.char_id, 54, 50)]);
    walk.arrivals();
    walk.player_stops_at(54, 50);
    assert_eq!(walk.arrivals(), [(THERAPIST, entry, Some(walk.char_id))]);
    assert!(walk.server.state().get_character(walk.char_id).unwrap().game_systems.forced_walk.is_none());
}

#[test]
fn a_player_who_stops_somewhere_else_drops_the_label() {
    let walk = fixture();
    walk.unit_walk(walk.char_id, 54, 50, Some(ARRIVAL));
    walk.arrivals();
    walk.player_stops_at(52, 50);
    assert_eq!(walk.arrivals(), []);
    assert!(walk.server.state().get_character(walk.char_id).unwrap().game_systems.forced_walk.is_none());
}

#[test]
fn unitwalk_answers_0_without_a_path_or_a_unit() {
    let walk = fixture();
    assert_eq!(walk.unit_walk(walk.char_id, i32::from(BLOCKED.0), i32::from(BLOCKED.1), None), 0);
    assert_eq!(walk.unit_walk(PORING, i32::from(BLOCKED.0), i32::from(BLOCKED.1), None), 0);
    assert_eq!(walk.unit_walk(123_456, 54, 50, None), 0);
    assert_eq!(walk.requested_moves(), []);
}

#[test]
fn a_player_already_on_the_cell_arrives_at_once() {
    let walk = fixture();
    walk.arrivals();
    assert_eq!(walk.unit_walk(walk.char_id, 50, 50, Some(ARRIVAL)), 1);
    assert_eq!(walk.requested_moves(), []);
    assert_eq!(walk.arrivals().len(), 1);
}

#[test]
fn unitwalk_walks_a_monster_that_the_ai_leaves_alone_until_it_arrives() {
    let walk = fixture();
    let entry = ScriptService::event_entry(ARRIVAL).unwrap();
    assert_eq!(walk.unit_walk(PORING, 33, 30, Some(ARRIVAL)), 1);
    assert_eq!(walk.sent(), [NOTIFY_MOVE]);
    assert_eq!(walk.mob(|mob| (mob.movements.len(), matches!(mob.action, MobAction::Moving), mob.forced_walk.is_some())), (3, true, true));
    assert_eq!(walk.unit_walk(PORING, 35, 30, None), 0, "rathena refuses a second unitwalk until the first ends");

    walk.service.mobs_action(&mut walk.instance.state_mut(), walk.instance.task_queue(), 0);
    assert_eq!(walk.mob(|mob| mob.movements.len()), 3, "the AI does not replace the path");

    walk.arrivals();
    {
        let key = walk.instance.state().key().clone();
        let mut state = walk.instance.state_mut();
        let mob = state.mobs_mut().get_mut(&PORING).unwrap();
        mob.clear_movement();
        (mob.x, mob.y) = (33, 30);
        walk.service.mob_walk_ended(mob, &key);
    }
    assert_eq!(walk.arrivals(), [(THERAPIST, entry, None)]);
    assert!(walk.mob(|mob| mob.forced_walk.is_none()));
}

#[test]
fn a_monster_whose_walk_was_cut_short_goes_back_to_its_ai() {
    let walk = fixture();
    walk.unit_walk(PORING, 33, 30, Some(ARRIVAL));
    walk.instance.state_mut().mobs_mut().get_mut(&PORING).unwrap().clear_movement();
    walk.service.mobs_action(&mut walk.instance.state_mut(), walk.instance.task_queue(), 0);
    assert!(walk.mob(|mob| mob.forced_walk.is_none()));
}

#[test]
fn unitstopwalk_stops_a_monster_and_drops_its_label() {
    let walk = fixture();
    walk.unit_walk(PORING, 33, 30, Some(ARRIVAL));
    walk.sent();
    walk.call(Function::UnitStopWalk, vec![Value::Number(PORING as i32)]).unwrap();
    assert_eq!(walk.sent(), [STOP_MOVE]);
    assert_eq!(walk.mob(|mob| (mob.movements.is_empty(), mob.forced_walk.is_none(), matches!(mob.action, MobAction::Idle))), (true, true, true));
}

#[test]
fn unitwalkto_takes_an_npc_to_the_cell_a_player_stands_on() {
    let walk = fixture();
    assert_eq!(walk.call(Function::UnitWalkTo, vec![Value::Number(BAKER as i32), Value::Number(walk.account_id as i32)]).unwrap(), Value::Number(1));
    let destination = walk.instance.state().script_skill_state.npcs[&BAKER].walk.as_ref().map(|walk| (walk.destination, walk.forced));
    assert_eq!(destination, Some(((50, 50), true)));
    assert_eq!(walk.unit_walk(BAKER, 45, 50, None), 0, "a second unitwalk waits for the first");
    assert_eq!(walk.walk_to(45, 50, &[]), 1, "npcwalkto replaces it, as in rathena");
}

#[test]
fn a_walk_past_the_client_pathfinder_goes_leg_by_leg_and_names_the_walker_on_arrival() {
    let walk = fixture();
    walk.instance.state_mut().update_cells((45, 0), (45, 98), |_| 0);
    let entry = ScriptService::event_entry(ARRIVAL).unwrap();
    assert_eq!(walk.walk_to(50, 50, &["", ARRIVAL]), 1, "the only way round is the gap at the top of the wall");
    assert!(!walk.instance.state().script_skill_state.npcs[&BAKER].walk.as_ref().unwrap().legs.is_empty());
    walk.arrivals();
    walk.sent();
    let mut tick = walk.first_step_at();
    let mut legs = 1;
    for _ in 0..1_000 {
        if !walk.walking() {
            break;
        }
        walk.step(tick);
        legs += walk.sent().iter().filter(|packet| **packet == NOTIFY_MOVE).count();
        tick += 1_000;
    }
    assert_eq!(walk.step(tick), (50, 50));
    assert!(legs > 5, "{legs} legs");
    let mut arrivals = Vec::new();
    while let Some(events) = walk.tasks.pop() {
        arrivals.extend(events.into_iter().filter_map(|event| match event {
            GameEvent::ScriptNpcEvent(event) => Some((event.npc_id, event.entry_id, event.args)),
            _ => None,
        }));
    }
    assert_eq!(arrivals, [(THERAPIST, entry, Some(vec![Value::Number(BAKER as i32)]))]);
}

#[test]
fn a_leg_that_ends_where_the_npc_already_stands_is_skipped() {
    let walk = fixture();
    walk.instance.state_mut().update_cells((45, 0), (45, 98), |_| 0);
    assert_eq!(walk.walk_to(50, 50, &[]), 1);
    {
        let mut state = walk.instance.state_mut();
        let npc_walk = state.script_skill_state.npcs.get_mut(&BAKER).unwrap().walk.as_mut().unwrap();
        let leg_end = npc_walk.steps.first().map(|end| (end.position().x, end.position().y)).unwrap();
        npc_walk.legs.push(leg_end);
    }
    let mut tick = walk.first_step_at();
    for _ in 0..1_000 {
        if !walk.walking() {
            break;
        }
        walk.step(tick);
        tick += 1_000;
    }
    assert_eq!(walk.step(tick), (50, 50));
}

#[test]
fn a_wall_raised_after_the_walk_started_is_seen_by_the_next_reachability_check() {
    let walk = fixture();
    assert_eq!(walk.walk_to(50, 50, &[]), 1);
    walk.instance.state_mut().update_cells((45, 0), (45, 99), |_| 0);
    assert_eq!(walk.walk_to(50, 50, &[]), 0);
}

#[test]
fn npc_commands_take_a_unit_id_in_place_of_the_name() {
    let walk = fixture();
    let therapist = Value::Number(THERAPIST as i32);
    assert_eq!(walk.call(Function::NpcWalkTo, vec![Value::Number(62), Value::Number(60), therapist.clone()]).unwrap(), Value::Number(1));
    assert!(walk.instance.state().script_skill_state.npcs[&THERAPIST].walk.is_some());
    walk.call(Function::NpcStop, vec![therapist.clone()]).unwrap();
    walk.call(Function::DisableNpc, vec![therapist.clone()]).unwrap();
    assert!(walk.instance.state().script_skill_state.npcs[&THERAPIST].hidden);
    walk.call(Function::EnableNpc, vec![therapist.clone()]).unwrap();
    assert!(!walk.instance.state().script_skill_state.npcs[&THERAPIST].hidden);
    assert!(walk.call(Function::Emotion, vec![Value::Number(1), therapist]).is_ok(), "an NPC can be the emotion's target");
    assert!(walk.call(Function::DisableNpc, vec![Value::Number(123_456)]).is_err());
}

#[test]
fn movenpc_onto_its_own_cell_turns_the_npc_without_drawing_it_again() {
    let walk = fixture();
    walk.sent();
    walk.call(Function::MoveNpc, vec![Value::Number(BAKER as i32), Value::Number(40), Value::Number(50), Value::Number(2)]).unwrap();
    assert_eq!(walk.sent(), [0x009c], "ZC_CHANGE_DIRECTION, no vanish");
    assert_eq!(walk.instance.state().script_skill_state.npcs[&BAKER].dir, 2);
    walk.call(Function::MoveNpc, vec![Value::String("Baker".into()), Value::Number(41), Value::Number(50), Value::Number(4)]).unwrap();
    assert_eq!(walk.sent()[0], 0x0080, "another cell is a jump");
    let turn = |dir: i32| walk.call(Function::MoveNpc, vec![Value::Number(BAKER as i32), Value::Number(-1), Value::Number(-1), Value::Number(dir)]).unwrap();
    turn(6);
    assert_eq!(walk.sent(), [0x009c], "-1, -1 turns it where it stands");
    let baker = walk.instance.state().script_skill_state.npcs[&BAKER].clone();
    assert_eq!((baker.x, baker.y, baker.dir), (41, 50, 6));
    assert_eq!(walk.call(Function::NpcWalkTo, vec![Value::Number(60), Value::Number(50), Value::Number(BAKER as i32)]).unwrap(), Value::Number(1));
    walk.sent();
    turn(0);
    assert!(walk.sent().is_empty(), "a walking NPC is not turned");
    assert_eq!(walk.instance.state().script_skill_state.npcs[&BAKER].dir, 6);
}

#[test]
fn getmapplayers_lists_each_player_of_the_map_with_their_cell_and_level() {
    let walk = fixture();
    let mut expected: Vec<Vec<Value>> = walk
        .server
        .state()
        .characters()
        .values()
        .map(|character| [character.account_id as i32, character.char_id as i32, i32::from(character.x()), i32::from(character.y()), character.get_base_level() as i32].map(Value::Number).to_vec())
        .collect();
    expected.sort_by_key(|player| player[1].number_value().unwrap());
    assert!(expected.iter().any(|player| player[1] == Value::Number(walk.char_id as i32)));
    let Value::Array(players) = walk.call(Function::GetMapPlayers, vec![Value::String("empty".into())]).unwrap() else { panic!("not an array") };
    let mut players: Vec<Vec<Value>> = players.into_iter().map(|player| if let Value::Array(fields) = player { fields } else { panic!("{player:?}") }).collect();
    players.sort_by_key(|player| player[1].number_value().unwrap());
    assert_eq!(players, expected);
    assert_eq!(walk.call(Function::GetMapPlayers, vec![Value::String("prontera".into())]).unwrap(), Value::Array(vec![]));
}

#[test]
fn getnpcid_resolves_a_list_of_names_in_one_call() {
    let walk = fixture();
    let names = Value::Array(vec!["Baker".into(), "Nobody".into(), "Battle Therapist#b01_a".into()]);
    let ids = walk.call(Function::GetNpcId, vec![Value::Number(0), names]).unwrap();
    assert_eq!(ids, Value::Array(vec![Value::Number(BAKER as i32), Value::Number(0), Value::Number(THERAPIST as i32)]));
}

#[test]
fn a_batch_is_answered_in_one_pass_and_refuses_requests_that_wait() {
    let walk = fixture();
    let call = |function, arguments| Request::Call { function, arguments };
    let replies = walk.send(Request::Batch(vec![
        call(Function::NpcWalkTo, vec![Value::Number(62), Value::Number(60), Value::Number(THERAPIST as i32)]),
        call(Function::Emotion, vec![Value::Number(1), Value::Number(BAKER as i32)]),
        call(Function::GetMapUsers, vec![Value::String("empty".into())]),
    ]));
    let users = walk.call(Function::GetMapUsers, vec![Value::String("empty".into())]).unwrap();
    assert_eq!(replies, Ok(Value::Array(vec![Value::Number(1), Value::Number(0), users])));
    assert!(walk.instance.state().script_skill_state.npcs[&THERAPIST].walk.is_some());

    for waiting in [call(Function::Sleep, vec![Value::Number(10)]), call(Function::SetUnitData, vec![Value::Number(BAKER as i32), Value::Number(0), Value::Number(0)]), Request::Batch(vec![])] {
        let error = walk.send(Request::Batch(vec![call(Function::GetMapUsers, vec![Value::String("empty".into())]), waiting])).unwrap_err();
        assert!(error.starts_with("batch request 1 cannot be batched"), "{error}");
    }
    let error = walk.send(Request::Batch(vec![call(Function::DisableNpc, vec![Value::Number(123_456)])])).unwrap_err();
    assert!(error.starts_with("batch request 0: NPC is not found"), "{error}");
}
