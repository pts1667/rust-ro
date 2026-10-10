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

fn hunt_quest(ctx: &Ctx, quest: i32, asked: i32, target: i32, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Yullo", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![10425, 6272])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Yullo", args![format!("Have you finished hunting the {asked} Caramels?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Yullo", args![format!("Hmm, I don't think you've hunted {target} yet...")])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Yullo",
                args![format!("Remember, I need help hunting Caramels so go and hunt {target} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Yullo",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Yullo", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Yullo", args![format!("Please kill {target} Caramels.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn yullo_caramel_hunt(ctx: &Ctx) -> Script {
    // The 100 quest's first prompt says 50 in the original script; kept as is.
    for (quest, asked, target, rewards) in [(60116, 50, 50, 1), (60117, 50, 100, 2), (60118, 150, 150, 3)] {
        if ctx.call(Function::CheckQuest, args![quest])? != -1 {
            hunt_quest(ctx, quest, asked, target, rewards)?;
        }
    }
    if ctx.player().base_level()? > 23 {
        if ctx.player().base_level()? < 61 {
            ctx.lines_as(
                "Yullo",
                args![
                    "I can't, no more and I mean it.",
                    "Look at me I am serious I can't kill any more of these things."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["What things?", "Ignore."])? {
                0 => {
                    ctx.lines_as(
                        "Yullo",
                        args![
                            "You know what, here is an idea.",
                            "You can gather them and I can give you a reward for your efforts!"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Gather Items", "Hunt Caramels", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Yullo",
                                args![
                                    "Ok, so I need 25 Porcupine",
                                    "Quill's.",
                                    "Please tell me you have some or can",
                                    "help me"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have Porcupine Quills", "Well I don't think I can help."])? {
                                0 => {
                                    if ctx.items().count(1027)? > 24 {
                                        ctx.lines_as(
                                            "Yullo",
                                            args!["Great, you have enough!", "Just like I promised a little reward."],
                                        )?;
                                        ctx.items().take(1027, 25)?;
                                        ctx.call(Function::GetExperience, args![10425, 6272])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Yullo",
                                        args!["Where are the Porcupine Quills?", "You think I wouldn't notice?"],
                                    )?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as("Yullo", args!["I need ^00CE0025 Porcupine Quills^000000."])?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Yullo",
                                args![
                                    "Ah, ok I think you can help me out by hunting some Caramels.",
                                    "How many would you like to hunt?"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Caramels", "100 Caramels", "150 Caramels"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Yullo",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60116)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Yullo",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60117)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Yullo",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60118)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as("Yullo", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Yullo", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Yullo",
                args![
                    "Hello, my name is Yullo.",
                    "Now wait a minute, are you not a little too high of level for this?"
                ],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Yullo",
            args![
                "Hmm, this maybe a little much of a request for you.",
                "Come back when you are taller and stronger."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}
