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

pub fn deadman(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60173])? != -1 {
        if ctx.call(Function::CheckQuest, args![60173, constants::HUNTING])? != 2 {
            ctx.lines_as("Deadman", args!["Have you finished hunting the 50 Disguise?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Deadman", args!["Hmm, I don't think you've hunted 50 yet."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Deadman",
                        args!["Remember, I need help hunting Disguise so go and hunt 50 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Deadman",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Deadman", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60173)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Deadman",
                                args!["Please kill 50 Disguise in order to make this a Place were people can go to and gather Experience."],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        ctx.lines_as("Deadman", args!["Amazing, you did that with speed."])?;
        ctx.call(Function::GetExperience, args![140600, 95800])?;
        ctx.quests().erase(60173)?;
        return ctx.close();
    }
    if ctx.call(Function::CheckQuest, args![60176])? != -1 {
        if ctx.call(Function::CheckQuest, args![60176, constants::HUNTING])? != 2 {
            ctx.lines_as("Deadman", args!["Have you finished hunting the 50 Loli Ruri?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Deadman", args!["Hmm, I don't think you've hunted 50 yet."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Deadman",
                        args!["Remember, I need help hunting Loli Ruri so go and hunt 50 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Deadman",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Deadman", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60176)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Deadman",
                                args![
                                    "Please kill 50 Loli Ruri in order to make this a Place were people can go to and gather Experience."
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        ctx.lines_as("Deadman", args!["Amazing, you did that with speed."])?;
        ctx.call(Function::GetExperience, args![332000, 239500])?;
        ctx.quests().erase(60176)?;
        return ctx.close();
    }
    if ctx.player().base_level()? > 59 && ctx.player().base_level()? < 99 {
        ctx.lines_as(
            "Deadman",
            args![
                "Oh great!",
                "You are here to help me right?",
                "If not, i don't know what to do with the Monsters in this Place!"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Sure", "No"])? {
            0 => {
                ctx.lines_as("Deadman", args!["Great! I knew the moment i saw you that i can count on you!", "I need you to reduce the Amount of Monsters which are currently rampaging on this Map!", "There are to many Disguise and Loli Ruri around, which makes it impossible for the lower level players to gain good experience here."])?;
                ctx.next()?;
                ctx.lines_as("Deadman", args!["So, which of those Monsters would you like to hunt for me?"])?;
                ctx.next()?;
                match ctx.menu(&["Disguise", "Loli Ruri", "Cancel"])? {
                    0 => {
                        if ctx.player().base_level()? > 90 {
                            ctx.lines_as(
                                "Deadman",
                                args![
                                    "Thank you for your great help with the Disguise!",
                                    "Currently the Situation is stable and i'm not in need of your help anymore to kill them."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Deadman",
                            args!["Kill 50 of those Disguise and let me know when you're done. I will reward you for your efforts."],
                        )?;
                        ctx.quests().start(60173)?;
                        return ctx.close();
                    }
                    1 => {
                        if ctx.player().base_level()? > 98 {
                            ctx.lines_as(
                                "Deadman",
                                args![
                                    "Thank you for your great help with the Loli Ruri!",
                                    "Currently the Situation is stable and i'm not in need of your help anymore to kill them."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Deadman",
                            args!["Kill 50 of those Loli Ruri and let me know when you're done. I will reward you for your efforts."],
                        )?;
                        ctx.quests().start(60176)?;
                        return ctx.close();
                    }
                    2 => {
                        ctx.lines_as("Deadman", args!["If you change your mind, please come back."])?;
                        return ctx.close();
                    }
                    _ => {}
                }
            }
            1 => {
                ctx.lines_as("Deadman", args!["If you change your mind, please come back."])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as("Deadman", args!["Oh boy, Oh boy!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Deadman",
        args!["Sorry I can't talk right now I am in a world of pain here, these darn Disguise and Loli Ruri are going to kill me."],
    )?;
    ctx.close()
}
