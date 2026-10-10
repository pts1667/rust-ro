use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum SatanBroadcastEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer5000,
    OnTimer15000,
    OnTimer25000,
    OnDisable,
}

pub(super) fn satan_broadcast_edq_run(ctx: &Ctx, mut step: SatanBroadcastEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SatanBroadcastEdqStep::Start => {
                step = SatanBroadcastEdqStep::OnInit;
                continue 'machine;
            }
            SatanBroadcastEdqStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Satan Broadcast#edq")])?;
                return Err(Stop::End);
            }
            SatanBroadcastEdqStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Satan Broadcast#edq")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SatanBroadcastEdqStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("moc_fild21"),
                        Val::from("Satan Morocc: I'm very impressed by you weaklings."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SatanBroadcastEdqStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("moc_fild21"),
                        Val::from("Satan Morocc: You may as well deal with all your regrets now. I shall snuff out your life soon!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SatanBroadcastEdqStep::OnTimer25000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("moc_fild21"), Val::from("Satan Morocc: It's only a matter of time until I get you, you worthless insects. Just a little while longer..."), ctx.constant("BC_MAP")?, Val::from("0xFFFF00")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SatanBroadcastEdqStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Satan Broadcast#edq")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SatanSummonEdqStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer5400000,
    OnTimer5415000,
}

pub(super) fn satan_summon_edq_run(ctx: &Ctx, mut step: SatanSummonEdqStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SatanSummonEdqStep::Start => {
                step = SatanSummonEdqStep::OnInit;
                continue 'machine;
            }
            SatanSummonEdqStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Satan Summon#edq")])?;
                return Err(Stop::End);
            }
            SatanSummonEdqStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Satan Summon#edq")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("moc_fild21"),
                        Val::from(177),
                        Val::from(217),
                        Val::from("Satan Morocc"),
                        Val::from(1916),
                        Val::from(1),
                        Val::from("Satan Summon#edq::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SatanSummonEdqStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("moc_fild21"), Val::from("Satan Summon#edq::OnMyMobDead")],
                )?;
                ctx.var("$@re_moc").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Satan Summon#edq")])?;
                return Err(Stop::End);
            }
            SatanSummonEdqStep::OnMyMobDead => {
                ctx.var("$@re_moc").set(Val::from(3))?;
                ctx.var("$@re_moc_time$")
                    .set(ctx.call(Function::GetTimeStr, vec![Val::from("%H%M%S"), Val::from(7)])?)?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("moc_fild21"),
                        Val::from("I'll let live just a little longer. You'll never find me through this time-space gap!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Time-Space Gap#edq::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Satan Broadcast#edq::OnDisable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Group of Evil#edq")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("moc_fild21"), Val::from("Satan Summon#edq::OnMyMobDead")],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Satan Summon#edq")])?;
                return Err(Stop::End);
            }
            SatanSummonEdqStep::OnTimer5400000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("moc_fild21"),
                        Val::from("You weaklings can't even scratch me!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SatanSummonEdqStep::OnTimer5415000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("moc_fild21"),
                        Val::from("I don't have time for this! Go away!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Satan Summon#edq::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}
