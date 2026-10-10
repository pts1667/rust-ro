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
    (196, 258, 1, 16711680),
    (128, 266, 2, 16711935),
    (150, 175, 3, 10092543),
    (230, 161, 4, 255),
    (273, 285, 5, 65280),
    (150, 217, 6, 65280),
    (150, 175, 7, 65280),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn veins_guide_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Veins Guide",
        args![
            "Desert City Veins welcomes adventurers seeking shelter from harsh sandstorms.",
            "If this is the first time for you to use the guide services, why don't you check the..."
        ],
    )?;
    let mut compass_marked = false;
    let mut village_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["Village Guide", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Veins Guide",
                    args![
                        "I can tell you any building location as long as it is in Veins.",
                        "So where do you want to go?"
                    ],
                )?;
                if !compass_marked {
                    ctx.lines(args!["Would you like me", "to mark locations", "on your Mini-Map?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Yes", "No"])? == 0 {
                        compass_marked = true;
                    }
                }
                loop {
                    if village_menu_shown {
                        ctx.next()?;
                    }
                    village_menu_shown = true;
                    match ctx.menu(&[
                        "Temple",
                        "Inn",
                        "Weapon Shop",
                        "Tool Shop",
                        "Airship",
                        "Tavern",
                        "Geological Research Institute",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "Our temple is located north,",
                                    "and always crowded with sincere followers of Goddess Freya."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (196, 258, 1, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "You can rest your fatigue of the journey in the Inn.",
                                    "The left building next to me is the Inn of Veins."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (128, 266, 2, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "Yes, you should protect yourself from danger on your own.",
                                    "Purchase high quality weapons at affordable prices.",
                                    "The Veins Weapon Shop is located to the West."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (150, 175, 3, 10092543))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "Have you packed enough necessities  for your adventure?",
                                    "If not, I suggest you check what the Veins in the Center can offer you."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (230, 161, 4, 255))?;
                            }
                        }
                        4 => {
                            ctx.lines_as("Veins Guide", args!["Please be aware that Veins only operates cargo airships."])?;
                            if compass_marked {
                                view_point(ctx, 1, (273, 285, 5, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "If you'd like to make friends with",
                                    "the townspeople, I suggest you",
                                    "go have a drink at the tavern to",
                                    "the west."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (150, 217, 6, 65280))?;
                            }
                        }
                        6 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "Are you interested in studying geology?",
                                    "Then you'd better go check out the",
                                    "Geological Research Institute on",
                                    "the 2nd floor of the weapon shop."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (150, 175, 7, 65280))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Veins Guide",
                                args![
                                    "If you like to get rid of all the location marks on your Mini-Map,",
                                    "just ask me again, and choose 'Remove Marks from Mini-Map' menu."
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
                ctx.lines_as(
                    "Veins Guide",
                    args![
                        "Okay, they are gone now. If you have more locations to ask, just let me know.",
                        "Enjoy your stay in Veins."
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Veins Guide",
                    args![
                        "When you are using the ''Village Guide'' menu,",
                        "make sure that building locations will be marked on your mini-map at the upper right side of your screen.",
                        "If you cannot see your mini-map, use the short cut key ''ctrl+tab'' or press the ''Map'' button on your basic information windows, okay?",
                        "And you can also zoom out your mini-map by using the ''-'' button in case you cannot view the entire map of the village."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as("Veins Guide", args!["Enjoy your stay in Veins."])?;
                ctx.close_window()?;
                break;
            }
            _ => {}
        }
    }
    ctx.close()
}
