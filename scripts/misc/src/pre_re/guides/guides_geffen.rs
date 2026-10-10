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

const MARK_SPOTS: [(i32, i32, i32, i32); 7] = [
    (237, 41, 2, 16711680),
    (237, 41, 3, 65280),
    (46, 345, 4, 16711935),
    (175, 220, 5, 16711935),
    (134, 221, 6, 16711935),
    (204, 214, 7, 16711935),
    (204, 214, 8, 65280),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_gef(ctx: &Ctx) -> Script {
    ctx.fx().cutin("gef_soldier", 2)?;
    ctx.lines_as(
        "Geffen Guide",
        args![
            "Welcome to Geffen,",
            "the City of Magic. If you",
            "need any guidance around",
            "the city, feel free to ask me",
            "and I'll do my best to assist you. ^FFFFFFcobo^000000"
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice.", "Cancel"])? {
            0 => {
                ctx.lines_as("Geffen Guide", args!["Please select", "a location from", "the following menu."])?;
                if ctx.var("compass_check").get()? == 0 {
                    ctx.lines(args!["Would you like me", "to mark locations", "on your Mini-Map?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Yes", "No"])? == 0 {
                        compass_marked = true;
                    }
                }
                loop {
                    if city_menu_shown {
                        ctx.next()?;
                    }
                    city_menu_shown = true;
                    match ctx.menu(&[
                        "^FF0000Magic Acedemy^000000",
                        "Forge Shop",
                        "Weapon Shop",
                        "Tool Shop",
                        "Pub",
                        "Inn",
                        "Geffen Tower",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args!["The Magic Academy in", "northwest Geffen handles", "Job Changes to the Mage class."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (61, 180, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args!["The Forge Shop is", "located just southeast", "from the center of Geffen."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (182, 59, 3, 65280))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args!["The Weapon Shop", "can be found northwest", "from the center of Geffen."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (99, 140, 4, 16711935))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args![
                                    "You can find the",
                                    "Tool Shop by heading",
                                    "southwest from the",
                                    "center of Geffen."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (44, 86, 5, 16711935))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args!["The Pub can be", "found northeast", "from the Geffen Tower."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (138, 138, 6, 16711935))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args!["The Inn can be", "found by traveling", "northeast from the", "center of Geffen."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (172, 174, 7, 16711935))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Geffen Guide",
                                args![
                                    "Geffen Tower is found",
                                    "in the center of the city.",
                                    "The Wizard Guild is at the",
                                    "top, and there's even a dungeon",
                                    "underneath it. There's many a",
                                    "mystery surrounding that tower..."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (120, 114, 8, 65280))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Geffen Guide",
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
                    "Geffen Guide",
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
                    "Geffen Guide",
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
                    "Geffen Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Geffen."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Geffen Guide",
                    args![
                        "Alright, adventurer.",
                        "I wish you safety on",
                        "your journeys through",
                        "the lands you may travel..."
                    ],
                )?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("gef_soldier", 255)?;
    ctx.end()
}
