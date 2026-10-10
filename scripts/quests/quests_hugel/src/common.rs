use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum UpperDrawerSecondStep {
    Start,
    OnInit,
    OnTouch,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum DraweropenerStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer4500,
}

pub(super) fn draweropener_run(ctx: &Ctx, mut step: DraweropenerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DraweropenerStep::Start => {
                step = DraweropenerStep::OnInit;
                continue 'machine;
            }
            DraweropenerStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#DrawerOpener")])?;
                return Err(Stop::End);
            }
            DraweropenerStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#DrawerOpener")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            DraweropenerStep::OnTimer1000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Upper Drawer#Second")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Upper Drawer#First")])?;
                return Err(Stop::End);
            }
            DraweropenerStep::OnTimer4500 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Upper Drawer#First")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Upper Drawer#Second")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#DrawerOpener")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum HumspawnerStep {
    Start,
    OnMonster,
    OnTimer60000,
    OnMonsterDead,
}

pub(super) fn humspawner_run(ctx: &Ctx, mut step: HumspawnerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HumspawnerStep::Start => {
                step = HumspawnerStep::OnMonster;
                continue 'machine;
            }
            HumspawnerStep::OnMonster => {
                if ctx.call(Function::Rand, vec![Val::from(0), Val::from(1)])?.is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("lhz_in01"),
                            Val::from("Intruder alert! Protect the research documents!"),
                            Val::from(1),
                            Val::from(16711680),
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("lhz_in01"),
                            Val::from("Intruder alert! Intruder alert!"),
                            Val::from(1),
                            Val::from(16711680),
                        ],
                    )?;
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("lhz_in01"),
                        Val::from(208),
                        Val::from(133),
                        Val::from("Guard"),
                        Val::from(1682),
                        Val::from(1),
                        Val::from("HuMSpawner::OnMonsterDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("lhz_in01"),
                        Val::from(217),
                        Val::from(135),
                        Val::from("Guard"),
                        Val::from(1682),
                        Val::from(1),
                        Val::from("HuMSpawner::OnMonsterDead"),
                    ],
                )?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            HumspawnerStep::OnTimer60000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("lhz_in01"), Val::from("HuMSpawner::OnMonsterDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            HumspawnerStep::OnMonsterDead => {
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum HiddenexplosionStep {
    Start,
    OnInit,
    OnTouch,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum HiddenactivatorHugelStep {
    Start,
    OnTouch,
    OnTimer180000,
    OnInit,
}

pub(super) fn hiddenactivator_hugel_run(ctx: &Ctx, mut step: HiddenactivatorHugelStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HiddenactivatorHugelStep::Start => {
                step = HiddenactivatorHugelStep::OnTouch;
                continue 'machine;
            }
            HiddenactivatorHugelStep::OnTouch => {
                if ctx.var("hg_ma1").get()? == 8 {
                    ctx.lines(args![
                        "^3355FFYou can hear two",
                        "people talking to",
                        "each other a short",
                        "distance away.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Suspicious Man",
                        args![
                            "You sure your research",
                            "is progressing? I haven't",
                            "seen anything that even",
                            "resembles real results!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Young Man",
                        args![
                            "I'm telling you, this isn't",
                            "like the last project. There's",
                            "no other way around it, it's",
                            "going to take some time.",
                            "You just have to be patient..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Suspicious Man",
                        args![
                            "Fine. That's fine by me.",
                            "But what about your fiancee?",
                            "I know she's near death's door:",
                            "can you really afford to wait",
                            "this long? Think about it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Young Man", args!["......", ".........", "............"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Suspicious Man",
                        args![
                            "Just finish this as",
                            "soon as you can. For",
                            "your fiancee's sake.",
                            "And your own sake.",
                            "Catch my meaning?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Young Man", args!["...I understand."])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou hear the sound of",
                        "footsteps steadily becoming",
                        "fainter and fainter. One of the",
                        "men must be walking away.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Young Man#Hu_Quest")])?;
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    return Err(Stop::End);
                } else if ((ctx.var("hg_ma1").get()? == 9 || ctx.var("hg_ma1").get()? == 10) || ctx.var("hg_ma1").get()? == 11) {
                    ctx.call(Function::EnableNpc, vec![Val::from("Young Man#Hu_Quest")])?;
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    return Err(Stop::End);
                }
                step = HiddenactivatorHugelStep::OnTimer180000;
                continue 'machine;
            }
            HiddenactivatorHugelStep::OnTimer180000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("Young Man#Hu_Quest")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            HiddenactivatorHugelStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Young Man#Hu_Quest")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum OdininitStep {
    Start,
    OnInit,
    OnTimer100000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Warpinside1Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

pub(super) fn warpinside_1_run(ctx: &Ctx, mut step: Warpinside1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warpinside1Step::Start => {
                step = Warpinside1Step::OnInit;
                continue 'machine;
            }
            Warpinside1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("warpinside#1")])?;
                return Err(Stop::End);
            }
            Warpinside1Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("warpinside#1")])?;
                return Err(Stop::End);
            }
            Warpinside1Step::OnTouch => {
                if (ctx.var("hg_odin").get()?.number()? > 19 && ctx.var("hg_odin").get()?.number()? < 23) {
                    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if subject1 == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#2::OnEnter")])?;
                    } else if subject1 == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#3::OnEnter")])?;
                    } else if subject1 == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#4::OnEnter")])?;
                    } else if subject1 == 4 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#5::OnEnter")])?;
                    }
                    ctx.call(Function::DisableNpc, vec![Val::from("warpinside#1")])?;
                    ctx.var("hg_odin").set(Val::from(21))?;
                    ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(36), Val::from(179)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Warpinside2Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

