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

/// One Pupa hunt. `hunt` is the count shown in the dialogue.
fn pupa_hunt_quest(ctx: &Ctx, quest: i32, hunt: &str, exp_grants: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest])? != -1 {
        if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? != 2 {
            ctx.lines_as("Halgus", args!["Have you gotten rid of the Pupa?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as(
                        "Halgus",
                        args![format!("Hmm, I don't think you've gotten rid of {hunt} Pupa yet...")],
                    )?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Halgus",
                        args![format!("Remember, get rid of {hunt} of those Pupa from the field.")],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Halgus",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Halgus", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.call(Function::EraseQuest, args![quest])?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Halgus",
                                args![format!("Remember, get rid of {hunt} of those Pupa from the field.")],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Halgus",
                args![
                    "Oh thank you so much.",
                    "I know that I could've probably done this myself but it's not easy to be standing here all of the time."
                ],
            )?;
            for _ in 0..exp_grants {
                ctx.call(Function::GetExperience, args![385, 30])?;
            }
            ctx.call(Function::EraseQuest, args![quest])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn halgus_pupa_hunt(ctx: &Ctx) -> Script {
    pupa_hunt_quest(ctx, 60110, "50", 1)?;
    pupa_hunt_quest(ctx, 60111, "100", 2)?;
    pupa_hunt_quest(ctx, 60112, "150", 3)?;
    if ctx.player().base_level()? > 1 {
        if ctx.player().base_level()? < 21 {
            ctx.lines_as(
                "Halgus",
                args![
                    "New here are you?",
                    "Well look, I like helping new faces around here.",
                    "And lets be honest, you are not the prettiest looking face I've seen."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Halgus",
                args![
                    "None-the-less my daughter thought it would be fun to gather pupa around this field and it's becoming unsightly.",
                    "Do you mind helping me get rid of some Pupa to clear out this field?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Sure, I don't mind", "No"])? {
                0 => {
                    ctx.lines_as("Halgus", args!["Really? What would you like to do to help?"])?;
                    ctx.next()?;
                    match ctx.menu(&["Gather Items", "Get rid of Pupa", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Halgus",
                                args!["Can you collect ^00CE0025 Chrysalis^000000 and return them to me, I promise to reward you well."],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have Chrysalis", "What did you need?"])? {
                                0 => {
                                    if ctx.items().count(915)? > 24 {
                                        ctx.lines_as(
                                            "Halgus",
                                            args![
                                                "Amazing you found so many that fast.",
                                                "Like I promised, give me the Chrysalis and I will reward you with my wisdom."
                                            ],
                                        )?;
                                        ctx.items().take(915, 25)?;
                                        ctx.call(Function::GetExperience, args![385, 30])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as("Halgus", args!["I need ^00CE0025 Chrysalis^000000 and no less please."])?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as("Halgus", args!["I need ^00CE0025 Chrysalis^000000."])?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        1 => {
                            ctx.lines_as("Halgus", args!["Thank you so much!", "How many would you like to get rid of?"])?;
                            ctx.next()?;
                            match ctx.menu(&["50 Pupa", "100 Pupa", "150 Pupa"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Halgus",
                                        args!["If you can get rid of 50 of those Pupa from the field I will be grateful."],
                                    )?;
                                    ctx.quests().start(60110)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Halgus",
                                        args!["If you can get rid of 100 of those Pupa from the field I will be grateful."],
                                    )?;
                                    ctx.quests().start(60111)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Halgus",
                                        args!["If you can get rid of 150 of those Pupa from the field I will be grateful."],
                                    )?;
                                    ctx.quests().start(60112)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as("Halgus", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Halgus", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as(
                "Halgus",
                args!["You could probably easily help me out, but I want to give charity to those who are not as strong as you."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as("Halgus", args!["Please return to me when you are a little stronger."])?;
        return ctx.close();
    }
    Ok(())
}
