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

#[derive(Clone, Copy, Debug)]
enum MagazineDealerKennyStep {
    Start,
    Exchange,
}

fn magazine_dealer_kenny_run(ctx: &Ctx, step: MagazineDealerKennyStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MagazineDealerKennyStep::Start => {
                ctx.mes("[Kenny]")?;
                if ctx.var("BaseClass").get()? == constants::JOB_GUNSLINGER {
                    ctx.lines(args![
                        "Welcome to my Magazine Shop.",
                        "As you may know, large numbers",
                        "of bullets can be carried more",
                        "easily when they're in Magazines. Now, can I interest you in",
                        "anything in particular?"
                    ])?;
                    ctx.next()?;
                    match ctx.menu(&[
                        "Wind Sphere Pack",
                        "Shadow Sphere Pack",
                        "Poison Sphere Pack",
                        "Water Sphere Pack",
                        "Fire Sphere Pack",
                        "Cartridge",
                        "Blood Cartridge",
                        "Silver Cartridge",
                        "Cancel",
                    ])? {
                        0 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13204, 12144]),
                        1 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13206, 12145]),
                        2 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13205, 12146]),
                        3 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13207, 12147]),
                        4 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13203, 12148]),
                        5 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13200, 12149]),
                        6 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13202, 12150]),
                        7 => return magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Exchange, args![13201, 12151]),
                        8 => {
                            ctx.lines_as(
                                "Kenny",
                                args![
                                    "Well, if you ever find",
                                    "that you have too many",
                                    "bullets, come and see me.",
                                    "It's a smart idea to store",
                                    "bullets with my Magazines."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.lines(args![
                    "Welcome to my shop.",
                    "Here, I provide Magazines",
                    "and Cartridges for Gunslingers.",
                    "Sorry, but it doesn't look like",
                    "my services would be of any",
                    "use to you, adventurer."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Kenny",
                    args![
                        "Eh, but if you happen to",
                        "know any Gunslingers, send",
                        "them my way. You can never",
                        "have too many bullets."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            MagazineDealerKennyStep::Exchange => {
                let bullet_id = runtime::arg(&args, 0, Val::from(0));
                let pack_id = runtime::arg(&args, 1, Val::from(0));
                ctx.lines_as(
                    "Kenny",
                    args![
                        "Now, you can trade",
                        shared::other_global_functions::f_insertplural(
                            ctx,
                            args![500, ctx.call(Function::GetItemName, args![bullet_id.clone()])?]
                        )?,
                        Val::from("and 500 zeny for 1 ")
                            + ctx.call(Function::GetItemName, args![pack_id.clone()])?
                            + Val::from(", so make sure"),
                        "you have sufficient bullets",
                        "and zeny for this exchange."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kenny",
                    args![
                        Val::from("You can exchange ")
                            + shared::other_global_functions::f_insertplural(
                                ctx,
                                args![500, ctx.call(Function::GetItemName, args![bullet_id.clone()])?]
                            )?
                            + Val::from(" and 500 zeny"),
                        Val::from("with 1 ") + ctx.call(Function::GetItemName, args![pack_id.clone()])? + Val::from(".")
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kenny",
                    args![
                        "Remember that I can't give",
                        "you more than 50 Magazines",
                        "at a time. Now please enter",
                        "the number of Magazines you",
                        "want to receive. If you want to cancel, then just enter ''0.''"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                ctx.mes("[Kenny]")?;
                let amount = input.number()?;
                if amount > 50 || amount == 0 {
                    ctx.lines(args![
                        "Hey, I can't give you",
                        "that many Magazines.",
                        "Please try again, and",
                        "enter a number no",
                        "greater than 50."
                    ])?;
                } else if ctx.call(Function::CountItem, args![bullet_id.clone()])?.number()? >= 500 * amount {
                    if ctx.player().zeny()? >= 500 * amount {
                        if ctx.call(Function::CheckWeight, args![pack_id.clone(), amount])? == 0 {
                            ctx.lines(args![
                                "Hey, you've got a lot",
                                "of junk crammed in your",
                                "Inventory. Free up some",
                                "space, and then come back",
                                "and trade your bullets for",
                                "some Magazines later, okay?"
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Alright, here are",
                                "your Magazines. Thanks",
                                "for visiting my shop, and",
                                "I hope that you use all",
                                "of your ammo wisely."
                            ])?;
                            ctx.player().set_zeny(ctx.player().zeny()? - 500 * amount)?;
                            ctx.call(Function::DelItem, args![bullet_id.clone(), 500 * amount])?;
                            ctx.call(Function::GetItem, args![pack_id.clone(), amount])?;
                        }
                    } else {
                        ctx.lines(args![
                            "Sorry, but you don't",
                            "have enough zeny for",
                            "this Magazine exchange.",
                            "Come back to my shop",
                            "after you've saved up",
                            "some more money."
                        ])?;
                    }
                } else {
                    ctx.lines(args![
                        "Sorry, but you don't",
                        "have enough bullets for",
                        "this Magazine exchange.",
                        "Maybe you should double",
                        "check your Inventory, and",
                        "then come back to me later."
                    ])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn magazine_dealer_kenny(ctx: &Ctx) -> Script {
    magazine_dealer_kenny_run(ctx, MagazineDealerKennyStep::Start, Vec::new()).map(|_| ())
}
