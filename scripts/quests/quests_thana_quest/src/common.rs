use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum CharmStoneAdmintt01Step {
    Start,
    OnInit,
    OnEnable,
    OnTimer1000,
    OnTimer600000,
    OnTimer1200000,
    OnTimer1800000,
    OnTimer2400000,
}

pub(super) fn charm_stone_admintt01_run(ctx: &Ctx, mut step: CharmStoneAdmintt01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CharmStoneAdmintt01Step::Start => {
                return Err(Stop::End);
            }
            CharmStoneAdmintt01Step::OnInit => {
                step = CharmStoneAdmintt01Step::OnEnable;
                continue 'machine;
            }
            CharmStoneAdmintt01Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CharmStoneAdmintt01Step::OnTimer1000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y1::OnEnable")])?;
                return Err(Stop::End);
            }
            CharmStoneAdmintt01Step::OnTimer600000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y2::OnEnable")])?;
                return Err(Stop::End);
            }
            CharmStoneAdmintt01Step::OnTimer1200000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y3::OnEnable")])?;
                return Err(Stop::End);
            }
            CharmStoneAdmintt01Step::OnTimer1800000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r4::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g4::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b4::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y4::OnEnable")])?;
                return Err(Stop::End);
            }
            CharmStoneAdmintt01Step::OnTimer2400000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_r4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_g4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_b4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Shining Crystal#tt_y4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#Charm Stone Admintt01::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tteffect01Step {
    Start,
    OnEnable,
    OnStop,
    OnTimer500,
    OnTimer1000,
    OnTimer1500,
    OnTimer2000,
    OnTimer2500,
    OnTimer3000,
}

pub(super) fn tteffect01_run(ctx: &Ctx, mut step: Tteffect01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tteffect01Step::Start => {
                return Err(Stop::End);
            }
            Tteffect01Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect01Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect01Step::OnTimer500 => {
                step = Tteffect01Step::OnTimer1000;
                continue 'machine;
            }
            Tteffect01Step::OnTimer1000 => {
                step = Tteffect01Step::OnTimer1500;
                continue 'machine;
            }
            Tteffect01Step::OnTimer1500 => {
                step = Tteffect01Step::OnTimer2000;
                continue 'machine;
            }
            Tteffect01Step::OnTimer2000 => {
                step = Tteffect01Step::OnTimer2500;
                continue 'machine;
            }
            Tteffect01Step::OnTimer2500 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
                return Err(Stop::End);
            }
            Tteffect01Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect01::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tteffect02Step {
    Start,
    OnEnable,
    OnStop,
    OnTimer500,
    OnTimer1000,
    OnTimer1500,
    OnTimer2000,
    OnTimer2500,
    OnTimer3000,
}

pub(super) fn tteffect02_run(ctx: &Ctx, mut step: Tteffect02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tteffect02Step::Start => {
                return Err(Stop::End);
            }
            Tteffect02Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect02Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect02Step::OnTimer500 => {
                step = Tteffect02Step::OnTimer1000;
                continue 'machine;
            }
            Tteffect02Step::OnTimer1000 => {
                step = Tteffect02Step::OnTimer1500;
                continue 'machine;
            }
            Tteffect02Step::OnTimer1500 => {
                step = Tteffect02Step::OnTimer2000;
                continue 'machine;
            }
            Tteffect02Step::OnTimer2000 => {
                step = Tteffect02Step::OnTimer2500;
                continue 'machine;
            }
            Tteffect02Step::OnTimer2500 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL4")?])?;
                return Err(Stop::End);
            }
            Tteffect02Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect02::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tteffect03Step {
    Start,
    OnEnable,
    OnStop,
    OnTimer500,
    OnTimer1000,
    OnTimer1500,
    OnTimer2000,
    OnTimer2500,
    OnTimer3000,
}

pub(super) fn tteffect03_run(ctx: &Ctx, mut step: Tteffect03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tteffect03Step::Start => {
                return Err(Stop::End);
            }
            Tteffect03Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect03Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect03Step::OnTimer500 => {
                step = Tteffect03Step::OnTimer1000;
                continue 'machine;
            }
            Tteffect03Step::OnTimer1000 => {
                step = Tteffect03Step::OnTimer1500;
                continue 'machine;
            }
            Tteffect03Step::OnTimer1500 => {
                step = Tteffect03Step::OnTimer2000;
                continue 'machine;
            }
            Tteffect03Step::OnTimer2000 => {
                step = Tteffect03Step::OnTimer2500;
                continue 'machine;
            }
            Tteffect03Step::OnTimer2500 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL2")?])?;
                return Err(Stop::End);
            }
            Tteffect03Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect03::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tteffect04Step {
    Start,
    OnEnable,
    OnStop,
    OnTimer500,
    OnTimer1000,
    OnTimer1500,
    OnTimer2000,
    OnTimer2500,
    OnTimer3000,
}

pub(super) fn tteffect04_run(ctx: &Ctx, mut step: Tteffect04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tteffect04Step::Start => {
                return Err(Stop::End);
            }
            Tteffect04Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect04Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect04Step::OnTimer500 => {
                step = Tteffect04Step::OnTimer1000;
                continue 'machine;
            }
            Tteffect04Step::OnTimer1000 => {
                step = Tteffect04Step::OnTimer1500;
                continue 'machine;
            }
            Tteffect04Step::OnTimer1500 => {
                step = Tteffect04Step::OnTimer2000;
                continue 'machine;
            }
            Tteffect04Step::OnTimer2000 => {
                step = Tteffect04Step::OnTimer2500;
                continue 'machine;
            }
            Tteffect04Step::OnTimer2500 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL3")?])?;
                return Err(Stop::End);
            }
            Tteffect04Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect04::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tteffect05Step {
    Start,
    OnEnable,
    OnStop,
    OnTimer500,
    OnTimer1000,
    OnTimer1500,
    OnTimer2000,
    OnTimer2500,
    OnTimer3000,
}

