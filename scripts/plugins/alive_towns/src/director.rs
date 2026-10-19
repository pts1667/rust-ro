//! One hidden controller NPC per town, `Alive Towns_<town>`: every second it sends, in a couple of batches, what the town's
//! townsfolk did since its previous run. A town that wakes up, or a run after a long gap, puts everyone in place instead.
//!
//! Map instances are only made when a player first enters the map, and an NPC timer needs its NPC on a loaded map, so a
//! player entering a town starts its controller (`MapEnter`), and the controller of an empty town lets its timer lapse.

use script_sdk_2::{Batch, Ctx, Function, Hook, Request, Script, Stop, Val, Value};

use crate::persona::unique_name;
use crate::schedule::{Plan, State, Step};
use crate::towns::{TOWNS, find};
use crate::Settings;

/// Labels of `Alive Towns` serve every `Alive Towns_<n>`, the way duplicated NPCs share their original's.
pub const CONTROLLER: &str = "Alive Towns";
/// A longer gap between two runs puts the town back in place instead of replaying what it missed.
const MAX_GAP_MS: i64 = 5_000;

pub fn controller_name(town: usize) -> String {
    format!("{CONTROLLER}_{town}")
}

fn controller_town(name: &str) -> Option<usize> {
    name.strip_prefix(CONTROLLER)?.strip_prefix('_')?.parse().ok().filter(|town| *town < TOWNS.len())
}

fn last_var(map: &str) -> String {
    format!("$@last_{map}")
}

fn active_var(map: &str) -> String {
    format!("$@active_{map}")
}

fn ids_var(map: &str) -> String {
    format!("$@ids_{map}$")
}

/// Clicking the hidden controller does nothing.
pub fn controller(_ctx: &Ctx) -> Script {
    Ok(())
}

/// No town is loaded at boot; [`entered`] starts them.
pub fn start(_ctx: &Ctx) -> Script {
    Ok(())
}

pub fn entered(ctx: &Ctx, hook: &Hook) -> Script {
    let Hook::MapEnter { map, .. } = hook else { return Ok(()) };
    let settings: Settings = ctx.plugin_config()?;
    let Some(town) = find(map).filter(|town| settings.towns.iter().any(|configured| configured == TOWNS[*town].map)) else { return Ok(()) };
    ctx.call(Function::InitNpcTimer, vec![controller_name(town).into()]).map(|_| ())
}

/// `OnTimer1000`: the timer starts again while the town is active, also when the run fails.
pub fn tick(ctx: &Ctx) -> Script {
    let name = ctx.npc().name()?;
    let town = controller_town(&name).ok_or_else(|| Stop::from(format!("{name} is not a town controller")))?;
    let result = run(ctx, town);
    if !matches!(result, Ok(false)) {
        ctx.call(Function::InitNpcTimer, vec![name.into()])?;
    }
    result.map(|_| ())
}

/// `OnDoor`, the arrival of a walk through a door: the walker goes inside.
pub fn went_inside(ctx: &Ctx) -> Script {
    let walker = ctx.arguments()?.first().ok_or_else(|| Stop::from("OnDoor runs on a walk arrival"))?.number()?;
    ctx.call(Function::DisableNpc, vec![walker.into()]).map(|_| ())
}

/// Runs one second of `town`; whether the town is still active.
fn run(ctx: &Ctx, town: usize) -> Result<bool, Stop> {
    let settings: Settings = ctx.plugin_config()?;
    let map = TOWNS[town].map;
    let mut reads = ctx.batch();
    reads.call(Function::GetTimeTick, vec![0.into()]);
    reads.request(Request::Read(last_var(map)));
    reads.call(Function::GetMapUsers, vec![map.into()]);
    reads.request(Request::Read(active_var(map)));
    reads.request(Request::Read(ids_var(map)));
    let replies = reads.send()?;
    let tick = |reply: &Val| reply.number().map(|value| i64::from(value as u32));
    let (now, last) = (tick(&replies[0])?, tick(&replies[1])?);
    let active = !settings.idle_when_empty || replies[2].number()? > 0;
    let was_active = replies[3].number()? != 0;

    let mut commands = ctx.batch();
    if active != was_active {
        commands.request(Request::Write { name: active_var(map), value: Value::Number(i32::from(active)) });
    }
    if !active {
        commands.send()?;
        return Ok(false);
    }
    let plan = Plan::new(&settings, town);
    let known: Vec<i32> = replies[4].text().split(',').filter_map(|id| id.parse().ok()).collect();
    let fetched = known.len() != settings.npcs_per_town as usize;
    let ids = if fetched {
        let names = plan.personas.iter().enumerate().map(|(index, persona)| Value::String(unique_name(persona.display, town, index as u32))).collect();
        let found = ctx.call(Function::GetNpcId, vec![0.into(), Val::from(Value::Array(names))])?;
        let ids: Vec<i32> = found.into_array().ok_or_else(|| Stop::from("GetNpcId did not answer an array"))?.iter().map(|id| id.number().unwrap_or(0)).collect();
        if ids.iter().any(|id| *id != 0) {
            let list = ids.iter().map(i32::to_string).collect::<Vec<_>>().join(",");
            commands.request(Request::Write { name: ids_var(map), value: Value::String(list) });
        }
        ids
    } else {
        known
    };
    let caught_up = was_active && !fetched && last != 0 && now > last && now - last <= MAX_GAP_MS;
    let door = format!("{}::OnDoor", controller_name(town));
    for (index, id) in ids.iter().enumerate().filter(|(_, id)| **id != 0) {
        if caught_up {
            for (_, step) in plan.steps_between(index as u32, last, now) {
                send(&mut commands, *id, step, &door);
            }
        } else {
            place(&mut commands, *id, plan.state_at(index as u32, now));
        }
    }
    commands.request(Request::Write { name: last_var(map), value: Value::Number(now as u32 as i32) });
    commands.send()?;
    Ok(true)
}

