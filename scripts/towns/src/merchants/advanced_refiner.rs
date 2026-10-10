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
enum SuhnbiCashStep {
    Start,
    SRefineValidate,
}

fn suhnbi_cash_run(ctx: &Ctx, step: SuhnbiCashStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            SuhnbiCashStep::Start => {
                ctx.lines_as(
                    "Suhnbi",
                    args![
                        "I am the Armsmith",
                        "I can refine all kinds of weapons,",
                        "armor and equipment, so let me",
                        "know what you want to refine."
                    ],
                )?;
                ctx.next()?;
                let mut l_indices: Vec<Val> = Vec::new();
                runtime::local_set(&mut l_indices, &Val::from(1), ctx.constant("EQI_HEAD_TOP")?, false);
                runtime::local_set(&mut l_indices, &Val::from(2), ctx.constant("EQI_ARMOR")?, false);
                runtime::local_set(&mut l_indices, &Val::from(3), ctx.constant("EQI_HAND_L")?, false);
                runtime::local_set(&mut l_indices, &Val::from(4), ctx.constant("EQI_HAND_R")?, false);
                runtime::local_set(&mut l_indices, &Val::from(5), ctx.constant("EQI_GARMENT")?, false);
                runtime::local_set(&mut l_indices, &Val::from(6), ctx.constant("EQI_SHOES")?, false);
                runtime::local_set(&mut l_indices, &Val::from(7), ctx.constant("EQI_ACC_L")?, false);
                runtime::local_set(&mut l_indices, &Val::from(8), ctx.constant("EQI_ACC_R")?, false);
                runtime::local_set(&mut l_indices, &Val::from(9), ctx.constant("EQI_HEAD_MID")?, false);
                runtime::local_set(&mut l_indices, &Val::from(10), ctx.constant("EQI_HEAD_LOW")?, false);
                let mut l_equipped = Val::from(0);
                let mut l_menu_s = Val::from("");
                for i in 1..=10 {
                    let index = Val::from(i);
                    if ctx
                        .call(
                            Function::GetEquipIsEquipped,
                            args![runtime::local_get(&l_indices, &index, false)],
                        )?
                        .is_true()
                    {
                        l_menu_s = l_menu_s
                            + shared::other_global_functions::f_getpositionname(ctx, args![runtime::local_get(&l_indices, &index, false)])?
                            + Val::from("-[")
                            + ctx.call(Function::GetEquipName, args![runtime::local_get(&l_indices, &index, false)])?
                            + Val::from("]");
                        l_equipped = Val::from(1);
                    }
                    l_menu_s = l_menu_s + Val::from(":");
                }
                if l_equipped == 0 {
                    ctx.lines_as("Suhnbi", args!["I don't think I can refine any items you have..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let l_part = runtime::local_get(&l_indices, &Val::from(runtime::select_values(ctx, &[l_menu_s])?), false);
                if !ctx.call(Function::GetEquipIsEquipped, args![l_part.clone()])?.is_true() {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !ctx.call(Function::GetEquipIsEnableRefine, args![l_part.clone()])?.is_true() {
                    ctx.lines_as("Suhnbi", args!["Go find another Blacksmith. You can't refine this thing."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.call(Function::GetEquipRefineryCnt, args![l_part.clone()])?.number()? >= 10 {
                    ctx.lines_as(
                        "Suhnbi",
                        args!["Hmm... someone perfected this already. I don't think I can work on it further."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                let l_refineitemid = ctx.call(Function::GetEquipId, args![l_part.clone()])?;
                let l_refinerycnt = ctx.call(Function::GetEquipRefineryCnt, args![l_part.clone()])?;
                let l_price = ctx.call(
                    Function::GetEquipRefineCost,
                    args![l_part.clone(), constants::REFINE_COST_ENRICHED, constants::REFINE_ZENY_COST],
                )?;
                let l_material = ctx.call(
                    Function::GetEquipRefineCost,
                    args![l_part.clone(), constants::REFINE_COST_ENRICHED, constants::REFINE_MATERIAL_ID],
                )?;
                let l_itemtype = ctx.call(Function::GetItemInfo, args![l_refineitemid.clone(), constants::ITEMINFO_TYPE])?;
                suhnbi_cash_run(
                    ctx,
                    SuhnbiCashStep::SRefineValidate,
                    args![
                        l_itemtype.clone(),
                        l_material,
                        l_price,
                        l_part.clone(),
                        l_refineitemid,
                        l_refinerycnt
                    ],
                )?;
                ctx.lines_as("Suhnbi", args!["Clang! Clang! Clang!"])?;
                if runtime::op(
                    &ctx.call(Function::GetEquipPercentRefinery, args![l_part.clone(), 1])?,
                    ">",
                    &ctx.call(Function::Rand, args![100])?,
                )?
                .is_true()
                {
                    ctx.call(Function::SuccessRefineItem, args![l_part.clone()])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, args![constants::ET_BEST])?;
                    ctx.lines_as(
                        "Suhnbi",
                        args![
                            "There you go! It's done.",
                            Val::from("It's been a while since I've made such a fine ")
                                + (if l_itemtype == constants::IT_WEAPON {
                                    Val::from("weapon")
                                } else {
                                    Val::from("armor")
                                })
                                + Val::from(". You must be happy because it has become stronger!")
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.call(Function::FailedRefineItem, args![l_part.clone()])?;
                ctx.next()?;
                let emotion = if ctx.rand(5)? == 0 {
                    constants::ET_MONEY
                } else {
                    constants::ET_HUK
                };
                ctx.call(Function::Emotion, args![emotion])?;
                ctx.lines_as("Suhnbi", args!["Uuuuuuuuuummmmmph!!!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Suhnbi",
                    args![
                        "...",
                        ".....",
                        ".......Huhuhuhuhu~",
                        "........It was your choice and my ability, no regret."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            SuhnbiCashStep::SRefineValidate => {
                let l_itemtype = runtime::arg(&args, 0, Val::from(0));
                let l_item_req = runtime::arg(&args, 1, Val::from(0));
                let mut l_price = runtime::arg(&args, 2, Val::from(0));
                let l_part = runtime::arg(&args, 3, Val::from(0));
                let mut l_card: Vec<Val> = Vec::new();
                runtime::local_set(
                    &mut l_card,
                    &Val::from(0),
                    ctx.call(Function::GetEquipCardId, args![l_part.clone(), 0])?,
                    false,
                );
                runtime::local_set(
                    &mut l_card,
                    &Val::from(1),
                    ctx.call(Function::GetEquipCardId, args![l_part.clone(), 1])?,
                    false,
                );
                runtime::local_set(
                    &mut l_card,
                    &Val::from(2),
                    ctx.call(Function::GetEquipCardId, args![l_part.clone(), 2])?,
                    false,
                );
                runtime::local_set(
                    &mut l_card,
                    &Val::from(3),
                    ctx.call(Function::GetEquipCardId, args![l_part.clone(), 3])?,
                    false,
                );
                let l_equip_lv;
                if l_itemtype == constants::IT_ARMOR {
                    l_equip_lv = ctx.call(Function::GetEquipArmorLevel, args![l_part.clone()])?;
                    if constants::VIP_SCRIPT != 0 && !ctx.call(Function::VipStatus, args![constants::VIP_STATUS_ACTIVE])?.is_true() {
                        if l_equip_lv == 1 {
                            l_price = l_price.try_mul(Val::from(10))?;
                        } else {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else if l_itemtype == constants::IT_WEAPON {
                    l_equip_lv = ctx.call(Function::GetEquipWeaponLevel, args![l_part.clone()])?;
                    if constants::VIP_SCRIPT != 0 && !ctx.call(Function::VipStatus, args![constants::VIP_STATUS_ACTIVE])?.is_true() {
                        if l_equip_lv == 1 {
                            l_price = l_price.try_mul(Val::from(40))?;
                        } else if l_equip_lv == 2 {
                            l_price = l_price.try_mul(Val::from(50))?;
                        } else if l_equip_lv == 3 {
                            l_price = l_price.try_mul(Val::from(2))?;
                        } else if l_equip_lv == 4 {
                            l_price = l_price.try_mul(Val::from(2))?;
                        } else {
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                } else {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.mes("[Suhnbi]")?;
                if l_itemtype == constants::IT_WEAPON {
                    ctx.lines(args![
                        Val::from("You want to refine a level ") + l_equip_lv + Val::from(" weapon?")
                    ])?;
                }
                ctx.lines(args![
                    Val::from("To refine that, you'll need to have one ^ff9999")
                        + ctx.call(Function::GetItemName, args![l_item_req.clone()])?
                        + Val::from("^000000 and ")
                        + l_price.clone()
                        + Val::from(" zeny."),
                    "Would you like to continue?"
                ])?;
                ctx.next()?;
                if runtime::select_values(ctx, &[Val::from("Yes:No")])? == 1 {
                    if ctx.call(Function::GetEquipPercentRefinery, args![l_part.clone()])?.number()? < 100 {
                        if l_itemtype == constants::IT_WEAPON {
                            ctx.lines_as(
                                "Suhnbi",
                                args![
                                    "Wow!!",
                                    "This weapon probably",
                                    "looks like it's been refined...",
                                    "many times...",
                                    "It may break if",
                                    "you refine it again."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "And if it breaks,",
                                "you can't use it anymore!",
                                "All the cards in it and the properties ^ff0000will be lost^000000!",
                                "^ff0000Besides, the equipment will break!^000000",
                                "Are you sure you still want to continue?"
                            ])?;
                            ctx.next()?;
                            if runtime::select_values(ctx, &[Val::from("Yes:No")])? == 2 {
                                ctx.lines_as(
                                    "Suhnbi",
                                    args![
                                        "Good.",
                                        "Because if the weapon breaks from unreasonable refining, then I get a bad mood, too."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            ctx.lines_as(
                                "Suhnbi",
                                args![
                                    "Giggle. Giggle. Oh, you have guts, daring to refine this.",
                                    "You know it's pretty risky, don't you?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines(args![
                                "If your defensive equipment is broken, you'll never be able to use it again.",
                                "Even your cards and your modifications will ^ff0000completely disappear^000000.",
                                "Do you really wish to continue?"
                            ])?;
                            ctx.next()?;
                            if runtime::select_values(ctx, &[Val::from("Yes:No")])? == 2 {
                                ctx.lines_as("Suhnbi", args!["What nonsense. You waste my precious time.", "Get lost, punk."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if ctx.call(Function::CountItem, args![l_item_req.clone()])?.number()? > 0
                        && runtime::op(&ctx.var("Zeny").get()?, ">", &l_price)?.is_true()
                    {
                        ctx.call(Function::DelItem, args![l_item_req.clone(), 1])?;
                        ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_price)?)?;
                        if shared::other_global_functions::f_isequipidhack(
                            ctx,
                            args![l_part.clone(), runtime::arg(&args, 4, Val::from(0))],
                        )?
                        .is_true()
                            || shared::other_global_functions::f_isequiprefinehack(
                                ctx,
                                args![l_part.clone(), runtime::arg(&args, 5, Val::from(0))],
                            )?
                            .is_true()
                            || shared::other_global_functions::f_isequipcardhack(
                                ctx,
                                args![
                                    l_part.clone(),
                                    runtime::local_get(&l_card, &Val::from(0), false),
                                    runtime::local_get(&l_card, &Val::from(1), false),
                                    runtime::local_get(&l_card, &Val::from(2), false),
                                    runtime::local_get(&l_card, &Val::from(3), false)
                                ],
                            )?
                            .is_true()
                        {
                            ctx.mes("[Suhnbi]")?;
                            ctx.call(Function::Emotion, args![constants::ET_FRET])?;
                            ctx.lines(args![
                                "Wait a second...",
                                "Do you think I'm stupid?!",
                                "You switched the item while I wasn't looking! Get out of here!"
                            ])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        return Ok(Val::from(0));
                    }
                    ctx.lines_as("Suhnbi", args!["Are these all you have?", "I'm very sorry, but I can't do anything without all the materials. Besides, I deserve some payments for my work, don't I?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Suhnbi", args!["I can't help it even if you're not happy about it..."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn suhnbi_cash(ctx: &Ctx) -> Script {
    suhnbi_cash_run(ctx, SuhnbiCashStep::Start, Vec::new()).map(|_| ())
}
