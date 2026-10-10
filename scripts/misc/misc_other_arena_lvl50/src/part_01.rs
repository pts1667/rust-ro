use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn lv50_waiting_room_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lv50_waiting_room(ctx: &Ctx) -> Script {
    lv50_waiting_room_body(ctx, Vec::new()).map(|_| ())
}

fn lv50_waiting_room_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Individual; Level 50 to 69"),
            Val::from(50),
            Val::from("Lv50 Waiting Room::OnStartArena"),
            Val::from(1),
            Val::from(1000),
            Val::from(50),
            Val::from(69),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv50_waiting_room_oninit(ctx: &Ctx) -> Script {
    lv50_waiting_room_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn lv50_waiting_room_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("force_1-1"), Val::from(99), Val::from(12)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnStart")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv50_waiting_room_onstartarena(ctx: &Ctx) -> Script {
    lv50_waiting_room_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn lv50_waiting_room_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv50_waiting_room_onstart(ctx: &Ctx) -> Script {
    lv50_waiting_room_onstart_body(ctx, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_onstart(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnStart, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer3000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer4000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer5000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer60000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer120000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer180000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer240000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer300000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer305000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer305000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer306000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer306000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer307000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer307000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer308000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer308000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer309000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer309000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer310000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer310000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer311000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer311000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer312000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer312000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer313000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer313000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer314000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer314000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimer315000(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimer315000, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_ontimeroff(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_onfailclearstage(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::OnFailClearStage, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on01_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On01Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on01_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On01End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on02_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On02Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on02_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On02End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on03_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On03Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on03_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On03End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on04_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On04Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on04_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On04End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on05_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On05Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on05_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On05End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on06_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On06Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on06_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On06End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on07_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On07Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on07_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On07End, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on08_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On08Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on09_start(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On09Start, Vec::new()).map(|_| ())
}

pub fn heel_and_toe_arena_on09_end(ctx: &Ctx) -> Script {
    heel_and_toe_arena_run(ctx, HeelAndToeArenaStep::On09End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena50Step {
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

fn arena_50_run(ctx: &Ctx, mut step: Arena50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena50Step::Start => {
                step = Arena50Step::OnReset01;
                continue 'machine;
            }
            Arena50Step::OnReset01 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02start#50::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_01_02#50")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02_03#50")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset02 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03start#50::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset03 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04start#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#50::OnReset")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset04 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_04_05#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05start#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#50::OnReset")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset05 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06start#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#50::OnReset")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset06 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07start#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#50::OnReset")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset07 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08start#50::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset08 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09start#50::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#50")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnReset09 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#50")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnStart => {
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#50::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_01#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01start#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnStart")])?;
                return Err(Stop::End);
            }
            Arena50Step::OnResetAll => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#50::OnReset")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_50(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_01(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset01, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_02(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset02, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_03(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset03, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_04(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset04, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_05(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset05, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_06(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset06, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_07(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset07, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_08(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset08, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_09(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnReset09, Vec::new()).map(|_| ())
}

pub fn arena_50_onstart(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_50_onreset_all(ctx: &Ctx) -> Script {
    arena_50_run(ctx, Arena50Step::OnResetAll, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080150Step {
    Start,
    OnTouch,
}

fn force_08_01_50_run(ctx: &Ctx, mut step: Force080150Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080150Step::Start => {
                step = Force080150Step::OnTouch;
                continue 'machine;
            }
            Force080150Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On01_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(40), Val::from(26)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_01_50(ctx: &Ctx) -> Script {
    force_08_01_50_run(ctx, Force080150Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_01_50_ontouch(ctx: &Ctx) -> Script {
    force_08_01_50_run(ctx, Force080150Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force010250Step {
    Start,
    OnTouch,
}

fn force_01_02_50_run(ctx: &Ctx, mut step: Force010250Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force010250Step::Start => {
                step = Force010250Step::OnTouch;
                continue 'machine;
            }
            Force010250Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On02_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(25), Val::from(69)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01_02_50(ctx: &Ctx) -> Script {
    force_01_02_50_run(ctx, Force010250Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01_02_50_ontouch(ctx: &Ctx) -> Script {
    force_01_02_50_run(ctx, Force010250Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force020350Step {
    Start,
    OnTouch,
}

fn force_02_03_50_run(ctx: &Ctx, mut step: Force020350Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force020350Step::Start => {
                step = Force020350Step::OnTouch;
                continue 'machine;
            }
            Force020350Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_02")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On03_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(25), Val::from(159)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02_03_50(ctx: &Ctx) -> Script {
    force_02_03_50_run(ctx, Force020350Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02_03_50_ontouch(ctx: &Ctx) -> Script {
    force_02_03_50_run(ctx, Force020350Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force030450Step {
    Start,
    OnTouch,
}

fn force_03_04_50_run(ctx: &Ctx, mut step: Force030450Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force030450Step::Start => {
                step = Force030450Step::OnTouch;
                continue 'machine;
            }
            Force030450Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On04_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(69), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03_04_50(ctx: &Ctx) -> Script {
    force_03_04_50_run(ctx, Force030450Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03_04_50_ontouch(ctx: &Ctx) -> Script {
    force_03_04_50_run(ctx, Force030450Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force040550Step {
    Start,
    OnTouch,
}

fn force_04_05_50_run(ctx: &Ctx, mut step: Force040550Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force040550Step::Start => {
                step = Force040550Step::OnTouch;
                continue 'machine;
            }
            Force040550Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On05_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(159), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04_05_50(ctx: &Ctx) -> Script {
    force_04_05_50_run(ctx, Force040550Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04_05_50_ontouch(ctx: &Ctx) -> Script {
    force_04_05_50_run(ctx, Force040550Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force050650Step {
    Start,
    OnTouch,
}

fn force_05_06_50_run(ctx: &Ctx, mut step: Force050650Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force050650Step::Start => {
                step = Force050650Step::OnTouch;
                continue 'machine;
            }
            Force050650Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On06_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(174), Val::from(130)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05_06_50(ctx: &Ctx) -> Script {
    force_05_06_50_run(ctx, Force050650Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05_06_50_ontouch(ctx: &Ctx) -> Script {
    force_05_06_50_run(ctx, Force050650Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force060750Step {
    Start,
    OnTouch,
}

fn force_06_07_50_run(ctx: &Ctx, mut step: Force060750Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force060750Step::Start => {
                step = Force060750Step::OnTouch;
                continue 'machine;
            }
            Force060750Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On07_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(174), Val::from(40)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06_07_50(ctx: &Ctx) -> Script {
    force_06_07_50_run(ctx, Force060750Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06_07_50_ontouch(ctx: &Ctx) -> Script {
    force_06_07_50_run(ctx, Force060750Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force070850Step {
    Start,
    OnTouch,
}

fn force_07_08_50_run(ctx: &Ctx, mut step: Force070850Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force070850Step::Start => {
                step = Force070850Step::OnTouch;
                continue 'machine;
            }
            Force070850Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On08_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(132), Val::from(26)])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#50")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07_08_50(ctx: &Ctx) -> Script {
    force_07_08_50_run(ctx, Force070850Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07_08_50_ontouch(ctx: &Ctx) -> Script {
    force_07_08_50_run(ctx, Force070850Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080950Step {
    Start,
    OnTouch,
}

fn force_08_09_50_run(ctx: &Ctx, mut step: Force080950Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080950Step::Start => {
                step = Force080950Step::OnTouch;
                continue 'machine;
            }
            Force080950Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On09_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_1-1"), Val::from(99), Val::from(82)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_09_50(ctx: &Ctx) -> Script {
    force_08_09_50_run(ctx, Force080950Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_09_50_ontouch(ctx: &Ctx) -> Script {
    force_08_09_50_run(ctx, Force080950Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExit50Step {
    Start,
    OnTouch,
}

fn force_exit_50_run(ctx: &Ctx, mut step: ForceExit50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExit50Step::Start => {
                step = ForceExit50Step::OnTouch;
                continue 'machine;
            }
            ForceExit50Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_50::OnEnable")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("prt_are_in"),
                        Val::from(22),
                        Val::from(191),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exit_50(ctx: &Ctx) -> Script {
    force_exit_50_run(ctx, ForceExit50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_exit_50_ontouch(ctx: &Ctx) -> Script {
    force_exit_50_run(ctx, ForceExit50Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01start50Step {
    Start,
    OnEnable,
}

fn force_01start_50_run(ctx: &Ctx, mut step: Force01start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01start50Step::Start => {
                step = Force01start50Step::OnEnable;
                continue 'machine;
            }
            Force01start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_50(ctx: &Ctx) -> Script {
    force_01start_50_run(ctx, Force01start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_50_onenable(ctx: &Ctx) -> Script {
    force_01start_50_run(ctx, Force01start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01ex50Step {
    Start,
    OnReset,
    OnSummonMob1,
    OnMyMobDead,
}

fn force_01ex_50_run(ctx: &Ctx, mut step: Force01ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01ex50Step::Start => {
                step = Force01ex50Step::OnReset;
                continue 'machine;
            }
            Force01ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_01ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01ex50Step::OnSummonMob1 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(26),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(15),
                        Val::from(25),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(30),
                        Val::from(25),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(31),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(24),
                        Val::from(19),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(28),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(18),
                        Val::from(23),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(24),
                        Val::from(25),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(18),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(20),
                        Val::from(18),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(35),
                        Val::from(31),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(35),
                        Val::from(28),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(35),
                        Val::from(25),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(35),
                        Val::from(21),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(16),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(15),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(14),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(17),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(18),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(19),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(29),
                        Val::from(20),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(16),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(19),
                        Val::from(16),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(32),
                        Val::from(19),
                        Val::from("Poisonous Toad"),
                        Val::from(1556),
                        Val::from(1),
                        Val::from("force_01ex#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01ex_50(ctx: &Ctx) -> Script {
    force_01ex_50_run(ctx, Force01ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01ex_50_onreset(ctx: &Ctx) -> Script {
    force_01ex_50_run(ctx, Force01ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01ex_50_onsummonmob1(ctx: &Ctx) -> Script {
    force_01ex_50_run(ctx, Force01ex50Step::OnSummonMob1, Vec::new()).map(|_| ())
}

pub fn force_01ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_01ex_50_run(ctx, Force01ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_01mob_50_run(ctx: &Ctx, mut step: Force01mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mob50Step::Start => {
                step = Force01mob50Step::OnEnable;
                continue 'machine;
            }
            Force01mob50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#50::OnSummonMob1")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(25),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_01mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(18),
                        Val::from(25),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_01mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(32),
                        Val::from(25),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_01mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(32),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_01mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(18),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_01mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_01mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_01mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On01_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_01")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_50(ctx: &Ctx) -> Script {
    force_01mob_50_run(ctx, Force01mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_50_onenable(ctx: &Ctx) -> Script {
    force_01mob_50_run(ctx, Force01mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_50_onreset(ctx: &Ctx) -> Script {
    force_01mob_50_run(ctx, Force01mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_50_run(ctx, Force01mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02start50Step {
    Start,
    OnEnable,
}

fn force_02start_50_run(ctx: &Ctx, mut step: Force02start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02start50Step::Start => {
                step = Force02start50Step::OnEnable;
                continue 'machine;
            }
            Force02start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#50::OnSummonMob2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_50(ctx: &Ctx) -> Script {
    force_02start_50_run(ctx, Force02start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_50_onenable(ctx: &Ctx) -> Script {
    force_02start_50_run(ctx, Force02start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mob50Step {
    Start,
    OnReset,
    OnSummonMob2,
    OnMyMobDead,
}

fn force_02mob_50_run(ctx: &Ctx, mut step: Force02mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mob50Step::Start => {
                step = Force02mob50Step::OnReset;
                continue 'machine;
            }
            Force02mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_02mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mob50Step::OnSummonMob2 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(24),
                        Val::from(76),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(28),
                        Val::from(76),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(24),
                        Val::from(86),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(86),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(25),
                        Val::from(100),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(118),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(28),
                        Val::from(79),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(87),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(28),
                        Val::from(99),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(112),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Mummy"),
                        Val::from(1393),
                        Val::from(1),
                        Val::from("force_02mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mob50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_50(ctx: &Ctx) -> Script {
    force_02mob_50_run(ctx, Force02mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_50_onreset(ctx: &Ctx) -> Script {
    force_02mob_50_run(ctx, Force02mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_50_onsummonmob2(ctx: &Ctx) -> Script {
    force_02mob_50_run(ctx, Force02mob50Step::OnSummonMob2, Vec::new()).map(|_| ())
}

pub fn force_02mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_50_run(ctx, Force02mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03start50Step {
    Start,
    OnEnable,
}

fn force_03start_50_run(ctx: &Ctx, mut step: Force03start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03start50Step::Start => {
                step = Force03start50Step::OnEnable;
                continue 'machine;
            }
            Force03start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03start_50(ctx: &Ctx) -> Script {
    force_03start_50_run(ctx, Force03start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_50_onenable(ctx: &Ctx) -> Script {
    force_03start_50_run(ctx, Force03start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03ex50Step {
    Start,
    OnReset,
    OnSummonMob03,
    OnMyMobDead,
}

fn force_03ex_50_run(ctx: &Ctx, mut step: Force03ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03ex50Step::Start => {
                step = Force03ex50Step::OnReset;
                continue 'machine;
            }
            Force03ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_03ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03ex50Step::OnSummonMob03 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(21),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(31),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(186),
                        Val::from("Marse"),
                        Val::from(1551),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(161),
                        Val::from("Marse"),
                        Val::from(1551),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(13),
                        Val::from(173),
                        Val::from("Marse"),
                        Val::from(1551),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(38),
                        Val::from(173),
                        Val::from("Marse"),
                        Val::from(1551),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(29),
                        Val::from(174),
                        Val::from("Marse"),
                        Val::from(1551),
                        Val::from(1),
                        Val::from("force_03ex#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03ex_50(ctx: &Ctx) -> Script {
    force_03ex_50_run(ctx, Force03ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03ex_50_onreset(ctx: &Ctx) -> Script {
    force_03ex_50_run(ctx, Force03ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03ex_50_onsummonmob_03(ctx: &Ctx) -> Script {
    force_03ex_50_run(ctx, Force03ex50Step::OnSummonMob03, Vec::new()).map(|_| ())
}

pub fn force_03ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_03ex_50_run(ctx, Force03ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_50_run(ctx: &Ctx, mut step: Force03mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mob50Step::Start => {
                step = Force03mob50Step::OnEnable;
                continue 'machine;
            }
            Force03mob50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#50::OnSummonMob_03")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(23),
                        Val::from(174),
                        Val::from("Karakasa"),
                        Val::from(1544),
                        Val::from(1),
                        Val::from("force_03mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(18),
                        Val::from(173),
                        Val::from("Karakasa"),
                        Val::from(1544),
                        Val::from(1),
                        Val::from("force_03mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(33),
                        Val::from(173),
                        Val::from("Karakasa"),
                        Val::from(1544),
                        Val::from(1),
                        Val::from("force_03mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(181),
                        Val::from("Karakasa"),
                        Val::from(1544),
                        Val::from(1),
                        Val::from("force_03mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(26),
                        Val::from(166),
                        Val::from("Karakasa"),
                        Val::from(1544),
                        Val::from(1),
                        Val::from("force_03mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_03mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_03mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On03_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_03")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_50(ctx: &Ctx) -> Script {
    force_03mob_50_run(ctx, Force03mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_50_onenable(ctx: &Ctx) -> Script {
    force_03mob_50_run(ctx, Force03mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_50_onreset(ctx: &Ctx) -> Script {
    force_03mob_50_run(ctx, Force03mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_50_run(ctx, Force03mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04start50Step {
    Start,
    OnEnable,
}

fn force_04start_50_run(ctx: &Ctx, mut step: Force04start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04start50Step::Start => {
                step = Force04start50Step::OnEnable;
                continue 'machine;
            }
            Force04start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_50(ctx: &Ctx) -> Script {
    force_04start_50_run(ctx, Force04start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_50_onenable(ctx: &Ctx) -> Script {
    force_04start_50_run(ctx, Force04start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04ex50Step {
    Start,
    OnReset,
    OnSummonMob04,
    OnMyMobDead,
}

fn force_04ex_50_run(ctx: &Ctx, mut step: Force04ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04ex50Step::Start => {
                step = Force04ex50Step::OnReset;
                continue 'machine;
            }
            Force04ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_04ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04ex50Step::OnSummonMob04 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(99),
                        Val::from(174),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(103),
                        Val::from(174),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(107),
                        Val::from(174),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(111),
                        Val::from(176),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(115),
                        Val::from(176),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(119),
                        Val::from(172),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(95),
                        Val::from(178),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(100),
                        Val::from(178),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(105),
                        Val::from(172),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(110),
                        Val::from(172),
                        Val::from("Flora"),
                        Val::from(1575),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(115),
                        Val::from(172),
                        Val::from("Flora"),
                        Val::from(1575),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(120),
                        Val::from(172),
                        Val::from("Flora"),
                        Val::from(1575),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(125),
                        Val::from(172),
                        Val::from("Flora"),
                        Val::from(1575),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(105),
                        Val::from(178),
                        Val::from("Flora"),
                        Val::from(1575),
                        Val::from(1),
                        Val::from("force_04ex#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04ex_50(ctx: &Ctx) -> Script {
    force_04ex_50_run(ctx, Force04ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04ex_50_onreset(ctx: &Ctx) -> Script {
    force_04ex_50_run(ctx, Force04ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04ex_50_onsummonmob_04(ctx: &Ctx) -> Script {
    force_04ex_50_run(ctx, Force04ex50Step::OnSummonMob04, Vec::new()).map(|_| ())
}

pub fn force_04ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_04ex_50_run(ctx, Force04ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}
