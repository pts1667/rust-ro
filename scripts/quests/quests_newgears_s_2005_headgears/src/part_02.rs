use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn relaxed_looking_lady_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("lhzbtq").get()? == 0 {
        ctx.lines_as("Relaxed-Looking Lady", args!["Hmmm...?", "What? Did you", "need something?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("N-Nothing."), Val::from("Um, you look so relaxed.")])? {
            1 => {
                ctx.lines_as(
                    "Relaxed-Looking Lady",
                    args!["Huh.", "Alright then.", "Well, try not to stare", "at people so much."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Relaxed-Looking Lady",
                    args![
                        "Really? You are probably",
                        "the twentieth person to",
                        "tell me that today. Mmm.",
                        "That's strange, isn't it?"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("I guess."), Val::from("It's not strange at all.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Relaxed-Looking Lady",
                    args!["Oh yeah.", "What's your name?", "That is, if you don't", "mind me asking you."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args![
                        ((Val::from("My name is ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("")),
                        "and I'm an adventurer~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Oh, one of those?",
                        "My name is Achiha, nice",
                        "to meet you. I don't really",
                        "do much of anything.",
                        "Just sit. Relax."
                    ],
                )?;
                ctx.var("lhzbtq").set(Val::from(1))?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "I do have a hobby,",
                        "though. Once in a while,",
                        "I'll sew a hat. Do you think",
                        "an adventurer like you would",
                        "want to have a hat I made?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Er, I dunno."), Val::from("Sure.")])? {
                    1 => {
                        ctx.lines_as(
                            "Achiha",
                            args![
                                "Mm. I mean, the",
                                "Red Bonnets I make",
                                "might not be sturdy",
                                "enough for battles.",
                                "But what about those",
                                "fish and cake hats?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Achiha",
                            args![
                                "Hats...?",
                                "That look like",
                                "fish or cake? Mm.",
                                "Haha. I just got it.",
                                "That's, that's funny."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Achiha",
                            args![
                                "Really? Well, I can sew",
                                "together a ^0000FFRed Bonnet^000000.",
                                "It reminds most people",
                                "of a baby's bonnet, but",
                                "it does look good on",
                                "most people I know."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Achiha",
                            args![
                                "Um, did you want me",
                                "to make you one? I can",
                                "go ahead and do it if you",
                                "bring me some materials.",
                                "Since it's just for fun,",
                                "I won't ask for too much."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Achiha",
                            args![
                                "Just bring",
                                "^0000FF1 Green Lace^000000,",
                                "^0000FF1 Silk Ribbon^000000,",
                                "^0000FF1 Scarlet Dyestuffs^000000,",
                                "^0000FF1 Sunday Hat^000000 and",
                                "^0000FF50,000 zeny^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Achiha",
                            args![
                                "I think I'm going",
                                "to just sit and relax",
                                "a little bit longer. But",
                                "if you want me to make",
                                "a hat for you, come back",
                                "with those materials, okay?"
                            ],
                        )?;
                        ctx.var("lhzbtq").set(Val::from(2))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    } else if ctx.var("lhzbtq").get()? == 1 {
        ctx.lines_as("Achiha", args!["Oh, hello.", "Isn't it such a nice,", "quiet, pleasant day?"])?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Indeed."), Val::from("Would you please make a hat for me?")])? {
            1 => {
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Yeah. Today would",
                        "be a nice day for a",
                        "picnic or a long stroll.",
                        "But all I want to do is",
                        "just sit and relax..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Achiha",
                    args![
                        "What are you talking",
                        "about? Oh, you mean the",
                        "Red Bonnet? Well, I guess",
                        "I can make one. But I think",
                        "I need some materials first."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "^0000FF1 Green Lace^000000,",
                        "^0000FF1 Silk Ribbon^000000,",
                        "^0000FF1 Scarlet Dyestuffs^000000,",
                        "^0000FF1 Sunday Hat^000000 and",
                        "^0000FF50,000 zeny^000000.",
                        "That's what I need."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Oh, but I don't",
                        "feel like making it",
                        "right now. I'm soooo",
                        "tired. Let me just sit,",
                        "relax, even if it's just",
                        "a little while longer..."
                    ],
                )?;
                ctx.var("lhzbtq").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("lhzbtq").get()? == 2 {
        ctx.lines_as(
            "Achiha",
            args![
                "Oh, good, you're here.",
                "I've been waiting for you.",
                "Did you bring everything that",
                "you need to make a ^0000FFRed Bonnet^000000?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes."), Val::from("I forgot what I need.")])? {
            1 => {
                if ((((ctx.call(Function::CountItem, vec![Val::from(10015)])?.number()? < 1
                    || ctx.call(Function::CountItem, vec![Val::from(10007)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(975)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(5032)])?.number()? < 1)
                    || ctx.var("Zeny").get()?.number()? < 50000)
                {
                    ctx.lines_as(
                        "Achiha",
                        args![
                            "Uh oh.",
                            "You forgot",
                            "a couple things.",
                            "Would you like me",
                            "to remind you what",
                            "you need to bring?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Achiha",
                        args![
                            "^0000FF1 Green Lace^000000,",
                            "^0000FF1 Silk Ribbon^000000,",
                            "^0000FF1 Scarlet Dyestuffs^000000,",
                            "^0000FF1 Sunday Hat^000000 and",
                            "^0000FF50,000 zeny^000000. Come back",
                            "when you're ready, okay?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Oh, you brought",
                        "everything. That's",
                        "good. Okay, just give",
                        "it to me. Um, let's see."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Well, I'm finished.",
                        "^333333*Yawn*^000000 And now I'm",
                        "even more tired. Here,",
                        "take this Red Bonnet.",
                        "I hope you like it~",
                        "I think I'll relax now..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(10015), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(10007), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(975), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(5032), Val::from(1)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(5109), Val::from(1)])?;
                ctx.var("lhzbtq").set(Val::from(3))?;
                ctx.next()?;
                ctx.lines_as("Achiha", args!["I hope you will enjoy the hat~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Oh, you really",
                        "forgot? Oh dear,",
                        "let me try to remember.",
                        "I didn't forget too, did I?",
                        "Oh next;, I remember now..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "^0000FF1 Green Lace^000000,",
                        "^0000FF1 Silk Ribbon^000000,",
                        "^0000FF1 Scarlet Dyestuffs^000000,",
                        "^0000FF1 Sunday Hat^000000 and",
                        "^0000FF50,000 zeny^000000. Come back",
                        "when you're ready, okay?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("lhzbtq").get()? == 3 {
        ctx.lines_as(
            "Achiha",
            args![
                "Oh, I remember you.",
                "You're the adventurer",
                "who likes my Red Bonnets",
                "so much. Did you want me",
                "to make another one for you?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes."), Val::from("I forgot what I need.")])? {
            1 => {
                if ((((ctx.call(Function::CountItem, vec![Val::from(10015)])?.number()? < 1
                    || ctx.call(Function::CountItem, vec![Val::from(10007)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(975)])?.number()? < 1)
                    || ctx.call(Function::CountItem, vec![Val::from(5032)])?.number()? < 1)
                    || ctx.var("Zeny").get()?.number()? < 50000)
                {
                    ctx.lines_as(
                        "Achiha",
                        args![
                            "Uh oh.",
                            "You forgot",
                            "a couple things.",
                            "Would you like me",
                            "to remind you what",
                            "you need to bring?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Achiha",
                        args![
                            "^0000FF1 Green Lace^000000,",
                            "^0000FF1 Silk Ribbon^000000,",
                            "^0000FF1 Scarlet Dyestuffs^000000,",
                            "^0000FF1 Sunday Hat^000000 and",
                            "^0000FF50,000 zeny^000000. Come back",
                            "when you're ready, okay?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Oh, you brought",
                        "everything. That's",
                        "good. Okay, just give",
                        "it to me. Um, let's see."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Well, I'm finished.",
                        "^333333*Yawn*^000000 And now I'm",
                        "even more tired. Here,",
                        "take this Red Bonnet.",
                        "I hope you like it~",
                        "I think I'll relax now..."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(10015), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(10007), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(975), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(5032), Val::from(1)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(50000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(5109), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as("Achiha", args!["I hope you will enjoy the hat~"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Achiha",
                    args![
                        "Oh, you really",
                        "forgot? Oh dear,",
                        "let me try to remember.",
                        "I didn't forget too, did I?",
                        "Oh next;, I remember now..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Achiha",
                    args![
                        "^0000FF1 Green Lace^000000,",
                        "^0000FF1 Silk Ribbon^000000,",
                        "^0000FF1 Scarlet Dyestuffs^000000,",
                        "^0000FF1 Sunday Hat^000000 and",
                        "^0000FF50,000 zeny^000000. Come back",
                        "when you're ready, okay?"
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

pub fn relaxed_looking_lady(ctx: &Ctx) -> Script {
    relaxed_looking_lady_body(ctx, Vec::new()).map(|_| ())
}

fn zealotus_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("zlmaskq").get()? == 0 {
        ctx.lines_as(
            "Zealotus",
            args![
                "Kneel, worm!",
                "As ruler of this",
                "Underground Prison,",
                "I command all who step",
                "into my private realm!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zealotus",
            args![
                "Resist, and you shall be",
                "punished! Grovel and kiss",
                "my feet, and perhaps you",
                "might be spared. Hohohohoho!"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("Oh, your highness!"), Val::from("Whatever.")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Zealotus",
                    args![
                        "The submissive woman is",
                        "nothing but an ideal dream",
                        "for the arrogant male! A true",
                        "woman revels in her power to",
                        "have her man do her bidding!"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes, it's so true!"), Val::from("Boooo!")])? {
                    1 => {
                        ctx.lines_as(
                            "Zealotus",
                            args![
                                "However, in my lust for power, I may have inadventently crushed",
                                "the spirits of my beloved a little too harshly. His pride crumbled,",
                                "my man even cowers in front of the humans! It pains me to see it."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zealotus",
                            args![
                                "It is beneath me to ask",
                                "this of you, but it will take",
                                "a human like you to make",
                                "him remember who he truly is,",
                                "a proud creature of darkness",
                                "who should fear only me!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zealotus",
                            args![
                                "Human. I offer you a small",
                                "share of my power if you can",
                                "take the pathetic, weeping lump",
                                "of monster crying in the corner",
                                "of this prison and make him",
                                "realize his true nature."
                            ],
                        )?;
                        ctx.var("zlmaskq").set(Val::from(1))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Zealotus",
                            args![
                                "You dirty, dirty human...",
                                "How dare you have an ",
                                "opinion different than mine!",
                                "No matter. The day will come",
                                "when all of your race shall",
                                "address me only as \"queen.\""
                            ],
                        )?;
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
                ctx.lines_as(
                    "Zealotus",
                    args![
                        "Mortal simpleton!",
                        "Bah! The mocking of",
                        "a boorish cur is worthless",
                        "to me. I have all the time in",
                        "the world to grind your pride",
                        "to dust beneath my heels."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if (ctx.var("zlmaskq").get()?.number()? > 0 && ctx.var("zlmaskq").get()?.number()? < 6) {
        ctx.lines_as(
            "Zealotus",
            args![
                "Hm. If my beloved is",
                "acting stubborn or refuses",
                "to listen, feel free to take",
                "drastic measures. Just think",
                "of what I would do in your",
                "place. Ohohohohoho~!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("zlmaskq").get()? == 6 {
        ctx.lines_as(
            "Zealotus",
            args![
                "Ooh, you're back.",
                "Phendark is certainly",
                "back to his old self again,",
                "thanks to your efforts, human.",
                "Yes, his anger, his courage",
                "and passion are all restored~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zealotus",
            args![
                "As I promised, I shall",
                "grant you a share of my",
                "power. However, I will need",
                "some items to form this minor",
                "contract between you and me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zealotus",
            args![
                "I will need",
                "^3131FF1 Cat's Eye^000000,",
                "^3131FF1 Forbidden Red Candle^000000 and",
                "^3131FF30 Worn-Out Magic Scrolls^000000.",
                "Then, I can grant you a measure",
                "of my power as I've promised."
            ],
        )?;
        ctx.var("zlmaskq").set(Val::from(7))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("zlmaskq").get()? == 7 {
        if ((ctx.call(Function::CountItem, vec![Val::from(7263)])?.is_true()
            && ctx.call(Function::CountItem, vec![Val::from(660)])?.is_true())
            && ctx.call(Function::CountItem, vec![Val::from(7099)])?.number()? > 29)
        {
            ctx.lines_as(
                "Zealotus",
                args![
                    "I see that you have",
                    "brought what I need to",
                    "complete the contract",
                    "between you and me,",
                    "human. Let's begin..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFZealotus takes the red",
                "candle you've given her and",
                "drips the wax into her open",
                "palm. The Cat's Eye begins",
                "to glow with an eerie light.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Zealotus",
                args![
                    "Now, place your index",
                    "finger into my palm so",
                    "that we may complete the",
                    "final step of this contract..."
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(
                ctx,
                &[Val::from("Don't complete the contract."), Val::from("Complete the contract.")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Zealotus",
                        args![
                            "Hm? What are you",
                            "afraid of? This is a",
                            "minor contract, so",
                            "you are not selling me",
                            "your soul, or anything",
                            "else for that matter."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as(
                        "Zealotus",
                        args![
                            "I, Zealotus, as ruler",
                            "of this realm, seal this",
                            "eternal contract with this",
                            "Forbidden Red Candle."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Zealotus",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(" will")),
                            "forever have a share",
                            "in my power. Those that",
                            "bow to me must also bow",
                            ((Val::from("to ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". It shall be done."))
                        ],
                    )?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_DEVIL")?])?;
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POISONHIT")?])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Zealotus",
                        args![
                            "Human, take this",
                            "mask with you as an",
                            "everlasting token of our",
                            "contract. So long as you",
                            "carry this, I will be at your",
                            "side. So says Zealotus!"
                        ],
                    )?;
                    ctx.call(Function::DelItem, vec![Val::from(7263), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(660), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7099), Val::from(30)])?;
                    ctx.call(
                        Function::GetNamedItem,
                        vec![Val::from(5121), ctx.call(Function::StrCharInfo, vec![Val::from(0)])?],
                    )?;
                    ctx.var("zlmaskq").set(Val::from(8))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        ctx.lines_as(
            "Zealotus",
            args![
                "I will need",
                "^3131FF1 Cat's Eye^000000,",
                "^3131FF1 Forbidden Red Candle^000000 and",
                "^3131FF30 Worn-Out Magic Scrolls^000000.",
                "Then, I can grant you a measure",
                "of my power as I've promised."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("zlmaskq").get()? == 8 {
        ctx.lines_as(
            "Zealotus",
            args![
                "Ah, I greet you in",
                "peace, human. Behold,",
                "the splendor of our realm!",
                "Though, I do not blame you",
                "if you have no interest in",
                "commanding these Injustices..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zealotus",
            args![
                "Thanks to your help,",
                "my Phendark has returned",
                "to his old, monstrously",
                "passionate ways. Now I can",
                "show him the stinging love",
                "of my whip! Hohohohohoho!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn zealotus_lhzhat(ctx: &Ctx) -> Script {
    zealotus_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn phendark_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_phen_point = Val::from(0);
    if ctx.var("zlmaskq").get()? == 0 {
        ctx.lines_as(
            "Phendark",
            args!["Huh? Oh no!", "Another h-human?!", "P-please! S-stay away,", "don't come near me!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("zlmaskq").get()? == 1 {
            ctx.lines_as(
                "Phendark",
                args!["Huh? Oh no!", "Another h-human?!", "P-please! S-stay away,", "don't come near me!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phendark",
                args![
                    "I... I swear!",
                    "I'm not carrying any",
                    "rare items or stuff you",
                    "can wear, so please don't",
                    "beat me! I... Oh my god, you",
                    "don't believe me, don't you?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phendark",
                args![
                    "You humans never leave",
                    "me alone! Why do you have",
                    "to bully me like this?! I'm",
                    "honestly not carrying anything",
                    "of value! Z-Zealotus, please!",
                    "Zealotus, heeeeeelp me~!"
                ],
            )?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
            if !(((((((((((((((((((((ctx.call(Function::CountItem, vec![Val::from(1950)])?.is_true()
                || ctx.call(Function::CountItem, vec![Val::from(1951)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1952)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1953)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1954)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1955)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1956)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1957)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1958)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1959)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1960)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1961)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1962)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1963)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1964)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1965)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1966)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1967)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1968)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1969)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1970)])?.is_true())
                || ctx.call(Function::CountItem, vec![Val::from(1971)])?.is_true())
            {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "You're pathetic!",
                    "Aren't you supposed",
                    "to be a monster? You know",
                    "what Zealotus would do if",
                    "she were actually here?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phendark",
                args!["^333333*Sniff...*^000000", "P-probably...", "Probably whip me."],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFYou nonchalantly^000000", "^3355FFbrandish your Whip.^000000"])?;
            ctx.next()?;
            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["That's right."])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FF*Snap!*",
                "*Snap!*",
                "*Crack crack crack!*",
                "*Snap snap snap crack!*^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Phendark",
                args![
                    "Oh! That stinging pain",
                    "that burns with bloodlust!",
                    "It's almost as good as",
                    "Zealtos's whip of love!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Phendark",
                args![
                    "Zealotus...",
                    "She must be pissed",
                    "at me, but I just can't",
                    "stop being afraid of all",
                    "you humans! Damn it all!"
                ],
            )?;
            ctx.var("zlmaskq").set(Val::from(2))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("zlmaskq").get()? == 2 || ctx.var("zlmaskq").get()? == 3) {
                if (ctx.call(Function::CountItem, vec![Val::from(7315)])?.number()? > 368 && ctx.var("zlmaskq").get()? == 3) {
                    ctx.lines_as(
                        "Phendark",
                        args![
                            "Y-you again!",
                            "Why do you hound me?!",
                            "Th-there's nothing I can",
                            "give you, so please don't",
                            "hurt me! Oh, oh p-please...!"
                        ],
                    )?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Feed him Dark Crystal Fragment."), Val::from("Threaten him.")])? {
                        1 => {
                            ctx.lines_as(
                                "Phendark",
                                args![
                                    "What...? You want",
                                    "me to eat these?",
                                    "It doesn't seem natural,",
                                    "but if Rybio says I should,",
                                    "it might not be that bad."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["...", "......"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Phendark",
                                args![
                                    "What's supposed to",
                                    "happen now? My inner",
                                    "demon is supposed to",
                                    "awaken by eating these?",
                                    "That sounds ridiculous!",
                                    "Though, I did just eat crystal."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Phendark",
                                args![
                                    "Wh-whoa. Ugh!",
                                    "My chest! Something's",
                                    "burning inside! I c-can't--!",
                                    "Can't think straight... I'm...",
                                    "Slowly... Losing my humanity!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Phendark", args!["Huh... Huuurg--!", "Huk-huk! Heeeeh!", "Heeeeeh! Waaoooooh!"])?;
                            ctx.next()?;
                            ctx.lines_as("Phendark", args!["...", "Grrrrr...!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Phendark",
                                args!["^333333*Pant pant*^000000", "What... just...", "What happened?"],
                            )?;
                            ctx.var("zlmaskq").set(Val::from(4))?;
                            ctx.call(Function::DelItem, vec![Val::from(7315), Val::from(369)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "So what happens",
                                    "if I don't decide not",
                                    "to hurt you? Whatcha",
                                    "gonna do then, huh?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args!["...", "......"])?;
                            ctx.next()?;
                            ctx.lines_as("Phendark", args!["S-stop it!", "Just--Just stop it!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.lines_as(
                    "Phendark",
                    args!["Humans...", "They're everywhere!", "You guys--I can't...", "You're torturing me!"],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("zlmaskq").get()? == 4 {
                    ctx.lines_as(
                        "Phendark",
                        args![
                            "My chest was on fire,",
                            "like I was, I dunno,",
                            "burning with anger or",
                            "something. It's gone",
                            "now, but what were",
                            "those crystals?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("zlmaskq").get()? == 5 {
                    ctx.lines_as(
                        "Phendark",
                        args![
                            "You again? Oh no,",
                            "you're not going to hurt",
                            "me or make me eat those",
                            "weird crystals again, are you?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        ((Val::from("^3131FF[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]^000000"))
                    ])?;
                    match runtime::select_values(ctx, &[Val::from("Rybio"), Val::from("Injustice"), Val::from("Zealotus")])? {
                        1 => {
                            ctx.mes("Rybio")?;
                        }
                        2 => {
                            ctx.mes("Injustice")?;
                        }
                        3 => {
                            ctx.mes("Zealotus")?;
                            l_phen_point = (l_phen_point.clone() + Val::from(1));
                        }
                        _ => {}
                    }
                    match runtime::select_values(ctx, &[Val::from("hates"), Val::from("likes")])? {
                        1 => {
                            ctx.mes("hates")?;
                            l_phen_point = (l_phen_point.clone() + Val::from(2));
                        }
                        2 => {
                            ctx.mes("likes")?;
                            l_phen_point = (l_phen_point.clone() + Val::from(1));
                        }
                        _ => {}
                    }
                    match runtime::select_values(
                        ctx,
                        &[
                            Val::from("Rybio."),
                            Val::from("Injustice."),
                            Val::from("Phendark."),
                            Val::from("Zealotus."),
                        ],
                    )? {
                        1 => {
                            ctx.mes("Rybio.")?;
                        }
                        2 => {
                            ctx.mes("Injustice.")?;
                        }
                        3 => {
                            ctx.mes("Phendark.")?;
                            l_phen_point = (l_phen_point.clone() + Val::from(1));
                        }
                        4 => {
                            ctx.mes("Zealotus.")?;
                            ctx.next()?;
                        }
                        _ => {}
                    }
                    if l_phen_point.clone() == 1 {
                        ctx.lines_as(
                            "Phendark",
                            args![
                                "I don't think",
                                "congratulations are",
                                "in order, but I guess",
                                "that's good news to hear.",
                                "Still, what a surprise!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_phen_point.clone() == 2 {
                        ctx.lines_as(
                            "Phendark",
                            args!["Hey...!", "I don't know if", "that's something", "I want to hear about!"],
                        )?;
                    } else if l_phen_point.clone() == 3 {
                        ctx.lines_as(
                            "Phendark",
                            args![
                                "I can't believe",
                                "something like that!",
                                "Oh, that doesn't matter.",
                                "Zealotus is too good for me.",
                                "I'm not even worthy of tasting",
                                "the sting of her Love Whip."
                            ],
                        )?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if l_phen_point.clone() == 4 {
                        ctx.lines_as(
                            "Phendark",
                            args!["...", "......", "Hates me?", "N-no, that can't--", "I didn't, that doesn't--"],
                        )?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as("Phendark", args!["Huh...?", "What is that", "supposed to mean?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    'b5: {
                        let subject5 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Insult him."), Val::from("Apologize.")],
                        )?);
                        let mut matched5 = false;
                        let no_case5 = !subject5.loosely_equals(&Val::from(1)) && !subject5.loosely_equals(&Val::from(2));
                        if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                            matched5 = true;
                        }
                        if matched5 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args![
                                    "First of all,",
                                    "what exactly sets",
                                    "you apart from all the",
                                    "other eligible monsters",
                                    "that she can choose from?",
                                    "Not like you're much better..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Phendark", args!["...", "......"])?;
                            ctx.next()?;
                            'b6: {
                                let subject6 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Break his pride."), Val::from("Tell him you were joking.")],
                                )?);
                                let mut matched6 = false;
                                let no_case6 = !subject6.loosely_equals(&Val::from(1)) && !subject6.loosely_equals(&Val::from(2));
                                if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                    matched6 = true;
                                }
                                if matched6 {
                                    ctx.lines(args![
                                        ((Val::from("^3131FF[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                            + Val::from("]^000000")),
                                        "I mean, there are guys",
                                        "like Bloody Murderer out",
                                        "there who are more evil",
                                        "than you, and best of all,",
                                        "not afraid of humans!"
                                    ])?;
                                    ctx.next()?;
                                    match runtime::select_values(
                                        ctx,
                                        &[Val::from("Go for the low blow."), Val::from("Try to salvage his confidence.")],
                                    )? {
                                        1 => {
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "You know, me and Zealotus",
                                                    "were actually talking about",
                                                    "you recently. She told me that",
                                                    "the sight of you makes her",
                                                    "feel sick! I mean, what kind",
                                                    "of monster is afraid of humans?"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                                args![
                                                    "Now you've reached the",
                                                    "point where even Injustice",
                                                    "is manlier than you now,",
                                                    "if you know what I mean!",
                                                    "Hahahahahahahahaha!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Phendark", args!["...", "......"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Phendark", args!["...", "......", "........."])?;
                                            ctx.next()?;
                                            ctx.lines_as("Phendark", args!["...", "......", ".........", "............"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Phendark", args!["GRRRRRR!", "THAT'S ENOUGH!"])?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Phendark",
                                                args![
                                                    "I don't care who the",
                                                    "hell she is, I'm going",
                                                    "to see Zealotus and give",
                                                    "that tramp a piece of my",
                                                    "mind! Grrrr! She'll be sorry!"
                                                ],
                                            )?;
                                            ctx.var("zlmaskq").set(Val::from(6))?;
                                            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.lines(args![
                                                ((Val::from("^3131FF[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                    + Val::from("]^000000")),
                                                "But know that I think",
                                                "about it, maybe you look",
                                                "much tougher than that",
                                                "Bloody Murderer guy."
                                            ])?;
                                            ctx.next()?;
                                            ctx.lines_as("Phendark", args!["*Sniff sniff*", "You really think so?"])?;
                                            ctx.next()?;
                                            ctx.lines(args!["...", "......"])?;
                                            ctx.next()?;
                                            ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["...Yeeeeeah."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                }
                                if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                    matched6 = true;
                                }
                                if matched6 {
                                    ctx.lines_as(
                                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                        args![
                                            "But now that I think",
                                            "about it, you actually",
                                            "are much better than all",
                                            "those other monsters.",
                                            "That stuff I said before?",
                                            "I was just kidding you."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Phendark", args!["^333333*Sniff!*^000000", "You're...", "Not helping!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                        if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                            matched5 = true;
                        }
                        if matched5 {
                            ctx.lines_as(
                                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                                args!["I...", "I was just kidding!", "How could she hate", "somebody like you?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Phendark",
                                args![
                                    "No... No.",
                                    "You're right.",
                                    "I don't deserve love.",
                                    "Not from Zealotus or",
                                    "from anybody else..."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else if ctx.var("zlmaskq").get()?.number()? > 5 {
                    ctx.lines_as(
                        "Phendark",
                        args![
                            "Zealotus! How dare",
                            "she say those things",
                            "against me! Less manly",
                            "than Injustice?! I'll just",
                            "have to prove her wrong!",
                            "Out the way, pithy human!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn phendark_lhzhat(ctx: &Ctx) -> Script {
    phendark_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn rybio_lhzhat_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("zlmaskq").get()? == 2 {
        ctx.lines_as(
            "Rybio",
            args![
                "You know, I usually just",
                "run up and slash like crazy",
                "once I see you humans, but",
                "my heart's not in it today, so",
                "I'm gonna give you just one",
                "chance to run for your life."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Talk about Phendark."), Val::from("Run for your life.")])? {
            1 => {
                ctx.lines_as(
                    "Rybio",
                    args![
                        "What th--? You know the",
                        "same Phendark I know?",
                        "Dayam, that's weird. But",
                        "yeah, him and Zealotus have",
                        "some kind of... I dunno what",
                        "it is, actually. Um, love?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rybio",
                    args![
                        "Anyway, Phendark has",
                        "been acting really weird",
                        "lately. I guess Zealotus",
                        "loved him to the point that",
                        "she abused him to the point",
                        "that he's scared of humans now."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rybio",
                    args![
                        "Huh. You know what'd help?",
                        "Dark Crystal Fragments. It's",
                        "worthless to humans, but if",
                        "creatures of darkness eat it,",
                        "it brings out more of their",
                        "inner demon. Scary, huh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rybio",
                    args![
                        "Since Phendark's pretty big, and he's acting like a total wuss, you",
                        "you should probably get him to eat ^3131FF369 Dark Crystal Fragments^000000. Don't",
                        "worry, you should be able to find those all over Midgard."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rybio",
                    args![
                        "I dunno why a human",
                        "like you would want to",
                        "help one of us out, though.",
                        "What's in it for you, exactly?"
                    ],
                )?;
                ctx.var("zlmaskq").set(Val::from(3))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Rybio",
                    args![
                        "That's right!",
                        "Run, get outta here!",
                        "If you're not fast enough,",
                        "I might eat you, human!",
                        "...Well... Probably not."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("zlmaskq").get()? == 3 {
        ctx.lines_as(
            "Rybio",
            args![
                "I don't get why someone",
                "like you, a seemingly heroic",
                "adventurer, would want to help",
                "out Phendark? Did Zealotus",
                "blackmail you or something?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rybio",
            args![
                "Well, you could",
                "probably help him",
                "by getting him to eat",
                "^3131FF369 Dark Crystal Fragments^000000",
                "to sort of stir up the demon",
                "that sleeping within, you know?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("zlmaskq").get()? == 4 {
        ctx.lines_as(
            "Rybio",
            args![
                "Feeding him all of those",
                "Dark Crystal Fragments didn't",
                "work? But that was supposed",
                "to be foolproof! Dayam, what",
                "the hell happened to Phendark's",
                "inner demon?! Man oh man..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rybio",
            args![
                "Well, I don't know what",
                "else you could try. I mean,",
                "maybe you could try motivating",
                "him. Reverse psychology?",
                "It's weird talking about this",
                "with a human. Hahahaha~"
            ],
        )?;
        ctx.var("zlmaskq").set(Val::from(5))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("zlmaskq").get()? == 5 {
        ctx.lines_as(
            "Rybio",
            args![
                "Phendark sure looks",
                "tough, but I guess even",
                "he isn't totally evil. Yeah.",
                "We monsters aren't all",
                "bad... Just mostly bad."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn rybio_lhzhat(ctx: &Ctx) -> Script {
    rybio_lhzhat_body(ctx, Vec::new()).map(|_| ())
}

fn freight_manager_toast_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Kirishu",
        args![
            "Oh man...",
            "It's almost lunchtime,",
            "but he hasn't come by",
            "yet. Where could h--Oh!",
            "Hello, welcome to Einbroch~"
        ],
    )?;
    ctx.next()?;
    if (ctx.call(Function::IsEquipped, vec![Val::from(5107)])?.is_true() || ctx.call(Function::CountItem, vec![Val::from(5107)])?.is_true())
    {
        ctx.lines_as(
            "Kirishu",
            args![
                "Wait a sec, that",
                "Crunch Toast! You must",
                "be the guy who helped out",
                "my buddy, Kasis. I owe you",
                "my gratitude since without your help, I wouldn't eat lunch!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kirishu",
            args![
                "It sounds like Kasis",
                "has been having business",
                "troubles lately, but I'm sure",
                "he'll weather it out. I just don't know what's taking him so long",
                "to get here. I'm sooo hungry!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Kirishu",
            args![
                "By any chance, are you",
                "heading to Juno? I have",
                "a favor to ask if you're",
                "going there. Or did you",
                "need help finding your",
                "way around this city?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[
                    Val::from("Sure, I'm heading to Juno."),
                    Val::from("I need help finding my way around!"),
                    Val::from("Cancel."),
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
                    "Kirishu",
                    args![
                        "Well, if you're going to",
                        "Juno and you're not busy,",
                        "would you visit a pastry",
                        "chef named Kasis for me?",
                        "He's an old friend and I'm",
                        "a little worried about him."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirishu",
                    args![
                        "Usually, he sends us",
                        "lunches around this time",
                        "without fail, but since lunch",
                        "hasn't arrived yet, I can't help but wonder, ''What happened?''"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirishu",
                    args![
                        "Of course, I'd be lying if",
                        "I said my hunger didn't factor",
                        "into my concern for his welfare. Anyway, if you have the time,",
                        "you can find Kasis in Juno's",
                        "Bakery. Thanks, adventurer."
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
                    "Kirishu",
                    args![
                        "First of all, if you",
                        "go just outside of the",
                        "Airport, you can find",
                        "Guides that can tell you",
                        "about all the important",
                        "places of interest in Einbroch."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirishu",
                    args![
                        "Now, if you want to take",
                        "a rest before or after a",
                        "flight on one of the Airships,",
                        "you can use the passenger",
                        "lounge on the second floor.",
                        "It's a good tip to know~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kirishu",
                    args!["Did you have any", "other questions to ask", "me while you're here?"],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[
                        Val::from("Can I take a picture with you?"),
                        Val::from("About Einbroch"),
                        Val::from("No, thanks."),
                    ],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Kirishu",
                            args![
                                "Pardon me?",
                                "I don't hear that",
                                "request often, but",
                                "I suppose it couldn't",
                                "do any harm to pose",
                                "for just one picture."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("ein_soldier.bmp"), Val::from(1)])?;
                        ctx.lines_as("Kirishu", args!["Cheese~"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as(
                            "Kirishu",
                            args![
                                "You know, it's hard to",
                                "believe that Einbroch has",
                                "become a major industrial",
                                "so quickly. I guess it's the",
                                "result of the demand for the",
                                "ores mined over in Einbech."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirishu",
                            args![
                                "Although Einbech and Einbroch",
                                "can't survive without each other, Einbech is in a pretty sorry state",
                                "compared to Einbroch. If you've",
                                "ever been there, you know what",
                                "I mean. It's almost tragic."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirishu",
                            args![
                                "Most of the prosperity of",
                                "this city is attributed to the",
                                "productivity of the factories.",
                                "But along with great wealth,",
                                "these factories have also",
                                "brought pollution problems."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kirishu",
                            args![
                                "If you hear the",
                                "Smog Alert, you should",
                                "run to safety as quickly",
                                "as you can! It's dangerous",
                                "to be out in the open when",
                                "the pollution levels are high!"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as(
                            "Kirishu",
                            args!["Well then, I hope", "that you enjoy your", "travels here in Einbroch."],
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
                    "Kirishu",
                    args![
                        "Have a good time in",
                        "Einbroch, adventurer.",
                        "I hope you enjoy your",
                        "travels here in the",
                        "Schwarzwald Republic."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn freight_manager_toast(ctx: &Ctx) -> Script {
    freight_manager_toast_body(ctx, Vec::new()).map(|_| ())
}
