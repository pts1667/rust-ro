use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum KeeperOfTheDoorAsnStep {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

pub(super) fn keeper_of_the_door_asn_run(ctx: &Ctx, mut step: KeeperOfTheDoorAsnStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KeeperOfTheDoorAsnStep::Start => {
                step = KeeperOfTheDoorAsnStep::OnInit;
                continue 'machine;
            }
            KeeperOfTheDoorAsnStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Keeper of the Door#ASN")])?;
                return Err(Stop::End);
            }
            KeeperOfTheDoorAsnStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Thomas#ASNTEST::OnDisable")])?;
                if ctx.var("assin_q").get()? == 3 {
                    ctx.var("assin_q").set(Val::from(3))?;
                } else {
                    ctx.var("assin_q").set(Val::from(4))?;
                }
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(87), Val::from(102)])?;
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("in_moc_16"), Val::from(16), Val::from(13), Val::from(1), Val::from(1)],
                )?;
                return Err(Stop::End);
            }
            KeeperOfTheDoorAsnStep::OnEnable => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("in_moc_16"),
                        Val::from("The door to the next room, at coordinates 87 137, has opened."),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("Keeper of the Door#ASN")])?;
                return Err(Stop::End);
            }
            KeeperOfTheDoorAsnStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Keeper of the Door#ASN")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Timestopper1Step {
    Start,
    OnEnable,
    OnTimer187000,
    OnDisable,
    OnMyMobDead,
}

pub(super) fn timestopper_1_run(ctx: &Ctx, mut step: Timestopper1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Timestopper1Step::Start => {
                step = Timestopper1Step::OnEnable;
                continue 'machine;
            }
            Timestopper1Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timestopper1Step::OnTimer187000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Thomas#ASNTEST::OnDisable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timestopper1Step::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Timestopper1Step::OnMyMobDead => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("in_moc_16"),
                        Val::from("Hey, what the hell was that?! I told you: No killing monsters!"),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("in_moc_16"),
                        Val::from("I'm bringing you back... *Sigh...*"),
                        ctx.constant("BC_MAP")?,
                    ],
                )?;
                ctx.var("assin_q").set(Val::from(3))?;
                ctx.call(Function::Warp, vec![Val::from("in_moc_16"), Val::from(87), Val::from(102)])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_moc_16"), Val::from("timestopper#1::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum GuildmasterAsn1Step {
    Start,
    OnTouch,
    OnCast,
}
