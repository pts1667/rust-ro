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

pub fn merchant_pron(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 6301
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines_as(
            "Merchant",
            args![
                "Haha!",
                "What are you, superhuman?",
                "You're carrying so much stuff!",
                "You better put some of that",
                "into Kafra Storage~"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Good day!",
            "Have you collected any",
            "items like Shells or Fluff?",
            "What about Jellopies? Oh yeah,",
            "I need those for something."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant",
        args![
            "Of course, I won't ask you",
            "to give me that stuff for free. What about if I trade a Red Potion for 5 Shells, 10 Fluff,",
            "or 10 Jellopies?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Sure!", "I think it's a rip-off!", "No, thanks."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["So which item do", "you want to bring me?", "Shells, Fluffs, or Jellopies?"],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["Shells", "Jellopies", "Fluff", "Cancel"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![935, 5])?;
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![909, 10])?;
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![914, 10])?;
                }
                if !matched2 && subject2 == 3 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Merchant", args!["Sure~", "No problem."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "A rip-off...?",
                    "If you check the market value",
                    "of the items being traded, I'm actually the one getting",
                    "ripped off here."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["Alright,", "no problem.", "But come back to me", "if you change your mind."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn merchant_morocc(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 6301
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines_as(
            "Merchant",
            args![
                "Haha!",
                "What are you, superhuman?",
                "You're carrying so much stuff!",
                "You better put some of that",
                "into Kafra Storage~"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Good day!",
            "Have you collected any",
            "Shells or Feathers of Bird?",
            "What about Jellopies? Oh yeah,",
            "I need those for something."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant",
        args![
            "Of course, I won't ask you",
            "to give me that stuff for free. What about if I trade a Red Potion for 5 Shells, 7 Feathers of Bird",
            "or 10 Jellopies?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Sure!", "I think it's a rip-off!", "No, thanks."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "So which item do",
                    "you want to bring me?",
                    "Shells, Feathers of Birds,",
                    "or Jellopies?"
                ],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["Shells", "Feathers of Birds", "Jellopies", "Cancel"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![935, 5])?;
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![916, 7])?;
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![909, 10])?;
                }
                if !matched2 && subject2 == 3 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Merchant", args!["Sure~", "No problem."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "A rip-off...?",
                    "If you check the market value",
                    "of the items being traded, I'm actually the one getting",
                    "ripped off here."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["Alright,", "no problem.", "But come back to me", "if you change your mind."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn merchant_pay(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 6301
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines_as(
            "Merchant",
            args![
                "Haha!",
                "What are you, superhuman?",
                "You're carrying so much stuff!",
                "You better put some of that",
                "into Kafra Storage~"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Good day!",
            "Have you collected",
            "items like Tree Roots?",
            "What about Jellopies? Oh yeah,",
            "I need those for something."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant",
        args![
            "Of course, I won't ask you",
            "to give me that stuff for free. What about if I trade a Red Potion for 6 Tree Roots or 10 Jellopies?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Sure!", "I think it's a rip-off!", "No, thanks."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["So which item do", "you want to bring me?", "Tree Roots, or Jellopies?"],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["Tree Roots", "Jellopies", "Cancel"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![902, 6])?;
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![909, 10])?;
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Merchant", args!["Sure~", "No problem."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "A rip-off...?",
                    "If you check the market value",
                    "of the items being traded, I'm actually the one getting",
                    "ripped off here."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["Alright,", "no problem.", "But come back to me", "if you change your mind."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn merchant_alde(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 6301
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines_as(
            "Merchant",
            args![
                "Haha!",
                "What are you, superhuman?",
                "You're carrying so much stuff!",
                "You better put some of that",
                "into Kafra Storage~"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Good day!",
            "Have you collected any",
            "Worm Peelings or Feather of Birds?",
            "How about Jellopies? Oh yeah,",
            "I need those for something."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant",
        args![
            "Of course, I won't ask you",
            "to give me that stuff for free. What about if I trade a Red Potion for 1 Worm Peeling, 7 Feather of Birds, or 10 Jellopies?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Sure!", "I think it's a rip-off!", "No, thanks."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "So which item do",
                    "you want to bring me?",
                    "Feathers of Birds?",
                    "Worm Peelings?",
                    "Or Jellopies?"
                ],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["Worm Peelings", "Feathers of Birds", "Jellopies", "Cancel"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![955, 1])?;
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![916, 7])?;
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![909, 10])?;
                }
                if !matched2 && subject2 == 3 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Merchant", args!["Sure~", "No problem."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "A rip-off...?",
                    "If you check the market value",
                    "of the items being traded, I'm actually the one getting",
                    "ripped off here."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["Alright,", "no problem.", "But come back to me", "if you change your mind."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn merchant_geff(ctx: &Ctx) -> Script {
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 6301
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines_as(
            "Merchant",
            args![
                "Haha!",
                "What are you, superhuman?",
                "You're carrying so much stuff!",
                "You better put some of that",
                "into Kafra Storage~"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Merchant",
        args![
            "Good day!",
            "Have you collected any",
            "items like Shells or Chrysalises?",
            "What about Jellopies? Oh yeah,",
            "I need those for something."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Merchant",
        args![
            "Of course, I won't ask you",
            "to give me that stuff for free. What about if I trade a Red Potion for 5 Shells or 6 Chrysalises, or 10 Jellopies?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Sure!", "I think it's a rip-off!", "No, thanks."])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["So which item do", "you want to bring me?", "Shells, Chrysalises, or Jellopies?"],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["Shells", "Chrysalises", "Jellopies", "Cancel"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![935, 5])?;
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![915, 6])?;
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    shared::merchants_novice_exchange::f_potexchange(ctx, args![909, 10])?;
                }
                if !matched2 && subject2 == 3 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Merchant", args!["Sure~", "No problem."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args![
                    "A rip-off...?",
                    "If you check the market value",
                    "of the items being traded, I'm actually the one getting",
                    "ripped off here."
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Merchant",
                args!["Alright,", "no problem.", "But come back to me", "if you change your mind."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}
