use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum CursedSoulStep {
    Start,
    OnTouch,
    OnMyMobDead,
}

fn cursed_soul_run(ctx: &Ctx, mut step: CursedSoulStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_spell = Val::from(0);
    'machine: loop {
        match step {
            CursedSoulStep::Start => {
                step = CursedSoulStep::OnTouch;
                continue 'machine;
            }
            CursedSoulStep::OnTouch => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                if (ctx.var("sign_q").get()? == 83 || ctx.var("sign_q").get()? == 90) && ctx.var("sign_sq").get()? == 0 {
                    ctx.call(
                        Function::KillMonster,
                        vec![Val::from("niflheim"), Val::from("#Cursed Soul::OnMyMobDead")],
                    )?;
                    ctx.lines_as("Ashe Bruce", args!["Leave now, or I will", "remove you by force...."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ashe Bruce",
                        args!["....And...", "....Whatever you do...", "....Do NOT touch my books..."],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Pick up the 1st book.:Pick up the 2nd book.:Pick up the 3rd book.:Leave immediately.",
                        )],
                    )? {
                        1 => {
                            ctx.call(
                                Function::Monster,
                                vec![
                                    Val::from("niflheim"),
                                    Val::from(349),
                                    Val::from(259),
                                    Val::from("Rideword"),
                                    Val::from(1478),
                                    Val::from(1),
                                ],
                            )?;
                            ctx.lines_as(
                                "Ashe Bruce",
                                args![
                                    "...!...",
                                    "How dare you touch my books",
                                    "when I specifically said",
                                    "'Don't touch my books!'"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ashe Bruce",
                                args!["....!...Grrrrr!", "I shall tear you apart...!", "Be bound by an eternal curse...!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Ashe Bruce",
                                args![
                                    "...!...",
                                    "You dare touch my books?!",
                                    "Right after I said not",
                                    "to touch them...?!",
                                    "Foolish mortal!",
                                    "...BEGONE!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                            ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(34), Val::from(162)])?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Ashe Bruce",
                                args![
                                    "Muhahahaha....",
                                    "Stubborn mortal~!",
                                    "Fine! I will give you",
                                    "a fighting chance and let",
                                    "you cast a spell."
                                ],
                            )?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(ctx, &[Val::from("Clover:Klaatu:Kleitos")])?) == 2 {
                                l_spell = (l_spell.clone() + Val::from(1));
                            }
                            if Val::from(runtime::select_values(ctx, &[Val::from("Verit:Veritas:Verata")])?) == 3 {
                                l_spell = (l_spell.clone() + Val::from(1));
                            }
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Necktie:Necklace:Nero:^FFFFFFNictu^000000")],
                            )?) == 4
                            {
                                l_spell = (l_spell.clone() + Val::from(1));
                            }
                            if l_spell.clone() == 3 {
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 5 {
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args!["That was the right", "spell! But nothing", "happened. Madness!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args!["That spell...", "You removed my curse?", "I don't believe it. I'm free!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args![
                                            "But do not underestimate",
                                            "my awesome powers! In fact,",
                                            "I command you to take this",
                                            "book and give it to that",
                                            "weak, pathetic witch!"
                                        ],
                                    )?;
                                    if ctx.var("sign_q").get()? == 83 {
                                        if ctx.var("sign_sq").get()? == 0 {
                                            ctx.var("sign_sq").set(Val::from(1))?;
                                            ctx.call(Function::GetItem, vec![Val::from(7304), Val::from(1)])?;
                                        }
                                    } else if ctx.var("sign_q").get()? == 90 {
                                        ctx.var("sign_q").set(Val::from(91))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7304), Val::from(1)])?;
                                    }
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            } else {
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(345),
                                        Val::from(259),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(347),
                                        Val::from(261),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(344),
                                        Val::from(253),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(346),
                                        Val::from(251),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(349),
                                        Val::from(249),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(350),
                                        Val::from(260),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.call(
                                    Function::Monster,
                                    vec![
                                        Val::from("niflheim"),
                                        Val::from(353),
                                        Val::from(256),
                                        Val::from("Orc Skeleton"),
                                        Val::from(1462),
                                        Val::from(1),
                                        Val::from("#Cursed Soul::OnMyMobDead"),
                                    ],
                                )?;
                                ctx.lines_as(
                                    "Ashe Bruce",
                                    args!["Muhahahahahaha!", "That's not the right spell!", "Now, death awaits you!"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Ashe Bruce",
                                args!["Well then.", "Try not to trip on", "your feet in your", "rush to leave."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                return Err(Stop::End);
            }
            CursedSoulStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn cursed_soul(ctx: &Ctx) -> Script {
    cursed_soul_run(ctx, CursedSoulStep::Start, Vec::new()).map(|_| ())
}

pub fn cursed_soul_ontouch(ctx: &Ctx) -> Script {
    cursed_soul_run(ctx, CursedSoulStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn cursed_soul_onmymobdead(ctx: &Ctx) -> Script {
    cursed_soul_run(ctx, CursedSoulStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CrayuStep {
    Start,
    OnTouch,
}

fn crayu_run(ctx: &Ctx, mut step: CrayuStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CrayuStep::Start => {
                step = CrayuStep::OnTouch;
                continue 'machine;
            }
            CrayuStep::OnTouch => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                if ctx.var("sign_q").get()?.number()? < 75 {
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()?.number()? < 82 {
                    ctx.lines_as(
                        "Crayu",
                        args![
                            "Mountain sunset to the west",
                            "Where the purple dusk falls ",
                            "Surrounded by beautiful melody",
                            "^You become the key that ignores its master"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()? == 86 {
                    ctx.lines_as(
                        "Crayu",
                        args![
                            "Mountain sunset to the west",
                            "Where the purple dusk falls ",
                            "Surrounded by beautiful melody",
                            "You become the key that ignores its master"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Crayu",
                        args![
                            "Hello adventurer.",
                            "I know this is rather",
                            "abrupt, but what do",
                            "you think of Serin?"
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("She's good.:She's evil!:She could go either way.")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as("Crayu", args!["Hm...?", "And why do", "you think so?"])?;
                    ctx.next()?;
                    'b1: {
                        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Just my opinion.:It's the truth!")])?);
                        let mut matched1 = false;
                        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                            matched1 = true;
                        }
                        if matched1 {
                            if ctx.var("sign_sq").get()?.number()? > 1 {
                                ctx.var("sign_sq").set(Val::from(0))?;
                            } else {
                                ctx.var("sign_sq").set((ctx.var("sign_sq").get()? + Val::from(1)))?;
                            }
                        }
                        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                            matched1 = true;
                        }
                        if matched1 {
                            break 'b1;
                        }
                    }
                    ctx.lines_as("Crayu", args!["Ah, I understand.", "Now, how may I help you?"])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("What exactly is Niflheim?:How do I become one of the chosen?")],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Crayu",
                                args![
                                    "Niflheim is commonly",
                                    "known as the city of the",
                                    "dead, but it's also the",
                                    "resting place of warriors",
                                    "who failed to enter Valhalla."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crayu",
                                args![
                                    "Some heroes may think",
                                    "of Niflheim as the tragic end,",
                                    "but it really all depends on",
                                    "your point of view."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["What do I need", "to do to become one", "of the chosen warriors?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crayu",
                                args![
                                    "First and foremost,",
                                    "you must prove your courage.",
                                    "It will be up to you to decide",
                                    "how you will demonstrate your",
                                    "bravery. The gods will only",
                                    "be watching and judging."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Crayu",
                                args![
                                    "Hmm...",
                                    "I think it would",
                                    "be a good idea if",
                                    "you talk to someone",
                                    "in Niflheim named ^FF0000Gen^000000."
                                ],
                            )?;
                            ctx.var("sign_q").set(Val::from(87))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("sign_q").get()? == 86 {
                    ctx.lines(args![
                        "Hmm...",
                        "I think it would",
                        "be a good idea if",
                        "you talk to someone",
                        "in Niflheim named ^FF0000Gen^000000."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn crayu(ctx: &Ctx) -> Script {
    crayu_run(ctx, CrayuStep::Start, Vec::new()).map(|_| ())
}

pub fn crayu_ontouch(ctx: &Ctx) -> Script {
    crayu_run(ctx, CrayuStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KidniffStep {
    Start,
    OnTouch,
}

fn kidniff_run(ctx: &Ctx, mut step: KidniffStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KidniffStep::Start => {
                step = KidniffStep::OnTouch;
                continue 'machine;
            }
            KidniffStep::OnTouch => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                if ctx.var("sign_q").get()? == 83 {
                    if ctx.var("sign_sq").get()? == 0 {
                        ctx.lines_as(
                            "Alakina Ann",
                            args!["^333333*Cries*^000000", "Where am I?", "I... I wanna go home~"],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_sq").get()? == 1 {
                        ctx.lines_as(
                            "Alakina Ann",
                            args![
                                "W-will you help me get",
                                "back home? Please? I miss",
                                "my mommy and my daddy and",
                                "I don't know how I got here.",
                                "^333333*Sniff*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("What can I do?:You can't go back...")])? {
                            1 => {
                                ctx.lines_as(
                                    "Alakina Ann",
                                    args![
                                        "I dunno. I-I think a ",
                                        "singing man told me to",
                                        "go meet a witch, but it's",
                                        "too scary to go outside..."
                                    ],
                                )?;
                                ctx.var("sign_sq").set(Val::from(2))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Alakina Ann", args!["^333333*Sniff*^000000", "N-no...", "I wanna go home..."])?;
                                ctx.close_window()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else if ctx.var("sign_sq").get()? == 2 {
                        ctx.lines_as(
                            "Alakina Ann",
                            args!["^333333*Cries*^000000", "Where am I?", "I... I wanna go home~"],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if (ctx.var("sign_sq").get()? == 3 || ctx.var("sign_sq").get()? == 4) {
                        if ctx.call(Function::CountItem, vec![Val::from(7309)])?.number()? > 0 {
                            ctx.lines_as(
                                "Alakina Ann",
                                args!["I... I can use", "this to go back home?", "Thank you! Thank you so much!"],
                            )?;
                            ctx.call(
                                Function::DelItem,
                                vec![Val::from(7309), ctx.call(Function::CountItem, vec![Val::from(7309)])?],
                            )?;
                            ctx.var("sign_sq").set(Val::from(5))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Alakina Ann",
                                args![
                                    "How am I gonna",
                                    "find that witch?",
                                    "She's the only one",
                                    "who knows how to",
                                    "get me back home...",
                                    "^333333*Cries*^000000"
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("sign_sq").get()? == 5 {
                            ctx.lines_as(
                                "Alakina Ann",
                                args![
                                    "Y-you lied to me!",
                                    "This wing doesn't do",
                                    "anything! I-I'm still here",
                                    "in this scary place! ^333333*Cries*^000000"
                                ],
                            )?;
                            ctx.var("sign_sq").set(Val::from(6))?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Alakina Ann",
                                args![
                                    "You said that you",
                                    "could help me get home,",
                                    "but you didn't help me",
                                    "at all. Y-you lied to me!",
                                    "I... I hate you! ^333333*Wah~!*^000000"
                                ],
                            )?;
                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    ctx.lines_as(
                        "Alakina Ann",
                        args!["^333333*Cries*^000000", "Where am I?", "I... I wanna go home~"],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn kidniff(ctx: &Ctx) -> Script {
    kidniff_run(ctx, KidniffStep::Start, Vec::new()).map(|_| ())
}

pub fn kidniff_ontouch(ctx: &Ctx) -> Script {
    kidniff_run(ctx, KidniffStep::OnTouch, Vec::new()).map(|_| ())
}

fn mysterious_energy_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if ctx.var("sign_q").get()? == 101 {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        ctx.var("sign_q").set(Val::from(105))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_1(ctx: &Ctx) -> Script {
    mysterious_energy_1_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if (ctx.var("sign_q").get()? == 102 || ctx.var("sign_q").get()? == 104) {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        if ctx.var("sign_q").get()? == 102 {
            ctx.var("sign_q").set(Val::from(106))?;
        } else if ctx.var("sign_q").get()? == 104 {
            ctx.var("sign_q").set(Val::from(108))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_2(ctx: &Ctx) -> Script {
    mysterious_energy_2_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if ((ctx.var("sign_q").get()? == 103 || ctx.var("sign_q").get()? == 105) || ctx.var("sign_q").get()? == 108) {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        if ctx.var("sign_q").get()? == 103 {
            ctx.var("sign_q").set(Val::from(107))?;
        } else if ctx.var("sign_q").get()? == 105 {
            ctx.var("sign_q").set(Val::from(109))?;
        } else if ctx.var("sign_q").get()? == 108 {
            ctx.var("sign_q").set(Val::from(112))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_3(ctx: &Ctx) -> Script {
    mysterious_energy_3_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if ctx.var("sign_q").get()? == 109 {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        if ctx.var("sign_q").get()? == 109 {
            ctx.var("sign_q").set(Val::from(113))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_4(ctx: &Ctx) -> Script {
    mysterious_energy_4_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if (ctx.var("sign_q").get()? == 106 || ctx.var("sign_q").get()? == 112) {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        if ctx.var("sign_q").get()? == 106 {
            ctx.var("sign_q").set(Val::from(110))?;
        } else if ctx.var("sign_q").get()? == 112 {
            ctx.var("sign_q").set(Val::from(116))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_5(ctx: &Ctx) -> Script {
    mysterious_energy_5_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if ctx.var("sign_q").get()? == 107 {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        if ctx.var("sign_q").get()? == 107 {
            ctx.var("sign_q").set(Val::from(111))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 116 {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone(), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_6(ctx: &Ctx) -> Script {
    mysterious_energy_6_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if (ctx.var("sign_q").get()? == 110 || ctx.var("sign_q").get()? == 111) {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone()])?;
        if ctx.var("sign_q").get()? == 110 {
            ctx.var("sign_q").set(Val::from(114))?;
        } else if ctx.var("sign_q").get()? == 111 {
            ctx.var("sign_q").set(Val::from(115))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sign_q").get()? == 113 {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone(), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_7(ctx: &Ctx) -> Script {
    mysterious_energy_7_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_energy_8_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_crash_s = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    l_crash_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])?;
    if (ctx.var("sign_q").get()? == 114 || ctx.var("sign_q").get()? == 115) {
        shared::quests_the_sign_quest::f_signseal(ctx, vec![l_crash_s.clone(), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFYou sense a strange,",
            "mysterious energy emanating from this area. For some reason, this power strikes you with a faint feeling of sadness.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn mysterious_energy_8(ctx: &Ctx) -> Script {
    mysterious_energy_8_body(ctx, Vec::new()).map(|_| ())
}

fn fountain_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.var("sign_q").get()?.number()? < 118 {
        ctx.lines(args![
            "^3355FFThe water in this",
            "fountain looks clean",
            "enough to drink...^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Drink.:Don't Drink.")])? {
            1 => {
                ctx.mes("...")?;
                ctx.close_window()?;
                ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes("...")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args![
            "^3355FFThe water in this",
            "fountain looks clean",
            "enough to be bottled...^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Bottle the water.:Don't bottle the water.")])? {
            1 => {
                if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                    ctx.lines(args![
                        "^3355FFUnfortunately, it",
                        "looks like you don't",
                        "have enough inventory",
                        "space to carry any more items...^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0 {
                    ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(12020), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "^3355FFUnfortunately, it",
                        "looks like you don't",
                        "have any Empty Bottles",
                        "to carry any of this water... ^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.mes("...")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn fountain_sign(ctx: &Ctx) -> Script {
    fountain_sign_body(ctx, Vec::new()).map(|_| ())
}

pub fn geffenia_warp(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::Start, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_oninit(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnInit, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ontouch(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_onenable(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ondisable(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ontimer10000(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ontimer20000(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnTimer20000, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ontimer30000(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ontimer40000(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnTimer40000, Vec::new()).map(|_| ())
}

pub fn geffenia_warp_ontimer45000(ctx: &Ctx) -> Script {
    geffenia_warp_run(ctx, GeffeniaWarpStep::OnTimer45000, Vec::new()).map(|_| ())
}

fn fountain_s_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CountItem, vec![Val::from(7025)])?.number()? > 0 {
        ctx.lines(args![
            "^3355FFAs you approach the fountain,",
            "a strange light begins to emit from the Lucifer's Lament in your pocket and from something deep within",
            "the fountain's water.^000000"
        ])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LEVEL99")?])?;
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_LEVEL99")?])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Throw Lucifer's Lament into the fountain.:Ignore the light.")])? {
            1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("geffen"),
                        Val::from("With a flash of light from Geffen Fountain, the door to Geffenia has opened."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x9CFF00"),
                    ],
                )?;
                ctx.lines(args![
                    "^3355FFOnce the Lucifer's Lament",
                    "splashes into the water, the",
                    "light reveals a peculiar warp",
                    "in front of the fountain...^000000"
                ])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAPPILLAR2")?])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Geffenia Warp::OnEnable")])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes(".......")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines(args!["^3355FFThis is the", "Geffen Fountain.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn fountain_s(ctx: &Ctx) -> Script {
    fountain_s_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StarterSerinStep {
    Start,
    OnTouch,
    OnEnable,
}

fn starter_serin_run(ctx: &Ctx, mut step: StarterSerinStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StarterSerinStep::Start => {
                step = StarterSerinStep::OnTouch;
                continue 'machine;
            }
            StarterSerinStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#serin::OnStart")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Starter#serin")])?;
                return Err(Stop::End);
            }
            StarterSerinStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Starter#serin")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn starter_serin(ctx: &Ctx) -> Script {
    starter_serin_run(ctx, StarterSerinStep::Start, Vec::new()).map(|_| ())
}

pub fn starter_serin_ontouch(ctx: &Ctx) -> Script {
    starter_serin_run(ctx, StarterSerinStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn starter_serin_onenable(ctx: &Ctx) -> Script {
    starter_serin_run(ctx, StarterSerinStep::OnEnable, Vec::new()).map(|_| ())
}

fn warp_serin_run(ctx: &Ctx, mut step: WarpSerinStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WarpSerinStep::Start => {
                step = WarpSerinStep::OnDisable;
                continue 'machine;
            }
            WarpSerinStep::OnDisable => {
                step = WarpSerinStep::OnInit;
                continue 'machine;
            }
            WarpSerinStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#serin")])?;
                return Err(Stop::End);
            }
            WarpSerinStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(30), Val::from(156)])?;
                return Err(Stop::End);
            }
            WarpSerinStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#serin")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_serin(ctx: &Ctx) -> Script {
    warp_serin_run(ctx, WarpSerinStep::Start, Vec::new()).map(|_| ())
}

pub fn warp_serin_ondisable(ctx: &Ctx) -> Script {
    warp_serin_run(ctx, WarpSerinStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn warp_serin_oninit(ctx: &Ctx) -> Script {
    warp_serin_run(ctx, WarpSerinStep::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_serin_ontouch(ctx: &Ctx) -> Script {
    warp_serin_run(ctx, WarpSerinStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn warp_serin_onenable(ctx: &Ctx) -> Script {
    warp_serin_run(ctx, WarpSerinStep::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TimerSerinStep {
    Start,
    OnStart,
    OnTimer600000,
    OnTimer620000,
}

fn timer_serin_run(ctx: &Ctx, mut step: TimerSerinStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerSerinStep::Start => {
                step = TimerSerinStep::OnStart;
                continue 'machine;
            }
            TimerSerinStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerSerinStep::OnTimer600000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#serin::OnEnable")])?;
                return Err(Stop::End);
            }
            TimerSerinStep::OnTimer620000 => {
                ctx.var("$@sign_w2").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Starter#serin::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#serin::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#dummy::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#serin::OnReset")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn timer_serin(ctx: &Ctx) -> Script {
    timer_serin_run(ctx, TimerSerinStep::Start, Vec::new()).map(|_| ())
}

pub fn timer_serin_onstart(ctx: &Ctx) -> Script {
    timer_serin_run(ctx, TimerSerinStep::OnStart, Vec::new()).map(|_| ())
}

pub fn timer_serin_ontimer600000(ctx: &Ctx) -> Script {
    timer_serin_run(ctx, TimerSerinStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn timer_serin_ontimer620000(ctx: &Ctx) -> Script {
    timer_serin_run(ctx, TimerSerinStep::OnTimer620000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CallmonsterSerinStep {
    Start,
    Oncall,
    OnMyMobDead,
    OnReset,
}

fn callmonster_serin_run(ctx: &Ctx, mut step: CallmonsterSerinStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CallmonsterSerinStep::Start => {
                step = CallmonsterSerinStep::Oncall;
                continue 'machine;
            }
            CallmonsterSerinStep::Oncall => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(196),
                        Val::from(44),
                        Val::from("Dark Lord Incarnation"),
                        Val::from(1605),
                        Val::from(1),
                        Val::from("CallMonster#serin::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CallmonsterSerinStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("que_sign01"), Val::from("CallMonster#serin::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnEnable")])?;
                }
                return Err(Stop::End);
            }
            CallmonsterSerinStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_sign01"), Val::from("CallMonster#serin::OnMyMobDead")],
                )?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn callmonster_serin(ctx: &Ctx) -> Script {
    callmonster_serin_run(ctx, CallmonsterSerinStep::Start, Vec::new()).map(|_| ())
}

pub fn callmonster_serin_oncall(ctx: &Ctx) -> Script {
    callmonster_serin_run(ctx, CallmonsterSerinStep::Oncall, Vec::new()).map(|_| ())
}

pub fn callmonster_serin_onmymobdead(ctx: &Ctx) -> Script {
    callmonster_serin_run(ctx, CallmonsterSerinStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn callmonster_serin_onreset(ctx: &Ctx) -> Script {
    callmonster_serin_run(ctx, CallmonsterSerinStep::OnReset, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum StarterWitchStep {
    Start,
    OnTouch,
    OnEnable,
}

fn starter_witch_run(ctx: &Ctx, mut step: StarterWitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StarterWitchStep::Start => {
                step = StarterWitchStep::OnTouch;
                continue 'machine;
            }
            StarterWitchStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#witch::OnStart")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Starter#witch")])?;
                return Err(Stop::End);
            }
            StarterWitchStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Starter#witch")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn starter_witch(ctx: &Ctx) -> Script {
    starter_witch_run(ctx, StarterWitchStep::Start, Vec::new()).map(|_| ())
}

pub fn starter_witch_ontouch(ctx: &Ctx) -> Script {
    starter_witch_run(ctx, StarterWitchStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn starter_witch_onenable(ctx: &Ctx) -> Script {
    starter_witch_run(ctx, StarterWitchStep::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TimerWitchStep {
    Start,
    OnStart,
    OnTimer600000,
    OnTimer620000,
}

fn timer_witch_run(ctx: &Ctx, mut step: TimerWitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerWitchStep::Start => {
                step = TimerWitchStep::OnStart;
                continue 'machine;
            }
            TimerWitchStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerWitchStep::OnTimer600000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#witch::OnEnable")])?;
                return Err(Stop::End);
            }
            TimerWitchStep::OnTimer620000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Starter#witch::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Warp#witch::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("CallMonster#witch::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#witch::OnEnable")])?;
                ctx.var("$@sign_w1").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn timer_witch(ctx: &Ctx) -> Script {
    timer_witch_run(ctx, TimerWitchStep::Start, Vec::new()).map(|_| ())
}

pub fn timer_witch_onstart(ctx: &Ctx) -> Script {
    timer_witch_run(ctx, TimerWitchStep::OnStart, Vec::new()).map(|_| ())
}

pub fn timer_witch_ontimer600000(ctx: &Ctx) -> Script {
    timer_witch_run(ctx, TimerWitchStep::OnTimer600000, Vec::new()).map(|_| ())
}

pub fn timer_witch_ontimer620000(ctx: &Ctx) -> Script {
    timer_witch_run(ctx, TimerWitchStep::OnTimer620000, Vec::new()).map(|_| ())
}

fn warp_witch_run(ctx: &Ctx, mut step: WarpWitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WarpWitchStep::Start => {
                step = WarpWitchStep::OnDisable;
                continue 'machine;
            }
            WarpWitchStep::OnDisable => {
                step = WarpWitchStep::OnInit;
                continue 'machine;
            }
            WarpWitchStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Warp#witch")])?;
                return Err(Stop::End);
            }
            WarpWitchStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Warp#witch")])?;
                return Err(Stop::End);
            }
            WarpWitchStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("niflheim"), Val::from(30), Val::from(156)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_witch(ctx: &Ctx) -> Script {
    warp_witch_run(ctx, WarpWitchStep::Start, Vec::new()).map(|_| ())
}

pub fn warp_witch_ondisable(ctx: &Ctx) -> Script {
    warp_witch_run(ctx, WarpWitchStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn warp_witch_oninit(ctx: &Ctx) -> Script {
    warp_witch_run(ctx, WarpWitchStep::OnInit, Vec::new()).map(|_| ())
}

pub fn warp_witch_onenable(ctx: &Ctx) -> Script {
    warp_witch_run(ctx, WarpWitchStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn warp_witch_ontouch(ctx: &Ctx) -> Script {
    warp_witch_run(ctx, WarpWitchStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CallmonsterWitchStep {
    Start,
    Oncall,
    OnMyMobDead,
    OnReset,
}

fn callmonster_witch_run(ctx: &Ctx, mut step: CallmonsterWitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CallmonsterWitchStep::Start => {
                step = CallmonsterWitchStep::Oncall;
                continue 'machine;
            }
            CallmonsterWitchStep::Oncall => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(196),
                        Val::from(195),
                        Val::from("Dark Lord Incarnation"),
                        Val::from(1605),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(180),
                        Val::from(180),
                        Val::from("Ancient Mummy"),
                        Val::from(1522),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(210),
                        Val::from(210),
                        Val::from("Ancient Mummy"),
                        Val::from(1522),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(200),
                        Val::from(200),
                        Val::from("Ancient Wraith"),
                        Val::from(1475),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(200),
                        Val::from(180),
                        Val::from("Ancient Wraith"),
                        Val::from(1475),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(180),
                        Val::from(200),
                        Val::from("Rotten Corpse"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(205),
                        Val::from(205),
                        Val::from("Rotten Corpse"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(190),
                        Val::from(190),
                        Val::from("Rotten Corpse"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(205),
                        Val::from(190),
                        Val::from("Rotten Corpse"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_sign01"),
                        Val::from(190),
                        Val::from(205),
                        Val::from("Rotten Corpse"),
                        Val::from(1423),
                        Val::from(1),
                        Val::from("CallMonster#witch::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CallmonsterWitchStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("que_sign01"), Val::from("CallMonster#witch::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#witch::OnEnable")])?;
                }
                return Err(Stop::End);
            }
            CallmonsterWitchStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_sign01"), Val::from("CallMonster#witch::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn callmonster_witch(ctx: &Ctx) -> Script {
    callmonster_witch_run(ctx, CallmonsterWitchStep::Start, Vec::new()).map(|_| ())
}

pub fn callmonster_witch_oncall(ctx: &Ctx) -> Script {
    callmonster_witch_run(ctx, CallmonsterWitchStep::Oncall, Vec::new()).map(|_| ())
}

pub fn callmonster_witch_onmymobdead(ctx: &Ctx) -> Script {
    callmonster_witch_run(ctx, CallmonsterWitchStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn callmonster_witch_onreset(ctx: &Ctx) -> Script {
    callmonster_witch_run(ctx, CallmonsterWitchStep::OnReset, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SignW6Step {
    Start,
    OnTouch,
}

fn sign_w6_run(ctx: &Ctx, mut step: SignW6Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SignW6Step::Start => {
                step = SignW6Step::OnTouch;
                continue 'machine;
            }
            SignW6Step::OnTouch => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                if (ctx.call(Function::CountItem, vec![Val::from(7313)])? == 1
                    && ((ctx.var("sign_q").get()? != 124 || ctx.var("sign_q").get()? != 125) || ctx.var("sign_q").get()? != 126))
                {
                    ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(197), Val::from(190)])?;
                    return Err(Stop::End);
                }
                if ctx.var("$@sign_w1").get()? == 1 {
                    ctx.lines(args![
                        "^3355FFSome sort of",
                        "strange force",
                        "is blocking you",
                        "from entering.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.var("$@sign_w1").set(Val::from(1))?;
                    ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(197), Val::from(190)])?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn sign_w6(ctx: &Ctx) -> Script {
    sign_w6_run(ctx, SignW6Step::Start, Vec::new()).map(|_| ())
}

pub fn sign_w6_ontouch(ctx: &Ctx) -> Script {
    sign_w6_run(ctx, SignW6Step::OnTouch, Vec::new()).map(|_| ())
}
