use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum WoundedManStep {
    Start,
    OnInit,
}

fn wounded_man_run(ctx: &Ctx, mut step: WoundedManStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WoundedManStep::Start => {
                if ctx.call(Function::CheckWeight, vec![Val::from(7343), Val::from(1)])? != 1 {
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
                if ctx.var("lhz_boss").get()?.number()? < 18 {
                    ctx.lines_as(
                        "?????",
                        args![
                            "^333333*Cough cough*^000000",
                            "Can't hold out...",
                            "Much longer. They...",
                            "They better send",
                            "someone soon..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^3355FFIt's a wounded man...!^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["H-hey...!", "Are you hurt?", "Do you need any he--"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "?????",
                        args![
                            "N-no..!",
                            "Get away, do-gooder!",
                            "Don't attract attention,",
                            "they're gonna find me!",
                            "Don't worry, don't ask,",
                            "j-just get the hell away!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lhz_boss").get()? == 18 {
                    ctx.lines_as(
                        "?????",
                        args![
                            "^333333*Cough cough*^000000",
                            "Can't hold out...",
                            "Much longer. They...",
                            "They better send",
                            "someone soon..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Jargeah?"), Val::from("Ignore him.")])? {
                        1 => {
                            ctx.lines_as(
                                "Jargeah",
                                args![
                                    "^333333*Cough Gasp*^000000",
                                    "H-how do you know my...",
                                    "Never mind that. Who...",
                                    "What... What organization",
                                    "are you working for...?"
                                ],
                            )?;
                            ctx.next()?;
                            let (input, status) = runtime::input_text(ctx, None, None)?;
                            ctx.var("@jargeah$").set(input)?;
                            if (ctx.var("@jargeah$").get()? == "Kafra Corporation" || ctx.var("@jargeah$").get()? == "Secret Wing") {
                                ctx.lines_as(
                                    "Jargeah",
                                    args![
                                        "Th-thank goodness!",
                                        "You finally came for me.",
                                        "If you came a little later,",
                                        "I'd be a goner. H-here,",
                                        "t-take this with you..."
                                    ],
                                )?;
                                ctx.var("lhz_boss").set(Val::from(19))?;
                                ctx.call(Function::GetItem, vec![Val::from(7343), Val::from(1)])?;
                                ctx.call(Function::ChangeQuest, vec![Val::from(12020), Val::from(12021)])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Jargeah",
                                    args![
                                        "Oh... Oh no.",
                                        "*Cough cough*",
                                        "I think... It might",
                                        "T-tell Cilantro that...",
                                        "Tell her I still lov-- ^333333*Huk*^000000"
                                    ],
                                )?;
                                ctx.close_window()?;
                                ctx.call(Function::DisableNpc, vec![Val::from("Wounded Man")])?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Jargeah",
                                args![
                                    "No... No, you're",
                                    "not the one who's",
                                    "supposed to come",
                                    "for m-me... ^333333*Huk*^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFThis isn't good.",
                                "Jargeah just passed",
                                "out in a very ugly way."
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args![
                                "^3355FFAnd so you left the",
                                "wounded man alone. Not",
                                "exactly the best moral choice.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                step = WoundedManStep::OnInit;
                continue 'machine;
            }
            WoundedManStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Wounded Man")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn wounded_man(ctx: &Ctx) -> Script {
    wounded_man_run(ctx, WoundedManStep::Start, Vec::new()).map(|_| ())
}

pub fn wounded_man_oninit(ctx: &Ctx) -> Script {
    wounded_man_run(ctx, WoundedManStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WoundedManSwitchStep {
    Start,
    OnTouch,
}

fn wounded_man_switch_run(ctx: &Ctx, mut step: WoundedManSwitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WoundedManSwitchStep::Start => {
                step = WoundedManSwitchStep::OnTouch;
                continue 'machine;
            }
            WoundedManSwitchStep::OnTouch => {
                if ctx.var("lhz_boss").get()? == 18 {
                    ctx.lines_as(
                        "?????",
                        args!["^333333*Cough cough*^000000", "Everything's getting", "darker. So c-cold..."],
                    )?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Wounded Man")])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn wounded_man_switch(ctx: &Ctx) -> Script {
    wounded_man_switch_run(ctx, WoundedManSwitchStep::Start, Vec::new()).map(|_| ())
}

pub fn wounded_man_switch_ontouch(ctx: &Ctx) -> Script {
    wounded_man_switch_run(ctx, WoundedManSwitchStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EsunaTriggerStep {
    Start,
    OnTouch,
}

fn esuna_trigger_run(ctx: &Ctx, mut step: EsunaTriggerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EsunaTriggerStep::Start => {
                step = EsunaTriggerStep::OnTouch;
                continue 'machine;
            }
            EsunaTriggerStep::OnTouch => {
                if (((ctx.var("lhz_boss").get()? == 26 || ctx.var("lhz_boss").get()? == 36) || ctx.var("lhz_boss").get()? == 38)
                    || ctx.var("lhz_boss").get()? == 40)
                {
                    ctx.lines_as("????", args!["Here,", "Come this way."])?;
                    ctx.close_window()?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Mysterious Woman")])?;
                    ctx.call(Function::Warp, vec![Val::from("lhz_fild01"), Val::from(64), Val::from(223)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn esuna_trigger(ctx: &Ctx) -> Script {
    esuna_trigger_run(ctx, EsunaTriggerStep::Start, Vec::new()).map(|_| ())
}

pub fn esuna_trigger_ontouch(ctx: &Ctx) -> Script {
    esuna_trigger_run(ctx, EsunaTriggerStep::OnTouch, Vec::new()).map(|_| ())
}

fn mysterious_woman_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()? == 26 {
        if ctx.call(Function::CheckWeight, vec![Val::from(7343), Val::from(1)])? != 1 {
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
        ctx.lines_as(
            "Esuna",
            args![
                "I've been waiting for you.",
                "We don't have much time,",
                "so I'll explain quickly. Your",
                "mission is to sneak into the",
                "Rekenber Corporation and",
                "steal incriminating evidence."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Esuna",
            args![
                "However, this place won't",
                "be easy to infiltrate and",
                "there'll be more security",
                "because they learned about",
                "what happened with Shinokas",
                "and... and Jargeah. Jargeah..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Esuna",
            args![
                "You'll need to acquire",
                "identification through one",
                "of our agents who's managed",
                "to get in really deep without",
                "arousing any suspicion."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Esuna",
            args![
                "Afterwards, find the",
                "^FF0000Secret Archive^000000, disable the",
                "security system and steal",
                "that evidence as quickly as",
                "possible. Understood?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Esuna",
            args![
                "For now, use the pass you",
                "have for the Rekenber buildings",
                "and meet up with Agent ^FF0000Lestin^000000.",
                "He'll explain everything else,",
                "so be careful and don't let",
                "them get suspicious of you!"
            ],
        )?;
        ctx.var("lhz_boss").set(Val::from(27))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12024), Val::from(12025)])?;
        ctx.close_window()?;
        ctx.call(Function::DisableNpc, vec![Val::from("Mysterious Woman")])?;
        return Err(Stop::End);
    } else {
        if ctx.var("lhz_boss").get()? == 27 {
            ctx.lines_as(
                "Esuna",
                args![
                    "Use the pass that you",
                    "have to enter the Rekenber",
                    "buildings and find Secret",
                    "Agent Lestin. Remember",
                    "that he's undercover..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("lhz_boss").get()?.number()? < 36 {
                ctx.lines_as("Esuna", args!["........."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("lhz_boss").get()? == 36 {
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "Good work.",
                            "Please bring this file",
                            "to President Weierstrass",
                            "right away. We'll also be",
                            "directly sending information",
                            "as a safeguard measure."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "I know it's sudden, but",
                            "there's a new development.",
                            "I can't explain it now, but you",
                            "have to check on the president",
                            "for me first. Plus, this new",
                            "intel needs to be confirmed..."
                        ],
                    )?;
                    ctx.var("lhz_boss").set(Val::from(37))?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Mysterious Woman")])?;
                    return Err(Stop::End);
                } else if ctx.var("lhz_boss").get()?.number()? < 38 {
                    ctx.lines_as("Esuna", args!["........", "....."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lhz_boss").get()? == 38 {
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "You're back. Listen,",
                            "you could not have helped",
                            "us out at a worse time. We",
                            "just learned that something",
                            "horrible has happened..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "I can't give you all",
                            "the details now, but",
                            "you've got to give this",
                            "file to the president as",
                            "quickly as you can. Hurry!"
                        ],
                    )?;
                    ctx.var("lhz_boss").set(Val::from(39))?;
                    ctx.call(Function::GetItem, vec![Val::from(7343), Val::from(1)])?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Mysterious Woman")])?;
                    return Err(Stop::End);
                } else if ctx.var("lhz_boss").get()? == 39 {
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "What are you waiting",
                            "for?! It's important that",
                            "you give that file to the",
                            "president as soon as",
                            "possible! It's bad news,",
                            "but he deserves to know..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("lhz_boss").get()? == 40 {
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "Good, you're back.",
                            "Listen, all members of",
                            "Secret Wing need to leave",
                            "the Schwarzwald Republic",
                            "immediately! We've been",
                            "severely compromised..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("What happened?")])? {
                        1 => {}
                        _ => {}
                    }
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "We've been tremendously",
                            "damaged. A lot of agents",
                            "died to give us this intel,",
                            "but we've been betrayed.",
                            "The president's closest",
                            "aide totally sold us out..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Who could have done such....?")])? {
                        1 => {}
                        _ => {}
                    }
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "I can't believe Kurelle",
                            "did this to us. And there's no",
                            "way we can save the president.",
                            "Even if we wanted to, we need",
                            "to stick with our contigency",
                            "plan. We all knew the risks..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "Kurelle has been secretly",
                            "meeting with directors from",
                            "Rekenber Corporation and",
                            "his mansion is littered with",
                            "incriminating evidence..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "Damn it! We have no",
                            "choice but to abandon",
                            "the president now! But",
                            "this won't mean that his",
                            "sacrifice, and Jargeah's",
                            "death, will be vain!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Esuna",
                        args![
                            "Still, aside from a few",
                            "agents, almost all of Secret",
                            "Wing must pull out of the",
                            "Schwarzwald Republic for",
                            "now so that we can live to",
                            "fight another day."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Esuna", args!["For now, this is", "goodbye. Take care..."])?;
                    ctx.var("lhz_boss").set(Val::from(41))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(12027), Val::from(12028)])?;
                    ctx.close_window()?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Mysterious Woman")])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Esuna", args!["..........", "......", "...."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn mysterious_woman(ctx: &Ctx) -> Script {
    mysterious_woman_body(ctx, Vec::new()).map(|_| ())
}

fn mysterious_woman_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Mysterious Woman")])?;
    return Err(Stop::End);
}

pub fn mysterious_woman_oninit(ctx: &Ctx) -> Script {
    mysterious_woman_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn researcher_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()?.number()? < 28 {
        ctx.lines_as(
            "Researcher",
            args![
                "You know what's weird?",
                "Why do they use blue and",
                "red wires when they make",
                "bombs? There's so many",
                "others you could use, like",
                "pink or yellow or or green..."
            ],
        )?;
        if ctx.var("lhz_boss").get()? == 27 {
            ctx.next()?;
            ctx.lines_as(
                "Researcher",
                args![
                    "Hey, here's a completely",
                    "hypothetical question. Let's",
                    "say you find a bomb and it's",
                    "about to go off. You better",
                    "cut a wire! So which one are",
                    "you gonna cut? Red or blue?"
                ],
            )?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Red"), Val::from("Blue")])?);
                let mut matched1 = false;
                let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Researcher",
                        args![
                            "Red, eh...?",
                            "...................",
                            "Heh, yeah, just like",
                            "in the movies. I like",
                            "the way you think~"
                        ],
                    )?;
                    if !(ctx.call(Function::CountItem, vec![Val::from(7348)])?.is_true()) {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou surrepticiously",
                        "check the researcher's",
                        "ID badge and see that the name ''Lestin'' is written on it.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Lestin", args!["So...", "Is there anything", "I can help you with?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("No"), Val::from("Yes")])? {
                        1 => {
                            ctx.lines_as(
                                "Lestin",
                                args![
                                    "Alright then.",
                                    "Just be quiet when",
                                    "you're in the Laboratory.",
                                    "The people here work",
                                    "pretty feverishly and get",
                                    "irritated pretty easily, okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Lestin",
                                args![
                                    "So what exactly did",
                                    "you need? I'm just an",
                                    "ordinary researcher,",
                                    "so I don't know how",
                                    "much help I could be..."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Show Secret Wing Card.")])? {
                                1 => {}
                                _ => {}
                            }
                            ctx.lines_as(
                                "Lestin",
                                args![
                                    "Whoa, whoa~!",
                                    "Careful where you",
                                    "flash that! Okay, I know",
                                    "who you are. But we better",
                                    "continue this someplace",
                                    "a bit more private..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(283), Val::from(166)])?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Researcher",
                        args![
                            "Blue, huh? Yeah,",
                            "that's what everyone",
                            "else here picks. Now",
                            "what color would I pick?",
                            "Well, that's a secret~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Lestin",
            args![
                "Geez, I've been so",
                "tired lately. Work?",
                "Forget it, I'm gonna",
                "just kick back today~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn researcher_1(ctx: &Ctx) -> Script {
    researcher_1_body(ctx, Vec::new()).map(|_| ())
}

fn researcher_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7349), Val::from(1)])? != 1 {
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
    if ctx.var("lhz_boss").get()?.number()? < 27 {
        ctx.lines_as(
            "Researcher",
            args!["This is a restricted", "area. Please leave", "immediately."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 27 {
        ctx.lines_as(
            "Lestin",
            args![
                "This place should be",
                "safe enough for us to talk",
                "for now, so please listen",
                "carefully. Esuna must have",
                "told you that they beefed",
                "up security lately, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "Anyway, since my location",
                "is being monitored, I can't",
                "risk going inside there. You",
                "need to sneak in on your own",
                "when the guards change shifts. It's an old trick, but it works."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "My pass will get you inside,",
                "but you'll only be able to",
                "stick around for 3 minutes",
                "at a time. The Secret Archive is to the right of this laboratory."
            ],
        )?;
        ctx.var("lhz_boss").set(Val::from(28))?;
        ctx.call(Function::GetItem, vec![Val::from(7349), Val::from(1)])?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12025), Val::from(12026)])?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "When you're finished,",
                "come back and give me",
                "the pass since it'll be real",
                "suspicious if I don't have it.",
                "Good luck to you. This job",
                "is risky, but not impossible."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()?.number()? < 35 {
        ctx.lines_as(
            "Lestin",
            args![
                "Remember, you gotta",
                "sneak past that set of",
                "two guards while they're",
                "changing shifts. If you hide",
                "behind a corner and wait for",
                "a bit, you should get lucky."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "Once you sneak into the",
                "Secret Archive, look for",
                "the File Search Engine that",
                "should be right next to the",
                "door. You gotta use to find",
                "specific information, okay?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "My suggestion? You really",
                "ought to look for any files",
                "containing any information",
                "regarding ^3355FFRekenber's secret",
                "that Shinokas discovered^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "Remember that this pass",
                "will get you into the Secret",
                "Archive for only 3 minutes",
                "at a time. Be really careful",
                "and don't get caught!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 35 {
        if (!(ctx.call(Function::CountItem, vec![Val::from(7349)])?.is_true())
            || !(ctx.call(Function::CountItem, vec![Val::from(7344)])?.is_true()))
        {
            ctx.lines_as(
                "Lestin",
                args![
                    "My pass will get you inside,",
                    "but you'll only be able to",
                    "stick around for 3 minutes",
                    "at a time. The Secret Archive is to the right of this laboratory."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lestin",
                args![
                    "When you're finished,",
                    "come back and give me",
                    "the pass since it'll be real",
                    "suspicious if I don't have it.",
                    "Good luck to you. This job",
                    "is risky, but not impossible."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Lestin",
            args![
                "You found what you were",
                "looking for? That must have",
                "been like looking for a needle",
                "in a haystick, but you managed",
                "to do it. Great work, guy~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "It's a good thing you",
                "found that when you did.",
                "Although it'd help to steal",
                "even more intel, sticking",
                "around even longer makes",
                "it easier for us to get caught."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "Alright, you better get",
                "out of here and find Esuna",
                "now. Watch your back and",
                "be careful. We're not all",
                "clear until this is all over."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lestin",
            args![
                "Esuna should be right",
                "outside of the city of",
                "Lighthalzen where you",
                "found her last time. She'll",
                "know that you're coming."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(7349), Val::from(1)])?;
        ctx.var("lhz_boss").set(Val::from(36))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12026), Val::from(12027)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Lestin",
            args![
                "Oh man...",
                "Everyone here is",
                "getting too paranoid",
                "for their own good!",
                "You better steer clear",
                "from this place for now."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn researcher_2(ctx: &Ctx) -> Script {
    researcher_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SneakStep {
    Start,
    OnTouch,
}

fn sneak_run(ctx: &Ctx, mut step: SneakStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SneakStep::Start => {
                return Err(Stop::End);
            }
            SneakStep::OnTouch => {
                if (ctx.var("lhz_boss").get()?.number()? > 27 && ctx.var("lhz_boss").get()?.number()? < 35) {
                    ctx.var("@sneaktime")
                        .set((ctx.call(Function::GetTimeTick, vec![Val::from(0)])?.try_rem(Val::from(100))?))?;
                    if ((ctx.var("@sneaktime").get()?.number()? > 10 && ctx.var("@sneaktime").get()?.number()? < 59)
                        || (ctx.var("@sneaktime").get()?.number()? < -10 && ctx.var("@sneaktime").get()?.number()? > -59))
                    {
                        ctx.lines(args![
                            "^3355FFAs you approach",
                            "the corner, you can",
                            "hear hushed whispers",
                            "just over the wall.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Senior Guard",
                            args!["Hey, my shift is over.", "Hurry and get the next", "guy to relieve me, will you?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Rookie Guard",
                            args!["Already?", "Wow, time sure", "flies fast. Fine,", "wait here a bit."],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFOne of the guards left",
                            "his post, and now there",
                            "is only one remaining",
                            "guard monitoring this area.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Senior Guard",
                            args![
                                "Criminy...",
                                "I need to go to",
                                "the bathroom. Well,",
                                "I'm sure nothing will",
                                "happen while I'm gone."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou listen to the",
                            "guard's footsteps as",
                            "they grow fainter and",
                            "fainter into the distance.^000000"
                        ])?;
                        ctx.next()?;
                        'b1: {
                            let subject1 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Sneak in now."), Val::from("Wait for another chance.")],
                            )?);
                            let mut matched1 = false;
                            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines(args![
                                    "^3355FFThis is the perfect",
                                    "opportunity to infiltrate",
                                    "the Secret Archive! You",
                                    "approach the door and",
                                    "find a device where you",
                                    "can insert Lestin's card pass.^000000"
                                ])?;
                                ctx.next()?;
                                'b2: {
                                    let subject2 =
                                        Val::from(runtime::select_values(ctx, &[Val::from("Insert Card"), Val::from("Retreat")])?);
                                    let mut matched2 = false;
                                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                        matched2 = true;
                                    }
                                    if matched2 {
                                        if !(ctx.call(Function::CountItem, vec![Val::from(7349)])?.is_true()) {
                                            ctx.lines(args![
                                                "^3355FFYou forgot to bring",
                                                "the card pass that",
                                                "you got from Lestin.",
                                                "You need it in order",
                                                "to open this door.^000000"
                                            ])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines(args![
                                            "^3355FFAfter inserting the",
                                            "pass, a panel within",
                                            "the door slides open,",
                                            "revealing a numeric keypad.",
                                            "You need to input the correct",
                                            "password to open the door.^000000"
                                        ])?;
                                        ctx.next()?;
                                        'j3: loop {
                                            let (input, status) = runtime::input_number(ctx, None, None)?;
                                            ctx.var("@sneakpass").set(input)?;
                                            if ctx.var("@sneakpass").get()? == 738495 {
                                                ctx.lines(args![
                                                    "^3355FF*Beep~*",
                                                    "You hear a pleasant",
                                                    "sounding electronic chirp,",
                                                    "signaling that you have input",
                                                    "the correct password. The door",
                                                    "automatically slides open."
                                                ])?;
                                                ctx.next()?;
                                                match runtime::select_values(ctx, &[Val::from("Enter"), Val::from("Retreat")])? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            "Security System",
                                                            args![
                                                                "You have 3 minutes to",
                                                                "search the Information Archive.",
                                                                "When this time elapses, you",
                                                                "will be automatically sent",
                                                                "outside for security reasons."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        ctx.var("lhz_boss").set(Val::from(29))?;
                                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Timer_Sneak::OnEnter")])?;
                                                        ctx.call(
                                                            Function::Warp,
                                                            vec![Val::from("lhz_in01"), Val::from(177), Val::from(35)],
                                                        )?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines(args![
                                                            "^3355FFPerhaps now would",
                                                            "not be the best time to",
                                                            "enter the Secret Archive.",
                                                            "Or at least, that's what",
                                                            "you've decided for yourself.^000000"
                                                        ])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            ctx.lines(args![
                                                "^3355FF*Eeeeeee*",
                                                "The door emits an",
                                                "unnerving, high pitched",
                                                "screech after you input",
                                                "the password. You really",
                                                "should try to input it again.^000000"
                                            ])?;
                                            ctx.var("@sneakerror").set((ctx.var("@sneakerror").get()? + Val::from(1)))?;
                                            ctx.next()?;
                                            if ctx.var("@sneakerror").get()?.number()? > 2 {
                                                ctx.lines_as(
                                                    "Security System",
                                                    args![
                                                        "*Gzzzzz*",
                                                        "You have entered the",
                                                        "password incorrectly",
                                                        "3 times. Please stand by",
                                                        "for managerial assistance."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFUh oh!",
                                                    "You better get",
                                                    "out of here before",
                                                    "you get caught!^000000"
                                                ])?;
                                                ctx.close_window()?;
                                                ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(191), Val::from(49)])?;
                                                return Err(Stop::End);
                                            }
                                            continue 'j3;
                                            break 'j3;
                                        }
                                    }
                                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                        matched2 = true;
                                    }
                                    if matched2 {
                                        ctx.lines(args![
                                            "^3355FFPerhaps now would",
                                            "not be the best time to",
                                            "enter the Secret Archive.",
                                            "Or at least, that's what",
                                            "you've decided for yourself.^000000"
                                        ])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                                matched1 = true;
                            }
                            if matched1 {
                                ctx.lines(args![
                                    "^3355FFPerhaps now would",
                                    "not be the best time to",
                                    "enter the Secret Archive.",
                                    "Or at least, that's what",
                                    "you've decided for yourself.^000000"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Door#sneak::OnSneak")])?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn sneak(ctx: &Ctx) -> Script {
    sneak_run(ctx, SneakStep::Start, Vec::new()).map(|_| ())
}

pub fn sneak_ontouch(ctx: &Ctx) -> Script {
    sneak_run(ctx, SneakStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn timer_sneak(ctx: &Ctx) -> Script {
    timer_sneak_run(ctx, TimerSneakStep::Start, Vec::new()).map(|_| ())
}

pub fn timer_sneak_ontouch(ctx: &Ctx) -> Script {
    timer_sneak_run(ctx, TimerSneakStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn timer_sneak_oninit(ctx: &Ctx) -> Script {
    timer_sneak_run(ctx, TimerSneakStep::OnInit, Vec::new()).map(|_| ())
}

pub fn timer_sneak_onenter(ctx: &Ctx) -> Script {
    timer_sneak_run(ctx, TimerSneakStep::OnEnter, Vec::new()).map(|_| ())
}

pub fn timer_sneak_ontimer180000(ctx: &Ctx) -> Script {
    timer_sneak_run(ctx, TimerSneakStep::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn timer_sneak_ontimer190000(ctx: &Ctx) -> Script {
    timer_sneak_run(ctx, TimerSneakStep::OnTimer190000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum FileSearchEngineStep {
    Start,
    LSearch,
    HoistEnd1,
}

fn file_search_engine_run(ctx: &Ctx, mut step: FileSearchEngineStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FileSearchEngineStep::Start => {
                if (ctx.var("lhz_boss").get()?.number()? > 28 && ctx.var("lhz_boss").get()?.number()? < 35) {
                    ctx.lines(args![
                        "^3355FFThis machine can be",
                        "used to locate specific",
                        "documents within the",
                        "Secret Archive. However,",
                        "you must enter the correct",
                        "keywords to in order to find",
                        "specific file locations."
                    ])?;
                    ctx.next()?;
                    step = FileSearchEngineStep::LSearch;
                    continue 'machine;
                }
                step = FileSearchEngineStep::HoistEnd1;
                continue 'machine;
            }
            FileSearchEngineStep::LSearch => {
                match runtime::select_values(ctx, &[Val::from("Search Engine."), Val::from("Cancel.")])? {
                    1 => {
                        ctx.lines(args![
                            "^663300- Search Engine Initiated -",
                            "- Please enter a keyword -",
                            " ",
                            "*Search Engine",
                            "is case sensitve.",
                            "Please do not use",
                            "capital letters.^000000"
                        ])?;
                        ctx.next()?;
                        let (input, status) = runtime::input_text(ctx, None, None)?;
                        ctx.var("@sneaksearch$").set(input)?;
                        if (ctx.var("@sneaksearch$").get()? == "kafra" || ctx.var("@sneaksearch$").get()? == "cool event") {
                            ctx.lines(args![
                                "^663300[Search Result]",
                                "Documents regarding",
                                "Kafra Corporation and",
                                "Cool Event Corp are",
                                "located in Arena 3-2.^000000"
                            ])?;
                            ctx.var("lhz_boss").set(Val::from(30))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("@sneaksearch$").get()? == "shinokas" {
                                ctx.lines(args![
                                    "^663300[Search Result]",
                                    "Documents regarding",
                                    "Shinokas are located",
                                    "in Arena 1-5.^000000"
                                ])?;
                                ctx.var("lhz_boss").set(Val::from(31))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ((ctx.var("@sneaksearch$").get()? == "stein" || ctx.var("@sneaksearch$").get()? == "STEIN")
                                    || ctx.var("@sneaksearch$").get()? == "S.T.E.I.N")
                                {
                                    ctx.lines(args![
                                        "^663300[Search Result]",
                                        "Documents regarding",
                                        "S.T.E.I.N are considered",
                                        "highly classified and",
                                        "cannot be accessed",
                                        "through this system.^000000"
                                    ])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("@sneaksearch$").get()? == "ymir" {
                                        ctx.lines(args![
                                            "^663300[Search Result]",
                                            "Documents regarding",
                                            "Ymir's Heart are ranked",
                                            "as highly classified and",
                                            "cannot be accessed",
                                            "through this system.^000000"
                                        ])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ((ctx.var("@sneaksearch$").get()? == "president"
                                        || ctx.var("@sneaksearch$").get()? == "karl")
                                        || ctx.var("@sneaksearch$").get()? == "weierstrass")
                                    {
                                        ctx.lines(args![
                                            "^663300[Search Result]",
                                            "2nd Class documents on",
                                            "President Karl Weierstrass",
                                            "are located in Area 1-7. For",
                                            "more highly classified files",
                                            "on Weierstrass, please use",
                                            "a higher security archive.^000000"
                                        ])?;
                                        ctx.var("lhz_boss").set(Val::from(32))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("@sneaksearch$").get()? == "einbroch" {
                                        ctx.lines(args![
                                            "^663300[Search Result]",
                                            "Documents regarding",
                                            "Einbroch are stored",
                                            "in Area 6-1.^000000"
                                        ])?;
                                        ctx.var("lhz_boss").set(Val::from(33))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("@sneaksearch$").get()? == "lighthalzen" {
                                        ctx.lines(args![
                                            "^663300[Search Result]",
                                            "Documents regarding",
                                            "Lighthalzen are stored",
                                            "in Area 3-3.^000000"
                                        ])?;
                                        ctx.var("lhz_boss").set(Val::from(34))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("@sneaksearch$").get()? == "rekenber" {
                                        ctx.lines(args![
                                            "^663300[Search Result]",
                                            "Documents regarding",
                                            "Rekenber are highly classified and cannot be accessed by this system.^000000"
                                        ])?;
                                        ctx.next()?;
                                        step = FileSearchEngineStep::LSearch;
                                        continue 'machine;
                                    } else {
                                        ctx.lines(args![
                                            "^663300[Search Result]",
                                            "Keyword not found.",
                                            "Please search another",
                                            "archive or increase",
                                            "access permissions.^000000"
                                        ])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                    2 => {
                        ctx.lines(args![
                            "^663300[Search Engine Close]",
                            "File search has been",
                            "canceled. Please be aware",
                            "that sudden shutdown may",
                            "cause system errors.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = FileSearchEngineStep::HoistEnd1;
                continue 'machine;
            }
            FileSearchEngineStep::HoistEnd1 => {
                ctx.lines(args![
                    "^3355FFThis machine can be",
                    "used to locate specific",
                    "documents within the",
                    "Secret Archive, However,",
                    "you no longer need to",
                    "search through the files.^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn file_search_engine(ctx: &Ctx) -> Script {
    file_search_engine_run(ctx, FileSearchEngineStep::Start, Vec::new()).map(|_| ())
}

fn door_sneak_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()?.number()? < 29 {
        ctx.lines_as(
            "Guard",
            args![
                "This is a",
                "restricted area.",
                "Please keep clear",
                "if you do not have",
                "special authorization.",
                "Thank you for cooperating."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()?.number()? < 36 {
        ctx.lines(args![
            "^3355FFThe door is shut, but",
            "there is a device that",
            "looks sort of like the",
            "entry keypad that was on",
            "the other side of this door.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Manipulate device."), Val::from("Investigate further.")])? {
            1 => {
                ctx.lines(args![
                    "^3355FFOnce you touch the",
                    "device, it automatically",
                    "responds and the door",
                    "quickly slides open.^000000"
                ])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(177), Val::from(26)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn door_sneak(ctx: &Ctx) -> Script {
    door_sneak_body(ctx, Vec::new()).map(|_| ())
}

fn area_1_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7344), Val::from(1)])? != 1 {
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
    if (ctx.var("lhz_boss").get()?.number()? > 28 && ctx.var("lhz_boss").get()?.number()? < 31) {
        ctx.lines(args![
            "^3355FFThere's literally thousands",
            "of documents to sort through.",
            "There's no way you can find",
            "something of value here in",
            "just three minutes...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 31 {
        ctx.lines(args![
            "^3355FFWait, one of these files",
            "looks pretty incriminating.",
            "It seems to contain the",
            "kind of information that",
            "you've been looking for.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717A piece of Ymir's Heart",
                "was uncovered in one of",
                "the mines in Einbech and",
                "immediately transported to",
                "the Laboratory for research.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717However, something",
                "happened to the miners",
                "who discovered the piece",
                "of Ymir's Heart. Apparently,",
                "a beast from Einbroch folk",
                "lore inhabited the area...^000000"
            ],
        )?;
        ctx.var("lhz_boss").set(Val::from(35))?;
        ctx.call(Function::GetItem, vec![Val::from(7344), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn area_1_5(ctx: &Ctx) -> Script {
    area_1_5_body(ctx, Vec::new()).map(|_| ())
}

fn area_1_7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lhz_boss").get()?.number()? > 28 && ctx.var("lhz_boss").get()?.number()? < 32) {
        ctx.lines(args![
            "^3355FFThere's literally thousands",
            "of documents to sort through.",
            "There's no way you can find",
            "something of value here in",
            "just three minutes...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 32 {
        ctx.lines(args![
            "^3355FFThis looks like a file",
            "containing information",
            "on President Weierstrauss.",
            "Perhaps there's something",
            "in here that might be helpful.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717Karl Weierstrass has",
                "been a long distinguished",
                "politician in the Schwarzwald",
                "Republic and was elected as",
                "its president in the year 984.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717Although he has enjoyed",
                "high popularity ratings and",
                "success in his endorsing his",
                "policies, Wierstrass is rumored",
                "to be have some sort of feud",
                "against Rekenber.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717Although there is no",
                "need to bring this to the",
                "attention of the media, it",
                "is highly recommended",
                "to monitor his activities",
                "throughout his term...^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThere's a great amount",
            "of information in this file,",
            "but it doesn't look like any",
            "of it will be of help to you.",
            "You should continue with",
            "your search for evidence.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThis file doesn't",
            "look like it contains",
            "any evidence that will",
            "help you. You should",
            "continue your search",
            "through the rest of the files.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn area_1_7(ctx: &Ctx) -> Script {
    area_1_7_body(ctx, Vec::new()).map(|_| ())
}

fn area_3_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lhz_boss").get()?.number()? > 28 && ctx.var("lhz_boss").get()?.number()? < 30) {
        ctx.lines(args![
            "^3355FFThere's literally thousands",
            "of documents to sort through.",
            "There's no way you can find",
            "something of value here in",
            "just three minutes...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 30 {
        ctx.lines(args![
            "^3355FFYou've found a file",
            "containing information",
            "on the Kafra Corporation",
            "and Cool Event Corp.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717Rekenber Corporation.",
                "in an effort to expand its",
                "power into the Rune-Midgarts",
                "Kingdom, will cooperate with",
                "and support Cool Event Corp.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717Although this partnership",
                "has been greatly successful,",
                "Kafra Corporation has been",
                "working to check Cool Event",
                "Corp's explosive growth.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717More than Kafra Corporation's",
                "lobbying, Kafra's 3rd Security",
                "Team stands as a formidable",
                "threat to our success. One of",
                "our highest priorities is to",
                "investigate their activities.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717It is believed that Kafra's",
                "3rd Security Team has alrady",
                "placed secret agents in key",
                "strategic locations within",
                "the Schwarzwald Republic.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThis file is very intriguing,",
            "but it doesn't really cover",
            "any information that can be",
            "considered incriminating.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThis file doesn't",
            "look like it contains",
            "any evidence that will",
            "help you. You should",
            "continue your search",
            "through the rest of the files.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn area_3_2(ctx: &Ctx) -> Script {
    area_3_2_body(ctx, Vec::new()).map(|_| ())
}

fn area_3_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lhz_boss").get()?.number()? > 28 && ctx.var("lhz_boss").get()?.number()? < 34) {
        ctx.lines(args![
            "^3355FFThere's literally thousands",
            "of documents to sort through.",
            "There's no way you can find",
            "something of value here in",
            "just three minutes...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 34 {
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717...After negotiating with",
                "the existing residents, all of",
                "the property rights were sold",
                "and the city was renamed",
                "''Lighthalzen'' in 865.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717The city was then separated",
                "into three districts. These are",
                "the common trade district, the",
                "old residential district and",
                "the Rekenber Headquarters.",
                ".................^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFThis file covers",
            "Lighthalzen's history.",
            "However, the founding of",
            "the city is common knowlege,",
            "so this document probably won't",
            "have any significant evidence.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThis file doesn't",
            "look like it contains",
            "any evidence that will",
            "help you. You should",
            "continue your search",
            "through the rest of the files.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn area_3_3(ctx: &Ctx) -> Script {
    area_3_3_body(ctx, Vec::new()).map(|_| ())
}

fn area_6_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("lhz_boss").get()?.number()? > 28 && ctx.var("lhz_boss").get()?.number()? < 33) {
        ctx.lines(args![
            "^3355FFThere's literally thousands",
            "of documents to sort through.",
            "There's no way you can find",
            "something of value here in",
            "just three minutes...^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 33 {
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717........",
                "Our organization purchased",
                "all of western Einbech and",
                "began construction of Einbroch",
                "in 927 in order to obtain more",
                "pieces of Ymir's Heart.^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Classified Info",
            args![
                "^8C1717Unofficially, each and",
                "every single factory belongs",
                "to the Rekenber Corporation.",
                "Their highest priority is",
                "to uncover Ymir Heart Pieces...^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFAlthough this file contains",
            "surprising information about",
            "Einbroch, none of it can be",
            "considered to be incriminating",
            "evidence against Rekenber."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines(args![
            "^3355FFThis file doesn't",
            "look like it contains",
            "any evidence that will",
            "help you. You should",
            "continue your search",
            "through the rest of the files.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn area_6_1(ctx: &Ctx) -> Script {
    area_6_1_body(ctx, Vec::new()).map(|_| ())
}

fn broken_machine_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(7346), Val::from(1)])? != 1 {
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
    if ctx.var("lhz_boss").get()?.number()? < 6 {
        ctx.lines(args![
            "^3355FFThere are several",
            "broken machines lying",
            "around that pretty much",
            "seem completely useless.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 6 {
        ctx.lines(args![
            "^3355FFThere are several",
            "broken machines lying",
            "around that pretty much",
            "seem completely useless.",
            "However, you catch the",
            "glimmer of a dim light",
            "amongst the scrap metal.^000000"
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Investigate it."), Val::from("Ignore it.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "^3355FFAfter digging through",
                    "the discarded machinery,",
                    "you find a strange rock that",
                    "is about as large as your fist",
                    "and shimmers with a faint glow.^000000"
                ])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("This might be important."), Val::from("This doesn't seem useful.")],
                )? {
                    1 => {
                        ctx.lines(args![
                            "^3355FFThis weird rock",
                            "might be just the",
                            "thing that ^000000Ghalstein^3355FF",
                            "sent you here to find.",
                            "He was probably right",
                            "not to describe it to you.",
                            "Words alone aren't enough...^000000"
                        ])?;
                        ctx.var("lhz_boss").set(Val::from(7))?;
                        ctx.call(Function::GetItem, vec![Val::from(7346), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args![
                            "^3355FFJust because this rock",
                            "looks funny doesn't make",
                            "it any more special than",
                            "the countless number of",
                            "rocks you've seen in",
                            "your entire lifetime.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines(args![
                    "^3355FFWhat can possibly",
                    "be of value in this",
                    "pile of useless junk?^000000"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines(args![
            "^3355FFThere are several",
            "broken machines lying",
            "around that pretty much",
            "seem completely useless.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn broken_machine(ctx: &Ctx) -> Script {
    broken_machine_body(ctx, Vec::new()).map(|_| ())
}

fn boss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Patch", args!["Tell me what you want."])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Current Variables:How Many:Shinokas - Quest Complete")])? {
        1 => {}
        2 => {
            let (input, status) = runtime::input_number(ctx, Some(0), Some(1000))?;
            l_input = input;
            ctx.var("lght_boss").set(l_input.clone())?;
        }
        3 => {
            ctx.var("shinokas_quest").set(Val::from(11))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    ctx.lines(args![" ", (Val::from("") + ctx.var("lght_boss").get()?)])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn boss(ctx: &Ctx) -> Script {
    boss_body(ctx, Vec::new()).map(|_| ())
}

fn maintenance_guy_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kudiuu",
        args![
            "Holy...!",
            "Will this place",
            "ever get cleaned up?!",
            "^333333*Cough cough*^000000 There's",
            "so much dust here, it's",
            "almost a health hazard!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn maintenance_guy(ctx: &Ctx) -> Script {
    maintenance_guy_body(ctx, Vec::new()).map(|_| ())
}
