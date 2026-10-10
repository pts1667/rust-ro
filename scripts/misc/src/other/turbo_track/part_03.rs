use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guide_tbt_n1_ontimer60000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Guide#TBT_") + l_w_s.clone())])?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("#cos_") + l_w_s.clone()) + Val::from("_end"))],
    )?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker1#TBT_") + l_w_s.clone())])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker3#TBT_") + l_w_s.clone())])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker4#TBT_") + l_w_s.clone())])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Disposable_Switch#") + l_w_s.clone())])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Flasher_Exit_1#") + l_w_s.clone())])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Flasher_Exit_2#") + l_w_s.clone())])?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-1"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("No_Unfair_Start#") + l_w_s.clone()) + Val::from("-2"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("snake#") + l_w_s.clone()) + Val::from("::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("hunting#") + l_w_s.clone()) + Val::from("::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Solo Mode#") + l_w_s.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_ontimer60000(ctx: &Ctx) -> Script {
    guide_tbt_n1_ontimer60000_body(ctx, Vec::new()).map(|_| ())
}

fn guide_tbt_n1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Guide#TBT_") + l_w_s.clone())])?;
    return Err(Stop::End);
}

pub fn guide_tbt_n1_oninit(ctx: &Ctx) -> Script {
    guide_tbt_n1_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nmaker1TtMainStep {
    Start,
    OnTouch,
    OnInit,
}

fn nmaker1_tt_main_run(ctx: &Ctx, mut step: Nmaker1TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            Nmaker1TtMainStep::Start => {
                step = Nmaker1TtMainStep::OnTouch;
                continue 'machine;
            }
            Nmaker1TtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        (((ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" is now entering the Small Cave! "))
                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(" is now in the lead!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(
                    Function::DoNpcEvent,
                    vec![(Val::from("Turbo Track Guide::OnEnd_") + l_w_s.clone())],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Notice_Maker1#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
            Nmaker1TtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker1#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nmaker1_tt_main(ctx: &Ctx) -> Script {
    nmaker1_tt_main_run(ctx, Nmaker1TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn nmaker1_tt_main_ontouch(ctx: &Ctx) -> Script {
    nmaker1_tt_main_run(ctx, Nmaker1TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn nmaker1_tt_main_oninit(ctx: &Ctx) -> Script {
    nmaker1_tt_main_run(ctx, Nmaker1TtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nmaker3TtMainStep {
    Start,
    OnTouch,
    OnInit,
}

fn nmaker3_tt_main_run(ctx: &Ctx, mut step: Nmaker3TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            Nmaker3TtMainStep::Start => {
                step = Nmaker3TtMainStep::OnTouch;
                continue 'machine;
            }
            Nmaker3TtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        (((ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" is now entering the Single Snail! "))
                            + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                            + Val::from(" is now in the lead!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Notice_Maker3#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
            Nmaker3TtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker3#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nmaker3_tt_main(ctx: &Ctx) -> Script {
    nmaker3_tt_main_run(ctx, Nmaker3TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn nmaker3_tt_main_ontouch(ctx: &Ctx) -> Script {
    nmaker3_tt_main_run(ctx, Nmaker3TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn nmaker3_tt_main_oninit(ctx: &Ctx) -> Script {
    nmaker3_tt_main_run(ctx, Nmaker3TtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Nmaker4TtMainStep {
    Start,
    OnTouch,
    OnInit,
}

fn nmaker4_tt_main_run(ctx: &Ctx, mut step: Nmaker4TtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    'machine: loop {
        match step {
            Nmaker4TtMainStep::Start => {
                step = Nmaker4TtMainStep::OnTouch;
                continue 'machine;
            }
            Nmaker4TtMainStep::OnTouch => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
                        (ctx.call(Function::StrCharInfo, vec![Val::from(0)])? + Val::from(" is currently in First Place!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xFFFF00"),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Notice_Maker4#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
            Nmaker4TtMainStep::OnInit => {
                l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Notice_Maker4#TBT_") + l_w_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn nmaker4_tt_main(ctx: &Ctx) -> Script {
    nmaker4_tt_main_run(ctx, Nmaker4TtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn nmaker4_tt_main_ontouch(ctx: &Ctx) -> Script {
    nmaker4_tt_main_run(ctx, Nmaker4TtMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn nmaker4_tt_main_oninit(ctx: &Ctx) -> Script {
    nmaker4_tt_main_run(ctx, Nmaker4TtMainStep::OnInit, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum WarpTtMainStep {
    Start,
    OnTouch,
}

fn warp_tt_main_run(ctx: &Ctx, mut step: WarpTtMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            WarpTtMainStep::Start => {
                step = WarpTtMainStep::OnTouch;
                continue 'machine;
            }
            WarpTtMainStep::OnTouch => {
                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(20)])?;
                if subject1 == 1 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(216), Val::from(378)],
                    )?;
                } else if subject1 == 2 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(218), Val::from(360)],
                    )?;
                } else if subject1 == 3 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(223), Val::from(361)],
                    )?;
                } else if subject1 == 4 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(243), Val::from(342)],
                    )?;
                } else if subject1 == 5 {
                    ctx.call(
                        Function::Warp,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(247), Val::from(364)],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn warp_tt_main(ctx: &Ctx) -> Script {
    warp_tt_main_run(ctx, WarpTtMainStep::Start, Vec::new()).map(|_| ())
}

pub fn warp_tt_main_ontouch(ctx: &Ctx) -> Script {
    warp_tt_main_run(ctx, WarpTtMainStep::OnTouch, Vec::new()).map(|_| ())
}

fn turbotrap_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn turbotrap_tt_main(ctx: &Ctx) -> Script {
    turbotrap_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn turbotrap_tt_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_hittrap = Val::from(0);
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BLASTMINEBOMB")?])?;
    l_hittrap = Val::from(10);
    if (l_hittrap.clone().number()? > 0 && l_hittrap.clone().number()? < 4) {
        ctx.call(Function::PercentHeal, vec![Val::from(-1), Val::from(0)])?;
    } else if (l_hittrap.clone().number()? > 4 && l_hittrap.clone().number()? < 8) {
        ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
    } else {
        ctx.call(Function::PercentHeal, vec![Val::from(-2), Val::from(0)])?;
    }
    return Err(Stop::End);
}

pub fn turbotrap_tt_main_ontouch(ctx: &Ctx) -> Script {
    turbotrap_tt_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn turbotrap_2_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn turbotrap_2_tt_main(ctx: &Ctx) -> Script {
    turbotrap_2_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn turbotrap_2_tt_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_hittrap = Val::from(0);
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FREEZING")?])?;
    l_hittrap = Val::from(10);
    if (l_hittrap.clone().number()? > 0 && l_hittrap.clone().number()? < 4) {
        ctx.call(Function::PercentHeal, vec![Val::from(-1), Val::from(0)])?;
    } else if (l_hittrap.clone().number()? > 4 && l_hittrap.clone().number()? < 8) {
        ctx.call(Function::PercentHeal, vec![Val::from(-5), Val::from(0)])?;
        ctx.call(
            Function::StartStatus,
            vec![ctx.constant("SC_FREEZE")?, Val::from(3000), Val::from(0)],
        )?;
    } else {
        ctx.call(
            Function::StartStatus,
            vec![ctx.constant("SC_FREEZE")?, Val::from(4000), Val::from(0)],
        )?;
        ctx.call(Function::PercentHeal, vec![Val::from(-2), Val::from(0)])?;
    }
    Ok(Val::from(0))
}

pub fn turbotrap_2_tt_main_ontouch(ctx: &Ctx) -> Script {
    turbotrap_2_tt_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn bing_1_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn bing_1_tt_main(ctx: &Ctx) -> Script {
    bing_1_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn bing_1_tt_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_bing1 = Val::from(0);
    let mut l_w_s = Val::from("");
    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
    l_bing1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
    if (l_bing1.clone().number()? > 0 && l_bing1.clone().number()? < 4) {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(217), Val::from(232)],
        )?;
    } else if l_bing1.clone() == 6 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(233), Val::from(207)],
        )?;
    } else if l_bing1.clone() == 7 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(208), Val::from(219)],
        )?;
    } else if l_bing1.clone() == 8 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(219), Val::from(202)],
        )?;
    } else if l_bing1.clone() == 9 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(218), Val::from(228)],
        )?;
    }
    ctx.call(Function::DisableNpc, vec![(Val::from("bing#") + l_w_s.clone())])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("bing2#") + l_w_s.clone())])?;
    return Err(Stop::End);
}

