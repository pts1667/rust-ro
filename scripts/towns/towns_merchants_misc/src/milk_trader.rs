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

pub fn milk_vendor(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "^3355FFJust a minute!",
            "I can't offer any of my",
            "services to you because",
            "you're carrying too much",
            "stuff. Put your extra items in",
            "Kafra Storage and come again~"
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Milk Vendor",
        args![
            "Hey, hey...",
            "If you bring me",
            "1 Empty Bottle and",
            "15 Zeny, I'll exchange",
            "them for 1 Milk. How",
            "does that sound?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Exchange all empty bottles.", "Cancel"])? == 0 {
        if ctx.items().count(713)? <= 0 {
            ctx.lines_as(
                "Milk Vendor",
                args![
                    "Hey...",
                    "You don't have",
                    "any Empty Bottles.",
                    "I can't really give you",
                    "this milk any other",
                    "way, you know..."
                ],
            )?;
            return ctx.close();
        }
        let bottles = ctx.call(Function::CountItem, args![713])?.number()?;
        let total_weight = bottles * 50;
        let total_cost = bottles * 15;
        if ctx.player().zeny()? < total_cost {
            ctx.lines_as(
                "Milk Vendor",
                args![
                    "Oh, whoa~!",
                    "You don't have enough",
                    "zeny to exchange all",
                    "these Empty Bottles for",
                    "Milk. You need to have",
                    Val::from("at least ") + total_cost + " zeny."
                ],
            )?;
            return ctx.close();
        }
        if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < total_weight {
            ctx.lines_as(
                "Milk Vendor",
                args![
                    "Hmm...",
                    "Would you make",
                    "a little more room",
                    "in your inventory",
                    "before I give you",
                    "all of this milk?"
                ],
            )?;
            return ctx.close();
        }
        ctx.player().set_zeny(ctx.player().zeny()? - total_cost)?;
        ctx.call(Function::DelItem, args![713, bottles])?;
        ctx.call(Function::GetItem, args![519, bottles])?;
        return ctx.close();
    }
    ctx.close()
}
