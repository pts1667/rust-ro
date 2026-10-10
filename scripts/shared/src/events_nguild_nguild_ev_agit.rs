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

pub fn f_agitbreak(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let map = runtime::arg(&args, 0, Val::from(0));
    let castle = runtime::arg(&args, 1, Val::from(0));
    let gid = ctx.call(Function::GetCharacterId, args![2])?;
    if gid.number()? <= 0 {
        return Ok(Val::from(0));
    }
    let economy = ctx.call(Function::GetCastleData, args![map.clone(), 2])?.try_sub(Val::from(5))?;
    let economy = if economy.number()? < 0 { Val::from(0) } else { economy };
    ctx.call(Function::SetCastleData, args![map.clone(), 2, economy])?;
    let defence = ctx.call(Function::GetCastleData, args![map.clone(), 3])?.try_sub(Val::from(5))?;
    let defence = if defence.number()? < 0 { Val::from(0) } else { defence };
    ctx.call(Function::SetCastleData, args![map.clone(), 3, defence])?;
    ctx.call(Function::SetCastleData, args![map.clone(), 1, gid.clone()])?;
    ctx.call(
        Function::MapAnnounce,
        args![map.clone(), "The emperium has been destroyed.", constants::BC_MAP, 52479],
    )?;
    ctx.call(
        Function::Announce,
        args![
            Val::from("The [")
                + ctx.call(Function::GetCastleName, args![map.clone()])?
                + Val::from("] castle has been conquered by the [")
                + ctx.call(Function::GetGuildInfo, args![gid.clone(), 0])?
                + Val::from("] guild."),
            constants::BC_ALL,
        ],
    )?;
    ctx.call(Function::DoNpcEvent, args![Val::from("::OnRecvCastle") + castle.clone()])?;
    ctx.call(Function::DisableNpc, args![Val::from("Kafra Staff#") + castle])?;
    for i in 4..=9 {
        ctx.call(Function::SetCastleData, args![map.clone(), i, 0])?;
    }
    if ctx.call(Function::GetGuildSkillLevel, args![gid, 10002])? == 0 {
        for i in 10..=17 {
            ctx.call(Function::SetCastleData, args![map.clone(), i, 0])?;
        }
    }
    Ok(Val::from(0))
}

pub fn f_agitend(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let map = runtime::arg(&args, 0, Val::from(0));
    let castle = runtime::arg(&args, 1, Val::from(0));
    ctx.call(Function::GvgOff, args![map.clone()])?;
    if ctx.call(Function::GetCastleData, args![map.clone(), 1])? == 0 {
        return Ok(Val::from(0));
    }
    ctx.call(
        Function::MapRespawnGuildId,
        args![map.clone(), ctx.call(Function::GetCastleData, args![map.clone(), 1])?, 4],
    )?;
    ctx.call(
        Function::KillMonster,
        args![map, Val::from("Agit_") + castle + Val::from("::OnAgitBreak")],
    )?;
    Err(Stop::End)
}

pub fn f_agitstart(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let map = runtime::arg(&args, 0, Val::from(0));
    let castle = runtime::arg(&args, 1, Val::from(0));
    let empx = runtime::arg(&args, 2, Val::from(0));
    let empy = runtime::arg(&args, 3, Val::from(0));
    ctx.call(
        Function::MapRespawnGuildId,
        args![map.clone(), ctx.call(Function::GetCastleData, args![map.clone(), 1])?, 2],
    )?;
    ctx.call(
        Function::Monster,
        args![
            map.clone(),
            empx,
            empy,
            "Emperium",
            1288,
            1,
            Val::from("Agit_") + castle + Val::from("::OnAgitBreak")
        ],
    )?;
    ctx.call(Function::GvgOn, args![map.clone()])?;
    if ctx.call(Function::GetCastleData, args![map, 1])? != 0 {
        return Ok(Val::from(0));
    }
    Err(Stop::End)
}

pub fn f_guildbreak(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let map = runtime::arg(&args, 0, Val::from(0));
    let castle = runtime::arg(&args, 1, Val::from(0));
    ctx.call(Function::KillMonster, args![map.clone(), "All"])?;
    ctx.call(
        Function::Announce,
        args![
            Val::from("Guild Base [") + ctx.call(Function::GetCastleName, args![map.clone()])? + Val::from("] has been abandoned."),
            0,
        ],
    )?;
    ctx.call(Function::DisableNpc, args![Val::from("Kafra Staff#") + castle])?;
    ctx.call(Function::SetCastleData, args![map, 0, 0])?;
    Ok(Val::from(0))
}
