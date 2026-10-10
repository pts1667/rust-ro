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

#[derive(Clone, Copy, Debug)]
enum GarrisonStep {
    Start,
    SubGarrison,
    SubGarrison2,
}

fn garrison_run(ctx: &Ctx, mut step: GarrisonStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            GarrisonStep::Start => {
                if ctx.var("BaseJob").get()? != constants::JOB_GUNSLINGER {
                    ctx.lines_as(
                        "Garrison",
                        args!["You're not a gunslinger.", "You're distracting me from my work. Go on."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.player().base_level()? < 55 {
                    ctx.lines_as("Garrison", args!["My name is Garrison. I'm a master in crafting guns.", "The gun I make is also called the Garrison. I named it that because I think the Garrison is as cool and perfect as myself."])?;
                    ctx.next()?;
                    ctx.lines_as("Garrison", args!["Why don't you commission a Garrison from me?"])?;
                    ctx.next()?;
                    garrison_run(ctx, GarrisonStep::SubGarrison, vec![])?;
                }
                'b1: {
                    let subject1 = ctx.var("gun_gs").get()?;
                    let mut matched1 = false;
                    if !matched1 && subject1 == 0 {
                        matched1 = true;
                    }
                    if matched1 {
                        if ctx.items().count(13104)? < 1 {
                            ctx.lines_as("Garrison", args!["My name is Garrison. I'm a master in crafting guns.", "The gun I make is also called the Garrison. I named it that because I think the Garrison is as cool and perfect as myself."])?;
                            ctx.next()?;
                            ctx.lines_as("Garrison", args!["Are you here to commission a Garrison from me?"])?;
                            ctx.next()?;
                            garrison_run(ctx, GarrisonStep::SubGarrison, vec![])?;
                        }
                        ctx.lines_as(
                            "Garrison",
                            args!["Hmm~ What's going on?", "Are you here because you need a weapon?"],
                        )?;
                        ctx.next()?;
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("I need a Garrison."), Val::from("Not really."), Val::from("Cancel")],
                            )?);
                            let mut matched2 = false;
                            if !matched2 && subject2 == 1 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Garrison",
                                    args![
                                        "Oh~ Welcome",
                                        "Everyone needs me~!",
                                        "It just proves that my gun, the Garrison, is the best of the best.",
                                        "So, are you going to ask me to make one now?"
                                    ],
                                )?;
                                ctx.next()?;
                                garrison_run(ctx, GarrisonStep::SubGarrison, vec![])?;
                            }
                            if !matched2 && subject2 == 2 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Garrison",
                                    args![
                                        "Hmm~ You look like you're wandering around without a mission.",
                                        "If you've got the time, will you do me a favor?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(ctx.player().name()?, args!["What is it?"])?;
                                ctx.next()?;
                                ctx.lines_as("Garrison", args!["Mmm~ Well, I had actually received a letter recently stating that there are many defective makes of a gun I created called the Six Shooter."])?;
                                ctx.next()?;
                                ctx.lines_as("Garrison", args!["I'm a perfectionist, but due to my old age, my eyes are growing dark, and I think I may have sold defective Six Shooters."])?;
                                ctx.next()?;
                                ctx.lines_as("Garrison", args!["So what I'm trying to say is, I'm hoping someone will find me spare parts which I can use to replace defective parts in the detective Six Shooters."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Garrison",
                                    args![
                                        "If you help me, I'll put a slot in that Garrison you're holding right now.",
                                        "What do you say?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Garrison", args!["Just so you know, if I slot your Garrison, any current refine points or slotted cards will disappear. Decide carefully."])?;
                                ctx.next()?;
                                'b3: {
                                    let subject3 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("I don't like the sound of this."), Val::from("Ok, I'll try it out.")],
                                    )?);
                                    let mut matched3 = false;
                                    if !matched3 && subject3 == 1 {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        ctx.lines_as(
                                            "Garrison",
                                            args!["Hmm, I see.", "I understand.", "I guess I'll look for a different man."],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched3 && subject3 == 2 {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        ctx.lines_as(
                                            "Garrison",
                                            args![
                                                "Oh, would you?",
                                                "I'm so thankful.",
                                                "These are the materials I need to make spare parts..."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Garrison",
                                            args![
                                                "10 Steel, 1 Elunium, 10 Emveretarcon, 30 Coal, and 10 Rusty Screws.",
                                                "I need this exact amount. Don't forget."
                                            ],
                                        )?;
                                        ctx.var("gun_gs").set(1)?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            if !matched2 && subject2 == 3 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as("Garrison", args!["Hmm... Come and see me later."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        if ctx.items().count(999)? < 10
                            || ctx.items().count(7317)? < 10
                            || ctx.items().count(1011)? < 10
                            || ctx.items().count(1003)? < 30
                            || ctx.items().count(985)? < 1
                        {
                            ctx.lines_as(
                                "Garrison",
                                args![
                                    "10 Steel, 1 Elunium, 10 Emveretarcon, 30 Coal, and 10 Rusty Screws.",
                                    "Don't forget -- the amount has to be exact."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "Oh~ You're back with what I need~",
                                "I'm so grateful.",
                                "Oh.. there's another favor I need to have done. Err..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Garrison", args!["I need to deliver the spare parts once I make them to another person, but I've got so much work to do around here."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Garrison",
                            args!["Sorry to ask, but I was hoping you can deliver what I make out of these materials to the next person."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Garrison",
                            args!["While you are on delivery, I will make preparations to slot your Garrison, as promised."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args!["-Hmm, I don't want to do it, but I'll do it anyway.-"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["Ok."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "Oh, thanks a lot.",
                                "Then deliver this to a person called Ravey. He lives in the slums of Lighthalzen."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "You receive the delivery goods from Garrison.",
                                "Let's deliver them to a person named Ravey, who lives in the slums of Lighthalzen."
                            ],
                        )?;
                        ctx.items().take(999, 10)?;
                        ctx.items().take(1011, 10)?;
                        ctx.items().take(7317, 10)?;
                        ctx.items().take(985, 1)?;
                        ctx.items().take(1003, 30)?;
                        ctx.var("gun_gs").set(2)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "You receive the delivery goods from Garrison.",
                                "Let's deliver them to a person named Ravey, who lives in the slums of Lighthalzen."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1 == 3 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as("Garrison", args!["Oh~ How was your trip?"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "Gyah~",
                                "I nearly died!!!",
                                "That man tried to attack me when he saw me. I barely escaped death!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Garrison", args!["Ahh~ Oh no.", "I had no idea the person who sent the letter was actually plotting to assassinate me. He was probably sent by one of our enemy guilds..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Garrison",
                            args!["It looks like I've sent you unintentionally on a very dangerous mission."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Garrison", args!["I'm so sorry.", "You may also know, but there's bad guys plotting to attack the Professor and the Gunslinger Guild all over the place."])?;
                        ctx.next()?;
                        ctx.lines_as("Garrison", args!["Consider this a part of your training as an Gunslinger."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Garrison",
                            args!["I'm sorry about what happened.", "But, as promised, I will slot your Garrison ..."],
                        )?;
                        ctx.next()?;
                        if ctx.items().count(13104)? < 1 {
                            ctx.lines_as(
                                "Garrison",
                                args![
                                    "Huh? Sigh.",
                                    "Did you sell off your gun while you were gone?",
                                    "I said I'd slot your gun, but I never said I'd slot a gun that isn't here.",
                                    "Go and get your Garrison and then get back to me."
                                ],
                            )?;
                            ctx.var("gun_gs").set(4)?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "Let's see~",
                                "*tonk* *tonk* *thump* *whump*",
                                "Here is your modified",
                                "Garrison.",
                                "Please use it well."
                            ],
                        )?;
                        ctx.items().take(13104, 1)?;
                        ctx.items().give(13105, 1)?;
                        ctx.var("gun_gs").set(5)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1 == 4 {
                        matched1 = true;
                    }
                    if matched1 {
                        if ctx.items().count(13104)? < 1 {
                            ctx.lines_as("Garrison", args!["Go and get your Garrison and then get back to me."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Garrison",
                            args!["Ah, there it is.", "Here is your modified", "Garrison.", "Do use it well."],
                        )?;
                        ctx.items().take(13104, 1)?;
                        ctx.items().give(13105, 1)?;
                        ctx.var("gun_gs").set(5)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1 == 5 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "Hmm~ You're back again~",
                                "What's the matter?",
                                "Did you come back because you need a weapon?",
                                "What do you need?"
                            ],
                        )?;
                        ctx.next()?;
                        'b4: {
                            let subject4 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Garrison"), Val::from("Slot a Garrison"), Val::from("Cancel")],
                            )?);
                            let mut matched4 = false;
                            if !matched4 && subject4 == 1 {
                                matched4 = true;
                            }
                            if matched4 {
                                garrison_run(ctx, GarrisonStep::SubGarrison, vec![])?;
                            }
                            if !matched4 && subject4 == 2 {
                                matched4 = true;
                            }
                            if matched4 {
                                garrison_run(ctx, GarrisonStep::SubGarrison2, vec![])?;
                            }
                            if !matched4 && subject4 == 3 {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines_as("Garrison", args!["I'll see you when I see you~"])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
                step = GarrisonStep::SubGarrison;
                continue 'machine;
            }
            GarrisonStep::SubGarrison => {
                ctx.lines_as(
                    "Garrison",
                    args![
                        "To create a Garrison, I need...",
                        "50 Steel, 3 Eluniums,",
                        "1 Oridecon, 50 Coal,",
                        "20 Rusty Screws, and",
                        "there's a fee of 30,000 Zeny.",
                        "Well, do you want one?."
                    ],
                )?;
                ctx.next()?;
                'b5: {
                    let subject5 = Val::from(runtime::select_values(
                        ctx,
                        &[
                            Val::from("Maybe later."),
                            Val::from("Yes, make it for me immediately."),
                            Val::from("Cancel"),
                        ],
                    )?);
                    let mut matched5 = false;
                    if !matched5 && subject5 == 1 {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as(
                            "Garrison",
                            args!["Hmmm~ I got worked up for nothing~", "Think about it. Come see me if you decide."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5 == 2 {
                        matched5 = true;
                    }
                    if matched5 {
                        if ctx.items().count(999)? < 50
                            || ctx.items().count(7317)? < 20
                            || ctx.items().count(984)? < 1
                            || ctx.items().count(1003)? < 50
                            || ctx.items().count(985)? < 3
                        {
                            ctx.lines_as(
                                "Garrison",
                                args![
                                    "You didn't bring enough materials",
                                    "To create a Garrison, I need...",
                                    "50 Steel, 3 Eluniums,",
                                    "1 Oridecon, 50 Coal,",
                                    "50 Rusty Screws,",
                                    "and there's a fee of 30,000 Zeny",
                                    "Don't forget."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.player().zeny()? < 30000 {
                            ctx.lines_as("Garrison", args!["You need more Zeny~!", "More Zeny!!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::CheckWeight, args![13104, 1])? == 0 {
                            ctx.lines_as(
                                "Garrison",
                                args![
                                    "There's no room in your inventory",
                                    "for my creation.",
                                    "Make some room",
                                    "in your inventory, and then",
                                    "Come and see me."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "Hmm, looks like the correct amount of materials and Zeny.",
                                "Here's a Garrison~",
                                "If you need one again, come and see me~"
                            ],
                        )?;
                        ctx.items().take(984, 1)?;
                        ctx.items().take(985, 3)?;
                        ctx.items().take(999, 50)?;
                        ctx.items().take(1003, 50)?;
                        ctx.items().take(7317, 20)?;
                        ctx.player().set_zeny(ctx.player().zeny()? - 30000)?;
                        ctx.items().give(13104, 1)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched5 && subject5 == 3 {
                        matched5 = true;
                    }
                    if matched5 {
                        ctx.lines_as("Garrison", args!["I'll see you when I see you~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = GarrisonStep::SubGarrison2;
                continue 'machine;
            }
            GarrisonStep::SubGarrison2 => {
                ctx.lines_as(
                    "Garrison",
                    args![
                        "In order to slot a Garrison, I need",
                        "10 Steel, 1 Elunium,",
                        "10 Emveretarcon, 30 Coal,",
                        "10 Rusty Screws, and",
                        "1 Garrison.",
                        "Well, do you want one?."
                    ],
                )?;
                ctx.next()?;
                'b6: {
                    let subject6 = Val::from(runtime::select_values(
                        ctx,
                        &[
                            Val::from("Maybe later."),
                            Val::from("Yes, make it for me immediately."),
                            Val::from("Cancel"),
                        ],
                    )?);
                    let mut matched6 = false;
                    if !matched6 && subject6 == 1 {
                        matched6 = true;
                    }
                    if matched6 {
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "You've got me worked up for nothing~",
                                "Get back to me when you've made up your mind."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched6 && subject6 == 2 {
                        matched6 = true;
                    }
                    if matched6 {
                        if ctx.items().count(999)? < 10
                            || ctx.items().count(7317)? < 10
                            || ctx.items().count(1011)? < 10
                            || ctx.items().count(1003)? < 30
                            || ctx.items().count(985)? < 1
                            || ctx.items().count(13104)? < 1
                        {
                            ctx.lines_as(
                                "Garrison",
                                args![
                                    "You didn't bring enough materials",
                                    "In order to slot a Garrison, I need",
                                    "10 Steel, 1 Elunium,",
                                    "10 Emveretarcon, 30 Coal,",
                                    "10 Rusty Screws, and",
                                    "1 Garrison.",
                                    "Don't forget."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::CheckWeight, args![13105, 1])? != 1 {
                            ctx.lines_as(
                                "Garrison",
                                args![
                                    "There's no room in your inventory",
                                    "for my creation.",
                                    "Make some room",
                                    "in your inventory, and then",
                                    "Come and see me."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Garrison",
                            args![
                                "Hmm, looks like the correct amount of materials and Zeny.",
                                "Here's a modified Garrison with a slot.",
                                "If you need one again, come and see me~"
                            ],
                        )?;
                        ctx.items().take(999, 10)?;
                        ctx.items().take(1011, 10)?;
                        ctx.items().take(7317, 10)?;
                        ctx.items().take(985, 1)?;
                        ctx.items().take(1003, 30)?;
                        ctx.items().take(13104, 1)?;
                        ctx.items().give(13105, 1)?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched6 && subject6 == 3 {
                        matched6 = true;
                    }
                    if matched6 {
                        ctx.lines_as("Garrison", args!["I'll see you when I see you~"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn garrison(ctx: &Ctx) -> Script {
    garrison_run(ctx, GarrisonStep::Start, Vec::new()).map(|_| ())
}

pub fn ravey(ctx: &Ctx) -> Script {
    if ctx.var("gun_gs").get()? == 2 {
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "Excuse me. Hello.",
                "I'm here to give you replacement parts for defective Gunslin..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Ravey", args!["You!!!", "Huuut~!!", "Die!!!"])?;
        ctx.var("gun_gs").set(3)?;
        ctx.call(Function::PercentHeal, args![100, 0])?;
        ctx.call(Function::PercentHeal, args![-90, 0])?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "- The man called Ravey suddenly attacks, as if he was waiting for the word Gunslinger...-",
                "You nearly died.-"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args!["You run away without looking back.", "Let's hurry back to Garrison.-"],
        )?;
        return ctx.close();
    }
    if ctx.var("gun_gs").get()? == 3 {
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "- The man called Ravey suddenly attacks, as if he was waiting for the word Gunslinger...-",
                "You nearly died.-"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(ctx.player().name()?, args!["Let's hurry back to Garrison.-"])?;
        return ctx.close();
    }
    ctx.lines_as("Ravey", args!["Ugh...", "Err.. Err..", "......"])?;
    ctx.close()
}

#[derive(Clone, Copy, Debug)]
enum IngridStep {
    Start,
    SubInferno,
}

fn ingrid_run(ctx: &Ctx, mut step: IngridStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            IngridStep::Start => {
                if ctx.var("BaseJob").get()? != constants::JOB_GUNSLINGER {
                    ctx.lines_as(
                        "Ingrid",
                        args![
                            "How are you!",
                            "I've been appointed the new Gunslinger Weapons Creator. The name is Ingrid."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Ingrid",
                        args!["However, you don't seem to be a member of the Gunslinger Guild, so I can't help you with anything. Sorry."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                'b1: {
                    let subject1 = ctx.var("gun_inf").get()?;
                    let mut matched1 = false;
                    if !matched1 && subject1 == 0 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Ingrid",
                            args![
                                "How are you!",
                                "I've been appointed the new Gunslinger",
                                "Weapons Creator. The name is Ingrid.",
                                "A pleasure to serve you!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args![
                                "It's only been a short while",
                                "since I've started working,",
                                "But I'll try my best to",
                                "assist you."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args![
                                "Ahh ... I feel so anxious.",
                                "As of now, I'm doing the job of creating a weapon called the Inferno for customers."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args!["The Inferno is the most advanced weapon made in our Guild laboratory."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ingrid", args!["It features incredible power and range, and an ergonomic design so that it can be held well and significantly minimize kickback when fired.", "It's a weapon worthy of being called the ultimate weapon for any Gunslinger.", "huff huff..."])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["You know quite a lot about it", "....."])?;
                        ctx.next()?;
                        ctx.lines_as("Ingrid", args!["Ah. Of course~", "I am the person who designed this weapon.."])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["Wooow~ That's incredible~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args![
                                "Ah Oops...",
                                "Did I end up bragging?",
                                "I'm still nothing compared to Professor Serena."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ingrid", args!["Actually, I wanted to become a gunslinger too, but because of my physical shortcomings weak determination, I had to give up."])?;
                        ctx.next()?;
                        ctx.lines_as("Ingrid", args!["While I was applying to become a gunslinger, I was lucky enough to have Professor Serena see my talents, and with her help, I'm where I am now helping with the manufacturing and sales of Gunslinger Weapons"])?;
                        ctx.next()?;
                        ctx.lines_as("Ingrid", args!["I'm still remorseful that I wasn't able to become a Gunslinger, but at least my brother, who took the Gunslinger test with me, passed and become one."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args!["I'm glad that even though I fell, my brother was able to fulfill our dream."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args!["Haha~ Look at me telling you all these things when you didn't even ask.", "Sorry."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "No, it's ok.",
                                "You may not have become a Gunslinger, but I think you've become a great person."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Ingrid", args!["Really? Thank you~", "I'll try my best."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Ingrid",
                            args![
                                "To produce an Inferno,",
                                "I need 100 Used Iron Plates, 10 Oridecons, 50 Old Rusty Screws, 100 Burning Hearts, and 200,000 Zeny.",
                                "Would you like to produce it for you?"
                            ],
                        )?;
                        ctx.next()?;
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("I'll think about it."), Val::from("Yes, please.")],
                            )?);
                            let mut matched2 = false;
                            if !matched2 && subject2 == 1 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Ingrid",
                                    args![
                                        "Ah~ I see.",
                                        "I understand.",
                                        "Ok, well, think about it.",
                                        "Come back when you've made up your mind."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if !matched2 && subject2 == 2 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as("Ingrid", args!["I understand.", "Here are the materials again."])?;
                                ctx.next()?;
                                ctx.lines_as("Inferno", args!["To produce an Inferno,", "I need 100 Used Iron Plates, 10 Oridecons, 50 Rusty Old Screws, 100 Burning Hearts, and 200,000 Zeny.", "You must bring me the correct number of materials.", "Please don't forget that."])?;
                                ctx.var("gun_inf").set(1)?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        ingrid_run(ctx, IngridStep::SubInferno, vec![])?;
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Ingrid",
                            args!["Hello~ You're back~", "Are you back because you need an Inferno?"],
                        )?;
                        ctx.next()?;
                        'b3: {
                            let subject3 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Nah, I'm back just because.."), Val::from("Yes, make me an Inferno.")],
                            )?);
                            let mut matched3 = false;
                            if !matched3 && subject3 == 1 {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Ingrid",
                                    args![
                                        "Ah~ I see.",
                                        "I understand.",
                                        "Ok, well, think about it.",
                                        "Come back when you've made up your mind."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if !matched3 && subject3 == 2 {
                                matched3 = true;
                            }
                            if matched3 {
                                ingrid_run(ctx, IngridStep::SubInferno, vec![])?;
                            }
                        }
                    }
                }
                step = IngridStep::SubInferno;
                continue 'machine;
            }
            IngridStep::SubInferno => {
                if ctx.items().count(7319)? < 100
                    || ctx.items().count(7317)? < 50
                    || ctx.items().count(984)? < 10
                    || ctx.items().count(7097)? < 100
                {
                    ctx.lines_as("Ingrid", args!["Yeah, the required materials are 100 Used Iron Plates, 10 Oridecons, 50 Rusty Old Screws, 100 Burning Hearts, and 200,000 Zeny", "You must bring me the correct number of materials.", "Please don't forget that."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.player().zeny()? < 200000 {
                    ctx.lines_as(
                        "Ingrid",
                        args![
                            "You've brought plenty of material,",
                            "but did you bring enough Zeny?",
                            "The price of creating an Inferno is 200,000 Zeny.",
                            "Please don't forget that."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.call(Function::CheckWeight, args![13162, 1])? != 1 {
                    ctx.lines_as(
                        "Ingrid",
                        args![
                            "It looks like you have too many items to carry my creation, so I can't give it to you.",
                            "Why don't you come and see me again when you're ready to receive it?"
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Ingrid",
                        args![
                            "I see you have the proper amount of materials and Zeny.",
                            "Here is the Inferno I've created for you.",
                            "Use it well."
                        ],
                    )?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 200000)?;
                    ctx.items().take(7319, 100)?;
                    ctx.items().take(984, 10)?;
                    ctx.items().take(7317, 50)?;
                    ctx.items().take(7097, 100)?;
                    ctx.items().give(13162, 1)?;
                    if ctx.var("gun_inf").get()? == 1 {
                        ctx.var("gun_inf").set(2)?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn ingrid(ctx: &Ctx) -> Script {
    ingrid_run(ctx, IngridStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum VanessaStep {
    Start,
    SubDestroyer,
    SubDestroyer2,
}

fn vanessa_run(ctx: &Ctx, mut step: VanessaStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            VanessaStep::Start => {
                if ctx.var("BaseJob").get()? != constants::JOB_GUNSLINGER {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "Hah !Hah !!",
                            "Ballitude! Commander Sambo!",
                            "Wrestling!, Muye Tai!",
                            "Pancracion!,Lucharibre!",
                            "I'm going to master all the bare hand weapons in this world!!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vanessa",
                        args!["Hmm?!", "Who are you? You're inturrupting my practice!", "Get lost!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                'b1: {
                    let subject1 = ctx.var("gunst").get()?;
                    let mut matched1 = false;
                    if !matched1 && subject1 == 0 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "Ha!Hyaa!!",
                                "Vale Tudo!Commando Sambo!",
                                "Wrestling!,Muay Thai!",
                                "Pankration!,Mucho Libre!",
                                "I will master all fighting styles",
                                "in this world!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "Mm?",
                                "You are a Gunslinger, huh?",
                                "Why are you disturbing",
                                "my exercises and me",
                                "standing beside me?"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args!["It's just watching exercise", "looks good...", "............."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args!["Oh! You like martial arts?", "Come here~ I'll lock", "you in an arm-bar~"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["N~ No thanks~", "it's okay~"])?;
                        ctx.next()?;
                        ctx.lines_as("Vanessa", args!["What's okay~", "Come here~!", "-Bam!Bam!-"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args![
                                "Aah~Ugh~ Don't do that~",
                                "Aa..Aaaahhhh~~!!",
                                "Argh~",
                                "-Sound of something broken-",
                                "Owww~~",
                                "*Sobs*~~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Vanessa", args!["Hmm...Did I do it too strong.", "Mm~"])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["Well obviously!", "Idiot!Idiot!!", "*sobs*~"])?;
                        ctx.next()?;
                        ctx.lines_as("Vanessa", args!["Umm~umm~", "Sorry~", "Okay, okay,", "Don't cry."])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["*sob*~"])?;
                        ctx.next()?;
                        ctx.lines_as("Vanessa", args!["Okay~ okay.."])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["*sob*~"])?;
                        ctx.next()?;
                        ctx.lines_as("Vanessa", args!["Stop! Arrgh!"])?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["........"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "Fine, okay.",
                                "I'm sorry what happened",
                                "I'll make a weapon for you",
                                "if you gather some",
                                "materials..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "I'm Vanessa Louise.",
                                "Originally a Martial Artist Applicant",
                                "but I took a wrong turn",
                                "and I'm stuck making weapons.",
                                "Oh my god~oh my~ my miserable life~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args!["Now onto the subject.", "The weapon I can create is", "called the Destroyer."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "I especially named it",
                                "after a technique I used in an arena",
                                "Hehe~",
                                "Take this~!Destroyer~"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(ctx.player().name()?, args!["*sob*~ Please~Stop~"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args!["Ah uh..Sorry...", "First Destroyer needs", "materials before creating it."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args!["A considerable amount of items is needed", "so think carefully."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "How about it? This kind of",
                                "opportunity is not common.",
                                "Want to make a request?"
                            ],
                        )?;
                        ctx.next()?;
                        'b2: {
                            let subject2 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Maybe next time."), Val::from("Okay.")],
                            )?);
                            let mut matched2 = false;
                            if !matched2 && subject2 == 1 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Vanessa",
                                    args![
                                        "Mm~ Is that so~",
                                        "It will be an opportunity for you.",
                                        "You'll regret it~",
                                        "Then see you next time~",
                                        "Bye~Bye~"
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            if !matched2 && subject2 == 2 {
                                matched2 = true;
                            }
                            if matched2 {
                                ctx.lines_as(
                                    "Vanessa",
                                    args![
                                        "Mm, Okay.",
                                        "To make a Destroyer,",
                                        "You need 50 Used Iron Plates",
                                        "5 Oridecons, 70 Rusty Old Screws",
                                        "and a fee of 100,000 zeny.",
                                        "You must bring me these exact materials. Understood?"
                                    ],
                                )?;
                                ctx.var("gunst").set(1)?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                    if !matched1 && subject1 == 1 {
                        matched1 = true;
                    }
                    if matched1 {
                        vanessa_run(ctx, VanessaStep::SubDestroyer, vec![])?;
                    }
                    if !matched1 && subject1 == 2 {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Vanessa",
                            args![
                                "Oh~ You're back.?",
                                "How are you nowadays?",
                                "There's a new item in the store.",
                                "Go ahead and choose."
                            ],
                        )?;
                        ctx.next()?;
                        'b3: {
                            let subject3 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Destroyer"), Val::from("Slotted Destroyer"), Val::from("Cancel")],
                            )?);
                            let mut matched3 = false;
                            if !matched3 && subject3 == 1 {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Vanessa",
                                    args![
                                        "Yeah, that's nice",
                                        "That's a normal Destroyer.",
                                        "It takes 50 Used Iron Plates,",
                                        "5 Oridecons, 70 Rusty Old Screws,",
                                        "and 100,000 Zeny to make it.",
                                        "You have to bring me the correct amount of materials.",
                                        "Do you want it?"
                                    ],
                                )?;
                                ctx.next()?;
                                'b4: {
                                    let subject4 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Maybe next time."), Val::from("Okay.")],
                                    )?);
                                    let mut matched4 = false;
                                    if !matched4 && subject4 == 1 {
                                        matched4 = true;
                                    }
                                    if matched4 {
                                        ctx.lines_as(
                                            "Vanessa",
                                            args![
                                                "What~ C'mon",
                                                "What's there to think about?",
                                                "Ok, well, think about it",
                                                "and come back..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched4 && subject4 == 2 {
                                        matched4 = true;
                                    }
                                    if matched4 {
                                        vanessa_run(ctx, VanessaStep::SubDestroyer, vec![])?;
                                    }
                                }
                            }
                            if !matched3 && subject3 == 2 {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Vanessa",
                                    args![
                                        "Oh~ The Slotted Destroyer~",
                                        "Unlike the normal Destroyer,",
                                        "I want you to find me one of the rare items I'm collecting.",
                                        "Then I'll give it to you."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Vanessa",
                                    args![
                                        "I'm looking for a Finger[2].",
                                        "I want 5 Oridecons too.",
                                        "You have to bring me the correct amount of materials.",
                                        "Do you still want a Slotted Destroyer?"
                                    ],
                                )?;
                                ctx.next()?;
                                'b5: {
                                    let subject5 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("I'll think about it."), Val::from("OK! I do!")],
                                    )?);
                                    let mut matched5 = false;
                                    if !matched5 && subject5 == 1 {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        ctx.lines_as(
                                            "Vanessa",
                                            args![
                                                "What~ C'mon",
                                                "What's there to think about?",
                                                "Ok, well, think about it",
                                                "and come back..."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                    if !matched5 && subject5 == 2 {
                                        matched5 = true;
                                    }
                                    if matched5 {
                                        vanessa_run(ctx, VanessaStep::SubDestroyer2, vec![])?;
                                    }
                                }
                            }
                            if !matched3 && subject3 == 3 {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.lines_as(
                                    "Vanessa",
                                    args!["Hmm~ Yeah~", "Take your time.", "Maybe you'd like to spar with me?"],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                        }
                    }
                }
                step = VanessaStep::SubDestroyer;
                continue 'machine;
            }
            VanessaStep::SubDestroyer => {
                if ctx.items().count(7319)? < 50 || ctx.items().count(7317)? < 70 || ctx.items().count(984)? < 5 {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "Yeah, the required materials are 50 Used Iron Plates,",
                            "5 Oridecons, 70 Rusty Old Screws, and 100,000 Zeny.",
                            "Keep in mind that you have to bring the correct amount."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.player().zeny()? < 100000 {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "You've brought plenty of material,",
                            "But the fee is 100,000.",
                            "Keep that in mind."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.call(Function::CheckWeight, args![13160, 1])? != 1 {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "It looks like you wouldn't be able to carry my creation with you even if I made it.",
                            "Go and empty your inventory a bit."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "Okay~ Very well~",
                            "All Checked~",
                            "You've got the perfect materials and Zeny~",
                            "Here's the Destroyer I made ahead of time for you.",
                            "Use it well."
                        ],
                    )?;
                    ctx.player().set_zeny(ctx.player().zeny()? - 100000)?;
                    ctx.items().take(7319, 50)?;
                    ctx.items().take(984, 5)?;
                    ctx.items().take(7317, 70)?;
                    ctx.items().give(13160, 1)?;
                    if ctx.var("gunst").get()? == 1 {
                        ctx.var("gunst").set(2)?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "If you ever need one again later,",
                            "Come and fine me anytime~",
                            "Next time I'll cast a different kind of bare hand technique."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
            VanessaStep::SubDestroyer2 => {
                if ctx.items().count(1812)? < 1 || ctx.items().count(984)? < 5 {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "Yeah, the required materials are 1 Finger[2]",
                            "and 5 Oridecons.",
                            "Keep in mind that the materials need to be exact."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if ctx.call(Function::CheckWeight, args![13161, 1])? != 1 {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "There's no space in your inventory.",
                            "Even if I made you one,",
                            "You wouldn't be able to carry it",
                            "Come back after you've cleared out your inventory."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "Okay~ Very well~",
                            "All Checked~",
                            "You've got the perfect materials and Zeny~",
                            "Here's the Slotted Destroyer I made ahead of time for you.",
                            "Use it well."
                        ],
                    )?;
                    ctx.items().take(1812, 1)?;
                    ctx.items().take(984, 5)?;
                    ctx.items().give(13161, 1)?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Vanessa",
                        args![
                            "If you ever need one again later,",
                            "Come and fine me anytime~",
                            "Next time I'll cast a different kind of bare hand technique."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn vanessa(ctx: &Ctx) -> Script {
    vanessa_run(ctx, VanessaStep::Start, Vec::new()).map(|_| ())
}

pub fn lab_director(ctx: &Ctx) -> Script {
    if ctx.var("gun_na").get()? == 1 {
        if ctx.items().count(1043)? > 999 && ctx.items().count(932)? > 999 {
            ctx.items().take(1043, 1000)?;
            ctx.items().take(932, 1000)?;
            ctx.var("gun_na").set(2)?;
            ctx.lines_as(
                "N. A",
                args![
                    "Ahh, it's all here! Ahh, and I",
                    "was worried about that no one",
                    "would be able to handle my ^ff0000Butcher^000000",
                    "when I've finished creating it!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "Alright, I'll give the ^ff0000Butcher^000000 to you.",
                    "However, we don't do work for",
                    "free, so we need to charge you",
                    "for it. Also, you need to obtain",
                    "permission to use the Butcher from",
                    "Lady Celena."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "Once you get the permission, I will",
                    "give the Butcher to you, after paying",
                    "the fee of 100000 zeny."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "N. A",
                args![
                    "Have you found ^ff00001000 Orc Claw^000000 and ^ff00001000 Skel Bone^000000 yet?",
                    "If you think it's too difficult,",
                    "you can choose to give up."
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("I'm not giving up!:I give up...")])?) == 1 {
                ctx.lines_as("N. A", args!["Alright, I trust you.", "Good luck."])?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "N. A",
                    args![
                        "You're giving up huh?",
                        "Well, I'll admit that the",
                        "test is quite difficult, but",
                        "you can't handle this weapon",
                        "if you can't handle the test.",
                        "You may come back later to",
                        "take the challenge again."
                    ],
                )?;
                ctx.var("gun_na").set(0)?;
                return ctx.close();
            }
        }
    } else if ctx.var("gun_na").get()? == 2 {
        ctx.lines_as(
            "N. A",
            args![
                "I already got Lady Celena's",
                "permission to let you use the",
                "Butcher. You can use it once",
                "you've paid me 100000 zeny.",
                "Do you want to pay now?"
            ],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Nope.:Yeah.")])?) == 1 {
            ctx.lines_as("N. A", args!["Alright. I await you to return", "with the money."])?;
            return ctx.close();
        } else {
            if ctx.player().zeny()? < 100000 {
                ctx.lines_as(
                    "N. A",
                    args![
                        "Huh, I don't think you have",
                        "enough money on you.",
                        "Come back with the money,",
                        "alright?"
                    ],
                )?;
                return ctx.close();
            }
            if ctx.call(Function::CheckWeight, args![13158, 1])? == 0 {
                ctx.lines_as(
                    "N. A",
                    args![
                        "You are overweight.",
                        "Even if I give you the",
                        "weapon, you cannot carry it.",
                        "Please clear your inventory."
                    ],
                )?;
                return ctx.close();
            }
            ctx.player().set_zeny(ctx.player().zeny()? - 100000)?;
            ctx.var("gun_na").set(0)?;
            ctx.items().give(13158, 1)?;
            ctx.lines_as(
                "N. A",
                args![
                    "One, two, three, four, five,",
                    "six... 99997, 99998, 99999...",
                    "100000. *ding~!* Very well!",
                    "The fee is clear now. You may",
                    "take the ^ff0000Butcher^000000 now!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "Mr. F. Harrison from Lighthalzen",
                    "is quite interested in your new",
                    "toy there. Show it to him some",
                    "time. He'll be glad."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("N. A", args!["Make good use of it!", "See ya!"])?;
            return ctx.close();
        }
    } else if ctx.var("gun_na").get()? == 10 {
        if ctx.items().count(999)? > 69
            && ctx.items().count(985)? > 4
            && ctx.items().count(984)? > 2
            && ctx.items().count(1003)? > 69
            && ctx.items().count(7317)? > 49
            && ctx.player().zeny()? >= 50000
        {
            ctx.items().take(999, 70)?;
            ctx.items().take(985, 5)?;
            ctx.items().take(984, 3)?;
            ctx.items().take(1003, 70)?;
            ctx.items().take(7317, 50)?;
            ctx.player().set_zeny(ctx.player().zeny()? - 50000)?;
            ctx.var("gun_na").set(11)?;
            ctx.lines_as(
                "N. A",
                args![
                    "Aha, you got me all the",
                    "materials. Here, let me get down",
                    "to it right away! It'll take",
                    "some time, so wait up..."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as("N. A", args!["Eh? You don't have the materials", "with you yet?"])?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "To make Drifter, I will need",
                    "70 Steel, 5 Elunium,",
                    "3 Oridecon, 70 Coal, 50 Rusty",
                    "Screws, and also a fee of",
                    "50000 zeny.",
                    "Come back to me once you have",
                    "everything ready."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("N. A", args!["If you don't want it anymore,", "you can cancel the request."])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Don't cancel.:Cancel it.")])?) == 1 {
                ctx.lines_as("N. A", args!["Well, please come back with the", "materials. I'll be waiting."])?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "N. A",
                    args![
                        "Alright, request to make",
                        "a Drifter for you is cancelled.",
                        "I wish you good luck",
                        "in your future."
                    ],
                )?;
                ctx.var("gun_na").set(0)?;
                return ctx.close();
            }
        }
    } else if ctx.var("gun_na").get()? == 11 {
        if ctx.call(Function::CheckWeight, args![13157, 1])? == 0 {
            ctx.lines_as(
                "N. A",
                args![
                    "You are overweight.",
                    "Even if I made you the",
                    "weapon, you cannot carry it.",
                    "Please clear your inventory."
                ],
            )?;
            return ctx.close();
        }
        ctx.var("gun_na").set(0)?;
        ctx.items().give(13157, 1)?;
        ctx.lines_as("N. A", args!["Ahh, here's the completed", "Drifter for you."])?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args![
                "Please learn to use the",
                "Gatlings well. The crazy",
                "destruction will definitely",
                "be mentally helpful to you."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("gun_na").get()? == 101 {
        ctx.lines_as(
            "N. A",
            args![
                "Ah, hello?",
                "I'm the Coordinator of",
                "Einbroch Weapon Development.",
                "My name is 'Lab Director'.",
                "Do you need something?"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("I need a Special Metal Rod.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "N. A",
            args![
                "Huh...? Hey, weren't you that guy",
                "who walked out of here with a",
                "Butcher a while ago? How was the",
                "Butcher?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args![
                "...... WHAT!? YOU BROKE IT!?",
                "I thought you would be able to",
                "use it well... You disappoint me!!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args!["You betrayed my faith in you!", "You traitor! Traitor!! TRAITOR!!!!"],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Explain everything.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "N. A",
            args![
                "...... Whew.",
                "So it was Mr. F. Harrison who",
                "broke it, huh? I'm sorry, I should",
                "not have suspected you."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args![
                "Lady Celena has the special metal",
                "rod that you want, but the doc is",
                "not in right now, and no one else",
                "knows where it is..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args![
                "I have other research right now",
                "so I can't waste my time looking",
                "for that... Grr... I need the",
                "Elemental Spheres to keep going",
                "with my research..."
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Make him an offer.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("N. A", args!["An offer? Like what?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("We find things for each other.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "N. A",
            args![
                "Hmm... That sounds good.",
                "Well, according to the offer,",
                "I'll look for the rod for you,",
                "while you find me those",
                "Elemental Spheres."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args![
                "30 Poison Sphere, 30 Flare Sphere,",
                "30 Lightning Sphere, 30 Blind",
                "Sphere, or 30 Freezing Sphere.",
                "Find me 30 of each Element."
            ],
        )?;
        ctx.var("gun_na").set(102)?;
        return ctx.close();
    } else if ctx.var("gun_na").get()? == 102 {
        ctx.lines_as(
            "N. A",
            args![
                "30 Poison Sphere, 30 Flare Sphere,",
                "30 Lightning Sphere, 30 Blind",
                "Sphere, or 30 Freezing Sphere.",
                "Find me 30 of each Element.",
                "Did you find them?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from(
                    "Nope.:I found 30 Poison Spheres.:I found 30 Flare Spheres.:I found 30 Lightning Spheres.:I found 30 Blind Spheres.:I found 30 Freezing Spheres.",
                )],
            )?);
            let mut matched1 = false;
            if !matched1 && subject1 == 1 {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("N. A", args!["Just bring me whatever type", "you could find."])?;
                return ctx.close();
            }
            if !matched1 && subject1 == 2 {
                matched1 = true;
            }
            if matched1 {
                if ctx.items().count(13205)? >= 30 {
                    ctx.items().take(13205, 30)?;
                    ctx.var("gun_na").set(103)?;
                }
                break 'b1;
            }
            if !matched1 && subject1 == 3 {
                matched1 = true;
            }
            if matched1 {
                if ctx.items().count(13203)? >= 30 {
                    ctx.items().take(13203, 30)?;
                    ctx.var("gun_na").set(103)?;
                }
                break 'b1;
            }
            if !matched1 && subject1 == 4 {
                matched1 = true;
            }
            if matched1 {
                if ctx.items().count(13204)? >= 30 {
                    ctx.items().take(13204, 30)?;
                    ctx.var("gun_na").set(103)?;
                }
                break 'b1;
            }
            if !matched1 && subject1 == 5 {
                matched1 = true;
            }
            if matched1 {
                if ctx.items().count(13206)? >= 30 {
                    ctx.items().take(13206, 30)?;
                    ctx.var("gun_na").set(103)?;
                }
                break 'b1;
            }
            if !matched1 && subject1 == 6 {
                matched1 = true;
            }
            if matched1 {
                if ctx.items().count(13207)? >= 30 {
                    ctx.items().take(13207, 30)?;
                    ctx.var("gun_na").set(103)?;
                }
                break 'b1;
            }
        }
        if ctx.var("gun_na").get()? != 103 {
            ctx.lines_as(
                "N. A",
                args!["Eh? What? Am I the only", "person who can't see them?", "Bring me more!"],
            )?;
            return ctx.close();
        }
        ctx.lines_as(
            "N. A",
            args![
                "Wow, you found them all for me!",
                "I had to turn Lady Celena's lab",
                "over to find this rod too.",
                "Let's trade then!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "-You gave the Elemental",
            "Spheres to Research Coordinator and got",
            "the Metal Rod in return.-"
        ])?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args![
                "Mr. F. Harrison is very good.",
                "I'm sure he can fix your",
                "Butcher for you.",
                "Well, see you later!"
            ],
        )?;
        return ctx.close();
    } else if ctx.var("gun_na").get()? == 103 || ctx.var("gun_na").get()? == 104 {
        ctx.lines_as(
            "N. A",
            args![
                "Mr. F. Harrison is very good.",
                "I'm sure he can fix your",
                "Butcher for you.",
                "Well, see you later!"
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "N. A",
        args![
            "Ah, hello?",
            "I'm the Coordinator of",
            "Einbroch Weapon Development.",
            "My name is 'Lab Director'.",
            "Do you need something?"
        ],
    )?;
    ctx.next()?;
    if ctx.var("BaseJob").get()? != constants::JOB_GUNSLINGER {
        let choice = runtime::select_values(ctx, &[Val::from("Talk to him.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "N. A",
            args![
                "If you see Gunslingers around,",
                "please tell them that I have",
                "the latest news on weapons.",
                "If necessary, please send them",
                "here. Heheheheh..."
            ],
        )?;
        return ctx.close();
    }
    if ctx.player().base_level()? < 55 {
        let choice = runtime::select_values(ctx, &[Val::from("Talk.")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "N. A",
            args!["... Destruction... Madness...", "Hmm... Attack speed over 180..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "N. A",
            args!["Ah, sorry, I'm developing some", "new weapons. Please don't", "disturb me."],
        )?;
        return ctx.close();
    }
    'b2: {
        let subject2 = Val::from(runtime::select_values(
            ctx,
            &[(if ctx.player().base_level()? > 67 {
                Val::from("Ask about 'Butcher'.")
            } else {
                Val::from("")
            }) + Val::from(":Ask about the 'Drifter'.:Cancel")],
        )?);
        let mut matched2 = false;
        if !matched2 && subject2 == 1 {
            matched2 = true;
        }
        if matched2 {
            ctx.lines_as(
                "N. A",
                args!["Ah, you heard the news", "shortly after the end", "of the development eh?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "^ff0000Butcher^000000 is the newest development",
                    "by us, the Einbroch Firearm Lab.",
                    "It is the newest type of Gatling",
                    "we have developed. While",
                    "^ff0000Drifter^000000 is a good weapon",
                    "as well, the ^ff0000Butcher^000000 definitely has",
                    "a much stronger firepower!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args!["Due to its strong firepower,", "the Butcher was also dubbed", "as the 'Murderer'."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "However, even Gunslingers could",
                    "have troubles controlling such",
                    "a powerful weapon."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("N. A", args!["Do you think you can handle it?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("I'm not sure...:Of course I can!!")])?) == 1 {
                ctx.lines_as(
                    "N. A",
                    args![
                        "The ^ff0000Butcher^000000 is a weapon that you",
                        "can't handle without a strong",
                        "will. I'll see you again when",
                        "you have enough confidence to",
                        "handle this monster."
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "N. A",
                    args![
                        "Aha, how self-confident you",
                        "are! However, I see people with",
                        "such self-confidence everywhere.",
                        "You'll need to prove it...",
                        "But how... Hmm..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("N. A", args!["Hmmm......"])?;
                ctx.next()?;
                ctx.lines_as("N. A", args!["Aha! I got it!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "N. A",
                    args![
                        "Here, bring me ^ff00001000 Orc Claws^000000 and ^ff00001000 Skel Bone^000000.",
                        "If you can bring me these items,",
                        "I'll let you use the Butcher."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("N. A", args!["Easy, ain't it? Go and prove", "your ability then! Heheheh..."])?;
                ctx.var("gun_na").set(1)?;
                return ctx.close();
            }
        }
        if !matched2 && subject2 == 2 {
            matched2 = true;
        }
        if matched2 {
            ctx.lines_as("N. A", args!["Ah, you're here to learn about", "the ^ff0000Drifter^000000 eh?"])?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "The ^ff0000Drifter^000000 is one of the many",
                    "highest-classed weapons developed",
                    "by Lady Celena. It's an automatic",
                    "Gatling which boasts very high",
                    "rate of fire, which is the highest",
                    "among all Gunslinger weapons."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "N. A",
                args![
                    "Of course, you will need the Skill",
                    "to allow you to control Gatlings,",
                    "but he who contorls Gatlings well",
                    "will receive full aid from the",
                    "^ff0000Drifter^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("N. A", args!["Do you want to try using the ^ff0000Drifter^000000?"])?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("Um, no.:Yeah!")])?) == 1 {
                ctx.lines_as("N. A", args!["If you want to try it out", "some time, come back here."])?;
                return ctx.close();
            } else {
                ctx.var("gun_na").set(10)?;
                ctx.lines_as(
                    "N. A",
                    args![
                        "Since it's really hard to find",
                        "the materials for constructing",
                        "the ^ff0000Drifter^000000, we only make them",
                        "on requests, and we require those",
                        "who want to use the ^ff0000Drifter^000000 to bring",
                        "us the materials."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "N. A",
                    args![
                        "I'll tell you the materials",
                        "needed, just bring them and",
                        "we'll construct it for you."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "N. A",
                    args![
                        "To make a Drifter, we need",
                        "70 Steel, 5 Eluniums, 3",
                        "Oridecons, 70 Coal, 50 Rusty",
                        "Screws, and a fee of",
                        "50,000 zeny.",
                        "Come back after you found",
                        "them all."
                    ],
                )?;
                return ctx.close();
            }
        }
        if !matched2 && subject2 == 3 {
            matched2 = true;
        }
        if matched2 {
            ctx.lines_as(
                "N. A",
                args![
                    "I have news on the latest",
                    "weapons but... I guess you're",
                    "too busy to hear them.",
                    "Maybe next time I guess."
                ],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn f_harrison(ctx: &Ctx) -> Script {
    if ctx.var("gun_na").get()? == 100 {
        if ctx.call(Function::CheckWeight, args![13102, 1])? == 0 {
            ctx.lines_as(
                "F. Harrison",
                args![
                    "You are overweight.",
                    "Even if I gave you the",
                    "weapon, you cannot carry it.",
                    "Please clear your inventory."
                ],
            )?;
            return ctx.close();
        }
        ctx.lines_as("F. Harrison", args!["Hmm... Oh?", "Ahh...... Eh?"])?;
        ctx.next()?;
        ctx.lines_as("F. Harrison", args!["Heheh... Hmm... Huh...?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("May I have my Butcher back now?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as("F. Harrison", args!["Uh? Oh, umm... Heheh..."])?;
        ctx.next()?;
        ctx.lines_as("F. Harrison", args!["Th-that, huh? Eh... Heheh..."])?;
        ctx.next()?;
        ctx.lines_as("F. Harrison", args!["Ahahahah! I uh..."])?;
        ctx.next()?;
        ctx.lines_as("F. Harrison", args!["That thing! BOOM!"])?;
        ctx.next()?;
        ctx.lines_as("F. Harrison", args!["Bam bam! I was gonna fire it!", "Yeah!"])?;
        ctx.next()?;
        ctx.lines_as(
            "F. Harrison",
            args!["But it resisted! So I went 'BAM!' with", "my hand! MUAHAHAHAH!!"],
        )?;
        ctx.next()?;
        ctx.lines_as("F. Harrison", args!["And it broke...... *sob sob*"])?;
        ctx.next()?;
        ctx.lines_as(
            "F. Harrison",
            args![
                "I call myself 'Dr. Everything' and",
                "I wanted to fix it myself, but I",
                "don't seem to have the materials",
                "to fix it... And the materials are",
                "so rare too... *sob sob*"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "F. Harrison",
            args![
                "Umm... I'm sorry for breaking",
                "it but... Could you get me the",
                "materials I need for fixing it?",
                "Bring me those items, and I can",
                "fix it, plus I'll modify it for",
                "you! I promise!!"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args!["So please bring me the materials...", "*sob sob sob*"])?;
        ctx.next()?;
        ctx.lines_as(
            "F. Harrison",
            args![
                "The materials I need are",
                "10 Steel, 2 Eluniums,",
                "1 Oridecon, 20 Coal...",
                "And a Special Metal Rod",
                "used in the Butcher...",
                "I think only Lady Celena can",
                "make those rods......"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "F. Harrison",
            args![
                "But first, you'll need to talk",
                "to Lady Celena's assistant, the 'Lab Director',",
                "and ask for his help.",
                "That kid's a bit hysterical, but",
                "she'll be nice help if you talk",
                "to her nicely..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "F. Harrison",
            args![
                "I'm really sorry... Here, I'll",
                "lend you my prized weapon,",
                "'Crimson Bolt'. But make sure",
                "you bring all those materials",
                "to me once you find them all!",
                "Promise me!!!"
            ],
        )?;
        ctx.var("gun_na").set(101)?;
        ctx.items().give(13102, 1)?;
        return ctx.close();
    } else if ctx.var("gun_na").get()? == 101 || ctx.var("gun_na").get()? == 102 {
        ctx.lines_as(
            "F. Harrison",
            args![
                "The materials I need are",
                "10 Steel, 2 Eluniums,",
                "1 Oridecon, 20 Coal...",
                "And a Special Metal Rod",
                "used in the Butcher...",
                "Talk to the 'Lab Director' for his",
                "assistance on the Rod."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("gun_na").get()? == 103 {
        if ctx.items().count(999)? >= 10 && ctx.items().count(985)? >= 2 && ctx.items().count(984)? >= 1 && ctx.items().count(1003)? >= 20 {
            if ctx.items().count(13102)? > 0 {
                ctx.items().take(999, 10)?;
                ctx.items().take(985, 2)?;
                ctx.items().take(984, 1)?;
                ctx.items().take(1003, 20)?;
                ctx.items().take(13102, 1)?;
                ctx.var("gun_na").set(104)?;
                ctx.lines_as(
                    "F. Harrison",
                    args![
                        "Aha! You got all the materials",
                        "for me! I'll get to the repair",
                        "right away, please hold on..."
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "F. Harrison",
                    args![
                        "Aha! You got all the materials",
                        "for me!",
                        "... But where's my treasured",
                        "'Crimson Bolt'!? I will not fix",
                        "your Butcher for you if you don't",
                        "bring it back to me!!"
                    ],
                )?;
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "F. Harrison",
                args![
                    "The materials I need are",
                    "10 Steel, 2 Eluniums,",
                    "1 Oridecon, 20 Coal...",
                    "And a Special Metal Rod",
                    "used in the Butcher...",
                    "Good thing you found the",
                    "Special Metal Rod already."
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("gun_na").get()? == 104 {
        if ctx.call(Function::CheckWeight, args![13159, 1])? == 0 {
            ctx.lines(args![
                "-You're overweight already.-",
                "-Come back after dropping some",
                "stuff first.-"
            ])?;
            return ctx.close();
        }
        ctx.var("gun_na").set(0)?;
        ctx.items().give(13159, 1)?;
        ctx.lines_as(
            "F. Harrison",
            args![
                "Whew... Finally it's fixed, thanks",
                "to your effort. I'm very sorry",
                "for breaking it, and thank you",
                "for your effort. I feel guilty",
                "for just fixing it, so I added",
                "some extra power on it. I hope",
                "it'll work nicely for you.",
                "Well, enjoy it."
            ],
        )?;
        return ctx.close();
    } else if ctx.var("gun_na").get()? == 0 {
        if ctx.var("BaseJob").get()? != constants::JOB_GUNSLINGER {
            ctx.lines_as(
                "F. Harrison",
                args!["Man, I'm so bored...", "I wonder if there's anything", "interesting..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "F. Harrison",
                args!["Oh well... I'll just play with my", "beautiful 'Crimson Bolt'."],
            )?;
            return ctx.close();
        }
        if ctx.player().base_level()? > 67 && ctx.items().count(13158)? > 0 {
            ctx.lines_as(
                "F. Harrison",
                args!["Man, I'm so bored...", "I wonder if there's anything", "interesting..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "F. Harrison",
                args!["Eh? Are you a Gunslinger?", "I haven't seen that weapon", "in your hand before..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "F. Harrison",
                args![
                    "May I take a look? Please?",
                    "C'mon, let me take a look!",
                    "I'm bored out of my mind here!",
                    "Pleeeeeeeeeeeease~?"
                ],
            )?;
            ctx.next()?;
            if Val::from(runtime::select_values(ctx, &[Val::from("No way.:Here...")])?) == 1 {
                ctx.lines_as(
                    "F. Harrison",
                    args![
                        "Hah! You think you're the only",
                        "one with a cool weapon!? Well",
                        "I got my beautiful Crimson Bolt!",
                        "Hmph!!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "F. Harrison",
                args![
                    "Heheh... This is the newest",
                    "development by Einbroch Firearms",
                    "Lab, the so-called uncontrollable",
                    "'Destroyer Butcher', eh?",
                    "Interesting... VERY interesting..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "F. Harrison",
                args![
                    "Hey, let me try it out for a",
                    "bit, alright? Don't worry, I",
                    "know how to handle weapons,",
                    "I won't break it!",
                    "That's that! Let's go~"
                ],
            )?;
            ctx.items().take(13158, 1)?;
            ctx.var("gun_na").set(100)?;
            return ctx.close();
        }
    }
    Ok(())
}
