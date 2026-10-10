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

fn show_point(ctx: &Ctx, x: i32, y: i32, id: i32, color: i32, text: &str) -> Script {
    ctx.call(Function::ViewPoint, args![1, x, y, id, color])?;
    ctx.mes(text)
}

pub fn soldier_alde(ctx: &Ctx) -> Script {
    ctx.fx().cutin("prt_soldier", 2)?;
    ctx.lines_as(
        "Al De Baran Guard",
        args![
            "I'm just an ordinary guard",
            "that you could find in any other city. I don't think I even have a name..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Al De Baran Guard",
        args![
            "I am in charge of the Service Guides from the Al De Baran Garrison. Let me guide you",
            "through our town!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Get Location Guide.", "End conversation."])? == 0 {
        ctx.call(Function::ViewPoint, args![1, 61, 229, 0, 16737843])?;
        ctx.call(Function::ViewPoint, args![1, 72, 197, 1, 255])?;
        ctx.call(Function::ViewPoint, args![1, 223, 222, 2, 65535])?;
        ctx.call(Function::ViewPoint, args![1, 233, 105, 3, 5329233])?;
        ctx.call(Function::ViewPoint, args![1, 197, 70, 4, 3364351])?;
        ctx.call(Function::ViewPoint, args![1, 60, 60, 5, 16733525])?;
        ctx.lines(args![
            "^FF6633+^000000 -> Kafra Main Office ",
            "^0000FF+^000000 -> Weapon Shop ",
            "^00FFFF+^000000 -> Sorcerer Guild (Closed)",
            "^515151+^000000 -> Pub",
            "^3355FF+^000000 -> Item Shop",
            "^FF5555+^000000 -> Alchemist Guild"
        ])?;
        ctx.close_window()?;
        ctx.fx().cutin("prt_soldier", 255)?;
        return ctx.end();
    }
    ctx.lines_as(
        "Al De Baran Guard",
        args![
            "We are sworn to protect Al De Baran! May the forces of evil always be crushed by the",
            "righteous fist of good!"
        ],
    )?;
    ctx.close_window()?;
    ctx.fx().cutin("prt_soldier", 255)?;
    ctx.end()
}

pub fn soldier_2alde(ctx: &Ctx) -> Script {
    ctx.fx().cutin("prt_soldier", 2)?;
    ctx.lines_as(
        "Al De Baran Guard",
        args!["I'm just an", "ordinary guard,", "the kind you can", "find in any other city."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Al De Baran Guard",
        args![
            "When I'm not too busy",
            "protecting the Al De Baran",
            "populace, I'm here giving directions to adventurers",
            "like yourself."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Kafra Main Office ",
        "Weapon Shop ",
        "Sorcerer Guild ",
        "Pub ",
        "Item Shop ",
        "Alchemist Guild ",
        "End Conversation ",
    ])? {
        0 => show_point(ctx, 61, 229, 0, 16737843, "^FF6633+^000000 -> Kafra Main Office ")?,
        1 => show_point(ctx, 72, 197, 1, 255, "^0000FF+^000000 -> Weapon Shop ")?,
        2 => show_point(ctx, 223, 222, 2, 65535, "^00FFFF+^000000 -> Sorcerer Guild (Closed)")?,
        3 => show_point(ctx, 233, 105, 3, 5329233, "^515151+^000000 -> Pub ")?,
        4 => show_point(ctx, 197, 70, 4, 3364351, "^3355FF+^000000 -> Item Shop ")?,
        5 => show_point(ctx, 60, 60, 5, 16733525, "^FF5555+^000000 -> Alchemist Guild")?,
        6 => ctx.lines_as(
            "Al De Baran Guard",
            args![
                "We are sworn to",
                "protect Al De Baran!",
                "May the forces of good",
                "always prevail over evil~"
            ],
        )?,
        _ => {}
    }
    ctx.close_window()?;
    ctx.fx().cutin("prt_soldier", 255)?;
    ctx.end()
}
