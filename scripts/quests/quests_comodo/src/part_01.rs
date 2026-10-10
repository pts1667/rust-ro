use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn hair_ornament_girl_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("[Hair Ornament Girl]")?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.lines(args![
            "Oh, hello!",
            "Would you like to order a hairband? They're great presents to give to your girlfriend, and show that you're thinking of her."
        ])?;
    } else {
        ctx.lines(args![
            "Hello~!",
            "I think one of these will suit a pretty girl like you very well. Would you like to order a hairband?"
        ])?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Hair Ornament Girl",
        args![
            "If you give me a few simple items then I can make a gorgeous Hair Band. If you have any questions, go ahead and have a look."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Cross Hat:Bulb Band:Stripe Hairband:Blue Hairband")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            if (ctx.call(Function::CountItem, vec![Val::from(2608)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(7069)])?.number()? > 499)
            {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "You've chosen the Cross Hat?",
                        "If you brought all the required items, I will make it for you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "But before I can make it, can you check that the items are right?",
                        "If it is a forged, upgraded item or has monster cards in it, then those cards and upgrades will be lost."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "It's better to bring just to keep the requested items in your inventory. Did you check? Then let's make it, okay?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Yes.")])? {
                    1 => {
                        ctx.lines_as("Hair Ornament Girl", args!["Ah, alright. I guess you should put all your other things into Kafra Storage first. Okay then, see you soon.", "See you soon~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("^3355FFSwish swish snip snip^000000")?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(2608), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7069), Val::from(500)])?;
                        ctx.lines_as(
                            "Hair Ornament Girl",
                            args!["Look at this! Well done~", "Please come again~ hoooo!"],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(5036), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args!["Oh, you've chosen Cross Hat?", "There are some items needed to make this headband."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args!["I'll need...", "^3355FF1 Rosary^000000", "^3355FF500 Destroyed Armor^000000"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "If you bring all the items then I will make it for you right away.",
                        "See you then~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if (ctx.call(Function::CountItem, vec![Val::from(2233)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(746)])?.number()? > 19)
            {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "You've chosen Bulb band?",
                        "If you brought all the required items, I will make it for you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "But before I can make it, can you check that the items are right?",
                        "If it is a forged, upgraded item or has monster cards in it, then those cards and upgrades will be lost."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "It's better to bring just to keep the requested items in your inventory. Did you check? Then let's make it, okay?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Yes.")])? {
                    1 => {
                        ctx.lines_as("Hair Ornament Girl", args!["Ah, alright. I guess you should put all your other things into Kafra Storage first. Okay then, see you soon."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("^3355FFSwish swish snip snip^000000")?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(2233), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(746), Val::from(20)])?;
                        ctx.lines_as(
                            "Hair Ornament Girl",
                            args!["Look at this! Well done~", "Please come again~ hoooo!"],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(5034), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "Oh, you've chosen Bulb Band?",
                        "There are some required items needed to make this headband."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args!["I'll need...", "^3355FF1 Slotted Circlet^000000", "^3355FF20 Glass Bead^000000"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "If you bring all the items then I will make it for you right away.",
                        "See you then~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.call(Function::CountItem, vec![Val::from(1099)])?.number()? > 1499 {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "You've chosen Stripe band?",
                        "If you brought all the required items, I will make it for you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hair Ornament Girl", args!["You want me to make it now?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Yes.")])? {
                    1 => {
                        ctx.lines_as("Hair Ornament Girl", args!["See you later then~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("^3355FFSwish swish snip snip^000000")?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(1099), Val::from(1500)])?;
                        ctx.lines_as("Hair Ornament Girl", args!["Look at this! Well done~", "Please come again~"])?;
                        ctx.call(Function::GetItem, vec![Val::from(5049), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args!["Oh, you've chosen the Stripe band? There are some required items needed to make this headband."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args!["I'll need...", "^3355FF1500 Worn-out Prison Uniform^000000"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "If you bring all the items then I will make it for you right away.",
                        "See you then~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            if ((ctx.call(Function::CountItem, vec![Val::from(2211)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(978)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(7003)])?.number()? > 299)
            {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "You've chosen Blue Hairband?",
                        "If you brought all the required items, I will make it for you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "But before I can make it, can you check that the items are right?",
                        "If it is a forged, upgraded item or has monster cards in it, then those cards and upgrades will be lost."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "It's better to bring just to keep the requested items in your inventory. Did you check? Then let's make it, okay?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("No.:Yes.")])? {
                    1 => {
                        ctx.lines_as("Hair Ornament Girl", args!["Ah, alright. I guess you should put all your other things into Kafra Storage first. Okay then, see you soon."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.mes("^3355FFSwish swish snip snip^000000")?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(2211), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(978), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7003), Val::from(300)])?;
                        ctx.lines_as("Hair Ornament Girl", args!["Look, at this! Well done!", "Please come again~ "])?;
                        ctx.call(Function::GetItem, vec![Val::from(5052), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "Oh, you've chosen Blue Hairband?",
                        "There are some required items needed to make this headband."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "I'll need...",
                        "^3355FF1 Bandana^000000",
                        "^3355FF1 Cobaltblue Dyestuffs^000000",
                        "^3355FF300 Anolian Skin^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hair Ornament Girl",
                    args![
                        "If you bring all the items then I will make it for you right away.",
                        "See you then~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn hair_ornament_girl(ctx: &Ctx) -> Script {
    hair_ornament_girl_body(ctx, Vec::new()).map(|_| ())
}

fn traveler_head_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Isac Mari", args!["I travel around the world, and collect many interesting stories from people. Sometimes they teach me how to make special items."])?;
    ctx.next()?;
    ctx.lines_as(
        "Isac Mari",
        args!["I know how to make all sorts of worldly hats. But right now, I can't make these since I don't have the right items."],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Mine Helmet:Parcel Hat:Grief for Greed:Opera Phantom Mask")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            if (((ctx.call(Function::CountItem, vec![Val::from(5009)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(5028)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 24)
                && ctx.call(Function::CountItem, vec![Val::from(747)])?.number()? > 0)
            {
                ctx.lines_as("Isac Mari", args!["Mine Helmet was designed for the miners in the coal mines to have light in the dark. If you brought all the required items, I will go ahead and make it for you~"])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["But can you check that the items in your inventory aren't upgraded or have cards compounded into them? If any of the required items have forged upgrades or cards, those will be lost once I create the Mine Hat."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["It's better to keep only the items used to make the Mine Hat in your inventory. Did you check? If so, let's get started."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give me a minute~!:Let's make the Mine Hat.")])? {
                    1 => {
                        ctx.lines_as(
                            "Isac Mari",
                            args!["Alright. Please check your items again. It's much better to be safe than sorry."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args!["^3355FF*Thook Thook*", "*Pop!*^000000"])?;
                        ctx.next()?;
                        ctx.lines_as("Isac Mari", args!["Whew! This is pretty hard!"])?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(5009), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(5028), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(747), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(999), Val::from(25)])?;
                        ctx.lines_as(
                            "Isac Mari",
                            args!["I'm not quite sure if this is the right one, but it looks about right. Ha ha~"],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(5031), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as("Isac Mari", args!["I hear Mine Helmets are used in the mines near Geffen. As you know, it's not possible to work without light, even though it attracts monsters in the caves."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["One day, a miner remodeled his own Safety Helmet so that he could work with both hands. Since then, it's been called the Mine Helm."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "If you want to make Mine Helmet,",
                        "^3355FF1 Safety Helmet^000000",
                        "^3355FF1 Candle^000000 ",
                        "^3355FF1 Crystal Mirror^000000",
                        "^3355FF25 Steel^000000",
                        "is needed to create it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args!["If you bring all the items then I will make it for you right away. See you later~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ((ctx.call(Function::CountItem, vec![Val::from(1059)])?.number()? > 149
                && ctx.call(Function::CountItem, vec![Val::from(907)])?.number()? > 99)
                && ctx.call(Function::CountItem, vec![Val::from(978)])?.number()? > 0)
            {
                ctx.lines_as("Isac Mari", args!["Parcel Hat seems to be useful for business people to carry around their products. If you brought everything I need, I will make it for you.", "If you brought all the required items, I will make it for you~"])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["But can you check that the items in your inventory aren't upgraded or have cards compounded into them? If any of the required items have forged upgrades or cards, those will be lost once I create the Parcel Hat."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["It's better to keep only the items used to make the Parcel Hat in your inventory. Did you check? If so, let's get started."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give me a minute~!:Let's make the Parcel Hat.")])? {
                    1 => {
                        ctx.lines_as(
                            "Isac Mari",
                            args!["Alright. Please check your items again. It's much better to be safe than sorry."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args!["^3355FF*Thook Thook*", "*Pop!*^000000"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Isac Mari",
                            args!["Oh...", "I think I messed up!", "No, wait, I think I can fix this..."],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(1059), Val::from(150)])?;
                        ctx.call(Function::DelItem, vec![Val::from(907), Val::from(100)])?;
                        ctx.call(Function::DelItem, vec![Val::from(978), Val::from(1)])?;
                        ctx.lines_as(
                            "Isac Mari",
                            args!["Well...", "Um, it looks like a Parcel Hat. So that's good enough for me! Ha ha~"],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(5023), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Isac Mari",
                    args!["I heard that around Alberta, Merchants are always selling goods from their carts in the middle of the streets."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args!["But in the East side of the country, they use a different way to transfer goods."],
                )?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["Goods would be packed into a bundle and placed on top of the head so you wouldn't need to carry things by hand. But it's hard to keep your balance."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "I'll need...",
                        "^3355FF150 Fabric^000000",
                        "^3355FF100 Resin^000000 and",
                        "^3355FF1 Cobaltblue Dyestuffs^000000",
                        "to make a Parcel Hat."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args!["If you bring all the items then I will make it for you right away. See you later~!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            if ((((ctx.call(Function::CountItem, vec![Val::from(2233)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(969)])?.number()? > 0)
                && ctx.call(Function::CountItem, vec![Val::from(949)])?.number()? > 79)
                && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 19)
                && ctx.call(Function::CountItem, vec![Val::from(938)])?.number()? > 799)
            {
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "Grief for Greed...",
                        "If you brought all the required items, I'll go ahead and make it for you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["But can you check that the items in your inventory aren't upgraded or have cards compounded into them? If any of the required items have forged upgrades or cards, those will be lost once I create the Money Loser's Grief."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["It's better to keep only the items used to make the Grief for Greed in your inventory. Did you check? If so, let's get started."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give me a minute~!:Let's make the Grief for Greed.")])? {
                    1 => {
                        ctx.lines_as(
                            "Isac Mari",
                            args!["Alright. Please check your items again. It's much better to be safe than sorry."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args!["^3355FF*Thook Thook*", "*Pop!*^000000"])?;
                        ctx.next()?;
                        ctx.lines_as("Isac Mari", args!["Whew! This is pretty hard!"])?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(2233), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(969), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(949), Val::from(80)])?;
                        ctx.call(Function::DelItem, vec![Val::from(999), Val::from(20)])?;
                        ctx.call(Function::DelItem, vec![Val::from(938), Val::from(800)])?;
                        ctx.lines_as(
                            "Isac Mari",
                            args!["I might have made a few mistakes, but it looks good enough. Here you go, your own Grief for Greed hat!"],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(5021), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "Grief for Greed...",
                        "There's a tale of an honest Merchant that lived in Morocc long ago who had a quarrel with a bad guild."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["Because the honest Merchant was not cheating people by his prices, his street stall was attacked by the bad guild and all his goods were taken."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["So, the honest Merchant lost everything except some broken items. To express his sadness, he took these broken items and created this magnificent hat, the Grief for Greed."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["This special hat appeals to people's sympathies, making them buy more goods to support the inner goodness of honest Merchants."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "For this hat, I'll need...",
                        "^3355FF1 Slotted Circlet^000000",
                        "^3355FF1 Gold^000000",
                        "^3355FF20 Steel^000000",
                        "^3355FF80 Feather^000000",
                        "^3355FF800 Sticky Mucus^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args!["If you bring all the items then I will make it for you right away. See you later~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            if (ctx.call(Function::CountItem, vec![Val::from(2281)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(1048)])?.number()? > 49)
            {
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "Opera Phantom Mask...",
                        "Frightening, but also tragic and romantic. If you brought all the required items, I will make it for you~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["But can you check that the items in your inventory aren't upgraded or have cards compounded into them? If any of the required items have forged upgrades or cards, those will be lost once I create the Opera Masque."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["It's better to keep only the items used to make the Opera Mask in your inventory. Did you check? If so, let's get started."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Give me a minute~!:Let's make the Opera Masque.")])? {
                    1 => {
                        ctx.lines_as(
                            "Isac Mari",
                            args!["Alright. Please check your items again. It's much better to be safe than sorry."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines(args!["^3355FF*Thook Thook*", "*Pop!*^000000"])?;
                        ctx.next()?;
                        ctx.lines_as("Isac Mari", args!["Whew! This is pretty hard!"])?;
                        ctx.next()?;
                        ctx.call(Function::DelItem, vec![Val::from(2281), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(1048), Val::from(50)])?;
                        ctx.lines_as(
                            "Isac Mari",
                            args!["I'm not quite sure if this is the right one.", "But it looks fine. Ha ha"],
                        )?;
                        ctx.call(Function::GetItem, vec![Val::from(5043), Val::from(1)])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "Opera Phantom Mask...",
                        "Behind this mask is a story about a man's love and personal tragedy."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["You may know him as the Phantom of the Opera. He was a musical genius that brought terror to the stage, because his scarred face forced him to live outside of society."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["However, he fell in love with a beautiful singer and used his talents to propel her to new heights of fame. He finally revealed himself to her."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["In the end...", "She deeply cared for him, but was in love with another singer. The Phantom disappeared, perhaps to haunt some other place, and all that was left of him was his mask..."])?;
                ctx.next()?;
                ctx.lines_as("Isac Mari", args!["The mask that he was wearing is differently shaped that other opera masks, and was made to cover his particular facial scarring."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args![
                        "If you want to make Phantom of Opera. You will need:",
                        "^3355FF1 Opera Masque^000000",
                        "^3355FF50 Horrendous Hair^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Isac Mari",
                    args!["If you bring all the items then I will make it for you right away. See you later~"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
    Ok(Val::from(0))
}

pub fn traveler_head(ctx: &Ctx) -> Script {
    traveler_head_body(ctx, Vec::new()).map(|_| ())
}

fn campground_boy_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 0 {
        ctx.lines_as(
            "Rochito",
            args![
                "Bread, fruits, vegetables...",
                "Bleh. All that other food is",
                "nothing compared to the ",
                "hearty flavor of meat. Yeap,",
                "BBQ camping in Comodo is",
                "heaven to a meat lover like me~"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("BBQ Camping...?:Cancel")])?) == 1 {
            ctx.lines_as(
                "Rochito",
                args![
                    "Yeah man... You can eat",
                    "Komodoru meat all day when",
                    "you go BBQ camping. Komodoru",
                    "is an animal native to Comodo",
                    "and every part of it is delicious. Every. Single. Morsel."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rochito",
                args![
                    "What makes this meat even",
                    "more delicious is the special",
                    "Koserahserah seasoning they",
                    "use. That stuff is almost...",
                    "addictive. Without it, we",
                    "can't start our barbeque!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rochito",
                args![
                    "If we settled on anything",
                    "less than the very best BBQ,",
                    "then our comrades that died",
                    "to help ^FF0000banish that witch^000000 would",
                    "surely be ashamed of us!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rochito",
                args![
                    "Say, would you do us a favor?",
                    "The Chief of Comodo was going",
                    "to bring the Koserahserah and",
                    "join us for our barbeque, but",
                    "we're guessing he's got to",
                    "cancel because of his duties."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rochito",
                args![
                    "Let's see, our Chief lives to",
                    "the west of these campgrounds.",
                    "Would you visit him and see",
                    "what's taking him so long",
                    "to get over here?"
                ],
            )?;
            ctx.var("dmdswrd_q").set(Val::from(1))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Rochito",
            args![
                "You know, people gather",
                "on these campgrounds in",
                "memory and respect of those",
                "that have fallen in battle",
                "against the witch of Comodo."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "Long before Comodo was",
                "built inside this huge cave,",
                "this witch used to live in here. A lot of people died trying to",
                "get rid of her, but there're rumors that she's still around..."
            ],
        )?;
    } else if subject1 == 1 {
        ctx.lines_as(
            "Rochito",
            args![
                "Komodoru meat is especially",
                "great when it's seasoned with",
                "Koserahserah. That flavoring",
                "is one of Comodo's claims to",
                "fame! You should try some~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "Speaking of which, our Chief",
                "still hasn't come and brought",
                "the Koserahserah! Would you",
                "see what's taking him so long?",
                "His house is located west of",
                "these campgrounds."
            ],
        )?;
    } else if subject1 == 2 {
        ctx.lines_as(
            "Rochito",
            args![
                "Oh, hey, you're back.",
                "Did you speak to our",
                "Chief? Don't tell me",
                "he had to cancel--we've",
                "been planning this outing",
                "with him for a quite a while..."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou give Rochito the",
            "Koserahserah seasoning,",
            "and explain why the Chief",
            "cannot attend the barbeque.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "Awww, nuts. I guess he's",
                "got responsbilities, but it's",
                "still a little disappointing.",
                "He's a buddy, after all. Well,",
                "at least he was kind enough",
                "to send the Koserahserah."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "It's really too bad that",
                "he can't join us. Ah, I've got",
                "an idea! Would you please bring",
                "this bottle to Tausupa, er, our",
                "Chief? It's Mureuchieligu, a",
                "special vintage wine~"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou've received a bottle",
            "of Mureuchieligu wine to",
            "deliver to the Comodo Chief.^000000"
        ])?;
        ctx.var("dmdswrd_q").set(Val::from(3))?;
    } else if subject1 == 3 {
        ctx.lines_as(
            "Rochito",
            args![
                "I know that I should be",
                "delivering that bottle of",
                "wine to the Chief myself,",
                "but I've got to tend to this",
                "barbeque. I hope you ",
                "understand..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "Anyway, you'd be doing",
                "me a huge favor if you",
                "spoke to the Chief, and gave",
                "him my thanks, along with",
                "that bottle of Mureuchieligu."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Rochito",
            args![
                "Oh, hey there~",
                "Thanks for helping us",
                "out earlier. I wish the",
                "Chief would join us in our",
                "barbeque, but I understand",
                "that he has to protect Comodo."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "Hey, you know what?",
                "I can't exactly repay you",
                "with, you know, actual stuff,",
                "but I can give you a hot tip.",
                "There's some guy at the local",
                "Pub with some precious info."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rochito",
            args![
                "Yeah, supposedly, this",
                "guy knows more about the",
                "cave that Comodo was built",
                "in... Anyway, I really get the",
                "feeling that it just might",
                "lead to something, you know?"
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn campground_boy_cmd(ctx: &Ctx) -> Script {
    campground_boy_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn camping_youth_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 0 || subject1 == 1 {
        ctx.lines_as(
            "Rockha",
            args![
                "Oh man...",
                "I'm so excited!",
                "My buddies and I've",
                "been planning to get",
                "together for this barbeque",
                "for such a long time~"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Buddies...?:Cancel")])?) == 1 {
            ctx.lines_as(
                "Rockha",
                args![
                    "Yeah, some of us know",
                    "each other when we fought",
                    "together in the War of the",
                    "Witch. In fact, one of them",
                    "is the Chief of this village!",
                    "Huh, why isn't he here yet?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rockha",
                args![
                    "Anyway, even though",
                    "our stations in life have",
                    "changed, thankfully we're",
                    "still friends. It's kinda weird, though, being on a first name",
                    "basis with a village chief."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rockha",
                args![
                    "Makes me feel...",
                    "Important. I'm hobnobbing",
                    "with a major political figure,",
                    "after all. Amazing where your",
                    "friends can end up in life..."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Rockha",
            args![
                "I've almost forgotten",
                "how much I love hanging",
                "out with these guys. We",
                "should have barbeques",
                "together all the time~"
            ],
        )?;
    } else if subject1 == 2 {
        ctx.lines_as(
            "Rockha",
            args![
                "Wha--? Tausupa can't come,",
                "but he still sent us all of this Koserahserah? What a guy...",
                "I guess... That gives us",
                "a reason to hold another",
                "barbeque here soon, right?"
            ],
        )?;
    } else if subject1 == 3 {
        ctx.lines_as(
            "Rockha",
            args![
                "Hey, when you deliver",
                "that wine to Tausupa, the",
                "Village Chief, would you let",
                "him know that we miss the guy?",
                "He may be busy, but he'll always be our irreplaceable buddy."
            ],
        )?;
    } else if subject1 == 4 {
        ctx.lines_as(
            "Rockha",
            args![
                "Oh hey, you spoke",
                "to Tausupa? Ah, it's",
                "too bad that he's busy,",
                "but it's great to hear that",
                "he'll enjoy our gift. Okay~",
                "I believe it's time to eat!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rockha",
            args![
                "But before that, let's",
                "make a toast... to Tausupa!",
                "Guardian of Comodo, and",
                "one of the best friends that",
                "a guy can have! Cheers!",
                "Hahahaha hahahaha~!"
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(5))?;
        ctx.next()?;
        ctx.lines_as(
            "Rockha",
            args![
                "Ahh, you know what would",
                "make this meal perfect?",
                "Some of that legendary",
                "^3355FFComodo Cheese^000000 that I heard",
                "about from ^3355FFToruna^000000. Sure, it",
                "might not exist, but still...!"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Rockha",
            args![
                "You know, you're a really",
                "chill person. Thanks for",
                "helping us keep in touch",
                "with our old friend, the",
                "Village Chief. Man, being",
                "responsible must be rough..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rockha",
            args![
                "There's two things that",
                "would make our barbeque",
                "absolutely perfect--having",
                "the Village Chief here, and",
                "some of that legendary Comodo Cheese that ^3355FFToruna^000000 told me about."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn camping_youth_cmd(ctx: &Ctx) -> Script {
    camping_youth_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn camping_maiden_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Emralhandas",
        args![
            "Rockha, let me pour",
            "you another drink. We",
            "always dreamed of this",
            "during the War of the Witch...",
            "Having a good time, all of us",
            "together in a time of peace~"
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("War of the Witch?:Cancel")])?) == 1 {
        ctx.lines_as(
            "Emralhandas",
            args![
                "Before I tell you about the",
                "War of the Witch, I guess",
                "I should tell you about the",
                "quest to retrieve 4 rare swords",
                "of power, said to be the most",
                "powerful weapons ever made."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Emralhandas",
            args![
                "Now, all 4 swords were",
                "successfully found 10 years",
                "after the search began. Then,",
                "4 protectors were chosen to",
                "ensure that the swords did",
                "not fall into the wrong hands."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Emralhandas",
            args![
                "You might not think so just",
                "by looking at us, but we were",
                "the protectors--me, Rockha,",
                "Rochito, and Tausupa, the chief",
                "of this village. Now, we guarded these weapons very carefully..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Emralhandas",
            args![
                "However, that didn't deter",
                "Mariposum, ancient witch of",
                "Comodo, from attacking us and",
                "trying to steal the swords. She",
                "was incredibly powerful, and",
                "we needed an army to fight her."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Emralhandas",
            args![
                "That was the War of the",
                "Witch, basically. Mariposum",
                "with her own strange weapon",
                "of mass destruction against the",
                "four of us backed by a large",
                "force of courageous soldiers."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Emralhandas",
            args![
                "Many of our comrades died...",
                "But finally we succeeded in",
                "imprisoning the witch by using",
                "the power of the swords. Yeah,",
                "if it weren't for Tausupa, we'd",
                "never have been able to do it."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Emralhandas",
        args![
            "Rockha, we still have",
            "to finish our mission:",
            "eat all of the barbeque",
            "that that we possibly can!",
            "Let's just stuff ourselves...",
            "All the way until tommorrow!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn camping_maiden_cmd(ctx: &Ctx) -> Script {
    camping_maiden_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn campground_lad_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 0 || subject1 == 1 {
        ctx.lines_as(
            "Rotute",
            args![
                "Years ago, there was a huge",
                "quest to retrieve 4 rare swords",
                "of incredible power, supposedly",
                "the strongest swords ever made!",
                "But you know, there's a strange",
                "rumor about a secret 5th sword."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rotute",
            args![
                "This 5th sword is supposed",
                "to be hidden near Glastheim.",
                "If it does exist, it might have",
                "the power to change the world!",
                "Isn't that freakin' scary?"
            ],
        )?;
    } else if subject1 == 2 || subject1 == 3 {
        ctx.lines_as(
            "Rotute",
            args![
                "Hey, isn't that Koserahserah?",
                "That's the best seasoning that",
                "you can have for meat dishes!",
                "Oh, you got that from the Chief",
                "to give to Rochito? Wow, they",
                "must be really good friends..."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Rotute",
            args![
                "Hello, thanks for helping",
                "out Rockha, Rochito and",
                "Emralhandas. They've known",
                "me since I was born, so it's",
                "like they're family to me."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rotute",
            args![
                "One of the reasons that",
                "they brought me here to",
                "Comodo was so that I could",
                "finally meet Tausupa. But...",
                "I guess he's too busy now.",
                "Still, I like this place!"
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn campground_lad_cmd(ctx: &Ctx) -> Script {
    campground_lad_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn bbq_boy_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Rinta",
        args![
            "I didn't mean to,",
            "but I accidentally",
            "eavesdropped on those",
            "people over there. Are",
            "they really old friends",
            "with our Village Chief?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Rinta",
        args![
            "In that case, they must",
            "be getting the special",
            "treatment usually reserved",
            "for visiting dignitaries and",
            "the like. Or something like that."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bbq_boy_cmd(ctx: &Ctx) -> Script {
    bbq_boy_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn bbq_visitor_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Razy",
        args![
            "Those people over there",
            "are friends of the Village",
            "Chief? I was wondering about",
            "them for the longest time...",
            "I didn't even know our Chief",
            "had friends outside of Comodo."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Razy",
        args![
            "You know, they kind of",
            "strike me as old war buddies,",
            "sprinkling their conversations",
            "with words like ''mission'' all",
            "the time. Then, there's those",
            "little scars they all have..."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bbq_visitor_cmd(ctx: &Ctx) -> Script {
    bbq_visitor_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn bbq_papa_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "BBQ Boy",
        args![
            "D-daddy! Is what those",
            "people talking about true?",
            "Was there really an evil",
            "witch here in our village?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "BBQ Mama",
        args![
            "Honey...",
            "I'm sure those",
            "good people were",
            "just kidding around~",
            "How can that be true?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "BBQ Papa",
        args![
            "Now, now, don't tell",
            "me my little man is afraid",
            "of something like a little",
            "witch. It's just an old fairy",
            "tale, son: no reason to",
            "feel frightened at all."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "BBQ Boy",
        args![
            "No, Daddy, I'm not",
            "scared! But if the witch",
            "really lived here, then her",
            "home is somewhere around",
            "here in Comodo, right? I'm",
            "gonna go find it someday!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "BBQ Mama",
        args![
            "Oh, but I'm sure our",
            "Chief would already know",
            "something about that witch",
            "if she truly exists. Anyway,",
            "let's hurry and eat before",
            "the barbeque burns, okay?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "BBQ Boy",
        args![
            "Yay, barbeque!",
            "Mmmmm... Someday,",
            "if that witch is real,",
            "I'm gonna find out",
            "all about her!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn bbq_papa_cmd(ctx: &Ctx) -> Script {
    bbq_papa_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn chief_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("dmdswrd_q").get()?.number()? > 4 {
        ctx.lines_as(
            "Tausupa",
            args![
                "Maybe you can't tell because",
                "of the way the light reflects,",
                "but Comodo is actually built",
                "inside a huge cave, giving the",
                "illusion of an eternal night.",
                "It's quite beautiful, really..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tausupa",
            args![
                "People come from all over",
                "the world to experience the",
                "excitement and beauty of",
                "our unique little village.",
                "We've become quite",
                "the tourist attraction~"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 3 {
        ctx.lines_as(
            "Tausupa",
            args![
                "Oh, how are my friends",
                "doing? I really wish that",
                "I could have delivered that",
                "Koserahserah personally,",
                "but I can't shirk my duties as",
                "Village Chief and protector."
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FFYou give Tausupa the",
            "Meruchieligu wine that",
            "Rochito asked you to deliver.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Tausupa",
            args![
                "Ah... How very kind of",
                "them! They really sent me",
                "this wine? I'm truly touched...",
                "Rockha must have chosen",
                "this--I'll be sure to enjoy it."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Tausupa",
            args![
                "Would you please express my",
                "thanks to my friends, ^3355FFRochito^000000,",
                "and ^3355FFRockha^000000? It's been far too",
                "long since I've seen them, but",
                "I hope that I get a chance to",
                "visit them someday soon."
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(4))?;
    } else if subject1 == 4 {
        ctx.lines_as(
            "Tausupa",
            args![
                "Would you please express my",
                "thanks to my friends, ^3355FFRochito^000000,",
                "and ^3355FFRockha^000000? It's been far too",
                "long since I've seen them, but",
                "I hope that I get a chance to",
                "visit them someday soon."
            ],
        )?;
    } else {
        ctx.lines_as(
            "Tausupa",
            args![
                "Greetings, adventurer,",
                "I am Tausupa, the Chief of",
                "Comodo, a city famous for its beauty and nightlife. I hope you",
                "enjoy your stay, whether you are seeking excitement or relaxation~"
            ],
        )?;
        ctx.next()?;
        match runtime::select_values(ctx, &[Val::from("About Casino:About Banished Witch:Cancel")])? {
            1 => {
                ctx.lines_as(
                    "Tausupa",
                    args![
                        "Ah yes, Comodo is world",
                        "famous for its Casino. There",
                        "are many games that you can",
                        "enjoy, but you'll need to use",
                        "the Casino's special Eulwo currency and conversion system..."
                    ],
                )?;
            }
            2 => {
                let subject3 = ctx.var("dmdswrd_q").get()?;
                if subject3 == 0 {
                    ctx.lines_as(
                        "Tausupa",
                        args![
                            "Banished witch...?",
                            "Ah ha ha, do not worry,",
                            "my friend, that is merely",
                            "a very old tale. Not worth",
                            "your concern at all..."
                        ],
                    )?;
                } else if subject3 == 1 {
                    ctx.lines_as(
                        "Tausupa",
                        args![
                            "Ah, judging from the scent",
                            "of BBQ meat on your clothes,",
                            "I'm guessing that you ran into",
                            "Rochito in the campgrounds, ",
                            "right? He must have told you that old story about the witch..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tausupa",
                        args![
                            "I'd like nothing better",
                            "than to join them, but I must",
                            "stay here. The witch does exist, and one my jobs is to make sure",
                            "that she does not revive by using my sword's power to suppress her."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tausupa",
                        args![
                            "Although I planned to see",
                            "them today, my duties must",
                            "take priority. Would you please",
                            "take this seasoning to Rochito",
                            "and let him know that I can't",
                            "come, and that I'm sorry...?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFYou have received",
                        "the Koserahserah",
                        "seasoning from the Chief.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Tausupa",
                        args![
                            "Thank you so much...",
                            "I understand that my",
                            "friends cannot begin the",
                            "barbeque without Comodo's",
                            "world famous seasoning..."
                        ],
                    )?;
                    ctx.var("dmdswrd_q").set(Val::from(2))?;
                } else if subject3 == 2 {
                    ctx.lines_as(
                        "Tausupa",
                        args![
                            "Please take this special",
                            "Koserserah seasoning to",
                            "my friend ^3355FFRochito^000000 at the",
                            "barbeque campground. Thanks",
                            "again for your help, adventurer."
                        ],
                    )?;
                }
            }
            3 => {
                ctx.lines_as(
                    "Tausupa",
                    args![
                        "Maybe you can't tell because",
                        "of the way the light reflects,",
                        "but Comodo is actually built",
                        "inside a huge cave, giving the",
                        "illusion of an eternal night.",
                        "It's quite beautiful, really..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Tausupa",
                    args![
                        "People come from all over",
                        "the world to experience the",
                        "excitement and beauty of",
                        "our unique little village.",
                        "We've become quite",
                        "the tourist attraction~"
                    ],
                )?;
            }
            _ => {}
        }
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn chief_cmd(ctx: &Ctx) -> Script {
    chief_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn toruna_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Toruna",
        args![
            "When I think about it,",
            "there are many fascinating",
            "facts about Comodo. Can",
            "you believe this entire city",
            "is built inside a natural cave?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("This cave is huge!:This place sure is strange...:Cancel")])? {
        1 => {
            ctx.lines_as(
                "Toruna",
                args![
                    "Oh, this is a huge cave,",
                    "but it's not impossible for",
                    "nature to create something",
                    "of this size and scale if given",
                    "thousands, maybe even millions",
                    "of years. Let me explain..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Toruna",
                args![
                    "Now, water is known as",
                    "a universal solvent: running",
                    "water will carve its own path",
                    "in rock, given enough time.",
                    "Now, keep in mind that this",
                    "cave was once solid limestone."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Toruna",
                args![
                    "Years and years of running",
                    "water from the rains and the",
                    "water table have broken down",
                    "this giant pocket of limestone,",
                    "clearing out this huge, open",
                    "area, the Comodo Cave."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Toruna",
                args![
                    "That may sound incredulous,",
                    "but cracks in limestone grow",
                    "into faults, then spacious gaps",
                    "over years and years of erosion. Now, isn't natural science just",
                    "amazing? You've got to agree..."
                ],
            )?;
        }
        2 => {
            let subject2 = ctx.var("dmdswrd_q").get()?;
            if subject2 == 5 {
                ctx.lines_as(
                    "Toruna",
                    args![
                        "Yes, that is rather",
                        "peculiar. What's also",
                        "strange is this rumor I've",
                        "been hearing about. Now, are you familiar with Comodo Cheese?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "It's this urban legend",
                        "about this magnificent cheese",
                        "that you can only find in Comodo. Now, not too many people believe",
                        "it. I mean, you need goats or cows",
                        "in order to make any cheese."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "However, there's this",
                        "strange man that insists",
                        "on its existence. The locals",
                        "here chalk him up to be some",
                        "sort of common loon, but who",
                        "knows? Maybe it does exist..."
                    ],
                )?;
                ctx.var("dmdswrd_q").set(Val::from(6))?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "Well, you can decide for",
                        "yourself whether he's off",
                        "his rocker. The last time",
                        "I saw him, he was in the",
                        "Comodo Bar, so you can",
                        "probably find him there."
                    ],
                )?;
            } else if subject2 == 6 {
                ctx.lines_as(
                    "Toruna",
                    args![
                        "Well, I like to think of",
                        "Comodo as unique. What's",
                        "really strange are some of",
                        "the locals in this area. I've",
                        "already told you about the",
                        "man in the Comodo Bar, yes?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "He keeps insisting",
                        "that there is a special",
                        "kind of cheese in Comodo",
                        "that you cannot get anywhere",
                        "else. It sounds crazy, but his",
                        "claim may be worth verifying..."
                    ],
                )?;
            } else {
                ctx.lines_as(
                    "Toruna",
                    args![
                        "Ah, did you know that",
                        "before it was an exotic",
                        "village of excitement and",
                        "leisure, Comodo was once",
                        "a haven for evil creatures, ruled by a witch named Meropusum?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "This witch lay dormant for",
                        "years until she was awoken",
                        "somehow years ago. She was",
                        "a huge threat to the people, but then she was finally defeated",
                        "in the famous War of the Witch."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "One of the heroes of that",
                        "war became the current Chief",
                        "of this village, and it was",
                        "through his leadership that",
                        "Comodo grew into a thriving",
                        "tourist attraction."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Toruna",
                    args![
                        "However, the threat of",
                        "Meropusum still exists,",
                        "so I would be careful when",
                        "exploring the uninhabited",
                        "areas of the Comodo region."
                    ],
                )?;
            }
        }
        3 => {
            ctx.lines_as(
                "Toruna",
                args![
                    "There's much to do",
                    "in this city of Comodo,",
                    "as well as much to learn",
                    "about it. I can never tire of",
                    "visiting this exotic city..."
                ],
            )?;
        }
        _ => {}
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn toruna_cmd(ctx: &Ctx) -> Script {
    toruna_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn rakusa_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 6 {
        ctx.lines_as(
            "Rakusa",
            args![
                "See that guy sitting",
                "over there? All he does",
                "is talk about Comodo Cheese,",
                "just mumbling nonsense about",
                "its incredible flavors all day",
                "long. He's nuts, I tell you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rakusa",
            args![
                "First of all, you need",
                "freakin' milk to make any",
                "cheese. You see any cows",
                "around here? Nope, I thought",
                "so! Still, I think he really",
                "believes that it exists..."
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(7))?;
    } else if subject1 == 7 {
        ctx.lines_as(
            "Rakusa",
            args![
                "Damn it! I know for a fact",
                "that the idea of Comodo even",
                "having a cheese is crazy, but",
                "that guy's talking has gotten",
                "me curious now! Maybe it does",
                "exist? How would it even taste?"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Rakusa",
            args![
                "You know, Comodo has",
                "a reputation for offering",
                "high class recreation, so",
                "people forget that we actually",
                "have two dangerous dungeons."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rakusa",
            args![
                "You adventurers need to",
                "be extra careful if you explore",
                "the caves--the monsters there",
                "are unusually strong. Make",
                "sure that you're well prepared",
                "before you even think of going!"
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn rakusa_cmd(ctx: &Ctx) -> Script {
    rakusa_cmd_body(ctx, Vec::new()).map(|_| ())
}

fn kichiri_cmd_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let subject1 = ctx.var("dmdswrd_q").get()?;
    if subject1 == 7 {
        ctx.lines_as(
            "Kichiri",
            args![
                "Freakin' ^3355FFMagatu^000000...!",
                "Can he talk about anything else",
                "aside from Comodo Cheese?",
                "I don't see why he's so excited",
                "about the stuff! Everyone knows",
                "it's just an old wive's tale."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kichiri",
            args![
                "Would you do",
                "me a favor and get",
                "Magatu to shut up?",
                "I just want to drink",
                "my alcohol in peace!"
            ],
        )?;
        ctx.var("dmdswrd_q").set(Val::from(8))?;
    } else if subject1 == 8 {
        ctx.lines_as(
            "Kichiri",
            args![
                "Cripes! Now Magatu's",
                "got me wondering! I mean,",
                "just because I've never seen",
                "Comodo Cheese doesn't mean",
                "it doesn't exist, right? Curses! Now I'm thinking about it too!"
            ],
        )?;
    } else {
        ctx.lines_as(
            "Kichiri",
            args![
                "There's more to do than",
                "gambling in Comodo, you",
                "know. Sometimes, I love",
                "to watch the Dancers on",
                "stage in the middle of the",
                "village. They're so glamorous~"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Kichiri",
            args![
                "I hear that the Dance",
                "Academy only accepts female",
                "Archers to become prospective",
                "Dancers. I guess that makes",
                "sense--they're tone and fit,",
                "but not musclebound either..."
            ],
        )?;
    }
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn kichiri_cmd(ctx: &Ctx) -> Script {
    kichiri_cmd_body(ctx, Vec::new()).map(|_| ())
}
