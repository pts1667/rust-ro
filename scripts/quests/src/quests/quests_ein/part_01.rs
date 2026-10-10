use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn cavitar_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ein_gear1").get()? == 2 {
        ctx.lines_as(
            "Cavitar",
            args![
                "It's been a long",
                "time, my friend. With",
                "your help, my wife has",
                "gotten much better."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cavitar",
            args![
                "Once again, I want to",
                "thank you for what you've",
                "done for me. Please drop",
                "by anytime whenever you",
                "come visit my town."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ein_gear1").get()? == 1 {
        ctx.lines_as(
            "Cavitar",
            args!["Oh, welcome.", "So did you bring", "everything that", "I've asked you?"],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No, not yet.")])? {
            1 => {
                if (((ctx.call(Function::CountItem, vec![Val::from(1011)])?.number()? > 9
                    && ctx.call(Function::CountItem, vec![Val::from(1003)])?.number()? > 2)
                    && ctx.call(Function::CountItem, vec![Val::from(912)])?.number()? > 9)
                    && ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 39)
                {
                    ctx.lines_as(
                        "Cavitar",
                        args!["Let me check the items", "that you brought before", "I give you an Oridecon."],
                    )?;
                    ctx.next()?;
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? > 2399 {
                        ctx.lines_as(
                            "Cavitar",
                            args![
                                "Good, you brought",
                                "everything! Now let me",
                                "give you this Oridecon.",
                                "I'm sorry that I can't offer",
                                "you much more than this."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Cavitar", args!["Alright then,", "adventurer. I hope", "you travel in safety."])?;
                        ctx.call(Function::DelItem, vec![Val::from(1003), Val::from(3)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7126), Val::from(40)])?;
                        ctx.call(Function::DelItem, vec![Val::from(1011), Val::from(10)])?;
                        ctx.call(Function::DelItem, vec![Val::from(912), Val::from(10)])?;
                        ctx.var("ein_gear1").set(Val::from(0))?;
                        ctx.call(Function::GetItem, vec![Val::from(615), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Cavitar",
                            args![
                                "Hmmm, it seems you're",
                                "carrying too many things",
                                "with you. Shouldn't you",
                                "store some of your items",
                                "in Kafra Storage first?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as(
                        "Cavitar",
                        args![
                            "Hm...?",
                            "You're missing some",
                            "things. Let me tell you",
                            "what you need to bring",
                            "once again so that I can",
                            "give you 1 Oridecon."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cavitar",
                        args![
                            "40 Large Jellopy,",
                            "10 Emveretarcon,",
                            "3 Coal and",
                            "10 Zargon.",
                            "Please don't",
                            "forget this."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            2 => {
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "Oh, alright.",
                        "Let me remind",
                        "you of what you",
                        "need to trade for",
                        "1 Oridecon in",
                        "case you've forgotten."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "40 Large Jellopy,",
                        "10 Emveretarcon,",
                        "3 Coal and",
                        "10 Zargon.",
                        "Come back",
                        "whenever you're ready."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("BaseLevel").get()?.number()? < 40 {
        ctx.lines_as(
            "Cavitar",
            args![
                "Recently, we've had some",
                "tunnel cave-ins which resulted",
                "in miner casualties. We're having",
                "a harder time working in the",
                "mines now that we're even",
                "lower on manpower."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cavitar",
            args![
                "What's really suspicious",
                "is that it seems something",
                "has been making the tunnels",
                "collapse on purpose. Some of",
                "us believe it's because we've",
                "angered the master of the cave."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cavitar",
            args![
                "The tunnel accident",
                "is still fresh in my mind.",
                "It seems that there are",
                "more ^FF0000Cave Master^000000 sightings",
                "when the tunnels started",
                "to inexplicably collapse."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cavitar",
            args![
                "I was off duty when",
                "the accident happened.",
                "Still, I hear the only survivor",
                "went crazy and disappeared",
                "somewhere. The poor bastard..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("Zeny").get()?.number()? < 1000 {
        ctx.lines_as(
            "Cavitar",
            args![
                "You're...",
                "You're just as",
                "bad off as I am!",
                "I feel so sorry for you",
                "that I'd give you some",
                "zeny if I could spare it..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Cavitar",
            args![
                "Hmm...?",
                "I'm sorry, but",
                "I don't think I have",
                "the luxury of chatting",
                "at the moment."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cavitar",
            args![
                "My wife was seriously",
                "injured when she was",
                "working with me in the",
                "mine when it collapsed.",
                "It's agonizing, wondering",
                "if she'll be okay or not."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cavitar",
            args![
                "Luckily no one",
                "was killed, but my",
                "wife is... ^333333*S-Sob*^000000",
                "I'm sorry, but I need",
                "to be alone right now..."
            ],
        )?;
        ctx.next()?;
        'b2: {
            let subject2 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("I can help you.:Okay...:An accident?")],
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
                    "Cavitar",
                    args![
                        "Are you serious?!",
                        "Thank you, thank you so much!",
                        "A-as you can see, I'm not doing",
                        "so well financially. But I've got to get my wife to a hospital."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "And since I've got to",
                        "take of her myself,",
                        "I haven't been able to work.",
                        "I don't know what I should do."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Then how can I help?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "Well, to be frank,",
                        "I'm in dire need of zeny.",
                        "It costs 1,000 zeny to cover",
                        "her medical expenses and",
                        "food for just one day."
                    ],
                )?;
                ctx.next()?;
                'b3: {
                    let subject3 = Val::from(runtime::select_values(ctx, &[Val::from("Help Him.:Quit.")])?);
                    let mut matched3 = false;
                    let no_case3 = !subject3.loosely_equals(&Val::from(1)) && !subject3.loosely_equals(&Val::from(2));
                    if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                        matched3 = true;
                    }
                    if matched3 {
                        if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(10)])? == 0 {
                            ctx.lines_as(
                                "Cavitar",
                                args![
                                    "Hmmm, it seems you're",
                                    "carrying too many things",
                                    "with you. Shouldn't you",
                                    "store some of your items",
                                    "in Kafra Storage first?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Cavitar",
                                args![
                                    "Oh...!",
                                    "Thank you, you",
                                    "can't imagine how",
                                    "grateful I am! I never",
                                    "believed that I'd meet",
                                    "someone as kind as you."
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.var("$ein_amano").get()? == 60 {
                                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])? == 1 {
                                    ctx.lines_as(
                                        "Cavitar",
                                        args![
                                            "Other adventurers have",
                                            "also been helping me, so",
                                            "with all of your help, my wife",
                                            "has been getting better."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Cavitar",
                                        args![
                                            "I would like to give",
                                            "you a small present",
                                            "as a token of my gratitute.",
                                            "If you would, please take this."
                                        ],
                                    )?;
                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                    ctx.var("ein_gear1").set(Val::from(2))?;
                                    let subject4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?;
                                    if subject4 == 1 {
                                        ctx.call(Function::GetItem, vec![Val::from(7290), Val::from(1)])?;
                                    } else if subject4 == 2 {
                                        ctx.call(Function::GetItem, vec![Val::from(7291), Val::from(1)])?;
                                    } else if subject4 == 3 {
                                        ctx.call(Function::GetItem, vec![Val::from(7293), Val::from(1)])?;
                                    } else if subject4 == 4 {
                                        ctx.call(Function::GetItem, vec![Val::from(7294), Val::from(1)])?;
                                    } else if subject4 == 5 {
                                        ctx.call(Function::GetItem, vec![Val::from(7295), Val::from(1)])?;
                                    } else if subject4 == 6 {
                                        ctx.call(Function::GetItem, vec![Val::from(7296), Val::from(1)])?;
                                    } else if subject4 == 7 {
                                        ctx.call(Function::GetItem, vec![Val::from(7289), Val::from(1)])?;
                                    }
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Cavitar",
                                        args![
                                            "I've been keeping these",
                                            "kinds of metals since I was",
                                            "sure they'd be more useful",
                                            "to someone other than me.",
                                            "Hopefully, you'll find this ore",
                                            "useful in one way or another."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Cavitar",
                                        args![
                                            "If I were still",
                                            "working, I'd try my",
                                            "best to pay you back.",
                                            "But my current situation",
                                            "doesn't really allow it..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Cavitar",
                                        args![
                                            "Still, I'd be happy",
                                            "to trade you 1 Oridecon",
                                            "if you can bring me some",
                                            "items. How does that sound?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Cavitar",
                                        args![
                                            "Would you bring me",
                                            "40 Large Jellopy,",
                                            "10 Emveretarcon,",
                                            "3 Coal and",
                                            "10 Zargon?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Sure~!:No, thanks.")])? {
                                        1 => {
                                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                            ctx.var("ein_gear1").set(Val::from(1))?;
                                            ctx.lines_as(
                                                "Cavitar",
                                                args![
                                                    "Great...!",
                                                    "Now I need to go",
                                                    "take care of my wife.",
                                                    "Come back when you",
                                                    "gather everything I've",
                                                    "asked you to bring."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines_as(
                                                "Cavitar",
                                                args![
                                                    "I see.",
                                                    "Well, I understand",
                                                    "that adventurers don't",
                                                    "stay in one place for",
                                                    "very long. Once again,",
                                                    "thank you for your help."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Cavitar",
                                                args!["But please take", "this small gift", "as a token of", "my gratitude."],
                                            )?;
                                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                            ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(1)])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                ctx.lines_as(
                                    "Cavitar",
                                    args![
                                        "If I were still",
                                        "working, I'd try my",
                                        "best to pay you back.",
                                        "But my current situation",
                                        "doesn't really allow it..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cavitar",
                                    args![
                                        "Still, I'd be happy",
                                        "to trade you 1 Oridecon",
                                        "if you can bring me some",
                                        "items. How does that sound?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Cavitar",
                                    args![
                                        "Would you bring me",
                                        "40 Large Jellopy,",
                                        "10 Emveretarcon,",
                                        "3 Coal and",
                                        "10 Zargon?"
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Sure~!:No, thanks.")])? {
                                    1 => {
                                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                        ctx.var("$ein_amano").set((ctx.var("$ein_amano").get()? + Val::from(1)))?;
                                        ctx.var("ein_gear1").set(Val::from(1))?;
                                        ctx.lines_as(
                                            "Cavitar",
                                            args![
                                                "Great...!",
                                                "Now I need to go",
                                                "take care of my wife.",
                                                "Come back when you",
                                                "gather everything I've",
                                                "asked you to bring."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as(
                                            "Cavitar",
                                            args![
                                                "I see.",
                                                "Well, I understand",
                                                "that adventurers don't",
                                                "stay in one place for",
                                                "very long. Once again,",
                                                "thank you for your help."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Cavitar",
                                            args!["But please take", "this small gift", "as a token of", "my gratitude."],
                                        )?;
                                        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(1000))?))?;
                                        ctx.call(Function::GetItem, vec![Val::from(1010), Val::from(1)])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                        matched3 = true;
                    }
                    if matched3 {
                        ctx.lines_as(
                            "Cavitar",
                            args![
                                "I understand.",
                                "I'm sorry that I've",
                                "asked so much of you.",
                                "But please understand",
                                "that I'm desperate..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "You're...",
                        "You're heartless!",
                        "^666666*Sob...*^000000",
                        "Can't you see that I'm",
                        "poor and need help?!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "Recently, we've had some",
                        "tunnel cave-ins which resulted",
                        "in miner casualties. We're having",
                        "a harder time working in the",
                        "mines now that we're even",
                        "lower on manpower."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "What's really suspicious",
                        "is that it seems something",
                        "has been making the tunnels",
                        "collapse on purpose. Some of",
                        "us believe it's because we've",
                        "angered the master of the cave."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "The tunnel accident",
                        "is still fresh in my mind.",
                        "It seems that there are",
                        "more ^FF0000Cave Master^000000 sightings",
                        "when the tunnels started",
                        "to inexplicably collapse."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Cavitar",
                    args![
                        "I was off duty when",
                        "the accident happened.",
                        "Still, I hear the only survivor",
                        "went crazy and disappeared",
                        "somewhere. The poor bastard..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn cavitar(ctx: &Ctx) -> Script {
    cavitar_body(ctx, Vec::new()).map(|_| ())
}
