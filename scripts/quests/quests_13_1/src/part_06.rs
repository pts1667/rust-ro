use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MonsterMasterStep {
    Start,
    OnEnable,
    OnStop,
    OnTimer3600000,
    OnTimer3960000,
    OnMyMobDead,
}

fn monster_master_run(ctx: &Ctx, mut step: MonsterMasterStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_monster = Val::from(0);
    'machine: loop {
        match step {
            MonsterMasterStep::Start => {
                step = MonsterMasterStep::OnEnable;
                continue 'machine;
            }
            MonsterMasterStep::OnEnable => {
                if ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?.number()? > 1 {
                    ctx.call(Function::InitNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            MonsterMasterStep::OnStop => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mid_camp"), Val::from("#monster_master::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MonsterMasterStep::OnTimer3600000 => {
                l_monster = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if l_monster.clone() == 1 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("mid_camp"),
                            Val::from(149),
                            Val::from(291),
                            Val::from("Escaped Tatacho"),
                            Val::from(1986),
                            Val::from(1),
                            Val::from("#monster_master::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("mid_camp"),
                            Val::from("Breeder Taab: Argh! My Tatacho ran away!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff00"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Tatacho#alba02::OnDisable")])?;
                } else if l_monster.clone() == 2 {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("mid_camp"),
                            Val::from(154),
                            Val::from(273),
                            Val::from("Escaped Hillsrion"),
                            Val::from(1989),
                            Val::from(1),
                            Val::from("#monster_master::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("mid_camp"),
                            Val::from("Breeder Taab: Argh! My Hillsrion ran away!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff00"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Hillsrion#alba01::OnDisable")])?;
                } else {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("mid_camp"),
                            Val::from(184),
                            Val::from(246),
                            Val::from("Escaped Cornus"),
                            Val::from(1992),
                            Val::from(1),
                            Val::from("#monster_master::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("mid_camp"),
                            Val::from("Breeder Taab: Argh! My Cornus ran away!"),
                            ctx.constant("BC_MAP")?,
                            Val::from("0x00ff00"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Cornus#alba03::OnDisable")])?;
                }
                return Err(Stop::End);
            }
            MonsterMasterStep::OnTimer3960000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mid_camp"), Val::from("#monster_master::OnMyMobDead")],
                )?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("mid_camp"),
                        Val::from("Breeder Taab: I've captured an escaped creature safely. Sorry for yelling so loud."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hillsrion#alba01::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Tatacho#alba02::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cornus#alba03::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MonsterMasterStep::OnMyMobDead => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("mid_camp"),
                        Val::from("Breeder Taab: I've captured an escaped creature safely. Sorry for yelling so loud."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Hillsrion#alba01::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Tatacho#alba02::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Cornus#alba03::OnEnable")])?;
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("mid_camp"), Val::from("#monster_master::OnMyMobDead")],
                )?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn monster_master(ctx: &Ctx) -> Script {
    monster_master_run(ctx, MonsterMasterStep::Start, Vec::new()).map(|_| ())
}

pub fn monster_master_onenable(ctx: &Ctx) -> Script {
    monster_master_run(ctx, MonsterMasterStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn monster_master_onstop(ctx: &Ctx) -> Script {
    monster_master_run(ctx, MonsterMasterStep::OnStop, Vec::new()).map(|_| ())
}

pub fn monster_master_ontimer3600000(ctx: &Ctx) -> Script {
    monster_master_run(ctx, MonsterMasterStep::OnTimer3600000, Vec::new()).map(|_| ())
}

pub fn monster_master_ontimer3960000(ctx: &Ctx) -> Script {
    monster_master_run(ctx, MonsterMasterStep::OnTimer3960000, Vec::new()).map(|_| ())
}

pub fn monster_master_onmymobdead(ctx: &Ctx) -> Script {
    monster_master_run(ctx, MonsterMasterStep::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum CatPawAgentStep {
    Start,
    Catwarp,
    AfterCatwarp,
}

fn cat_paw_agent_run(ctx: &Ctx, mut step: CatPawAgentStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CatPawAgentStep::Start => {
                if ctx.var("ep13_yong1").get()?.number()? < 1 {
                    ctx.lines_as(
                        "Cat Paw Agent",
                        args!["Welcome to Cat Trading.", "I guess you're a first-time", "customer, huh?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Cat Paw Agent",
                        args![
                            "How'd you like to make",
                            "a contract with us?",
                            "We'll guarantee that you'll",
                            "be provided with various",
                            "conveniences during your",
                            "stay at this expedition camp?"
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Tell me more about your services.:No. thanks")])? {
                        1 => {
                            ctx.lines_as("Cat Paw Agent", args!["Before making a contract with us, youll have to go through a few steps to satisfy our terms and conditions."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cat Paw Agent",
                                args![
                                    "More clearly, you'll have to provide us something in return for our services.",
                                    "You know what I mean, don't you?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Cat Paw Agent",
                                args![
                                    "For more information, please speak to Agent Gyaruk standing down there.",
                                    "I'm sure you'll find our business proposition to be very reasonable."
                                ],
                            )?;
                            ctx.var("ep13_yong1").set(Val::from(1))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Cat Paw Agent",
                                args!["Well, feel free to come back whenever you change your mind."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    if ctx.var("ep13_yong1").get()? == 1 {
                        ctx.lines_as(
                            "Cat Paw Agent",
                            args![
                                "For more information, please speak to Agent Gyaruk standing down there.",
                                "I'm sure you'll find our business proposition to be very reasonable."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("ep13_yong1").get()? == 2 {
                            ctx.lines_as(
                                "Cat Paw Agent",
                                args![
                                    "Thank you for making an official contract with us.",
                                    "You're now eligable to use our services."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Cat Paw Agent", args!["For our newcomers, we're offering a location-saving service with which you can save this camp as your returning location.", "Would you like to save your location?"])?;
                            ctx.var("ep13_yong1").set(Val::from(3))?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Save your location:Cancel")])? {
                                1 => {
                                    ctx.call(
                                        Function::SavePoint,
                                        vec![Val::from("mid_camp"), Val::from(56), Val::from(139), Val::from(1), Val::from(1)],
                                    )?;
                                    ctx.lines_as(
                                        "Cat Paw Agent",
                                        args![
                                            "Thank you.",
                                            "Your location has been saved.",
                                            "You can now directly return to this camp."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            if (ctx.var("ep13_yong1").get()?.number()? > 2 && ctx.var("ep13_yong1").get()?.number()? < 20) {
                                ctx.lines_as(
                                    "Cat Paw Agent",
                                    args![
                                        "Cat Trading's available services are as followed.",
                                        "For additional services, please consult Agent Gyaruk."
                                    ],
                                )?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Save your location:Cancel")])? {
                                    1 => {
                                        ctx.call(
                                            Function::SavePoint,
                                            vec![Val::from("mid_camp"), Val::from(56), Val::from(139), Val::from(1), Val::from(1)],
                                        )?;
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "Thank you.",
                                                "Your location has been saved.",
                                                "You can now directly return to this camp."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                if (ctx.var("ep13_yong1").get()?.number()? > 19 && ctx.var("ep13_yong1").get()?.number()? < 40) {
                                    ctx.lines_as(
                                        "Cat Paw Agent",
                                        args![
                                            "Cat Trading's available services are as followed.",
                                            "For additional services, please consult Agent Gyaruk."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Save your location:Use Storage:Cancel")])? {
                                        1 => {
                                            ctx.call(
                                                Function::SavePoint,
                                                vec![Val::from("mid_camp"), Val::from(56), Val::from(139), Val::from(1), Val::from(1)],
                                            )?;
                                            ctx.lines_as(
                                                "Cat Paw Agent",
                                                args![
                                                    "Thank you.",
                                                    "Your location has been saved.",
                                                    "You can now directly return to this camp."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            if !(shared::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true()) {
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args![
                                                        "I'm sorry, but you",
                                                        "need the Novice's",
                                                        "Basic Skill Level 6 to",
                                                        "use the Storage Service."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            } else if ctx.var("Zeny").get()?.number()? >= 60 {
                                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60))?))?;
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args!["Thank you.", "Your storage will", "be opened shortly."],
                                                )?;
                                                ctx.close_window()?;
                                                ctx.call(Function::OpenStorage, vec![])?;
                                                return Err(Stop::End);
                                            } else {
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args![
                                                        "I'm sorry, but you don't",
                                                        "have enough money?",
                                                        "Cat Trading's storage",
                                                        "service is 60 zeny.",
                                                        "It's cheap, isn't it?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                        }
                                        3 => {
                                            ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    if (ctx.var("ep13_yong1").get()?.number()? > 39 && ctx.var("ep13_yong1").get()?.number()? < 100) {
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "Cat Trading's available services are as followed.",
                                                "For additional services, please consult Agent Gyaruk."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from("Save your location:Use Storage:Use Cat Warp (Midgard):Cancel")],
                                        )? {
                                            1 => {
                                                ctx.call(
                                                    Function::SavePoint,
                                                    vec![Val::from("mid_camp"), Val::from(56), Val::from(139), Val::from(1), Val::from(1)],
                                                )?;
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args![
                                                        "Thank you.",
                                                        "Your location has been saved.",
                                                        "You can now directly return to this camp."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                if !(shared::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true()) {
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args![
                                                            "I'm sorry, but you",
                                                            "need the Novice's",
                                                            "Basic Skill Level 6 to",
                                                            "use the Storage Service."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("Zeny").get()?.number()? >= 60 {
                                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60))?))?;
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args!["Thank you.", "Your storage will", "be opened shortly."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::OpenStorage, vec![])?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args![
                                                            "I'm sorry, but you don't",
                                                            "have enough money?",
                                                            "Cat Trading's storage",
                                                            "service is 60 zeny.",
                                                            "It's cheap, isn't it?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args![
                                                        "The warp service is only",
                                                        "available for customers with",
                                                        "40 or more Cat Trading Points.",
                                                        "Please remember, you can't come back easily once you move to Midgard."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if (ctx.var("ep13_yong1").get()?.number()? > 39
                                                    && ctx.var("ep13_yong1").get()?.number()? <= 49)
                                                {
                                                    'b6: {
                                                        let subject6 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Prontera -> 5500z:Cancel")],
                                                        )?);
                                                        let mut matched6 = false;
                                                        let no_case6 = !subject6.loosely_equals(&Val::from(1))
                                                            && !subject6.loosely_equals(&Val::from(2));
                                                        if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                                            matched6 = true;
                                                        }
                                                        if matched6 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(5500), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                                            matched6 = true;
                                                        }
                                                        if matched6 {
                                                            break 'b6;
                                                        }
                                                    }
                                                } else {
                                                    if (ctx.var("ep13_yong1").get()?.number()? > 49
                                                        && ctx.var("ep13_yong1").get()?.number()? < 60)
                                                    {
                                                        'b7: {
                                                            let subject7 = Val::from(runtime::select_values(
                                                                ctx,
                                                                &[Val::from("Alberta -> 5500z:Prontera -> 5500z:Cancel")],
                                                            )?);
                                                            let mut matched7 = false;
                                                            let no_case7 = !subject7.loosely_equals(&Val::from(1))
                                                                && !subject7.loosely_equals(&Val::from(2))
                                                                && !subject7.loosely_equals(&Val::from(3));
                                                            if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                                                                matched7 = true;
                                                            }
                                                            if matched7 {
                                                                cat_paw_agent_run(
                                                                    ctx,
                                                                    CatPawAgentStep::Catwarp,
                                                                    vec![Val::from(5500), Val::from(1)],
                                                                )?;
                                                            }
                                                            if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                                                                matched7 = true;
                                                            }
                                                            if matched7 {
                                                                cat_paw_agent_run(
                                                                    ctx,
                                                                    CatPawAgentStep::Catwarp,
                                                                    vec![Val::from(5500), Val::from(2)],
                                                                )?;
                                                            }
                                                            if !matched7 && subject7.loosely_equals(&Val::from(3)) {
                                                                matched7 = true;
                                                            }
                                                            if matched7 {
                                                                break 'b7;
                                                            }
                                                        }
                                                    } else {
                                                        if (ctx.var("ep13_yong1").get()?.number()? > 59
                                                            && ctx.var("ep13_yong1").get()?.number()? < 70)
                                                        {
                                                            'b8: {
                                                                let subject8 = Val::from(runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "Alberta -> 5025z:Prontera -> 5025z:Izlude -> 5025z:Cancel",
                                                                    )],
                                                                )?);
                                                                let mut matched8 = false;
                                                                let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                                                    && !subject8.loosely_equals(&Val::from(2))
                                                                    && !subject8.loosely_equals(&Val::from(3))
                                                                    && !subject8.loosely_equals(&Val::from(4));
                                                                if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                                                    matched8 = true;
                                                                }
                                                                if matched8 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(1)],
                                                                    )?;
                                                                }
                                                                if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                                                    matched8 = true;
                                                                }
                                                                if matched8 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(2)],
                                                                    )?;
                                                                }
                                                                if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                                                                    matched8 = true;
                                                                }
                                                                if matched8 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(3)],
                                                                    )?;
                                                                }
                                                                if !matched8 && subject8.loosely_equals(&Val::from(4)) {
                                                                    matched8 = true;
                                                                }
                                                                if matched8 {
                                                                    break 'b8;
                                                                }
                                                            }
                                                        } else if (ctx.var("ep13_yong1").get()?.number()? > 69
                                                            && ctx.var("ep13_yong1").get()?.number()? < 80)
                                                        {
                                                            'b9: {
                                                                let subject9 = Val::from(runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "Alberta -> 5025z:Prontera -> 5025z:Izlude -> 5025z:Geffen -> 5025z:Cancel",
                                                                    )],
                                                                )?);
                                                                let mut matched9 = false;
                                                                let no_case9 = !subject9.loosely_equals(&Val::from(1))
                                                                    && !subject9.loosely_equals(&Val::from(2))
                                                                    && !subject9.loosely_equals(&Val::from(3))
                                                                    && !subject9.loosely_equals(&Val::from(4))
                                                                    && !subject9.loosely_equals(&Val::from(5));
                                                                if !matched9 && subject9.loosely_equals(&Val::from(1)) {
                                                                    matched9 = true;
                                                                }
                                                                if matched9 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(1)],
                                                                    )?;
                                                                }
                                                                if !matched9 && subject9.loosely_equals(&Val::from(2)) {
                                                                    matched9 = true;
                                                                }
                                                                if matched9 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(2)],
                                                                    )?;
                                                                }
                                                                if !matched9 && subject9.loosely_equals(&Val::from(3)) {
                                                                    matched9 = true;
                                                                }
                                                                if matched9 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(3)],
                                                                    )?;
                                                                }
                                                                if !matched9 && subject9.loosely_equals(&Val::from(4)) {
                                                                    matched9 = true;
                                                                }
                                                                if matched9 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(5025), Val::from(4)],
                                                                    )?;
                                                                }
                                                                if !matched9 && subject9.loosely_equals(&Val::from(5)) {
                                                                    matched9 = true;
                                                                }
                                                                if matched9 {
                                                                    break 'b9;
                                                                }
                                                            }
                                                        } else if (ctx.var("ep13_yong1").get()?.number()? > 79
                                                            && ctx.var("ep13_yong1").get()?.number()? < 90)
                                                        {
                                                            'b10: {
                                                                let subject10 = Val::from(runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "Alberta -> 4765z:Prontera -> 4765z:Izlude -> 4765z:Geffen -> 4765z:Payon -> 4765z:Cancel",
                                                                    )],
                                                                )?);
                                                                let mut matched10 = false;
                                                                let no_case10 = !subject10.loosely_equals(&Val::from(1))
                                                                    && !subject10.loosely_equals(&Val::from(2))
                                                                    && !subject10.loosely_equals(&Val::from(3))
                                                                    && !subject10.loosely_equals(&Val::from(4))
                                                                    && !subject10.loosely_equals(&Val::from(5))
                                                                    && !subject10.loosely_equals(&Val::from(6));
                                                                if !matched10 && subject10.loosely_equals(&Val::from(1)) {
                                                                    matched10 = true;
                                                                }
                                                                if matched10 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(1)],
                                                                    )?;
                                                                }
                                                                if !matched10 && subject10.loosely_equals(&Val::from(2)) {
                                                                    matched10 = true;
                                                                }
                                                                if matched10 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(2)],
                                                                    )?;
                                                                }
                                                                if !matched10 && subject10.loosely_equals(&Val::from(3)) {
                                                                    matched10 = true;
                                                                }
                                                                if matched10 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(3)],
                                                                    )?;
                                                                }
                                                                if !matched10 && subject10.loosely_equals(&Val::from(4)) {
                                                                    matched10 = true;
                                                                }
                                                                if matched10 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(4)],
                                                                    )?;
                                                                }
                                                                if !matched10 && subject10.loosely_equals(&Val::from(5)) {
                                                                    matched10 = true;
                                                                }
                                                                if matched10 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(5)],
                                                                    )?;
                                                                }
                                                                if !matched10 && subject10.loosely_equals(&Val::from(6)) {
                                                                    matched10 = true;
                                                                }
                                                                if matched10 {
                                                                    break 'b10;
                                                                }
                                                            }
                                                        } else if (ctx.var("ep13_yong1").get()?.number()? > 89
                                                            && ctx.var("ep13_yong1").get()?.number()? < 100)
                                                        {
                                                            'b11: {
                                                                let subject11 = Val::from(runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "Alberta -> 4765z:Prontera -> 4765z:Izlude -> 4765z:Geffen -> 4765z:Payon -> 4765z:Morocc -> 4765z:Cancel",
                                                                    )],
                                                                )?);
                                                                let mut matched11 = false;
                                                                let no_case11 = !subject11.loosely_equals(&Val::from(1))
                                                                    && !subject11.loosely_equals(&Val::from(2))
                                                                    && !subject11.loosely_equals(&Val::from(3))
                                                                    && !subject11.loosely_equals(&Val::from(4))
                                                                    && !subject11.loosely_equals(&Val::from(5))
                                                                    && !subject11.loosely_equals(&Val::from(6))
                                                                    && !subject11.loosely_equals(&Val::from(7));
                                                                if !matched11 && subject11.loosely_equals(&Val::from(1)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(1)],
                                                                    )?;
                                                                }
                                                                if !matched11 && subject11.loosely_equals(&Val::from(2)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(2)],
                                                                    )?;
                                                                }
                                                                if !matched11 && subject11.loosely_equals(&Val::from(3)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(3)],
                                                                    )?;
                                                                }
                                                                if !matched11 && subject11.loosely_equals(&Val::from(4)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(4)],
                                                                    )?;
                                                                }
                                                                if !matched11 && subject11.loosely_equals(&Val::from(5)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(5)],
                                                                    )?;
                                                                }
                                                                if !matched11 && subject11.loosely_equals(&Val::from(6)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4765), Val::from(6)],
                                                                    )?;
                                                                }
                                                                if !matched11 && subject11.loosely_equals(&Val::from(7)) {
                                                                    matched11 = true;
                                                                }
                                                                if matched11 {
                                                                    break 'b11;
                                                                }
                                                            }
                                                        } else if ctx.var("ep13_yong1").get()?.number()? > 99 {
                                                            'b12: {
                                                                let subject12 = Val::from(runtime::select_values(
                                                                    ctx,
                                                                    &[Val::from(
                                                                        "Alberta -> 4590z:Prontera -> 4590z:Izlude -> 4590z:Geffen -> 4590z:Payon -> 4590z:Morocc -> 4590z:Al De Baran -> 4590z:Cancel",
                                                                    )],
                                                                )?);
                                                                let mut matched12 = false;
                                                                let no_case12 = !subject12.loosely_equals(&Val::from(1))
                                                                    && !subject12.loosely_equals(&Val::from(2))
                                                                    && !subject12.loosely_equals(&Val::from(3))
                                                                    && !subject12.loosely_equals(&Val::from(4))
                                                                    && !subject12.loosely_equals(&Val::from(5))
                                                                    && !subject12.loosely_equals(&Val::from(6))
                                                                    && !subject12.loosely_equals(&Val::from(7))
                                                                    && !subject12.loosely_equals(&Val::from(8));
                                                                if !matched12 && subject12.loosely_equals(&Val::from(1)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(1)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(2)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(2)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(3)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(3)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(4)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(4)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(5)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(5)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(6)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(6)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(7)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    cat_paw_agent_run(
                                                                        ctx,
                                                                        CatPawAgentStep::Catwarp,
                                                                        vec![Val::from(4590), Val::from(7)],
                                                                    )?;
                                                                }
                                                                if !matched12 && subject12.loosely_equals(&Val::from(8)) {
                                                                    matched12 = true;
                                                                }
                                                                if matched12 {
                                                                    break 'b12;
                                                                }
                                                            }
                                                        } else {
                                                            ctx.lines_as("Cat Paw Agent", args!["I'm sorry, but you're not eligible to use the warp service. Please check your points, and then come back."])?;
                                                        }
                                                    }
                                                }
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            4 => {
                                                ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else if ctx.var("ep13_yong1").get()?.number()? > 99 {
                                        ctx.lines_as(
                                            "Cat Paw Agent",
                                            args![
                                                "Cat Trading's available services are as followed.",
                                                "For additional services, please consult Agent Gyaruk."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from(
                                                "Save your location:Use Storage:Use Cat Warp (Midgard):Use Cat Warp (Jottunheim):Cancel",
                                            )],
                                        )? {
                                            1 => {
                                                ctx.call(
                                                    Function::SavePoint,
                                                    vec![Val::from("mid_camp"), Val::from(56), Val::from(139), Val::from(1), Val::from(1)],
                                                )?;
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args![
                                                        "Thank you.",
                                                        "Your location has been saved.",
                                                        "You can now directly return to this camp."
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                if !(shared::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true()) {
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args![
                                                            "I'm sorry, but you",
                                                            "need the Novice's",
                                                            "Basic Skill Level 6 to",
                                                            "use the Storage Service."
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("Zeny").get()?.number()? >= 60 {
                                                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(60))?))?;
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args!["Thank you.", "Your storage will", "be opened shortly."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    ctx.call(Function::OpenStorage, vec![])?;
                                                    return Err(Stop::End);
                                                } else {
                                                    ctx.lines_as(
                                                        "Cat Paw Agent",
                                                        args![
                                                            "I'm sorry, but you don't",
                                                            "have enough money?",
                                                            "Cat Trading's storage",
                                                            "service is 60 zeny.",
                                                            "It's cheap, isn't it?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Cat Paw Agent",
                                                    args![
                                                        "The warp service is only",
                                                        "available for customers with",
                                                        "40 or more Cat Trading Points.",
                                                        "Please remember, you can't come back easily once you move to Midgard."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                if (ctx.var("ep13_yong1").get()?.number()? > 99
                                                    && ctx.var("ep13_yong1").get()?.number()? < 200)
                                                {
                                                    'b14: {
                                                        let subject14 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 4590z:Prontera -> 4590z:Izlude -> 4590z:Geffen -> 4590z:Payon -> 4590z:Morocc -> 4590z:Al De Baran -> 4590z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched14 = false;
                                                        let no_case14 = !subject14.loosely_equals(&Val::from(1))
                                                            && !subject14.loosely_equals(&Val::from(2))
                                                            && !subject14.loosely_equals(&Val::from(3))
                                                            && !subject14.loosely_equals(&Val::from(4))
                                                            && !subject14.loosely_equals(&Val::from(5))
                                                            && !subject14.loosely_equals(&Val::from(6))
                                                            && !subject14.loosely_equals(&Val::from(7))
                                                            && !subject14.loosely_equals(&Val::from(8));
                                                        if !matched14 && subject14.loosely_equals(&Val::from(1)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(2)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(3)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(4)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(5)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(6)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(6)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(7)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4590), Val::from(7)],
                                                            )?;
                                                        }
                                                        if !matched14 && subject14.loosely_equals(&Val::from(8)) {
                                                            matched14 = true;
                                                        }
                                                        if matched14 {
                                                            break 'b14;
                                                        }
                                                    }
                                                } else if (ctx.var("ep13_yong1").get()?.number()? > 199
                                                    && ctx.var("ep13_yong1").get()?.number()? < 250)
                                                {
                                                    'b15: {
                                                        let subject15 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 4170z:Prontera -> 4170z:Izlude -> 4170z:Geffen -> 4170z:Payon -> 4170z:Morocc -> 4170z:Al De Baran -> 4170z:Juno -> 4170z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched15 = false;
                                                        let no_case15 = !subject15.loosely_equals(&Val::from(1))
                                                            && !subject15.loosely_equals(&Val::from(2))
                                                            && !subject15.loosely_equals(&Val::from(3))
                                                            && !subject15.loosely_equals(&Val::from(4))
                                                            && !subject15.loosely_equals(&Val::from(5))
                                                            && !subject15.loosely_equals(&Val::from(6))
                                                            && !subject15.loosely_equals(&Val::from(7))
                                                            && !subject15.loosely_equals(&Val::from(8))
                                                            && !subject15.loosely_equals(&Val::from(9));
                                                        if !matched15 && subject15.loosely_equals(&Val::from(1)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(2)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(3)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(4)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(5)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(6)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(6)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(7)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(7)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(8)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4170), Val::from(8)],
                                                            )?;
                                                        }
                                                        if !matched15 && subject15.loosely_equals(&Val::from(9)) {
                                                            matched15 = true;
                                                        }
                                                        if matched15 {
                                                            break 'b15;
                                                        }
                                                    }
                                                } else if (ctx.var("ep13_yong1").get()?.number()? > 249
                                                    && ctx.var("ep13_yong1").get()?.number()? < 300)
                                                {
                                                    'b16: {
                                                        let subject16 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 4025z:Prontera -> 4025z:Izlude -> 4025z:Geffen -> 4025z:Payon -> 4025z:Morocc -> 4025z:Al De Baran -> 4025z:Juno -> 4025z:Einbroch -> 4025z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched16 = false;
                                                        let no_case16 = !subject16.loosely_equals(&Val::from(1))
                                                            && !subject16.loosely_equals(&Val::from(2))
                                                            && !subject16.loosely_equals(&Val::from(3))
                                                            && !subject16.loosely_equals(&Val::from(4))
                                                            && !subject16.loosely_equals(&Val::from(5))
                                                            && !subject16.loosely_equals(&Val::from(6))
                                                            && !subject16.loosely_equals(&Val::from(7))
                                                            && !subject16.loosely_equals(&Val::from(8))
                                                            && !subject16.loosely_equals(&Val::from(9))
                                                            && !subject16.loosely_equals(&Val::from(10));
                                                        if !matched16 && subject16.loosely_equals(&Val::from(1)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(2)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(3)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(4)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(5)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(6)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(6)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(7)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(7)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(8)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(8)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(9)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(4025), Val::from(9)],
                                                            )?;
                                                        }
                                                        if !matched16 && subject16.loosely_equals(&Val::from(10)) {
                                                            matched16 = true;
                                                        }
                                                        if matched16 {
                                                            break 'b16;
                                                        }
                                                    }
                                                } else if ctx.var("ep13_yong1").get()?.number()? > 299 {
                                                    'b17: {
                                                        let subject17 = Val::from(runtime::select_values(
                                                            ctx,
                                                            &[Val::from(
                                                                "Alberta -> 3970z:Prontera -> 3970z:Izlude -> 3970z:Geffen -> 3970z:Payon -> 3970z:Morocc -> 3970z:Al De Baran -> 3970z:Juno -> 3970z:Einbroch -> 3970z:Lighthalzen -> 3970z:Cancel",
                                                            )],
                                                        )?);
                                                        let mut matched17 = false;
                                                        let no_case17 = !subject17.loosely_equals(&Val::from(1))
                                                            && !subject17.loosely_equals(&Val::from(2))
                                                            && !subject17.loosely_equals(&Val::from(3))
                                                            && !subject17.loosely_equals(&Val::from(4))
                                                            && !subject17.loosely_equals(&Val::from(5))
                                                            && !subject17.loosely_equals(&Val::from(6))
                                                            && !subject17.loosely_equals(&Val::from(7))
                                                            && !subject17.loosely_equals(&Val::from(8))
                                                            && !subject17.loosely_equals(&Val::from(9))
                                                            && !subject17.loosely_equals(&Val::from(10))
                                                            && !subject17.loosely_equals(&Val::from(11));
                                                        if !matched17 && subject17.loosely_equals(&Val::from(1)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(1)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(2)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(2)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(3)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(3)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(4)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(4)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(5)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(5)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(6)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(6)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(7)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(7)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(8)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(8)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(9)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(9)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(10)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            cat_paw_agent_run(
                                                                ctx,
                                                                CatPawAgentStep::Catwarp,
                                                                vec![Val::from(3970), Val::from(10)],
                                                            )?;
                                                        }
                                                        if !matched17 && subject17.loosely_equals(&Val::from(11)) {
                                                            matched17 = true;
                                                        }
                                                        if matched17 {
                                                            break 'b17;
                                                        }
                                                    }
                                                } else {
                                                    ctx.lines_as("Cat Paw Agent", args!["I'm sorry, but you're not eligible to use the warp service. Please check your points, and then come back."])?;
                                                }
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            4 => {
                                                'b18: {
                                                    let subject18 = Val::from(runtime::select_values(
                                                        ctx,
                                                        &[Val::from("Splendide Camp -> 5500z:Manuk Camp -> 5500z:Cancel")],
                                                    )?);
                                                    let mut matched18 = false;
                                                    let no_case18 = !subject18.loosely_equals(&Val::from(1))
                                                        && !subject18.loosely_equals(&Val::from(2))
                                                        && !subject18.loosely_equals(&Val::from(3));
                                                    if !matched18 && subject18.loosely_equals(&Val::from(1)) {
                                                        matched18 = true;
                                                    }
                                                    if matched18 {
                                                        cat_paw_agent_run(
                                                            ctx,
                                                            CatPawAgentStep::Catwarp,
                                                            vec![Val::from(5500), Val::from(11)],
                                                        )?;
                                                    }
                                                    if !matched18 && subject18.loosely_equals(&Val::from(2)) {
                                                        matched18 = true;
                                                    }
                                                    if matched18 {
                                                        cat_paw_agent_run(
                                                            ctx,
                                                            CatPawAgentStep::Catwarp,
                                                            vec![Val::from(5500), Val::from(12)],
                                                        )?;
                                                    }
                                                    if !matched18 && subject18.loosely_equals(&Val::from(3)) {
                                                        matched18 = true;
                                                    }
                                                    if matched18 {
                                                        ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                }
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            5 => {
                                                ctx.lines_as("Cat Paw Agent", args!["Thank you for using our service."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        ctx.lines_as("Cat Paw Agent", args!["*Yawn...*", "I want to eat fish."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                        }
                    }
                }
                step = CatPawAgentStep::AfterCatwarp;
                continue 'machine;
            }
            CatPawAgentStep::Catwarp => {
                if runtime::op(&ctx.var("Zeny").get()?, "<", &runtime::arg(&args, 0, Val::from(0)))?.is_true() {
                    ctx.lines_as("Cat Paw Agent", args!["Don't play with money."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.close_window()?;
                ctx.var("Zeny")
                    .set((ctx.var("Zeny").get()?.try_sub(runtime::arg(&args, 0, Val::from(0)))?))?;
                let subject19 = runtime::arg(&args, 1, Val::from(0));
                if subject19 == 1 {
                    ctx.call(Function::Warp, vec![Val::from("alberta"), Val::from(117), Val::from(56)])?;
                    return Err(Stop::End);
                } else if subject19 == 2 {
                    ctx.call(Function::Warp, vec![Val::from("prontera"), Val::from(116), Val::from(72)])?;
                    return Err(Stop::End);
                } else if subject19 == 3 {
                    ctx.call(Function::Warp, vec![Val::from("izlude"), Val::from(91), Val::from(105)])?;
                    return Err(Stop::End);
                } else if subject19 == 4 {
                    ctx.call(Function::Warp, vec![Val::from("geffen"), Val::from(120), Val::from(39)])?;
                    return Err(Stop::End);
                } else if subject19 == 5 {
                    ctx.call(Function::Warp, vec![Val::from("payon"), Val::from(161), Val::from(58)])?;
                    return Err(Stop::End);
                } else if subject19 == 6 {
                    ctx.call(Function::Warp, vec![Val::from("morocc"), Val::from(156), Val::from(46)])?;
                    return Err(Stop::End);
                } else if subject19 == 7 {
                    ctx.call(Function::Warp, vec![Val::from("aldebaran"), Val::from(168), Val::from(112)])?;
                    return Err(Stop::End);
                } else if subject19 == 8 {
                    ctx.call(Function::Warp, vec![Val::from("yuno"), Val::from(158), Val::from(125)])?;
                    return Err(Stop::End);
                } else if subject19 == 9 {
                    ctx.call(Function::Warp, vec![Val::from("einbroch"), Val::from(158), Val::from(301)])?;
                    return Err(Stop::End);
                } else if subject19 == 10 {
                    ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(163), Val::from(64)])?;
                    return Err(Stop::End);
                } else if subject19 == 11 {
                    ctx.call(Function::Warp, vec![Val::from("spl_fild02"), Val::from(32), Val::from(225)])?;
                    return Err(Stop::End);
                } else if subject19 == 12 {
                    ctx.call(Function::Warp, vec![Val::from("man_fild02"), Val::from(129), Val::from(61)])?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
            CatPawAgentStep::AfterCatwarp => {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn cat_paw_agent(ctx: &Ctx) -> Script {
    cat_paw_agent_run(ctx, CatPawAgentStep::Start, Vec::new()).map(|_| ())
}

fn fluffy_gyaruk_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ep13_yong1").get()?.number()? < 1 {
        ctx.lines_as("Fluffy Gyaruk", args!["*Sniff Sniff* Can't you smell fish", "around here?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("ep13_yong1").get()? == 1 {
        ctx.lines_as("Fluffy Gyaruk", args!["Hmm?", "Oh, did the Cat Paw Agent send you?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Yes.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["Oh, okay...", "Well, where should I begin.", "(Mumble Mumble)"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What do you guys do?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "Oh, sure.",
                "I can start from there.",
                "You used Kafra Corporation services on the Midgard Continent, didn't you?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Yes.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["Cat Trading is a new trading company based on this undeveloped continent."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "Well, frankly we wanted to join the Midgard Market, but it was too competitive...",
                "Not only that, they don't have enough fish for all of us either... (Mumble Mumble)"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What services do you offer?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["*Purr* Oh, our service range is similar to the Kafra's Location Saving, Storage Service, Warp Service, and more."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["The only difference is that, our services are limited depending on the customer's credit."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Credit?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "As you might know already, we're a considerably small company struggling to make ends meet in this niche market.",
                "We desperately need your support to grow."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["You can increase your credit by helping us secure our food or by collecting minerals."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "The more credits you earn, the more services you can use. You can also apply for special events.",
                "Say, how'd you like to make a membership contract with us?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sure.:I need to think it over.")])? {
            1 => {
                ctx.lines_as(
                    "Fluffy Gyaruk",
                    args![
                        "Excellent! Thank you for joining the Cat Trading membership service.",
                        "We hope you'll support our exploration of this unknown land."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Fluffy Gyaruk",
                    args![
                        "Please go speak to the Cat Paw Agent over there to use a basic service.",
                        "Oh, and please come back afterwards."
                    ],
                )?;
                ctx.var("ep13_yong1").set(Val::from(2))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Fluffy Gyaruk", args!["How disappointing!", "But I understand."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("ep13_yong1").get()? == 3 {
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "We offer one service to new customers.",
                "If you want to use more services, you must do some things for us."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What kind of things?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "For now...",
                "Please catch the main dish of the Cat Trading employees, fish, and collect minerals once every day.",
                "Your credit will increase each day if you do those things."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["Of course, we'll reward you with a reasonable amount of EXP for the services."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![
                "This is a good opportunity to increase your credit and EXP ate the same time.",
                "Doesn't that sound like a win-win situation?",
                "Would you like to start now?"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("Sure.:I need to go prepare first.")])? {
            1 => {
                ctx.lines_as("Fluffy Gyaruk", args!["Excellent.", "For more information about fishing and mining, please speak to the two Cat Agents standing at the river over there."])?;
                ctx.var("ep13_yong1").set(Val::from(4))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as("Fluffy Gyaruk", args!["No problem.", "Please take your time."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if ctx.var("ep13_yong1").get()? == 4 {
        ctx.lines_as(
            "Fluffy Gyaruk",
            args!["For more information about fishing and mining, please speak to the two Cat Agents standing at the river over there."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Fluffy Gyaruk",
            args![((Val::from("Your current credit points with us are ^0000FF") + ctx.var("ep13_yong1").get()?) + Val::from("^000000."))],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("How can I increase my points?:I'm not interested.")])? {
            1 => {
                ctx.lines_as("Cat Paw Agent", args!["You can increase them daily by fishing and mining, you need to submit caught Pieces of Fish to Gorurung and minerals to the Mining Agent."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn fluffy_gyaruk(ctx: &Ctx) -> Script {
    fluffy_gyaruk_body(ctx, Vec::new()).map(|_| ())
}

fn ferocious_gorurug_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckQuest, vec![Val::from(12060), ctx.constant("PLAYTIME")?])? == -1 {
        if ctx.var("ep13_yong1").get()?.number()? < 4 {
            ctx.lines_as("Ferocious Gorurug", args!["Grrr...."])?;
            ctx.next()?;
            ctx.lines(args!["A cat purring like a lion", "is looking into the water."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_yong1").get()? == 4 {
            ctx.lines_as("Ferocious Gorurug", args!["I'm busy.", "I need to catch fish."])?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("Gyaruk has sent me.")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Ferocious Gorurug",
                args!["Gyaruk sent you?", "Oh, you are here to fish? *Purr*"],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("What should I do?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Ferocious Gorurug",
                args![
                    "You need to catch fish. There are schools of fish in the waters.",
                    "It's hard to see them but if you use your cursor to click around, you should be able to catch them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ferocious Gorurug",
                args![
                    "It's easier to fish in the water outside the town.",
                    "If you're not afraid of monsters, you can go fishing over there.",
                    "*Purr*"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ferocious Gorurug",
                args![
                    "Now, let me teach you how to fish.",
                    "It's simple: bring your cursor to a school of fish, and then click it to grab them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ferocious Gorurug",
                args![
                    "You'll have a hard time to catch them at first, but you'll get better.",
                    "It gets easier with a bit of practice."
                ],
            )?;
            ctx.var("ep13_yong1").set(Val::from(5))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("ep13_yong1").get()? == 5 {
            if ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? > 9 {
                ctx.lines_as("Ferocious Gorurug", args!["You've brough Pieces of Fish!", "GOOD JOB!"])?;
                ctx.call(Function::DelItem, vec![Val::from(6039), Val::from(10)])?;
                ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
                ctx.var("ep13_yong1").set(Val::from(10))?;
                ctx.call(Function::SetQuest, vec![Val::from(12060)])?;
                ctx.next()?;
                ctx.lines(args![
                    ((Val::from("^0000ffYou gain EXP ") + Val::from("30,000")) + Val::from("^000000"))
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    "Ferocious Gorurug",
                    args![
                        "Bring your cursor to a school of fish, then click it to grab them.",
                        "Don't move while doing it!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            if (ctx.var("ep13_yong1").get()?.number()? > 5 && ctx.var("ep13_yong1").get()?.number()? < 20) {
                if ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? > 9 {
                    ctx.lines_as("Ferocious Gorurug", args!["You've brough Pieces of Fish!", "GOOD JOB!"])?;
                    ctx.call(Function::DelItem, vec![Val::from(6039), Val::from(10)])?;
                    ctx.call(Function::GetExperience, vec![Val::from(15000), Val::from(0)])?;
                    ctx.call(Function::SetQuest, vec![Val::from(12060)])?;
                    ctx.var("ep13_yong1").set((ctx.var("ep13_yong1").get()? + Val::from(1)))?;
                    ctx.next()?;
                    ctx.mes("^0000ffYou gain EXP 15,000^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Ferocious Gorurug",
                        args![
                            "Do you want to fish again?",
                            "Don't forget to bring me",
                            "Pieces of Fish if you catch them."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if (ctx.var("ep13_yong1").get()?.number()? > 19 && ctx.var("ep13_yong1").get()?.number()? < 40) {
                    if ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? > 9 {
                        ctx.lines_as("Ferocious Gorurug", args!["You've brough Pieces of Fish!", "GOOD JOB!"])?;
                        ctx.call(Function::DelItem, vec![Val::from(6039), Val::from(10)])?;
                        ctx.call(Function::GetExperience, vec![Val::from(15000), Val::from(0)])?;
                        ctx.call(Function::SetQuest, vec![Val::from(12060)])?;
                        ctx.var("ep13_yong1").set((ctx.var("ep13_yong1").get()? + Val::from(1)))?;
                        ctx.next()?;
                        ctx.mes("^0000ffYou gain EXP 15,000^000000")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Ferocious Gorurug",
                            args!["Hey, I've been waiting for you!", "Good luck fishing today!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if (ctx.var("ep13_yong1").get()?.number()? > 39 && ctx.var("ep13_yong1").get()?.number()? < 60) {
                        if ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? > 9 {
                            ctx.lines_as("Ferocious Gorurug", args!["You've brough Pieces of Fish!", "GOOD JOB!"])?;
                            ctx.call(Function::DelItem, vec![Val::from(6039), Val::from(10)])?;
                            ctx.call(Function::GetExperience, vec![Val::from(15000), Val::from(0)])?;
                            ctx.call(Function::SetQuest, vec![Val::from(12060)])?;
                            ctx.var("ep13_yong1").set((ctx.var("ep13_yong1").get()? + Val::from(1)))?;
                            ctx.next()?;
                            ctx.mes("^0000ffYou gain EXP 15,000^000000")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.mes("Gorurug looks happy to see you.")?;
                            ctx.next()?;
                            ctx.lines_as("Ferocious Gorurug", args!["What do you want to catch today? *Purr*"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("ep13_yong1").get()?.number()? > 59 {
                            runtime::npc_skill(ctx, &Val::from("AL_HEAL"), &Val::from(8), &Val::from(50), &Val::from(50))?;
                            if ctx.call(Function::CountItem, vec![Val::from(6039)])?.number()? > 9 {
                                ctx.lines_as("Ferocious Gorurug", args!["You've brough Pieces of Fish!", "GOOD JOB!"])?;
                                ctx.call(Function::DelItem, vec![Val::from(6039), Val::from(10)])?;
                                ctx.call(Function::GetExperience, vec![Val::from(15000), Val::from(0)])?;
                                ctx.call(Function::SetQuest, vec![Val::from(12060)])?;
                                ctx.var("ep13_yong1").set((ctx.var("ep13_yong1").get()? + Val::from(1)))?;
                                ctx.next()?;
                                ctx.mes("^0000ffYou gain EXP 1,500^000000")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.mes("Gorurung welcomes you with a happy purr.")?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Ferocious Gorurug",
                                    args!["Welcome!", "You're here to give me fishes, aren't you?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            ctx.mes("Ferocious Gorurug is asleep.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    } else {
        if (ctx.call(Function::CheckQuest, vec![Val::from(12060), ctx.constant("PLAYTIME")?])? == 0
            || ctx.call(Function::CheckQuest, vec![Val::from(12060), ctx.constant("PLAYTIME")?])? == 1)
        {
            ctx.lines_as(
                "Ferocious Gorurug",
                args![
                    "*Yawn*",
                    "I'm sorry, but I'm off-duty.",
                    "I can't accept any fish right now.",
                    "Come back tomorrow, alright?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.call(Function::EraseQuest, vec![Val::from(12060)])?;
            ctx.lines_as(
                "Ferocious Gorurug",
                args![
                    "*Purr*",
                    "Another day has started, back to work!",
                    "You can now bring me fish if you catch them."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn ferocious_gorurug(ctx: &Ctx) -> Script {
    ferocious_gorurug_body(ctx, Vec::new()).map(|_| ())
}
