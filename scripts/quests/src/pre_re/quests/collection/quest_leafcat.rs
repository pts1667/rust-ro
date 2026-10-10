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

pub fn lella_leafcat_hunt(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60143])? != -1 {
        if ctx.call(Function::CheckQuest, args![60143, constants::HUNTING])? != 2 {
            ctx.lines_as("Lella", args!["Have you finished hunting the 50 Leaf Cats?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Lella", args!["Hmm, I don't think you've hunted 50 yet..."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Lella",
                        args!["Remember, I need help hunting Leaf Cats so go and hunt 50 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Lella",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Lella", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60143)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lella", args!["Please kill 50 Leaf Cats."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lella", args!["Amazing, you did that with speed."])?;
            ctx.call(Function::GetExperience, args![25740, 31512])?;
            ctx.quests().erase(60143)?;
            return ctx.close();
        }
    }
    if ctx.call(Function::CheckQuest, args![60144])? != -1 {
        if ctx.call(Function::CheckQuest, args![60144, constants::HUNTING])? != 2 {
            ctx.lines_as("Lella", args!["Have you finished hunting the 50 Leaf Cats?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Lella", args!["Hmm, I don't think you've hunted 100 yet..."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Lella",
                        args!["Remember, I need help hunting Leaf Cats so go and hunt 100 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Lella",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Lella", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60144)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lella", args!["Please kill 100 Leaf Cats."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lella", args!["Amazing, you did that with speed."])?;
            ctx.call(Function::GetExperience, args![25740, 31512])?;
            ctx.call(Function::GetExperience, args![25740, 31512])?;
            ctx.quests().erase(60144)?;
            return ctx.close();
        }
    }
    if ctx.call(Function::CheckQuest, args![60145])? != -1 {
        if ctx.call(Function::CheckQuest, args![60145, constants::HUNTING])? != 2 {
            ctx.lines_as("Lella", args!["Have you finished hunting the 150 Leaf Cats?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Lella", args!["Hmm, I don't think you've hunted 150 yet..."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Lella",
                        args!["Remember, I need help hunting Leaf Cats so go and hunt 150 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Lella",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Lella", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60145)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lella", args!["Please kill 150 Leaf Cats."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lella", args!["Amazing, you did that with speed."])?;
            ctx.call(Function::GetExperience, args![25740, 31512])?;
            ctx.call(Function::GetExperience, args![25740, 31512])?;
            ctx.call(Function::GetExperience, args![25740, 31512])?;
            ctx.quests().erase(60145)?;
            return ctx.close();
        }
    }
    if ctx.player().base_level()? <= 35 {
        ctx.lines_as("Lella", args!["Quickly escape before they get their claws into your sanity!"])?;
        return ctx.close();
    }
    if ctx.player().base_level()? >= 66 {
        ctx.lines_as(
            "Lella",
            args!["Not sure why you are here, but I can't offer you a bounty as there would be no effort in it for you!"],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Lella",
        args![
            "Why hello!",
            "You look like someone who is willing to help someone who is slowly going mad."
        ],
    )?;
    ctx.next()?;
    ctx.mes("[Lella]")?;
    'b7: {
        let subject7 = ctx.menu(&["Sure", "No"])?;
        let mut matched7 = false;
        if !matched7 && subject7 == 0 {
            matched7 = true;
        }
        if matched7 {
            ctx.lines_as("Lella", args!["Thanks! You're a life saver!"])?;
            ctx.next()?;
            'b8: {
                let subject8 = ctx.menu(&["Gather Items", "Hunt Leaf Cats", "Cancel"])?;
                let mut matched8 = false;
                if !matched8 && subject8 == 0 {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines(args![
                        "I really need to gather ^00CE0050 Huge Leafs^000000 for my sanity.",
                        "Do you have any?"
                    ])?;
                    ctx.next()?;
                    match ctx.menu(&["I have 50 Huge Leafs", "What, sorry I was day dreaming"])? {
                        0 => {
                            if ctx.items().count(7198)? > 49 {
                                ctx.lines_as(
                                    "Lella",
                                    args!["Wonderful I can tell it is just a little bit quieter around here!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Lella", args!["The more leafs I collect the quieter it seems around here."])?;
                                ctx.items().take(7198, 50)?;
                                ctx.call(Function::GetExperience, args![25740, 31512])?;
                                return ctx.close();
                            } else {
                                ctx.lines_as("Lella", args!["Meow meow meow meow meow...", "NOOOOOOOOOOOO."])?;
                                return ctx.close();
                            }
                        }
                        1 => {
                            ctx.lines_as("Lella", args!["I need ^00CE0050 Huge Leafs^000000."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                if !matched8 && subject8 == 1 {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as(
                        "Lella",
                        args![
                            "Ah, ok I think you can help me out by hunting some Leaf Cats.",
                            "How many would you like to hunt?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["50 Leaf Cats", "100 Leaf Cats", "150 Leaf Cats"])? {
                        0 => {
                            ctx.lines_as(
                                "Lella",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.quests().start(60143)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Lella",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.quests().start(60144)?;
                            return ctx.close();
                        }
                        2 => {
                            ctx.lines_as(
                                "Lella",
                                args!["Come back when you've finished your task and I will give you a small reward."],
                            )?;
                            ctx.quests().start(60145)?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                if !matched8 && subject8 == 2 {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as("Lella", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
            }
        }
        if !matched7 && subject7 == 1 {
            matched7 = true;
        }
        if matched7 {
            ctx.lines_as("Lella", args!["If you change your mind, please come back."])?;
            return ctx.close();
        }
    }
    Ok(())
}
