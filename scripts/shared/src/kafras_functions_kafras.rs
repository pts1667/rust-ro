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

pub fn f_checkkafcode(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !ctx.var("#kafra_code").get()?.is_true() {
        return Ok(Val::from(0));
    }
    ctx.mes("Enter your storage password:")?;
    let (code, _) = runtime::input_number(ctx, None, None)?;
    if !code.loosely_equals(&ctx.var("#kafra_code").get()?) {
        ctx.call(Function::DispBottom, args!["Wrong storage password."])?;
        ctx.close_window()?;
        ctx.fx().cutin("", 255)?;
        return Err(Stop::End);
    }
    ctx.var("@kafcode_try").set(Val::from(0))?;
    Ok(Val::from(0))
}

pub fn f_entkafcode(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.mes("Enter a number 1000~10000000:")?;
    ctx.var("@kafcode_try").set(ctx.var("@kafcode_try").get()? + Val::from(1))?;
    if ctx.var("@kafcode_try").get()?.number()? > 10 {
        ctx.var("@kafcode_try").set(Val::from(0))?;
        return Ok(Val::from(0));
    }
    let (code, status) = runtime::input_number(ctx, None, None)?;
    if status == 1 {
        ctx.mes("You can't use such big password.")?;
        return Ok(Val::from(0));
    }
    if code.number()? < 1000 {
        ctx.mes("You shouldn't use such short password.")?;
        return Ok(Val::from(0));
    }
    Ok(code)
}

pub fn f_kafcart(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if !ctx.var("BaseClass").get()?.loosely_equals(&ctx.constant("JOB_MERCHANT")?) {
        ctx.lines_as("Kafra Employee", args!["I'm sorry, but the", "Pushcart rental service"])?;
        ctx.lines(args![
            "is only available to Merchants,",
            "Blacksmiths, Master Smiths,",
            "Alchemists and Biochemists."
        ])?;
        return Ok(Val::from(1));
    }
    if ctx.call(Function::CheckCart, vec![])? == 1 {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "You already have",
                "a Pushcart equipped.",
                "Unfortunately, we can't",
                "rent more than one to",
                "each customer at a time."
            ],
        )?;
        return Ok(Val::from(1));
    }
    if ctx.call(Function::GetSkillLv, args!["MC_PUSHCART"])? == 0 {
        ctx.lines_as(
            "Kafra Employee",
            args!["You can only rent a cart after learning the \"Push Cart\" skill."],
        )?;
        return Ok(Val::from(1));
    }
    if ctx.items().count(7061)? > 0 && runtime::arg(&args, 0, Val::from(0)) != 2 {
        ctx.items().take(7061, 1)?;
    } else {
        let fee = runtime::arg(&args, 1, Val::from(0));
        ctx.lines_as(
            "Kafra Employee",
            args![
                "The Pushcart rental",
                Val::from("fee is ") + fee.clone() + Val::from(" zeny. Would"),
                "you like to rent a Pushcart?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Rent a Pushcart.", "Cancel"])? == 1 {
            return Ok(Val::from(0));
        }
        if runtime::op(&ctx.var("Zeny").get()?, "<", &fee)?.is_true() {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "I'm sorry, but you",
                    "don't have enough",
                    "zeny to pay the Pushcart",
                    Val::from("rental fee of ") + fee.clone() + Val::from(" zeny.")
                ],
            )?;
            return Ok(Val::from(1));
        }
        ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(fee)?)?;
        ctx.var("resrvpts").set(ctx.var("resrvpts").get()? + Val::from(48))?;
    }
    ctx.call(Function::SetCart, vec![])?;
    Ok(Val::from(1))
}

