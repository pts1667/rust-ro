use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn lv70_waiting_room_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lv70_waiting_room(ctx: &Ctx) -> Script {
    lv70_waiting_room_body(ctx, Vec::new()).map(|_| ())
}

fn lv70_waiting_room_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Individual; Level 70 to 89"),
            Val::from(50),
            Val::from("Lv70 Waiting Room::OnStartArena"),
            Val::from(1),
            Val::from(1000),
            Val::from(70),
            Val::from(89),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv70_waiting_room_oninit(ctx: &Ctx) -> Script {
    lv70_waiting_room_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn lv70_waiting_room_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("force_3-1"), Val::from(99), Val::from(12)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnStart")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv70_waiting_room_onstartarena(ctx: &Ctx) -> Script {
    lv70_waiting_room_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn lv70_waiting_room_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv70_waiting_room_onstart(ctx: &Ctx) -> Script {
    lv70_waiting_room_onstart_body(ctx, Vec::new()).map(|_| ())
}

pub fn cadillac_arena(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_onstart(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnStart, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer3000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer4000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer7000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer60000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer120000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer180000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer240000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer300000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer360000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer420000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer420000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer425000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer425000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer426000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer426000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer427000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer427000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer428000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer428000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer429000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer429000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer430000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer430000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer431000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer431000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer432000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer432000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer433000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer433000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer434000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer434000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimer435000(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimer435000, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_ontimeroff(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_onfailclearstage(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::OnFailClearStage, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on01_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On01Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on01_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On01End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on02_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On02Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on02_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On02End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on03_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On03Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on03_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On03End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on04_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On04Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on04_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On04End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on05_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On05Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on05_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On05End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on06_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On06Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on06_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On06End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on07_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On07Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on07_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On07End, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on08_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On08Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on09_start(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On09Start, Vec::new()).map(|_| ())
}

pub fn cadillac_arena_on09_end(ctx: &Ctx) -> Script {
    cadillac_arena_run(ctx, CadillacArenaStep::On09End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena70Step {
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

fn arena_70_run(ctx: &Ctx, mut step: Arena70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena70Step::Start => {
                step = Arena70Step::OnReset01;
                continue 'machine;
            }
            Arena70Step::OnReset01 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02start#70::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_01_02#70")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset02 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03start#70::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02_03#70")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset03 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04start#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#70::OnReset")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset04 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_04_05#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05start#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#70::OnReset")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset05 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06start#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#70::OnReset")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset06 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07start#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#70::OnReset")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset07 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08start#70::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset08 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09start#70::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#70")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnReset09 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#70")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnStart => {
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#70::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_01#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01start#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnStart")])?;
                return Err(Stop::End);
            }
            Arena70Step::OnResetAll => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#70::OnReset")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_70(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_01(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset01, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_02(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset02, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_03(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset03, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_04(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset04, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_05(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset05, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_06(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset06, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_07(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset07, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_08(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset08, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_09(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnReset09, Vec::new()).map(|_| ())
}

pub fn arena_70_onstart(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_70_onreset_all(ctx: &Ctx) -> Script {
    arena_70_run(ctx, Arena70Step::OnResetAll, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080170Step {
    Start,
    OnTouch,
}

fn force_08_01_70_run(ctx: &Ctx, mut step: Force080170Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080170Step::Start => {
                step = Force080170Step::OnTouch;
                continue 'machine;
            }
            Force080170Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On01_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(40), Val::from(26)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_01_70(ctx: &Ctx) -> Script {
    force_08_01_70_run(ctx, Force080170Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_01_70_ontouch(ctx: &Ctx) -> Script {
    force_08_01_70_run(ctx, Force080170Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force010270Step {
    Start,
    OnTouch,
}

fn force_01_02_70_run(ctx: &Ctx, mut step: Force010270Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force010270Step::Start => {
                step = Force010270Step::OnTouch;
                continue 'machine;
            }
            Force010270Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On02_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(25), Val::from(69)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01_02_70(ctx: &Ctx) -> Script {
    force_01_02_70_run(ctx, Force010270Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01_02_70_ontouch(ctx: &Ctx) -> Script {
    force_01_02_70_run(ctx, Force010270Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force020370Step {
    Start,
    OnTouch,
}

fn force_02_03_70_run(ctx: &Ctx, mut step: Force020370Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force020370Step::Start => {
                step = Force020370Step::OnTouch;
                continue 'machine;
            }
            Force020370Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On03_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(25), Val::from(159)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02_03_70(ctx: &Ctx) -> Script {
    force_02_03_70_run(ctx, Force020370Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02_03_70_ontouch(ctx: &Ctx) -> Script {
    force_02_03_70_run(ctx, Force020370Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force030470Step {
    Start,
    OnTouch,
}

fn force_03_04_70_run(ctx: &Ctx, mut step: Force030470Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force030470Step::Start => {
                step = Force030470Step::OnTouch;
                continue 'machine;
            }
            Force030470Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On04_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(69), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03_04_70(ctx: &Ctx) -> Script {
    force_03_04_70_run(ctx, Force030470Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03_04_70_ontouch(ctx: &Ctx) -> Script {
    force_03_04_70_run(ctx, Force030470Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force040570Step {
    Start,
    OnTouch,
}

fn force_04_05_70_run(ctx: &Ctx, mut step: Force040570Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force040570Step::Start => {
                step = Force040570Step::OnTouch;
                continue 'machine;
            }
            Force040570Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On05_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(159), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04_05_70(ctx: &Ctx) -> Script {
    force_04_05_70_run(ctx, Force040570Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04_05_70_ontouch(ctx: &Ctx) -> Script {
    force_04_05_70_run(ctx, Force040570Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force050670Step {
    Start,
    OnTouch,
}

fn force_05_06_70_run(ctx: &Ctx, mut step: Force050670Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force050670Step::Start => {
                step = Force050670Step::OnTouch;
                continue 'machine;
            }
            Force050670Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On06_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(174), Val::from(130)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05_06_70(ctx: &Ctx) -> Script {
    force_05_06_70_run(ctx, Force050670Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05_06_70_ontouch(ctx: &Ctx) -> Script {
    force_05_06_70_run(ctx, Force050670Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force060770Step {
    Start,
    OnTouch,
}

fn force_06_07_70_run(ctx: &Ctx, mut step: Force060770Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force060770Step::Start => {
                step = Force060770Step::OnTouch;
                continue 'machine;
            }
            Force060770Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On07_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(174), Val::from(40)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06_07_70(ctx: &Ctx) -> Script {
    force_06_07_70_run(ctx, Force060770Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06_07_70_ontouch(ctx: &Ctx) -> Script {
    force_06_07_70_run(ctx, Force060770Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force070870Step {
    Start,
    OnTouch,
}

fn force_07_08_70_run(ctx: &Ctx, mut step: Force070870Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force070870Step::Start => {
                step = Force070870Step::OnTouch;
                continue 'machine;
            }
            Force070870Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On08_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(132), Val::from(26)])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#70")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07_08_70(ctx: &Ctx) -> Script {
    force_07_08_70_run(ctx, Force070870Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07_08_70_ontouch(ctx: &Ctx) -> Script {
    force_07_08_70_run(ctx, Force070870Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080970Step {
    Start,
    OnTouch,
}

fn force_08_09_70_run(ctx: &Ctx, mut step: Force080970Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080970Step::Start => {
                step = Force080970Step::OnTouch;
                continue 'machine;
            }
            Force080970Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On09_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_3-1"), Val::from(99), Val::from(82)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_09_70(ctx: &Ctx) -> Script {
    force_08_09_70_run(ctx, Force080970Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_09_70_ontouch(ctx: &Ctx) -> Script {
    force_08_09_70_run(ctx, Force080970Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExit70Step {
    Start,
    OnTouch,
}

fn force_exit_70_run(ctx: &Ctx, mut step: ForceExit70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExit70Step::Start => {
                step = ForceExit70Step::OnTouch;
                continue 'machine;
            }
            ForceExit70Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_70::OnEnable")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("prt_are_in"),
                        Val::from(22),
                        Val::from(87),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exit_70(ctx: &Ctx) -> Script {
    force_exit_70_run(ctx, ForceExit70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_exit_70_ontouch(ctx: &Ctx) -> Script {
    force_exit_70_run(ctx, ForceExit70Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01start70Step {
    Start,
    OnEnable,
}

fn force_01start_70_run(ctx: &Ctx, mut step: Force01start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01start70Step::Start => {
                step = Force01start70Step::OnEnable;
                continue 'machine;
            }
            Force01start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_70(ctx: &Ctx) -> Script {
    force_01start_70_run(ctx, Force01start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_70_onenable(ctx: &Ctx) -> Script {
    force_01start_70_run(ctx, Force01start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_01mob_70_run(ctx: &Ctx, mut step: Force01mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mob70Step::Start => {
                step = Force01mob70Step::OnEnable;
                continue 'machine;
            }
            Force01mob70Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(25),
                        Val::from(36),
                        Val::from("Kobold"),
                        Val::from(1545),
                        Val::from(1),
                        Val::from("force_01mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(20),
                        Val::from(36),
                        Val::from("Kobold"),
                        Val::from(1545),
                        Val::from(1),
                        Val::from("force_01mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(25),
                        Val::from(20),
                        Val::from("Kobold"),
                        Val::from(1546),
                        Val::from(1),
                        Val::from("force_01mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(30),
                        Val::from(36),
                        Val::from("Kobold"),
                        Val::from(1547),
                        Val::from(1),
                        Val::from("force_01mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(28),
                        Val::from(15),
                        Val::from("Kobold"),
                        Val::from(1547),
                        Val::from(1),
                        Val::from("force_01mob#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_01mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_01mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On01_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_01")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_70(ctx: &Ctx) -> Script {
    force_01mob_70_run(ctx, Force01mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_70_onenable(ctx: &Ctx) -> Script {
    force_01mob_70_run(ctx, Force01mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_70_onreset(ctx: &Ctx) -> Script {
    force_01mob_70_run(ctx, Force01mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_70_run(ctx, Force01mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02start70Step {
    Start,
    OnEnable,
}

fn force_02start_70_run(ctx: &Ctx, mut step: Force02start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02start70Step::Start => {
                step = Force02start70Step::OnEnable;
                continue 'machine;
            }
            Force02start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_70(ctx: &Ctx) -> Script {
    force_02start_70_run(ctx, Force02start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_70_onenable(ctx: &Ctx) -> Script {
    force_02start_70_run(ctx, Force02start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02ex70Step {
    Start,
    OnReset,
    OnSummonMob2,
    OnMyMobDead,
}

fn force_02ex_70_run(ctx: &Ctx, mut step: Force02ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02ex70Step::Start => {
                step = Force02ex70Step::OnReset;
                continue 'machine;
            }
            Force02ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_02ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02ex70Step::OnSummonMob2 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(21),
                        Val::from(78),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(22),
                        Val::from(93),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(29),
                        Val::from(93),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(25),
                        Val::from(101),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(26),
                        Val::from(101),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(27),
                        Val::from(101),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(28),
                        Val::from(101),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(24),
                        Val::from(104),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(24),
                        Val::from(113),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(29),
                        Val::from(120),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(29),
                        Val::from(126),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(30),
                        Val::from(110),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02ex#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02ex_70(ctx: &Ctx) -> Script {
    force_02ex_70_run(ctx, Force02ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02ex_70_onreset(ctx: &Ctx) -> Script {
    force_02ex_70_run(ctx, Force02ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02ex_70_onsummonmob2(ctx: &Ctx) -> Script {
    force_02ex_70_run(ctx, Force02ex70Step::OnSummonMob2, Vec::new()).map(|_| ())
}

pub fn force_02ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_02ex_70_run(ctx, Force02ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mob70Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_02mob_70_run(ctx: &Ctx, mut step: Force02mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mob70Step::Start => {
                step = Force02mob70Step::OnReset;
                continue 'machine;
            }
            Force02mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_02mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mob70Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(25),
                        Val::from(79),
                        Val::from("Horong"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_02mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(29),
                        Val::from(114),
                        Val::from("Horong"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_02mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02ex#70::OnSummonMob2")])?;
                return Err(Stop::End);
            }
            Force02mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_02mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On02_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_02")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_70(ctx: &Ctx) -> Script {
    force_02mob_70_run(ctx, Force02mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_70_onreset(ctx: &Ctx) -> Script {
    force_02mob_70_run(ctx, Force02mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_70_onenable(ctx: &Ctx) -> Script {
    force_02mob_70_run(ctx, Force02mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_02mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_70_run(ctx, Force02mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03start70Step {
    Start,
    OnEnable,
}

fn force_03start_70_run(ctx: &Ctx, mut step: Force03start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03start70Step::Start => {
                step = Force03start70Step::OnEnable;
                continue 'machine;
            }
            Force03start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03start_70(ctx: &Ctx) -> Script {
    force_03start_70_run(ctx, Force03start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_70_onenable(ctx: &Ctx) -> Script {
    force_03start_70_run(ctx, Force03start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03ex70Step {
    Start,
    OnReset,
    OnSummonMob03,
    OnMyMobDead,
}

fn force_03ex_70_run(ctx: &Ctx, mut step: Force03ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03ex70Step::Start => {
                step = Force03ex70Step::OnReset;
                continue 'machine;
            }
            Force03ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_03ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03ex70Step::OnSummonMob03 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(10),
                        Val::from(170),
                        Val::from("Enchanted Peach Tree"),
                        Val::from(1550),
                        Val::from(1),
                        Val::from("force_03ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(26),
                        Val::from(180),
                        Val::from("Enchanted Peach Tree"),
                        Val::from(1550),
                        Val::from(1),
                        Val::from("force_03ex#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03ex_70(ctx: &Ctx) -> Script {
    force_03ex_70_run(ctx, Force03ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03ex_70_onreset(ctx: &Ctx) -> Script {
    force_03ex_70_run(ctx, Force03ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03ex_70_onsummonmob_03(ctx: &Ctx) -> Script {
    force_03ex_70_run(ctx, Force03ex70Step::OnSummonMob03, Vec::new()).map(|_| ())
}

pub fn force_03ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_03ex_70_run(ctx, Force03ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_70_run(ctx: &Ctx, mut step: Force03mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mob70Step::Start => {
                step = Force03mob70Step::OnEnable;
                continue 'machine;
            }
            Force03mob70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#70::OnSummonMob_03")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(23),
                        Val::from(174),
                        Val::from("Parasite"),
                        Val::from(1555),
                        Val::from(1),
                        Val::from("force_03mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(33),
                        Val::from(173),
                        Val::from("Parasite"),
                        Val::from(1555),
                        Val::from(1),
                        Val::from("force_03mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(26),
                        Val::from(166),
                        Val::from("Blood Butterfly"),
                        Val::from(1526),
                        Val::from(1),
                        Val::from("force_03mob#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_03mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mob70Step::OnMyMobDead => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#70::OnSummonMob_03")])?;
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_03mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On03_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_03")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_70(ctx: &Ctx) -> Script {
    force_03mob_70_run(ctx, Force03mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_70_onenable(ctx: &Ctx) -> Script {
    force_03mob_70_run(ctx, Force03mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_70_onreset(ctx: &Ctx) -> Script {
    force_03mob_70_run(ctx, Force03mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_70_run(ctx, Force03mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04start70Step {
    Start,
    OnEnable,
}

fn force_04start_70_run(ctx: &Ctx, mut step: Force04start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04start70Step::Start => {
                step = Force04start70Step::OnEnable;
                continue 'machine;
            }
            Force04start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_70(ctx: &Ctx) -> Script {
    force_04start_70_run(ctx, Force04start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_70_onenable(ctx: &Ctx) -> Script {
    force_04start_70_run(ctx, Force04start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04ex70Step {
    Start,
    OnReset,
    OnSummonMob04,
    OnMyMobDead,
}

fn force_04ex_70_run(ctx: &Ctx, mut step: Force04ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04ex70Step::Start => {
                step = Force04ex70Step::OnReset;
                continue 'machine;
            }
            Force04ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_04ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04ex70Step::OnSummonMob04 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(94),
                        Val::from(179),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_04ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(110),
                        Val::from(179),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_04ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(90),
                        Val::from(170),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_04ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(100),
                        Val::from(170),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_04ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(125),
                        Val::from(178),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_04ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(125),
                        Val::from(169),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_04ex#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04ex_70(ctx: &Ctx) -> Script {
    force_04ex_70_run(ctx, Force04ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04ex_70_onreset(ctx: &Ctx) -> Script {
    force_04ex_70_run(ctx, Force04ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04ex_70_onsummonmob_04(ctx: &Ctx) -> Script {
    force_04ex_70_run(ctx, Force04ex70Step::OnSummonMob04, Vec::new()).map(|_| ())
}

pub fn force_04ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_04ex_70_run(ctx, Force04ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_04mob_70_run(ctx: &Ctx, mut step: Force04mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04mob70Step::Start => {
                step = Force04mob70Step::OnEnable;
                continue 'machine;
            }
            Force04mob70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#70::OnSummonMob_04")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(87),
                        Val::from(174),
                        Val::from("Stem Worm"),
                        Val::from(1440),
                        Val::from(1),
                        Val::from("force_04mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(103),
                        Val::from(174),
                        Val::from("Stem Worm"),
                        Val::from(1440),
                        Val::from(1),
                        Val::from("force_04mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(96),
                        Val::from(170),
                        Val::from("Stem Worm"),
                        Val::from(1440),
                        Val::from(1),
                        Val::from("force_04mob#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_04mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_04mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On04_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_04")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04mob_70(ctx: &Ctx) -> Script {
    force_04mob_70_run(ctx, Force04mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04mob_70_onenable(ctx: &Ctx) -> Script {
    force_04mob_70_run(ctx, Force04mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_04mob_70_onreset(ctx: &Ctx) -> Script {
    force_04mob_70_run(ctx, Force04mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_04mob_70_run(ctx, Force04mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05start70Step {
    Start,
    OnEnable,
}

fn force_05start_70_run(ctx: &Ctx, mut step: Force05start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05start70Step::Start => {
                step = Force05start70Step::OnEnable;
                continue 'machine;
            }
            Force05start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_70(ctx: &Ctx) -> Script {
    force_05start_70_run(ctx, Force05start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_70_onenable(ctx: &Ctx) -> Script {
    force_05start_70_run(ctx, Force05start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05ex70Step {
    Start,
    OnReset,
    OnSummonMob05,
    OnMyMobDead,
}

fn force_05ex_70_run(ctx: &Ctx, mut step: Force05ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05ex70Step::Start => {
                step = Force05ex70Step::OnReset;
                continue 'machine;
            }
            Force05ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_05ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05ex70Step::OnSummonMob05 => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_3-1"),
                            Val::from(174),
                            Val::from(174),
                            Val::from("Ride Word"),
                            Val::from(1478),
                            Val::from(1),
                            Val::from("force_05ex#70::OnMyMobDead"),
                        ],
                    )?;
                } else if subject1 == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_3-1"),
                            Val::from(173),
                            Val::from(173),
                            Val::from("Mantis"),
                            Val::from(1457),
                            Val::from(1),
                            Val::from("force_05ex#70::OnMyMobDead"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            Force05ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05ex_70(ctx: &Ctx) -> Script {
    force_05ex_70_run(ctx, Force05ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05ex_70_onreset(ctx: &Ctx) -> Script {
    force_05ex_70_run(ctx, Force05ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05ex_70_onsummonmob_05(ctx: &Ctx) -> Script {
    force_05ex_70_run(ctx, Force05ex70Step::OnSummonMob05, Vec::new()).map(|_| ())
}

pub fn force_05ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_05ex_70_run(ctx, Force05ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_05mob_70_run(ctx: &Ctx, mut step: Force05mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mob70Step::Start => {
                step = Force05mob70Step::OnEnable;
                continue 'machine;
            }
            Force05mob70Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(164),
                        Val::from(183),
                        Val::from("Argiope"),
                        Val::from(1429),
                        Val::from(1),
                        Val::from("force_05mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(168),
                        Val::from(158),
                        Val::from("Argiope"),
                        Val::from(1429),
                        Val::from(1),
                        Val::from("force_05mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(175),
                        Val::from(174),
                        Val::from("Argiope"),
                        Val::from(1429),
                        Val::from(1),
                        Val::from("force_05mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(176),
                        Val::from(179),
                        Val::from("Argiope"),
                        Val::from(1429),
                        Val::from(1),
                        Val::from("force_05mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(183),
                        Val::from(160),
                        Val::from("Argiope"),
                        Val::from(1429),
                        Val::from(1),
                        Val::from("force_05mob#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_05mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_05mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On05_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_05")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#70::OnSummonMob_05")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_70(ctx: &Ctx) -> Script {
    force_05mob_70_run(ctx, Force05mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_70_onenable(ctx: &Ctx) -> Script {
    force_05mob_70_run(ctx, Force05mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_70_onreset(ctx: &Ctx) -> Script {
    force_05mob_70_run(ctx, Force05mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_70_run(ctx, Force05mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06start70Step {
    Start,
    OnEnable,
}

fn force_06start_70_run(ctx: &Ctx, mut step: Force06start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06start70Step::Start => {
                step = Force06start70Step::OnEnable;
                continue 'machine;
            }
            Force06start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_70(ctx: &Ctx) -> Script {
    force_06start_70_run(ctx, Force06start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_70_onenable(ctx: &Ctx) -> Script {
    force_06start_70_run(ctx, Force06start70Step::OnEnable, Vec::new()).map(|_| ())
}
