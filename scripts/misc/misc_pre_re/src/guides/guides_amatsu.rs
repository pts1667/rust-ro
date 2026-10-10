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

fn show_point(ctx: &Ctx, speaker: &str, x: i32, y: i32, id: i32, color: i32, lines: Vec<Val>) -> Script {
    ctx.call(Function::ViewPoint, args![1, x, y, id, color])?;
    ctx.lines_as(speaker, lines)?;
    ctx.close()
}

fn ask_destination(ctx: &Ctx, speaker: &str) -> Script {
    match ctx.menu(&["Palace", "Tool Shop", "Weapon Shop", "Bar"])? {
        0 => show_point(
            ctx,
            speaker,
            85,
            235,
            0,
            16724821,
            args![
                "On the mini-map,",
                "go to ^FF3355+^000000",
                "to find the Palace.",
                "Have a good time",
                "in Amatsu."
            ],
        ),
        1 => show_point(
            ctx,
            speaker,
            96,
            118,
            1,
            13525760,
            args![
                "On the mini-map,",
                "go to ^CE6300+^000000",
                "to find the Tool Shop.",
                "Have a good time",
                "in Amatsu."
            ],
        ),
        2 => show_point(
            ctx,
            speaker,
            132,
            117,
            2,
            5635891,
            args![
                "On the mini-map,",
                "go to ^55FF33+^000000",
                "to find the Weapon Shop.",
                "Have a good time",
                "in Amatsu."
            ],
        ),
        3 => show_point(
            ctx,
            speaker,
            217,
            116,
            3,
            3364351,
            args![
                "On the mini-map,",
                "go to ^3355FF+^000000",
                "to find the Bar.",
                "Have a good time",
                "in Amatsu."
            ],
        ),
        _ => Ok(()),
    }
}

pub fn amatsu_guide_ama(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Amachang",
        args!["Welcome to Amatsu,", "the town of kind towners", "and beautiful cherry blossoms."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Amachang",
        args![
            "I'm Amachang,",
            "the 13th Miss Amatsu.",
            "I will guide you about town",
            "as Miss Amatsu.",
            "Please tell me",
            "if you want to know something."
        ],
    )?;
    ctx.next()?;
    ask_destination(ctx, "Amachang")
}

pub fn guide_man_2ama(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Guide Man",
        args!["Welcome, tourist from Midgard.", "I'm the guide of", "our beautiful town, Amatsu."],
    )?;
    ctx.next()?;
    ctx.lines_as("Guide Man", args!["What are you looking for?"])?;
    ctx.next()?;
    ask_destination(ctx, "Guide Man")
}
