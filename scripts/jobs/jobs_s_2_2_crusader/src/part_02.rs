use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum SummonerCr3Step {
    Start,
    OnTimer345000,
    OnTimer345500,
    OnTimer346000,
    OnInit,
    OnStart,
    OnReset,
    OnEnd,
    OnDead,
}

fn summoner_cr3_run(ctx: &Ctx, mut step: SummonerCr3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SummonerCr3Step::Start => {
                step = SummonerCr3Step::OnTimer345000;
                continue 'machine;
            }
            SummonerCr3Step::OnTimer345000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr3::OnReset")])?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnTimer345500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr3::OnEnd")])?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnTimer346000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr3::OnStart")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr3")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(50),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(50),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(50),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(70),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(70),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(80),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(90),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(90),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr3")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(50),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(50),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(50),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(70),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(70),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(80),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(92),
                        Val::from(90),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(104),
                        Val::from(90),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_cru"), Val::from("Summoner#cr3::OnDead")],
                )?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Summoner#cr3")])?;
                return Err(Stop::End);
            }
            SummonerCr3Step::OnDead => {
                ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(353), Val::from(251)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn summoner_cr3(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::Start, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_ontimer345000(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnTimer345000, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_ontimer345500(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnTimer345500, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_ontimer346000(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnTimer346000, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_oninit(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_onstart(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnStart, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_onreset(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnReset, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_onend(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnEnd, Vec::new()).map(|_| ())
}

pub fn summoner_cr3_ondead(ctx: &Ctx) -> Script {
    summoner_cr3_run(ctx, SummonerCr3Step::OnDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SummonerCr4Step {
    Start,
    OnTimer345000,
    OnTimer345500,
    OnTimer346000,
    OnInit,
    OnStart,
    OnReset,
    OnEnd,
    OnDead,
}

fn summoner_cr4_run(ctx: &Ctx, mut step: SummonerCr4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SummonerCr4Step::Start => {
                step = SummonerCr4Step::OnTimer345000;
                continue 'machine;
            }
            SummonerCr4Step::OnTimer345000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr4::OnReset")])?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnTimer345500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr4::OnEnd")])?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnTimer346000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Summoner#cr4::OnStart")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr4")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(50),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(55),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(60),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(65),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(70),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(75),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(80),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(85),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(90),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(90),
                        Val::from(34),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(105),
                        Val::from(34),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Summoner#cr4")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(50),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(55),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(60),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(65),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(70),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(75),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(80),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(85),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(98),
                        Val::from(90),
                        Val::from("Familiar"),
                        Val::from(1005),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(90),
                        Val::from(34),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(105),
                        Val::from(34),
                        Val::from("Mushroom"),
                        Val::from(1182),
                        Val::from(1),
                        Val::from("Summoner#cr3::OnDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_cru"), Val::from("Summoner#cr4::OnDead")],
                )?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Summoner#cr4")])?;
                return Err(Stop::End);
            }
            SummonerCr4Step::OnDead => {
                ctx.call(Function::Warp, vec![Val::from("prt_fild05"), Val::from(353), Val::from(251)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn summoner_cr4(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::Start, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_ontimer345000(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnTimer345000, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_ontimer345500(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnTimer345500, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_ontimer346000(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnTimer346000, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_oninit(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnInit, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_onstart(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnStart, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_onreset(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnReset, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_onend(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnEnd, Vec::new()).map(|_| ())
}

pub fn summoner_cr4_ondead(ctx: &Ctx) -> Script {
    summoner_cr4_run(ctx, SummonerCr4Step::OnDead, Vec::new()).map(|_| ())
}

fn patron_knight_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Bliant Piyord",
        args![
            "Welcome.",
            "You will take",
            "the last test here.",
            "If you are ready for",
            "the test, enter the",
            "waiting room."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Bliant Piyord", args!["Only one person can take the test at a time. If someone else is already testing, please wait until the testing area is made available once again."])?;
    ctx.next()?;
    ctx.lines_as("Bliant Piyord", args!["Each person will get 4 minutes to complete the test. If you wish to leave in the middle of the test, please disconnect from the game."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn patron_knight_2(ctx: &Ctx) -> Script {
    patron_knight_2_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_cr1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn waiting_room_cr1(ctx: &Ctx) -> Script {
    waiting_room_cr1_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_cr1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Waiting Room#cr1")])?;
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Waiting Room"),
            Val::from(20),
            Val::from("Waiting Room#cr1::OnStartArena"),
            Val::from(1),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_cr1_oninit(ctx: &Ctx) -> Script {
    waiting_room_cr1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_cr1_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("job_cru"), Val::from(168), Val::from(21)],
    )?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr0::OnStart")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_cr1_onstartarena(ctx: &Ctx) -> Script {
    waiting_room_cr1_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn waiting_room_cr1_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn waiting_room_cr1_onstart(ctx: &Ctx) -> Script {
    waiting_room_cr1_onstart_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ZombieGuideStep {
    Start,
    OnTouch,
}

fn zombie_guide_run(ctx: &Ctx, mut step: ZombieGuideStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ZombieGuideStep::Start => {
                step = ZombieGuideStep::OnTouch;
                continue 'machine;
            }
            ZombieGuideStep::OnTouch => {
                ctx.lines_as(
                    "Bliant Piyord",
                    args![
                        "Go forth and defeat all",
                        "the monsters that appear.",
                        "You will not pass if any",
                        "are remaining."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Bliant Piyord",
                    args!["You will be given", "4 minutes. Go forth", "and do your best..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn zombie_guide(ctx: &Ctx) -> Script {
    zombie_guide_run(ctx, ZombieGuideStep::Start, Vec::new()).map(|_| ())
}

pub fn zombie_guide_ontouch(ctx: &Ctx) -> Script {
    zombie_guide_run(ctx, ZombieGuideStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr0Step {
    Start,
    OnInit,
    OnStart,
    OnMonster1,
    OnMonster2,
    OnMonster3,
    OnDead,
    OnEnd,
    OnReset,
}

fn monster_summon_cr0_run(ctx: &Ctx, mut step: MonsterSummonCr0Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr0Step::Start => {
                step = MonsterSummonCr0Step::OnInit;
                continue 'machine;
            }
            MonsterSummonCr0Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr0")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnStart => {
                ctx.var(".mymonstercount").set(Val::from(0))?;
                ctx.call(Function::EnableNpc, vec![Val::from("Monster Summon#cr0")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr1::OnStart")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr2::OnStart")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr3::OnStart")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr4::OnStart")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr6::OnStart")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnMonster1 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(45),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(45),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(45),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(45),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(45),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(45),
                        Val::from("Zombie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnMonster2 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(80),
                        Val::from("Soldier Skeleton"),
                        Val::from(1028),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(80),
                        Val::from("Soldier Skeleton"),
                        Val::from(1028),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(80),
                        Val::from("Soldier Skeleton"),
                        Val::from(1028),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnMonster3 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(110),
                        Val::from("Archer Skeleton"),
                        Val::from(1016),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(115),
                        Val::from("Mummy"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Monster Summon#cr0::OnDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnDead => {
                ctx.var(".mymonstercount").set((ctx.var(".mymonstercount").get()? + Val::from(1)))?;
                if ctx.var(".mymonstercount").get()?.number()? >= 10 {
                    ctx.var("crus_q").set(Val::from(10))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(3014), Val::from(3015)])?;
                }
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr0")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr0Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_cru"), Val::from("Monster Summon#cr0::OnDead")],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr0(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_oninit(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_onstart(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnStart, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_onmonster1(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnMonster1, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_onmonster2(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnMonster2, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_onmonster3(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnMonster3, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_ondead(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnDead, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_onend(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnEnd, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr0_onreset(ctx: &Ctx) -> Script {
    monster_summon_cr0_run(ctx, MonsterSummonCr0Step::OnReset, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr1Step {
    Start,
    OnInit,
    OnTouch,
    OnStart,
    OnEnd,
}

fn monster_summon_cr1_run(ctx: &Ctx, mut step: MonsterSummonCr1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr1Step::Start => {
                step = MonsterSummonCr1Step::OnInit;
                continue 'machine;
            }
            MonsterSummonCr1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr1")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr1Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr0::OnMonster1")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr1::OnEnd")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr1Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster Summon#cr1")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr1Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr1")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr1(ctx: &Ctx) -> Script {
    monster_summon_cr1_run(ctx, MonsterSummonCr1Step::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr1_oninit(ctx: &Ctx) -> Script {
    monster_summon_cr1_run(ctx, MonsterSummonCr1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr1_ontouch(ctx: &Ctx) -> Script {
    monster_summon_cr1_run(ctx, MonsterSummonCr1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr1_onstart(ctx: &Ctx) -> Script {
    monster_summon_cr1_run(ctx, MonsterSummonCr1Step::OnStart, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr1_onend(ctx: &Ctx) -> Script {
    monster_summon_cr1_run(ctx, MonsterSummonCr1Step::OnEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr2Step {
    Start,
    OnInit,
    OnTouch,
    OnStart,
    OnEnd,
}

fn monster_summon_cr2_run(ctx: &Ctx, mut step: MonsterSummonCr2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr2Step::Start => {
                step = MonsterSummonCr2Step::OnInit;
                continue 'machine;
            }
            MonsterSummonCr2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr2")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr2Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr0::OnMonster2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr2::OnEnd")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr2Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster Summon#cr2")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr2Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr2(ctx: &Ctx) -> Script {
    monster_summon_cr2_run(ctx, MonsterSummonCr2Step::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr2_oninit(ctx: &Ctx) -> Script {
    monster_summon_cr2_run(ctx, MonsterSummonCr2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr2_ontouch(ctx: &Ctx) -> Script {
    monster_summon_cr2_run(ctx, MonsterSummonCr2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr2_onstart(ctx: &Ctx) -> Script {
    monster_summon_cr2_run(ctx, MonsterSummonCr2Step::OnStart, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr2_onend(ctx: &Ctx) -> Script {
    monster_summon_cr2_run(ctx, MonsterSummonCr2Step::OnEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr3Step {
    Start,
    OnInit,
    OnTouch,
    OnStart,
    OnEnd,
}

fn monster_summon_cr3_run(ctx: &Ctx, mut step: MonsterSummonCr3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr3Step::Start => {
                step = MonsterSummonCr3Step::OnInit;
                continue 'machine;
            }
            MonsterSummonCr3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr3")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr3Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr0::OnMonster3")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr3::OnEnd")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr3Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster Summon#cr3")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr3Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr3(ctx: &Ctx) -> Script {
    monster_summon_cr3_run(ctx, MonsterSummonCr3Step::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr3_oninit(ctx: &Ctx) -> Script {
    monster_summon_cr3_run(ctx, MonsterSummonCr3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr3_ontouch(ctx: &Ctx) -> Script {
    monster_summon_cr3_run(ctx, MonsterSummonCr3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr3_onstart(ctx: &Ctx) -> Script {
    monster_summon_cr3_run(ctx, MonsterSummonCr3Step::OnStart, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr3_onend(ctx: &Ctx) -> Script {
    monster_summon_cr3_run(ctx, MonsterSummonCr3Step::OnEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr4Step {
    Start,
    OnInit,
    OnTouch,
    OnDead,
    OnStart,
    OnReset,
    OnEnd,
}

fn monster_summon_cr4_run(ctx: &Ctx, mut step: MonsterSummonCr4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr4Step::Start => {
                step = MonsterSummonCr4Step::OnInit;
                continue 'machine;
            }
            MonsterSummonCr4Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr4")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr4Step::OnTouch => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_cru"),
                        Val::from(168),
                        Val::from(150),
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        Val::from(1036),
                        Val::from(1),
                        Val::from("Monster Summon#cr4-a::OnDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr4::OnEnd")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr4Step::OnDead => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr5::OnStart")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr4Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster Summon#cr4")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr4Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("job_cru"), Val::from("Monster Summon#cr4-a::OnDead")],
                )?;
                return Err(Stop::End);
            }
            MonsterSummonCr4Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr4")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr4(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_oninit(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_ontouch(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_ondead(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::OnDead, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_onstart(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::OnStart, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_onreset(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::OnReset, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_onend(ctx: &Ctx) -> Script {
    monster_summon_cr4_run(ctx, MonsterSummonCr4Step::OnEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr4AStep {
    Start,
    OnDead,
}

fn monster_summon_cr4_a_run(ctx: &Ctx, mut step: MonsterSummonCr4AStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr4AStep::Start => {
                step = MonsterSummonCr4AStep::OnDead;
                continue 'machine;
            }
            MonsterSummonCr4AStep::OnDead => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr5::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr4_a(ctx: &Ctx) -> Script {
    monster_summon_cr4_a_run(ctx, MonsterSummonCr4AStep::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr4_a_ondead(ctx: &Ctx) -> Script {
    monster_summon_cr4_a_run(ctx, MonsterSummonCr4AStep::OnDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MonsterSummonCr6Step {
    Start,
    OnTimer241000,
    OnInit,
    OnStart,
    OnEnd,
    OnStop,
}

fn monster_summon_cr6_run(ctx: &Ctx, mut step: MonsterSummonCr6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MonsterSummonCr6Step::Start => {
                step = MonsterSummonCr6Step::OnTimer241000;
                continue 'machine;
            }
            MonsterSummonCr6Step::OnTimer241000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_cru"),
                        Val::from(160),
                        Val::from(14),
                        Val::from(175),
                        Val::from(178),
                        Val::from("job_cru"),
                        Val::from(24),
                        Val::from(169),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr0::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr4::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr0::OnEnd")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr4::OnEnd")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr5::OnEnd")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr6::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Monster Summon#cr6::OnEnd")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#cr1::OnStart")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr6Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr6")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr6Step::OnStart => {
                ctx.call(Function::EnableNpc, vec![Val::from("Monster Summon#cr6")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MonsterSummonCr6Step::OnEnd => {
                ctx.call(Function::DisableNpc, vec![Val::from("Monster Summon#cr6")])?;
                return Err(Stop::End);
            }
            MonsterSummonCr6Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_summon_cr6(ctx: &Ctx) -> Script {
    monster_summon_cr6_run(ctx, MonsterSummonCr6Step::Start, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr6_ontimer241000(ctx: &Ctx) -> Script {
    monster_summon_cr6_run(ctx, MonsterSummonCr6Step::OnTimer241000, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr6_oninit(ctx: &Ctx) -> Script {
    monster_summon_cr6_run(ctx, MonsterSummonCr6Step::OnInit, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr6_onstart(ctx: &Ctx) -> Script {
    monster_summon_cr6_run(ctx, MonsterSummonCr6Step::OnStart, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr6_onend(ctx: &Ctx) -> Script {
    monster_summon_cr6_run(ctx, MonsterSummonCr6Step::OnEnd, Vec::new()).map(|_| ())
}

pub fn monster_summon_cr6_onstop(ctx: &Ctx) -> Script {
    monster_summon_cr6_run(ctx, MonsterSummonCr6Step::OnStop, Vec::new()).map(|_| ())
}
