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

pub fn seiyablem_dummy(ctx: &Ctx) -> Script {
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
        "Seiyablem",
        args![
            "I'm an engineer that specializes in adding Slots to normal Weapons and Armor.",
            "My service fee, the required materials and the chance of success all depend on the specific item I'm working on."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Seiyablem",
        args![
            "Adding Slots may seem simple, but it's far more complicated than it looks.",
            "If you're interested in my service, let me know."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = ctx.menu(&["Add Slot to Weapon", "Add Slot to Armor", "Slot Attempt Info", "Cancel"])?;
        let mut matched1 = false;
        if !matched1 && subject1 == 0 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Seiyablem",
                args![
                    "You want to add a Slot to a weapon?",
                    "I categorize weapons by their difficulty in adding a Slot, C Class being easiest and S Class the most difficult.",
                    "Which class would you like to try?"
                ],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = ctx.menu(&["C Class", "B Class", "A Class", "S Class"])?;
                let mut matched2 = false;
                if !matched2 && subject2 == 0 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as("Seiyablem", args!["C Class, eh?", "Now, I can only upgrade certain equipment in that class, so please choose one of the following items, so we can try to add a Slot to it."])?;
                    ctx.next()?;
                    'b3: {
                        let subject3 = ctx.menu(&["Trident", "Rope", "Violin"])?;
                        let mut matched3 = false;
                        if !matched3 && subject3 == 0 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1460, 1461, 40, 66, 200, 1010, 10])?;
                        }
                        if !matched3 && subject3 == 1 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1950, 1951, 40, 66, 200, 1010, 10])?;
                        }
                        if !matched3 && subject3 == 2 {
                            matched3 = true;
                        }
                        if matched3 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1901, 1902, 40, 66, 200, 1010, 10])?;
                        }
                    }
                }
                if !matched2 && subject2 == 1 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "B Class?",
                            "Alright, this is the average equipment category.",
                            "Please choose the weapon to which you'd like me to try to add a Slot."
                        ],
                    )?;
                    ctx.next()?;
                    'b4: {
                        let subject4 = ctx.menu(&[
                            "Chain",
                            "Gladius",
                            "Gakkung Bow",
                            "Pike",
                            "Haedonggum",
                            "Lute",
                            "Wire",
                            "Waghnakh",
                            "Arbalest Bow",
                        ])?;
                        let mut matched4 = false;
                        if !matched4 && subject4 == 0 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1519, 1520, 40, 61, 300, 1010, 10])?;
                        }
                        if !matched4 && subject4 == 1 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1219, 1220, 40, 61, 300, 984, 1, 999, 5])?;
                        }
                        if !matched4 && subject4 == 2 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1714, 1716, 40, 61, 300, 984, 2, 999, 5])?;
                        }
                        if !matched4 && subject4 == 3 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1407, 1408, 40, 61, 200, 1010, 10])?;
                        }
                        if !matched4 && subject4 == 4 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1123, 1128, 40, 61, 300, 984, 2, 999, 5])?;
                        }
                        if !matched4 && subject4 == 5 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1905, 1906, 40, 61, 300, 1011, 10])?;
                        }
                        if !matched4 && subject4 == 6 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1954, 1955, 40, 61, 300, 1011, 10])?;
                        }
                        if !matched4 && subject4 == 7 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1801, 1802, 40, 61, 300, 1010, 10])?;
                        }
                        if !matched4 && subject4 == 8 {
                            matched4 = true;
                        }
                        if matched4 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1713, 1715, 40, 61, 300, 984, 2, 999, 5])?;
                        }
                    }
                }
                if !matched2 && subject2 == 2 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "Ooh, A Class.",
                            "This is some high risk territory!",
                            "Alright, which weapon would you like me to try adding a Slot?"
                        ],
                    )?;
                    ctx.next()?;
                    'b5: {
                        let subject5 = ctx.menu(&[
                            "Hunter Bow",
                            "Survivor's Rod(INT)",
                            "Zweihander",
                            "Flamberge",
                            "Infiltrator",
                            "Ballista",
                            "Stunner",
                            "Berserk",
                            "Claymore",
                        ])?;
                        let mut matched5 = false;
                        if !matched5 && subject5 == 0 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1718, 1726, 40, 61, 500, 984, 2, 999, 10])?;
                        }
                        if !matched5 && subject5 == 1 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1619, 1620, 40, 61, 500, 984, 5, 999, 10])?;
                        }
                        if !matched5 && subject5 == 2 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1168, 1171, 40, 61, 800, 984, 5, 999, 10])?;
                        }
                        if !matched5 && subject5 == 3 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1129, 1149, 40, 61, 500, 984, 2, 999, 10])?;
                        }
                        if !matched5 && subject5 == 4 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1261, 1266, 40, 61, 700, 984, 5, 999, 10])?;
                        }
                        if !matched5 && subject5 == 5 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1722, 1727, 40, 61, 500, 984, 5, 999, 10])?;
                        }
                        if !matched5 && subject5 == 6 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1522, 1532, 40, 61, 500, 984, 2, 999, 10])?;
                        }
                        if !matched5 && subject5 == 7 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1814, 1816, 40, 61, 500, 984, 5, 999, 10])?;
                        }
                        if !matched5 && subject5 == 8 {
                            matched5 = true;
                        }
                        if matched5 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1163, 1172, 40, 61, 500, 984, 2, 999, 10])?;
                        }
                    }
                }
                if !matched2 && subject2 == 3 {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "Whoa, whoa, whoa...",
                            "S class? Alright...",
                            "It'd be a near miracle if I can actually pull this off.",
                            "Okay... Which weapon shall we try adding a Slot to?"
                        ],
                    )?;
                    ctx.next()?;
                    'b6: {
                        let subject6 = ctx.menu(&[
                            "Gungnir",
                            "Poison Knife",
                            "Ice Pick",
                            "Sucsamad",
                            "Ginnungagap",
                            "Cutlas",
                            "Crescent Scythe",
                            "Survivor's Rod(DEX)",
                        ])?;
                        let mut matched6 = false;
                        if !matched6 && subject6 == 0 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1413, 1418, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 1 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1239, 13016, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 2 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1230, 13017, 40, 51, 2000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 3 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1236, 13018, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 4 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![13002, 13019, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 5 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1135, 13400, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 6 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1466, 1476, 40, 51, 1000, 984, 5, 999, 10])?;
                        }
                        if !matched6 && subject6 == 7 {
                            matched6 = true;
                        }
                        if matched6 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![1617, 1618, 40, 51, 2000, 984, 5, 999, 10])?;
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
                "Seiyablem",
                args![
                    "Armor, you said?",
                    "Sure, no problem.",
                    "Armor is divided into 4 different classes, depending on the work difficulty.",
                    "C Class is the easiest one, and S Class he hardest one.",
                    "Which class would you like to try?"
                ],
            )?;
            ctx.next()?;
            'b7: {
                let subject7 = ctx.menu(&["C Class", "B Class", "A Class", "S Class"])?;
                let mut matched7 = false;
                if !matched7 && subject7 == 0 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "C Class Armor, eh?",
                            "Sounds reasonable.",
                            "Which armor would you like to try adding a Slot?"
                        ],
                    )?;
                    ctx.next()?;
                    'b8: {
                        let subject8 = ctx.menu(&["Mantle", "Coat", "Circlet", "Biretta"])?;
                        let mut matched8 = false;
                        if !matched8 && subject8 == 0 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2307, 2308, 40, 66, 200, 999, 3])?;
                        }
                        if !matched8 && subject8 == 1 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2309, 2310, 40, 66, 200, 999, 3])?;
                        }
                        if !matched8 && subject8 == 2 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2232, 2233, 40, 66, 200, 999, 3])?;
                        }
                        if !matched8 && subject8 == 3 {
                            matched8 = true;
                        }
                        if matched8 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2216, 2217, 40, 66, 200, 999, 3])?;
                        }
                    }
                }
                if !matched7 && subject7 == 1 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Seiyablem",
                        args!["You have chosen average B Class.", "So, what kind of armor do you have?"],
                    )?;
                    ctx.next()?;
                    'b9: {
                        let subject9 = ctx.menu(&[
                            "Mirror Shield",
                            "Chain Mail",
                            "Saint Robe",
                            "Silk Robe",
                            "Boots",
                            "Shoes",
                            "Muffler",
                            "Guard",
                            "Buckler",
                            "Shield",
                            "Bongun Hat",
                        ])?;
                        let mut matched9 = false;
                        if !matched9 && subject9 == 0 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2107, 2108, 40, 61, 250, 999, 5])?;
                        }
                        if !matched9 && subject9 == 1 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2314, 2315, 40, 61, 250, 999, 5])?;
                        }
                        if !matched9 && subject9 == 2 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2325, 2326, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 3 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2321, 2322, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 4 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2405, 2406, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 5 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2403, 2404, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 6 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2503, 2504, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 7 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2101, 2102, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 8 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2103, 2104, 40, 61, 300, 999, 5])?;
                        }
                        if !matched9 && subject9 == 9 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2105, 2106, 40, 61, 250, 999, 5])?;
                        }
                        if !matched9 && subject9 == 10 {
                            matched9 = true;
                        }
                        if matched9 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![5046, 5168, 40, 61, 250, 999, 5])?;
                        }
                    }
                }
                if !matched7 && subject7 == 2 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "Ooh, A Class.",
                            "This is some high risk territory!",
                            "Alright, which armor would you like me to try adding a Slot?"
                        ],
                    )?;
                    ctx.next()?;
                    'b10: {
                        let subject10 = ctx.menu(&[
                            "Gemmed Sallet",
                            "Bucket Hat",
                            "Memory Book",
                            "Tights",
                            "Legion Plate Armor",
                            "Full Plate",
                            "Thief Clothes",
                            "Greaves",
                            "Coif",
                            "Manteau",
                            "Helm",
                            "Ninja Suit",
                            "Orc Helm",
                            "Ancient Cape",
                            "Monk Hat",
                            "Golden Gear",
                            "Brooch",
                            "Munak Hat",
                        ])?;
                        let mut matched10 = false;
                        if !matched10 && subject10 == 0 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2230, 2231, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 1 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![5114, 5120, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 2 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2109, 2121, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 3 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2330, 2331, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 4 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2341, 2342, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 5 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2316, 2317, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 6 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2335, 2336, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 7 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2411, 2412, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 8 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![5092, 5093, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 9 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2505, 2506, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 10 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2228, 2229, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 11 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2337, 2359, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 12 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2299, 5157, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 13 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2507, 2525, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 14 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2251, 5158, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 15 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2246, 5159, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 16 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2605, 2625, 40, 61, 400, 985, 1])?;
                        }
                        if !matched10 && subject10 == 17 {
                            matched10 = true;
                        }
                        if matched10 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2264, 5167, 40, 61, 300, 985, 1])?;
                        }
                    }
                }
                if !matched7 && subject7 == 3 {
                    matched7 = true;
                }
                if matched7 {
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "Um... S Class?",
                            "You sure about this?",
                            "Alright... Let me know the Armor to which you'd like to add a Slot, and I'll tell what I need to try it."
                        ],
                    )?;
                    ctx.next()?;
                    'b11: {
                        let subject11 = ctx.menu(&[
                            "Majestic Goat",
                            "Spiky Band",
                            "Bone Helm",
                            "Corsair",
                            "Crown",
                            "Tiara",
                            "Sphinx Hat",
                            "Robe of Cast",
                            "Earring",
                            "Ring",
                            "Bow Thimble",
                        ])?;
                        let mut matched11 = false;
                        if !matched11 && subject11 == 0 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2256, 5160, 40, 51, 2000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 1 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2258, 5161, 40, 51, 2000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 2 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![5017, 5162, 40, 51, 2000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 3 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![5019, 5163, 40, 51, 2000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 4 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2235, 5165, 40, 51, 2000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 5 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2234, 5164, 40, 51, 2000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 6 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![5053, 5166, 40, 51, 1000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 7 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2343, 2360, 40, 51, 1000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 8 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2602, 2622, 40, 51, 1000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 9 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2601, 2621, 40, 51, 1000, 985, 2])?;
                        }
                        if !matched11 && subject11 == 10 {
                            matched11 = true;
                        }
                        if matched11 {
                            shared::merchants_socket_enchant::func_socket(ctx, args![2619, 2671, 40, 51, 1000, 985, 2])?;
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
                "Seiyablem",
                args![
                    "Well, I haven't really refined the art of Slot Addition.",
                    "It's so complicated that I'd be lying if I told you that I knew every factor that affected the process.",
                    "Still, I do notice a few trends..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Seiyablem",
                args![
                    "In some towns, Slot addition attempts are more successful for equipment with fewer upgrades.",
                    "In other towns, the opposite is true.",
                    "isn't that really peculiar?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Seiyablem",
                args![
                    "Oh, there's also something really important that you should know.",
                    "If you ask me to add a Slot to something, make sure that you don't have multiples of it in your inventory."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Seiyablem", args!["Here's an example: if you have a +7 Manteau and a normal Manteau in your inventory, then I will randomly pick one of them for my Slot Addition attempt.", "Just remember to be careful."])?;
            ctx.next()?;
            ctx.lines_as(
                "Seiyablem",
                args![
                    "Again, ^FF0000only carry one of the equipment to which you want me to add a Slot^000000.",
                    "All other equipment with the same name should be placed in your Kafra Storage, got it?"
                ],
            )?;
            return ctx.close();
        }
        if !matched1 && subject1 == 3 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Seiyablem",
                args![
                    "Take it easy, adventurer.",
                    "If you ever want to try adding a Slot to some of your equipment, come back and let me know.",
                    "Seeya~"
                ],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn young_man_dummy(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Young Man",
        args![
            "I'm considering becoming a Slotting Engineer.",
            "It's a new field where you can add Slots to equipment, and it'd be cool if I can work in such a lucrative career."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args![
            "There's a huge element of luck involved in successfully adding Slots from what I can understand.",
            "You fail sometimes, but if you're super lucky, you can add two Slots.",
            "Crazy. Huh?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Young Man",
        args![
            "But don't get too excited.",
            "Only certain equipment can handle the addition of two extra Slots.",
            "There's so much more I have to learn before I can even be an apprentice..."
        ],
    )?;
    ctx.close()
}
