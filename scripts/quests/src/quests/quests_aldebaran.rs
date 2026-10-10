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

pub fn trader_01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Trader",
        args!["Who is this mysterious man?", "I, the enigmatic and debonair 'Trader?'"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Trader",
        args!["Traveling about the Midgard Continent, with all of his wonderful hats? Nobody knows..."],
    )?;
    ctx.next()?;
    ctx.lines_as("Trader", args!["For what purpose do I collect and trade these hats from all around the world? Choose a hat, get closer to unraveling the mystery..."])?;
    ctx.next()?;
    ctx.lines(args![
        " (1) ^3355FFDoctor Band^000000:",
        "1 Red Bandana + 50 Irons + 1 Cracked Diamond + 3500 Zeny",
        " (2)^3355FFFeather Bonnet^000000:",
        "1 Romantic Gent + 300 Feather of Birds + 500 Zeny",
        " (3) ^3355FFPhantom of Opera^000000:",
        "20 Iron + 1 Singing Plant + 5000 Zeny ",
        " (4) ^3355FFSakkat^000000:",
        "120 Trunks + 10000 Zeny "
    ])?;
    ctx.next()?;
    match ctx.menu(&[" Doctor Band ", " Feather Bonnet ", " Phantom of Opera ", " Sakkat "])? {
        0 => {
            if ctx.items().count(2275)? > 0 && ctx.items().count(998)? > 49 && ctx.items().count(733)? > 0 && ctx.player().zeny()? > 3499 {
                ctx.items().take(2275, 1)?;
                ctx.items().take(998, 50)?;
                ctx.items().take(733, 1)?;
                ctx.player().set_zeny(ctx.player().zeny()? - 3500)?;
                ctx.lines_as("Trader", args!["Hm! You don't have a medical license, do you? It's alright, I've heard about a rogue, unlicensed physician who performed medical miracles! But... That might have been a comic book."])?;
                ctx.next()?;
                ctx.lines_as("Trader", args!["Oh whatever. Just don't get caught."])?;
                ctx.items().give(2273, 1)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Trader",
                args![
                    "You guy~",
                    "Check the requirements again.",
                    "You don't look like an idiot though. So c'mon man, get real."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            if ctx.items().count(2247)? > 0 && ctx.items().count(916)? > 299 && ctx.player().zeny()? > 499 {
                ctx.items().take(2247, 1)?;
                ctx.items().take(916, 300)?;
                ctx.player().set_zeny(ctx.player().zeny()? - 500)?;
                ctx.lines_as("Trader", args!["Ooh~! You have good fashion sense. I know you've had a hard time collecting this stuff, but this hat is worth it. Take it. All you need now is a fur coat and a cane!"])?;
                ctx.items().give(5018, 1)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Trader",
                args![
                    "You guy~",
                    "Go check my requirements again. You don't look like an idiot though. C'mon man, get real."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            if ctx.items().count(998)? > 19 && ctx.items().count(707)? > 0 && ctx.player().zeny()? > 4999 {
                ctx.items().take(998, 20)?;
                ctx.items().take(707, 1)?;
                ctx.player().set_zeny(ctx.player().zeny()? - 5000)?;
                ctx.lines_as("Trader", args!["This one? It's a little bit scary, though it has some sort of romantic quality. What do you think? You like it? Alright, take it, it's yours!"])?;
                ctx.items().give(2281, 1)?;
                return ctx.close();
            }
            ctx.lines_as("Trader", args!["Buffoon. Go check the requirements again. ^3355FFPhantom of Opera^000000 isn't easy to come by. So c'mon man, get real."])?;
            return ctx.close();
        }
        3 => {
            if ctx.items().count(1019)? > 119 && ctx.player().zeny()? > 9999 {
                ctx.items().take(1019, 120)?;
                ctx.player().set_zeny(ctx.player().zeny()? - 10000)?;
                ctx.lines_as(
                    "Trader",
                    args!["If you have a chance to visit the Uplander Village, Payon, please go and meet the Sakkat Craftsman."],
                )?;
                ctx.next()?;
                ctx.lines_as("Trader", args!["He's never sold Sakkat to Traders other than me, since only I can recognize its quality. Due to its rarity, Sakkat has become a very unique and exceptional product. Okay! Take it, it's yours!"])?;
                ctx.items().give(2280, 1)?;
                return ctx.close();
            }
            ctx.lines_as(
                "Trader",
                args!["MORON~ Check my requirements again. C'mon man, you don't look like an idiot, so get real~"],
            )?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}
