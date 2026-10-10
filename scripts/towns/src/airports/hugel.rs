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

pub fn toairplane_hugel(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn toairplane_hugel_ontouch(ctx: &Ctx) -> Script {
    ctx.lines(args![
        "To use the airship, you are required to pay 1,200 zeny or a Free Airship Ticket.",
        "Would you like to use the service?"
    ])?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 0 {
        if ctx.items().count(7311)? > 0 {
            ctx.items().take(7311, 1)?;
            ctx.warp("airplane", 244, 58)?;
            return ctx.end();
        }
        if ctx.player().zeny()? >= 1200 {
            ctx.player().set_zeny(ctx.player().zeny()? - 1200)?;
            ctx.warp("airplane", 244, 58)?;
            return ctx.end();
        }
        ctx.lines(args![
            "I am sorry, but you do not have enough money.",
            "Please remember, you are required to pay 1,200 zeny to use the service."
        ])?;
        return ctx.close();
    }
    ctx.mes("Thank you, please come again.")?;
    ctx.close()
}
