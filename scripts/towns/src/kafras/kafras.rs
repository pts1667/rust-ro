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

pub fn kafra_service(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_05", 2)?;
    ctx.lines_as(
        "Kafra Leilah",
        args![
            "Hm...?",
            "Oh, welcome to",
            "the Kafra Corporation",
            "Headquarters. Did you",
            "need something?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Save", "Use Storage", "Rent a Pushcart", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "Your Respawn Point has",
                    "been saved here, inside",
                    "of the Kafra Corporation",
                    "Headquarters. Thank you."
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::SavePoint, args!["aldeba_in", 96, 179, 1, 1])?;
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "Please make use of",
                    "the Kafra Services that are",
                    "available throughout all of",
                    "Midgard. Thank you for",
                    "visiting the Kafra Headquarters."
                ],
            )?;
            ctx.close_window()?;
        }
        1 => {
            if ctx.player().zeny()? < 20 {
                ctx.lines_as(
                    "Kafra Leilah",
                    args![
                        "Excuse me, but it",
                        "seems that you don't",
                        "have the 20 zeny to pay",
                        "the Storage access fee..."
                    ],
                )?;
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return Err(Stop::End);
            }
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "Although this facility is",
                    "exclusively intended for",
                    "the training of Kafra Employee",
                    "and administrative functions,",
                    "I'll access your Storage for you."
                ],
            )?;
            ctx.next()?;
            ctx.player().set_zeny(ctx.player().zeny()? - 20)?;
            ctx.var("resrvpts")
                .set((ctx.var("resrvpts").get()? + Val::from(20).try_div(Val::from(5))?))?;
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "In the future, please",
                    "ask the Kafra Employee on",
                    "duty if you wish to use",
                    "any of the Kafra Services.",
                    "Thank you for your patronage."
                ],
            )?;
            shared::kafras_functions_kafras::f_checkkafcode(ctx, vec![])?;
            ctx.close_window()?;
            ctx.call(Function::OpenStorage, vec![])?;
        }
        2 => {
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "My apologies, but I'm",
                    "not on duty. I'd assist you",
                    "if I could, but actually don't",
                    "have any available Pushcarts.",
                    "Why don't you ask another Kafra",
                    "Employee for assistance?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "Kafra Employees are",
                    "stationed all over the",
                    "Midgard continent,",
                    "and you should be able to find",
                    "plenty outside in Al De Baran."
                ],
            )?;
            ctx.close_window()?;
        }
        3 => {
            ctx.lines_as(
                "Kafra Leilah",
                args![
                    "^666666*Whew...*^000000",
                    "Great, because I'm",
                    "actually on my break",
                    "right now. Choosing",
                    "''Cancel'' was a good",
                    "move on your part."
                ],
            )?;
            ctx.close_window()?;
        }
        _ => {}
    }
    ctx.fx().cutin("", 255)?;
    return Err(Stop::End);
}

