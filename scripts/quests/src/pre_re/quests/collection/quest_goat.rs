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

fn hunt_quest(ctx: &Ctx, quest: i32, target: i32, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Vegetable Farmer", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![258489, 155155])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Vegetable Farmer",
        args![format!("Have you finished hunting the {target} Goats?")],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as(
                "Vegetable Farmer",
                args![format!("Hmm, I don't think you've hunted {target} yet.")],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Vegetable Farmer",
                args![format!("Remember, I need help hunting Goats so go and hunt {target} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Vegetable Farmer",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as(
                        "Vegetable Farmer",
                        args!["Ok then, well come back here if you change your mind."],
                    )?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Vegetable Farmer",
                        args![format!(
                            "Please kill {target} Goats so I can farm my vegetables without any worries."
                        )],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn vegetable_farmer_goat(ctx: &Ctx) -> Script {
    for (quest, target, rewards) in [(60137, 50, 1), (60138, 100, 2), (60139, 150, 3)] {
        if ctx.call(Function::CheckQuest, args![quest])? != -1 {
            hunt_quest(ctx, quest, target, rewards)?;
        }
    }
    if ctx.player().base_level()? > 69 {
        if ctx.player().base_level()? < 85 {
            ctx.lines_as(
                "Vegetable Farmer",
                args![
                    "Oh great!",
                    "You are here to help me right?",
                    "If not, then I am in a world of hurt."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Sure", "No"])? {
                0 => {
                    ctx.lines_as(
                        "Vegetable Farmer",
                        args!["These Goats keep eating my vegetables.", "Can you do what you can to help me?"],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Gather Items", "Hunt Goats", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Vegetable Farmer",
                                args!["I need ^FF0000Antelope Horns^000000 to show for your hard work, and I will reward you."],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have Antelope Horns", "What did you need?"])? {
                                0 => {
                                    if ctx.items().count(7106)? > 49 {
                                        ctx.lines_as(
                                            "Vegetable Farmer",
                                            args!["Amazing, you did that with speed.", "I am truly grateful."],
                                        )?;
                                        ctx.items().take(7106, 50)?;
                                        ctx.call(Function::GetExperience, args![258489, 155155])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as("Vegetable Farmer", args!["It doesn't look like you have enough."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Vegetable Farmer",
                                        args!["I need to see that you have gathered ^FF000050 Antelope Horns^000000, and then I can reward you."],
                                    )?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Vegetable Farmer",
                                        args!["I need to see that you have gathered ^FF000050 Antelope Horns^000000, and then I can reward you."],
                                    )?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Vegetable Farmer",
                                args!["Thank you.", "How many Goats would you like to hunt?"],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Goats", "100 Goats", "150 Goats"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Vegetable Farmer",
                                        args![
                                            "Kill 50 of those Goats and let me know when you're done. I will reward you for your efforts."
                                        ],
                                    )?;
                                    ctx.quests().start(60137)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Vegetable Farmer",
                                        args![
                                            "Kill 100 of those Goats and let me know when you're done. I will reward you for your efforts."
                                        ],
                                    )?;
                                    ctx.quests().start(60138)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Vegetable Farmer",
                                        args![
                                            "Kill 150 of those Goats and let me know when you're done. I will reward you for your efforts."
                                        ],
                                    )?;
                                    ctx.quests().start(60139)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as("Vegetable Farmer", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Vegetable Farmer", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Vegetable Farmer", args!["Oh boy, Oh boy!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Vegetable Farmer",
                args!["Sorry I can't talk right now I am in a world of hurt here, these darn Goats are going to kill me."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Vegetable Farmer",
            args!["My vegetables, where did they all go?", "Oh, no..no!!!"],
        )?;
        return ctx.close();
    }
    Ok(())
}
