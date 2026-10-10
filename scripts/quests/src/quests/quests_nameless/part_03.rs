use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn female_follower_em_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("aru_em").get()?.number()? < 9 {
        ctx.lines_as(
            "Sappie",
            args![
                "I work really hard to keep",
                "this room clean and cozy so",
                "that the High Priestess can",
                "rest and refresh herself in",
                "comfort. Take a deep breath:",
                "smell that relaxing aroma?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()? == 9 {
        ctx.lines_as(
            "Sappie",
            args![
                "I work really hard to keep",
                "this room clean and cozy so",
                "that the High Priestess can",
                "rest and refresh herself in",
                "comfort. Take a deep breath:",
                "smell that relaxing aroma?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Oh, you're right, that",
                "is a nice smell! So, is",
                "Priestess Niren away",
                "from her office today?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sappie",
            args![
                "Oh, yes, she's out since",
                "there are many followers",
                "that wish to meet her,",
                "even early in the morning."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Do you know where", "I could find her?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sappie",
            args![
                "Hm... Sippie mentioned",
                "something... Ah, right. Sippie",
                "said that High Priestess Niren",
                "decided to go to Cheshrumnir",
                "Garden. That garden is very",
                "large, and quite beautiful."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["The High Priestess", "must be very busy."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sappie",
            args![
                "Oh, you can't even imagine!",
                "She's been passing her wisdom",
                "to the people ever since she",
                "was a baby, and crowds of",
                "people still clamor for",
                "her sage teachings."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sappie",
            args![
                "If you really want",
                "to talk to her, it'd",
                "be best if you catch her",
                "before she's surrounded",
                "by Freya's followers."
            ],
        )?;
        ctx.var("aru_em").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()? == 10 {
        ctx.lines_as(
            "Sappie",
            args![
                "High Priestess Niren must",
                "be in Cheshrumnir Garden.",
                "You should try to talk to her",
                "before the crowds show up."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()?.number()? > 21 {
        ctx.lines_as(
            "Sappie",
            args![
                "It's rumored that the",
                "Goddess Freya spoke to",
                "our pope, giving her an",
                "important message. Now",
                "everyone wants to hear",
                "what Freya told her..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Sappie",
            args![
                "I work really hard to keep",
                "this room clean and cozy so",
                "that the High Priestess can",
                "rest and refresh herself in",
                "comfort. Take a deep breath:",
                "smell that relaxing aroma?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn female_follower_em(ctx: &Ctx) -> Script {
    female_follower_em_body(ctx, Vec::new()).map(|_| ())
}

fn ishmael_em_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
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
    if ctx.var("aru_em").get()?.number()? < 17 {
        ctx.lines_as(
            "Ishmael",
            args![
                "I-I don't know you, do I?",
                "Sorry, but would you, um,",
                "just step away? I... I don't",
                "like being too close to",
                "other people. Please!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("aru_em").get()? == 17 {
        ctx.lines_as(
            "Ishmael",
            args![
                "I-I don't know you, do I?",
                "Sorry, but would you, um,",
                "just step away? Wait, you",
                "don't have something you",
                "need from me, do you?"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Give Her Niren's File:Leave Her Alone")])? {
            1 => {
                if ctx.call(Function::CountItem, vec![Val::from(7343)])?.number()? > 0 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["You know High Priestess", "Niren, right? This is from her."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "Oh, really? Okay, just...",
                            "Just toss the file over",
                            "to me. I-I don't want",
                            "you coming any closer."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "I'll be glad to help out",
                            "the priestess. Even though",
                            "we're natives, she's never",
                            "discriminated against us.",
                            "Hmm... What does she want?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "Let's see... She wants",
                            "me to forge a copy of this",
                            "file? Piece of cake, I'm an",
                            "expert at forging writing!",
                            "Just give me a moment and--"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "Dang it! I completely forgot!",
                            "Someone stole my pen yesterday!",
                            "Now how am I gonna do this...?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "My pen was made of a very",
                            "rare gemstone called ^FF0000Sardonyx^000000.",
                            "*Sob* I don't think anyone",
                            "sells Sardonyx in Arunafeltz.",
                            "You can only get that in the",
                            "countries next to us..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "I really want to help",
                            "High Priestess Niren...",
                            "I can't let her down after",
                            "she's been so good to us..."
                        ],
                    )?;
                    ctx.var("aru_em").set(Val::from(18))?;
                    ctx.call(Function::DelItem, vec![Val::from(7343), Val::from(1)])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2137), Val::from(2138)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args!["^3355FFYou seem to have", "misplaced Niren's file...^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Ishmael",
                    args!["I... I don't see you", "backing away. Stay back,", "and don't you dare touch me!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("aru_em").get()? == 18 {
        if ctx.call(Function::CountItem, vec![Val::from(725)])?.number()? > 0 {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "Would you be able to forge",
                    "a copy of High Priestess",
                    "Niren's file if you had",
                    "this gemstone?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Ishmael", args!["Oh, you found", "a Sardonyx for me?", "It looks perfect!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Ishmael",
                args![
                    "Er, but, um...",
                    "Would you please",
                    "not come any closer?",
                    "I still... Uh... I don't mean",
                    "to be rude... It's just...",
                    "I'm just not good with..."
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.next()?;
            ctx.lines_as(
                "Ishmael",
                args![
                    "If you just toss the",
                    "Sardonyx over here,",
                    "I'll be able to forge",
                    "your copy right away."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Don't Give It to Her:Give It to Her")])? {
                1 => {
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "Huh? I thought you",
                            "needed my help? I can't",
                            "do anything unless you",
                            "give me that gemstone."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Ishmael", args!["Yay, thank you so much!", "Now I can get to work~"])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FF*Scribble*", "*Scribble*^000000"])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COMBOATTACK2")?])?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FF*Scribble*", "*Scribble*^000000"])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COMBOATTACK4")?])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FF*Scribble*",
                        "*Scribble*^000000",
                        "^3355FF*Scribble*",
                        "*Scribble*^000000"
                    ])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COMBOATTACK4")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ishmael",
                        args![
                            "There you go! I made the",
                            "best quality copy I could for",
                            "you since High Priestess",
                            "Niren requested for it.",
                            "Would you please send",
                            "her my regards? Heh heh~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou received a file",
                        "containing a forged",
                        "written approval for",
                        "vacation from Ishmael.^000000"
                    ])?;
                    ctx.call(Function::DelItem, vec![Val::from(725), Val::from(1)])?;
                    ctx.var("aru_em").set(Val::from(19))?;
                    ctx.call(Function::GetItem, vec![Val::from(7343), Val::from(1)])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2138), Val::from(2139)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines_as(
                "Ishmael",
                args!["If only I only had", "my precious pen made", "of Sardonyx! Waaaah~!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Ishmael",
            args![
                "High Priestess Niren is",
                "such a nice lady. I have to",
                "help her out whenever I get",
                "the chance. Er, do you mind",
                "stepping back a bit? I just...",
                "I don't like being near people!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn ishmael_em(ctx: &Ctx) -> Script {
    ishmael_em_body(ctx, Vec::new()).map(|_| ())
}

fn niren_em_sky_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn niren_em_sky(ctx: &Ctx) -> Script {
    niren_em_sky_body(ctx, Vec::new()).map(|_| ())
}

fn zhed_em_sky_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn zhed_em_sky(ctx: &Ctx) -> Script {
    zhed_em_sky_body(ctx, Vec::new()).map(|_| ())
}

fn pope_rachel2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn pope_rachel2(ctx: &Ctx) -> Script {
    pope_rachel2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum EmSkySStep {
    Start,
    OnTouch,
}

fn em_sky_s_run(ctx: &Ctx, mut step: EmSkySStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EmSkySStep::Start => {
                step = EmSkySStep::OnTouch;
                continue 'machine;
            }
            EmSkySStep::OnTouch => {
                if ctx.var("aru_em").get()? == 21 {
                    ctx.lines(args![
                        "^3355FFHigh Priest Zhed and High",
                        "Priestess Niren have already",
                        "arrived, and are carefully",
                        "watching the pope's face.",
                        "They seem worried about",
                        "what will happen...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(0)])?;
                    ctx.lines_as(
                        "Zhed",
                        args![
                            "How have you been, Your",
                            "Eminence? I apologize for",
                            "not visiting you sooner, but",
                            "I've been too ashamed about",
                            "my competency as a high priest."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as("Pope", args!["High Priest Zhed...", "You came... Thank you..."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe pope faintly smiled",
                        "at High Priest Zhed, glad",
                        "to finally see him again.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(0)])?;
                    ctx.lines_as(
                        "Niren",
                        args![
                            "Your Eminence...",
                            "I hope you understand",
                            "that I've tried my best",
                            "to only show you what is",
                            "beautiful in this world..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as("Pope", args!["...............................", "High Priestess Niren..."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(0)])?;
                    ctx.lines_as(
                        "Niren",
                        args![
                            "However, today I must",
                            "reveal to you the world's",
                            "ugliness as well. I'm sorry...",
                            "I gave the Sky Garden people",
                            "a vacation today so that we",
                            "tell you something in private."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as("Pope", args!["Niren... What is it?"])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(0)])?;
                    ctx.lines_as(
                        "Zhed",
                        args![
                            "You may want to brace",
                            "yourself, Your Eminence,",
                            "for what we are about to",
                            "tell you. You may very",
                            "well be shocked..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFHigh Priest Zhed explained",
                        "all of Arunafeltz's secrets",
                        "in detail. He revealed the",
                        "split between the moderates",
                        "and hard liners among the",
                        "priests, the corruption...^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFHe also explained the role",
                        "of the priests in the inhumane",
                        "testing conducted by the",
                        "Rekenber Corporation, and",
                        "the hard liner priests grab",
                        "for power at Thor Volcano.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe pope stared at",
                        "High Priest Zhed in",
                        "shock as he spoke.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Zhed",
                        args![
                            "We all understand that",
                            "you're already burdened",
                            "enough with your duties",
                            "as pope, but we feel that",
                            "you should take action soon."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "...............................",
                            "...............................",
                            "..............................."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(2)])?;
                    ctx.lines_as(
                        "Niren",
                        args![
                            "We don't mean to force",
                            "you, Your Eminence, but",
                            "we don't have much time.",
                            "We're facing the possibility",
                            "of war, and we must stop it",
                            "in order to save Arunafeltz."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "...............................",
                            "...............................",
                            "...............................",
                            ".......................Finally."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gman2"), Val::from(0)])?;
                    ctx.lines_as("Zhed, Niren", args!["Excuse me?"])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "Niren, Zhed...",
                            "I'm glad that you all",
                            "finally came to me for",
                            "a solution to all this.",
                            "I'm young, but I remember",
                            "what I saw in the holy ground."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "Don't you remember, Niren?",
                            "You found me in the holy",
                            "ground, and safely led me",
                            "out. But while I was there,",
                            "I saw ^3131FFYmir's Heart^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Zhed", args!["How... How did you know", "that was Ymir's Heart?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "The priests in the Sky",
                            "Garden don't mind talking",
                            "freely around me. I guess",
                            "they looked at me more as",
                            "a child, rather than as pope."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "I'm already aware of the",
                            "moderate priest group",
                            "conducting their cruel tests,",
                            "and of the hard liners building",
                            "their military power. However,",
                            "I couldn't really do anything."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "Most people don't take my",
                            "position seriously because",
                            "of my age. Nobody believes",
                            "I can change the situation.",
                            "I've also heard everything",
                            "about what you did, Zhed."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "I'm sorry that you were",
                            "punished by the other priests",
                            "for allowing an outsider to",
                            "enter the holy ground, but",
                            "I understand your reasons.",
                            "Isn't this the adventurer?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                            "I'm very glad to see you",
                            "again. Not only did you",
                            "share news of the outside",
                            "world with me, but you've",
                            "been protecting our peace."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(0)])?;
                    ctx.lines_as(
                        "Niren",
                        args![
                            "I really didn't know...",
                            "How'd you feel about this,",
                            "or how much you actually",
                            "knew. I'm truly sorry, Your",
                            "Eminence. I'm ashamed",
                            "for underestimating you."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "Please don't apologize,",
                            "Niren. If it weren't for you,",
                            "I'd be drawn into the utter",
                            "foolishness of the selfish",
                            "people in this temple. Niren,",
                            "you're like a mother to me."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(0)])?;
                    ctx.lines_as("Niren", args!["Your Eminence...", "Thank you. I feel", "the same way..."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as("Pope", args!["Zhed...?", "I need you to do", "something for me."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(0)])?;
                    ctx.lines_as("Zhed", args!["Of course,", "Your Eminence."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "Write a report about the",
                            "tests conducted by the",
                            "moderates in Arunafeltz and",
                            "the Schwarzwald Republic,",
                            "and their relationship with",
                            "that Schwarzwald Corporation."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Zhed", args!["I'll get it done immediately,", "Your Eminence. Thank you."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as("Pope", args!["Niren."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gwoman"), Val::from(0)])?;
                    ctx.lines_as("Niren", args!["Yes, Your Eminence?"])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "Please collect the",
                            "evidence that proves",
                            "that the hard liners have",
                            "been gearing for war."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Niren", args!["Yes, Your Eminence."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "I'll summon all the High",
                            "Priests and Priestesses",
                            "soon. I'd like the two of",
                            "you to stand by my side.",
                            "I won't let anyone disgrace",
                            "our holy ground with bloodshed."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_gman"), Val::from(0)])?;
                    ctx.lines_as("Zhed", args!["As you command,", "Your Eminence."])?;
                    ctx.next()?;
                    ctx.call(Function::Cutin, vec![Val::from("ra_bishop"), Val::from(2)])?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...")),
                            "I appreciate all your work",
                            "on behalf of Arunafeltz.",
                            "My influence may still be",
                            "weak, but I'll do my best",
                            "with Zhed and Niren's support."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Pope",
                        args![
                            "I would be expecting",
                            "too much to ask you to",
                            "stay by my side. But I'd like",
                            "to thank you for all you've",
                            "done. But if it is Freya's",
                            "will, we'll meet again..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Pope", args!["May Freya bless you..."])?;
                    ctx.next()?;
                    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_ENCHANTPOISON")?])?;
                    ctx.lines(args![
                        "^3355FFThe pope prayed in earnest",
                        "for you, and you feel a strong",
                        "aura of warmth and kindness",
                        "permeate your entire being.^000000"
                    ])?;
                    ctx.var("aru_em").set(Val::from(22))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2140), Val::from(2141)])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("rachel"), Val::from(142), Val::from(136)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn em_sky_s(ctx: &Ctx) -> Script {
    em_sky_s_run(ctx, EmSkySStep::Start, Vec::new()).map(|_| ())
}

pub fn em_sky_s_ontouch(ctx: &Ctx) -> Script {
    em_sky_s_run(ctx, EmSkySStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn em_end(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::Start, Vec::new()).map(|_| ())
}

pub fn em_end_oninit(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnInit, Vec::new()).map(|_| ())
}

pub fn em_end_ontouch(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer4000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer7000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer10000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer10000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer15000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer15000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer19000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer19000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer23000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer23000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer30000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer35000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer35000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer43000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer43000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer47000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer47000, Vec::new()).map(|_| ())
}

pub fn em_end_ontimer53000(ctx: &Ctx) -> Script {
    em_end_run(ctx, EmEndStep::OnTimer53000, Vec::new()).map(|_| ())
}

fn muff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(907), Val::from(200)])? == 0 {
        ctx.lines_as(
            "Muff",
            args![
                "You're carrying way too",
                "much stuff right now.",
                "Come back after you put",
                "your stuff in Kafra Storage."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 0 {
        ctx.lines_as(
            "Muff",
            args![
                "What th-?! Geez,",
                "you didn't have to",
                "scare me like that!",
                "What is it that you want?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("You look troubled.:Nothing. Take care!")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.var("BaseLevel").get()?.number()? < 61 {
                    ctx.lines_as(
                        "Muff",
                        args![
                            "Oh. That's awfully",
                            "thoughtful of you.",
                            "I was thinking about",
                            "asking you for help,",
                            "but you don't look like",
                            "you could handle it. Sorry."
                        ],
                    )?;
                    ctx.var("diamond_edq").set(Val::from(3))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Muff",
                        args![
                            "Well, I'm actually in",
                            "pretty big trouble.",
                            "I borrowed some money",
                            "from this loan shark",
                            "named ^0000FFBelder^000000 in Alberta.",
                            "It was a bad move!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Muff",
                        args![
                            "My business hasn't been",
                            "good, but I felt like I didn't",
                            "have a choice. I made enough",
                            "money to repay him now, but",
                            "then I lost my bond of debt."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Muff",
                        args![
                            "When I asked Belder if",
                            "I could pay him back without",
                            "that bond, he insisted that",
                            "he didn't remember loaning",
                            "money to me. I bet he just",
                            "wants to keep my collateral!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Muff",
                        args![
                            "I knew that guy was shady!",
                            "Would you help me find my",
                            "lost bond of debt? You see,",
                            "the collateral I gave him",
                            "is really valuable to me."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Do you know where you lost it?:I'm sorry to hear that. Bye!")])? {
                        1 => {
                            ctx.lines_as(
                                "Muff",
                                args![
                                    "If I knew where I lost",
                                    "it, then it wouldn't be",
                                    "lost now, would it? Well,",
                                    "I remember going to a union",
                                    "meeting, and I had a drink",
                                    "or two. Okay, I had a lot."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Muff",
                                args![
                                    "On the way home,",
                                    "I stumbled in a field",
                                    "near Comodo... I remember",
                                    "seeing some water... That",
                                    "must be where I dropped my",
                                    "wallet with the bond of debt."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Muff",
                                args![
                                    "If you can find my",
                                    "wallet, I'll make sure",
                                    "to repay you. Please",
                                    "help me if you can!"
                                ],
                            )?;
                            ctx.call(Function::SetQuest, vec![Val::from(3100)])?;
                            ctx.var("diamond_edq").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Muff",
                                args!["H-hey! Where are you", "going?! You can't just", "leave me here! I need help!"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Muff",
                    args![
                        "What, are you trying to",
                        "make me mad on purpose?",
                        "I get it. You're working for",
                        "Belder! I'm gonna get",
                        "my treasure from that",
                        "bastard, no matter what!"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I don't understand.:I'm sorry, I was just kidding...")])? {
                    1 => {
                        ctx.lines_as(
                            "Muff",
                            args![
                                "You think you can",
                                "fool me?! Get lost!",
                                "I'm not telling you",
                                "anything, you snake!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Muff",
                            args!["So you're not spying on", "me for Belder? Well, then", "why are you pestering me?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    if (ctx.var("diamond_edq").get()? == 3 && ctx.var("BaseLevel").get()?.number()? < 61) {
        ctx.lines_as(
            "Muff",
            args![
                "Wait a second...",
                "You're not as strong",
                "as you were earlier.",
                "You'd better train first",
                "so you can be better",
                "prepared to help me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("diamond_edq").get()? == 3 && ctx.var("BaseLevel").get()?.number()? > 60) {
        ctx.lines_as(
            "Muff",
            args![
                "Well, I'm actually in",
                "pretty big trouble.",
                "I borrowed some money",
                "from this loan shark",
                "named ^0000FFBelder^000000 in Alberta.",
                "It was a bad move!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "My business hasn't been",
                "good, but I felt like I didn't",
                "have a choice. I made enough",
                "money to repay him now, but",
                "then I lost my bond of debt."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "When I asked Belder if",
                "I could pay him back without",
                "that bond, he insisted that",
                "he didn't remember loaning",
                "money to me. I bet he just",
                "wants to keep my collateral!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "I knew that guy was shady!",
                "Would you help me find my",
                "lost bond of debt? You see,",
                "the collateral I gave him",
                "is really valuable to me."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Do you know where you lost it?:What was the collateral?:Bye!")],
        )? {
            1 => {
                ctx.lines_as(
                    "Muff",
                    args![
                        "If I knew where I lost",
                        "it, then it wouldn't be",
                        "lost now, would it? Well,",
                        "I remember going to a union",
                        "meeting, and I had a drink",
                        "or two. Okay, I had a lot."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Muff",
                    args![
                        "I took this path in the",
                        "Papuchicha Forest to head",
                        "back home, and I must have",
                        "passed out near a river.",
                        "After that, my wallet with the",
                        "bond of debt was missing."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Muff",
                    args![
                        "I don't care about the",
                        "other stuff in my wallet,",
                        "but I really need my bond",
                        "of debt to get my collateral",
                        "back from that Belder."
                    ],
                )?;
                ctx.call(Function::SetQuest, vec![Val::from(3100)])?;
                ctx.var("diamond_edq").set(Val::from(1))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Muff",
                    args![
                        "Well, the collateral was",
                        "this huge, stunningly",
                        "beautiful jewel. It's really",
                        "precious to me. But well,",
                        "I don't want to tell you",
                        "more about it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Muff",
                    args![
                        "I understand if you don't",
                        "want to help me after",
                        "hearing that, but if you",
                        "change your mind, please",
                        "come talk to me again."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Muff",
                    args!["H-hey! Where are you", "going?! You can't just", "leave me here! I need help!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("diamond_edq").get()? == 1 || ctx.var("diamond_edq").get()? == 2) {
        ctx.lines_as(
            "Muff",
            args![
                "You didn't leave to",
                "look for my wallet yet?",
                "I think it's somewhere",
                "in the Papuchicha Forest.",
                "Please find the bond of debt",
                "inside as soon as you can."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()? == 4 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Is this wet, tattered", "piece of paper the bond", "of debt that you need?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "Yes, that's it!",
                "Ugh, but look at it...",
                "It's ruined! You can't",
                "even read what's written",
                "on it! I need... I need",
                "to fix this somehow!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Do you have any ideas?",
                "I mean, if we dried the",
                "paper, the letters would",
                "still be faded, right?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "I got it! There's this",
                "famous inventor named",
                "^FF0000Dorian^000000 in Izlude. I heard",
                "he just invented something",
                "like a Magic Dryer. Yeah,",
                "that should work perfectly!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "I don't know exactly",
                "how it works, but it",
                "should be able to restore",
                "my bond of debt. I mean...",
                "It's magic, right? Will you",
                "ask Dorian to help me out?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Accept His Request:Decline His Request")])? {
            1 => {
                ctx.lines_as(
                    "Muff",
                    args![
                        "Thank you so much!",
                        "Please talk to Inventor",
                        "Dorian in Izlude, and ask",
                        "him to restore my bond of",
                        "debt. I'll be waiting for",
                        "you right here, okay?"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3102), Val::from(3103)])?;
                ctx.var("diamond_edq").set(Val::from(6))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Muff",
                    args![
                        "Really? I was hoping",
                        "that you'd continue to",
                        "help me, but... Alright.",
                        "I'm sure you've got other",
                        "problems. If you change your",
                        "mind, though, just come back."
                    ],
                )?;
                ctx.var("diamond_edq").set(Val::from(5))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("diamond_edq").get()? == 5 {
        ctx.lines_as(
            "Muff",
            args![
                "Oh, I knew you'd come",
                "back and help me restore",
                "my bond of debt! You look",
                "too nice to just leave",
                "me hanging like that."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Accept His Request:Decline His Request")])? {
            1 => {
                ctx.lines_as(
                    "Muff",
                    args![
                        "Thank you so much!",
                        "Please talk to Inventor",
                        "Dorian in Izlude, and ask",
                        "him to restore my bond of",
                        "debt. I'll be waiting for",
                        "you right here, okay?"
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3102), Val::from(3103)])?;
                ctx.var("diamond_edq").set(Val::from(6))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Muff",
                    args![
                        "Really? I was hoping",
                        "that you'd continue to",
                        "help me, but... Alright.",
                        "I'm sure you've got other",
                        "problems. If you change your",
                        "mind, though, just come back."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if (ctx.var("diamond_edq").get()?.number()? > 5 && ctx.var("diamond_edq").get()?.number()? < 13) {
        ctx.lines_as(
            "Muff",
            args![
                "Would you please visit",
                "^FF0000Inventor Dorian^000000 in Izlude,",
                "and ask him to use his",
                "Magic Dryer to restore",
                "my bond of debt?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("diamond_edq").get()? == 13 && ctx.call(Function::CountItem, vec![Val::from(7722)])?.number()? > 0) {
        ctx.lines_as(
            "Muff",
            args![
                "You're back! So how",
                "did it go? I was getting",
                "pretty anxious... So did",
                "Dorian's Magic Dryer work?"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DelItem, vec![Val::from(7722), Val::from(1)])?;
        ctx.lines_as(
            "Muff",
            args![
                "Oh! It worked much better",
                "than I expected! Belder",
                "can't complain now! Haha!",
                "You've done well, my friend.",
                "Here, please take these as",
                "a meager reward for your help."
            ],
        )?;
        ctx.call(Function::CompleteQuest, vec![Val::from(3109)])?;
        ctx.var("diamond_edq").set(Val::from(14))?;
        ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(608), Val::from(4)])?;
        ctx.next()?;
        ctx.lines_as(
            "Muff",
            args![
                "Now that I have my bond",
                "of debt, I should get my",
                "jewel back from Belder.",
                "He won't have any excuse",
                "to keep my treasure now!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("diamond_edq").get()? == 13 && ctx.call(Function::CountItem, vec![Val::from(7722)])?.number()? < 1) {
        ctx.lines_as(
            "Muff",
            args![
                "H-hey! Where's my",
                "bond of debt?! Don't",
                "tell me you lost it!",
                "You'd better go back",
                "to Dorian... Hopefully",
                "you left it with him!"
            ],
        )?;
        ctx.var("diamond_edq").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()?.number()? > 13 {
        ctx.lines_as(
            "Muff",
            args![
                "Yes! Finally, I'm",
                "free of debt! I've got",
                "my jewel back, business",
                "is doing well... Life sure",
                "is good right now. Hahaha!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Muff",
        args![
            "Listen to me: never",
            "take out a loan if you",
            "can help it. And if you do,",
            "borrow from someone",
            "more reputable than...",
            "Well, you know my story.",
            "*Sigh*"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn muff(ctx: &Ctx) -> Script {
    muff_body(ctx, Vec::new()).map(|_| ())
}

fn belder_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("diamond_edq").get()? == 0 {
        ctx.lines_as(
            "Belder",
            args![
                "I ought to hire some",
                "part-time promotional",
                "workers... Some sexy",
                "ladies ought to bring in",
                "the customers by th--Oh!",
                "Welcome to Belder Loans!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("I'll promote your loans!:Uh...")])? {
            1 => {
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines_as(
                        "Belder",
                        args![
                            "You? But you're a dude!",
                            "No sweaty, stinky, sleazy",
                            "man should be the image",
                            "of Belder Loans! We need",
                            "to exude trust, dependability,",
                            "and... And lady charms."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Belder",
                        args![
                            "You? Well, don't take",
                            "this the wrong way, but",
                            "all your curves? Wrong",
                            "places, honey. Later!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Belder",
                    args![
                        "Hey, aren't you",
                        "interested in a loan?",
                        "Better you come to",
                        "me than you relying",
                        "on some guy that'll",
                        "try to rip you off!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("diamond_edq").get()?.number()? < 13 {
        ctx.lines_as(
            "Belder",
            args![
                "Welcome! So, do you",
                "need money fast? Belder",
                "Loans get you the money",
                "you need at low interest rates!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ask About Muff")])? {
            1 => {
                ctx.lines_as(
                    "Belder",
                    args![
                        "Muff? Oh! Is that",
                        "supposed to be a name?",
                        "I thought you meant--No.",
                        "Nope. Doesn't ring a bell."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Belder",
                    args![
                        "Huh? Collateral?",
                        "Now why would I refuse",
                        "to take back his money",
                        "if I really loaned it to him?",
                        "Leave me alone now, this kind",
                        "of talk is bad for business!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    if ctx.var("diamond_edq").get()?.number()? > 12 {
        ctx.lines_as(
            "Belder",
            args![
                "He really found his bond",
                "of debt? Damn it--I mean,",
                "what a surprise! That's,",
                "uh, real good for him.",
                "Now that I think of it, I did",
                "loan a Muff money once."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["......"])?;
        ctx.next()?;
        ctx.lines_as(
            "Belder",
            args![
                "What's with that dirty look?",
                "I've got nothing to hide!",
                "Can't you see that I'm",
                "the victim here? Me!",
                "Just... Scram before you",
                "make me more upset."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Belder",
        args![
            "Welcome! Did you need",
            "money fa--Oh. Hey, it's",
            "you again, the do-gooder.",
            "Why don't you do me some",
            "good, and go back where",
            "you came from. Jerk."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn belder(ctx: &Ctx) -> Script {
    belder_body(ctx, Vec::new()).map(|_| ())
}

fn belder_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("diamond_edq").get()?.number()? < 3 {
        ctx.lines_as(
            "Belder",
            args![
                "Cash flow problems?",
                "Well, I'm your solution!",
                "I offer unsecured loans",
                "at an extremely low interest",
                "rate, twenty-four hours a day!",
                "Belder Loans is Alberta's best!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn belder_ontouch(ctx: &Ctx) -> Script {
    belder_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn heap_of_earth_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("diamond_edq").get()? == 1 {
        ctx.lines(args![
            "^3355FFIt looks like someone",
            "dug a hole in the ground,",
            "and then covered it again.^000000"
        ])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Dig Up the Spot:Cancel")])? {
            1 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["There must be something", "in the ground. I better", "dig it up and check..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Ergh...! Ugh!",
                        "This would be a lot",
                        "easier with a shovel!",
                        "Only a little more to go..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Hm? I hit something?",
                        "Let's--Yucky! It's so",
                        "wet and--Ugh! Nasty",
                        "little thing. Now what?",
                        "I dropped it! I have to",
                        "try to dig it up again..."
                    ],
                )?;
                ctx.call(Function::ChangeQuest, vec![Val::from(3100), Val::from(3101)])?;
                ctx.var("diamond_edq").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Something isn't quite",
                        "kosher about all this.",
                        "Nope, this time, I'm",
                        "not going to do this."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("diamond_edq").get()? == 2 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Here we are... It's...",
                "A soggy wallet? This",
                "must be the one Muff",
                "lost. Oh, and the bond",
                "of debt is inside... Mm...",
                "Ugh, it's really soaked..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Well, I've done all", "I can for now. I should", "bring this back to Muff."],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(3101), Val::from(3102)])?;
        ctx.var("diamond_edq").set(Val::from(4))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("diamond_edq").get()?.number()? > 3 {
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["This is the same spot", "I dug up from before.", "It's useless to me now!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args![
            "Huh. There's a weird",
            "bump in the ground. Well,",
            "I can't just go looking at",
            "anything that's slightly",
            "out of the ordinary. Heh.",
            "I'm not weird like that."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn heap_of_earth(ctx: &Ctx) -> Script {
    heap_of_earth_body(ctx, Vec::new()).map(|_| ())
}
