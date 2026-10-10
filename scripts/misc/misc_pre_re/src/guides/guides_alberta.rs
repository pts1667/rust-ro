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

pub fn guide_alb(ctx: &Ctx) -> Script {
    let mut marks_enabled = false;
    let mut shown_before = false;
    let mut running = true;
    ctx.fx().cutin("prt_soldier", 2)?;
    ctx.lines_as(
        "Alberta Guide",
        args![
            "Welcome to Alberta,",
            "the Port City. Feel free",
            "to ask me if you're having",
            "trouble finding anything in",
            "town, or if you just need",
            "guidance around the city."
        ],
    )?;
    while running {
        ctx.next()?;
        match ctx.menu(&["City Guide", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Alberta Guide",
                    args!["Please select", "a location from", "the following menu."],
                )?;
                if !marks_enabled {
                    ctx.lines(args!["Would you like me", "to mark locations", "on your Mini-Map?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Yes", "No"])? == 0 {
                        marks_enabled = true;
                    }
                }
                let mut in_guide = true;
                while in_guide {
                    if shown_before {
                        ctx.next()?;
                    } else {
                        shown_before = true;
                    }
                    match ctx.menu(&["^FF0000Merchant Guild^000000", "Weapon Shop", "Tool Shop", "Inn", "Forge", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Alberta Guide",
                                args![
                                    "The Merchant Guild",
                                    "handles Job Changes",
                                    "to the Merchant Class,",
                                    "and is located in the",
                                    "southwest corner",
                                    "of Alberta."
                                ],
                            )?;
                            if marks_enabled {
                                ctx.call(Function::ViewPoint, args![1, 33, 41, 2, 16711680])?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Alberta Guide",
                                args!["The Weapon Shop", "can be found in the", "southern end of Alberta."],
                            )?;
                            if marks_enabled {
                                ctx.call(Function::ViewPoint, args![1, 117, 37, 3, 16711935])?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Alberta Guide",
                                args![
                                    "The Tool Shop",
                                    "is kind of close",
                                    "to the center of",
                                    "Alberta. It shouldn't",
                                    "be too hard to find."
                                ],
                            )?;
                            if marks_enabled {
                                ctx.call(Function::ViewPoint, args![1, 98, 154, 4, 16711935])?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Alberta Guide",
                                args!["There's an Inn", "at the northern", "end of Alberta", "where you can rest."],
                            )?;
                            if marks_enabled {
                                ctx.call(Function::ViewPoint, args![1, 65, 233, 5, 16711935])?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Alberta Guide",
                                args![
                                    "The Forge in Alberta",
                                    "is in the same building",
                                    "as the Merchant Guild.",
                                    "It's to the southwest."
                                ],
                            )?;
                            if marks_enabled {
                                ctx.call(Function::ViewPoint, args![1, 35, 41, 6, 16711935])?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Alberta Guide",
                                args![
                                    "Please ask me to ''Remove",
                                    "Marks from Mini-Map'' if you",
                                    "no longer wish to have the",
                                    "location marks displayed",
                                    "on your Mini-Map."
                                ],
                            )?;
                            in_guide = false;
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                ctx.call(Function::ViewPoint, args![2, 237, 41, 2, 16711680])?;
                ctx.call(Function::ViewPoint, args![2, 237, 41, 3, 16711935])?;
                ctx.call(Function::ViewPoint, args![2, 46, 345, 4, 16711935])?;
                ctx.call(Function::ViewPoint, args![2, 175, 220, 5, 16711935])?;
                ctx.call(Function::ViewPoint, args![2, 175, 220, 6, 16711935])?;
                marks_enabled = false;
            }
            2 => {
                ctx.lines_as(
                    "Alberta Guide",
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
                    "Alberta Guide",
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
                    "Alberta Guide",
                    args![
                        "On your Mini-Map,",
                        "click on the ''+'' and ''-''",
                        "symbols to zoom in and",
                        "our of your Mini-Map. We",
                        "hope you enjoy your travels",
                        "here in the city of Alberta."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as(
                    "Alberta Guide",
                    args![
                        "Be safe when you",
                        "travel and don't hesitate",
                        "to ask me if you have any",
                        "questions about Alberta."
                    ],
                )?;
                ctx.close_window()?;
                running = false;
            }
            _ => {}
        }
    }
    ctx.fx().cutin("prt_soldier", 255)?;
    ctx.end()
}
