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

pub fn myu_08_hat(ctx: &Ctx) -> Script {
    ctx.lines_as("Myu", args!["Meow..."])?;
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.next()?;
    if ctx.var("hatcat2008").get()? == 0 {
        ctx.lines_as(
            "Myu",
            args![
                "Oh? Aren't you an adventurer?",
                "What brings you here? Ho, you are not here to harm Wild Roses, are you?",
                "(Meow!)"
            ],
        )?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Meow..?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Myu",
            args![
                "Ah, Never mind that.",
                "This place is the village of Deserted people.",
                "Homeland of those who walk in the shadows and lay low."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args![
                "....And yet, our peace is being disturbed by those barking dogs.",
                "That's right, Adventurer.",
                "Would you be so kind and punish those Kobold Archers?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args!["They have their own territory south of here, but they're still trying to invade ours."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args!["How Impudent. I tried to talk to them in a peaceful way, or even fight them off, but it was of no use."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args!["So what I'm asking you is... to drive them out of our territory by using all means necessary.."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args!["If you manage to do that, I will give you something precious which I really adore."],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(
                ctx,
                &[Val::from("That sounds troublesome..:What should I do?")],
            )?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Myu",
                    args![
                        "Hmph, yeah, it was kinda funny huh.",
                        "Anybody who comes here....they all come to harm the Wild Roses..",
                        "I failed to notice that."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Myu", args!["...It was a fruitless effort Meow..."])?;
                ctx.next()?;
                ctx.lines_as(ctx.player().name()?, args![" (Meow again...What a weirdo.)"])?;
                return ctx.close();
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Myu",
                    args!["Of course take care of those Kobold Archers Meow!", " ", "(Meow!)"],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("Meow again?!")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as(
                    "Myu",
                    args![
                        "Hmm Hmm I told you to never mind that.",
                        "So as I was saying...",
                        "I mean... meow... er..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Myu",
                    args![
                        "Exactly 1,000 times no more, no less.",
                        "Give them 1,000 warnings and they will back off."
                    ],
                )?;
                ctx.next()?;
                let choice = runtime::select_values(ctx, &[Val::from("What do you mean by 1,000 warnings...?")])?;
                ctx.var("@menu").set(choice)?;
                ctx.lines_as("Myu", args!["Hu~Do I have to spell it out for you?"])?;
                ctx.next()?;
                ctx.mes("As she says that, Myu slides her neck mocking like she's slitting her throat.")?;
                ctx.next()?;
                let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Isn't that too much?:That's easy")])?);
                if subject2 == 1 {
                    ctx.lines_as(
                        "Myu",
                        args![
                            "..But they won't give up if we dont really teach them a lesson.",
                            "Well If you think it's too much for you, so be it."
                        ],
                    )?;
                    return ctx.close();
                } else if subject2 == 2 {
                    ctx.lines_as(
                        "Myu",
                        args![
                            "But you have to keep one thing in mind and it is",
                            "obiously, NEVER EVER do any harm to Wild Roses."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Myu",
                        args![
                            "If you do, it's all over for you.",
                            "So you better be careful.. ",
                            "Meow Meow Meow.."
                        ],
                    )?;
                    ctx.var("hatcat2008").set(Val::from(1))?;
                    ctx.quests().start(7054)?;
                    ctx.quests().start(7055)?;
                    return ctx.close();
                }
            }
        }
    } else if ctx.var("hatcat2008").get()? == 1 {
        if ctx.call(Function::CheckQuest, args![7055, constants::HUNTING])? == 2 {
            ctx.lines_as("Myu", args!["Did you think I didn't know what you have done?", "Huh?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Myu",
                args![
                    "How..just how could you kill those precious children..?",
                    "And you were saying that you are on our side?!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Myu", args!["Our business is over!!!", " ", "(Meow..)"])?;
            ctx.var("hatcat2008").set(Val::from(0))?;
            ctx.quests().erase(7054)?;
            ctx.quests().erase(7055)?;
            return ctx.close();
        } else if ctx.call(Function::CheckQuest, args![7054, constants::HUNTING])? == 2 {
            ctx.lines_as(
                "Myu",
                args![
                    "Oh..wow unbelievable!",
                    "Now, those Kobolds should have learned a lesson, haven't they?",
                    "Meow Meow Meow!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Myu",
                args![
                    "Here, Take this.",
                    "This is the Seal of our 'Brave Kitty Cats'.",
                    "This is soooo valuable, so don't lose it."
                ],
            )?;
            ctx.next()?;
            let choice = runtime::select_values(ctx, &[Val::from("This is it?")])?;
            ctx.var("@menu").set(choice)?;
            ctx.lines_as(
                "Myu",
                args![
                    "What more did you expect?",
                    "That Pow is really a valuable thing!",
                    "Now, just take it and leave. Wild Roses feel uncomfortable with an adventurer around them."
                ],
            )?;
            ctx.var("hatcat2008").set(Val::from(2))?;
            ctx.items().give(5446, 1)?;
            ctx.quests().erase(7054)?;
            ctx.quests().erase(7055)?;
            return ctx.close();
        }
        ctx.lines_as(
            "Myu",
            args!["You get it now, huh?", "ONE THOUSAND TIMES! -chuckles-", "(Meow Meow Meow~)"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.player().name()?,
            args![
                "(What a weirdo.. Anyways..what's with that meow meow sound?)",
                "Just keep your promises.",
                "after the work is done...Ok?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Myu", args!["Of course! Don't worry about that~"])?;
        return ctx.close();
    } else if ctx.var("hatcat2008").get()? == 2 {
        ctx.lines_as(
            "Myu",
            args![
                "This place is the village of Deserted people.",
                "Homeland of those who walk in the shadows and lay low."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args![
                "Oh, Aren't you the Adventurer who took the Seal of our 'Brave Kitty Cats'.",
                "What brings you here?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Myu",
            args![
                "Oh yeah, right, even after all the threats we gave the Kobolds..",
                "our situation hasn't changed at all. Would you mind helping us once more?"
            ],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "Myu slides her neck mocking like she's slitting her throat.",
            "Her mouth looks like it's saying.",
            " 'O.N.E. T.H.O.U.S.A.N.D.' "
        ])?;
        ctx.next()?;
        let subject3 = Val::from(runtime::select_values(ctx, &[Val::from("NO!:Sure.")])?);
        if subject3 == 1 {
            ctx.lines_as(
                "Myu",
                args![
                    "...!...",
                    ".. Ah.. that's too bad.",
                    "Well, looks like a bait has spoiled.... ",
                    "It's ok. That's how it is. Meow."
                ],
            )?;
            return ctx.close();
        } else if subject3 == 2 {
            ctx.lines_as(
                "Myu",
                args![
                    "You know the drill right?",
                    "Never ever touch the Wild Roses, Only hunt down the Kobold Archers.",
                    "Give them 1,000 times despair!"
                ],
            )?;
            ctx.var("hatcat2008").set(Val::from(1))?;
            ctx.quests().start(7054)?;
            ctx.quests().start(7055)?;
            return ctx.close();
        }
    }
    ctx.lines_as("Myu", args!["Meow..", "This Sunshine...makes me sleepy.."])?;
    return ctx.close();
}

pub fn trainee_2008hat01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Trainee Elgo",
        args![
            "Welcome.",
            "Please talk to our teacher if you are here to make a dyes.",
            "And talk to my friend next to me if it is about the delivery."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("And just what are you doing?:I see.")],
        )?);
        let mut matched1 = false;
        let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Elgo", args!["Well, I dye clothes.", "like handkerchiefs, or ribbons."])?;
            ctx.next()?;
            ctx.lines_as(
                "Elgo",
                args![
                    "I can dye small things really pretty..",
                    "Such as a ^4d4dffCute Ribbon^000000.",
                    "Bring me a Cute Ribbon if you want to dye it."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Elgo",
                args![
                    "It'd be a good chance for me to practice one,",
                    "and you won't have to pay a single zeny.",
                    "Oh yeah, but I will at least need the required^4d4dffdyestuffs^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Elgo",
                args![
                    "Of course, I won't fail.",
                    "I will put all my efforts into it.",
                    "I want to challenge my skills!!"
                ],
            )?;
            ctx.next()?;
            if ((((ctx.items().count(2250)? > 0 || ctx.items().count(5441)? > 0) || ctx.items().count(5439)? > 0)
                || ctx.items().count(5440)? > 0)
                || ctx.items().count(5438)? > 0)
            {
                ctx.lines_as(ctx.player().name()?, args!["(I have a Cute Ribbon...what should I do..?)"])?;
                ctx.next()?;
                'b2: {
                    let subject2 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Ask Elgo to dye it.:Leave it alone.")],
                    )?);
                    let mut matched2 = false;
                    let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                    if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            "Elgo",
                            args![
                                "Oh, So you want to dye your Cute Ribbon, aren't you?",
                                "What colour is your Cute Ribbon?",
                                "Which one is the one you want to be dyed?"
                            ],
                        )?;
                        ctx.next()?;
                        let subject3 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Yellow one..:White one..:Blue one..:Red one..:Green one..:Never mind.")],
                        )?);
                        if subject3 == 1 {
                            if ctx.items().count(2250)? > 0 {
                                ctx.lines_as(
                                    "Elgo",
                                    args!["Ok Ok, Yellow Cute Ribbon, huh?", "What color do you want it to be dyed?"],
                                )?;
                                ctx.next()?;
                                let subject4 = Val::from(runtime::select_values(ctx, &[Val::from("White:Blue:Red:Green")])?);
                                if subject4 == 1 {
                                    if ctx.items().count(982)? > 0 {
                                        ctx.lines_as("Elgo", args!["Wow, pure and innocent white!", "Ok, let's do it."])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(2250, 1)?;
                                        ctx.items().take(982, 1)?;
                                        ctx.items().give(5441, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it white, bring me ^4d4dff1 White Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject4 == 2 {
                                    if ctx.items().count(978)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cool blue!!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(2250, 1)?;
                                        ctx.items().take(978, 1)?;
                                        ctx.items().give(5440, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it blue, bring me ^4d4dff1 Blue Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject4 == 3 {
                                    if ctx.items().count(975)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cute Red!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(2250, 1)?;
                                        ctx.items().take(975, 1)?;
                                        ctx.items().give(5439, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it red, bring me ^4d4dff1 Red Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject4 == 4 {
                                    if ctx.items().count(979)? > 0 {
                                        ctx.lines_as("Elgo", args!["Nature's Green!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(2250, 1)?;
                                        ctx.items().take(979, 1)?;
                                        ctx.items().give(5438, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it green, bring me ^4d4dff1 Green Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                }
                            }
                            ctx.lines_as(
                                "Elgo",
                                args!["Yellow is the basic color of a Cute Ribbon.", "But you don't even have one."],
                            )?;
                            return ctx.close();
                        } else if subject3 == 2 {
                            if ctx.items().count(5441)? > 0 {
                                ctx.lines_as(
                                    "Elgo",
                                    args!["Ok Ok, This white Cute Ribbon, huh?", "What color do you want it to be dyed?"],
                                )?;
                                ctx.next()?;
                                let subject5 = Val::from(runtime::select_values(ctx, &[Val::from("Yellow:Blue:Red:Green")])?);
                                if subject5 == 1 {
                                    if ctx.items().count(976)? > 0 {
                                        ctx.lines_as("Elgo", args!["Basic Yellow!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5441, 1)?;
                                        ctx.items().take(976, 1)?;
                                        ctx.items().give(2250, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it yellow, bring me ^4d4dff1 yellow Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject5 == 2 {
                                    if ctx.items().count(978)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cool blue!!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5441, 1)?;
                                        ctx.items().take(978, 1)?;
                                        ctx.items().give(5440, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it blue, bring me ^4d4dff1 Blue Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject5 == 3 {
                                    if ctx.items().count(975)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cute Red!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5441, 1)?;
                                        ctx.items().take(975, 1)?;
                                        ctx.items().give(5439, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it red, bring me ^4d4dff1 Red Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject5 == 4 {
                                    if ctx.items().count(979)? > 0 {
                                        ctx.lines_as("Elgo", args!["Nature's Green!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5441, 1)?;
                                        ctx.items().take(979, 1)?;
                                        ctx.items().give(5438, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it green, bring me ^4d4dff1 Green Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                }
                            }
                            ctx.lines_as(
                                "Elgo",
                                args!["You want to dye a white Cute Ribbon, right?", "But you don't even have one."],
                            )?;
                            return ctx.close();
                        } else if subject3 == 3 {
                            if ctx.items().count(5440)? > 0 {
                                ctx.lines_as(
                                    "Elgo",
                                    args!["Ok Ok, This blue Cute Ribbon, huh?", "What color do you want it to be dyed?"],
                                )?;
                                ctx.next()?;
                                let subject6 = Val::from(runtime::select_values(ctx, &[Val::from("White:Yellow:Red:Green")])?);
                                if subject6 == 1 {
                                    if ctx.items().count(982)? > 0 {
                                        ctx.lines_as("Elgo", args!["Wow, pure and innocent white!", "Ok, let's do it."])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5440, 1)?;
                                        ctx.items().take(982, 1)?;
                                        ctx.items().give(5441, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it white, bring me ^4d4dff1 White Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject6 == 2 {
                                    if ctx.items().count(976)? > 0 {
                                        ctx.lines_as("Elgo", args!["Basic Yellow!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5440, 1)?;
                                        ctx.items().take(976, 1)?;
                                        ctx.items().give(2250, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it yellow, bring me ^4d4dff1 yellow Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject6 == 3 {
                                    if ctx.items().count(975)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cute Red!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5440, 1)?;
                                        ctx.items().take(975, 1)?;
                                        ctx.items().give(5439, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it red, bring me ^4d4dff1 Red Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject6 == 4 {
                                    if ctx.items().count(979)? > 0 {
                                        ctx.lines_as("Elgo", args!["Nature's Green!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5440, 1)?;
                                        ctx.items().take(979, 1)?;
                                        ctx.items().give(5438, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it green, bring me ^4d4dff1 Green Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                }
                            }
                            ctx.lines_as(
                                "Elgo",
                                args!["You want to dye a blue Cute Ribbon, right?", "But you don't even have one."],
                            )?;
                            return ctx.close();
                        } else if subject3 == 4 {
                            if ctx.items().count(5439)? > 0 {
                                ctx.lines_as(
                                    "Elgo",
                                    args!["Ok Ok, This Red Cute Ribbon, huh?", "What color do you want it to be dyed?"],
                                )?;
                                ctx.next()?;
                                let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("White:Blue:Yellow:Green")])?);
                                if subject7 == 1 {
                                    if ctx.items().count(982)? > 0 {
                                        ctx.lines_as("Elgo", args!["Wow, pure and innocent white!", "Ok, let's do it."])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5439, 1)?;
                                        ctx.items().take(982, 1)?;
                                        ctx.items().give(5441, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it white, bring me ^4d4dff1 White Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject7 == 2 {
                                    if ctx.items().count(978)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cool blue!!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5439, 1)?;
                                        ctx.items().take(978, 1)?;
                                        ctx.items().give(5440, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it blue, bring me ^4d4dff1 Blue Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject7 == 3 {
                                    if ctx.items().count(976)? > 0 {
                                        ctx.lines_as("Elgo", args!["Basic Yellow!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5439, 1)?;
                                        ctx.items().take(976, 1)?;
                                        ctx.items().give(2250, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it yellow, bring me ^4d4dff1 yellow Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject7 == 4 {
                                    if ctx.items().count(979)? > 0 {
                                        ctx.lines_as("Elgo", args!["Nature's Green!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5439, 1)?;
                                        ctx.items().take(979, 1)?;
                                        ctx.items().give(5438, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it green, bring me ^4d4dff1 Green Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                }
                            }
                            ctx.lines_as(
                                "Elgo",
                                args!["You want to dye a red Cute Ribbon, right?", "But you don't even have one."],
                            )?;
                            return ctx.close();
                        } else if subject3 == 5 {
                            if ctx.items().count(5438)? > 0 {
                                ctx.lines_as(
                                    "Elgo",
                                    args!["Ok Ok, This Green Cute Ribbon, huh?", "What color do you want it to be dyed?"],
                                )?;
                                ctx.next()?;
                                let subject8 = Val::from(runtime::select_values(ctx, &[Val::from("White:Blue:Red:Yellow")])?);
                                if subject8 == 1 {
                                    if ctx.items().count(982)? > 0 {
                                        ctx.lines_as("Elgo", args!["Wow, pure and innocent white!", "Ok, let's do it."])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5438, 1)?;
                                        ctx.items().take(982, 1)?;
                                        ctx.items().give(5441, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it white, bring me ^4d4dff1 White Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject8 == 2 {
                                    if ctx.items().count(978)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cool blue!!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5438, 1)?;
                                        ctx.items().take(978, 1)?;
                                        ctx.items().give(5440, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it blue, bring me ^4d4dff1 Blue Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject8 == 3 {
                                    if ctx.items().count(975)? > 0 {
                                        ctx.lines_as("Elgo", args!["Cute Red!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5438, 1)?;
                                        ctx.items().take(975, 1)?;
                                        ctx.items().give(5439, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it red, bring me ^4d4dff1 Red Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                } else if subject8 == 4 {
                                    if ctx.items().count(976)? > 0 {
                                        ctx.lines_as("Elgo", args!["Basic Yellow!", "Let's do it!"])?;
                                        ctx.next()?;
                                        ctx.mes("- Elgo starts to dissolve dyes and chemicals in water, then he begins to dye the Cute Ribbon. -")?;
                                        ctx.next()?;
                                        ctx.lines_as(
                                            "Elgo",
                                            args![
                                                ".. Hmm..",
                                                "This should do it.",
                                                "What do you think?",
                                                "Did this color came out nicely?"
                                            ],
                                        )?;
                                        ctx.items().take(5438, 1)?;
                                        ctx.items().take(976, 1)?;
                                        ctx.items().give(2250, 1)?;
                                        return ctx.close();
                                    }
                                    ctx.lines_as(
                                        "Elgo",
                                        args!["If you want to dye it yellow, bring me ^4d4dff1 yellow Dyestuffs^000000."],
                                    )?;
                                    return ctx.close();
                                }
                            }
                            ctx.lines_as(
                                "Elgo",
                                args!["You want to dye a Green Cute Ribbon, right?", "But you don't even have one."],
                            )?;
                            return ctx.close();
                        } else if subject3 == 6 {
                            ctx.lines_as(
                                ctx.player().name()?,
                                args!["Ah, I'm not sure this time. I will come back next time.", "Take Care."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as("Elgo", args!["Ok. It's fine. I will see you later~ Bye Bye~!"])?;
                            return ctx.close();
                        }
                    }
                    if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                        matched2 = true;
                    }
                    if matched2 {
                        ctx.lines_as(
                            ctx.player().name()?,
                            args!["Hmm, I will bring you one next time.", "Take Care."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elgo", args!["Ok, Good bye!"])?;
                        return ctx.close();
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Elgo", args!["Uh..Would you please leave?", "I need to get back to work..."])?;
            return ctx.close();
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum OrcLady2008hat03Step {
    Start,
    OnTouch,
}

fn orc_lady_2008hat03_run(ctx: &Ctx, mut step: OrcLady2008hat03Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            OrcLady2008hat03Step::Start => {
                step = OrcLady2008hat03Step::OnTouch;
                continue 'machine;
            }
            OrcLady2008hat03Step::OnTouch => {
                ctx.lines_as(
                    ctx.player().name()?,
                    args![
                        "This Orc Lady is absorbed in something.",
                        "I don't know what she's making...but she wouldn't notice me even I walk up on her."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.player().name()?,
                    args!["Should I talk to her?...Uh..Do we even understand eachother?"],
                )?;
                ctx.next()?;
                ctx.lines(args![
                    "THe Orc Lady suddenly turns around looking surprised.",
                    "A Maneater Blossom fell on the ground out of her hands."
                ])?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("Pick up the flowers for Orc Lady.:Run away!")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Orc Lady",
                            args![
                                ".. ...",
                                "Adventurer with weapon helps Orc?",
                                "You weirdo.",
                                "My name is Aite-Nah-Zir-Be."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            ctx.player().name()?,
                            args!["Surprisingly the Orc Lady introduced herself to me.", "How should I react?"],
                        )?;
                        ctx.next()?;
                        let subject2 = Val::from(runtime::select_values(
                            ctx,
                            &[Val::from("Ask her what she was up to.:Say good bye.")],
                        )?);
                        if subject2 == 1 {
                            ctx.lines_as(
                                "Aite",
                                args![
                                    "I'm making a Corolla.",
                                    "Weaving the flower...with a ribbon..",
                                    "Brides...wear...Corollas.."
                                ],
                            )?;
                            ctx.next()?;
                            let choice = runtime::select_values(ctx, &[Val::from("Would you make one for me?")])?;
                            ctx.var("@menu").set(choice)?;
                            if (ctx.items().count(10007)? > 0 && ctx.items().count(1032)? > 999) {
                                ctx.lines_as("Aite", args![".. Materials..you..have..", "..Too much time to make one."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Aite",
                                    args![
                                        "Trade with you.. the one I just made.",
                                        "But you, weirdo.",
                                        "You humans are hard...to...understand."
                                    ],
                                )?;
                                ctx.items().take(10007, 1)?;
                                ctx.items().take(1032, 1000)?;
                                ctx.items().give(5436, 1)?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            ctx.lines_as(
                                "Aite",
                                args!["I need... Silk... Ribbon.", "1,000 Maneater Blossoms..", "Bring me...."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                ctx.player().name()?,
                                args![
                                    "(^4d4dff Silk Ribbon and 1,000 Maneater Blossoms^000000, huh..)",
                                    "Got it. I will go and bring back the materials."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if subject2 == 2 {
                            ctx.lines_as(
                                ctx.player().name()?,
                                args![
                                    "I nodded goodbye to her with a smile on my face saying nothing.",
                                    "She nodded back to be as if she understood that was my good bye."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(ctx.player().name()?, args!["Let's run away while she's still in shock."])?;
                        ctx.close_window()?;
                        ctx.warp("gef_fild10", 223, 203)?;
                        return Err(Stop::End);
                    }
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn orc_lady_2008hat03(ctx: &Ctx) -> Script {
    orc_lady_2008hat03_run(ctx, OrcLady2008hat03Step::Start, Vec::new()).map(|_| ())
}

pub fn orc_lady_2008hat03_ontouch(ctx: &Ctx) -> Script {
    orc_lady_2008hat03_run(ctx, OrcLady2008hat03Step::OnTouch, Vec::new()).map(|_| ())
}
