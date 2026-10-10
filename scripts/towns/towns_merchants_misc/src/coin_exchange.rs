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

pub fn merchant_13_2(ctx: &Ctx) -> Script {
    let npc_tag = Val::from("[") + ctx.call(Function::StrNpcInfo, args![1])? + Val::from("]");
    ctx.lines(args![npc_tag.clone()])?;
    if ctx.call(Function::IsEquipped, args![2782])?.is_true() && ctx.var("ep13_2_rhea").get()? == 100 {
        if !ctx.call(Function::CheckWeight, args![1201, 1])?.is_true() {
            ctx.lines(args![
                "It looks like you're carrying too many things.",
                "Why not put some of your items in storage and come back?"
            ])?;
            return ctx.close();
        }
        ctx.lines(args!["Hello.", "What are you looking for?"])?;
        ctx.next()?;
        let mut items: Vec<Val> = Vec::new();
        let book;
        let coin;
        let choice;
        if ctx.call(Function::StrNpcInfo, args![1])? == "Merchant of Manuk" {
            runtime::local_set(&mut items, &Val::from(2), Val::from(12342), false);
            runtime::local_set(&mut items, &Val::from(3), Val::from(12343), false);
            runtime::local_set(&mut items, &Val::from(4), Val::from(12348), false);
            book = Val::from(11019);
            coin = Val::from(6080);
            choice = runtime::select_values(
                ctx,
                &[Val::from(
                    "View item description:Manuk's Opportunity:Manuk's Courage:Manuk's Faith:Cancel",
                )],
            )?;
        } else {
            runtime::local_set(&mut items, &Val::from(2), Val::from(12344), false);
            runtime::local_set(&mut items, &Val::from(3), Val::from(12345), false);
            runtime::local_set(&mut items, &Val::from(4), Val::from(12349), false);
            book = Val::from(11018);
            coin = Val::from(6081);
            choice = runtime::select_values(
                ctx,
                &[Val::from(
                    "View item description:Buy Pinguicula's Fruit Jam:Buy Luciola's Honey Jam:Buy Cornus' Tears:Do nothing",
                )],
            )?;
        }
        ctx.lines(args![npc_tag.clone()])?;
        if choice == 1 {
            ctx.mes("Here are the item descriptions.")?;
            ctx.close_window()?;
            ctx.call(Function::ReadBook, args![book, 1])?;
            return ctx.end();
        } else if choice < 5 {
            ctx.lines(args![
                Val::from("I can sell you 3 ")
                    + ctx.call(
                        Function::GetItemName,
                        args![runtime::local_get(&items, &Val::from(choice), false)]
                    )?
                    + Val::from(" for ^3131FF1 coin^000000.")
            ])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Purchase:Do not purchase")])? {
                1 => {
                    if ctx.call(Function::CountItem, args![coin.clone()])?.is_true() {
                        ctx.lines(args![npc_tag.clone(), "Thank you for coming."])?;
                        ctx.call(Function::DelItem, args![coin, 1])?;
                        ctx.call(
                            Function::GetItem,
                            args![runtime::local_get(&items, &Val::from(choice), false), 3],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines(args![npc_tag.clone(), "You don't have enough coins."])?;
                        return ctx.close();
                    }
                }
                2 => ctx.lines(args![npc_tag.clone()])?,
                _ => {}
            }
        }
        ctx.mes("Come again if you change your mind.")?;
        return ctx.close();
    } else {
        if ctx.call(Function::StrNpcInfo, args![1])? == "Merchant of Manuk" {
            ctx.lines(args!["Rtt od d", "Qwo hd is d irr"])?;
        } else {
            ctx.lines(args!["BurWehAla", "tasnarAndu Ie Ru"])?;
        }
        return ctx.close();
    }
}
