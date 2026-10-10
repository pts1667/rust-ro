use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn representative_li_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_curse").get()? == 7 {
        ctx.lines_as(
            "Representative",
            args![
                "Greetings, and welcome",
                "to the Rekenber Corporation.",
                "How may I be of service today?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Building Information:Corporation History")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Representative",
                    args!["Please tell me", "which floor you'd like", "to know more about."],
                )?;
                ctx.next()?;
                'l2: loop {
                    if !(true) {
                        break 'l2;
                    }
                    'b2: {
                        match runtime::select_values(ctx, &[Val::from("1F:2F:B1:Cancel")])? {
                            1 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The ^3131FFRekenber Library^000000 can",
                                        "be found at the end of the",
                                        "left hallway. Our library is",
                                        "a great resource of innovative",
                                        "ideas and information for our",
                                        "system development employees."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The ^3131FFBall Room^000000, where",
                                        "various official events are",
                                        "usually held, can be accessed",
                                        "through the right hallway."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "Please use the stairs",
                                        "located on both sides of",
                                        "the Help Desk to go to the",
                                        "Second Floor. The Second",
                                        "Floor is mostly used for",
                                        "administrative purposes."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "There, you can find",
                                        "the ^3131FFConference Room^000000,",
                                        "^3131FFSecretary's Office^000000, the",
                                        "^3131FFAuditorium^000000 and the",
                                        "^3131FFChairman's Office^000000."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The first underground floor",
                                        "is used by ^3131FFRegenschirm^000000,",
                                        "our laboratory affiliate. For",
                                        "security reasons, this floor",
                                        "is not accessible to visitors."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "We are always doing our",
                                        "best to provide the best",
                                        "services to our customers.",
                                        "Remember that Rekenber",
                                        "is the name you can trust.",
                                        "Thank you and have a nice day."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Representative",
                    args![
                        "If you're interested in",
                        "learning the history of",
                        "our corporation, please",
                        "speak to the representative",
                        "inside our Library. Thank you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Representative",
                    args![
                        "Please head down",
                        "the hallway to the left in",
                        "order to find our Library.",
                        "Thank you and have a nice day."
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(8))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2087), Val::from(2088)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as(
            "Representative",
            args![
                "Greetings, and welcome",
                "to the Rekenber Corporation.",
                "How may I be of service today?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Building Information.")])? {
            1 => {
                ctx.lines_as(
                    "Representative",
                    args!["Please tell me", "which floor you'd like", "to know more about."],
                )?;
                ctx.next()?;
                'l5: loop {
                    if !(true) {
                        break 'l5;
                    }
                    'b5: {
                        match runtime::select_values(ctx, &[Val::from("1F:2F:B1:Cancel")])? {
                            1 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The ^3131FFRekenber Library^000000 can",
                                        "be found at the end of the",
                                        "left hallway. Our library is",
                                        "a great resource of innovative",
                                        "ideas and information for our",
                                        "system development employees."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The ^3131FFBall Room^000000, where",
                                        "various official events are",
                                        "usually held, can be accessed",
                                        "through the right hallway."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "Please use the stairs",
                                        "located on both sides of",
                                        "the Help Desk to go to the",
                                        "Second Floor. The Second",
                                        "Floor is mostly used for",
                                        "administrative purposes."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "There, you can find",
                                        "the ^3131FFConference Room^000000,",
                                        "^3131FFSecretary's Office^000000, the",
                                        "^3131FFAuditorium^000000 and the",
                                        "^3131FFChairman's Office^000000."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The first underground floor",
                                        "is used by ^3131FFRegenschirm^000000,",
                                        "our laboratory affiliate. For",
                                        "security reasons, this floor",
                                        "is not accessible to visitors."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "We are always doing our",
                                        "best to provide the best",
                                        "services to our customers.",
                                        "Remember that Rekenber",
                                        "is the name you can trust.",
                                        "Thank you and have a nice day."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn representative_li_01(ctx: &Ctx) -> Script {
    representative_li_01_body(ctx, Vec::new()).map(|_| ())
}

fn representative_li_02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_curse").get()?.number()? > 6 {
        ctx.lines_as(
            "Representative",
            args!["Welcome to the", "Rekenber Corporation.", "How may I help you?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Corporation History:Rekenber's Businesses")])? {
            1 => {
                ctx.lines_as(
                    "Representative",
                    args![
                        "Rekenber was established",
                        "400 years ago, around the",
                        "same time as the foundation",
                        "of the Schwarzwald Republic."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Representative",
                    args![
                        "We began as the ''Zent Zerter",
                        "Lighthal Research Center,''",
                        "named after our first chairman.",
                        "In 560 A.W. (After War), our",
                        "organization was renamed after",
                        "our new chairman, Mr. Rekenber."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Representative",
                    args![
                        "Mr. Rekenber expanded the",
                        "Corporation's purposes, but",
                        "also founded the Regenschirm",
                        "Laboratory to continue this",
                        "company's original goal of",
                        "scientific research."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Representative",
                    args![
                        "In the year 700 A.W.,",
                        "^FF0000Doctor Varmunt^000000 joined",
                        "Regenschirm. It was his work",
                        "in science that enabled the",
                        "Rekenber Corporation to grow into the nation's biggest company."
                    ],
                )?;
                if ctx.var("lhz_curse").get()? == 8 {
                    ctx.var("lhz_curse").set(Val::from(9))?;
                } else if ctx.var("lhz_curse").get()? == 9 {
                    ctx.var("lhz_curse").set(Val::from(10))?;
                }
                ctx.next()?;
            }
            2 => {
                ctx.lines_as(
                    "Representative",
                    args![
                        "In addition to merchandising,",
                        "freight transport and trading,",
                        "the Rekenber Corporation is",
                        "also heavily involved with",
                        "providing the Airship service,",
                        "one of our major projects."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Representative",
                    args![
                        "Rekenber is involved in",
                        "almost any business that",
                        "you can imagine. Remember",
                        "that Rekenber is the name",
                        "that you can trust."
                    ],
                )?;
                if ctx.var("lhz_curse").get()? == 8 {
                    ctx.var("lhz_curse").set(Val::from(9))?;
                } else if ctx.var("lhz_curse").get()? == 9 {
                    ctx.var("lhz_curse").set(Val::from(10))?;
                }
                ctx.next()?;
            }
            _ => {}
        }
        ctx.lines_as(
            "Representative",
            args![
                "If you'd like to know more",
                "about our mission statement,",
                "please refer to the Rekenber",
                "Guidebook located to my side.",
                "Thank you and have a nice day."
            ],
        )?;
        if ctx.call(Function::IsBeginQuest, vec![Val::from(2088)])?.is_true() {
            ctx.call(Function::ChangeQuest, vec![Val::from(2088), Val::from(2089)])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Representative",
            args!["Welcome to the", "Rekenber Corporation.", "How may I help you?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Building Information.")])? {
            1 => {
                ctx.lines_as(
                    "Representative",
                    args!["Please tell me", "which floor you'd like", "to know more about."],
                )?;
                ctx.next()?;
                'l3: loop {
                    if !(true) {
                        break 'l3;
                    }
                    'b3: {
                        match runtime::select_values(ctx, &[Val::from("1F:2F:B1:Cancel")])? {
                            1 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The ^3131FFRekenber Library^000000 can",
                                        "be found at the end of the",
                                        "left hallway. Our library is",
                                        "a great resource of innovative",
                                        "ideas and information for our",
                                        "system development employees."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The ^3131FFBall Room^000000, where",
                                        "various official events are",
                                        "usually held, can be accessed",
                                        "through the right hallway."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "Please use the stairs",
                                        "located on both sides of",
                                        "the Help Desk to go to the",
                                        "Second Floor. The Second",
                                        "Floor is mostly used for",
                                        "administrative purposes."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "There, you can find",
                                        "the ^3131FFConference Room^000000,",
                                        "^3131FFSecretary's Office^000000, the",
                                        "^3131FFAuditorium^000000 and the",
                                        "^3131FFChairman's Office^000000."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "The first underground floor",
                                        "is used by ^3131FFRegenschirm^000000,",
                                        "our laboratory affiliate. For",
                                        "security reasons, this floor",
                                        "is not accessible to visitors."
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.lines_as(
                                    "Representative",
                                    args![
                                        "We are always doing our",
                                        "best to provide the best",
                                        "services to our customers.",
                                        "Remember that Rekenber",
                                        "is the name you can trust.",
                                        "Thank you and have a nice day."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn representative_li_02(ctx: &Ctx) -> Script {
    representative_li_02_body(ctx, Vec::new()).map(|_| ())
}

fn rekenber_guidebook_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("..............")?;
    ctx.next()?;
    ctx.lines(args![
        "^3131FF#The Vision^000000",
        "In the pursuit of knowledge,",
        "Rekenber will search the",
        "Rune-Midgarts continent for",
        "ancient relics. We hope to make",
        "significant scientific progress by learning the secrets of the past."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "By making scientific",
        "headway, we hope we can",
        "improve current technologies",
        "to provide more convenient",
        "and affordable services in",
        "the Schwarzwald Republic."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^3131FF#The Commitment^000000",
        "Although magic and the",
        "power of the gods has always",
        "maintained an aura of mystery",
        "and superstition, Rekenber hopes^FFFFFF ^3131FF to understand these forces from",
        "a more logical standpoint."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "^FF0000Our goal is to make",
        "the lives of our customers",
        "easier and more enjoyable",
        "by making the ancient power",
        "of the gods more accessible",
        "by means of new technologies."
    ])?;
    if ctx.var("lhz_curse").get()? == 10 {
        ctx.var("lhz_curse").set(Val::from(11))?;
    }
    ctx.next()?;
    ctx.mes("..............")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rekenber_guidebook_li(ctx: &Ctx) -> Script {
    rekenber_guidebook_li_body(ctx, Vec::new()).map(|_| ())
}

fn mad_scientist_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_exit = Val::from(0);
    if ctx.var("lhz_curse").get()? == 13 {
        ctx.lines_as(
            "Wolfchev",
            args![
                "No one shall",
                "interrupt my",
                "research! If you",
                "dare, I'll simply...",
                "Eat you. Eat you alive."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("No... N-no!:Do you need any help?")])? {
            1 => {
                ctx.lines_as("Wolfchev", args!["Out of my sight,", "microcephalic fool!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Wolfchev",
                    args![
                        "Huh? You think I have",
                        "the luxury of remembering",
                        "the face of every part-timer",
                        "I've fired? Get lost, or I'll",
                        "treat you to the pain of",
                        "being eaten alive!"
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(14))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("lhz_curse").get()? == 14 {
        ctx.lines_as("Wolfchev", args!["You again?!", "What the hell do", "you want from me?!"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Let me speak with you.:Sorry for bothering you.")])? {
            1 => {
                ctx.lines_as(
                    "Wolfchev",
                    args!["I don't have time to", "waste with drivel! I'm", "too busy with my research!"],
                )?;
                ctx.next()?;
                ctx.lines_as("Wolfchev", args!["...........!"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Wolfchev",
                    args![
                        "Ah, but wait! I am collecting",
                        "something. Yes, bring me the",
                        "thing I must collect. Yes, yes.",
                        "Here's a hint... It's round...",
                        "Shiny... Kids love playing games with them! Oh, I said too much!"
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(15))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2089), Val::from(2090)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                ctx.lines_as(
                    "Wolfchev",
                    args![
                        "''Sorry?!'' Do you",
                        "think ''sorry'' will",
                        "get back that precious",
                        "minute I've lost yelling",
                        "at you?! Get the hell out!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("lhz_curse").get()? == 15 {
        if ctx.call(Function::CountItem, vec![Val::from(746)])?.number()? > 0 {
            ctx.call(Function::DelItem, vec![Val::from(746), Val::from(1)])?;
            ctx.lines_as(
                "Wolfchev",
                args![
                    "Yes...! Beads!",
                    "You brought them!",
                    "You're not as dumb",
                    "as I thought you'd be!"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Why do you want Glass Beads?")])? {
                1 => {
                    ctx.lines_as(
                        "Wolfchev",
                        args![
                            "..................",
                            "I take that back!",
                            "You should know by",
                            "now that I would never",
                            "tell you why I neeeeed",
                            "these Beads. Bweh-heh!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Wolfchev",
                args![
                    "This favor you've done",
                    "is worth a small chat and",
                    "I can spare a minute or two",
                    "for you inane questions. So",
                    "what is it you want to know?!"
                ],
            )?;
            ctx.next()?;
            'l4: loop {
                if !(true) {
                    break 'l4;
                }
                'b4: {
                    match runtime::select_values(ctx, &[Val::from("Ask about hobbies:Ask about work")])? {
                        1 => {
                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                ctx.lines_as(
                                    "Wolfchev",
                                    args![
                                        "Hyuu~ I think you're",
                                        "just a little too innocent",
                                        "to know about my secret",
                                        "hobby. Yes, yes, I couldn't",
                                        "tell you possibly, it'd be",
                                        "so weird, so strange..."
                                    ],
                                )?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as(
                                    "Wolfchev",
                                    args![
                                        "Oh. Oh no, oh no,",
                                        "I couldn't possibly...",
                                        "It's a-- I-It's a secret.",
                                        "You wouldn't want to know",
                                        "anyway. Bweh-heh-heh-heh!"
                                    ],
                                )?;
                                ctx.next()?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Wolfchev",
                                args![
                                    "Oh, I don't know if you",
                                    "can call it work. After all,",
                                    "I do research whatever it is",
                                    "I want. And they pay me to",
                                    "do it! This is the best place",
                                    "for a scientist like me, yes."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Wolfchev",
                                args![
                                    "Now, in a perfect world,",
                                    "my test subjects would be",
                                    "much more cooperative, but",
                                    "I suppose I cannot blame",
                                    "them. Not that I cause them",
                                    "undue suffering or anything..."
                                ],
                            )?;
                            ctx.next()?;
                            l_exit = Val::from(1);
                        }
                        _ => {}
                    }
                    if l_exit.clone().is_true() {
                        break 'l4;
                    }
                }
            }
            match runtime::select_values(ctx, &[Val::from("What kind of research?")])? {
                1 => {
                    ctx.lines_as(
                        "Wolfchev",
                        args![
                            "Well, I couldn't tell you",
                            "exactly. But don't you",
                            "worry, the discovery I'm",
                            "working on will benefit the",
                            "entire world, you'll see."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wolfchev",
                        args![
                            "Let's just say that once",
                            "I'm successful, I'll satisfy",
                            "one of mankind's most primal",
                            "instincts, the desire to become",
                            "powerful and gain dominance",
                            "over those that are weaker."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wolfchev",
                        args![
                            "I haven't made as much",
                            "progress as I'd like, but",
                            "no matter. It's only a matter",
                            "of experimentation! Yes, to",
                            "make mankind stronger and",
                            "better and more powerful and--"
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            let choice = runtime::select_values(ctx, &[Val::from("Um, what are you testing on?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Wolfchev",
                args![
                    "What do you know?",
                    "I better get back to",
                    "work if I wish to keep",
                    "on schedule. I can't afford",
                    "the leisure of speaking with",
                    "you any longer. Bweh heh heh!"
                ],
            )?;
            ctx.var("lhz_curse").set(Val::from(16))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Wolfchev",
                args!["I don't have time to", "waste with drivel! I'm", "too busy with my research!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Wolfchev", args!["...........!"])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
            ctx.lines_as(
                "Wolfchev",
                args![
                    "Ah, but wait! I am collecting",
                    "something. Yes, bring me the",
                    "thing I must collect. Yes, yes.",
                    "Here's a hint... It's round...",
                    "Shiny... Kids love playing games with them! Oh, I said too much!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn mad_scientist_li(ctx: &Ctx) -> Script {
    mad_scientist_li_body(ctx, Vec::new()).map(|_| ())
}

fn secretary_slierre_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_li_keka = Val::from(0);
    if ctx.var("lhz_curse").get()? == 26 {
        ctx.lines_as(
            "Sueii Slierre",
            args![
                "Excuse me, but you are",
                "not allowed to be in here.",
                "If you have questions regarding",
                "the Rekenber Corporation, I can",
                "direct you to someone qualified",
                "to give you an answer."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("About the Slums:About Secretary Slierre")])? {
            1 => {
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "The slums? All I know",
                        "is that this corporation",
                        "specifically targeted that",
                        "area in order to provide ample",
                        "opportunity for employment."
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(27))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "I'm Sueii Slierre,",
                        "the personal secretary",
                        "for the chairman of the",
                        "Rekenber Corporation."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        if ctx.var("lhz_curse").get()? == 27 {
            ctx.lines_as(
                "Sueii Slierre",
                args![
                    "Did you have something",
                    "else to ask? I can only",
                    "divulge information that is",
                    "public knowledge, but I can",
                    "tell you who to contact for",
                    "more specific inquiries."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("About the Laboratory:About the Corporation")])? {
                1 => {
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "Rekenber is perhaps the",
                            "biggest contributer of the",
                            "Regenschirm Laboratory.",
                            "Their work will benefit the",
                            "entire Midgard continent, hence our highly involved support."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "A representative at the",
                            "Help Desk will be happy",
                            "to assist you if you have",
                            "more inquiries regarding",
                            "the Regenschirm Laboratory."
                        ],
                    )?;
                    ctx.var("lhz_curse").set(Val::from(28))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "I'm sorry, but I don't have",
                            "any special information",
                            "regarding our corporation.",
                            "Why don't you ask one of",
                            "our representatives at the",
                            "Help Desk to learn more?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("lhz_curse").get()? == 28 {
            ctx.lines_as(
                "Sueii Slierre",
                args!["You're back? I really", "doubt that I can be of", "any assistance to you."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Wolfchev's Research")])? {
                1 => {
                    ctx.lines_as("Sueii Slierre", args![".............!"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sueii Slierre",
                        args!["How do you know", "Wolfchev? Are you an", "acquaintance of his or...?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "I'm a friend of his.:Oh, we're family, you know...:I've heard about him before, so...",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "Well, Wolfchev is not",
                                    "only brilliant, he's also",
                                    "a respected professional.",
                                    "He should be doing just fine."
                                ],
                            )?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "Ah, you should be",
                                    "very proud of Wolfchev.",
                                    "In addition to being a",
                                    "genius, Wolfchev is also",
                                    "quite the professional,",
                                    "truly a model scientist."
                                ],
                            )?;
                            ctx.next()?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "Ah yes, Wolfchev has",
                                    "quite the reputation. In",
                                    "fact, he's in such high",
                                    "demand that Regenschirm",
                                    "wants him on their staff."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "Now if you'll excuse me,",
                            "I have many task to perform,",
                            "so please visit our Help Desk",
                            "if you have further inquiries."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("W-Wait!:Alright, I understand.")])? {
                        1 => {
                            ctx.lines_as("Sueii Slierre", args!["...", "......"])?;
                            ctx.next()?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "Thank you for your",
                                    "cooperation. As you well",
                                    "know, the Help Desk is there",
                                    "to answer any of your questions regarding the Rekenber Corporation."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "I'm sorry, but please",
                            "understand that my office",
                            "isn't the place for visitors to",
                            "submit their general inquiries.",
                            "Please visit the Help Desk if",
                            "you have any more questions."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("About Wolfchev's Research")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "I couldn't tell you any",
                            "more about Wolfchev.",
                            "But is there anything you",
                            "need to tell me about him?",
                            "Well, if you have something",
                            "to ask, be quick about it."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Wolfchev's Past:Wolfchev's Test Subjects")])? {
                        1 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "I remember hearing that he",
                                    "received a high recommendation",
                                    "to work here, but specifics elude me since I don't work in Human",
                                    "Resources. There's a rumor that",
                                    "he had a troubled love life..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "But aside from a few",
                                    "rumors, we really don't",
                                    "know much about Wolfchev's",
                                    "personal life. But then again,",
                                    "it may be unethical to pry too",
                                    "much into our employee's lives."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {}
                        _ => {}
                    }
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "Wolfchev's test subjects?",
                            "Well, I know we have a policy",
                            "of using the most humane",
                            "methods depending on the",
                            "experiment. And of course, he should only be testing on animals."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from("Wolfchev's research is great!:His experiments are suspicious...")],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "Yes, yes, I'd agree if",
                                    "I understood science a",
                                    "little bit better. Now, you'll",
                                    "have to excuse me. I've been distracted long enough as it is..."
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(228), Val::from(226)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Sueii Slierre",
                                args![
                                    "What exactly do you mean?",
                                    "Are you sure that you haven't",
                                    "misunderstood anything about",
                                    "Wolfchev's work? You'll need",
                                    "to illustrate your claim for us",
                                    "to be on the same page..."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Show Evidence:Cancel")])? {
                                1 => {
                                    if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                                        ctx.lines(args![
                                            "^3355FFYou reveal the Handcuffs",
                                            "you found in the laboratory,",
                                            "and Secretary Slierre's face",
                                            "is instantly shadowed by a",
                                            "deeply troubled look.^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as("Sueii Slierre", args!["...", "......"])?;
                                        ctx.next()?;
                                        let choice =
                                            runtime::select_values(ctx, &[Val::from("What's Regenschirm up to?:What's Wolfchev up to?")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as("Sueii Slierre", args!["...", "......"])?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sueii Slierre",
                                            args![
                                                "So... You're",
                                                "suspicious about",
                                                "Wolfchev's research in",
                                                "the Regenschirm Laboratory?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                                            1 => {}
                                            2 => {
                                                ctx.lines_as(
                                                    "Sueii Slierre",
                                                    args![
                                                        "Wait, what exactly",
                                                        "do you feel suspicious",
                                                        "about? Basically, which",
                                                        "party do you feel is most",
                                                        "at fault in this situation?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                match runtime::select_values(ctx, &[Val::from("Regenschirm:Mr. Wolfchev")])? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            "Sueii Slierre",
                                                            args![
                                                                "Let me assure you that",
                                                                "Regenschirm has a strict set",
                                                                "of protocals and procedures",
                                                                "to ensure safety and the",
                                                                "prevention of unnecessary",
                                                                "cruelty in experimentation."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        match runtime::select_values(
                                                            ctx,
                                                            &[Val::from("What about the creatures in Regenschirm?")],
                                                        )? {
                                                            1 => {
                                                                ctx.lines_as(
                                                                    "Sueii Slierre",
                                                                    args![
                                                                        "Creatures? I would",
                                                                        "guess that they're the",
                                                                        "result of experimentation.",
                                                                        "But I wouldn't know for sure."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                    2 => {}
                                                    _ => {}
                                                }
                                            }
                                            _ => {}
                                        }
                                        ctx.lines_as(
                                            "Sueii Slierre",
                                            args![
                                                "I can't be sure right",
                                                "now, but this looks like",
                                                "fairly concrete evidence.",
                                                "We'll send some people over",
                                                "to Regenschirm right away!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Sueii Slierre",
                                            args![
                                                "For now, your claim",
                                                "merits an investigation.",
                                                "I'll let you know if we",
                                                "find anything significant..."
                                            ],
                                        )?;
                                        ctx.var("lhz_curse").set(Val::from(30))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Sueii Slierre",
                                            args![
                                                "Evidence...?",
                                                "I'm sorry, but you don't",
                                                "seem to be carrying anything",
                                                "that can be construed as proof.",
                                                "I suggest you bring something that actually supports your claim."
                                            ],
                                        )?;
                                        ctx.var("lhz_curse").set(Val::from(29))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Sueii Slierre",
                                        args!["If you're finished,", "I'd like to get back on", "task. Please excuse me."],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.var("lhz_curse").set(Val::from(30))?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else if ctx.var("lhz_curse").get()? == 29 {
            ctx.lines_as(
                "Sueii Slierre",
                args![
                    "Hmm, have you come back",
                    "to address your claim about",
                    "Wolfchev's work? If you don't",
                    "have any evidence, then you",
                    "shouldn't be making rumors..."
                ],
            )?;
            ctx.next()?;
            if ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 0 {
                ctx.lines(args![
                    "^3355FFYou reveal the Handcuffs",
                    "you found in the laboratory,",
                    "and Secretary Slierre's face",
                    "is instantly shadowed by a",
                    "deeply troubled look.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as("Sueii Slierre", args!["...", "......"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("What's Regenschirm up to?:What's Wolfchev up to?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Sueii Slierre", args!["...", "......"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "So... You're",
                        "suspicious about",
                        "Wolfchev's research in",
                        "the Regenschirm Laboratory?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                    1 => {}
                    2 => {
                        ctx.lines_as(
                            "Sueii Slierre",
                            args![
                                "Wait, what exactly",
                                "do you feel suspicious",
                                "about? Basically, which",
                                "party do you feel is most",
                                "at fault in this situation?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Regenschirm:Mr. Wolfchev")])? {
                            1 => {
                                ctx.lines_as(
                                    "Sueii Slierre",
                                    args![
                                        "Let me assure you that",
                                        "Regenschirm has a strict set",
                                        "of protocals and procedures",
                                        "to ensure safety and the",
                                        "prevention of unnecessary",
                                        "cruelty in experimentation."
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("What about the creatures in Regenschirm?")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Sueii Slierre",
                                            args![
                                                "Creatures? I would",
                                                "guess that they're the",
                                                "result of experimentation.",
                                                "But I wouldn't know for sure."
                                            ],
                                        )?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                            }
                            2 => {}
                            _ => {}
                        }
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "I can't be sure right",
                        "now, but this looks like",
                        "fairly concrete evidence.",
                        "We'll send some people over",
                        "to Regenschirm right away!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "For now, your claim",
                        "merits an investigation.",
                        "I'll let you know if we",
                        "find anything significant..."
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(30))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "Hmm. I can't consider",
                        "whatever you brought this",
                        "time as evidence that would",
                        "allay my doubts about your",
                        "claim. Now, if you'll excuse",
                        "me, I need to get back on task."
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(29))?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(228), Val::from(226)])?;
                return Err(Stop::End);
            }
        } else if ctx.var("lhz_curse").get()? == 30 {
            l_li_keka = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
            if l_li_keka.clone().number()? > 7 {
                if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                    ctx.lines_as(
                        "Sueii Slierre",
                        args![
                            "Oh, I'd like to have",
                            "a word with you. Would",
                            "you please come back after",
                            "reducing the weight of the",
                            "items you are carrying please?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "Oh good, you're here.",
                        "You were right all along.",
                        "In our investigation, we found",
                        "that Wolfchev was conducting",
                        "unauthorized and very dangerous",
                        "research. I owe you our thanks."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("What was he doing...?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "It turns out that Wolfchev",
                        "was kidnapping weak and sick",
                        "people from the slums and",
                        "using them as his guinea pigs.",
                        "Rest assured, he'll be punished",
                        "for his behavior, if not fired."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "I think you deserve an",
                        "apology. Without your",
                        "report, our corporation's",
                        "reputation could have been",
                        "potentially damaged. Thank you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "Yes, there's nothing so",
                        "taboo as trying to perform",
                        "Homunculus experiments",
                        "on people! Anyway, please",
                        "accept this as a token of",
                        "our gratitude, adventurer."
                    ],
                )?;
                ctx.var("lhz_curse").set(Val::from(31))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2094), Val::from(2095)])?;
                ctx.call(Function::GetItem, vec![Val::from(603), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(12016), Val::from(10)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "Let me promise you",
                        "that Rekenber will ensure",
                        "that this kind of incident",
                        "will not be repeated and",
                        "we'll do everything in our",
                        "power to compensate for this..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Sueii Slierre",
                    args![
                        "Oh, our investigation",
                        "of your claim is still in",
                        "progress. However, we",
                        "will let you know when any",
                        "new developments arise."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("lhz_curse").get()?.number()? > 30 {
            ctx.lines_as(
                "Sueii Slierre",
                args![
                    "I'm glad to know that",
                    "we have such proactive",
                    "and concerned customers",
                    "such as yourself to support",
                    "the Rekenber Corporation."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Sueii Slierre",
                args![
                    "Excuse me, but you are",
                    "not allowed to be in here.",
                    "Please visit the Help Desk",
                    "if you have any questions about",
                    "the Rekenber Corporation.",
                    "Thank you for cooperating."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn secretary_slierre_li(ctx: &Ctx) -> Script {
    secretary_slierre_li_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LiEndStep {
    Start,
    OnTouch,
}

fn li_end_run(ctx: &Ctx, mut step: LiEndStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LiEndStep::Start => {
                step = LiEndStep::OnTouch;
                continue 'machine;
            }
            LiEndStep::OnTouch => {
                if ctx.var("lhz_curse").get()?.number()? > 30 {
                    ctx.lines(args![
                        "^3131FFThere's no trace of",
                        "that mad scientist. Only",
                        "his stacks of well organized",
                        "files remain here in the lab.^000000"
                    ])?;
                    if ctx.var("lhz_curse").get()? == 31 {
                        ctx.var("lhz_curse").set(Val::from(32))?;
                        ctx.call(Function::CompleteQuest, vec![Val::from(2095)])?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn li_end(ctx: &Ctx) -> Script {
    li_end_run(ctx, LiEndStep::Start, Vec::new()).map(|_| ())
}

pub fn li_end_ontouch(ctx: &Ctx) -> Script {
    li_end_run(ctx, LiEndStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum LiToendStep {
    Start,
    OnTouch,
}

fn li_toend_run(ctx: &Ctx, mut step: LiToendStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LiToendStep::Start => {
                step = LiToendStep::OnTouch;
                continue 'machine;
            }
            LiToendStep::OnTouch => {
                if ctx.var("lhz_curse").get()?.number()? > 30 {
                    ctx.call(Function::Warp, vec![Val::from("lhz_que01"), Val::from(97), Val::from(30)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(277), Val::from(130)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn li_toend(ctx: &Ctx) -> Script {
    li_toend_run(ctx, LiToendStep::Start, Vec::new()).map(|_| ())
}

pub fn li_toend_ontouch(ctx: &Ctx) -> Script {
    li_toend_run(ctx, LiToendStep::OnTouch, Vec::new()).map(|_| ())
}

fn file_li_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["Name: Engeod", "Age: XX", "Height: XXX", "Weight: XX"])?;
    ctx.next()?;
    ctx.lines(args!["Name: Kashutii", "Age: XX", "Height: XXX", "Weight: XX"])?;
    ctx.next()?;
    ctx.lines(args!["Name: Prufoz", "Age: XX", "Height: XXX", "Weight: XX"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn file_li(ctx: &Ctx) -> Script {
    file_li_body(ctx, Vec::new()).map(|_| ())
}

fn a_file_li_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["Name: Engeod", "Age: XX", "Height: XXX", "Weight: XX"])?;
    ctx.next()?;
    ctx.lines(args!["Name: Kashutii", "Age: XX", "Height: XXX", "Weight: XX"])?;
    ctx.next()?;
    ctx.lines(args!["Name: Prufoz", "Age: XX", "Height: XXX", "Weight: XX"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn a_file_li_1(ctx: &Ctx) -> Script {
    a_file_li_1_body(ctx, Vec::new()).map(|_| ())
}
