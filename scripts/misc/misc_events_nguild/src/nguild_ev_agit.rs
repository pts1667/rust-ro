#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args};

#[derive(Clone, Copy, Debug)]
enum AgitN01Step {
    Start,
    OnInterIfInitOnce,
    OnRecvCastleN01,
    OnAgitStart,
    OnAgitBreak,
    OnGuildBreak,
    OnAgitEliminate,
    OnAgitEnd,
}

fn agit_n01_run(ctx: &Ctx, mut step: AgitN01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AgitN01Step::Start => {
                step = AgitN01Step::OnInterIfInitOnce;
                continue 'machine;
            }
            AgitN01Step::OnInterIfInitOnce => {
                ctx.call(Function::DoNpcEvent, args!["::OnRecvCastleN01"])?;
                return Err(Stop::End);
            }
            AgitN01Step::OnRecvCastleN01 => {
                ctx.call(
                    Function::RequestGuildInfo,
                    args![ctx.call(Function::GetCastleData, args!["nguild_alde", 1])?],
                )?;
                return Err(Stop::End);
            }
            AgitN01Step::OnAgitStart => {
                shared::events_nguild_nguild_ev_agit::f_agitstart(ctx, args!["nguild_alde", "N01", 216, 24])?;
                return Err(Stop::End);
            }
            AgitN01Step::OnAgitBreak => {
                shared::events_nguild_nguild_ev_agit::f_agitbreak(ctx, args!["nguild_alde", "N01"])?;
                step = AgitN01Step::OnAgitEliminate;
                continue 'machine;
            }
            AgitN01Step::OnGuildBreak => {
                shared::events_nguild_nguild_ev_agit::f_guildbreak(ctx, args!["nguild_alde", "N01"])?;
                return Err(Stop::End);
            }
            AgitN01Step::OnAgitEliminate => {
                ctx.call(
                    Function::MapRespawnGuildId,
                    args!["nguild_alde", ctx.call(Function::GetCastleData, args!["nguild_alde", 1])?, 6,],
                )?;
                ctx.call(
                    Function::Monster,
                    args!["nguild_alde", 216, 24, "EMPERIUM", 1288, 1, "Agit_N01::OnAgitBreak",],
                )?;
                return Err(Stop::End);
            }
            AgitN01Step::OnAgitEnd => {
                shared::events_nguild_nguild_ev_agit::f_agitend(ctx, args!["nguild_alde", "N01"])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn agit_n01(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::Start, Vec::new()).map(|_| ())
}

pub fn agit_n01_oninterifinitonce(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnInterIfInitOnce, Vec::new()).map(|_| ())
}

pub fn agit_n01_onrecvcastlen01(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnRecvCastleN01, Vec::new()).map(|_| ())
}

pub fn agit_n01_onagitstart(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnAgitStart, Vec::new()).map(|_| ())
}

pub fn agit_n01_onagitbreak(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnAgitBreak, Vec::new()).map(|_| ())
}

pub fn agit_n01_onguildbreak(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnGuildBreak, Vec::new()).map(|_| ())
}

pub fn agit_n01_onagiteliminate(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnAgitEliminate, Vec::new()).map(|_| ())
}

pub fn agit_n01_onagitend(ctx: &Ctx) -> Script {
    agit_n01_run(ctx, AgitN01Step::OnAgitEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AgitN02Step {
    Start,
    OnInterIfInitOnce,
    OnRecvCastleN02,
    OnAgitStart,
    OnAgitBreak,
    OnGuildBreak,
    OnAgitEliminate,
    OnAgitEnd,
}

fn agit_n02_run(ctx: &Ctx, mut step: AgitN02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AgitN02Step::Start => {
                step = AgitN02Step::OnInterIfInitOnce;
                continue 'machine;
            }
            AgitN02Step::OnInterIfInitOnce => {
                ctx.call(Function::DoNpcEvent, args!["::OnRecvCastleN02"])?;
                return Err(Stop::End);
            }
            AgitN02Step::OnRecvCastleN02 => {
                ctx.call(
                    Function::RequestGuildInfo,
                    args![ctx.call(Function::GetCastleData, args!["nguild_gef", 1])?],
                )?;
                return Err(Stop::End);
            }
            AgitN02Step::OnAgitStart => {
                shared::events_nguild_nguild_ev_agit::f_agitstart(ctx, args!["nguild_gef", "N02", 198, 182])?;
                return Err(Stop::End);
            }
            AgitN02Step::OnAgitBreak => {
                shared::events_nguild_nguild_ev_agit::f_agitbreak(ctx, args!["nguild_gef", "N02"])?;
                step = AgitN02Step::OnAgitEliminate;
                continue 'machine;
            }
            AgitN02Step::OnGuildBreak => {
                shared::events_nguild_nguild_ev_agit::f_guildbreak(ctx, args!["nguild_gef", "N02"])?;
                return Err(Stop::End);
            }
            AgitN02Step::OnAgitEliminate => {
                ctx.call(
                    Function::MapRespawnGuildId,
                    args!["nguild_gef", ctx.call(Function::GetCastleData, args!["nguild_gef", 1])?, 6,],
                )?;
                ctx.call(
                    Function::Monster,
                    args!["nguild_gef", 198, 182, "EMPERIUM", 1288, 1, "Agit_N02::OnAgitBreak",],
                )?;
                return Err(Stop::End);
            }
            AgitN02Step::OnAgitEnd => {
                shared::events_nguild_nguild_ev_agit::f_agitend(ctx, args!["nguild_gef", "N02"])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn agit_n02(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::Start, Vec::new()).map(|_| ())
}

pub fn agit_n02_oninterifinitonce(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnInterIfInitOnce, Vec::new()).map(|_| ())
}

pub fn agit_n02_onrecvcastlen02(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnRecvCastleN02, Vec::new()).map(|_| ())
}

pub fn agit_n02_onagitstart(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnAgitStart, Vec::new()).map(|_| ())
}

