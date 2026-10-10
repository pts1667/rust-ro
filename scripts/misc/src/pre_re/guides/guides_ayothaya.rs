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

fn show_point(ctx: &Ctx, x: i32, y: i32, id: i32, color: i32, lines: Vec<Val>, location: Vec<Val>) -> Script {
    ctx.lines_as("Noi", lines)?;
    ctx.next()?;
    ctx.lines_as("Noi", location)?;
    ctx.call(Function::ViewPoint, args![1, x, y, id, color])?;
    ctx.close()
}

pub fn noi_ayo(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Noi",
        args![
            "Welcome to Ayothaya.",
            "Our beautiful village is built",
            "above the water, surrounded",
            "by a dense forest."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Noi", args!["There are many tourist attractions in this village that you won't be able to find anywhere else. Our fish markets and the unique architecture of our buildings are enough reason to visit Ayotaya."])?;
    ctx.next()?;
    ctx.lines_as("Noi", args!["Please feel free", "to take a look around."])?;
    ctx.next()?;
    match ctx.menu(&["Building Locations.", "Remove marks from mini-map.", "Cancel."])? {
        0 => {
            ctx.lines_as("Noi", args!["Where would", "you like to visit?"])?;
            ctx.next()?;
            match ctx.menu(&["Weapon Shop", "Tool Shop", "Tavern", "Shrine", "Fishing Spot", "Cancel"])? {
                0 => show_point(
                    ctx,
                    165,
                    90,
                    2,
                    5635891,
                    args![
                        "At our Weapon Shop,",
                        "you will find great weapons",
                        "favored by brave Ayothayan seafarers."
                    ],
                    args!["Our Weapon Shop", "is located at ^55FF33+^000000."],
                ),
                1 => show_point(
                    ctx,
                    129,
                    86,
                    3,
                    3364351,
                    args![
                        "We Ayothayans always make sure we have everything we need before we go traveling. It never hurts to be prepared, doesn't it?"
                    ],
                    args!["Our Tool Shop", "is located at ^3355FF+^000000."],
                ),
                2 => show_point(
                    ctx,
                    232,
                    76,
                    4,
                    65280,
                    args![
                        "One of the basics of adventuring is gathering information, or at least that's what they say. You can meet people from all sorts of places in the Tavern. I'm sure you can learn something useful there."
                    ],
                    args!["Of course, you must", "drop by our Tavern.", "It is located at ^00FF00+^000000."],
                ),
                3 => show_point(
                    ctx,
                    208,
                    283,
                    5,
                    65280,
                    args![
                        "If you wish to pray to God, or achieve a state of peace in your mind, why don't you visit our Shrine? Even if it's just for sight-seeing, everyone is",
                        "welcome there."
                    ],
                    args!["Our Shrine", "is located at ^00FF00+^000000."],
                ),
                4 => show_point(
                    ctx,
                    253,
                    99,
                    6,
                    65280,
                    args![
                        "Since Ayothaya was built above the surface of the water and close to a beach, it's been a favorite spot for fishermen. Why don't you catch some fish for dinner at the Fishing Spot?"
                    ],
                    args!["Our famous", "Fishing Spot", "is located at ^00FF00+^000000"],
                ),
                5 => {
                    ctx.lines_as("Noi", args!["If you wish to remove location marks on your mini-map, please select the 'Remove marks from mini-map' command from the menu."])?;
                    ctx.close()
                }
                _ => Ok(()),
            }
        }
        1 => {
            for (x, y, id, color) in [
                (165, 90, 2, 5635891),
                (129, 86, 3, 3364351),
                (232, 76, 4, 65280),
                (208, 283, 5, 65280),
                (253, 99, 6, 65280),
            ] {
                ctx.call(Function::ViewPoint, args![2, x, y, id, color])?;
            }
            ctx.lines_as(
                "Noi",
                args![
                    "Alright...",
                    "I've removed all the",
                    "location marks from",
                    "your mini-map.",
                    "Thank you."
                ],
            )?;
            ctx.close()
        }
        2 => {
            ctx.lines_as("Noi", args!["Please enjoy", "your travels."])?;
            ctx.close()
        }
        _ => Ok(()),
    }
}
