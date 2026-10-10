use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MrKiddMocExtra01Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

fn mr_kidd_moc_extra01_run(ctx: &Ctx, mut step: MrKiddMocExtra01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MrKiddMocExtra01Step::Start => {
                return Err(Stop::End);
            }
            MrKiddMocExtra01Step::OnInit => {
                step = MrKiddMocExtra01Step::OnDisable;
                continue 'machine;
            }
            MrKiddMocExtra01Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mr. Kidd#moc_extra01")])?;
                return Err(Stop::End);
            }
            MrKiddMocExtra01Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Mr. Kidd#moc_extra01")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mr_kidd_moc_extra01(ctx: &Ctx) -> Script {
    mr_kidd_moc_extra01_run(ctx, MrKiddMocExtra01Step::Start, Vec::new()).map(|_| ())
}

pub fn mr_kidd_moc_extra01_oninit(ctx: &Ctx) -> Script {
    mr_kidd_moc_extra01_run(ctx, MrKiddMocExtra01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn mr_kidd_moc_extra01_ondisable(ctx: &Ctx) -> Script {
    mr_kidd_moc_extra01_run(ctx, MrKiddMocExtra01Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mr_kidd_moc_extra01_onenable(ctx: &Ctx) -> Script {
    mr_kidd_moc_extra01_run(ctx, MrKiddMocExtra01Step::OnEnable, Vec::new()).map(|_| ())
}

fn dismembered_corpse_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "You find a horribly dismembered dead body.",
        "It is disgusting to decribe the status of the Corpse!."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn dismembered_corpse_moc2(ctx: &Ctx) -> Script {
    dismembered_corpse_moc2_body(ctx, Vec::new()).map(|_| ())
}

fn crushed_corpse_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "You find a horribly disfigured corpse.",
        "that appears to have been crushed to death."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn crushed_corpse_moc2(ctx: &Ctx) -> Script {
    crushed_corpse_moc2_body(ctx, Vec::new()).map(|_| ())
}

fn mutated_corpse_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["You find a terribly mutated corpse.", "R.I.P."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mutated_corpse_moc2(ctx: &Ctx) -> Script {
    mutated_corpse_moc2_body(ctx, Vec::new()).map(|_| ())
}

fn mutilated_corpse_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["You find a terribly mutilated corpse.", "Rest in peace."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mutilated_corpse_moc2(ctx: &Ctx) -> Script {
    mutilated_corpse_moc2_body(ctx, Vec::new()).map(|_| ())
}

fn disfigured_corpse_moc2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args!["You find a horribly disfigured corpse.", "... . . . . . !!!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn disfigured_corpse_moc2(ctx: &Ctx) -> Script {
    disfigured_corpse_moc2_body(ctx, Vec::new()).map(|_| ())
}
