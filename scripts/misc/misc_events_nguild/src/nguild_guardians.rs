#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args};

fn spawn_guardians(ctx: &Ctx, castle: &str, died_event: &str, spots: &[(i32, i32, &str, i32)]) -> Result<(), Stop> {
    for (slot, &(x, y, kind, kind_id)) in spots.iter().enumerate() {
        let slot = slot as i32;
        if ctx.call(Function::GetCastleData, args![castle, 10 + slot])? == 1 {
            ctx.call(Function::Guardian, args![castle, x, y, kind, kind_id, died_event, slot])?;
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum GuardianN01Step {
    Start,
    OnAgitInit,
    OnGuardianDied,
}

fn guardian_n01_run(ctx: &Ctx, mut step: GuardianN01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardianN01Step::Start => {
                step = GuardianN01Step::OnAgitInit;
                continue 'machine;
            }
            GuardianN01Step::OnAgitInit => {
                spawn_guardians(
                    ctx,
                    "nguild_alde",
                    "Guardian_N01::OnGuardianDied",
                    &[
                        (18, 219, "Soldier Guardian", 1287),
                        (117, 42, "Soldier Guardian", 1287),
                        (207, 153, "Soldier Guardian", 1287),
                        (68, 70, "Archer Guardian", 1285),
                        (187, 140, "Archer Guardian", 1285),
                        (62, 204, "Knight Guardian", 1286),
                        (113, 100, "Knight Guardian", 1286),
                        (211, 174, "Knight Guardian", 1286),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardianN01Step::OnGuardianDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_alde", "A Guardian Has Fallen", 17])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn guardian_n01(ctx: &Ctx) -> Script {
    guardian_n01_run(ctx, GuardianN01Step::Start, Vec::new()).map(|_| ())
}

pub fn guardian_n01_onagitinit(ctx: &Ctx) -> Script {
    guardian_n01_run(ctx, GuardianN01Step::OnAgitInit, Vec::new()).map(|_| ())
}

pub fn guardian_n01_onguardiandied(ctx: &Ctx) -> Script {
    guardian_n01_run(ctx, GuardianN01Step::OnGuardianDied, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardianN02Step {
    Start,
    OnAgitInit,
    OnGuardianDied,
}

fn guardian_n02_run(ctx: &Ctx, mut step: GuardianN02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardianN02Step::Start => {
                step = GuardianN02Step::OnAgitInit;
                continue 'machine;
            }
            GuardianN02Step::OnAgitInit => {
                spawn_guardians(
                    ctx,
                    "nguild_gef",
                    "Guardian_N02::OnGuardianDied",
                    &[
                        (30, 178, "Soldier Guardian", 1287),
                        (64, 180, "Soldier Guardian", 1287),
                        (61, 25, "Soldier Guardian", 1287),
                        (61, 44, "Archer Guardian", 1285),
                        (189, 43, "Archer Guardian", 1285),
                        (51, 192, "Knight Guardian", 1286),
                        (49, 67, "Knight Guardian", 1286),
                        (181, 14, "Knight Guardian", 1286),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardianN02Step::OnGuardianDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_gef", "A Guardian Has Fallen", 17])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn guardian_n02(ctx: &Ctx) -> Script {
    guardian_n02_run(ctx, GuardianN02Step::Start, Vec::new()).map(|_| ())
}

pub fn guardian_n02_onagitinit(ctx: &Ctx) -> Script {
    guardian_n02_run(ctx, GuardianN02Step::OnAgitInit, Vec::new()).map(|_| ())
}

pub fn guardian_n02_onguardiandied(ctx: &Ctx) -> Script {
    guardian_n02_run(ctx, GuardianN02Step::OnGuardianDied, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardianN03Step {
    Start,
    OnAgitInit,
    OnGuardianDied,
}

fn guardian_n03_run(ctx: &Ctx, mut step: GuardianN03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardianN03Step::Start => {
                step = GuardianN03Step::OnAgitInit;
                continue 'machine;
            }
            GuardianN03Step::OnAgitInit => {
                spawn_guardians(
                    ctx,
                    "nguild_pay",
                    "Guardian_N03::OnGuardianDied",
                    &[
                        (210, 120, "Soldier Guardian", 1287),
                        (69, 26, "Soldier Guardian", 1287),
                        (23, 141, "Soldier Guardian", 1287),
                        (224, 87, "Archer Guardian", 1285),
                        (81, 45, "Archer Guardian", 1285),
                        (214, 53, "Knight Guardian", 1286),
                        (69, 26, "Knight Guardian", 1286),
                        (23, 141, "Knight Guardian", 1286),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardianN03Step::OnGuardianDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_pay", "A Guardian Has Fallen", 17])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn guardian_n03(ctx: &Ctx) -> Script {
    guardian_n03_run(ctx, GuardianN03Step::Start, Vec::new()).map(|_| ())
}

pub fn guardian_n03_onagitinit(ctx: &Ctx) -> Script {
    guardian_n03_run(ctx, GuardianN03Step::OnAgitInit, Vec::new()).map(|_| ())
}

pub fn guardian_n03_onguardiandied(ctx: &Ctx) -> Script {
    guardian_n03_run(ctx, GuardianN03Step::OnGuardianDied, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GuardianN04Step {
    Start,
    OnAgitInit,
    OnGuardianDied,
}

fn guardian_n04_run(ctx: &Ctx, mut step: GuardianN04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardianN04Step::Start => {
                step = GuardianN04Step::OnAgitInit;
                continue 'machine;
            }
            GuardianN04Step::OnAgitInit => {
                spawn_guardians(
                    ctx,
                    "nguild_prt",
                    "Guardian_N04::OnGuardianDied",
                    &[
                        (196, 92, "Soldier Guardian", 1287),
                        (113, 200, "Soldier Guardian", 1287),
                        (111, 186, "Soldier Guardian", 1287),
                        (76, 202, "Archer Guardian", 1285),
                        (90, 26, "Archer Guardian", 1285),
                        (58, 59, "Knight Guardian", 1286),
                        (112, 200, "Knight Guardian", 1286),
                        (101, 194, "Knight Guardian", 1286),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardianN04Step::OnGuardianDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_prt", "A Guardian Has Fallen", 17])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn guardian_n04(ctx: &Ctx) -> Script {
    guardian_n04_run(ctx, GuardianN04Step::Start, Vec::new()).map(|_| ())
}

pub fn guardian_n04_onagitinit(ctx: &Ctx) -> Script {
    guardian_n04_run(ctx, GuardianN04Step::OnAgitInit, Vec::new()).map(|_| ())
}

pub fn guardian_n04_onguardiandied(ctx: &Ctx) -> Script {
    guardian_n04_run(ctx, GuardianN04Step::OnGuardianDied, Vec::new()).map(|_| ())
}
