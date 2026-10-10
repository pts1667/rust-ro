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

pub fn rice_mill_grandma_rat(ctx: &Ctx) -> Script {
    ctx.mes("[Rice Mill Grandma]")?;
    if !ctx.var("lunar_rat").get()?.is_true() {
        ctx.lines(args![
            "Those filthy little",
            "creatures! Scurrying",
            "around, snatching things",
            "from me! If only they",
            "weren't so blamed quick..."
        ])?;
        ctx.next()?;
        ctx.var("@menu").set(runtime::select_values(ctx, &[Val::from("What's wrong?")])?)?;
        ctx.lines_as(
            "Rice Mill Grandma",
            args![
                "Oh, I was just making",
                "some rice cakes and",
                "pastries to celebrate",
                "the new year, but these",
                "animals have been stealing",
                "the Rice Pouches I've prepared."
            ],
        )?;
        ctx.next()?;
        ctx.var("@menu").set(runtime::select_values(ctx, &[Val::from("Rice Pouches?")])?)?;
        ctx.lines_as(
            "Rice Mill Grandma",
            args![
                "Yes, my son needs those",
                "Rice Pouches to pound the",
                "rice in a mortar, but I can't",
                "make any pastries if I don't",
                "even have the rice. Do you",
                "think you can help me?"
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Of course.", "I don't even know you."])? == 0 {
            ctx.var("lunar_rat").set(Val::from(1))?;
            ctx.lines_as(ctx.player().name()?, args!["Of course.", "What can I do?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Rice Mill Grandma",
                args![
                    "Well, do you think you",
                    "can catch the animals that",
                    "stole my Rice Pouches?",
                    "They're these blue and",
                    "white rats and these",
                    "nasty little moles."
                ],
            )?;
            ctx.next()?;
            ctx.var("@menu").set(runtime::select_values(ctx, &[Val::from("Not a problem.")])?)?;
            ctx.lines_as(
                ctx.player().name()?,
                args![
                    "Not a problem.",
                    "I'll catch those animals,",
                    "and bring back any Rice",
                    "Pouches that I can find."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Rice Mill Grandma",
                args!["Oh, thank you!", "Good luck catching", "those pests for me~"],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Rice Mill Grandma",
            args![
                "I... Well...",
                "That's true, but I was",
                "planning on giving you",
                "something nice in return",
                "for your help. I know you're",
                "not a bad person, so..."
            ],
        )?;
        return ctx.close();
    }
    if (ctx.var("lunar_rat").get()?.number()? >= 1 && ctx.var("lunar_rat").get()?.number()? <= 2) || ctx.var("lunar_rat").get()? == 4 {
        if ctx.items().count(7770)? >= 1 && ctx.var("lunar_rat").get()? == 2 {
            ctx.lines(args![
                "Oh, is that the",
                "Sweet Rice my son made?",
                "Would you let me have it?",
                "Please wait here a moment,",
                "and I'll make you some pastry~"
            ])?;
            ctx.items().take(7770, 1)?;
            ctx.var("lunar_rat").set(Val::from(3))?;
            return ctx.close();
        }
        if ctx.var("lunar_rat").get()? == 4 {
            if ctx.items().count(7770)? >= 1 {
                ctx.lines(args![
                    "Oh, is that the",
                    "Sweet Rice my son made?",
                    "Would you let me have it?",
                    "Please wait here a moment,",
                    "and I'll make you some pastry~"
                ])?;
                ctx.items().take(7770, 1)?;
                ctx.var("lunar_rat").set(Val::from(5))?;
                return ctx.close();
            }
            ctx.lines(args![
                "Oh, where are all the",
                "Rice Pouches? We need",
                "them to make more rice cakes..."
            ])?;
            return ctx.close();
        }
        if ctx.items().count(7869)? < 1 && ctx.var("lunar_rat").get()?.number()? <= 2 {
            ctx.lines(args![
                "Ooh, those white and",
                "blue mouses might have",
                "snatched my Rice Pouches",
                "Those moles probably took",
                "them too. Such nasty little",
                "creatures, aren't they?"
            ])?;
            return ctx.close();
        }
        ctx.lines(args![
            "Oh, is that one of my",
            "Rice Pouches? I'm sorry,",
            "but the pain my hips..",
            "Would you mind being",
            "a dear, and delivering",
            "that to my son for me?"
        ])?;
        ctx.var("lunar_rat").set(Val::from(2))?;
        return ctx.close();
    }
    if ctx.var("lunar_rat").get()? == 3 || ctx.var("lunar_rat").get()? == 5 {
        let reward = ctx.rand_range(1, 100)?;
        match reward {
            ..=5 => {
                if ctx.var("lunar_rat").get()? == 3 {
                    ctx.items().give(9038, 1)?;
                    ctx.var("lunar_rat").set(Val::from(4))?;
                } else if ctx.var("lunar_rat").get()? == 5 {
                    ctx.items().give(668, 1)?;
                }
            }
            6..=10 => ctx.items().give(12198, 2)?,
            11..=30 => ctx.items().give(12195, 3)?,
            31..=40 => ctx.items().give(12196, 2)?,
            41..=55 => ctx.items().give(12123, 2)?,
            56..=70 => ctx.items().give(12122, 2)?,
            71..=80 => ctx.items().give(12124, 2)?,
            81..=90 => ctx.items().give(12198, 2)?,
            91..=100 => ctx.items().give(12197, 3)?,
            _ => {}
        }
        if ctx.rand_range(1, 20)? <= 5 && ctx.var("lunar_rat").get()? == 4 {
            ctx.items().give(668, 1)?;
        }
        if ctx.var("lunar_rat").get()? == 3 {
            ctx.var("lunar_rat").set(Val::from(2))?;
        } else if ctx.var("lunar_rat").get()? == 5 {
            ctx.var("lunar_rat").set(Val::from(4))?;
        }
        ctx.lines(args![
            "It's not nearly enough",
            "to repay you for what you've",
            "done for me, but I'd like",
            "you to have this pastry that",
            "I just made. Please enjoy it~"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Rice Mill Grandma",
            args![
                "Thank you for the",
                "Sweet Rice! I'll be",
                "sure to make something",
                "delicious for you if you",
                "bring me more, okay?"
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}

pub fn rice_mill_man_rat(ctx: &Ctx) -> Script {
    ctx.mes("[Rice Mill Man]")?;
    if !ctx.var("lunar_rat").get()?.is_true() {
        ctx.lines(args![
            "Breaks my heart...",
            "My mom spends all this",
            "time preparing rice for",
            "the new year, and it's all",
            "stolen by rats and vermin!"
        ])?;
        return ctx.close();
    }
    if ctx.var("lunar_rat").get()? == 1 {
        ctx.lines(args![
            "I hear from my mother",
            "that you're helping her out.",
            "She's a sweet old lady, huh?",
            "If you find ang Rice Pouches,",
            "you might want to have her",
            "inspect them first."
        ])?;
        return ctx.close();
    }
    if ctx.var("lunar_rat").get()?.number()? >= 2 {
        if ctx.items().count(7869)? < 1 {
            ctx.lines(args![
                "If you bring me some",
                "Rice Pouches, then I can",
                "pound into into Sweet Rice.",
                "You're here to help out my",
                "mother, right? Thanks,",
                "I really appreciate that."
            ])?;
            return ctx.close();
        }
        ctx.lines(args![
            "Oh, you brought me some",
            "Rice Pouches from my mother?",
            "Give me a second, and I'll",
            "get this rice pounded into",
            "paste, lickety split."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Rice Mill Man",
            args!["Hoo! Haa! Hi-yah!", "Woosha! Whoosha!", "Ka-taaaaaaaaaa!", "WOOOSHA!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rice Mill Man",
            args![
                "All done. Here, this",
                "Sweet Rice is ready to",
                "be made into pastries",
                "Would you please bring",
                "this to my mother?"
            ],
        )?;
        ctx.items().take(7869, 1)?;
        ctx.items().give(7770, 1)?;
        return ctx.close();
    }
    Ok(())
}

pub fn miss_lunar_rat(ctx: &Ctx) -> Script {
    ctx.mes("[Lunar]")?;
    if ctx.var("MaxWeight").get()?.number()? - ctx.var("Weight").get()?.number()? < 3000
        || ctx.call(Function::CheckWeight, args![1201, 1])? == 0
    {
        ctx.lines(args![
            "Oh, I'm sorry, but we",
            "can't do any business if",
            "you're carry so much stuff.",
            "Would you mind storing some of",
            "your things with the Kafra Service?"
        ])?;
        return ctx.close();
    }
    if ctx.var("lunar_rat").get()?.number()? < 4 {
        ctx.lines(args![
            "I want to earn enough",
            "money to buy that big",
            "crescent silver pin.",
            "I have to sell as much",
            "of this Mojji as I can!"
        ])?;
        return ctx.close();
    }
    if ctx.var("lunar_rat").get()?.number()? >= 4 {
        ctx.lines(args![
            "Oh, hello! I'm selling",
            "special rice cakes made",
            "from sweet rice. You can",
            "eat it yourself, or feed it",
            "to your New Year Doll if",
            "you have one as a pet."
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "Lunar",
            args!["Anyway, I'm selling", "10 Mojji for 3,000 zeny", "Would you like to try some?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Yes", "No"])? == 0 {
            if ctx.player().zeny()? >= 3000 {
                ctx.player().set_zeny(ctx.player().zeny()? - 3000)?;
                ctx.items().give(554, 10)?;
                ctx.lines_as(
                    "Lunar",
                    args!["Thank you, and I hope you", "have a happy new year!", "Please come again~"],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Lunar",
                args!["Oh, I'm sorry, but you", "don't have enough", "zeny right now...."],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Lunar",
            args![
                "Oh, alright. Well, I'll",
                "be here if you or your",
                "friends want some Mojji",
                "later, alight? Goodbye~"
            ],
        )?;
        return ctx.close();
    }
    Ok(())
}