pub(super) fn warpinside_2_run(ctx: &Ctx, mut step: Warpinside2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warpinside2Step::Start => {
                step = Warpinside2Step::OnInit;
                continue 'machine;
            }
            Warpinside2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("warpinside#2")])?;
                return Err(Stop::End);
            }
            Warpinside2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("warpinside#2")])?;
                return Err(Stop::End);
            }
            Warpinside2Step::OnTouch => {
                if (ctx.var("hg_odin").get()?.number()? > 19 && ctx.var("hg_odin").get()?.number()? < 23) {
                    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if subject1 == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#1::OnEnter")])?;
                    } else if subject1 == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#3::OnEnter")])?;
                    } else if subject1 == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#4::OnEnter")])?;
                    } else if subject1 == 4 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#5::OnEnter")])?;
                    }
                    ctx.call(Function::DisableNpc, vec![Val::from("warpinside#2")])?;
                    ctx.var("hg_odin").set(Val::from(21))?;
                    ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(36), Val::from(179)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Warpinside3Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

pub(super) fn warpinside_3_run(ctx: &Ctx, mut step: Warpinside3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warpinside3Step::Start => {
                step = Warpinside3Step::OnInit;
                continue 'machine;
            }
            Warpinside3Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("warpinside#3")])?;
                return Err(Stop::End);
            }
            Warpinside3Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("warpinside#3")])?;
                return Err(Stop::End);
            }
            Warpinside3Step::OnTouch => {
                if (ctx.var("hg_odin").get()?.number()? > 19 && ctx.var("hg_odin").get()?.number()? < 23) {
                    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if subject1 == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#1::OnEnter")])?;
                    } else if subject1 == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#2::OnEnter")])?;
                    } else if subject1 == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#4::OnEnter")])?;
                    } else if subject1 == 4 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#5::OnEnter")])?;
                    }
                    ctx.call(Function::DisableNpc, vec![Val::from("warpinside#3")])?;
                    ctx.var("hg_odin").set(Val::from(21))?;
                    ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(36), Val::from(179)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Warpinside4Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

pub(super) fn warpinside_4_run(ctx: &Ctx, mut step: Warpinside4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warpinside4Step::Start => {
                step = Warpinside4Step::OnInit;
                continue 'machine;
            }
            Warpinside4Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("warpinside#4")])?;
                return Err(Stop::End);
            }
            Warpinside4Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("warpinside#4")])?;
                return Err(Stop::End);
            }
            Warpinside4Step::OnTouch => {
                if (ctx.var("hg_odin").get()?.number()? > 19 && ctx.var("hg_odin").get()?.number()? < 23) {
                    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if subject1 == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#1::OnEnter")])?;
                    } else if subject1 == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#2::OnEnter")])?;
                    } else if subject1 == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#3::OnEnter")])?;
                    } else if subject1 == 4 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#5::OnEnter")])?;
                    }
                    ctx.call(Function::DisableNpc, vec![Val::from("warpinside#4")])?;
                    ctx.var("hg_odin").set(Val::from(21))?;
                    ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(36), Val::from(179)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Warpinside5Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

pub(super) fn warpinside_5_run(ctx: &Ctx, mut step: Warpinside5Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Warpinside5Step::Start => {
                step = Warpinside5Step::OnInit;
                continue 'machine;
            }
            Warpinside5Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("warpinside#5")])?;
                return Err(Stop::End);
            }
            Warpinside5Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("warpinside#5")])?;
                return Err(Stop::End);
            }
            Warpinside5Step::OnTouch => {
                if (ctx.var("hg_odin").get()?.number()? > 19 && ctx.var("hg_odin").get()?.number()? < 23) {
                    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if subject1 == 1 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#1::OnEnter")])?;
                    } else if subject1 == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#2::OnEnter")])?;
                    } else if subject1 == 3 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#3::OnEnter")])?;
                    } else if subject1 == 4 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("warpinside#4::OnEnter")])?;
                    }
                    ctx.call(Function::DisableNpc, vec![Val::from("warpinside#5")])?;
                    ctx.var("hg_odin").set(Val::from(21))?;
                    ctx.call(Function::Warp, vec![Val::from("que_hugel"), Val::from(36), Val::from(179)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}
