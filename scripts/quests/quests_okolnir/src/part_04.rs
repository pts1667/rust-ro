use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guard_of_shadow_main_all_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_in = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_in = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_0")) + l_in.clone())],
    )?;
    let subject1 = l_in.clone();
    if subject1 == 1 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(234), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(284), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(235), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(285), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(236), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(286), false);
    } else if subject1 == 2 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(223), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(289), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(224), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(290), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(225), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(291), false);
    } else if subject1 == 3 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(235), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(295), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(236), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(296), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(237), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(297), false);
    } else if subject1 == 4 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(224), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(302), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(225), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(303), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(226), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(304), false);
    }
    l_i = Val::from(0);
    'l2: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
            break 'l2;
        }
        'b2: {
            ctx.call(
                Function::Monster,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_n, &l_i.clone(), false),
                    runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                    Val::from("Guard of Shadow"),
                    Val::from(1752),
                    Val::from(1),
                    ((((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_0")) + l_in.clone()) + Val::from("::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn guard_of_shadow_main_all_ontouch(ctx: &Ctx) -> Script {
    guard_of_shadow_main_all_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn guard_of_shadow_main_all_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_in = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_in = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
    )?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_0")) + l_in.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_0")) + l_in.clone())],
    )?;
    return Err(Stop::End);
}

pub fn guard_of_shadow_main_all_ondisable(ctx: &Ctx) -> Script {
    guard_of_shadow_main_all_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn guard_of_shadow_main_all_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn guard_of_shadow_main_all_onmymobdead(ctx: &Ctx) -> Script {
    guard_of_shadow_main_all_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn guard_of_shadow_main_all_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_i = Val::from(1);
    'l1: loop {
        if !(l_i.clone().number()? <= 4) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::DisableNpc,
                vec![(((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_0")) + l_i.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn guard_of_shadow_main_all_oninit(ctx: &Ctx) -> Script {
    guard_of_shadow_main_all_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn bloody_hunter_main_all_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn bloody_hunter_main_all(ctx: &Ctx) -> Script {
    bloody_hunter_main_all_body(ctx, Vec::new()).map(|_| ())
}

fn bloody_hunter_main_all_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_in = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_in = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone())],
    )?;
    'b1: {
        let subject1 = l_in.clone();
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(265), false);
            runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(297), false);
            runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(266), false);
            runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(296), false);
            runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(267), false);
            runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(295), false);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(270), false);
            runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(284), false);
            runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(271), false);
            runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(283), false);
            runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(272), false);
            runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(282), false);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(269), false);
            runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(308), false);
            runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(270), false);
            runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(307), false);
            runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(271), false);
            runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(306), false);
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(278), false);
            runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(301), false);
            runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(279), false);
            runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(300), false);
            runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(280), false);
            runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(299), false);
        }
    }
    l_i = Val::from(0);
    'l2: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
            break 'l2;
        }
        'b2: {
            ctx.call(
                Function::Monster,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_n, &l_i.clone(), false),
                    runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                    Val::from("Bloody Hunter"),
                    Val::from(1753),
                    Val::from(1),
                    ((((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone()) + Val::from("::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn bloody_hunter_main_all_ontouch(ctx: &Ctx) -> Script {
    bloody_hunter_main_all_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn bloody_hunter_main_all_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_in = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_in = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
    )?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone())],
    )?;
    return Err(Stop::End);
}

pub fn bloody_hunter_main_all_ondisable(ctx: &Ctx) -> Script {
    bloody_hunter_main_all_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn bloody_hunter_main_all_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn bloody_hunter_main_all_onmymobdead(ctx: &Ctx) -> Script {
    bloody_hunter_main_all_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn bloody_hunter_main_all_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_i = Val::from(1);
    'l1: loop {
        if !(l_i.clone().number()? <= 4) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::DisableNpc,
                vec![(((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac0")) + l_i.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn bloody_hunter_main_all_oninit(ctx: &Ctx) -> Script {
    bloody_hunter_main_all_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn temple_keeper_main_all_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn temple_keeper_main_all(ctx: &Ctx) -> Script {
    temple_keeper_main_all_body(ctx, Vec::new()).map(|_| ())
}

fn temple_keeper_main_all_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_in = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_in = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone())],
    )?;
    let subject1 = l_in.clone();
    if subject1 == 1 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(246), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(330), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(247), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(330), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(248), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(330), false);
    } else if subject1 == 2 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(254), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(330), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(255), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(330), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(256), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(330), false);
    }
    l_i = Val::from(0);
    'l2: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
            break 'l2;
        }
        'b2: {
            ctx.call(
                Function::Monster,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_n, &l_i.clone(), false),
                    runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                    Val::from("Keeper Of The Temple"),
                    Val::from(1933),
                    Val::from(1),
                    ((((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone()) + Val::from("::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn temple_keeper_main_all_ontouch(ctx: &Ctx) -> Script {
    temple_keeper_main_all_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn temple_keeper_main_all_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_in = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_in = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?).try_sub(Val::from(1))?),
    )?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac0")) + l_in.clone())],
    )?;
    return Err(Stop::End);
}

