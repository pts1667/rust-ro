use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

pub fn ep13_warp_65_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_65_2_run(ctx, Ep13Warp652Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_65_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_65_2_run(ctx, Ep13Warp652Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_65_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_65_2_run(ctx, Ep13Warp652Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_65_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_65_2_run(ctx, Ep13Warp652Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_81_2(ctx: &Ctx) -> Script {
    ep13_warp_81_2_run(ctx, Ep13Warp812Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_81_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_81_2_run(ctx, Ep13Warp812Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_81_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_81_2_run(ctx, Ep13Warp812Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_81_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_81_2_run(ctx, Ep13Warp812Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_81_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_81_2_run(ctx, Ep13Warp812Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_83_2(ctx: &Ctx) -> Script {
    ep13_warp_83_2_run(ctx, Ep13Warp832Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_83_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_83_2_run(ctx, Ep13Warp832Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_83_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_83_2_run(ctx, Ep13Warp832Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_83_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_83_2_run(ctx, Ep13Warp832Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_83_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_83_2_run(ctx, Ep13Warp832Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn ep13_warp_85_2(ctx: &Ctx) -> Script {
    ep13_warp_85_2_run(ctx, Ep13Warp852Step::Start, Vec::new()).map(|_| ())
}

pub fn ep13_warp_85_2_oninit(ctx: &Ctx) -> Script {
    ep13_warp_85_2_run(ctx, Ep13Warp852Step::OnInit, Vec::new()).map(|_| ())
}

pub fn ep13_warp_85_2_ondisable(ctx: &Ctx) -> Script {
    ep13_warp_85_2_run(ctx, Ep13Warp852Step::OnDisable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_85_2_onenable(ctx: &Ctx) -> Script {
    ep13_warp_85_2_run(ctx, Ep13Warp852Step::OnEnable, Vec::new()).map(|_| ())
}

pub fn ep13_warp_85_2_ontouch(ctx: &Ctx) -> Script {
    ep13_warp_85_2_run(ctx, Ep13Warp852Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Ep13Nd2fMngStep {
    Start,
    OnReset,
}

fn ep13_nd2f_mng_run(ctx: &Ctx, mut step: Ep13Nd2fMngStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Ep13Nd2fMngStep::Start => {
                shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
                ctx.mes("Enter password.")?;
                ctx.next()?;
                if shared::other_gm_npcs::f_gm_npc(ctx, vec![Val::from(1854), Val::from(0)])? == 1 {
                    ctx.mes("Reset Control devices?")?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
                        1 => {
                            ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_nd2f_mng::OnReset")])?;
                            ctx.var("$@08_ep13nydun02_in").set(Val::from(0))?;
                            ctx.mes("Done.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.mes("Canceled.")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.mes("Invalid.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Ep13Nd2fMngStep::OnReset;
                continue 'machine;
            }
            Ep13Nd2fMngStep::OnReset => {
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_s1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_s3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_11::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_13::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_14::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_21::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_22::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_24::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_25::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_26::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_31::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_33::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_35::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_41::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_42::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_45::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_46::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_52::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_55::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_56::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_61::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_62::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_64::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_65::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_66::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_71::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_72::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_73::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_75::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_76::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_81::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_82::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_83::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_84::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_85::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_91::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_92::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_93::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_94::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_95::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_e1::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_e2::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_e3::OnEnable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_22_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_24_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_43_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_45_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_61_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_65_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_81_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_83_2::OnDisable")])?;
                ctx.call(Function::DoNpcEvent, vec![Val::from("ep13_warp_85_2::OnDisable")])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn ep13_nd2f_mng(ctx: &Ctx) -> Script {
    ep13_nd2f_mng_run(ctx, Ep13Nd2fMngStep::Start, Vec::new()).map(|_| ())
}

pub fn ep13_nd2f_mng_onreset(ctx: &Ctx) -> Script {
    ep13_nd2f_mng_run(ctx, Ep13Nd2fMngStep::OnReset, Vec::new()).map(|_| ())
}
