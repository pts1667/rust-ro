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

pub fn keedz_nif(ctx: &Ctx) -> Script {
    ctx.lines_as("Keedz", args!["I don't allow any living person", "to come in this place!"])?;
    ctx.close()
}

pub fn gigantia_nif(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![5038])?.is_true()
        || ctx.call(Function::IsEquipped, args![2257])?.is_true()
        || ctx.call(Function::IsEquipped, args![2256])?.is_true()
    {
        ctx.lines_as(ctx.player().name()?, args!["What's up?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Gigantia",
            args!["Just...", "Come over here.", "I have something", "I must do for you."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Gigantia",
            args![
                "Your horn is crooked.",
                "Always make sure your horn",
                "is worn straight and neat.",
                "The Lord of Death is always",
                "looking at you."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Gigantia",
        args![
            "The Lord of Death knows",
            "and sees all. It's useless",
            "to hide, and escape from",
            "Death's sweet embrace."
        ],
    )?;
    ctx.close()
}

pub fn gigantia_nif_ontouch(ctx: &Ctx) -> Script {
    if ctx.call(Function::IsEquipped, args![5038])?.is_true()
        || ctx.call(Function::IsEquipped, args![2257])?.is_true()
        || ctx.call(Function::IsEquipped, args![2256])?.is_true()
    {
        ctx.lines_as("Gigantia", args!["Hey, wait!"])?;
        return ctx.close();
    }
    ctx.end()
}

pub fn undead_chicken_nif(ctx: &Ctx) -> Script {
    ctx.call(Function::PercentHeal, args![-5, 0])?;
    ctx.lines_as("Undead Chicken", args!["I lived a peaceful life as a normal chicken. But then came the day I was tragically killed and eaten by humans. Well... Heh heh~! Now it's my turn! *Cackles*"])?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["Ouch...!", "A chicken...", "It bit me!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Undead Chicken",
        args![
            "Ho ho~!",
            "I can talk AND feast",
            "on living humans!",
            "Being a zombie is great!",
            "*Cackles*"
        ],
    )?;
    ctx.close()
}

pub fn undead_familiar_nif(ctx: &Ctx) -> Script {
    ctx.call(Function::PercentHeal, args![-5, 0])?;
    ctx.lines_as(
        "Vatoman",
        args![
            "Oooh, how handy, a living",
            "human~! Fresh blood is",
            "always tasty...! I think I'll just",
            "take a liiittle sip."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(ctx.player().name()?, args!["Ow! My vein!", "Did you just", "suck my blood?!"])?;
    ctx.next()?;
    ctx.lines_as("Vatoman", args!["Mwahahaha~", "Foolish mortal!", "Beware my powers!"])?;
    ctx.close()
}

pub fn child_niflheim(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Alakina Ann",
        args![
            "Where...where am I...?",
            "I remember I was sleeping",
            "and when I woke up, I was here...",
            "Mommy, have you seen my mommy?",
            "I wanna go home...*Sob*"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["About the witch", "About the curse", "Cancel."])? {
        0 => {
            ctx.lines_as("Alakina Ann", args!["Witch...? I've never seen a witch, but I've read about them in books. I'm not sure if they exist or not, but it would be so horrifying if they did..."])?;
            ctx.next()?;
            ctx.lines_as("Alakina Ann", args!["Why is it so cold in here...?", "A-aren't you cold, or even scared? I wanna go home... It's warm over there. Could you help me get back... Please...? H-help me..."])?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as("Alakina Ann", args!["A c-curse...?", "My grandma says that there are lots of curses in the world. Some of them, you can tell it's a curse. But there are other curses that follow you throughout life..."])?;
            ctx.next()?;
            ctx.lines_as("Alakina Ann", args!["Grandma says curses can take other forms... Like if someone's always mad at you, and won't forgive you, you can see that as a curse too."])?;
            ctx.next()?;
            ctx.lines_as(
                "Alakina Ann",
                args!["My grandma told me there's only one spell that can break a strong curse that you can't lift with Blessings..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Alakina Ann",
                args![
                    "Klaatu...",
                    "Verata.....",
                    "Ne...ni...umm...?",
                    "What was the last part?",
                    "I-I can't remember",
                    "the last part of the spell!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Alakina Ann", args!["If you repeat the spell in that order, it will release you from someone's anger and hate. And if you do it near a bad spirit, they might get the curse instead."])?;
            ctx.next()?;
            ctx.lines_as(
                "Alakina Ann",
                args!["Why are you asking me about these scary kind of things...? I just wanna go home..."],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Alakina Ann",
                args![
                    "It's so cold in here...",
                    "I'm freezing...",
                    "And I wanna go home and",
                    "get away from this scary place..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Alakina Ann", args!["Please... help me.", "Could you take me with you...?"])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum CursedSpiritNifStep {
    Start,
    OnMyMobDead,
}

fn cursed_spirit_nif_run(ctx: &Ctx, mut step: CursedSpiritNifStep) -> Script {
    'machine: loop {
        match step {
            CursedSpiritNifStep::Start => {
                let mut spell = 0;
                ctx.call(Function::KillMonster, args!["niflheim", "Cursed Spirit#nif::OnMyMobDead"])?;
                ctx.lines_as(
                    "Ashe Bruce",
                    args![
                        "I sense you're cursed",
                        "by a powerful spell...",
                        "Hmm... It's clear what",
                        "you must be up to...."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ashe Bruce",
                    args!["You wish to get", "rid of your curse....", "By giving it to me!!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ashe Bruce",
                    args!["Just because I'm a cursed spirit, you adventurers think you can just dump your curses on me?!"],
                )?;
                ctx.npc().emotion(constants::ET_FRET)?;
                ctx.next()?;
                ctx.lines_as(
                    "Ashe Bruce",
                    args![
                        "I refuse to let",
                        "you remain here.....",
                        "Leave now, or I will",
                        "remove you by force...."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Ashe Bruce",
                    args!["....And...", "....Whatever you do...", "....Do NOT touch my books..."],
                )?;
                ctx.next()?;
                match ctx.menu(&[
                    "Touch the first book.",
                    "Touch the second book.",
                    "Touch the third book.",
                    "Okay, I am leaving.",
                ])? {
                    0 => {
                        ctx.call(
                            Function::Monster,
                            args!["niflheim", 349, 259, "Rideword", 1478, 1, "Cursed Spirit#nif::OnMyMobDead"],
                        )?;
                        ctx.lines_as(
                            "Ashe Bruce",
                            args![
                                "...!...",
                                "How dare you touch my books",
                                "when I specifically said",
                                "'Don't touch my books!'"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ashe Bruce",
                            args!["....!...Grrrrr!", "I shall tear you apart...!", "Be bound by an eternal curse...!"],
                        )?;
                        return ctx.close();
                    }
                    1 => {
                        ctx.lines_as(
                            "Ashe Bruce",
                            args![
                                "...!...",
                                "You dare touch my books?!",
                                "Right after I said not",
                                "to touch them...?!",
                                "Foolish mortal!",
                                "...BEGONE!"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.warp("niflheim", 34, 162)?;
                        return ctx.end();
                    }
                    2 => {
                        ctx.lines_as(
                            "Ashe Bruce",
                            args![
                                "Muhahahaha....",
                                "Stubborn mortal~!",
                                "Fine! I will give you",
                                "a fighting chance and let",
                                "you cast a spell."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ashe Bruce",
                            args![
                                "But Blessings won't",
                                "work with the curse",
                                "that you have...",
                                "And the spell to lift",
                                "your curse has been",
                                "lost to the ages~!"
                            ],
                        )?;
                        ctx.npc().emotion(constants::ET_KIK)?;
                        ctx.next()?;
                        if ctx.menu(&["Clover", "Klaatu", "Klaytos"])? == 1 {
                            spell += 1;
                        }
                        if ctx.menu(&["Verit", "Veritas", "Verata"])? == 2 {
                            spell += 1;
                        }
                        if ctx.menu(&["Necktie", "Necklace", "Nero", "^FFFFFFNictu!!!^000000"])? == 3 {
                            spell += 1;
                        }
                        if spell == 3 {
                            let subject2 = ctx.call(Function::Rand, args![1, 5])?;
                            if subject2 == 1 {
                                if ctx.var("morison_meat").get()?.number()? < 15 {
                                    ctx.var("morrison_meat").set(Val::from(15))?;
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args!["You... You broke the curse!", "How did you know that spell?!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args![
                                            "I suppose you expect for me to",
                                            "melt in agony about now, don't",
                                            "you? Well... Sorry to disappoint",
                                            "you, mortal, but I can never die!"
                                        ],
                                    )?;
                                    return ctx.close();
                                }
                                ctx.lines_as(
                                    "Ashe Bruce",
                                    args![
                                        "...! You cast the correct spell?!",
                                        "...!...",
                                        "But...You're still cursed...",
                                        "Umhaaaaaaaaaaaaaaaaa.....!"
                                    ],
                                )?;
                                return ctx.close();
                            } else if subject2 == 2 {
                                if ctx.var("thai_head").get()? == 1 {
                                    ctx.var("thai_head").set(Val::from(2))?;
                                    ctx.lines_as("Ashe Bruce", args!["What's...", "this feeling?"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ashe Bruce", args!["No...!", "NOOOOOOOOOOOOOOOO!"])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ashe Bruce", args!["Why did your spell have to work?!"])?;
                                    return ctx.close();
                                }
                                ctx.lines_as("Ashe Bruce", args!["You...", "cast the correct spell?!"])?;
                                ctx.next()?;
                                ctx.lines_as("Ashe Bruce", args!["Hoho~", "But you're still cursed..."])?;
                                return ctx.close();
                            } else if subject2 == 3 {
                                if ctx.var("thai_head").get()? == 8 {
                                    ctx.var("thai_head").set(Val::from(7))?;
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args!["You... You broke the curse!", "Who taught you that spell?!"],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Ashe Bruce",
                                        args![
                                            "I suppose you expect for me to",
                                            "melt in agony about now, don't",
                                            "you? Well... Sorry to disappoint",
                                            "you, mortal, but I can never die!"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Ashe Bruce", args!["So long as I'm...", "still...", "cursed."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Ashe Bruce", args!["NOOOOOOOOOO!"])?;
                                    return ctx.close();
                                }
                                ctx.lines_as(
                                    "Ashe Bruce",
                                    args![
                                        "...! You cast the correct spell?!",
                                        "...!...",
                                        "But...You're still cursed...",
                                        "Umhaaaaaaaaaaaaaaaaa.....!"
                                    ],
                                )?;
                                return ctx.close();
                            } else if subject2 == 4 {
                                ctx.lines_as(
                                    "Ashe Bruce",
                                    args![
                                        "...! You cast the correct spell?!",
                                        "...!...",
                                        "But...You're still cursed...",
                                        "Mwahahahaaaa.....!"
                                    ],
                                )?;
                                return ctx.close();
                            }
                        }
                        for (x, y) in [(345, 259), (347, 261), (344, 253), (346, 251), (349, 249), (350, 260), (353, 256)] {
                            ctx.call(
                                Function::Monster,
                                args!["niflheim", x, y, "Orc Skeleton", 1462, 1, "Cursed Spirit#nif::OnMyMobDead"],
                            )?;
                        }
                        ctx.lines_as(
                            "Ashe Bruce",
                            args![
                                "Muhahahahahaha!",
                                "That's not the right spell!",
                                "Now, death awaits you!",
                                "You're eternally bound",
                                "to the curse...!"
                            ],
                        )?;
                        return ctx.close();
                    }
                    3 => {
                        ctx.lines_as("Ashe Bruce", args!["...", "....."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ashe Bruce",
                            args!["Well then.", "Try not to trip on", "your feet in your", "rush to leave."],
                        )?;
                        return ctx.close();
                    }
                    _ => {}
                }
                step = CursedSpiritNifStep::OnMyMobDead;
                continue 'machine;
            }
            CursedSpiritNifStep::OnMyMobDead => {
                return ctx.end();
            }
        }
    }
}

pub fn cursed_spirit_nif(ctx: &Ctx) -> Script {
    cursed_spirit_nif_run(ctx, CursedSpiritNifStep::Start)
}

pub fn cursed_spirit_nif_onmymobdead(ctx: &Ctx) -> Script {
    cursed_spirit_nif_run(ctx, CursedSpiritNifStep::OnMyMobDead)
}
