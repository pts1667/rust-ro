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

pub fn randomw(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    match ctx.rand(3)? {
        1 => ctx.warp("lhz_cube", 66, 136)?,
        2 => ctx.warp("lhz_cube", 66, 74)?,
        _ => ctx.warp("lhz_cube", 67, 193)?,
    }
    Err(Stop::End)
}
