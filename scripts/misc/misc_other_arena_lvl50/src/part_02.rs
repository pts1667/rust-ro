use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force04mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_04mob_50_run(ctx: &Ctx, mut step: Force04mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04mob50Step::Start => {
                step = Force04mob50Step::OnEnable;
                continue 'machine;
            }
            Force04mob50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#50::OnSummonMob_04")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(85),
                        Val::from(172),
                        Val::from("Kobold"),
                        Val::from(1547),
                        Val::from(1),
                        Val::from("force_04mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(95),
                        Val::from(172),
                        Val::from("Kobold"),
                        Val::from(1547),
                        Val::from(1),
                        Val::from("force_04mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(120),
                        Val::from(172),
                        Val::from("Kobold"),
                        Val::from(1545),
                        Val::from(1),
                        Val::from("force_04mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_04mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_04mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On04_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_04")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04mob_50(ctx: &Ctx) -> Script {
    force_04mob_50_run(ctx, Force04mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_04mob_50_onenable(ctx: &Ctx) -> Script {
    force_04mob_50_run(ctx, Force04mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_04mob_50_onreset(ctx: &Ctx) -> Script {
    force_04mob_50_run(ctx, Force04mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_04mob_50_run(ctx, Force04mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05start50Step {
    Start,
    OnEnable,
}

fn force_05start_50_run(ctx: &Ctx, mut step: Force05start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05start50Step::Start => {
                step = Force05start50Step::OnEnable;
                continue 'machine;
            }
            Force05start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_50(ctx: &Ctx) -> Script {
    force_05start_50_run(ctx, Force05start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_50_onenable(ctx: &Ctx) -> Script {
    force_05start_50_run(ctx, Force05start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05ex50Step {
    Start,
    OnReset,
    OnSummonMob05,
    OnMyMobDead,
}

fn force_05ex_50_run(ctx: &Ctx, mut step: Force05ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05ex50Step::Start => {
                step = Force05ex50Step::OnReset;
                continue 'machine;
            }
            Force05ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_05ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05ex50Step::OnSummonMob05 => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_1-1"),
                            Val::from(174),
                            Val::from(174),
                            Val::from("Desert Wolf"),
                            Val::from(1432),
                            Val::from(1),
                            Val::from("force_05ex#50::OnMyMobDead"),
                        ],
                    )?;
                } else if subject1 == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_1-1"),
                            Val::from(173),
                            Val::from(173),
                            Val::from("Zerom"),
                            Val::from(1470),
                            Val::from(1),
                            Val::from("force_05ex#50::OnMyMobDead"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            Force05ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05ex_50(ctx: &Ctx) -> Script {
    force_05ex_50_run(ctx, Force05ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05ex_50_onreset(ctx: &Ctx) -> Script {
    force_05ex_50_run(ctx, Force05ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05ex_50_onsummonmob_05(ctx: &Ctx) -> Script {
    force_05ex_50_run(ctx, Force05ex50Step::OnSummonMob05, Vec::new()).map(|_| ())
}

pub fn force_05ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_05ex_50_run(ctx, Force05ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_05mob_50_run(ctx: &Ctx, mut step: Force05mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mob50Step::Start => {
                step = Force05mob50Step::OnEnable;
                continue 'machine;
            }
            Force05mob50Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(166),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(170),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(177),
                        Val::from(170),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(173),
                        Val::from("Orc Lady"),
                        Val::from(1452),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(173),
                        Val::from("Orc Lady"),
                        Val::from(1452),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(181),
                        Val::from(173),
                        Val::from("Golem"),
                        Val::from(1540),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(177),
                        Val::from("Golem"),
                        Val::from(1540),
                        Val::from(1),
                        Val::from("force_05mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_05mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_05mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On05_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_05")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#50::OnSummonMob_05")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_50(ctx: &Ctx) -> Script {
    force_05mob_50_run(ctx, Force05mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_50_onenable(ctx: &Ctx) -> Script {
    force_05mob_50_run(ctx, Force05mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_50_onreset(ctx: &Ctx) -> Script {
    force_05mob_50_run(ctx, Force05mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_50_run(ctx, Force05mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06start50Step {
    Start,
    OnEnable,
}

fn force_06start_50_run(ctx: &Ctx, mut step: Force06start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06start50Step::Start => {
                step = Force06start50Step::OnEnable;
                continue 'machine;
            }
            Force06start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_50(ctx: &Ctx) -> Script {
    force_06start_50_run(ctx, Force06start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_50_onenable(ctx: &Ctx) -> Script {
    force_06start_50_run(ctx, Force06start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06ex50Step {
    Start,
    OnReset,
    OnSummonMob06,
    OnMyMobDead,
}

fn force_06ex_50_run(ctx: &Ctx, mut step: Force06ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06ex50Step::Start => {
                step = Force06ex50Step::OnReset;
                continue 'machine;
            }
            Force06ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_06ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06ex50Step::OnSummonMob06 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(118),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(108),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(99),
                        Val::from("Smokie"),
                        Val::from(1561),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(108),
                        Val::from("Golem"),
                        Val::from(1540),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(99),
                        Val::from("Skel Worker"),
                        Val::from(1469),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(85),
                        Val::from("Skel Worker"),
                        Val::from(1469),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(90),
                        Val::from("Golem"),
                        Val::from(1540),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(79),
                        Val::from("Scorpion"),
                        Val::from(1559),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(175),
                        Val::from(118),
                        Val::from("Scorpion"),
                        Val::from(1559),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(175),
                        Val::from(108),
                        Val::from("Sandman"),
                        Val::from(1558),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(175),
                        Val::from(99),
                        Val::from("Sandman"),
                        Val::from(1558),
                        Val::from(1),
                        Val::from("force_06ex#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06ex_50(ctx: &Ctx) -> Script {
    force_06ex_50_run(ctx, Force06ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06ex_50_onreset(ctx: &Ctx) -> Script {
    force_06ex_50_run(ctx, Force06ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06ex_50_onsummonmob_06(ctx: &Ctx) -> Script {
    force_06ex_50_run(ctx, Force06ex50Step::OnSummonMob06, Vec::new()).map(|_| ())
}

pub fn force_06ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_06ex_50_run(ctx, Force06ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_06mob_50_run(ctx: &Ctx, mut step: Force06mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mob50Step::Start => {
                step = Force06mob50Step::OnEnable;
                continue 'machine;
            }
            Force06mob50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#50::OnSummonMob_06")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(90),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_06mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(173),
                        Val::from(79),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_06mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(172),
                        Val::from(70),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_06mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(175),
                        Val::from(70),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_06mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(118),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(1),
                        Val::from("force_06mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_06mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_06mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On06_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_06")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_50(ctx: &Ctx) -> Script {
    force_06mob_50_run(ctx, Force06mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_50_onenable(ctx: &Ctx) -> Script {
    force_06mob_50_run(ctx, Force06mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06mob_50_onreset(ctx: &Ctx) -> Script {
    force_06mob_50_run(ctx, Force06mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_50_run(ctx, Force06mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07start50Step {
    Start,
    OnEnable,
}

fn force_07start_50_run(ctx: &Ctx, mut step: Force07start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07start50Step::Start => {
                step = Force07start50Step::OnEnable;
                continue 'machine;
            }
            Force07start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07start_50(ctx: &Ctx) -> Script {
    force_07start_50_run(ctx, Force07start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_50_onenable(ctx: &Ctx) -> Script {
    force_07start_50_run(ctx, Force07start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07ex50Step {
    Start,
    OnReset,
    OnSummonMob07,
    OnMyMobDead,
}

fn force_07ex_50_run(ctx: &Ctx, mut step: Force07ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07ex50Step::Start => {
                step = Force07ex50Step::OnReset;
                continue 'machine;
            }
            Force07ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_07ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07ex50Step::OnSummonMob07 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(36),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(167),
                        Val::from(36),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(36),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(175),
                        Val::from(36),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(179),
                        Val::from(36),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(36),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(32),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(28),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(24),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(20),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(183),
                        Val::from(16),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(179),
                        Val::from(16),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(175),
                        Val::from(16),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(171),
                        Val::from(16),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(167),
                        Val::from(16),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(16),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(20),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(24),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(28),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(32),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(31),
                        Val::from("Punk"),
                        Val::from(1481),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(163),
                        Val::from(23),
                        Val::from("Punk"),
                        Val::from(1481),
                        Val::from(1),
                        Val::from("force_07ex#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07ex_50(ctx: &Ctx) -> Script {
    force_07ex_50_run(ctx, Force07ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07ex_50_onreset(ctx: &Ctx) -> Script {
    force_07ex_50_run(ctx, Force07ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07ex_50_onsummonmob_07(ctx: &Ctx) -> Script {
    force_07ex_50_run(ctx, Force07ex50Step::OnSummonMob07, Vec::new()).map(|_| ())
}

pub fn force_07ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_07ex_50_run(ctx, Force07ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_07mob_50_run(ctx: &Ctx, mut step: Force07mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mob50Step::Start => {
                step = Force07mob50Step::OnEnable;
                continue 'machine;
            }
            Force07mob50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#50::OnSummonMob_07")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(174),
                        Val::from(25),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(1),
                        Val::from("force_07mob#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_07mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_07mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On07_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_07")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_50(ctx: &Ctx) -> Script {
    force_07mob_50_run(ctx, Force07mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_50_onenable(ctx: &Ctx) -> Script {
    force_07mob_50_run(ctx, Force07mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_50_onreset(ctx: &Ctx) -> Script {
    force_07mob_50_run(ctx, Force07mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_50_run(ctx, Force07mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08start50Step {
    Start,
    OnEnable,
}

fn force_08start_50_run(ctx: &Ctx, mut step: Force08start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08start50Step::Start => {
                step = Force08start50Step::OnEnable;
                continue 'machine;
            }
            Force08start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08ex#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_50(ctx: &Ctx) -> Script {
    force_08start_50_run(ctx, Force08start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_50_onenable(ctx: &Ctx) -> Script {
    force_08start_50_run(ctx, Force08start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08ex50Step {
    Start,
    OnEnable,
}

fn force_08ex_50_run(ctx: &Ctx, mut step: Force08ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08ex50Step::Start => {
                step = Force08ex50Step::OnEnable;
                continue 'machine;
            }
            Force08ex50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_08")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08ex_50(ctx: &Ctx) -> Script {
    force_08ex_50_run(ctx, Force08ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08ex_50_onenable(ctx: &Ctx) -> Script {
    force_08ex_50_run(ctx, Force08ex50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09start50Step {
    Start,
    OnEnable,
}

fn force_09start_50_run(ctx: &Ctx, mut step: Force09start50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09start50Step::Start => {
                step = Force09start50Step::OnEnable;
                continue 'machine;
            }
            Force09start50Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#50::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_50(ctx: &Ctx) -> Script {
    force_09start_50_run(ctx, Force09start50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_50_onenable(ctx: &Ctx) -> Script {
    force_09start_50_run(ctx, Force09start50Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09ex50Step {
    Start,
    OnReset,
    OnSummonMob09,
    OnMyMobDead,
}

fn force_09ex_50_run(ctx: &Ctx, mut step: Force09ex50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09ex50Step::Start => {
                step = Force09ex50Step::OnReset;
                continue 'machine;
            }
            Force09ex50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_09ex#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09ex50Step::OnSummonMob09 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(110),
                        Val::from(110),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(89),
                        Val::from(110),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(89),
                        Val::from(89),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(110),
                        Val::from(89),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(105),
                        Val::from(105),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(94),
                        Val::from(105),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(94),
                        Val::from(94),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(110),
                        Val::from(110),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(89),
                        Val::from(110),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(89),
                        Val::from(89),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(110),
                        Val::from(89),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(105),
                        Val::from(105),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(94),
                        Val::from(105),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(94),
                        Val::from(94),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(105),
                        Val::from(94),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#50::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09ex50Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09ex_50(ctx: &Ctx) -> Script {
    force_09ex_50_run(ctx, Force09ex50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09ex_50_onreset(ctx: &Ctx) -> Script {
    force_09ex_50_run(ctx, Force09ex50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09ex_50_onsummonmob_09(ctx: &Ctx) -> Script {
    force_09ex_50_run(ctx, Force09ex50Step::OnSummonMob09, Vec::new()).map(|_| ())
}

pub fn force_09ex_50_onmymobdead(ctx: &Ctx) -> Script {
    force_09ex_50_run(ctx, Force09ex50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09mob50Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_50_run(ctx: &Ctx, mut step: Force09mob50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mob50Step::Start => {
                step = Force09mob50Step::OnEnable;
                continue 'machine;
            }
            Force09mob50Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-1"),
                        Val::from(99),
                        Val::from(99),
                        Val::from("Vocal"),
                        Val::from(1581),
                        Val::from(1),
                        Val::from("force_09mob#50::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#50::OnSummonMob_09")])?;
                return Err(Stop::End);
            }
            Force09mob50Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-1"), Val::from("force_09mob#50::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mob50Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-1"), Val::from("force_09mob#50::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::On09_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_09")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_All")])?;
                    ctx.var("$arena_min50end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                    ctx.var("$arena_sec50end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_50(ctx: &Ctx) -> Script {
    force_09mob_50_run(ctx, Force09mob50Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_50_onenable(ctx: &Ctx) -> Script {
    force_09mob_50_run(ctx, Force09mob50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_09mob_50_onreset(ctx: &Ctx) -> Script {
    force_09mob_50_run(ctx, Force09mob50Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_50_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_50_run(ctx, Force09mob50Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn staff_50_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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

pub fn staff_50_1(ctx: &Ctx) -> Script {
    staff_50_1_body(ctx, Vec::new()).map(|_| ())
}

fn staff_50_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$arena_min50end").get()?, "<", &ctx.var("$arena_min50st").get()?)?.is_true() {
        if runtime::op(&ctx.var("$arena_sec50end").get()?, "<", &ctx.var("$arena_sec50st").get()?)?.is_true() {
            ctx.var("@record_min50").set(
                (((Val::from(60).try_sub(ctx.var("$arena_min50st").get()?)?) + ctx.var("$arena_min50end").get()?).try_sub(Val::from(1))?),
            )?;
            ctx.var("@record_sec50")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec50st").get()?)?) + ctx.var("$arena_sec50end").get()?))?;
        } else {
            ctx.var("@record_min50")
                .set(((Val::from(60).try_sub(ctx.var("$arena_min50st").get()?)?) + ctx.var("$arena_min50end").get()?))?;
            ctx.var("@record_sec50")
                .set((ctx.var("$arena_sec50end").get()?.try_sub(ctx.var("$arena_sec50st").get()?)?))?;
        }
    } else {
        if runtime::op(&ctx.var("$arena_sec50end").get()?, "<", &ctx.var("$arena_sec50st").get()?)?.is_true() {
            ctx.var("@record_min50")
                .set(((ctx.var("$arena_min50end").get()?.try_sub(ctx.var("$arena_min50st").get()?)?).try_sub(Val::from(1))?))?;
            ctx.var("@record_sec50")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec50st").get()?)?) + ctx.var("$arena_sec50end").get()?))?;
        } else {
            ctx.var("@record_min50")
                .set((ctx.var("$arena_min50end").get()?.try_sub(ctx.var("$arena_min50st").get()?)?))?;
            ctx.var("@record_sec50")
                .set((ctx.var("$arena_sec50end").get()?.try_sub(ctx.var("$arena_sec50st").get()?)?))?;
        }
    }
    ctx.var("@gap50").set(
        (((Val::from(60).try_mul(ctx.var("$top_50min").get()?)?) + ctx.var("$top_50sec").get()?)
            .try_sub(((Val::from(60).try_mul(ctx.var("@record_min50").get()?)?) + ctx.var("@record_sec50").get()?))?),
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
            ((((Val::from("is ") + ctx.var("@record_min50").get()?) + Val::from("minutes ")) + ctx.var("@record_sec50").get()?)
                + Val::from("seconds.")),
            "Congratulations!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((Val::from("The fastest player among people who cleared lvl 50s arena time force battle is ^3131FF")
                + ctx.var("$arena_50topn$").get()?)
                + Val::from("^000000."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((((Val::from("^3131FF") + ctx.var("$arena_50topn$").get()?) + Val::from("^000000's running time was ^3131FF"))
                + ctx.var("$top_50min").get()?)
                + Val::from("^000000minutes ^3131FF"))
                + ctx.var("$top_50sec").get()?)
                + Val::from("^000000seconds."))
        ],
    )?;
    ctx.next()?;
    if ctx.var("@gap50").get()?.number()? < 0 {
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
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#50::OnNomal1")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_50::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#50::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv50 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as("Staff", args!["Wow! You have renewed the record!", "What a great job!"])?;
        ctx.next()?;
        ctx.lines_as("Staff", args![((Val::from("You have been recorded as the fastest player among people who cleared ^FF0000Arena Time Force Battle lvl 50s^000000, ^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000."))])?;
        ctx.var("$top_50min").set(ctx.var("@record_min50").get()?)?;
        ctx.var("$top_50sec").set(ctx.var("@record_sec50").get()?)?;
        ctx.var("$arena_50topn$")
            .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_50")])?;
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
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#50::OnNomal2")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_50::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#50::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv50 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    }
}

pub fn staff_50_2(ctx: &Ctx) -> Script {
    staff_50_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnTimer50Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer60000,
    OnStop,
}

fn arn_timer_50_run(ctx: &Ctx, mut step: ArnTimer50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnTimer50Step::Start => {
                step = ArnTimer50Step::OnEnable;
                continue 'machine;
            }
            ArnTimer50Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArnTimer50Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("This broadcast informs you about the restriction for arena lvl 50s."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimer50Step::OnTimer3000 => {
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
            ArnTimer50Step::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("prt_are_in"), Val::from("Please proceed your battle quickly as possible in order to avoid disadvantage. Thank you for your cooperation."), Val::from(0), Val::from(16764416)])?;
                return Err(Stop::End);
            }
            ArnTimer50Step::OnTimer60000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#50::OnTimeOver2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_50::OnOut")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_50::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv50 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            ArnTimer50Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_timer_50(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_timer_50_onenable(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arn_timer_50_ontimer2000(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arn_timer_50_ontimer3000(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arn_timer_50_ontimer4000(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn arn_timer_50_ontimer60000(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arn_timer_50_onstop(ctx: &Ctx) -> Script {
    arn_timer_50_run(ctx, ArnTimer50Step::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnWarp50Step {
    Start,
    OnOut,
}

fn arn_warp_50_run(ctx: &Ctx, mut step: ArnWarp50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnWarp50Step::Start => {
                step = ArnWarp50Step::OnOut;
                continue 'machine;
            }
            ArnWarp50Step::OnOut => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from(14),
                        Val::from(195),
                        Val::from(29),
                        Val::from(178),
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

pub fn arn_warp_50(ctx: &Ctx) -> Script {
    arn_warp_50_run(ctx, ArnWarp50Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_warp_50_onout(ctx: &Ctx) -> Script {
    arn_warp_50_run(ctx, ArnWarp50Step::OnOut, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Cast50Step {
    Start,
    OnTimeOver1,
    OnTimeOver2,
    OnNomal1,
    OnNomal2,
}

fn cast_50_run(ctx: &Ctx, mut step: Cast50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Cast50Step::Start => {
                step = Cast50Step::OnTimeOver1;
                continue 'machine;
            }
            Cast50Step::OnTimeOver1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Arena will be reactivated due to an error occurred during battle."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast50Step::OnTimeOver2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Arena will be activated due to an error occurred in the waiting room."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast50Step::OnNomal1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast50Step::OnNomal2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn cast_50(ctx: &Ctx) -> Script {
    cast_50_run(ctx, Cast50Step::Start, Vec::new()).map(|_| ())
}

pub fn cast_50_ontimeover1(ctx: &Ctx) -> Script {
    cast_50_run(ctx, Cast50Step::OnTimeOver1, Vec::new()).map(|_| ())
}

pub fn cast_50_ontimeover2(ctx: &Ctx) -> Script {
    cast_50_run(ctx, Cast50Step::OnTimeOver2, Vec::new()).map(|_| ())
}

pub fn cast_50_onnomal1(ctx: &Ctx) -> Script {
    cast_50_run(ctx, Cast50Step::OnNomal1, Vec::new()).map(|_| ())
}

pub fn cast_50_onnomal2(ctx: &Ctx) -> Script {
    cast_50_run(ctx, Cast50Step::OnNomal2, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Alloff50Step {
    Start,
    OnEnable,
    OnInit,
}

fn alloff_50_run(ctx: &Ctx, mut step: Alloff50Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Alloff50Step::Start => {
                step = Alloff50Step::OnEnable;
                continue 'machine;
            }
            Alloff50Step::OnEnable => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#50::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnTimerOff")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Heel and Toe#arena")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#50")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arena#50")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_50::OnStop")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Heel and Toe#arena")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("arena#50")])?;
                return Err(Stop::End);
            }
            Alloff50Step::OnInit => {
                if (!(ctx.var("$top_50min").get()?.is_true()) && !(ctx.var("$top_50sec").get()?.is_true())) {
                    ctx.var("$top_50min").set(Val::from(5))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn alloff_50(ctx: &Ctx) -> Script {
    alloff_50_run(ctx, Alloff50Step::Start, Vec::new()).map(|_| ())
}

pub fn alloff_50_onenable(ctx: &Ctx) -> Script {
    alloff_50_run(ctx, Alloff50Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn alloff_50_oninit(ctx: &Ctx) -> Script {
    alloff_50_run(ctx, Alloff50Step::OnInit, Vec::new()).map(|_| ())
}
