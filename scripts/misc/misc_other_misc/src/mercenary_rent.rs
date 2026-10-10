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

pub fn mercenary_manager_main(ctx: &Ctx) -> Script {
    let l_name_s = vec![Val::from("Spear"), Val::from("Sword"), Val::from("Bow")];
    let l_faith_s = vec![
        Val::from("SPEAR_MERC_GUILD"),
        Val::from("SWORD_MERC_GUILD"),
        Val::from("ARCH_MERC_GUILD"),
    ];
    let l_item = vec![Val::from(12182), Val::from(12172), Val::from(12162)];
    let npc_s = ctx.call(Function::StrNpcInfo, args![2])?;
    let mut kind = 0;
    while kind < l_name_s.len() as i32 {
        if runtime::compare(&npc_s, &runtime::local_get(&l_name_s, &Val::from(kind), true)).is_true() {
            break;
        }
        kind += 1;
    }
    let name = runtime::local_get(&l_name_s, &Val::from(kind), true);
    let faith_key = runtime::local_get(&l_faith_s, &Val::from(kind), true);
    let item_id = runtime::local_get(&l_item, &Val::from(kind), false);
    let faith_merc = ctx.call(Function::MercenaryGetFaith, args![runtime::getd(ctx, &faith_key, &[])?])?;
    ctx.lines_as(
        "Mercenary Manager",
        args!["Welcome to the", name.clone() + " Mercenary Guild.", "What can I do for you?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Hire Mercenary", "Mercenary Info", "Nothing", "10th Grade Mercenaries"])? {
        0 => {
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "You want to hire a",
                    name.clone() + " Mercenary?",
                    "Which Grade were you",
                    "interested in hiring?"
                ],
            )?;
            ctx.next()?;
            let mut grade_menu = Val::from("");
            for i in 1..=9 {
                grade_menu =
                    grade_menu + shared::other_global_functions::f_getnumsuffix(ctx, args![i])? + " Grade " + name.clone() + " Mercenary:";
            }
            let grade = runtime::select_values(ctx, &[grade_menu])?;
            let base_level = 5 + grade * 10;
            let zeny_cost = 7 * grade;
            let faith_cost: i32 = match grade {
                7 => 50,
                8 => 100,
                9 => 300,
                _ => 0,
            };
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    Val::from("So you want to hire a ") + shared::other_global_functions::f_getnumsuffix(ctx, args![grade])?,
                    Val::from("Grade ") + name.clone() + " Mercenary?",
                    "You need to have attained",
                    Val::from("Base Level ") + base_level + " or higher, and",
                    Val::from("must pay the ") + zeny_cost + ",000 zeny fee."
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Yes", "No"])? == 1 {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "Oh, really? Well, now",
                        "might not be a good time",
                        "for you to consider hiring",
                        "a Mercenary, but please feel",
                        "free to come back if your",
                        "needs change. Thank you~"
                    ],
                )?;
            } else if faith_cost != 0 && runtime::op(&faith_merc, "<", &Val::from(faith_cost))?.is_true() {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "Oh... Your Loyalty rating",
                        Val::from("with the ") + name.clone() + " Mercenary",
                        "Guild isn't high enough to",
                        "hire this Mercenary. Please",
                        "come back after you earn",
                        Val::from("") + faith_cost + " or more Loyalty with us."
                    ],
                )?;
            } else if ctx.player().base_level()? < base_level {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "I'm sorry, but your Base",
                        "Level isn't high enough",
                        "to hire this Mercenary.",
                        "Please come back to me",
                        Val::from("once you reach Base Level ") + base_level + "."
                    ],
                )?;
            } else if ctx.player().zeny()? < zeny_cost * 1000 {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "I'm sorry, but you",
                        "don't have enough zeny",
                        "to hire this Mercenary.",
                        Val::from("The hiring fee is ") + zeny_cost + ",000 zeny."
                    ],
                )?;
            } else {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "Great! Our Mercenaries",
                        "are sincere and devoted",
                        "to protecting their clients.",
                        "Summoned Mercenaries will",
                        "offer their support to you for",
                        "30 minutes. Take care now."
                    ],
                )?;
                ctx.player().set_zeny(ctx.player().zeny()? - zeny_cost * 1000)?;
                ctx.call(Function::GetItem, args![item_id.clone().try_sub(10)? + grade, 1])?;
            }
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "Mercenaries are soldiers",
                    "that will fight at your side",
                    "on the battlefield, but there",
                    "are a few terms and conditions",
                    "you must fulfill to hire them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "You must fulfill a level",
                    "requirement and pay a zeny",
                    "fee to hire a Mercenary.",
                    "Higher grade Mercenaries",
                    "will also require that you",
                    "build a Loyalty rating with us."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "Mercenary contracts can't be",
                    "transferred to other people,",
                    "and we only allow a 5 Base Level difference between the Mercenary",
                    "and client so you can't hire one much stronger than you."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "Well, you can figure out the",
                    "details when you actually form",
                    "a contract with one of our",
                    "Mercenaries, and receive",
                    "the Summon Scroll that will",
                    "call a Mercenary to your side."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "You can't give this scroll",
                    "to anyone else, and the",
                    "Mercenary will only remain",
                    "with you for 30 minutes after",
                    "you summon him. Don't forget",
                    "about the time limit, okay?"
                ],
            )?;
            ctx.close()
        }
        2 => {
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "No? You didn't need any",
                    "help? Well, feel free to",
                    "ask me if you have any",
                    "questions about Mercenaries."
                ],
            )?;
            ctx.close()
        }
        3 => {
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "10th Grade Mercenaries are",
                    "the best we have to offer,",
                    "and we use different criteria",
                    "for our clients to hire them.",
                    "There's no zeny fee, but you",
                    "must have 500 Loyalty."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "Once you make a contract",
                    "with a 10th Grade Mercenary,",
                    "your Loyalty rating will be",
                    "decreased by 400. In other",
                    "words, you pay 400 Loyalty",
                    "to hire a 10th Grade Mercenary."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Mercenary Manager",
                args![
                    "You must also be at",
                    "Base Level 90 or higher to",
                    "hire a 10th Grade Mercenary.",
                    "Are you still interested in",
                    "forming this contract?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Yes", "No"])? == 1 {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "I understand... It takes",
                        "hard work and sacrifice to",
                        "even reach the point where",
                        "you can hire a 10th Grade",
                        "Mercenary. Have you considered",
                        "hiring a lower grade Mercenary?"
                    ],
                )?;
            } else if faith_merc.number()? < 500 {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "I'm sorry, but your",
                        "Loyalty rating is too",
                        "low to hire a 10th Grade",
                        "Mercenary. You must have",
                        "500 or more Loyalty to",
                        "form a contract with one."
                    ],
                )?;
            } else if ctx.player().base_level()? < 90 {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "I'm sorry, but you must",
                        "be at Base Level 90 or",
                        "higher to form a contract",
                        "with a 10th Grade Mercenary."
                    ],
                )?;
            } else {
                ctx.lines_as(
                    "Mercenary Manager",
                    args![
                        "Congratulations! It looks",
                        "like you're planning in taking",
                        "on some very dangerous work",
                        "since you're hiring a 10th",
                        "Grade Mercenary. I wish you",
                        "the best of luck with him."
                    ],
                )?;
                ctx.call(Function::MercenarySetFaith, args![runtime::getd(ctx, &faith_key, &[])?, -400])?;
                ctx.call(Function::GetItem, args![item_id.clone(), 1])?;
            }
            ctx.close()
        }
        _ => Ok(()),
    }
}

