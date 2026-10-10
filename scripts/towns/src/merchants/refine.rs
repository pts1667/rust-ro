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

pub fn christopher_1(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    let mut l_sell = Val::from(0);
    ctx.lines_as(
        "Christopher Guillenrow",
        args![
            "Welcome to Christopher's Workshop. Ye can get all yer stuff for forging here. What business",
            "brings ye to me?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Purchase Anvil:Purchase Forging Item:Purchase Metal:Purify Rough Ores:Cancel",
            )],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4))
            && !subject1.loosely_equals(&Val::from(5));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Christopher Guillenrow", args!["A better Anvil gives ye a greeeater chance to make better weapons, ye know? But they'll cost ye more zeny. Just get it off yer chest and buy what fits your purposes best, laddy."])?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Anvil - 30,000 zeny:Oridecon Anvil - 120,000 zeny:Golden Anvil - 300,000 zeny:Better Anvil than the others.:Cancel.",
                    )],
                )?);
                let mut matched2 = false;
                let no_case2 = !subject2.loosely_equals(&Val::from(1))
                    && !subject2.loosely_equals(&Val::from(2))
                    && !subject2.loosely_equals(&Val::from(3))
                    && !subject2.loosely_equals(&Val::from(4))
                    && !subject2.loosely_equals(&Val::from(5));
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.player().zeny()? < 30000 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(986, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 30000)?;
                    ctx.lines_as("Christopher Guillenrow", args!["This is the cheapest one, but efficient enough to forge most items. Thank ye fer shopping at me workshop.  Feel free to come anytime, whenever ye need."])?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.player().zeny()? < 120000 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(987, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 120000)?;
                    ctx.lines_as("Christopher Guillenrow", args!["Aye, friend ye have an eye for the anvil. This must be the proper anvil for a Blacksmith, eh? Thank ye fer shopping at me workshop.  Feel free to come anytime, whenever ye need."])?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.player().zeny()? < 300000 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(988, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 300000)?;
                    ctx.lines_as("Christopher Guillenrow", args!["This one is the best among all me stuffs in me workshop! With this, ye can rule the Blacksmith world! Thank ye fer shopping at me workshop.  Feel free to come anytime, whenever ye need."])?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Well, sorry. But I don't have anythin' harder' than the Golden Anvil."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Christopher Guillenrow", args!["Me thinks 'Ringgel,' the Legendary Anvil Maker would have one. But, I don't think ye can find him, though he be somewhere in this world."])?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Okay, feel free to come anytime, whenever ye need. Fare ye well."],
                    )?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Christopher Guillenrow",
                args!["A respectable blacksmith uses fine tools. Ye can become one o'those with me Stuff. Choose anything ye want."],
            )?;
            ctx.next()?;
            'b3: {
                let subject3 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Mini-Furnace - 150 zeny:Iron Hammer - 1000 zeny:Golden Hammer - 3000 zeny:Oridecon Hammer - 5000 zeny:Cancel.",
                    )],
                )?);
                let mut matched3 = false;
                let no_case3 = !subject3.loosely_equals(&Val::from(1))
                    && !subject3.loosely_equals(&Val::from(2))
                    && !subject3.loosely_equals(&Val::from(3))
                    && !subject3.loosely_equals(&Val::from(4))
                    && !subject3.loosely_equals(&Val::from(5));
                if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                    matched3 = true;
                }
                if matched3 {
                    ctx.lines_as("Christopher Guillenrow", args!["It's a much needed tool fer refining metal! So, How many do ye wish to buy? If ye want to quit, just type the number '0.'"])?;
                    ctx.next()?;
                    'l4: loop {
                        'b4: {
                            let (input, status) = runtime::input_number(ctx, None, None)?;
                            l_input = input;
                            if l_input == 0 {
                                ctx.lines_as("Christopher Guillenrow", args!["Aye, the deal is canceled. Fare ye well."])?;
                                return ctx.close();
                            } else {
                                if l_input.clone().number()? < 0 || l_input.clone().number()? > 500 {
                                    ctx.lines_as("Christopher Guillenrow", args!["Ye can buy 500, er less."])?;
                                    ctx.next()?;
                                } else {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    l_sell = l_input.clone().try_mul(Val::from(150))?;
                    if runtime::op(&ctx.var("Zeny").get()?, "<", &l_sell.clone())?.is_true() {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    if ctx.call(Function::CheckWeight, args![612, l_input.clone()])? == 0 {
                        ctx.lines_as("Christopher Guillenrow", args!["Ye look like you don't got enough room in yer inventory. Put some stuff into your Kafra Storage, why don't ye?"])?;
                        return ctx.close();
                    }
                    ctx.call(Function::GetItem, args![612, l_input.clone()])?;
                    ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_sell.clone())?)?;
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Thank ye fer shopping at me workshop. Feel free to come anytime, whenever ye need."],
                    )?;
                    return ctx.close();
                }
                if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                    matched3 = true;
                }
                if matched3 {
                    if ctx.player().zeny()? < 1000 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(613, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Thank ye fer shopping at me workshop. Feel free to come anytime, whenever ye need."],
                    )?;
                    return ctx.close();
                }
                if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                    matched3 = true;
                }
                if matched3 {
                    if ctx.player().zeny()? < 3000 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(614, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 3000)?;
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Thank ye fer shopping at me workshop. Feel free to come anytime, whenever ye need."],
                    )?;
                    return ctx.close();
                }
                if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                    matched3 = true;
                }
                if matched3 {
                    if ctx.player().zeny()? < 5000 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(615, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Thank ye fer shopping at me workshop. Feel free to come anytime, whenever ye need."],
                    )?;
                    return ctx.close();
                }
                if !matched3 && subject3.loosely_equals(&Val::from(5)) {
                    matched3 = true;
                }
                if matched3 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Feel free to come anytime, whenever ye need. Fare ye well."],
                    )?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Christopher Guillenrow",
                args!["I prepare every Metal, and only the high quality ones o'course. Now then, which one do ye need?"],
            )?;
            ctx.next()?;
            'b5: {
                let subject5 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Phracon - 200z.:Emveretarcon - 1000z.:Cancel.")],
                )?);
                let mut matched5 = false;
                let no_case5 = !subject5.loosely_equals(&Val::from(1))
                    && !subject5.loosely_equals(&Val::from(2))
                    && !subject5.loosely_equals(&Val::from(3));
                if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                    matched5 = true;
                }
                if matched5 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["So, How many do ye wish to buy? If ye dont want anything, just type the number as '0.'"],
                    )?;
                    ctx.next()?;
                    'l6: loop {
                        'b6: {
                            let (input, status) = runtime::input_number(ctx, None, None)?;
                            l_input = input;
                            if l_input == 0 {
                                ctx.lines_as("Christopher Guillenrow", args!["Deal has", "been canceled.", "Fare ye well."])?;
                                return ctx.close();
                            } else {
                                if l_input.clone().number()? < 0 || l_input.clone().number()? > 500 {
                                    ctx.lines_as("Christopher Guillenrow", args!["Ye can buy 500, er less."])?;
                                    ctx.next()?;
                                } else {
                                    break 'l6;
                                }
                            }
                        }
                    }
                    l_sell = l_input.clone().try_mul(Val::from(200))?;
                    if runtime::op(&ctx.var("Zeny").get()?, "<", &l_sell.clone())?.is_true() {
                        ctx.lines_as("Christopher Guillenrow", args!["Ye don't have enough money. Ye know I can't sell this at a lower price... You know how the wifey nags about Zeny."])?;
                        return ctx.close();
                    }
                    if ctx.call(Function::CheckWeight, args![1010, l_input.clone()])? == 0 {
                        ctx.lines_as("Christopher Guillenrow", args!["Ye look like you don't have the room to carry anythin' new. Why don't ye put some things into Kafra Storage n' come back."])?;
                        return ctx.close();
                    }
                    ctx.call(Function::GetItem, args![1010, l_input.clone()])?;
                    ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_sell.clone())?)?;
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Thank ye fer shopping at me workshop. Feel free to come anytime, whenever ye need."],
                    )?;
                    return ctx.close();
                }
                if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                    matched5 = true;
                }
                if matched5 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["So, how many do ye wish to buy? If ye dont want anything at all, just type the number as '0.'"],
                    )?;
                    ctx.next()?;
                    'l7: loop {
                        'b7: {
                            let (input, status) = runtime::input_number(ctx, None, None)?;
                            l_input = input;
                            if l_input == 0 {
                                ctx.lines_as("Christopher Guillenrow", args!["Deal has", "been canceled.", "Fare ye well."])?;
                                return ctx.close();
                            } else {
                                if l_input.clone().number()? < 0 || l_input.clone().number()? > 500 {
                                    ctx.lines_as("Christopher Guillenrow", args!["Ye can buy 500, er less."])?;
                                    ctx.next()?;
                                } else {
                                    break 'l7;
                                }
                            }
                        }
                    }
                    l_sell = l_input.clone().try_mul(Val::from(1000))?;
                    if runtime::op(&ctx.var("Zeny").get()?, "<", &l_sell.clone())?.is_true() {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I don't think I can let ye have this with the zeny ye have. I can't lose me money because of ye."],
                        )?;
                        return ctx.close();
                    }
                    if ctx.call(Function::CheckWeight, args![1011, l_input.clone()])? == 0 {
                        ctx.lines_as("Christopher Guillenrow", args!["Me friend... Seems to me ye don't have Inventory space. Why doncha put some things into Kafra Storage first?"])?;
                        return ctx.close();
                    }
                    ctx.call(Function::GetItem, args![1011, l_input.clone()])?;
                    ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_sell.clone())?)?;
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Thank ye fer shopping at me workshop. Feel free to come anytime, whenever ye need, whenever ye want."],
                    )?;
                    return ctx.close();
                }
                if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                    matched5 = true;
                }
                if matched5 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Feel free to come anytime, whenever ye need. Fare ye well."],
                    )?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Christopher Guillenrow", args!["I can purify yer Oridecon and Elunium. I make a refined Ore out of 5 o'each rough ones. Well... Which one do ye want to make?"])?;
            ctx.next()?;
            'b8: {
                let subject8 = Val::from(runtime::select_values(ctx, &[Val::from("Make Oridecon:Make Elunium:Cancel.")])?);
                let mut matched8 = false;
                let no_case8 = !subject8.loosely_equals(&Val::from(1))
                    && !subject8.loosely_equals(&Val::from(2))
                    && !subject8.loosely_equals(&Val::from(3));
                if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                    matched8 = true;
                }
                if matched8 {
                    if ctx.items().count(756)? < 5 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I told ye, I need 5 o'the rough Oridecons fer one Oridecon."],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.items().take(756, 5)?;
                        ctx.items().give(984, 1)?;
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["Here's an Oridecon fer ye. Ye will be always welcome here, I'll be waitin' for ye."],
                        )?;
                        return ctx.close();
                    }
                }
                if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                    matched8 = true;
                }
                if matched8 {
                    if ctx.items().count(757)? < 5 {
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["I told ye, I need 5 rough Eluniums fer one Elunium."],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.items().take(757, 5)?;
                        ctx.items().give(985, 1)?;
                        ctx.lines_as(
                            "Christopher Guillenrow",
                            args!["Arrr, here's yer Elunium. Yer business is always welcome here, so feel free to come again."],
                        )?;
                        return ctx.close();
                    }
                }
                if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                    matched8 = true;
                }
                if matched8 {
                    ctx.lines_as(
                        "Christopher Guillenrow",
                        args!["Feel free to come anytime, whenever ye need. Fare ye well."],
                    )?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(5)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Christopher Guillenrow",
                args!["Feel free to come anytime, whenever ye need and whenever ye want. Fare ye well."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn paul_spanner(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    let mut l_item = Val::from(0);
    let mut l_item_cost = Val::from(0);
    let mut l_item_price = Val::from(0);
    let mut l_item_weight = Val::from(0);
    let mut l_sell = Val::from(0);
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Paul Spanner",
        args![
            "Welcome, my friend.",
            "In my shop, you will find everything that you need in forging.",
            "Tell me what you need."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Purchase Anvil.:Purchase Forging Items.:Purchase Metal.:Process Ores.:Quit.",
            )],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4))
            && !subject1.loosely_equals(&Val::from(5));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Paul Spanner",
                args![
                    "Anvil is the most necessary item for Blacksmiths.",
                    "Since you will use an Anvil more than once, you'd better buy a nice one."
                ],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Anvil - 30,000z.:Oridecon Anvil - 120,000z.:Golden Anvil - 300,000z.:I need a better anvil.:Cancel.",
                    )],
                )?);
                let mut matched2 = false;
                let no_case2 = !subject2.loosely_equals(&Val::from(1))
                    && !subject2.loosely_equals(&Val::from(2))
                    && !subject2.loosely_equals(&Val::from(3))
                    && !subject2.loosely_equals(&Val::from(4))
                    && !subject2.loosely_equals(&Val::from(5));
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.player().zeny()? < 30000 {
                        ctx.lines_as(
                            "Paul Spanner",
                            args!["With that much of money, you cannot even buy a toy anvil!"],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(986, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 30000)?;
                    ctx.lines_as(
                        "Paul Spanner",
                        args![
                            "It is the cheapest anvil which has the most basic ability.",
                            "Thank you for using my shop. If you need anything, just let me know."
                        ],
                    )?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.player().zeny()? < 120000 {
                        ctx.lines_as(
                            "Paul Spanner",
                            args!["With that much of money, you cannot even buy a toy anvil!"],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(987, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 120000)?;
                    ctx.lines_as(
                        "Paul Spanner",
                        args![
                            "Ah, you have an eye for anvil. A Blacksmith needs an anvil at least as good as this.",
                            "Thank you for using my shop. If you need anything, just let me know."
                        ],
                    )?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.player().zeny()? < 300000 {
                        ctx.lines_as(
                            "Paul Spanner",
                            args!["With that much of money, you cannot even buy a toy anvil!"],
                        )?;
                        return ctx.close();
                    }
                    ctx.items().give(988, 1)?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 300000)?;
                    ctx.lines_as(
                        "Paul Spanner",
                        args![
                            "I can tell your ambition to become a good Blacksmith just by looking at you to choose this Golden Anvil!",
                            "This anvil will surely aid you in creating the best weapons."
                        ],
                    )?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Paul Spanner", args!["I am sorry, but I do not sell better anvils than Golden Anvil.", "Unless you find the legendary anvil of 'Linggell', I don't think that you could find better one than Golden Anvil in any other places."])?;
                    return ctx.close();
                }
                if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Paul Spanner", args!["If you need anything, just let me know."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Paul Spanner",
                args![
                    "You need various materials to process ores and to forge weapons.",
                    "I have everything that you need. Take a look."
                ],
            )?;
            ctx.next()?;
            'b3: {
                let subject3 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Mini Furnace - 150z.:Iron Hammer - 1,000z.:Golden Hammer - 3,000z.:Oridecon Hammer - 5,000z.:Cancel.",
                    )],
                )?);
                let mut matched3 = false;
                let no_case3 = !subject3.loosely_equals(&Val::from(1))
                    && !subject3.loosely_equals(&Val::from(2))
                    && !subject3.loosely_equals(&Val::from(3))
                    && !subject3.loosely_equals(&Val::from(4))
                    && !subject3.loosely_equals(&Val::from(5));
                if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                    matched3 = true;
                }
                if matched3 {
                    l_item = Val::from(612);
                    l_item_cost = Val::from(150);
                    l_item_weight = Val::from(200);
                    ctx.lines_as("Paul Spanner", args!["You definitely need this furnace to process ores!"])?;
                    ctx.next()?;
                    break 'b3;
                }
                if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                    matched3 = true;
                }
                if matched3 {
                    l_item = Val::from(613);
                    l_item_cost = Val::from(1000);
                    l_item_weight = Val::from(200);
                    break 'b3;
                }
                if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                    matched3 = true;
                }
                if matched3 {
                    l_item = Val::from(614);
                    l_item_cost = Val::from(3000);
                    l_item_weight = Val::from(300);
                    break 'b3;
                }
                if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                    matched3 = true;
                }
                if matched3 {
                    l_item = Val::from(615);
                    l_item_cost = Val::from(5000);
                    l_item_weight = Val::from(400);
                    break 'b3;
                }
                if !matched3 && subject3.loosely_equals(&Val::from(5)) {
                    matched3 = true;
                }
                if matched3 {
                    ctx.lines_as("Paul Spanner", args!["If you need anything, just let me know."])?;
                    return ctx.close();
                }
            }
            ctx.lines_as(
                "Paul Spanner",
                args!["So, how many do you need? If you want to cancel the trade, enter '0'."],
            )?;
            ctx.next()?;
            'l4: loop {
                'b4: {
                    let (input, status) = runtime::input_number(ctx, None, None)?;
                    l_input = input;
                    if l_input == 0 {
                        ctx.lines_as(
                            "Paul Spanner",
                            args!["You have canceled the trade. If you need anything, just let me know."],
                        )?;
                        return ctx.close();
                    } else {
                        if l_input.clone().number()? < 0 || l_input.clone().number()? > 500 {
                            ctx.lines_as("Paul Spanner", args!["You can only buy 500 or less at a time."])?;
                            ctx.next()?;
                        } else {
                            break 'l4;
                        }
                    }
                }
            }
            l_sell = l_input.clone().try_mul(l_item_cost.clone())?;
            if runtime::op(&ctx.var("Zeny").get()?, "<", &l_sell.clone())?.is_true() {
                ctx.lines_as(
                    "Paul Spanner",
                    args!["You don't have enough money. Sorry, I cannot sell them at a loss."],
                )?;
                return ctx.close();
            }
            if ctx.call(Function::CheckWeight, args![l_item.clone(), l_input.clone()])? == 0 {
                ctx.lines_as(
                    "Paul Spanner",
                    args!["Hey, you look pale. Why don't you go lighten your weight first."],
                )?;
                return ctx.close();
            }
            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_sell.clone())?)?;
            ctx.call(Function::GetItem, args![l_item.clone(), l_input.clone()])?;
            ctx.lines_as(
                "Paul Spanner",
                args!["Thank you for using my shop. If you need anything, just let me know."],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Paul Spanner",
                args!["I have high quality metal.", "So, which metal would you like to buy?"],
            )?;
            ctx.next()?;
            'b5: {
                let subject5 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Phracon - 200z.:Emveretarcon - 1,000z.:Quit.")],
                )?);
                let mut matched5 = false;
                let no_case5 = !subject5.loosely_equals(&Val::from(1))
                    && !subject5.loosely_equals(&Val::from(2))
                    && !subject5.loosely_equals(&Val::from(3));
                if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                    matched5 = true;
                }
                if matched5 {
                    l_item = Val::from(1010);
                    l_item_price = Val::from(200);
                    break 'b5;
                }
                if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                    matched5 = true;
                }
                if matched5 {
                    l_item = Val::from(1011);
                    l_item_price = Val::from(1000);
                    break 'b5;
                }
                if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                    matched5 = true;
                }
                if matched5 {
                    ctx.lines_as("Paul Spanner", args!["If you need anything, just let me know."])?;
                    return ctx.close();
                }
            }
            ctx.lines_as(
                "Paul Spanner",
                args!["So, how many of them do you need? If you want to cancel the trade, enter '0'."],
            )?;
            ctx.next()?;
            'l6: loop {
                'b6: {
                    let (input, status) = runtime::input_number(ctx, None, None)?;
                    l_input = input;
                    if l_input == 0 {
                        ctx.lines_as(
                            "Paul Spanner",
                            args!["The trade has been canceled. If you need anything, just let me know."],
                        )?;
                        return ctx.close();
                    } else {
                        if l_input.clone().number()? < 0 || l_input.clone().number()? > 500 {
                            ctx.lines_as("Paul Spanner", args!["You can buy 500 or less at a time."])?;
                            ctx.next()?;
                        } else {
                            break 'l6;
                        }
                    }
                }
            }
            l_sell = l_input.clone().try_mul(l_item_price.clone())?;
            if runtime::op(&ctx.var("Zeny").get()?, "<", &l_sell.clone())?.is_true() {
                ctx.lines_as(
                    "Paul Spanner",
                    args!["You don't have enough money. Sorry, I cannot sell them at a loss."],
                )?;
                return ctx.close();
            }
            if ctx.call(Function::CheckWeight, args![l_item.clone(), l_input.clone()])? == 0 {
                ctx.lines_as(
                    "Paul Spanner",
                    args!["Hey, you look pale. Why don't you go lighten your weight first?"],
                )?;
                return ctx.close();
            }
            ctx.call(Function::GetItem, args![l_item.clone(), l_input.clone()])?;
            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_sell.clone())?)?;
            ctx.lines_as(
                "Paul Spanner",
                args!["Thank you for using my shop. If you need anything, just let me know."],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Paul Spanner",
                args![
                    "I can process Oridecon and Elunium for you.",
                    "You need 5 ores to process them into one Oridecon or Elunium.",
                    "So, which one do you want to process?"
                ],
            )?;
            'b7: {
                let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("Oridecon:Elunium:Quit.")])?);
                let mut matched7 = false;
                let no_case7 = !subject7.loosely_equals(&Val::from(1))
                    && !subject7.loosely_equals(&Val::from(2))
                    && !subject7.loosely_equals(&Val::from(3));
                if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                    matched7 = true;
                }
                if matched7 {
                    if ctx.items().count(756)? < 5 {
                        ctx.lines_as("Paul Spanner", args!["You need 5 ores to process them into one pure Oridecon."])?;
                        return ctx.close();
                    } else {
                        ctx.items().take(756, 5)?;
                        ctx.items().give(984, 1)?;
                        ctx.lines_as("Paul Spanner", args!["There you go. Thank you for using my service."])?;
                        return ctx.close();
                    }
                }
                if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                    matched7 = true;
                }
                if matched7 {
                    if ctx.items().count(757)? < 5 {
                        ctx.lines_as("Paul Spanner", args!["You need 5 ores to process them into one pure Elunium."])?;
                        return ctx.close();
                    } else {
                        ctx.items().take(757, 5)?;
                        ctx.items().give(985, 1)?;
                        ctx.lines_as("Paul Spanner", args!["There you go. Thank you for using my service."])?;
                        return ctx.close();
                    }
                }
                if !matched7 && subject7.loosely_equals(&Val::from(3)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as("Paul Spanner", args!["If you need anything, just let me know."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(5)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Paul Spanner", args!["If you need anything, just let me know."])?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn hollgrehenn(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as(
            "Hollgrehenn",
            args![
                "I'm a blacksmith who can refine weapons and equipment.",
                "Do you have any items that you'd like to refine?"
            ],
        )?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Hollgrehenn", 0])?;
    }
    ctx.end()
}

