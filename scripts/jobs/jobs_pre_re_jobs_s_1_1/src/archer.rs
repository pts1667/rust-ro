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

pub fn archer_guildsman_archer(ctx: &Ctx) -> Script {
    if ctx.var("Upper").get()? == 1 {
        if ctx.player().class()? == constants::JOB_NOVICE_HIGH
            && (ctx.var("advjob").get()? == constants::JOB_SNIPER
                || ctx.var("advjob").get()? == constants::JOB_CLOWN
                || ctx.var("advjob").get()? == constants::JOB_GYPSY)
        {
            ctx.lines_as(
                "Archer Guildsman",
                args!["Hey, I know you.", "You took this test", "before, didn't you?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Guildsman",
                args![
                    "Ah, you must have been",
                    "to Valhalla and been reborn.",
                    "Wow, that's so impressive!"
                ],
            )?;
            ctx.next()?;
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.lines_as(
                    "Archer Guildsman",
                    args![
                        "Err...",
                        "You'd better learn all the Basic Skills first before you can become an Archer."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Archer Guildsman", args!["Alright, see you later."])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Archer Guildsman",
                args![
                    "Well then. I don't",
                    "need to say anything else.",
                    "I know you'll make a great Archer..."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Skill, args!["NV_TRICKDEAD", 0, constants::SKILL_PERM])?;
            ctx.call(Function::JobChange, args![constants::JOB_ARCHER_HIGH])?;
            ctx.call(Function::Skill, args!["AC_MAKINGARROW", 1, constants::SKILL_PERM])?;
            ctx.call(Function::Skill, args!["AC_CHARGEARROW", 1, constants::SKILL_PERM])?;
            ctx.lines_as(
                "Archer Guildsman",
                args![
                    "Although there's no special",
                    "reward for you this time, I hope you understand. Take care of yourself."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("Archer Guildsman", args!["Oh...?", "Hey, what are", "you doing here...?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Archer Guildsman",
            args![
                "I can tell that you're not cut out to be an Archer. It sort of feels like you're meant to do",
                "something else..."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Archer Guildsman", args!["Good day. How may I help you?"])?;
    ctx.next()?;
    match ctx.menu(&["I want to be an Archer.", "I need the requirements, please.", "Nothing, thanks."])? {
        0 => {
            if ctx.var("BaseJob").get()? == constants::JOB_ARCHER {
                ctx.lines_as("Archer Guildsman", args!["You've already become an Archer..."])?;
                return ctx.close();
            }
            if ctx.var("BaseJob").get()? != constants::JOB_ARCHER && ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                ctx.lines_as(
                    "Archer Guildsman",
                    args!["Hmm...", "You don't look much like a Novice at all..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Archer Guildsman",
                    args!["Anyway, whatever you are, you can't choose a job as an Archer because you have a job already."],
                )?;
                return ctx.close();
            }
            if ctx.var("job_archer_q").get()? == 0 {
                ctx.lines_as(
                    "Archer Guildsman",
                    args!["Do you want to be an Archer?", "If so, you need to fill out this application form."],
                )?;
                ctx.next()?;
                if ctx.menu(&["Apply.", "Cancel"])? == 0 {
                    ctx.var("job_archer_q").set(Val::from(1))?;
                    ctx.quests().start(1004)?;
                    ctx.lines_as(
                        "Archer Guildsman",
                        args!["Okay, sign here. Alright, um, I'll promote you once you meet the requirements."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Guildsman",
                        args!["If you think you've met them already, we can check that now.", "Are you ready?"],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Yes, I am.", "No, not yet."])? == 0 {
                        ctx.lines_as("Archer Guildsman", args!["Alright, let me check."])?;
                        ctx.next()?;
                    } else {
                        ctx.lines_as(
                            "Archer Guildsman",
                            args!["I understand. Be my guest if you want to look at the requirements."],
                        )?;
                        return ctx.close();
                    }
                } else {
                    ctx.lines_as("Archer Guildsman", args!["Well, alright.", "See you next time."])?;
                    return ctx.close();
                }
            }
            ctx.lines_as(
                "Archer Guildsman",
                args![Val::from("Are you...") + ctx.player().name()? + Val::from("?")],
            )?;
            ctx.next()?;
            if !shared::other_global_functions::f_canchangejob(ctx, vec![])?.is_true() {
                ctx.lines_as(
                    "Archer Guildsman",
                    args!["Well, you're not at the right job level. Please check the requirements again."],
                )?;
                ctx.next()?;
                ctx.lines_as("Archer Guildsman", args!["Your job level must be at least 10, and don't forget you should learn all of the Basic Skills. Once you've done that, come back."])?;
                return ctx.close();
            }
            let mut total2 = 0;
            if ctx.var("job_archer_q").get()? == 1 {
                let s_points = ctx.items().count(1066)? * 5;
                let a_points = ctx.items().count(1067)? * 3;
                let b_points = ctx.items().count(1068)? * 2;
                let c_points = ctx.items().count(1019)?;
                let total = s_points + a_points + b_points + c_points;
                total2 = (a_points + b_points) * 2 + c_points;
                ctx.lines_as("Archer Guildsman", args!["Excellent!", "Now then,", "let's see..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Archer Guildsman",
                    args!["I will appraise the value of the various types of Trunks, needed to produce a Bow, that you've brought."],
                )?;
                ctx.next()?;
                if ctx.items().count(1066)? == 0
                    && ctx.items().count(1067)? == 0
                    && ctx.items().count(1068)? == 0
                    && ctx.items().count(1019)? == 0
                {
                    ctx.lines_as(
                        "Archer Guildsman",
                        args![
                            "Um...",
                            "Unfortunately you didn't bring any of the required items. There's nothing for me to appraise."
                        ],
                    )?;
                    return ctx.close();
                }
                ctx.mes("[Archer Guildsman]")?;
                if ctx.items().count(1066)? != 0 {
                    ctx.lines(args![
                        Val::from(" Grade S : ") + ctx.items().count(1066)? + Val::from(" ea, Grade: ") + s_points + Val::from(" . ")
                    ])?;
                }
                if ctx.items().count(1067)? != 0 {
                    ctx.lines(args![
                        Val::from(" Grade A : ") + ctx.items().count(1067)? + Val::from(" ea, Grade : ") + a_points + Val::from(" . ")
                    ])?;
                }
                if ctx.items().count(1068)? != 0 {
                    ctx.lines(args![
                        Val::from(" Grade B : ") + ctx.items().count(1068)? + Val::from(" ea, Grade : ") + b_points + Val::from(" . ")
                    ])?;
                }
                if ctx.items().count(1019)? != 0 {
                    ctx.lines(args![
                        Val::from(" Grade C : ") + ctx.items().count(1019)? + Val::from(" ea, Grade : ") + c_points + Val::from(" . ")
                    ])?;
                }
                if total < 25 {
                    ctx.lines(args![Val::from("Total Grades: ^FF0000") + total + Val::from("^000000 / 40")])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Guildsman",
                        args!["Less than 25!? You have to get a grade of at least 25! Come on, try harder!"],
                    )?;
                    return ctx.close();
                } else {
                    ctx.lines(args![Val::from("Total Grades: ^0000FF") + total + Val::from("^000000 / 40")])?;
                    ctx.next()?;
                    ctx.mes("[Archer Guildsman]")?;
                    if total > 40 {
                        ctx.lines(args!["Wow! More than 40!", "Excellent! Congratulations!"])?;
                    } else if total > 30 {
                        ctx.lines(args!["More than 30! Nice job!", "Congratulations!"])?;
                    } else {
                        ctx.mes("*Sigh* Well, you just barely passed... Anyway, well done.")?;
                    }
                }
                ctx.next()?;
                ctx.lines_as("Archer Guildsman", args!["I'll transfer these Trunks to our Bow Production Department. Now that you've met the requirements, let me promote you right away!"])?;
                if ctx.items().count(1066)? != 0 {
                    ctx.call(Function::DelItem, args![1066, ctx.items().count(1066)?])?;
                }
                if ctx.items().count(1067)? != 0 {
                    ctx.call(Function::DelItem, args![1067, ctx.items().count(1067)?])?;
                }
                if ctx.items().count(1068)? != 0 {
                    ctx.call(Function::DelItem, args![1068, ctx.items().count(1068)?])?;
                }
                if ctx.items().count(1019)? != 0 {
                    ctx.call(Function::DelItem, args![1019, ctx.items().count(1019)?])?;
                }
            }
            ctx.next()?;
            shared::other_global_functions::job_change(ctx, args![constants::JOB_ARCHER])?;
            shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
            ctx.quests().complete(1004)?;
            ctx.lines_as("Archer Guildsman", args!["Congratulations!", "You are now an Archer!"])?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Guildsman",
                args!["Of course, we expect that you will help contribute towards the future of the Archer Guild with your efforts."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Guildsman",
                args!["Ah, your bow has arrived from the Bow Production Department. Here, take it! It's yours~"],
            )?;
            ctx.items().give(1702, 1)?;
            ctx.call(Function::GetItem, args![1750, total2])?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Guildsman",
                args!["Now, off you go. Hunt with pride, knowing you were trained by one of the best!"],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Archer Guildsman",
                args!["I will explain the requirements for being an Archer."],
            )?;
            if ctx.var("BaseJob").get()? != constants::JOB_NOVICE {
                if ctx.var("BaseJob").get()? == constants::JOB_ARCHER {
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Guildsman",
                        args!["But...", "You're already an Archer. You should know these already..."],
                    )?;
                } else {
                    ctx.next()?;
                    ctx.lines_as(
                        "Archer Guildsman",
                        args!["Wait a second. You've chosen a different job already. You don't need to know this~"],
                    )?;
                }
                ctx.mes("So...Yeah...no real reason to tell you the requirements...")?;
            }
            ctx.next()?;
            ctx.lines_as(
                "Archer Guildsman",
                args!["First of all, you have to the Job Level 9 as a Novice, and know all of the Basic Skills."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Archer Guildsman",
                args!["An Archer needs extremely high concentration and reflexes, so we do not accept those who have little patience."],
            )?;
            ctx.next()?;
            ctx.lines_as("Archer Guildsman", args!["You also have to gather ^FF0000Trunks^000000. There are 4 different types of Trunks, each of differing quality. You'll be given different grades for your Trunks, depending on their quality."])?;
            ctx.next()?;
            ctx.lines_as("Archer Guildsman", args!["In order to become an Archer, you must receive a grade of at least ^0000FF25^000000 points out of 40. You can get Trunks from 'Willow,' the tree. Be careful, though. They can be tough monsters."])?;
            return ctx.close();
        }
        _ => ctx.close(),
    }
}
