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

pub fn ice_cream_maker(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    ctx.lines_as(
        "Ice Cream Maker",
        args![
            "Fresh Ice Cream made with snow from Lutie!",
            "Enjoy it now, it won't be on sale for long!",
            "^3355FF100 Zeny^000000 Ice Cream,",
            "Ice Cream!"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Gimme Ice Cream!", "Cancel Trade"])? == 1 {
        ctx.lines(args![
            "Are you sure you don't want any?",
            "I won't be selling it for long,",
            "and once I run out,",
            "there won't be any more!!!"
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Ice Cream Maker",
        args![
            "Fresh Ice Cream made with snow from Lutie!",
            "Everyone wants our delicious ice cream, ",
            "but we have a limited amount,",
            "so you can only purchase 5 at a time!!"
        ],
    )?;
    ctx.next()?;
    loop {
        let (input, _) = runtime::input_number(ctx, None, None)?;
        l_input = input;
        if l_input.number()? < 1 {
            ctx.lines_as(
                "Ice Cream Maker",
                args![
                    "If you don't want to buy any,",
                    "could you please let the next customer",
                    "make a purchase?",
                    "Thank you."
                ],
            )?;
            return ctx.close();
        }
        if l_input.number()? > 5 {
            ctx.lines_as(
                "Ice Cream Maker",
                args![
                    "Ouch",
                    "You expect too much.",
                    "Dear customer,",
                    "If you eat more than 5 Ice creams,",
                    "If you might haveto make a lot of trips",
                    "to the bathroom tonight."
                ],
            )?;
            ctx.next()?;
            continue;
        }
        break;
    }
    if ctx.player().zeny()? < l_input.number()? * 100 {
        ctx.lines_as(
            "Ice Cream Maker",
            args![
                "Dear customer, your wallet seems to be light.",
                "Price is ^3355FF100 Zeny^000000 per ice cream."
            ],
        )?;
        return ctx.close();
    }
    if ctx.call(Function::CheckWeight, args![536, l_input.clone()])? == 0 {
        ctx.lines_as(
            "Ice Cream Maker",
            args![
                "Dear customer,you look like you're carrying a lot.",
                "Ice Cream is fine,",
                "but you must consider your weight",
                "before making a purchase."
            ],
        )?;
        return ctx.close();
    }
    ctx.player().set_zeny(ctx.player().zeny()? - 100 * l_input.number()?)?;
    ctx.call(Function::GetItem, args![536, l_input.clone()])?;
    ctx.close()
}
