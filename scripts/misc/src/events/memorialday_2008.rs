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

pub fn lauds_memorial(ctx: &Ctx) -> Script {
    if ctx.var("memorial08").get()?.number()? < 1 {
        ctx.lines_as("Mad Sago Lauds", args!["Hey, yo!", "What are you doing there!?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "Do you know what day it is!?",
                "It's a very important date!",
                "A very important date to remember!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mad Sago Lauds", args!["What is this!?"])?;
        ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
        ctx.next()?;
        ctx.lines_as("Mad Sago Lauds", args!["Where is your towel!?"])?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["My what?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["How are you supposed to go on your trip without your towel!?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "If you think this is just another holiday, I'm not going to waste my breath.",
                "But if you like to pay a tribute to great soldiers, I will help you ready yourself."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["I guess so...", "Are you crazy or something?"])? == 1 {
            ctx.lines_as("Mad Sago Lauds", args!["Fine, be unprepared!"])?;
            ctx.next()?;
            ctx.lines_as("Mad Sago Lauds", args!["^FF0000YOU WILL RUE THE DAY!^000000"])?;
            return ctx.close();
        }
        ctx.lines_as("Mad Sago Lauds", args!["Great!", "Then listen to me carefully."])?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "Now before you can pay tribute to the fallen soldiers you must be properly equipped.",
                "Without your towel you will be lost!",
                "If you bring me the materials, I can make you a towel."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["Listen closely.", "Bring me ^FF000030 Fabric and 20 Fluffs^000000."],
        )?;
        ctx.var("memorial08").set(Val::from(1))?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()? == 1 {
        if ctx.items().count(1059)? < 30 || ctx.items().count(914)? < 20 {
            ctx.lines_as(
                "Mad Sago Lauds",
                args![
                    "What are these? They aren't enough?!",
                    "*Sigh* Do I really have to tell you again?!",
                    "Bring me ^FF000030 Fabric and 20 Fluffs^000000."
                ],
            )?;
            return ctx.close();
        }
        ctx.mes("***Mad Sago Lauds appears to be muttering to himself***")?;
        ctx.next()?;
        ctx.lines_as("Mad Sago Lauds", args!["So, did you bring the towel materials?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "Excellent; you've brought them all.",
                "Then I shall make you a Towel of Memory as I promised.",
                "Give me a moment."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mad Sago Lauds", args!["There you go!"])?;
        ctx.items().take(1059, 30)?;
        ctx.items().take(914, 20)?;
        ctx.items().give(6025, 1)?;
        ctx.call(Function::GetNamedItem, args![6025, ctx.player().name()?])?;
        ctx.var("memorial08").set(Val::from(2))?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()? == 2 {
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["Ah, you have fluffy new towel now I even put your name on it"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["Say, how do you like to go on a journey to pay a tribute to Memorial Day?"],
        )?;
        ctx.next()?;
        if ctx.menu(&["Sure.", "No, I'm Busy."])? == 1 {
            ctx.lines_as(
                "Mad Sago Lauds",
                args!["Don't come crying to me if someone decides to build a highway through your home!"],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["Good. By the way, do you even know what the towel is for?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "It seems you're just carrying it without understanding its meaning.",
                "What a shame! You should go speak to ^FF0000Grast in Prontera^000000."
            ],
        )?;
        ctx.var("memorial08").set(Val::from(3))?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()? == 3 {
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["What are you still doing here?", "I told you to go speak to Grast in Prontera!"],
        )?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()? == 4 || ctx.var("memorial08").get()? == 5 || ctx.var("memorial08").get()? == 6 {
        ctx.lines_as("Mad Sago Lauds", args!["Go help Grast, and then come back."])?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()? == 7 {
        ctx.lines_as(
            "Mad Sago Lauds",
            args!["Oh, you've brought them all.", "Hahaha!", "HAHAHAHAHAHA!!!! cough cough"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "Please give them to me. Thank you for your hard work, by the way.",
                "Say, have you learned anything from the journey?",
                "Now is to go visit the plaque at 12 o'clock direction in Prontera."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Mad Sago Lauds", args!["What do you mean you were just there?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "What? Why are you giving me your garbage!",
                "Those items are not useful to me at all.",
                "You should be going to the plaque!"
            ],
        )?;
        ctx.var("memorial08").set(Val::from(8))?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()? == 8 {
        ctx.lines_as(
            "Mad Sago Lauds",
            args![
                "Stop trying to give me your garbage!",
                "Go to the plaque at 12 o'clock direction in Prontera for your journey!"
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("memorial08").get()?.number()? >= 9 {
        let random_msg = ctx.call(Function::Rand, args![1, 5])?;
        if random_msg == 1 {
            ctx.lines_as("Mad Sago Lauds", args!["Don't panic!"])?;
            ctx.call(Function::Emotion, args![constants::ET_SURPRISE])?;
            return ctx.close();
        }
        if random_msg == 2 {
            ctx.lines_as(
                "Mad Sago Lauds",
                args![
                    "Life... is like a grapefruit.",
                    "It's orange and squishy, and has a few pips in it, and some folks have half a one for breakfast."
                ],
            )?;
            return ctx.close();
        }
        if random_msg == 3 {
            ctx.lines_as(
                "Mad Sago Lauds",
                args!["There was a point to this story, but it has temporarily escaped the chronicler's mind."],
            )?;
            return ctx.close();
        }
        if random_msg == 4 {
            ctx.lines_as("Mad Sago Lauds", args!["42!"])?;
            return ctx.close();
        }
        if random_msg == 5 {
            ctx.lines_as(
                "Mad Sago Lauds",
                args!["It is a mistake to think you can solve any major problems just with potatoes."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum MemorialPlaqueMemorialStep {
    Start,
    LCleanPlaque,
}

fn memorial_plaque_memorial_run(ctx: &Ctx, mut step: MemorialPlaqueMemorialStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            MemorialPlaqueMemorialStep::Start => {
                if ctx.var("memorial08").get()?.number()? < 8 {
                    ctx.lines_as(ctx.player().name()?, args!["- It's a dusty old plaque.-"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()? == 8 {
                    ctx.lines_as(
                        ctx.player().name()?,
                        args![
                            "- It's a dusty old plaque.-",
                            "'This must be what Lauds was talking about.'",
                            "'Let's dust it off with the towel.'"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.var("memorial08").set(Val::from(9))?;
                    ctx.call(Function::GetExperience, args![93750, 43750])?;
                } else {
                    ctx.var("memorial08").get()?;
                }
                step = MemorialPlaqueMemorialStep::LCleanPlaque;
                continue 'machine;
            }
            MemorialPlaqueMemorialStep::LCleanPlaque => {
                ctx.mes("- You see a message from the cleaned plaque.-")?;
                ctx.next()?;
                ctx.lines(args![
                    "-Although no sculptured marble should rise to their memory,-",
                    "-nor engraved stone bear record of their deeds,-",
                    "-yet will their remembrance be as lasting as the land they honored.-",
                    "-Daniel Webster-"
                ])?;
                if ctx.var("memorial08").get()? == 9 {
                    ctx.call(Function::GetExperience, args![93750, 43750])?;
                }
                ctx.next()?;
                ctx.mes("-There's another message.-")?;
                ctx.next()?;
                ctx.lines(args![
                    "-I may not have gone where I intended to go,-",
                    "-but I think I have ended up where I needed to be.-",
                    "- Douglas Adams.-"
                ])?;
                if ctx.var("memorial08").get()? == 9 {
                    ctx.call(Function::GetExperience, args![93750, 43750])?;
                }
                ctx.next()?;
                ctx.mes("-This is the last message.-")?;
                ctx.next()?;
                ctx.lines(args![
                    "-True heroism is remarkably sober, very undramatic.-",
                    "-It is not the urge to surpass all others at whatever cost,-",
                    "- but the urge to serve others at whatever cost. -",
                    "- Arthur Ashe -"
                ])?;
                if ctx.var("memorial08").get()? == 9 {
                    ctx.call(Function::GetExperience, args![93750, 43750])?;
                    ctx.var("memorial08").set(Val::from(10))?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn memorial_plaque_memorial(ctx: &Ctx) -> Script {
    memorial_plaque_memorial_run(ctx, MemorialPlaqueMemorialStep::Start).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum GrastMemorialStep {
    Start,
    LGiveUp,
    LContinue,
}

fn grast_memorial_run(ctx: &Ctx, mut step: GrastMemorialStep) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GrastMemorialStep::Start => {
                if ctx.var("memorial08").get()?.number()? < 3 {
                    ctx.lines_as(
                        "Grast",
                        args![
                            "Memorial Day is a sad and yet glorious day.",
                            "I wonder how many people remember them..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()? == 3 {
                    ctx.lines_as(
                        "Grast",
                        args![
                            "Oh, isn't that a Towel of Memory?",
                            "I'm so glad to meet someone who understands the meaning of Memorial Day."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grast",
                        args!["It is very important to know what we're celebrating today, don't you think?"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grast",
                        args!["If you like to go on a journey to pay a tribute to Memorial Day, you should bring me some materials I ask."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grast",
                        args![
                            "Please bring me ^FF0000one of each Red Potion, Green Potion, Awakening Potion, and Butterfly Wing^000000.",
                            "I'll be waiting for your return."
                        ],
                    )?;
                    ctx.var("memorial08").set(Val::from(4))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()? == 4 {
                    if ctx.items().count(501)? < 1
                        || ctx.items().count(506)? < 1
                        || ctx.items().count(656)? < 1
                        || ctx.items().count(602)? < 1
                    {
                        ctx.lines_as(
                            "Grast",
                            args![
                                "Oops, you haven't brought all materials.",
                                "Please make sure you need to bring me",
                                "^FF0000one of each Red Potion, Green Potion, Awakening Potion, and Butteryfly Wing^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grast",
                            args![
                                "You'll have to bring me more materials afterwards.",
                                "If you feel too burdened to gather them all,",
                                "I can provide you all the materials."
                            ],
                        )?;
                        ctx.next()?;
                        step = if ctx.menu(&["I'll gather the rest.", "Give me the materials."])? == 1 {
                            GrastMemorialStep::LGiveUp
                        } else {
                            GrastMemorialStep::LContinue
                        };
                        continue 'machine;
                    }
                    ctx.lines_as("Grast", args!["Oh, you've brought the materials I asked."])?;
                    ctx.next()?;
                    ctx.lines_as("Grast", args!["Hmm, you will need some more things still..."])?;
                    ctx.next()?;
                    ctx.lines_as("Grast", args!["Please bring me ^FF0000one of each Trap, Yggdrasil Leaf, Blue Gemstone, Crystal Mirror, Meat, and Carrot.^000000", "I'll be waiting for your return."])?;
                    ctx.var("memorial08").set(Val::from(5))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()? == 5 {
                    if ctx.items().count(1065)? < 1
                        || ctx.items().count(610)? < 1
                        || ctx.items().count(717)? < 1
                        || ctx.items().count(747)? < 1
                        || ctx.items().count(517)? < 1
                        || ctx.items().count(515)? < 1
                    {
                        ctx.lines_as(
                            "Grast",
                            args![
                                "Oops, you haven't brought all materials.",
                                "Please make sure you'll have to bring me",
                                "^FF0000one of each Trap, Yggdrasil Leaf, Blue Gemstone, Crystal Mirror, Meat, and Carrot^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grast",
                            args![
                                "You'll have to bring me more materials afterwards.",
                                "If you feel too burdened to gather them all, I can provide you all the materials."
                            ],
                        )?;
                        ctx.next()?;
                        step = if ctx.menu(&["I'll gather the rest.", "Give me the materials."])? == 1 {
                            GrastMemorialStep::LGiveUp
                        } else {
                            GrastMemorialStep::LContinue
                        };
                        continue 'machine;
                    }
                    ctx.lines_as(
                        "Grast",
                        args![
                            "Oh, you've brought everything I asked.",
                            "Hmm, I think you need just a little bit more."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grast",
                        args!["Please bring me ^FF0000one of each Pet Incubator, Firecracker, Poring Doll, and Bouquet^000000."],
                    )?;
                    ctx.var("memorial08").set(Val::from(6))?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()? == 6 {
                    if ctx.items().count(643)? < 1
                        || ctx.items().count(12018)? < 1
                        || ctx.items().count(741)? < 1
                        || ctx.items().count(745)? < 1
                    {
                        ctx.lines_as(
                            "Grast",
                            args![
                                "Oops, you haven't brought all materials.",
                                "Please make sure you need to bring me",
                                "^FF0000one of each Pet Incubator, Firecracker, Poring Doll and Bouquet"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grast",
                            args![
                                "They are the last batch of materials.",
                                "If you feel too burdened to gather them all, I can provide you all the materials."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Grast",
                            args![
                                "If I provide you all materials, however, I won't have to thank you for your service.",
                                Val::from("It's your call, ") + ctx.player().name()? + Val::from(".")
                            ],
                        )?;
                        ctx.next()?;
                        step = if ctx.menu(&["I'll gather the rest.", "Give me the materials."])? == 1 {
                            GrastMemorialStep::LGiveUp
                        } else {
                            GrastMemorialStep::LContinue
                        };
                        continue 'machine;
                    }
                    ctx.lines_as(
                        "Grast",
                        args![
                            "Have you brought the materials I asked?",
                            "Ah, thank you for your hard work you've brought all of them."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grast",
                        args![
                            "I hope you'll learn a valuable lesson while gathering these materials.",
                            "I like to give you a small gift for your service."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Grast",
                        args![
                            "You see, I have two different gifts in my each hand.",
                            "A best thing would be giving you both of them, but...",
                            Val::from("How do you like to test your luck, ") + ctx.player().name()? + Val::from("?")
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Grast", args!["Okay, which hand would you like to pick?"])?;
                    ctx.next()?;
                    if ctx.menu(&["Left hand.", "Right hand."])? == 1 {
                        ctx.lines_as(
                            "Grast",
                            args![
                                "You've selected my right hand.",
                                "Here's the gift for you.",
                                "Now, please bring all these materials to Lauds."
                            ],
                        )?;
                        ctx.var("memorial08").set(Val::from(7))?;
                        ctx.items().give(617, 1)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    ctx.lines_as(
                        "Grast",
                        args![
                            "You've selected my left hand.",
                            "Here's the gift for you.",
                            "Now, please bring all these materials to Lauds."
                        ],
                    )?;
                    ctx.var("memorial08").set(Val::from(7))?;
                    ctx.items().give(12109, 1)?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()? == 7 {
                    ctx.lines_as(
                        "Grast",
                        args![
                            "Have you met Lauds?",
                            "I hope you'll remember your freedom and happiness are built on thousands of lives sacrificed in war."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if ctx.var("memorial08").get()?.number()? >= 8 {
                    ctx.lines_as(
                        "Grast",
                        args!["A towel is about the most massively useful thing an adventurer can have."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = GrastMemorialStep::LGiveUp;
                continue 'machine;
            }
            GrastMemorialStep::LGiveUp => {
                ctx.lines_as(
                    "Grast",
                    args![
                        "Oh, I see. I guess you're quite busy nowadays, huh?",
                        "No problem; I'll give you all the supplies..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Grast",
                    args![
                        "There you go.",
                        "I crushed all the items together into a more compact form for you.",
                        "You can thank me later for that extra service.",
                        "Please bring them to Lauds."
                    ],
                )?;
                ctx.var("memorial08").set(Val::from(7))?;
                ctx.items().give(7126, 1)?;
                ctx.next()?;
                ctx.lines_as(
                    "Grast",
                    args![
                        "By the way, he had an unfortunate accident, and has kind of lost his mind.",
                        "Please don't be alarmed even if he starts babbling."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            GrastMemorialStep::LContinue => {
                ctx.lines_as("Grast", args!["That's a good idea.", "Then I'll be waiting for your return."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn grast_memorial(ctx: &Ctx) -> Script {
    grast_memorial_run(ctx, GrastMemorialStep::Start).map(|_| ())
}
