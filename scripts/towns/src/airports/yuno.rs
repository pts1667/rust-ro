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

pub fn airport_staff_y_air1a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airport Staff",
        args![
            "Welcome to Juno Airport where we offer domestic flights to Einbroch, Lighthalzen and Hugel,",
            "and international flights to Izlude and Rachel.",
            "How may I be of service?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Board the Airship.", "Cancel."])? == 0 {
        ctx.lines_as(
            "Airport Staff",
            args![
                "The boarding fee for all",
                "flights is 1,200 zeny. If you",
                "use a Free Ticket for Airship,",
                "the boarding fee will be waived.So would you like to depart?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.items().count(7311)? > 0 {
                ctx.items().take(7311, 1)?;
                ctx.warp("y_airport", 148, 51)?;
                return ctx.end();
            }
            if ctx.player().zeny()? >= 1200 {
                ctx.player().set_zeny(ctx.player().zeny()? - 1200)?;
                ctx.warp("y_airport", 148, 51)?;
                return ctx.end();
            }
            ctx.lines_as(
                "Airport Staff",
                args!["I'm sorry, but you don't", "have 1,200 zeny to pay", "for the boarding fee."],
            )?;
            return ctx.close();
        }
    }
    ctx.lines_as("Airport Staff", args!["Thank you and", "have a nice day."])?;
    return ctx.close();
}

pub fn arrival_staff_y_air2a(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airport Staff",
        args![
            "Welcome to Juno Airport. If you've just arrived from your",
            "flight, let me guide you to the main terminal. Otherwise, please",
            "board the departing Airship to reach your intended destination."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Exit to main terminal", "Cancel"])? == 0 {
        ctx.lines_as(
            "Airport Staff",
            args![
                "Once you're in the main terminal, you must pay the fee once again",
                "to board a departing Airship. You should only exit if your intended",
                "destination is Juno. Proceed to",
                "exit to the main terminal?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            ctx.warp("y_airport", 142, 40)?;
            return ctx.end();
        }
    }
    ctx.lines_as(
        "Airport Staff",
        args![
            "Alright, thank you",
            "for your patronage",
            "and I hope you have",
            "a pleasant flight~"
        ],
    )?;
    return ctx.close();
}

pub fn domestic_boarding(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Boarding Staff",
        args![
            "Would you like to board the",
            "Airship that flies to Einbroch,",
            "Lighthalzen and Hugel? If so,",
            "please let me guide you to that",
            "Airship's boarding area."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 0 {
        ctx.warp("yuno", 59, 244)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Boarding Staff",
        args![
            "Very well, then.",
            "Thank you for your",
            "patronage, and I hope",
            "you enjoy your travels~"
        ],
    )?;
    return ctx.close();
}

pub fn international_boarding(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Boarding Staff",
        args![
            "Would you like to board",
            "the Airship which flies to",
            "Juno, Izlude and Rachel?",
            "If so, let me guide",
            "you to the boarding area."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 0 {
        ctx.warp("yuno", 47, 244)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Boarding Staff",
        args![
            "Alright, then.",
            "Thank you for flying",
            "with us, and I hope you",
            "enjoy your travels on our",
            "state of the art Airships."
        ],
    )?;
    return ctx.close();
}

pub fn airship_staff_yuno01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airship Staff",
        args![
            "Welcome to Juno Airport.",
            "Please use this door to",
            "board the Airship that will",
            "be flying all the way to Izlude",
            "in the Rune-Midgarts Kingdom,",
            "and to Rachel in the Arunafeltz",
            "Republic."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Airship Staff",
        args![
            "Otherwise, if Juno is",
            "your intended destination,",
            "please head down the stairs",
            "and ask the Arrival Staff to lead",
            "you to the main terminal. Thank",
            "you, and enjoy your travels."
        ],
    )?;
    return ctx.close();
}

pub fn airship_staff_yuno02(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airship Staff",
        args![
            "Welcome to Juno Airport.",
            "Please use this door to",
            "board the Airship which stops",
            "over Einbroch, Lighthalzen and",
            "Hugel in the Schwarzwald Republic."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Airship Staff",
        args![
            "Otherwise, if Juno is",
            "your intended destination,",
            "please head down the stairs",
            "and ask the Arrival Staff to lead",
            "you to the main terminal. Thank",
            "you, and enjoy your travels."
        ],
    )?;
    return ctx.close();
}
