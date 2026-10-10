use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force09mobPriStep {
    Start,
    OnOn1,
    OnOn2,
    OnOn3,
    OnOn4,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_pri_run(ctx: &Ctx, mut step: Force09mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mobPriStep::Start => {
                step = Force09mobPriStep::OnOn1;
                continue 'machine;
            }
            Force09mobPriStep::OnOn1 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Mimic"),
                        Val::from(1474),
                        Val::from(1),
                        Val::from("force_09mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09mobPriStep::OnOn2 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Wrath Dead"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_09mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09mobPriStep::OnOn3 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Dark Illusion"),
                        Val::from(1605),
                        Val::from(1),
                        Val::from("force_09mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09mobPriStep::OnOn4 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Zombie Prisoner"),
                        Val::from(1480),
                        Val::from(1),
                        Val::from("force_09mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_09mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(88),
                        Val::from(89),
                        Val::from(111),
                        Val::from(110),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(4),
                        Val::from("force_09mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_09mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_09mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On09_End")])?;
                    ctx.var("$@arn_2").set(ctx.call(Function::GetTimeTick, vec![Val::from(2)])?)?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_09")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_pri(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_pri_onon1(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::OnOn1, Vec::new()).map(|_| ())
}

pub fn force_09mob_pri_onon2(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::OnOn2, Vec::new()).map(|_| ())
}

pub fn force_09mob_pri_onon3(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::OnOn3, Vec::new()).map(|_| ())
}

pub fn force_09mob_pri_onon4(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::OnOn4, Vec::new()).map(|_| ())
}

