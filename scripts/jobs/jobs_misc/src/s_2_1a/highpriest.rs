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

pub fn high_priest_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()?.number()? == 0 || ctx.var("Upper").get()?.number()? != 1 {
        let karma = ctx.rand_range(1, 10)?;
        if karma > 4 {
            ctx.lines_as("High Priest", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "High Priest",
            args![
                "Through the power",
                "of holiness, may we",
                "find peace, strength",
                "and protection. Deliver",
                "us from the forces of evil..."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("advjob").get()?.number()? == constants::JOB_HIGH_PRIEST
        && ctx.player().class()? == constants::JOB_ACOLYTE_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "High Priest",
            args![
                "Our world is in",
                "need of people of",
                "talent and conviction.",
                "Please continue your",
                "good works as an even",
                "greater hero of holiness..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("High Priest", args!["Would you like", "to become a High Priest?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "High Priest",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "High Priest",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_HIGH_PRIEST])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "High Priest",
            args![
                "Congratulations.",
                "As a High Priest,",
                "I hope you will guide",
                "others upon the path",
                "to holiness..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("High Priest", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "High Priest",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
