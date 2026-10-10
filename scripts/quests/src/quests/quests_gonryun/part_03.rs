use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn blacksmith_sayummoon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 17 {
        ctx.lines_as("Aumgarl", args!["*Sob*..*sob*....", "My poor baby Lyroo..", "*Sniff*..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()?.number()? < 32 {
        'b1: {
            let subject1 = ctx.var("b_sword").get()?;
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(17))
                && !subject1.loosely_equals(&Val::from(18))
                && !subject1.loosely_equals(&Val::from(19))
                && !subject1.loosely_equals(&Val::from(20))
                && !subject1.loosely_equals(&Val::from(21))
                && !subject1.loosely_equals(&Val::from(22))
                && !subject1.loosely_equals(&Val::from(23))
                && !subject1.loosely_equals(&Val::from(24))
                && !subject1.loosely_equals(&Val::from(25))
                && !subject1.loosely_equals(&Val::from(26))
                && !subject1.loosely_equals(&Val::from(27))
                && !subject1.loosely_equals(&Val::from(28))
                && !subject1.loosely_equals(&Val::from(29))
                && !subject1.loosely_equals(&Val::from(30))
                && !subject1.loosely_equals(&Val::from(31));
            if !matched1 && subject1.loosely_equals(&Val::from(17)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Aumgarl", args!["*Sob*...", "My poor baby Lyroo..", "*Sniff*..."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Sir...?:...........")])?) == 1 {
                    ctx.lines_as(
                        "Aumgarl",
                        args!["Hmm...Who are you?", "Do you need something?", "If not, please leave..."],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("I came to repair this sword.:I heard you are a famous blacksmith...")],
                    )?) == 1
                    {
                        ctx.lines_as(
                            "Aumgarl",
                            args!["Umm...", "I'm sorry, but we're closed.", "Come again another time.", "Goodbye."],
                        )?;
                        ctx.var("b_sword").set(Val::from(18))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Aumgarl",
                        args![
                            "Yes, I am a blacksmith,",
                            "but I don't think I'm famous.",
                            "And unfortunately, we're closed right now."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Aumgarl",
                    args!["If you have nothing to say to me,", "then please leave me alone."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(18)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Aumgarl", args!["Like I said, we're closed!", "Come back later."])?;
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(8)])? == 7 {
                    ctx.var("b_sword").set(Val::from(19))?;
                    ctx.mes("Didn't you hear me?! Sheesh!")?;
                }
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(173), Val::from(169)])?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(19)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Aumgarl",
                    args!["Hmm...", "You're a persistent one!!", "What is it that you want?"],
                )?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "I want to marry your granddaughter.:Your granddaughter is such a pretty girl.:I came to repair the sword.",
                        )],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1))
                        && !subject2.loosely_equals(&Val::from(2))
                        && !subject2.loosely_equals(&Val::from(3));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                ".....",
                                ".........",
                                ".............",
                                "...................",
                                "........................"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Who the heck are you?",
                                "What do you want with my",
                                "granddaughter all of a sudden!?",
                                "Get the hell out of here!",
                                "Don't ever come back here again!"
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "You're darn right~",
                                "My granddaughter is soo~ pretty.",
                                "Hahaha...ah....hah...",
                                ".....*Sob, sob*...."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from("I want to marry her...:I came to repair the sword.:Anything wrong?")],
                        )? {
                            1 => {
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        ".....",
                                        ".........",
                                        ".............",
                                        "...................",
                                        "........................"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        "Who the heck are you?",
                                        "What do you want with my",
                                        "granddaughter all of a sudden!?",
                                        "Get out of here you pervert!",
                                        "Don't ever come back here again!"
                                    ],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        "...............",
                                        "A sword?",
                                        "I don't remember when the last",
                                        "time I worked with steel was.",
                                        "Hmm...",
                                        "May I see the sword?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Aumgarl", args!["Hmm......", "Hmmmm......", "............"])?;
                                ctx.next()?;
                                ctx.mes("^0000FFHe intently studied the pieces of the sword for a while.^000000")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        "Hmm...",
                                        "So this is the one, huh?",
                                        "Well...I am sorry, but this sword",
                                        "is not something I can repair",
                                        "at the moment."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aumgarl",
                                    args!["Why don't you go find", "another blacksmith?", "I am truly sorry."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            3 => {
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        "Um..well...",
                                        "This is not something I normally",
                                        "tell strangers, but....",
                                        "My granddaughter Lyroo has",
                                        "an incurable disease..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        "Not long after she was born,",
                                        "her parents died from an accident.",
                                        "Since that day, I have been taking care of her..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aumgarl",
                                    args![
                                        "But that wasn't the end of it...",
                                        "Soon after, she started to suffer",
                                        "from a serious illness. I've met",
                                        "doctors from all around",
                                        "Midgard..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Aumgarl", args!["But not one of them could", "diagnose her sickness..."])?;
                                ctx.next()?;
                                'b4: {
                                    let subject4 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("I am sorry to hear that.:I'll make her happy.:I want to cure her.")],
                                    )?);
                                    let mut matched4 = false;
                                    let no_case4 = !subject4.loosely_equals(&Val::from(1))
                                        && !subject4.loosely_equals(&Val::from(2))
                                        && !subject4.loosely_equals(&Val::from(3));
                                    if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                                        matched4 = true;
                                    }
                                    if matched4 {
                                        ctx.lines_as(
                                            "Aumgarl",
                                            args![
                                                "*Sigh*...",
                                                "I'll just have to accept it",
                                                "as her fate. I just feel",
                                                "so sorry for Lyroo.",
                                                "..."
                                            ],
                                        )?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                        matched4 = true;
                                    }
                                    if matched4 {
                                        ctx.lines_as(
                                            "Aumgarl",
                                            args![
                                                "Umm...",
                                                "What do you mean...?",
                                                "'Make her happy?'",
                                                "You're not making any sense...",
                                                "I'm confused..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from(
                                                "I'll make her happy all the time.:I want to take her with me.:I will cure her.",
                                            )],
                                        )? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Aumgarl",
                                                    args![
                                                        "...",
                                                        "What are you talking about?",
                                                        "Make her happy all the time?",
                                                        "She is dying even at",
                                                        "this moment."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Aumgarl",
                                                    args!["I think you're out of", "your mind, sicko!", "Get out of here!"],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Aumgarl",
                                                    args![
                                                        "...What do you mean?",
                                                        "You want to take her to where?",
                                                        "What are you gonna do?"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if Val::from(runtime::select_values(ctx, &[Val::from("I will cure her.:I am sorry.")])?)
                                                    == 1
                                                {
                                                    ctx.lines_as(
                                                        "Aumgarl",
                                                        args![
                                                            "It's useless talking about",
                                                            "it any longer. I don't think",
                                                            "there is anyone who can",
                                                            "cure my granddaughter."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Aumgarl",
                                                    args!["Ahh...", "I'm not blaming you.", "It's not your fault anyway.", "...*Sigh*..."],
                                                )?;
                                                ctx.next()?;
                                                if Val::from(runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Well...:I'll pray for her recovery.")],
                                                )?) == 1
                                                {
                                                    ctx.lines_as(
                                                        "Aumgarl",
                                                        args![
                                                            "Err..",
                                                            "I am sorry to ask you this...",
                                                            "But would you do me a favor?",
                                                            "Umm..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    if Val::from(runtime::select_values(ctx, &[Val::from("No.:Sure.")])?) == 1 {
                                                        ctx.lines_as(
                                                            "Aumgarl",
                                                            args!["Err...", "Alright, I won't bother you..", "Goodbye..."],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    ctx.lines_as(
                                                        "Aumgarl",
                                                        args![
                                                            "It won't be easy, but",
                                                            "would you help me to find",
                                                            "a doctor...No, anybody",
                                                            "who can cure Lyroo..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Aumgarl",
                                                        args![
                                                            "Anyone who has any information about her illness?? Please...",
                                                            "...*Sob*..."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    let choice = runtime::select_values(ctx, &[Val::from("Yes, Sir.")])?;
                                                    ctx.var("@menu").set(choice)?;
                                                    ctx.lines_as(
                                                        "Aumgarl",
                                                        args![
                                                            "Oh...",
                                                            "Thank you so much.",
                                                            "If Lyroo can get her health back,",
                                                            "I'll never forget your help."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Aumgarl", args!["Thank you...", "Thank you..."])?;
                                                    ctx.var("b_sword").set(Val::from(20))?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                ctx.lines_as(
                                                    "Aumgarl",
                                                    args![
                                                        "Well...thank you.",
                                                        "I'm going to try my best to",
                                                        "make her happy until the",
                                                        "very end..",
                                                        "Please come by sometime",
                                                        "to say hi to Lyroo."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Aumgarl",
                                                    args![
                                                        ".............",
                                                        "How're you going to cure",
                                                        "my granddaughter?",
                                                        "You don't look like",
                                                        "a doctor to me."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("I am a doctor.:I know a famous doctor.:I'll cure her no matter what.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as(
                                                            "Aumgarl",
                                                            args![
                                                                "Ehm...",
                                                                "Do I look like a fool to you?",
                                                                "I hate people who lie like that. Get out of my sight.",
                                                                "Get out of my sight.",
                                                                " "
                                                            ],
                                                        )?;
                                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                                                        ctx.close_window()?;
                                                        ctx.call(
                                                            Function::Warp,
                                                            vec![Val::from("geffen"), Val::from(173), Val::from(169)],
                                                        )?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as(
                                                            "Aumgarl",
                                                            args![
                                                                "A famous doctor?",
                                                                "I've met all kinds of",
                                                                "famous doctors. I've",
                                                                "probably met whoever",
                                                                "you may know already."
                                                            ],
                                                        )?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    3 => {
                                                        ctx.lines_as(
                                                            "Aumgarl",
                                                            args![
                                                                "................",
                                                                "You want to cure her",
                                                                "no matter what?",
                                                                "How're going to do that?",
                                                                "What if it turns worse,",
                                                                "Huh?"
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Aumgarl",
                                                            args!["I'm tired of your bragging.", "Please just leave us alone."],
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
                                    if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                                        matched4 = true;
                                    }
                                    if matched4 {
                                        ctx.lines_as(
                                            "Aumgarl",
                                            args![
                                                ".................",
                                                "Thanks for your concern...",
                                                "but you're a stranger to us.",
                                                "I can't let you do that.",
                                                "Thanks anyways."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                ".........",
                                "Umm...",
                                "I'm sorry, but I am",
                                "very tired right now.",
                                "Can't help you..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(20)) {
                matched1 = true;
            }
            if !matched1 && subject1.loosely_equals(&Val::from(21)) {
                matched1 = true;
            }
            if !matched1 && subject1.loosely_equals(&Val::from(22)) {
                matched1 = true;
            }
            if !matched1 && subject1.loosely_equals(&Val::from(23)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Aumgarl",
                    args![
                        "Hmmm...",
                        "Why don't you try looking",
                        "someplace that's always",
                        "crowded with people. I'm",
                        "sure there must be someone",
                        "out there who knows..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(24)) {
                matched1 = true;
            }
            if !matched1 && subject1.loosely_equals(&Val::from(25)) {
                matched1 = true;
            }
            if !matched1 && subject1.loosely_equals(&Val::from(26)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Aumgarl",
                    args![
                        "Ahh...",
                        "Lyroo is upstairs...",
                        "Bue she's in pain.",
                        "Try not to talk to her",
                        "for too long."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(27)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Aumgarl",
                    args![
                        "Oh my...",
                        "Thank you...",
                        "I never thought you'd",
                        "really help me.",
                        "Go ahead and see Lyroo."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(28)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Aumgarl", args!["Thank you so much...", "Without your help..."])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.mes("it would have been hopeless.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Aumgarl",
                    args![
                        "Alright....",
                        "Is there anything I can",
                        "do for you? I want to",
                        "return your favor somehow."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("No...I don't need anything.:Would you repair the sword for me?")],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Hmm...",
                                "Whenever you have a favor to ask,",
                                "come and find me. I'll do",
                                "my best to help you."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(29))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Hmm....",
                                "Show me this sword.",
                                "I need to take a look at it first",
                                "to find out whether I can repair it or not."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^0000FFYou Show the pieces of the",
                            "broken sword to Aumgarl.",
                            "....................",
                            "Aumgarl took a careful look",
                            "at the pieces for awhile...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "...", "...", "..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Hmm...",
                                "It's not possible to start work",
                                "immediately. I'll need",
                                "some materials."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "To repair this sword, I need",
                                "to restock my supplies.",
                                "Since Lyroo is upstairs in pain,",
                                "I don't think I can leave the house."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "So...could you get the",
                                "materials for me?",
                                "I am sorry I am asking",
                                "you to do this, as well."
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Umm...let's do it next time.:Alright, I'll go get them.")],
                        )?) == 1
                        {
                            ctx.lines_as(
                                "Aumgarl",
                                args![
                                    "Umm.....",
                                    "I am sorry I can't",
                                    "help you right away.",
                                    "Come back here when",
                                    "you need my help then."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Oh~~ really?",
                                "Thank you!",
                                "I'm sure I can repair it for you.",
                                "Now, listen carefully,",
                                "this is what I will need-"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "^FF00FF One Anvil",
                                "2 Rough Oridecons",
                                "5 Broken Swords",
                                "2 Steels",
                                "1 Hammer of Blacksmith",
                                "2 Star Crumbs",
                                "5 Live Coals^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "It's quite a lot, huh?",
                                "But these are the necessary",
                                "supplies we need to repair the sword.",
                                "I'll go find some other materials."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(30))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(29)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Aumgarl",
                    args!["Oh~ It's you.", "Got any favors to ask?", "I'll do my best", "to help you."],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Maybe next time...:Please repair this sword for me.")])? {
                    1 => {
                        ctx.lines_as(
                            "Aumgarl",
                            args!["Well...", "If you ever have a favor to", "ask, come and find me."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Hmm....",
                                "Show me this sword.",
                                "I need to take a look at it first",
                                "to find out whether I can repair it or not."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^0000FFYou show the pieces of the",
                            "broken sword to Aumgarl.",
                            "....................",
                            "Aumgarl took a careful look",
                            "at the pieces for a while...^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines(args!["...", "...", "...", "..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Hmm...",
                                "Well,",
                                "It's not possible to start work",
                                "immediately.",
                                "I need some materials."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "To repair this sword, I need",
                                "to restock my supplies.",
                                "Since Lyroo is upstairs in pain,",
                                "I don't think I can leave the house."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "So...could you get the",
                                "materials for me?",
                                "I am sorry I am asking",
                                "you to do this, as well."
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Umm..let's do it next time.:Alright, I'll go get them.")],
                        )?) == 1
                        {
                            ctx.lines_as(
                                "Aumgarl",
                                args![
                                    "Umm.....",
                                    "I am sorry I can't",
                                    "help you right away.",
                                    "Come back here when",
                                    "you need my help then."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "Oh~~ really?",
                                "Thank you!",
                                "I'm sure I can repair it for you.",
                                "Now, listen carefully,",
                                "this is what I will need-"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "^FF00FF One Anvil",
                                "2 Rough Oridecons",
                                "5 Broken Swords",
                                "2 Steels",
                                "1 Hammer of Blacksmith",
                                "2 Star Crumbs",
                                "5 Live Coals^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Aumgarl",
                            args![
                                "It's quite a lot, huh?",
                                "These are necessary supplies.",
                                "We need them to repair the sword.",
                                "I'll go find some other materials."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(30))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(30)) {
                matched1 = true;
            }
            if matched1 {
                if ((((((ctx.call(Function::CountItem, vec![Val::from(986)])?.number()? > 0
                    && ctx.call(Function::CountItem, vec![Val::from(756)])?.number()? > 1)
                    && ctx.call(Function::CountItem, vec![Val::from(7110)])?.number()? > 4)
                    && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 1)
                    && ctx.call(Function::CountItem, vec![Val::from(1005)])?.number()? > 0)
                    && ctx.call(Function::CountItem, vec![Val::from(1000)])?.number()? > 1)
                    && ctx.call(Function::CountItem, vec![Val::from(7098)])?.number()? > 4)
                {
                    ctx.lines_as(
                        "Aumgarl",
                        args![
                            "Oh, good! You got them all.",
                            "Alright, come back later,",
                            "and I'll have it repaired."
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(986), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(756), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7110), Val::from(5)])?;
                    ctx.call(Function::DelItem, vec![Val::from(999), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1005), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(1000), Val::from(2)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7098), Val::from(5)])?;
                    ctx.var("b_sword").set(Val::from(31))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Aumgarl",
                    args!["Hmm.....", "Haven't found everything yet?", "Here's the list again-"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aumgarl",
                    args![
                        "^FF00FF 1 Anvil",
                        "2 Rough Oridecons",
                        "5 Broken Swords",
                        "2 Steels",
                        "1 Hammer of Blacksmith",
                        "2 Star Crumbs",
                        "5 Live Coals^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Aumgarl", args!["Wrote them down?", "I've almost found the other materials."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(31)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 2 {
                    ctx.lines_as(
                        "Aumgarl",
                        args![
                            "Hey, you're back.",
                            "Here it is! This sword",
                            "is a great one, for",
                            "sure. I noticed that at",
                            "first sight.",
                            "Indeed..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Aumgarl",
                        args![
                            "I haven't seen quality",
                            "of this magnitude in a",
                            "in a long time.",
                            "I envy you...",
                            "Hehehe......"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["'You received repaired", "'^FF0000SaYumMoon's sword^000000'."])?;
                    ctx.var("b_sword").set(Val::from(32))?;
                    ctx.call(Function::GetItem, vec![Val::from(1123), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Aumgarl",
                    args![
                        "Umm... it's not done yet.",
                        "Would you give me a little",
                        "bit more time? It's taking",
                        "longer than I thought. Sorry."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    ctx.lines_as(
        "Aumgarl",
        args![
            "Thank you...",
            "You've been a great help",
            "to us. I hope my work",
            "has been of some use to you."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn blacksmith_sayummoon(ctx: &Ctx) -> Script {
    blacksmith_sayummoon_body(ctx, Vec::new()).map(|_| ())
}

fn girl_gnbs2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 20 {
        ctx.lines_as("Lyroo", args!["Ah...Ah....", "Ah...Hi......."])?;
        ctx.next()?;
        ctx.mes("^0000FFThis girl seems to be in serious pain...^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()?.number()? < 24 {
        ctx.lines_as(
            "Lyroo",
            args![
                "Aaa....Aaa....",
                "you...you....",
                "you are the....one...",
                "who will....cure....",
                "...me? Aa...."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Lyroo", args!["..tha... thank you...."])?;
        ctx.next()?;
        ctx.lines(args!["^0000FFBetter stop talking to her and", "hurry and find a cure.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ((ctx.var("b_sword").get()? == 24 || ctx.var("b_sword").get()? == 25) || ctx.var("b_sword").get()? == 26) {
        ctx.lines_as("Lyroo", args!["Ahh...ahh...."])?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
            ctx.mes("It's ..you.. pretty...sister..")?;
        } else {
            ctx.mes("It's ..you.. handsome...brother..")?;
        }
        ctx.mes("Ahh...heh heh...")?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Ask about her health condition.:Talk to her for a while.")],
        )?) == 1
        {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 3 {
                ctx.lines_as(
                    "Lyroo",
                    args![
                        "Uh....umm...?",
                        "Umm... I...",
                        "I can't talk...too long..",
                        "So...listen....",
                        "carefully..please..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lyroo",
                    args![
                        "^0030FFMy body...repeatedly turns cold and",
                        "hot... I'm not sweating at all...",
                        "....But I'm paralyzed...often",
                        "often..and...my heart beats...",
                        "...irregularly.^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Lyroo", args!["Ahh...hah~", "..........."])?;
                ctx.next()?;
                ctx.mes("^0000FFShe seems to have fallen asleep.^000000")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Lyroo", args!["I...am...so...", "...tired.", "............."])?;
            ctx.next()?;
            ctx.mes("^0000FFShe seems to have fallen asleep.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Lyroo",
            args!["Ahh........", "I..want to ...talk to...you..", "but.... sorry...."],
        )?;
        ctx.next()?;
        ctx.mes("^0000FFShe seems to have fallen asleep.^000000")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()? == 27 {
        ctx.lines_as("Lyroo", args!["uh...umm?"])?;
        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
            ctx.mes("Ah....It's you, pretty sister...")?;
        } else {
            ctx.mes("Ah....It's you, handsome brother...")?;
        }
        ctx.mes("Uhm? what is that you have?")?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("It's nothing.:It's your medicine.")])?) == 1 {
            ctx.lines_as("Lyroo", args!["Oh...", "Alright...", "..."])?;
            ctx.next()?;
            ctx.mes("^0000FFYou really should be giving her the medicine.^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Lyroo", args!["Ah!", "You found the medicine~!", "I'm...so relieved..."])?;
        ctx.next()?;
        ctx.lines(args![
            "^0000FFYou gave her the medicine you",
            "received from Cylrnel.",
            "After taking the medicine, Lyroo",
            "fell asleep.^000000"
        ])?;
        ctx.call(Function::DelItem, vec![Val::from(606), Val::from(1)])?;
        ctx.var("b_sword").set(Val::from(28))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Lyroo", args!["Heehehe..."])?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
        ctx.mes("Thank you my pretty sister.")?;
    } else {
        ctx.mes("Thank you handsome brother~")?;
    }
    ctx.lines(args!["I'll get my health back", "and be a strong girl!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn girl_gnbs2(ctx: &Ctx) -> Script {
    girl_gnbs2_body(ctx, Vec::new()).map(|_| ())
}

fn young_man_sayummoon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 20 {
        ctx.lines_as(
            "Neil",
            args![
                "Oh man, it feels so good",
                "to be out of the hospital...",
                "I thought I was going to be",
                "there forever~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Neil",
            args![
                "Listen guy, appreciate",
                "your health, 'cuz almost",
                "nothing is worse than",
                "being really really sick..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Neil",
        args![
            "Not long ago, I was in the",
            "hospital, stuck lying in bed...",
            "No one knew what was wrong",
            "with me...but I grew weaker",
            "everyday..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Neil",
        args![
            "But then, this ^FF5000doctor from Juno^000000",
            "arrived, and she helped me",
            "recover in no time!",
            "Though...I still can't walk so well."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Neil",
        args![
            "All the other doctors had said",
            "my case was hopeless, and that",
            "I'd never recover. Luckily,",
            "this Juno doctor was able to",
            "find a cure..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Neil", args!["It was a miracle!", "I'm so happy to be alive!"])?;
    if ctx.var("b_sword").get()? == 20 {
        ctx.var("b_sword").set(Val::from(21))?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn young_man_sayummoon(ctx: &Ctx) -> Script {
    young_man_sayummoon_body(ctx, Vec::new()).map(|_| ())
}

fn active_little_girl_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 21 {
        ctx.lines_as(
            "Hisa",
            args![
                ".....",
                "This town is so boring.",
                "No events, no festivals...",
                "All people do all day is stay at",
                "at home and study. How dull!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Hisa",
        args![
            "There's one person in Juno that",
            "I still respect. Her name is ^FF5500Cylrnel^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hisa",
        args![
            "She's an expert in medical",
            "treatment. Whenever someone falls",
            "ill in this town, she comes to",
            "cure that person right away..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hisa",
        args![
            "She also tells me lots of",
            "interesting stories about",
            "her experiences. She's even",
            "made a journey around the",
            "world... It's so interesting!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hisa",
        args!["She's probably at home after", "coming back from some trip.", "Haha~"],
    )?;
    if ctx.var("b_sword").get()? == 21 {
        ctx.var("b_sword").set(Val::from(22))?;
    }
    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])? == 2 {
        ctx.next()?;
        ctx.lines_as(
            "Hisa",
            args![
                "Oh right~!",
                "She's also known to be a fickle",
                "and forgetful woman. So you'd better watch out, hehe~"
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn active_little_girl_gnbs(ctx: &Ctx) -> Script {
    active_little_girl_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn doctor_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sick = Val::from(0);
    let mut l_sick1_s = Val::from("");
    let mut l_sick2_s = Val::from("");
    let mut l_sick3_s = Val::from("");
    let mut l_sick4_s = Val::from("");
    if ctx.var("b_sword").get()?.number()? < 23 {
        ctx.lines_as(
            "??????",
            args![
                "I don't believe we've met",
                "before, but would you mind",
                "coming back later? I've got a lot of work to do at the moment."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()?.number()? < 27 {
        let subject1 = ctx.var("b_sword").get()?;
        if subject1 == 23 {
            ctx.lines_as(
                "Cylrnel",
                args!["Hello, can I help you?", "If it's not urgent,", "please come back later."],
            )?;
            ctx.next()?;
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 2 {
                ctx.lines_as("Cylrnel", args!["Hmm...you look like you", "want to ask me something?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("You're so beautiful.:Do you like traveling?")],
                )?) == 1
                {
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])? == 4 {
                        ctx.lines_as(
                            "Cylrnel",
                            args![
                                "...excuse me?",
                                "Ah ha ha~",
                                "You're a funny guy.",
                                "Trying to hit on me?",
                                "Ah ha ha ha."
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Yes, I fell in love at first sight.:I have a favor to ask.")],
                        )?) == 1
                        {
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 3 {
                                ctx.lines_as(
                                    "Cylrnel",
                                    args![
                                        "HAHAHAHHAHAHAHAH!",
                                        "Ah...I'm sorry..",
                                        "I haven't seen a guy like you",
                                        "for a long time."
                                    ],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cylrnel",
                                    args![
                                        "You make me laugh...",
                                        "But guess what?",
                                        "I already know you're not",
                                        "here to hit on me.",
                                        "So what did you really want?"
                                    ],
                                )?;
                                ctx.next()?;
                                let choice = runtime::select_values(ctx, &[Val::from("Well, actually...")])?;
                                ctx.var("@menu").set(choice)?;
                                ctx.lines(args![
                                    "^FF0000You Tell Cylrnel about Lyroo,",
                                    "and about the favor for Aumgarl the blacksmith.^000000"
                                ])?;
                                ctx.next()?;
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                                    ctx.lines_as(
                                        "Cylrnel",
                                        args![
                                            "Hmm.....",
                                            "So that's what happened...",
                                            "Well, I need to know the",
                                            "exact symptoms...go and",
                                            "find out for me and then",
                                            "come back afterwards."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Cylrnel",
                                        args!["Without the exact symptoms, I can't make an accurate diagnosis."],
                                    )?;
                                    ctx.var("b_sword").set(Val::from(24))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Cylrnel", args!["Well now...", "That's quite a long story."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cylrnel",
                                    args![
                                        "I have no idea why you're",
                                        "trying to help these people,",
                                        "so it's a bit hard to believe",
                                        "you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Cylrnel",
                                args![
                                    "Oh brother!",
                                    "I never thought someone",
                                    "like you could still exist.",
                                    "I'm sorry, but you're",
                                    "really not my type."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 3 {
                            ctx.lines_as("Cylrnel", args!["Is that so?", "well then,", "Let me hear your story."])?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Well, it's like this...")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines(args![
                                "^FF0000You Tell Cylrnel about Lyroo,",
                                "and about the favor of Aumgarl the blacksmith.^000000"
                            ])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 2 {
                                ctx.lines_as(
                                    "Cylrnel",
                                    args![
                                        "Hmm.....",
                                        "So that's what happened...",
                                        "Well, I need to know the",
                                        "exact symptoms...go and",
                                        "find out for me and then",
                                        "come back afterwards."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cylrnel",
                                    args!["Without the exact symptoms, I can't make an accurate diagnosis."],
                                )?;
                                ctx.var("b_sword").set(Val::from(24))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Cylrnel", args!["Well now...", "That's quite a long story."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cylrnel",
                                args![
                                    "I have no idea why you're",
                                    "trying to help these people,",
                                    "so it's a bit hard to believe",
                                    "you..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Cylrnel",
                            args![
                                "Well...",
                                "While I'd like to help,",
                                "You're a stranger, and the",
                                "residents need my help.",
                                "I don't have any extra time",
                                "to help you. Please leave."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(246), Val::from(143)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Cylrnel",
                        args!["Ahahaha..", "You're a funny guy.", "But, that won't", "work on me!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "Err...well, I do, but...",
                        "If you don't have any",
                        "favors to ask, please leave.",
                        "I'm very busy right now."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Cylrnel", args!["I'm sorry, but I don't have", "any time for you right now."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 24 {
            ctx.lines_as(
                "Cylrnel",
                args![
                    "Oh, you're back...",
                    "Did you figure out the symptoms?",
                    "I wanted to go with you, but",
                    "I've been really busy."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Not yet...:Yes, Here.")])?) == 1 {
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "What! Why not!?",
                        "Hurry to her house!",
                        "She's in serious pain!",
                        "Her body could be paralyzed",
                        "at any moment!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Cylrnel",
                args![
                    "What a relief...",
                    "You're earlier than I thought.",
                    "I'll ask you some questions about",
                    "her condition. Answer correctly."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cylrnel", args!["First, how was her body temperature?"])?;
            ctx.var("sick").set(Val::from(0))?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Very hot.:Very cold.:Normal.:Turns hot and cold repeatedly.")])? {
                1 => {
                    l_sick1_s = Val::from("Very hot.");
                }
                2 => {
                    l_sick1_s = Val::from("Very cold.");
                }
                3 => {
                    l_sick1_s = Val::from("Normal.");
                }
                4 => {
                    l_sick1_s = Val::from("Turns hot and cold repeatedly");
                    l_sick = (l_sick.clone() + Val::from(1));
                }
                _ => {}
            }
            ctx.lines_as("Cylrnel", args!["Okay...", "Now, about physiological condition."])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "No sweating at all.:Sweating all over the body.:Runny nose.:No tears, sweats at all.",
                )],
            )? {
                1 => {
                    l_sick2_s = Val::from("No sweating at all.");
                    l_sick = (l_sick.clone() + Val::from(1));
                }
                2 => {
                    l_sick2_s = Val::from("Sweating all over the body.");
                }
                3 => {
                    l_sick2_s = Val::from("Runny nose.");
                }
                4 => {
                    l_sick2_s = Val::from("No tears, sweats at all.");
                }
                _ => {}
            }
            ctx.lines_as("Cylrnel", args!["Next, tell me about", "her physical condition."])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Nothing in particular.:Becomes paralyzed often.:Muscles became soft.:Muscles became hard.",
                )],
            )? {
                1 => {
                    l_sick3_s = Val::from("Nothing in particular.");
                }
                2 => {
                    l_sick3_s = Val::from("Becomes paralyzed often.");
                    l_sick = (l_sick.clone() + Val::from(1));
                }
                3 => {
                    l_sick3_s = Val::from("Muscles became soft.");
                }
                4 => {
                    l_sick3_s = Val::from("Muscles became hard.");
                }
                _ => {}
            }
            ctx.lines_as("Cylrnel", args!["How about internal organs?"])?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Stomach hurts like it's been stabbed by a knife.:Head hurts like it's been smashed by a hammer.:Heart beats irregularly.:Has difficulty in breathing.",
                )],
            )? {
                1 => {
                    l_sick4_s = Val::from("Stomach hurts like it's been stabbed by a knife.");
                }
                2 => {
                    l_sick4_s = Val::from("Head hurts like it's been smashed by a hammer.");
                }
                3 => {
                    l_sick4_s = Val::from("Heart beats irregularly.");
                    l_sick = (l_sick.clone() + Val::from(1));
                }
                4 => {
                    l_sick4_s = Val::from("Has difficulty in breathing.");
                }
                _ => {}
            }
            ctx.lines_as(
                "Cylrnel",
                args!["Hmm.. alright.", "Let me check this.", "so the symptoms are", ".............."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cylrnel",
                args![
                    ((Val::from("^FF0000") + l_sick1_s.clone()) + Val::from("")),
                    ((Val::from("") + l_sick2_s.clone()) + Val::from("")),
                    ((Val::from("") + l_sick3_s.clone()) + Val::from("")),
                    ((Val::from("") + l_sick4_s.clone()) + Val::from("^000000")),
                    "right?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("No..not exactly..:Yes, I am sure.")])?) == 1 {
                ctx.lines_as(
                    "Cylrnel",
                    args!["Then go back to her and", "find the exact symptoms", "right away~!!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Cylrnel",
                args![
                    "You're sure about this, right?",
                    "If they're the wrong symptoms,",
                    "I can't be responsible."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Err..let me go and double check.:I'm sure.")],
            )?) == 1
            {
                ctx.lines_as(
                    "Cylrnel",
                    args!["Then, go back to her and", "find the exact symptoms", "right away~!!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Cylrnel",
                args![
                    "Hmm.....",
                    "Aright. I'll trust you.",
                    "Now, go get these ingredients.",
                    "Get them as fast as possible.",
                    "Alright?"
                ],
            )?;
            ctx.next()?;
            if l_sick.clone() == 4 {
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "^FF00002 Yggdrasil Seeds",
                        "3 Aloes",
                        "1 Witherless Rose",
                        "10 Witch Starsands",
                        "5 Burning Hearts",
                        "5 Ice Cubics"
                    ],
                )?;
                ctx.var("b_sword").set(Val::from(26))?;
                ctx.next()?;
                ctx.lines_as("Cylrnel", args!["Wrote them down?", "I'll tell you once again.", "We need..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "^FF00002 Yggdrasil Seeds",
                        "3 Aloes",
                        "1 Witherless Rose",
                        "10 Witch Starsands",
                        "5 Burning Hearts",
                        "5 Ice Cubics"
                    ],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "^FF00001 Yggdrasil Seed",
                        "1 Aloe",
                        "5 Witch Starsands",
                        "3 Burning Hearts",
                        "3 Ice Cubics"
                    ],
                )?;
                ctx.var("b_sword").set(Val::from(25))?;
                ctx.next()?;
                ctx.lines_as("Cylrnel", args!["Wrote them down?", "I'll tell you once again.", "We need..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "^FF00001 Yggdrasil Seed",
                        "1 Aloe",
                        "5 Witch Starsands",
                        "3 Burning Hearts",
                        "3 Ice Cubics"
                    ],
                )?;
                ctx.next()?;
            }
            ctx.lines_as(
                "Cylrnel",
                args!["Get them as fast as possible.", "There isn't much time for Lyroo."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 25 {
            if (((ctx.call(Function::CountItem, vec![Val::from(608)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(704)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(1061)])?.number()? > 2)
                && ctx.call(Function::CountItem, vec![Val::from(7066)])?.number()? > 2)
            {
                ctx.lines_as(
                    "Cylrnel",
                    args!["Hmm...", "Good, you got them all.", "Let me see now...", "............."],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^FF0000Cylrnel began to mix",
                    "the ingredients.",
                    "................",
                    ".............",
                    ".........",
                    "......^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "............",
                        "What the?!?!",
                        "...",
                        "Hey you...",
                        "You gave me the wrong information!"
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(608), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(704), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(1061), Val::from(5)])?;
                ctx.call(Function::DelItem, vec![Val::from(7097), Val::from(3)])?;
                ctx.call(Function::DelItem, vec![Val::from(7066), Val::from(3)])?;
                ctx.var("b_sword").set(Val::from(24))?;
                ctx.next()?;
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "I've chosen the ingredients",
                        "according to the symptoms",
                        "you've described...",
                        "But this isn't medicine!!",
                        "Now go and find out her exact symptoms!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(246), Val::from(143)])?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Cylrnel",
                args![
                    "...",
                    "Hey~ you don't have all",
                    "the ingredients yet.",
                    "Go get them all right away."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cylrnel",
                args![
                    "^FF00001 Seed of Yggdrasil",
                    "1 Aloe",
                    "5 Witch Starsands",
                    "3 Burning Hearts",
                    "3 Ice Cubics"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cylrnel", args!["Alright?", "Now, hurry up!"])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(246), Val::from(143)])?;
            return Err(Stop::End);
        } else if subject1 == 26 {
            if (((((ctx.call(Function::CountItem, vec![Val::from(608)])?.number()? > 1
                && ctx.call(Function::CountItem, vec![Val::from(704)])?.number()? > 2)
                && ctx.call(Function::CountItem, vec![Val::from(748)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(1061)])?.number()? > 9)
                && ctx.call(Function::CountItem, vec![Val::from(7097)])?.number()? > 4)
                && ctx.call(Function::CountItem, vec![Val::from(7066)])?.number()? > 4)
            {
                ctx.lines_as(
                    "Cylrnel",
                    args!["Hmm...", "Good, you got them all.", "Let me see..", "............."],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^FF0000Cylrnel began to mix",
                    "the ingredients.",
                    "................",
                    ".............",
                    ".........",
                    "......^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "Here~! It's done...",
                        "I don't think this medicine will",
                        "cure her disease completely.",
                        "However, it will greatly alleviate her pain."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cylrnel",
                    args![
                        "It wasn't easy to make this",
                        "medicine. Take care, bring",
                        "this to Lyroo and give her my regards..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(608), Val::from(2)])?;
                ctx.call(Function::DelItem, vec![Val::from(704), Val::from(3)])?;
                ctx.call(Function::DelItem, vec![Val::from(748), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(1061), Val::from(10)])?;
                ctx.call(Function::DelItem, vec![Val::from(7097), Val::from(5)])?;
                ctx.call(Function::DelItem, vec![Val::from(7066), Val::from(5)])?;
                ctx.var("b_sword").set(Val::from(27))?;
                ctx.call(Function::GetItem, vec![Val::from(606), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Cylrnel",
                args![
                    "...",
                    "Hey~ You don't have all",
                    "the ingredients yet.",
                    "Go get them all right away!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cylrnel",
                args![
                    "^FF00002 Yggdrasil Seeds",
                    "3 Aloes",
                    "1 Witherless Rose",
                    "10 Witch Starsands",
                    "5 Burning Hearts",
                    "5 Ice Cubics"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cylrnel", args!["Alright?", "Go get them as fast as possible."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(246), Val::from(143)])?;
            return Err(Stop::End);
        }
    }
    ctx.lines_as(
        "Cylrnel",
        args![
            "Hey there...",
            "How is she doing?",
            "Getting better?",
            "By taking that medicine,",
            "she'll recover fast."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cylrnel",
        args![
            "Tell her to come to me",
            "sometime to get medical",
            "treatment. Walking from her",
            "house to here would be good",
            "exercise too."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Cylrnel",
        args![
            "The name of her disease is",
            "called '^FF0000Amarhade^000000'.",
            "It's a rare, relatively",
            "unknown illness, so an exact",
            "treatment hasn't been found yet."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Cylrnel", args!["But I found a treatment...", "And...it's..a...secret! Haha~!"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn doctor_gnbs(ctx: &Ctx) -> Script {
    doctor_gnbs_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GoAwayGnbsStep {
    Start,
    OnTouch,
}

fn go_away_gnbs_run(ctx: &Ctx, mut step: GoAwayGnbsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GoAwayGnbsStep::Start => {
                step = GoAwayGnbsStep::OnTouch;
                continue 'machine;
            }
            GoAwayGnbsStep::OnTouch => {
                if ctx.var("b_sword").get()?.number()? < 22 {
                    ctx.lines_as("??????", args!["^FF0000What are you doing here?", "Get out of here!^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(246), Val::from(143)])?;
                    return Err(Stop::End);
                } else if ctx.var("b_sword").get()? == 22 {
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])? == 10 {
                        ctx.lines_as("??????", args!["I am very busy right now. Please leave."])?;
                        ctx.var("b_sword").set(Val::from(23))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("??????", args!["^FF0000What are you doing here?", "Get out of here!^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(246), Val::from(143)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn go_away_gnbs(ctx: &Ctx) -> Script {
    go_away_gnbs_run(ctx, GoAwayGnbsStep::Start, Vec::new()).map(|_| ())
}

pub fn go_away_gnbs_ontouch(ctx: &Ctx) -> Script {
    go_away_gnbs_run(ctx, GoAwayGnbsStep::OnTouch, Vec::new()).map(|_| ())
}
