use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn memory_seal_tt5_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Memory Seal#tt5")])?;
    return Err(Stop::End);
}

pub fn memory_seal_tt5_oninit(ctx: &Ctx) -> Script {
    memory_seal_tt5_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt5_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::EnableNpc, vec![Val::from("Memory Seal#tt5")])?;
    return Err(Stop::End);
}

pub fn memory_seal_tt5_onenable(ctx: &Ctx) -> Script {
    memory_seal_tt5_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn memory_seal_tt5_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::DisableNpc, vec![Val::from("Memory Seal#tt5")])?;
    ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_ICECRASH")?])?;
    return Err(Stop::End);
}

pub fn memory_seal_tt5_ondisable(ctx: &Ctx) -> Script {
    memory_seal_tt5_ondisable_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum SommonThanatosStep {
    Start,
    OnEnable,
    OnMyMobDead,
}

fn sommon_thanatos_run(ctx: &Ctx, mut step: SommonThanatosStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SommonThanatosStep::Start => {
                step = SommonThanatosStep::OnEnable;
                continue 'machine;
            }
            SommonThanatosStep::OnEnable => {
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![ctx.constant("EF_MAPPILLAR2")?, ctx.constant("AREA")?, Val::from("#sommon_thanatos")],
                )?;
                ctx.call(
                    Function::NpcSpecialEffect,
                    vec![
                        ctx.constant("EF_SEISMICWEAPON")?,
                        ctx.constant("AREA")?,
                        Val::from("#sommon_thanatos"),
                    ],
                )?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("thana_boss"),
                        Val::from(141),
                        Val::from(218),
                        Val::from("Thanatos Phantom"),
                        Val::from(1708),
                        Val::from(1),
                        Val::from("#sommon_thanatos::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            SommonThanatosStep::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("thana_boss"), Val::from("#sommon_thanatos::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("thana_boss"),
                            Val::from("RAWWWWWWWWWWR........ This can't be........................."),
                            ctx.constant("BC_MAP")?,
                            Val::from("0xff0000"),
                        ],
                    )?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("#cooltime_thana::OnEnable")])?;
                    ctx.var("$@thana_summon").set(Val::from(6))?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn sommon_thanatos(ctx: &Ctx) -> Script {
    sommon_thanatos_run(ctx, SommonThanatosStep::Start, Vec::new()).map(|_| ())
}

pub fn sommon_thanatos_onenable(ctx: &Ctx) -> Script {
    sommon_thanatos_run(ctx, SommonThanatosStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn sommon_thanatos_onmymobdead(ctx: &Ctx) -> Script {
    sommon_thanatos_run(ctx, SommonThanatosStep::OnMyMobDead, Vec::new()).map(|_| ())
}

pub fn cooltime_thana(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::Start, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_oninit(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnInit, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_onenable(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_onstop(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnStop, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer3000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer3000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer6000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer6000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer16000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer16000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer26000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer26000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer31000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer31000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer32000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer32000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer33000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer33000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer34000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer34000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer35000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer35000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer36000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer36000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer37000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer37000, Vec::new()).map(|_| ())
}

pub fn cooltime_thana_ontimer7200000(ctx: &Ctx) -> Script {
    cooltime_thana_run(ctx, CooltimeThanaStep::OnTimer7200000, Vec::new()).map(|_| ())
}

fn to_7th_floor_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn to_7th_floor(ctx: &Ctx) -> Script {
    to_7th_floor_body(ctx, Vec::new()).map(|_| ())
}

fn to_7th_floor_ontouch_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((runtime::op(
        &ctx.call(Function::EaClass, vec![])?,
        "&",
        &runtime::op(&ctx.constant("EAJL_2")?, "|", &ctx.constant("EAJL_UPPER")?)?,
    )?
    .is_true()
        || runtime::op(&ctx.call(Function::EaClass, vec![])?, "&", &ctx.constant("EAJL_THIRD")?)?.is_true())
        || (((((((ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?)
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_TAEKWON")?))
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_STAR_GLADIATOR")?))
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SOUL_LINKER")?))
            || ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_NINJA")?))
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_GUNSLINGER")?))
            || ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_SUMMONER")?))
            && ctx.var("BaseLevel").get()?.number()? > 94))
    {
        if ctx.call(Function::CountItem, vec![Val::from(7425)])?.number()? > 0 {
            ctx.lines(args![
                "The shadow of a Black Key is gleaming in the center of the portal.",
                "To pass this way, looks like I need something."
            ])?;
            ctx.next()?;
            ctx.mes("The Black Key reacts to the portal and floats in the air.")?;
            ctx.next()?;
            ctx.lines(args![
                "The key seems to separate into many particles and suddenly collapses into a ball.",
                "The barricade is removed through the power from newly formed black magic gem."
            ])?;
            ctx.close_window()?;
            ctx.call(Function::DelItem, vec![Val::from(7425), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(7430), Val::from(1)])?;
            ctx.call(Function::Warp, vec![Val::from("thana_step"), Val::from(69), Val::from(369)])?;
            return Err(Stop::End);
        }
        ctx.lines(args![
            "The shadow of a Black Key is gleaming in the center of the portal.",
            "To pass this way, looks like I need something."
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("A mysterious power is blocking my path.")?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn to_7th_floor_ontouch(ctx: &Ctx) -> Script {
    to_7th_floor_ontouch_body(ctx, Vec::new()).map(|_| ())
}
