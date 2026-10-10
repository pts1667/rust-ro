use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum TestHelperSgStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer2000,
    OnTimer4000,
    OnTimer5000,
    OnTimer7000,
    OnTimer9000,
}

fn test_helper_sg_run(ctx: &Ctx, mut step: TestHelperSgStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TestHelperSgStep::Start => {
                step = TestHelperSgStep::OnInit;
                continue 'machine;
            }
            TestHelperSgStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Test Helper#sg")])?;
                return Err(Stop::End);
            }
            TestHelperSgStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TestHelperSgStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Please go back to where you came from and finish the rest of your job change quest."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TestHelperSgStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("This is the end of practical examination. Next candidate, please get ready."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TestHelperSgStep::OnTimer5000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_sage"),
                        Val::from(100),
                        Val::from(82),
                        Val::from(131),
                        Val::from(113),
                        Val::from("yuno_in03"),
                        Val::from(163),
                        Val::from(180),
                    ],
                )?;
                return Err(Stop::End);
            }
            TestHelperSgStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_sage"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TestHelperSgStep::OnTimer9000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Test Helper#sg")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#sg::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn test_helper_sg(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::Start, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_oninit(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnInit, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_onenable(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_ontimer2000(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_ontimer4000(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_ontimer5000(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_ontimer7000(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnTimer7000, Vec::new()).map(|_| ())
}

pub fn test_helper_sg_ontimer9000(ctx: &Ctx) -> Script {
    test_helper_sg_run(ctx, TestHelperSgStep::OnTimer9000, Vec::new()).map(|_| ())
}
