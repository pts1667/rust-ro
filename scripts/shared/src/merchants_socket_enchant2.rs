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

fn zeny_charge(zeny_req: &Val) -> Result<Val, Stop> {
    if zeny_req.clone().number()? >= 1000 {
        Ok(zeny_req.clone().try_div(Val::from(1000))? + Val::from(",000"))
    } else {
        Ok(zeny_req.clone())
    }
}

fn weapon_or_armor(ctx: &Ctx, item_id: &Val) -> Result<&'static str, Stop> {
    let locations = ctx.call(Function::GetItemInfo, args![item_id.clone(), constants::ITEMINFO_LOCATIONS])?;
    let is_weapon = runtime::op(&locations, "&", &Val::from(constants::EQP_HAND_R))?.is_true();
    Ok(if is_weapon { "weapon" } else { "armor" })
}

fn consume_materials(ctx: &Ctx, item_id: &Val, item_req: &[Val], req_amount: &[Val], zeny_req: &Val) -> Script {
    ctx.call(Function::DelItem, args![item_id.clone(), 1])?;
    ctx.call(
        Function::DelItem,
        args![
            runtime::local_get(item_req, &Val::from(0), false),
            runtime::local_get(req_amount, &Val::from(0), false)
        ],
    )?;
    if runtime::local_get(item_req, &Val::from(1), false) != 0 && runtime::local_get(req_amount, &Val::from(1), false) != 0 {
        ctx.call(
            Function::DelItem,
            args![
                runtime::local_get(item_req, &Val::from(1), false),
                runtime::local_get(req_amount, &Val::from(1), false)
            ],
        )?;
    }
    ctx.var("Zeny")
        .set(ctx.var("Zeny").get()?.try_sub(zeny_req.clone().try_mul(Val::from(1000))?)?)
}

pub fn func_socket2(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let item_id = runtime::arg(&args, 0, Val::from(0));
    let zeny_req = runtime::arg(&args, 4, Val::from(0));
    let mut item_req: Vec<Val> = Vec::new();
    let mut req_amount: Vec<Val> = Vec::new();
    runtime::local_set(&mut item_req, &Val::from(0), runtime::arg(&args, 5, Val::from(0)), false);
    runtime::local_set(&mut item_req, &Val::from(1), runtime::arg(&args, 7, Val::from(0)), false);
    runtime::local_set(&mut req_amount, &Val::from(0), runtime::arg(&args, 6, Val::from(0)), false);
    runtime::local_set(&mut req_amount, &Val::from(1), runtime::arg(&args, 8, Val::from(0)), false);
    ctx.mes("[Leablem]")?;
    if runtime::local_get(&item_req, &Val::from(1), false) != 0 && runtime::local_get(&req_amount, &Val::from(1), false) != 0 {
        ctx.lines(args![
            ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from("... Okay, then you need to bring me"),
            Val::from("^FF0000") + zeny_charge(&zeny_req)? + Val::from(",000 zeny as service charge,"),
            Val::from("^FF0000")
                + runtime::local_get(&req_amount, &Val::from(0), false)
                + Val::from("ea ")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::local_get(&item_req, &Val::from(0), false)]
                )?
                + Val::from(" and ")
                + runtime::local_get(&req_amount, &Val::from(1), false)
                + Val::from("ea ")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::local_get(&item_req, &Val::from(1), false)]
                )?
                + Val::from("."),
            Val::from("^FF0000Of course, you need a ") + ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from(".^000000"),
        ])?;
    } else {
        ctx.lines(args![
            ctx.call(Function::GetItemName, args![item_id.clone()])? + Val::from("? Okay, then you need to bring me"),
            Val::from("^FF0000")
                + zeny_charge(&zeny_req)?
                + Val::from(",000 zeny as service charge and ")
                + runtime::local_get(&req_amount, &Val::from(0), false)
                + Val::from(" ")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::local_get(&item_req, &Val::from(0), false)]
                )?
                + (if runtime::local_get(&req_amount, &Val::from(0), false).number()? > 1
                    && runtime::local_get(&item_req, &Val::from(0), false) != 999
                {
                    "s"
                } else {
                    ""
                })
                + Val::from(" as the requirement."),
            Val::from("^FF0000Of course, don't forget to bring me a ")
                + ctx.call(Function::GetItemName, args![item_id.clone()])?
                + Val::from("."),
            "^FF0000You should have all items.^000000"
        ])?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Leablem",
        args![
            "Did you already bring all of them?",
            "For your information, if you fail to create a slot,",
            Val::from("you will lose all the item requirement as well as the target ") + weapon_or_armor(ctx, &item_id)? + Val::from("."),
            Val::from("Also remember, if the ")
                + weapon_or_armor(ctx, &item_id)?
                + Val::from(" has been upgraded, and has been inserted with a card,"),
            "you will lose them even if you succeed in creating a slot."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Ask for slot creation.", "Try next time."])? {
        0 => {
            if runtime::op(&ctx.var("Zeny").get()?, ">=", &zeny_req.clone().try_mul(Val::from(1000))?)?.is_true()
                && runtime::op(
                    &ctx.call(Function::CountItem, args![runtime::local_get(&item_req, &Val::from(0), false)])?,
                    ">=",
                    &runtime::local_get(&req_amount, &Val::from(0), false),
                )?
                .is_true()
                && runtime::op(
                    &ctx.call(Function::CountItem, args![runtime::arg(&args, 7, Val::from(512))])?,
                    ">=",
                    &runtime::local_get(&req_amount, &Val::from(1), false),
                )?
                .is_true()
                && ctx.call(Function::CountItem, args![item_id.clone()])?.number()? > 0
            {
                ctx.lines_as(
                    "Leablem",
                    args!["Alright then, let the work begin!", "You'd better pray for a successful result."],
                )?;
                ctx.next()?;
                let roll = ctx.call(Function::Rand, args![1, 100])?;
                if runtime::op(&roll, ">", &runtime::arg(&args, 2, Val::from(0)))?.is_true()
                    && runtime::op(&roll, "<", &runtime::arg(&args, 3, Val::from(0)))?.is_true()
                {
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_LORD])?;
                    ctx.lines_as(
                        "Leablem",
                        args!["Great, it seems to be successful.", "It looks pretty well done. Congratulations!"],
                    )?;
                    consume_materials(ctx, &item_id, &item_req, &req_amount, &zeny_req)?;
                    ctx.call(Function::GetItem, args![runtime::arg(&args, 1, Val::from(0)), 1])?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["See you again, buddy!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.call(Function::NpcSpecialEffect, args![constants::EF_SUI_EXPLOSION])?;
                    ctx.lines_as(
                        "Leablem",
                        args![
                            "Wah! ...I am so sorry, it failed.",
                            "However, I am completely innocent.",
                            "This is your luck, and it is destined by god, okay?",
                            "Don't be so disappointed,",
                            "and try next time."
                        ],
                    )?;
                    consume_materials(ctx, &item_id, &item_req, &req_amount, &zeny_req)?;
                    ctx.next()?;
                    ctx.lines_as("Leablem", args!["See you again, buddy!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            } else {
                ctx.lines_as(
                    "Leablem",
                    args![
                        "Are you stupid or what? You didn't bring all of the required items!",
                        "Go bring them quick!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        1 => {
            ctx.lines_as("Leablem", args!["See you next time."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}
