#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

#[derive(Clone, Copy, Debug)]
enum FKiennaStep {
    Start,
    OnInit,
}

fn f_kienna_run(ctx: &Ctx, step: FKiennaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FKiennaStep::Start => {
                if ctx.var("rog_sk").get()? == 6 {
                    ctx.npc().do_event("#1stmove::OnDisable")?;
                    ctx.lines_as(
                        "Kienna",
                        args![
                            "Alright, in this",
                            "exercise, you'll need",
                            "to predict which way I'm",
                            "going to move and block",
                            "me from moving, essentially",
                            "immobilizing me. Get ready~"
                        ],
                    )?;
                    ctx.var("rog_sk").set(Val::from(7))?;
                    ctx.next()?;
                    let mut suc: i32 = 0;
                    for _ in 0..10 {
                        let direction = ctx.call(Function::Rand, args![1, 3])?;
                        if Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Block her to the Left:Block her to the Right:Block her retreat")],
                        )?)
                        .loosely_equals(&direction)
                        {
                            ctx.lines_as("Kienna", args!["Huh...?", "You blocked me!", "Very nice work~"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_POTION1")?])?;
                            ctx.call(Function::SpecialEffect, vec![ctx.constant("EF_POTION7")?])?;
                            suc += 1;
                        } else {
                            ctx.lines_as(
                                "Kienna",
                                args!["Sorry, but I wasn't", "moving in that direction.", "Your block attempt failed..."],
                            )?;
                        }
                        ctx.next()?;
                    }
                    ctx.lines_as("Kienna", args!["Alright, we're done", "here. You earned a"])?;
                    match suc {
                        10 => {
                            ctx.lines(args!["training grade of ''^0000FFS^000000.''", "That's a perfect score!"])?;
                        }
                        9 => {
                            ctx.lines(args!["training grade of ''^0000FFA^000000.''", "You're really good at this!"])?;
                        }
                        8 => {
                            ctx.lines(args!["training grade of ''^0000FFB +^000000.''", "That's very commendable!"])?;
                        }
                        7 => {
                            ctx.lines(args!["training grade of ''^0000FFB^000000.''", "That's very nice work~"])?;
                        }
                        6 => {
                            ctx.lines(args![
                                "training grade of ''^0000FFC +^000000.''",
                                "Not too bad, but you",
                                "just barely passed!"
                            ])?;
                        }
                        5 => ctx.mes("training grade of ''^FF0000C^000000.''")?,
                        4 => ctx.mes("training grade of ''^FF0000D+^000000.''")?,
                        3 => ctx.mes("training grade of ''^FF0000D^000000.''")?,
                        2 => ctx.mes("training grade of ''^FF0000F^000000.''")?,
                        1 => ctx.mes("training grade of ''^FF0000F -^000000.''")?,
                        0 => {
                            ctx.lines(args![
                                "training grade of...",
                                "Actually, I'm not able",
                                "to calculate it. What",
                                "could have happened?!"
                            ])?;
                        }
                        _ => {}
                    }
                    ctx.next()?;
                    if suc > 5 {
                        ctx.lines_as(
                            "Kienna",
                            args![
                                "I'm happy to say that",
                                "you've completed your",
                                "training! Let me send",
                                "you back to Thor Greg",
                                "now so that you can finish",
                                "learning ^FF0000Close Confine^000000."
                            ],
                        )?;
                        ctx.var("rog_sk").set(Val::from(8))?;
                        ctx.call(
                            Function::DisableNpc,
                            args![Val::from("Kienna#") + runtime::arg(&args, 0, Val::from(0))],
                        )?;
                        ctx.npc().do_event("#1st5min::OnDisable")?;
                        ctx.set_npc_visible("#1strecog", true)?;
                        ctx.close_window()?;
                        ctx.npc().do_event("Waiting Room#rogue10::OnEnable")?;
                        ctx.warp("in_rogue", 264, 124)?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "Kienna",
                            args![
                                "Hm. With this grade,",
                                "I don't think you're",
                                "quite ready to finish",
                                "learning Close Confine.",
                                "Would you like to try the",
                                "training exercise again?"
                            ],
                        )?;
                        ctx.next()?;
                        if Val::from(runtime::select_values(ctx, &[Val::from("Yes!:No, thanks.")])?) == 1 {
                            ctx.call(
                                Function::DisableNpc,
                                args![Val::from("Kienna#") + runtime::arg(&args, 0, Val::from(0))],
                            )?;
                            ctx.npc().do_event("#1stmove::OnEnable")?;
                            ctx.var("rog_sk").set(Val::from(6))?;
                            ctx.warp("prt_are01", 150, 150)?;
                            return Err(Stop::End);
                        }
                        ctx.call(
                            Function::DisableNpc,
                            args![Val::from("Kienna#") + runtime::arg(&args, 0, Val::from(0))],
                        )?;
                        ctx.npc().do_event("#1st5min::OnDisable")?;
                        ctx.set_npc_visible("#1strecog", true)?;
                        ctx.var("rog_sk").set(Val::from(6))?;
                        ctx.npc().do_event("Waiting Room#rogue10::OnEnable")?;
                        ctx.warp("in_rogue", 264, 124)?;
                        return Err(Stop::End);
                    }
                } else if ctx.var("rog_sk").get()? == 7 {
                    ctx.lines_as(
                        "Kienna",
                        args![
                            "You must have canceled",
                            "your training in the middle",
                            "of the exercise. I'm sorry,",
                            "but you'll have to start",
                            "from the very beginning",
                            "of this training."
                        ],
                    )?;
                    ctx.var("rog_sk").set(Val::from(6))?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::DisableNpc,
                        args![Val::from("Kienna#") + runtime::arg(&args, 0, Val::from(0))],
                    )?;
                    ctx.npc().do_event("#1stmove::OnEnable")?;
                    ctx.warp("prt_are01", 150, 150)?;
                    return Err(Stop::End);
                } else if ctx.var("rog_sk").get()? == 8 {
                    ctx.lines_as(
                        "Kienna",
                        args![
                            "You've already completed",
                            "the training exercise for",
                            "the Close Confine skill.",
                            "You no longer have need",
                            "for my assistance."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.npc().do_event("Waiting Room#rogue10::OnEnable")?;
                    ctx.warp("in_rogue", 264, 124)?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Kienna",
                        args![
                            "I'm sorry, but I don't",
                            "think you belong here.",
                            "Let me send you to the",
                            "Rogue Guild if you're lost..."
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.npc().do_event("Waiting Room#rogue10::OnEnable")?;
                    ctx.warp("in_rogue", 264, 124)?;
                    return Err(Stop::End);
                }
            }
            FKiennaStep::OnInit => {
                ctx.set_npc_visible("Kienna#1st", false)?;
                ctx.set_npc_visible("Kienna#2nd", false)?;
                ctx.set_npc_visible("Kienna#3rd", false)?;
                ctx.set_npc_visible("Kienna#4th", false)?;
                ctx.set_npc_visible("Kienna#5th", false)?;
                ctx.set_npc_visible("Kienna#6th", false)?;
                ctx.set_npc_visible("Kienna#7th", false)?;
                ctx.set_npc_visible("Kienna#8th", false)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn f_kienna(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_kienna_run(ctx, FKiennaStep::Start, args)
}
