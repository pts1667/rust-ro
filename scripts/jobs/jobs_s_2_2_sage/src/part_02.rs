use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn history_professor_sa_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_point = Val::from(0);
    ctx.mes("[Saphien Layless]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
            ctx.lines(args![
                "Do I know you? Were you one of my students?",
                "Oh, it doesn't matter anyway. You wouldn't be a Sage without graduating from this academy..."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "In any case, you must study the past in order to better understand the present and to predict...the future.",
                    "This sentence contains all the truth of the world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "I guess you're treading the right path...",
                    "One of these days you'll look back to this moment and realize it changed your life."
                ],
            )?;
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.mes("A Novice? Why is a novice here?")?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args!["One whose life has many possibilities... ", "How do you wish to lead your life?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "No matter what you decide to be, everything depends on your choice.",
                    "May God bless you so that you can choose the path best for you..."
                ],
            )?;
        } else {
            ctx.mes("Welcome, I am in charge of historical studies here in the academy.")?;
            ctx.next()?;
            ctx.lines_as("Saphien Layless", args!["The world as we know it is a result of events that described in the records of the ages. It is historical events that have shaped the world as it is today.", "Therefore, knowing the past means you will better understand the present and...the future."])?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args!["Reflect upon your past.", "You will see your path in the future..."],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("sage_q").get()? == 9 {
        if ctx.var("sage_q2").get()? == 0 {
            ctx.lines(args![
                ((Val::from("Welcome, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(". I am glad to meet you.")),
                "My name is Saphien Layless, I will be in charge of your class for a while."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The subject you're studying is Yggdrasil.",
                    "So...do you even know what Yggdrasil is?"
                ],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes, I know very well.:No, I don't.")])? {
                1 => {
                    ctx.lines_as("Saphien Layless", args!["Okay then, what is Yggdrasil?", "Please answer me."])?;
                    ctx.next()?;
                    match runtime::select_values(
                        ctx,
                        &[Val::from(
                            "It's a name of a health item.:It's the source of life in the world.:Suckah, I lied.",
                        )],
                    )? {
                        1 => {
                            ctx.lines_as(
                                "Saphien Layless",
                                args![
                                    ((Val::from("Wrong. ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", you got - 10 points.")),
                                    "That's just one of the gifts from Yggdrasil."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Saphien Layless",
                                args!["Yggdrasil is the name of the tree that is the source of life in this world."],
                            )?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Saphien Layless",
                                args![
                                    "That's right. Yggdrasil, the so-called 'World Tree', ",
                                    "is the name of the tree that has been the source of life in this world."
                                ],
                            )?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Saphien Layless",
                                args![
                                    "Great Schott...If you don't know, just say so.",
                                    ((Val::from("By the way, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                        + Val::from(", you just got - 10 points for lying and for being wrong."))
                                ],
                            )?;
                        }
                        _ => {}
                    }
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Saphien Layless",
                        args![
                            "Ah well, I expected you would know at least a little bit about Yggdrasil...",
                            "Yggdrasil is the name of the tree that is the source of life in this world."
                        ],
                    )?;
                    ctx.next()?;
                }
                _ => {}
            }
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Before we start the class, I'll need some reserve items.",
                    "It's for better understanding of Yggdrasil."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Anything is fine as long as it's related to the Yggdrasil tree.",
                    "Seeds or fruits of the tree would be good. I know it's difficult to find, but please try."
                ],
            )?;
            ctx.next()?;
            ctx.var("sage_q2").set(Val::from(1))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2047), Val::from(2050)])?;
            ctx.lines_as(
                "Saphien Layless",
                args!["When we have the reserve items, we will start the class.", "Please come back."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.call(Function::CountItem, vec![Val::from(607)])?.number()? > 0 {
                ctx.lines_as(
                    "Saphien Layless",
                    args!["Oh, did you bring them with you? Excellent!", "You brought Yggdrasilberry!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Okay, now I am starting the class.",
                        "Listen carefully, I will not accept dozing off in the middle of class."
                    ],
                )?;
                ctx.next()?;
            } else if ctx.call(Function::CountItem, vec![Val::from(608)])?.number()? > 0 {
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Hmm, did you prepare for class? Let's see...",
                        "Oh! So you brought me the Yggdrasil Seed?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Very well. Now I am starting the class..",
                        "Listen carefully, I will not accept dozing off in the middle of class."
                    ],
                )?;
                ctx.next()?;
            } else if ctx.call(Function::CountItem, vec![Val::from(610)])?.number()? > 0 {
                ctx.lines_as(
                    "Saphien Layless",
                    args!["Hmm, did you prepare for class? Let's see...", "Yggdrasil Leaf..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "You can purchase this item from the town of Al De Baran! You didn't show any effort.",
                        ((Val::from("So, I must give you - 10 points, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Okay, now I am starting the class.",
                        "Listen carefully, I will not accept you dozing off in the middle of class, or any more slacking."
                    ],
                )?;
                ctx.next()?;
            } else {
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Huh? You're not ready to this class yet!",
                        "I told you to bring me any items related Yggdrasil."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Tell me when you're ready.",
                        "Don't worry about being late, we professors are paid to wait around."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The root of this world, Yggdrasil, is a gigantic tree.",
                    "It takes root all over the continent of Rune-Midgarts and its leaves reach the sky."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Outside of this continent, there is a ocean where a giant snake named Yormungandr is coiled up.",
                    "The world as we know it consists of 3 places: Asgard, Midgard and Utgard."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Saphien Layless", args!["Utgard is where all the titans live."])?;
            ctx.next()?;
            ctx.lines_as("Saphien Layless", args!["Midgard is where all human beings live."])?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "And Asgard is where the gods reside...",
                    "An ash tree taking root in the middle of the Rune-Midgarts continent, that is Yggdrasil."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Saphien Layless", args!["This continent was born from the heart of Ymir, and Yggdrasil holds the continent together by grasping it with its 3 roots.", "These roots stretch into 3 different places."])?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The first root reaches Asgard where the gods live and ",
                    "where we, mere humans, haven't yet explored or experienced."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The second root reaches Jotunnheim where all the giants live.",
                    "We've been told this name many times through myths and legends."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The third one reaches Niflheim.",
                    "That place is rumored to be covered with a thick, black fog."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Items, such as Seed of Yggdrasil, Yggdrasilbrry and Leaf of Yggdrasil are",
                    "considered a part of Yggdrasil tree."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Yggdrasilberry has a fascinating scent that is rumored to",
                    "restore full HP and SP at once."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The seed of Yggdrasil which has the fragrance of a blooming flower and the flavor of nut is rumored to",
                    "restore half of HP and SP at once."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "The leaf of Yggdrasil which is filled with vital force is rumored to",
                    "revive the dead, bringing them back to this world."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Lastly... if by some chance you discover a way into Asgard in the future, ",
                    "I hope you will find the Yggdrasil tree."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "Even though the story of the Yggdrasil tree only exists in myths and legends, ",
                    "we Sages are obligated to discover the truth of the Yggdrasil tree."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "That is the end of today's class.",
                    "Please record the articles you've learned today, and try to remember as much as you can."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "In the next class, you will write a thesis about Yggdrasil...",
                    "Please bring the following items to prepare for class."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "^3355FF1 Feather of Birds^000000 which will be used as a pen,",
                    "^3355FF1 Animal Skin^000000 which will be used as paper,",
                    "^3355FF1 Trunk^000000 which will be used to bind a book,",
                    "^3355FF1 Squid Ink^000000 which will be used as ink,",
                    "^3355FF1 Empty Bottle^000000 which will be used for holding the squid ink."
                ],
            )?;
            ctx.next()?;
            ctx.var("sage_q2").set(Val::from(0))?;
            ctx.var("sage_q").set(Val::from(10))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2050), Val::from(2051)])?;
            ctx.lines_as(
                "Saphien Layless",
                args![
                    "I will help you to write your thesis when you're ready with all those items.",
                    "I am looking forward to the next class with you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sage_q").get()? == 10 {
            if ((((ctx.call(Function::CountItem, vec![Val::from(916)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(919)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(1019)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(1024)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0)
            {
                ctx.call(Function::DelItem, vec![Val::from(916), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(919), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(1019), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(1024), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
                ctx.lines(args![
                    "Now, you're writing your thesis.",
                    "I will assist you with your writing."
                ])?;
                ctx.next()?;
                ctx.mes("..........")?;
                ctx.next()?;
                ctx.mes("....................")?;
                ctx.next()?;
                ctx.mes(".................................")?;
                ctx.next()?;
                ctx.lines(args![
                    ".....There is a ocean around the continent,",
                    "The ocean is coiled up by"
                ])?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "A giant ash tree.:A giant snake Yormungandr.:A giant turtle and elephants.:A giant dragon.",
                    )],
                )? {
                    1 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("A giant ash tree.")?;
                    }
                    2 => {
                        ctx.mes("A giant snake Yormungandr.")?;
                    }
                    3 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("A giant turtle and elephants.")?;
                    }
                    4 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("A giant dragon.")?;
                    }
                    _ => {}
                }
                ctx.mes("The continent consists of three places such as,")?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Mt.Mjolnir, where spiders live,:Uranos, where titans live,:Utgard, where titans live,:Lutie, the winter land,",
                    )],
                )? {
                    1 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Mt.Mjolnir where spiders live,")?;
                    }
                    2 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Uranos where titans live,")?;
                    }
                    3 => {
                        ctx.mes("Utgard where titans live,")?;
                    }
                    4 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Lutie, the winter land,")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Midgard, where humans live,:Rune-Midgarts where humans live,:Tritonia, where mermaids live,:Morocc, the desert city,",
                    )],
                )? {
                    1 => {
                        ctx.mes("Midgard, where humans live in,")?;
                    }
                    2 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Rune-Midgarts, where humans live,")?;
                    }
                    3 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Tritonia, where mermaids live,")?;
                    }
                    4 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Morocc, the desert city,")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Abguard, where gods live.:Asgard, where gods live.:Schwarzwald, where citizens live.:Prontera, the capital of Rune-Midgarts.",
                    )],
                )? {
                    1 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Abguard where gods live.")?;
                    }
                    2 => {
                        ctx.mes("Asgard, where gods live.")?;
                    }
                    3 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Schwarzwald, where citizens live.")?;
                    }
                    4 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Prontera, the capital of Rune-Midgarts.")?;
                    }
                    _ => {}
                }
                ctx.mes("The continent consists of the three places stated above.")?;
                ctx.next()?;
                if l_w_point.clone().number()? > 0 {
                    ctx.lines_as(
                        "Saphien Layless",
                        args![
                            ((Val::from("...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", have you lost your mind?")),
                            "Your work is too poor to be considered as a thesis!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Saphien Layless",
                        args![
                            "I don't think you can submit your work to the dean.",
                            "Go study harder and try again!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines(args![".....as we studied from the last class,", "Yggdrasil is..."])?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "A giant ash tree.:A fabulous Mastella tree.:A giant willow.:A giant dead branch.",
                    )],
                )? {
                    1 => {
                        ctx.mes("A giant ash tree.")?;
                    }
                    2 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("A fabulous Mastella tree.")?;
                    }
                    3 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("A giant willow.")?;
                    }
                    4 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("A giant dead branch.")?;
                    }
                    _ => {}
                }
                ctx.mes("The root of Yggdrasil is divided into 3 parts. Those parts reach to 3 places...")?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Schwarzwald, Jotunnheim, Niflheim.:Midgard, Jotunnheim, Naffleheim.:Asgard, Jotunnheim, Naffleheim.:Asgard, Jotunnheim, Niflheim.",
                    )],
                )? {
                    1 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Schwarzwald, Jotunnheim, Niflheim.")?;
                    }
                    2 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Midgard, Jotunnheim, Naffleheim.")?;
                    }
                    3 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("Asgard, Jotunnheim, Naffleheim.")?;
                    }
                    4 => {
                        ctx.mes("Asgard, Jotunnheim, Niflheim.")?;
                    }
                    _ => {}
                }
                ctx.mes("One who has a Seed of Yggdrasil,")?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "can be cured from all the abnormal statuses.:can restore full HP and SP at once.:can restore half of total HP and SP.:can be cured from Silence, Curse and Chaos.",
                    )],
                )? {
                    1 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("can be cured from all the abnormal statuses.")?;
                    }
                    2 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("can restore full HP and SP at once.")?;
                    }
                    3 => {
                        ctx.mes("can restore half of total HP and SP.")?;
                    }
                    4 => {
                        l_w_point = (l_w_point.clone() + Val::from(1));
                        ctx.mes("can be cured from Silence, Curse and Chaos.")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                if l_w_point.clone().number()? > 0 {
                    ctx.lines_as(
                        "Saphien Layless",
                        args![
                            ((Val::from("...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(", have you lost your mind?")),
                            "Your work is too poor to be considered as a thesis!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Saphien Layless",
                        args![
                            "I don't think you can submit your work to the dean.",
                            "Go study harder and try again!"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("..........")?;
                ctx.next()?;
                ctx.mes("....................")?;
                ctx.next()?;
                ctx.mes(".................................")?;
                ctx.next()?;
                ctx.var("sage_q").set(Val::from(15))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2051), Val::from(2052)])?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "Oh, did you finish already? Well done.",
                        "Please handle this with care, because you won't be able to do this ever again."
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(1550), Val::from(1)])?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "I assume you're ready to submit your work to the dean.",
                        "Whether you will pass the test or not is his decision."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "I am not sure that you're ready to write a thesis.",
                    "I am afraid to say I already informed you of what you need."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "^3355FF1 Feather of Birds^000000 which will be used as a pen,",
                        "^3355FF1 Animal Skin^000000 which will be used as paper,",
                        "^3355FF1 Trunk^000000 which will be used to bind a book,",
                        "^3355FF1 Squid Ink^000000 which will be used as ink,",
                        "^3355FF1 Empty Bottle^000000 which will be used for holding squid ink."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "I will assist you in writing your thesis.",
                        "Please bring all those items with you."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if ctx.var("sage_q").get()? == 15 {
                ctx.lines(args![
                    "Huh? Aren't you supposed to head over to the dean?",
                    "If you've finished writing your thesis, please bring it to the dean."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Saphien Layless",
                    args![
                        "You can have only one chance to write your thesis before you become a Sage.",
                        "So please move on."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args![
                    "I am not sure if you have business with me, but please come back later.",
                    "I have some issues to think about.."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn history_professor_sa(ctx: &Ctx) -> Script {
    history_professor_sa_body(ctx, Vec::new()).map(|_| ())
}

fn biology_professor_sa_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_items: Vec<Val> = Vec::new();
    ctx.mes("[Lucius Celsus]")?;
    if !ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_MAGE")?) {
        if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SAGE")?) {
            ctx.lines(args![
                "What is your business with me?",
                "You must make a reservation a week in advance if you have any questions."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "You don't know how busy person I am...don't you?",
                    "If you're a Sage, you're supposed to know about me by now."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "You have too much time on your hands. Go explore some dungeons.",
                    "I think it will be more helpful than wasting your time on me."
                ],
            )?;
        } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?) {
            ctx.mes("What brings you to me, kid?")?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "You'd better go out and play with your pals. ",
                    "This is not a place where you can fool around."
                ],
            )?;
        } else {
            ctx.mes("Hmm? What brings you to me? Are you interested in watching monsters?")?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "You're allowed to watch. However, do not disturb them by making any fuss.",
                    "And keep your hands off, some of these guys are way too dangerous to touch."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "By the way, if you catch any rare monsters in future, let me know.",
                    "I am willing to purchase those at any cost."
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("sage_q").get()? == 11 {
        if ctx.var("sage_q2").get()? == 0 {
            ctx.lines(args![
                "Welcome to my class, did you earn good results in the practical exam?",
                "My name is Lucius Celsus, the expert of Biology."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "Huh...how rude of you! You're expected to introduce yourself to me as soon as I greet you!",
                    "What is your name, young one?"
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[ctx.call(Function::StrCharInfo, vec![Val::from(0)])?])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "A fine name. It's nice to meet you.",
                    "So, are you aware of the subject you're studying?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "As you know, your topic of study is monsters.",
                    "How many times have you fought with monsters?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("Well, I can't even count.:A few times, I guess...")],
            )?) == 1
            {
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Oh shut up, you brat. Don't be so sure about yourself.",
                        "Even if you have much experience with monsters, you will have a hard time comprehending my lecture."
                    ],
                )?;
            }
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "Yes, that's what I guessed about you. You're just book smart.",
                    "However, I am sure you will encounter most of the monsters mentioned in my lecture."
                ],
            )?;
            ctx.next()?;
            ctx.var("sage_m4")
                .set(ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?)?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "Let's get started.",
                    "Make sure you're ready for the practical examination during my lecture."
                ],
            )?;
            ctx.next()?;
            if ctx.var("sage_m4").get()? == 1 {
                ctx.var("sage_q2").set(Val::from(1))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2048), Val::from(2053)])?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Go bring the following items to me.",
                        "5 ^3355FFTentacle^000000,",
                        "5 ^3355FFSingle Cell^000000,",
                        "5 ^3355FFFish Tail^000000."
                    ],
                )?;
            } else if ctx.var("sage_m4").get()? == 2 {
                ctx.var("sage_q2").set(Val::from(2))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2048), Val::from(2054)])?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Go bring the following items to me.",
                        "5 ^3355FFNipper^000000,",
                        "5 ^3355FFClam Flesh^000000,",
                        "5 ^3355FFHeart of Mermaid^000000."
                    ],
                )?;
            } else {
                ctx.var("sage_q2").set(Val::from(3))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(2048), Val::from(2054)])?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Go bring following items to me.",
                        "5 ^3355FFTendon^000000,",
                        "5 ^3355FFNipper^000000,",
                        "5 ^3355FFSharp Scale^000000."
                    ],
                )?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args!["I will proceed with the class when you bring those to me.", "Have fun."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("sage_q2").get()?.number()? >= 1 && ctx.var("sage_q2").get()?.number()? <= 3) {
            let subject1 = ctx.var("sage_q2").get()?;
            if subject1 == 1 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(962), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(1052), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1023), false);
            } else if subject1 == 2 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(960), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(966), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(950), false);
            } else if subject1 == 3 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1050), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(960), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(963), false);
            }
            if ((ctx
                .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                .number()?
                > 4
                && ctx
                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                    .number()?
                    > 4)
                && ctx
                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(2), false)])?
                    .number()?
                    > 4)
            {
                ctx.lines(args![
                    "You showed great effort to gather all of those.",
                    "Well, I am not sure if you gathered them by yourself or bought them from shops..."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Somehow the monsters that drop those items have something in common.",
                        "Can you tell me what that similarity is?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "They possess water property.:They are fishes.:They are aggressive.:Um...they monsters.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "Yes, they possess water property and at the same time they are fishes.",
                                "Most fish class monsters live underwater, so they are attributed with the water property."
                            ],
                        )?;
                    }
                    2 => {
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "Yes, they possess water property and at the same time they are fishes.",
                                "Most fish class monsters live underwater, so they are attributed with the water property."
                            ],
                        )?;
                    }
                    3 => {
                        ctx.var("sage_m4").set(Val::from(4))?;
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "...I didn't know Phens were aggressive nowadays?",
                                "Or do Marina and Plankton team up to start a fight with you?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "All the monsters from which you obtained these items are not agressive... get a grip.",
                                "They are all fishes and possess water property."
                            ],
                        )?;
                    }
                    4 => {
                        ctx.var("sage_m4").set(Val::from(4))?;
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "What...! What are you here for!? You are here to study about specific monsters, microcephalic moron!",
                                "Sigh...they are all fishes and possess water property."
                            ],
                        )?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Not all fish class monsters possess water property, but most of them do.",
                        "So which kind of magic would work best on most fish class monsters?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Lightening Bolt.:Fire Bolt.:Thunder Storm.:Frost Diver.")])? {
                    1 => {
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "That's right, Lightening Bolt, which possesses the wind property, works best on water property monsters.",
                                "Although you might want to be careful of monsters that recognize magic casting."
                            ],
                        )?;
                    }
                    2 => {
                        ctx.var("sage_m4").set(Val::from(4))?;
                        ctx.lines_as("Lucius Celsus", args!["What? Fire Bolt! Fire cannot beat water, you imbecile!", "Most fishes are attributed with the water property. Therefore, they are weak to wind property magic spells. Don't you get it?"])?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Lucius Celsus",
                            args![
                                "Yeah, Thunder Storm spell is fine... it's a wind property spell.",
                                "However, you will be in trouble if you use the spell in a poorly chosen spot."
                            ],
                        )?;
                    }
                    4 => {
                        ctx.var("sage_m4").set(Val::from(4))?;
                        ctx.lines_as("Lucius Celsus", args!["I can't fathom such stupidity! This question asks you to choose a property that counters water! Don't you get it?", "Logically, any magic spell possessing the water property cannot overcome the water atrribute monsters!"])?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "By the way, although some monsters such as Penomena or Aster are considered to be fish class monsters, ",
                        "they have a different property than the others. You'd better be careful with them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args!["Okay, let me teach you about insect monsters.", "Let's see... hmm... hmm..."],
                )?;
                ctx.next()?;
                let subject4 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if subject4 == 1 {
                    ctx.var("sage_q2").set(Val::from(4))?;
                    if ctx.call(Function::CheckQuest, vec![Val::from(2053)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2053), Val::from(2056)])?;
                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2054)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2054), Val::from(2056)])?;
                    } else {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2055), Val::from(2056)])?;
                    }
                    ctx.lines(args![
                        "5 ^3355FFCobweb^000000,",
                        "5 ^3355FFShell^000000,",
                        "5 ^3355FFInsect Feeler^000000."
                    ])?;
                } else if subject4 == 2 {
                    ctx.var("sage_q2").set(Val::from(5))?;
                    if ctx.call(Function::CheckQuest, vec![Val::from(2053)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2053), Val::from(2057)])?;
                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2054)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2054), Val::from(2057)])?;
                    } else {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2055), Val::from(2057)])?;
                    }
                    ctx.lines(args![
                        "5 ^3355FFHorn^000000,",
                        "5 ^3355FFSnail's Shell^000000,",
                        "5 ^3355FFMoth Dust^000000."
                    ])?;
                } else if subject4 == 3 {
                    ctx.var("sage_q2").set(Val::from(6))?;
                    if ctx.call(Function::CheckQuest, vec![Val::from(2053)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2053), Val::from(2058)])?;
                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2054)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2054), Val::from(2058)])?;
                    } else {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2055), Val::from(2058)])?;
                    }
                    ctx.lines(args![
                        "5 ^3355FFMantis Scythe^000000,",
                        "5 ^3355FFWorm Peeling^000000,",
                        "5 ^3355FFRainbow Shell^000000."
                    ])?;
                } else if subject4 == 4 {
                    ctx.var("sage_q2").set(Val::from(7))?;
                    if ctx.call(Function::CheckQuest, vec![Val::from(2053)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2053), Val::from(2059)])?;
                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2054)])? != -1 {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2054), Val::from(2059)])?;
                    } else {
                        ctx.call(Function::ChangeQuest, vec![Val::from(2055), Val::from(2059)])?;
                    }
                    ctx.lines(args![
                        "5 ^3355FFCobweb^000000,",
                        "5 ^3355FFMantis Scythe^000000,",
                        "5 ^3355FFSolid Shell^000000."
                    ])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args!["I will proceed with the class when you bring those to me.", "Have fun."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "What, you already forgot what I told you just a minute before?",
                "What a nusance... listen carefully this time."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    ((Val::from("5 ^3355FF")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                        + Val::from("^000000,")),
                    ((Val::from("5 ^3355FF")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                        + Val::from("^000000,")),
                    ((Val::from("5 ^3355FF")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                        + Val::from("^000000,"))
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("sage_q2").get()?.number()? >= 4 && ctx.var("sage_q2").get()?.number()? <= 7) {
            let subject5 = ctx.var("sage_q2").get()?;
            if subject5 == 4 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1025), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(935), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(928), false);
            } else if subject5 == 5 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(947), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(946), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1057), false);
            } else if subject5 == 6 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1031), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(955), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(1013), false);
            } else if subject5 == 7 {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_items, &Val::from(base + 0), Val::from(1025), false);
                runtime::local_set(&mut l_items, &Val::from(base + 1), Val::from(1031), false);
                runtime::local_set(&mut l_items, &Val::from(base + 2), Val::from(943), false);
            }
            if ((ctx
                .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(0), false)])?
                .number()?
                > 4
                && ctx
                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(1), false)])?
                    .number()?
                    > 4)
                && ctx
                    .call(Function::CountItem, vec![runtime::local_get(&l_items, &Val::from(2), false)])?
                    .number()?
                    > 4)
            {
                ctx.lines(args![
                    "Well done. So, did you watch insects while gathering those items?",
                    "Oh well, I believe you did a good job with the task."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Insect class monsters do not share the same property most of the time, ",
                        "You must think twice before you cast a magic spell on an insect."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "It's remarkable that insects can detect hidden objects.",
                        "Therefore, any hiding skill such as the Hiding skill or Cloaking skill will not work on insect monsters."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "Some insects form a group and live together.",
                        "They are controlled by the head of the group..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "For instance, Maya the queen ant...",
                        "Mistress, the queen of hornets,",
                        "or Golden Thiefbug, the king of thiefbugs..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "You cannot beat those boss monsters alone, ",
                        "you'd better form a party if you want to fight with them."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "That's the end of my class...it's time for you to write a thesis.",
                        "Bring me following items for writing the thesis."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "^3355FF1 Feather of Birds^000000 which will be used as a pen,",
                        "^3355FF1 Animal Skin^000000 which will be used as paper,",
                        "^3355FF1 Trunk^000000 which will be used to bind a book,",
                        "^3355FF1 Squid Ink^000000 which will be used as ink,",
                        "^3355FF1 Empty Bottle^000000 which will be used for holding squid ink."
                    ],
                )?;
                ctx.next()?;
                ctx.var("sage_q2").set(Val::from(0))?;
                ctx.var("sage_q").set(Val::from(12))?;
                if ctx.call(Function::CheckQuest, vec![Val::from(2056)])? != -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2056), Val::from(2051)])?;
                } else if ctx.call(Function::CheckQuest, vec![Val::from(2057)])? != -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2057), Val::from(2051)])?;
                } else if ctx.call(Function::CheckQuest, vec![Val::from(2058)])? != -1 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2058), Val::from(2051)])?;
                } else {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2059), Val::from(2051)])?;
                }
                ctx.lines_as(
                    "Lucius Celsus",
                    args![
                        "I will help you in writing the thesis when you bring all of those items.",
                        "You're almost there. Isn't learning easy?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args![
                "What, you already forgot what I told you?",
                "What a nuisance...listen carefully this time."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    ((Val::from("5 ^3355FF")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(0), false)])?)
                        + Val::from("^000000,")),
                    ((Val::from("5 ^3355FF")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(1), false)])?)
                        + Val::from("^000000,")),
                    ((Val::from("5 ^3355FF")
                        + ctx.call(Function::GetItemName, vec![runtime::local_get(&l_items, &Val::from(2), false)])?)
                        + Val::from("^000000,"))
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("Zzz...Zzz...")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("sage_q").get()? == 12 {
        if ((((ctx.call(Function::CountItem, vec![Val::from(916)])?.number()? > 0
            && ctx.call(Function::CountItem, vec![Val::from(919)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(1019)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(1024)])?.number()? > 0)
            && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 0)
        {
            ctx.call(Function::DelItem, vec![Val::from(916), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(919), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1019), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1024), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(713), Val::from(1)])?;
            ctx.lines(args![
                "Hmph. Lucky brat brought all of the items.",
                "Well, I don't expect you to write an outrageously great thesis though..."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "Well, if you really want to write it by yourself, I can let you handle it but...",
                    "I will give you a work of staggering genius. Just make a copy of it under your name."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args!["You got a problem with that? Tough, that's my style.", "Just do what I say."],
            )?;
            ctx.next()?;
            ctx.mes("..........")?;
            ctx.next()?;
            ctx.mes("....................")?;
            ctx.next()?;
            ctx.mes(".................................")?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from(".....Monsters vary by physical appearance,")])?;
            ctx.var("@menu").set(choice)?;
            ctx.mes(".....Monsters vary by physical appearance,")?;
            let choice = runtime::select_values(ctx, &[Val::from("...and possess various elemental properties.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.mes("...and possess various elemental properties.")?;
            let choice = runtime::select_values(ctx, &[Val::from("You must be aware of each monster's properties,")])?;
            ctx.var("@menu").set(choice)?;
            ctx.mes("You must be aware of each monster's properties,")?;
            let choice = runtime::select_values(
                ctx,
                &[Val::from(
                    "...and be aware that certain spells work differently on different monsters.",
                )],
            )?;
            ctx.var("@menu").set(choice)?;
            ctx.mes("...and be aware that certain spells work differently on different monsters.")?;
            let choice = runtime::select_values(
                ctx,
                &[Val::from(
                    "You must be especially careful of holy property and shadow property monsters.",
                )],
            )?;
            ctx.var("@menu").set(choice)?;
            ctx.mes("You must be especially careful of holy property and shadow property monsters.")?;
            let choice = runtime::select_values(
                ctx,
                &[Val::from("These monsters are most dangerous, though occasionally cute.")],
            )?;
            ctx.var("@menu").set(choice)?;
            ctx.mes("These monsters are most dangerous.")?;
            ctx.next()?;
            ctx.mes("..........")?;
            ctx.next()?;
            ctx.mes("....................")?;
            ctx.next()?;
            ctx.mes(".................................")?;
            ctx.next()?;
            ctx.var("sage_q").set(Val::from(15))?;
            ctx.call(Function::ChangeQuest, vec![Val::from(2051), Val::from(2052)])?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "Are you done? Okay, then it's over.",
                    "You won't be able to write another thesis again, handle this with care."
                ],
            )?;
            ctx.call(Function::GetItem, vec![Val::from(1550), Val::from(1)])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "Show this masterpiece to the dean.",
                    "Then, he will let you graduate from the academy. See you."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "What, are you sure that you're ready? No, I don't think so.",
                "Oh well... listen carefully this time."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "^3355FF1 Feather of Birds^000000 which will be used as a pen,",
                    "^3355FF1 Animal Skin^000000 which will be used as paper,",
                    "^3355FF1 Trunk^000000 which will be used to bind a book,",
                    "^3355FF1 Squid Ink^000000 which will be used as ink,",
                    "^3355FF1 Empty Bottle^000000 which will be used for holding squid ink."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "You have been doing fine, I guess you can do this without a problem.",
                    "Go get them. Hurry up."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        if ctx.var("sage_q").get()? == 15 {
            ctx.lines(args![
                "What are you doing here, Go show your thesis to the dean!",
                "Don't wasn't any more time here."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "It seems you have too much time on your hands. Okay, I will assign you some tasks.",
                    "Hahaha, did you say no? Alright then, fine. Scram."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "Wah~! My brain is gonna blow up soon! Why must I have to prepare all of these things?!",
                "Who are you?! Don't disturb me, I'm busy!!"
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Lucius Celsus",
                args![
                    "If you just want to watch monsters here, fine with me...",
                    "Just don't ask me any questions."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn biology_professor_sa(ctx: &Ctx) -> Script {
    biology_professor_sa_body(ctx, Vec::new()).map(|_| ())
}
