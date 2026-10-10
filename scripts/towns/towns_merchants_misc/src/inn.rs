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

pub fn inn_employee_sammy(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Employee Sammy]", "Nenkaras", "prt_in", 238, 130])?;
    ctx.warp("prt_in", 247, 104)?;
    ctx.end()
}

pub fn inn_employee_ahlma(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Employee Ahlma]", "Nenkaras", "prt_in", 64, 136])?;
    ctx.warp("prt_in", 60, 166)?;
    ctx.end()
}

pub fn inn_employee_jennie(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Employee Jennie]", "'Fisherman Inn'!", "alberta_in", 26, 142])?;
    ctx.warp("alberta_in", 18, 188)?;
    ctx.end()
}

pub fn inn_employee_cena(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(
        ctx,
        args![
            "[Employee Cena]",
            "'Ifrit,' the only Inn in the city of Geffen.",
            "geffen_in",
            70,
            59
        ],
    )?;
    ctx.warp("geffen_in", 31, 31)?;
    ctx.end()
}

pub fn inn_employee_ahee(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Employee Ahee]", "Payon Inn", "payon_in01", 136, 61])?;
    ctx.warp("payon_in01", 132, 11)?;
    ctx.end()
}

pub fn inn_maid_rilim(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Rilim]", "Al De Baran Inn", "aldeba_in", 92, 50])?;
    ctx.warp("aldeba_in", 92, 112)?;
    ctx.end()
}

pub fn inn_keeper_annie(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Annie]", "Rachel Inn", "ra_in01", 375, 58])?;
    ctx.warp("ra_in01", 384, 128)?;
    ctx.end()
}

