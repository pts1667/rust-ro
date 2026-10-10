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
enum KafraEmployeeBunnyStep {
    Start,
    EventInfo,
}

fn first_missing_item_text(ctx: &Ctx, required: &[Val]) -> Result<Option<Val>, Stop> {
    for index in (0..8_i32).step_by(2) {
        let item = runtime::local_get(required, &Val::from(index), false);
        let amount = runtime::local_get(required, &Val::from(index + 1), false);
        if ctx.call(Function::CountItem, args![item.clone()])?.number()? < amount.number()? {
            let text = shared::other_global_functions::f_insertplural(ctx, args![amount, ctx.call(Function::GetItemName, args![item])?])?;
            return Ok(Some(text));
        }
    }
    Ok(None)
}

fn kafra_employee_bunny_run(ctx: &Ctx, mut step: KafraEmployeeBunnyStep) -> Script {
    'machine: loop {
        match step {
            KafraEmployeeBunnyStep::Start => {
                if ctx.var("bunybnd").get()? == 1 {
                    ctx.lines_as(
                        "Kafra Employee",
                        args![
                            "Hello there~!",
                            "How'd you like to",
                            "participate in Kafra",
                            "Corporation's special",
                            "^529DFFBunny Band Event^000000?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Sure, I brought the items.", "Event Information", "Cancel"])? {
                        0 => {
                            ctx.lines_as(
                                "Kafra Employee",
                                args!["Alrighty~", "Let me check to", "see if you brought", "all the items..."],
                            )?;
                            ctx.next()?;
                            // flat (item, amount) pairs
                            let mut required: Vec<Val> = Vec::new();
                            runtime::local_set(&mut required, &Val::from(0), Val::from(949), false);
                            runtime::local_set(&mut required, &Val::from(1), Val::from(100), false);
                            runtime::local_set(&mut required, &Val::from(2), Val::from(706), false);
                            runtime::local_set(&mut required, &Val::from(3), Val::from(1), false);
                            runtime::local_set(&mut required, &Val::from(4), Val::from(722), false);
                            runtime::local_set(&mut required, &Val::from(5), Val::from(1), false);
                            runtime::local_set(&mut required, &Val::from(6), Val::from(2213), false);
                            runtime::local_set(&mut required, &Val::from(7), Val::from(1), false);
                            if let Some(missing) = first_missing_item_text(ctx, &required)? {
                                ctx.lines_as(
                                    "Kafra Employee",
                                    args!["Ooh, I'm sorry", "but you need to", "bring at least", missing + Val::from(".")],
                                )?;
                                return ctx.close();
                            }
                            ctx.lines_as(
                                "Kafra Employee",
                                args![
                                    "Great, I see that",
                                    "you've gathered",
                                    "everything I need to",
                                    "make the Bunny Band.",
                                    "Please wait a moment",
                                    "while I put it together..."
                                ],
                            )?;
                            ctx.next()?;
                            if let Some(missing) = first_missing_item_text(ctx, &required)? {
                                ctx.lines_as(
                                    "Kafra Employee",
                                    args![
                                        "Hm? I'm sorry,",
                                        "but I actually can't",
                                        "make this right now. You",
                                        Val::from("need ") + missing,
                                        "in order for me to put this",
                                        "Bunny Band together..."
                                    ],
                                )?;
                                return ctx.close();
                            }
                            ctx.items().take(949, 100)?;
                            ctx.items().take(706, 1)?;
                            ctx.items().take(722, 1)?;
                            ctx.items().take(2213, 1)?;
                            ctx.lines_as(
                                "Kafra Employee",
                                args!["Ah, here you go~", "The perfect Bunny Band!", "Well, I hope you enjoy it."],
                            )?;
                            ctx.items().give(2214, 1)?;
                            ctx.var("bunybnd").set(Val::from(0))?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Kafra Employee",
                                args![
                                    "Thank you for",
                                    "participating in this",
                                    "special event and your",
                                    "continued patronage",
                                    "of the Kafra Services~"
                                ],
                            )?;
                            return ctx.close();
                        }
                        1 => {
                            step = KafraEmployeeBunnyStep::EventInfo;
                            continue 'machine;
                        }
                        2 => return ctx.close(),
                        _ => {}
                    }
                }
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "Hi there~! Would you like",
                        "to join our special ^529DFFBunny",
                        "Band Event^000000 hosted by the Kafra",
                        "Corporation and sponsored by",
                        "the Alberta Merchant Guild?"
                    ],
                )?;
                ctx.next()?;
                match ctx.menu(&["Join the Event", "Event Information", "Cancel"])? {
                    0 => {
                        ctx.var("bunybnd").set(Val::from(1))?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "Great! Thanks for",
                                "participating! If you",
                                "haven't already heard,",
                                "you need to collect these",
                                "items if you want me put a",
                                "Bunny Band together for you..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "100 Feathers,",
                                "1 Four-Leaf Clover,",
                                "1 Kitty Band and",
                                "1 Pearl. That's it!",
                                "Good luck and I'll be",
                                "waiting for you here~"
                            ],
                        )?;
                        return ctx.close();
                    }
                    2 => return ctx.close(),
                    _ => {}
                }
                step = KafraEmployeeBunnyStep::EventInfo;
                continue 'machine;
            }
            KafraEmployeeBunnyStep::EventInfo => {
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "To thank our valued",
                        "customers, Kafra Corporation",
                        "has prepared a special event",
                        "where Kafra Employee will assemble",
                        "Bunny Bands for adventurers",
                        "who bring the required items."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "For this special,",
                        "one of a kind item,",
                        "bring 100 Feathers,",
                        "1 Four-Leaf Clover,",
                        "1 Kitty Band and",
                        "1 Pearl."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "When you're ready,",
                        "come back and bring",
                        "those items to me.",
                        "(Sponsored by the",
                        "Alberta Merchant Guild)."
                    ],
                )?;
                return ctx.close();
            }
        }
    }
}

pub fn kafra_employee_bunny(ctx: &Ctx) -> Script {
    kafra_employee_bunny_run(ctx, KafraEmployeeBunnyStep::Start)
}
