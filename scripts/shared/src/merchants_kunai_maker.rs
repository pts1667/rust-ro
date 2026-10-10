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

pub fn kunai_trade(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let item_req = vec![runtime::arg(&args, 0, Val::from(0)), runtime::arg(&args, 2, Val::from(0))];
    let req_amount = vec![runtime::arg(&args, 1, Val::from(0)), runtime::arg(&args, 3, Val::from(0))];
    let item_id = runtime::arg(&args, 4, Val::from(0));
    ctx.lines_as(
        "Kashin",
        args![
            "You can exchange",
            runtime::local_get(&req_amount, &Val::from(0), false)
                + Val::from(" ")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::local_get(&item_req, &Val::from(0), false)]
                )?
                + Val::from(" and"),
            runtime::local_get(&req_amount, &Val::from(1), false)
                + Val::from(" ")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::local_get(&item_req, &Val::from(1), false)]
                )?
                + Val::from(" for every"),
            Val::from("set of 10 ") + ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from("."),
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kashin",
        args![
            "I can only give you a maximum of 500 sets of Kunais at a time.",
            "If you want to cancel the trade, then please enter ''0.'' How many",
            "Kunai sets would you like?"
        ],
    )?;
    ctx.next()?;
    let (input, _) = runtime::input_number(ctx, None, None)?;
    let sets = input.number()?;
    if sets < 1 || sets > 500 {
        ctx.lines_as(
            "Kashin",
            args![
                "Eh? I'm sorry, but",
                "I can't give you that",
                "many Kunai sets. Please",
                "enter a value less than 500."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx
        .call(Function::CountItem, args![runtime::local_get(&item_req, &Val::from(0), false)])?
        .number()?
        < sets * runtime::local_get(&req_amount, &Val::from(0), false).number()?
        || ctx
            .call(Function::CountItem, args![runtime::local_get(&item_req, &Val::from(1), false)])?
            .number()?
            < sets * runtime::local_get(&req_amount, &Val::from(1), false).number()?
    {
        ctx.lines_as(
            "Kashin",
            args![
                "Hmm, you don't have",
                "enough items for this",
                "Kunai exchange. Please",
                "check your items again."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckWeight, args![item_id.clone(), sets * 10])? == 0 {
        ctx.lines_as(
            "Kashin",
            args![
                "Hmm, it seems like your",
                "Inventory doesn't have",
                "enough space to store",
                "more items. You better",
                "free up some space first."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Kashin",
        args![
            "Great, everything is in",
            "order, so let's go ahead",
            "and complete this trade.",
            "I'm sure that you'll be",
            "quite satisfied with",
            "these Kunais."
        ],
    )?;
    ctx.call(
        Function::DelItem,
        args![
            runtime::local_get(&item_req, &Val::from(0), false),
            runtime::local_get(&req_amount, &Val::from(0), false).number()? * sets,
        ],
    )?;
    ctx.call(
        Function::DelItem,
        args![
            runtime::local_get(&item_req, &Val::from(1), false),
            runtime::local_get(&req_amount, &Val::from(1), false).number()? * sets,
        ],
    )?;
    ctx.call(Function::GetItem, args![item_id, 10 * sets])?;
    ctx.close_window()?;
    Err(Stop::End)
}
