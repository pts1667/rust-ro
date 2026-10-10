use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn lv80_waiting_room_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lv80_waiting_room(ctx: &Ctx) -> Script {
    lv80_waiting_room_body(ctx, Vec::new()).map(|_| ())
}

fn lv80_waiting_room_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            (Val::from("Individual; Level 80 to ") + ctx.constant("MAX_LEVEL")?),
            Val::from(50),
            Val::from("Lv80 Waiting Room::OnStartArena"),
            Val::from(1),
            Val::from(1000),
            Val::from(80),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv80_waiting_room_oninit(ctx: &Ctx) -> Script {
    lv80_waiting_room_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn lv80_waiting_room_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("force_4-1"), Val::from(99), Val::from(12)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnStart")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv80_waiting_room_onstartarena(ctx: &Ctx) -> Script {
    lv80_waiting_room_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn lv80_waiting_room_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv80_waiting_room_onstart(ctx: &Ctx) -> Script {
    lv80_waiting_room_onstart_body(ctx, Vec::new()).map(|_| ())
}

pub fn octus_arena(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_onstart(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnStart, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer3000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer4000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer8000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer60000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer120000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer180000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer240000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer300000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer360000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer420000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer420000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer480000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer480000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer485000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer485000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer486000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer486000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer487000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer487000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer488000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer488000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer489000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer489000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer490000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer490000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer491000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer491000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer492000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer492000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer493000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer493000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer494000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer494000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimer495000(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimer495000, Vec::new()).map(|_| ())
}

pub fn octus_arena_ontimeroff(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn octus_arena_onfailclearstage(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::OnFailClearStage, Vec::new()).map(|_| ())
}

pub fn octus_arena_on01_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On01Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on01_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On01End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on02_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On02Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on02_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On02End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on03_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On03Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on03_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On03End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on04_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On04Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on04_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On04End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on05_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On05Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on05_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On05End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on06_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On06Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on06_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On06End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on07_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On07Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on07_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On07End, Vec::new()).map(|_| ())
}

pub fn octus_arena_on08_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On08Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on09_start(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On09Start, Vec::new()).map(|_| ())
}

