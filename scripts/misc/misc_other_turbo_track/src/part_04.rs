use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum PointManagerTtStep {
    Start,
    SExchangePoints,
}

fn point_manager_tt_run(ctx: &Ctx, mut step: PointManagerTtStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    let mut l_my_arena_all = Val::from(0);
    let mut l_my_turbo_all = Val::from(0);
    let mut l_want_point = Val::from(0);
    let mut l_want_point1 = Val::from(0);
    'machine: loop {
        match step {
            PointManagerTtStep::Start => {
                ctx.lines_as(
                    "Turbo Track Point Manager",
                    args![
                        "Good day.",
                        "Did you enjoy your",
                        "time in Turbo Track?",
                        "How may I be of",
                        "assistance?"
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Check Points:Convert Points:^660000Conversion Info^000000")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1))
                        && !subject1.loosely_equals(&Val::from(2))
                        && !subject1.loosely_equals(&Val::from(3));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Turbo Track Point Manager",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                "you currently have",
                                ((Val::from("") + ctx.var("tt_point").get()?) + Val::from(" Turbo Track Points")),
                                ((Val::from("and ") + ctx.var("arena_point").get()?) + Val::from(" Arena Points."))
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Turbo Track Point Manager",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                "you currently have",
                                ((Val::from("") + ctx.var("tt_point").get()?) + Val::from(" Turbo Track Points")),
                                ((Val::from("and ") + ctx.var("arena_point").get()?) + Val::from(" Arena Points."))
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Turbo Track Point Manager",
                            args![
                                "Please choose from among the following Arena Point conversions. When converting more than",
                                "10 Arena Points at once, you",
                                "can only convert Arena Points",
                                "in ^4D4DFFmultiples of 10^000000."
                            ],
                        )?;
                        ctx.next()?;
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "2 AP -> 1 TTP:4 AP -> 2 TTP:6 AP -> 3 TTP:8 AP -> 4 TTP:10 AP or more:Cancel",
                                )],
                            )?);
                            let mut matched2 = false;
                            let no_case2 = !subject2.loosely_equals(&Val::from(1))
                                && !subject2.loosely_equals(&Val::from(2))
                                && !subject2.loosely_equals(&Val::from(3))
                                && !subject2.loosely_equals(&Val::from(4))
                                && !subject2.loosely_equals(&Val::from(5))
                                && !subject2.loosely_equals(&Val::from(6));
                            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                matched2 = true;
                            }
                            if matched2 {
                                point_manager_tt_run(
                                    ctx,
                                    PointManagerTtStep::SExchangePoints,
                                    vec![Val::from(28999), Val::from(2), Val::from(1)],
                                )?;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                matched2 = true;
                            }
                            if matched2 {
                                point_manager_tt_run(
                                    ctx,
                                    PointManagerTtStep::SExchangePoints,
                                    vec![Val::from(28998), Val::from(4), Val::from(2)],
                                )?;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                                matched2 = true;
                            }
                            if matched2 {
                                point_manager_tt_run(
                                    ctx,
                                    PointManagerTtStep::SExchangePoints,
                                    vec![Val::from(28997), Val::from(6), Val::from(2)],
                                )?;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                                matched2 = true;
                            }
                            if matched2 {
                                point_manager_tt_run(
                                    ctx,
                                    PointManagerTtStep::SExchangePoints,
                                    vec![Val::from(28996), Val::from(8), Val::from(4)],
                                )?;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as("Turbo Track Point Manager", args!["Please enter the number of times you wish to convert 10 Arena Points into Turbo Track Points. The largest value you may enter", "is 20. To cancel, enter '^3355FF0^000000.'"])?;
                                ctx.next()?;
                                let (input, status) = runtime::input_number(ctx, None, None)?;
                                l_input = input;
                                if l_input.clone().number()? <= 0 {
                                    ctx.lines_as("Turbo Track Point Manager", args!["You have", "canceled", "your request."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if l_input.clone().number()? > 20 {
                                    ctx.lines_as(
                                        "Turbo Track Point Manager",
                                        args![
                                            "Your request exceeds",
                                            "the maximum limit. Please",
                                            "enter a value no greater than 20."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    l_want_point1 = (Val::from(10).try_mul(l_input.clone())?);
                                    l_want_point = (Val::from(5).try_mul(l_input.clone())?);
                                    l_my_turbo_all = (ctx.var("tt_point").get()? + l_want_point.clone());
                                    l_my_arena_all = (ctx.var("arena_point").get()?.try_sub(l_want_point1.clone())?);
                                    if l_my_turbo_all.clone().number()? > 28999 {
                                        ctx.lines_as("Turbo Track Point Manager", args!["Unfortunately, your Turbo Track Points will exceed the maximum limit if we proceed with point conversion. Please spend more", "of your Turbo Track Points before using this service. Thank you."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if l_my_arena_all.clone().number()? < 0 {
                                        ctx.lines_as(
                                            "Turbo Track Point Manager",
                                            args![
                                                "I am sorry, but you do",
                                                "not have enough Arena Points",
                                                "to perform this Turbo Track",
                                                "Point conversion."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Turbo Track Point Manager",
                                            args![
                                                "You have converted",
                                                "10 Arena Points into",
                                                ((Val::from("Turbo Track Points ") + l_input.clone()) + Val::from(" times.")),
                                                ((Val::from("A total of ") + l_want_point1.clone()) + Val::from(" Arena Points")),
                                                "has been converted into",
                                                ((Val::from("") + l_want_point.clone()) + Val::from(" Turbo Track Points."))
                                            ],
                                        )?;
                                        ctx.var("arena_point").set(l_my_arena_all.clone())?;
                                        ctx.var("tt_point").set(l_my_turbo_all.clone())?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Turbo Track Point Manager",
                                            args![
                                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                                "you now have",
                                                ((Val::from("^00688B") + ctx.var("tt_point").get()?)
                                                    + Val::from("^000000 Turbo Track Points")),
                                                ((Val::from("and ^4682B4") + ctx.var("arena_point").get()?)
                                                    + Val::from("^000000 Arena Points.")),
                                                "Thank you for your patronage."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(6)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as("Turbo Track Point Manager", args!["You have", "canceled", "your request."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = PointManagerTtStep::SExchangePoints;
                continue 'machine;
            }
            PointManagerTtStep::SExchangePoints => {
                if runtime::op(&ctx.var("tt_point").get()?, ">", &runtime::arg(&args, 0, Val::from(0)))?.is_true() {
                    ctx.lines_as("Turbo Track Point Manager", args!["Unfortunately, your Turbo Track Points will exceed the maximum limit if we proceed with point conversion. Please spend more", "of your Turbo Track Points before using this service. Thank you."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if runtime::op(&ctx.var("arena_point").get()?, ">=", &runtime::arg(&args, 1, Val::from(0)))?.is_true() {
                    ctx.lines_as(
                        "Turbo Track Point Manager",
                        args![
                            "As requested,",
                            "2 Arena Points",
                            "have been converted",
                            "into 1 Turbo Track Point."
                        ],
                    )?;
                    ctx.var("arena_point")
                        .set((ctx.var("arena_point").get()?.try_sub(runtime::arg(&args, 1, Val::from(0)))?))?;
                    ctx.var("tt_point")
                        .set((ctx.var("tt_point").get()? + runtime::arg(&args, 2, Val::from(0))))?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Turbo Track Point Manager",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                            "you now have",
                            ((Val::from("^00688B") + ctx.var("tt_point").get()?) + Val::from("^000000 Turbo Track Points")),
                            ((Val::from("and ^4682B4") + ctx.var("arena_point").get()?) + Val::from("^000000 Arena Points.")),
                            "Thank you for your patronage."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Turbo Track Point Manager",
                        args![
                            "I'm sorry, but you do not have enough Arena Points. You need",
                            "at least 2 Arena Points in order",
                            "to use this service."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn point_manager_tt(ctx: &Ctx) -> Script {
    point_manager_tt_run(ctx, PointManagerTtStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EnTurboStep {
    Start,
    OnTouch,
}

fn en_turbo_run(ctx: &Ctx, mut step: EnTurboStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_mount_s = Val::from("");
    'machine: loop {
        match step {
            EnTurboStep::Start => {
                step = EnTurboStep::OnTouch;
                continue 'machine;
            }
            EnTurboStep::OnTouch => {
                if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                    ctx.lines(args![
                        "^3355FFWait a second!",
                        "Right now, you're carrying",
                        "too many items with you.",
                        "Please come back after",
                        "putting storing some of your",
                        "things using the Kafra Service.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ((ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
                    || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
                    && (ctx.call(Function::CheckRiding, vec![])?.is_true() || Val::from(0).is_true()))
                {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 20000 {
                        if !(runtime::op(&ctx.call(Function::EaClass, vec![])?, "&", &ctx.constant("EAJL_THIRD")?)?.is_true()) {
                            l_mount_s = (if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                                Val::from("Peco Peco")
                            } else {
                                Val::from("Grand Peco")
                            });
                            ctx.lines_as(
                                "Helper",
                                args![
                                    (l_mount_s.clone() + Val::from("s are prohibited")),
                                    "in the Turbo Track Arena.",
                                    "Please dismount from your",
                                    (l_mount_s.clone() + Val::from(" and you will receive")),
                                    "a Free Ticket for Peco Ride",
                                    ((Val::from("for retrieving your ") + l_mount_s.clone()) + Val::from("."))
                                ],
                            )?;
                        } else {
                            l_mount_s = (if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                                Val::from("Dragon")
                            } else {
                                Val::from("Gryphon")
                            });
                            ctx.lines_as(
                                "Helper",
                                args![
                                    (l_mount_s.clone() + Val::from("s are prohibited")),
                                    "in the Turbo Track Arena.",
                                    "Please dismount from your",
                                    (l_mount_s.clone() + Val::from(". You can retrieve")),
                                    "it for free outside."
                                ],
                            )?;
                        }
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("alde_gld"), Val::from(183), Val::from(199)])?;
                        return Err(Stop::End);
                    } else {
                        if !(runtime::op(&ctx.call(Function::EaClass, vec![])?, "&", &ctx.constant("EAJL_THIRD")?)?.is_true()) {
                            ctx.call(Function::SetRiding, vec![Val::from(0)])?;
                            ctx.call(Function::GetItem, vec![Val::from(7310), Val::from(1)])?;
                        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                        } else {
                            ctx.call(Function::SetRiding, vec![Val::from(0)])?;
                        }
                        ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(100), Val::from(65)])?;
                    }
                } else {
                    if (((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_RANGER")?)
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_RANGER_T")?))
                        || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BABY_RANGER")?))
                        && Val::from(0).is_true())
                    {
                        if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 20000 {
                            ctx.lines_as(
                                "Helper",
                                args![
                                    "Wargs are prohibited",
                                    "in the Turbo Track Arena.",
                                    "Please dismount from your Warg."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("alde_gld"), Val::from(183), Val::from(199)])?;
                            return Err(Stop::End);
                        } else {
                            ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(100), Val::from(65)])?;
                        }
                    } else {
                        if (((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_MECHANIC")?)
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_MECHANIC_T")?))
                            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BABY_MECHANIC")?))
                            && ctx.call(Function::CheckMadogear, vec![])?.is_true())
                        {
                            if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 20000 {
                                ctx.lines_as(
                                    "Helper",
                                    args![
                                        "Magic Gears are prohibited",
                                        "in the Turbo Track Arena.",
                                        "Please dismount from your",
                                        "Magic Gear. You can retrieve",
                                        "it for free outside."
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::Warp, vec![Val::from("alde_gld"), Val::from(183), Val::from(199)])?;
                                return Err(Stop::End);
                            } else {
                                ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(100), Val::from(65)])?;
                            }
                        } else {
                            ctx.call(Function::Warp, vec![Val::from("turbo_room"), Val::from(100), Val::from(65)])?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn en_turbo(ctx: &Ctx) -> Script {
    en_turbo_run(ctx, EnTurboStep::Start, Vec::new()).map(|_| ())
}

pub fn en_turbo_ontouch(ctx: &Ctx) -> Script {
    en_turbo_run(ctx, EnTurboStep::OnTouch, Vec::new()).map(|_| ())
}

fn turbo_track_guide_entran_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Guide", args!["Welcome to", "the Al De Baran", "Turbo Track~"])?;
    ctx.next()?;
    ctx.mes("[Guide]")?;
    ctx.mes("Peco Pecos are prohibited inside the Turbo Track Arena.")?;
    ctx.mes("But anyone riding on a Peco Peco will receive a Free Peco Peco Mount Ticket at the Turbo Track Entrance and automatically dismount.")?;
    ctx.next()?;
    ctx.lines_as("Guide", args!["Well then,", "enjoy your time", "in Turbo Track~!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn turbo_track_guide_entran(ctx: &Ctx) -> Script {
    turbo_track_guide_entran_body(ctx, Vec::new()).map(|_| ())
}

fn mountmanager_turbo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_mount_s = Val::from("");
    let mut l_n_s = Val::from("");
    let mut l_riding = Val::from(0);
    let mut l_skill = Val::from(0);
    let mut l_skill_s = Val::from("");
    let mut l_zeny = Val::from(0);
    let mut l_zeny_s = Val::from("");
    l_n_s = ((Val::from("[") + ctx.call(Function::StrNpcInfo, vec![Val::from(1)])?) + Val::from("]"));
    ctx.lines(args![l_n_s.clone()])?;
    if ctx.call(Function::IsMounting, vec![])?.is_true() {
        ctx.lines(args![
            "Please get off of that creature you're riding on.",
            "Then talk to me again."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?)
        || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_CRUSADER")?))
    {
        if !(runtime::op(&ctx.call(Function::EaClass, vec![])?, "&", &ctx.constant("EAJL_THIRD")?)?.is_true()) {
            if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
                l_zeny = Val::from(2500);
                l_zeny_s = Val::from("2,500");
                l_mount_s = Val::from("Peco Peco");
            } else {
                l_zeny = Val::from(3500);
                l_zeny_s = Val::from("3,500");
                l_mount_s = Val::from("Grand Peco");
            }
            l_skill = Val::from(63);
            l_skill_s = Val::from("Peco Peco Ride");
            l_riding = ctx.call(Function::CheckRiding, vec![])?;
            l_i = Val::from(1);
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_KNIGHT")?) {
            l_mount_s = Val::from("Dragon");
            l_skill = Val::from(63);
            l_skill_s = Val::from("Dragon Training");
            l_riding = Val::from(0);
            l_i = Val::from(2);
        } else {
            l_mount_s = Val::from("Gryphon");
            l_skill = Val::from(63);
            l_skill_s = Val::from("Peco Peco Ride");
            l_riding = ctx.call(Function::CheckRiding, vec![])?;
            l_i = Val::from(1);
        }
    } else {
        if ((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_MECHANIC")?)
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_MECHANIC_T")?))
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_BABY_MECHANIC")?))
        {
            l_mount_s = Val::from("Magic Gear");
            l_skill = Val::from(2255);
            l_skill_s = Val::from("Magic Gear License");
            l_riding = ctx.call(Function::CheckMadogear, vec![])?;
            l_i = Val::from(3);
        } else {
            ctx.lines(args!["Thank you for", "visiting Al De Baran's", "Turbo Track~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines(args![
        "Welcome, would you like",
        ((Val::from("to retrieve your ") + l_mount_s.clone()) + Val::from("?"))
    ])?;
    if l_zeny.clone().is_true() {
        ctx.lines(args![
            "Please show me your Free",
            "Ticket for Peco Ride. You",
            "may also pay a rental fee",
            ((Val::from("of ") + l_zeny_s.clone()) + Val::from(" zeny."))
        ])?;
    }
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes:Cancel")])? {
        1 => {
            if ctx.call(Function::GetSkillLv, vec![l_skill.clone()])? == 0 {
                ctx.lines(args![
                    l_n_s.clone(),
                    "I'm sorry, but you're",
                    "not eligible for this",
                    "service. Please go learn",
                    ((Val::from("the ") + l_skill_s.clone()) + Val::from(" skill first."))
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if l_riding.clone().is_true() {
                ctx.lines(args![
                    l_n_s.clone(),
                    "You're already",
                    "mounted on a",
                    (l_mount_s.clone() + Val::from(".")),
                    "Thank you~"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if l_zeny.clone().is_true() {
                    if (ctx.call(Function::CountItem, vec![Val::from(7310)])?.number()? < 1
                        && runtime::op(&ctx.var("Zeny").get()?, "<", &l_zeny.clone())?.is_true())
                    {
                        ctx.lines(args![
                            l_n_s.clone(),
                            "I'm sorry, but you",
                            "don't have a Free Ticket",
                            ((Val::from("for Peco Ride or ") + l_zeny_s.clone()) + Val::from(" zeny.")),
                            "to use the Peco rental service."
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.call(Function::CountItem, vec![Val::from(7310)])?.number()? > 0 {
                        ctx.call(Function::DelItem, vec![Val::from(7310), Val::from(1)])?;
                    } else {
                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_zeny.clone())?))?;
                    }
                }
                let subject2 = l_i.clone();
                if subject2 == 1 {
                    ctx.call(Function::SetRiding, vec![])?;
                } else if subject2 == 2 {
                } else if subject2 == 3 {
                }
                ctx.lines(args![l_n_s.clone(), "Thank you for", "your patronage~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines(args![
                l_n_s.clone(),
                "Are you going",
                "back to race in",
                "the Turbo Track?",
                "Good luck!"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn mountmanager_turbo(ctx: &Ctx) -> Script {
    mountmanager_turbo_body(ctx, Vec::new()).map(|_| ())
}

fn sign_tbt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Al De Baran Turbo Track",
        args!["This way...", "to the Al De Baran", "Turbo Track Arena!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Al De Baran Turbo Track",
        args![
            "Turbo Track is",
            "an arena where",
            "participants compete to be the first to reach the Finish Line! Don't miss the chance to race against your friends!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sign_tbt(ctx: &Ctx) -> Script {
    sign_tbt_body(ctx, Vec::new()).map(|_| ())
}
