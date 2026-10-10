use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum Nyd2fRedCStep {
    Start,
    OnInstanceInit,
    OnEnable,
    OnMyMobDead,
    OnDisable,
    OnTimer180000,
}

fn nyd_2f_red_c_run(ctx: &Ctx, mut step: Nyd2fRedCStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_label_s = Val::from("");
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            Nyd2fRedCStep::Start => {
                step = Nyd2fRedCStep::OnInstanceInit;
                continue 'machine;
            }
            Nyd2fRedCStep::OnInstanceInit => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedCStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?;
                l_label_s = (ctx.call(
                    Function::InstanceNpcName,
                    vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
                )? + Val::from("::OnMyMobDead"));
                if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("red")).is_true() {
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(120),
                            Val::from(285),
                            Val::from("Nidhoggur's Guardian#1"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(119),
                            Val::from(286),
                            Val::from("Nidhoggur's Guardian#2"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(120),
                            Val::from(270),
                            Val::from("Nidhoggur's Guardian#3"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(119),
                            Val::from(269),
                            Val::from("Nidhoggur's Guardian#4"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(118),
                            Val::from(270),
                            Val::from("Nidhoggur's Guardian#5"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("white")).is_true() {
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(120),
                            Val::from(380),
                            Val::from("Nidhoggur's Guardian#1"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(119),
                            Val::from(381),
                            Val::from("Nidhoggur's Guardian#2"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(120),
                            Val::from(367),
                            Val::from("Nidhoggur's Guardian#3"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(119),
                            Val::from(366),
                            Val::from("Nidhoggur's Guardian#4"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(118),
                            Val::from(365),
                            Val::from("Nidhoggur's Guardian#5"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("yellow")).is_true() {
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(282),
                            Val::from(284),
                            Val::from("Nidhoggur's Guardian#1"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(283),
                            Val::from(283),
                            Val::from("Nidhoggur's Guardian#2"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(280),
                            Val::from(275),
                            Val::from("Nidhoggur's Guardian#3"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(281),
                            Val::from(276),
                            Val::from("Nidhoggur's Guardian#4"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(281),
                            Val::from(277),
                            Val::from("Nidhoggur's Guardian#5"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("green")).is_true() {
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(282),
                            Val::from(378),
                            Val::from("Nidhoggur's Guardian#1"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(283),
                            Val::from(377),
                            Val::from("Nidhoggur's Guardian#2"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(280),
                            Val::from(368),
                            Val::from("Nidhoggur's Guardian#3"),
                            Val::from(2020),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(281),
                            Val::from(368),
                            Val::from("Nidhoggur's Guardian#4"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            l_map_s.clone(),
                            Val::from(281),
                            Val::from(367),
                            Val::from("Nidhoggur's Guardian#5"),
                            Val::from(2021),
                            Val::from(1),
                            l_label_s.clone(),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            Nyd2fRedCStep::OnMyMobDead => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?;
                if ctx
                    .call(
                        Function::MobCount,
                        vec![
                            l_map_s.clone(),
                            (ctx.call(
                                Function::InstanceNpcName,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
                            )? + Val::from("::OnMyMobDead")),
                        ],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            l_map_s.clone(),
                            Val::from("Nidhoggur's Shadow : You're not bad... but I will be your opponent this time."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff99"),
                        ],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_logic")])? + Val::from("::OnEnable"))],
                    )?;
                    ctx.call(
                        Function::DoNpcEvent,
                        vec![
                            (ctx.call(
                                Function::InstanceNpcName,
                                vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
                            )? + Val::from("::OnDisable")),
                        ],
                    )?;
                    return Err(Stop::End);
                }
                return Err(Stop::End);
            }
            Nyd2fRedCStep::OnDisable => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::KillMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
                        )? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![runtime::substr(
                                &ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?,
                                &Val::from(0),
                                &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?).try_sub(Val::from(3))?),
                            )?],
                        )? + Val::from("::OnDisable")),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedCStep::OnTimer180000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        (ctx.call(
                            Function::InstanceNpcName,
                            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?],
                        )? + Val::from("::OnMyMobDead")),
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(ctx.call(Function::InstanceNpcName, vec![Val::from("nyd_2f_boss_enter_call")])? + Val::from("::OnWarpColor"))],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_red_c(ctx: &Ctx) -> Script {
    nyd_2f_red_c_run(ctx, Nyd2fRedCStep::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_c_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_2f_red_c_run(ctx, Nyd2fRedCStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_c_onenable(ctx: &Ctx) -> Script {
    nyd_2f_red_c_run(ctx, Nyd2fRedCStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_c_onmymobdead(ctx: &Ctx) -> Script {
    nyd_2f_red_c_run(ctx, Nyd2fRedCStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_c_ondisable(ctx: &Ctx) -> Script {
    nyd_2f_red_c_run(ctx, Nyd2fRedCStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_c_ontimer180000(ctx: &Ctx) -> Script {
    nyd_2f_red_c_run(ctx, Nyd2fRedCStep::OnTimer180000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nyd2fRedWarp1Step {
    Start,
    OnInstanceInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

fn nyd_2f_red_warp1_run(ctx: &Ctx, mut step: Nyd2fRedWarp1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Nyd2fRedWarp1Step::Start => {
                step = Nyd2fRedWarp1Step::OnInstanceInit;
                continue 'machine;
            }
            Nyd2fRedWarp1Step::OnInstanceInit => {
                step = Nyd2fRedWarp1Step::OnDisable;
                continue 'machine;
            }
            Nyd2fRedWarp1Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedWarp1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                return Err(Stop::End);
            }
            Nyd2fRedWarp1Step::OnTouch => {
                if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("red")).is_true() {
                    ctx.call(
                        Function::Warp,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                            Val::from(115),
                            Val::from(278),
                        ],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("white")).is_true() {
                    ctx.call(
                        Function::Warp,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                            Val::from(115),
                            Val::from(373),
                        ],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("yellow")).is_true() {
                    ctx.call(
                        Function::Warp,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                            Val::from(284),
                            Val::from(278),
                        ],
                    )?;
                } else if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(0)])?, &Val::from("green")).is_true() {
                    ctx.call(
                        Function::Warp,
                        vec![
                            ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                            Val::from(284),
                            Val::from(374),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn nyd_2f_red_warp1(ctx: &Ctx) -> Script {
    nyd_2f_red_warp1_run(ctx, Nyd2fRedWarp1Step::Start, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_warp1_oninstanceinit(ctx: &Ctx) -> Script {
    nyd_2f_red_warp1_run(ctx, Nyd2fRedWarp1Step::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_warp1_ondisable(ctx: &Ctx) -> Script {
    nyd_2f_red_warp1_run(ctx, Nyd2fRedWarp1Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_warp1_onenable(ctx: &Ctx) -> Script {
    nyd_2f_red_warp1_run(ctx, Nyd2fRedWarp1Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn nyd_2f_red_warp1_ontouch(ctx: &Ctx) -> Script {
    nyd_2f_red_warp1_run(ctx, Nyd2fRedWarp1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InsNyd2SpawnMobsStep {
    Start,
    OnInstanceInit,
    OnMyRhynDead,
    OnMyPhyDead,
    OnMyDarkshaDead,
    OnMyPingDead,
}

fn ins_nyd2_spawn_mobs_run(ctx: &Ctx, mut step: InsNyd2SpawnMobsStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_map_s = Val::from("");
    'machine: loop {
        match step {
            InsNyd2SpawnMobsStep::Start => {
                step = InsNyd2SpawnMobsStep::OnInstanceInit;
                continue 'machine;
            }
            InsNyd2SpawnMobsStep::OnInstanceInit => {
                l_map_s = ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        l_map_s.clone(),
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Rhyncho"),
                        Val::from(2020),
                        Val::from(40),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyRhynDead")),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        l_map_s.clone(),
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Phylla"),
                        Val::from(2021),
                        Val::from(40),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyPhyDead")),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        l_map_s.clone(),
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Dark Shadow"),
                        Val::from(2023),
                        Val::from(40),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyDarkshaDead")),
                    ],
                )?;
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        l_map_s.clone(),
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Dark Pinguicula"),
                        Val::from(2015),
                        Val::from(40),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyPingDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd2SpawnMobsStep::OnMyRhynDead => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Rhyncho"),
                        Val::from(2020),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyRhynDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd2SpawnMobsStep::OnMyPhyDead => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Phylla"),
                        Val::from(2021),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyPhyDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd2SpawnMobsStep::OnMyDarkshaDead => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Dark Shadow"),
                        Val::from(2023),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyDarkshaDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
            InsNyd2SpawnMobsStep::OnMyPingDead => {
                ctx.call(
                    Function::AreaMonster,
                    vec![
                        ctx.call(Function::InstanceMapName, vec![Val::from("2@nyd")])?,
                        Val::from(20),
                        Val::from(12),
                        Val::from(380),
                        Val::from(172),
                        Val::from("Dark Pinguicula"),
                        Val::from(2015),
                        Val::from(1),
                        (ctx.call(Function::InstanceNpcName, vec![Val::from("ins_nyd2_spawn_mobs")])? + Val::from("::OnMyPingDead")),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ins_nyd2_spawn_mobs(ctx: &Ctx) -> Script {
    ins_nyd2_spawn_mobs_run(ctx, InsNyd2SpawnMobsStep::Start, Vec::new()).map(|_| ())
}

pub fn ins_nyd2_spawn_mobs_oninstanceinit(ctx: &Ctx) -> Script {
    ins_nyd2_spawn_mobs_run(ctx, InsNyd2SpawnMobsStep::OnInstanceInit, Vec::new()).map(|_| ())
}

pub fn ins_nyd2_spawn_mobs_onmyrhyndead(ctx: &Ctx) -> Script {
    ins_nyd2_spawn_mobs_run(ctx, InsNyd2SpawnMobsStep::OnMyRhynDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd2_spawn_mobs_onmyphydead(ctx: &Ctx) -> Script {
    ins_nyd2_spawn_mobs_run(ctx, InsNyd2SpawnMobsStep::OnMyPhyDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd2_spawn_mobs_onmydarkshadead(ctx: &Ctx) -> Script {
    ins_nyd2_spawn_mobs_run(ctx, InsNyd2SpawnMobsStep::OnMyDarkshaDead, Vec::new()).map(|_| ())
}

pub fn ins_nyd2_spawn_mobs_onmypingdead(ctx: &Ctx) -> Script {
    ins_nyd2_spawn_mobs_run(ctx, InsNyd2SpawnMobsStep::OnMyPingDead, Vec::new()).map(|_| ())
}

fn nidhoggur_manager_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    let mut l_var_s = Val::from("");
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.mes("Enter the password.")?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from("1854"), Val::from(1)])? == 0 {
        ctx.mes("Incorrect password.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("What do you need to do?")?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from(
            "Reset variables to allow entrance to Nidhoggur:change variable 'ins_nyd':Remove 3 day timer 3135:Confirm variable number",
        )],
    )? {
        1 => {
            ctx.var("ins_nyd").set(Val::from(200))?;
            ctx.var("ins_nyd2").set(Val::from(0))?;
            ctx.call(Function::EraseQuest, vec![Val::from(3135)])?;
            ctx.lines(args!["ins_nyd set to 200 ins_nyd2 set to 0", "quest 3135 erased."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.mes("Which variable do you want to change?")?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("ins_nyd:ins_nyd2")])? {
                1 => {
                    l_var_s = Val::from("ins_nyd");
                }
                2 => {
                    l_var_s = Val::from("ins_nyd2");
                }
                _ => {}
            }
            ctx.mes("Input the variable number")?;
            ctx.next()?;
            let (input, status) = runtime::input_number(ctx, None, None)?;
            l_input = input;
            runtime::setd(
                ctx,
                &l_var_s.clone(),
                l_input.clone(),
                &mut [
                    (".@input", runtime::LocalMut::Scalar(&mut l_input)),
                    (".@var$", runtime::LocalMut::Scalar(&mut l_var_s)),
                ],
            )?;
            ctx.lines(args![
                (((l_var_s.clone() + Val::from(" has been set to ")) + l_input.clone()) + Val::from(" ."))
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.mes("The 3 day timer will be removed when you close this dialog.")?;
            ctx.next()?;
            ctx.call(Function::EraseQuest, vec![Val::from(3135)])?;
            ctx.mes("Finished removing Quest Timer.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        4 => {
            ctx.lines(args![
                ((Val::from("ins_nyd is at ") + ctx.var("ins_nyd").get()?) + Val::from(".")),
                ((Val::from("ins_nyd2 is at ") + ctx.var("ins_nyd2").get()?) + Val::from("."))
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn nidhoggur_manager(ctx: &Ctx) -> Script {
    nidhoggur_manager_body(ctx, Vec::new()).map(|_| ())
}

fn purification_admin_nyd2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.mes("Please enter the password")?;
    l_i = shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from("dragonslayer"), Val::from(1)])?;
    ctx.next()?;
    if l_i.clone() == 0 {
        ctx.mes("Enter the password exactly.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    match runtime::select_values(
        ctx,
        &[Val::from(
            "Generate Purification Stone:Remove Purification Stone:Turn Entrance NPC OFF:Turn Entrance NPC On:Cancel",
        )],
    )? {
        1 => {
            ctx.mes("Purification stone has been created and will stay on for 30 minutes.")?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Purification Stone#nyd2::OnEnable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.mes("The Purification Stone has been destroyed.")?;
            ctx.call(Function::DoNpcEvent, vec![Val::from("Purification Stone#nyd2::OnDisable")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        3 => {
            ctx.mes("The Yggdrasil Gatekeeper at nyd_dun02 100 201 is now OFF.")?;
            ctx.call(Function::DisableNpc, vec![Val::from("Yggdrasil Gatekeeper")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        4 => {
            ctx.mes("The Yggdrasil Gatekeeper at nyd_dun02 100 201 is now On.")?;
            ctx.call(Function::EnableNpc, vec![Val::from("Yggdrasil Gatekeeper")])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        5 => {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn purification_admin_nyd2(ctx: &Ctx) -> Script {
    purification_admin_nyd2_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EraseQuest, vec![Val::from(3135)])?;
    ctx.call(Function::EraseQuest, vec![Val::from(3136)])?;
    ctx.var("ins_nyd").set(Val::from(200))?;
    ctx.var("ins_nyd2").set(Val::from(0))?;
    ctx.mes("^0000FFThe records and after-effect related to the Nidhoggur's Nest have been removed. You can generate and enter the dungeon again.^000000")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2(ctx: &Ctx) -> Script {
    purification_stone_nyd2_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Purification Stone#nyd2")])?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2_oninit(ctx: &Ctx) -> Script {
    purification_stone_nyd2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2_onenable(ctx: &Ctx) -> Script {
    purification_stone_nyd2_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("Purification Stone#nyd2")])?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2_ondisable(ctx: &Ctx) -> Script {
    purification_stone_nyd2_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Purification Stone#nyd2")])?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2_ontimer1000(ctx: &Ctx) -> Script {
    purification_stone_nyd2_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_ontimer1740000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(
        Function::MapAnnounce,
        vec![
            Val::from("nyd_dun02"),
            Val::from("The Purification Stone will be destroyed in one minute."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00FF99"),
            ctx.constant("FW_NORMAL")?,
            Val::from(12),
        ],
    )?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2_ontimer1740000(ctx: &Ctx) -> Script {
    purification_stone_nyd2_ontimer1740000_body(ctx, Vec::new()).map(|_| ())
}

fn purification_stone_nyd2_ontimer1800000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Purification Stone#nyd2")])?;
    return Err(Stop::End);
}

pub fn purification_stone_nyd2_ontimer1800000(ctx: &Ctx) -> Script {
    purification_stone_nyd2_ontimer1800000_body(ctx, Vec::new()).map(|_| ())
}
