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

pub fn cellerb(ctx: &Ctx) -> Script {
    ctx.mes("[Staff Idul Fitri]")?;
    if !(ctx.call(Function::GetTime, args![constants::DT_MONTH])? == constants::OCTOBER
        && (ctx.call(Function::GetTime, args![constants::DT_DAYOFMONTH])? == 24
            || ctx.call(Function::GetTime, args![constants::DT_DAYOFMONTH])? == 25))
    {
        ctx.mes("Congratulation! Celebrate Feast Day Of Ramadan Idul Fitri 1427 H.")?;
        ctx.npc().special_effect(constants::EF_SANDMAN)?;
        return ctx.close();
    }
    ctx.lines(args![
        format!("Haii......^FF8800{}^000000!!", ctx.player().name()?),
        "First day of Idul Fitri has arrived.",
        "Congratulation celebrate him.",
        "There is event special today."
    ])?;
    ctx.next()?;
    ctx.lines_as("Staff Idul Fitri", args!["Event today.....^009500Idul Fitri Quest!^000000"])?;
    ctx.next()?;
    ctx.lines_as(
        "Staff Idul Fitri",
        args!["If you interest to follow this event, I will cook it to you."],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Allright. I like that!!"), Val::from("Next time.... Thanks.")])? {
        1 => {
            ctx.lines_as(
                "Staff Idul Fitri",
                args![
                    "I have something that might interest you.",
                    "I need all of the following items:",
                    "^D5A500Ketupat Sayur Ingredient :^000000",
                    "^00B6FF~5 Ketupat, 5 Carrot~,^000000",
                    "^CC6633~5 Sweet Potato, 10 Meat~,^000000",
                    "^000088~2 Green Herb, 5 Stem~.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.mes("[Staff Idul Fitri]")?;
            if ctx.items().count(552)? < 5
                || ctx.items().count(515)? < 5
                || ctx.items().count(516)? < 5
                || ctx.items().count(517)? < 10
                || ctx.items().count(511)? < 2
                || ctx.items().count(905)? < 5
            {
                ctx.lines(args!["You don't have enough items.", "Come back when you have them all."])?;
                return ctx.close();
            }
            ctx.items().take(552, 5)?;
            ctx.items().take(515, 5)?;
            ctx.items().take(516, 2)?;
            ctx.items().take(517, 10)?;
            ctx.items().take(511, 2)?;
            ctx.items().take(905, 5)?;
            ctx.lines(args![
                "I see you already have all the items you need.",
                "Just a moment, please!!"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^009500-Plupping snapping bubbling~^000000",
                "^009500-Clinking clingking~^000000",
                "^009500-Clang clang~^000000"
            ])?;
            ctx.items().give(583, 1)?;
            ctx.next()?;
            ctx.lines_as(
                "Staff Idul Fitri",
                args!["We appreciate your participation in this special event."],
            )?;
            ctx.npc().emotion(constants::ET_THANKS)?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Staff Idul Fitri",
                args!["Oh well, maybe you will participate in tommorow's quest."],
            )?;
            ctx.npc().emotion(constants::ET_SCRATCH)?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
