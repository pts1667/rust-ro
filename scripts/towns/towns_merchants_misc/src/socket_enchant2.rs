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

pub fn leablem_dummy(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Leablem",
        args![
            "Good day. My name is Leablem,",
            "and I'm a technician that specializes in",
            "adding slots to weapons and armors.",
            "It can be very hard sometimes, but I like",
            "what I do, and take pride in it."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Leablem",
        args![
            "I've learned most of the things from my brother, Seyablem,",
            "but I think I still have a lot to learn.",
            "So please understand my slotting services are limited,",
            "unlike my brother Seyablem."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Leablem",
        args![
            "My brother wasn't so happy to teach me",
            "his skills because he was worried that his skills",
            "wouldn't be unique anymore. He did, however, his best",
            "to teach me."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Leablem",
        args![
            "Thanks to him, I've learned lots of things.",
            "I'm not very confident",
            "in trying many different things,",
            "but I'll do what",
            "I can do for now."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Leablem",
        args![
            "In fact, there's one thing",
            "that my brother doesn't want me to do.",
            "I don't want to upset him, so...let's just skip that part.",
            "I'll say no more about it!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Leablem",
        args![
            "Anyways, do you have any equipment",
            "which you'd like to add to the slots?",
            "My service charge, the materials,",
            "and the success chance all depend on",
            "the specific item I'm working on."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Weapon", "Armor", "About that thing you skipped", "More information", "Quit"])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Leablem",
                args![
                    "Weapon, you said? Sure, no problem.",
                    "Weapons are divided into 4 different class",
                    "depending on the work difficulty.",
                    "C class is the easiest one, and S class is the hardest one.",
                    "Which class would you like to try?"
                ],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["C", "B", "A", "S"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Leablem", args!["C class, I see.", "So, what kind of weapon do you have?"])?;
                    ctx.next()?;
                    'b3: {
                        let subject3 = ctx.menu(&[
                            "Book of Mother Earth",
                            "Book of Billows",
                            "Book of Gust of Wind",
                            "Book of the Blazing Sun",
                        ])?;
                        let mut matched3 = false;
                        if !matched3 && subject3 == 0 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1554, 1569, 40, 66, 200, 1010, 10])?;
                        }
                        if !matched3 && subject3 == 1 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1553, 1568, 40, 66, 200, 1010, 10])?;
                        }
                        if !matched3 && subject3 == 2 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1556, 1571, 40, 66, 200, 1010, 10])?;
                        }
                        if !matched3 && subject3 == 3 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1555, 1570, 40, 66, 200, 1010, 10])?;
                        }
                    }
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Leablem",
                        args!["You have chosen average B class.", "So, what kind of weapon do you have?"],
                    )?;
                    ctx.next()?;
                    'b4: {
                        let subject4 = ctx.menu(&["Orcish Axe", "Scimiter", "Spike"])?;
                        let mut matched4 = false;
                        if !matched4 && subject4 == 0 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1304, 1309, 40, 66, 200, 984, 1])?;
                        }
                        if !matched4 && subject4 == 1 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1113, 1114, 40, 66, 200, 984, 1])?;
                        }
                        if !matched4 && subject4 == 2 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1523, 1538, 40, 61, 300, 984, 1])?;
                        }
                    }
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "A class? You seem to want little too much. But, no problem.",
                            "So, what kind of weapon do you have?"
                        ],
                    )?;
                    ctx.next()?;
                    'b5: {
                        let subject5 = ctx.menu(&[
                            "Dragon Killer",
                            "Katar of Quaking",
                            "Katar of Raging Blaze",
                            "Katar of Frozen Icicle",
                            "Katar of Piercing Wind",
                            "Golden Mace",
                            "Oriental Lute",
                            "Queen's Whip",
                            "Spectral Spear",
                            "Gae Bolg",
                            "Schweizersabel",
                        ])?;
                        let mut matched5 = false;
                        if !matched5 && subject5 == 0 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![13001, 13030, 40, 61, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 1 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1257, 1276, 40, 66, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 2 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1258, 1277, 40, 66, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 3 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1256, 1275, 40, 66, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 4 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1259, 1278, 40, 66, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 5 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1524, 1539, 40, 61, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 6 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1918, 1922, 40, 61, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 7 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1970, 1976, 40, 61, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 8 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1477, 1479, 40, 56, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 9 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1474, 1480, 40, 56, 500, 984, 2])?;
                        }
                        if !matched5 && subject5 == 10 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1167, 1178, 40, 61, 500, 984, 2])?;
                        }
                    }
                }
                if !matched2 && subject2 == 3 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "Huh, S Class?! Oh my god, are you serious?*Tremble*",
                            "So...which S class weapon would you like to make a slot?!"
                        ],
                    )?;
                    ctx.next()?;
                    'b6: {
                        let subject6 = ctx.menu(&[
                            "Zephyrus",
                            "Mailbreaker",
                            "Dragon Slayer",
                            "Swordbreaker",
                            "Assasin Dagger",
                            "Grand Cross",
                            "Executioner",
                        ])?;
                        let mut matched6 = false;
                        if !matched6 && subject6 == 0 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1468, 1481, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 1 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1225, 13032, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 2 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1166, 1180, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 3 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1224, 13031, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 4 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1232, 13033, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 5 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1528, 1540, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 6 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![1169, 1179, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                    }
                }
            }
        }
        if !matched1 && subject1 == 1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Leablem",
                args![
                    "Armor, you said? Sure, no problem.",
                    "Armors are divided into 4 different class",
                    "depending on the work difficulty.",
                    "C class is the easiest one, and S class is the hardest one.",
                    "Which class would you like to try?"
                ],
            )?;
            ctx.next()?;
            'b7: {
                let subject7 = ctx.menu(&["C", "B", "A", "S"])?;
                let mut matched7 = false;
                if !matched7 && subject7 == 0 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as("Leablem", args!["C class, I see.", "So, what kind of armor do you have?"])?;
                    ctx.next()?;
                    'b8: {
                        let subject8 = ctx.menu(&["Sunflower", "Ph.D Hat", "Big Ribbon", "Boys Cap"])?;
                        let mut matched8 = false;
                        if !matched8 && subject8 == 0 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2253, 5351, 40, 66, 100, 999, 3])?;
                        }
                        if !matched8 && subject8 == 1 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![5012, 5347, 40, 66, 100, 999, 3])?;
                        }
                        if !matched8 && subject8 == 2 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2244, 5348, 40, 66, 100, 999, 5])?;
                        }
                        if !matched8 && subject8 == 3 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![5016, 5349, 40, 66, 100, 999, 5])?;
                        }
                    }
                }
                if !matched7 && subject7 == 1 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Leablem",
                        args!["You have chosen average B class.", "So, what kind of armor do you have?"],
                    )?;
                    ctx.next()?;
                    'b9: {
                        let subject9 = ctx.menu(&["Skull Ring", "High Heels"])?;
                        let mut matched9 = false;
                        if !matched9 && subject9 == 0 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2609, 2715, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 1 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2409, 2432, 40, 61, 300, 999, 5])?;
                        }
                    }
                }
                if !matched7 && subject7 == 2 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "A class? You seem to want little too much. But, no problem.",
                            "So, what kind of armor do you have?"
                        ],
                    )?;
                    ctx.next()?;
                    'b10: {
                        let subject10 = ctx.menu(&["Pirate Bandana", "Black Leather Boots"])?;
                        let mut matched10 = false;
                        if !matched10 && subject10 == 0 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2287, 5350, 50, 61, 500, 985, 1])?;
                        }
                        if !matched10 && subject10 == 1 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2425, 2434, 40, 51, 500, 985, 1])?;
                        }
                    }
                }
                if !matched7 && subject7 == 3 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "Huh, S Class?! Oh my god, are you serious?*Tremble*",
                            "So...which S class armor would you like to make a slot?!"
                        ],
                    )?;
                    ctx.next()?;
                    'b11: {
                        let subject11 = ctx.menu(&["Mage Coat", "Holy Robe", "Sacred Mission", "Undershirt", "Pantie"])?;
                        let mut matched11 = false;
                        if !matched11 && subject11 == 0 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2334, 2372, 40, 51, 1000, 985, 1])?;
                        }
                        if !matched11 && subject11 == 1 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2327, 2373, 45, 51, 1000, 985, 1])?;
                        }
                        if !matched11 && subject11 == 2 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2111, 2128, 45, 51, 1000, 985, 1])?;
                        }
                        if !matched11 && subject11 == 3 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2522, 2523, 45, 51, 1000, 985, 1])?;
                        }
                        if !matched11 && subject11 == 4 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant2::func_socket2(ctx, args![2339, 2371, 45, 51, 1000, 985, 1])?;
                        }
                    }
                }
            }
        }
        if !matched1 && subject1 == 2 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Leablem",
                args![
                    "Oh, it's nothing special, but he thinks",
                    "it's not something for humans to mess with. ",
                    "I don't want to upset my brother,",
                    "so I'd better forget about it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "...That service itself has a very high success chance,",
                    "but requires a lot of materials and a high-rate service charge.",
                    "I highly doubt that anyone would actually want to use",
                    "the service, even if I did open it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Leablem", args!["...Ummm..."])?;
            ctx.next()?;
            ctx.lines_as("Leablem", args!["............."])?;
            ctx.next()?;
            ctx.lines_as("Leablem", args!["............."])?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "You seem interested in using that service.",
                    "If you want, I can at least tell you about it.",
                    "But you must promise to keep this a secret.",
                    "If my brother finds out what I'm about to tell you,",
                    "he's gonna give me a beating!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "This special service has a 90% success chance,",
                    "but requires 2 Gold and 2 hundred million zeny:",
                    "I can add slots to the Hat of the Sun God."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "My brother said that, even since ancient times, it's forbidden to modify that hat.",
                    "But I believe humans are too curious not to",
                    "experiment, especially when it's forbidden.",
                    "I just don't want my brother to yell at me. That's all."
                ],
            )?;
            ctx.next()?;
            'b12: {
                let subject12 = ctx.menu(&["Use the Service", "Quit"])?;
                let mut matched12 = false;
                if !matched12 && subject12 == 0 {
                    matched12 = true;
                }
                if matched12 {
                    ctx.lines_as("Leablem", args!["................."])?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["................."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "...Wh... What? No!",
                            "Giving me that dirty look",
                            "won't make me do it!",
                            "No, I said no! No is no! No!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["No, I won't! No!"])?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["I can't do it..."])?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["I can't do... It..."])?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["................."])?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["Umm..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "You're so persistent. Okay, I'll make you an exception.",
                            "Just don't tell anyone,",
                            "especially my brother. Do you promise?"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "Okay then, please bring me",
                            "^FF00002 Gold, 2 hundred million zeny -- the service charge --,",
                            "^FF0000and a Hat of the Sun God.",
                            "^FF0000You know your chance of success is 90% with this, don't you?^000000",
                            "We don't have time to waste,",
                            "so let's get down to business right away."
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.player().zeny()? > 199999999 && ctx.items().count(969)? > 1 && ctx.items().count(5022)? > 0 {
                        ctx.lines_as("Leablem", args!["Shall we start now?", "Pray to your gods for good luck."])?;
                        ctx.next()?;
                        let roll = ctx.rand_range(1, 100)?;
                        if roll > 4 && roll < 95 {
                            ctx.npc().special_effect(constants::EF_LORD)?;
                            ctx.lines_as(
                                "Leablem",
                                args![
                                    "Excellent! Wow, I guess the 90% success chance is true!",
                                    "Look, the slot was added in the perfect place.",
                                    "Congratulations."
                                ],
                            )?;
                            ctx.items().take(5022, 1)?;
                            ctx.items().take(969, 2)?;
                            ctx.player().set_zeny(ctx.player().zeny()? - 200000000)?;
                            ctx.items().give(5353, 1)?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Leablem",
                                args![
                                    "This is the first and last time I will do this for you.",
                                    "Don't ever tell anyone",
                                    "about this. Okay?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            ctx.npc().special_effect(constants::EF_SUI_EXPLOSION)?;
                            ctx.lines_as(
                                "Leablem",
                                args![
                                    "Wah! Oh my god, we failed!",
                                    "My god, I guess we had the worst kind of luck...",
                                    "What should we do? ...This is why",
                                    "I didn't want to do in the first place!"
                                ],
                            )?;
                            ctx.items().take(5022, 1)?;
                            ctx.items().take(969, 2)?;
                            ctx.player().set_zeny(ctx.player().zeny()? - 200000000)?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Leablem",
                                args!["No, don't ever ask me to do such a risky thing again!", "Bah~"],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        ctx.lines_as("Leablem", args!["Umm..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Leablem",
                            args![
                                "You don't have enough materials.",
                                "Man, why did you even ask me to try?",
                                "I'm not going to do this again for you,",
                                "even if you bring enough materials. Bah~"
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched12 && subject12 == 1 {
                    matched12 = true;
                }
                if matched12 {
                    ctx.lines_as("Leablem", args!["Don't ever tell anyone", "about what I told you. Okay?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Leablem",
                args![
                    "Do you want to know more information?",
                    "Well, I don't think there's something",
                    "that you specifically need to know.",
                    "But I can at least give you a tip."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "The slot technicians in each town",
                    "provide different slotting success chances",
                    "depending on your equipment's upgrade level.",
                    "I don't know if that's true,",
                    "but I can see the difference in each town."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "For instance, a slot technician in one town",
                    "provides a higher success chance to",
                    "equipment items at higher upgrade levels,",
                    "while a technician in another town has higher",
                    "success at lower upgrade levels."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "When you fail to add a slot, the equipment",
                    "will become broken and disappear.",
                    "I try my best to succeed, but",
                    "slotting is complicated work."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Leablem",
                args![
                    "Oh, there's an important thing that you need to be careful about.",
                    "^FF0000Try not to carry two identical equipment items",
                    "^FF0000with you when you add slots.",
                    "^FF0000Otherwise, slots may be added to one of the randomly chosen items,",
                    "^FF0000and in the worst case, you may lose an item that is more valuable to you.^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Leablem", args!["I hope this information will be helpful to you.", "Thanks."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if !matched1 && subject1 == 4 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Leablem", args!["Alright, then."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    }
    Ok(())
}
