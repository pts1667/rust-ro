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

const LANDMARKS: [(i32, i32, i32, i32); 5] = [
    (150, 249, 1, 16711680),
    (115, 149, 2, 16711935),
    (42, 87, 3, 10092543),
    (83, 78, 4, 255),
    (273, 125, 5, 65280),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn rachel_guide(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Rachel Guide",
        args![
            "Welcome to the capital of",
            "Arunafeltz, Rachel where a warm",
            "breath of goddess Freya reaches.",
            "If this is the first time for you",
            "to use the guide services, why",
            "don't you check the \"Notice\" menu first?"
        ],
    )?;
    let mut compass_marked = false;
    let mut village_guide_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["Village Guide", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Rachel Guide",
                    args![
                        "I can tell you any building location as long as it is in Rachel.",
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
                    if village_guide_shown {
                        ctx.next()?;
                    }
                    village_guide_shown = true;
                    match ctx.menu(&["Cheshrumnir", "Inn", "Weapon Shop", "Tool Shop", "Airport", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Rachel Guide",
                                args![
                                    "Cheshrumnir is a holy ground where pope, the incarnation of goddess Freya stays.",
                                    "Take the road to the norh to find the building."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, LANDMARKS[0])?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Rachel Guide",
                                args![
                                    "You can rest your fatigue off the journey in the Inn.",
                                    "The left building next to me is the Inn of Rachel."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, LANDMARKS[1])?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Rachel Guide",
                                args![
                                    "Do you want to check out the weapons that are sold in Rachel?",
                                    "The weapon shop is located nearby the western gate."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, LANDMARKS[2])?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Rachel Guide",
                                args![
                                    "Rachel tool shop sells the best quality potions.",
                                    "It's located nearby the western gate."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, LANDMARKS[3])?;
                            }
                        }
                        4 => {
                            ctx.lines_as("Rachel Guide", args!["The Airport is located outside the eastern gate."])?;
                            if compass_marked {
                                view_point(ctx, 1, LANDMARKS[4])?;
                            }
                        }
                        5 => {
                            ctx.lines_as(
                                "Rachel Guide",
                                args![
                                    "If you like to get rid of all the location marks on your Mini-Map,",
                                    "just ask me again, and choose \"Remove Marks from Mini-Map\" menu."
                                ],
                            )?;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            1 => {
                for landmark in LANDMARKS {
                    view_point(ctx, 2, landmark)?;
                }
                ctx.lines_as(
                    "Rachel Guide",
                    args!["Okay, they are gone now. If you have more locations to ask, just let me know."],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Rachel Guide",
                    args![
                        "When you are using the ''Village Guide'' menu,",
                        "make sure that building locations will be marked on your mini-map at the upper right side of your screen.",
                        "If you cannot see your mini-map, use the short cut key ''ctrl+tab'' or press the ''Map'' button on your basic information windows, okay?",
                        "And you can also zoom out your mini-map by using the ''-'' button in case you cannot view the entire map of the village."
                    ],
                )?;
            }
            3 => {
                ctx.lines_as("Rachel Guide", args!["Hope you have a wonderfull journey", "in Arunafeltz."])?;
                ctx.close_window()?;
                return ctx.close();
            }
            _ => {}
        }
    }
}
