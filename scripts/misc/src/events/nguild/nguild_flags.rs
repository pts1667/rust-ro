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

fn flag_emblem(ctx: &Ctx, castle: &str) -> Script {
    ctx.call(
        Function::FlagEmblem,
        args![ctx.call(Function::GetCastleData, args![castle, 1])?],
    )?;
    ctx.end()
}

pub fn nguild_aldebaran_a1_1(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Al De Baran", "nguild_alde", 218, 170, 1, 1])?;
    ctx.close()
}

pub fn nguild_aldebaran_a1_1_onrecvcastlen01(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_alde")
}

pub fn nguild_aldebaran_a1_6(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Al De Baran", "nguild_alde", 218, 170, 0, 1])?;
    ctx.close()
}

pub fn nguild_aldebaran_a1_6_onrecvcastlen01(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_alde")
}

pub fn nguild_geffen_g1_1(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Geffen", "nguild_gef", 83, 47, 1, 1])?;
    ctx.close()
}

pub fn nguild_geffen_g1_1_onrecvcastlen02(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_gef")
}

pub fn nguild_geffen_g1_6(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Geffen", "nguild_gef", 83, 47, 0, 1])?;
    ctx.close()
}

pub fn nguild_geffen_g1_6_onrecvcastlen02(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_gef")
}

pub fn nguild_payon_f1_1(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Payon", "nguild_pay", 87, 29, 1, 1])?;
    ctx.close()
}

pub fn nguild_payon_f1_1_onrecvcastlen03(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_pay")
}

pub fn nguild_payon_f1_6(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Payon", "nguild_pay", 87, 29, 0, 1])?;
    ctx.close()
}

pub fn nguild_payon_f1_6_onrecvcastlen03(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_pay")
}

pub fn nguild_prontera_p1_1(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Prontera", "nguild_prt", 97, 174, 1, 1])?;
    ctx.close()
}

pub fn nguild_prontera_p1_1_onrecvcastlen04(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_prt")
}

pub fn nguild_prontera_p1_7(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_flags::f_flags(ctx, args!["Prontera", "nguild_prt", 97, 174, 0, 1])?;
    ctx.close()
}

pub fn nguild_prontera_p1_7_onrecvcastlen04(ctx: &Ctx) -> Script {
    flag_emblem(ctx, "nguild_prt")
}
