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

pub fn assassin_cross_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("Assassin Cross", args!["Congratulations..."])?;
            ctx.next()?;
            ctx.lines_as("Assassin Cross", args!["..."])?;
            ctx.next()?;
            ctx.lines_as("Assassin Cross", args!["...", "......"])?;
            ctx.next()?;
            ctx.lines_as("Assassin Cross", args!["...", "......", "Honor to", "the warriors."])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Assassin Cross",
            args!["We are the warriors", "of the desert. Nobody", "looks down upon us.", "Nobody..."],
        )?;
        return ctx.close();
    }

    if ctx.var("advjob").get()? == constants::JOB_ASSASSIN_CROSS
        && ctx.player().class()? == constants::JOB_THIEF_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Assassin Cross",
            args!["The time has come.", "The world needs you...", "More than ever."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Assassin Cross",
            args!["I ask that you continue to live in the shadows, but as an even greater Assassin with a new appearance."],
        )?;
        ctx.next()?;
        ctx.lines_as("Assassin Cross", args!["Will you become", "an Assassin Cross?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as("Assassin Cross", args!["When you are", "ready, come back."])?;
            ctx.next()?;
            ctx.lines_as("Assassin Cross", args!["Honor to", "the warriors."])?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Assassin Cross",
                args!["You still haven't", "learned everything", "that you can."],
            )?;
            ctx.next()?;
            ctx.lines_as("Assassin Cross", args!["Use all your", "Skill Points", "and then come back."])?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_ASSASSIN_CROSS])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "Assassin Cross",
            args![
                "Congratulations.",
                "As an Assassin Cross,",
                "I hope that you fight for a brighter future within the darkness."
            ],
        )?;
        return ctx.close();
    }

    ctx.lines_as("Assassin Cross", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Assassin Cross",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors."
        ],
    )?;
    ctx.close()
}