pub fn agit_n02_onagitbreak(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnAgitBreak, Vec::new()).map(|_| ())
}

pub fn agit_n02_onguildbreak(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnGuildBreak, Vec::new()).map(|_| ())
}

pub fn agit_n02_onagiteliminate(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnAgitEliminate, Vec::new()).map(|_| ())
}

pub fn agit_n02_onagitend(ctx: &Ctx) -> Script {
    agit_n02_run(ctx, AgitN02Step::OnAgitEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AgitN03Step {
    Start,
    OnInterIfInitOnce,
    OnRecvCastleN03,
    OnAgitStart,
    OnAgitBreak,
    OnGuildBreak,
    OnAgitEliminate,
    OnAgitEnd,
}

fn agit_n03_run(ctx: &Ctx, mut step: AgitN03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AgitN03Step::Start => {
                step = AgitN03Step::OnInterIfInitOnce;
                continue 'machine;
            }
            AgitN03Step::OnInterIfInitOnce => {
                ctx.call(Function::DoNpcEvent, args!["::OnRecvCastleN03"])?;
                return Err(Stop::End);
            }
            AgitN03Step::OnRecvCastleN03 => {
                ctx.call(
                    Function::RequestGuildInfo,
                    args![ctx.call(Function::GetCastleData, args!["nguild_pay", 1])?],
                )?;
                return Err(Stop::End);
            }
            AgitN03Step::OnAgitStart => {
                shared::events_nguild_nguild_ev_agit::f_agitstart(ctx, args!["nguild_pay", "N03", 139, 139])?;
                return Err(Stop::End);
            }
            AgitN03Step::OnAgitBreak => {
                shared::events_nguild_nguild_ev_agit::f_agitbreak(ctx, args!["nguild_pay", "N03"])?;
                step = AgitN03Step::OnAgitEliminate;
                continue 'machine;
            }
            AgitN03Step::OnGuildBreak => {
                shared::events_nguild_nguild_ev_agit::f_guildbreak(ctx, args!["nguild_pay", "N03"])?;
                return Err(Stop::End);
            }
            AgitN03Step::OnAgitEliminate => {
                ctx.call(
                    Function::MapRespawnGuildId,
                    args!["nguild_pay", ctx.call(Function::GetCastleData, args!["nguild_pay", 1])?, 6,],
                )?;
                ctx.call(
                    Function::Monster,
                    args!["nguild_pay", 139, 139, "EMPERIUM", 1288, 1, "Agit_N03::OnAgitBreak",],
                )?;
                return Err(Stop::End);
            }
            AgitN03Step::OnAgitEnd => {
                shared::events_nguild_nguild_ev_agit::f_agitend(ctx, args!["nguild_pay", "N03"])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn agit_n03(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::Start, Vec::new()).map(|_| ())
}

pub fn agit_n03_oninterifinitonce(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnInterIfInitOnce, Vec::new()).map(|_| ())
}

pub fn agit_n03_onrecvcastlen03(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnRecvCastleN03, Vec::new()).map(|_| ())
}

pub fn agit_n03_onagitstart(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnAgitStart, Vec::new()).map(|_| ())
}

pub fn agit_n03_onagitbreak(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnAgitBreak, Vec::new()).map(|_| ())
}

pub fn agit_n03_onguildbreak(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnGuildBreak, Vec::new()).map(|_| ())
}

pub fn agit_n03_onagiteliminate(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnAgitEliminate, Vec::new()).map(|_| ())
}

