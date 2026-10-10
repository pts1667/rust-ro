#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Script, Stop, Val, args, constants, runtime};

pub fn kunai_merchant_kashin(ctx: &Ctx) -> Script {
    if ctx.var("BaseClass").get()? == constants::JOB_NINJA {
        ctx.lines_as(
            "Kashin",
            args![
                "I am Kashin, distributor",
                "of Kunai for Ninjas. Take",
                "a look around and let me",
                "know if you're interested",
                "in any of my wares."
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&[
            "10 Fell Poison Kunai",
            "10 Icicle Kunai",
            "10 High Wind Kunai",
            "10 Black Earth Kunai",
            "10 Heat Wave Kunai",
            "Cancel",
        ])? {
            0 => {
                shared::merchants_kunai_maker::kunai_trade(ctx, args![13250, 20, 7524, 1, 13259])?;
            }
            1 => {
                shared::merchants_kunai_maker::kunai_trade(ctx, args![13251, 8, 7522, 2, 13255])?;
            }
            2 => {
                shared::merchants_kunai_maker::kunai_trade(ctx, args![13252, 4, 7523, 2, 13257])?;
            }
            3 => {
                shared::merchants_kunai_maker::kunai_trade(ctx, args![13253, 2, 7524, 1, 13256])?;
            }
            4 => {
                shared::merchants_kunai_maker::kunai_trade(ctx, args![13254, 1, 7521, 2, 13258])?;
            }
            _ => {
                ctx.lines_as(
                    "Kashin",
                    args![
                        "Well then, thank you",
                        "for visiting my shop.",
                        "Please come to me when",
                        "you need to buy some",
                        "Kunais. Goodbye for now~"
                    ],
                )?;
                return ctx.close();
            }
        }
    }
    ctx.lines_as(
        "Kashin",
        args![
            "I am Kashin, distributor",
            "of Kunai for Ninjas. If you",
            "have any friends that are",
            "Ninjas, then you might",
            "want to tell them about me."
        ],
    )?;
    ctx.close()
}
