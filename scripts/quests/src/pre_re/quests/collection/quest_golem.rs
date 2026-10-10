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

pub fn private_jeremy_hunt(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60140])? != -1 {
        return hunt_quest(ctx, 60140, "50", "50", 1);
    }
    if ctx.call(Function::CheckQuest, args![60141])? != -1 {
        // The intro line says 50 for this quest, as in the original script.
        return hunt_quest(ctx, 60141, "50", "100", 2);
    }
    if ctx.call(Function::CheckQuest, args![60142])? != -1 {
        return hunt_quest(ctx, 60142, "150", "150", 3);
    }
    if ctx.player().base_level()? <= 24 {
        ctx.lines_as("Private Jeremy", args!["It is dangerous here citizen."])?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 61 {
        ctx.lines_as("Private Jeremy", args!["Greetings citizen!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Private Jeremy",
            args!["Please do not interrupt my mission for the Morocc Guard."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Private Jeremy",
        args!["Greetings citizen!", "Say, how would you like to help me?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Sure", "No"])? {
        0 => {
            ctx.lines_as("Private Jeremy", args!["Thanks! I really appreciate it."])?;
            ctx.next()?;
            match ctx.menu(&["Gather Items", "Hunt Golems", "Cancel"])? {
                0 => {
                    ctx.lines_as(
                        "Private Jeremy",
                        args!["The Morocc Guard needs ^FF0000Stone Hearts^000000 for our training regimen."],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["I have Stone Hearts", "What did you need?"])? {
                        0 => {
                            if ctx.items().count(953)? > 24 {
                                ctx.lines_as("Private Jeremy", args!["The Morocc Guard thanks you, citizen."])?;
                                ctx.items().take(953, 25)?;
                                ctx.call(Function::GetExperience, args![14000, 9000])?;
                                return ctx.close();
                            }
                            ctx.lines_as("Private Jeremy", args!["It doesn't look like you have enough."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Private Jeremy",
                                args!["The Morocc Guard needs ^FF000025 Stone Hearts^000000, for our training regimen."],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Private Jeremy",
                                args!["The Morocc Guard needs ^FF000025 Stone Hearts^000000, for our training regimen."],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Private Jeremy",
                        args![
                            "Ah, ok I think you can help me out by hunting some Golems.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["50 Golems", "100 Golems", "150 Golems"])? {
                        0 => {
                            ctx.lines_as(
                                "Private Jeremy",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60140])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Private Jeremy",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60141])?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as(
                                "Private Jeremy",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60142])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as("Private Jeremy", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as("Private Jeremy", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

fn hunt_quest(ctx: &Ctx, quest: i32, intro_count: &str, goal: &str, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Private Jeremy", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![14000, 9000])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Private Jeremy",
        args![format!("Have you finished hunting the {intro_count} Golems?")],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as(
                "Private Jeremy",
                args![format!("Hmm, I don't think you've hunted {goal} yet...")],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Private Jeremy",
                args![format!("Remember, I need help hunting Golems so go and hunt {goal} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Private Jeremy",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Private Jeremy", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Private Jeremy", args![format!("Please kill {goal} Golems.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
