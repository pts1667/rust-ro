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

pub fn f_partyrelay_exp(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let exp = if ctx.player().base_level()? > 94 {
        1047600
    } else if ctx.player().base_level()? > 89 {
        859200
    } else if ctx.player().base_level()? > 79 {
        385200
    } else if ctx.player().base_level()? > 69 {
        195600
    } else if ctx.player().base_level()? > 59 {
        67200
    } else if ctx.player().base_level()? > 49 {
        28800
    } else {
        10800
    };
    ctx.call(Function::GetExperience, args![exp, 0])?;
    return Ok(Val::from(0));
}
