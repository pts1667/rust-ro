use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk11Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk12Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk13Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk14Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk15Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk21Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk22Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk23Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk24Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk25Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk31Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk32Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk33Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk34Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum MobMonk35Step {
    Start,
    OnTouch,
    OnDisable,
}

#[derive(Clone, Copy, Debug)]
pub(super) enum ResetterMonkStep {
    Start,
    OnTimer500000,
    OnInit,
    OnEnable,
}

pub(super) fn resetter_monk_run(ctx: &Ctx, mut step: ResetterMonkStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ResetterMonkStep::Start => {
                step = ResetterMonkStep::OnTimer500000;
                continue 'machine;
            }
            ResetterMonkStep::OnTimer500000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#1_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#2_5::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_1::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_3::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_4::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("mob_monk#3_5::OnDisable")])?;
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            ResetterMonkStep::OnInit => {
                step = ResetterMonkStep::OnEnable;
                continue 'machine;
            }
            ResetterMonkStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
