#![allow(
    unused_imports,
    unused_labels,
    unused_assignments,
    unused_mut,
    unused_parens,
    unused_variables,
    unreachable_code
)]

use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, constants, runtime};

pub fn trader_val09(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    ctx.lines_as(
        "Marco Bassinio",
        args![
            "Hey, folks! Here's something you don't see everyday!",
            "Something you can never find in Rune-Midgarts!",
            "Something that makes you happy with just one bite!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Marco Bassinio",
        args![
            "It's the perfect dessert and the perfect gift for loved ones.",
            "High-quality, traditional homemade chocolate only 5000z each!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["I'll take it, please!", "I want to wrap the chocolate!", "End trading."])? {
        0 => {
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "Ahaha, my dear.",
                    "This chocolate is nothing like others.",
                    "Every piece bears the devotion of the person who made it!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "So, that's why I can't sell more than 5 of them at a time.",
                    "If you really really want more, then talk to me again.",
                    "How many do you want anyway?"
                ],
            )?;
            ctx.next()?;
            while !(1..=5).contains(&l_input.number()?) {
                l_input = runtime::input_number(ctx, None, None)?.0;
                if l_input.number()? < 1 {
                    ctx.lines_as(
                        "Marco Bassinio",
                        args![
                            "Oh, it's such a shame!",
                            "I'm sure you'll miss this opportunity and regret you didn't buy it."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marco Bassinio",
                        args![
                            "Remember, you can never find this anywhere else!",
                            "Come back anytime, when you change your mind."
                        ],
                    )?;
                    return ctx.close();
                }
                if l_input.number()? > 5 {
                    ctx.lines_as(
                        "Marco Bassinio",
                        args![
                            "Ugh.. Didn't I tell you?",
                            "5 is the maximum!",
                            "I can't sell more than that to the same person."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Marco Bassinio",
                        args![
                            "And you know it's not like an everyday meal.",
                            "Eating too much is not really good for you."
                        ],
                    )?;
                    ctx.next()?;
                }
            }
            if !ctx.call(Function::CheckWeight, args![558, l_input.clone()])?.is_true() {
                ctx.lines_as(
                    "Marco Bassinio",
                    args!["You're carrying too many items.", "Please use the Kafra Services."],
                )?;
                return ctx.close();
            }
            let l_price = l_input.number()? * 5000;
            if ctx.player().zeny()? < l_price {
                ctx.lines_as(
                    "Marco Bassinio",
                    args![
                        "Looks like you don't have enough zeny with ya.",
                        "Maybe you should borrow some zeny from a friend.",
                        "Cuz, I'm not gonna be here everyday."
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "Good for you!",
                    "It's also perfect as a gift!",
                    "You know you can't get this kind of chocolate normally."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "If you want more, you should come back.",
                    "Might be a good idea to buy some more while you have a chance...!"
                ],
            )?;
            ctx.player().set_zeny(ctx.player().zeny()? - l_price)?;
            ctx.call(Function::GetItem, args![558, l_input.clone()])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "If you want to gift-wrap the chocolate, of course, you need chocolate, plus, wrapping paper, wrapping strap and a box."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "You also need to pay 500 zeny to carve your name on the box.",
                    "Are you all prepared...?"
                ],
            )?;
            ctx.next()?;
            if ctx.items().count(7175)? < 1 || ctx.items().count(7174)? < 1 || ctx.items().count(7948)? < 1 || ctx.player().zeny()? < 500 {
                ctx.lines_as(
                    "Marco Bassinio",
                    args![
                        "Hmm.. Looks like you don't have enough materials to decorate the gift box..",
                        "You can't just put your gift into some plain looking box..",
                        "Don't you think?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Marco Bassinio",
                    args![
                        "You need to bring some wrapping paper, wrapping strap, and a box.",
                        "Oh, also bring 500 zeny, and don't forget to bring your true loving heart with you!!"
                    ],
                )?;
                return ctx.close();
            }
            if ctx.items().count(558)? < 1 {
                ctx.lines_as(
                    "Marco Bassinio",
                    args![
                        "Hey, look, adventurer!",
                        "I can't create something right away!",
                        "You know I'm not an alchemist or anything."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Marco Bassinio",
                    args!["You're not saying that you want an empty chocolate box without any chocolate in it, am I right?"],
                )?;
                return ctx.close();
            }
            if !ctx.call(Function::CheckWeight, args![12744, 1])?.is_true() {
                ctx.lines_as(
                    "Marco Bassinio",
                    args!["You're carrying too many items.", "Please use the Kafra Services."],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "Here, look!",
                    "It's your chocolate box with your name on it.",
                    "Isn't it fabulous?",
                    "See, your name looks great on the box!"
                ],
            )?;
            ctx.next()?;
            ctx.items().take(558, 1)?;
            ctx.items().take(7175, 1)?;
            ctx.items().take(7174, 1)?;
            ctx.items().take(7948, 1)?;
            ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
            ctx.call(Function::GetNamedItem, args![12744, ctx.player().name()?])?;
            ctx.lines_as(
                "Marco Bassinio",
                args!["Happy Valentine's Day!", "Valentine's the reason I came back."],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "Oh, it's such a shame!",
                    "I'm sure you'll miss this opportunity and regret you didn't buy it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Marco Bassinio",
                args![
                    "Remember, you can never find this anywhere else!",
                    "Come back anytime, when you change your mind."
                ],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn packs_trader_val09(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    ctx.lines_as(
        "Packs Trader",
        args![
            "Hello.",
            "I am a Packs Trader, I sell paper boxes and supplies for packing presents."
        ],
    )?;
    ctx.next()?;
    loop {
        ctx.lines_as("Packs Trader", args!["Do you have something to buy?"])?;
        ctx.next()?;
        'b2: {
            match ctx.menu(&["Packing Paper", "Packing Ribbon", "Box", "Cancel."])? {
                0 => {
                    ctx.lines_as(
                        "Packs Trader",
                        args![
                            "It's 200 zeny for 1 Packing Paper.",
                            "How many do you want?",
                            "You can't buy more than 10 items at once."
                        ],
                    )?;
                    ctx.next()?;
                    l_input = runtime::input_number(ctx, None, None)?.0;
                    if l_input.number()? <= 0 {
                        ctx.lines_as("Packs Trader", args!["Nothing to buy.", "Come back when you need something."])?;
                        return ctx.close();
                    }
                    if l_input.number()? > 10 {
                        ctx.lines_as("Packs Trader", args!["I told you not to buy more than 10..."])?;
                        ctx.next()?;
                        break 'b2;
                    }
                    if !ctx.call(Function::CheckWeight, args![7175, l_input.clone()])?.is_true() {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["You're carrying too many items.", "Please use the Kafra Services."],
                        )?;
                        return ctx.close();
                    }
                    let l_price = l_input.number()? * 200;
                    if ctx.player().zeny()? < l_price {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["You don't have enough money.", "Please check your wallet."],
                        )?;
                        ctx.next()?;
                        break 'b2;
                    } else {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["Here they are.", "Hope it makes your Valentine's Day more pleasing!"],
                        )?;
                        ctx.player().set_zeny(ctx.player().zeny()? - l_price)?;
                        ctx.call(Function::GetItem, args![7175, l_input.clone()])?;
                        ctx.next()?;
                        break 'b2;
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Packs Trader",
                        args![
                            "It's 200 zeny for 1 Packing Ribbon.",
                            "How many do you want?",
                            "You can't buy more than 10 items at once."
                        ],
                    )?;
                    ctx.next()?;
                    l_input = runtime::input_number(ctx, None, None)?.0;
                    if l_input.number()? <= 0 {
                        ctx.lines_as("Packs Trader", args!["Nothing to buy.", "Come back when you need something."])?;
                        return ctx.close();
                    }
                    if l_input.number()? > 10 {
                        ctx.lines_as("Packs Trader", args!["I told you not to buy more than 10..."])?;
                        ctx.next()?;
                        break 'b2;
                    }
                    if !ctx.call(Function::CheckWeight, args![7174, l_input.clone()])?.is_true() {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["You're carrying too many items.", "Please use the Kafra Services."],
                        )?;
                        return ctx.close();
                    }
                    let l_price = l_input.number()? * 200;
                    if ctx.player().zeny()? < l_price {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["You don't have enough money.", "Please check your wallet."],
                        )?;
                        ctx.next()?;
                        break 'b2;
                    } else {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["Here they are.", "Hope it makes your Valentine's Day more pleasing!"],
                        )?;
                        ctx.player().set_zeny(ctx.player().zeny()? - l_price)?;
                        ctx.call(Function::GetItem, args![7174, l_input.clone()])?;
                        ctx.next()?;
                        break 'b2;
                    }
                }
                2 => {
                    ctx.lines_as(
                        "Packs Trader",
                        args![
                            "It's 600 zeny for 1 Box.",
                            "How many do you want?",
                            "You can't buy more than 10 items at once."
                        ],
                    )?;
                    ctx.next()?;
                    l_input = runtime::input_number(ctx, None, None)?.0;
                    if l_input.number()? <= 0 {
                        ctx.lines_as("Packs Trader", args!["Nothing to buy.", "Come back when you need something."])?;
                        return ctx.close();
                    }
                    if l_input.number()? > 10 {
                        ctx.lines_as("Packs Trader", args!["I told you not to buy more than 10..."])?;
                        ctx.next()?;
                        break 'b2;
                    }
                    if !ctx.call(Function::CheckWeight, args![7948, l_input.clone()])?.is_true() {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["You're carrying too many items.", "Please use the Kafra Services."],
                        )?;
                        return ctx.close();
                    }
                    let l_price = l_input.number()? * 600;
                    if ctx.player().zeny()? < l_price {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["You don't have enough money.", "Please check your wallet."],
                        )?;
                        ctx.next()?;
                        break 'b2;
                    } else {
                        ctx.lines_as(
                            "Packs Trader",
                            args!["Here they are.", "Hope it makes your Valentine's Day more pleasing!"],
                        )?;
                        ctx.player().set_zeny(ctx.player().zeny()? - l_price)?;
                        ctx.call(Function::GetItem, args![7948, l_input.clone()])?;
                        ctx.next()?;
                        break 'b2;
                    }
                }
                3 => {
                    ctx.lines_as("Packs Trader", args!["Goodbye!", "And enjoy your Valentine's Day."])?;
                    return ctx.close();
                }
                _ => {}
            }
        }
    }
}

