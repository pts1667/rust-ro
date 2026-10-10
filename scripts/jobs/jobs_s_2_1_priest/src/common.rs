use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum ZombieGeneratorPrstStep {
    Start,
    OnInit,
    OnEnable,
    Onm1,
    Onm2,
    Onm3,
    Onm4,
    Onm5,
    OnDisable,
    OnTimer300000,
    OnTimer300500,
    OnTimer301500,
    OnTimer302000,
}

pub(super) fn zombie_generator_prst_run(ctx: &Ctx, mut step: ZombieGeneratorPrstStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ZombieGeneratorPrstStep::Start => {
                step = ZombieGeneratorPrstStep::OnInit;
                continue 'machine;
            }
            ZombieGeneratorPrstStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie_Generator#prst")])?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie1_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie2_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie3_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie4_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie5_1::OnEnable")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::Onm1 => {
                ctx.var(".mymobs").set(Val::from(13))?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(24),
                        Val::from(52),
                        Val::from("Theft"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(18),
                        Val::from(52),
                        Val::from("Want of Virtue"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(30),
                        Val::from(52),
                        Val::from("Jealousy"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::Onm2 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(21),
                        Val::from(62),
                        Val::from("Fury"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(27),
                        Val::from(62),
                        Val::from("Envy"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::Onm3 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(24),
                        Val::from(72),
                        Val::from("Arrogance"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(18),
                        Val::from(72),
                        Val::from("Lewdness"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(30),
                        Val::from(72),
                        Val::from("Sloth"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::Onm4 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(21),
                        Val::from(82),
                        Val::from("Gluttony"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(27),
                        Val::from(82),
                        Val::from("Greed"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::Onm5 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(24),
                        Val::from(92),
                        Val::from("Despair"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(18),
                        Val::from(92),
                        Val::from("Distrust"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(30),
                        Val::from(92),
                        Val::from("Fear"),
                        Val::from(1015),
                        Val::from(1),
                        Val::from("Z_C#prst::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Zombie_Generator#prst")])?;
                ctx.call(Function::KillMonster, vec![Val::from("job_prist"), Val::from("All")])?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::OnTimer300000 => {
                ctx.call(
                    Function::AreaWarp,
                    vec![
                        Val::from("job_prist"),
                        Val::from(8),
                        Val::from(34),
                        Val::from(39),
                        Val::from(109),
                        Val::from("prontera"),
                        Val::from(234),
                        Val::from(318),
                    ],
                )?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::OnTimer300500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::OnDisable")])?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::OnTimer301500 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Zombie_Generator#prst::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto#2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Peter S. Alberto::OnEnable")])?;
                return Err(Stop::End);
            }
            ZombieGeneratorPrstStep::OnTimer302000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Zombie11Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Zombie21Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Zombie31Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Zombie41Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Zombie51Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MummyGeneratorStep {
    Start,
    OnInit,
    OnEnable,
    Onm1,
    Onm2,
    Onm3,
    OnDisable,
}

pub(super) fn mummy_generator_run(ctx: &Ctx, mut step: MummyGeneratorStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MummyGeneratorStep::Start => {
                step = MummyGeneratorStep::OnInit;
                continue 'machine;
            }
            MummyGeneratorStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy_Generator")])?;
                return Err(Stop::End);
            }
            MummyGeneratorStep::OnEnable => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy1_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy2_1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Mummy3_1::OnEnable")])?;
                return Err(Stop::End);
            }
            MummyGeneratorStep::Onm1 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(90),
                        Val::from(55),
                        Val::from("Khamoz"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(105),
                        Val::from(55),
                        Val::from("Amocsis"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MummyGeneratorStep::Onm2 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(90),
                        Val::from(70),
                        Val::from("Mentuhoteph"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(105),
                        Val::from(70),
                        Val::from("Akenaten"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MummyGeneratorStep::Onm3 => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(90),
                        Val::from(85),
                        Val::from("Mehnes"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("job_prist"),
                        Val::from(105),
                        Val::from(85),
                        Val::from("Snepheru"),
                        Val::from(1041),
                        Val::from(1),
                    ],
                )?;
                return Err(Stop::End);
            }
            MummyGeneratorStep::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Mummy_Generator")])?;
                ctx.call(Function::KillMonster, vec![Val::from("job_prist"), Val::from("All")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mummy11Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mummy21Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mummy31Step {
    Start,
    OnInit,
    OnTouch,
    OnEnable,
    OnDisable,
}
