#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, runtime};

pub fn f_tt(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::compare(&Val::from(ctx.npc().hidden_name()?), &Val::from("main")).is_true() {
        return Err(Stop::End);
    }
    let first = runtime::charat(&Val::from(ctx.npc().map_name()?), &Val::from(6))?;
    let rest = if runtime::strlen(&Val::from(ctx.npc().map_name()?)).number()? > 9 {
        runtime::substr(&Val::from(ctx.npc().map_name()?), &Val::from(8), &Val::from(9))?
    } else {
        let last = runtime::strlen(&Val::from(ctx.npc().map_name()?)).number()? - 1;
        runtime::charat(&Val::from(ctx.npc().map_name()?), &Val::from(last))?
    };
    Ok(first + rest)
}
