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

pub fn dragon_hunter(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckQuest, args![60182])? != -1 {
        if ctx.call(Function::CheckQuest, args![60182, constants::HUNTING])? != 2 {
            ctx.lines_as("Dragon Hunter", args!["Have you finished hunting the 50 Zhu Po Long?"])?;
            ctx.next()?;
            'b1: {
                let subject1 = ctx.menu(&["Yes", "No", "I want to quit"])?;
                let mut matched1 = false;
                if !matched1 && subject1 == 0 {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as("Dragon Hunter", args!["Hmm, I don't think you've hunted 50 yet."])?;
                    return ctx.close();
                }
                if !matched1 && subject1 == 1 {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Dragon Hunter",
                        args!["Remember, I need help hunting Zhu Po Long so go and hunt 50 of them."],
                    )?;
                    return ctx.close();
                }
                if !matched1 && subject1 == 2 {
                    matched1 = true;
                }
                if matched1 {
                    ctx.lines_as(
                        "Dragon Hunter",
                        args![
                            "Are you sure that you want to stop hunting?",
                            "Any progress that you've made will be erased"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.lines_as("Dragon Hunter", args!["Ok then, well come back here if you change your mind."])?;
                            ctx.quests().erase(60182)?;
                            return ctx.close();
                        }
                        1 => {
                            ctx.lines_as(
                                "Dragon Hunter",
                                args![
                                    "Please kill 50 Zhu Po Long in order to make this a Place were people can go to and gather Experience."
                                ],
                            )?;
                            return ctx.close();
                        }
                        _ => {}
                    }
                }
            }
        }
        ctx.lines_as("Dragon Hunter", args!["Amazing, you did that with speed."])?;
        ctx.call(Function::GetExperience, args![151300, 60520])?;
        ctx.quests().erase(60182)?;
        return ctx.close();
    }
    if ctx.player().base_level()? > 49 && ctx.player().base_level()? < 91 {
        ctx.lines_as(
            "Dragon Hunter",
            args![
                "Oh great!",
                "You are here to help me right?",
                "If not, i don't know what to do with the Monsters in this Place!"
            ],
        )?;
        ctx.next()?;
        'b3: {
            let subject3 = ctx.menu(&["Sure", "No"])?;
            let mut matched3 = false;
            if !matched3 && subject3 == 0 {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as("Dragon Hunter", args!["Great! I knew the moment i saw you that i can count on you!", "I need you to reduce the Amount of Monsters which are currently rampaging on this Map!", "There are to many Zhu Po Long around, which makes it impossible for the lower level players to gain good experience here."])?;
                ctx.next()?;
                ctx.lines_as("Dragon Hunter", args!["So, would you still like to help me?"])?;
                ctx.next()?;
                match ctx.menu(&["Sure", "Cancel"])? {
                    0 => {
                        if ctx.player().base_level()? > 90 {
                            ctx.lines_as(
                                "Dragon Hunter",
                                args![
                                    "Thank you for your great help with the Zhu Po Long!",
                                    "Currently the Situation is stable and i'm not in need of your help anymore to kill them."
                                ],
                            )?;
                            return ctx.close();
                        }
                        ctx.lines_as(
                            "Dragon Hunter",
                            args!["Kill 50 of those Zhu Po Long and let me know when you're done. I will reward you for your efforts."],
                        )?;
                        ctx.quests().start(60182)?;
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as("Dragon Hunter", args!["If you change your mind, please come back."])?;
                        return ctx.close();
                    }
                    _ => {}
                }
            }
            if !matched3 && subject3 == 1 {
                matched3 = true;
            }
            if matched3 {
                ctx.lines_as("Dragon Hunter", args!["If you change your mind, please come back."])?;
                return ctx.close();
            }
        }
    }
    ctx.lines_as("Dragon Hunter", args!["Oh boy, Oh boy!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Dragon Hunter",
        args!["Sorry I can't talk right now I am in a world of pain here, these darn Zhu Po Long are going to kill me."],
    )?;
    ctx.close()
}
