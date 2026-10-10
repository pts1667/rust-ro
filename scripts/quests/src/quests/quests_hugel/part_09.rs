use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Ashe4Step {
    Start,
    OnTouch,
}

fn ashe_4_run(ctx: &Ctx, mut step: Ashe4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ashe4Step::Start => {
                step = Ashe4Step::OnTouch;
                continue 'machine;
            }
            Ashe4Step::OnTouch => {
                if ctx.var("hg_odin").get()? == 59 {
                    ctx.lines_as("???", args!["...Silence."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "???",
                        args![
                            "Don't turn, don't",
                            "flinch, don't even",
                            "breathe. You're being",
                            "followed. Hold on a sec",
                            "while I take care of him."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(
                        Function::NpcSpecialEffect,
                        vec![ctx.constant("EF_SONICBLOWHIT")?, ctx.constant("AREA")?, Val::from("Hit")],
                    )?;
                    ctx.lines_as("???", args!["Ha!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "???",
                        args![
                            "...............................",
                            "Good. You've done some",
                            "good work. Just drop the file",
                            "to the ground. And for your",
                            "own good, don't turn around."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou drop the file",
                        "to the ground, and it",
                        "immediately vanishes in",
                        "a small whirlwind of sand.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "???",
                        args![
                            "I know that you've",
                            "gotten involved in all",
                            "this unexpectedly. Before",
                            "I leave, have you got any",
                            "questions? Otherwise, I'll go."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Who's Ashe?:What happened to Laura?")])? {
                        1 => {
                            ctx.lines_as(
                                "???",
                                args![
                                    "You'll never see her",
                                    "again. She's already",
                                    "been assigned to a new",
                                    "mission. If it makes you",
                                    "feel better, she's one of the",
                                    "members of ''Secret Wing.''"
                                ],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "???",
                                args![
                                    "She'll be fine. If you're that",
                                    "interested in archaeology,",
                                    "you'll be happy to know that",
                                    "she'll run the excavation the",
                                    "way she wants to. Rekenber and Arunafeltz should leave her alone."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "???",
                                args![
                                    "You're probably curious",
                                    "about Ashe, and perhaps",
                                    "myself as well. Now, have you",
                                    "ever heard of ''Secret Wing?''"
                                ],
                            )?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "???",
                        args![
                            "Secret Wing is a secret",
                            "organization that's scattered",
                            "now, but some of the members",
                            "are still active. Just consider us enemies of Rekenber Corporation",
                            "in the Schwarzwald Republic."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "???",
                        args![
                            "Everything you saw and",
                            "heard is all true. The ghost",
                            "of Giantes still remains within",
                            "the Odin Shrine. Rekenber wants",
                            "it for their ends, as well as",
                            "Arunafeltz. It's complicated..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "???",
                        args![
                            "The people of Rune-Midgarts",
                            "and Schwarzwald Republic still",
                            "don't know what's going on.",
                            "A lot of people are just being",
                            "used... Like that one lady,",
                            "that archaeologist, Laura."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::GetExperience, vec![Val::from(700000), Val::from(0)])?;
                    ctx.var("hg_odin").set(Val::from(60))?;
                    ctx.call(Function::CompleteQuest, vec![Val::from(11008)])?;
                    ctx.lines_as("???", args!["...Thanks for", "the file. Take care.", "Always watch your back."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn ashe_4(ctx: &Ctx) -> Script {
    ashe_4_run(ctx, Ashe4Step::Start, Vec::new()).map(|_| ())
}

pub fn ashe_4_ontouch(ctx: &Ctx) -> Script {
    ashe_4_run(ctx, Ashe4Step::OnTouch, Vec::new()).map(|_| ())
}

fn boatman_hugel_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Boatman",
        args![
            "Ah, hello~",
            "Would you like to sail",
            "to the Odin Shrine? The",
            "fare for 1 passenger is",
            "800 zeny. Shall we board?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("No, thanks.:Sure!")])? {
        1 => {
            ctx.lines_as(
                "Boatman",
                args![
                    "Ah, alright.",
                    "If you change your",
                    "mind, I invite you",
                    "to return and just",
                    "let me know."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            if ctx.var("Zeny").get()?.number()? < 800 {
                ctx.lines_as(
                    "Boatman",
                    args![
                        "Hm? You don't have enough",
                        "money to pay the fare. Well,",
                        "just come back when you do",
                        "have the zeny, and then I'll",
                        "take you to the Odin Shrine."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Boatman",
                args!["Alright then,", "climb aboard!", "We'll arrive near", "the Odin Shrine soon~"],
            )?;
            ctx.close_window()?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(800))?))?;
            if ctx.var("hg_odin").get()? == 2 {
                ctx.var("hg_odin").set(Val::from(3))?;
            } else if ctx.var("hg_odin").get()? == 4 {
                ctx.var("hg_odin").set(Val::from(5))?;
            } else if ctx.var("hg_odin").get()? == 12 {
                ctx.var("hg_odin").set(Val::from(13))?;
            } else if ctx.var("hg_odin").get()? == 14 {
                ctx.var("hg_odin").set(Val::from(15))?;
            }
            ctx.call(Function::Warp, vec![Val::from("odin_tem01"), Val::from(100), Val::from(146)])?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn boatman_hugel(ctx: &Ctx) -> Script {
    boatman_hugel_body(ctx, Vec::new()).map(|_| ())
}

fn boatman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Boatman",
        args![
            "Ah, would you",
            "like to sail back to",
            "Hugel now, or did you",
            "want to explore a bit more?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("I still need to look around...:Yes, take me back to Hugel.")])? {
        1 => {
            ctx.lines_as(
                "Boatman",
                args![
                    "Well, alright.",
                    "Just let me know",
                    "when you're ready to",
                    "leave this dangerous place."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Boatman",
                args!["Alright then,", "climb aboard!", "We'll arrive near", "Hugel very soon~"],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("hugel"), Val::from(206), Val::from(109)])?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn boatman(ctx: &Ctx) -> Script {
    boatman_body(ctx, Vec::new()).map(|_| ())
}

fn odininit_run(ctx: &Ctx, mut step: OdininitStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OdininitStep::Start => {
                step = OdininitStep::OnInit;
                continue 'machine;
            }
            OdininitStep::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            OdininitStep::OnTimer100000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                if subject1 == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#1::OnEnter")])?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#2::OnEnter")])?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#3::OnEnter")])?;
                    return Err(Stop::End);
                } else if subject1 == 4 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#4::OnEnter")])?;
                    return Err(Stop::End);
                } else if subject1 == 5 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#5::OnEnter")])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn odininit(ctx: &Ctx) -> Script {
    odininit_run(ctx, OdininitStep::Start, Vec::new()).map(|_| ())
}

pub fn odininit_oninit(ctx: &Ctx) -> Script {
    odininit_run(ctx, OdininitStep::OnInit, Vec::new()).map(|_| ())
}

pub fn odininit_ontimer100000(ctx: &Ctx) -> Script {
    odininit_run(ctx, OdininitStep::OnTimer100000, Vec::new()).map(|_| ())
}

pub fn warpinside_1(ctx: &Ctx) -> Script {
    warpinside_1_run(ctx, Warpinside1Step::Start, Vec::new()).map(|_| ())
}

pub fn warpinside_1_oninit(ctx: &Ctx) -> Script {
    warpinside_1_run(ctx, Warpinside1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warpinside_1_onenter(ctx: &Ctx) -> Script {
    warpinside_1_run(ctx, Warpinside1Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn warpinside_1_ontouch(ctx: &Ctx) -> Script {
    warpinside_1_run(ctx, Warpinside1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warpinside_2(ctx: &Ctx) -> Script {
    warpinside_2_run(ctx, Warpinside2Step::Start, Vec::new()).map(|_| ())
}

pub fn warpinside_2_oninit(ctx: &Ctx) -> Script {
    warpinside_2_run(ctx, Warpinside2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warpinside_2_onenter(ctx: &Ctx) -> Script {
    warpinside_2_run(ctx, Warpinside2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn warpinside_2_ontouch(ctx: &Ctx) -> Script {
    warpinside_2_run(ctx, Warpinside2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warpinside_3(ctx: &Ctx) -> Script {
    warpinside_3_run(ctx, Warpinside3Step::Start, Vec::new()).map(|_| ())
}

pub fn warpinside_3_oninit(ctx: &Ctx) -> Script {
    warpinside_3_run(ctx, Warpinside3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warpinside_3_onenter(ctx: &Ctx) -> Script {
    warpinside_3_run(ctx, Warpinside3Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn warpinside_3_ontouch(ctx: &Ctx) -> Script {
    warpinside_3_run(ctx, Warpinside3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warpinside_4(ctx: &Ctx) -> Script {
    warpinside_4_run(ctx, Warpinside4Step::Start, Vec::new()).map(|_| ())
}

pub fn warpinside_4_oninit(ctx: &Ctx) -> Script {
    warpinside_4_run(ctx, Warpinside4Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warpinside_4_onenter(ctx: &Ctx) -> Script {
    warpinside_4_run(ctx, Warpinside4Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn warpinside_4_ontouch(ctx: &Ctx) -> Script {
    warpinside_4_run(ctx, Warpinside4Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn warpinside_5(ctx: &Ctx) -> Script {
    warpinside_5_run(ctx, Warpinside5Step::Start, Vec::new()).map(|_| ())
}

pub fn warpinside_5_oninit(ctx: &Ctx) -> Script {
    warpinside_5_run(ctx, Warpinside5Step::OnInit, Vec::new()).map(|_| ())
}

pub fn warpinside_5_onenter(ctx: &Ctx) -> Script {
    warpinside_5_run(ctx, Warpinside5Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn warpinside_5_ontouch(ctx: &Ctx) -> Script {
    warpinside_5_run(ctx, Warpinside5Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EmptyStep {
    Start,
    OnTouch,
}

fn empty_run(ctx: &Ctx, mut step: EmptyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EmptyStep::Start => {
                step = EmptyStep::OnTouch;
                continue 'machine;
            }
            EmptyStep::OnTouch => {
                ctx.lines(args![
                    "^3355FFThis place is empty.",
                    "Everyone must",
                    "have already left.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn empty(ctx: &Ctx) -> Script {
    empty_run(ctx, EmptyStep::Start, Vec::new()).map(|_| ())
}

pub fn empty_ontouch(ctx: &Ctx) -> Script {
    empty_run(ctx, EmptyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Object1Step {
    Start,
    OnTouch,
}

fn object_1_run(ctx: &Ctx, mut step: Object1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Object1Step::Start => {
                step = Object1Step::OnTouch;
                continue 'machine;
            }
            Object1Step::OnTouch => {
                if ctx.var("hg_odin").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFA weathered structure",
                        "of the Odin Shrine is",
                        "half buried beneath the",
                        "ground, but it still looks",
                        "fairly stable and durable.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["......", "........."],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Pass:Explore")])? {
                        1 => {
                            ctx.lines(args!["^3355FFYou decided to pass", "by the shrine's remnants.^000000"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 1 {
                                ctx.lines(args![
                                    "^3355FFYou catch a bright",
                                    "shimmer among the",
                                    "remnants of the shrine.^000000"
                                ])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Ignore:Pick it up")])? {
                                    1 => {
                                        ctx.lines(args![
                                            "^3355FFYou decide to continue",
                                            "searching through the",
                                            "shrine's ruins.^000000"
                                        ])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines(args![
                                            "^3355FFYou find a shining stone",
                                            "within the ruins, and carefully",
                                            "put it into your bag.^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.mes("^FF0000Thud!^000000")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                            args!["What...?", "What was", "that noise?!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                                        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Wa...waaaahhhh!"])?;
                                        ctx.close_window()?;
                                        ctx.var("hg_odin").set(Val::from(22))?;
                                        ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(163), Val::from(31)])?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            ctx.lines(args![
                                "^3355FFUnfortunately, you",
                                "were unable to find",
                                "anything in the ruins.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_odin").get()? == 22 {
                    ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(163), Val::from(31)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn object_1(ctx: &Ctx) -> Script {
    object_1_run(ctx, Object1Step::Start, Vec::new()).map(|_| ())
}

pub fn object_1_ontouch(ctx: &Ctx) -> Script {
    object_1_run(ctx, Object1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Object2Step {
    Start,
    OnTouch,
}

fn object_2_run(ctx: &Ctx, mut step: Object2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Object2Step::Start => {
                step = Object2Step::OnTouch;
                continue 'machine;
            }
            Object2Step::OnTouch => {
                ctx.lines(args![
                    "^3355FFWhen you come back",
                    "to your senses, you",
                    "find that nothing is",
                    "left in this place.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn object_2(ctx: &Ctx) -> Script {
    object_2_run(ctx, Object2Step::Start, Vec::new()).map(|_| ())
}

pub fn object_2_ontouch(ctx: &Ctx) -> Script {
    object_2_run(ctx, Object2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Object3Step {
    Start,
    OnTouch,
}

fn object_3_run(ctx: &Ctx, mut step: Object3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Object3Step::Start => {
                step = Object3Step::OnTouch;
                continue 'machine;
            }
            Object3Step::OnTouch => {
                if ctx.var("hg_odin").get()? == 22 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Huh?", "What's happening?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Waaaaahhhh!"])?;
                    ctx.close_window()?;
                    ctx.var("hg_odin").set(Val::from(23))?;
                    ctx.call(Function::Warp, vec![Val::from("odin_tem03"), Val::from(264), Val::from(260)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn object_3(ctx: &Ctx) -> Script {
    object_3_run(ctx, Object3Step::Start, Vec::new()).map(|_| ())
}

pub fn object_3_ontouch(ctx: &Ctx) -> Script {
    object_3_run(ctx, Object3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlexWarpStep {
    Start,
    OnTouch,
}

fn alex_warp_run(ctx: &Ctx, mut step: AlexWarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlexWarpStep::Start => {
                step = AlexWarpStep::OnTouch;
                continue 'machine;
            }
            AlexWarpStep::OnTouch => {
                if ctx.var("hg_odin").get()? == 17 {
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Haven't you found it yet?",
                            "You said that it'd be here!",
                            "You told me so many people",
                            "are drawn to his place since",
                            "Ymir's Heart might really",
                            "be buried in this area!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Julian",
                        args!["I'm pretty sure it's", "around here somewhere,", "but I need more time to dig!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "I understand why you wanted",
                            "to follow me here, but there's",
                            "no way to confirm its existence",
                            "in this area without digging",
                            "it up. Plus, I'm still worried",
                            "about the ''Shinokas'' issue..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Shinokas?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "We couldn't find",
                            "Ymir's Heart inside",
                            "the village, either...",
                            "Why don't you just--"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Julian",
                        args![
                            "Yeah, you think you can do",
                            "everything yourself since",
                            "you're so much better than",
                            "me, huh?! I'm the black sheep",
                            "of the family, and father loves",
                            "you more than me, right?!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Alex", args!["You already know that."])?;
                    ctx.next()?;
                    ctx.lines_as("Julian", args!["Wha--?!", "Why... I...!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "Geez, you're impatient.",
                            "Listen, your ambitions",
                            "can ruin everything. You're",
                            "not planning on defying me,",
                            "are you? Because if you are..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Julian",
                        args![
                            "You can never understand",
                            "why I want to find Ymir's",
                            "Heart! I'm not going to give",
                            "up looking for it, so just",
                            "leave me alone!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Alex",
                        args![
                            "I know you better than",
                            "yourself, and I can do",
                            "everything better than you",
                            "can. Forget Ymir's Heart for",
                            "now. Giantes is our highest,",
                            "most important priority!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Julian",
                        args!["You always...!", "...............................", "Wait... You hear that?"],
                    )?;
                    ctx.next()?;
                    ctx.var("hg_odin").set(Val::from(18))?;
                    ctx.lines_as("Alex", args!["We'll continue this", "conversation later."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("hg_odin").get()? == 18 {
                    ctx.lines(args!["......", "........."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.call(Function::Warp, vec![Val::from("hu_in01"), Val::from(15), Val::from(155)])?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn alex_warp(ctx: &Ctx) -> Script {
    alex_warp_run(ctx, AlexWarpStep::Start, Vec::new()).map(|_| ())
}

pub fn alex_warp_ontouch(ctx: &Ctx) -> Script {
    alex_warp_run(ctx, AlexWarpStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlexWarp2Step {
    Start,
    OnTouch,
}

fn alex_warp2_run(ctx: &Ctx, mut step: AlexWarp2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlexWarp2Step::Start => {
                step = AlexWarp2Step::OnTouch;
                continue 'machine;
            }
            AlexWarp2Step::OnTouch => {
                if ctx.var("hg_odin").get()?.number()? > 59 {
                    ctx.call(Function::Warp, vec![Val::from("hu_in01"), Val::from(102), Val::from(90)])?;
                } else if (ctx.var("hg_odin").get()?.number()? > 21 && ctx.var("hg_odin").get()?.number()? < 60) {
                    ctx.call(Function::Warp, vec![Val::from("hu_in01"), Val::from(173), Val::from(90)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("hu_in01"), Val::from(33), Val::from(90)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn alex_warp2(ctx: &Ctx) -> Script {
    alex_warp2_run(ctx, AlexWarp2Step::Start, Vec::new()).map(|_| ())
}

pub fn alex_warp2_ontouch(ctx: &Ctx) -> Script {
    alex_warp2_run(ctx, AlexWarp2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlexWarp3Step {
    Start,
    OnTouch,
}

fn alex_warp3_run(ctx: &Ctx, mut step: AlexWarp3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlexWarp3Step::Start => {
                step = AlexWarp3Step::OnTouch;
                continue 'machine;
            }
            AlexWarp3Step::OnTouch => {
                ctx.mes("^3355FFThe door is locked.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alex_warp3(ctx: &Ctx) -> Script {
    alex_warp3_run(ctx, AlexWarp3Step::Start, Vec::new()).map(|_| ())
}

pub fn alex_warp3_ontouch(ctx: &Ctx) -> Script {
    alex_warp3_run(ctx, AlexWarp3Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AlexWarp4Step {
    Start,
    OnTouch,
}

fn alex_warp4_run(ctx: &Ctx, mut step: AlexWarp4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AlexWarp4Step::Start => {
                step = AlexWarp4Step::OnTouch;
                continue 'machine;
            }
            AlexWarp4Step::OnTouch => {
                ctx.mes("^3355FFThe door is locked.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn alex_warp4(ctx: &Ctx) -> Script {
    alex_warp4_run(ctx, AlexWarp4Step::Start, Vec::new()).map(|_| ())
}

pub fn alex_warp4_ontouch(ctx: &Ctx) -> Script {
    alex_warp4_run(ctx, AlexWarp4Step::OnTouch, Vec::new()).map(|_| ())
}
