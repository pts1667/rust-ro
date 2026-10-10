use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Force04mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_04mob_party_run(ctx: &Ctx, mut step: Force04mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force04mobPartyStep::Start => {
                step = Force04mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force04mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(174),
                        Val::from(78),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(184),
                        Val::from(78),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(184),
                        Val::from(68),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(174),
                        Val::from(68),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(169),
                        Val::from(87),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(170),
                        Val::from(87),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(171),
                        Val::from(87),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(172),
                        Val::from(87),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(169),
                        Val::from(77),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(170),
                        Val::from(77),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(171),
                        Val::from(77),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(172),
                        Val::from(77),
                        Val::from("Ride Word"),
                        Val::from(1478),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(183),
                        Val::from(83),
                        Val::from("Wraith Dead"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(183),
                        Val::from(80),
                        Val::from("Wraith Dead"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(183),
                        Val::from(73),
                        Val::from("Wraith Dead"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(183),
                        Val::from(70),
                        Val::from("Wraith Dead"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(179),
                        Val::from(77),
                        Val::from("Wraith Dead"),
                        Val::from(1566),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(169),
                        Val::from(72),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(171),
                        Val::from(72),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(173),
                        Val::from(72),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(175),
                        Val::from(72),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(177),
                        Val::from(72),
                        Val::from("Assaulter"),
                        Val::from(1364),
                        Val::from(1),
                        Val::from("force_04mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force04mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_04mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force04mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_04mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::EnableNpc, vec![Val::from("force_04_03")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("force_03_05")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("force_05start#party")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On04_End1")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_04mob_party(ctx: &Ctx) -> Script {
    force_04mob_party_run(ctx, Force04mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_04mob_party_onenable(ctx: &Ctx) -> Script {
    force_04mob_party_run(ctx, Force04mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_04mob_party_onreset(ctx: &Ctx) -> Script {
    force_04mob_party_run(ctx, Force04mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_04mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_04mob_party_run(ctx, Force04mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05startPartyStep {
    Start,
    OnTouch,
}

fn force_05start_party_run(ctx: &Ctx, mut step: Force05startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05startPartyStep::Start => {
                step = Force05startPartyStep::OnTouch;
                continue 'machine;
            }
            Force05startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_05mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_05start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05start_party(ctx: &Ctx) -> Script {
    force_05start_party_run(ctx, Force05startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05start_party_ontouch(ctx: &Ctx) -> Script {
    force_05start_party_run(ctx, Force05startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force05mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_05mob_party_run(ctx: &Ctx, mut step: Force05mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force05mobPartyStep::Start => {
                step = Force05mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force05mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(25),
                        Val::from(68),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(36),
                        Val::from(68),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(88),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(15),
                        Val::from(78),
                        Val::from("Penomena"),
                        Val::from(1441),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(33),
                        Val::from(88),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(29),
                        Val::from(87),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(25),
                        Val::from(81),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(25),
                        Val::from(78),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(25),
                        Val::from(72),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(29),
                        Val::from(70),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(21),
                        Val::from(70),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(19),
                        Val::from(76),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(19),
                        Val::from(83),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(23),
                        Val::from(89),
                        Val::from("Sting"),
                        Val::from(1489),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(22),
                        Val::from(85),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(23),
                        Val::from(85),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(24),
                        Val::from(85),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(23),
                        Val::from(86),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(22),
                        Val::from(86),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(22),
                        Val::from(71),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(22),
                        Val::from(72),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(23),
                        Val::from(71),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(23),
                        Val::from(72),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(24),
                        Val::from(71),
                        Val::from("Cramp"),
                        Val::from(1570),
                        Val::from(1),
                        Val::from("force_05mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force05mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_05mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force05mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_05mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::On06_Start")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On05_End2")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_05mob_party(ctx: &Ctx) -> Script {
    force_05mob_party_run(ctx, Force05mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_05mob_party_onenable(ctx: &Ctx) -> Script {
    force_05mob_party_run(ctx, Force05mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_05mob_party_onreset(ctx: &Ctx) -> Script {
    force_05mob_party_run(ctx, Force05mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_05mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_05mob_party_run(ctx, Force05mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06startPartyStep {
    Start,
    OnTouch,
}

fn force_06start_party_run(ctx: &Ctx, mut step: Force06startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06startPartyStep::Start => {
                step = Force06startPartyStep::OnTouch;
                continue 'machine;
            }
            Force06startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_06mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_06start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06start_party(ctx: &Ctx) -> Script {
    force_06start_party_run(ctx, Force06startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06start_party_ontouch(ctx: &Ctx) -> Script {
    force_06start_party_run(ctx, Force06startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force06mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_06mob_party_run(ctx: &Ctx, mut step: Force06mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force06mobPartyStep::Start => {
                step = Force06mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force06mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(19),
                        Val::from(135),
                        Val::from("Cloud Hermit"),
                        Val::from(1531),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(24),
                        Val::from(135),
                        Val::from("Cloud Hermit"),
                        Val::from(1531),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(28),
                        Val::from(135),
                        Val::from("Cloud Hermit"),
                        Val::from(1531),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(32),
                        Val::from(135),
                        Val::from("Cloud Hermit"),
                        Val::from(1531),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(24),
                        Val::from(127),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(27),
                        Val::from(127),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(24),
                        Val::from(128),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(27),
                        Val::from(128),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(24),
                        Val::from(129),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(27),
                        Val::from(129),
                        Val::from("Shinobi"),
                        Val::from(1560),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(140),
                        Val::from("Tengu"),
                        Val::from(1563),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(136),
                        Val::from("Tengu"),
                        Val::from(1563),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(132),
                        Val::from("Tengu"),
                        Val::from(1563),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(128),
                        Val::from("Tengu"),
                        Val::from(1563),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(124),
                        Val::from("Tengu"),
                        Val::from(1563),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(120),
                        Val::from("Tengu"),
                        Val::from(1563),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(140),
                        Val::from("Wicked Nymph"),
                        Val::from(1564),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(136),
                        Val::from("Wicked Nymph"),
                        Val::from(1564),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(132),
                        Val::from("Wicked Nymph"),
                        Val::from(1564),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(128),
                        Val::from("Wicked Nymph"),
                        Val::from(1564),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(124),
                        Val::from("Wicked Nymph"),
                        Val::from(1564),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(16),
                        Val::from(120),
                        Val::from("Wicked Nymph"),
                        Val::from(1564),
                        Val::from(1),
                        Val::from("force_06mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force06mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_06mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force06mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_06mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::On07_Start")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On06_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_06mob_party(ctx: &Ctx) -> Script {
    force_06mob_party_run(ctx, Force06mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_06mob_party_onenable(ctx: &Ctx) -> Script {
    force_06mob_party_run(ctx, Force06mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_06mob_party_onreset(ctx: &Ctx) -> Script {
    force_06mob_party_run(ctx, Force06mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_06mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_06mob_party_run(ctx, Force06mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07startPartyStep {
    Start,
    OnTouch,
}

fn force_07start_party_run(ctx: &Ctx, mut step: Force07startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07startPartyStep::Start => {
                step = Force07startPartyStep::OnTouch;
                continue 'machine;
            }
            Force07startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_07mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_07start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07start_party(ctx: &Ctx) -> Script {
    force_07start_party_run(ctx, Force07startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_07start_party_ontouch(ctx: &Ctx) -> Script {
    force_07start_party_run(ctx, Force07startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force07mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_07mob_party_run(ctx: &Ctx, mut step: Force07mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force07mobPartyStep::Start => {
                step = Force07mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force07mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(104),
                        Val::from(134),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(104),
                        Val::from(136),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(104),
                        Val::from(137),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(104),
                        Val::from(139),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(111),
                        Val::from(134),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(111),
                        Val::from(136),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(111),
                        Val::from(137),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(111),
                        Val::from(139),
                        Val::from("Greatest General"),
                        Val::from(1541),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(98),
                        Val::from(122),
                        Val::from("Khalitzburg"),
                        Val::from(1438),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(90),
                        Val::from(119),
                        Val::from("Khalitzburg"),
                        Val::from(1438),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(98),
                        Val::from(122),
                        Val::from("Executioner"),
                        Val::from(1487),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(108),
                        Val::from(140),
                        Val::from("Chimera"),
                        Val::from(1456),
                        Val::from(1),
                        Val::from("force_07mob#party::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            Force07mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_07mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force07mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_07mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::On08_Start")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On07_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_07mob_party(ctx: &Ctx) -> Script {
    force_07mob_party_run(ctx, Force07mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_07mob_party_onenable(ctx: &Ctx) -> Script {
    force_07mob_party_run(ctx, Force07mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_07mob_party_onreset(ctx: &Ctx) -> Script {
    force_07mob_party_run(ctx, Force07mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_07mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_07mob_party_run(ctx, Force07mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08startPartyStep {
    Start,
    OnTouch,
}

fn force_08start_party_run(ctx: &Ctx, mut step: Force08startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force08startPartyStep::Start => {
                step = Force08startPartyStep::OnTouch;
                continue 'machine;
            }
            Force08startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_08mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_08start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08start_party(ctx: &Ctx) -> Script {
    force_08start_party_run(ctx, Force08startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_08start_party_ontouch(ctx: &Ctx) -> Script {
    force_08start_party_run(ctx, Force08startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force08mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_08mob_party_run(ctx: &Ctx, mut step: Force08mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    'machine: loop {
        match step {
            Force08mobPartyStep::Start => {
                step = Force08mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force08mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(172),
                        Val::from(154),
                        Val::from("Khalitzburg"),
                        Val::from(1438),
                        Val::from(1),
                        Val::from("force_08mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(174),
                        Val::from(145),
                        Val::from("Ghostring"),
                        Val::from(1576),
                        Val::from(1),
                        Val::from("force_08mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(174),
                        Val::from(145),
                        Val::from("Chimera"),
                        Val::from(1456),
                        Val::from(1),
                        Val::from("force_08mob#party::OnMyMobDead"),
                    ],
                )?;
                l_i = Val::from(0);
                'l1: loop {
                    if !(l_i.clone().number()? < 5) {
                        break 'l1;
                    }
                    'b1: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("force_1-2"),
                                ctx.call(Function::Rand, vec![Val::from(162), Val::from(184)])?,
                                ctx.call(Function::Rand, vec![Val::from(122), Val::from(185)])?,
                                Val::from("Injustice"),
                                Val::from(1446),
                                Val::from(1),
                                Val::from("force_08mob#party::OnMyMobDead"),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        ctx.call(Function::Rand, vec![Val::from(162), Val::from(184)])?,
                        ctx.call(Function::Rand, vec![Val::from(122), Val::from(185)])?,
                        Val::from("Raydric Archer"),
                        Val::from(1453),
                        Val::from(1),
                        Val::from("force_08mob#party::OnMyMobDead"),
                    ],
                )?;
                l_i = Val::from(0);
                'l2: loop {
                    if !(l_i.clone().number()? < 5) {
                        break 'l2;
                    }
                    'b2: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("force_1-2"),
                                ctx.call(Function::Rand, vec![Val::from(162), Val::from(184)])?,
                                ctx.call(Function::Rand, vec![Val::from(122), Val::from(185)])?,
                                Val::from("Nightmare Terror"),
                                Val::from(1554),
                                Val::from(1),
                                Val::from("force_08mob#party::OnMyMobDead"),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                l_i = Val::from(0);
                'l3: loop {
                    if !(l_i.clone().number()? < 6) {
                        break 'l3;
                    }
                    'b3: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("force_1-2"),
                                ctx.call(Function::Rand, vec![Val::from(162), Val::from(184)])?,
                                ctx.call(Function::Rand, vec![Val::from(122), Val::from(185)])?,
                                Val::from("Ancient Mummy"),
                                Val::from(1522),
                                Val::from(1),
                                Val::from("force_08mob#party::OnMyMobDead"),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                l_i = Val::from(0);
                'l4: loop {
                    if !(l_i.clone().number()? < 10) {
                        break 'l4;
                    }
                    'b4: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("force_1-2"),
                                ctx.call(Function::Rand, vec![Val::from(162), Val::from(184)])?,
                                ctx.call(Function::Rand, vec![Val::from(122), Val::from(185)])?,
                                Val::from("Skel Prisoner"),
                                Val::from(1479),
                                Val::from(1),
                                Val::from("force_08mob#party::OnMyMobDead"),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                l_i = Val::from(0);
                'l5: loop {
                    if !(l_i.clone().number()? < 5) {
                        break 'l5;
                    }
                    'b5: {
                        ctx.call(
                            Function::Monster,
                            vec![
                                Val::from("force_1-2"),
                                ctx.call(Function::Rand, vec![Val::from(162), Val::from(184)])?,
                                ctx.call(Function::Rand, vec![Val::from(122), Val::from(185)])?,
                                Val::from("Hunter Fly"),
                                Val::from(1422),
                                Val::from(1),
                                Val::from("force_08mob#party::OnMyMobDead"),
                            ],
                        )?;
                    }
                    l_i = (l_i.clone() + Val::from(1));
                }
                return Err(Stop::End);
            }
            Force08mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_08mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force08mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_08mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::On09_Start")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On08_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_08mob_party(ctx: &Ctx) -> Script {
    force_08mob_party_run(ctx, Force08mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_08mob_party_onenable(ctx: &Ctx) -> Script {
    force_08mob_party_run(ctx, Force08mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_08mob_party_onreset(ctx: &Ctx) -> Script {
    force_08mob_party_run(ctx, Force08mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_08mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_08mob_party_run(ctx, Force08mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09startPartyStep {
    Start,
    OnTouch,
}

fn force_09start_party_run(ctx: &Ctx, mut step: Force09startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09startPartyStep::Start => {
                step = Force09startPartyStep::OnTouch;
                continue 'machine;
            }
            Force09startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_09mob#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_09start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09start_party(ctx: &Ctx) -> Script {
    force_09start_party_run(ctx, Force09startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09start_party_ontouch(ctx: &Ctx) -> Script {
    force_09start_party_run(ctx, Force09startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force09mobPartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_09mob_party_run(ctx: &Ctx, mut step: Force09mobPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force09mobPartyStep::Start => {
                step = Force09mobPartyStep::OnEnable;
                continue 'machine;
            }
            Force09mobPartyStep::OnEnable => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(86),
                        Val::from(180),
                        Val::from("Elder"),
                        Val::from(1573),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(86),
                        Val::from(176),
                        Val::from("Elder"),
                        Val::from(1573),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(95),
                        Val::from(183),
                        Val::from("Elder"),
                        Val::from(1573),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(94),
                        Val::from(175),
                        Val::from("Elder"),
                        Val::from(1573),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(76),
                        Val::from(178),
                        Val::from("Elder"),
                        Val::from(1573),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(84),
                        Val::from(179),
                        Val::from("Explosion"),
                        Val::from(1532),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(74),
                        Val::from(181),
                        Val::from("Explosion"),
                        Val::from(1532),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(73),
                        Val::from(176),
                        Val::from("Explosion"),
                        Val::from(1532),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(62),
                        Val::from(178),
                        Val::from("Explosion"),
                        Val::from(1532),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(68),
                        Val::from(177),
                        Val::from("Explosion"),
                        Val::from(1532),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(122),
                        Val::from(177),
                        Val::from("Lava Golem"),
                        Val::from(1549),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(112),
                        Val::from(179),
                        Val::from("Lava Golem"),
                        Val::from(1549),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(122),
                        Val::from(178),
                        Val::from("Anolian"),
                        Val::from(1488),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(121),
                        Val::from(177),
                        Val::from("Anolian"),
                        Val::from(1488),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("force_1-2"),
                        Val::from(122),
                        Val::from(177),
                        Val::from("Anolian"),
                        Val::from(1488),
                        Val::from(1),
                        Val::from("force_09mob#party::OnMyMobDead"),
                    ],
                )?;
                ctx.var("$force_09_pt").set(Val::from(15))?;
                return Err(Stop::End);
            }
            Force09mobPartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_09mob#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force09mobPartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_09mob#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("arena_p::On10_Start")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Slipslowrun#party::On09_End")])?;
                    ctx.var("$arn_partyc").set((ctx.var("$arn_partyc").get()? + Val::from(1)))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_09mob_party(ctx: &Ctx) -> Script {
    force_09mob_party_run(ctx, Force09mobPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_09mob_party_onenable(ctx: &Ctx) -> Script {
    force_09mob_party_run(ctx, Force09mobPartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_09mob_party_onreset(ctx: &Ctx) -> Script {
    force_09mob_party_run(ctx, Force09mobPartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_09mob_party_onmymobdead(ctx: &Ctx) -> Script {
    force_09mob_party_run(ctx, Force09mobPartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force10startPartyStep {
    Start,
    OnTouch,
}

fn force_10start_party_run(ctx: &Ctx, mut step: Force10startPartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force10startPartyStep::Start => {
                step = Force10startPartyStep::OnTouch;
                continue 'machine;
            }
            Force10startPartyStep::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-1#party::OnEnable")])?;
                ctx.call(Function::DisableNpc, vec![Val::from("force_10start#party")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_10start_party(ctx: &Ctx) -> Script {
    force_10start_party_run(ctx, Force10startPartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_10start_party_ontouch(ctx: &Ctx) -> Script {
    force_10start_party_run(ctx, Force10startPartyStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Force10mob1PartyStep {
    Start,
    OnEnable,
    OnReset,
    OnMyMobDead,
}

fn force_10mob_1_party_run(ctx: &Ctx, mut step: Force10mob1PartyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Force10mob1PartyStep::Start => {
                step = Force10mob1PartyStep::OnEnable;
                continue 'machine;
            }
            Force10mob1PartyStep::OnEnable => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(2)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_1-2"),
                            Val::from(16),
                            Val::from(179),
                            Val::from("Evil Snake Lord"),
                            Val::from(1529),
                            Val::from(1),
                            Val::from("force_10mob-1#party::OnMyMobDead"),
                        ],
                    )?;
                } else if subject1 == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("force_1-2"),
                            Val::from(24),
                            Val::from(179),
                            Val::from("Dracula"),
                            Val::from(1530),
                            Val::from(1),
                            Val::from("force_10mob-1#party::OnMyMobDead"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            Force10mob1PartyStep::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("force_1-2"), Val::from("force_10mob-1#party::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            Force10mob1PartyStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("force_1-2"), Val::from("force_10mob-1#party::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("force_10mob-2#party::OnEnable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn force_10mob_1_party(ctx: &Ctx) -> Script {
    force_10mob_1_party_run(ctx, Force10mob1PartyStep::Start, Vec::new()).map(|_| ())
}

pub fn force_10mob_1_party_onenable(ctx: &Ctx) -> Script {
    force_10mob_1_party_run(ctx, Force10mob1PartyStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn force_10mob_1_party_onreset(ctx: &Ctx) -> Script {
    force_10mob_1_party_run(ctx, Force10mob1PartyStep::OnReset, Vec::new()).map(|_| ())
}

pub fn force_10mob_1_party_onmymobdead(ctx: &Ctx) -> Script {
    force_10mob_1_party_run(ctx, Force10mob1PartyStep::OnMyMobDead, Vec::new()).map(|_| ())
}
