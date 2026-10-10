use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum HeelAndToeArenaStep {
    Start,
    OnStart,
    OnTimer3000,
    OnTimer4000,
    OnTimer5000,
    OnTimer60000,
    OnTimer120000,
    OnTimer180000,
    OnTimer240000,
    OnTimer300000,
    OnTimer305000,
    OnTimer306000,
    OnTimer307000,
    OnTimer308000,
    OnTimer309000,
    OnTimer310000,
    OnTimer311000,
    OnTimer312000,
    OnTimer313000,
    OnTimer314000,
    OnTimer315000,
    OnTimerOff,
    OnFailClearStage,
    On01Start,
    On01End,
    On02Start,
    On02End,
    On03Start,
    On03End,
    On04Start,
    On04End,
    On05Start,
    On05End,
    On06Start,
    On06End,
    On07Start,
    On07End,
    On08Start,
    On09Start,
    On09End,
}

pub(super) fn heel_and_toe_arena_run(ctx: &Ctx, mut step: HeelAndToeArenaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HeelAndToeArenaStep::Start => {
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$arena_min50st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                ctx.var("$arena_sec50st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Good day, my name is Heel and Toe, the guide of Time Force Battle for lvl 50s!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Remember your right decision will save a lot of your time!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer5000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_1-1"), Val::from("In order to complete this battle course, you must start from the far left of this room to the clock wise direction. Please move to the far left side. You have 5 minutes from now."), Val::from(0)])?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_1-1"), Val::from("Remaining Time : 4 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_1-1"), Val::from("Remaining Time : 3 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_1-1"), Val::from("Remaining Time : 2 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_1-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_1-1"), Val::from("Remaining Time : 1 minute "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer300000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Your time is over. I hope you had a good time~"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer305000 => {
                step = HeelAndToeArenaStep::OnTimer306000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer306000 => {
                step = HeelAndToeArenaStep::OnTimer307000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer307000 => {
                step = HeelAndToeArenaStep::OnTimer308000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer308000 => {
                step = HeelAndToeArenaStep::OnTimer309000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer309000 => {
                step = HeelAndToeArenaStep::OnTimer310000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer310000 => {
                step = HeelAndToeArenaStep::OnTimer311000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer311000 => {
                step = HeelAndToeArenaStep::OnTimer312000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer312000 => {
                step = HeelAndToeArenaStep::OnTimer313000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer313000 => {
                step = HeelAndToeArenaStep::OnTimer314000;
                continue 'machine;
            }
            HeelAndToeArenaStep::OnTimer314000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimer315000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnFailClearStage")])?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::OnFailClearStage => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#50::OnTimeOver1")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Heel and Toe#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#50::OnReset_All")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#50::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv50 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On01Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("In order to clear this battle, you must kill all Smokies!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On01End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On02Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Please escape to the door from monsters!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On02End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On03Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("In order to clear this battle, you must kill all Karakasa!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On03End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Clear! A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("In order to clear this battle, you must kill all Kobolds and escape to the east room!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On04End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On05Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("In order to clear this battle, you must kill all monsters except obstructor monsters!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On05End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Clear! A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On06Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("In order to clear this battle, you must kill all Drops and escape to the south room!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On06End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On07Start => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_1-1"), Val::from("In order to clear this battle, you must get rid of a Red Plant in the center of this room while dodging attacks from Hydras!"), Val::from(0)])?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On07End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Clear! A door to the west room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On08Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_1-1"), Val::from("Please escape to the north exit!"), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On09Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("In order to clear this battle, you must defeat a Vocal!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            HeelAndToeArenaStep::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_1-1"),
                        Val::from("Boss Clear! - A door at the north has opened. Thank you. "),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