pub fn kafra_employee(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_05", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 20, 600])?;
    ctx.call(Function::SavePoint, args!["aldebaran", 143, 109, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Al De Baran"])?;
    Ok(())
}

pub fn kafra_employee_l130(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_03", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome~!",
            "The Kafra Services",
            "are always on your side.",
            "So how can I help you?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 30, 750])?;
    ctx.call(Function::SavePoint, args!["geffen", 119, 40, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Geffen"])?;
    Ok(())
}

pub fn kafra_employee_l143(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome!",
            "The Kafra Corporation",
            "will always support the",
            "adventurers of Rune-Midgarts",
            "with its excellent service. So",
            "what can I do for you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 30, 750])?;
    ctx.call(Function::SavePoint, args!["geffen", 200, 124, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Geffen"])?;
    Ok(())
}

pub fn kafra_employee_l160(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_05", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 60, 930])?;
    ctx.call(Function::SavePoint, args!["morocc", 156, 46, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Morocc"])?;
    Ok(())
}

pub fn kafra(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome!",
            "The Kafra Corporation",
            "will always support the",
            "adventurers of Rune-Midgarts",
            "with its excellent service. So",
            "what can I do for you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 60, 930])?;
    ctx.call(Function::SavePoint, args!["morocc", 157, 272, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Morocc"])?;
    Ok(())
}

pub fn kafra_employee_l190(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_05", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 60, 930])?;
    ctx.call(Function::SavePoint, args!["payon", 160, 58, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Payon"])?;
    Ok(())
}

pub fn kafra_employee_l203(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "The Kafra services are",
            "always on your side.",
            "How may I assist you?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 60, 930])?;
    ctx.call(Function::SavePoint, args!["payon", 257, 242, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Payon"])?;
    Ok(())
}

pub fn kafra_employee_l217(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_03", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
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
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 5, 1, 90, 1200])?;
    ctx.call(Function::SavePoint, args!["pay_arche", 49, 144, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at the Payon Dungeon"])?;
    Ok(())
}

pub fn kafra_employee_l233(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_06", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation~",
            "The Kafra Services are",
            "always here to support",
            "you. So how can I be",
            "of service today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["prontera", 157, 327, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Prontera"])?;
    Ok(())
}

pub fn kafra_employee_l248(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_03", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome~!",
            "The Kafra Services",
            "are always on your side.",
            "So how can I help you?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["prontera", 150, 33, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Prontera"])?;
    Ok(())
}

pub fn kafra_employee_l261(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_05", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["prontera", 33, 208, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Prontera"])?;
    Ok(())
}

pub fn kafra_employee_l274(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome!",
            "The Kafra Corporation",
            "will always support the",
            "adventurers of Rune-Midgarts",
            "with its excellent service. So",
            "what can I do for you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["prontera", 281, 203, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Prontera"])?;
    Ok(())
}

pub fn kafra_employee_l289(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_01", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
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
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["prontera", 116, 73, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Prontera"])?;
    Ok(())
}

pub fn kafra_employee_l305(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_08", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["yuno", 158, 125, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Juno"])?;
    Ok(())
}

pub fn kafra_employee_l318(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_08", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["yuno", 328, 101, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Juno"])?;
    Ok(())
}

pub fn kafra_employee_l331(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_09", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "The Kafra Corporation",
            "is always working to provide",
            "you with convenient services.",
            "How may I be of assistance?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 40, 800])?;
    ctx.call(Function::SavePoint, args!["yuno", 274, 229, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Juno"])?;
    Ok(())
}

pub fn kafra_employee_l346(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 50, 850])?;
    ctx.call(Function::SavePoint, args!["alberta", 31, 231, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Alberta"])?;
    Ok(())
}

pub fn kafra_employee_l360(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_06", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation~",
            "The Kafra Services are",
            "always here to support",
            "you. So how can I be",
            "of service today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 0, 50, 850])?;
    ctx.call(Function::SavePoint, args!["alberta", 117, 57, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Alberta"])?;
    Ok(())
}

pub fn kafra_employee_l377(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_07", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "You know that our",
            "service is always",
            "on your side~"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 80, 1000])?;
    ctx.call(Function::SavePoint, args!["comodo", 204, 143, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the town of Comodo"])?;
    Ok(())
}

pub fn kafra_employee_l391(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_07", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "You know that our",
            "service is always",
            "on your side~"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 80, 1000])?;
    ctx.call(Function::SavePoint, args!["cmd_fild07", 127, 134, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in Pyros Lighthouse"])?;
    Ok(())
}

pub fn kaf_izlude(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_01", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
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
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 40, 820])?;
    ctx.call(Function::SavePoint, args!["izlude", 94, 103, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Izlude"])?;
    Ok(())
}

pub fn kafra_employee_l426(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![0, 3, 0, 80, 700])?;
    ctx.call(Function::SavePoint, args!["moscovia", 221, 194, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Moscovia"])?;
    Ok(())
}

pub fn kafra_employee_l435(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 3, 1, 50, 700])?;
    ctx.call(Function::SavePoint, args!["amatsu", 116, 94, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Amatsu"])?;
    Ok(())
}

pub fn kafra_employee_l450(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 3, 1, 50, 700])?;
    ctx.call(Function::SavePoint, args!["ayothaya", 149, 69, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Ayothaya"])?;
    Ok(())
}

pub fn kafra_employee_ein3(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_08", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome~!",
            "The Kafra Services",
            "are always on your side.",
            "So how can I help you?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 4, 1, 40, 850])?;
    ctx.call(Function::SavePoint, args!["einbech", 182, 124, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the town of Einbech"])?;
    Ok(())
}

pub fn kafra_employee_ein2(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_08", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 4, 1, 50, 800])?;
    ctx.call(Function::SavePoint, args!["einbroch", 238, 198, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Einbroch"])?;
    Ok(())
}

pub fn kafra_employee_ein1(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_09", 2)?;
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
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 4, 1, 50, 800])?;
    ctx.call(Function::SavePoint, args!["einbroch", 240, 197, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Einbroch"])?;
    Ok(())
}

pub fn kafra_employee_l507(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 3, 1, 50, 700])?;
    ctx.call(Function::SavePoint, args!["gonryun", 160, 62, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Kunlun"])?;
    Ok(())
}

pub fn kafra_employee_l522(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_08", 2)?;
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
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 4, 1, 40, 800])?;
    ctx.call(Function::SavePoint, args!["lighthalzen", 158, 94, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Lighthalzen"])?;
    Ok(())
}

pub fn kafra_employee_l535(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_09", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome~!",
            "The Kafra Services",
            "are always on your side.",
            "So how can I help you?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 4, 1, 40, 800])?;
    ctx.call(Function::SavePoint, args!["lighthalzen", 194, 313, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Lighthalzen"])?;
    Ok(())
}

pub fn kafra_employee_l547(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_09", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome~!",
            "The Kafra Services",
            "are always on your side.",
            "So how can I help you?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 4, 1, 40, 800])?;
    ctx.call(Function::SavePoint, args!["lhz_in02", 278, 215, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Lighthalzen"])?;
    Ok(())
}

pub fn kafra_employee_l561(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_01", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 3, 1, 50, 700])?;
    ctx.call(Function::SavePoint, args!["louyang", 217, 92, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Luoyang"])?;
    Ok(())
}

pub fn kafra_employee_l576(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_07", 2)?;
    shared::kafras_functions_kafras::f_kafset(ctx, vec![])?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "You know that our",
            "service is always",
            "on your side~"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 0, 1, 80, 0])?;
    ctx.call(Function::SavePoint, args!["umbala", 126, 131, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Umbala"])?;
    Ok(())
}

pub fn kafra_employee_l592(ctx: &Ctx) -> Script {
    shared::kafras_functions_kafras::f_kafra(ctx, args![1, 2, 1, 150, 0])?;
    ctx.call(Function::SavePoint, args!["niflheim", 192, 182, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![1, 1, "in the city of Niflheim"])?;
    Ok(())
}

pub fn kafra_employee_l601(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_03", 2)?;
    if ctx.var("BaseJob").get()? == constants::JOB_NOVICE && ctx.var("job_merchant_q2").get()?.number()? > 0 {
        shared::pre_re_jobs_1_1_merchant::f_merckafra(ctx, vec![])?;
    }
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 2, 1, 120, 1200])?;
    ctx.call(Function::SavePoint, args!["izlu2dun", 87, 170, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at Byalan Island"])?;
    Ok(())
}

pub fn kafra_employee_l620(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome!",
            "The Kafra Corporation",
            "will always support the",
            "adventurers of Rune-Midgarts",
            "with its excellent service. So",
            "what can I do for you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 1, 1, 40, 0])?;
    ctx.call(Function::SavePoint, args!["prt_fild05", 274, 243, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at the Prontera Culverts"])?;
    Ok(())
}

pub fn kafra_employee_l635(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 6, 1, 100, 0])?;
    ctx.call(Function::SavePoint, args!["mjolnir_02", 98, 352, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at Mjolnir Dead Pit"])?;
    Ok(())
}

pub fn kafra_employee_l649(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_04", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome!",
            "The Kafra Corporation",
            "will always support the",
            "adventurers of Rune-Midgarts",
            "with its excellent service. So",
            "what can I do for you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 2, 1, 90, 1200])?;
    ctx.call(Function::SavePoint, args!["moc_ruins", 41, 141, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at the Pyramids"])?;
    Ok(())
}

pub fn kafra_employee_l664(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_02", 2)?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Welcome to the",
            "Kafra Corporation.",
            "Kafra's Employees are",
            "always ready to serve you.",
            "How can I help you today?"
        ],
    )?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 6, 1, 130, 0])?;
    ctx.call(Function::SavePoint, args!["gef_fild10", 54, 326, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at the Orc Dungeon"])?;
    Ok(())
}

pub fn kafra_employee_l678(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_01", 2)?;
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
    shared::kafras_functions_kafras::f_kafra(ctx, args![5, 2, 1, 50, 0])?;
    ctx.call(Function::SavePoint, args!["alb2trea", 92, 64, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "at Sunken Ship"])?;
    Ok(())
}

pub fn kafra_employee_l693(ctx: &Ctx) -> Script {
    ctx.fx().cutin("kafra_01", 2)?;
    shared::kafras_functions_kafras::f_kafra(ctx, args![0, 3, 0, 80, 700])?;
    ctx.call(Function::SavePoint, args!["brasilis", 195, 259, 1, 1])?;
    shared::kafras_functions_kafras::f_kafend(ctx, args![0, 1, "in the city of Brasilis"])?;
    Ok(())
}
