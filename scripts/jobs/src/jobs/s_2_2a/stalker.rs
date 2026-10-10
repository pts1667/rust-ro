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

pub fn stalker_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()?.number()? == 0 || ctx.var("Upper").get()?.number()? != 1 {
        let karma = ctx.rand_range(1, 10)?;
        if karma > 4 {
            ctx.lines_as("Stalker", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "Stalker",
            args![
                "Heh...",
                "It's tough",
                "being a hero",
                "and being shady,",
                "untrustworthy,",
                "sneaky..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Stalker",
            args![
                "But when the",
                "going gets rough",
                "my pals know they",
                "can count on me.",
                "I need them and",
                "they need me."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("advjob").get()?.number()? == constants::JOB_STALKER
        && ctx.player().class()? == constants::JOB_THIEF_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "Stalker",
            args![
                "This world needs",
                "more heroes who are",
                "willing to walk the line",
                "between order and lawlessness."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Stalker",
            args![
                "Are you ready",
                "to join the ranks",
                "of the sneakiest of",
                "warriors? Are you ready",
                "to become a Stalker?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "Stalker",
                args!["When you're ready,", "feel free to come back.", "Honor to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "Stalker",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_STALKER])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "Stalker",
            args![
                "Congratulations!",
                "As a Stalker, I hope",
                "you stab the right people",
                "in the back. Banish the",
                "wicked using their own",
                "dastardly methods!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Stalker", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "Stalker",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
