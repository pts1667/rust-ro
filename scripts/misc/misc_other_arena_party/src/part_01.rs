use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn ponox_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn ponox(ctx: &Ctx) -> Script {
    ponox_body(ctx, Vec::new()).map(|_| ())
}

fn ponox_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Party Mode - 5 Member Parties"),
            Val::from(11),
            Val::from("Ponox::OnStartArena"),
            Val::from(5),
            Val::from(0),
            Val::from(10),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn ponox_oninit(ctx: &Ctx) -> Script {
    ponox_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn ponox_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("toarena#party::OnInit")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("toout#party::OnInit")])?;
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("prt_are_in"), Val::from(73), Val::from(78)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("toout#party::OnTimer")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Helper#party::OnEnter")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn ponox_onstartarena(ctx: &Ctx) -> Script {
    ponox_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn ponox_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn ponox_onstart(ctx: &Ctx) -> Script {
    ponox_onstart_body(ctx, Vec::new()).map(|_| ())
}

fn helper_party_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$arn_partywait").get()? == 0 {
        ctx.var("$arn_partywait").set(Val::from(1))?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.lines_as(
            "Helper Iriff",
            args!["Good day, challengers!", "You are in the party arena waiting room."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Helper Iriff",
            args![
                "Only one person at a time is allowed to stay in this waiting room.",
                "Would you like to start a battle now?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("No"), Val::from("Yes")])? {
            1 => {
                ctx.var("$arn_partywait").set(Val::from(0))?;
                ctx.lines_as(
                    "Helper Iriff",
                    args!["I see.", "However, please remember you have only a limited amount of time."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Helper Iriff",
                    args![
                        "Thank you, let me start a battle.",
                        "A warp portal leading to the arena room will be open.",
                        "I hope you will survive until the end of the battle and engrave your name on the list of honor..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("toarena#party::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Helper#party::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::OnStart")])?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    return Err(Stop::End);
}

pub fn helper_party(ctx: &Ctx) -> Script {
    helper_party_body(ctx, Vec::new()).map(|_| ())
}

fn helper_party_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Helper#party")])?;
    return Err(Stop::End);
}

pub fn helper_party_onenter(ctx: &Ctx) -> Script {
    helper_party_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn helper_party_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Helper#party")])?;
    return Err(Stop::End);
}

pub fn helper_party_onstop(ctx: &Ctx) -> Script {
    helper_party_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn helper_party_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$arn_partywait").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn helper_party_oninit(ctx: &Ctx) -> Script {
    helper_party_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ToarenaPartyStep {
    Start,
    OnInit,
    OnTouch,
    OnEnter,
}

fn toarena_party_run(ctx: &Ctx, mut step: ToarenaPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ToarenaPartyStep::Start => {
                step = ToarenaPartyStep::OnInit;
                continue 'machine;
            }
            ToarenaPartyStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("toarena#party")])?;
                return Err(Stop::End);
            }
            ToarenaPartyStep::OnTouch => {
                if ctx.var("Zeny").get()?.number()? < 1000 {
                    ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
                } else {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                    ctx.var("$arn_partywait").set(Val::from(0))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("toout#party::OnStop")])?;
                    ctx.call(Function::Warp, vec![Val::from("force_1-2"), Val::from(99), Val::from(26)])?;
                }
                step = ToarenaPartyStep::OnEnter;
                continue 'machine;
            }
            ToarenaPartyStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("toarena#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn toarena_party(ctx: &Ctx) -> Script {
    toarena_party_run(ctx, ToarenaPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn toarena_party_oninit(ctx: &Ctx) -> Script {
    toarena_party_run(ctx, ToarenaPartyStep::OnInit, Vec::new()).map(|_| ())
}

pub fn toarena_party_ontouch(ctx: &Ctx) -> Script {
    toarena_party_run(ctx, ToarenaPartyStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn toarena_party_onenter(ctx: &Ctx) -> Script {
    toarena_party_run(ctx, ToarenaPartyStep::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TooutPartyStep {
    Start,
    OnInit,
    OnTimer,
    OnTimer60000,
    OnTimer70000,
    OnTouch,
    OnEnter,
    OnStop,
}

fn toout_party_run(ctx: &Ctx, mut step: TooutPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TooutPartyStep::Start => {
                step = TooutPartyStep::OnInit;
                continue 'machine;
            }
            TooutPartyStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("toout#party")])?;
                return Err(Stop::End);
            }
            TooutPartyStep::OnTimer => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TooutPartyStep::OnTimer60000 => {
                ctx.var("$arn_partywait").set(Val::from(0))?;
                ctx.call(Function::EnableNpc, vec![Val::from("toout#party")])?;
                return Err(Stop::End);
            }
            TooutPartyStep::OnTimer70000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("toout#party::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Ponox::OnStart")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("toout#party")])?;
                step = TooutPartyStep::OnTouch;
                continue 'machine;
            }
            TooutPartyStep::OnTouch => {
                ctx.var("$arn_partywait").set(Val::from(0))?;
                ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
                step = TooutPartyStep::OnEnter;
                continue 'machine;
            }
            TooutPartyStep::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("toout#party")])?;
                return Err(Stop::End);
            }
            TooutPartyStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn toout_party(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn toout_party_oninit(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnInit, Vec::new()).map(|_| ())
}

