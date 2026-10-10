#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn bingo_waiting_room(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn bingo_waiting_room_oninit(ctx: &Ctx) -> Script {
    ctx.call(
        Function::WaitingRoom,
        args!["Bingo Waiting Room - 5 People", 50, "Bingo Waiting Room::OnWarp", 5, 1000],
    )?;
    ctx.end()
}

pub fn bingo_waiting_room_onwarp(ctx: &Ctx) -> Script {
    ctx.call(Function::WarpWaitingPc, args!["que_bingo", 46, 141])?;
    ctx.npc().do_event("start#bingo::OnStart")?;
    ctx.set_npc_visible("plate1#bingo", true)?;
    ctx.set_npc_visible("plate2#bingo", true)?;
    ctx.set_npc_visible("plate3#bingo", true)?;
    ctx.set_npc_visible("plate4#bingo", true)?;
    ctx.set_npc_visible("plate5#bingo", true)?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    ctx.end()
}

pub fn bingo_waiting_room_onstart(ctx: &Ctx) -> Script {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum Plate1BingoStep {
    Start,
    OnInit,
    OnTouch,
    LFill,
}

fn plate1_bingo_run(ctx: &Ctx, mut step: Plate1BingoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Plate1BingoStep::Start => {
                step = Plate1BingoStep::OnInit;
                continue 'machine;
            }
            Plate1BingoStep::OnInit => {
                ctx.set_npc_visible("plate1#bingo", false)?;
                ctx.set_npc_visible("plate2#bingo", false)?;
                ctx.set_npc_visible("plate3#bingo", false)?;
                ctx.set_npc_visible("plate4#bingo", false)?;
                ctx.set_npc_visible("plate5#bingo", false)?;
                return Err(Stop::End);
            }
            Plate1BingoStep::OnTouch => {
                ctx.var("@bingo_a1$").set(Val::from(0))?;
                ctx.var("@bingo_a2$").set(Val::from(0))?;
                ctx.var("@bingo_a3$").set(Val::from(0))?;
                ctx.var("@bingo_a4$").set(Val::from(0))?;
                ctx.var("@bingo_a5$").set(Val::from(0))?;
                ctx.var("@bingo_b1$").set(Val::from(0))?;
                ctx.var("@bingo_b2$").set(Val::from(0))?;
                ctx.var("@bingo_b3$").set(Val::from(0))?;
                ctx.var("@bingo_b4$").set(Val::from(0))?;
                ctx.var("@bingo_b5$").set(Val::from(0))?;
                ctx.var("@bingo_c1$").set(Val::from(0))?;
                ctx.var("@bingo_c2$").set(Val::from(0))?;
                ctx.var("@bingo_c3$").set(Val::from(0))?;
                ctx.var("@bingo_c4$").set(Val::from(0))?;
                ctx.var("@bingo_c5$").set(Val::from(0))?;
                ctx.var("@bingo_d1$").set(Val::from(0))?;
                ctx.var("@bingo_d2$").set(Val::from(0))?;
                ctx.var("@bingo_d3$").set(Val::from(0))?;
                ctx.var("@bingo_d4$").set(Val::from(0))?;
                ctx.var("@bingo_d5$").set(Val::from(0))?;
                ctx.var("@bingo_e1$").set(Val::from(0))?;
                ctx.var("@bingo_e2$").set(Val::from(0))?;
                ctx.var("@bingo_e3$").set(Val::from(0))?;
                ctx.var("@bingo_e4$").set(Val::from(0))?;
                ctx.var("@bingo_e5$").set(Val::from(0))?;
                ctx.var("@bingo_case").set(Val::from(1))?;
                step = Plate1BingoStep::LFill;
                continue 'machine;
            }
            Plate1BingoStep::LFill => {
                ctx.var("@bingo_fill").set(shared::other_hugel_bingo::func_bingo(
                    ctx,
                    args![ctx.var("@bingo_case").get()?],
                )?)?;
                if !ctx.var("@bingo_fill").get()?.is_true() {
                    ctx.lines(args![
                        "The numbers you have entered",
                        "exceed the limit, or you have",
                        "already entered these numbers.",
                        "Please enter your numbers again."
                    ])?;
                    ctx.next()?;
                } else if ctx.var("@bingo_fill").get()?.is_true() {
                    if ctx.var("@bingoplate").get_at(runtime::index(&Val::from(25))?)?.number()? < 10 {
                        ctx.var("@bingo_e5$")
                            .set(Val::from("0") + ctx.var("@bingoplate").get_at(runtime::index(&Val::from(25))?)? + Val::from(""))?;
                    } else {
                        ctx.var("@bingo_e5$")
                            .set(ctx.var("@bingoplate").get_at(runtime::index(&Val::from(25))?)?)?;
                    }
                    ctx.npc().do_event("start#bingo::OnEnter")?;
                    ctx.lines(args![
                        bingo_row(ctx, 'a')?,
                        bingo_row(ctx, 'b')?,
                        bingo_row(ctx, 'c')?,
                        bingo_row(ctx, 'd')?,
                        bingo_row(ctx, 'e')?,
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Plate1BingoStep::LFill;
                continue 'machine;
            }
        }
    }
}

pub fn plate1_bingo(ctx: &Ctx) -> Script {
    plate1_bingo_run(ctx, Plate1BingoStep::Start, Vec::new()).map(|_| ())
}

pub fn plate1_bingo_oninit(ctx: &Ctx) -> Script {
    plate1_bingo_run(ctx, Plate1BingoStep::OnInit, Vec::new()).map(|_| ())
}

pub fn plate1_bingo_ontouch(ctx: &Ctx) -> Script {
    plate1_bingo_run(ctx, Plate1BingoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StartBingoStep {
    Start,
    OnStart,
    OnEnter,
    OnTimer1000,
    OnTimer6000,
    OnTimer11000,
    OnTimer192000,
    OnTimer200000,
    OnTimer202000,
    OnTimer203000,
    OnTimer204000,
}

fn start_bingo_run(ctx: &Ctx, mut step: StartBingoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StartBingoStep::Start => {
                step = StartBingoStep::OnStart;
                continue 'machine;
            }
            StartBingoStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            StartBingoStep::OnEnter => {
                if ctx.var("$@hu_bingoa").get()? == 4 {
                    ctx.var("$@hu_bingoa").set(Val::from(5))?;
                    ctx.npc().do_event("start2#bingo::OnStart")?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                    return Err(Stop::End);
                }
                ctx.var("$@hu_bingoa").set(ctx.var("$@hu_bingoa").get()? + Val::from(1))?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["que_bingo", "Eukran: Hello, everyone! I'm Eukran, your Bingo Guide~", 1, 16755540],
                )?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer6000 => {
                ctx.call(Function::MapAnnounce, args!["que_bingo", "Eukran: Game participants, please enter the Warp Portal at the bottom of your screen and choose a Bingo Plate by entering a number.", 1, 16755540])?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "que_bingo",
                        "Eukran: All participants must choose their Bingo Plates within 3 minutes, or the game will be canceled.",
                        1,
                        16755540
                    ],
                )?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer192000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "que_bingo",
                        "Eukran: Please enter a number in 5 seconds, or the game will be canceled.",
                        1,
                        16755540
                    ],
                )?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer200000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "que_bingo",
                        "Eukran: I'm sorry, but the game has been canceled. Please come again and enjoy a game of Bingo with us!",
                        1,
                        16755540
                    ],
                )?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer202000 => {
                ctx.set_npc_visible("plate1#bingo", false)?;
                ctx.set_npc_visible("plate2#bingo", false)?;
                ctx.set_npc_visible("plate3#bingo", false)?;
                ctx.set_npc_visible("plate4#bingo", false)?;
                ctx.set_npc_visible("plate5#bingo", false)?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer203000 => {
                ctx.call(Function::AreaWarp, args!["que_bingo", 44, 115, 54, 126, "que_bingo", 40, 124])?;
                ctx.set_npc_visible("out3#bingo", true)?;
                return Err(Stop::End);
            }
            StartBingoStep::OnTimer204000 => {
                ctx.call(Function::AreaWarp, args!["que_bingo", 44, 115, 54, 126, "que_bingo", 40, 121])?;
                ctx.set_npc_visible("out3#bingo", false)?;
                ctx.var("$@hu_bingoa").set(Val::from(0))?;
                ctx.var("$@hu_bingob").set(Val::from(0))?;
                ctx.npc().do_event("Bingo Waiting Room::OnStart")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn start_bingo(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::Start, Vec::new()).map(|_| ())
}

pub fn start_bingo_onstart(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnStart, Vec::new()).map(|_| ())
}

