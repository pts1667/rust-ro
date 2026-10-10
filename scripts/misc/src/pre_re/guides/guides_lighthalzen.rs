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

const MARK_SPOTS: [(i32, i32, i32, i32); 10] = [
    (102, 247, 2, 16711680),
    (233, 164, 3, 16711935),
    (236, 276, 4, 10092543),
    (198, 257, 5, 255),
    (159, 133, 6, 65280),
    (267, 75, 7, 65280),
    (74, 53, 8, 16751103),
    (93, 110, 9, 16750848),
    (196, 46, 10, 3342387),
    (199, 163, 11, 16776960),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_lhz(ctx: &Ctx) -> Script {
    ctx.fx().cutin("ein_soldier", 2)?;
    ctx.lines_as(
        "Lighthalzen Guide",
        args![
            "Welcome to Lighthalzen,",
            "the Corporation City-State.",
            "If you need any guidance",
            "around the city, feel free",
            "to ask me and I'll do my",
            "very best to help you."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice.", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Lighthalzen Guide",
                    args![
                        "Please be aware that I'm",
                        "in charge of providing info",
                        "regarding the West District",
                        "of Lighthalzen. Now, please",
                        "select the location that you'd",
                        "like to learn more about."
                    ],
                )?;
                if !compass_marked {
                    ctx.next()?;
                    ctx.lines_as(
                        "Lighthalzen Guide",
                        args!["But before that,", "would you like me", "to mark locations", "on your Mini-Map?"],
                    )?;
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
                        "^FF0000Rekenber Corporation^000000",
                        "Train Station",
                        "Police Station",
                        "Bank",
                        "Hotel",
                        "Airport",
                        "Merchant Guild",
                        "Jewelry Shop",
                        "Weapon Shop",
                        "Departement Store",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "Rekenber Corporation,",
                                    "the largest company in",
                                    "the Schwarzwald Republic,",
                                    "in located in northwestern",
                                    "Lighthalzen. You can't miss",
                                    "the headquarters building."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (102, 247, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "The Train Station is",
                                    "located in the center of",
                                    "the city, where we have",
                                    "a direct railroad to Einbroch."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (233, 164, 3, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "Our Police Station is just",
                                    "north of the city's center.",
                                    "Please don't hesitate to report",
                                    "any suspicious persons and",
                                    "activity, or if you have any",
                                    "problems whatsoever."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (236, 276, 4, 10092543))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "The Bank is located",
                                    "just opposite to the",
                                    "Lighthalzen Police Station,",
                                    "which is a pretty good idea",
                                    "when I think about it, actually. ^FFFFFFspacer^000000"
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (198, 257, 5, 255))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "Our Hotel is located in",
                                    "the middle of the South Plaza.",
                                    "Due to its quality services and",
                                    "luxurious accomodations, this",
                                    "hotel is extremely popular."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (159, 133, 6, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "The Airport is to the far",
                                    "west of the Central Promenade.",
                                    "You can travel anywhere within",
                                    "the Schwarzwald Republic by",
                                    "riding on one of the Airships."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (267, 75, 7, 65280))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args!["The Merchant Guild can be", "found in the southwestern", "part of Lighthalzen."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (74, 53, 8, 16751103))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args!["The Jewelry Shop is", "located just west of", "the South Plaza."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (93, 110, 9, 16750848))?;
                            }
                        }
                        8 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "The Weapon Shop is",
                                    "located at the end of",
                                    "the Central Promenade.",
                                    "It's at least worth a look",
                                    "if you're serious about",
                                    "adventuring around here."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (196, 46, 10, 3342387))?;
                            }
                        }
                        9 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
                                args![
                                    "The Department Store is",
                                    "located in the middle of",
                                    "Lighthalzen and is the biggest",
                                    "and most convenient place for",
                                    "shopping for almost everything."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (199, 163, 11, 16776960))?;
                            }
                        }
                        10 => {
                            ctx.lines_as(
                                "Lighthalzen Guide",
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
                    "Lighthalzen Guide",
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
                    "Lighthalzen Guide",
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
                    "Lighthalzen Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in Lighthalzen."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Lighthalzen Guide",
                    args![
                        "Lighthalzen is divided",
                        "into the East and West",
                        "districts by a railroad that",
                        "runs right through the middle.",
                        "There are always guards on",
                        "watch to protect the peace."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Lighthalzen Guide",
                    args![
                        "Please don't hesitate",
                        "to report any suspicious",
                        "activity or persons to us.",
                        "We hope that you enjoy",
                        "our fair city, adventurer."
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