pub fn f_kafend(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_type = runtime::arg(&args, 0, Val::from(0));
    ctx.mes("[Kafra Employee]")?;
    if runtime::arg(&args, 1, Val::from(0)) == 1 {
        ctx.lines(args![
            "Your Respawn Point",
            "has been saved here",
            Val::from("") + runtime::arg(&args, 2, Val::from(0)) + Val::from("."),
            "Thank you for using",
            "the Kafra Services."
        ])?;
    } else if l_type == 0 || l_type == 5 {
        ctx.lines(args!["We, here at Kafra Corporation,", "are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence."])?;
    } else if l_type == 1 {
        ctx.call(Function::PercentHeal, args![0, -25])?;
        ctx.lines(args![
            "^666666Kaffffra n-never",
            "diiiiiiiiiiiiiies. On...",
            "On y-yooour siiiiide~^000000"
        ])?;
    } else if l_type == 2 {
        ctx.lines(args!["Saved.", "Thank you for your patronage."])?;
    }
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    Err(Stop::End)
}

pub fn f_kafinfo(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_menu_s: Vec<Val> = Vec::new();
    if runtime::arg(&args, 0, Val::from(0)) == 2 {
        runtime::local_set(&mut l_menu_s, &Val::from(0), Val::from("Check Special Reserve Points."), true);
        runtime::local_set(&mut l_menu_s, &Val::from(1), Val::from(""), true);
        runtime::local_set(&mut l_menu_s, &Val::from(2), Val::from(""), true);
        runtime::local_set(&mut l_menu_s, &Val::from(3), Val::from("Cancel"), true);
    } else {
        runtime::local_set(&mut l_menu_s, &Val::from(0), Val::from("Check Special Reserve Points."), true);
        runtime::local_set(&mut l_menu_s, &Val::from(1), Val::from("Storage Password Service"), true);
        runtime::local_set(&mut l_menu_s, &Val::from(2), Val::from("Kafra Employee Locations"), true);
        runtime::local_set(&mut l_menu_s, &Val::from(3), Val::from("Cancel"), true);
    }
    let l_menu_list_s = runtime::implode(&l_menu_s, &Val::from(":"))?;
    loop {
        let l_j = Val::from(runtime::select_values(ctx, &[l_menu_list_s.clone()])?).try_sub(Val::from(1))?;
        'b2: {
            let subject2 = l_j;
            let mut matched2 = false;
            let no_case2 = !subject2.loosely_equals(&Val::from(0))
                && !subject2.loosely_equals(&Val::from(1))
                && !subject2.loosely_equals(&Val::from(2));
            if !matched2 && subject2.loosely_equals(&Val::from(0)) {
                matched2 = true;
            }
            if matched2 {
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "Let's see...",
                        Val::from("") + ctx.player().name()? + Val::from("..."),
                        "Ah, you have a total of",
                        Val::from("") + ctx.var("resrvpts").get()? + Val::from(" Special Reserve Points.")
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "You can exchange your",
                        "Special Reserve Points for",
                        "rewards at the Kafra Main Office in Al De Baran. Please use our",
                        "convenient services to see the benefits of our rewards program."
                    ],
                )?;
                if runtime::arg(&args, 0, Val::from(0)) == 1 {
                    return Ok(Val::from(0));
                }
                ctx.next()?;
                break 'b2;
            }
            if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                matched2 = true;
            }
            if matched2 {
                f_setkafcode(ctx, args!["[Kafra Employee]", "Kafra Services"])?;
            }
            if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                matched2 = true;
            }
            if matched2 {
                for i in 0..=3 {
                    ctx.call(
                        Function::ViewPoint,
                        args![
                            1,
                            ctx.var("@viewpx").get_at(runtime::index(&Val::from(i))?)?,
                            ctx.var("@viewpy").get_at(runtime::index(&Val::from(i))?)?,
                            i + 1,
                            16711935
                        ],
                    )?;
                }
                ctx.next()?;
                for i in 0..=3 {
                    ctx.call(
                        Function::ViewPoint,
                        args![
                            2,
                            ctx.var("@viewpx").get_at(runtime::index(&Val::from(i))?)?,
                            ctx.var("@viewpy").get_at(runtime::index(&Val::from(i))?)?,
                            i + 1,
                            16711935
                        ],
                    )?;
                }
                break 'b2;
            }
            if !matched2 && no_case2 {
                matched2 = true;
            }
            if matched2 {
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn f_kafra(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    crate::other_global_functions::f_cleargarbage(ctx, vec![])?;
    let l_welcome = runtime::arg(&args, 0, Val::from(0));
    let l_menu_num = runtime::arg(&args, 1, Val::from(0));
    'b1: {
        let subject1 = l_welcome.clone();
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1))
            && !subject1.loosely_equals(&Val::from(2))
            && !subject1.loosely_equals(&Val::from(3))
            && !subject1.loosely_equals(&Val::from(4))
            && !subject1.loosely_equals(&Val::from(5));
        if !matched1 && no_case1 {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "Welcome to the",
                    "Kafra Corporation.",
                    "The Kafra services",
                    "are always on your side.",
                    "How may I assist you?"
                ],
            )?;
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "^666666W-weeeelc-c-come",
                    "to th-the K-kaaafrrrra",
                    "C-coorpoor-r-ratioooonn...^000000"
                ],
            )?;
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    Val::from("Welcome. ^ff0000")
                        + ctx.call(Function::GetGuildInfo, args![ctx.call(Function::GetCharacterId, args![2])?, 0])?
                        + Val::from("^000000 Member."),
                    "The Kafra Coporation will stay with you wherever you go."
                ],
            )?;
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "So, have you come from a faraway land to study our culture, or are you just sightseeing?",
                    "In either case, why not stay awhile?",
                    "The air is eternally heavy with the",
                    "scent of pleasant wildflowers."
                ],
            )?;
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Kafra Employee",
                args!["With our many Kafra", "service locations, you're never", "far from home."],
            )?;
            break 'b1;
        }
    }
    ctx.next()?;
    let mut l_k_menu0_s: Vec<Val> = Vec::new();
    if l_welcome == 2 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Use Guild Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Use Teleport Service"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(4), Val::from("Cancel"), true);
    } else if l_menu_num == 1 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Save"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Cancel"), true);
    } else if l_menu_num == 2 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Cancel"), true);
    } else if l_menu_num == 3 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Save"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Check Other Information"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(4), Val::from("Cancel"), true);
    } else if l_menu_num == 5 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Check Other Information"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Cancel"), true);
    } else if l_menu_num == 6 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Check Other Information"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Cancel"), true);
    } else if l_menu_num == 7 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Save"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Cancel"), true);
    } else if l_menu_num == 8 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Save"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Check Other Information"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Cancel"), true);
    } else if l_menu_num == 9 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Use Teleport Service"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Check Other Information"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(4), Val::from("Cancel"), true);
    } else if l_menu_num == 10 {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Save"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Cancel"), true);
    } else {
        runtime::local_set(&mut l_k_menu0_s, &Val::from(0), Val::from("Save"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(1), Val::from("Use Storage"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(2), Val::from("Use Teleport Service"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(3), Val::from("Rent a Pushcart"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(4), Val::from("Check Other Information"), true);
        runtime::local_set(&mut l_k_menu0_s, &Val::from(5), Val::from("Cancel"), true);
    }
    let l_menu_s = runtime::implode(&l_k_menu0_s, &Val::from(":"))?;
    loop {
        let l_j = Val::from(runtime::select_values(ctx, &[l_menu_s.clone()])?).try_sub(Val::from(1))?;
        if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Save" {
            return Ok(Val::from(0));
        } else if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Use Storage" {
            if l_welcome == 2 {
                f_kafstor(ctx, args![2, 0, 0])?;
            } else {
                f_kafstor(ctx, args![0, runtime::arg(&args, 3, Val::from(0)), l_welcome.clone()])?;
            }
            ctx.next()?;
        } else if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Use Teleport Service" {
            if l_menu_num != 4 {
                f_kaftele(ctx, args![l_welcome.clone()])?;
            } else {
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "Because of the ^FF0000Limited",
                        "Transport Agreement^000000, the",
                        "Kafra Corporation cannot",
                        "provide Teleport Services",
                        "in the Schwarzwald Republic."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Kafra Employee",
                    args![
                        "We ask that you please",
                        "use the Airship Service",
                        "instead. Thank you for your",
                        "understanding and cooperation."
                    ],
                )?;
            }
            ctx.next()?;
        } else if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Rent a Pushcart" {
            if f_kafcart(ctx, args![l_welcome.clone(), runtime::arg(&args, 4, Val::from(0))])? == 1 {
                ctx.next()?;
            }
        } else if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Check Other Information" {
            f_kafinfo(ctx, args![runtime::arg(&args, 2, Val::from(0))])?;
            ctx.next()?;
        } else if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Cancel" {
            f_kafend(ctx, args![l_welcome.clone(), 0])?;
            return Err(Stop::End);
        } else if runtime::local_get(&l_k_menu0_s, &l_j, true) == "Use Guild Storage" {
            f_kafstor(ctx, args![1, 0])?;
            ctx.next()?;
        }
    }
}

