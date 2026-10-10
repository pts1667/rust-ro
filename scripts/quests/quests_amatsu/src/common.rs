use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum DokebiEzStep {
    Start,
    OnInit,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

pub(super) fn dokebi_ez_run(ctx: &Ctx, mut step: DokebiEzStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_dokebi_tel = Val::from(0);
    'machine: loop {
        match step {
            DokebiEzStep::Start => {
                step = DokebiEzStep::OnInit;
                continue 'machine;
            }
            DokebiEzStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dokebi#ez")])?;
                return Err(Stop::End);
            }
            DokebiEzStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dokebi#ez")])?;
                ctx.var(".mymobs").set(Val::from(9))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(45),
                        Val::from(95),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(45),
                        Val::from(99),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(47),
                        Val::from(101),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(51),
                        Val::from(101),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(55),
                        Val::from(101),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(55),
                        Val::from(97),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(55),
                        Val::from(93),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(53),
                        Val::from(91),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(49),
                        Val::from(91),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#ez::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            DokebiEzStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("ama_test"), Val::from("Dokebi#ez::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            DokebiEzStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("ama_test"),
                            Val::from("I...I will be baaaack~~~~!!!"),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Coach#ama")])?;
                    ctx.var("event_momo").set(Val::from(2))?;
                } else {
                    l_dokebi_tel = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if l_dokebi_tel.clone() == 1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![Val::from("ama_test"), Val::from("!!IT'S A RAID!!!! RUN!!"), ctx.constant("BC_MAP")?],
                        )?;
                    } else if l_dokebi_tel.clone() == 2 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![Val::from("ama_test"), Val::from(" Ow, Ouch!!! "), ctx.constant("BC_MAP")?],
                        )?;
                    } else if l_dokebi_tel.clone() == 3 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("ama_test"),
                                Val::from(" But I didn't even do anything Baaad~!!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                    } else if l_dokebi_tel.clone() == 4 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![Val::from("ama_test"), Val::from(" I'm sorry~~ Waaaaah~~ "), ctx.constant("BC_MAP")?],
                        )?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum DokebiHdStep {
    Start,
    OnInit,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

pub(super) fn dokebi_hd_run(ctx: &Ctx, mut step: DokebiHdStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_dokebi_tel = Val::from(0);
    'machine: loop {
        match step {
            DokebiHdStep::Start => {
                step = DokebiHdStep::OnInit;
                continue 'machine;
            }
            DokebiHdStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dokebi#hd")])?;
                return Err(Stop::End);
            }
            DokebiHdStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dokebi#hd")])?;
                ctx.var(".mymobs").set(Val::from(9))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(45),
                        Val::from(95),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(45),
                        Val::from(99),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(47),
                        Val::from(101),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(51),
                        Val::from(101),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(55),
                        Val::from(101),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(55),
                        Val::from(97),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(55),
                        Val::from(93),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(53),
                        Val::from(91),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(49),
                        Val::from(91),
                        Val::from("Dokebi"),
                        Val::from(1110),
                        Val::from(1),
                        Val::from("Dokebi#hd::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            DokebiHdStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("ama_test"), Val::from("Dokebi#hd::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            DokebiHdStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("ama_test"),
                            Val::from(" I will be baaaack~~~~!!!"),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.call(Function::EnableNpc, vec![Val::from("Coach#ama")])?;
                    ctx.var("event_momo").set(Val::from(2))?;
                } else {
                    l_dokebi_tel = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                    if l_dokebi_tel.clone() == 1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("ama_test"),
                                Val::from("!! IT'S A RAID!!!! RUN!!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                    }
                    if l_dokebi_tel.clone() == 2 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![Val::from("ama_test"), Val::from(" Ow, Ouch!!! "), ctx.constant("BC_MAP")?],
                        )?;
                    }
                    if l_dokebi_tel.clone() == 3 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("ama_test"),
                                Val::from(" But I didn't even do anything Baaaaad~!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                    }
                    if l_dokebi_tel.clone() == 4 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("ama_test"),
                                Val::from(" I'm sorry~~! Waaaaah~~ "),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum CoachAmaStep {
    Start,
    OnInit,
    OnTouch,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum AmMutAmaStep {
    Start,
    OnInit,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

pub(super) fn am_mut_ama_run(ctx: &Ctx, mut step: AmMutAmaStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_am_mut_tel = Val::from(0);
    'machine: loop {
        match step {
            AmMutAmaStep::Start => {
                step = AmMutAmaStep::OnInit;
                continue 'machine;
            }
            AmMutAmaStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Am Mut#ama")])?;
                return Err(Stop::End);
            }
            AmMutAmaStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Am Mut#ama")])?;
                ctx.var(".mymobs").set(Val::from(3))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(35),
                        Val::from(102),
                        Val::from("Am Mut"),
                        Val::from(1301),
                        Val::from(1),
                        Val::from("Am Mut#ama::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(40),
                        Val::from(91),
                        Val::from("Am Mut"),
                        Val::from(1301),
                        Val::from(1),
                        Val::from("Am Mut#ama::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("ama_test"),
                        Val::from(56),
                        Val::from(108),
                        Val::from("Am Mut"),
                        Val::from(1301),
                        Val::from(1),
                        Val::from("Am Mut#ama::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            AmMutAmaStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("ama_test"), Val::from("Am Mut#ama::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            AmMutAmaStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 1 {
                    ctx.call(Function::EnableNpc, vec![Val::from("Coach#after")])?;
                } else {
                    l_am_mut_tel = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if l_am_mut_tel.clone() == 1 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("ama_test"),
                                Val::from(" Augh!! I, I made mistake...!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                    }
                    if l_am_mut_tel.clone() == 2 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("ama_test"),
                                Val::from(" Ugh...How could I lose?!... "),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                    }
                    if l_am_mut_tel.clone() == 3 {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![Val::from("ama_test"), Val::from(" Mommmmy~~!!!"), ctx.constant("BC_MAP")?],
                        )?;
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum TimerAmaStep {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnTimer1000,
    OnTimer181000,
    OnTimer301000,
    OnTimer361000,
    OnTimer361500,
    OnTimer362000,
    OnTimer362500,
}

pub(super) fn timer_ama_run(ctx: &Ctx, mut step: TimerAmaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TimerAmaStep::Start => {
                step = TimerAmaStep::OnInit;
                continue 'machine;
            }
            TimerAmaStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Timer#ama")])?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Timer#ama")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ama_test"),
                        Val::from(" The Timer has been activated. You have 6 minutes. Annihilate the monsters in time! "),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer181000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("ama_test"), Val::from(" 3 minutes left. "), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer301000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("ama_test"), Val::from(" 1 minute left. "), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer361000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("ama_test"),
                        Val::from("Beep- Beep- Beep- Time over."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer361500 => {
                ctx.call(Function::EnableNpc, vec![Val::from("backwarp#ama")])?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer362000 => {
                ctx.call(Function::DisableNpc, vec![Val::from("backwarp#ama")])?;
                return Err(Stop::End);
            }
            TimerAmaStep::OnTimer362500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Assistant#ama::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Timer#ama::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BackwarpAmaStep {
    Start,
    OnInit,
    OnTouch,
}
