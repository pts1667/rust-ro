use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn sir_jore_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? > 16
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 22)
    {
        if ctx.var("sign_q").get()? == 15 {
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me..."])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.mes("^313FFF*Clink*^000000")?;
            ctx.next()?;
            ctx.lines_as("Sir Jore", args!["...", "......"])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
            ctx.lines_as("Sir Jore", args!["...", "......", "...No!", "Look what", "you made me do!"])?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines_as(
                "Sir Jore",
                args![
                    "I've been fiddling",
                    "with this sample for",
                    "five hours. And now",
                    "it's ruined! ^333333*Sob*^000000"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines_as(
                "Sir Jore",
                args![
                    "^333333*Sob*^000000",
                    "I came to this town",
                    "so I could focus on",
                    "my research without",
                    "any interruptions. So",
                    "why are you here?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Daewoon sent me.:Oops, sorry. Later~")])? {
                1 => {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "O-oh!",
                            "That's right.",
                            "You're here to be",
                            "tested for the piece",
                            "of the Sobbing Starlight."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.lines_as("Sir Jore", args!["So...", "Er. Then, what...", "W-what's your name?"])?;
                    ctx.next()?;
                    let choice = runtime::select_values(
                        ctx,
                        &[(ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(", thanks."))],
                    )?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "N-nice to meet you.",
                            "My name is Jore. Just",
                            "a normal person who loves",
                            "research. S-sorry if I seem",
                            "a little nervous! I'm actually",
                            "quite... shy around people."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "Oh no...!",
                            "If you were able to",
                            "find me, there will be",
                            "others! When would I get",
                            "the time to do my research?!",
                            "N-no! I h-h-hate people!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "Still, I did promise",
                            "M-M-Metz and he is my",
                            "friend. So I must accept",
                            "some guests. Even if hundreds",
                            "of them knock on my door..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "But first of all,",
                            "I'm going to need",
                            "a new research sample.",
                            "I think it's only fair that",
                            "you get it for me since you",
                            "made me ruin the last one."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "N-now, d-don't worry.",
                            "The items are actually",
                            "quite easy to get. It's",
                            "five hours part that's",
                            "hard. Now, let's see..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("sign_q").set(Val::from(16))?;
                    ctx.lines_as(
                        "Sir Jore",
                        args!["Just bring me", "10 Empty Test Tubes,", "10 Green Herbs and", "2 Yggdrasil Leafs."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "You see, l-lately I've",
                            "been studying Biology. ",
                            "I think the secret to life",
                            "can be found in the Leaf of",
                            "Yggdrasil. They can be used",
                            "to revive the dead, after all."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "Oh, and make sure to",
                            "bring those things to me",
                            "me before I go to bed at",
                            "precisely 10:00 PM PST.",
                            "I do have a regular sleeping",
                            "schedule, you know."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "I spent five hours",
                            "observing the changes",
                            "in that research sample.",
                            "All of that hard work lost!",
                            "^333333*Wah~!*^000000"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if ctx.var("sign_q").get()? == 16 {
            if ((ctx.call(Function::CountItem, vec![Val::from(1092)])?.number()? > 9
                && ctx.call(Function::CountItem, vec![Val::from(610)])?.number()? > 1)
                && ctx.call(Function::CountItem, vec![Val::from(511)])?.number()? > 9)
            {
                ctx.call(Function::DelItem, vec![Val::from(1092), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(610), Val::from(2)])?;
                ctx.call(Function::DelItem, vec![Val::from(511), Val::from(10)])?;
                ctx.var("sign_q").set(Val::from(17))?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "Ah! Th-thank you for",
                        "bringing what I need.",
                        "Now I can continue my",
                        "research. Oh, and see if",
                        "you're worthy of obtaining",
                        "the Sobbing Starlight."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "Now, for your assignment.",
                        "Have you ever heard about",
                        "the ^CE3131Stone of Sage^000000? Rumors about",
                        ((Val::from("it have been spreading around, but no one has confirmed the truth about it, ")
                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "Although I have no clue",
                        "what the Stone of S-S-age",
                        "may actually be, I have a gut",
                        "feeling that I need it to bring",
                        "my Biology research to the next step. This is how you'll help me."
                    ],
                )?;
                ctx.next()?;
                ctx.var("sign_q").set(Val::from(17))?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "I need you to investigate",
                        "this Stone of Sage by finding",
                        "a father and daughter who were",
                        "famous for being great Alchemists. They vanished deep into a forest, but I believe they know something."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "Please bring m-me",
                        "10 Empty Test Tube,",
                        "10 Green Herb and",
                        "2 Leaf of Yggdrasil",
                        "so that I can make a",
                        "new research sample."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("sign_q").get()? == 17 {
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "I want you to find two",
                        "Alchemists, a father and",
                        "daughter, who have gone into",
                        "seclusion deep in some forest",
                        "so that I can learn more about",
                        "the Stone of Sage."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "Of course, this will",
                        "possibly further my",
                        "research, but it's also",
                        "how I'll judge whether or",
                        "not you're qualified for my",
                        "piece of the Sobbing Starlight."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()?.number()? < 15 {
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "The mystery of life.",
                        "The desire to discover its",
                        "secrets drives me through",
                        "all the research and sacrifice."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Sir Jore", args!["I-I'm not that confident in", "my abilities, but I do believe that everything in this world is created by certain rules. Still, would I be punished by God for trying to create life he did not intend?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "But I can't stop now.",
                        "One of these days, someone",
                        "will reveal the secret of life.",
                        "Why not me? That knowledge",
                        "would make humans greater",
                        "than any other race..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("sign_q").get()? == 19 {
                ctx.lines_as(
                    "Sir Jore",
                    args!["Ah, you've returned.", "So, did you learn what", "exactly is the Stone of Sage?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        "Well, I'm not sure,",
                        "but I was told that it's",
                        "red and can cure people.",
                        "But it might be something",
                        "we're already familiar with."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "That's all...?",
                        "Maybe it was nothing",
                        "but a rumor after all.",
                        "A-anyway, please tell me",
                        "what you think it might be."
                    ],
                )?;
                ctx.next()?;
                let (input, status) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if runtime::compare(&l_input_s.clone(), &Val::from("red gemstone")).is_true() {
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "What...! Huh.",
                            "Perhaps I should just",
                            "forget about the Stone of",
                            "Sage and just research.",
                            "Still, I'm so ashamed of",
                            "relying on a rumor..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("sign_q").set(Val::from(20))?;
                    ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "Thank you very much",
                            "for helping me. Now I can",
                            "go back to my studies and",
                            "hope that nobody bothers me.",
                            "But that'll never happen.",
                            "^333333*Sigh...*^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Sir Jore", args!["W-well, you've earned this", "piece of the Sobbing Starlight.", "For the next piece, you must visit ^0063FFLady Jesqurienne^000000 in Geffen, the city of magic. Since she travels often, she's probably in the Inn."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Sir Jore",
                        args![
                            "O-o-okay then.",
                            "Good luck getting",
                            "all the pieces of ",
                            "Sobbing Starlight,",
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". Farewell~"))
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Sir Jore",
                        args!["Err...?", "I don't think", "I quite understood", "you. What did you say?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else if ctx.var("sign_q").get()? == 20 {
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "Now you must visit",
                        "^0063FFLady Jesqurienne^000000 in",
                        "Geffen. Good luck",
                        "passing her test."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Sir Jore",
                    args![
                        "H-how is this supposed",
                        "to work? Like this? No!",
                        "No, that's not right!",
                        "Think, Jore, think...",
                        "What's the answer?!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? > 6
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 17)
        {
            ctx.lines(args![
                "^3355FFYou find a tense man",
                "holding test tubes between",
                "his fingers, standing in a pile",
                "of books. He seems to be in",
                "agony for some reason."
            ])?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Speak to him.:Ignore him.")])?);
                let mut matched2 = false;
                let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Excuse me..."])?;
                    ctx.next()?;
                    ctx.lines_as("Sir Jore", args!["...", "......"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFToo preoccupied with",
                        "his thoughts, this strange",
                        "man is unable to hear you.^000000"
                    ])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Try again.:Ignore him.")])? {
                        1 => {
                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["EXCUSE ME!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Sir Jore",
                                args![
                                    "...!",
                                    "Oh, h-h-hello.",
                                    "Sorry, but I'm kind of",
                                    "busy right now. Yes, yes,",
                                    "would you come back at",
                                    "precisely 5:00 PM PST?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Sir Jore",
                                args![
                                    "Let's see...",
                                    "Now if I recalibrated",
                                    "the faust exhaust, then",
                                    "the bioneutron analyzer",
                                    "would need to be adjusted",
                                    "for cytoplasmic balance..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines(args![
                                "^3355FFYou decide to leave",
                                "him alone and let him",
                                "continue mumbling to",
                                "himself and playing",
                                "with his test tubes.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines(args![
                        "^3355FFYou decide to leave",
                        "him alone and let him",
                        "continue mumbling to",
                        "himself and playing",
                        "with his test tubes.^000000"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            ctx.lines_as("Sir Jore", args!["...z...z...Z"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn sir_jore_sign(ctx: &Ctx) -> Script {
    sir_jore_sign_body(ctx, Vec::new()).map(|_| ())
}

fn sir_jore_sign_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PHARMACY_OK")?])?;
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_PHARMACY_FAIL")?])?;
    }
    return Err(Stop::End);
}

pub fn sir_jore_sign_ontouch(ctx: &Ctx) -> Script {
    sir_jore_sign_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn piru_piru_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Piru Piru]")?;
    if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 12
        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? <= 24)
    {
        if ctx.var("sign_q").get()? == 17 {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines(args![
                "Oh, I'm sooo tired~",
                "But we can't sleep yet.",
                "*Sob* Master, can't we",
                "just call it a day already?"
            ])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "What do you do in here?:About vanished Alchemists:What is the Stone of Sage?",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Piru Piru",
                        args![
                            "My master, Sir Jore,",
                            "is researching a way to",
                            "create artificial life! Still,",
                            "it's not easy and we haven't",
                            "accomplished anything yet..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Piru Piru",
                        args![
                            "It doesn't help that my",
                            "master spends all of his",
                            "time on research. He hasn't",
                            "been taking care of himself",
                            "and is losing a lot of weight.",
                            "I'm really worried about him."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Piru Piru", args!["Oh, the father and daughter", "who were both Alchemists, right? Did you know they vanished because they invented the monster potion summoning skill?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Piru Piru",
                        args![
                            "Their discovery caused",
                            "such great hysteria in the",
                            "scientific community. In the",
                            "end they had no choice but",
                            "to live in seclusion deep",
                            "in the ^CE3131forest to the south^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Piru Piru",
                        args![
                            "They devoted their lives",
                            "to their research, just like",
                            "my master. If he makes a major",
                            "breakthrough, we'll have to",
                            "disappear like they did..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                3 => {
                    ctx.lines_as(
                        "Piru Piru",
                        args![
                            "Hmm? Well, I wouldn't",
                            "know anything about that.",
                            "In fact, I'm always staying",
                            "in this lab, so I never hear",
                            "any rumors or news outside."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            ctx.lines(args![
                "Oh, I'm sooo tired~",
                "But we can't sleep yet.",
                "*Sob* Master, can't we",
                "just call it a day already?"
            ])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("What do you do in here?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Piru Piru",
                args![
                    "My master, Sir Jore,",
                    "is researching a way to",
                    "create artificial life! Still,",
                    "it's not easy and we haven't",
                    "accomplished anything yet..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Piru Piru",
                args![
                    "It doesn't help that my",
                    "master spends all of his",
                    "time on research. He hasn't",
                    "been taking care of himself",
                    "and is losing a lot of weight.",
                    "I'm really worried about him."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? >= 6
            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 12)
        {
            ctx.lines(args![
                "Everyday we study and",
                "take notes and test and",
                "experiment and record",
                "results and... Horrible!"
            ])?;
            ctx.next()?;
            ctx.lines_as("Piru Piru", args!["Although my master and I are the only ones who use this lab, we do have a set working schedule. We work from 10 AM to 5 PM and have our free time from 5 PM to 10 PM, which is when Master goes to bed."])?;
            ctx.next()?;
            ctx.lines_as(
                "Piru Piru",
                args![
                    "Of course, Master",
                    "is happy if he can",
                    "just do more research.",
                    "In any case, please visit",
                    "us after 5 PM if you have",
                    "any business with us."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args!["....Zzz...Zzz...", "Zzz....Zzz......"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Wake her up:Leave her sleep")])? {
                1 => {
                    ctx.lines_as(
                        "Piru Piru",
                        args![
                            "Wh-wha...?",
                            "Why do you gotta",
                            "wake me up? Come back",
                            "after 10:00 AM tomorrow, kay?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Piru Piru", args!["Zzzz.....Zzzzz..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn piru_piru_sign(ctx: &Ctx) -> Script {
    piru_piru_sign_body(ctx, Vec::new()).map(|_| ())
}

fn pleur_warp_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_warp_s = Val::from(0);
    ctx.lines(args![
        "^3355FFYou catch a glimpse",
        "of a girl heading directly",
        "into a deep forest. You decide",
        "to follow her and see if you can",
        "learn more.^000000"
    ])?;
    ctx.close_window()?;
    l_warp_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
    if l_warp_s.clone().number()? < 5 {
        ctx.call(Function::Warp, vec![Val::from("prt_maze02"), Val::from(11), Val::from(146)])?;
    }
    if l_warp_s.clone().number()? > 7 {
        ctx.call(Function::Warp, vec![Val::from("prt_maze03"), Val::from(55), Val::from(8)])?;
    }
    if (l_warp_s.clone().number()? > 4 && l_warp_s.clone().number()? < 8) {
        ctx.call(Function::Warp, vec![Val::from("prt_maze01"), Val::from(62), Val::from(129)])?;
    }
    return Err(Stop::End);
}

pub fn pleur_warp(ctx: &Ctx) -> Script {
    pleur_warp_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MovesignStep {
    Start,
    OnTouch,
}

fn movesign_run(ctx: &Ctx, mut step: MovesignStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_warp_s = Val::from(0);
    'machine: loop {
        match step {
            MovesignStep::Start => {
                step = MovesignStep::OnTouch;
                continue 'machine;
            }
            MovesignStep::OnTouch => {
                ctx.lines(args![
                    "^3355FFYou catch a glimpse",
                    "of a girl heading directly",
                    "into a deep forest. You decide",
                    "to follow her and see if you can",
                    "learn more.^000000"
                ])?;
                ctx.close_window()?;
                l_warp_s = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                if l_warp_s.clone().number()? < 5 {
                    ctx.call(Function::Warp, vec![Val::from("prt_maze02"), Val::from(11), Val::from(146)])?;
                }
                if l_warp_s.clone().number()? > 7 {
                    ctx.call(Function::Warp, vec![Val::from("prt_maze03"), Val::from(55), Val::from(8)])?;
                }
                if (l_warp_s.clone().number()? > 4 && l_warp_s.clone().number()? < 8) {
                    ctx.call(Function::Warp, vec![Val::from("prt_maze01"), Val::from(62), Val::from(129)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn movesign(ctx: &Ctx) -> Script {
    movesign_run(ctx, MovesignStep::Start, Vec::new()).map(|_| ())
}

pub fn movesign_ontouch(ctx: &Ctx) -> Script {
    movesign_run(ctx, MovesignStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MazewarpStep {
    Start,
    OnTouch,
}

fn mazewarp_run(ctx: &Ctx, mut step: MazewarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MazewarpStep::Start => {
                step = MazewarpStep::OnTouch;
                continue 'machine;
            }
            MazewarpStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("prt_maze02"), Val::from(90), Val::from(170)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mazewarp(ctx: &Ctx) -> Script {
    mazewarp_run(ctx, MazewarpStep::Start, Vec::new()).map(|_| ())
}

pub fn mazewarp_ontouch(ctx: &Ctx) -> Script {
    mazewarp_run(ctx, MazewarpStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MusicStep {
    Start,
    OnTouch,
}

fn music_run(ctx: &Ctx, mut step: MusicStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MusicStep::Start => {
                step = MusicStep::OnTouch;
                continue 'machine;
            }
            MusicStep::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("effect\\������ ��ؽ�.wav"), Val::from(1)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn music(ctx: &Ctx) -> Script {
    music_run(ctx, MusicStep::Start, Vec::new()).map(|_| ())
}

pub fn music_ontouch(ctx: &Ctx) -> Script {
    music_run(ctx, MusicStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SkillStep {
    Start,
    OnEnable,
    OnDisable,
    OnMyMobDead,
}

fn skill_run(ctx: &Ctx, mut step: SkillStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SkillStep::Start => {
                step = SkillStep::OnEnable;
                continue 'machine;
            }
            SkillStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("prt_maze02"),
                        Val::from(14),
                        Val::from(177),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("#skill::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("prt_maze02"),
                        Val::from(17),
                        Val::from(171),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("#skill::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("prt_maze02"),
                        Val::from(24),
                        Val::from(173),
                        Val::from("Flora"),
                        Val::from(1118),
                        Val::from(1),
                        Val::from("#skill::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("prt_maze02"),
                        Val::from(17),
                        Val::from(175),
                        Val::from("Marine Sphere"),
                        Val::from(1142),
                        Val::from(1),
                        Val::from("#skill::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("prt_maze02"),
                        Val::from(17),
                        Val::from(168),
                        Val::from("Marine Sphere"),
                        Val::from(1142),
                        Val::from(1),
                        Val::from("#skill::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SkillStep::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("prt_maze02"), Val::from("#skill::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            SkillStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn skill(ctx: &Ctx) -> Script {
    skill_run(ctx, SkillStep::Start, Vec::new()).map(|_| ())
}

pub fn skill_onenable(ctx: &Ctx) -> Script {
    skill_run(ctx, SkillStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn skill_ondisable(ctx: &Ctx) -> Script {
    skill_run(ctx, SkillStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn skill_onmymobdead(ctx: &Ctx) -> Script {
    skill_run(ctx, SkillStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn pleur_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Pleur]")?;
    if ctx.var("sign_q").get()?.number()? < 19 {
        ctx.lines(args!["La la la~", "La la la~"])?;
    } else {
        ctx.lines(args!["Hmmm...?", "The Stone of Sage..."])?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pleur_sign(ctx: &Ctx) -> Script {
    pleur_sign_body(ctx, Vec::new()).map(|_| ())
}

fn pleur_sign_onho_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
    return Err(Stop::End);
}

pub fn pleur_sign_onho(ctx: &Ctx) -> Script {
    pleur_sign_onho_body(ctx, Vec::new()).map(|_| ())
}

fn pleur_sign_onkis2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUPCHUP")?])?;
    return Err(Stop::End);
}

pub fn pleur_sign_onkis2(ctx: &Ctx) -> Script {
    pleur_sign_onkis2_body(ctx, Vec::new()).map(|_| ())
}

fn pleur_sign_ongasp_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    return Err(Stop::End);
}

pub fn pleur_sign_ongasp(ctx: &Ctx) -> Script {
    pleur_sign_ongasp_body(ctx, Vec::new()).map(|_| ())
}

fn pleur_sign_onomg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
    return Err(Stop::End);
}

pub fn pleur_sign_onomg(ctx: &Ctx) -> Script {
    pleur_sign_onomg_body(ctx, Vec::new()).map(|_| ())
}

fn gordon_sign_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    ctx.mes("[Gordon]")?;
    if ctx.var("sign_q").get()?.number()? < 18 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
        ctx.lines(args!["Hello darling.", "What did you do today?"])?;
        ctx.next()?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Pleur#sign::OnHo")])?;
        ctx.lines_as(
            "Pleur",
            args!["I played Hide-and-Seek", "with a white bear and a", "blue bear, father."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
        ctx.lines_as(
            "Gordon",
            args![
                "Darling...",
                "Aren't you tired",
                "of playing with the",
                "animals? We've lived",
                "in this forest for so long..."
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Pleur#sign::OnKis2")])?;
        ctx.lines_as("Pleur", args!["Don't worry father, I understand. For now, this is the only place where we can relax and live inpeace. I think we deserve to rest after accomplishing our goals..."])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.lines_as(
            "Gordon",
            args![
                "Thank you, Pluer.",
                "I have no regrets about",
                "our work, but sometimes",
                "I do wish for a more",
                "carefree life for you..."
            ],
        )?;
        if ctx.var("sign_q").get()?.number()? < 17 {
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("sign_q").get()? == 17 {
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Roar~!:Excuse me.")])? {
                1 => {
                    ctx.var("sign_q").set(Val::from(18))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#skill::OnEnable")])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Pleur#sign::OnOMG")])?;
                    ctx.lines_as("Pleur", args!["No no no!", "Summon Flora!"])?;
                    ctx.next()?;
                    ctx.lines_as("Gordon", args!["Great Schott!", "Summon... Marine Sphere!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Pleur#sign::OnGasp")])?;
                    ctx.lines_as(
                        "Gordon",
                        args![
                            "Eh?! Don't you know",
                            "how dangerous this place",
                            "is? What are you doing",
                            "here in the middle of",
                            "this forest?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Actually, I think",
                            "I've been looking for",
                            "you. I've been sent on",
                            "an errand to find these",
                            "two famous Alchemists."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gordon",
                        args![
                            "Mm...?",
                            "Well, we're retired",
                            "now, but I suppose it'd",
                            "do no harm if you had",
                            "something to ask us..."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ask about Stone of Sage:Ask about Alchemy:Quit")])? {
                        1 => {
                            ctx.var("sign_q").set(Val::from(19))?;
                            ctx.lines_as(
                                "Gordon",
                                args![
                                    "Stone of Sage?",
                                    "Huh. To be honest,",
                                    "I don't know anything",
                                    "about it at all. I guess",
                                    "its existence is pretty",
                                    "much just a rumor, really."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gordon",
                                args![
                                    "All I've heard is that",
                                    "the Stone of Sage might",
                                    "be a catylst to transmute",
                                    "materials into gold. If it",
                                    "really existed, it would be",
                                    "the ultimate alchemic item."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pleur",
                                args![
                                    "However, I've also heard",
                                    "it's red, can make humans",
                                    "immortal and can cure any",
                                    "sort of disease or ailment.",
                                    "Just where do these rumors",
                                    "come from? It's crazy..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gordon",
                                args![
                                    "Now, I even hear that some",
                                    "people are working on trying",
                                    "to create the stone themselves.",
                                    "If they succeed, it'll have a huge effect on the entire world!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gordon",
                                args![
                                    "I'm sorry that you've gone",
                                    "through the trouble of finding",
                                    "us for this kind of information. We're retired after all, so we",
                                    "may be out of the loop."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pleur",
                                args![
                                    "Although we're retired, we",
                                    "would be much interested in",
                                    "knowing if someone does manage",
                                    "to create such a stone. If that",
                                    "happens, would you tell us?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gordon",
                                args![
                                    "Now let me guide you",
                                    "on a safe path back out",
                                    "of this maze. I've lived here",
                                    "quite a while, so I can find",
                                    "the exit with my eyes closed.",
                                    "Farewell, adventurer~"
                                ],
                            )?;
                            ctx.close_window()?;
                            ctx.call(Function::Warp, vec![Val::from("mjolnir_12"), Val::from(44), Val::from(23)])?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as("Pleur", args!["I hope you understand that", "my father and I devoted and sacrificed so much for our work. Finally, we discovered a way to summon monsters using potions."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Pleur",
                                args![
                                    "However, too many Alchemists",
                                    "hounded us for our information",
                                    "once we announced the results",
                                    "of our research. It was more",
                                    "than we could handle..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Gordon",
                                args![
                                    "I'm sorry, but if you have any",
                                    "questions about Alchemy, there",
                                    "are many qualified researchers and practitioners out there. We came to this forest to find peace..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        3 => {
                            ctx.lines_as(
                                "Gordon",
                                args!["Hah hah hah~", "Did you forget", "what you were", "going to ask me?"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    } else if ctx.var("sign_q").get()? == 18 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
        ctx.lines(args![
            "Oh...!",
            "You scared us!",
            "Roaring like some",
            "animal! What do you",
            "think you were doing?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args![
                "Actually, I think",
                "I've been looking for",
                "you. I've been sent on",
                "an errand to find these",
                "two famous Alchemists."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gordon",
            args![
                "Mm...?",
                "Well, we're retired",
                "now, but I suppose it'd",
                "do no harm if you had",
                "something to ask us..."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ask about Stone of Sage:Ask about Alchemy:Quit")])? {
            1 => {
                ctx.var("sign_q").set(Val::from(19))?;
                ctx.lines_as(
                    "Gordon",
                    args![
                        "Stone of Sage?",
                        "Huh. To be honest,",
                        "I don't know anything",
                        "about it at all. I guess",
                        "its existence is pretty",
                        "much just a rumor, really."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gordon",
                    args![
                        "All I've heard is that",
                        "the Stone of Sage might",
                        "be a catylst to transmute",
                        "materials into gold. If it",
                        "really existed, it would be",
                        "the ultimate alchemic item."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pleur",
                    args![
                        "However, I've also heard",
                        "it's red, can make humans",
                        "immortal and can cure any",
                        "sort of disease or ailment.",
                        "Just where do these rumors",
                        "come from? It's crazy..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gordon",
                    args![
                        "Now, I even hear that some",
                        "people are working on trying",
                        "to create the stone themselves.",
                        "If they succeed, it'll have a huge effect on the entire world!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gordon",
                    args![
                        "I'm sorry that you've gone",
                        "through the trouble of finding",
                        "us for this kind of information. We're retired after all, so we",
                        "may be out of the loop."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pleur",
                    args![
                        "Although we're retired, we",
                        "would be much interested in",
                        "knowing if someone does manage",
                        "to create such a stone. If that",
                        "happens, would you tell us?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gordon",
                    args![
                        "Now let me guide you",
                        "on a safe path back out",
                        "of this maze. I've lived here",
                        "quite a while, so I can find",
                        "the exit with my eyes closed.",
                        "Farewell, adventurer~"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("mjolnir_12"), Val::from(44), Val::from(23)])?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Pleur", args!["I hope you understand that", "my father and I devoted and sacrificed so much for our work. Finally, we discovered a way to summon monsters using potions."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Pleur",
                    args![
                        "However, too many Alchemists",
                        "hounded us for our information",
                        "once we announced the results",
                        "of our research. It was more",
                        "than we could handle..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Gordon",
                    args![
                        "I'm sorry, but if you have any",
                        "questions about Alchemy, there",
                        "are many qualified researchers and practitioners out there. We came to this forest to find peace..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Gordon",
                    args!["Hah hah hah~", "Did you forget", "what you were", "going to ask me?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("sign_q").get()?.number()? > 18 {
        ctx.lines(args![
            "Sometimes I miss",
            "being an alchemist.",
            "But then again, a man",
            "of my brilliance and genius",
            "would put all those newbie",
            "scientists to shame~"
        ])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
        ctx.lines_as("Pleur", args!["D...", "D...Daddy!!"])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.lines_as("Gordon", args!["Heh heh~", "Am I being", "too arrogant?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn gordon_sign(ctx: &Ctx) -> Script {
    gordon_sign_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum JesqurienneSignStep {
    Start,
    OnOmg,
    OnHo,
}

fn jesqurienne_sign_run(ctx: &Ctx, mut step: JesqurienneSignStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    'machine: loop {
        match step {
            JesqurienneSignStep::Start => {
                shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
                ctx.mes("[Jesqurienne]")?;
                if ctx.var("sign_q").get()?.number()? < 21 {
                    ctx.lines(args!["Bartender~?", "Give me another drink."])?;
                    if ctx.var("sign_q").get()?.number()? < 20 {
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.var("sign_q").get()? == 20 {
                        ctx.next()?;
                        ctx.lines_as("Jesqurienne", args!["Ahhhhhh~", "Hm? Why hello there,"])?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("you fine speciman of a man~")?;
                        } else {
                            ctx.mes("you precious, adorable girl~")?;
                        }
                        ctx.lines(args![
                            "I'm Jesqurienne. You've heard",
                            "of me, haven't you? Aren't you surprised to see me?"
                        ])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("I've never heard of you.:Oh gosh, it's Jesqurienne!")])? {
                            1 => {
                                ctx.mes("[Jesqurienne]")?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                                ctx.lines(args!["Ho ho ho~!", "Surely you", "must be joking~"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("You got me, you got me.:Nope. Doesn't ring a bell.")])? {
                                    1 => {
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                        ctx.lines_as(
                                            "Jesqurienne",
                                            args![
                                                "Ohohohohoho~!",
                                                "I knew it! I knew it!",
                                                "Then again, there's little",
                                                "that a famous genius like",
                                                "myself does ^333333not^000000 know!"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Jesqurienne",
                                            args!["Gah--!", "My glass is", "already empty?", "Bartender, another", "drink please~"],
                                        )?;
                                        ctx.next()?;
                                    }
                                    2 => {
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                        ctx.lines_as(
                                            "Jesqurienne",
                                            args![
                                                "...",
                                                "......",
                                                "Now I understand.",
                                                "You've been living",
                                                "under a rock for all",
                                                "this time, haven't you?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Jesqurienne",
                                            args![
                                                "But I guess someone",
                                                "as simple minded as you",
                                                "wouldn't recognize a famous,",
                                                "brilliant genius once you",
                                                "saw her, wouldn't you?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                            }
                            2 => {
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args![
                                        "Ohohohohoho~!",
                                        "I knew it! I knew it!",
                                        "Then again, there's little",
                                        "that a famous genius like",
                                        "myself does ^333333not^000000 know!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args!["Gah--!", "My glass is", "already empty?", "Bartender, another", "drink please~"],
                                )?;
                                ctx.next()?;
                            }
                            _ => {}
                        }
                        ctx.lines_as(
                            "Jesqurienne",
                            args![
                                "Anyway, I'm so proud",
                                "of my supreme intelligence!",
                                "I may be somewhat obsessive",
                                "about studying, but that's just another reason to admire me~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jesqurienne",
                            args![
                                "The Wizard and Sage",
                                "exams? No problem~",
                                "Even the Alchemist test",
                                "was fairly simple. So far",
                                "I haven't met anyone",
                                "smarter than me~"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("^333333Stuck-up chick.^000000:Take it easy on the drinks...")])? {
                            1 => {
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args![
                                        "What...?",
                                        "What did you say?",
                                        "No one has ever",
                                        "said anything like",
                                        "that to me before!",
                                        "H-how dare you..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args![
                                        "Prove that you can",
                                        "hold a candle to my",
                                        "superior intellect and",
                                        "I'll be willing to accept",
                                        "your little insult! It's time",
                                        "for a Quiz Challenge!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                    args![
                                        "Quiz... Challenge?",
                                        "Like a trivia game?",
                                        "But who's going to",
                                        "ask us the questions?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Aaron#sign::onSmile")])?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                                ctx.call(
                                    Function::Emotion,
                                    vec![
                                        ctx.constant("ET_SURPRISE")?,
                                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                    ],
                                )?;
                                ctx.lines_as(
                                    "Aaron",
                                    args!["Did someone say", "''Quiz Challenge?''", "I believe I can be", "of assistance."],
                                )?;
                                ctx.next()?;
                                ctx.var("sign_q").set(Val::from(21))?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args![
                                        "Professor Aaron?",
                                        "Ho ho~ If he's asking",
                                        "the questions, it looks",
                                        "like your chances of",
                                        "beating me are zero!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args![
                                        "Ho ho ho~",
                                        "Plenty of people have",
                                        "told me not to drink so",
                                        "much. But once I start",
                                        "I can't stop. I love to",
                                        "drink that much!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Jesqurienne",
                                    args![
                                        "And...",
                                        "Sometimes I'd rather",
                                        "look at the world through",
                                        "these hazy eyes. See things",
                                        "the way I want to see them..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                } else if (ctx.var("sign_q").get()? == 21 || ctx.var("sign_q").get()? == 22) {
                    ctx.lines(args![
                        "It amuses me that",
                        "you think that you'd",
                        "stand a chance against",
                        "me in a Quiz Challenge.",
                        "Ho ho ho ho ho~!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![
                            "Professor Aaron",
                            "is waiting for you",
                            "to ask him to begin.",
                            "And I'm waiting for the",
                            "moment when you realize",
                            "you have no hope of winning!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()? == 23 {
                    ctx.lines(args![
                        "Bwahahahaha!",
                        "You should have known",
                        "from the beginning that you",
                        "never had a chance! Of course,",
                        "I did, but you know, I guess you had to learn for yourself."
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                    ctx.lines_as("Jesqurienne", args!["I'd insult you if it weren't for the fact that you embarassed yourself enough during the Quiz Challenge. As a matter of fact, I'll even forgive you for wounding my pride earlier."])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_KIK")?])?;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![
                            "Can you believe it?",
                            "Even drunk I can beat",
                            "you in a test of intelligence!",
                            "Bwahaha! I can't believe how",
                            "simple minded you are...!"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("I can't let you win!:Darn it...")])? {
                        1 => {
                            ctx.var("sign_q").set(Val::from(22))?;
                            ctx.lines_as(
                                "Jesqurienne",
                                args![
                                    "You're a slow learner,",
                                    "aren't you? Well, I welcome",
                                    "your ''challenge'' anytime!",
                                    "Bwah hah hah hah hah~!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Jesqurienne",
                                args![
                                    "Just now you realize",
                                    "how pitiful your intellect",
                                    "is compared to mine? Ho ho ho!",
                                    "There's hope for every fool!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("sign_q").get()? == 24 {
                    ctx.lines(args!["What...?", "I... I...", "I don't b-believe it."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![
                            "I lost...",
                            "M-maybe you were right.",
                            "I am arrogant after all...",
                            "I should stop drinking so",
                            "much. Travel around and",
                            "broaden my knowledge..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![
                            "You're the only",
                            "one who has been",
                            "able to beat me in",
                            "a battle of minds.",
                            "W-who are you...?"
                        ],
                    )?;
                    ctx.next()?;
                    let (input, status) = runtime::input_text(ctx, None, None)?;
                    l_input_s = input;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![((Val::from("") + l_input_s.clone()) + Val::from("...")), "I will remember that."],
                    )?;
                    ctx.next()?;
                    ctx.mes("[Jesqurienne]")?;
                    ctx.var("sign_q").set(Val::from(25))?;
                    ctx.call(Function::GetItem, vec![Val::from(7177), Val::from(1)])?;
                    ctx.lines(args![
                        "Here...",
                        "My old friend, Metz, told",
                        "me to give this to someone",
                        "who is worthy. I was going",
                        "to keep it, but I suppose",
                        "you're its true owner..."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Jesqurienne", args!["If you want to know more about the Sobbing Starlight, find a man named ^3131CEDearles^000000. It may be hard to find him since he wanders from place to place, but it might help to know that he loves gambling..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![
                            "Good luck finding",
                            "Dearles. I'm taking",
                            "this loss pretty badly,",
                            "so I think I'll stay here",
                            "and have another drink.",
                            ((Val::from("Farewell, ") + l_input_s.clone()) + Val::from("."))
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_q").get()? == 25 {
                    ctx.lines(args![
                        "Ooooh...",
                        "My head...",
                        "Oh...! It's you...!",
                        "So did you find Dearles?"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jesqurienne",
                        args![
                            "Hmm...",
                            "He's addicted to",
                            "gambling, so maybe you",
                            "can find him some place",
                            "where you can do that..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args!["OoOooOhh~", "Sooooo dizzy.", "Maybe I drank", "too much again~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = JesqurienneSignStep::OnOmg;
                continue 'machine;
            }
            JesqurienneSignStep::OnOmg => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                return Err(Stop::End);
            }
            JesqurienneSignStep::OnHo => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn jesqurienne_sign(ctx: &Ctx) -> Script {
    jesqurienne_sign_run(ctx, JesqurienneSignStep::Start, Vec::new()).map(|_| ())
}

pub fn jesqurienne_sign_onomg(ctx: &Ctx) -> Script {
    jesqurienne_sign_run(ctx, JesqurienneSignStep::OnOmg, Vec::new()).map(|_| ())
}

pub fn jesqurienne_sign_onho(ctx: &Ctx) -> Script {
    jesqurienne_sign_run(ctx, JesqurienneSignStep::OnHo, Vec::new()).map(|_| ())
}
