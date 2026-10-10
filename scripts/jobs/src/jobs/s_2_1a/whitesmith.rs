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

pub fn mastersmith_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("MasterSmith", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "MasterSmith",
            args!["Ah...", "The pinnacle", "of craftsmanship.", "That's the work of", "a MasterSmith."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "MasterSmith",
            args![
                "Once you're the",
                "the best of the best,",
                "you begin forging with",
                "the goal of discovering",
                "new and better ways of",
                "crafting..."
            ],
        )?;
        return ctx.close();
    }

    if ctx.var("advjob").get()? == constants::JOB_WHITESMITH
        && ctx.player().class()? == constants::JOB_MERCHANT_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "MasterSmith",
            args![
                "The time has come!",
                "Our world needs brave,",
                "hard-working adventurers",
                "like you..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("MasterSmith", args!["Would you like to", "become a MasterSmith?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "MasterSmith",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "MasterSmith",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_WHITESMITH])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "MasterSmith",
            args![
                "Congratulations!",
                "As a MasterSmith,",
                "I hope you will forge",
                "a path towards a brighter",
                "future for Midgard."
            ],
        )?;
        return ctx.close();
    }

    ctx.lines_as("MasterSmith", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "MasterSmith",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
