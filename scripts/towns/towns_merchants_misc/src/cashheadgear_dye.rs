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

pub fn alora(ctx: &Ctx) -> Script {
    let mut l_c = Val::from(0);
    let mut l_colour_s = Val::from("");
    let mut l_dyhg = Val::from(0);
    let mut l_hg_1: Vec<Val> = Vec::new();
    let mut l_hg_2: Vec<Val> = Vec::new();
    let mut l_hg_3: Vec<Val> = Vec::new();
    let mut l_hg_4: Vec<Val> = Vec::new();
    let mut l_hg_5: Vec<Val> = Vec::new();
    let mut l_hg_6: Vec<Val> = Vec::new();
    let mut l_hg_7: Vec<Val> = Vec::new();
    let mut l_hgn_s: Vec<Val> = Vec::new();
    let mut l_i = Val::from(0);
    let mut l_menu_s = Val::from("");
    let mut l_mine = Val::from(0);
    let mut l_myhg = Val::from(0);
    let mut l_t = Val::from(0);
    let l_dyeid = Val::from(6220);
    runtime::local_set(&mut l_hgn_s, &Val::from(1), Val::from("Mage Hat"), true);
    runtime::local_set(&mut l_hgn_s, &Val::from(2), Val::from("Beanie"), true);
    runtime::local_set(&mut l_hgn_s, &Val::from(3), Val::from("Drooping Cat"), true);
    runtime::local_set(&mut l_hgn_s, &Val::from(4), Val::from("Deviruchi Hat"), true);
    runtime::local_set(&mut l_hgn_s, &Val::from(5), Val::from("Wig"), true);
    runtime::local_set(&mut l_hgn_s, &Val::from(6), Val::from("Ribbon"), true);
    runtime::local_set(&mut l_hgn_s, &Val::from(7), Val::from("Magestic Goat"), true);
    runtime::local_set(&mut l_hg_1, &Val::from(1), Val::from(5027), false);
    runtime::local_set(&mut l_hg_1, &Val::from(2), Val::from(5242), false);
    runtime::local_set(&mut l_hg_1, &Val::from(3), Val::from(5241), false);
    runtime::local_set(&mut l_hg_1, &Val::from(4), Val::from(5240), false);
    runtime::local_set(&mut l_hg_1, &Val::from(5), Val::from(5239), false);
    runtime::local_set(&mut l_hg_1, &Val::from(6), Val::from(5238), false);
    runtime::local_set(&mut l_hg_2, &Val::from(1), Val::from(5076), false);
    runtime::local_set(&mut l_hg_2, &Val::from(2), Val::from(5237), false);
    runtime::local_set(&mut l_hg_2, &Val::from(3), Val::from(5236), false);
    runtime::local_set(&mut l_hg_2, &Val::from(4), Val::from(5235), false);
    runtime::local_set(&mut l_hg_3, &Val::from(1), Val::from(5058), false);
    runtime::local_set(&mut l_hg_3, &Val::from(2), Val::from(5233), false);
    runtime::local_set(&mut l_hg_3, &Val::from(3), Val::from(5231), false);
    runtime::local_set(&mut l_hg_3, &Val::from(4), Val::from(5230), false);
    runtime::local_set(&mut l_hg_3, &Val::from(5), Val::from(5232), false);
    runtime::local_set(&mut l_hg_3, &Val::from(6), Val::from(5234), false);
    runtime::local_set(&mut l_hg_4, &Val::from(1), Val::from(5038), false);
    runtime::local_set(&mut l_hg_4, &Val::from(2), Val::from(5227), false);
    runtime::local_set(&mut l_hg_4, &Val::from(3), Val::from(5228), false);
    runtime::local_set(&mut l_hg_4, &Val::from(4), Val::from(5229), false);
    runtime::local_set(&mut l_hg_5, &Val::from(1), Val::from(5273), false);
    runtime::local_set(&mut l_hg_5, &Val::from(2), Val::from(5274), false);
    runtime::local_set(&mut l_hg_5, &Val::from(3), Val::from(5275), false);
    runtime::local_set(&mut l_hg_5, &Val::from(4), Val::from(5276), false);
    runtime::local_set(&mut l_hg_6, &Val::from(1), Val::from(2208), false);
    runtime::local_set(&mut l_hg_6, &Val::from(2), Val::from(5191), false);
    runtime::local_set(&mut l_hg_6, &Val::from(3), Val::from(5192), false);
    runtime::local_set(&mut l_hg_6, &Val::from(4), Val::from(5193), false);
    runtime::local_set(&mut l_hg_6, &Val::from(5), Val::from(5194), false);
    runtime::local_set(&mut l_hg_6, &Val::from(6), Val::from(5195), false);
    runtime::local_set(&mut l_hg_6, &Val::from(7), Val::from(5196), false);
    runtime::local_set(&mut l_hg_6, &Val::from(8), Val::from(5197), false);
    runtime::local_set(&mut l_hg_7, &Val::from(1), Val::from(2256), false);
    runtime::local_set(&mut l_hg_7, &Val::from(2), Val::from(5217), false);
    ctx.lines_as(
        "Alora",
        args!["Hello, I can change your headgear's color if you bring me a Mysterious Dyestuff."],
    )?;
    ctx.next()?;
    ctx.lines_as("Alora", args!["Do you have a headgear that you would like to dye?"])?;
    ctx.next()?;
    l_i = Val::from(1);
    while l_i.number()? < l_hgn_s.len() as i32 {
        if l_i == 1 {
            l_menu_s = runtime::local_get(&l_hgn_s, &l_i, true);
        } else {
            l_menu_s = l_menu_s + Val::from(":") + runtime::local_get(&l_hgn_s, &l_i, true);
        }
        l_i = Val::from(l_i.number()? + 1);
    }
    l_menu_s = l_menu_s + Val::from(":Cancel");
    l_t = Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?);
    if l_t == l_hgn_s.len() as i32 {
        ctx.lines_as("Alora", args!["Have a good journey adventurer!", "If you ever are curious to try a new color on your Kafra headgear or the ones you found on your adventures please come to me!"])?;
        return ctx.close();
    }
    ctx.lines_as("Alora", args!["Okay, what color do you want to change it to?"])?;
    ctx.next()?;
    l_i = Val::from(1);
    while l_i.number()?
        < runtime::getd_size(
            ctx,
            &(Val::from(".@hg_") + l_t.clone()),
            &[
                (".@c", runtime::Local::Scalar(&l_c)),
                (".@colour$", runtime::Local::Scalar(&l_colour_s)),
                (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
                (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
                (".@hg_1", runtime::Local::Array(&l_hg_1)),
                (".@hg_2", runtime::Local::Array(&l_hg_2)),
                (".@hg_3", runtime::Local::Array(&l_hg_3)),
                (".@hg_4", runtime::Local::Array(&l_hg_4)),
                (".@hg_5", runtime::Local::Array(&l_hg_5)),
                (".@hg_6", runtime::Local::Array(&l_hg_6)),
                (".@hg_7", runtime::Local::Array(&l_hg_7)),
                (".@hgn$", runtime::Local::Array(&l_hgn_s)),
                (".@i", runtime::Local::Scalar(&l_i)),
                (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                (".@mine", runtime::Local::Scalar(&l_mine)),
                (".@myhg", runtime::Local::Scalar(&l_myhg)),
                (".@t", runtime::Local::Scalar(&l_t)),
            ],
        )?
        .number()?
    {
        if l_i == 1 && l_t != 5 {
            l_menu_s = Val::from("Normal");
        } else if l_i == 1 && l_t == 5 {
            l_menu_s = ctx.call(
                Function::GetItemName,
                args![runtime::getd(
                    ctx,
                    &(Val::from(".@hg_") + l_t.clone() + Val::from("[") + l_i.clone() + Val::from("]")),
                    &[
                        (".@c", runtime::Local::Scalar(&l_c)),
                        (".@colour$", runtime::Local::Scalar(&l_colour_s)),
                        (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
                        (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
                        (".@hg_1", runtime::Local::Array(&l_hg_1)),
                        (".@hg_2", runtime::Local::Array(&l_hg_2)),
                        (".@hg_3", runtime::Local::Array(&l_hg_3)),
                        (".@hg_4", runtime::Local::Array(&l_hg_4)),
                        (".@hg_5", runtime::Local::Array(&l_hg_5)),
                        (".@hg_6", runtime::Local::Array(&l_hg_6)),
                        (".@hg_7", runtime::Local::Array(&l_hg_7)),
                        (".@hgn$", runtime::Local::Array(&l_hgn_s)),
                        (".@i", runtime::Local::Scalar(&l_i)),
                        (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                        (".@mine", runtime::Local::Scalar(&l_mine)),
                        (".@myhg", runtime::Local::Scalar(&l_myhg)),
                        (".@t", runtime::Local::Scalar(&l_t)),
                    ],
                )?],
            )?;
        } else {
            let scanned = runtime::sscanf(
                &ctx.call(
                    Function::GetItemName,
                    args![runtime::getd(
                        ctx,
                        &(Val::from(".@hg_") + l_t.clone() + Val::from("[") + l_i.clone() + Val::from("]")),
                        &[
                            (".@c", runtime::Local::Scalar(&l_c)),
                            (".@colour$", runtime::Local::Scalar(&l_colour_s)),
                            (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
                            (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
                            (".@hg_1", runtime::Local::Array(&l_hg_1)),
                            (".@hg_2", runtime::Local::Array(&l_hg_2)),
                            (".@hg_3", runtime::Local::Array(&l_hg_3)),
                            (".@hg_4", runtime::Local::Array(&l_hg_4)),
                            (".@hg_5", runtime::Local::Array(&l_hg_5)),
                            (".@hg_6", runtime::Local::Array(&l_hg_6)),
                            (".@hg_7", runtime::Local::Array(&l_hg_7)),
                            (".@hgn$", runtime::Local::Array(&l_hgn_s)),
                            (".@i", runtime::Local::Scalar(&l_i)),
                            (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                            (".@mine", runtime::Local::Scalar(&l_mine)),
                            (".@myhg", runtime::Local::Scalar(&l_myhg)),
                            (".@t", runtime::Local::Scalar(&l_t)),
                        ],
                    )?],
                )?,
                &(Val::from("%s ") + runtime::local_get(&l_hgn_s, &l_t, true)),
            );
            if let Some(value) = scanned.get(0).cloned() {
                l_colour_s = value;
            }
            l_menu_s = l_menu_s + Val::from(":") + l_colour_s.clone();
        }
        l_i = Val::from(l_i.number()? + 1);
    }
    l_menu_s = l_menu_s + Val::from(":Cancel");
    l_c = Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?);
    if l_c.loosely_equals(&runtime::getd_size(
        ctx,
        &(Val::from(".@hg_") + l_t.clone()),
        &[
            (".@c", runtime::Local::Scalar(&l_c)),
            (".@colour$", runtime::Local::Scalar(&l_colour_s)),
            (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
            (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
            (".@hg_1", runtime::Local::Array(&l_hg_1)),
            (".@hg_2", runtime::Local::Array(&l_hg_2)),
            (".@hg_3", runtime::Local::Array(&l_hg_3)),
            (".@hg_4", runtime::Local::Array(&l_hg_4)),
            (".@hg_5", runtime::Local::Array(&l_hg_5)),
            (".@hg_6", runtime::Local::Array(&l_hg_6)),
            (".@hg_7", runtime::Local::Array(&l_hg_7)),
            (".@hgn$", runtime::Local::Array(&l_hgn_s)),
            (".@i", runtime::Local::Scalar(&l_i)),
            (".@menu$", runtime::Local::Scalar(&l_menu_s)),
            (".@mine", runtime::Local::Scalar(&l_mine)),
            (".@myhg", runtime::Local::Scalar(&l_myhg)),
            (".@t", runtime::Local::Scalar(&l_t)),
        ],
    )?) {
        ctx.lines_as("Alora", args!["Oh, okay no problem!"])?;
        return ctx.close();
    }
    l_dyhg = runtime::getd(
        ctx,
        &(Val::from(".@hg_") + l_t.clone() + Val::from("[") + l_c.clone() + Val::from("]")),
        &[
            (".@c", runtime::Local::Scalar(&l_c)),
            (".@colour$", runtime::Local::Scalar(&l_colour_s)),
            (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
            (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
            (".@hg_1", runtime::Local::Array(&l_hg_1)),
            (".@hg_2", runtime::Local::Array(&l_hg_2)),
            (".@hg_3", runtime::Local::Array(&l_hg_3)),
            (".@hg_4", runtime::Local::Array(&l_hg_4)),
            (".@hg_5", runtime::Local::Array(&l_hg_5)),
            (".@hg_6", runtime::Local::Array(&l_hg_6)),
            (".@hg_7", runtime::Local::Array(&l_hg_7)),
            (".@hgn$", runtime::Local::Array(&l_hgn_s)),
            (".@i", runtime::Local::Scalar(&l_i)),
            (".@menu$", runtime::Local::Scalar(&l_menu_s)),
            (".@mine", runtime::Local::Scalar(&l_mine)),
            (".@myhg", runtime::Local::Scalar(&l_myhg)),
            (".@t", runtime::Local::Scalar(&l_t)),
        ],
    )?;
    ctx.lines_as(
        "Alora",
        args![
            "Oh I'm so excited aren't you?",
            Val::from("And just to be sure, what color is the ")
                + runtime::local_get(&l_hgn_s, &l_t, true)
                + Val::from(" you want me to use?")
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alora",
        args![
            Val::from("Please understand that I'm going to use the ^FF00001st ")
                + runtime::local_get(&l_hgn_s, &l_t, true)
                + Val::from(" of that color in your inventory!^000000")
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Alora",
        args![
            Val::from("Any upgrades and cards will be dissolved by the mysterious dye, so be sure you are ok with having a ^0000FF+0 ")
                + ctx.call(Function::GetItemName, args![l_dyhg.clone()])?
                + Val::from(" without any cards.^000000")
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, args![Val::from(0)])?,
        args!["Ok, thanks for the warning, I think I'll give you my"],
    )?;
    ctx.next()?;
    l_menu_s = Val::from("Nevermind");
    l_i = Val::from(1);
    while l_i.number()?
        < runtime::getd_size(
            ctx,
            &(Val::from(".@hg_") + l_t.clone()),
            &[
                (".@c", runtime::Local::Scalar(&l_c)),
                (".@colour$", runtime::Local::Scalar(&l_colour_s)),
                (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
                (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
                (".@hg_1", runtime::Local::Array(&l_hg_1)),
                (".@hg_2", runtime::Local::Array(&l_hg_2)),
                (".@hg_3", runtime::Local::Array(&l_hg_3)),
                (".@hg_4", runtime::Local::Array(&l_hg_4)),
                (".@hg_5", runtime::Local::Array(&l_hg_5)),
                (".@hg_6", runtime::Local::Array(&l_hg_6)),
                (".@hg_7", runtime::Local::Array(&l_hg_7)),
                (".@hgn$", runtime::Local::Array(&l_hgn_s)),
                (".@i", runtime::Local::Scalar(&l_i)),
                (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                (".@mine", runtime::Local::Scalar(&l_mine)),
                (".@myhg", runtime::Local::Scalar(&l_myhg)),
                (".@t", runtime::Local::Scalar(&l_t)),
            ],
        )?
        .number()?
    {
        if l_i == 1 {
            l_menu_s = l_menu_s.clone()
                + Val::from(":Normal ")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::getd(
                        ctx,
                        &(Val::from(".@hg_") + l_t.clone() + Val::from("[") + l_i.clone() + Val::from("]")),
                        &[
                            (".@c", runtime::Local::Scalar(&l_c)),
                            (".@colour$", runtime::Local::Scalar(&l_colour_s)),
                            (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
                            (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
                            (".@hg_1", runtime::Local::Array(&l_hg_1)),
                            (".@hg_2", runtime::Local::Array(&l_hg_2)),
                            (".@hg_3", runtime::Local::Array(&l_hg_3)),
                            (".@hg_4", runtime::Local::Array(&l_hg_4)),
                            (".@hg_5", runtime::Local::Array(&l_hg_5)),
                            (".@hg_6", runtime::Local::Array(&l_hg_6)),
                            (".@hg_7", runtime::Local::Array(&l_hg_7)),
                            (".@hgn$", runtime::Local::Array(&l_hgn_s)),
                            (".@i", runtime::Local::Scalar(&l_i)),
                            (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                            (".@mine", runtime::Local::Scalar(&l_mine)),
                            (".@myhg", runtime::Local::Scalar(&l_myhg)),
                            (".@t", runtime::Local::Scalar(&l_t)),
                        ],
                    )?],
                )?;
        } else {
            l_menu_s = l_menu_s.clone()
                + Val::from(":")
                + ctx.call(
                    Function::GetItemName,
                    args![runtime::getd(
                        ctx,
                        &(Val::from(".@hg_") + l_t.clone() + Val::from("[") + l_i.clone() + Val::from("]")),
                        &[
                            (".@c", runtime::Local::Scalar(&l_c)),
                            (".@colour$", runtime::Local::Scalar(&l_colour_s)),
                            (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
                            (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
                            (".@hg_1", runtime::Local::Array(&l_hg_1)),
                            (".@hg_2", runtime::Local::Array(&l_hg_2)),
                            (".@hg_3", runtime::Local::Array(&l_hg_3)),
                            (".@hg_4", runtime::Local::Array(&l_hg_4)),
                            (".@hg_5", runtime::Local::Array(&l_hg_5)),
                            (".@hg_6", runtime::Local::Array(&l_hg_6)),
                            (".@hg_7", runtime::Local::Array(&l_hg_7)),
                            (".@hgn$", runtime::Local::Array(&l_hgn_s)),
                            (".@i", runtime::Local::Scalar(&l_i)),
                            (".@menu$", runtime::Local::Scalar(&l_menu_s)),
                            (".@mine", runtime::Local::Scalar(&l_mine)),
                            (".@myhg", runtime::Local::Scalar(&l_myhg)),
                            (".@t", runtime::Local::Scalar(&l_t)),
                        ],
                    )?],
                )?;
        }
        l_i = Val::from(l_i.number()? + 1);
    }
    l_mine = Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])? - 1);
    if !l_mine.is_true() {
        ctx.lines_as("Alora", args!["It's best to be very sure, have a safe journey."])?;
        return ctx.close();
    }
    l_myhg = runtime::getd(
        ctx,
        &(Val::from(".@hg_") + l_t.clone() + Val::from("[") + l_mine.clone() + Val::from("]")),
        &[
            (".@c", runtime::Local::Scalar(&l_c)),
            (".@colour$", runtime::Local::Scalar(&l_colour_s)),
            (".@dyeid", runtime::Local::Scalar(&l_dyeid)),
            (".@dyhg", runtime::Local::Scalar(&l_dyhg)),
            (".@hg_1", runtime::Local::Array(&l_hg_1)),
            (".@hg_2", runtime::Local::Array(&l_hg_2)),
            (".@hg_3", runtime::Local::Array(&l_hg_3)),
            (".@hg_4", runtime::Local::Array(&l_hg_4)),
            (".@hg_5", runtime::Local::Array(&l_hg_5)),
            (".@hg_6", runtime::Local::Array(&l_hg_6)),
            (".@hg_7", runtime::Local::Array(&l_hg_7)),
            (".@hgn$", runtime::Local::Array(&l_hgn_s)),
            (".@i", runtime::Local::Scalar(&l_i)),
            (".@menu$", runtime::Local::Scalar(&l_menu_s)),
            (".@mine", runtime::Local::Scalar(&l_mine)),
            (".@myhg", runtime::Local::Scalar(&l_myhg)),
            (".@t", runtime::Local::Scalar(&l_t)),
        ],
    )?;
    if l_dyhg.loosely_equals(&l_myhg) {
        ctx.lines_as("Alora", args!["Woah what happened?"])?;
        return ctx.close();
    }
    if !ctx.call(Function::CountItem, args![l_dyeid.clone()])?.is_true() || !ctx.call(Function::CountItem, args![l_myhg.clone()])?.is_true()
    {
        ctx.lines_as("Alora", args!["Oh my, you seem to be missing something."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Alora",
        args!["Looks great doesn't it?!", "I hope you'll come back to dye more pretty headgears!"],
    )?;
    ctx.call(Function::DelItem, args![l_dyeid, 1])?;
    ctx.call(Function::DelItem, args![l_myhg, 1])?;
    ctx.call(Function::GetItem, args![l_dyhg, 1])?;
    ctx.close()
}
