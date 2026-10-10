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

pub fn anxious_leprechaun_8pday(ctx: &Ctx) -> Script {
    if ctx.var("stpatrick2008").get()?.number()? < 1 {
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Hmm...",
                "Tis a fine day it be.",
                "If you have a moment to spare.",
                "Come here to me now, come here and I'll tell ya something."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Go ahead.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["These past years I've come to see the world and each time the snakes have stolen me treasure."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Oh?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["Ye fine folks of this land have been so gracious to recover me treasure again and again."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["I've learned me lesson and will never forget how those vile snakes have wronged me."],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What did you do?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "I made a safe place to hide me treasure.",
                "Hidden in a secret place the treasure would be safe until I returned to the world the following year."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "This year, I have made me journey to see the outside world once again.",
                "I traveled to me cache of gold I found it to be safe and undisturbed."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "I thought to meself, I am very clever to hide the treasure from the snakes.",
                "Oh, but I was a fool still."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("What happened?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "My hiding place was too clever.",
                "I pulled and I heaved and pushed, but the hiding place would not budge.",
                "So once again, I am without treasure this year."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "I have not even begun brewing me famous green ale on account of this mess.",
                "I will ask ye, will ya help get me treasure back?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Sure, I can help you.", "I'm too busy."])? == 1 {
            ctx.lines_as(
                "O'Riley the Leprechaun",
                args!["Aye, I understand.", "Thank you for listening to me tale."],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["To find my treasure it be true.", "To my treasure I give my secret to you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "So, listen well and make no omissions.",
                "Make the journey to the city of magicians.",
                "Over the bridge and across the water",
                "Climb the mountain until north ye can travel no farther."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Passed the stone steps set into the air.",
                "Hidden in the mountain side, me treasure is there."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Ye must find a way to break it open.",
                "Ye will need a mighty explosion to free the cache to be sure."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Ye might try some Firecrackers.",
                "Course, Ye would need a great number of them, Ye would need at least ^FF0000200 Firecreackers^000000"
            ],
        )?;
        ctx.var("stpatrick2008").set(Val::from(1))?;
        return ctx.close();
    }
    if ctx.var("stpatrick2008").get()? == 1 {
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Tis a fine day it be.",
                "Have you found me treasure yet, have you?",
                "Me hiding spot is a might hard to crack."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Ye might try some Firecrackers.",
                "Course, Ye would need a great number of them, Ye would need at least ^FF0000200 Firecreackers^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["I be understanding if you can't get to it to be sure to be sure."],
        )?;
        return ctx.close();
    }
    if ctx.var("stpatrick2008").get()? == 2 {
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Oh, welcome back!",
                "Thank you for returning me treasure!",
                "Me gratitude knows no bounds!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["I am so happy,", "I'll be starting up me brew right away"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["It seems that the snakes have returned this year and are here to steal my treasure again."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "The snakes carry the coins of my kinsmen and must be punished.",
                "But ye have had a long journey, for which I am very grateful."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["When ye have had a moment to rest ye legs, come talk to me again and we may speak again"],
        )?;
        ctx.close_window()?;
        ctx.var("stpatrick2008").set(Val::from(3))?;
        ctx.call(Function::GetExperience, args![200000, 70000])?;
        ctx.items().take(7721, 1)?;
        return ctx.end();
    }
    if ctx.var("stpatrick2008").get()? == 3 {
        ctx.lines_as("O'Riley the Leprechaun", args!["Ah, well rested I hope?"])?;
        ctx.next()?;
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args![
                "Thank you for coming by again.",
                "The snakes carry the coins of my kindsmen and must be punished."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("O'Riley the Leprechaun", args!["I would be so generous to give you some of my famous brew if you would bring me the ill-gotten gains carried by those vile snakes"])?;
        ctx.next()?;
        ctx.mes("And if you find one of the treasures of my kin please bring me those coins so that the snakes don't get them.")?;
        ctx.var("stpatrick2008").set(Val::from(4))?;
        return ctx.close();
    }
    if ctx.var("stpatrick2008").get()? == 4 {
        ctx.lines_as(
            "O'Riley the Leprechaun",
            args!["The snakes carry too many of me kinsmen's coin I be passing out pints of me brew as reward for their return."],
        )?;
        ctx.next()?;
        ctx.lines_as("O'Riley the Leprechaun", args!["I'll reward ye with one jug of ale for ^0000FF1 Golden Coins^000000, ^0000FF5 Silver Coins^000000, or ^0000FF10 Bronze Coins^000000.", "It be a fair bounty for the ill-gotten coins.", "So, what kind of coin have ye brought?"])?;
        ctx.next()?;
        match ctx.menu(&["Bronze Coins", "Silver Coins", "Gold Coins", "Quit."])? {
            0 => {
                ctx.lines_as("O'Riley the Leprechaun", args!["Thank you. Here's your ale~"])?;
                if ctx.items().count(7915)? >= 10 {
                    ctx.items().give(12135, 1)?;
                    ctx.items().take(7915, 10)?;
                }
                return ctx.close();
            }
            1 => {
                ctx.lines_as("O'Riley the Leprechaun", args!["Thank you. Here's your ale~"])?;
                if ctx.items().count(7916)? >= 5 {
                    ctx.items().give(12135, 1)?;
                    ctx.items().take(7916, 5)?;
                }
                return ctx.close();
            }
            2 => {
                ctx.lines_as("O'Riley the Leprechaun", args!["Thank you. Here's your ale~"])?;
                if ctx.items().count(7720)? >= 1 {
                    ctx.items().give(12135, 1)?;
                    ctx.items().take(7720, 1)?;
                }
                return ctx.close();
            }
            3 => {
                ctx.lines_as("O'Riley the Leprechaun", args!["I'll reward ye with one jug of ale for ^0000FF1 Golden Coins^000000, ^0000FF5 Silver Coins^000000, or ^0000FF10 Bronze Coins^000000."])?;
                return ctx.close();
            }
            _ => {}
        }
    }
    Ok(())
}

pub fn rocks_08stpattysday(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn rocks_08stpattysday_ontouch(ctx: &Ctx) -> Script {
    if ctx.var("stpatrick2008").get()?.number()? < 1 {
        ctx.mes("- You've found a pile of rocks covered with soil. -")?;
        return ctx.close();
    }
    if ctx.var("stpatrick2008").get()? == 1 {
        ctx.mes("- You've found a pile of rocks covered with soil. -")?;
        ctx.next()?;
        if ctx.items().count(12018)? < 200 {
            ctx.lines(args!["The rocks won't budge.", "Maybe O'Riley knows a way to move the rocks."])?;
            return ctx.close();
        }
        if ctx.menu(&["Use Firecrackers.", "Ignore"])? == 1 {
            return ctx.close();
        }
        ctx.lines(args![
            "You buried 200 Firecrackers under the pile of rocks.",
            "You light the fuse."
        ])?;
        ctx.next()?;
        ctx.mes("*BOOM!*")?;
        ctx.fx().special_effect(constants::EF_LORD)?;
        ctx.next()?;
        ctx.lines(args![
            "After A cloud of dust and smoke has dissipated,",
            "You've found a box between the rocks and soil.",
            "This box must contain O'Riley's valuables.",
            "Let's bring the box to O'Riley."
        ])?;
        ctx.close_window()?;
        ctx.var("stpatrick2008").set(Val::from(2))?;
        ctx.items().take(12018, 200)?;
        ctx.items().give(7721, 1)?;
    }
    ctx.end()
}
