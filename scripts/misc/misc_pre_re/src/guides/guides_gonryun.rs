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

fn show_point(ctx: &Ctx, x: i32, y: i32, id: i32, color: i32, lines: Vec<Val>) -> Script {
    ctx.call(Function::ViewPoint, args![1, x, y, id, color])?;
    ctx.lines_as("He Yuen Zhe", lines)?;
    ctx.close()
}

pub fn kunlun_guide_gon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "He Yuen Zhe",
        args![
            "Ni Hao!",
            "Welcome to Kunlun~",
            "Take a walk around and experience",
            "the ancient history and tradition",
            "of our breath taking city."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "He Yuen Zhe",
        args![
            "I am responsible for helping you",
            "with any questions you may have.",
            "Please feel free to ask me anything."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Residence of the Chief",
        "Tool Dealer",
        "Weapon Dealer",
        "Armor Dealer",
        "Wine Maker",
    ])? {
        0 => show_point(
            ctx,
            109,
            131,
            0,
            16724821,
            args![
                "Please follow your minimap, and head over to the ^FF3355+^000000 mark.",
                "There, you'll get to the residence of the Chief. Enjoy your stay in lovely Kunlun!",
                "Xie Xie!"
            ],
        ),
        1 => show_point(
            ctx,
            147,
            82,
            1,
            13525760,
            args![
                "Please follow your minimap, and head over to the ^CE6300+^000000 mark.",
                "There, you'll get to the Tool Dealer. Enjoy your stay in lovely Kunlun!",
                "Xie Xie!"
            ],
        ),
        2 => show_point(
            ctx,
            174,
            104,
            2,
            5635891,
            args![
                "Please follow your minimap, and head over to the ^55FF33+^000000 mark.",
                "There, you'll get to the Weapon Dealer. Enjoy your stay in lovely Kunlun!",
                "Xie Xie!"
            ],
        ),
        3 => show_point(
            ctx,
            173,
            84,
            3,
            3364351,
            args![
                "Please follow your minimap, and head over to the ^3355FF+^000000 mark.",
                "There, you'll get to the Armor Dealer. Enjoy your stay in lovely Kunlun!",
                "Xie Xie!"
            ],
        ),
        4 => show_point(
            ctx,
            215,
            114,
            3,
            13461961,
            args![
                "Please follow your minimap, and head over to the ^CD69C9+^000000 mark.",
                "There, you'll get to the Wine Maker. Enjoy your stay in lovely Kunlun!",
                "Xie Xie!"
            ],
        ),
        _ => Ok(()),
    }
}
