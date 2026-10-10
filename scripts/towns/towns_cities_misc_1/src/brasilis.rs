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

pub fn crewman_bra2(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Crewman",
        args![
            "Hey, have you heard of a place called Brasilis?",
            "It's a tropical city that's hot like the desert but also rainy. It is a very mysterious place."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Crewman", args!["We recently found a new ocean route to get there easily."])?;
    let cost;
    if constants::VIP_SCRIPT != 0 {
        cost = if ctx.call(Function::VipStatus, args![constants::VIP_STATUS_ACTIVE])?.is_true() {
            1000
        } else {
            10000
        };
        ctx.mes("It's just 10,000 zeny for a round trip, and 1,000 for VIP! So do you want to go?")?;
    } else {
        cost = 10000;
        ctx.mes("It's just 10,000 zeny for a round trip! So do you want to go?")?;
    }
    ctx.next()?;
    match ctx.menu(&["Take me to Brasilis!", "I'll stay here."])? {
        0 => {
            if ctx.player().zeny()? < cost {
                ctx.lines_as(
                    "Crewman",
                    args![Val::from("I said ") + shared::other_global_functions::f_insertcomma(ctx, args![cost])? + Val::from(" zeny.")],
                )?;
                return ctx.close();
            }
            ctx.lines_as("Crewman", args!["Cool~!! Let's go~!"])?;
            ctx.player().set_zeny(ctx.player().zeny()? - cost)?;
            ctx.close_window()?;
            ctx.warp("brasilis", 314, 60)?;
            return ctx.end();
        }
        1 => {
            ctx.lines_as(
                "Crewman",
                args!["Well if you're ever interested, let me know and I can take you there."],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn crewman_bra1(ctx: &Ctx) -> Script {
    ctx.lines_as("Crewman", args!["My ship is going to back to Alberta, do you want to join us?"])?;
    ctx.next()?;
    match ctx.menu(&["Go back to Alberta.", "Not yet~."])? {
        0 => {
            ctx.lines_as("Crewman", args!["I sure do miss home."])?;
            ctx.close_window()?;
            ctx.warp("alberta", 244, 115)?;
            return ctx.end();
        }
        1 => {
            ctx.lines_as("Crewman", args!["Ok, suit yourself. We'll see you when we get back then."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn signpost_bra1(ctx: &Ctx) -> Script {
    ctx.mes(":: Art Museum ::")?;
    return ctx.close();
}

pub fn signpost_bra2(ctx: &Ctx) -> Script {
    ctx.mes(":: Verass Monument ::")?;
    return ctx.close();
}

pub fn signpost_bra3(ctx: &Ctx) -> Script {
    ctx.lines(args![":: Market ::", " ", "- For your Potions and Weaponry -"])?;
    return ctx.close();
}

pub fn signpost_bra4(ctx: &Ctx) -> Script {
    ctx.lines(args![":: Jungle Cable ::", "- Not for the faint of heart -"])?;
    return ctx.close();
}

pub fn signpost_bra5(ctx: &Ctx) -> Script {
    ctx.mes(":: Brasilis Hotel ::")?;
    return ctx.close();
}

pub fn ice_cream_maker(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Ice Cream Maker",
        args![
            "Come~come~",
            "Ice cream is the perfect snack for a hot day~",
            "It's just ^3355FF100 Zeny^000000~",
            "Ice Cream~",
            "Get 'yer Ice Cream!"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Give me one!", "Ice Cream?", "Cancel."])? {
        0 => {
            ctx.lines_as(
                "Ice Cream Maker",
                args![
                    "Since there are so many people want to get a cool ice cream you can order only 5 at a time.",
                    "So how many d'ya want?"
                ],
            )?;
            ctx.next()?;
            let quantity = loop {
                let (input, _) = runtime::input_number(ctx, None, None)?;
                let requested = input.number()?;
                if requested == 0 {
                    ctx.lines_as(
                        "Ice Cream Maker",
                        args!["None?", "Fine get outta the way, I have customers to serve."],
                    )?;
                    return ctx.close();
                }
                if requested < 0 || requested > 5 {
                    ctx.lines_as(
                        "Ice Cream Maker",
                        args![
                            "Wow.",
                            "You ordered too much.",
                            "If you eat over 5 you might need to fight with a monster in your stomach. Calm down buddy."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    break requested;
                }
            };
            let price = quantity * 100;
            if ctx.player().zeny()? < price {
                ctx.lines_as(
                    "Ice Cream Maker",
                    args![
                        "Dood~! You don't have enough money.",
                        "It's only ^3355FF100 Zeny^000000~ Seriously!"
                    ],
                )?;
                return ctx.close();
            }
            if !ctx.call(Function::CheckWeight, args![536, quantity])?.is_true() {
                ctx.lines_as(
                    "Ice Cream Maker",
                    args!["You seem to have too much stuff.", "Lighten your pack before buying this."],
                )?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - price)?;
            ctx.call(Function::GetItem, args![536, quantity])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Ice Cream Maker",
                args![
                    "'Ice cream is...",
                    "Wait, don't you know",
                    "what Ice Cream is?",
                    "What rock have you",
                    "been living under?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Ice Cream Maker",
                args![
                    "I'm not going to even start with how weird that sounds.",
                    "Anyway, get 'yer Ice Cream right here while it's nice and cold."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Ice Cream Maker",
                args!["Don't miss your chance to eat the greatest Ice Cream in all the land~!"],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
