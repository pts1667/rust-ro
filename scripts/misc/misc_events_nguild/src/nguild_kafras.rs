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

pub fn kafra_service_n01(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_kafras::f_gkafra(ctx, args!["nguild_alde", "Prontera"])?;
    ctx.end()
}

pub fn kafra_service_n01_onrecvcastlen01(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetCastleData, args!["nguild_alde", 9])?.number()? < 1 {
        ctx.set_npc_visible("Kafra Service#N01", false)?;
    }
    ctx.end()
}

pub fn kafra_service_n02(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_kafras::f_gkafra(ctx, args!["nguild_gef", "Prontera"])?;
    ctx.end()
}

pub fn kafra_service_n02_onrecvcastlen02(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetCastleData, args!["nguild_gef", 9])?.number()? < 1 {
        ctx.set_npc_visible("Kafra Service#N02", false)?;
    }
    ctx.end()
}

pub fn kafra_service_n03(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_kafras::f_gkafra(ctx, args!["nguild_pay", "Prontera"])?;
    ctx.end()
}

pub fn kafra_service_n03_onrecvcastlen03(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetCastleData, args!["nguild_pay", 9])?.number()? < 1 {
        ctx.set_npc_visible("Kafra Service#N03", false)?;
    }
    ctx.end()
}

pub fn kafra_service_n04(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_kafras::f_gkafra(ctx, args!["nguild_prt", "Prontera"])?;
    ctx.end()
}

pub fn kafra_service_n04_onrecvcastlen04(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetCastleData, args!["nguild_prt", 9])?.number()? < 1 {
        ctx.set_npc_visible("Kafra Service#N04", false)?;
    }
    ctx.end()
}
