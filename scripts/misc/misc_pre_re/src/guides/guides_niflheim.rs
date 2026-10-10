#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn roaming_man_nif(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Ricael",
        args![
            "You must be lost...",
            "Why would anyone come",
            "to this horrid, dreadful",
            "place on purpose...??"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ricael", args!["Ever since I stumbled fell down into this giant tree, I've suffered endlessly here. I've wasted years in sadness, being unable to escape Niflheim."])?;
    ctx.next()?;
    ctx.lines_as(
        "Ricael",
        args![
            "But in searching for an",
            "escape route, I probably know",
            "this town better than anyone",
            "else. I guess knowing the",
            "layout might help you escape",
            "if it weren't so futile."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Ask building locations.", "Remove marks on the mini-map.", "Cancel."])? {
        0 => {
            ctx.lines_as("Ricael", args!["So, um, which place do you want to know about?"])?;
            ctx.next()?;
            match ctx.menu(&["Witch's castle", "Tool shop", "Weapon shop", "Pub", "Cancel"])? {
                0 => {
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "There. I made a ^FF3355+^000000 mark",
                            "on your mini-map so that you can",
                            "go to the castle where that",
                            "creepy witch lives."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "I went there once, but then I",
                            "ran away and decided that I",
                            "should try to not die as much",
                            "as possible. That's pretty",
                            "much my life goal here in",
                            "Niflheim."
                        ],
                    )?;
                    ctx.call(Function::ViewPoint, args![1, 253, 191, 2, 16724821])?;
                }
                1 => {
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "The Tool shop is located",
                            "at the ^CE6300+^000000 mark I made",
                            "on your mini-map."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "They sell some unique items that",
                            "you cannot find outside of this",
                            "town. Of course, they weren't so",
                            "special once I realized no",
                            "Potion can ease the pain I feel.",
                            "...I wish I was in prison."
                        ],
                    )?;
                    ctx.npc().emotion(constants::ET_KEK)?;
                    ctx.call(Function::ViewPoint, args![1, 217, 196, 3, 13525760])?;
                }
                2 => {
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "The Weapon shop is located",
                            "at the ^55FF33+^000000 mark I made",
                            "on your mini-map."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "They sell some unique items which",
                            "you cannot find outside of this",
                            "town... Of course, fighting",
                            "the monsters here will just",
                            "make them angrier. You may as",
                            "well let them eat you."
                        ],
                    )?;
                    ctx.npc().emotion(constants::ET_KEK)?;
                    ctx.call(Function::ViewPoint, args![1, 216, 171, 4, 5635891])?;
                }
                3 => {
                    ctx.lines_as(
                        "Ricael",
                        args!["The Pub is located at", "the ^3355FF+^000000 mark I've made", "on your mini-map."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "Sometimes I see dead people in the",
                            "Pub enjoying themselves, having a",
                            "good time. I used to be able to",
                            "have fun once, but now all I feel",
                            "is the cold tingle of loneliness",
                            "and despair...every waking moment."
                        ],
                    )?;
                    ctx.call(Function::ViewPoint, args![1, 189, 207, 5, 3364351])?;
                }
                4 => {
                    ctx.lines_as(
                        "Ricael",
                        args![
                            "If you want to remove the location",
                            "marks from your mini-map, please",
                            "choose 'Remove marks on the",
                            "mini-map' from the menu."
                        ],
                    )?;
                }
                _ => {}
            }
        }
        1 => {
            ctx.call(Function::ViewPoint, args![2, 253, 191, 2, 16724821])?;
            ctx.call(Function::ViewPoint, args![2, 217, 196, 3, 13525760])?;
            ctx.call(Function::ViewPoint, args![2, 216, 171, 4, 5635891])?;
            ctx.call(Function::ViewPoint, args![2, 189, 207, 5, 3364351])?;
            ctx.lines_as(
                "Ricael",
                args![
                    "I removed the location marks from",
                    "your mini-map. Go ahead and ask",
                    "me if you want to mark the",
                    "building locations again. It",
                    "helps me ignore the depression",
                    "that gnaws at me constantly."
                ],
            )?;
        }
        2 => {
            ctx.lines_as(
                "Ricael",
                args![
                    "It's not a good idea to search",
                    "Niflheim by yourself...",
                    "At least try to be careful."
                ],
            )?;
        }
        _ => {}
    }
    ctx.close()
}