pub fn mercenary_merchant_dummy(ctx: &Ctx) -> Script {
    let l_item = vec![
        Val::from(12184),
        Val::from(12185),
        Val::from(12241),
        Val::from(12242),
        Val::from(12243),
    ];
    let l_cost = vec![Val::from(2500), Val::from(5000), Val::from(800), Val::from(1500), Val::from(3000)];
    ctx.lines_as(
        "Mercenary Goods Merchant",
        args![
            "Hello, I sell goods",
            "that Mercenaries can",
            "use. Is there anything",
            "in particular that",
            "you're looking for?"
        ],
    )?;
    ctx.next()?;
    let mut menu = Val::from("");
    for item in &l_item {
        menu = menu + ctx.call(Function::GetItemName, args![item.clone()])? + Val::from(":");
    }
    let m = runtime::select_values(ctx, &[menu])? - 1;
    let item = runtime::local_get(&l_item, &Val::from(m), false);
    let cost = runtime::local_get(&l_cost, &Val::from(m), false);
    let cost_text = cost.clone() + Val::from("");
    let cost_len = runtime::strlen(&cost_text).number()?;
    let cost_text = if cost_len <= 3 {
        cost_text
    } else {
        runtime::insertchar(&cost_text, &Val::from(","), &Val::from(cost_len - 3))?
    };
    ctx.lines_as(
        "Mercenary Goods Merchant",
        args![
            ctx.call(Function::GetItemName, args![item.clone()])?,
            Val::from("each cost ") + cost_text + Val::from(" zeny."),
            "How many would you like?"
        ],
    )?;
    ctx.next()?;
    let (input, _) = runtime::input_number(ctx, None, None)?;
    if !input.is_true() {
        ctx.lines_as(
            "Mercenary Goods Merchant",
            args![
                "You changed your mind?",
                "Alright, feel free to come",
                "back to me whenever you want",
                "to buy any Mercenary Potions."
            ],
        )?;
        return ctx.close();
    }
    if input.number()? < 0 || input.number()? > 10000 {
        ctx.lines_as(
            "Mercenary Goods Merchant",
            args![
                "I'm sorry, but you",
                "can only buy up to",
                "10,000 of these potions",
                "at a time. Please enter",
                "a number from 1 to 10,000."
            ],
        )?;
        return ctx.close();
    }
    let total = input.number()? * cost.number()?;
    if total > ctx.player().zeny()? {
        ctx.lines_as(
            "Mercenary Goods Merchant",
            args![
                "I'm sorry, but you don't",
                "have enough zeny for this",
                "many potions. Well, I'll be",
                "be here when you're ready",
                "to purchase something",
                "for your Mercenaries."
            ],
        )?;
        return ctx.close();
    }
    if !ctx.call(Function::CheckWeight, args![item.clone(), input.clone()])?.is_true() {
        ctx.lines_as(
            "Mercenary Goods Merchant",
            args![
                "If I gave you that many",
                "potions, you wouldn't be",
                "able to carry them with you.",
                "Please come back after",
                "you free up some space",
                "in your Inventory."
            ],
        )?;
        return ctx.close();
    }
    ctx.mes("[Mercenary Goods Merchant]")?;
    if input == 1 {
        ctx.lines(args![
            Val::from("Here's your ") + ctx.call(Function::GetItemName, args![item.clone()])? + Val::from(".")
        ])?;
    } else {
        ctx.lines(args![
            "Here you are, this is exactly",
            shared::other_global_functions::f_insertplural(
                ctx,
                args![input.clone(), ctx.call(Function::GetItemName, args![item.clone()])?]
            )? + Val::from(".")
        ])?;
    }
    ctx.lines(args![
        "Thank you, and please come",
        "again when you need more",
        "potions for your Mercenaries."
    ])?;
    ctx.player().set_zeny(ctx.player().zeny()? - total)?;
    ctx.call(Function::GetItem, args![item, input])?;
    ctx.close()
}

