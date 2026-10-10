use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn tao_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (((ctx.var("mao_request").get()? == 28 || ctx.var("mao_request").get()? == 29) || ctx.var("mao_request").get()? == 126)
        || ctx.var("mao_request").get()? == 127)
    {
        ctx.lines_as(
            "Tao",
            args![
                "Meow, what the hell",
                "happened to you? Kidd",
                "brought you in here, all",
                "beat up and asked me to",
                "take care of you till you woke",
                "up. Are you alright, meow?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tao",
            args![
                "Oh yes, as soon as you",
                "feel better, Kidd wants you",
                "to go to the commanding",
                "officer's room that's over",
                "to the left, meow. Okay?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Tao", args!["Mm...?", "What do you", "need, meow?"])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Ask about Mission"), Val::from("M-meow?")])? {
        1 => {
            if ctx.var("mao_request").get()?.number()? > 1 {
                ctx.lines_as(
                    "Tao",
                    args![
                        "The mission...?",
                        "Just enter the room",
                        "to the left. You can tell",
                        "left from right, meow?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Tao",
                args![
                    "Good luck, meow.",
                    "Tao keeps this place",
                    "safe from people that",
                    "don't belong here, meow."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Tao",
                args![
                    "Meow, meow.",
                    "It's a long story, meow.",
                    "Something about wanting",
                    "a Big Ribbon and hunting",
                    "Wild Roses, meow. It all started when meow, meow, meow, meow..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn tao(ctx: &Ctx) -> Script {
    tao_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Roombar1Step {
    Start,
    OnTouch,
}

fn roombar1_run(ctx: &Ctx, mut step: Roombar1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Roombar1Step::Start => {
                step = Roombar1Step::OnTouch;
                continue 'machine;
            }
            Roombar1Step::OnTouch => {
                if (((((((ctx.var("mao_request").get()? == 2 || ctx.var("mao_request").get()? == 24)
                    || ctx.var("mao_request").get()? == 28)
                    || ctx.var("mao_request").get()? == 29)
                    || ctx.var("mao_request").get()? == 123)
                    || ctx.var("mao_request").get()? == 126)
                    || ctx.var("mao_request").get()? == 127)
                    || ctx.var("prt_curse").get()? == 24)
                {
                    if !(ctx.var("$@maobar_room").get()?.is_true()) {
                        ctx.var("$@maobar_room").set(Val::from(1))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#maobartimer1::OnEnter")])?;
                        if ((((ctx.var("mao_request").get()? == 2 || ctx.var("mao_request").get()? == 28)
                            || ctx.var("mao_request").get()? == 29)
                            || ctx.var("mao_request").get()? == 126)
                            || ctx.var("mao_request").get()? == 127)
                        {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#1::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#1::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#1::OnEnter")])?;
                        } else if ctx.var("mao_request").get()? == 24 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#2::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#2::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_table::OnEnter")])?;
                        } else if ctx.var("mao_request").get()? == 123 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#2::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#2::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_table::OnEnter")])?;
                        } else if ctx.var("prt_curse").get()? == 24 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Marjana#poison::OnEnable")])?;
                        }
                        ctx.lines_as(
                            "Tao",
                            args![
                                "Ah, that place is protected",
                                "by security magic, so you'll",
                                "only have ^4D4DFF4 minutes^000000 to remain",
                                "there. Don't waste time, meow!"
                            ],
                        )?;
                        ctx.close_window()?;
                        if (ctx.var("mao_request").get()? == 24 || ctx.var("mao_request").get()? == 123) {
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(15), Val::from(8)])?;
                        } else {
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(11), Val::from(7)])?;
                        }
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Tao",
                        args![
                            "Sooo sorry, meow~",
                            "Someone else is already",
                            "inside. Just come back",
                            "again later, meow?"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Tao",
                    args![
                        "Wh-who are you?",
                        "This is a restricted",
                        "area, meow! If you don't",
                        "have permission, then get",
                        "out of there right meow!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn roombar1(ctx: &Ctx) -> Script {
    roombar1_run(ctx, Roombar1Step::Start, Vec::new()).map(|_| ())
}

pub fn roombar1_ontouch(ctx: &Ctx) -> Script {
    roombar1_run(ctx, Roombar1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobartimer1Step {
    Start,
    OnEnter,
    OnStop,
    OnTimer240000,
    OnTimer245000,
    OnTimer250000,
}

fn maobartimer1_run(ctx: &Ctx, mut step: Maobartimer1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maobartimer1Step::Start => {
                step = Maobartimer1Step::OnEnter;
                continue 'machine;
            }
            Maobartimer1Step::OnEnter => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("You will now enter the Master Zone, Area 1."),
                        Val::from(1),
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maobartimer1Step::OnStop => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("The security magic in the Master Zone, Area 1 is deactivated."),
                        Val::from(1),
                        Val::from(7396315),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar6::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Marjana#poison::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_table::OnInit")])?;
                ctx.var("$@maobar_room").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maobartimer1Step::OnTimer240000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar6::OnEnter")])?;
                return Err(Stop::End);
            }
            Maobartimer1Step::OnTimer245000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar6::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Marjana#poison::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_table::OnInit")])?;
                return Err(Stop::End);
            }
            Maobartimer1Step::OnTimer250000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("The security magic in the Master Zone, Area 1 is deactivated."),
                        Val::from(1),
                        Val::from(7396315),
                    ],
                )?;
                ctx.var("$@maobar_room").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maobartimer1(ctx: &Ctx) -> Script {
    maobartimer1_run(ctx, Maobartimer1Step::Start, Vec::new()).map(|_| ())
}

pub fn maobartimer1_onenter(ctx: &Ctx) -> Script {
    maobartimer1_run(ctx, Maobartimer1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maobartimer1_onstop(ctx: &Ctx) -> Script {
    maobartimer1_run(ctx, Maobartimer1Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maobartimer1_ontimer240000(ctx: &Ctx) -> Script {
    maobartimer1_run(ctx, Maobartimer1Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn maobartimer1_ontimer245000(ctx: &Ctx) -> Script {
    maobartimer1_run(ctx, Maobartimer1Step::OnTimer245000, Vec::new()).map(|_| ())
}

pub fn maobartimer1_ontimer250000(ctx: &Ctx) -> Script {
    maobartimer1_run(ctx, Maobartimer1Step::OnTimer250000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobar6Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maobar6_run(ctx: &Ctx, mut step: Maobar6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maobar6Step::Start => {
                step = Maobar6Step::OnInit;
                continue 'machine;
            }
            Maobar6Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maobar7")])?;
                return Err(Stop::End);
            }
            Maobar6Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maobar7")])?;
                return Err(Stop::End);
            }
            Maobar6Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maobar6(ctx: &Ctx) -> Script {
    maobar6_run(ctx, Maobar6Step::Start, Vec::new()).map(|_| ())
}

pub fn maobar6_oninit(ctx: &Ctx) -> Script {
    maobar6_run(ctx, Maobar6Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maobar6_onenter(ctx: &Ctx) -> Script {
    maobar6_run(ctx, Maobar6Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maobar6_ontouch(ctx: &Ctx) -> Script {
    maobar6_run(ctx, Maobar6Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Roombar2Step {
    Start,
    OnTouch,
}

fn roombar2_run(ctx: &Ctx, mut step: Roombar2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Roombar2Step::Start => {
                step = Roombar2Step::OnTouch;
                continue 'machine;
            }
            Roombar2Step::OnTouch => {
                if (ctx.var("mao_request").get()? == 25
                    || (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 123))
                {
                    if ctx.var("mao_request").get()? == 121 {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I can't go back to",
                                "Lin yet... I still need",
                                "to fully investigate",
                                "Thanatos Tower..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                        return Err(Stop::End);
                    }
                    if !(ctx.var("$@maobar_room2").get()?.is_true()) {
                        ctx.var("$@maobar_room2").set(Val::from(1))?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#maobartimer2::OnEnter")])?;
                        if ctx.var("mao_request").get()? == 25 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#3::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#3::OnEnter")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("#Rabsent::OnEnter")])?;
                        } else if ctx.var("mao_request").get()? == 122 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_empty::OnEnter")])?;
                        }
                        ctx.lines_as(
                            "Tao",
                            args![
                                "Ah, that place is protected",
                                "by security magic, so you'll",
                                "only have ^4D4DFF4 minutes^000000 to remain",
                                "there. Don't waste time, meow!"
                            ],
                        )?;
                        ctx.close_window()?;
                        if (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 122) {
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(80), Val::from(21)])?;
                        } else {
                            ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(144), Val::from(57)])?;
                        }
                        return Err(Stop::End);
                    }
                    ctx.mes("^3355FFThe door is locked.^000000")?;
                    if (ctx.var("mao_request").get()?.number()? > 102 && ctx.var("mao_request").get()?.number()? < 122) {
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Hey, R! It's me!", "Would you please", "open the door?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "R.",
                            args!["I-I'm sorry, but now's", "not a good time. Come", "back here later, please!"],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                        return Err(Stop::End);
                    }
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Tao",
                    args![
                        "This is a restricted area,",
                        "so if you don't have any",
                        "permission, then get out",
                        "of here, right meow!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn roombar2(ctx: &Ctx) -> Script {
    roombar2_run(ctx, Roombar2Step::Start, Vec::new()).map(|_| ())
}

pub fn roombar2_ontouch(ctx: &Ctx) -> Script {
    roombar2_run(ctx, Roombar2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobartimer2Step {
    Start,
    OnEnter,
    OnStop,
    OnTimer240000,
    OnTimer245000,
    OnTimer250000,
}

fn maobartimer2_run(ctx: &Ctx, mut step: Maobartimer2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maobartimer2Step::Start => {
                step = Maobartimer2Step::OnEnter;
                continue 'machine;
            }
            Maobartimer2Step::OnEnter => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("Security Level in the Master Zone, Area 2 has activated."),
                        Val::from(1),
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            Maobartimer2Step::OnStop => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("Security Level in the Master Zone, Area 2 deactivated."),
                        Val::from(1),
                        Val::from(7396315),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar7::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar8::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Rabsent::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_empty::OnInit")])?;
                ctx.var("$@maobar_room2").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Maobartimer2Step::OnTimer240000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar7::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar8::OnEnter")])?;
                return Err(Stop::End);
            }
            Maobartimer2Step::OnTimer245000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar7::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobar8::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Valdes#3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#3::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Rabsent::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_empty::OnInit")])?;
                return Err(Stop::End);
            }
            Maobartimer2Step::OnTimer250000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_job01"),
                        Val::from("Security Level in the Master Zone, Area 2 deactivated."),
                        Val::from(1),
                        Val::from(7396315),
                    ],
                )?;
                ctx.var("$@maobar_room2").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maobartimer2(ctx: &Ctx) -> Script {
    maobartimer2_run(ctx, Maobartimer2Step::Start, Vec::new()).map(|_| ())
}

pub fn maobartimer2_onenter(ctx: &Ctx) -> Script {
    maobartimer2_run(ctx, Maobartimer2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maobartimer2_onstop(ctx: &Ctx) -> Script {
    maobartimer2_run(ctx, Maobartimer2Step::OnStop, Vec::new()).map(|_| ())
}

pub fn maobartimer2_ontimer240000(ctx: &Ctx) -> Script {
    maobartimer2_run(ctx, Maobartimer2Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn maobartimer2_ontimer245000(ctx: &Ctx) -> Script {
    maobartimer2_run(ctx, Maobartimer2Step::OnTimer245000, Vec::new()).map(|_| ())
}

pub fn maobartimer2_ontimer250000(ctx: &Ctx) -> Script {
    maobartimer2_run(ctx, Maobartimer2Step::OnTimer250000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobar7Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maobar7_run(ctx: &Ctx, mut step: Maobar7Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maobar7Step::Start => {
                step = Maobar7Step::OnInit;
                continue 'machine;
            }
            Maobar7Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maobar7")])?;
                return Err(Stop::End);
            }
            Maobar7Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maobar7")])?;
                return Err(Stop::End);
            }
            Maobar7Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maobar7(ctx: &Ctx) -> Script {
    maobar7_run(ctx, Maobar7Step::Start, Vec::new()).map(|_| ())
}

pub fn maobar7_oninit(ctx: &Ctx) -> Script {
    maobar7_run(ctx, Maobar7Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maobar7_onenter(ctx: &Ctx) -> Script {
    maobar7_run(ctx, Maobar7Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maobar7_ontouch(ctx: &Ctx) -> Script {
    maobar7_run(ctx, Maobar7Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maobar8Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maobar8_run(ctx: &Ctx, mut step: Maobar8Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maobar8Step::Start => {
                step = Maobar8Step::OnInit;
                continue 'machine;
            }
            Maobar8Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maobar8")])?;
                return Err(Stop::End);
            }
            Maobar8Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maobar8")])?;
                return Err(Stop::End);
            }
            Maobar8Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maobar8(ctx: &Ctx) -> Script {
    maobar8_run(ctx, Maobar8Step::Start, Vec::new()).map(|_| ())
}

pub fn maobar8_oninit(ctx: &Ctx) -> Script {
    maobar8_run(ctx, Maobar8Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maobar8_onenter(ctx: &Ctx) -> Script {
    maobar8_run(ctx, Maobar8Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maobar8_ontouch(ctx: &Ctx) -> Script {
    maobar8_run(ctx, Maobar8Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maoexit1Step {
    Start,
    OnTouch,
}

fn maoexit1_run(ctx: &Ctx, mut step: Maoexit1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maoexit1Step::Start => {
                step = Maoexit1Step::OnTouch;
                continue 'machine;
            }
            Maoexit1Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobartimer1::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maoexit1(ctx: &Ctx) -> Script {
    maoexit1_run(ctx, Maoexit1Step::Start, Vec::new()).map(|_| ())
}

pub fn maoexit1_ontouch(ctx: &Ctx) -> Script {
    maoexit1_run(ctx, Maoexit1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maoexit2Step {
    Start,
    OnTouch,
}

fn maoexit2_run(ctx: &Ctx, mut step: Maoexit2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maoexit2Step::Start => {
                step = Maoexit2Step::OnTouch;
                continue 'machine;
            }
            Maoexit2Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobartimer2::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maoexit2(ctx: &Ctx) -> Script {
    maoexit2_run(ctx, Maoexit2Step::Start, Vec::new()).map(|_| ())
}

pub fn maoexit2_ontouch(ctx: &Ctx) -> Script {
    maoexit2_run(ctx, Maoexit2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maoexit3Step {
    Start,
    OnTouch,
}

fn maoexit3_run(ctx: &Ctx, mut step: Maoexit3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maoexit3Step::Start => {
                step = Maoexit3Step::OnTouch;
                continue 'machine;
            }
            Maoexit3Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("que_job01"), Val::from(52), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#maobartimer2::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn maoexit3(ctx: &Ctx) -> Script {
    maoexit3_run(ctx, Maoexit3Step::Start, Vec::new()).map(|_| ())
}

pub fn maoexit3_ontouch(ctx: &Ctx) -> Script {
    maoexit3_run(ctx, Maoexit3Step::OnTouch, Vec::new()).map(|_| ())
}

fn valdes_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7417), Val::from(1)])? != 1 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("mao_request").get()? == 2 {
        if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
        {
            ctx.lines_as(
                "Valdes",
                args![
                    "Welcome. Ah, this must be",
                    "your first assignment here.",
                    "I'm the commanding officer",
                    "of these missions, so please",
                    "listen to me very closely."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "The Assassin Guild agreed to",
                    "perform two missions that are",
                    "supposed to be related to the",
                    "missing children in Morocc.",
                    "You must choose one mission",
                    "in which you will participate."
                ],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as(
                "Valdes",
                args![
                    "Good, good, I've been",
                    "waiting for you. Since our",
                    "time is limited, I'll be brief.",
                    "I'm the commanding officer",
                    "for these two missions that the Assassin Guild has agreed to take."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Valdes",
                args![
                    "Due to the nature of these",
                    "missions and their objectives,",
                    "we have decided to recruit help",
                    "from outside. We recognize that",
                    "Assassins may not be suited",
                    "for some of the objectives..."
                ],
            )?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Valdes",
            args![
                "The client for the first",
                "mission is the Dandelion",
                "organization. If you accept,",
                "then you will work under Kidd",
                "and both of you will search and",
                "pursue a specified target."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Valdes",
            args![
                "The client for the other",
                "mission is an academic that",
                "has identified himself as ''R.'' His credentials check out,",
                "and it seems that all he has",
                "requested special protection."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Valdes",
            args![
                "I'll be honest: both of our",
                "clients strike me as suspicious, but if working with them brings",
                "us closer to helping those lost",
                "children, we've got to take that chance. I hope you understand."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Valdes",
            args![
                "Will you work as a hunter for the Dandelion organization, or will",
                "you work as Mr. R's bodyguard?",
                "Please choose the client whose",
                "job best suits your skills and",
                "abilities before anything else."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Dandelion Organization"), Val::from("Mr. R")])? {
            1 => {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "So you're choosing to",
                        "work for the Dandelion",
                        "organization? In that case, Kidd is your immediate superior.",
                        "I trust the two of you will work well together to finish this."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kidd", args!["Hey there~", "I'm Kidd.", "Nice to meet you."])?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_OK")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Kidd#1")])?,
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Alright, please take this",
                        "written request and meet",
                        "the Dandelion representative",
                        "in the pub. He'll brief you",
                        "further on your mission."
                    ],
                )?;
                ctx.var("mao_request").set(Val::from(3))?;
                ctx.call(Function::GetItem, vec![Val::from(7417), Val::from(1)])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Well then, Lin, I will",
                        "assign you to bodyguard",
                        "duty over Mr. R. Make sure",
                        "to keep a close eye on him.",
                        "That is all. Dismissed!"
                    ],
                )?;
                ctx.close_window()?;
            }
            2 => {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Alright. Lin will be",
                        "your immediate superior",
                        "in this mission. You'll be",
                        "working with her to ensure",
                        "that no harm befalls Mr. R."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args!["Heya, I'm Lin.", "I'm lookin' forward", "to working together", "with you. Heh heh~"],
                )?;
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_OK")?,
                        ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Lin#1")])?,
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Here, take this written",
                        "request to Mr. R who will",
                        "be waiting in the next room.",
                        "This document will prove to",
                        "him that you have been assigned",
                        "as his personal bodyguard."
                    ],
                )?;
                ctx.var("mao_request").set(Val::from(103))?;
                ctx.call(Function::GetItem, vec![Val::from(7418), Val::from(1)])?;
                ctx.close_window()?;
            }
            _ => {}
        }
    } else {
        if (ctx.var("mao_request").get()? == 3 || ctx.var("mao_request").get()? == 103) {
            if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
            {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "We're running out of",
                        "time. Hurry and meet the",
                        "Dandelion representative",
                        "in the pub so that you can",
                        "proceed with the mission."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Valdes",
                args![
                    "Please meet with the",
                    "representative from",
                    "Dandelion in the pub as",
                    "soon as possible. We can't",
                    "afford to waste time when",
                    "children's lives are at stake."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("mao_request").get()?.number()? > 3 && ctx.var("mao_request").get()?.number()? < 5) {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Your first priority is",
                        "to speak to Kidd, your",
                        "immediate superior in the",
                        "mission that you've accepted.",
                        "I trust that your work will please the Dandelion organization."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("mao_request").get()?.number()? > 103 && ctx.var("mao_request").get()?.number()? < 105) {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Your first priority is",
                        "to speak to Lin, your",
                        "immediate superior in the",
                        "mission that you've accepted.",
                        "Please do your best to protect Mr. R. as his personal bodyguard."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("mao_request").get()? == 28 || ctx.var("mao_request").get()? == 29) {
                ctx.lines_as(
                    "Valdes",
                    args![
                        "So how do you feel?",
                        "Although this mission",
                        "is technically a failure,",
                        "we shouldn't feel too bad.",
                        "Considering all the unknowns,",
                        "we did surprisingly well."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "W-wait; a minute...",
                        "Are you saying we stopped",
                        "Satan Morocc's revival, but",
                        "Raiyan Moore escaped?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args!["Yeah...", "That snake managed", "to get away in all of", "that confusion."],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["L-Lin...?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "Not only did he play",
                        "me for a fool, but he",
                        "escaped right before",
                        "my eyes! Don't bother",
                        "chasing him... I'll be",
                        "the one who turns him in!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kidd",
                    args![
                        "Lin, I understand how",
                        "you must feel, but don't",
                        "be so hard on yourself.",
                        "Lin, you did your job",
                        "perfectly, and there's no",
                        "way we could have known..."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Although I agree with Kidd,",
                        "I will entrust with the task",
                        "of bringing back Raiyan, Lin,",
                        "if that's what you really want.",
                        "Everyone else must feel very exhausted, so let's take a break."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "...I'm sorry, Valdes,",
                        "but I don't want to lose",
                        "Moore's trail. I'd better",
                        "go now while I can..."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#1::OnInit")])?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["She seems upset...."],
                )?;
                ctx.next()?;
                ctx.mes("[Valdes]")?;
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines(args![
                        "Well, as a fellow Assassin,",
                        "I'm sure you can understand",
                        "what's she going through.",
                        "We have too much pride to",
                        "take this kind of failure lightly. "
                    ])?;
                } else {
                    ctx.lines(args![
                        "Well, she's an Assassin.",
                        "All of us have too much pride",
                        "to accept any kind of failure.",
                        "It's just... It's just not in",
                        "our vocabulary, you see?"
                    ])?;
                }
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(7416), ctx.call(Function::CountItem, vec![Val::from(7416)])?],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(7417), ctx.call(Function::CountItem, vec![Val::from(7417)])?],
                )?;
                if ctx.var("mao_request").get()? == 28 {
                    ctx.call(Function::GetExperience, vec![Val::from(1050000), Val::from(0)])?;
                } else if ctx.var("mao_request").get()? == 29 {
                    ctx.call(Function::GetExperience, vec![Val::from(1280000), Val::from(0)])?;
                }
                ctx.var("mao_request").set(Val::from(30))?;
                ctx.call(Function::GetItem, vec![Val::from(12107), Val::from(1)])?;
                ctx.next()?;
                ctx.mes("[Valdes]")?;
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines(args![
                        "Regardless of our",
                        "original objective,",
                        "I must say that you",
                        "did a good job. We",
                        "didn't save the children,",
                        "but we stopped Satan Morocc."
                    ])?;
                } else {
                    ctx.lines(args![
                        "We failed our original",
                        "objective to save those kids,",
                        "but we did stop Satan Morocc.",
                        "You did a good job, and we'll",
                        "notify your guild of your good",
                        "work. Thanks for your help."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "Take care of yourself,",
                            "and take pride in the fact",
                            "that the Assassin Guild",
                            "considers you a valuable",
                            "ally. Be safe, adventurer..."
                        ],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Kidd, when you're",
                        "ready, I have another",
                        "assignment for you in",
                        "Prontera. But for now,",
                        "I want you to take it easy."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kidd",
                    args!["Heh. Alright,", "Valdes. I guess", "my work is never", "finished~"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#1::OnInit")])?;
            } else if (ctx.var("mao_request").get()? == 126 || ctx.var("mao_request").get()? == 127) {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "How are you feeling?",
                            "I've got some bad news:",
                            "All of us, every member of",
                            "the Assassin Guild... We",
                            "were tricked by Raiyan Moore..."
                        ],
                    )?;
                } else {
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "How are you feeling?",
                            "I've got some bad news:",
                            "these missions we were",
                            "assigned... They were all",
                            "part of an elaborate scheme",
                            "that all of us fell for..."
                        ],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "We were fooled by that",
                        "damned bastard. Everything",
                        "that Raiyan Moore wanted us",
                        "to do was for the sake of",
                        "Satan Morocc's reincarnation.",
                        "He almost got away with it..."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "He got away from us",
                        "this time... I'm sorry.",
                        "I should have caught him,",
                        "but he had help. I failed you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Lin..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kidd",
                    args![
                        "Lin, you don't gotta",
                        "apologize. There's no",
                        "way you could've known.",
                        "C'mon, we understand",
                        "how you feel, but it's not",
                        "your fault at all."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(0)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Kidd is right. We shouldn't",
                        "be blaming ourselves or ",
                        "each other. For now, we can",
                        "be satisfied with preventing",
                        "Satan Morocc's revival, and",
                        "then focus on Raiyan Moore."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(255)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "Don't worry, Valdes.",
                        "I'm going to take care of",
                        "Raiyan Moore. He's not going",
                        "to get away from me again."
                    ],
                )?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Well, Lin...",
                        "Although I'd prefer for",
                        "you to rest for now, I can",
                        "respect your conviction.",
                        "Alright, I'll entrust you with",
                        "the task of finding Moore."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("[Valdes]")?;
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines(args![
                        "Ah, and before I forget,",
                        "let me give you your fee",
                        "for taking this mission.",
                        "We may have technically",
                        "failed our original objective,",
                        "but you did very good work."
                    ])?;
                } else {
                    ctx.lines(args![
                        "Ah, and before I forget,",
                        "let me give you your fee",
                        "for taking this mission.",
                        "I thank you on behalf of",
                        "the Assassin Guild for",
                        "your efforts and hard work."
                    ])?;
                }
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(7416), ctx.call(Function::CountItem, vec![Val::from(7416)])?],
                )?;
                ctx.call(
                    Function::DelItem,
                    vec![Val::from(7418), ctx.call(Function::CountItem, vec![Val::from(7418)])?],
                )?;
                if ctx.var("mao_request").get()? == 126 {
                    ctx.call(Function::GetExperience, vec![Val::from(1050000), Val::from(0)])?;
                } else if ctx.var("mao_request").get()? == 127 {
                    ctx.call(Function::GetExperience, vec![Val::from(1280000), Val::from(0)])?;
                }
                ctx.var("mao_request").set(Val::from(128))?;
                ctx.call(Function::GetItem, vec![Val::from(12106), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Technically, we failed to",
                        "achieve our original mission",
                        "objective, but I will notify your guild and tell them that you were",
                        "instrumental in preventing",
                        "a worldwide catastrophe."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lin",
                    args![
                        "Valdes...",
                        "I'm leaving now.",
                        ((Val::from("Wish me luck, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(255)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#1::OnInit")])?;
                ctx.mes("[Valdes]")?;
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines(args!["Alright, then.", "You're dismissed.", "I'll see you next time."])?;
                } else {
                    ctx.lines(args![
                        "Alright, then.",
                        "You're dismissed.",
                        "Once again, thank you",
                        "for helping us. You've",
                        "been a valuable ally to the",
                        "Assassin Guild, so be proud."
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Valdes",
                    args![
                        "Kidd, when you're",
                        "ready, I have another",
                        "assignment for you in",
                        "Prontera. But for now,",
                        "I want you to take it easy."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kidd",
                    args!["Heh. Alright,", "Valdes. I guess", "my work is never", "finished~"],
                )?;
                ctx.close_window()?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#1::OnInit")])?;
            } else if (ctx.var("mao_request").get()? == 30 || ctx.var("mao_request").get()? == 128) {
                if (ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?)
                    || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN_CROSS")?))
                {
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "You may go ahead",
                            "and take a rest. The",
                            "last mission you took",
                            "was so critical, you",
                            "can afford to take",
                            "a short vacation."
                        ],
                    )?;
                } else {
                    ctx.lines_as(
                        "Valdes",
                        args![
                            "You've been of great",
                            "help to the Assassin",
                            "Guild. Your aid will",
                            "always be welcome here.",
                            "If it weren't for you, then",
                            "Satan Morocc would have..."
                        ],
                    )?;
                }
                ctx.close_window()?;
            }
        }
    }
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn valdes_1(ctx: &Ctx) -> Script {
    valdes_1_body(ctx, Vec::new()).map(|_| ())
}

fn valdes_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Valdes#1")])?;
    return Err(Stop::End);
}

pub fn valdes_1_oninit(ctx: &Ctx) -> Script {
    valdes_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn valdes_1_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Valdes#1")])?;
    return Err(Stop::End);
}

pub fn valdes_1_onenter(ctx: &Ctx) -> Script {
    valdes_1_onenter_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Valdes2Step {
    Start,
    OnInit,
    OnEnter,
}

fn valdes_2_run(ctx: &Ctx, mut step: Valdes2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Valdes2Step::Start => {
                step = Valdes2Step::OnInit;
                continue 'machine;
            }
            Valdes2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Valdes#2")])?;
                return Err(Stop::End);
            }
            Valdes2Step::OnEnter => {
                ctx.call(Function::DisableNpc, vec![Val::from("Valdes#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn valdes_2(ctx: &Ctx) -> Script {
    valdes_2_run(ctx, Valdes2Step::Start, Vec::new()).map(|_| ())
}

pub fn valdes_2_oninit(ctx: &Ctx) -> Script {
    valdes_2_run(ctx, Valdes2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn valdes_2_onenter(ctx: &Ctx) -> Script {
    valdes_2_run(ctx, Valdes2Step::OnEnter, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Valdes3Step {
    Start,
    OnInit,
    OnEnter,
}

fn valdes_3_run(ctx: &Ctx, mut step: Valdes3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Valdes3Step::Start => {
                step = Valdes3Step::OnInit;
                continue 'machine;
            }
            Valdes3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Valdes#3")])?;
                return Err(Stop::End);
            }
            Valdes3Step::OnEnter => {
                ctx.call(Function::DisableNpc, vec![Val::from("Valdes#3")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn valdes_3(ctx: &Ctx) -> Script {
    valdes_3_run(ctx, Valdes3Step::Start, Vec::new()).map(|_| ())
}

pub fn valdes_3_oninit(ctx: &Ctx) -> Script {
    valdes_3_run(ctx, Valdes3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn valdes_3_onenter(ctx: &Ctx) -> Script {
    valdes_3_run(ctx, Valdes3Step::OnEnter, Vec::new()).map(|_| ())
}

fn lin_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Cutin, vec![Val::from("mocseal_rin01.bmp"), Val::from(2)])?;
    if ctx.var("mao_request").get()?.number()? < 3 {
        ctx.lines_as(
            "Lin",
            args![
                "Hmm... If you're here",
                "for the reason that I think",
                "you're here, you should talk",
                "to our boss Valdes first."
            ],
        )?;
        ctx.close_window()?;
    } else if ctx.var("mao_request").get()? == 3 {
        ctx.lines_as(
            "Lin",
            args![
                "Hey. You're working",
                "with Kidd, right? He's",
                "right over there. Anyway,",
                "don't worry, he's pretty",
                "easy to get along with."
            ],
        )?;
        ctx.close_window()?;
    } else if ctx.var("mao_request").get()? == 103 {
        ctx.lines_as(
            "Lin",
            args![
                "First things first.",
                "We need to talk to our",
                "client, R, over in the next",
                "room. I'll meet you there."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Lin#1")])?;
    } else if (ctx.var("mao_request").get()? == 28 || ctx.var("mao_request").get()? == 29) {
        ctx.lines_as("Lin", args!["...", "......", "........."])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFLin seems extremely",
            "exhausted and depressed.",
            "For now, let's go talk to Valdes, the commanding officer.^000000"
        ])?;
        ctx.close_window()?;
    } else if (ctx.var("mao_request").get()? == 126 || ctx.var("mao_request").get()? == 127) {
        ctx.lines_as(
            "Lin",
            args![
                "H-hey...",
                "I hear that",
                "Kidd saved you.",
                "...............................",
                ((Val::from("Sorry, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                "I l-let you down..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFLin seems extremely",
            "exhausted and depressed.",
            "For now, let's go talk to Valdes, the commanding officer.^000000"
        ])?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn lin_1(ctx: &Ctx) -> Script {
    lin_1_body(ctx, Vec::new()).map(|_| ())
}

fn lin_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Lin#1")])?;
    return Err(Stop::End);
}

pub fn lin_1_oninit(ctx: &Ctx) -> Script {
    lin_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn lin_1_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Lin#1")])?;
    return Err(Stop::End);
}

pub fn lin_1_onenter(ctx: &Ctx) -> Script {
    lin_1_onenter_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Lin2Step {
    Start,
    OnInit,
    OnEnter,
}

fn lin_2_run(ctx: &Ctx, mut step: Lin2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Lin2Step::Start => {
                step = Lin2Step::OnInit;
                continue 'machine;
            }
            Lin2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#2")])?;
                return Err(Stop::End);
            }
            Lin2Step::OnEnter => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lin#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn lin_2(ctx: &Ctx) -> Script {
    lin_2_run(ctx, Lin2Step::Start, Vec::new()).map(|_| ())
}

pub fn lin_2_oninit(ctx: &Ctx) -> Script {
    lin_2_run(ctx, Lin2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn lin_2_onenter(ctx: &Ctx) -> Script {
    lin_2_run(ctx, Lin2Step::OnEnter, Vec::new()).map(|_| ())
}
