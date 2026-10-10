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

fn coco_hunt(ctx: &Ctx, quest: i32, count: &str, asked_count: &str, rewards: usize) -> Script {
    if ctx.call(Function::CheckQuest, args![quest])? == -1 {
        return Ok(());
    }
    if ctx.call(Function::CheckQuest, args![quest, constants::HUNTING])? == 2 {
        ctx.lines_as("Nutters", args!["Amazing, you did that with speed."])?;
        for _ in 0..rewards {
            ctx.call(Function::GetExperience, args![3600, 3905])?;
        }
        ctx.call(Function::EraseQuest, args![quest])?;
        return ctx.close();
    }
    ctx.lines_as("Nutters", args![format!("Have you finished hunting the {asked_count} Cocos?")])?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No", "I want to quit"])? {
        0 => {
            ctx.lines_as("Nutters", args![format!("Hmm, I don't think you've hunted {count} yet...")])?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Nutters",
                args![format!("Remember, I need help hunting Cocos so go and hunt {count} of them.")],
            )?;
            ctx.close()
        }
        2 => {
            ctx.lines_as(
                "Nutters",
                args![
                    "Are you sure that you want to stop hunting?",
                    "Any progress that you've made will be erased"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.lines_as("Nutters", args!["Ok then, well come back here if you change your mind."])?;
                    ctx.call(Function::EraseQuest, args![quest])?;
                    ctx.close()
                }
                1 => {
                    ctx.lines_as("Nutters", args![format!("Please kill {count} Cocos.")])?;
                    ctx.close()
                }
                _ => Ok(()),
            }
        }
        _ => Ok(()),
    }
}

fn start_hunt(ctx: &Ctx, quest: i32) -> Script {
    ctx.lines_as(
        "Nutters",
        args!["Come back when you've finished your task and I will give you a small reward."],
    )?;
    ctx.call(Function::SetQuest, args![quest])?;
    ctx.close()
}

pub fn nutters_coco_hunt(ctx: &Ctx) -> Script {
    coco_hunt(ctx, 60113, "50", "50", 1)?;
    coco_hunt(ctx, 60114, "100", "50", 2)?; // the original prompt for 100 says 50; kept as is
    coco_hunt(ctx, 60115, "150", "150", 3)?;
    if ctx.player().base_level()? <= 17 {
        ctx.lines_as(
            "Nutters",
            args![
                "You are just too small to fight these things for me.",
                "Can you get a little taller then return?"
            ],
        )?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 60 {
        ctx.lines_as("Nutters", args!["You are much to strong to be fighting these things!"])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Nutters",
        args![
            "I..I just can't seem to find any.",
            "Pesky Coco's have hidden all the Acorns.",
            "Do you think you could help me?"
        ],
    )?;
    ctx.next()?;
    // The cases fall through into each other, so this stays a labelled switch.
    'b7: {
        let subject7 = runtime::select_values(ctx, &[Val::from("Sure:No")])?;
        let mut matched7 = false;
        if !matched7 && subject7 == 1 {
            matched7 = true;
        }
        if matched7 {
            ctx.lines_as(
                "Nutters",
                args!["Oh, that's great! I knew I could count on you, just from looking at you!"],
            )?;
            ctx.next()?;
            'b8: {
                let subject8 = runtime::select_values(ctx, &[Val::from("Gather Items:Hunt Cocos:Cancel")])?;
                let mut matched8 = false;
                if !matched8 && subject8 == 1 {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as(
                        "Nutters",
                        args![
                            "Can you help me find ^00CE0025 Acorns^000000.",
                            "I will reward you with much if you can."
                        ],
                    )?;
                    ctx.next()?;
                    let subject9 = runtime::select_values(ctx, &[Val::from("I have 25 Acorns:Please come again?")])?;
                    match subject9 {
                        1 => {
                            if ctx.items().count(1026)? > 24 {
                                ctx.lines_as(
                                    "Nutters",
                                    args!["Oh great you found out where they were hiding them.", "Gimme Gimme!!!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nutters",
                                    args![
                                        "Oh that's right, sorry I just love roasting Acorns. Mmm.",
                                        "Here is what I promised you."
                                    ],
                                )?;
                                ctx.items().take(1026, 25)?;
                                ctx.call(Function::GetExperience, args![3600, 3905])?;
                                return ctx.close();
                            }
                            ctx.lines_as(
                                "Nutters",
                                args![
                                    "Where are my ^00CE00Acorns^000000?",
                                    "Hey, I'm not playing, you better not hold out on me."
                                ],
                            )?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as("Nutters", args!["I need ^00CE0025 Acorns^000000."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                if !matched8 && subject8 == 2 {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as(
                        "Nutters",
                        args![
                            "Ah, ok I think you can help me out by hunting some Cocos.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("50 Cocos:100 Cocos:150 Cocos")])? {
                        1 => return start_hunt(ctx, 60113),
                        2 => return start_hunt(ctx, 60114),
                        3 => return start_hunt(ctx, 60115),
                        _ => {}
                    }
                }
                if !matched8 && subject8 == 3 {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as("Nutters", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
            }
        }
        if !matched7 && subject7 == 2 {
            matched7 = true;
        }
        if matched7 {
            ctx.lines_as("Nutters", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
    }
    Ok(())
}