fn send(commands: &mut Batch, id: i32, step: Step, door_label: &str) {
    match step {
        Step::Walk { to, door } => {
            let mut arguments = vec![i32::from(to.0).into(), i32::from(to.1).into(), id.into()];
            if door {
                arguments.push(door_label.into());
            }
            commands.call(Function::NpcWalkTo, arguments)
        }
        Step::Face(dir) => commands.call(Function::MoveNpc, vec![id.into(), (-1).into(), (-1).into(), i32::from(dir).into()]),
        Step::Emote(emote) => commands.call(Function::Emotion, vec![emote.into(), id.into()]),
        Step::Show => commands.call(Function::EnableNpc, vec![id.into()]),
    };
}

/// Puts the NPC where `state` says, hidden first so that it is not drawn at the door it waits behind.
fn place(commands: &mut Batch, id: i32, state: State) {
    if state.hidden {
        commands.call(Function::DisableNpc, vec![id.into()]);
    }
    let (x, y) = (i32::from(state.cell.0), i32::from(state.cell.1));
    commands.call(Function::MoveNpc, vec![id.into(), x.into(), y.into(), i32::from(state.dir).into()]);
    if !state.hidden {
        commands.call(Function::EnableNpc, vec![id.into()]);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;

    use script_sdk_2::MockTransport;

    use super::*;
    use crate::tests::{config_json, settings};

    const FIRST_ID: i32 = 50_000;

    struct World {
        now: Rc<Cell<i64>>,
        users: Rc<Cell<i32>>,
        loaded: Rc<Cell<bool>>,
    }

    /// The controller of Prontera, its townsfolk ids starting at [`FIRST_ID`] once the town is `loaded`.
    fn world() -> (World, MockTransport) {
        let world = World { now: Rc::new(Cell::new(9_000_000)), users: Rc::new(Cell::new(1)), loaded: Rc::new(Cell::new(true)) };
        let (now, users, loaded) = (world.now.clone(), world.users.clone(), world.loaded.clone());
        let variables: RefCell<HashMap<String, Value>> = RefCell::new(HashMap::new());
        let config = config_json();
        let name = controller_name(find("prontera").unwrap());
        let transport = MockTransport::new(move |request| match request {
            Request::Read(variable) => Ok(variables.borrow().get(variable).cloned().unwrap_or_else(|| if variable.ends_with('$') { Value::String(String::new()) } else { Value::Number(0) })),
            Request::Write { name, value } => {
                variables.borrow_mut().insert(name.clone(), value.clone());
                Ok(Value::default())
            }
            Request::Call { function: Function::PluginConfig, .. } => Ok(Value::String(config.clone())),
            Request::Call { function: Function::StrNpcInfo, .. } => Ok(Value::String(name.clone())),
            Request::Call { function: Function::GetTimeTick, .. } => Ok(Value::Number(now.get() as u32 as i32)),
            Request::Call { function: Function::GetMapUsers, .. } => Ok(Value::Number(users.get())),
            Request::Call { function: Function::GetNpcId, arguments } => {
                let Value::Array(names) = &arguments[1] else { return Err("names".into()) };
                Ok(Value::Array((0..names.len()).map(|index| Value::Number(if loaded.get() { FIRST_ID + index as i32 } else { 0 })).collect()))
            }
            _ => Ok(Value::default()),
        });
        (world, transport)
    }

    fn placed(transport: &MockTransport, since: usize) -> usize {
        transport.requests()[since..].iter().filter(|request| matches!(request, Request::Call { function: Function::MoveNpc, arguments } if arguments[1] != Value::Number(-1))).count()
    }

    #[test]
    fn a_waking_town_is_put_in_place_then_follows_the_plan() {
        let settings = settings();
        let prontera = find("prontera").unwrap();
        let plan = Plan::new(&settings, prontera);
        let (world, transport) = world();
        let ctx = Ctx::new(&transport);

        tick(&ctx).unwrap();
        let moved = transport.calls(Function::MoveNpc);
        assert_eq!(moved.len(), 100);
        let first = plan.state_at(0, world.now.get());
        assert_eq!(moved[0][..3], [Value::Number(FIRST_ID), Value::Number(i32::from(first.cell.0)), Value::Number(i32::from(first.cell.1))]);
        assert_eq!(transport.calls(Function::InitNpcTimer), [vec![Value::String(controller_name(prontera))]]);

        let before = transport.requests().len();
        let last = world.now.get();
        world.now.set(last + 1_040);
        tick(&ctx).unwrap();
        let door = format!("{}::OnDoor", controller_name(prontera));
        let expected_transport = MockTransport::silent();
        let expected_ctx = Ctx::new(&expected_transport);
        let mut batch = expected_ctx.batch();
        for index in 0..100 {
            for (_, step) in plan.steps_between(index, last, last + 1_040) {
                send(&mut batch, FIRST_ID + index as i32, step, &door);
            }
        }
        assert!(!batch.is_empty());
        batch.send().unwrap();
        let expected: Vec<Request> = expected_transport.requests().into_iter().filter(|request| !matches!(request, Request::Batch(_))).collect();
        let sent: Vec<Request> = transport.requests()[before..]
            .iter()
            .filter(|request| matches!(request, Request::Call { function, .. } if matches!(function, Function::NpcWalkTo | Function::MoveNpc | Function::Emotion | Function::EnableNpc)))
            .cloned()
            .collect();
        assert_eq!(sent, expected);
        assert_eq!(transport.calls(Function::GetNpcId).len(), 1, "the ids are kept between runs");

        let before = transport.requests().len();
        world.now.set(world.now.get() + 6_000);
        tick(&ctx).unwrap();
        assert_eq!(placed(&transport, before), 100, "a long gap puts everyone back in place");
    }

    #[test]
    fn an_empty_town_lets_its_timer_lapse_until_a_player_enters() {
        let (world, transport) = world();
        let ctx = Ctx::new(&transport);
        tick(&ctx).unwrap();
        world.users.set(0);
        world.now.set(world.now.get() + 1_000);
        tick(&ctx).unwrap();
        assert_eq!(transport.calls(Function::InitNpcTimer).len(), 1, "not started again once empty");

        let prontera = Hook::MapEnter { char_id: 1, map: "prontera".into(), x: 150, y: 150 };
        entered(&ctx, &prontera).unwrap();
        entered(&ctx, &Hook::MapEnter { char_id: 1, map: "prt_fild08".into(), x: 1, y: 1 }).unwrap();
        assert_eq!(transport.calls(Function::InitNpcTimer).len(), 2);
        world.users.set(1);
        world.now.set(world.now.get() + 60_000);
        let before = transport.requests().len();
        tick(&ctx).unwrap();
        assert_eq!(placed(&transport, before), 100, "a town that wakes up is put in place");
    }

    #[test]
    fn ids_are_looked_up_again_until_the_town_is_loaded() {
        let (world, transport) = world();
        world.loaded.set(false);
        let ctx = Ctx::new(&transport);
        tick(&ctx).unwrap();
        assert!(transport.calls(Function::MoveNpc).is_empty());
        world.loaded.set(true);
        world.now.set(world.now.get() + 1_000);
        tick(&ctx).unwrap();
        assert_eq!(transport.calls(Function::GetNpcId).len(), 2);
        assert_eq!(transport.calls(Function::MoveNpc).len(), 100);
    }

    #[test]
    fn walks_through_doors_hide_the_walker_on_arrival() {
        let transport = MockTransport::new(|request| match request {
            Request::Arguments => Ok(Value::Array(vec![Value::Number(50_123)])),
            _ => Ok(Value::default()),
        });
        went_inside(&Ctx::new(&transport)).unwrap();
        assert_eq!(transport.calls(Function::DisableNpc), [vec![Value::Number(50_123)]]);
        let silent = MockTransport::silent();
        let ctx = Ctx::new(&silent);
        let mut batch = ctx.batch();
        send(&mut batch, 7, Step::Walk { to: (10, 12), door: true }, "Alive Towns_21::OnDoor");
        batch.send().unwrap();
        assert_eq!(silent.calls(Function::NpcWalkTo), [vec![Value::Number(10), Value::Number(12), Value::Number(7), Value::String("Alive Towns_21::OnDoor".into())]]);
        assert_eq!(controller_town("Alive Towns_21"), Some(21));
        assert_eq!(controller_town("Alive Towns"), None);
    }
}
