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
        ctx.lines_as("Gregor", args!["Oh thank you so much!", "I think that I can rest easily now."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![4000, 2000])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Gregor", args![format!("Have you finished hunting the {asked} Peco Pecos?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Gregor", args![format!("Hmm, I don't think you've hunted {target} yet...")])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Gregor",
                args![format!("Please hurry and kill {target} of those tasty err pesky Peco Pecos.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Gregor",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Gregor", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Gregor",
                        args![format!("Please hurry and kill {target} of those tasty err pesky Peco Pecos.")],
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

pub fn gregor_pecopeco_hunt(ctx: &Ctx) -> Script {
    // The 100 quest's first prompt says 50 in the original script; kept as is.
    for (quest, asked, target, rewards) in [(60101, 50, 50, 1), (60102, 50, 100, 2), (60103, 150, 150, 3)] {
        if ctx.call(Function::CheckQuest, args![quest])? != -1 {
            hunt_quest(ctx, quest, asked, target, rewards)?;
        }
    }
    if ctx.player().base_level()? > 9 {
        if ctx.player().base_level()? < 31 {
            ctx.lines_as(
                "Gregor",
                args![
                    "Peco Pecos are fascinating.",
                    "I am still doing my, uh, research...",
                    "But now it is of a different nature."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Gregor",
                args![
                    "You see I've been uh, ''researching''",
                    "these animals by collecting their",
                    "Bill of Birds but lately they seem",
                    "to have become hostile towards me."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Gregor", args!["Can you do me a favor please?"])?;
            ctx.next()?;
            // The original cases fall through into each other, so they stay in a labelled block.
            'b7: {
                let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("What kind of favor?:No")])?);
                let mut matched7 = false;
                if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Gregor",
                        args!["I am so afraid that these Peco Pecos will hurt me for my, uh, researching activities."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Gregor",
                        args![
                            "Can you help protect me by killing them?",
                            "I think I'll feel safer...",
                            "Please..."
                        ],
                    )?;
                    ctx.next()?;
                    'b8: {
                        let subject8 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Gather Items:Hunt Peco Pecos:Cancel")],
                        )?);
                        let mut matched8 = false;
                        if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                            matched8 = true;
                        }
                        if matched8 {
                            ctx.lines_as(
                                "Gregor",
                                args!["I need some ^FF0000Bill of Birds^000000 for my, uh, research..."],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["I have some Bill of Birds", "What did you need?"])? {
                                0 => {
                                    if ctx.items().count(925)? > 24 {
                                        ctx.lines_as("Gregor", args!["Thank you so much."])?;
                                        ctx.items().take(925, 25)?;
                                        ctx.call(Function::GetExperience, args![4000, 2000])?;
                                        ctx.next()?;
                                        ctx.lines_as("Gregor", args!["This is gonna be delicious!"])?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as("Gregor", args!["It doesn't look like you have enough."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Gregor",
                                        args!["Can you help me out and bring me ^FF000025 Bill of Birds^000000?"],
                                    )?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Gregor",
                                        args!["Can you help me out and bring me ^FF000025 Bill of Birds^000000?"],
                                    )?;
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
                                "Gregor",
                                args![
                                    "Ah, ok I think you can help me out by hunting some Peco Pecos.",
                                    "How many would you like to hunt?"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Peco Pecos", "100 Peco Pecos", "150 Peco Pecos"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Gregor",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60101)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Gregor",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60102)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Gregor",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60103)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                            matched8 = true;
                        }
                        if matched8 {
                            ctx.lines_as("Gregor", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                    }
                }
                if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as("Gregor", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
            }
        } else {
            ctx.lines_as("Gregor", args!["Hmmm... you seem to be too strong to be fighting over here."])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as("Gregor", args!["Hi there."])?;
        ctx.next()?;
        ctx.lines_as("Gregor", args!["Those picky's over there seem to be about your speed."])?;
        return ctx.close();
    }
    Ok(())
}