pub fn f_kafset(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    runtime::array_delete(ctx, "@wrpc$", &Val::from(0), Some(&runtime::array_size(ctx, "@wrpc$")?))?;
    runtime::array_delete(ctx, "@wrpd$", &Val::from(0), Some(&runtime::array_size(ctx, "@wrpd$")?))?;
    runtime::array_delete(ctx, "@wrpp$", &Val::from(0), Some(&runtime::array_size(ctx, "@wrpp$")?))?;
    runtime::array_delete(ctx, "@viewpx", &Val::from(0), Some(&runtime::array_size(ctx, "@viewpx")?))?;
    runtime::array_delete(ctx, "@viewpy", &Val::from(0), Some(&runtime::array_size(ctx, "@viewpy")?))?;
    let l_map_s = ctx.call(Function::StrNpcInfo, args![4])?;
    if l_map_s == "prontera" {
        ctx.var("@wrpd$").set_at(0, Val::from("Izlude"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Geffen"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Payon"))?;
        ctx.var("@wrpd$").set_at(3, Val::from("Morocc"))?;
        ctx.var("@wrpd$").set_at(4, Val::from("Orc Dungeon"))?;
        ctx.var("@wrpd$").set_at(5, Val::from("Alberta"))?;
        ctx.var("@wrpp").set_at(0, Val::from(600))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1200))?;
        ctx.var("@wrpp").set_at(3, Val::from(1200))?;
        ctx.var("@wrpp").set_at(4, Val::from(1700))?;
        ctx.var("@wrpp").set_at(5, Val::from(1800))?;
        ctx.var("@viewpx").set_at(0, Val::from(151))?;
        ctx.var("@viewpx").set_at(1, Val::from(29))?;
        ctx.var("@viewpx").set_at(2, Val::from(282))?;
        ctx.var("@viewpx").set_at(3, Val::from(152))?;
        ctx.var("@viewpy").set_at(0, Val::from(29))?;
        ctx.var("@viewpy").set_at(1, Val::from(207))?;
        ctx.var("@viewpy").set_at(2, Val::from(200))?;
        ctx.var("@viewpy").set_at(3, Val::from(326))?;
    } else if l_map_s == "alberta" {
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1800))?;
        ctx.var("@wrpp").set_at(2, Val::from(1800))?;
        ctx.var("@wrpd$").set_at(0, Val::from("Payon"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Morocc"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Prontera"))?;
        ctx.var("@viewpx").set_at(0, Val::from(28))?;
        ctx.var("@viewpx").set_at(1, Val::from(113))?;
        ctx.var("@viewpx").set_at(2, Val::from(0))?;
        ctx.var("@viewpx").set_at(3, Val::from(0))?;
        ctx.var("@viewpy").set_at(0, Val::from(229))?;
        ctx.var("@viewpy").set_at(1, Val::from(60))?;
        ctx.var("@viewpy").set_at(2, Val::from(0))?;
        ctx.var("@viewpy").set_at(3, Val::from(0))?;
    } else if l_map_s == "aldebaran" {
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1800))?;
        ctx.var("@wrpp").set_at(3, Val::from(1700))?;
        ctx.var("@wrpd$").set_at(0, Val::from("Geffen"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Juno"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Izlude"))?;
        ctx.var("@wrpd$").set_at(3, Val::from("Mjolnir Dead Pit"))?;
    } else if l_map_s == "comodo" {
        ctx.var("@wrpd$").set_at(0, Val::from("Morocc"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Comodo Pharos Beacon"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Umbala"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1800))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1800))?;
    } else if l_map_s == "cmd_fild07" {
        ctx.var("@wrpd$").set_at(0, Val::from("Comodo"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Morocc"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
    } else if l_map_s == "geffen" {
        ctx.var("@wrpd$").set_at(0, Val::from("Prontera"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Al De Baran"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Orc Dungeon"))?;
        ctx.var("@wrpd$").set_at(3, Val::from("Mjolnir Dead Pit"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1700))?;
        ctx.var("@wrpp").set_at(3, Val::from(1700))?;
        ctx.var("@viewpx").set_at(0, Val::from(120))?;
        ctx.var("@viewpx").set_at(1, Val::from(203))?;
        ctx.var("@viewpy").set_at(0, Val::from(62))?;
        ctx.var("@viewpy").set_at(1, Val::from(123))?;
    } else if l_map_s == "izlude" {
        ctx.var("@wrpd$").set_at(0, Val::from("Geffen"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Payon"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Morocc"))?;
        ctx.var("@wrpd$").set_at(3, Val::from("Al De Baran"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1200))?;
        ctx.var("@wrpp").set_at(3, Val::from(1800))?;
    } else if l_map_s == "morocc" {
        ctx.var("@wrpd$").set_at(0, Val::from("Prontera"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Payon"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Alberta"))?;
        ctx.var("@wrpd$").set_at(3, Val::from("Comodo"))?;
        ctx.var("@wrpd$").set_at(4, Val::from("Comodo Pharos Beacon"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1800))?;
        ctx.var("@wrpp").set_at(3, Val::from(1800))?;
        ctx.var("@wrpp").set_at(4, Val::from(1200))?;
        ctx.var("@viewpx").set_at(0, Val::from(156))?;
        ctx.var("@viewpx").set_at(1, Val::from(163))?;
        ctx.var("@viewpx").set_at(2, Val::from(28))?;
        ctx.var("@viewpx").set_at(3, Val::from(292))?;
        ctx.var("@viewpy").set_at(0, Val::from(97))?;
        ctx.var("@viewpy").set_at(1, Val::from(260))?;
        ctx.var("@viewpy").set_at(2, Val::from(167))?;
        ctx.var("@viewpy").set_at(3, Val::from(211))?;
    } else if l_map_s == "umbala" {
        ctx.var("@wrpd$").set_at(0, Val::from("Comodo"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1800))?;
    } else if l_map_s == "payon" {
        ctx.var("@wrpd$").set_at(0, Val::from("Prontera"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Alberta"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Morocc"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1200))?;
    } else if l_map_s == "yuno" {
        ctx.var("@wrpd$").set_at(0, Val::from("Al De Baran"))?;
        ctx.var("@wrpp").set_at(0, Val::from(1200))?;
        ctx.var("@viewpx").set_at(0, Val::from(328))?;
        ctx.var("@viewpx").set_at(1, Val::from(278))?;
        ctx.var("@viewpx").set_at(2, Val::from(153))?;
        ctx.var("@viewpx").set_at(3, Val::from(0))?;
        ctx.var("@viewpy").set_at(0, Val::from(108))?;
        ctx.var("@viewpy").set_at(1, Val::from(221))?;
        ctx.var("@viewpy").set_at(2, Val::from(187))?;
        ctx.var("@viewpy").set_at(3, Val::from(0))?;
    } else if l_map_s == "job3_rune01" {
        ctx.var("@wrpd$").set_at(0, Val::from("Izlude"))?;
        ctx.var("@wrpd$").set_at(1, Val::from("Geffen"))?;
        ctx.var("@wrpd$").set_at(2, Val::from("Payon"))?;
        ctx.var("@wrpd$").set_at(3, Val::from("Morocc"))?;
        ctx.var("@wrpd$").set_at(4, Val::from("Alberta"))?;
        ctx.var("@wrpp").set_at(0, Val::from(600))?;
        ctx.var("@wrpp").set_at(1, Val::from(1200))?;
        ctx.var("@wrpp").set_at(2, Val::from(1200))?;
        ctx.var("@wrpp").set_at(3, Val::from(1200))?;
        ctx.var("@wrpp").set_at(4, Val::from(1800))?;
    }
    let l_warp_size = runtime::array_size(ctx, "@wrpd$")?;
    if ctx.constant("VIP_SCRIPT")?.is_true() && !ctx.call(Function::VipStatus, args![ctx.constant("VIP_STATUS_ACTIVE")?])?.is_true() {
        let mut l_i = Val::from(0);
        while runtime::op(&l_i, "<", &l_warp_size)?.is_true() {
            ctx.var("@wrpp").set_at(
                runtime::index(&l_i)?,
                ctx.var("@wrpp").get_at(runtime::index(&l_i)?)?.try_mul(Val::from(2))?,
            )?;
            l_i = l_i + Val::from(1);
        }
    }
    let mut l_i = Val::from(0);
    while runtime::op(&l_i, "<", &l_warp_size)?.is_true() {
        ctx.var("@wrpc$").set_at(
            runtime::index(&l_i)?,
            ctx.var("@wrpd$").get_at(runtime::index(&l_i)?)?
                + Val::from(" -> ")
                + ctx.var("@wrpp").get_at(runtime::index(&l_i)?)?
                + Val::from(" z"),
        )?;
        l_i = l_i + Val::from(1);
    }
    ctx.var("@wrpc$").set_at(runtime::index(&l_warp_size)?, Val::from("Cancel"))?;
    Ok(Val::from(0))
}

pub fn f_kafstor(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let l_type = runtime::arg(&args, 0, Val::from(0));
    let l_fee = runtime::arg(&args, 1, Val::from(0));
    if l_type == 1 {
        if ctx.call(Function::GuildOpenStorage, vec![])?.is_true() {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "I'm sorry but another guild member is using the guild storage",
                    "right now.  Please wait until that person is finished."
                ],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return Err(Stop::End);
        }
        ctx.fx().cutin("", 255)?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if !crate::other_global_functions::f_canopenstorage(ctx, vec![])?.is_true() {
        ctx.mes("[Kafra Employee]")?;
        if runtime::arg(&args, 2, Val::from(0)) == 1 {
            ctx.lines(args![
                "^666666S-s-ssoooorry,",
                "y-you're a-a-aaaa",
                "Nooviiice... N-neeeds",
                "B-basic sssskill l-level 6...^000000"
            ])?;
            return Ok(Val::from(0));
        }
        ctx.lines(args![
            "I'm sorry, but you",
            "need the Novice's",
            "Basic Skill Level 6 to",
            "use the Storage Service."
        ])?;
        return Ok(Val::from(0));
    }
    if l_type != 2 {
        if ctx.call(Function::CountItem, args![7059])?.is_true() {
            ctx.items().take(7059, 1)?;
        } else {
            if runtime::op(&ctx.var("Zeny").get()?, "<", &l_fee)?.is_true() {
                ctx.mes("[Kafra Employee]")?;
                if runtime::arg(&args, 2, Val::from(0)) == 1 {
                    ctx.call(Function::PercentHeal, args![-50, -50])?;
                    ctx.lines(args![
                        "^666666Zeeeeeny...",
                        "M-more z-zeny...!",
                        "N-neeed 150... zeny...",
                        "Ergh! T-taking bl-blood~!^000000"
                    ])?;
                    return Ok(Val::from(0));
                }
                ctx.lines(args![
                    "I'm sorry, but you don't",
                    "have enough zeny to use",
                    "the Storage Service. Our",
                    Val::from("Storage access fee is ") + l_fee.clone() + Val::from(" zeny.")
                ])?;
                return Ok(Val::from(0));
            }
            ctx.var("Zeny").set(ctx.var("Zeny").get()?.try_sub(l_fee.clone())?)?;
            ctx.var("resrvpts")
                .set(ctx.var("resrvpts").get()? + l_fee.clone().try_div(Val::from(5))?)?;
        }
        if runtime::arg(&args, 2, Val::from(0)) == 1 {
            ctx.call(Function::PercentHeal, args![0, -10])?;
            ctx.mes("[Kafra Employee]")?;
            for _ in 0..5 {
                ctx.mes("^666666Thank you.. for... using...^000000")?;
            }
        } else {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "Here, let me open",
                    "your Storage for you.",
                    "Thank you for using",
                    "the Kafra Service."
                ],
            )?;
        }
    }
    f_checkkafcode(ctx, vec![])?;
    ctx.close_window()?;
    ctx.call(Function::OpenStorage, vec![])?;
    ctx.fx().cutin("", 255)?;
    Err(Stop::End)
}

