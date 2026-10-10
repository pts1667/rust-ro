use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum SnorrenEp13md17Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
}

pub(super) fn snorren_ep13md17_run(ctx: &Ctx, mut step: SnorrenEp13md17Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SnorrenEp13md17Step::Start => {
                ctx.call(Function::DisableNpc, vec![Val::from("Snorren#ep13md17")])?;
                return Err(Stop::End);
            }
            SnorrenEp13md17Step::OnInit => {
                step = SnorrenEp13md17Step::OnDisable;
                continue 'machine;
            }
            SnorrenEp13md17Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Snorren#ep13md17")])?;
                return Err(Stop::End);
            }
            SnorrenEp13md17Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Snorren#ep13md17")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum EggKeeperDraco131Step {
    Start,
    OnEnable,
    OnDisable,
    OnTouch,
}

pub(super) fn egg_keeper_draco_13_1_run(ctx: &Ctx, mut step: EggKeeperDraco131Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            EggKeeperDraco131Step::Start => {
                step = EggKeeperDraco131Step::OnEnable;
                continue 'machine;
            }
            EggKeeperDraco131Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Egg Keeper Draco#13_1")])?;
                return Err(Stop::End);
            }
            EggKeeperDraco131Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Egg Keeper Draco#13_1")])?;
                return Err(Stop::End);
            }
            EggKeeperDraco131Step::OnTouch => {
                ctx.call(Function::DisableNpc, vec![Val::from("Egg Keeper Draco#13_1")])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("nyd_dun02"),
                        Val::from(201),
                        Val::from(157),
                        Val::from("Egg Keeper Draco"),
                        Val::from(2013),
                        Val::from(1),
                        Val::from("Egg Keeper Draco#13_3::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum StrangerEp132Dan02Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnCall,
    OnReset,
    OnTouch,
}

pub(super) fn stranger_ep13_2_dan02_run(ctx: &Ctx, mut step: StrangerEp132Dan02Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            StrangerEp132Dan02Step::Start => {
                return Err(Stop::End);
            }
            StrangerEp132Dan02Step::OnInit => {
                step = StrangerEp132Dan02Step::OnDisable;
                continue 'machine;
            }
            StrangerEp132Dan02Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("Stranger#ep13_2_dan02")])?;
                return Err(Stop::End);
            }
            StrangerEp132Dan02Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Stranger#ep13_2_dan02")])?;
                return Err(Stop::End);
            }
            StrangerEp132Dan02Step::OnCall => {
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("nyd_dun02"),
                        Val::from(206),
                        Val::from(114),
                        Val::from("Runway Dandelion"),
                        Val::from(2026),
                        Val::from(1),
                        Val::from("Stranger#ep13_2_dan04::OnMyMobDead"),
                    ],
                )?;
                return Err(Stop::End);
            }
            StrangerEp132Dan02Step::OnReset => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("nyd_dun02"), Val::from("Stranger#ep13_2_dan04::OnMyMobDead")],
                )?;
                return Err(Stop::End);
            }
            StrangerEp132Dan02Step::OnTouch => {
                ctx.call(
                    Function::Emotion,
                    vec![
                        ctx.constant("ET_SURPRISE")?,
                        Val::from(ctx.call(Function::GetCharacterId, vec![Val::from(0)])?.is_true()),
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Stranger#ep13_2_dan02")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Stranger#ep13_2_dan01::OnCall")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum HalfBuriedGem1Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer300000,
}

pub(super) fn half_buried_gem_1_run(ctx: &Ctx, mut step: HalfBuriedGem1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HalfBuriedGem1Step::Start => {
                shared::quests_quests_13_2::jewel_13_2(ctx, vec![Val::from("Red"), Val::from(1)])?;
                step = HalfBuriedGem1Step::OnInit;
                continue 'machine;
            }
            HalfBuriedGem1Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Half-buried Gem#1")])?;
                return Err(Stop::End);
            }
            HalfBuriedGem1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Half-buried Gem#1")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            HalfBuriedGem1Step::OnDisable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("spl_fild02"),
                        Val::from(25),
                        Val::from(220),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(10),
                        Val::from("Half-buried Gem#1::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Half-buried Gem#1")])?;
                return Err(Stop::End);
            }
            HalfBuriedGem1Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("spl_fild02"), Val::from("Half-buried Gem#1::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Half-buried Gem#1::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            HalfBuriedGem1Step::OnTimer300000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("spl_fild02"), Val::from("Half-buried Gem#1::OnMyMobDead")],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Half-buried Gem#1::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum HalfBuriedGem2Step {
    Start,
    OnInit,
    OnEnable,
    OnDisable,
    OnMyMobDead,
    OnTimer300000,
}

