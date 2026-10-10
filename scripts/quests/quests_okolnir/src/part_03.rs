use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn getspells_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_n_s: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::StartStatus,
        vec![ctx.constant("SC_STONE")?, Val::from(300000), Val::from(0), Val::from(10000)],
    )?;
    let subject1 = runtime::getd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        &[
            (".@n$", runtime::Local::Array(&l_n_s)),
            (".@sub$", runtime::Local::Scalar(&l_sub_s)),
        ],
    )?;
    if subject1 == 0 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("103"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("153"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("1"), true);
        runtime::local_set(
            &mut l_n_s,
            &Val::from(base + 3),
            Val::from("Piamette : One white bird has dropped with its wing pierced by an arrow."),
            true,
        );
    } else if subject1 == 1 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("102"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("135"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("2"), true);
        runtime::local_set(
            &mut l_n_s,
            &Val::from(base + 3),
            Val::from("Piamette : One bird is caught in a snare, and dropped into a lake."),
            true,
        );
    } else if subject1 == 2 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("113"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("111"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("3"), true);
        runtime::local_set(
            &mut l_n_s,
            &Val::from(base + 3),
            Val::from("Piamette : One bird has died trapped in it's cage."),
            true,
        );
    } else if subject1 == 3 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("161"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("105"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("4"), true);
        runtime::local_set(
            &mut l_n_s,
            &Val::from(base + 3),
            Val::from("Piamette : One bird was poisoned to death."),
            true,
        );
    } else if subject1 == 4 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("168"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("135"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("5"), true);
        runtime::local_set(
            &mut l_n_s,
            &Val::from(base + 3),
            Val::from("Piamette : One bird vomited blood while singing seven days and seven nights."),
            true,
        );
    } else if subject1 == 5 {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n_s, &Val::from(base + 0), Val::from("150"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 1), Val::from("159"), true);
        runtime::local_set(&mut l_n_s, &Val::from(base + 2), Val::from("6"), true);
        runtime::local_set(
            &mut l_n_s,
            &Val::from(base + 3),
            Val::from("Piamette : The last one broke her neck wriggling to get out of from it's eggshell!"),
            true,
        );
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#gdtimer01_") + l_sub_s.clone()) + Val::from("::OnStop"))],
        )?;
    }
    ctx.call(Function::DisableNpc, vec![(Val::from("#getspell01_") + l_sub_s.clone())])?;
    ctx.call(
        Function::Warp,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            runtime::atoi(&runtime::local_get(&l_n_s, &Val::from(0), true)),
            runtime::atoi(&runtime::local_get(&l_n_s, &Val::from(1), true)),
        ],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        runtime::atoi(&runtime::local_get(&l_n_s, &Val::from(2), true)),
        &mut [
            (".@n$", runtime::LocalMut::Array(&mut l_n_s)),
            (".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s)),
        ],
    )?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            runtime::local_get(&l_n_s, &Val::from(3), true),
            ctx.constant("BC_MAP")?,
            Val::from("0xFF0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn getspells_main_ontouch(ctx: &Ctx) -> Script {
    getspells_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn getspells_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_i = Val::from(1);
    'l1: loop {
        if !(l_i.clone().number()? <= 8) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::DisableNpc,
                vec![(((Val::from("#getspell0") + l_i.clone()) + Val::from("_")) + l_sub_s.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn getspells_main_oninit(ctx: &Ctx) -> Script {
    getspells_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn main_cages_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn main_cages(ctx: &Ctx) -> Script {
    main_cages_body(ctx, Vec::new()).map(|_| ())
}

fn main_cages_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::EndStatus, vec![ctx.constant("SC_STONE")?])?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(108), false);
    runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(151), false);
    runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(107), false);
    runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(135), false);
    runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(113), false);
    runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(114), false);
    runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(158), false);
    runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(106), false);
    runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(163), false);
    runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(133), false);
    runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(150), false);
    runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(154), false);
    l_i = Val::from(0);
    'l1: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::Warp,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_n, &l_i.clone(), false),
                    runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    l_i = Val::from(1);
    'l2: loop {
        if !(l_i.clone().number()? <= 6) {
            break 'l2;
        }
        'b2: {
            ctx.call(
                Function::DisableNpc,
                vec![(((Val::from("#") + l_sub_s.clone()) + Val::from("_cage0")) + l_i.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn main_cages_ontouch(ctx: &Ctx) -> Script {
    main_cages_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn main_cages_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_i = Val::from(1);
    'l1: loop {
        if !(l_i.clone().number()? <= 6) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::DisableNpc,
                vec![(((Val::from("#") + l_sub_s.clone()) + Val::from("_cage0")) + l_i.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    return Err(Stop::End);
}

pub fn main_cages_oninit(ctx: &Ctx) -> Script {
    main_cages_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn gd_main_mobctrl_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gd_main_mobctrl(ctx: &Ctx) -> Script {
    gd_main_mobctrl_body(ctx, Vec::new()).map(|_| ())
}

fn gd_main_mobctrl_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_mob_1: Vec<Val> = Vec::new();
    let mut l_mob_1_s: Vec<Val> = Vec::new();
    let mut l_mob_2: Vec<Val> = Vec::new();
    let mut l_mob_2_s: Vec<Val> = Vec::new();
    let mut l_mob_3: Vec<Val> = Vec::new();
    let mut l_mob_3_s: Vec<Val> = Vec::new();
    let mut l_mob_4: Vec<Val> = Vec::new();
    let mut l_mob_4_s: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_1_s, &Val::from(base + 0), Val::from("Ball of Piamette"), true);
    runtime::local_set(&mut l_mob_1_s, &Val::from(base + 1), Val::from("1738"), true);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_1, &Val::from(base + 0), Val::from(107), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 1), Val::from(152), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 2), Val::from(109), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 3), Val::from(135), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 4), Val::from(113), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 5), Val::from(116), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 6), Val::from(157), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 7), Val::from(107), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 8), Val::from(163), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 9), Val::from(133), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 10), Val::from(149), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 11), Val::from(156), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 12), Val::from(131), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 13), Val::from(139), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 14), Val::from(135), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 15), Val::from(136), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 16), Val::from(131), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 17), Val::from(132), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 18), Val::from(128), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 19), Val::from(136), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 20), Val::from(110), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 21), Val::from(145), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 22), Val::from(129), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 23), Val::from(114), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 24), Val::from(148), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 25), Val::from(114), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 26), Val::from(155), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 27), Val::from(128), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 28), Val::from(152), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 29), Val::from(145), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 30), Val::from(131), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 31), Val::from(151), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 32), Val::from(110), false);
    runtime::local_set(&mut l_mob_1, &Val::from(base + 33), Val::from(130), false);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_2_s, &Val::from(base + 0), Val::from("Alice"), true);
    runtime::local_set(&mut l_mob_2_s, &Val::from(base + 1), Val::from("1275"), true);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_2, &Val::from(base + 0), Val::from(130), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 1), Val::from(139), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 2), Val::from(135), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 3), Val::from(137), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 4), Val::from(130), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 5), Val::from(132), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 6), Val::from(128), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 7), Val::from(137), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 8), Val::from(109), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 9), Val::from(145), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 10), Val::from(109), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 11), Val::from(130), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 12), Val::from(128), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 13), Val::from(114), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 14), Val::from(147), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 15), Val::from(114), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 16), Val::from(154), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 17), Val::from(128), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 18), Val::from(151), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 19), Val::from(145), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 20), Val::from(130), false);
    runtime::local_set(&mut l_mob_2, &Val::from(base + 21), Val::from(151), false);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_3_s, &Val::from(base + 0), Val::from("Violy"), true);
    runtime::local_set(&mut l_mob_3_s, &Val::from(base + 1), Val::from("1390"), true);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_3, &Val::from(base + 0), Val::from(132), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 1), Val::from(139), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 2), Val::from(135), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 3), Val::from(135), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 4), Val::from(132), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 5), Val::from(132), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 6), Val::from(128), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 7), Val::from(135), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 8), Val::from(111), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 9), Val::from(145), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 10), Val::from(111), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 11), Val::from(130), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 12), Val::from(130), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 13), Val::from(114), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 14), Val::from(149), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 15), Val::from(114), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 16), Val::from(156), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 17), Val::from(128), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 18), Val::from(153), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 19), Val::from(145), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 20), Val::from(132), false);
    runtime::local_set(&mut l_mob_3, &Val::from(base + 21), Val::from(151), false);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_4_s, &Val::from(base + 0), Val::from("Ancient Mimic"), true);
    runtime::local_set(&mut l_mob_4_s, &Val::from(base + 1), Val::from("1699"), true);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mob_4, &Val::from(base + 0), Val::from(133), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 1), Val::from(139), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 2), Val::from(135), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 3), Val::from(134), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 4), Val::from(133), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 5), Val::from(132), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 6), Val::from(128), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 7), Val::from(135), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 8), Val::from(112), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 9), Val::from(145), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 10), Val::from(131), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 11), Val::from(114), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 12), Val::from(150), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 13), Val::from(114), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 14), Val::from(157), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 15), Val::from(128), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 16), Val::from(154), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 17), Val::from(145), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 18), Val::from(133), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 19), Val::from(151), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 20), Val::from(112), false);
    runtime::local_set(&mut l_mob_4, &Val::from(base + 21), Val::from(130), false);
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_c = Val::from(1);
    'l1: loop {
        if !(l_c.clone().number()? <= 4) {
            break 'l1;
        }
        'b1: {
            l_i = Val::from(0);
            'l2: loop {
                if !(runtime::op(
                    &l_i.clone(),
                    "<",
                    &runtime::getd_size(
                        ctx,
                        &(Val::from(".@mob_") + l_c.clone()),
                        &[
                            (".@c", runtime::Local::Scalar(&l_c)),
                            (".@i", runtime::Local::Scalar(&l_i)),
                            (".@mob_1", runtime::Local::Array(&l_mob_1)),
                            (".@mob_1$", runtime::Local::Array(&l_mob_1_s)),
                            (".@mob_2", runtime::Local::Array(&l_mob_2)),
                            (".@mob_2$", runtime::Local::Array(&l_mob_2_s)),
                            (".@mob_3", runtime::Local::Array(&l_mob_3)),
                            (".@mob_3$", runtime::Local::Array(&l_mob_3_s)),
                            (".@mob_4", runtime::Local::Array(&l_mob_4)),
                            (".@mob_4$", runtime::Local::Array(&l_mob_4_s)),
                            (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                        ],
                    )?,
                )?
                .is_true())
                {
                    break 'l2;
                }
                'b2: {
                    ctx.call(
                        Function::Monster,
                        vec![
                            (Val::from("que_q") + l_sub_s.clone()),
                            runtime::getd(
                                ctx,
                                &((((Val::from(".@mob_") + l_c.clone()) + Val::from("[")) + l_i.clone()) + Val::from("]")),
                                &[
                                    (".@c", runtime::Local::Scalar(&l_c)),
                                    (".@i", runtime::Local::Scalar(&l_i)),
                                    (".@mob_1", runtime::Local::Array(&l_mob_1)),
                                    (".@mob_1$", runtime::Local::Array(&l_mob_1_s)),
                                    (".@mob_2", runtime::Local::Array(&l_mob_2)),
                                    (".@mob_2$", runtime::Local::Array(&l_mob_2_s)),
                                    (".@mob_3", runtime::Local::Array(&l_mob_3)),
                                    (".@mob_3$", runtime::Local::Array(&l_mob_3_s)),
                                    (".@mob_4", runtime::Local::Array(&l_mob_4)),
                                    (".@mob_4$", runtime::Local::Array(&l_mob_4_s)),
                                    (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                                ],
                            )?,
                            runtime::getd(
                                ctx,
                                &((((Val::from(".@mob_") + l_c.clone()) + Val::from("[")) + (l_i.clone() + Val::from(1))) + Val::from("]")),
                                &[
                                    (".@c", runtime::Local::Scalar(&l_c)),
                                    (".@i", runtime::Local::Scalar(&l_i)),
                                    (".@mob_1", runtime::Local::Array(&l_mob_1)),
                                    (".@mob_1$", runtime::Local::Array(&l_mob_1_s)),
                                    (".@mob_2", runtime::Local::Array(&l_mob_2)),
                                    (".@mob_2$", runtime::Local::Array(&l_mob_2_s)),
                                    (".@mob_3", runtime::Local::Array(&l_mob_3)),
                                    (".@mob_3$", runtime::Local::Array(&l_mob_3_s)),
                                    (".@mob_4", runtime::Local::Array(&l_mob_4)),
                                    (".@mob_4$", runtime::Local::Array(&l_mob_4_s)),
                                    (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                                ],
                            )?,
                            runtime::getd(
                                ctx,
                                &((Val::from(".@mob_") + l_c.clone()) + Val::from("$")),
                                &[
                                    (".@c", runtime::Local::Scalar(&l_c)),
                                    (".@i", runtime::Local::Scalar(&l_i)),
                                    (".@mob_1", runtime::Local::Array(&l_mob_1)),
                                    (".@mob_1$", runtime::Local::Array(&l_mob_1_s)),
                                    (".@mob_2", runtime::Local::Array(&l_mob_2)),
                                    (".@mob_2$", runtime::Local::Array(&l_mob_2_s)),
                                    (".@mob_3", runtime::Local::Array(&l_mob_3)),
                                    (".@mob_3$", runtime::Local::Array(&l_mob_3_s)),
                                    (".@mob_4", runtime::Local::Array(&l_mob_4)),
                                    (".@mob_4$", runtime::Local::Array(&l_mob_4_s)),
                                    (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                                ],
                            )?,
                            runtime::atoi(&runtime::getd(
                                ctx,
                                &((Val::from(".@mob_") + l_c.clone()) + Val::from("$[1]")),
                                &[
                                    (".@c", runtime::Local::Scalar(&l_c)),
                                    (".@i", runtime::Local::Scalar(&l_i)),
                                    (".@mob_1", runtime::Local::Array(&l_mob_1)),
                                    (".@mob_1$", runtime::Local::Array(&l_mob_1_s)),
                                    (".@mob_2", runtime::Local::Array(&l_mob_2)),
                                    (".@mob_2$", runtime::Local::Array(&l_mob_2_s)),
                                    (".@mob_3", runtime::Local::Array(&l_mob_3)),
                                    (".@mob_3$", runtime::Local::Array(&l_mob_3_s)),
                                    (".@mob_4", runtime::Local::Array(&l_mob_4)),
                                    (".@mob_4$", runtime::Local::Array(&l_mob_4_s)),
                                    (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                                ],
                            )?),
                            Val::from(1),
                            ((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnMyMobDead")),
                        ],
                    )?;
                }
                l_i = (l_i.clone() + Val::from(2));
            }
            l_c = (l_c.clone() + Val::from(1));
        }
    }
    return Err(Stop::End);
}

pub fn gd_main_mobctrl_onenable(ctx: &Ctx) -> Script {
    gd_main_mobctrl_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn gd_main_mobctrl_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gd_main_mobctrl_onreset(ctx: &Ctx) -> Script {
    gd_main_mobctrl_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn gd_main_mobctrl_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if ctx
        .call(
            Function::MobCount,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                ((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnMyMobDead")),
            ],
        )?
        .number()?
        < 7
    {
        let base = Val::from(0).number()?;
        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(107), false);
        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(152), false);
        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(109), false);
        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(135), false);
        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(113), false);
        runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(116), false);
        runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(157), false);
        runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(107), false);
        runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(163), false);
        runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(133), false);
        runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(149), false);
        runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(156), false);
        l_i = Val::from(0);
        'l1: loop {
            if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
                break 'l1;
            }
            'b1: {
                ctx.call(
                    Function::Monster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        runtime::local_get(&l_n, &l_i.clone(), false),
                        runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                        Val::from("Ball of Piamette"),
                        Val::from(1738),
                        Val::from(1),
                        ((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnMyMobDead")),
                    ],
                )?;
            }
            l_i = (l_i.clone() + Val::from(2));
        }
    }
    return Err(Stop::End);
}

