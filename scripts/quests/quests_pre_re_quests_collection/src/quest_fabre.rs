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

/// One Fabre hunt. `header` is the count in the opening line, `hunt` the count in the later lines.
fn fabre_hunt_quest(ctx: &Ctx, quest: i32, header: &str, hunt: &str, exp_grants: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest])? != -1 {
        if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? != 2 {
            ctx.lines_as("Langry", args![format!("Have you finished hunting the {header} Fabres?")])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Langry", args![format!("Hmm, I don't think you've hunted {hunt} yet...")])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Langry",
                        args![format!("Remember, I need help hunting Fabres so go and hunt {hunt} of them.")],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Langry",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Langry", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.call(Function::EraseQuest, args![quest])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Langry", args![format!("Please kill {hunt} Fabres.")])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Langry", args!["Amazing, you did that with speed."])?;
            for _ in 0..exp_grants {
                ctx.call(Function::GetExperience, args![385, 30])?;
            }
            ctx.call(Function::EraseQuest, args![quest])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn langry_fabre_hunt(ctx: &Ctx) -> Script {
    fabre_hunt_quest(ctx, 60107, "50", "50", 1)?;
    // As shipped: 60108 opens with "50" and grants experience twice, 60109 three times.
    fabre_hunt_quest(ctx, 60108, "50", "100", 2)?;
    fabre_hunt_quest(ctx, 60109, "150", "150", 3)?;
    if ctx.player().base_level()? > 1 {
        if ctx.player().base_level()? < 21 {
            ctx.lines_as("Langry", args!["Do you think you can help me?", "Please?"])?;
            ctx.next()?;
            match ctx.menu(&["Sure", "No"])? {
                0 => {
                    ctx.lines_as("Langry", args!["Really?!", "That's great!"])?;
                    ctx.next()?;
                    match ctx.menu(&["Gather Items", "Hunt Fabres", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Langry",
                                args![
                                    "I need to collect ^0000CE25 Fluff^000000 to",
                                    "complete this community service project.",
                                    "You know what? I can even reward you a little for helping me."
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have some Fluff", "What was that again?"])? {
                                0 => {
                                    if ctx.items().count(914)? > 24 {
                                        ctx.lines_as(
                                            "Langry",
                                            args![
                                                "Thank you for the Fluff. This helps me out greatly.",
                                                "Oh and here you go, this should help you get stronger, faster!"
                                            ],
                                        )?;
                                        ctx.items().take(914, 25)?;
                                        ctx.call(Function::GetExperience, args![385, 30])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Langry",
                                        args!["I see no Fluff, wait yea I do it's the lies coming from your mouth."],
                                    )?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as("Langry", args!["I need ^00CE0025 Fluff^000000."])?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Langry",
                                args![
                                    "Ah, ok I think you can help me out by hunting some Fabres.",
                                    "How many would you like to hunt?"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Fabres", "100 Fabres", "150 Fabres"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Langry",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60107)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Langry",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60108)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Langry",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60109)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as("Langry", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Langry", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Langry",
                args![
                    "Hello, my name is Langry.",
                    "I have to complete this community service, but I am just too lazy."
                ],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Langry",
            args!["You may just be the most unlikely person ever, recycle a few Fabres and come back."],
        )?;
        return ctx.close();
    }
    Ok(())
}
