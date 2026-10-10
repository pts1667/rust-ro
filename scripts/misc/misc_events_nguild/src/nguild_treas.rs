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
enum TreasureN01Step {
    Start,
    OnRecvCastleN01,
    OnInit,
    OnDied,
}

fn treasure_n01_run(ctx: &Ctx, mut step: TreasureN01Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TreasureN01Step::Start => {
                step = TreasureN01Step::OnRecvCastleN01;
                continue 'machine;
            }
            TreasureN01Step::OnRecvCastleN01 => {
                return Err(Stop::End);
            }
            TreasureN01Step::OnInit => {
                if ctx.var("$boxnumn01").get()? == 0 {
                    return Err(Stop::End);
                }
                ctx.var("$@bxn01").set(ctx.var("$boxnumn01").get()?)?;
                shared::events_nguild_nguild_treas::f_gldtreas(
                    ctx,
                    args![
                        "nguild_alde",
                        "N01",
                        ctx.var("$boxnumn01").get()?,
                        ctx.var("$@bxn01").get()?,
                        ctx.var("$@boxidn01").get()?,
                        Val::from(1324) + ctx.call(Function::Rand, args![10])?,
                        114,
                        218,
                        123,
                        227,
                        1
                    ],
                )?;
                return Err(Stop::End);
            }
            TreasureN01Step::OnDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_alde", "Treasure Chest Broken Open", 17])?;
                ctx.var("$boxnumn01").set(ctx.var("$boxnumn01").get()?.number()? - 1)?;
                if ctx.var("$boxnumn01").get()? == 0 {
                    ctx.call(
                        Function::MapAnnounce,
                        args![
                            "nguild_alde",
                            "All of the treasure boxes have been opened.  You must wait untill the next day for them to appear again.",
                            0
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn treasure_n01(ctx: &Ctx) -> Script {
    treasure_n01_run(ctx, TreasureN01Step::Start, Vec::new()).map(|_| ())
}

pub fn treasure_n01_onrecvcastlen01(ctx: &Ctx) -> Script {
    treasure_n01_run(ctx, TreasureN01Step::OnRecvCastleN01, Vec::new()).map(|_| ())
}

pub fn treasure_n01_oninit(ctx: &Ctx) -> Script {
    treasure_n01_run(ctx, TreasureN01Step::OnInit, Vec::new()).map(|_| ())
}

pub fn treasure_n01_ondied(ctx: &Ctx) -> Script {
    treasure_n01_run(ctx, TreasureN01Step::OnDied, Vec::new()).map(|_| ())
}

pub fn switch_tresn01(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_treas::f_gldtreassw(ctx, args!["nguild_alde", 218, 176])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum TreasureN02Step {
    Start,
    OnRecvCastleN02,
    OnInit,
    OnDied,
}

fn treasure_n02_run(ctx: &Ctx, mut step: TreasureN02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TreasureN02Step::Start => {
                step = TreasureN02Step::OnRecvCastleN02;
                continue 'machine;
            }
            TreasureN02Step::OnRecvCastleN02 => {
                return Err(Stop::End);
            }
            TreasureN02Step::OnInit => {
                if ctx.var("$boxnumn02").get()? == 0 {
                    return Err(Stop::End);
                }
                ctx.var("$@bxn02").set(ctx.var("$boxnumn02").get()?)?;
                shared::events_nguild_nguild_treas::f_gldtreas(
                    ctx,
                    args![
                        "nguild_gef",
                        "N02",
                        ctx.var("$boxnumn02").get()?,
                        ctx.var("$@bxn02").get()?,
                        ctx.var("$@boxidn02").get()?,
                        Val::from(1334) + ctx.call(Function::Rand, args![10])?,
                        150,
                        108,
                        158,
                        114,
                        1
                    ],
                )?;
                return Err(Stop::End);
            }
            TreasureN02Step::OnDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_gef", "Treasure Chest Broken Open", 17])?;
                ctx.var("$boxnumn02").set(ctx.var("$boxnumn02").get()?.number()? - 1)?;
                if ctx.var("$boxnumn02").get()? == 0 {
                    ctx.call(
                        Function::MapAnnounce,
                        args![
                            "nguild_gef",
                            "All of the treasure boxes have been opened.  You must wait untill the next day for them to appear again.",
                            0
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn treasure_n02(ctx: &Ctx) -> Script {
    treasure_n02_run(ctx, TreasureN02Step::Start, Vec::new()).map(|_| ())
}

pub fn treasure_n02_onrecvcastlen02(ctx: &Ctx) -> Script {
    treasure_n02_run(ctx, TreasureN02Step::OnRecvCastleN02, Vec::new()).map(|_| ())
}

pub fn treasure_n02_oninit(ctx: &Ctx) -> Script {
    treasure_n02_run(ctx, TreasureN02Step::OnInit, Vec::new()).map(|_| ())
}

pub fn treasure_n02_ondied(ctx: &Ctx) -> Script {
    treasure_n02_run(ctx, TreasureN02Step::OnDied, Vec::new()).map(|_| ())
}

pub fn switch_tresn02(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_treas::f_gldtreassw(ctx, args!["nguild_gef", 40, 49])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum TreasureN03Step {
    Start,
    OnRecvCastleN03,
    OnInit,
    OnDied,
}

fn treasure_n03_run(ctx: &Ctx, mut step: TreasureN03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TreasureN03Step::Start => {
                step = TreasureN03Step::OnRecvCastleN03;
                continue 'machine;
            }
            TreasureN03Step::OnRecvCastleN03 => {
                return Err(Stop::End);
            }
            TreasureN03Step::OnInit => {
                if ctx.var("$boxnumn03").get()? == 0 {
                    return Err(Stop::End);
                }
                ctx.var("$@bxn03").set(ctx.var("$boxnumn03").get()?)?;
                shared::events_nguild_nguild_treas::f_gldtreas(
                    ctx,
                    args![
                        "nguild_pay",
                        "N03",
                        ctx.var("$boxnumn03").get()?,
                        ctx.var("$@bxn03").get()?,
                        ctx.var("$@boxidn03").get()?,
                        Val::from(1344) + ctx.call(Function::Rand, args![10])?,
                        286,
                        4,
                        295,
                        13,
                        1
                    ],
                )?;
                return Err(Stop::End);
            }
            TreasureN03Step::OnDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_pay", "Treasure Chest Broken Open", 17])?;
                ctx.var("$boxnumn03").set(ctx.var("$boxnumn03").get()?.number()? - 1)?;
                if ctx.var("$boxnumn03").get()? == 0 {
                    ctx.call(
                        Function::MapAnnounce,
                        args![
                            "nguild_pay",
                            "All of the treasure boxes have been opened.  You must wait untill the next day for them to appear again.",
                            0
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn treasure_n03(ctx: &Ctx) -> Script {
    treasure_n03_run(ctx, TreasureN03Step::Start, Vec::new()).map(|_| ())
}

pub fn treasure_n03_onrecvcastlen03(ctx: &Ctx) -> Script {
    treasure_n03_run(ctx, TreasureN03Step::OnRecvCastleN03, Vec::new()).map(|_| ())
}

pub fn treasure_n03_oninit(ctx: &Ctx) -> Script {
    treasure_n03_run(ctx, TreasureN03Step::OnInit, Vec::new()).map(|_| ())
}

pub fn treasure_n03_ondied(ctx: &Ctx) -> Script {
    treasure_n03_run(ctx, TreasureN03Step::OnDied, Vec::new()).map(|_| ())
}

pub fn switch_tresn03(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_treas::f_gldtreassw(ctx, args!["nguild_pay", 120, 59])?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum TreasureN04Step {
    Start,
    OnRecvCastleN04,
    OnInit,
    OnDied,
}

fn treasure_n04_run(ctx: &Ctx, mut step: TreasureN04Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            TreasureN04Step::Start => {
                step = TreasureN04Step::OnRecvCastleN04;
                continue 'machine;
            }
            TreasureN04Step::OnRecvCastleN04 => {
                return Err(Stop::End);
            }
            TreasureN04Step::OnInit => {
                if ctx.var("$boxnumn04").get()? == 0 {
                    return Err(Stop::End);
                }
                ctx.var("$@bxn04").set(ctx.var("$boxnumn04").get()?)?;
                shared::events_nguild_nguild_treas::f_gldtreas(
                    ctx,
                    args![
                        "nguild_prt",
                        "N04",
                        ctx.var("$boxnumn04").get()?,
                        ctx.var("$@bxn04").get()?,
                        ctx.var("$@boxidn04").get()?,
                        Val::from(1354) + ctx.call(Function::Rand, args![10])?,
                        6,
                        204,
                        15,
                        213,
                        1
                    ],
                )?;
                return Err(Stop::End);
            }
            TreasureN04Step::OnDied => {
                ctx.call(Function::MapAnnounce, args!["nguild_prt", "Treasure Chest Broken Open", 17])?;
                ctx.var("$boxnumn04").set(ctx.var("$boxnumn04").get()?.number()? - 1)?;
                if ctx.var("$boxnumn04").get()? == 0 {
                    ctx.call(
                        Function::MapAnnounce,
                        args![
                            "nguild_prt",
                            "All of the treasure boxes have been opened.  You must wait untill the next day for them to appear again.",
                            0
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn treasure_n04(ctx: &Ctx) -> Script {
    treasure_n04_run(ctx, TreasureN04Step::Start, Vec::new()).map(|_| ())
}

pub fn treasure_n04_onrecvcastlen04(ctx: &Ctx) -> Script {
    treasure_n04_run(ctx, TreasureN04Step::OnRecvCastleN04, Vec::new()).map(|_| ())
}

pub fn treasure_n04_oninit(ctx: &Ctx) -> Script {
    treasure_n04_run(ctx, TreasureN04Step::OnInit, Vec::new()).map(|_| ())
}

pub fn treasure_n04_ondied(ctx: &Ctx) -> Script {
    treasure_n04_run(ctx, TreasureN04Step::OnDied, Vec::new()).map(|_| ())
}

pub fn switch_tresn04(ctx: &Ctx) -> Script {
    shared::events_nguild_nguild_treas::f_gldtreassw(ctx, args!["nguild_prt", 109, 179])?;
    ctx.end()
}
