use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum MonsterControlerSchGdStep {
    Start,
    OnInit,
    OnTimer3600000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MonsterControler1SchStep {
    Start,
    OnEnable,
    OnKill,
    OnMyMobDead,
}

pub(super) fn monster_controler1_sch_run(ctx: &Ctx, mut step: MonsterControler1SchStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_callwhere = Val::from(0);
    'machine: loop {
        match step {
            MonsterControler1SchStep::Start => {
                step = MonsterControler1SchStep::OnEnable;
                continue 'machine;
            }
            MonsterControler1SchStep::OnEnable => {
                l_callwhere = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if l_callwhere.clone() == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("schg_dun01"),
                            Val::from(164),
                            Val::from(236),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#sch::OnMyMobDead"),
                        ],
                    )?;
                } else if l_callwhere.clone() == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("schg_dun01"),
                            Val::from(172),
                            Val::from(122),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#sch::OnMyMobDead"),
                        ],
                    )?;
                } else if l_callwhere.clone() == 3 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("schg_dun01"),
                            Val::from(247),
                            Val::from(159),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#sch::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("schg_dun01"),
                            Val::from(250),
                            Val::from(224),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#sch::OnMyMobDead"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            MonsterControler1SchStep::OnKill => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("schg_dun01"), Val::from("Monster Controler1#sch::OnMyMObDead")],
                )?;
                return Err(Stop::End);
            }
            MonsterControler1SchStep::OnMyMobDead => {
                if ctx.call(
                    Function::MobCount,
                    vec![Val::from("schg_dun01"), Val::from("Monster Controler1#sch::OnMyMObDead")],
                )? == 0
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("schg_dun01"),
                            Val::from("Kublin: Aargh!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("schg_dun01"),
                            Val::from("Morestone: I thought I heard Kublin screaming!! Who is there? What happened to Kublin? Hey you!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dwarf#sch_gd::OnEnable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ControllerGdeventSStep {
    Start,
    OnInit,
    Onwin,
    OngameStart,
    OnStop,
    OnTimer40000,
    OnTimer60000,
    OnTimer63000,
}

pub(super) fn controller_gdevent_s_run(ctx: &Ctx, mut step: ControllerGdeventSStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_roulette_where = Val::from(0);
    'machine: loop {
        match step {
            ControllerGdeventSStep::Start => {
                step = ControllerGdeventSStep::OnInit;
                continue 'machine;
            }
            ControllerGdeventSStep::OnInit => {
                ctx.var("$@gdeventv_s2").set(Val::from(0))?;
                return Err(Stop::End);
            }
            ControllerGdeventSStep::Onwin => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("treg#sch_gd::OnEnable")])?;
                return Err(Stop::End);
            }
            ControllerGdeventSStep::OngameStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_1_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_2_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_3_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_4_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_5_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_6_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_7_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_8_s::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_9_s::OnEnable")])?;
                l_roulette_where = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                if l_roulette_where.clone() == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_1_s::OnBingo")])?;
                } else {
                    if l_roulette_where.clone() == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_2_s::OnBingo")])?;
                    } else {
                        if l_roulette_where.clone() == 3 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_3_s::OnBingo")])?;
                        } else {
                            if l_roulette_where.clone() == 4 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_4_s::OnBingo")])?;
                            } else if l_roulette_where.clone() == 5 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_5_s::OnBingo")])?;
                            } else if l_roulette_where.clone() == 6 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_6_s::OnBingo")])?;
                            } else if l_roulette_where.clone() == 7 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_7_s::OnBingo")])?;
                            } else if l_roulette_where.clone() == 8 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_8_s::OnBingo")])?;
                            } else {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_9_s::OnBingo")])?;
                            }
                        }
                    }
                }
                return Err(Stop::End);
            }
            ControllerGdeventSStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ControllerGdeventSStep::OnTimer40000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("schg_que01"),
                        Val::from("Pierrot Pier: Time is running out, hurry up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ControllerGdeventSStep::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("schg_que01"),
                        Val::from("Pierrot Pier: Time is up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("schg_que01"), Val::from("schg_que01"), Val::from(100), Val::from(79)],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("removepp_sch_gd")])?;
                ctx.var("$@gdeventv_s2").set(Val::from(2))?;
                return Err(Stop::End);
            }
            ControllerGdeventSStep::OnTimer63000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
