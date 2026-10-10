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

pub fn li_dokebi_hunt(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60128])? != -1 {
        return hunt_quest(ctx, 60128, "50", "50", 1);
    }
    if ctx.call(Function::CheckQuest, args![60129])? != -1 {
        // The intro line says 50 for this quest, as in the original script.
        return hunt_quest(ctx, 60129, "50", "100", 2);
    }
    if ctx.call(Function::CheckQuest, args![60130])? != -1 {
        return hunt_quest(ctx, 60130, "150", "150", 3);
    }
    if ctx.player().base_level()? <= 34 {
        ctx.lines_as("Li", args!["This place is dangerous!"])?;
        ctx.next()?;
        ctx.lines_as("Li", args!["You should leave quickly!"])?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 71 {
        ctx.lines_as("Li", args!["Hello", "I read fortunes in Payon."])?;
        return ctx.close();
    }
    ctx.lines_as("Li", args!["Hello. DO you think you could help me?"])?;
    ctx.next()?;
    match ctx.menu(&["Sure", "No"])? {
        0 => {
            ctx.lines_as("Li", args!["There are so many Dokebis here...I just don't feel safe."])?;
            ctx.next()?;
            match ctx.menu(&["Gather Items", "Hunt Dokebis", "Cancel"])? {
                0 => {
                    ctx.lines_as(
                        "Li",
                        args![
                            "My fortune teller business needs a good supply of ^FF0000Dokebi Horns^000000 to give my customers good luck."
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["I have some Dokebi Horns", "What did you need?"])? {
                        0 => {
                            if ctx.items().count(1021)? > 49 {
                                ctx.lines_as("Li", args!["Oh, great!", "Thank you for the horns."])?;
                                ctx.items().take(1021, 50)?;
                                ctx.call(Function::GetExperience, args![42000, 36000])?;
                                return ctx.close();
                            }
                            ctx.lines_as("Li", args!["It doesn't look like you have enough."])?;
                            ctx.next()?;
                            ctx.lines_as("Li", args!["I need ^FF000050 Dokebi Horns^000000 for my fortune telling."])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Li",
                                args!["If you bring me", "^FF000050 Dokebi Horns^000000, I can give you some good luck."],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Li",
                        args![
                            "Ah, ok I think you can help me out by hunting some Dokebis.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["50 Dokebis", "100 Dokebis", "150 Dokebis"])? {
                        0 => {
                            ctx.lines_as(
                                "Li",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60128])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Li",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60129])?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as(
                                "Li",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60130])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                2 => {
                    ctx.lines_as("Li", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as("Li", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

fn hunt_quest(ctx: &Ctx, quest: i32, intro_count: &str, goal: &str, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Li", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![42000, 36000])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Li", args![format!("Have you finished hunting the {intro_count} Dokebis?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Li", args![format!("Hmm, I don't think you've hunted {goal} yet...")])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Li",
                args![format!("Remember, I need help hunting Dokebis so go and hunt {goal} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Li",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Li", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Li", args![format!("Please kill {goal} Dokebis.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
