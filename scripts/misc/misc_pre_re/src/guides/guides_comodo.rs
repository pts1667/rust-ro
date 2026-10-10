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

pub fn comodo_guide_cmd(ctx: &Ctx) -> Script {
    shared::pre_re_guides_guides_comodo::f_cmdguide(ctx, args!["Native Kokomo"])?;
    Ok(())
}

pub fn comodo_guide_2cmd(ctx: &Ctx) -> Script {
    shared::pre_re_guides_guides_comodo::f_cmdguide(ctx, args!["Native Nutcoco"])?;
    Ok(())
}

pub fn guide_2cmd(ctx: &Ctx) -> Script {
    shared::pre_re_guides_guides_comodo::f_cmdguide(ctx, args!["Native Papaya"])?;
    Ok(())
}