pub fn aragham(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as(
            "Aragham",
            args![
                "De~ sert~ Blacksmith~",
                "Aragham Ssaleh~ who can refine anything Ssaleh~",
                "Let me see what you'd like to refine~"
            ],
        )?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Aragham", 0])?;
    }
    ctx.end()
}

pub fn antonio(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as("Antonio", args!["Stop nagging and get me the equipment you want to refine."])?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Antonio", 0])?;
    }
    ctx.end()
}

pub fn fredrik(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as(
            "Fredrik Hermanthorn",
            args![
                "I am a blacksmith who can refine your weapon or equipment.",
                "Do you have anything that you'd like to refine?"
            ],
        )?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Fredrik", 0])?;
    }
    ctx.end()
}

pub fn lambert(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as("Lambert", args!["Wel...come... This is... the best... smithy... in Juno."])?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Lambert", 0])?;
    }
    ctx.end()
}

pub fn manthasman(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as(
            "Manthasman Pruhag",
            args![
                "Hahaha, you already knew it. So you'd like to refine something?",
                "Let's try something risky today!"
            ],
        )?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Manthasman Pruhag", 0])?;
    }
    ctx.end()
}

pub fn fulerr(ctx: &Ctx) -> Script {
    if ctx.call(Function::GetBattleFlag, args!["feature.refineui"])?.is_true() {
        ctx.lines_as(
            "Fulerr",
            args!["Hehe... You wanna refine?", "Hehe.. Consider it done..", "Hehe...."],
        )?;
        ctx.close_window()?;
    } else {
        shared::merchants_refine::refinemain(ctx, args!["Fulerr", 0])?;
    }
    ctx.end()
}

