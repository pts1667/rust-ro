#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn f_gm_npc(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    let mut l_x = Val::from(0);
    let mut l_y = Val::from(0);
    if ctx.call(Function::GetGmLevel, vec![])?.number()? < 99 {
        let position = ctx
            .call(Function::GetMapXy, args![constants::BL_NPC])?
            .into_array()
            .ok_or_else(|| Stop::Error("Invalid position".into()))?;
        l_map_s = position[0].clone();
        l_x = position[1].clone();
        l_y = position[2].clone();
        return Err(Stop::End);
    }
    if args.is_empty() {
        return Ok(Val::from(0));
    }
    if runtime::arg(&args, 1, Val::from(0)) == 0 {
        let input = if runtime::arg(&args, 3, Val::from(0)).is_true() {
            let (input, status) = runtime::input_number(
                ctx,
                Some(runtime::arg(&args, 2, Val::from(0)).number()?),
                Some(runtime::arg(&args, 3, Val::from(0)).number()?),
            )?;
            if status != 0 {
                return Ok(Val::from(-2));
            }
            input
        } else {
            runtime::input_number(ctx, None, None)?.0
        };
        if input == 0 {
            return Ok(Val::from(-1));
        }
        return Ok(Val::from(input.loosely_equals(&runtime::arg(&args, 0, Val::from(0)))));
    }
    let (input, _) = runtime::input_text(ctx, None, None)?;
    Ok(Val::from(input.loosely_equals(&runtime::arg(&args, 0, Val::from(0)))))
}
