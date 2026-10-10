use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn mr_jun_ayo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please enlighten your weight -",
            "- and try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tomyumgoong").get()? == 5 {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
        ctx.lines_as("Mr. Jun", args!["Umm...?", "You don't need", "more Lemons", "again, do you?"])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("No.:Give me Lemons, old man!:Can I have some more Lemons, please?")],
        )? {
            1 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "Then why don't you taste",
                        "the Lemons I gave you last time? They're really sour, but also very delicious!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "What...!?",
                        "What's wrong with you, kid?",
                        "What have you done with the",
                        "Lemons I gave you last time?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "Oh, I am sorry....recently my lemon storage",
                        "was attacked by ^3131FF'Leaf Cats'^000000 that are roaming around the Shrine."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "I wish that I could help you,",
                        "but I don't even have enough lemons for my business.",
                        "If you need lemons, why don't you go hunt them?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    ctx.lines_as("Mr. Jun", args!["..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mr_jun_ayo(ctx: &Ctx) -> Script {
    mr_jun_ayo_body(ctx, Vec::new()).map(|_| ())
}

fn mr_jun_ayo_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_jun_mark = Val::from(0);
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please enlighten your weight -",
            "- and try again. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tomyumgoong").get()? == 4 {
        ctx.lines_as("Mr. Jun", args!["Hello, there?", "Did you need", "some help...?"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SCRATCH")?])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("I need some Lemons.:No, thanks.")])?) == 1 {
            ctx.lines_as(
                "Mr. Jun",
                args![
                    "Umm...",
                    "Lemons?",
                    "As you see, I have plenty",
                    "of Lemons on my tree.",
                    "Do you really need some?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes, I will pay you.:No, thanks.")])?) == 1 {
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "I started growing this Lemon tree as a hobby, but I didn't expect to have such a good harvest of sweet,",
                        "succulent Lemons."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "Did you just say you were",
                        "going to pay me for the Lemons?",
                        "Oh, you've got me wrong. I don't intend to sell these."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "But...",
                        "I'm more than willing",
                        "to share some of them with",
                        "you, but only if you can..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Jun",
                    args![
                        "Play a small game with me!",
                        "How about that? I've been so bored to death. Even an old man like me needs to enjoy himself!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Mr. Jun", args!["Hahahaha~!", "What do you say?", "Would you like to try?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Sure, why not!:No, thanks.")])?) == 1 {
                    ctx.lines_as(
                        "Mr. Jun",
                        args!["Good!", "The game I want us", "to play is an easy", "children's game."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Jun",
                        args![
                            "^660000Rock,",
                            "Paper, Scissors!^000000",
                            "Perhaps you've heard",
                            "of it in your land."
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SCISSOR")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Jun",
                        args![
                            "Nuh-uh, don't look at me like that. I'm an old man, and I want to play this game and re-live my youth",
                            "a little bit. You'll understand when you're my age."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Jun",
                        args![
                            "Alright, if you win",
                            "^3131FF3^000000 out of ^3131FF5^000000 matches,",
                            "The Lemons are yours.",
                            "Are you ready?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Jun",
                        args!["Okay...", "As you kids say,", "^990000Let's get eXtreme!^000000"],
                    )?;
                    ctx.next()?;
                    ctx.var("@user_score").set(Val::from(0))?;
                    ctx.var("@pc_score").set(Val::from(0))?;
                    ctx.lines_as("Mr. Jun", args!["Rock!", "Paper!", "Scissors!"])?;
                    ctx.next()?;
                    'l1: loop {
                        if !(true) {
                            break 'l1;
                        }
                        'b1: {
                            if (ctx.var("@pc_score").get()? == 5 || ctx.var("@user_score").get()? == 3) {
                                break 'l1;
                            } else {
                                l_jun_mark = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                match runtime::select_values(ctx, &[Val::from("Scissors:Rock:Paper")])? {
                                    1 => {
                                        if l_jun_mark.clone() == 1 {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SCISSOR")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_SCISSOR")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                "Mr. Jun",
                                                args!["Hmpf.", "It's a draw.", "One more time!", "Rock! Paper!", "Scissors!"],
                                            )?;
                                            ctx.next()?;
                                        } else if l_jun_mark.clone() == 2 {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_ROCK")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_SCISSOR")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.var("@pc_score").set((ctx.var("@pc_score").get()? + Val::from(1)))?;
                                            ctx.lines_as("Mr. Jun", args!["Yes...!", "Oh my God, yes!"])?;
                                            ctx.next()?;
                                        } else {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_WRAP")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_SCISSOR")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.var("@pc_score").set((ctx.var("@pc_score").get()? + Val::from(1)))?;
                                            ctx.var("@user_score").set((ctx.var("@user_score").get()? + Val::from(1)))?;
                                            ctx.lines_as("Mr. Jun", args!["What...?", "I don't believe it!", "Bah.. !"])?;
                                            ctx.next()?;
                                        }
                                    }
                                    2 => {
                                        if l_jun_mark.clone() == 1 {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SCISSOR")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_ROCK")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.var("@pc_score").set((ctx.var("@pc_score").get()? + Val::from(1)))?;
                                            ctx.var("@user_score").set((ctx.var("@user_score").get()? + Val::from(1)))?;
                                            ctx.lines_as("Mr. Jun", args!["No...!", "Sacrilege!"])?;
                                            ctx.next()?;
                                        } else if l_jun_mark.clone() == 2 {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_ROCK")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_ROCK")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                "Mr. Jun",
                                                args!["Hmpf.", "It's a draw.", "One more time!", "Rock! Paper!", "Scissors!"],
                                            )?;
                                            ctx.next()?;
                                        } else {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_WRAP")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_ROCK")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.var("@pc_score").set((ctx.var("@pc_score").get()? + Val::from(1)))?;
                                            ctx.lines_as("Mr. Jun", args!["Bwahahaha!", "You lose, kid!"])?;
                                            ctx.next()?;
                                        }
                                    }
                                    3 => {
                                        if l_jun_mark.clone() == 1 {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_SCISSOR")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_WRAP")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.var("@pc_score").set((ctx.var("@pc_score").get()? + Val::from(1)))?;
                                            ctx.lines_as("Mr. Jun", args!["Heh heh~", "Young fool!"])?;
                                            ctx.next()?;
                                        } else if l_jun_mark.clone() == 2 {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_ROCK")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_WRAP")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.var("@user_score").set((ctx.var("@user_score").get()? + Val::from(1)))?;
                                            ctx.var("@pc_score").set((ctx.var("@pc_score").get()? + Val::from(1)))?;
                                            ctx.lines_as("Mr. Jun", args!["Noooo...!", "This cannot be!"])?;
                                            ctx.next()?;
                                        } else {
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_WRAP")?])?;
                                            ctx.call(
                                                Function::Emotion,
                                                vec![
                                                    ctx.constant("ET_WRAP")?,
                                                    Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                "Mr. Jun",
                                                args!["Hmpf.", "It's a draw.", "One more time!", "Rock! Paper!", "Scissors!"],
                                            )?;
                                            ctx.next()?;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    if ctx.var("@user_score").get()? == 3 {
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_HNG")?])?;
                        ctx.lines_as(
                            "Mr. Jun",
                            args![
                                "You... won.",
                                "I'm so exhausted.",
                                "I guess I can't compete",
                                "with you youngsters",
                                "anymore. Ha ha ha!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Jun",
                            args![
                                "As promised,",
                                "I shall share some",
                                "of my Lemons with you.",
                                "Grab as many as you want!",
                                "^3355FFYou have plucked",
                                "10 Lemons from the tree.^000000"
                            ],
                        )?;
                        ctx.var("tomyumgoong").set(Val::from(5))?;
                        ctx.call(Function::GetItem, vec![Val::from(568), Val::from(10)])?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
                        ctx.lines_as(
                            "Mr. Jun",
                            args![
                                "Hmm...",
                                "Looks like you need a lot of them. Oh well, that's fine with me. I had a good time with you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Jun", args!["Don't forget this though:", "Don't be selfish, and always share what you have with others. That's one of the most important values for human beings."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SCISSOR")?])?;
                    ctx.lines_as(
                        "Mr. Jun",
                        args![
                            "Muhahahaha!",
                            "Victory is mine!",
                            "Looks like you won't be getting",
                            "any Lemons for now! But feel free to challenge me anytime!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Mr. Jun",
                    args!["Afraid are we?", "There's no fooling you:", "I'm one of the best at games!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Mr. Jun",
                args!["Hmm...", "Now what can", "I possibly do", "with all these", "Lemons?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as("Mr. Jun", args!["Feel free to", "ask me for help", "any time, okay?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Mr. Jun", args!["..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mr_jun_ayo_ontouch(ctx: &Ctx) -> Script {
    mr_jun_ayo_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn merchant_ayo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("tomyumgoong").get()? == 6 {
        ctx.lines_as(
            "Merchant Thongdum",
            args!["Hello, there~", "Are you looking for", "ingredients to make", "Tom Yum Goong?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Merchant Thongdum",
            args![
                "Let's see...",
                "I'm selling chiles,",
                "fish sauce, and some",
                "spices and flavorings",
                "you'll need. I'll sell",
                "it all to you for 2,000 zeny."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Thanks, I'll take it.:It's a rip-off, man!")],
        )?) == 1
        {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THANKS")?])?;
            if ctx.var("Zeny").get()?.number()? > 1999 {
                if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 1199 {
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(2000))?))?;
                    ctx.var("tomyumgoong").set(Val::from(7))?;
                    ctx.call(Function::GetItem, vec![Val::from(7286), Val::from(30)])?;
                    ctx.lines_as(
                        "Merchant Thongdum",
                        args!["Thank you.", "I hope you will enjoy", "your Tom Yum Goong~"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Merchant Thongdum",
                    args!["Uh oh...", "It looks like", "you're carrying", "too much stuff", "right now."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Merchant Thongdum",
                    args!["I can't give you", "anything if you don't", "have the room to hold it!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Merchant Thongdum",
                args![
                    "Hmm...?",
                    "Right now, it looks",
                    "like you don't have the",
                    "money to buy what you",
                    "need from me."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.call(Function::Emotion, vec![ctx.constant("ET_STARE_ABOUT")?])?;
        ctx.lines_as(
            "Merchant Thongdum",
            args![
                "Don't say that.",
                "My prices are always",
                "reasonable. I want you to know",
                "that I'm a respectable Merchant."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("tomyumgoong").get()? == 7 {
        ctx.lines_as(
            "Merchant Thongdum",
            args![
                "I thought I provided you",
                "with enough ingredients, but",
                "now that I think about it, I don't",
                "know how much Tom Yum Goong",
                "you're making."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Merchant Thongdum", args!["Would you like to", "buy some more Chilis?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("No, thanks.:Yes, please.")])?) == 1 {
            ctx.lines_as("Merchant Thongdum", args!["Fare well~"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("Zeny").get()?.number()? > 1999 {
            if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 1199 {
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(2000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(7286), Val::from(30)])?;
                ctx.lines_as("Merchant Thongdum", args!["Thank you,", "come again!"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Merchant Thongdum",
                args!["Uh oh...", "It looks like", "you're carrying", "too much stuff", "right now."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Merchant Thongdum",
                args!["I can't give you", "anything if you don't", "have the room to hold it!"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Merchant Thongdum",
            args![
                "Hmm...?",
                "Right now, it looks",
                "like you don't have the",
                "money to buy what you",
                "need from me."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Merchant Thongdum", args!["Hello there!", "Ever try a Chili before?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant Thongdom",
        args![
            "Despite its tiny appearance,",
            "it has a very strong flavor. It's also hot and spicy, so you should be careful when you eat one."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Merchant Thongdum", args!["The people of Ayothaya enjoy sensational food with distinct, spicy flavors. In Ayothayan cuisine, we use things like Chilis, garlic, ginger and cilantro."])?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant Thongdum",
        args!["I'm selling Chilis,", "so if you need any,", "please come to me~"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn merchant_ayo(ctx: &Ctx) -> Script {
    merchant_ayo_body(ctx, Vec::new()).map(|_| ())
}
