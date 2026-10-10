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

pub fn find_13_2(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_check: Vec<Val> = Vec::new();
    let subject1 = runtime::arg(&args, 0, Val::from(0));
    let mut location = "";
    if subject1 == 0 {
        runtime::local_set(&mut l_check, &Val::from(0), Val::from(2), false);
        runtime::local_set(&mut l_check, &Val::from(1), Val::from(10), false);
        location = "Mt. Mjolnir";
    } else if subject1 == 1 {
        runtime::local_set(&mut l_check, &Val::from(0), Val::from(3), false);
        runtime::local_set(&mut l_check, &Val::from(1), Val::from(5), false);
        location = "Abyss Lake";
    } else if subject1 == 2 {
        runtime::local_set(&mut l_check, &Val::from(0), Val::from(4), false);
        runtime::local_set(&mut l_check, &Val::from(1), Val::from(5), false);
        location = "Thor Volcano";
    }
    if !ctx
        .var("ep13_2_tre")
        .get()?
        .loosely_equals(&runtime::local_get(&l_check, &Val::from(0), false))
        || ctx.call(Function::CountItem, args![6076])? != 1
        || ctx.call(Function::CountItem, args![6077])?.number()? >= runtime::local_get(&l_check, &Val::from(1), false).number()?
    {
        return Err(Stop::End);
    }
    ctx.lines(args![
        Val::from("- Just arrived at ")
            + location
            + ". I think I can find the mineral by using the Mineral Detector in the Portable Toolbox. -"
    ])?;
    ctx.next()?;
    if ctx.menu(&["Check current location.", "Do nothing."])? == 0 {
        ctx.mes("- You pressed the button of the detector. -")?;
        ctx.next()?;
        ctx.lines_as("Mineral Detector", args!["Bee beep -"])?;
        if runtime::arg(&args, 1, Val::from(0)).is_true() {
            return Ok(Val::from(0));
        }
        ctx.next()?;
        ctx.mes("- Nothing's found. -")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("- You decide to do nothing. -")?;
    ctx.close_window()?;
    Err(Stop::End)
}

pub fn jewel_13_2(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let color = runtime::arg(&args, 0, Val::from(0));
    let num = runtime::arg(&args, 1, Val::from(0));
    if ctx.var("ep13_2_rhea").get()?.number()? < 5 {
        ctx.lines(args![
            "- Under a round pile of earth, -",
            Val::from("- there's a ") + color.clone() + " Gem -",
            "- half-buried. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("ep13_2_rhea").get()? != 5 {
        ctx.lines(args![
            "- Small pile of earth -",
            Val::from("- which you dug up the ") + color.clone() + " Gem -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !ctx.call(Function::CheckWeight, args![7575, 2])?.is_true() {
        ctx.mes("Sorry, your inventory is full!")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckQuest, args![Val::from(8240) + num.clone()])? != -1 {
        ctx.lines(args![
            "- Small pile of earth -",
            Val::from("- which you dug up the ") + color.clone() + " Gem -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "- Under a round pile of earth, -",
        Val::from("- there's a ") + color.clone() + " Gem -",
        "- half-buried. -"
    ])?;
    ctx.next()?;
    if ctx.menu(&["Dig out the Gem", "Leave it alone"])? == 1 {
        ctx.lines(args![
            "- You overspread some earth over the Gem -",
            "- and left the Gem as it was. -"
        ])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["...What the heck am I doing now..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.rand_range(1, 4)? == 2 {
        ctx.lines(args![
            "- You dug up a pile of earth -",
            Val::from("- and pulled out the ") + color.clone() + " Gem -"
        ])?;
        ctx.call(Function::SetQuest, args![Val::from(8240) + num.clone()])?;
        ctx.call(Function::GetItem, args![Val::from(7574) + num.clone(), 1])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args!["- As you dug up a pile of earth, -", "- a Thief Bug attacks you! -"])?;
    ctx.call(
        Function::DoNpcEvent,
        args![Val::from("Half-buried Gem#") + num.clone() + "::OnDisable"],
    )?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["What the heck is this?!?!?!"])?;
    ctx.close_window()?;
    Err(Stop::End)
}
