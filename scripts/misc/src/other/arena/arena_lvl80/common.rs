use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum OctusArenaStep {
    Start,
    OnStart,
    OnTimer3000,
    OnTimer4000,
    OnTimer8000,
    OnTimer60000,
    OnTimer120000,
    OnTimer180000,
    OnTimer240000,
    OnTimer300000,
    OnTimer360000,
    OnTimer420000,
    OnTimer480000,
    OnTimer485000,
    OnTimer486000,
    OnTimer487000,
    OnTimer488000,
    OnTimer489000,
    OnTimer490000,
    OnTimer491000,
    OnTimer492000,
    OnTimer493000,
    OnTimer494000,
    OnTimer495000,
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

pub(super) fn octus_arena_run(ctx: &Ctx, mut step: OctusArenaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OctusArenaStep::Start => {
                return Err(Stop::End);
            }
            OctusArenaStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$arena_min80st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                ctx.var("$arena_sec80st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Good day, my name is Octus, the guide of Time Force Battle for lvl 80s!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Remember your right decision will save a lot of your time!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer8000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_4-1"), Val::from("In order to complete this battle course, you must start from the far left of this room to the clock wise direction. Please move to the far left side. You have 7 minutes from now."), Val::from(0)])?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 7 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 6 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 5 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 4 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer300000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 3 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer360000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 2 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer420000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_4-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Remaining Time : 1 minute "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer480000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Your time is over. I hope you had a good time~"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer485000 => {
                step = OctusArenaStep::OnTimer486000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer486000 => {
                step = OctusArenaStep::OnTimer487000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer487000 => {
                step = OctusArenaStep::OnTimer488000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer488000 => {
                step = OctusArenaStep::OnTimer489000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer489000 => {
                step = OctusArenaStep::OnTimer490000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer490000 => {
                step = OctusArenaStep::OnTimer491000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer491000 => {
                step = OctusArenaStep::OnTimer492000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer492000 => {
                step = OctusArenaStep::OnTimer493000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer493000 => {
                step = OctusArenaStep::OnTimer494000;
                continue 'machine;
            }
            OctusArenaStep::OnTimer494000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("prt_are_in"),
                        Val::from(178),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimer495000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("prt_are_in"),
                        Val::from(178),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnFailClearStage")])?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            OctusArenaStep::OnFailClearStage => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#80::OnTimeOver1")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("prt_are_in"),
                        Val::from(178),
                        Val::from(190),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Octus#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#80::OnReset_All")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#80::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv80 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            OctusArenaStep::On01Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, you must kill all Nightmares!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On01End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On02Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, you must kill all monsters!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On02End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On03Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, you must kill all Assaulters!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On03End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Clear! A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, you must kill all Nine Tails!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On04End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On05Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, you must kill all Walking Petites!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On05End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Clear! A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On06Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, kill all monsters in this room!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On06End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On07Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, kill all Fur-Seals while dodging Mermen!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On07End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Clear! A door to the west room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On08Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_4-1"), Val::from("Please escape to the north exit!"), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On09Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("In order to clear this battle, you must defeat an Ancient Mummy!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            OctusArenaStep::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_4-1"),
                        Val::from("Boss Clear! - A door at the north has opened. Thank you. "),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