pub fn octus_arena_on09_end(ctx: &Ctx) -> Script {
    octus_arena_run(ctx, OctusArenaStep::On09End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena80Step {
    Start,
    OnReset01,
    OnReset02,
    OnReset03,
    OnReset04,
    OnReset05,
    OnReset06,
    OnReset07,
    OnReset08,
    OnReset09,
    OnStart,
    OnResetAll,
}

fn arena_80_run(ctx: &Ctx, mut step: Arena80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena80Step::Start => {
                step = Arena80Step::OnReset01;
                continue 'machine;
            }
            Arena80Step::OnReset01 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02start#80::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_01_02#80")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset02 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03start#80::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02_03#80")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset03 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04start#80::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#80::OnReset")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset04 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_04_05#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05start#80::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#80::OnReset")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset05 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06start#80::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#80::OnReset")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset06 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07start#80::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset07 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08start#80::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset08 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09start#80::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#80")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnReset09 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#80")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnStart => {
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#80::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_01#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01start#80::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnStart")])?;
                return Err(Stop::End);
            }
            Arena80Step::OnResetAll => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#80::OnReset")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_80(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_01(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset01, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_02(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset02, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_03(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset03, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_04(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset04, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_05(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset05, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_06(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset06, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_07(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset07, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_08(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset08, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_09(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnReset09, Vec::new()).map(|_| ())
}

pub fn arena_80_onstart(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_80_onreset_all(ctx: &Ctx) -> Script {
    arena_80_run(ctx, Arena80Step::OnResetAll, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080180Step {
    Start,
    OnTouch,
}

fn force_08_01_80_run(ctx: &Ctx, mut step: Force080180Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080180Step::Start => {
                step = Force080180Step::OnTouch;
                continue 'machine;
            }
            Force080180Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On01_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(40), Val::from(26)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_01_80(ctx: &Ctx) -> Script {
    force_08_01_80_run(ctx, Force080180Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_01_80_ontouch(ctx: &Ctx) -> Script {
    force_08_01_80_run(ctx, Force080180Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force010280Step {
    Start,
    OnTouch,
}

fn force_01_02_80_run(ctx: &Ctx, mut step: Force010280Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force010280Step::Start => {
                step = Force010280Step::OnTouch;
                continue 'machine;
            }
            Force010280Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On02_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(25), Val::from(69)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01_02_80(ctx: &Ctx) -> Script {
    force_01_02_80_run(ctx, Force010280Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01_02_80_ontouch(ctx: &Ctx) -> Script {
    force_01_02_80_run(ctx, Force010280Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force020380Step {
    Start,
    OnTouch,
}

fn force_02_03_80_run(ctx: &Ctx, mut step: Force020380Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force020380Step::Start => {
                step = Force020380Step::OnTouch;
                continue 'machine;
            }
            Force020380Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On03_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(25), Val::from(159)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02_03_80(ctx: &Ctx) -> Script {
    force_02_03_80_run(ctx, Force020380Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02_03_80_ontouch(ctx: &Ctx) -> Script {
    force_02_03_80_run(ctx, Force020380Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force030480Step {
    Start,
    OnTouch,
}

fn force_03_04_80_run(ctx: &Ctx, mut step: Force030480Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force030480Step::Start => {
                step = Force030480Step::OnTouch;
                continue 'machine;
            }
            Force030480Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On04_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(69), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03_04_80(ctx: &Ctx) -> Script {
    force_03_04_80_run(ctx, Force030480Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03_04_80_ontouch(ctx: &Ctx) -> Script {
    force_03_04_80_run(ctx, Force030480Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force040580Step {
    Start,
    OnTouch,
}

fn force_04_05_80_run(ctx: &Ctx, mut step: Force040580Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force040580Step::Start => {
                step = Force040580Step::OnTouch;
                continue 'machine;
            }
            Force040580Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On05_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(159), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04_05_80(ctx: &Ctx) -> Script {
    force_04_05_80_run(ctx, Force040580Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04_05_80_ontouch(ctx: &Ctx) -> Script {
    force_04_05_80_run(ctx, Force040580Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force050680Step {
    Start,
    OnTouch,
}

fn force_05_06_80_run(ctx: &Ctx, mut step: Force050680Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force050680Step::Start => {
                step = Force050680Step::OnTouch;
                continue 'machine;
            }
            Force050680Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On06_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(174), Val::from(130)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05_06_80(ctx: &Ctx) -> Script {
    force_05_06_80_run(ctx, Force050680Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05_06_80_ontouch(ctx: &Ctx) -> Script {
    force_05_06_80_run(ctx, Force050680Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force060780Step {
    Start,
    OnTouch,
}

fn force_06_07_80_run(ctx: &Ctx, mut step: Force060780Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force060780Step::Start => {
                step = Force060780Step::OnTouch;
                continue 'machine;
            }
            Force060780Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On07_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(174), Val::from(40)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06_07_80(ctx: &Ctx) -> Script {
    force_06_07_80_run(ctx, Force060780Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06_07_80_ontouch(ctx: &Ctx) -> Script {
    force_06_07_80_run(ctx, Force060780Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force070880Step {
    Start,
    OnTouch,
}

fn force_07_08_80_run(ctx: &Ctx, mut step: Force070880Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force070880Step::Start => {
                step = Force070880Step::OnTouch;
                continue 'machine;
            }
            Force070880Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On08_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(132), Val::from(26)])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#80")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07_08_80(ctx: &Ctx) -> Script {
    force_07_08_80_run(ctx, Force070880Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07_08_80_ontouch(ctx: &Ctx) -> Script {
    force_07_08_80_run(ctx, Force070880Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080980Step {
    Start,
    OnTouch,
}

fn force_08_09_80_run(ctx: &Ctx, mut step: Force080980Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080980Step::Start => {
                step = Force080980Step::OnTouch;
                continue 'machine;
            }
            Force080980Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On09_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_4-1"), Val::from(99), Val::from(82)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_09_80(ctx: &Ctx) -> Script {
    force_08_09_80_run(ctx, Force080980Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_09_80_ontouch(ctx: &Ctx) -> Script {
    force_08_09_80_run(ctx, Force080980Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExit80Step {
    Start,
    OnTouch,
}

fn force_exit_80_run(ctx: &Ctx, mut step: ForceExit80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExit80Step::Start => {
                step = ForceExit80Step::OnTouch;
                continue 'machine;
            }
            ForceExit80Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_80::OnEnable")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("prt_are_in"),
                        Val::from(73),
                        Val::from(192),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exit_80(ctx: &Ctx) -> Script {
    force_exit_80_run(ctx, ForceExit80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_exit_80_ontouch(ctx: &Ctx) -> Script {
    force_exit_80_run(ctx, ForceExit80Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01start80Step {
    Start,
    OnEnable,
}

fn force_01start_80_run(ctx: &Ctx, mut step: Force01start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01start80Step::Start => {
                step = Force01start80Step::OnEnable;
                continue 'machine;
            }
            Force01start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_80(ctx: &Ctx) -> Script {
    force_01start_80_run(ctx, Force01start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_80_onenable(ctx: &Ctx) -> Script {
    force_01start_80_run(ctx, Force01start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01ex80Step {
    Start,
    OnReset,
    OnSummonMob1,
    OnMyMobDead,
}

fn force_01ex_80_run(ctx: &Ctx, mut step: Force01ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01ex80Step::Start => {
                step = Force01ex80Step::OnReset;
                continue 'machine;
            }
            Force01ex80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_01ex#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01ex80Step::OnSummonMob1 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(11),
                        Val::from(25),
                        Val::from("Hunter Fly"),
                        Val::from(1422),
                        Val::from(1),
                        Val::from("force_01ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(22),
                        Val::from(22),
                        Val::from("Hunter Fly"),
                        Val::from(1422),
                        Val::from(1),
                        Val::from("force_01ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(25),
                        Val::from(25),
                        Val::from("Hunter Fly"),
                        Val::from(1422),
                        Val::from(1),
                        Val::from("force_01ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(35),
                        Val::from(13),
                        Val::from("Hunter Fly"),
                        Val::from(1422),
                        Val::from(1),
                        Val::from("force_01ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(18),
                        Val::from(33),
                        Val::from("Hunter Fly"),
                        Val::from(1422),
                        Val::from(1),
                        Val::from("force_01ex#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01ex80Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01ex_80(ctx: &Ctx) -> Script {
    force_01ex_80_run(ctx, Force01ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01ex_80_onreset(ctx: &Ctx) -> Script {
    force_01ex_80_run(ctx, Force01ex80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01ex_80_onsummonmob1(ctx: &Ctx) -> Script {
    force_01ex_80_run(ctx, Force01ex80Step::OnSummonMob1, Vec::new()).map(|_| ())
}

pub fn force_01ex_80_onmymobdead(ctx: &Ctx) -> Script {
    force_01ex_80_run(ctx, Force01ex80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_01mob_80_run(ctx: &Ctx, mut step: Force01mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mob80Step::Start => {
                step = Force01mob80Step::OnEnable;
                continue 'machine;
            }
            Force01mob80Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(35),
                        Val::from(13),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_01mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(25),
                        Val::from(36),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_01mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(22),
                        Val::from(23),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_01mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(25),
                        Val::from(17),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_01mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(25),
                        Val::from(15),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_01mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#80::OnSummonMob1")])?;
                return Err(Stop::End);
            }
            Force01mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_01mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_01mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On01_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_01")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_80(ctx: &Ctx) -> Script {
    force_01mob_80_run(ctx, Force01mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_80_onenable(ctx: &Ctx) -> Script {
    force_01mob_80_run(ctx, Force01mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_80_onreset(ctx: &Ctx) -> Script {
    force_01mob_80_run(ctx, Force01mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_80_run(ctx, Force01mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02start80Step {
    Start,
    OnEnable,
}

fn force_02start_80_run(ctx: &Ctx, mut step: Force02start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02start80Step::Start => {
                step = Force02start80Step::OnEnable;
                continue 'machine;
            }
            Force02start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_80(ctx: &Ctx) -> Script {
    force_02start_80_run(ctx, Force02start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_80_onenable(ctx: &Ctx) -> Script {
    force_02start_80_run(ctx, Force02start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mob80Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_02mob_80_run(ctx: &Ctx, mut step: Force02mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mob80Step::Start => {
                step = Force02mob80Step::OnReset;
                continue 'machine;
            }
            Force02mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_02mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mob80Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(24),
                        Val::from(80),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(23),
                        Val::from(110),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(26),
                        Val::from(90),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(28),
                        Val::from(75),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(24),
                        Val::from(90),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(28),
                        Val::from(85),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(24),
                        Val::from(82),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(28),
                        Val::from(86),
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(27),
                        Val::from(73),
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(26),
                        Val::from(118),
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(25),
                        Val::from(127),
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(27),
                        Val::from(100),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(29),
                        Val::from(90),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Raydric Archer"),
                        Val::from(1453),
                        Val::from(1),
                        Val::from("force_02mob#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_02mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On02_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_02")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_80(ctx: &Ctx) -> Script {
    force_02mob_80_run(ctx, Force02mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_80_onreset(ctx: &Ctx) -> Script {
    force_02mob_80_run(ctx, Force02mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_80_onenable(ctx: &Ctx) -> Script {
    force_02mob_80_run(ctx, Force02mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_02mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_80_run(ctx, Force02mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03start80Step {
    Start,
    OnEnable,
}

fn force_03start_80_run(ctx: &Ctx, mut step: Force03start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03start80Step::Start => {
                step = Force03start80Step::OnEnable;
                continue 'machine;
            }
            Force03start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03start_80(ctx: &Ctx) -> Script {
    force_03start_80_run(ctx, Force03start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_80_onenable(ctx: &Ctx) -> Script {
    force_03start_80_run(ctx, Force03start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03ex80Step {
    Start,
    OnReset,
    OnSummonMob03,
    OnMyMobDead,
}

fn force_03ex_80_run(ctx: &Ctx, mut step: Force03ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03ex80Step::Start => {
                step = Force03ex80Step::OnReset;
                continue 'machine;
            }
            Force03ex80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_03ex#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03ex80Step::OnSummonMob03 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(26),
                        Val::from(173),
                        Val::from("Clock Tower Keeper"),
                        Val::from(1527),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(21),
                        Val::from(173),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(31),
                        Val::from(173),
                        Val::from("Clock Tower Keeper"),
                        Val::from(1527),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(26),
                        Val::from(178),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(21),
                        Val::from(178),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(31),
                        Val::from(178),
                        Val::from("Marionette"),
                        Val::from(1459),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(26),
                        Val::from(186),
                        Val::from("Sidewinder"),
                        Val::from(1424),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(26),
                        Val::from(161),
                        Val::from("Sidewinder"),
                        Val::from(1424),
                        Val::from(1),
                        Val::from("force_03ex#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03ex80Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03ex_80(ctx: &Ctx) -> Script {
    force_03ex_80_run(ctx, Force03ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03ex_80_onreset(ctx: &Ctx) -> Script {
    force_03ex_80_run(ctx, Force03ex80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03ex_80_onsummonmob_03(ctx: &Ctx) -> Script {
    force_03ex_80_run(ctx, Force03ex80Step::OnSummonMob03, Vec::new()).map(|_| ())
}

pub fn force_03ex_80_onmymobdead(ctx: &Ctx) -> Script {
    force_03ex_80_run(ctx, Force03ex80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_80_run(ctx: &Ctx, mut step: Force03mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mob80Step::Start => {
                step = Force03mob80Step::OnEnable;
                continue 'machine;
            }
            Force03mob80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#80::OnSummonMob_03")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(23),
                        Val::from(174),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_03mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(18),
                        Val::from(173),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_03mob#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_03mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mob80Step::OnMyMobDead => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#80::OnSummonMob_03")])?;
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_03mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On03_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_03")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_80(ctx: &Ctx) -> Script {
    force_03mob_80_run(ctx, Force03mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_80_onenable(ctx: &Ctx) -> Script {
    force_03mob_80_run(ctx, Force03mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_80_onreset(ctx: &Ctx) -> Script {
    force_03mob_80_run(ctx, Force03mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_80_run(ctx, Force03mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04start80Step {
    Start,
    OnEnable,
}

fn force_04start_80_run(ctx: &Ctx, mut step: Force04start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04start80Step::Start => {
                step = Force04start80Step::OnEnable;
                continue 'machine;
            }
            Force04start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_80(ctx: &Ctx) -> Script {
    force_04start_80_run(ctx, Force04start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_80_onenable(ctx: &Ctx) -> Script {
    force_04start_80_run(ctx, Force04start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04ex80Step {
    Start,
    OnReset,
    OnSummonMob04,
    OnMyMobDead,
}

fn force_04ex_80_run(ctx: &Ctx, mut step: Force04ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04ex80Step::Start => {
                step = Force04ex80Step::OnReset;
                continue 'machine;
            }
            Force04ex80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_04ex#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04ex80Step::OnSummonMob04 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(99),
                        Val::from(174),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(101),
                        Val::from(174),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(103),
                        Val::from(174),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(105),
                        Val::from(174),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(107),
                        Val::from(174),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(109),
                        Val::from(177),
                        Val::from("Pasana"),
                        Val::from(1464),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(99),
                        Val::from(170),
                        Val::from("Minorous"),
                        Val::from(1461),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(111),
                        Val::from(170),
                        Val::from("Minorous"),
                        Val::from(1461),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(110),
                        Val::from(170),
                        Val::from("Lava Golem"),
                        Val::from(1549),
                        Val::from(1),
                        Val::from("force_04ex#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04ex80Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04ex_80(ctx: &Ctx) -> Script {
    force_04ex_80_run(ctx, Force04ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04ex_80_onreset(ctx: &Ctx) -> Script {
    force_04ex_80_run(ctx, Force04ex80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04ex_80_onsummonmob_04(ctx: &Ctx) -> Script {
    force_04ex_80_run(ctx, Force04ex80Step::OnSummonMob04, Vec::new()).map(|_| ())
}

pub fn force_04ex_80_onmymobdead(ctx: &Ctx) -> Script {
    force_04ex_80_run(ctx, Force04ex80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_04mob_80_run(ctx: &Ctx, mut step: Force04mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04mob80Step::Start => {
                step = Force04mob80Step::OnEnable;
                continue 'machine;
            }
            Force04mob80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#80::OnSummonMob_04")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(80),
                        Val::from(172),
                        Val::from("Nine Tails"),
                        Val::from(1471),
                        Val::from(1),
                        Val::from("force_04mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(105),
                        Val::from(172),
                        Val::from("Nine Tails"),
                        Val::from(1471),
                        Val::from(1),
                        Val::from("force_04mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(90),
                        Val::from(172),
                        Val::from("Nine Tails"),
                        Val::from(1471),
                        Val::from(1),
                        Val::from("force_04mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(117),
                        Val::from(172),
                        Val::from("Nine Tails"),
                        Val::from(1471),
                        Val::from(1),
                        Val::from("force_04mob#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_04mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_04mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On04_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_04")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04mob_80(ctx: &Ctx) -> Script {
    force_04mob_80_run(ctx, Force04mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04mob_80_onenable(ctx: &Ctx) -> Script {
    force_04mob_80_run(ctx, Force04mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_04mob_80_onreset(ctx: &Ctx) -> Script {
    force_04mob_80_run(ctx, Force04mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_04mob_80_run(ctx, Force04mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05start80Step {
    Start,
    OnEnable,
}

fn force_05start_80_run(ctx: &Ctx, mut step: Force05start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05start80Step::Start => {
                step = Force05start80Step::OnEnable;
                continue 'machine;
            }
            Force05start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_80(ctx: &Ctx) -> Script {
    force_05start_80_run(ctx, Force05start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_80_onenable(ctx: &Ctx) -> Script {
    force_05start_80_run(ctx, Force05start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05ex80Step {
    Start,
    OnReset,
    OnSummonMob05,
    OnMyMobDead,
}

fn force_05ex_80_run(ctx: &Ctx, mut step: Force05ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05ex80Step::Start => {
                step = Force05ex80Step::OnReset;
                continue 'machine;
            }
            Force05ex80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_05ex#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05ex80Step::OnSummonMob05 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(168),
                        Val::from(190),
                        Val::from("Petite"),
                        Val::from(1466),
                        Val::from(1),
                        Val::from("force_05ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(173),
                        Val::from(166),
                        Val::from("Petite"),
                        Val::from(1466),
                        Val::from(1),
                        Val::from("force_05ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(171),
                        Val::from(176),
                        Val::from("Petite"),
                        Val::from(1466),
                        Val::from(1),
                        Val::from("force_05ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(188),
                        Val::from(167),
                        Val::from("Petite"),
                        Val::from(1466),
                        Val::from(1),
                        Val::from("force_05ex#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05ex80Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05ex_80(ctx: &Ctx) -> Script {
    force_05ex_80_run(ctx, Force05ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05ex_80_onreset(ctx: &Ctx) -> Script {
    force_05ex_80_run(ctx, Force05ex80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05ex_80_onsummonmob_05(ctx: &Ctx) -> Script {
    force_05ex_80_run(ctx, Force05ex80Step::OnSummonMob05, Vec::new()).map(|_| ())
}

pub fn force_05ex_80_onmymobdead(ctx: &Ctx) -> Script {
    force_05ex_80_run(ctx, Force05ex80Step::OnMyMobDead, Vec::new()).map(|_| ())
}
