use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum RoomOfWaterDoorStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer30000,
    OnTimer50000,
    OnTimer60000,
    OnTimer61000,
    OnTimer62000,
    OnTimer63000,
}

fn room_of_water_door_run(ctx: &Ctx, mut step: RoomOfWaterDoorStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfWaterDoorStep::Start => {
                step = RoomOfWaterDoorStep::OnInit;
                continue 'machine;
            }
            RoomOfWaterDoorStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water#Door")])?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Water#Door")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water::OnDisable")])?;
                ctx.var(".mymobs").set(Val::from(5))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(114),
                        Val::from(169),
                        Val::from("Marine Sphere"),
                        Val::from(1142),
                        Val::from(1),
                        Val::from("Room of Water#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(112),
                        Val::from(169),
                        Val::from("Hydra"),
                        Val::from(1068),
                        Val::from(1),
                        Val::from("Room of Water#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(116),
                        Val::from(169),
                        Val::from("Hydra"),
                        Val::from(1068),
                        Val::from(1),
                        Val::from("Room of Water#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(114),
                        Val::from(171),
                        Val::from("Hydra"),
                        Val::from(1068),
                        Val::from(1),
                        Val::from("Room of Water#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(114),
                        Val::from(167),
                        Val::from("Hydra"),
                        Val::from(1068),
                        Val::from(1),
                        Val::from("Room of Water#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water#Door")])?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_wiz"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" has succeeded in eliminating the monsters.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
                    ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(116), Val::from(97)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water#Door::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("The guard monster has appeared. You have 1 minute."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer50000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("Time is up."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water#Door::OnDisable")])?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer61000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Water#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer62000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfWaterDoorStep::OnTimer63000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water#Failed")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Water#Door::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_water_door(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_oninit(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_onenable(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ondisable(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_onmymobdead(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer1000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer30000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer50000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer60000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer61000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer61000, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer62000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer62000, Vec::new()).map(|_| ())
}

pub fn room_of_water_door_ontimer63000(ctx: &Ctx) -> Script {
    room_of_water_door_run(ctx, RoomOfWaterDoorStep::OnTimer63000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfWaterFailedStep {
    Start,
    OnInit,
    OnTouch,
}

fn room_of_water_failed_run(ctx: &Ctx, mut step: RoomOfWaterFailedStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfWaterFailedStep::Start => {
                step = RoomOfWaterFailedStep::OnInit;
                continue 'machine;
            }
            RoomOfWaterFailedStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Water#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfWaterFailedStep::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has not succeeded.")),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(110)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_water_failed(ctx: &Ctx) -> Script {
    room_of_water_failed_run(ctx, RoomOfWaterFailedStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_water_failed_oninit(ctx: &Ctx) -> Script {
    room_of_water_failed_run(ctx, RoomOfWaterFailedStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_water_failed_ontouch(ctx: &Ctx) -> Script {
    room_of_water_failed_run(ctx, RoomOfWaterFailedStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfEarthStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer2000,
    OnTimer3000,
    OnTimer33000,
    OnTimer63000,
    OnTimer93000,
    OnTimer123000,
    OnTimer153000,
    OnTimer173000,
    OnTimer183000,
    OnTimer184000,
    OnTimer185000,
    OnTimer186000,
}

fn room_of_earth_run(ctx: &Ctx, mut step: RoomOfEarthStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfEarthStep::Start => {
                step = RoomOfEarthStep::OnInit;
                continue 'machine;
            }
            RoomOfEarthStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth")])?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Earth")])?;
                {
                    ctx.var(".mymobs").set(Val::from(10))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(120),
                            Val::from(102),
                            Val::from("Hode"),
                            Val::from(1127),
                            Val::from(1),
                            Val::from("Room of Earth::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(111),
                            Val::from(93),
                            Val::from("Savage"),
                            Val::from(1166),
                            Val::from(1),
                            Val::from("Room of Earth::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(127),
                            Val::from(86),
                            Val::from("Mantis"),
                            Val::from(1139),
                            Val::from(1),
                            Val::from("Room of Earth::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(111),
                        Val::from(102),
                        Val::from("Yoyo"),
                        Val::from(1057),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(120),
                        Val::from(102),
                        Val::from("Deniro"),
                        Val::from(1105),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(111),
                        Val::from(102),
                        Val::from("Caramel"),
                        Val::from(1103),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(120),
                        Val::from(93),
                        Val::from("Giearth"),
                        Val::from(1121),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(107),
                        Val::from(98),
                        Val::from("Bigfoot"),
                        Val::from(1060),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(124),
                        Val::from(98),
                        Val::from("Orc Warrior"),
                        Val::from(1023),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(104),
                        Val::from(86),
                        Val::from("Vitata"),
                        Val::from(1176),
                        Val::from(1),
                        Val::from("Room of Earth::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth")])?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_wiz"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" has succeeded in eliminating the monsters.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth#Door::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Earth Room; The job change test will now proceed."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("The time limit is 3 minutes."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Eliminate all monster within the time limit."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer33000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("2 minutes and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer63000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("2 minutes remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer93000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("1 minute and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer123000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("1 minute remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer153000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer173000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer183000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("Time is up."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth::OnDisable")])?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer184000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Earth#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer185000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthStep::OnTimer186000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth#Failed")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_earth(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_earth_oninit(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_earth_onenable(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ondisable(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room_of_earth_onmymobdead(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer1000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer2000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer3000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer33000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer33000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer63000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer63000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer93000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer93000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer123000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer123000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer153000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer153000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer173000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer173000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer183000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer183000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer184000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer185000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer185000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_ontimer186000(ctx: &Ctx) -> Script {
    room_of_earth_run(ctx, RoomOfEarthStep::OnTimer186000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfEarthDoorStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer30000,
    OnTimer50000,
    OnTimer60000,
    OnTimer61000,
    OnTimer62000,
    OnTimer63000,
}

fn room_of_earth_door_run(ctx: &Ctx, mut step: RoomOfEarthDoorStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfEarthDoorStep::Start => {
                step = RoomOfEarthDoorStep::OnInit;
                continue 'machine;
            }
            RoomOfEarthDoorStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth#Door")])?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Earth#Door")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth::OnDisable")])?;
                {
                    ctx.var(".mymobs").set(Val::from(7))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(116),
                            Val::from(97),
                            Val::from("Flora"),
                            Val::from(1118),
                            Val::from(1),
                            Val::from("Room of Earth#Door::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(114),
                        Val::from(95),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Room of Earth#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(118),
                        Val::from(95),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Room of Earth#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(114),
                        Val::from(99),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Room of Earth#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(118),
                        Val::from(99),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Room of Earth#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(116),
                        Val::from(94),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Room of Earth#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(116),
                        Val::from(100),
                        Val::from("Mandragora"),
                        Val::from(1020),
                        Val::from(1),
                        Val::from("Room of Earth#Door::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth#Door")])?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_wiz"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" has succeeded in eliminating the monster.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(100)])?;
                    ctx.call(Function::Warp, vec![Val::from("job_wiz"), Val::from(46), Val::from(99)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth#Door::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("The guard monster has appeared. You have 1 minute."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer50000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("End time."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth#Door::OnDisable")])?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer61000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Earth#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer62000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfEarthDoorStep::OnTimer63000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth#Failed")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Earth#Door::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_earth_door(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_oninit(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_onenable(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ondisable(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_onmymobdead(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer1000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer30000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer50000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer60000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer61000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer61000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer62000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer62000, Vec::new()).map(|_| ())
}

pub fn room_of_earth_door_ontimer63000(ctx: &Ctx) -> Script {
    room_of_earth_door_run(ctx, RoomOfEarthDoorStep::OnTimer63000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfEarthFailedStep {
    Start,
    OnInit,
    OnTouch,
}

fn room_of_earth_failed_run(ctx: &Ctx, mut step: RoomOfEarthFailedStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfEarthFailedStep::Start => {
                step = RoomOfEarthFailedStep::OnInit;
                continue 'machine;
            }
            RoomOfEarthFailedStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Earth#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfEarthFailedStep::OnTouch => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" has not succeeded.")),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(110)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_earth_failed(ctx: &Ctx) -> Script {
    room_of_earth_failed_run(ctx, RoomOfEarthFailedStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_earth_failed_oninit(ctx: &Ctx) -> Script {
    room_of_earth_failed_run(ctx, RoomOfEarthFailedStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_earth_failed_ontouch(ctx: &Ctx) -> Script {
    room_of_earth_failed_run(ctx, RoomOfEarthFailedStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfFireStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer2000,
    OnTimer3000,
    OnTimer33000,
    OnTimer63000,
    OnTimer93000,
    OnTimer123000,
    OnTimer153000,
    OnTimer173000,
    OnTimer183000,
    OnTimer184000,
    OnTimer185000,
    OnTimer186000,
}

fn room_of_fire_run(ctx: &Ctx, mut step: RoomOfFireStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfFireStep::Start => {
                step = RoomOfFireStep::OnInit;
                continue 'machine;
            }
            RoomOfFireStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire")])?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Fire")])?;
                {
                    ctx.var(".mymobs").set(Val::from(8))?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(58),
                            Val::from(110),
                            Val::from("Zerom"),
                            Val::from(1178),
                            Val::from(1),
                            Val::from("Room of Fire::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(54),
                            Val::from(89),
                            Val::from("Desert Wolf"),
                            Val::from(1106),
                            Val::from(1),
                            Val::from("Room of Fire::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(33),
                        Val::from(110),
                        Val::from("Goblin"),
                        Val::from(1123),
                        Val::from(1),
                        Val::from("Room of Fire::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(40),
                        Val::from(103),
                        Val::from("Scorpion"),
                        Val::from(1001),
                        Val::from(1),
                        Val::from("Room of Fire::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(51),
                        Val::from(103),
                        Val::from("Frilldora"),
                        Val::from(1119),
                        Val::from(1),
                        Val::from("Room of Fire::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(40),
                        Val::from(92),
                        Val::from("PecoPeco"),
                        Val::from(1019),
                        Val::from(1),
                        Val::from("Room of Fire::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(51),
                        Val::from(92),
                        Val::from("Elder Willow"),
                        Val::from(1033),
                        Val::from(1),
                        Val::from("Room of Fire::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(37),
                        Val::from(89),
                        Val::from("Metaller"),
                        Val::from(1058),
                        Val::from(1),
                        Val::from("Room of Fire::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire")])?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_wiz"),
                            (ctx.call(Function::StrCharInfo, vec![Val::from(0)])?
                                + Val::from(" has succeeded in eliminating the monsters.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire#Door::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Fire Room; The job change test shall now proceed."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer2000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Time limit is 3 minutes. We will now start the test."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Please eliminate all monsters within the time limit."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer33000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("2 minutes and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer63000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("2 minutes remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer93000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("1 minute and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer123000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("1 minute remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer153000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer173000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer183000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("Time is up."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire::OnDisable")])?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer184000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Fire#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer185000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireStep::OnTimer186000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire#Failed")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_fire(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_fire_oninit(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_fire_onenable(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ondisable(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room_of_fire_onmymobdead(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer1000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer2000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer2000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer3000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer33000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer33000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer63000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer63000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer93000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer93000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer123000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer123000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer153000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer153000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer173000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer173000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer183000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer183000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer184000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer184000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer185000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer185000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_ontimer186000(ctx: &Ctx) -> Script {
    room_of_fire_run(ctx, RoomOfFireStep::OnTimer186000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfFireDoorStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer1000,
    OnTimer30000,
    OnTimer60000,
    OnTimer90000,
    OnTimer110000,
    OnTimer120000,
    OnTimer121000,
    OnTimer122000,
    OnTimer123000,
}

fn room_of_fire_door_run(ctx: &Ctx, mut step: RoomOfFireDoorStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfFireDoorStep::Start => {
                step = RoomOfFireDoorStep::OnInit;
                continue 'machine;
            }
            RoomOfFireDoorStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire#Door")])?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Fire#Door")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire::OnDisable")])?;
                ctx.var(".mymobs").set(Val::from(3))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_wiz"),
                        Val::from(44),
                        Val::from(99),
                        Val::from("Greatest General"),
                        Val::from(1277),
                        Val::from(1),
                        Val::from("Room of Fire#Door::OnMyMobDead"),
                    ],
                )?;
                {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(43),
                            Val::from(99),
                            Val::from("Horong"),
                            Val::from(1129),
                            Val::from(1),
                            Val::from("Room of Fire#Door::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("job_wiz"),
                            Val::from(45),
                            Val::from(99),
                            Val::from("Horong"),
                            Val::from(1129),
                            Val::from(1),
                            Val::from("Room of Fire#Door::OnMyMobDead"),
                        ],
                    )?;
                }
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_wiz"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire#Door")])?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_wiz"),
                            ((Val::from("Congratulations, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                + Val::from(". You have passed the job change test.")),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.var("wiz_q").set(Val::from(7))?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(9017), Val::from(9018)])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire#Door::OnDisable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Test Helper#wiz::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("The guard monster has appeared. You have 2 minutes."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer30000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("1 minute and 30 seconds remaining."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("1 minute remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer90000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("30 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer110000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("10 seconds remaining."), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer120000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_wiz"), Val::from("Time is up."), ctx.constant("BC_MAP")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire#Door::OnDisable")])?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer121000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Room of Fire#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer122000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_wiz"),
                        Val::from("Next candidate, please enter."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            RoomOfFireDoorStep::OnTimer123000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire#Failed")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Room of Fire#Door::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn room_of_fire_door(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_oninit(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_onenable(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ondisable(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_onmymobdead(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer1000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer1000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer30000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer30000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer60000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer90000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer90000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer110000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer110000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer120000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer120000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer121000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer121000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer122000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer122000, Vec::new()).map(|_| ())
}

pub fn room_of_fire_door_ontimer123000(ctx: &Ctx) -> Script {
    room_of_fire_door_run(ctx, RoomOfFireDoorStep::OnTimer123000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum RoomOfFireFailedStep {
    Start,
    OnInit,
    OnTouch,
}

fn room_of_fire_failed_run(ctx: &Ctx, mut step: RoomOfFireFailedStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            RoomOfFireFailedStep::Start => {
                step = RoomOfFireFailedStep::OnInit;
                continue 'machine;
            }
            RoomOfFireFailedStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Room of Fire#Failed")])?;
                return Err(Stop::End);
            }
            RoomOfFireFailedStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(110)])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn room_of_fire_failed(ctx: &Ctx) -> Script {
    room_of_fire_failed_run(ctx, RoomOfFireFailedStep::Start, Vec::new()).map(|_| ())
}

pub fn room_of_fire_failed_oninit(ctx: &Ctx) -> Script {
    room_of_fire_failed_run(ctx, RoomOfFireFailedStep::OnInit, Vec::new()).map(|_| ())
}

pub fn room_of_fire_failed_ontouch(ctx: &Ctx) -> Script {
    room_of_fire_failed_run(ctx, RoomOfFireFailedStep::OnTouch, Vec::new()).map(|_| ())
}

fn test_helper_wiz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn test_helper_wiz(ctx: &Ctx) -> Script {
    test_helper_wiz_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Test Helper#wiz")])?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_oninit(ctx: &Ctx) -> Script {
    test_helper_wiz_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_onenable(ctx: &Ctx) -> Script {
    test_helper_wiz_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Test Helper#wiz")])?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_ondisable(ctx: &Ctx) -> Script {
    test_helper_wiz_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_ontimer2000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("job_wiz"),
            Val::from("Please return and complete the rest of the job change processes."),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_ontimer2000(ctx: &Ctx) -> Script {
    test_helper_wiz_ontimer2000_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_ontimer4000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("job_wiz"),
            Val::from("This is the end of the test. Next candidate, please stand by."),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_ontimer4000(ctx: &Ctx) -> Script {
    test_helper_wiz_ontimer4000_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_ontimer5000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::AreaWarp,
        vec![
            Val::from("job_wiz"),
            Val::from(33),
            Val::from(82),
            Val::from(57),
            Val::from(113),
            Val::from("gef_tower"),
            Val::from(110),
            Val::from(30),
        ],
    )?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_ontimer5000(ctx: &Ctx) -> Script {
    test_helper_wiz_ontimer5000_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_ontimer7000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("job_wiz"),
            Val::from("Next candidate, please enter."),
            ctx.constant("BC_MAP")?,
        ],
    )?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_ontimer7000(ctx: &Ctx) -> Script {
    test_helper_wiz_ontimer7000_body(ctx, Vec::new()).map(|_| ())
}

fn test_helper_wiz_ontimer9000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DoNpcEvent, vec![Val::from("Test Helper#wiz::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("Arena Assistant::OnStart")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn test_helper_wiz_ontimer9000(ctx: &Ctx) -> Script {
    test_helper_wiz_ontimer9000_body(ctx, Vec::new()).map(|_| ())
}
