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

#[derive(Clone, Copy, Debug)]
enum JovovichStep {
    Start,
    SNoDye,
}

fn jovovich_run(ctx: &Ctx, mut step: JovovichStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut dyed = false;
    let mut headpalette = Val::from(0);
    'machine: loop {
        match step {
            JovovichStep::Start => {
                ctx.lines_as("Hairdresser Jovovich", args!["Welcome~!", "How may I help you?"])?;
                ctx.next()?;
                ctx.mes("[Hairdresser Jovovich]")?;
                if ctx.var("Sex").get()? == constants::SEX_MALE {
                    ctx.mes("Oh, no! Your hair is damaged. It seems as if you may need professional treatment. Come sit over here, please. Come.")?;
                } else if ctx.rand(20)? > 11 {
                    ctx.mes("Eh!? Oh my! Oh no no no no! Your hair is sooo damaged! It's not good if you leave your hair like this.")?;
                    ctx.next()?;
                    ctx.lines_as("Hairdresser Jovovich", args!["Would you let me treat your hair? Please?"])?;
                } else {
                    ctx.mes("Wow! Your hair would be perfect once it's dyed~ How about dying your hair for a change?")?;
                }
                ctx.next()?;
                let subject2 = ctx.menu(&["Dye Hair", "Tips and Information", "Cancel"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Hairdresser Jovovich", args!["Yes yes, good choice~", "Well then, this is your chance for a make-over, your chance to blossom in beauty! Choose the color you would like."])?;
                    ctx.next()?;
                    loop {
                        if dyed {
                            ctx.lines_as(
                                "Hairdresser Jovovich",
                                args!["What do you think? Did you want a different color?"],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Yes", "No"])? == 0 {
                                ctx.lines_as("Hairdresser Jovovich", args!["Okay! Choose the color that you would like."])?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as("Hairdresser Jovovich", args!["Hmm, I'm sort of disappointed. I wanted to do a better job. But I promise I'll do it better next time. Please come again~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        match ctx.menu(&[
                            "Red, please.",
                            "Yellow, please.",
                            "Violet, please.",
                            "Orange, please.",
                            "Green, please.",
                            "Blue, please.",
                            "White, please.",
                            "Black, please.",
                            "Actually, I like my hair as it is.",
                        ])? {
                            0 => headpalette = Val::from(8),
                            1 => headpalette = Val::from(1),
                            2 => headpalette = Val::from(2),
                            3 => headpalette = Val::from(3),
                            4 => headpalette = Val::from(4),
                            5 => headpalette = Val::from(5),
                            6 => headpalette = Val::from(6),
                            7 => headpalette = Val::from(7),
                            8 => {
                                if dyed {
                                    ctx.lines_as("Hairdresser Jovovich", args!["You must like your hair color~"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Hairdresser Jovovich",
                                    args!["Eehh~? You're not going to dye your hair? I'm a little sad..."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                        if headpalette.loosely_equals(&ctx.call(Function::GetLook, args![constants::VAR_HEADPALETTE])?) {
                            ctx.lines_as(
                                "Hairdresser Jovovich",
                                args!["Eh? But that's the hair color you already have. Please choose a different color."],
                            )?;
                            ctx.next()?;
                        } else {
                            match headpalette.number()? {
                                1 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![976, "yellow"])?;
                                }
                                2 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![978, "violet"])?;
                                }
                                3 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![980, "orange"])?;
                                }
                                4 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![979, "green"])?;
                                }
                                5 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![981, "blue"])?;
                                }
                                6 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![982, "white"])?;
                                }
                                7 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![983, "black"])?;
                                }
                                8 => {
                                    jovovich_run(ctx, JovovichStep::SNoDye, args![975, "red"])?;
                                }
                                _ => {}
                            }
                            if ctx.player().zeny()? < 1000 {
                                ctx.lines_as(
                                    "Hairdresser Jovovich",
                                    args!["The fee is 1000 zeny. Do you not have enough...?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            match headpalette.number()? {
                                1 => {
                                    ctx.items().take(976, 1)?;
                                }
                                2 => {
                                    ctx.items().take(978, 1)?;
                                }
                                3 => {
                                    ctx.items().take(980, 1)?;
                                }
                                4 => {
                                    ctx.items().take(979, 1)?;
                                }
                                5 => {
                                    ctx.items().take(981, 1)?;
                                }
                                6 => {
                                    ctx.items().take(982, 1)?;
                                }
                                7 => {
                                    ctx.items().take(983, 1)?;
                                }
                                8 => {
                                    ctx.items().take(975, 1)?;
                                }
                                _ => {}
                            }
                            ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
                            ctx.call(Function::SetLook, args![constants::VAR_HEADPALETTE, headpalette.clone()])?;
                            dyed = true;
                        }
                    }
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Hairdresser Jovovich", args!["When you're feeling down , when you get dumped, when you want to impress someone, or even when you just want to stand out..."])?;
                    ctx.next()?;
                    ctx.lines_as("Hairdresser Jovovich", args!["For that special place and time, wouldn't you want a hairstyle of your very own? As long as you have the appropriate dyestuffs, I will make your hair look wonderful."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hairdresser Jovovich",
                        args!["And try not to worry too much about the fee. Acquiring beauty is the same as acquiring everything. Hehe~"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hairdresser Jovovich",
                        args![
                            "* Fees and Information *",
                            "- 1 Dyestuffs item of the color of hair you want.",
                            "- 1000 zeny fee."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Hairdresser Jovovich",
                        args!["Men or Women...", "Everyone has the right and obligation to be beautiful."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = JovovichStep::SNoDye;
                continue 'machine;
            }
            JovovichStep::SNoDye => {
                if ctx.call(Function::CountItem, args![runtime::arg(&args, 0, Val::from(0))])? == 0 {
                    ctx.lines_as(
                        "Hairdresser Jovovich",
                        args![
                            Val::from("Eh?! But you need the item '")
                                + ctx.call(Function::GetItemName, args![runtime::arg(&args, 0, Val::from(0))])?
                                + Val::from("' to dye your hair ")
                                + runtime::arg(&args, 1, Val::from(0))
                                + Val::from("...")
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn jovovich(ctx: &Ctx) -> Script {
    jovovich_run(ctx, JovovichStep::Start, Vec::new()).map(|_| ())
}