pub fn bing_1_tt_main_ontouch(ctx: &Ctx) -> Script {
    bing_1_tt_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn bing_2_tt_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn bing_2_tt_main(ctx: &Ctx) -> Script {
    bing_2_tt_main_body(ctx, Vec::new()).map(|_| ())
}

fn bing_2_tt_main_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(10)])?;
    if subject1 == 1 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(217), Val::from(232)],
        )?;
    } else if subject1 == 2 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(233), Val::from(207)],
        )?;
    } else if subject1 == 3 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(208), Val::from(219)],
        )?;
    } else if subject1 == 4 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(219), Val::from(202)],
        )?;
    } else if subject1 == 5 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(218), Val::from(228)],
        )?;
    } else if subject1 == 6 {
        ctx.call(
            Function::Warp,
            vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(220), Val::from(195)],
        )?;
    }
    return Err(Stop::End);
}

pub fn bing_2_tt_main_ontouch(ctx: &Ctx) -> Script {
    bing_2_tt_main_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn bing_2_tt_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_w_s = Val::from("");
    l_w_s = shared::other_turbo_track::f_tt(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("bing2#") + l_w_s.clone())])?;
    return Err(Stop::End);
}

pub fn bing_2_tt_main_oninit(ctx: &Ctx) -> Script {
    bing_2_tt_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum KafraStaffTtStep {
    Start,
    MSave,
}

fn kafra_staff_tt_run(ctx: &Ctx, mut step: KafraStaffTtStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KafraStaffTtStep::Start => {
                ctx.call(Function::Cutin, vec![Val::from("kafra_03"), Val::from(2)])?;
                ctx.lines_as(
                    "Kafra Staff",
                    args![
                        "Welcome~ The Kafra Services",
                        "are always on your side. Let me",
                        "remind you that using the Save",
                        "Service here will change your",
                        "Respawn Point to Al De Baran.",
                        "Now, how may I assist you?"
                    ],
                )?;
                shared::kafras_functions_kafras::f_kafra(ctx, vec![Val::from(5), Val::from(8), Val::from(1), Val::from(40), Val::from(0)])?;
                step = KafraStaffTtStep::MSave;
                continue 'machine;
            }
            KafraStaffTtStep::MSave => {
                ctx.call(
                    Function::SavePoint,
                    vec![Val::from("aldebaran"), Val::from(168), Val::from(112), Val::from(1), Val::from(1)],
                )?;
                shared::kafras_functions_kafras::f_kafend(ctx, vec![Val::from(0), Val::from(1), Val::from("in Al De Baran")])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn kafra_staff_tt(ctx: &Ctx) -> Script {
    kafra_staff_tt_run(ctx, KafraStaffTtStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum TurboTrackGuideStep {
    Start,
    OnInit,
    OnEndN1,
    OnEndN4,
    OnEndN8,
    OnEndN16,
    OnEndE4,
    OnEndE8,
    OnEndE16,
    OnNewTop1,
    OnNewTop2,
    OnNewTop3,
    OnNewTop4,
    OnNewTop5,
    OnWinN4,
    OnWinN8,
    OnWinN16,
    OnWinE4,
    OnWinE8,
    OnWinE16,
}

fn turbo_track_guide_run(ctx: &Ctx, mut step: TurboTrackGuideStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_exitloop = Val::from(0);
    'machine: loop {
        match step {
            TurboTrackGuideStep::Start => {
                ctx.lines_as(
                    "Turbo Track Guide",
                    args![
                        "Good day~",
                        "Is there anything that",
                        "you would like to know",
                        "about the Turbo Track?",
                        "Feel free to ask me",
                        "any questions."
                    ],
                )?;
                ctx.next()?;
                'l1: loop {
                    if !(true) {
                        break 'l1;
                    }
                    'b1: {
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Race Rules:Game Courses:Game Modes:Prohibited Items and Skills:Cancel")],
                            )?);
                            let mut matched2 = false;
                            let no_case2 = !subject2.loosely_equals(&Val::from(1))
                                && !subject2.loosely_equals(&Val::from(2))
                                && !subject2.loosely_equals(&Val::from(3))
                                && !subject2.loosely_equals(&Val::from(4))
                                && !subject2.loosely_equals(&Val::from(5));
                            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "The rules for all the races",
                                        "are pretty simple. Just get",
                                        "past all the obstacles and",
                                        "try to get to the Finish Line",
                                        "as quickly as you can."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Of course, some of the courses",
                                        "in the races may present a few",
                                        "unexpected situations, but I can't really illustrate an example...",
                                        "Just know that there will",
                                        "be traps lying about."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Classes that can mount",
                                        "a Peco Peco, such as Knight,",
                                        "Lord Knight, Crusader and",
                                        "Paladin, must get off their",
                                        "Peco Pecos before entering",
                                        "the Turbo Track Arena."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Well, more accurately,",
                                        "someone at the Turbo Track",
                                        "entrance will ask you to get",
                                        "off and will give you a ticket",
                                        "so that you can rent your",
                                        "Peco again free of charge."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Turbo Track Guide", args!["Now, the winner of the race", "is the person who reaches the", "Finish Line first. Once someone", "crosses the Finish Line, the race ends and everyone is automatically transported to a Waiting Room."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Keep in mind that every",
                                        "race is only 15 minutes long.",
                                        "If no one can reach the Finish",
                                        "Line within that time, the race",
                                        "will end without a winner."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "There are no character",
                                        "class or level requirements",
                                        "to participate in the Turbo",
                                        "Track. However, there is a",
                                        "participation fee of 1,000 zeny."
                                    ],
                                )?;
                                ctx.next()?;
                                break 'b2;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Races in the Turbo Track",
                                        "consist of various courses:",
                                        "^4d4dffLog Bridge, Cube Hills, the",
                                        "Single Snail, Snake Dice, Small",
                                        "Cave and the Invisible Maze."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "The availability of specific",
                                        "courses is determined by the",
                                        "game mode. Would you like to",
                                        "learn more about a certain course?"
                                    ],
                                )?;
                                ctx.next()?;
                                l_exitloop = Val::from(1);
                                'l3: loop {
                                    if !(l_exitloop.clone().is_true()) {
                                        break 'l3;
                                    }
                                    'b3: {
                                        match runtime::select_values(
                                            ctx,
                                            &[Val::from(
                                                "Log Bridge and Cube Hills:Cursed Desert and Flasher Maze:Single Snail and Invisible Maze:Snake Dice and Small Cave:No, thanks.",
                                            )],
                                        )? {
                                            1 => {
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "In the Log Bridge",
                                                        "course, you must cross",
                                                        "over a single log. If you",
                                                        "fall off, you'll be brought",
                                                        "back to the beginning of",
                                                        "the Log Bridge."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "The Cube Hills is a maze",
                                                        "consisting of piles and piles",
                                                        "of boxes. You can climb over",
                                                        "some boxes, but cannot pass",
                                                        ",through others. You'll have to navigate and find the best path."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                            }
                                            2 => {
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "The Cursed Desert is",
                                                        "a short course that is full",
                                                        "of surprises. If you're really",
                                                        "unlucky, you may find yourself",
                                                        "confused or even cursed!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Turbo Track Guide", args!["The Flasher Maze", "is full of traps that will", "blind so you better be extra", "careful in navigating this maze. It might be wiser to try to slow down and avoid the traps..."])?;
                                                ctx.next()?;
                                            }
                                            3 => {
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "The Single Snail is",
                                                        "a narrow, spiral path",
                                                        "shaped like a snail's shell.",
                                                        "There aren't many obstacles,",
                                                        "but it will be tough to race at your top speed in this course."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "The Invisible Maze",
                                                        "looks like a huge, open",
                                                        "room, but it's actually full",
                                                        "of invisible walls. You'll",
                                                        "be given some hints, so",
                                                        "it's not impossible."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                            }
                                            4 => {
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "The Snake Dice course",
                                                        "offers winding paths, much",
                                                        "like the body of a snake. The",
                                                        "factor of luck also plays a role in this course, which you'll",
                                                        "have to see for yourself."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args![
                                                        "The Small Cave is simply",
                                                        "a replica of the Payon Cave.",
                                                        "This part should be a breeze",
                                                        "if you're pretty comfortable with hunting in the Payon Cave."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                            }
                                            5 => {
                                                ctx.lines_as(
                                                    "Turbo Track Guide",
                                                    args!["I see.", "Okay then,", "best of luck to", "you in the races!"],
                                                )?;
                                                ctx.next()?;
                                                l_exitloop = Val::from(0);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args!["Would you like to", "know anything else", "about Turbo Track?"],
                                )?;
                                ctx.next()?;
                                break 'b2;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "There are three different",
                                        "game modes available for",
                                        "races in the Turbo Track:",
                                        "Normal, Expert and Solo."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Unlike the other game",
                                        "modes, ^4d4dffExpert Mode",
                                        "allows players to PvP^000000. This mode",
                                        "is ideal for races between",
                                        "parties or guilds."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Now, the Invisible Maze is",
                                        "a special course that is only",
                                        "available in 16 person races",
                                        "in ^4d4dffNormal^000000 and ^4D4DFFExpert^000000 modes."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "If you play Solo Mode,",
                                        "you can experience every",
                                        "course, and the name of time",
                                        "of the fastest player will be",
                                        "recorded in our Hall of Honor."
                                    ],
                                )?;
                                ctx.next()?;
                                break 'b2;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args![
                                        "Now in Turbo Track, the",
                                        "following items are prohibited",
                                        "from use: Green Potion, Panacea",
                                        "and Fly Wing. If you do try to use them, they won't work until after you leave the race track."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Turbo Track Guide", args!["The following skills cannot", "be used during Turbo Track", "races: Snatch, Teleport, Warp", "Portal, Hiding, Cloaking, Stealth, Cure, Ice Wall, Berserk, Basilica, Sheltering Bless and Spear Dynamo."])?;
                                ctx.next()?;
                            }
                            if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Turbo Track Guide",
                                    args!["Please enjoy your", "time in the Al De Baran", "Turbo Track. Thank you~"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
                step = TurboTrackGuideStep::OnInit;
                continue 'machine;
            }
            TurboTrackGuideStep::OnInit => {
                if ctx.var("$ttnames$").get_at(runtime::index(&Val::from(0))?)? == "" {
                    let base = Val::from(0).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Breezy Havana"))?;
                    let base = Val::from(1).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("RS125"))?;
                    let base = Val::from(2).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Hollgrehenn"))?;
                    let base = Val::from(3).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Antonio"))?;
                    let base = Val::from(4).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Aragham"))?;
                    let base = Val::from(5).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Kafra Jasmine"))?;
                    let base = Val::from(6).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Chris"))?;
                    let base = Val::from(7).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Breezy Havana"))?;
                    let base = Val::from(8).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("RS125"))?;
                    let base = Val::from(9).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Breezy Havana"))?;
                    let base = Val::from(10).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Nari"))?;
                    let base = Val::from(11).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Senorita Sylvia"))?;
                    let base = Val::from(12).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("Joo Jahk"))?;
                    let base = Val::from(13).number()?;
                    ctx.var("$ttnames$")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from("RS125"))?;
                    let base = Val::from(0).number()?;
                    ctx.var("$ttranks")
                        .set_at(runtime::index(&Val::from(base + 0))?, Val::from(999999))?;
                    ctx.var("$ttranks").set_at(runtime::index(&Val::from(base + 1))?, Val::from(0))?;
                    ctx.var("$ttranks").set_at(runtime::index(&Val::from(base + 2))?, Val::from(0))?;
                    ctx.var("$ttranks").set_at(runtime::index(&Val::from(base + 3))?, Val::from(0))?;
                    ctx.var("$ttranks").set_at(runtime::index(&Val::from(base + 4))?, Val::from(0))?;
                    ctx.var("$ttranks").set_at(runtime::index(&Val::from(base + 5))?, Val::from(0))?;
                }
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndN1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Normal Mode - Solo] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndN4 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Normal Mode - 4 Person] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndN8 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Normal Mode - 8 Person] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndN16 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Normal Mode - 16 Person] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndE4 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Expert Mode - 4 Person] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndE8 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Expert Mode - 8 Person] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnEndE16 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        Val::from("A [Expert Mode - 16 Person] game will end shortly."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xadff2f"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnNewTop1 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(9))?)?)
                            + Val::from(" has ranked Number One in the Turbo Track Hall of Honor!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnNewTop2 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(10))?)?)
                            + Val::from(" has ranked Second in the Turbo Track Hall of Honor!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnNewTop3 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(11))?)?)
                            + Val::from(" has ranked Third in the Turbo Track Hall of Honor!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnNewTop4 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(12))?)?)
                            + Val::from(" has ranked Fourth in the Turbo Track Hall of Honor!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnNewTop5 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(13))?)?)
                            + Val::from(" has ranked Fifth in the Turbo Track Hall of Honor!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnWinN4 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(1))?)?)
                            + Val::from(" just won a [Normal Mode - 4 Person] game!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnWinN8 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(2))?)?)
                            + Val::from(" just won a [Normal Mode - 8 Person] game!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnWinN16 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(3))?)?)
                            + Val::from(" just won a [Normal Mode - 16 Person] game!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnWinE4 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(4))?)?)
                            + Val::from(" just won an [Expert Mode - 4 Person] game!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnWinE8 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(5))?)?)
                            + Val::from(" just won an [Expert Mode - 8 Person] game!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
            TurboTrackGuideStep::OnWinE16 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("turbo_room"),
                        ((Val::from("Congratulations! ") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(6))?)?)
                            + Val::from(" just won an [Expert Mode - 16 Person] game!")),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x70DBDB"),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn turbo_track_guide(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::Start, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_oninit(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnInit, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_n1(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndN1, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_n4(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndN4, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_n8(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndN8, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_n16(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndN16, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_e4(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndE4, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_e8(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndE8, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onend_e16(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnEndE16, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onnew_top1(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnNewTop1, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onnew_top2(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnNewTop2, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onnew_top3(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnNewTop3, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onnew_top4(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnNewTop4, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onnew_top5(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnNewTop5, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onwin_n4(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnWinN4, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onwin_n8(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnWinN8, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onwin_n16(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnWinN16, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onwin_e4(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnWinE4, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onwin_e8(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnWinE8, Vec::new()).map(|_| ())
}

pub fn turbo_track_guide_onwin_e16(ctx: &Ctx) -> Script {
    turbo_track_guide_run(ctx, TurboTrackGuideStep::OnWinE16, Vec::new()).map(|_| ())
}

fn hall_of_honor_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^2F4F4FTurbo Track Hall of Honor^000000",
        " ",
        "The First:",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(7))?)?) + Val::from("^000000")),
        " ",
        "The Last:",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(8))?)?) + Val::from("^000000"))
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hall_of_honor_tt(ctx: &Ctx) -> Script {
    hall_of_honor_tt_body(ctx, Vec::new()).map(|_| ())
}

