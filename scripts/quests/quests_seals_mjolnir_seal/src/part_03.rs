use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn dwarf_blacksmith_west_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_card: Vec<Val> = Vec::new();
    let mut l_equip_id = Val::from(0);
    let mut l_equip_refine = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_indices: Vec<Val> = Vec::new();
    let mut l_itemtype = Val::from(0);
    let mut l_menu_s = Val::from("");
    let mut l_part = Val::from(0);
    if runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as(
            "Vestri",
            args!["If you want something, you should earn it through your own efforts."],
        )?;
        ctx.next()?;
        ctx.lines_as("Vestri", args!["No matter what other people say, if you're confident and your will is unwavering, you'll always be satisfied with the results."])?;
        ctx.next()?;
        ctx.lines_as("Vestri", args!["What do you think?", "Kids these days..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
            ctx.lines_as(
                "Vestri",
                args!["I don't feel", "like doing anything", "today. Anything at all..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vestri",
                args!["Someone must have forged something really monstrous, otherwise I wouldn't be feeling so worthless!"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Vestri",
                args!["Yeah, I think I need a break! Don't you think I need a break, human?"],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("god_mjo_0").get()? == 11 {
                ctx.lines_as(
                    "Vestri",
                    args!["There's nothing like taking a relaxing break after putting your heart into your work."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vestri",
                    args!["What do you think,", "human? Isn't that one", "of life's simple pleasures?"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_mjo_0").get()? == 10 {
                    ctx.lines_as(
                        "Vestri",
                        args!["If you want something, you should earn it through your own efforts."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Vestri", args!["No matter what other people say, if you're confident and your will is unwavering, you'll always be satisfied with the results."])?;
                    ctx.next()?;
                    ctx.lines_as("Vestri", args!["What do you think?", "Kids these days..."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("god_mjo_0").get()? == 1 {
                        if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3) || ctx.var("god_mjo_3").get()? == 3)
                            || ctx.var("god_mjo_4").get()? == 3)
                        {
                            ctx.lines_as(
                                "Vestri",
                                args!["I really hope I meet a decent human being next time. So far, I haven't met one useful human."],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("god_mjo_3").get()? == 2 {
                            ctx.lines_as("Vestri", args!["Perfect preparation does not always result in success. There's a point when you've got to just go out and do it."])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Vestri",
                                args![
                                    "I don't know how",
                                    "big your goals are,",
                                    "but put your heart into",
                                    "whatever it is that you",
                                    "plan to accomplish in life."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ((((ctx.var("god_mjo_1").get()? == 0 || ctx.var("god_mjo_1").get()? == 1)
                            || ctx.var("god_mjo_2").get()? == 0)
                            || ctx.var("god_mjo_2").get()? == 1)
                            || ctx.var("god_mjo_4").get()? != 0)
                        {
                            ctx.lines_as("Vestri", args!["What do you want?"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["If you want something, you should earn it through your own efforts."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Vestri", args!["No matter what other people say, if you're confident and your will is unwavering, you'll always be satisfied with the results."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.var("god_mjo_3").set(Val::from(3))?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["You didn't answer the question! Now, you've probably got the wrong Dwarf here..."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["Get out of here, and go to your human Blacksmiths if you want equipment upgrades!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else if ctx.var("god_mjo_3").get()? == 1 {
                            ctx.lines_as(
                                "Vestri",
                                args![
                                    "Great...!",
                                    "Which one should I upgrade first, huh? My heart is pounding with anticipation..."
                                ],
                            )?;
                            ctx.next()?;
                            let base = Val::from(1).number()?;
                            runtime::local_set(&mut l_indices, &Val::from(base + 0), ctx.constant("EQI_HEAD_TOP")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 1), ctx.constant("EQI_ARMOR")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 2), ctx.constant("EQI_HAND_L")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 3), ctx.constant("EQI_HAND_R")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 4), ctx.constant("EQI_GARMENT")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 5), ctx.constant("EQI_SHOES")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 6), ctx.constant("EQI_ACC_L")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 7), ctx.constant("EQI_ACC_R")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 8), ctx.constant("EQI_HEAD_MID")?, false);
                            runtime::local_set(&mut l_indices, &Val::from(base + 9), ctx.constant("EQI_HEAD_LOW")?, false);
                            l_i = Val::from(1);
                            'l2: loop {
                                if !(l_i.clone().number()? <= 10) {
                                    break 'l2;
                                }
                                'b2: {
                                    if ctx
                                        .call(
                                            Function::GetEquipIsEquipped,
                                            vec![runtime::local_get(&l_indices, &l_i.clone(), false)],
                                        )?
                                        .is_true()
                                    {
                                        l_menu_s = ((((l_menu_s.clone()
                                            + shared::other_global_functions::f_getpositionname(
                                                ctx,
                                                vec![runtime::local_get(&l_indices, &l_i.clone(), false)],
                                            )?)
                                            + Val::from("-["))
                                            + ctx.call(
                                                Function::GetEquipName,
                                                vec![runtime::local_get(&l_indices, &l_i.clone(), false)],
                                            )?)
                                            + Val::from("]"));
                                    }
                                    l_menu_s = (l_menu_s.clone() + Val::from(":"));
                                }
                                l_i = (l_i.clone() + Val::from(1));
                            }
                            l_part = runtime::local_get(&l_indices, &Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?), false);
                            if ctx.call(Function::GetEquipIsEquipped, vec![l_part.clone()])? == 0 {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ctx.call(Function::GetEquipIsEnableRefine, vec![l_part.clone()])? == 0 {
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "What...?!",
                                        "This isn't upgradable!",
                                        "What the hell do you want",
                                        "me to do with this?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            l_equip_id = ctx.call(Function::GetEquipId, vec![l_part.clone()])?;
                            l_itemtype = ctx.call(Function::GetItemInfo, vec![l_equip_id.clone(), ctx.constant("ITEMINFO_TYPE")?])?;
                            l_equip_refine = ctx.call(Function::GetEquipRefineryCnt, vec![l_part.clone()])?;
                            let base = Val::from(0).number()?;
                            runtime::local_set(
                                &mut l_card,
                                &Val::from(base + 0),
                                ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(0)])?,
                                false,
                            );
                            runtime::local_set(
                                &mut l_card,
                                &Val::from(base + 1),
                                ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(1)])?,
                                false,
                            );
                            runtime::local_set(
                                &mut l_card,
                                &Val::from(base + 2),
                                ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(2)])?,
                                false,
                            );
                            runtime::local_set(
                                &mut l_card,
                                &Val::from(base + 3),
                                ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(3)])?,
                                false,
                            );
                            if l_equip_refine.clone().number()? >= 10 {
                                ctx.lines_as("Vestri", args!["Oh, this is excellent! This piece here has been perfectly refined! But this isn't what I want. I can't do any work on this at all."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ctx.call(Function::GetEquipPercentRefinery, vec![l_part.clone()])? == 100 {
                                ctx.lines_as("Vestri", args!["This item isn't even a challenge to upgrade. You can get humans to do this kind of beginner's stuff. Get them to refine it first."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Vestri",
                                    args!["Come on...", "Bring me something", "that presents an", "element of risk!"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if !l_itemtype.clone().loosely_equals(&ctx.constant("IT_WEAPON")?) {
                                ctx.lines_as(
                                    "Vestri",
                                    args!["Armor?!", "Didn't I tell", "you that I only work", "on Level 4 weapons?"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Vestri", args!["You can have a human", "Blacksmith work on that kind of stuff! Now, a Dwarf like me needs something that's more of a challenge!"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ctx.call(Function::GetEquipWeaponLevel, vec![l_part.clone()])? != 4 {
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "Hey...",
                                        "Don't insult me by expecting me to work on anything less than a Level 4 weapon."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Vestri",
                                    args!["Bring me a Level 4 weapon for me to work on next time, got it?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as("Vestri", args!["Okay, let me give you the mandatory warning. If your weapon happens to be destroyed by chance during the upgrade, you'll never see the weapon again."])?;
                            ctx.next()?;
                            ctx.lines_as("Vestri", args!["That also means that if the weapon is destroyed, any ^FF0000Cards^000000 inserted into the weapon will also be gone."])?;
                            ctx.next()?;
                            ctx.lines_as("Vestri", args!["If you understand,", "then let's get on with it!"])?;
                            ctx.next()?;
                            if Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Sure, let's do it!:N-no, I changed my mind!")],
                            )?) == 2
                            {
                                ctx.lines_as(
                                    "Vestri",
                                    args!["Bah...!", "How do you survive", "in this world with that", "kind of cowardice?!"],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "Oh, forget it.",
                                        "I know you're just being careful. Damn, I was just so eager to get",
                                        "to work!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ctx.call(Function::CountItem, vec![Val::from(984)])?.number()? > 0 {
                                ctx.call(Function::DelItem, vec![Val::from(984), Val::from(1)])?;
                            } else {
                                ctx.lines_as(
                                    "Vestri",
                                    args!["Huh...", "You forgot to", "bring an Oridecon.", "Hurry up and get one."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if ((shared::other_global_functions::f_isequipidhack(ctx, vec![l_part.clone(), l_equip_id.clone()])?.is_true()
                                || shared::other_global_functions::f_isequipcardhack(
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
                                || shared::other_global_functions::f_isequiprefinehack(ctx, vec![l_part.clone(), l_equip_refine.clone()])?
                                    .is_true())
                            {
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if runtime::op(
                                &ctx.call(Function::GetEquipPercentRefinery, vec![l_part.clone()])?,
                                ">",
                                &ctx.call(Function::Rand, vec![Val::from(100)])?,
                            )?
                            .is_true()
                            {
                                ctx.mes("^3355FF*Clang Clang!*^000000")?;
                                ctx.call(Function::SuccessRefineItem, vec![l_part.clone()])?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "Mwahahaha~",
                                        "I've still got it!",
                                        "So aren't you happy",
                                        "with an even more",
                                        "powerful weapon?"
                                    ],
                                )?;
                                ctx.next()?;
                            } else {
                                ctx.lines_as("Vestri", args!["^3355FF*Clang Clang!*^000000"])?;
                                ctx.call(Function::FailedRefineItem, vec![l_part.clone()])?;
                                ctx.next()?;
                                ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                ctx.lines_as("Vestri", args!["Waaahhhhh!", "Dear God, no!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "I-It's alright!",
                                        "Bad things happen",
                                        "sometimes. Let's just",
                                        "think of it as both us",
                                        "of us having a bad day.",
                                        "Yeah, no regrets!"
                                    ],
                                )?;
                                ctx.next()?;
                            }
                            ctx.var("god_mjo_3").set(Val::from(2))?;
                            ctx.lines_as(
                                "Vestri",
                                args![
                                    "Well, my friend,",
                                    "if you ever visit my brothers, please give them my regards.",
                                    "Take care."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.var("god_mjo_3").get()? == 0 {
                            ctx.lines_as("Vestri", args!["What do you want?"])?;
                            ctx.next()?;
                            match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                1 => {
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["If you want something, you should earn it through your own efforts."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Vestri", args!["No matter what other people say, if you're confident and your will is unwavering, you'll always be satisfied with the results."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    ctx.lines_as("Vestri", args!["Huh...", "I don't know how I can be much help to an adventurer like you. Aside from being a Blacksmith..."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Vestri", args!["There's nothing I enjoy more than upgrading high level weapons that are on the verge of breaking."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Vestri", args!["Don't you like the idea of challenging the limit? To me, upgrading feels like climbing unconquered mountains!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Vestri", args!["Alright. Perhaps I'm meant to help you out, so I'll upgrade a weapon for you. All you need to do is bring me the weapon and material."])?;
                                    ctx.next()?;
                                    ctx.var("god_mjo_3").set(Val::from(1))?;
                                    ctx.lines_as("Vestri", args!["Here's the condition: You've got to bring me a Level 4 Weapon that's been upgraded to the point where it might break. Oh, and bring an Oridecon!"])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        } else {
                            ctx.lines_as("Vestri", args!["Zzzz Zzzz Zzzz..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    } else {
                        if ctx.var("god_mjo_0").get()? == 2 {
                            if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3)
                                || ctx.var("god_mjo_3").get()? == 3)
                                || ctx.var("god_mjo_4").get()? == 3)
                            {
                                ctx.lines_as(
                                    "Vestri",
                                    args!["I really hope I meet a decent human being next time. So far, I haven't met one useful human."],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("god_mjo_2").get()? == 2 {
                                ctx.lines_as("Vestri", args!["Perfect preparation does not always result in success. There's a point when you've got to just go out and do it."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "I don't know how",
                                        "big your goals are,",
                                        "but put your heart into",
                                        "whatever it is that you",
                                        "plan to accomplish in life."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if (((ctx.var("god_mjo_1").get()? == 0 || ctx.var("god_mjo_1").get()? == 1)
                                || ctx.var("god_mjo_3").get()? != 0)
                                || ctx.var("god_mjo_4").get()? != 0)
                            {
                                ctx.lines_as("Vestri", args!["What do you want?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Vestri",
                                            args!["If you want something, you should earn it through your own efforts."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Vestri", args!["No matter what other people say, if you're confident and your will is unwavering, you'll always be satisfied with the results."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.var("god_mjo_2").set(Val::from(3))?;
                                        ctx.lines_as(
                                            "Vestri",
                                            args!["You didn't answer the question! Now, you've probably got the wrong Dwarf here..."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Vestri",
                                            args!["Get out of here, and go to your human Blacksmiths if you want equipment upgrades!"],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else if ctx.var("god_mjo_2").get()? == 1 {
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "Great...!",
                                        "Which one should I upgrade first, huh? My heart is pounding with anticipation..."
                                    ],
                                )?;
                                ctx.next()?;
                                let base = Val::from(1).number()?;
                                runtime::local_set(&mut l_indices, &Val::from(base + 0), ctx.constant("EQI_HEAD_TOP")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 1), ctx.constant("EQI_ARMOR")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 2), ctx.constant("EQI_HAND_L")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 3), ctx.constant("EQI_HAND_R")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 4), ctx.constant("EQI_GARMENT")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 5), ctx.constant("EQI_SHOES")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 6), ctx.constant("EQI_ACC_L")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 7), ctx.constant("EQI_ACC_R")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 8), ctx.constant("EQI_HEAD_MID")?, false);
                                runtime::local_set(&mut l_indices, &Val::from(base + 9), ctx.constant("EQI_HEAD_LOW")?, false);
                                l_i = Val::from(1);
                                'l5: loop {
                                    if !(l_i.clone().number()? <= 10) {
                                        break 'l5;
                                    }
                                    'b5: {
                                        if ctx
                                            .call(
                                                Function::GetEquipIsEquipped,
                                                vec![runtime::local_get(&l_indices, &l_i.clone(), false)],
                                            )?
                                            .is_true()
                                        {
                                            l_menu_s = ((((l_menu_s.clone()
                                                + shared::other_global_functions::f_getpositionname(
                                                    ctx,
                                                    vec![runtime::local_get(&l_indices, &l_i.clone(), false)],
                                                )?)
                                                + Val::from("-["))
                                                + ctx.call(
                                                    Function::GetEquipName,
                                                    vec![runtime::local_get(&l_indices, &l_i.clone(), false)],
                                                )?)
                                                + Val::from("]"));
                                        }
                                        l_menu_s = (l_menu_s.clone() + Val::from(":"));
                                    }
                                    l_i = (l_i.clone() + Val::from(1));
                                }
                                l_part =
                                    runtime::local_get(&l_indices, &Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?), false);
                                if ctx.call(Function::GetEquipIsEquipped, vec![l_part.clone()])? == 0 {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.call(Function::GetEquipIsEnableRefine, vec![l_part.clone()])? == 0 {
                                    ctx.lines_as(
                                        "Vestri",
                                        args![
                                            "What...?!",
                                            "This isn't upgradable!",
                                            "What the hell do you want",
                                            "me to do with this?"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                l_equip_id = ctx.call(Function::GetEquipId, vec![l_part.clone()])?;
                                l_itemtype = ctx.call(Function::GetItemInfo, vec![l_equip_id.clone(), ctx.constant("ITEMINFO_TYPE")?])?;
                                l_equip_refine = ctx.call(Function::GetEquipRefineryCnt, vec![l_part.clone()])?;
                                let base = Val::from(0).number()?;
                                runtime::local_set(
                                    &mut l_card,
                                    &Val::from(base + 0),
                                    ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(0)])?,
                                    false,
                                );
                                runtime::local_set(
                                    &mut l_card,
                                    &Val::from(base + 1),
                                    ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(1)])?,
                                    false,
                                );
                                runtime::local_set(
                                    &mut l_card,
                                    &Val::from(base + 2),
                                    ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(2)])?,
                                    false,
                                );
                                runtime::local_set(
                                    &mut l_card,
                                    &Val::from(base + 3),
                                    ctx.call(Function::GetEquipCardId, vec![l_part.clone(), Val::from(3)])?,
                                    false,
                                );
                                if l_equip_refine.clone().number()? >= 10 {
                                    ctx.lines_as("Vestri", args!["Oh, this is excellent! This piece here has been perfectly refined! But this isn't what I want. I can't do any work on this at all."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.call(Function::GetEquipPercentRefinery, vec![l_part.clone()])? == 100 {
                                    ctx.lines_as("Vestri", args!["This item isn't even a challenge to upgrade. You can get humans to do this kind of beginner's stuff. Get them to refine it first."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["Come on...", "Bring me something", "that presents an", "element of risk!"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if !l_itemtype.clone().loosely_equals(&ctx.constant("IT_WEAPON")?) {
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["Armor?!", "Didn't I tell", "you that I only work", "on Level 4 weapons?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.call(Function::GetEquipWeaponLevel, vec![l_part.clone()])? != 4 {
                                    ctx.lines_as(
                                        "Vestri",
                                        args![
                                            "Hey...",
                                            "Don't insult me by expecting me to work on anything less than a Level 4 weapon."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["Bring me a Level 4 weapon for me to work on next time, got it?"],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as("Vestri", args!["Okay, let me give you the mandatory warning. If your weapon happens to be destroyed by chance during the upgrade, you'll never see the weapon again."])?;
                                ctx.next()?;
                                ctx.lines_as("Vestri", args!["That also means that if the weapon is destroyed, any ^FF0000Cards^000000 inserted into the weapon will also be gone."])?;
                                ctx.next()?;
                                ctx.lines_as("Vestri", args!["If you understand,", "then let's get on with it!"])?;
                                ctx.next()?;
                                if Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Sure, let's do it!:...no, I am out.")],
                                )?) == 2
                                {
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["Bah...!", "How do you survive", "in this world with that", "kind of cowardice?!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args![
                                            "Oh, forget it.",
                                            "I know you're just being careful. Damn, I was just so eager to get",
                                            "to work!"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ctx.call(Function::CountItem, vec![Val::from(984)])?.number()? > 0 {
                                    ctx.call(Function::DelItem, vec![Val::from(984), Val::from(1)])?;
                                } else {
                                    ctx.lines_as(
                                        "Vestri",
                                        args!["Huh...", "You forgot to", "bring an Oridecon.", "Hurry up and get one."],
                                    )?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if ((shared::other_global_functions::f_isequipidhack(ctx, vec![l_part.clone(), l_equip_id.clone()])?
                                    .is_true()
                                    || shared::other_global_functions::f_isequipcardhack(
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
                                    || shared::other_global_functions::f_isequiprefinehack(
                                        ctx,
                                        vec![l_part.clone(), l_equip_refine.clone()],
                                    )?
                                    .is_true())
                                {
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                }
                                if runtime::op(
                                    &ctx.call(Function::GetEquipPercentRefinery, vec![l_part.clone()])?,
                                    ">",
                                    &ctx.call(Function::Rand, vec![Val::from(100)])?,
                                )?
                                .is_true()
                                {
                                    ctx.mes("^3355FF*Clang Clang!*^000000")?;
                                    ctx.call(Function::SuccessRefineItem, vec![l_part.clone()])?;
                                    ctx.next()?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args![
                                            "Mwahahaha~",
                                            "I've still got it!",
                                            "So aren't you happy",
                                            "with an even more",
                                            "powerful weapon?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                } else {
                                    ctx.lines_as("Vestri", args!["^3355FF*Clang Clang!*^000000"])?;
                                    ctx.call(Function::FailedRefineItem, vec![l_part.clone()])?;
                                    ctx.next()?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_HUK")?])?;
                                    ctx.lines_as("Vestri", args!["Waaahhhhh!", "Dear God, no!"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Vestri",
                                        args![
                                            "I-It's alright!",
                                            "Bad things happen",
                                            "sometimes. Let's just",
                                            "think of it as both us",
                                            "of us having a bad day.",
                                            "Yeah, no regrets!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                                ctx.var("god_mjo_2").set(Val::from(2))?;
                                ctx.lines_as(
                                    "Vestri",
                                    args![
                                        "Well, my friend,",
                                        "if you ever visit my brothers, please give them my regards.",
                                        "Take care."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else if ctx.var("god_mjo_2").get()? == 0 {
                                ctx.lines_as("Vestri", args!["What do you want?"])?;
                                ctx.next()?;
                                match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                    1 => {
                                        ctx.lines_as(
                                            "Vestri",
                                            args!["If you want something, you should earn it through your own efforts."],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as("Vestri", args!["No matter what other people say, if you're confident and your will is unwavering, you'll always be satisfied with the results."])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    2 => {
                                        ctx.lines_as("Vestri", args!["Huh...", "I don't know how I can be much help to an adventurer like you. Aside from being a Blacksmith..."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Vestri", args!["There's nothing I enjoy more than upgrading high level weapons that are on the verge of breaking."])?;
                                        ctx.next()?;
                                        ctx.lines_as("Vestri", args!["Don't you like the idea of challenging the limit? To me, upgrading feels like climbing unconquered mountains!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Vestri", args!["Alright. Perhaps I'm meant to help you out, so I'll upgrade a weapon for you. All you need to do is bring me the weapon and material."])?;
                                        ctx.next()?;
                                        ctx.var("god_mjo_2").set(Val::from(1))?;
                                        ctx.lines_as("Vestri", args!["Here's the condition: You've got to bring me a Level 4 Weapon that's been upgraded to the point where it might break. Oh, and bring an Oridecon!"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    _ => {}
                                }
                            } else {
                                ctx.lines_as("Vestri", args!["Zzzz Zzzz Zzzz..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        } else {
                            if ctx.var("god_mjo_0").get()? == 0 {
                                ctx.lines_as(
                                    "Vestri",
                                    args!["It's always a pleasure to engage myself in hard work, especially smithing."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Vestri", args!["Upgrading is always enjoyable!", "I have no regrets when failure, and I'm always pleased when I'm successful. I'll upgrade everyday to make the best of my life~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Vestri", args!["Zzzz Zzzz Zzzz..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn dwarf_blacksmith_west(ctx: &Ctx) -> Script {
    dwarf_blacksmith_west_body(ctx, Vec::new()).map(|_| ())
}

fn dwarf_blacksmith_north_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_talk_not = Val::from(0);
    let mut l_talk_to = Val::from(0);
    if runtime::op(&ctx.var("$god3").get()?, "<", &ctx.var("$@god_check1").get()?)?.is_true() {
        ctx.lines_as("Nordri", args!["What...?", "I don't have any", "Eluniums or Oridecons!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Nordri",
            args!["Vestri took them all. ^333333*Sigh*^000000 Those were my treasures, you know..."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if runtime::op(&ctx.var("$god4").get()?, ">=", &ctx.var("$@god_check2").get()?)?.is_true() {
            ctx.lines_as(
                "Nordri",
                args![
                    "What's happening?",
                    "I sense change in the winds, but what that change may be, I cannot tell."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nordri",
                args!["Should I go into the cave to figure it out? Huh. This is most peculiar."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if ctx.var("god_mjo_0").get()? == 11 {
                ctx.lines_as("Nordri", args!["Bwahahah!", "Even the gods know that we Dwarves are the most talented of artisans! Maybe it doesn't seem that way now, but someday we shall return."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                if ctx.var("god_mjo_0").get()? == 10 {
                    ctx.lines_as("Nordri", args!["What...?", "I don't have any", "Eluniums or Oridecons!"])?;
                    ctx.next()?;
                    ctx.lines_as("Nordri", args!["Vestri took them all.", "Don't you understand?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    if ctx.var("god_mjo_0").get()? == 1 {
                        if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3) || ctx.var("god_mjo_3").get()? == 3)
                            || ctx.var("god_mjo_4").get()? == 3)
                        {
                            ctx.lines_as("Nordri", args!["Eh heh heh~", "It's nice being in the Mjolnir Forest. To be surrounded by the quiet, peaceful air and not having to worry about anything..."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            if ctx.var("god_mjo_4").get()? == 2 {
                                ctx.lines_as(
                                    "Nordri",
                                    args!["Shouldn't you be on your way by now? Eh, it's none of my business."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Nordri", args!["If you have the Zeny to spare, why don't you buy some snacks for your travels? Perhaps some delicious Bananas, or even some Pumpkins..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if (((((ctx.var("god_mjo_1").get()? == 0 || ctx.var("god_mjo_1").get()? == 1)
                                    || ctx.var("god_mjo_2").get()? == 0)
                                    || ctx.var("god_mjo_2").get()? == 1)
                                    || ctx.var("god_mjo_3").get()? == 0)
                                    || ctx.var("god_mjo_3").get()? == 1)
                                {
                                    ctx.lines_as("Nordri", args!["What business", "do you have with", "me, human?"])?;
                                    ctx.next()?;
                                    match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                        1 => {
                                            ctx.lines_as("Nordri", args!["Huh.", "If that's the case,", "then leave me alone."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        2 => {
                                            ctx.var("god_mjo_4").set(Val::from(3))?;
                                            ctx.lines_as(
                                                "Nordri",
                                                args!["Huh.", "That's fine. Still...", "I'm surprised to see", "such a polite human."],
                                            )?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                        _ => {}
                                    }
                                } else if ctx.var("god_mjo_4").get()? == 1 {
                                    if ctx.call(Function::CountItem, vec![Val::from(501)])?.number()? > 0 {
                                        ctx.call(Function::DelItem, vec![Val::from(501), Val::from(1)])?;
                                        ctx.lines_as(
                                            "Nordri",
                                            args![
                                                "Ah, you've brought",
                                                "me a Red Potion, just",
                                                "like I asked. In return,",
                                                "I will tell you an old story.",
                                                "I'm sure you'll like it."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        'l2: loop {
                                            if !(true) {
                                                break 'l2;
                                            }
                                            'b2: {
                                                if l_talk_to.clone() == 0 {
                                                    ctx.lines_as(
                                                        "Nordri",
                                                        args![
                                                            "This is a legend about one",
                                                            "of Thor's journeys into Utgard,",
                                                            "land of the giants."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Nordri",
                                                        args![
                                                            "There, he made a wager with their king in which he would challenge",
                                                            "the giants of their land in tests of skill and strength."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["The first was an eating contest. Thor ate all of the meat on his table, but his opponent, Utgardaloki, ate his meat, the bones and even the plates."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["In truth, Thor was tricked, and the opponent that defeated him was not", "actually a giant. Illusion was used to disguise Thor's opponent, but I forget was it was. Do you know?"])?;
                                                    ctx.next()?;
                                                    match runtime::select_values(ctx, &[Val::from("Greed:Sea:Blaze:Hog")])? {
                                                        3 => {}
                                                        _ => {
                                                            l_talk_not = Val::from(1);
                                                        }
                                                    }
                                                } else if l_talk_to.clone() == 1 {
                                                    ctx.lines_as("Nordri", args!["Yes, only a Blaze could have effortlessly consumed meat, bones and plates by burning. Of course!", "I remember now!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["The next contest was a race. In that match, Thor's servant, Tialfi competed with the king's servant, Hugi. However, no matter how many matches they had, Hugi would win every time."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["Once again, illusion magic was", "used to disguise the competitor. Do you know what Tialfi was really racing against?"])?;
                                                    ctx.next()?;
                                                    match runtime::select_values(ctx, &[Val::from("Language:Thoughts:Turtle:Wolf")])? {
                                                        2 => {}
                                                        _ => {
                                                            l_talk_not = Val::from(1);
                                                        }
                                                    }
                                                } else if l_talk_to.clone() == 2 {
                                                    ctx.lines_as("Nordri", args!["Yes, right! Tialfi was racing 'thoughts!' Nothing can move faster than the speed of thought, so it's no wonder Tialfi would always lose."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["For the final contest, Thor would challenge an old woman in a wrestling match. At first, Thor thought his victory was assured, but he learned that he could not defeat the old crone."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["Once again, Thor was the victim", "of illusion magic. He didn't realize he wasn't actually wrestling with an old woman. Do you know what his opponent really was?"])?;
                                                    ctx.next()?;
                                                    match runtime::select_values(ctx, &[Val::from("Curse:Earth:Heart:Old Age")])? {
                                                        4 => {}
                                                        _ => {
                                                            l_talk_not = Val::from(1);
                                                        }
                                                    }
                                                } else if l_talk_to.clone() == 3 {
                                                    ctx.lines_as("Nordri", args!["Yes, Thor was wrestling with 'Old Age!' No matter how strong anybody is, you can't fight against aging."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["After Thor lost every match,", "the king told him the truth about the contests. Thor grew furious, but it was too late. The king and the giants all vanished by then."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["There are two lessons", "to be learned from this tale. First, don't believe everything you see. Second, never be overconfident of your own power."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["There will always be someone or something more powerful than you. It is important to always do your best and have an attitude of humility."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["Try to live as good a life as you can. Remember that you can't have everything that you want. Obssession and irrationality go hand in hand."])?;
                                                    ctx.next()?;
                                                    ctx.var("god_mjo_4").set(Val::from(2))?;
                                                    ctx.lines_as("Nordri", args!["Thank you for listening to my long story. If you meet someone with a dangerous obsession, please tell this story of Thor and the illusions of the giant king."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                if l_talk_not.clone() == 1 {
                                                    ctx.lines_as("Nordri", args!["Huh...?", "I don't think that's right. Let me think, maybe I can remember it. Hopefully, it'll come to me sooner or later..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Nordri", args!["Shall we talk more of this once again after enjoying another Red Potion? Hahahahaha~"])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else {
                                                    l_talk_to = (l_talk_to.clone() + Val::from(1));
                                                }
                                            }
                                        }
                                    } else {
                                        ctx.lines_as("Nordri", args!["Oooh...", "I'm sooo thirsty!"])?;
                                        ctx.next()?;
                                        ctx.lines_as("Nordri", args!["Hm, didn't I ask you to bring a Red Potion? There's no way I can tell any stories with such a dry throat~"])?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                } else {
                                    if ctx.var("god_mjo_4").get()? == 0 {
                                        ctx.lines_as("Nordri", args!["What business", "do you have with", "me, human?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                            1 => {
                                                ctx.lines_as("Nordri", args!["Huh.", "If that's the case,", "then leave me alone."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.lines_as("Nordri", args!["Odd. Recently, too many humans have been interested in meeting with me and my brothers. Still, I cannot say their visits have been unpleasant."])?;
                                                ctx.next()?;
                                                ctx.var("god_mjo_4").set(Val::from(1))?;
                                                ctx.lines_as("Nordri", args!["I'm a little thirsty. Would you bring me a Red Potion. If you do that for me, I will tell you an important story. Heh heh heh~"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else {
                                        ctx.lines_as("Nordri", args!["Zzzz Zzzz..."])?;
                                        ctx.close_window()?;
                                    }
                                }
                            }
                        }
                    } else {
                        if ctx.var("god_mjo_0").get()? == 2 {
                            if (((ctx.var("god_mjo_1").get()? == 3 || ctx.var("god_mjo_2").get()? == 3)
                                || ctx.var("god_mjo_3").get()? == 3)
                                || ctx.var("god_mjo_4").get()? == 3)
                            {
                                ctx.lines_as("Nordri", args!["Eh heh heh~", "It's nice being in the Mjolnir Forest. To be surrounded by the quiet, peaceful air and not having to worry about anything..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                if ctx.var("god_mjo_1").get()? == 2 {
                                    ctx.lines_as(
                                        "Nordri",
                                        args!["Shouldn't you be on your way by now? Eh, it's none of my business."],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Nordri", args!["If you have the Zeny to spare, why don't you buy some snacks for your travels? Perhaps some delicious Bananas, or even some Pumpkins..."])?;
                                    ctx.close_window()?;
                                    return Err(Stop::End);
                                } else {
                                    if ((ctx.var("god_mjo_2").get()? != 0 || ctx.var("god_mjo_3").get()? != 0)
                                        || ctx.var("god_mjo_4").get()? != 0)
                                    {
                                        ctx.lines_as("Nordri", args!["What business", "do you have with", "me, human?"])?;
                                        ctx.next()?;
                                        match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                            1 => {
                                                ctx.lines_as("Nordri", args!["Huh.", "If that's the case,", "then leave me alone."])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            2 => {
                                                ctx.var("god_mjo_1").set(Val::from(3))?;
                                                ctx.lines_as(
                                                    "Nordri",
                                                    args!["Huh.", "That's fine. Still...", "I'm surprised to see", "such a polite human."],
                                                )?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            _ => {}
                                        }
                                    } else if ctx.var("god_mjo_1").get()? == 1 {
                                        if ctx.call(Function::CountItem, vec![Val::from(501)])?.number()? > 0 {
                                            ctx.call(Function::DelItem, vec![Val::from(501), Val::from(1)])?;
                                            ctx.lines_as(
                                                "Nordri",
                                                args![
                                                    "Ah, you've brought",
                                                    "me a Red Potion...?",
                                                    "In return, I will",
                                                    "tell you an old story.",
                                                    "I'm sure you'll like it."
                                                ],
                                            )?;
                                            ctx.next()?;
                                            'l8: loop {
                                                if !(true) {
                                                    break 'l8;
                                                }
                                                'b8: {
                                                    if l_talk_to.clone() == 0 {
                                                        ctx.lines_as("Nordri", args!["There is a story of a Dwarf named Alvis who contained more knowledge than a library and was braver than Siegfried the warrior."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Nordri",
                                                            args![
                                                                "Unfortunately...",
                                                                "He was too ambitious.",
                                                                "He fell in love with Thrud, Thor's first daughter, at first sight."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["He asked Thor for permission to marry Thrud, but as was expected, Thor refused. Alvis should have given up there, but he didn't."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["So Thor decided to test his knowledge by asking him some questions. The first question was, 'What is the name of the ground in human terms?' Do you know the answer?"])?;
                                                        ctx.next()?;
                                                        match runtime::select_values(ctx, &[Val::from("Ymir's body:Earth:Lane:Universe")])?
                                                        {
                                                            2 => {}
                                                            _ => {
                                                                l_talk_not = Val::from(1);
                                                            }
                                                        }
                                                    } else if l_talk_to.clone() == 1 {
                                                        ctx.lines_as("Nordri", args!["Yes, that's right, 'Earth.' Alvis was able to also answer Thor's question correctly."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["Thor then gave Alvis another question. What is the giant's term for the round shell that covers the earth?"])?;
                                                        ctx.next()?;
                                                        match runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Ymir's Head:Sky:Cloud Factory:High House")],
                                                        )? {
                                                            4 => {}
                                                            _ => {
                                                                l_talk_not = Val::from(1);
                                                            }
                                                        }
                                                    } else if l_talk_to.clone() == 2 {
                                                        ctx.lines_as("Nordri", args!["That's it, 'High House.' Since the giants are so huge, it might have looked that way to them."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["So Thor gave him another question. What is the term for the ball that rises in the sky at night, as used by the gods?"])?;
                                                        ctx.next()?;
                                                        match runtime::select_values(
                                                            ctx,
                                                            &[Val::from("Circling Wheel:Moon:False Sun:Fast Stranger")],
                                                        )? {
                                                            3 => {}
                                                            _ => {
                                                                l_talk_not = Val::from(1);
                                                            }
                                                        }
                                                    } else if l_talk_to.clone() == 3 {
                                                        ctx.lines_as("Nordri", args!["Yes! Gods refer to the", "moon as the 'false sun.'", "Although Alvis answered all of Thor's questions, he didn't notice the sun was rising."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as(
                                                            "Nordri",
                                                            args![
                                                                "Yes...",
                                                                "Thor prolonged his test so that Alvis would be turned to stone",
                                                                "once the sun rose."
                                                            ],
                                                        )?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["There are two", "things we can learn", "from this story. First, do not covet something to the point of challenging fate or the gods."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["Secondly, do not have too much pride in yourself. No matter how much ability or talent you may have, you cannot get everything", "you want."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["It's most", "important to live a", "good and virtuous life. It's great to achieve your desires, but be aware that some desires are not meant to be fulfilled."])?;
                                                        ctx.next()?;
                                                        ctx.var("god_mjo_1").set(Val::from(2))?;
                                                        ctx.lines_as("Nordri", args!["Thank you for listening to my long story. If you meet anyone afflicted with an insatiable desire, please tell him this story of Thor and Alvis, a brave yet very defiant Dwarf."])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    }
                                                    if l_talk_not.clone() == 1 {
                                                        ctx.lines_as("Nordri", args!["Huh...?", "I don't think that's right. Let me think, maybe I can remember it. Hopefully, it'll come to me sooner or later..."])?;
                                                        ctx.next()?;
                                                        ctx.lines_as("Nordri", args!["Shall we talk more of this once again after enjoying another Red Potion? Hahahahaha~"])?;
                                                        ctx.close_window()?;
                                                        return Err(Stop::End);
                                                    } else {
                                                        l_talk_to = (l_talk_to.clone() + Val::from(1));
                                                    }
                                                }
                                            }
                                        } else {
                                            ctx.lines_as("Nordri", args!["Oooh...", "I'm sooo thirsty!"])?;
                                            ctx.next()?;
                                            ctx.lines_as("Nordri", args!["Hm, didn't I ask you to bring a Red Potion? There's no way I can tell any stories with such a dry throat~"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    } else {
                                        if ctx.var("god_mjo_1").get()? == 0 {
                                            ctx.lines_as("Nordri", args!["What business", "do you have with", "me, human?"])?;
                                            ctx.next()?;
                                            match runtime::select_values(ctx, &[Val::from("Nothing.:Excuse me.")])? {
                                                1 => {
                                                    ctx.lines_as("Nordri", args!["You're funny, leave me alone."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                2 => {
                                                    ctx.lines_as("Nordri", args!["Odd. Recently, too many humans have been interested in meeting with me and my brothers. Still, I cannot say their visits have been unpleasant."])?;
                                                    ctx.next()?;
                                                    ctx.var("god_mjo_1").set(Val::from(1))?;
                                                    ctx.lines_as("Nordri", args!["I'm a little thirsty. Would you bring me a Red Potion. If you do that for me, I will tell you an important story. Heh heh heh~"])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                                _ => {}
                                            }
                                        } else {
                                            ctx.lines_as("Nordri", args!["Zzzz Zzzz Zzzz..."])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        }
                                    }
                                }
                            }
                        } else {
                            if ctx.var("god_mjo_0").get()? == 0 {
                                ctx.lines_as("Nordri", args!["I am Nordri,", "Dwarven Blacksmith."])?;
                                ctx.next()?;
                                ctx.lines_as("Nordri", args!["I am in charge of this Northern part of Mount Mjolnir. If you wish to pass, you must first secure my approval!"])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Nordri",
                                    args!["Ho ho ho...", "Calm down, it", "was only a joke.", "Hahahahaha!"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as("Nordri", args!["Zzzz Zzzz Zzzz..."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn dwarf_blacksmith_north(ctx: &Ctx) -> Script {
    dwarf_blacksmith_north_body(ctx, Vec::new()).map(|_| ())
}
