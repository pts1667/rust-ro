use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force05ex60Step {
    Start,
    OnReset,
    OnSummonMob05,
    OnMyMobDead,
}

fn force_05ex_60_run(ctx: &Ctx, mut step: Force05ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05ex60Step::Start => {
                step = Force05ex60Step::OnReset;
                continue 'machine;
            }
            Force05ex60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_05ex#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05ex60Step::OnSummonMob05 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(168),
                        Val::from(177),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(170),
                        Val::from(179),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(177),
                        Val::from(179),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(179),
                        Val::from(178),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(179),
                        Val::from(170),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(177),
                        Val::from(168),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(170),
                        Val::from(168),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(173),
                        Val::from(174),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(174),
                        Val::from(174),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(173),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(174),
                        Val::from(173),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(173),
                        Val::from(181),
                        Val::from("Kapha"),
                        Val::from(1543),
                        Val::from(1),
                        Val::from("force_05ex#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05ex60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05ex_60(ctx: &Ctx) -> Script {
    force_05ex_60_run(ctx, Force05ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05ex_60_onreset(ctx: &Ctx) -> Script {
    force_05ex_60_run(ctx, Force05ex60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05ex_60_onsummonmob_05(ctx: &Ctx) -> Script {
    force_05ex_60_run(ctx, Force05ex60Step::OnSummonMob05, Vec::new()).map(|_| ())
}

pub fn force_05ex_60_onmymobdead(ctx: &Ctx) -> Script {
    force_05ex_60_run(ctx, Force05ex60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05mob60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_05mob_60_run(ctx: &Ctx, mut step: Force05mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mob60Step::Start => {
                step = Force05mob60Step::OnReset;
                continue 'machine;
            }
            Force05mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_05mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mob60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#60::OnSummonMob_05")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(163),
                        Val::from(173),
                        Val::from("Drainliar"),
                        Val::from(1434),
                        Val::from(1),
                        Val::from("force_05mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(173),
                        Val::from(173),
                        Val::from("Myst"),
                        Val::from(1553),
                        Val::from(1),
                        Val::from("force_05mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(181),
                        Val::from(173),
                        Val::from("Orc Skeleton"),
                        Val::from(1462),
                        Val::from(1),
                        Val::from("force_05mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(171),
                        Val::from(177),
                        Val::from("Orc Skeleton"),
                        Val::from(1462),
                        Val::from(1),
                        Val::from("force_05mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(173),
                        Val::from(181),
                        Val::from("Raggler"),
                        Val::from(1445),
                        Val::from(1),
                        Val::from("force_05mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_05mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On05_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_05")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_60(ctx: &Ctx) -> Script {
    force_05mob_60_run(ctx, Force05mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_60_onreset(ctx: &Ctx) -> Script {
    force_05mob_60_run(ctx, Force05mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_60_onenable(ctx: &Ctx) -> Script {
    force_05mob_60_run(ctx, Force05mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_60_run(ctx, Force05mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06start60Step {
    Start,
    OnEnable,
}

fn force_06start_60_run(ctx: &Ctx, mut step: Force06start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06start60Step::Start => {
                step = Force06start60Step::OnEnable;
                continue 'machine;
            }
            Force06start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_60(ctx: &Ctx) -> Script {
    force_06start_60_run(ctx, Force06start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_60_onenable(ctx: &Ctx) -> Script {
    force_06start_60_run(ctx, Force06start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06ex60Step {
    Start,
    OnReset,
    OnEnable,
    OnSubOn,
    OnMyMobDead,
}

fn force_06ex_60_run(ctx: &Ctx, mut step: Force06ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06ex60Step::Start => {
                step = Force06ex60Step::OnReset;
                continue 'machine;
            }
            Force06ex60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_06ex#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06ex60Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(130),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(130),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(125),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(125),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(120),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(120),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(115),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(115),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(110),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(110),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(105),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(105),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(95),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(95),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(90),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(90),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(85),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(85),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(80),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(80),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(169),
                        Val::from(75),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(178),
                        Val::from(75),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_06ex#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06ex60Step::OnSubOn => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_2-1"),
                            ctx.call(Function::Rand, vec![Val::from(170), Val::from(177)])?,
                            ctx.call(Function::Rand, vec![Val::from(70), Val::from(120)])?,
                            Val::from("Sidewinder"),
                            Val::from(1424),
                            Val::from(1),
                            Val::from("force_06ex#60::OnMyMobDead"),
                        ],
                    )?;
                } else if subject1 == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_2-1"),
                            ctx.call(Function::Rand, vec![Val::from(170), Val::from(177)])?,
                            ctx.call(Function::Rand, vec![Val::from(70), Val::from(120)])?,
                            Val::from("Hermit Plant"),
                            Val::from(1565),
                            Val::from(1),
                            Val::from("force_06ex#60::OnMyMobDead"),
                        ],
                    )?;
                } else if subject1 == 3 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_2-1"),
                            ctx.call(Function::Rand, vec![Val::from(170), Val::from(177)])?,
                            ctx.call(Function::Rand, vec![Val::from(70), Val::from(120)])?,
                            Val::from("Cruiser"),
                            Val::from(1443),
                            Val::from(1),
                            Val::from("force_06ex#60::OnMyMobDead"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            Force06ex60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06ex_60(ctx: &Ctx) -> Script {
    force_06ex_60_run(ctx, Force06ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06ex_60_onreset(ctx: &Ctx) -> Script {
    force_06ex_60_run(ctx, Force06ex60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06ex_60_onenable(ctx: &Ctx) -> Script {
    force_06ex_60_run(ctx, Force06ex60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06ex_60_onsubon(ctx: &Ctx) -> Script {
    force_06ex_60_run(ctx, Force06ex60Step::OnSubOn, Vec::new()).map(|_| ())
}

pub fn force_06ex_60_onmymobdead(ctx: &Ctx) -> Script {
    force_06ex_60_run(ctx, Force06ex60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mob60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_06mob_60_run(ctx: &Ctx, mut step: Force06mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mob60Step::Start => {
                step = Force06mob60Step::OnReset;
                continue 'machine;
            }
            Force06mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_06mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mob60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#60::OnEnable")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(174),
                        Val::from(100),
                        Val::from("Miyabi Doll"),
                        Val::from(1552),
                        Val::from(1),
                        Val::from("force_06mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(174),
                        Val::from(80),
                        Val::from("Miyabi Doll"),
                        Val::from(1552),
                        Val::from(1),
                        Val::from("force_06mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_06mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On06_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_06")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#60::OnSubOn")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_60(ctx: &Ctx) -> Script {
    force_06mob_60_run(ctx, Force06mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_60_onreset(ctx: &Ctx) -> Script {
    force_06mob_60_run(ctx, Force06mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_60_onenable(ctx: &Ctx) -> Script {
    force_06mob_60_run(ctx, Force06mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_60_run(ctx, Force06mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07start60Step {
    Start,
    OnEnable,
}

fn force_07start_60_run(ctx: &Ctx, mut step: Force07start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07start60Step::Start => {
                step = Force07start60Step::OnEnable;
                continue 'machine;
            }
            Force07start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07start_60(ctx: &Ctx) -> Script {
    force_07start_60_run(ctx, Force07start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_60_onenable(ctx: &Ctx) -> Script {
    force_07start_60_run(ctx, Force07start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mob60Step {
    Start,
    OnReset,
    OnEnable,
    OnMyMobDead,
}

fn force_07mob_60_run(ctx: &Ctx, mut step: Force07mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mob60Step::Start => {
                step = Force07mob60Step::OnReset;
                continue 'machine;
            }
            Force07mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_07mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mob60Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(170),
                        Val::from(25),
                        Val::from("Jakk"),
                        Val::from(1436),
                        Val::from(1),
                        Val::from("force_07mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(170),
                        Val::from(25),
                        Val::from("Jakk"),
                        Val::from(1436),
                        Val::from(1),
                        Val::from("force_07mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(175),
                        Val::from(25),
                        Val::from("Myst"),
                        Val::from(1553),
                        Val::from(1),
                        Val::from("force_07mob#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(179),
                        Val::from(25),
                        Val::from("Isis"),
                        Val::from(1421),
                        Val::from(1),
                        Val::from("force_07mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_07mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On07_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_07")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_60(ctx: &Ctx) -> Script {
    force_07mob_60_run(ctx, Force07mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_60_onreset(ctx: &Ctx) -> Script {
    force_07mob_60_run(ctx, Force07mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_60_onenable(ctx: &Ctx) -> Script {
    force_07mob_60_run(ctx, Force07mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_60_run(ctx, Force07mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08start60Step {
    Start,
    OnEnable,
}

fn force_08start_60_run(ctx: &Ctx, mut step: Force08start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08start60Step::Start => {
                step = Force08start60Step::OnEnable;
                continue 'machine;
            }
            Force08start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08ex#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_60(ctx: &Ctx) -> Script {
    force_08start_60_run(ctx, Force08start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_60_onenable(ctx: &Ctx) -> Script {
    force_08start_60_run(ctx, Force08start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08ex60Step {
    Start,
    OnEnable,
}

fn force_08ex_60_run(ctx: &Ctx, mut step: Force08ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08ex60Step::Start => {
                step = Force08ex60Step::OnEnable;
                continue 'machine;
            }
            Force08ex60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_08")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08ex_60(ctx: &Ctx) -> Script {
    force_08ex_60_run(ctx, Force08ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08ex_60_onenable(ctx: &Ctx) -> Script {
    force_08ex_60_run(ctx, Force08ex60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09start60Step {
    Start,
    OnEnable,
}

fn force_09start_60_run(ctx: &Ctx, mut step: Force09start60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09start60Step::Start => {
                step = Force09start60Step::OnEnable;
                continue 'machine;
            }
            Force09start60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#60::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_60(ctx: &Ctx) -> Script {
    force_09start_60_run(ctx, Force09start60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_60_onenable(ctx: &Ctx) -> Script {
    force_09start_60_run(ctx, Force09start60Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09ex60Step {
    Start,
    OnReset,
    OnSummonMob09,
    OnMyMobDead,
}

fn force_09ex_60_run(ctx: &Ctx, mut step: Force09ex60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09ex60Step::Start => {
                step = Force09ex60Step::OnReset;
                continue 'machine;
            }
            Force09ex60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_09ex#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09ex60Step::OnSummonMob09 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(90),
                        Val::from(100),
                        Val::from("Horong"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(92),
                        Val::from(100),
                        Val::from("Horong"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(99),
                        Val::from(100),
                        Val::from("Horong"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(105),
                        Val::from(100),
                        Val::from("Horong"),
                        Val::from(1578),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(91),
                        Val::from(108),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(91),
                        Val::from(104),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(91),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(91),
                        Val::from(96),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(91),
                        Val::from(92),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(95),
                        Val::from(108),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(99),
                        Val::from(108),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(100),
                        Val::from(108),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(104),
                        Val::from(108),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(108),
                        Val::from(108),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(108),
                        Val::from(104),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(108),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(108),
                        Val::from(96),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(108),
                        Val::from(92),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(104),
                        Val::from(102),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(196),
                        Val::from(102),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(114),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(121),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(85),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(78),
                        Val::from(100),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(96),
                        Val::from(118),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(103),
                        Val::from(118),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_09ex#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09ex60Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09ex_60(ctx: &Ctx) -> Script {
    force_09ex_60_run(ctx, Force09ex60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09ex_60_onreset(ctx: &Ctx) -> Script {
    force_09ex_60_run(ctx, Force09ex60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09ex_60_onsummonmob_09(ctx: &Ctx) -> Script {
    force_09ex_60_run(ctx, Force09ex60Step::OnSummonMob09, Vec::new()).map(|_| ())
}

pub fn force_09ex_60_onmymobdead(ctx: &Ctx) -> Script {
    force_09ex_60_run(ctx, Force09ex60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09mob60Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_60_run(ctx: &Ctx, mut step: Force09mob60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mob60Step::Start => {
                step = Force09mob60Step::OnEnable;
                continue 'machine;
            }
            Force09mob60Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#60::OnSummonMob_09")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(99),
                        Val::from(99),
                        Val::from("Goblin Leader"),
                        Val::from(1539),
                        Val::from(1),
                        Val::from("force_09mob#60::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09mob60Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_2-1"), Val::from("force_09mob#60::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mob60Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_2-1"), Val::from("force_09mob#60::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::On09_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_09")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_All")])?;
                    ctx.var("$arena_min60end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                    ctx.var("$arena_sec60end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_60(ctx: &Ctx) -> Script {
    force_09mob_60_run(ctx, Force09mob60Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_60_onenable(ctx: &Ctx) -> Script {
    force_09mob_60_run(ctx, Force09mob60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_09mob_60_onreset(ctx: &Ctx) -> Script {
    force_09mob_60_run(ctx, Force09mob60Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_60_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_60_run(ctx, Force09mob60Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn staff_60_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Staff",
        args![
            "You did a good job.",
            "Even if you have failed to clear a time attack battle, I will reward you with a small amount of arena points."
        ],
    )?;
    ctx.next()?;
    if ctx.var("arena_point").get()? == 30000 {
        ctx.lines_as(
            "Staff",
            args![
                "Uh huh!",
                "You already have enough arena points.",
                "Please spend some arena points later. When I see you next time, I will make sure to give you some reward."
            ],
        )?;
        ctx.next()?;
    } else {
        ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(1)))?;
    }
    ctx.lines_as("Staff", args!["Let me guide you outside. I hope you had a good time."])?;
    ctx.close_window()?;
    ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
    ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
    return Err(Stop::End);
}

pub fn staff_60_1(ctx: &Ctx) -> Script {
    staff_60_1_body(ctx, Vec::new()).map(|_| ())
}

fn staff_60_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$arena_min60end").get()?, "<", &ctx.var("$arena_min60st").get()?)?.is_true() {
        if runtime::op(&ctx.var("$arena_sec60end").get()?, "<", &ctx.var("$arena_sec60st").get()?)?.is_true() {
            ctx.var("@record_min60").set(
                (((Val::from(60).try_sub(ctx.var("$arena_min60st").get()?)?) + ctx.var("$arena_min60end").get()?).try_sub(Val::from(1))?),
            )?;
            ctx.var("@record_sec60")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec60st").get()?)?) + ctx.var("$arena_sec60end").get()?))?;
        } else {
            ctx.var("@record_min60")
                .set(((Val::from(60).try_sub(ctx.var("$arena_min60st").get()?)?) + ctx.var("$arena_min60end").get()?))?;
            ctx.var("@record_sec60")
                .set((ctx.var("$arena_sec60end").get()?.try_sub(ctx.var("$arena_sec60st").get()?)?))?;
        }
    } else {
        if runtime::op(&ctx.var("$arena_sec60end").get()?, "<", &ctx.var("$arena_sec60st").get()?)?.is_true() {
            ctx.var("@record_min60")
                .set(((ctx.var("$arena_min60end").get()?.try_sub(ctx.var("$arena_min60st").get()?)?).try_sub(Val::from(1))?))?;
            ctx.var("@record_sec60")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec60st").get()?)?) + ctx.var("$arena_sec60end").get()?))?;
        } else {
            ctx.var("@record_min60")
                .set((ctx.var("$arena_min60end").get()?.try_sub(ctx.var("$arena_min60st").get()?)?))?;
            ctx.var("@record_sec60")
                .set((ctx.var("$arena_sec60end").get()?.try_sub(ctx.var("$arena_sec60st").get()?)?))?;
        }
    }
    ctx.var("@gap60").set(
        (((Val::from(60).try_mul(ctx.var("$top_60min").get()?)?) + ctx.var("$top_60sec").get()?)
            .try_sub(((Val::from(60).try_mul(ctx.var("@record_min60").get()?)?) + ctx.var("@record_sec60").get()?))?),
    )?;
    ctx.lines_as(
        "Staff",
        args![
            "Wow, you did a good job~ ",
            ((Val::from("Your name is...^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                + Val::from("^000000, isn't it?")),
            ((Val::from("^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                + Val::from("^000000, total time you spent to pass the battle.."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((Val::from("is ") + ctx.var("@record_min60").get()?) + Val::from("minutes ")) + ctx.var("@record_sec60").get()?)
                + Val::from("seconds.")),
            "Congratulations!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((Val::from("The fastest player among people who cleared Lv60 arena time force battle is ^3131FF")
                + ctx.var("$arena_60topn$").get()?)
                + Val::from("^000000."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((((Val::from("^3131FF") + ctx.var("$arena_60topn$").get()?) + Val::from("^000000's running time was ^3131FF"))
                + ctx.var("$top_60min").get()?)
                + Val::from("^000000minutes ^3131FF"))
                + ctx.var("$top_60sec").get()?)
                + Val::from("^000000seconds."))
        ],
    )?;
    ctx.next()?;
    if ctx.var("@gap60").get()?.number()? < 0 {
        ctx.lines_as(
            "Staff",
            args!["Although you failed to make a new record, I hope you will succeed next time."],
        )?;
        ctx.next()?;
        if ctx.var("arena_point").get()?.number()? > 29980 {
            ctx.lines_as(
                "Staff",
                args![
                    "Then let me reward you with some arena points....eh?",
                    "Your arena points have exceeded the maximum amount. I cannot give you more points until you spend some points."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args!["You can check the amount of arena points you have in the arena waiting room."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args![
                    "I hope you had a good time and let me guide you to the entrance of arena.",
                    "Thank you."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(20)))?;
            ctx.lines_as("Staff", args!["Let me reward you some arena points.", "If you wish to check the amount of arena points you have, please go talk to ^3131FFVendigos^000000 at the arena entrance."])?;
            ctx.next()?;
            ctx.lines_as("Staff", args!["Let me guide you to the entrance of arena.", "See you later~"])?;
            ctx.close_window()?;
        }
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_EXIT")?])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#60::OnNomal1")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_60::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#60::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv60 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as("Staff", args!["Wow! You have renewed the record!", "What a great job!"])?;
        ctx.next()?;
        ctx.lines_as("Staff", args![((Val::from("You have been recorded as the fastest player among people who cleared ^FF0000Arena Time Force Battle lvl 60s^000000, ^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000."))])?;
        ctx.var("$top_60min").set(ctx.var("@record_min60").get()?)?;
        ctx.var("$top_60sec").set(ctx.var("@record_sec60").get()?)?;
        ctx.var("$arena_60topn$")
            .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_60")])?;
        ctx.next()?;
        if ctx.var("arena_point").get()?.number()? > 29950 {
            ctx.lines_as(
                "Staff",
                args![
                    "Then let me reward you with some arena points....eh?",
                    "Your arena points have exceeded the maximum amount. I cannot give you more points until you spend some points."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args!["You can check the amount of arena points you have in the arena waiting room."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Staff",
                args![
                    "I hope you had a good time and let me guide you to the entrance of arena.",
                    "Thank you."
                ],
            )?;
            ctx.close_window()?;
        } else {
            ctx.lines_as(
                "Staff",
                args![
                    "Let me reward you with some arena points.",
                    "At the same time, since you have renewed the record you will receive an extra amount of the points this time."
                ],
            )?;
            ctx.next()?;
            ctx.var("arena_point").set((ctx.var("arena_point").get()? + Val::from(50)))?;
            ctx.lines_as("Staff", args!["Let me reward you some arena points.", "If you wish to check the amount of arena points you have, please go talk to ^3131FFVendigos^000000 at the arena entrance."])?;
            ctx.next()?;
            ctx.lines_as("Staff", args!["Let me guide you to the entrance of arena.", "See you later~"])?;
            ctx.close_window()?;
        }
        ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#60::OnNomal2")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_60::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#60::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv60 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    }
}

pub fn staff_60_2(ctx: &Ctx) -> Script {
    staff_60_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnTimer60Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer60000,
    OnStop,
}

fn arn_timer_60_run(ctx: &Ctx, mut step: ArnTimer60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnTimer60Step::Start => {
                step = ArnTimer60Step::OnEnable;
                continue 'machine;
            }
            ArnTimer60Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArnTimer60Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("This broadcast informs you about the restriction for arena lvl 60s."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimer60Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("For a smooth game play, exit warp portal will be activated in 1 minute."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimer60Step::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("prt_are_in"), Val::from("Please proceed your battle quickly as possible in order to avoid disadvantage. Thank you for your cooperation."), Val::from(0), Val::from(16764416)])?;
                return Err(Stop::End);
            }
            ArnTimer60Step::OnTimer60000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#60::OnTimeOver2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_60::OnOut")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_60::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv60 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            ArnTimer60Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_timer_60(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_timer_60_onenable(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arn_timer_60_ontimer2000(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arn_timer_60_ontimer3000(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arn_timer_60_ontimer4000(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn arn_timer_60_ontimer60000(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arn_timer_60_onstop(ctx: &Ctx) -> Script {
    arn_timer_60_run(ctx, ArnTimer60Step::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnWarp60Step {
    Start,
    OnOut,
}

fn arn_warp_60_run(ctx: &Ctx, mut step: ArnWarp60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnWarp60Step::Start => {
                step = ArnWarp60Step::OnOut;
                continue 'machine;
            }
            ArnWarp60Step::OnOut => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from(14),
                        Val::from(143),
                        Val::from(29),
                        Val::from(126),
                        Val::from("arena_room"),
                        Val::from(100),
                        Val::from(75),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_warp_60(ctx: &Ctx) -> Script {
    arn_warp_60_run(ctx, ArnWarp60Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_warp_60_onout(ctx: &Ctx) -> Script {
    arn_warp_60_run(ctx, ArnWarp60Step::OnOut, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Cast60Step {
    Start,
    OnTimeOver1,
    OnNomal1,
    OnNomal2,
    OnTimeOver2,
}

fn cast_60_run(ctx: &Ctx, mut step: Cast60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Cast60Step::Start => {
                step = Cast60Step::OnTimeOver1;
                continue 'machine;
            }
            Cast60Step::OnTimeOver1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated due to an error occurred during battle."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast60Step::OnNomal1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast60Step::OnNomal2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast60Step::OnTimeOver2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Arena will be reactivated due to an error occurred in the waiting room."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cast_60(ctx: &Ctx) -> Script {
    cast_60_run(ctx, Cast60Step::Start, Vec::new()).map(|_| ())
}

pub fn cast_60_ontimeover1(ctx: &Ctx) -> Script {
    cast_60_run(ctx, Cast60Step::OnTimeOver1, Vec::new()).map(|_| ())
}

pub fn cast_60_onnomal1(ctx: &Ctx) -> Script {
    cast_60_run(ctx, Cast60Step::OnNomal1, Vec::new()).map(|_| ())
}

pub fn cast_60_onnomal2(ctx: &Ctx) -> Script {
    cast_60_run(ctx, Cast60Step::OnNomal2, Vec::new()).map(|_| ())
}

pub fn cast_60_ontimeover2(ctx: &Ctx) -> Script {
    cast_60_run(ctx, Cast60Step::OnTimeOver2, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Alloff60Step {
    Start,
    OnEnable,
    OnInit,
}

fn alloff_60_run(ctx: &Ctx, mut step: Alloff60Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Alloff60Step::Start => {
                step = Alloff60Step::OnEnable;
                continue 'machine;
            }
            Alloff60Step::OnEnable => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(139),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#60::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnTimerOff")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Minilover#arena")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#60")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arena#60")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_60::OnStop")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Minilover#arena")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("arena#60")])?;
                return Err(Stop::End);
            }
            Alloff60Step::OnInit => {
                if (!(ctx.var("$top_60min").get()?.is_true()) && !(ctx.var("$top_60sec").get()?.is_true())) {
                    ctx.var("$top_60min").set(Val::from(6))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn alloff_60(ctx: &Ctx) -> Script {
    alloff_60_run(ctx, Alloff60Step::Start, Vec::new()).map(|_| ())
}

pub fn alloff_60_onenable(ctx: &Ctx) -> Script {
    alloff_60_run(ctx, Alloff60Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn alloff_60_oninit(ctx: &Ctx) -> Script {
    alloff_60_run(ctx, Alloff60Step::OnInit, Vec::new()).map(|_| ())
}
