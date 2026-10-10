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

pub fn pr_officer_moscovia(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Moscovia P.R. Officer",
        args![
            "Moscovia! Do you know Moscovia?",
            "the paradise spreading on the endless seas...",
            "Welcome to Moscovia",
            "It's adventurous and mystic."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["About Moscovia...", "Go to Moscovia.", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Moscovia P.R. Officer",
                args![
                    "Moscovia is a beautiful kingdom",
                    "on an island located north of Rune-",
                    "Midgarts."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Moscovia P.R. Officer",
                args![
                    "I'm sure that you will be",
                    "absolutely fascinated",
                    "by Moscovia's beautiful scenery",
                    "and gorgeous palace."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Moscovia P.R. Officer",
                args![
                    "Now that our long winter has",
                    "passed,",
                    "I'm happy that I can now show you",
                    "our gorgeous hometown."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Moscovia P.R. Officer",
                args![
                    "If you feel like visiting Moscovia",
                    "take the chance now!",
                    "I'll help you to have a nice trip",
                    "to Moscovia!"
                ],
            )?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Moscovia P.R. Officer",
                args![
                    "Ok then, let us start now.",
                    "You should pay me 10,000 zeny",
                    "to go to Moscovia.",
                    "But when you come back,",
                    "you don't have to pay."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Moscovia P.R. Officer", args!["Can we leave now?"])?;
            ctx.next()?;
            if ctx.menu(&["Let's go!", "Cancel"])? == 1 {
                ctx.lines_as(
                    "Moscovia P.R. Officer",
                    args![
                        "If you're too busy now,",
                        "please tell me again whenever you want.",
                        "I'm always ready to guide anyone to Moscovia."
                    ],
                )?;
                return ctx.close();
            }
            if ctx.player().zeny()? < 10000 {
                ctx.lines_as(
                    "Moscovia P.R. Officer",
                    args![
                        "I'm sorry but you don't have",
                        "enough zeny now",
                        "You need 10,000 zeny",
                        "to go to Moscovia",
                        "Thank you."
                    ],
                )?;
                ctx.close()
            } else {
                ctx.lines_as("Moscovia P.R. Officer", args!["Ok then, we're leaving now."])?;
                ctx.close_window()?;
                ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
                ctx.warp("moscovia", 163, 55)?;
                ctx.end()
            }
        }
        _ => {
            ctx.lines_as(
                "Moscovia P.R. Officer",
                args![
                    "If you're too busy now,",
                    "please tell me again whenever you want.",
                    "I'm always ready to guide anyone to Moscovia."
                ],
            )?;
            ctx.close()
        }
    }
}

pub fn moscovia_p_r_officer_2(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Moscovia P.R. Officer",
        args![
            "How was your trip?",
            "Do you have good memories from Moscovia?",
            "A ship is now leaving",
            "for Rune-Midgarts."
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Return to Alberta", "Cancel"])? == 1 {
        ctx.lines_as(
            "Moscovia P.R. Officer",
            args!["If you want to see more", "please take your time."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Moscovia P.R. Officer",
        args!["Please come and visit soon.", "Ok then, Let's get going."],
    )?;
    ctx.close_window()?;
    ctx.warp("alberta", 243, 67)?;
    ctx.end()
}

pub fn soldier_mosk1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Soldier",
        args![
            "Our dear Csar Alexsay III is in the palace.",
            "He rules over Moscovia.",
            "Please be careful not to cause him any trouble."
        ],
    )?;
    ctx.close()
}

pub fn soldier_mosk(ctx: &Ctx) -> Script {
    ctx.lines_as("Soldier", args!["Please be silent or the Csar will be angry."])?;
    ctx.close()
}
