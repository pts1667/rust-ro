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

pub fn pandit_chacha_child07(ctx: &Ctx) -> Script {
    if ctx.player().base_level()? > 50 {
        ctx.lines_as(
            "Pandit chacha",
            args![
                "Hahaha~",
                "It is special day, comes only one time in a year.",
                "If you see little adventurers around you send to me~",
                "I will give the special gift."
            ],
        )?;
        return ctx.close();
    }
    if ctx.player().base_level()? < 20 {
        ctx.lines_as(
            "Pandit chacha",
            args![
                "Hahaha~",
                "You are a baby adventurer not little adventurer.",
                "When you more grow up, come back again. hahaha."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("oversea_event9").get()?.number()? < 1 {
        ctx.lines_as(
            "Pandit chacha",
            args![
                "Hahaha~",
                "Welcome, little adventurers!",
                "Today is special day, isn't it~?",
                "I would like to give the small gift to little adventurer...",
                "What about you? Do you want to take it?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["No. I will take it later.", "Sure, I want."])? == 0 {
            ctx.lines_as(
                "Pandit chacha",
                args![
                    "That's too bad.... hum...",
                    "I gathered some stuffs from far a way world to make it....",
                    "Whenever come back again if you want it..."
                ],
            )?;
            return ctx.close();
        }
        ctx.var("oversea_event9").set(Val::from(1))?;
        ctx.items().give(11705, 10)?;
        ctx.lines_as(
            "Pandit chacha",
            args![
                "Look. This is a child Potion.",
                "The weight is just 1 but recover much HP.",
                "If you want to get more, bring the 1 Wedding Bouquet and 1 Witherless Rose."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("oversea_event9").get()?.number()? == 1 {
        if ctx.items().count(745)? > 0 && ctx.items().count(748)? > 0 {
            ctx.lines_as(
                "Pandit chacha",
                args![
                    "Ahha!!",
                    "You have remembered my beautiful composition.",
                    "You did good work.",
                    "Could you give me 1 Wedding Bouquet and 1 Witherless Rose? "
                ],
            )?;
            ctx.next()?;
            if ctx.menu(&["Not yet.", "Sure, take it."])? == 0 {
                ctx.lines_as("Pandit chacha", args!["If you are not prepared yet, call me when you ready."])?;
                return ctx.close();
            }
            ctx.lines_as(
                "Pandit chacha",
                args![
                    "Hahaha~",
                    "Oh~ you get it.",
                    "Here, I will exchange to the 50 child potion.",
                    "Once you get this 50 child potion, I won't give any more."
                ],
            )?;
            ctx.close_window()?;
            ctx.items().take(745, 1)?;
            ctx.items().take(748, 1)?;
            ctx.var("oversea_event9").set(Val::from(2))?;
            ctx.items().give(11705, 50)?;
            return ctx.end();
        }
        ctx.lines_as(
            "Pandit chacha",
            args![
                "Little adventurers, you should bring the 1 Wedding Bouquet and 1 Witherless Rose.",
                "If you bring these stuffs I will exchange them for child Potion. "
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Pandit chacha", args!["Hahaha~", "Are you enjoying children week~?"])?;
    ctx.close()
}
