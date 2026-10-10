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

pub fn airship_staff_izlude(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airship Staff",
        args![
            "Welcome! Would you like",
            "to board the Airship that",
            "departs on the flight which stops",
            "in Juno and Rachel?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Board the Airship to Juno/Rachel.", "Cancel."])? == 0 {
        ctx.lines_as(
            "Airship Staff",
            args![
                "The boarding fee is",
                "1,200 zeny. However, this",
                "charged is waived if you use",
                "a Free Ticket for Airship. Now,",
                "would you still like to board?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.items().count(7311)? > 0 {
                ctx.items().take(7311, 1)?;
                ctx.warp("airplane_01", 244, 58)?;
                return ctx.end();
            }
            if ctx.player().zeny()? >= 1200 {
                ctx.player().set_zeny(ctx.player().zeny()? - 1200)?;
                ctx.warp("airplane_01", 244, 58)?;
                return ctx.end();
            }
            ctx.lines_as(
                "Airship Staff",
                args!["I'm sorry, but you don't", "have 1,200 zeny to pay", "for the boarding fee."],
            )?;
            return ctx.close();
        }
    }
    ctx.lines_as(
        "Airship Staff",
        args!["Thank you and", "please come again.", "Have a good day~"],
    )?;
    ctx.close()
}
