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

pub fn great_wizard(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "BLIZZARDRISS",
        args![
            "Hey ! My friend !",
            "I see that you are a mage.",
            "Look into yourself to discover",
            "your hidden abilities !"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "About mage's hidden ability ",
        "About skill ^3355FF' Energy Coat '^000000 ",
        "End conversation",
    ])? {
        0 => {
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "For many years",
                    "I have studied the ancient",
                    "magic's of Geffen.",
                    "Recently, I discovered",
                    "a very good forgotten skill! ! !",
                    "Isn't that fortunate?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "The skill uses your mental",
                    "energy to block attacks against you.",
                    "It is like a magical shield, or armor.",
                    "Only the most special of persons can",
                    "use this amazing skills."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "But the most amazing part is that I",
                    "can use the skill! ! !",
                    "I can use this to protect myself !",
                    "Pu hah hah hah !!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "Ok Ok, don't get too excited.",
                    "Listen carefully to what I can tell you.",
                    "Those who use this spell must tap",
                    "the hidden energies and abilities",
                    "locked within themselves !"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "If you wish to learn this skill, you",
                    "must first have a few items to",
                    "be used during the process."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "three ^3355FFGlass Bead^000000 ",
                    "one ^3355FF1 carat Diamond^000000 ",
                    "five ^3355FFShell^000000 ",
                    "one ^3355FFSolid Shell^000000 ",
                    "Bring me these items."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "Also, one more thing. . .",
                    "You must be sufficiently experienced",
                    "in the magical arts. This means you must",
                    "either be a wizard or mage job level 35+."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "Wizards already have already",
                    "experienced the role of a mage",
                    "and so do not require a job level.",
                    "In any case, one who wishes to",
                    "be trained in this art must be in a",
                    "healthy and strong mental state."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args!["Okay . .", "There is nothing more to say, are you interested?"],
            )?;
            return ctx.close();
        }
        1 => {
            if ctx.call(Function::GetSkillLv, args!["MG_ENERGYCOAT"])? == 1 {
                ctx.lines_as(
                    "BLIZZARDRISS",
                    args![
                        "It seems that you have ",
                        "already mastered this skill.",
                        "Your skill in 'Energy Coat' ",
                        "is evident.",
                        "I am sorry, ",
                        "I have nothing more to teach you ..."
                    ],
                )?;
                return ctx.close();
            }
            if ctx.items().count(746)? > 2
                && ctx.items().count(730)? > 0
                && ctx.items().count(935)? > 4
                && ctx.items().count(943)? > 0
                && (ctx.player().job_level()? > 34
                    || ctx.var("BaseJob").get()? == constants::JOB_WIZARD
                    || ctx.var("BaseJob").get()? == constants::JOB_SAGE)
                && ctx.var("BaseClass").get()? == constants::JOB_MAGE
            {
                ctx.lines_as(
                    "BLIZZARDRISS",
                    args![
                        "Okay, I have received your request.",
                        "I will now awaken your hidden energies . .",
                        ". . . . . .",
                        ". . . . . . . . . . . . .",
                        ". . . . . . . . . . . . . . . . . . . . . . . . . ."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "BLIZZARDRISS",
                    args![
                        "Ancient powers of",
                        "Geffen! I seek the enlightenment",
                        "and honor of your presence. ",
                        "I am humbled in your presence!",
                        " ..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "BLIZZARDRISS",
                    args![
                        "Grant me your power!",
                        "Release your spirit",
                        "Mana Shield! Metal Armor!",
                        ". . . . . . . . . . . .",
                        "ENERGY COAT! ! !"
                    ],
                )?;
                ctx.next()?;
                ctx.items().take(746, 3)?;
                ctx.items().take(730, 1)?;
                ctx.items().take(935, 5)?;
                ctx.items().take(943, 1)?;
                ctx.call(Function::Skill, args!["MG_ENERGYCOAT", 1, constants::SKILL_PERM])?;
                ctx.lines_as(
                    "BLIZZARDRISS",
                    args![
                        ". . . . .",
                        "It is done. . .",
                        "You know have the ",
                        "elite skill of ^3355FF' Energy Coat '^000000 .",
                        "Use it well."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "BLIZZARDRISS",
                    args![
                        "Do not shame our",
                        "class with disgraceful",
                        "use of this or any skill.",
                        "Your new power calls for new responsibility."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "Look!!",
                    "Didn't you listen to my explanation ? !",
                    "You have not prepared fully",
                    "for me to assist you.",
                    "Check that you have all the requirements."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "If you need me to explain",
                    "all this again, then ask me.",
                    "I would be happy to explain again",
                    "if only you would listen. . ."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "BLIZZARDRISS",
                args![
                    "The wise man must have patience !",
                    "Prepare yourself again,",
                    "and return when you are ready."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
