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
enum InventorJaaxStep {
    Start,
    SBuyQuiver,
}

fn inventor_jaax_run(ctx: &Ctx, mut step: InventorJaaxStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            InventorJaaxStep::Start => {
                if ctx.call(Function::CheckWeight, args![1201, 1])? == 0
                    || ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 2000
                {
                    ctx.lines_as(
                        "Inventor Jaax",
                        args![
                            "Hey, you're carrying",
                            "way too much stuff. Why don't you stash it away in Kafra Storage? We can talk after you do that, right?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Inventor Jaax",
                    args![
                        "My name is Jaax.",
                        "Without ego, I can",
                        "say that I am perhaps the",
                        "^663300greatest inventor of our time^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Inventor Jaax",
                    args![
                        "This time, I've",
                        "created something",
                        "truly extraordinary. I call them... ^663300Magic Quivers^000000 !! This will be remembered in history as an",
                        "arrow revolution!"
                    ],
                )?;
                ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                ctx.next()?;
                ctx.lines_as("Inventor Jaax", args!["I've studied magic and quivers for years, working night and day until I finally figured how to condense arrows with magic! With magic quivers, you'll be carrying more arrows, but with less weight!"])?;
                ctx.next()?;
                ctx.lines_as("Inventor Jaax", args!["Would you like to try using one of my arrow quivers? I have no doubt that someone like you can appreciate my genius!"])?;
                ctx.next()?;
                'b1: {
                    let subject1 = ctx.menu(&[
                        "Quiver",
                        "Iron Arrow Quiver",
                        "Steel Arrow Quiver",
                        "Oridecon Arrow Quiver",
                        "Fire Arrow Quiver",
                        "Silver Arrow Quiver",
                        "Wind Arrow Quiver",
                        "Stone Arrow Quiver",
                        "Crystal Arrow Quiver",
                        "Shadow Arrow Quiver",
                        "Immaterial Arrow Quiver",
                        "Rusty Arrow Quiver",
                    ])?;
                    let mut matched1 = false;
                    if !matched1 && subject1 == 0 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1750, 500, 500, 12004])?;
                    }
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1770, 500, 500, 12005])?;
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1753, 500, 500, 12006])?;
                    }
                    if !matched1 && subject1 == 3 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1765, 500, 500, 12007])?;
                    }
                    if !matched1 && subject1 == 4 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1752, 500, 500, 12008])?;
                    }
                    if !matched1 && subject1 == 5 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1751, 500, 500, 12009])?;
                    }
                    if !matched1 && subject1 == 6 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1755, 500, 500, 12010])?;
                    }
                    if !matched1 && subject1 == 7 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1756, 500, 500, 12011])?;
                    }
                    if !matched1 && subject1 == 8 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1754, 500, 500, 12012])?;
                    }
                    if !matched1 && subject1 == 9 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1767, 500, 500, 12013])?;
                    }
                    if !matched1 && subject1 == 10 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1757, 500, 500, 12014])?;
                    }
                    if !matched1 && subject1 == 11 {
                        matched1 = true;
                    }
                    if matched1 {
                        inventor_jaax_run(ctx, InventorJaaxStep::SBuyQuiver, args![1762, 500, 500, 12015])?;
                    }
                }
                step = InventorJaaxStep::SBuyQuiver;
                continue 'machine;
            }
            InventorJaaxStep::SBuyQuiver => {
                let item_id = runtime::arg(&args, 0, Val::from(0));
                let amount = runtime::arg(&args, 1, Val::from(0));
                let zeny_req = runtime::arg(&args, 2, Val::from(0));
                if runtime::op(&ctx.call(Function::CountItem, args![item_id.clone()])?, ">=", &amount)?.is_true() {
                    ctx.lines_as(
                        "Inventor Jaax",
                        args![
                            "Excellent!",
                            Val::from("Are you carrying any Arrows with you? I'll provide you with a quiver that can carry ")
                                + amount.clone()
                                + Val::from(" of your ")
                                + ctx.call(Function::GetItemName, args![item_id.clone()])?
                                + Val::from("s for only ^FF3131")
                                + zeny_req.clone()
                                + Val::from(" Zeny^000000.")
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Store as many Arrows in quivers as possible", "Purchase 1 quiver", "Cancel"])? {
                        0 => {
                            let arrows = ctx.call(Function::CountItem, args![item_id.clone()])?;
                            let quiver = arrows.clone().try_div(amount.clone())?;
                            let arrows_used = quiver.clone().try_mul(amount.clone())?;
                            let arrow_zeny = quiver.clone().try_mul(zeny_req.clone())?;
                            ctx.lines(args![
                                "Number of",
                                Val::from("Arrows: ^3131FF") + arrows + Val::from(" ^000000"),
                                "Maximum Number",
                                "of Purchasable",
                                Val::from("Quivers: ^3131FF") + quiver.clone() + Val::from(" ^000000"),
                                Val::from("Zeny required: ^3131FF") + arrow_zeny.clone() + Val::from(" Zeny^000000")
                            ])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Inventor Jaax",
                                args![
                                    "Would you like to",
                                    "buy as many Quivers",
                                    "as you can for the Arrows",
                                    "you are currently carrying?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Yes", "Cancel"])? == 0 {
                                if runtime::op(&arrow_zeny, "<", &ctx.var("Zeny").get()?)?.is_true() {
                                    ctx.lines_as("Inventor Jaax", args!["There you go!", "Just remember, ^FF0000you won't be able to use the Quiver when your carried weight is 90% of your maximum weight limit^000000."])?;
                                    ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(arrow_zeny)?)?;
                                    ctx.call(Function::DelItem, args![item_id.clone(), arrows_used])?;
                                    ctx.call(Function::GetItem, args![runtime::arg(&args, 3, Val::from(0)), quiver])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Inventor Jaax",
                                        args![
                                            "So...",
                                            "Just keep track",
                                            "of how much you're",
                                            "carrying from time",
                                            "to time and you should",
                                            "be alright."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as(
                                        "Inventor Jaax",
                                        args![
                                            "I'm sorry, but you don't have enough Zeny. I can't just give these away after working years",
                                            "to develop this revolutionary technology!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            ctx.lines_as(
                                "Inventor Jaax",
                                args!["You changed your mind?", "When the glory of owning", "a quiver is so close?"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        1 => {
                            if runtime::op(&ctx.var("Zeny").get()?, ">", &zeny_req)?.is_true() {
                                ctx.lines_as("Inventor Jaax", args!["There you go!", "Just remember, ^FF0000you won't be able to use the Quiver when your carried weight is 90% of your maximum weight limit^000000."])?;
                                ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(zeny_req.clone())?)?;
                                ctx.call(Function::DelItem, args![item_id.clone(), amount.clone()])?;
                                ctx.call(Function::GetItem, args![runtime::arg(&args, 3, Val::from(0)), 1])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Inventor Jaax",
                                    args![
                                        "So...",
                                        "Just keep track",
                                        "of how much you're",
                                        "carrying from time",
                                        "to time and you should",
                                        "be alright."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Inventor Jaax",
                                    args![
                                        "You don't even",
                                        Val::from("have ") + zeny_req.clone() + Val::from(" Zeny?"),
                                        "I'm so sorry. I had no",
                                        "idea that you were so...",
                                        "^333333Destitute^000000."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        2 => {
                            ctx.lines_as(
                                "Inventor Jaax",
                                args![
                                    "What...?",
                                    "Do you not see that this invention can forever change the way Arrows are carried?! The future is now!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                } else {
                    ctx.lines_as(
                        "Inventor Jaax",
                        args![
                            Val::from("You can carry a maximum of ")
                                + amount.clone()
                                + Val::from(" Arrows within this quiver. It was made using my secret method,"),
                            "so the total weight of the Arrows and Quiver is less than carrying the Arrows alone."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Inventor Jaax",
                        args![
                            Val::from("It's a miracle of science! One that you can experience for yourself if you bring me at least ")
                                + amount.clone()
                                + Val::from(" Arrows and ")
                                + zeny_req.clone()
                                + Val::from(" Zeny for each Quiver.")
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn inventor_jaax(ctx: &Ctx) -> Script {
    inventor_jaax_run(ctx, InventorJaaxStep::Start, Vec::new()).map(|_| ())
}
