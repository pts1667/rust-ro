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

pub fn gypsy_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("Gypsy", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Gypsy",
            args![
                "Move left,",
                "move right~!",
                "And step...!",
                "Dancing can be",
                "more than a hobby.",
                "For me, it's a way of life~"
            ],
        )?;
        return ctx.close();
    }

    if ctx.var("advjob").get()? == constants::JOB_GYPSY
        && ctx.player().class()? == constants::JOB_ARCHER_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Gypsy",
            args![
                "The land of Midgard",
                "is in need of talented women",
                "to subtly change the balances",
                "in the battle between good",
                "and evil."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Gypsy", args!["Are you ready", "to take up this role,", "and become a Gypsy?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Gypsy",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Gypsy",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_GYPSY])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "Gypsy",
            args![
                "Congratulations!",
                "As a Gypsy, I know",
                "that your performances",
                "sway the hearts of all",
                "those who will be watching..."
            ],
        )?;
        return ctx.close();
    }

    ctx.lines_as("Gypsy", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Gypsy",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
