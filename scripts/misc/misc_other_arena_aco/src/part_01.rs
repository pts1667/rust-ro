use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum OnlyacoArenaStep {
    Start,
    OnTouch,
}

fn onlyaco_arena_run(ctx: &Ctx, mut step: OnlyacoArenaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OnlyacoArenaStep::Start => {
                step = OnlyacoArenaStep::OnTouch;
                continue 'machine;
            }
            OnlyacoArenaStep::OnTouch => {
                if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::Warp, vec![Val::from("arena_room"), Val::from(135), Val::from(129)])?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("Only Acolyte class are applicable to join this mode.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn onlyaco_arena(ctx: &Ctx) -> Script {
    onlyaco_arena_run(ctx, OnlyacoArenaStep::Start, Vec::new()).map(|_| ())
}

pub fn onlyaco_arena_ontouch(ctx: &Ctx) -> Script {
    onlyaco_arena_run(ctx, OnlyacoArenaStep::OnTouch, Vec::new()).map(|_| ())
}

fn acolyte_waiting_room_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn acolyte_waiting_room(ctx: &Ctx) -> Script {
    acolyte_waiting_room_body(ctx, Vec::new()).map(|_| ())
}

fn acolyte_waiting_room_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WaitingRoom,
        vec![
            Val::from("Personal Force, Acolyte Class"),
            Val::from(50),
            Val::from("Acolyte Waiting Room::OnStartArena"),
            Val::from(1),
            Val::from(1000),
            Val::from(10),
        ],
    )?;
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn acolyte_waiting_room_oninit(ctx: &Ctx) -> Script {
    acolyte_waiting_room_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn acolyte_waiting_room_onstartarena_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::WarpWaitingPc,
        vec![Val::from("force_5-1"), Val::from(99), Val::from(12)],
    )?;
    ctx.call(Function::EnableNpc, vec![Val::from("arena#aco")])?;
    ctx.call(Function::DisableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn acolyte_waiting_room_onstartarena(ctx: &Ctx) -> Script {
    acolyte_waiting_room_onstartarena_body(ctx, Vec::new()).map(|_| ())
}

fn acolyte_waiting_room_onstart_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableWaitingRoomEvent, vec![])?;
    return Err(Stop::End);
}

