use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn wop_master_onangelingwin(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnAngelingWin, Vec::new()).map(|_| ())
}

pub fn wop_master_ondevilingwin(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnDevilingWin, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer5000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer8000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer12000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer32000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer32000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer62000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer62000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer70000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer70000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer75000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer75000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer80000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer80000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer85000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer85000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer90000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer90000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer95000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer95000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer100000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer100000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer700000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer700000, Vec::new()).map(|_| ())
}

pub fn wop_master_ontimer703000(ctx: &Ctx) -> Script {
    wop_master_run(ctx, WopMasterStep::OnTimer703000, Vec::new()).map(|_| ())
}

fn wop_warp_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_warp_a(ctx: &Ctx) -> Script {
    wop_warp_a_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_a_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_a")])?;
    return Err(Stop::End);
}

pub fn wop_warp_a_oninit(ctx: &Ctx) -> Script {
    wop_warp_a_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#wop_warp_a")])?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#wop_warp_a")],
    )?;
    return Err(Stop::End);
}

pub fn wop_warp_a_onenable(ctx: &Ctx) -> Script {
    wop_warp_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_a_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_a")])?;
    return Err(Stop::End);
}

pub fn wop_warp_a_ondisable(ctx: &Ctx) -> Script {
    wop_warp_a_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_a_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("wop_team").set(Val::from(1))?;
    ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(57), Val::from(82)])?;
    return Err(Stop::End);
}

pub fn wop_warp_a_ontouch(ctx: &Ctx) -> Script {
    wop_warp_a_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_d_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_warp_d(ctx: &Ctx) -> Script {
    wop_warp_d_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_d_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_d")])?;
    return Err(Stop::End);
}

pub fn wop_warp_d_oninit(ctx: &Ctx) -> Script {
    wop_warp_d_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_d_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#wop_warp_d")])?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#wop_warp_d")],
    )?;
    return Err(Stop::End);
}

pub fn wop_warp_d_onenable(ctx: &Ctx) -> Script {
    wop_warp_d_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_d_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#wop_warp_d")])?;
    return Err(Stop::End);
}

pub fn wop_warp_d_ondisable(ctx: &Ctx) -> Script {
    wop_warp_d_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn wop_warp_d_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("wop_team").set(Val::from(2))?;
    ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(140), Val::from(82)])?;
    return Err(Stop::End);
}

