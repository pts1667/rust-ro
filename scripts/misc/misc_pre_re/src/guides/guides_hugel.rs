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

const MARK_SPOTS: [(i32, i32, i32, i32); 11] = [
    (156, 116, 2, 16711680),
    (104, 79, 3, 16711935),
    (129, 66, 4, 10092543),
    (178, 146, 5, 255),
    (70, 158, 6, 65280),
    (93, 167, 7, 65280),
    (91, 105, 8, 16751103),
    (206, 228, 9, 16750848),
    (52, 91, 10, 16777215),
    (58, 72, 11, 16750848),
    (55, 209, 12, 6750207),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn hugel_guide_granny_huge(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hugel Guide Granny",
        args![
            "Oh, hello~ you are one energetic adventurer.",
            "Welcome to Hugel. I was honored to guide you to this beautiful village.",
            "If this is the first time for you to use the guide services, why don't you check the ''Notice'' menu first?"
        ],
    )?;
    let mut compass_marked = false;
    let mut village_menu_shown = false;
    loop {
        ctx.next()?;
        match ctx.menu(&["Village Guide", "Remove Marks from Mini-Map", "Notice", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Hugel Guide Granny",
                    args![
                        "I can tell you any building location as long as it is in Hugel.",
                        "So where do you want to go?"
                    ],
                )?;
                if !compass_marked {
                    ctx.lines(args!["Would you like me", "to mark locations", "on your Mini-Map?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Yes.", "No."])? == 0 {
                        compass_marked = true;
                    }
                }
                loop {
                    if village_menu_shown {
                        ctx.next()?;
                    }
                    village_menu_shown = true;
                    match ctx.menu(&[
                        "Church",
                        "Inn",
                        "Pub",
                        "Airport",
                        "Weapon Shop",
                        "Tool Shop",
                        "Party Supplies Shop",
                        "^3131FFHunter Job Change Place^000000",
                        "^3131FFShrine Expedition's Place^000000",
                        "Monster Race Arena",
                        "Bingo Game Room",
                        "Cancel",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args!["Well, to me, this Church is rather like a place for old folks like me, you know..."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (156, 116, 2, 16711680))?;
                            }
                        }
                        1 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "Pudding rather than praise.",
                                    "You'd better unpack your stuffs first before you start looking around this village.",
                                    "It is the building right next to me."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (104, 79, 3, 16711935))?;
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "Yes, when you travel, you want to drop by a pub and make new friends.",
                                    "Go east from here, then you will arrive at the pub."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (129, 66, 4, 10092543))?;
                            }
                        }
                        3 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "A while ago, strangers came to village and built that strange airport kind of thing...",
                                    "What do they call it? Airship?"
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (178, 146, 5, 255))?;
                            }
                        }
                        4 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "Well, we have a weapon shop in the center of village.",
                                    "But I don't know if there is any weapon that you find useful."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (70, 158, 6, 65280))?;
                            }
                        }
                        5 => {
                            ctx.lines_as("Hugel Guide Granny", args!["Yes, I love Hugel brand Red Potions. I haven't tasted Red Potions from any other brands yet...hohoho. ", "The tool shop is located in the center of village."])?;
                            if compass_marked {
                                view_point(ctx, 1, (93, 167, 7, 65280))?;
                            }
                        }
                        6 => {
                            ctx.lines_as("Hugel Guide Granny", args!["The party supplies shop is around the center of village.", "Make sure that you will not use any firecracker stuffs near other people, because it is dangerous, you know?"])?;
                            if compass_marked {
                                view_point(ctx, 1, (91, 105, 8, 16751103))?;
                            }
                        }
                        7 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "Oh, are you an aspiring Hunter?",
                                    "Then head northeast following the beach, then you will find the Hunter job change place."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (206, 228, 9, 16750848))?;
                            }
                        }
                        8 => {
                            ctx.lines_as("Hugel Guide Granny", args!["I heard that the shrine expedition is staying in a house at the west.", "They have put some kind of sign in the middle of village, so I guess that they are hiring people for something...", "I wonder what they are doing in here...hmmm."])?;
                            if compass_marked {
                                view_point(ctx, 1, (52, 91, 10, 16777215))?;
                            }
                        }
                        9 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "I also like playing Monster Race games. It is pretty fun, you know?",
                                    "Oh, you haven't tried it yet? No~ you'd better try. Trust me, you will like it."
                                ],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (58, 72, 11, 16750848))?;
                            }
                        }
                        10 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args!["Do you like bingo games? If you do, go visit Euklan's Bingo Game Room."],
                            )?;
                            if compass_marked {
                                view_point(ctx, 1, (55, 209, 12, 6750207))?;
                            }
                        }
                        11 => {
                            ctx.lines_as(
                                "Hugel Guide Granny",
                                args![
                                    "If you like to get rid of all the location marks on your Mini-Map,",
                                    "just ask me again, and choose ''Remove Marks from Mini-Map'' menu."
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
                    "Hugel Guide Granny",
                    args!["Okay, they are gone now. If you have more locations to ask, just let me know."],
                )?;
            }
            2 => {
                ctx.lines_as("Hugel Guide Granny", args!["When you are using the ''Village Guide'' menu, ", "make sure that building locations will be marked on your mini-map at the upper right side of your screen.", "If you cannot see your mini-map, use the short cut key ''ctrl+tab'' or press the ''Map'' button on your basic information windows, okay?", "And you can also zoom out your mini-map by using the ''-'' button in case you cannot view the entire map of the village."])?;
            }
            3 => {
                ctx.lines_as("Hugel Guide Granny", args!["This guide job is pretty exciting. Hohoho~"])?;
                return ctx.close();
            }
            _ => {}
        }
    }
}
