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

pub fn f_bardskillyhelle(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0
        || ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000
    {
        ctx.lines(args!["^3355FFHold it right there!", "You're carrying too many items and don't have enough inventory space to receive any rewards. Please make more inventory space available and come back to take this challenge."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(
        Function::Emotion,
        args![constants::ET_HUK, ctx.call(Function::GetCharacterId, args![0])?.is_true()],
    )?;
    ctx.call(Function::Emotion, args![constants::ET_HUK])?;
    ctx.lines_as("Hen Yhelle", args!["Cluck-Cluuuck?", "Cluck cluck cluck!"])?;
    ctx.next()?;
    let found_egg = ctx.call(Function::Rand, args![1, 3])? == 2;
    ctx.call(
        Function::DisableNpc,
        args![Val::from("Yhelle#bard_chick") + runtime::arg(&args, 0, Val::from(0))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        args![Val::from("Yhelle#bard_chick") + runtime::arg(&args, 1, Val::from(0)) + Val::from("::OnEnable")],
    )?;
    ctx.lines(args![
        "^3355FFUpon sensing your",
        "presense, the hen",
        "quickly ran away.^000000"
    ])?;
    if found_egg && ctx.var("qskill_bard").get()?.number()? > 1 && ctx.var("qskill_bard").get()?.number()? < 7 {
        ctx.lines(args![
            "^3355FFYou found ^3333331 Egg^3355FF in the",
            "place where Yhelle the",
            "Hen was roosting.^000000"
        ])?;
        ctx.var("qskill_bard").set(ctx.var("qskill_bard").get()?.number()? + 1)?;
        ctx.items().give(574, 1)?;
    }
    Ok(Val::from(0))
}
