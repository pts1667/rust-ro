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

pub fn noyee_g(ctx: &Ctx) -> Script {
    if ctx.var("BaseLevel").get()?.number()? < 70 {
        ctx.lines_as("Noyee", args!["Did you know that there's a difference between the armor you can buy from NPC shops and the kinds you obtain from hunting monsters?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Noyee",
            args!["The difference, my friend, is in the number of ^3355FFslots^000000 in your armor."],
        )?;
        ctx.next()?;
        ctx.lines_as("Noyee", args!["The same kind of Armor will have the same Defense. However, if an Armor has a slot, you can insert a Monster Card which will add an enhancement to that armor."])?;
        ctx.next()?;
        ctx.lines_as(
            "Noyee",
            args!["Both Monster Cards and Slotted Armors are rarely dropped by monsters, and are thus valuable commodies."],
        )?;
        return ctx.close();
    } else if runtime::op(&ctx.var("$god1").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
        ctx.lines_as("Noyee", args!["Did you know that there's a difference between the armor you can buy from NPC shops and the kinds you obtain from hunting monsters?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Noyee",
            args!["The difference, my friend, is in the number of ^3355FFslots^000000 in your armor."],
        )?;
        ctx.next()?;
        ctx.lines_as("Noyee", args!["The same kind of Armor will have the same Defense. However, if an Armor has a slot, you can insert a Monster Card which will add an enhancement to that armor."])?;
        ctx.next()?;
        ctx.lines_as(
            "Noyee",
            args!["Both Monster Cards and Slotted Armors are rarely dropped by monsters, and are thus valuable commodies."],
        )?;
        return ctx.close();
    } else {
        ctx.lines_as(
            "Noyee",
            args![
                "You know, there's a very interesting place that looks",
                "sort of like a laboratory in Juno."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Noyee", args!["Anyway, no one is really sure if it is or not. Recently, it's been the focus of many rumors. I have no idea if any of them are true though."])?;
        ctx.close_window()?;
        if ctx.var("god_sl_1").get()? == 0 {
            ctx.var("god_sl_1").set(Val::from(1))?;
        }
    }
    Ok(())
}

pub fn manager_g(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![908, 500])? == 0 {
        ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
        return ctx.close();
    }
    if ctx.var("BaseLevel").get()?.number()? < 70 {
        ctx.lines_as(
            "Cukure",
            args!["I can't believe how busy I am at work nowadays. I don't even have time to see any of my friends or go shopping."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cukure",
            args![
                "Wait...!",
                "I can't even remember the last",
                "time I went out with a guy! At this rate, I might retire before I can find a boyfriend. Noooo, I'm still in my prime~!"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
        return ctx.close();
    }
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 0 {
            ctx.lines_as(
                "Cukure",
                args!["I can't believe how busy I am at work nowadays. I don't even have time to see any of my friends or go shopping."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "Wait...!",
                    "I can't even remember the last",
                    "time I went out with a guy! At this rate, I might retire before I can find a boyfriend. Noooo, I'm still in my prime~!"
                ],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
            return ctx.close();
        } else if ctx.var("god_sl_1").get()? == 1 {
            ctx.lines_as("Cukure", args!["We conduct research on godly artifacts that were left behind by the gods long ago. More specifically, we're interested in the tools the gods used."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "Recently, many of the members of our research group, aside from the leaders, have left so now we're short on manpower."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["Since we're understaffed, we can't continue most of our research projects that are currently in progress. Speaking of which..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args!["Do you mind if I ask", "an adventurer like yourself", "to help us in our research?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["We won't experiment on you or anything, we just need you to run some simple errands. And of course, you will be rewarded."])?;
            ctx.next()?;
            match ctx.menu(&["I'm kind of busy...", "Sure, why not?"])? {
                0 => {
                    ctx.lines_as("Cukure", args!["Oh, I see.", "Well, if you happen to find anyone that might work well as a temporary research assistant, would you recommend them to us?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cukure",
                        args![
                            "I'm so sorry to ask this of you, but I hope you understand that we're seriously lacking in terms",
                            "of human resources."
                        ],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.fx().cutin("god_kukur02", 2)?;
                    ctx.lines_as(
                        "Cukure",
                        args!["Really~?!", "That's great!", "Hahahahaha! I'm so", "glad to hear that!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Cukure", args!["Go inside and speak to the investigators there. They're all kind of swamped, so I'm sure they'll need your assistance."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cukure",
                        args![
                            "Oh, wait...",
                            "Let me give you a temporary admission pass. Thank you",
                            "so much for your help~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFYou've received", "a temporary", "admission pass.^000000"])?;
                    ctx.var("god_sl_1").set(Val::from(2))?;
                    ctx.close_window()?;
                }
                _ => {}
            }
        } else if ctx.var("god_sl_1").get()?.number()? < 10 {
            ctx.fx().cutin("god_kukur03", 2)?;
            ctx.lines_as("Cukure", args!["It's really difficult to managing a laboratory. Rival labs always try to entice our staff away or interrupt our progress."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "And because of this job, it's really hard for me to enjoy",
                    "having a personal life..."
                ],
            )?;
            ctx.next()?;
            ctx.fx().cutin("god_kukur02", 2)?;
            ctx.lines_as(
                "Cukure",
                args![
                    "Hahahaha...",
                    "What am I doing?",
                    "Please just ignore what I just said. It's really not that big of a deal. ^333333*Sigh...*^000000"
                ],
            )?;
            ctx.close_window()?;
        } else if ctx.var("god_sl_1").get()?.number()? < 50 {
            ctx.lines_as(
                "Cukure",
                args![
                    "Ah, thank you",
                    "for helping us out.",
                    "I hope you'll continue",
                    "to lend us your assistance."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args!["Although our researchers are kind of strange, they're all pretty nice and they mean well."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args!["So I hope you understand if a few of them treat you in a way that seems kind of rude."],
            )?;
            ctx.close_window()?;
        } else if ctx.var("god_sl_1").get()? == 50 {
            ctx.fx().cutin("god_kukur02", 2)?;
            ctx.lines_as(
                "Cukure",
                args!["Ah...", "You really helped us out a lot. Thank you for all you have done."],
            )?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["As you already know, we're trying to find out whether or not we can reproduce 'Sleipnir,' one of the tools used by the gods."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "However....",
                    "So far it seems impossible, at least with the technology that we currently have. Hmm..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["Anyways, we will continue to research and hope that we'll be able to accomplish our goal of recreating Sleipnir someday."])?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["Thanks to you, we were able to complete a rudimentary study of Sleipnir. Now we can plan out a more advanced study of Sleipnir."])?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "Before I forget, let me give you your reward for assisting in our research projects. Please give",
                    "me a minute..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "Ah, there it is.",
                    "It might not be much, but I hope you accept it as a token of my gratitude. Good luck on your travels!"
                ],
            )?;
            if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
                ctx.var("$god1").set((ctx.var("$god1").get()? + Val::from(1)))?;
            }
            if ctx.var("$god1").get()?.loosely_equals(&ctx.var("$@god_check1").get()?) {
                ctx.call(
                    Function::Announce,
                    args!["The 1st seal of [Sleipnir] has appeared.", constants::BC_ALL],
                )?;
            } else if ctx.var("$god1").get()?.loosely_equals(&ctx.var("$@god_check2").get()?) {
                if ctx.var("$god1").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                    && ctx.var("$god2").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                    && ctx.var("$god3").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                    && ctx.var("$god4").get()?.loosely_equals(&ctx.var("$@god_check2").get()?)
                {
                    ctx.call(
                        Function::Announce,
                        args![
                            "our seals have been released at the same time with the seal of [Sleipnir].",
                            constants::BC_ALL
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::Announce,
                        args!["The 1st seal of [Sleipnir] has been released.", constants::BC_ALL],
                    )?;
                }
            }
            ctx.var("god_sl_1").set(Val::from(51))?;
            let l_god_treasure = ctx.rand_range(1, 900)?;
            let l_god_treasure1 = ctx.rand_range(1, 1000)?;
            if l_god_treasure < 101 {
                if l_god_treasure1 < 210 {
                    ctx.items().give(2102, 1)?;
                } else if l_god_treasure1 < 420 {
                    ctx.items().give(2104, 1)?;
                } else if l_god_treasure1 < 630 {
                    ctx.items().give(2106, 1)?;
                } else if l_god_treasure1 < 840 {
                    ctx.items().give(2108, 1)?;
                } else {
                    ctx.items().give(2109, 1)?;
                }
            } else if l_god_treasure < 201 {
                if l_god_treasure1 < 48 {
                    ctx.items().give(2254, 1)?;
                } else if l_god_treasure1 < 96 {
                    ctx.items().give(2210, 1)?;
                } else if l_god_treasure1 < 150 {
                    ctx.items().give(2213, 1)?;
                } else if l_god_treasure1 < 190 {
                    ctx.items().give(2255, 1)?;
                } else if l_god_treasure1 < 238 {
                    ctx.items().give(2217, 1)?;
                } else if l_god_treasure1 < 290 {
                    ctx.items().give(2223, 1)?;
                } else if l_god_treasure1 < 340 {
                    ctx.items().give(2227, 1)?;
                } else if l_god_treasure1 < 348 {
                    ctx.items().give(2229, 1)?;
                } else if l_god_treasure1 < 400 {
                    ctx.items().give(2231, 1)?;
                } else if l_god_treasure1 < 448 {
                    ctx.items().give(2233, 1)?;
                } else if l_god_treasure1 < 496 {
                    ctx.items().give(5053, 1)?;
                } else if l_god_treasure1 < 544 {
                    ctx.items().give(5019, 1)?;
                } else if l_god_treasure1 < 592 {
                    ctx.items().give(2245, 1)?;
                } else if l_god_treasure1 < 640 {
                    ctx.items().give(2247, 1)?;
                } else if l_god_treasure1 < 688 {
                    ctx.items().give(2248, 1)?;
                } else if l_god_treasure1 < 736 {
                    ctx.items().give(5166, 1)?;
                } else if l_god_treasure1 < 784 {
                    ctx.items().give(5158, 1)?;
                } else if l_god_treasure1 < 832 {
                    ctx.items().give(2249, 1)?;
                } else if l_god_treasure1 < 880 {
                    ctx.items().give(5157, 1)?;
                } else if l_god_treasure1 < 940 {
                    ctx.items().give(2285, 1)?;
                } else {
                    ctx.items().give(5093, 1)?;
                }
            } else if l_god_treasure < 301 {
                if l_god_treasure1 < 160 {
                    ctx.items().give(5014, 1)?;
                } else if l_god_treasure1 < 320 {
                    ctx.items().give(5005, 1)?;
                } else if l_god_treasure1 < 480 {
                    ctx.items().give(5054, 1)?;
                } else if l_god_treasure1 < 540 {
                    ctx.items().give(2265, 1)?;
                } else if l_god_treasure1 < 700 {
                    ctx.items().give(2266, 1)?;
                } else if l_god_treasure1 < 800 {
                    ctx.items().give(2260, 1)?;
                } else if l_god_treasure1 < 900 {
                    ctx.items().give(5113, 1)?;
                } else {
                    ctx.items().give(2270, 1)?;
                }
            } else if l_god_treasure < 401 {
                if l_god_treasure1 < 70 {
                    ctx.items().give(2286, 1)?;
                } else if l_god_treasure1 < 140 {
                    ctx.items().give(5002, 1)?;
                } else if l_god_treasure1 < 210 {
                    ctx.items().give(5147, 1)?;
                } else if l_god_treasure1 < 280 {
                    ctx.items().give(2217, 1)?;
                } else if l_god_treasure1 < 350 {
                    ctx.items().give(5120, 1)?;
                } else if l_god_treasure1 < 420 {
                    ctx.items().give(2261, 1)?;
                } else if l_god_treasure1 < 490 {
                    ctx.items().give(5162, 1)?;
                } else if l_god_treasure1 < 560 {
                    ctx.items().give(5030, 1)?;
                } else if l_god_treasure1 < 630 {
                    ctx.items().give(5109, 1)?;
                } else if l_god_treasure1 < 700 {
                    ctx.items().give(5084, 1)?;
                } else if l_god_treasure1 < 770 {
                    ctx.items().give(5168, 1)?;
                } else if l_god_treasure1 < 840 {
                    ctx.items().give(2214, 1)?;
                } else if l_god_treasure1 < 900 {
                    ctx.items().give(2295, 1)?;
                } else if l_god_treasure1 < 950 {
                    ctx.items().give(5167, 1)?;
                } else {
                    ctx.items().give(5018, 1)?;
                }
            } else if l_god_treasure < 501 {
                if l_god_treasure1 < 85 {
                    ctx.items().give(2310, 1)?;
                } else if l_god_treasure1 < 170 {
                    ctx.items().give(2311, 1)?;
                } else if l_god_treasure1 < 255 {
                    ctx.items().give(2313, 1)?;
                } else if l_god_treasure1 < 340 {
                    ctx.items().give(2317, 1)?;
                } else if l_god_treasure1 < 425 {
                    ctx.items().give(2319, 1)?;
                } else if l_god_treasure1 < 510 {
                    ctx.items().give(2320, 1)?;
                } else if l_god_treasure1 < 595 {
                    ctx.items().give(2322, 1)?;
                } else if l_god_treasure1 < 680 {
                    ctx.items().give(2359, 1)?;
                } else if l_god_treasure1 < 765 {
                    ctx.items().give(2326, 1)?;
                } else if l_god_treasure1 < 850 {
                    ctx.items().give(2342, 1)?;
                } else if l_god_treasure1 < 935 {
                    ctx.items().give(2331, 1)?;
                } else {
                    ctx.items().give(2336, 1)?;
                }
            } else if l_god_treasure < 601 {
                if l_god_treasure1 < 200 {
                    ctx.items().give(2422, 1)?;
                } else if l_god_treasure1 < 400 {
                    ctx.items().give(2404, 1)?;
                } else if l_god_treasure1 < 600 {
                    ctx.items().give(2406, 1)?;
                } else if l_god_treasure1 < 800 {
                    ctx.items().give(2407, 1)?;
                } else {
                    ctx.items().give(2412, 1)?;
                }
            } else if l_god_treasure < 701 {
                if l_god_treasure1 < 200 {
                    ctx.items().give(2513, 1)?;
                } else if l_god_treasure1 < 400 {
                    ctx.items().give(2504, 1)?;
                } else if l_god_treasure1 < 600 {
                    ctx.items().give(2506, 1)?;
                } else if l_god_treasure1 < 800 {
                    ctx.items().give(2514, 1)?;
                } else {
                    ctx.items().give(2508, 1)?;
                }
            } else if l_god_treasure < 801 {
                if l_god_treasure1 < 110 {
                    ctx.items().give(1122, 1)?;
                } else if l_god_treasure1 < 220 {
                    ctx.items().give(2622, 1)?;
                } else if l_god_treasure1 < 330 {
                    ctx.items().give(2623, 1)?;
                } else if l_god_treasure1 < 440 {
                    ctx.items().give(2624, 1)?;
                } else if l_god_treasure1 < 550 {
                    ctx.items().give(2625, 1)?;
                } else if l_god_treasure1 < 660 {
                    ctx.items().give(2607, 1)?;
                } else if l_god_treasure1 < 770 {
                    ctx.items().give(2626, 1)?;
                } else if l_god_treasure1 < 880 {
                    ctx.items().give(2617, 1)?;
                } else {
                    ctx.items().give(2671, 1)?;
                }
            } else if l_god_treasure1 < 150 {
                ctx.items().give(2281, 1)?;
            } else if l_god_treasure1 < 260 {
                ctx.items().give(2297, 1)?;
            } else if l_god_treasure1 < 370 {
                ctx.items().give(5087, 1)?;
            } else if l_god_treasure1 < 480 {
                ctx.items().give(5088, 1)?;
            } else if l_god_treasure1 < 590 {
                ctx.items().give(5089, 1)?;
            } else if l_god_treasure1 < 700 {
                ctx.items().give(5090, 1)?;
            } else if l_god_treasure1 < 810 {
                ctx.items().give(5086, 1)?;
            } else if l_god_treasure1 < 920 {
                ctx.items().give(2292, 1)?;
            } else {
                ctx.items().give(5006, 1)?;
            }

            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args![
                    "Ah, I'll also need to take back that temporary pass I gave to you. I'm sorry, but it's protocol we have to follow..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["Once again,", "thank you for", "all your help~"])?;
            ctx.close_window()?;
        } else {
            ctx.fx().cutin("god_kukur02", 2)?;
            ctx.lines_as(
                "Cukure",
                args!["With your assistance, we successfully completed our rudimentary study."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Cukure",
                args!["From now on, we'll plan out our more advanced studies based on the information we now have on the godly artifacts."],
            )?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["Now that things", "have slowed down", "a little..."])?;
            ctx.next()?;
            ctx.lines_as("Cukure", args!["I can finally", "go on a date!", "Hahahahahaha~!"])?;
            ctx.close_window()?;
        }
    } else if ctx.var("god_sl_1").get()? == 0 {
        ctx.fx().cutin("god_kukur03", 2)?;
        ctx.lines_as(
            "Cukure",
            args!["I can't believe how busy I am at work nowadays. I don't even have time to see any of my friends or go shopping."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Cukure",
            args![
                "Wait...!",
                "I can't even remember the last",
                "time I went out with a guy! At this rate, I might retire before I can find a boyfriend. Noooo, I'm still in my prime~!"
            ],
        )?;
        ctx.call(Function::Emotion, args![constants::ET_PROFUSELY_SWEAT])?;
        ctx.close_window()?;
    } else if ctx.var("god_sl_1").get()?.number()? < 51 {
        ctx.lines_as(
            "Cukure",
            args![
                "Ah, you've come back.",
                "I know you were going to help us, but someone already applied for the assistant position."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Cukure", args!["Anyway, we've already finished our current research projects. Now, we're in the planning stages for the next projects we'll be conducting..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Cukure",
            args!["Thank you anyway, though. I hope you will come and help us the next time we need assistance on our projects."],
        )?;
        ctx.close_window()?;
    } else {
        ctx.lines_as(
            "Cukure",
            args!["Now that we've completed our rudimentary studies, we'll simply plan out our future research projects."],
        )?;
        ctx.next()?;
        ctx.lines_as("Cukure", args!["Now that things", "have slowed down", "a little..."])?;
        ctx.next()?;
        ctx.lines_as("Cukure", args!["I can finally", "go on a date!", "Hahahahahaha~!"])?;
        ctx.close_window()?;
    }
    ctx.fx().cutin("god_kukur01", 255)?;
    return ctx.end();
}

pub fn researcher_g1(ctx: &Ctx) -> Script {
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 2 {
            ctx.var("god_sl_1").set(Val::from(11))?;
        }
        if ctx.var("god_sl_1").get()? == 11
            || ctx.var("god_sl_1").get()? == 22
            || ctx.var("god_sl_1").get()? == 33
            || ctx.var("god_sl_1").get()? == 44
        {
            if ctx.var("god_sl_2").get()? == 0 {
                ctx.lines_as(
                    "Hallandaute",
                    args!["Ah, are you the one whom Ms. Kirin referred? Yes, she said you were going to help me in my research."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hallandaute",
                    args![
                        "Frankly, we don't really need anyone who doesn't specialize",
                        "in our field of study."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hallandaute", args!["But if you don't mind helping us out by doing little, menial tasks, then it'd probably bring some relief to our workload."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Hallandaute",
                    args!["Now...", "What would be", "a good job for you?", "Let me think..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hallandaute",
                    args![
                        "I got it!",
                        "Why don't you go and visit ^FF0000Metto^000000 for me? Please bring me any news that he'd like to report."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hallandaute", args!["Since he's working on a project similar to mine, he might be in a position to help me. Metto lives in Juno, so you can easily find me."])?;
                ctx.var("god_sl_2").set(Val::from(1))?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 1 {
                if ctx.var("metto_q").get()? == 0 {
                    ctx.lines_as("Hallandaute", args!["I guess you haven't found Metto yet. Although he might be a little over enthusiastic at times, he's not a bad guy at all."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hallandaute",
                        args!["We've been friends for a long time after working in the same field for so long. I wonder how he's doing?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["Anyway, just ask him about any progress on his research. And if he happens to ask you for help, please lend him your assistance."])?;
                    return ctx.close();
                } else if ctx.var("metto_q").get()?.number()? < 9 {
                    ctx.lines_as(
                        "Hallandaute",
                        args!["Hmmm...", "So how's Metto?", "Hmm? It seems like", "he has changed..."],
                    )?;
                    return ctx.close();
                } else {
                    ctx.lines_as("Hallandaute", args!["Hmmm...", "So how's Metto?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hallandaute",
                        args![
                            "A mad scientist?",
                            "That doesn't sound right. In my wildest dreams, I never would have suspected Metto of turning out that way..."
                        ],
                    )?;
                    ctx.call(Function::Emotion, args![constants::ET_SCRATCH])?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["Huh. That's...", "That's too bad. I don't know what to say. We used to go watch movies and celebrate our birthdays together. P-please give me a moment..."])?;
                    ctx.call(Function::Emotion, args![constants::ET_THINK])?;
                    ctx.var("god_sl_2").set(Val::from(2))?;
                    return ctx.close();
                }
            } else if ctx.var("god_sl_2").get()? == 2 {
                if ctx.call(Function::Rand, args![1, 10])? == 7 {
                    ctx.lines_as(
                        "Hallandaute",
                        args![
                            "Ah...",
                            "You came back.",
                            "Alright, let me give you the gist of what I'm researching."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["I'm trying to learn more about the '^FF0000Wheel of the Unknown^000000,' which was discovered to be a piece of the artifact known as ^FF0000Sleipnir^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hallandaute",
                        args![
                            "It looks like",
                            "a normal cogwheel...",
                            "But I can't figure out how it's supposed to work."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hallandaute",
                        args![
                            "The most peculiar thing of all about this cogwheel is the metal",
                            "of which it is composed."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["It looks and feels like steel,but extreme heat and cold don't affect it. It also cannot be dented by any kind of impact or force I can apply to it."])?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["Sometimes, I find that the Wheel of the Unknown resonates with certain objects, but I haven't got it to occur all of the time."])?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["I'm guessing that I'm missing some sort of special condition that will make the Wheel of the Unknown resonate with specific materials."])?;
                    ctx.next()?;
                    ctx.lines_as("Hallandaute", args!["Anyway, I thank you for helping me out earlier. That's basically all I needed, so please talk to the other researchers and see if they need any assistance."])?;
                    ctx.var("god_sl_2").set(Val::from(0))?;
                    if ctx.var("god_sl_1").get()? == 11 {
                        ctx.var("god_sl_1").set(Val::from(12))?;
                    }
                    if ctx.var("god_sl_1").get()? == 22 {
                        ctx.var("god_sl_1").set(Val::from(23))?;
                    }
                    if ctx.var("god_sl_1").get()? == 33 {
                        ctx.var("god_sl_1").set(Val::from(34))?;
                    }
                    if ctx.var("god_sl_1").get()? == 44 {
                        ctx.var("god_sl_1").set(Val::from(50))?;
                    }
                    return ctx.close();
                } else {
                    ctx.lines_as(
                        "Hallandaute",
                        args!["Oh, sorry.", "I'm quite busy compiling all of this information at the moment."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hallandaute",
                        args!["Let's see...", "Now how would I...", "^333333*Mumble mumble*^000000"],
                    )?;
                    return ctx.close();
                }
            } else {
                ctx.lines_as(
                    "Hallandaute",
                    args![
                        "Hmmm...",
                        "How does this work?",
                        "There's still too much we don't know about the godly artifacts. ^333333*Sigh*^000000"
                    ],
                )?;
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "Hallandaute",
                args!["It's not easy, working on this alone. Having an extra pair of hands is always good."],
            )?;
            ctx.next()?;
            ctx.lines_as("Hallandaute", args!["Right now, I just need someone with whom I can share the workload. Still, different assistants might be better depending on the project."])?;
            ctx.next()?;
            ctx.lines_as("Hallandaute", args!["Basically, if an assistant is especially trained in the field related to my current research project, he'll probably be more useful than someone who's not."])?;
            ctx.next()?;
            ctx.lines_as("Hallandaute", args!["Of course, there are people who are good at everything. But since those are rare, I try to choose just the right assistant for each of my projects."])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Hallandaute",
            args!["It's not easy, working on this alone. Having an extra pair of hands is always good."],
        )?;
        ctx.next()?;
        ctx.lines_as("Hallandaute", args!["Right now, I just need someone with whom I can share the workload. Still, different assistants might be better depending on the project."])?;
        ctx.next()?;
        ctx.lines_as("Hallandaute", args!["Basically, if an assistant is especially trained in the field related to my current research project, he'll probably be more useful than someone who's not."])?;
        ctx.next()?;
        ctx.lines_as("Hallandaute", args!["Of course, there are people who are good at everything. But since those are rare, I try to choose just the right assistant for each of my projects."])?;
        return ctx.close();
    }
}

pub fn researcher_g2(ctx: &Ctx) -> Script {
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 2 {
            ctx.var("god_sl_1").set(Val::from(21))?;
        }
        if ctx.var("god_sl_1").get()? == 21
            || ctx.var("god_sl_1").get()? == 32
            || ctx.var("god_sl_1").get()? == 43
            || ctx.var("god_sl_1").get()? == 14
        {
            if ctx.var("god_sl_2").get()? == 0 {
                ctx.lines_as("Aadin", args!["^333333*Yawn~*^000000", "Hmmmmm?", "Ah..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Nice to meet you.",
                        "You're the temporary assistant, right? That's great, all of us are having a hard time working",
                        "without much help."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args!["Okay...", "Let me explain", "to you the project", "that I'm working on."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "I'm trying to find out everything I can about the '^FF0000Feather of Angel Wing^000000.' Supposedly, it's one of",
                        "the parts of 'Sleipnir.'"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Right now, I'm",
                        "focusing on studying",
                        "the background story of this feather, as well as how it came",
                        "to be a material for a godly artifact."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["I was recently told that there is a person in Payon who actually knows this story. Very few people in the world are privy to having such knowledge."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Now, we don't have enough",
                        "staff for me to send someone to Payon. Also, I don't feel safe taking this feather around with me."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["It shouldn't be that difficult to find the person who knows about Sleipnir's origin. However, you must memorize every single", "word of the story."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args!["The results of this research could be affected by the words you deliver to me."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Thank you for the trouble.",
                        "There's no real time limit for this task, but the sooner you complete it, the better."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["Have a safe trip~"])?;
                ctx.var("god_sl_2").set(Val::from(1))?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 1 {
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Mmm...?",
                        "You haven't left for Payon yet to find the man who knows of Sleipnir's origin?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Please leave for",
                        "Payon as soon as",
                        "possible so that I can get the information that I need for my research. Thank you, and have",
                        "a safe trip."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 2 {
                ctx.lines_as("Aadin", args!["Ah, you're back. So have you memorized the story?"])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["I know it'd be much easier if this tale was already written, but you must understand that many stories regarding the gods are only orally transmitted."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args!["Alright,", "I'm listening.", "Please let me", "know what you've learned."],
                )?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["...", "......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Hmm, I see.",
                        "Fascinating.",
                        "However, I'm sure",
                        "you've missed some",
                        "important detail."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["Are you sure you've listened to the tale in its entirety? I'm sorry to ask this of you, but would you go to Payon and listen to this story once more?"])?;
                ctx.var("god_sl_2").set(Val::from(3))?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 3 {
                ctx.lines_as(
                    "Aadin",
                    args![
                        "Mm...?",
                        "You haven't left",
                        "for Payon yet...?",
                        "I hope you can",
                        "tell me the rest",
                        "of the story soon."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args![
                        "In the meantime, I will try to complete as much of my research",
                        "as I can with the story details that you've already brought."
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 4 {
                ctx.lines_as("Aadin", args!["Ah, you're back.", "So have you", "memorized the story?"])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["I know it'd be much easier if this tale was already written, but you must understand that many stories regarding the gods are only orally transmitted."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args!["Alright,", "I'm listening.", "Please let me", "know what you've learned."],
                )?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["...", "......"])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["...", "......", "........."])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["...", "......", ".........", "............"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args!["Hmm, I see.", "Fascinating.", "you've missed some", "important detail."],
                )?;
                ctx.next()?;
                match ctx.menu(&["Well, um...", "Impossible!"])? {
                    0 => {
                        ctx.lines_as(
                            "Aadin",
                            args!["Hmm.", "No, wait...", "That was the", "complete story.", "My apologies~"],
                        )?;
                    }
                    1 => {
                        ctx.lines_as(
                            "Aadin",
                            args!["Ah~", "I stand corrected.", "Thank you for pointing", "out my error in judgment."],
                        )?;
                    }
                    _ => {}
                }
                ctx.next()?;
                ctx.lines_as("Aadin", args!["Yes, it seems that the story you've just told me should be authentic. Of course, it's understandable if little changes are made, depending on the storyteller."])?;
                ctx.next()?;
                ctx.lines_as("Aadin", args!["Surely, I'll be able to answer some of my most important questions with the information you've brought to me, though it still won't be easy."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Aadin",
                    args!["After all, finding answers tends to cause new questions to arise. Nonetheless, I thank you for your help."],
                )?;
                ctx.var("god_sl_2").set(Val::from(0))?;
                if ctx.var("god_sl_1").get()? == 21 {
                    ctx.var("god_sl_1").set(Val::from(22))?;
                }
                if ctx.var("god_sl_1").get()? == 32 {
                    ctx.var("god_sl_1").set(Val::from(33))?;
                }
                if ctx.var("god_sl_1").get()? == 43 {
                    ctx.var("god_sl_1").set(Val::from(44))?;
                }
                if ctx.var("god_sl_1").get()? == 14 {
                    ctx.var("god_sl_1").set(Val::from(50))?;
                }
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "Aadin",
                args!["Sometimes, different versions of the same myth or legend arise."],
            )?;
            ctx.next()?;
            ctx.lines_as("Aadin", args!["Although myths, as they change with time, may conflict with each other, the fundamental basis of these stories is what makes them timeless."])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Aadin",
            args!["Sometimes, different versions of the same myth or legend arise."],
        )?;
        ctx.next()?;
        ctx.lines_as("Aadin", args!["Although myths, as they change with time, may conflict with each other, the fundamental basis of these stories is what makes them timeless."])?;
        return ctx.close();
    }
    Ok(())
}

pub fn researcher_g3(ctx: &Ctx) -> Script {
    let mut l_count_sl_1 = 0;
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 2 {
            ctx.var("god_sl_1").set(Val::from(31))?;
        }
        if ctx.var("god_sl_1").get()? == 31
            || ctx.var("god_sl_1").get()? == 42
            || ctx.var("god_sl_1").get()? == 13
            || ctx.var("god_sl_1").get()? == 24
        {
            if ctx.var("god_sl_2").get()? == 0 {
                ctx.lines_as("Kurdt", args!["Ah, you must be", "the new assistant.", "Pleased to meet you."])?;
                ctx.next()?;
                ctx.lines_as("Kurdt", args!["In all honesty, I was expecting someone with a bit more experience in the field of research, but it's understandable."])?;
                ctx.next()?;
                ctx.lines_as("Kurdt", args!["After all, we're not in the position to expect professional assistance. In any case, I believe you'll bring a new perspective to our work."])?;
                ctx.next()?;
                ctx.lines_as("Kurdt", args!["I've almost completed my project and don't need too much in the way of manpower. However, the last task I need finished is difficult."])?;
                ctx.next()?;
                ctx.lines_as("Kurdt", args!["I was thinking of using my own funds to hire an outside agency, but there's no need to do that now that you're here~"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kurdt",
                    args!["What I'd like to ask you to do is to gather materials so that I can create the 'Spirit of Fish.'"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kurdt",
                    args!["I'm not sure if I can actually produce it or not, but we can at least give this experiment a try."],
                )?;
                ctx.next()?;
                ctx.lines_as("Kurdt", args!["Basically, you and collect things like 'Fish Scales' or Fish Tails,' basically anything you can obtain from marine life."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kurdt",
                    args![
                        "I don't need a",
                        "ridiculous amount of one",
                        "kind of material, I'd say about 10 of one kind of each item should work. Give or take 5..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Kurdt", args!["I should mention that I also require a good variety of marine related materials. You can't conduct an experiment without different objects to test, right?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kurdt",
                    args!["Anyway, I hope you don't run into too much trouble finding what I need for this experiment. Good luck~"],
                )?;
                ctx.var("god_sl_2").set(Val::from(1))?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 1 {
                ctx.lines_as(
                    "Kurdt",
                    args!["Ah, you've returned.", "Okay, let me check", "what you brought to me. Hmm..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kurdt",
                    args![
                        "Ooh...",
                        "I could use these.",
                        "And these too. Oh!",
                        "You adventurers carry",
                        "a lot of things, don't you?"
                    ],
                )?;
                if ctx.items().count(918)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(950)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(951)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(956)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(959)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(960)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(961)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(962)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(963)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(964)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(965)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(966)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(7013)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1054)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1053)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1052)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1051)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1050)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1024)? > 8 {
                    l_count_sl_1 += 1;
                }
                if ctx.items().count(1023)? > 8 {
                    l_count_sl_1 += 1;
                }
                ctx.next()?;
                if l_count_sl_1 > 14 {
                    ctx.lines_as(
                        "Kurdt",
                        args![
                            "Ah....!",
                            "The materials you have should be enough for me to conduct my experiment. Thank you so much."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kurdt",
                        args![
                            "With these, I can continue my research without any problem. I'm pleasantly surprised with the job you've done."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kurdt",
                        args!["You really were", "such a great help.", "Thank you so much!", "Hahaha....!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kurdt",
                        args![
                            "Now, there may be",
                            "other researchers",
                            "who might need your help in completing their projects. Why don't you ask around to see if",
                            "they need anything?"
                        ],
                    )?;
                    if ctx.items().count(918)? > 8 {
                        ctx.items().take(918, 9)?;
                    }
                    if ctx.items().count(950)? > 8 {
                        ctx.items().take(950, 9)?;
                    }
                    if ctx.items().count(951)? > 8 {
                        ctx.items().take(951, 9)?;
                    }
                    if ctx.items().count(956)? > 8 {
                        ctx.items().take(956, 9)?;
                    }
                    if ctx.items().count(959)? > 8 {
                        ctx.items().take(959, 9)?;
                    }
                    if ctx.items().count(960)? > 8 {
                        ctx.items().take(960, 9)?;
                    }
                    if ctx.items().count(961)? > 8 {
                        ctx.items().take(961, 9)?;
                    }
                    if ctx.items().count(962)? > 8 {
                        ctx.items().take(962, 9)?;
                    }
                    if ctx.items().count(963)? > 8 {
                        ctx.items().take(963, 9)?;
                    }
                    if ctx.items().count(964)? > 8 {
                        ctx.items().take(964, 9)?;
                    }
                    if ctx.items().count(965)? > 8 {
                        ctx.items().take(965, 9)?;
                    }
                    if ctx.items().count(966)? > 8 {
                        ctx.items().take(966, 9)?;
                    }
                    if ctx.items().count(1023)? > 8 {
                        ctx.items().take(1023, 9)?;
                    }
                    if ctx.items().count(1024)? > 8 {
                        ctx.items().take(1024, 9)?;
                    }
                    if ctx.items().count(1050)? > 8 {
                        ctx.items().take(1050, 9)?;
                    }
                    if ctx.items().count(1051)? > 8 {
                        ctx.items().take(1051, 9)?;
                    }
                    if ctx.items().count(1052)? > 8 {
                        ctx.items().take(1052, 9)?;
                    }
                    if ctx.items().count(1053)? > 8 {
                        ctx.items().take(1053, 9)?;
                    }
                    if ctx.items().count(1054)? > 8 {
                        ctx.items().take(1054, 9)?;
                    }
                    if ctx.items().count(7013)? > 8 {
                        ctx.items().take(7013, 9)?;
                    }
                    ctx.var("god_sl_2").set(Val::from(0))?;
                    if ctx.var("god_sl_1").get()? == 31 {
                        ctx.var("god_sl_1").set(Val::from(32))?;
                    }
                    if ctx.var("god_sl_1").get()? == 42 {
                        ctx.var("god_sl_1").set(Val::from(43))?;
                    }
                    if ctx.var("god_sl_1").get()? == 13 {
                        ctx.var("god_sl_1").set(Val::from(14))?;
                    }
                    if ctx.var("god_sl_1").get()? == 24 {
                        ctx.var("god_sl_1").set(Val::from(50))?;
                    }
                    return ctx.close();
                } else {
                    ctx.lines_as(
                        "Kurdt",
                        args![
                            "Still, you don't seem to have enough materials for me to",
                            "complete my research",
                            "Remember, I need",
                            "a huge variety."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Kurdt", args!["I'm sorry for the hassle, but do you think you could go out and collect things so that I can attempt to recreate the", "'Spirit of Fish?'"])?;
                    return ctx.close();
                }
            }
        } else {
            ctx.lines_as(
                "Kurdt",
                args!["Hmmm....", "Funds and", "manpower are", "important for", "research to progress."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kurdt",
                args!["But no amount of financing or staffing can replace the value of enthusiasm and passion in conducting research."],
            )?;
            ctx.next()?;
            ctx.lines_as("Kurdt", args!["Some financiers are so ignorant that they think that if they throw more money and people into a project, the secrets of science will be unlocked that much faster."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kurdt",
                args!["But if the people doing your research are unmotivated and uninspired, you'll never get anywhere."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Kurdt",
            args!["Hmmm....", "Funds and", "manpower are", "important for", "research to progress."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kurdt",
            args!["But no amount of financing or staffing can replace the value of enthusiasm and passion in conducting research."],
        )?;
        ctx.next()?;
        ctx.lines_as("Kurdt", args!["Some financiers are so ignorant that they think that if they throw more money and people into a project, the secrets of science will be unlocked that much faster."])?;
        ctx.next()?;
        ctx.lines_as(
            "Kurdt",
            args!["But if the people doing your research are unmotivated and uninspired, you'll never get anywhere."],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn researcher_g4(ctx: &Ctx) -> Script {
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 2 {
            ctx.var("god_sl_1").set(Val::from(41))?;
        }
        if ctx.var("god_sl_1").get()? == 41
            || ctx.var("god_sl_1").get()? == 12
            || ctx.var("god_sl_1").get()? == 23
            || ctx.var("god_sl_1").get()? == 34
        {
            if ctx.var("god_sl_2").get()? == 0 {
                ctx.lines_as(
                    "Pavel",
                    args!["^333333*Sniff sniff*...", "*Achoo!*^000000 Huh?", "Who are you?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["Oh, sorry.", "You must be the new assistan--^333333*Aaachoo!^000000 Ehhm, ^333333*Sniff sniff*^000000 I'm sorry. As you can see, I'm feeling a little bit under the weather."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Pavel",
                    args!["It's good to know that you're help to help me. But ^333333*Achoooo! Sniff Sniff*^000000"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Pavel",
                    args![
                        "Anyway, go look at --^333333*Aaachooo!*^000000 that ^333333*Achoooo!*^000000 thing... ^333333*Sniff sniff*^000000"
                    ],
                )?;
                ctx.var("god_sl_2").set(Val::from(1))?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 1 {
                ctx.lines_as(
                    "Pavel",
                    args!["I told you to go look at-- ^333333*Aaachhoooo!*^000000 that ^333333*Aaachoooo! Achoo! glhk glhk*^000000"],
                )?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 2 {
                ctx.lines_as(
                    "Pavel",
                    args![
                        "^333333*Cough cough*",
                        "*Sniff sniff...*^000000",
                        "So did you take a look at it? Yeah, I don't think you'd understand, even if you saw it. ^333333*glhk glhk*^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["^333333*AAAAACHHHOOO!*^000000", "That slab of stone, no wait, ^333333*Achoo!*^000000 the letters engraved on that thing are ancien--^333333*Achoo!*^000000 language is no longer used nowadays."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Pavel",
                    args!["I can't ^333333*glhk glhk*^000000 tell what ancient languag-- ^333333*Achoo!*^000000 it is, so... Hmm..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["Considering ^333333*Achoo! Sniff sniff*^000000 the material used for the slab isn't ^333333*Sniff*^000000 ordinary stone, there must be some kind of device installed in it? ^333333*Cough*^000000"])?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["Not only that ^333333*Sniff sniff* letters are not just engraved. ^333333*AAAACHOO!*^000000 Something covers the ^333333*Cough*^000000 surface."])?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["I must ^333333*Cough cough*^000000 remove the device for so that I can ^333333*Cough cough*^000000 investigate this further ^333333*Sniff*^000000 Butjust look at me. I'm a wreck. I can't even ^333333*AAAACHOO!*^000000 ...talk."])?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["^333333*Sniff sniff*^000000", "So I hope you don't mind ^333333*Achoo!*^000000 helping me remove ^333333*Cough*^000000 the device. Thank you so much. ^333333*Sniff*^000000"])?;
                ctx.var("god_sl_2").set(Val::from(3))?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 3 {
                ctx.lines_as(
                    "Pavel",
                    args![
                        "^333333Aaaaaccccchhhhoooo...!*^000000",
                        "Um, what I meant to say w--^333333AAAACHOOO!*^000000",
                        "^333333*Sniff sniff...*^000000"
                    ],
                )?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 4 {
                ctx.lines_as(
                    "Pavel",
                    args![
                        "^333333*Cough*^000000",
                        "Oh... ^333333*Sniff*^000000",
                        "Now I see...",
                        "^333333*glhk glhk*^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Pavel", args!["I was~ ^333333*Achoo!*^000000 right, ^333333*Achoo!*^000000 Something was installed in the sl-^333333*Cough*^000000 slab. I couldn't look at it myself because of this damn ^333333*Cooooough c-c-cough*^000000 flu."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Pavel",
                    args![
                        "Great job~",
                        "You did some ^333333*Sniff*^000000 good work. Thank you so much for helping me. ^333333*Achoo! Achoo!*^000000"
                    ],
                )?;
                ctx.var("god_sl_2").set(Val::from(0))?;
                if ctx.var("god_sl_1").get()? == 41 {
                    ctx.var("god_sl_1").set(Val::from(42))?;
                }
                if ctx.var("god_sl_1").get()? == 12 {
                    ctx.var("god_sl_1").set(Val::from(13))?;
                }
                if ctx.var("god_sl_1").get()? == 23 {
                    ctx.var("god_sl_1").set(Val::from(24))?;
                }
                if ctx.var("god_sl_1").get()? == 34 {
                    ctx.var("god_sl_1").set(Val::from(50))?;
                }
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "Pavel",
                args!["Whether or not we can actually detect or sense spirits isn't as important as first proving their existence."],
            )?;
            ctx.close_window()?;
        }
    } else {
        ctx.lines_as(
            "Pavel",
            args!["Whether or not we can actually detect or sense spirits isn't as important as first proving their existence."],
        )?;
        ctx.close_window()?;
    }
    Ok(())
}

pub fn slab_g(ctx: &Ctx) -> Script {
    let l_input = Val::from(0);
    let mut l_input_s = Val::from("");
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 41
            || ctx.var("god_sl_1").get()? == 12
            || ctx.var("god_sl_1").get()? == 23
            || ctx.var("god_sl_1").get()? == 34
        {
            if ctx.var("god_sl_2").get()? == 1 || ctx.var("god_sl_2").get()? == 2 {
                ctx.lines(args![
                    "....whgks ^ff00ffdirdnl^000000sjs wkrdjswhgks whdtnfb",
                    "............djswhgks wkdusdnlrnlfn",
                    ".......whsjs rhf tk dmqsjs rj ehddjfn",
                    "wkdusgks ^ff00fftkaryf^000000dnl durjfflrhsjs wkrdjswhgks ......",
                    "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                    "^ff00ffghswka^000000gks ........fusjs tmld........."
                ])?;
                ctx.var("god_sl_2").set(Val::from(2))?;
                ctx.next()?;
                ctx.mes("^3355FFThere is some writing engraved on this slab of stone. Since you can't read it and the stone looks weathered with time, it seems to be written in an ancient language.^000000")?;
                ctx.next()?;
                ctx.mes("^3355FFUpon touching the slab's surface, you feel that it is surprisingly warm. Perhaps there is more to this slab than meets the eye.^000000")?;
                return ctx.close();
            } else if ctx.var("god_sl_2").get()? == 3 {
                ctx.lines(args![
                    "....whgks ^ff00ffdirdnl^000000sjs wkrdjswhgks whdtnfn",
                    "............djswhgks wkdusdnlrnlfn",
                    ".......whsjs rhf tk dmqsjs rj ehddjfn",
                    "wkdusgks ^ff00fftkaryf^000000dnl durjfflrhsjs wkrdjswhgks ......",
                    "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                    "^ff00ffghswka^000000gks.........fusjs tmld........."
                ])?;
                ctx.next()?;
                ctx.mes("^3355FFUpon taking a closer look at the stone slab, you see that there are small devices in the crevices of each engraved letter in one of the words.")?;
                ctx.next()?;
                ctx.lines(args![
                    "....whgks ^ff00ffdirdnl^000000sjs wkrdjswhgks whdtnfb",
                    "............djswhgks wkdusdnlrnlfn",
                    ".......whsjs rhf tk dmqsjs rj ehddjfn",
                    "wkdusgks ^ff00fftkaryf^000000dnl durjfflrhsjs wkrdjswhgks ......",
                    "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                    "^ff00ffghswka^000000gks ........fusjs tmld........."
                ])?;
                ctx.next()?;
                let (input, _) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s == "dirdnl" {
                    ctx.lines(args!["Zap~", "^3355FFAs you press the word '^ff0000dirdnl^3355FF,' the slab emitted a strange beeping noise but nothing else happened. Perhaps you must press some of the other words.^000000"])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "....whgks ^ff00ffdirdnl^000000sjs wkrdjswhgks whdtnfb",
                        "............djswhgks wkdusdnlrnlfn",
                        ".......whsjs rhf tk dmqsjs rj ehddjfn",
                        "wkdusgks ^ff00fftkaryf^000000dnl durjfflrhsjs wkrdjswhgks ......",
                        "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                        "^ff00ffghswka^000000gks ........fusjs tmld........."
                    ])?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        "^3355FFNothing happened.",
                        "Perhaps that word",
                        "is not on the slab.^000000"
                    ])?;
                    return ctx.close();
                }
                let (input, _) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s == "tkaryf" {
                    ctx.lines(args![
                        "^3355FFYou press the word '^ff0000tkaryf^3355FF,' and another beep was emitted from",
                        "the slab.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "....whgks ^ff00ffdirdnl^000000sjs wkrdjswhgks whdtnfb",
                        "............djswhgks wkdusdnlrnlfn",
                        ".......whsjs rhf tk dmqsjs rj ehddjfn",
                        "wkdusgks ^ff00fftkaryf^000000dnl durjfflrhsjs wkrdjswhgks ......",
                        "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                        "^ff00ffghswka^000000gks ........fusjs tmld........."
                    ])?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        "^3355FFNothing happened.",
                        "Perhaps that word",
                        "is not on the slab.^000000"
                    ])?;
                    return ctx.close();
                }
                let (input, _) = runtime::input_text(ctx, None, None)?;
                l_input_s = input;
                if l_input_s == "ghswka" {
                    ctx.mes("^3355FFYou press on the word '^ff0000ghswka^3355FF,' resulting in an affirmative beep. Nothing has still happened, so perhaps you must press another word.^000000")?;
                    ctx.next()?;
                    ctx.lines(args![
                        "....whgks ^ff00ffdirdnl^000000sjs wkrdjswhgks whdtnfb",
                        "............djswhgks wkdusdnlrnlfn",
                        ".......whsjs rhf tk dmqsjs rj ehddjfn",
                        "wkdusgks ^ff00fftkaryf^000000dnl durjfflrhsjs wkrdjswhgks ......",
                        "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                        "^ff00ffghswka^000000gks ........fusjs tmld........."
                    ])?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        ((Val::from("You touched a word ^ff0000") + l_input.clone()) + Val::from("^000000, nothing happened.")),
                        "You decided to think up something different."
                    ])?;
                    return ctx.close();
                }
                ctx.mes("^666666*Eeeeeeeeee~*^000000")?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFThe slab began",
                    "to vibrate and the",
                    "engraved letters slowly",
                    "disappeared, revealing",
                    "a new set of words underneath.^000000"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "wkrdjswhgks dirdnlsjs wkrdjswhgks whdtnfh",
                    "wkrdjswhgks dldgnsdjs wkrdjswhgks wkdusdnlrnlfn",
                    "thfdh duTsjswhsjs rhf tk dmqsjs rj ehddjfn",
                    "wkdusgks tkaryfdnl durjfflrhsjs wkrdjswhgks tkatnfu",
                    "wkrdjswhgks shfh wkrdjswhgks wkdus wkrdjswhgks dldgns",
                    "ghswkagks tkatnfusjs tmldaldgks akrnl"
                ])?;
                ctx.next()?;
                ctx.lines(args![
                    "^3355FFYou still don't",
                    "understand what is",
                    "written on the slab.^000000"
                ])?;
                ctx.var("god_sl_2").set(Val::from(4))?;
                ctx.next()?;
                ctx.mes("After a while, the original engraving on the slab re-appeared, once again concealing the writing underneath. It seems you can only reveal the hidden letters temporarily.")?;
                return ctx.close();
            } else {
                ctx.lines(args![
                    "....whgks dirdnlsjs wkrdjswhgks whdtnfn",
                    "............djswhgks wkdusdnlrnlfn",
                    ".......whsjs rhf tk dmqsjs rj ehddjfn",
                    "wkdusgks tkaryfdnl durjfflrhsjs wkrdjswhgks ......",
                    "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                    "ghswkagks .........fusjs tmld........."
                ])?;
                ctx.next()?;
                ctx.mes("^3355FFThis slab of stone has been weathered with time, and its surface is engraved with writing from a strange language.^000000")?;
                ctx.next()?;
                ctx.mes("^3355FFStrangely enough, the surface of the stone is emitting heat.^000000")?;
                return ctx.close();
            }
        } else {
            ctx.lines(args![
                "....whgks dirdnlsjs wkrdjswhgks whdtnfn",
                "............djswhgks wkdusdnlrnlfn",
                ".......whsjs rhf tk dmqsjs rj ehddjfn",
                "wkdusgks tkaryfdnl durjfflrhsjs wkrdjswhgks ......",
                "wkrdjswhgks shfh wkrdjswhgks wkdus wkr...",
                "ghswkagks .........fusjs tmld........."
            ])?;
            ctx.next()?;
            ctx.mes("^3355FFThere is some writing engraved on this slab of stone. Since you can't read it and the stone looks weathered with time, it seems to be written in an ancient language.^000000")?;
            ctx.next()?;
            ctx.mes("^3355FFUpon touching the slab's surface, you feel that it is surprisingly warm. Perhaps there is more to this slab than meets the eye.^000000")?;
            return ctx.close();
        }
    } else {
        ctx.mes("^3355FFThere is some writing engraved on this slab of stone. Since you can't read it and the stone looks weathered with time, it seems to be written in an ancient language.^000000")?;
        return ctx.close();
    }
}

pub fn friar_g5(ctx: &Ctx) -> Script {
    if runtime::op(&ctx.var("$god1").get()?, "<", &ctx.var("$@god_check2").get()?)?.is_true() {
        if ctx.var("god_sl_1").get()? == 21
            || ctx.var("god_sl_1").get()? == 32
            || ctx.var("god_sl_1").get()? == 43
            || ctx.var("god_sl_1").get()? == 14
        {
            if ctx.var("god_sl_2").get()? == 1 || ctx.var("god_sl_2").get()? == 3 {
                ctx.lines_as(
                    "Lania",
                    args!["No matter how old the stories are, you can always find good advice in folk tales and legends."],
                )?;
                ctx.next()?;
                ctx.lines_as("Lania", args!["Despite the passing of time and mankind's progression, certain fundamentals of life are timeless and will never change."])?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Umm...:I see.")])?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Lania",
                            args![
                                "Hmm...?",
                                "Do you need",
                                "anything from me?",
                                "I will try to help",
                                "you as much as I can."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lania",
                            args!["I believe that it's no use having ability or power unless you use that power for the right purposes."],
                        )?;
                        ctx.next()?;
                        match ctx.menu(&["Oh no, that's okay.", "Sleipnir's story."])? {
                            0 => {
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "I must have",
                                        "misunderstood me.",
                                        "However, feel free",
                                        "to ask me for help",
                                        "if you ever need it."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["I consider helping others as special training for my spiritual improvement. I hope that you will act with kindness and generosity towards others as well."])?;
                                return ctx.close();
                            }
                            1 => {
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "Ah, I see.",
                                        "I can tell you what I know about Sleipnir. Let's see now if I can remember everything about it."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args!["Ah yes.", "I remember it now.", "Yes, Sleipnir was the", "name of Odin's stallion."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "In the world of",
                                        "gods, there were",
                                        "three legendary stallions: Sleipnir, Svadilfari and Gullfaxi."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Gullfaxi was owned by Hrungnir, the king of the giants. Svadilfari, the stallion that sired Sleipnir, was also owned by a giant."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Okay, now let's talk about how 'Sleipnir' was born. This story gives us a glimpse of the fallibility of gods and giants."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["In Asgard, the realm of the gods, there was a huge protective rampart that was almost completely destroyed in a recent war."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "However...",
                                        "Gods being gods,",
                                        "they were not eager",
                                        "to rebuild the walls",
                                        "themselves."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "One day, a very",
                                        "skinny man rode on",
                                        "a dark horse and offered to rebuild, and even improve, the rampart protecting Asgard."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args!["His offer was tempting to the gods, who needed every protection from their enemies."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["This man promised to rebuild", "the Asgardian rampart in only 18 months, but in return he wanted the goddess Freya as his wife, as well as ownership of the sun and the moon."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Now, this price was unreasonable for the gods to pay. The Asgardians were all insulted and angered by the stranger's demands. Still, they did want their rampart to be rebuilt."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![".....", "Are you bored?", "If you are, we can", "finish this", "story later."],
                                )?;
                                ctx.next()?;
                                match ctx.menu(&["Yeah, let's finish this later.", "No, please go ahead."])? {
                                    0 => {
                                        ctx.lines_as("Lania", args!["I understand that my story was not that interesting. But I will try my best to entertain you next time. Please travel in safety."])?;
                                        return ctx.close();
                                    }
                                    1 => {
                                        ctx.lines_as("Lania", args!["Oh alright.", "Then let me continue."])?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "The gods were furious with the stranger's ridiculous demands. However, Loki proposed to Odin",
                                        "that they should let the stranger rebuild their defensive walls."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Loki suggested that if the stranger could complete most of the rampart renovation, but later than as agreed, the gods would not be obligated to pay the stranger any price."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Odin decided to follow Loki's advice, and told the stranger that the wall must be repaired in 6 months, instead of 18 months as the stranger had proposed. He could receive help from no man."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["In return, the stranger would be paid as he had asked if he could fulfill those conditions. Since he was just a man, it seemed impossible."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "Hmmm...",
                                        "I know this story is quite long. Did you want to listen some more, or did you need a break?"
                                    ],
                                )?;
                                ctx.next()?;
                                match ctx.menu(&["Let me take down some notes first.", "I need a break!", "No, please go ahead."])? {
                                    0 => {
                                        ctx.lines_as("Lania", args!["Ah....", "I understand this story is quite long. My apologies. Alright, I'll wait for you to finish writing your notes."])?;
                                        ctx.next()?;
                                        ctx.lines(args!["...", "......", "........."])?;
                                        ctx.next()?;
                                        ctx.lines(args!["...", "......", ".........", "............"])?;
                                        ctx.next()?;
                                        ctx.lines(args!["...", "......", ".........", "............", "..............."])?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "...",
                                            "......",
                                            ".........",
                                            "............",
                                            "...............",
                                            ".................."
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as("Lania", args!["May I continue the story now?"])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("Yes, please.")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as("Lania", args!["Okay, now", "where was I?"])?;
                                        ctx.next()?;
                                    }
                                    1 => {
                                        ctx.lines_as("Lania", args!["I understand that the tale of Sleipnir is pretty long. But if you have the patience, I will try to relate this story to you another time. Farewell~"])?;
                                        return ctx.close();
                                    }
                                    2 => {
                                        ctx.lines_as("Lania", args!["Oh good.", "Then please let me continue."])?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                                ctx.lines_as("Lania", args!["Now, the stranger insisted that he be able to use his horse to help him do this reconstruction work. Since the horse was rather plain and simple looking, Odin agreed."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Odin didn't think", "twice about his", "request, and regretted this decision upon observing the stranger's first day of rebuilding the rampart."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Strangely enough, the stranger, using his horse to carry the materials, rebuilt the rampart at a speed that was inhuman."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["This upset the gods, since the stranger was actually a mighty giant in disguise. However, this did not violate their agreement,", "so the gods could do nothing."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Although the stranger's skills were great, his horse greatly sped up the rampart reconstruction. This was the legendary magical horse, Svadilfari."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["The gods grew more anxious as the wall would soon be completed before the alloted six months were up. The Asgardians soon grew very angry with Loki."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Despite this, Loki remained calm and assured the gods that he had a plan. The gods distrusted his craftiness, and grew desperate as the stranger's work speedily progressed."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Then, the reconstruction of the rampart suddenly halted. Both the gods and the stranger were equally surprised, as the horse Svadilfari was now missing."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args!["The stranger would", "now need to continue", "his work by himself."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "Hmmm...",
                                        "Are you still listening? If you're tired, I understand and we can",
                                        "stop for now."
                                    ],
                                )?;
                                ctx.next()?;
                                match ctx.menu(&["I'm so exhausted!", "No, please go ahead."])? {
                                    0 => {
                                        ctx.lines_as("Lania", args!["I know that this story might seem too long. But if you ever want to sit down and hear it again, I'll be glad to go over it again. Thank you for listening, adventurer."])?;
                                        return ctx.close();
                                    }
                                    1 => {
                                        ctx.lines_as("Lania", args!["Oh alright,", "I'll go on then."])?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                                ctx.lines_as(
                                    "Lania",
                                    args!["In their curiosity, the gods decided to investigate Savdilfari's disappearance. "],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "Surprisingly,",
                                        "they found that",
                                        "Svadilfari fell in love with a beautiful mare, which was why",
                                        "it did not work on the rampart."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["So when the 6 months passed, the stranger almost finished rebuilding the Asgardian walls. Only the castle gate was left untouched."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["The gods were happy to tell the stranger that they could not pay him what he requested, since he did not complete the rampart as originally agreed."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["In his rage, the stranger accused the gods of tricking him so that they would not have to pay him with the sun, the moon and the beautiful goddess Freya. Then, he revealed his true identity."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["He was a", "terrible frost giant,", "or Hrimthurs as they say."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        ".....",
                                        "Are you bored?",
                                        "If you are, we can",
                                        "talk about this",
                                        "later you know."
                                    ],
                                )?;
                                ctx.next()?;
                                match ctx.menu(&["I'm so bored~", "No, please go ahead."])? {
                                    0 => {
                                        ctx.lines_as("Lania", args!["I see. Well, stories are meant to be enjoyed and I suppose you're not in the mood to sit and listen."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Lania", args!["But if you ever do want to hear about Sleipnir, feel free to visit me once again. Have a good day~"])?;
                                        return ctx.close();
                                    }
                                    1 => {
                                        ctx.lines_as(
                                            "Lania",
                                            args![
                                                "Oh, I'm glad~",
                                                "Some people would",
                                                "actually be sleeping",
                                                "after listening so long~"
                                            ],
                                        )?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "Anyway...",
                                        "The stranger turned out",
                                        "to be a giant training in",
                                        "stone masonry. Although he was invading Asgard, the gods didn't worry."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Thor quickly killed the giant with his mighty hammer, Mjolnir, by hurling it at the giant's head, which exploded into a thousand pieces."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args!["As for the horse, Svadilfari, it was nowhere to be found for", "a long, long time."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![".....", "Are you bored?", "If you are, let's talk", "about this later."],
                                )?;
                                ctx.next()?;
                                match ctx.menu(&["Let me take down some notes.", "See you later.", "No, please go ahead."])? {
                                    0 => {
                                        ctx.lines_as(
                                            "Lania",
                                            args!["Ah....", "I see that you really want to keep track of every important detail!"],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Lania", args!["My apologies, I'll wait for you to complete your notes then."])?;
                                        ctx.next()?;
                                        ctx.lines(args!["...", "......", "........."])?;
                                        ctx.next()?;
                                        ctx.lines(args!["...", "......", ".........", "............"])?;
                                        ctx.next()?;
                                        ctx.lines(args!["...", "......", ".........", "............", "..............."])?;
                                        ctx.next()?;
                                        ctx.lines(args![
                                            "...",
                                            "......",
                                            ".........",
                                            "............",
                                            "...............",
                                            ".................."
                                        ])?;
                                        ctx.next()?;
                                        ctx.lines_as("Lania", args!["May I continue the story now?"])?;
                                        ctx.next()?;
                                        let choice = runtime::select_values(ctx, &[Val::from("Yes, please.")])?;
                                        ctx.var("@menu").set(choice)?;
                                        ctx.lines_as("Lania", args!["Okay. Hmm...", "Now where was I?"])?;
                                        ctx.next()?;
                                    }
                                    1 => {
                                        ctx.lines_as("Lania", args!["Oh, I guess my story wasn't really that interesting. But if you want to listen to this story again, please come visit me again."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Lania", args!["Okay then,", "travel safely~"])?;
                                        return ctx.close();
                                    }
                                    2 => {
                                        ctx.lines_as("Lania", args!["Okay. Hmm...", "Now where was I?"])?;
                                        ctx.next()?;
                                    }
                                    _ => {}
                                }
                                ctx.lines_as("Lania", args!["After a while, Loki, who looked tattered and worn out, brought a strange looking pony with eight legs to Asgard."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args!["Loki introduced the pony as 'Sleipnir' and presented it to", "Odin as a present."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["In actuality, Loki transformed into the mare that seduced Svadilfari so that it would no longer work on the rampart."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["So 'Sleipnir' was born from the legendary mare Svadilfari, and the god Loki who transformed himself into a horse."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Now, stories that are spread through word of mouth might have differences in the small details, depending on who's telling the story."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["Still, the fundamental story line should not be different."])?;
                                ctx.next()?;
                                ctx.lines_as("Lania", args!["I don't usually get to practice storytelling, and I'm not too confident of the authenticity of my version of Sleipnir's tale."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "But I hope you will understand that I did my best. It was the most that I could do to help you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lania",
                                    args![
                                        "If you wish to listen to this story again, you're always welcome.",
                                        "Be safe on your adventures~"
                                    ],
                                )?;
                                if ctx.var("god_sl_2").get()? == 1 {
                                    ctx.var("god_sl_2").set(Val::from(2))?;
                                } else if ctx.var("god_sl_2").get()? == 3 {
                                    ctx.var("god_sl_2").set(Val::from(4))?;
                                }
                                return ctx.close();
                            }
                            _ => {}
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Lania",
                            args!["In times of agony, please reflect on your past to help you find a solution."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Lania",
                            args![
                                "Like the old myths and legends, your experiences are stories from which you can learn valuable lessons."
                            ],
                        )?;
                        return ctx.close();
                    }
                }
            } else {
                ctx.lines_as("Lania", args!["I told you everything I know. Hopefully, I was of some help to you. Please treat others with the same generosity and kindness."])?;
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "Lania",
                args!["Training isn't limited to just physical and mental conditioning."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lania",
                args!["The way you lead your life and interact with other people can also be considered part of your training."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Lania",
                args!["Everyone will learn and benefit differently, depending on how they live their day to day lives."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Lania",
            args!["Training isn't limited to just physical and mental conditioning."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lania",
            args!["The way you lead your life and interact with other people can also be considered part of your training."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lania",
            args!["Everyone will learn and benefit differently, depending on how they live their day to day lives."],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn switch_god0(ctx: &Ctx) -> Script {
    if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
        ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
        ctx.next()?;
        ctx.warp("que_god01", 60, 88)?;
        return ctx.end();
    } else {
        ctx.lines(args![
            "^3355FFThe door is locked.",
            "There is some",
            "sort of security device on the right side. It seems the device needs to detect some kind of",
            "pass before you can enter.^000000"
        ])?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum GodSlW0Step {
    Start,
    OnTouch,
}

fn god_sl_w0_run(ctx: &Ctx, mut step: GodSlW0Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodSlW0Step::Start => {
                step = GodSlW0Step::OnTouch;
                continue 'machine;
            }
            GodSlW0Step::OnTouch => {
                if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
                    ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
                    ctx.next()?;
                    ctx.warp("que_god01", 60, 88)?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn god_sl_w0(ctx: &Ctx) -> Script {
    god_sl_w0_run(ctx, GodSlW0Step::Start, Vec::new()).map(|_| ())
}

pub fn god_sl_w0_ontouch(ctx: &Ctx) -> Script {
    god_sl_w0_run(ctx, GodSlW0Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn switch_god1(ctx: &Ctx) -> Script {
    if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
        ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
        ctx.next()?;
        ctx.warp("que_god01", 62, 119)?;
        return ctx.end();
    } else {
        ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum GodSlW1Step {
    Start,
    OnTouch,
}

fn god_sl_w1_run(ctx: &Ctx, mut step: GodSlW1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodSlW1Step::Start => {
                step = GodSlW1Step::OnTouch;
                continue 'machine;
            }
            GodSlW1Step::OnTouch => {
                if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
                    ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
                    ctx.next()?;
                    ctx.warp("que_god01", 62, 119)?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn god_sl_w1(ctx: &Ctx) -> Script {
    god_sl_w1_run(ctx, GodSlW1Step::Start, Vec::new()).map(|_| ())
}

pub fn god_sl_w1_ontouch(ctx: &Ctx) -> Script {
    god_sl_w1_run(ctx, GodSlW1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn switch_god2(ctx: &Ctx) -> Script {
    if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
        ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
        ctx.next()?;
        ctx.warp("que_god01", 12, 119)?;
        return ctx.end();
    } else {
        ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum GodSlW2Step {
    Start,
    OnTouch,
}

fn god_sl_w2_run(ctx: &Ctx, mut step: GodSlW2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodSlW2Step::Start => {
                step = GodSlW2Step::OnTouch;
                continue 'machine;
            }
            GodSlW2Step::OnTouch => {
                if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
                    ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
                    ctx.next()?;
                    ctx.warp("que_god01", 12, 119)?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn god_sl_w2(ctx: &Ctx) -> Script {
    god_sl_w2_run(ctx, GodSlW2Step::Start, Vec::new()).map(|_| ())
}

pub fn god_sl_w2_ontouch(ctx: &Ctx) -> Script {
    god_sl_w2_run(ctx, GodSlW2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn switch_god3(ctx: &Ctx) -> Script {
    if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
        ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
        ctx.next()?;
        ctx.warp("que_god01", 12, 52)?;
        return ctx.end();
    } else {
        ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum GodSlW3Step {
    Start,
    OnTouch,
}

fn god_sl_w3_run(ctx: &Ctx, mut step: GodSlW3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodSlW3Step::Start => {
                step = GodSlW3Step::OnTouch;
                continue 'machine;
            }
            GodSlW3Step::OnTouch => {
                if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
                    ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
                    ctx.next()?;
                    ctx.warp("que_god01", 12, 52)?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn god_sl_w3(ctx: &Ctx) -> Script {
    god_sl_w3_run(ctx, GodSlW3Step::Start, Vec::new()).map(|_| ())
}

pub fn god_sl_w3_ontouch(ctx: &Ctx) -> Script {
    god_sl_w3_run(ctx, GodSlW3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn switch_god4(ctx: &Ctx) -> Script {
    if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
        ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
        ctx.next()?;
        ctx.warp("que_god01", 50, 52)?;
        return ctx.end();
    } else {
        ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum GodSlW4Step {
    Start,
    OnTouch,
}

fn god_sl_w4_run(ctx: &Ctx, mut step: GodSlW4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GodSlW4Step::Start => {
                step = GodSlW4Step::OnTouch;
                continue 'machine;
            }
            GodSlW4Step::OnTouch => {
                if ctx.var("god_sl_1").get()?.number()? > 1 && ctx.var("god_sl_1").get()?.number()? < 51 {
                    ctx.mes("^3355FFThe door is locked. You slide your temporary pass on the security device on the right side of the door, and the door unlocks.^000000")?;
                    ctx.next()?;
                    ctx.warp("que_god01", 50, 52)?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("^3355FFThe door is locked. There is some sort of security device on the right side. It seems the device needs to detect some kind of pass before you can enter.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn god_sl_w4(ctx: &Ctx) -> Script {
    god_sl_w4_run(ctx, GodSlW4Step::Start, Vec::new()).map(|_| ())
}

pub fn god_sl_w4_ontouch(ctx: &Ctx) -> Script {
    god_sl_w4_run(ctx, GodSlW4Step::OnTouch, Vec::new()).map(|_| ())
}
