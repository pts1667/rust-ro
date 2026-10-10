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
    ctx.lines_as("Ms. Yoon", lines)?;
    ctx.close()
}

pub fn juno_guide_yuno(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Ms. Yoon",
        args![
            "A place that takes the vision of the future, and gives it form in the present. Welcome to",
            "the city of Juno!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Armory",
        "Tool Shop",
        "Sage Castle (Sage Job Change Place)",
        "Street of Book Stores",
        "Juphero Plaza",
        "Library of the Republic",
        "Schweicherbil Magic Academy",
        "Monster Museum",
        "Forge",
        "Airport",
        "End Conversation",
    ])? {
        0 => show_point(
            ctx,
            120,
            138,
            0,
            16724821,
            args![
                "Please look",
                "at the mini map.",
                "^FF3355+^000000 -> Armory",
                "Thank you,",
                "have a good day."
            ],
        ),
        1 => show_point(
            ctx,
            193,
            142,
            1,
            3364351,
            args![
                "Please look",
                "at the mini map.",
                "^3355FF+^000000 -> Tool Shop",
                "Thank you,",
                "have a good day."
            ],
        ),
        2 => show_point(
            ctx,
            90,
            318,
            2,
            3407701,
            args![
                "Please look",
                "at the mini map.",
                "^33FF55+^000000 -> Sage Castle",
                "( Sage Job Change Place )",
                "Thank you, have a good day."
            ],
        ),
        3 => show_point(
            ctx,
            257,
            102,
            3,
            16724821,
            args![
                "Please look",
                "at the mini map.",
                "^FF3355+^000000 -> Street of Book Stores",
                "Thank you, have a good day."
            ],
        ),
        4 => show_point(
            ctx,
            157,
            170,
            4,
            3364351,
            args![
                "Please look",
                "at the mini map.",
                "^3355FF+^000000 -> Juphero Plaza",
                "Thank you,",
                "have a good day."
            ],
        ),
        5 => show_point(
            ctx,
            336,
            204,
            5,
            3407701,
            args![
                "Please look",
                "at the mini map.",
                "^33FF55+^000000 -> Library of the Republic",
                "Thank you, have a good day."
            ],
        ),
        6 => show_point(
            ctx,
            323,
            281,
            6,
            16724821,
            args![
                "Please look at the mini map.",
                "^FF3355+^000000 -> Schweicherbil Magic Academy",
                "Thank you, have a good day."
            ],
        ),
        7 => show_point(
            ctx,
            278,
            288,
            7,
            3364351,
            args![
                "Please look at the mini map.",
                "^3355FF+^000000 -> Monster Museum",
                "Thank you, have a good day."
            ],
        ),
        8 => show_point(
            ctx,
            120,
            138,
            8,
            16724821,
            args![
                "Please look at the mini map.",
                "^FF3355+^000000 -> Forge",
                "The forge is located underneath Armory.",
                "Thank you, have a good day."
            ],
        ),
        9 => show_point(
            ctx,
            53,
            214,
            9,
            16724821,
            args![
                "Please look at the mini map.",
                "^FF3355+^000000 -> Airport",
                "Thank you, have a good day."
            ],
        ),
        10 => {
            ctx.lines_as(
                "Ms. Yoon",
                args!["A great city of wise men.", "A city of Knowledge!", "Welcome to Juno."],
            )?;
            ctx.close()
        }
        _ => Ok(()),
    }
}
