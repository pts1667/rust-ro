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
        ctx.lines_as("Cuir", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![68950, 43300])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Cuir", args![format!("Have you finished hunting the {asked} Alligators?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Cuir", args![format!("Hmm, I don't think you've hunted {target} yet...")])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Cuir",
                args![format!("Remember, I need help hunting Alligators so go and hunt {target} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Cuir",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Cuir", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Cuir", args![format!("Please kill {target} Alligators.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn cuir_gator_hunt(ctx: &Ctx) -> Script {
    // The 100 quest's first prompt says 50 in the original script; kept as is.
    for (quest, asked, target, rewards) in [(60119, 50, 50, 1), (60120, 50, 100, 2), (60121, 150, 150, 3)] {
        if ctx.call(Function::CheckQuest, args![quest])? != -1 {
            hunt_quest(ctx, quest, asked, target, rewards)?;
        }
    }
    if ctx.player().base_level()? > 44 {
        if ctx.player().base_level()? < 81 {
            ctx.lines_as("Cuir", args!["You look like a sturdy adventurer!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Cuir",
                args![
                    "I was sent here to minimize the Alligator population but the task is proving to be quite difficult for just me to complete.",
                    "How'd you like to help me out?"
                ],
            )?;
            ctx.next()?;
            // The original cases fall through into each other, so they stay in a labelled block.
            'b7: {
                let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("Sure:No")])?);
                let mut matched7 = false;
                if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Cuir",
                        args![
                            "You can help me out by either hunting Alligators or by gathering items for me.",
                            "Which would you like to do?"
                        ],
                    )?;
                    ctx.next()?;
                    'b8: {
                        let subject8 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Gather Items:Hunt Alligators:Cancel")],
                        )?);
                        let mut matched8 = false;
                        if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                            matched8 = true;
                        }
                        if matched8 {
                            ctx.lines_as(
                                "Cuir",
                                args!["If you can bring me ^00CC0020 Anolian Skins^000000. I'll help you get stronger."],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have the Anolian Skins", "Gators bite!"])? {
                                0 => {
                                    if ctx.items().count(7003)? > 19 {
                                        ctx.lines_as(
                                            "Cuir",
                                            args![
                                                "Thank you for the ^00CE0020 Anolian Skins^000000!",
                                                "I hope you can continue to help me collect these skins.",
                                                "The armor creators around the world are clamoring for them."
                                            ],
                                        )?;
                                        ctx.items().take(7003, 20)?;
                                        ctx.call(Function::GetExperience, args![68950, 43300])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Cuir",
                                        args!["I know a gator skin when I see it and I don't see ^00CE0020 Anolian Skins^000000."],
                                    )?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as("Cuir", args!["I need ^00CE0020 Anolian Skins^000000."])?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                            matched8 = true;
                        }
                        if matched8 {
                            ctx.lines_as(
                                "Cuir",
                                args![
                                    "Ah, ok I think you can help me out by hunting some Alligators.",
                                    "How many would you like to hunt?"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Alligators", "100 Alligators", "150 Alligators"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Cuir",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60119)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Cuir",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60120)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Cuir",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60121)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                            matched8 = true;
                        }
                        if matched8 {
                            ctx.lines_as("Cuir", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                    }
                }
                if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as("Cuir", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
            }
        } else {
            ctx.lines_as(
                "Cuir",
                args!["You are too powerful for this task, I need the skins in fair condition, not exploded!"],
            )?;
            ctx.next()?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Cuir",
            args![
                "You certainly are brave, but bravery turns to foolishness if you continue going East.",
                "Perhaps when you are stronger you and I can do business."
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}
