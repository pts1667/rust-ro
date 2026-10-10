use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hot_land_surface_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 8 {
        if (ctx.call(Function::CountItem, vec![Val::from(7704)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7342)])?.number()? > 0)
        {
            ctx.lines(args![
                "^3355FFYou use the pyrometer",
                "to check the surface",
                "temperature of the ground",
                "here in the volcano.^000000"
            ])?;
            ctx.next()?;
            ctx.mes("^3131FFBeep-- Beep-- Bee-^000000")?;
            ctx.next()?;
            ctx.mes("^3131FFCurrent Temperature: 2300 ThT^000000")?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou record the", "temperature in", "your report.^000000"])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2117), Val::from(2118)])?;
            ctx.var("aru_vol").set(Val::from(9))?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I should take a few more",
                    "temperature measurements",
                    "before I submit this report,",
                    "just to be absolutely sure."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou need both the",
            "pyrometer and the",
            "report form to measure",
            "and record the temperature",
            "of the ground's surface here.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis patch of ground",
        "emits an intense heat",
        "that stings your face.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hot_land_surface_1(ctx: &Ctx) -> Script {
    hot_land_surface_1_body(ctx, Vec::new()).map(|_| ())
}

fn hot_land_surface_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 9 {
        if (ctx.call(Function::CountItem, vec![Val::from(7704)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7342)])?.number()? > 0)
        {
            ctx.lines(args![
                "^3355FFYou use the pyrometer",
                "to check the surface",
                "temperature of the ground",
                "here in the volcano.^000000"
            ])?;
            ctx.next()?;
            ctx.mes("^3131FFBeep-- Beep-- Bee-^000000")?;
            ctx.next()?;
            ctx.mes("^3131FFCurrent Temperature: 2270 ThT^000000")?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou record the", "temperature in", "your report.^000000"])?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2118), Val::from(2119)])?;
            ctx.var("aru_vol").set(Val::from(10))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou need both the",
            "pyrometer and the",
            "report form to measure",
            "and record the temperature",
            "of the ground's surface here.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis patch of ground",
        "emits an intense heat",
        "that stings your face.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hot_land_surface_2(ctx: &Ctx) -> Script {
    hot_land_surface_2_body(ctx, Vec::new()).map(|_| ())
}

fn hot_land_surface_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 10 {
        if (ctx.call(Function::CountItem, vec![Val::from(7704)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(7342)])?.number()? > 0)
        {
            ctx.lines(args![
                "^3355FFYou use the pyrometer",
                "to check the surface",
                "temperature of the ground",
                "here in the volcano.^000000"
            ])?;
            ctx.next()?;
            ctx.mes("^3131FFBeep-- Beep-- Bee-^000000")?;
            ctx.next()?;
            ctx.mes("^3131FFCurrent Temperature: 2500 ThT^000000")?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou record the", "temperature in", "your report.^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "I've taken enough",
                    "measurements. I should",
                    "submit this report to the",
                    "geological camp now~"
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2119), Val::from(2120)])?;
            ctx.var("aru_vol").set(Val::from(11))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "^3355FFYou need both the",
            "pyrometer and the",
            "report form to measure",
            "and record the temperature",
            "of the ground's surface here.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 11 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I've taken enough",
                "measurements. I should",
                "submit this report to the",
                "geological camp now~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis patch of ground",
        "emits an intense heat",
        "that stings your face.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hot_land_surface_3(ctx: &Ctx) -> Script {
    hot_land_surface_3_body(ctx, Vec::new()).map(|_| ())
}