pub(super) fn tteffect05_run(ctx: &Ctx, mut step: Tteffect05Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Tteffect05Step::Start => {
                return Err(Stop::End);
            }
            Tteffect05Step::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect05Step::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            Tteffect05Step::OnTimer500 => {
                step = Tteffect05Step::OnTimer1000;
                continue 'machine;
            }
            Tteffect05Step::OnTimer1000 => {
                step = Tteffect05Step::OnTimer1500;
                continue 'machine;
            }
            Tteffect05Step::OnTimer1500 => {
                step = Tteffect05Step::OnTimer2000;
                continue 'machine;
            }
            Tteffect05Step::OnTimer2000 => {
                step = Tteffect05Step::OnTimer2500;
                continue 'machine;
            }
            Tteffect05Step::OnTimer2500 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_BEGINSPELL7")?])?;
                return Err(Stop::End);
            }
            Tteffect05Step::OnTimer3000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("#tteffect05::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum GatetoThanatosStep {
    Start,
    OnInit,
    OnEnable,
    OnOn2,
    OnTouch,
    OnTimer6000,
    OnTimer1000,
    OnTimer5000,
    OnTimer3000,
}

pub(super) fn gateto_thanatos_run(ctx: &Ctx, mut step: GatetoThanatosStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_pillar = Val::from(0);
    let mut l_touch = Val::from(0);
    'machine: loop {
        match step {
            GatetoThanatosStep::Start => {
                return Err(Stop::End);
            }
            GatetoThanatosStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("#gateto_thanatos")])?;
                return Err(Stop::End);
            }
            GatetoThanatosStep::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("#gateto_thanatos")])?;
                step = GatetoThanatosStep::OnOn2;
                continue 'machine;
            }
            GatetoThanatosStep::OnOn2 => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            GatetoThanatosStep::OnTouch => {
                l_touch = Val::from(1);
                step = GatetoThanatosStep::OnTimer6000;
                continue 'machine;
            }
            GatetoThanatosStep::OnTimer6000 => {
                if (ctx.var("$@thana_summon").get()? == 0 || ctx.var("$@thana_summon").get()? == 6) {
                    ctx.call(Function::DisableNpc, vec![Val::from("#gateto_thanatos")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                } else if ctx.var("$@thana_summon").get()? == 5 {
                    if l_touch.clone().is_true() {
                        ctx.call(Function::Warp, vec![Val::from("thana_boss"), Val::from(136), Val::from(116)])?;
                    } else {
                        ctx.call(Function::DoNpcEvent, vec![Val::from("#gateto_thanatos::OnOn2")])?;
                    }
                }
                return Err(Stop::End);
            }
            GatetoThanatosStep::OnTimer1000 => {
                step = GatetoThanatosStep::OnTimer5000;
                continue 'machine;
            }
            GatetoThanatosStep::OnTimer5000 => {
                l_pillar = Val::from(1);
                step = GatetoThanatosStep::OnTimer3000;
                continue 'machine;
            }
            GatetoThanatosStep::OnTimer3000 => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL7")?, ctx.constant("AREA")?, Val::from("#tteffect05")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL3")?, ctx.constant("AREA")?, Val::from("#tteffect04")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL2")?, ctx.constant("AREA")?, Val::from("#tteffect03")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL4")?, ctx.constant("AREA")?, Val::from("#tteffect02")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_BEGINSPELL5")?, ctx.constant("AREA")?, Val::from("#tteffect01")],
                )?;
                if l_pillar.clone().is_true() {
                    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_MAPPILLAR2")?])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum CooltimeThanaStep {
    Start,
    OnInit,
    OnEnable,
    OnStop,
    OnTimer3000,
    OnTimer6000,
    OnTimer16000,
    OnTimer26000,
    OnTimer31000,
    OnTimer32000,
    OnTimer33000,
    OnTimer34000,
    OnTimer35000,
    OnTimer36000,
    OnTimer37000,
    OnTimer7200000,
}

pub(super) fn cooltime_thana_run(ctx: &Ctx, mut step: CooltimeThanaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            CooltimeThanaStep::Start => {
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnInit => {
                ctx.var("$@thana_summon").set(Val::from(0))?;
                ctx.var("$@thana_summon2").set(Val::from(0))?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer3000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("Warning!!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer6000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("The seal will re-activate in 30 seconds."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer16000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("20 seconds left..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer26000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("10 seconds left..."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer31000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("5 seconds."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer32000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("4 seconds."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer33000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("3 seconds."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer34000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("2 seconds."),
                        ctx.constant("BC_MAP")?,
                        Val::from("0x00ff00"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer35000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("thana_boss"),
                        Val::from("Time's up!"),
                        ctx.constant("BC_MAP")?,
                        Val::from("0xff0000"),
                    ],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer36000 => {
                step = CooltimeThanaStep::OnTimer37000;
                continue 'machine;
            }
            CooltimeThanaStep::OnTimer37000 => {
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("thana_boss"), Val::from("tha_t12"), Val::from(130), Val::from(52)],
                )?;
                return Err(Stop::End);
            }
            CooltimeThanaStep::OnTimer7200000 => {
                ctx.var("$@thana_summon").set(Val::from(0))?;
                ctx.var("$@thana_summon2").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Memory Seal#tt1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Memory Seal#tt2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Memory Seal#tt3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Memory Seal#tt4::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#thanatos_seal::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
