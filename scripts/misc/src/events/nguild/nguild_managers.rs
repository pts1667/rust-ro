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

pub fn frolo(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_managers::f_gldmanager(ctx, args!["Frolo", "nguild_alde", 119, 223, "N01"])?;
    ctx.close()
}

pub fn leiber(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_managers::f_gldmanager(ctx, args!["Leiber", "nguild_gef", 155, 112, "N02"])?;
    ctx.close()
}

pub fn dundar(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_managers::f_gldmanager(ctx, args!["Dundar", "nguild_pay", 290, 7, "N03"])?;
    ctx.close()
}

pub fn thefton(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_managers::f_gldmanager(ctx, args!["Thefton", "nguild_prt", 15, 209, "N04"])?;
    ctx.close()
}
