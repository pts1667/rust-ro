use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn uwe_kleine_ein_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_aaa = Val::from(0);
    let mut l_bbb = Val::from(0);
    let mut l_ccc = Val::from(0);
    let mut l_eee = Val::from(0);
    if ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_BLACKSMITH")?) {
        if ctx.var("ein_cook").get()?.number()? > 999 {
            l_ccc = (ctx.var("ein_cook").get()?.try_sub(Val::from(1000))?);
            if l_ccc.clone().number()? > 199 {
                ctx.lines_as(
                    "Uwe",
                    args![
                        "Oh hello hello~",
                        "It's been a long",
                        "time since we've talked,",
                        "you cutie adventurer~"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("So... You're a chef.:Yes, I tried my best.")])? {
                    1 => {
                        ctx.lines_as(
                            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                            args!["So...", "You're a chef."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "That's right, it's",
                                "what I do for business.",
                                "Now, as for what I do for",
                                "^EEA9B8pleasure^000000, well, please don't",
                                "hesitate to ask, sugar honey~ "
                            ],
                        )?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Uwe", args!["Only continuous effort will lead you to success. I guess that means you've learned your lesson, sugar honey~"])?;
                        ctx.next()?;
                    }
                    _ => {}
                }
                if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Oh, I was going to give",
                            "you a little something,",
                            "but you've carrying too",
                            "many things. Why don't",
                            "you put some of it away",
                            "in your Kafra Storage?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.var("ein_cook").set(Val::from(219))?;
                ctx.call(Function::GetItem, vec![Val::from(612), Val::from(10)])?;
                ctx.lines_as(
                    "Uwe",
                    args![
                        "I had these lying",
                        "around, so why don't",
                        "you take them, sugar",
                        "honey? They're not very",
                        "pricey, but they're useful",
                        "in doing some smithing."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Uwe", args!["Alright then~", "Hope you enjoy", "your Mini Furances"])?;
                let subject2 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                if subject2 == 1 {
                    ctx.mes("...Ho ho~")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 2 {
                    ctx.mes("...*Tee Hee~*")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if subject2 == 3 {
                    ctx.mes("...Behbie~")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else if l_ccc.clone().number()? > 99 {
                ctx.lines_as(
                    "Uwe",
                    args![
                        "Oh hello hello~",
                        "It's been a long",
                        "time since we've talked,",
                        "you cutie adventurer~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Uwe",
                    args![
                        "So, sugar honey,",
                        "how is it going with",
                        "the little favor I asked",
                        "you about last time?",
                        "Did you already forget",
                        "the 6 Large Jellopy?"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("There you go.:I want to quit.:Can I do it later?")])? {
                    1 => {
                        if ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 5 {
                            if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(199)])? == 0 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, I was going to give",
                                        "you a little something,",
                                        "but you've carrying too",
                                        "many things. Why don't",
                                        "you put some of it away",
                                        "in your Kafra Storage?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Uwe", args!["Thank you ^EEA9B8so^000000 much!", "Here's the Coal I promised~", "Oh, and take these Mini Furnaces. They were just lying around, so you know, I thought, well, why don't you take them? *Tee hee hee~*"])?;
                            ctx.call(Function::DelItem, vec![Val::from(7126), Val::from(6)])?;
                            ctx.var("ein_cook").set(Val::from(219))?;
                            ctx.call(Function::GetItem, vec![Val::from(1003), Val::from(1)])?;
                            ctx.call(Function::GetItem, vec![Val::from(612), Val::from(10)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "I hope you enjoy!",
                                    "Personally, I think the",
                                    "best part of smithing",
                                    "is standing over the flaming",
                                    "heat and getting all ^EEA9B8hot and",
                                    "sweaty^000000. Ooh, how exciting~!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 0 {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Ooh, you're missing",
                                    "some. I guess you can",
                                    "still give them to me, but",
                                    "you'd get less of a reward"
                                ],
                            )?;
                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                ctx.mes("...you ^EEA9B8naughty boy^000000.")?;
                            } else {
                                ctx.mes("...you ^EEA9B8naughty girl^000000.")?;
                            }
                            match runtime::select_values(ctx, &[Val::from("Give him all.:Cancel.")])? {
                                1 => {
                                    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Oh, I was going to give",
                                                "you a little something,",
                                                "but you've carrying too",
                                                "many things. Why don't",
                                                "you put some of it away",
                                                "in your Kafra Storage?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as("Uwe", args!["Just kidding~", "Don't be so nervous, I don't really need those anymore. But since I know you're hard working, I'll throw in a little something extra..."])?;
                                    ctx.next()?;
                                    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(220)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Oh, I was going to give",
                                                "you a little something,",
                                                "but you've carrying too",
                                                "many things. Why don't",
                                                "you put some of it away",
                                                "in your Kafra Storage?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.mes("[Uwe]")?;
                                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?) {
                                        ctx.mes("I usually give you cutie MasterSmiths 10 Mini Furnaces, but you can have ^EEA9B811^000000. There's the little something extra I was talking about.")?;
                                    } else {
                                        ctx.mes("I usually give you cutie Blacksmiths 10 Mini Furnaces, but you can have ^EEA9B811^000000. There's the little something extra I was talking about.")?;
                                    }
                                    ctx.var("ein_cook").set(Val::from(219))?;
                                    ctx.call(Function::GetItem, vec![Val::from(612), Val::from(11)])?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["I know it's not very extravagant, but these will come in handy next time you're smithing. So make the best use of them, 'kay?"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Oh, I was going to give",
                                                "you a little something,",
                                                "but you've carrying too",
                                                "many things. Why don't",
                                                "you put some of it away",
                                                "in your Kafra Storage?"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines_as("Uwe", args!["Well... Then... Anyway,", "Congratulation to became"])?;
                                    if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?) {
                                        ctx.mes("a Master Smith!!")?;
                                    } else {
                                        ctx.mes("a Black Smith!!")?;
                                    }
                                    ctx.mes("This is my present for it.")?;
                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                        ctx.mes("...you ^EEA9B8naughty boy^000000.")?;
                                    } else {
                                        ctx.mes("...you ^EEA9B8naughty girl^000000.")?;
                                    }
                                }
                                _ => {}
                            }
                            if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(100)])? == 0 {
                                ctx.next()?;
                                ctx.lines(args![
                                    "Oh, I was going to give",
                                    "you a little something,",
                                    "but you've carrying too",
                                    "many things. Why don't",
                                    "you put some of it away",
                                    "in your Kafra Storage?"
                                ])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.var("ein_cook").set(Val::from(219))?;
                            ctx.call(Function::GetItem, vec![Val::from(612), Val::from(5)])?;
                            ctx.next()?;
                            ctx.lines_as("Uwe", args!["I know it's not very extravagant, but these will come in handy next time you're smithing. So make the best use of them, 'kay?"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as("Uwe", args!["^666666You didn't bring", "any Large Jellopy...?^000000."])?;
                            ctx.var("ein_cook").set(Val::from(219))?;
                            ctx.next()?;
                            ctx.lines_as("Uwe", args!["It's fine! It's not like I feel ^EEA9B8betrayed^000000 or anything, but I don't need them anymore! Since we're both in the business of smithing, I just hope that you remember to follow through on your favors, 'kay? Buhbye~"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    2 => {
                        if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oh, I was going to give",
                                    "you a little something,",
                                    "but you've carrying too",
                                    "many things. Why don't",
                                    "you put some of it away",
                                    "in your Kafra Storage?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Quit...?",
                                "Oh well, at least you're honest. But let me give you a little something, since we're both fellow smiths."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args!["Still...", "That doesn't change", "the fact that you've", "been very, very bad."],
                        )?;
                        if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                            ctx.mes("...You ^EEA9B8naughty boy^000000.")?;
                        } else {
                            ctx.mes("...You ^EEA9B8naughty girl^000000.")?;
                        }
                        ctx.var("ein_cook").set(Val::from(219))?;
                        ctx.call(Function::GetItem, vec![Val::from(612), Val::from(10)])?;
                        ctx.next()?;
                        ctx.lines_as("Uwe", args!["I know it's not very extravagant, but these will come in handy next time you're smithing. So make the best use of them, 'kay?"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    3 => {
                        ctx.lines_as("Uwe", args!["Oooh, you came back later than I thought, so I don't need really need them anymore. Then again, just holding onto them doesn't sound right either..."])?;
                        ctx.next()?;
                        'b5: {
                            let subject5 = Val::from(runtime::select_values(ctx, &[Val::from("Here you go~:I want to quit.")])?);
                            let mut matched5 = false;
                            let no_case5 = !subject5.loosely_equals(&Val::from(1)) && !subject5.loosely_equals(&Val::from(2));
                            if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                matched5 = true;
                            }
                            if matched5 {
                                if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(199)])? == 0 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, I was going to give",
                                            "you a little something,",
                                            "but you've carrying too",
                                            "many things. Why don't",
                                            "you put some of it away",
                                            "in your Kafra Storage?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 5 {
                                    ctx.lines_as("Uwe", args!["Thank you ^EEA9B8so^000000 much!", "Here's the Coal I promised~", "Oh, and take these Mini Furnaces. They were just lying around, so you know, I thought, well, why don't you take them? *Tee hee hee~*"])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7126), Val::from(6)])?;
                                    ctx.var("ein_cook").set(Val::from(219))?;
                                    ctx.call(Function::GetItem, vec![Val::from(1003), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(612), Val::from(10)])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 0 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Ooh, you're missing",
                                            "some. I guess you can",
                                            "still give them to me, but",
                                            "you'd get less of a reward"
                                        ],
                                    )?;
                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                        ctx.mes("...You ^EEA9B8naughty boy^000000.")?;
                                    } else {
                                        ctx.mes("...You ^EEA9B8naughty girl^000000.")?;
                                    }
                                    match runtime::select_values(ctx, &[Val::from("Give them all.:Cancel.")])? {
                                        1 => {
                                            if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(220)])? == 0 {
                                                ctx.lines_as(
                                                    "Uwe",
                                                    args![
                                                        "Oh, I was going to give",
                                                        "you a little something,",
                                                        "but you've carrying too",
                                                        "many things. Why don't",
                                                        "you put some of it away",
                                                        "in your Kafra Storage?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as("Uwe", args!["Just kidding~", "Don't be so nervous, I don't really need those anymore. But since I know you're hard working, I'll throw in a little something extra..."])?;
                                            ctx.next()?;
                                            ctx.mes("[Uwe]")?;
                                            if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?) {
                                                ctx.mes("I usually give you cutie MasterSmiths 10 Mini Furnaces, but you can have ^EEA9B811^000000. There's the little something extra I was talking about.")?;
                                            } else {
                                                ctx.mes("I usually give you cutie Blacksmiths 10 Mini Furnaces, but you can have ^EEA9B811^000000. There's the little something extra I was talking about.")?;
                                            }
                                            ctx.var("ein_cook").set(Val::from(219))?;
                                            ctx.call(Function::GetItem, vec![Val::from(612), Val::from(11)])?;
                                            ctx.next()?;
                                            ctx.lines_as("Uwe", args!["I know it's not very extravagant, but these will come in handy next time you're smithing. So make the best use of them, 'kay?"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
                                                ctx.lines_as(
                                                    "Uwe",
                                                    args![
                                                        "Oh, I was going to give",
                                                        "you a little something,",
                                                        "but you've carrying too",
                                                        "many things. Why don't",
                                                        "you put some of it away",
                                                        "in your Kafra Storage?"
                                                    ],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Uwe",
                                                args![
                                                    "Okay.",
                                                    "Anyway~!",
                                                    "When you do get all",
                                                    "that Large Jellopy,",
                                                    "just come back to me."
                                                ],
                                            )?;
                                            if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                                ctx.mes("...you ^EEA9B8naughty boy^000000.")?;
                                            } else {
                                                ctx.mes("...you ^EEA9B8naughty girl^000000.")?;
                                            }
                                            ctx.var("ein_cook").set(Val::from(219))?;
                                            ctx.call(Function::GetItem, vec![Val::from(612), Val::from(10)])?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Uwe",
                                                args![
                                                    "This Portable Furnace is not expensive",
                                                    "but very important item for smiths, right?",
                                                    "Use it!",
                                                    "And visit me someday again. Huhuhu.."
                                                ],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else {
                                    ctx.lines_as("Uwe", args!["^666666You didn't bring", "any Large Jellopy...?^000000."])?;
                                    ctx.var("ein_cook").set(Val::from(219))?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["It's fine! It's not like I feel ^EEA9B8betrayed^000000 or anything, but I don't need them anymore! Since we're both in the business of smithing, I just hope that you remember to follow through on your favors, 'kay? Buhbye~"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                matched5 = true;
                            }
                            if matched5 {
                                if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(200)])? == 0 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, I was going to give",
                                            "you a little something,",
                                            "but you've carrying too",
                                            "many things. Why don't",
                                            "you put some of it away",
                                            "in your Kafra Storage?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Uwe", args!["Well... Then... Anyway,", "Congratulation to became"])?;
                                if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?) {
                                    ctx.lines(args!["a Master Smith!!", "This is my present for it. Huhuhu."])?;
                                } else {
                                    ctx.lines(args!["a Black Smith!!", "This is my present for it. Huhuhu."])?;
                                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                                        ctx.mes("...You ^EEA9B8naughty boy^000000.")?;
                                    } else {
                                        ctx.mes("...You ^EEA9B8naughty girl^000000.")?;
                                    }
                                }
                                ctx.var("ein_cook").set(Val::from(219))?;
                                ctx.call(Function::GetItem, vec![Val::from(612), Val::from(10)])?;
                                ctx.next()?;
                                ctx.lines_as("Uwe", args!["I know it's not very extravagant, but these will come in handy next time you're smithing. So make the best use of them, 'kay?"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        if ctx.var("ein_cook").get()? == 0 {
            ctx.lines_as(
                "Uwe",
                args![
                    "Cooking is such a joy~!",
                    "The scents, the flavors, the sensation of sheer ^EEA9B8satiation^000000..."
                ],
            )?;
            ctx.next()?;
            'b7: {
                let subject7 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Um, isn't this a forge?:Ignore him.")],
                )?);
                let mut matched7 = false;
                let no_case7 = !subject7.loosely_equals(&Val::from(1)) && !subject7.loosely_equals(&Val::from(2));
                if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Is this a forge?",
                            "Oh, sugar honey,",
                            "you haven't been here",
                            "before, haven't you?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "My name is Uwe Kleine",
                            "and this is my forge~! I am",
                            "the most elegant Blacksmith",
                            "and the best chef here in the",
                            "Schwarzwald Republic~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Cooking is probably one",
                            "of the greatest pleasures",
                            "a craftsman can enjoy. Ooh,",
                            "what do you think is the most",
                            "important factor in cooking,"
                        ],
                    )?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.mes("you ^EEA9B8naughty little man^000000?")?;
                    } else {
                        ctx.mes("you ^EEA9B8naughty, naughty girl^000000?")?;
                    }
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Heart.:Ingredients.:Skills.:Tools.")])? {
                        1 => {
                            ctx.lines_as("Uwe", args!["...", "......", "......Heart?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Na-na-na-na-noooo~",
                                    "If a heart was all you",
                                    "needed to cook or forge,",
                                    "then anyone can make and",
                                    "upgrade a Claymore, right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "What you really need",
                                    "when cooking and forging",
                                    "is ^EEA9B8desire^000000. The burning desire",
                                    "and passion to make something",
                                    "great! If that was what you meant, then you are absolutely right!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args!["But any other meaning", "to that answer would have", "been absolutely wrooooong."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oooh, I just received",
                                    "some Emvertarcons today,",
                                    "so please have one, 'kay?",
                                    "Welcome to Einbroch, cutie~"
                                ],
                            )?;
                            ctx.var("ein_cook").set(Val::from(1))?;
                            ctx.call(Function::GetItem, vec![Val::from(1011), Val::from(1)])?;
                        }
                        2 => {
                            ctx.lines_as("Uwe", args!["...", "......", "......Ingredients?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Na-na-na-na-noooo~",
                                    "People always blame their",
                                    "ingredients when something",
                                    "goes wrong! It's those people",
                                    "who just embarrass themselves~ "
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Uwe", args!["Now, if you have the pride to only use the very best ingredients you can find, then that's fine. But the quality of whatever you make can't depend on the ingredients alone."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Speaking of ingredients,",
                                    "someone just gave me some",
                                    "Steel. Why don't you have 2 of",
                                    "them as my little way of saying, ''Welcome to Einbroch, cutie~!''"
                                ],
                            )?;
                            ctx.var("ein_cook").set(Val::from(3))?;
                            ctx.call(Function::GetItem, vec![Val::from(998), Val::from(2)])?;
                        }
                        3 => {
                            ctx.lines_as("Uwe", args!["...", "......", "......Skills?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, skill alone isn't",
                                    "enough to make something",
                                    "great. You also need to put in",
                                    "the effort. You can't complain",
                                    "about not having enough skill",
                                    "when you slack off, right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "You always need to invest",
                                    "time and effort to make your",
                                    "crafts stand out. Now, if you",
                                    "meet someone whose skill",
                                    "makes yours seem pathetic, driving you to improve yourself..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Then by all means,",
                                    "feel free to follow",
                                    "your drive and improve",
                                    "your skills! Just don't",
                                    "fall back on your lack",
                                    "of skills as any excuse."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oooh, I just received",
                                    "some Emvertarcons today,",
                                    "so please have one, 'kay?",
                                    "Welcome to Einbroch, cutie~"
                                ],
                            )?;
                            ctx.var("ein_cook").set(Val::from(5))?;
                            ctx.call(Function::GetItem, vec![Val::from(1011), Val::from(1)])?;
                        }
                        4 => {
                            ctx.lines_as("Uwe", args!["...", "......", "......Tools?"])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, not just anyone can",
                                    "bake delicious cakes, even",
                                    "if they had the best oven in",
                                    "in the Schwarzwald Republic.",
                                    "And a child with all the tools",
                                    "couldn't put a sword together."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Tools are helpful, but",
                                    "master craftsmen never blame",
                                    "their tools for their mistakes."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oh, and speaking of",
                                    "tools, I just received",
                                    "some new hammers today.",
                                    "Why don't you have one as",
                                    "my way of saying ''Welcome",
                                    "to Einbroch, sugar honey~''"
                                ],
                            )?;
                            ctx.var("ein_cook").set(Val::from(7))?;
                            ctx.call(Function::GetItem, vec![Val::from(613), Val::from(1)])?;
                        }
                        _ => {}
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Oh...",
                            "And feel free to",
                            "come and see me",
                            "anytime, alright?",
                            "So don't be shy"
                        ],
                    )?;
                    let subject9 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if subject9 == 1 {
                        ctx.mes("...Ho ho~")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject9 == 2 {
                        ctx.mes("...*Tee Hee~*")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else if subject9 == 3 {
                        ctx.mes("...Behbie~")?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Hmmm...?",
                            "Why don't you stay and chat?",
                            "Oh, you adventurers are always",
                            "so busy with your things to do",
                            "and people to see. Ah well~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else {
            if ctx.var("ein_cook").get()?.number()? < 10 {
                ctx.lines_as(
                    "Uwe",
                    args![
                        "Oh! Hello again,",
                        "you cutie adventurer.",
                        "So tell me, what brings",
                        ((Val::from("you here, ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?"))
                    ],
                )?;
                ctx.next()?;
                'b10: {
                    let subject10 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Ask about Cooking.:Ask about Einbroch.:Cancel.")],
                    )?);
                    let mut matched10 = false;
                    let no_case10 = !subject10.loosely_equals(&Val::from(1))
                        && !subject10.loosely_equals(&Val::from(2))
                        && !subject10.loosely_equals(&Val::from(3));
                    if !matched10 && subject10.loosely_equals(&Val::from(1)) {
                        matched10 = true;
                    }
                    if matched10 {
                        if ctx.var("ein_cook").get()? == 1 {
                            ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                            ctx.next()?;
                            'b11: {
                                let subject11 = Val::from(runtime::select_values(ctx, &[Val::from("Ingredients:Skills:Tools:Cancel")])?);
                                let mut matched11 = false;
                                let no_case11 = !subject11.loosely_equals(&Val::from(1))
                                    && !subject11.loosely_equals(&Val::from(2))
                                    && !subject11.loosely_equals(&Val::from(3))
                                    && !subject11.loosely_equals(&Val::from(4));
                                if !matched11 && subject11.loosely_equals(&Val::from(1)) {
                                    matched11 = true;
                                }
                                if matched11 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Now for beginners,",
                                            "learning to select",
                                            "and use ingredients",
                                            "is one of the most",
                                            "important fundamentals."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is like forging",
                                            "since specific ingredients",
                                            "are needed to make specialty",
                                            "items or dishes. You can't just",
                                            "skip them if you really need",
                                            "them, right? Right!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Once you make up your",
                                            "mind to do something,",
                                            "focus on finishing it, 'kay?",
                                            "Never cut corners and always",
                                            "dedicate yourself to make the",
                                            "very best finished product~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject12 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject12 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject12 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject12 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched11 && subject11.loosely_equals(&Val::from(2)) {
                                    matched11 = true;
                                }
                                if matched11 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "When you're beginning to learn",
                                            "skills, you can't let yourself be discouraged! Practice makes",
                                            "perfect, you know? But never",
                                            "use your lack of skills as an",
                                            "excuse if you happen to fail..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["Effort is also an essential", "in forging and cooking! Now,", "on the other hand, if you put", "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "The key is to always",
                                            "give 100% effort and work",
                                            "on improving your skills.",
                                            "Before you know it, you'll",
                                            "be a respected master!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject13 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject13 == 1 {
                                        ctx.mes("Hohohohoho.")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject13 == 2 {
                                        ctx.mes("Umhohohoho.*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject13 == 3 {
                                        ctx.mes("Umho.")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched11 && subject11.loosely_equals(&Val::from(3)) {
                                    matched11 = true;
                                }
                                if matched11 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is a little",
                                            "easier if you have",
                                            "nicer tools to use,",
                                            "but that's it. Tools by",
                                            "themselves can't make",
                                            "just anybody a master."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "In the end, the best",
                                            "tools are the ones you're",
                                            "most comfortable with using.",
                                            "In fact, I still use the old knife I used back when I was just",
                                            "a little novice chef~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, while we're on",
                                            "the subject of tools,",
                                            "I just received some",
                                            "new hammers today.",
                                            "Would you like a couple?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(50)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Mmmm...?",
                                                "You're holding too many",
                                                "things. I can't give you any",
                                                "hammers if you don't have",
                                                "room. Hurry, put your extra",
                                                "stuff in your Kafra Storage~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.var("ein_cook").set(Val::from(11))?;
                                    ctx.call(Function::GetItem, vec![Val::from(614), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(613), Val::from(1)])?;
                                    ctx.lines_as("Uwe", args!["Well, I hope", "you like them~"])?;
                                    let subject14 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject14 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject14 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject14 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched11 && subject11.loosely_equals(&Val::from(4)) {
                                    matched11 = true;
                                }
                                if matched11 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        } else if ctx.var("ein_cook").get()? == 3 {
                            ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                            ctx.next()?;
                            'b15: {
                                let subject15 = Val::from(runtime::select_values(ctx, &[Val::from("Heart:Skills:Tools:Cancel.")])?);
                                let mut matched15 = false;
                                let no_case15 = !subject15.loosely_equals(&Val::from(1))
                                    && !subject15.loosely_equals(&Val::from(2))
                                    && !subject15.loosely_equals(&Val::from(3))
                                    && !subject15.loosely_equals(&Val::from(4));
                                if !matched15 && subject15.loosely_equals(&Val::from(1)) {
                                    matched15 = true;
                                }
                                if matched15 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Ah, heart. Just like forging,",
                                            "you need passion and desire",
                                            "to create something special.",
                                            "Every smith and cook knows that",
                                            "you can do anything if you have",
                                            "the will and the commitment."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that's all",
                                            "I really have to",
                                            "say about that. But",
                                            "come back and chat",
                                            "whenever you please."
                                        ],
                                    )?;
                                    let subject16 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject16 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject16 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject16 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched15 && subject15.loosely_equals(&Val::from(2)) {
                                    matched15 = true;
                                }
                                if matched15 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "When you're beginning to learn",
                                            "skills, you can't let yourself be discouraged! Practice makes",
                                            "perfect, you know? But never",
                                            "use your lack of skills as an",
                                            "excuse if you happen to fail..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["Effort is also an essential", "in forging and cooking! Now,", "on the other hand, if you put", "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "The key is to always",
                                            "give 100% effort and work",
                                            "on improving your skills.",
                                            "Before you know it, you'll",
                                            "be a respected master!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject17 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject17 == 1 {
                                        ctx.mes("Hohohohoho.")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject17 == 2 {
                                        ctx.mes("Umhohohoho.*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject17 == 3 {
                                        ctx.mes("Umho.")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched15 && subject15.loosely_equals(&Val::from(3)) {
                                    matched15 = true;
                                }
                                if matched15 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is a little",
                                            "easier if you have",
                                            "nicer tools to use,",
                                            "but that's it. Tools by",
                                            "themselves can't make",
                                            "just anybody a master."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "In the end, the best",
                                            "tools are the ones you're",
                                            "most comfortable with using.",
                                            "In fact, I still use the old knife I used back when I was just",
                                            "a little novice chef~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, while we're on",
                                            "the subject of tools,",
                                            "I just received some",
                                            "new hammers today.",
                                            "Would you like a couple?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(50)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Mmmm...?",
                                                "You're holding too many",
                                                "things. I can't give you any",
                                                "hammers if you don't have",
                                                "room. Hurry, put your extra",
                                                "stuff in your Kafra Storage~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.var("ein_cook").set(Val::from(13))?;
                                    ctx.call(Function::GetItem, vec![Val::from(614), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(613), Val::from(1)])?;
                                    ctx.lines_as("Uwe", args!["Well, I hope", "you like them~"])?;
                                    let subject18 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject18 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject18 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject18 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched15 && subject15.loosely_equals(&Val::from(4)) {
                                    matched15 = true;
                                }
                                if matched15 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        } else if ctx.var("ein_cook").get()? == 5 {
                            ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                            ctx.next()?;
                            'b19: {
                                let subject19 = Val::from(runtime::select_values(ctx, &[Val::from("Heart:Ingredients:Tools:Cancel")])?);
                                let mut matched19 = false;
                                let no_case19 = !subject19.loosely_equals(&Val::from(1))
                                    && !subject19.loosely_equals(&Val::from(2))
                                    && !subject19.loosely_equals(&Val::from(3))
                                    && !subject19.loosely_equals(&Val::from(4));
                                if !matched19 && subject19.loosely_equals(&Val::from(1)) {
                                    matched19 = true;
                                }
                                if matched19 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Ah, heart. Just like forging,",
                                            "you need passion and desire",
                                            "to create something special.",
                                            "Every smith and cook knows that",
                                            "you can do anything if you have",
                                            "the will and the commitment."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that's all",
                                            "I really have to",
                                            "say about that. But",
                                            "come back and chat",
                                            "whenever you please."
                                        ],
                                    )?;
                                    let subject20 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject20 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject20 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject20 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched19 && subject19.loosely_equals(&Val::from(2)) {
                                    matched19 = true;
                                }
                                if matched19 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Now for beginners,",
                                            "learning to select",
                                            "and use ingredients",
                                            "is one of the most",
                                            "important fundamentals."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is like forging",
                                            "since specific ingredients",
                                            "are needed to make specialty",
                                            "items or dishes. You can't just",
                                            "skip them if you really need",
                                            "them, right? Right!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Once you make up your",
                                            "mind to do something,",
                                            "focus on finishing it, 'kay?",
                                            "Never cut corners and always",
                                            "dedicate yourself to make the",
                                            "very best finished product~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject21 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject21 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject21 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject21 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched19 && subject19.loosely_equals(&Val::from(3)) {
                                    matched19 = true;
                                }
                                if matched19 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is a little",
                                            "easier if you have",
                                            "nicer tools to use,",
                                            "but that's it. Tools by",
                                            "themselves can't make",
                                            "just anybody a master."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "In the end, the best",
                                            "tools are the ones you're",
                                            "most comfortable with using.",
                                            "In fact, I still use the old knife I used back when I was just",
                                            "a little novice chef~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, while we're on",
                                            "the subject of tools,",
                                            "I just received some",
                                            "new hammers today.",
                                            "Would you like a couple?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(50)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Mmmm...?",
                                                "You're holding too many",
                                                "things. I can't give you any",
                                                "hammers if you don't have",
                                                "room. Hurry, put your extra",
                                                "stuff in your Kafra Storage~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.var("ein_cook").set(Val::from(15))?;
                                    ctx.call(Function::GetItem, vec![Val::from(614), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(613), Val::from(1)])?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["Well, I hope", "you like them~"])?;
                                    let subject22 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject22 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject22 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject22 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched19 && subject19.loosely_equals(&Val::from(4)) {
                                    matched19 = true;
                                }
                                if matched19 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        } else if ctx.var("ein_cook").get()? == 7 {
                            ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                            ctx.next()?;
                            'b23: {
                                let subject23 = Val::from(runtime::select_values(ctx, &[Val::from("Heart:Ingredients:Skills:Cancel.")])?);
                                let mut matched23 = false;
                                let no_case23 = !subject23.loosely_equals(&Val::from(1))
                                    && !subject23.loosely_equals(&Val::from(2))
                                    && !subject23.loosely_equals(&Val::from(3))
                                    && !subject23.loosely_equals(&Val::from(4));
                                if !matched23 && subject23.loosely_equals(&Val::from(1)) {
                                    matched23 = true;
                                }
                                if matched23 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Ah, heart. Just like forging,",
                                            "you need passion and desire",
                                            "to create something special.",
                                            "Every smith and cook knows that",
                                            "you can do anything if you have",
                                            "the will and the commitment."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that's all",
                                            "I really have to",
                                            "say about that. But",
                                            "come back and chat",
                                            "whenever you please."
                                        ],
                                    )?;
                                    let subject24 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject24 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject24 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject24 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched23 && subject23.loosely_equals(&Val::from(2)) {
                                    matched23 = true;
                                }
                                if matched23 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Now for beginners,",
                                            "learning to select",
                                            "and use ingredients",
                                            "is one of the most",
                                            "important fundamentals."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is like forging",
                                            "since specific ingredients",
                                            "are needed to make specialty",
                                            "items or dishes. You can't just",
                                            "skip them if you really need",
                                            "them, right? Right!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Once you make up your",
                                            "mind to do something,",
                                            "focus on finishing it, 'kay?",
                                            "Never cut corners and always",
                                            "dedicate yourself to make the",
                                            "very best finished product~"
                                        ],
                                    )?;
                                    ctx.var("ein_cook").set(Val::from(17))?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject25 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject25 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject25 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject25 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched23 && subject23.loosely_equals(&Val::from(3)) {
                                    matched23 = true;
                                }
                                if matched23 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "'I couldn't do it cuz",
                                            "I'm not skillful with it.'",
                                            "It's the most foolish words in the world.",
                                            "There's only way for cooking.",
                                            "Practice, practice and try!."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "I know it's the easiest",
                                            "lame excuse for a person",
                                            "who never try hard anything."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "It's very same for a smith job.",
                                            "If you don't try to",
                                            "upgrade your skill,",
                                            "your works will be poor",
                                            "forever.."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "But there's different way",
                                            "to blaim on your skill.",
                                            "When you face the higher wall,",
                                            "you can feel that you're not enough yet.",
                                            "Then you can reproch yourself and your skills.",
                                            "It's natural."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "After that ordeal, if you try harder",
                                            "and make your goal higher,",
                                            "every people around you will",
                                            "look up to you."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["Well, I hope", "you like them~"])?;
                                    let subject26 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject26 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject26 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject26 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched23 && subject23.loosely_equals(&Val::from(4)) {
                                    matched23 = true;
                                }
                                if matched23 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        } else if ctx.var("ein_cook").get()? == 9 {
                            ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                            ctx.next()?;
                            'b27: {
                                let subject27 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Heart:Ingredients:Skills:Tools:Cancel.")],
                                )?);
                                let mut matched27 = false;
                                let no_case27 = !subject27.loosely_equals(&Val::from(1))
                                    && !subject27.loosely_equals(&Val::from(2))
                                    && !subject27.loosely_equals(&Val::from(3))
                                    && !subject27.loosely_equals(&Val::from(4))
                                    && !subject27.loosely_equals(&Val::from(5));
                                if !matched27 && subject27.loosely_equals(&Val::from(1)) {
                                    matched27 = true;
                                }
                                if matched27 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Ah, heart. Just like forging,",
                                            "you need passion and desire",
                                            "to create something special.",
                                            "Every smith and cook knows that",
                                            "you can do anything if you have",
                                            "the will and the commitment."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that's all",
                                            "I really have to",
                                            "say about that. But",
                                            "come back and chat",
                                            "whenever you please."
                                        ],
                                    )?;
                                    let subject28 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject28 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject28 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject28 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched27 && subject27.loosely_equals(&Val::from(2)) {
                                    matched27 = true;
                                }
                                if matched27 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Now for beginners,",
                                            "learning to select",
                                            "and use ingredients",
                                            "is one of the most",
                                            "important fundamentals."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is like forging",
                                            "since specific ingredients",
                                            "are needed to make specialty",
                                            "items or dishes. You can't just",
                                            "skip them if you really need",
                                            "them, right? Right!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Once you make up your",
                                            "mind to do something,",
                                            "focus on finishing it, 'kay?",
                                            "Never cut corners and always",
                                            "dedicate yourself to make the",
                                            "very best finished product~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject29 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject29 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject29 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject29 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched27 && subject27.loosely_equals(&Val::from(3)) {
                                    matched27 = true;
                                }
                                if matched27 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "When you're beginning to learn",
                                            "skills, you can't let yourself be discouraged! Practice makes",
                                            "perfect, you know? But never",
                                            "use your lack of skills as an",
                                            "excuse if you happen to fail..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["Effort is also an essential", "in forging and cooking! Now,", "on the other hand, if you put", "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "The key is to always",
                                            "give 100% effort and work",
                                            "on improving your skills.",
                                            "Before you know it, you'll",
                                            "be a respected master!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    let subject30 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject30 == 1 {
                                        ctx.mes("Hohohohoho.")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject30 == 2 {
                                        ctx.mes("Umhohohoho.*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject30 == 3 {
                                        ctx.mes("Umho.")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched27 && subject27.loosely_equals(&Val::from(4)) {
                                    matched27 = true;
                                }
                                if matched27 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Cooking is a little",
                                            "easier if you have",
                                            "nicer tools to use,",
                                            "but that's it. Tools by",
                                            "themselves can't make",
                                            "just anybody a master."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "In the end, the best",
                                            "tools are the ones you're",
                                            "most comfortable with using.",
                                            "In fact, I still use the old knife I used back when I was just",
                                            "a little novice chef~"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, while we're on",
                                            "the subject of tools,",
                                            "I just received some",
                                            "new hammers today.",
                                            "Would you like a couple?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    if ctx.call(Function::CheckWeight, vec![Val::from(908), Val::from(50)])? == 0 {
                                        ctx.lines_as(
                                            "Uwe",
                                            args![
                                                "Mmmm...?",
                                                "You're holding too many",
                                                "things. I can't give you any",
                                                "hammers if you don't have",
                                                "room. Hurry, put your extra",
                                                "stuff in your Kafra Storage~"
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    ctx.var("ein_cook").set(Val::from(19))?;
                                    ctx.call(Function::GetItem, vec![Val::from(614), Val::from(1)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(613), Val::from(1)])?;
                                    ctx.next()?;
                                    ctx.lines_as("Uwe", args!["Well, I hope", "you like them~"])?;
                                    let subject31 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                    if subject31 == 1 {
                                        ctx.mes("...Ho ho~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject31 == 2 {
                                        ctx.mes("...*Tee Hee~*")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    } else if subject31 == 3 {
                                        ctx.mes("...Behbie~")?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                                if !matched27 && subject27.loosely_equals(&Val::from(5)) {
                                    matched27 = true;
                                }
                                if matched27 {
                                    ctx.lines_as(
                                        "Uwe",
                                        args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                        }
                    }
                    if !matched10 && subject10.loosely_equals(&Val::from(2)) {
                        matched10 = true;
                    }
                    if matched10 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch was originally",
                                "built to support Einbech's",
                                "mining efforts. Because it's",
                                "small and crowded with people,",
                                "there's no room to build the ore refining factories over there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Since Einbroch used to be",
                                "an empty lot, it was perfect",
                                "for building factories. That's",
                                "what my grandfather told me a",
                                "long time ago. Anyway, Einbroch quickly grew into a major city."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now people think that this",
                                "city was never planned to be",
                                "just an extension of Einbech.",
                                "See that rampart over there?",
                                "It doesn't connect to Einbech at all! No protection for them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "It's like the government",
                                "lost all interest in Einbech.",
                                "Even the miners there have",
                                "been moving here to work in ",
                                "the factories. But more people hasn't made this city more lively."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch may look modern",
                                "and exciting now, but soon",
                                "you'll see that there's no sign",
                                "of warmth or life. So... Just don't live here in your old age."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Then why are you here?:But I like the city life~")])? {
                            1 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, sugar honey,",
                                        "I've been waiting for",
                                        "someone. But... It looks",
                                        "like that person won't be",
                                        "coming back anyway."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oooh, but let's not talk",
                                        "about that. Next time you",
                                        "drop by, we'll talk about",
                                        "something more fun, 'kay?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Big cities can be nice,",
                                        "but you never get to enjoy",
                                        "the sensation of stepping",
                                        "barefoot through the grass,",
                                        "or the magnificent sight of",
                                        "the shining stars at night."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "And the people who live",
                                        "in the cities can be so",
                                        "uptight! I've known more",
                                        "than a few who look down",
                                        "on people from smaller towns."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "It's sad to see some",
                                        "people will always be",
                                        "that ignorant. I... I just",
                                        "can't believe those people",
                                        "can be sooo close minded!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh...!",
                                        "I'm sorry, I guess",
                                        "I went a little overboard.",
                                        "^666666*Titter*^000000 Next time you drop",
                                        "by, we'll talk about something",
                                        "more fun, 'kay? Buhbye~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched10 && subject10.loosely_equals(&Val::from(3)) {
                        matched10 = true;
                    }
                    if matched10 {
                        ctx.lines_as(
                            "Uwe",
                            args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else if (ctx.var("ein_cook").get()?.number()? < 20 || ctx.var("ein_cook").get()? == 219) {
                ctx.lines_as(
                    "Uwe",
                    args!["Oh, welcome back.", "So how's the weapon", "forging coming along?"],
                )?;
                ctx.next()?;
                'b33: {
                    let subject33 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Talk about Cooking:Talk about Einbroch:Talk about Cooking Utensils:Cancel",
                        )],
                    )?);
                    let mut matched33 = false;
                    let no_case33 = !subject33.loosely_equals(&Val::from(1))
                        && !subject33.loosely_equals(&Val::from(2))
                        && !subject33.loosely_equals(&Val::from(3))
                        && !subject33.loosely_equals(&Val::from(4));
                    if !matched33 && subject33.loosely_equals(&Val::from(1)) {
                        matched33 = true;
                    }
                    if matched33 {
                        ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                        ctx.next()?;
                        'b34: {
                            let subject34 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Heart:Ingredients:Skills:Tools:Cancel.")],
                            )?);
                            let mut matched34 = false;
                            let no_case34 = !subject34.loosely_equals(&Val::from(1))
                                && !subject34.loosely_equals(&Val::from(2))
                                && !subject34.loosely_equals(&Val::from(3))
                                && !subject34.loosely_equals(&Val::from(4))
                                && !subject34.loosely_equals(&Val::from(5));
                            if !matched34 && subject34.loosely_equals(&Val::from(1)) {
                                matched34 = true;
                            }
                            if matched34 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Ah, heart. Just like forging,",
                                        "you need passion and desire",
                                        "to create something special.",
                                        "Every smith and cook knows that",
                                        "you can do anything if you have",
                                        "the will and the commitment."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that's all",
                                        "I really have to",
                                        "say about that. But",
                                        "come back and chat",
                                        "whenever you please."
                                    ],
                                )?;
                                let subject35 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject35 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject35 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject35 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched34 && subject34.loosely_equals(&Val::from(2)) {
                                matched34 = true;
                            }
                            if matched34 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Now for beginners,",
                                        "learning to select",
                                        "and use ingredients",
                                        "is one of the most",
                                        "important fundamentals."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Cooking is like forging",
                                        "since specific ingredients",
                                        "are needed to make specialty",
                                        "items or dishes. You can't just",
                                        "skip them if you really need",
                                        "them, right? Right!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Once you make up your",
                                        "mind to do something,",
                                        "focus on finishing it, 'kay?",
                                        "Never cut corners and always",
                                        "dedicate yourself to make the",
                                        "very best finished product~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                let subject36 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject36 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject36 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject36 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched34 && subject34.loosely_equals(&Val::from(3)) {
                                matched34 = true;
                            }
                            if matched34 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "When you're beginning to learn",
                                        "skills, you can't let yourself be discouraged! Practice makes",
                                        "perfect, you know? But never",
                                        "use your lack of skills as an",
                                        "excuse if you happen to fail..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Effort is also an essential",
                                        "in forging and cooking! Now,",
                                        "on the other hand, if you put",
                                        "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "The key is to always",
                                        "give 100% effort and work",
                                        "on improving your skills.",
                                        "Before you know it, you'll",
                                        "be a respected master!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                let subject37 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject37 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject37 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject37 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched34 && subject34.loosely_equals(&Val::from(4)) {
                                matched34 = true;
                            }
                            if matched34 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Cooking is a little",
                                        "easier if you have",
                                        "nicer tools to use,",
                                        "but that's it. Tools by",
                                        "themselves can't make",
                                        "just anybody a master."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "In the end, the best",
                                        "tools are the ones you're",
                                        "most comfortable with using.",
                                        "In fact, I still use the old knife I used back when I was just",
                                        "a little novice chef~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                let subject38 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject38 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject38 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject38 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched34 && subject34.loosely_equals(&Val::from(5)) {
                                matched34 = true;
                            }
                            if matched34 {
                                ctx.lines_as(
                                    "Uwe",
                                    args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched33 && subject33.loosely_equals(&Val::from(2)) {
                        matched33 = true;
                    }
                    if matched33 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch was originally",
                                "built to support Einbech's",
                                "mining efforts. Because it's",
                                "small and crowded with people,",
                                "there's no room to build the ore refining factories over there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Since Einbroch used to be",
                                "an empty lot, it was perfect",
                                "for building factories. That's",
                                "what my grandfather told me a",
                                "long time ago. Anyway, Einbroch quickly grew into a major city."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now people think that this",
                                "city was never planned to be",
                                "just an extension of Einbech.",
                                "See that rampart over there?",
                                "It doesn't connect to Einbech at all! No protection for them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "It's like the government",
                                "lost all interest in Einbech.",
                                "Even the miners there have",
                                "been moving here to work in ",
                                "the factories. But more people hasn't made this city more lively."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch may look modern",
                                "and exciting now, but soon",
                                "you'll see that there's no sign",
                                "of warmth or life. So... Just don't live here in your old age."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Then why are you here?:But I like the city life~")])? {
                            1 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, sugar honey,",
                                        "I've been waiting for someone. But... It looks like that person won't be coming back anyway."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Uwe", args!["Oooh, but let's not talk about that. Next time you drop by, we'll talk about something more fun, 'kay? Buhbye~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as("Uwe", args!["Well, big cities are nice, but you never get to enjoy the sensation of stepping barefoot through the grass, or the clear sight of the stars at night."])?;
                                ctx.next()?;
                                ctx.lines_as("Uwe", args!["And the people who live in cities can be so uptight! I've known more than a few who look down on people who live in smaller towns."])?;
                                ctx.next()?;
                                ctx.lines_as("Uwe", args!["It's sad to see some people will always be that ignorant. I... I just can't believe people can be sooo close minded!"])?;
                                ctx.next()?;
                                ctx.lines_as("Uwe", args!["Oh...!", "I'm sorry, I guess I went a little overboard. ^666666*Titter*^000000 Next time you drop by, we'll talk about something more fun, 'kay? Buhbye~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched33 && subject33.loosely_equals(&Val::from(3)) {
                        matched33 = true;
                    }
                    if matched33 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Cooking utensils?",
                                "Well, they're usually",
                                "made out of ^0000FFJubilee^000000,",
                                "a crystallization dropped",
                                "from mineral monsters."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Jubilee is too soft",
                                "to make weapons, but",
                                "it conducts heat very well",
                                "so it's perfect for making",
                                "cooking utensils! *Tee hee~*"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Uwe", args!["That reminds me...", "I need to make some new cooking utensils soon. Would you do me a favor and let me have some of your materials? Pretty pleeeeeease~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "If you don't mind,",
                                "would you please give me",
                                "30 ^0000FFJubilee^000000 and 4 ^0000FFLarge Jellopy^000000.",
                                "Then, I'll trade you 1 Coal for all of those. How does it sound?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure.:No, thanks.")])? {
                            1 => {
                                if (ctx.call(Function::CountItem, vec![Val::from(7312)])?.number()? > 29
                                    && ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 3)
                                {
                                    ctx.call(Function::DelItem, vec![Val::from(7312), Val::from(30)])?;
                                    ctx.call(Function::DelItem, vec![Val::from(7126), Val::from(4)])?;
                                    ctx.call(Function::GetItem, vec![Val::from(1003), Val::from(1)])?;
                                    ctx.lines_as("Uwe", args!["Hooray!", "Thank you", "^EEA9B8soooo^000000 much!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.lines_as("Uwe", args!["Hmmm...?", "You don't have", "enough of them?"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh well, don't",
                                            "worry. You can take",
                                            "your time and bring",
                                            "that stuff whenever",
                                            " you can, 'kay?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            2 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "I understand.",
                                        "You might not have all",
                                        "that Jubilee and Large Jellopy",
                                        "I'm asking for at the moment,",
                                        "anyway. But if you do get them, would visit me again, cutie?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched33 && subject33.loosely_equals(&Val::from(4)) {
                        matched33 = true;
                    }
                    if matched33 {
                        ctx.lines_as(
                            "Uwe",
                            args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else if ctx.var("ein_cook").get()?.number()? > 299 {
                if ctx.var("Class").get()?.loosely_equals(&ctx.constant("JOB_WHITESMITH")?) {
                    ctx.lines_as(
                        "Uwe",
                        args!["Oooh...!", "Congratulations!", "You finally became", "a cutie MasterSmith!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Mmmm...",
                            "I know!",
                            "Let's celebrate!",
                            "Here sugar honey,",
                            "have some of these",
                            "cookies I just baked~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("ein_cook").set(Val::from(9))?;
                    ctx.call(Function::GetItem, vec![Val::from(538), Val::from(5)])?;
                    ctx.lines_as("Uwe", args!["Okay, enjoy!", "I hope you have", "a good time smithing!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Hmmm?",
                            "Oh, it's you~",
                            "I'm sorry, I was",
                            "just thinking about",
                            "something but now I just",
                            "lost my train of thought."
                        ],
                    )?;
                    ctx.var("ein_cook").set(Val::from(219))?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "What was I going",
                            "to say? Hmm, I can't",
                            "quite remember. Oh well,",
                            "I'll see you later, cutie~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    } else {
        l_aaa = (ctx.var("ein_cook").get()?.try_rem(Val::from(2))?);
        if l_aaa.clone() == 1 {
            l_bbb = ctx.var("ein_cook").get()?;
            if l_bbb.clone().number()? > 99 {
                l_bbb = (l_bbb.clone().try_sub(Val::from(100))?);
                if l_bbb.clone().number()? > 99 {
                    l_bbb = (l_bbb.clone().try_sub(Val::from(100))?);
                    if l_bbb.clone().number()? > 99 {
                        l_ccc = Val::from(3);
                    } else {
                        l_ccc = Val::from(2);
                    }
                } else {
                    l_ccc = Val::from(1);
                }
            } else {
                l_ccc = Val::from(0);
            }
            if l_ccc.clone().number()? < 3 {
                ctx.lines_as(
                    "Uwe",
                    args![
                        "Hm...?",
                        "I never forget a face,",
                        "but somehow I feel like",
                        "we've met somewhere before..."
                    ],
                )?;
                ctx.next()?;
                if l_ccc.clone() == 0 {
                    ctx.var("ein_cook").set((ctx.var("ein_cook").get()? + Val::from(300)))?;
                } else if l_ccc.clone() == 2 {
                    ctx.var("ein_cook").set((ctx.var("ein_cook").get()? + Val::from(100)))?;
                }
                ctx.lines_as(
                    "Uwe",
                    args![
                        "Ah...!",
                        "I have it!",
                        ((Val::from("") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("!")),
                        "Yes, now I remember you.",
                        "Decided to transcend, eh?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Uwe", args!["What brings to"])?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.mes("me, you ^EEA9B8naughty boy^000000?")?;
                } else {
                    ctx.mes("me, you ^EEA9B8naughty girl^000000?")?;
                }
                ctx.next()?;
            } else if l_ccc.clone() == 3 {
                ctx.lines_as("Uwe", args!["Ah, hello again,", "cutie adventurer.", "How can I help you?"])?;
                ctx.next()?;
            }
            'b41: {
                let subject41 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Ask about Einbroch.:Ask about Cooking.:Cancel.")],
                )?);
                let mut matched41 = false;
                let no_case41 = !subject41.loosely_equals(&Val::from(1))
                    && !subject41.loosely_equals(&Val::from(2))
                    && !subject41.loosely_equals(&Val::from(3));
                if !matched41 && subject41.loosely_equals(&Val::from(1)) {
                    matched41 = true;
                }
                if matched41 {
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Einbroch was originally",
                            "built to support Einbech's",
                            "mining efforts. Because it's",
                            "small and crowded with people,",
                            "there's no room to build the ore refining factories over there."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Since Einbroch used to be",
                            "an empty lot, it was perfect",
                            "for building factories. That's",
                            "what my grandfather told me a",
                            "long time ago. Anyway, Einbroch quickly grew into a major city."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Now people think that this",
                            "city was never planned to be",
                            "just an extension of Einbech.",
                            "See that rampart over there?",
                            "It doesn't connect to Einbech at all! No protection for them..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "It's like the government",
                            "lost all interest in Einbech.",
                            "Even the miners there have",
                            "been moving here to work in ",
                            "the factories. But more people hasn't made this city more lively."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Einbroch may look modern",
                            "and exciting now, but soon",
                            "you'll see that there's no sign",
                            "of warmth or life. So... Just don't live here in your old age."
                        ],
                    )?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Then why are you here?:I won't. Thanks for the advice.")])? {
                        1 => {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, I have some",
                                    "precious memories of",
                                    "this place. Once, there was",
                                    "a man who lived here who",
                                    "was just like a father to me."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oh, but I'm sure that",
                                    "you don't want to hear",
                                    "about that. Next time you",
                                    "drop by, we'll talk about",
                                    "something more fun, 'kay?",
                                    "Buhbye for now, cutie~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        2 => {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oh, that is such",
                                    "a good decision,",
                                    "sugar honey! Oh, you",
                                    "cutie adventurers are",
                                    "so precious, so lovable.",
                                    "^333333*Tee hee hee~*^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, that was",
                                    "a fun talk. Come",
                                    "back and visit, 'kay?",
                                    "I'll miss you until",
                                    "the next time~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                if !matched41 && subject41.loosely_equals(&Val::from(2)) {
                    matched41 = true;
                }
                if matched41 {
                    ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                    ctx.next()?;
                    match runtime::select_values(ctx, &[Val::from("Heart:Ingredients:Skills:Tools:Cancel.")])? {
                        1 => {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Ah, heart. Just like forging,",
                                    "you need passion and desire",
                                    "to create something special.",
                                    "Every smith and cook knows that",
                                    "you can do anything if you have",
                                    "the will and the commitment."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, that's all",
                                    "I really have to",
                                    "say about that. But",
                                    "come back and chat",
                                    "whenever you please."
                                ],
                            )?;
                        }
                        2 => {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Now for beginners,",
                                    "learning to select",
                                    "and use ingredients",
                                    "is one of the most",
                                    "important fundamentals."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Cooking is like forging",
                                    "since specific ingredients",
                                    "are needed to make specialty",
                                    "items or dishes. You can't just",
                                    "skip them if you really need",
                                    "them, right? Right!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Once you make up your",
                                    "mind to do something,",
                                    "focus on finishing it, 'kay?",
                                    "Never cut corners and always",
                                    "dedicate yourself to make the",
                                    "very best finished product~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, that was",
                                    "a fun talk. Come",
                                    "back and visit, 'kay?",
                                    "I'll miss you until",
                                    "the next time~"
                                ],
                            )?;
                        }
                        3 => {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "When you're beginning to learn",
                                    "skills, you can't let yourself be discouraged! Practice makes",
                                    "perfect, you know? But never",
                                    "use your lack of skills as an",
                                    "excuse if you happen to fail..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Effort is also an essential",
                                    "in forging and cooking! Now,",
                                    "on the other hand, if you put",
                                    "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "The key is to always",
                                    "give 100% effort and work",
                                    "on improving your skills.",
                                    "Before you know it, you'll",
                                    "be a respected master!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, that was",
                                    "a fun talk. Come",
                                    "back and visit, 'kay?",
                                    "I'll miss you until",
                                    "the next time~"
                                ],
                            )?;
                        }
                        4 => {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Cooking is a little",
                                    "easier if you have",
                                    "nicer tools to use,",
                                    "but that's it. Tools by",
                                    "themselves can't make",
                                    "just anybody a master."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "In the end, the best",
                                    "tools are the ones you're",
                                    "most comfortable with using.",
                                    "In fact, I still use the old knife I used back when I was just",
                                    "a little novice chef~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well, that was",
                                    "a fun talk. Come",
                                    "back and visit, 'kay?",
                                    "I'll miss you until",
                                    "the next time~"
                                ],
                            )?;
                        }
                        5 => {
                            ctx.lines_as(
                                "Uwe",
                                args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                    let subject44 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                    if subject44 == 1 {
                        ctx.mes("...Ho ho~")?;
                    } else if subject44 == 2 {
                        ctx.mes("...*Tee Hee~*")?;
                    } else if subject44 == 3 {
                        ctx.mes("...Behbie~")?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched41 && subject41.loosely_equals(&Val::from(3)) {
                    matched41 = true;
                }
                if matched41 {
                    ctx.lines_as(
                        "Uwe",
                        args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        if ctx.var("ein_cook").get()?.number()? > 999 {
            l_eee = (ctx.var("ein_cook").get()?.try_sub(Val::from(1000))?);
            if l_eee.clone().number()? > 199 {
                ctx.lines_as("Uwe", args!["Ah, hello again,", "cutie adventurer.", "How can I help you?"])?;
                ctx.next()?;
                'b45: {
                    let subject45 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Talk about Einbroch.:Ask him to forge a Weapon.:Talk about Cooking.:Cancel.",
                        )],
                    )?);
                    let mut matched45 = false;
                    let no_case45 = !subject45.loosely_equals(&Val::from(1))
                        && !subject45.loosely_equals(&Val::from(2))
                        && !subject45.loosely_equals(&Val::from(3))
                        && !subject45.loosely_equals(&Val::from(4));
                    if !matched45 && subject45.loosely_equals(&Val::from(1)) {
                        matched45 = true;
                    }
                    if matched45 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch was originally",
                                "built to support Einbech's",
                                "mining efforts. Because it's",
                                "small and crowded with people,",
                                "there's no room to build the ore refining factories over there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Since Einbroch used to be",
                                "an empty lot, it was perfect",
                                "for building factories. That's",
                                "what my grandfather told me a",
                                "long time ago. Anyway, Einbroch quickly grew into a major city."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now people think that this",
                                "city was never planned to be",
                                "just an extension of Einbech.",
                                "See that rampart over there?",
                                "It doesn't connect to Einbech at all! No protection for them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "It's like the government",
                                "lost all interest in Einbech.",
                                "Even the miners there have",
                                "been moving here to work in ",
                                "the factories. But more people hasn't made this city more lively."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch may look modern",
                                "and exciting now, but soon",
                                "you'll see that there's no sign",
                                "of warmth or life. So... Just don't live here in your old age."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Then why are you here?:I won't. Thanks for the advice.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, I have some",
                                        "precious memories of",
                                        "this place. Once, there was",
                                        "a man who lived here who",
                                        "was just like a father to me."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, but I'm sure that",
                                        "you don't want to hear",
                                        "about that. Next time you",
                                        "drop by, we'll talk about",
                                        "something more fun, 'kay?",
                                        "Buhbye for now, cutie~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, that is such",
                                        "a good decision,",
                                        "sugar honey! Oh, you",
                                        "cutie adventurers are",
                                        "so precious, so lovable.",
                                        "^333333*Tee hee hee~*^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched45 && subject45.loosely_equals(&Val::from(2)) {
                        matched45 = true;
                    }
                    if matched45 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Well...",
                                "I actually just",
                                "do smithing work",
                                "to create my own",
                                "cooking tools."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Oh, I understand",
                                "that somebody needs",
                                "to fight the monsters,",
                                "but I'm the wrong person",
                                "to ask for forging weapons.",
                                "I... am a strict pacifist~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Just go look",
                                "around for a little",
                                "bit, I'm sure you'll",
                                "find a Blacksmith",
                                "who's willing to forge",
                                "you a good weapon~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched45 && subject45.loosely_equals(&Val::from(3)) {
                        matched45 = true;
                    }
                    if matched45 {
                        ctx.lines_as("Uwe", args!["Ooh, cooking!", "So what did you", "want to ask me...?"])?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Heart:Ingredients:Skills:Tools:Cancel.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Ah, heart. Just like forging,",
                                        "you need passion and desire",
                                        "to create something special.",
                                        "Every smith and cook knows that",
                                        "you can do anything if you have",
                                        "the will and the commitment."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that's all",
                                        "I really have to",
                                        "say about that. But",
                                        "come back and chat",
                                        "whenever you please."
                                    ],
                                )?;
                            }
                            2 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Now for beginners,",
                                        "learning to select",
                                        "and use ingredients",
                                        "is one of the most",
                                        "important fundamentals."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Cooking is like forging",
                                        "since specific ingredients",
                                        "are needed to make specialty",
                                        "items or dishes. You can't just",
                                        "skip them if you really need",
                                        "them, right? Right!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Once you make up your",
                                        "mind to do something,",
                                        "focus on finishing it, 'kay?",
                                        "Never cut corners and always",
                                        "dedicate yourself to make the",
                                        "very best finished product~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                            }
                            3 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "When you're beginning to learn",
                                        "skills, you can't let yourself be discouraged! Practice makes",
                                        "perfect, you know? But never",
                                        "use your lack of skills as an",
                                        "excuse if you happen to fail..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Effort is also an essential",
                                        "in forging and cooking! Now,",
                                        "on the other hand, if you put",
                                        "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "The key is to always",
                                        "give 100% effort and work",
                                        "on improving your skills.",
                                        "Before you know it, you'll",
                                        "be a respected master!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                            }
                            4 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Cooking is a little",
                                        "easier if you have",
                                        "nicer tools to use,",
                                        "but that's it. Tools by",
                                        "themselves can't make",
                                        "just anybody a master."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "In the end, the best",
                                        "tools are the ones you're",
                                        "most comfortable with using.",
                                        "In fact, I still use the old knife I used back when I was just",
                                        "a little novice chef~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                            }
                            5 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args!["Oh...?", "Well, feel", "free to visit", "me whenever", "you want, 'kay?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                        let subject48 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if subject48 == 1 {
                            ctx.mes("...Ho ho~")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject48 == 2 {
                            ctx.mes("...*Tee Hee~*")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject48 == 3 {
                            ctx.mes("...Behbie~")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched45 && subject45.loosely_equals(&Val::from(4)) {
                        matched45 = true;
                    }
                    if matched45 {
                        ctx.lines_as("Uwe", args!["Alright then,", "sugar honey.", "Take care~", "Hohohohoho~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else if l_eee.clone().number()? > 99 {
                ctx.lines_as(
                    "Uwe",
                    args![
                        "Oh hello hello~",
                        "It's been a long",
                        "time since we've talked,",
                        "you cutie adventurer~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Uwe",
                    args![
                        "So, sugar honey,",
                        "how is it going with",
                        "the little favor I asked",
                        "you about last time?",
                        "Did you already forget",
                        "the 6 Large Jellopy?"
                    ],
                )?;
                ctx.next()?;
                'b49: {
                    let subject49 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Talk about Einbroch.:Ask him to forge a Weapon.:Give him the Materials.:Cancel.",
                        )],
                    )?);
                    let mut matched49 = false;
                    let no_case49 = !subject49.loosely_equals(&Val::from(1))
                        && !subject49.loosely_equals(&Val::from(2))
                        && !subject49.loosely_equals(&Val::from(3))
                        && !subject49.loosely_equals(&Val::from(4));
                    if !matched49 && subject49.loosely_equals(&Val::from(1)) {
                        matched49 = true;
                    }
                    if matched49 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch was originally",
                                "built to support Einbech's",
                                "mining efforts. Because it's",
                                "small and crowded with people,",
                                "there's no room to build the ore refining factories over there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Since Einbroch used to be",
                                "an empty lot, it was perfect",
                                "for building factories. That's",
                                "what my grandfather told me a",
                                "long time ago. Anyway, Einbroch quickly grew into a major city."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now people think that this",
                                "city was never planned to be",
                                "just an extension of Einbech.",
                                "See that rampart over there?",
                                "It doesn't connect to Einbech at all! No protection for them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "It's like the government",
                                "lost all interest in Einbech.",
                                "Even the miners there have",
                                "been moving here to work in ",
                                "the factories. But more people hasn't made this city more lively."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch may look modern",
                                "and exciting now, but soon",
                                "you'll see that there's no sign",
                                "of warmth or life. So... Just don't live here in your old age."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Then why are you here?:I won't. Thanks for the advice.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, I have some",
                                        "precious memories of",
                                        "this place. Once, there was",
                                        "a man who lived here who",
                                        "was just like a father to me."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, but I'm sure that",
                                        "you don't want to hear",
                                        "about that. Next time you",
                                        "drop by, we'll talk about",
                                        "something more fun, 'kay?",
                                        "Buhbye for now, cutie~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, that is such",
                                        "a good decision,",
                                        "sugar honey! Oh, you",
                                        "cutie adventurers are",
                                        "so precious, so lovable.",
                                        "^333333*Tee hee hee~*^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched49 && subject49.loosely_equals(&Val::from(2)) {
                        matched49 = true;
                    }
                    if matched49 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Well...",
                                "I actually just",
                                "do smithing work",
                                "to create my own",
                                "cooking tools."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Oh, I understand",
                                "that somebody needs",
                                "to fight the monsters,",
                                "but I'm the wrong person",
                                "to ask for forging weapons.",
                                "I... am a strict pacifist~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Just go look",
                                "around for a little",
                                "bit, I'm sure you'll",
                                "find a Blacksmith",
                                "who's willing to forge",
                                "you a good weapon~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched49 && subject49.loosely_equals(&Val::from(3)) {
                        matched49 = true;
                    }
                    if matched49 {
                        if ctx.call(Function::CountItem, vec![Val::from(7126)])?.number()? > 5 {
                            ctx.lines_as("Uwe", args!["Thank you ^EEA9B8so^000000 much!", "Here's the Coal I promised~"])?;
                            ctx.call(Function::DelItem, vec![Val::from(7126), Val::from(6)])?;
                            ctx.var("ein_cook").set((ctx.var("ein_cook").get()? + Val::from(100)))?;
                            ctx.call(Function::GetItem, vec![Val::from(1003), Val::from(1)])?;
                            ctx.next()?;
                        } else {
                            ctx.lines_as("Uwe", args!["Huh...?"])?;
                            if ctx.call(Function::CountItem, vec![Val::from(7126)])? == 0 {
                                ctx.lines(args!["You brought", "none at all...?"])?;
                            } else {
                                ctx.lines(args!["This isn't enough", "Large Jellopy...!"])?;
                            }
                            ctx.lines(args![
                                "Next time, be sure",
                                "to bring 6 Large Jellopy,",
                                "okay? Don't forget, cutie~"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now, I can't teach you everything about cooking, but I will give you some good advice for beginners.",
                                "I hope you pay attention, sugar",
                                "honey. Now what would you like",
                                "to hear more about? Hmm...?"
                            ],
                        )?;
                        ctx.next()?;
                        'b51: {
                            let subject51 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Heart:Materials:Skills:Tools:Cancel.")],
                            )?);
                            let mut matched51 = false;
                            let no_case51 = !subject51.loosely_equals(&Val::from(1))
                                && !subject51.loosely_equals(&Val::from(2))
                                && !subject51.loosely_equals(&Val::from(3))
                                && !subject51.loosely_equals(&Val::from(4))
                                && !subject51.loosely_equals(&Val::from(5));
                            if !matched51 && subject51.loosely_equals(&Val::from(1)) {
                                matched51 = true;
                            }
                            if matched51 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Ah, heart. Just like forging,",
                                        "you need passion and desire",
                                        "to create something special.",
                                        "Every smith and cook knows that",
                                        "you can do anything if you have",
                                        "the will and the commitment."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that's all",
                                        "I really have to",
                                        "say about that. But",
                                        "come back and chat",
                                        "whenever you please."
                                    ],
                                )?;
                                let subject52 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject52 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject52 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject52 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched51 && subject51.loosely_equals(&Val::from(2)) {
                                matched51 = true;
                            }
                            if matched51 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Now for beginners,",
                                        "learning to select",
                                        "and use ingredients",
                                        "is one of the most",
                                        "important fundamentals."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Cooking is like forging",
                                        "since specific ingredients",
                                        "are needed to make specialty",
                                        "items or dishes. You can't just",
                                        "skip them if you really need",
                                        "them, right? Right!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Once you make up your",
                                        "mind to do something,",
                                        "focus on finishing it, 'kay?",
                                        "Never cut corners and always",
                                        "dedicate yourself to make the",
                                        "very best finished product~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                let subject53 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject53 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject53 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject53 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched51 && subject51.loosely_equals(&Val::from(3)) {
                                matched51 = true;
                            }
                            if matched51 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "When you're beginning to learn",
                                        "skills, you can't let yourself be discouraged! Practice makes",
                                        "perfect, you know? But never",
                                        "use your lack of skills as an",
                                        "excuse if you happen to fail..."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Effort is also an essential",
                                        "in forging and cooking! Now,",
                                        "on the other hand, if you put",
                                        "in all the effort but didn't learn any of the skills, you'll still get nowhere fast, right? Right!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "The key is to always",
                                        "give 100% effort and work",
                                        "on improving your skills.",
                                        "Before you know it, you'll",
                                        "be a respected master!"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                let subject54 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject54 == 1 {
                                    ctx.mes("Hohohohoho.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject54 == 2 {
                                    ctx.mes("Umhohohoho.*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject54 == 3 {
                                    ctx.mes("Umho.")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched51 && subject51.loosely_equals(&Val::from(4)) {
                                matched51 = true;
                            }
                            if matched51 {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Cooking is a little",
                                        "easier if you have",
                                        "nicer tools to use,",
                                        "but that's it. Tools by",
                                        "themselves can't make",
                                        "just anybody a master."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "In the end, the best",
                                        "tools are the ones you're",
                                        "most comfortable with using.",
                                        "In fact, I still use the old knife I used back when I was just",
                                        "a little novice chef~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                let subject55 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                if subject55 == 1 {
                                    ctx.mes("...Ho ho~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject55 == 2 {
                                    ctx.mes("...*Tee Hee~*")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else if subject55 == 3 {
                                    ctx.mes("...Behbie~")?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                            }
                            if !matched51 && subject51.loosely_equals(&Val::from(5)) {
                                matched51 = true;
                            }
                            if matched51 {
                                ctx.lines_as("Uwe", args!["Alright then,", "sugar honey.", "Take care~", "Hohohohoho~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        break 'b49;
                    }
                    if !matched49 && subject49.loosely_equals(&Val::from(4)) {
                        matched49 = true;
                    }
                    if matched49 {
                        ctx.lines_as("Uwe", args!["Alright then,", "sugar honey.", "Take care~", "Hohohohoho~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            } else {
                ctx.lines_as("Uwe", args!["Hello, cutie~", "How can I help you?"])?;
                ctx.next()?;
                'b56: {
                    let subject56 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from(
                            "Talk about Einbroch.:Ask him to forge a Weapon.:Master, I want to learn cooking.:Cancel.",
                        )],
                    )?);
                    let mut matched56 = false;
                    let no_case56 = !subject56.loosely_equals(&Val::from(1))
                        && !subject56.loosely_equals(&Val::from(2))
                        && !subject56.loosely_equals(&Val::from(3))
                        && !subject56.loosely_equals(&Val::from(4));
                    if !matched56 && subject56.loosely_equals(&Val::from(1)) {
                        matched56 = true;
                    }
                    if matched56 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch was originally",
                                "built to support Einbech's",
                                "mining efforts. Because it's",
                                "small and crowded with people,",
                                "there's no room to build the ore refining factories over there."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Since Einbroch used to be",
                                "an empty lot, it was perfect",
                                "for building factories. That's",
                                "what my grandfather told me a",
                                "long time ago. Anyway, Einbroch quickly grew into a major city."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now people think that this",
                                "city was never planned to be",
                                "just an extension of Einbech.",
                                "See that rampart over there?",
                                "It doesn't connect to Einbech at all! No protection for them..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "It's like the government",
                                "lost all interest in Einbech.",
                                "Even the miners there have",
                                "been moving here to work in ",
                                "the factories. But more people hasn't made this city more lively."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Einbroch may look modern",
                                "and exciting now, but soon",
                                "you'll see that there's no sign",
                                "of warmth or life. So... Just don't live here in your old age."
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Then why are you here?:I won't. Thanks for the advice.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, I have some",
                                        "precious memories of",
                                        "this place. Once, there was",
                                        "a man who lived here who",
                                        "was just like a father to me."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, but I'm sure that",
                                        "you don't want to hear",
                                        "about that. Next time you",
                                        "drop by, we'll talk about",
                                        "something more fun, 'kay?",
                                        "Buhbye for now, cutie~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Oh, that is such",
                                        "a good decision,",
                                        "sugar honey! Oh, you",
                                        "cutie adventurers are",
                                        "so precious, so lovable.",
                                        "^333333*Tee hee hee~*^000000"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Uwe",
                                    args![
                                        "Well, that was",
                                        "a fun talk. Come",
                                        "back and visit, 'kay?",
                                        "I'll miss you until",
                                        "the next time~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                    if !matched56 && subject56.loosely_equals(&Val::from(2)) {
                        matched56 = true;
                    }
                    if matched56 {
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Well...",
                                "I actually just",
                                "do smithing work",
                                "to create my own",
                                "cooking tools."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Oh, I understand",
                                "that somebody needs",
                                "to fight the monsters,",
                                "but I'm the wrong person",
                                "to ask for forging weapons.",
                                "I... am a strict pacifist~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Just go look",
                                "around for a little",
                                "bit, I'm sure you'll",
                                "find a Blacksmith",
                                "who's willing to forge",
                                "you a good weapon~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched56 && subject56.loosely_equals(&Val::from(3)) {
                        matched56 = true;
                    }
                    if matched56 {
                        ctx.lines_as(
                            "Uwe",
                            args!["Mm...?", "Did you just", "say that you", "want to learn", "the art of cooking?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "I'm sorry, but I can't",
                                "really give culinary",
                                "lessons. But I will",
                                "give good advice for",
                                "hopeful beginners."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Now...",
                                "For your tuition",
                                "I'll need-- Gosh,",
                                "there's just so many",
                                "things. Get some paper,",
                                "and a pen for this list..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.var("ein_cook").set((ctx.var("ein_cook").get()? + Val::from(100)))?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Just kidding!",
                                "^333333*Titter~*^000000 I don't need",
                                "much to make some",
                                "cooking utensils. Bring",
                                "6 ^0000FFLarge Jellopy^000000. That's it!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "In return, I will give you",
                                "1 Coal and some useful",
                                "cooking advice for novices.",
                                "I give this advice for free to",
                                "my smithing colleages, though..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Uwe",
                            args![
                                "Why ^EEA9B8don't^000000 you",
                                "become a Blacksmith?",
                                "I'm much more confident",
                                "in that field. ^333333*Tee hee~*^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Uwe", args!["Well, I'll be", "waiting right", "here until you", "come back."])?;
                        let subject58 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                        if subject58 == 1 {
                            ctx.mes("...Ho ho~")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject58 == 2 {
                            ctx.mes("...*Tee Hee~*")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject58 == 3 {
                            ctx.mes("...Behbie~")?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched56 && subject56.loosely_equals(&Val::from(4)) {
                        matched56 = true;
                    }
                    if matched56 {
                        ctx.lines_as("Uwe", args!["Take care,", "cutie adventurer!", "Hohoho!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
            }
        } else if ctx.var("ein_cook").get()? == 0 {
            ctx.lines_as(
                "Uwe",
                args![
                    "Cooking is such a joy~!",
                    "The scents, the flavors, the sensation of sheer ^EEA9B8satiation^000000..."
                ],
            )?;
            ctx.next()?;
            'b59: {
                let subject59 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Um, isn't this a forge?:Ignore him.")],
                )?);
                let mut matched59 = false;
                let no_case59 = !subject59.loosely_equals(&Val::from(1)) && !subject59.loosely_equals(&Val::from(2));
                if !matched59 && subject59.loosely_equals(&Val::from(1)) {
                    matched59 = true;
                }
                if matched59 {
                    ctx.mes("[Uwe]")?;
                    ctx.var("ein_cook").set(Val::from(1000))?;
                    ctx.lines(args![
                        "Is this a forge?",
                        "Oh, sugar honey,",
                        "you haven't been here",
                        "before, haven't you?"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "My name is Uwe Kleine",
                            "and this is my forge~! I am",
                            "the most elegant Blacksmith",
                            "and the best chef here in the",
                            "Schwarzwald Republic~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Uwe", args!["So, how can", "I help you, you", "adooooooooorable"])?;
                    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                        ctx.mes("hunk of a man?")?;
                    } else {
                        ctx.mes("womanly woman?")?;
                    }
                    ctx.next()?;
                    'b60: {
                        let subject60 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from(
                                "Talk about Einbroch.:Ask him to forge a Weapon.:Master, I want to learn cooking.:Cancel.",
                            )],
                        )?);
                        let mut matched60 = false;
                        let no_case60 = !subject60.loosely_equals(&Val::from(1))
                            && !subject60.loosely_equals(&Val::from(2))
                            && !subject60.loosely_equals(&Val::from(3))
                            && !subject60.loosely_equals(&Val::from(4));
                        if !matched60 && subject60.loosely_equals(&Val::from(1)) {
                            matched60 = true;
                        }
                        if matched60 {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Einbroch was originally",
                                    "built to support Einbech's",
                                    "mining efforts. Because it's",
                                    "small and crowded with people,",
                                    "there's no room to build the ore refining factories over there."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Since Einbroch used to be",
                                    "an empty lot, it was perfect",
                                    "for building factories. That's",
                                    "what my grandfather told me a",
                                    "long time ago. Anyway, Einbroch quickly grew into a major city."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Now people think that this",
                                    "city was never planned to be",
                                    "just an extension of Einbech.",
                                    "See that rampart over there?",
                                    "It doesn't connect to Einbech at all! No protection for them..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "It's like the government",
                                    "lost all interest in Einbech.",
                                    "Even the miners there have",
                                    "been moving here to work in ",
                                    "the factories. But more people hasn't made this city more lively."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Einbroch may look modern",
                                    "and exciting now, but soon",
                                    "you'll see that there's no sign",
                                    "of warmth or life. So... Just don't live here in your old age."
                                ],
                            )?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Then why are you here?:I won't. Thanks for the advice.")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, I have some",
                                            "precious memories of",
                                            "this place. Once, there was",
                                            "a man who lived here who",
                                            "was just like a father to me."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, but I'm sure that",
                                            "you don't want to hear",
                                            "about that. Next time you",
                                            "drop by, we'll talk about",
                                            "something more fun, 'kay?",
                                            "Buhbye for now, cutie~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Oh, that is such",
                                            "a good decision,",
                                            "sugar honey! Oh, you",
                                            "cutie adventurers are",
                                            "so precious, so lovable.",
                                            "^333333*Tee hee hee~*^000000"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Uwe",
                                        args![
                                            "Well, that was",
                                            "a fun talk. Come",
                                            "back and visit, 'kay?",
                                            "I'll miss you until",
                                            "the next time~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                        if !matched60 && subject60.loosely_equals(&Val::from(2)) {
                            matched60 = true;
                        }
                        if matched60 {
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Well...",
                                    "I actually just",
                                    "do smithing work",
                                    "to create my own",
                                    "cooking tools."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Oh, I understand",
                                    "that somebody needs",
                                    "to fight the monsters,",
                                    "but I'm the wrong person",
                                    "to ask for forging weapons.",
                                    "I... am a strict pacifist~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Just go look",
                                    "around for a little",
                                    "bit, I'm sure you'll",
                                    "find a Blacksmith",
                                    "who's willing to forge",
                                    "you a good weapon~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        if !matched60 && subject60.loosely_equals(&Val::from(3)) {
                            matched60 = true;
                        }
                        if matched60 {
                            ctx.lines_as(
                                "Uwe",
                                args!["Mm...?", "Did you just", "say that you", "want to learn", "the art of cooking?"],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "I'm sorry, but I can't",
                                    "really give culinary",
                                    "lessons. But I will",
                                    "give good advice for",
                                    "hopeful beginners."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Now...",
                                    "For your tuition",
                                    "I'll need-- Gosh,",
                                    "there's just so many",
                                    "things. Get some paper,",
                                    "and a pen for this list..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.var("ein_cook").set((ctx.var("ein_cook").get()? + Val::from(100)))?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Just kidding!",
                                    "^333333*Titter~*^000000 I don't need",
                                    "much to make some",
                                    "cooking utensils. Bring",
                                    "6 ^0000FFLarge Jellopy^000000. That's it!"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "In return, I will give you",
                                    "1 Coal and some useful",
                                    "cooking advice for novices.",
                                    "I give this advice for free to",
                                    "my smithing colleages, though..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Uwe",
                                args![
                                    "Why ^EEA9B8don't^000000 you",
                                    "become a Blacksmith?",
                                    "I'm much more confident",
                                    "in that field. ^333333*Tee hee~*^000000"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Uwe", args!["Well, I'll be", "waiting right", "here until you", "come back."])?;
                            let subject62 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                            if subject62 == 1 {
                                ctx.mes("...Ho ho~")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if subject62 == 2 {
                                ctx.mes("...*Tee Hee~*")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if subject62 == 3 {
                                ctx.mes("...Behbie~")?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                        if !matched60 && subject60.loosely_equals(&Val::from(4)) {
                            matched60 = true;
                        }
                        if matched60 {
                            ctx.lines_as("Uwe", args!["Take care,", "sugar honey~", "Ho ho ho!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                }
                if !matched59 && subject59.loosely_equals(&Val::from(2)) {
                    matched59 = true;
                }
                if matched59 {
                    ctx.lines_as(
                        "Uwe",
                        args![
                            "Cooking begins with",
                            "fire and ends with fire.",
                            "There's a certain art to",
                            "creating fine, delicious",
                            "foods to delight the palate~"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn uwe_kleine_ein(ctx: &Ctx) -> Script {
    uwe_kleine_ein_body(ctx, Vec::new()).map(|_| ())
}
