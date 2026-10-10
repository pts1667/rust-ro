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

pub fn airport_staff_airport1a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airport Staff",
        args![
            "Welcome to the",
            "Einbroch Airport,",
            "where we offer nonstop",
            "flights to the cities of",
            "Juno, Lighthalzen and Hugel."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Board the Airship", "Cancel"])? == 0 {
        ctx.lines_as(
            "Airport Staff",
            args![
                "The Airship boarding fee",
                "is 1,200 zeny, but if you've",
                "got a Free Ticket for Airship,",
                "the fee will be waived. Will",
                "you board the Airship?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.items().count(7311)? > 0 {
                ctx.items().take(7311, 1)?;
                ctx.warp("airport", 148, 51)?;
                return ctx.end();
            }
            if ctx.player().zeny()? >= 1200 {
                ctx.player().set_zeny(ctx.player().zeny()? - 1200)?;
                ctx.warp("airport", 148, 51)?;
                return ctx.end();
            }
            ctx.lines_as(
                "Airport Staff",
                args![
                    "I'm sorry, but you don't",
                    "have a Free Ticket for",
                    "Airship and you don't have",
                    "enough zeny for boarding",
                    "the Airship. Remember, the",
                    "boarding fee is 1,200 zeny."
                ],
            )?;
            return ctx.close();
        }
    }
    ctx.lines_as("Airport Staff", args!["Thank you and", "have a nice day."])?;
    ctx.close()
}

pub fn arrival_staff_airport2a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Arrival Staff",
        args![
            "Welcome to Einbroch Airport.",
            "If you are arriving from your",
            "flight, let me guide you to the",
            "main terminal. Otherwise, please board the Airship to depart to",
            "Juno, Lighthalzen and Hugel."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Exit to main terminal.", "Cancel."])? == 0 {
        ctx.lines_as(
            "Arrival Staff",
            args![
                "Once you're in the main terminal, you will need to pay the fee again",
                "to board an Airship. You should",
                "only exit if the city of Einbroch",
                "is your intended destination.",
                "Proceed to the main terminal?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            ctx.warp("airport", 142, 40)?;
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

pub fn airship_staff_ein01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airship Staff",
        args![
            "Welcome to the",
            "Einbroch Airport.",
            "Please use this door to",
            "board the Airship which stops",
            "over Juno, Lighthalzen and",
            "Hugel in the Schwarzwald Republic."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Airship Staff",
        args![
            "Otherwise, if Einbroch is",
            "your intended destination,",
            "please head down the stairs",
            "and ask the Arrival Staff to lead",
            "you to the main terminal. Thank",
            "you, and enjoy your travels."
        ],
    )?;
    ctx.close()
}
