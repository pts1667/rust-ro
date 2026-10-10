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

pub fn lilla_dryad_hunt(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60131])? != -1 {
        return hunt_quest(ctx, 60131, "50", "50", 1);
    }
    if ctx.call(Function::CheckQuest, args![60132])? != -1 {
        // The intro line says 50 for this quest, as in the original script.
        return hunt_quest(ctx, 60132, "50", "100", 2);
    }
    if ctx.call(Function::CheckQuest, args![60133])? != -1 {
        return hunt_quest(ctx, 60133, "150", "150", 3);
    }
    if ctx.player().base_level()? <= 59 {
        ctx.lines_as(
            "Lilla",
            args!["You are way too tiny to be helping me.", "Thank you for your offer though."],
        )?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 86 {
        ctx.lines_as("Lilla", args!["Sorry but you are a little too old and scary to talk to!"])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Lilla",
        args![
            "Why hello!",
            "You look like someone who is willing to help a lil thing like myself out."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Sure", "No"])? {
        0 => {
            ctx.lines_as("Lilla", args!["Really?!", "That's great!"])?;
            ctx.next()?;
            match ctx.menu(&["Gather Items", "Hunt Dryads", "Cancel"])? {
                0 => {
                    ctx.lines_as(
                        "Lilla",
                        args![
                            "I really need to gather ^00CE0050 Sharp Leafs^000000 for my garden tea party.",
                            "Do you have any?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["I have some Sharp Leafs", "What did you need?"])? {
                        0 => {
                            if ctx.items().count(7100)? > 49 {
                                ctx.lines_as(
                                    "Lilla",
                                    args!["Hey, your a sweet little thing.", "Thanks so much for helping me!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Lilla",
                                    args![
                                        "Oh, I almost forgot. I have something for you ^_^",
                                        "Thank you so much again for your help."
                                    ],
                                )?;
                                ctx.items().take(7100, 50)?;
                                ctx.call(Function::GetExperience, args![262485, 141835])?;
                                return ctx.close();
                            }
                            ctx.lines_as(
                                "Lilla",
                                args!["Why, I may look cute and silly but please don't play games with me."],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lilla", args!["I need ^00CE0050 Sharp Leafs^000000."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Lilla",
                        args![
                            "Ah, ok I think you can help me out by hunting some Dryads.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["50 Dryads", "100 Dryads", "150 Dryads"])? {
                        0 => {
                            ctx.lines_as(
                                "Lilla",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60131])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Lilla",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60132])?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as(
                                "Lilla",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60133])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as("Lilla", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as("Lilla", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

fn hunt_quest(ctx: &Ctx, quest: i32, intro_count: &str, goal: &str, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Lilla", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![262485, 141835])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Lilla", args![format!("Have you finished hunting the {intro_count} Dryads?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Lilla", args![format!("Hmm, I don't think you've hunted {goal} yet...")])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Lilla",
                args![format!("Remember, I need help hunting Dryads so go and hunt {goal} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Lilla",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Lilla", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Lilla", args![format!("Please kill {goal} Dryads.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
