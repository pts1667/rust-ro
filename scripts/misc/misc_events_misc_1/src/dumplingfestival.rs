#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum ExorcistMasterFahaeStep {
    Start,
    LSORRY,
    MFIN,
    MHERB,
    LNOZENY,
}

fn sell_quantity(ctx: &Ctx, price: i32, item: i32) -> Result<ExorcistMasterFahaeStep, Stop> {
    ctx.lines_as(
        "Exorcist Master Fahae",
        args![
            "How many do you require?",
            "You may only buy 5 at one time.",
            format!("Each costs {price}z."),
            "(Type in 0 to cancel)"
        ],
    )?;
    ctx.next()?;
    let (input, _) = runtime::input_number(ctx, None, None)?;
    ctx.var("@input").set(input)?;
    if ctx.var("@input").get()? == 0 {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("@input").get()?.number()? > 5 {
        return Ok(ExorcistMasterFahaeStep::LSORRY);
    }
    if ctx.player().zeny()? < ctx.var("@input").get()?.number()? * price {
        return Ok(ExorcistMasterFahaeStep::LNOZENY);
    }
    ctx.player()
        .set_zeny(ctx.player().zeny()? - price * ctx.var("@input").get()?.number()?)?;
    ctx.call(Function::GetItem, args![item, ctx.var("@input").get()?])?;
    ctx.lines_as(
        "Exorcist Master Fahae",
        args!["Here you go, I hope you may succeed in my quest."],
    )?;
    ctx.close_window()?;
    Err(Stop::End)
}

fn exorcist_master_fahae_run(ctx: &Ctx, mut step: ExorcistMasterFahaeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ExorcistMasterFahaeStep::Start => {
                ctx.lines_as(
                    "Exorcist Master Fahae",
                    args![
                        "Greetings young warrior, I'm the Exorcist Master Fahae.",
                        "I have been tracking the elusive Bacsojin for some time."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Exorcist Master Fahae",
                    args![
                        "I feel it is time I let another warrior try to complete",
                        "my quest. I have some items to assist you in my quest."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["I wish to buy", "No thanks"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = ExorcistMasterFahaeStep::MFIN;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines_as(
                    "Exorcist Master Fahae",
                    args!["I have 2 items which might become useful to you."],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Realgar Wine", "Exorcize Herb"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = ExorcistMasterFahaeStep::MHERB;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                step = sell_quantity(ctx, 20000, 682)?;
                continue 'machine;
            }
            ExorcistMasterFahaeStep::LSORRY => {
                ctx.lines_as(
                    "Exorcist Master Fahae",
                    args!["You must not be stingy, it is the path of God to be honest."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ExorcistMasterFahaeStep::MFIN => {
                ctx.lines_as(
                    "Exorcist Master Fahae",
                    args!["The path of fully vanquishing evil is far, help me in the way of God."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ExorcistMasterFahaeStep::MHERB => {
                step = sell_quantity(ctx, 10000, 683)?;
                continue 'machine;
            }
            ExorcistMasterFahaeStep::LNOZENY => {
                ctx.lines_as("Exorcist Master Fahae", args!["Money doesn't bring joy to everyone, but we need it to support the temple and myself. Please, try to kill some monsters and take their drops."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn exorcist_master_fahae(ctx: &Ctx) -> Script {
    exorcist_master_fahae_run(ctx, ExorcistMasterFahaeStep::Start, Vec::new()).map(|_| ())
}
