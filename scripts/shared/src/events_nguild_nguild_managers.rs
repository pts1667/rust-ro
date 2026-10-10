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

#[derive(Clone, Copy, Debug)]
enum FGldmanagerStep {
    Start,
    CheckKafra,
    HireKafra,
    HireDeclined,
    DismissKafra,
    DismissCancelled,
    KeepKafra,
    AskTreasure,
    TreasureDeclined,
    Farewell,
}

fn npc_tag(npc_name: &Val) -> Val {
    Val::from("[ ") + npc_name.clone() + Val::from(" ]")
}

fn f_gldmanager_run(ctx: &Ctx, mut step: FGldmanagerStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gid = Val::from(0);
    let mut l_map_name_s = Val::from("");
    let mut l_npc_name_s = Val::from("");
    'machine: loop {
        match step {
            FGldmanagerStep::Start => {
                l_npc_name_s = runtime::arg(&args, 0, Val::from(0));
                l_map_name_s = runtime::arg(&args, 1, Val::from(0));
                l_gid = ctx.call(Function::GetCastleData, args![l_map_name_s.clone(), 1])?;
                ctx.call(Function::GetGuildMaster, args![l_gid.clone()])?;
                ctx.lines(args![npc_tag(&l_npc_name_s)])?;
                if l_gid == 0 {
                    ctx.mes("I am waiting for my master.  Brave adventurer, follow your destiny!")?;
                    return Ok(Val::from(0));
                }
                if !ctx.call(Function::GetCharacterId, args![2])?.loosely_equals(&l_gid) {
                    ctx.lines(args![
                        Val::from("I am here to follow ^5533FF")
                            + ctx.call(Function::GetGuildMaster, args![l_gid.clone()])?
                            + Val::from("^000000's command! Hey! Your not even a part of the guild!!"),
                        "Where are the guardians? Destroy these intruders!"
                    ])?;
                    return Ok(Val::from(0));
                }
                if ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 2])? == 0 {
                    ctx.lines(args![
                        Val::from("You're not ^5533FF")
                            + ctx.call(Function::GetGuildMaster, args![l_gid.clone()])?
                            + Val::from("^000000! I am here to follow ^5533FF")
                            + ctx.call(Function::GetGuildMaster, args![l_gid.clone()])?
                            + Val::from("^000000's command only")
                    ])?;
                    return Ok(Val::from(0));
                }
                ctx.lines(args![
                    Val::from("Welcome Master ^5533FF")
                        + ctx.call(Function::GetGuildMaster, args![l_gid.clone()])?
                        + Val::from("^000000 ! I will assist you in any way I can!")
                ])?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Kafra Staff Employment / Dismissal", "Enter Treasure Room", "Cancel"])?;
                ctx.var("@menu").set(choice)?;
                step = match choice {
                    1 => FGldmanagerStep::CheckKafra,
                    2 => FGldmanagerStep::AskTreasure,
                    3 => FGldmanagerStep::Farewell,
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                };
                continue 'machine;
            }
            FGldmanagerStep::CheckKafra => {
                ctx.lines(args![npc_tag(&l_npc_name_s)])?;
                if ctx.call(Function::GetCastleData, args![l_map_name_s.clone(), 9])? == 1 {
                    step = FGldmanagerStep::DismissKafra;
                    continue 'machine;
                }
                if ctx.call(Function::GetGuildSkillLevel, args![l_gid.clone(), 10001])? == 0 {
                    ctx.lines(args![
                        "Master, you don't have a contract with the Kafra Staff Company.",
                        "In order to hire a Kafra, you must first learn the Guild skill ^5533FFContract With Kafra^000000."
                    ])?;
                    return Ok(Val::from(0));
                }
                step = FGldmanagerStep::HireKafra;
                continue 'machine;
            }
            FGldmanagerStep::HireKafra => {
                ctx.mes("Would you like to employ the services of a Kafra? You will need ^5533FF10,000 Zeny^000000 to do so... ")?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Employ Kafra.", "Cancel"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = FGldmanagerStep::HireDeclined;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines(args![npc_tag(&l_npc_name_s)])?;
                if ctx.player().zeny()? < 10000 {
                    ctx.mes("Master, you do not have enough money to employ a Kafra. Employment has been cancelled.")?;
                    return Ok(Val::from(0));
                }
                ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
                ctx.call(
                    Function::EnableNpc,
                    args![Val::from("Kafra Staff#") + runtime::arg(&args, 4, Val::from(0))],
                )?;
                ctx.call(Function::SetCastleData, args![l_map_name_s.clone(), 9, 1])?;
                ctx.mes("You have created a contract with the Kafra Staff Company.")?;
                ctx.next()?;
                ctx.fx().cutin("kafra_01", 2)?;
                ctx.lines_as(
                    " Kafra Staff ",
                    args!["How do you do? I'm here to provide you with helpful service! I'll do the best I can to serve you."],
                )?;
                ctx.next()?;
                ctx.fx().cutin("kafra_01", 255)?;
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "I think the Kafra Staff will benefit our guild members."
                ])?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::HireDeclined => {
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "As you wish Master.  But I suggest we get a Kafra as soon as possible!"
                ])?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::DismissKafra => {
                ctx.mes("Would you like to dismiss the current Kafra?")?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Dismissal", "Cancel"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = FGldmanagerStep::KeepKafra;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.fx().cutin("kafra_01", 2)?;
                ctx.lines_as(
                    " Kafra Staff ",
                    args!["Have I done anything wrong? If I did, will you please forgive me?"],
                )?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Dismiss", "Cancel"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = FGldmanagerStep::DismissCancelled;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines_as(
                    " Kafra Staff ",
                    args!["It's unfortunate that I won't be able to serve your guild anymore...."],
                )?;
                ctx.next()?;
                ctx.call(
                    Function::DisableNpc,
                    args![Val::from("Kafra Staff#") + runtime::arg(&args, 4, Val::from(0))],
                )?;
                ctx.call(Function::SetCastleData, args![l_map_name_s.clone(), 9, 0])?;
                ctx.fx().cutin("kafra_01", 255)?;
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "The Kafra has been dismissed.  But... we should really get a Kafra as soon as possible!"
                ])?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::DismissCancelled => {
                ctx.lines_as(" Kafra Staff ", args!["Thank you master, I'll do my best! ^^."])?;
                ctx.fx().cutin("kafra_01", 255)?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::KeepKafra => {
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "Master, I think you should keep the current Kafra Staff because she is already trying her best to serve us"
                ])?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::AskTreasure => {
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "Would you to go to our Treasure Room? Only you, the Guild Master, are allowed to enter this room."
                ])?;
                ctx.next()?;
                let choice = runtime::select(ctx, &["Enter Treasure room.", "Cancel"])?;
                ctx.var("@menu").set(choice)?;
                match choice {
                    1 => {}
                    2 => {
                        step = FGldmanagerStep::TreasureDeclined;
                        continue 'machine;
                    }
                    _ => return Err(Stop::Error("Invalid menu selection".into())),
                }
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "Please follow me through the secret passage way.",
                    "You must pull down on the secret switch in order to get out."
                ])?;
                ctx.next()?;
                ctx.call(
                    Function::Warp,
                    args![
                        l_map_name_s.clone(),
                        runtime::arg(&args, 2, Val::from(0)),
                        runtime::arg(&args, 3, Val::from(0))
                    ],
                )?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::TreasureDeclined => {
                ctx.lines(args![
                    npc_tag(&l_npc_name_s),
                    "The goods are produced everyday.",
                    "You should get them whenever you can because they might dissapear if you take them at the wrong time."
                ])?;
                return Ok(Val::from(0));
            }
            FGldmanagerStep::Farewell => {
                ctx.lines(args![npc_tag(&l_npc_name_s), "As you wish, master."])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn f_gldmanager(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_gldmanager_run(ctx, FGldmanagerStep::Start, args)
}
