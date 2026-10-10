use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum MonsterControlerAruGdStep {
    Start,
    OnInit,
    OnTimer3600000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MonsterControler1AruStep {
    Start,
    OnEnable,
    OnKill,
    OnMyMobDead,
}

pub(super) fn monster_controler1_aru_run(ctx: &Ctx, mut step: MonsterControler1AruStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_callwhere = Val::from(0);
    'machine: loop {
        match step {
            MonsterControler1AruStep::Start => {
                step = MonsterControler1AruStep::OnEnable;
                continue 'machine;
            }
            MonsterControler1AruStep::OnEnable => {
                l_callwhere = ctx.call(Function::Rand, vec![Val::from(1), Val::from(4)])?;
                if l_callwhere.clone() == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("arug_dun01"),
                            Val::from(150),
                            Val::from(340),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#aru::OnMyMobDead"),
                        ],
                    )?;
                } else if l_callwhere.clone() == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("arug_dun01"),
                            Val::from(232),
                            Val::from(293),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#aru::OnMyMobDead"),
                        ],
                    )?;
                } else if l_callwhere.clone() == 3 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("arug_dun01"),
                            Val::from(156),
                            Val::from(167),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#aru::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("arug_dun01"),
                            Val::from(243),
                            Val::from(56),
                            Val::from("Kublin"),
                            Val::from(1980),
                            Val::from(1),
                            Val::from("Monster Controler1#aru::OnMyMobDead"),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            MonsterControler1AruStep::OnKill => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("arug_dun01"), Val::from("Monster Controler1#aru::OnMyMObDead")],
                )?;
                return Err(Stop::End);
            }
            MonsterControler1AruStep::OnMyMobDead => {
                if ctx.call(
                    Function::MobCount,
                    vec![Val::from("arug_dun01"), Val::from("Monster Controler1#aru::OnMyMObDead")],
                )? == 0
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_dun01"),
                            Val::from("Kublin: Aargh!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("arug_dun01"),
                            Val::from("Morestone: I thought I heard Kublin screaming!! Who is there? What happened to Kublin? Hey you!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x99CC00"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dwarf#aru_gd::OnEnable")])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ControllerGdeventAStep {
    Start,
    OnInit,
    Onwin,
    OngameStart,
    OnStop,
    OnTimer40000,
    OnTimer60000,
    OnTimer63000,
}

pub(super) fn controller_gdevent_a_run(ctx: &Ctx, mut step: ControllerGdeventAStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_roulette_where = Val::from(0);
    'machine: loop {
        match step {
            ControllerGdeventAStep::Start => {
                step = ControllerGdeventAStep::OnInit;
                continue 'machine;
            }
            ControllerGdeventAStep::OnInit => {
                ctx.var("$@gdeventv_a2").set(Val::from(0))?;
                return Err(Stop::End);
            }
            ControllerGdeventAStep::Onwin => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("treg#aru_gd::OnEnable")])?;
                return Err(Stop::End);
            }
            ControllerGdeventAStep::OngameStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_1_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_2_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_3_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_4_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_5_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_6_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_7_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_8_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_9_a::OnEnable")])?;
                l_roulette_where = ctx.call(Function::Rand, vec![Val::from(1), Val::from(9)])?;
                if l_roulette_where.clone() == 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_1_a::OnBingo")])?;
                } else {
                    if l_roulette_where.clone() == 2 {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_2_a::OnBingo")])?;
                    } else {
                        if l_roulette_where.clone() == 3 {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_3_a::OnBingo")])?;
                        } else {
                            if l_roulette_where.clone() == 4 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_4_a::OnBingo")])?;
                            } else if l_roulette_where.clone() == 5 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_5_a::OnBingo")])?;
                            } else if l_roulette_where.clone() == 6 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_6_a::OnBingo")])?;
                            } else if l_roulette_where.clone() == 7 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_7_a::OnBingo")])?;
                            } else if l_roulette_where.clone() == 8 {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_8_a::OnBingo")])?;
                            } else {
                                ctx.call(Function::DoNpcEvent, vec![Val::from("paper_sp_9_a::OnBingo")])?;
                            }
                        }
                    }
                }
                return Err(Stop::End);
            }
            ControllerGdeventAStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ControllerGdeventAStep::OnTimer40000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: Time is running out, hurry up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            ControllerGdeventAStep::OnTimer60000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("arug_que01"),
                        Val::from("Pierrot Pier: Time is up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x99CC00"),
                    ],
                )?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("arug_que01"), Val::from("arug_que01"), Val::from(100), Val::from(79)],
                )?;
                ctx.call(Function::EnableNpc, vec![Val::from("removepp_aru_gd")])?;
                ctx.var("$@gdeventv_a2").set(Val::from(2))?;
                return Err(Stop::End);
            }
            ControllerGdeventAStep::OnTimer63000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
