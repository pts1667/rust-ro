use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn ordinary_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !(ctx.var("lhz_boss").get()?.is_true()) {
        ctx.lines_as(
            "Ghalstein",
            args![
                "Hello there~",
                "Isn't Lighthalzen such",
                "a wonder to behold with",
                "all of its splendor and",
                "magnificent beauty?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("I guess."), Val::from("Yes, it is."), Val::from("Well, I don't know.")],
        )? {
            1 => {
                ctx.lines_as(
                    "Ghalstein",
                    args![
                        "Oh, and the weather is",
                        "so nice today, perfect",
                        "for a stroll in the park",
                        "or a lovely picnic. Ah~"
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Ghalstein",
                    args![
                        "Oh, I'm so glad you",
                        "agree. The people who",
                        "live here are so kind and",
                        "so happy, you can literally",
                        "feel everyone's gentle warmth."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Ghalstein",
                    args![
                        "Well, maybe if you",
                        "don't see it right this",
                        "moment, I'm sure you'll",
                        "discover something to",
                        "love about this city soon."
                    ],
                )?;
            }
            _ => {}
        }
        ctx.next()?;
        if ctx.var("BaseLevel").get()?.number()? < 60 {
            ctx.lines_as("Ghalstein", args!["Ha ha ha!", "Anyway, I hope you", "have a good day today~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Ghalstein",
            args![
                "Yes, Lighthalzen",
                "couldn't be perfecter.",
                "Ahahahaha... But still...",
                "Hm. Um. Oh, never mind."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Ghalstein", args!["Ha ha ha!", "Anyway, I hope you", "have a good day today~"])?;
        ctx.var("lhz_boss").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("lhz_boss").get()? == 1 {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])? == 4 {
                ctx.lines_as(
                    "Ghalstein",
                    args![
                        "Hm? It's a little",
                        "cloudy today, isn't it?",
                        "It would be nice to stay",
                        "inside, listening to some",
                        "nice music while enjoying",
                        "a hot cup of tea, wouldn't it?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Ghalstein",
                args![
                    "Ahhh!",
                    "Such pleasant weather~",
                    "I'm so glad I'm outside",
                    "and enjoying the sunlight.",
                    "Feel it. It's so warm and",
                    "nice on your face, isn't it?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("lhz_boss").get()? == 2 {
                ctx.lines_as(
                    "Ghalstein",
                    args![
                        "The sun is gently",
                        "shining, the breeze is",
                        "gently blowing. Oh, this",
                        "weather could not be better!"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("Yes, you're right."), Val::from("Uh, what were you doing just now?")],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "Yes, right now is the",
                                "best time to be outside",
                                "and to enjoy all of this",
                                "fresh, wonderful air. Come,",
                                "take a deep breath with me.",
                                "Ooooh. Ahhhhhh~ Excellent!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "...",
                                "......",
                                "I don't quite follow what",
                                "you're saying. I was merely",
                                "enjoying the sunny perfection",
                                "of Lighthalzen as usual..."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("But I heard you talking about something...")])? {
                            1 => {}
                            _ => {}
                        }
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "No! You must",
                                "have mistaken me",
                                "for somebody else!",
                                "Now, if you'll excuse me..."
                            ],
                        )?;
                        ctx.var("lhz_boss").set(Val::from(3))?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFEven if it wasn't",
                            "Ghalstein that you",
                            "just heard, he reacted",
                            "pretty strongly to what",
                            "you said to him. Almost",
                            "as if he had something to hide... ^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                if ctx.var("lhz_boss").get()? == 3 {
                    ctx.lines_as(
                        "Ghalstein",
                        args![
                            "Hm? Judging from the",
                            "look on your face, you",
                            "apparently still believe",
                            "that I was involved in some",
                            "kind of clandestine meeting."
                        ],
                    )?;
                    ctx.next()?;
                    'b4: {
                        let subject4 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Yes, I'm sure of it!"), Val::from("Well...")],
                        )?);
                        let mut matched4 = false;
                        let no_case4 = !subject4.loosely_equals(&Val::from(1)) && !subject4.loosely_equals(&Val::from(2));
                        if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                            matched4 = true;
                        }
                        if matched4 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "Yes, I'm sure of it!",
                                    "That was definitely you!",
                                    "Your voice, your way of",
                                    "speaking is unmistakable!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Ghalstein", args!["......", "..........", "..............."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "Alright. Yes, it's",
                                    "true that I did meet",
                                    "someone in secret. But",
                                    "what gives you the right",
                                    "to interrogate me in this",
                                    "rather aggressive fashion?"
                                ],
                            )?;
                            ctx.next()?;
                            'b5: {
                                let subject5 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("I'm not interrogating..."), Val::from("Sorry about that.")],
                                )?);
                                let mut matched5 = false;
                                let no_case5 = !subject5.loosely_equals(&Val::from(1)) && !subject5.loosely_equals(&Val::from(2));
                                if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                    matched5 = true;
                                }
                                if matched5 {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args!["I'm not interrogating", "you. No, not at all.", "It's, um, it's just..."],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("Who are you dealing with?"), Val::from("I was just curious.")],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Who are you dealing",
                                                    "with? I heard something",
                                                    "about some organization",
                                                    "that's supposedly very",
                                                    "powerful and I'm starting",
                                                    "to get really concerned."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "I don't think I can",
                                                    "ignore hearing about",
                                                    "any kind of dangerous",
                                                    "corporation, even if",
                                                    "I wanted to. Turning my",
                                                    "back to this seems wrong..."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Ghalstein", args!["Hm...", "You really", "feel that way?"])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Ghalstein",
                                                args![
                                                    "Please give me",
                                                    "a moment. I need",
                                                    "some time to think",
                                                    "about how I can best",
                                                    "handle this situation."
                                                ],
                                            )?;
                                            ctx.var("lhz_boss").set(Val::from(4))?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Ghalstein",
                                                args![
                                                    "Just curious...?",
                                                    "Well, I hope that",
                                                    "you don't interfere",
                                                    "too much with other",
                                                    "people's lives just",
                                                    "out of curiosity."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Ghalstein",
                                                args![
                                                    "You really shouldn't",
                                                    "make something your",
                                                    "business if you're not",
                                                    "interested or willing",
                                                    "to commit. Now, if you",
                                                    "would excuse me..."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                    matched5 = true;
                                }
                                if matched5 {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "Sorry about that.",
                                            "I didn't mean to",
                                            "stick my nose into",
                                            "your personal business."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "Well, that's quite",
                                            "alright. I accept",
                                            "your apology, but",
                                            "I must warn you",
                                            "not to pry into my",
                                            "personal affairs."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                        if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                            matched4 = true;
                        }
                        if matched4 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["Well...", "Now I don't know.", "I suppose I could", "be wrong, I guess."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    ".......",
                                    "Yes, I wouldn't",
                                    "recommend running",
                                    "around, indiscriminately",
                                    "accusing random people",
                                    "of doing strange things."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    if ctx.var("lhz_boss").get()? == 4 {
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "I've come into a",
                                "decision and found",
                                "a way that I can let",
                                "you into my confidence",
                                "without putting the efforts",
                                "of my organization at risk."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "As you may have already",
                                "guessed, I'm a member of an",
                                "organization whose ultimate",
                                "goal is to put an end to the",
                                "Rekenber Corporation."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "However, before I relate",
                                "more information regarding",
                                "my organization and its work, I shall require your cooperation."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("How can I cooperate?")])? {
                            1 => {}
                            _ => {}
                        }
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["How can I cooperate?", "Um, what exactly is it", "that you want me to do?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ghalstein",
                            args![
                                "You do seem interested",
                                "in knowing more about us,",
                                "and we can also use your",
                                "expertise as an adventurer.",
                                "But first you must decide if",
                                "you will commit to cooperation."
                            ],
                        )?;
                        ctx.next()?;
                        'b8: {
                            let subject8 = Val::from(runtime::select_values(
                                ctx,
                                &[
                                    Val::from("I still don't get it..."),
                                    Val::from("Sure, I'll cooperate."),
                                    Val::from("No, thanks."),
                                ],
                            )?);
                            let mut matched8 = false;
                            let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                && !subject8.loosely_equals(&Val::from(2))
                                && !subject8.loosely_equals(&Val::from(3));
                            if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                matched8 = true;
                            }
                            if matched8 {
                                ctx.lines_as(
                                    "Ghalstein",
                                    args!["Alright, let me make this ", "clearer for you. You have", "two choices."],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Two choices?")])? {
                                    1 => {}
                                    _ => {}
                                }
                                ctx.lines_as(
                                    "Ghalstein",
                                    args![
                                        " If you choose",
                                        "to cooperate with us, you'll",
                                        "need to periodically drink",
                                        "this red magic tonic."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ghalstein",
                                    args![
                                        "The red tonic will allow",
                                        "us to monitor your every",
                                        "move so that we know you",
                                        "won't betray us. You'll have to",
                                        "continue cooperating or else",
                                        "we won't give you the cure."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ghalstein",
                                    args![
                                        "Throughout our mission,",
                                        "you'll need to take a red",
                                        "tonic every so often to avoid",
                                        "suffering fron unpleasant",
                                        "consequences. When we",
                                        "finish, you get the antidote."
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from(".........")])? {
                                    1 => {}
                                    _ => {}
                                }
                                ctx.lines_as(
                                    "Ghalstein",
                                    args![
                                        "If you choose not to",
                                        "cooperate, simply drink",
                                        "this blue tonic, which will",
                                        "erase every fragment of your",
                                        "memory regarding our meeting."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Ghalstein", args!["So, what have", "you decided to do?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Don't cooperate"), Val::from("Cooperate")])? {
                                    1 => {}
                                    2 => {
                                        ctx.lines_as("Ghalstein", args!["Ah, good choice.", "So will you take", "this red tonic?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Do not take it."), Val::from("Take it.")])? {
                                            1 => {}
                                            2 => {
                                                ctx.lines_as("Ghalstein", args!["Very good.", "Then please,", "drink this right away."])?;
                                                ctx.next()?;
                                                ctx.lines(args![
                                                    "^3355FFYou drink the red tonic.",
                                                    "It doesn't feel any different",
                                                    "from a Red Potion, but it",
                                                    "would be wise to cooperate",
                                                    "with Ghalstein until this",
                                                    "mission is accomplished.^000000"
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Ghalstein",
                                                    args![
                                                        "You may feel a little",
                                                        "disoriented after drinking",
                                                        "this tonic for the first time.",
                                                        "I'll give you a moment to",
                                                        "steady yourself before we",
                                                        "talk about our mission."
                                                    ],
                                                )?;
                                                ctx.var("lhz_boss").set(Val::from(5))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                                break 'b8;
                            }
                            if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                matched8 = true;
                            }
                            if matched8 {
                                ctx.lines_as("Ghalstein", args!["Ah, good choice.", "So will you take", "this red tonic?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Do not take it."), Val::from("Take it.")])? {
                                    1 => {}
                                    2 => {
                                        ctx.lines_as("Ghalstein", args!["Very good.", "Then please,", "drink this right away."])?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "^3355FFYou drink the red tonic.",
                                            "It doesn't feel any different",
                                            "from a Red Potion, but it",
                                            "would be wise to cooperate",
                                            "with Ghalstein until this",
                                            "mission is accomplished.^000000"
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ghalstein",
                                            args![
                                                "You may feel a little",
                                                "disoriented after drinking",
                                                "this tonic for the first time.",
                                                "I'll give you a moment to",
                                                "steady yourself before we",
                                                "talk about our mission."
                                            ],
                                        )?;
                                        ctx.var("lhz_boss").set(Val::from(5))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                            if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                                matched8 = true;
                            }
                            if matched8 {
                                break 'b8;
                            }
                        }
                        ctx.lines_as(
                            "Ghalstein",
                            args!["You really feel", "that way? Well, that's", "a shame. Pity, really..."],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou suddenly get the",
                            "weird feeling that someone",
                            "has been behind you this",
                            "entire time. As you slowly",
                            "turn around to look, you",
                            "were knocked unconscious...^000000"
                        ])?;
                        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
                        ctx.call(Function::PercentHeal, vec![Val::from(-50), Val::from(0)])?;
                        ctx.next()?;
                        ctx.lines(args!["..........", "........", ".....", ".."])?;
                        ctx.next()?;
                        ctx.lines_as("??????", args!["^666666Now...^000000"])?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("^666666Make him^000000")?;
                        } else {
                            ctx.mes("^666666Make her^000000")?;
                        }
                        ctx.mes("^666666swallow this pill.^000000")?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FFYou're forced to swallow",
                            "some strange pill which",
                            "immediately makes all your",
                            "thoughts hazier until it begins",
                            "to feel like your entire mind",
                            "is getting bleached somehow.^000000"
                        ])?;
                        ctx.var("lhz_boss").set(Val::from(0))?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(217), Val::from(313)])?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("lhz_boss").get()? == 5 {
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "So how are you feeling?",
                                    "Any nausea that you may",
                                    "be experiencing should",
                                    "be going away very soon.",
                                    "Anyway, let's get started."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "Rekenber Corporation",
                                    "is closely affiliated with",
                                    "Regenschirm, an underground",
                                    "laboratory inhabited by strange",
                                    "and suspicious creatures."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "Your mission will be to",
                                    "infiltrate this underground lab",
                                    "and bring back evidence which",
                                    "proves the existence of these",
                                    "creatures. You must gather",
                                    "various kinds of proof."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "But more importantly,",
                                    "there is ^FF0000something there",
                                    "that you have to bring",
                                    "back to me^000000. I can't tell",
                                    "you exactly what it is,",
                                    "but trust me. You'll know."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "If you search the",
                                    "Laboratory enough,",
                                    "you'll find what I'm",
                                    "talking about. In any",
                                    "case, you'll need to find",
                                    "a way to get inside the lab."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "It might be helpful to",
                                    "know that we've received",
                                    "reports about disappearances",
                                    "of people from the ^FF0000eastside",
                                    "slums^000000. We believe they are",
                                    "perhaps traveling illegally..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "If someone there is",
                                    "transporting people",
                                    "without authorization,",
                                    "maybe you can find a",
                                    "contact who can sneak",
                                    "you inside the Laboratory."
                                ],
                            )?;
                            ctx.var("lhz_boss").set(Val::from(6))?;
                            ctx.call(Function::ChangeQuest, vec![Val::from(12015), Val::from(12016)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("lhz_boss").get()? == 6 {
                                ctx.lines_as(
                                    "Ghalstein",
                                    args![
                                        "For now, it may be best",
                                        "to start investigating the",
                                        "eastside slums to see if you",
                                        "can find somebody who can",
                                        "get you inside the Laboratory."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("lhz_boss").get()? == 7 {
                                    if !(runtime::op(&ctx.var("misc_quest").get()?, "&", &Val::from(512))?.is_true()) {
                                        ctx.lines_as("Ghalstein", args!["An error occurred."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if ((ctx.call(Function::CountItem, vec![Val::from(7346)])?.is_true()
                                        && ctx.call(Function::CountItem, vec![Val::from(7347)])?.number()? > 9)
                                        && ctx.call(Function::CountItem, vec![Val::from(7345)])?.number()? > 9)
                                    {
                                        ctx.lines_as(
                                            "Ghalstein",
                                            args![
                                                "Ah, this looks like a",
                                                "sufficient amount of",
                                                "evidence for us to be",
                                                "able to effectively attack",
                                                "our enemy. I believe you're",
                                                "ready for the next mission."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ghalstein",
                                            args![
                                                "But first, I believe it's",
                                                "time for another red tonic",
                                                "dose so that we can keep",
                                                "monitoring you. Oh, and",
                                                "don't worry, it's not addictive",
                                                "or anything strange like that."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Ghalstein",
                                            args![
                                                "Now, find ^FF0000Gusharr^000000, one of",
                                                "our members, ^FF0000near Juno^000000.",
                                                "He will tell you about your",
                                                "next mission. Show this to",
                                                "him to prove that you are",
                                                "working with us. Don't forget."
                                            ],
                                        )?;
                                        ctx.call(Function::DelItem, vec![Val::from(7346), Val::from(1)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7347), Val::from(10)])?;
                                        ctx.call(Function::DelItem, vec![Val::from(7345), Val::from(10)])?;
                                        ctx.var("lhz_boss").set(Val::from(8))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7348), Val::from(1)])?;
                                        ctx.call(Function::ChangeQuest, vec![Val::from(12016), Val::from(12017)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ((!(ctx.call(Function::CountItem, vec![Val::from(7346)])?.is_true())
                                        && !(ctx.call(Function::CountItem, vec![Val::from(7347)])?.is_true()))
                                        && !(ctx.call(Function::CountItem, vec![Val::from(7345)])?.is_true()))
                                    {
                                        ctx.lines_as(
                                            "Ghalstein",
                                            args![
                                                "When you get inside the",
                                                "Laboratory, make sure that",
                                                "you search around for any",
                                                "records and get any sort of",
                                                "concrete evidence we can",
                                                "use. I'll be counting on you."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else {
                                        ctx.lines_as(
                                            "Ghalstein",
                                            args![
                                                "We'll need more evidence",
                                                "than this from the Laboratory",
                                                "to make any impact against our",
                                                "enemy. I know what we need is",
                                                "difficult to find, but I'm sure",
                                                "that you can pull this off."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else if ctx.var("lhz_boss").get()? == 8 {
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "Please find Gushaar",
                                            "in the vicinity of Juno.",
                                            "He will give you all the",
                                            "details for your next mission."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("lhz_boss").get()?.number()? < 25 {
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "You've made the decision",
                                            "to help us in order to learn",
                                            "more about our organzation.",
                                            "We appreciate all the help we",
                                            "can get, but I hope you don't",
                                            "regret getting in this deep."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "There's a certain price",
                                            "for knowledge. I believe",
                                            "you're capable of paying it,",
                                            "but whether or not this ordeal",
                                            "will be worthwhile to you is",
                                            "difficult to determine."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("lhz_boss").get()?.number()? < 43 {
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "You're so close to",
                                            "accomplishing your",
                                            "mission. If all goes as",
                                            "planned, we can do some",
                                            "real damage to our enemy.",
                                            "And you'll finally be free."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.var("lhz_boss").get()? == 43 {
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "I can't believe what",
                                            "happened. I just heard",
                                            "that the president was",
                                            "betrayed! This is horrible!",
                                            "We'll have to start all over.",
                                            "Was it all for nothing?!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "No. No, Jargeah would be",
                                            "so disappointed in me if",
                                            "I just gave up. I'll have to",
                                            "follow our dream, no matter",
                                            "what the obstacles are.",
                                            "Jargeah, give me strength!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "Also, you've done a great",
                                            "service for us. Although we",
                                            "failed, you carried out your",
                                            "missions perfectly. Here,",
                                            "take this antidote so that",
                                            "you'll finally be free of us."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "I'd like you to take",
                                            "this small gift, as way",
                                            "of apology. We asked you",
                                            "to do so much, but ultimately,",
                                            "we weren't able to accomplish",
                                            "what we set out to do. Still..."
                                        ],
                                    )?;
                                    ctx.var("lhz_boss").set(Val::from(44))?;
                                    ctx.call(Function::GetItem, vec![Val::from(617), Val::from(1)])?;
                                    ctx.call(Function::GetExperience, vec![Val::from(1800000), Val::from(0)])?;
                                    ctx.call(Function::CompleteQuest, vec![Val::from(12028)])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "I'd like to thank you",
                                            "for everything. Hopefully,",
                                            "you found the answers",
                                            "that you were looking for",
                                            "when you decided to help",
                                            "us. Farewell, faithful friend."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Ghalstein",
                                        args![
                                            "I hate being patient,",
                                            "but the old adage is",
                                            "true. He who runs away,",
                                            "lives to fight another day.",
                                            "But that day seems so",
                                            "far off into the future..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn ordinary_man(ctx: &Ctx) -> Script {
    ordinary_man_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SwitchStep {
    Start,
    OnTouch,
}

fn switch_run(ctx: &Ctx, mut step: SwitchStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SwitchStep::Start => {
                step = SwitchStep::OnTouch;
                continue 'machine;
            }
            SwitchStep::OnTouch => {
                if ctx.var("lhz_boss").get()? == 1 {
                    ctx.lines_as("??????", args!["^666666Come on.", "This way.^000000"])?;
                    ctx.next()?;
                    ctx.lines_as("??????", args!["^333333Hurry up, before", "the others see you...^000000"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["(Wait a second...", "That voice is", "awfully familiar.)"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Ignore it."), Val::from("Keep listening.")])? {
                        1 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["(That voice is", "awfully familiar...", "But what do I care?)"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "(This is too weird.",
                                    "I guess I better keep",
                                    "listening to see what",
                                    "I can learn about this...)"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "??????",
                                args!["^333333So why did you tell", "me to come to such", "a dangerous place...?^000000"],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFThat's...^000000",
                                "That's ^660000Ghalstein's^3355FF voice!^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "????",
                                args![
                                    "^666666Something unexpected",
                                    "came up. Hence the sudden",
                                    "change in plans. Now listen...^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFSuddenly, both of the",
                                "voices grew much quieter",
                                "as the two men spoke to each",
                                "other in low, hoarse whispers",
                                "that are almost inaudible.^000000"
                            ])?;
                            ctx.next()?;
                            ctx.lines_as("????", args!["^666666....", "...Relate.....", "...Business.....Kafr...^000000"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Ghalstein",
                                args![
                                    "^333333Oh. That does",
                                    "sound very urgent.",
                                    "But it's not safe to",
                                    "talk about this here.",
                                    "Let's move somewhere",
                                    "a little more secure.^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("????", args!["^666666No problem.", "........................^000000"])?;
                            ctx.next()?;
                            ctx.lines(args!["........................", ".....................", ".................."])?;
                            ctx.next()?;
                            ctx.lines(args![
                                "^3355FFThat other voice...",
                                "Whoever it belonged",
                                "to is gone! What's",
                                "going on here?^000000"
                            ])?;
                            ctx.var("lhz_boss").set(Val::from(2))?;
                            ctx.call(Function::SetQuest, vec![Val::from(12015)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn switch(ctx: &Ctx) -> Script {
    switch_run(ctx, SwitchStep::Start, Vec::new()).map(|_| ())
}

pub fn switch_ontouch(ctx: &Ctx) -> Script {
    switch_run(ctx, SwitchStep::OnTouch, Vec::new()).map(|_| ())
}

fn dismal_guy_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()?.number()? < 8 {
        ctx.lines_as(
            "Gushaar",
            args![
                "Decisions, decisions.",
                "Sometimes it's hard to",
                "make a choice when it",
                "isn't clear which one will",
                "give you the most benefit."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "I think most of the good",
                "things in life have some",
                "kind of cost, but they're",
                "usually worth it. I mean,",
                "nothing would have any",
                "worth without any cost, right?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 8 {
        ctx.lines_as(
            "Gushaar",
            args![
                "Alright. Choose",
                "your poison. I got",
                "two different kinds of",
                "tonics for you, red and blue.",
                "So which one will you drink?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("????"), Val::from("Red Tonic"), Val::from("Blue Tonic")],
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
                    "Gushaar",
                    args![
                        "Well, when you don't",
                        "make a decision, you",
                        "don't take action. And",
                        "when you don't take any",
                        "action, nothing happens.",
                        "Come on, don't be boring~"
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
                    "Gushaar",
                    args![
                        "Red, huh? I like",
                        "your style. But, eh,",
                        "isn't there something",
                        "you want to show me first?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Oh, right!"), Val::from("What are you talking about?")])? {
                    1 => {
                        if !(ctx.call(Function::CountItem, vec![Val::from(7348)])?.is_true()) {
                            ctx.lines_as(
                                "Gushaar",
                                args![
                                    "Hey... Umm...",
                                    "Aren't you supposed",
                                    "to be carrying some",
                                    "sort of secret something?",
                                    "I'd tell you want it was if we",
                                    "weren't so, well, secretive."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines(args![
                            "^3355FFYou suavely flash",
                            "your Membership Card",
                            "to Gushaar, who gives",
                            "you an approving nod.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Gushaar",
                            args![
                                "Yeah, that's what",
                                "I wanted to see. Good,",
                                "you're the one I've been",
                                "waiting here for. Alright,",
                                "just give me a second..."
                            ],
                        )?;
                        ctx.var("lhz_boss").set(Val::from(9))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Gushaar",
                            args![
                                "Huh?! Oh geez, I almost",
                                "never make this mistake.",
                                "I'm sorry, I thought you might",
                                "have been someone else."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Gushaar",
                    args![
                        "Blue...? Huh.",
                        "That shouldn't be",
                        "the one you want.",
                        "Well, if you're who",
                        "I think you are... Are you?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("lhz_boss").get()? == 9 {
        ctx.lines_as(
            "Gushaar",
            args![
                "Alright, first of all,",
                "you should know why the",
                "Rekenber Corporation is",
                "our sworn enemy. Did you",
                "know that it basically controls",
                "the Schwarzwald Republic?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "They're so powerful, they",
                "manipulate everything in the",
                "economy, media and even the",
                "politics of this nation. There are other corporations, sure, but",
                "they're nothing in comparsion."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "Rekenber Corporation...",
                "They've even got a hold on",
                "all the other companies, simply",
                "by bribing politicians to pass",
                "the bills they want passed.",
                "This isn't a real republic..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "The ultimate goal of our",
                "group, ''Secret Wing,'' is to",
                "establish true independence",
                "in the Schwarzwald Republic",
                "by destroying Rekenber Corporation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "Fortunately, the new president",
                "of the Schwarzwald Republic,",
                "Mr. Weierstrass, is sympathetic",
                "to our cause. He is working with us to bring about a revolution and",
                "overthrow Rekenber Corporation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "Your mission will be to",
                "visit with the president and",
                "bring back any information",
                "he may have for us. Remember,",
                "because of his position, you can only see him during certain hours."
            ],
        )?;
        ctx.var("lhz_boss").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 10 {
        ctx.lines_as(
            "Gushaar",
            args![
                "I hope you understand",
                "''Secret Wing's'' ultimate",
                "goal now. In any case, you've",
                "got to meet our president,",
                "Karl Theodor Weierstrass."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "It won't be easy to see",
                "such an important man,",
                "so show your Secret Wing",
                "Membership Card to his",
                "secretary and there should",
                "be very little hassle."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gushaar",
            args![
                "Because of his schedule,",
                "you'll only be allowed to see",
                "him from ^FF00008:00 PM to 11:00 PM^000000,",
                "and from ^FF000011:00 AM to 2:00PM PST^000000.Good luck on this mission, friend."
            ],
        )?;
        ctx.var("lhz_boss").set(Val::from(11))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(12017), Val::from(12018)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Gushaar",
            args![
                "Rekenber...",
                "I shall see your",
                "destruction if it's the",
                "very last thing I do!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn dismal_guy(ctx: &Ctx) -> Script {
    dismal_guy_body(ctx, Vec::new()).map(|_| ())
}

fn secretary_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()?.number()? < 11 {
        ctx.lines_as("Hes O'Neil", args!["Good day, how", "may I help you?"])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[
                    Val::from("Oh. I'm fine, thanks."),
                    Val::from("I want to meet the president~"),
                    Val::from("W-who are you...?"),
                ],
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
                    "Hes O'Neil",
                    args![
                        "Since this is an",
                        "important government",
                        "building, please keep",
                        "in mind that certain areas,",
                        "monitored by guards, are",
                        "off limits to visitors."
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
                    "Hes O'Neil",
                    args!["President Weierstrass?", "Do you have an appointment", "to meet with him today?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])? {
                    1 => {
                        ctx.lines_as(
                            "Hes O'Neil",
                            args![
                                "Alright, let me see.",
                                "May I please have your",
                                "name so that I can look",
                                "it up in today's schedule?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))],
                        )? {
                            1 => {}
                            _ => {}
                        }
                        ctx.lines_as(
                            "Hes O'Neil",
                            args![
                                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                "I'm sorry, but I can't find",
                                "your name on the list. You",
                                "can only visit the president",
                                "if you have an appointment.",
                                "Thank you for your cooperation."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Hes O'Neil",
                            args![
                                "I'm so sorry, but because",
                                "the president is an incredibly",
                                "busy man, he can only make",
                                "time to see the people who",
                                "have an appointment."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Hes O'Neil",
                    args![
                        "Hmm...?",
                        "I'm the personal",
                        "secretary to the",
                        "president of the",
                        "Schwarzwald Republic,",
                        "if that's what you mean."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        if ctx.var("lhz_boss").get()?.number()? < 16 {
            ctx.lines_as("Hes O'Neil", args!["Good day, how", "may I help you?"])?;
            ctx.next()?;
            'b4: {
                let subject4 = Val::from(runtime::select_values(
                    ctx,
                    &[
                        Val::from("Show Secret Wing Card."),
                        Val::from("I'm here to see the president."),
                        Val::from("Nothing."),
                    ],
                )?);
                let mut matched4 = false;
                let no_case4 = !subject4.loosely_equals(&Val::from(1))
                    && !subject4.loosely_equals(&Val::from(2))
                    && !subject4.loosely_equals(&Val::from(3));
                if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                    matched4 = true;
                }
                if matched4 {
                    if !(ctx.call(Function::CountItem, vec![Val::from(7348)])?.is_true()) {
                        ctx.lines(args![
                            "^3355FFWait a second...",
                            "You don't seem to",
                            "be carrying your",
                            "''Secret Wing''",
                            "Membership Card.^000000"
                        ])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if ((ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? > 10
                        && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? < 15)
                        || (ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? > 19
                            && ctx.call(Function::GetTime, vec![ctx.constant("DT_HOUR")?])?.number()? <= 23))
                    {
                        ctx.lines(args![
                            "^3355FFYou suavely flash",
                            "your ''Secret Wing''",
                            "Membership Card.^000000"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hes O'Neil",
                            args![
                                "Oh... I see.",
                                "The president has",
                                "been expecting you.",
                                "Please, right this way."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(113), Val::from(53)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Hes O'Neil",
                        args![
                            "Oh... I see.",
                            "I'm sorry, but you've",
                            "come too early for your",
                            "appointment. Would you",
                            "come back to see the",
                            "president later?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                    matched4 = true;
                }
                if matched4 {
                    ctx.lines_as(
                        "Hes O'Neil",
                        args!["President Weierstrass?", "Do you have an appointment", "to meet with him today?"],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No")])? {
                        1 => {
                            ctx.lines_as(
                                "Hes O'Neil",
                                args![
                                    "Alright, let me see.",
                                    "May I please have your",
                                    "name so that I can look",
                                    "it up in today's schedule?"
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(
                                ctx,
                                &[((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))],
                            )? {
                                1 => {}
                                _ => {}
                            }
                            ctx.lines_as(
                                "Hes O'Neil",
                                args![
                                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                                    "I'm sorry, but I can't find",
                                    "your name on the list. You",
                                    "can only visit the president",
                                    "if you have an appointment.",
                                    "Thank you for your cooperation."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Hes O'Neil",
                                args![
                                    "I'm so sorry, but because",
                                    "the president is an incredibly",
                                    "busy man, he can only make",
                                    "time to see the people who",
                                    "have an appointment."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                    matched4 = true;
                }
                if matched4 {
                    ctx.lines_as(
                        "Hes O'Neil",
                        args![
                            "Since this is an",
                            "important government",
                            "building, please keep",
                            "in mind that certain areas,",
                            "monitored by guards, are",
                            "off limits to visitors."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("lhz_boss").get()? == 16 {
                if !(ctx.call(Function::CountItem, vec![Val::from(7342)])?.is_true()) {
                    ctx.lines_as(
                        "Hes O'Neil",
                        args![
                            "Hm? I thought the",
                            "president was supposed",
                            "to give you some kind of",
                            "file, but perhaps I was",
                            "mistaken? Let me think..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Hes O'Neil", args!["Hello, may I be", "of any assistance?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I brought this.")])? {
                    1 => {}
                    _ => {}
                }
                ctx.lines_as(
                    "Hes O'Neil",
                    args![
                        "Ah, the file folder",
                        "I needed. I'm so sorry",
                        "to trouble you. Thank",
                        "you so much for your help."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7342), Val::from(1)])?;
                ctx.var("lhz_boss").set(Val::from(17))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("lhz_boss").get()?.number()? < 21 {
                ctx.lines_as(
                    "Hes O'Neil",
                    args![
                        "I'm sorry, but the",
                        "president is currently",
                        "outside on business.",
                        "Please come back to",
                        "visit him another time."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("lhz_boss").get()?.number()? < 43 {
                ctx.lines_as(
                    "Hes O'Neil",
                    args!["Oh, the president", "is expecting you.", "Please, go right"],
                )?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.mes("on ahead, sir.")?;
                } else {
                    ctx.mes("on ahead, ma'am.")?;
                }
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(113), Val::from(53)])?;
                return Err(Stop::End);
            } else if ctx.var("hg_tre").get()? == 56 {
                ctx.lines_as(
                    "Hes O'Neil",
                    args!["The President has given the order that ", "nobody is allowed to enter this place."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["However, you will change your mind after listening to my explanation."],
                )?;
                ctx.next()?;
                ctx.lines_as("Hes O'Neil", args!["...............", "Is it something very important?"])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["It is the most important thing."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hes O'Neil",
                    args![
                        "Alright then, there you go.",
                        "(Whisper) I hope you will become his source of strength."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(113), Val::from(53)])?;
                return Err(Stop::End);
            } else if ctx.var("hg_tre").get()?.number()? > 56 {
                ctx.lines_as(
                    "Hes O'Neil",
                    args!["The president said that you're welcome to visit him anytime."],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(113), Val::from(53)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Hes O'Neil",
                    args![
                        "I'm sorry...",
                        "It seems that the",
                        "president would like",
                        "to have some privacy now."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn secretary_1(ctx: &Ctx) -> Script {
    secretary_1_body(ctx, Vec::new()).map(|_| ())
}

fn guard_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhz_boss").get()?.number()? < 11 {
        ctx.lines_as(
            "Guard",
            args![
                "You are in the",
                "President's House.",
                "Arms are prohibited",
                "without authorization in",
                "this government building.",
                "Thank you for your cooperation."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Nude, vec![])?;
        ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(83), Val::from(22)])?;
        return Err(Stop::End);
    } else if (ctx.var("lhz_boss").get()? == 11 || ctx.var("lhz_boss").get()? == 12) {
        if ctx.var("lhz_boss").get()? == 12 {
            ctx.var("@visit_pre")
                .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?)?;
        }
        if ctx.var("@visit_pre").get()? == 7 {
            ctx.lines_as("?????", args!["I see.", "Then..."])?;
            ctx.next()?;
            ctx.lines_as("????????", args!["Ummm....", "Understood...", "......"])?;
            ctx.next()?;
            ctx.call(Function::EnableNpc, vec![Val::from("A Fine Gentleman")])?;
            ctx.next()?;
            ctx.lines_as("?????", args![".........."])?;
            ctx.next()?;
            ctx.lines_as("Guard", args!["Ah, Mr. Keshnaar.", "Are you leaving?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Keshnaar",
                args!["Yes, it's about time", "for me to depart. Ah,", "and may I ask who this"],
            )?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("young gentleman here is?")?;
            } else {
                ctx.mes("lovely young lady is?")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Guard",
                args![
                    "Mr. Keshnaar,",
                    ((Val::from("I present ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                    ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(", allow me")),
                    "to introduce Eridan Keshnaar."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Keshnaar",
                args![
                    "A pleasure to meet you.",
                    "Now, if you would excuse",
                    "me, I have some business",
                    "to attend to. Good day~"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Guard", args!["Take care,", "Mr. Keshnaar."])?;
            ctx.var("lhz_boss").set(Val::from(13))?;
            ctx.call(Function::DisableNpc, vec![Val::from("A Fine Gentleman")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Guard",
            args![
                "Please wait a moment.",
                "Currently, the president",
                "is seeing another guest.",
                "Thank you for your patience."
            ],
        )?;
        if ctx.var("lhz_boss").get()? == 11 {
            ctx.var("lhz_boss").set(Val::from(12))?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("lhz_boss").get()? == 13 {
        ctx.lines_as(
            "Guard",
            args![
                "Now you may enter",
                "and speak with the",
                "president. Thank you",
                "for waiting all this time."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Thank you."), Val::from("Who was that gentleman...?")])? {
            1 => {
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines(args![
                        ((Val::from("A Mister ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))
                    ])?;
                } else if ctx.call(Function::GetPartnerId, vec![])?.is_true() {
                    ctx.lines(args![
                        ((Val::from("A Missis ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))
                    ])?;
                } else {
                    ctx.lines(args![
                        ((Val::from("A Miss ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(""))
                    ])?;
                }
                ctx.mes("has arrived to see you now.")?;
                ctx.next()?;
                ctx.mes(".....")?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. President",
                    args!["Oh, you're early!", "I'm sorry, but would you", "please wait one minute?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Guard",
                    args![
                        "Yes, sir.",
                        "My apologies, but would",
                        "you please wait until the",
                        "president is ready?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Guard",
                    args![
                        "Oh, Mr. Keshnaar is the",
                        "government negotiator from",
                        "the Rekenber Corporation.",
                        "He often visits the president",
                        "to discuss various issues."
                    ],
                )?;
                ctx.var("lhz_boss").set(Val::from(14))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Guard", args!["Greetings.", "You may enter to", "see the president."])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("yuno_pre"), Val::from(78), Val::from(69)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn guard(ctx: &Ctx) -> Script {
    guard_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AFineGentlemanStep {
    Start,
    OnInit,
}

fn a_fine_gentleman_run(ctx: &Ctx, mut step: AFineGentlemanStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AFineGentlemanStep::Start => {
                step = AFineGentlemanStep::OnInit;
                continue 'machine;
            }
            AFineGentlemanStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("A Fine Gentleman")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn a_fine_gentleman(ctx: &Ctx) -> Script {
    a_fine_gentleman_run(ctx, AFineGentlemanStep::Start, Vec::new()).map(|_| ())
}

pub fn a_fine_gentleman_oninit(ctx: &Ctx) -> Script {
    a_fine_gentleman_run(ctx, AFineGentlemanStep::OnInit, Vec::new()).map(|_| ())
}
