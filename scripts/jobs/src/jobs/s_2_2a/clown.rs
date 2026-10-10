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

pub fn minstrel_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()? == 0 || ctx.var("Upper").get()? != 1 {
        if ctx.rand_range(1, 10)? > 4 {
            ctx.lines_as("Minstrel", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as("Minstrel", args!["Do you want to", "sing a song with me?", "Sha la la la la~"])?;
        return ctx.close();
    }
    if ctx.var("advjob").get()? == constants::JOB_CLOWN
        && ctx.player().class()? == constants::JOB_ARCHER_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Minstrel",
            args![
                "The dreary world",
                "of mortals is in need",
                "of more cheerful song.",
                "Will you bring it to them",
                "and turn the tide in the",
                "battle against evil?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Minstrel", args!["Will you do this", "for Midgard...", "As a Minstrel?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Minstrel",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Minstrel",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_CLOWN])?;
        ctx.var("advjob").set(Val::from(0))?;
        ctx.lines_as(
            "Minstrel",
            args![
                "Congratulations!",
                "As a Minstrel, your",
                "your songs will bring",
                "hope to your allies, and",
                "desperation to your foes."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Minstrel", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Minstrel",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
