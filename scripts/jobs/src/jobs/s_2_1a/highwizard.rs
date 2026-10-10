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

pub fn high_wizard_valkyrie(ctx: &Ctx) -> Script {
    if ctx.var("advjob").get()?.number()? == 0 || ctx.var("Upper").get()?.number()? != 1 {
        let karma = ctx.rand_range(1, 10)?;
        if karma > 4 {
            ctx.lines_as("High Wizard", args!["Congratulations.", "Honor to the warriors!"])?;
            return ctx.close();
        }
        ctx.lines_as(
            "High Wizard",
            args![
                "We High Wizards have",
                "the responsibility of",
                "using our destructive magic",
                "for the right purposes."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "High Wizard",
            args![
                "A lifetime of training",
                "is required before becoming",
                "a High Wizard. Can you imagine",
                "what would happen if our power",
                "was placed in the wrong hands?!"
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("advjob").get()?.number()? == constants::JOB_HIGH_WIZARD
        && ctx.player().class()? == constants::JOB_MAGE_HIGH
        && ctx.player().job_level()? > 39
    {
        ctx.lines_as(
            "High Wizard",
            args![
                "It is time.",
                "And Midgard has",
                "need of those who can",
                "wield the strongest of magic..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("High Wizard", args!["Would you like to", "become a High Wizard?"])?;
        ctx.next()?;
        if ctx.menu(&["No.", "Yes."])? == 0 {
            ctx.lines_as(
                "High Wizard",
                args!["When you're ready,", "feel free to come back.", "Honors to the warriors!"],
            )?;
            return ctx.close();
        }
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as(
                "High Wizard",
                args![
                    "It is still possible for you to learn more skills. Please use",
                    "all of your remaining Skill Points before returning to me."
                ],
            )?;
            return ctx.close();
        }
        ctx.call(Function::JobChange, args![constants::JOB_HIGH_WIZARD])?;
        ctx.var("advjob").set(0)?;
        ctx.lines_as(
            "High Wizard",
            args![
                "Congratulations.",
                "As a High Wizard,",
                "I hope use you use",
                "your powers to bring",
                "peace to the oppressed."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("High Wizard", args!["Welcome", "to Valhalla,", "the Hall of Honor."])?;
    ctx.next()?;
    ctx.lines_as(
        "High Wizard",
        args![
            "Please make",
            "yourself comfortable",
            "while you are here.",
            "Honor to the warriors!"
        ],
    )?;
    ctx.close()
}
