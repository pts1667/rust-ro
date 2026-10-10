use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum BroadcastMode1Step {
    Start,
    OnInstanceInit,
    OnTimer15000,
    OnTimer60000,
    OnTimer120000,
}

pub(super) fn broadcast_mode1_run(ctx: &Ctx, mut step: BroadcastMode1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BroadcastMode1Step::Start => {
                return Err(Stop::End);
            }
            BroadcastMode1Step::OnInstanceInit => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            BroadcastMode1Step::OnTimer15000 => {
                step = BroadcastMode1Step::OnTimer60000;
                continue 'machine;
            }
            BroadcastMode1Step::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@tower")])?,
                        Val::from("Notice : In any abnormal situation where you defeat a monster, you can't advance to the next level!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xff0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BroadcastMode1Step::OnTimer120000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("1@tower")])?,
                        Val::from("Notice : In any abnormal situation where you defeat a monster, you can't advance to the next level!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xff0000"),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum LucidCrystal102Step {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
}

pub(super) fn lucid_crystal_102_run(ctx: &Ctx, mut step: LucidCrystal102Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            LucidCrystal102Step::Start => {
                if ctx.var("in_102tower").get()?.number()? < 10 {
                    ctx.mes(
                        "^0000ffA mysterious voice echoes through the room as you touch the lucid crystal radiating a strong light.^000000",
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Voice",
                        args!["Welcome to my place, intruders. I've had fun watching you endure all the difficulties I've set before you."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mysterious Voice", args!["Unfortunately, the time has come to end the show."])?;
                    ctx.next()?;
                    ctx.lines_as("Mysterious Voice", args!["It's still too early to celebrate your victory against my right-hand man Knothen because he isn't completely destroyed!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Voice",
                        args!["I shall resurrect him for your next show. Defeat him again, and then I'll gladly accept your challenge."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Voice",
                        args![
                            "Good-bye for now.",
                            "Once again, I had such a great time, humans. I look forward to seeing you again."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.mes("^0000ffAs soon as the voice stopped talking, an irresistible force lifted and moved you somewhere else.")?;
                    ctx.close_window()?;
                    ctx.var("in_102tower").set(Val::from(10))?;
                    ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(223), Val::from(36)])?;
                } else {
                    ctx.mes("^0000ffThe radiating crystal piece seems to beckon you, just like last time.^000000")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mysterious Voice",
                        args!["I must have underestimated you... I didn't expect to see you again."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mysterious Voice", args!["I can smell your sweat, hear the gasps of your exhaustion in combat. Ah~ The human desire for victory always excites me."])?;
                    ctx.next()?;
                    ctx.lines_as("Mysterious Voice", args!["I now allow you to come receive an audience from me. Come to me, to Naght Sieger the Hegemon-King of the Darkness!"])?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::Warp,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("6@tower")])?,
                            Val::from(32),
                            Val::from(12),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            LucidCrystal102Step::OnInstanceInit => {
                step = LucidCrystal102Step::OnDisable;
                continue 'machine;
            }
            LucidCrystal102Step::OnDisable => {
                ctx.call(
                    Function::DisableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Lucid Crystal#102")])?],
                )?;
                return Err(Stop::End);
            }
            LucidCrystal102Step::OnEnable => {
                ctx.call(
                    Function::EnableNpc,
                    vec![ctx.call(Function::InstanceNpcName, vec![Val::from("Lucid Crystal#102")])?],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("#102Effect1")])? + Val::from("::OnEnable"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
