use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum CadillacArenaStep {
    Start,
    OnStart,
    OnTimer3000,
    OnTimer4000,
    OnTimer7000,
    OnTimer60000,
    OnTimer120000,
    OnTimer180000,
    OnTimer240000,
    OnTimer300000,
    OnTimer360000,
    OnTimer420000,
    OnTimer425000,
    OnTimer426000,
    OnTimer427000,
    OnTimer428000,
    OnTimer429000,
    OnTimer430000,
    OnTimer431000,
    OnTimer432000,
    OnTimer433000,
    OnTimer434000,
    OnTimer435000,
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

pub(super) fn cadillac_arena_run(ctx: &Ctx, mut step: CadillacArenaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CadillacArenaStep::Start => {
                return Err(Stop::End);
            }
            CadillacArenaStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$arena_min70st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_MINUTE")?])?)?;
                ctx.var("$arena_sec70st")
                    .set(ctx.call(Function::GetTime, vec![ctx.constant("DT_SECOND")?])?)?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Good day, my name is Cadillac, the guide of Time Force Battle for lvl 70s!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer4000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Remember your right decision will save a lot of your time!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer7000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_3-1"), Val::from("In order to complete this battle course, you must start from the far left of this room to the clock wise direction. Please move to the far left side. You have 7 minutes from now."), Val::from(0)])?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_3-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Remaining Time : 6 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_3-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Remaining Time : 5 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_3-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Remaining Time : 4 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_3-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Remaining Time : 3 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer300000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_3-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Remaining Time : 2 minutes "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer360000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_3-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Remaining Time : 1 minute "), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer420000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Your time is over. I hope you had a good time~"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer425000 => {
                step = CadillacArenaStep::OnTimer426000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer426000 => {
                step = CadillacArenaStep::OnTimer427000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer427000 => {
                step = CadillacArenaStep::OnTimer428000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer428000 => {
                step = CadillacArenaStep::OnTimer429000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer429000 => {
                step = CadillacArenaStep::OnTimer430000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer430000 => {
                step = CadillacArenaStep::OnTimer431000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer431000 => {
                step = CadillacArenaStep::OnTimer432000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer432000 => {
                step = CadillacArenaStep::OnTimer433000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer433000 => {
                step = CadillacArenaStep::OnTimer434000;
                continue 'machine;
            }
            CadillacArenaStep::OnTimer434000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(87),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimer435000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(87),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnFailClearStage")])?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CadillacArenaStep::OnFailClearStage => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("cast#70::OnTimeOver1")])?;
                ctx.call(
                    Function::MapWarp,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("prt_are_in"),
                        Val::from(126),
                        Val::from(87),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cadillac#arena::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena#70::OnReset_All")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("alloff#70::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Lv70 Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On01Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill all Kobolds!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On01End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On02Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill all Horongs and escape!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On02End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("A door to the north room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On03Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill all monsters except Enchanted Peach Trees!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On03End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Clear! A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill all Stem Worms while dodging Bathories!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On04End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("A door to the east room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On05Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill all Argiopes!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On05End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Clear! A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On06Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill all Hammer Goblins!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On06End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("A door to the south room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On07Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill an Alice in the center!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On07End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Clear! A door to the west room has opened!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On08Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("force_3-1"), Val::from("Please escape to the north exit!"), Val::from(0)],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On09Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("In order to clear this battle, you must kill a Kobold Leader and all Kobolds!"),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            CadillacArenaStep::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_3-1"),
                        Val::from("Boss Clear! - A door at the north has opened. Thank you. "),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
