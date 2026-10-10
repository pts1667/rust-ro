use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn paper_sp_6_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while6 = Val::from(0);
    let mut l_paper_x6 = Val::from(0);
    let mut l_paper_y6 = Val::from(0);
    l_paper_while6 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while6.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while6 = (l_paper_while6.clone() + Val::from(1));
                l_paper_x6 = ctx.call(Function::Rand, vec![Val::from(111), Val::from(124)])?;
                l_paper_y6 = ctx.call(Function::Rand, vec![Val::from(73), Val::from(86)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x6.clone(),
                        l_paper_y6.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_6_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_6_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_6_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x6 = Val::from(0);
    let mut l_paper_y6 = Val::from(0);
    l_paper_x6 = ctx.call(Function::Rand, vec![Val::from(111), Val::from(124)])?;
    l_paper_y6 = ctx.call(Function::Rand, vec![Val::from(73), Val::from(86)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x6.clone(),
            l_paper_y6.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_6_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_6_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_7_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_7_a(ctx: &Ctx) -> Script {
    paper_sp_7_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_7_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while7 = Val::from(0);
    let mut l_paper_x7 = Val::from(0);
    let mut l_paper_y7 = Val::from(0);
    l_paper_while7 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while7.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while7 = (l_paper_while7.clone() + Val::from(1));
                l_paper_x7 = ctx.call(Function::Rand, vec![Val::from(81), Val::from(95)])?;
                l_paper_y7 = ctx.call(Function::Rand, vec![Val::from(59), Val::from(72)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x7.clone(),
                        l_paper_y7.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_7_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_7_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_7_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x7 = Val::from(0);
    let mut l_paper_y7 = Val::from(0);
    l_paper_x7 = ctx.call(Function::Rand, vec![Val::from(81), Val::from(95)])?;
    l_paper_y7 = ctx.call(Function::Rand, vec![Val::from(59), Val::from(72)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x7.clone(),
            l_paper_y7.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_7_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_7_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_8_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_8_a(ctx: &Ctx) -> Script {
    paper_sp_8_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_8_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while8 = Val::from(0);
    let mut l_paper_x8 = Val::from(0);
    let mut l_paper_y8 = Val::from(0);
    l_paper_while8 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while8.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while8 = (l_paper_while8.clone() + Val::from(1));
                l_paper_x8 = ctx.call(Function::Rand, vec![Val::from(96), Val::from(110)])?;
                l_paper_y8 = ctx.call(Function::Rand, vec![Val::from(59), Val::from(72)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x8.clone(),
                        l_paper_y8.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_8_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_8_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_8_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x8 = Val::from(0);
    let mut l_paper_y8 = Val::from(0);
    l_paper_x8 = ctx.call(Function::Rand, vec![Val::from(96), Val::from(110)])?;
    l_paper_y8 = ctx.call(Function::Rand, vec![Val::from(59), Val::from(72)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x8.clone(),
            l_paper_y8.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_8_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_8_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_9_a_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn paper_sp_9_a(ctx: &Ctx) -> Script {
    paper_sp_9_a_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_9_a_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_while9 = Val::from(0);
    let mut l_paper_x9 = Val::from(0);
    let mut l_paper_y9 = Val::from(0);
    l_paper_while9 = Val::from(0);
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            if l_paper_while9.clone() == 100 {
                break 'l1;
            } else {
                l_paper_while9 = (l_paper_while9.clone() + Val::from(1));
                l_paper_x9 = ctx.call(Function::Rand, vec![Val::from(111), Val::from(124)])?;
                l_paper_y9 = ctx.call(Function::Rand, vec![Val::from(59), Val::from(72)])?;
                ctx.call(
                    Function::MakeItem,
                    vec![
                        Val::from(6030),
                        Val::from(1),
                        Val::from("arug_que01"),
                        l_paper_x9.clone(),
                        l_paper_y9.clone(),
                    ],
                )?;
            }
        }
    }
    return Err(Stop::End);
}

pub fn paper_sp_9_a_onenable(ctx: &Ctx) -> Script {
    paper_sp_9_a_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn paper_sp_9_a_onbingo_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_x9 = Val::from(0);
    let mut l_paper_y9 = Val::from(0);
    l_paper_x9 = ctx.call(Function::Rand, vec![Val::from(111), Val::from(124)])?;
    l_paper_y9 = ctx.call(Function::Rand, vec![Val::from(59), Val::from(72)])?;
    ctx.call(
        Function::MakeItem,
        vec![
            Val::from(6031),
            Val::from(1),
            Val::from("arug_que01"),
            l_paper_x9.clone(),
            l_paper_y9.clone(),
        ],
    )?;
    return Err(Stop::End);
}

pub fn paper_sp_9_a_onbingo(ctx: &Ctx) -> Script {
    paper_sp_9_a_onbingo_body(ctx, Vec::new()).map(|_| ())
}

fn removepp_aru_gd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn removepp_aru_gd(ctx: &Ctx) -> Script {
    removepp_aru_gd_body(ctx, Vec::new()).map(|_| ())
}

fn removepp_aru_gd_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("removepp_aru_gd")])?;
    return Err(Stop::End);
}

pub fn removepp_aru_gd_oninit(ctx: &Ctx) -> Script {
    removepp_aru_gd_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn removepp_aru_gd_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_paper_aru_gd = Val::from(0);
    let mut l_spaper_aru_gd = Val::from(0);
    l_paper_aru_gd = ctx.call(Function::CountItem, vec![Val::from(6030)])?;
    l_spaper_aru_gd = ctx.call(Function::CountItem, vec![Val::from(6031)])?;
    if (l_paper_aru_gd.clone().number()? > 0 || l_spaper_aru_gd.clone().number()? > 0) {
        ctx.call(Function::DelItem, vec![Val::from(6030), l_paper_aru_gd.clone()])?;
        ctx.call(Function::DelItem, vec![Val::from(6031), l_spaper_aru_gd.clone()])?;
    }
    return Err(Stop::End);
}

pub fn removepp_aru_gd_ontouch(ctx: &Ctx) -> Script {
    removepp_aru_gd_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn eff_mvp_aru_gd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn eff_mvp_aru_gd(ctx: &Ctx) -> Script {
    eff_mvp_aru_gd_body(ctx, Vec::new()).map(|_| ())
}

fn eff_mvp_aru_gd_onmvp_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn eff_mvp_aru_gd_onmvp(ctx: &Ctx) -> Script {
    eff_mvp_aru_gd_onmvp_body(ctx, Vec::new()).map(|_| ())
}

fn eff_mvp_aru_gd_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_1_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_3_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_5_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_7_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_9_a")],
    )?;
    return Err(Stop::End);
}

pub fn eff_mvp_aru_gd_ontimer1000(ctx: &Ctx) -> Script {
    eff_mvp_aru_gd_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn eff_mvp_aru_gd_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_2_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_4_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_6_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_8_a")],
    )?;
    return Err(Stop::End);
}

pub fn eff_mvp_aru_gd_ontimer2000(ctx: &Ctx) -> Script {
    eff_mvp_aru_gd_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn eff_mvp_aru_gd_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_1_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_3_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_5_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_7_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_9_a")],
    )?;
    return Err(Stop::End);
}

pub fn eff_mvp_aru_gd_ontimer3000(ctx: &Ctx) -> Script {
    eff_mvp_aru_gd_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn eff_mvp_aru_gd_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_2_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_4_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_6_a")],
    )?;
    ctx.call(
        Function::NpcSpecialEffect,
        vec![ctx.constant("EF_MVP")?, ctx.constant("AREA")?, Val::from("paper_sp_8_a")],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn eff_mvp_aru_gd_ontimer4000(ctx: &Ctx) -> Script {
    eff_mvp_aru_gd_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gergath_aru_gd(ctx: &Ctx) -> Script {
    gergath_aru_gd_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Gergath#aru_gd")])?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_oninit(ctx: &Ctx) -> Script {
    gergath_aru_gd_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Gergath#aru_gd")])?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FORESTLIGHT")?])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_onenable(ctx: &Ctx) -> Script {
    gergath_aru_gd_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : My dear friend, how have you been? This is something I prepared for you."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer5000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : Do you still remember when we were young? We fought everyday like we were sworn enemies."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer10000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer15000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : No matter how hard the adults tried, we had no intentions to change. Haha."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer15000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer15000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer20000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : But I believe that deep inside, we valued our friendship with each other."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer20000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer20000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer25000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : After I lost my mother's remnants in Mt. Mjolnir, you came to find me."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer25000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer25000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer30000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : When you, handed me my mother's remnants, I couldn't even say thank you."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer30000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer30000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer35000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : I couldn't say it even as I became an old man."),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer35000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer35000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer40000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("arug_que01"),
            Val::from("Gergath : I found out about your whereabouts not long ago. I heard you were taking care of orphaned children?"),
            ctx.constant("BC_MAP")?,
            Val::from("0xFFFF00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer40000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer40000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer45000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::MapAnnounce, vec![Val::from("arug_que01"), Val::from("Gergath : Please accept this as a gift from a useless friend to you and your beloved children. I hope you will like it, haha."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer45000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer45000_body(ctx, Vec::new()).map(|_| ())
}

fn gergath_aru_gd_ontimer50000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::MapAnnounce, vec![Val::from("arug_que01"), Val::from("Gergath : Pierrot Pier will help you. I hope you can use this chance to return to the past and enjoy yourself with your children."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Gergath#aru_gd")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gergath_aru_gd_ontimer50000(ctx: &Ctx) -> Script {
    gergath_aru_gd_ontimer50000_body(ctx, Vec::new()).map(|_| ())
}

fn aru_flower_01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn aru_flower_01(ctx: &Ctx) -> Script {
    aru_flower_01_body(ctx, Vec::new()).map(|_| ())
}

fn event_controller_aru_gd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])?.number()? < 1 {
        ctx.mes("Incorrect password.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.mes("How can I help you?")?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Reset.:No, thanks.")])? {
            1 => {
                ctx.mes("Completed.")?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![Val::from("Monster Controler1#aru::OnControler1#aru_gd")],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![Val::from("Monster Controler1#aru::OnControler1#aru_gd")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Dwarf#aru_gd")])?;
                ctx.var("$@gdeventv_a1").set(Val::from(0))?;
                ctx.var("$@gdeventv_a2").set(Val::from(0))?;
                ctx.var("$@gdevents_a$").set(Val::from(""))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.mes("Good bye~")?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn event_controller_aru_gd(ctx: &Ctx) -> Script {
    event_controller_aru_gd_body(ctx, Vec::new()).map(|_| ())
}