pub fn agit_n03_onagitend(ctx: &Ctx) -> Script {
    agit_n03_run(ctx, AgitN03Step::OnAgitEnd, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum AgitN04Step {
    Start,
    OnInterIfInitOnce,
    OnRecvCastleN04,
    OnAgitStart,
    OnAgitBreak,
    OnGuildBreak,
    OnAgitEliminate,
    OnAgitEnd,
}

fn agit_n04_run(ctx: &Ctx, mut step: AgitN04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            AgitN04Step::Start => {
                step = AgitN04Step::OnInterIfInitOnce;
                continue 'machine;
            }
            AgitN04Step::OnInterIfInitOnce => {
                ctx.call(Function::DoNpcEvent, args!["::OnRecvCastleN04"])?;
                return Err(Stop::End);
            }
            AgitN04Step::OnRecvCastleN04 => {
                ctx.call(
                    Function::RequestGuildInfo,
                    args![ctx.call(Function::GetCastleData, args!["nguild_prt", 1])?],
                )?;
                return Err(Stop::End);
            }
            AgitN04Step::OnAgitStart => {
                shared::events_nguild_nguild_ev_agit::f_agitstart(ctx, args!["nguild_prt", "N04", 197, 197])?;
                return Err(Stop::End);
            }
            AgitN04Step::OnAgitBreak => {
                shared::events_nguild_nguild_ev_agit::f_agitbreak(ctx, args!["nguild_prt", "N04"])?;
                step = AgitN04Step::OnAgitEliminate;
                continue 'machine;
            }
            AgitN04Step::OnGuildBreak => {
                shared::events_nguild_nguild_ev_agit::f_guildbreak(ctx, args!["nguild_prt", "N04"])?;
                return Err(Stop::End);
            }
            AgitN04Step::OnAgitEliminate => {
                ctx.call(
                    Function::MapRespawnGuildId,
                    args!["nguild_prt", ctx.call(Function::GetCastleData, args!["nguild_prt", 1])?, 6,],
                )?;
                ctx.call(
                    Function::Monster,
                    args!["nguild_prt", 197, 197, "EMPERIUM", 1288, 1, "Agit_N04::OnAgitBreak",],
                )?;
                return Err(Stop::End);
            }
            AgitN04Step::OnAgitEnd => {
                shared::events_nguild_nguild_ev_agit::f_agitend(ctx, args!["nguild_prt", "N04"])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn agit_n04(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::Start, Vec::new()).map(|_| ())
}

pub fn agit_n04_oninterifinitonce(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnInterIfInitOnce, Vec::new()).map(|_| ())
}

pub fn agit_n04_onrecvcastlen04(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnRecvCastleN04, Vec::new()).map(|_| ())
}

pub fn agit_n04_onagitstart(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnAgitStart, Vec::new()).map(|_| ())
}

pub fn agit_n04_onagitbreak(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnAgitBreak, Vec::new()).map(|_| ())
}

pub fn agit_n04_onguildbreak(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnGuildBreak, Vec::new()).map(|_| ())
}

pub fn agit_n04_onagiteliminate(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnAgitEliminate, Vec::new()).map(|_| ())
}

pub fn agit_n04_onagitend(ctx: &Ctx) -> Script {
    agit_n04_run(ctx, AgitN04Step::OnAgitEnd, Vec::new()).map(|_| ())
}

pub fn treasspawn(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn treasspawn_onclock0005(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_treas::f_gldtreas(
        ctx,
        args![
            "nguild_alde",
            "N01",
            ctx.var("$boxnumn01").get()?,
            ctx.var("$@bxn01").get()?,
            ctx.var("$@boxidn01").get()?,
            1324,
            114,
            218,
            123,
            227,
            0,
        ],
    )?;
    shared::events_nguild_nguild_treas::f_gldtreas(
        ctx,
        args![
            "nguild_gef",
            "N02",
            ctx.var("$boxnumn02").get()?,
            ctx.var("$@bxn02").get()?,
            ctx.var("$@boxidn02").get()?,
            1334,
            150,
            108,
            158,
            114,
            0,
        ],
    )?;
    shared::events_nguild_nguild_treas::f_gldtreas(
        ctx,
        args![
            "nguild_pay",
            "N03",
            ctx.var("$boxnumn03").get()?,
            ctx.var("$@bxn03").get()?,
            ctx.var("$@boxidn03").get()?,
            1344,
            286,
            4,
            295,
            13,
            0,
        ],
    )?;
    shared::events_nguild_nguild_treas::f_gldtreas(
        ctx,
        args![
            "nguild_prt",
            "N04",
            ctx.var("$boxnumn04").get()?,
            ctx.var("$@bxn04").get()?,
            ctx.var("$@boxidn04").get()?,
            1354,
            6,
            204,
            15,
            213,
            0,
        ],
    )?;
    ctx.end()
}
