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

pub fn linkflag(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !ctx.call(Function::GetCharacterId, args![2])?.is_true()
        || !ctx.call(Function::GetCharacterId, args![2])?.loosely_equals(&ctx.call(
            Function::GetCastleData,
            args![ctx.call(Function::StrNpcInfo, args![4])?, ctx.constant("CD_GUILD_ID")?],
        )?)
    {
        return Err(Stop::End);
    }
    if runtime::arg(&args, 0, Val::from(0)) == "Convenience Facility" {
        ctx.lines(args![
            "^3355FFThis is the Stronghold",
            "Teleport Service. Would",
            "you like to teleport to the",
            "Convenience Facility for",
            "guild members?^000000"
        ])?;
        if ctx.menu(&["Go to Convenience Facility", "Cancel"])? == 0 {
            ctx.call(
                Function::Warp,
                args![
                    ctx.call(Function::StrNpcInfo, args![4])?,
                    runtime::arg(&args, 1, Val::from(0)),
                    runtime::arg(&args, 2, Val::from(0)),
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::arg(&args, 0, Val::from(0)) == "Emperium Center" {
        ctx.lines(args![
            "^3355FFThis is the Stronghold",
            "Teleport Service. Would",
            "you like to teleport to",
            "the Emperium Center?^000000"
        ])?;
        if ctx.menu(&["Teleport", "Cancel"])? == 0 {
            ctx.call(
                Function::Warp,
                args![
                    ctx.call(Function::StrNpcInfo, args![4])?,
                    runtime::arg(&args, 1, Val::from(0)),
                    runtime::arg(&args, 2, Val::from(0)),
                ],
            )?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFThis is the Stronghold",
        "Teleport Service. Please",
        "choose a destination",
        "within the stronghold.^000000"
    ])?;
    let mut menu = Val::from("");
    for i in (0..args.len()).step_by(3) {
        menu = menu + runtime::arg(&args, i as i32, Val::from(0)) + Val::from(":");
    }
    let choice = runtime::select_values(ctx, &[menu + Val::from("Cancel")])? - 1;
    let cancel_index = args.len() as i32 / 3;
    if choice != cancel_index {
        ctx.call(
            Function::Warp,
            args![
                ctx.call(Function::StrNpcInfo, args![4])?,
                runtime::arg(&args, choice * 3 + 1, Val::from(0)),
                runtime::arg(&args, choice * 3 + 2, Val::from(0)),
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn returnflag(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let map_s = runtime::arg(&args, 0, Val::from(0));
    let str_s = if runtime::compare(&ctx.call(Function::StrNpcInfo, args![4])?, &Val::from("aru")).is_true() {
        Val::from("Arunafeltz")
    } else {
        Val::from("Schwarzwald")
    };
    let gid = ctx.call(Function::GetCastleData, args![map_s.clone(), ctx.constant("CD_GUILD_ID")?])?;
    if !gid.is_true() {
        ctx.lines(args![
            Val::from("[ ") + str_s.clone() + Val::from(" Royal Edict ]"),
            "The Holy Kingdom of",
            str_s.clone() + Val::from(" declares that"),
            "one has yet to claim lordship",
            "over this stronghold. The one",
            "that breaks the Emperium will",
            "be recognized as its new owner."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::GetCharacterId, args![2])?.loosely_equals(&gid) && runtime::arg(&args, 1, Val::from(0)).is_true() {
        ctx.lines_as(
            " Ringing Voice ",
            args!["Courageous one,", "do you wish to return", "to your stronghold?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Return to the Stronghold", "Cancel"])? == 0
            && ctx
                .call(Function::GetCharacterId, args![2])?
                .loosely_equals(&ctx.call(Function::GetCastleData, args![map_s.clone(), ctx.constant("CD_GUILD_ID")?])?)
        {
            let (x, y) = if runtime::compare(&map_s, &Val::from("arug")).is_true() {
                if map_s == "arug_cas01" {
                    (67, 193)
                } else if map_s == "arug_cas02" {
                    (43, 256)
                } else {
                    (121, 318)
                }
            } else if map_s == "schg_cas02" {
                (136, 188)
            } else if map_s == "schg_cas03" {
                (308, 202)
            } else {
                (120, 290)
            };
            ctx.call(Function::Warp, args![map_s.clone(), x, y])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        Val::from("[ ") + str_s.clone() + Val::from(" Royal Edict ]"),
        "The Holy Kingdom of",
        str_s.clone() + Val::from(" decrees that"),
        "this stronghold is owned",
        Val::from("by the ^FF0000") + ctx.call(Function::GetGuildInfo, args![gid.clone(), 0])? + Val::from("^000000 Guild."),
    ])?;
    ctx.next()?;
    ctx.lines(args![
        Val::from("[ ") + str_s.clone() + Val::from(" Royal Edict ]"),
        Val::from("^FF0000") + ctx.call(Function::GetGuildMaster, args![gid.clone()])? + Val::from("^000000 is"),
        Val::from("Guild Master of ^FF0000") + ctx.call(Function::GetGuildInfo, args![gid, 0])? + Val::from("^000000."),
        "Any that object must claim this",
        "stronghold through strength of",
        "steel and magic during the",
        "appointed Guild Siege times."
    ])?;
    ctx.close_window()?;
    Err(Stop::End)
}
