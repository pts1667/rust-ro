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

pub fn champion_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("Champion", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as("Champion", args!["Skill.", "Speed.", "Strength.", "Agility."])?;
        ctx.next()?;
        ctx.lines_as(
            "Champion",
            args![
                "A Champion can",
                "benefit from all",
                "these things. But",
                "one can only master",
                "so much in life..."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("advjob").get()? == constants::JOB_CHAMPION
        && ctx.player().class()? == constants::JOB_ACOLYTE_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Champion",
            args![
                "It's time.",
                "Time for great heroes",
                "to stand up against the",
                "forces of evil which plague",
                "the world of Midgard!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Champion", args!["Would you like", "to become a Champion?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Champion",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Champion",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_CHAMPION])?;
        ctx.var("advjob").set(Val::from(0))?;
        ctx.lines_as(
            "Champion",
            args![
                "Congratulations!",
                "Live as a Champion,",
                "and bring light into",
                "the world through the",
                "strength of your fists."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Champion", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Champion",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
