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

pub fn f_cmdguide(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        runtime::arg(&args, 0, Val::from(0)),
        args![
            "Welcome to Comodo, the",
            "city of dreams and fantasy,",
            "where the nightlife never ends!",
            "I know this area really well,",
            "so let me know if you need",
            "directions anywhere here."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Casino",
        "Hula Dance Stage ^3355FF(Dancer Job Change)^000000",
        "Weapon and Armor Shop",
        "Tool Shop",
        "Tourist Shop",
        "Kafra Co. Western Branch",
        "Chief's House",
        "Pub",
        "Campground",
        "End Conversation",
    ])? {
        0 => show_point(
            ctx,
            140,
            98,
            0,
            16737843,
            args![
                "Please refer to the cross mark,",
                "^FF6633+^000000, on your Mini-Map to find the",
                "Casino, a haven for rest for",
                "weary travlers and the heart",
                "of Comodo's nightlife.",
            ],
        ),
        1 => show_point(
            ctx,
            188,
            168,
            1,
            255,
            args![
                "Please refer to the cross mark,",
                "^0000FF+^000000, on your Mini-Map to find the",
                "Hula Dance Stage, the place",
                "where female Archers can",
                "change jobs to Dancers.",
            ],
        ),
        2 => show_point(
            ctx,
            266,
            70,
            2,
            65535,
            args![
                "Please refer to the cross mark,",
                "^00FFFF+^000000, on your Mini-Map to find the",
                "Weapon and Armor shop. Be",
                "sure to check that shop for",
                "any special items that are",
                "unique to Comodo!",
            ],
        ),
        3 => show_point(
            ctx,
            86,
            128,
            3,
            5329233,
            args![
                "Please refer to the cross mark,",
                "^515151+^000000, on your Mini-Map to find the",
                "Tool Shop. If you've never been",
                "there before, then I suggest",
                "you check it out and stock up",
                "on tools you might need later.",
            ],
        ),
        4 => show_point(
            ctx,
            298,
            124,
            4,
            3364351,
            args![
                "Please refer to the cross mark,",
                "^3355FF+^000000, on your Mini-Map to find the",
                "Tourist Shop where you can ",
                "buy gifts that can only be found in the Comodo region~",
            ],
        ),
        5 => show_point(
            ctx,
            136,
            202,
            5,
            16733525,
            args![
                "Please refer to the cross mark,",
                "^FF5555+^000000, on your Mini-Map to find the",
                "Western branch of the Kafra",
                "Corporation. They offer some",
                "pretty important services that you may want to check out later.",
            ],
        ),
        6 => show_point(
            ctx,
            114,
            294,
            5,
            16733525,
            args![
                "Please refer to the cross mark,",
                "^FF5555+^000000, on your Mini-Map to find the",
                "Chief's House. You're welcome",
                "to visit him, and he's usually",
                "happy to have visitors.",
            ],
        ),
        7 => show_point(
            ctx,
            166,
            298,
            5,
            16733525,
            args![
                "Please refer to the cross mark,",
                "^FF5555+^000000, on your Mini-Map to find the",
                "Pub. There, you can meet other",
                "tourists, relax, and socialize",
                "in an enjoyable environment~",
            ],
        ),
        8 => show_point(
            ctx,
            210,
            308,
            5,
            16733525,
            args![
                "Please refer to the cross mark,",
                "^FF5555+^000000, on your Mini-Map to find the",
                "Campground. Gather with your",
                "family and friends, and enjoy",
                "the special barbeque of",
                "Comodo's camping grounds~",
            ],
        ),
        9 => {
            ctx.lines_as(
                runtime::arg(&args, 0, Val::from(0)),
                args![
                    "Actually, it always looks",
                    "like nighttime in Comodo",
                    "because it's built in a huge",
                    "cave. We don't get any sunlight",
                    "here, but the darkness here is",
                    "more exciting than gloomy~",
                ],
            )?;
            ctx.close_window()?;
            Err(Stop::End)
        }
        _ => Ok(Val::from(0)),
    }
}

// Marks a spot on the Mini-Map, then closes the window and ends the script.
fn show_point(
    ctx: &Ctx,
    x: i32,
    y: i32,
    id: i32,
    color: i32,
    lines: Vec<Val>,
) -> Result<Val, Stop> {
    ctx.call(Function::ViewPoint, args![1, x, y, id, color])?;
    ctx.lines(lines)?;
    ctx.close_window()?;
    Err(Stop::End)
}
