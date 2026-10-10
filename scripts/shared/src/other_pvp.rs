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

pub fn f_pvp_fsrs(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut maps: Vec<Val> = Vec::new();
    let mut names: Vec<Val> = Vec::new();
    let mut limits: Vec<Val> = Vec::new();
    if !args.is_empty() {
        let mut levels: Vec<Val> = Vec::new();
        runtime::local_set(&mut levels, &Val::from(0), runtime::arg(&args, 0, Val::from(0)), false);
        runtime::local_set(&mut levels, &Val::from(1), runtime::arg(&args, 1, Val::from(0)), false);
        if runtime::op(
            &ctx.var("BaseLevel").get()?,
            "<",
            &runtime::local_get(&levels, &Val::from(0), false),
        )?
        .is_true()
            || runtime::op(
                &ctx.var("BaseLevel").get()?,
                ">",
                &runtime::local_get(&levels, &Val::from(1), false),
            )?
            .is_true()
        {
            ctx.lines_as(
                "PVP Fight Square Reception Staff",
                args![
                    Val::from("Sorry, but you base level has to be between LV ")
                        + runtime::local_get(&levels, &Val::from(0), false)
                        + " and LV "
                        + runtime::local_get(&levels, &Val::from(1), false)
                        + "."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    if ctx.call(Function::StrNpcInfo, args![4])? == "pvp_y_room" {
        let base_s = Val::from("pvp_y_") + ctx.call(Function::StrNpcInfo, args![2])?;
        runtime::local_set(&mut maps, &Val::from(0), base_s.clone() + "-1", true);
        runtime::local_set(&mut maps, &Val::from(1), base_s.clone() + "-2", true);
        runtime::local_set(&mut maps, &Val::from(2), base_s.clone() + "-3", true);
        runtime::local_set(&mut maps, &Val::from(3), base_s.clone() + "-4", true);
        runtime::local_set(&mut maps, &Val::from(4), base_s + "-5", true);
        runtime::local_set(&mut names, &Val::from(0), Val::from("Prontera"), true);
        runtime::local_set(&mut names, &Val::from(1), Val::from("Izlude"), true);
        runtime::local_set(&mut names, &Val::from(2), Val::from("Payon"), true);
        runtime::local_set(&mut names, &Val::from(3), Val::from("Alberta"), true);
        runtime::local_set(&mut names, &Val::from(4), Val::from("Morocc"), true);
        runtime::local_set(&mut limits, &Val::from(0), Val::from(128), false);
        runtime::local_set(&mut limits, &Val::from(1), Val::from(128), false);
        runtime::local_set(&mut limits, &Val::from(2), Val::from(128), false);
        runtime::local_set(&mut limits, &Val::from(3), Val::from(128), false);
        runtime::local_set(&mut limits, &Val::from(4), Val::from(128), false);
    } else {
        runtime::local_set(&mut maps, &Val::from(0), Val::from("pvp_n_8-1"), true);
        runtime::local_set(&mut maps, &Val::from(1), Val::from("pvp_n_8-2"), true);
        runtime::local_set(&mut maps, &Val::from(2), Val::from("pvp_n_8-3"), true);
        runtime::local_set(&mut maps, &Val::from(3), Val::from("pvp_n_8-4"), true);
        runtime::local_set(&mut maps, &Val::from(4), Val::from("pvp_n_8-5"), true);
        runtime::local_set(&mut names, &Val::from(0), Val::from("Sandwich"), true);
        runtime::local_set(&mut names, &Val::from(1), Val::from("Lock on"), true);
        runtime::local_set(&mut names, &Val::from(2), Val::from("Four Room"), true);
        runtime::local_set(&mut names, &Val::from(3), Val::from("Under cross"), true);
        runtime::local_set(&mut names, &Val::from(4), Val::from("Compass Room"), true);
        runtime::local_set(&mut limits, &Val::from(0), Val::from(64), false);
        runtime::local_set(&mut limits, &Val::from(1), Val::from(32), false);
        runtime::local_set(&mut limits, &Val::from(2), Val::from(32), false);
        runtime::local_set(&mut limits, &Val::from(3), Val::from(32), false);
        runtime::local_set(&mut limits, &Val::from(4), Val::from(32), false);
    }
    let mut menu = Val::from("");
    for i in 0..5 {
        menu = menu
            + runtime::local_get(&names, &Val::from(i), true)
            + " ["
            + ctx.call(Function::GetMapUsers, args![runtime::local_get(&maps, &Val::from(i), true)])?
            + " / "
            + runtime::local_get(&limits, &Val::from(i), false)
            + "]:";
    }
    let choice = runtime::select_values(ctx, &[menu + "Cancel."])? - 1;
    if choice == 5 {
        ctx.close_window()?;
        return Err(Stop::End);
    }
    let index = Val::from(choice);
    if runtime::op(
        &ctx.call(Function::GetMapUsers, args![runtime::local_get(&maps, &index, true)])?,
        ">=",
        &runtime::local_get(&limits, &index, false),
    )?
    .is_true()
    {
        ctx.lines_as("PVP Fight Square Reception Staff", args!["This map is currently full."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Warp, args![runtime::local_get(&maps, &index, true), 0, 0])?;
    Err(Stop::End)
}
