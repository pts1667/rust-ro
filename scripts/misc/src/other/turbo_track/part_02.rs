use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum CosEnd2TtMainStep {
    Start,
    OnTouch,
    OnInit,
}

fn cos_end2_tt_main_run(ctx: &Ctx, mut step: CosEnd2TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_pts: Vec<Val> = Vec::new();
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            CosEnd2TtMainStep::Start => {
                step = CosEnd2TtMainStep::OnTouch;
                continue 'machine;
            }
            CosEnd2TtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                            + Val::from(" is second to reach the Finish Line! Congratulations!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                if (l_w_s.clone() == "e8" || l_w_s.clone() == "n8") {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_pts, &Val::from(base + 0), Val::from(28961), false);
                    runtime::local_set(&mut l_pts, &Val::from(base + 1), Val::from(40), false);
                }
                if (l_w_s.clone() == "e16" || l_w_s.clone() == "n16") {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_pts, &Val::from(base + 0), Val::from(28951), false);
                    runtime::local_set(&mut l_pts, &Val::from(base + 1), Val::from(50), false);
                }
                if runtime::op(
                    &ctx.var("tt_point").get()?,
                    "<",
                    &runtime::local_get(&l_pts, &Val::from(0), false),
                )?
                .is_true()
                {
                    ctx.var("tt_point")
                        .set((ctx.var("tt_point").get()? + runtime::local_get(&l_pts, &Val::from(1), false)))?;
                }
                ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end2"))],
                )?;
                ctx.call(
                    Function::EnableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end3"))],
                )?;
                return Err(Stop::End);
            }
            CosEnd2TtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end2"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cos_end2_tt_main(ctx: &Ctx) -> Script {
    cos_end2_tt_main_run(ctx, CosEnd2TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn cos_end2_tt_main_ontouch(ctx: &Ctx) -> Script {
    cos_end2_tt_main_run(ctx, CosEnd2TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn cos_end2_tt_main_oninit(ctx: &Ctx) -> Script {
    cos_end2_tt_main_run(ctx, CosEnd2TtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CosEnd3TtMainStep {
    Start,
    OnTouch,
    OnInit,
}

fn cos_end3_tt_main_run(ctx: &Ctx, mut step: CosEnd3TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_pts: Vec<Val> = Vec::new();
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            CosEnd3TtMainStep::Start => {
                step = CosEnd3TtMainStep::OnTouch;
                continue 'machine;
            }
            CosEnd3TtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(" is third to reach the Finish Line! Congratulations!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                if (l_w_s.clone() == "e8" || l_w_s.clone() == "n8") {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_pts, &Val::from(base + 0), Val::from(28971), false);
                    runtime::local_set(&mut l_pts, &Val::from(base + 1), Val::from(30), false);
                }
                if (l_w_s.clone() == "e16" || l_w_s.clone() == "n16") {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_pts, &Val::from(base + 0), Val::from(28951), false);
                    runtime::local_set(&mut l_pts, &Val::from(base + 1), Val::from(50), false);
                }
                if runtime::op(
                    &ctx.var("tt_point").get()?,
                    "<",
                    &runtime::local_get(&l_pts, &Val::from(0), false),
                )?
                .is_true()
                {
                    ctx.var("tt_point")
                        .set((ctx.var("tt_point").get()? + runtime::local_get(&l_pts, &Val::from(1), false)))?;
                }
                ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("Winner Helper#TBT_") + l_w_s.clone()) + Val::from("::OnEnable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("Master#") + l_w_s.clone()) + Val::from("::OnDisable"))],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Broadcast#") + l_w_s.clone())])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end3"))],
                )?;
                return Err(Stop::End);
            }
            CosEnd3TtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end3"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cos_end3_tt_main(ctx: &Ctx) -> Script {
    cos_end3_tt_main_run(ctx, CosEnd3TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn cos_end3_tt_main_ontouch(ctx: &Ctx) -> Script {
    cos_end3_tt_main_run(ctx, CosEnd3TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn cos_end3_tt_main_oninit(ctx: &Ctx) -> Script {
    cos_end3_tt_main_run(ctx, CosEnd3TtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DswitchTtMainStep {
    Start,
    OnTouch,
}

fn dswitch_tt_main_run(ctx: &Ctx, mut step: DswitchTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            DswitchTtMainStep::Start => {
                step = DswitchTtMainStep::OnTouch;
                continue 'machine;
            }
            DswitchTtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 1 {
                    ctx.call(Function::EnableNpc, vec![(Val::from("Flasher_Exit_1#") + l_w_s.clone())])?;
                } else {
                    ctx.call(Function::EnableNpc, vec![(Val::from("Flasher_Exit_2#") + l_w_s.clone())])?;
                }
                ctx.call(Function::DisableNpc, vec![(Val::from("Disposable_Switch#") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dswitch_tt_main(ctx: &Ctx) -> Script {
    dswitch_tt_main_run(ctx, DswitchTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn dswitch_tt_main_ontouch(ctx: &Ctx) -> Script {
    dswitch_tt_main_run(ctx, DswitchTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum FlasherTtMainStep {
    Start,
    OnTouch,
    OnInit,
}

fn flasher_tt_main_run(ctx: &Ctx, mut step: FlasherTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            FlasherTtMainStep::Start => {
                step = FlasherTtMainStep::OnTouch;
                continue 'machine;
            }
            FlasherTtMainStep::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has just passed the Flasher Maze!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                ctx.call(
                    Function::Warp,
                    vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(185), Val::from(227)],
                )?;
                return Err(Stop::End);
            }
            FlasherTtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn flasher_tt_main(ctx: &Ctx) -> Script {
    flasher_tt_main_run(ctx, FlasherTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn flasher_tt_main_ontouch(ctx: &Ctx) -> Script {
    flasher_tt_main_run(ctx, FlasherTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn flasher_tt_main_oninit(ctx: &Ctx) -> Script {
    flasher_tt_main_run(ctx, FlasherTtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WhelperTtMainStep {
    Start,
    OnEnable,
    OnTimer4000,
    OnTimer8000,
    OnTimer12000,
    OnTimer16000,
    OnTimer20000,
    OnTimer24000,
    OnTimer25000,
    OnTimer28000,
    OnTimer30000,
    OnTimer32000,
    OnTimer35000,
    OnTimer36000,
    OnTimer40000,
    OnTimer44000,
    OnTimer45000,
    OnTimer48000,
    OnTimer50000,
    OnTimer52000,
    OnTimer55000,
    OnTimer58000,
    OnTimer60000,
    OnTimer65000,
    OnTimer70000,
    OnTimer71000,
    RName,
    AfterRName,
    OnInit,
}

fn whelper_tt_main_run(ctx: &Ctx, mut step: WhelperTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rn_s = Val::from("");
    let mut l_s = Val::from(0);
    let mut l_string_s = Val::from("");
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            WhelperTtMainStep::Start => {
                if ctx.var("tt_rank").get()?.number()? < 29999 {
                    ctx.lines_as("Guide", args!["Congratulations!"])?;
                    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                    if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                        if ctx.var("tt_point").get()?.number()? < 28961 {
                            ctx.lines(args!["As the winner, your reward", "is 40 Turbo Track Points~"])?;
                            ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(40)))?;
                            ctx.var("my_point").set(ctx.var("tt_point").get()?)?;
                            ctx.lines(args![
                                "You now have a total of",
                                (ctx.var("tt_point").get()? + Val::from(" Turbo Track points,")),
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("."))
                            ])?;
                        }
                        ctx.lines(args![
                            "Unfortunately, I can't give",
                            "you any Turbo Track Points",
                            "since you would exceed the",
                            "maximum limit. Sorry,",
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("..."))
                        ])?;
                    } else {
                        ctx.lines(args![
                            "As the winner, your reward",
                            ((Val::from("is ")
                                + (if runtime::compare(&l_w_s.clone(), &Val::from("8")).is_true() {
                                    Val::from("50")
                                } else {
                                    Val::from("40")
                                }))
                                + Val::from(" Turbo Track Points,")),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("~"))
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Guide",
                            args![
                                "However, you cannot receive",
                                "any points if you exceed the",
                                ((Val::from("point limit. You now have a total of ") + ctx.var("my_point").get()?)
                                    + Val::from(" Turbo Track points."))
                            ],
                        )?;
                    }
                    ctx.var("tt_rank").set((ctx.var("tt_rank").get()? + Val::from(1)))?;
                    ctx.next()?;
                    if runtime::op(
                        &ctx.var("tt_rank").get()?,
                        ">",
                        &ctx.var("$ttranks").get_at(runtime::index(&Val::from(1))?)?,
                    )?
                    .is_true()
                    {
                        let base = Val::from(0).number()?;
                        ctx.var("$ttranks").set_at(
                            runtime::index(&Val::from(base + 0))?,
                            ctx.var("$ttranks").get_at(runtime::index(&Val::from(0))?)?,
                        )?;
                        ctx.var("$ttranks")
                            .set_at(runtime::index(&Val::from(base + 1))?, ctx.var("tt_rank").get()?)?;
                        ctx.var("$ttranks").set_at(
                            runtime::index(&Val::from(base + 2))?,
                            ctx.var("$ttranks").get_at(runtime::index(&Val::from(1))?)?,
                        )?;
                        ctx.var("$ttranks").set_at(
                            runtime::index(&Val::from(base + 3))?,
                            ctx.var("$ttranks").get_at(runtime::index(&Val::from(2))?)?,
                        )?;
                        ctx.var("$ttranks").set_at(
                            runtime::index(&Val::from(base + 4))?,
                            ctx.var("$ttranks").get_at(runtime::index(&Val::from(3))?)?,
                        )?;
                        ctx.var("$ttranks").set_at(
                            runtime::index(&Val::from(base + 5))?,
                            ctx.var("$ttranks").get_at(runtime::index(&Val::from(4))?)?,
                        )?;
                        let base = Val::from(9).number()?;
                        ctx.var("$ttnames$").set_at(
                            runtime::index(&Val::from(base + 0))?,
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        )?;
                        ctx.lines_as(
                            "Guide",
                            args![
                                "Congratulations!",
                                "You are ranked as",
                                "the top player for",
                                "winning the most games!"
                            ],
                        )?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Turbo Track Guide::OnNew_Top1")])?;
                        if ctx.var("tt_rank").get()? == 29999 {
                            if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(7))?)? == "Breezy Havana" {
                                ctx.var("$ttnames$").set_at(
                                    runtime::index(&Val::from(7))?,
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                )?;
                                ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                ctx.call(
                                    Function::Announce,
                                    vec![
                                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                            + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                        ctx.constant("BC_ALL")?,
                                        Val::from("FF0000"),
                                    ],
                                )?;
                            } else if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(8))?)? == "RS125" {
                                ctx.var("$ttnames$").set_at(
                                    runtime::index(&Val::from(8))?,
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                )?;
                                ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                }
                            }
                        }
                    } else {
                        if runtime::op(
                            &ctx.var("tt_rank").get()?,
                            ">",
                            &ctx.var("$ttranks").get_at(runtime::index(&Val::from(2))?)?,
                        )?
                        .is_true()
                        {
                            let base = Val::from(0).number()?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(0))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 1))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(1))?)?,
                            )?;
                            ctx.var("$ttranks")
                                .set_at(runtime::index(&Val::from(base + 2))?, ctx.var("tt_rank").get()?)?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 3))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(3))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 4))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(4))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 5))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(5))?)?,
                            )?;
                            let base = Val::from(10).number()?;
                            ctx.var("$ttnames$").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            )?;
                            ctx.lines_as(
                                "Guide",
                                args![
                                    "Congratulations!",
                                    "You've ranked Second",
                                    "among the Top Five Players",
                                    "who've won the most games!"
                                ],
                            )?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Turbo Track Guide::OnNew_Top2")])?;
                            if ctx.var("tt_rank").get()? == 29999 {
                                if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(7))?)? == "Breezy Havana" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(7))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                } else if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(8))?)? == "RS125" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(8))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                }
                            }
                        } else if runtime::op(
                            &ctx.var("tt_rank").get()?,
                            ">",
                            &ctx.var("$ttranks").get_at(runtime::index(&Val::from(3))?)?,
                        )?
                        .is_true()
                        {
                            let base = Val::from(0).number()?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(0))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 1))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(1))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 2))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(2))?)?,
                            )?;
                            ctx.var("$ttranks")
                                .set_at(runtime::index(&Val::from(base + 3))?, ctx.var("tt_rank").get()?)?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 4))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(4))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 5))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(5))?)?,
                            )?;
                            let base = Val::from(11).number()?;
                            ctx.var("$ttnames$").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            )?;
                            ctx.lines_as(
                                "Guide",
                                args![
                                    "Congratulations!",
                                    "You've ranked Third",
                                    "among the Top Five Players",
                                    "who've won the most games!"
                                ],
                            )?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Turbo Track Guide::OnNew_Top3")])?;
                            if ctx.var("tt_rank").get()? == 29999 {
                                if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(7))?)? == "Breezy Havana" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(7))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                } else if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(8))?)? == "RS125" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(8))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                }
                            }
                        } else if runtime::op(
                            &ctx.var("tt_rank").get()?,
                            ">",
                            &ctx.var("$ttranks").get_at(runtime::index(&Val::from(4))?)?,
                        )?
                        .is_true()
                        {
                            let base = Val::from(0).number()?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(0))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 1))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(1))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 2))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(2))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 3))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(3))?)?,
                            )?;
                            ctx.var("$ttranks")
                                .set_at(runtime::index(&Val::from(base + 4))?, ctx.var("tt_rank").get()?)?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 5))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(5))?)?,
                            )?;
                            let base = Val::from(12).number()?;
                            ctx.var("$ttnames$").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            )?;
                            ctx.lines_as(
                                "Guide",
                                args![
                                    "Congratulations!",
                                    "You've ranked Fourth",
                                    "among the Top Five Players",
                                    "who've won the most games!"
                                ],
                            )?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Turbo Track Guide::OnNew_Top4")])?;
                            if ctx.var("tt_rank").get()? == 29999 {
                                if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(7))?)? == "Breezy Havana" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(7))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                } else if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(8))?)? == "RS125" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(8))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                }
                            }
                        } else if runtime::op(
                            &ctx.var("tt_rank").get()?,
                            ">",
                            &ctx.var("$ttranks").get_at(runtime::index(&Val::from(5))?)?,
                        )?
                        .is_true()
                        {
                            let base = Val::from(0).number()?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(0))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 1))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(1))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 2))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(2))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 3))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(3))?)?,
                            )?;
                            ctx.var("$ttranks").set_at(
                                runtime::index(&Val::from(base + 4))?,
                                ctx.var("$ttranks").get_at(runtime::index(&Val::from(4))?)?,
                            )?;
                            ctx.var("$ttranks")
                                .set_at(runtime::index(&Val::from(base + 5))?, ctx.var("tt_rank").get()?)?;
                            let base = Val::from(13).number()?;
                            ctx.var("$ttnames$").set_at(
                                runtime::index(&Val::from(base + 0))?,
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            )?;
                            ctx.lines_as(
                                "Guide",
                                args![
                                    "Congratulations!",
                                    "You've ranked Fifth",
                                    "among the Top Five Players",
                                    "who've won the most games!"
                                ],
                            )?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Turbo Track Guide::OnNew_Top5")])?;
                            if ctx.var("tt_rank").get()? == 29999 {
                                if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(7))?)? == "Breezy Havana" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(7))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                } else if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(8))?)? == "RS125" {
                                    ctx.var("$ttnames$").set_at(
                                        runtime::index(&Val::from(8))?,
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    )?;
                                    ctx.lines(args!["You've secured your place in", "the Turbo Track Hall of Honor!"])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                                + Val::from(" has joined the Turbo Track Hall of Honor!")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("FF0000"),
                                        ],
                                    )?;
                                }
                            }
                        } else {
                            ctx.lines_as(
                                "Guide",
                                args![
                                    "If you can win more games",
                                    "than everybody else, your",
                                    "name will be registered in",
                                    "our Top Five Player Ranking."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Guide",
                                args![
                                    "What do you",
                                    ((Val::from("think, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                    "Glory can be yours if",
                                    "you can achieve victory!"
                                ],
                            )?;
                        }
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Thank you for",
                            "participating in",
                            "the Turbo Track.",
                            "You will be transported",
                            "to a Waiting Room shortly."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
                    return Err(Stop::End);
                } else if ctx
                    .var("$ttnames$")
                    .get_at(runtime::index(&Val::from(7))?)?
                    .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                {
                    if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                        ctx.lines_as(
                            "Guide",
                            args![
                                "Oh wow!",
                                "You're a member",
                                "in our Hall of Honor,",
                                "aren't you? This is great!",
                                "I'm talking to a living legend!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Guide", args!["Right, your reward..."])?;
                        if ctx.var("tt_point").get()?.number()? < 28961 {
                            ctx.mes("40 Turbo Track Points!")?;
                            ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(40)))?;
                            ctx.lines(args![
                                "You now have a total of",
                                (ctx.var("tt_point").get()? + Val::from(" Turbo Track Points."))
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Unfortunately, I can't give",
                                "you any Turbo Track Points",
                                "since you would exceed the",
                                "maximum limit. Sorry,",
                                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("..."))
                            ])?;
                        }
                    } else {
                        ctx.lines(args![
                            "Oh, I'm sorry! You're",
                            "here so that I can tell you",
                            "how many Turbo Track Points"
                        ])?;
                        ctx.var("my_point").set(ctx.var("tt_point").get()?)?;
                        ctx.lines(args![
                            ((((Val::from("you have, right? You've got a total of ") + ctx.var("my_point").get()?)
                                + Val::from(" Turbo Track Points, "))
                                + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from("."))
                        ])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Oh, and thanks for",
                            "participating in the",
                            "Turbo Track! You'll be sent",
                            "to the Waiting Room soon~"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
                    return Err(Stop::End);
                } else if ctx
                    .var("$ttnames$")
                    .get_at(runtime::index(&Val::from(8))?)?
                    .loosely_equals(&ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                {
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Hey, aren't you",
                            "in our Hall of Honor?",
                            "I've been watching your",
                            "races... You're pretty quick",
                            "on your feet, hotshot~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Guide]")?;
                    if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                        ctx.mes("Right, your reward...")?;
                        if ctx.var("tt_point").get()?.number()? < 28961 {
                            ctx.mes("40 Turbo Track Points!")?;
                            ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(40)))?;
                            ctx.lines(args![
                                "You now have a total of",
                                (ctx.var("tt_point").get()? + Val::from(" Turbo Track Points."))
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Unfortunately, I can't give",
                                "you any Turbo Track Points",
                                "since you would exceed the",
                                "maximum limit. Sorry..."
                            ])?;
                        }
                    } else {
                        ctx.mes("Oh right, your current")?;
                        ctx.var("my_point").set(ctx.var("tt_point").get()?)?;
                        ctx.lines(args![
                            "Turbo Track Point total!",
                            "You've got a total of",
                            (ctx.var("my_point").get()? + Val::from(" Turbo Track points,")),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("."))
                        ])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Oh, and thanks for",
                            "participating in the",
                            "Turbo Track! You'll be sent",
                            "to the Waiting Room soon~"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Awwww~",
                            "You were almost able",
                            "to join our Hall of Honor...!",
                            "But don't let that get you",
                            "down. Maybe next time!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Guide]")?;
                    if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                        ctx.lines(args!["Right, your reward...", "40 Turbo Track Points!"])?;
                        if ctx.var("tt_point").get()?.number()? < 28961 {
                            ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(40)))?;
                            ctx.var("my_point").set(ctx.var("tt_point").get()?)?;
                            ctx.lines(args![
                                "You now have a total of",
                                (ctx.var("tt_point").get()? + Val::from(" Turbo Track Points."))
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Unfortunately, I can't give",
                                "you any Turbo Track Points",
                                "since you would exceed the",
                                "maximum limit. Sorry..."
                            ])?;
                        }
                    } else {
                        ctx.lines(args![
                            "Oh, right.",
                            "Currently, you",
                            "have a total of",
                            (ctx.var("tt_point").get()? + Val::from(" Turbo Track points."))
                        ])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Thank you for",
                            "participating in",
                            "the Turbo Track.",
                            "You will be transported",
                            "to a Waiting Room shortly."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
                    return Err(Stop::End);
                }
            }
            WhelperTtMainStep::OnEnable => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Winner Helper#TBT_") + l_w_s.clone())])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer4000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        ((Val::from("This is the ending broadcast for Turbo Track ")
                            + whelper_tt_main_run(ctx, WhelperTtMainStep::RName, vec![l_w_s.clone()])?)
                            + Val::from(".")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("For smooth game play, the game will end in approximately 1 minute."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        Val::from("At that time, a Warp portal will open."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x33FF66"),
                    ],
                )?;
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer16000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Players in the arena must be ready to leave through the Warp Portal."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer20000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("In the meantime, winners must proceed to receive their rewards as soon as possible."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Players within the arena must be in ready to enter the warp."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer24000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Thank you for visiting Al De Baran Turbo Track."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer25000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("In the meantime, winners must procceed to receive their rewards as soon as possible."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer28000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Al De Baran Turbo Track is brought to you by..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer30000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Thank you for visiting Al De Baran Turbo Track."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                step = WhelperTtMainStep::OnTimer32000;
                continue 'machine;
            }
            WhelperTtMainStep::OnTimer32000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("[Al De Baran Kafra Corporation Headquarters]"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer35000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Al De Baran Turbo Track is brought to you by..."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer36000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("[Al De Baran Guild Castle Management Luina]"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer40000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("[Blacksmith Union]"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("[Al De Baran Kafra Corporation Headquarters]"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer44000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("And [Comodo Casino]."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer45000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("[Al De Baran Guild Castle Management Luina]"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer48000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Once again, we'd like to thank our sponsors."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer50000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("[Blacksmith Union]"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer52000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Have a good day."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer55000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("And [Comodo Casino]."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer58000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(
                        Function::MapWarp,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("turbo_room"),
                            Val::from(72),
                            Val::from(89),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer60000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true() {
                    ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker1#TBT_") + l_w_s.clone())])?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker3#TBT_") + l_w_s.clone())])?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker4#TBT_") + l_w_s.clone())])?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Disposable_Switch#") + l_w_s.clone())])?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("Flasher_Exit_1#") + l_w_s.clone())])?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("Flasher_Exit_2#") + l_w_s.clone())])?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-1"))],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-2"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![((Val::from("snake#") + l_w_s.clone()) + Val::from("::OnReset"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![((Val::from("hunting#") + l_w_s.clone()) + Val::from("::OnReset"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(whelper_tt_main_run(ctx, WhelperTtMainStep::RName, vec![l_w_s.clone()])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("bing2#") + l_w_s.clone())])?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end"))],
                    )?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Once again, we'd like to thank our sponsors."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer65000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("Have a good day."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x33FF66"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer70000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(
                        Function::MapWarp,
                        vec![
                            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                            Val::from("turbo_room"),
                            Val::from(72),
                            Val::from(89),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            WhelperTtMainStep::OnTimer71000 => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                if !(runtime::compare(&l_w_s.clone(), &Val::from("4")).is_true()) {
                    ctx.call(Function::DisableNpc, vec![(Val::from("Winner Helper#TBT_") + l_w_s.clone())])?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end"))],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end2"))],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end3"))],
                    )?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("Notice_Maker1#TBT_") + l_w_s.clone())])?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("Notice_Maker3#TBT_") + l_w_s.clone())])?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("Notice_Maker4#TBT_") + l_w_s.clone())])?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Disposable_Switch#") + l_w_s.clone())])?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Flasher_Exit_1#") + l_w_s.clone())])?;
                    ctx.call(Function::EnableNpc, vec![(Val::from("Flasher_Exit_2#") + l_w_s.clone())])?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-1"))],
                    )?;
                    ctx.call(
                        Function::EnableNpc,
                        vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-2"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![((Val::from("snake#") + l_w_s.clone()) + Val::from("::OnReset"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![((Val::from("hunting#") + l_w_s.clone()) + Val::from("::OnReset"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(whelper_tt_main_run(ctx, WhelperTtMainStep::RName, vec![l_w_s.clone()])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(Function::DisableNpc, vec![(Val::from("bing2#") + l_w_s.clone())])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
                step = WhelperTtMainStep::AfterRName;
                continue 'machine;
            }
            WhelperTtMainStep::RName => {
                l_string_s = runtime::arg(&args, 0, Val::from(0));
                l_s = (if runtime::strlen(&l_string_s.clone()).number()? > 2 {
                    runtime::substr(&l_string_s.clone(), &Val::from(1), &Val::from(2))?
                } else {
                    runtime::charat(
                        &l_string_s.clone(),
                        &(runtime::strlen(&l_string_s.clone()).try_sub(Val::from(1))?),
                    )?
                });
                l_rn_s = ((((if runtime::compare(
                    &ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                    &(Val::from("_e_") + l_s.clone()),
                )
                .is_true()
                {
                    Val::from("Expert mode")
                } else {
                    Val::from("Normal mode")
                }) + Val::from(" - "))
                    + l_s.clone())
                    + Val::from(" person"));
                return Ok(l_rn_s.clone());
                return Ok(Val::from(0));
            }
            WhelperTtMainStep::AfterRName => {
                step = WhelperTtMainStep::OnInit;
                continue 'machine;
            }
            WhelperTtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Winner Helper#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn whelper_tt_main(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_onenable(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer4000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer8000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer12000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer16000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer20000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer24000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer24000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer25000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer28000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer28000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer30000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer32000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer32000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer35000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer35000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer36000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer36000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer40000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer40000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer44000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer44000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer45000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer45000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer48000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer48000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer50000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer52000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer52000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer55000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer55000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer58000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer58000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer60000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer65000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer65000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer70000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer70000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_ontimer71000(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnTimer71000, Vec::new()).map(|_| ())
}

