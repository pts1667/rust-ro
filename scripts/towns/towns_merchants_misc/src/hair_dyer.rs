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

pub fn hair_dyer_lich(ctx: &Ctx) -> Script {
    let mut choose_success = 0;
    let mut headpalette = Val::from(0);
    ctx.mes("[Rossa]")?;
    if ctx.var("Sex").get()? == constants::SEX_MALE {
        ctx.lines(args![
            "Welcome, come in~",
            "Oh, I see that you take",
            "much better care of your",
            "hair than those other boys.",
            "Now would you like to dye",
            "your hair another color?"
        ])?;
    } else {
        ctx.lines(args![
            "Oh, wow~ Where did",
            "you get your hair styled?",
            "I love it! But... It would",
            "be even more beautiful if",
            "you dyed your hair. What",
            "do you think about that?"
        ])?;
    }
    while choose_success != 2 {
        ctx.next()?;
        match ctx.menu(&["Dye Hair", "Coloring Information", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Rossa",
                    args![
                        "Ho ho ho ho~",
                        "So which color would",
                        "you like to try? Something",
                        "vivid or dark? Sexy or cute?"
                    ],
                )?;
                ctx.next()?;
                while choose_success != 2 {
                    if choose_success == 1 {
                        ctx.lines_as(
                            "Rossa",
                            args!["Ooh, I like this color!", "But would you like to", "try a different one?"],
                        )?;
                        ctx.next()?;
                        match ctx.menu(&["Yes", "No"])? {
                            0 => {
                                ctx.lines_as("Rossa", args!["Please select", "another color~"])?;
                                ctx.next()?;
                            }
                            1 => {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "An excellent choice~",
                                        "Alright then, thank you",
                                        "for using my service and",
                                        "I hope you come by again!"
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                            }
                            _ => {}
                        }
                    }
                    while choose_success != 2 {
                        match ctx.menu(&[
                            "Red, please.",
                            "Yellow, please.",
                            "Purple, please.",
                            "Orange, please.",
                            "Green, please.",
                            "Blue, please.",
                            "White, please.",
                            "Dark Brown, please.",
                            "I like my hair color.",
                        ])? {
                            0 => headpalette = Val::from(8),
                            1 => headpalette = Val::from(1),
                            2 => headpalette = Val::from(2),
                            3 => headpalette = Val::from(3),
                            4 => headpalette = Val::from(4),
                            5 => headpalette = Val::from(5),
                            6 => headpalette = Val::from(6),
                            7 => headpalette = Val::from(7),
                            8 => {
                                if choose_success != 0 {
                                    ctx.lines_as(
                                        "Rossa",
                                        args![
                                            "Are you sure?",
                                            "Alright then, you",
                                            "know what's best for",
                                            "your beauty and to tell",
                                            "the truth, I agree with you~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                } else {
                                    ctx.lines_as(
                                        "Rossa",
                                        args![
                                            "Oh, I see. Still, I can't",
                                            "help but feel so disappointed.",
                                            "You'd look so good if you dyed",
                                            "your hair a different color~"
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                }
                                choose_success = 2;
                            }
                            _ => {}
                        }
                        if ctx
                            .call(Function::GetLook, args![constants::VAR_HEADPALETTE])?
                            .loosely_equals(&headpalette)
                        {
                            ctx.lines_as(
                                "Rossa",
                                args![
                                    "Hmm, your hair color",
                                    "is still fine, so there's",
                                    "no need to dye it the same",
                                    "color again, if that's what",
                                    "you're worried about."
                                ],
                            )?;
                            ctx.next()?;
                        } else {
                            if headpalette == 8 && ctx.call(Function::CountItem, args![975])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Scarlet Dyestuffs with you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 1 && ctx.call(Function::CountItem, args![976])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Lemon Dyestuffs with you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 2 && ctx.call(Function::CountItem, args![981])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Violet Dyestuffs with you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 3 && ctx.call(Function::CountItem, args![980])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Orange Dyestuffs with you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 4 && ctx.call(Function::CountItem, args![979])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Darkgreen Dyestuffs",
                                        "with you. Would you come",
                                        "back after you get some?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 5 && ctx.call(Function::CountItem, args![978])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Cobaltblue Dyestuffs",
                                        "with you. Would you come",
                                        "back after you get some?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 6 && ctx.call(Function::CountItem, args![982])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "White Dyestuffs with you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            } else if headpalette == 7 && ctx.call(Function::CountItem, args![983])? == 0 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm sorry dear,",
                                        "but I can't dye your",
                                        "hair if you didn't bring",
                                        "Black Dyestuffs with you..."
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            }
                            if ctx.player().zeny()? < 1000 {
                                ctx.lines_as(
                                    "Rossa",
                                    args![
                                        "Oh, I'm so sorry dear,",
                                        "but my service fee is",
                                        "1,000 zeny. Did you forget",
                                        "to bring your money with you?"
                                    ],
                                )?;
                                ctx.close_window()?;
                                choose_success = 2;
                                break;
                            }
                            match headpalette.number()? {
                                8 => {
                                    ctx.items().take(975, 1)?;
                                }
                                1 => {
                                    ctx.items().take(976, 1)?;
                                }
                                2 => {
                                    ctx.items().take(981, 1)?;
                                }
                                3 => {
                                    ctx.items().take(980, 1)?;
                                }
                                4 => {
                                    ctx.items().take(979, 1)?;
                                }
                                5 => {
                                    ctx.items().take(978, 1)?;
                                }
                                6 => {
                                    ctx.items().take(982, 1)?;
                                }
                                7 => {
                                    ctx.items().take(983, 1)?;
                                }
                                _ => {}
                            }
                            ctx.player().set_zeny(ctx.player().zeny()? - 1000)?;
                            ctx.call(Function::SetLook, args![constants::VAR_HEADPALETTE, headpalette.clone()])?;
                            choose_success = 1;
                            break;
                        }
                    }
                }
            }
            1 => {
                ctx.lines_as(
                    "Rossa",
                    args![
                        "When you're feeling",
                        "down, when you just want",
                        "to look nice for the one you",
                        "love, or when you just want",
                        "a different look, why don't",
                        "you dye your hair?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Rossa",
                    args![
                        "All you need is one",
                        "Dyestuffs item of the",
                        "color that you want to",
                        "dye your hair, as well as",
                        "a 1,000 zeny service fee.",
                        "I'm here for your beauty needs~"
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Rossa",
                    args![
                        "You know, when you",
                        "put some effort into",
                        "your appearance, you'll",
                        "not only look better, but",
                        "you'll feel better about",
                        "yourself. Makes sense, right?"
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    }
    ctx.close()
}
