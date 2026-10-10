use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum Start01GnbsStep {
    Start,
    OnInit,
    OnCommandOn,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer11Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer1_1_run(ctx: &Ctx, mut step: Timer11Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer11Step::Start => {
                step = Timer11Step::OnInit;
                continue 'machine;
            }
            Timer11Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-1")])?;
                return Err(Stop::End);
            }
            Timer11Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer1-1")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer11Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-1")])?;
                return Err(Stop::End);
            }
            Timer11Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem1-1::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer12Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer1_2_run(ctx: &Ctx, mut step: Timer12Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer12Step::Start => {
                step = Timer12Step::OnInit;
                continue 'machine;
            }
            Timer12Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-2")])?;
                return Err(Stop::End);
            }
            Timer12Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer1-2")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer12Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-2")])?;
                return Err(Stop::End);
            }
            Timer12Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem1-2::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer13Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer1_3_run(ctx: &Ctx, mut step: Timer13Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer13Step::Start => {
                step = Timer13Step::OnInit;
                continue 'machine;
            }
            Timer13Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-3")])?;
                return Err(Stop::End);
            }
            Timer13Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer1-3")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer13Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-3")])?;
                return Err(Stop::End);
            }
            Timer13Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem1-3::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer14Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer1_4_run(ctx: &Ctx, mut step: Timer14Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer14Step::Start => {
                step = Timer14Step::OnInit;
                continue 'machine;
            }
            Timer14Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-4")])?;
                return Err(Stop::End);
            }
            Timer14Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer1-4")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer14Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-4")])?;
                return Err(Stop::End);
            }
            Timer14Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem1-4::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer15Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer1_5_run(ctx: &Ctx, mut step: Timer15Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer15Step::Start => {
                step = Timer15Step::OnInit;
                continue 'machine;
            }
            Timer15Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-5")])?;
                return Err(Stop::End);
            }
            Timer15Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer1-5")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer15Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer1-5")])?;
                return Err(Stop::End);
            }
            Timer15Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem1-5::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Start02GnbsStep {
    Start,
    OnInit,
    OnCommandOn,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer21Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer2_1_run(ctx: &Ctx, mut step: Timer21Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer21Step::Start => {
                step = Timer21Step::OnInit;
                continue 'machine;
            }
            Timer21Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-1")])?;
                return Err(Stop::End);
            }
            Timer21Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer2-1")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer21Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-1")])?;
                return Err(Stop::End);
            }
            Timer21Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem2-1::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer22Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer2_2_run(ctx: &Ctx, mut step: Timer22Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer22Step::Start => {
                step = Timer22Step::OnInit;
                continue 'machine;
            }
            Timer22Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-2")])?;
                return Err(Stop::End);
            }
            Timer22Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer2-2")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer22Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-2")])?;
                return Err(Stop::End);
            }
            Timer22Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem2-2::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer23Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer2_3_run(ctx: &Ctx, mut step: Timer23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer23Step::Start => {
                step = Timer23Step::OnInit;
                continue 'machine;
            }
            Timer23Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-3")])?;
                return Err(Stop::End);
            }
            Timer23Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer2-3")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer23Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-3")])?;
                return Err(Stop::End);
            }
            Timer23Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem2-3::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer24Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer2_4_run(ctx: &Ctx, mut step: Timer24Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer24Step::Start => {
                step = Timer24Step::OnInit;
                continue 'machine;
            }
            Timer24Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-4")])?;
                return Err(Stop::End);
            }
            Timer24Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer2-4")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer24Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-4")])?;
                return Err(Stop::End);
            }
            Timer24Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem2-4::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer25Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer2_5_run(ctx: &Ctx, mut step: Timer25Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer25Step::Start => {
                step = Timer25Step::OnInit;
                continue 'machine;
            }
            Timer25Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-5")])?;
                return Err(Stop::End);
            }
            Timer25Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer2-5")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer25Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-5")])?;
                return Err(Stop::End);
            }
            Timer25Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem2-5::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer26Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer2_6_run(ctx: &Ctx, mut step: Timer26Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer26Step::Start => {
                step = Timer26Step::OnInit;
                continue 'machine;
            }
            Timer26Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-6")])?;
                return Err(Stop::End);
            }
            Timer26Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer2-6")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer26Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer2-6")])?;
                return Err(Stop::End);
            }
            Timer26Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem2-6::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Start03GnbsStep {
    Start,
    OnInit,
    OnCommandOn,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer31Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_1_run(ctx: &Ctx, mut step: Timer31Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer31Step::Start => {
                step = Timer31Step::OnInit;
                continue 'machine;
            }
            Timer31Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-1")])?;
                return Err(Stop::End);
            }
            Timer31Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-1")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer31Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-1")])?;
                return Err(Stop::End);
            }
            Timer31Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-1::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer32Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_2_run(ctx: &Ctx, mut step: Timer32Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer32Step::Start => {
                step = Timer32Step::OnInit;
                continue 'machine;
            }
            Timer32Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-2")])?;
                return Err(Stop::End);
            }
            Timer32Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-2")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer32Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-2")])?;
                return Err(Stop::End);
            }
            Timer32Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-2::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer33Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_3_run(ctx: &Ctx, mut step: Timer33Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer33Step::Start => {
                step = Timer33Step::OnInit;
                continue 'machine;
            }
            Timer33Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-3")])?;
                return Err(Stop::End);
            }
            Timer33Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-3")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer33Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-3")])?;
                return Err(Stop::End);
            }
            Timer33Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-3::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer34Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_4_run(ctx: &Ctx, mut step: Timer34Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer34Step::Start => {
                step = Timer34Step::OnInit;
                continue 'machine;
            }
            Timer34Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-4")])?;
                return Err(Stop::End);
            }
            Timer34Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-4")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer34Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-4")])?;
                return Err(Stop::End);
            }
            Timer34Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-4::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer35Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_5_run(ctx: &Ctx, mut step: Timer35Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer35Step::Start => {
                step = Timer35Step::OnInit;
                continue 'machine;
            }
            Timer35Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-5")])?;
                return Err(Stop::End);
            }
            Timer35Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-5")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer35Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-5")])?;
                return Err(Stop::End);
            }
            Timer35Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-5::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer36Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_6_run(ctx: &Ctx, mut step: Timer36Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer36Step::Start => {
                step = Timer36Step::OnInit;
                continue 'machine;
            }
            Timer36Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-6")])?;
                return Err(Stop::End);
            }
            Timer36Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-6")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer36Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-6")])?;
                return Err(Stop::End);
            }
            Timer36Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-6::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timer37Step {
    Start,
    OnInit,
    OnCommandOn,
    OnCommandOff,
    OnTimer3000,
}

pub(super) fn timer3_7_run(ctx: &Ctx, mut step: Timer37Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timer37Step::Start => {
                step = Timer37Step::OnInit;
                continue 'machine;
            }
            Timer37Step::OnInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-7")])?;
                return Err(Stop::End);
            }
            Timer37Step::OnCommandOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("timer3-7")])?;
                ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
                ctx.call(Function::StartNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timer37Step::OnCommandOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("timer3-7")])?;
                return Err(Stop::End);
            }
            Timer37Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#getitem3-7::OnCommandOn")])?;
                return Err(Stop::End);
            }
        }
    }
}
