use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn timeout_ufe_ontimer135000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer135000, Vec::new()).map(|_| ())
}

pub fn timeout_ufe_ontimer142000(ctx: &Ctx) -> Script {
    timeout_ufe_run(ctx, TimeoutUfeStep::OnTimer142000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Guard1UfeStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnTimer12000,
    OnTimer30000,
    OnMyMobDead,
}

fn guard_1_ufe_run(ctx: &Ctx, mut step: Guard1UfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guard1UfeStep::Start => {
                step = Guard1UfeStep::OnEnable;
                continue 'machine;
            }
            Guard1UfeStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard1UfeStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard1UfeStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("I admire your patience."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard1UfeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Let's see if everything you have experienced"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard1UfeStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("were traps for intruders..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard1UfeStep::OnTimer12000 => {
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-1#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.var("$@jupeelevatorinuse2").set(Val::from(1))?;
                return Err(Stop::End);
            }
            Guard1UfeStep::OnTimer30000 => {
                if ctx.var("$@jupeelevatorinuse2").get()? == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-2#ufe::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Guard1UfeStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    if ctx.var("$@jupeelevatorinuse2").get()? == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-2#ufe::OnEnable")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                    } else if (((ctx.var("$@jupeelevatorinuse2").get()? == 4
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-2#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-3#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-4#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                    {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("GuardEnd#ufe::OnEnable")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn guard_1_ufe(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::Start, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_onenable(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_ondisable(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_ontimer2000(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_ontimer5000(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_ontimer8000(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_ontimer12000(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_ontimer30000(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn guard_1_ufe_onmymobdead(ctx: &Ctx) -> Script {
    guard_1_ufe_run(ctx, Guard1UfeStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Guard2UfeStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnTimer11000,
    OnTimer12000,
    OnTimer30000,
    OnMyMobDead,
}

fn guard_2_ufe_run(ctx: &Ctx, mut step: Guard2UfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guard2UfeStep::Start => {
                step = Guard2UfeStep::OnEnable;
                continue 'machine;
            }
            Guard2UfeStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("This city was not"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("as magnificient as you thought."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("This is a place where all the fears of humans flourish."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Yes. Nobody leaves alive!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnTimer12000 => {
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-2#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.var("$@jupeelevatorinuse2").set(Val::from(2))?;
                return Err(Stop::End);
            }
            Guard2UfeStep::OnTimer30000 => {
                if ctx.var("$@jupeelevatorinuse2").get()? == 2 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-3#ufe::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Guard2UfeStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    if ctx.var("$@jupeelevatorinuse2").get()? == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-3#ufe::OnEnable")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                    } else if (((ctx.var("$@jupeelevatorinuse2").get()? == 4
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-1#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-3#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-4#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                    {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("GuardEnd#ufe::OnEnable")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn guard_2_ufe(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::Start, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_onenable(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ondisable(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ontimer2000(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ontimer5000(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ontimer8000(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ontimer11000(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ontimer12000(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_ontimer30000(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn guard_2_ufe_onmymobdead(ctx: &Ctx) -> Script {
    guard_2_ufe_run(ctx, Guard2UfeStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Guard3UfeStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnTimer12000,
    OnTimer30000,
    OnMyMobDead,
}

fn guard_3_ufe_run(ctx: &Ctx, mut step: Guard3UfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guard3UfeStep::Start => {
                step = Guard3UfeStep::OnEnable;
                continue 'machine;
            }
            Guard3UfeStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard3UfeStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard3UfeStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("What do you see?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard3UfeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Are your eyes actually seeing something?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard3UfeStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Or do you just believe you are seeing?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard3UfeStep::OnTimer12000 => {
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-3#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.var("$@jupeelevatorinuse2").set(Val::from(3))?;
                return Err(Stop::End);
            }
            Guard3UfeStep::OnTimer30000 => {
                if ctx.var("$@jupeelevatorinuse2").get()? == 3 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-4#ufe::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Guard3UfeStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    if ctx.var("$@jupeelevatorinuse2").get()? == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-4#ufe::OnEnable")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                    } else if (((ctx.var("$@jupeelevatorinuse2").get()? == 4
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-1#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-2#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-4#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                    {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("GuardEnd#ufe::OnEnable")])?;
                        ctx.call(Function::StopNpcTimer, vec![])?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn guard_3_ufe(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::Start, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_onenable(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_ondisable(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_ontimer2000(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_ontimer5000(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_ontimer8000(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_ontimer12000(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_ontimer30000(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn guard_3_ufe_onmymobdead(ctx: &Ctx) -> Script {
    guard_3_ufe_run(ctx, Guard3UfeStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Guard4UfeStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnMyMobDead,
}

fn guard_4_ufe_run(ctx: &Ctx, mut step: Guard4UfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Guard4UfeStep::Start => {
                step = Guard4UfeStep::OnEnable;
                continue 'machine;
            }
            Guard4UfeStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard4UfeStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Guard4UfeStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Do not forget. That which limits you is nothing but yourself."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard4UfeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Nothing is what you fear and you have nothing to fear..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Guard4UfeStep::OnTimer8000 => {
                ctx.var(".mymobs").set(Val::from(8))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1684),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(42),
                        Val::from("Guard"),
                        Val::from(1684),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(44),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(35),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(38),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(45),
                        Val::from(53),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from(48),
                        Val::from(51),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Guard-4#ufe::OnMyMobDead"),
                    ],
                )?;
                ctx.var("$@jupeelevatorinuse2").set(Val::from(4))?;
                return Err(Stop::End);
            }
            Guard4UfeStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1
                    && ((ctx
                        .call(
                            Function::GetVariableOfNpc,
                            vec![Val::from(".mymobs"), Val::from("Guard-1#ufe"), Val::from(0)],
                        )?
                        .number()?
                        < 1
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-2#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                        && ctx
                            .call(
                                Function::GetVariableOfNpc,
                                vec![Val::from(".mymobs"), Val::from("Guard-3#ufe"), Val::from(0)],
                            )?
                            .number()?
                            < 1)
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("GuardEnd#ufe::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn guard_4_ufe(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::Start, Vec::new()).map(|_| ())
}

pub fn guard_4_ufe_onenable(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn guard_4_ufe_ondisable(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn guard_4_ufe_ontimer2000(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn guard_4_ufe_ontimer5000(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn guard_4_ufe_ontimer8000(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn guard_4_ufe_onmymobdead(ctx: &Ctx) -> Script {
    guard_4_ufe_run(ctx, Guard4UfeStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn guardend_ufe(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_oninit(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_onenable(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ondisable(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer2000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer5000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer5000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer8000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer8000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer11000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer11000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer12000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer12000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer22000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer22000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer24000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer24000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer25000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn guardend_ufe_ontimer26000(ctx: &Ctx) -> Script {
    guardend_ufe_run(ctx, GuardendUfeStep::OnTimer26000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum S4fEnterUfeStep {
    Start,
    OnInit,
    OnTouch,
}

fn s_4f_enter_ufe_run(ctx: &Ctx, mut step: S4fEnterUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            S4fEnterUfeStep::Start => {
                step = S4fEnterUfeStep::OnInit;
                continue 'machine;
            }
            S4fEnterUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("4F Enter#ufe")])?;
                return Err(Stop::End);
            }
            S4fEnterUfeStep::OnTouch => {
                ctx.call(
                    Function::Warp,
                    vec![
                        Val::from("jupe_core"),
                        ctx.call(Function::Rand, vec![Val::from(149), Val::from(151)])?,
                        Val::from(286),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn s_4f_enter_ufe(ctx: &Ctx) -> Script {
    s_4f_enter_ufe_run(ctx, S4fEnterUfeStep::Start, Vec::new()).map(|_| ())
}

pub fn s_4f_enter_ufe_oninit(ctx: &Ctx) -> Script {
    s_4f_enter_ufe_run(ctx, S4fEnterUfeStep::OnInit, Vec::new()).map(|_| ())
}

pub fn s_4f_enter_ufe_ontouch(ctx: &Ctx) -> Script {
    s_4f_enter_ufe_run(ctx, S4fEnterUfeStep::OnTouch, Vec::new()).map(|_| ())
}

fn gate_start_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gate_start(ctx: &Ctx) -> Script {
    gate_start_body(ctx, Vec::new()).map(|_| ())
}

fn gate_start_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![Val::from("gate#start"), Val::from(1)])?;
    ctx.lines(args![
        "^3355FFIt's a Warp Portal",
        "that will teleport you",
        "to the previous floor.^000000"
    ])?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Use it.:Ignore it.")])? {
        1 => {
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_LIGHTSPHERE")?])?;
            ctx.close_window()?;
            ctx.call(Function::StopNpcTimer, vec![])?;
            ctx.call(Function::Warp, vec![Val::from("juperos_02"), Val::from(130), Val::from(142)])?;
        }
        2 => {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Not now!", "I can't leave yet!"],
            )?;
            ctx.close_window()?;
            ctx.call(Function::StopNpcTimer, vec![])?;
            ctx.call(Function::Warp, vec![Val::from("jupe_gate"), Val::from(50), Val::from(168)])?;
        }
        _ => {}
    }
    return Err(Stop::End);
}

pub fn gate_start_ontouch(ctx: &Ctx) -> Script {
    gate_start_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn gate_start_ontimer10000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::Warp, vec![Val::from("juperos_02"), Val::from(128), Val::from(278)])?;
    ctx.call(Function::EnableNpc, vec![Val::from("gate#start#2")])?;
    ctx.call(Function::DisableNpc, vec![Val::from("gate#start")])?;
    return Err(Stop::End);
}

pub fn gate_start_ontimer10000(ctx: &Ctx) -> Script {
    gate_start_ontimer10000_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GateStart2Step {
    Start,
    OnInit,
    OnTouch,
    OnTimer2000,
}

fn gate_start_2_run(ctx: &Ctx, mut step: GateStart2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GateStart2Step::Start => {
                step = GateStart2Step::OnInit;
                continue 'machine;
            }
            GateStart2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("gate#start#2")])?;
                return Err(Stop::End);
            }
            GateStart2Step::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("juperos_02"), Val::from(130), Val::from(142)])?;
                return Err(Stop::End);
            }
            GateStart2Step::OnTimer2000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("gate#start")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("gate#start#2")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn gate_start_2(ctx: &Ctx) -> Script {
    gate_start_2_run(ctx, GateStart2Step::Start, Vec::new()).map(|_| ())
}

pub fn gate_start_2_oninit(ctx: &Ctx) -> Script {
    gate_start_2_run(ctx, GateStart2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn gate_start_2_ontouch(ctx: &Ctx) -> Script {
    gate_start_2_run(ctx, GateStart2Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn gate_start_2_ontimer2000(ctx: &Ctx) -> Script {
    gate_start_2_run(ctx, GateStart2Step::OnTimer2000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum JupeGoto2fStep {
    Start,
    OnTouch,
}

fn jupe_goto2f_run(ctx: &Ctx, mut step: JupeGoto2fStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            JupeGoto2fStep::Start => {
                step = JupeGoto2fStep::OnTouch;
                continue 'machine;
            }
            JupeGoto2fStep::OnTouch => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if subject1 == 1 {
                    ctx.call(Function::Warp, vec![Val::from("juperos_01"), Val::from(120), Val::from(72)])?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.call(Function::Warp, vec![Val::from("juperos_01"), Val::from(120), Val::from(112)])?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.call(Function::Warp, vec![Val::from("juperos_01"), Val::from(79), Val::from(112)])?;
                    return Err(Stop::End);
                } else if subject1 == 4 {
                    ctx.call(Function::Warp, vec![Val::from("juperos_01"), Val::from(79), Val::from(72)])?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn jupe_goto2f(ctx: &Ctx) -> Script {
    jupe_goto2f_run(ctx, JupeGoto2fStep::Start, Vec::new()).map(|_| ())
}

pub fn jupe_goto2f_ontouch(ctx: &Ctx) -> Script {
    jupe_goto2f_run(ctx, JupeGoto2fStep::OnTouch, Vec::new()).map(|_| ())
}

fn juperos_manager_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_loc_s = Val::from("");
    let mut l_var_s = Val::from("");
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "Juperos Manager",
        args![
            "I'm the NPC in",
            "charge of managing",
            "the global variables for",
            "the quests related to",
            "Juperos. GM, please",
            "enter the password."
        ],
    )?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])?.number()? < 1 {
        ctx.lines_as("Juperos Manager", args!["Incorrect password."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Juperos Manager",
            args![
                "Select a map. Remember:",
                "Make sure that users aren't",
                "doing the quest on the map",
                "you're going to reset, or else",
                "you'll cancel their progress",
                "through the quest."
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("jupe_Area1:jupe_Area2:jupe_ele_r:jupe_ele")])? {
            1 => {
                l_loc_s = Val::from("jupe_Area1");
                l_var_s = Val::from("$@JupreArea1InUse");
            }
            2 => {
                l_loc_s = Val::from("jupe_Area2");
                l_var_s = Val::from("$@JupreArea2InUse");
            }
            3 => {
                l_loc_s = Val::from("jupe_ele_r");
                l_var_s = Val::from("$@JupeElevatorInUse");
            }
            4 => {
                l_loc_s = Val::from("jupe_ele");
                l_var_s = Val::from("$@JupeElevatorInUse2");
            }
            _ => {}
        }
        ctx.lines_as(
            "Juperos Manager",
            args![
                "You've decided",
                ((Val::from("to reset ") + l_loc_s.clone()) + Val::from(".")),
                "Shall we proceed?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Yes.:No.")])? {
            1 => {
                ctx.lines_as(
                    "Juperos Manager",
                    args![
                        "The global variable in",
                        (l_loc_s.clone() + Val::from(" has been")),
                        "reset to 0. Thank you."
                    ],
                )?;
                runtime::setd(
                    ctx,
                    &l_var_s.clone(),
                    Val::from(0),
                    &mut [
                        (".@loc$", runtime::LocalMut::Scalar(&mut l_loc_s)),
                        (".@var$", runtime::LocalMut::Scalar(&mut l_var_s)),
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Juperos Manager", args!["This command", "has been canceled."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn juperos_manager(ctx: &Ctx) -> Script {
    juperos_manager_body(ctx, Vec::new()).map(|_| ())
}
