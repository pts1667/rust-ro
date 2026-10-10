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

pub fn f_bg_badge(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let team = runtime::arg(&args, 1, Val::from(0));
    let bg_type = runtime::arg(&args, 2, Val::from(0));
    let (badge, amount_win, amount_lose) = if bg_type == "Tierra" {
        (7828, 3, 1)
    } else if bg_type == "Flavius" {
        (7829, 9, 3)
    } else {
        return Ok(Val::from(0));
    };
    let won = runtime::arg(&args, 0, Val::from(0)) == 1;
    if won {
        if team == "Guillaume" {
            ctx.lines_as("Axl Rose", args!["Blessed Guillaume!"])?;
        } else if team == "Croix" {
            ctx.lines_as("Swandery", args!["Blessed Croix!"])?;
        }
        ctx.lines(args![
            "Let's enjoy our glorious victory!",
            ctx.player().name()? + ", it's a sign reflecting victory."
        ])?;
    } else if team == "Guillaume" {
        ctx.lines_as(
            "Axl Rose",
            args![
                "You lost, but you're dedicated to this battle.",
                "This is a reward for your great dedication by Guillaume Marollo!",
                "Just take this defeat as a lesson, and next time you will definitely win."
            ],
        )?;
    } else if team == "Croix" {
        ctx.lines_as(
            "Swandery",
            args![
                format!("Oh, {} Don't be sad.", ctx.player().name()?),
                "Even though we didn't win, we did our best.",
                "This is a Royal gift from Croix, and please don't forget this battle. We will win the next one."
            ],
        )?;
    }
    ctx.close_window()?;
    let mut amount = if won { amount_win } else { amount_lose };
    if constants::VIP_SCRIPT != 0 && ctx.call(Function::VipStatus, args![constants::VIP_STATUS_ACTIVE])?.is_true() {
        amount += 2;
    }
    let medal_gap = 500 - ctx.call(Function::CountItem, args![badge])?.number()?;
    if medal_gap >= amount {
        ctx.call(Function::GetItem, args![badge, amount])?;
    } else {
        ctx.call(Function::GetItem, args![badge, medal_gap])?;
    }
    return Ok(Val::from(0));
}
