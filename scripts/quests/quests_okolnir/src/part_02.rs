use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn maze_manager_main_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer2000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer3000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer3000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer3000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer120000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer120000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer120000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer121000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer121000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer121000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer123000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer123000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer123000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer240000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer240000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer240000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer241000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer241000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer241000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer242000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer242000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer242000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer360000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#Maze_Manager_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer360000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer360000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_oninit(ctx: &Ctx) -> Script {
    maze_manager_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MiroBfMainStep {
    Start,
    OnEnable,
    OnDisable,
    Cell,
    AfterCell,
    OnMyMobDead,
}

fn miro_bf_main_run(ctx: &Ctx, mut step: MiroBfMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cell: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            MiroBfMainStep::Start => {
                step = MiroBfMainStep::OnEnable;
                continue 'machine;
            }
            MiroBfMainStep::OnEnable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(44), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(46), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(50), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(52), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(50), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(52), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(56), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(279), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(279), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(64), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(301), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(64), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(298), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 20), Val::from(62), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 21), Val::from(272), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 22), Val::from(64), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 23), Val::from(272), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 24), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 25), Val::from(245), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 26), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 27), Val::from(243), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 28), Val::from(72), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 29), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 30), Val::from(72), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 31), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 32), Val::from(68), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 33), Val::from(257), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 34), Val::from(68), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 35), Val::from(255), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 36), Val::from(73), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 37), Val::from(263), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 38), Val::from(73), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 39), Val::from(261), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 40), Val::from(75), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 41), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 42), Val::from(75), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 43), Val::from(249), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 44), Val::from(79), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 45), Val::from(283), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 46), Val::from(79), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 47), Val::from(281), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 48), Val::from(82), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 49), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 50), Val::from(84), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 51), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 52), Val::from(89), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 53), Val::from(295), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 54), Val::from(89), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 55), Val::from(293), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 56), Val::from(88), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 57), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 58), Val::from(90), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 59), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 60), Val::from(88), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 61), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 62), Val::from(90), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 63), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 64), Val::from(94), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 65), Val::from(256), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 66), Val::from(96), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 67), Val::from(256), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 68), Val::from(64), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 69), Val::from(301), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 70), Val::from(64), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 71), Val::from(299), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 72), Val::from(100), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 73), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 74), Val::from(102), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 75), Val::from(251), false);
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
                                Val::from(" "),
                                Val::from(1934),
                                Val::from(1),
                                ((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                miro_bf_main_run(ctx, MiroBfMainStep::Cell, vec![l_sub_s.clone(), Val::from(0)])?;
                return Err(Stop::End);
            }
            MiroBfMainStep::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                    ],
                )?;
                miro_bf_main_run(ctx, MiroBfMainStep::Cell, vec![l_sub_s.clone(), Val::from(1)])?;
                return Err(Stop::End);
                step = MiroBfMainStep::AfterCell;
                continue 'machine;
            }
            MiroBfMainStep::Cell => {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_cell, &Val::from(base + 0), Val::from(44), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 1), Val::from(270), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 2), Val::from(47), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 3), Val::from(270), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 4), Val::from(50), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 5), Val::from(287), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 6), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 7), Val::from(287), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 8), Val::from(50), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 9), Val::from(265), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 10), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 11), Val::from(265), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 12), Val::from(56), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 13), Val::from(279), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 14), Val::from(59), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 15), Val::from(279), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 16), Val::from(64), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 17), Val::from(298), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 18), Val::from(64), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 19), Val::from(301), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 20), Val::from(62), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 21), Val::from(272), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 22), Val::from(65), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 23), Val::from(272), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 24), Val::from(58), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 25), Val::from(242), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 26), Val::from(58), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 27), Val::from(245), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 28), Val::from(72), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 29), Val::from(286), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 30), Val::from(72), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 31), Val::from(289), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 32), Val::from(68), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 33), Val::from(254), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 34), Val::from(68), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 35), Val::from(259), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 36), Val::from(73), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 37), Val::from(260), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 38), Val::from(73), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 39), Val::from(263), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 40), Val::from(75), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 41), Val::from(248), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 42), Val::from(75), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 43), Val::from(251), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 44), Val::from(79), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 45), Val::from(280), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 46), Val::from(79), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 47), Val::from(283), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 48), Val::from(82), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 49), Val::from(271), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 50), Val::from(85), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 51), Val::from(271), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 52), Val::from(89), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 53), Val::from(292), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 54), Val::from(89), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 55), Val::from(295), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 56), Val::from(88), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 57), Val::from(276), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 58), Val::from(91), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 59), Val::from(276), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 60), Val::from(88), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 61), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 62), Val::from(91), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 63), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 64), Val::from(94), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 65), Val::from(256), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 66), Val::from(97), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 67), Val::from(256), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 68), Val::from(64), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 69), Val::from(298), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 70), Val::from(64), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 71), Val::from(301), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 72), Val::from(100), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 73), Val::from(251), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 74), Val::from(103), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 75), Val::from(251), false);
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_cell.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::SetCell,
                            vec![
                                (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                                runtime::local_get(&l_cell, &l_i.clone(), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(1)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(2)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(3)), false),
                                ctx.constant("CELL_WALKABLE")?,
                                runtime::arg(&args, 1, Val::from(0)),
                            ],
                        )?;
                        ctx.call(
                            Function::SetCell,
                            vec![
                                (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                                runtime::local_get(&l_cell, &l_i.clone(), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(1)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(2)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(3)), false),
                                ctx.constant("CELL_SHOOTABLE")?,
                                runtime::arg(&args, 1, Val::from(0)),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(4));
                }
                return Ok(Val::from(0));
            }
            MiroBfMainStep::AfterCell => {
                step = MiroBfMainStep::OnMyMobDead;
                continue 'machine;
            }
            MiroBfMainStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn miro_bf_main(ctx: &Ctx) -> Script {
    miro_bf_main_run(ctx, MiroBfMainStep::Start, Vec::new()).map(|_| ())
}

