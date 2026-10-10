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

const MARK_SPOTS: [(i32, i32, i32, i32); 14] = [
    (237, 41, 4, 65280),
    (237, 41, 5, 255),
    (46, 345, 6, 65280),
    (175, 220, 7, 16711680),
    (134, 221, 8, 16711680),
    (204, 191, 9, 16711680),
    (107, 192, 10, 16711680),
    (179, 184, 11, 65280),
    (208, 154, 12, 65280),
    (120, 267, 13, 65280),
    (192, 267, 14, 65280),
    (133, 183, 15, 65280),
    (156, 360, 16, 65280),
    (75, 91, 17, 65280),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_prt(ctx: &Ctx) -> Script {
    ctx.fx().cutin("prt_soldier", 2)?;
    ctx.lines_as(
        "Prontera Guide",
        args![
            "Welcome to Prontera,",
            "the beautiful capital of the",
            "Rune-Midgarts Kingdom. If",
            "you have questions or need help finding something in the city, don't hesitate to ask."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide.", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Prontera Guide",
                    args!["Please select", "a location from", "the following menu."],
                )?;
                if !compass_marked {
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
                        "Swordman Association",
                        "^0000FFSanctuary^000000",
                        "Prontera Chivalry",
                        "Weapon Shop",
                        "Tool Shop",
                        "Inn",
                        "Trading Post",
                        "Pub",
                        "Library",
                        "Job Agency",
                        "Prontera Castle",
                        "City Hall",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args![
                                    "The Swordman Association,",
                                    "which handles Job Changes",
                                    "to the Swordman class, has",
                                    "moved to Izlude. This facility",
                                    "is just an empty building now."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (237, 41, 4, 65280))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args![
                                    "The Prontera Sanctuary",
                                    "handles Job Changes to",
                                    "the Acolyte class, and can",
                                    "be found in the northeast",
                                    "corner of Prontera."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (236, 316, 5, 16711680))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args![
                                    "The Prontera Chivralry,",
                                    "which is responsible for",
                                    "the safety of our capital, is",
                                    "in Prontera's northwest corner."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (46, 345, 6, 65280))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args!["The Weapon Shop", "is located northeast", "of the central fountain."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (175, 220, 7, 16711935))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args!["The Tool Shop", "is located northwest", "of the central fountain."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (134, 221, 8, 16711935))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args![
                                    "The Inns in Prontera are",
                                    "located both to the east",
                                    "and west of Prontera's",
                                    "central fountain area."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (204, 191, 9, 16711935))?;
                                view_point(ctx, 1, (107, 192, 10, 16711935))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args!["The Trading Post", "can be found southeast", "from the central fountain."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (179, 184, 11, 65280))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args!["The Pub is located", "southeast of the fountain,", "behind the Trading Post."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (208, 154, 12, 65280))?;
                            }
                        }
                        8 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args![
                                    "If you head north from",
                                    "the central fountain, you'll",
                                    "find an empty area in which",
                                    "both branches of the Prontera",
                                    "Library can be accessed if you",
                                    "head towards the east or west."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (120, 267, 13, 65280))?;
                                view_point(ctx, 1, (192, 267, 14, 65280))?;
                            }
                        }
                        9 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args!["The Job Agency is", "just southwest of the", "central fountain area."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (133, 183, 15, 65280))?;
                            }
                        }
                        10 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args![
                                    "The Prontera Castle is",
                                    "located at the northern",
                                    "sector of this city. You can",
                                    "go to the fields that are north",
                                    "of Prontera by going through",
                                    "the castle's rear exit."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (156, 360, 16, 65280))?;
                            }
                        }
                        11 => {
                            ctx.lines_as(
                                "Prontera Guide",
                                args!["The City Hall", "is located in the", "southwest corner", "in our city of Prontera."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (75, 91, 17, 130817))?;
                            }
                        }
                        12 => {
                            ctx.lines_as(
                                "Prontera Guide",
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
                    "Prontera Guide",
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
                    "Prontera Guide",
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
                    "Prontera Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Prontera."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Prontera Guide",
                    args![
                        "Well, adventurer...",
                        "I hope your journeys",
                        "through Rune-Midgarts",
                        "are both fun and safe."
                    ],
                )?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("prt_soldier", 255)?;
    ctx.end()
}
