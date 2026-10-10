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
    (218, 255, 2, 16711935),
    (263, 94, 3, 16711935),
    (309, 80, 4, 16711935),
    (145, 174, 5, 16711935),
    (135, 98, 6, 16711935),
    (280, 167, 7, 16711935),
];

fn view_point(ctx: &Ctx, action: i32, (x, y, id, color): (i32, i32, i32, i32)) -> Result<(), Stop> {
    ctx.call(Function::ViewPoint, args![action, x, y, id, color])?;
    Ok(())
}

pub fn soldier_ba(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Soldier",
        args!["Welcome to Luoyang,", "a city with a long", "and colorful history."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Soldier",
        args![
            "Recently we've developed",
            "an ocean lane to accomodate",
            "positive exchange with",
            "foreign nations."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Soldier", args!["Luoyang is well-known for various specialties in addition to its rich history. Here you can find many things unique to our land."])?;
    ctx.next()?;
    ctx.lines_as(
        "Soldier",
        args!["Please take your time", "and we invite you to enjoy", "your trip here in Luoyang."],
    )?;
    ctx.next()?;
    match ctx.menu(&["Ask Building Locations.", "Remove all marks from mini-map.", "Cancel."])? {
        0 => {
            ctx.lines_as("Soldier", args!["Where would you like to go?"])?;
            ctx.next()?;
            match ctx.menu(&[
                "Dragon Castle",
                "Doctor's Office",
                "City Hall",
                "Weapon Shop",
                "Tool Shop",
                "Tavern",
                "Cancel",
            ])? {
                0 => {
                    ctx.lines_as(
                        "Soldier",
                        args!["The Dragon Castle is located at ^FF3355+^000000. It is where all the nobles reside, including our lord."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args!["Since you're an outsider, I guess it would be appropriate for you to visit our lord first."],
                    )?;
                    view_point(ctx, 1, (218, 255, 2, 16724821))?;
                }
                1 => {
                    ctx.lines_as(
                        "Soldier",
                        args!["We have a very skillful doctor.", "You can find her office at ^CE6300+^000000."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "It is said that there",
                            "is no disease she cannot cure. Well, I can't guarantee if that's true or not."
                        ],
                    )?;
                    view_point(ctx, 1, (263, 94, 3, 13525760))?;
                }
                2 => {
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "We have a City Hall where the federal government operates.",
                            "It is located at ^A5BAAD+^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args!["If you have any problems,", "you should talk with the", "employees in City Hall."],
                    )?;
                    view_point(ctx, 1, (309, 80, 4, 10861229))?;
                }
                3 => {
                    ctx.lines_as("Soldier", args!["The Weapon Shop is located at ^55FF33+^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "You will see",
                            "marvelous weapons forged",
                            "by the well-experienced",
                            "blacksmiths of Luoyang."
                        ],
                    )?;
                    view_point(ctx, 1, (145, 174, 5, 5635891))?;
                }
                4 => {
                    ctx.lines_as("Soldier", args!["The Tool Shop is located at ^3355FF+^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "Knowing your enemy",
                            "is half the battle!",
                            "It's also safer to prepare yourself than to be sorry later. Why don't you go check their supplies?"
                        ],
                    )?;
                    view_point(ctx, 1, (135, 98, 6, 3364351))?;
                }
                5 => {
                    ctx.lines_as(
                        "Soldier",
                        args!["When you get tired during your trip, I suggest that you visit the Tavern. It's located at ^00FF00+^000000."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "The Tavern is a good place",
                            "to meet other tourists, as well as to hear of any news that may be helpful to know."
                        ],
                    )?;
                    view_point(ctx, 1, (280, 167, 7, 65280))?;
                }
                6 => {
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "If you wish to remove all marks",
                            "on your mini-map, please choose 'Remove all marks from mini-map.' from the menu."
                        ],
                    )?;
                }
                _ => {}
            }
        }
        1 => {
            for spot in MARK_SPOTS {
                view_point(ctx, 2, spot)?;
            }
            ctx.lines_as(
                "Soldier",
                args![
                    "There, I've erased all the marks on your mini-map. Feel free to ask me about building locations whenever you need to."
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "Soldier",
                args!["I guess it's fun", "sometimes to go exploring", "on your own. Take care."],
            )?;
        }
        _ => {}
    }
    ctx.close()
}

pub fn representative_lou(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Representative",
        args![
            "Welcome to Luoyang,",
            "an ancient land with",
            "a history full of tales",
            "of bravery."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Representative",
        args!["We now provide an ocean lane to accomodate foreign travelers and intercultural exchange from which all can benefit."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Representative",
        args![
            "Luoyang is famous for",
            "its elaborate history, as well as specialties that are unique to this nation. Please take your time and enjoy your stay."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Ask Building Locations.", "Remove all marks from mini-map.", "Cancel."])? {
        0 => {
            ctx.lines_as("Representative", args!["Where would you like to go?"])?;
            ctx.next()?;
            match ctx.menu(&[
                "Dragon Castle",
                "Doctor's Office",
                "City Hall",
                "Weapon Shop",
                "Tool Shop",
                "Tavern",
                "Cancel",
            ])? {
                0 => {
                    ctx.lines_as(
                        "Representative",
                        args!["The Dragon Castle is located at ^FF3355+^000000. It is where all the nobles reside, including our lord."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Representative",
                        args!["Since you're an outsider, I guess it would be appropriate for you to visit our lord first."],
                    )?;
                    view_point(ctx, 1, (218, 255, 2, 16777011))?;
                }
                1 => {
                    ctx.lines_as(
                        "Representative",
                        args!["We have a very skillful doctor.", "You can find her office at ^CE6300+^000000."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Representative",
                        args![
                            "It is said that there",
                            "is no disease she cannot cure. Well, I can't guarantee if that's true or not."
                        ],
                    )?;
                    view_point(ctx, 1, (263, 94, 3, 16764515))?;
                }
                2 => {
                    ctx.lines_as(
                        "Representative",
                        args![
                            "We have a City Hall where the federal government operates.",
                            "It is located at ^A5BAAD+^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Representative",
                        args!["If you have any problems,", "you should talk with the", "employees in City Hall."],
                    )?;
                    view_point(ctx, 1, (309, 80, 4, 16711935))?;
                }
                3 => {
                    ctx.lines_as("Representative", args!["The Weapon Shop is located at ^55FF33+^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Representative",
                        args![
                            "You will see",
                            "marvelous weapons forged",
                            "by the well-experienced",
                            "blacksmiths of Luoyang."
                        ],
                    )?;
                    view_point(ctx, 1, (145, 174, 5, 16733695))?;
                }
                4 => {
                    ctx.lines_as("Representative", args!["The Tool Shop is located at ^3355FF+^000000."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Representative",
                        args![
                            "Knowing your enemy",
                            "is half the battle!",
                            "It's also safer to prepare yourself than to be sorry later. Why don't you go check their supplies?"
                        ],
                    )?;
                    view_point(ctx, 1, (135, 98, 6, 16724821))?;
                }
                5 => {
                    ctx.lines_as(
                        "Representative",
                        args!["When you get tired during your trip, I suggest that you visit the Tavern. It's located at ^00FF00+^000000."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Soldier",
                        args![
                            "The Tavern is a good place",
                            "to meet other tourists, as well as to hear of any news that may be helpful to know."
                        ],
                    )?;
                    view_point(ctx, 1, (280, 167, 7, 16711935))?;
                }
                6 => {
                    ctx.lines_as(
                        "Representative",
                        args![
                            "If you wish to remove all marks",
                            "on your mini-map, please choose 'Remove all marks from mini-map.' from the menu."
                        ],
                    )?;
                }
                _ => {}
            }
        }
        1 => {
            for spot in MARK_SPOTS {
                view_point(ctx, 2, spot)?;
            }
            ctx.lines_as(
                "Representative",
                args![
                    "Done! All the marks on your mini-map are erased. Feel free to ask me about building locations whenever you need to."
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "Representative",
                args![
                    "I understand that you want to explore Luoyang and see the",
                    "sights for yourself. Alright then, take care!"
                ],
            )?;
        }
        _ => {}
    }
    ctx.close()
}
