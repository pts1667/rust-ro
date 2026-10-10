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

pub fn peco_peco_breeder_knt(ctx: &Ctx) -> Script {
    let mut price = 0;
    if ctx.var("Upper").get()? == 0 {
        price = 2500;
    }
    if ctx.var("Upper").get()? == 1 {
        price = 2500;
    }
    if ctx.var("Upper").get()? == 2 {
        price = 2500;
    }
    ctx.mes("[Peco Peco Breeder]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_KNIGHT && ctx.player().class()? < constants::JOB_RUNE_KNIGHT {
        ctx.lines(args![
            "Welcome.",
            "Honorable Knight,",
            "would you like to rent",
            "a Peco Peco? The rental",
            Val::from("fee is ") + price + " zeny."
        ])?;
        ctx.next()?;
        match ctx.menu(&["Rent Peco Peco", "Cancel"])? {
            0 => {
                if ctx.player().zeny()? < price {
                    ctx.lines_as(
                        "Peco Peco Breeder",
                        args!["You do not", "have enough zeny.", "Are you...", "bankrupt?"],
                    )?;
                    return ctx.close();
                }
                if ctx.call(Function::GetSkillLv, args!["KN_RIDING"])? == 0 {
                    ctx.lines_as(
                        "Peco Peco Breeder",
                        args![
                            "I'm sorry, but you're",
                            "not eligible for this",
                            "service. Please go learn",
                            "the Peco Peco Ride skill first."
                        ],
                    )?;
                    return ctx.close();
                }
                if ctx.call(Function::CheckRiding, vec![])?.is_true() {
                    ctx.lines_as("Peco Peco Breeder", args!["You're already", "mounted on a", "Peco Peco."])?;
                    return ctx.close();
                }
                if ctx.call(Function::IsMounting, vec![])?.is_true() {
                    ctx.lines_as("Peco Peco Breeder", args!["Please remove your cash mount."])?;
                    return ctx.close();
                }
                ctx.player().set_zeny(ctx.player().zeny()? - price)?;
                ctx.call(Function::SetRiding, vec![])?;
                return ctx.close();
            }
            _ => {
                ctx.lines_as("Peco Peco Breeder", args!["I see.", "Well then,", "have a good day."])?;
                return ctx.close();
            }
        }
    } else {
        ctx.lines(args![
            "I'm sorry, but these",
            "Peco Pecos are only",
            "available for Knights",
            "and Lord Knights."
        ])?;
        return ctx.close();
    }
    Ok(())
}

pub fn peco_peco_breeder_cru(ctx: &Ctx) -> Script {
    let mut price = 0;
    if ctx.var("Upper").get()? == 0 {
        price = 3500;
    }
    if ctx.var("Upper").get()? == 1 {
        price = 3500;
    }
    if ctx.var("Upper").get()? == 2 {
        price = 3500;
    }
    ctx.mes("[Peco Peco Breeder]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_CRUSADER && ctx.player().class()? < constants::JOB_RUNE_KNIGHT {
        if ctx.var("Upper").get()? == 1 {
            ctx.mes("Welcome, Paladin.")?;
        } else {
            ctx.mes("Welcome, Crusader.")?;
        }
        ctx.lines(args![
            "We have a special",
            "Peco Peco prepared",
            "for you. To rent one",
            Val::from("will cost ") + price + " zeny."
        ])?;
        ctx.next()?;
        match ctx.menu(&["Rent a PecoPeco", "Quit"])? {
            0 => {
                if ctx.player().zeny()? < price {
                    ctx.lines_as(
                        "Peco Peco Breeder",
                        args![
                            "You do not",
                            "have enough zeny.",
                            "If you would like",
                            "a Peco Peco please",
                            Val::from("bring ") + price + " zeny..."
                        ],
                    )?;
                    return ctx.close();
                }
                if ctx.call(Function::GetSkillLv, args!["KN_RIDING"])? == 0 {
                    ctx.lines_as(
                        "Peco Peco Breeder",
                        args!["You must first learn", "to ride a PecoPeco before", "I can rent one to you."],
                    )?;
                    return ctx.close();
                }
                if ctx.call(Function::CheckRiding, vec![])?.is_true() {
                    ctx.lines_as("Peco Peco Breeder", args!["You are already", "mounted on a Peco Peco."])?;
                    return ctx.close();
                }
                if ctx.call(Function::IsMounting, vec![])?.is_true() {
                    ctx.lines_as("Peco Peco Breeder", args!["Please remove your cash mount."])?;
                    return ctx.close();
                }
                ctx.player().set_zeny(ctx.player().zeny()? - price)?;
                ctx.call(Function::SetRiding, vec![])?;
                return ctx.close();
            }
            _ => {
                ctx.lines_as("Peco Peco Breeder", args!["See you around."])?;
                return ctx.close();
            }
        }
    } else {
        ctx.lines(args![
            "What can I do for you?",
            "Please be aware that",
            "this Peco Peco rental",
            "service is strictly for",
            "Crusaders and Paladins."
        ])?;
        return ctx.close();
    }
    Ok(())
}

pub fn falcon_breeder_hnt(ctx: &Ctx) -> Script {
    let mut price = 0;
    if ctx.var("Upper").get()? == 0 {
        price = 2500;
    }
    if ctx.var("Upper").get()? == 1 {
        price = 2500;
    }
    if ctx.var("Upper").get()? == 2 {
        price = 2500;
    }
    ctx.mes("[Falcon Breeder]")?;
    if ctx.var("BaseJob").get()? == constants::JOB_HUNTER {
        ctx.lines(args![
            "Do you need a Falcon?",
            "You can rent your own",
            "trusty bird of prey for a",
            Val::from("fee of just ") + price + " zeny~"
        ])?;
        ctx.next()?;
        match ctx.menu(&["Rent Falcon", "Cancel"])? {
            0 => {
                if ctx.player().zeny()? < price {
                    ctx.lines_as(
                        "Falcon Breeder",
                        args![
                            "What is this?",
                            "You don't have",
                            "enough zeny?!",
                            "You better start",
                            "hunting money",
                            "instead of monsters~"
                        ],
                    )?;
                    return ctx.close();
                }
                if ctx.call(Function::GetSkillLv, args!["HT_FALCON"])? == 0 {
                    ctx.lines_as(
                        "Falcon Breeder",
                        args![
                            "Gosh~",
                            "Go learn how to",
                            "manage a Falcon",
                            "first! I can't rent one",
                            "to you if you can't",
                            "handle it, you know."
                        ],
                    )?;
                    return ctx.close();
                }
                if ctx.call(Function::CheckFalcon, vec![])?.is_true() {
                    ctx.lines_as(
                        "Falcon Breeder",
                        args!["Um...", "You already have", "a Falcon. It's right", "there, can't you see it?"],
                    )?;
                    return ctx.close();
                }
                ctx.player().set_zeny(ctx.player().zeny()? - price)?;
                ctx.call(Function::SetFalcon, vec![])?;
                return ctx.close();
            }
            _ => {
                ctx.lines_as(
                    "Falcon Breeder",
                    args![
                        "W-wait, where're",
                        "you goin'? These",
                        "Falcons are top notch,",
                        "I guarantee it! C'mon, yo~"
                    ],
                )?;
                return ctx.close();
            }
        }
    } else {
        ctx.lines(args![
            "Young fool!",
            "Falcons can only",
            "be used by Hunters",
            "and Snipers, capish?",
            "...Heh heh, jealous?"
        ])?;
        return ctx.close();
    }
    Ok(())
}
