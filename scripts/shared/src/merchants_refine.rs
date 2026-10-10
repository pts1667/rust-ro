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

pub fn orimain(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_npc_name_s = runtime::arg(&args, 0, Val::from(0));
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
        l_npc_name_s.clone(),
        args![
            "I can purify your",
            "Rough Oridecons or",
            "Rough Eluniums. I'll need",
            "5 Rough Stones to make",
            "1 pure one for you."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Make Oridecon", "Make Elunium", "Ask about Enchanted Stones"])? {
        0 => {
            if ctx.items().count(756)? > 4 {
                ctx.items().take(756, 5)?;
                ctx.items().give(984, 1)?;
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args!["Here's your Oridecon.", "You're welcome to come", "back whenever you want."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args![
                        "You're kidding me, right?",
                        "I just told you that I need 5 Rough Oridecons to make a pure Oridecon."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        1 => {
            if ctx.items().count(757)? > 4 {
                ctx.items().take(757, 5)?;
                ctx.items().give(985, 1)?;
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args!["Here's your Elunium.", "You're welcome to come", "back whenever you want."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args![
                        "You're kidding me, right?",
                        "I just told you that I need 5 Rough Eluniums to make a pure Elunium."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Enchanted Stones...?",
                    "I've been a stonesmith for 20 years, so I've heard a lot about them. Supposedly, there are",
                    "four different kinds."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["Each Enchanted Stone possesses one of the following elemental properties: Earth, Wind, Water and Fire."],
            )?;
            ctx.next()?;
            ctx.lines_as(l_npc_name_s.clone(), args!["If someone combines a Enchanted Stone with a weapon while smithing, that weapon will possess the same property as the Stone."])?;
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["Needless to say, you need to have some smithing skill to produce this kind of elemental weapon."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn phramain(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    let mut l_material = Val::from(0);
    let l_npc_name_s = runtime::arg(&args, 0, Val::from(0));
    let mut l_price = 0;
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
        l_npc_name_s.clone(),
        args![
            "I sell 2 kinds of Metal",
            "for tempering weaponry.",
            "I have ^007777Phracon^000000 for Level 1",
            "Weapons, and ^007777Emveretarcon^000000",
            "for Level 2 Weapons."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Phracon - 200 Zeny", "Emveretarcon - 1000 Zeny", "Ask about other Metals"])? {
        0 => {
            l_material = Val::from(1010);
            l_price = 200;
        }
        1 => {
            l_material = Val::from(1011);
            l_price = 1000;
        }
        2 => {
            ctx.lines_as(l_npc_name_s.clone(), args!["Other metals?", "Well, you'll need special metals to upgrade higher level weapons, or any kind of armor. But you know, Oridecon and Elunium is really", "hard to just find..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            "So how many do you wish to buy?",
            "If you don't want any, please enter the number, '0.'"
        ],
    )?;
    ctx.next()?;
    loop {
        let (input, status) = runtime::input_number(ctx, None, None)?;
        l_input = input;
        if l_input == 0 {
            ctx.lines_as(l_npc_name_s.clone(), args!["The deal has", "been cancelled."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if l_input.number()? < 0 || l_input.number()? > 500 {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["Alright, you can", "puchase up to 500.", "No more than that,", "got it? Good."],
            )?;
            ctx.next()?;
        } else {
            break;
        }
    }
    let l_sell = l_input.number()? * l_price;
    if ctx.player().zeny()? < l_sell {
        ctx.lines_as(
            l_npc_name_s.clone(),
            args![
                "Err...",
                "You don't have",
                "enough Zeny to buy",
                ((Val::from("") + l_input.clone()) + Val::from(" of them."))
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CheckWeight, args![l_material.clone(), l_input.clone()])? == 0 {
        ctx.lines_as(l_npc_name_s.clone(), args!["Hmm...", "I can't give you anything if you don't have enough room in your inventory. Why don't you put your extra things in Kafra Storage and try again?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::GetItem, args![l_material.clone(), l_input.clone()])?;
    ctx.player().set_zeny(ctx.player().zeny()? - l_sell)?;
    ctx.lines_as(l_npc_name_s.clone(), args!["Here you are!", "Thank you for", "your patronage."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn refinemain(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_card: Vec<Val> = Vec::new();
    let mut l_equip_lv = Val::from(0);
    let mut l_equipped = Val::from(0);
    let mut l_fullprice = Val::from(0);
    let mut l_indices: Vec<Val> = Vec::new();
    let mut l_itemtype = Val::from(0);
    let mut l_material = Val::from(0);
    let mut l_menu2 = Val::from(0);
    let mut l_menu_s = Val::from("");
    let mut l_part = Val::from(0);
    let mut l_price = Val::from(0);
    let mut l_refinecheck = Val::from(0);
    let mut l_refinecnt = Val::from(0);
    let mut l_refineitemid = Val::from(0);
    let mut l_refinerycnt = Val::from(0);
    let mut l_safe = Val::from(0);
    let l_npc_name_s = runtime::arg(&args, 0, Val::from(0));
    let l_features = runtime::arg(&args, 1, Val::from(0));
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            "I'm the Armsmith.",
            "I can refine all kinds of weapons, armor and equipment, so let me",
            "know what you want me to refine."
        ],
    )?;
    ctx.next()?;
    for (index, name) in [
        "EQI_HEAD_TOP",
        "EQI_ARMOR",
        "EQI_HAND_L",
        "EQI_HAND_R",
        "EQI_GARMENT",
        "EQI_SHOES",
        "EQI_ACC_L",
        "EQI_ACC_R",
        "EQI_HEAD_MID",
        "EQI_HEAD_LOW",
    ]
    .into_iter()
    .enumerate()
    {
        runtime::local_set(&mut l_indices, &Val::from(index as i32 + 1), ctx.constant(name)?, false);
    }
    let mut l_i = 1;
    while l_i < l_indices.len() as i32 {
        if ctx
            .call(
                Function::GetEquipIsEquipped,
                args![runtime::local_get(&l_indices, &Val::from(l_i), false)],
            )?
            .is_true()
        {
            l_menu_s = ((((l_menu_s.clone()
                + crate::other_global_functions::f_getpositionname(ctx, vec![runtime::local_get(&l_indices, &Val::from(l_i), false)])?)
                + Val::from("-["))
                + ctx.call(
                    Function::GetEquipName,
                    args![runtime::local_get(&l_indices, &Val::from(l_i), false)],
                )?)
                + Val::from("]"));
            l_equipped = Val::from(1);
        }
        l_menu_s = (l_menu_s.clone() + Val::from(":"));
        l_i += 1;
    }
    if l_equipped == 0 {
        ctx.lines_as(l_npc_name_s.clone(), args!["I don't think I can refine any items you have..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_part = runtime::local_get(&l_indices, &Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?), false);
    if !(ctx.call(Function::GetEquipIsEquipped, args![l_part.clone()])?.is_true()) {
        ctx.lines_as(
            l_npc_name_s.clone(),
            args!["You're not wearing", "anything there that", "I can refine."],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !(ctx.call(Function::GetEquipIsEnableRefine, args![l_part.clone()])?.is_true()) {
        ctx.lines_as(l_npc_name_s.clone(), args!["I don't think I can", "refine this item at all..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::GetEquipRefineryCnt, args![l_part.clone()])?.number()? >= 10 {
        ctx.lines_as(
            l_npc_name_s.clone(),
            args!["I can't refine this", "any more. This is as", "refined as it gets!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_refineitemid = ctx.call(Function::GetEquipId, args![l_part.clone()])?;
    l_refinerycnt = ctx.call(Function::GetEquipRefineryCnt, args![l_part.clone()])?;
    for index in 0..4_i32 {
        runtime::local_set(
            &mut l_card,
            &Val::from(index),
            ctx.call(Function::GetEquipCardId, args![l_part.clone(), index])?,
            false,
        );
    }
    l_price = ctx.call(
        Function::GetEquipRefineCost,
        args![
            l_part.clone(),
            ctx.constant("REFINE_COST_NORMAL")?,
            ctx.constant("REFINE_ZENY_COST")?
        ],
    )?;
    l_material = ctx.call(
        Function::GetEquipRefineCost,
        args![
            l_part.clone(),
            ctx.constant("REFINE_COST_NORMAL")?,
            ctx.constant("REFINE_MATERIAL_ID")?
        ],
    )?;
    l_itemtype = ctx.call(
        Function::GetItemInfo,
        args![l_refineitemid.clone(), ctx.constant("ITEMINFO_TYPE")?],
    )?;
    if l_itemtype.loosely_equals(&ctx.constant("IT_ARMOR")?) {
        l_equip_lv = ctx.call(Function::GetEquipArmorLevel, args![l_part.clone()])?;
        if l_equip_lv == 1 {
            l_safe = Val::from(4);
        } else {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if (ctx.constant("VIP_SCRIPT")?.is_true() && !(ctx.call(Function::VipStatus, vec![ctx.constant("VIP_STATUS_ACTIVE")?])?.is_true()))
        {
            if l_equip_lv == 1 {
                l_price = l_price.try_mul(Val::from(10))?;
            } else {
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if l_itemtype.loosely_equals(&ctx.constant("IT_WEAPON")?) {
        l_equip_lv = ctx.call(Function::GetEquipWeaponLevel, args![l_part.clone()])?;
        if l_equip_lv == 1 {
            l_safe = Val::from(7);
        } else if l_equip_lv == 2 {
            l_safe = Val::from(6);
        } else if l_equip_lv == 3 {
            l_safe = Val::from(5);
        } else if l_equip_lv == 4 {
            l_safe = Val::from(4);
        } else {
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.constant("VIP_SCRIPT")?.is_true() && !ctx.call(Function::VipStatus, args![ctx.constant("VIP_STATUS_ACTIVE")?])?.is_true() {
            if l_equip_lv == 1 {
                l_price = l_price.try_mul(Val::from(40))?;
            } else if l_equip_lv == 2 {
                l_price = l_price.try_mul(Val::from(50))?;
            } else if l_equip_lv == 3 || l_equip_lv == 4 {
                l_price = l_price.try_mul(Val::from(2))?;
            } else {
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        l_safe = Val::from(4);
    }
    if l_features != 1 {
        ctx.lines_as(
            l_npc_name_s.clone(),
            args![
                "To refine this I need",
                ((Val::from("one ^003366") + ctx.call(Function::GetItemName, args![l_material.clone()])?) + Val::from("^000000 and")),
                ((Val::from("a service fee of ") + l_price.clone()) + Val::from(" Zeny.")),
                "Do you really wish to continue?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 1 {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["Yeah...", "There's no need to", "rush. Take your time."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.call(Function::GetEquipPercentRefinery, args![l_part.clone()])?.number()? < 100 {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["Oh no! If I continue to", "refine this, there's a risk it could"],
            )?;
            if l_material == 985 {
                ctx.mes("be destroyed! That means that ^FF0000this equipment^000000, and ^FF0000any cards^000000 or special properties added to this armor, ^FF0000will be gone^000000.")?;
            } else {
                ctx.lines(args![
                    "be destroyed, and you'd ^FF0000lose the weapon^000000, any ^FF0000cards in the weapon^000000,",
                    "or any added special properties."
                ])?;
            }
            ctx.next()?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "I can't make it any clearer.",
                    "Once a weapon is destroyed,",
                    "there's no getting it back.",
                    "You really have a chance to",
                    "^FF0000lose this weapon^000000 forever.",
                    "Do you still want to refine?"
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Yes", "No"])? == 1 {
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args![
                        "I completely agree...",
                        "I might be a great refiner, but sometimes even I make mistakes."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        if (ctx.call(Function::CountItem, args![l_material.clone()])?.number()? < 1
            || runtime::op(&ctx.var("Zeny").get()?, "<", &l_price)?.is_true())
        {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "You don't seem to have",
                    ((Val::from("enough Zeny or ") + ctx.call(Function::GetItemName, args![l_material.clone()])?) + Val::from("...")),
                    "Go get some more. I'll be",
                    "here all day if you need me."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_price.clone())?))?;
        ctx.call(Function::DelItem, args![l_material.clone(), 1])?;
        if ((crate::other_global_functions::f_isequipidhack(ctx, args![l_part.clone(), l_refineitemid.clone()])?.is_true()
            || crate::other_global_functions::f_isequipcardhack(
                ctx,
                vec![
                    l_part.clone(),
                    runtime::local_get(&l_card, &Val::from(0), false),
                    runtime::local_get(&l_card, &Val::from(1), false),
                    runtime::local_get(&l_card, &Val::from(2), false),
                    runtime::local_get(&l_card, &Val::from(3), false),
                ],
            )?
            .is_true())
            || crate::other_global_functions::f_isequiprefinehack(ctx, args![l_part.clone(), l_refinerycnt.clone()])?.is_true())
        {
            ctx.lines(args![((Val::from("[") + l_npc_name_s.clone()) + Val::from("]"))])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
            ctx.lines(args![
                "Wait a second...",
                "Do you think I'm stupid?!",
                "You switched the item while I wasn't looking! Get out of here!"
            ])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if runtime::op(
            &ctx.call(Function::GetEquipPercentRefinery, args![l_part.clone()])?,
            "<=",
            &ctx.call(Function::Rand, args![100])?,
        )?
        .is_true()
        {
            ctx.call(Function::FailedRefineItem, args![l_part.clone()])?;
            ctx.lines(args![((Val::from("[") + l_npc_name_s.clone()) + Val::from("]"))])?;
            ctx.call(
                Function::Emotion,
                vec![
                    (if !(ctx.call(Function::Rand, args![5])?.is_true()) {
                        ctx.constant("ET_MONEY")?
                    } else {
                        ctx.constant("ET_HUK")?
                    }),
                ],
            )?;
            let l_lose = ctx.call(Function::Rand, args![1, 3])?;
            if l_lose == 1 {
                ctx.lines(args![
                    "OH! MY GOD!",
                    "Damn it! Not again!",
                    "I'm terribly sorry, but you know practice does make perfect.",
                    "Um, right? Heh heh..."
                ])?;
            } else if l_lose == 2 {
                ctx.lines(args!["Nooooooo!", "It broke!", "I-I'm sorry!"])?;
            } else {
                ctx.lines(args![
                    "Crap!",
                    "It couldn't take",
                    "much more tempering!",
                    "Sorry about this..."
                ])?;
            }
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines(args![((Val::from("[") + l_npc_name_s.clone()) + Val::from("]"))])?;
        ctx.call(Function::SuccessRefineItem, args![l_part.clone()])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        let l_win = ctx.call(Function::Rand, args![1, 3])?;
        if l_win == 1 {
            ctx.lines(args![
                "Perfect!",
                "Heh heh!",
                "Once again,",
                "flawless work",
                "from the master~"
            ])?;
        } else if l_win == 2 {
            ctx.lines(args![
                "Success...!",
                "Yet again, my amazing",
                "talent truly dazzles",
                "and shines today."
            ])?;
        } else {
            ctx.lines(args![
                "Heh heh!",
                "I'm all done.",
                "No doubt, my work is",
                "to your satisfaction.",
                "Sheer, utter perfection~"
            ])?;
        }
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if runtime::op(&l_refinerycnt, "<", &l_safe)?.is_true() {
        ctx.lines_as(
            l_npc_name_s.clone(),
            args!["I can refine this to the safe limit or a desired number of times. It's your choice."],
        )?;
        ctx.next()?;
        l_menu2 = Val::from(runtime::select_values(
            ctx,
            &[
                Val::from("To the safe limit, please."),
                Val::from("I'll decide how many times."),
                Val::from("I've changed my mind..."),
            ],
        )?);
    } else {
        l_menu2 = Val::from(2);
    }
    if l_menu2 == 1 {
        l_refinecnt = (l_safe.clone().try_sub(l_refinerycnt.clone())?);
    } else if l_menu2 == 2 {
        ctx.next()?;
        ctx.lines_as(
            l_npc_name_s.clone(),
            args!["How many times would you like me to refine your item?"],
        )?;
        ctx.next()?;
        let (input, status) = runtime::input_number(ctx, None, None)?;
        l_refinecnt = input;
        l_refinecheck = (l_refinecnt.clone() + l_refinerycnt.clone());
        if (l_refinecnt.number()? < 1 || l_refinecheck.number()? > 10) {
            ctx.lines_as(l_npc_name_s.clone(), args!["I can't refine this item that many times."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if runtime::op(&l_refinecheck, ">", &l_safe)?.is_true() {
            l_refinecheck = (l_refinecheck.clone().try_sub(l_safe.clone())?);
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    ((Val::from("This will try to refine the equipment ") + l_refinecheck.clone())
                        + Val::from(" times past the safe limit. Your equipment may be destroyed... is that ok?"))
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Yes..."), Val::from("No...")])?) == 2 {
                ctx.lines_as(l_npc_name_s.clone(), args!["You said so... So be it."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else if l_menu2 == 3 {
        ctx.next()?;
        ctx.lines_as(l_npc_name_s.clone(), args!["You said so... So be it."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    l_fullprice = l_price.clone().try_mul(l_refinecnt.clone())?;
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            ((((((Val::from("That will cost you ") + l_refinecnt.clone()) + Val::from(" "))
                + ctx.call(Function::GetItemName, args![l_material.clone()])?)
                + Val::from(" and "))
                + l_fullprice.clone())
                + Val::from(" Zeny. Is that ok?"))
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Yes"), Val::from("No...")])?) == 2 {
        ctx.lines_as(l_npc_name_s.clone(), args!["You said so... So be it."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (runtime::op(&ctx.call(Function::CountItem, args![l_material.clone()])?, "<", &l_refinecnt)?.is_true()
        || runtime::op(&ctx.var("Zeny").get()?, "<", &l_fullprice)?.is_true())
    {
        ctx.lines_as(
            l_npc_name_s.clone(),
            args!["Is that all you got? Unfortunately I can't work for you at a lower price. Try putting yourself in my shoes."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(l_fullprice.clone())?))?;
    ctx.call(Function::DelItem, args![l_material.clone(), l_refinecnt.clone()])?;
    while l_refinecnt.is_true() {
        if ctx.call(Function::GetEquipIsEquipped, args![l_part.clone()])? == 0 {
            ctx.lines_as(l_npc_name_s.clone(), args!["Look here... you don't have any items on..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if (((crate::other_global_functions::f_isequipidhack(ctx, args![l_part.clone(), l_refineitemid.clone()])?.is_true()
            || crate::other_global_functions::f_isequipcardhack(
                ctx,
                vec![
                    l_part.clone(),
                    runtime::local_get(&l_card, &Val::from(0), false),
                    runtime::local_get(&l_card, &Val::from(1), false),
                    runtime::local_get(&l_card, &Val::from(2), false),
                    runtime::local_get(&l_card, &Val::from(3), false),
                ],
            )?
            .is_true())
            || crate::other_global_functions::f_isequiprefinehack(ctx, args![l_part.clone(), l_refinerycnt.clone()])?.is_true())
            || (l_menu2 == 1 && ctx.call(Function::GetEquipPercentRefinery, args![l_part.clone()])?.number()? < 100))
        {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Clang... No, but did you imagine I could be so stupid?!",
                    "You changed it...",
                    "Get out before I stun you with my Hammer!!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("Clang, clang!!!")?;
        if (l_menu2 == 2
            && runtime::op(
                &ctx.call(Function::GetEquipPercentRefinery, args![l_part.clone()])?,
                "<=",
                &ctx.call(Function::Rand, args![100])?,
            )?
            .is_true())
        {
            ctx.call(Function::FailedRefineItem, args![l_part.clone()])?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args!["WAHHHH!!! I'm so sorry... I warned you this could happen..."],
            )?;
            l_refinecnt = (l_refinecnt.clone().try_sub(Val::from(1))?);
            if l_refinecnt == 0 {
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.mes("Here's the unused Zeny and materials back...")?;
            ctx.call(Function::GetItem, args![l_material.clone(), l_refinecnt.clone()])?;
            l_fullprice = (l_refinecnt.clone().try_mul(l_price.clone())?);
            ctx.var("Zeny").set((ctx.var("Zeny").get()? + l_fullprice.clone()))?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.call(Function::SuccessRefineItem, args![l_part.clone()])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
        l_refinecnt = (l_refinecnt.clone().try_sub(Val::from(1))?);
        l_refinerycnt = ctx.call(Function::GetEquipRefineryCnt, args![l_part.clone()])?;
        ctx.next()?;
    }
    ctx.lines_as(l_npc_name_s.clone(), args!["All finished... Come again soon."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn repairmain(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_npc_name_s = runtime::arg(&args, 0, Val::from(0));
    let l_repairprice = Val::from(5000);
    ctx.lines_as(
        l_npc_name_s.clone(),
        args![
            "Hey there!",
            "Do you want me",
            "to repair any items?",
            "You can count on me",
            "for item repairs!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Actually, I do have some items...", "None at the moment."])? {
        0 => {
            let mut l_checkitem = Val::from(1);
            while ctx.call(Function::GetBrokenId, args![l_checkitem.clone()])? != 0 {
                l_checkitem = l_checkitem + Val::from(1);
            }
            l_checkitem = l_checkitem.try_sub(Val::from(1))?;
            if !(l_checkitem.is_true()) {
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args![
                        "Oh wow, this is incredible!",
                        "You must take very good care of your things. None of your items are damaged!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    l_npc_name_s.clone(),
                    args!["If everyone is like you, I'm going to be unemployed!! Haha~!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Hmm...",
                    "Let's see...",
                    "Out of all your items,",
                    ((Val::from("") + l_checkitem.clone()) + Val::from(" are damaged.")),
                    "Would you like to repair?"
                ],
            )?;
            ctx.next()?;
            let l_totalcost = l_repairprice.clone().try_mul(l_checkitem.clone())?;
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    ((((Val::from("Each repair costs ") + l_repairprice.clone())
                        + Val::from(" Zeny. So to repair all your damaged items would cost "))
                        + l_totalcost.clone())
                        + Val::from(" Zeny! Would you like to repair the items?"))
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    if ctx.player().zeny()? < l_totalcost.number()? {
                        ctx.lines_as(l_npc_name_s.clone(), args!["Whoa whoa...", "Check your wallet before you receive the repair bill! I can't repair anything because you don't have enough Zeny."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    let mut l_checkitem2 = Val::from(1);
                    while ctx.call(Function::GetBrokenId, args![l_checkitem2.clone()])? != 0 {
                        l_checkitem2 = l_checkitem2 + Val::from(1);
                    }
                    l_checkitem2 = l_checkitem2.try_sub(Val::from(1))?;
                    if l_checkitem.loosely_equals(&l_checkitem2) {
                        ctx.player().set_zeny(ctx.player().zeny()? - l_totalcost.number()?)?;
                        while l_checkitem.is_true() {
                            ctx.call(Function::Repair, args![l_checkitem.clone()])?;
                            l_checkitem = l_checkitem.try_sub(Val::from(1))?;
                        }
                        ctx.lines_as(
                            l_npc_name_s.clone(),
                            args!["Okay! All done. Now, try to be a little more careful. Items have lives too you know."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            l_npc_name_s.clone(),
                            args!["Mmm? Something's wrong. Wait... Equip the items you need to repair and then come back to me."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                1 => {
                    ctx.lines_as(l_npc_name_s.clone(), args!["Well, it's no skin off my nose, but it's not good to leave items damaged. You should get them repaired as soon as possible!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
        1 => {
            ctx.lines_as(
                l_npc_name_s.clone(),
                args![
                    "Hohoho...",
                    "You don't have",
                    "any business with me",
                    "if you don't have any",
                    "items to repair."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}
