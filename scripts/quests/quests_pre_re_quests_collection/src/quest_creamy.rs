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

fn laertes_creamy_hunt_quest(ctx: &Ctx, quest: i32, intro_count: &str, goal_count: &str, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Laertes", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![2950, 1125])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Laertes",
        args![format!("Have you finished hunting the {intro_count} Creamys?")],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as(
                "Laertes",
                args![format!("Hmm, I don't think you've hunted {goal_count} yet...")],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Laertes",
                args![format!(
                    "Remember, I need help hunting Creamys so go and hunt {goal_count} of them."
                )],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Laertes",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Laertes", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Laertes", args![format!("Please kill {goal_count} Creamys.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn laertes_creamy_hunt(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60122])? != -1 {
        return laertes_creamy_hunt_quest(ctx, 60122, "50", "50", 1);
    }
    if ctx.call(Function::CheckQuest, args![60123])? != -1 {
        return laertes_creamy_hunt_quest(ctx, 60123, "50", "100", 2);
    }
    if ctx.call(Function::CheckQuest, args![60124])? != -1 {
        return laertes_creamy_hunt_quest(ctx, 60124, "150", "150", 3);
    }
    if ctx.player().base_level()? <= 14 {
        ctx.lines_as("Laertes", args!["Your level is too low!"])?;
        ctx.next()?;
        ctx.lines_as("Laertes", args!["Go kill more Porings!"])?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 45 {
        ctx.lines_as("Laertes", args!["Good Morning!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Laertes",
            args![
                "I work for an apothecary in Prontera",
                "I come here to collect materials for medicine."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Laertes", args!["Good Morning!", "Perhaps you could help me?"])?;
    ctx.next()?;
    match ctx.menu(&["Sure", "No"])? {
        0 => {
            ctx.lines_as(
                "Laertes",
                args![
                    "I work for an apothecary in Prontera.",
                    "I come here to collect materials for medicine."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Gather Items", "Hunt Creamys", "Cancel"])? {
                0 => {
                    ctx.lines_as(
                        "Laertes",
                        args!["I need to have a good supply of ^FF0000Powder of Butterfly^000000"],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["I have Powder of Butterfly", "What did you need?"])? {
                        0 => {
                            if ctx.items().count(924)? > 24 {
                                ctx.lines_as("Laertes", args!["Hey, thank you again.", "Here you go, as promised."])?;
                                ctx.items().take(924, 25)?;
                                ctx.call(Function::GetExperience, args![2950, 1125])?;
                                return ctx.close();
                            }
                            ctx.lines_as("Laertes", args!["It doesn't look like you have enough."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Laertes",
                                args![
                                    "If you bring me ^FF000025 Powder of Butterfly^000000,",
                                    "I will give you some medical supplies."
                                ],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Laertes",
                                args![
                                    "If you bring me ^FF000025 Powder of Butterfly^000000,",
                                    "I will give you some medical supplies."
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Laertes",
                        args![
                            "Ah, ok I think you can help me out by hunting some Creamys.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["50 Creamys", "100 Creamys", "150 Creamys"])? {
                        0 => {
                            ctx.lines_as(
                                "Laertes",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.quests().start(60122)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Laertes",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.quests().start(60123)?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as(
                                "Laertes",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.quests().start(60124)?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as("Laertes", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as("Laertes", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
