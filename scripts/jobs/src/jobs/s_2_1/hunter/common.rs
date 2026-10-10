use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum ManagerHntStep {
    Start,
    OnInit,
    OnEnable,
    OnMyMobDead,
    OnMyMobDead2,
    OnReset,
    OnDisable,
    OnTimer1000,
    OnTimer3000,
    OnTimer5000,
    OnTimer7000,
    OnTimer9000,
    OnTimer11000,
    OnTimer13000,
    OnTimer14000,
    OnTimer74000,
    OnTimer134000,
    OnTimer164000,
    OnTimer187000,
    OnTimer188000,
    OnTimer189000,
    OnTimer191000,
    OnTimer192000,
    OnTimer193000,
    OnTimer194000,
    OnTimer195000,
    OnTimer197000,
}

pub(super) fn manager_hnt_run(ctx: &Ctx, mut step: ManagerHntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ManagerHntStep::Start => {
                step = ManagerHntStep::OnInit;
                continue 'machine;
            }
            ManagerHntStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Manager#hnt")])?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Switch#hnt::OnDisable")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Manager#hnt")])?;
                ctx.var(".mymobs").set(Val::from(6))?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(67),
                        Val::from(80),
                        Val::from("Job Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(114),
                        Val::from(78),
                        Val::from("Job Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(89),
                        Val::from(127),
                        Val::from("Job Change Monster"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(73),
                        Val::from("Job Change Monster"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(125),
                        Val::from(70),
                        Val::from("Job Change Monster"),
                        Val::from(1016),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(90),
                        Val::from(92),
                        Val::from("Job Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(85),
                        Val::from(100),
                        Val::from("Job Test Monster"),
                        Val::from(1016),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(72),
                        Val::from(102),
                        Val::from("Job Test Monster"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(108),
                        Val::from(103),
                        Val::from("Job Test Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(88),
                        Val::from(127),
                        Val::from("Job Test Monster"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(125),
                        Val::from(69),
                        Val::from("Job Test Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(77),
                        Val::from(112),
                        Val::from("Job Tester Monster"),
                        Val::from(1016),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(106),
                        Val::from("Job Tester Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(73),
                        Val::from("Job Tester Monster"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(125),
                        Val::from(70),
                        Val::from("Job Tester Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(90),
                        Val::from(91),
                        Val::from("Job Tester Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(67),
                        Val::from(80),
                        Val::from("Hunter Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(77),
                        Val::from(112),
                        Val::from("Hunter Change Monster"),
                        Val::from(1016),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(106),
                        Val::from("Hunter Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(73),
                        Val::from("Hunter Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(125),
                        Val::from(70),
                        Val::from("Hunter Change Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(90),
                        Val::from(91),
                        Val::from("Job Transfer Monster"),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(85),
                        Val::from(100),
                        Val::from("Job Transfer Monster"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(72),
                        Val::from(102),
                        Val::from("Job Transfer Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(108),
                        Val::from(103),
                        Val::from("Job Transfer Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(77),
                        Val::from(112),
                        Val::from("Job Transfer Monster"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(112),
                        Val::from(139),
                        Val::from("Binnie"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(112),
                        Val::from(139),
                        Val::from("Darrel"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(112),
                        Val::from(139),
                        Val::from("Rex"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(112),
                        Val::from(139),
                        Val::from("Anselmo"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(90),
                        Val::from(91),
                        Val::from("Anolian"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(73),
                        Val::from("Monster Sample"),
                        Val::from(1002),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(53),
                        Val::from(106),
                        Val::from("Not Me"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(77),
                        Val::from(112),
                        Val::from("Help Me"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(72),
                        Val::from(102),
                        Val::from("Do Not Hit Me"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(108),
                        Val::from(103),
                        Val::from("Attack Speed 184"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Manager#hnt::OnMyMobDead2"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnMyMobDead => {
                ctx.var(".mymobs").set((ctx.var(".mymobs").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".mymobs").get()?.number()? < 3 {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            Val::from("Okay, good job... Now, find the switch in the center of the map!! Be careful of the traps!!"),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                    ctx.var("hntr_q").set(Val::from(14))?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("switch#hnt::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnDisable")])?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("job_hunte"),
                            Val::from("Okay~ You're almost there!!"),
                            ctx.constant("BC_MAP")?,
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            ManagerHntStep::OnMyMobDead2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from("!! You made a mistake...Please try again.")),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.var("hntr_q").set(Val::from(13))?;
                ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#hnt::OnStart")])?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnReset => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                step = ManagerHntStep::OnDisable;
                continue 'machine;
            }
            ManagerHntStep::OnDisable => {
                ctx.call(Function::KillMonster, vec![Val::from("job_hunte"), Val::from("All")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Manager#hnt")])?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer1000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from("The test shall now begin."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from("As mentioned before, only hunt the monsters labeled 'Job change monster'."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from("***** Be careful of the traps when hunting. *****"),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer7000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from("Once you hunt 4 'Job change monster' the switch in the center will begin to operate."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer9000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("job_hunte"), Val::from("When you activate the escape switch, exit the testing area through the warp portal in the 12 o'clock direction."), ctx.constant("BC_MAP")?])?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer11000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from("Everything must be completed within 3 minutes."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer13000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from("You will have 3 minutes from now on. You will be notified after each minute passes."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer14000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" ****** 3 minutes remaining. ****** "),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer74000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" ****** 2 minutes remaining. ****** "),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer134000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" ****** 1 minute remaining. ****** "),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer164000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" ****** 30 seconds remaining. ****** "),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer187000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" Test ends in 5 seconds..."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer188000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" Test ends in 4 seconds..."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer189000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" Test ends in 3 seconds..."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer191000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" Test ends in 2 seconds..."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer192000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" Test ends in 1 second."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer193000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![Val::from("job_hunte"), Val::from(" 0 "), ctx.constant("BC_MAP")?],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer194000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(" Time's up. Please try again."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer195000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_hunte"),
                        Val::from(50),
                        Val::from(64),
                        Val::from(129),
                        Val::from(143),
                        Val::from("job_hunte"),
                        Val::from(176),
                        Val::from(22),
                    ],
                )?;
                return Err(Stop::End);
            }
            ManagerHntStep::OnTimer197000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#hnt::OnStart")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SwitchHntStep {
    Start,
    OnTouch,
    OnDisable,
    OnEnable,
}

pub(super) fn switch_hnt_run(ctx: &Ctx, mut step: SwitchHntStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SwitchHntStep::Start => {
                step = SwitchHntStep::OnTouch;
                continue 'machine;
            }
            SwitchHntStep::OnTouch => {
                ctx.lines(args!["^3355FFThere are 3 buttons", "on the escape switch.^000000"])?;
                ctx.var("hntr_q").set(Val::from(15))?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Escape"), Val::from("Cancel"), Val::from("Re-test")])? {
                    1 => {
                        ctx.lines(args!["^3355FFThe Escape Warp Portal", "has now been activated.^000000"])?;
                        ctx.close_window()?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("job_hunte"),
                                Val::from(" !! Escape Warp Portal activation complete. !! "),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.call(Function::EnableNpc, vec![Val::from("exit#hnttest")])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args!["^3355FFCanceling", "Operation.^000000"])?;
                        ctx.close_window()?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("job_hunte"),
                                Val::from(" !! Operation has been cancelled. !! "),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("job_hunte"),
                                Val::from(" !! Cancellation warp activating... !! "),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.lines(args!["^3355FFYou will soon be", "returned to the", "waiting room.^000000"])?;
                        ctx.close_window()?;
                        ctx.var("hntr_q").set(Val::from(13))?;
                        ctx.call(Function::Warp, vec![Val::from("job_hunte"), Val::from(176), Val::from(22)])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Manager#hnt::OnReset")])?;
                        ctx.call(Function::DoNpcEvent, vec![Val::from("Waiting Room#hnt::OnStart")])?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                step = SwitchHntStep::OnDisable;
                continue 'machine;
            }
            SwitchHntStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("exit#hnttest")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Switch#hnt")])?;
                return Err(Stop::End);
            }
            SwitchHntStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Switch#hnt")])?;
                return Err(Stop::End);
            }
        }
    }
}
