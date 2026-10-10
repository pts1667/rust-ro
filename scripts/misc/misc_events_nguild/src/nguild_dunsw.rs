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

pub fn switch_dunn01(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_dunsw::f_glddunsw(ctx, args!["nguild_alde", "02", 32, 122])?;
    ctx.close()
}

pub fn switch_dunn02(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_dunsw::f_glddunsw(ctx, args!["nguild_gef", "04", 39, 258])?;
    ctx.close()
}

pub fn switch_dunn03(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_dunsw::f_glddunsw(ctx, args!["nguild_pay", "01", 186, 165])?;
    ctx.close()
}

pub fn switch_dunn04(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_dunsw::f_glddunsw(ctx, args!["nguild_prt", "03", 28, 251])?;
    ctx.close()
}