pub fn acolyte_waiting_room_onstart(ctx: &Ctx) -> Script {
    acolyte_waiting_room_onstart_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArenaAcoStep {
    Start,
    OnTouch,
}

fn arena_aco_run(ctx: &Ctx, mut step: ArenaAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaAcoStep::Start => {
                step = ArenaAcoStep::OnTouch;
                continue 'machine;
            }
            ArenaAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("allkill#aco::OnEnable")])?;
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ACOLYTE")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnStart")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco2::OnStart")])?;
                }
                ctx.call(Function::DisableNpc, vec![Val::from("arena#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_aco(ctx: &Ctx) -> Script {
    arena_aco_run(ctx, ArenaAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn arena_aco_ontouch(ctx: &Ctx) -> Script {
    arena_aco_run(ctx, ArenaAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ArenaAco1Step {
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

fn arena_aco1_run(ctx: &Ctx, mut step: ArenaAco1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ArenaAco1Step::Start => {
                step = ArenaAco1Step::OnStart;
                continue 'machine;
            }
            ArenaAco1Step::OnStart => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnEnable")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset01 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_01_02#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset02 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#aco::OnReset")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_02_03#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_03start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset03 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_03_04#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_04start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset04 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_04_05#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_05start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset05 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_05_06#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_06start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset06 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_06_07#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_07start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset07 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_07_08#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_08start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset08 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_08_09#aco")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("force_09start#aco")])?;
                return Err(Stop::End);
            }
            ArenaAco1Step::OnReset09 => {
                ctx.call(Function::EnableNpc, vec![Val::from("force_exit#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn arena_aco1(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::Start, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onstart(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnStart, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_01(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset01, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_02(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset02, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_03(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset03, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_04(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset04, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_05(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset05, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_06(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset06, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_07(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset07, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_08(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset08, Vec::new()).map(|_| ())
}

pub fn arena_aco1_onreset_09(ctx: &Ctx) -> Script {
    arena_aco1_run(ctx, ArenaAco1Step::OnReset09, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TroccoAco1Step {
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
    On05Start,
    On05End,
    On06Start,
    On07Start,
    On07End,
    On08Start,
    On09Start,
    On09End,
}

fn trocco_aco1_run(ctx: &Ctx, mut step: TroccoAco1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TroccoAco1Step::Start => {
                step = TroccoAco1Step::OnEnable;
                continue 'machine;
            }
            TroccoAco1Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.var("$@arn_1").set(ctx.call(Function::GetTimeTick, vec![Val::from(2)])?)?;
                return Err(Stop::End);
            }
            TroccoAco1Step::OnTimer2000 => {
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
            TroccoAco1Step::OnTimer3000 => {
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
            TroccoAco1Step::OnTimer4000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("force_5-1"), Val::from("You now have 8 minutes to complete this challenge. Enter the Left Entrance now to begin. Remember that you will be traveling clockwise as you clear the stages."), runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?])?;
                return Err(Stop::End);
            }
            TroccoAco1Step::OnTimer60000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer120000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer180000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer240000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer300000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer360000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer420000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("force_5-1")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
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
            TroccoAco1Step::OnTimer480000 => {
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
            TroccoAco1Step::OnTimer485000 => {
                step = TroccoAco1Step::OnTimer486000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer486000 => {
                step = TroccoAco1Step::OnTimer487000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer487000 => {
                step = TroccoAco1Step::OnTimer488000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer488000 => {
                step = TroccoAco1Step::OnTimer489000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer489000 => {
                step = TroccoAco1Step::OnTimer490000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer490000 => {
                step = TroccoAco1Step::OnTimer491000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer491000 => {
                step = TroccoAco1Step::OnTimer492000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer492000 => {
                step = TroccoAco1Step::OnTimer493000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer493000 => {
                step = TroccoAco1Step::OnTimer494000;
                continue 'machine;
            }
            TroccoAco1Step::OnTimer494000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(177), Val::from(86)],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::OnTimer495000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(177), Val::from(86)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnFailClearStage")])?;
                return Err(Stop::End);
            }
            TroccoAco1Step::OnTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TroccoAco1Step::OnFailClearStage => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("force_5-1"), Val::from("prt_are_in"), Val::from(177), Val::from(86)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::OnTimerOff")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Acolyte Waiting Room::OnStart")])?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On01Start => {
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
            TroccoAco1Step::On01End => {
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
            TroccoAco1Step::On02Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Kill at least 10 Drops!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On02End => {
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
            TroccoAco1Step::On03Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Terminate all Zombies!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On03End => {
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
            TroccoAco1Step::On04Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("As a courtesy, we'll restore some of your HP and SP. Now, head to the next room! Go go go!!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On05Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Obliterate all Orc Zombies!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On05End => {
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
            TroccoAco1Step::On06Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Reach the South Exit while dodging Hydras!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On07Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Kill all Archer Skeletons and Firelock Soldiers!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On07End => {
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
            TroccoAco1Step::On08Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Boss Battle. Please proceed to the room to the north!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On09Start => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Objective: Kill Zombie Prisoners, Skel Prisoners and Zombie Troops!"),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TroccoAco1Step::On09End => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("force_5-1"),
                        Val::from("Boss Defeated! The North Exit has opened. Thank you."),
                        runtime::op(&ctx.constant("BC_MAP")?, "|", &ctx.constant("BC_NPC")?)?,
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn trocco_aco1(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_onenable(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer2000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer3000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer4000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer4000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer60000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer120000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer180000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer180000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer240000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer300000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer360000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer360000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer420000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer420000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer480000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer480000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer485000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer485000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer486000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer486000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer487000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer487000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer488000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer488000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer489000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer489000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer490000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer490000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer491000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer491000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer492000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer492000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer493000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer493000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer494000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer494000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimer495000(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimer495000, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_ontimeroff(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnTimerOff, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_onfailclearstage(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::OnFailClearStage, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on01_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On01Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on01_end(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On01End, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on02_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On02Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on02_end(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On02End, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on03_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On03Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on03_end(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On03End, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on04_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On04Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on05_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On05Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on05_end(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On05End, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on06_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On06Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on07_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On07Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on07_end(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On07End, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on08_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On08Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on09_start(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On09Start, Vec::new()).map(|_| ())
}

pub fn trocco_aco1_on09_end(ctx: &Ctx) -> Script {
    trocco_aco1_run(ctx, TroccoAco1Step::On09End, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01startAcoStep {
    Start,
    OnTouch,
}

fn force_01start_aco_run(ctx: &Ctx, mut step: Force01startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01startAcoStep::Start => {
                step = Force01startAcoStep::OnTouch;
                continue 'machine;
            }
            Force01startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_01mob#aco::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_01start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01start_aco(ctx: &Ctx) -> Script {
    force_01start_aco_run(ctx, Force01startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_01start_aco_ontouch(ctx: &Ctx) -> Script {
    force_01start_aco_run(ctx, Force01startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force01mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_01mob_aco_run(ctx: &Ctx, mut step: Force01mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force01mobAcoStep::Start => {
                step = Force01mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force01mobAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On01_Start")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(22),
                        Val::from(29),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(1),
                        Val::from("force_01mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(29),
                        Val::from(29),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(1),
                        Val::from("force_01mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(29),
                        Val::from(22),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(1),
                        Val::from("force_01mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(22),
                        Val::from(22),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(1),
                        Val::from("force_01mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(25),
                        Val::from(25),
                        Val::from("Red Plant"),
                        Val::from(1078),
                        Val::from(1),
                        Val::from("force_01mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force01mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_01mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force01mobAcoStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_01mob#aco::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On01_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_01")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_01mob_aco(ctx: &Ctx) -> Script {
    force_01mob_aco_run(ctx, Force01mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_01mob_aco_onenable(ctx: &Ctx) -> Script {
    force_01mob_aco_run(ctx, Force01mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_01mob_aco_onreset(ctx: &Ctx) -> Script {
    force_01mob_aco_run(ctx, Force01mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_01mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_01mob_aco_run(ctx, Force01mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02startAcoStep {
    Start,
    OnTouch,
}

fn force_02start_aco_run(ctx: &Ctx, mut step: Force02startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02startAcoStep::Start => {
                step = Force02startAcoStep::OnTouch;
                continue 'machine;
            }
            Force02startAcoStep::OnTouch => {
                ctx.var("$@drop_gate").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_02mob#aco::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_02start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02start_aco(ctx: &Ctx) -> Script {
    force_02start_aco_run(ctx, Force02startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_02start_aco_ontouch(ctx: &Ctx) -> Script {
    force_02start_aco_run(ctx, Force02startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force02mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_02mob_aco_run(ctx: &Ctx, mut step: Force02mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force02mobAcoStep::Start => {
                step = Force02mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force02mobAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On02_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(22),
                        Val::from(69),
                        Val::from(29),
                        Val::from(130),
                        Val::from("Drops"),
                        Val::from(1572),
                        Val::from(20),
                        Val::from("force_02mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force02mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_02mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force02mobAcoStep::OnMyMobDead => {
                if (ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_02mob#aco::OnMyMobDead")],
                    )?
                    .number()?
                    < 11
                    && ctx.var("$@drop_gate").get()? == 0)
                {
                    ctx.var("$@drop_gate").set(Val::from(1))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On02_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_02")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_02mob_aco(ctx: &Ctx) -> Script {
    force_02mob_aco_run(ctx, Force02mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_02mob_aco_onenable(ctx: &Ctx) -> Script {
    force_02mob_aco_run(ctx, Force02mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_02mob_aco_onreset(ctx: &Ctx) -> Script {
    force_02mob_aco_run(ctx, Force02mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_02mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_02mob_aco_run(ctx, Force02mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03startAcoStep {
    Start,
    OnTouch,
}

fn force_03start_aco_run(ctx: &Ctx, mut step: Force03startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03startAcoStep::Start => {
                step = Force03startAcoStep::OnTouch;
                continue 'machine;
            }
            Force03startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_03mob#aco::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_03start#aco")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn force_03start_aco(ctx: &Ctx) -> Script {
    force_03start_aco_run(ctx, Force03startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_03start_aco_ontouch(ctx: &Ctx) -> Script {
    force_03start_aco_run(ctx, Force03startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force03mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_03mob_aco_run(ctx: &Ctx, mut step: Force03mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force03mobAcoStep::Start => {
                step = Force03mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force03mobAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On03_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(9),
                        Val::from(157),
                        Val::from(41),
                        Val::from(190),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(10),
                        Val::from("force_03mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force03mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_03mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force03mobAcoStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_03mob#aco::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On03_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_03")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_03mob_aco(ctx: &Ctx) -> Script {
    force_03mob_aco_run(ctx, Force03mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_03mob_aco_onenable(ctx: &Ctx) -> Script {
    force_03mob_aco_run(ctx, Force03mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_03mob_aco_onreset(ctx: &Ctx) -> Script {
    force_03mob_aco_run(ctx, Force03mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_03mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_03mob_aco_run(ctx, Force03mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force04startAcoStep {
    Start,
    OnTouch,
}

fn force_04start_aco_run(ctx: &Ctx, mut step: Force04startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04startAcoStep::Start => {
                step = Force04startAcoStep::OnTouch;
                continue 'machine;
            }
            Force04startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On04_Start")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_04")])?;
                ctx.call(Function::PercentHeal, vec![Val::from(50), Val::from(50)])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_04start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04start_aco(ctx: &Ctx) -> Script {
    force_04start_aco_run(ctx, Force04startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_04start_aco_ontouch(ctx: &Ctx) -> Script {
    force_04start_aco_run(ctx, Force04startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05startAcoStep {
    Start,
    OnTouch,
}

fn force_05start_aco_run(ctx: &Ctx, mut step: Force05startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05startAcoStep::Start => {
                step = Force05startAcoStep::OnTouch;
                continue 'machine;
            }
            Force05startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#aco::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_aco(ctx: &Ctx) -> Script {
    force_05start_aco_run(ctx, Force05startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_aco_ontouch(ctx: &Ctx) -> Script {
    force_05start_aco_run(ctx, Force05startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_05mob_aco_run(ctx: &Ctx, mut step: Force05mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mobAcoStep::Start => {
                step = Force05mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force05mobAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On05_Start")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(159),
                        Val::from(157),
                        Val::from(187),
                        Val::from(190),
                        Val::from("Orc Zombie"),
                        Val::from(1463),
                        Val::from(8),
                        Val::from("force_05mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_05mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mobAcoStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_05mob#aco::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On05_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_05")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_aco(ctx: &Ctx) -> Script {
    force_05mob_aco_run(ctx, Force05mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_aco_onenable(ctx: &Ctx) -> Script {
    force_05mob_aco_run(ctx, Force05mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_aco_onreset(ctx: &Ctx) -> Script {
    force_05mob_aco_run(ctx, Force05mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_aco_run(ctx, Force05mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06startAcoStep {
    Start,
    OnTouch,
}

fn force_06start_aco_run(ctx: &Ctx, mut step: Force06startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06startAcoStep::Start => {
                step = Force06startAcoStep::OnTouch;
                continue 'machine;
            }
            Force06startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#aco::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_06")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_aco(ctx: &Ctx) -> Script {
    force_06start_aco_run(ctx, Force06startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_aco_ontouch(ctx: &Ctx) -> Script {
    force_06start_aco_run(ctx, Force06startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_06mob_aco_run(ctx: &Ctx, mut step: Force06mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mobAcoStep::Start => {
                step = Force06mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force06mobAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On06_Start")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(124),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(121),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(173),
                        Val::from(116),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(175),
                        Val::from(113),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(175),
                        Val::from(110),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(106),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(173),
                        Val::from(106),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(176),
                        Val::from(106),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(179),
                        Val::from(106),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(174),
                        Val::from(100),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(174),
                        Val::from(96),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(174),
                        Val::from(92),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(174),
                        Val::from(88),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(80),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(173),
                        Val::from(80),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(176),
                        Val::from(80),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(179),
                        Val::from(80),
                        Val::from("Hydra"),
                        Val::from(1579),
                        Val::from(1),
                        Val::from("force_06mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_06mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mobAcoStep::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_aco(ctx: &Ctx) -> Script {
    force_06mob_aco_run(ctx, Force06mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_aco_onenable(ctx: &Ctx) -> Script {
    force_06mob_aco_run(ctx, Force06mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06mob_aco_onreset(ctx: &Ctx) -> Script {
    force_06mob_aco_run(ctx, Force06mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_aco_run(ctx, Force06mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07startAcoStep {
    Start,
    OnTouch,
}

fn force_07start_aco_run(ctx: &Ctx, mut step: Force07startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07startAcoStep::Start => {
                step = Force07startAcoStep::OnTouch;
                continue 'machine;
            }
            Force07startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#aco::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#aco")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn force_07start_aco(ctx: &Ctx) -> Script {
    force_07start_aco_run(ctx, Force07startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_aco_ontouch(ctx: &Ctx) -> Script {
    force_07start_aco_run(ctx, Force07startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_07mob_aco_run(ctx: &Ctx, mut step: Force07mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mobAcoStep::Start => {
                step = Force07mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force07mobAcoStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On07_Start")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(170),
                        Val::from(32),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_07mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(184),
                        Val::from(23),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_07mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(165),
                        Val::from(9),
                        Val::from("Archer Skeleton"),
                        Val::from(1420),
                        Val::from(1),
                        Val::from("force_07mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(184),
                        Val::from(15),
                        Val::from("Firelock Soldier"),
                        Val::from(1523),
                        Val::from(1),
                        Val::from("force_07mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(189),
                        Val::from(23),
                        Val::from("Firelock Soldier"),
                        Val::from(1523),
                        Val::from(1),
                        Val::from("force_07mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_07mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mobAcoStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_07mob#aco::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On07_End")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_07")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_aco(ctx: &Ctx) -> Script {
    force_07mob_aco_run(ctx, Force07mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_aco_onenable(ctx: &Ctx) -> Script {
    force_07mob_aco_run(ctx, Force07mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_aco_onreset(ctx: &Ctx) -> Script {
    force_07mob_aco_run(ctx, Force07mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_aco_run(ctx, Force07mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08startAcoStep {
    Start,
    OnTouch,
}

fn force_08start_aco_run(ctx: &Ctx, mut step: Force08startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08startAcoStep::Start => {
                step = Force08startAcoStep::OnTouch;
                continue 'machine;
            }
            Force08startAcoStep::OnTouch => {
                ctx.call(Function::PercentHeal, vec![Val::from(50), Val::from(50)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On08_Start")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_08")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_aco(ctx: &Ctx) -> Script {
    force_08start_aco_run(ctx, Force08startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_aco_ontouch(ctx: &Ctx) -> Script {
    force_08start_aco_run(ctx, Force08startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09startAcoStep {
    Start,
    OnTouch,
}

fn force_09start_aco_run(ctx: &Ctx, mut step: Force09startAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09startAcoStep::Start => {
                step = Force09startAcoStep::OnTouch;
                continue 'machine;
            }
            Force09startAcoStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On09_Start")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#aco::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#aco")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_aco(ctx: &Ctx) -> Script {
    force_09start_aco_run(ctx, Force09startAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_aco_ontouch(ctx: &Ctx) -> Script {
    force_09start_aco_run(ctx, Force09startAcoStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09mobAcoStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_aco_run(ctx: &Ctx, mut step: Force09mobAcoStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mobAcoStep::Start => {
                step = Force09mobAcoStep::OnEnable;
                continue 'machine;
            }
            Force09mobAcoStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Zombie Prisoner"),
                        Val::from(1480),
                        Val::from(1),
                        Val::from("force_09mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_5-1"),
                        ctx.call(Function::Rand, vec![Val::from(88), Val::from(111)])?,
                        ctx.call(Function::Rand, vec![Val::from(89), Val::from(110)])?,
                        Val::from("Skel Prisoner"),
                        Val::from(1479),
                        Val::from(1),
                        Val::from("force_09mob#aco::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        Val::from("force_5-1"),
                        Val::from(88),
                        Val::from(89),
                        Val::from(111),
                        Val::from(110),
                        Val::from("Zombie"),
                        Val::from(1394),
                        Val::from(4),
                        Val::from("force_09mob#aco::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force09mobAcoStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_5-1"), Val::from("force_09mob#aco::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mobAcoStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_5-1"), Val::from("force_09mob#aco::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Trocco#aco1::On09_End")])?;
                    ctx.var("$@arn_2").set(ctx.call(Function::GetTimeTick, vec![Val::from(2)])?)?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_aco1::OnReset_09")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_aco(ctx: &Ctx) -> Script {
    force_09mob_aco_run(ctx, Force09mobAcoStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_aco_onenable(ctx: &Ctx) -> Script {
    force_09mob_aco_run(ctx, Force09mobAcoStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_09mob_aco_onreset(ctx: &Ctx) -> Script {
    force_09mob_aco_run(ctx, Force09mobAcoStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_aco_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_aco_run(ctx, Force09mobAcoStep::OnMyMobDead, Vec::new()).map(|_| ())
}
