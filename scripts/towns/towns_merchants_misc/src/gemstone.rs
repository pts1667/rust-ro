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
enum JadePayStep {
    Start,
    TradeGems,
}

fn jade_pay_run(ctx: &Ctx, mut step: JadePayStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_gems = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_item_id = Val::from(0);
    let mut l_item_req = Val::from(0);
    'machine: loop {
        match step {
            JadePayStep::Start => {
                if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
                    ctx.mes("^3355FFWait a second! Right now, you're carrying too many items with you. Please come back after putting some of your things into Kafra Storage.^000000")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as(
                    "Jade",
                    args![
                        "Bring me two",
                        "Gemstones of the",
                        "same color, and I will",
                        "change them to Gemstones",
                        "of a different color."
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    let subject1 = ctx.menu(&[
                        "Blue Gemstones into Red ones!",
                        "Red Gemstones into Yellow ones!",
                        "Yellow Gemstones into Blue ones!",
                    ])?;
                    let mut matched1 = false;
                    if !matched1 && subject1 == 0 {
                        matched1 = true;
                    }
                    if matched1 {
                        jade_pay_run(ctx, JadePayStep::TradeGems, args![717, 716])?;
                    }
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        jade_pay_run(ctx, JadePayStep::TradeGems, args![716, 715])?;
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        jade_pay_run(ctx, JadePayStep::TradeGems, args![715, 717])?;
                    }
                }
                step = JadePayStep::TradeGems;
                continue 'machine;
            }
            JadePayStep::TradeGems => {
                l_item_req = runtime::arg(&args, 0, Val::from(0));
                l_item_id = runtime::arg(&args, 1, Val::from(0));
                if ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? < 2 {
                    ctx.lines_as(
                        "Jade",
                        args![
                            "Hah...!",
                            "You're kidding me, right?",
                            "I can't provide you with this",
                            "service if you don't",
                            "give me at least",
                            Val::from("2 ") + ctx.call(Function::GetItemName, args![l_item_req.clone()])? + Val::from("s!")
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                l_gems = Val::from(ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? / 2);
                ctx.lines_as(
                    "Jade",
                    args![
                        "I believe I can create",
                        Val::from("a total of ")
                            + l_gems.clone()
                            + Val::from(" ")
                            + ctx.call(Function::GetItemName, args![l_item_id.clone()])?
                            + Val::from("s"),
                        Val::from("using the ") + ctx.call(Function::GetItemName, args![l_item_req.clone()])? + Val::from("s"),
                        "that you currently have.",
                        "What do you want to do?"
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&["Give me as many as you can.", "I want to set the amount.", "I quit."])? {
                    0 => {
                        ctx.call(Function::DelItem, args![l_item_req.clone(), l_gems.number()? * 2])?;
                        ctx.call(Function::GetItem, args![l_item_id.clone(), l_gems.clone()])?;
                    }
                    1 => {
                        ctx.lines_as(
                            "Jade",
                            args!["So how many", "do you want?", "The maximum number", "that you can enter is 100."],
                        )?;
                        ctx.next()?;
                        loop {
                            let (input, _) = runtime::input_number(ctx, Some(0), Some(101))?;
                            l_input = input;
                            if l_input == 0 {
                                ctx.lines_as("Jade", args!["None at all?", "I guess you", "changed your mind..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if l_input.number()? > 100 {
                                ctx.lines_as(
                                    "Jade",
                                    args!["Errm...", "I asked you to enter", "an amount no greater", "than 100, remember...?"],
                                )?;
                                ctx.next()?;
                            } else if l_gems.number()? < l_input.number()? {
                                ctx.lines_as("Jade", args!["Errm...", "You don't have that", "many gems to trade..."])?;
                                ctx.next()?;
                            } else {
                                break;
                            }
                        }
                        ctx.call(Function::DelItem, args![l_item_req.clone(), l_input.number()? * 2])?;
                        ctx.call(Function::GetItem, args![l_item_id.clone(), l_input.clone()])?;
                    }
                    2 => {
                        ctx.lines_as("Jade", args!["Sure, no problem.", "Come back any time."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Jade",
                    args![
                        "There you go.",
                        "Feel free to come",
                        "back any time.",
                        "Hm, what's that look for?",
                        "Is there something on my face?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn jade_pay(ctx: &Ctx) -> Script {
    jade_pay_run(ctx, JadePayStep::Start, Vec::new()).map(|_| ())
}