pub fn gd_main_mobctrl_onmymobdead(ctx: &Ctx) -> Script {
    gd_main_mobctrl_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn windpaths_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn windpaths_main(ctx: &Ctx) -> Script {
    windpaths_main_body(ctx, Vec::new()).map(|_| ())
}

fn windpaths_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("windpath03")).is_true() {
        ctx.call(
            Function::Warp,
            vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(119), Val::from(103)],
        )?;
    } else {
        ctx.call(
            Function::Warp,
            vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(146), Val::from(109)],
        )?;
    }
    return Err(Stop::End);
}

pub fn windpaths_main_ontouch(ctx: &Ctx) -> Script {
    windpaths_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn windpaths_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("windpath03_") + l_sub_s.clone())])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("windpath04_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn windpaths_main_oninit(ctx: &Ctx) -> Script {
    windpaths_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn nm_switch_main(ctx: &Ctx) -> Script {
    nm_switch_main_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("#nm_switch_") + l_sub_s.clone())])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontouch(ctx: &Ctx) -> Script {
    nm_switch_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("#nm_switch_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn nm_switch_main_onenable(ctx: &Ctx) -> Script {
    nm_switch_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("#nm_switch_") + l_sub_s.clone())])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ondisable(ctx: &Ctx) -> Script {
    nm_switch_main_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: Did you have a good time with Piamette?"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontimer1000(ctx: &Ctx) -> Script {
    nm_switch_main_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: But it's the end. There will be summoned monsters coming soon..."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontimer4000(ctx: &Ctx) -> Script {
    nm_switch_main_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontimer9000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: So come here to me safely..."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontimer9000(ctx: &Ctx) -> Script {
    nm_switch_main_ontimer9000_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontimer10000(ctx: &Ctx) -> Script {
    nm_switch_main_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontimer190000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone02::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontimer190000(ctx: &Ctx) -> Script {
    nm_switch_main_ontimer190000_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_ontimer370000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone03::OnEnable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn nm_switch_main_ontimer370000(ctx: &Ctx) -> Script {
    nm_switch_main_ontimer370000_body(ctx, Vec::new()).map(|_| ())
}

fn nm_switch_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("#nm_switch_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn nm_switch_main_oninit(ctx: &Ctx) -> Script {
    nm_switch_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NmsommainJin01Step {
    Start,
    OnEnable,
    OnDisable,
    OnTimer5000,
    OnTimer25000,
    OnTimer55000,
    OnTimer85000,
    OnTimer120000,
    OnTimer180000,
    OnMyMobDead,
}

fn nmsommain_jin01_run(ctx: &Ctx, mut step: NmsommainJin01Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    let mut l_xy2: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            NmsommainJin01Step::Start => {
                step = NmsommainJin01Step::OnEnable;
                continue 'machine;
            }
            NmsommainJin01Step::OnEnable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnEnable"))],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin01Step::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnMyMobDead")),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin01Step::OnTimer5000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(226), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(288), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(227), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(228), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(229), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(291), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(230), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(231), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(293), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(232), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(294), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(233), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(295), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(234), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(296), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(235), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(297), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 20), Val::from(228), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 21), Val::from(286), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 22), Val::from(229), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 23), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 24), Val::from(230), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 25), Val::from(288), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 26), Val::from(231), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 27), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 28), Val::from(232), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 29), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 30), Val::from(233), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 31), Val::from(291), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 32), Val::from(234), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 33), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 34), Val::from(235), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 35), Val::from(293), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 36), Val::from(236), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 37), Val::from(294), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 38), Val::from(237), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 39), Val::from(295), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 40), Val::from(230), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 41), Val::from(284), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 42), Val::from(231), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 43), Val::from(285), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 44), Val::from(232), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 45), Val::from(286), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 46), Val::from(233), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 47), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 48), Val::from(234), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 49), Val::from(288), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 50), Val::from(235), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 51), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 52), Val::from(236), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 53), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 54), Val::from(237), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 55), Val::from(291), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 56), Val::from(238), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 57), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 58), Val::from(239), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 59), Val::from(293), false);
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from("The Western magic formation is working to summon Guard of Shadow."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x4d4dff"),
                    ],
                )?;
                l_i = Val::from(0);
                'l1: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy.len() as i32))?.is_true()) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy, &l_i.clone(), false),
                                runtime::local_get(&l_xy, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Guard of Shadow"),
                                Val::from(1752),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin01Step::OnTimer25000 => {
                step = NmsommainJin01Step::OnTimer55000;
                continue 'machine;
            }
            NmsommainJin01Step::OnTimer55000 => {
                step = NmsommainJin01Step::OnTimer85000;
                continue 'machine;
            }
            NmsommainJin01Step::OnTimer85000 => {
                step = NmsommainJin01Step::OnTimer120000;
                continue 'machine;
            }
            NmsommainJin01Step::OnTimer120000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy2, &Val::from(base + 0), Val::from(226), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 1), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 2), Val::from(227), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 3), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 4), Val::from(228), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 5), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 6), Val::from(229), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 7), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 8), Val::from(230), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 9), Val::from(295), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 10), Val::from(231), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 11), Val::from(296), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 12), Val::from(231), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 13), Val::from(297), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 14), Val::from(231), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 15), Val::from(298), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 16), Val::from(231), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 17), Val::from(299), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 18), Val::from(230), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 19), Val::from(300), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 20), Val::from(229), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 21), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 22), Val::from(228), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 23), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 24), Val::from(227), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 25), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 26), Val::from(226), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 27), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 28), Val::from(225), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 29), Val::from(300), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 30), Val::from(224), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 31), Val::from(299), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 32), Val::from(224), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 33), Val::from(298), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 34), Val::from(224), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 35), Val::from(297), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 36), Val::from(224), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 37), Val::from(296), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 38), Val::from(225), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 39), Val::from(295), false);
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy2.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy2, &l_i.clone(), false),
                                runtime::local_get(&l_xy2, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Guard of Shadow"),
                                Val::from(1752),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin01Step::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin01Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn nmsommain_jin01(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::Start, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_onenable(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ondisable(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ontimer5000(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ontimer25000(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ontimer55000(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnTimer55000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ontimer85000(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnTimer85000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ontimer120000(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_ontimer180000(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin01_onmymobdead(ctx: &Ctx) -> Script {
    nmsommain_jin01_run(ctx, NmsommainJin01Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn main_stone01_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn main_stone01(ctx: &Ctx) -> Script {
    main_stone01_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone01_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(227), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(294), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(229), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(294), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(231), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(296), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(231), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(298), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(230), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(300), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(228), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(301), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(226), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(301), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(224), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(299), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(224), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(297), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(225), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(295), false);
    l_i = Val::from(0);
    'l1: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy.len() as i32))?.is_true()) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::Monster,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_xy, &l_i.clone(), false),
                    runtime::local_get(&l_xy, &(l_i.clone() + Val::from(1)), false),
                    Val::from("Western Magic Guardian"),
                    Val::from(1752),
                    Val::from(1),
                    ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn main_stone01_onenable(ctx: &Ctx) -> Script {
    main_stone01_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone01_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn main_stone01_onreset(ctx: &Ctx) -> Script {
    main_stone01_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone01_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if ctx.call(
        Function::MobCount,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnMyMobDead")),
        ],
    )? == 0
    {
        runtime::setd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
            (runtime::getd(
                ctx,
                &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
                &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
            )? + Val::from(1)),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnDisable"))],
        )?;
        if runtime::getd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
            &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
        )? == 3
        {
            ctx.call(
                Function::DoNpcEvent,
                vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnFight"))],
            )?;
        }
    }
    return Err(Stop::End);
}

pub fn main_stone01_onmymobdead(ctx: &Ctx) -> Script {
    main_stone01_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NmsommainJin02Step {
    Start,
    OnEnable,
    OnDisable,
    OnTimer5000,
    OnTimer25000,
    OnTimer55000,
    OnTimer85000,
    OnTimer120000,
    OnTimer180000,
    OnMyMobDead,
}

fn nmsommain_jin02_run(ctx: &Ctx, mut step: NmsommainJin02Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    let mut l_xy2: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            NmsommainJin02Step::Start => {
                step = NmsommainJin02Step::OnEnable;
                continue 'machine;
            }
            NmsommainJin02Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin02Step::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnMyMobDead")),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin02Step::OnTimer5000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from("The Eastern magic formation is working to summon Bloody Hunter."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x4d4dff"),
                    ],
                )?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(263), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(264), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(291), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(267), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(288), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(268), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(269), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(286), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(285), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(284), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(272), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(283), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 20), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 21), Val::from(294), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 22), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 23), Val::from(293), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 24), Val::from(267), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 25), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 26), Val::from(268), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 27), Val::from(291), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 28), Val::from(269), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 29), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 30), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 31), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 32), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 33), Val::from(288), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 34), Val::from(272), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 35), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 36), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 37), Val::from(286), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 38), Val::from(274), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 39), Val::from(285), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 40), Val::from(267), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 41), Val::from(296), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 42), Val::from(268), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 43), Val::from(295), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 44), Val::from(269), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 45), Val::from(294), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 46), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 47), Val::from(283), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 48), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 49), Val::from(282), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 50), Val::from(272), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 51), Val::from(281), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 52), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 53), Val::from(280), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 54), Val::from(274), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 55), Val::from(279), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 56), Val::from(275), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 57), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 58), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 59), Val::from(275), false);
                l_i = Val::from(0);
                'l1: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy.len() as i32))?.is_true()) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy, &l_i.clone(), false),
                                runtime::local_get(&l_xy, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Bloody Hunter"),
                                Val::from(1753),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin02Step::OnTimer25000 => {
                step = NmsommainJin02Step::OnTimer55000;
                continue 'machine;
            }
            NmsommainJin02Step::OnTimer55000 => {
                step = NmsommainJin02Step::OnTimer85000;
                continue 'machine;
            }
            NmsommainJin02Step::OnTimer85000 => {
                step = NmsommainJin02Step::OnTimer120000;
                continue 'machine;
            }
            NmsommainJin02Step::OnTimer120000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy2, &Val::from(base + 0), Val::from(274), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 1), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 2), Val::from(275), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 3), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 4), Val::from(276), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 5), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 6), Val::from(277), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 7), Val::from(301), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 8), Val::from(278), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 9), Val::from(300), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 10), Val::from(279), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 11), Val::from(299), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 12), Val::from(279), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 13), Val::from(298), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 14), Val::from(279), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 15), Val::from(297), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 16), Val::from(279), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 17), Val::from(296), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 18), Val::from(278), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 19), Val::from(295), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 20), Val::from(277), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 21), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 22), Val::from(276), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 23), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 24), Val::from(275), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 25), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 26), Val::from(274), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 27), Val::from(294), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 28), Val::from(273), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 29), Val::from(295), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 30), Val::from(272), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 31), Val::from(296), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 32), Val::from(272), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 33), Val::from(297), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 34), Val::from(272), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 35), Val::from(298), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 36), Val::from(272), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 37), Val::from(299), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 38), Val::from(273), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 39), Val::from(300), false);
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy2.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy2, &l_i.clone(), false),
                                runtime::local_get(&l_xy2, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Bloody Hunter"),
                                Val::from(1753),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin02Step::OnTimer180000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin02Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn nmsommain_jin02(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::Start, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_onenable(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ondisable(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ontimer5000(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ontimer25000(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ontimer55000(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnTimer55000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ontimer85000(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnTimer85000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ontimer120000(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_ontimer180000(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin02_onmymobdead(ctx: &Ctx) -> Script {
    nmsommain_jin02_run(ctx, NmsommainJin02Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn main_stone02_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn main_stone02(ctx: &Ctx) -> Script {
    main_stone02_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone02_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(275), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(301), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(277), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(301), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(279), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(299), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(279), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(297), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(278), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(295), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(276), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(294), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(274), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(294), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(272), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(296), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(272), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(298), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(273), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(300), false);
    l_i = Val::from(0);
    'l1: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy.len() as i32))?.is_true()) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::Monster,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_xy, &l_i.clone(), false),
                    runtime::local_get(&l_xy, &(l_i.clone() + Val::from(1)), false),
                    Val::from("Eastern Magic Guardian"),
                    Val::from(1753),
                    Val::from(1),
                    ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone02::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn main_stone02_onenable(ctx: &Ctx) -> Script {
    main_stone02_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone02_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone02::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn main_stone02_onreset(ctx: &Ctx) -> Script {
    main_stone02_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone02_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if ctx.call(
        Function::MobCount,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone02::OnMyMobDead")),
        ],
    )? == 0
    {
        runtime::setd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
            (runtime::getd(
                ctx,
                &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
                &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
            )? + Val::from(1)),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnDisable"))],
        )?;
        if runtime::getd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
            &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
        )? == 3
        {
            ctx.call(
                Function::DoNpcEvent,
                vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnFight"))],
            )?;
        }
    }
    return Err(Stop::End);
}

pub fn main_stone02_onmymobdead(ctx: &Ctx) -> Script {
    main_stone02_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum NmsommainJin03Step {
    Start,
    OnEnable,
    OnDisable,
    OnTimer5000,
    OnTimer120000,
    OnTimer240000,
    OnTimer360000,
    OnMyMobDead,
}

fn nmsommain_jin03_run(ctx: &Ctx, mut step: NmsommainJin03Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    let mut l_xy2: Vec<Val> = Vec::new();
    let mut l_xy3: Vec<Val> = Vec::new();
    let mut l_xy4: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            NmsommainJin03Step::Start => {
                step = NmsommainJin03Step::OnEnable;
                continue 'machine;
            }
            NmsommainJin03Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin03Step::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnMyMobDead")),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin03Step::OnTimer5000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from("The Northern magic formation is working to summon Keeper of the Temple."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x4d4dff"),
                    ],
                )?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(247), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(329), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(249), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(329), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(329), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(253), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(329), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(255), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(329), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(243), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(339), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(245), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(337), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(247), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(335), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(247), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(333), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(254), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(333), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 20), Val::from(256), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 21), Val::from(335), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 22), Val::from(258), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 23), Val::from(337), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 24), Val::from(260), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 25), Val::from(339), false);
                l_i = Val::from(0);
                'l1: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy.len() as i32))?.is_true()) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy, &l_i.clone(), false),
                                runtime::local_get(&l_xy, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Keeper Of The Temple"),
                                Val::from(1933),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin03Step::OnTimer120000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy2, &Val::from(base + 0), Val::from(251), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 1), Val::from(343), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 2), Val::from(252), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 3), Val::from(343), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 4), Val::from(255), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 5), Val::from(341), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 6), Val::from(255), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 7), Val::from(340), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 8), Val::from(254), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 9), Val::from(337), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 10), Val::from(253), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 11), Val::from(336), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 12), Val::from(250), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 13), Val::from(336), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 14), Val::from(249), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 15), Val::from(337), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 16), Val::from(248), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 17), Val::from(340), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 18), Val::from(248), false);
                runtime::local_set(&mut l_xy2, &Val::from(base + 19), Val::from(341), false);
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy2.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy2, &l_i.clone(), false),
                                runtime::local_get(&l_xy2, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Keeper Of The Temple"),
                                Val::from(1933),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin03Step::OnTimer240000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy3, &Val::from(base + 0), Val::from(250), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 1), Val::from(343), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 2), Val::from(252), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 3), Val::from(343), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 4), Val::from(254), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 5), Val::from(342), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 6), Val::from(255), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 7), Val::from(340), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 8), Val::from(255), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 9), Val::from(338), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 10), Val::from(253), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 11), Val::from(336), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 12), Val::from(250), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 13), Val::from(336), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 14), Val::from(248), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 15), Val::from(338), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 16), Val::from(248), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 17), Val::from(340), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 18), Val::from(249), false);
                runtime::local_set(&mut l_xy3, &Val::from(base + 19), Val::from(342), false);
                l_i = Val::from(0);
                'l3: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy3.len() as i32))?.is_true()) {
                        break 'l3;
                    }
                    'b3: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy3, &l_i.clone(), false),
                                runtime::local_get(&l_xy3, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Keeper Of The Temple"),
                                Val::from(1933),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                return Err(Stop::End);
            }
            NmsommainJin03Step::OnTimer360000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy4, &Val::from(base + 0), Val::from(250), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 1), Val::from(343), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 2), Val::from(252), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 3), Val::from(343), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 4), Val::from(254), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 5), Val::from(342), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 6), Val::from(255), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 7), Val::from(340), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 8), Val::from(255), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 9), Val::from(338), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 10), Val::from(253), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 11), Val::from(336), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 12), Val::from(251), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 13), Val::from(336), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 14), Val::from(249), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 15), Val::from(337), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 16), Val::from(248), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 17), Val::from(339), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 18), Val::from(248), false);
                runtime::local_set(&mut l_xy4, &Val::from(base + 19), Val::from(341), false);
                l_i = Val::from(0);
                'l4: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy4.len() as i32))?.is_true()) {
                        break 'l4;
                    }
                    'b4: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                (Val::from("que_q") + l_sub_s.clone()),
                                runtime::local_get(&l_xy4, &l_i.clone(), false),
                                runtime::local_get(&l_xy4, &(l_i.clone() + Val::from(1)), false),
                                Val::from("Keeper Of The Temple"),
                                Val::from(1933),
                                Val::from(1),
                                ((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            NmsommainJin03Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn nmsommain_jin03(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::Start, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_onenable(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_ondisable(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_ontimer5000(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_ontimer120000(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_ontimer240000(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_ontimer360000(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn nmsommain_jin03_onmymobdead(ctx: &Ctx) -> Script {
    nmsommain_jin03_run(ctx, NmsommainJin03Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn main_stone03_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn main_stone03(ctx: &Ctx) -> Script {
    main_stone03_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone03_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(251), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(343), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(252), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(343), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(255), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(341), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(255), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(340), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(254), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(337), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(253), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(336), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(250), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(336), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(249), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(337), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(248), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(340), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(248), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(341), false);
    l_i = Val::from(0);
    'l1: loop {
        if !(runtime::op(&l_i.clone(), "<", &Val::from(l_xy.len() as i32))?.is_true()) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::Monster,
                vec![
                    (Val::from("que_q") + l_sub_s.clone()),
                    runtime::local_get(&l_xy, &l_i.clone(), false),
                    runtime::local_get(&l_xy, &(l_i.clone() + Val::from(1)), false),
                    Val::from("Northern Magic Guardian"),
                    Val::from(1933),
                    Val::from(1),
                    ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone03::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn main_stone03_onenable(ctx: &Ctx) -> Script {
    main_stone03_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone03_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone03::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn main_stone03_onreset(ctx: &Ctx) -> Script {
    main_stone03_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn main_stone03_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if ctx.call(
        Function::MobCount,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#") + l_sub_s.clone()) + Val::from("_stone03::OnMyMobDead")),
        ],
    )? == 0
    {
        runtime::setd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
            (runtime::getd(
                ctx,
                &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
                &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
            )? + Val::from(1)),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnDisable"))],
        )?;
        if runtime::getd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
            &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
        )? == 3
        {
            ctx.call(
                Function::DoNpcEvent,
                vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnFight"))],
            )?;
        }
    }
    return Err(Stop::End);
}

pub fn main_stone03_onmymobdead(ctx: &Ctx) -> Script {
    main_stone03_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn guard_of_shadow_main_all_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn guard_of_shadow_main_all(ctx: &Ctx) -> Script {
    guard_of_shadow_main_all_body(ctx, Vec::new()).map(|_| ())
}
