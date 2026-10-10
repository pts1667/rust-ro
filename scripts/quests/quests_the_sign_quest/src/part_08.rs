use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn pleasant_featured_lady_s_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.var("sign_q").get()?.number()? < 82 {
        ctx.lines_as(
            "Serin",
            args!["I'm sorry,", "but you're not", "the person I'm", "looking for. No..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("sign_q").get()? == 82 {
            ctx.lines_as(
                "Serin",
                args![
                    "Kind adventurer...",
                    "You're the one I'm looking",
                    "for, the one who's been on",
                    "a long journey to prove"
                ],
            )?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("his courage.")?;
            } else {
                ctx.mes("her courage.")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Serin",
                args![
                    "Even in the darkness of",
                    "Niflheim, I can recognize",
                    "you. You shine out to me like",
                    "a star! I beg of you, please",
                    "listen to my story..."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("I'm busy, sorry.:Sure, go ahead.")])? {
                1 => {
                    ctx.lines_as(
                        "Serin",
                        args![
                            "I know I can't stop",
                            "you from pursing your",
                            "path to glory. But if you",
                            "should change your mind...!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Serin",
                        args![
                            "Thank you...",
                            "Now, if you've heard",
                            "from the others and as",
                            "you can tell by my appearance,",
                            "I clearly don't belong here."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Serin",
                        args![
                            "I might not have",
                            "the luck to be selected",
                            "by Valkyrie, but I need",
                            "your help to get out of",
                            "here as soon as I can~"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Er, I'm busy, now that I think about it.:Sure, let's do it.")])? {
                        1 => {
                            ctx.lines_as(
                                "Serin",
                                args![
                                    "I know I can't stop",
                                    "you from pursing your",
                                    "path to glory. But if you",
                                    "should change your mind...!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Serin",
                                args![
                                    "I know I'm asking too much",
                                    "of you, but if you'd also help",
                                    "the other unfortunate souls in",
                                    "this place, I'd be truly grateful. Somehow, I think solving their problems will help me get back."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Sure, why not?:Why should I help them?")])? {
                                1 => {
                                    ctx.lines_as("Serin", args!["I knew you'd understand!", "You really are a kind person.", "Now, in order to help the other", "misfortunate in Niflheim, you've got to remember to gain their trust first and get close to them, okay?"])?;
                                    ctx.var("sign_q").set(Val::from(83))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("Serin", args!["I... I understand.", "Although I asked too much in asking you to help the other people here, I was hoping you'd realize that", "would be the best way to learn clues for a way to escape for now."])?;
                                    ctx.var("sign_q").set(Val::from(84))?;
                                    ctx.close_window()?;
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
        } else {
            if ctx.var("sign_q").get()? == 83 {
                if ctx.var("sign_sq").get()?.number()? < 8 {
                    ctx.lines_as(
                        "Serin",
                        args![
                            "Hmm...?",
                            "You don't know which",
                            "people you need to help",
                            "here in Niflheim? Mmm,",
                            "let me think about it..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Serin",
                        args![
                            "In this doomed place,",
                            "there isn't too much trust",
                            "amongst some of the people",
                            "here, but some will share their",
                            "problems if you ^3355FFapproach them",
                            "closely^000000, ^3355FFface to face^000000. Okay?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Serin",
                        args![
                            "In this way, if they talk",
                            "to you first, they might",
                            "be more willing to accept",
                            "help or tell you what you",
                            "really need to know.",
                            "I hope this helps..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("sign_sq").get()? == 8 {
                    ctx.lines_as(
                        "Serin",
                        args![
                            "Thank you so much for",
                            "your help. This is all I have,",
                            "and it means a lot to me, but",
                            "I want you to have this ring. It's the only way I can properly express my gratitude for your aid so far."
                        ],
                    )?;
                    ctx.var("sign_q").set(Val::from(85))?;
                    ctx.var("sign_sq").set(Val::from(0))?;
                    ctx.call(Function::GetItem, vec![Val::from(2642), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Serin",
                        args!["To the dead,", "living people", "are either good", "prey or good marks."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("sign_q").get()? == 84 {
                    ctx.lines_as(
                        "Serin",
                        args![
                            "Hello, adventurer.",
                            "How have you been doing?",
                            "I'm surprised to still see you",
                            "around this dangerous town."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Serin",
                        args![
                            "If you wander around",
                            "Niflheim, you'll find that",
                            "some of the deceased here are",
                            "different than most of the others in this town. Please talk to them and see if you can be of any help."
                        ],
                    )?;
                    ctx.var("sign_q").set(Val::from(87))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sign_q").get()? == 85 {
                        ctx.lines_as(
                            "Serin",
                            args![
                                "There's a Bard in",
                                "Niflheim with a very",
                                "sweet, yet melancholy voice.",
                                "He sings for strangers just",
                                "like you, adventurer."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Serin",
                            args![
                                "I think he might know",
                                "some useful information,",
                                "so it might be a good",
                                "idea to speak with him."
                            ],
                        )?;
                        ctx.var("sign_q").set(Val::from(86))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("sign_q").get()?.number()? < 92 {
                            ctx.lines_as(
                                "Serin",
                                args![
                                    ((Val::from("Hello, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(",")),
                                    "I'm surprised that",
                                    "you've managed to survive",
                                    "here all this time. It's no small feat for a mortal to be able to remain alive in Niflheim..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("sign_q").get()? == 92 {
                                if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 0 {
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("~")),
                                            "How is everything?",
                                            "Once again, I want to",
                                            "thank you for helping the",
                                            "poor souls in this place."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            "Hmm...?",
                                            "What's with that",
                                            "look on your face?",
                                            "Do you have something",
                                            "you want to ask me?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                } else {
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            ((Val::from("Ah, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("~")),
                                            "How is everything?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            "Hmm...?",
                                            "What's with that",
                                            "look on your face?",
                                            "Do you have something",
                                            "you want to ask me?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                                match runtime::select_values(ctx, &[Val::from("Nothing.:About the Queen of the Dead...")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Serin",
                                            args!["Well, if you ever", "need my help, please", "don't hesitate to ask."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "The Queen of the Dead?",
                                                "Why, she's the ruler of Niflheim.",
                                                "As Loki and Angrboda's last child, she was given complete control of this realm by Odin."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Serin", args!["She usually rules from her", "castle, Hell, but sometimes she goes out to tour Niflheim. That's all I really know about her."])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("About the Symbol of Nine Realms")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "Oh that...?",
                                                "Sometimes it's called",
                                                "the Voucher of the Dead.",
                                                "Odin gave it to the Queen",
                                                "of the Dead to signify her",
                                                "authority over Niflheim."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "There are many rumors",
                                                "floating around about that",
                                                "symbol and its abilities, but",
                                                "all we know for sure is that it can be used to command the dead..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("About Angrboda")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "Angrboda...?",
                                                "She's the mother of the",
                                                "Queen of Hell, as well as",
                                                "some of the most powerful",
                                                "monsters in this universe."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "The gods feared her monstrous",
                                                "children so much that they bound",
                                                "them and sealed Angrboda away.",
                                                "Nothing has been heard",
                                                "of her since..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("I was told by the Witch that you're...")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "Oh, the Witch was",
                                                "talking about me? I'm",
                                                "aware that she doesn't",
                                                "think very highly of me.",
                                                "Then again, it's not like",
                                                "I can trust her either."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "Well, it can't be helped.",
                                                "At least we both agree that",
                                                "somehow I don't really belong",
                                                "here in Niflheim."
                                            ],
                                        )?;
                                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                        ctx.var("sign_q").set(Val::from(93))?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                if ctx.var("sign_q").get()? == 93 {
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            "I'm sorry that I sort of",
                                            "rambled on and on that last",
                                            "time we spoke. Now that I think",
                                            "about it, I didn't really give you any information that might have been very useful to you."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            "But lately I've heard that the",
                                            "Queen of the Dead is seeking her",
                                            "mother, Angrboda. It's strange that even the queen, with all of her",
                                            "power, is unable to find her."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Serin",
                                        args![
                                            "I have no idea how,",
                                            "but it seems like Angrboda",
                                            "has been sealed away in the",
                                            "world of Midgard..."
                                        ],
                                    )?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                    ctx.var("sign_q").set(Val::from(94))?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ctx.var("sign_q").get()? == 94 {
                                        ctx.lines_as("Serin", args!["I've told you", "everything I know.", "But if you want to learn more", "about Angrboda, you probably need to talk to someone who actually communicates with the gods."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Serin",
                                            args![
                                                "Perhaps Valkyrie",
                                                "or someone who actually",
                                                "lived in Asgard may know",
                                                "more about where Angrboda",
                                                "is sealed away."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Serin", args!["Also, why would you want", "the Symbol of the Nine Realms?", "It's a very dangerous object that could cause a lot of trouble if it fell into the wrong hands..."])?;
                                        ctx.next()?;
                                        'b5: {
                                            let subject5 = Val::from(runtime::select_values(
                                                ctx,
                                                &[Val::from("What do you mean??:Don't worry, it won't.")],
                                            )?);
                                            let mut matched5 = false;
                                            let no_case5 =
                                                !subject5.loosely_equals(&Val::from(1)) && !subject5.loosely_equals(&Val::from(2));
                                            if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                                matched5 = true;
                                            }
                                            if matched5 {
                                                ctx.lines_as("Serin", args!["There are plenty of dead", "people in Niflheim that are", "driven by regret and despair.", "The rage of being dead may result in some of them performing horrific deeds if they had the symbol."])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("Then what should I do?:Thanks for the concern, but I'm not worried.")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as("Serin", args!["I really think that you", "should bring that symbol", "to me before you show it to anybody else. Then, I want you to know why I really need its power."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Serin",
                                                            args![
                                                                "I can't explain myself",
                                                                "now, but I want to use the",
                                                                "power of the symbol to do",
                                                                "something really helpful",
                                                                "for you. You'll just need",
                                                                "to trust me on this."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        let choice = runtime::select_values(ctx, &[Val::from("I'll think about it.")])?;
                                                        ctx.var("@menu").set(choice)?;
                                                        ctx.lines_as("Serin", args!["Then I will", "wait for you here."])?;
                                                        ctx.var("sign_q").set(Val::from(95))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as("Serin", args!["Alright...", "It's your decision."])?;
                                                        ctx.var("sign_q").set(Val::from(95))?;
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
                                                    "Serin",
                                                    args![
                                                        "Ah...",
                                                        "Alright, I suppose",
                                                        "that if you're worthy",
                                                        "of obtaining the symbol,",
                                                        "you should also be able",
                                                        "to protect it from abuse."
                                                    ],
                                                )?;
                                                ctx.var("sign_q").set(Val::from(95))?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else {
                                        if ctx.var("sign_q").get()?.number()? < 118 {
                                            ctx.lines_as(
                                                "Serin",
                                                args![
                                                    "If you obtain the",
                                                    "Symbol of Nine Realms,",
                                                    "please bring it to me",
                                                    ((Val::from("right away, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                        + Val::from("."))
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("sign_q").get()? == 118 {
                                                ctx.lines_as("Serin", args!["Yes...", "This is the Symbol of the", "Nine Realms. It only has enough", "power for one use, but it's still not easy to get this. Now, what were you planning to use this for?"])?;
                                                ctx.next()?;
                                                let choice = runtime::select_values(ctx, &[Val::from("Tell her your story.")])?;
                                                ctx.var("@menu").set(choice)?;
                                                ctx.lines_as(
                                                    "Serin",
                                                    args![
                                                        "Ah, so you're here",
                                                        "in Niflheim to overcome",
                                                        "the ordeals of the gods and",
                                                        "prove your courage to Valkyrie."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Serin",
                                                    args![
                                                        "So wait...",
                                                        "You've obtained that symbol",
                                                        "just for that witch? This may",
                                                        "seem rude, but I want to tell",
                                                        "you exactly how I feel about her."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Serin", args!["Isn't it suspicious that she", "hasn't explained her intentions to you? The Symbol of Nine Realms", "is one of the most powerful objects in Niflheim. Whatever she's planning has to be large scale."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Serin", args!["Then, there's that Metz Brayde.", "Do you really believe that after he completes his research, he'll just give it back to you? You can't just trust someone to hold an object of enormous power and give it back!"])?;
                                                ctx.next()?;
                                                match runtime::select_values(
                                                    ctx,
                                                    &[Val::from("W-what should I do?:I can't believe this!")],
                                                )? {
                                                    1 => {
                                                        ctx.lines_as("Serin", args!["Well, I can't really tell", "you what to do, but I don't think it's a good idea to let that witch have the Symbol of Nine Realms. There's no telling what she'll", "use its power for."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Serin",
                                                            args![
                                                                "Actually...",
                                                                "There is this rumor",
                                                                "going around, but I don't",
                                                                "know if you want to hear it..."
                                                            ],
                                                        )?;
                                                        ctx.var("sign_q").set(Val::from(131))?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    2 => {
                                                        ctx.lines_as("Serin", args!["I'm sorry if you were offended.", "I know I can't really convince you to believe anything, but please understand that I just don't want anyone to take advantage of you. Forgive my rudeness."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    _ => {}
                                                }
                                            } else {
                                                if ctx.var("sign_q").get()?.number()? < 129 {
                                                    ctx.lines(args![
                                                        "^3355FFThis is only",
                                                        "a lingering trace",
                                                        "of Serin, a manifestation",
                                                        "of her definitive memory.^000000"
                                                    ])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    if ctx.var("sign_q").get()? == 129 {
                                                        ctx.lines_as("Serin", args!["...", "......"])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Serin", args!["...", "......", ".........."])?;
                                                        if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                                                            ctx.next()?;
                                                            match runtime::select_values(
                                                                ctx,
                                                                &[Val::from("Give Serin her ring.:Keep the ring.")],
                                                            )? {
                                                                1 => {
                                                                    ctx.lines_as("Serin", args!["...", "......", "............."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as("Serin", args![((Val::from("....") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".....")), "I don't want to forget you.", "I can forget my obsession with", "life and all my other memories,", "But if I can keep just one memory, I want it to be of your kindness."])?;
                                                                    ctx.next()?;
                                                                    ctx.lines_as(
                                                                        "Serin",
                                                                        args![
                                                                            "Even now, you're still",
                                                                            "so very kind to me. Thank",
                                                                            "you for giving back my ring,",
                                                                            "my most precious possession.",
                                                                            "Thank you, thank you..."
                                                                        ],
                                                                    )?;
                                                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^3355FFThe sound of Serin's voice", "softened and when it grew silent, her eyes blankly stared ahead as", "if she were in a trance. It looks like she has already forgotten", "everything...^000000"])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args![
                                                                        "^3355FFBut Serin has also been able",
                                                                        "to forget her sadness. The tears streaked across her cheeks and",
                                                                        "the faint smile on her lips tell you that her memories of you",
                                                                        "will always remain in her heart.^000000"
                                                                    ])?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(2643), Val::from(1)])?;
                                                                    ctx.call(
                                                                        Function::GetExperience,
                                                                        vec![Val::from(500000), Val::from(0)],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                2 => {
                                                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                _ => {}
                                                            }
                                                        } else {
                                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        }
                                                    } else {
                                                        if ctx.var("sign_q").get()? == 130 {
                                                            ctx.lines_as("Serin", args!["..........."])?;
                                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Serin", args!["..........."])?;
                                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Serin", args!["..........."])?;
                                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                                            ctx.next()?;
                                                            ctx.lines_as("Serin", args!["..........."])?;
                                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                                            ctx.close_window()?;
                                                            return Err(Stop::End);
                                                        } else {
                                                            if ctx.var("sign_q").get()? == 131 {
                                                                ctx.lines_as("Serin", args!["........"])?;
                                                                ctx.next()?;
                                                                let choice = runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from("Tell me the rumors about the witch.")],
                                                                )?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Serin", args!["Well, I've heard that the witch has been hiding deep in Niflheim and will only speak to those who pass her test. Apparently, she's been preparing some sort of ritual", "to gain more power..."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Serin",
                                                                    args![
                                                                        "It's almost too horrible",
                                                                        "to believe, but I think that",
                                                                        "she's preparing the ritual to",
                                                                        "summon Dark Lord, ruler of",
                                                                        "the Realm of Fiends."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                let choice = runtime::select_values(ctx, &[Val::from("Dark Lord...?")])?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.lines_as("Serin", args!["Well, most people don't", "know this, but Dark Lord", "comes from the Realm of Fiends.", "In his own domain, he's so powerful that even the gods fear his might."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Serin", args!["Now, sometimes Dark Lord can", "enter the world of Midgard, but only in a manifestation that represents just a small portion", "of his full power."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Serin", args!["You see, it takes too much energy to travel from the Realm of Fiends to Midgard. And since the life force in Midgard conflicts with his powers, Dark Lord can't reach your world directly."])?;
                                                                ctx.next()?;
                                                                let choice = runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from("Wouldn't Lady Hell stop him?")],
                                                                )?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.lines_as("Serin", args!["Well, Dark Lord only wants", "Midgard, so the Queen of the Dead doesn't feel threatened about her control of Niflheim."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Serin", args!["In fact, I'm sure she's aware", "that Dark Lord will cause thousands of deaths in Midgard, meaning that there would be thousands more souls for her to rule over. So it's not to her advantage to stop him."])?;
                                                                ctx.next()?;
                                                                let choice = runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from("Then what should we do...?")],
                                                                )?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.lines_as(
                                                                    "Serin",
                                                                    args![
                                                                        "No matter what the cost,",
                                                                        "we've got to stop her from",
                                                                        "summoning Dark Lord in order",
                                                                        "to protect Midgard from",
                                                                        "total annihiliation."
                                                                    ],
                                                                )?;
                                                                ctx.next()?;
                                                                ctx.lines_as("Serin", args!["I used to be a pretty", "skilled Wizard, so I know", "that she needs to draw a really huge magic circle in Niflheim if she wants to summon Dark Lord."])?;
                                                                ctx.next()?;
                                                                let choice =
                                                                    runtime::select_values(ctx, &[Val::from("Can you tell me where?")])?;
                                                                ctx.var("@menu").set(choice)?;
                                                                ctx.lines_as("Serin", args!["Well, there are certain rules", "and conditions to perform the summoning. There aren't any spaces wide enough in the Valley of Gyoll, and Niflheim's entrance is watched by too many people..."])?;
                                                                ctx.next()?;
                                                                ctx.lines_as(
                                                                    "Serin",
                                                                    args![
                                                                        "Now, if I were to summon",
                                                                        "Dark Lord, I would probably",
                                                                        "do it somewhere inside town",
                                                                        "to avoid attention. Hmmm...",
                                                                        "I'll send you there once I figure out where the circle might be."
                                                                    ],
                                                                )?;
                                                                ctx.var("sign_q").set(Val::from(132))?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.var("sign_q").get()? == 132 {
                                                                if ctx.var("$@sign_w2").get()? == 1 {
                                                                    ctx.lines_as(
                                                                        "Serin",
                                                                        args![
                                                                            "I need to finish",
                                                                            "my preparations to send",
                                                                            "you to the magic circle,",
                                                                            "so I need a little more time..."
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.lines_as(
                                                                    "Serin",
                                                                    args!["Alright...", "So are you", "ready to leave?"],
                                                                )?;
                                                                ctx.next()?;
                                                                match runtime::select_values(ctx, &[Val::from("Yes:Not yet")])? {
                                                                    1 => {
                                                                        ctx.lines_as("Serin", args!["Okay~", "Let's go..."])?;
                                                                        ctx.close_window()?;
                                                                        ctx.var("$@sign_w2").set(Val::from(1))?;
                                                                        ctx.call(
                                                                            Function::Warp,
                                                                            vec![Val::from("que_sign01"), Val::from(199), Val::from(36)],
                                                                        )?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    2 => {
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    _ => {}
                                                                }
                                                            } else if (ctx.var("sign_q").get()?.number()? < 135
                                                                || ctx.var("sign_q").get()? == 199)
                                                            {
                                                                if ctx.var("$@sign_w2").get()? == 1 {
                                                                    ctx.lines_as(
                                                                        "Serin",
                                                                        args![
                                                                            "My preparations",
                                                                            "are taking longer",
                                                                            "than I thought. Would",
                                                                            "you wait a little while?"
                                                                        ],
                                                                    )?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.lines(args![
                                                                    "^3355FFThis is just a lingering",
                                                                    "trace of Serin, an impression",
                                                                    "of her soul and memories that",
                                                                    "you can still sense somehow.^000000"
                                                                ])?;
                                                                ctx.next()?;
                                                                match runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from("Follow the trace.:Ignore it.")],
                                                                )? {
                                                                    1 => {
                                                                        ctx.close_window()?;
                                                                        ctx.var("sign_q").set(Val::from(199))?;
                                                                        ctx.var("$@sign_w2").set(Val::from(1))?;
                                                                        ctx.call(
                                                                            Function::Warp,
                                                                            vec![Val::from("que_sign01"), Val::from(199), Val::from(36)],
                                                                        )?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    2 => {
                                                                        ctx.close_window()?;
                                                                        return Err(Stop::End);
                                                                    }
                                                                    _ => {}
                                                                }
                                                            } else if ctx.var("sign_q").get()?.number()? < 137 {
                                                                ctx.lines_as("Serin", args!["...", "......", "W-who...", "Who are you?"])?;
                                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else if ctx.var("sign_q").get()?.number()? > 199 {
                                                                ctx.lines(args![
                                                                    "^3355FFThis is just a lingering",
                                                                    "trace of Serin, an impression",
                                                                    "of her soul and memories that",
                                                                    "you can still sense somehow.",
                                                                    "What could have happened to her?^000000"
                                                                ])?;
                                                                if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 0 {
                                                                    ctx.next()?;
                                                                    ctx.lines(args![
                                                                        "^33555FFSuddenly, Serin's",
                                                                        "gold ring sparkled",
                                                                        "with a soft glow and",
                                                                        "faded away.^000000"
                                                                    ])?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(2642), Val::from(1)])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.close_window()?;
                                                                return Err(Stop::End);
                                                            } else {
                                                                if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 0 {
                                                                    ctx.lines_as("Serin", args!["Thank you..."])?;
                                                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                                                                    ctx.call(Function::DelItem, vec![Val::from(2642), Val::from(1)])?;
                                                                    ctx.next()?;
                                                                    ctx.lines(args!["^3355FFYou returned", "Serin's gold ring.^000000"])?;
                                                                    ctx.close_window()?;
                                                                    return Err(Stop::End);
                                                                }
                                                                ctx.lines_as("Serin", args!["...", "......"])?;
                                                                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
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
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn pleasant_featured_lady_s(ctx: &Ctx) -> Script {
    pleasant_featured_lady_s_body(ctx, Vec::new()).map(|_| ())
}

fn witch_s_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_kir_talk1 = Val::from(0);
    let mut l_kir_talk2 = Val::from(0);
    let mut l_kir_talk3 = Val::from(0);
    shared::quests_the_sign_quest::f_updatesignvars(ctx, vec![])?;
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Kirkena]")?;
    if (ctx.call(Function::CountItem, vec![Val::from(7313)])? == 1
        && ((ctx.var("sign_q").get()? != 124 || ctx.var("sign_q").get()? != 125) || ctx.var("sign_q").get()? != 126))
    {
        ctx.lines(args![
            "That Witch's Medal...",
            "You must be here to",
            "help your friend take",
            "care of Serin. Are you",
            ((Val::from("ready to go, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
        ])?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes.:Not yet...")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                match runtime::select_values(ctx, &[Val::from("Go directly to Serin:Path Towards Serin")])? {
                    1 => {
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "Thank you",
                                "for your help.",
                                "Serin's threat may",
                                "be too much for just",
                                "one adventurer alone..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_sign01"), Val::from(195), Val::from(189)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "Thank you",
                                "for your help.",
                                "Serin's threat may",
                                "be too much for just",
                                "one adventurer alone..."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("que_sign02"), Val::from(35), Val::from(313)])?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Kirkena", args!["Please hurry...", "Time is of the essence..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    if ctx.var("sign_q").get()? == 90 {
        if ctx.call(Function::CountItem, vec![Val::from(2642)])? == 1 {
            ctx.lines(args![
                "That bastard stole",
                "two spell books from me.",
                "You've brought one of them",
                "back, but the other book",
                "is still missing."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Kirkena",
                args!["Would you please", "retrieve my other", "spell book for me,", "adventurer?"],
            )?;
            ctx.var("sign_sq").set(Val::from(0))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Some bastard stole",
                "two of my spell books.",
                "I'd be grateful if you can",
                "find him in Niflheim and",
                "bring my books back to me."
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sign_q").get()? == 91 {
            if ctx.call(Function::CountItem, vec![Val::from(7304)])?.number()? > 0 {
                ctx.lines(args![
                    "Once that fool stole my spell",
                    "books, he was cursed after he",
                    "cast those spells without my",
                    "permission. I'm so relieved to",
                    "finally hold these in my",
                    "hands again."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "Now mortal...",
                        "Why is it that",
                        "you've come to see me?",
                        "I'm sure that there must",
                        "be something you want."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("He asked me to see you for....")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "As I expected,",
                        "there is something behind",
                        "all of this. Now, you may",
                        "know where you are, but",
                        "do you understand why",
                        "the dead are here?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "This realm is a place",
                        "for warriors that have failed",
                        "to prove their courage. Keep in",
                        "mind that it's not too late for",
                        "you to join their ranks."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "The realm of Niflheim",
                        "is ruled by the Queen of the",
                        "Dead. Sometimes she appears",
                        "in her shining armor and makes the rounds. Everyone who sees her is stunned by the image of authority."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "Now...",
                        "What I need you to do is",
                        "ask the Queen of the Dead",
                        "for the Symbol of Nine Realms."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("What is that?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "The symbol acts as",
                        "a voucher of the queen's",
                        "authority and represents her",
                        "undeniable right to rule over",
                        "the dead. But right now, I can't",
                        "explain why I need it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "I'll tell you why I need",
                        "it once you bring me the",
                        "queen's symbol. Now, please",
                        "keep this secret and tell no one that I asked you to bring the symbol to me."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("How am I supposed to get the symbol?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "Taking the symbol",
                        "by force is out of the",
                        "question. Not even the gods",
                        "would consider battling the",
                        "Queen of the Dead."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "You will need to",
                        "earn the queen's favor",
                        "in order to even have",
                        "a chance of obtaining",
                        "the Symbol of Nine Realms."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirkena",
                    args![
                        "Now, there is a rumor",
                        "that the Queen of the Dead",
                        "is searching for her lost mother, Angrboda. Now, if you could find where Angrboda has been",
                        "sealed away..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(7304), Val::from(1)])?;
                ctx.var("sign_q").set(Val::from(92))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args!["Hmm...?", "Why haven't you", "brought my books", "back to me yet?"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("sign_q").get()? == 92 {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?.number()? < 9 {
                    ctx.lines(args![
                        "If you need to learn more",
                        "about Angrboda, why don't you",
                        "just ask around? I'm sure that",
                        "everyone is aware that the queen",
                        "is searching for her mother."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kirkena",
                        args![
                            "Once again, don't let",
                            "anyone know that I've asked",
                            "you to bring me the Symbol of",
                            "the Nine Realms. I especially",
                            "don't want Serin to find out."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kirkena",
                        args![
                            "If I can help it,",
                            "I won't tolerate",
                            "Serin's presense here",
                            "in Niflheim. If I could",
                            "banish her, I'd do it!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines(args![
                        "I almost hate",
                        "suggesting it, but",
                        "I think Serin might",
                        "have some information",
                        "on Angrboda's whereabouts."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kirkena",
                        args![
                            "But remember to be",
                            "careful around her.",
                            "Something about that",
                            "Serin isn't quite right..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("sign_q").get()?.number()? < 118 {
                    ctx.lines(args![
                        "It may be almost",
                        "impossible to obtain",
                        "the Symbol of Nine Realms.",
                        "Still, you and I both need it."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kirkena",
                        args![
                            "I hope you understand",
                            "that the symbol is more",
                            "than just a mere voucher.",
                            "It actually contains",
                            "enormous power..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("sign_q").get()? == 118 {
                        ctx.lines(args![
                            "The Symbol of Nine Realms?",
                            "How were you able to get that?!",
                            "Ah, you must have found the Queen of the Dead's mother, Angrboda, right? Great work~"
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirkena",
                            args![
                                "Now, if you'll just",
                                "hand me the symbol, I'll tell",
                                "you what we need to do next."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Give the symbol.:Don't give the symbol.")])? {
                            1 => {
                                ctx.lines_as("Kirkena", args!["Listen carefully.", "Since you were actually", "able to get the symbol, I believe that you're the only person capable of performing this next task."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "We're going to use this",
                                        "symbol to create something,",
                                        "but we need one last item.",
                                        "I need you to find something",
                                        "imbued with Serin's vibes."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "Serin's already hid herself",
                                        "deep within Niflheim, but I'm",
                                        "sure you can find something",
                                        "if you can find any remaining",
                                        "trace of her."
                                    ],
                                )?;
                                ctx.call(Function::DelItem, vec![Val::from(7305), Val::from(1)])?;
                                ctx.var("sign_q").set(Val::from(119))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "Huh...?",
                                        "Don't tell me that",
                                        "you've taken that wench's",
                                        "side! Well, I won't take the",
                                        "symbol against your will, but",
                                        "you better think this over..."
                                    ],
                                )?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        if ctx.var("sign_q").get()? == 119 {
                            if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 0 {
                                ctx.lines(args![
                                    "This is Serin's gold ring?",
                                    "Excellent, this has been",
                                    "strongly infused with her",
                                    "vibes. It's really tough to find something like this that resonates so strongly with its owner."
                                ])?;
                                ctx.call(Function::DelItem, vec![Val::from(2642), Val::from(1)])?;
                                ctx.var("sign_q").set(Val::from(120))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines(args![
                                    "Hmm, you don't have",
                                    "anything related to Serin.",
                                    "This isn't good. She's hid",
                                    "herself already and it might",
                                    "be too late to find any trace of her. I'll have to do something..."
                                ])?;
                                ctx.var("sign_q").set(Val::from(121))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if (ctx.var("sign_q").get()? == 120 || ctx.var("sign_q").get()? == 121) {
                                ctx.lines(args![
                                    "Now that the potion I'm",
                                    "making is being processed,",
                                    "I can finally tell you what",
                                    "I've been trying to do with",
                                    "the objects I've asked you",
                                    "to get for me."
                                ])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "I didn't explain the",
                                        "reason I wanted the Symbol",
                                        "of Nine Realms to prevent Serin",
                                        "from getting suspicious and forcing you to reveal my plans. You see, Serin isn't fit for Niflheim."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Kirkena", args!["Although she's in the", "world of the dead, Serin", "has never accepted death and", "has desired to return to life. Of course, that's the natural response for everyone that comes here..."])?;
                                ctx.next()?;
                                ctx.lines_as("Kirkena", args!["However, unlike the others", "who have despaired, lost", "hope and accepted their fate,", "Serin has never given up her hope to return to life. With her hope, she retains her beauty and light."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Kirkena",
                                    args![
                                        "But she's only denying",
                                        "the fate she deserves.",
                                        "What she's trying to do",
                                        "is wrong and we need to",
                                        "stop her before it's too late."
                                    ],
                                )?;
                                ctx.next()?;
                                'l4: loop {
                                    if !(true) {
                                        break 'l4;
                                    }
                                    'b4: {
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("What does she want?:What is she going to do?:What should I do?")],
                                        )? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args!["Make no mistake:", "Serin wants the Symbol", "of the Nine Realms. "],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Now, if she could use it", "multiple times, she could just", "command the guards of Niflheim", "to just let her pass and re-enter the world of the living. But your symbol can be used just once..."])?;
                                                ctx.next()?;
                                                l_kir_talk1 = Val::from(1);
                                                if ((l_kir_talk1.clone() == 1 && l_kir_talk2.clone() == 1) && l_kir_talk3.clone() == 1) {
                                                    if ctx.var("sign_q").get()? == 120 {
                                                        ctx.var("sign_q").set(Val::from(122))?;
                                                    } else if ctx.var("sign_q").get()? == 121 {
                                                        ctx.var("sign_q").set(Val::from(123))?;
                                                    }
                                                }
                                            }
                                            2 => {
                                                ctx.lines_as("Kirkena", args!["I believe Serin is going to", "summon the Dark Lord. Since", "she used to be a great wizard,", "it's entirely possible. However, she might need the Symbol of the Nine Realms in order to do it."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Now, you may believe that", "Dark Lord can be defeated by", "humans, but what you might not", "know is that the manifestation of Dark Lord in your world is limited to a fraction of his true power."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Sometimes Dark Lord can", "cross over into Midgard,", "but traveling from his world into", "ours takes vast amounts of energy. Also the life force in your world conflicts with his dark powers."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "Now, if Dark Lord were",
                                                        "summoned into Niflheim,",
                                                        "he would have easy access",
                                                        "to Midgard and the use",
                                                        "of all of his destructive power."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "Serin plans to strike",
                                                        "a deal with Dark Lord:",
                                                        "In exchange for summoning",
                                                        "him to Niflheim, Serin will",
                                                        "be brought back to life."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "Although you've found",
                                                        "favor with Lady Hell, she",
                                                        "will probably not stop Dark",
                                                        "Lord's approach. Dark Lord",
                                                        "only wants Midgard."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "Now that I think about",
                                                        "it, massive death would",
                                                        "actually increase Lady Hell's",
                                                        "sphere of influence. I don't think she'll help Dark Lord, but she",
                                                        "also won't get in his way."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                l_kir_talk2 = Val::from(1);
                                                if ((l_kir_talk1.clone() == 1 && l_kir_talk2.clone() == 1) && l_kir_talk3.clone() == 1) {
                                                    if ctx.var("sign_q").get()? == 120 {
                                                        ctx.var("sign_q").set(Val::from(122))?;
                                                    } else if ctx.var("sign_q").get()? == 121 {
                                                        ctx.var("sign_q").set(Val::from(123))?;
                                                    }
                                                }
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "I need you to take",
                                                        "this tonic and have",
                                                        "Serin drink it. This",
                                                        "special potion will erase",
                                                        "most of her memories."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "Without her memories,",
                                                        "Serin should also lose",
                                                        "her twisted ambitions.",
                                                        "Now, with the Symbol of Nine Realms, Serin cannot refuse drinking this potion."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Kirkena",
                                                    args![
                                                        "That's why having the",
                                                        "symbol is so crucial.",
                                                        "Serin is clever enough",
                                                        "to avoid drinking this",
                                                        "for quite a while now..."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                l_kir_talk3 = Val::from(1);
                                                if ((l_kir_talk1.clone() == 1 && l_kir_talk2.clone() == 1) && l_kir_talk3.clone() == 1) {
                                                    if ctx.var("sign_q").get()? == 120 {
                                                        ctx.var("sign_q").set(Val::from(122))?;
                                                    } else if ctx.var("sign_q").get()? == 121 {
                                                        ctx.var("sign_q").set(Val::from(123))?;
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                        if (ctx.var("sign_q").get()? == 122 || ctx.var("sign_q").get()? == 123) {
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (ctx.var("sign_q").get()? == 122 || ctx.var("sign_q").get()? == 123) {
                                    ctx.lines(args![
                                        "The potion is finally",
                                        "completed. Now, take this",
                                        "and command Serin to drink it."
                                    ])?;
                                    ctx.next()?;
                                    ctx.mes("[Kirkena]")?;
                                    if ctx.var("sign_q").get()? == 122 {
                                        ctx.lines(args![
                                            "Luckily, I was able to",
                                            "use Serin's gold ring to",
                                            "trace her location. I'm going",
                                            "to send you there, so please",
                                            "do whatever it takes to stop her."
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as("Kirkena", args!["Please take these with you", "as they'll probably be useful", "when you deal with her. As for", "these vouchers, give them to your allies. They'll send them straight to where Serin is located."])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Kirkena",
                                            args![
                                                "When you're finished",
                                                "dealing with Serin, don't",
                                                "forget to bring back all",
                                                "of the vouchers I've",
                                                "given you, alright?"
                                            ],
                                        )?;
                                        ctx.var("sign_q").set(Val::from(124))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7308), Val::from(1)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(2643), Val::from(1)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(7313), Val::from(5)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if ctx.var("sign_q").get()? == 123 {
                                        ctx.lines(args![
                                            "I'm not exactly sure",
                                            "where Serin is hiding,",
                                            "but I'll send you to her",
                                            "general location. Do",
                                            "everything in your",
                                            "power to stop her."
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Kirkena",
                                            args![
                                                "Now, I'm lending you",
                                                "these Witch's Medals. You",
                                                "may lend these to your allies",
                                                "to help you deal with Serin.",
                                                "Remember, if they carry more",
                                                "than one, they won't work."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Kirkena",
                                            args![
                                                "Let your allies know",
                                                "that they have to take",
                                                "the secret passge through",
                                                "the right side of the portrait",
                                                "on the castle's second floor.",
                                                "Then, I can send them to Serin. "
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Kirkena",
                                            args![
                                                "When you're finished",
                                                "dealing with Serin, don't",
                                                "forget to bring back all of",
                                                "the Witch's Medals that",
                                                "I've lent to you, alright?"
                                            ],
                                        )?;
                                        ctx.var("sign_q").set(Val::from(124))?;
                                        ctx.call(Function::GetItem, vec![Val::from(7308), Val::from(1)])?;
                                        ctx.call(Function::GetItem, vec![Val::from(7313), Val::from(5)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ((ctx.var("sign_q").get()? == 124 || ctx.var("sign_q").get()? == 125)
                                        || ctx.var("sign_q").get()? == 198)
                                    {
                                        if ctx.var("$@sign_w1").get()? == 1 {
                                            ctx.lines(args![
                                                "Please wait",
                                                "a bit. I'm still",
                                                "trying to finish",
                                                "these preparations..."
                                            ])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                                            ctx.lines(args![
                                                "You're finally",
                                                "back. We might not",
                                                "have enough time to",
                                                "stop her, but we have",
                                                "to try. Are you ready?"
                                            ])?;
                                            ctx.next()?;
                                            match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                                                1 => {
                                                    ctx.close_window()?;
                                                    ctx.var("$@sign_w1").set(Val::from(1))?;
                                                    ctx.call(
                                                        Function::Warp,
                                                        vec![Val::from("que_sign01"), Val::from(195), Val::from(189)],
                                                    )?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.lines_as("Kirkena", args!["Hurry up, then!", "There's no time", "to waste!"])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            ctx.lines(args![
                                                "We've got to",
                                                "stop Serin before",
                                                "she does something",
                                                "truly horrible. Are",
                                                "you ready to go?"
                                            ])?;
                                            ctx.next()?;
                                            match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                                                1 => {
                                                    ctx.lines_as(
                                                        "Kirkena",
                                                        args![
                                                            "Okay then.",
                                                            "Good luck,",
                                                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                                + Val::from("..."))
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::Warp, vec![Val::from("que_sign02"), Val::from(35), Val::from(313)])?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.lines_as("Kirkena", args!["Hurry up, then!", "There's no time", "to waste!"])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
                                            }
                                        }
                                    } else if (ctx.var("sign_q").get()? == 127 || ctx.var("sign_q").get()? == 128) {
                                        if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                                            if ctx.call(Function::CountItem, vec![Val::from(7313)])?.number()? < 5 {
                                                ctx.lines(args![
                                                    "Hmmm...?",
                                                    "Where are all the",
                                                    "vouchers I've lent to you?",
                                                    "Please retrieve them all",
                                                    "before coming back to me."
                                                ])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines(args![
                                                    "You've done well.",
                                                    "Without her memories of her",
                                                    "previous life, Serin can remain",
                                                    "here in Niflheim peacefully."
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Although she failed to return to life, I'm sure she is glad to have met you. I think she's also started to learn that life's meaning is not in living alone..."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Once again, thank", "you so much for your", "help. Once you finish here,", ((Val::from("please visit our queen, Lady Hell. I believe she wants to see you for some reason, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("..."))])?;
                                                ctx.call(Function::DelItem, vec![Val::from(7313), Val::from(5)])?;
                                                ctx.var("sign_q").set(Val::from(129))?;
                                                ctx.call(Function::GetExperience, vec![Val::from(500000), Val::from(0)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        } else {
                                            if ctx.call(Function::CountItem, vec![Val::from(7313)])?.number()? < 5 {
                                                ctx.lines(args![
                                                    "Hmmm...?",
                                                    "Where are all the",
                                                    "vouchers I've lent to you?",
                                                    "Please retrieve them all",
                                                    "before coming back to me."
                                                ])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines(args![
                                                    "You've done well.",
                                                    "Without her memories of her",
                                                    "previous life, Serin can remain",
                                                    "here in Niflheim peacefully."
                                                ])?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Although she failed to return to life, I'm sure she is glad to have met you. I think she's also started to learn that life's meaning is not in living alone..."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Kirkena", args!["Once again, thank", "you so much for your", "help. Once you finish here,", "please visit our queen, Lady Hell. I believe she wants to see you for some reason..."])?;
                                                ctx.call(Function::DelItem, vec![Val::from(7313), Val::from(5)])?;
                                                ctx.var("sign_q").set(Val::from(130))?;
                                                ctx.call(Function::GetExperience, vec![Val::from(300000), Val::from(0)])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                    } else {
                                        if (ctx.var("sign_q").get()? == 136 || ctx.var("sign_q").get()? == 135) {
                                            ctx.lines(args!["I'm so relieved that", "my expectations about", "you weren't wrong."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kirkena",
                                                args![
                                                    "You've done well.",
                                                    "Without her memories of her",
                                                    "previous life, Serin can remain",
                                                    "here in Niflheim peacefully."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Kirkena", args!["Although she failed to return to life, I'm sure she is glad to have met you. I think she's also started to learn that life's meaning is not in living alone..."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kirkena",
                                                args![
                                                    "Once again, thank",
                                                    "you so much for your",
                                                    "help. Once you finish here,",
                                                    "please visit our queen, Lady Hell. I believe she wants to see you for some reason..."
                                                ],
                                            )?;
                                            if ctx.call(Function::CountItem, vec![Val::from(2643)])? == 1 {
                                                ctx.var("sign_q").set(Val::from(129))?;
                                            } else {
                                                ctx.var("sign_q").set(Val::from(130))?;
                                            }
                                            ctx.call(Function::GetExperience, vec![Val::from(300000), Val::from(0)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("sign_q").get()? == 200 {
                                            ctx.lines(args![
                                                "How could you fail when",
                                                "the stakes are so high?",
                                                "Serin almost summoned",
                                                "the Dark Lord, but luckily,",
                                                "somebody else stopped",
                                                "her in the nick of time."
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines_as("Kirkena", args!["Fortunately, there are many people who are coming to Niflheim as part of the ordeals set before them by the gods. It looks like you just failed that ordeal, and Valkyrie will choose someone else."])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kirkena",
                                                args![
                                                    "It's over. You failed.",
                                                    "Still, consider yourself",
                                                    "lucky. Your soul could have",
                                                    "been bound to Niflheim if it",
                                                    "weren't for Serin's wish for",
                                                    "you to remain free."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Kirkena",
                                                args![
                                                    "You really should thank",
                                                    "her for that. Though, it's",
                                                    "ironic that you failed your",
                                                    "ordeals for her sake..."
                                                ],
                                            )?;
                                            if ctx.call(Function::CountItem, vec![Val::from(2642)])?.number()? > 0 {
                                                ctx.call(Function::DelItem, vec![Val::from(2642), Val::from(1)])?;
                                            }
                                            if ctx.call(Function::CountItem, vec![Val::from(7308)])?.number()? > 0 {
                                                ctx.call(Function::DelItem, vec![Val::from(7308), Val::from(1)])?;
                                            }
                                            ctx.var("sign_q").set(Val::from(201))?;
                                            ctx.call(Function::GetExperience, vec![Val::from(200000), Val::from(0)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if ctx.var("sign_q").get()?.number()? > 200 {
                                            ctx.lines(args!["Hmpf.", "I don't really", "want to talk to", "you anymore."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else if (ctx.var("sign_q").get()?.number()? > 130 && ctx.var("sign_q").get()?.number()? < 136) {
                                            ctx.lines(args!["You've got to hurry", "and stop Serin!"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            ctx.lines(args![
                                                "Hm? Still exploring",
                                                "Niflheim? Although you're",
                                                "strong enough to survive,",
                                                "I still think you're risking",
                                                "your life by remaining here",
                                                "when you don't have to."
                                            ])?;
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
        }
    }
    Ok(Val::from(0))
}

pub fn witch_s(ctx: &Ctx) -> Script {
    witch_s_body(ctx, Vec::new()).map(|_| ())
}
