use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum ArenaAco2Step {
    Start,
    OnStart,
    OnReset01,
    OnReset02,
    OnReset03,
    OnReset04,
    OnReset05,
    OnReset06,
    OnReset07,
    OnReset08,
    OnReset09,
}

fn arena_aco2_run(ctx: &Ctx, mut step: ArenaAco2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaAco2Step::Start => {
                step = ArenaAco2Step::OnStart;
                continue 'machine;
            }
            ArenaAco2Step::OnStart => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnEnable")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset01 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_01_02#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset02 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#pri::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02_03#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_03start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset03 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_04start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset04 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_04_05#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_05start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset05 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_06start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset06 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_07start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset07 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset08 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_09start#pri")])?;
                return Err(Stop::End);
            }
            ArenaAco2Step::OnReset09 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#aco")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn arena_aco2(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onstart(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_01(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset01, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_02(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset02, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_03(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset03, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_04(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset04, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_05(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset05, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_06(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset06, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_07(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset07, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_08(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset08, Vec::new()).map(|_| ())
}

pub fn arena_aco2_onreset_09(ctx: &Ctx) -> Script {
    arena_aco2_run(ctx, ArenaAco2Step::OnReset09, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TroccoAco2Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer3000,
    OnTimer4000,
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

fn trocco_aco2_run(ctx: &Ctx, mut step: TroccoAco2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TroccoAco2Step::Start => {
                step = TroccoAco2Step::OnEnable;
                continue 'machine;
            }
            TroccoAco2Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$@arn_1").set(ctx.call(Function::GetTimeTick, vec![Val::from(2)])?)?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Good day! I am Trocco, your host for the Acolyte Class Time Force Battle!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Make your decisions carefully. Here, the right choices make the difference between victory and defeat!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_5-1"), Val::from("You now have 8 minutes to complete this challenge. Enter the Left Entrance now to begin. Remember that you will be traveling clockwise as you clear the stages."), runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?])?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 7 Minutes"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 6 Minutes"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 5 Minutes"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 4 Minutes"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer300000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 3 Minutes"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer360000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 2 Minutes"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer420000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                }
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time left: 1 Minute"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer480000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Time's up! I hope you had fun!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer485000 => {
                step = TroccoAco2Step::OnTimer486000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer486000 => {
                step = TroccoAco2Step::OnTimer487000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer487000 => {
                step = TroccoAco2Step::OnTimer488000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer488000 => {
                step = TroccoAco2Step::OnTimer489000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer489000 => {
                step = TroccoAco2Step::OnTimer490000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer490000 => {
                step = TroccoAco2Step::OnTimer491000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer491000 => {
                step = TroccoAco2Step::OnTimer492000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer492000 => {
                step = TroccoAco2Step::OnTimer493000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer493000 => {
                step = TroccoAco2Step::OnTimer494000;
                continue 'machine;
            }
            TroccoAco2Step::OnTimer494000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(177), Val::from(86)],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimer495000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(177), Val::from(86)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnFailClearStage")])?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TroccoAco2Step::OnFailClearStage => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(177), Val::from(86)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On01Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Kill all Zombies!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On01End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The North Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On02Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Defeat all monsters!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On02End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The North Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On03Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Destroy all Red Plants!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On03End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The East Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Obliterate every monster!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On04End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The East Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On05Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Defeat Gargoyle!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On05End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The South Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On06Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Kill all Khalitzburgs!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On06End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The South Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On07Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Kill 1 Ancient Mummy!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On07End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The West Exit has opened!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On08Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("The time has come for the Boss Battle. Please move north into the next room..."),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On09Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Defeat the Boss Monster!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco2Step::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Boss Cleared! The North Exit has opened! Thank you."),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn trocco_aco2(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_onenable(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer2000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer3000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer4000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer60000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer120000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer180000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer240000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer300000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer360000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer420000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer420000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer480000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer480000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer485000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer485000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer486000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer486000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer487000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer487000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer488000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer488000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer489000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer489000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer490000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer490000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer491000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer491000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer492000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer492000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer493000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer493000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer494000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer494000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimer495000(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimer495000, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_ontimeroff(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_onfailclearstage(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::OnFailClearStage, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on01_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On01Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on01_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On01End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on02_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On02Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on02_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On02End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on03_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On03Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on03_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On03End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on04_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On04Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on04_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On04End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on05_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On05Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on05_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On05End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on06_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On06Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on06_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On06End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on07_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On07Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on07_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On07End, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on08_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On08Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on09_start(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On09Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco2_on09_end(ctx: &Ctx) -> Script {
    trocco_aco2_run(ctx, TroccoAco2Step::On09End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01startPriStep {
    Start,
    OnTouch,
}

fn force_01start_pri_run(ctx: &Ctx, mut step: Force01startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01startPriStep::Start => {
                step = Force01startPriStep::OnTouch;
                continue 'machine;
            }
            Force01startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#pri::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_pri(ctx: &Ctx) -> Script {
    force_01start_pri_run(ctx, Force01startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_pri_ontouch(ctx: &Ctx) -> Script {
    force_01start_pri_run(ctx, Force01startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mobPriStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_01mob_pri_run(ctx: &Ctx, mut step: Force01mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mobPriStep::Start => {
                step = Force01mobPriStep::OnEnable;
                continue 'machine;
            }
            Force01mobPriStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On01_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(13),
                        Val::from(9),
                        Val::from(38),
                        Val::from(41),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(10),
                        Val::from("force_01mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_01mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_01mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On01_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_01")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_pri(ctx: &Ctx) -> Script {
    force_01mob_pri_run(ctx, Force01mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_pri_onenable(ctx: &Ctx) -> Script {
    force_01mob_pri_run(ctx, Force01mobPriStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_pri_onreset(ctx: &Ctx) -> Script {
    force_01mob_pri_run(ctx, Force01mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_pri_run(ctx, Force01mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02startPriStep {
    Start,
    OnTouch,
}

fn force_02start_pri_run(ctx: &Ctx, mut step: Force02startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02startPriStep::Start => {
                step = Force02startPriStep::OnTouch;
                continue 'machine;
            }
            Force02startPriStep::OnTouch => {
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#pri::OnOn1")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#pri::OnOn2")])?;
                }
                ctx.call(Function::DisableNpc, vec![Val::from("force_02start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_pri(ctx: &Ctx) -> Script {
    force_02start_pri_run(ctx, Force02startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_pri_ontouch(ctx: &Ctx) -> Script {
    force_02start_pri_run(ctx, Force02startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mobPriStep {
    Start,
    OnOn1,
    OnOn2,
    OnReset,
    OnMyMobDead,
}

fn force_02mob_pri_run(ctx: &Ctx, mut step: Force02mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mobPriStep::Start => {
                step = Force02mobPriStep::OnOn1;
                continue 'machine;
            }
            Force02mobPriStep::OnOn1 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On02_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(22),
                        Val::from(69),
                        Val::from(29),
                        Val::from(130),
                        Val::from("Orc Skeleton"),
                        Val::from(1462),
                        Val::from(5),
                        Val::from("force_02mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mobPriStep::OnOn2 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On02_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(22),
                        Val::from(69),
                        Val::from(29),
                        Val::from(130),
                        Val::from("Ghoul"),
                        Val::from(1423),
                        Val::from(5),
                        Val::from("force_02mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_02mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_02mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On02_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_02")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_pri(ctx: &Ctx) -> Script {
    force_02mob_pri_run(ctx, Force02mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_pri_onon1(ctx: &Ctx) -> Script {
    force_02mob_pri_run(ctx, Force02mobPriStep::OnOn1, Vec::new()).map(|_| ())
}

pub fn force_02mob_pri_onon2(ctx: &Ctx) -> Script {
    force_02mob_pri_run(ctx, Force02mobPriStep::OnOn2, Vec::new()).map(|_| ())
}

pub fn force_02mob_pri_onreset(ctx: &Ctx) -> Script {
    force_02mob_pri_run(ctx, Force02mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_pri_run(ctx, Force02mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03startPriStep {
    Start,
    OnTouch,
}

fn force_03start_pri_run(ctx: &Ctx, mut step: Force03startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03startPriStep::Start => {
                step = Force03startPriStep::OnTouch;
                continue 'machine;
            }
            Force03startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#pri::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03start_pri(ctx: &Ctx) -> Script {
    force_03start_pri_run(ctx, Force03startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_pri_ontouch(ctx: &Ctx) -> Script {
    force_03start_pri_run(ctx, Force03startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mobPriStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_pri_run(ctx: &Ctx, mut step: Force03mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mobPriStep::Start => {
                step = Force03mobPriStep::OnEnable;
                continue 'machine;
            }
            Force03mobPriStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On03_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(14),
                        Val::from(163),
                        Val::from(37),
                        Val::from(185),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(5),
                        Val::from("force_03mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_03mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_03mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On03_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_03")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_pri(ctx: &Ctx) -> Script {
    force_03mob_pri_run(ctx, Force03mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_pri_onenable(ctx: &Ctx) -> Script {
    force_03mob_pri_run(ctx, Force03mobPriStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_pri_onreset(ctx: &Ctx) -> Script {
    force_03mob_pri_run(ctx, Force03mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_pri_run(ctx, Force03mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04startPriStep {
    Start,
    OnTouch,
}

fn force_04start_pri_run(ctx: &Ctx, mut step: Force04startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04startPriStep::Start => {
                step = Force04startPriStep::OnTouch;
                continue 'machine;
            }
            Force04startPriStep::OnTouch => {
                if ctx.var("BaseLevel").get()?.number()? < 90 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#pri::OnOn1")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_04mob#pri::OnOn2")])?;
                }
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_pri(ctx: &Ctx) -> Script {
    force_04start_pri_run(ctx, Force04startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_pri_ontouch(ctx: &Ctx) -> Script {
    force_04start_pri_run(ctx, Force04startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04mobPriStep {
    Start,
    OnOn1,
    OnOn2,
    OnReset,
    OnMyMobDead,
}

fn force_04mob_pri_run(ctx: &Ctx, mut step: Force04mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04mobPriStep::Start => {
                step = Force04mobPriStep::OnOn1;
                continue 'machine;
            }
            Force04mobPriStep::OnOn1 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On04_Start")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(71), Val::from(130)])?,
                        ctx.call(Function::Rand, vec![Val::from(170), Val::from(178)])?,
                        Val::from("Evil Druid"),
                        Val::from(1435),
                        Val::from(1),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(71), Val::from(130)])?,
                        ctx.call(Function::Rand, vec![Val::from(170), Val::from(178)])?,
                        Val::from("Wrath"),
                        Val::from(1475),
                        Val::from(1),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(71), Val::from(130)])?,
                        ctx.call(Function::Rand, vec![Val::from(170), Val::from(178)])?,
                        Val::from("Zombie Prisoner"),
                        Val::from(1480),
                        Val::from(1),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(71), Val::from(130)])?,
                        ctx.call(Function::Rand, vec![Val::from(170), Val::from(178)])?,
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mobPriStep::OnOn2 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On04_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(71),
                        Val::from(170),
                        Val::from(130),
                        Val::from(178),
                        Val::from("Evil Druid"),
                        Val::from(1435),
                        Val::from(2),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(71),
                        Val::from(170),
                        Val::from(130),
                        Val::from(178),
                        Val::from("Wrath"),
                        Val::from(1475),
                        Val::from(2),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(71),
                        Val::from(170),
                        Val::from(130),
                        Val::from(178),
                        Val::from("Zombie Prisoner"),
                        Val::from(1480),
                        Val::from(2),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(71), Val::from(130)])?,
                        ctx.call(Function::Rand, vec![Val::from(170), Val::from(178)])?,
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_04mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_04mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_04mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On04_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_04")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04mob_pri(ctx: &Ctx) -> Script {
    force_04mob_pri_run(ctx, Force04mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_04mob_pri_onon1(ctx: &Ctx) -> Script {
    force_04mob_pri_run(ctx, Force04mobPriStep::OnOn1, Vec::new()).map(|_| ())
}

pub fn force_04mob_pri_onon2(ctx: &Ctx) -> Script {
    force_04mob_pri_run(ctx, Force04mobPriStep::OnOn2, Vec::new()).map(|_| ())
}

pub fn force_04mob_pri_onreset(ctx: &Ctx) -> Script {
    force_04mob_pri_run(ctx, Force04mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_04mob_pri_run(ctx, Force04mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05startPriStep {
    Start,
    OnTouch,
}

fn force_05start_pri_run(ctx: &Ctx, mut step: Force05startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05startPriStep::Start => {
                step = Force05startPriStep::OnTouch;
                continue 'machine;
            }
            Force05startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#pri::OnEnable")])?;
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#pri::OnOn1")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#pri::OnOn2")])?;
                }
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_pri(ctx: &Ctx) -> Script {
    force_05start_pri_run(ctx, Force05startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_pri_ontouch(ctx: &Ctx) -> Script {
    force_05start_pri_run(ctx, Force05startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05exPriStep {
    Start,
    OnOn1,
    OnOn2,
    OnReset,
    OnMyMobDead,
}

fn force_05ex_pri_run(ctx: &Ctx, mut step: Force05exPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05exPriStep::Start => {
                step = Force05exPriStep::OnOn1;
                continue 'machine;
            }
            Force05exPriStep::OnOn1 => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(161),
                        Val::from(159),
                        Val::from(187),
                        Val::from(190),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(10),
                        Val::from("force_05ex#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05exPriStep::OnOn2 => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(161),
                        Val::from(159),
                        Val::from(187),
                        Val::from(190),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(10),
                        Val::from("force_05ex#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05exPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_05ex#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05exPriStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05ex_pri(ctx: &Ctx) -> Script {
    force_05ex_pri_run(ctx, Force05exPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05ex_pri_onon1(ctx: &Ctx) -> Script {
    force_05ex_pri_run(ctx, Force05exPriStep::OnOn1, Vec::new()).map(|_| ())
}

pub fn force_05ex_pri_onon2(ctx: &Ctx) -> Script {
    force_05ex_pri_run(ctx, Force05exPriStep::OnOn2, Vec::new()).map(|_| ())
}

pub fn force_05ex_pri_onreset(ctx: &Ctx) -> Script {
    force_05ex_pri_run(ctx, Force05exPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05ex_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_05ex_pri_run(ctx, Force05exPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05mobPriStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_05mob_pri_run(ctx: &Ctx, mut step: Force05mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mobPriStep::Start => {
                step = Force05mobPriStep::OnEnable;
                continue 'machine;
            }
            Force05mobPriStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On05_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(161),
                        Val::from(159),
                        Val::from(187),
                        Val::from(190),
                        Val::from("Gargoyle"),
                        Val::from(1597),
                        Val::from(4),
                        Val::from("force_05mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_05mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_05mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_05ex#pri::OnReset")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On05_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_05")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_pri(ctx: &Ctx) -> Script {
    force_05mob_pri_run(ctx, Force05mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_pri_onenable(ctx: &Ctx) -> Script {
    force_05mob_pri_run(ctx, Force05mobPriStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_pri_onreset(ctx: &Ctx) -> Script {
    force_05mob_pri_run(ctx, Force05mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_pri_run(ctx, Force05mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06startPriStep {
    Start,
    OnTouch,
}

fn force_06start_pri_run(ctx: &Ctx, mut step: Force06startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06startPriStep::Start => {
                step = Force06startPriStep::OnTouch;
                continue 'machine;
            }
            Force06startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#pri::OnEnable")])?;
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#pri::OnOn1")])?;
                } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#pri::OnOn2")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#pri::OnOn3")])?;
                }
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_pri(ctx: &Ctx) -> Script {
    force_06start_pri_run(ctx, Force06startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_pri_ontouch(ctx: &Ctx) -> Script {
    force_06start_pri_run(ctx, Force06startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06exPriStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_06ex_pri_run(ctx: &Ctx, mut step: Force06exPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06exPriStep::Start => {
                step = Force06exPriStep::OnEnable;
                continue 'machine;
            }
            Force06exPriStep::OnEnable => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(67),
                        Val::from(178),
                        Val::from(131),
                        Val::from("Permeter"),
                        Val::from(1314),
                        Val::from(5),
                        Val::from("force_06ex#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06exPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_06ex#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06exPriStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06ex_pri(ctx: &Ctx) -> Script {
    force_06ex_pri_run(ctx, Force06exPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06ex_pri_onenable(ctx: &Ctx) -> Script {
    force_06ex_pri_run(ctx, Force06exPriStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06ex_pri_onreset(ctx: &Ctx) -> Script {
    force_06ex_pri_run(ctx, Force06exPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06ex_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_06ex_pri_run(ctx, Force06exPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mobPriStep {
    Start,
    OnOn1,
    OnOn2,
    OnOn3,
    OnReset,
    OnMyMobDead,
}

fn force_06mob_pri_run(ctx: &Ctx, mut step: Force06mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mobPriStep::Start => {
                step = Force06mobPriStep::OnOn1;
                continue 'machine;
            }
            Force06mobPriStep::OnOn1 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On06_Start")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(170), Val::from(178)])?,
                        ctx.call(Function::Rand, vec![Val::from(67), Val::from(131)])?,
                        Val::from("Khalitzburg"),
                        Val::from(1438),
                        Val::from(1),
                        Val::from("force_06mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mobPriStep::OnOn2 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On06_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(67),
                        Val::from(178),
                        Val::from(131),
                        Val::from("Khalitzburg"),
                        Val::from(1438),
                        Val::from(2),
                        Val::from("force_06mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mobPriStep::OnOn3 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On06_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(67),
                        Val::from(178),
                        Val::from(131),
                        Val::from("Khalitzburg"),
                        Val::from(1438),
                        Val::from(4),
                        Val::from("force_06mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_06mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_06mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_06ex#pri::OnReset")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On06_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_06")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_pri(ctx: &Ctx) -> Script {
    force_06mob_pri_run(ctx, Force06mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_pri_onon1(ctx: &Ctx) -> Script {
    force_06mob_pri_run(ctx, Force06mobPriStep::OnOn1, Vec::new()).map(|_| ())
}

pub fn force_06mob_pri_onon2(ctx: &Ctx) -> Script {
    force_06mob_pri_run(ctx, Force06mobPriStep::OnOn2, Vec::new()).map(|_| ())
}

pub fn force_06mob_pri_onon3(ctx: &Ctx) -> Script {
    force_06mob_pri_run(ctx, Force06mobPriStep::OnOn3, Vec::new()).map(|_| ())
}

pub fn force_06mob_pri_onreset(ctx: &Ctx) -> Script {
    force_06mob_pri_run(ctx, Force06mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_pri_run(ctx, Force06mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07startPriStep {
    Start,
    OnTouch,
}

fn force_07start_pri_run(ctx: &Ctx, mut step: Force07startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07startPriStep::Start => {
                step = Force07startPriStep::OnTouch;
                continue 'machine;
            }
            Force07startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#pri::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07start_pri(ctx: &Ctx) -> Script {
    force_07start_pri_run(ctx, Force07startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_pri_ontouch(ctx: &Ctx) -> Script {
    force_07start_pri_run(ctx, Force07startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mobPriStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_07mob_pri_run(ctx: &Ctx, mut step: Force07mobPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mobPriStep::Start => {
                step = Force07mobPriStep::OnEnable;
                continue 'machine;
            }
            Force07mobPriStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On07_Start")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(160), Val::from(188)])?,
                        ctx.call(Function::Rand, vec![Val::from(9), Val::from(42)])?,
                        Val::from("Ancient Mummy"),
                        Val::from(1522),
                        Val::from(1),
                        Val::from("force_07mob#pri::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mobPriStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_07mob#pri::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mobPriStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_07mob#pri::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On07_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_07")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_pri(ctx: &Ctx) -> Script {
    force_07mob_pri_run(ctx, Force07mobPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_pri_onenable(ctx: &Ctx) -> Script {
    force_07mob_pri_run(ctx, Force07mobPriStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_pri_onreset(ctx: &Ctx) -> Script {
    force_07mob_pri_run(ctx, Force07mobPriStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_pri_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_pri_run(ctx, Force07mobPriStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08startPriStep {
    Start,
    OnTouch,
}

fn force_08start_pri_run(ctx: &Ctx, mut step: Force08startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08startPriStep::Start => {
                step = Force08startPriStep::OnTouch;
                continue 'machine;
            }
            Force08startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On08_Start")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnReset_08")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_pri(ctx: &Ctx) -> Script {
    force_08start_pri_run(ctx, Force08startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_pri_ontouch(ctx: &Ctx) -> Script {
    force_08start_pri_run(ctx, Force08startPriStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09startPriStep {
    Start,
    OnTouch,
}

fn force_09start_pri_run(ctx: &Ctx, mut step: Force09startPriStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09startPriStep::Start => {
                step = Force09startPriStep::OnTouch;
                continue 'machine;
            }
            Force09startPriStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco2::On09_Start")])?;
                if ctx.var("BaseLevel").get()?.number()? < 70 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#pri::OnOn4")])?;
                } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#pri::OnOn1")])?;
                } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#pri::OnOn2")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#pri::OnOn3")])?;
                }
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#pri")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_pri(ctx: &Ctx) -> Script {
    force_09start_pri_run(ctx, Force09startPriStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_pri_ontouch(ctx: &Ctx) -> Script {
    force_09start_pri_run(ctx, Force09startPriStep::OnTouch, Vec::new()).map(|_| ())
}
