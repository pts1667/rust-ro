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

fn vending_machine_man_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Titicupe", args!["Hmm...?", "I, Titicupe, esteemed manager of the Toy Factory and genius inventor, has done it again! I've just created the world's most sophisticated vending machine!"])?;
    ctx.next()?;
    match ctx.menu(&[
        "Talk to him about the Vending Machine",
        "Items in the Vending Machine",
        "Stop talking",
    ])? {
        0 => {
            ctx.lines_as("Titicupe", args!["Can you see the little snowman to the left side of me? Can you? If you look at his mouth closely, you'll see that it's been built quite largely. You might say, a bit too large for normal purposes..."])?;
            ctx.next()?;
            ctx.lines_as(
                "Titicupe",
                args![
                    "You see, that snowman is actually an amazing vending machine that I invented out of my blood, sweat, tears and snow."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Titicupe",
                args![
                    "You can put all sorts of things inside its mouth, and if you insert the right items, you'll get some hats in return..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Titicupe",
                args!["Fashionable hats that you've always dreamed of wearing...! Bwahahahaha~!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Titicupe", args!["Oh, this creation of mine is so magnificent!"])?;
            ctx.npc().emotion(constants::ET_CHUPCHUP)?;
            ctx.next()?;
            ctx.lines_as("Titicupe", args!["And the headgears--! I... I can't contain myself!"])?;
            ctx.next()?;
            ctx.mes("^3355FFToy factory manager Titicupe jumps and convulses with joy. Clearly he's insane, but it may be possible that he may be brilliant.^000000")?;
            ctx.npc().emotion(constants::ET_SURPRISE)?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        1 => {
            ctx.lines_as("Titicupe", args!["Oh right! You need to know what kind of items to put into the Vending Machine to get what you want, yes? Go ahead... Ask me~"])?;
            ctx.npc().emotion(constants::ET_AHA)?;
            ctx.next()?;
            match ctx.menu(&[
                "Raccoon Hat",
                "Spore Hat",
                "Wonder Nutshell",
                "Ranbow Eggshell",
                "Blush",
                "Chef Hat",
                "Candle",
                "Cake Hat",
                "End Conversation",
            ])? {
                0 => {
                    ctx.lines_as(
                        "Titicupe",
                        args![
                            "Raccoon Hat?",
                            "1 Kitty Band",
                            "20 Dragon Scale",
                            "200 Tough Scalelike Stem",
                            "300 Sea-otter Fur"
                        ],
                    )?;
                }
                1 => {
                    ctx.lines_as(
                        "Titicupe",
                        args!["Spore Hat?", "850 Poison Spore", "300 Burnt Tree", "1 Tongue"],
                    )?;
                }
                2 => {
                    ctx.lines_as("Titicupe", args!["Wonder Nutshell?", "1 Nut Shell", "500 Wing of Dragonfly"])?;
                }
                3 => {
                    ctx.lines_as(
                        "Titicupe",
                        args![
                            "Rainbow Eggshell?",
                            "1 Egg Shell",
                            "1 Cobaltblue Dyestuffs",
                            "50 Claw of Desert Wolf"
                        ],
                    )?;
                }
                4 => {
                    ctx.lines_as(
                        "Titicupe",
                        args!["Blush?!", "You!", "You know something...", "Heh heh~ Anyway", "100 Alice's Apron"],
                    )?;
                }
                5 => {
                    ctx.lines_as(
                        "Titicupe",
                        args![
                            "Chef Hat?",
                            "120 Piece of Cake",
                            "1 White Dyestuffs",
                            "330 Feather",
                            "450 Dragon Scale"
                        ],
                    )?;
                }
                6 => {
                    ctx.lines_as("Titicupe", args!["Candle?", "1 Bomb Wick", "50 Matchstick", "100 Royal Jelly"])?;
                }
                7 => {
                    ctx.lines_as(
                        "Titicupe",
                        args![
                            "Cake Hat?",
                            "10 Candy",
                            "5 Candy Cane",
                            "15 Well-baked Cookie",
                            "20 Piece Of Cake",
                            "10 Steel"
                        ],
                    )?;
                }
                8 => {
                    ctx.lines_as(
                        "Titicupe",
                        args!["Ask me anytime. Those kinds of questions are no problem for geniuses like me!"],
                    )?;
                }
                _ => {}
            }
            ctx.next()?;
            ctx.lines_as("Titicupe", args!["So happy!!", "I'm a genius~!", "Ho ho ho", "Ho ho ho ho!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Titicupe",
                args!["Sometimes, I can't believe how magnificent this invention of mine is...!"],
            )?;
            ctx.next()?;
            ctx.lines_as("Titicupe", args!["I...", "I love you", "Mister Snowman", "Vending Machine."])?;
            ctx.npc().emotion(constants::ET_CHUPCHUP)?;
            ctx.next()?;
            ctx.mes("^3355FFToy factory manager Titicupe begins to jump around and emit screams of unbridled ecstacy. At this point, it's not difficult to doubt his sanity, as well as his genius.")?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn vending_machine_man(ctx: &Ctx) -> Script {
    vending_machine_man_body(ctx, Vec::new()).map(|_| ())
}

fn farewell(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Audi",
        args![
            "^555555Thank you for coming by.",
            "Please come again!",
            "Oh, and tell a friend!^000000"
        ],
    )?;
    ctx.close()
}

pub fn vending_machine(ctx: &Ctx) -> Script {
    ctx.lines_as("Audi", args!["^555555Exciting item vending machine. Invented by great Titicupe. Select the desired item and place required items into snowmouth receptacle.^000000"])?;
    ctx.next()?;
    match ctx.menu(&[
        "^FF3355Notice (Must Read)^000000",
        "Raccoon Hat",
        "Spore Hat",
        "Wonder Nutshell",
        "Rainbow Eggshell",
        "Blush",
        "Chef Hat",
        "Candle",
        "Cake Hat",
        "Cancel",
    ])? {
        0 => {
            ctx.lines_as("Audi", args!["Hi~ !", "I need to tell you one important thing. This vending machine is not equipped to differentiate between special items and ordinary items."])?;
            ctx.next()?;
            ctx.lines_as("Audi", args!["So if you use an item that has been ^FF3355upgraded, or has a card compounded to it^000000, any cards or upgrades will be lost in creating the new item."])?;
            ctx.next()?;
            ctx.lines_as("Audi", args!["So when you try to exchange and generate items, please put all the valuable items you won't be using into the Kafra Storage."])?;
            ctx.next()?;
            ctx.lines_as(
                "Audi",
                args!["Please keep this in mind because this Vending Machine does not provide any returns, refunds or exchanges."],
            )?;
            ctx.next()?;
            ctx.lines_as("Audi", args!["Have a great time!", "Thank you!"])?;
            return ctx.close();
        }
        1 => {
            if ctx.items().count(2213)? > 0
                && ctx.items().count(1036)? > 19
                && ctx.items().count(7012)? > 199
                && ctx.items().count(7065)? > 299
            {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(2213, 1)?;
                ctx.items().take(1036, 20)?;
                ctx.items().take(7012, 200)?;
                ctx.items().take(7065, 300)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args!["^3355FF*Vroooooom~~*", "*Bzzzzzt*", "*choogachooga*", "*Kapang!*^000000"])?;
                ctx.items().give(5033, 1)?;
                ctx.next()?;
                return farewell(ctx);
            }
        }
        2 => {
            if ctx.items().count(7033)? > 849 && ctx.items().count(7068)? > 299 && ctx.items().count(1015)? > 0 {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(7033, 850)?;
                ctx.items().take(7068, 300)?;
                ctx.items().take(1015, 1)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args![
                    "^3355FF*Vroooooom~~*",
                    "*Bzzzzzt*",
                    "*choogachooga*",
                    "*Kapang!*",
                    "*Wek Wek*^000000"
                ])?;
                ctx.items().give(5029, 1)?;
                ctx.next()?;
                ctx.lines(args![
                    "^555555Thank you for coming by.",
                    "Please come again!",
                    "Oh, and tell a friend!^000000"
                ])?;
                return ctx.close();
            }
        }
        3 => {
            if ctx.items().count(5037)? > 0 && ctx.items().count(7064)? > 499 {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(5037, 1)?;
                ctx.items().take(7064, 500)?;
                ctx.lines(args![
                    "^3355FF*Vroooooom~~*",
                    "*Bzzzzzt*",
                    "*choogachooga*",
                    "*Kapang!*",
                    "*BeepBoopBeepBoop*",
                    "*Beeeeeeoop*^000000"
                ])?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.items().give(5050, 1)?;
                ctx.next()?;
                return farewell(ctx);
            }
        }
        4 => {
            if ctx.items().count(5015)? > 0 && ctx.items().count(978)? > 0 && ctx.items().count(7030)? > 49 {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(5015, 1)?;
                ctx.items().take(978, 1)?;
                ctx.items().take(7030, 50)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args![
                    "^3355FF*Vroooooom~~*",
                    "*Bzzzzzt*",
                    "*choogachooga*",
                    "*OoooEeeeeeee~*^000000"
                ])?;
                ctx.items().give(5039, 1)?;
                ctx.next()?;
                return farewell(ctx);
            }
        }
        5 => {
            if ctx.items().count(7047)? > 99 {
                ctx.mes("^3355FFYou gingerly place all 100 Aprons into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(7047, 100)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args![
                    "^3355FF*Vroooooom~~*",
                    "*Bzzzzzt*",
                    "*choogachooga*",
                    "*Kapang!*",
                    "*ChoopChoop*",
                    "*Chaaawah!*^000000"
                ])?;
                ctx.items().give(5040, 1)?;
                ctx.lines_as(
                    "Audi",
                    args![
                        "^555555Thank you for coming by.",
                        "Please come again!",
                        "Oh, and tell a friend!^000000"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Titicupe",
                    args![
                        "You got Blush?! Huh. I guess you really like looking glamourous~! I bet you really appreciate my genius now, huh?"
                    ],
                )?;
                return ctx.close();
            }
        }
        6 => {
            if ctx.items().count(539)? > 119
                && ctx.items().count(982)? > 0
                && ctx.items().count(949)? > 329
                && ctx.items().count(1036)? > 449
            {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(539, 120)?;
                ctx.items().take(982, 1)?;
                ctx.items().take(949, 330)?;
                ctx.items().take(1036, 450)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args![
                    "^3355FF*Vroooooom~~*",
                    "*Bzzzzzt*",
                    "*choogachooga*",
                    "*Kapang!*",
                    "*Muuuuuuugeeee*^000000"
                ])?;
                ctx.items().give(5026, 1)?;
                ctx.next()?;
                return farewell(ctx);
            }
        }
        7 => {
            if ctx.items().count(2279)? > 0 && ctx.items().count(7035)? > 49 && ctx.items().count(526)? > 99 {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(2279, 1)?;
                ctx.items().take(7035, 50)?;
                ctx.items().take(526, 100)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args!["^3355FF*Vroooooom~~*", "*Bzzzzzt*", "*choogachooga*", "*Kapang!*^000000"])?;
                ctx.items().give(5028, 1)?;
                ctx.next()?;
                return farewell(ctx);
            }
        }
        8 => {
            if ctx.items().count(529)? > 9
                && ctx.items().count(530)? > 4
                && ctx.items().count(538)? > 14
                && ctx.items().count(539)? > 19
                && ctx.items().count(999)? > 9
            {
                ctx.mes("^3355FFYou gingerly place the items into the snowman's mouth.^000000")?;
                ctx.next()?;
                ctx.items().take(529, 10)?;
                ctx.items().take(530, 5)?;
                ctx.items().take(538, 15)?;
                ctx.items().take(539, 20)?;
                ctx.items().take(999, 10)?;
                ctx.npc().emotion(constants::ET_O)?;
                ctx.lines(args!["^3355FF*Vroooooom~~*", "*Bzzzzzt*", "*choogachooga*", "*Kapang!*^000000"])?;
                ctx.items().give(5024, 1)?;
                ctx.next()?;
                return farewell(ctx);
            }
        }
        9 => {
            ctx.lines_as("Audi", args!["^555555Please...", "Insert...", "Items.^000000"])?;
            return ctx.close();
        }
        _ => {}
    }
    ctx.npc().emotion(constants::ET_X)?;
    ctx.lines_as("Audi", args!["^555555Error Error!", "Incorrect items!^000000"])?;
    ctx.next()?;
    ctx.lines_as(
        "Titicupe",
        args![
            "I...",
            "I guess a lot of people would do anything to get their hands on some Blush."
        ],
    )?;
    ctx.close()
}
