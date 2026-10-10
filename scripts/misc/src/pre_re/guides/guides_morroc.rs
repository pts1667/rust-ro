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

const MARK_SPOTS: [(i32, i32, i32, i32); 6] = [
    (237, 41, 2, 65280),
    (237, 41, 3, 255),
    (46, 345, 4, 65280),
    (175, 220, 5, 16711680),
    (175, 220, 6, 16711680),
    (175, 220, 7, 16711680),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_moc(ctx: &Ctx) -> Script {
    ctx.fx().cutin("moc_soldier", 2)?;
    ctx.lines_as(
        "Morocc Guide",
        args![
            "Welcome to Morocc,",
            "the frontier town of the",
            "Rune-Midgarts Kingdom.",
            "Please ask me for help if",
            "you're having any trouble",
            "finding anything in town."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as("Morocc Guide", args!["Please select", "a location from", "the following menu."])?;
                if !compass_marked {
                    ctx.lines(args!["Would you like me", "to mark locations", "on your Mini-Map?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Yes.", "No."])? == 0 {
                        compass_marked = true;
                    }
                }
                loop {
                    if city_menu_shown {
                        ctx.next()?;
                    }
                    city_menu_shown = true;
                    match ctx.menu(&[
                        "^FF0000Thief Guild^000000",
                        "Weapon Shop",
                        "Inn",
                        "Pub",
                        "Mercenary Guild",
                        "Forge",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Morocc Guide",
                                args![
                                    "The Thief Guild is",
                                    "in charge of all Job",
                                    "Changes to the Thief",
                                    "Class. From what I hear,",
                                    "you can find them inside",
                                    "the Pyramids nearby..."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (24, 297, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Morocc Guide",
                                args!["The Weapon Shop", "is in the southeast", "end of Morocc."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (253, 56, 3, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Morocc Guide",
                                args![
                                    "There are Inns",
                                    "where you can rest",
                                    "at the southeast and",
                                    "northeast ends of Morocc."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (197, 66, 4, 16711935))?;
                                view_point(ctx, 1, (273, 269, 5, 16711935))?;
                            }
                        }
                        3 => {
                            ctx.lines_as("Morocc Guide", args!["You can find the", "Pub in northeast Morocc."])?;
                            if compass_marked {
                                view_point(ctx, 1, (52, 259, 6, 16711935))?;
                            }
                        }
                        4 => {
                            ctx.lines_as("Morocc Guide", args!["The Mercenary", "Guild is located", "in East Morocc."])?;
                            if compass_marked {
                                view_point(ctx, 1, (284, 171, 7, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Morocc Guide",
                                args!["The Forge is", "located just", "southwest from", "the center of Morocc."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (47, 47, 7, 16711935))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Morocc Guide",
                                args![
                                    "Please ask me to ''Remove",
                                    "Marks from Mini-Map'' if you",
                                    "no longer wish to have the",
                                    "location marks displayed",
                                    "on your Mini-Map."
                                ],
                            )?;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                for spot in MARK_SPOTS {
                    view_point(ctx, 2, spot)?;
                }
                compass_marked = false;
            }
            2 => {
                ctx.lines_as(
                    "Morocc Guide",
                    args![
                        "Advances in sorcery and",
                        "technology have allowed",
                        "us to update our information",
                        "system, enabling up to mark",
                        "locations on your Mini-Map",
                        "for easier navigation."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Morocc Guide",
                    args![
                        "Your Mini-Map is located",
                        "in the upper right corner",
                        "of the screen. If you can't",
                        "see it, press the Ctrl + Tab",
                        "keys or click the ''Map'' button in your Basic Info Window."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Morocc Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Morocc."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Morocc Guide",
                    args!["Alright then,", "try to stay out of", "too much trouble", "out there, adventurer."],
                )?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("moc_soldier", 255)?;
    ctx.end()
}
