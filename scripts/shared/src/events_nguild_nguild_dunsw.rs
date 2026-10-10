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

pub fn f_glddunsw(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_gid = ctx.call(Function::GetCastleData, args![runtime::arg(&args, 0, Val::from(0)), 1])?;
    if l_gid == 0 {
        ctx.lines_as(
            " Echoing Voice ",
            args![" ' The one who can overcome an ordeal and show true bravery... will find the way... ' "],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        " Echoing Voice ",
        args![" ' Only the one who can show true bravery can take this test. '"],
    )?;
    ctx.next()?;
    ctx.lines(args![" ", "There's a small lever. Will you pull it?"])?;
    ctx.next()?;
    if ctx.menu(&["Pull.", "Do not."])? == 0 {
        if ctx.call(Function::GetCharacterId, args![2])?.loosely_equals(&l_gid) {
            ctx.call(
                Function::Warp,
                args![
                    Val::from("gld_dun") + runtime::arg(&args, 1, Val::from(0)),
                    runtime::arg(&args, 2, Val::from(0)),
                    runtime::arg(&args, 3, Val::from(0)),
                ],
            )?;
            return Err(Stop::End);
        }
        ctx.lines(args![" ", " Nothing happened."])?;
    }
    Ok(Val::from(0))
}