pub fn whelper_tt_main_oninit(ctx: &Ctx) -> Script {
    whelper_tt_main_run(ctx, WhelperTtMainStep::OnInit, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_et_to_sec = Val::from(0);
    let mut l_gap = Val::from(0);
    let mut l_hour_end = Val::from(0);
    let mut l_hour_start = Val::from(0);
    let mut l_min_end = Val::from(0);
    let mut l_min_start = Val::from(0);
    let mut l_record_hour = Val::from(0);
    let mut l_record_min = Val::from(0);
    let mut l_record_sec = Val::from(0);
    let mut l_record_time = Val::from(0);
    let mut l_sec_end = Val::from(0);
    let mut l_sec_start = Val::from(0);
    let mut l_st_to_sec = Val::from(0);
    let mut l_topbun = Val::from(0);
    let mut l_topcho = Val::from(0);
    l_hour_start = (ctx.var("$@start_time").get()?.try_div(Val::from(10000))?);
    l_min_start = ((ctx.var("$@start_time").get()?.try_rem(Val::from(10000))?).try_div(Val::from(100))?);
    l_sec_start = (ctx.var("$@start_time").get()?.try_rem(Val::from(100))?);
    l_hour_end = (ctx.var("$@end_time").get()?.try_div(Val::from(10000))?);
    l_min_end = ((ctx.var("$@end_time").get()?.try_rem(Val::from(10000))?).try_div(Val::from(100))?);
    l_sec_end = (ctx.var("$@end_time").get()?.try_rem(Val::from(100))?);
    if (ctx.var("hour_start").get()? == 23 && ctx.var("hour_end").get()? == 0) {
        l_hour_end = Val::from(24);
    }
    l_st_to_sec =
        (((l_hour_start.clone().try_mul(Val::from(3600))?) + (l_min_start.clone().try_mul(Val::from(60))?)) + l_sec_start.clone());
    l_et_to_sec = (((l_hour_end.clone().try_mul(Val::from(3600))?) + (l_min_end.clone().try_mul(Val::from(60))?)) + l_sec_end.clone());
    l_record_time = (l_et_to_sec.clone().try_sub(l_st_to_sec.clone())?);
    l_record_hour = (l_record_time.clone().try_div(Val::from(3600))?);
    l_record_min = ((l_record_time.clone().try_rem(Val::from(3600))?).try_div(Val::from(60))?);
    l_record_sec = (l_record_time.clone().try_rem(Val::from(60))?);
    if l_record_min.clone().number()? < 0 {
        ctx.lines_as(
            "Guide",
            args![
                "Good work! Maybe you didn't",
                "set any new records, but you",
                "went the distance. Now, let",
                "me relieve you of your fatigue~"
            ],
        )?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
        ctx.next()?;
        ctx.mes("[Guide]")?;
        if ctx.var("tt_point").get()?.number()? < 28991 {
            ctx.lines(args!["You will be rewarded", "with 10 Turbo Track points~!"])?;
            ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(10)))?;
            ctx.var("my_point").set(ctx.var("tt_point").get()?)?;
            ctx.lines(args![
                "You now have a total of",
                (ctx.var("tt_point").get()? + Val::from(" Turbo Track points."))
            ])?;
        } else {
            ctx.lines(args![
                "Unfortunately, I can't give",
                "you any Turbo Track Points",
                "since you would exceed the",
                "maximum limit. Sorry,",
                (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("..."))
            ])?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Guide",
            args![
                "Thank you for",
                "participating in",
                "in Turbo Track.",
                "You will be guided",
                "to a Waiting Room soon."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
        return Err(Stop::End);
    } else {
        l_topbun = ((ctx
            .var("$ttranks")
            .get_at(runtime::index(&Val::from(0))?)?
            .try_rem(Val::from(3600))?)
        .try_div(Val::from(60))?);
        l_topcho = (ctx.var("$ttranks").get_at(runtime::index(&Val::from(0))?)?.try_rem(Val::from(60))?);
        l_gap = (ctx
            .var("$ttranks")
            .get_at(runtime::index(&Val::from(0))?)?
            .try_sub(l_record_time.clone())?);
        if l_gap.clone().number()? < 0 {
            ctx.lines_as(
                "Guide",
                args![
                    "Good work! Maybe you didn't",
                    "set any new records, but you",
                    "went the distance. Now, let",
                    "me relieve you of your fatigue~"
                ],
            )?;
            ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
            ctx.next()?;
            ctx.mes("[Guide]")?;
            if ctx.var("tt_point").get()?.number()? < 28991 {
                ctx.lines(args!["You will be rewarded", "with 10 Turbo Track points~!"])?;
                ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(10)))?;
                ctx.var("my_point").set(ctx.var("tt_point").get()?)?;
                ctx.lines(args![
                    "You now have a total of",
                    (ctx.var("tt_point").get()? + Val::from(" Turbo Track points."))
                ])?;
            } else {
                ctx.lines(args![
                    "Unfortunately, I can't give",
                    "you any Turbo Track Points",
                    "since you would exceed the",
                    "maximum limit. Sorry,",
                    (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("..."))
                ])?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Guide",
                args![
                    "Thank you for",
                    "participating in",
                    "in Turbo Track.",
                    "You will be guided",
                    "to a Waiting Room soon."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Guide",
                args![
                    "Congratulations!",
                    "You ranked as the",
                    "top player in Solo Mode!",
                    "Your name will be entered",
                    ((Val::from("into our records, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("~"))
                ],
            )?;
            let base = Val::from(0).number()?;
            ctx.var("$ttranks")
                .set_at(runtime::index(&Val::from(base + 0))?, l_record_time.clone())?;
            let base = Val::from(0).number()?;
            ctx.var("$ttnames$").set_at(
                runtime::index(&Val::from(base + 0))?,
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            )?;
            ctx.next()?;
            if ctx.var("tt_point").get()?.number()? < 28961 {
                ctx.lines_as(
                    "Guide",
                    args![
                        "Since you've set",
                        "a new record, you",
                        "will be rewarded with",
                        "40 Turbo Track Points!"
                    ],
                )?;
                ctx.var("tt_point").set((ctx.var("tt_point").get()? + Val::from(40)))?;
                ctx.lines(args![
                    "You now have a total of",
                    (ctx.var("tt_point").get()? + Val::from(" Turbo Track points."))
                ])?;
            } else {
                ctx.lines_as(
                    "Guide",
                    args![
                        "Unfortunately, I can't give",
                        "you any Turbo Track Points",
                        "since you would exceed the",
                        "maximum limit. Sorry,",
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("..."))
                    ],
                )?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Guide",
                args![
                    "Thank you for",
                    "participating in",
                    "in Turbo Track.",
                    "You will be guided",
                    "to a Waiting Room soon."
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(72), Val::from(89)])?;
            return Err(Stop::End);
        }
    }
}

pub fn guide_tbt_n1(ctx: &Ctx) -> Script {
    guide_tbt_n1_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Guide#TBT_") + l_w_s.clone())])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_onenable(ctx: &Ctx) -> Script {
    guide_tbt_n1_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("This is the ending broadcast of Turbo Track Solo Mode."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer4000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer8000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("For smooth game play, the game will end in approximately 1 minute from now."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer8000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer8000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer12000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("At this time, the warp portal will open."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer12000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer12000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer16000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("Players within the arena must be ready for this."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer16000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer16000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer20000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("In the meantime, the winner must procceed to receive rewards as soon as possible."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer20000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer20000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer24000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("Thank you for visiting Al De Baran Turbo Track."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer24000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer24000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer28000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("Al De Baran Turbo Track is brought to you by..."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer28000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer28000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer32000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("[Al De Baran Kafra Corporation Headquarters]"),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer32000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer32000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer36000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("[Al De Baran Guild Castle Management Luina]"),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer36000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer36000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer40000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("[Blacksmith Union]"),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer40000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer40000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer44000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("And [Comodo Casino]."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer44000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer44000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer48000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("Once again, we'd like to thank our sponsors."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer48000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer48000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer52000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("Have a good day."),
            ctx.constant("BC_MAP")?,
            Val::from("0x33FF66"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer52000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer52000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_ontimer56000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapWarp,
        vec![
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from("turbo_room"),
            Val::from(72),
            Val::from(89),
        ],
    )?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer56000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer56000_body(ctx, Vec::new()).map(|_| ())
}
