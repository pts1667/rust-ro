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

#[derive(Clone, Copy, Debug)]
enum ApprenticeCraftsmanStep {
    Start,
    EnchantArmor,
}

fn set_items(items: &mut Vec<Val>, ids: &[i32]) {
    for (i, &id) in ids.iter().enumerate() {
        runtime::local_set(items, &Val::from(i as i32), Val::from(id), false);
    }
}

fn apprentice_craftsman_run(ctx: &Ctx, mut step: ApprenticeCraftsmanStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ApprenticeCraftsmanStep::Start => {
                let mut l_items: Vec<Val> = Vec::new();
                let mut l_j: i32 = 0;
                let mut l_k = false;
                ctx.mes("[Apprentice Craftsman]")?;
                if ctx.player().zeny()? >= 400000 {
                    ctx.mes("I've been studying ways to enhance an armor to maximize its capability.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Apprentice Craftsman",
                        args!["Enchanting is an awesome skill that infuses a mysterious status powers into the armor's hidden socket."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Apprentice Craftsman", args!["However, you have to keep in mind that if there are two armors of the same kind in your possession, the Enchantment will be applied in the order they are placed in your inventory."])?;
                    ctx.next()?;
                    ctx.lines_as("Apprentice Craftsman", args!["In that case, the Enchantment may be applied to an item which you didn't mean to Enchant. So just bring ^5555ffONE Armor^000000 you want enchanted to be safe..."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Apprentice Craftsman",
                        args!["I'm not responsible for what would happen if you have more than one of the same kind in your inventory."],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Non Slotted Armor.", "Slotted Armor.", "High Grade Armor.", "Maybe next time."])? {
                        0 => {
                            set_items(
                                &mut l_items,
                                &[
                                    2307, 2309, 2314, 2316, 2321, 2325, 2327, 2328, 2330, 2332, 2334, 2335, 2341, 2344, 2346, 2348, 2350,
                                    2337, 2386, 2394, 2395, 2396,
                                ],
                            );
                            l_j = 50;
                        }
                        1 => {
                            set_items(
                                &mut l_items,
                                &[
                                    2311, 2318, 2319, 2320, 2308, 2310, 2315, 2317, 2322, 2324, 2326, 2331, 2333, 2336, 2342, 2345, 2347,
                                    2349, 2351,
                                ],
                            );
                            l_j = 55;
                            l_k = true;
                        }
                        2 => {
                            set_items(
                                &mut l_items,
                                &[
                                    2364, 2365, 2391, 2374, 2375, 2376, 2377, 2378, 2379, 2380, 2381, 2382, 2387, 2388, 2389, 2390,
                                ],
                            );
                            l_j = 60;
                        }
                        3 => {
                            ctx.lines_as(
                                "Apprentice Craftsman",
                                args!["Please come back when you have any interest in enchanting your armor."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                    let mut l_menu_s = Val::from("");
                    for i in 0..l_items.len() as i32 {
                        l_menu_s = l_menu_s
                            + ctx.call(Function::GetItemName, args![runtime::local_get(&l_items, &Val::from(i), false)])?
                            + Val::from(if l_k { " [1]" } else { "" })
                            + Val::from(":");
                    }
                    apprentice_craftsman_run(
                        ctx,
                        ApprenticeCraftsmanStep::EnchantArmor,
                        args![
                            runtime::local_get(
                                &l_items,
                                &Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])? - 1),
                                false
                            ),
                            l_j
                        ],
                    )?;
                    return Err(Stop::End);
                }
                ctx.mes("I am in charge of Enchanting Armors. Simply put, I've been studying ways to power-up armor.")?;
                ctx.next()?;
                ctx.lines_as("Apprentice Craftsman", args!["If by any chance, you would want to enchant your armor, bring me 400,000 zeny and the armor you want to enchant and you are all set to go."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ApprenticeCraftsmanStep::EnchantArmor => {
                let l_itemid = runtime::arg(&args, 0, Val::from(0));
                let l_failrate = runtime::arg(&args, 1, Val::from(0));
                ctx.mes("[Apprentice Craftsman]")?;
                if ctx.call(Function::CountItem, args![l_itemid.clone()])? == 1 {
                    ctx.mes("Socket enchant will cost you 400,000 zeny. And there will be a random option enchanted. Of course, there is a chance of breaking your armor.")?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Apprentice Craftsman",
                        args![
                            "First and most importantly.",
                            "^ff5555Existing Refine Level of the Armor",
                            "and Cards will be GONE.^000000",
                            "Do you still want to try an Enchant?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Hmm... Let me think it over.", "Go ahead."])? == 0 {
                        ctx.lines_as(
                            "Apprentice Craftsman",
                            args!["Well, I can't blame you. Safety first, eh?", "Now you have a nice day."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as("Apprentice Craftsman", args!["Quite of an adventurer huh? Well, shall we?"])?;
                    ctx.close_window()?;
                    ctx.call(Function::SpecialEffect, args![constants::EF_MAPPILLAR])?;
                    if ctx.player().zeny()? < 400000 {
                        ctx.lines_as("Apprentice Craftsman", args!["Sorry, but you don't have enough zeny."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.call(Function::ProgressBar, args!["ffff00", 7])?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 400000)?;
                    ctx.call(Function::DelItem, args![l_itemid.clone(), 1])?;
                    let l_addpart: i32 = match ctx.call(Function::Rand, args![1, l_failrate.clone()])?.number()? {
                        1 => 4702,
                        2 => 4712,
                        3 => 4722,
                        4 => 4732,
                        5 => 4742,
                        6 => 4752,
                        7 | 8 => 4701,
                        9 | 10 => 4711,
                        11 | 12 => 4721,
                        13 | 14 => 4731,
                        15 | 16 => 4741,
                        17 | 18 => 4751,
                        19..=21 => 4700,
                        22..=24 => 4710,
                        25..=27 => 4720,
                        28..=30 => 4730,
                        31..=33 => 4740,
                        34..=36 => 4750,
                        _ => {
                            ctx.call(Function::SpecialEffect, args![constants::EF_PHARMACY_FAIL])?;
                            ctx.lines_as(
                                "Apprentice Craftsman",
                                args!["Well that's too bad.", "The requested equipment has failed to enchant."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    };
                    ctx.call(Function::GetItem2, args![l_itemid.clone(), 1, 1, 0, 0, 0, 0, 0, l_addpart])?;
                } else {
                    ctx.lines(args![
                        "Hmm? There's nothing to be enchanted!",
                        "Please come back with just ONE equipment to be enchanted."
                    ])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn apprentice_craftsman(ctx: &Ctx) -> Script {
    apprentice_craftsman_run(ctx, ApprenticeCraftsmanStep::Start, Vec::new()).map(|_| ())
}