pub fn miro_bf_main_onenable(ctx: &Ctx) -> Script {
    miro_bf_main_run(ctx, MiroBfMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn miro_bf_main_ondisable(ctx: &Ctx) -> Script {
    miro_bf_main_run(ctx, MiroBfMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn miro_bf_main_onmymobdead(ctx: &Ctx) -> Script {
    miro_bf_main_run(ctx, MiroBfMainStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MiroRfMainStep {
    Start,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    Cell,
    AfterCell,
}

fn miro_rf_main_run(ctx: &Ctx, mut step: MiroRfMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cell: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            MiroRfMainStep::Start => {
                step = MiroRfMainStep::OnEnable;
                continue 'machine;
            }
            MiroRfMainStep::OnEnable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(57), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(301), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(57), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(299), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(48), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(291), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(48), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(68), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(70), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(290), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(72), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(295), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(72), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(293), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(90), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(296), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(92), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(296), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 20), Val::from(56), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 21), Val::from(282), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 22), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 23), Val::from(282), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 24), Val::from(66), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 25), Val::from(283), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 26), Val::from(66), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 27), Val::from(281), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 28), Val::from(80), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 29), Val::from(284), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 30), Val::from(82), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 31), Val::from(284), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 32), Val::from(44), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 33), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 34), Val::from(46), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 35), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 36), Val::from(50), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 37), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 38), Val::from(52), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 39), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 40), Val::from(54), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 41), Val::from(269), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 42), Val::from(54), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 43), Val::from(267), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 44), Val::from(66), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 45), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 46), Val::from(66), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 47), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 48), Val::from(81), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 49), Val::from(273), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 50), Val::from(81), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 51), Val::from(272), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 52), Val::from(88), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 53), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 54), Val::from(90), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 55), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 56), Val::from(94), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 57), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 58), Val::from(96), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 59), Val::from(276), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 60), Val::from(64), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 61), Val::from(258), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 62), Val::from(66), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 63), Val::from(258), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 64), Val::from(76), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 65), Val::from(263), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 66), Val::from(76), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 67), Val::from(261), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 68), Val::from(87), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 69), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 70), Val::from(87), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 71), Val::from(263), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 72), Val::from(50), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 73), Val::from(252), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 74), Val::from(52), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 75), Val::from(252), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 76), Val::from(76), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 77), Val::from(252), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 78), Val::from(78), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 79), Val::from(252), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 80), Val::from(99), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 81), Val::from(255), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 82), Val::from(99), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 83), Val::from(253), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 84), Val::from(53), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 85), Val::from(245), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 86), Val::from(53), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 87), Val::from(243), false);
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
                                Val::from(" "),
                                Val::from(1935),
                                Val::from(1),
                                ((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                miro_rf_main_run(ctx, MiroRfMainStep::Cell, vec![l_sub_s.clone(), Val::from(0)])?;
                return Err(Stop::End);
            }
            MiroRfMainStep::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                    ],
                )?;
                miro_rf_main_run(ctx, MiroRfMainStep::Cell, vec![l_sub_s.clone(), Val::from(1)])?;
                return Err(Stop::End);
            }
            MiroRfMainStep::OnMyMobDead => {
                return Err(Stop::End);
                step = MiroRfMainStep::AfterCell;
                continue 'machine;
            }
            MiroRfMainStep::Cell => {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_cell, &Val::from(base + 0), Val::from(57), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 1), Val::from(298), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 2), Val::from(57), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 3), Val::from(301), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 4), Val::from(48), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 5), Val::from(288), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 6), Val::from(48), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 7), Val::from(291), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 8), Val::from(68), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 9), Val::from(290), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 10), Val::from(71), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 11), Val::from(290), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 12), Val::from(72), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 13), Val::from(292), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 14), Val::from(72), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 15), Val::from(295), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 16), Val::from(90), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 17), Val::from(296), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 18), Val::from(93), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 19), Val::from(296), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 20), Val::from(56), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 21), Val::from(282), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 22), Val::from(59), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 23), Val::from(282), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 24), Val::from(66), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 25), Val::from(280), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 26), Val::from(66), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 27), Val::from(283), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 28), Val::from(80), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 29), Val::from(284), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 30), Val::from(83), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 31), Val::from(284), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 32), Val::from(44), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 33), Val::from(273), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 34), Val::from(47), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 35), Val::from(273), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 36), Val::from(50), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 37), Val::from(273), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 38), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 39), Val::from(273), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 40), Val::from(54), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 41), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 42), Val::from(54), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 43), Val::from(269), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 44), Val::from(66), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 45), Val::from(270), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 46), Val::from(66), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 47), Val::from(271), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 48), Val::from(81), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 49), Val::from(272), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 50), Val::from(81), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 51), Val::from(273), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 52), Val::from(88), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 53), Val::from(276), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 54), Val::from(91), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 55), Val::from(276), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 56), Val::from(94), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 57), Val::from(276), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 58), Val::from(97), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 59), Val::from(276), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 60), Val::from(64), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 61), Val::from(258), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 62), Val::from(67), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 63), Val::from(258), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 64), Val::from(76), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 65), Val::from(260), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 66), Val::from(76), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 67), Val::from(263), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 68), Val::from(87), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 69), Val::from(262), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 70), Val::from(87), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 71), Val::from(265), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 72), Val::from(50), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 73), Val::from(252), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 74), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 75), Val::from(252), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 76), Val::from(76), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 77), Val::from(252), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 78), Val::from(79), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 79), Val::from(252), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 80), Val::from(99), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 81), Val::from(252), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 82), Val::from(99), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 83), Val::from(255), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 84), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 85), Val::from(242), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 86), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 87), Val::from(245), false);
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_cell.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::SetCell,
                            vec![
                                (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                                runtime::local_get(&l_cell, &l_i.clone(), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(1)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(2)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(3)), false),
                                ctx.constant("CELL_WALKABLE")?,
                                runtime::arg(&args, 1, Val::from(0)),
                            ],
                        )?;
                        ctx.call(
                            Function::SetCell,
                            vec![
                                (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                                runtime::local_get(&l_cell, &l_i.clone(), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(1)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(2)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(3)), false),
                                ctx.constant("CELL_SHOOTABLE")?,
                                runtime::arg(&args, 1, Val::from(0)),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(4));
                }
                return Ok(Val::from(0));
            }
            MiroRfMainStep::AfterCell => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn miro_rf_main(ctx: &Ctx) -> Script {
    miro_rf_main_run(ctx, MiroRfMainStep::Start, Vec::new()).map(|_| ())
}

