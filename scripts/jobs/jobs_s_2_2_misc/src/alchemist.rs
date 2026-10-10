#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn alchemist_guildsman_am(ctx: &Ctx) -> Script {
    let mut l_items: Vec<Val> = Vec::new();
    ctx.mes("[Parmy Gianino]")?;
    if ctx.var("Upper").get()? == 1 {
        ctx.lines(args![
            "Welcome to the",
            "Alchemist Unio--",
            "I-Impossible! How c-can",
            "something like this happen?"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Parmy Gianino",
            args![
                "Wait, wait...",
                "I'm sorry. I was confused,",
                "that's all. You look just like",
                "someone I used to know. ",
                "Still, I get this weird",
                "feeling about you..."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT {
        if ctx.var("BaseJob").get()? == constants::JOB_ALCHEMIST {
            ctx.lines(args![
                Val::from("Welcome, ") + ctx.player().name()? + Val::from("."),
                "The Alchemist Union",
                "is busy today, like always."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Parmy Gianino",
                args![
                    "Everyone is busy with their",
                    "own research, but recently, some headway has been made in the field of biotechnology."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Parmy Gianino", args!["Everyone is hoping that the biotechnological studies will yield positive results. Speaking of which, I wonder how the Alchemists working on artificial life are doing..."])?;
            return ctx.close();
        } else if ctx.var("BaseClass").get()? == constants::JOB_NOVICE {
            ctx.lines(args![
                "Welcome to the",
                "Alchemist Union.",
                "We are recruiting",
                "talented people",
                "with novel ideas."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Parmy Gianino",
                args!["If you're interested in working with chemistry, visit us later when you become more knowledgable."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Parmy Gianino",
                args![
                    "Just one thing:",
                    "You've got to have",
                    "knowledge of items",
                    "as a Merchant first."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines(args![
            "Welcome to the",
            "Alchemist Union.",
            "We are recruiting",
            "talented people",
            "with novel ideas."
        ])?;
        ctx.next()?;
        ctx.lines_as("Parmy Gianino", args!["If you know any exceptional Merchants, by all means, please refer them to us. Those types of people tend to have a talent for Alchemy and experimentation~"])?;
        return ctx.close();
    }
    if ctx.var("alch_q").get()? == 0 {
        ctx.lines(args!["Welcome to the", "Alchemist Union.", "How may I help you?"])?;
        ctx.next()?;
        match ctx.menu(&[
            "I would like to learn about Alchemists.",
            "I want to become an Alchemist.",
            "Nothing.",
        ])? {
            0 => {
                ctx.lines_as("Parmy Gianino", args!["Alchemists study and create new substances and items out of existing materials. Our knowledge allows us to change the properties of chemicals at the atomic level."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Parmy Gianino",
                    args![
                        "Most people think our final goal",
                        "is to create gold, but that's not the entire truth. We also want to create things like medicines",
                        "and new materials."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Parmy Gianino", args!["A few of us research the", "creation of life, although many of us consider that god's territory. That field is so complex, most of us deal with slightly less complicated projects anyway."])?;
                ctx.next()?;
                ctx.lines_as("Parmy Gianino", args!["If you are interested in becoming an Alchemist, I recommend that you first get a lot of experience as a Merchant. Being a Merchant is a great opportunity to learn about materials as you deal with them."])?;
                ctx.next()?;
                ctx.lines_as("Parmy Gianino", args!["Whether or not you try to become", "an Alchemist is your decision. The road to becoming an Alchemist is very challenging, and you'll need to focus on experimentation and research, instead of commerce."])?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Parmy Gianino",
                    args![
                        "Is that so?",
                        "Nice to meet you.",
                        "My name is Parmy Gianino",
                        "of the Alchemist Union."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Parmy Gianino",
                    args![
                        "If you join our Union and",
                        "complete the training, you",
                        "will be officially recognized",
                        "as an Alchemist and be able",
                        "to contribute to our research."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Parmy Gianino",
                    args![
                        "But we don't accept everyone.",
                        "You must have a lot of tenacity",
                        "and sincere devotion in exploring",
                        "the various fields of science."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Parmy Gianino",
                    args![
                        "There are a couple",
                        "of requirements to join",
                        "the Alchemist Union, but",
                        "we'll discuss that",
                        "after you apply."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Parmy Gianino",
                    args!["Well then, would", "you like to apply", "for registration?"],
                )?;
                ctx.next()?;
                if ctx.menu(&["I would like to apply.", "I'll do it later."])? == 0 {
                    if ctx.var("JobLevel").get()?.number()? < 40 {
                        ctx.lines_as(
                            "Parmy Gianino",
                            args![
                                "Hmmm...",
                                "Just a moment.",
                                "I'm sorry to say that",
                                "you're not experienced",
                                "enough as a Merchant to",
                                "join us right now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Parmy Gianino",
                            args![
                                "You must be at least",
                                "^551A8BJob Level 40^000000 to become",
                                "an Alchemist. Come back",
                                "later when you meet the",
                                "Job Level requirement, okay?"
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.lines_as("Parmy Gianino", args!["Alright, your application has been accepted. Now, you must pay the 50,000 Zeny application fee and bring some items before you can begin your formal training."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Parmy Gianino",
                        args![
                            "But if you bring an ^551A8BOld Magic Book^000000 and ^551A8BHammer of Blacksmith^000000,",
                            "we will accept that as a substitute for the item requirement."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Parmy Gianino", args!["Now...", "Please sign", "the application."])?;
                    ctx.next()?;
                    ctx.var("@menu")
                        .set(runtime::select_values(ctx, &[ctx.call(Function::StrCharInfo, args![0])?])?)?;
                    ctx.lines_as("Parmy Gianino", args!["Good, good. Now, if you have", "the Zeny for your application fee ready, I will tell you which items you will need to bring. Now, pay attention."])?;
                    ctx.next()?;
                    if ctx.player().zeny()? < 50000 {
                        ctx.lines_as(
                            "Parmy Gianino",
                            args![
                                "Uh oh. You don't",
                                "seem to have enough Zeny.",
                                "Come back to me when you have 50,000 Zeny, otherwise we can't process your application."
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.player().set_zeny(ctx.player().zeny()? - 50000)?;
                    ctx.lines_as(
                        "Parmy Gianino",
                        args!["Let's see.", Val::from("") + ctx.player().name()?, "needs to bring..."],
                    )?;
                    match ctx.rand_range(1, 3)? {
                        1 => {
                            ctx.var("alch_q").set(Val::from(1))?;
                            ctx.quests().start(2028)?;
                            ctx.mes("^551A8B7 Berserk Potions^000000.")?;
                        }
                        2 => {
                            ctx.var("alch_q").set(Val::from(2))?;
                            ctx.quests().start(2029)?;
                            ctx.mes("^551A8B100 Mini Furnaces^000000.")?;
                        }
                        3 => {
                            ctx.var("alch_q").set(Val::from(3))?;
                            ctx.quests().start(2030)?;
                            ctx.mes("^551A8B7,500 Fire Arrows^000000.")?;
                        }
                        _ => {}
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Parmy Gianino",
                        args![
                            "Once you've gathered",
                            "those items, come back",
                            "to me and your training",
                            "as an Alchemist will begin.",
                            "See you soon~"
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Parmy Gianino",
                    args!["Talented Merchants", "are always welcome here.", "Please come back soon."],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as("Parmy Gianino", args!["Umm...", "Please let me know", "if you need anything."])?;
                return ctx.close();
            }
            _ => {}
        }
    } else if ctx.var("alch_q").get()?.number()? >= 1 && ctx.var("alch_q").get()?.number()? <= 3 {
        if ctx.items().count(1006)? > 0 && ctx.items().count(1005)? > 0 {
            ctx.lines(args![
                "Well now~!",
                "You've brought an",
                "Old Magic Book and",
                "a Hammer of Blacksmith.",
                "We'll put these items",
                "to good use."
            ])?;
            ctx.next()?;
            ctx.items().take(1006, 1)?;
            ctx.items().take(1005, 1)?;
            ctx.lines_as(
                "Parmy Gianino",
                args![
                    "Okay, now you need to learn",
                    "the basics to being an Alchemist and learn the procedures for mixing chemicals and medicines."
                ],
            )?;
            ctx.var("alch_q").set(Val::from(4))?;
            if ctx.call(Function::CheckQuest, args![2028])? != -1 {
                ctx.quests().change(2028, 2031)?;
            } else if ctx.call(Function::CheckQuest, args![2029])? != -1 {
                ctx.quests().change(2029, 2031)?;
            } else {
                ctx.quests().change(2030, 2031)?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Parmy Gianino",
                args![
                    "But before all of that, you need to speak to Raspuchin. I'm not really sure what you'll be talking about with him..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Parmy Gianino", args!["It shouldn't be anything extraordinary, but you're still required to speak to Raspuchin, since apparently he's a part of the Alchemist selection process."])?;
            return ctx.close();
        }
        match ctx.var("alch_q").get()?.number()? {
            1 => {
                runtime::local_set(&mut l_items, &Val::from(0), Val::from(657), false);
                runtime::local_set(&mut l_items, &Val::from(1), Val::from(7), false);
            }
            2 => {
                runtime::local_set(&mut l_items, &Val::from(0), Val::from(612), false);
                runtime::local_set(&mut l_items, &Val::from(1), Val::from(100), false);
            }
            3 => {
                runtime::local_set(&mut l_items, &Val::from(0), Val::from(1752), false);
                runtime::local_set(&mut l_items, &Val::from(1), Val::from(7500), false);
            }
            _ => {}
        }
        if runtime::op(
            &ctx.call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?,
            ">=",
            &runtime::local_get(&l_items, &Val::from(1), false),
        )?
        .is_true()
        {
            ctx.lines(args![
                "Seems like",
                "you're all ready.",
                "The Union will put",
                "these items to good use."
            ])?;
            ctx.next()?;
            ctx.call(
                Function::DelItem,
                vec![
                    runtime::local_get(&l_items, &Val::from(0), false),
                    runtime::local_get(&l_items, &Val::from(1), false),
                ],
            )?;
            ctx.lines_as(
                "Parmy Gianino",
                args![
                    "Okay, now you need to learn",
                    "the basics to being an Alchemist and learn the procedures for mixing chemicals and medicines."
                ],
            )?;
            ctx.var("alch_q").set(Val::from(4))?;
            if ctx.call(Function::CheckQuest, args![2028])? != -1 {
                ctx.quests().change(2028, 2031)?;
            } else if ctx.call(Function::CheckQuest, args![2029])? != -1 {
                ctx.quests().change(2029, 2031)?;
            } else {
                ctx.quests().change(2030, 2031)?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Parmy Gianino",
                args![
                    "But before all of that, you need to speak to Raspuchin. I'm not really sure what you'll be talking about with him..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Parmy Gianino", args!["It shouldn't be anything extraordinary, but you're still required to speak to Raspuchin, since apparently he's a part of the Alchemist selection process."])?;
            return ctx.close();
        }
        ctx.lines(args![
            "Aren't you ready?",
            "Like I said before,",
            "you must bring",
            Val::from("^551A8B")
                + shared::other_global_functions::f_insertplural(
                    ctx,
                    vec![
                        runtime::local_get(&l_items, &Val::from(1), false),
                        ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                    ]
                )?
                + Val::from("^000000.")
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Parmy Gianino",
            args!["Come back when you", "have prepared the", "required items."],
        )?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 4 {
        ctx.lines(args![
            "Go and talk to",
            "Mr. Raspuchin.",
            "He's involved in the",
            "Alchemist selection process, whatever that might mean."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Parmy Gianino",
            args![
                "Hopefully, it",
                "won't be too much of",
                "a problem. I guess he'll just interview you, and ask you",
                "some simple questions."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines(args!["Ah, I'm sorry, but", "I'm busy right now~"])?;
        ctx.next()?;
        ctx.lines_as(
            "Parmy Gianino",
            args![
                "Why don't you ask",
                "someone else if you're",
                "not sure who to visit",
                "next? Good luck~"
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn fastidious_alchemist_am(ctx: &Ctx) -> Script {
    let mut l_w_point = 0;
    ctx.mes("[Raspuchin Gregory]")?;
    if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT {
        if ctx.var("BaseJob").get()? == constants::JOB_ALCHEMIST {
            ctx.lines(args!["Heeheehee", "keheheh~!", "Eh? What do you want?!"])?;
            ctx.next()?;
            ctx.lines_as("Raspuchin Gregory", args!["You're not here to steal my experimental results or plagiarize my work, are you? How dare you consider intellectual theft!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "You're not, are you?",
                    "Well, as a colleague,",
                    "let me just warn you",
                    "that such tricks aren't",
                    "tolerated here in the",
                    "Alchemist Union!"
                ],
            )?;
            return ctx.close();
        } else if ctx.var("BaseClass").get()? == constants::JOB_NOVICE {
            ctx.lines(args![
                "Heeheehee",
                "keheheh~!",
                "How cute, you've come",
                "all this way just to play..."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "I'll let you",
                    "go this time...",
                    "But next time, don't",
                    "expect to leave so easily..."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines(args!["What is it?!", "You're curious as", "to what I'm doing?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args!["Heehee", "keheheh~!", "Why, I'm busy", "researching,", "of course!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Once this",
                    "potion is complete...",
                    "You can use it to take",
                    "over an entire nation!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Hee hee hee!",
                    "Something this",
                    "dangerous has to",
                    "be kept a secret!",
                    "Understand?"
                ],
            )?;
            return ctx.close();
        }
    }
    if ctx.var("alch_q").get()? == 0 {
        ctx.lines(args!["Heeheehee", "keheheh~!", "What do you", "want, kid?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Raspuchin Gregory",
            args!["A Merchant should go and set up shop and vend items. Why are you wandering in a place like this?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Raspuchin Gregory",
            args!["Heheheh~!", "Go vend somwhere else!", "And leave me to my", "dark enterprise!"],
        )?;
        return ctx.close();
    } else if ctx.var("alch_q").get()?.number()? >= 1 && ctx.var("alch_q").get()?.number()? <= 3 {
        ctx.lines(args!["Heeheehee", "keheheh~!", "What do you", "want, kid?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Raspuchin Gregory",
            args!["What...?", "Learn Alchemy?!", "Don't even speak", "such nonsense!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Raspuchin Gregory", args!["Even if you tried studying for a thousand years, maybe even more, it'd be useless to you! Forget about it and just worry about your store!"])?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 4 || ctx.var("alch_q").get()? == 5 {
        if ctx.var("alch_q").get()? == 4 {
            ctx.lines(args!["Heeheehee", "keheheh~!", "What do you", "want, kid?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args!["What...?", "Join the Union!?", "I don't like it...", "I just don't...!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Nowadays, anyone thinks they can",
                    "be Alchemists just by knowing how to mix a few herbs. That's why my interview is necessary."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Heeheehee",
                    "keheheh~!",
                    "I plan on weeding out all the dumb and incompetent, and chase them",
                    "all away! We don't need morons!"
                ],
            )?;
            ctx.next()?;
            if ctx.var("JobLevel").get()? == 50 {
                ctx.lines_as("Raspuchin Gregory", args!["Wait...", "Maybe I've", "misjudged you."])?;
                if ctx.var("Sex").get()? == constants::SEX_MALE {
                    ctx.lines(args![
                        "You might be a pretty boy,",
                        "but I can tell you're smart",
                        "from your eyes."
                    ])?;
                } else {
                    ctx.lines(args!["Huh. You're a cutie alright,", "but I can tell you've got brains."])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "You're not just some stupid kid.",
                        "I can tell youve gone through some rough times as a Merchant. Excellent. Keh heh heh~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args!["Fine, just so we don't insult each other's intelligence, I'll just let you pass the interview."],
                )?;
                ctx.next()?;
                ctx.lines_as("Raspuchin Gregory", args!["So hurry up, become an Alchemist, do some good research, and you might turn out to be of some help to me. Hahahahahaha~!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "Now go to Darwin!",
                        "He'll teach you how to do the experiments. Just tell him that",
                        "I sent you."
                    ],
                )?;
                ctx.var("alch_q").set(Val::from(6))?;
                ctx.quests().change(2031, 2032)?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "Surprised, are you?",
                        "Keheheh~ If you thought",
                        "becoming an Alchemist was",
                        "just a matter of changing",
                        "your clothes, then you're",
                        "sadly mistaken."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args!["Now, try solving", "all these problems.", "Let's see how smart", "really are."],
                )?;
            }
        } else if ctx.var("alch_q").get()? == 5 {
            ctx.lines(args![
                "What...?!",
                "You want to take",
                "the test again?!",
                "I thought I told",
                "you to leave!"
            ])?;
            ctx.next()?;
            ctx.lines_as("Raspuchin Gregory", args!["I don't like it...", "I don't like this!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Fine...",
                    "I'll try to overlook your pitiful performance last time and give",
                    "you another chance. Don't screw",
                    "up again, got it?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args!["Now then,", "give me all the", "^551A8Bright^000000 answers", "this time."],
            )?;
        }
        ctx.next()?;
        match ctx.rand_range(1, 3)? {
            1 => {
                ctx.lines_as("Raspuchin Gregory", args!["12 + 23 + 34 + 45 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 114 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["1000 - 36 - 227 - 348 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 389 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["14 * 17 * 3 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 714 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["9765 / 3 / 5 / 7 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 93 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(47 * 28) - (1376 / 4) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 972 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(2646 / 7) + (13 * 28) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 742 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "How much do",
                        "12 Red Potions,",
                        "1 Butterfly Wing",
                        "and 5 Fly Wings cost",
                        "after a 24 % discount?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 909 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args!["What is the", "total weight of", "3 Scimiters, 2 Helms", "and 1 Long Coat?"],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 450 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "What is the",
                        "total defense of",
                        "a Biretta, Mantle,",
                        "Opera Mask, Ribbon,",
                        "Muffler, Boots, and",
                        "Ear Muffs?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 20 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "If you buy 5 Helms",
                        "with a 24 % discount",
                        "and sell it at 20",
                        "how much profit",
                        "do you earn?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 8800 {
                    l_w_point += 1;
                }
            }
            2 => {
                ctx.lines_as("Raspuchin Gregory", args!["13 + 25 + 37 + 48 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 123 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["1000 - 58 - 214 - 416 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 312 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["12 * 24 * 3 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 864 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["10530 / 3 / 5 / 2 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 351 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(35 * 19) - (1792 / 7) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 409 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(2368 / 8) + (24 * 17) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 704 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(2646 / 7) + (13 * 28) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 742 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "What is the",
                        "total price of",
                        "15 Green Potions,",
                        "6 Magnifiers and",
                        "4 Traps after",
                        "a 24 % discount?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 934 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args!["What is the", "total weight of", "3 Ring Pommel Sabers,", "4 Caps, and 2 Boots?"],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 550 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "What is the",
                        "total defense of",
                        "a Buckler, Long Coat,",
                        "Gas Mask, Big Ribbon,",
                        "Cute Ribbon, Sakkat,",
                        "and Glasses?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 16 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "How much profit do you",
                        "make if you buy Tights",
                        "at a 24 % discount and",
                        "sell it at 20 % of",
                        "the normal price?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 8520 {
                    l_w_point += 1;
                }
            }
            3 => {
                ctx.lines_as("Raspuchin Gregory", args!["12 + 23 + 34 + 45 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 114 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["1000 - 58 - 214 - 416 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 312 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["14 * 17 * 3 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 714 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["10530 / 3 / 5 / 2 = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 351 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(47 * 28) - (1376 / 4) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 972 {
                    l_w_point += 1;
                }
                ctx.lines_as("Raspuchin Gregory", args!["(2646 / 7) + (13 * 28) = ?"])?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 742 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "What is the",
                        "total cost of",
                        "6 Red Potions,",
                        "7 Green Potions,",
                        "and 8 Fly Wings",
                        "after a 24 % discount?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 798 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args!["What is the", "total weight of", "2 Ring Pommel Sabers,", "3 Caps, and 3 boots?"],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 480 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "What is the",
                        "total defense of",
                        "a Mirror Shield, Mr. Smile, Leather Jacket, Silk Robe, Wedding Veil, Muffler, and Eye Patch?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 12 {
                    l_w_point += 1;
                }
                ctx.lines_as(
                    "Raspuchin Gregory",
                    args![
                        "If you buy 4 Padded Armors",
                        "at a 24% discount and sell",
                        "them at 20% of the original",
                        "price, how much profit would",
                        "you make from this sale?"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                if input != 7680 {
                    l_w_point += 1;
                }
            }
            _ => {}
        }
        if l_w_point == 0 {
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Ooh...",
                    "Excellent! Great!",
                    "You got them all correct!?",
                    "Keheheh, I have no choice but to acknowledge you..."
                ],
            )?;
            ctx.next()?;
        } else if l_w_point == 1 {
            ctx.lines_as(
                "Raspuchin Gregory",
                args!["You got one wrong!", "But I'll let it slide.", "You pass the interview!"],
            )?;
            ctx.next()?;
        } else if l_w_point == 2 && ctx.var("alch_q").get()? == 5 {
            ctx.lines_as(
                "Raspuchin Gregory",
                args!["You've got serious", "weaknesses in math,", "but I'll let you go this time..."],
            )?;
            ctx.next()?;
        } else {
            ctx.var("alch_q").set(Val::from(5))?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Keheheh! Idiot!",
                    "Just listening to your",
                    "answers is making me feel",
                    "stupider! You might as well",
                    "have got them all wrong!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "How can a person that",
                    "can't even answer all of",
                    "these simple questions think",
                    "of becoming an Alchemist?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Raspuchin Gregory", args!["Hm...?", "Did you get", "any right?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Raspuchin Gregory",
                args![
                    "Fool! Even if you make one little mistake, everything goes wrong",
                    "in Alchemy! Now get out of here!",
                    "You make me sick!"
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Raspuchin Gregory",
            args![
                "So hurry up, become an Alchemist, do some good research, and you might turn out to be of some help to me. Hahahahahaha~!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Raspuchin Gregory",
            args![
                "Now go to Darwin!",
                "He'll teach you how to do the experiments. Just tell him that",
                "I sent you."
            ],
        )?;
        ctx.var("alch_q").set(Val::from(6))?;
        ctx.quests().change(2031, 2032)?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 6 {
        ctx.lines(args!["What are you doing?", "Go and find Darwin now."])?;
        ctx.next()?;
        ctx.lines_as(
            "Raspuchin Gregory",
            args!["Keheheheheheheheh~", "Don't think this is the end of it!"],
        )?;
        return ctx.close();
    }
    ctx.lines(args!["Keheheheheheheheh~", "Don't think this is the end of it!"])?;
    ctx.close()
}

pub fn studying_man_am(ctx: &Ctx) -> Script {
    let mut l_w_point = 0;
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you are carrying -",
            "- too many items with you. -",
            "- Please come back again -",
            "- after you store some items into kafra storage. -"
        ])?;
        return ctx.close();
    }
    ctx.mes("[Darwin]")?;
    if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT {
        if ctx.var("BaseJob").get()? == constants::JOB_ALCHEMIST {
            ctx.lines(args!["Ah...", "You...", "You've become", "an Alchemist."])?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Remember...",
                    "In your quest",
                    "to make your",
                    "dreams come true,",
                    "do not lose what",
                    "you cherish."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Darwin", args!["Ah...", "Harmona...", "My love..."])?;
            return ctx.close();
        } else {
            ctx.lines(args![
                "When you have",
                "your dreams, you",
                "have everything.",
                "Without them, you have",
                "nothing more to lose."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "These cursed eyes...",
                    "They've lost sight of",
                    "my dreams a long time ago.",
                    "Ha ha ha ha..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Does paradise",
                    "really exist...?",
                    "Not without my love...",
                    "Not without Harmona..."
                ],
            )?;
            return ctx.close();
        }
    }
    if ctx.var("alch_q").get()? == 6 {
        ctx.mes("...")?;
        ctx.next()?;
        ctx.lines_as("Darwin", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Darwin", args!["...", "......", "Who is it...?"])?;
        ctx.next()?;
        ctx.call(Function::Monster, args!["alde_alche", 13, 15, "Wolf", 1013, 1])?;
        ctx.call(Function::KillMonster, args!["alde_alche", "All"])?;
        ctx.lines_as(
            "Darwin",
            args!["A wolf?", "Or a human?", "You must be seeking", "something, are you not?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "After all...",
                "Everyone has desires",
                "to fulfill. Be be careful.",
                "Do not be like the wild",
                "wolf drawn to the flowers."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args!["In your efforts to gain something else, you may end up sacrificing something precious to you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "Cultivating joy and happiness",
                "is much like cultivating flowers.",
                "If something is missing, the",
                "flower will wilt away..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Darwin", args!["What brings you", "to this kind of place?"])?;
        ctx.next()?;
        match ctx.menu(&["I want to learn how to experiment.", "Tell me more about flowers.", "Nothing."])? {
            0 => {
                ctx.lines_as(
                    "Darwin",
                    args![
                        "You wish to",
                        "learn Alchemy?",
                        "Everything I know,",
                        "I've learned for the",
                        "sake of making my",
                        "dream come true..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "I'll teach",
                        "you the basics...",
                        "But everything you",
                        "learn afterwards must",
                        "be directed through",
                        "your own motivations."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "I will teach you",
                        "how to make simple",
                        "medicine. So please",
                        "bring the following",
                        "materials right away."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "^551A8B3 Medicine Bowls^000000,",
                        "^551A8B3 Empty Bottles^000000,",
                        "^551A8B1 Red Herb^000000,",
                        "^551A8B1 Yellow Herb^000000 and",
                        "^551A8B1 White Herb^000000."
                    ],
                )?;
                ctx.var("alch_q").set(Val::from(7))?;
                ctx.quests().change(2032, 2033)?;
                ctx.next()?;
                ctx.lines_as("Darwin", args!["Once you have", "prepared everything,", "return to me."])?;
                return ctx.close();
            }
            1 => {
                ctx.lines_as(
                    "Darwin",
                    args![
                        "Flowers...?",
                        "In the darkest",
                        "recesses of my mind,",
                        "there is a blossum",
                        "that I faintly remember..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "For the one that",
                        "I love, I put all",
                        "of my efforts into",
                        "researching that one thing."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "I won't tell you the details,",
                        "but I was basically researching",
                        "the relationship between",
                        "wolves and flowers."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "But yes...",
                        "It was a flower.",
                        "With its shine, it was said",
                        "to let you see paradise.",
                        "The ^551A8BIllusion Flower^000000..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "I even made",
                        "a Homunculus,",
                        "but no one believed that I could create new life from a flower..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "Then...",
                        "Well, some other things happened, and now I have nothing left. Time no longer has any meaning for me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "Ahh...",
                        "Harmona...",
                        "Where have you gone?",
                        "I hope you're in a field",
                        "of beautiful flowers..."
                    ],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Darwin",
                    args![
                        "Consider what",
                        "is most precious",
                        "to you. It cannot",
                        "be protected if you",
                        "do not recognize it."
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    } else if ctx.var("alch_q").get()? == 7 {
        ctx.lines(args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Darwin", args!["...", "......", "Who is it...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "Ah...",
                "You are the one who",
                "wishes to learn Alchemy.",
                "Have you prepared everything?"
            ],
        )?;
        ctx.next()?;
        if ctx.items().count(710)? > 0 {
            ctx.lines_as("Darwin", args!["Wait.", "That Illusion Flower.", "How did you get that?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Where did you find it?!",
                    "The flower that slowly",
                    "blooms under the",
                    "moonlight?",
                    "It's beautiful...!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args!["Th-That flower...", "Please let me see it.", "The Illusion Flower!", "Uwaaaaaaah!!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Would you be so kind",
                    "as to let me have this flower?",
                    "I'm sure that this is the Moonlight Flower that I've been seeking!"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Sorry, I can't give it to you.", "I brought it to give to you."])? == 0 {
                ctx.lines_as(
                    "Darwin",
                    args![
                        "I understand.",
                        "You can't give",
                        "such a precious",
                        "flower to just anyone.",
                        "Well... It's okay."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Darwin", args!["It just brought back old memories. I shouldn't have asked in the first place. In any case, please bring what is needed for the experiment."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args!["Please leave that flower", "somewhere else. It brings", "back too many memories..."],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Darwin",
                args![
                    "Are you",
                    "serious?!",
                    "Thank you!",
                    "Such a precious flower.",
                    "Ah, Harmona, my love..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Yes...",
                    "I shall repay you for this.",
                    "I shall plant all of my knowledge of Alchemy directly into your mind..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args!["Open your eyes wide,", "and look into my eyes!!", "Don't stop until the end!!"],
            )?;
            ctx.next()?;
            ctx.mes("Lorem ipsum dolor sit amet,")?;
            ctx.next()?;
            ctx.lines(args!["Lorem ipsum dolor sit amet,", "consectetuer adipiscing elit."])?;
            ctx.next()?;
            ctx.lines(args![
                "Lorem ipsum dolor sit amet,",
                "consectetuer adipiscing elit.",
                "Aenean fermentum ullamcorper."
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "Lorem ipsum dolor sit amet,",
                "consectetuer adipiscing elit.",
                "Aenean fermentum ullamcorper.",
                "Vestibulum ante ipsum primis in"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "Lorem ipsum dolor sit amet,",
                "consectetuer adipiscing elit.",
                "Aenean fermentum ullamcorper.",
                "Vestibulum ante ipsum primis in",
                "faucibus orci luctus et ultrices"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "Lorem ipsum dolor sit amet,",
                "consectetuer adipiscing elit.",
                "Aenean fermentum ullamcorper.",
                "Vestibulum ante ipsum primis in",
                "faucibus orci luctus et ultrices",
                "posuere cubilia Curae; Morbi"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "Lorem ipsum dolor sit amet,",
                "consectetuer adipiscing elit.",
                "Aenean fermentum ullamcorper.",
                "Vestibulum ante ipsum primis in",
                "faucibus orci luctus et ultrices",
                "posuere cubilia Curae; Morbi",
                "massa, fermentum vitae..."
            ])?;
            ctx.next()?;
            ctx.items().take(710, 1)?;
            ctx.lines_as(
                "Darwin",
                args![
                    "^666666*Gasp...*^000000",
                    "You are now",
                    "an Alchemist!!",
                    "Go to the Union",
                    "and cast away the last",
                    "vestiges of Merchant life!!"
                ],
            )?;
            ctx.var("alch_q").set(Val::from(40))?;
            ctx.quests().change(2033, 2034)?;
            return ctx.close();
        } else if ctx.items().count(7134)? > 2
            && ctx.items().count(713)? > 2
            && ctx.items().count(507)? > 0
            && ctx.items().count(508)? > 0
            && ctx.items().count(509)? > 0
        {
            ctx.lines_as(
                "Darwin",
                args!["Seems like you have everything ready. As promised, I will teach you how to make simple medicine."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args!["First, prepare the Medicine Bowl. Then, you put the Herbs inside, like this, and slowly crush them."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Pour small amounts",
                    "of clean water and stir",
                    "the mixture until it thickens.",
                    "Afterwards, add some more Herbs."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args!["That's how you make it. If you think you have enough, gently pour the mixture into an empty bottle."],
            )?;
            ctx.items().take(7134, 3)?;
            ctx.items().take(713, 3)?;
            ctx.items().take(507, 1)?;
            ctx.items().take(508, 1)?;
            ctx.items().take(509, 1)?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "There you go,",
                    "it's complete.",
                    "Now, make some medicine",
                    "using the simple procedure",
                    "I just explained to you."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&[
                "Prepare the Medicine Bowl.",
                "Put the Medicine Bowl on your head.",
                "Kick the Medicine Bowl.",
            ])? {
                0 => {}
                1 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["What...?"])?;
                    ctx.next()?;
                }
                2 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["No!"])?;
                    ctx.next()?;
                }
                _ => {}
            }
            match ctx.menu(&[
                "Put some dirt in the Medicine Bowl.",
                "Put some Herbs in the Medicine Bowl.",
                "Put a Harp in the Medicine Bowl.",
            ])? {
                0 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["...Eh!?", "That's not", "medicine!"])?;
                    ctx.next()?;
                }
                1 => {}
                2 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["A Harp?", "And how would", "you do that?"])?;
                    ctx.next()?;
                }
                _ => {}
            }
            match ctx.menu(&["Crush the Herbs.", "Crush the Medicine Bowl.", "Crush Darwin's foot."])? {
                0 => {}
                1 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["Wh-What are", "you doing!?"])?;
                    ctx.next()?;
                }
                2 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["Agh...!", "What do you", "think you're", "doing?!"])?;
                    ctx.next()?;
                }
                _ => {}
            }
            match ctx.menu(&["Spray clean water.", "Drink clean water.", "Pour clean water."])? {
                0 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["Huh?", "What are you doing?"])?;
                    ctx.next()?;
                }
                1 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["W-wait...", "Are you", "taking a break?"])?;
                    ctx.next()?;
                }
                2 => {}
                _ => {}
            }
            match ctx.menu(&[
                "Continue crushing the Herbs.",
                "Continue eating the Herbs.",
                "Continue dancing and singing.",
            ])? {
                0 => {}
                1 => {
                    l_w_point += 1;
                    ctx.lines_as(
                        "Darwin",
                        args!["Eat the Herbs?", "I think you need", "to focus on the", "task at hand..."],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    l_w_point += 1;
                    ctx.lines_as(
                        "Darwin",
                        args!["Singing and", "dancing? Alchemists", "don't do that, have", "you gone crazy?"],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            match ctx.menu(&[
                "Put noodles in and fry it.",
                "Pour it in an empty bottle.",
                "Hold the Medicine Bowl and drink it.",
            ])? {
                0 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["We're Alchemists,", "not restaurant chefs."])?;
                    ctx.next()?;
                }
                1 => {}
                2 => {
                    l_w_point += 1;
                    ctx.lines_as("Darwin", args!["Huh...", "Pretty sloppy..."])?;
                    ctx.next()?;
                }
                _ => {}
            }
            if l_w_point > 0 {
                ctx.lines_as("Darwin", args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Darwin",
                    args![
                        "You messed up the mixture",
                        "since you didn't follow the procedure! Get some more ingredients so you can try it",
                        "again until you get it right."
                    ],
                )?;
                return ctx.close();
            }
            ctx.items().give(501, 1)?;
            ctx.items().give(503, 1)?;
            ctx.items().give(504, 1)?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Good job.",
                    "It came out pretty well considering it's your first time. Go ahead and keep the medicines that you've just made."
                ],
            )?;
            ctx.var("alch_q").set(Val::from(8))?;
            ctx.quests().change(2033, 2035)?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "Now, go into the next room",
                    "and speak to Van Helmont to",
                    "continue your training."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args!["Never forget...", "You must always protect", "what is most precious to you."],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Darwin",
                args![
                    "Have you forgotten",
                    "what you need to bring?",
                    "Let me remind you once",
                    "again. You must come",
                    "back with..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Darwin",
                args![
                    "^551A8B3 Medicine Bowls^000000,",
                    "^551A8B3 Empty Bottle^000000,",
                    "^551A8B1 Red Herb^000000,",
                    "^551A8B1 Yellow Herb^000000 and",
                    "^551A8B1 White Herb^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Darwin", args!["Come back", "when you are", "ready..."])?;
            return ctx.close();
        }
    } else if ctx.var("alch_q").get()? == 8 {
        ctx.lines(args![
            "I said to go",
            "to Van Helmont.",
            "I'd like to teach you",
            "more, but I can't."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "Aah...",
                "Harmona, my love.",
                "I can't even see the flower anymore. My soul quietly",
                "withers as well.."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 40 {
        ctx.lines(args![
            "I have already given you all of my knowledge and have nothing more",
            "to teach you."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "Go to the second floor and talk to speak to our Union Leader. Once",
                "you do that, your life as an Alchemist will begin."
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines(args![
            "When you have",
            "your dreams, you",
            "have everything.",
            "Without them, you have",
            "nothing more to lose."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "These cursed eyes...",
                "They've lost sight of",
                "my dreams a long time ago.",
                "Ha ha ha ha..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Darwin",
            args![
                "Does paradise",
                "really exist...?",
                "Not without my love...",
                "Not without Harmona..."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn experiment_expert_am(ctx: &Ctx) -> Script {
    let mut l_w_point = 0;
    ctx.mes("[Van Helmont]")?;
    if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT {
        if ctx.var("BaseJob").get()? == constants::JOB_ALCHEMIST {
            ctx.lines(args![
                "What do you want?",
                "I'm busy!! Don't",
                "bother me and get",
                "on your way."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Now, come on...",
                    "You'll never get",
                    "any research completed if you just slack off. Go out and learn all that you can."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Reading science journals and performing experiments. That's what Alchemy is all about. Now, let me get back to work!"
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines(args![
                "Just a little...",
                "A little bit more...",
                "Nooo! Just a little",
                "bit more and it",
                "would've been done!"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Why...?!",
                    "Why, another failure?!",
                    "My calculations were",
                    "all correct! W-Wait...!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args!["Perhaps, if I capacitated the thermal flux by using the neutronic gradient, it just might work...!"],
            )?;
            return ctx.close();
        }
    }
    if ctx.var("alch_q").get()? == 8 {
        ctx.lines(args!["Arrrrgh...!", "Why isn't this formula working? What's wrong? In theory, it's all correct, but there must be an error in the formula somewhere..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args![
                "I pour it in here, and it should stop. Wait, this is the wrong solution! How could I make such",
                "a dumb mistake?! When did these",
                "get switched?!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args![
                "Okay, okay...",
                "I just need to fix this part.",
                "No need to start over. I just",
                "need to fix it... But wait. Wait..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["................."])?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["Um...", "Who are you?"])?;
        ctx.next()?;
        if ctx.menu(&["I want to become an Alchemist.", "......."])? == 1 {
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Hmm...?",
                    "What, did you just want to watch? Fine, fine, but do it quietly and leave right when you're done."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Van Helmont", args!["Stay out of my way and don't go around touching stuff. There might be some volatile materials, so it'd be dangerous to have any accidents."])?;
            return ctx.close();
        }
        ctx.lines_as("Van Helmont", args!["You...?", "An Alchemist?", "What a funny Merchant."])?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args![
                "Well, that's nice, but I have very urgent experiments that require",
                "my attention, so don't get",
                "in the way."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Teach me something.", "..."])? == 0 {
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Argh...!",
                    "Didn't I just tell you not to bother me? What's so hard to understand about that?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Fine, fine. I'll give you an assignment. Learn something",
                    "new and come back. Let's see.",
                    "What would be good..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Okay, I got it.",
                    "Go learn how to make",
                    "a Counteragent and Mixture",
                    "from Molgenstein."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Van Helmont", args!["You don't need to bring anything. Just go watch him at work and have him tell you how he makes those solutions. Got it?"])?;
            ctx.next()?;
            ctx.mes("[Van Helmont]")?;
            ctx.var("alch_q").set(Val::from(9))?;
            ctx.quests().change(2035, 2036)?;
            ctx.lines(args![
                "Well then, see you later.",
                "You'd better get going as",
                "soon as you can."
            ])?;
            return ctx.close();
        }
        ctx.lines_as("Van Helmont", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["...", "......"])?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["So if I recalibrate the combustion rate of this compound, that should negate any cohesive tendencies in this particle flux..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args![
                "But what am I going to do",
                "about all of this spontaneous",
                "crystallization?! I can't very well remove this matrix, I need it for the catalyst to reach the triple point."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["Damn!", "What am", "I going to do?!"])?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 9 {
        ctx.mes("Alright, if I make an incision here in the Tentacle, and add a Jellopy and Sticky Mucus solution into the... Where the hell did my Medicine Bowl go?")?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args!["Did I use them all at a time like this?! I wonder if Nicholas has any left. Ugh, what a pain. Wait. Wait a minute..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Van Helmont", args!["...", "......", "Who are you?"])?;
        ctx.next()?;
        if ctx.menu(&["I want to become an Alchemist?", "......."])? == 1 {
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Hmm...?",
                    "What, did you just want to watch? Fine, fine, but do it quietly and leave right when you're done."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Van Helmont", args!["Stay out of my way and don't go around touching stuff. There might be some volatile materials, so it'd be dangerous to have any accidents."])?;
            return ctx.close();
        }
        ctx.lines_as("Van Helmont", args!["Ah, of course. The Merchant from before. So what did you learn from Molgenstein? I didn't just send you there for fun, you know."])?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args!["Let me ask you", "some questions to", "check what you've", "learned."],
        )?;
        ctx.next()?;
        if ctx.var("misc_quest").get()?.number()? & 4 != 0 {
            ctx.lines_as(
                "Van Helmont",
                args!["Which item is not", "necessary to make", "a Counteragent?"],
            )?;
            ctx.next()?;
            if ctx.menu(&["Karvodailnirol", "Detrimindexta", "Alcohol"])? != 0 {
                l_w_point += 1;
            }
            ctx.lines_as("Van Helmont", args!["What item is not", "necessary to make", "a Mixture?"])?;
            ctx.next()?;
            if ctx.menu(&["Karvodailnirol", "Detrimindexta", "Alcohol"])? != 1 {
                l_w_point += 1;
            }
            if l_w_point > 0 {
                ctx.lines_as(
                    "Van Helmont",
                    args![
                        "Weren't you listening to Molgenstein at all? Maybe you",
                        "have to watch him make it again."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Van Helmont",
                    args![
                        "If you can't tell the exact items that you need in an experiment,",
                        "you might end up hurting yourself!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Good, you've learned well.",
                    "Okay, now you know something about experimentation. You're done here, so now I can continue with my experiments."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "Go out and find the room next",
                    "to this one and talk to Nicholas. He'll continue your training."
                ],
            )?;
            ctx.next()?;
            ctx.var("alch_q").set(Val::from(20))?;
            ctx.quests().change(2036, 2037)?;
            ctx.lines_as(
                "Van Helmont",
                args![
                    "What are you",
                    "still doing here?",
                    "Go! We both have",
                    "more important",
                    "things to do!"
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as("Van Helmont", args!["What item do", "you need to make", "a Counteragent?"])?;
            ctx.next()?;
            ctx.var("@menu")
                .set(runtime::select_values(ctx, &[Val::from("Feather:Sticky Mucus:Animal Gore")])?)?;
            ctx.lines_as("Van Helmont", args!["What item do", "you need to make", "a Mixture?"])?;
            ctx.next()?;
            ctx.var("@menu").set(runtime::select_values(
                ctx,
                &[Val::from("Monster Feed:Ancient Lips:Rotten Bandage")],
            )?)?;
            ctx.lines_as(
                "Van Helmont",
                args!["Be honest. You don't know, do you?! Didn't I say to go to Molgenstein and have him teach you?!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Van Helmont",
                args!["Don't even think about coming back until you talk to him! Now stop bothering me and get out of here!"],
            )?;
            return ctx.close();
        }
    } else if ctx.var("alch_q").get()? == 20 {
        ctx.lines(args!["What...?", "I thought I told you to", "talk to Nicholas next door?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args![
                "I need to continue my research,",
                "and you need to finish becoming an Alchemist. Come on, get moving!"
            ],
        )?;
        return ctx.close();
    } else {
        ctx.lines(args![
            "Just a little...",
            "A little bit more...",
            "Nooo! Just a little",
            "bit more and it",
            "would've been done!"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args!["Why...?!", "Why, another failure?!", "My calculations were", "all correct! Wait..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Van Helmont",
            args!["Perhaps, if I capacitated the thermal flux by using the neutronic gradient, it just might work..."],
        )?;
        return ctx.close();
    }
}

pub fn master_alchemist_am(ctx: &Ctx) -> Script {
    ctx.fx().cutin("job_alche_vincent", 2)?;
    ctx.mes("[Vincent Carsciallo]")?;
    if ctx.var("Upper").get()? == 1 {
        ctx.lines(args!["You have transcended...", "Excellent, excellent."])?;
        ctx.next()?;
        ctx.lines_as(
            "Vincent Carsciallo",
            args!["You don't belong here.", "Go and explore the wide world, my friend."],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    }
    if ctx.var("BaseJob").get()? != constants::JOB_MERCHANT {
        if ctx.var("BaseJob").get()? == constants::JOB_ALCHEMIST {
            ctx.lines(args!["Welcome!", "So how is your", "research coming along?"])?;
            ctx.next()?;
            ctx.lines_as("Vincent Carsciallo", args!["At times you get results that are unexpected from an experiment. Although these may be setbacks in your research, such results can also lead to new discoveries."])?;
            ctx.next()?;
            ctx.lines_as(
                "Vincent Carsciallo",
                args![
                    "If you discover something new,",
                    "come and tell us. Don't forget that we are all working together to unlock the mysteries of science!"
                ],
            )?;
        } else if ctx.var("BaseClass").get()? == constants::JOB_NOVICE {
            ctx.lines(args![
                "Hm...",
                "A Novice?",
                "You shouldn't be",
                "playing in a place",
                "like this."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Vincent Carsciallo",
                args![
                    "There are a lot of volatile chemicals and dangerous",
                    "materials in this building. It'd be a lot better if you just played outside."
                ],
            )?;
        } else {
            ctx.lines(args![
                "Hmm...?",
                "What's an adventurer",
                "doing here in the",
                "Alchemist Union?"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Vincent Carsciallo",
                args![
                    "I'm afraid there's",
                    "not much we can offer",
                    "you here if you're not",
                    "a member of our Union."
                ],
            )?;
        }
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    }
    if ctx.var("alch_q").get()? == 0 {
        ctx.lines(args!["Hmm...?", "A Merchant?", "Are you interested", "in learning Alchemy?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Vincent Carsciallo",
            args![
                "This is the Alchemist Union.",
                "We research and experiment with many different substances in order to create new materials without using magic."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Vincent Carsciallo",
            args![
                "Someday, we hope to unlock",
                "the secret of life, as well as the other mysteries of science."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Vincent Carsciallo", args!["After being traveling as a Merchant for a long time, you must have developed some scientific curiosity. If you'd like to learn Alchemy, why don't you try joining the Alchemist Union?"])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    } else if ctx.var("alch_q").get()? == 40 {
        if ctx.var("JobLevel").get()?.number()? < 40 {
            ctx.var("alch_q").set(Val::from(0))?;
            ctx.lines(args![
                "Hmm...you don't seem to be qualified yet.",
                "Remember, you must reach at least job level 40 to become an Alchemist."
            ])?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines(args![
                "Ah, you're almost",
                "ready to become an",
                "Alchemist, but you must",
                "first allocate your unused",
                "Skill Points."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Vincent Carsciallo",
                args!["Talk to me again", "once you have spent", "all of your extra", "Skill Points."],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
        if ctx.call(Function::CheckQuest, args![2039])? != -1 {
            ctx.quests().change(2039, 2040)?;
        }
        if ctx.call(Function::CheckQuest, args![2034])? != -1 {
            ctx.quests().change(2034, 2040)?;
        }
        ctx.lines(args![
            "Ah, well done.",
            "I can see that you",
            "have learned all of",
            "the basics of Alchemy."
        ])?;
        ctx.next()?;
        ctx.var("alch_q").set(Val::from(0))?;
        ctx.quests().complete(2040)?;
        let l_jlevel = ctx.player().job_level()?;
        shared::other_global_functions::job_change(ctx, args![constants::JOB_ALCHEMIST])?;
        shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
        ctx.lines_as(
            "Vincent Carsciallo",
            args![
                "Henceforth, you are",
                "now a member of our",
                "illustrious Union.",
                "I hope you learn a lot..."
            ],
        )?;
        ctx.next()?;
        if l_jlevel == 50 {
            ctx.items().give(7133, 1)?;
            ctx.lines_as(
                "Vincent Carsciallo",
                args![
                    "Let me give you",
                    "something special.",
                    "You can use this to",
                    "begin your life",
                    "of research."
                ],
            )?;
        } else {
            match ctx.rand_range(1, 6)? {
                1 => {
                    ctx.items().give(7127, 1)?;
                }
                2 => {
                    ctx.items().give(7128, 1)?;
                }
                3 => {
                    ctx.items().give(7129, 1)?;
                }
                4 => {
                    ctx.items().give(7130, 1)?;
                }
                5 => {
                    ctx.items().give(7131, 1)?;
                }
                6 => {
                    ctx.items().give(7144, 1)?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Vincent Carsciallo",
                args!["And...", "Here's a little", "something to help", "you begin your", "research."],
            )?;
        }
        ctx.next()?;
        ctx.lines_as(
            "Vincent Carsciallo",
            args![
                "I'll see",
                "you later then...",
                "Remember to carry",
                "yourself with pride",
                "as an Alchemist!"
            ],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    } else {
        ctx.lines(args![
            "Ah...",
            "I believe you've",
            "already registered",
            "for training to become",
            "an Alchemist."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Vincent Carsciallo",
            args![
                "Please listen to the",
                "other Alchemists and follow their instructions carefully. You will learn much from them."
            ],
        )?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return ctx.end();
    }
}

pub fn chief_researcher_am(ctx: &Ctx) -> Script {
    let mut l_alch_t = 0;
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you are carrying -",
            "- too many items with you. -",
            "- Please come back again -",
            "- after you store some items into kafra storage. -"
        ])?;
        return ctx.close();
    }
    if ctx.var("alch_q").get()?.number()? > 19 && ctx.var("alch_q").get()?.number()? < 22 {
        if ctx.var("alch_q").get()? == 20 {
            ctx.lines_as(
                "Nicholas Flamel",
                args!["Ooh...", "You're the upstart", "Merchant that wants", "to become an Alchemist?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Nicholas Flamel", args!["Not just anyone can become an Alchemist, you know. You've got to have motivation and clear goals and a strong sense of focus."])?;
            ctx.next()?;
            ctx.lines_as("Nicholas Flamel", args!["Alchemists must memorize many chemical equations, scientific laws and a lot of other information. It's actually pretty tough."])?;
            ctx.next()?;
            ctx.lines_as("Nicholas Flamel", args!["If you can't focus, you'll be confused later when you look at Alchemy charts. My test will judge your ability to do just that."])?;
            ctx.next()?;
        }
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Find the words scrambled",
                "in the group of letters I give you. They can be made by using some",
                "or all of the letters."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nicholas Flamel",
            args!["You pass if you", "choose the word", "that is ^551A8BIN^000000 the puzzle."],
        )?;
        ctx.next()?;
        match ctx.rand_range(1, 3)? {
            1 => {
                ctx.mes("t m y a n y e o b n e g p r i")?;
                ctx.next()?;
                if ctx.menu(&["Brake", "Brass", "Bug", "Broken", "Brigan?"])? == 4 {
                    l_alch_t += 10;
                }
                ctx.mes("o n c u t a p j l e r s v m u")?;
                ctx.next()?;
                if ctx.menu(&["vendor", "storage", "weapon", "simple", "streetshop"])? == 0 {
                    l_alch_t += 10;
                }
                ctx.mes("t v a r m e g p h e u b o y l")?;
                ctx.next()?;
                if ctx.menu(&["molasses", "party", "leader", "sweets", "treacle"])? == 1 {
                    l_alch_t += 10;
                }
                ctx.mes("q z a h n a i n b r d p t n c")?;
                ctx.next()?;
                if ctx.menu(&["partisan", "partizan", "pato", "paros", "pack"])? == 1 {
                    l_alch_t += 10;
                }
            }
            2 => {
                ctx.mes("m p d i c f a r o g n k w a s")?;
                ctx.next()?;
                if ctx.menu(&["packman", "sunshine", "ragnarok", "wonderland", "frost"])? == 0 {
                    l_alch_t += 10;
                }
                ctx.mes("g b n o p r e f a r e t a s k")?;
                ctx.next()?;
                if ctx.menu(&["purple", "smoker", "ragnarok", "bolt", "burnt wood"])? == 2 {
                    l_alch_t += 10;
                }
                ctx.mes("u g n i s j e k c e o g n d p")?;
                ctx.next()?;
                if ctx.menu(&["scab", "kinship", "donate", "source", "opening"])? == 4 {
                    l_alch_t += 10;
                }
                ctx.mes("r o e h n r o m c a i n p t t")?;
                ctx.next()?;
                if ctx.menu(&["forgemerchant", "potionmerchant", "dcmerchant", "vendingmerchant", "battlemerchant"])? == 1 {
                    l_alch_t += 10;
                }
            }
            3 => {
                ctx.mes("s m i e x b w u n e t a g l r")?;
                ctx.next()?;
                if ctx.menu(&["tiger", "wolf", "pumpkin", "tripped", "tore"])? == 0 {
                    l_alch_t += 10;
                }
                ctx.mes("n i e g b o p d s o a u w r v")?;
                ctx.next()?;
                if ctx.menu(&["bash", "provoke", "endure", "stun", "abracadabra"])? == 2 {
                    l_alch_t += 10;
                }
                ctx.mes("l r m g r e x t a v i n e d e")?;
                ctx.next()?;
                if ctx.menu(&["alberta", "latifoliate", "crimson", "maple", "evergreen"])? == 4 {
                    l_alch_t += 10;
                }
                ctx.mes("r o e h n r o m c a i n p t t")?;
                ctx.next()?;
                if ctx.menu(&["forgemerchant", "potionmerchant", "dcmerchant", "vendingmerchant", "battlemerchant"])? == 1 {
                    l_alch_t += 10;
                }
            }
            _ => {}
        }
        ctx.lines_as("Nicholas Flamel", args!["Ah, you finished.", "Now, let's see..."])?;
        if l_alch_t > 30 {
            ctx.var("alch_q").set(Val::from(22))?;
            ctx.mes("Excellent job!")?;
            ctx.next()?;
            ctx.lines_as("Nicholas Flamel", args!["Great, you found all of those hidden words. With that kind of concentration, you should have no problem memorizing information."])?;
            ctx.next()?;
            ctx.lines_as(
                "Nicholas Flamel",
                args![
                    "Come back in a little bit while",
                    "I prepare the next assignment",
                    "for your training."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nicholas Flamel",
                args![
                    "Oh, and before you talk to",
                    "me again, make sure you have",
                    "^551A8Bplenty of room in your inventory^000000."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.var("alch_q").set(Val::from(21))?;
            ctx.mes("^666666*Gasp!*^000000 H-horrible!")?;
            ctx.next()?;
            ctx.lines_as(
                "Nicholas Flamel",
                args!["Judging from these results, you obviously have a problem with concentrating."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nicholas Flamel",
                args!["If you can't even solve these easy word puzzles, how can you keep track of your experiments and research?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nicholas Flamel",
                args!["Why don't you relax", "and rest a bit before", "you take the test again?"],
            )?;
            return ctx.close();
        }
    } else if ctx.var("alch_q").get()? == 22 {
        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 1370 {
            ctx.lines_as(
                "Nicholas Flamel",
                args![
                    "Whoa...",
                    "You're carrying too much stuff! First, put some of your things in Kafra Storage."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Alright...",
                "For your next",
                "assignment, you'll",
                "need to travel to ^551A8BJuno^000000."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nicholas Flamel", args!["There, you'll need to talk to ^551A8BBain^000000 and ^551A8BBajin^000000. Those two are doing Alchemy research with the Sages", "in Juno. You'll learn something by assisting them with their project."])?;
        ctx.next()?;
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Come back here to me after you",
                "help them out. They'll need all of these items to continue their experiments."
            ],
        )?;
        ctx.next()?;
        ctx.var("alch_q").set(Val::from(23))?;
        ctx.quests().change(2037, 2038)?;
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "1 Mixture,",
                "5 Burnt Tree,",
                "5 Fine Sand,",
                "3 Rough Oridecon",
                "and 3 Rough Elunium."
            ],
        )?;
        ctx.items().give(974, 1)?;
        ctx.items().give(7068, 5)?;
        ctx.items().give(7043, 5)?;
        ctx.items().give(756, 3)?;
        ctx.items().give(757, 3)?;
        ctx.next()?;
        ctx.lines_as(
            "Nicholas Flamel",
            args!["Alright.", "Have a safe trip", "and come back in", "one piece."],
        )?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 23 {
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Didn't I say to",
                "go to Juno and help",
                "Bain and Bajin with",
                "their Alchemy research?"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 24 {
        ctx.var("alch_q").set(Val::from(40))?;
        ctx.quests().change(2038, 2039)?;
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Ah, you're back!",
                "I just got a message from Bain",
                "and Bajin. They let me know that they were very happy with your assistance."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "If you were good enough",
                "to help out those brothers,",
                "you definitely qualify to be",
                "an Alchemist."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nicholas Flamel", args!["Good work!", "All you have to do now is speak to the Union Leader on the 2nd floor! Congratulations, you'll become an Alchemist very soon!"])?;
        return ctx.close();
    } else if ctx.var("alch_q").get()? == 40 && ctx.var("BaseJob").get()? == constants::JOB_MERCHANT {
        ctx.lines_as("Nicholas Flamel", args!["All you have to do now is speak to the Union Leader on the 2nd floor! Congratulations, you'll become an Alchemist very soon!"])?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Lorem ipsum dolor sit amet,",
                "consectetuer adipiscing elit.",
                "Vivamus sem. Sed metus",
                "lacus, viverra id, rutrum eget,",
                "rhoncus sit amet, lectus."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nicholas Flamel",
            args![
                "Suspendisse sit amet urna in",
                "nisl fringilla faucibus. Nulla scelerisque eros...",
                "^666666*Mumble Mumble*^000000"
            ],
        )?;
        return ctx.close();
    }
}
