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

pub fn mantis_researcher(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60179])? != -1 {
        if ctx.call(Function::CheckQuest, args![60179, constants::HUNTING])? != 2 {
            ctx.lines_as("Mantis Researcher", args!["Have you finished hunting the 50 Mantis?"])?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No", "I want to quit"])? {
                0 => {
                    ctx.lines_as("Mantis Researcher", args!["Hmm, I don't think you've hunted 50 yet."])?;
                    return ctx.close();
                }
                1 => {
                    ctx.lines_as(
                        "Mantis Researcher",
                        args!["Remember, I need help hunting Mantis so go and hunt 50 of them."],
                    )?;
                    return ctx.close();
                }
                2 => {
                    ctx.lines_as(
                        "Mantis Researcher",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as(
                                "Mantis Researcher",
                                args!["Ok then, well come back here if you change your mind."],
                            )?;
                            ctx.quests().erase(60179)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Mantis Researcher",
                                args!["Please kill 50 Mantis in order to make this a Place were people can go to and gather Experience."],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        ctx.lines_as("Mantis Researcher", args!["Amazing, you did that with speed."])?;
        ctx.call(Function::GetExperience, args![18818, 7527])?;
        ctx.quests().erase(60179)?;
        return ctx.close();
    }
    if ctx.player().base_level()? > 34 && ctx.player().base_level()? < 71 {
        ctx.lines_as(
            "Mantis Researcher",
            args![
                "Oh great!",
                "You are here to help me right?",
                "If not, i don't know what to do with the Monsters in this Place!"
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["Sure", "No"])? {
            0 => {
                ctx.lines_as("Mantis Researcher", args!["Great! I knew the moment i saw you that i can count on you!", "I need you to reduce the Amount of Monsters which are currently rampaging on this Map!", "There are to many Mantis around, which makes it impossible for the lower level players to gain good experience here."])?;
                ctx.next()?;
                ctx.lines_as("Mantis Researcher", args!["So, would you still like to help me?"])?;
                ctx.next()?;
                match ctx.menu(&["Sure", "Cancel"])? {
                    0 => {
                        if ctx.player().base_level()? > 70 {
                            ctx.lines_as(
                                "Mantis Researcher",
                                args![
                                    "Thank you for your great help with the Mantis!",
                                    "Currently the Situation is stable and i'm not in need of your help anymore to kill them."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Mantis Researcher",
                            args!["Kill 50 of those Mantis and let me know when you're done. I will reward you for your efforts."],
                        )?;
                        ctx.quests().start(60179)?;
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as("Mantis Researcher", args!["If you change your mind, please come back."])?;
                        return ctx.close();
                    }
                    _ => {}
                }
            }
            1 => {
                ctx.lines_as("Mantis Researcher", args!["If you change your mind, please come back."])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as("Mantis Researcher", args!["Oh boy, Oh boy!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Mantis Researcher",
        args!["Sorry I can't talk right now I am in a world of pain here, these darn Mantis are going to kill me."],
    )?;
    ctx.close()
}
