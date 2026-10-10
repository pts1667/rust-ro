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

pub fn f_okolnir(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::compare(&ctx.call(Function::StrNpcInfo, args![0])?, &Val::from("main")).is_true() {
        return Err(Stop::End);
    }
    if runtime::compare(&ctx.call(Function::StrNpcInfo, args![4])?, &Val::from("cas")).is_true() {
        return Ok(
            runtime::substr(&ctx.call(Function::StrNpcInfo, args![4])?, &Val::from(0), &Val::from(2))?
                + runtime::substr(&ctx.call(Function::StrNpcInfo, args![4])?, &Val::from(8), &Val::from(9))?,
        );
    }
    runtime::substr(&ctx.call(Function::StrNpcInfo, args![4])?, &Val::from(5), &Val::from(9))
}
