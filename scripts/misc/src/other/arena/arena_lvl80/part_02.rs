use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force05mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_05mob_80_run(ctx: &Ctx, mut step: Force05mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mob80Step::Start => {
                step = Force05mob80Step::OnEnable;
                continue 'machine;
            }
            Force05mob80Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(164),
                        Val::from(174),
                        Val::from("Petite"),
                        Val::from(1465),
                        Val::from(1),
                        Val::from("force_05mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(169),
                        Val::from(159),
                        Val::from("Petite"),
                        Val::from(1465),
                        Val::from(1),
                        Val::from("force_05mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(174),
                        Val::from(183),
                        Val::from("Petite"),
                        Val::from(1465),
                        Val::from(1),
                        Val::from("force_05mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(177),
                        Val::from(170),
                        Val::from("Petite"),
                        Val::from(1465),
                        Val::from(1),
                        Val::from("force_05mob#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_05mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_05mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On05_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_05")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#80::OnSummonMob_05")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_80(ctx: &Ctx) -> Script {
    force_05mob_80_run(ctx, Force05mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_80_onenable(ctx: &Ctx) -> Script {
    force_05mob_80_run(ctx, Force05mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_80_onreset(ctx: &Ctx) -> Script {
    force_05mob_80_run(ctx, Force05mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_80_run(ctx, Force05mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06start80Step {
    Start,
    OnEnable,
}

fn force_06start_80_run(ctx: &Ctx, mut step: Force06start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06start80Step::Start => {
                step = Force06start80Step::OnEnable;
                continue 'machine;
            }
            Force06start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_80(ctx: &Ctx) -> Script {
    force_06start_80_run(ctx, Force06start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_80_onenable(ctx: &Ctx) -> Script {
    force_06start_80_run(ctx, Force06start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06ex80Step {
    Start,
    OnEnable,
}

fn force_06ex_80_run(ctx: &Ctx, mut step: Force06ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06ex80Step::Start => {
                step = Force06ex80Step::OnEnable;
                continue 'machine;
            }
            Force06ex80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On06_start")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06ex_80(ctx: &Ctx) -> Script {
    force_06ex_80_run(ctx, Force06ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06ex_80_onenable(ctx: &Ctx) -> Script {
    force_06ex_80_run(ctx, Force06ex80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_06mob_80_run(ctx: &Ctx, mut step: Force06mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mob80Step::Start => {
                step = Force06mob80Step::OnEnable;
                continue 'machine;
            }
            Force06mob80Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(173),
                        Val::from(118),
                        Val::from("Baphomet Jr."),
                        Val::from(1431),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(173),
                        Val::from(90),
                        Val::from("Baphomet Jr."),
                        Val::from(1431),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(177),
                        Val::from(72),
                        Val::from("Baphomet Jr."),
                        Val::from(1431),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(171),
                        Val::from(108),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(171),
                        Val::from(85),
                        Val::from("Deviruchi"),
                        Val::from(1433),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(171),
                        Val::from(79),
                        Val::from("Alice"),
                        Val::from(1521),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(175),
                        Val::from(118),
                        Val::from("Alice"),
                        Val::from(1521),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(175),
                        Val::from(99),
                        Val::from("Alice"),
                        Val::from(1521),
                        Val::from(1),
                        Val::from("force_06mob#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_06mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_06mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On06_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_06")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_80(ctx: &Ctx) -> Script {
    force_06mob_80_run(ctx, Force06mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_80_onenable(ctx: &Ctx) -> Script {
    force_06mob_80_run(ctx, Force06mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06mob_80_onreset(ctx: &Ctx) -> Script {
    force_06mob_80_run(ctx, Force06mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_80_run(ctx, Force06mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07start80Step {
    Start,
    OnEnable,
}

fn force_07start_80_run(ctx: &Ctx, mut step: Force07start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07start80Step::Start => {
                step = Force07start80Step::OnEnable;
                continue 'machine;
            }
            Force07start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07start_80(ctx: &Ctx) -> Script {
    force_07start_80_run(ctx, Force07start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_80_onenable(ctx: &Ctx) -> Script {
    force_07start_80_run(ctx, Force07start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07ex80Step {
    Start,
    OnReset,
    OnSummonMob07,
    OnMyMobDead,
}

fn force_07ex_80_run(ctx: &Ctx, mut step: Force07ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07ex80Step::Start => {
                step = Force07ex80Step::OnReset;
                continue 'machine;
            }
            Force07ex80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_07ex#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07ex80Step::OnSummonMob07 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(160),
                        Val::from(34),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_07ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(163),
                        Val::from(27),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_07ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(163),
                        Val::from(23),
                        Val::from("Merman"),
                        Val::from(1451),
                        Val::from(1),
                        Val::from("force_07ex#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07ex80Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07ex_80(ctx: &Ctx) -> Script {
    force_07ex_80_run(ctx, Force07ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07ex_80_onreset(ctx: &Ctx) -> Script {
    force_07ex_80_run(ctx, Force07ex80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07ex_80_onsummonmob_07(ctx: &Ctx) -> Script {
    force_07ex_80_run(ctx, Force07ex80Step::OnSummonMob07, Vec::new()).map(|_| ())
}

pub fn force_07ex_80_onmymobdead(ctx: &Ctx) -> Script {
    force_07ex_80_run(ctx, Force07ex80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_07mob_80_run(ctx: &Ctx, mut step: Force07mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mob80Step::Start => {
                step = Force07mob80Step::OnEnable;
                continue 'machine;
            }
            Force07mob80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#80::OnSummonMob_07")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(165),
                        Val::from(27),
                        Val::from("Fur-Seal"),
                        Val::from(1533),
                        Val::from(1),
                        Val::from("force_07mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(184),
                        Val::from(23),
                        Val::from("Fur-Seal"),
                        Val::from(1533),
                        Val::from(1),
                        Val::from("force_07mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(174),
                        Val::from(19),
                        Val::from("Fur-Seal"),
                        Val::from(1533),
                        Val::from(1),
                        Val::from("force_07mob#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_07mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_07mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On07_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_07")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_80(ctx: &Ctx) -> Script {
    force_07mob_80_run(ctx, Force07mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_80_onenable(ctx: &Ctx) -> Script {
    force_07mob_80_run(ctx, Force07mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_80_onreset(ctx: &Ctx) -> Script {
    force_07mob_80_run(ctx, Force07mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_80_run(ctx, Force07mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08start80Step {
    Start,
    OnEnable,
}

fn force_08start_80_run(ctx: &Ctx, mut step: Force08start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08start80Step::Start => {
                step = Force08start80Step::OnEnable;
                continue 'machine;
            }
            Force08start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08ex#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_80(ctx: &Ctx) -> Script {
    force_08start_80_run(ctx, Force08start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_80_onenable(ctx: &Ctx) -> Script {
    force_08start_80_run(ctx, Force08start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08ex80Step {
    Start,
    OnEnable,
}

fn force_08ex_80_run(ctx: &Ctx, mut step: Force08ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08ex80Step::Start => {
                step = Force08ex80Step::OnEnable;
                continue 'machine;
            }
            Force08ex80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_08")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08ex_80(ctx: &Ctx) -> Script {
    force_08ex_80_run(ctx, Force08ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_08ex_80_onenable(ctx: &Ctx) -> Script {
    force_08ex_80_run(ctx, Force08ex80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09start80Step {
    Start,
    OnEnable,
}

fn force_09start_80_run(ctx: &Ctx, mut step: Force09start80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09start80Step::Start => {
                step = Force09start80Step::OnEnable;
                continue 'machine;
            }
            Force09start80Step::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#80::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_80(ctx: &Ctx) -> Script {
    force_09start_80_run(ctx, Force09start80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_80_onenable(ctx: &Ctx) -> Script {
    force_09start_80_run(ctx, Force09start80Step::OnEnable, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09ex80Step {
    Start,
    OnReset,
    OnSummonMob09,
    OnMyMobDead,
}

fn force_09ex_80_run(ctx: &Ctx, mut step: Force09ex80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09ex80Step::Start => {
                step = Force09ex80Step::OnReset;
                continue 'machine;
            }
            Force09ex80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_09ex#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09ex80Step::OnSummonMob09 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(91),
                        Val::from(99),
                        Val::from("Argos"),
                        Val::from(1430),
                        Val::from(1),
                        Val::from("force_09ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(95),
                        Val::from(105),
                        Val::from("Argos"),
                        Val::from(1430),
                        Val::from(1),
                        Val::from("force_09ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(122),
                        Val::from(99),
                        Val::from("Argos"),
                        Val::from(1430),
                        Val::from(1),
                        Val::from("force_09ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(88),
                        Val::from(107),
                        Val::from("Argos"),
                        Val::from(1430),
                        Val::from(1),
                        Val::from("force_09ex#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(85),
                        Val::from(104),
                        Val::from("Argos"),
                        Val::from(1430),
                        Val::from(1),
                        Val::from("force_09ex#80::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09ex80Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09ex_80(ctx: &Ctx) -> Script {
    force_09ex_80_run(ctx, Force09ex80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09ex_80_onreset(ctx: &Ctx) -> Script {
    force_09ex_80_run(ctx, Force09ex80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09ex_80_onsummonmob_09(ctx: &Ctx) -> Script {
    force_09ex_80_run(ctx, Force09ex80Step::OnSummonMob09, Vec::new()).map(|_| ())
}

pub fn force_09ex_80_onmymobdead(ctx: &Ctx) -> Script {
    force_09ex_80_run(ctx, Force09ex80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09mob80Step {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_80_run(ctx: &Ctx, mut step: Force09mob80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mob80Step::Start => {
                step = Force09mob80Step::OnEnable;
                continue 'machine;
            }
            Force09mob80Step::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_4-1"),
                        Val::from(99),
                        Val::from(99),
                        Val::from("Ancient Mummy"),
                        Val::from(1522),
                        Val::from(1),
                        Val::from("force_09mob#80::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#80::OnSummonMob_09")])?;
                return Err(Stop::End);
            }
            Force09mob80Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_4-1"), Val::from("force_09mob#80::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mob80Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_4-1"), Val::from("force_09mob#80::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::On09_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_09")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_All")])?;
                    ctx.var("$arena_min80end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                    ctx.var("$arena_sec80end")
                        .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_80(ctx: &Ctx) -> Script {
    force_09mob_80_run(ctx, Force09mob80Step::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_80_onenable(ctx: &Ctx) -> Script {
    force_09mob_80_run(ctx, Force09mob80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_09mob_80_onreset(ctx: &Ctx) -> Script {
    force_09mob_80_run(ctx, Force09mob80Step::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_80_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_80_run(ctx, Force09mob80Step::OnMyMobDead, Vec::new()).map(|_| ())
}

fn staff_80_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
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

pub fn staff_80_1(ctx: &Ctx) -> Script {
    staff_80_1_body(ctx, Vec::new()).map(|_| ())
}

fn staff_80_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if runtime::op(&ctx.var("$arena_min80end").get()?, "<", &ctx.var("$arena_min80st").get()?)?.is_true() {
        if runtime::op(&ctx.var("$arena_sec80end").get()?, "<", &ctx.var("$arena_sec80st").get()?)?.is_true() {
            ctx.var("@record_min80").set(
                (((Val::from(60).try_sub(ctx.var("$arena_min80st").get()?)?) + ctx.var("$arena_min80end").get()?).try_sub(Val::from(1))?),
            )?;
            ctx.var("@record_sec80")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec80st").get()?)?) + ctx.var("$arena_sec80end").get()?))?;
        } else {
            ctx.var("@record_min80")
                .set(((Val::from(60).try_sub(ctx.var("$arena_min80st").get()?)?) + ctx.var("$arena_min80end").get()?))?;
            ctx.var("@record_sec80")
                .set((ctx.var("$arena_sec80end").get()?.try_sub(ctx.var("$arena_sec80st").get()?)?))?;
        }
    } else {
        if runtime::op(&ctx.var("$arena_sec80end").get()?, "<", &ctx.var("$arena_sec80st").get()?)?.is_true() {
            ctx.var("@record_min80")
                .set(((ctx.var("$arena_min80end").get()?.try_sub(ctx.var("$arena_min80st").get()?)?).try_sub(Val::from(1))?))?;
            ctx.var("@record_sec80")
                .set(((Val::from(60).try_sub(ctx.var("$arena_sec80st").get()?)?) + ctx.var("$arena_sec80end").get()?))?;
        } else {
            ctx.var("@record_min80")
                .set((ctx.var("$arena_min80end").get()?.try_sub(ctx.var("$arena_min80st").get()?)?))?;
            ctx.var("@record_sec80")
                .set((ctx.var("$arena_sec80end").get()?.try_sub(ctx.var("$arena_sec80st").get()?)?))?;
        }
    }
    ctx.var("@gap80").set(
        (((Val::from(60).try_mul(ctx.var("$top_80min").get()?)?) + ctx.var("$top_80sec").get()?)
            .try_sub(((Val::from(60).try_mul(ctx.var("@record_min80").get()?)?) + ctx.var("@record_sec80").get()?))?),
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
            ((((Val::from("is ") + ctx.var("@record_min80").get()?) + Val::from("minutes ")) + ctx.var("@record_sec80").get()?)
                + Val::from("seconds.")),
            "Congratulations!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((Val::from("The fastest player among people who cleared Lv80 arena time force battle is ^3131FF")
                + ctx.var("$arena_80topn$").get()?)
                + Val::from("^000000."))
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Staff",
        args![
            ((((((Val::from("^3131FF") + ctx.var("$arena_80topn$").get()?) + Val::from("^000000's running time was ^3131FF"))
                + ctx.var("$top_80min").get()?)
                + Val::from("^000000minutes ^3131FF"))
                + ctx.var("$top_80sec").get()?)
                + Val::from("^000000seconds."))
        ],
    )?;
    ctx.next()?;
    if ctx.var("@gap80").get()?.number()? < 0 {
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
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#80::OnNomal1")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_80::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#80::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv80 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    } else {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
        ctx.lines_as("Staff", args!["Wow! You have renewed the record!", "What a great job!"])?;
        ctx.next()?;
        ctx.lines_as("Staff", args![((Val::from("You have been recorded as the fastest player among people who cleared ^FF0000Arena Time Force Battle lvl 80s^000000, ^3131FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000."))])?;
        ctx.var("$top_80min").set(ctx.var("@record_min80").get()?)?;
        ctx.var("$top_80sec").set(ctx.var("@record_sec80").get()?)?;
        ctx.var("$arena_80topn$")
            .set(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Vendigos::OnLineRec_80")])?;
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
        ctx.call(Function::DoNpcEvent, vec![Val::from("cast#80::OnNomal2")])?;
        ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(100), Val::from(75)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_80::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#80::OnEnable")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Lv80 Waiting Room::OnStart")])?;
        return Err(Stop::End);
    }
}

pub fn staff_80_2(ctx: &Ctx) -> Script {
    staff_80_2_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnTimer80Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer60000,
    OnStop,
}

fn arn_timer_80_run(ctx: &Ctx, mut step: ArnTimer80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnTimer80Step::Start => {
                step = ArnTimer80Step::OnEnable;
                continue 'machine;
            }
            ArnTimer80Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ArnTimer80Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from("This broadcast informs you about the restriction for arena lvl 80s."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            ArnTimer80Step::OnTimer3000 => {
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
            ArnTimer80Step::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("prt_are_in"), Val::from("Please proceed your battle quickly as possible in order to avoid disadvantage. Thank you for your cooperation."), Val::from(0), Val::from(16764416)])?;
                return Err(Stop::End);
            }
            ArnTimer80Step::OnTimer60000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#80::OnTimeOver2")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arn_warp_80::OnOut")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_80::OnStop")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#80::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv80 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            ArnTimer80Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arn_timer_80(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_timer_80_onenable(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn arn_timer_80_ontimer2000(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn arn_timer_80_ontimer3000(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn arn_timer_80_ontimer4000(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn arn_timer_80_ontimer60000(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn arn_timer_80_onstop(ctx: &Ctx) -> Script {
    arn_timer_80_run(ctx, ArnTimer80Step::OnStop, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArnWarp80Step {
    Start,
    OnOut,
}

fn arn_warp_80_run(ctx: &Ctx, mut step: ArnWarp80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArnWarp80Step::Start => {
                step = ArnWarp80Step::OnOut;
                continue 'machine;
            }
            ArnWarp80Step::OnOut => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("prt_are_in"),
                        Val::from(66),
                        Val::from(195),
                        Val::from(81),
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

pub fn arn_warp_80(ctx: &Ctx) -> Script {
    arn_warp_80_run(ctx, ArnWarp80Step::Start, Vec::new()).map(|_| ())
}

pub fn arn_warp_80_onout(ctx: &Ctx) -> Script {
    arn_warp_80_run(ctx, ArnWarp80Step::OnOut, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Cast80Step {
    Start,
    OnTimeOver1,
    OnTimeOver2,
    OnNomal1,
    OnNomal2,
}

fn cast_80_run(ctx: &Ctx, mut step: Cast80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Cast80Step::Start => {
                step = Cast80Step::OnTimeOver1;
                continue 'machine;
            }
            Cast80Step::OnTimeOver1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Arena will be reactivated due to an error occurred during battle."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast80Step::OnTimeOver2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Arena will be activated due to an error occurred in the waiting room."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast80Step::OnNomal1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Arena will be reactivated."),
                        Val::from(0),
                        Val::from(16764416),
                    ],
                )?;
                return Err(Stop::End);
            }
            Cast80Step::OnNomal2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
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

pub fn cast_80(ctx: &Ctx) -> Script {
    cast_80_run(ctx, Cast80Step::Start, Vec::new()).map(|_| ())
}

pub fn cast_80_ontimeover1(ctx: &Ctx) -> Script {
    cast_80_run(ctx, Cast80Step::OnTimeOver1, Vec::new()).map(|_| ())
}

pub fn cast_80_ontimeover2(ctx: &Ctx) -> Script {
    cast_80_run(ctx, Cast80Step::OnTimeOver2, Vec::new()).map(|_| ())
}

pub fn cast_80_onnomal1(ctx: &Ctx) -> Script {
    cast_80_run(ctx, Cast80Step::OnNomal1, Vec::new()).map(|_| ())
}

pub fn cast_80_onnomal2(ctx: &Ctx) -> Script {
    cast_80_run(ctx, Cast80Step::OnNomal2, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Alloff80Step {
    Start,
    OnEnable,
    OnInit,
}

fn alloff_80_run(ctx: &Ctx, mut step: Alloff80Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Alloff80Step::Start => {
                step = Alloff80Step::OnEnable;
                continue 'machine;
            }
            Alloff80Step::OnEnable => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_04ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09ex#80::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnTimerOff")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Octus#arena")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01_02#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02_03#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03_04#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04_05#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05_06#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06_07#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07_08#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08_09#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_exit#80")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("arena#80")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#arn_timer_80::OnStop")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Octus#arena")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("arena#80")])?;
                return Err(Stop::End);
            }
            Alloff80Step::OnInit => {
                if (!(ctx.var("$top_80min").get()?.is_true()) && !(ctx.var("$top_80sec").get()?.is_true())) {
                    ctx.var("$top_80min").set(Val::from(8))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn alloff_80(ctx: &Ctx) -> Script {
    alloff_80_run(ctx, Alloff80Step::Start, Vec::new()).map(|_| ())
}

pub fn alloff_80_onenable(ctx: &Ctx) -> Script {
    alloff_80_run(ctx, Alloff80Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn alloff_80_oninit(ctx: &Ctx) -> Script {
    alloff_80_run(ctx, Alloff80Step::OnInit, Vec::new()).map(|_| ())
}