pub fn temple_keeper_main_all_ondisable(ctx: &Ctx) -> Script {
    temple_keeper_main_all_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn temple_keeper_main_all_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn temple_keeper_main_all_onmymobdead(ctx: &Ctx) -> Script {
    temple_keeper_main_all_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn temple_keeper_main_all_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_i = Val::from(1);
    'l1: loop {
        if !(l_i.clone().number()? <= 2) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::DisableNpc,
                vec![(((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac0")) + l_i.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn temple_keeper_main_all_oninit(ctx: &Ctx) -> Script {
    temple_keeper_main_all_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnMyMobDead")),
        ],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_ondisable(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_onfight_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_onfight(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_onfight_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden : You guys reached here.. Are your guardians dead...?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_ontimer1000(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden : I sincerely welcome all your best efforts!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_ontimer4000(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ICECRASH")?])?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss"))],
    )?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SPHERE")?])?;
    ctx.call(
        Function::Monster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from(252),
            Val::from(340),
            Val::from("Wish Maiden"),
            Val::from(1931),
            Val::from(1),
            ((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnMyMobDead")),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_ontimer5000(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if !(ctx
        .call(
            Function::MobCount,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                ((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnMyMobDead")),
            ],
        )?
        .is_true())
    {
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#okolnir_") + l_sub_s.clone()) + Val::from("::OnStop"))],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift::OnEnable"))],
        )?;
        ctx.call(
            Function::MapAnnounce,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                Val::from("Wish Maiden : ..Good, you deserve the Goddess' shine."),
                ctx.constant("BC_MAP")?,
                Val::from("0x00ff00"),
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_onmymobdead(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_boss_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss"))],
    )?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_boss_oninit(ctx: &Ctx) -> Script {
    wish_maiden_main_boss_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_check = Val::from(0);
    let mut l_gid = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_rwd = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_t_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_t_s = ((if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, &Val::from("aru")).is_true() {
        Val::from("arug_cas0")
    } else {
        Val::from("schg_cas0")
    }) + runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?).try_sub(Val::from(1))?),
    )?);
    l_gid = ctx.call(Function::GetCastleData, vec![l_t_s.clone(), Val::from(1)])?;
    if ctx
        .call(Function::GetCharacterId, vec![Val::from(2)])?
        .loosely_equals(&l_gid.clone())
    {
        if ctx
            .call(
                Function::GetGuildInfo,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(2)])?, Val::from(2)],
            )?
            .loosely_equals(&Val::from(1))
        {
            if runtime::compare(&l_sub_s.clone(), &Val::from("aru")).is_true() {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(7835), false);
                runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(7836), false);
                runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(7837), false);
                runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(7838), false);
                runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(2513), false);
                runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(7291), false);
                runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(10), false);
                runtime::local_set(&mut l_n, &Val::from(base + 12), Val::from(7293), false);
                runtime::local_set(&mut l_n, &Val::from(base + 13), Val::from(10), false);
                runtime::local_set(&mut l_n, &Val::from(base + 14), Val::from(7063), false);
                runtime::local_set(&mut l_n, &Val::from(base + 15), Val::from(100), false);
                runtime::local_set(&mut l_n, &Val::from(base + 16), Val::from(985), false);
                runtime::local_set(&mut l_n, &Val::from(base + 17), Val::from(20), false);
                l_rwd = Val::from(2541);
            } else {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(7830), false);
                runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(7831), false);
                runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(7832), false);
                runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(7833), false);
                runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(7834), false);
                runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(2357), false);
                runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 12), Val::from(7510), false);
                runtime::local_set(&mut l_n, &Val::from(base + 13), Val::from(100), false);
                runtime::local_set(&mut l_n, &Val::from(base + 14), Val::from(969), false);
                runtime::local_set(&mut l_n, &Val::from(base + 15), Val::from(10), false);
                runtime::local_set(&mut l_n, &Val::from(base + 16), Val::from(985), false);
                runtime::local_set(&mut l_n, &Val::from(base + 17), Val::from(20), false);
                l_rwd = Val::from(2383);
            }
            l_i = Val::from(0);
            'l1: loop {
                if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
                    break 'l1;
                }
                'b1: {
                    if runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_n, &l_i.clone(), false)])?,
                        ">=",
                        &runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                    )?
                    .is_true()
                    {
                        l_check = (l_check.clone() + Val::from(1));
                    }
                }
                l_i = (l_i.clone() + Val::from(2));
            }
            if l_check.clone().number()? >= 9 {
                ctx.call(Function::Cutin, vec![Val::from("wish_maiden12"), Val::from(1)])?;
                ctx.lines_as(
                    "Wish Maiden",
                    args![
                        "As I declared, I will give the Goddess' shine to you.",
                        "You have the requirements to carry it..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Wish Maiden", args!["You will be granted the power of the great Valkyrie..."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Wish Maiden",
                    args!["I will give you some things for fun.", "All of you enjoy them together..."],
                )?;
                ctx.next()?;
                ctx.lines_as("Wish Maiden", args!["Go to Rachel and Juno to meet the Ravies sisters."])?;
                ctx.next()?;
                ctx.call(Function::Cutin, vec![Val::from("wish_maiden32"), Val::from(1)])?;
                ctx.lines_as(
                    "Wish Maiden",
                    args![
                        "I will open the gate for you to come back here.",
                        "...Okolnir won't last forever..."
                    ],
                )?;
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::DelItem,
                            vec![
                                runtime::local_get(&l_n, &l_i.clone(), false),
                                runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                ctx.call(Function::GetItem, vec![l_rwd.clone(), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(7840), Val::from(1)])?;
                ctx.call(
                    Function::Announce,
                    vec![
                        ((((((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("], of guild ["))
                            + ctx.call(Function::GetGuildInfo, vec![l_gid.clone(), Val::from(0)])?)
                            + Val::from("] has brought a "))
                            + ctx.call(Function::GetItemName, vec![l_rwd.clone()])?)
                            + Val::from(" into this world.")),
                        ctx.constant("BC_ALL")?,
                        Val::from("0x70dbdb"),
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(255)])?;
                ctx.call(
                    Function::DisableNpc,
                    vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift"))],
                )?;
                ctx.call(
                    Function::EnableNpc,
                    vec![((Val::from("#to_agit_") + l_sub_s.clone()) + Val::from("_gate"))],
                )?;
                return Err(Stop::End);
            } else {
                ctx.call(Function::Cutin, vec![Val::from("wish_maiden13"), Val::from(1)])?;
                ctx.lines_as(
                    "Wish Maiden",
                    args![
                        ((Val::from("As I declared, you are worthy of holding the ")
                            + ctx.call(Function::GetItemName, vec![l_rwd.clone()])?)
                            + Val::from(".")),
                        "However, you do not have the requirements on you..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Wish Maiden", args!["...Did you forget something?"])?;
                ctx.close_window()?;
            }
        } else {
            ctx.call(Function::Cutin, vec![Val::from("wish_maiden12"), Val::from(1)])?;
            ctx.lines_as(
                "Wish Maiden",
                args![
                    "All of you worked together as a team...",
                    "Humans are strong when they are united, but are easily swayed by lust."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wish Maiden",
                args![
                    "Humans are imperfect, so their chief god is there for them when they need help.",
                    "...."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Wish Maiden", args!["Always be real.", "Do not regret your actions..."])?;
            ctx.close_window()?;
        }
    }
    ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift"))],
    )?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift_onenable(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift_ondisable(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_ontimer280000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden : ... Okolnir will soon disappear... I will send you back to where you originally came from."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift_ontimer280000(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_ontimer280000_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_ontimer290000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    let mut l_t_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapWarp,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            l_t_s.clone(),
            Val::from(157),
            Val::from(369),
        ],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#to_agit_") + l_sub_s.clone()) + Val::from("_gate::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift_ontimer290000(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_ontimer290000_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_ontimer300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#okolnir_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_pcc")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
        Val::from(2),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#okolnir_") + l_sub_s.clone()) + Val::from("_time01::OnEnable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift_ontimer300000(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_ontimer300000_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_main_gift_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift"))],
    )?;
    return Err(Stop::End);
}

pub fn wish_maiden_main_gift_oninit(ctx: &Ctx) -> Script {
    wish_maiden_main_gift_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn to_agit_main_gate_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn to_agit_main_gate(ctx: &Ctx) -> Script {
    to_agit_main_gate_body(ctx, Vec::new()).map(|_| ())
}

fn to_agit_main_gate_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    l_i = runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?).try_sub(Val::from(1))?),
    )?;
    if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, &Val::from("aru")).is_true() {
        let subject1 = l_i.clone();
        if subject1 == 1 {
            ctx.call(Function::Warp, vec![Val::from("arug_cas01"), Val::from(157), Val::from(369)])?;
        } else if subject1 == 2 {
            ctx.call(Function::Warp, vec![Val::from("arug_cas02"), Val::from(349), Val::from(355)])?;
        } else if subject1 == 3 {
            ctx.call(Function::Warp, vec![Val::from("arug_cas03"), Val::from(321), Val::from(153)])?;
        } else if subject1 == 4 {
            ctx.call(Function::Warp, vec![Val::from("arug_cas04"), Val::from(321), Val::from(153)])?;
        } else if subject1 == 5 {
            ctx.call(Function::Warp, vec![Val::from("arug_cas05"), Val::from(321), Val::from(153)])?;
        }
    } else {
        let subject2 = l_i.clone();
        if subject2 == 1 {
            ctx.call(Function::Warp, vec![Val::from("schg_cas01"), Val::from(369), Val::from(306)])?;
        } else if subject2 == 2 {
            ctx.call(Function::Warp, vec![Val::from("schg_cas02"), Val::from(177), Val::from(355)])?;
        } else if subject2 == 3 {
            ctx.call(Function::Warp, vec![Val::from("schg_cas03"), Val::from(81), Val::from(95)])?;
        } else if subject2 == 4 {
            ctx.call(Function::Warp, vec![Val::from("schg_cas04"), Val::from(369), Val::from(306)])?;
        } else if subject2 == 5 {
            ctx.call(Function::Warp, vec![Val::from("schg_cas05"), Val::from(369), Val::from(306)])?;
        }
    }
    return Err(Stop::End);
}

