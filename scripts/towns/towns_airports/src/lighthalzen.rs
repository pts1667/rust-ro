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

pub fn airport_staff_lhz_air1a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airport Staff",
        args![
            "Welcome to the",
            "Lighthalzen Airport,",
            "where we offer nonstop",
            "flights to Einbroch, Juno and Hugel."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Board the Airship.", "Cancel."])? == 0 {
        ctx.lines_as(
            "Airport Staff",
            args![
                "The boarding fee is",
                "1,200 zeny, but you can",
                "waive the fee if you redeem",
                "a Free Ticket for Airship."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.items().count(7311)? > 0 {
                ctx.items().take(7311, 1)?;
                ctx.warp("lhz_airport", 148, 51)?;
                return ctx.end();
            }
            if ctx.player().zeny()? >= 1200 {
                ctx.player().set_zeny(ctx.player().zeny()? - 1200)?;
                ctx.warp("lhz_airport", 148, 51)?;
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
        "Airport Staff",
        args!["Thank you and", "please come again.", "Have a good day~"],
    )?;
    ctx.close()
}

pub fn arrival_staff_lhz_air2a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Arrival Staff",
        args![
            "Welcome to Lighthalzen Airport.",
            "Please let me guide you to the",
            "main terminal if you are arriving from your flight. Otherwise, please",
            "board the departing Airship to reach your intended destination."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Exit to main terminal.", "Cancel."])? == 0 {
        ctx.lines_as(
            "Arrival Staff",
            args![
                "Once you're in the main terminal, you will need to pay the fee again",
                "to board an Airship. You should",
                "only exit if Lighthalzen is your intended destination. Shall we",
                "proceed to the main terminal?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            ctx.warp("lhz_airport", 142, 40)?;
            return ctx.end();
        }
    }
    ctx.lines_as(
        "Arrival Staff",
        args![
            "Alright, thank you",
            "for your patronage",
            "and I hope you have",
            "a pleasant flight~"
        ],
    )?;
    ctx.close()
}
