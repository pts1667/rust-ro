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

pub fn func_socket(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_item_id = runtime::arg(&args, 0, Val::from(0));
    let l_zeny_req = runtime::arg(&args, 4, Val::from(0));
    let mut l_item_req: Vec<Val> = Vec::new();
    let mut l_req_amount: Vec<Val> = Vec::new();
    runtime::local_set(&mut l_item_req, &Val::from(0), runtime::arg(&args, 5, Val::from(0)), false);
    runtime::local_set(&mut l_item_req, &Val::from(1), runtime::arg(&args, 7, Val::from(0)), false);
    runtime::local_set(&mut l_req_amount, &Val::from(0), runtime::arg(&args, 6, Val::from(0)), false);
    runtime::local_set(&mut l_req_amount, &Val::from(1), runtime::arg(&args, 8, Val::from(0)), false);

    let consume_materials = || -> Result<(), Stop> {
        ctx.call(Function::DelItem, args![l_item_id.clone(), 1])?;
        ctx.call(
            Function::DelItem,
            args![
                runtime::local_get(&l_item_req, &Val::from(0), false),
                runtime::local_get(&l_req_amount, &Val::from(0), false),
            ],
        )?;
        if runtime::local_get(&l_item_req, &Val::from(1), false) != 0 && runtime::local_get(&l_req_amount, &Val::from(1), false) != 0 {
            ctx.call(
                Function::DelItem,
                args![
                    runtime::local_get(&l_item_req, &Val::from(1), false),
                    runtime::local_get(&l_req_amount, &Val::from(1), false),
                ],
            )?;
        }
        ctx.var("Zeny")
            .set(ctx.var("Zeny").get()?.try_sub(l_zeny_req.clone().try_mul(Val::from(1000))?)?)?;
        Ok(())
    };

    ctx.lines_as(
        "Seiyablem",
        args![
            ((Val::from("You want to add a Slot to a ") + ctx.call(Function::GetItemName, args![l_item_id.clone()])?) + Val::from("?")),
            (((((((((Val::from("Alright, please bring me ^FF0000") + runtime::local_get(&l_req_amount, &Val::from(0), false))
                + Val::from(" "))
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::local_get(&l_item_req, &Val::from(0), false)]
                )?)
                + (if (runtime::local_get(&l_req_amount, &Val::from(0), false).number()? > 1
                    && runtime::local_get(&l_item_req, &Val::from(0), false) != 999)
                {
                    Val::from("s")
                } else {
                    Val::from("")
                }))
                + Val::from("^000000, "))
                + (if (runtime::local_get(&l_item_req, &Val::from(1), false) != 0
                    && runtime::local_get(&l_req_amount, &Val::from(1), false) != 0)
                {
                    (((((Val::from("^FF0000") + runtime::local_get(&l_req_amount, &Val::from(1), false)) + Val::from(" "))
                        + ctx.call(
                            Function::GetItemName,
                            args![runtime::local_get(&l_item_req, &Val::from(1), false)],
                        )?)
                        + (if (runtime::local_get(&l_req_amount, &Val::from(1), false).number()? > 1
                            && runtime::local_get(&l_item_req, &Val::from(1), false) != 999)
                        {
                            Val::from("s")
                        } else {
                            Val::from("")
                        }))
                        + Val::from("^000000 "))
                } else {
                    Val::from("")
                }))
                + Val::from("and my ^FF0000"))
                + (if l_zeny_req.clone().number()? >= 1000 {
                    ((l_zeny_req.clone().try_div(Val::from(1000))?) + Val::from(",000"))
                } else {
                    l_zeny_req.clone()
                }))
                + Val::from(",000 zeny^000000 service fee.")),
            ((Val::from("Ah, and don't forget to bring that ") + ctx.call(Function::GetItemName, args![l_item_id.clone()])?)
                + Val::from("!"))
        ],
    )?;
    ctx.next()?;
    ctx.mes("[Seiyablem]")?;
    if runtime::op(
        &ctx.call(
            Function::GetItemInfo,
            args![l_item_id.clone(), ctx.constant("ITEMINFO_LOCATIONS")?],
        )?,
        "&",
        &ctx.constant("EQP_HAND_R")?,
    )?
    .is_true()
    {
        ctx.lines(args![
            "I can try to add a slot now if you have the required items and zeny.",
            "However, you should know that there's a chance that I might fail.",
            "Therefore, I need to give you a fair warning..."
        ])?;
        ctx.next()?;
        ctx.lines_as("Seiyablem", args!["If this attempt to add a Slot to your Weapon fails, then the ^FF0000Weapon^000000, ^FF0000and any Cards compounded to it will be destroyed^000000."])?;
    } else {
        ctx.lines(args!["If you have all the required materials, my zeny service fee and the Armor, then we can go ahead with the Slot Addition attempt.", "But before that, I must warn you of the risk."])?;
        ctx.next()?;
        ctx.lines_as("Seiyablem", args!["If this attempt to add a Slot to your Armor fails, then the ^FF0000Armor^000000, ^FF0000it's upgrades^000000 ^FF0000and any Cards compounded to it will be destroyed^000000."])?;
    }
    ctx.mes("Do you still want to try to add a Slot?")?;
    ctx.next()?;
    match ctx.menu(&["Attempt Slot Addition", "Cancel"])? {
        0 => {
            if runtime::op(&ctx.var("Zeny").get()?, ">=", &l_zeny_req.clone().try_mul(Val::from(1000))?)?.is_true()
                && runtime::op(
                    &ctx.call(
                        Function::CountItem,
                        args![runtime::local_get(&l_item_req, &Val::from(0), false)],
                    )?,
                    ">=",
                    &runtime::local_get(&l_req_amount, &Val::from(0), false),
                )?
                .is_true()
                && runtime::op(
                    &ctx.call(Function::CountItem, args![runtime::arg(&args, 7, Val::from(512))])?,
                    ">=",
                    &runtime::local_get(&l_req_amount, &Val::from(1), false),
                )?
                .is_true()
                && ctx.call(Function::CountItem, args![l_item_id.clone()])?.number()? > 0
            {
                ctx.lines_as(
                    "Seiyablem",
                    args!["Alright then, let the work begin!", "You'd better pray for a successful result."],
                )?;
                ctx.next()?;
                let l_a = ctx.call(Function::Rand, args![1, 100])?;
                if runtime::op(&l_a, ">", &runtime::arg(&args, 2, Val::from(0)))?.is_true()
                    && runtime::op(&l_a, "<", &runtime::arg(&args, 3, Val::from(0)))?.is_true()
                {
                    ctx.call(
                        Function::NpcSpecialEffect,
                        args![if runtime::arg(&args, 3, Val::from(0)) == 51 {
                            ctx.constant("EF_LORD")?
                        } else {
                            ctx.constant("EF_SANCTUARY")?
                        }],
                    )?;
                    ctx.lines_as(
                        "Seiyablem",
                        args!["Great, it seems to be successful.", "It looks pretty well done. Congratulations!"],
                    )?;
                    consume_materials()?;
                    ctx.call(Function::GetItem, args![runtime::arg(&args, 1, Val::from(0)), 1])?;
                    ctx.next()?;
                    ctx.lines_as("Seiyablem", args!["See you again, buddy!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.call(Function::NpcSpecialEffect, args![ctx.constant("EF_SUI_EXPLOSION")?])?;
                    ctx.lines_as(
                        "Seiyablem",
                        args![
                            "Wah! ...I am so sorry, it failed.",
                            "However, I am completely innocent.",
                            "This is your luck, and it is destined by god, okay?",
                            "Don't be so disappointed, and try next time."
                        ],
                    )?;
                    consume_materials()?;
                    ctx.next()?;
                    ctx.lines_as("Seiyablem", args!["I wish you good luck next time!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as(
                    "Seiyablem",
                    args![
                        "I'd like to go ahead with this Slot Addition attempt, but you're missing a few things.",
                        "You sure that you have the equipment, required materials and the zeny?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        1 => {
            ctx.lines_as(
                "Seiyablem",
                args![
                    "Need some time to think about it, huh?",
                    "Alright, I can understand.",
                    "Just remember that life's no fun if you're always playing it safe~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}