pub(super) fn half_buried_gem_2_run(ctx: &Ctx, mut step: HalfBuriedGem2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            HalfBuriedGem2Step::Start => {
                shared::quests_quests_13_2::jewel_13_2(ctx, vec![Val::from("Blue"), Val::from(2)])?;
                step = HalfBuriedGem2Step::OnInit;
                continue 'machine;
            }
            HalfBuriedGem2Step::OnInit => {
                ctx.call(Function::EnableNpc, vec![Val::from("Half-buried Gem#2")])?;
                return Err(Stop::End);
            }
            HalfBuriedGem2Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("Half-buried Gem#2")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            HalfBuriedGem2Step::OnDisable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(
                    Function::Monster,
                    vec![
                        Val::from("man_fild03"),
                        Val::from(227),
                        Val::from(109),
                        Val::from("Thief Bug"),
                        Val::from(1051),
                        Val::from(10),
                        Val::from("Half-buried Gem#2::OnMyMobDead"),
                    ],
                )?;
                ctx.call(Function::DisableNpc, vec![Val::from("Half-buried Gem#2")])?;
                return Err(Stop::End);
            }
            HalfBuriedGem2Step::OnMyMobDead => {
                if ctx
                    .call(
                        Function::MobCount,
                        vec![Val::from("man_fild03"), Val::from("Half-buried Gem#2::OnMyMobDead")],
                    )?
                    .number()?
                    < 1
                {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("Half-buried Gem#2::OnEnable")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            HalfBuriedGem2Step::OnTimer300000 => {
                ctx.call(
                    Function::KillMonster,
                    vec![Val::from("man_fild03"), Val::from("Half-buried Gem#2::OnMyMobDead")],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Half-buried Gem#2::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj0102TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj0201TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj0202TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj0401TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj0402TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj0901TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Mj1001TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Hu01TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Hu02TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Hu03TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Hu04TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Hu05TStep {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ve01TStep {
    Start,
    OnEnable,
    OnTimer40000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ve02TStep {
    Start,
    OnEnable,
    OnTimer40000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ve03TStep {
    Start,
    OnEnable,
    OnTimer40000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ve04TStep {
    Start,
    OnEnable,
    OnTimer40000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare1Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare2Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare3Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare4Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare5Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare6Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Tukare7Step {
    Start,
    OnEnable,
    OnTimer60000,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13WarpS0Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer4000,
    OnTimer6000,
    OnTimer8000,
    OnTimer10000,
    OnDisable,
}

pub(super) fn ep13_warp_s_0_run(ctx: &Ctx, mut step: Ep13WarpS0Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13WarpS0Step::Start => {
                step = Ep13WarpS0Step::OnEnable;
                continue 'machine;
            }
            Ep13WarpS0Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                step = Ep13WarpS0Step::OnTimer2000;
                continue 'machine;
            }
            Ep13WarpS0Step::OnTimer2000 => {
                step = Ep13WarpS0Step::OnTimer4000;
                continue 'machine;
            }
            Ep13WarpS0Step::OnTimer4000 => {
                step = Ep13WarpS0Step::OnTimer6000;
                continue 'machine;
            }
            Ep13WarpS0Step::OnTimer6000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                return Err(Stop::End);
            }
            Ep13WarpS0Step::OnTimer8000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                return Err(Stop::End);
            }
            Ep13WarpS0Step::OnTimer10000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Ep13WarpS0Step::OnDisable => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13WarpS1Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer4000,
    OnTimer6000,
    OnTimer8000,
    OnTimer10000,
    OnTimer12000,
    OnDisable,
}

pub(super) fn ep13_warp_s_1_run(ctx: &Ctx, mut step: Ep13WarpS1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13WarpS1Step::Start => {
                step = Ep13WarpS1Step::OnEnable;
                continue 'machine;
            }
            Ep13WarpS1Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                step = Ep13WarpS1Step::OnTimer2000;
                continue 'machine;
            }
            Ep13WarpS1Step::OnTimer2000 => {
                step = Ep13WarpS1Step::OnTimer4000;
                continue 'machine;
            }
            Ep13WarpS1Step::OnTimer4000 => {
                step = Ep13WarpS1Step::OnTimer6000;
                continue 'machine;
            }
            Ep13WarpS1Step::OnTimer6000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                return Err(Stop::End);
            }
            Ep13WarpS1Step::OnTimer8000 => {
                step = Ep13WarpS1Step::OnTimer10000;
                continue 'machine;
            }
            Ep13WarpS1Step::OnTimer10000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                return Err(Stop::End);
            }
            Ep13WarpS1Step::OnTimer12000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Ep13WarpS1Step::OnDisable => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13WarpS2Step {
    Start,
    OnEnable,
    OnTimer2000,
    OnTimer4000,
    OnTimer6000,
    OnTimer8000,
    OnTimer10000,
    OnTimer12000,
    OnTimer15000,
    OnDisable,
}

pub(super) fn ep13_warp_s_2_run(ctx: &Ctx, mut step: Ep13WarpS2Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13WarpS2Step::Start => {
                step = Ep13WarpS2Step::OnEnable;
                continue 'machine;
            }
            Ep13WarpS2Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                step = Ep13WarpS2Step::OnTimer2000;
                continue 'machine;
            }
            Ep13WarpS2Step::OnTimer2000 => {
                step = Ep13WarpS2Step::OnTimer4000;
                continue 'machine;
            }
            Ep13WarpS2Step::OnTimer4000 => {
                step = Ep13WarpS2Step::OnTimer6000;
                continue 'machine;
            }
            Ep13WarpS2Step::OnTimer6000 => {
                step = Ep13WarpS2Step::OnTimer8000;
                continue 'machine;
            }
            Ep13WarpS2Step::OnTimer8000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                return Err(Stop::End);
            }
            Ep13WarpS2Step::OnTimer10000 => {
                step = Ep13WarpS2Step::OnTimer12000;
                continue 'machine;
            }
            Ep13WarpS2Step::OnTimer12000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                return Err(Stop::End);
            }
            Ep13WarpS2Step::OnTimer15000 => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_STEAL")?])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
            Ep13WarpS2Step::OnDisable => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_WIND")?])?;
                ctx.call(Function::DisableNpc, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp222Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_22_2_run(ctx: &Ctx, mut step: Ep13Warp222Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp222Step::Start => {
                step = Ep13Warp222Step::OnInit;
                continue 'machine;
            }
            Ep13Warp222Step::OnInit => {
                step = Ep13Warp222Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp222Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_22_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp222Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_22_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp222Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_31::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_41::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_42::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43_2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_22_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp242Step {
    Start,
    OnEnable,
    OnTouch,
    OnInit,
    OnDisable,
}

pub(super) fn ep13_warp_24_2_run(ctx: &Ctx, mut step: Ep13Warp242Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp242Step::Start => {
                step = Ep13Warp242Step::OnEnable;
                continue 'machine;
            }
            Ep13Warp242Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_24_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp242Step::OnTouch => {
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_33::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43_2::OnEnable")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_25::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_26::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_35::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_45::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_45_2::OnEnable")])?;
                }
                step = Ep13Warp242Step::OnInit;
                continue 'machine;
            }
            Ep13Warp242Step::OnInit => {
                step = Ep13Warp242Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp242Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_24_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp432Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_43_2_run(ctx: &Ctx, mut step: Ep13Warp432Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp432Step::Start => {
                step = Ep13Warp432Step::OnInit;
                continue 'machine;
            }
            Ep13Warp432Step::OnInit => {
                step = Ep13Warp432Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp432Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_43_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp432Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_43_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp432Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_52::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_61::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_61_2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp452Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_45_2_run(ctx: &Ctx, mut step: Ep13Warp452Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp452Step::Start => {
                step = Ep13Warp452Step::OnInit;
                continue 'machine;
            }
            Ep13Warp452Step::OnInit => {
                step = Ep13Warp452Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp452Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_45_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp452Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_45_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp452Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_46::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_55::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_56::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_65::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_65_2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_45_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp612Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_61_2_run(ctx: &Ctx, mut step: Ep13Warp612Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp612Step::Start => {
                step = Ep13Warp612Step::OnInit;
                continue 'machine;
            }
            Ep13Warp612Step::OnInit => {
                step = Ep13Warp612Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp612Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_61_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp612Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_61_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp612Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_62::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_71::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_72::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_81::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_81_2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_61_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp652Step {
    Start,
    OnEnable,
    OnTouch,
    OnInit,
    OnDisable,
}

pub(super) fn ep13_warp_65_2_run(ctx: &Ctx, mut step: Ep13Warp652Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp652Step::Start => {
                step = Ep13Warp652Step::OnEnable;
                continue 'machine;
            }
            Ep13Warp652Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_65_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp652Step::OnTouch => {
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_64::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_73::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_82::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_83::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_83_2::OnEnable")])?;
                } else {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_66::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_75::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_76::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_85::OnEnable")])?;
                    ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_85_2::OnEnable")])?;
                }
                step = Ep13Warp652Step::OnInit;
                continue 'machine;
            }
            Ep13Warp652Step::OnInit => {
                step = Ep13Warp652Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp652Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_65_2")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp812Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_81_2_run(ctx: &Ctx, mut step: Ep13Warp812Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp812Step::Start => {
                step = Ep13Warp812Step::OnInit;
                continue 'machine;
            }
            Ep13Warp812Step::OnInit => {
                step = Ep13Warp812Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp812Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_81_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp812Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_81_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp812Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_91::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_92::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_93::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_e1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_81_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp832Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_83_2_run(ctx: &Ctx, mut step: Ep13Warp832Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp832Step::Start => {
                step = Ep13Warp832Step::OnInit;
                continue 'machine;
            }
            Ep13Warp832Step::OnInit => {
                step = Ep13Warp832Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp832Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_83_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp832Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_83_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp832Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_84::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_94::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_e2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_83_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Ep13Warp852Step {
    Start,
    OnInit,
    OnDisable,
    OnEnable,
    OnTouch,
}

pub(super) fn ep13_warp_85_2_run(ctx: &Ctx, mut step: Ep13Warp852Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Warp852Step::Start => {
                step = Ep13Warp852Step::OnInit;
                continue 'machine;
            }
            Ep13Warp852Step::OnInit => {
                step = Ep13Warp852Step::OnDisable;
                continue 'machine;
            }
            Ep13Warp852Step::OnDisable => {
                ctx.call(Function::DisableNpc, vec![Val::from("ep13_warp_85_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp852Step::OnEnable => {
                ctx.call(Function::EnableNpc, vec![Val::from("ep13_warp_85_2")])?;
                return Err(Stop::End);
            }
            Ep13Warp852Step::OnTouch => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_95::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_e3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_85_2::OnEnable")])?;
                return Err(Stop::End);
            }
        }
    }
}
