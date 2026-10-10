use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum KiehlRoomWarpStep {
    Start,
    OnTouch,
    OnEnable,
    OnTimer30000,
    OnInit,
}

pub(super) fn kiehl_room_warp_run(ctx: &Ctx, mut step: KiehlRoomWarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KiehlRoomWarpStep::Start => {
                step = KiehlRoomWarpStep::OnTouch;
                continue 'machine;
            }
            KiehlRoomWarpStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(10), Val::from(31)])?;
                return Err(Stop::End);
            }
            KiehlRoomWarpStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            KiehlRoomWarpStep::OnTimer30000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Kiehl_Room_Warp")])?;
                return Err(Stop::End);
            }
            KiehlRoomWarpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kiehl_Room_Warp")])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BigDoor1WarpStep {
    Start,
    OnEnable,
    OnInit,
    OnTimer30000,
    OnTouch,
}

pub(super) fn big_door_1_warp_run(ctx: &Ctx, mut step: BigDoor1WarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BigDoor1WarpStep::Start => {
                step = BigDoor1WarpStep::OnEnable;
                continue 'machine;
            }
            BigDoor1WarpStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            BigDoor1WarpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_1_Warp")])?;
                return Err(Stop::End);
            }
            BigDoor1WarpStep::OnTimer30000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_1_Warp")])?;
                ctx.call(
                    Function::SetVariableOfNpc,
                    vec![
                        Val::from(".khdoor1opened"),
                        Val::from("Big Door#BigDoorKHQ1"),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            BigDoor1WarpStep::OnTouch => {
                ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(55), Val::from(33)])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BigDoor2WarpStep {
    Start,
    OnEnable,
    OnInit,
    OnTimer30000,
    OnTouch,
}

pub(super) fn big_door_2_warp_run(ctx: &Ctx, mut step: BigDoor2WarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BigDoor2WarpStep::Start => {
                step = BigDoor2WarpStep::OnEnable;
                continue 'machine;
            }
            BigDoor2WarpStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            BigDoor2WarpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_2_Warp")])?;
                return Err(Stop::End);
            }
            BigDoor2WarpStep::OnTimer30000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_2_Warp")])?;
                ctx.call(
                    Function::SetVariableOfNpc,
                    vec![
                        Val::from(".khdoor2opened"),
                        Val::from("Big Door#BigDoorKHQ2"),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            BigDoor2WarpStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 48 {
                    ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(173), Val::from(35)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(173), Val::from(52)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BigDoor3WarpStep {
    Start,
    OnEnable,
    OnInit,
    OnTimer30000,
    OnTouch,
}

pub(super) fn big_door_3_warp_run(ctx: &Ctx, mut step: BigDoor3WarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BigDoor3WarpStep::Start => {
                step = BigDoor3WarpStep::OnEnable;
                continue 'machine;
            }
            BigDoor3WarpStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            BigDoor3WarpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_3_Warp")])?;
                return Err(Stop::End);
            }
            BigDoor3WarpStep::OnTimer30000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_3_Warp")])?;
                ctx.call(
                    Function::SetVariableOfNpc,
                    vec![
                        Val::from(".khdoor3opened"),
                        Val::from("Big Door#BigDoorKHQ3"),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            BigDoor3WarpStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 49 {
                    ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(82), Val::from(108)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(68), Val::from(108)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum BigDoor4WarpStep {
    Start,
    OnEnable,
    OnInit,
    OnTimer30000,
    OnTouch,
}

pub(super) fn big_door_4_warp_run(ctx: &Ctx, mut step: BigDoor4WarpStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BigDoor4WarpStep::Start => {
                step = BigDoor4WarpStep::OnEnable;
                continue 'machine;
            }
            BigDoor4WarpStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            BigDoor4WarpStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_4_Warp")])?;
                return Err(Stop::End);
            }
            BigDoor4WarpStep::OnTimer30000 => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                ctx.call(Function::DisableNpc, vec![Val::from("Big_Door_4_Warp")])?;
                ctx.call(
                    Function::SetVariableOfNpc,
                    vec![
                        Val::from(".khdoor4opened"),
                        Val::from("Big Door#BigDoorKHQ4"),
                        Val::from(0),
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            BigDoor4WarpStep::OnTouch => {
                if ctx.var("kielhyrequest").get()? == 50 {
                    ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(38), Val::from(178)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("kh_kiehl01"), Val::from(47), Val::from(171)])?;
                }
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum KiehlRoomTrapStep {
    Start,
    OnTouch,
    OnTimer300000,
    OnTimer600000,
    OnTimer900000,
    OnTimer1200000,
    OnGlobalTimerOff,
}

pub(super) fn kiehl_room_trap_run(ctx: &Ctx, mut step: KiehlRoomTrapStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KiehlRoomTrapStep::Start => {
                return Err(Stop::End);
            }
            KiehlRoomTrapStep::OnTouch => {
                if ctx.var(".khtrapsprung").get()?.number()? < 1 {
                    ctx.var(".khtrapsprung").set(Val::from(1))?;
                    ctx.var("$@khquestbusy").set(Val::from(1))?;
                    ctx.call(Function::InitNpcTimer, vec![])?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("kh_kiehl02"),
                            Val::from(47),
                            Val::from(13),
                            Val::from("Aliot"),
                            Val::from(1740),
                            Val::from(1),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("kh_kiehl02"),
                            Val::from(45),
                            Val::from(13),
                            Val::from("Alicel"),
                            Val::from(1739),
                            Val::from(1),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("kh_kiehl02"),
                            Val::from(49),
                            Val::from(13),
                            Val::from("Constant"),
                            Val::from(1745),
                            Val::from(1),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("kh_kiehl02"),
                            Val::from(51),
                            Val::from(13),
                            Val::from("Aliot"),
                            Val::from(1740),
                            Val::from(1),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("kh_kiehl02"),
                            Val::from(53),
                            Val::from(13),
                            Val::from("Alicel"),
                            Val::from(1739),
                            Val::from(1),
                        ],
                    )?;
                    ctx.call(
                        Function::Monster,
                        vec![
                            Val::from("kh_kiehl02"),
                            Val::from(49),
                            Val::from(13),
                            Val::from("Constant"),
                            Val::from(1745),
                            Val::from(1),
                        ],
                    )?;
                }
                return Err(Stop::End);
            }
            KiehlRoomTrapStep::OnTimer300000 => {
                step = KiehlRoomTrapStep::OnTimer600000;
                continue 'machine;
            }
            KiehlRoomTrapStep::OnTimer600000 => {
                step = KiehlRoomTrapStep::OnTimer900000;
                continue 'machine;
            }
            KiehlRoomTrapStep::OnTimer900000 => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("kh_kiehl02")])? == 0 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("KiehlRoom::OnReset")])?;
                    ctx.call(Function::StopNpcTimer, vec![])?;
                }
                return Err(Stop::End);
            }
            KiehlRoomTrapStep::OnTimer1200000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("KiehlRoom::OnReset")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            KiehlRoomTrapStep::OnGlobalTimerOff => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum KiehlRoomExitStep {
    Start,
    OnTouch,
    OnEnable,
    OnInit,
}

pub(super) fn kiehl_room_exit_run(ctx: &Ctx, mut step: KiehlRoomExitStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            KiehlRoomExitStep::Start => {
                step = KiehlRoomExitStep::OnTouch;
                continue 'machine;
            }
            KiehlRoomExitStep::OnTouch => {
                if ctx.call(Function::GetMapUsers, vec![Val::from("kh_kiehl02")])?.number()? < 1 {
                    ctx.call(Function::DoNpcEvent, vec![Val::from("KiehlRoom::OnReset")])?;
                }
                ctx.call(Function::Warp, vec![Val::from("lighthalzen"), Val::from(193), Val::from(200)])?;
                return Err(Stop::End);
            }
            KiehlRoomExitStep::OnEnable => {
                ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_SUMMONSLAVE")?])?;
                return Err(Stop::End);
            }
            KiehlRoomExitStep::OnInit => {
                ctx.call(Function::DisableNpc, vec![Val::from("Kiehl_Room_Exit")])?;
                return Err(Stop::End);
            }
        }
    }
}