pub fn vurewell(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Vurewell"])?;
    ctx.end()
}

pub fn begnahd(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Begnahd"])?;
    ctx.end()
}

pub fn sade(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Sade"])?;
    ctx.end()
}

pub fn kahlamanlith(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Kahlamanlith"])?;
    ctx.end()
}

pub fn dilemma(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Dilemma"])?;
    ctx.end()
}

pub fn tirehaus(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Tirehaus"])?;
    ctx.end()
}

pub fn krugg(ctx: &Ctx) -> Script {
    shared::merchants_refine::phramain(ctx, args!["Krugg"])?;
    ctx.end()
}

pub fn dietrich(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Dietrich"])?;
    ctx.end()
}

pub fn hakhim(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Hakhim"])?;
    ctx.end()
}

pub fn abdula(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Abdula"])?;
    ctx.end()
}

pub fn xenophon(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Xenophon Zolotas"])?;
    ctx.end()
}

pub fn delight(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Delight"])?;
    ctx.end()
}

pub fn matestein(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Matestein"])?;
    ctx.end()
}

pub fn fruel(ctx: &Ctx) -> Script {
    shared::merchants_refine::orimain(ctx, args!["Fruel"])?;
    ctx.end()
}

pub fn repairman_alb(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_moc(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_pay(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_prt(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Grendal"])?;
    ctx.end()
}

pub fn repairman_juno(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_gef(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_alde(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_lhz(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_prt_gld(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_gef_fild(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_pay_gld(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_alde_gld(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_aru_gld(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}

pub fn repairman_sch_gld(ctx: &Ctx) -> Script {
    shared::merchants_refine::repairmain(ctx, args!["Repairman"])?;
    ctx.end()
}
