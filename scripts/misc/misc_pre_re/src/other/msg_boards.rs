#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args};

pub fn sign_iz1(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "Welcome to the Swordsman Academy."])?;
    ctx.close()
}

pub fn sign_iz2(ctx: &Ctx) -> Script {
    ctx.lines(args!["^993300- The Sign Reads -^000000", "Welcome."])?;
    ctx.close()
}