fn guard_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 11 {
        ctx.lines_as(
            "Guard",
            args!["Only authorized", "personnel can enter this", "area. Identify yourself!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I'm a research student working",
                "under Director Gio for the",
                "Veins Geological Research",
                "Institute. Would you please",
                "stamp this temperature",
                "report for me?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard",
            args![
                "Oh, I see. Well, I'm",
                "not the one that stamps",
                "reports. Go inside and",
                "ask Sahedi to help you.",
                "He's at the airship just",
                "south of the train station."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2120), Val::from(2121)])?;
        ctx.var("aru_vol").set(Val::from(12))?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(248), Val::from(190)])?;
        return Err(Stop::End);
    } else if (ctx.var("aru_vol").get()?.number()? > 11 && ctx.var("aru_vol").get()?.number()? < 24) {
        ctx.lines_as(
            "Guard",
            args![
                "Oh, you're that student",
                "from the institute. I don't",
                "think we're expecting",
                "any reports soon."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh, we just found out",
                "that the instruments we",
                "used were faulty, so we",
                "had to revise our report."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Guard",
            args![
                "Your tools were broken",
                "the first time? Okay, okay,",
                "I can understand that.",
                "Alright, you can pass."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(248), Val::from(190)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Guard", args!["Who are you?!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guard_vol(ctx: &Ctx) -> Script {
    guard_vol_body(ctx, Vec::new()).map(|_| ())
}

fn sahedi_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
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
    if ctx.var("aru_vol").get()? == 12 {
        ctx.lines_as(
            "Sahedi",
            args![
                "I'm sorry, but I don't",
                "think I know you. Only",
                "authorized personnel is",
                "allowed in this area, so",
                "if you don't have any",
                "reason to be here..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "I'm a research student working",
                "under Director Gio for the",
                "Veins Geological Research",
                "Institute. Would you please",
                "stamp this temperature",
                "report for me?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Um, Gio is sick right",
                "now, so that's why he",
                "had me fill out this report",
                "form and submit it for him."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args![
                "Ah, so that's why his",
                "report's late this time.",
                "I'm sorry to hear that.",
                "And here I thought he was",
                "just wasting his time on",
                "women and alcohol..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Sahedi", args!["Let's see..."])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as(
            "Sahedi",
            args![
                "Oh God! Why is the",
                "temperature so high?!",
                "We've had a few reports",
                "like this in the past, but...",
                "Is this... How bad is this?"
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_PROFUSELY_SWEAT")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh... Oh, no!",
                "Yikes! I guess if it's",
                "higher than normal...",
                "It might be bad?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args![
                "What should I do?",
                "Should I activate",
                "the alarm? I don't...",
                "I don't wanna die!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "You might want to calm",
                "down first. I'll take a look",
                "around the camp, so please",
                "don't say anything that will",
                "make anyone else panic for now."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args![
                "Okay...",
                "Please go ahead, and",
                "see if this camp will be",
                "safe from any disaster."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2121), Val::from(2122)])?;
        ctx.var("aru_vol").set(Val::from(13))?;
        ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("aru_vol").get()?.number()? > 12 && ctx.var("aru_vol").get()?.number()? < 23) {
        ctx.lines_as(
            "Sahedi",
            args![
                "So, are we in any",
                "danger? Does it look",
                "like this volcano will",
                "erupt anytime soon?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oh, I'm not finished", "investigating yet. Would", "you please wait a bit longer?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args![
                "Sure, sure. Just make",
                "sure that you do a real",
                "thorough check of everything",
                "in the volcano for me, yeah?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 23 {
        ctx.lines_as(
            "Sahedi",
            args![
                "So, are we in any",
                "danger? Does it look",
                "like this volcano will",
                "erupt anytime soon?"
            ],
        )?;
        ctx.next()?;
        'l1: loop {
            if !(true) {
                break 'l1;
            }
            'b1: {
                if Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?) == 1 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "I guess I can't hide it",
                            "from you... The recent",
                            "activity of this volcano",
                            "has recently been fairly...",
                            "disconcerting."
                        ],
                    )?;
                    ctx.next()?;
                    break 'l1;
                } else {
                    ctx.lines(args![
                        "^3355FFWait... You should take",
                        "advantage of this situation.",
                        "This could be your chance to",
                        "intervene in the conflict",
                        "between two contries!^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou might not be able",
                        "to stop their war, but",
                        "maybe you can distract",
                        "them with the threat",
                        "of natural disaster...^000000"
                    ])?;
                    ctx.next()?;
                }
            }
        }
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as(
            "Sahedi",
            args![
                "Huh? Are you serious?",
                "Thor Volcano's gonna",
                "erupt?! We-we have to",
                "get the hell out of here!",
                "It'll be a disaster, just",
                "like it happened in the past!"
            ],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SPARK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Yes, I agree. There's",
                "a good chance of an...",
                "explosion that'll cause",
                "a lot of collateral damage."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args!["What are our chances?", "How much time do we", "have to evacuate?"],
        )?;
        ctx.next()?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SPARK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Well... Uh...",
                "According to my data...",
                "Analysis... There's a 75%",
                "chance of eruption within",
                "the next thirty days."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args!["What?! We must report", "this to the high priest", "immediately! Aitra!"],
        )?;
        ctx.next()?;
        ctx.call(Function::EnableNpc, vec![Val::from("Aitra#vol")])?;
        ctx.lines_as("Aitra", args!["Yes, sir!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Sahedi",
            args![
                "This is an emergency.",
                "Bring this message to",
                "the high priest as soon",
                "as possible. And don't",
                "forget to pack all your",
                "things before you leave."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Aitra", args!["Huh?", " ...Yes, sir."])?;
        ctx.next()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Aitra#vol")])?;
        ctx.lines_as(
            "Sahedi",
            args![
                "Oh, this is a nightmare...",
                "Will you please take your",
                "report to your director, Gio?",
                "Hopefully he'll have some",
                "advice for what we can",
                "do about this disaster..."
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2128), Val::from(60211)])?;
        ctx.var("aru_vol").set(Val::from(24))?;
        ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Sahedi", args!["Argh, I'm so busy!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sahedi_vol(ctx: &Ctx) -> Script {
    sahedi_vol_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Colonel1Step {
    Start,
    OnTouch,
}

fn colonel1_run(ctx: &Ctx, mut step: Colonel1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Colonel1Step::Start => {
                step = Colonel1Step::OnTouch;
                continue 'machine;
            }
            Colonel1Step::OnTouch => {
                if ctx.var("aru_vol").get()? == 13 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Colonel Vito#1")])?;
                    ctx.lines_as(
                        "????",
                        args![
                            "You...!",
                            "What are you doing",
                            "just standing around?!",
                            "Aren't you supposed to",
                            "be transporting cargo? ",
                            "Attention to orders!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Are you talking to me?", "No, I'm from the Veins Geo--"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Colonel Vito",
                        args![
                            "Look at you. You don't",
                            "even have your uniform",
                            "yet. Still a rookie, eh?",
                            "Looks like I'll have to",
                            "personally train you as",
                            "one of our holy knights!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["But I'm not--"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Colonel Vito",
                        args![
                            "You should be honored to",
                            "have the rare opportunity",
                            "to be trained by me, the",
                            "great Colonel Vito. I'll mold",
                            "you into a true warrior for",
                            "Freya! Now follow me!"
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2122), Val::from(2123)])?;
                    ctx.var("aru_vol").set(Val::from(14))?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Colonel Vito#1")])?;
                    ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(156), Val::from(68)])?;
                    return Err(Stop::End);
                } else if ctx.var("aru_vol").get()? == 14 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Colonel Vito#1")])?;
                    ctx.lines_as(
                        "Colonel Vito",
                        args!["What are you still", "doing standing there?", "Don't slack off! Come!"],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Colonel Vito#1")])?;
                    ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(156), Val::from(68)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn colonel1(ctx: &Ctx) -> Script {
    colonel1_run(ctx, Colonel1Step::Start, Vec::new()).map(|_| ())
}

pub fn colonel1_ontouch(ctx: &Ctx) -> Script {
    colonel1_run(ctx, Colonel1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Colonel2Step {
    Start,
    OnTouch,
}

fn colonel2_run(ctx: &Ctx, mut step: Colonel2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Colonel2Step::Start => {
                step = Colonel2Step::OnTouch;
                continue 'machine;
            }
            Colonel2Step::OnTouch => {
                if ctx.var("aru_vol").get()? == 13 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Colonel Vito#2")])?;
                    ctx.lines_as(
                        "????",
                        args![
                            "You...!",
                            "What are you doing",
                            "just standing around?!",
                            "Aren't you supposed to",
                            "be transporting cargo? ",
                            "Attention to orders!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Are you talking to me?", "No, I'm from the Veins Geo--"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Colonel Vito",
                        args![
                            "Look at you. You don't",
                            "even have your uniform",
                            "yet. Still a rookie, eh?",
                            "Looks like I'll have to",
                            "personally train you as",
                            "one of our holy knights!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["But I'm not--"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Colonel Vito",
                        args![
                            "You should be honored to",
                            "have the rare opportunity",
                            "to be trained by me, the",
                            "great Colonel Vito. I'll mold",
                            "you into a true warrior for",
                            "Freya! Now follow me!"
                        ],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2122), Val::from(2123)])?;
                    ctx.var("aru_vol").set(Val::from(14))?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Colonel Vito#2")])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(156), Val::from(68)])?;
                    return Err(Stop::End);
                } else if ctx.var("aru_vol").get()? == 14 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Colonel Vito#1")])?;
                    ctx.lines_as(
                        "Colonel Vito",
                        args!["What are you still", "doing standing there?", "Don't slack off! Come!"],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Colonel Vito#2")])?;
                    ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(156), Val::from(68)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn colonel2(ctx: &Ctx) -> Script {
    colonel2_run(ctx, Colonel2Step::Start, Vec::new()).map(|_| ())
}

pub fn colonel2_ontouch(ctx: &Ctx) -> Script {
    colonel2_run(ctx, Colonel2Step::OnTouch, Vec::new()).map(|_| ())
}

fn colonel_vito_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn colonel_vito_1(ctx: &Ctx) -> Script {
    colonel_vito_1_body(ctx, Vec::new()).map(|_| ())
}

fn colonel_vito_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Colonel Vito#1")])?;
    return Err(Stop::End);
}

pub fn colonel_vito_1_oninit(ctx: &Ctx) -> Script {
    colonel_vito_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn colonel_vito_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn colonel_vito_2(ctx: &Ctx) -> Script {
    colonel_vito_2_body(ctx, Vec::new()).map(|_| ())
}

fn colonel_vito_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Colonel Vito#2")])?;
    return Err(Stop::End);
}

pub fn colonel_vito_2_oninit(ctx: &Ctx) -> Script {
    colonel_vito_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn colonel_vito_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_answer_s = Val::from("");
    let mut l_input_s = Val::from("");
    if ctx.var("aru_vol").get()? == 14 {
        ctx.lines_as(
            "Colonel Vito",
            args![
                "I am Colonel Vito,",
                "and I'm in charge of",
                "the Arunafeltz camp",
                "here in Thor Volcano.",
                "What's your name, soldier?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", sir."))],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Colonel Vito",
            args![
                "Hmpf! That's a weakling's",
                "name! I can tell that your",
                "mind and body are too weak!",
                "That won't do. How will you",
                "be worthy of serving the",
                "beautiful, graceful Freya?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Colonel Vito",
            args![
                "You need more training.",
                "Take a break first, and",
                "prepare yourself. We will",
                "begin as soon as you're ready."
            ],
        )?;
        ctx.var("aru_vol").set(Val::from(15))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("aru_vol").get()? == 15 {
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "First, we need to take",
                    "care of that weak mind",
                    "of yours. This first",
                    "training exercise will",
                    "be verbal based."
                ],
            )?;
            ctx.next()?;
            'l1: loop {
                if !(true) {
                    break 'l1;
                }
                'b1: {
                    ctx.lines_as(
                        "Colonel Vito",
                        args!["Question one!", "Who do we fight for?", "Who do we live for?"],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Odin:Freya:Thor")])?) == 2 {
                        break 'l1;
                    }
                    ctx.lines_as("Colonel Vito", args!["You idiot!"])?;
                    ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
                    ctx.next()?;
                }
            }
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Right! Freya is not only",
                    "a goddess of love, but she's",
                    "also a goddess of war.",
                    "We are on a sacred mission",
                    "to recover the pieces of",
                    "Ymir's Heart for her sake."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Freya was greatly wounded",
                    "in the war among gods,",
                    "humans, and demons.",
                    "Odin, the leader of the",
                    "gods, tried to help her, but",
                    "even his power wasn't enough."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Odin did advise her to",
                    "obtain Ymir's Heart, as",
                    "it would fully recover her",
                    "powers. That is why we are",
                    "preparing for war: we must",
                    "obtain Ymir's Heart for Freya!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Colonel Vito", args!["DO YOU UNDERSTAND?!"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes, sir!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Now, there's a country",
                    "called the Rune-Midgarts",
                    "Kingdom that's full of fools.",
                    "Their ancestors branded us",
                    "as heretics and drove us",
                    "to this deserted land."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "We cannot forgive how they",
                    "denied us our freedom to",
                    "worship Freya. Our people",
                    "will have revenge on them.",
                    "Mark my word, soldier.",
                    "Now, repeat after me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![((Val::from("I, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(","))],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![((Val::from("I, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(","))],
            )?;
            ctx.next()?;
            'l2: loop {
                if !(true) {
                    break 'l2;
                }
                'b2: {
                    ctx.lines_as("Colonel Vito", args!["^FF0000as a devoted servant", "of Goddess Freya"])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    l_answer_s = Val::from("as a devoted servant of Goddess Freya");
                    if l_input_s.clone().loosely_equals(&l_answer_s.clone()) {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "as a devoted servant",
                                "of Goddess Freya, the",
                                "patron saint of the",
                                "great Arunafeltz,^000000"
                            ],
                        )?;
                        ctx.next()?;
                        break 'l2;
                    } else {
                        ctx.lines_as("Colonel Vito", args!["Wrong! Try again!"])?;
                        ctx.next()?;
                    }
                }
            }
            'l3: loop {
                if !(true) {
                    break 'l3;
                }
                'b3: {
                    ctx.lines_as(
                        "Colonel Vito",
                        args!["^FF0000I pledge my honor to", "overthrow our mortal enemy"],
                    )?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    l_answer_s = Val::from("I pledge my honor to overthrow our mortal enemy");
                    if l_input_s.clone().loosely_equals(&l_answer_s.clone()) {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I pledge my honor to", "overthrow our mortal enemy,", "the Rune-Midgarts Kingdom."],
                        )?;
                        ctx.next()?;
                        break 'l3;
                    } else {
                        ctx.lines_as("Colonel Vito", args!["Wrong! Try again!"])?;
                        ctx.next()?;
                    }
                }
            }
            'l4: loop {
                if !(true) {
                    break 'l4;
                }
                'b4: {
                    ctx.lines_as("Colonel Vito", args!["^FF0000I will show no mercy^000000"])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    l_answer_s = Val::from("I will show no mercy");
                    if l_input_s.clone().loosely_equals(&l_answer_s.clone()) {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["I will show no mercy.", "Nothing will stay my hand.^000000"],
                        )?;
                        ctx.next()?;
                        break 'l4;
                    } else {
                        ctx.lines_as("Colonel Vito", args!["Wrong! Try again!"])?;
                        ctx.next()?;
                    }
                }
            }
            'l5: loop {
                if !(true) {
                    break 'l5;
                }
                'b5: {
                    ctx.lines_as("Colonel Vito", args!["^FF0000I shall devote", "my entire life^000000"])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    l_answer_s = Val::from("I shall devote my entire life");
                    if l_input_s.clone().loosely_equals(&l_answer_s.clone()) {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "I shall devote my",
                                "entire life to the",
                                "full recovery of",
                                "Goddess Freya.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        break 'l5;
                    } else {
                        ctx.lines_as("Colonel Vito", args!["Wrong! Try again!"])?;
                        ctx.next()?;
                    }
                }
            }
            'l6: loop {
                if !(true) {
                    break 'l6;
                }
                'b6: {
                    ctx.lines_as("Colonel Vito", args!["^FF0000Down with the", "Rune-Midgarts Kingdom!^000000"])?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    l_answer_s = Val::from("Down with the Rune-Midgarts Kingdom!");
                    if l_input_s.clone().loosely_equals(&l_answer_s.clone()) {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["Down with the", "Rune-Midgarts Kingdom!^000000"],
                        )?;
                        ctx.next()?;
                        break 'l6;
                    } else {
                        ctx.lines_as("Colonel Vito", args!["Wrong! Try again!"])?;
                        ctx.next()?;
                    }
                }
            }
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Good. Now you know the",
                    "kind of attitude that you",
                    "must have as a holy warrior",
                    "in Freya's service. That",
                    "is all for the first exercise,",
                    "but there's one more left."
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2123), Val::from(2124)])?;
            ctx.var("aru_vol").set(Val::from(16))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_vol").get()? == 16 {
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Before we actually do",
                    "the 2nd training exercise,",
                    "I have a duty to assign to",
                    "you. Find the huge pipe zone",
                    "to the north of this building",
                    "and find any broken machines."
                ],
            )?;
            ctx.var("aru_vol").set(Val::from(17))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_vol").get()? == 17 {
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Inspect the machines",
                    "in the pipe zone to the",
                    "north of this building.",
                    "Don't dawdle: move out!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_vol").get()? == 18 {
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Good work. Remember",
                    "that your first priority is to",
                    "check the control panel.",
                    "It needs to be regularly",
                    "inspected since it controls",
                    "the camp's energy resources."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes, sir!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Now it's time for the",
                    "second training exercise",
                    "which will strengthen your",
                    "body. See the dummy in",
                    "front of you? Practice by",
                    "chopping it 10 times. Go!"
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2125), Val::from(2126)])?;
            ctx.var("aru_vol").set(Val::from(19))?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Yes, sir!:What do you mean by chop?")],
            )?) == 1
            {
                ctx.lines_as(
                    "Colonel Vito",
                    args!["Focus your energy in", "your yell when you strike!", "Chop! 10 Times! Do it!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "Chop...? It's a hand chop.",
                    "You strike the enemy with",
                    "the bottom of your hand",
                    "like a knife blade. How",
                    "did you join the army",
                    "without knowing that?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args!["Focus your energy in", "your yell when you strike!", "Chop! 10 Times! Do it!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("aru_vol").get()? == 20 {
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "The more you train,",
                    "the stronger you become.",
                    "As you grow stronger, so",
                    "does Freya's holy troops.",
                    "Train everyday, and don't",
                    "you ever slack off!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes, sir!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Colonel Vito",
                args![
                    "That is all for your",
                    "training. If you have",
                    "any questions about camp",
                    "life, go ask Sahedi right",
                    "outside this building.",
                    "You are dismissed."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "(^333333Well... I think",
                    "I will go back to",
                    "Sahedi. Hopefully,",
                    "he'll think of me as",
                    "a geological researcher",
                    "instead of as a soldier.^000000)"
                ],
            )?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2126), Val::from(2127)])?;
            ctx.var("aru_vol").set(Val::from(21))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Colonel Vito",
        args![
            "We must be ever",
            "vigilant in our training.",
            "You can never know when",
            "Freya will call on us to fight!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn colonel_vito_3(ctx: &Ctx) -> Script {
    colonel_vito_3_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VolStudy1Step {
    Start,
    OnTouch,
}

fn vol_study1_run(ctx: &Ctx, mut step: VolStudy1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VolStudy1Step::Start => {
                step = VolStudy1Step::OnTouch;
                continue 'machine;
            }
            VolStudy1Step::OnTouch => {
                if ((ctx.var("aru_vol").get()?.number()? > 13 && ctx.var("aru_vol").get()?.number()? < 17)
                    || ctx.var("aru_vol").get()? == 19)
                {
                    ctx.call(Function::Warp, vec![Val::from("thor_camp"), Val::from(156), Val::from(67)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn vol_study1(ctx: &Ctx) -> Script {
    vol_study1_run(ctx, VolStudy1Step::Start, Vec::new()).map(|_| ())
}

pub fn vol_study1_ontouch(ctx: &Ctx) -> Script {
    vol_study1_run(ctx, VolStudy1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SitaVolStep {
    Start,
    OnTouch,
}

fn sita_vol_run(ctx: &Ctx, mut step: SitaVolStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SitaVolStep::Start => {
                step = SitaVolStep::OnTouch;
                continue 'machine;
            }
            SitaVolStep::OnTouch => {
                if ctx.var("aru_vol").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFThere's a stream of",
                        "magma running down",
                        "through a path under",
                        "the barbed wires.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou can hear the faint",
                        "sound of hammering, as if",
                        "iron was being manufactured",
                        "from deep underground.^000000"
                    ])?;
                    ctx.var("aru_vol").set(Val::from(22))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn sita_vol(ctx: &Ctx) -> Script {
    sita_vol_run(ctx, SitaVolStep::Start, Vec::new()).map(|_| ())
}

pub fn sita_vol_ontouch(ctx: &Ctx) -> Script {
    sita_vol_run(ctx, SitaVolStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum BukiVolStep {
    Start,
    OnTouch,
}

fn buki_vol_run(ctx: &Ctx, mut step: BukiVolStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BukiVolStep::Start => {
                step = BukiVolStep::OnTouch;
                continue 'machine;
            }
            BukiVolStep::OnTouch => {
                if ctx.var("aru_vol").get()? == 22 {
                    ctx.lines(args![
                        "^3355FFPeople are carrying",
                        "many heavy wooden boxes",
                        "imprinted with the stamp",
                        "of the Schwarzwald Republic.",
                        "These must contain military",
                        "supplies for the camp.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I should go back", "to Sahedi now."],
                    )?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2127), Val::from(2128)])?;
                    ctx.var("aru_vol").set(Val::from(23))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn buki_vol(ctx: &Ctx) -> Script {
    buki_vol_run(ctx, BukiVolStep::Start, Vec::new()).map(|_| ())
}

pub fn buki_vol_ontouch(ctx: &Ctx) -> Script {
    buki_vol_run(ctx, BukiVolStep::OnTouch, Vec::new()).map(|_| ())
}

fn soldier_vol1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Thor Volcano Camp Soldier",
        args![
            "Freya, I'm so exhausted!",
            "I'm starving to death too!",
            "When will we get more rations?"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_vol1(ctx: &Ctx) -> Script {
    soldier_vol1_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_vol2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Thor Volcano Camp Soldier",
        args![
            "I hear we'll be eating",
            "chicken salad, fried chicken,",
            "and Kunlun style chicken for",
            "dinner tonight! I wonder who",
            "was rich enough to donate",
            "so much chicken to us?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Thor Volcano Camp Soldier", args!["Heheh...", "I can't wait for dinner!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn soldier_vol2(ctx: &Ctx) -> Script {
    soldier_vol2_body(ctx, Vec::new()).map(|_| ())
}

fn control_panel_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 17 {
        ctx.lines(args![
            "^3355FFThis control panel",
            "controls the main power",
            "resources for the Thor",
            "Volcano camp, helping it",
            "run its operations smoothly",
            "under the ground."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "All these pipes with molten",
                "rock flowing through them...",
                "I guess their heat is what",
                "powers this camp. That's",
                "actually a smart idea~"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThe control panel emits",
            "faint mechanical noises",
            "and seems to be running",
            "pretty smoothly. Everything",
            "looks to be in working order."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["No problems here.", "I guess I can go", "back to that colonel."],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2124), Val::from(2125)])?;
        ctx.var("aru_vol").set(Val::from(18))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn control_panel_vol(ctx: &Ctx) -> Script {
    control_panel_vol_body(ctx, Vec::new()).map(|_| ())
}

fn dummy_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_vol").get()? == 19 {
        ctx.lines(args![
            "^3355FFIt's a training dummy",
            "that looks like it can",
            "take a beating. Its chest",
            "is marked with the emblem",
            "of the Rune-Midgarts Kingdom.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Wow...",
                "They really hate the",
                "Rune-Midgarts Kingdom...",
                "(^333333Technically, I shouldn't",
                "be doing this. Isn't this",
                "considered treason?^000000)"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Colonel Vito",
            args![
                "I can't hear you,",
                "soldier! Make your",
                "voice loud and clear!",
                "Now... Execute attack!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Yes, sir!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["One!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Two!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Three!!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Four!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Five!!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Six!!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Seven!!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Eight!!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Nine!"])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Ten!"])?;
        ctx.var("aru_vol").set(Val::from(20))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_vol").get()? == 20 {
        ctx.mes("^3355FF*THUD*^000000")?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT2")?])?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Colonel Vito",
            args!["Soldier...", "How many times", "did I order you to", "chop the dummy?"],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["10 times, sir!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Colonel Vito",
            args!["And how many times", "did you actually", "chop the dummy?"],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["11 times, sir!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Colonel Vito",
            args!["Unacceptable!", "Listen to your orders", "this time, and do it", "again properly!"],
        )?;
        ctx.var("aru_vol").set(Val::from(19))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn dummy_1(ctx: &Ctx) -> Script {
    dummy_1_body(ctx, Vec::new()).map(|_| ())
}

fn aitra_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn aitra_vol(ctx: &Ctx) -> Script {
    aitra_vol_body(ctx, Vec::new()).map(|_| ())
}

fn aitra_vol_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Aitra#vol")])?;
    return Err(Stop::End);
}

pub fn aitra_vol_oninit(ctx: &Ctx) -> Script {
    aitra_vol_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn high_priest_vol_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou'd better not",
        "do anything too",
        "conspicuous in",
        "front of him.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn high_priest_vol(ctx: &Ctx) -> Script {
    high_priest_vol_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou'd better not",
        "do anything too",
        "conspicuous in",
        "front of him.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo1(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo1_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou'd better not",
        "do anything too",
        "conspicuous in",
        "front of him.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo2(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo2_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFYou'd better not",
        "do anything too",
        "conspicuous in",
        "front of him.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo3(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo3_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
    ctx.lines_as(
        "Soldier",
        args![
            "Oh Freya...",
            "Bless this Old Blue Box",
            "so that I don't get anything",
            "lame again. I am so tired",
            "of getting arrows from these..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo4(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo4_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Soldier", args!["Ah, time to", "go to work."])?;
    ctx.next()?;
    ctx.lines_as(
        "Soldier",
        args!["What th-?!", "Something smells", "like sweaty socks that", "haven't been washed!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo5(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo5_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Soldier", args!["Yo-ho! Yo-ho!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo6(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo6_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Soldier",
        args![
            "I guess I need to go",
            "on Guardian polishing",
            "duty. Those things are",
            "so huge, and I gotta",
            "clean up about twenty..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo7(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo7_body(ctx, Vec::new()).map(|_| ())
}

fn guardian_vol_7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^3355FFThere are many guardians",
        "here in different stages",
        "of disassembly. It looks",
        "like they're all in the",
        "middle of being repaired.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guardian_vol_7(ctx: &Ctx) -> Script {
    guardian_vol_7_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo8_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Soldier",
        args![
            "Aren't you the one from",
            "the geological research",
            "center? There's nothing",
            "for you here, we're just",
            "performing maintenance",
            "on these guardians."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo8(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo8_body(ctx, Vec::new()).map(|_| ())
}

fn thor_volcano_soldier_vo9_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Soldier",
        args!["Why am I always", "stationed here?!", "No one ever comes", "here! Nobody!"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thor_volcano_soldier_vo9(ctx: &Ctx) -> Script {
    thor_volcano_soldier_vo9_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TotcampStep {
    Start,
    OnTouch,
}

fn totcamp_run(ctx: &Ctx, mut step: TotcampStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TotcampStep::Start => {
                step = TotcampStep::OnTouch;
                continue 'machine;
            }
            TotcampStep::OnTouch => {
                if ctx.var("rachel_camel").get()?.number()? < 24 {
                    ctx.call(Function::Warp, vec![Val::from("que_thor"), Val::from(65), Val::from(55)])?;
                    return Err(Stop::End);
                }
                ctx.call(Function::Warp, vec![Val::from("que_thor"), Val::from(182), Val::from(55)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn totcamp(ctx: &Ctx) -> Script {
    totcamp_run(ctx, TotcampStep::Start, Vec::new()).map(|_| ())
}

pub fn totcamp_ontouch(ctx: &Ctx) -> Script {
    totcamp_run(ctx, TotcampStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Tov1Step {
    Start,
    OnTouch,
}

fn tov_1_run(ctx: &Ctx, mut step: Tov1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tov1Step::Start => {
                step = Tov1Step::OnTouch;
                continue 'machine;
            }
            Tov1Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("thor_v02"), Val::from(146), Val::from(84)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tov_1(ctx: &Ctx) -> Script {
    tov_1_run(ctx, Tov1Step::Start, Vec::new()).map(|_| ())
}

pub fn tov_1_ontouch(ctx: &Ctx) -> Script {
    tov_1_run(ctx, Tov1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Tov2Step {
    Start,
    OnTouch,
}

fn tov_2_run(ctx: &Ctx, mut step: Tov2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tov2Step::Start => {
                step = Tov2Step::OnTouch;
                continue 'machine;
            }
            Tov2Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("thor_v02"), Val::from(146), Val::from(84)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn tov_2(ctx: &Ctx) -> Script {
    tov_2_run(ctx, Tov2Step::Start, Vec::new()).map(|_| ())
}

pub fn tov_2_ontouch(ctx: &Ctx) -> Script {
    tov_2_run(ctx, Tov2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VolbqStep {
    Start,
    OnTouch,
}

fn volbq_run(ctx: &Ctx, mut step: VolbqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VolbqStep::Start => {
                step = VolbqStep::OnTouch;
                continue 'machine;
            }
            VolbqStep::OnTouch => {
                ctx.lines(args![
                    "^3355FFThere is an old man",
                    "in high priest robes",
                    "in front of you.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou'd better not",
                    "do anything too",
                    "conspicuous in",
                    "front of him.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn volbq(ctx: &Ctx) -> Script {
    volbq_run(ctx, VolbqStep::Start, Vec::new()).map(|_| ())
}

pub fn volbq_ontouch(ctx: &Ctx) -> Script {
    volbq_run(ctx, VolbqStep::OnTouch, Vec::new()).map(|_| ())
}

fn guard_goto_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Guard", args!["What now?", "Can't you see I'm busy?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Let me go out.:I'm sorry.")])?) == 1 {
        ctx.lines_as("Guard", args!["Hurry up, and get out!"])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("que_thor"), Val::from(145), Val::from(60)])?;
        return Err(Stop::End);
    }
    ctx.lines_as("Guard", args!["If you're sorry,", "stop bugging me!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guard_goto(ctx: &Ctx) -> Script {
    guard_goto_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WhoauStep {
    Start,
    OnTouch,
}

fn whoau_run(ctx: &Ctx, mut step: WhoauStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WhoauStep::Start => {
                step = WhoauStep::OnTouch;
                continue 'machine;
            }
            WhoauStep::OnTouch => {
                ctx.lines_as(
                    "House Owner",
                    args!["Wh-who the hell", "are you? Honey!", "There's this...", "person in our home!"],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFHubbie never came.",
                    "Nobody messes with",
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". Nobody.^000000"))
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn whoau(ctx: &Ctx) -> Script {
    whoau_run(ctx, WhoauStep::Start, Vec::new()).map(|_| ())
}

pub fn whoau_ontouch(ctx: &Ctx) -> Script {
    whoau_run(ctx, WhoauStep::OnTouch, Vec::new()).map(|_| ())
}

fn bartender_ve_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Bartender",
        args![
            "Welcome to our tavern.",
            "Please, have a seat.",
            "Let me bring you a",
            "glass of ice water first."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bartender_ve(ctx: &Ctx) -> Script {
    bartender_ve_body(ctx, Vec::new()).map(|_| ())
}

fn female_customer_ve1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Female Customer",
        args![
            "Bartender, aren't any of",
            "your regulars nice young",
            "men? You know, I've been",
            "pretty lonely lately..."
        ],
    )?;
    ctx.next()?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
        ctx.lines_as("Bartender", args!["Haha, well,", "I'm not so sure.."])?;
        ctx.next()?;
        ctx.lines_as("Bartender", args!["How about... me?"])?;
        ctx.next()?;
        ctx.lines_as("Female Customer", args!["Hmpf..."])?;
        ctx.next()?;
        ctx.lines_as("Bartender", args!["Oh, come on!", "I was kidding~"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Bartender",
        args!["Haha, well,", "I'm not so sure..", "What about this nice", "young adventurer here?"],
    )?;
    ctx.next()?;
    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Meee?"])?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_PROFUSELY_SWEAT")?,
            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Female Customer", args!["Mmm...", "Not my style."])?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_THINK")?,
            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn female_customer_ve1(ctx: &Ctx) -> Script {
    female_customer_ve1_body(ctx, Vec::new()).map(|_| ())
}
