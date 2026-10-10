use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum TimerSneakStep {
    Start,
    OnTouch,
    OnInit,
    OnEnter,
    OnTimer180000,
    OnTimer190000,
}

pub(super) fn timer_sneak_run(ctx: &Ctx, mut step: TimerSneakStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerSneakStep::Start => {
                step = TimerSneakStep::OnTouch;
                continue 'machine;
            }
            TimerSneakStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("lhz_in01"), Val::from(191), Val::from(49)])?;
                return Err(Stop::End);
            }
            TimerSneakStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Timer_Sneak")])?;
                return Err(Stop::End);
            }
            TimerSneakStep::OnEnter => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerSneakStep::OnTimer180000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Timer_Sneak")])?;
                return Err(Stop::End);
            }
            TimerSneakStep::OnTimer190000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Timer_Sneak")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Bully1Step {
    Start,
    OnInit,
    OnEnter,
    OnReset,
    OnMyMobDead,
    OnTimer120000,
}

pub(super) fn bully1_run(ctx: &Ctx, mut step: Bully1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Bully1Step::Start => {
                step = Bully1Step::OnInit;
                continue 'machine;
            }
            Bully1Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#bully1")])?;
                return Err(Stop::End);
            }
            Bully1Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#bully1")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(96),
                        Val::from(53),
                        Val::from("Thug"),
                        Val::from(1592),
                        Val::from(1),
                        Val::from("#bully1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(87),
                        Val::from(47),
                        Val::from("Thug"),
                        Val::from(1592),
                        Val::from(1),
                        Val::from("#bully1::OnMyMobDead"),
                    ],
                )?;
                ctx.var(".bullymobs").set(Val::from(2))?;
                return Err(Stop::End);
            }
            Bully1Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("airplane_01"), Val::from("#bully1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Bully1Step::OnMyMobDead => {
                ctx.var(".bullymobs").set((ctx.var(".bullymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".bullymobs").get()?.number()? < 1 {
                    ctx.lines_as(
                        "Bully",
                        args![
                            "Damn it! N-next time...",
                            "Next time we'll g-get rid",
                            "of those damned packages!",
                            "^333333*Cough cough*^000000 For now, we",
                            "retreat and fight another day!"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#bully1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnEnter")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Bully1Step::OnTimer120000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#bully1::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#bully1::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnEnter")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Bully2Step {
    Start,
    OnInit,
    OnEnter,
    OnReset,
    OnMyMobDead,
    OnTimer120000,
}

pub(super) fn bully2_run(ctx: &Ctx, mut step: Bully2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Bully2Step::Start => {
                step = Bully2Step::OnInit;
                continue 'machine;
            }
            Bully2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#bully2")])?;
                return Err(Stop::End);
            }
            Bully2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#bully2")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(96),
                        Val::from(53),
                        Val::from("Thug"),
                        Val::from(1592),
                        Val::from(1),
                        Val::from("#bully2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(87),
                        Val::from(47),
                        Val::from("Thug"),
                        Val::from(1592),
                        Val::from(1),
                        Val::from("#bully2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("airplane_01"),
                        Val::from(97),
                        Val::from(43),
                        Val::from("Thug"),
                        Val::from(1592),
                        Val::from(1),
                        Val::from("#bully2::OnMyMobDead"),
                    ],
                )?;
                ctx.var(".bullymobs").set(Val::from(3))?;
                return Err(Stop::End);
            }
            Bully2Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("airplane_01"), Val::from("#bully2::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Bully2Step::OnMyMobDead => {
                ctx.var(".bullymobs").set((ctx.var(".bullymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".bullymobs").get()?.number()? < 1 {
                    ctx.lines(args![
                        "^3355FFHey-- there's a slit",
                        "in the wrapping on one",
                        "of the packages. It was",
                        "probably ripped a little",
                        "while you were fighting.^000000"
                    ])?;
                    ctx.close_window()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#bully2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#packidentity::OnEnter")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            Bully2Step::OnTimer120000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#bully2::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#bully2::OnInit")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#packidentity::OnEnter")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum PackidentityStep {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
    OnTimer120000,
}

pub(super) fn packidentity_run(ctx: &Ctx, mut step: PackidentityStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            PackidentityStep::Start => {
                return Err(Stop::End);
            }
            PackidentityStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#packidentity")])?;
                return Err(Stop::End);
            }
            PackidentityStep::OnEnter => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#packidentity")])?;
                return Err(Stop::End);
            }
            PackidentityStep::OnTouch => {
                if ctx.var("lhz_rekenber").get()? == 19 {
                    ctx.lines(args![
                        "^3355FFYou peek through the",
                        "slit in the wrapping",
                        "that is covering one",
                        "of the packages.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args![
                            "Oh... my God!",
                            "These are... These are",
                            "weapons of mass destruction.",
                            "There's even parts for assembling guardians, the kinds that usually",
                            "defend those Guild Castles..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#packidentity::OnInit")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                    return Err(Stop::End);
                }
                step = PackidentityStep::OnTimer120000;
                continue 'machine;
            }
            PackidentityStep::OnTimer120000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Man#Lyozien::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#packidentity::OnInit")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