pub fn f_kaftele(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as("Kafra Employee", args!["Please choose", "your destination."])?;
    ctx.next()?;
    let l_j = Val::from(runtime::select_values(
        ctx,
        &[runtime::array_implode(ctx, "@wrpc$", &Val::from(":"))?],
    )?)
    .try_sub(Val::from(1))?;
    if ctx.var("@wrpc$").get_at(runtime::index(&l_j)?)? == "Cancel" {
        return Ok(Val::from(0));
    }
    if ctx.items().count(7060)? > 0 && runtime::arg(&args, 0, Val::from(0)) != 2 {
        ctx.items().take(7060, 1)?;
    } else {
        if runtime::op(&ctx.var("Zeny").get()?, "<", &ctx.var("@wrpp").get_at(runtime::index(&l_j)?)?)?.is_true() {
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "I'm sorry, but you don't have",
                    "enough zeny for the Teleport",
                    "Service. The fee to teleport",
                    Val::from("to ")
                        + ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)?
                        + Val::from(" is ")
                        + ctx.var("@wrpp").get_at(runtime::index(&l_j)?)?
                        + Val::from(" zeny.")
                ],
            )?;
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return Err(Stop::End);
        }
        ctx.var("Zeny")
            .set(ctx.var("Zeny").get()?.try_sub(ctx.var("@wrpp").get_at(runtime::index(&l_j)?)?)?)?;
        ctx.var("resrvpts")
            .set(ctx.var("resrvpts").get()? + ctx.var("@wrpp").get_at(runtime::index(&l_j)?)?.try_div(Val::from(16))?)?;
    }
    ctx.fx().cutin("", 255)?;
    if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Al De Baran" {
        ctx.warp("aldebaran", 168, 112)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Alberta" {
        ctx.warp("alberta", 117, 56)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Comodo" {
        ctx.warp("comodo", 209, 143)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Comodo Pharos Beacon" {
        ctx.warp("cmd_fild07", 127, 134)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Geffen" {
        ctx.warp("geffen", 120, 39)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Izlude" {
        ctx.warp("izlude", 91, 105)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Juno" {
        ctx.warp("yuno", 158, 125)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Mjolnir Dead Pit" {
        ctx.warp("mjolnir_02", 99, 351)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Morocc" {
        ctx.warp("morocc", 156, 46)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Orc Dungeon" {
        ctx.warp("gef_fild10", 52, 326)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Payon" {
        ctx.warp("payon", 161, 58)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Prontera" {
        ctx.warp("prontera", 116, 72)?;
    } else if ctx.var("@wrpd$").get_at(runtime::index(&l_j)?)? == "Umbala" {
        ctx.warp("umbala", 100, 154)?;
    }
    Err(Stop::End)
}

