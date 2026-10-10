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

const MARK_SPOTS: [(i32, i32, i32, i32); 9] = [
    (237, 41, 2, 65280),
    (237, 41, 3, 255),
    (46, 345, 4, 16711935),
    (175, 220, 5, 16711680),
    (175, 220, 6, 16711680),
    (175, 220, 7, 16711680),
    (237, 41, 8, 255),
    (46, 345, 9, 65280),
    (175, 220, 10, 16711680),
];

const MARK_SPOTS_2: [(i32, i32, i32, i32); 3] = [(237, 41, 0, 16711935), (237, 41, 1, 16711680), (46, 345, 2, 16711935)];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_pay(ctx: &Ctx) -> Script {
    ctx.fx().cutin("pay_soldier", 2)?;
    ctx.lines_as(
        "Payon Guide",
        args![
            "Welcome to the",
            "mountain city of Payon.",
            "If you're unfamiliar with this",
            "area, I can help you find what",
            "you're looking for around here."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice.", "Cancel"])? {
            0 => {
                ctx.lines_as("Payon Guide", args!["Please select", "a location from", "the following menu."])?;
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
                        "^FF0000Archer Guild^000000",
                        "Weapon Shop",
                        "Tool Shop",
                        "Pub",
                        "Central Palace",
                        "The Empress",
                        "Palace Annex",
                        "Royal Kitchen",
                        "Forge",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args![
                                    "The Archer Guild handles",
                                    "Job Changes to the Archer",
                                    "Class. You'll need to enter",
                                    "the Archer Village which is",
                                    "to the northeast of Payon."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (227, 328, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args![
                                    "The Weapon Shop",
                                    "can be found in the",
                                    "northwest corner of",
                                    "the city of Payon."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (139, 159, 3, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args!["The Tool Shop", "is located near", "the northwest", "corner of Payon."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (144, 85, 4, 16711935))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args![
                                    "The Pub can be",
                                    "found in the northeast",
                                    "part of Payon. It's the",
                                    "best place to relax after",
                                    "a long day of hunting."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (220, 117, 5, 16711935))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args!["The Central Palace", "is located to the north", "within the city of Payon."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (155, 245, 6, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args!["The Empress", "can be found to the", "northwest in Payon."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (107, 324, 7, 65280))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args!["The Palace Annex", "can be found in the", "western part of Payon."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (130, 204, 8, 65280))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args!["The Royal Kitchen", "is located near the", "northern end of Payon."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (154, 325, 9, 65280))?;
                            }
                        }
                        8 => {
                            ctx.lines_as("Payon Guide", args!["The Forge is", "situated near", "the center of Payon."])?;
                            if compass_marked {
                                view_point(ctx, 1, (126, 169, 10, 16776960))?;
                            }
                        }
                        9 => {
                            ctx.lines_as(
                                "Payon Guide",
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
                    "Payon Guide",
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
                    "Payon Guide",
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
                    "Payon Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Payon."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as("Payon Guide", args!["Be safe in", "your travels,", "brave adventurer."])?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("", 255)?;
    ctx.end()
}

pub fn guide_2pay(ctx: &Ctx) -> Script {
    ctx.fx().cutin("pay_soldier", 2)?;
    ctx.lines_as(
        "Payon Guide",
        args![
            "Welcome to the",
            "mountain city of Payon.",
            "If you're unfamiliar with this",
            "area, I can help you find what",
            "you're looking for around here."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as("Payon Guide", args!["Please, select a menu."])?;
                if !compass_marked {
                    ctx.mes("Would you like to leave indicators on the mini-map?")?;
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
                    match ctx.menu(&["^FF0000Archer Guild^000000", "Tool Shop", "Payon Dungeon", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args![
                                    "The Archer Guild,",
                                    "found northeast in",
                                    "the Archer Village,",
                                    "handles Job Changes",
                                    "to the Archer Class."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (144, 164, 0, 16776960))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args!["You can find", "a Tool Shop in", "the northeast corner", "of the Archer Village."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (71, 156, 1, 16776960))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args![
                                    "The entrance to",
                                    "the Payon Dungeon",
                                    "is located at the west",
                                    "end of the village."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (34, 132, 2, 16777215))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Payon Guide",
                                args![
                                    "If you'd like to erase the marks on the mini-map, select menu, 'Wipe all indicators on the mini-map'."
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
            }
            2 => {
                ctx.lines_as(
                    "Payon Guide",
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
                    "Payon Guide",
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
                    "Payon Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Payon."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as("Payon Guide", args!["Be safe in", "your travels,", "brave adventurer."])?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("", 255)?;
    ctx.end()
}
