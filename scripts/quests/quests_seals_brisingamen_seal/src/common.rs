use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum Brisinsold2Step {
    Start,
    OnInit,
    OnSold2On,
    OnTimer420000,
    OnTimer480000,
    OnTimer540000,
    OnTimer542000,
    OnTimer550000,
    OnTimer550500,
}

pub(super) fn brisinsold2_run(ctx: &Ctx, mut step: Brisinsold2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Brisinsold2Step::Start => {
                step = Brisinsold2Step::OnInit;
                continue 'machine;
            }
            Brisinsold2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Soldier#2_brising")])?;
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("que_god02"),
                        Val::from(15),
                        Val::from(125),
                        Val::from(185),
                        Val::from(131),
                        Val::from("geffen"),
                        Val::from(120),
                        Val::from(100),
                    ],
                )?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnSold2On => {
                ctx.call(Function::EnableNpc, vec![Val::from("Soldier#2_brising")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnTimer420000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("Lowen's spirit is beginning to weaken..."),
                        Val::from(0),
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnTimer480000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("Your presense in her past can't be maintained for much longer..."),
                        Val::from(0),
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnTimer540000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("que_god02"),
                        Val::from("Lowen's spirit can't keep you in the past any longer."),
                        Val::from(0),
                        Val::from(7396315),
                    ],
                )?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnTimer542000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("que_god02"),
                        Val::from(15),
                        Val::from(125),
                        Val::from(185),
                        Val::from(131),
                        Val::from("geffen"),
                        Val::from(120),
                        Val::from(100),
                    ],
                )?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnTimer550000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#brisinsummon::OnReset")])?;
                return Err(Stop::End);
            }
            Brisinsold2Step::OnTimer550500 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BrisinsummonStep {
    Start,
    OnInit,
    OnDoppel1On,
    OnDoppel1Off,
    OnDoppel2On,
    OnDoppel2Off,
    OnKnight1On,
    OnKnight2On,
    OnKnight3On,
    OnKnight1Off,
    OnKnight2Off,
    OnKnight3Off,
    OnLowenOn,
    OnLowenOff,
    OnHermiteOff,
    OnLowen2Off,
    OnSummon,
    OnMobDeath,
    OnReset,
}

pub(super) fn brisinsummon_run(ctx: &Ctx, mut step: BrisinsummonStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BrisinsummonStep::Start => {
                step = BrisinsummonStep::OnInit;
                continue 'machine;
            }
            BrisinsummonStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#lowen")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Valkyrie#1")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hermite")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnDoppel1On => {
                ctx.call(Function::EnableNpc, vec![Val::from("#doppelganger1")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnDoppel1Off => {
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger1")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnDoppel2On => {
                ctx.call(Function::EnableNpc, vec![Val::from("#doppelganger2")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnDoppel2Off => {
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger2")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnKnight1On => {
                ctx.call(Function::EnableNpc, vec![Val::from("#knight1")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnKnight2On => {
                ctx.call(Function::EnableNpc, vec![Val::from("#knight2")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnKnight3On => {
                ctx.call(Function::EnableNpc, vec![Val::from("#knight3")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnKnight1Off => {
                ctx.call(Function::DisableNpc, vec![Val::from("#knight1")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnKnight2Off => {
                ctx.call(Function::DisableNpc, vec![Val::from("#knight2")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnKnight3Off => {
                ctx.call(Function::DisableNpc, vec![Val::from("#knight3")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnLowenOn => {
                ctx.call(Function::EnableNpc, vec![Val::from("#lowen")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnLowenOff => {
                ctx.call(Function::DisableNpc, vec![Val::from("#lowen")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnHermiteOff => {
                ctx.call(Function::DisableNpc, vec![Val::from("#hermite")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnLowen2Off => {
                ctx.call(Function::DisableNpc, vec![Val::from("Lowen Ellenen#2")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnSummon => {
                ctx.var(".brisinmobdead").set(Val::from(9))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(34),
                        Val::from(128),
                        Val::from(""),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(34),
                        Val::from(127),
                        Val::from(""),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(80),
                        Val::from(127),
                        Val::from(""),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(102),
                        Val::from(128),
                        Val::from(""),
                        Val::from(1036),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(130),
                        Val::from(128),
                        Val::from(""),
                        Val::from(1036),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(62),
                        Val::from(128),
                        Val::from(""),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(64),
                        Val::from(127),
                        Val::from(""),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(150),
                        Val::from(127),
                        Val::from(""),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("que_god02"),
                        Val::from(176),
                        Val::from(127),
                        Val::from(""),
                        Val::from(1041),
                        Val::from(1),
                        Val::from("#brisinsummon::OnMobDeath"),
                    ],
                )?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnMobDeath => {
                ctx.var(".brisinmobdead")
                    .set((ctx.var(".brisinmobdead").get()?.try_sub(Val::from(1))?))?;
                if ctx.var(".brisinmobdead").get()?.number()? > 0 {
                    return Err(Stop::End);
                }
                ctx.var("god_brising").set(Val::from(31))?;
                ctx.call(Function::EnableNpc, vec![Val::from("Valkyrie#1")])?;
                return Err(Stop::End);
            }
            BrisinsummonStep::OnReset => {
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#doppelganger2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight1")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#knight3")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("#lowen")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Soldier#1_brising")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Soldier#2_brising")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("#hermite")])?;
                ctx.call(Function::EnableNpc, vec![Val::from("Lowen Ellenen#2")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Valkyrie#1")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("que_god02"), Val::from("#brisinsummon::OnMobDeath")],
                )?;
                return Err(Stop::End);
            }
        }
    }
}
