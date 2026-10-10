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

fn show_point(ctx: &Ctx, x: i32, y: i32, id: i32, color: i32, lines: Vec<Val>) -> Script {
    ctx.lines_as("Brasilis Guide", lines)?;
    ctx.call(Function::ViewPoint, args![1, x, y, id, color])?;
    ctx.close()
}

pub fn brasilis_guide(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Brasilis Guide",
        args![
            "Welcome to ^8B4513Brasilis^000000, a country as passionate as the sun.",
            "If you have any questions, please ask me."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Ask about locations", "Remove Marks from Mini-Map", "Cancel"])? {
        0 => {
            ctx.lines_as("Brasilis Guide", args!["Where can I guide you?"])?;
            ctx.next()?;
            match ctx.menu(&[
                "[ Hotel ]",
                "[ Jungle Cable ]",
                "[ Art Museum ]",
                "[ Market ]",
                "[ Verass Monument ]",
            ])? {
                0 => show_point(
                    ctx,
                    274,
                    151,
                    2,
                    16724821,
                    args![
                        "The Brasilis Hotel is located just above, ^FF3355+^000000.",
                        "Is there anything else I can do for you?"
                    ],
                ),
                1 => show_point(
                    ctx,
                    308,
                    335,
                    3,
                    13525760,
                    args![
                        "Do you want to go through the rough jungle? You can take a ",
                        "Jungle Cable here ^CE6300+^000000.",
                        "Is there anything else I can do for you?"
                    ],
                ),
                2 => show_point(
                    ctx,
                    137,
                    167,
                    4,
                    65280,
                    args![
                        "The pride of Brasilis, the world scale Art Museum is at ^A5BAAD+^000000.",
                        "Is there anything else I can do for you?"
                    ],
                ),
                3 => show_point(
                    ctx,
                    254,
                    248,
                    5,
                    5635891,
                    args![
                        "You can buy items for hunting at the Market here ^55FF33+^000000.",
                        "Is there anything else I can do for you?"
                    ],
                ),
                4 => show_point(
                    ctx,
                    195,
                    235,
                    6,
                    3364351,
                    args![
                        "The iconic monument of Brasilis, the Verass Monument stands at ^3355FF+^000000.",
                        "Is there anything else I can do for you?"
                    ],
                ),
                _ => ctx.end(),
            }
        }
        1 => {
            ctx.lines_as(
                "Brasilis Guide",
                args![
                    "I'll remove all marks from your mini-map.",
                    "Is there anything else I can do for you?"
                ],
            )?;
            for (x, y, id) in [(274, 151, 2), (308, 335, 3), (137, 167, 4), (254, 248, 5), (195, 235, 6)] {
                ctx.call(Function::ViewPoint, args![0, x, y, id, 65280])?;
            }
            ctx.close()
        }
        2 => {
            ctx.lines_as(
                "Brasilis Guide",
                args!["Wandering on your own is always the best way to explore. Anyway, take care."],
            )?;
            ctx.close()
        }
        _ => Ok(()),
    }
}