pub fn force_09mob_pri_onreset(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_pri_run(ctx, Force09mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force0801AcoStep {
    Start,
    OnTouch,
}

fn force_08_01_aco_run(ctx: &Ctx, mut step: Force0801AcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force0801AcoStep::Start => {
                step = Force0801AcoStep::OnTouch;
                continue 'machine;
            }
            Force0801AcoStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("force_5-1"), Val::from(40), Val::from(26)])?;
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::EnableNpc, vec![Val::from("force_01start#aco")])?;
                } else {
                    ctx.call(Function::EnableNpc, vec![Val::from("force_01start#pri")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08_01_aco(ctx: &Ctx) -> Script {
    force_08_01_aco_run(ctx, Force0801AcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_08_01_aco_ontouch(ctx: &Ctx) -> Script {
    force_08_01_aco_run(ctx, Force0801AcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ForceExitAcoStep {
    Start,
    OnTouch,
}

fn force_exit_aco_run(ctx: &Ctx, mut step: ForceExitAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ForceExitAcoStep::Start => {
                step = ForceExitAcoStep::OnTouch;
                continue 'machine;
            }
            ForceExitAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnEnable")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(21), Val::from(35)],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_exit_aco(ctx: &Ctx) -> Script {
    force_exit_aco_run(ctx, ForceExitAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_exit_aco_ontouch(ctx: &Ctx) -> Script {
    force_exit_aco_run(ctx, ForceExitAcoStep::OnTouch, Vec::new()).map(|_| ())
}

fn staff_aco_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Staff",
        args![
            "Nice work!",
            "You might have failed",
            "the Time Attack battle,",
            "but you still earned a",
            "small Arena Point reward~"
        ],
    )?;
    ctx.next()?;
    if ctx.var("arena_point").get()? == 30000 {
        ctx.lines_as(
            "Staff",
            args![
                "Wait, I'm sorry, but you",
                "have too many Arena Points.",
                "Since you've reached the point",
                "limitation, I can't give you any point rewards until you spend",
                "some of your Arena Points."
            ],
        )?;
        ctx.next()?;
    } else {
        ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(1)))?;
    }
    ctx.lines_as(
        "Staff",
        args![
            "Well, let me guide",
            "you back outside.",
            "I hope you enjoyed",
            "your battle in the area~"
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
    ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
    return Err(Stop::End);
}

pub fn staff_aco_1(ctx: &Ctx) -> Script {
    staff_aco_1_body(ctx, Vec::new()).map(|_| ())
}

fn staff_aco_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_acotop_n_s = Val::from("");
    let mut l_acotop_t = Val::from(0);
    let mut l_end_timeaco = Val::from(0);
    let mut l_et_to_secaco = Val::from(0);
    let mut l_gapaco = Val::from(0);
    let mut l_hour_endaco = Val::from(0);
    let mut l_hour_startaco = Val::from(0);
    let mut l_min_endaco = Val::from(0);
    let mut l_min_startaco = Val::from(0);
    let mut l_record_houraco = Val::from(0);
    let mut l_record_minaco = Val::from(0);
    let mut l_record_secaco = Val::from(0);
    let mut l_record_timeaco = Val::from(0);
    let mut l_sec_endaco = Val::from(0);
    let mut l_sec_startaco = Val::from(0);
    let mut l_st_to_secaco = Val::from(0);
    let mut l_start_timeaco = Val::from(0);
    let mut l_topbunaco = Val::from(0);
    let mut l_topchoaco = Val::from(0);
    l_start_timeaco = ctx.var("$@arn_1").get()?;
    l_hour_startaco = (l_start_timeaco.clone().try_div(Val::from(10000))?);
    l_min_startaco = ((l_start_timeaco.clone().try_rem(Val::from(10000))?).try_div(Val::from(100))?);
    l_sec_startaco = (l_start_timeaco.clone().try_rem(Val::from(100))?);
    l_end_timeaco = ctx.var("$@arn_2").get()?;
    l_hour_endaco = (l_end_timeaco.clone().try_div(Val::from(10000))?);
    l_min_endaco = ((l_end_timeaco.clone().try_rem(Val::from(10000))?).try_div(Val::from(100))?);
    l_sec_endaco = (l_end_timeaco.clone().try_rem(Val::from(100))?);
    if (l_hour_startaco.clone() == 23 && l_hour_endaco.clone() == 0) {
        l_hour_endaco = Val::from(24);
    }
    l_st_to_secaco =
        (((l_hour_startaco.clone().try_mul(Val::from(3600))?) + (l_min_startaco.clone().try_mul(Val::from(60))?)) + l_sec_startaco.clone());
    l_et_to_secaco =
        (((l_hour_endaco.clone().try_mul(Val::from(3600))?) + (l_min_endaco.clone().try_mul(Val::from(60))?)) + l_sec_endaco.clone());
    l_record_timeaco = (l_et_to_secaco.clone().try_sub(l_st_to_secaco.clone())?);
    l_record_houraco = (l_record_timeaco.clone().try_div(Val::from(3600))?);
    l_record_minaco = ((l_record_timeaco.clone().try_rem(Val::from(3600))?).try_div(Val::from(60))?);
    l_record_secaco = (l_record_timeaco.clone().try_rem(Val::from(60))?);
    if (((l_record_timeaco.clone().number()? < 0 || l_record_houraco.clone().number()? < 0) || l_record_minaco.clone().number()? < 0)
        || l_record_secaco.clone().number()? < 0)
    {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as(
            "Staff",
            args![
                "How did you...?",
                "According to these re-",
                "Uh oh. These records",
                "got messed up somehow.",
                "Th-This isn't good at all!",
                "Now what am I gonna do?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Staff",
            args![
                "Well, I'll give you the",
                "benefit of the doubt and",
                "assume you completed the",
                "arena battle under the time",
                "limit. So, let me give you the",
                "standard Arena Point reward."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Staff",
            args![
                "I'm really sorry about",
                "this, especially if you",
                "broke some record, but",
                "all I can do is restore your",
                ((Val::from("HP and SP for you, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("."))
            ],
        )?;
        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
        ctx.next()?;
        ctx.mes("[Staff]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FLAG")?])?;
        ctx.lines(args!["Thank you.", "I hope you enjoy", "your time in the Arena~"])?;
        ctx.close_window()?;
        if ctx.var("arena_point").get()?.number()? < 29981 {
            ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(20)))?;
            ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
        } else {
            ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
        }
        return Err(Stop::End);
    } else {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
            l_acotop_t = ctx.var("$arn_acotop").get()?;
            l_acotop_n_s = ctx.var("$arn_acotopn$").get()?;
        } else if ctx.var("BaseLevel").get()?.number()? < 70 {
            l_acotop_t = ctx.var("$arn_pritop60").get()?;
            l_acotop_n_s = ctx.var("$arn_pritopn60$").get()?;
        } else if ctx.var("BaseLevel").get()?.number()? < 80 {
            l_acotop_t = ctx.var("$arn_pritop70").get()?;
            l_acotop_n_s = ctx.var("$arn_pritopn70$").get()?;
        } else if ctx.var("BaseLevel").get()?.number()? < 90 {
            l_acotop_t = ctx.var("$arn_pritop80").get()?;
            l_acotop_n_s = ctx.var("$arn_pritopn80$").get()?;
        } else {
            l_acotop_t = ctx.var("$arn_pritop90").get()?;
            l_acotop_n_s = ctx.var("$arn_pritopn90$").get()?;
        }
        l_topbunaco = ((l_acotop_t.clone().try_rem(Val::from(3600))?).try_div(Val::from(60))?);
        l_topchoaco = (l_acotop_t.clone().try_rem(Val::from(60))?);
        l_gapaco = (l_acotop_t.clone().try_sub(l_record_timeaco.clone())?);
        ctx.lines_as(
            "Staff",
            args![
                ((Val::from("^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000, right?")),
                "Hey, you did great! You",
                "completed this Arena Battle",
                ((((Val::from("in ") + l_record_minaco.clone()) + Val::from(" min and ")) + l_record_secaco.clone())
                    + Val::from(" seconds!"))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Staff",
            args![
                ((Val::from("Currently, ^3131FF") + l_acotop_n_s.clone()) + Val::from("^000000")),
                "is the top player, with a record",
                ((((Val::from("of ^3131FF") + l_topbunaco.clone()) + Val::from("^000000 minutes, ^3131FF")) + l_topchoaco.clone())
                    + Val::from("^000000 seconds, of the Acolyte Class Time Force Battle."))
            ],
        )?;
        ctx.next()?;
        if ((l_acotop_t.clone().number()? < 0 || l_topbunaco.clone().number()? < 0) || l_topchoaco.clone().number()? < 0) {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.lines_as(
                "Staff",
                args![
                    "Wait a second...",
                    "Your time is better",
                    "than that. Well now.",
                    "It's time I made a little",
                    "correction to the records."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
            ctx.lines_as("Staff", args!["Wow! A new record!", "Excellent!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args![
                    "^333333*Ahem*^000000 Oh wow!",
                    "A new record! Your",
                    "name will now be listed",
                    "under the Time Force Battle",
                    "Arena - Acolyte Class Record!"
                ],
            )?;
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                ctx.var("$arn_acotop").set(l_record_timeaco.clone())?;
                ctx.var("$arn_acotopn$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_aco")])?;
            } else {
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.var("$arn_pritop60").set(l_record_timeaco.clone())?;
                    ctx.var("$arn_pritopn60$")
                        .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                    ctx.var("$arn_pritop70").set(l_record_timeaco.clone())?;
                    ctx.var("$arn_pritopn70$")
                        .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                    ctx.var("$arn_pritop80").set(l_record_timeaco.clone())?;
                    ctx.var("$arn_pritopn80$")
                        .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                } else {
                    ctx.var("$arn_pritop90").set(l_record_timeaco.clone())?;
                    ctx.var("$arn_pritopn90$")
                        .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                }
                ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_pri")])?;
            }
            ctx.next()?;
            if ctx.var("arena_point").get()? == 30000 {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Alright, let me reward you",
                        "with some Arena Poi--Wait.",
                        "I can't. Your Arena Points are",
                        "already maxed out. I'm sorry,",
                        "but you'll have to spend some before you can receive more points."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Anyway, you can check",
                        "your current Arena Point",
                        "total in the Arena Lobby",
                        "with Vendigos. Well, I hope",
                        "you enjoyed your battle. Now, let me guide you back outside..."
                    ],
                )?;
                ctx.close_window()?;
            } else if ctx.var("arena_point").get()?.number()? > 29950 {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Let me reward you with some",
                        "Arena Points. This time, you'll",
                        "be getting more points since",
                        "you set a new record. Please",
                        "talk with ^3131FFVendigos^000000 in the lobby",
                        "to check your new point total."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Well, I hope you",
                        "enjoyed your battle.",
                        "Now let me guide you",
                        "back to the Arena Lobby..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("arena_point").set(Val::from(30000))?;
            } else {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Let me reward you with some",
                        "Arena Points. This time, you'll",
                        "be getting more points since",
                        "you set a new record. Please",
                        "talk with ^3131FFVendigos^000000 in the lobby",
                        "to check your new point total."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Well, I hope you",
                        "enjoyed your battle.",
                        "Now let me guide you",
                        "back to the Arena Lobby..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(50)))?;
            }
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
            ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("allkill#aco::OnEnable")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
            return Err(Stop::End);
        }
        if ctx.var("gapaco").get()?.number()? < 0 {
            ctx.lines_as(
                "Staff",
                args![
                    "You didn't break the",
                    "current record this time,",
                    "but you still gave an awesome",
                    "performance. Excellent work!"
                ],
            )?;
            ctx.next()?;
            if ctx.var("arena_point").get()? == 30000 {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Alright, let me reward you",
                        "with some Arena Poi--Wait.",
                        "I can't. Your Arena Points are",
                        "already maxed out. I'm sorry,",
                        "but you'll have to spend some before you can receive more points."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Anyway, you can check",
                        "your current Arena Point",
                        "total in the Arena Lobby",
                        "with Vendigos. Well, I hope",
                        "you enjoyed your battle. Now, let me guide you back outside..."
                    ],
                )?;
                ctx.close_window()?;
            } else if ctx.var("arena_point").get()?.number()? > 29980 {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Let me reward you",
                        "with some Arena Points.",
                        "Please check your new",
                        "Arena Point total in the",
                        "Arena Lobby by speaking",
                        "to the friendly ^3131FFVendigos.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Well, I hope you",
                        "enjoyed your battle.",
                        "Now let me guide you",
                        "back to the Arena Lobby..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("arena_point").set(Val::from(30000))?;
            } else {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Let me reward you",
                        "with some Arena Points.",
                        "Please check your new",
                        "Arena Point total in the",
                        "Arena Lobby by speaking",
                        "to the friendly ^3131FFVendigos.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Well, I hope you",
                        "enjoyed your battle.",
                        "Now let me guide you",
                        "back to the Arena Lobby..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(20)))?;
            }
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
            ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("allkill#aco::OnEnable")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
            return Err(Stop::End);
        } else {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.lines_as("Staff", args!["Wow! A new record!", "This is awsome!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args![
                    "Oooh... A brand new",
                    ((Val::from("record. ^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000, your")),
                    "name will now be listed in",
                    "the Time Force Battle Arena",
                    "Acolyte Class Records.",
                    "Congratulations~"
                ],
            )?;
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                ctx.var("$arn_acotop").set(l_record_timeaco.clone())?;
                ctx.var("$arn_acotopn$").set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Vendigos::OnLineRec_aco")])?;
            } else if ctx.var("BaseLevel").get()?.number()? < 70 {
                ctx.var("$arn_pritop60").set(l_record_timeaco.clone())?;
                ctx.var("$arn_pritopn60$")
                    .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Vendigos::OnLineRec_pri60")])?;
            } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                ctx.var("$arn_pritop70").set(l_record_timeaco.clone())?;
                ctx.var("$arn_pritopn70$")
                    .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Vendigos::OnLineRec_pri70")])?;
            } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                ctx.var("$arn_pritop80").set(l_record_timeaco.clone())?;
                ctx.var("$arn_pritopn80$")
                    .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Vendigos::OnLineRec_pri80")])?;
            } else {
                ctx.var("$arn_pritop90").set(l_record_timeaco.clone())?;
                ctx.var("$arn_pritopn90$")
                    .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Vendigos::OnLineRec_pri90")])?;
            }
            ctx.next()?;
            if ctx.var("arena_point").get()? == 30000 {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Alright, let me reward you",
                        "with some Arena Poi--Wait.",
                        "I can't. Your Arena Points are",
                        "already maxed out. I'm sorry,",
                        "but you'll have to spend some before you can receive more points."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Anyway, you can check",
                        "your current Arena Point",
                        "total in the Arena Lobby",
                        "with Vendigos. Well, I hope",
                        "you enjoyed your battle. Now, let me guide you back outside..."
                    ],
                )?;
                ctx.close_window()?;
            } else if ctx.var("arena_point").get()?.number()? > 29980 {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Let me reward you with some",
                        "Arena Points. This time, you'll",
                        "be getting more points since",
                        "you set a new record. Please",
                        "talk with ^3131FFVendigos^000000 in the lobby",
                        "to check your new point total."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Well, I hope you",
                        "enjoyed your battle.",
                        "Now let me guide you",
                        "back to the Arena Lobby..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("arena_point").set(Val::from(30000))?;
            } else {
                ctx.lines_as(
                    "Staff",
                    args![
                        "Let me reward you with some",
                        "Arena Points. This time, you'll",
                        "be getting more points since",
                        "you set a new record. Please",
                        "talk with ^3131FFVendigos^000000 in the lobby",
                        "to check your new point total."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Staff",
                    args![
                        "Well, I hope you",
                        "enjoyed your battle.",
                        "Now let me guide you",
                        "back to the Arena Lobby..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(20)))?;
            }
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
            ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("allkill#aco::OnEnable")])?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
        }
        return Err(Stop::End);
    }
}

