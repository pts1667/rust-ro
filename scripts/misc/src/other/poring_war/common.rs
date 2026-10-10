use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
pub(super) enum WopMasterStep {
    Start,
    OnReset,
    OnStart,
    OnAngelingWarn,
    OnDevilingWarn,
    OnDevilingEnd,
    OnAngelingEnd,
    OnStop,
    OnAngelingWin,
    OnDevilingWin,
    OnTimer5000,
    OnTimer8000,
    OnTimer12000,
    OnTimer32000,
    OnTimer62000,
    OnTimer70000,
    OnTimer75000,
    OnTimer80000,
    OnTimer85000,
    OnTimer90000,
    OnTimer95000,
    OnTimer100000,
    OnTimer700000,
    OnTimer703000,
}

pub(super) fn wop_master_run(ctx: &Ctx, mut step: WopMasterStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_i = Val::from(0);
    'machine: loop {
        match step {
            WopMasterStep::Start => {
                return Err(Stop::End);
            }
            WopMasterStep::OnReset => {
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PARTYLOCK")?],
                )?;
                ctx.call(Function::RemoveMapFlag, vec![Val::from("poring_w02"), ctx.constant("MF_PVP")?])?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOGUILD")?],
                )?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOCALCRANK")?],
                )?;
                ctx.call(
                    Function::MapWarp,
                    vec![Val::from("poring_w02"), Val::from("poring_w01"), Val::from(112), Val::from(138)],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
                ctx.var("$@wop_team_a").set(Val::from(0))?;
                ctx.var("$@wop_team_d").set(Val::from(0))?;
                ctx.var("$@wop_deadcount_a").set(Val::from(0))?;
                ctx.var("$@wop_deadcount_d").set(Val::from(0))?;
                ctx.var("$@wop_teamcount").set(Val::from(0))?;
                ctx.var("$@wop_doorcount_a").set(Val::from(0))?;
                ctx.var("$@wop_doorcount_d").set(Val::from(0))?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_a::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_d::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_rtry::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_d::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Poring#wop_door_all::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnStart => {
                ctx.call(Function::InitNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnAngelingWarn => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Deviling Team Recruitment is complete. The battle will be canceled automatically if the Angeling Team Recruitment isn't ready in 1 minute."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnDevilingWarn => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Angeling Team Recruitment is complete. The battle will be canceled automatically if the Deviling Team Recruitment isn't ready in 1 minute."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnDevilingEnd => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Deviling Team Recruitment failed. The battle will be canceled shortly."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnAngelingEnd => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Angeling Team Recruitment failed. The battle will be canceled shortly."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnAngelingWin => {
                l_i = Val::from(1);
                step = WopMasterStep::OnDevilingWin;
                continue 'machine;
            }
            WopMasterStep::OnDevilingWin => {
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PARTYLOCK")?],
                )?;
                ctx.call(Function::RemoveMapFlag, vec![Val::from("poring_w02"), ctx.constant("MF_PVP")?])?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOGUILD")?],
                )?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOCALCRANK")?],
                )?;
                if l_i.clone().is_true() {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("poring_w02"),
                            ((Val::from("Mr. Doppel: Angeling Team of party ")
                                + ctx.call(Function::GetPartyName, vec![ctx.var("$@wop_team_a").get()?])?)
                                + Val::from(" won the battle!")),
                            Val::from(0),
                            Val::from(15761536),
                        ],
                    )?;
                } else {
                    ctx.call(
                        Function::MapAnnounce,
                        vec![
                            Val::from("poring_w02"),
                            ((Val::from("Mr. Doppel: Deviling Team of party ")
                                + ctx.call(Function::GetPartyName, vec![ctx.var("$@wop_team_d").get()?])?)
                                + Val::from(" won the battle!")),
                            Val::from(0),
                            Val::from(15761536),
                        ],
                    )?;
                }
                ctx.call(Function::DoNpcEvent, vec![Val::from("Deviruchi#wop_endmaster::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#aroom_ingate_wop::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#aroom_outgate_wop::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#droom_ingate_wop::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#droom_outgate_wop::OnDisable")])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer5000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: Welcome to all the warriors that have come to fight the battle."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Mr. Doppel#wop_team_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("Mr. Doppel#wop_team_d::OnEnable")])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer8000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: Each member of the team must join the group, and the leader will register their team name."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer12000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: You have 50 seconds. Join the group, register it's name and go to the battlefield."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer32000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: You still have 30 seconds. Join the group, register it's name and go to the battlefield."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer62000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: Time is up. The rules will be explained shortly before the battle."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer70000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: All ready? Let me explain the battle rules."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer75000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Mr. Doppel: The Angeling Team has to protect 2 Angelings on their bases and, at the same time, eliminate the Devilings on their Deviling Team bases, and vice-versa."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer80000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Mr. Doppel: Your team wins if you eliminate the 2 monsters of the enemy team. The rules are as simple as that."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer85000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Mr. Doppel: But the Angelings or the Devilings return to life after a certain period of time, so it's important to eliminate the other Poring quickly, after you have eliminated the first."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer90000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Mr. Doppel: Also know that those Porings are furious and will attack everybody, it doesnt matter to what team they belong."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer95000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Mr. Doppel: Basically, the Porings are natural enemies of the humans, so the attack is innevitable, even if both are on the same team."), Val::from(0), Val::from(15761536)])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer100000 => {
                ctx.call(Function::MapAnnounce, vec![Val::from("poring_w02"), Val::from("Mr. Doppel: In certain areas, you will need War Badges that are obtained by eliminating members of the other team. Alright, lets begin!"), Val::from(0), Val::from(15761536)])?;
                ctx.call(
                    Function::SetMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PARTYLOCK")?],
                )?;
                ctx.call(Function::SetMapFlag, vec![Val::from("poring_w02"), ctx.constant("MF_PVP")?])?;
                ctx.call(
                    Function::SetMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOGUILD")?],
                )?;
                ctx.call(
                    Function::SetMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOCALCRANK")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_a::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_warp_d::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnAngelingSpawn")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnAngelingSpawn")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnDevilingSpawn")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnDevilingSpawn")])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer700000 => {
                ctx.call(
                    Function::MapAnnounce,
                    vec![
                        Val::from("poring_w02"),
                        Val::from("Mr. Doppel: Time is up. Soon, you will be teleported to the Winners Stage."),
                        Val::from(0),
                        Val::from(15761536),
                    ],
                )?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PARTYLOCK")?],
                )?;
                ctx.call(Function::RemoveMapFlag, vec![Val::from("poring_w02"), ctx.constant("MF_PVP")?])?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOGUILD")?],
                )?;
                ctx.call(
                    Function::RemoveMapFlag,
                    vec![Val::from("poring_w02"), ctx.constant("MF_PVP_NOCALCRANK")?],
                )?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium1::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_angellium2::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium1::OnReset")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("#wop_devillium2::OnReset")])?;
                return Err(Stop::End);
            }
            WopMasterStep::OnTimer703000 => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("Deviruchi#wop_endmaster::OnEnable")])?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}
