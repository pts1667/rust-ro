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

pub fn func_bingo(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let row_a = ["@bingo_a1$", "@bingo_a2$", "@bingo_a3$", "@bingo_a4$", "@bingo_a5$"];
    let row_b = ["@bingo_b1$", "@bingo_b2$", "@bingo_b3$", "@bingo_b4$", "@bingo_b5$"];
    let row_c = ["@bingo_c1$", "@bingo_c2$", "@bingo_c3$", "@bingo_c4$", "@bingo_c5$"];
    let row_d = ["@bingo_d1$", "@bingo_d2$", "@bingo_d3$", "@bingo_d4$", "@bingo_d5$"];
    let row_e = ["@bingo_e1$", "@bingo_e2$", "@bingo_e3$", "@bingo_e4$"];
    let mut l_i = runtime::arg(&args, 0, Val::from(0)).number()?;
    while l_i < 26 {
        bingo_copy_row(ctx, 1, &row_a)?;
        if l_i > 5 {
            ctx.lines(args![bingo_row(ctx, &row_a, "]")?])?;
        } else {
            bingo_partial_row(ctx, &row_a, l_i)?;
        }
        bingo_copy_row(ctx, 6, &row_b)?;
        if l_i > 10 {
            ctx.lines(args![bingo_row(ctx, &row_b, "]")?])?;
        } else if l_i < 6 {
            ctx.mes("[   ] [   ] [   ] [   ] [   ]")?;
        } else {
            bingo_partial_row(ctx, &row_b, l_i - 5)?;
        }
        bingo_copy_row(ctx, 11, &row_c)?;
        if l_i > 15 {
            ctx.lines(args![bingo_row(ctx, &row_c, "]")?])?;
        } else if l_i < 11 {
            ctx.mes("[   ] [   ] [   ] [   ] [   ]")?;
        } else {
            bingo_partial_row(ctx, &row_c, l_i - 10)?;
        }
        bingo_copy_row(ctx, 16, &row_d)?;
        if l_i > 20 {
            ctx.lines(args![bingo_row(ctx, &row_d, "]")?])?;
        } else if l_i < 16 {
            ctx.mes("[   ] [   ] [   ] [   ] [   ]")?;
        } else {
            bingo_partial_row(ctx, &row_d, l_i - 15)?;
        }
        bingo_copy_row(ctx, 21, &row_e)?;
        if l_i < 21 {
            ctx.mes("[   ] [   ] [   ] [   ] [   ]")?;
        } else {
            bingo_partial_row(ctx, &row_e, l_i - 20)?;
        }
        ctx.next()?;
        let (bingo_input, _status) = runtime::input_number(ctx, None, None)?;
        if bingo_input.number()? < 1 || bingo_input.number()? > 25 {
            ctx.var("@bingo_case").set(l_i)?;
            return Ok(Val::from(0));
        }
        if l_i > 1 {
            let mut j = l_i;
            while j > 0 {
                if bingo_input.loosely_equals(&ctx.var("@bingoplate").get_at(runtime::index(&Val::from(j - 1))?)?) {
                    ctx.var("@bingo_case").set(l_i)?;
                    return Ok(Val::from(0));
                }
                j -= 1;
            }
        }
        ctx.var("@bingoplate").set_at(runtime::index(&Val::from(l_i))?, bingo_input)?;
        l_i += 1;
    }
    Ok(Val::from(1))
}

pub fn func_bingoresult(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_num = runtime::arg(&args, 0, Val::from(0));
    let suffix = crate::other_global_functions::f_getnumsuffix(ctx, vec![l_num.clone()])?;
    let bingo_number = ctx.var("$bingo").get_at(runtime::index(&(l_num.clone().try_sub(Val::from(1))?))?)?;
    let message =
        Val::from("Eukran: The ") + suffix + Val::from(" number is ") + bingo_number + Val::from(". Please check your Bingo Plate.");
    ctx.call(Function::MapAnnounce, args!["que_bingo", message, 4, 16755540])?;
    for letter in ["a", "b", "c", "d", "e"] {
        ctx.call(Function::EnableNpc, args![l_num.clone() + format!("{letter}#bingo")])?;
    }
    ctx.var("$@bingoresult").set(ctx.var("$@bingoresult").get()? + Val::from(1))?;
    Err(Stop::End)
}

fn bingo_cell(ctx: &Ctx, plate: i32, name: &str) -> Script {
    if ctx.var("@bingoplate").get_at(runtime::index(&Val::from(plate))?)?.number()? < 10 {
        ctx.var(name)
            .set(Val::from("0") + ctx.var("@bingoplate").get_at(runtime::index(&Val::from(plate))?)? + Val::from(""))
    } else {
        ctx.var(name)
            .set(ctx.var("@bingoplate").get_at(runtime::index(&Val::from(plate))?)?)
    }
}

fn bingo_copy_row(ctx: &Ctx, first_plate: i32, names: &[&str]) -> Script {
    for (offset, name) in names.iter().enumerate() {
        bingo_cell(ctx, first_plate + offset as i32, name)?;
    }
    Ok(())
}

fn bingo_row(ctx: &Ctx, names: &[&str], tail: &str) -> Result<Val, Stop> {
    let mut row = Val::from("[");
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            row = row + Val::from("] [");
        }
        row = row + ctx.var(name).get()?;
    }
    Ok(row + Val::from(tail))
}

// Step 1 blanks the row, step n reveals the first n-1 cells of it.
fn bingo_partial_row(ctx: &Ctx, names: &[&str], step: i32) -> Script {
    match step {
        1 => ctx.mes("^ff0000[__]^000000 [   ] [   ] [   ] [   ]"),
        2 => ctx.lines(args![bingo_row(ctx, &names[..1], "] ^ff0000[__]^000000 [   ] [   ] [   ]")?]),
        3 => ctx.lines(args![bingo_row(ctx, &names[..2], "] ^ff0000[__]^000000 [   ] [   ]")?]),
        4 => ctx.lines(args![bingo_row(ctx, &names[..3], "] ^ff0000[__]^000000 [   ]")?]),
        5 => ctx.lines(args![bingo_row(ctx, &names[..4], "] ^ff0000[__]^000000")?]),
        _ => Ok(()),
    }
}
