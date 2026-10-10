#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum GolbalVarStep {
    Start,
    LNumber,
    LVar,
    OnInit,
}

fn show_god_list(ctx: &Ctx) -> Result<Val, Stop> {
    ctx.lines(args![
        Val::from("^0000FF$God1^000000 = ^FF0000") + ctx.var("$god1").get()? + Val::from("^000000."),
        Val::from("^0000FF$God2^000000 = ^FF0000") + ctx.var("$god2").get()? + Val::from("^000000."),
        Val::from("^0000FF$God3^000000 = ^FF0000") + ctx.var("$god3").get()? + Val::from("^000000."),
        Val::from("^0000FF$God4^000000 = ^FF0000") + ctx.var("$god4").get()? + Val::from("^000000.")
    ])?;
    ctx.close_window()?;
    Err(Stop::End)
}

fn golbal_var_run(ctx: &Ctx, mut step: GolbalVarStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    'machine: loop {
        match step {
            GolbalVarStep::Start => {
                shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
                ctx.lines_as("Check", args!["Please enter the password."])?;
                ctx.next()?;
                if shared::other_gm_npcs::f_gm_npc(ctx, args![68392411, 0])? != 1 {
                    ctx.lines_as("Check", args!["Incorrect password."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Check", args!["Please choose a menu."])?;
                ctx.next()?;
                match ctx.menu(&[
                    "Now",
                    "No.1",
                    "No.2",
                    "No.3",
                    "No.4",
                    "Reset",
                    "god_sl_1",
                    "god_eremes",
                    "god_brising",
                    "god_mjo_0",
                    "god_mjo_1",
                    "god_mjo_2",
                    "god_mjo_3",
                    "god_mjo_4",
                ])? {
                    0 => return show_god_list(ctx),
                    1 => {
                        golbal_var_run(ctx, GolbalVarStep::LNumber, args!["$God1"])?;
                    }
                    2 => {
                        golbal_var_run(ctx, GolbalVarStep::LNumber, args!["$God2"])?;
                    }
                    3 => {
                        golbal_var_run(ctx, GolbalVarStep::LNumber, args!["$God3"])?;
                    }
                    4 => {
                        golbal_var_run(ctx, GolbalVarStep::LNumber, args!["$God4"])?;
                    }
                    5 => {
                        ctx.lines_as(
                            "Check",
                            args![
                                "Are you really sure that you want to reset the entire list of God Globalvar?",
                                "Please enter the password."
                            ],
                        )?;
                        ctx.next()?;
                        if shared::other_gm_npcs::f_gm_npc(ctx, args![68392411, 0])? != 1 {
                            ctx.lines_as("Check", args!["The command has been canceled."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Check", args!["Now, the entire list of God Globalvar is being reset."])?;
                        ctx.next()?;
                        ctx.var("$god1").set(0)?;
                        ctx.var("$god2").set(0)?;
                        ctx.var("$god3").set(0)?;
                        ctx.var("$god4").set(0)?;
                        return show_god_list(ctx);
                    }
                    6 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_sl_1"])?;
                    }
                    7 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_eremes"])?;
                    }
                    8 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_brising"])?;
                    }
                    9 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_mjo_0"])?;
                    }
                    10 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_mjo_1"])?;
                    }
                    11 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_mjo_2"])?;
                    }
                    12 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_mjo_3"])?;
                    }
                    13 => {
                        golbal_var_run(ctx, GolbalVarStep::LVar, args!["god_mjo_4"])?;
                    }
                    _ => {}
                }
                step = GolbalVarStep::LNumber;
                continue 'machine;
            }
            GolbalVarStep::LNumber => {
                let (input, status) = runtime::input_number(ctx, Some(0), Some(ctx.var("$@god_check2").get()?.number()?))?;
                l_input = input;
                ctx.lines_as(
                    "Check",
                    args![
                        Val::from("Would you like to change to ") + l_input.clone() + Val::from("?"),
                        "Please enter the password."
                    ],
                )?;
                ctx.next()?;
                if shared::other_gm_npcs::f_gm_npc(ctx, args![68392411, 0])? != 1 {
                    ctx.lines_as("Check", args!["The command has been canceled."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Check",
                    args![Val::from("The command ") + l_input.clone() + Val::from(" has been confirmed.")],
                )?;
                ctx.next()?;
                runtime::setd(
                    ctx,
                    &runtime::arg(&args, 0, Val::from(0)),
                    l_input.clone(),
                    &mut [(".@input", runtime::LocalMut::Scalar(&mut l_input))],
                )?;
                ctx.lines(args![runtime::arg(&args, 0, Val::from(0)) + Val::from(" ") + l_input.clone()])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            GolbalVarStep::LVar => {
                let (input, status) = runtime::input_number(ctx, None, None)?;
                l_input = input;
                runtime::setd(
                    ctx,
                    &runtime::arg(&args, 0, Val::from(0)),
                    l_input.clone(),
                    &mut [(".@input", runtime::LocalMut::Scalar(&mut l_input))],
                )?;
                ctx.next()?;
                ctx.lines_as("Check", args!["Done."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            GolbalVarStep::OnInit => {
                ctx.var("$@god_check1").set(Val::from(50))?;
                ctx.var("$@god_check2").set(Val::from(100))?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn golbal_var(ctx: &Ctx) -> Script {
    golbal_var_run(ctx, GolbalVarStep::Start, Vec::new()).map(|_| ())
}

pub fn golbal_var_oninit(ctx: &Ctx) -> Script {
    golbal_var_run(ctx, GolbalVarStep::OnInit, Vec::new()).map(|_| ())
}