pub fn event_ring_maker_val09(ctx: &Ctx) -> Script {
    if ctx.player().base_level()? < 75 {
        ctx.lines_as(
            "Event Ring Maker",
            args!["Hello, I only make the Valentine rings to those experienced adventurer Level 75 or above."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Event Ring Maker",
            args![
                "You're not fully experienced yet.",
                "Come back when you're experienced enough to handle the quests."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("iroval09ring").get()?.number()? >= 1 {
        ctx.lines_as(
            "Event Ring Maker",
            args!["The box with the ring, carved with your name, is for the one you love."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Event Ring Maker",
            args![
                "As for the rings that you receive from others, they should all be registered with the Vote Manager.",
                "She is standing near the Prontera Fountain."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Event Ring Maker",
            args![
                "Only the most popular male and female are subjected to getting rewards.",
                "Be aware, and always try to stay popular!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "Hi, there, how are ya?",
            "Come to me if you're interested in the event, 'Who's Valentine's Hottest?'"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "I make the most precious rings that you can give to your sweethearts.",
            "Those rings are very special because I carve your names on them!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "Isn't it exciting?",
            "Isn't it such a brilliant idea?",
            "Give these special rings to your sweethearts!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "You should hurry 'cuz this event will only last for two weeks.",
            "Give that special someone a gift of a Valentine's ring."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "Remember, you can only generate the ring once.",
            "You also need Wrapping Paper, Wrapping Strap and a Box to make the ring."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "So you better be sure of who you give this to.",
            "By the way, It costs 1,000 zeny.",
            "Would you like to make one?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Hmm.. I gotta give it a second thought...", "Sure."])? == 0 {
        ctx.lines_as(
            "Event Ring Maker",
            args![
                "Alrighty!",
                "You can't put a rush on such a thing like this.",
                "Think about what you truly want.",
                "Just follow your heart!"
            ],
        )?;
        return ctx.close();
    }
    if ctx.items().count(7175)? < 1 || ctx.items().count(7174)? < 1 || ctx.items().count(7948)? < 1 || ctx.player().zeny()? < 1000 {
        ctx.lines_as(
            "Event Ring Maker",
            args![
                "Well, you don't have enough materials to make a gift box.",
                "Check what you have, and come back later with all the materials."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Event Ring Maker", args!["Okie Dokie!", "I'll make it right away."])?;
    ctx.next()?;
    ctx.items().take(7175, 1)?;
    ctx.items().take(7174, 1)?;
    ctx.items().take(7948, 1)?;
    ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
    ctx.var("iroval09ring").set(Val::from(1))?;
    let ring = if ctx.var("Sex").get()? == constants::SEX_MALE {
        12742
    } else {
        12743
    };
    ctx.call(Function::GetNamedItem, args![ring, ctx.player().name()?])?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "Here, the most precious ring in the world!",
            "Don't forget, you can never make this ring again."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args!["You must pick out the one that you really really love, and give this ring to that person."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Event Ring Maker",
        args![
            "Of course, you've got to get rings from others, that's the way you can participate in the voting, right?",
            "Challenge yourself to become Valentine's Hottest!"
        ],
    )?;
    ctx.close()
}

pub fn valentine_vote_manager_v(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    ctx.lines_as(
        "Valentine Vote Manager",
        args![
            "Hello, I'm the Valentine's Vote Manager.",
            "I'm in charge of collecting rings for this event!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valentine Vote Manager",
        args!["I register the rings you get from others and I calculate the total number of rings."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valentine Vote Manager",
        args![
            "You know what I do besides just counting those rings?",
            "I can tell you the adventurer's name who's got the most number of votes."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Valentine Vote Manager",
        args!["Plus, you can also find out how many votes he/she got."],
    )?;
    ctx.next()?;
    loop {
        ctx.lines_as("Valentine Vote Manager", args!["So, what do you want?"])?;
        ctx.next()?;
        'b2: {
            match ctx.menu(&["Please register my rings.", "Please count my votes.", "Nothing, for now."])? {
                0 => {
                    ctx.lines_as(
                        "Valentine Vote Manager",
                        args!["Please tell me how many rings you want to register."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Valentine Vote Manager", args!["When you write the number of the rings, the number shouldn't be larger than the number of rings you actually have.", "'0', cancels everything."])?;
                    ctx.next()?;
                    l_input = runtime::input_number(ctx, None, None)?.0;
                    if l_input.number()? <= 0 {
                        ctx.lines_as(
                            "Valentine Vote Manager",
                            args!["You have entered 0.", "Registration is cancelled."],
                        )?;
                        ctx.next()?;
                        break 'b2;
                    }
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        if ctx.items().count(7947)? > l_input.number()? {
                            ctx.lines_as(
                                "Valentine Vote Manager",
                                args![
                                    "Seems like the value you entered is too small.",
                                    "I know you've got more. Be honest, dear."
                                ],
                            )?;
                            ctx.next()?;
                            break 'b2;
                        }
                        if ctx.items().count(7947)? < l_input.number()? {
                            ctx.lines_as(
                                "Valentine Vote Manager",
                                args![
                                    "Seems like the value you entered is too large.",
                                    "I know you've got less. Be honest, dear."
                                ],
                            )?;
                            ctx.next()?;
                            break 'b2;
                        }
                        ctx.lines_as(
                            "Valentine Vote Manager",
                            args![
                                "I'll take those silver rings, and count the votes for you.",
                                "Thank you for participating."
                            ],
                        )?;
                        ctx.call(Function::DelItem, args![7947, l_input.clone()])?;
                        ctx.var("val09rings").set(ctx.var("val09rings").get()? + l_input.clone())?;
                        if ctx.var("val09rings").get()?.number()? > ctx.var("$val09votes_m").get()?.number()? {
                            ctx.var("$val09votes_m").set(ctx.var("val09rings").get()?)?;
                            ctx.var("$val09name_m$").set(ctx.player().name()?)?;
                        }
                        ctx.next()?;
                        break 'b2;
                    } else {
                        if ctx.items().count(7946)? > l_input.number()? {
                            ctx.lines_as(
                                "Valentine Vote Manager",
                                args![
                                    "Seems like the value you entered is too small.",
                                    "I know you've got more. Be honest, dear."
                                ],
                            )?;
                            ctx.next()?;
                            break 'b2;
                        }
                        if ctx.items().count(7946)? < l_input.number()? {
                            ctx.lines_as(
                                "Valentine Vote Manager",
                                args![
                                    "Seems like the value you entered is too large.",
                                    "I know you've got less. Be honest, dear."
                                ],
                            )?;
                            ctx.next()?;
                            break 'b2;
                        }
                        ctx.lines_as(
                            "Valentine Vote Manager",
                            args![
                                "I'll take those gold rings, and count the votes for you.",
                                "Thank you for participating."
                            ],
                        )?;
                        ctx.call(Function::DelItem, args![7946, l_input.clone()])?;
                        ctx.var("val09rings").set(ctx.var("val09rings").get()? + l_input.clone())?;
                        if ctx.var("val09rings").get()?.number()? > ctx.var("$val09votes_f").get()?.number()? {
                            ctx.var("$val09votes_f").set(ctx.var("val09rings").get()?)?;
                            ctx.var("$val09name_f$").set(ctx.player().name()?)?;
                        }
                        ctx.next()?;
                        break 'b2;
                    }
                }
                1 => {
                    ctx.lines_as(
                        "Valentine Vote Manager",
                        args![
                            "Let's see...",
                            Val::from("You have registered....") + ctx.var("val09rings").get()? + Val::from(" rings so far."),
                            "and..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valentine Vote Manager",
                        args![
                            Val::from("The current record shows... ")
                                + ctx.var("$val09name_m$").get()?
                                + Val::from(" is the male vote leader who's registered the total of ")
                                + ctx.var("$val09votes_m").get()?
                                + Val::from(" rings.")
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Valentine Vote Manager",
                        args![
                            Val::from("The current record shows... ")
                                + ctx.var("$val09name_f$").get()?
                                + Val::from(" is the female vote leader who's registered the total of ")
                                + ctx.var("$val09votes_f").get()?
                                + Val::from(" rings.")
                        ],
                    )?;
                    ctx.next()?;
                    break 'b2;
                }
                2 => {
                    ctx.lines_as(
                        "Valentine Vote Manager",
                        args![
                            "Hey, you can be popular too!",
                            "Anyone can... really!",
                            "Though you have to try a lot harder, but still~ Hahaha!"
                        ],
                    )?;
                    return ctx.close();
                }
                _ => {}
            }
        }
    }
}

pub fn dessert_manager_val09(ctx: &Ctx) -> Script {
    if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.lines_as(
            "Charles Orleans",
            args![
                "Monsieur~! What brings you to my beautiful atelier?",
                "What is it that you want?",
                "Well, my sparkling eyes get dried and lose their shine if not for the pretty little lady."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args![
                "Please leave me alone unless you have business with me.",
                "Haaaa~ I'm a busy person.",
                "Don't bother me....",
                "Annoying, annoying, annoying~~!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Charles Orleans",
        args![
            "Oh, Mademoiselle!",
            "This little trifling space felt like heaven the minute you walked in!",
            "Can I help you with anything, if it's alright?"
        ],
    )?;
    ctx.next()?;
    if ctx.menu(&["Please, make me some chocolate.", "Don't bother. I'm just passing by."])? == 1 {
        ctx.lines_as(
            "Charles Orleans",
            args![
                "Ahhh, this is so heartbreaking.",
                "How could you say that?",
                "You're just so mean.",
                "Don't bother? Just passing by?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args![
                "What can I do to make you",
                "pay a little attention to me?",
                "Please.. I feel like my soul is lost.."
            ],
        )?;
        return ctx.close();
    }
    if ctx.items().count(558)? < 3 {
        ctx.lines_as(
            "Charles Orleans",
            args![
                "Ahhh, Mademoiselle.",
                "I'm not an alchemist, or a magician.",
                "I don't just make chocolate out of anything."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args![
                "I always think of making chocolate as artistic work.",
                "You see, I'm no ordinary cook...",
                "I make chocolate with feelings..",
                "messages of loving hearts.."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args![
                "I make masterpieces.",
                "No one can imitate the looks and the taste.",
                "Yes, it's nothing like ordinary chocolate!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args!["I'm afraid I can't make it and prove it to you now.", "This is really a shame!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Charles Orleans", args!["I really want to thank you for visiting me and if you only bring ^3152ff3 Chocolates^000000, I'll make you chocolate like you've never seen..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args![
                "never tasted before...",
                "Mademoiselle, with your spirit, I'm sure you can bring 3 pieces of chocolate.",
                "I have no doubt at all."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Charles Orleans",
            args![
                "So... what do you think?",
                "Can you bring ^3152ff3 Chocolates^000000?",
                "I could get them myself, but I'm tied up with so much work as you see right now."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Charles Orleans", args!["Adios, Mademoiselle.....", "I'll be waiting for you."])?;
        return ctx.close();
    }
    if !ctx.call(Function::CheckWeight, args![559, 1])?.is_true() {
        ctx.lines_as(
            "Charles Orleans",
            args!["You're carrying too many items.", "Please use the Kafra Services."],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Charles Orleans",
        args![
            "Oh, Mademoiselle!",
            "I'll make the best chocolate with the pieces you've brought.",
            "I'm going to put the light of your eyes into this chocolate that no one can resist."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charles Orleans",
        args!["It'll be stronger than a sweet sweet love potion....."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charles Orleans",
        args![
            "Un, Deux, Trois, Quatre.....",
            "Just like the ugly duckling that turned to a beautiful swan-",
            "Ordinary chocolate pieces are becoming a piece of art!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charles Orleans",
        args![
            "They're changing!",
            "They're getting warm, softly changing the shape, getting stronger again!",
            "Oh, is it a master piece or",
            "what...!!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Charles Orleans",
        args![
            "Here you go, Mademoiselle!",
            "Ahahahaha, just look at this!",
            "I can't believe I made this.",
            "Oh, I can't take my eyes off..!!"
        ],
    )?;
    ctx.items().take(558, 3)?;
    ctx.items().give(559, 1)?;
    ctx.next()?;
    ctx.lines_as(
        "Charles Orleans",
        args![
            "Alright. Mademoiselle,",
            "I hope this is just what you wanted, for it bears your beautiful heart inside."
        ],
    )?;
    ctx.close()
}