pub fn mercenary_switch(ctx: &Ctx) -> Script {
    let l_name_s = vec![Val::from("Spear"), Val::from("Sword"), Val::from("Bow")];
    let npc_s = ctx.call(Function::StrNpcInfo, args![2])?;
    let mut kind = 0;
    while kind < l_name_s.len() as i32 {
        if npc_s.loosely_equals(&runtime::local_get(&l_name_s, &Val::from(kind), true)) {
            break;
        }
        kind += 1;
    }
    let name = runtime::local_get(&l_name_s, &Val::from(kind), true);
    ctx.lines_as("Checker", args!["Please input a password."])?;
    ctx.next()?;
    let l_i = shared::other_gm_npcs::f_gm_npc(ctx, args![1854, 0, 0, 10000])?;
    ctx.mes("[Checker]")?;
    if l_i == -2 {
        ctx.mes("Error.")?;
    } else if l_i == 0 {
        ctx.mes("Wrong number")?;
    } else if l_i == 1 {
        ctx.mes("Please select.")?;
        ctx.next()?;
        match ctx.menu(&["Turn off Mercenary NPC", "Turn on Mercenary NPC"])? {
            0 => {
                ctx.mes("NPCs are turned off.")?;
                ctx.call(Function::DisableNpc, args![Val::from("Mercenary Manager#") + name.clone()])?;
                ctx.call(Function::DisableNpc, args![Val::from("Mercenary Merchant#") + name.clone()])?;
            }
            1 => {
                ctx.mes("NPCs are turned on.")?;
                ctx.call(Function::EnableNpc, args![Val::from("Mercenary Manager#") + name.clone()])?;
                ctx.call(Function::EnableNpc, args![Val::from("Mercenary Merchant#") + name.clone()])?;
            }
            _ => {}
        }
    }
    ctx.close()
}
