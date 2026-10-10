use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn getitem2_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 8 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(9))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem2_1(ctx: &Ctx) -> Script {
    getitem2_1_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-1")])?;
    return Err(Stop::End);
}

pub fn getitem2_1_oninit(ctx: &Ctx) -> Script {
    getitem2_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_1_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem2-1")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-1::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem2_1_oncommandon(ctx: &Ctx) -> Script {
    getitem2_1_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_1_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-1")])?;
    return Err(Stop::End);
}

pub fn getitem2_1_oncommandoff(ctx: &Ctx) -> Script {
    getitem2_1_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_2_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 8 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-2::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace2_2_gnbs(ctx: &Ctx) -> Script {
    trace2_2_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_2_gnbs_ontimer130000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-2#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace2_2_gnbs_ontimer130000(ctx: &Ctx) -> Script {
    trace2_2_gnbs_ontimer130000_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_2_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-2#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace2_2_gnbs_oninit(ctx: &Ctx) -> Script {
    trace2_2_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_2_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace2-2#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace2_2_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace2_2_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer2_2(ctx: &Ctx) -> Script {
    timer2_2_run(ctx, Timer22Step::Start, Vec::new()).map(|_| ())
}

pub fn timer2_2_oninit(ctx: &Ctx) -> Script {
    timer2_2_run(ctx, Timer22Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer2_2_oncommandon(ctx: &Ctx) -> Script {
    timer2_2_run(ctx, Timer22Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer2_2_oncommandoff(ctx: &Ctx) -> Script {
    timer2_2_run(ctx, Timer22Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer2_2_ontimer3000(ctx: &Ctx) -> Script {
    timer2_2_run(ctx, Timer22Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem2_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 8 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(9))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem2_2(ctx: &Ctx) -> Script {
    getitem2_2_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-2")])?;
    return Err(Stop::End);
}

pub fn getitem2_2_oninit(ctx: &Ctx) -> Script {
    getitem2_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_2_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem2-2")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-2::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem2_2_oncommandon(ctx: &Ctx) -> Script {
    getitem2_2_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_2_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-2")])?;
    return Err(Stop::End);
}

pub fn getitem2_2_oncommandoff(ctx: &Ctx) -> Script {
    getitem2_2_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_3_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 8 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-3::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace2_3_gnbs(ctx: &Ctx) -> Script {
    trace2_3_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_3_gnbs_ontimer110000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-3#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace2_3_gnbs_ontimer110000(ctx: &Ctx) -> Script {
    trace2_3_gnbs_ontimer110000_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_3_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-3#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace2_3_gnbs_oninit(ctx: &Ctx) -> Script {
    trace2_3_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_3_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace2-3#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace2_3_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace2_3_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer2_3(ctx: &Ctx) -> Script {
    timer2_3_run(ctx, Timer23Step::Start, Vec::new()).map(|_| ())
}

pub fn timer2_3_oninit(ctx: &Ctx) -> Script {
    timer2_3_run(ctx, Timer23Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer2_3_oncommandon(ctx: &Ctx) -> Script {
    timer2_3_run(ctx, Timer23Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer2_3_oncommandoff(ctx: &Ctx) -> Script {
    timer2_3_run(ctx, Timer23Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer2_3_ontimer3000(ctx: &Ctx) -> Script {
    timer2_3_run(ctx, Timer23Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem2_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 8 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(9))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem2_3(ctx: &Ctx) -> Script {
    getitem2_3_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-3")])?;
    return Err(Stop::End);
}

pub fn getitem2_3_oninit(ctx: &Ctx) -> Script {
    getitem2_3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_3_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem2-3")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-3::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem2_3_oncommandon(ctx: &Ctx) -> Script {
    getitem2_3_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_3_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-3")])?;
    return Err(Stop::End);
}

pub fn getitem2_3_oncommandoff(ctx: &Ctx) -> Script {
    getitem2_3_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_4_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 8 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-4::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace2_4_gnbs(ctx: &Ctx) -> Script {
    trace2_4_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_4_gnbs_ontimer230000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-4#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace2_4_gnbs_ontimer230000(ctx: &Ctx) -> Script {
    trace2_4_gnbs_ontimer230000_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_4_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-4#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace2_4_gnbs_oninit(ctx: &Ctx) -> Script {
    trace2_4_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_4_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace2-4#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace2_4_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace2_4_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer2_4(ctx: &Ctx) -> Script {
    timer2_4_run(ctx, Timer24Step::Start, Vec::new()).map(|_| ())
}

pub fn timer2_4_oninit(ctx: &Ctx) -> Script {
    timer2_4_run(ctx, Timer24Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer2_4_oncommandon(ctx: &Ctx) -> Script {
    timer2_4_run(ctx, Timer24Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer2_4_oncommandoff(ctx: &Ctx) -> Script {
    timer2_4_run(ctx, Timer24Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer2_4_ontimer3000(ctx: &Ctx) -> Script {
    timer2_4_run(ctx, Timer24Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem2_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 8 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(9))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem2_4(ctx: &Ctx) -> Script {
    getitem2_4_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_4_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-4")])?;
    return Err(Stop::End);
}

pub fn getitem2_4_oninit(ctx: &Ctx) -> Script {
    getitem2_4_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_4_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem2-4")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-4::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem2_4_oncommandon(ctx: &Ctx) -> Script {
    getitem2_4_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_4_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-4")])?;
    return Err(Stop::End);
}

pub fn getitem2_4_oncommandoff(ctx: &Ctx) -> Script {
    getitem2_4_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_5_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 8 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-5::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace2_5_gnbs(ctx: &Ctx) -> Script {
    trace2_5_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_5_gnbs_ontimer190000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-5#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace2_5_gnbs_ontimer190000(ctx: &Ctx) -> Script {
    trace2_5_gnbs_ontimer190000_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_5_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-5#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace2_5_gnbs_oninit(ctx: &Ctx) -> Script {
    trace2_5_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_5_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace2-5#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace2_5_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace2_5_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer2_5(ctx: &Ctx) -> Script {
    timer2_5_run(ctx, Timer25Step::Start, Vec::new()).map(|_| ())
}

pub fn timer2_5_oninit(ctx: &Ctx) -> Script {
    timer2_5_run(ctx, Timer25Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer2_5_oncommandon(ctx: &Ctx) -> Script {
    timer2_5_run(ctx, Timer25Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer2_5_oncommandoff(ctx: &Ctx) -> Script {
    timer2_5_run(ctx, Timer25Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer2_5_ontimer3000(ctx: &Ctx) -> Script {
    timer2_5_run(ctx, Timer25Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem2_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 8 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(9))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem2_5(ctx: &Ctx) -> Script {
    getitem2_5_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_5_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-5")])?;
    return Err(Stop::End);
}

pub fn getitem2_5_oninit(ctx: &Ctx) -> Script {
    getitem2_5_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_5_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem2-5")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-5::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem2_5_oncommandon(ctx: &Ctx) -> Script {
    getitem2_5_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_5_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-5")])?;
    return Err(Stop::End);
}

pub fn getitem2_5_oncommandoff(ctx: &Ctx) -> Script {
    getitem2_5_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_6_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 8 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-6::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace2_6_gnbs(ctx: &Ctx) -> Script {
    trace2_6_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_6_gnbs_ontimer110000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-6#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace2_6_gnbs_ontimer110000(ctx: &Ctx) -> Script {
    trace2_6_gnbs_ontimer110000_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_6_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace2-6#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace2_6_gnbs_oninit(ctx: &Ctx) -> Script {
    trace2_6_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace2_6_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace2-6#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace2_6_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace2_6_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer2_6(ctx: &Ctx) -> Script {
    timer2_6_run(ctx, Timer26Step::Start, Vec::new()).map(|_| ())
}

pub fn timer2_6_oninit(ctx: &Ctx) -> Script {
    timer2_6_run(ctx, Timer26Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer2_6_oncommandon(ctx: &Ctx) -> Script {
    timer2_6_run(ctx, Timer26Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer2_6_oncommandoff(ctx: &Ctx) -> Script {
    timer2_6_run(ctx, Timer26Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer2_6_ontimer3000(ctx: &Ctx) -> Script {
    timer2_6_run(ctx, Timer26Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem2_6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 8 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(9))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem2_6(ctx: &Ctx) -> Script {
    getitem2_6_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_6_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-6")])?;
    return Err(Stop::End);
}

pub fn getitem2_6_oninit(ctx: &Ctx) -> Script {
    getitem2_6_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_6_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem2-6")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer2-6::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start02#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem2_6_oncommandon(ctx: &Ctx) -> Script {
    getitem2_6_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem2_6_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem2-6")])?;
    return Err(Stop::End);
}

pub fn getitem2_6_oncommandoff(ctx: &Ctx) -> Script {
    getitem2_6_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn start03_gnbs_run(ctx: &Ctx, mut step: Start03GnbsStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Start03GnbsStep::Start => {
                step = Start03GnbsStep::OnInit;
                continue 'machine;
            }
            Start03GnbsStep::OnInit => {
                ctx.call(Function::Sleep, vec![Val::from(10000)])?;
                step = Start03GnbsStep::OnCommandOn;
                continue 'machine;
            }
            Start03GnbsStep::OnCommandOn => {
                ctx.call(
                    Function::DoNpcEvent,
                    vec![
                        ((Val::from("trace3-") + ctx.call(Function::Rand, vec![Val::from(1), Val::from(7)])?)
                            + Val::from("#gnbs::OnCommandOn")),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn start03_gnbs(ctx: &Ctx) -> Script {
    start03_gnbs_run(ctx, Start03GnbsStep::Start, Vec::new()).map(|_| ())
}

pub fn start03_gnbs_oninit(ctx: &Ctx) -> Script {
    start03_gnbs_run(ctx, Start03GnbsStep::OnInit, Vec::new()).map(|_| ())
}

pub fn start03_gnbs_oncommandon(ctx: &Ctx) -> Script {
    start03_gnbs_run(ctx, Start03GnbsStep::OnCommandOn, Vec::new()).map(|_| ())
}

fn trace3_1_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-1::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_1_gnbs(ctx: &Ctx) -> Script {
    trace3_1_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_1_gnbs_ontimer200000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-1#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_1_gnbs_ontimer200000(ctx: &Ctx) -> Script {
    trace3_1_gnbs_ontimer200000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_1_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-1#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_1_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_1_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_1_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-1#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_1_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_1_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_1(ctx: &Ctx) -> Script {
    timer3_1_run(ctx, Timer31Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_1_oninit(ctx: &Ctx) -> Script {
    timer3_1_run(ctx, Timer31Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_1_oncommandon(ctx: &Ctx) -> Script {
    timer3_1_run(ctx, Timer31Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_1_oncommandoff(ctx: &Ctx) -> Script {
    timer3_1_run(ctx, Timer31Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_1_ontimer3000(ctx: &Ctx) -> Script {
    timer3_1_run(ctx, Timer31Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_1(ctx: &Ctx) -> Script {
    getitem3_1_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_1_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-1")])?;
    return Err(Stop::End);
}

pub fn getitem3_1_oninit(ctx: &Ctx) -> Script {
    getitem3_1_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_1_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-1")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-1::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_1_oncommandon(ctx: &Ctx) -> Script {
    getitem3_1_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_1_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-1")])?;
    return Err(Stop::End);
}

pub fn getitem3_1_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_1_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_2_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-2::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_2_gnbs(ctx: &Ctx) -> Script {
    trace3_2_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_2_gnbs_ontimer130000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-2#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_2_gnbs_ontimer130000(ctx: &Ctx) -> Script {
    trace3_2_gnbs_ontimer130000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_2_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-2#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_2_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_2_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_2_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-2#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_2_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_2_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_2(ctx: &Ctx) -> Script {
    timer3_2_run(ctx, Timer32Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_2_oninit(ctx: &Ctx) -> Script {
    timer3_2_run(ctx, Timer32Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_2_oncommandon(ctx: &Ctx) -> Script {
    timer3_2_run(ctx, Timer32Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_2_oncommandoff(ctx: &Ctx) -> Script {
    timer3_2_run(ctx, Timer32Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_2_ontimer3000(ctx: &Ctx) -> Script {
    timer3_2_run(ctx, Timer32Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_2(ctx: &Ctx) -> Script {
    getitem3_2_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_2_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-2")])?;
    return Err(Stop::End);
}

pub fn getitem3_2_oninit(ctx: &Ctx) -> Script {
    getitem3_2_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_2_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-2")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-2::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_2_oncommandon(ctx: &Ctx) -> Script {
    getitem3_2_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_2_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-2")])?;
    return Err(Stop::End);
}

pub fn getitem3_2_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_2_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_3_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-3::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_3_gnbs(ctx: &Ctx) -> Script {
    trace3_3_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_3_gnbs_ontimer110000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-3#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_3_gnbs_ontimer110000(ctx: &Ctx) -> Script {
    trace3_3_gnbs_ontimer110000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_3_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-3#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_3_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_3_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_3_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-3#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_3_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_3_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_3(ctx: &Ctx) -> Script {
    timer3_3_run(ctx, Timer33Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_3_oninit(ctx: &Ctx) -> Script {
    timer3_3_run(ctx, Timer33Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_3_oncommandon(ctx: &Ctx) -> Script {
    timer3_3_run(ctx, Timer33Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_3_oncommandoff(ctx: &Ctx) -> Script {
    timer3_3_run(ctx, Timer33Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_3_ontimer3000(ctx: &Ctx) -> Script {
    timer3_3_run(ctx, Timer33Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_3_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_3(ctx: &Ctx) -> Script {
    getitem3_3_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_3_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-3")])?;
    return Err(Stop::End);
}

pub fn getitem3_3_oninit(ctx: &Ctx) -> Script {
    getitem3_3_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_3_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-3")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-3::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_3_oncommandon(ctx: &Ctx) -> Script {
    getitem3_3_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_3_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-3")])?;
    return Err(Stop::End);
}

pub fn getitem3_3_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_3_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_4_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-4::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_4_gnbs(ctx: &Ctx) -> Script {
    trace3_4_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_4_gnbs_ontimer230000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-4#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_4_gnbs_ontimer230000(ctx: &Ctx) -> Script {
    trace3_4_gnbs_ontimer230000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_4_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-4#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_4_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_4_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_4_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-4#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_4_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_4_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_4(ctx: &Ctx) -> Script {
    timer3_4_run(ctx, Timer34Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_4_oninit(ctx: &Ctx) -> Script {
    timer3_4_run(ctx, Timer34Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_4_oncommandon(ctx: &Ctx) -> Script {
    timer3_4_run(ctx, Timer34Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_4_oncommandoff(ctx: &Ctx) -> Script {
    timer3_4_run(ctx, Timer34Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_4_ontimer3000(ctx: &Ctx) -> Script {
    timer3_4_run(ctx, Timer34Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_4_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_4(ctx: &Ctx) -> Script {
    getitem3_4_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_4_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-4")])?;
    return Err(Stop::End);
}

pub fn getitem3_4_oninit(ctx: &Ctx) -> Script {
    getitem3_4_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_4_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-4")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-4::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_4_oncommandon(ctx: &Ctx) -> Script {
    getitem3_4_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_4_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-4")])?;
    return Err(Stop::End);
}

pub fn getitem3_4_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_4_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_5_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-5::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_5_gnbs(ctx: &Ctx) -> Script {
    trace3_5_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_5_gnbs_ontimer190000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-5#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_5_gnbs_ontimer190000(ctx: &Ctx) -> Script {
    trace3_5_gnbs_ontimer190000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_5_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-5#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_5_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_5_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_5_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-5#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_5_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_5_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_5(ctx: &Ctx) -> Script {
    timer3_5_run(ctx, Timer35Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_5_oninit(ctx: &Ctx) -> Script {
    timer3_5_run(ctx, Timer35Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_5_oncommandon(ctx: &Ctx) -> Script {
    timer3_5_run(ctx, Timer35Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_5_oncommandoff(ctx: &Ctx) -> Script {
    timer3_5_run(ctx, Timer35Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_5_ontimer3000(ctx: &Ctx) -> Script {
    timer3_5_run(ctx, Timer35Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_5_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_5(ctx: &Ctx) -> Script {
    getitem3_5_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_5_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-5")])?;
    return Err(Stop::End);
}

pub fn getitem3_5_oninit(ctx: &Ctx) -> Script {
    getitem3_5_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_5_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-5")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-5::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_5_oncommandon(ctx: &Ctx) -> Script {
    getitem3_5_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_5_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-5")])?;
    return Err(Stop::End);
}

pub fn getitem3_5_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_5_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_6_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-6::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_6_gnbs(ctx: &Ctx) -> Script {
    trace3_6_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_6_gnbs_ontimer110000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-6#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_6_gnbs_ontimer110000(ctx: &Ctx) -> Script {
    trace3_6_gnbs_ontimer110000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_6_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-6#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_6_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_6_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_6_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-6#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_6_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_6_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_6(ctx: &Ctx) -> Script {
    timer3_6_run(ctx, Timer36Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_6_oninit(ctx: &Ctx) -> Script {
    timer3_6_run(ctx, Timer36Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_6_oncommandon(ctx: &Ctx) -> Script {
    timer3_6_run(ctx, Timer36Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_6_oncommandoff(ctx: &Ctx) -> Script {
    timer3_6_run(ctx, Timer36Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_6_ontimer3000(ctx: &Ctx) -> Script {
    timer3_6_run(ctx, Timer36Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_6_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_6(ctx: &Ctx) -> Script {
    getitem3_6_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_6_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-6")])?;
    return Err(Stop::End);
}

pub fn getitem3_6_oninit(ctx: &Ctx) -> Script {
    getitem3_6_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_6_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-6")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-6::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_6_oncommandon(ctx: &Ctx) -> Script {
    getitem3_6_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_6_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-6")])?;
    return Err(Stop::End);
}

pub fn getitem3_6_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_6_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_7_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? == 9 {
        ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-7::OnCommandOn")])?;
    }
    return Err(Stop::End);
}

pub fn trace3_7_gnbs(ctx: &Ctx) -> Script {
    trace3_7_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_7_gnbs_ontimer110000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-7#gnbs")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn trace3_7_gnbs_ontimer110000(ctx: &Ctx) -> Script {
    trace3_7_gnbs_ontimer110000_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_7_gnbs_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::InitNpcTimer, vec![])?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    ctx.call(Function::DisableNpc, vec![Val::from("trace3-7#gnbs")])?;
    return Err(Stop::End);
}

pub fn trace3_7_gnbs_oninit(ctx: &Ctx) -> Script {
    trace3_7_gnbs_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn trace3_7_gnbs_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("trace3-7#gnbs")])?;
    ctx.call(Function::SetNpcTimer, vec![Val::from(0)])?;
    ctx.call(Function::StartNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn trace3_7_gnbs_oncommandon(ctx: &Ctx) -> Script {
    trace3_7_gnbs_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

pub fn timer3_7(ctx: &Ctx) -> Script {
    timer3_7_run(ctx, Timer37Step::Start, Vec::new()).map(|_| ())
}

pub fn timer3_7_oninit(ctx: &Ctx) -> Script {
    timer3_7_run(ctx, Timer37Step::OnInit, Vec::new()).map(|_| ())
}

pub fn timer3_7_oncommandon(ctx: &Ctx) -> Script {
    timer3_7_run(ctx, Timer37Step::OnCommandOn, Vec::new()).map(|_| ())
}

pub fn timer3_7_oncommandoff(ctx: &Ctx) -> Script {
    timer3_7_run(ctx, Timer37Step::OnCommandOff, Vec::new()).map(|_| ())
}

pub fn timer3_7_ontimer3000(ctx: &Ctx) -> Script {
    timer3_7_run(ctx, Timer37Step::OnTimer3000, Vec::new()).map(|_| ())
}

fn getitem3_7_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()? != 9 {
        return Err(Stop::End);
    }
    ctx.var("b_sword").set(Val::from(10))?;
    ctx.lines(args![
        "You found a ^FF0000piece of blade^000000.",
        "Seems like it's a part of the sword you've been looking for."
    ])?;
    ctx.next()?;
    ctx.lines(args![
        "You put all the pieces on the",
        "ground and assembled them.",
        "It looks like you've found all of the pieces of the blade."
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn getitem3_7(ctx: &Ctx) -> Script {
    getitem3_7_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_7_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-7")])?;
    return Err(Stop::End);
}

pub fn getitem3_7_oninit(ctx: &Ctx) -> Script {
    getitem3_7_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_7_oncommandon_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("#getitem3-7")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("timer3-7::OnCommandOff")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("start03#gnbs::OnCommandOn")])?;
    return Err(Stop::End);
}

pub fn getitem3_7_oncommandon(ctx: &Ctx) -> Script {
    getitem3_7_oncommandon_body(ctx, Vec::new()).map(|_| ())
}

fn getitem3_7_oncommandoff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("#getitem3-7")])?;
    return Err(Stop::End);
}

pub fn getitem3_7_oncommandoff(ctx: &Ctx) -> Script {
    getitem3_7_oncommandoff_body(ctx, Vec::new()).map(|_| ())
}

fn madam_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Shang Hua Yen",
        args!["Ho, ho, ho~", "Who might this be?", "A visitor from out of town~", "Welcome!"],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Hello, Madam~:Where's the chief?")])?) == 1 {
        ctx.lines_as("Shang Hua Yen", args!["Hello, darling~"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_FEMALE")?) {
        ctx.lines_as(
            "Shang Hua Yen",
            args![
                "Hoho~ he's upstairs.",
                "My, are you such a pretty lady~",
                "Just don't be too enraptured by my husband, alright?",
                "Tee hee~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Shang Hua Yen",
        args![
            "Oh, he's upstairs...",
            "My~! Aren't you a darling young",
            "man. But still, not nearly as handsome as my husband~",
            "Tee hee~"
        ],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn madam_gnbs(ctx: &Ctx) -> Script {
    madam_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn strange_dead_body_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 7 {
        ctx.lines(args!["Here's a decomposing corpse.", "It seems like monsters devoured it."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("b_sword").get()?.number()? > 6 && ctx.var("b_sword").get()?.number()? < 11) {
        ctx.lines(args![
            "There's a written message between the bones of the corpse.",
            "of the corpse.",
            "Take a look?"
        ])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Yes.:Eew, no!")])?) == 1 {
            ctx.lines(args![
                "^3355FFThe paper is old and fading,",
                "but there are words on it.",
                "It's written in blood.",
                "The letters are faded and it's almost impossible to read...^000000 "
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^FF0000Damn I can't.. believe.. failed..",
                "Sayum... I should've..",
                "watchout.. ehhh.. but.. break",
                "..pieces.. and separate..",
                "I.. with",
                "this...^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFThe rest was indecipherable...",
                "He probably couldn't take it any longer...^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            ((Val::from("[ ^6699FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000 ]")),
            "Eew, no!",
            " ",
            "Yucky yucky YUCKY!"
        ])?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou feel like there's something important here...",
            "But the rotting carcass has a",
            "foul odor that makes you feel nauseated.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.var("b_sword").get()? == 11 || ctx.var("b_sword").get()? == 12) {
        ctx.lines(args!["^3355FFIt's a rotting human corpse...", "I should just pass by.^000000"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        "^3355FFA skeleton chewed up by",
        "monsters...I feel sorry for",
        "him, but start getting this",
        "really creepy feeling after staring at it for a while.^000000"
    ])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn strange_dead_body_gnbs(ctx: &Ctx) -> Script {
    strange_dead_body_gnbs_body(ctx, Vec::new()).map(|_| ())
}

fn girl_gnbs1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 12 {
        ctx.lines_as(
            "Shi Ying Xiao",
            args![
                "..........",
                "I was happy meeting visitors",
                "from outside the village,",
                "but because of the thief, I feel terrible now..."
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Shi Ying Xiao",
        args![
            "Hehe...",
            "I am so happy now~",
            "Lots of visitors are coming to our",
            "village now. But most of all,",
            "Someone found my father's heirloom~"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("I'm glad you're relieved.:That would be me!")],
    )?) == 1
    {
        ctx.lines_as(
            "Shi Ying Xiao",
            args![
                "Yes, the mood of the entire",
                "village seems to have calmed...",
                "I'm sure whoever found my father's sword is a great person."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        ((Val::from("[ ^6699FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000 ]")),
        "That would be me!"
    ])?;
    ctx.next()?;
    ctx.lines(args![
        ((Val::from("[ ^6699FF") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("^000000 ]")),
        "I AM YOUR HERO!"
    ])?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    ctx.lines_as(
        "Shi Ying Xiao",
        args!["Thank you so much for bringing hope to our village, hero~"],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn girl_gnbs1(ctx: &Ctx) -> Script {
    girl_gnbs1_body(ctx, Vec::new()).map(|_| ())
}

fn stranger_gnbs_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("b_sword").get()?.number()? < 14 {
        ctx.lines_as("Zuo Hei", args!["Hmm...", "I'm pretty busy right now, come back later."])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("b_sword").get()?.number()? < 17 {
        let subject1 = ctx.var("b_sword").get()?;
        if subject1 == 14 {
            let subject2 = ctx.var("nakha").get()?;
            if subject2 == 0 {
                ctx.lines_as(
                    "Zuo Hei",
                    args!["Hmm...", "What is it?", "What do you want?", "I don't like to be bothered."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Ask about the blacksmith.:Why are you being so mean?")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Zuo Hei",
                        args![
                            "Ahh..",
                            "So you're the one who found",
                            "the sword, eh?",
                            "Did you also get asked to repair it?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Exactly.:No..I'm just...")])?) == 1 {
                        ctx.lines_as(
                            "Zuo Hei",
                            args![
                                "Hmm...",
                                "If you want some information from",
                                "me, come back after helping the",
                                "person in the village who's in",
                                "trouble right now."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Zuo Hei",
                            args![
                                "It's another heirloom",
                                "problem, but I want to",
                                "see for myself that",
                                "you're really interested",
                                "in helping others."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(16))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Zuo Hei",
                        args![
                            "Hmm, I didn't think so...",
                            "Now, don't bother me anymore.",
                            "I've got a bunch of things to do."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Grr...",
                        "Why should you care about the",
                        "someone else's personality?",
                        "That's none of your business.",
                        "Take a look in the mirror first",
                        "before you say things like that."
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 1 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Hmm...",
                        "You're in the middle of helping",
                        "someone now, I hear.",
                        "Why don't you go and take care of them first.",
                        "Once you start to help someone,",
                        "you can't just quit halfway."
                    ],
                )?;
                ctx.var("b_sword").set(Val::from(16))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 2 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Hmm...",
                        "You're in the middle of helping",
                        "someone now, I hear.",
                        "Why don't you go and take care of them first.",
                        "Once you start to help someone,",
                        "you can't just quit halfway."
                    ],
                )?;
                ctx.var("b_sword").set(Val::from(16))?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 3 {
                ctx.lines_as("Zuo Hei", args!["Hmm...", "What do you want??"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Ask about the blacksmith.:Why are you being so mean?")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Zuo Hei",
                        args![
                            "Ahh...",
                            "So you are the one who found",
                            "the sword, eh? I assume you were also asked to repair it..."
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Exactly.:No...I'm just...")])?) == 1 {
                        ctx.lines_as(
                            "Zuo Hei",
                            args![
                                "Hmm...",
                                "Since it's been shattered,",
                                "you'll need a very skilled smith.",
                                "Go to ^FF0000Geffen^000000, you'll find someone",
                                "who can help you."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(17))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Zuo Hei",
                        args![
                            "Hmm, I didn't think so...",
                            "Now, don't bother me anymore.",
                            "I've got a bunch of things to do."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Grr...",
                        "Why should you care about the",
                        "someone else's personality?",
                        "That's none of your business.",
                        "Take a look in the mirror first",
                        "before you say things like that."
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject2 == 4 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Guess I was wasting my time.",
                        "If you make a promise to someone,",
                        "it's your responsibility to",
                        "follow through with it to completion."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "I don't like people who fail to",
                        "keep the promises they make.",
                        "You'll not get any information from me.",
                        "Don't ever bother me again."
                    ],
                )?;
                ctx.var("b_sword").set(Val::from(15))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if subject1 == 15 {
            ctx.lines_as("Zuo Hei", args!["Sorry, I'm busy right now.", "Why don't you come back later."])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if subject1 == 16 {
            let subject3 = ctx.var("nakha").get()?;
            if subject3 == 0 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Hmm......",
                        "You're not done with the",
                        "requirement I've given you...",
                        "I can't give you any information",
                        "until you finish your job."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject3 == 1 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Hmm....",
                        "Once you start to help someone,",
                        "you can't just quit halfway.",
                        "Why don't you go and take care of them first."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject3 == 2 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Hmm....",
                        "Once you start to help someone,",
                        "you can't just quit halfway.",
                        "Why don't you go and take care of them first."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject3 == 3 {
                ctx.lines_as("Zuo Hei", args!["Hmm......", "What do you want?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Ask about the blacksmith.:Why are you being so mean?")],
                )?) == 1
                {
                    ctx.lines_as(
                        "Zuo Hei",
                        args![
                            "Ahh...",
                            "So you are the one who found",
                            "the sword eh?",
                            "Did you also get asked to repair it?"
                        ],
                    )?;
                    ctx.next()?;
                    if Val::from(runtime::select_values(ctx, &[Val::from("Exactly.:No, i'm just...")])?) == 1 {
                        ctx.lines_as(
                            "Zuo Hei",
                            args![
                                "Hmm...",
                                "Since it's been shattered,",
                                "you'll need a very skilled smith.",
                                "Go to ^FF0000Geffen^000000, you'll find someone",
                                "who can help you."
                            ],
                        )?;
                        ctx.var("b_sword").set(Val::from(17))?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Zuo Hei",
                        args![
                            "Oh, I suppose not.",
                            "Why are you asking me, then?",
                            "Don't bother me anymore.",
                            "I've got a bunch of things to do."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Grr...",
                        "Why should you care about the",
                        "someone else's personality?",
                        "That's none of your business.",
                        "Take a look in the mirror first",
                        "before you say things like that."
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_ANGER")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if subject3 == 4 {
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "Guess I was wasting my time.",
                        "If you make a promise to",
                        "someone, it's your",
                        "responsibility to",
                        "follow through with it to completion."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Zuo Hei",
                    args![
                        "I don't like people who fail",
                        "to keep their promises.",
                        "You'll not get any information from me. Don't ever bother me again."
                    ],
                )?;
                ctx.var("b_sword").set(Val::from(15))?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if ctx.var("b_sword").get()?.number()? < 33 {
        ctx.lines_as(
            "Zuo Hei",
            args![
                "Hey there...uhm..",
                "Didn't I tell you to go to ^FF0000Geffen^000000?",
                "You'll find a famous blacksmith",
                "that can repair the sword for you.",
                "That's all the information I can",
                "give you, really."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Zuo Hei", args!["It's up to you to make good use of it."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Zuo Hei",
        args![
            "Hmm...",
            "Helping people in trouble is",
            "such a nice thing to do.",
            "You are doing the right thing."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn stranger_gnbs(ctx: &Ctx) -> Script {
    stranger_gnbs_body(ctx, Vec::new()).map(|_| ())
}