pub fn to_agit_main_gate_ontouch(ctx: &Ctx) -> Script {
    to_agit_main_gate_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn to_agit_main_gate_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#to_agit_") + l_sub_s.clone()) + Val::from("_gate"))],
    )?;
    return Err(Stop::End);
}

pub fn to_agit_main_gate_oninit(ctx: &Ctx) -> Script {
    to_agit_main_gate_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_time01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn okolnir_main_time01(ctx: &Ctx) -> Script {
    okolnir_main_time01_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_time01_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    runtime::setd(
        ctx,
        &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn okolnir_main_time01_onenable(ctx: &Ctx) -> Script {
    okolnir_main_time01_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_time01_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    runtime::setd(
        ctx,
        &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn okolnir_main_time01_onreset(ctx: &Ctx) -> Script {
    okolnir_main_time01_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_time01_ontimer3600000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if runtime::getd(
        ctx,
        &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )?
    .number()?
        < 12
    {
        runtime::setd(
            ctx,
            &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
            (runtime::getd(
                ctx,
                &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
                &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
            )? + Val::from(1)),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
        ctx.call(Function::InitNpcTimer, vec![])?;
    } else if runtime::getd(
        ctx,
        &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )? == 12
    {
        runtime::setd(
            ctx,
            &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
            Val::from(0),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
        runtime::setd(
            ctx,
            &((Val::from("$gqse_") + l_sub_s.clone()) + Val::from("_time")),
            Val::from(0),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
        ctx.call(Function::EnableNpc, vec![(Val::from("Wish Maiden#gq_") + l_sub_s.clone())])?;
        ctx.call(Function::EnableNpc, vec![(Val::from("Piamette#") + l_sub_s.clone())])?;
        ctx.call(
            Function::EnableNpc,
            vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss"))],
        )?;
        ctx.call(Function::StopNpcTimer, vec![])?;
    }
    return Err(Stop::End);
}

pub fn okolnir_main_time01_ontimer3600000(ctx: &Ctx) -> Script {
    okolnir_main_time01_ontimer3600000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_time01_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if runtime::getd(
        ctx,
        &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )? == 2
    {
        ctx.call(Function::InitNpcTimer, vec![])?;
    }
    return Err(Stop::End);
}

pub fn okolnir_main_time01_oninit(ctx: &Ctx) -> Script {
    okolnir_main_time01_oninit_body(ctx, Vec::new()).map(|_| ())
}