#[derive(Clone, Copy, Debug)]
enum FSetkafcodeStep {
    Start,
    SSET,
}

fn f_setkafcode_run(ctx: &Ctx, step: FSetkafcodeStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            FSetkafcodeStep::Start => {
                let npc = runtime::arg(&args, 0, Val::from(0));
                let comp = runtime::arg(&args, 1, Val::from(0));
                ctx.lines(args![npc.clone()])?;
                if ctx.var("#kafra_code").get()? == 0 {
                    ctx.lines(args![
                        Val::from("") + comp.clone() + Val::from(" proudly presents you a new service:"),
                        "Additional storage protection with a password."
                    ])?;
                    ctx.next()?;
                    if ctx.menu(&["Set new password -> 5000z", "Cancel"])? == 0 {
                        f_setkafcode_run(ctx, FSetkafcodeStep::SSET, args![npc.clone(), comp.clone()])?;
                    }
                    ctx.close_window()?;
                    ctx.fx().cutin("", 255)?;
                    return Err(Stop::End);
                }
                ctx.mes("Your storage is protected with a password. What would you do now?")?;
                ctx.next()?;
                'b1: {
                    match ctx.menu(&["Change old password -> 5000z", "Remove storage password -> 1000z", "Cancel"])? {
                        0 => {
                            ctx.lines(args![npc.clone(), "At first, please enter your ^0000FFold password^000000."])?;
                            let code = f_entkafcode(ctx, vec![])?;
                            if !code.is_true() || !code.loosely_equals(&ctx.var("#kafra_code").get()?) {
                                ctx.mes("Wrong password. You can't set a new password.")?;
                                ctx.call(Function::Emotion, args![ctx.constant("ET_SCRATCH")?])?;
                                break 'b1;
                            }
                            ctx.next()?;
                            f_setkafcode_run(ctx, FSetkafcodeStep::SSET, args![npc.clone(), comp.clone()])?;
                        }
                        1 => {
                            ctx.lines(args![npc.clone(), "Please, enter your password before its removal."])?;
                            let code = f_entkafcode(ctx, vec![])?;
                            if !code.is_true() {
                                ctx.mes("The password hasn't been removed.")?;
                                ctx.call(Function::Emotion, args![ctx.constant("ET_SCRATCH")?])?;
                                break 'b1;
                            }
                            ctx.next()?;
                            ctx.lines(args![npc.clone()])?;
                            if ctx.player().zeny()? < 1000 {
                                ctx.mes("You don't have enough zeny.")?;
                                ctx.call(Function::Emotion, args![ctx.constant("ET_MONEY")?])?;
                                break 'b1;
                            }
                            ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
                            if code.loosely_equals(&ctx.var("#kafra_code").get()?) {
                                ctx.var("#kafra_code").set(Val::from(0))?;
                                ctx.lines(args![
                                    "You've successfully cleared your storage password.",
                                    Val::from("Thank you for using ") + comp.clone() + Val::from(".")
                                ])?;
                                ctx.call(Function::Emotion, args![ctx.constant("ET_THANKS")?])?;
                            } else {
                                ctx.lines(args![
                                    "Wrong password. We won't return your 1000z.",
                                    "Please, next time enter correct password."
                                ])?;
                                ctx.call(Function::Emotion, args![ctx.constant("ET_SORRY")?])?;
                            }
                        }
                        _ => {}
                    }
                }
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return Err(Stop::End);
            }
            FSetkafcodeStep::SSET => {
                let npc = runtime::arg(&args, 0, Val::from(0));
                let comp = runtime::arg(&args, 1, Val::from(0));
                ctx.lines(args![
                    npc.clone(),
                    "Now enter your ^FF0000new password^000000 to protect your storage from thieves."
                ])?;
                let code = f_entkafcode(ctx, vec![])?;
                if !code.is_true() {
                    ctx.mes("The password hasn't been changed.")?;
                    ctx.call(Function::Emotion, args![ctx.constant("ET_SCRATCH")?])?;
                    return Ok(Val::from(0));
                }
                ctx.next()?;
                ctx.lines(args![npc])?;
                if ctx.player().zeny()? < 5000 {
                    ctx.mes("You don't have enough zeny.")?;
                    ctx.call(Function::Emotion, args![ctx.constant("ET_MONEY")?])?;
                    return Ok(Val::from(0));
                }
                ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
                ctx.var("#kafra_code").set(code)?;
                ctx.lines(args![
                    "You've protected your storage with a secret password.",
                    Val::from("Thank you for using ") + comp + Val::from(".")
                ])?;
                ctx.call(Function::Emotion, args![ctx.constant("ET_THANKS")?])?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn f_setkafcode(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    f_setkafcode_run(ctx, FSetkafcodeStep::Start, args)
}
