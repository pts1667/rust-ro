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

pub fn moscovia_guide_mosk(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Moscovia Guide",
        args![
            "Welcome to Moscovia",
            "Here is the paradise spreading on",
            "the endless seas",
            "You'll be happy with the beautiful",
            "scenery and the sunlight!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Moscovia Guide",
        args![
            "I was sent from Moscovia Palace",
            "to guide tourists and to give them",
            "information on this town.",
            "If you have some questions, please ask me."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Ask where you can go.", "Delete all the marks on the mini-map.", "Cancel."])? {
        0 => {
            ctx.lines_as("Moscovia Guide", args!["Where would you like to go?"])?;
            ctx.next()?;
            match ctx.menu(&["The Palace", "Armor Shop", "Tool Shop", "An Inn", "Cancel"])? {
                0 => {
                    ctx.lines_as(
                        "Moscovia Guide",
                        args![
                            "The Palace can be found ^ff0000+^000000 at the",
                            "end of the North sea from",
                            "Rune-Midgarts.",
                            "There resides our Lord the Czar of",
                            "Moscovia and his retainers."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::ViewPoint, args![1, 257, 138, 1, 16711680])?;
                    ctx.end()
                }
                1 => {
                    ctx.call(Function::ViewPoint, args![1, 185, 187, 2, 65280])?;
                    ctx.lines_as(
                        "Moscovia Guide",
                        args![
                            "The Armor Shop is located at the",
                            "southwest corner of town..",
                            "You can buy armor made by the best",
                            "craftsmen of Moscovia there."
                        ],
                    )?;
                    ctx.close()
                }
                2 => {
                    ctx.lines_as(
                        "Moscovia Guide",
                        args![
                            "The Tool Shop is located just south",
                            "from the center of town.",
                            "You can find all sorts of things",
                            "you need for your travels."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::ViewPoint, args![1, 223, 174, 3, 65280])?;
                    ctx.end()
                }
                3 => {
                    ctx.lines_as(
                        "Moscovia Guide",
                        args![
                            "The Inn 'Sticky Herb Tree' is just",
                            "north from the center of town.",
                            "If you need to rest, there is no",
                            "better place to stay."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::ViewPoint, args![1, 229, 208, 4, 3364351])?;
                    ctx.end()
                }
                _ => ctx.close(),
            }
        }
        1 => {
            ctx.lines_as(
                "Moscovia Guide",
                args![
                    "I've deleted all marks on the mini-map.",
                    "Whenever you'd like to put marks",
                    "there, you can ask me."
                ],
            )?;
            ctx.call(Function::ViewPoint, args![2, 257, 138, 1, 16711680])?;
            ctx.call(Function::ViewPoint, args![2, 185, 187, 2, 65280])?;
            ctx.call(Function::ViewPoint, args![2, 223, 174, 3, 65280])?;
            ctx.call(Function::ViewPoint, args![2, 229, 208, 4, 3364351])?;
            ctx.close()
        }
        _ => {
            ctx.lines_as("Moscovia Guide", args!["It'd be great to walk about alone.", "Take care."])?;
            ctx.close()
        }
    }
}
