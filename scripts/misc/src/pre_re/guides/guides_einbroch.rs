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

// The Laboratory is removed with colour 65280 but shown with 255.
const MARK_SPOTS: [(i32, i32, i32, i32); 9] = [
    (63, 228, 2, 16711680),
    (236, 279, 3, 16711935),
    (158, 78, 4, 16711935),
    (232, 190, 5, 16711935),
    (260, 201, 6, 65280),
    (215, 221, 7, 65280),
    (36, 49, 8, 65280),
    (244, 90, 9, 65280),
    (174, 195, 10, 16776960),
];

const MARK_SPOTS_2: [(i32, i32, i32, i32); 5] = [
    (43, 213, 2, 16711680),
    (142, 112, 3, 16711935),
    (176, 136, 4, 16711935),
    (250, 110, 5, 16711935),
    (138, 251, 6, 65280),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_ein(ctx: &Ctx) -> Script {
    ctx.fx().cutin("ein_soldier", 2)?;
    ctx.lines_as(
        "Einbroch Guide",
        args![
            "Welcome",
            "to Einbroch,",
            "the City of Steel.",
            "Please ask me if you",
            "have any questions."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide.", "Remove Marks from Mini-Map.", "Notice.", "Cancel."])? {
            0 => {
                ctx.lines_as(
                    "Einbroch Guide",
                    args!["Please select", "a location from", "the following menu."],
                )?;
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
                        "^FF0000Airport^000000",
                        "Train Station",
                        "Factory",
                        "Plaza",
                        "Hotel",
                        "Weapon Shop",
                        "Laboratory",
                        "Blacksmith Guild",
                        "Einbroch Tower",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The ^FF0000Airport^000000 is located",
                                    "in the northwestern part",
                                    "of the city. There you can",
                                    "see our city's pride and joy, the Airship. Remember that you must pay admission to board the Airship."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (63, 228, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Train Station is",
                                    "located in the northeast",
                                    "part of Einbroch. Trains",
                                    "running between here",
                                    "and Einbech run all day",
                                    "long, everyday."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (236, 279, 3, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Factory, perhaps the",
                                    "most important facility in",
                                    "Einbroch, is located in the",
                                    "southern part of the city."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (158, 78, 4, 16711935))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Plaza, our biggest",
                                    "shopping district, can be",
                                    "found just east from the",
                                    "center of Einbroch."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (232, 190, 5, 16711935))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Hotel is east of",
                                    "the Plaza and offers top",
                                    "caliber accomodations.",
                                    "There, you can enjoy your",
                                    "stay in Einbroch in comfort~"
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (260, 201, 6, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Weapon Shop is",
                                    "located north from the",
                                    "Plaza. There you can",
                                    "purchase weapons for",
                                    "your personal use."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (215, 221, 7, 65280))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Laboratory is an",
                                    "annex of the Factory and",
                                    "is located in the southwest",
                                    "sector of Einbroch."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (36, 49, 8, 255))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Blacksmith Guild is",
                                    "located in the southeast",
                                    "part of Einbroch. You can",
                                    "upgrade your equipment",
                                    "by using their services."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (244, 90, 9, 65280))?;
                            }
                        }
                        8 => {
                            ctx.lines_as(
                                "Einbroch Guide",
                                args![
                                    "The Einbroch Tower is",
                                    "located in the center of",
                                    "the city. From the top of",
                                    "the tower, you can view",
                                    "all of Einbroch."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (174, 195, 10, 16776960))?;
                            }
                        }
                        9 => {
                            ctx.lines_as(
                                "Einbroch Guide",
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
                ctx.lines_as(
                    "Einbroch Guide",
                    args![
                        "Okay, the marks from",
                        "your Mini-Map have been",
                        "removed. If you need any",
                        "guidance around Einbroch,",
                        "please let me or one of the",
                        "other Einbroch Guides know."
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Einbroch Guide",
                    args![
                        "Through the technology of",
                        "the Schwarzwald Republic,",
                        "we've upgraded to a digital",
                        "information system that allows",
                        "us to mark locations on your",
                        "Mini-Map for easier navigation."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Einbroch Guide",
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
                    "Einbroch Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in Einbroch, adventurer."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Einbroch Guide",
                    args![
                        "We hope that you",
                        "enjoy your travels",
                        "here in Einbroch.",
                        "Oh, and please be",
                        "aware of the Smog Alerts."
                    ],
                )?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("ein_soldier", 255)?;
    ctx.end()
}

pub fn guide_4ein(ctx: &Ctx) -> Script {
    ctx.fx().cutin("ein_soldier", 2)?;
    ctx.lines_as(
        "Einbech Guide",
        args![
            "Welcome to Einbech,",
            "the Mining Town. We're",
            "here to assist tourists,",
            "so if you have any questions,",
            "please feel free to ask us."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide.", "Remove Marks from Mini-Map.", "Notice.", "Cancel."])? {
            0 => {
                ctx.lines_as(
                    "Einbech Guide",
                    args!["Please select", "a location from", "the following menu."],
                )?;
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
                    match ctx.menu(&["Train Station", "Tavern", "Tool Shop", "Swordman Guild", "Mine", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Einbech Guide",
                                args![
                                    "The Train Stations are",
                                    "located in the northwest",
                                    "and northeast parts of",
                                    "Einbech. There, you can",
                                    "take a train to Einbroch."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (43, 213, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Einbech Guide",
                                args![
                                    "The Tavern is located",
                                    "in the southern part of",
                                    "Einbech. It's a nice place",
                                    "to relax after a long day."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (142, 112, 3, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Einbech Guide",
                                args![
                                    "You can find the Tool",
                                    "Shop in the center of",
                                    "Einbech. There, you can",
                                    "purchase any tools you",
                                    "might need for your travels."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (176, 136, 4, 16711935))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Einbech Guide",
                                args![
                                    "The Swordman Guild",
                                    "is located in the eastern",
                                    "outskirts of Einbech. It's",
                                    "under construction and they",
                                    "haven't started accepting",
                                    "applications."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (250, 110, 5, 16711935))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Einbech Guide",
                                args![
                                    "The Mine, which is",
                                    "Einbech's major industry,",
                                    "is located in the northern",
                                    "part of this town. It's where",
                                    "we get all our ores, although monsters get in the miners' way."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (138, 251, 6, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Einbech Guide",
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
                for spot in MARK_SPOTS_2 {
                    view_point(ctx, 2, spot)?;
                }
                compass_marked = false;
                ctx.lines_as(
                    "Einbech Guide",
                    args![
                        "Okay, the marks from",
                        "your Mini-Map have been",
                        "removed. If you need any",
                        "guidance around Einbech,",
                        "please let me or one of the",
                        "other Einbech Guides know."
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Einbech Guide",
                    args![
                        "Through the technology of",
                        "the Schwarzwald Republic,",
                        "we've upgraded to a digital",
                        "information system that allows",
                        "us to mark locations on your",
                        "Mini-Map for easier navigation."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Einbech Guide",
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
                    "Einbech Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in Einbech, adventurer."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Einbech Guide",
                    args!["We hope that you", "enjoy your travels", "here in Einbech."],
                )?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("ein_soldier", 255)?;
    ctx.end()
}
