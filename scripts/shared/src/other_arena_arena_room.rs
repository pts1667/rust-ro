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

pub fn func_are_rew(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let item_id = runtime::arg(&args, 0, Val::from(0));
    let amount = runtime::arg(&args, 1, Val::from(0));
    let arena_point = runtime::arg(&args, 2, Val::from(0));
    ctx.lines_as(
        "Givu",
        args![
            Val::from("Would you like to exchange your arena points with ") + ctx.call(Function::GetItemName, args![item_id.clone()])? + "?",
            Val::from("You can exchange ^3131FF") + arena_point.clone() + " arena points with " + amount.clone() + " " + ctx.call(Function::GetItemName, args![item_id.clone()])? + "^000000.",
            "If you wish to cancel, please enter 0. If you don't, please enter how many ^3131FFtimes^000000 of arena points you wish to spend.",
        ],
    )?;
    ctx.next()?;
    let (input, _) = runtime::input_number(ctx, None, None)?;
    let reward = input.number()?;
    if reward <= 0 {
        ctx.lines_as("Givu", args!["You have canceled your request."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if reward > 1500 {
        ctx.lines_as("Givu", args!["You have exceeded the maximum capacity."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&ctx.var("arena_point").get()?, "<", &arena_point.clone().try_mul(reward)?)?.is_true() {
        ctx.lines_as(
            "Givu",
            args![
                "You do not have enough arena points.",
                "Please check the total amount of arena points you have."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.var("arena_point")
        .set(ctx.var("arena_point").get()?.try_sub(arena_point.clone().try_mul(reward)?)?)?;
    ctx.call(Function::GetItem, args![item_id.clone(), amount.clone().try_mul(reward)?])?;
    ctx.lines_as("Givu", args!["Thank you, please come again."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}
