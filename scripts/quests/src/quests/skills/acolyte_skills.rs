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

pub fn cleric(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Acolyte Klift",
        args![
            "Ahh . Brothers ! Does the task of",
            "caring for out lost sheep burden",
            "and tire you ?",
            "I am here to assist you."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "^3355FF' About acolyte's hidden ability '^000000 ..",
        "^3355FF' Holy light '^000000 training",
        "End conversation",
    ])? {
        0 => {
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "Our members of the clergy",
                    "naturally learn a skill as",
                    "as they mature. As they approach ",
                    "their senior years as an acolyte",
                    "this special skill."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "Faced with great danger and",
                    "an endless struggle with evil,",
                    "our brethren are much in need of assistance.",
                    "^3355FF' Holy Light '^000000 is that skill.",
                    "To gain this ability for yourself,",
                    "takes some work."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "To gain the skill, you must find these items.",
                    "^FF33551 Opal^000000",
                    "^FF33551 Crystal Blue^000000",
                    "^FF33551 Rosary^000000 "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "In order to be able to even use this skill,",
                    "one must be have sufficient experience.",
                    "^FF3355 Job Level 30^000000",
                    "is required to learn this skill."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "As for a priest, because of their",
                    "already vast amount of experience,",
                    "they are able to gain this skill at any",
                    "job level."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            if ctx.call(Function::GetSkillLv, args!["AL_HOLYLIGHT"])? == 1 {
                ctx.lines_as(
                    "Acolyte Klift",
                    args![
                        "Brother, you already possess",
                        "the skill of ` Holy Light '.",
                        "You cannot gain a skill you",
                        "already possess ..",
                        "I pray that you are using",
                        "this skill for the work of good . ."
                    ],
                )?;
                return ctx.close();
            }
            if ctx.items().count(727)? > 0
                && ctx.items().count(991)? > 0
                && ctx.items().count(2608)? > 0
                && (ctx.player().job_level()? > 29
                    || ctx.var("BaseJob").get()? == constants::JOB_PRIEST
                    || ctx.var("BaseJob").get()? == constants::JOB_MONK)
                && ctx.var("BaseClass").get()? == constants::JOB_ACOLYTE
            {
                ctx.lines_as(
                    "Acolyte Klift",
                    args![
                        "Your faith has proven worthy",
                        "for you to gain the ' Holy Light ' skill.",
                        "Your skill is adequate",
                        "to use this skill.",
                        "Use it wisely. . ."
                    ],
                )?;
                ctx.next()?;
                ctx.items().take(727, 1)?;
                ctx.items().take(991, 1)?;
                ctx.items().take(2608, 1)?;
                ctx.call(Function::Skill, args!["AL_HOLYLIGHT", 1, constants::SKILL_PERM])?;
                ctx.lines_as(
                    "Acolyte Klift",
                    args![
                        "You now know ' Holy Light '",
                        "May you use this skill only in the",
                        "best conduct . . . . ."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "Oh, it is clear. . .",
                    "You are not yet ready to",
                    "receive the ' Holy Light ' skill."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    "You should listen carefully to what",
                    "is necessary for this skill.",
                    "If you listen closely,",
                    "you may learn what you lack.",
                    "So that you may improve upon yourself."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Acolyte Klift",
                args![
                    ". . . . .",
                    "I understand your zeal.",
                    "You have much time yet to",
                    "practice and gain experience.",
                    "Blessings upon you . . . . ."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
