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

pub fn hypnotist_2(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hypnotist",
        args![
            "Greetings, adventurer.",
            "I'm a member of the Hypnotist",
            "Academy sent here to Prontera",
            "to provide Skill Reset services",
            "to certain First Class characters for a really good price: free!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hypnotist",
        args![
            "Although I offer unlimited",
            "skill resets for now, I have",
            "two conditions that must be",
            "fulfilled. First, you must be",
            "lower than ^FF0000Base Level 40^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hypnotist",
        args![
            "Second, you must be a",
            "^FF0000Swordman, Acolyte, Mage,",
            "Thief, Archer, Merchant,",
            "Taekwon Boy, Taekwon Girl, Gunslinger and Ninja^000000",
            "Job character to qualify.",
            "Now, do you have any questions?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Skill Reset?", "I want a Skill Reset", "Nothing"])? {
        0 => {
            ctx.lines_as(
                "Hypnotist",
                args![
                    "Skill Resets allow adventuers",
                    "to redistribute their Skill",
                    "Points if they are unhappy",
                    "with their current skills."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hypnotist",
                args![
                    "Before proceeding with",
                    "a Skill Reset, you must",
                    "reduce all of the weight",
                    "of all carried items on your",
                    "character to 0. You can put",
                    "extra items in Kafra Storage."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hypnotist",
                args![
                    "Ah, it's also important",
                    "to remove your ^FF0000Pushcart^000000",
                    "if you have one equipped.",
                    "Otherwise, hypnosis won't",
                    "work, or will backfire..."
                ],
            )?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Hypnotist",
                args![
                    "Are you sure that you",
                    "want to proceed with",
                    "my ^FF0000Skill Reset^000000 service?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Yes", "Cancel"])? == 0 {
                if ctx.player().base_level()? > 40 {
                    ctx.lines_as(
                        "Hypnotist",
                        args![
                            "I'm sorry, but characters",
                            "with Base Levels higher",
                            "than 40 are ineligible for the",
                            "Skill Reset service I provide."
                        ],
                    )?;
                    return ctx.close();
                }
                if (ctx.player().class()? > 0 && ctx.player().class()? < 7 && ctx.var("Upper").get()? == 0)
                    || ctx.player().class()? == constants::JOB_GUNSLINGER
                    || ctx.player().class()? == constants::JOB_NINJA
                    || ctx.player().class()? == constants::JOB_TAEKWON
                {
                    if ctx.call(Function::CheckCart, vec![])?.is_true() {
                        ctx.lines_as(
                            "Hypnotist",
                            args![
                                "Oh! Please remove your",
                                "Pushcart before proceeding",
                                "with the Skill Reset service.",
                                "Thanks for cooperating~"
                            ],
                        )?;
                        return ctx.close();
                    }
                    if ctx.var("Weight").get()?.is_true() {
                        ctx.lines_as(
                            "Hypnotist",
                            args![
                                "If you're here for my Skill",
                                "Reset service, please",
                                "remember that you can't",
                                "reset your skills until the",
                                "^FF0000weight of your carried items in",
                                "your Inventory is reduced to 0^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Hypnotist",
                            args![
                                "Why don't you place your",
                                "things into the Kafra Storage",
                                "for now? That way, you can",
                                "safely keep all of your goods."
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.lines_as(
                        "Hypnotist",
                        args![
                            "Thank you for using",
                            "my Skill Redistribution",
                            "services. Oh, and best",
                            "of luck to you on your",
                            "travels, adventurer."
                        ],
                    )?;
                    ctx.call(Function::ResetSkills, vec![])?;
                    return ctx.close();
                }
                ctx.lines_as(
                    "Hypnotist",
                    args![
                        "I'm sorry, but your",
                        "Job Class doesn't qualify",
                        "for the Skill Reset service",
                        "that I provide. I can only",
                        "offer Skill Resets to the",
                        "following Jobs..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hypnotist",
                    args![
                        "^FF0000Swordman, Acolyte,",
                        "Mage, Thief, Archer,",
                        "Merchant, Taekwon",
                        "Boy, Taekwon Girl, Gunslinger and Ninja^000000."
                    ],
                )?;
                return ctx.close();
            }
            farewell(ctx)
        }
        2 => farewell(ctx),
        _ => Ok(()),
    }
}

fn farewell(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hypnotist",
        args!["Thank you, and good", "luck on your adventures.", "Please travel in safety~"],
    )?;
    ctx.close()
}
