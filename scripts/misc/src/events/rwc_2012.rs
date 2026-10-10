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

pub fn driller_pron(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.mes("You're carrying too many items in your inventory. Visit Kafra storage and try again.")?;
        return ctx.close();
    }
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 10000 {
        ctx.mes("You cannot proceed because you're overweight.")?;
        return ctx.close();
    }
    let part = ctx.constant("EQI_ACC_L")?;
    if !ctx.call(Function::GetEquipIsEquipped, args![part.clone()])?.is_true() {
        ctx.lines_as(
            "Driller",
            args!["My job is to drill a card slot into RWC Memorial accessories."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Driller",
            args!["I'm sorry but you don't have any item equipped on your right accessory position."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Driller", args!["My job is to drill a card slot into RWC Memorial accessories. Moreover, I only treat ^ff0000pure items^000000, or those which have not been enchanted."])?;
    ctx.next()?;
    let equip_id = ctx.call(Function::GetEquipId, args![part.clone()])?;
    if equip_id != 2966 && equip_id != 2968 {
        ctx.lines_as("Driller", args!["However, I can see that the accessory you are wearing on the right side cannot be treated. Please equip a RWC Memorial accessory."])?;
        return ctx.close();
    }
    ctx.lines_as("Driller", args!["You should also know that making a card slot it extremely dangerous. ^ff0000Chances to succeed are about 50%.^000000 Do you wish to proceed?"])?;
    ctx.next()?;
    if ctx.menu(&["Cancel", "Let's go!"])? == 0 {
        ctx.lines_as("Driller", args!["See ya then."])?;
        return ctx.close();
    }
    let (slot_item, name, kind) = if equip_id == 2966 {
        (2967, "RWC 2012 Memorial Ring", "ring")
    } else {
        (2969, "RWC 2012 Memorial Pendant", "pendant")
    };
    if ctx.call(Function::GetEquipCardId, args![part.clone(), 3])?.number()? > 0 {
        ctx.lines_as(
            "Driller",
            args!["This item has already been enchanted. I can't work on this as it is against the rules."],
        )?;
        return ctx.close();
    }
    if shared::other_global_functions::f_isequipidhack(ctx, args![part.clone(), equip_id.clone()])?.is_true() {
        return ctx.close();
    }
    ctx.call(Function::DelEquip, args![part.clone()])?;
    if ctx.rand_range(1, 10)? > 5 {
        ctx.call(Function::GetItem, args![slot_item, 1])?;
        ctx.call(Function::SpecialEffect, args![constants::EF_REPAIRWEAPON])?;
        ctx.lines_as(
            "Driller",
            args![Val::from("Yay! Success! Your ") + Val::from(name) + Val::from(" now has a card slot. Check it out!")],
        )?;
        return ctx.close();
    }
    ctx.call(Function::SpecialEffect, args![constants::EF_LORD])?;
    ctx.lines_as(
        "Driller",
        args![Val::from("Awww... Damn weak ") + Val::from(kind) + Val::from("... It broke during the procedure. I'm sorry.")],
    )?;
    ctx.close()
}

pub fn goldberg_pron(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.mes("You are carrying too many items. Come back after you have organized your inventory.")?;
        return ctx.close();
    }
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 10000 {
        ctx.mes("You cannot proceed because you're overweight.")?;
        return ctx.close();
    }
    ctx.lines_as(
        "Goldberg",
        args!["Hello! I am in charge of enchanting RWC Memorial accessories with some mystic powers."],
    )?;
    ctx.next()?;
    let part = ctx.constant("EQI_ACC_L")?;
    if !ctx.call(Function::GetEquipIsEquipped, args![part.clone()])?.is_true() {
        ctx.lines_as(
            "Goldberg",
            args!["I'm sorry but you don't have any item equipped on your right accessory position."],
        )?;
        return ctx.close();
    }
    let equip_id = ctx.call(Function::GetEquipId, args![part.clone()])?;
    if !(2966..=2969).contains(&equip_id.clone().number()?) {
        ctx.lines_as("Goldberg", args!["However, I can see that the accessory you are wearing is not something I can work on. Please equip a RWC Memorial accessory."])?;
        return ctx.close();
    }
    let action = ctx.menu(&["Sorry, not interested.", "Please, empower my accessory.", "Remove the Enchant."])?;
    if action == 0 {
        ctx.lines_as("Goldberg", args!["Alright, then, see you next time..."])?;
        return ctx.close();
    }
    let mut equip_card: Vec<Val> = Vec::new();
    for i in 0..4_i32 {
        let card = ctx.call(Function::GetEquipCardId, args![part.clone(), i])?;
        runtime::local_set(&mut equip_card, &Val::from(i), card, false);
    }
    if action == 1 {
        let subject = ctx.call(Function::GetEquipId, args![part.clone()])?;
        let option = if subject == 2966 {
            [2, 2, 4, 4]
        } else if subject == 2967 {
            [0, 2, 4, 4]
        } else if subject == 2968 {
            [1, 1, 3, 3]
        } else if subject == 2969 {
            [0, 1, 3, 3]
        } else {
            ctx.lines_as(
                "Goldberg",
                args![
                    "I'm sorry, but I cannot work on the accessory you are currently wearing.",
                    "If you have equipped your RWC Memorial accessory on the left side, try to swap it to the right side."
                ],
            )?;
            return ctx.close();
        };
        let mut slot = 0;
        let mut op_type = 0;
        for i in (0..4_i32).rev() {
            if runtime::local_get(&equip_card, &Val::from(i), false) == 0 {
                slot = i;
                op_type = option[i as usize];
                break;
            }
        }
        let (choices, pick): (Vec<i32>, usize) = match op_type {
            4 => {
                ctx.lines_as("Goldberg", args!["Which enchantment would you like to infuse?"])?;
                ctx.next()?;
                (
                    vec![1, 2, 3, 4],
                    ctx.menu(&["Cancel", "Fighting Spirit", "ATK (%)", "Max HP", "HP"])?,
                )
            }
            3 => (vec![5, 6, 7], ctx.menu(&["Cancel", "Spell", "MATK (%)", "SP"])?),
            2 => {
                ctx.lines_as("Goldberg", args!["^ff0000Be careful! There is about a 25% chance that the enchantment will fail. If this happens, the item will be destroyed.^000000 Which enchantment would you like to infuse?"])?;
                ctx.next()?;
                (
                    vec![8, 9, 10, 11, 12, 13, 14],
                    ctx.menu(&["Cancel", "STR", "AGI", "VIT", "INT", "DEX", "LUK", "SP"])?,
                )
            }
            1 => {
                ctx.lines_as("Goldberg", args!["^ff0000There is about a 25% chance that the enchantment will fail. If this happens, the item will be destroyed.^000000 Which enchantment would you like to infuse?"])?;
                ctx.next()?;
                (
                    vec![8, 9, 10, 11, 12, 13, 15, 16],
                    ctx.menu(&["Cancel", "STR", "AGI", "VIT", "INT", "DEX", "LUK", "MHP", "HP"])?,
                )
            }
            _ => {
                ctx.lines_as(
                    "Goldberg",
                    args!["Your accessory has received so many enchantments that I can hardly work on it anymore."],
                )?;
                return ctx.close();
            }
        };
        if pick == 0 {
            ctx.lines_as("Goldberg", args!["Alright, then, see you next time."])?;
            return ctx.close();
        }
        let enchant_type = choices[pick - 1];
        ctx.lines_as("Goldberg", args!["The power of the enchantment will be randomly chosen. ^ff0000Once infused, the enchantment cannot be removed.^000000 Shall we continue?"])?;
        ctx.next()?;
        if ctx.menu(&["No, please stop.", "Yes, please proceed."])? == 0 {
            ctx.lines_as("Goldberg", args!["Alright, then, see you next time..."])?;
            return ctx.close();
        }
        if !ctx.call(Function::GetEquipIsEquipped, args![part.clone()])?.is_true() {
            ctx.lines_as("Goldberg", args!["Do not take off your equipment while I'm working, okay?"])?;
            return ctx.close();
        }
        let enc: [i32; 3] = match enchant_type {
            1 => [4811, 4810, 4809],
            2 => [4819, 4766, 4767],
            3 | 15 => [4861, 4862, 4867],
            4 | 16 => [4795, 4796, 4797],
            5 => [4760, 4761, 4806],
            6 => [4815, 4814, 4813],
            7 | 14 => [4870, 4800, 4871],
            8 => [4700, 4701, 4702],
            9 => [4730, 4731, 4732],
            10 => [4740, 4741, 4742],
            11 => [4710, 4711, 4712],
            12 => [4720, 4721, 4722],
            13 => [4750, 4751, 4752],
            _ => {
                ctx.lines_as("Goldberg", args!["We have got a problem, let me check it up."])?;
                return ctx.close();
            }
        };
        let roll = if enchant_type < 8 {
            ctx.rand_range(1, 300)?
        } else {
            ctx.rand_range(1, 400)?
        };
        let enchant = if roll < 151 {
            enc[0]
        } else if roll < 251 {
            enc[1]
        } else if roll < 301 {
            enc[2]
        } else {
            9
        };
        if shared::other_global_functions::f_isequipidhack(ctx, args![part.clone(), equip_id.clone()])?.is_true()
            || shared::other_global_functions::f_isequipcardhack(
                ctx,
                args![
                    part.clone(),
                    runtime::local_get(&equip_card, &Val::from(0), false),
                    runtime::local_get(&equip_card, &Val::from(1), false),
                    runtime::local_get(&equip_card, &Val::from(2), false),
                    runtime::local_get(&equip_card, &Val::from(3), false)
                ],
            )?
            .is_true()
        {
            return ctx.close();
        }
        runtime::local_set(&mut equip_card, &Val::from(slot), Val::from(enchant), false);
        if enchant == 0 {
            for i in slot + 1..4 {
                runtime::local_set(&mut equip_card, &Val::from(i), Val::from(0), false);
            }
        }
        let equip_refine = ctx.call(Function::GetEquipRefineryCnt, args![part.clone()])?;
        ctx.call(Function::DelEquip, args![part.clone()])?;
        if enchant == 9 {
            ctx.lines_as(
                "Goldberg",
                args![
                    "Oh gosh!",
                    "The item was not strong enough to bear the enchantment and thus got destroyed. I am sorry."
                ],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_LORD])?;
            return ctx.close();
        }
        if enchant == 0 {
            ctx.lines_as("Goldberg", args!["Oh... It looks like there was an instability of some sort between all the powers infused. This caused all the enchantments to vanish. It is a shame, but please try again!"])?;
        } else {
            ctx.lines_as(
                "Goldberg",
                args![
                    "Great!",
                    (Val::from("The enchantment is a success! It will be applied in socket No.^990000") + Val::from(slot + 1))
                        + Val::from("^000000.")
                ],
            )?;
            ctx.call(Function::SpecialEffect, args![constants::EF_REPAIRWEAPON])?;
        }
        ctx.call(
            Function::GetItem2,
            args![
                equip_id.clone(),
                1,
                1,
                equip_refine,
                0,
                runtime::local_get(&equip_card, &Val::from(0), false),
                runtime::local_get(&equip_card, &Val::from(1), false),
                runtime::local_get(&equip_card, &Val::from(2), false),
                runtime::local_get(&equip_card, &Val::from(3), false)
            ],
        )?;
        return ctx.close();
    }
    if action == 2 {
        ctx.lines_as(
            "Goldberg",
            args!["I will just initialize the enchant option without doing anything to the slotted card. You wanna continue?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["I will stop.", "Yep, sure, go on."])? == 0 {
            ctx.lines_as("Goldberg", args!["Come back if you change your mind."])?;
            return ctx.close();
        }
        if ctx.call(Function::CountItem, args![6665])? == 0 {
            ctx.lines_as(
                "Goldberg",
                args!["I'm sorry. But you don't have the RWC Initialization coupon. Can you check your inventory?"],
            )?;
            return ctx.close();
        }
        if ctx.call(
            Function::GetItemInfo,
            args![runtime::local_get(&equip_card, &Val::from(3), false), constants::ITEMINFO_SUBTYPE],
        )? != constants::CARD_ENCHANT
        {
            ctx.lines_as(
                "Goldberg",
                args!["Hm... this equipment is clean. I cannot initialize it if there's nothing! Check it again."],
            )?;
            return ctx.close();
        }
        ctx.call(Function::SpecialEffect, args![constants::EF_REPAIRWEAPON])?;
        ctx.lines_as("Goldberg", args!["The enchant option in your item will be initialized."])?;
        ctx.items().take(6665, 1)?;
        ctx.call(Function::DelEquip, args![part.clone()])?;
        let mut i = ctx
            .call(Function::GetItemInfo, args![equip_id.clone(), constants::ITEMINFO_SLOT])?
            .number()?;
        while i < constants::MAX_SLOTS {
            if ctx.call(
                Function::GetItemInfo,
                args![runtime::local_get(&equip_card, &Val::from(i), false), constants::ITEMINFO_SUBTYPE],
            )? == constants::CARD_ENCHANT
            {
                runtime::local_set(&mut equip_card, &Val::from(i), Val::from(0), false);
            }
            i += 1;
        }
        // Refine count is not read on this path.
        ctx.call(
            Function::GetItem2,
            args![
                equip_id.clone(),
                1,
                1,
                0,
                0,
                runtime::local_get(&equip_card, &Val::from(0), false),
                runtime::local_get(&equip_card, &Val::from(1), false),
                runtime::local_get(&equip_card, &Val::from(2), false),
                runtime::local_get(&equip_card, &Val::from(3), false)
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}
