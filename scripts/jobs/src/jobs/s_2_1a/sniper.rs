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

pub fn sniper_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()?.number()? == 0 || ctx.var("Upper").get()?.number()? != 1 {
        let karma = ctx.rand_range(1, 10)?;
        if karma > 4 {
            ctx.lines_as("Sniper", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Sniper",
            args![
                "One shot.",
                "One kill.",
                "It's not so hard",
                "once you develop the",
                "vision for that style",
                "of battling."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("advjob").get()?.number()? == constants::JOB_SNIPER
        && ctx.player().class()? == constants::JOB_ARCHER_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Sniper",
            args![
                "The world is in",
                "need of mighty Bowmen",
                "like you. Are you ready for",
                "the awesome responsibility?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Sniper",
            args!["Are you willing to", "take the next step and", "become a Sniper?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Sniper",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Sniper",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_SNIPER])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "Sniper",
            args![
                "Congratulations!",
                "As a Sniper, I hope",
                "that the minions of evil",
                "will never be safe so",
                "long as they are in",
                "your sight!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Sniper", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Sniper",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
