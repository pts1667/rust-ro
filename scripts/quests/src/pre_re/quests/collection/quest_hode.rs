#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hunt_hodes_quest(ctx: &Ctx, quest: i32, hunt: i32, asked: i32, experience_grants: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest])? == -1 {
        return Ok(());
    }
    if ctx.call(Function::CheckQuest, args![quest, ctx.constant("HUNTING")?])? == 2 {
        ctx.lines_as("Shone", args!["Amazing, you did that with speed."])?;
        for _ in 0..experience_grants {
            ctx.call(Function::GetExperience, args![15775, 1125])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Shone", args![format!("Have you finished hunting the {asked} Hodes?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Shone", args![format!("Hmm, I don't think you've hunted {hunt} yet...")])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Shone",
                args![format!("Remember, I need help hunting Hodes so go and hunt {hunt} of them.")],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Shone",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Shone", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as("Shone", args![format!("Please kill {hunt} Hodes.")])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn shone_hode_hunt(ctx: &Ctx) -> Script {
    hunt_hodes_quest(ctx, 60104, 50, 50, 1)?;
    // The first question of this quest says 50 in the source script, the rest say 100.
    hunt_hodes_quest(ctx, 60105, 100, 50, 2)?;
    hunt_hodes_quest(ctx, 60106, 150, 150, 3)?;
    if ctx.player().base_level()? <= 24 {
        ctx.lines_as(
            "Shone",
            args![
                "You need some help, but I can't provide that for you.",
                "Come back when you are stronger and have earned much more wisdom."
            ],
        )?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 61 {
        ctx.lines_as("Shone", args!["You are much too strong to be fighting these things!"])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Shone",
        args!["Wait! You can see me?", "Oh well, I haven't yet mastered my skills."],
    )?;
    ctx.next()?;
    ctx.lines_as("Shone", args!["Hey do you think you can help me?"])?;
    ctx.next()?;
    'b7: {
        let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("Sure:No")])?);
        let mut matched7 = false;
        if !matched7 && subject7.loosely_equals(&Val::from(1)) {
            matched7 = true;
        }
        if matched7 {
            ctx.lines_as("Shone", args!["Thanks a lot, pal!"])?;
            ctx.next()?;
            'b8: {
                let subject8 = Val::from(runtime::select_values(ctx, &[Val::from("Gather Items:Hunt Hodes:Cancel")])?);
                let mut matched8 = false;
                if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as(
                        "Shone",
                        args![
                            "I am trying to collect ^00CE0025 Earthworm Peelings^000000.",
                            "I will reward you if you help."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("I have 25 Earthworm Peelings:What was it again?")])? {
                        1 => {
                            if ctx.items().count(1055)? > 24 {
                                ctx.lines(args![
                                    "Thank you for the 25 Earthworm",
                                    "Peelings!",
                                    "Here is that reward I promised you, I hope take this wisdom I have passed on to you and share it with others."
                                ])?;
                                ctx.items().take(1055, 25)?;
                                ctx.call(Function::GetExperience, args![15775, 1125])?;
                                return ctx.close();
                            }
                            ctx.lines_as(
                                "Shone",
                                args!["I don't see any ^00CE00Earthworm Peerlings^000000.", "Are you trying to scam me?"],
                            )?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as("Shone", args!["I need ^00CE0025 Earthworm Peelings^000000."])?;
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
                        "Shone",
                        args![
                            "Ah, ok I think you can help me out by hunting some Hodes.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("50 Hodes:100 Hodes:150 Hodes")])? {
                        // menu choices 1, 2 and 3 start quests 60104, 60105 and 60106
                        choice @ 1..=3 => {
                            ctx.lines_as(
                                "Shone",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.call(Function::SetQuest, args![60103 + choice])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as("Shone", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
            }
        }
        if !matched7 && subject7.loosely_equals(&Val::from(2)) {
            matched7 = true;
        }
        if matched7 {
            ctx.lines_as("Shone", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
    }
    Ok(())
}
