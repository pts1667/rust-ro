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

pub fn f_flags(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let gid = ctx.call(Function::GetCastleData, args![runtime::arg(&args, 1, Val::from(0)), 1])?;
    if runtime::arg(&args, 5, Val::from(0)) == 0 {
        return Ok(Val::from(0));
    }
    if gid == 0 {
        ctx.lines(args![
            " [ Edict of the Divine Rune-Midgarts Kingdom ]",
            " ",
            "1. Follow the ordinance of The Divine Rune-Midgarts Kingdom, ",
            "We declare that",
            "there is no formal master of this castle.",
            " ",
            "2. To the one who can ",
            "overcome all trials",
            "and destroy the Emperium,",
            "the king will endow the one with",
            "ownership of this castle."
        ])?;
        return Ok(Val::from(0));
    }
    if ctx.call(Function::GetCharacterId, args![2])?.loosely_equals(&gid) && runtime::arg(&args, 4, Val::from(0)) == 1 {
        ctx.lines(args!["Brave ones...", "Do you wish to return to your honorable place?"])?;
        ctx.next()?;
        if ctx.menu(&["Return to the guild castle.", "Quit."])? == 0 {
            if ctx
                .call(Function::GetCharacterId, args![2])?
                .loosely_equals(&ctx.call(Function::GetCastleData, args![runtime::arg(&args, 1, Val::from(0)), 1])?)
            {
                ctx.call(
                    Function::Warp,
                    args![
                        runtime::arg(&args, 1, Val::from(0)),
                        runtime::arg(&args, 2, Val::from(0)),
                        runtime::arg(&args, 3, Val::from(0))
                    ],
                )?;
            }
        }
        return Ok(Val::from(0));
    }
    ctx.lines_as(
        " Edict of the Divine Rune-Midgarts Kingdom ",
        args![
            " ",
            "1. Following the ordinance of the",
            "Divine Rune-Midgarts Kingdom,",
            "we approve that this place is in",
            Val::from("the private possession of ^ff0000")
                + ctx.call(Function::GetGuildInfo, args![gid.clone(), 0])?
                + Val::from("^000000 Guild."),
            " ",
            Val::from("2. The guild Master of ^ff0000")
                + ctx.call(Function::GetGuildInfo, args![gid.clone(), 0])?
                + Val::from("^000000 Guild is"),
            Val::from("^FF0000") + ctx.call(Function::GetGuildMaster, args![gid])? + Val::from("^000000"),
            "If there is anyone who objects to this,",
            " prove your strength and honor with a steel blade in your hand."
        ],
    )?;
    Ok(Val::from(0))
}
