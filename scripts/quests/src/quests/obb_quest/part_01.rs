use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn tourist_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_th_rand = Val::from(0);
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a moment! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please enlighten your weight -",
            "- and try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("thai_head").get()?.number()? >= 1 && ctx.var("thai_head").get()?.number()? <= 5) {
        'b1: {
            let subject1 = ctx.var("thai_head").get()?;
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1))
                && !subject1.loosely_equals(&Val::from(2))
                && !subject1.loosely_equals(&Val::from(3))
                && !subject1.loosely_equals(&Val::from(4))
                && !subject1.loosely_equals(&Val::from(5));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                l_th_rand = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if (l_th_rand.clone().number()? > 6 && l_th_rand.clone().number()? < 20) {
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["You're that meanie..."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["Aren't you...?"])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["..........."])?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["Alright, alright, but you don't have to be mean."])?;
                    ctx.var("thai_head").set(Val::from(2))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Tourist", args!["..........."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "I didn't think you were that",
                        "mean. Scram, I ain't in the mood",
                        "to talk to you."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Tourist", args![".........."])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "So do you have business in Lutie?:You don't want to go to Lutie for a tour.:What made you come over here?",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Tourist", args!["Yes.", "Well, actually..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "I wanted to give my girlfriend",
                                "some well baked cookies.",
                                "Sadly, in my hometown of",
                                "Morocc, we don't have fancy",
                                "foods like that."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "I've been asking all these",
                                "merchants about it, and I was",
                                "told that I could only get them",
                                "in Lutie...that's my story."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args!["Now I'm running out of money and", "food...I can't stay here much longer..."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Do you want me to help you?:Well, good luck with everything.")],
                        )?) == 1
                        {
                            ctx.var("thai_head").set(Val::from(3))?;
                            ctx.lines_as("Tourist", args!["Thank god! Thank you so much.", "I knew you would help me~!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "Umm...",
                                    "Before we talk about that help...",
                                    "Er, hmmmm...",
                                    "Well, you know I'm pretty",
                                    "much starving to death now..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "Could you please get me some Meat?",
                                    "I know I'm asking you too much,",
                                    "but I'm pretty much penniless",
                                    "so I don't really have much",
                                    "of a choice..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.var("thai_head").set(Val::from(1))?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                ".........",
                                "How could you be so cold",
                                "hearted after hearing my story?",
                                "...You're pretty mean!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "...Oh really? Hmm...people said I",
                                "could only get the well baked",
                                "cookie in Lutie."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "I wanted to give my girlfriend",
                                "some well baked cookies.",
                                "Sadly, in my hometown of",
                                "Morocc, we don't have fancy",
                                "foods like that."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "I've been asking all these",
                                "merchants about it, and I was",
                                "told that I could only get them",
                                "in Lutie...that's my story."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args!["Now I'm running out of money and", "food...I can't stay here much longer..."],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Do you want me to help you?:Well, good luck with everything.")],
                        )?) == 1
                        {
                            ctx.var("thai_head").set(Val::from(3))?;
                            ctx.lines_as("Tourist", args!["Thank god! Thank you so much.", "I knew you would help me~!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "Umm...",
                                    "Before we talk about that help...",
                                    "Er, hmmmm...",
                                    "Well, you know I'm pretty",
                                    "much starving to death now..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "Could you please get me some Meat?",
                                    "I know I'm asking you too much,",
                                    "but I'm pretty much penniless",
                                    "so I don't really have much",
                                    "of a choice..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.var("thai_head").set(Val::from(1))?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                ".........",
                                "How could you be so cold",
                                "hearted after hearing my story?",
                                "...You're pretty mean!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Tourist",
                            args!["I was told that I could only find", "the entrance to Lutie in", "Al De Baran..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "So I came here after searching",
                                "for Al De Baran...almost",
                                "killing myself when I passed the desert...*Sob*"
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
                if ctx.call(Function::CountItem, vec![Val::from(517)])? == 0 {
                    ctx.lines_as(
                        "Tourist",
                        args![
                            "I see...I can understand that you",
                            "can't help me. It's alright...",
                            "It's not like I have the right",
                            "to demand anything of you...",
                            "But..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Tourist", args!["*Sob*...so... hungry..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_th_rand = ctx.call(Function::Rand, vec![Val::from(1), Val::from(100)])?;
                if (l_th_rand.clone().number()? > 0 && l_th_rand.clone().number()? < 10) {
                    if ctx.call(Function::CountItem, vec![Val::from(517)])? == 10 {
                        ctx.lines_as("Tourist", args!["Ah...thanks! Thank you so much!"])?;
                        ctx.call(Function::DelItem, vec![Val::from(517), Val::from(10)])?;
                        ctx.next()?;
                    } else if ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? < 10 {
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "I appreciate that you're",
                                "helping me out of the",
                                "goodness of your heart,",
                                "but..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "What are you...",
                                "Cheap?! You can feed",
                                "a starving man more",
                                "than this...bring me",
                                "more Meat!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? > 10 {
                        ctx.lines_as("Tourist", args!["I appreciate you bringing", "all this for me but..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tourist",
                            args![
                                "Geez, you want I should",
                                "choke myself to death?",
                                "I can't eat all of this!",
                                "Bring me less Meat, yeah?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if (l_th_rand.clone().number()? > 11 && l_th_rand.clone().number()? < 80) {
                        if ctx.call(Function::CountItem, vec![Val::from(517)])? == 20 {
                            ctx.lines_as("Tourist", args!["Ah...thanks! Thank you so much!"])?;
                            ctx.call(Function::DelItem, vec![Val::from(517), Val::from(20)])?;
                            ctx.next()?;
                        } else if ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? < 20 {
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "I appreciate that you're",
                                    "helping me out of the",
                                    "goodness of your heart,",
                                    "but..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "What are you...",
                                    "Cheap?! You can feed",
                                    "a starving man more",
                                    "than this...bring me",
                                    "more Meat!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? > 20 {
                            ctx.lines_as("Tourist", args!["I appreciate you bringing", "all this for me but..."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "Geez, you want I should",
                                    "choke myself to death?",
                                    "I can't eat all of this!",
                                    "Bring me less Meat, yeah?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.call(Function::CountItem, vec![Val::from(517)])? == 40 {
                            ctx.lines_as("Tourist", args!["Ah...thanks! Thank you so much!"])?;
                            ctx.call(Function::DelItem, vec![Val::from(517), Val::from(40)])?;
                            ctx.next()?;
                        } else if ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? < 40 {
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "I appreciate that you're",
                                    "helping me out of the",
                                    "goodness of your heart,",
                                    "but..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "What are you...",
                                    "Cheap?! You can feed",
                                    "a starving man more",
                                    "than this...bring me",
                                    "more Meat!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::CountItem, vec![Val::from(517)])?.number()? > 40 {
                            ctx.lines_as(
                                "Tourist",
                                args![
                                    "Geez, you want I should",
                                    "choke myself to death?",
                                    "I can't eat all of this!",
                                    "Bring me less Meat, yeah?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                ctx.var("thai_head").set(Val::from(4))?;
                ctx.lines(args![
                    "^3355FFHe seemed to have been",
                    "starving for a long time.",
                    "He ate all the meat I gave him",
                    "and it was all gone in a flash.^000000"
                ])?;
                if ctx.call(Function::CountItem, vec![Val::from(538)])?.number()? > 0 {
                    ctx.mes("^0000FF...Oh my! He took my cookies without even asking and ate all of those too!!^000000")?;
                    ctx.call(
                        Function::DelItem,
                        vec![Val::from(538), ctx.call(Function::CountItem, vec![Val::from(538)])?],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "*Phew*...thanks.",
                        "At long last...",
                        "I feel stuffed.",
                        "...Oh. Right.",
                        "My girlfriend."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Er...about the well-baked",
                        "cookie I told you about? Um... I",
                        "know I'm being rude and selfish",
                        "...But I don't think I can find",
                        "it on my own."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Tourist", args!["So, could you bring me ^0000FF20 Well-baked Cookies^000000?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Although I shouldn't be asking for",
                        "any more favors, and you'd have",
                        "to go through all this trouble...",
                        "while I'd be safe over here..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "I really want to make my",
                        "girlfriend happy. Everything",
                        "would be worth it for",
                        "that smile on her face...",
                        "I just can't pass this chance up!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Haven't you been in love...?",
                        "Surely you understand the",
                        "sway that this woman has",
                        "over me! Otherwise...I'd be",
                        "selling jewels...anything",
                        "other than this."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "I'm begging you, since you're",
                        "stronger and less delicate",
                        "than me, please bring me",
                        "^0000FF20 Well-baked Cookies^000000!",
                        "For me, it's impossible!!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(4)) {
                matched1 = true;
            }
            if matched1 {
                if ctx.call(Function::CountItem, vec![Val::from(538)])?.number()? > 19 {
                    ctx.lines_as(
                        "Tourist",
                        args![
                            "Ah~!! Thank you so much!!",
                            "There really are many good",
                            "people like you in this world."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("thai_head").set(Val::from(5))?;
                    ctx.call(Function::DelItem, vec![Val::from(538), Val::from(20)])?;
                    ctx.lines_as(
                        "Tourist",
                        args![
                            "I appreciate what you've done for",
                            "me...now I can see her smile with",
                            "happiness."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tourist",
                        args![
                            "Hmm, there is still one problem",
                            "left for me though...How do I",
                            "get back home to Morocc??",
                            "Well, if it can make my girl",
                            "happy..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tourist",
                        args![
                            "...I'll get around to it.",
                            "Sooner or later. Man,",
                            "that desert's really big,",
                            "you know?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Tourist",
                    args![
                        "I see...I can understand that you",
                        "can't help me. It's alright...",
                        "It's not like I have the right",
                        "to demand anything of you...",
                        "But..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched1 && subject1.loosely_equals(&Val::from(5)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Tourist", args!["Thank you so much!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Hmm...I wanna pay you back...",
                        "And you're an adventurer...",
                        "But I don't got any like...",
                        "magic armor or those",
                        "cards you guys are",
                        "always playing with..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Hmmm...",
                        "....I got it!",
                        "Sooner or later, you",
                        "wanna visit some sort",
                        "of wise man or...",
                        "...something, right?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Visit my pal ^649664Jacob^000000 in",
                        "Morocc. Tell him that I",
                        "introduced you. He knows",
                        "about a wise man...",
                        "Though, he isn't one",
                        "himself..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Ah, sorry, I haven't even told",
                        "you my name. My name is...",
                        "^0000FFPandger Mayer^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args!["So yeah, tell my pal Jacob", "that Pandger Mayer introduced you."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "So...you tell 'em:",
                        "^0000FFPandger Mayer introduced you^000000",
                        "Exactly like that~",
                        "^0000FFPandger Mayer introduced you^000000",
                        "^FF0000Don't forget it~^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args!["Sorry I can't pay you back with", "more than this, though. Kinda", "stinks, huh?"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Here, take this as an extra",
                        "token of my gratitute.",
                        "This doohickey has been",
                        "pretty handy in sticky",
                        "situations..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Tourist", args!["Hopefully, it'll be useful."])?;
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 500 {
                    ctx.lines(args![
                        "Umm...it seems you're carrying too much stuff with you.",
                        "Doncha want me to help you?"
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.var("thai_head").set(Val::from(6))?;
                ctx.call(Function::GetItem, vec![Val::from(1205), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as("Tourist", args!["Heh...once gain, thanks for", "the help. Take care, guy."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("thai_head").get()?.number()? > 5 {
        let subject3 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
        if subject3 == 1 {
            ctx.lines_as(
                "Tourist",
                args![
                    "Hey, thanks a bunch for your",
                    "help. Anyways...now I remember",
                    "I gotta go back through the",
                    "desert again...*Sigh*",
                    "Yeah, I better get back..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tourist",
                args![
                    "...no matter how hard it is.",
                    "But...yeah. It's gonna be pretty",
                    "hard. Crossing the desert...",
                    "alone. By myself...crud."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject3 == 2 {
            ctx.lines_as(
                "Tourist",
                args![
                    "Hey, thanks a bunch for your",
                    "help....I ain't sure if I",
                    "can make it back to Morocc",
                    "in one piece..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tourist",
                args![
                    "But I better go back",
                    "to my girlfriend. That's",
                    "the point of me coming",
                    "here in the first place,",
                    "anyway."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject3 == 3 {
            ctx.lines_as(
                "Tourist",
                args![
                    "Hey, thanks a bunch for your",
                    "help. Man...Morocc is a long",
                    "way off, isn't it? Aw nuts...",
                    "I guess if I was crazy",
                    "enough to come here because",
                    "of my girlfriend..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tourist",
                args![
                    "I'm crazy enough to walk all",
                    "way back to Morocc...",
                    "...",
                    "...unless...",
                    "You'll gimme a piggy-back ride?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tourist",
                args![
                    "...kidding. I'm not that",
                    "big of a jerk, I guess.",
                    "Try not to look so",
                    "surprised!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("JobLevel").get()?.number()? < 36 {
            ctx.lines_as("Tourist", args!["Where am I...?", "...Who am I?"])?;
            ctx.next()?;
            ctx.lines_as("Tourist", args!["........."])?;
            ctx.next()?;
            ctx.lines_as("Tourist", args!["....."])?;
            ctx.next()?;
            ctx.lines_as("Tourist", args!["........."])?;
            ctx.next()?;
            ctx.lines_as("Tourist", args!["Mahahahahahaha!!"])?;
            ctx.next()?;
            ctx.lines_as("Tourist", args!["*Drools*...I love chocolate!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Tourist",
                args![
                    "Wait...",
                    "I have a girlfriend...",
                    "and...and...",
                    "cookies...?",
                    "I feel sooo lost..."
                ],
            )?;
            ctx.next()?;
            ctx.mes("^3355FFWe don't talk to crazy people~^000000")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Tourist",
            args!["Where am I?", "*Sob*...I guess I got lost...", "Hey, where is this!?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Ignore him.:This is Alberta.:This is Al De Baran.")])? {
            1 => {
                ctx.lines_as("Tourist", args![".........."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Ah...I see. So this is",
                        "Alberta...crud. I'm",
                        "supposed to go to ^0000FFLutie^000000..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Hey, thanks for letting me know.",
                        "Now at least I know where I am.",
                        "Geez...they got boats here",
                        "that go everywhere, yeah?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Tourist", args!["Oh right, everywhere...but Lutie.", "Man, this stinks."])?;
                ctx.close_window()?;
                ctx.var("thai_head").set(Val::from(2))?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Tourist",
                    args!["Oh...oh right!", "Yeah, Al De Baran...", "Guess I'm not lost", "after all."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tourist",
                    args![
                        "I was told that I can see a ^FF0000Clock Tower^000000 in Al De Baran.",
                        "Can you tell me where I can find the ^FF0000Clock Tower^000000?"
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "It's in the center of the town.:Eh, I think I gave you the wrong town name.",
                    )],
                )?) == 1
                {
                    ctx.lines_as(
                        "Tourist",
                        args![
                            "..............",
                            "You know, I'm beginning",
                            "to feel like...",
                            "you're pullin' my",
                            "chain..."
                        ],
                    )?;
                    ctx.var("thai_head").set(Val::from(1))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Tourist",
                    args![
                        "Hah~!...I knew it! Well, it's",
                        "pretty hard to admit when",
                        "you do something wrong.",
                        "Thanks for being honest, pal."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn tourist_thai(ctx: &Ctx) -> Script {
    tourist_thai_body(ctx, Vec::new()).map(|_| ())
}

fn jacob_thai_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input_s = Val::from("");
    let mut l_rem = Val::from(0);
    let mut l_remrem = Val::from(0);
    if (ctx.var("thai_head").get()?.number()? >= 6 && ctx.var("thai_head").get()?.number()? <= 11) {
        let subject1 = ctx.var("thai_head").get()?;
        if subject1 == 6 {
            ctx.lines_as("Tommy", args!["Wahhhh~~~!!", "Dad~~ let me have a Munak~~~!", "Waaahhhh!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Tommy",
                args!["I want Munak, Munakkkk~~!!", "Other people have a Munak, why can't I~~?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["Munak Munak Munak Munak~~~!"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["Er...Ummm..."])?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["Munak Munak Munak Munak~~~!"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["Tommy, daddy is kind of busy right now..."])?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["Munak Munak Munak Munak~~~!"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["*Sigh*...alright...alright. If you want it that bad..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "Gosh...being a dad is not an easy",
                    "thing to do...Hmmm? Do you have any business with me?",
                    " ",
                    "[Tommy]",
                    "^FF0000Munak Munak Munak Munak!!^000000"
                ],
            )?;
            ctx.next()?;
            let (input, status) = runtime::input_text(ctx, None, None)?;
            l_input_s = input;
            if l_input_s.clone() != "Pandger Mayer introduced you" {
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![((Val::from("") + l_input_s.clone()) + Val::from(""))],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args!["....Sorry, what did you say? I have no idea what you're talking about."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Pandger Mayer introduced you."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "Oh, you mean that guy in love?",
                    "Last time I saw him, he was",
                    "leaving to find cookies somewhere..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "There is only one reason he could have mentioned my name to you...",
                    " ",
                    " ",
                    "[Tommy]",
                    "^FF0000Munak Munak Munak Munak!!^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "......",
                    "*Sigh* For now, I have a favor to",
                    "ask of you. Could you bring me",
                    "^0000FF1 No Recipient^000000?"
                ],
            )?;
            ctx.var("thai_head").set(Val::from(7))?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "I need this kid to calm down first...",
                    " ",
                    " ",
                    "[Tommy]",
                    "^FF0000Munak Munak Munak Munak!!^000000"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 7 {
            if ctx.call(Function::CountItem, vec![Val::from(636)])?.number()? > 0 {
                ctx.call(Function::DelItem, vec![Val::from(636), Val::from(1)])?;
                ctx.lines_as("Tommy", args!["^FF0000Munak Munak Munak Munak!!^000000"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "Oh, thank you so much...",
                        "Let me calm this kid down",
                        "first. Sorry my little boy is being so loud."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Jacob", args!["Tommy, Tommy?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Tommy",
                    args![
                        "^FF0000Moo^9F0000naah^5F0000ahhh^3F0000ahhh^000000kkk!!..Err?",
                        "What's this, Daddy?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args!["This is called No Recipient~", "With this, I can get you a Munak~"],
                )?;
                ctx.next()?;
                ctx.lines_as("Tommy", args!["Are you serious, Daddy?!"])?;
                ctx.next()?;
                ctx.lines_as("Jacob", args!["Yes, I am~", "Have you ever heard me lie?"])?;
                ctx.next()?;
                ctx.lines_as("Tommy", args![".......", "I know you always lie to Mommy."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "..........",
                        "No no no no no...that's not",
                        "considered lying, Tommy.",
                        "I am just trying to make it sound better to Mommy~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Tommy", args!["......."])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFHe looks at his father in",
                    "doubt. Kids are too smart",
                    "to lie to nowadays...^000000"
                ])?;
                ctx.next()?;
                ctx.mes("[Tommy]")?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.mes("So, mister, do you think my Daddy can catch a Munak with this?")?;
                } else {
                    ctx.mes("So, lady, do you think my Daddy can catch a Munak with this?")?;
                }
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Yes.:...not sure.:No, ^FF0000never^000000.")],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1))
                        && !subject2.loosely_equals(&Val::from(2))
                        && !subject2.loosely_equals(&Val::from(3));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as("Tommy", args!["Heh!! I knew my Daddy", "wouldn't tell me a lie!! Thank you!!"])?;
                        ctx.next()?;
                        ctx.lines_as("Jacob", args!["(...*Phew!* Thank you~)"])?;
                        ctx.var("thai_head").set(Val::from(9))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.mes("[Tommy]")?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("So tell me, mister. Did he lie to me, then?")?;
                        } else {
                            ctx.mes("So tell me, lady. Did he lie to me, then?")?;
                        }
                        ctx.next()?;
                        ctx.lines_as(
                            "Jacob",
                            args![
                                "Wait...wait, Tommy, that's not true...",
                                "(^0000FFGosh, why are you doing this to me, help me already!!^000000)"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tommy",
                            args!["Can we catch a Munak with this or", "not, huh? Tell meeeeehhhh~~~!"],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Yes, we can.:No, we can't.")])? {
                            1 => {
                                ctx.lines_as("Tommy", args!["Heh heh! I knew it!", "Thank you~!!!"])?;
                                ctx.next()?;
                                ctx.lines_as("Tommy", args!["Now I can have a Munak!"])?;
                                ctx.next()?;
                                ctx.lines_as("Jacob", args!["(...*Phew* Thank you~)"])?;
                                ctx.var("thai_head").set(Val::from(9))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.var("thai_head").set(Val::from(8))?;
                                ctx.lines_as("Tommy", args!["I knew it!! Daddy, you're a liar!!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tommy",
                                    args![
                                        "I hate you...I hate you!!!",
                                        "I wish Baphomet would take you somewhere and eat you!!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Jacob", args!["Shoot...Tommy, Tommy...", "Listen to me...."])?;
                                ctx.next()?;
                                ctx.lines_as("Tommy", args!["I don't wanna listen to you!!!", "I hate yoooooouhhhh!!"])?;
                                ctx.next()?;
                                ctx.lines_as("Jacob", args!["....You."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Jacob",
                                    args![
                                        "I don't want to see your ugly face",
                                        "around here any more...",
                                        "Leave~!",
                                        "How could you do this to me, huh?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Tommy",
                                    args![
                                        "I hate yoooooouhhhh!!",
                                        " ",
                                        " ",
                                        "[Jacob]",
                                        "Tommy...Tommy please...",
                                        "Will you let me explain...?"
                                    ],
                                )?;
                                ctx.var("thai_head").set(Val::from(8))?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.var("thai_head").set(Val::from(8))?;
                        ctx.lines_as("Tommy", args!["I knew it!! Daddy, you're a liar!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tommy",
                            args![
                                "I hate you...I hate you!!!",
                                "I wish Baphomet will take you somewhere and eat you!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Jacob", args!["Shoot...Tommy, Tommy...", "Listen to me...."])?;
                        ctx.next()?;
                        ctx.lines_as("Tommy", args!["I don't wanna listen to you!!!", "I hate yoooooouhhhh!!"])?;
                        ctx.next()?;
                        ctx.lines_as("Jacob", args!["....You."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Jacob",
                            args![
                                "I don't want to see",
                                "your ugly face any more.",
                                "Leave!",
                                "How could you do this to me, huh?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Tommy",
                            args![
                                "I hate yoooooouhhhh!!",
                                "[Jacob]",
                                "Tommy...Tommy, please...",
                                "Will you let me explain...?"
                            ],
                        )?;
                        ctx.var("thai_head").set(Val::from(8))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
            ctx.lines_as("Tommy", args!["^FF0000Munak Munak Munak Munak!!^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args!["*Sigh*...that's my boy. Please get me a No Recipient as soon as you can!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 8 {
            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(1000)])? == 792 {
                ctx.lines_as("Tommy", args!["^FF0000Munak Munak Munak Munak!!^000000"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "....*Sigh* Okay.",
                        "I will give you a chance",
                        "to make up for your mistake.",
                        "Please get me a No Recipient as soon as you can."
                    ],
                )?;
                ctx.var("thai_head").set(Val::from(7))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Jacob",
                args![
                    "....I don't want to talk to YOU",
                    "anymore. Get outta here~!",
                    "It's your fault my little",
                    "boy thinks I'm a big liar!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 9 {
            ctx.lines_as(
                "Jacob",
                args![
                    "*Phew* Now I can leave him alone.",
                    "Kids nowadays are so impatient...I am worried about my son."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "However, I can't do anything",
                    "about his temper...I think",
                    "he's just too young to know",
                    "what he's doing."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "I'm having a hard time trying to",
                    "be a good parent, and I'm",
                    "beginning to appreciate my",
                    "own parents' efforts to raise me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "Respect your parents while they're",
                    "still alive. It's too late to be",
                    "sorry after you've lost them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "I was a wild kid as well...my",
                    "parents had a hard time raising",
                    "me. I'm ashamed of what I've done",
                    "when I was young..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "Those days, my parents couldn't",
                    "afford to feed me three meals a",
                    "day. And there were many",
                    "creditors that always came",
                    "to our house early",
                    "early in the morning."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "Have you every heard about",
                    "distraint-papers? As soon",
                    "as those creditors put that",
                    "distraint-paper on any of my",
                    "things,it would no longer be",
                    "mine."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "I hated my parents for not providing me with a sweet home,",
                    "and I became rebellious.",
                    "But, no matter how bad I was,",
                    "Even in those difficult circumstances..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "They never gave up on me.",
                    "Later, they told me that I was their one and only hope...",
                    "even though I was more of a burden than anything else at the moment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Tommy",
                args![
                    "Daddy, do you realize you've told",
                    "that story hundreds of times already? *Piff~*",
                    "Stop it already~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFTommy was jumping and running",
                "around like a crazy rabbit and",
                "eventually bumped against me.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args!["...Tommy, respect your dad.", "This is the story of your father's life."],
            )?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["A.aa.aand.!!!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args!["How many times have I told you", "to behave in front of other people?!"],
            )?;
            ctx.next()?;
            ctx.mes("[Jacob]")?;
            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                ctx.mes("Say you're sorry to him!")?;
            } else {
                ctx.mes("Say you're sorry to her!")?;
            }
            ctx.next()?;
            ctx.lines_as("Tommy", args!["*Piff~*"])?;
            ctx.next()?;
            ctx.lines_as("Jacob", args!["I don't hear it~!!"])?;
            ctx.next()?;
            ctx.lines_as("Tommy", args!["...I'm sorry...*bows head*"])?;
            ctx.var("thai_head").set(Val::from(10))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 10 {
            l_rem = ctx.call(Function::Rand, vec![Val::from(1), Val::from(11)])?;
            if l_rem.clone() == 1 {
                ctx.lines_as(
                    "Jacob",
                    args![
                        "Ah~ yes! Didn't you say Pandger Mayer introduced me to you?",
                        "He wouldn't introduce me to just anyone. Oh, you must be",
                        "...an adventurer."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Jacob", args!["Right.", "I have something to tell you."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "When you go west from Morocc, and",
                        "pass through a cave, you will",
                        "arrive at a town called Comodo."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "When you get there, you will see an old man named ^0000FFElder Creek^000000.",
                        "As a Sage, he has gained the respect of many."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "People say that many adventurers",
                        "are visiting this man for some",
                        "reason. Why don't you go talk to him and see if he has some wisdom for you?"
                    ],
                )?;
                ctx.var("thai_head").set(Val::from(11))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (l_rem.clone().number()? > 1 && l_remrem.clone().number()? < 7) {
                ctx.lines_as(
                    "Jacob",
                    args![
                        "Respect your parents when they're",
                        "still alive. It's too late to be sorry after you've lost them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["(Hmmm...I just know that there's more that this guy can tell me...)"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Jacob",
                    args!["When you have a kid,", "don't forget to teach them", "this one thing:"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jacob",
                    args![
                        "'^0000FFBe good to other people.^000000'",
                        "'^0000FFTry to be in someone else's shoes before judging that person.^000000'",
                        "Do you understand?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["(There's more this guy can tell me, I just know it...)"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if subject1 == 11 {
            ctx.lines_as(
                "Jacob",
                args!["Now, why don't you visit the Sage?", "And I thank you for your kindness."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Jacob",
                args![
                    "He's in Comodo, if you didn't",
                    "catch that. An adventurer like you should be able to get there with no problem."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("thai_head").get()?.number()? > 11 {
        ctx.lines_as(
            "Jacob",
            args![
                "Thank you for the favour you did",
                "for me...although we couldn't",
                "catch a Munak in the end..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jacob",
            args![
                "It seems he learned something",
                "through the experience.",
                "He no longer demands something",
                "I cannot do for him, or",
                "misbehaves."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Jacob",
            args!["That was another favor you did for me, even if you didn't mean to. Thank you."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Jacob",
            args![
                "*Yawns*....",
                "Today is such a boring day~.",
                "I guess I'd better go take a walk",
                "with my son. Do you want to walk with me?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Jacob's son", args!["I'm happy to hear that, daddy~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn jacob_thai(ctx: &Ctx) -> Script {
    jacob_thai_body(ctx, Vec::new()).map(|_| ())
}