pub fn wop_warp_d_ontouch(ctx: &Ctx) -> Script {
    wop_warp_d_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_angellium1(ctx: &Ctx) -> Script {
    wop_angellium1_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium1_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("poring_w02"), Val::from("#wop_angellium1::OnMyMobDead")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_angellium1_onreset(ctx: &Ctx) -> Script {
    wop_angellium1_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium1_onangelingspawn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Monster,
        vec![
            Val::from("poring_w02"),
            Val::from(80),
            Val::from(82),
            Val::from("Angeling"),
            Val::from(1766),
            Val::from(1),
            Val::from("#wop_angellium1::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_angellium1_onangelingspawn(ctx: &Ctx) -> Script {
    wop_angellium1_onangelingspawn_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium1_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_a")
        .set((ctx.var("$@wop_deadcount_a").get()? + Val::from(1)))?;
    if ctx.var("$@wop_deadcount_a").get()? == 1 {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("poring_w02"),
                Val::from("Mr. Doppel: The Angeling on the 1st base has been killed! 1 point lost!!"),
                Val::from(0),
                Val::from(65280),
            ],
        )?;
        ctx.call(Function::InitNpcTimer, vec![])?;
    } else {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnDevilingWin")])?;
    }
    return Err(Stop::End);
}

pub fn wop_angellium1_onmymobdead(ctx: &Ctx) -> Script {
    wop_angellium1_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium1_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_a")
        .set((ctx.var("$@wop_deadcount_a").get()?.try_sub(Val::from(1))?))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnAngelingSpawn")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w02"),
            Val::from("Mr. Doppel: The Angeling on the 1st base has been revived! 1 point gained!!"),
            Val::from(0),
            Val::from(65280),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    Ok(Val::from(0))
}

pub fn wop_angellium1_ontimer120000(ctx: &Ctx) -> Script {
    wop_angellium1_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_angellium2(ctx: &Ctx) -> Script {
    wop_angellium2_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium2_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("poring_w02"), Val::from("#wop_angellium2::OnMyMobDead")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_angellium2_onreset(ctx: &Ctx) -> Script {
    wop_angellium2_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium2_onangelingspawn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Monster,
        vec![
            Val::from("poring_w02"),
            Val::from(98),
            Val::from(41),
            Val::from("Angeling"),
            Val::from(1766),
            Val::from(1),
            Val::from("#wop_angellium2::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_angellium2_onangelingspawn(ctx: &Ctx) -> Script {
    wop_angellium2_onangelingspawn_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium2_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_a")
        .set((ctx.var("$@wop_deadcount_a").get()? + Val::from(1)))?;
    if ctx.var("$@wop_deadcount_a").get()? == 1 {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("poring_w02"),
                Val::from("Mr. Doppel: The Angeling on the 2nd base has been killed! 1 point lost!!"),
                Val::from(0),
                Val::from(65280),
            ],
        )?;
        ctx.call(Function::InitNpcTimer, vec![])?;
    } else {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnDevilingWin")])?;
    }
    return Err(Stop::End);
}

pub fn wop_angellium2_onmymobdead(ctx: &Ctx) -> Script {
    wop_angellium2_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn wop_angellium2_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_a")
        .set((ctx.var("$@wop_deadcount_a").get()?.try_sub(Val::from(1))?))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnAngelingSpawn")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w02"),
            Val::from("Mr. Doppel: The Angeling on the 2nd base has been revived! 1 point gained!!"),
            Val::from(0),
            Val::from(65280),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_angellium2_ontimer120000(ctx: &Ctx) -> Script {
    wop_angellium2_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_devillium1(ctx: &Ctx) -> Script {
    wop_devillium1_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium1_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("poring_w02"), Val::from("#wop_devillium1::OnMyMobDead")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_devillium1_onreset(ctx: &Ctx) -> Script {
    wop_devillium1_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium1_ondevilingspawn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Monster,
        vec![
            Val::from("poring_w02"),
            Val::from(116),
            Val::from(82),
            Val::from("Deviling"),
            Val::from(1767),
            Val::from(1),
            Val::from("#wop_devillium1::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_devillium1_ondevilingspawn(ctx: &Ctx) -> Script {
    wop_devillium1_ondevilingspawn_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium1_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_d")
        .set((ctx.var("$@wop_deadcount_d").get()? + Val::from(1)))?;
    if ctx.var("$@wop_deadcount_d").get()? == 1 {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("poring_w02"),
                Val::from("Mr. Doppel: The Deviling on the 1st base has been killed! 1 point lost!!"),
                Val::from(0),
                Val::from(65280),
            ],
        )?;
        ctx.call(Function::InitNpcTimer, vec![])?;
    } else {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnAngelingWin")])?;
    }
    return Err(Stop::End);
}

pub fn wop_devillium1_onmymobdead(ctx: &Ctx) -> Script {
    wop_devillium1_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium1_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_d")
        .set((ctx.var("$@wop_deadcount_d").get()?.try_sub(Val::from(1))?))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OndevilingSpawn")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w02"),
            Val::from("Mr. Doppel: The Deviling on the 1st base has been revived! 1 point gained!!"),
            Val::from(0),
            Val::from(65280),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_devillium1_ontimer120000(ctx: &Ctx) -> Script {
    wop_devillium1_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_devillium2(ctx: &Ctx) -> Script {
    wop_devillium2_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium2_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::KillMonster,
        vec![Val::from("poring_w02"), Val::from("#wop_devillium2::OnMyMobDead")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wop_devillium2_onreset(ctx: &Ctx) -> Script {
    wop_devillium2_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium2_ondevilingspawn_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Monster,
        vec![
            Val::from("poring_w02"),
            Val::from(98),
            Val::from(124),
            Val::from("Deviling"),
            Val::from(1767),
            Val::from(1),
            Val::from("#wop_devillium2::OnMyMobDead"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wop_devillium2_ondevilingspawn(ctx: &Ctx) -> Script {
    wop_devillium2_ondevilingspawn_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium2_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_d")
        .set((ctx.var("$@wop_deadcount_d").get()? + Val::from(1)))?;
    if ctx.var("$@wop_deadcount_d").get()? == 1 {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("poring_w02"),
                Val::from("Mr. Doppel: The deviling on the 2nd base has been killed! 1 point lost!!"),
                Val::from(0),
                Val::from(65280),
            ],
        )?;
        ctx.call(Function::InitNpcTimer, vec![])?;
    } else {
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnAngelingWin")])?;
    }
    return Err(Stop::End);
}

pub fn wop_devillium2_onmymobdead(ctx: &Ctx) -> Script {
    wop_devillium2_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn wop_devillium2_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_deadcount_d")
        .set((ctx.var("$@wop_deadcount_d").get()?.try_sub(Val::from(1))?))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OndevilingSpawn")])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w02"),
            Val::from("Mr. Doppel: The deviling on the 2nd base has been revived! 1 point gained!!"),
            Val::from(0),
            Val::from(65280),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    Ok(Val::from(0))
}

pub fn wop_devillium2_ontimer120000(ctx: &Ctx) -> Script {
    wop_devillium2_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_a_point = Val::from(0);
    let mut l_d_point = Val::from(0);
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_a_point = ctx.var("$@wop_deadcount_a").get()?;
    l_d_point = ctx.var("$@wop_deadcount_d").get()?;
    if ctx
        .call(Function::GetCharacterId, vec![Val::from(1)])?
        .loosely_equals(&ctx.var("$@wop_team_a").get()?)
    {
        if runtime::op(&l_a_point.clone(), "<", &l_d_point.clone())?.is_true() {
            ctx.lines_as(
                "Devi",
                args![
                    "Good work!",
                    "Thanks to you, we increased our chances of victory.",
                    "Please, accept these Poring Coins as a sign of our appreciation."
                ],
            )?;
            ctx.next()?;
            if ctx.var("wop_team").get()? == 1 {
                ctx.lines_as("Devi", args!["Goodbye, my human friend."])?;
                ctx.close_window()?;
                ctx.call(Function::GetItem, vec![Val::from(7539), Val::from(3)])?;
                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Devi",
                    args![
                        "Wait a minute, you look suspicious!!",
                        "Ah, I got it! You are on the side of the Devilings?!",
                        "You are spying on us!",
                        "I will never forgive you!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::PercentHeal, vec![Val::from(99), Val::from(0)])?;
                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                return Err(Stop::End);
            }
        } else if l_a_point.clone().loosely_equals(&l_d_point.clone()) {
            ctx.lines_as(
                "Devi",
                args![
                    "You did the best you could.",
                    "I' ts a shame the battle was tied...",
                    "Even so, thanks for coming to fight for us. Please, accept this Poring Coin."
                ],
            )?;
            ctx.next()?;
            if ctx.var("wop_team").get()? == 1 {
                ctx.lines_as("Devi", args!["Goodbye, my human friend."])?;
                ctx.close_window()?;
                ctx.call(Function::GetItem, vec![Val::from(7539), Val::from(1)])?;
                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Devi",
                    args![
                        "Wait a minute, you look suspicious!!",
                        "Ah, I got it! You are on the side of the Devilings?!",
                        "You are spying on us!",
                        "It must have been you that made us tie the battle!!",
                        "I will never forgive you!"
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::PercentHeal, vec![Val::from(99), Val::from(0)])?;
                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                return Err(Stop::End);
            }
        } else if ctx.var("wop_team").get()? == 1 {
            ctx.lines_as(
                "Devi",
                args![
                    "Well, I guess we had bad luck...",
                    "I cant give you anything, since we lost and everything..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Devi", args!["Goodbye, my human friend."])?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
            return Err(Stop::End);
        } else {
            ctx.lines_as(
                "Devi",
                args![
                    "Wait a minute... There is a traitor here!",
                    "Ah, I got it! You are on the side of the Devilings?!",
                    "You are spying on us, Angelings!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Devi", args!["You should be ashamed!"])?;
            ctx.close_window()?;
            ctx.call(Function::PercentHeal, vec![Val::from(99), Val::from(0)])?;
            ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
            return Err(Stop::End);
        }
    } else {
        if ctx
            .call(Function::GetCharacterId, vec![Val::from(1)])?
            .loosely_equals(&ctx.var("$@wop_team_d").get()?)
        {
            if runtime::op(&l_a_point.clone(), ">", &l_d_point.clone())?.is_true() {
                ctx.lines_as(
                    "Devi",
                    args![
                        "Good work!",
                        "Thanks to you, we increased our chances of victory.",
                        "Please, accept these Poring Coins as a sign of our appreciation."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("wop_team").get()? == 2 {
                    ctx.lines_as("Devi", args!["Goodbye, my human friend."])?;
                    ctx.close_window()?;
                    ctx.call(Function::GetItem, vec![Val::from(7539), Val::from(3)])?;
                    ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Devi",
                        args![
                            "Wait a minute, you look suspicious!!",
                            "Ah, I got it! You are on the side of the Angelings?!",
                            "You are spying on us!!",
                            "I will never forgive you!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::PercentHeal, vec![Val::from(99), Val::from(0)])?;
                    ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                    return Err(Stop::End);
                }
            } else if l_a_point.clone().loosely_equals(&l_d_point.clone()) {
                ctx.lines_as(
                    "Devi",
                    args![
                        "You did the best you could.",
                        "It's a shame the battle was tied...",
                        "Even so, thanks for coming to fight for us. Please, accept this Poring Coin."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("wop_team").get()? == 2 {
                    ctx.lines_as("Devi", args!["Goodbye, my human friend."])?;
                    ctx.close_window()?;
                    ctx.call(Function::GetItem, vec![Val::from(7539), Val::from(1)])?;
                    ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Devi",
                        args![
                            "Wait a minute, you look suspicious!!",
                            "Ah, I got it! You are on the side of the Angelings?!",
                            "You are spying on us!",
                            "It must have been you that made us tie the battle!!",
                            "I will never forgive you!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::PercentHeal, vec![Val::from(99), Val::from(0)])?;
                    ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                    return Err(Stop::End);
                }
            } else if ctx.var("wop_team").get()? == 2 {
                ctx.lines_as(
                    "Devi",
                    args![
                        "Well, I guess we had bad luck...",
                        "I cant give you anything, since we lost and everything..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Devi", args!["I will allow you to return to your human world."])?;
                ctx.close_window()?;
                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Devi",
                    args![
                        "Wait a minute, you look suspicious!!",
                        "Ah, I got it! You are on the side of the Angelings?!",
                        "You are spying on us, Devilings!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Devi", args!["You should be ashamed!"])?;
                ctx.close_window()?;
                ctx.call(Function::PercentHeal, vec![Val::from(99), Val::from(0)])?;
                ctx.call(Function::Warp, vec![Val::from("prt_fild08"), Val::from(150), Val::from(370)])?;
                return Err(Stop::End);
            }
        } else {
            ctx.lines_as(
                "Devi",
                args![
                    "This is weird...Your party name ain't registered.",
                    "Im sorry, but rules are rules. I can't help you if your party ain't registered."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn deviruchi_wop_endmaster(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Deviruchi#wop_endmaster")])?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_oninit(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Deviruchi#wop_endmaster")])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_onenable(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Deviruchi#wop_endmaster")])?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_ondisable(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapWarp,
        vec![Val::from("poring_w02"), Val::from("poring_w02"), Val::from(99), Val::from(196)],
    )?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_ontimer3000(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w02"),
            Val::from("Mr. Doppel: Allow me to offer a souvenir to the humans that fought bravely for the Porings! Please, speak to Devi."),
            Val::from(0),
            Val::from(65280),
        ],
    )?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_ontimer5000(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_ontimer65000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("poring_w02"),
            Val::from("Mr. Doppel: Time is up! Let me teleport you."),
            Val::from(0),
            Val::from(65280),
        ],
    )?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_ontimer65000(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_ontimer65000_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_ontimer68000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapWarp,
        vec![Val::from("poring_w02"), Val::from("poring_w01"), Val::from(112), Val::from(138)],
    )?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_ontimer68000(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_ontimer68000_body(ctx, Vec::new()).map(|_| ())
}

fn deviruchi_wop_endmaster_ontimer68100_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_master::OnStop")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnReset")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnReset")])?;
    ctx.var("$@wop_team_a").set(Val::from(0))?;
    ctx.var("$@wop_team_d").set(Val::from(0))?;
    ctx.var("$@wop_deadcount_a").set(Val::from(0))?;
    ctx.var("$@wop_deadcount_d").set(Val::from(0))?;
    ctx.var("$@wop_teamcount").set(Val::from(0))?;
    ctx.var("$@wop_doorcount_a").set(Val::from(0))?;
    ctx.var("$@wop_doorcount_d").set(Val::from(0))?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_a::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_d::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Mr. Doppel#wop_team_a::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Mr. Doppel#wop_team_d::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_a::OnEnable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_d::OnEnable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_all::OnEnable")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn deviruchi_wop_endmaster_ontimer68100(ctx: &Ctx) -> Script {
    deviruchi_wop_endmaster_ontimer68100_body(ctx, Vec::new()).map(|_| ())
}

fn angeling_guardian_wop_da_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Cutin,
        vec![(Val::from("wop_emb0") + ctx.var("$@wop_doorcount_a").get()?), Val::from(1)],
    )?;
    if ctx.var("$@wop_doorcount_a").get()?.number()? < 4 {
        if ctx.var("wop_team").get()? == 2 {
            ctx.lines(args![
                "There is a device to equip the War Badges.",
                "I can see the empty slots to equip the Badges."
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Equip War Badge.:Cancel.")])? {
                1 => {
                    if ctx.call(Function::CountItem, vec![Val::from(7773)])?.is_true() {
                        if ctx.var("$@wop_doorcount_a").get()?.number()? < 4 {
                            ctx.mes("War Badge equipped.")?;
                            ctx.call(Function::DelItem, vec![Val::from(7773), Val::from(1)])?;
                            ctx.var("$@wop_doorcount_a")
                                .set((ctx.var("$@wop_doorcount_a").get()? + Val::from(1)))?;
                            ctx.call(
                                Function::Cutin,
                                vec![(Val::from("wop_emb0") + ctx.var("$@wop_doorcount_a").get()?), Val::from(1)],
                            )?;
                            if ctx.var("$@wop_doorcount_a").get()? == 4 {
                                ctx.next()?;
                                ctx.mes("Gate Activated.")?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("#aroom_ingate_wop::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("#aroom_outgate_wop::OnEnable")])?;
                            }
                            ctx.close_window()?;
                        } else {
                            ctx.lines(args!["All War Badges equipped.", "No need to equip any more."])?;
                            ctx.close_window()?;
                        }
                    } else {
                        ctx.mes("^4d4dff - War Badge missing. You can obtain a War badge by killing members of the other team. - ^000000")?;
                        ctx.close_window()?;
                    }
                }
                2 => {
                    ctx.mes("Cancel.")?;
                    ctx.close_window()?;
                }
                _ => {}
            }
        } else {
            ctx.lines(args![
                "There is a device to equip a War Badge.",
                "Be carefull to not allow the enemy to take and equip your War Badge here."
            ])?;
            ctx.close_window()?;
        }
    } else {
        ctx.mes("All War Badges have been equipped.")?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn angeling_guardian_wop_da(ctx: &Ctx) -> Script {
    angeling_guardian_wop_da_body(ctx, Vec::new()).map(|_| ())
}

fn angeling_guardian_wop_da_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_doorcount_a").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn angeling_guardian_wop_da_oninit(ctx: &Ctx) -> Script {
    angeling_guardian_wop_da_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_ingate_wop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn aroom_ingate_wop(ctx: &Ctx) -> Script {
    aroom_ingate_wop_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_ingate_wop_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#aroom_ingate_wop")])?;
    return Err(Stop::End);
}

pub fn aroom_ingate_wop_oninit(ctx: &Ctx) -> Script {
    aroom_ingate_wop_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_ingate_wop_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#aroom_ingate_wop")])?;
    return Err(Stop::End);
}

pub fn aroom_ingate_wop_onenable(ctx: &Ctx) -> Script {
    aroom_ingate_wop_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_ingate_wop_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#aroom_ingate_wop")])?;
    return Err(Stop::End);
}

pub fn aroom_ingate_wop_ondisable(ctx: &Ctx) -> Script {
    aroom_ingate_wop_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_ingate_wop_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(99), Val::from(49)])?;
    return Err(Stop::End);
}

pub fn aroom_ingate_wop_ontouch(ctx: &Ctx) -> Script {
    aroom_ingate_wop_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_outgate_wop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn aroom_outgate_wop(ctx: &Ctx) -> Script {
    aroom_outgate_wop_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_outgate_wop_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#aroom_outgate_wop")])?;
    return Err(Stop::End);
}

pub fn aroom_outgate_wop_oninit(ctx: &Ctx) -> Script {
    aroom_outgate_wop_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_outgate_wop_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#aroom_outgate_wop")])?;
    return Err(Stop::End);
}

pub fn aroom_outgate_wop_onenable(ctx: &Ctx) -> Script {
    aroom_outgate_wop_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_outgate_wop_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#aroom_outgate_wop")])?;
    return Err(Stop::End);
}

pub fn aroom_outgate_wop_ondisable(ctx: &Ctx) -> Script {
    aroom_outgate_wop_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn aroom_outgate_wop_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(99), Val::from(54)])?;
    return Err(Stop::End);
}

pub fn aroom_outgate_wop_ontouch(ctx: &Ctx) -> Script {
    aroom_outgate_wop_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn deviling_guardian_wop_dd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::Cutin,
        vec![(Val::from("wop_emb0") + ctx.var("$@wop_doorcount_d").get()?), Val::from(1)],
    )?;
    if ctx.var("$@wop_doorcount_d").get()?.number()? < 4 {
        if ctx.var("wop_team").get()? == 1 {
            ctx.lines(args![
                "There is a device to equip the War Badges.",
                "I can see the empty slots to equip the Badges."
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Equip War Badge.:Cancel.")])? {
                1 => {
                    if ctx.call(Function::CountItem, vec![Val::from(7773)])?.is_true() {
                        if ctx.var("$@wop_doorcount_d").get()?.number()? < 4 {
                            ctx.mes("War Badge equipped.")?;
                            ctx.call(Function::DelItem, vec![Val::from(7773), Val::from(1)])?;
                            ctx.var("$@wop_doorcount_d")
                                .set((ctx.var("$@wop_doorcount_d").get()? + Val::from(1)))?;
                            ctx.call(
                                Function::Cutin,
                                vec![(Val::from("wop_emb0") + ctx.var("$@wop_doorcount_d").get()?), Val::from(1)],
                            )?;
                            if ctx.var("$@wop_doorcount_d").get()? == 4 {
                                ctx.next()?;
                                ctx.mes("Gate Activated.")?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("#droom_ingate_wop::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("#droom_outgate_wop::OnEnable")])?;
                            }
                            ctx.close_window()?;
                        } else {
                            ctx.lines(args!["All War Badges equipped.", "No need to equip any more."])?;
                            ctx.close_window()?;
                        }
                    } else {
                        ctx.mes("^4d4dff - War Badge missing. You can obtain a War badge by killing members of the other team. - ^000000")?;
                        ctx.close_window()?;
                    }
                }
                2 => {
                    ctx.mes("Cancel.")?;
                    ctx.close_window()?;
                }
                _ => {}
            }
        } else {
            ctx.lines(args![
                "There is a device to equip a War Badge.",
                "Be carefull to not allow the enemy to take and equip your War Badge here."
            ])?;
            ctx.close_window()?;
        }
    } else {
        ctx.mes("All War Badges have been equipped.")?;
        ctx.close_window()?;
    }
    ctx.call(Function::Cutin, vec![Val::from(""), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn deviling_guardian_wop_dd(ctx: &Ctx) -> Script {
    deviling_guardian_wop_dd_body(ctx, Vec::new()).map(|_| ())
}

fn deviling_guardian_wop_dd_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.var("$@wop_doorcount_d").set(Val::from(0))?;
    return Err(Stop::End);
}

pub fn deviling_guardian_wop_dd_oninit(ctx: &Ctx) -> Script {
    deviling_guardian_wop_dd_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn droom_ingate_wop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn droom_ingate_wop(ctx: &Ctx) -> Script {
    droom_ingate_wop_body(ctx, Vec::new()).map(|_| ())
}

fn droom_ingate_wop_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#droom_ingate_wop")])?;
    return Err(Stop::End);
}

pub fn droom_ingate_wop_oninit(ctx: &Ctx) -> Script {
    droom_ingate_wop_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn droom_ingate_wop_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#droom_ingate_wop")])?;
    return Err(Stop::End);
}

pub fn droom_ingate_wop_onenable(ctx: &Ctx) -> Script {
    droom_ingate_wop_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn droom_ingate_wop_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#droom_ingate_wop")])?;
    return Err(Stop::End);
}

pub fn droom_ingate_wop_ondisable(ctx: &Ctx) -> Script {
    droom_ingate_wop_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn droom_ingate_wop_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(98), Val::from(116)])?;
    return Err(Stop::End);
}

pub fn droom_ingate_wop_ontouch(ctx: &Ctx) -> Script {
    droom_ingate_wop_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn droom_outgate_wop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn droom_outgate_wop(ctx: &Ctx) -> Script {
    droom_outgate_wop_body(ctx, Vec::new()).map(|_| ())
}

fn droom_outgate_wop_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#droom_outgate_wop")])?;
    return Err(Stop::End);
}

pub fn droom_outgate_wop_oninit(ctx: &Ctx) -> Script {
    droom_outgate_wop_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn droom_outgate_wop_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#droom_outgate_wop")])?;
    return Err(Stop::End);
}

pub fn droom_outgate_wop_onenable(ctx: &Ctx) -> Script {
    droom_outgate_wop_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn droom_outgate_wop_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#droom_outgate_wop")])?;
    return Err(Stop::End);
}

pub fn droom_outgate_wop_ondisable(ctx: &Ctx) -> Script {
    droom_outgate_wop_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn droom_outgate_wop_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("poring_w02"), Val::from(98), Val::from(111)])?;
    return Err(Stop::End);
}

pub fn droom_outgate_wop_ontouch(ctx: &Ctx) -> Script {
    droom_outgate_wop_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn angeling_side_poring_wpa_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn angeling_side_poring_wpa(ctx: &Ctx) -> Script {
    angeling_side_poring_wpa_body(ctx, Vec::new()).map(|_| ())
}

fn deviling_side_marin_wpd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn deviling_side_marin_wpd(ctx: &Ctx) -> Script {
    deviling_side_marin_wpd_body(ctx, Vec::new()).map(|_| ())
}

fn wop_ex_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_ex_1(ctx: &Ctx) -> Script {
    wop_ex_1_body(ctx, Vec::new()).map(|_| ())
}

fn wop_ex_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Porings",
        args![
            "P~ooooooooo -!!!",
            "Let's teach them a lesson, ring!",
            "We are no longer betting our lives for Jellopies, ring!!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Porings", args!["Let's go! Let's fight! Let's win, win, win!!!"])?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa1")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa2")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa3")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa4")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa5")])?,
        ],
    )?;
    ctx.next()?;
    if ctx.var("wop_team").get()? == 1 {
        ctx.lines_as(
            "Porings",
            args![
                "Ohhhhhh, there he is, the warrior that fought for us!!",
                "Oh my god... Nice, ring!",
                "Poring~ Poring~ Poooooo~",
                "Party of Angeling~ Popopo, Poring!"
            ],
        )?;
    } else if ctx.var("wop_team").get()? == 2 {
        ctx.lines_as(
            "Porings",
            args![
                "Wait, YOU! Aren't you on the side of the Devilings?!",
                "Get out now! Leave!!",
                "Booooo~ Boo~ Boooo~"
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa2")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa3")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa4")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SURPRISE")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa5")])?,
            ],
        )?;
    } else {
        ctx.lines_as(
            "Porings",
            args!["Hey, human. How about fighting for us Porings on Angeling's side??!"],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HELP")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_GO")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa2")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HELP")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa3")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_GO")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa4")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_GO")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Angeling Side Poring#wa5")])?,
            ],
        )?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Porings",
        args!["Party of Angeling~ Popopo, Poring!", "Victory~ Victory~ Pooooooo~"],
    )?;
    ctx.next()?;
    ctx.mes("- Porings are cheerfully shouting for the victory. -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wop_ex_1_ontouch(ctx: &Ctx) -> Script {
    wop_ex_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn wop_ex_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wop_ex_2(ctx: &Ctx) -> Script {
    wop_ex_2_body(ctx, Vec::new()).map(|_| ())
}

fn wop_ex_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Marins",
        args![
            "Woooooaaaaaaaaaaa!",
            "Finally, it's time for us to teach those stupid pigs a lesson!!",
            "Those Porings with low-grades are all idiots!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Marins", args!["Let's go! Let's fight! Fight for Triumph!!!!"])?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd1")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd2")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd3")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd4")])?,
        ],
    )?;
    ctx.call(
        Function::Emotion,
        vec![
            ctx.constant("ET_GO")?,
            ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd5")])?,
        ],
    )?;
    ctx.next()?;
    if ctx.var("wop_team").get()? == 2 {
        ctx.lines_as(
            "Marins",
            args![
                "Ohhhh... Here is the warrior that fought for us!!",
                "Amazing!",
                "This proves that humans recognize us as true monsters!!"
            ],
        )?;
    } else if ctx.var("wop_team").get()? == 1 {
        ctx.lines_as(
            "Marins",
            args![
                "Wait, YOU! Aren't you on the side of the Angelings?!",
                "Get out now! Leave!!",
                "Booooo~ Boo~ Boooo~"
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd2")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HUK")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd3")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_FRET")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd4")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_SURPRISE")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd5")])?,
            ],
        )?;
    } else {
        ctx.lines_as(
            "Marins",
            args!["Hey, human. Don't you wanna fight for Deviling, the noble of darkness? What do you say?!"],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HELP")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd1")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_GO")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd2")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_HELP")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd3")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_GO")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd4")])?,
            ],
        )?;
        ctx.call(
            Function::Emotion,
            vec![
                ctx.constant("ET_GO")?,
                ctx.call(Function::GetNpcId, vec![Val::from(0), Val::from("Deviling Side Marin#wd5")])?,
            ],
        )?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Marins",
        args!["The world's Best Miraculous Poring! Deviling has it all! Go, Deviling, Go-!!"],
    )?;
    ctx.next()?;
    ctx.mes("- Marins are cheerfully shouting for the victory. -")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn wop_ex_2_ontouch(ctx: &Ctx) -> Script {
    wop_ex_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}
