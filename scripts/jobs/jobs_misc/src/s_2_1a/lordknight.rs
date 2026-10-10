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

pub fn lord_knight_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("Lord Knight", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Lord Knight",
            args!["We Knights have an", "awesome responsibility...", "To serve and protect."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Lord Knight",
            args![
                "Even at the cost",
                "of our own lives,",
                "we must safeguard the",
                "well being of our comrades."
            ],
        )?;
        return ctx.close();
    }

    if ctx.var("advjob").get()? == constants::JOB_LORD_KNIGHT
        && ctx.player().class()? == constants::JOB_SWORDMAN_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Lord Knight",
            args![
                "Your time has come!",
                "The world still needs you.",
                "Please continue your life",
                "as a hero with a new appearance."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Lord Knight", args!["Would you like", "to become a Lord Knight?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Lord Knight",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Lord Knight",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_LORD_KNIGHT])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "Lord Knight",
            args![
                "Congratulations!",
                "As a Lord Knight,",
                "I hope that you will be",
                "at the forefront of battle,",
                "and lead your allies to victory!"
            ],
        )?;
        return ctx.close();
    }

    ctx.lines_as("Lord Knight", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Lord Knight",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
