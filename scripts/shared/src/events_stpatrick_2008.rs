#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val};

pub fn f_08stpattyseventbox(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    match ctx.rand_range(1, 6)? {
        1 => ctx.items().give(7915, 1)?,
        2 => {
            ctx.items().give(7915, 1)?;
            ctx.items().give(7916, 1)?;
            ctx.items().give(7720, 1)?;
        }
        3 => ctx.items().give(7720, 1)?,
        4 => {
            ctx.items().give(7915, 1)?;
            ctx.items().give(7916, 1)?;
        }
        5 => ctx.items().give(7916, 2)?,
        6 => ctx.items().give(7915, 2)?,
        _ => return Ok(Val::from(0)),
    }
    Err(Stop::End)
}