pub fn toout_party_ontimer(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnTimer, Vec::new()).map(|_| ())
}

pub fn toout_party_ontimer60000(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn toout_party_ontimer70000(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnTimer70000, Vec::new()).map(|_| ())
}

pub fn toout_party_ontouch(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn toout_party_onenter(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn toout_party_onstop(ctx: &Ctx) -> Script {
    toout_party_run(ctx, TooutPartyStep::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArenaOutStep {
    Start,
    OnTouch,
}

fn arena_out_run(ctx: &Ctx, mut step: ArenaOutStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaOutStep::Start => {
                step = ArenaOutStep::OnTouch;
                continue 'machine;
            }
            ArenaOutStep::OnTouch => {
                ctx.var("$arn_partywait").set(Val::from(0))?;
                ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_out(ctx: &Ctx) -> Script {
    arena_out_run(ctx, ArenaOutStep::Start, Vec::new()).map(|_| ())
}

pub fn arena_out_ontouch(ctx: &Ctx) -> Script {
    arena_out_run(ctx, ArenaOutStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_onstart(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnStart, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer2000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer3000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer4000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer5000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer60000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer120000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer180000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer240000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer300000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer360000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer420000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer420000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer480000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer480000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer540000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer540000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer600000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer605000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer605000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer606000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer606000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer607000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer607000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer608000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer608000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer609000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer609000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer610000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer610000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer611000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer611000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer612000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer612000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer613000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer613000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimer614000(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimer614000, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_onfail(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnFail, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_ontimeroff(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on01_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On01End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on02_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On02End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on03_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On03End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on04_start(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On04Start, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on04_end1(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On04End1, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on04_end2(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On04End2, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on05_end1(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On05End1, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on05_end2(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On05End2, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on06_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On06End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on07_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On07End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on08_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On08End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on09_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On09End, Vec::new()).map(|_| ())
}

pub fn slipslowrun_party_on10_end(ctx: &Ctx) -> Script {
    slipslowrun_party_run(ctx, SlipslowrunPartyStep::On10End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArenaPStep {
    Start,
    OnStart,
    On04Start,
    On06Start,
    On07Start,
    On08Start,
    On09Start,
    On10Start,
    OnExit,
    OnReset,
}

fn arena_p_run(ctx: &Ctx, mut step: ArenaPStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaPStep::Start => {
                step = ArenaPStep::OnStart;
                continue 'machine;
            }
            ArenaPStep::OnStart => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_1-2"), Val::from("prt_are_in"), Val::from(177), Val::from(138)],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_00")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_00")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_00")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_05")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_03")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_03")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09_10")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_10_09")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09_exit")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_01start#party")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02start#party")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_03start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_10start#party")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#party")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-1#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-2#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_exitmob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnStart")])?;
                ctx.var("$arn_partyc").set(Val::from(0))?;
                ctx.var("$arn_partywait").set(Val::from(0))?;
                return Err(Stop::End);
            }
            ArenaPStep::On04Start => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_04start#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::On06Start => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_06start#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::On07Start => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_07start#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::On08Start => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08start#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::On09Start => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_09start#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::On10Start => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_09_10")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_10start#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::OnExit => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_10_09")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_09_exit")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#party")])?;
                return Err(Stop::End);
            }
            ArenaPStep::OnReset => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-1#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-2#party::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_exitmob#party::OnReset")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_p(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::Start, Vec::new()).map(|_| ())
}

pub fn arena_p_onstart(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_p_on04_start(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::On04Start, Vec::new()).map(|_| ())
}

pub fn arena_p_on06_start(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::On06Start, Vec::new()).map(|_| ())
}

pub fn arena_p_on07_start(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::On07Start, Vec::new()).map(|_| ())
}

pub fn arena_p_on08_start(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::On08Start, Vec::new()).map(|_| ())
}

pub fn arena_p_on09_start(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::On09Start, Vec::new()).map(|_| ())
}

pub fn arena_p_on10_start(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::On10Start, Vec::new()).map(|_| ())
}

pub fn arena_p_onexit(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::OnExit, Vec::new()).map(|_| ())
}

pub fn arena_p_onreset(ctx: &Ctx) -> Script {
    arena_p_run(ctx, ArenaPStep::OnReset, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09ExitStep {
    Start,
    OnTouch,
}

fn force_09_exit_run(ctx: &Ctx, mut step: Force09ExitStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09ExitStep::Start => {
                step = Force09ExitStep::OnTouch;
                continue 'machine;
            }
            Force09ExitStep::OnTouch => {
                ctx.var("$arena_minptend")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                ctx.var("$arena_secptend")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                ctx.call(Function::Warp, vec![Val::from("prt_are_in"), Val::from(73), Val::from(139)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_pt::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnTimerOff")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09_exit(ctx: &Ctx) -> Script {
    force_09_exit_run(ctx, Force09ExitStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09_exit_ontouch(ctx: &Ctx) -> Script {
    force_09_exit_run(ctx, Force09ExitStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01startPartyStep {
    Start,
    OnTouch,
}

fn force_01start_party_run(ctx: &Ctx, mut step: Force01startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01startPartyStep::Start => {
                step = Force01startPartyStep::OnTouch;
                continue 'machine;
            }
            Force01startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_party(ctx: &Ctx) -> Script {
    force_01start_party_run(ctx, Force01startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_party_ontouch(ctx: &Ctx) -> Script {
    force_01start_party_run(ctx, Force01startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_01mob_party_run(ctx: &Ctx, mut step: Force01mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mobPartyStep::Start => {
                step = Force01mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force01mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(15),
                        Val::from(35),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(35),
                        Val::from(35),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(15),
                        Val::from(15),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(35),
                        Val::from(15),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(15),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(17),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(19),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(21),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(15),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(17),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(19),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(21),
                        Val::from(25),
                        Val::from("Clock"),
                        Val::from(1528),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(15),
                        Val::from(30),
                        Val::from("Alarm"),
                        Val::from(1476),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(17),
                        Val::from(22),
                        Val::from("Alarm"),
                        Val::from(1476),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(19),
                        Val::from(32),
                        Val::from("Alarm"),
                        Val::from(1476),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(21),
                        Val::from(22),
                        Val::from("Alarm"),
                        Val::from(1476),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(26),
                        Val::from(33),
                        Val::from("Clock Tower Keeper"),
                        Val::from(1527),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(26),
                        Val::from(33),
                        Val::from("Clock Tower Keeper"),
                        Val::from(1527),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(26),
                        Val::from(14),
                        Val::from("Ancient Worm"),
                        Val::from(1567),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(30),
                        Val::from(27),
                        Val::from("Ancient Worm"),
                        Val::from(1567),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(29),
                        Val::from(25),
                        Val::from("Ancient Worm"),
                        Val::from(1567),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(26),
                        Val::from(14),
                        Val::from("Ancient Worm"),
                        Val::from(1567),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(33),
                        Val::from(26),
                        Val::from("Incubus"),
                        Val::from(1580),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(29),
                        Val::from(28),
                        Val::from("Incubus"),
                        Val::from(1580),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(33),
                        Val::from(20),
                        Val::from("Incubus"),
                        Val::from(1580),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(33),
                        Val::from(30),
                        Val::from("Incubus"),
                        Val::from(1580),
                        Val::from(1),
                        Val::from("force_01mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_01mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_01mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::EnableNpc, vec![Val::from("force_01_00")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On01_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                    if ctx.var("$arn_partyc").get()? == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On04_Start")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("force_03_04")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("force_04start#party")])?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_party(ctx: &Ctx) -> Script {
    force_01mob_party_run(ctx, Force01mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_party_onenable(ctx: &Ctx) -> Script {
    force_01mob_party_run(ctx, Force01mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_party_onreset(ctx: &Ctx) -> Script {
    force_01mob_party_run(ctx, Force01mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_party_run(ctx, Force01mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02startPartyStep {
    Start,
    OnTouch,
}

fn force_02start_party_run(ctx: &Ctx, mut step: Force02startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02startPartyStep::Start => {
                step = Force02startPartyStep::OnTouch;
                continue 'machine;
            }
            Force02startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_party(ctx: &Ctx) -> Script {
    force_02start_party_run(ctx, Force02startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_party_ontouch(ctx: &Ctx) -> Script {
    force_02start_party_run(ctx, Force02startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_02mob_party_run(ctx: &Ctx, mut step: Force02mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mobPartyStep::Start => {
                step = Force02mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force02mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(163),
                        Val::from(36),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(184),
                        Val::from(36),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(184),
                        Val::from(16),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(163),
                        Val::from(16),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(171),
                        Val::from(37),
                        Val::from("Joker"),
                        Val::from(1437),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(177),
                        Val::from(37),
                        Val::from("Joker"),
                        Val::from(1437),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(184),
                        Val::from(29),
                        Val::from("Joker"),
                        Val::from(1437),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(184),
                        Val::from(22),
                        Val::from("Joker"),
                        Val::from(1437),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(177),
                        Val::from(16),
                        Val::from("Joker"),
                        Val::from(1437),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(170),
                        Val::from(16),
                        Val::from("Joker"),
                        Val::from(1437),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(169),
                        Val::from(28),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(171),
                        Val::from(28),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(173),
                        Val::from(28),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(175),
                        Val::from(28),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(177),
                        Val::from(28),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(177),
                        Val::from(23),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(175),
                        Val::from(23),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(173),
                        Val::from(23),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(171),
                        Val::from(23),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(169),
                        Val::from(23),
                        Val::from("Bathory"),
                        Val::from(1525),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(187),
                        Val::from(30),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(187),
                        Val::from(30),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(187),
                        Val::from(30),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(187),
                        Val::from(22),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(187),
                        Val::from(22),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(187),
                        Val::from(22),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(173),
                        Val::from(40),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(173),
                        Val::from(40),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(181),
                        Val::from(25),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(181),
                        Val::from(25),
                        Val::from("Arclouse"),
                        Val::from(1477),
                        Val::from(1),
                        Val::from("force_02mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_02mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_02mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::EnableNpc, vec![Val::from("force_02_00")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On02_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                    if ctx.var("$arn_partyc").get()? == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On04_Start")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("force_03_04")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("force_04start#party")])?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_party(ctx: &Ctx) -> Script {
    force_02mob_party_run(ctx, Force02mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_party_onenable(ctx: &Ctx) -> Script {
    force_02mob_party_run(ctx, Force02mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_02mob_party_onreset(ctx: &Ctx) -> Script {
    force_02mob_party_run(ctx, Force02mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_party_run(ctx, Force02mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03startPartyStep {
    Start,
    OnTouch,
}

fn force_03start_party_run(ctx: &Ctx, mut step: Force03startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03startPartyStep::Start => {
                step = Force03startPartyStep::OnTouch;
                continue 'machine;
            }
            Force03startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03start_party(ctx: &Ctx) -> Script {
    force_03start_party_run(ctx, Force03startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_party_ontouch(ctx: &Ctx) -> Script {
    force_03start_party_run(ctx, Force03startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_party_run(ctx: &Ctx, mut step: Force03mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mobPartyStep::Start => {
                step = Force03mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force03mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(89),
                        Val::from(81),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(90),
                        Val::from(81),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(91),
                        Val::from(81),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(92),
                        Val::from(81),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(93),
                        Val::from(81),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(96),
                        Val::from(85),
                        Val::from("Wind Ghost"),
                        Val::from(1450),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(98),
                        Val::from(85),
                        Val::from("Wind Ghost"),
                        Val::from(1450),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(100),
                        Val::from(85),
                        Val::from("Wind Ghost"),
                        Val::from(1450),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(102),
                        Val::from(85),
                        Val::from("Wind Ghost"),
                        Val::from(1450),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(104),
                        Val::from(85),
                        Val::from("Wind Ghost"),
                        Val::from(1450),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(88),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(90),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(92),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(94),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(96),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(98),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(100),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(102),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(104),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(106),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(108),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(110),
                        Val::from(79),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(91),
                        Val::from(86),
                        Val::from("Wanderer"),
                        Val::from(1490),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(108),
                        Val::from(86),
                        Val::from("Wanderer"),
                        Val::from(1490),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(91),
                        Val::from(69),
                        Val::from("Wanderer"),
                        Val::from(1490),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(108),
                        Val::from(69),
                        Val::from("Wanderer"),
                        Val::from(1490),
                        Val::from(1),
                        Val::from("force_03mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_03mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_03mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::EnableNpc, vec![Val::from("force_03_00")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On03_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                    if ctx.var("$arn_partyc").get()? == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On04_Start")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("force_03_04")])?;
                        ctx.call(Function::EnableNpc, vec![Val::from("force_04start#party")])?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_party(ctx: &Ctx) -> Script {
    force_03mob_party_run(ctx, Force03mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_party_onenable(ctx: &Ctx) -> Script {
    force_03mob_party_run(ctx, Force03mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_party_onreset(ctx: &Ctx) -> Script {
    force_03mob_party_run(ctx, Force03mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_party_run(ctx, Force03mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04startPartyStep {
    Start,
    OnTouch,
}

fn force_04start_party_run(ctx: &Ctx, mut step: Force04startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04startPartyStep::Start => {
                step = Force04startPartyStep::OnTouch;
                continue 'machine;
            }
            Force04startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_party(ctx: &Ctx) -> Script {
    force_04start_party_run(ctx, Force04startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_party_ontouch(ctx: &Ctx) -> Script {
    force_04start_party_run(ctx, Force04startPartyStep::OnTouch, Vec::new()).map(|_| ())
}
