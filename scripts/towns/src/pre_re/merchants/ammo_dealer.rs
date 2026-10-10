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
enum BulletDealerTonyAlbStep {
    Start,
    BulletTrade,
}

fn bullet_dealer_tony_alb_run(ctx: &Ctx, step: BulletDealerTonyAlbStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            BulletDealerTonyAlbStep::Start => {
                ctx.mes("[Tony]")?;
                if ctx.var("BaseClass").get()? == constants::JOB_GUNSLINGER {
                    ctx.lines(args![
                        "I'm Tony, the Bullet Dealer.",
                        "Come to me whenever you're",
                        "short on ammo. Just bring me",
                        "the materials, and I'll make",
                        "you the bullets you need."
                    ])?;
                    ctx.next()?;
                    match ctx.menu(&[
                        "Poison Sphere",
                        "Flare Sphere",
                        "Lighting Sphere",
                        "Blind Sphere",
                        "Freezing Sphere",
                        "Cancel",
                    ])? {
                        0 => {
                            bullet_dealer_tony_alb_run(ctx, BulletDealerTonyAlbStep::BulletTrade, args![937, 10, 13205])?;
                        }
                        1 => {
                            bullet_dealer_tony_alb_run(ctx, BulletDealerTonyAlbStep::BulletTrade, args![7097, 2, 13203])?;
                        }
                        2 => {
                            bullet_dealer_tony_alb_run(ctx, BulletDealerTonyAlbStep::BulletTrade, args![7053, 3, 13204])?;
                        }
                        3 => {
                            bullet_dealer_tony_alb_run(ctx, BulletDealerTonyAlbStep::BulletTrade, args![1024, 5, 13206])?;
                        }
                        4 => {
                            bullet_dealer_tony_alb_run(ctx, BulletDealerTonyAlbStep::BulletTrade, args![7054, 2, 13207])?;
                        }
                        5 => {
                            ctx.lines_as(
                                "Tony",
                                args![
                                    "Changed your mind?",
                                    "Well, if you ever need",
                                    "any bullets, I'll be right",
                                    "here. Come back whenever",
                                    "you think you'll need more",
                                    "ammunition, Gunslinger."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.lines(args![
                    "Hey, I'm Tony. I'm in",
                    "charge of distributing",
                    "and making bullets for",
                    "Gunslingers. It's just",
                    "how our guild likes",
                    "to do things."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Tony",
                    args![
                        "I'm sorry if you came",
                        "here to buy some bullets.",
                        "I can only do business with",
                        "fully fledged Gunslingers."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            BulletDealerTonyAlbStep::BulletTrade => {
                let required_id = runtime::arg(&args, 0, Val::from(0));
                let required_amount = runtime::arg(&args, 1, Val::from(0));
                let bullet_id = runtime::arg(&args, 2, Val::from(0));
                ctx.lines_as(
                    "Tony",
                    args![
                        "For every set of",
                        shared::other_global_functions::f_insertplural(
                            ctx,
                            args![30, ctx.call(Function::GetItemName, args![bullet_id.clone()])?]
                        )? + Val::from(","),
                        "you must give me",
                        "1 Emveretarcon,",
                        "1 Phracon, and",
                        shared::other_global_functions::f_insertplural(
                            ctx,
                            args![
                                required_amount.clone(),
                                ctx.call(Function::GetItemName, args![required_id.clone()])?
                            ]
                        )? + Val::from(".")
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tony",
                    args![
                        "Remember that I can give",
                        "a maximum of 500 sets of",
                        "30 bullets at a time. Please",
                        "enter the number of bullet sets",
                        "that you'd like. If you want to",
                        "cancel, then just enter ''0.''"
                    ],
                )?;
                ctx.next()?;
                let (input, _) = runtime::input_number(ctx, None, None)?;
                ctx.mes("[Tony]")?;
                let amount = input.number()?;
                if amount < 1 || amount > 500 {
                    ctx.lines(args![
                        "Hey, I can't give you",
                        "that many bullets. Don't",
                        "forget to enter a number",
                        "that's no higher than 500",
                        "if you want to trade your",
                        "items for some bullets."
                    ])?;
                } else {
                    if ctx.items().count(1010)? >= amount
                        && ctx.items().count(1011)? >= amount
                        && ctx.call(Function::CountItem, args![required_id.clone()])?.number()? >= amount * required_amount.number()?
                    {
                        if ctx.call(Function::CheckWeight, args![bullet_id.clone(), amount * 30])? == 0 {
                            ctx.lines(args![
                                "Eh? Your Inventory doesn't",
                                "have enough space for this",
                                "many bullets. Come back later",
                                "after you make more space",
                                "available. Try putting some of",
                                "your things into Kafra Storage."
                            ])?;
                        } else {
                            ctx.lines(args![
                                "Great, everything seems",
                                "to be in order. Let me take",
                                "these materials, and here are",
                                "your bullets. It's a pleasure",
                                "to do business with you~"
                            ])?;
                            ctx.call(Function::DelItem, args![1010, amount])?;
                            ctx.call(Function::DelItem, args![1011, amount])?;
                            ctx.call(Function::DelItem, args![required_id, amount * required_amount.number()?])?;
                            ctx.call(Function::GetItem, args![bullet_id, amount * 30])?;
                        }
                    } else {
                        ctx.lines(args![
                            "Huh. It looks like you",
                            "don't have enough materials",
                            "for that many bullets. Well,",
                            "it's no problem. Just come",
                            "back after gathering everything",
                            "that you need, okay?"
                        ])?;
                    }
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn bullet_dealer_tony_alb(ctx: &Ctx) -> Script {
    bullet_dealer_tony_alb_run(ctx, BulletDealerTonyAlbStep::Start, Vec::new()).map(|_| ())
}