pub fn start_bingo_onenter(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer1000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer6000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer11000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer192000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer192000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer200000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer200000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer202000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer202000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer203000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer203000, Vec::new()).map(|_| ())
}

pub fn start_bingo_ontimer204000(ctx: &Ctx) -> Script {
    start_bingo_run(ctx, StartBingoStep::OnTimer204000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Start2BingoStep {
    Start,
    OnStop,
    OnStart,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnTimer10000,
    OnTimer15000,
    OnTimer25000,
    OnTimer35000,
    OnTimer45000,
    OnTimer55000,
    OnTimer65000,
    OnTimer75000,
    OnTimer85000,
    OnTimer95000,
    OnTimer105000,
    OnTimer115000,
    OnTimer125000,
    OnTimer135000,
    OnTimer145000,
    OnTimer155000,
    OnTimer179000,
    OnTimer199000,
    OnTimer219000,
    OnTimer239000,
    OnTimer259000,
    OnTimer279000,
    OnTimer299000,
    OnTimer319000,
    OnTimer339000,
    OnTimer20000,
    OnTimer30000,
    OnTimer40000,
    OnTimer50000,
    OnTimer60000,
    OnTimer70000,
    OnTimer80000,
    OnTimer90000,
    OnTimer100000,
    OnTimer110000,
    OnTimer120000,
    OnTimer130000,
    OnTimer140000,
    OnTimer150000,
    OnTimer160000,
    OnTimer180000,
    OnTimer200000,
    OnTimer220000,
    OnTimer240000,
    OnTimer260000,
    OnTimer280000,
    OnTimer300000,
    OnTimer320000,
    OnTimer340000,
    OnTimer440000,
    OnTimer445000,
    OnTimer460000,
}

fn start2_bingo_run(ctx: &Ctx, mut step: Start2BingoStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_max_index = Val::from(0);
    'machine: loop {
        match step {
            Start2BingoStep::Start => {
                step = Start2BingoStep::OnStop;
                continue 'machine;
            }
            Start2BingoStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["que_bingo", "Eukran: Great, everyone seems to be ready~", 1, 16755540],
                )?;
                ctx.set_npc_visible("plate1#bingo", false)?;
                ctx.set_npc_visible("plate2#bingo", false)?;
                ctx.set_npc_visible("plate3#bingo", false)?;
                ctx.set_npc_visible("plate4#bingo", false)?;
                ctx.set_npc_visible("plate5#bingo", false)?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["que_bingo", "Eukran: Now, let the game begin!", 1, 16755540],
                )?;
                for i in 0..25_i32 {
                    ctx.var("$@bingoarray").set_at(runtime::index(&Val::from(i))?, Val::from(i + 1))?;
                }
                l_max_index = Val::from(24);
                for i in 0..25_i32 {
                    ctx.var("$@bingo0").set(ctx.call(Function::Rand, args![0, l_max_index.clone()])?)?;
                    ctx.var("$bingo").set_at(
                        runtime::index(&Val::from(i))?,
                        ctx.var("$@bingoarray").get_at(runtime::index(&ctx.var("$@bingo0").get()?)?)?,
                    )?;
                    ctx.var("$@bingoarray").set_at(
                        runtime::index(&ctx.var("$@bingo0").get()?)?,
                        ctx.var("$@bingoarray").get_at(runtime::index(&l_max_index)?)?,
                    )?;
                    l_max_index = l_max_index.clone().try_sub(Val::from(1))?;
                }
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer8000 => {
                ctx.call(Function::MapAnnounce, args!["que_bingo", "Eukran: I'll announce the Bingo Numbers. If you get 5 lines by matching 5 Bingo Numbers in a straight line on your Bingo Plate, yell out ''Bingo'' to win~", 1, 16755540])?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer10000 => {
                ctx.var("$@bingoresult").set(Val::from(1))?;
                shared::other_hugel_bingo::func_bingoresult(ctx, args![ctx.var("$@bingoresult").get()?])?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer15000 => {
                step = Start2BingoStep::OnTimer25000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer25000 => {
                step = Start2BingoStep::OnTimer35000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer35000 => {
                step = Start2BingoStep::OnTimer45000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer45000 => {
                step = Start2BingoStep::OnTimer55000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer55000 => {
                step = Start2BingoStep::OnTimer65000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer65000 => {
                step = Start2BingoStep::OnTimer75000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer75000 => {
                step = Start2BingoStep::OnTimer85000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer85000 => {
                step = Start2BingoStep::OnTimer95000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer95000 => {
                step = Start2BingoStep::OnTimer105000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer105000 => {
                step = Start2BingoStep::OnTimer115000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer115000 => {
                step = Start2BingoStep::OnTimer125000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer125000 => {
                step = Start2BingoStep::OnTimer135000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer135000 => {
                step = Start2BingoStep::OnTimer145000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer145000 => {
                step = Start2BingoStep::OnTimer155000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer155000 => {
                step = Start2BingoStep::OnTimer179000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer179000 => {
                step = Start2BingoStep::OnTimer199000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer199000 => {
                step = Start2BingoStep::OnTimer219000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer219000 => {
                step = Start2BingoStep::OnTimer239000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer239000 => {
                step = Start2BingoStep::OnTimer259000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer259000 => {
                step = Start2BingoStep::OnTimer279000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer279000 => {
                step = Start2BingoStep::OnTimer299000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer299000 => {
                step = Start2BingoStep::OnTimer319000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer319000 => {
                step = Start2BingoStep::OnTimer339000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer339000 => {
                ctx.npc().do_event("1a#bingo::OnInit")?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer20000 => {
                step = Start2BingoStep::OnTimer30000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer30000 => {
                step = Start2BingoStep::OnTimer40000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer40000 => {
                step = Start2BingoStep::OnTimer50000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer50000 => {
                step = Start2BingoStep::OnTimer60000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer60000 => {
                step = Start2BingoStep::OnTimer70000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer70000 => {
                step = Start2BingoStep::OnTimer80000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer80000 => {
                step = Start2BingoStep::OnTimer90000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer90000 => {
                step = Start2BingoStep::OnTimer100000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer100000 => {
                step = Start2BingoStep::OnTimer110000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer110000 => {
                step = Start2BingoStep::OnTimer120000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer120000 => {
                step = Start2BingoStep::OnTimer130000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer130000 => {
                step = Start2BingoStep::OnTimer140000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer140000 => {
                step = Start2BingoStep::OnTimer150000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer150000 => {
                step = Start2BingoStep::OnTimer160000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer160000 => {
                step = Start2BingoStep::OnTimer180000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer180000 => {
                step = Start2BingoStep::OnTimer200000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer200000 => {
                step = Start2BingoStep::OnTimer220000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer220000 => {
                step = Start2BingoStep::OnTimer240000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer240000 => {
                step = Start2BingoStep::OnTimer260000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer260000 => {
                step = Start2BingoStep::OnTimer280000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer280000 => {
                step = Start2BingoStep::OnTimer300000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer300000 => {
                step = Start2BingoStep::OnTimer320000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer320000 => {
                step = Start2BingoStep::OnTimer340000;
                continue 'machine;
            }
            Start2BingoStep::OnTimer340000 => {
                shared::other_hugel_bingo::func_bingoresult(ctx, args![ctx.var("$@bingoresult").get()?])?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer440000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "que_bingo",
                        "Eukran: I've announced all of the selected numbers, but I haven't heard anyone yell ''Bingo.''",
                        1,
                        16755540
                    ],
                )?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer445000 => {
                ctx.call(Function::MapAnnounce, args!["que_bingo", "Eukran: I'll give you all 10 seconds to check if any of you have won. If no one can yell ''Bingo'' in 10 seconds, this game will end without a winner.", 1, 16755540])?;
                return Err(Stop::End);
            }
            Start2BingoStep::OnTimer460000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "que_bingo",
                        "Eukran: I'm sorry, but this game has ended without a winner. Thanks for playing, everyone~",
                        1,
                        16755540
                    ],
                )?;
                ctx.npc().do_event("1a#bingo::OnInit")?;
                ctx.call(Function::AreaWarp, args!["que_bingo", 44, 115, 54, 126, "que_bingo", 40, 121])?;
                ctx.var("$@hu_bingoa").set(Val::from(0))?;
                ctx.npc().do_event("Bingo Waiting Room::OnStart")?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn start2_bingo(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::Start, Vec::new()).map(|_| ())
}

pub fn start2_bingo_onstop(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnStop, Vec::new()).map(|_| ())
}

pub fn start2_bingo_onstart(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnStart, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer2000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer5000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer8000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer10000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer15000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer25000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer35000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer35000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer45000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer45000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer55000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer55000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer65000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer65000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer75000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer75000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer85000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer85000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer95000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer95000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer105000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer105000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer115000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer115000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer125000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer125000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer135000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer135000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer145000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer145000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer155000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer155000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer179000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer179000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer199000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer199000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer219000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer219000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer239000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer239000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer259000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer259000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer279000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer279000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer299000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer299000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer319000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer319000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer339000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer339000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer20000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer30000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer40000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer40000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer50000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer60000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer70000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer70000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer80000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer80000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer90000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer90000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer100000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer100000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer110000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer110000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer120000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer130000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer130000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer140000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer140000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer150000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer150000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer160000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer160000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer180000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer200000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer200000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer220000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer220000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer240000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer260000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer260000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer280000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer280000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer300000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer320000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer320000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer340000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer340000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer440000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer440000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer445000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer445000, Vec::new()).map(|_| ())
}

pub fn start2_bingo_ontimer460000(ctx: &Ctx) -> Script {
    start2_bingo_run(ctx, Start2BingoStep::OnTimer460000, Vec::new()).map(|_| ())
}

pub fn s_1a_bingo(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn s_1a_bingo_oninit(ctx: &Ctx) -> Script {
    for letter in ['a', 'b', 'c', 'd', 'e'] {
        for n in 1..=25 {
            ctx.set_npc_visible(&format!("{n}{letter}#bingo"), false)?;
        }
    }
    ctx.end()
}

const BINGO_CELLS: [&str; 25] = [
    "@bingo_a1$",
    "@bingo_a2$",
    "@bingo_a3$",
    "@bingo_a4$",
    "@bingo_a5$",
    "@bingo_b1$",
    "@bingo_b2$",
    "@bingo_b3$",
    "@bingo_b4$",
    "@bingo_b5$",
    "@bingo_c1$",
    "@bingo_c2$",
    "@bingo_c3$",
    "@bingo_c4$",
    "@bingo_c5$",
    "@bingo_d1$",
    "@bingo_d2$",
    "@bingo_d3$",
    "@bingo_d4$",
    "@bingo_d5$",
    "@bingo_e1$",
    "@bingo_e2$",
    "@bingo_e3$",
    "@bingo_e4$",
    "@bingo_e5$",
];

fn bingo_row(ctx: &Ctx, row: char) -> Result<Val, Stop> {
    let mut text = Val::from("[");
    for col in 1..=5 {
        if col > 1 {
            text = text + Val::from("] [");
        }
        text = text + ctx.var(&format!("@bingo_{row}{col}$")).get()?;
    }
    Ok(text + Val::from("]"))
}

fn bingo_line_complete(ctx: &Ctx, cells: [i32; 5]) -> Result<bool, Stop> {
    for &cell in &cells[1..] {
        let first = ctx.var("@bingoplate").get_at(runtime::index(&Val::from(cells[0]))?)?;
        if !first.loosely_equals(&ctx.var("@bingoplate").get_at(runtime::index(&Val::from(cell))?)?) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn s_1a_bingo_ontouch(ctx: &Ctx) -> Script {
    for (i, &cell) in BINGO_CELLS.iter().enumerate() {
        let k = i as i32 + 1;
        if ctx.var("@bingoplate").get_at(runtime::index(&Val::from(k))?)?.loosely_equals(
            &ctx.var("$bingo")
                .get_at(runtime::index(&(ctx.var("$@bingoresult").get()?.try_sub(Val::from(1))?))?)?,
        ) {
            ctx.var(cell).set(Val::from("^ff0000-  -^000000"))?;
            ctx.var("@bingoplate").set_at(runtime::index(&Val::from(k))?, Val::from(99))?;
            break;
        }
    }
    ctx.var("@bingowin").set(Val::from(0))?;
    for line in [
        [1, 2, 3, 4, 5],
        [6, 7, 8, 9, 10],
        [11, 12, 13, 14, 15],
        [16, 17, 18, 19, 20],
        [21, 22, 23, 24, 25],
        [1, 6, 11, 16, 21],
        [2, 7, 12, 17, 22],
        [3, 8, 13, 18, 23],
        [4, 9, 14, 19, 24],
        [5, 10, 15, 20, 25],
        [1, 7, 13, 19, 25],
        [5, 9, 13, 17, 21],
    ] {
        if bingo_line_complete(ctx, line)? {
            ctx.var("@bingowin").set(ctx.var("@bingowin").get()? + Val::from(1))?;
        }
    }
    let ordinal = if ctx.var("$@bingoresult").get()? == 1 || ctx.var("$@bingoresult").get()? == 21 {
        "st Number - "
    } else if ctx.var("$@bingoresult").get()? == 2 || ctx.var("$@bingoresult").get()? == 22 {
        "nd Number - "
    } else if ctx.var("$@bingoresult").get()? == 3 || ctx.var("$@bingoresult").get()? == 23 {
        "rd Number - "
    } else {
        "th Number - "
    };
    ctx.lines(args![
        Val::from("[ ")
            + ctx.var("$@bingoresult").get()?
            + Val::from(ordinal)
            + ctx
                .var("$bingo")
                .get_at(runtime::index(&(ctx.var("$@bingoresult").get()?.try_sub(Val::from(1))?))?)?
            + Val::from(" ]"),
    ])?;
    ctx.lines(args![
        bingo_row(ctx, 'a')?,
        bingo_row(ctx, 'b')?,
        bingo_row(ctx, 'c')?,
        bingo_row(ctx, 'd')?,
        bingo_row(ctx, 'e')?,
        Val::from("[Currently Finished Lines - ") + ctx.var("@bingowin").get()? + Val::from(" ]"),
    ])?;
    if ctx.var("$@bingoresult").get()?.number()? > 15 && ctx.var("@bingowin").get()?.number()? > 4 {
        ctx.next()?;
        ctx.lines(args![
            "We just have made 5 lines!",
            "Say ^ff0000Bingo^000000!",
            "W-we just matched",
            "5 numbers in a row!",
            "Quickly, say ''^FF0000Bingo^000000!''",
            "Remember, you'll only",
            "have one chance to say it!"
        ])?;
        let (input, _) = runtime::input_text(ctx, None, None)?;
        ctx.var("@bingoyell$").set(input)?;
        if ctx.var("@bingoyell$").get()? == "Bingo" {
            if ctx.var("$@hu_bingoa").get()? == 5 {
                ctx.var("$@hu_bingoa").set(Val::from(6))?;
                ctx.npc().do_event("start2#bingo::OnStop")?;
                ctx.var("$@bingowinner$").set(ctx.player().name()?)?;
                ctx.npc().do_event("win1a#bingo::OnWin")?;
            } else if ctx.var("$@hu_bingoa").get()? == 6 {
                ctx.next()?;
                ctx.lines(args![
                    "Oh no! I'm sorry, but",
                    "someone already yelled",
                    "''bingo'' before you did.",
                    "I'm sorry, but you missed",
                    "your chance! Better luck,",
                    "next time, alright?"
                ])?;
            }
        } else {
            ctx.next()?;
            ctx.lines(args![
                "I'm sorry, but you",
                "said it wrong. Next time,",
                "make sure that you yell",
                "out the word, ''^FF0000Bingo^000000,'' okay?"
            ])?;
        }
    }
    ctx.close()
}

pub fn win1a_bingo(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn win1a_bingo_onwin(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.end()
}

pub fn win1a_bingo_ontimer1000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("1a#bingo::OnInit")?;
    ctx.set_npc_visible("win2a#bingo", true)?;
    ctx.set_npc_visible("win2b#bingo", true)?;
    ctx.set_npc_visible("win2c#bingo", true)?;
    ctx.set_npc_visible("win2d#bingo", true)?;
    ctx.set_npc_visible("win2e#bingo", true)?;
    ctx.end()
}

pub fn win2a_bingo(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn win2a_bingo_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("win2a#bingo", false)?;
    ctx.set_npc_visible("win2b#bingo", false)?;
    ctx.set_npc_visible("win2c#bingo", false)?;
    ctx.set_npc_visible("win2d#bingo", false)?;
    ctx.set_npc_visible("win2e#bingo", false)?;
    ctx.end()
}

pub fn win2a_bingo_ontouch(ctx: &Ctx) -> Script {
    if !ctx.var("$@bingowinner$").get()?.loosely_equals(&Val::from(ctx.player().name()?)) {
        return ctx.end();
    }
    ctx.call(Function::NpcSpecialEffect, args![constants::EF_SUI_EXPLOSION])?;
    ctx.call(Function::SoundEffect, args!["tming_success.wav", 1])?;
    if ctx.var("$@bingoresult").get()? == 16 {
        ctx.items().give(7515, 50)?;
    } else {
        ctx.items().give(7515, 1)?;
    }
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.end()
}

pub fn win2a_bingo_ontimer1000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["que_bingo", "Eukran: Wow, Bingo! It's Bingo!", 1, 16755540],
    )?;
    ctx.end()
}

pub fn win2a_bingo_ontimer5000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "que_bingo",
            Val::from("Eukran: ") + ctx.var("$@bingowinner$").get()? + Val::from(" has said Bingo!"),
            1,
            16755540
        ],
    )?;
    ctx.end()
}

pub fn win2a_bingo_ontimer10000(ctx: &Ctx) -> Script {
    let reward = if ctx.var("$@bingoresult").get()? == 16 {
        "! You will be rewarded with 50 Marvelous Medals."
    } else {
        "! You will be rewarded with 1 Marvelous Medal."
    };
    ctx.call(
        Function::MapAnnounce,
        args![
            "que_bingo",
            Val::from("Eukran: Congratulations, ") + ctx.var("$@bingowinner$").get()? + Val::from(reward),
            1,
            16755540
        ],
    )?;
    ctx.end()
}

pub fn win2a_bingo_ontimer15000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "que_bingo",
            "Eukran: Thank you all for participating in the game. See you next time!",
            1,
            16755540
        ],
    )?;
    ctx.end()
}

pub fn win2a_bingo_ontimer20000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("end#bingo::OnEnd")?;
    ctx.npc().do_event("win2a#bingo::OnInit")?;
    ctx.end()
}

pub fn end_bingo(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn end_bingo_onend(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.end()
}

pub fn end_bingo_ontimer1000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("end1a#bingo", true)?;
    ctx.set_npc_visible("end1b#bingo", true)?;
    ctx.set_npc_visible("end1c#bingo", true)?;
    ctx.set_npc_visible("end1d#bingo", true)?;
    ctx.set_npc_visible("end1e#bingo", true)?;
    ctx.end()
}

pub fn end_bingo_ontimer5000(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("end1a#bingo", false)?;
    ctx.set_npc_visible("end1b#bingo", false)?;
    ctx.set_npc_visible("end1c#bingo", false)?;
    ctx.set_npc_visible("end1d#bingo", false)?;
    ctx.set_npc_visible("end1e#bingo", false)?;
    ctx.call(Function::AreaWarp, args!["que_bingo", 44, 115, 54, 126, "que_bingo", 40, 121])?;
    ctx.var("$@hu_bingoa").set(Val::from(0))?;
    ctx.npc().do_event("Bingo Waiting Room::OnStart")?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum End1aBingoStep {
    Start,
    OnInit,
    OnTouch,
}

fn end1a_bingo_run(ctx: &Ctx, mut step: End1aBingoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            End1aBingoStep::Start => {
                step = End1aBingoStep::OnInit;
                continue 'machine;
            }
            End1aBingoStep::OnInit => {
                ctx.set_npc_visible("end1a#bingo", false)?;
                ctx.set_npc_visible("end1b#bingo", false)?;
                ctx.set_npc_visible("end1c#bingo", false)?;
                ctx.set_npc_visible("end1d#bingo", false)?;
                ctx.set_npc_visible("end1e#bingo", false)?;
                return Err(Stop::End);
            }
            End1aBingoStep::OnTouch => {
                if ctx.var("hg_ma1").get()? == 6 {
                    ctx.warp("que_bingo", 45, 186)?;
                } else {
                    ctx.warp("que_bingo", 40, 121)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn end1a_bingo(ctx: &Ctx) -> Script {
    end1a_bingo_run(ctx, End1aBingoStep::Start, Vec::new()).map(|_| ())
}

pub fn end1a_bingo_oninit(ctx: &Ctx) -> Script {
    end1a_bingo_run(ctx, End1aBingoStep::OnInit, Vec::new()).map(|_| ())
}

pub fn end1a_bingo_ontouch(ctx: &Ctx) -> Script {
    end1a_bingo_run(ctx, End1aBingoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Out3BingoStep {
    Start,
    OnInit,
    OnTouch,
}

fn out3_bingo_run(ctx: &Ctx, mut step: Out3BingoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Out3BingoStep::Start => {
                step = Out3BingoStep::OnInit;
                continue 'machine;
            }
            Out3BingoStep::OnInit => {
                ctx.set_npc_visible("out3#bingo", false)?;
                return Err(Stop::End);
            }
            Out3BingoStep::OnTouch => {
                ctx.warp("que_bingo", 40, 121)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn out3_bingo(ctx: &Ctx) -> Script {
    out3_bingo_run(ctx, Out3BingoStep::Start, Vec::new()).map(|_| ())
}

pub fn out3_bingo_oninit(ctx: &Ctx) -> Script {
    out3_bingo_run(ctx, Out3BingoStep::OnInit, Vec::new()).map(|_| ())
}

pub fn out3_bingo_ontouch(ctx: &Ctx) -> Script {
    out3_bingo_run(ctx, Out3BingoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Go3BingoStep {
    Start,
    OnTouch,
}

fn go3_bingo_run(ctx: &Ctx, mut step: Go3BingoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Go3BingoStep::Start => {
                step = Go3BingoStep::OnTouch;
                continue 'machine;
            }
            Go3BingoStep::OnTouch => {
                let subject = ctx.var("$@hu_bingob").get()?;
                if subject == 0 {
                    ctx.warp("que_bingo", 49, 125)?;
                } else if subject == 1 {
                    ctx.warp("que_bingo", 53, 121)?;
                } else if subject == 2 {
                    ctx.warp("que_bingo", 51, 116)?;
                } else if subject == 3 {
                    ctx.warp("que_bingo", 46, 116)?;
                } else if subject == 4 {
                    ctx.warp("que_bingo", 45, 121)?;
                }
                ctx.var("$@hu_bingob").set(ctx.var("$@hu_bingob").get()? + Val::from(1))?;
                if ctx.var("$@hu_bingob").get()? == 5 {
                    ctx.var("$@hu_bingob").set(Val::from(0))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn go3_bingo(ctx: &Ctx) -> Script {
    go3_bingo_run(ctx, Go3BingoStep::Start, Vec::new()).map(|_| ())
}

pub fn go3_bingo_ontouch(ctx: &Ctx) -> Script {
    go3_bingo_run(ctx, Go3BingoStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn arcade_helper_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Helper",
        args!["If you'd like to play", "a game of bingo, then", "please proceed this way."],
    )?;
    ctx.close()
}

pub fn arcade_owner(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Arcade Owner",
        args![
            "Welcome to the",
            "Bingo Game Arcade.",
            "Care to play a game of",
            "bingo? If you have any",
            "questions, feel free to ask."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Rules for Bingo", "Bingo Room", "Marvelous Medals"])? {
        0 => {
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "The rules for playing bingo",
                    "are simple. First, take a board",
                    "with 25 boxes organized so that",
                    "there are five rows and five",
                    "columns. Then, number the",
                    "boxes in any order you like."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "Of course, you must use",
                    "the numbers 1 through 25.",
                    "When everyone's bingo board",
                    "is ready, the game will begin.",
                    "Our game coordinator will call out a number from 1 to 25 at random."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "Each time the coordinator",
                    "calls out a number, make sure",
                    "that you mark the corresponding",
                    "numbered square on your bingo",
                    "board. Now, these are the",
                    "conditions for winning..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "If you can make a line of",
                    "5 squares in a row, horizontally, vertically, or diagonally, using",
                    "the numbers called out by the",
                    "coordinator, you quickly yell",
                    "the word, ''Bingo.''"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "If you are the first to yell",
                    "the word, ''Bingo,'' you'll",
                    "win! But if someone beats you",
                    "to it, then it can't be helped.",
                    "Anyway, it costs 1,000 zeny",
                    "to play each bingo game~"
                ],
            )?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "Ah, if you want to join a",
                    "bingo game, enter the right",
                    "door. There must be at least",
                    "5 people to play a game, so",
                    "you may need to wait until",
                    "that requirement is fulfilled."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "If you just want to",
                    "watch the bingo game,",
                    "then you may enter the",
                    "left door as a spectator",
                    "in the Bingo Room."
                ],
            )?;
            ctx.close()
        }
        2 => {
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "When you win a bingo",
                    "game, you will be rewarded",
                    "with ''Marvelous Medals,''",
                    "which can only be used within",
                    "this arcade. You also can't trade medals with other players."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "You usually get 1 Marvelous",
                    "Medal for winning a bingo game,",
                    "but you can win 50 at one time",
                    "under special conditions. You",
                    "can also play Monster Racing",
                    "games to win more medals."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Arcade Owner",
                args![
                    "Collect as many Marvelous",
                    "Medals as you can, and trade",
                    "them for products in the Monster Racing Arena. I hear there's also",
                    "a place in Einbroch where you can use them, but I wouldn't know."
                ],
            )?;
            ctx.close()
        }
        _ => Ok(()),
    }
}
