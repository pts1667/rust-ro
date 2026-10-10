use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum InsNyd1fTimerStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTimer900000,
    OnTimer1200000,
    OnTimer1500000,
    OnTimer1800000,
    OnTimer1830000,
    OnTimer1850000,
}

pub(super) fn ins_nyd_1f_timer_run(ctx: &Ctx, mut step: InsNyd1fTimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            InsNyd1fTimerStep::Start => {
                step = InsNyd1fTimerStep::OnInstanceInit;
                continue 'machine;
            }
            InsNyd1fTimerStep::OnInstanceInit => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd_1f_timer")])?],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd_1f_timer")])?],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd_1f_timer")])?],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?, Val::from("All")],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnTimer900000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from("World Tree Yggdrasil : There's not much time left. Please hurry."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnTimer1200000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from("World Tree Yggdrasil : My powers are slowly disappearing. Please hurry."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnTimer1500000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from("World Tree Yggdrasil : I'm... almost at my limit... please hurry up."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnTimer1800000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from(
                            "World Tree Yggdrasil : You've failed... but I will use what power I have left... to send you out of here.",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnTimer1830000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@nyd")])?,
                        Val::from("Opening of the Gate has failed."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd1fTimerStep::OnTimer1850000 => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd_1f_timer")])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_call_mon_1")])? + Val::from("::OnDisable"))],
                )?;
                ctx.call(
                    Function::InstanceWarpAll,
                    vec![
                        Val::from("mid_camp"),
                        Val::from(310),
                        Val::from(150),
                        ctx.call(Function::InstanceId, vec![])?,
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum InsNyd1SpawnMobsStep {
    Start,
    OnInstanceInit,
    OnMyTreeDead,
    OnMyRhynDead,
    OnMyPhyDead,
    OnMyAquaDead,
    OnMyPingDead,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Nyd2fEnterBroadStep {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
    OnTimer12000,
    OnTimer15000,
    OnTimer18000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Nyd2fBossEnterCallStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnDisable,
    OnTimer180000,
    OnWarpColor,
    OnMyMobDead,
}