pub fn hotel_employee_01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hotel Employee",
        args![
            "Welcome to",
            "the Royal Dragon,",
            "where you can find the",
            "finest accomodations",
            "and the best service."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Save Point", "Rest - 5,000 zeny", "Cancel"])? {
        0 => {
            ctx.call(Function::SavePoint, args!["lhz_in02", 209, 275, 1, 1])?;
            ctx.lines_as(
                "Hotel Employee",
                args!["Thank you, your", "Respawn Point has", "been saved here in", "the Royal Dragon."],
            )?;
            return ctx.close();
        }
        1 => {
            if ctx.player().zeny()? < 5000 {
                ctx.lines_as(
                    "Hotel Employee",
                    args!["I'm sorry, but", "you need 5,000 zeny", "in order to check in."],
                )?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
            ctx.lines_as(
                "Hotel Employee",
                args!["Thank you~", "I hope you enjoy", "your stay in the", "Royal Dragon."],
            )?;
            ctx.close_window()?;
            ctx.warp("lhz_in02", 219, 150)?;
            ctx.call(Function::PercentHeal, args![100, 100])?;
            runtime::npc_skill(ctx, &Val::from("AL_BLESSING"), &Val::from(10), &Val::from(99), &Val::from(99))?;
            return ctx.end();
        }
        2 => {
            ctx.lines_as("Hotel Employee", args!["Thank you and", "have a nice day."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn inn_maid_receptionist(ctx: &Ctx) -> Script {
    shared::merchants_inn::f_innmaid(ctx, args!["[Receptionist]", "Hugel Inn", "hu_in01", 263, 95])?;
    ctx.warp("hu_in01", 267, 5)?;
    ctx.end()
}

pub fn inn_master_receptionist(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Inn Master",
        args!["Good day~", "Welcome to the", "most comfortable", "inn here in Veins~"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Save", "Take a Rest -> 5000 zeny", "Quit"])? {
        0 => {
            ctx.lines_as(
                "Inn Master",
                args!["Your Respawn Point", "has been saved in Veins.", "Enjoy your stay in town~"],
            )?;
            ctx.call(Function::SavePoint, args!["ve_in", 157, 209, 1, 1])?;
            return ctx.close();
        }
        1 => {
            ctx.mes("[Inn Master]")?;
            if ctx.player().zeny()? < 5000 {
                ctx.lines(args![
                    "I'm sorry, but I don't",
                    "think you have enough",
                    "money to check in. The",
                    "service charge is 5,000 zeny."
                ])?;
                return ctx.close();
            }
            ctx.mes("Enjoy your stay~")?;
            ctx.close_window()?;
            ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
            ctx.call(Function::PercentHeal, args![100, 100])?;
            ctx.warp("ve_in", 184, 228)?;
            return ctx.end();
        }
        2 => {
            ctx.lines_as("Inn Master", args!["Please come again."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn hotel_keeper_bra1(ctx: &Ctx) -> Script {
    ctx.lines_as("Hotel Keeper", args!["Welcome to the beautiful Brasilis Hotel."])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Save:Rest -5000 zeny:Cancel")])?);
        let mut matched1 = false;
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Hotel Keeper", args!["Do you want to save here at the Brasilis Hotel?"])?;
            ctx.next()?;
            let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("No thank you.:Absolutely.")])?);
            if subject2.loosely_equals(&Val::from(1)) {
                ctx.lines_as("Hotel Keeper", args!["Ok then, enjoy your stay."])?;
                return ctx.close();
            } else if subject2.loosely_equals(&Val::from(2)) {
                ctx.lines_as(
                    "Hotel Keeper",
                    args!["Your respawn has been saved here at the hotel. I hope that you enjoy your stay here in Brasilis."],
                )?;
                ctx.call(Function::SavePoint, args!["bra_in01", 144, 69, 1, 1])?;
                return ctx.close();
            }
            return ctx.end();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.player().zeny()? > 4999 {
                ctx.lines_as("Hotel Keeper", args!["I will show you a great room."])?;
                ctx.close_window()?;
                if ctx.player().zeny()? < 5000 {
                    return ctx.end();
                }
                ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
                ctx.call(Function::PercentHeal, args![100, 100])?;
                ctx.warp("bra_in01", 144, 69)?;
                return ctx.end();
            }
            ctx.lines_as(
                "Hotel Keeper",
                args!["I'm sorry, but the service charge is 5,000 zeny per night."],
            )?;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            return ctx.close();
        }
    }
    Ok(())
}

pub fn hotel_employee_ein(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Hotel Employee",
        args![
            "Good day, welcome to the",
            "Einbroch Hotel. The staff is",
            "always striving to accommodate",
            "our guests with the highest",
            "standards in cleanliness,",
            "service and convenience~"
        ],
    )?;
    ctx.next()?;
    if ctx.var("kain_ticket").get()? == 5 {
        ctx.lines_as(
            "Hotel Employee",
            args![
                "Are you looking",
                "for Mr. Defru Ark?",
                "Oh right, he did mention",
                "waiting for some package",
                "from the Airport. Now let",
                "me pull up that information..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hotel Employee",
            args!["Ah, here we are.", "Mr. Defru Ark is", "staying in Room 201."],
        )?;
        ctx.next()?;
    } else {
        match ctx.menu(&["Save", "Take Rest - 5,000 zeny", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Hotel Employee",
                    args![
                        "Your Respawn Point",
                        "has been saved here",
                        "in the Einbroch Hotel.",
                        "Thank you, and please",
                        "come again."
                    ],
                )?;
                ctx.close_window()?;
                ctx.call(Function::SavePoint, args!["ein_in01", 200, 224, 1, 1])?;
                return ctx.end();
            }
            1 => {
                if ctx.player().zeny()? > 4999 {
                    ctx.lines_as("Hotel Employee", args!["Thank you.", "Please enjoy", "your rest~"])?;
                    ctx.close_window()?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
                    ctx.call(Function::PercentHeal, args![100, 100])?;
                    ctx.warp("ein_in01", 272, 167)?;
                    return ctx.end();
                }
                ctx.lines_as(
                    "Hotel Employee",
                    args![
                        "I'm sorry, but the",
                        "accommodation fee is",
                        "5,000 zeny. Next time,",
                        "please make sure that you",
                        "bring enough zeny, okay?"
                    ],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as("Hotel Employee", args!["Thank you and", "please come again~"])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.lines_as("Hotel Employee", args!["Have a good day."])?;
    ctx.close()
}
