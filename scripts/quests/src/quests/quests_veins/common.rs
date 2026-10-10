use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum VolTimeStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer10000,
    OnTimer15000,
    OnTimer20000,
    OnTimer30000,
    OnTimer35000,
    OnTimer40000,
    OnTimer45000,
    OnTimer50000,
    OnTimer55000,
}

pub(super) fn vol_time_run(ctx: &Ctx, mut step: VolTimeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VolTimeStep::Start => {
                step = VolTimeStep::OnInit;
                continue 'machine;
            }
            VolTimeStep::OnInit => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            VolTimeStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Rachel Guard#vol1_1")])?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer10000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Guard Karlum: Lamir! It's Karlum! Your love is here!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer15000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Lamir: Karlum? What are you doing here?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer20000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from(
                            "Guard Karlum: Lamir, you can stop pretending now. I've come to realize that your coldness masks your love~",
                        ),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Lamir: What are you talking about? Sorry, Karlum, but I don't have any special feelings for you."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer35000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Guard Karlum: I know, it's embarrassing to confess your true feelings~"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer40000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Guard Karlum: However, I can't deny that your shyness is breaking my heart."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer45000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Lamir: Karlum, when will you realize that I haven't, and won't ever fall in love with you?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer50000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Guard Karlum: ............."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            VolTimeStep::OnTimer55000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Guard Karlum: Wha--? But I thought...? Huh, sorry. I should get going..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol1_1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Rachel Guard#vol1")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum VolTime2Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer30000,
}

pub(super) fn vol_time2_run(ctx: &Ctx, mut step: VolTime2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VolTime2Step::Start => {
                step = VolTime2Step::OnInit;
                continue 'machine;
            }
            VolTime2Step::OnInit => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            VolTime2Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            VolTime2Step::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ra_temin"),
                        Val::from("Guard Krodger: Phew~, now I'm done cleaning up this mess."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFCE00"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Rachel Guard#vol2_1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Rachel Guard#vol2")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Flower Vase#vol")])?;
                return Err(Stop::End);
            }
        }
    }
}
