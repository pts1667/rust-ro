use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MoksMushroomsMush2Step {
    Start,
    OnTimer20000,
}

fn moks_mushrooms_mush2_run(ctx: &Ctx, mut step: MoksMushroomsMush2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksMushroomsMush2Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 9 && ctx.var("hg_tre").get()?.number()? < 15) {
                    ctx.mes("- You found mushrooms that are as big as your palm. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You decided to gather the mushrooms. -")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- You were being clumsy and broke the mushrooms. You have failed in gathering the mushrooms. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered mushrooms. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 11 {
                                ctx.mes("Collected Moks Mushroom Solution: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 12 {
                                ctx.mes("Collected Moks Mushroom Solution: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 13 {
                                ctx.mes("Collected Moks Mushroom Solution: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 14 {
                                ctx.mes("Collected Moks Mushroom Solution: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 15 {
                                ctx.mes("Collected Moks Mushroom Solution: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Mushrooms#Mush2")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 15 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksMushroomsMush2Step::OnTimer20000;
                continue 'machine;
            }
            MoksMushroomsMush2Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Mushrooms#Mush2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_mushrooms_mush2(ctx: &Ctx) -> Script {
    moks_mushrooms_mush2_run(ctx, MoksMushroomsMush2Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_mushrooms_mush2_ontimer20000(ctx: &Ctx) -> Script {
    moks_mushrooms_mush2_run(ctx, MoksMushroomsMush2Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoksMushroomsMush3Step {
    Start,
    OnTimer20000,
}

fn moks_mushrooms_mush3_run(ctx: &Ctx, mut step: MoksMushroomsMush3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksMushroomsMush3Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 9 && ctx.var("hg_tre").get()?.number()? < 15) {
                    ctx.mes("- You found mushrooms that are as big as your palm. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You decided to gather the mushrooms. -")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- You were being clumsy and broke the mushrooms. You have failed in gathering the mushrooms. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered mushrooms. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 11 {
                                ctx.mes("Collected Moks Mushroom Solution: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 12 {
                                ctx.mes("Collected Moks Mushroom Solution: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 13 {
                                ctx.mes("Collected Moks Mushroom Solution: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 14 {
                                ctx.mes("Collected Moks Mushroom Solution: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 15 {
                                ctx.mes("Collected Moks Mushroom Solution: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Mushrooms#Mush3")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 15 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksMushroomsMush3Step::OnTimer20000;
                continue 'machine;
            }
            MoksMushroomsMush3Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Mushrooms#Mush3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_mushrooms_mush3(ctx: &Ctx) -> Script {
    moks_mushrooms_mush3_run(ctx, MoksMushroomsMush3Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_mushrooms_mush3_ontimer20000(ctx: &Ctx) -> Script {
    moks_mushrooms_mush3_run(ctx, MoksMushroomsMush3Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ClamC1Step {
    Start,
    OnTimer20000,
}

fn clam_c1_run(ctx: &Ctx, mut step: ClamC1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ClamC1Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 19 && ctx.var("hg_tre").get()?.number()? < 25) {
                    ctx.mes("- You found very fresh clams whose shells are shining under the sunlight. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You started opening clam shells to gather Clam Flesh.-")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- Clams are too strong to open. You have failed in gathering Clam Flesh. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Clam Flesh. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 21 {
                                ctx.mes("Collected Clam Flesh: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 22 {
                                ctx.mes("Collected Clam Flesh: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 23 {
                                ctx.mes("Collected Clam Flesh: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 24 {
                                ctx.mes("Collected Clam Flesh: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 25 {
                                ctx.mes("Collected Clam Flesh: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Clam#C1")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 25 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ClamC1Step::OnTimer20000;
                continue 'machine;
            }
            ClamC1Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Clam#C1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn clam_c1(ctx: &Ctx) -> Script {
    clam_c1_run(ctx, ClamC1Step::Start, Vec::new()).map(|_| ())
}

pub fn clam_c1_ontimer20000(ctx: &Ctx) -> Script {
    clam_c1_run(ctx, ClamC1Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ClamC2Step {
    Start,
    OnTimer20000,
}

fn clam_c2_run(ctx: &Ctx, mut step: ClamC2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ClamC2Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 19 && ctx.var("hg_tre").get()?.number()? < 25) {
                    ctx.mes("- You found very fresh clams whose shells are shining under the sunlight. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You started opening clam shells to gather Clam Flesh.-")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- Clams are too strong to open. You have failed in gathering Clam Flesh. -")?;
                                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Clam Flesh. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 21 {
                                ctx.mes("Collected Clam Flesh: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 22 {
                                ctx.mes("Collected Clam Flesh: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 23 {
                                ctx.mes("Collected Clam Flesh: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 24 {
                                ctx.mes("Collected Clam Flesh: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 25 {
                                ctx.mes("Collected Clam Flesh: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Clam#C2")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 25 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ClamC2Step::OnTimer20000;
                continue 'machine;
            }
            ClamC2Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Clam#C2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn clam_c2(ctx: &Ctx) -> Script {
    clam_c2_run(ctx, ClamC2Step::Start, Vec::new()).map(|_| ())
}

pub fn clam_c2_ontimer20000(ctx: &Ctx) -> Script {
    clam_c2_run(ctx, ClamC2Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ClamC3Step {
    Start,
    OnTimer20000,
}

fn clam_c3_run(ctx: &Ctx, mut step: ClamC3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ClamC3Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 19 && ctx.var("hg_tre").get()?.number()? < 25) {
                    ctx.mes("- You found very fresh clams whose shells are shining under the sunlight. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You started opening clam shells to gather Clam Flesh.-")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- Clams are too strong to open. You have failed in gathering Clam Flesh. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Clam Flesh. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 21 {
                                ctx.mes("Collected Clam Flesh: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 22 {
                                ctx.mes("Collected Clam Flesh: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 23 {
                                ctx.mes("Collected Clam Flesh: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 24 {
                                ctx.mes("Collected Clam Flesh: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 25 {
                                ctx.mes("Collected Clam Flesh: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Clam#C3")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 25 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ClamC3Step::OnTimer20000;
                continue 'machine;
            }
            ClamC3Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Clam#C3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn clam_c3(ctx: &Ctx) -> Script {
    clam_c3_run(ctx, ClamC3Step::Start, Vec::new()).map(|_| ())
}

pub fn clam_c3_ontimer20000(ctx: &Ctx) -> Script {
    clam_c3_run(ctx, ClamC3Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ClamC4Step {
    Start,
    OnTimer20000,
}

fn clam_c4_run(ctx: &Ctx, mut step: ClamC4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ClamC4Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 19 && ctx.var("hg_tre").get()?.number()? < 25) {
                    ctx.mes("- You found very fresh clams whose shells are shining under the sunlight. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Gather them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You started opening clam shells to gather Clam Flesh.-")?;
                            ctx.next()?;
                            ctx.mes("- *Snip Snip* -")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Snip Snip* -", "- *Snip Snip* -", "- *Snip Snip* -"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- Clams are too strong to open. You have failed in gathering Clam Flesh. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Clam Flesh. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 21 {
                                ctx.mes("Collected Clam Flesh: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 22 {
                                ctx.mes("Collected Clam Flesh: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 23 {
                                ctx.mes("Collected Clam Flesh: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 24 {
                                ctx.mes("Collected Clam Flesh: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 25 {
                                ctx.mes("Collected Clam Flesh: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Clam#C4")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 25 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["I think that this will do."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = ClamC4Step::OnTimer20000;
                continue 'machine;
            }
            ClamC4Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Clam#C4")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn clam_c4(ctx: &Ctx) -> Script {
    clam_c4_run(ctx, ClamC4Step::Start, Vec::new()).map(|_| ())
}

pub fn clam_c4_ontimer20000(ctx: &Ctx) -> Script {
    clam_c4_run(ctx, ClamC4Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoksBugsBug1Step {
    Start,
    OnTimer20000,
}

fn moks_bugs_bug1_run(ctx: &Ctx, mut step: MoksBugsBug1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksBugsBug1Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 29 && ctx.var("hg_tre").get()?.number()? < 35) {
                    ctx.mes("- You found big brown Moks Bugs in the bushes. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Catch them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You nervously stretched your hands toward Moks Bugs.-")?;
                            ctx.next()?;
                            ctx.mes("- *Whizz Whizz*-")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-", "- *Whizz Whizz*-"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- The bugs quickly ran away. You have failed to gather their shells. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Moks Bugs Shells. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 31 {
                                ctx.mes("Collected Moks Bugs Shells: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 32 {
                                ctx.mes("Collected Moks Bugs Shells: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 33 {
                                ctx.mes("Collected Moks Bugs Shells: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 34 {
                                ctx.mes("Collected Moks Bugs Shells: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 35 {
                                ctx.mes("Collected Moks Bugs Shells: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Bugs#Bug1")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 35 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Aww...I don't want to touch dirty things like bugs!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksBugsBug1Step::OnTimer20000;
                continue 'machine;
            }
            MoksBugsBug1Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Bugs#Bug1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_bugs_bug1(ctx: &Ctx) -> Script {
    moks_bugs_bug1_run(ctx, MoksBugsBug1Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_bugs_bug1_ontimer20000(ctx: &Ctx) -> Script {
    moks_bugs_bug1_run(ctx, MoksBugsBug1Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoksBugsBug2Step {
    Start,
    OnTimer20000,
}

fn moks_bugs_bug2_run(ctx: &Ctx, mut step: MoksBugsBug2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksBugsBug2Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 29 && ctx.var("hg_tre").get()?.number()? < 35) {
                    ctx.mes("- You found big brown Moks Bugs in the bushes. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Catch them:Pass.")])? {
                        1 => {
                            ctx.mes("- You nervously stretched your hands toward Moks Bugs.-")?;
                            ctx.next()?;
                            ctx.mes("- *Whizz Whizz*-")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-", "- *Whizz Whizz*-"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- The bugs quickly ran away. You have failed to gather their shells. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Moks Bugs Shells. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 31 {
                                ctx.mes("Collected Moks Bugs Shells: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 32 {
                                ctx.mes("Collected Moks Bugs Shells: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 33 {
                                ctx.mes("Collected Moks Bugs Shells: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 34 {
                                ctx.mes("Collected Moks Bugs Shells: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 35 {
                                ctx.mes("Collected Moks Bugs Shells: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Bugs#Bug2")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 35 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Aww...I don't want to touch dirty things like bugs!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksBugsBug2Step::OnTimer20000;
                continue 'machine;
            }
            MoksBugsBug2Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Bugs#Bug2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_bugs_bug2(ctx: &Ctx) -> Script {
    moks_bugs_bug2_run(ctx, MoksBugsBug2Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_bugs_bug2_ontimer20000(ctx: &Ctx) -> Script {
    moks_bugs_bug2_run(ctx, MoksBugsBug2Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoksBugsBug3Step {
    Start,
    OnTimer20000,
}

fn moks_bugs_bug3_run(ctx: &Ctx, mut step: MoksBugsBug3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksBugsBug3Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 29 && ctx.var("hg_tre").get()?.number()? < 35) {
                    ctx.mes("- You found big brown Moks Bugs in the bushes. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Catch them:Pass.")])? {
                        1 => {
                            ctx.mes("- You nervously stretched your hands toward Moks Bugs.-")?;
                            ctx.next()?;
                            ctx.mes("- *Whizz Whizz*-")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-", "- *Whizz Whizz*-"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- The bugs quickly ran away. You have failed to gather their shells. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Moks Bugs Shells. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 31 {
                                ctx.mes("Collected Moks Bugs Shells: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 32 {
                                ctx.mes("Collected Moks Bugs Shells: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 33 {
                                ctx.mes("Collected Moks Bugs Shells: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 34 {
                                ctx.mes("Collected Moks Bugs Shells: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 35 {
                                ctx.mes("Collected Moks Bugs Shells: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Bugs#Bug3")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 35 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Aww...I don't want to touch dirty things like bugs!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksBugsBug3Step::OnTimer20000;
                continue 'machine;
            }
            MoksBugsBug3Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Bugs#Bug3")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_bugs_bug3(ctx: &Ctx) -> Script {
    moks_bugs_bug3_run(ctx, MoksBugsBug3Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_bugs_bug3_ontimer20000(ctx: &Ctx) -> Script {
    moks_bugs_bug3_run(ctx, MoksBugsBug3Step::OnTimer20000, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MoksBugsBug4Step {
    Start,
    OnTimer20000,
}

fn moks_bugs_bug4_run(ctx: &Ctx, mut step: MoksBugsBug4Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MoksBugsBug4Step::Start => {
                if (ctx.var("hg_tre").get()?.number()? > 29 && ctx.var("hg_tre").get()?.number()? < 35) {
                    ctx.mes("- You found big brown Moks Bugs in the bushes. -")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Catch them.:Pass.")])? {
                        1 => {
                            ctx.mes("- You nervously stretched your hands toward Moks Bugs.-")?;
                            ctx.next()?;
                            ctx.mes("- *Whizz Whizz*-")?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-"])?;
                            ctx.next()?;
                            ctx.lines(args!["- *Whizz Whizz Bzzzz* -", "- *Whizz Whizz*-", "- *Whizz Whizz*-"])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_CONE")?])?;
                            ctx.next()?;
                            if ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?.number()? > 4 {
                                ctx.mes("- The bugs quickly ran away. You have failed to gather their shells. -")?;
                                ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_STUNATTACK")?])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.mes("- You have successfully gathered Moks Bugs Shells. -")?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_VALLENTINE2")?])?;
                            ctx.var("hg_tre").set((ctx.var("hg_tre").get()? + Val::from(1)))?;
                            if ctx.var("hg_tre").get()? == 31 {
                                ctx.mes("Collected Moks Bugs Shells: 1ea ")?;
                            } else if ctx.var("hg_tre").get()? == 32 {
                                ctx.mes("Collected Moks Bugs Shells: 2ea ")?;
                            } else if ctx.var("hg_tre").get()? == 33 {
                                ctx.mes("Collected Moks Bugs Shells: 3ea ")?;
                            } else if ctx.var("hg_tre").get()? == 34 {
                                ctx.mes("Collected Moks Bugs Shells: 4ea ")?;
                            } else if ctx.var("hg_tre").get()? == 35 {
                                ctx.mes("Collected Moks Bugs Shells: 5ea ")?;
                            }
                            ctx.call(Function::InitNpcTimer, vec![])?;
                            ctx.call(Function::DisableNpc, vec![Val::from("Moks Bugs#Bug4")])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("- You decided to pass by them. -")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else if ctx.var("hg_tre").get()? == 35 {
                    ctx.lines_as(
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        args!["Aww...I don't want to touch dirty things like bugs!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = MoksBugsBug4Step::OnTimer20000;
                continue 'machine;
            }
            MoksBugsBug4Step::OnTimer20000 => {
                ctx.call(Function::EnableNpc, vec![Val::from("Moks Bugs#Bug4")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn moks_bugs_bug4(ctx: &Ctx) -> Script {
    moks_bugs_bug4_run(ctx, MoksBugsBug4Step::Start, Vec::new()).map(|_| ())
}

pub fn moks_bugs_bug4_ontimer20000(ctx: &Ctx) -> Script {
    moks_bugs_bug4_run(ctx, MoksBugsBug4Step::OnTimer20000, Vec::new()).map(|_| ())
}

fn a_pile_of_paper_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_tre").get()? == 52 {
        ctx.lines(args![
            "- You found a pile of paper that was tied up with a thick ribbon.",
            "On the very top paper, a period and a title saying 'Report of the **th Research'",
            "were written."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Good, I must take this with me."],
        )?;
        ctx.var("hg_tre").set(Val::from(53))?;
        ctx.call(Function::GetItem, vec![Val::from(7342), Val::from(1)])?;
        ctx.call(Function::DoNpcEvent, vec![Val::from("HuMSpawner::OnMonster")])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn a_pile_of_paper(ctx: &Ctx) -> Script {
    a_pile_of_paper_body(ctx, Vec::new()).map(|_| ())
}

pub fn humspawner(ctx: &Ctx) -> Script {
    humspawner_run(ctx, HumspawnerStep::Start, Vec::new()).map(|_| ())
}

pub fn humspawner_onmonster(ctx: &Ctx) -> Script {
    humspawner_run(ctx, HumspawnerStep::OnMonster, Vec::new()).map(|_| ())
}

pub fn humspawner_ontimer60000(ctx: &Ctx) -> Script {
    humspawner_run(ctx, HumspawnerStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn humspawner_onmonsterdead(ctx: &Ctx) -> Script {
    humspawner_run(ctx, HumspawnerStep::OnMonsterDead, Vec::new()).map(|_| ())
}

fn unethical_machine_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_tre").get()? == 54 {
        if ctx.call(Function::CountItem, vec![Val::from(7138)])?.is_true() {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args![
                    "This must be it...whoa, it looks very spooky.",
                    "Today, I am going to destroy you",
                    "for making innocent people's lives miserable!"
                ],
            )?;
            ctx.next()?;
            ctx.mes("- You threw a Marine Sphere Bottle toward the machine. -")?;
            ctx.next()?;
            ctx.mes("BOOM!")?;
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_SUI_EXPLOSION")?])?;
            ctx.call(Function::EnableNpc, vec![Val::from("HiddenExplosion")])?;
            ctx.call(
                Function::MapAnnounce,
                vec![
                    Val::from("lhz_dun02"),
                    Val::from("Beep------------------ "),
                    Val::from(1),
                    Val::from(10079487),
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(7138), Val::from(1)])?;
            ctx.var("hg_tre").set(Val::from(55))?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "- You were staggered for the fact that",
                "you forgot to bring the Marine Sphere Bottle with you. -"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(Val::from(0))
}

pub fn unethical_machine(ctx: &Ctx) -> Script {
    unethical_machine_body(ctx, Vec::new()).map(|_| ())
}

fn hiddenexplosion_run(ctx: &Ctx, mut step: HiddenexplosionStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HiddenexplosionStep::Start => {
                step = HiddenexplosionStep::OnInit;
                continue 'machine;
            }
            HiddenexplosionStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("HiddenExplosion")])?;
                return Err(Stop::End);
            }
            HiddenexplosionStep::OnTouch => {
                ctx.call(Function::PercentHeal, vec![Val::from(-10), Val::from(0)])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_HIT5")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("HiddenExplosion")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn hiddenexplosion(ctx: &Ctx) -> Script {
    hiddenexplosion_run(ctx, HiddenexplosionStep::Start, Vec::new()).map(|_| ())
}

pub fn hiddenexplosion_oninit(ctx: &Ctx) -> Script {
    hiddenexplosion_run(ctx, HiddenexplosionStep::OnInit, Vec::new()).map(|_| ())
}

pub fn hiddenexplosion_ontouch(ctx: &Ctx) -> Script {
    hiddenexplosion_run(ctx, HiddenexplosionStep::OnTouch, Vec::new()).map(|_| ())
}

fn girl_hu_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Girl",
        args![
            "I love sitting in front of a stove because it makes me warm~",
            "Ah...it is so warm and relaxing...and now I feel sleepy...Zzzzz Zzzz..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn girl_hu(ctx: &Ctx) -> Script {
    girl_hu_body(ctx, Vec::new()).map(|_| ())
}

fn pub_granny_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Pub Granny",
        args![
            "Recently we have many tourists visiting our town.",
            "They make me pretty busy, but I am so happy to see them enjoying my food. Hohoho!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pub_granny(ctx: &Ctx) -> Script {
    pub_granny_body(ctx, Vec::new()).map(|_| ())
}

fn hugeltree_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("BaseLevel").get()?.number()? > 49 {
        if !(ctx.var("hg_memory").get()?.is_true()) {
            ctx.var("hg_memory").set(Val::from(1))?;
            ctx.call(Function::SetQuest, vec![Val::from(8057)])?;
            ctx.lines(args![
                "^3355FFFor some reason, you",
                "feel very warm, safe,",
                "and secure near this",
                "tree. Just being near",
                "it brings you a sense",
                "of overwhelming comfort.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_memory").get()? == 1 {
            ctx.lines(args![
                "^3355FFFor some reason, you",
                "feel very warm, safe,",
                "and secure near this",
                "tree. Just being near",
                "it brings you a sense",
                "of overwhelming comfort.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("hg_memory").get()? == 6 {
            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_BEGINSPELL5")?])?;
            ctx.lines(args![
                "^3355FFWhat's this strange",
                "feeling of dread?",
                "This peculiar chill...",
                "It's almost as if you",
                "were in Niflheim...^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines(args![
                "^3355FFFor some reason, you",
                "feel very warm, safe,",
                "and secure near this",
                "tree. Just being near",
                "it brings you a sense",
                "of overwhelming comfort.^000000"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines(args![
            "^3355FFFor some reason, you",
            "feel very warm, safe,",
            "and secure near this",
            "tree. Just being near",
            "it brings you a sense",
            "of overwhelming comfort.^000000"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn hugeltree(ctx: &Ctx) -> Script {
    hugeltree_body(ctx, Vec::new()).map(|_| ())
}

fn manainne_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_memory").get()? == 1 {
        ctx.lines_as(
            "Manainne",
            args![
                "This tree means so much",
                "to everyone in this town,",
                "and we all have special",
                "memories of this place. But",
                "that El Schatt fellow plans to",
                "chop it down for his shop..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Manainne",
            args![
                "You see him? That's",
                "El Schatt right there.",
                "I've never met anyone",
                "so cruel and selfish!",
                "Right now he's surveying",
                "the land for construction..."
            ],
        )?;
        ctx.var("hg_memory").set(Val::from(2))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8057), Val::from(8058)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if ctx.var("hg_memory").get()? == 2 {
            ctx.lines_as(
                "Manainne",
                args![
                    "This tree means so much",
                    "to everyone in this town,",
                    "and we all have special",
                    "memories of this place. But",
                    "that El Schatt fellow plans to",
                    "chop it down for his shop..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Manainne",
                args![
                    "You see him? That's",
                    "El Schatt right there.",
                    "I've never met anyone",
                    "so cruel and selfish!",
                    "Right now he's surveying",
                    "the land for construction..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("hg_memory").get()? == 5 {
                ctx.lines_as(
                    "Manainne",
                    args![
                        "You know, El Schatt and",
                        "I used to get along when",
                        "he dated my older sister,",
                        "Kanainne. In fact, they'd",
                        "always spend time together right under that tree. Weird, huh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Manainne",
                    args![
                        "Then, well, Kanainne",
                        "got really sick, and we...",
                        "We couldn't help her. She",
                        "passed away, and then El Schatt",
                        "just disappeared. It was only",
                        "recently that he's come back."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Manainne",
                    args![
                        "Now El Schatt is just a",
                        "heartless, money grubbing...!",
                        "He's completely changed.",
                        "My sister would be rolling",
                        "in her grave, seeing him now..."
                    ],
                )?;
                ctx.var("hg_memory").set(Val::from(6))?;
                ctx.call(Function::ChangeQuest, vec![Val::from(8061), Val::from(8062)])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("hg_memory").get()? == 6 {
                    ctx.lines_as(
                        "Manainne",
                        args![
                            "Now El Schatt is just a",
                            "heartless, money grubbing...!",
                            "He's completely changed.",
                            "My sister would be rolling",
                            "in her grave, seeing him now..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("hg_memory").get()? == 7 {
                        ctx.lines_as(
                            "Manainne",
                            args![
                                "What? That's...",
                                "That's an incredible",
                                "story. You met my dead",
                                "sister in Niflheim?! You",
                                "must be lying to me!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Manainne",
                            args![
                                "Let's... Let's just",
                                "stop talking about this,",
                                "okay? I appreciate that you",
                                "want to stop El Schatt from",
                                "cutting down our tree, but",
                                "this is almost too much."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.var("hg_memory").set(Val::from(8))?;
                        ctx.lines_as(
                            "Manainne",
                            args![
                                "Gosh, all that talk",
                                "about Niflheim is...",
                                "It's giving me the chills!",
                                "A-aren't you cold?"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("hg_memory").get()? == 8 {
                            ctx.lines_as("Manainne", args!["......", ".........", "Huh? You're...", "Still here...?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "Y-you know what's",
                                    "weird? The weather's",
                                    "perfectly fine, but I...",
                                    "I feel so cold all of a",
                                    "sudden. You don't feel",
                                    "it at all, don't you?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "I think I'm coming down",
                                    "with something. My head",
                                    "feels so numb, and I c-can't",
                                    "focus any of my thoughts.",
                                    "This is freakin' me out..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Manainne", args!["...!"])?;
                            ctx.next()?;
                            ctx.var("hg_memory").set(Val::from(9))?;
                            ctx.lines_as("Manainne", args!["......", ".........."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_memory").get()? == 9 {
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "...I'm here. I'm really",
                                    "back! Oh, it's been so",
                                    "long! Hello, adventurer,",
                                    "remember me? I'm Kanainne,",
                                    "the spirit you saw down in",
                                    "Niflheim. Let me explain..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "My spirit was able to follow",
                                    "you here, and now that I'm",
                                    "here, I only have so much time",
                                    "to possess my sister's body,",
                                    "and then talk to El Schatt.",
                                    "El Schatt? El Schatt...?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "El Schatt",
                                args!["Manainne...?", "What do you want?", "Quit bugging me,", "I'm busy right now..."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "It's... It's me,",
                                    "El Schatt, it's Kanainne.",
                                    "Of course you remember",
                                    "me, right? Listen, I need to--"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "El Schatt",
                                args![
                                    "Manainne...?!",
                                    "I'm shocked...",
                                    "Don't you respect",
                                    "your own sister?!",
                                    "I can't believe you'd...",
                                    "Knock it off right now!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.var("hg_memory").set(Val::from(10))?;
                            ctx.lines_as(
                                "El Schatt",
                                args![
                                    "Look, out of my respect",
                                    "for you and the relationship",
                                    "that me and your sister had,",
                                    "I'm going to forget you said",
                                    "that. But don't you dare use",
                                    "her name that way again!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Manainne", args!["......", ".........", "............"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_memory").get()? == 10 {
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "Wait, El Schatt!",
                                    "It's really me, but",
                                    "I can only speak to you",
                                    "through my sister for",
                                    "a short time. L-listen..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "El Schatt",
                                args![
                                    "Manainne...",
                                    "You don't make jokes",
                                    "like that about the departed,",
                                    "especially if it's Kannaine.",
                                    "You know better than that!",
                                    "Where the hell's your respect?!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "''If you're the wind,",
                                    "I'll be the sky, so that you",
                                    "can fly freely in my arms...''"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("El Schatt", args!["Huh...?!"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "''My soul is just",
                                    "for you, the shining",
                                    "sun that brightens",
                                    "up my whole world.''"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("El Schatt", args!["How do you...", "How do you know", "those words...?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "''In my dream, your",
                                    "eyes held the reflection",
                                    "of the ocean's clear waters... Someday I'll show them to you.''"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "''Your lovely face,",
                                    "your enticing smile...",
                                    "They're forever be in my",
                                    "heart, so I'll never be lonely.''"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("El Schatt", args!["Manainne...", "How do you", "know that song?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "El Schatt...",
                                    "It's me, Kanainne.",
                                    "You wrote this song",
                                    "just for me. But I died",
                                    "before you could finish it."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "That was your dream,",
                                    "wasn't it? To play your",
                                    "beautiful music to make",
                                    "people happy? Did you ever",
                                    "finish our song for me?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "El Schatt",
                                args![
                                    "Hahahah, Manainne, alright.",
                                    "Your sister must have told",
                                    "you about that song. Ha ha...",
                                    "You're t-taking it too far...",
                                    "Please, just... Just stop."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Manainne",
                                args!["El Schatt...", "Listen to me.", "Look at me. It's", "really your Kanainne."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "El Schatt",
                                args![
                                    "Kanainne...?",
                                    "It's too much to",
                                    "believe. I want to",
                                    "believe it so badly...",
                                    "Is it really you?"
                                ],
                            )?;
                            ctx.var("hg_memory").set(Val::from(11))?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_memory").get()? == 11 {
                            ctx.lines_as("Manainne", args!["El Schatt..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("hg_memory").get()? == 12 {
                            ctx.lines(args![
                                "^3355FFManainne is sobbing",
                                "uncontrollably, and there's",
                                "nothing you can do to comfort",
                                "her. Nothing at all. You must",
                                "leave her alone to her despair",
                                "and unfathomable sense of loss.^000000"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Manainne",
                                args![
                                    "You know, even though",
                                    "it's not a wealthy area,",
                                    "I really love this small",
                                    "and quiet town. Even if",
                                    "I could live anywhere in",
                                    "the world, I'd live right here~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
            }
        }
    }
}

pub fn manainne(ctx: &Ctx) -> Script {
    manainne_body(ctx, Vec::new()).map(|_| ())
}

fn a_spirit_hquest_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_memory").get()? == 6 {
        ctx.lines_as("Spirit", args!["El Schatt...", "Manainne...", "I miss them..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Spirit",
            args![
                "Those names... Yes,",
                "I remember now. Manainee",
                "was... my younger sister.",
                "El Schatt was... my lover.",
                "M-my name was... Kanainne!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kanainne's Spirit",
            args![
                "Now I know why I'm here",
                "in this realm. El Schatt...",
                "I've been watching him.",
                "He isn't able to let me go...",
                "But I can't directly intervene",
                "with the affairs of the living."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kanainne's Spirit",
            args![
                "I... I can borrow my",
                "sister's body? Yes, if she",
                "cooperates, maybe... Maybe",
                "it will work. I must... must do",
                "something before he does",
                "he hurts himself even more..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kanainne's Spirit",
            args![
                "You... You can help",
                "me... Just go back to ",
                "the living... to Hugel...",
                "Let me send you there..."
            ],
        )?;
        ctx.var("hg_memory").set(Val::from(7))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8062), Val::from(8063)])?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("hugel"), Val::from(78), Val::from(148)])?;
    } else if ctx.var("hg_memory").get()? == 7 {
        ctx.lines_as(
            "Kanainne's Spirit",
            args![
                "You...! You can help",
                "me... Just go back to ",
                "the living... to Hugel...",
                "Let me send you there..."
            ],
        )?;
        ctx.close_window()?;
        ctx.call(Function::Warp, vec![Val::from("hugel"), Val::from(78), Val::from(148)])?;
    } else {
        ctx.lines_as(
            "Spirit",
            args![
                "The living shouldn't",
                "be here in Niflheim...",
                "Go back! Go back to your",
                "own world before it's too late!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Spirit", args!["El Schatt...", "Manainne...", "I miss them..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn a_spirit_hquest(ctx: &Ctx) -> Script {
    a_spirit_hquest_body(ctx, Vec::new()).map(|_| ())
}

fn perfitz_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("hg_memory").get()? == 3 {
        ctx.var("hg_memory").set(Val::from(4))?;
        ctx.lines_as(
            "Perfitz",
            args![
                "I'm pleased that my son has",
                "finally set his mind on the",
                "family business. I didn't know",
                "he had it in him to become",
                "such a proactive entrepreneur."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Perfitz",
            args![
                "It's funny... He used to",
                "deperately fight me when it",
                "came to building shops where",
                "that tree has planted. Yes, I'm",
                "proud that he's grown as a",
                "business person. Heh heh~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Perfitz",
            args![
                "Although I'm pleased,",
                "I get this gut feeling that",
                "something's not right, like",
                "he's too excited about cutting",
                "that tree down. Maybe this old",
                "man is just too suspcious~"
            ],
        )?;
        ctx.call(Function::ChangeQuest, vec![Val::from(8059), Val::from(8060)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("hg_memory").get()? == 4 {
        ctx.lines_as(
            "Perfitz",
            args![
                "It's funny how things",
                "change. You know, El Schatt",
                "actually used to protest against my idea of building shops in the",
                "area when that tree grows. Hmm... I wonder, what happened to him?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Perfitz",
            args![
                "Money is very dependable...",
                "But it's value is relative, and",
                "can change at any moment.",
                "Still, at my age, you can't",
                "live without it. Make sure you",
                "financially prepare of old age."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Perfitz",
            args![
                "Now, most people assume that",
                "I have very few friends because",
                "I trust money more than people.",
                "That's somewhat true: money",
                "will never betray you, and few",
                "friends can be so dependable."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Perfitz",
            args![
                "However, friends that can",
                "be trusted for life are worth",
                "more than all the money in the",
                "world. To a rich man like me,",
                "those are the only friends",
                "that are worth having."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn perfitz(ctx: &Ctx) -> Script {
    perfitz_body(ctx, Vec::new()).map(|_| ())
}
