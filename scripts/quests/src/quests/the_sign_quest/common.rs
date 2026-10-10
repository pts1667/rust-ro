use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum SDanceTimerStep {
    Start,
    OnEnable,
    OnDisable,
    OnButtonOff,
    OnDisableAll,
    OnTimer2000,
    OnTimer4000,
    OnTimer6000,
    OnTimer8000,
    OnTimer11000,
    OnTimer13000,
    OnTimer18000,
    OnTimer21000,
    OnTimer24000,
    OnTimer30000,
    OnTimer34000,
    OnTimer38000,
    OnTimer44000,
}

pub(super) fn s_dance_timer_run(ctx: &Ctx, mut step: SDanceTimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SDanceTimerStep::Start => {
                step = SDanceTimerStep::OnEnable;
                continue 'machine;
            }
            SDanceTimerStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnButtonOff => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnDisableAll => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnDisable")])?;
                return Ok(Val::from(0));
            }
            SDanceTimerStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from("The test will be begin shortly. Please do your best~"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" UP LEFT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer6000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer8000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN LEFT & UP RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer11000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer13000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN LEFT & UP RIGHT & UP LEFT & STAY CENTER"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer18000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN LEFT & DOWN RIGHT & DOWN LEFT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer21000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN RIGHT & CENTER & DOWN RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer24000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN LEFT & UP LEFT & UP RIGHT & DOWN RIGHT & CENTER"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer30000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" DOWN LEFT & UP RIGHT & UP LEFT & DOWN RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer34000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" UP RIGHT & DOWN RIGHT & UP RIGHT & DOWN RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer38000 => {
                s_dance_timer_run(ctx, SDanceTimerStep::OnDisableAll, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from(" UP LEFT & UP RIGHT & CENTER & UP RIGHT & DOWN LEFT & DOWN RIGHT"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnUp")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnEnable")])?;
                return Err(Stop::End);
            }
            SDanceTimerStep::OnTimer44000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnReset")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("cmd_in01"),
                        Val::from("Well done."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("Examiner#sd")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Standby Room#sign::OnReset")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SDanceUpStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnUp,
    OnReset,
    OnTouch,
}

pub(super) fn s_dance_up_run(ctx: &Ctx, mut step: SDanceUpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SDanceUpStep::Start => {
                step = SDanceUpStep::OnInit;
                continue 'machine;
            }
            SDanceUpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("s_dance#up")])?;
                return Err(Stop::End);
            }
            SDanceUpStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("s_dance#up")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            SDanceUpStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("s_dance#up")])?;
                return Err(Stop::End);
            }
            SDanceUpStep::OnUp => {
                ctx.var(".s_up").set((ctx.var(".s_up").get()? + Val::from(1)))?;
                return Err(Stop::End);
            }
            SDanceUpStep::OnReset => {
                ctx.var(".s_up").set(Val::from(0))?;
                return Err(Stop::End);
            }
            SDanceUpStep::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("effect\\sign_up.wav"), Val::from(1)])?;
                if ctx.var(".s_up").get()? == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnDisable")])?;
                } else if ctx.var(".s_up").get()? == 2 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnDisable")])?;
                } else if ctx.var(".s_up").get()? == 3 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnDisable")])?;
                } else if ctx.var(".s_up").get()? == 4 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnDisable")])?;
                } else if ctx.var(".s_up").get()? == 5 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnDisable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SDanceDownStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnUp,
    OnReset,
    OnTouch,
}

pub(super) fn s_dance_down_run(ctx: &Ctx, mut step: SDanceDownStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SDanceDownStep::Start => {
                step = SDanceDownStep::OnInit;
                continue 'machine;
            }
            SDanceDownStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                return Err(Stop::End);
            }
            SDanceDownStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("s_dance#down")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            SDanceDownStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("s_dance#down")])?;
                return Err(Stop::End);
            }
            SDanceDownStep::OnUp => {
                ctx.var(".s_down").set((ctx.var(".s_down").get()? + Val::from(1)))?;
                return Err(Stop::End);
            }
            SDanceDownStep::OnReset => {
                ctx.var(".s_down").set(Val::from(0))?;
                return Err(Stop::End);
            }
            SDanceDownStep::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("effect\\sign_down.wav"), Val::from(1)])?;
                if ctx.var(".s_down").get()? == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                } else {
                    if ctx.var(".s_down").get()? == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                    } else {
                        if ctx.var(".s_down").get()? == 3 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                        } else {
                            if ctx.var(".s_down").get()? == 4 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                            } else {
                                if ctx.var(".s_down").get()? == 5 {
                                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                                } else {
                                    if ctx.var(".s_down").get()? == 6 {
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnEnable")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                                    } else if ctx.var(".s_down").get()? == 7 {
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                                    } else if ctx.var(".s_down").get()? == 8 {
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                                    } else if ctx.var(".s_down").get()? == 9 {
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                                    } else if ctx.var(".s_down").get()? == 10 {
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnDisable")])?;
                                    }
                                }
                            }
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SDanceLeftStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnUp,
    OnReset,
    OnTouch,
}