pub fn miro_rf_main_onenable(ctx: &Ctx) -> Script {
    miro_rf_main_run(ctx, MiroRfMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn miro_rf_main_ondisable(ctx: &Ctx) -> Script {
    miro_rf_main_run(ctx, MiroRfMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn miro_rf_main_onmymobdead(ctx: &Ctx) -> Script {
    miro_rf_main_run(ctx, MiroRfMainStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MiroYfMainStep {
    Start,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    Cell,
    AfterCell,
}

fn miro_yf_main_run(ctx: &Ctx, mut step: MiroYfMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_cell: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    'machine: loop {
        match step {
            MiroYfMainStep::Start => {
                step = MiroYfMainStep::OnEnable;
                continue 'machine;
            }
            MiroYfMainStep::OnEnable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(44), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(46), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(292), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(67), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(295), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(67), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(293), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(94), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(301), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(94), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(299), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 12), Val::from(79), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 13), Val::from(289), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 14), Val::from(79), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 15), Val::from(287), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 16), Val::from(56), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 17), Val::from(282), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 18), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 19), Val::from(282), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 20), Val::from(71), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 21), Val::from(283), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 22), Val::from(71), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 23), Val::from(281), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 24), Val::from(100), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 25), Val::from(281), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 26), Val::from(102), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 27), Val::from(281), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 28), Val::from(44), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 29), Val::from(261), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 30), Val::from(46), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 31), Val::from(261), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 32), Val::from(50), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 33), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 34), Val::from(52), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 35), Val::from(265), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 36), Val::from(56), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 37), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 38), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 39), Val::from(270), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 40), Val::from(72), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 41), Val::from(278), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 42), Val::from(73), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 43), Val::from(278), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 44), Val::from(82), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 45), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 46), Val::from(84), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 47), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 48), Val::from(88), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 49), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 50), Val::from(90), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 51), Val::from(266), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 52), Val::from(94), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 53), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 54), Val::from(96), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 55), Val::from(271), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 56), Val::from(60), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 57), Val::from(257), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 58), Val::from(60), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 59), Val::from(255), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 60), Val::from(73), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 61), Val::from(263), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 62), Val::from(73), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 63), Val::from(261), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 64), Val::from(75), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 65), Val::from(257), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 66), Val::from(75), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 67), Val::from(255), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 68), Val::from(87), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 69), Val::from(257), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 70), Val::from(87), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 71), Val::from(255), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 72), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 73), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 74), Val::from(58), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 75), Val::from(249), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 76), Val::from(80), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 77), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 78), Val::from(80), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 79), Val::from(249), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 80), Val::from(53), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 81), Val::from(245), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 82), Val::from(53), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 83), Val::from(243), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 84), Val::from(75), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 85), Val::from(245), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 86), Val::from(75), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 87), Val::from(243), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 88), Val::from(100), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 89), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 90), Val::from(102), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 91), Val::from(251), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 92), Val::from(100), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 93), Val::from(256), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 94), Val::from(102), false);
                runtime::local_set(&mut l_xy, &Val::from(base + 95), Val::from(256), false);
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
                                Val::from(" "),
                                Val::from(1936),
                                Val::from(1),
                                ((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(2));
                }
                miro_yf_main_run(ctx, MiroYfMainStep::Cell, vec![l_sub_s.clone(), Val::from(0)])?;
                return Err(Stop::End);
            }
            MiroYfMainStep::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                    ],
                )?;
                miro_yf_main_run(ctx, MiroYfMainStep::Cell, vec![l_sub_s.clone(), Val::from(1)])?;
                return Err(Stop::End);
            }
            MiroYfMainStep::OnMyMobDead => {
                return Err(Stop::End);
                step = MiroYfMainStep::AfterCell;
                continue 'machine;
            }
            MiroYfMainStep::Cell => {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_cell, &Val::from(base + 0), Val::from(44), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 1), Val::from(292), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 2), Val::from(47), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 3), Val::from(292), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 4), Val::from(67), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 5), Val::from(292), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 6), Val::from(67), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 7), Val::from(295), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 8), Val::from(94), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 9), Val::from(298), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 10), Val::from(94), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 11), Val::from(301), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 12), Val::from(79), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 13), Val::from(286), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 14), Val::from(79), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 15), Val::from(289), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 16), Val::from(56), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 17), Val::from(282), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 18), Val::from(59), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 19), Val::from(282), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 20), Val::from(71), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 21), Val::from(280), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 22), Val::from(71), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 23), Val::from(283), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 24), Val::from(100), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 25), Val::from(281), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 26), Val::from(103), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 27), Val::from(281), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 28), Val::from(44), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 29), Val::from(261), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 30), Val::from(47), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 31), Val::from(261), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 32), Val::from(50), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 33), Val::from(265), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 34), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 35), Val::from(265), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 36), Val::from(56), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 37), Val::from(270), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 38), Val::from(59), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 39), Val::from(270), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 40), Val::from(72), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 41), Val::from(278), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 42), Val::from(73), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 43), Val::from(278), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 44), Val::from(82), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 45), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 46), Val::from(85), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 47), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 48), Val::from(88), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 49), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 50), Val::from(91), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 51), Val::from(266), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 52), Val::from(94), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 53), Val::from(271), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 54), Val::from(97), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 55), Val::from(271), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 56), Val::from(60), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 57), Val::from(254), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 58), Val::from(60), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 59), Val::from(257), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 60), Val::from(73), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 61), Val::from(260), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 62), Val::from(73), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 63), Val::from(263), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 64), Val::from(75), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 65), Val::from(254), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 66), Val::from(75), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 67), Val::from(257), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 68), Val::from(87), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 69), Val::from(254), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 70), Val::from(87), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 71), Val::from(257), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 72), Val::from(58), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 73), Val::from(248), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 74), Val::from(58), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 75), Val::from(251), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 76), Val::from(80), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 77), Val::from(248), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 78), Val::from(80), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 79), Val::from(251), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 80), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 81), Val::from(242), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 82), Val::from(53), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 83), Val::from(245), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 84), Val::from(75), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 85), Val::from(242), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 86), Val::from(75), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 87), Val::from(245), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 88), Val::from(100), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 89), Val::from(251), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 90), Val::from(103), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 91), Val::from(251), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 92), Val::from(100), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 93), Val::from(256), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 94), Val::from(103), false);
                runtime::local_set(&mut l_cell, &Val::from(base + 95), Val::from(256), false);
                l_i = Val::from(0);
                'l2: loop {
                    if !(runtime::op(&l_i.clone(), "<", &Val::from(l_cell.len() as i32))?.is_true()) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::SetCell,
                            vec![
                                (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                                runtime::local_get(&l_cell, &l_i.clone(), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(1)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(2)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(3)), false),
                                ctx.constant("CELL_WALKABLE")?,
                                runtime::arg(&args, 1, Val::from(0)),
                            ],
                        )?;
                        ctx.call(
                            Function::SetCell,
                            vec![
                                (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                                runtime::local_get(&l_cell, &l_i.clone(), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(1)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(2)), false),
                                runtime::local_get(&l_cell, &(l_i.clone() + Val::from(3)), false),
                                ctx.constant("CELL_SHOOTABLE")?,
                                runtime::arg(&args, 1, Val::from(0)),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(4));
                }
                return Ok(Val::from(0));
            }
            MiroYfMainStep::AfterCell => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn miro_yf_main(ctx: &Ctx) -> Script {
    miro_yf_main_run(ctx, MiroYfMainStep::Start, Vec::new()).map(|_| ())
}