fn solo_mode_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^2F4F4FSolo Mode Record^000000",
        " ",
        "The best player",
        "in Solo Mode...",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(0))?)?) + Val::from("^000000 !"))
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn solo_mode_tt(ctx: &Ctx) -> Script {
    solo_mode_tt_body(ctx, Vec::new()).map(|_| ())
}

fn normal_mode_record_tt_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "<<Recent Top Players>>",
        "Winners of Normal Mode - 4 Person",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(1))?)?) + Val::from("^000000")),
        "Winners of Normal Mode - 8 Person",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(2))?)?) + Val::from("^000000")),
        "Winners of Normal Mode - 16 Person",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(3))?)?) + Val::from("^000000"))
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn normal_mode_record_tt(ctx: &Ctx) -> Script {
    normal_mode_record_tt_body(ctx, Vec::new()).map(|_| ())
}

fn expert_mode_record_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "<<Recent Top Players>>",
        "Winners of Expert Mode - 4 Person",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(4))?)?) + Val::from("^000000")),
        "Winners of Expert Mode - 8 Person",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(5))?)?) + Val::from("^000000")),
        "Winners of Expert Mode - 16 Person",
        ((Val::from("^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(6))?)?) + Val::from("^000000"))
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn expert_mode_record(ctx: &Ctx) -> Script {
    expert_mode_record_body(ctx, Vec::new()).map(|_| ())
}

fn hall_of_honor_tt2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^2F4F4FTurbo Track Hall of Honor^000000",
        " ",
        ((Val::from("1st: ^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(9))?)?) + Val::from("^000000")),
        ((Val::from("2nd: ^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(10))?)?) + Val::from("^000000")),
        ((Val::from("3rd: ^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(11))?)?) + Val::from("^000000")),
        ((Val::from("4th: ^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(12))?)?) + Val::from("^000000")),
        ((Val::from("5th: ^4d4dff") + ctx.var("$ttnames$").get_at(runtime::index(&Val::from(13))?)?) + Val::from("^000000"))
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn hall_of_honor_tt2(ctx: &Ctx) -> Script {
    hall_of_honor_tt2_body(ctx, Vec::new()).map(|_| ())
}

fn point_exchange_helper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    let mut l_name_s = Val::from("");
    let mut l_scroll = Val::from(0);
    let mut l_total_point = Val::from(0);
    if ((ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000
        || ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0)
    {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many items with you.",
            "Please come back after",
            "putting storing some of your",
            "things using the Kafra Service.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("tt_point").get()?.number()? < 10 {
        ctx.lines_as(
            "Item Exchange Helper",
            args![
                "Good day,",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "You've got a total of",
                ((Val::from("") + ctx.var("tt_point").get()?) + Val::from(" Turbo Track Points."))
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Item Exchange Helper",
            args![
                "If you have at least",
                "10 Turbo Track Points,",
                "you can exchange these",
                "points for items. Would",
                "you like to see the Turbo",
                "Track Point exchange list?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:Cancel.")])?) == 1 {
            ctx.lines_as(
                "Item Exchange Helper",
                args![
                    "^3355FF10 TTP^000000: 1 Free Ticket",
                    "for Kafra Transportation",
                    "^3355FF12 TTP^000000: 1 Level 5 Magic Scroll",
                    "^3355FF40 TTP^000000: Experience Points"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Item Exchange Helper",
                args![
                    "^3355FF150 TTP^000000: 1 3 Carat Diamond",
                    "^3355FF300 TTP^000000: 1 Gift Box",
                    "^3355FF400 TTP^000000: 1 Speed Potion"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Item Exchange Helper", args!["You can also convert Turbo Track Points into Arena Points. For more information, please speak to the Arena Point Manager. Thank you."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Item Exchange Helper",
            args![
                "We hope that you enjoy",
                "Al De Baran's Turbo Track.",
                "Try to earn as many points",
                "as you can so that you can",
                "exchange them for useful",
                "stuff later. Happy racing~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Item Exchange Helper",
            args![
                "Good day,",
                ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from(".")),
                "You've got a total of",
                ((Val::from("") + ctx.var("tt_point").get()?) + Val::from(" Turbo Track Points.")),
                "Would you like to exchange",
                "these points for items?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Exchange.:Cancel.")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Item Exchange Helper",
                    args![
                        "Please choose the item for",
                        "which you'd like to exchange",
                        "your Turbo Track Points."
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Free Ticket for Kafra Transportation:Level 5 Magic Scroll:Experience Points:3 Carat Diamond:Gift Box:Speed Potion:Cancel.",
                    )],
                )? {
                    1 => {
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "Each Free Ticket for",
                                "Kafra Transportation",
                                "costs 10 Turbo Track Points.",
                                "Please enter the amount that",
                                "you wish to have. To cancel,",
                                "please enter ''^3355FF0^000000.''"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 0 {
                            ctx.lines_as("Item Exchange Helper", args!["You have", "canceled", "your request."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (l_input.clone().number()? < 0 || l_input.clone().number()? > 50) {
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Your request exceeds",
                                    "the maximum limit. You",
                                    "can only receive a maximum",
                                    "of 50 tickets at once."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            l_total_point = (Val::from(10).try_mul(l_input.clone())?);
                            if runtime::op(&l_total_point.clone(), ">", &ctx.var("tt_point").get()?)?.is_true() {
                                ctx.lines_as(
                                    "Item Exchange Helper",
                                    args![
                                        "I'm sorry, but you do",
                                        "not have enough Turbo",
                                        "Track Points. Please check",
                                        "the amount of Turbo Track",
                                        "Points you have earned before",
                                        "redeeming your points again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("tt_point")
                                .set((ctx.var("tt_point").get()?.try_sub(l_total_point.clone())?))?;
                            ctx.call(Function::GetItem, vec![Val::from(7060), l_input.clone()])?;
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Thank you for",
                                    "your patronage.",
                                    "We hope you enjoy",
                                    "your time here in",
                                    "the Turbo Track~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        ctx.lines_as("Item Exchange Helper", args!["Each Magic Scroll costs", "12 Turbo Track Points. Please", "select the Magic Scroll you would like to receive. Each scroll enables a one time use of a Level 5 spell or skill."])?;
                        ctx.next()?;
                        match runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Earth Spike:Cold Bolt:Fire Bolt:Lightning Bolt:Soul Strike:Fire Ball:Fire Wall:Frost Diver:Heal",
                            )],
                        )? {
                            1 => {
                                l_scroll = Val::from(687);
                                l_name_s = Val::from("Earth Spike");
                            }
                            2 => {
                                l_scroll = Val::from(689);
                                l_name_s = Val::from("Cold Boltl");
                            }
                            3 => {
                                l_scroll = Val::from(691);
                                l_name_s = Val::from("Fire Bolt");
                            }
                            4 => {
                                l_scroll = Val::from(693);
                                l_name_s = Val::from("Lightning Bolt");
                            }
                            5 => {
                                l_scroll = Val::from(695);
                                l_name_s = Val::from("Soul Strike");
                            }
                            6 => {
                                l_scroll = Val::from(697);
                                l_name_s = Val::from("Fire Ball");
                            }
                            7 => {
                                l_scroll = Val::from(699);
                                l_name_s = Val::from("Fire Wall");
                            }
                            8 => {
                                l_scroll = Val::from(12000);
                                l_name_s = Val::from("Frost Diver");
                            }
                            9 => {
                                l_scroll = Val::from(12002);
                                l_name_s = Val::from("Heal");
                            }
                            _ => {}
                        }
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "Please enter the number",
                                ((Val::from("of ^4d4dff") + l_name_s.clone()) + Val::from("^000000 Magic Scrolls that")),
                                "you would like to receive.",
                                "To cancel, enter ''^3355FF0^000000.''"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 0 {
                            ctx.lines_as("Item Exchange Helper", args!["You have", "canceled", "your request."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (l_input.clone().number()? < 0 || l_input.clone().number()? > 50) {
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "I'm sorry, but your",
                                    "request has exceeded the",
                                    "maximum limit. You can only",
                                    "request up to 50 scrolls at once."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            l_total_point = (Val::from(12).try_mul(l_input.clone())?);
                            if runtime::op(&l_total_point.clone(), ">", &ctx.var("tt_point").get()?)?.is_true() {
                                ctx.lines_as(
                                    "Item Exchange Helper",
                                    args![
                                        "I'm sorry, but you do",
                                        "not have enough Turbo",
                                        "Track Points. Please check",
                                        "the amount of Turbo Track",
                                        "Points you have earned before",
                                        "redeeming your points again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("tt_point")
                                .set((ctx.var("tt_point").get()?.try_sub(l_total_point.clone())?))?;
                            ctx.call(Function::GetItem, vec![l_scroll.clone(), l_input.clone()])?;
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Thank you for",
                                    "your patronage.",
                                    "We hope you enjoy",
                                    "your time here in",
                                    "the Turbo Track~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    3 => {
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "You can exchange",
                                "40 Turbo Track Points",
                                "to receive Base Level",
                                "Experience. Would you",
                                "like to exchange your Turbo",
                                "Track Points for Experience?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("No:Yes")])?) == 2 {
                            if ctx.var("tt_point").get()?.number()? < 40 {
                                ctx.lines_as(
                                    "Item Exchange Helper",
                                    args![
                                        "I'm sorry, but you do",
                                        "not have enough Turbo",
                                        "Track Points. Please check",
                                        "the amount of Turbo Track",
                                        "Points you have earned before",
                                        "redeeming your points again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("tt_point").set((ctx.var("tt_point").get()?.try_sub(Val::from(40))?))?;
                            if ctx.var("BaseLevel").get()?.number()? < 70 {
                                ctx.call(Function::GetExperience, vec![Val::from(3000), Val::from(0)])?;
                            } else if ctx.var("BaseLevel").get()?.number()? < 80 {
                                ctx.call(Function::GetExperience, vec![Val::from(9000), Val::from(0)])?;
                            } else if ctx.var("BaseLevel").get()?.number()? < 90 {
                                ctx.call(Function::GetExperience, vec![Val::from(10000), Val::from(0)])?;
                            } else if runtime::op(&ctx.var("BaseLevel").get()?, "<", &ctx.constant("MAX_LEVEL")?)?.is_true() {
                                ctx.call(Function::GetExperience, vec![Val::from(30000), Val::from(0)])?;
                            }
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Thank you, your",
                                    "Turbo Track Points",
                                    "have been converted into",
                                    "Base Level Experience."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Item Exchange Helper", args!["You have", "canceled", "your request."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    4 => {
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "You can exchange",
                                "150 Turbo Track Points",
                                "for ^3131FF1 3 Carat Diamond^000000.",
                                "Please enter the number of",
                                "diamonds you would like to receive.",
                                "To cancel, enter ''^3355FF0^000000.''"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 0 {
                            ctx.lines_as("Item Exchange Helper", args!["You have", "canceled", "your request."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (l_input.clone().number()? < 0 || l_input.clone().number()? > 10) {
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Your request exceeds",
                                    "the maximum limit. You",
                                    "can only receive a maximum",
                                    "of 10 diamonds at once."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            l_total_point = (Val::from(150).try_mul(l_input.clone())?);
                            if runtime::op(&l_total_point.clone(), ">", &ctx.var("tt_point").get()?)?.is_true() {
                                ctx.lines_as(
                                    "Item Exchange Helper",
                                    args![
                                        "I'm sorry, but you do",
                                        "not have enough Turbo",
                                        "Track Points. Please check",
                                        "the amount of Turbo Track",
                                        "Points you have earned before",
                                        "redeeming your points again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("tt_point")
                                .set((ctx.var("tt_point").get()?.try_sub(l_total_point.clone())?))?;
                            ctx.call(Function::GetItem, vec![Val::from(732), l_input.clone()])?;
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Thank you for",
                                    "your patronage.",
                                    "We hope you enjoy",
                                    "your time here in",
                                    "the Turbo Track~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    5 => {
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "You can exchange",
                                "300 Turbo Track Points",
                                "for ^3131FF1 Gift Box^000000. Please enter",
                                "the number of Gift Boxes",
                                "that you'd like to receive.",
                                "To cancel, enter ''^3355FF0^000000.''"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 0 {
                            ctx.lines_as("Item Exchange Helper", args!["You have", "canceled", "your request."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (l_input.clone().number()? < 0 || l_input.clone().number()? > 10) {
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Your request exceeds",
                                    "the maximum limit. You",
                                    "can only receive a maximum",
                                    "of 10 Gift Boxes at once."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            l_total_point = (Val::from(300).try_mul(l_input.clone())?);
                            if runtime::op(&l_total_point.clone(), ">", &ctx.var("tt_point").get()?)?.is_true() {
                                ctx.lines_as(
                                    "Item Exchange Helper",
                                    args![
                                        "I'm sorry, but you do",
                                        "not have enough Turbo",
                                        "Track Points. Please check",
                                        "the amount of Turbo Track",
                                        "Points you have earned before",
                                        "redeeming your points again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("tt_point")
                                .set((ctx.var("tt_point").get()?.try_sub(l_total_point.clone())?))?;
                            ctx.call(Function::GetItem, vec![Val::from(644), l_input.clone()])?;
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Thank you for",
                                    "your patronage.",
                                    "We hope you enjoy",
                                    "your time here in",
                                    "the Turbo Track~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    6 => {
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "You can exchange",
                                "400 Turbo Track Points",
                                "for ^3131FF1 Speed Potion^000000. Please",
                                "enter the number of potions",
                                "that you'd like to receive.",
                                "To cancel, enter ''^3355FF0^000000.''"
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_input = input;
                        if l_input.clone() == 0 {
                            ctx.lines_as("Item Exchange Helper", args!["You have", "canceled", "your request."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (l_input.clone().number()? < 0 || l_input.clone().number()? > 10) {
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Your request exceeds",
                                    "the maximum limit. You",
                                    "can only receive a maximum",
                                    "of 10 potions at once."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            l_total_point = (Val::from(400).try_mul(l_input.clone())?);
                            if runtime::op(&l_total_point.clone(), ">", &ctx.var("tt_point").get()?)?.is_true() {
                                ctx.lines_as(
                                    "Item Exchange Helper",
                                    args![
                                        "I'm sorry, but you do",
                                        "not have enough Turbo",
                                        "Track Points. Please check",
                                        "the amount of Turbo Track",
                                        "Points you have earned before",
                                        "redeeming your points again."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("tt_point")
                                .set((ctx.var("tt_point").get()?.try_sub(l_total_point.clone())?))?;
                            ctx.call(Function::GetItem, vec![Val::from(12016), l_input.clone()])?;
                            ctx.lines_as(
                                "Item Exchange Helper",
                                args![
                                    "Thank you for",
                                    "your patronage.",
                                    "We hope you enjoy",
                                    "your time here in",
                                    "the Turbo Track~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    7 => {
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args!["This Item Exchange", "Service is brought", "to you by..."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Item Exchange Helper",
                            args![
                                "This Item Exchange Service",
                                "is brought to you by the Kafra",
                                "Corporation, Blacksmith Guild",
                                "and the Comodo Casino and",
                                "the Al De Baran Guild Castle",
                                "Management Luina."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Item Exchange Helper",
                    args![
                        "Turbo Track Points can be",
                        "converted into Arena Points.",
                        "You can save a maximum of",
                        "^4D4DFF29,000 Turbo Track Points."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Item Exchange Helper",
                    args![
                        "Before reaching the maximum",
                        "amount of Turbo Track Points,",
                        "you might want to spend some",
                        "of them so that you can keep",
                        "getting your point rewards after winning Turbo Track races."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn point_exchange_helper(ctx: &Ctx) -> Script {
    point_exchange_helper_body(ctx, Vec::new()).map(|_| ())
}
