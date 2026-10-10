use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum SlipslowrunPartyStep {
    Start,
    OnStart,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
    OnTimer5000,
    OnTimer60000,
    OnTimer120000,
    OnTimer180000,
    OnTimer240000,
    OnTimer300000,
    OnTimer360000,
    OnTimer420000,
    OnTimer480000,
    OnTimer540000,
    OnTimer600000,
    OnTimer605000,
    OnTimer606000,
    OnTimer607000,
    OnTimer608000,
    OnTimer609000,
    OnTimer610000,
    OnTimer611000,
    OnTimer612000,
    OnTimer613000,
    OnTimer614000,
    OnFail,
    OnTimerOff,
    On01End,
    On02End,
    On03End,
    On04Start,
    On04End1,
    On04End2,
    On05End1,
    On05End2,
    On06End,
    On07End,
    On08End,
    On09End,
    On10End,
}

pub(super) fn slipslowrun_party_run(ctx: &Ctx, mut step: SlipslowrunPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SlipslowrunPartyStep::Start => {
                step = SlipslowrunPartyStep::OnStart;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$arena_minptst")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                ctx.var("$arena_secptst")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Good day, my name is Slipslowrun! I am here to assist you in the party arena battles!"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("The goal of the party arena is eliminating every monster in each room."),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_1-2"), Val::from("There is no order to enter one among 3 rooms at 3 direction. But remember you will eliminate all monsters in a room in order to procceed to the next step."), ctx.constant("BC_ALL")?])?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("You have 10 minutes from now. I expect you will do your best! "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 9 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 8 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 7 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 6 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer300000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 5 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer360000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 4 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer420000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 3 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer480000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 2 minutes "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer540000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-2")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Remaining Time : 1 minute "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer600000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Time is over! Please make sure you do not leave anything behind you before you leave ."),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer605000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::OnReset")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_1-2"), Val::from("prt_are_in"), Val::from(177), Val::from(138)],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer606000 => {
                step = SlipslowrunPartyStep::OnTimer607000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer607000 => {
                step = SlipslowrunPartyStep::OnTimer608000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer608000 => {
                step = SlipslowrunPartyStep::OnTimer609000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer609000 => {
                step = SlipslowrunPartyStep::OnTimer610000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer610000 => {
                step = SlipslowrunPartyStep::OnTimer611000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer611000 => {
                step = SlipslowrunPartyStep::OnTimer612000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer612000 => {
                step = SlipslowrunPartyStep::OnTimer613000;
                continue 'machine;
            }
            SlipslowrunPartyStep::OnTimer613000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_1-2"), Val::from("prt_are_in"), Val::from(177), Val::from(138)],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimer614000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_1-2"), Val::from("prt_are_in"), Val::from(177), Val::from(138)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnFail")])?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnFail => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::OnTimerOff")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_1-2"), Val::from("prt_are_in"), Val::from(177), Val::from(138)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#party::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Ponox::OnStart")])?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On01End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the east room has opened!"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On02End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the west room has opened!"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On03End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the south room has opened!"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the 4th room at the east has opened!"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On04End1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A warp portal at the west has opened! Please clear the 5th room at the end of the west hall! "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On04End2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A warp portal at the west north room has opened! "),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On05End1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the east room has opened~"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On05End2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the north room has opened~"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On06End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the east room has opened~"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On07End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door at the north has opened~"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On08End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A door to the west hall has opened~"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("A west exit has opened!"),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            SlipslowrunPartyStep::On10End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-2"),
                        Val::from("Boss stage cleared! An exit at the east has opened! Thank you."),
                        ctx.constant("BC_ALL")?,
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
