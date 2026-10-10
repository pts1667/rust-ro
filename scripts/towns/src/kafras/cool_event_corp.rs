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

pub fn cool_event_corp_staff(ctx: &Ctx) -> Script {
    shared::kafras_cool_event_corp::f_cooleventcorp(
        ctx,
        args![
            "Save:Use Storage::Rent a Pushcart:Storage Password Service:Cancel",
            "in the town of Lighthalzen",
            "lighthalzen",
            95,
            240,
        ],
    )?;
    return ctx.end();
}

pub fn cool_event_corp_staff_l184(ctx: &Ctx) -> Script {
    shared::kafras_cool_event_corp::f_cooleventcorp(
        ctx,
        args![
            "Save:Use Storage::Rent a Pushcart:Storage Password Service:Cancel",
            "in the village of Hugel",
            "hugel",
            96,
            155,
        ],
    )?;
    return ctx.end();
}

pub fn cool_event_corp_staff_l191(ctx: &Ctx) -> Script {
    shared::kafras_cool_event_corp::f_cooleventcorp(
        ctx,
        args![
            "Save:Use Storage:Teleport Service:Rent a Pushcart:Storage Password Service:Cancel",
            "in the town of Rachel",
            "rachel",
            113,
            137,
            "Veins",
        ],
    )?;
    return ctx.end();
}

pub fn cool_event_corp_staff_l198(ctx: &Ctx) -> Script {
    shared::kafras_cool_event_corp::f_cooleventcorp(
        ctx,
        args![
            "Save:Use Storage:Teleport Service:Rent a Pushcart:Storage Password Service:Cancel",
            "in the town of Veins",
            "veins",
            204,
            103,
            "Rachel",
        ],
    )?;
    return ctx.end();
}
