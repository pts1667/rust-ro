use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum FindshipStep {
    Start,
    OnEnable,
    OnTimer300000,
    OnInit,
    OnTouch,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BaehideunMainStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer300000,
    OnDisable,
    OnMyMobDead,
}

pub(super) fn baehideun_main_run(ctx: &Ctx, mut step: BaehideunMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_m: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            BaehideunMainStep::Start => {
                step = BaehideunMainStep::OnInit;
                continue 'machine;
            }
            BaehideunMainStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            BaehideunMainStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                l_c = runtime::charat(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from(9))?;
                {
                    let subject1 = l_c.clone();
                    if subject1 == 1 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_m, &Val::from(base + 0), Val::from(89), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 1), Val::from(112), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 2), Val::from(1425), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 3), Val::from(85), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 4), Val::from(110), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 5), Val::from(1425), false);
                    } else if subject1 == 2 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_m, &Val::from(base + 0), Val::from(89), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 1), Val::from(112), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 2), Val::from(1425), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 3), Val::from(80), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 4), Val::from(110), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 5), Val::from(1426), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 6), Val::from(83), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 7), Val::from(114), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 8), Val::from(1426), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 9), Val::from(85), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 10), Val::from(110), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 11), Val::from(1425), false);
                    } else if subject1 == 3 || subject1 == 4 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_m, &Val::from(base + 0), Val::from(85), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 1), Val::from(111), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 2), Val::from(1451), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 3), Val::from(89), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 4), Val::from(112), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 5), Val::from(1543), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 6), Val::from(90), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 7), Val::from(106), false);
                        runtime::local_set(&mut l_m, &Val::from(base + 8), Val::from(1543), false);
                    }
                }
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_m.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("mosk_ship"),
                                runtime::local_get(&l_m, &l_i.clone(), false),
                                runtime::local_get(&l_m, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Sea Monster"),
                                runtime::local_get(&l_m, &(l_i.clone() + Val::from(2)), false),
                                Val::from(1),
                                (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(3));
                }
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            BaehideunMainStep::OnTimer300000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![
                        Val::from("mosk_ship"),
                        (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                step = BaehideunMainStep::OnDisable;
                continue 'machine;
            }
            BaehideunMainStep::OnDisable => {
                ctx.var("$@mos1_edq").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            BaehideunMainStep::OnMyMobDead => {
                if !(ctx
                    .call(
                        Function::MobCount,
                        vec![
                            Val::from("mosk_ship"),
                            (ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnMyMobDead")),
                        ],
                    )?
                    .is_true())
                {
                    ctx.var("$@mos1_edq").set(Val::from(0))?;
                    ctx.lines_as(
                        "Mr. Ibanoff",
                        args!["Now that all the monsters are gone,", "we can start sailing again", "normally."],
                    )?;
                    l_c = runtime::charat(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from(9))?;
                    let subject3 = l_c.clone();
                    if subject3 == 1 || subject3 == 2 {
                    } else if subject3 == 3 {
                        ctx.var("mos_whale_edq").set(Val::from(11))?;
                    } else if subject3 == 4 {
                        ctx.var("mos_whale_edq").set(Val::from(26))?;
                    }
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("::OnDisable"))],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum GrayWolfRus03Step {
    Start,
    OnEnable,
    OnTimer120000,
    OnDisable,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum WallRus04Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MosRusMainStep {
    Start,
    OnEnable,
    OnTimer120000,
    OnDisable,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum LusalkaRus23Step {
    Start,
    OnEnable,
    OnTimer300000,
    OnDisable,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum S1Rus27Step {
    Start,
    OnTouch,
    OnEnable,
    OnTimer30000,
    OnInit,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum OldTreasureBoxRus37Step {
    Start,
    OnCall,
    OnDisable,
    OnTimer180000,
    OnMyMobDead,
}
