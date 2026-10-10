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

pub fn lemly_frilldora_hunt(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60134])? != -1 {
        if ctx.call(Function::CheckQuest, args![60134, constants::HUNTING])? != 2 {
            ctx.lines_as("Lemly", args!["Have you finished hunting the 50 Frilldora?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Lemly", args!["Hmm, I don't think you've hunted 50 yet..."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Lemly",
                        args!["Remember, I need help hunting Frilldora so go and hunt 50 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Lemly",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Lemly", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60134)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lemly", args!["Please kill 50 Frilldora."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lemly", args!["Amazing, you did that with speed."])?;
            ctx.call(Function::GetExperience, args![30000, 23000])?;
            ctx.quests().erase(60134)?;
            return ctx.close();
        }
    }
    if ctx.call(Function::CheckQuest, args![60135])? != -1 {
        if ctx.call(Function::CheckQuest, args![60135, constants::HUNTING])? != 2 {
            ctx.lines_as("Lemly", args!["Have you finished hunting the 50 Frilldora?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Lemly", args!["Hmm, I don't think you've hunted 100 yet..."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Lemly",
                        args!["Remember, I need help hunting Frilldora so go and hunt 100 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Lemly",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Lemly", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60135)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lemly", args!["Please kill 100 Frilldora."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lemly", args!["Amazing, you did that with speed."])?;
            ctx.call(Function::GetExperience, args![30000, 23000])?;
            ctx.call(Function::GetExperience, args![30000, 23000])?;
            ctx.quests().erase(60135)?;
            return ctx.close();
        }
    }
    if ctx.call(Function::CheckQuest, args![60136])? != -1 {
        if ctx.call(Function::CheckQuest, args![60136, constants::HUNTING])? != 2 {
            ctx.lines_as("Lemly", args!["Have you finished hunting the 150 Frilldora?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Lemly", args!["Hmm, I don't think you've hunted 150 yet..."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Lemly",
                        args!["Remember, I need help hunting Frilldora so go and hunt 150 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Lemly",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Lemly", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60136)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as("Lemly", args!["Please kill 150 Frilldora."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lemly", args!["Amazing, you did that with speed."])?;
            ctx.call(Function::GetExperience, args![30000, 23000])?;
            ctx.call(Function::GetExperience, args![30000, 23000])?;
            ctx.call(Function::GetExperience, args![30000, 23000])?;
            ctx.quests().erase(60136)?;
            return ctx.close();
        }
    }
    if ctx.player().base_level()? > 29 {
        if ctx.player().base_level()? < 66 {
            ctx.lines_as("Lemly", args!["Hey there cutie!"])?;
            ctx.next()?;
            ctx.lines_as("Lemly", args!["Think you could take some time to give me a hand?"])?;
            ctx.next()?;
            match ctx.menu(&["Sure", "No"])? {
                0 => {
                    ctx.lines_as("Lemly", args!["I am looking for lizard ^FF0000Frills^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Lemly",
                        args!["The lizards around here have a natural stealth, a property that the Assassin's Guild wants to study."],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Gather Items", "Hunt Frilldora", "Cancel"])? {
                        0 => match ctx.menu(&["I have some Frills", "What did you need?"])? {
                            0 => {
                                if ctx.items().count(1012)? > 24 {
                                    ctx.lines_as("Lemly", args!["Hey, thank you.", "These are pretty icky though."])?;
                                    ctx.items().take(1012, 25)?;
                                    ctx.call(Function::GetExperience, args![30000, 23000])?;
                                    return ctx.close();
                                }
                                ctx.lines_as("Lemly", args!["Sigh,", "Please, don't waste my time."])?;
                                ctx.next()?;
                                ctx.lines_as("Lemly", args!["I need ^FF000025 Frills^000000 for the Assassin's Guild."])?;
                                return ctx.close();
                            }
                            1 => {
                                ctx.lines_as("Lemly", args!["I need ^FF000025 Frills^000000 for the Assassin's Guild."])?;
                                return ctx.close();
                            }
                            _ => {}
                        },
                        1 => {
                            ctx.lines_as(
                                "Lemly",
                                args![
                                    "Ah, ok I think you can help me out by hunting some Frilldora.",
                                    "How many would you like to hunt?"
                                ],
                            )?;
                            ctx.next()?;
                            match ctx.menu(&["50 Frilldora", "100 Frilldora", "150 Frilldora"])? {
                                0 => {
                                    ctx.lines_as(
                                        "Lemly",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60134)?;
                                    return ctx.close();
                                }
                                1 => {
                                    ctx.lines_as(
                                        "Lemly",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60135)?;
                                    return ctx.close();
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Lemly",
                                        args!["Come back when you've finished your task and I will give you a small reward."],
                                    )?;
                                    ctx.quests().start(60136)?;
                                    return ctx.close();
                                }
                                _ => {}
                            }
                        }
                        2 => {
                            ctx.lines_as("Lemly", args!["If you change your mind, please come back."])?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                1 => {
                    ctx.lines_as("Lemly", args!["If you change your mind, please come back."])?;
                    return ctx.close();
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Lemly", args!["Hey there."])?;
            ctx.next()?;
            ctx.lines_as("Lemly", args!["These lizards are gross."])?;
            ctx.next()?;
            ctx.lines_as("Lemly", args!["I can't believe I am stuck doing this."])?;
            return ctx.close();
        }
    } else {
        ctx.lines_as("Lemly", args!["What are you doing here?", "The desert will kill you here!"])?;
        return ctx.close();
    }
    Ok(())
}
