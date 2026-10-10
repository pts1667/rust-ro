#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args};

pub fn fight_square_reception_8(ctx: &Ctx) -> Script {
    shared::other_pvp::f_pvp_fsrs(ctx, vec![])?;
    ctx.end()
}

pub fn fight_square_reception_8_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::WaitingRoom, args!["Free for all", 0])?;
    ctx.end()
}

pub fn fight_square_reception_n(ctx: &Ctx) -> Script {
    shared::other_pvp::f_pvp_fsrs(ctx, vec![])?;
    ctx.end()
}

pub fn fight_square_reception_n_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::WaitingRoom, args!["Free for all", 0])?;
    ctx.end()
}
