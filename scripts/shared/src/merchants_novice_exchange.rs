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

pub fn f_potexchange(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let item_req = runtime::arg(&args, 0, Val::from(0));
    let req_amount = runtime::arg(&args, 1, Val::from(0));
    if runtime::op(&ctx.call(Function::CountItem, args![item_req.clone()])?, "<", &req_amount)?.is_true() {
        ctx.lines_as(
            "Merchant",
            args![
                "Hey, where are",
                Val::from("all those ") + ctx.call(Function::GetItemName, args![item_req.clone()])?,
                "that you promised?",
                Val::from("Give me ") + ctx.call(Function::GetItemName, args![item_req.clone()])? + "!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Okay, let me check",
            Val::from("how many ") + ctx.call(Function::GetItemName, args![item_req.clone()])? + " you",
            "have on you. Hmm..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant",
        args![
            "You have",
            Val::from("a total of ")
                + ctx.call(Function::CountItem, args![item_req.clone()])?
                + " "
                + ctx.call(Function::GetItemName, args![item_req.clone()])?
                + ".",
            "I can give you a total",
            Val::from("of ")
                + ctx
                    .call(Function::CountItem, args![item_req.clone()])?
                    .try_div(req_amount.clone())?
                + " Red Potions for those."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Merchant", args!["What do you say?", "Do we have a deal?"])?;
    ctx.next()?;
    if ctx.menu(&["Deal.", "No deal."])? == 0 {
        ctx.lines_as(
            "Merchant",
            args![
                "You know the exact",
                "number of Red Potions",
                "you want to receive for",
                Val::from("those ") + ctx.call(Function::GetItemName, args![item_req.clone()])? + ", don't you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Merchant",
            args![
                "Now, I can trade you",
                "a minimum of 1 Red Potion",
                "and a maximum of 100 Red Potions",
                "at one time. If you change your",
                "mind, just enter '0' to cancel."
            ],
        )?;
        ctx.next()?;
        let potions = loop {
            let (input, _) = runtime::input_number(ctx, Some(0), Some(101))?;
            let entered = input.number()?;
            if entered == 0 {
                ctx.lines_as(
                    "Merchant",
                    args![
                        "What...?",
                        "Why the hell do you",
                        "even bother to talk to me?",
                        "Pretty indecisive, aren't you?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            if entered <= 100 {
                break entered;
            }
            ctx.lines_as(
                "Merchant",
                args!["I can't give you more", "than 100 Red Potions", "at once. Let's try", "this again."],
            )?;
            ctx.next()?;
        };
        let items_due = Val::from(potions).try_mul(req_amount.clone())?;
        if runtime::op(&ctx.call(Function::CountItem, args![item_req.clone()])?, "<", &items_due)?.is_true() {
            ctx.lines_as("Merchant", args!["Uh oh, the number you entered doesn't seem right. You better check the number of Red Potions that you can trade for again."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Merchant",
            args![
                "There you go!",
                "Check how many Red Potions",
                "I've given you, it should be good. Thanks, that was a good deal~"
            ],
        )?;
        ctx.call(Function::DelItem, args![item_req, items_due])?;
        ctx.call(Function::GetItem, args![501, potions])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Huh~",
            "Alright.",
            "Though aren't",
            "Red Potions more useful",
            "to an adventurer like you?"
        ],
    )?;
    ctx.close_window()?;
    Err(Stop::End)
}