pub(super) fn s_dance_left_run(ctx: &Ctx, mut step: SDanceLeftStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SDanceLeftStep::Start => {
                step = SDanceLeftStep::OnInit;
                continue 'machine;
            }
            SDanceLeftStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                return Err(Stop::End);
            }
            SDanceLeftStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("s_dance#left")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            SDanceLeftStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("s_dance#left")])?;
                return Err(Stop::End);
            }
            SDanceLeftStep::OnUp => {
                ctx.var(".s_left").set((ctx.var(".s_left").get()? + Val::from(1)))?;
                return Err(Stop::End);
            }
            SDanceLeftStep::OnReset => {
                ctx.var(".s_left").set(Val::from(0))?;
                return Err(Stop::End);
            }
            SDanceLeftStep::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("effect\\sign_left.wav"), Val::from(1)])?;
                if ctx.var(".s_left").get()? == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                } else {
                    if ctx.var(".s_left").get()? == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                    } else {
                        if ctx.var(".s_left").get()? == 3 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                        } else if ctx.var(".s_left").get()? == 4 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                        } else if ctx.var(".s_left").get()? == 5 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                        } else if ctx.var(".s_left").get()? == 6 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                        } else if ctx.var(".s_left").get()? == 7 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnDisable")])?;
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SDanceRightStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnUp,
    OnReset,
    OnTouch,
}

pub(super) fn s_dance_right_run(ctx: &Ctx, mut step: SDanceRightStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SDanceRightStep::Start => {
                step = SDanceRightStep::OnInit;
                continue 'machine;
            }
            SDanceRightStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                return Err(Stop::End);
            }
            SDanceRightStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("s_dance#right")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            SDanceRightStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("s_dance#right")])?;
                return Err(Stop::End);
            }
            SDanceRightStep::OnUp => {
                ctx.var(".s_right").set((ctx.var(".s_right").get()? + Val::from(1)))?;
                return Err(Stop::End);
            }
            SDanceRightStep::OnReset => {
                ctx.var(".s_right").set(Val::from(0))?;
                return Err(Stop::End);
            }
            SDanceRightStep::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("effect\\sign_right.wav"), Val::from(1)])?;
                if ctx.var(".s_right").get()? == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                } else {
                    if ctx.var(".s_right").get()? == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnUp")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnEnable")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                    } else {
                        if ctx.var(".s_right").get()? == 3 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                            ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                        } else {
                            if ctx.var(".s_right").get()? == 4 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#up::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                            } else if ctx.var(".s_right").get()? == 5 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                            } else if ctx.var(".s_right").get()? == 6 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                            } else if ctx.var(".s_right").get()? == 7 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                            } else if ctx.var(".s_right").get()? == 8 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnUp")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#left::OnEnable")])?;
                                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnDisable")])?;
                            }
                        }
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SDanceCenStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnUp,
    OnReset,
    OnTouch,
}

