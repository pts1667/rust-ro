use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum MiniloverArenaStep {
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
    OnTimer360000,
    OnTimer365000,
    OnTimer366000,
    OnTimer367000,
    OnTimer368000,
    OnTimer369000,
    OnTimer370000,
    OnTimer371000,
    OnTimer372000,
    OnTimer373000,
    OnTimer374000,
    OnTimer375000,
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

pub(super) fn minilover_arena_run(ctx: &Ctx, mut step: MiniloverArenaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MiniloverArenaStep::Start => {
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$arena_min60st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                ctx.var("$arena_sec60st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Good day, my name is Minilover, the guide of Time Force Battle for lvl 60s!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Remember your right decision will save a lot of your time!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer5000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_2-1"), Val::from("In order to complete this battle course, you must start from the far left of this room to the clock wise direction. Please move to the far left side. You have 6 minutes from now."), Val::from(0)])?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_2-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_2-1"), Val::from("Remaining Time : 5 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_2-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_2-1"), Val::from("Remaining Time : 4 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_2-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_2-1"), Val::from("Remaining Time : 3 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_2-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_2-1"), Val::from("Remaining Time : 2 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer300000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_2-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_2-1"), Val::from("Remaining Time : 1 minute "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer360000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Your time is over. I hope you had a good time~"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer365000 => {
                step = MiniloverArenaStep::OnTimer366000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer366000 => {
                step = MiniloverArenaStep::OnTimer367000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer367000 => {
                step = MiniloverArenaStep::OnTimer368000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer368000 => {
                step = MiniloverArenaStep::OnTimer369000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer369000 => {
                step = MiniloverArenaStep::OnTimer370000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer370000 => {
                step = MiniloverArenaStep::OnTimer371000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer371000 => {
                step = MiniloverArenaStep::OnTimer372000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer372000 => {
                step = MiniloverArenaStep::OnTimer373000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer373000 => {
                step = MiniloverArenaStep::OnTimer374000;
                continue 'machine;
            }
            MiniloverArenaStep::OnTimer374000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(139),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimer375000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(139),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnFailClearStage")])?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::OnFailClearStage => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#60::OnTimeOver1")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(139),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Minilover#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#60::OnReset_All")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#60::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv60 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On01Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("In order to clear this battle, you must kill at least 5 Goblins while dodging Rotar Zairos!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On01End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On02Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Escape to the north exit from the monsters!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On02End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On03Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("In order to clear this battle, you must kill all Mantises!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On03End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Clear! A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from(
                            "In order to clear this battle, you must kill all non-aggressive monsters while dodging aggressive monsters!",
                        ),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On04End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On05Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("In order to clear this battle, you must kill all monsters except Hydras and Kaphas!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On05End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Clear! A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On06Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("In order to clear this battle, you must kill all Miyabi Dolls and escape to the south exit!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On06End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On07Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("In order to clear this battle, you must kill all monsters!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On07End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Clear! A door to the west room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On08Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_2-1"), Val::from("Please escape to the north exit!"), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On09Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("In order to clear this battle, you must defeat a Goblin Leader!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            MiniloverArenaStep::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_2-1"),
                        Val::from("Boss Clear! - North exit has opened. Thank you."),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
