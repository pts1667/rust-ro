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

pub fn f_innmaid(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let npc_name = runtime::arg(&args, 0, Val::from(0));
    ctx.lines(args![
        npc_name.clone(),
        "Welcome to",
        runtime::arg(&args, 1, Val::from(0)) + Val::from("."),
        "How may I help you?"
    ])?;
    ctx.next()?;
    match ctx.menu(&["Save", "Take a Rest -> 5000 zeny", "Cancel"])? {
        0 => {
            ctx.lines(args![
                npc_name.clone(),
                "Your respawn point",
                "has been saved.",
                "Thank you,",
                "please come again."
            ])?;
            ctx.call(
                Function::SavePoint,
                args![
                    runtime::arg(&args, 2, Val::from(0)),
                    runtime::arg(&args, 3, Val::from(0)),
                    runtime::arg(&args, 4, Val::from(0)),
                    1,
                    1
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        1 => {
            ctx.lines(args![npc_name.clone()])?;
            if ctx.player().zeny()? < 5000 {
                ctx.mes("I'm sorry, but the service charge is 5,000 zeny. Please make sure that you have enough money to check in next time, okay?")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines(args!["Thank you.", "I hope you", "enjoy your rest~"])?;
            ctx.close_window()?;
            ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
            ctx.call(Function::PercentHeal, args![100, 100])?;
            return Ok(Val::from(0));
        }
        2 => {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}