pub fn staff_aco_2(ctx: &Ctx) -> Script {
    staff_aco_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnTimerAcoStep {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer60000,
    OnStop,
}

fn arn_timer_aco_run(ctx: &Ctx, mut step: ArnTimerAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnTimerAcoStep::Start => {
                step = ArnTimerAcoStep::OnEnable;
                continue 'machine;
            }
            ArnTimerAcoStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArnTimerAcoStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("This broadcast is to inform you about the Acolyte Class Arena."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimerAcoStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("For smoother game play, the Warp Portal in the Final Waiting Room will activate in 1 minute."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimerAcoStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("Thank you."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimerAcoStep::OnTimer60000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_aco::OnOut")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            ArnTimerAcoStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_timer_aco(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn arn_timer_aco_onenable(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn arn_timer_aco_ontimer2000(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arn_timer_aco_ontimer3000(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arn_timer_aco_ontimer4000(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn arn_timer_aco_ontimer60000(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arn_timer_aco_onstop(ctx: &Ctx) -> Script {
    arn_timer_aco_run(ctx, ArnTimerAcoStep::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnWarpAcoStep {
    Start,
    OnInit,
    OnOut,
    OnHide,
    OnTouch,
}

fn arn_warp_aco_run(ctx: &Ctx, mut step: ArnWarpAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnWarpAcoStep::Start => {
                step = ArnWarpAcoStep::OnInit;
                continue 'machine;
            }
            ArnWarpAcoStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("arn_warp_aco")])?;
                return Err(Stop::End);
            }
            ArnWarpAcoStep::OnOut => {
                ctx.call(Function::EnableNpc, vec![Val::from("arn_warp_aco")])?;
                return Err(Stop::End);
            }
            ArnWarpAcoStep::OnHide => {
                ctx.call(Function::DisableNpc, vec![Val::from("arn_warp_aco")])?;
                return Err(Stop::End);
            }
            ArnWarpAcoStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_aco::OnHide")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_warp_aco(ctx: &Ctx) -> Script {
    arn_warp_aco_run(ctx, ArnWarpAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn arn_warp_aco_oninit(ctx: &Ctx) -> Script {
    arn_warp_aco_run(ctx, ArnWarpAcoStep::OnInit, Vec::new()).map(|_| ())
}

pub fn arn_warp_aco_onout(ctx: &Ctx) -> Script {
    arn_warp_aco_run(ctx, ArnWarpAcoStep::OnOut, Vec::new()).map(|_| ())
}

pub fn arn_warp_aco_onhide(ctx: &Ctx) -> Script {
    arn_warp_aco_run(ctx, ArnWarpAcoStep::OnHide, Vec::new()).map(|_| ())
}

pub fn arn_warp_aco_ontouch(ctx: &Ctx) -> Script {
    arn_warp_aco_run(ctx, ArnWarpAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TimerAco2Step {
    Start,
    OnEnable,
    OnStop,
}

fn timer_aco2_run(ctx: &Ctx, mut step: TimerAco2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerAco2Step::Start => {
                step = TimerAco2Step::OnEnable;
                continue 'machine;
            }
            TimerAco2Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerAco2Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn timer_aco2(ctx: &Ctx) -> Script {
    timer_aco2_run(ctx, TimerAco2Step::Start, Vec::new()).map(|_| ())
}

pub fn timer_aco2_onenable(ctx: &Ctx) -> Script {
    timer_aco2_run(ctx, TimerAco2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn timer_aco2_onstop(ctx: &Ctx) -> Script {
    timer_aco2_run(ctx, TimerAco2Step::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AllkillAcoStep {
    Start,
    OnEnable,
}

fn allkill_aco_run(ctx: &Ctx, mut step: AllkillAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AllkillAcoStep::Start => {
                step = AllkillAcoStep::OnEnable;
                continue 'machine;
            }
            AllkillAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#aco::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#pri::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#pri::OnReset")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#pri")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#aco")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arn_warp_aco")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_aco::OnStop")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn allkill_aco(ctx: &Ctx) -> Script {
    allkill_aco_run(ctx, AllkillAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn allkill_aco_onenable(ctx: &Ctx) -> Script {
    allkill_aco_run(ctx, AllkillAcoStep::OnEnable, Vec::new()).map(|_| ())
}

fn arena_record_staff_aco_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_acotop_n_s = Val::from("");
    let mut l_acotop_t = Val::from(0);
    let mut l_acotopbun = Val::from(0);
    let mut l_acotopcho = Val::from(0);
    let mut l_pritop_n_s = Val::from("");
    let mut l_pritop_t = Val::from(0);
    let mut l_pritopbun = Val::from(0);
    let mut l_pritopcho = Val::from(0);
    ctx.lines_as(
        "Mathea",
        args![
            "Hello, I'm in charge of",
            "the Acolyte Class Records",
            "in the Arena. If you'd like to",
            "view the other records, please",
            "talk to the Arena Record Staff,",
            "Owen Kheuv, and he'll help you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mathea",
        args![
            "Would you like to",
            "see who are the top",
            "players in the Acolyte",
            "Class Arena Challenges?",
            "Please choose from the menu."
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[(Val::from(
            "Acolyte Mode:Priest - Level 70 or lower:Priest - Level 80 or lower:Priest - Level 90 or lower:Priest - Level 91 ~ ",
        ) + Val::from("99"))],
    )? {
        1 => {
            l_acotop_t = ctx.var("$arn_acotop").get()?;
            l_acotop_n_s = ctx.var("$arn_acotopn$").get()?;
            l_acotopbun = ((l_acotop_t.clone().try_rem(Val::from(3600))?).try_div(Val::from(60))?);
            l_acotopcho = (l_acotop_t.clone().try_rem(Val::from(60))?);
            ctx.lines_as(
                "Mathea",
                args![
                    ((Val::from("^3131FF") + l_acotop_n_s.clone()) + Val::from("^000000")),
                    "is the top player of the",
                    "Acolyte Mode, finishing",
                    ((Val::from("with a time of ^3131FF") + l_acotopbun.clone()) + Val::from("^000000 minutes")),
                    ((Val::from("and ^3131FF") + l_acotopcho.clone()) + Val::from("^000000 seconds. Thank you")),
                    "for participating in the Arena."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            l_pritop_t = ctx.var("$arn_pritop60").get()?;
            l_pritop_n_s = ctx.var("$arn_pritopn60$").get()?;
        }
        3 => {
            l_pritop_t = ctx.var("$arn_pritop70").get()?;
            l_pritop_n_s = ctx.var("$arn_pritopn70$").get()?;
        }
        4 => {
            l_pritop_t = ctx.var("$arn_pritop80").get()?;
            l_pritop_n_s = ctx.var("$arn_pritopn80$").get()?;
        }
        5 => {
            l_pritop_t = ctx.var("$arn_pritop90").get()?;
            l_pritop_n_s = ctx.var("$arn_pritopn90$").get()?;
        }
        _ => {}
    }
    l_pritopbun = ((l_pritop_t.clone().try_rem(Val::from(3600))?).try_div(Val::from(60))?);
    l_pritopcho = (l_pritop_t.clone().try_rem(Val::from(60))?);
    ctx.lines_as(
        "Mathea",
        args![
            ((Val::from("^3131FF") + l_pritop_n_s.clone()) + Val::from("^000000")),
            "is the top player of this",
            "Priest Mode, finishing ",
            ((Val::from("with a time of ^3131FF") + l_pritopbun.clone()) + Val::from(" minutes")),
            ((Val::from("and ^3131FF") + l_pritopcho.clone()) + Val::from(" seconds. Thank you")),
            "for participating in the Arena."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn arena_record_staff_aco(ctx: &Ctx) -> Script {
    arena_record_staff_aco_body(ctx, Vec::new()).map(|_| ())
}

fn arena_record_staff_aco_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("$arn_acotopn$").get()? == "" {
        ctx.var("$arn_pritop60").set(Val::from(480))?;
        ctx.var("$arn_pritop70").set(Val::from(480))?;
        ctx.var("$arn_pritop80").set(Val::from(480))?;
        ctx.var("$arn_pritop90").set(Val::from(480))?;
        ctx.var("$arn_acotop").set(Val::from(480))?;
        ctx.var("$arn_pritopn60$").set(Val::from("Default"))?;
        ctx.var("$arn_pritopn70$").set(Val::from("Default"))?;
        ctx.var("$arn_pritopn80$").set(Val::from("Default"))?;
        ctx.var("$arn_pritopn90$").set(Val::from("Default"))?;
        ctx.var("$arn_acotopn$").set(Val::from("Default"))?;
    }
    return Err(Stop::End);
}

pub fn arena_record_staff_aco_oninit(ctx: &Ctx) -> Script {
    arena_record_staff_aco_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VendigosStep {
    Start,
    OnLineRecAco,
    OnLineRecPri60,
    OnLineRecPri70,
    OnLineRecPri80,
    OnLineRecPri90,
}

fn vendigos_run(ctx: &Ctx, mut step: VendigosStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VendigosStep::Start => {
                step = VendigosStep::OnLineRecAco;
                continue 'machine;
            }
            VendigosStep::OnLineRecAco => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arena_room"),
                        ((Val::from("Vendigos: ") + ctx.var("$arn_acotopn$").get()?)
                            + Val::from(" has made a new record in the Arena Time Force Battle - Acolyte Mode. Congratulations!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VendigosStep::OnLineRecPri60 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arena_room"),
                        ((Val::from("Vendigos: ") + ctx.var("$arn_pritopn60$").get()?)
                            + Val::from(
                                " has made a new record in the Arena Time Force Battle - Priest: Level 70 or lower. Congratulations!",
                            )),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VendigosStep::OnLineRecPri70 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arena_room"),
                        ((Val::from("Vendigos: ") + ctx.var("$arn_pritopn70$").get()?)
                            + Val::from(
                                " has made a new record in the Arena Time Force Battle - Priest: Level 80 or lower. Congratulations!",
                            )),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VendigosStep::OnLineRecPri80 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arena_room"),
                        ((Val::from("Vendigos: ") + ctx.var("$arn_pritopn80$").get()?)
                            + Val::from(
                                " has made a new record in the Arena Time Force Battle - Priest: Level 90 or lower. Congratulations!",
                            )),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VendigosStep::OnLineRecPri90 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arena_room"),
                        ((((Val::from("Vendigos: ") + ctx.var("$arn_pritopn90$").get()?)
                            + Val::from(" has made a new record in the Arena Time Force Battle - Priest: Level 91~"))
                            + Val::from("99"))
                            + Val::from(". Congratulations!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn vendigos(ctx: &Ctx) -> Script {
    vendigos_run(ctx, VendigosStep::Start, Vec::new()).map(|_| ())
}

pub fn vendigos_onlinerec_aco(ctx: &Ctx) -> Script {
    vendigos_run(ctx, VendigosStep::OnLineRecAco, Vec::new()).map(|_| ())
}

pub fn vendigos_onlinerec_pri60(ctx: &Ctx) -> Script {
    vendigos_run(ctx, VendigosStep::OnLineRecPri60, Vec::new()).map(|_| ())
}

pub fn vendigos_onlinerec_pri70(ctx: &Ctx) -> Script {
    vendigos_run(ctx, VendigosStep::OnLineRecPri70, Vec::new()).map(|_| ())
}

pub fn vendigos_onlinerec_pri80(ctx: &Ctx) -> Script {
    vendigos_run(ctx, VendigosStep::OnLineRecPri80, Vec::new()).map(|_| ())
}

pub fn vendigos_onlinerec_pri90(ctx: &Ctx) -> Script {
    vendigos_run(ctx, VendigosStep::OnLineRecPri90, Vec::new()).map(|_| ())
}

fn guide_alias_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Alias",
        args![
            "Hello there, I'm here",
            "to inform you about",
            "the Izlude Arena's",
            "Acolyte Class Mode.",
            "My name is Alias,",
            "your Arena Guide."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args![
            "Now, there are two modes",
            "under Acolyte Class Mode.",
            "These are ^3131FFAcolyte Mode^000000 and",
            "^3131FFPriest Mode^000000. For both modes,",
            "you will need to wait inside the ^3131FFAcolyte Class Waiting Room^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args![
            "Once it's your turn,",
            "you'll be sent out of the",
            "waiting room and guided to",
            "the arena grounds. Oh, and",
            "the entrance fee is 1,000 zeny."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args![
            "It'll be handy to know",
            "that the Red Plants and",
            "Permeters in the Acolyte",
            "Class Mode will drop items",
            "and give experience. Other",
            "monsters, however, won't."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args![
            "Once you've been guided",
            "to the actual arena grounds,",
            "listen to ^3131FFTrocco^000000 for the mission objectives that you will have to",
            "complete within the time limit",
            "of ^3131FF8 minutes^000000. Don't forget~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args![
            "Upon completing the",
            "entire stage, you will be",
            "warped to the ^3131DDFinale Waiting",
            "Room^000000 where you'll be rewarded",
            "with Arena Points. But you must get your points within 1 minute."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args![
            "Once you're automatically",
            "sent outside of the Finale",
            "Waiting Room, you won't have",
            "the chance to get your Arena",
            "Points if you didn't get them",
            "there, so be careful~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alias",
        args!["Well, I hope you enjoy", "the Acolyte Mode Arena!", "Good luck and good fighting!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guide_alias(ctx: &Ctx) -> Script {
    guide_alias_body(ctx, Vec::new()).map(|_| ())
}

fn log_on_aco_arena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1357), Val::from(0)])?;
    if l_i.clone() == -1 {
        ctx.mes("^3355FFIncorrect Password.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == 0 {
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "!!CAUTION!!",
            args![
                "^3355FFThe following menu",
                "the record for that",
                "particular mode in the",
                "Arena Acolyte Class Mode.^000000"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[(Val::from("Cancel:Acolyte:~level 70:~level 80:~level 90:~level ") + Val::from("99"))],
        )? {
            1 => {
                ctx.lines(args!["^3355FFCommand has", "been canceled.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.var("$arn_acotop").set(Val::from(480))?;
                ctx.var("$arn_acotopn$").set(Val::from("Default"))?;
            }
            3 => {
                ctx.var("$arn_pritop60").set(Val::from(480))?;
                ctx.var("$arn_pritopn60$").set(Val::from("Default"))?;
            }
            4 => {
                ctx.var("$arn_pritop70").set(Val::from(480))?;
                ctx.var("$arn_pritopn70$").set(Val::from("Default"))?;
            }
            5 => {
                ctx.var("$arn_pritop80").set(Val::from(480))?;
                ctx.var("$arn_pritopn80$").set(Val::from("Default"))?;
            }
            6 => {
                ctx.var("$arn_pritop90").set(Val::from(480))?;
                ctx.var("$arn_pritopn90$").set(Val::from("Default"))?;
            }
            _ => {}
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn log_on_aco_arena(ctx: &Ctx) -> Script {
    log_on_aco_arena_body(ctx, Vec::new()).map(|_| ())
}

fn acolink_arena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1357), Val::from(0)])?;
    if l_i.clone() == -1 {
        ctx.lines(args!["Command has", "been canceled."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if l_i.clone() == 0 {
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "This NPC opens and",
            "closes the Warp Portal",
            "to the Arena's Acolyte",
            "Class Mode. Choose",
            "an option from the menu."
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Cancel:Warp ON:Warp OFF")])? {
            1 => {
                ctx.lines(args!["Command has", "been canceled."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(Function::EnableNpc, vec![Val::from("onlyaco#arena")])?;
                ctx.lines(args!["The Warp Portal", "will be opened shortly."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.call(Function::DisableNpc, vec![Val::from("onlyaco#arena")])?;
                ctx.lines(args!["The Warp Portal", "will be closed shortly."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn acolink_arena(ctx: &Ctx) -> Script {
    acolink_arena_body(ctx, Vec::new()).map(|_| ())
}
