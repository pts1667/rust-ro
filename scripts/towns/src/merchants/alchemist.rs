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
enum GuildDealerStep {
    Start,
    SSellManual,
}

fn guild_dealer_run(ctx: &Ctx, step: GuildDealerStep, args: Vec<Val>) -> Result<Val, Stop> {
    match step {
        GuildDealerStep::Start => {
            if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
                ctx.lines(args![
                    "- Wait a minute! -",
                    "- Currently you are carrying -",
                    "- too many items with you. -",
                    "- Please come back again -",
                    "- after you store some items into kafra storage. -"
                ])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Gever Al Sharp",
                args!["Welcome to the", "Alchemist Union.", "How can I assist you today?"],
            )?;
            ctx.next()?;
            match ctx.menu(&["Purchase materials.", "Purchase a production manual.", "Quit."])? {
                0 => {
                    ctx.lines_as("Gever Al Sharp", args!["What would you like?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Medicine Bowl - 8 Zeny", "Cancel."])? == 0 {
                        ctx.lines_as(
                            "Gever Al Sharp",
                            args!["How many do you want?", "Enter '0' if you want to quit."],
                        )?;
                        ctx.next()?;
                        let amount = loop {
                            let (input, _) = runtime::input_number(ctx, Some(0), Some(2001))?;
                            if input == 0 {
                                ctx.lines_as("Gever Al Sharp", args!["The deal was cancelled.", "Come again next time."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            let typed = input.number()?;
                            if (1..=2000).contains(&typed) {
                                break typed;
                            }
                            ctx.lines_as("Gever Al Sharp", args!["The number must", "be less than 2000."])?;
                            ctx.next()?;
                        };
                        let sell = amount * 8;
                        let weight = amount * 10;
                        if ctx.player().zeny()? < sell {
                            ctx.lines_as(
                                "Gever Al Sharp",
                                args!["You don't", "have enough zeny.", "Check how much zeny", "you have first."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < weight {
                            ctx.lines_as(
                                "Gever Al Sharp",
                                args![
                                    "It doesn't seem like",
                                    "you can carry everything.",
                                    "Please check the space",
                                    "in your inventory."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.player().set_zeny(ctx.player().zeny()? - sell)?;
                        ctx.call(Function::GetItem, args![7134, amount])?;
                        ctx.lines_as("Gever Al Sharp", args!["Thank you.", "Come again."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Gever Al Sharp",
                        args![
                            "Well then,",
                            "come again when",
                            "you need to purchase",
                            "materials related to",
                            "Alchemy, alright?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                1 => {
                    ctx.lines_as(
                        "Gever Al Sharp",
                        args![
                            "What do you need?",
                            "Manuals are generally 100,000 zeny. But there are a couple of special manuals that will cost more."
                        ],
                    )?;
                    ctx.next()?;
                    let (item_id, price): (i32, i32) = match ctx.menu(&[
                        "Potion Creation Guide",
                        "Alcohol Creation Guide",
                        "Bottle Grenade Creation Guide",
                        "Acid Bottle Creation Guide",
                        "Plant Bottle Creation Guide",
                        "Marine Sphere Bottle Creation Guide",
                        "Glistening Coat Creation Guide",
                        "Condensed Potion Creation Guide",
                        "Cancel Deal.",
                    ])? {
                        0 => (7144, 100000),
                        1 => (7127, 100000),
                        2 => (7128, 100000),
                        3 => (7129, 100000),
                        4 => (7130, 100000),
                        5 => (7131, 100000),
                        6 => (7132, 100000),
                        7 => (7133, 240000),
                        _ => {
                            ctx.lines_as(
                                "Gever Al Sharp",
                                args!["Well then...", "Come back if you", "ever need to buy", "a production manual."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    };
                    return guild_dealer_run(ctx, GuildDealerStep::SSellManual, args![item_id, price]);
                }
                _ => {
                    ctx.lines_as("Gever Al Sharp", args!["Alright then,", "have a good day."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        GuildDealerStep::SSellManual => {
            let item_id = runtime::arg(&args, 0, Val::from(0));
            let zeny_req = runtime::arg(&args, 1, Val::from(0));
            if runtime::op(&ctx.var("Zeny").get()?, "<", &zeny_req)?.is_true() {
                ctx.lines_as(
                    "Gever Al Sharp",
                    args!["You don't", "have enough zeny.", "Check how much zeny", "you have first."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Gever Al Sharp",
                args![
                    ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from("?"),
                    Val::from("That'll be ") + zeny_req.clone() + Val::from(" zeny.")
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Purchase.", "Quit."])? == 0 {
                ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(zeny_req)?)?;
                ctx.call(Function::GetItem, args![item_id, 1])?;
                ctx.lines_as("Gever Al Sharp", args!["Thank you for", "your patronage."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as("Gever Al Sharp", args!["Come again", "next time."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
}

pub fn guild_dealer(ctx: &Ctx) -> Script {
    guild_dealer_run(ctx, GuildDealerStep::Start, Vec::new()).map(|_| ())
}
