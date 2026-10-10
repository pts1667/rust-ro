use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum MobRogue15Step {
    Start,
    OnTouch,
    OnDisable,
    OnMyMobDead,
}

fn mob_rogue_15_run(ctx: &Ctx, mut step: MobRogue15Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue15Step::Start => {
                step = MobRogue15Step::OnTouch;
                continue 'machine;
            }
            MobRogue15Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(240),
                            Val::from(319),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#15::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(259),
                            Val::from(319),
                            Val::from("Archer Skeleton"),
                            Val::from(1016),
                            Val::from(1),
                            Val::from("mob_rogue#15::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(259),
                            Val::from(320),
                            Val::from("Ghoul"),
                            Val::from(1036),
                            Val::from(1),
                            Val::from("mob_rogue#15::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(240),
                            Val::from(319),
                            Val::from("Mummy"),
                            Val::from(1041),
                            Val::from(1),
                            Val::from("mob_rogue#15::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(259),
                            Val::from(319),
                            Val::from("Archer Skeleton"),
                            Val::from(1016),
                            Val::from(1),
                            Val::from("mob_rogue#15::OnMyMobDead"),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("in_rogue"),
                            Val::from(259),
                            Val::from(320),
                            Val::from("Ghoul"),
                            Val::from(1036),
                            Val::from(1),
                            Val::from("mob_rogue#15::OnMyMobDead"),
                        ],
                    )?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
            MobRogue15Step::OnDisable => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("in_rogue"), Val::from("mob_rogue#15::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            MobRogue15Step::OnMyMobDead => {
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_15(ctx: &Ctx) -> Script {
    mob_rogue_15_run(ctx, MobRogue15Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_15_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_15_run(ctx, MobRogue15Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn mob_rogue_15_ondisable(ctx: &Ctx) -> Script {
    mob_rogue_15_run(ctx, MobRogue15Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn mob_rogue_15_onmymobdead(ctx: &Ctx) -> Script {
    mob_rogue_15_run(ctx, MobRogue15Step::OnMyMobDead, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MobRogue16Step {
    Start,
    OnTouch,
}

fn mob_rogue_16_run(ctx: &Ctx, mut step: MobRogue16Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MobRogue16Step::Start => {
                step = MobRogue16Step::OnTouch;
                continue 'machine;
            }
            MobRogue16Step::OnTouch => {
                if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#15::OnDisable")])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("mag_dun02"), Val::from(181), Val::from(176)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn mob_rogue_16(ctx: &Ctx) -> Script {
    mob_rogue_16_run(ctx, MobRogue16Step::Start, Vec::new()).map(|_| ())
}

pub fn mob_rogue_16_ontouch(ctx: &Ctx) -> Script {
    mob_rogue_16_run(ctx, MobRogue16Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum ResetterRogueStep {
    Start,
    OnTimer500000,
    OnEnable,
    OnInit,
}

fn resetter_rogue_run(ctx: &Ctx, mut step: ResetterRogueStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ResetterRogueStep::Start => {
                step = ResetterRogueStep::OnTimer500000;
                continue 'machine;
            }
            ResetterRogueStep::OnTimer500000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#6::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#7::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#8::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#9::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#10::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#12::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#13::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#15::OnDisable")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ResetterRogueStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ResetterRogueStep::OnInit => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("resetter#rogue::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn resetter_rogue(ctx: &Ctx) -> Script {
    resetter_rogue_run(ctx, ResetterRogueStep::Start, Vec::new()).map(|_| ())
}

pub fn resetter_rogue_ontimer500000(ctx: &Ctx) -> Script {
    resetter_rogue_run(ctx, ResetterRogueStep::OnTimer500000, Vec::new()).map(|_| ())
}

pub fn resetter_rogue_onenable(ctx: &Ctx) -> Script {
    resetter_rogue_run(ctx, ResetterRogueStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn resetter_rogue_oninit(ctx: &Ctx) -> Script {
    resetter_rogue_run(ctx, ResetterRogueStep::OnInit, Vec::new()).map(|_| ())
}

fn switch_rogreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines(args![
        "^F08080Tah dah~",
        "Monsters for the",
        "Rogue Job Change",
        "have been reset^000000."
    ])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#1::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#2::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#3::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#4::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#5::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#6::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#7::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#8::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#9::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#10::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#12::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#13::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("mob_rogue#15::OnDisable")])?;
    ctx.call(Function::DoNpcEvent, vec![Val::from("resetter#rogue::OnEnable")])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn switch_rogreset(ctx: &Ctx) -> Script {
    switch_rogreset_body(ctx, Vec::new()).map(|_| ())
}
