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
    (237, 41, 0, 65280),
    (237, 41, 1, 255),
    (46, 345, 2, 65280),
    (175, 220, 3, 16711680),
    (134, 221, 4, 16711680),
    (204, 214, 5, 16711680),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn guide_iz(ctx: &Ctx) -> Script {
    ctx.fx().cutin("prt_soldier", 2)?;
    ctx.lines_as(
        "Izlude Guide",
        args![
            "Welcome to Izlude,",
            "Prontera's satellite city.",
            "If you need any guidance",
            "around Izlude, feel free",
            "to ask me at anytime."
        ],
    )?;
    let mut compass_marked = false;
    let mut city_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice.", "Cancel"])? {
            0 => {
                ctx.lines_as("Izlude Guide", args!["Please select", "a location from", "the following menu."])?;
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
                        "^FF0000Swordman Association^000000",
                        "Swordman Hall",
                        "Arena",
                        "Izlude Marina",
                        "Weapon Shop",
                        "Tool Shop",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Izlude Guide",
                                args![
                                    "The Swordman Association",
                                    "is located on an island that is",
                                    "in west Izlude. If you're thinking of changing jobs to Swordman,",
                                    "you should check it out."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (52, 140, 0, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Izlude Guide",
                                args!["The Swordman Hall", "is located in the eastern", "island connected to Izlude."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (214, 130, 1, 65280))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Izlude Guide",
                                args!["Izlude's famous", "Arena is located at the", "northern end of Izlude."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (128, 225, 2, 65280))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Izlude Guide",
                                args![
                                    "You can find the",
                                    "Marina in the northeast",
                                    "part of Izlude. There, you can",
                                    "ride a ship which will take you",
                                    "to Alberta or Byalan Island."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (200, 180, 3, 16711680))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Izlude Guide",
                                args!["You can easily", "find the Weapon Shop", "in northwest Izlude."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (111, 149, 4, 16711935))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Izlude Guide",
                                args!["The Tool Shop shouldn't", "be too hard to find in the", "northeast part of Izlude."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (148, 148, 5, 16711935))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Izlude Guide",
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
                    "Izlude Guide",
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
                    "Izlude Guide",
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
                    "Izlude Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Izlude."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Izlude Guide",
                    args![
                        "Okay then, feel",
                        "free to come to me",
                        "if you ever feel lost",
                        "around Izlude, alright?"
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
