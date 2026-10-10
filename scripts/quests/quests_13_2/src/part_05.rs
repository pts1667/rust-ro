use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn scientist_lifeguard_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_dayq_chk = Val::from(0);
    let mut l_rand_dayq_s: Vec<Val> = Vec::new();
    let mut l_stone_chk = Val::from(0);
    let mut l_stoneelse_chk = Val::from(0);
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        ctx.mes("[Dr. Lifeguard]")?;
        if ctx.var("ep13_2_tre").get()? == 0 {
            ctx.mes("Recently, it seems like my skin is becoming really dry.")?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args![
                    "I don't know if it's caused by the cold weather or our chronic disease getting worse.",
                    "It hasn't been too long since we administered the Bradium, so why."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args!["Huu... You see, we are afraid of such things... What a hard life."],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("A chronic disease?:Bradium?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args![
                    "We Sapha, who are descendants of Hwergelmir, survived and rebuilt the Giant tribe in the flood of the blood of Ymir."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Dr. Lifeguard", args!["For no apparent reason, we developed a chronic disease that makes our bodies dry and harden over time just like an old tree."])?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args!["I think that this is the curse of Ymir who bled by the gods?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args!["But we couldn't just wait for death to slowly creep up on us and exterminate our entire race."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args!["To heal this rare disease the smartest Sapha studied and studied."],
            )?;
            ctx.next()?;
            ctx.lines_as("Dr. Lifeguard", args!["What they found was that by injecting a refined Bradium, which is buried around Jotunheim, it can postpone the progression of the disease."])?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args!["So all of the giants of Sapha established their towns around mines to get Bradium easier."],
            )?;
            ctx.next()?;
            ctx.lines_as("Dr. Lifeguard", args!["It seems the amount of buried Bradium is enough.", "But for the eternal prosperity of our tribe, and to make the vaccine for this incurable disease, we have to study more and more."])?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args![
                    "Bradium is the stem of our life.",
                    "But the buried Bradium will be exhausted someday, so we Scientists have to find an alternative for our descendants."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args!["Recently, we've learned that there are various minerals in the human world Midgard."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args![
                    "Now the world called Midgard is",
                    "like a holy place for us,",
                    "which can bring the wind of",
                    "revolution for us."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Dr. Lifeguard",
                args![
                    "Contact with Midgard",
                    "must be the lead of Hwergelmir",
                    "for us, the Sapha which has such",
                    "a closed-off society."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Dr. Lifeguard", args!["I'll draw a result that can", "satisfy his lead!"])?;
            ctx.var("ep13_2_tre").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if (ctx.var("ep13_2_tre").get()? == 1 || ctx.var("ep13_2_tre").get()? == 100) {
            if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                ctx.lines(args![
                    "It looks like you're carrying too many things.",
                    "Why not put some of your items in storage and come back?"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            l_dayq_chk = ctx.call(Function::CheckQuest, vec![Val::from(2182), ctx.constant("PLAYTIME")?])?;
            if (l_dayq_chk.clone() == 0 || l_dayq_chk.clone() == 1) {
                ctx.lines(args![
                    "The future of the Sapha is up to you.",
                    "I do not have any need of your help right now."
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.call(Function::EraseQuest, vec![Val::from(2182)])?;
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject1 == 1 {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 0), Val::from("Mt. Mjolnir"), true);
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 1), Val::from("10"), true);
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 2), Val::from("2"), true);
                } else if subject1 == 2 {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 0), Val::from("Abyss Lake"), true);
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 1), Val::from("5"), true);
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 2), Val::from("3"), true);
                } else if subject1 == 3 {
                    let base = Val::from(0).number()?;
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 0), Val::from("Thor Volcano"), true);
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 1), Val::from("5"), true);
                    runtime::local_set(&mut l_rand_dayq_s, &Val::from(base + 2), Val::from("4"), true);
                }
                ctx.mes("Recently, someone who came from Schwarzwald gave me a small mineral as a gift from his home town when he saw our study of Bradium.")?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "I can't say for sure until now,",
                        "but this mineral seems to have",
                        "many interesting features."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "As I examine this mineral",
                        "more and more, it's really",
                        "different from the one in Jotunheim.",
                        "This mineral must be helpful",
                        "to make a new vaccine!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "But the amount of the mineral",
                        "is too little to make a",
                        "preparation for the",
                        "examination."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args!["You said you're from Midgard, right? Do you think you can bring me a little bit more of that mineral.."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "He said it's from",
                        ((Val::from("^FF0000") + runtime::local_get(&l_rand_dayq_s, &Val::from(0), true)) + Val::from("^000000.")),
                        ((Val::from("I think ^FF0000") + runtime::local_get(&l_rand_dayq_s, &Val::from(1), true))
                            + Val::from("^000000 would be enough."))
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "I'll give you this portable toolbox.",
                        "Things like a detector and a little hammer for mining minerals are included in this toolbox."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args!["Take it, it shold be", "helpful in collecting", "the minerals I need."],
                )?;
                ctx.var("ep13_2_tre")
                    .set(runtime::atoi(&runtime::local_get(&l_rand_dayq_s, &Val::from(2), true)))?;
                ctx.call(
                    Function::SetQuest,
                    vec![
                        (Val::from(2179)
                            + (runtime::atoi(&runtime::local_get(&l_rand_dayq_s, &Val::from(2), true)).try_sub(Val::from(2))?)),
                    ],
                )?;
                ctx.call(Function::GetItem, vec![Val::from(6076), Val::from(1)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ((ctx.var("ep13_2_tre").get()? == 2 || ctx.var("ep13_2_tre").get()? == 3) || ctx.var("ep13_2_tre").get()? == 4) {
            if ((ctx.var("ep13_2_tre").get()? == 2 && ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? > 9)
                || (ctx.var("ep13_2_tre").get()? != 2 && ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? > 4))
            {
                ctx.lines(args!["Oh! Yes, that's it!", "This much will be enough!"])?;
                ctx.next()?;
                ctx.lines_as("Dr. Lifeguard", args!["I really appreciate your help."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "These are coins of the Manuk.",
                        "This may not be much, but if you collect more, they can be used to buy some items from our merchants."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Dr. Lifeguard",
                    args![
                        "If you want to help my study for the sake of the Sapha, come to me at any time.",
                        "I will be very happy for any help you can give."
                    ],
                )?;
                l_stone_chk = ctx.call(Function::CountItem, vec![Val::from(6077)])?;
                l_stoneelse_chk = ctx.call(Function::CountItem, vec![Val::from(6078)])?;
                ctx.call(Function::DelItem, vec![Val::from(6077), l_stone_chk.clone()])?;
                ctx.call(Function::DelItem, vec![Val::from(6078), l_stoneelse_chk.clone()])?;
                ctx.call(Function::DelItem, vec![Val::from(6076), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(6080), Val::from(3)])?;
                ctx.call(
                    Function::ChangeQuest,
                    vec![
                        (Val::from(2179) + (ctx.var("ep13_2_tre").get()?.try_sub(Val::from(2))?)),
                        Val::from(2182),
                    ],
                )?;
                ctx.var("ep13_2_tre").set(Val::from(100))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines(args!["The future of the Sapha is up to you.", "I wish you good luck."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else {
            ctx.mes("Hmm...")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Scientist",
            args![
                "Pdh doud pjfdd",
                "fo pdi Wrdpp doff",
                "WosNuffremu Ha TurAshTi",
                "SeGothShar An AshDur",
                "UorVeLars No Ador"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn scientist_lifeguard(ctx: &Ctx) -> Script {
    scientist_lifeguard_body(ctx, Vec::new()).map(|_| ())
}

fn mjo_no_find_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mjo_no_find(ctx: &Ctx) -> Script {
    mjo_no_find_body(ctx, Vec::new()).map(|_| ())
}

fn mjo_no_find_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(0)])?;
    return Err(Stop::End);
}

pub fn mjo_no_find_ontouch(ctx: &Ctx) -> Script {
    mjo_no_find_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mjo_find_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_stst = Val::from(0);
    if (ctx.var("ep13_2_tre").get()? == 2 && ctx.call(Function::CountItem, vec![Val::from(6076)])? == 1) {
        if ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? < 10 {
            ctx.mes("- I can see some different colored minerals where the detector pointed to. This must be the mineral that Mr. Lifeguard told me about. -")?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Mine minerals.:Do nothing.")])? {
                1 => {
                    ctx.mes("Clang! Clang! Clang! Clang!")?;
                    ctx.close_window()?;
                    ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), Val::from(4)])?;
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?.number()? < 4 {
                        ctx.mes("- You failed to mine minerals. -")?;
                        ctx.call(Function::GetItem, vec![Val::from(6078), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("- You succeeded to mine minerals. -")?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_AHA")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        l_stst = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                        if l_stst.clone() == 5 {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(5)])?;
                        } else if l_stst.clone() == 1 {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(1)])?;
                        } else {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(2)])?;
                        }
                        ctx.close_window()?;
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_t::OnEnable"))],
                        )?;
                        ctx.call(Function::DisableNpc, vec![])?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.mes("- You decide to do nothing. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.mes("- I think I have enough minerals. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn mjo_find(ctx: &Ctx) -> Script {
    mjo_find_body(ctx, Vec::new()).map(|_| ())
}

fn mjo_find_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn mjo_find_oninit(ctx: &Ctx) -> Script {
    mjo_find_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn mjo_find_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("ep13_2_tre").get()? == 2 && ctx.call(Function::CountItem, vec![Val::from(6076)])? == 1)
        && ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? < 10)
    {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COUPLECASTING")?])?;
    }
    return Err(Stop::End);
}

pub fn mjo_find_ontouch(ctx: &Ctx) -> Script {
    mjo_find_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj01_find_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj01_find(ctx: &Ctx) -> Script {
    mj01_find_body(ctx, Vec::new()).map(|_| ())
}

fn mj01_find_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(32), Val::from(309), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(217), Val::from(34), Val::from(2), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj01_find_ontouch(ctx: &Ctx) -> Script {
    mj01_find_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj01_01_t_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj01_01_t(ctx: &Ctx) -> Script {
    mj01_01_t_body(ctx, Vec::new()).map(|_| ())
}

fn mj01_01_t_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn mj01_01_t_onenable(ctx: &Ctx) -> Script {
    mj01_01_t_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn mj01_01_t_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#mj01_01")])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn mj01_01_t_ontimer60000(ctx: &Ctx) -> Script {
    mj01_01_t_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn mj01_02_t_run(ctx: &Ctx, mut step: Mj0102TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj0102TStep::Start => {
                step = Mj0102TStep::OnEnable;
                continue 'machine;
            }
            Mj0102TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj0102TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj01_02")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj01_02_t(ctx: &Ctx) -> Script {
    mj01_02_t_run(ctx, Mj0102TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj01_02_t_onenable(ctx: &Ctx) -> Script {
    mj01_02_t_run(ctx, Mj0102TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj01_02_t_ontimer60000(ctx: &Ctx) -> Script {
    mj01_02_t_run(ctx, Mj0102TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn mj02_find_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj02_find_1(ctx: &Ctx) -> Script {
    mj02_find_1_body(ctx, Vec::new()).map(|_| ())
}

fn mj02_find_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(90), Val::from(195), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(206), Val::from(187), Val::from(2), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj02_find_1_ontouch(ctx: &Ctx) -> Script {
    mj02_find_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj02_find_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj02_find_2(ctx: &Ctx) -> Script {
    mj02_find_2_body(ctx, Vec::new()).map(|_| ())
}

fn mj02_find_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(90), Val::from(195), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(206), Val::from(187), Val::from(2), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj02_find_2_ontouch(ctx: &Ctx) -> Script {
    mj02_find_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj02_01_t_run(ctx: &Ctx, mut step: Mj0201TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj0201TStep::Start => {
                step = Mj0201TStep::OnEnable;
                continue 'machine;
            }
            Mj0201TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj0201TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj02_01")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj02_01_t(ctx: &Ctx) -> Script {
    mj02_01_t_run(ctx, Mj0201TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj02_01_t_onenable(ctx: &Ctx) -> Script {
    mj02_01_t_run(ctx, Mj0201TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj02_01_t_ontimer60000(ctx: &Ctx) -> Script {
    mj02_01_t_run(ctx, Mj0201TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn mj02_02_t_run(ctx: &Ctx, mut step: Mj0202TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj0202TStep::Start => {
                step = Mj0202TStep::OnEnable;
                continue 'machine;
            }
            Mj0202TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj0202TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj02_02")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj02_02_t(ctx: &Ctx) -> Script {
    mj02_02_t_run(ctx, Mj0202TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj02_02_t_onenable(ctx: &Ctx) -> Script {
    mj02_02_t_run(ctx, Mj0202TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj02_02_t_ontimer60000(ctx: &Ctx) -> Script {
    mj02_02_t_run(ctx, Mj0202TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn mj04_find_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj04_find_1(ctx: &Ctx) -> Script {
    mj04_find_1_body(ctx, Vec::new()).map(|_| ())
}

fn mj04_find_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(125), Val::from(380), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(220), Val::from(130), Val::from(2), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj04_find_1_ontouch(ctx: &Ctx) -> Script {
    mj04_find_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj04_find_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj04_find_2(ctx: &Ctx) -> Script {
    mj04_find_2_body(ctx, Vec::new()).map(|_| ())
}

fn mj04_find_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(125), Val::from(380), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(220), Val::from(130), Val::from(2), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj04_find_2_ontouch(ctx: &Ctx) -> Script {
    mj04_find_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj04_find_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj04_find_3(ctx: &Ctx) -> Script {
    mj04_find_3_body(ctx, Vec::new()).map(|_| ())
}

fn mj04_find_3_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(125), Val::from(380), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(220), Val::from(130), Val::from(2), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj04_find_3_ontouch(ctx: &Ctx) -> Script {
    mj04_find_3_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj04_01_t_run(ctx: &Ctx, mut step: Mj0401TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj0401TStep::Start => {
                step = Mj0401TStep::OnEnable;
                continue 'machine;
            }
            Mj0401TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj0401TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj04_01")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj04_01_t(ctx: &Ctx) -> Script {
    mj04_01_t_run(ctx, Mj0401TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj04_01_t_onenable(ctx: &Ctx) -> Script {
    mj04_01_t_run(ctx, Mj0401TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj04_01_t_ontimer60000(ctx: &Ctx) -> Script {
    mj04_01_t_run(ctx, Mj0401TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn mj04_02_t_run(ctx: &Ctx, mut step: Mj0402TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj0402TStep::Start => {
                step = Mj0402TStep::OnEnable;
                continue 'machine;
            }
            Mj0402TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj0402TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj04_02")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj04_02_t(ctx: &Ctx) -> Script {
    mj04_02_t_run(ctx, Mj0402TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj04_02_t_onenable(ctx: &Ctx) -> Script {
    mj04_02_t_run(ctx, Mj0402TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj04_02_t_ontimer60000(ctx: &Ctx) -> Script {
    mj04_02_t_run(ctx, Mj0402TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn mj09_find_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj09_find_1(ctx: &Ctx) -> Script {
    mj09_find_1_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(229), Val::from(214), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj09_find_1_ontouch(ctx: &Ctx) -> Script {
    mj09_find_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj09_find_2(ctx: &Ctx) -> Script {
    mj09_find_2_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(229), Val::from(214), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj09_find_2_ontouch(ctx: &Ctx) -> Script {
    mj09_find_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj09_find_3(ctx: &Ctx) -> Script {
    mj09_find_3_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_3_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(229), Val::from(214), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj09_find_3_ontouch(ctx: &Ctx) -> Script {
    mj09_find_3_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj09_find_4(ctx: &Ctx) -> Script {
    mj09_find_4_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_4_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(229), Val::from(214), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj09_find_4_ontouch(ctx: &Ctx) -> Script {
    mj09_find_4_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj09_find_5(ctx: &Ctx) -> Script {
    mj09_find_5_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_find_5_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(229), Val::from(214), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj09_find_5_ontouch(ctx: &Ctx) -> Script {
    mj09_find_5_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj09_01_t_run(ctx: &Ctx, mut step: Mj0901TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj0901TStep::Start => {
                step = Mj0901TStep::OnEnable;
                continue 'machine;
            }
            Mj0901TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj0901TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj09_01")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj09_01_t(ctx: &Ctx) -> Script {
    mj09_01_t_run(ctx, Mj0901TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj09_01_t_onenable(ctx: &Ctx) -> Script {
    mj09_01_t_run(ctx, Mj0901TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj09_01_t_ontimer60000(ctx: &Ctx) -> Script {
    mj09_01_t_run(ctx, Mj0901TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn mj10_find_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj10_find_1(ctx: &Ctx) -> Script {
    mj10_find_1_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(188), Val::from(260), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj10_find_1_ontouch(ctx: &Ctx) -> Script {
    mj10_find_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj10_find_2(ctx: &Ctx) -> Script {
    mj10_find_2_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(188), Val::from(260), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj10_find_2_ontouch(ctx: &Ctx) -> Script {
    mj10_find_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj10_find_3(ctx: &Ctx) -> Script {
    mj10_find_3_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_3_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(188), Val::from(260), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj10_find_3_ontouch(ctx: &Ctx) -> Script {
    mj10_find_3_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj10_find_4(ctx: &Ctx) -> Script {
    mj10_find_4_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_4_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(188), Val::from(260), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj10_find_4_ontouch(ctx: &Ctx) -> Script {
    mj10_find_4_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn mj10_find_5(ctx: &Ctx) -> Script {
    mj10_find_5_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_find_5_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(0), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(229), Val::from(214), Val::from(1), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn mj10_find_5_ontouch(ctx: &Ctx) -> Script {
    mj10_find_5_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn mj10_01_t_run(ctx: &Ctx, mut step: Mj1001TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Mj1001TStep::Start => {
                step = Mj1001TStep::OnEnable;
                continue 'machine;
            }
            Mj1001TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Mj1001TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#mj10_01")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mj10_01_t(ctx: &Ctx) -> Script {
    mj10_01_t_run(ctx, Mj1001TStep::Start, Vec::new()).map(|_| ())
}

pub fn mj10_01_t_onenable(ctx: &Ctx) -> Script {
    mj10_01_t_run(ctx, Mj1001TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn mj10_01_t_ontimer60000(ctx: &Ctx) -> Script {
    mj10_01_t_run(ctx, Mj1001TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn hu_find_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn hu_find_1(ctx: &Ctx) -> Script {
    hu_find_1_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(1), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(251), Val::from(345), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(134), Val::from(322), Val::from(2), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(72), Val::from(104), Val::from(3), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(65), Val::from(99), Val::from(4), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(323), Val::from(84), Val::from(5), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hu_find_1_ontouch(ctx: &Ctx) -> Script {
    hu_find_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn hu_find_2(ctx: &Ctx) -> Script {
    hu_find_2_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(1), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(251), Val::from(345), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(134), Val::from(322), Val::from(2), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(72), Val::from(104), Val::from(3), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(65), Val::from(99), Val::from(4), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(323), Val::from(84), Val::from(5), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hu_find_2_ontouch(ctx: &Ctx) -> Script {
    hu_find_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn hu_find_3(ctx: &Ctx) -> Script {
    hu_find_3_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_3_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(1), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(251), Val::from(345), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(134), Val::from(322), Val::from(2), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(72), Val::from(104), Val::from(3), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(65), Val::from(99), Val::from(4), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(323), Val::from(84), Val::from(5), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hu_find_3_ontouch(ctx: &Ctx) -> Script {
    hu_find_3_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn hu_find_4(ctx: &Ctx) -> Script {
    hu_find_4_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_4_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(1), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(251), Val::from(345), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(134), Val::from(322), Val::from(2), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(72), Val::from(104), Val::from(3), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(65), Val::from(99), Val::from(4), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(323), Val::from(84), Val::from(5), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hu_find_4_ontouch(ctx: &Ctx) -> Script {
    hu_find_4_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_stst = Val::from(0);
    if (ctx.var("ep13_2_tre").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(6076)])? == 1) {
        if ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? < 5 {
            ctx.mes("- I can see some different colored minerals where the detector pointed to. This must be the mineral that Mr. Lifeguard told me about. -")?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Mine minerals.:Do nothing.")])? {
                1 => {
                    ctx.mes("Clang! Clang! Clang! Clang!")?;
                    ctx.close_window()?;
                    ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), Val::from(4)])?;
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?.number()? < 4 {
                        ctx.mes("- You failed to mine minerals. -")?;
                        ctx.call(Function::GetItem, vec![Val::from(6078), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("- You succeeded to mine minerals. -")?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_AHA")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        l_stst = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                        if l_stst.clone() == 5 {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(5)])?;
                        } else if l_stst.clone() == 1 {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(2)])?;
                        } else {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(1)])?;
                        }
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_t::OnEnable"))],
                        )?;
                        ctx.call(Function::DisableNpc, vec![])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.mes("- You decide to do nothing. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.mes("- I think I have enough minerals. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn hu_find(ctx: &Ctx) -> Script {
    hu_find_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn hu_find_oninit(ctx: &Ctx) -> Script {
    hu_find_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn hu_find_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("ep13_2_tre").get()? == 3 && ctx.call(Function::CountItem, vec![Val::from(6076)])? == 1)
        && ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? < 5)
    {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COUPLECASTING")?])?;
    }
    return Err(Stop::End);
}

pub fn hu_find_ontouch(ctx: &Ctx) -> Script {
    hu_find_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn hu_01_t_run(ctx: &Ctx, mut step: Hu01TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hu01TStep::Start => {
                step = Hu01TStep::OnEnable;
                continue 'machine;
            }
            Hu01TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Hu01TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hu_01")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hu_01_t(ctx: &Ctx) -> Script {
    hu_01_t_run(ctx, Hu01TStep::Start, Vec::new()).map(|_| ())
}

pub fn hu_01_t_onenable(ctx: &Ctx) -> Script {
    hu_01_t_run(ctx, Hu01TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn hu_01_t_ontimer60000(ctx: &Ctx) -> Script {
    hu_01_t_run(ctx, Hu01TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn hu_02_t_run(ctx: &Ctx, mut step: Hu02TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hu02TStep::Start => {
                step = Hu02TStep::OnEnable;
                continue 'machine;
            }
            Hu02TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Hu02TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hu_02")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hu_02_t(ctx: &Ctx) -> Script {
    hu_02_t_run(ctx, Hu02TStep::Start, Vec::new()).map(|_| ())
}

pub fn hu_02_t_onenable(ctx: &Ctx) -> Script {
    hu_02_t_run(ctx, Hu02TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn hu_02_t_ontimer60000(ctx: &Ctx) -> Script {
    hu_02_t_run(ctx, Hu02TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn hu_03_t_run(ctx: &Ctx, mut step: Hu03TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hu03TStep::Start => {
                step = Hu03TStep::OnEnable;
                continue 'machine;
            }
            Hu03TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Hu03TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hu_03")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hu_03_t(ctx: &Ctx) -> Script {
    hu_03_t_run(ctx, Hu03TStep::Start, Vec::new()).map(|_| ())
}

pub fn hu_03_t_onenable(ctx: &Ctx) -> Script {
    hu_03_t_run(ctx, Hu03TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn hu_03_t_ontimer60000(ctx: &Ctx) -> Script {
    hu_03_t_run(ctx, Hu03TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn hu_04_t_run(ctx: &Ctx, mut step: Hu04TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hu04TStep::Start => {
                step = Hu04TStep::OnEnable;
                continue 'machine;
            }
            Hu04TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Hu04TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hu_04")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hu_04_t(ctx: &Ctx) -> Script {
    hu_04_t_run(ctx, Hu04TStep::Start, Vec::new()).map(|_| ())
}

pub fn hu_04_t_onenable(ctx: &Ctx) -> Script {
    hu_04_t_run(ctx, Hu04TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn hu_04_t_ontimer60000(ctx: &Ctx) -> Script {
    hu_04_t_run(ctx, Hu04TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn hu_05_t_run(ctx: &Ctx, mut step: Hu05TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Hu05TStep::Start => {
                step = Hu05TStep::OnEnable;
                continue 'machine;
            }
            Hu05TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Hu05TStep::OnTimer60000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#hu_05")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hu_05_t(ctx: &Ctx) -> Script {
    hu_05_t_run(ctx, Hu05TStep::Start, Vec::new()).map(|_| ())
}

pub fn hu_05_t_onenable(ctx: &Ctx) -> Script {
    hu_05_t_run(ctx, Hu05TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn hu_05_t_ontimer60000(ctx: &Ctx) -> Script {
    hu_05_t_run(ctx, Hu05TStep::OnTimer60000, Vec::new()).map(|_| ())
}

fn ve_find_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn ve_find_1(ctx: &Ctx) -> Script {
    ve_find_1_body(ctx, Vec::new()).map(|_| ())
}

fn ve_find_1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(2), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(152), Val::from(134), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(256), Val::from(228), Val::from(2), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(85), Val::from(189), Val::from(3), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(282), Val::from(268), Val::from(4), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ve_find_1_ontouch(ctx: &Ctx) -> Script {
    ve_find_1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn ve_find_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn ve_find_2(ctx: &Ctx) -> Script {
    ve_find_2_body(ctx, Vec::new()).map(|_| ())
}

fn ve_find_2_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::quests_quests_13_2::find_13_2(ctx, vec![Val::from(2), Val::from(1)])?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(152), Val::from(134), Val::from(1), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(256), Val::from(228), Val::from(2), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(85), Val::from(189), Val::from(3), Val::from(65280)],
    )?;
    ctx.call(
        Function::ViewPoint,
        vec![Val::from(1), Val::from(282), Val::from(268), Val::from(4), Val::from(65280)],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ve_find_2_ontouch(ctx: &Ctx) -> Script {
    ve_find_2_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn ve_find_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_stst = Val::from(0);
    if (ctx.var("ep13_2_tre").get()? == 4 && ctx.call(Function::CountItem, vec![Val::from(6076)])? == 1) {
        if ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? < 5 {
            ctx.mes("- I can see some different colored minerals where the detector pointed to. This must be the mineral that Mr. Lifeguard told me about. -")?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Mine minerals.:Do nothing.")])? {
                1 => {
                    ctx.mes("Clang! Clang! Clang! Clang!")?;
                    ctx.close_window()?;
                    ctx.call(Function::ProgressBar, vec![Val::from("ffff00"), Val::from(4)])?;
                    if ctx.call(Function::Rand, vec![Val::from(1), Val::from(6)])?.number()? < 4 {
                        ctx.mes("- You failed to mine minerals. -")?;
                        ctx.call(Function::GetItem, vec![Val::from(6078), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("- You succeeded to mine minerals. -")?;
                        ctx.call(
                            Function::Emotion,
                            vec![
                                ctx.constant("ET_AHA")?,
                                Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                            ],
                        )?;
                        l_stst = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                        if l_stst.clone() == 5 {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(5)])?;
                        } else if l_stst.clone() == 1 {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(2)])?;
                        } else {
                            ctx.call(Function::GetItem, vec![Val::from(6077), Val::from(1)])?;
                        }
                        ctx.call(
                            Function::DoNpcEvent,
                            vec![(ctx.call(Function::StrNpcInfo, vec![Val::from(0)])? + Val::from("_t::OnEnable"))],
                        )?;
                        ctx.call(Function::DisableNpc, vec![])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                2 => {
                    ctx.mes("- You decide to do nothing. -")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.mes("- I think I have enough minerals. -")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    return Err(Stop::End);
}

pub fn ve_find(ctx: &Ctx) -> Script {
    ve_find_body(ctx, Vec::new()).map(|_| ())
}

fn ve_find_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![])?;
    return Err(Stop::End);
}

pub fn ve_find_oninit(ctx: &Ctx) -> Script {
    ve_find_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn ve_find_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.var("ep13_2_tre").get()? == 4 && ctx.call(Function::CountItem, vec![Val::from(6076)])? == 1)
        && ctx.call(Function::CountItem, vec![Val::from(6077)])?.number()? < 5)
    {
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_COUPLECASTING")?])?;
    }
    return Err(Stop::End);
}

pub fn ve_find_ontouch(ctx: &Ctx) -> Script {
    ve_find_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn ve_01_t_run(ctx: &Ctx, mut step: Ve01TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ve01TStep::Start => {
                step = Ve01TStep::OnEnable;
                continue 'machine;
            }
            Ve01TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ve01TStep::OnTimer40000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#ve_01")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ve_01_t(ctx: &Ctx) -> Script {
    ve_01_t_run(ctx, Ve01TStep::Start, Vec::new()).map(|_| ())
}

pub fn ve_01_t_onenable(ctx: &Ctx) -> Script {
    ve_01_t_run(ctx, Ve01TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ve_01_t_ontimer40000(ctx: &Ctx) -> Script {
    ve_01_t_run(ctx, Ve01TStep::OnTimer40000, Vec::new()).map(|_| ())
}

fn ve_02_t_run(ctx: &Ctx, mut step: Ve02TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ve02TStep::Start => {
                step = Ve02TStep::OnEnable;
                continue 'machine;
            }
            Ve02TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ve02TStep::OnTimer40000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#ve_02")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ve_02_t(ctx: &Ctx) -> Script {
    ve_02_t_run(ctx, Ve02TStep::Start, Vec::new()).map(|_| ())
}

pub fn ve_02_t_onenable(ctx: &Ctx) -> Script {
    ve_02_t_run(ctx, Ve02TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ve_02_t_ontimer40000(ctx: &Ctx) -> Script {
    ve_02_t_run(ctx, Ve02TStep::OnTimer40000, Vec::new()).map(|_| ())
}

fn ve_03_t_run(ctx: &Ctx, mut step: Ve03TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ve03TStep::Start => {
                step = Ve03TStep::OnEnable;
                continue 'machine;
            }
            Ve03TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ve03TStep::OnTimer40000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#ve_03")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ve_03_t(ctx: &Ctx) -> Script {
    ve_03_t_run(ctx, Ve03TStep::Start, Vec::new()).map(|_| ())
}

pub fn ve_03_t_onenable(ctx: &Ctx) -> Script {
    ve_03_t_run(ctx, Ve03TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ve_03_t_ontimer40000(ctx: &Ctx) -> Script {
    ve_03_t_run(ctx, Ve03TStep::OnTimer40000, Vec::new()).map(|_| ())
}

fn ve_04_t_run(ctx: &Ctx, mut step: Ve04TStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ve04TStep::Start => {
                step = Ve04TStep::OnEnable;
                continue 'machine;
            }
            Ve04TStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Ve04TStep::OnTimer40000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("#ve_04")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ve_04_t(ctx: &Ctx) -> Script {
    ve_04_t_run(ctx, Ve04TStep::Start, Vec::new()).map(|_| ())
}

pub fn ve_04_t_onenable(ctx: &Ctx) -> Script {
    ve_04_t_run(ctx, Ve04TStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn ve_04_t_ontimer40000(ctx: &Ctx) -> Script {
    ve_04_t_run(ctx, Ve04TStep::OnTimer40000, Vec::new()).map(|_| ())
}

fn high_laphine_grenouille_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_dayq_chk = Val::from(0);
    let mut l_ttalk = Val::from(0);
    if (ctx.call(Function::IsEquipped, vec![Val::from(2782)])? == 1 && ctx.var("ep13_2_rhea").get()? == 100) {
        if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
            ctx.lines_as(
                "Grenouille",
                args![
                    "It looks like you're carrying too many things.",
                    "Why not put some of your items in storage and come back?"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        l_dayq_chk = ctx.call(Function::CheckQuest, vec![Val::from(2186), ctx.constant("PLAYTIME")?])?;
        if (l_dayq_chk.clone() == -1 || l_dayq_chk.clone() == 2) {
            if (ctx.var("ep13_2_tre1").get()?.number()? < 1 || ctx.var("ep13_2_tre1").get()? == 6) {
                if l_dayq_chk.clone() == 2 {
                    ctx.call(Function::EraseQuest, vec![Val::from(2186)])?;
                }
                ctx.var("ep13_2_tre1").set(Val::from(0))?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "I've heard that some strangers from Midgard are around here, it must be you.",
                        "I'm Grenouille, a scientist."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "It's been a long time since",
                        "the Laphine built the Splendide camp here",
                        "and confronted each other."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "A number of soldiers who",
                        "left their hometown Alfheim are",
                        "restraining them with fighting against",
                        "the harsh cold of Jotunheim",
                        "for the holy duty and the faith to",
                        "stop the evil attempts to",
                        "sickening Yggdrasil."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "They're well trained soldiers,",
                        "and their captains hold them in high esteem,",
                        "so they're enjoying a",
                        "comfortable life there."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args!["But there's a saying that", "constant dripping water", "wears away the stone."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "Continuous small battles without big progression",
                        "makes the soldiers in Splendide exhausted."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "One day, someone began to sing",
                        "a song of Alfheim, that spread rapidly...",
                        "Now they strongly",
                        "miss their hometown."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "I was so worried about them and",
                        "I finally found a way to",
                        "save them from the deep sadness.",
                        "I hope you to help me to do that."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args!["There's a Laphine General", "who is called Flowery,", "in Splendide."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "She is a collecting maniac,",
                        "so her house is always",
                        "in a mess with miscellaneous",
                        "things. Flowery must have",
                        "seeds from the flowers",
                        "of Alfheim."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "I'll make the flower of",
                        "Alfheim open with the seed.",
                        "The flower of their hometown."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "There are some devices that purify",
                        "the environment and have a function",
                        "that can hasten the growth of plants."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args!["Please bring me the flower of Alfheim that bloomed by the device."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grenouille",
                    args![
                        "The flower of Alfheim is not",
                        "strong enough to keep it's life by",
                        "depending on the Splendide device,",
                        "but ^FF0000the way^000000 can fill",
                        "Splendide by it's scent like it never falls!"
                    ],
                )?;
                ctx.var("ep13_2_tre1").set(Val::from(1))?;
                ctx.call(Function::SetQuest, vec![Val::from(2183)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("ep13_2_tre1").get()? == 1 || ctx.var("ep13_2_tre1").get()? == 2) {
                ctx.lines_as(
                    "Grenouille",
                    args!["Get a seed from Flowery,", "and then use a Purifier", "to make the seed blossom."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if ctx.var("ep13_2_tre1").get()? == 3 {
                if ctx.call(Function::CountItem, vec![Val::from(6079)])? == 1 {
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SWEAT")?])?;
                    ctx.call(
                        Function::Emotion,
                        vec![
                            ctx.constant("ET_SWEAT")?,
                            Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                        ],
                    )?;
                    ctx.lines_as("Grenouille", args!["Hoo, this is one of the", "most common kinds of flowers."])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                    ctx.lines_as(
                        "Grenouille",
                        args![
                            "Haha, Don't worry.",
                            "Common flowers like this can bring the familiarity of home back to the soldiers."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grenouille",
                        args![
                            "For now, I'll extract a liquid that",
                            "includes the scent of this flower by using a distiller.",
                            "Making a ^3131FFPerfume^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Grenouille", args!["Would you please wait a while?"])?;
                    ctx.call(Function::ChangeQuest, vec![Val::from(2183), Val::from(2184)])?;
                    ctx.var("ep13_2_tre1").set(Val::from(4))?;
                    ctx.call(Function::DelItem, vec![Val::from(6079), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("Grenouille", args!["Where is the flower?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                if ctx.var("ep13_2_tre1").get()? == 4 {
                    l_ttalk = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
                    if l_ttalk.clone().number()? < 8 {
                        ctx.mes("- clitter-clatter -")?;
                        ctx.next()?;
                        ctx.lines(args!["- clitter-clatter -", "- clitter-clatter -"])?;
                        ctx.next()?;
                        ctx.lines(args!["- clitter-clatter -", "- clitter-clatter -", "- Spraying something -"])?;
                        ctx.next()?;
                        ctx.lines_as("Grenouille", args!["Not finished yet.", "Please wait a second."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.mes("- clitter-clatter -")?;
                        ctx.next()?;
                        ctx.lines(args!["- clitter-clatter -", "- clitter-clatter -"])?;
                        ctx.next()?;
                        ctx.lines(args!["- clitter-clatter -", "- clitter-clatter -", "- Spraying something -"])?;
                        ctx.next()?;
                        ctx.lines_as("Grenouille", args!["Wow, finished!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grenouille",
                            args!["I think I'll call this perfume", "The ^3131FFSoul of Alfheim^000000!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grenouille",
                            args![
                                "This perfume can relieve their minds by using",
                                "the scent of their hometown Alfheim.",
                                "Though it's not a permanent solution, I think it will help them feel comfortable for now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grenouille",
                            args![
                                "Here, I'll give you this perfume,",
                                "help the exhausted soldiers and",
                                "let me know the result, please!"
                            ],
                        )?;
                        ctx.var("ep13_2_tre1").set(Val::from(5))?;
                        ctx.call(Function::GetItem, vec![Val::from(6082), Val::from(5)])?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(2184), Val::from(2185)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else if ctx.var("ep13_2_tre1").get()? == 5 {
                    if ctx.call(Function::CountItem, vec![Val::from(6082)])?.number()? < 1 {
                        ctx.lines_as("Grenouille", args!["Oh! How's the reaction of the soldiers?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args![
                                "Yes, they seemed to be happy.",
                                "But, it still made someone cry because he probably misses Alfheim."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Grenouille", args!["Oh! That means it works!", "Thanks."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grenouille",
                            args![
                                "Thanks for your help.",
                                "This is my reward for you.",
                                "I hope this will be helpful to you.",
                                "If you have something to talk to me about, come to me whenever. Nice to meet you, human from Midgard."
                            ],
                        )?;
                        ctx.var("ep13_2_tre1").set(Val::from(6))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(2185), Val::from(2186)])?;
                        ctx.call(Function::GetItem, vec![Val::from(6081), Val::from(3)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Grenouille",
                            args!["Go and spray this perfume for the exhausted soldiers in Splendide!"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines_as("Grenouille", args!["I hope this long battle will end soon."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            ctx.lines_as("Grenouille", args!["I hope this long battle will end soon."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "High Laphine",
            args!["BurYurDath Ee NeKoIyaz er ModRuAlah Yee VeldDuDur Yee SeYurOsa U ReNudNud Ra "],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "High Laphine",
            args!["DiebVrumDana er TingNothDim No RasVilHir Yu YurAdorShar Yu DanaVeldDur Ha VilModMe O SharThus"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn high_laphine_grenouille(ctx: &Ctx) -> Script {
    high_laphine_grenouille_body(ctx, Vec::new()).map(|_| ())
}
