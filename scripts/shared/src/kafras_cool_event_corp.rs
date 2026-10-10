#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn f_cooleventcorp(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.fx().cutin("zonda_01", 2)?;
    ctx.lines_as(
        "Cool Event Corp. Staff",
        args![
            "Welcome to Cool Event Corp.",
            "Our staff is always working",
            "to surpass your expactations",
            "for quality service. So how",
            "may I assist you today?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[runtime::arg(&args, 0, Val::from(0))])?);
        let mut matched1 = false;
        let no_case1 = subject1 != 1 && subject1 != 2 && subject1 != 3 && subject1 != 4 && subject1 != 5 && subject1 != 6;
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Cool Event Corp. Staff",
                args![
                    "Your Respawn Point",
                    "has been saved here",
                    runtime::arg(&args, 1, Val::from(0)) + Val::from("."),
                    "Thank you for using the",
                    "Cool Event Corp. service~"
                ],
            )?;
            ctx.call(
                Function::SavePoint,
                args![
                    runtime::arg(&args, 2, Val::from(0)),
                    runtime::arg(&args, 3, Val::from(0)),
                    runtime::arg(&args, 4, Val::from(0)),
                    1,
                    1
                ],
            )?;
            ctx.close_window()?;
            break 'b1;
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("[Cool Event Corp. Staff]")?;
            if !crate::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true() {
                ctx.lines(args![
                    "I'm sorry, but you",
                    "need the Novice's",
                    "Basic Skill Level 6 to",
                    "use the Storage Service."
                ])?;
                ctx.close_window()?;
                break 'b1;
            }
            if ctx.player().zeny()? < 40 {
                ctx.lines(args![
                    "I'm sorry, but you don't",
                    "have enough Zeny to use",
                    "the Storage Service. Our",
                    "Storage access fee is 40 Zeny."
                ])?;
                ctx.close_window()?;
                break 'b1;
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 40)?;
            ctx.lines(args![
                "Let me open your personal",
                "storage for you right away.",
                "Thanks for supporting Cool",
                "Event Corp. by using our",
                "services. Have a good day~"
            ])?;
            crate::kafras_functions_kafras::f_checkkafcode(ctx, vec![])?;
            ctx.close_window()?;
            ctx.call(Function::OpenStorage, vec![])?;
            break 'b1;
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines(args!["Please choose", "your destination."])?;
            ctx.next()?;
            let cost: i32 = if constants::VIP_SCRIPT != 0 && !ctx.call(Function::VipStatus, args![constants::VIP_STATUS_ACTIVE])?.is_true()
            {
                4400
            } else {
                2200
            };
            let destination = runtime::arg(&args, 5, Val::from(0));
            if Val::from(runtime::select_values(
                ctx,
                &[destination.clone() + Val::from(" -> ") + Val::from(cost) + Val::from(" z:Cancel")],
            )?) == 1
            {
                if ctx.call(Function::CountItem, args![7060])? != 0 || ctx.player().zeny()? >= cost {
                    if ctx.call(Function::CountItem, args![7060])? != 0 {
                        ctx.items().take(7060, 1)?;
                    } else {
                        ctx.player().set_zeny(ctx.player().zeny()? - cost)?;
                    }
                    if destination == "Veins" {
                        ctx.warp("veins", 205, 101)?;
                    } else if destination == "Rachel" {
                        ctx.warp("rachel", 115, 125)?;
                    }
                    ctx.fx().cutin("", 255)?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Cool Event Corp. Staff",
                        args![
                            "I'm sorry, but you don't have",
                            "enough zeny for the Teleport",
                            "Service. The fee to teleport",
                            Val::from("to ") + destination.clone() + Val::from(" is ") + Val::from(cost) + Val::from(" zeny.")
                        ],
                    )?;
                }
            }
            ctx.close_window()?;
            break 'b1;
        }
        if !matched1 && subject1 == 4 {
            matched1 = true;
        }
        if matched1 {
            ctx.mes("[Cool Event Corp. Staff]")?;
            if ctx.var("BaseClass").get()? != constants::JOB_MERCHANT {
                ctx.lines(args!["I'm sorry, but the", "Pushcart rental service"])?;
                ctx.lines(args![
                    "is only available to Merchants,",
                    "Blacksmiths, White Smiths,",
                    "Alchemists and Creators."
                ])?;
                ctx.close_window()?;
                break 'b1;
            }
            if ctx.call(Function::GetSkillLv, args!["MC_PUSHCART"])? == 0 {
                ctx.lines(args!["You can only rent a cart after", "learning the Pushcart Skill."])?;
                ctx.close_window()?;
                break 'b1;
            }
            if ctx.call(Function::CheckCart, vec![])? == 1 {
                ctx.lines(args![
                    "You already have",
                    "a Pushcart equipped.",
                    "Unfortunately, we can't",
                    "rent more than one to",
                    "each customer at a time."
                ])?;
                ctx.close_window()?;
                break 'b1;
            }
            if ctx.call(Function::CountItem, args![7061])?.is_true() {
                ctx.items().take(7061, 1)?;
                ctx.call(Function::SetCart, vec![])?;
                ctx.close_window()?;
                break 'b1;
            }
            ctx.lines(args![
                "The Pushcart rental",
                "fee is 800 Zeny. Would",
                "you like to rent a Pushcart?"
            ])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Rent a Pushcart:Cancel")])?) == 1 {
                if ctx.player().zeny()? < 800 {
                    ctx.lines_as(
                        "Cool Event Corp. Staff",
                        args![
                            "I'm sorry, but you",
                            "don't have enough",
                            "Zeny to pay the Pushcart",
                            "rental fee of 800 Zeny."
                        ],
                    )?;
                } else {
                    ctx.player().set_zeny(ctx.player().zeny()? - 800)?;
                    ctx.call(Function::SetCart, vec![])?;
                }
            }
            ctx.close_window()?;
            break 'b1;
        }
        if !matched1 && subject1 == 5 {
            matched1 = true;
        }
        if matched1 {
            crate::kafras_functions_kafras::f_setkafcode(ctx, args!["[Cool Event Corp. Staff]", "Cool Event Corp."])?;
        }
        if !matched1 && subject1 == 6 {
            matched1 = true;
        }
        if !matched1 && no_case1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Cool Event Corp. Staff",
                args![
                    "Cool Event Corp. is always",
                    "striving to provide the best",
                    "services for our customers.",
                    "Help us become the best by",
                    "providing us with your opinions",
                    "and honest feedback. Thank you."
                ],
            )?;
            ctx.close_window()?;
        }
    }
    ctx.fx().cutin("", 255)?;
    Err(Stop::End)
}
