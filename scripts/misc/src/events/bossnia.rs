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

pub fn bossnia_staff_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Riss",
        args![
            "Hello?",
            "I found some beautiful places",
            "while I travelled all over the world.",
            "I am an adventurer.",
            "Haha~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args![
            "What? My name is...",
            "'Bossnia Staff'?? No no...",
            "Well, my name is not so important.",
            "Sometimes you should be generous.",
            "No time for considering that kind of small stuff",
            "when you have to concentrate on more important things."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args![
            "Hum... anyway I want to say... something..",
            "While I was travelling through some places,",
            "I found a really fearful place."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args![
            "Most of the time when you come to a place,",
            "there is one strong and fearful monster.",
            "Isn't it?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args![
            "But... but... in there...",
            "There are lots of fearful and strong monsters in there...",
            "That was really frightful."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args!["If I had reacted a bit later... a few seconds...", "I might have been killed."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args![
            "What?",
            "You want to go in there?",
            "Oh~ Boy~ you didn't get me.",
            "In there......."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Riss",
        args![
            "Uh... you already know?",
            "Although you know the place, you want to go in...",
            "Good, I will send you there.",
            "But after you went there, don't hold"
        ],
    )?;
    if constants::VIP_SCRIPT != 0 {
        ctx.mes("a grudge against me. It'll cost you 1 Reset Stone for 5 access.")?;
        ctx.next()?;
        ctx.mes("[Riss]")?;
        let uses_access = if ctx.var("bossnia_event").get()?.number()? > 0 {
            ctx.lines(args![(Val::from("Remaining access: ") + ctx.var("bossnia_event").get()?)])?;
            true
        } else if ctx.call(Function::CountItem, args![6320])?.is_true() {
            ctx.mes("Do you want to go?")?;
            false
        } else {
            ctx.lines(args![
                "You don't have a ticket now.....",
                "So come to me again with a Reset Stone later."
            ])?;
            return ctx.close();
        };
        ctx.next()?;
        if runtime::select_values(ctx, &[Val::from("Enter:Leave")])? == 2 {
            return ctx.close();
        }
        let course = Val::from(runtime::select_values(ctx, &[Val::from("First:Second:Third:Fourth")])?);
        let (warp_x, warp_y) = match runtime::select_values(ctx, &[Val::from("Warp 1:Warp 2:Warp 3:Warp 4")])? {
            1 => (31, 208),
            2 => (31, 31),
            3 => (208, 31),
            4 => (208, 208),
            _ => (0, 0),
        };
        if uses_access {
            ctx.var("bossnia_event")
                .set(Val::from(ctx.var("bossnia_event").get()?.number()? - 1))?;
        } else {
            ctx.items().take(6320, 1)?;
            ctx.var("bossnia_event").set(Val::from(4))?;
        }
        ctx.fx().special_effect(constants::EF_MAXPOWER)?;
        ctx.call(Function::Warp, args![Val::from("bossnia_0") + course, warp_x, warp_y])?;
        ctx.close()
    } else {
        ctx.mes("a grudge against me. Also it costs 5,000 zeny.")?;
        ctx.next()?;
        if ctx.player().zeny()? > 4999 {
            ctx.lines_as(
                "Riss",
                args!["Would you really like to take the challenge?", "Ok, just choose the course."],
            )?;
            ctx.next()?;
            let course = Val::from(runtime::select_values(ctx, &[Val::from("First:Second:Third:Fourth")])?);
            ctx.lines_as("Riss", args!["Take care, boy~", "Don't hold a grudge against me."])?;
            ctx.close_window()?;
            ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
            ctx.call(
                Function::Warp,
                args![
                    Val::from("bossnia_0") + course,
                    ctx.call(Function::Rand, args![202, 204])?,
                    ctx.call(Function::Rand, args![202, 204])?
                ],
            )?;
            ctx.end()
        } else {
            ctx.lines_as(
                "Riss",
                args!["You don't have enough money...", "Come back when you have at least 5,000 zeny."],
            )?;
            ctx.close()
        }
    }
}
