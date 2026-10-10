use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn wincingoldman_veins_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "- Wait a moment! -",
            "- Currently you are carrying -",
            "- too many items with you. -",
            "- Please come back after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("veins_stone").get()? == 0 {
        ctx.lines_as(
            "Zabaroo",
            args![
                "My back is killing me",
                "after stooping over to",
                "pick up stones all day long...",
                "The pain... It's unbearable!"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Bend with your knees, yo.:Gosh, how bad is it?")])? {
            1 => {
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Whippersnapper!",
                        "I didn't ask you for",
                        "your advice! Don't",
                        "patronize an old man!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Oh, it hurts so much, it's",
                        "almost crippling. But I don't",
                        "have any choice. I need to",
                        "get enough stops to fill",
                        "this gap if I want to get",
                        "paid. Arrrrgh, damn it!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "If I don't get enough",
                        "money to pay for my",
                        "granddaughter's medicine,",
                        "I won't be able to buy any",
                        "medicine for my granddaughter!",
                        "And that will be horrible! Ag!"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Can I help?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "I appreciate your kindness,",
                        "stranger, but no. I have",
                        "to do this on my own.",
                        "A man must have his pride...."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("^333333*Snap*^000000")?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Argh! My back...!",
                        "This was totally",
                        "unforeseeable!",
                        "Please! Please,",
                        "for the love of Freya,",
                        "please help me!"
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("I will help you.")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Zabaroo", args!["Thank you!", "Thanks so much!"])?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("What do you want me to do?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Ow-ow-ow! Oh... Okay...",
                        "You see those all dark",
                        "stones stuck in the ground?",
                        "Those are what I need to",
                        "pick up. Now don't go",
                        "lifting rocks just yet..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "I asked some other old",
                        "man in town, Absar, to",
                        "make me something to help",
                        "with my back. He was going",
                        "to help me if I gave hi--",
                        "ARGH! My back! It hurts!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "It hurts so bad! But it's",
                        "especially painful right",
                        "when I'm about to finish",
                        "sentences in which I intend to",
                        "tell you important informat--",
                        "ARRRGH! Find Absar! Quickly!"
                    ],
                )?;
                ctx.next()?;
                'l2: loop {
                    if !(true) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.lines_as(
                            "Zabaroo",
                            args![
                                "Wait, wait...",
                                "Maybe I can answer",
                                "a few of your questions",
                                "before my body is wracked",
                                "with throbbing pain. Let's...",
                                "Let's at least give it a try."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("What's with these stones?:Where's the old man?:Nothing.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Zabaroo",
                                    args![
                                        "Well, we use these dark",
                                        "stones because they're",
                                        "pretty and easy to process.",
                                        "They're sort of a specialty",
                                        "of this town. I get paid to",
                                        "harvest these handy rocks."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Zabaroo",
                                    args![
                                        "We can sell these to tourists,",
                                        "and we even have a factory",
                                        "that uses these stones.",
                                        "Still, it's not like the",
                                        "townspeople are getting",
                                        "rich off these stones."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Zabaroo", args!["All of us are still", "barely making a living..."])?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Zabaroo",
                                    args![
                                        "Absar? He's inside",
                                        "the Tool Shop. You",
                                        "can't miss him... Just",
                                        "look for the man with",
                                        "the crazy eyes!"
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Zabaroo",
                                    args![
                                        "Thanks for your help.",
                                        "If you can't find Absar",
                                        "in the Tool Shop, then",
                                        "you might want to stop",
                                        "by the Tavern. A-auuugh!"
                                    ],
                                )?;
                                ctx.var("veins_stone").set(Val::from(1))?;
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
    } else {
        if ctx.var("veins_stone").get()? == 1 {
            ctx.lines_as(
                "Zabaroo",
                args![
                    "Thanks for your help.",
                    "If you can't find Absar",
                    "in the Tool Shop, then",
                    "you might want to stop",
                    "by the Tavern. A-auuugh!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("veins_stone").get()?.number()? > 1 && ctx.var("veins_stone").get()?.number()? < 4) {
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Geez, Absar sure can",
                        "be fussy. Still, do your",
                        "best to get what he wants.",
                        "He won't help me otherwise!",
                        "Other than that, he's not",
                        "really that bad a guy..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("veins_stone").get()? == 4 {
                ctx.lines_as(
                    "Zabaroo",
                    args!["Oh good, you're back!", "Did you bring what", "Absar made for me?"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Yes, here...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "So this is what he",
                        "was talking about?",
                        "How does it... Ah!",
                        "Here we go! If I pull",
                        "the handle, that end of",
                        "the stick will pick stuff up!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "This is great! I won't",
                        "have to bend over to pick",
                        "up stones anymore! Heh,",
                        "he must be awfully proud",
                        "of this useful invention~",
                        "I can imagine him strutting."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args!["Um, did he have anything", "to say after he gave this", "to you? I'm just curious."],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("This message...")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Oh... I thought he",
                        "would forget all about",
                        "that. Well, it's a relief",
                        "to know that now. That's",
                        "really very nice of him."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args!["I hate to ask you...", "But would you mind", "helping me out one", "more time?"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Huh? What is it?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Don't worry, it's not",
                        "too hard. Would you just",
                        "deliver the stones I gathered",
                        "to the factory in town? It's",
                        "near the airport or airship or",
                        "something. It won't take long."
                    ],
                )?;
                ctx.var("veins_stone").set(Val::from(5))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("veins_stone").get()?.number()? > 4 && ctx.var("veins_stone").get()?.number()? < 7) {
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Thanks again for your help!",
                        "What did Absar call this",
                        "thing again? A Tactile...",
                        "Extendable... Damn it...",
                        "Why's the name so long?",
                        "Anyway, it's usefull~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("veins_stone").get()? == 7 {
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Oh, you're back!",
                        "I really appreciate all",
                        "of your help. I don't have",
                        "much, and I know you weren't",
                        "expecting a reward, but I'd",
                        "like to give you something."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Ah, here we are. I found",
                        "these while harvesting stones",
                        "I was told that adventurers",
                        "find these useful. Anyway,",
                        "I hope you like these rocks..."
                    ],
                )?;
                ctx.var("veins_stone").set(Val::from(8))?;
                ctx.call(Function::GetExperience, vec![Val::from(300000), Val::from(0)])?;
                ctx.call(Function::GetItem, vec![Val::from(985), Val::from(3)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("veins_stone").get()?.number()? > 7 {
                ctx.lines_as(
                    "Zabaroo",
                    args![
                        "Thanks to you and Absar,",
                        "my poor back hasn't been",
                        "bothering me at all lately.",
                        "I should be taking better",
                        "care of myself at my age..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn wincingoldman_veins(ctx: &Ctx) -> Script {
    wincingoldman_veins_body(ctx, Vec::new()).map(|_| ())
}

fn strange_old_man_ve_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("veins_stone").get()? == 0 {
        ctx.lines_as(
            "Absar",
            args!["Bwahahaha!", "Once... Once this is completed, I'll...", "Mwahahahahahahahah!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("veins_stone").get()? == 1 {
        ctx.lines_as(
            "Absar",
            args!["I'm so close to", "completion. Now, if", "I just turn this here..."],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Excuse me...:........")])?) == 1 {
            ctx.lines_as(
                "Absar",
                args![
                    "What?! Who dares",
                    "disturb me?! N-no!",
                    "Look! Look what you did!",
                    "You made me screw up!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
            ctx.call(Function::PercentHeal, vec![Val::from(-30), Val::from(0)])?;
            ctx.next()?;
            ctx.lines_as("Absar", args!["!@#$%#@#$!*~", "F$#@#%^^^&&!"])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ve_in"), Val::from(262), Val::from(309)])?;
            return Err(Stop::End);
        }
        ctx.lines_as("Absar", args!["Oh, no..."])?;
        ctx.next()?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ASPERSIO")?])?;
        ctx.next()?;
        ctx.lines_as("Absar", args!["How...?!", "No! I failed again!", "D-DAAAAAAAAAAAMN IIIII--"])?;
        ctx.next()?;
        ctx.lines_as("Absar", args!["Huh? What are you...", "What do you want?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Oh, I-I'm...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Absar",
            args![
                "Spit it out. Tell me",
                "what you want, not your",
                "name. Hurry, can't you",
                "see that I'm busy?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("I... I'm sorry.:I'm here for Mr. Zabaroo...")],
        )?) == 1
        {
            ctx.lines_as(
                "Absar",
                args![
                    "If only you didn't",
                    "interrupt me! Then",
                    "I'd already have...",
                    "Ugh! Back to work!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Absar",
            args![
                "Zabaroo? What does...",
                "Oh. Never mind. I think",
                "I remember what he asked",
                "me to make him. So did",
                "you bring all the materials?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes:What materials...?")])?) == 1 {
            ctx.lines_as(
                "Absar",
                args![
                    "Great, we can get",
                    "started and m--",
                    "Liar. You don't even know",
                    "what you're supposed to",
                    "bring me, do you?!",
                    "Get out of here!"
                ],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("ve_in"), Val::from(262), Val::from(309)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Absar",
            args![
                "Of course. I didn't tell him",
                "what materials I needed",
                "anyway. Heh heh! Now, this",
                "is what I need you to bring.",
                "Listen up, okay? And hurry."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Absar",
            args![
                "^4D4DFF5 Maneater Roots^000000,",
                "^4D4DFF2 Glacial Hearts^000000, and",
                "^4D4DFF5 Steel^000000. If you don't",
                "come back soon, then",
                "I won't help you. I've got",
                "projects I'm working on!"
            ],
        )?;
        ctx.var("veins_stone").set(Val::from(2))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("veins_stone").get()? == 2 {
        if ((ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 4
            && ctx.call(Function::CountItem, vec![Val::from(1033)])?.number()? > 4)
            && ctx.call(Function::CountItem, vec![Val::from(7561)])?.number()? > 1)
        {
            ctx.lines_as(
                "Absar",
                args![
                    "Good, you finally",
                    "brought everthing.",
                    "I was just about to",
                    "give up on you, so",
                    "consider yourself lucky!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Absar",
                args!["Give me a second.", "You won't have to", "wait long to see", "my great invention!"],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(999), Val::from(5)])?;
            ctx.call(Function::DelItem, vec![Val::from(1033), Val::from(5)])?;
            ctx.call(Function::DelItem, vec![Val::from(7561), Val::from(2)])?;
            ctx.var("veins_stone").set(Val::from(3))?;
            ctx.close_window()?;
            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FIRESPLASHHIT")?])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Absar",
            args![
                "What the hell?",
                "Hurry and bring",
                "^4D4DFF5 Maneater Roots^000000,",
                "^4D4DFF2 Glacial Hearts^000000, and",
                "^4D4DFF5 Steel^000000! Do you think",
                "I'm doing this for fun?!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("veins_stone").get()? == 3 {
        ctx.lines_as(
            "Absar",
            args![
                "Here you are...",
                "Well, I don't know",
                "if you appreciate",
                "inventions, but this",
                "is a Tactile Extendable",
                "Clamp-Release Mechanism!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Tack... What...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Absar",
            args![
                "...............................",
                "You can use this to pick",
                "things up. From a distance."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Absar",
            args![
                "Anyway, when you get back",
                "to Zabaroo, let me him know",
                "that he doesn't owe me",
                "anything anymore. He'll...",
                "He'll know what I mean..."
            ],
        )?;
        ctx.var("veins_stone").set(Val::from(4))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Absar",
        args![
            "It's a ''Tactile Extendable",
            "Clamp-Release Mechanism.''",
            "What's so hard to understand",
            "about that? Hmpf! I know!",
            "It must be the educational",
            "system! They're to blame!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn strange_old_man_ve(ctx: &Ctx) -> Script {
    strange_old_man_ve_body(ctx, Vec::new()).map(|_| ())
}

fn factory_manager_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("veins_stone").get()?.is_true() && ctx.var("veins_stone").get()?.number()? < 5) {
        ctx.lines_as(
            "Mirhen",
            args![
                "Hey, employees only beyond this point!",
                "Geez, don't we have a sigh that says that?",
                "We should get one...",
                "Anyway, get out of here."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("veins"), Val::from(269), Val::from(221)])?;
        return Err(Stop::End);
    } else if ctx.var("veins_stone").get()? == 5 {
        ctx.lines_as(
            "Mirhen",
            args![
                "Hey, employees only",
                "beyond this point!",
                "Geez, don't we have",
                "a sign that says that?",
                "We should get one...",
                "Anyway, get out of here."
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("I'm sorry.:I've brought these...")])?) == 1 {
            ctx.lines_as("Mirhen", args!["You're sorry,", "I get it. Ummm...", "Aren't you leaving?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Mirhen",
            args!["Huh? What's this name", "tag? Zabaroo? Wait a sec...", "You're definitely not him!"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I'm here on his behalf.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Mirhen",
            args![
                "Ah, I get it. Zabaroo's",
                "back has really been",
                "bothering him lately.",
                "Okay, we can bend the",
                "rules a bit in this situation."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mirhen",
            args!["Bring those rocks over", "to Bahed over there.", "He'll take care of them."],
        )?;
        ctx.var("veins_stone").set(Val::from(6))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Mirhen",
        args![
            "Hetarium ...",
            "What's so special about",
            "it? I mean, it looks just like",
            "Iron Ore. Why the hell is",
            "it like classified info?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Mirhen",
        args![
            "I wish I could return to",
            "Rekenber Headquarters...",
            "I hate being stuck here",
            "gathering silly rocks in",
            "the middle of nowhere."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn factory_manager(ctx: &Ctx) -> Script {
    factory_manager_body(ctx, Vec::new()).map(|_| ())
}

fn factory_worker_ve1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("veins_stone").get()?.number()? < 6 {
        ctx.lines_as(
            "Bahed",
            args![
                "lately, I've been feeling so weak...",
                "I heard the guy before me had to quit because he also started feeling weak."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bahed",
            args![
                "It couldn't be...",
                "It couldn't be because of this 'Hetarium'...",
                "That's just plain crazy..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("veins_stone").get()? == 6 {
        ctx.lines_as(
            "Bahed",
            args![
                "Oh, you got the rocks?",
                "You brought them for Zabaroo?",
                "Oh, how is he doing? I hear",
                "he threw out his back."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("He still hurts.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Bahed",
            args![
                "Oh no... I'm so sorry",
                "to hear that. I hope he",
                "feels better soon. Anyway,",
                "why don't you bring the",
                "rocks over here?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bahed",
            args![
                "Let me confirm Zabaroo's",
                "quota for today. Ah, and",
                "don't worry, we pay him",
                "for his quotas regularly."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I have a question...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Bahed",
            args![
                "Hm? What'd you want",
                "to know? I can't say that",
                "I know everything, but",
                "I'll try my best to tell you",
                "what I know. I mean,",
                "you helped Zabaroo, so..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What's this Hetarium??")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Bahed",
            args![
                "Well, it hasn't been that",
                "long since these rocks",
                "attracted attention outside",
                "of town. Before all this,",
                "the townspeople just",
                "made carvings out of them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bahed",
            args![
                "Then, all of a sudden,",
                "these people from--I guess",
                "it was Schwarzwald--came",
                "and bought a lot of these",
                "stones. Later, they even built",
                "this factory to process them!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bahed",
            args![
                "Oh, they tried using machines",
                "to harvest these stones, but",
                "they all broke down too soon.",
                "That's why they hired people",
                "in Veins to collect them."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bahed",
            args![
                "I'm not sure many people",
                "know who owns this factory.",
                "Maybe it's Rekenber? Yes,",
                "I saw one of their corporate",
                "airships come to town, so",
                "I think it might be them."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("How do they use Hetarium??")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Bahed",
            args![
                "I'm not really sure. I just",
                "happened to overhear some",
                "of the higher ups mention",
                "something about hearts?",
                "Pieces of hearts? Doesn't",
                "make any sense to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Bahed",
            args![
                "All I gathered was that",
                "they were using the stones",
                "to build a machine related",
                "to those hearts. That's",
                "all I know. Anyway, please",
                "leave the stones over there~"
            ],
        )?;
        ctx.var("veins_stone").set(Val::from(7))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Bahed",
        args![
            "Everyday I feel",
            "weaker and weaker...",
            "Could this be chronic",
            "fatigue syndrome? Ugh..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn factory_worker_ve1(ctx: &Ctx) -> Script {
    factory_worker_ve1_body(ctx, Vec::new()).map(|_| ())
}

fn factory_worker_ve2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Worker",
        args![
            "What the heck are these rocks?",
            "They're ordinary stones, aren't they?",
            "They don't pay me enough here..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn factory_worker_ve2(ctx: &Ctx) -> Script {
    factory_worker_ve2_body(ctx, Vec::new()).map(|_| ())
}

fn kid_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()? == 0 {
        ctx.lines_as(
            "Rooney",
            args![
                "Where the heck",
                "is it? I don't... Where...?",
                "He's got to be around here",
                "somewhere, I think..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What's wrong?:...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Rooney",
            args![
                "Oh, it's just...",
                "Some guy sent me on a",
                "delivery errand, but I can't",
                "find the recipient. He said",
                "that I can't miss him, but",
                "I still can't figure it out."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rooney",
            args![
                "I should have asked for",
                "the exact location. I mean,",
                "if I don't find him, then I'll",
                "never get paid for doing",
                "this delivery. ^333333*Sigh*^000000"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Sounds tough. Good luck!:Do you need any help?")],
        )?) == 1
        {
            ctx.lines_as(
                "Rooney",
                args![
                    "Thanks. I think",
                    "I just might need it.",
                    "Where could this guy",
                    "be? If he's expecting",
                    "a delivery, he should",
                    "make himself easy to find..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Rooney",
            args![
                "Yeah, sure, it'd be great",
                "if you could help me. Let's",
                "see, I need to deliver this",
                "letter to a bard named...",
                "It was... Ah, ^FF0000Lasda Midar^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rooney",
            args![
                "I've looked everywhere",
                "in town for someone that",
                "looks like a Bard, but I'm",
                "not having any luck. If you",
                "find him, would you tell me?"
            ],
        )?;
        ctx.var("que_sch").set(Val::from(1))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 1 {
        ctx.lines_as(
            "Rooney",
            args![
                "I still haven't",
                "found Lasda Midar.",
                "Would you let me know",
                "if you find him so that",
                "I can deliver his letter?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 2 {
        if ctx.var("Zeny").get()?.number()? < 100 {
            ctx.lines_as("Rooney", args!["Lasda Midar...", "Where the heck", "could that guy be?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Rooney",
            args![
                "Wow, did you really",
                "find Lasda Midar?",
                "Why couldn't I find him?",
                "Anyway, thank you so much",
                "for your help. I thought I was",
                "going to fail my delivery!"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("He asked me to give you this.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("Rooney", args!["Hey, alright! Thanks", "for the cash! Heh heh~"])?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(100))?))?;
        ctx.var("que_sch").set(Val::from(3))?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Rooney", args!["Nice day out, isn't it?"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kid_sch(ctx: &Ctx) -> Script {
    kid_sch_body(ctx, Vec::new()).map(|_| ())
}

fn bard_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
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
    if ctx.var("que_sch").get()?.number()? < 1 {
        ctx.lines_as(
            "Lasda",
            args![
                "Hello there, isn't today",
                "such a wonderful day?",
                "Nice weather always",
                "inspires the poet in me,",
                "and I can't seem to stop",
                "singing my heart out~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("que_sch").get()? == 1 {
            ctx.lines_as(
                "Lasda",
                args![
                    "Why, what a glorious",
                    "day! I should sing a song",
                    "in praise of its wonder!",
                    "Ooooh... La la la-la~",
                    "Girls, girls, giiiirls..."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Excuse me...:......")])?) == 1 {
                ctx.lines_as(
                    "Lasda",
                    args![
                        "Why, what a glorious",
                        "day! I should sing a song",
                        "in praise of its wonder!",
                        "Ooooh... La la la-la~",
                        "Girls, girls, giiiirls..."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Lasda",
                args![
                    "Girls, girls, giiiirls...",
                    "Making my heart",
                    "beat like... Like it's",
                    "going too fast. My heart's",
                    "racing... And you're at the",
                    "finish line--Love Champion!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lasda",
                args![
                    "Oh? Don't stare at me",
                    "like that, you're making",
                    "me blush. Did you need to",
                    "talk to me or something?."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'm sorry.:Are you ^ff0000Lasda Midar^000000?")],
            )?) == 1
            {
                ctx.lines_as(
                    "Lasda",
                    args![
                        "Oh, that's alright.",
                        "...............................",
                        "Girl, I'll laugh at all your",
                        "jokes, and agree with yourv",
                        "politics~ You're hot! But",
                        "baby, you're dumb as bricks~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Lasda", args!["Oh? Oh, yes, that's me.", "How can I help you?"])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("I'm here to deliver this to you.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Lasda",
                args![
                    "Ah! He must have finally",
                    "sent it. Would you please",
                    "give me a second? Let's see..."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args!["^3355FFLasda opened the letter", "and started reading it.^000000"])?;
            ctx.next()?;
            ctx.lines_as("Lasda", args!["Umm...", "Oh, no..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Lasda",
                args![
                    "Ah, I'm sorry for",
                    "making you wait.",
                    "Here, please give",
                    "this money to that",
                    "kid that was supposed",
                    "to come find me here."
                ],
            )?;
            ctx.var("que_sch").set(Val::from(2))?;
            ctx.var("Zeny").set((ctx.var("Zeny").get()? + Val::from(100)))?;
            ctx.next()?;
            ctx.lines_as(
                "Lasda",
                args![
                    "I'd like you to come",
                    "talk to me again later",
                    "if you have the time",
                    "See you around~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("que_sch").get()? == 2 {
                ctx.lines(args!["^3355FFLasda seems to be", "lost in deep thought.^000000"])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("que_sch").get()? == 3 {
                    ctx.lines_as(
                        "Lasda",
                        args![
                            "Oh good, you came back!",
                            "Thanks for delivering that",
                            "letter for me. Listen, you",
                            "mind listening to me for",
                            "a bit? I want to ask you",
                            "for your help with something."
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:No.")])?) == 1 {
                        ctx.lines_as(
                            "Lasda",
                            args![
                                "You see, the letter you",
                                "brought me was from my old",
                                "best friend. I haven't heard",
                                "from him for a while, so",
                                "I was pretty worried..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lasda",
                            args![
                                "Vitre said in his letter",
                                "that he's in jail under",
                                "false charges so he wants",
                                "me to help him out. However,",
                                "there isn't much I can do.",
                                "Do you think you can help him?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Sure.:No.")])?) == 1 {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "Thank you so much!",
                                    "I guess the best thing",
                                    "to do for now is to find",
                                    "my friend in a prison in",
                                    "^FF0000Morocc^000000, and see what",
                                    "you can do to help him.."
                                ],
                            )?;
                            ctx.var("que_sch").set(Val::from(4))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Lasda", args!["I... I guess...", "You must not be", "able to help him too..."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Lasda",
                        args![
                            "I suppose that you",
                            "must already be busy",
                            "doing something else.",
                            "Well, I understand..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("que_sch").get()?.number()? < 8 {
                        ctx.lines_as(
                            "Lasda",
                            args![
                                "I still can't believe",
                                "Vitre is being held",
                                "in prison in Morocc. I mean,",
                                "what could have happened?",
                                "I hope you can help him..."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("que_sch").get()? == 8 {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "He was arrested because",
                                    "he's suspected of espionage?",
                                    "That doesn't make any sense.",
                                    "What could be going on?"
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("^ff0000Krieg^000000 told me that.")])?;
                            ctx.var("@menu").set(choice)?;
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "Krieg? Do you mean Krieg",
                                    "Laje Mandi? I know him quite",
                                    "well, actually. Let me write",
                                    "you a letter or recommendation.",
                                    "Hopefully, it'll be enough to",
                                    "let you enter the prison."
                                ],
                            )?;
                            ctx.var("que_sch").set(Val::from(9))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("que_sch").get()? == 9 {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "Hurry and go bring",
                                    "my letter to Krieg.",
                                    "I wonder what happened...",
                                    "There must be some kind",
                                    "of weird misunderstanding."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("que_sch").get()?.number()? < 19 {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "He really asked you",
                                    "to break him out of jail?",
                                    "Well, I know that's a morally",
                                    "hazy area, but I hope that",
                                    "you do your best to help Vitre."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("que_sch").get()?.number()? < 25 {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "Thanks for offering",
                                    "to help me out. I still",
                                    "can't believe Vitre had",
                                    "the gall to just break",
                                    "out of prison, though..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "I still can't do",
                                    "anything to help him,",
                                    "so please do what you",
                                    "can to take care of him",
                                    "for me. I really appreciate it."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("que_sch").get()? == 25 {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "Ah, you're back. I'm sure",
                                    "you have a lot to ask me",
                                    "right now. You deserve to",
                                    "know that everything I asked",
                                    "you to do was part of a plan",
                                    "to confirm Vitre's guilt."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "We arrested him once",
                                    "we learned that he was",
                                    "an Arunafeltz spy, but we",
                                    "couldn't punish him since",
                                    "we lacked concrete proof."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "That's why we allowed",
                                    "him to escape: we planned",
                                    "on following him to get the",
                                    "proof that we needed. Our",
                                    "sting was even able to round",
                                    "up all of his compatriots!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "I'm sorry for keeping",
                                    "you in the dark, but it was",
                                    "essential to the plan. We",
                                    "couldn't have done it without",
                                    "your help. Please take this",
                                    "reward with our sincere thanks."
                                ],
                            )?;
                            ctx.var("que_sch").set(Val::from(26))?;
                            ctx.call(Function::GetItem, vec![Val::from(12106), Val::from(1)])?;
                            ctx.call(Function::GetExperience, vec![Val::from(600000), Val::from(0)])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Lasda",
                                args![
                                    "We're having too many",
                                    "cases involving spies",
                                    "like Vitre lately. This",
                                    "might be a sign that",
                                    "Arunafeltz is planning",
                                    "to move against us..."
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

pub fn bard_sch(ctx: &Ctx) -> Script {
    bard_sch_body(ctx, Vec::new()).map(|_| ())
}

fn prison_ward_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 4 {
        ctx.lines_as(
            "Jesse",
            args![
                "Only authorized",
                "personnel can enter",
                "this prison. You need",
                "a permit if you want",
                "to be able to enter."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("que_sch").get()? == 4 {
            ctx.lines_as(
                "Jesse",
                args![
                    "What? You want to meet",
                    "the prisoner? I'm sorry,",
                    "but he's not allowed to",
                    "see anyone since he was",
                    "arrested on suspicion",
                    "of espionage."
                ],
            )?;
            ctx.var("que_sch").set(Val::from(5))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("que_sch").get()? == 5 {
                ctx.lines_as(
                    "Jesse",
                    args![
                        "Huh. You're awfully",
                        "persistent. Alright,",
                        "if you can do me a favor,",
                        "I'll let you in. Bring me",
                        "1 dish of ^FF0000Fried Monkey Tails^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Jesse",
                    args![
                        "Of course, I can only let",
                        "you inside. Whether they'll",
                        "actually let you meet the",
                        "prisoner is another matter.",
                        "So do we have a deal?"
                    ],
                )?;
                ctx.var("que_sch").set(Val::from(6))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("que_sch").get()? == 6 {
                    if ctx.call(Function::CountItem, vec![Val::from(12066)])?.number()? > 0 {
                        ctx.lines_as(
                            "Jesse",
                            args![
                                "Ah, that's the stuff!",
                                "Thanks for the Fried",
                                "Monkey Tails~ Alright,",
                                "go talk to ^FF0000Sir Krieg^000000 in",
                                "Morocc Castle. Get his",
                                "approval, and I'll let you in."
                            ],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(12066), Val::from(1)])?;
                        ctx.var("que_sch").set(Val::from(7))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Jesse",
                        args![
                            "Bring me a plate of",
                            "^FF0000Fried Monkey Tails^000000.",
                            "Do it, or I won't help",
                            "you out. I mean, I know",
                            "you mean well, but I'm",
                            "risking my job here..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("que_sch").get()?.number()? < 10 {
                    ctx.lines_as(
                        "Jesse",
                        args![
                            "You didn't get approval",
                            "from Sir Krieg yet? You'd",
                            "better do it, or there's no",
                            "point in entering this prison.",
                            "You can't just sneak around",
                            "inside this place, you know?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("que_sch").get()? == 10 {
                    ctx.lines_as(
                        "Jesse",
                        args!["Did you really get", "Sir Krieg's approval?", "Alright, you may enter now."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("ra_in01"), Val::from(48), Val::from(355)])?;
                    return Err(Stop::End);
                } else if ctx.var("que_sch").get()?.number()? < 19 {
                    ctx.lines_as("Jesse", args!["Do you want to enter?"])?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No")])?) == 1 {
                        ctx.lines_as(
                            "Jesse",
                            args![
                                "Be careful when you talk",
                                "to that guy: he's a smooth",
                                "talker, and almost charmed",
                                "a lot of the guards into",
                                "letting him go free."
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("ra_in01"), Val::from(48), Val::from(355)])?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Jesse",
                        args![
                            "Take your time.",
                            "If you're not mentally",
                            "prepared, then it's not",
                            "a good idea to talk",
                            "to the prisoner."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.var("que_sch").get()?.number()? < 19 {
                    ctx.lines_as(
                        "Jesse",
                        args![
                            "Argh, I'm in trouble.",
                            "The prisoner escaped.",
                            "How could I let this...",
                            "happen?! Damn, I need to",
                            "report this to Mr. Krieg"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    ctx.lines_as(
        "Jesse",
        args![
            "What a relief! They",
            "really set it up so that",
            "the prisoner could escape?",
            "Well, I thought I was going",
            "to get fired for all that."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn prison_ward_sch(ctx: &Ctx) -> Script {
    prison_ward_sch_body(ctx, Vec::new()).map(|_| ())
}

fn public_security_officer_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 7 {
        ctx.lines_as(
            "Krieg",
            args![
                "I'm in charge of public",
                "security here in Morocc.",
                "Lately, there have been",
                "more incidents disturbing",
                "the public peace and",
                "many unsettling rumors..."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 7 {
        ctx.lines_as("Krieg", args!["Hello, adventurer.", "How may I help you?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I'd like to see a prisoner, Mr. Vitre.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Krieg",
            args![
                "Vitre? I'm sorry, but",
                "I can't approve of that.",
                "I'd allow visitors for normal",
                "prisoners, but not for people",
                "suspected of espionage.",
                "That's why he's in jail."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Krieg",
            args![
                "If I knew you personally,",
                "or if someone I trust can",
                "vouch for you, then I'd",
                "reconsider letting you",
                "meet Vitre. Otherwise,",
                "I just can't do it."
            ],
        )?;
        ctx.var("que_sch").set(Val::from(8))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 8 {
        ctx.lines_as(
            "Krieg",
            args![
                "I can't let you meet",
                "Vitre until I'm absolutely",
                "sure that you're not involved",
                "with any espionage activities."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 9 {
        ctx.lines_as(
            "Krieg",
            args![
                "I can't let you meet",
                "Vitre until I'm absolutely",
                "sure that you're not involved",
                "with any espionage activities."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Here's a letter from Lasda.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Krieg",
            args![
                "Lasda? Now there's a",
                "man I hold in high regard.",
                "Please let me read what",
                "he has to say. Hmmm..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Krieg",
            args![
                "Alright, I'll let you visit",
                "Vitre. I'll send a message",
                "to the prison ward so that",
                "he'll let you talk to him."
            ],
        )?;
        ctx.var("que_sch").set(Val::from(10))?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()?.number()? < 26 {
        ctx.lines_as(
            "Krieg",
            args!["Hmm... This is almost too", "difficult for me to handle.", "What should I do?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Krieg",
        args![
            "I understand that you were",
            "instrumental in solving a",
            "problem regarding public",
            "safety. I'd just like to thank",
            "you, and apologize for any",
            "trouble I might have caused."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn public_security_officer(ctx: &Ctx) -> Script {
    public_security_officer_body(ctx, Vec::new()).map(|_| ())
}

fn upset_looking_bard_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("que_sch").get()?.number()? < 10 {
        ctx.lines_as("Vitre", args!["............."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 10 {
        ctx.lines_as(
            "Vitre",
            args![
                "...............................",
                "...............................",
                "...............................",
                "Damn, what should I do?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Excuse me...")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Vitre",
            args![
                "Hm? I'm Vitre Bizlita.",
                "As you can see, I'm not in",
                "any real position to help",
                "you. Still, did you want",
                "to ask me something?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Nothing.:I'm here on behalf of Mr. Lasda.")])? {
            1 => {
                ctx.lines_as(
                    "Vitre",
                    args!["Well...", "It's nice to", "receive visitors.", "Jail can be lonely...."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Vitre",
                    args![
                        "Oh, good. Lasda finally",
                        "got my letter, eh? I don't",
                        "know what the hell's going",
                        "on. I mean, all I remember",
                        "is that these strange men",
                        "came and brought me here."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vitre",
                    args![
                        "I... I don't think",
                        "they'll let me out",
                        "of here. I mean, they",
                        "jailed me and there's no",
                        "proof I did anything wrong."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vitre",
                    args![
                        "My life is in danger as",
                        "long as I'm here, so I have",
                        "to get out as soon as I can.",
                        "Luckily, I figured out that",
                        "I can open these doors if",
                        "I just had 2 things."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vitre",
                    args![
                        "I just need a ^FF0000Megaphone^000000",
                        "and a ^FF0000Violin^000000. Luckily, they're",
                        "pretty mundane objects, so no",
                        "one would suspect anything if",
                        "you brought them here. Um...",
                        "You will help me, won't you?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vitre",
                    args![
                        "I understand if you have any",
                        "doubts about my innocence.",
                        "But think about it: wouldn't",
                        "you want to see what I do",
                        "with a Megaphone and Violin?",
                        "Sounds pretty cool, huh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou can hear someone",
                    "mumbling from the cell",
                    "next to Vitre's.^000000"
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "????",
                    args!["I wonder how the lady", "in the Dancer Job Change", "place is doing by now..."],
                )?;
                ctx.var("que_sch").set(Val::from(11))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("que_sch").get()?.number()? < 18 {
        ctx.lines_as(
            "Vitre",
            args![
                "Didn't you bring the",
                "Megaphone and Violin?",
                "The longer I sit here,",
                "the more likely it is",
                "that they'll kill me!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("que_sch").get()? == 18 {
        if (ctx.call(Function::CountItem, vec![Val::from(7040)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(1901)])?.number()? > 0)
        {
            ctx.lines_as(
                "Vitre",
                args![
                    "You brought me a",
                    "Megaphone and Violin?",
                    "Perfect! Now, step aside",
                    "Can't have you getting hurt."
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "...............................",
                "...............................",
                "..............................."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFVitre drew in a deep",
                "breath, and then smashed",
                "the steel bars of his cell",
                "with the Violin. Surprisingly,",
                "the door swings open with",
                "a very loud noise.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Vitre",
                args![
                    "Heh! That was a little",
                    "harder than I thought,",
                    "but it looks like I'm free~",
                    "Alright, let's get a move on."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Jesse", args!["What?! What's this noise?!"])?;
            ctx.next()?;
            ctx.call(Function::EnableNpc, vec![Val::from("Jesse#sch")])?;
            ctx.lines_as(
                "Jesse",
                args!["Hey! How did you", "get out of your cell?!", "Get back in there, NOW!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vitre",
                args![
                    "If you were me, would",
                    "you go back in your cell",
                    "just because someone's",
                    "yelling at you? Forget it~"
                ],
            )?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFVitre raised the Megaphone",
                "to his mouth, and drew in",
                "another deep breath.^000000"
            ])?;
            ctx.next()?;
            ctx.lines_as("Vitre", args!["Wah!"])?;
            ctx.next()?;
            ctx.lines_as("Jesse", args!["Arg...", "Oh Lord...!", "M-my ears..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Vitre",
                args!["Heh! I love it when", "a plan comes together~", "Let's get out of here!"],
            )?;
            ctx.next()?;
            ctx.call(Function::DelItem, vec![Val::from(1901), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(7040), Val::from(1)])?;
            ctx.var("que_sch").set(Val::from(19))?;
            ctx.close_window()?;
            ctx.call(Function::DisableNpc, vec![Val::from("Jesse#sch")])?;
            ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(294), Val::from(153)])?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Vitre",
            args![
                "Didn't you bring",
                "a Megaphone and",
                "a Violin? Please",
                "hurry, I don't have",
                "much time left!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Vitre",
            args!["Heh! I love it when", "a plan comes together~", "Let's get out of here!"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(294), Val::from(153)])?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn upset_looking_bard_sch(ctx: &Ctx) -> Script {
    upset_looking_bard_sch_body(ctx, Vec::new()).map(|_| ())
}

fn jesse_sch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn jesse_sch(ctx: &Ctx) -> Script {
    jesse_sch_body(ctx, Vec::new()).map(|_| ())
}

fn jesse_sch_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Jesse#sch")])?;
    return Err(Stop::End);
}

pub fn jesse_sch_oninit(ctx: &Ctx) -> Script {
    jesse_sch_oninit_body(ctx, Vec::new()).map(|_| ())
}