pub fn miro_yf_main_onenable(ctx: &Ctx) -> Script {
    miro_yf_main_run(ctx, MiroYfMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn miro_yf_main_ondisable(ctx: &Ctx) -> Script {
    miro_yf_main_run(ctx, MiroYfMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn miro_yf_main_onmymobdead(ctx: &Ctx) -> Script {
    miro_yf_main_run(ctx, MiroYfMainStep::OnMyMobDead, Vec::new()).map(|_| ())
}

fn windpath01_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn windpath01_main(ctx: &Ctx) -> Script {
    windpath01_main_body(ctx, Vec::new()).map(|_| ())
}

fn windpath01_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if ctx.call(Function::CountItem, vec![Val::from(7839)])?.is_true() {
        ctx.mes("The Warp Gate responds to the Crystal Key.")?;
        ctx.call(Function::DelItem, vec![Val::from(7839), Val::from(1)])?;
        ctx.close_window()?;
        ctx.call(
            Function::Warp,
            vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(114), Val::from(158)],
        )?;
        return Err(Stop::End);
    }
    ctx.mes("You need the Crystal Key to activate the Warp Gate.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn windpath01_main_ontouch(ctx: &Ctx) -> Script {
    windpath01_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn piamette_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn piamette_main(ctx: &Ctx) -> Script {
    piamette_main_body(ctx, Vec::new()).map(|_| ())
}

fn piamette_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Piamette#") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn piamette_main_oninit(ctx: &Ctx) -> Script {
    piamette_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn event_start01_main(ctx: &Ctx) -> Script {
    event_start01_main_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("#event_start01_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn event_start01_main_onenable(ctx: &Ctx) -> Script {
    event_start01_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("#event_start01_") + l_sub_s.clone())])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn event_start01_main_ondisable(ctx: &Ctx) -> Script {
    event_start01_main_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    ctx.call(Function::InitNpcTimer, vec![])?;
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("#event_start01_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn event_start01_main_ontouch(ctx: &Ctx) -> Script {
    event_start01_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette mumblings : ..There were six birds, Teo. But one is...."),
            ctx.constant("BC_MAP")?,
            Val::from("0xdb7093"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn event_start01_main_ontimer2000(ctx: &Ctx) -> Script {
    event_start01_main_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_ontimer6000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::MapAnnounce, vec![(Val::from("que_q") + l_sub_s.clone()), Val::from("Piamette mumblings : ...Shh.. Teo. The birds have returned..Are they birds that ran away before...? Or breakers who disguises themselves as good adventurers?"), ctx.constant("BC_MAP")?, Val::from("0xdb7093")])?;
    return Err(Stop::End);
}

pub fn event_start01_main_ontimer6000(ctx: &Ctx) -> Script {
    event_start01_main_ontimer6000_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette mumblings : Ha! They are caged in."),
            ctx.constant("BC_MAP")?,
            Val::from("0xdb7093"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn event_start01_main_ontimer10000(ctx: &Ctx) -> Script {
    event_start01_main_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_ontimer14000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette : Teo, Catch the birds! Put the cage away from here!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer01_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn event_start01_main_ontimer14000(ctx: &Ctx) -> Script {
    event_start01_main_ontimer14000_body(ctx, Vec::new()).map(|_| ())
}

fn event_start01_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("#event_start01_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn event_start01_main_oninit(ctx: &Ctx) -> Script {
    event_start01_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gdtimer01_main(ctx: &Ctx) -> Script {
    gdtimer01_main_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_onenable(ctx: &Ctx) -> Script {
    gdtimer01_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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
                Function::DoNpcEvent,
                vec![((((Val::from("#getspell0") + l_i.clone()) + Val::from("_")) + l_sub_s.clone()) + Val::from("::OnDisable"))],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_onstop(ctx: &Ctx) -> Script {
    gdtimer01_main_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell08_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell01_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer1000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell01_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell02_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer10000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer20000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell02_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell03_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer20000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer20000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer30000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell03_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell04_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer30000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer30000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer40000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell04_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell05_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer40000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer40000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer50000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell05_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell06_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer50000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer50000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell06_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell07_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer60000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer70000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell07_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#getspell08_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer70000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer70000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer75000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if runtime::getd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        &[(".@i", runtime::Local::Scalar(&l_i)), (".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )?
    .number()?
        < 6
    {
        ctx.call(
            Function::MapAnnounce,
            vec![
                ((Val::from("que_q") + l_sub_s.clone()) + Val::from("")),
                Val::from("Piamette : Silly birds! Silly Teo! Why can't you put away the cage at once?!"),
                ctx.constant("BC_MAP")?,
                Val::from("0x00ff00"),
            ],
        )?;
        l_i = Val::from(1);
        'l1: loop {
            if !(l_i.clone().number()? <= 6) {
                break 'l1;
            }
            'b1: {
                ctx.call(
                    Function::EnableNpc,
                    vec![(((Val::from("#") + l_sub_s.clone()) + Val::from("_cage0")) + l_i.clone())],
                )?;
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        runtime::setd(
            ctx,
            &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
            Val::from(0),
            &mut [
                (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                (".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s)),
            ],
        )?;
    }
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer75000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer75000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer76000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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

pub fn gdtimer01_main_ontimer76000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer76000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer01_main_ontimer80000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer01_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer01_main_ontimer80000(ctx: &Ctx) -> Script {
    gdtimer01_main_ontimer80000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gdtimer02_main(ctx: &Ctx) -> Script {
    gdtimer02_main_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_onenable(ctx: &Ctx) -> Script {
    gdtimer02_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    ctx.call(Function::StopNpcTimer, vec![])?;
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_onstop(ctx: &Ctx) -> Script {
    gdtimer02_main_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette : All of them are caught!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xdb7093"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_ontimer4000(ctx: &Ctx) -> Script {
    gdtimer02_main_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_ontimer8000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette : Let's call out the keeper of the key not to let the birds go far away."),
            ctx.constant("BC_MAP")?,
            Val::from("0xdb7093"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_ontimer8000(ctx: &Ctx) -> Script {
    gdtimer02_main_ontimer8000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_ontimer12000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_xy: Vec<Val> = Vec::new();
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette : The keeper of the key is protecting my cage."),
            ctx.constant("BC_MAP")?,
            Val::from("0xdb7093"),
        ],
    )?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_xy, &Val::from(base + 0), Val::from(108), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 1), Val::from(151), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 2), Val::from(109), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 3), Val::from(135), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 4), Val::from(115), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 5), Val::from(116), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 6), Val::from(158), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 7), Val::from(106), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 8), Val::from(163), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 9), Val::from(133), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 10), Val::from(150), false);
    runtime::local_set(&mut l_xy, &Val::from(base + 11), Val::from(154), false);
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
                    Val::from("Key Stone"),
                    Val::from(1905),
                    Val::from(1),
                    ((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                ],
            )?;
        }
        l_i = (l_i.clone() + Val::from(2));
    }
    return Err(Stop::End);
}

pub fn gdtimer02_main_ontimer12000(ctx: &Ctx) -> Script {
    gdtimer02_main_ontimer12000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_ontimer112000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Piamette : Whooping fun times are back! Now, Go! I'll get back to!!"),
            ctx.constant("BC_MAP")?,
            Val::from("0xdb7093"),
        ],
    )?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    l_i = Val::from(1);
    'l1: loop {
        if !(l_i.clone().number()? <= 6) {
            break 'l1;
        }
        'b1: {
            ctx.call(
                Function::EnableNpc,
                vec![(((Val::from("#") + l_sub_s.clone()) + Val::from("_cage0")) + l_i.clone())],
            )?;
        }
        l_i = (l_i.clone() + Val::from(1));
    }
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        Val::from(0),
        &mut [
            (".@i", runtime::LocalMut::Scalar(&mut l_i)),
            (".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s)),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_ontimer112000(ctx: &Ctx) -> Script {
    gdtimer02_main_ontimer112000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_ontimer113000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer01_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_ontimer113000(ctx: &Ctx) -> Script {
    gdtimer02_main_ontimer113000_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gdtimer02_main_onreset(ctx: &Ctx) -> Script {
    gdtimer02_main_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn gdtimer02_main_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if !(ctx
        .call(
            Function::MobCount,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                ((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
            ],
        )?
        .is_true())
    {
        l_i = Val::from(1);
        'l1: loop {
            if !(l_i.clone().number()? <= 6) {
                break 'l1;
            }
            'b1: {
                ctx.call(
                    Function::EnableNpc,
                    vec![(((Val::from("#") + l_sub_s.clone()) + Val::from("_cage0")) + l_i.clone())],
                )?;
            }
            l_i = (l_i.clone() + Val::from(1));
        }
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#piamette_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
        )?;
        ctx.call(Function::StopNpcTimer, vec![])?;
    }
    return Err(Stop::End);
}

pub fn gdtimer02_main_onmymobdead(ctx: &Ctx) -> Script {
    gdtimer02_main_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum PiametteMainL1220Step {
    Start,
    OnEnable,
    OnReset,
    OnTimer1000,
    OnTimer5000,
    OnMyMobDead,
}

fn piamette_main_l1220_run(ctx: &Ctx, mut step: PiametteMainL1220Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    'machine: loop {
        match step {
            PiametteMainL1220Step::Start => {
                step = PiametteMainL1220Step::OnEnable;
                continue 'machine;
            }
            PiametteMainL1220Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            PiametteMainL1220Step::OnReset => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        ((Val::from("#piamette_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            PiametteMainL1220Step::OnTimer1000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from("Piamette : All the keeper of keys are dead now? Who freed my birds? Teo, who did it?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                return Err(Stop::End);
            }
            PiametteMainL1220Step::OnTimer5000 => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from("Piamette : I'm pissed off now!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xdb7093"),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Piamette#") + l_sub_s.clone())])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from(131),
                        Val::from(135),
                        Val::from("Angry Piamette"),
                        Val::from(1930),
                        Val::from(1),
                        ((Val::from("#piamette_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            PiametteMainL1220Step::OnMyMobDead => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from("Piamette has been released, so the warp gate toward the South is working now."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.call(Function::EnableNpc, vec![(Val::from("windpath03_") + l_sub_s.clone())])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("windpath04_") + l_sub_s.clone())])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![((Val::from("#nm_switch_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn piamette_main_l1220(ctx: &Ctx) -> Script {
    piamette_main_l1220_run(ctx, PiametteMainL1220Step::Start, Vec::new()).map(|_| ())
}

pub fn piamette_main_l1220_onenable(ctx: &Ctx) -> Script {
    piamette_main_l1220_run(ctx, PiametteMainL1220Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn piamette_main_l1220_onreset(ctx: &Ctx) -> Script {
    piamette_main_l1220_run(ctx, PiametteMainL1220Step::OnReset, Vec::new()).map(|_| ())
}

pub fn piamette_main_l1220_ontimer1000(ctx: &Ctx) -> Script {
    piamette_main_l1220_run(ctx, PiametteMainL1220Step::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn piamette_main_l1220_ontimer5000(ctx: &Ctx) -> Script {
    piamette_main_l1220_run(ctx, PiametteMainL1220Step::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn piamette_main_l1220_onmymobdead(ctx: &Ctx) -> Script {
    piamette_main_l1220_run(ctx, PiametteMainL1220Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn getspells_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn getspells_main(ctx: &Ctx) -> Script {
    getspells_main_body(ctx, Vec::new()).map(|_| ())
}

fn getspells_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_c = runtime::charat(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from(9))?;
    ctx.call(
        Function::EnableNpc,
        vec![(((Val::from("#getspell0") + l_c.clone()) + Val::from("_")) + l_sub_s.clone())],
    )?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LOCKON")?])?;
    return Err(Stop::End);
}

pub fn getspells_main_onenable(ctx: &Ctx) -> Script {
    getspells_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn getspells_main_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_c = runtime::charat(&ctx.call(Function::StrNpcInfo, vec![Val::from(2)])?, &Val::from(9))?;
    ctx.call(
        Function::DisableNpc,
        vec![(((Val::from("#getspell0") + l_c.clone()) + Val::from("_")) + l_sub_s.clone())],
    )?;
    return Err(Stop::End);
}

pub fn getspells_main_ondisable(ctx: &Ctx) -> Script {
    getspells_main_ondisable_body(ctx, Vec::new()).map(|_| ())
}
