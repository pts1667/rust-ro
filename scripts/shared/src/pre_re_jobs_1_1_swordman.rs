#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn f_jobswdmedic(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::PercentHeal, args![100, 0])?;
    ctx.lines_as(
        "Medic",
        args![Val::from("This is the ") + runtime::arg(&args, 0, Val::from(0)) + Val::from(" check point! Cheer up!")],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn f_jobswdstaff(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Test Hall Staff", args!["Do you surrender?"])?;
    ctx.next()?;
    if ctx.menu(&["Yes.", "No."])? == 0 {
        ctx.call(
            Function::MapAnnounce,
            args![
                "job_sword1",
                Val::from("Applicant ") + ctx.player().name()? + Val::from(" quit the test."),
                constants::BC_MAP,
            ],
        )?;
        ctx.warp("izlude_in", 65, 165)?;
        return Err(Stop::End);
    }
    ctx.lines_as("Test Hall Staff", args!["Bravo! Go for it again!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

#[derive(Clone, Copy, Debug)]
enum FJobswdteststaffStep {
    Start,
    OnTouch,
}

fn f_jobswdteststaff_run(ctx: &Ctx, mut step: FJobswdteststaffStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FJobswdteststaffStep::Start => {
                step = FJobswdteststaffStep::OnTouch;
                continue 'machine;
            }
            FJobswdteststaffStep::OnTouch => {
                ctx.lines_as(
                    "Test Hall Staff",
                    args![Val::from("Applicant ") + ctx.player().name()? + Val::from(". Do you surrender??")],
                )?;
                ctx.next()?;
                if ctx.menu(&["Yes.", "No."])? == 0 {
                    ctx.call(
                        Function::MapAnnounce,
                        args![
                            "job_sword1",
                            Val::from("Applicant ") + ctx.player().name()? + Val::from(" quit the test.."),
                            constants::BC_MAP,
                        ],
                    )?;
                    ctx.warp("izlude_in", 65, 165)?;
                    return Err(Stop::End);
                }
                ctx.call(
                    Function::Warp,
                    args![
                        "job_sword1",
                        runtime::arg(&args, 0, Val::from(0)),
                        runtime::arg(&args, 1, Val::from(0))
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn f_jobswdteststaff(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_jobswdteststaff_run(ctx, FJobswdteststaffStep::Start, args)
}

#[derive(Clone, Copy, Debug)]
enum FJobswdteststaff2Step {
    Start,
    OnTouch,
}

fn f_jobswdteststaff2_run(ctx: &Ctx, mut step: FJobswdteststaff2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FJobswdteststaff2Step::Start => {
                step = FJobswdteststaff2Step::OnTouch;
                continue 'machine;
            }
            FJobswdteststaff2Step::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "job_sword1",
                        Val::from("Applicant ")
                            + ctx.player().name()?
                            + Val::from(". Pass the ")
                            + runtime::arg(&args, 0, Val::from(0))
                            + Val::from(" course."),
                        constants::BC_MAP,
                    ],
                )?;
                ctx.call(
                    Function::Warp,
                    args![
                        "job_sword1",
                        runtime::arg(&args, 1, Val::from(0)),
                        runtime::arg(&args, 2, Val::from(0))
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn f_jobswdteststaff2(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_jobswdteststaff2_run(ctx, FJobswdteststaff2Step::Start, args)
}
