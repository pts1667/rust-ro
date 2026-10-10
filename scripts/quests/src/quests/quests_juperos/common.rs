use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum RestrictedAreaJupeStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer5000,
    OnTimer7000,
    OnTimer9000,
    OnTimer9001,
    OnTimer23000,
    OnTimer46000,
    OnTimer69000,
    OnTimer92000,
    OnTimer115000,
    OnTimer161000,
    OnTimer184000,
    OnTimer207000,
    OnTimer230000,
    OnTimer253000,
    OnTimer276000,
    OnTimer299000,
    OnTimer322000,
    OnTimer345000,
    OnTimer368000,
    OnTimer391000,
    OnTimer414000,
    OnTimer460000,
    OnTimer483000,
    OnTimer506000,
    OnTimer529000,
    OnTimer552000,
    OnTimer556000,
    OnTimer561000,
    OnTimer598000,
    OnTimer600000,
    OnTimer603000,
    OnTimer621000,
    OnTimer1200000,
    OnTouch,
}

pub(super) fn restricted_area_jupe_run(ctx: &Ctx, mut step: RestrictedAreaJupeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RestrictedAreaJupeStep::Start => {
                step = RestrictedAreaJupeStep::OnInit;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Restricted Area#jupe")])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Restricted Area#jupe")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("juperos_02"),
                        Val::from("Vroom! Vroom!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xE559A2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("juperos_02"),
                        Val::from("Attention, visitors."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xE559A2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer9000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("juperos_02"), Val::from("You are allowed to enter the next zone for a short period of time. Please use the portal in the center of the map."), ctx.constant("BC_MAP")?, Val::from("0xE559A2")])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer9001 => {
                step = RestrictedAreaJupeStep::OnTimer23000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer23000 => {
                step = RestrictedAreaJupeStep::OnTimer46000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer46000 => {
                step = RestrictedAreaJupeStep::OnTimer69000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer69000 => {
                step = RestrictedAreaJupeStep::OnTimer92000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer92000 => {
                step = RestrictedAreaJupeStep::OnTimer115000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer115000 => {
                step = RestrictedAreaJupeStep::OnTimer161000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer161000 => {
                step = RestrictedAreaJupeStep::OnTimer184000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer184000 => {
                step = RestrictedAreaJupeStep::OnTimer207000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer207000 => {
                step = RestrictedAreaJupeStep::OnTimer230000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer230000 => {
                step = RestrictedAreaJupeStep::OnTimer253000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer253000 => {
                step = RestrictedAreaJupeStep::OnTimer276000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer276000 => {
                step = RestrictedAreaJupeStep::OnTimer299000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer299000 => {
                step = RestrictedAreaJupeStep::OnTimer322000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer322000 => {
                step = RestrictedAreaJupeStep::OnTimer345000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer345000 => {
                step = RestrictedAreaJupeStep::OnTimer368000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer368000 => {
                step = RestrictedAreaJupeStep::OnTimer391000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer391000 => {
                step = RestrictedAreaJupeStep::OnTimer414000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer414000 => {
                step = RestrictedAreaJupeStep::OnTimer460000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer460000 => {
                step = RestrictedAreaJupeStep::OnTimer483000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer483000 => {
                step = RestrictedAreaJupeStep::OnTimer506000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer506000 => {
                step = RestrictedAreaJupeStep::OnTimer529000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer529000 => {
                step = RestrictedAreaJupeStep::OnTimer552000;
                continue 'machine;
            }
            RestrictedAreaJupeStep::OnTimer552000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer556000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("juperos_02"),
                        Val::from("Attention, visitors. The gate to the next zone will close shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xE559A2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer561000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("juperos_02"),
                        Val::from("1 minute remaining until Gate Closure. "),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xE559A2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer598000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BIG_PORTAL")?])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer600000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("juperos_02"),
                        Val::from("The gate is being closed..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xE559A2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer603000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("juperos_02"),
                        Val::from(" Switches will reactivate shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xE559A2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer621000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Restricted Area#jupe")])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTimer1200000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("3F Gate Switch#jupe::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RestrictedAreaJupeStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("jupe_gate"), Val::from(50), Val::from(167)])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ElevatorGuard1UfeStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer1200,
    OnTimer1400,
    OnTimer1600,
    OnTimer1800,
    OnTimer2000,
    OnTimer2200,
    OnTimer2400,
    OnTimer2600,
    OnTimer120000,
    OnTimer120005,
    OnMyMobDead,
}

pub(super) fn elevator_guard1_ufe_run(ctx: &Ctx, mut step: ElevatorGuard1UfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ElevatorGuard1UfeStep::Start => {
                step = ElevatorGuard1UfeStep::OnInit;
                continue 'machine;
            }
            ElevatorGuard1UfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Elevator Guard1#ufe")])?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnEnable => {
                ctx.var(".mymobs").set(Val::from(9))?;
                ctx.call(Function::EnableNpc, vec![Val::from("Elevator Guard1#ufe")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer1000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(44),
                        Val::from(99),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer1200 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(55),
                        Val::from(99),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer1400 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(45),
                        Val::from(84),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer1600 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(54),
                        Val::from(84),
                        Val::from("Guard"),
                        Val::from(1675),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer1800 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(45),
                        Val::from(99),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer2000 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(54),
                        Val::from(99),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer2200 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(48),
                        Val::from(84),
                        Val::from("Guard"),
                        Val::from(1669),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer2400 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(52),
                        Val::from(84),
                        Val::from("Guard"),
                        Val::from(1683),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer2600 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from(50),
                        Val::from(84),
                        Val::from("Chief Guard"),
                        Val::from(1684),
                        Val::from(1),
                        Val::from("Elevator Guard1#ufe::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer120000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele_r"),
                        Val::from("It is disappointing to see that you are too weak to even defeat a hallucination..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("Switch#ufe")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Switch On#ufe")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Annihilation#ufe::OnEnable")])?;
                ctx.var("$@jupeelevatorinuse").set(Val::from(0))?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnTimer120005 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("jupe_ele_r"), Val::from("Elevator Guard1#ufe::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ElevatorGuard1UfeStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Elevator Safety#ufe::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum TimeoutUfeStep {
    Start,
    OnEnable,
    OnDisable,
    OnTimer59000,
    OnTimer120000,
    OnTimer122000,
    OnTimer125000,
    OnTimer127000,
    OnTimer129000,
    OnTimer131000,
    OnTimer133000,
    OnTimer134000,
    OnTimer135000,
    OnTimer142000,
}

pub(super) fn timeout_ufe_run(ctx: &Ctx, mut step: TimeoutUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimeoutUfeStep::Start => {
                step = TimeoutUfeStep::OnEnable;
                continue 'machine;
            }
            TimeoutUfeStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer59000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("jupe_ele")])? == 0 {
                    ctx.call(Function::KillMonster, vec![Val::from("jupe_ele"), Val::from("All")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-1#ufe::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-2#ufe::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-3#ufe::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-4#ufe::OnDisable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                    ctx.var("$@jupeelevatorinuse2").set(Val::from(0))?;
                    ctx.var("$@jupeelevatorinuse").set(Val::from(0))?;
                    ctx.call(Function::DisableNpc, vec![Val::from("Switch On#ufe")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Switch#ufe")])?;
                }
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer120000 => {
                ctx.call(Function::KillMonster, vec![Val::from("jupe_ele"), Val::from("All")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-1#ufe::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-2#ufe::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-3#ufe::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Guard-4#ufe::OnDisable")])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("In the end, you can't even overcome your inner fear..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer122000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("You have never encountered your inner fears, have you?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer125000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Did you expect this would be the end of the hallucination?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer127000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("What if the voice you're hearing is also a hallucination?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer129000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("What if you're just dreaming all of this?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer131000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("What if the existence of this city is a lie?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer133000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Are you even real?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFF0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer134000 => {
                step = TimeoutUfeStep::OnTimer135000;
                continue 'machine;
            }
            TimeoutUfeStep::OnTimer135000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("jupe_ele"), Val::from("jupe_gate"), Val::from(49), Val::from(138)],
                )?;
                return Err(Stop::End);
            }
            TimeoutUfeStep::OnTimer142000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("GuardEnd#ufe")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("4F Enter#ufe")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("TimeOut#ufe::OnDisable")])?;
                ctx.var("$@jupeelevatorinuse2").set(Val::from(0))?;
                ctx.var("$@jupeelevatorinuse").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Switch On#ufe")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Switch#ufe")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum GuardendUfeStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer2000,
    OnTimer5000,
    OnTimer8000,
    OnTimer11000,
    OnTimer12000,
    OnTimer22000,
    OnTimer24000,
    OnTimer25000,
    OnTimer26000,
}

pub(super) fn guardend_ufe_run(ctx: &Ctx, mut step: GuardendUfeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GuardendUfeStep::Start => {
                step = GuardendUfeStep::OnInit;
                continue 'machine;
            }
            GuardendUfeStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("GuardEnd#ufe")])?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("GuardEnd#ufe")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("TimeOut#ufe::OnDisable")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("GuardEnd#ufe")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("I am not going to tell you anything."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("My city, my people are now but a memory."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Everything was a mistake. We were not supposed to be here."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("Is this a place where humans are forbidden?"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("You want to know, don't you? Go ahead... Go deeper."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("4F Enter#ufe")])?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer22000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("jupe_ele"),
                        Val::from("It's not real anyway. All of it's safe, it can't hurt you..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x66FF00"),
                    ],
                )?;
                ctx.call(Function::SoundEffectAll, vec![Val::from("earth_quake.wav"), Val::from(0)])?;
                ctx.call(Function::DisableNpc, vec![Val::from("4F Enter#ufe")])?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer24000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("jupe_ele"), Val::from("jupe_core"), Val::from(150), Val::from(286)],
                )?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer25000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("jupe_ele"), Val::from("jupe_core"), Val::from(151), Val::from(286)],
                )?;
                return Err(Stop::End);
            }
            GuardendUfeStep::OnTimer26000 => {
                ctx.var("$@jupeelevatorinuse2").set(Val::from(0))?;
                ctx.var("$@jupeelevatorinuse").set(Val::from(0))?;
                ctx.call(Function::DisableNpc, vec![Val::from("Switch On#ufe")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Switch#ufe")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("GuardEnd#ufe::OnDisable")])?;
                return Ok(Val::from(0));
            }
        }
    }
}
