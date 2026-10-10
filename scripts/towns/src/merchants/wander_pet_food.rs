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

pub fn pet_enthusiast(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Pet Enthusiast Jenny",
        args!["Oh, Hi there!", "Are you a lover of animals like I am?"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pet Enthusiast Jenny",
        args![
            "I know that it's really hard to keep your cute pets happy.",
            "All it takes is the right kind of food."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pet Enthusiast Jenny",
        args!["I know that sometimes it is really hard to get the right food for your pet."],
    )?;
    ctx.next()?;
    ctx.lines_as("Pet Enthusiast Jenny", args!["Do you have any food that you are looking for?"])?;
    ctx.next()?;
    if ctx.menu(&["Yes", "No"])? == 1 {
        return ctx.close();
    }
    if ctx.items().count(7158)? > 19 && ctx.items().count(970)? > 0 {
        ctx.lines_as(
            "Pet Enthusiast Jenny",
            args![
                "Oh, you have ^ff000020 Broken Liquor Jar^000000s and ^ff00001 Alcohol^000000!",
                "Do you want to exchange them for",
                "^ff000020 Spirit Liquor^000000 for your Wanderer pet?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes, please!", "No thank you."])? == 0 {
            ctx.lines_as(
                "Pet Enthusiast Jenny",
                args!["Ok here you go then.", "I hope it keeps your", "cute pet happy."],
            )?;
            ctx.items().take(7158, 20)?;
            ctx.items().take(970, 1)?;
            ctx.items().give(7824, 20)?;
            return ctx.close();
        }
        ctx.lines_as(
            "Pet Enthusiast Jenny",
            args!["Hehe, well just let me know if you change your mind."],
        )?;
        ctx.close()
    } else {
        ctx.lines_as(
            "Pet Enthusiast Jenny",
            args!["Actually, right now I can help you get Spirit Liquor for Wanderer pets."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pet Enthusiast Jenny",
            args![
                "All you have to do is bring me ^FF0000 20 Broken Liquor Jars^000000 and ^FF0000 1 Alcohol^000000.",
                "I can give you ^FF0000 20 Spirit Liquor^000000 for that."
            ],
        )?;
        ctx.next()?;
        ctx.mes("You can get the Broken Liquor Jars from Tengu monsters in Amatsu dungeon.")?;
        ctx.close()
    }
}

pub fn berry_toe(ctx: &Ctx) -> Script {
    ctx.call(Function::NpcSpecialEffect, args![constants::EF_CHANGEDARK])?;
    ctx.end()
}
