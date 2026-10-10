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

pub fn scholar_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("Scholar", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Scholar",
            args![
                "It takes a lifetime...",
                "Literally a lifetime",
                "to amass the knowledge",
                "necessary to become",
                "a Scholar..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Scholar",
            args![
                "It's overwhelming.",
                "The more you learn, the",
                "more you discover what",
                "else you don't know.",
                "There's no end to the",
                "process of learning..."
            ],
        )?;
        return ctx.close();
    }

    if ctx.var("advjob").get()? == constants::JOB_PROFESSOR
        && ctx.player().class()? == constants::JOB_MAGE_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Scholar",
            args![
                "Midgard doesn't",
                "have enough Scholars to",
                "help usher in a new age",
                "of prosperity. The",
                "world needs you..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Scholar",
            args![
                "Will you take this",
                "awesome responsibility?",
                "Will you serve Midgard",
                "as a Scholar?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Scholar",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Scholar",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_PROFESSOR])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "Scholar",
            args![
                "Congratulations!",
                "As a Professor, I hope",
                "that you will take an",
                "active part in bringing",
                "the light of knowledge",
                "where there is darkness."
            ],
        )?;
        return ctx.close();
    }

    ctx.lines_as("Scholar", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Scholar",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
