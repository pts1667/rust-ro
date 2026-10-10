use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn lv60_waiting_room_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn lv60_waiting_room(ctx: &Ctx) -> Script {
    lv60_waiting_room_body(ctx, Vec::new()).map(|_| ())
}

fn lv60_waiting_room_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Individual; Level 60 to 79"),
            Val::from(50),
            Val::from("Lv60 Waiting Room::OnStartArena"),
            Val::from(1),
            Val::from(1000),
            Val::from(60),
            Val::from(79),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv60_waiting_room_oninit(ctx: &Ctx) -> Script {
    lv60_waiting_room_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn lv60_waiting_room_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("force_2-1"), Val::from(99), Val::from(12)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnStart")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv60_waiting_room_onstartarena(ctx: &Ctx) -> Script {
    lv60_waiting_room_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn lv60_waiting_room_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn lv60_waiting_room_onstart(ctx: &Ctx) -> Script {
    lv60_waiting_room_onstart_body(ctx, Vec::new()).map(|_| ())
}

pub fn minilover_arena(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_onstart(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnStart, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer3000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer4000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer5000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer60000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer120000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer180000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer240000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer300000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer360000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer365000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer365000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer366000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer366000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer367000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer367000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer368000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer368000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer369000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer369000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer370000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer370000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer371000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer371000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer372000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer372000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer373000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer373000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer374000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer374000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimer375000(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimer375000, Vec::new()).map(|_| ())
}

pub fn minilover_arena_ontimeroff(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn minilover_arena_onfailclearstage(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::OnFailClearStage, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on01_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On01Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on01_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On01End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on02_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On02Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on02_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On02End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on03_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On03Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on03_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On03End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on04_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On04Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on04_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On04End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on05_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On05Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on05_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On05End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on06_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On06Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on06_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On06End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on07_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On07Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on07_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On07End, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on08_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On08Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on09_start(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On09Start, Vec::new()).map(|_| ())
}

pub fn minilover_arena_on09_end(ctx: &Ctx) -> Script {
    minilover_arena_run(ctx, MiniloverArenaStep::On09End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Arena60Step {
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

fn arena_60_run(ctx: &Ctx, mut step: Arena60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Arena60Step::Start => {
                step = Arena60Step::OnReset01;
                continue 'machine;
            }
            Arena60Step::OnReset01 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02start#60::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_01_02#60")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02_03#60")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset02 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03start#60::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset03 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04start#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#60::OnReset")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset04 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_04_05#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05start#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#60::OnReset")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset05 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06start#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#60::OnReset")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset06 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07start#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#60::OnReset")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset07 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08start#60::OnEnable")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset08 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09start#60::OnEnable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#60")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnReset09 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#60")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnStart => {
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#60::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_01#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01start#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnStart")])?;
                return Err(Stop::End);
            }
            Arena60Step::OnResetAll => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#60::OnReset")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_60(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_01(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset01, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_02(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset02, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_03(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset03, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_04(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset04, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_05(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset05, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_06(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset06, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_07(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset07, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_08(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset08, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_09(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnReset09, Vec::new()).map(|_| ())
}

pub fn arena_60_onstart(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_60_onreset_all(ctx: &Ctx) -> Script {
    arena_60_run(ctx, Arena60Step::OnResetAll, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080160Step {
    Start,
    OnTouch,
}

fn force_08_01_60_run(ctx: &Ctx, mut step: Force080160Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080160Step::Start => {
                step = Force080160Step::OnTouch;
                continue 'machine;
            }
            Force080160Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On01_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(40), Val::from(26)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_01_60(ctx: &Ctx) -> Script {
    force_08_01_60_run(ctx, Force080160Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_01_60_ontouch(ctx: &Ctx) -> Script {
    force_08_01_60_run(ctx, Force080160Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force010260Step {
    Start,
    OnTouch,
}

fn force_01_02_60_run(ctx: &Ctx, mut step: Force010260Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force010260Step::Start => {
                step = Force010260Step::OnTouch;
                continue 'machine;
            }
            Force010260Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On02_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(25), Val::from(69)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01_02_60(ctx: &Ctx) -> Script {
    force_01_02_60_run(ctx, Force010260Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01_02_60_ontouch(ctx: &Ctx) -> Script {
    force_01_02_60_run(ctx, Force010260Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force020360Step {
    Start,
    OnTouch,
}

fn force_02_03_60_run(ctx: &Ctx, mut step: Force020360Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force020360Step::Start => {
                step = Force020360Step::OnTouch;
                continue 'machine;
            }
            Force020360Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_02")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On03_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(25), Val::from(159)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02_03_60(ctx: &Ctx) -> Script {
    force_02_03_60_run(ctx, Force020360Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02_03_60_ontouch(ctx: &Ctx) -> Script {
    force_02_03_60_run(ctx, Force020360Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force030460Step {
    Start,
    OnTouch,
}

fn force_03_04_60_run(ctx: &Ctx, mut step: Force030460Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force030460Step::Start => {
                step = Force030460Step::OnTouch;
                continue 'machine;
            }
            Force030460Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On04_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(69), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03_04_60(ctx: &Ctx) -> Script {
    force_03_04_60_run(ctx, Force030460Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03_04_60_ontouch(ctx: &Ctx) -> Script {
    force_03_04_60_run(ctx, Force030460Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force040560Step {
    Start,
    OnTouch,
}

fn force_04_05_60_run(ctx: &Ctx, mut step: Force040560Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force040560Step::Start => {
                step = Force040560Step::OnTouch;
                continue 'machine;
            }
            Force040560Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On05_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(159), Val::from(174)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04_05_60(ctx: &Ctx) -> Script {
    force_04_05_60_run(ctx, Force040560Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04_05_60_ontouch(ctx: &Ctx) -> Script {
    force_04_05_60_run(ctx, Force040560Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force050660Step {
    Start,
    OnTouch,
}

fn force_05_06_60_run(ctx: &Ctx, mut step: Force050660Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force050660Step::Start => {
                step = Force050660Step::OnTouch;
                continue 'machine;
            }
            Force050660Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On06_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(174), Val::from(130)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05_06_60(ctx: &Ctx) -> Script {
    force_05_06_60_run(ctx, Force050660Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05_06_60_ontouch(ctx: &Ctx) -> Script {
    force_05_06_60_run(ctx, Force050660Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force060760Step {
    Start,
    OnTouch,
}

fn force_06_07_60_run(ctx: &Ctx, mut step: Force060760Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force060760Step::Start => {
                step = Force060760Step::OnTouch;
                continue 'machine;
            }
            Force060760Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On07_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(174), Val::from(40)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06_07_60(ctx: &Ctx) -> Script {
    force_06_07_60_run(ctx, Force060760Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06_07_60_ontouch(ctx: &Ctx) -> Script {
    force_06_07_60_run(ctx, Force060760Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force070860Step {
    Start,
    OnTouch,
}

fn force_07_08_60_run(ctx: &Ctx, mut step: Force070860Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force070860Step::Start => {
                step = Force070860Step::OnTouch;
                continue 'machine;
            }
            Force070860Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On08_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(132), Val::from(26)])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#60")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07_08_60(ctx: &Ctx) -> Script {
    force_07_08_60_run(ctx, Force070860Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07_08_60_ontouch(ctx: &Ctx) -> Script {
    force_07_08_60_run(ctx, Force070860Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force080960Step {
    Start,
    OnTouch,
}

fn force_08_09_60_run(ctx: &Ctx, mut step: Force080960Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force080960Step::Start => {
                step = Force080960Step::OnTouch;
                continue 'machine;
            }
            Force080960Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On09_Start")])?;
                ctx.call(Function::Warp, vec![Val::from("force_2-1"), Val::from(99), Val::from(82)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_09_60(ctx: &Ctx) -> Script {
    force_08_09_60_run(ctx, Force080960Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08_09_60_ontouch(ctx: &Ctx) -> Script {
    force_08_09_60_run(ctx, Force080960Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExit60Step {
    Start,
    OnTouch,
}

fn force_exit_60_run(ctx: &Ctx, mut step: ForceExit60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExit60Step::Start => {
                step = ForceExit60Step::OnTouch;
                continue 'machine;
            }
            ForceExit60Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_60::OnEnable")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("prt_are_in"),
                        Val::from(22),
                        Val::from(139),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exit_60(ctx: &Ctx) -> Script {
    force_exit_60_run(ctx, ForceExit60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_exit_60_ontouch(ctx: &Ctx) -> Script {
    force_exit_60_run(ctx, ForceExit60Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01start60Step {
    Start,
    OnEnable,
}

fn force_01start_60_run(ctx: &Ctx, mut step: Force01start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01start60Step::Start => {
                step = Force01start60Step::OnEnable;
                continue 'machine;
            }
            Force01start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_60(ctx: &Ctx) -> Script {
    force_01start_60_run(ctx, Force01start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_60_onenable(ctx: &Ctx) -> Script {
    force_01start_60_run(ctx, Force01start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01ex60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_01ex_60_run(ctx: &Ctx, mut step: Force01ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01ex60Step::Start => {
                step = Force01ex60Step::OnReset;
                continue 'machine;
            }
            Force01ex60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_01ex#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01ex60Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(25),
                        Val::from("Rotar Zairo"),
                        Val::from(1392),
                        Val::from(1),
                        Val::from("force_01ex#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01ex60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01ex_60(ctx: &Ctx) -> Script {
    force_01ex_60_run(ctx, Force01ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01ex_60_onreset(ctx: &Ctx) -> Script {
    force_01ex_60_run(ctx, Force01ex60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01ex_60_onenable(ctx: &Ctx) -> Script {
    force_01ex_60_run(ctx, Force01ex60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01ex_60_onmymobdead(ctx: &Ctx) -> Script {
    force_01ex_60_run(ctx, Force01ex60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mob60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_01mob_60_run(ctx: &Ctx, mut step: Force01mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mob60Step::Start => {
                step = Force01mob60Step::OnReset;
                continue 'machine;
            }
            Force01mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_01mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mob60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#60::OnEnable")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(32),
                        Val::from("Goblin Archer"),
                        Val::from(1577),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(21),
                        Val::from(26),
                        Val::from("Goblin"),
                        Val::from(1534),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(36),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(15),
                        Val::from("Goblin"),
                        Val::from(1534),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(40),
                        Val::from(30),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(24),
                        Val::from("Goblin"),
                        Val::from(1534),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(9),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(15),
                        Val::from("Goblin Archer"),
                        Val::from(1577),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(12),
                        Val::from(33),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(24),
                        Val::from(20),
                        Val::from("Goblin"),
                        Val::from(1535),
                        Val::from(1),
                        Val::from("force_01mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_01mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 6
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On01_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_01")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_60(ctx: &Ctx) -> Script {
    force_01mob_60_run(ctx, Force01mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_60_onreset(ctx: &Ctx) -> Script {
    force_01mob_60_run(ctx, Force01mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_60_onenable(ctx: &Ctx) -> Script {
    force_01mob_60_run(ctx, Force01mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_60_run(ctx, Force01mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02start60Step {
    Start,
    OnEnable,
}

fn force_02start_60_run(ctx: &Ctx, mut step: Force02start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02start60Step::Start => {
                step = Force02start60Step::OnEnable;
                continue 'machine;
            }
            Force02start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_60(ctx: &Ctx) -> Script {
    force_02start_60_run(ctx, Force02start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_60_onenable(ctx: &Ctx) -> Script {
    force_02start_60_run(ctx, Force02start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mob60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_02mob_60_run(ctx: &Ctx, mut step: Force02mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mob60Step::Start => {
                step = Force02mob60Step::OnReset;
                continue 'machine;
            }
            Force02mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_02mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mob60Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(24),
                        Val::from(76),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(76),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(24),
                        Val::from(86),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(24),
                        Val::from(76),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(76),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(24),
                        Val::from(86),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(24),
                        Val::from(76),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(76),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(23),
                        Val::from(76),
                        Val::from("Requiem"),
                        Val::from(1468),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(27),
                        Val::from(76),
                        Val::from("Requiem"),
                        Val::from(1468),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(86),
                        Val::from("Requiem"),
                        Val::from(1468),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(86),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(100),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(118),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(25),
                        Val::from(100),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(118),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(79),
                        Val::from("Zerom"),
                        Val::from(1470),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(23),
                        Val::from(87),
                        Val::from("Zerom"),
                        Val::from(1470),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(99),
                        Val::from("Zerom"),
                        Val::from(1470),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(23),
                        Val::from(112),
                        Val::from("Zerom"),
                        Val::from(1470),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Matyr"),
                        Val::from(1460),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Matyr"),
                        Val::from(1460),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Matyr"),
                        Val::from(1460),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Orc Zombie"),
                        Val::from(1463),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Orc Zombie"),
                        Val::from(1463),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Orc Zombie"),
                        Val::from(1463),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Orc Zombie"),
                        Val::from(1463),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Orc Zombie"),
                        Val::from(1463),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Matyr"),
                        Val::from(1460),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(28),
                        Val::from(128),
                        Val::from("Matyr"),
                        Val::from(1460),
                        Val::from(1),
                        Val::from("force_02mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mob60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_60(ctx: &Ctx) -> Script {
    force_02mob_60_run(ctx, Force02mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_60_onreset(ctx: &Ctx) -> Script {
    force_02mob_60_run(ctx, Force02mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_60_onenable(ctx: &Ctx) -> Script {
    force_02mob_60_run(ctx, Force02mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_02mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_60_run(ctx, Force02mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03start60Step {
    Start,
    OnEnable,
}

fn force_03start_60_run(ctx: &Ctx, mut step: Force03start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03start60Step::Start => {
                step = Force03start60Step::OnEnable;
                continue 'machine;
            }
            Force03start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03start_60(ctx: &Ctx) -> Script {
    force_03start_60_run(ctx, Force03start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_60_onenable(ctx: &Ctx) -> Script {
    force_03start_60_run(ctx, Force03start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03ex60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_03ex_60_run(ctx: &Ctx, mut step: Force03ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03ex60Step::Start => {
                step = Force03ex60Step::OnReset;
                continue 'machine;
            }
            Force03ex60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_03ex#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03ex60Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(21),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(31),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(178),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(21),
                        Val::from(178),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(31),
                        Val::from(178),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(19),
                        Val::from(174),
                        Val::from("Obeaune"),
                        Val::from(1425),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(161),
                        Val::from("Obeaune"),
                        Val::from(1425),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(13),
                        Val::from(173),
                        Val::from("Obeaune"),
                        Val::from(1425),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(38),
                        Val::from(173),
                        Val::from("Obeaune"),
                        Val::from(1425),
                        Val::from(1),
                        Val::from("force_03ex#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03ex60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03ex_60(ctx: &Ctx) -> Script {
    force_03ex_60_run(ctx, Force03ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03ex_60_onreset(ctx: &Ctx) -> Script {
    force_03ex_60_run(ctx, Force03ex60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03ex_60_onenable(ctx: &Ctx) -> Script {
    force_03ex_60_run(ctx, Force03ex60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03ex_60_onmymobdead(ctx: &Ctx) -> Script {
    force_03ex_60_run(ctx, Force03ex60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mob60Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_60_run(ctx: &Ctx, mut step: Force03mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mob60Step::Start => {
                step = Force03mob60Step::OnEnable;
                continue 'machine;
            }
            Force03mob60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#60::OnEnable")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(23),
                        Val::from(174),
                        Val::from("Mantis"),
                        Val::from(1457),
                        Val::from(1),
                        Val::from("force_03mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(18),
                        Val::from(173),
                        Val::from("Mantis"),
                        Val::from(1457),
                        Val::from(1),
                        Val::from("force_03mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(33),
                        Val::from(173),
                        Val::from("Mantis"),
                        Val::from(1457),
                        Val::from(1),
                        Val::from("force_03mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(26),
                        Val::from(181),
                        Val::from("Mantis"),
                        Val::from(1457),
                        Val::from(1),
                        Val::from("force_03mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_mob01#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_03mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On03_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_03")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_60(ctx: &Ctx) -> Script {
    force_03mob_60_run(ctx, Force03mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_60_onenable(ctx: &Ctx) -> Script {
    force_03mob_60_run(ctx, Force03mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_60_onreset(ctx: &Ctx) -> Script {
    force_03mob_60_run(ctx, Force03mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_60_run(ctx, Force03mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04start60Step {
    Start,
    OnEnable,
}

fn force_04start_60_run(ctx: &Ctx, mut step: Force04start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04start60Step::Start => {
                step = Force04start60Step::OnEnable;
                continue 'machine;
            }
            Force04start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_60(ctx: &Ctx) -> Script {
    force_04start_60_run(ctx, Force04start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_60_onenable(ctx: &Ctx) -> Script {
    force_04start_60_run(ctx, Force04start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04ex60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_04ex_60_run(ctx: &Ctx, mut step: Force04ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04ex60Step::Start => {
                step = Force04ex60Step::OnReset;
                continue 'machine;
            }
            Force04ex60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_04ex#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04ex60Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(84),
                        Val::from(177),
                        Val::from("Sasquatch"),
                        Val::from(1442),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(125),
                        Val::from(170),
                        Val::from("Sasquatch"),
                        Val::from(1442),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(109),
                        Val::from(172),
                        Val::from("Baby Leopard"),
                        Val::from(1524),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(121),
                        Val::from(172),
                        Val::from("Baby Leopard"),
                        Val::from(1524),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(104),
                        Val::from(173),
                        Val::from("Chepet"),
                        Val::from(1444),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(75),
                        Val::from(174),
                        Val::from("Dokebi"),
                        Val::from(1491),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(102),
                        Val::from(176),
                        Val::from("Dokebi"),
                        Val::from(1491),
                        Val::from(1),
                        Val::from("force_04ex#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04ex60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04ex_60(ctx: &Ctx) -> Script {
    force_04ex_60_run(ctx, Force04ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04ex_60_onreset(ctx: &Ctx) -> Script {
    force_04ex_60_run(ctx, Force04ex60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04ex_60_onenable(ctx: &Ctx) -> Script {
    force_04ex_60_run(ctx, Force04ex60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_04ex_60_onmymobdead(ctx: &Ctx) -> Script {
    force_04ex_60_run(ctx, Force04ex60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04mob60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_04mob_60_run(ctx: &Ctx, mut step: Force04mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04mob60Step::Start => {
                step = Force04mob60Step::OnReset;
                continue 'machine;
            }
            Force04mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_04mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04mob60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#60::OnEnable")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(79),
                        Val::from(174),
                        Val::from("Golem"),
                        Val::from(1540),
                        Val::from(1),
                        Val::from("force_04mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(131),
                        Val::from(178),
                        Val::from("Marse"),
                        Val::from(1551),
                        Val::from(1),
                        Val::from("force_04mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_04mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#60::OnReset")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On04_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_04")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04mob_60(ctx: &Ctx) -> Script {
    force_04mob_60_run(ctx, Force04mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04mob_60_onreset(ctx: &Ctx) -> Script {
    force_04mob_60_run(ctx, Force04mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04mob_60_onenable(ctx: &Ctx) -> Script {
    force_04mob_60_run(ctx, Force04mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_04mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_04mob_60_run(ctx, Force04mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05start60Step {
    Start,
    OnEnable,
}

fn force_05start_60_run(ctx: &Ctx, mut step: Force05start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05start60Step::Start => {
                step = Force05start60Step::OnEnable;
                continue 'machine;
            }
            Force05start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_60(ctx: &Ctx) -> Script {
    force_05start_60_run(ctx, Force05start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_60_onenable(ctx: &Ctx) -> Script {
    force_05start_60_run(ctx, Force05start60Step::OnEnable, Vec::new()).map(|_| ())
}