pub(super) fn s_dance_cen_run(ctx: &Ctx, mut step: SDanceCenStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SDanceCenStep::Start => {
                step = SDanceCenStep::OnInit;
                continue 'machine;
            }
            SDanceCenStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnDisable")])?;
                return Err(Stop::End);
            }
            SDanceCenStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("s_dance#cen")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            SDanceCenStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("s_dance#cen")])?;
                return Err(Stop::End);
            }
            SDanceCenStep::OnUp => {
                ctx.var(".s_center").set((ctx.var(".s_center").get()? + Val::from(1)))?;
                return Err(Stop::End);
            }
            SDanceCenStep::OnReset => {
                ctx.var(".s_center").set(Val::from(0))?;
                return Err(Stop::End);
            }
            SDanceCenStep::OnTouch => {
                ctx.call(Function::SoundEffect, vec![Val::from("effect\\sign_center.wav"), Val::from(1)])?;
                if ctx.var(".s_center").get()? == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnDisable")])?;
                } else if ctx.var(".s_center").get()? == 2 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#down::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnDisable")])?;
                } else if ctx.var(".s_center").get()? == 3 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnDisable")])?;
                } else if ctx.var(".s_center").get()? == 4 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Examiner#sd::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnUp")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#right::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("s_dance#cen::OnDisable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ExaminerSdStep {
    Start,
    OnInit,
    OnTouch,
    OnUp,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SerinDummyStep {
    Start,
    OnDisable,
    OnInit,
    OnEnable,
    OnStart,
    OnTimer3000,
    OnTimer6000,
    OnTimer9000,
    OnTimer13000,
}

pub(super) fn serin_dummy_run(ctx: &Ctx, mut step: SerinDummyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SerinDummyStep::Start => {
                return Err(Stop::End);
            }
            SerinDummyStep::OnDisable => {
                step = SerinDummyStep::OnInit;
                continue 'machine;
            }
            SerinDummyStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Serin#dummy")])?;
                return Err(Stop::End);
            }
            SerinDummyStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Serin#dummy")])?;
                return Err(Stop::End);
            }
            SerinDummyStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SerinDummyStep::OnTimer3000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_GLASSWALL")?])?;
                return Err(Stop::End);
            }
            SerinDummyStep::OnTimer6000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_CRASHEARTH")?])?;
                return Err(Stop::End);
            }
            SerinDummyStep::OnTimer9000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
            SerinDummyStep::OnTimer13000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#dummy::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Dark Lord#serin::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Serin#serin::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum GeffeniaWarpStep {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
    OnTimer10000,
    OnTimer20000,
    OnTimer30000,
    OnTimer40000,
    OnTimer45000,
}

pub(super) fn geffenia_warp_run(ctx: &Ctx, mut step: GeffeniaWarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_geffenia_warp = Val::from(0);
    'machine: loop {
        match step {
            GeffeniaWarpStep::Start => {
                step = GeffeniaWarpStep::OnInit;
                continue 'machine;
            }
            GeffeniaWarpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Geffenia Warp")])?;
                return Err(Stop::End);
            }
            GeffeniaWarpStep::OnTouch => {
                l_geffenia_warp = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if l_geffenia_warp.clone() == 1 {
                    ctx.call(Function::Warp, vec![Val::from("gefenia01"), Val::from(58), Val::from(169)])?;
                } else if l_geffenia_warp.clone() == 2 {
                    ctx.call(Function::Warp, vec![Val::from("gefenia02"), Val::from(116), Val::from(115)])?;
                } else if l_geffenia_warp.clone() == 3 {
                    ctx.call(Function::Warp, vec![Val::from("gefenia03"), Val::from(130), Val::from(206)])?;
                } else if l_geffenia_warp.clone() == 4 {
                    ctx.call(Function::Warp, vec![Val::from("gefenia04"), Val::from(133), Val::from(88)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(116), Val::from(115)])?;
                }
                return Err(Stop::End);
            }
            GeffeniaWarpStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Geffenia Warp")])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAPPILLAR2")?])?;
                return Err(Stop::End);
            }
            GeffeniaWarpStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Geffenia Warp")])?;
                return Err(Stop::End);
            }
            GeffeniaWarpStep::OnTimer10000 => {
                step = GeffeniaWarpStep::OnTimer20000;
                continue 'machine;
            }
            GeffeniaWarpStep::OnTimer20000 => {
                step = GeffeniaWarpStep::OnTimer30000;
                continue 'machine;
            }
            GeffeniaWarpStep::OnTimer30000 => {
                step = GeffeniaWarpStep::OnTimer40000;
                continue 'machine;
            }
            GeffeniaWarpStep::OnTimer40000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAPPILLAR2")?])?;
                return Err(Stop::End);
            }
            GeffeniaWarpStep::OnTimer45000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Geffenia Warp::OnDisable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("geffen"),
                        Val::from("The portal to Geffenia is now closed."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x9CFF00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum WarpSerinStep {
    Start,
    OnDisable,
    OnInit,
    OnTouch,
    OnEnable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum WarpWitchStep {
    Start,
    OnDisable,
    OnInit,
    OnEnable,
    OnTouch,
}
