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
        ctx.lines_as("Local Villager", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![250266, 144452])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Local Villager",
        args![format!("Have you finished hunting the {asked} Demon Pungus?")],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as(
                "Local Villager",
                args![format!("Hmm, I don't think you've hunted {target} yet...")],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Local Villager",
                args![format!(
                    "Remember, I need help hunting Demon Pungus so go and hunt {target} of them."
                )],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Local Villager",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Local Villager", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Local Villager", args![format!("Please kill {target} Demon Pungus.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn local_villager_hunt(ctx: &Ctx) -> Script {
    // The 100 quest's first prompt says 50 in the original script; kept as is.
    for (quest, asked, target, rewards) in [(60125, 50, 50, 1), (60126, 50, 100, 2), (60127, 150, 150, 3)] {
        if ctx.call(Function::CheckQuest, args![quest])? != -1 {
            hunt_quest(ctx, quest, asked, target, rewards)?;
        }
    }
    if ctx.player().base_level()? > 59 {
        if ctx.player().base_level()? < 75 {
            ctx.lines_as(
                "Local Villager",
                args!["Hey, you there. Yeah you!", "Do you think you can help me with something?"],
            )?;
            ctx.next()?;
            match ctx.menu(&["Sure", "No"])? {
                0 => {
                    ctx.lines_as(
                        "Local Villager",
                        args!["I am trying to make my way through but I can't seem to get past these Demon Pungus."],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Gather Items", "Hunt Demon Pungus", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Local Villager",
                                args![
                                    "Can you help by collecting ^FF0000Bacillus^000000?",
                                    "I will reward you for helping clear this out for me."
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have Bacillus", "What did you need?"])? {
                                0 => {
                                    if ctx.items().count(7119)? > 49 {
                                        ctx.lines_as("Local Villager", args!["I had my doubts, but you have proven me wrong."])?;
                                        ctx.items().take(7119, 50)?;
                                        ctx.call(Function::GetExperience, args![250266, 144452])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as("Local Villager", args!["It doesn't look like you have enough."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Local Villager",
                                        args!["I need to see that you have gathered ^FF000050 Bacillus^000000, and then I can reward you."],
                                    )?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Local Villager",
                                        args!["I need to see that you have gathered ^FF000050 Bacillus^000000, and then I can reward you."],
                                    )?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Local Villager",
                                args![
                                    "Ah, ok I think you can help me out by hunting some Demon Pungus.",
                                    "How many would you like to hunt?"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Demon Pungus", "100 Demon Pungus", "150 Demon Pungus"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Local Villager",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60125)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Local Villager",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60126)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Local Villager",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60127)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as("Local Villager", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Local Villager", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Local Villager", args!["Um, um. I don't think.."])?;
            ctx.next()?;
            ctx.lines_as("Local Villager", args!["Yeah, sorry I can't talk right now."])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Local Villager",
            args!["Its amazing, truly it is.", "I just wish I could get through this all."],
        )?;
        return ctx.close();
    }
    Ok(())
}
