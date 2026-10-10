use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force06ex70Step {
    Start,
    OnReset,
    OnSummonMob06,
    OnMyMobDead,
}

fn force_06ex_70_run(ctx: &Ctx, mut step: Force06ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06ex70Step::Start => {
                step = Force06ex70Step::OnReset;
                continue 'machine;
            }
            Force06ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_06ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06ex70Step::OnSummonMob06 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(176),
                        Val::from(99),
                        Val::from("Goblin"),
                        Val::from(1534),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(170),
                        Val::from(86),
                        Val::from("Goblin"),
                        Val::from(1535),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(176),
                        Val::from(97),
                        Val::from("Goblin"),
                        Val::from(1535),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(170),
                        Val::from(108),
                        Val::from("Goblin"),
                        Val::from(1535),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(176),
                        Val::from(112),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(173),
                        Val::from(120),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(177),
                        Val::from(74),
                        Val::from("Goblin"),
                        Val::from(1536),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(173),
                        Val::from(118),
                        Val::from("Goblin"),
                        Val::from(1538),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(171),
                        Val::from(101),
                        Val::from("Goblin"),
                        Val::from(1538),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(174),
                        Val::from(118),
                        Val::from("Goblin"),
                        Val::from(1538),
                        Val::from(1),
                        Val::from("force_06ex#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06ex_70(ctx: &Ctx) -> Script {
    force_06ex_70_run(ctx, Force06ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06ex_70_onreset(ctx: &Ctx) -> Script {
    force_06ex_70_run(ctx, Force06ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06ex_70_onsummonmob_06(ctx: &Ctx) -> Script {
    force_06ex_70_run(ctx, Force06ex70Step::OnSummonMob06, Vec::new()).map(|_| ())
}

pub fn force_06ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_06ex_70_run(ctx, Force06ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_06mob_70_run(ctx: &Ctx, mut step: Force06mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mob70Step::Start => {
                step = Force06mob70Step::OnEnable;
                continue 'machine;
            }
            Force06mob70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#70::OnSummonMob_06")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(173),
                        Val::from(90),
                        Val::from("Goblin"),
                        Val::from(1537),
                        Val::from(1),
                        Val::from("force_06mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(173),
                        Val::from(79),
                        Val::from("Goblin"),
                        Val::from(1537),
                        Val::from(1),
                        Val::from("force_06mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(172),
                        Val::from(70),
                        Val::from("Goblin"),
                        Val::from(1537),
                        Val::from(1),
                        Val::from("force_06mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(175),
                        Val::from(70),
                        Val::from("Goblin"),
                        Val::from(1537),
                        Val::from(1),
                        Val::from("force_06mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(171),
                        Val::from(118),
                        Val::from("Goblin"),
                        Val::from(1537),
                        Val::from(1),
                        Val::from("force_06mob#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_06mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_06mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On06_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_06")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_70(ctx: &Ctx) -> Script {
    force_06mob_70_run(ctx, Force06mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_70_onenable(ctx: &Ctx) -> Script {
    force_06mob_70_run(ctx, Force06mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06mob_70_onreset(ctx: &Ctx) -> Script {
    force_06mob_70_run(ctx, Force06mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_70_run(ctx, Force06mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07start70Step {
    Start,
    OnEnable,
}

fn force_07start_70_run(ctx: &Ctx, mut step: Force07start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07start70Step::Start => {
                step = Force07start70Step::OnEnable;
                continue 'machine;
            }
            Force07start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07start_70(ctx: &Ctx) -> Script {
    force_07start_70_run(ctx, Force07start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_70_onenable(ctx: &Ctx) -> Script {
    force_07start_70_run(ctx, Force07start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07ex70Step {
    Start,
    OnReset,
    OnSummonMob07,
    OnMyMobDead,
}

fn force_07ex_70_run(ctx: &Ctx, mut step: Force07ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07ex70Step::Start => {
                step = Force07ex70Step::OnReset;
                continue 'machine;
            }
            Force07ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_07ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07ex70Step::OnSummonMob07 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(168),
                        Val::from(26),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(186),
                        Val::from(23),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(160),
                        Val::from(23),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(188),
                        Val::from(33),
                        Val::from("Nightmare"),
                        Val::from(1427),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(174),
                        Val::from(13),
                        Val::from("Raydric"),
                        Val::from(1453),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(163),
                        Val::from(15),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(164),
                        Val::from(15),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(163),
                        Val::from(16),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(164),
                        Val::from(16),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(187),
                        Val::from(13),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(187),
                        Val::from(14),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(186),
                        Val::from(14),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(186),
                        Val::from(14),
                        Val::from("Farmiliar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_07ex#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07ex_70(ctx: &Ctx) -> Script {
    force_07ex_70_run(ctx, Force07ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07ex_70_onreset(ctx: &Ctx) -> Script {
    force_07ex_70_run(ctx, Force07ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07ex_70_onsummonmob_07(ctx: &Ctx) -> Script {
    force_07ex_70_run(ctx, Force07ex70Step::OnSummonMob07, Vec::new()).map(|_| ())
}

pub fn force_07ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_07ex_70_run(ctx, Force07ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_07mob_70_run(ctx: &Ctx, mut step: Force07mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mob70Step::Start => {
                step = Force07mob70Step::OnEnable;
                continue 'machine;
            }
            Force07mob70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#70::OnSummonMob_07")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(174),
                        Val::from(25),
                        Val::from("Alice"),
                        Val::from(1521),
                        Val::from(1),
                        Val::from("force_07mob#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_07mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_07mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On07_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_07")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_70(ctx: &Ctx) -> Script {
    force_07mob_70_run(ctx, Force07mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_70_onenable(ctx: &Ctx) -> Script {
    force_07mob_70_run(ctx, Force07mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_70_onreset(ctx: &Ctx) -> Script {
    force_07mob_70_run(ctx, Force07mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_70_run(ctx, Force07mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08start70Step {
    Start,
    OnEnable,
}

fn force_08start_70_run(ctx: &Ctx, mut step: Force08start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08start70Step::Start => {
                step = Force08start70Step::OnEnable;
                continue 'machine;
            }
            Force08start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08ex#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_70(ctx: &Ctx) -> Script {
    force_08start_70_run(ctx, Force08start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_70_onenable(ctx: &Ctx) -> Script {
    force_08start_70_run(ctx, Force08start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08ex70Step {
    Start,
    OnEnable,
}

fn force_08ex_70_run(ctx: &Ctx, mut step: Force08ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08ex70Step::Start => {
                step = Force08ex70Step::OnEnable;
                continue 'machine;
            }
            Force08ex70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_08")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08ex_70(ctx: &Ctx) -> Script {
    force_08ex_70_run(ctx, Force08ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08ex_70_onenable(ctx: &Ctx) -> Script {
    force_08ex_70_run(ctx, Force08ex70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09start70Step {
    Start,
    OnEnable,
}

fn force_09start_70_run(ctx: &Ctx, mut step: Force09start70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09start70Step::Start => {
                step = Force09start70Step::OnEnable;
                continue 'machine;
            }
            Force09start70Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#70::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_70(ctx: &Ctx) -> Script {
    force_09start_70_run(ctx, Force09start70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_70_onenable(ctx: &Ctx) -> Script {
    force_09start_70_run(ctx, Force09start70Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09ex70Step {
    Start,
    OnReset,
    OnSummonMob09,
    OnMyMobDead,
}

fn force_09ex_70_run(ctx: &Ctx, mut step: Force09ex70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09ex70Step::Start => {
                step = Force09ex70Step::OnReset;
                continue 'machine;
            }
            Force09ex70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_09ex#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09ex70Step::OnSummonMob09 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(93),
                        Val::from(100),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(94),
                        Val::from(100),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(93),
                        Val::from(99),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(85),
                        Val::from(114),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(87),
                        Val::from(114),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(89),
                        Val::from(114),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(91),
                        Val::from(114),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(93),
                        Val::from(114),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(95),
                        Val::from(114),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(85),
                        Val::from(112),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(85),
                        Val::from(110),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(85),
                        Val::from(108),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(85),
                        Val::from(106),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(85),
                        Val::from(104),
                        Val::from("Familiar"),
                        Val::from(1419),
                        Val::from(1),
                        Val::from("force_09ex#70::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09ex70Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09ex_70(ctx: &Ctx) -> Script {
    force_09ex_70_run(ctx, Force09ex70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09ex_70_onreset(ctx: &Ctx) -> Script {
    force_09ex_70_run(ctx, Force09ex70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09ex_70_onsummonmob_09(ctx: &Ctx) -> Script {
    force_09ex_70_run(ctx, Force09ex70Step::OnSummonMob09, Vec::new()).map(|_| ())
}

pub fn force_09ex_70_onmymobdead(ctx: &Ctx) -> Script {
    force_09ex_70_run(ctx, Force09ex70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09mob70Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_70_run(ctx: &Ctx, mut step: Force09mob70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mob70Step::Start => {
                step = Force09mob70Step::OnEnable;
                continue 'machine;
            }
            Force09mob70Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(99),
                        Val::from(99),
                        Val::from("Kobold Leader"),
                        Val::from(1548),
                        Val::from(1),
                        Val::from("force_09mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(98),
                        Val::from(99),
                        Val::from("Kobold"),
                        Val::from(1545),
                        Val::from(1),
                        Val::from("force_09mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(100),
                        Val::from(99),
                        Val::from("Kobold"),
                        Val::from(1546),
                        Val::from(1),
                        Val::from("force_09mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_3-1"),
                        Val::from(99),
                        Val::from(98),
                        Val::from("Kobold"),
                        Val::from(1547),
                        Val::from(1),
                        Val::from("force_09mob#70::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#70::OnSummonMob_09")])?;
                return Err(Stop::End);
            }
            Force09mob70Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_3-1"), Val::from("force_09mob#70::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mob70Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_3-1"), Val::from("force_09mob#70::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::On09_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_09")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_All")])?;
                    ctx.var("$arena_min70end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                    ctx.var("$arena_sec70end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_70(ctx: &Ctx) -> Script {
    force_09mob_70_run(ctx, Force09mob70Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_70_onenable(ctx: &Ctx) -> Script {
    force_09mob_70_run(ctx, Force09mob70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_09mob_70_onreset(ctx: &Ctx) -> Script {
    force_09mob_70_run(ctx, Force09mob70Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_70_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_70_run(ctx, Force09mob70Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn staff_70_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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

pub fn staff_70_1(ctx: &Ctx) -> Script {
    staff_70_1_body(ctx, Vec::new()).map(|_| ())
}

fn staff_70_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$arena_min70end").get()?, "<", &ctx.var("$arena_min70st").get()?)?.is_true() {
        if runtime::op(&ctx.var("$arena_sec70end").get()?, "<", &ctx.var("$arena_sec70st").get()?)?.is_true() {
            ctx.var("@record_min70").set(
                (((Val::from(60).try_sub(ctx.var("$arena_min70st").get()?)?) + ctx.var("$arena_min70end").get()?).try_sub(Val::from(1))?),
            )?;
            ctx.var("@record_sec70")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec70st").get()?)?) + ctx.var("$arena_sec70end").get()?))?;
        } else {
            ctx.var("@record_min70")
                .set(((Val::from(60).try_sub(ctx.var("$arena_min70st").get()?)?) + ctx.var("$arena_min70end").get()?))?;
            ctx.var("@record_sec70")
                .set((ctx.var("$arena_sec70end").get()?.try_sub(ctx.var("$arena_sec70st").get()?)?))?;
        }
    } else {
        if runtime::op(&ctx.var("$arena_sec70end").get()?, "<", &ctx.var("$arena_sec70st").get()?)?.is_true() {
            ctx.var("@record_min70")
                .set(((ctx.var("$arena_min70end").get()?.try_sub(ctx.var("$arena_min70st").get()?)?).try_sub(Val::from(1))?))?;
            ctx.var("@record_sec70")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec70st").get()?)?) + ctx.var("$arena_sec70end").get()?))?;
        } else {
            ctx.var("@record_min70")
                .set((ctx.var("$arena_min70end").get()?.try_sub(ctx.var("$arena_min70st").get()?)?))?;
            ctx.var("@record_sec70")
                .set((ctx.var("$arena_sec70end").get()?.try_sub(ctx.var("$arena_sec70st").get()?)?))?;
        }
    }
    ctx.var("@gap70").set(
        (((Val::from(60).try_mul(ctx.var("$top_70min").get()?)?) + ctx.var("$top_70sec").get()?)
            .try_sub(((Val::from(60).try_mul(ctx.var("@record_min70").get()?)?) + ctx.var("@record_sec70").get()?))?),
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
            ((((Val::from("is ") + ctx.var("@record_min70").get()?) + Val::from("minutes ")) + ctx.var("@record_sec70").get()?)
                + Val::from("seconds.")),
            "Congratulations!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((Val::from("The fastest player among people who cleared Lv70 arena time force battle is ^3131FF")
                + ctx.var("$arena_70topn$").get()?)
                + Val::from("^000000."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((((Val::from("^3131FF") + ctx.var("$arena_70topn$").get()?) + Val::from("^000000's running time was ^3131FF"))
                + ctx.var("$top_70min").get()?)
                + Val::from("^000000minutes ^3131FF"))
                + ctx.var("$top_70sec").get()?)
                + Val::from("^000000seconds."))
        ],
    )?;
    ctx.next()?;
    if ctx.var("@gap70").get()?.number()? < 0 {
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
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#70::OnNomal1")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_70::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#70::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv70 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as("Staff", args!["Wow! You have renewed the record!", "What a great job!"])?;
        ctx.next()?;
        ctx.lines_as("Staff", args![((Val::from("You have been recorded as the fastest player among people who cleared ^FF0000Arena Time Force Battle lvl 70s^000000, ^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000."))])?;
        ctx.var("$top_70min").set(ctx.var("@record_min70").get()?)?;
        ctx.var("$top_70sec").set(ctx.var("@record_sec70").get()?)?;
        ctx.var("$arena_70topn$")
            .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_70")])?;
        ctx.next()?;
        if ctx.var("arena_point").get()?.number()? > 29970 {
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
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#70::OnNomal2")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_70::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#70::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv70 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    }
}

pub fn staff_70_2(ctx: &Ctx) -> Script {
    staff_70_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnTimer70Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer60000,
    OnStop,
}

fn arn_timer_70_run(ctx: &Ctx, mut step: ArnTimer70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnTimer70Step::Start => {
                step = ArnTimer70Step::OnEnable;
                continue 'machine;
            }
            ArnTimer70Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArnTimer70Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("This broadcast informs you about the restriction for arena lvl 70s."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimer70Step::OnTimer3000 => {
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
            ArnTimer70Step::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("prt_are_in"), Val::from("Please proceed your battle quickly as possible in order to avoid disadvantage. Thank you for your cooperation."), Val::from(0), Val::from(16764416)])?;
                return Err(Stop::End);
            }
            ArnTimer70Step::OnTimer60000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#70::OnTimeOver2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_70::OnOut")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_70::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv70 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            ArnTimer70Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_timer_70(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_timer_70_onenable(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arn_timer_70_ontimer2000(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arn_timer_70_ontimer3000(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arn_timer_70_ontimer4000(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn arn_timer_70_ontimer60000(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arn_timer_70_onstop(ctx: &Ctx) -> Script {
    arn_timer_70_run(ctx, ArnTimer70Step::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnWarp70Step {
    Start,
    OnOut,
}

fn arn_warp_70_run(ctx: &Ctx, mut step: ArnWarp70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnWarp70Step::Start => {
                step = ArnWarp70Step::OnOut;
                continue 'machine;
            }
            ArnWarp70Step::OnOut => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from(14),
                        Val::from(91),
                        Val::from(29),
                        Val::from(74),
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

pub fn arn_warp_70(ctx: &Ctx) -> Script {
    arn_warp_70_run(ctx, ArnWarp70Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_warp_70_onout(ctx: &Ctx) -> Script {
    arn_warp_70_run(ctx, ArnWarp70Step::OnOut, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Cast70Step {
    Start,
    OnTimeOver1,
    OnTimeOver2,
    OnNomal1,
    OnNomal2,
}

fn cast_70_run(ctx: &Ctx, mut step: Cast70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Cast70Step::Start => {
                step = Cast70Step::OnTimeOver1;
                continue 'machine;
            }
            Cast70Step::OnTimeOver1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Arena will be reactivated due to an error occurred during battle."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast70Step::OnTimeOver2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Arena will be activated due to an error occurred in the waiting room."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast70Step::OnNomal1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast70Step::OnNomal2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
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

pub fn cast_70(ctx: &Ctx) -> Script {
    cast_70_run(ctx, Cast70Step::Start, Vec::new()).map(|_| ())
}

pub fn cast_70_ontimeover1(ctx: &Ctx) -> Script {
    cast_70_run(ctx, Cast70Step::OnTimeOver1, Vec::new()).map(|_| ())
}

pub fn cast_70_ontimeover2(ctx: &Ctx) -> Script {
    cast_70_run(ctx, Cast70Step::OnTimeOver2, Vec::new()).map(|_| ())
}

pub fn cast_70_onnomal1(ctx: &Ctx) -> Script {
    cast_70_run(ctx, Cast70Step::OnNomal1, Vec::new()).map(|_| ())
}

pub fn cast_70_onnomal2(ctx: &Ctx) -> Script {
    cast_70_run(ctx, Cast70Step::OnNomal2, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Alloff70Step {
    Start,
    OnEnable,
    OnInit,
}

fn alloff_70_run(ctx: &Ctx, mut step: Alloff70Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Alloff70Step::Start => {
                step = Alloff70Step::OnEnable;
                continue 'machine;
            }
            Alloff70Step::OnEnable => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#70::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnTimerOff")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Cadillac#arena")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#70")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arena#70")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_70::OnStop")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Cadillac#arena")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("arena#70")])?;
                return Err(Stop::End);
            }
            Alloff70Step::OnInit => {
                if (!(ctx.var("$top_70min").get()?.is_true()) && !(ctx.var("$top_70sec").get()?.is_true())) {
                    ctx.var("$top_70min").set(Val::from(7))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn alloff_70(ctx: &Ctx) -> Script {
    alloff_70_run(ctx, Alloff70Step::Start, Vec::new()).map(|_| ())
}

pub fn alloff_70_onenable(ctx: &Ctx) -> Script {
    alloff_70_run(ctx, Alloff70Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn alloff_70_oninit(ctx: &Ctx) -> Script {
    alloff_70_run(ctx, Alloff70Step::OnInit, Vec::new()).map(|_| ())
}
