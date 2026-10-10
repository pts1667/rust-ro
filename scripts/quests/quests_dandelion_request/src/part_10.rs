use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate23Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
}

fn dandelion_maogate2_3_run(ctx: &Ctx, mut step: DandelionMaogate23Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate23Step::Start => {
                step = DandelionMaogate23Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate23Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate2_3")])?;
                return Err(Stop::End);
            }
            DandelionMaogate23Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate2_3")])?;
                return Err(Stop::End);
            }
            DandelionMaogate23Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate2_3(ctx: &Ctx) -> Script {
    dandelion_maogate2_3_run(ctx, DandelionMaogate23Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_3_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate2_3_run(ctx, DandelionMaogate23Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_3_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate2_3_run(ctx, DandelionMaogate23Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_3_onspell(ctx: &Ctx) -> Script {
    dandelion_maogate2_3_run(ctx, DandelionMaogate23Step::OnSpell, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum DandelionMaogate24Step {
    Start,
    OnInit,
    OnEnter,
    OnSpell,
}

fn dandelion_maogate2_4_run(ctx: &Ctx, mut step: DandelionMaogate24Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            DandelionMaogate24Step::Start => {
                step = DandelionMaogate24Step::OnInit;
                continue 'machine;
            }
            DandelionMaogate24Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Dandelion#maogate2_4")])?;
                return Err(Stop::End);
            }
            DandelionMaogate24Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("Dandelion#maogate2_4")])?;
                return Err(Stop::End);
            }
            DandelionMaogate24Step::OnSpell => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_LORD")?])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn dandelion_maogate2_4(ctx: &Ctx) -> Script {
    dandelion_maogate2_4_run(ctx, DandelionMaogate24Step::Start, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_4_oninit(ctx: &Ctx) -> Script {
    dandelion_maogate2_4_run(ctx, DandelionMaogate24Step::OnInit, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_4_onenter(ctx: &Ctx) -> Script {
    dandelion_maogate2_4_run(ctx, DandelionMaogate24Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn dandelion_maogate2_4_onspell(ctx: &Ctx) -> Script {
    dandelion_maogate2_4_run(ctx, DandelionMaogate24Step::OnSpell, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Maogate2End2Step {
    Start,
    OnInit,
    OnEnter,
    OnTouch,
}

fn maogate2_end2_run(ctx: &Ctx, mut step: Maogate2End2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Maogate2End2Step::Start => {
                step = Maogate2End2Step::OnInit;
                continue 'machine;
            }
            Maogate2End2Step::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#maogate2_end2")])?;
                return Err(Stop::End);
            }
            Maogate2End2Step::OnEnter => {
                ctx.call(Function::EnableNpc, vec![Val::from("#maogate2_end2")])?;
                return Err(Stop::End);
            }
            Maogate2End2Step::OnTouch => {
                if ctx.var("mao_request").get()? == 126 {
                    ctx.call(Function::Cutin, vec![Val::from("mocseal_kid01.bmp"), Val::from(2)])?;
                    ctx.lines_as(
                        "Kidd",
                        args![
                            ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("...!")),
                            "Can you hear me? Oh man...",
                            "You don't look so hot. Come",
                            "on, we gotta get you back",
                            "to the guild for now..."
                        ],
                    )?;
                    ctx.var("mao_request").set(Val::from(127))?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::MapWarp,
                        vec![Val::from("que_job03"), Val::from("que_job01"), Val::from(59), Val::from(49)],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk2::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk3::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk4::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk5::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk6::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_talk7::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Kidd#maogate2_2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Lin#maogate2_2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("R#maogate2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end::OnStop")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_end2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_1::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_2::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_3::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion#maogate2_4::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_setting::OnEnter")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Dandelion Member#2_bt::OnInit")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_battle::OnStop")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#maogate2_battle::OnEnter")])?;
                    ctx.call(Function::EnableNpc, vec![Val::from("#maogate2")])?;
                    ctx.var("$mao_gate2").set(Val::from(0))?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn maogate2_end2(ctx: &Ctx) -> Script {
    maogate2_end2_run(ctx, Maogate2End2Step::Start, Vec::new()).map(|_| ())
}

pub fn maogate2_end2_oninit(ctx: &Ctx) -> Script {
    maogate2_end2_run(ctx, Maogate2End2Step::OnInit, Vec::new()).map(|_| ())
}

pub fn maogate2_end2_onenter(ctx: &Ctx) -> Script {
    maogate2_end2_run(ctx, Maogate2End2Step::OnEnter, Vec::new()).map(|_| ())
}

pub fn maogate2_end2_ontouch(ctx: &Ctx) -> Script {
    maogate2_end2_run(ctx, Maogate2End2Step::OnTouch, Vec::new()).map(|_| ())
}

fn member_mao1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Chairman",
        args![
            "So it's agreed.",
            "Tomorrow we'll clean",
            "the pond near the south",
            "gate tomorrow. I hope",
            "that everyone can make it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Members",
        args![
            "Sure, no problem.",
            "When are we planning",
            "on reconstructing the",
            "daycare facilities?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chairman",
        args![
            "Hmmm...",
            "I guess that's an",
            "issue that we can",
            "save for tomorrow's",
            "meeting. I'll see you then."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Is this some", "sort of volunteer", "service group?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_mao1(ctx: &Ctx) -> Script {
    member_mao1_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao1_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Member#mao1")])?;
    return Err(Stop::End);
}

pub fn member_mao1_onstop(ctx: &Ctx) -> Script {
    member_mao1_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao1_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Member#mao1")])?;
    return Err(Stop::End);
}

pub fn member_mao1_onenter(ctx: &Ctx) -> Script {
    member_mao1_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao1_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if (ctx.var("mao_request").get()? == 30 || ctx.var("mao_request").get()? == 128) {
        ctx.lines_as("?????", args!["...That's what", "happened... Worked", "hard... This time..."])?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args!["There is just one", "regret... Was that...", "...tan... was... better..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "????????",
            args!["I know, I know that!", "But we still accomplished", "what we set out to do, right?"],
        )?;
        ctx.next()?;
        ctx.lines_as("??????", args!["Quiet!", "You're too loud."])?;
        ctx.next()?;
        ctx.lines_as(
            "?????",
            args![
                "I'm pretty sure...",
                "...is... not going to",
                "be quiet about this.",
                "Perhaps... it'll happen."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "???",
            args!["Shh. Act naturally!", "Someone's listening", "to us! Get out of here!"],
        )?;
        ctx.close_window()?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao1::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao2::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao3::OnStop")])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao4::OnStop")])?;
        if ctx.var("$maoattack").get()?.number()? > 49 {
            ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_manager::OnStart")])?;
            ctx.var("$maoattack").set(Val::from(0))?;
        } else {
            ctx.call(Function::DoNpcEvent, vec![Val::from("#mao_timer::OnStart")])?;
        }
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn member_mao1_ontouch(ctx: &Ctx) -> Script {
    member_mao1_ontouch_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Chairman",
        args![
            "So it's agreed.",
            "Tomorrow we'll clean",
            "the pond near the south",
            "gate tomorrow. I hope",
            "that everyone can make it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Members",
        args![
            "Sure, no problem.",
            "When are we planning",
            "on reconstructing the",
            "daycare facilities?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chairman",
        args![
            "Hmmm...",
            "I guess that's an",
            "issue that we can",
            "save for tomorrow's",
            "meeting. I'll see you then."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Is this some", "sort of volunteer", "service group?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_mao2(ctx: &Ctx) -> Script {
    member_mao2_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao2_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Member#mao2")])?;
    return Err(Stop::End);
}

pub fn member_mao2_onstop(ctx: &Ctx) -> Script {
    member_mao2_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao2_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Member#mao2")])?;
    return Err(Stop::End);
}

pub fn member_mao2_onenter(ctx: &Ctx) -> Script {
    member_mao2_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Chairman",
        args![
            "So it's agreed.",
            "Tomorrow we'll clean",
            "the pond near the south",
            "gate tomorrow. I hope",
            "that everyone can make it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Members",
        args![
            "Sure, no problem.",
            "When are we planning",
            "on reconstructing the",
            "daycare facilities?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chairman",
        args![
            "Hmmm...",
            "I guess that's an",
            "issue that we can",
            "save for tomorrow's",
            "meeting. I'll see you then."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Is this some", "sort of volunteer", "service group?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_mao3(ctx: &Ctx) -> Script {
    member_mao3_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao3_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Member#mao3")])?;
    return Err(Stop::End);
}

pub fn member_mao3_onstop(ctx: &Ctx) -> Script {
    member_mao3_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao3_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Member#mao3")])?;
    return Err(Stop::End);
}

pub fn member_mao3_onenter(ctx: &Ctx) -> Script {
    member_mao3_onenter_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Chairman",
        args![
            "So it's agreed.",
            "Tomorrow we'll clean",
            "the pond near the south",
            "gate tomorrow. I hope",
            "that everyone can make it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Members",
        args![
            "Sure, no problem.",
            "When are we planning",
            "on reconstructing the",
            "daycare facilities?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Chairman",
        args![
            "Hmmm...",
            "I guess that's an",
            "issue that we can",
            "save for tomorrow's",
            "meeting. I'll see you then."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Is this some", "sort of volunteer", "service group?"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn member_mao4(ctx: &Ctx) -> Script {
    member_mao4_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao4_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Member#mao4")])?;
    return Err(Stop::End);
}

pub fn member_mao4_onstop(ctx: &Ctx) -> Script {
    member_mao4_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn member_mao4_onenter_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Member#mao4")])?;
    return Err(Stop::End);
}

pub fn member_mao4_onenter(ctx: &Ctx) -> Script {
    member_mao4_onenter_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MaoTimerStep {
    Start,
    OnStart,
    OnTimer300000,
    OnStop,
}

fn mao_timer_run(ctx: &Ctx, mut step: MaoTimerStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MaoTimerStep::Start => {
                step = MaoTimerStep::OnStart;
                continue 'machine;
            }
            MaoTimerStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            MaoTimerStep::OnTimer300000 => {
                step = MaoTimerStep::OnStop;
                continue 'machine;
            }
            MaoTimerStep::OnStop => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao1::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao2::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao3::OnEnter")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Member#mao4::OnEnter")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mao_timer(ctx: &Ctx) -> Script {
    mao_timer_run(ctx, MaoTimerStep::Start, Vec::new()).map(|_| ())
}

pub fn mao_timer_onstart(ctx: &Ctx) -> Script {
    mao_timer_run(ctx, MaoTimerStep::OnStart, Vec::new()).map(|_| ())
}

pub fn mao_timer_ontimer300000(ctx: &Ctx) -> Script {
    mao_timer_run(ctx, MaoTimerStep::OnTimer300000, Vec::new()).map(|_| ())
}

pub fn mao_timer_onstop(ctx: &Ctx) -> Script {
    mao_timer_run(ctx, MaoTimerStep::OnStop, Vec::new()).map(|_| ())
}
